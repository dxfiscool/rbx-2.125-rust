// Auto-generated shard A — next 120 RBX::Reflection stubs — EA-sorted, skip already stubbed
// Source: ida/export.json filtered demangled contains RBX::Reflection (16171 total)
// Format: // 0xADDR — mangled + #[doc(alias = "RBX::...")] + todo!("0xADDR") using rbx_core::SharedPtr
#![allow(unused_imports)]
use rbx_core::SharedPtr;
/// Cutover value model for this shard: the `RBX::Reflection::Variant`
/// payloads touched by the descriptors below (float/double/bool/int/string,
/// `G3D::Vector2/3`, `Vector2int16`, `CoordinateFrame`, enum ids, refs).
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Nil,
    Bool(bool),
    Int(i32),
    Float(f32),
    Double(f64),
    Text(String),
    Vector2([f32; 2]),
    Vector3(Vector3),
    Vector2i([i16; 2]),
    CoordinateFrame(CoordinateFrame),
    EnumValue(i32),
    Instance(u32),
    InstanceList(Vec<u32>),
}

/// `G3D::Vector3` cutover (IDA 0x5f0f70 compares the three lanes directly).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// `G3D::CoordinateFrame` cutover: rotation plus translation (IDA 0x5f32dc
/// compares the translation lanes then `G3D::Matrix3::operator==`).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct CoordinateFrame {
    pub rotation: [[f32; 3]; 3],
    pub translation: Vector3,
}

impl CoordinateFrame {
    pub fn identity() -> Self {
        Self {
            rotation: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            translation: Vector3::default(),
        }
    }
}

/// `RBX::Reflection::PropDescriptor<C, T>` bound storage (IDA 0x5f0cec,
/// 0x5f0e00, 0x5f2b1c): name/category/attributes/permissions plus the live
/// value. The getter/setter member-pointer pair (and the `DescribedBase`
/// header strip `a2 - 36` at 0x5f0ca8/0x5f0cc8) collapses into direct field
/// access; the vtable/`auto_ptr` ownership dance is Drop glue.
#[derive(Debug, Clone, PartialEq)]
pub struct Prop<T: Clone + PartialEq + std::fmt::Debug> {
    pub name: String,
    pub category: String,
    pub attributes: u32,
    pub permissions: u32,
    pub value: T,
}

impl<T: Clone + PartialEq + std::fmt::Debug> Prop<T> {
    pub fn new(name: &str, category: &str, value: T, attributes: u32, permissions: u32) -> Self {
        Self {
            name: name.to_owned(),
            category: category.to_owned(),
            attributes,
            permissions,
            value,
        }
    }
}

impl Value {
    /// `RBX::Reflection::Variant::convert<float>` (IDA 0x5f0b96): direct on a
    /// float payload (`rbx::any_cast`), numeric widening otherwise.
    pub fn as_float(&self) -> f32 {
        match self {
            Value::Float(v) => *v,
            Value::Int(v) => *v as f32,
            Value::Double(v) => *v as f32,
            Value::Bool(v) => *v as i32 as f32,
            other => panic!("Variant::convert<float> on {other:?} (IDA 0x5f0ae8)"),
        }
    }

    /// `RBX::Reflection::Variant::convert<G3D::Vector3>` (IDA 0x5f108a).
    pub fn as_vector3(&self) -> Vector3 {
        match self {
            Value::Vector3(v) => *v,
            other => panic!("Variant::convert<Vector3> on {other:?} (IDA 0x5f0fd8)"),
        }
    }

    /// `RBX::Reflection::Variant::convert<G3D::CoordinateFrame>` (IDA 0x5f340c).
    pub fn as_coordinate_frame(&self) -> CoordinateFrame {
        match self {
            Value::CoordinateFrame(v) => *v,
            other => panic!("Variant::convert<CoordinateFrame> on {other:?} (IDA 0x5f340c)"),
        }
    }

    /// `RBX::Reflection::Variant::convert<double>`.
    pub fn as_double(&self) -> f64 {
        match self {
            Value::Double(v) => *v,
            Value::Float(v) => *v as f64,
            Value::Int(v) => *v as f64,
            other => panic!("Variant::convert<double> on {other:?}"),
        }
    }

    /// `RBX::Reflection::Variant::convert<bool>` (IDA 0x5f1810 path).
    pub fn as_bool(&self) -> bool {
        match self {
            Value::Bool(v) => *v,
            Value::Int(v) => *v != 0,
            Value::Float(v) => *v != 0.0,
            other => panic!("Variant::convert<bool> on {other:?}"),
        }
    }

    /// `RBX::Reflection::Variant::convert<std::string>`: direct on a text
    /// payload (`any_cast`), numeric widening otherwise.
    pub fn as_text(&self) -> String {
        match self {
            Value::Text(v) => v.clone(),
            Value::Int(v) => v.to_string(),
            Value::Float(v) => v.to_string(),
            Value::Double(v) => v.to_string(),
            Value::Bool(v) => v.to_string(),
            other => panic!("Variant::convert<string> on {other:?}"),
        }
    }

    /// `RBX::Reflection::Variant::convert<G3D::Vector2int16>`.
    pub fn as_vector2i(&self) -> [i16; 2] {
        match self {
            Value::Vector2i(v) => *v,
            other => panic!("Variant::convert<Vector2int16> on {other:?}"),
        }
    }
}

/// `RBX::Reflection::FunctionDescriptor::Arguments` cutover: positional
/// argument variants; an absent index reads as nil.
#[derive(Debug, Clone, Default)]
pub struct Arguments {
    pub args: Vec<Value>,
}

/// `RBX::Reflection::FunctionDescriptor::SignatureDescriptor` cutover
/// (IDA 0x5f15e0): declared return type plus `(name, type)` arguments.
#[derive(Debug, Clone, Default)]
pub struct Signature {
    pub return_type: &'static str,
    pub args: Vec<(String, &'static str)>,
}

/// `RBX::Reflection::BoundFuncDesc<C, R, N>` header cutover (IDA 0x5f19c4,
/// 0x5f1bd0, 0x5f1ddc, 0x5f1434): bound member id, name, declared signature,
/// permissions and attributes. The member-function pointer pair folds into
/// `member`; class-descriptor registration collapses into parameters.
#[derive(Debug, Clone)]
pub struct BoundFunc {
    pub name: String,
    pub member: usize,
    pub signature: Signature,
    pub permissions: u32,
    pub attributes: u32,
}
/// `RBX::Reflection::EnumPropDescriptor<C, E>` cutover (IDA 0x5f9d30,
/// 0x4a5834): name/category/attributes/permissions, the live enum value and
/// the item table. The getter/setter member-pointer pair (+44) folds into
/// direct field access; the `EnumDesc` singleton link (+40/+48) folds into
/// the owned table (same shape as `FaceNormalPropDesc` in descriptor.rs).
#[derive(Debug, Clone)]
pub struct EnumProp {
    pub name: String,
    pub category: String,
    pub attributes: u32,
    pub permissions: u32,
    pub value: i32,
    pub enum_desc: crate::enum_desc::EnumDesc,
}

impl EnumProp {
    pub fn new(
        name: &str,
        category: &str,
        initial: i32,
        enum_desc: crate::enum_desc::EnumDesc,
        attributes: u32,
        permissions: u32,
    ) -> Self {
        Self {
            name: name.to_owned(),
            category: category.to_owned(),
            attributes,
            permissions,
            value: initial,
            enum_desc,
        }
    }

    /// `EnumDesc<T>::convertToIndex` (IDA 0x4aa47c): `ReleaseAssert(value>=0)`
    /// (enumconverter.h:350), `value_ordinals[value]` or -1.
    pub fn convert_to_index(&self, value: i32) -> i32 {
        assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
        usize::try_from(value)
            .ok()
            .and_then(|slot| self.enum_desc.value_ordinals.get(slot).copied())
            .unwrap_or(-1)
    }
}

// 0x5f0a90 — __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceEfED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,float>::~PropDescriptor()")]
pub fn stub_0x5f0a90() {
    // IDA 0x5f0a90: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x5f0ac0 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIfE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<float>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x5f0ac0(prop: &Prop<f32>) -> Value {
    // IDA 0x5f0ac0: `getVariant`: `getValue` via slot 8, tag
    // `Type::getSingleton<float>`, pack with `placement_any::operator=`.
    Value::Float(prop.value)
}

// 0x5f0ae8 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIfE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<float>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x5f0ae8(prop: &mut Prop<f32>, value: &Value) {
    // IDA 0x5f0ae8: `setVariant`: `any_cast<float>` on a float payload,
    // else `Variant::convert<float>` on a copied variant (0x5f0b6a-0x5f0ba8),
    // then `setValue` via slot 12.
    prop.value = value.as_float();
}

// 0x5f0c40 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIfE9copyValueEPKNS0_13DescribedBaseEPS3_
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<float>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x5f0c40(dst: &mut Prop<f32>, src: &Prop<f32>) {
    // IDA 0x5f0c40: `copyValue`: `getValue` into a temp via slot 8, then
    // `setValue` into the destination via slot 12.
    dst.value = src.value;
}

// 0x5f0c78 — __ZN3RBX10Reflection23TypedPropertyDescriptorIfED1Ev
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<float>::~TypedPropertyDescriptor()")]
pub fn stub_0x5f0c78() {
    // IDA 0x5f0c78: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x5f0ca0 — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,float>::GetSetImpl<float (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(float)>::isReadOnly(void)const")]
pub fn stub_0x5f0ca0() -> bool {
    // IDA 0x5f0ca0: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0x5f0ca4 — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,float>::GetSetImpl<float (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(float)>::isWriteOnly(void)const")]
pub fn stub_0x5f0ca4() -> bool {
    // IDA 0x5f0ca4: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0x5f0ca8 — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,float>::GetSetImpl<float (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x5f0ca8(prop: &Prop<f32>) -> f32 {
    // IDA 0x5f0ca8: `GetSetImpl::getValue`: strip the `DescribedBase` header
    // (`a2 - 36`), decode the getter member pointer (`offset >> 1`, virtual
    // bit `& 1`), invoke it.
    prop.value
}

// 0x5f0cc8 — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,float>::GetSetImpl<float (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
pub fn stub_0x5f0cc8(prop: &mut Prop<f32>, value: f32) {
    // IDA 0x5f0cc8: `GetSetImpl::setValue`: strip the `DescribedBase` header
    // (`a2 - 36`), decode the setter member pointer (`offset >> 1`, virtual
    // bit `& 1`), invoke it with the new value.
    prop.value = value;
}

// 0x5f0cec — __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceEN3G3D7Vector3EEC2IMS2_KFS4_vEMS2_FvRKS4_EEEPKcSE_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,G3D::Vector3>::PropDescriptor<G3D::Vector3 (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(G3D::Vector3 const&)>(char const*,char const*,G3D::Vector3 (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(G3D::Vector3 const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x5f0cec(
    name: &str,
    category: &str,
    initial: Vector3,
    attributes: u32,
    permissions: u32,
) -> Prop<Vector3> {
    // IDA 0x5f0cec: `PropDescriptor<PartInstance, Vector3>` get/set ctor:
    // fetch the PartInstance class descriptor, `new` the GetSetImpl holding
    // the getter/setter member pointers (0x5f0d1a-0x5f0d54), forward into
    // `TypedPropertyDescriptor<Vector3>::TypedPropertyDescriptor`. Rust: the
    // member-pointer pair folds into the stored value.
    Prop::new(name, category, initial, attributes, permissions)
}

// 0x5f0e00 — __ZN3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector3EEC2ERNS0_15ClassDescriptorEPKcS8_St8auto_ptrINS4_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector3>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<G3D::Vector3>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x5f0e00(
    name: &str,
    category: &str,
    initial: Vector3,
    attributes: u32,
    permissions: u32,
) -> Prop<Vector3> {
    // IDA 0x5f0e00: `TypedPropertyDescriptor<Vector3>` ctor: tag
    // `Type::getSingleton<Vector3>`, base `PropertyDescriptor` init, vtable
    // install, take over the `auto_ptr<GetSet>`, then clear the read-only /
    // write-only attribute bits when the GetSet reports them (0x5f0eaa,
    // 0x5f0ec6). Rust: the GetSet folds into the stored value; attribute
    // masking is a no-op on the recorded bits.
    Prop::new(name, category, initial, attributes, permissions)
}

// 0x5f0f24 — __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceEN3G3D7Vector3EED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,G3D::Vector3>::~PropDescriptor()")]
pub fn stub_0x5f0f24() {
    // IDA 0x5f0f24: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x5f0f50 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector3EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector3>::isReadOnly(void)const")]
pub fn stub_0x5f0f50() {
    // IDA 0x5f0f50: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x5f0f60 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector3EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector3>::isWriteOnly(void)const")]
pub fn stub_0x5f0f60() {
    // IDA 0x5f0f60: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x5f0f70 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector3EE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector3>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x5f0f70(a: &Prop<Vector3>, b: &Prop<Vector3>) -> bool {
    // IDA 0x5f0f70: `equalValues`: `getValue` both sides via slot 8, compare
    // lane by lane (`v7[i] == v6[i]`, 0x5f0fa4-0x5f0fcc).
    a.value == b.value
}

// 0x5f0fd8 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector3EE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector3>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x5f0fd8(prop: &mut Prop<Vector3>, value: &Value) {
    // IDA 0x5f0fd8: `setVariant`: `any_cast<Vector3>` on a Vector3 payload
    // (typeinfo + `"N3G3D7Vector3E"` name check, 0x5f1062), else
    // `Variant::convert<Vector3>` on a copied variant, then `setValue`.
    prop.value = value.as_vector3();
}

// 0x5f1160 — __ZN3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector3EED1Ev
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector3>::~TypedPropertyDescriptor()")]
pub fn stub_0x5f1160() {
    // IDA 0x5f1160: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x5f1188 — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEN3G3D7Vector3EE10GetSetImplIMS2_KFS4_vEMS2_FvRKS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,G3D::Vector3>::GetSetImpl<G3D::Vector3 (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(G3D::Vector3 const&)>::isReadOnly(void)const")]
pub fn stub_0x5f1188() -> bool {
    // IDA 0x5f1188: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0x5f118c — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEN3G3D7Vector3EE10GetSetImplIMS2_KFS4_vEMS2_FvRKS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,G3D::Vector3>::GetSetImpl<G3D::Vector3 (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(G3D::Vector3 const&)>::isWriteOnly(void)const")]
pub fn stub_0x5f118c() -> bool {
    // IDA 0x5f118c: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0x5f1190 — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEN3G3D7Vector3EE10GetSetImplIMS2_KFS4_vEMS2_FvRKS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,G3D::Vector3>::GetSetImpl<G3D::Vector3 (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(G3D::Vector3 const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x5f1190(prop: &Prop<Vector3>) -> Vector3 {
    // IDA 0x5f1190: `GetSetImpl<Vector3>::getValue`: header strip, getter
    // member-pointer decode, invoke.
    prop.value
}

// 0x5f11b8 — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEN3G3D7Vector3EE10GetSetImplIMS2_KFS4_vEMS2_FvRKS4_EE8setValueEPNS0_13DescribedBaseESA_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,G3D::Vector3>::GetSetImpl<G3D::Vector3 (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(G3D::Vector3 const&)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector3 const&)const")]
pub fn stub_0x5f11b8(prop: &mut Prop<Vector3>, value: Vector3) {
    // IDA 0x5f11b8: `GetSetImpl<Vector3>::setValue`: header strip, setter
    // member-pointer decode, invoke with the new value.
    prop.value = value;
}

// 0x5f11dc — __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceEN3G3D7Vector3EEC2IiMS2_FvRKS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,G3D::Vector3>::PropDescriptor<int,void (RBX::PartInstance::*)(G3D::Vector3 const&)>(char const*,char const*,int,void (RBX::PartInstance::*)(G3D::Vector3 const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x5f11dc(
    name: &str,
    category: &str,
    initial: Vector3,
    attributes: u32,
    permissions: u32,
) -> Prop<Vector3> {
    // IDA 0x5f11dc: `PropDescriptor<PartInstance, Vector3>` write-only ctor
    // (`int` placeholder getter + setter member pointer): `new` the SetImpl
    // (0x5f1206-0x5f1232), forward into the `TypedPropertyDescriptor`
    // ctor. Reads go through `SetImpl::getValue` (0x5f12f0) and throw.
    Prop::new(name, category, initial, attributes, permissions)
}

// 0x5f12e8 — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEN3G3D7Vector3EE7SetImplIMS2_FvRKS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,G3D::Vector3>::SetImpl<void (RBX::PartInstance::*)(G3D::Vector3 const&)>::isReadOnly(void)const")]
pub fn stub_0x5f12e8() {
    // IDA 0x5f12e8: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x5f12ec — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEN3G3D7Vector3EE7SetImplIMS2_FvRKS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,G3D::Vector3>::SetImpl<void (RBX::PartInstance::*)(G3D::Vector3 const&)>::isWriteOnly(void)const")]
pub fn stub_0x5f12ec() {
    // IDA 0x5f12ec: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x5f12f0 — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEN3G3D7Vector3EE7SetImplIMS2_FvRKS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,G3D::Vector3>::SetImpl<void (RBX::PartInstance::*)(G3D::Vector3 const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x5f12f0() {
    // IDA 0x5f12f0: `SetImpl::getValue` (write-only prop): `throw
    // runtime_error("can't get value")` (0x5f131c-0x5f1400). Rust cutover
    // panics with the same message.
    panic!("can't get value (IDA 0x5f12f0)");
}

// 0x5f1410 — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEN3G3D7Vector3EE7SetImplIMS2_FvRKS4_EE8setValueEPNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,G3D::Vector3>::SetImpl<void (RBX::PartInstance::*)(G3D::Vector3 const&)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector3 const&)const")]
pub fn stub_0x5f1410(prop: &mut Prop<Vector3>, value: Vector3) {
    // IDA 0x5f1410: `SetImpl::setValue`: header strip, setter member-pointer
    // decode, invoke with the new value.
    prop.value = value;
}

// 0x5f1434 — __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEbELi1EEC2EMS2_FSB_bEPKcSH_bNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PartInstance,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(bool),1>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::PartInstance::*)(bool),char const*,char const*,bool,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x5f1434(
    name: &str,
    category: &str,
    member: usize,
    is_default: bool,
    permissions: u32,
    attributes: u32,
) -> BoundFunc {
    // IDA 0x5f1434: 1-arg `BoundFuncDesc` ctor: class-descriptor fetch,
    // `FunctionDescriptor` init, member pair at +40, declared signature via
    // `declareSignature` (0x5f15e0). `category`/`is_default` ride the
    // descriptor header; Rust folds them into the name record.
    let _ = (category, is_default);
    BoundFunc {
        name: name.to_owned(),
        member,
        signature: Signature { return_type: "InstanceList", args: Vec::new() },
        permissions,
        attributes,
    }
}

// 0x5f15e0 — __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEbELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PartInstance,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x5f15e0(func: &mut BoundFunc, arg_name: &str) {
    // IDA 0x5f15e0: `declareSignature`: store the shared-vector return
    // `Type` at +28, `RBX::Name::declare` the arg name, `getSingleton<bool>`
    // for it, `SignatureDescriptor::addArgument` (0x5f15ec-0x5f160e).
    func.signature.return_type = "InstanceList";
    func.signature.args.push((arg_name.to_owned(), "bool"));
}

// 0x5f1610 — __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEbELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PartInstance,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(bool),1>::~BoundFuncDesc()")]
pub fn stub_0x5f1610() {
    // IDA 0x5f1610: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x5f16e4 — __ZNK3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PartInstance,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x5f16e4(
    args: &Arguments,
    call: &dyn Fn(bool) -> Vec<u32>,
    default: Option<bool>,
) -> Value {
    // IDA 0x5f16e4: 1-arg `execute`: `ArgHelper::getArg<bool, 1>` then
    // `Call1Helper::call` (0x5f1708-0x5f1722).
    let flag = stub_0x5f1810(args, default);
    stub_0x5f1724(call, flag)
}

// 0x5f1724 — __ZN3RBX10Reflection11Call1HelperINS_12PartInstanceEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEbEbSB_E4callEPS2_SD_RNS0_7VariantERKb
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::PartInstance,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::PartInstance::*)(bool),bool,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::call(RBX::PartInstance*,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::PartInstance::*)(bool),RBX::Reflection::Variant &,bool const&)")]
pub fn stub_0x5f1724(call: &dyn Fn(bool) -> Vec<u32>, arg: bool) -> Value {
    // IDA 0x5f1724: `Call1Helper::call`: member-pointer decode, invoke with
    // the bool arg, tag the shared-vector return type, pack with
    // `placement_any::operator=` (0x5f1798-0x5f17a4).
    Value::InstanceList(call(arg))
}

// 0x5f1810 — __ZN3RBX10Reflection9ArgHelper6getArgIbLi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "bool RBX::Reflection::ArgHelper::getArg<bool,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<bool> const&,boost::disable_if<boost::is_same<bool,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
pub fn stub_0x5f1810(args: &Arguments, default: Option<bool>) -> bool {
    // IDA 0x5f1810: `ArgHelper::getArg<bool, 1>`: direct bool fetch, else a
    // `void`-singleton probe then `Variant::convert<bool>` (0x5f189c-0x5f18c4);
    // missing/nil without a default throws `runtime_error("Argument %d
    // missing or nil", 1)` (0x5f1930-0x5f1986). Rust cutover panics.
    match args.args.first() {
        Some(Value::Nil) | None => default.unwrap_or_else(|| panic!("Argument 1 missing or nil (IDA 0x5f1810)")),
        Some(v) => v.as_bool(),
    }
}

// 0x5f19c4 — __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFbvELi0EEC2EMS2_FbvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PartInstance,bool ()(void),0>::BoundFuncDesc(bool (RBX::PartInstance::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x5f19c4(name: &str, member: usize, permissions: u32, attributes: u32) -> BoundFunc {
    // IDA 0x5f19c4: `BoundFuncDesc<PartInstance, bool>::BoundFuncDesc`:
    // class-descriptor fetch, `FunctionDescriptor` init, vtable install,
    // member pair at +40, return type `Type::getSingleton<bool>` at +28.
    BoundFunc {
        name: name.to_owned(),
        member,
        signature: Signature { return_type: "bool", args: Vec::new() },
        permissions,
        attributes,
    }
}

// 0x5f1ac8 — __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFbvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PartInstance,bool ()(void),0>::~BoundFuncDesc()")]
pub fn stub_0x5f1ac8() {
    // IDA 0x5f1ac8: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x5f1b7c — __ZNK3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFbvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PartInstance,bool ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x5f1b7c(_func: &BoundFunc, call: &dyn Fn() -> bool) -> Value {
    // IDA 0x5f1b7c: `BoundFuncDesc<bool>::execute` tail-calls
    // `Call0Helper<bool>::call`.
    stub_0x5f1ba0(call)
}

// 0x5f1ba0 — __ZN3RBX10Reflection11Call0HelperINS_12PartInstanceEMS2_FbvEbE4callEPS2_S4_RNS0_7VariantE
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::PartInstance,bool (RBX::PartInstance::*)(void),bool>::call(RBX::PartInstance*,bool (RBX::PartInstance::*)(void),RBX::Reflection::Variant &)")]
pub fn stub_0x5f1ba0(call: &dyn Fn() -> bool) -> Value {
    // IDA 0x5f1ba0: `Call0Helper<bool>::call`: header strip, member-pointer
    // decode, invoke, tag `Type::getSingleton<bool>`, pack with
    // `placement_any::operator=<bool>`.
    Value::Bool(call())
}

// 0x5f1bd0 — __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFfvELi0EEC2EMS2_FfvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PartInstance,float ()(void),0>::BoundFuncDesc(float (RBX::PartInstance::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x5f1bd0(name: &str, member: usize, permissions: u32, attributes: u32) -> BoundFunc {
    // IDA 0x5f1bd0: `BoundFuncDesc<PartInstance, float>::BoundFuncDesc`:
    // class-descriptor fetch, `FunctionDescriptor` init, vtable install,
    // member pair at +40, return type `Type::getSingleton<float>` at +28.
    BoundFunc {
        name: name.to_owned(),
        member,
        signature: Signature { return_type: "float", args: Vec::new() },
        permissions,
        attributes,
    }
}

// 0x5f1cd4 — __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFfvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PartInstance,float ()(void),0>::~BoundFuncDesc()")]
pub fn stub_0x5f1cd4() {
    // IDA 0x5f1cd4: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x5f1d88 — __ZNK3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFfvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PartInstance,float ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x5f1d88(_func: &BoundFunc, call: &dyn Fn() -> f32) -> Value {
    // IDA 0x5f1d88: `BoundFuncDesc<float>::execute` tail-calls
    // `Call0Helper<float>::call`.
    stub_0x5f1dac(call)
}

// 0x5f1dac — __ZN3RBX10Reflection11Call0HelperINS_12PartInstanceEMS2_FfvEfE4callEPS2_S4_RNS0_7VariantE
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::PartInstance,float (RBX::PartInstance::*)(void),float>::call(RBX::PartInstance*,float (RBX::PartInstance::*)(void),RBX::Reflection::Variant &)")]
pub fn stub_0x5f1dac(call: &dyn Fn() -> f32) -> Value {
    // IDA 0x5f1dac: `Call0Helper<float>::call`: header strip, member-pointer
    // decode, invoke, tag `Type::getSingleton<float>`, pack with
    // `placement_any::operator=<float>`.
    Value::Float(call())
}

// 0x5f1ddc — __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PartInstance,void ()(void),0>::BoundFuncDesc(void (RBX::PartInstance::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x5f1ddc(name: &str, member: usize, permissions: u32, attributes: u32) -> BoundFunc {
    // IDA 0x5f1ddc: `BoundFuncDesc<PartInstance, void>::BoundFuncDesc`:
    // class-descriptor fetch, `FunctionDescriptor` init, vtable install,
    // member pair at +40, return type `Type::getSingleton<void>` at +28.
    BoundFunc {
        name: name.to_owned(),
        member,
        signature: Signature { return_type: "void", args: Vec::new() },
        permissions,
        attributes,
    }
}

// 0x5f1ee0 — __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFvvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PartInstance,void ()(void),0>::~BoundFuncDesc()")]
pub fn stub_0x5f1ee0() {
    // IDA 0x5f1ee0: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x5f1f94 — __ZNK3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PartInstance,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x5f1f94(_func: &BoundFunc, call: &dyn Fn()) {
    // IDA 0x5f1f94: `BoundFuncDesc<void>::execute`: header strip, decode the
    // member pair at +40 (`offset >> 1`, virtual bit `& 1`), invoke it; no
    // return packing for `void`.
    call();
}

// 0x5f2180 — __ZNSt6vectorIPKN3RBX10Reflection14EnumDescriptor4ItemESaIS5_EE9push_backERKS5_
#[doc(alias = "std::vector<RBX::Reflection::EnumDescriptor::Item const*,std::allocator<RBX::Reflection::EnumDescriptor::Item const*>>::push_back(RBX::Reflection::EnumDescriptor::Item const* const&)")]
pub fn stub_0x5f2180(items: &mut Vec<crate::enum_desc::EnumItem>, item: crate::enum_desc::EnumItem) {
    // IDA 0x5f2180: `vector<EnumDescriptor::Item const*>::push_back`: fast
    // path stores at the finish pointer and bumps it (0x5f2192-0x5f219c),
    // else `_M_insert_aux` grows. Rust: `Vec::push` covers both.
    items.push(item);
}

// 0x5f2558 — __ZN3RBX10Reflection14EnumDescriptor4ItemD1Ev
#[doc(alias = "RBX::Reflection::EnumDescriptor::Item::~Item()")]
pub fn stub_0x5f2558() {
    // IDA 0x5f2558: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x5f2b1c — __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceEfEC2IMS2_KFfvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,float>::PropDescriptor<float (RBX::PartInstance::*)(void)const,int>(char const*,char const*,float (RBX::PartInstance::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x5f2b1c(
    name: &str,
    category: &str,
    initial: f32,
    attributes: u32,
    permissions: u32,
) -> Prop<f32> {
    // IDA 0x5f2b1c: `PropDescriptor<PartInstance, float>` read-only ctor
    // (getter + `int` placeholder): `new` the GetImpl (0x5f2b48-0x5f2b72),
    // forward into `TypedPropertyDescriptor<float>`. Writes go through
    // `GetImpl::setValue` (0x5f2c50) and throw.
    Prop::new(name, category, initial, attributes, permissions)
}

// 0x5f2c28 — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEfE7GetImplIMS2_KFfvEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,float>::GetImpl<float (RBX::PartInstance::*)(void)const>::isReadOnly(void)const")]
pub fn stub_0x5f2c28() {
    // IDA 0x5f2c28: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x5f2c2c — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEfE7GetImplIMS2_KFfvEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,float>::GetImpl<float (RBX::PartInstance::*)(void)const>::isWriteOnly(void)const")]
pub fn stub_0x5f2c2c() {
    // IDA 0x5f2c2c: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x5f2c30 — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEfE7GetImplIMS2_KFfvEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,float>::GetImpl<float (RBX::PartInstance::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x5f2c30(prop: &Prop<f32>) -> f32 {
    // IDA 0x5f2c30: `GetImpl<float>::getValue`: header strip, getter
    // member-pointer decode, invoke.
    prop.value
}

// 0x5f2c50 — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEfE7GetImplIMS2_KFfvEE8setValueEPNS0_13DescribedBaseERKf
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,float>::GetImpl<float (RBX::PartInstance::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
pub fn stub_0x5f2c50() {
    // IDA 0x5f2c50: `GetImpl::setValue` (read-only prop): `throw
    // runtime_error("can't set value")` (0x5f2c7c-0x5f2d60). Rust cutover
    // panics with the same message.
    panic!("can't set value (IDA 0x5f2c50)");
}

// 0x5f2d70 — __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceEN3G3D7Vector3EEC2IMS2_KFKS4_vEMS2_FvRS7_EEEPKcSE_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,G3D::Vector3>::PropDescriptor<G3D::Vector3 const (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(G3D::Vector3 const&)>(char const*,char const*,G3D::Vector3 const (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(G3D::Vector3 const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x5f2d70(
    name: &str,
    category: &str,
    initial: Vector3,
    attributes: u32,
    permissions: u32,
) -> Prop<Vector3> {
    // IDA 0x5f2d70: `PropDescriptor<PartInstance, Vector3>` get/set ctor with
    // a `Vector3 const` getter: `new` the GetSetImpl (0x5f2d9e-0x5f2dd8),
    // forward into the `TypedPropertyDescriptor` ctor.
    Prop::new(name, category, initial, attributes, permissions)
}

// 0x5f2e84 — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEN3G3D7Vector3EE10GetSetImplIMS2_KFKS4_vEMS2_FvRS7_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,G3D::Vector3>::GetSetImpl<G3D::Vector3 const (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(G3D::Vector3 const&)>::isReadOnly(void)const")]
pub fn stub_0x5f2e84() -> bool {
    // IDA 0x5f2e84: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0x5f2e88 — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEN3G3D7Vector3EE10GetSetImplIMS2_KFKS4_vEMS2_FvRS7_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,G3D::Vector3>::GetSetImpl<G3D::Vector3 const (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(G3D::Vector3 const&)>::isWriteOnly(void)const")]
pub fn stub_0x5f2e88() -> bool {
    // IDA 0x5f2e88: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0x5f2e8c — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEN3G3D7Vector3EE10GetSetImplIMS2_KFKS4_vEMS2_FvRS7_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,G3D::Vector3>::GetSetImpl<G3D::Vector3 const (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(G3D::Vector3 const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x5f2e8c(prop: &Prop<Vector3>) -> Vector3 {
    // IDA 0x5f2e8c: `GetSetImpl<Vector3 const>::getValue`: header strip,
    // getter member-pointer decode, invoke.
    prop.value
}

// 0x5f2eb4 — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEN3G3D7Vector3EE10GetSetImplIMS2_KFKS4_vEMS2_FvRS7_EE8setValueEPNS0_13DescribedBaseESA_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,G3D::Vector3>::GetSetImpl<G3D::Vector3 const (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(G3D::Vector3 const&)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector3 const&)const")]
pub fn stub_0x5f2eb4(prop: &mut Prop<Vector3>, value: Vector3) {
    // IDA 0x5f2eb4: `GetSetImpl<Vector3 const>::setValue`: header strip,
    // setter member-pointer decode, invoke.
    prop.value = value;
}

// 0x5f2ed8 — __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceEN3G3D7Vector3EEC2IMS2_KFRKS4_vEMS2_FvS8_EEEPKcSE_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,G3D::Vector3>::PropDescriptor<G3D::Vector3 const& (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(G3D::Vector3 const&)>(char const*,char const*,G3D::Vector3 const& (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(G3D::Vector3 const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x5f2ed8(
    name: &str,
    category: &str,
    initial: Vector3,
    attributes: u32,
    permissions: u32,
) -> Prop<Vector3> {
    // IDA 0x5f2ed8: `PropDescriptor<PartInstance, Vector3>` get/set ctor with
    // a `Vector3 const&` getter: same shape as 0x5f0cec (GetSetImpl `new` +
    // typed-descriptor forward).
    Prop::new(name, category, initial, attributes, permissions)
}

// 0x5f2fec — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEN3G3D7Vector3EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,G3D::Vector3>::GetSetImpl<G3D::Vector3 const& (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(G3D::Vector3 const&)>::isReadOnly(void)const")]
pub fn stub_0x5f2fec() -> bool {
    // IDA 0x5f2fec: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0x5f2ff0 — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEN3G3D7Vector3EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,G3D::Vector3>::GetSetImpl<G3D::Vector3 const& (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(G3D::Vector3 const&)>::isWriteOnly(void)const")]
pub fn stub_0x5f2ff0() -> bool {
    // IDA 0x5f2ff0: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0x5f2ff4 — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEN3G3D7Vector3EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,G3D::Vector3>::GetSetImpl<G3D::Vector3 const& (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(G3D::Vector3 const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x5f2ff4(prop: &Prop<Vector3>) -> Vector3 {
    // IDA 0x5f2ff4: `GetSetImpl<Vector3 const&>::getValue`: header strip,
    // getter member-pointer decode, invoke.
    prop.value
}

// 0x5f3028 — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEN3G3D7Vector3EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE8setValueEPNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,G3D::Vector3>::GetSetImpl<G3D::Vector3 const& (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(G3D::Vector3 const&)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector3 const&)const")]
pub fn stub_0x5f3028(prop: &mut Prop<Vector3>, value: Vector3) {
    // IDA 0x5f3028: `GetSetImpl<Vector3 const&>::setValue`: header strip,
    // setter member-pointer decode, invoke.
    prop.value = value;
}

// 0x5f304c — __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceEN3G3D15CoordinateFrameEEC2IMS2_KFRKS4_vEMS2_FvS8_EEEPKcSE_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,G3D::CoordinateFrame>::PropDescriptor<G3D::CoordinateFrame const& (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(G3D::CoordinateFrame const&)>(char const*,char const*,G3D::CoordinateFrame const& (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(G3D::CoordinateFrame const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x5f304c(
    name: &str,
    category: &str,
    initial: CoordinateFrame,
    attributes: u32,
    permissions: u32,
) -> Prop<CoordinateFrame> {
    // IDA 0x5f304c: `PropDescriptor<PartInstance, CoordinateFrame>` get/set
    // ctor: class-descriptor fetch, `new` the GetSetImpl (0x5f307a-0x5f30b4),
    // forward into the `TypedPropertyDescriptor` ctor.
    Prop::new(name, category, initial, attributes, permissions)
}

// 0x5f3160 — __ZN3RBX10Reflection23TypedPropertyDescriptorIN3G3D15CoordinateFrameEEC2ERNS0_15ClassDescriptorEPKcS8_St8auto_ptrINS4_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::CoordinateFrame>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<G3D::CoordinateFrame>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x5f3160(
    name: &str,
    category: &str,
    initial: CoordinateFrame,
    attributes: u32,
    permissions: u32,
) -> Prop<CoordinateFrame> {
    // IDA 0x5f3160: `TypedPropertyDescriptor<CoordinateFrame>` ctor: same
    // shape as 0x5f0e00 (type tag, base init, vtable, `auto_ptr` takeover,
    // read/write attribute masking at 0x5f3216/0x5f3232).
    Prop::new(name, category, initial, attributes, permissions)
}

// 0x5f3290 — __ZN3RBX10Reflection14PropDescriptorINS_12PartInstanceEN3G3D15CoordinateFrameEED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,G3D::CoordinateFrame>::~PropDescriptor()")]
pub fn stub_0x5f3290() {
    // IDA 0x5f3290: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x5f32bc — __ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D15CoordinateFrameEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::CoordinateFrame>::isReadOnly(void)const")]
pub fn stub_0x5f32bc() {
    // IDA 0x5f32bc: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x5f32cc — __ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D15CoordinateFrameEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::CoordinateFrame>::isWriteOnly(void)const")]
pub fn stub_0x5f32cc() {
    // IDA 0x5f32cc: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x5f32dc — __ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D15CoordinateFrameEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::CoordinateFrame>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x5f32dc(a: &Prop<CoordinateFrame>, b: &Prop<CoordinateFrame>) -> bool {
    // IDA 0x5f32dc: `equalValues`: `getValue` both sides via slot 8, compare
    // the translation lanes, then `G3D::Matrix3::operator==` on the rotation
    // (0x5f3356-0x5f338c).
    a.value == b.value
}

// 0x5f33d4 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D15CoordinateFrameEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::CoordinateFrame>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x5f33d4(prop: &Prop<CoordinateFrame>) -> Value {
    // IDA 0x5f33d4: `getVariant`: 48-byte frame temp via slot 8 (0x5f33e8),
    // tag `Type::getSingleton<CoordinateFrame>`, pack with
    // `placement_any::operator=<CoordinateFrame>` (0x5f33fc-0x5f3408).
    Value::CoordinateFrame(prop.value)
}

// 0x5f340c — __ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D15CoordinateFrameEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::CoordinateFrame>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x5f340c(prop: &mut Prop<CoordinateFrame>, value: &Value) {
    // IDA 0x5f340c: `setVariant`: `any_cast<CoordinateFrame>` on a frame
    // payload (typeinfo + `"N3G3D15CoordinateFrameE"` check, 0x5f3496;
    // `Matrix3` copy at 0x5f3508), else `Variant::convert<CoordinateFrame>`
    // on a copied variant, then `setValue`.
    prop.value = value.as_coordinate_frame();
}

// 0x5f3598 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D15CoordinateFrameEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::CoordinateFrame>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x5f3598(dst: &mut Prop<CoordinateFrame>, src: &Prop<CoordinateFrame>) {
    // IDA 0x5f3598: `copyValue`: 48-byte frame temp via slot 8
    // (0x5f35ae), `setValue` into the destination via slot 12 (0x5f35be).
    dst.value = src.value;
}

// 0x5f3760 — __ZN3RBX10Reflection23TypedPropertyDescriptorIN3G3D15CoordinateFrameEED1Ev
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::CoordinateFrame>::~TypedPropertyDescriptor()")]
pub fn stub_0x5f3760() {
    // IDA 0x5f3760: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x5f3784 — __ZN3RBX10Reflection23TypedPropertyDescriptorIN3G3D15CoordinateFrameEED0Ev
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::CoordinateFrame>::~TypedPropertyDescriptor()")]
pub fn stub_0x5f3784() {
    // IDA 0x5f3784: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x5f37b0 — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEN3G3D15CoordinateFrameEE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(G3D::CoordinateFrame const&)>::isReadOnly(void)const")]
pub fn stub_0x5f37b0() -> bool {
    // IDA 0x5f37b0: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0x5f37b4 — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEN3G3D15CoordinateFrameEE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(G3D::CoordinateFrame const&)>::isWriteOnly(void)const")]
pub fn stub_0x5f37b4() -> bool {
    // IDA 0x5f37b4: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0x5f37b8 — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEN3G3D15CoordinateFrameEE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(G3D::CoordinateFrame const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x5f37b8(prop: &Prop<CoordinateFrame>) -> CoordinateFrame {
    // IDA 0x5f37b8: `GetSetImpl<CoordinateFrame>::getValue`: header strip,
    // getter member-pointer decode, invoke.
    prop.value
}

// 0x5f37f4 — __ZNK3RBX10Reflection14PropDescriptorINS_12PartInstanceEN3G3D15CoordinateFrameEE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE8setValueEPNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PartInstance,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::PartInstance::*)(void)const,void (RBX::PartInstance::*)(G3D::CoordinateFrame const&)>::setValue(RBX::Reflection::DescribedBase *,G3D::CoordinateFrame const&)const")]
pub fn stub_0x5f37f4(prop: &mut Prop<CoordinateFrame>, value: CoordinateFrame) {
    // IDA 0x5f37f4: `GetSetImpl<CoordinateFrame>::setValue`: header strip,
    // setter member-pointer decode, invoke.
    prop.value = value;
}

// 0x5f9108 — __ZN3RBX10Reflection14PropDescriptorINS_15PhysicsSettingsEbED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,bool>::~PropDescriptor()")]
pub fn stub_0x5f9108() {
    // IDA 0x5f9108: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x5f912c — __ZN3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::~EnumPropDescriptor()")]
pub fn stub_0x5f912c() {
    // IDA 0x5f912c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x5f915c — __ZN3RBX10Reflection14PropDescriptorINS_15PhysicsSettingsEdED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,double>::~PropDescriptor()")]
pub fn stub_0x5f915c() {
    // IDA 0x5f915c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x5f9ba0 — __ZN3RBX10Reflection14PropDescriptorINS_15PhysicsSettingsEdEC2IMS2_KFdvEMS2_FvdEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,double>::PropDescriptor<double (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(double)>(char const*,char const*,double (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(double),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x5f9ba0(
    name: &str,
    category: &str,
    initial: f64,
    attributes: u32,
    permissions: u32,
) -> Prop<f64> {
    // IDA 0x5f9ba0: `PropDescriptor<PhysicsSettings, double>` get/set ctor:
    // `new` the GetSetImpl, forward into the `TypedPropertyDescriptor`
    // ctor. Same shape as the PartInstance float twin at 0x5f0cec.
    Prop::new(name, category, initial, attributes, permissions)
}

// 0x5f9cb4 — __ZN3RBX10Reflection14PropDescriptorINS_15PhysicsSettingsEdED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,double>::~PropDescriptor()")]
pub fn stub_0x5f9cb4() {
    // IDA 0x5f9cb4: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x5f9ce0 — __ZNK3RBX10Reflection14PropDescriptorINS_15PhysicsSettingsEdE10GetSetImplIMS2_KFdvEMS2_FvdEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,double>::GetSetImpl<double (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(double)>::isReadOnly(void)const")]
pub fn stub_0x5f9ce0() -> bool {
    // IDA 0x5f9ce0: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0x5f9ce4 — __ZNK3RBX10Reflection14PropDescriptorINS_15PhysicsSettingsEdE10GetSetImplIMS2_KFdvEMS2_FvdEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,double>::GetSetImpl<double (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(double)>::isWriteOnly(void)const")]
pub fn stub_0x5f9ce4() -> bool {
    // IDA 0x5f9ce4: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0x5f9ce8 — __ZNK3RBX10Reflection14PropDescriptorINS_15PhysicsSettingsEdE10GetSetImplIMS2_KFdvEMS2_FvdEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,double>::GetSetImpl<double (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(double)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x5f9ce8(prop: &Prop<f64>) -> f64 {
    // IDA 0x5f9ce8: `GetSetImpl<double>::getValue`: header strip, getter
    // member-pointer decode, invoke.
    prop.value
}

// 0x5f9d08 — __ZNK3RBX10Reflection14PropDescriptorINS_15PhysicsSettingsEdE10GetSetImplIMS2_KFdvEMS2_FvdEE8setValueEPNS0_13DescribedBaseERKd
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,double>::GetSetImpl<double (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(double)>::setValue(RBX::Reflection::DescribedBase *,double const&)const")]
pub fn stub_0x5f9d08(prop: &mut Prop<f64>, value: f64) {
    // IDA 0x5f9d08: `GetSetImpl<double>::setValue`: header strip, setter
    // member-pointer decode, invoke.
    prop.value = value;
}

// 0x5f9d30 — __ZN3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::EnumPropDescriptor<RBX::EThrottle::EThrottleType (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(RBX::EThrottle::EThrottleType)>(char const*,char const*,RBX::EThrottle::EThrottleType (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(RBX::EThrottle::EThrottleType),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x5f9d30(
    name: &str,
    category: &str,
    initial: i32,
    attributes: u32,
    permissions: u32,
) -> EnumProp {
    // IDA 0x5f9d30: `EnumPropDescriptor<PhysicsSettings, EThrottleType>`
    // ctor: `new` the GetSetImpl holding the getter/setter pair, link the
    // `EnumDesc<EThrottleType>` singleton at +40/+48 (same shape as 0x4a5834).
    // The item pairs register in the singleton C2 (cf. 0x4aaef8); the table
    // here clones that singleton link.
    EnumProp::new(name, category, initial, crate::descriptor::stub_0x4aaef8().clone(), attributes, permissions)
}

// 0x5f9ee4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::~EnumPropDescriptor()")]
pub fn stub_0x5f9ee4() {
    // IDA 0x5f9ee4: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x5f9f10 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::isReadOnly(void)const")]
pub fn stub_0x5f9f10() {
    // IDA 0x5f9f10: EnumPropDescriptor::isReadOnly/isWriteOnly -- forwards through the bound member descriptor at +44 (`(**(this+44))(this+44)`, decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x5f9f20 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::isWriteOnly(void)const")]
pub fn stub_0x5f9f20() {
    // IDA 0x5f9f20: EnumPropDescriptor::isReadOnly/isWriteOnly -- forwards through the bound member descriptor at +44 (`(**(this+44))(this+44)`, decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x5f9f30 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x5f9f30(a: &EnumProp, b: &EnumProp) -> bool {
    // IDA 0x5f9f30: `equalValues`: get both sides via the +44 member,
    // compare (same shape as 0x4a9fe0).
    a.value == b.value
}

// 0x5f9f58 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x5f9f58(prop: &EnumProp) -> Value {
    // IDA 0x5f9f58: `getVariant`: get via the +44 member, tag
    // `Type::getSingleton<int>`, `placement_any<int>` (same shape as
    // 0x4aa008).
    Value::Int(prop.value)
}

// 0x5f9f7c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x5f9f7c(prop: &mut EnumProp, value: &Value) {
    // IDA 0x5f9f7c: `setVariant`: int payloads use `any_cast<int>` directly,
    // else `Variant::convert<int>`, then set (same shape as 0x4aa02c).
    prop.value = match value {
        Value::Int(v) => *v,
        Value::EnumValue(v) => *v,
        Value::Float(v) => *v as i32,
        Value::Bool(v) => *v as i32,
        other => panic!("Variant::convert<int> on {other:?} (IDA 0x5f9f7c)"),
    };
}

// 0x5fa0c8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE9copyValueEPKNS0_13DescribedBaseEPS6_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x5fa0c8(dst: &mut EnumProp, src: &EnumProp) {
    // IDA 0x5fa0c8: `copyValue`: get via the +44 member, set into the
    // destination (same shape as 0x4aa178).
    dst.value = src.value;
}

// 0x5fa0ec — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::hasStringValue(void)const")]
pub fn stub_0x5fa0ec() -> bool {
    // IDA 0x5fa0ec: EnumPropDescriptor::hasStringValue -- hardcoded `return 1` (decompiled 0x10244/0x10dc8/0x11650).
    true
}

// 0x5fa0f0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x5fa0f0(prop: &EnumProp) -> String {
    // IDA 0x5fa0f0: `getStringValue`: get via the +44 member, then
    // `EnumDesc<EThrottleType>::convertToString` (same shape as 0x4aa1a0).
    prop.enum_desc.lookup_name(prop.value).unwrap_or_default().to_owned()
}

// 0x5fa114 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_0x5fa114(prop: &mut EnumProp, name: &str) -> bool {
    // IDA 0x5fa114: `setStringValue`: `Name::lookup`, `convertToValue`; on
    // 1 set and return 1, else 0 (same shape as 0x4aa1c4).
    match prop.enum_desc.lookup_value(name) {
        Some(v) => {
            prop.value = v;
            true
        }
        None => false,
    }
}

// 0x5fa154 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_0x5fa154(prop: &EnumProp) -> i32 {
    // IDA 0x5fa154: `writeValue`: get via the +44 member, `clearValue`,
    // store the int tag + value, return it (same shape as 0x4aa204).
    prop.value
}

// 0x5fa174 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x5fa174(prop: &mut EnumProp, text: &str) -> bool {
    // IDA 0x5fa174: `readValue`: element text into a string, `Name::lookup`,
    // `convertToValue`; success sets (same shape as 0x4aa224). Empty/missing
    // text leaves the object untouched.
    match prop.enum_desc.lookup_value(text) {
        Some(v) => {
            prop.value = v;
            true
        }
        None => false,
    }
}

// 0x5fa3b4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x5fa3b4(prop: &EnumProp) -> i32 {
    // IDA 0x5fa3b4: `getIndexValue` tail-jumps to
    // `EnumDesc<EThrottleType>::convertToIndex` (same shape as 0x4aa464).
    prop.convert_to_index(prop.value)
}

// 0x5fa3d0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_0x5fa3d0(prop: &mut EnumProp, index: usize) -> bool {
    // IDA 0x5fa3d0: `setIndexValue`: bounds-check against the enum count,
    // load `values[index]`, set, return 1; else 0 (same shape as 0x4aa480).
    match prop.enum_desc.values.get(index) {
        Some(&v) => {
            prop.value = v;
            true
        }
        None => false,
    }
}

// 0x5fa404 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x5fa404(prop: &EnumProp) -> i32 {
    // IDA 0x5fa404: `getEnumValue` tail-jumps to the +44 member get
    // (same shape as 0x4aa4b4).
    prop.value
}

// 0x5fa40c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x5fa40c(prop: &mut EnumProp, value: i32) -> bool {
    // IDA 0x5fa40c: `setEnumValue`: `find_if(items, equalValue)`; hit sets
    // and returns 1, miss returns 0 (same shape as 0x4aa4bc).
    if prop.enum_desc.items.iter().any(|it| it.value == value) {
        prop.value = value;
        true
    } else {
        false
    }
}

// 0x5fa458 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x5fa458(prop: &EnumProp) -> Option<crate::enum_desc::EnumItem> {
    // IDA 0x5fa458: `getEnumItem`: get the value, return
    // `convertToItem(enumdesc, &v)` (same shape as 0x4aa508).
    usize::try_from(prop.value)
        .ok()
        .and_then(|slot| prop.enum_desc.items_by_value.get(slot).copied().flatten())
        .and_then(|idx| prop.enum_desc.items.get(idx).cloned())
}

// 0x5fa478 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_0x5fa478(prop: &mut EnumProp, name: &str) -> bool {
    // IDA 0x5fa478: `setStringValue` (`Name` overload): `convertToValue`,
    // success sets and returns 1, else 0 (same shape as 0x4aa528).
    match prop.enum_desc.lookup_value(name) {
        Some(v) => {
            prop.value = v;
            true
        }
        None => false,
    }
}

// 0x5fa4ac — __ZNK3RBX10Reflection8EnumDescINS_9EThrottle13EThrottleTypeEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::convertToIndex(RBX::EThrottle::EThrottleType)const")]
pub fn stub_0x5fa4ac(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0x5fa4ac: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0x5fa51c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x5fa51c(prop: &mut EnumProp, value: i32) -> bool {
    // IDA 0x5fa51c: `setIntValue`: `value >= 0` and `value <
    // value_to_value.size`, load `mapped = value_to_value[value]`;
    // `mapped == -1` returns 0, else set and return 1 (same shape as
    // 0x4aa55c).
    match usize::try_from(value)
        .ok()
        .and_then(|slot| prop.enum_desc.value_to_value.get(slot).copied())
    {
        Some(mapped) if mapped != -1 => {
            prop.value = mapped;
            true
        }
        _ => false,
    }
}

// 0x5fa55c — __ZNK3RBX10Reflection14PropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::GetSetImpl<RBX::EThrottle::EThrottleType (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(RBX::EThrottle::EThrottleType)>::isReadOnly(void)const")]
pub fn stub_0x5fa55c() -> bool {
    // IDA 0x5fa55c: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0x5fa560 — __ZNK3RBX10Reflection14PropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::GetSetImpl<RBX::EThrottle::EThrottleType (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(RBX::EThrottle::EThrottleType)>::isWriteOnly(void)const")]
pub fn stub_0x5fa560() -> bool {
    // IDA 0x5fa560: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0x5fa564 — __ZNK3RBX10Reflection14PropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::GetSetImpl<RBX::EThrottle::EThrottleType (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(RBX::EThrottle::EThrottleType)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x5fa564(prop: &EnumProp) -> i32 {
    // IDA 0x5fa564: `GetSetImpl<EThrottleType>::getValue` (+44 member):
    // header strip, getter member-pointer decode, invoke.
    prop.value
}

// 0x5fa584 — __ZNK3RBX10Reflection14PropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::GetSetImpl<RBX::EThrottle::EThrottleType (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(RBX::EThrottle::EThrottleType)>::setValue(RBX::Reflection::DescribedBase *,RBX::EThrottle::EThrottleType const&)const")]
pub fn stub_0x5fa584(prop: &mut EnumProp, value: i32) {
    // IDA 0x5fa584: `GetSetImpl<EThrottleType>::setValue` (+44 member):
    // header strip, setter member-pointer decode, invoke.
    prop.value = value;
}

// 0x5fa5a8 — __ZN3RBX10Reflection14PropDescriptorINS_15PhysicsSettingsEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,bool>::PropDescriptor<bool (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(bool)>(char const*,char const*,bool (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x5fa5a8(
    name: &str,
    category: &str,
    initial: bool,
    attributes: u32,
    permissions: u32,
) -> Prop<bool> {
    // IDA 0x5fa5a8: `PropDescriptor<PhysicsSettings, bool>` get/set ctor:
    // `new` the GetSetImpl, forward into the `TypedPropertyDescriptor`
    // ctor. Same shape as the PartInstance float twin at 0x5f0cec.
    Prop::new(name, category, initial, attributes, permissions)
}

// 0x5fa6bc — __ZN3RBX10Reflection14PropDescriptorINS_15PhysicsSettingsEbED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,bool>::~PropDescriptor()")]
pub fn stub_0x5fa6bc() {
    // IDA 0x5fa6bc: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x5fa6e8 — __ZNK3RBX10Reflection14PropDescriptorINS_15PhysicsSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,bool>::GetSetImpl<bool (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(bool)>::isReadOnly(void)const")]
pub fn stub_0x5fa6e8() -> bool {
    // IDA 0x5fa6e8: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0x5fa6ec — __ZNK3RBX10Reflection14PropDescriptorINS_15PhysicsSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,bool>::GetSetImpl<bool (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(bool)>::isWriteOnly(void)const")]
pub fn stub_0x5fa6ec() -> bool {
    // IDA 0x5fa6ec: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0x5fa6f0 — __ZNK3RBX10Reflection14PropDescriptorINS_15PhysicsSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,bool>::GetSetImpl<bool (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x5fa6f0(prop: &Prop<bool>) -> bool {
    // IDA 0x5fa6f0: `GetSetImpl<bool>::getValue` for PhysicsSettings:
    // header strip, getter member-pointer decode, invoke.
    prop.value
}

// 0x5fa714 — __ZNK3RBX10Reflection14PropDescriptorINS_15PhysicsSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,bool>::GetSetImpl<bool (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
pub fn stub_0x5fa714(prop: &mut Prop<bool>, value: bool) {
    // IDA 0x5fa714: `GetSetImpl<bool>::setValue` for PhysicsSettings:
    // header strip, setter member-pointer decode, invoke.
    prop.value = value;
}

// 0x5fc1f4 — __ZN3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::StarterGuiService::CoreGuiType>::EnumDesc(void)")]
pub fn stub_0x5fc1f4() -> crate::enum_desc::EnumDesc {
    // IDA 0x5fc1f4: EnumDesc<T>::C1 -- EnumDescriptor base ctor with name "Enum", vtable install, empty tables (decompiled; cf. 0x37148c). Pairs are registered by the addPair stubs.
    crate::enum_desc::EnumDesc::new("Enum")
}

// 0x5fc1f8 — __ZN3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::StarterGuiService::CoreGuiType>::EnumDesc(void)")]
pub fn stub_0x5fc1f8() -> crate::enum_desc::EnumDesc {
    // IDA 0x5fc1f8: EnumDesc<T>::C2 -- EnumDescriptor base ctor with name "CoreGuiType", vtable install, empty tables (decompiled; cf. 0x37148c). Pairs are registered by the addPair stubs.
    crate::enum_desc::EnumDesc::new("CoreGuiType")
}

// 0x5fcc38 — __ZN3RBX10Reflection14PropDescriptorINS_17StarterGuiServiceEbED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::StarterGuiService,bool>::~PropDescriptor()")]
pub fn stub_0x5fcc38() {
    // IDA 0x5fcc38: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x5fcc64 — __ZN3RBX10Reflection13BoundFuncDescINS_17StarterGuiServiceEFvNS2_11CoreGuiTypeEbELi2EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::StarterGuiService,void ()(RBX::StarterGuiService::CoreGuiType,bool),2>::~BoundFuncDesc()")]
pub fn stub_0x5fcc64() {
    // IDA 0x5fcc64: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x5fccac — __ZN3RBX10Reflection13BoundFuncDescINS_17StarterGuiServiceEFbNS2_11CoreGuiTypeEELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::StarterGuiService,bool ()(RBX::StarterGuiService::CoreGuiType),1>::~BoundFuncDesc()")]
pub fn stub_0x5fccac() {
    // IDA 0x5fccac: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

/// `Singleton<EnumDesc<NumSidesEnum>>` link for the Pyramid suite below (cf.
/// 0x4aaef8): guard-once table; item pairs register in the singleton C2 and
/// are unmodeled here, same as the other name-only tables in this crate.
static NUM_SIDES_DESC: std::sync::LazyLock<crate::enum_desc::EnumDesc> =
    std::sync::LazyLock::new(|| crate::enum_desc::EnumDesc::new("NumSidesEnum"));

pub fn num_sides_enum_prop(
    name: &str,
    category: &str,
    initial: i32,
    attributes: u32,
    permissions: u32,
) -> EnumProp {
    EnumProp::new(name, category, initial, NUM_SIDES_DESC.clone(), attributes, permissions)
}

// 0x60a614 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x60a614(dst: &mut EnumProp, src: &EnumProp) {
    // IDA 0x60a614: `copyValue` get-then-set (same shape as
    // 0x4aa178/0x5fa0c8).
    dst.value = src.value;
}

// 0x60a638 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::hasStringValue(void)const")]
pub fn stub_0x60a638() -> bool {
    // IDA 0x60a638: EnumPropDescriptor::hasStringValue -- hardcoded `return 1` (decompiled 0x10244/0x10dc8/0x11650).
    true
}

// 0x60a63c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x60a63c(prop: &EnumProp) -> String {
    // IDA 0x60a63c: `getStringValue` via `convertToString` (same shape as
    // 0x4aa1a0/0x5fa0f0).
    prop.enum_desc.lookup_name(prop.value).unwrap_or_default().to_owned()
}

// 0x60a660 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_0x60a660(prop: &mut EnumProp, name: &str) -> bool {
    // IDA 0x60a660: `setStringValue` lookup-and-set (same shape as
    // 0x4aa1c4/0x5fa114).
    match prop.enum_desc.lookup_value(name) {
        Some(v) => {
            prop.value = v;
            true
        }
        None => false,
    }
}

// 0x60a6a0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_0x60a6a0(prop: &EnumProp) -> i32 {
    // IDA 0x60a6a0: `writeValue` get + int tag store (same shape as
    // 0x4aa204/0x5fa154).
    prop.value
}

// 0x60a6c0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x60a6c0(prop: &mut EnumProp, text: &str) -> bool {
    // IDA 0x60a6c0: `readValue` text-lookup-and-set (same shape as
    // 0x4aa224/0x5fa174).
    match prop.enum_desc.lookup_value(text) {
        Some(v) => {
            prop.value = v;
            true
        }
        None => false,
    }
}

// 0x60a900 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x60a900(prop: &EnumProp) -> i32 {
    // IDA 0x60a900: `getIndexValue` via `convertToIndex` at 0x60a9f8
    // (same shape as 0x4aa464/0x5fa3b4).
    prop.convert_to_index(prop.value)
}

// 0x60a91c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_0x60a91c(prop: &mut EnumProp, index: usize) -> bool {
    // IDA 0x60a91c: `setIndexValue` bounds-check + set (same shape as
    // 0x4aa480/0x5fa3d0).
    match prop.enum_desc.values.get(index) {
        Some(&v) => {
            prop.value = v;
            true
        }
        None => false,
    }
}

// 0x60a950 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x60a950(prop: &EnumProp) -> i32 {
    // IDA 0x60a950: `getEnumValue` tail-jump to the +44 member get
    // (same shape as 0x4aa4b4/0x5fa404).
    prop.value
}

// 0x60a958 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x60a958(prop: &mut EnumProp, value: i32) -> bool {
    // IDA 0x60a958: `setEnumValue` find-and-set (same shape as
    // 0x4aa4bc/0x5fa40c).
    if prop.enum_desc.items.iter().any(|it| it.value == value) {
        prop.value = value;
        true
    } else {
        false
    }
}

// 0x60a9a4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x60a9a4(prop: &EnumProp) -> Option<crate::enum_desc::EnumItem> {
    // IDA 0x60a9a4: `getEnumItem` via `convertToItem` (same shape as
    // 0x4aa508/0x5fa458).
    usize::try_from(prop.value)
        .ok()
        .and_then(|slot| prop.enum_desc.items_by_value.get(slot).copied().flatten())
        .and_then(|idx| prop.enum_desc.items.get(idx).cloned())
}

// 0x60a9c4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_0x60a9c4(prop: &mut EnumProp, name: &str) -> bool {
    // IDA 0x60a9c4: `setStringValue` (`Name` overload, same shape as
    // 0x4aa528/0x5fa478).
    match prop.enum_desc.lookup_value(name) {
        Some(v) => {
            prop.value = v;
            true
        }
        None => false,
    }
}

// 0x60a9f8 — __ZNK3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::convertToIndex(RBX::PyramidInstance::NumSidesEnum)const")]
pub fn stub_0x60a9f8(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0x60a9f8: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0x60aa68 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x60aa68(prop: &mut EnumProp, value: i32) -> bool {
    // IDA 0x60aa68: `setIntValue`: `value_to_value` map, -1 rejects
    // (same shape as 0x4aa55c/0x5fa51c).
    match usize::try_from(value)
        .ok()
        .and_then(|slot| prop.enum_desc.value_to_value.get(slot).copied())
    {
        Some(mapped) if mapped != -1 => {
            prop.value = mapped;
            true
        }
        _ => false,
    }
}

// 0x60aaa8 — __ZNK3RBX10Reflection14PropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::GetSetImpl<RBX::PyramidInstance::NumSidesEnum (RBX::PyramidInstance::*)(void)const,void (RBX::PyramidInstance::*)(RBX::PyramidInstance::NumSidesEnum)>::isReadOnly(void)const")]
pub fn stub_0x60aaa8() -> bool {
    // IDA 0x60aaa8: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0x60aaac — __ZNK3RBX10Reflection14PropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::GetSetImpl<RBX::PyramidInstance::NumSidesEnum (RBX::PyramidInstance::*)(void)const,void (RBX::PyramidInstance::*)(RBX::PyramidInstance::NumSidesEnum)>::isWriteOnly(void)const")]
pub fn stub_0x60aaac() -> bool {
    // IDA 0x60aaac: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0x60aab0 — __ZNK3RBX10Reflection14PropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::GetSetImpl<RBX::PyramidInstance::NumSidesEnum (RBX::PyramidInstance::*)(void)const,void (RBX::PyramidInstance::*)(RBX::PyramidInstance::NumSidesEnum)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x60aab0(prop: &EnumProp) -> i32 {
    // IDA 0x60aab0: `GetSetImpl<NumSidesEnum>::getValue` (+44 member):
    // header strip, getter member-pointer decode, invoke.
    prop.value
}

// 0x60aad0 — __ZNK3RBX10Reflection14PropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::GetSetImpl<RBX::PyramidInstance::NumSidesEnum (RBX::PyramidInstance::*)(void)const,void (RBX::PyramidInstance::*)(RBX::PyramidInstance::NumSidesEnum)>::setValue(RBX::Reflection::DescribedBase *,RBX::PyramidInstance::NumSidesEnum const&)const")]
pub fn stub_0x60aad0(prop: &mut EnumProp, value: i32) {
    // IDA 0x60aad0: `GetSetImpl<NumSidesEnum>::setValue` (+44 member):
    // header strip, setter member-pointer decode, invoke.
    prop.value = value;
}

// 0x60d780 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4TeamES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Team,RBX::Team>(rbx_core::SharedPtr<RBX::Team> const*,RBX::Team *)const")]
pub fn stub_0x60d780() {
    // IDA 0x60d780: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0x60efc8 — __ZN3RBX10Reflection14PropDescriptorINS_11Scale9FrameEN3G3D12Vector2int16EED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scale9Frame,G3D::Vector2int16>::~PropDescriptor()")]
pub fn stub_0x60efc8() {
    // IDA 0x60efc8: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x60effc — __ZN3RBX10Reflection14PropDescriptorINS_11Scale9FrameESsED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scale9Frame,std::string>::~PropDescriptor()")]
pub fn stub_0x60effc() {
    // IDA 0x60effc: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x60f9bc — __ZN3RBX10Reflection14PropDescriptorINS_11Scale9FrameESsEC2IMS2_KFSsvEMS2_FvSsEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scale9Frame,std::string>::PropDescriptor<std::string (RBX::Scale9Frame::*)(void)const,void (RBX::Scale9Frame::*)(std::string)>(char const*,char const*,std::string (RBX::Scale9Frame::*)(void)const,void (RBX::Scale9Frame::*)(std::string),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x60f9bc(
    name: &str,
    category: &str,
    initial: String,
    attributes: u32,
    permissions: u32,
) -> Prop<String> {
    // IDA 0x60f9bc: `PropDescriptor<Scale9Frame, string>` get/set ctor:
    // `new` the GetSetImpl, forward into the `TypedPropertyDescriptor`
    // ctor. Same shape as 0x5f0cec.
    Prop::new(name, category, initial, attributes, permissions)
}

// 0x60fad0 — __ZN3RBX10Reflection14PropDescriptorINS_11Scale9FrameESsED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scale9Frame,std::string>::~PropDescriptor()")]
pub fn stub_0x60fad0() {
    // IDA 0x60fad0: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x60fafc — __ZNK3RBX10Reflection14PropDescriptorINS_11Scale9FrameESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scale9Frame,std::string>::GetSetImpl<std::string (RBX::Scale9Frame::*)(void)const,void (RBX::Scale9Frame::*)(std::string)>::isReadOnly(void)const")]
pub fn stub_0x60fafc() -> bool {
    // IDA 0x60fafc: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0x60fb00 — __ZNK3RBX10Reflection14PropDescriptorINS_11Scale9FrameESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scale9Frame,std::string>::GetSetImpl<std::string (RBX::Scale9Frame::*)(void)const,void (RBX::Scale9Frame::*)(std::string)>::isWriteOnly(void)const")]
pub fn stub_0x60fb00() -> bool {
    // IDA 0x60fb00: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0x60fb04 — __ZNK3RBX10Reflection14PropDescriptorINS_11Scale9FrameESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scale9Frame,std::string>::GetSetImpl<std::string (RBX::Scale9Frame::*)(void)const,void (RBX::Scale9Frame::*)(std::string)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x60fb04(prop: &Prop<String>) -> String {
    // IDA 0x60fb04: `GetSetImpl<string>::getValue`: header strip, getter
    // member-pointer decode, invoke.
    prop.value.clone()
}

// 0x60fb2c — __ZNK3RBX10Reflection14PropDescriptorINS_11Scale9FrameESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE8setValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scale9Frame,std::string>::GetSetImpl<std::string (RBX::Scale9Frame::*)(void)const,void (RBX::Scale9Frame::*)(std::string)>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_0x60fb2c(prop: &mut Prop<String>, value: String) {
    // IDA 0x60fb2c: `GetSetImpl<string>::setValue`: header strip, setter
    // member-pointer decode, invoke with the new value.
    prop.value = value;
}

// 0x60fc70 — __ZN3RBX10Reflection14PropDescriptorINS_11Scale9FrameEN3G3D12Vector2int16EEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scale9Frame,G3D::Vector2int16>::PropDescriptor<G3D::Vector2int16 (RBX::Scale9Frame::*)(void)const,void (RBX::Scale9Frame::*)(G3D::Vector2int16)>(char const*,char const*,G3D::Vector2int16 (RBX::Scale9Frame::*)(void)const,void (RBX::Scale9Frame::*)(G3D::Vector2int16),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x60fc70(
    name: &str,
    category: &str,
    initial: [i16; 2],
    attributes: u32,
    permissions: u32,
) -> Prop<[i16; 2]> {
    // IDA 0x60fc70: `PropDescriptor<Scale9Frame, Vector2int16>` get/set
    // ctor: `new` the GetSetImpl, forward into the `TypedPropertyDescriptor`
    // ctor. Same shape as 0x5f0cec.
    Prop::new(name, category, initial, attributes, permissions)
}

// 0x60fd84 — __ZN3RBX10Reflection14PropDescriptorINS_11Scale9FrameEN3G3D12Vector2int16EED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scale9Frame,G3D::Vector2int16>::~PropDescriptor()")]
pub fn stub_0x60fd84() {
    // IDA 0x60fd84: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x60fdb0 — __ZNK3RBX10Reflection14PropDescriptorINS_11Scale9FrameEN3G3D12Vector2int16EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scale9Frame,G3D::Vector2int16>::GetSetImpl<G3D::Vector2int16 (RBX::Scale9Frame::*)(void)const,void (RBX::Scale9Frame::*)(G3D::Vector2int16)>::isReadOnly(void)const")]
pub fn stub_0x60fdb0() -> bool {
    // IDA 0x60fdb0: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0x60fdb4 — __ZNK3RBX10Reflection14PropDescriptorINS_11Scale9FrameEN3G3D12Vector2int16EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scale9Frame,G3D::Vector2int16>::GetSetImpl<G3D::Vector2int16 (RBX::Scale9Frame::*)(void)const,void (RBX::Scale9Frame::*)(G3D::Vector2int16)>::isWriteOnly(void)const")]
pub fn stub_0x60fdb4() -> bool {
    // IDA 0x60fdb4: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0x60fdb8 — __ZNK3RBX10Reflection14PropDescriptorINS_11Scale9FrameEN3G3D12Vector2int16EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scale9Frame,G3D::Vector2int16>::GetSetImpl<G3D::Vector2int16 (RBX::Scale9Frame::*)(void)const,void (RBX::Scale9Frame::*)(G3D::Vector2int16)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x60fdb8(prop: &Prop<[i16; 2]>) -> [i16; 2] {
    // IDA 0x60fdb8: `GetSetImpl<Vector2int16>::getValue`: header strip,
    // getter member-pointer decode, invoke.
    prop.value
}

// 0x60fde0 — __ZNK3RBX10Reflection14PropDescriptorINS_11Scale9FrameEN3G3D12Vector2int16EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scale9Frame,G3D::Vector2int16>::GetSetImpl<G3D::Vector2int16 (RBX::Scale9Frame::*)(void)const,void (RBX::Scale9Frame::*)(G3D::Vector2int16)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector2int16 const&)const")]
pub fn stub_0x60fde0(prop: &mut Prop<[i16; 2]>, value: [i16; 2]) {
    // IDA 0x60fde0: `GetSetImpl<Vector2int16>::setValue`: header strip,
    // setter member-pointer decode, invoke.
    prop.value = value;
}

use rbx_core::signal::Signal;

/// Which `GuiBase2d` layout prop changed (IDA 0x61044c).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenGuiLayoutProp {
    AbsoluteSize,
    AbsolutePosition,
    Other,
}

/// `RBX::GuiButton` modal state visible to ScreenGui (IDA 0x6106ee reads the
/// modal flag at +782 to pick insert vs remove).
#[derive(Debug, Clone, Default)]
pub struct GuiButtonState {
    pub id: u32,
    pub modal: bool,
}

/// `rbx::signals::signal<void(const PropertyDescriptor*)>` for ScreenGui
/// (IDA 0x610cd8): owns the connected slots; strong refs live in `holders`
/// because `Signal::connect` keeps only weak refs (same shape as
/// `EventSource` in descriptor.rs).
#[derive(Default)]
pub struct ScreenGuiPropSignal {
    signal: Signal<String>,
    holders: parking_lot::Mutex<Vec<SharedPtr<dyn Fn(String) + Send + Sync>>>,
}

impl ScreenGuiPropSignal {
    pub fn fire(&self, prop: &str) {
        self.signal.fire(prop.to_owned());
    }

    pub fn disconnect_all(&self) {
        self.holders.lock().clear();
        self.signal.disconnect_all();
    }
}

// 0x61044c — __ZN3RBX9ScreenGui17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
#[doc(alias = "RBX::ScreenGui::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_0x61044c(changed: ScreenGuiLayoutProp, raised: &mut Vec<String>) -> bool {
    // IDA 0x61044c: `onPropertyChanged`: `AbsoluteSize` recomputes layout
    // (vf+152 on +176, 0x6104a2) and raises `unk_1326300` (0x6104ae);
    // `AbsolutePosition` recomputes (0x610482) and raises `unk_132632C`
    // (0x61048e); anything else returns without raising (0x610470).
    match changed {
        ScreenGuiLayoutProp::AbsoluteSize => {
            raised.push("AbsoluteSize".to_owned());
            true
        }
        ScreenGuiLayoutProp::AbsolutePosition => {
            raised.push("AbsolutePosition".to_owned());
            true
        }
        ScreenGuiLayoutProp::Other => false,
    }
}

// 0x6106ec — __ZN3RBX9ScreenGui20onModalButtonChangedEPKNS_10Reflection18PropertyDescriptorEPNS_9GuiButtonE
#[doc(alias = "RBX::ScreenGui::onModalButtonChanged(RBX::Reflection::PropertyDescriptor const*,RBX::GuiButton *)")]
pub fn stub_0x6106ec(buttons: &mut Vec<u32>, button: &GuiButtonState, _prop: &str) {
    // IDA 0x6106ec: `onModalButtonChanged`: the modal flag at `button + 782`
    // picks `insertModalButton` (0x6106fa) vs `removeModalButton` (0x6106f6).
    if button.modal {
        if !buttons.contains(&button.id) {
            buttons.push(button.id);
        }
    } else {
        buttons.retain(|&b| b != button.id);
    }
}

// 0x610bec — __ZN3RBX10Reflection14PropDescriptorINS_9ScreenGuiEN3G3D12Vector2int16EED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ScreenGui,G3D::Vector2int16>::~PropDescriptor()")]
pub fn stub_0x610bec() {
    // IDA 0x610bec: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x610cd8 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE7connectIN5boost3_bi6bind_tIvNSA_4_mfi3mf2IvNS2_9ScreenGuiES6_PNS2_9GuiButtonEEENSB_5list3INSB_5valueIPSF_EENSA_3argILi1EEENSK_ISH_EEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScreenGui,RBX::Reflection::PropertyDescriptor const*,RBX::GuiButton *>,boost::_bi::list3<boost::_bi::value<RBX::ScreenGui*>,boost::arg<1>,boost::_bi::value<RBX::GuiButton *>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScreenGui,RBX::Reflection::PropertyDescriptor const*,RBX::GuiButton *>,boost::_bi::list3<boost::_bi::value<RBX::ScreenGui*>,boost::arg<1>,boost::_bi::value<RBX::GuiButton *>>> const&)")]
pub fn stub_0x610cd8(
    sig: &ScreenGuiPropSignal,
    buttons: SharedPtr<parking_lot::Mutex<Vec<u32>>>,
    button: SharedPtr<parking_lot::Mutex<GuiButtonState>>,
) {
    // IDA 0x610cd8: `signal::connect<bind_t<mf2<ScreenGui,
    // onModalButtonChanged>>>`: `new` the islot holding the bound
    // `(ScreenGui*, arg<1>, GuiButton*)` triple (0x610cf0-0x610d2e),
    // `signal::insert`, hand out the connection (0x610d32-0x610d3c).
    // Rust: `Signal::connect` (concrete closure type); the triple folds
    // into the closure.
    let (live_buttons, live_button) = (SharedPtr::clone(&buttons), SharedPtr::clone(&button));
    let slot = SharedPtr::new(move |prop: String| {
        let state = live_button.lock().clone();
        stub_0x6106ec(&mut live_buttons.lock(), &state, &prop);
    });
    sig.signal.connect(SharedPtr::clone(&slot));
    sig.holders.lock().push(slot);
}

// 0x6126a0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7GuiMainES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GuiMain,RBX::GuiMain>(rbx_core::SharedPtr<RBX::GuiMain> const*,RBX::GuiMain *)const")]
pub fn stub_0x6126a0() {
    // IDA 0x6126a0: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0x613c80 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf2IvNS2_9ScreenGuiES6_PNS2_9GuiButtonEEENSB_5list3INSB_5valueIPSF_EENSA_3argILi1EEENSK_ISH_EEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScreenGui,RBX::Reflection::PropertyDescriptor const*,RBX::GuiButton *>,boost::_bi::list3<boost::_bi::value<RBX::ScreenGui*>,boost::arg<1>,boost::_bi::value<RBX::GuiButton *>>>>::~callable_slot()")]
pub fn stub_0x613c80() {
    // IDA 0x613c80: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x613cac — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf2IvNS2_9ScreenGuiES6_PNS2_9GuiButtonEEENSB_5list3INSB_5valueIPSF_EENSA_3argILi1EEENSK_ISH_EEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScreenGui,RBX::Reflection::PropertyDescriptor const*,RBX::GuiButton *>,boost::_bi::list3<boost::_bi::value<RBX::ScreenGui*>,boost::arg<1>,boost::_bi::value<RBX::GuiButton *>>>>::~callable_slot()")]
pub fn stub_0x613cac() {
    // IDA 0x613cac: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x613d80 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf2IvNS3_9ScreenGuiES7_PNS3_9GuiButtonEEENSC_5list3INSC_5valueIPSG_EENSB_3argILi1EEENSL_ISI_EEEEEELi1ES8_E4callES7_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScreenGui,RBX::Reflection::PropertyDescriptor const*,RBX::GuiButton *>,boost::_bi::list3<boost::_bi::value<RBX::ScreenGui*>,boost::arg<1>,boost::_bi::value<RBX::GuiButton *>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_0x613d80(target: &dyn Fn(&str, &GuiButtonState), prop: &str, button: &GuiButtonState) {
    // IDA 0x613d80: `callable::call`: pack the prop into the 1-arg list and
    // tail-jump to `list3::operator()` (0x613d9e).
    stub_0x613dc0(target, prop, button);
}

// 0x613da0 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf2IvNS3_9ScreenGuiES7_PNS3_9GuiButtonEEENSC_5list3INSC_5valueIPSG_EENSB_3argILi1EEENSL_ISI_EEEEEELi1ES8_E4callES7_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScreenGui,RBX::Reflection::PropertyDescriptor const*,RBX::GuiButton *>,boost::_bi::list3<boost::_bi::value<RBX::ScreenGui*>,boost::arg<1>,boost::_bi::value<RBX::GuiButton *>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_0x613da0() {
    // IDA 0x613da0: non-virtual thunk to `rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScreenGui,RBX::` — this/arg-adjust + tail-call (decompiled). Rust uses static dispatch; no thunk needed. Target unmodeled: cutover no-op.
}

// 0x613dc0 — __ZN5boost3_bi5list3INS0_5valueIPN3RBX9ScreenGuiEEENS_3argILi1EEENS2_IPNS3_9GuiButtonEEEEclINS_4_mfi3mf2IvS4_PKNS3_10Reflection18PropertyDescriptorESA_EENS0_5list1IRSJ_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::ScreenGui *>,boost::arg<1>,boost::_bi::value<RBX::GuiButton *>>::operator()<boost::_mfi::mf2<void,RBX::ScreenGui,RBX::Reflection::PropertyDescriptor const*,RBX::GuiButton *>,boost::_bi::list1<RBX::Reflection::PropertyDescriptor const*&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::ScreenGui,RBX::Reflection::PropertyDescriptor const*,RBX::GuiButton *> &,boost::_bi::list1<RBX::Reflection::PropertyDescriptor const*&> &,int)")]
pub fn stub_0x613dc0(target: &dyn Fn(&str, &GuiButtonState), prop: &str, button: &GuiButtonState) {
    // IDA 0x613dc0: `list3::operator()`: decode the member pair
    // (`offset >> 1`, virtual bit `& 1`), invoke
    // `ScreenGui::onModalButtonChanged(screen, prop, button)`
    // (0x613dd6-0x613de0).
    target(prop, button);
}

// 0x613df0 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf2IvNS3_9ScreenGuiES7_PNS3_9GuiButtonEEENSC_5list3INSC_5valueIPSG_EENSB_3argILi1EEENSL_ISI_EEEEEELi1ES8_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScreenGui,RBX::Reflection::PropertyDescriptor const*,RBX::GuiButton *>,boost::_bi::list3<boost::_bi::value<RBX::ScreenGui*>,boost::arg<1>,boost::_bi::value<RBX::GuiButton *>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
pub fn stub_0x613df0() {
    // IDA 0x613df0: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x613e1c — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf2IvNS3_9ScreenGuiES7_PNS3_9GuiButtonEEENSC_5list3INSC_5valueIPSG_EENSB_3argILi1EEENSL_ISI_EEEEEELi1ES8_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScreenGui,RBX::Reflection::PropertyDescriptor const*,RBX::GuiButton *>,boost::_bi::list3<boost::_bi::value<RBX::ScreenGui*>,boost::arg<1>,boost::_bi::value<RBX::GuiButton *>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
pub fn stub_0x613e1c() {
    // IDA 0x613e1c: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x6141e4 — __ZN3RBX10Reflection14PropDescriptorINS_9ScreenGuiEN3G3D12Vector2int16EEC2IMNS_9GuiBase2dEKFRKNS3_7Vector2EvEMS2_FvS4_EEEPKcSG_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ScreenGui,G3D::Vector2int16>::PropDescriptor<G3D::Vector2 const& (RBX::GuiBase2d::*)(void)const,void (RBX::ScreenGui::*)(G3D::Vector2int16)>(char const*,char const*,G3D::Vector2 const& (RBX::GuiBase2d::*)(void)const,void (RBX::ScreenGui::*)(G3D::Vector2int16),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x6141e4(
    name: &str,
    category: &str,
    initial: [i16; 2],
    attributes: u32,
    permissions: u32,
) -> Prop<[i16; 2]> {
    // IDA 0x6141e4: `PropDescriptor<ScreenGui, Vector2int16>` get/set ctor:
    // `new` the GetSetImpl, forward into the `TypedPropertyDescriptor`
    // ctor. Same shape as 0x5f0cec.
    Prop::new(name, category, initial, attributes, permissions)
}

// 0x6142f8 — __ZN3RBX10Reflection23TypedPropertyDescriptorIN3G3D12Vector2int16EEC2ERNS0_15ClassDescriptorEPKcS8_St8auto_ptrINS4_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2int16>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2int16>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x6142f8(
    name: &str,
    category: &str,
    initial: [i16; 2],
    attributes: u32,
    permissions: u32,
) -> Prop<[i16; 2]> {
    // IDA 0x6142f8: `TypedPropertyDescriptor<Vector2int16>` ctor: type tag,
    // base init, vtable, `auto_ptr` takeover, read/write attribute masking.
    // Same shape as 0x5f0e00.
    Prop::new(name, category, initial, attributes, permissions)
}

// 0x61441c — __ZN3RBX10Reflection14PropDescriptorINS_9ScreenGuiEN3G3D12Vector2int16EED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ScreenGui,G3D::Vector2int16>::~PropDescriptor()")]
pub fn stub_0x61441c() {
    // IDA 0x61441c: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x614448 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D12Vector2int16EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2int16>::isReadOnly(void)const")]
pub fn stub_0x614448() {
    // IDA 0x614448: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x614458 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D12Vector2int16EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2int16>::isWriteOnly(void)const")]
pub fn stub_0x614458() {
    // IDA 0x614458: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x614468 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D12Vector2int16EE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2int16>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x614468(a: &Prop<[i16; 2]>, b: &Prop<[i16; 2]>) -> bool {
    // IDA 0x614468: `equalValues`: `getValue` both sides via slot 8,
    // compare lane by lane.
    a.value == b.value
}

// 0x61449c — __ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D12Vector2int16EE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2int16>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x61449c(prop: &Prop<[i16; 2]>) -> Value {
    // IDA 0x61449c: `getVariant`: `getValue` via slot 8, tag
    // `Type::getSingleton<Vector2int16>`, pack with `placement_any`.
    Value::Vector2i(prop.value)
}

// 0x6144c8 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D12Vector2int16EE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2int16>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x6144c8(prop: &mut Prop<[i16; 2]>, value: &Value) {
    // IDA 0x6144c8: `setVariant`: `any_cast<Vector2int16>` on a matching
    // payload, else `Variant::convert<Vector2int16>`, then `setValue`.
    prop.value = value.as_vector2i();
}

// 0x614624 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D12Vector2int16EE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2int16>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x614624(dst: &mut Prop<[i16; 2]>, src: &Prop<[i16; 2]>) {
    // IDA 0x614624: `copyValue`: 4-byte temp via slot 8, `setValue` into
    // the destination via slot 12.
    dst.value = src.value;
}

// 0x61464c — __ZN3RBX10Reflection23TypedPropertyDescriptorIN3G3D12Vector2int16EED1Ev
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2int16>::~TypedPropertyDescriptor()")]
pub fn stub_0x61464c() {
    // IDA 0x61464c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x614670 — __ZN3RBX10Reflection23TypedPropertyDescriptorIN3G3D12Vector2int16EED0Ev
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2int16>::~TypedPropertyDescriptor()")]
pub fn stub_0x614670() {
    // IDA 0x614670: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x61469c — __ZNK3RBX10Reflection14PropDescriptorINS_9ScreenGuiEN3G3D12Vector2int16EE10GetSetImplIMNS_9GuiBase2dEKFRKNS3_7Vector2EvEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ScreenGui,G3D::Vector2int16>::GetSetImpl<G3D::Vector2 const& (RBX::GuiBase2d::*)(void)const,void (RBX::ScreenGui::*)(G3D::Vector2int16)>::isReadOnly(void)const")]
pub fn stub_0x61469c() -> bool {
    // IDA 0x61469c: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0x6146a0 — __ZNK3RBX10Reflection14PropDescriptorINS_9ScreenGuiEN3G3D12Vector2int16EE10GetSetImplIMNS_9GuiBase2dEKFRKNS3_7Vector2EvEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ScreenGui,G3D::Vector2int16>::GetSetImpl<G3D::Vector2 const& (RBX::GuiBase2d::*)(void)const,void (RBX::ScreenGui::*)(G3D::Vector2int16)>::isWriteOnly(void)const")]
pub fn stub_0x6146a0() -> bool {
    // IDA 0x6146a0: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0x6146a4 — __ZNK3RBX10Reflection14PropDescriptorINS_9ScreenGuiEN3G3D12Vector2int16EE10GetSetImplIMNS_9GuiBase2dEKFRKNS3_7Vector2EvEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ScreenGui,G3D::Vector2int16>::GetSetImpl<G3D::Vector2 const& (RBX::GuiBase2d::*)(void)const,void (RBX::ScreenGui::*)(G3D::Vector2int16)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x6146a4(prop: &Prop<[i16; 2]>) -> [i16; 2] {
    // IDA 0x6146a4: `GetSetImpl<Vector2int16 const&>::getValue` for
    // ScreenGui: header strip, getter member-pointer decode, invoke.
    prop.value
}

// 0x6146d4 — __ZNK3RBX10Reflection14PropDescriptorINS_9ScreenGuiEN3G3D12Vector2int16EE10GetSetImplIMNS_9GuiBase2dEKFRKNS3_7Vector2EvEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ScreenGui,G3D::Vector2int16>::GetSetImpl<G3D::Vector2 const& (RBX::GuiBase2d::*)(void)const,void (RBX::ScreenGui::*)(G3D::Vector2int16)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector2int16 const&)const")]
pub fn stub_0x6146d4(prop: &mut Prop<[i16; 2]>, value: [i16; 2]) {
    // IDA 0x6146d4: `GetSetImpl<Vector2int16 const&>::setValue` for
    // ScreenGui: header strip, setter member-pointer decode, invoke.
    prop.value = value;
}

// 0x615c20 — __ZN3RBX10Reflection14PropDescriptorINS_4SeatEbED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Seat,bool>::~PropDescriptor()")]
pub fn stub_0x615c20() {
    // IDA 0x615c20: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x6178b4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4SeatES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Seat,RBX::Seat>(rbx_core::SharedPtr<RBX::Seat> const*,RBX::Seat *)const")]
pub fn stub_0x6178b4() {
    // IDA 0x6178b4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0x61877c — __ZN3RBX10Reflection14PropDescriptorINS_4SeatEbEC2IMNS_8SeatImplINS_17BasicPartInstanceEEEKFRKbvEMS7_FvS9_EEEPKcSF_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Seat,bool>::PropDescriptor<bool const& (RBX::SeatImpl<RBX::BasicPartInstance>::*)(void)const,void (RBX::SeatImpl<RBX::BasicPartInstance>::*)(bool const&)>(char const*,char const*,bool const& (RBX::SeatImpl<RBX::BasicPartInstance>::*)(void)const,void (RBX::SeatImpl<RBX::BasicPartInstance>::*)(bool const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x61877c(
    name: &str,
    category: &str,
    initial: bool,
    attributes: u32,
    permissions: u32,
) -> Prop<bool> {
    // IDA 0x61877c: `PropDescriptor<Seat, bool>` get/set ctor (getter through
    // `SeatImpl`, `bool const&`): `new` the GetSetImpl, forward into the
    // `TypedPropertyDescriptor` ctor. Same shape as 0x5f0cec.
    Prop::new(name, category, initial, attributes, permissions)
}

// 0x618890 — __ZN3RBX10Reflection14PropDescriptorINS_4SeatEbED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Seat,bool>::~PropDescriptor()")]
pub fn stub_0x618890() {
    // IDA 0x618890: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x6188bc — __ZNK3RBX10Reflection14PropDescriptorINS_4SeatEbE10GetSetImplIMNS_8SeatImplINS_17BasicPartInstanceEEEKFRKbvEMS7_FvS9_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Seat,bool>::GetSetImpl<bool const& (RBX::SeatImpl<RBX::BasicPartInstance>::*)(void)const,void (RBX::SeatImpl<RBX::BasicPartInstance>::*)(bool const&)>::isReadOnly(void)const")]
pub fn stub_0x6188bc() -> bool {
    // IDA 0x6188bc: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0x6188c0 — __ZNK3RBX10Reflection14PropDescriptorINS_4SeatEbE10GetSetImplIMNS_8SeatImplINS_17BasicPartInstanceEEEKFRKbvEMS7_FvS9_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Seat,bool>::GetSetImpl<bool const& (RBX::SeatImpl<RBX::BasicPartInstance>::*)(void)const,void (RBX::SeatImpl<RBX::BasicPartInstance>::*)(bool const&)>::isWriteOnly(void)const")]
pub fn stub_0x6188c0() -> bool {
    // IDA 0x6188c0: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0x6188c4 — __ZNK3RBX10Reflection14PropDescriptorINS_4SeatEbE10GetSetImplIMNS_8SeatImplINS_17BasicPartInstanceEEEKFRKbvEMS7_FvS9_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Seat,bool>::GetSetImpl<bool const& (RBX::SeatImpl<RBX::BasicPartInstance>::*)(void)const,void (RBX::SeatImpl<RBX::BasicPartInstance>::*)(bool const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x6188c4(prop: &Prop<bool>) -> bool {
    // IDA 0x6188c4: `GetSetImpl<bool const&>::getValue` for Seat: header
    // strip, getter member-pointer decode, invoke.
    prop.value
}

// 0x6188ec — __ZNK3RBX10Reflection14PropDescriptorINS_4SeatEbE10GetSetImplIMNS_8SeatImplINS_17BasicPartInstanceEEEKFRKbvEMS7_FvS9_EE8setValueEPNS0_13DescribedBaseES9_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Seat,bool>::GetSetImpl<bool const& (RBX::SeatImpl<RBX::BasicPartInstance>::*)(void)const,void (RBX::SeatImpl<RBX::BasicPartInstance>::*)(bool const&)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
pub fn stub_0x6188ec(prop: &mut Prop<bool>, value: bool) {
    // IDA 0x6188ec: `GetSetImpl<bool const&>::setValue` for Seat: header
    // strip, setter member-pointer decode, invoke.
    prop.value = value;
}

// 0x61a5fc — __ZN3RBX10Reflection13BoundFuncDescINS_9SelectionEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Selection,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()")]
pub fn stub_0x61a5fc() {
    // IDA 0x61a5fc: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x61a630 — __ZN3RBX10Reflection13BoundFuncDescINS_9SelectionEFvN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Selection,void ()(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),1>::~BoundFuncDesc()")]
pub fn stub_0x61a630() {
    // IDA 0x61a630: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x61a73c — __ZN3RBX10Reflection9EventDescINS_9SelectionEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Selection,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Selection::*>::~EventDesc()")]
pub fn stub_0x61a73c() {
    // IDA 0x61a73c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x61bd38 — __ZN3RBX10Reflection9EventDescINS_9SelectionEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Selection,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Selection::*>::~EventDesc()")]
pub fn stub_0x61bd38() {
    // IDA 0x61bd38: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

/// `RBX::Reflection::GenericSlotWrapper` cutover for this shard (cf. 0x4a40c8
/// in descriptor.rs): a stored callable invoked with packed `Value` args.
pub struct SlotWrapper {
    pub invoke: Box<dyn Fn(&[Value]) + Send + Sync>,
}

/// `RBX::Reflection::EventDesc<C, Sig>` header cutover (IDA 0x61c078,
/// 0x630a70): name/category/title, member-signal id, declared signature,
/// permissions and attributes. The member-signal pointer (+40) folds into
/// `member`.
#[derive(Debug, Clone)]
pub struct EventDesc {
    pub name: String,
    pub category: String,
    pub title: String,
    pub member: usize,
    pub signature: Signature,
    pub permissions: u32,
    pub attributes: u32,
}

/// `RBX::Reflection::EventSource` for a zero-arg `rbx::signal<void()>`
/// (IDA 0x61bdec/0x61bff0/0x61c064): owns the connected slots; strong refs
/// live in `holders` because `Signal::connect` keeps only weak refs.
#[derive(Default)]
pub struct EventSource0 {
    signal: Signal<()>,
    holders: parking_lot::Mutex<Vec<(SharedPtr<SlotWrapper>, SharedPtr<dyn Fn(()) + Send + Sync>)>>,
}

impl EventSource0 {
    pub fn disconnect_all(&self) {
        self.holders.lock().clear();
        self.signal.disconnect_all();
    }
}

// 0x61bdec — __ZNK3RBX10Reflection13EventDescImplILi0ENS_9SelectionEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::Selection,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Selection::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_0x61bdec(src: &EventSource0, wrapper: SharedPtr<SlotWrapper>) {
    // IDA 0x61bdec: `EventDescImpl<0>::connectGeneric`: bind the
    // `GenericSlotWrapper` into a slot on the member signal. Rust:
    // `Signal::connect` (concrete closure type); the wrapper folds in.
    let w = SharedPtr::clone(&wrapper);
    let slot = SharedPtr::new(move |_: ()| (w.invoke)(&[]));
    src.signal.connect(SharedPtr::clone(&slot));
    src.holders.lock().push((wrapper, slot));
}

// 0x61bff0 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_9SelectionEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::Selection,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Selection::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x61bff0(src: &EventSource0, args: &[Value]) {
    // IDA 0x61bff0: `EventDescImpl<0>::fireEvent`:
    // `ReleaseAssert(args.size() == 0)` (Event.h:295, 0x61c008-0x61c04e,
    // with the `_debugHook` path), then invoke the member signal
    // (0x61c04e-0x61c054).
    assert!(args.is_empty(), "args.size() == 0 include/Reflection/Event.h:295 (IDA 0x61bff0)");
    src.signal.fire(());
}

// 0x61c064 — __ZNK3RBX10Reflection13EventDescBaseINS_9SelectionEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Selection,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Selection::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x61c064(src: &EventSource0) {
    // IDA 0x61c064: `EventDescBase::disconnectAll`: header strip
    // (`a2 - 36`), `signal::disconnectAll` on the member signal at +40.
    src.disconnect_all();
}

// 0x61c078 — __ZN3RBX10Reflection13BoundFuncDescINS_9SelectionEFvN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEELi1EEC2EMS2_FvSB_EPKcSH_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Selection,void ()(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),1>::BoundFuncDesc(void (RBX::Selection::*)(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x61c078(
    name: &str,
    member: usize,
    arg_name: &str,
    permissions: u32,
    attributes: u32,
) -> BoundFunc {
    // IDA 0x61c078: 1-arg void `BoundFuncDesc` ctor: class-descriptor fetch,
    // `FunctionDescriptor` init, member pair at +40, default-arg slot at +48
    // cleared (0x61c0fe), then `declareSignature` in-ctor (0x61c120-0x61c134).
    let mut func = BoundFunc {
        name: name.to_owned(),
        member,
        signature: Signature { return_type: "void", args: Vec::new() },
        permissions,
        attributes,
    };
    stub_0x61c210(&mut func, arg_name);
    func
}

// 0x61c210 — __ZN3RBX10Reflection13BoundFuncDescINS_9SelectionEFvN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Selection,void ()(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x61c210(func: &mut BoundFunc, arg_name: &str) {
    // IDA 0x61c210: `declareSignature`: store the `void` return `Type` at
    // +28 (0x61c220), `RBX::Name::declare` the arg name (0x61c22a),
    // `getSingleton<InstanceList>` for it (0x61c22c),
    // `SignatureDescriptor::addArgument` (0x61c23e).
    func.signature.return_type = "void";
    func.signature.args.push((arg_name.to_owned(), "InstanceList"));
}

// 0x61c240 — __ZN3RBX10Reflection13BoundFuncDescINS_9SelectionEFvN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Selection,void ()(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),1>::~BoundFuncDesc()")]
pub fn stub_0x61c240() {
    // IDA 0x61c240: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x61c35c — __ZNK3RBX10Reflection13BoundFuncDescINS_9SelectionEFvN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Selection,void ()(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x61c35c(
    args: &Arguments,
    call: &dyn Fn(Vec<u32>),
    default: Option<Vec<u32>>,
) {
    // IDA 0x61c35c: 1-arg void `execute`: `ArgHelper::getArg<InstanceList,
    // 1>` then `Call1Helper<void>::call` (0x61c3c6 + tail-call).
    let arg = match args.args.first() {
        Some(Value::Nil) | None => default.unwrap_or_else(|| panic!("Argument 1 missing or nil (IDA 0x61c35c)")),
        Some(Value::InstanceList(v)) => v.clone(),
        Some(other) => panic!("Variant::convert<InstanceList> on {other:?} (IDA 0x61c35c)"),
    };
    stub_0x61c440(call, arg);
}

// 0x61c440 — __ZN3RBX10Reflection11Call1HelperINS_9SelectionEMS2_FvN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEESB_vE4callEPS2_SD_RNS0_7VariantERKSB_
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Selection,void (RBX::Selection::*)(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,void>::call(RBX::Selection*,void (RBX::Selection::*)(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),RBX::Reflection::Variant &,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> const&)")]
pub fn stub_0x61c440(call: &dyn Fn(Vec<u32>), arg: Vec<u32>) {
    // IDA 0x61c440: `Call1Helper<void>::call`: member-pointer decode,
    // shared-arg refcount bump (0x61c4a6-0x61c4b8), invoke, release. No
    // return packing for `void`; `Arc` clone/drop covers the refcounts.
    call(arg);
}

// 0x61c528 — __ZN3RBX10Reflection13BoundFuncDescINS_9SelectionEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EEC2EMS2_FSB_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Selection,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Selection::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x61c528(name: &str, member: usize, permissions: u32, attributes: u32) -> BoundFunc {
    // IDA 0x61c528: `BoundFuncDesc<Selection, InstanceList>::BoundFuncDesc`:
    // class-descriptor fetch, `FunctionDescriptor` init, vtable install,
    // member pair at +40, return type `Type::getSingleton<InstanceList>` at
    // +28 (0x61c5be).
    BoundFunc {
        name: name.to_owned(),
        member,
        signature: Signature { return_type: "InstanceList", args: Vec::new() },
        permissions,
        attributes,
    }
}

// 0x61c62c — __ZN3RBX10Reflection13BoundFuncDescINS_9SelectionEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Selection,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()")]
pub fn stub_0x61c62c() {
    // IDA 0x61c62c: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x61c6e0 — __ZNK3RBX10Reflection13BoundFuncDescINS_9SelectionEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Selection,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x61c6e0(_func: &BoundFunc, call: &dyn Fn() -> Vec<u32>) -> Value {
    // IDA 0x61c6e0: `BoundFuncDesc<InstanceList>::execute` forwards to
    // `Call0Helper<InstanceList>::call` (0x61c6e8-0x61c6ea + tail-call).
    stub_0x61c704(call)
}

// 0x61c704 — __ZN3RBX10Reflection11Call0HelperINS_9SelectionEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvESB_E4callEPS2_SD_RNS0_7VariantE
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::Selection,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Selection::*)(void),rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::call(RBX::Selection*,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Selection::*)(void),RBX::Reflection::Variant &)")]
pub fn stub_0x61c704(call: &dyn Fn() -> Vec<u32>) -> Value {
    // IDA 0x61c704: `Call0Helper<InstanceList>::call`: header strip,
    // member-pointer decode, invoke, tag the shared-vector return type, pack
    // with `placement_any::operator=` (0x61c774-0x61c780).
    Value::InstanceList(call())
}

// 0x61dbf4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12SelectionBoxES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::SelectionBox,RBX::SelectionBox>(rbx_core::SharedPtr<RBX::SelectionBox> const*,RBX::SelectionBox *)const")]
pub fn stub_0x61dbf4() {
    // IDA 0x61dbf4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0x61f874 — __ZN3RBX10Reflection17RefPropDescriptorINS_14SelectionLassoENS_8HumanoidEED1Ev
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionLasso,RBX::Humanoid>::~RefPropDescriptor()")]
pub fn stub_0x61f874() {
    // IDA 0x61f874: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x61f8c4 — __ZN3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEED1Ev
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionPartLasso,RBX::PartInstance>::~RefPropDescriptor()")]
pub fn stub_0x61f8c4() {
    // IDA 0x61f8c4: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x61f900 — __ZN3RBX10Reflection14PropDescriptorINS_19SelectionPointLassoEN3G3D7Vector3EED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SelectionPointLasso,G3D::Vector3>::~PropDescriptor()")]
pub fn stub_0x61f900() {
    // IDA 0x61f900: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x62118c — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19SelectionPointLassoES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::SelectionPointLasso,RBX::SelectionPointLasso>(rbx_core::SharedPtr<RBX::SelectionPointLasso> const*,RBX::SelectionPointLasso *)const")]
pub fn stub_0x62118c() {
    // IDA 0x62118c: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0x6219e4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_18SelectionPartLassoES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::SelectionPartLasso,RBX::SelectionPartLasso>(rbx_core::SharedPtr<RBX::SelectionPartLasso> const*,RBX::SelectionPartLasso *)const")]
pub fn stub_0x6219e4() {
    // IDA 0x6219e4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

/// `RBX::Reflection::RefPropDescriptor<C, T*>` cutover (IDA 0x62307c):
/// name/category/attributes/permissions, the `RefType<T*>` tag in
/// `expected`, and the live target id. The member GetSet (+44) folds into
/// direct field access; the `DescribedBase`/`+ 36` Instance-side adjusts
/// (0x6231dc/0x62349e/0x62352e) are `Arc` pointer mechanics with no Rust
/// equivalent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefProp {
    pub name: String,
    pub category: String,
    pub attributes: u32,
    pub permissions: u32,
    pub expected: &'static str,
    pub target: Option<u32>,
}

impl RefProp {
    pub fn new(
        name: &str,
        category: &str,
        expected: &'static str,
        attributes: u32,
        permissions: u32,
    ) -> Self {
        Self {
            name: name.to_owned(),
            category: category.to_owned(),
            attributes,
            permissions,
            expected,
            target: None,
        }
    }
}

// 0x6227bc — __ZN3RBX10Reflection14PropDescriptorINS_19SelectionPointLassoEN3G3D7Vector3EEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SelectionPointLasso,G3D::Vector3>::PropDescriptor<G3D::Vector3 (RBX::SelectionPointLasso::*)(void)const,void (RBX::SelectionPointLasso::*)(G3D::Vector3)>(char const*,char const*,G3D::Vector3 (RBX::SelectionPointLasso::*)(void)const,void (RBX::SelectionPointLasso::*)(G3D::Vector3),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x6227bc(
    name: &str,
    category: &str,
    initial: Vector3,
    attributes: u32,
    permissions: u32,
) -> Prop<Vector3> {
    // IDA 0x6227bc: `PropDescriptor<SelectionPointLasso, Vector3>` get/set
    // ctor: `new` the GetSetImpl, forward into the
    // `TypedPropertyDescriptor` ctor. Same shape as 0x5f0cec.
    Prop::new(name, category, initial, attributes, permissions)
}

// 0x6228d0 — __ZN3RBX10Reflection14PropDescriptorINS_19SelectionPointLassoEN3G3D7Vector3EED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SelectionPointLasso,G3D::Vector3>::~PropDescriptor()")]
pub fn stub_0x6228d0() {
    // IDA 0x6228d0: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x6228fc — __ZNK3RBX10Reflection14PropDescriptorINS_19SelectionPointLassoEN3G3D7Vector3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SelectionPointLasso,G3D::Vector3>::GetSetImpl<G3D::Vector3 (RBX::SelectionPointLasso::*)(void)const,void (RBX::SelectionPointLasso::*)(G3D::Vector3)>::isReadOnly(void)const")]
pub fn stub_0x6228fc() -> bool {
    // IDA 0x6228fc: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0x622900 — __ZNK3RBX10Reflection14PropDescriptorINS_19SelectionPointLassoEN3G3D7Vector3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SelectionPointLasso,G3D::Vector3>::GetSetImpl<G3D::Vector3 (RBX::SelectionPointLasso::*)(void)const,void (RBX::SelectionPointLasso::*)(G3D::Vector3)>::isWriteOnly(void)const")]
pub fn stub_0x622900() -> bool {
    // IDA 0x622900: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0x622904 — __ZNK3RBX10Reflection14PropDescriptorINS_19SelectionPointLassoEN3G3D7Vector3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SelectionPointLasso,G3D::Vector3>::GetSetImpl<G3D::Vector3 (RBX::SelectionPointLasso::*)(void)const,void (RBX::SelectionPointLasso::*)(G3D::Vector3)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x622904(prop: &Prop<Vector3>) -> Vector3 {
    // IDA 0x622904: `GetSetImpl<Vector3>::getValue` for SelectionPointLasso:
    // header strip, getter member-pointer decode, invoke.
    prop.value
}

// 0x62292c — __ZNK3RBX10Reflection14PropDescriptorINS_19SelectionPointLassoEN3G3D7Vector3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SelectionPointLasso,G3D::Vector3>::GetSetImpl<G3D::Vector3 (RBX::SelectionPointLasso::*)(void)const,void (RBX::SelectionPointLasso::*)(G3D::Vector3)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector3 const&)const")]
pub fn stub_0x62292c(prop: &mut Prop<Vector3>, value: Vector3) {
    // IDA 0x62292c: `GetSetImpl<Vector3>::setValue` for SelectionPointLasso:
    // header strip, setter member-pointer decode, invoke.
    prop.value = value;
}

// 0x62307c — __ZN3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEEC2IMS2_KFPS3_vEMS2_FvS6_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionPartLasso,RBX::PartInstance>::RefPropDescriptor<RBX::PartInstance* (RBX::SelectionPartLasso::*)(void)const,void (RBX::SelectionPartLasso::*)(RBX::PartInstance*)>(char const*,char const*,RBX::PartInstance* (RBX::SelectionPartLasso::*)(void)const,void (RBX::SelectionPartLasso::*)(RBX::PartInstance*),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x62307c(
    name: &str,
    category: &str,
    expected: &'static str,
    attributes: u32,
    permissions: u32,
) -> RefProp {
    // IDA 0x62307c: `RefPropDescriptor<SelectionPartLasso, PartInstance>`
    // ctor: link the `RefType<PartInstance*>` singleton, `new` the member
    // GetSet, forward into the typed-descriptor ctor.
    RefProp::new(name, category, expected, attributes, permissions)
}

// 0x623120 — __ZN3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEED0Ev
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionPartLasso,RBX::PartInstance>::~RefPropDescriptor()")]
pub fn stub_0x623120() {
    // IDA 0x623120: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x623150 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionPartLasso,RBX::PartInstance>::isReadOnly(void)const")]
pub fn stub_0x623150() {
    // IDA 0x623150: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x623160 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionPartLasso,RBX::PartInstance>::isWriteOnly(void)const")]
pub fn stub_0x623160() {
    // IDA 0x623160: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x623170 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionPartLasso,RBX::PartInstance>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x623170(a: &RefProp, b: &RefProp) -> bool {
    // IDA 0x623170: `equalValues`: get both sides via the +44 member
    // (0x623180/0x623196), raw-pointer compare.
    a.target == b.target
}

// 0x623198 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionPartLasso,RBX::PartInstance>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x623198(prop: &RefProp) -> Value {
    // IDA 0x623198: `getVariant`: get via slot 8 (0x6231bc),
    // `shared_from<PartInstance>` with the `+ 36` adjust (0x6231c4-0x6231dc),
    // pack the shared ref into the out Variant.
    match prop.target {
        Some(id) => Value::Instance(id),
        None => Value::Nil,
    }
}

// 0x6232b0 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionPartLasso,RBX::PartInstance>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x6232b0(prop: &mut RefProp, value: &Value) {
    // IDA 0x6232b0: `setVariant`: `Variant::get<shared_ptr<DescribedBase>>`
    // (0x6232d4), then the checked set entry (vf+64, 0x623312).
    match value {
        Value::Instance(id) => prop.target = Some(*id),
        Value::Nil => prop.target = None,
        other => panic!("Variant::get<shared_ptr<DescribedBase>> on {other:?} (IDA 0x6232b0)"),
    }
}

// 0x623378 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionPartLasso,RBX::PartInstance>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x623378(dst: &mut RefProp, src: &RefProp) {
    // IDA 0x623378: `copyValue`: get temp via slot 8 (0x62338a), set via
    // slot 12 (0x62339a).
    dst.target = src.target;
}

// 0x62339c — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionPartLasso,RBX::PartInstance>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_0x62339c(prop: &RefProp) -> Option<u32> {
    // IDA 0x62339c: `writeValue`: get via slot 8 (0x6233c0), `+ 36` adjust
    // (0x6233ca), `InstanceHandle` wrap, `XmlNameValuePair::setValue`
    // (0x6233ce-0x623406). Returns the serialized ref.
    prop.target
}

// 0x623470 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionPartLasso,RBX::PartInstance>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x623470(prop: &mut RefProp, id: Option<u32>) {
    // IDA 0x623470: `readValue`: `a3 + 12` skips the Xml pair header, then
    // the `IReferenceBinder` entry (vf+4) resolves and sets. Binder
    // resolution collapses into the id parameter.
    prop.target = id;
}

// 0x623494 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE11getRefValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionPartLasso,RBX::PartInstance>::getRefValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x623494(prop: &RefProp) -> Option<u32> {
    // IDA 0x623494: `getRefValue`: get via slot 8 (0x62349e), `+ 36`
    // Instance-side adjust when nonzero (0x6234a2-0x6234a4).
    prop.target
}

// 0x6234a8 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE11setRefValueEPNS0_13DescribedBaseES6_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionPartLasso,RBX::PartInstance>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
pub fn stub_0x6234a8(prop: &mut RefProp, id: Option<u32>, actual: Option<&'static str>) {
    // IDA 0x6234a8: `setRefValue`: null passes; else
    // `__dynamic_cast<PartInstance>` (0x6234d6); miss throws `bad_cast`
    // (0x6234f0-0x62351e); hit sets via slot 12 (0x6234ec). Rust cutover
    // panics with the same type name.
    match (id, actual) {
        (None, _) => prop.target = None,
        (Some(id), Some(t)) if t != prop.expected => panic!("std::bad_cast (IDA 0x6234a8): {t} is not a {}", prop.expected),
        (Some(id), _) => prop.target = Some(id),
    }
}

// 0x623524 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionPartLasso,RBX::PartInstance>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
pub fn stub_0x623524(prop: &mut RefProp, id: Option<u32>) {
    // IDA 0x623524: `setRefValueUnsafe`: `a3 - 36` header strip
    // (0x62352e-0x623534), set via slot 12 with no `__dynamic_cast` check.
    prop.target = id;
}

// 0x623544 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionPartLasso,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
pub fn stub_0x623544(prop: &mut RefProp, id: u32) {
    // IDA 0x623544: `assignIDREF`: `shared_count` copy (0x623572),
    // `pi - 36` Instance adjust (0x6235aa), set via slot 12. Rust: `Arc`
    // clone/drop covers the refcounts.
    prop.target = Some(id);
}

// 0x623624 — __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
#[doc(alias = "non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::SelectionPartLasso,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
pub fn stub_0x623624() {
    // IDA 0x623624: non-virtual thunk to `RBX::Reflection::RefPropDescriptor<RBX::SelectionPartLasso,RBX::PartInstance>::assignIDREF( int a1, int a2, int a3, int a4,\` — this/arg-adjust + tail-call (arg a1 -= 40) (decompiled). Rust uses static dispatch; no thunk needed. Target unmodeled: cutover no-op.
}

// 0x62362c — __ZNK3RBX10Reflection14PropDescriptorINS_18SelectionPartLassoEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SelectionPartLasso,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::SelectionPartLasso::*)(void)const,void (RBX::SelectionPartLasso::*)(RBX::PartInstance *)>::isReadOnly(void)const")]
pub fn stub_0x62362c() -> bool {
    // IDA 0x62362c: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0x623630 — __ZNK3RBX10Reflection14PropDescriptorINS_18SelectionPartLassoEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SelectionPartLasso,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::SelectionPartLasso::*)(void)const,void (RBX::SelectionPartLasso::*)(RBX::PartInstance *)>::isWriteOnly(void)const")]
pub fn stub_0x623630() -> bool {
    // IDA 0x623630: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0x623634 — __ZNK3RBX10Reflection14PropDescriptorINS_18SelectionPartLassoEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SelectionPartLasso,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::SelectionPartLasso::*)(void)const,void (RBX::SelectionPartLasso::*)(RBX::PartInstance *)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x623634(prop: &RefProp) -> Option<u32> {
    // IDA 0x623634: `GetSetImpl<PartInstance*>::getValue`: header strip,
    // getter member-pointer decode, invoke.
    prop.target
}

// 0x623654 — __ZNK3RBX10Reflection14PropDescriptorINS_18SelectionPartLassoEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SelectionPartLasso,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::SelectionPartLasso::*)(void)const,void (RBX::SelectionPartLasso::*)(RBX::PartInstance *)>::setValue(RBX::Reflection::DescribedBase *,RBX::PartInstance * const&)const")]
pub fn stub_0x623654(prop: &mut RefProp, id: Option<u32>) {
    // IDA 0x623654: `GetSetImpl<PartInstance*>::setValue`: header strip,
    // setter member-pointer decode, invoke.
    prop.target = id;
}

// 0x623b10 — __ZN3RBX10Reflection17RefPropDescriptorINS_14SelectionLassoENS_8HumanoidEEC2IMS2_KFPS3_vEMS2_FvS6_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionLasso,RBX::Humanoid>::RefPropDescriptor<RBX::Humanoid* (RBX::SelectionLasso::*)(void)const,void (RBX::SelectionLasso::*)(RBX::Humanoid*)>(char const*,char const*,RBX::Humanoid* (RBX::SelectionLasso::*)(void)const,void (RBX::SelectionLasso::*)(RBX::Humanoid*),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x623b10(
    name: &str,
    category: &str,
    expected: &'static str,
    attributes: u32,
    permissions: u32,
) -> RefProp {
    // IDA 0x623b10: `RefPropDescriptor<SelectionLasso, Humanoid>` ctor:
    // `RefType<Humanoid*>` link, member GetSet `new`, typed-descriptor
    // forward (same shape as 0x62307c).
    RefProp::new(name, category, expected, attributes, permissions)
}

// 0x623bb4 — __ZN3RBX10Reflection17RefPropDescriptorINS_14SelectionLassoENS_8HumanoidEED0Ev
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionLasso,RBX::Humanoid>::~RefPropDescriptor()")]
pub fn stub_0x623bb4() {
    // IDA 0x623bb4: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x623be4 — __ZNK3RBX10Reflection17RefPropDescriptorINS_14SelectionLassoENS_8HumanoidEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionLasso,RBX::Humanoid>::isReadOnly(void)const")]
pub fn stub_0x623be4() {
    // IDA 0x623be4: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x623bf4 — __ZNK3RBX10Reflection17RefPropDescriptorINS_14SelectionLassoENS_8HumanoidEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionLasso,RBX::Humanoid>::isWriteOnly(void)const")]
pub fn stub_0x623bf4() {
    // IDA 0x623bf4: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x623c04 — __ZNK3RBX10Reflection17RefPropDescriptorINS_14SelectionLassoENS_8HumanoidEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionLasso,RBX::Humanoid>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x623c04(a: &RefProp, b: &RefProp) -> bool {
    // IDA 0x623c04: `equalValues`: raw-pointer compare via slot 8
    // (same shape as 0x623170).
    a.target == b.target
}

// 0x623c2c — __ZNK3RBX10Reflection17RefPropDescriptorINS_14SelectionLassoENS_8HumanoidEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionLasso,RBX::Humanoid>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x623c2c(prop: &RefProp) -> Value {
    // IDA 0x623c2c: `getVariant`: get via slot 8, `shared_from<Humanoid>`,
    // pack the shared ref (same shape as 0x623198).
    match prop.target {
        Some(id) => Value::Instance(id),
        None => Value::Nil,
    }
}

// 0x623d44 — __ZNK3RBX10Reflection17RefPropDescriptorINS_14SelectionLassoENS_8HumanoidEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionLasso,RBX::Humanoid>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x623d44(prop: &mut RefProp, value: &Value) {
    // IDA 0x623d44: `setVariant` through the checked `setRefValue` entry
    // (vf+64, same shape as 0x6232b0).
    match value {
        Value::Instance(id) => prop.target = Some(*id),
        Value::Nil => prop.target = None,
        other => panic!("Variant::get<shared_ptr<DescribedBase>> on {other:?} (IDA 0x623d44)"),
    }
}

// 0x623e0c — __ZNK3RBX10Reflection17RefPropDescriptorINS_14SelectionLassoENS_8HumanoidEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionLasso,RBX::Humanoid>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x623e0c(dst: &mut RefProp, src: &RefProp) {
    // IDA 0x623e0c: `copyValue`: get temp via slot 8, set via slot 12
    // (same shape as 0x623378).
    dst.target = src.target;
}

// 0x623e30 — __ZNK3RBX10Reflection17RefPropDescriptorINS_14SelectionLassoENS_8HumanoidEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionLasso,RBX::Humanoid>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_0x623e30(prop: &RefProp) -> Option<u32> {
    // IDA 0x623e30: `writeValue`: `InstanceHandle` wrap + `+ 36` adjust,
    // `XmlNameValuePair::setValue` (same shape as 0x62339c). Returns the
    // serialized ref.
    prop.target
}

// 0x623f04 — __ZNK3RBX10Reflection17RefPropDescriptorINS_14SelectionLassoENS_8HumanoidEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionLasso,RBX::Humanoid>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x623f04(prop: &mut RefProp, id: Option<u32>) {
    // IDA 0x623f04: `readValue`: binder lookup on the Xml pair payload,
    // set on hit (same shape as 0x623470). Binder resolution collapses
    // into the id parameter.
    prop.target = id;
}

// 0x623f28 — __ZNK3RBX10Reflection17RefPropDescriptorINS_14SelectionLassoENS_8HumanoidEE11getRefValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionLasso,RBX::Humanoid>::getRefValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x623f28(prop: &RefProp) -> Option<u32> {
    // IDA 0x623f28: `getRefValue`: get via slot 8, `+ 36` Instance adjust
    // when nonzero (same shape as 0x623494).
    prop.target
}

// 0x623f3c — __ZNK3RBX10Reflection17RefPropDescriptorINS_14SelectionLassoENS_8HumanoidEE11setRefValueEPNS0_13DescribedBaseES6_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionLasso,RBX::Humanoid>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
pub fn stub_0x623f3c(prop: &mut RefProp, id: Option<u32>, actual: Option<&'static str>) {
    // IDA 0x623f3c: `setRefValue` with the `__dynamic_cast<Humanoid>` check
    // (same shape as 0x6234a8).
    match (id, actual) {
        (None, _) => prop.target = None,
        (Some(id), Some(t)) if t != prop.expected => panic!("std::bad_cast (IDA 0x623f3c): {t} is not a {}", prop.expected),
        (Some(id), _) => prop.target = Some(id),
    }
}

// 0x623fb8 — __ZNK3RBX10Reflection17RefPropDescriptorINS_14SelectionLassoENS_8HumanoidEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionLasso,RBX::Humanoid>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
pub fn stub_0x623fb8(prop: &mut RefProp, id: Option<u32>) {
    // IDA 0x623fb8: `setRefValueUnsafe`: `a3 - 36` header strip, set via
    // slot 12 with no `__dynamic_cast` check.
    prop.target = id;
}

// 0x623fd8 — __ZNK3RBX10Reflection17RefPropDescriptorINS_14SelectionLassoENS_8HumanoidEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionLasso,RBX::Humanoid>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
pub fn stub_0x623fd8(prop: &mut RefProp, id: u32) {
    // IDA 0x623fd8: `assignIDREF`: `shared_count` copy, `pi - 36` adjust,
    // set via slot 12. Rust: `Arc` clone/drop covers the refcounts.
    prop.target = Some(id);
}

// 0x6240b8 — __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_14SelectionLassoENS_8HumanoidEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
#[doc(alias = "non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::SelectionLasso,RBX::Humanoid>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
pub fn stub_0x6240b8() {
    // IDA 0x6240b8: non-virtual thunk to `RBX::Reflection::RefPropDescriptor<RBX::SelectionLasso,RBX::Humanoid>::assignIDREF( int a1, int a2, int a3, int a4, ` — this/arg-adjust + tail-call (arg a1 -= 40) (decompiled). Rust uses static dispatch; no thunk needed. Target unmodeled: cutover no-op.
}

// 0x6240c0 — __ZNK3RBX10Reflection14PropDescriptorINS_14SelectionLassoEPNS_8HumanoidEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SelectionLasso,RBX::Humanoid *>::GetSetImpl<RBX::Humanoid * (RBX::SelectionLasso::*)(void)const,void (RBX::SelectionLasso::*)(RBX::Humanoid *)>::isReadOnly(void)const")]
pub fn stub_0x6240c0() -> bool {
    // IDA 0x6240c0: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0x6240c4 — __ZNK3RBX10Reflection14PropDescriptorINS_14SelectionLassoEPNS_8HumanoidEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SelectionLasso,RBX::Humanoid *>::GetSetImpl<RBX::Humanoid * (RBX::SelectionLasso::*)(void)const,void (RBX::SelectionLasso::*)(RBX::Humanoid *)>::isWriteOnly(void)const")]
pub fn stub_0x6240c4() -> bool {
    // IDA 0x6240c4: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0x6240c8 — __ZNK3RBX10Reflection14PropDescriptorINS_14SelectionLassoEPNS_8HumanoidEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SelectionLasso,RBX::Humanoid *>::GetSetImpl<RBX::Humanoid * (RBX::SelectionLasso::*)(void)const,void (RBX::SelectionLasso::*)(RBX::Humanoid *)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x6240c8(prop: &RefProp) -> Option<u32> {
    // IDA 0x6240c8: `GetSetImpl<Humanoid*>::getValue`: header strip, getter
    // member-pointer decode, invoke.
    prop.target
}

// 0x6240e8 — __ZNK3RBX10Reflection14PropDescriptorINS_14SelectionLassoEPNS_8HumanoidEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SelectionLasso,RBX::Humanoid *>::GetSetImpl<RBX::Humanoid * (RBX::SelectionLasso::*)(void)const,void (RBX::SelectionLasso::*)(RBX::Humanoid *)>::setValue(RBX::Reflection::DescribedBase *,RBX::Humanoid * const&)const")]
pub fn stub_0x6240e8(prop: &mut RefProp, id: Option<u32>) {
    // IDA 0x6240e8: `GetSetImpl<Humanoid*>::setValue`: header strip, setter
    // member-pointer decode, invoke.
    prop.target = id;
}

// 0x625080 — __ZN3RBX10Reflection14PropDescriptorINS_20SkateboardControllerEfED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardController,float>::~PropDescriptor()")]
pub fn stub_0x625080() {
    // IDA 0x625080: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x6250ac — __ZN3RBX10Reflection9EventDescINS_20SkateboardControllerEFvSsEN3rbx6signalIS3_EEMS2_S6_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::SkateboardController,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::SkateboardController::*>::~EventDesc()")]
pub fn stub_0x6250ac() {
    // IDA 0x6250ac: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

/// `RBX::Reflection::EventSource` for a one-`string` `rbx::signal`
/// (IDA 0x626770/0x6268c4/0x626a68): owns the connected slots; strong refs
/// live in `holders` because `Signal::connect` keeps only weak refs.
#[derive(Default)]
pub struct EventSource1String {
    signal: Signal<String>,
    holders: parking_lot::Mutex<Vec<(SharedPtr<SlotWrapper>, SharedPtr<dyn Fn(String) + Send + Sync>)>>,
}

impl EventSource1String {
    pub fn disconnect_all(&self) {
        self.holders.lock().clear();
        self.signal.disconnect_all();
    }
}

/// `RBX::Reflection::Type` record (IDA 0x62ffac): `Descriptor` init folds
/// into the name/category record; the tag-emptiness assert (Type.h:77) and
/// the `addToAllTypes` registry push are real.
#[derive(Debug, Clone)]
pub struct ClassType {
    pub name: String,
    pub category: String,
}

static ALL_TYPES: std::sync::LazyLock<parking_lot::Mutex<Vec<String>>> =
    std::sync::LazyLock::new(|| parking_lot::Mutex::new(Vec::new()));

pub fn register_type(name: &str) {
    ALL_TYPES.lock().push(name.to_owned());
}

/// `RBX::Reflection::RefType<T*>` singleton (IDA 0x62f85c/0x630100):
/// guard-once `Type::Type<T*>` construct + vtable install; the destructor
/// runs at process exit.
#[derive(Debug, Clone, Copy)]
pub struct RefType {
    pub target: &'static str,
}

// 0x626538 — __ZN3RBX10Reflection9EventDescINS_20SkateboardControllerEFvSsEN3rbx6signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::SkateboardController,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::SkateboardController::*>::EventDesc(rbx::signal<void ()(std::string)> RBX::SkateboardController::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x626538(
    name: &str,
    category: &str,
    title: &str,
    member: usize,
    arg_name: &str,
    permissions: u32,
    attributes: u32,
) -> EventDesc {
    // IDA 0x626538: `EventDesc<SkateboardController, void(string)>` ctor:
    // base `EventDescriptor` init, member-signal pointer stored at +40,
    // one-item signature list (`string` arg) appended.
    EventDesc {
        name: name.to_owned(),
        category: category.to_owned(),
        title: title.to_owned(),
        member,
        signature: Signature {
            return_type: "void",
            args: vec![(arg_name.to_owned(), "string")],
        },
        permissions,
        attributes,
    }
}

// 0x6266bc — __ZN3RBX10Reflection9EventDescINS_20SkateboardControllerEFvSsEN3rbx6signalIS3_EEMS2_S6_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::SkateboardController,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::SkateboardController::*>::~EventDesc()")]
pub fn stub_0x6266bc() {
    // IDA 0x6266bc: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x626770 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_20SkateboardControllerEFvSsEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::SkateboardController,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::SkateboardController::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_0x626770(src: &EventSource1String, wrapper: SharedPtr<SlotWrapper>) {
    // IDA 0x626770: `EventDescImpl<1, string>::connectGeneric`: bind the
    // `GenericSlotWrapper` into a 1-arg slot on the member signal (same
    // shape as 0x61bdec).
    let w = SharedPtr::clone(&wrapper);
    let slot = SharedPtr::new(move |arg: String| (w.invoke)(&[Value::Text(arg)]));
    src.signal.connect(SharedPtr::clone(&slot));
    src.holders.lock().push((wrapper, slot));
}

// 0x6268c4 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_20SkateboardControllerEFvSsEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::SkateboardController,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::SkateboardController::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x6268c4(src: &EventSource1String, args: &[Value]) {
    // IDA 0x6268c4: `EventDescImpl<1>::fireEvent`:
    // `ReleaseAssert(args.size() == 1)` (Event.h:320, 0x626900-0x62695c,
    // with the `_debugHook` path), unpack the string arg, invoke the member
    // signal.
    assert!(args.len() == 1, "args.size() == 1 include/Reflection/Event.h:320 (IDA 0x6268c4)");
    src.signal.fire(args[0].as_text());
}

// 0x626a68 — __ZNK3RBX10Reflection13EventDescBaseINS_20SkateboardControllerEFvSsEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::SkateboardController,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::SkateboardController::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x626a68(src: &EventSource1String) {
    // IDA 0x626a68: `EventDescBase::disconnectAll` (same shape as 0x61c064).
    src.disconnect_all();
}

// 0x626a7c — __ZN3RBX10Reflection14PropDescriptorINS_20SkateboardControllerEfEC2IMS2_KFfvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardController,float>::PropDescriptor<float (RBX::SkateboardController::*)(void)const,int>(char const*,char const*,float (RBX::SkateboardController::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x626a7c(
    name: &str,
    category: &str,
    initial: f32,
    attributes: u32,
    permissions: u32,
) -> Prop<f32> {
    // IDA 0x626a7c: `PropDescriptor<SkateboardController, float>` read-only
    // ctor (getter + `int` placeholder): `new` the GetImpl, forward into
    // `TypedPropertyDescriptor<float>`. Same shape as 0x5f2b1c.
    Prop::new(name, category, initial, attributes, permissions)
}

// 0x626b88 — __ZN3RBX10Reflection14PropDescriptorINS_20SkateboardControllerEfED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardController,float>::~PropDescriptor()")]
pub fn stub_0x626b88() {
    // IDA 0x626b88: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x626bb4 — __ZNK3RBX10Reflection14PropDescriptorINS_20SkateboardControllerEfE7GetImplIMS2_KFfvEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardController,float>::GetImpl<float (RBX::SkateboardController::*)(void)const>::isReadOnly(void)const")]
pub fn stub_0x626bb4() {
    // IDA 0x626bb4: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x626bb8 — __ZNK3RBX10Reflection14PropDescriptorINS_20SkateboardControllerEfE7GetImplIMS2_KFfvEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardController,float>::GetImpl<float (RBX::SkateboardController::*)(void)const>::isWriteOnly(void)const")]
pub fn stub_0x626bb8() {
    // IDA 0x626bb8: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x626bbc — __ZNK3RBX10Reflection14PropDescriptorINS_20SkateboardControllerEfE7GetImplIMS2_KFfvEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardController,float>::GetImpl<float (RBX::SkateboardController::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x626bbc(prop: &Prop<f32>) -> f32 {
    // IDA 0x626bbc: `GetImpl<float>::getValue` for SkateboardController:
    // header strip, getter member-pointer decode, invoke.
    prop.value
}

// 0x626bdc — __ZNK3RBX10Reflection14PropDescriptorINS_20SkateboardControllerEfE7GetImplIMS2_KFfvEE8setValueEPNS0_13DescribedBaseERKf
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardController,float>::GetImpl<float (RBX::SkateboardController::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
pub fn stub_0x626bdc() {
    // IDA 0x626bdc: `GetImpl<float>::setValue` for SkateboardController
    // (read-only): `throw runtime_error("can't set value")`.
    panic!("can't set value (IDA 0x626bdc)");
}

// 0x627234 — __ZN3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::EnumDesc(void)")]
pub fn stub_0x627234() -> crate::enum_desc::EnumDesc {
    // IDA 0x627234: EnumDesc<T>::C1 -- EnumDescriptor base ctor with name "Enum", vtable install, empty tables (decompiled; cf. 0x37148c). Pairs are registered by the addPair stubs.
    crate::enum_desc::EnumDesc::new("Enum")
}

// 0x627238 — __ZN3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::EnumDesc(void)")]
pub fn stub_0x627238() -> crate::enum_desc::EnumDesc {
    // IDA 0x627238: EnumDesc<T>::C2 -- EnumDescriptor base ctor with name "MoveState", vtable install, empty tables (decompiled; cf. 0x37148c). Pairs are registered by the addPair stubs.
    crate::enum_desc::EnumDesc::new("MoveState")
}

// 0x6295fc — __ZN3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,int>::~PropDescriptor()")]
pub fn stub_0x6295fc() {
    // IDA 0x6295fc: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x629630 — __ZN3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,bool>::~PropDescriptor()")]
pub fn stub_0x629630() {
    // IDA 0x629630: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x62965c — __ZN3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::~EnumPropDescriptor()")]
pub fn stub_0x62965c() {
    // IDA 0x62965c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x629680 — __ZN3RBX10Reflection9EventDescINS_18SkateboardPlatformEFvNS2_9MoveStateES3_EN3rbx6signalIS4_EEMS2_S7_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::SkateboardPlatform,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState),rbx::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>,rbx::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)> RBX::SkateboardPlatform::*>::~EventDesc()")]
pub fn stub_0x629680() {
    // IDA 0x629680: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x6296a4 — __ZN3RBX10Reflection9EventDescINS_18SkateboardPlatformEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx6signalIS7_EEMS2_SA_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::SkateboardPlatform,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::SkateboardPlatform::*>::~EventDesc()")]
pub fn stub_0x6296a4() {
    // IDA 0x6296a4: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x6296c8 — __ZN3RBX10Reflection9EventDescINS_18SkateboardPlatformEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::SkateboardPlatform,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::SkateboardPlatform::*>::~EventDesc()")]
pub fn stub_0x6296c8() {
    // IDA 0x6296c8: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x6296f4 — __ZN3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEED1Ev
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::~RefPropDescriptor()")]
pub fn stub_0x6296f4() {
    // IDA 0x6296f4: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x629728 — __ZN3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEED1Ev
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::~RefPropDescriptor()")]
pub fn stub_0x629728() {
    // IDA 0x629728: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x629754 — __ZN3RBX10Reflection13BoundFuncDescINS_18SkateboardPlatformEFvN3G3D7Vector3EELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::SkateboardPlatform,void ()(G3D::Vector3),1>::~BoundFuncDesc()")]
pub fn stub_0x629754() {
    // IDA 0x629754: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x629794 — __ZN3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::addPair(RBX::SkateboardPlatform::MoveState,char const*)")]
pub fn stub_0x629794(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0x629794: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0x62b180 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_18SkateboardPlatformES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::SkateboardPlatform,RBX::SkateboardPlatform>(rbx_core::SharedPtr<RBX::SkateboardPlatform> const*,RBX::SkateboardPlatform *)const")]
pub fn stub_0x62b180() {
    // IDA 0x62b180: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0x62e0b8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_20SkateboardControllerES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::SkateboardController,RBX::SkateboardController>(rbx_core::SharedPtr<RBX::SkateboardController> const*,RBX::SkateboardController *)const")]
pub fn stub_0x62e0b8() {
    // IDA 0x62e0b8: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0x62f4f8 — __ZN3RBX10Reflection13BoundFuncDescINS_18SkateboardPlatformEFvN3G3D7Vector3EELi1EEC2EMS2_FvS4_EPKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::SkateboardPlatform,void ()(G3D::Vector3),1>::BoundFuncDesc(void (RBX::SkateboardPlatform::*)(G3D::Vector3),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x62f4f8(
    name: &str,
    member: usize,
    arg_name: &str,
    permissions: u32,
    attributes: u32,
) -> BoundFunc {
    // IDA 0x62f4f8: `BoundFuncDesc<SkateboardPlatform, void(Vector3)>` ctor:
    // member pair at +40, default slot at +48 cleared (0x62f57e),
    // `getSingleton<void>` + in-ctor `declareSignature` (0x62f5a0+).
    let mut func = BoundFunc {
        name: name.to_owned(),
        member,
        signature: Signature { return_type: "void", args: Vec::new() },
        permissions,
        attributes,
    };
    stub_0x62f670(&mut func, arg_name);
    func
}

// 0x62f670 — __ZN3RBX10Reflection13BoundFuncDescINS_18SkateboardPlatformEFvN3G3D7Vector3EELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::SkateboardPlatform,void ()(G3D::Vector3),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x62f670(func: &mut BoundFunc, arg_name: &str) {
    // IDA 0x62f670: `declareSignature`: `void` return `Type` at +28
    // (0x62f680), `Name::declare` (0x62f68a), `getSingleton<Vector3>`
    // (0x62f68c), `addArgument` (0x62f69e).
    func.signature.return_type = "void";
    func.signature.args.push((arg_name.to_owned(), "Vector3"));
}

// 0x62f6a0 — __ZN3RBX10Reflection13BoundFuncDescINS_18SkateboardPlatformEFvN3G3D7Vector3EELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::SkateboardPlatform,void ()(G3D::Vector3),1>::~BoundFuncDesc()")]
pub fn stub_0x62f6a0() {
    // IDA 0x62f6a0: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x62f774 — __ZNK3RBX10Reflection13BoundFuncDescINS_18SkateboardPlatformEFvN3G3D7Vector3EELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::SkateboardPlatform,void ()(G3D::Vector3),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x62f774(args: &Arguments, call: &dyn Fn(Vector3)) {
    // IDA 0x62f774: void `execute`: `ArgHelper::getArg<Vector3, 1>` into a
    // 12-byte temp (0x62f798), member-pointer decode (0x62f79c-0x62f7a8),
    // invoke with the three lanes (0x62f7b6). No `CallHelper` hop here.
    let arg = match args.args.first() {
        Some(Value::Nil) | None => panic!("Argument 1 missing or nil (IDA 0x62f774)"),
        Some(Value::Vector3(v)) => *v,
        Some(other) => panic!("Variant::convert<Vector3> on {other:?} (IDA 0x62f774)"),
    };
    call(arg);
}

// 0x62f7b8 — __ZN3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEEC2IMS2_KFPS3_vEiEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::RefPropDescriptor<RBX::Humanoid* (RBX::SkateboardPlatform::*)(void)const,int>(char const*,char const*,RBX::Humanoid* (RBX::SkateboardPlatform::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x62f7b8(
    name: &str,
    category: &str,
    expected: &'static str,
    attributes: u32,
    permissions: u32,
) -> RefProp {
    // IDA 0x62f7b8: `RefPropDescriptor<SkateboardPlatform, Humanoid>` ctor
    // (same shape as 0x62307c).
    RefProp::new(name, category, expected, attributes, permissions)
}

// 0x62f85c — __ZN3RBX10Reflection7RefTypeIPNS_8HumanoidEE9singletonEv
#[doc(alias = "RBX::Reflection::RefType<RBX::Humanoid *>::singleton(void)")]
pub fn stub_0x62f85c() -> &'static RefType {
    // IDA 0x62f85c: `RefType<Humanoid*>::singleton`: guard-once
    // (`__cxa_guard_acquire`, 0x62f8b8) `Type::Type<Humanoid*>` construct
    // (0x62f8e6), vtable install (0x62f8fa), return the static (0x62f928).
    // Rust: `LazyLock`; the destructor runs at process exit.
    static TYPE: std::sync::LazyLock<RefType> = std::sync::LazyLock::new(|| RefType { target: "Humanoid" });
    &TYPE
}

// 0x62f954 — __ZN3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEED0Ev
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::~RefPropDescriptor()")]
pub fn stub_0x62f954() {
    // IDA 0x62f954: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x62f984 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::isReadOnly(void)const")]
pub fn stub_0x62f984() {
    // IDA 0x62f984: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x62f994 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::isWriteOnly(void)const")]
pub fn stub_0x62f994() {
    // IDA 0x62f994: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x62f9a4 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x62f9a4(a: &RefProp, b: &RefProp) -> bool {
    // IDA 0x62f9a4: `equalValues` raw-pointer compare (same shape as
    // 0x623170).
    a.target == b.target
}

// 0x62f9cc — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x62f9cc(prop: &RefProp) -> Value {
    // IDA 0x62f9cc: `getVariant` via `shared_from<Humanoid>` (same shape as
    // 0x623198).
    match prop.target {
        Some(id) => Value::Instance(id),
        None => Value::Nil,
    }
}

// 0x62fae4 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x62fae4(prop: &mut RefProp, value: &Value) {
    // IDA 0x62fae4: `setVariant` through the checked entry (same shape as
    // 0x6232b0).
    match value {
        Value::Instance(id) => prop.target = Some(*id),
        Value::Nil => prop.target = None,
        other => panic!("Variant::get<shared_ptr<DescribedBase>> on {other:?} (IDA 0x62fae4)"),
    }
}

// 0x62fbac — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x62fbac(dst: &mut RefProp, src: &RefProp) {
    // IDA 0x62fbac: `copyValue` (same shape as 0x623378).
    dst.target = src.target;
}

// 0x62fbd0 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_0x62fbd0(prop: &RefProp) -> Option<u32> {
    // IDA 0x62fbd0: `writeValue` (same shape as 0x62339c).
    prop.target
}

// 0x62fca4 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x62fca4(prop: &mut RefProp, id: Option<u32>) {
    // IDA 0x62fca4: `readValue` (same shape as 0x623470).
    prop.target = id;
}

// 0x62fcc8 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE11getRefValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::getRefValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x62fcc8(prop: &RefProp) -> Option<u32> {
    // IDA 0x62fcc8: `getRefValue` (same shape as 0x623494).
    prop.target
}

// 0x62fcdc — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE11setRefValueEPNS0_13DescribedBaseES6_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
pub fn stub_0x62fcdc(prop: &mut RefProp, id: Option<u32>, actual: Option<&'static str>) {
    // IDA 0x62fcdc: `setRefValue` with the `__dynamic_cast<Humanoid>` check
    // (same shape as 0x6234a8).
    match (id, actual) {
        (None, _) => prop.target = None,
        (Some(id), Some(t)) if t != prop.expected => panic!("std::bad_cast (IDA 0x62fcdc): {t} is not a {}", prop.expected),
        (Some(id), _) => prop.target = Some(id),
    }
}

// 0x62fd58 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
pub fn stub_0x62fd58(prop: &mut RefProp, id: Option<u32>) {
    // IDA 0x62fd58: `setRefValueUnsafe` (same shape as 0x623524/0x623fb8).
    prop.target = id;
}

// 0x62fd78 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
pub fn stub_0x62fd78(prop: &mut RefProp, id: u32) {
    // IDA 0x62fd78: `assignIDREF` (same shape as 0x623544/0x623fd8).
    prop.target = Some(id);
}

// 0x62fe58 — __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
#[doc(alias = "non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
pub fn stub_0x62fe58() {
    // IDA 0x62fe58: non-virtual thunk to `RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::assignIDREF( int a1, int a2, int a3, int a4, ` — this/arg-adjust + tail-call (arg a1 -= 40) (decompiled). Rust uses static dispatch; no thunk needed. Target unmodeled: cutover no-op.
}

// 0x62fe60 — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEPNS_8HumanoidEE7GetImplIMS2_KFS4_vEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid *>::GetImpl<RBX::Humanoid * (RBX::SkateboardPlatform::*)(void)const>::isReadOnly(void)const")]
pub fn stub_0x62fe60() {
    // IDA 0x62fe60: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x62fe64 — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEPNS_8HumanoidEE7GetImplIMS2_KFS4_vEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid *>::GetImpl<RBX::Humanoid * (RBX::SkateboardPlatform::*)(void)const>::isWriteOnly(void)const")]
pub fn stub_0x62fe64() {
    // IDA 0x62fe64: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x62fe68 — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEPNS_8HumanoidEE7GetImplIMS2_KFS4_vEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid *>::GetImpl<RBX::Humanoid * (RBX::SkateboardPlatform::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x62fe68(prop: &RefProp) -> Option<u32> {
    // IDA 0x62fe68: `GetImpl<Humanoid*>::getValue`: header strip, getter
    // member-pointer decode, invoke.
    prop.target
}

// 0x62fe88 — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEPNS_8HumanoidEE7GetImplIMS2_KFS4_vEE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid *>::GetImpl<RBX::Humanoid * (RBX::SkateboardPlatform::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::Humanoid * const&)const")]
pub fn stub_0x62fe88() {
    // IDA 0x62fe88: `GetImpl<Humanoid*>::setValue` (read-only ref prop):
    // `throw runtime_error("can't set value")`. Rust cutover panics.
    panic!("can't set value (IDA 0x62fe88)");
}

// 0x62ffa8 — __ZN3RBX10Reflection7RefTypeIPNS_8HumanoidEED1Ev
#[doc(alias = "RBX::Reflection::RefType<RBX::Humanoid *>::~RefType()")]
pub fn stub_0x62ffa8() {
    // IDA 0x62ffa8: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x62ffac — __ZN3RBX10Reflection4TypeC2IPNS_8HumanoidEEEPKcS6_PT_
#[doc(alias = "RBX::Reflection::Type::Type<RBX::Humanoid *>(char const*,char const*,RBX::Humanoid * *)")]
pub fn stub_0x62ffac(name: &str, category: &str) -> ClassType {
    // IDA 0x62ffac: `Type::Type<Humanoid*>`: `Descriptor` init, vtable
    // install, `typeinfo for'Humanoid*` (0x62ffe4), `Name::declare` the tag
    // (0x62ffec-0x62fff6), `ReleaseAssert(!tag.empty())` (Type.h:77), then
    // `addToAllTypes` (0x630048). Rust: assert + registry push.
    assert!(!name.is_empty(), "!this->tag.empty() include/reflection/Type.h:77 (IDA 0x62ffac)");
    register_type(name);
    ClassType { name: name.to_owned(), category: category.to_owned() }
}

// 0x630058 — __ZN3RBX10Reflection7RefTypeIPNS_8HumanoidEED0Ev
#[doc(alias = "RBX::Reflection::RefType<RBX::Humanoid *>::~RefType()")]
pub fn stub_0x630058() {
    // IDA 0x630058: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x63005c — __ZN3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEEC2IMS2_KFPS3_vEiEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::RefPropDescriptor<RBX::SkateboardController* (RBX::SkateboardPlatform::*)(void)const,int>(char const*,char const*,RBX::SkateboardController* (RBX::SkateboardPlatform::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x63005c(
    name: &str,
    category: &str,
    expected: &'static str,
    attributes: u32,
    permissions: u32,
) -> RefProp {
    // IDA 0x63005c: `RefPropDescriptor<SkateboardPlatform,
    // SkateboardController>` ctor (same shape as 0x62307c).
    RefProp::new(name, category, expected, attributes, permissions)
}

// 0x630100 — __ZN3RBX10Reflection7RefTypeIPNS_20SkateboardControllerEE9singletonEv
#[doc(alias = "RBX::Reflection::RefType<RBX::SkateboardController *>::singleton(void)")]
pub fn stub_0x630100() -> &'static RefType {
    // IDA 0x630100: `RefType<SkateboardController*>::singleton`:
    // guard-once construct + return the static (same shape as 0x62f85c).
    static TYPE: std::sync::LazyLock<RefType> = std::sync::LazyLock::new(|| RefType { target: "SkateboardController" });
    &TYPE
}

// 0x6301f8 — __ZN3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEED0Ev
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::~RefPropDescriptor()")]
pub fn stub_0x6301f8() {
    // IDA 0x6301f8: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x630228 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::isReadOnly(void)const")]
pub fn stub_0x630228() {
    // IDA 0x630228: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x630238 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::isWriteOnly(void)const")]
pub fn stub_0x630238() {
    // IDA 0x630238: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x630248 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x630248(a: &RefProp, b: &RefProp) -> bool {
    // IDA 0x630248: `equalValues` raw-pointer compare (same shape as
    // 0x623170).
    a.target == b.target
}

// 0x630270 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x630270(prop: &RefProp) -> Value {
    // IDA 0x630270: `getVariant` via `shared_from<SkateboardController>`
    // (same shape as 0x623198).
    match prop.target {
        Some(id) => Value::Instance(id),
        None => Value::Nil,
    }
}

// 0x630388 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x630388(prop: &mut RefProp, value: &Value) {
    // IDA 0x630388: `setVariant` through the checked entry (same shape as
    // 0x6232b0).
    match value {
        Value::Instance(id) => prop.target = Some(*id),
        Value::Nil => prop.target = None,
        other => panic!("Variant::get<shared_ptr<DescribedBase>> on {other:?} (IDA 0x630388)"),
    }
}

// 0x630450 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x630450(dst: &mut RefProp, src: &RefProp) {
    // IDA 0x630450: `copyValue` (same shape as 0x623378).
    dst.target = src.target;
}

// 0x630474 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_0x630474(prop: &RefProp) -> Option<u32> {
    // IDA 0x630474: `writeValue` (same shape as 0x62339c).
    prop.target
}

// 0x630548 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x630548(prop: &mut RefProp, id: Option<u32>) {
    // IDA 0x630548: `readValue` (same shape as 0x623470).
    prop.target = id;
}

// 0x63056c — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE11getRefValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::getRefValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x63056c(prop: &RefProp) -> Option<u32> {
    // IDA 0x63056c: `getRefValue` (same shape as 0x623494).
    prop.target
}

// 0x630580 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE11setRefValueEPNS0_13DescribedBaseES6_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
pub fn stub_0x630580(prop: &mut RefProp, id: Option<u32>, actual: Option<&'static str>) {
    // IDA 0x630580: `setRefValue` with the
    // `__dynamic_cast<SkateboardController>` check (same shape as 0x6234a8).
    match (id, actual) {
        (None, _) => prop.target = None,
        (Some(id), Some(t)) if t != prop.expected => panic!("std::bad_cast (IDA 0x630580): {t} is not a {}", prop.expected),
        (Some(id), _) => prop.target = Some(id),
    }
}

// 0x6305fc — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
pub fn stub_0x6305fc(prop: &mut RefProp, id: Option<u32>) {
    // IDA 0x6305fc: `setRefValueUnsafe` (same shape as 0x623524).
    prop.target = id;
}

// 0x63061c — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
pub fn stub_0x63061c(prop: &mut RefProp, id: u32) {
    // IDA 0x63061c: `assignIDREF` (same shape as 0x623544).
    prop.target = Some(id);
}

// 0x6306fc — __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
#[doc(alias = "non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
pub fn stub_0x6306fc() {
    // IDA 0x6306fc: non-virtual thunk to `RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::assignIDREF( int a1) { return RBX::Reflection::RefPropDescri` — this/arg-adjust + tail-call (arg a1 -= 40) (decompiled). Rust uses static dispatch; no thunk needed. Target unmodeled: cutover no-op.
}

// 0x630874 — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEPNS_20SkateboardControllerEE7GetImplIMS2_KFS4_vEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController *>::GetImpl<RBX::SkateboardController * (RBX::SkateboardPlatform::*)(void)const>::isReadOnly(void)const")]
pub fn stub_0x630874() {
    // IDA 0x630874: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x630878 — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEPNS_20SkateboardControllerEE7GetImplIMS2_KFS4_vEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController *>::GetImpl<RBX::SkateboardController * (RBX::SkateboardPlatform::*)(void)const>::isWriteOnly(void)const")]
pub fn stub_0x630878() {
    // IDA 0x630878: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x63087c — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEPNS_20SkateboardControllerEE7GetImplIMS2_KFS4_vEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController *>::GetImpl<RBX::SkateboardController * (RBX::SkateboardPlatform::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x63087c(prop: &RefProp) -> Option<u32> {
    // IDA 0x63087c: `GetImpl<SkateboardController*>::getValue`: header
    // strip, getter member-pointer decode, invoke.
    prop.target
}

// 0x63089c — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEPNS_20SkateboardControllerEE7GetImplIMS2_KFS4_vEE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController *>::GetImpl<RBX::SkateboardController * (RBX::SkateboardPlatform::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::SkateboardController * const&)const")]
pub fn stub_0x63089c() {
    // IDA 0x63089c: `GetImpl<SkateboardController*>::setValue` (read-only):
    // `throw runtime_error("can't set value")`.
    panic!("can't set value (IDA 0x63089c)");
}

// 0x6309bc — __ZN3RBX10Reflection7RefTypeIPNS_20SkateboardControllerEED1Ev
#[doc(alias = "RBX::Reflection::RefType<RBX::SkateboardController *>::~RefType()")]
pub fn stub_0x6309bc() {
    // IDA 0x6309bc: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x6309c0 — __ZN3RBX10Reflection4TypeC2IPNS_20SkateboardControllerEEEPKcS6_PT_
#[doc(alias = "RBX::Reflection::Type::Type<RBX::SkateboardController *>(char const*,char const*,RBX::SkateboardController * *)")]
pub fn stub_0x6309c0(name: &str, category: &str) -> ClassType {
    // IDA 0x6309c0: `Type::Type<SkateboardController*>`: `Descriptor` init,
    // tag declare, non-empty assert (Type.h:77), `addToAllTypes` (same shape
    // as 0x62ffac).
    assert!(!name.is_empty(), "!this->tag.empty() include/reflection/Type.h:77 (IDA 0x6309c0)");
    register_type(name);
    ClassType { name: name.to_owned(), category: category.to_owned() }
}

// 0x630a6c — __ZN3RBX10Reflection7RefTypeIPNS_20SkateboardControllerEED0Ev
#[doc(alias = "RBX::Reflection::RefType<RBX::SkateboardController *>::~RefType()")]
pub fn stub_0x630a6c() {
    // IDA 0x630a6c: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

/// `any_cast<shared_ptr<Instance>>` on a fire arg (IDA 0x630ecc/0x63143a).
pub fn instance_arg(value: &Value) -> u32 {
    match value {
        Value::Instance(id) => *id,
        other => panic!("any_cast<shared_ptr<Instance>> on {other:?}"),
    }
}

/// `any_cast<MoveState>` on a fire arg (IDA 0x631994/0x6319aa).
pub fn move_state_arg(value: &Value) -> i32 {
    match value {
        Value::EnumValue(v) => *v,
        Value::Int(v) => *v,
        other => panic!("any_cast<MoveState> on {other:?}"),
    }
}

/// `RBX::Reflection::EventSource` for a one-`Instance`
/// `rbx::signal` (IDA 0x630ca8/0x630dfc/0x630f5c).
#[derive(Default)]
pub struct EventSource1Instance {
    signal: Signal<u32>,
    holders: parking_lot::Mutex<Vec<(SharedPtr<SlotWrapper>, SharedPtr<dyn Fn(u32) + Send + Sync>)>>,
}

impl EventSource1Instance {
    pub fn disconnect_all(&self) {
        self.holders.lock().clear();
        self.signal.disconnect_all();
    }
}

/// `RBX::Reflection::EventSource` for a two-`Instance`
/// `rbx::signal` (IDA 0x631214/0x631368/0x631518).
#[derive(Default)]
pub struct EventSource2Instance {
    signal: Signal<(u32, u32)>,
    holders: parking_lot::Mutex<Vec<(SharedPtr<SlotWrapper>, SharedPtr<dyn Fn((u32, u32)) + Send + Sync>)>>,
}

impl EventSource2Instance {
    pub fn disconnect_all(&self) {
        self.holders.lock().clear();
        self.signal.disconnect_all();
    }
}

/// `RBX::Reflection::EventSource` for a `(MoveState, MoveState)`
/// `rbx::signal` (IDA 0x6317d0/0x631924/0x6319c0).
#[derive(Default)]
pub struct EventSource2MoveState {
    signal: Signal<(i32, i32)>,
    holders: parking_lot::Mutex<Vec<(SharedPtr<SlotWrapper>, SharedPtr<dyn Fn((i32, i32)) + Send + Sync>)>>,
}

impl EventSource2MoveState {
    pub fn disconnect_all(&self) {
        self.holders.lock().clear();
        self.signal.disconnect_all();
    }
}

/// `Singleton<EnumDesc<MoveState>>` link for 0x6332b0 (cf. 0x4aaef8):
/// guard-once table; item pairs register in the singleton C2.
static MOVE_STATE_DESC: std::sync::LazyLock<crate::enum_desc::EnumDesc> =
    std::sync::LazyLock::new(|| crate::enum_desc::EnumDesc::new("MoveState"));

/// `boost::_bi::bind_t<mf2<GenericSlotWrapper, MoveState, MoveState>, ...>`
/// (IDA 0x631b4c, same shape as `BoundExplosionSlot` in descriptor.rs):
/// the member triple folds into the target.
#[derive(Clone)]
pub struct BoundMoveStateSlot {
    pub target: SharedPtr<SlotWrapper>,
}

/// `boost::function2<void, MoveState, MoveState>` holding one bound slot
/// (IDA 0x631fc8, same shape as `ExplosionSlotFunction`).
#[derive(Default, Clone)]
pub struct MoveStateSlotFunction {
    bound: Option<BoundMoveStateSlot>,
}

/// `boost::detail::function::functor_manager_operation_type` as switched at
/// 0x6320c0: op 4 is `get_functor_type_tag` (same shape as `FunctorOp`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveStateFunctorOp {
    Clone = 0,
    Move = 1,
    Destroy = 2,
    CheckNoCopy = 3,
    GetType = 4,
}

/// typeinfo name compared by `manage` case 4 (cf. 0x4a490a).
pub const MOVE_STATE_BIND_T_TYPEINFO: &str = "bind_t<mf2<GenericSlotWrapper,MoveState,MoveState>>";

// 0x630a70 — __ZN3RBX10Reflection9EventDescINS_18SkateboardPlatformEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::SkateboardPlatform,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::SkateboardPlatform::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::SkateboardPlatform::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x630a70(
    name: &str,
    category: &str,
    title: &str,
    member: usize,
    arg_name: &str,
    permissions: u32,
    attributes: u32,
) -> EventDesc {
    // IDA 0x630a70: `EventDesc<SkateboardPlatform, void(Instance)>` ctor:
    // member-signal pointer stored at +40 (`v39[10] = a2`, 0x630aea),
    // one-item signature with `getSingleton<shared_ptr<Instance>>`
    // (0x630b14-0x630b42).
    EventDesc {
        name: name.to_owned(),
        category: category.to_owned(),
        title: title.to_owned(),
        member,
        signature: Signature {
            return_type: "void",
            args: vec![(arg_name.to_owned(), "Instance")],
        },
        permissions,
        attributes,
    }
}

// 0x630bf4 — __ZN3RBX10Reflection9EventDescINS_18SkateboardPlatformEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::SkateboardPlatform,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::SkateboardPlatform::*>::~EventDesc()")]
pub fn stub_0x630bf4() {
    // IDA 0x630bf4: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x630ca8 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_18SkateboardPlatformEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::SkateboardPlatform,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::SkateboardPlatform::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_0x630ca8(src: &EventSource1Instance, wrapper: SharedPtr<SlotWrapper>) {
    // IDA 0x630ca8: `EventDescImpl<1, Instance>::connectGeneric`:
    // `signal::connect<function1>` on the member signal, clear the temp
    // (0x630d34-0x630d5a).
    let w = SharedPtr::clone(&wrapper);
    let slot = SharedPtr::new(move |id: u32| (w.invoke)(&[Value::Instance(id)]));
    src.signal.connect(SharedPtr::clone(&slot));
    src.holders.lock().push((wrapper, slot));
}

// 0x630dfc — __ZNK3RBX10Reflection13EventDescImplILi1ENS_18SkateboardPlatformEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::SkateboardPlatform,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::SkateboardPlatform::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x630dfc(src: &EventSource1Instance, args: &[Value]) {
    // IDA 0x630dfc: `fireEvent` for the 1-Instance event:
    // `ReleaseAssert(args.size() == 1)` (Event.h:320), `any_cast<Instance>`
    // (0x630ecc), `signal_with_args<1>` invoke (0x630eac-0x630ef2).
    assert!(args.len() == 1, "args.size() == 1 include/Reflection/Event.h:320 (IDA 0x630dfc)");
    src.signal.fire(instance_arg(&args[0]));
}

// 0x630f5c — __ZNK3RBX10Reflection13EventDescBaseINS_18SkateboardPlatformEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::SkateboardPlatform,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::SkateboardPlatform::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x630f5c(src: &EventSource1Instance) {
    // IDA 0x630f5c: `EventDescBase::disconnectAll` for the 1-Instance
    // event.
    src.disconnect_all();
}

// 0x630f70 — __ZN3RBX10Reflection9EventDescINS_18SkateboardPlatformEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::SkateboardPlatform,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::SkateboardPlatform::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::SkateboardPlatform::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x630f70(
    name: &str,
    category: &str,
    title: &str,
    member: usize,
    arg0_name: &str,
    arg1_name: &str,
    permissions: u32,
    attributes: u32,
) -> EventDesc {
    // IDA 0x630f70: `EventDesc<SkateboardPlatform, void(Instance,
    // Instance)>` ctor: member at +40, two-item signature with
    // `getSingleton<shared_ptr<Instance>>` types (0x631054-0x63107e).
    EventDesc {
        name: name.to_owned(),
        category: category.to_owned(),
        title: title.to_owned(),
        member,
        signature: Signature {
            return_type: "void",
            args: vec![
                (arg0_name.to_owned(), "Instance"),
                (arg1_name.to_owned(), "Instance"),
            ],
        },
        permissions,
        attributes,
    }
}

// 0x631160 — __ZN3RBX10Reflection9EventDescINS_18SkateboardPlatformEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx6signalIS7_EEMS2_SA_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::SkateboardPlatform,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::SkateboardPlatform::*>::~EventDesc()")]
pub fn stub_0x631160() {
    // IDA 0x631160: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x631214 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_18SkateboardPlatformEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::SkateboardPlatform,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::SkateboardPlatform::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_0x631214(src: &EventSource2Instance, wrapper: SharedPtr<SlotWrapper>) {
    // IDA 0x631214: `EventDescImpl<2, Instance>::connectGeneric`:
    // `signal::connect<function2>` on the member signal, clear the temp
    // (0x6312a0-0x6312c6).
    let w = SharedPtr::clone(&wrapper);
    let slot = SharedPtr::new(move |(a, b): (u32, u32)| {
        (w.invoke)(&[Value::Instance(a), Value::Instance(b)]);
    });
    src.signal.connect(SharedPtr::clone(&slot));
    src.holders.lock().push((wrapper, slot));
}

// 0x631368 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_18SkateboardPlatformEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::SkateboardPlatform,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::SkateboardPlatform::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x631368(src: &EventSource2Instance, args: &[Value]) {
    // IDA 0x631368: `fireEvent` for the 2-Instance event: two
    // `any_cast<shared_ptr<Instance>>` (+4 at 0x63143a, +72 at 0x63145a —
    // 68-byte Variant stride), `signal_with_args<2>` invoke.
    assert!(args.len() == 2, "args.size() == 2 (IDA 0x631368)");
    let (a, b) = (instance_arg(&args[0]), instance_arg(&args[1]));
    src.signal.fire((a, b));
}

// 0x631518 — __ZNK3RBX10Reflection13EventDescBaseINS_18SkateboardPlatformEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::SkateboardPlatform,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::SkateboardPlatform::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x631518(src: &EventSource2Instance) {
    // IDA 0x631518: `EventDescBase::disconnectAll` for the 2-Instance
    // event.
    src.disconnect_all();
}

// 0x63152c — __ZN3RBX10Reflection9EventDescINS_18SkateboardPlatformEFvNS2_9MoveStateES3_EN3rbx6signalIS4_EEMS2_S7_EC2ES8_PKcSB_SB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::SkateboardPlatform,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState),rbx::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>,rbx::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)> RBX::SkateboardPlatform::*>::EventDesc(rbx::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)> RBX::SkateboardPlatform::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x63152c(
    name: &str,
    category: &str,
    title: &str,
    member: usize,
    arg0_name: &str,
    arg1_name: &str,
    permissions: u32,
    attributes: u32,
) -> EventDesc {
    // IDA 0x63152c: `EventDesc<SkateboardPlatform, void(MoveState,
    // MoveState)>` ctor: member at +40, two-item signature with
    // `getSingleton<MoveState>` types (0x631610-0x63163a).
    EventDesc {
        name: name.to_owned(),
        category: category.to_owned(),
        title: title.to_owned(),
        member,
        signature: Signature {
            return_type: "void",
            args: vec![
                (arg0_name.to_owned(), "MoveState"),
                (arg1_name.to_owned(), "MoveState"),
            ],
        },
        permissions,
        attributes,
    }
}

// 0x63171c — __ZN3RBX10Reflection9EventDescINS_18SkateboardPlatformEFvNS2_9MoveStateES3_EN3rbx6signalIS4_EEMS2_S7_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::SkateboardPlatform,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState),rbx::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>,rbx::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)> RBX::SkateboardPlatform::*>::~EventDesc()")]
pub fn stub_0x63171c() {
    // IDA 0x63171c: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x6317d0 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_18SkateboardPlatformEFvNS2_9MoveStateES3_EN3rbx6signalIS4_EEMS2_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::SkateboardPlatform,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState),rbx::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>,rbx::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)> RBX::SkateboardPlatform::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_0x6317d0(src: &EventSource2MoveState, wrapper: SharedPtr<SlotWrapper>) {
    // IDA 0x6317d0: `EventDescImpl<2, MoveState>::connectGeneric`:
    // `signal::connect<function2>` on the member signal, clear the temp
    // (0x63185c-0x631882, same shape as 0x631214).
    let w = SharedPtr::clone(&wrapper);
    let slot = SharedPtr::new(move |(a, b): (i32, i32)| {
        (w.invoke)(&[Value::EnumValue(a), Value::EnumValue(b)]);
    });
    src.signal.connect(SharedPtr::clone(&slot));
    src.holders.lock().push((wrapper, slot));
}

// 0x631924 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_18SkateboardPlatformEFvNS2_9MoveStateES3_EN3rbx6signalIS4_EEMS2_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::SkateboardPlatform,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState),rbx::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>,rbx::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)> RBX::SkateboardPlatform::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x631924(src: &EventSource2MoveState, args: &[Value]) {
    // IDA 0x631924: `fireEvent` for the MoveState pair:
    // `ReleaseAssert(args.size() == 2)` (Event.h:349 — 136 bytes = 2 x
    // 68-byte Variants, 0x631942-0x631984), two `any_cast<MoveState>`
    // (0x631994/0x6319aa), `signal_with_args<2>` invoke.
    assert!(args.len() == 2, "args.size() == 2 include/Reflection/Event.h:349 (IDA 0x631924)");
    let (a, b) = (move_state_arg(&args[0]), move_state_arg(&args[1]));
    src.signal.fire((a, b));
}

// 0x6319c0 — __ZNK3RBX10Reflection13EventDescBaseINS_18SkateboardPlatformEFvNS2_9MoveStateES3_EN3rbx6signalIS4_EEMS2_S7_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::SkateboardPlatform,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState),rbx::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>,rbx::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)> RBX::SkateboardPlatform::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x6319c0(src: &EventSource2MoveState) {
    // IDA 0x6319c0: `EventDescBase::disconnectAll` for the MoveState pair.
    src.disconnect_all();
}

// 0x631b4c — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS1_18SkateboardPlatform9MoveStateES7_NS_10shared_ptrIS3_EENS_3argILi1EEENSA_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISF_T0_T1_T2_EENSD_9list_av_3IT3_T4_T5_E4typeEEEMSI_FSF_SJ_SK_ESN_SO_SP_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list_av_3<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")]
pub fn stub_0x631b4c(wrapper: SharedPtr<SlotWrapper>) -> BoundMoveStateSlot {
    // IDA 0x631b4c: `bind<mf2<GenericSlotWrapper, MoveState, MoveState>>`:
    // store the member triple plus the bound wrapper and the two
    // placeholders via `list3` (0x631bb6-0x631bdc, same shape as 0x4a3fac).
    // The member function is fixed (`execute2`), so the triple folds into
    // the target.
    BoundMoveStateSlot { target: wrapper }
}

// 0x631c68 — __ZN3RBX10Reflection18GenericSlotWrapper8execute2INS_18SkateboardPlatform9MoveStateES4_EEvRKT_RKT0_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute2<RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>(RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&)")]
pub fn stub_0x631c68(wrapper: &SlotWrapper, a: i32, b: i32) {
    // IDA 0x631c68: `GenericSlotWrapper::execute2<MoveState, MoveState>`:
    // 2-Variant vector with `MoveState` tags (0x631d00-0x631d2a), vf+8
    // dispatch (0x631d3a), teardown (0x631d44). Same shape as 0x4a40c8.
    (wrapper.invoke)(&[Value::EnumValue(a), Value::EnumValue(b)]);
}

// 0x631fc8 — __ZN5boost9function2IvN3RBX18SkateboardPlatform9MoveStateES3_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS3_SD_EENS6_5list3INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSK_ILi2EEEEEEEEEvT_
#[doc(alias = "void boost::function2<void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")]
pub fn stub_0x631fc8(func: &mut MoveStateSlotFunction, bound: &BoundMoveStateSlot) {
    // IDA 0x631fc8: `function2::assign_to<bind_t>` (0x632050): vtable
    // assign of the bound triple into the function buffer (same shape as
    // 0x4a442c).
    func.bound = Some(bound.clone());
}

// 0x6320c0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_18SkateboardPlatform9MoveStateESD_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSK_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x6320c0(
    op: MoveStateFunctorOp,
    src: &BoundMoveStateSlot,
    slot: &mut Option<BoundMoveStateSlot>,
) -> Option<&'static str> {
    // IDA 0x6320c0: `functor_manager<...>::manage`: op 4
    // (`get_functor_type_tag`) returns the `bind_t` typeinfo (0x6320d6);
    // anything else delegates to `manager` (0x6320c2, same shape as
    // 0x4a4524).
    if op == MoveStateFunctorOp::GetType {
        return Some(MOVE_STATE_BIND_T_TYPEINFO);
    }
    stub_0x6323b0(op, src, slot);
    None
}

// 0x6320dc — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_18SkateboardPlatform9MoveStateESD_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSK_ILi2EEEEEEEvSB_SB_E6invokeERNS1_15function_bufferESB_SB_
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>::invoke(boost::detail::function::function_buffer &,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)")]
pub fn stub_0x6320dc(bound: &BoundMoveStateSlot, a: i32, b: i32) {
    // IDA 0x6320dc: `void_function_obj_invoker2::invoke` tail-calls
    // `bind_t::operator()` (0x6320f2, same shape as 0x4a4540).
    stub_0x632394(bound, a, b);
}

// 0x6320f4 — __ZNK5boost6detail8function13basic_vtable2IvN3RBX18SkateboardPlatform9MoveStateES5_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_SF_EENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x6320f4(func: &mut MoveStateSlotFunction, bound: &BoundMoveStateSlot) -> bool {
    // IDA 0x6320f4: `basic_vtable2::assign_to` const overload (same shape
    // as 0x4a4554): store a clone, report success.
    func.bound = Some(bound.clone());
    true
}

// 0x6321dc — __ZNK5boost6detail8function13basic_vtable2IvN3RBX18SkateboardPlatform9MoveStateES5_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_SF_EENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x6321dc(func: &mut MoveStateSlotFunction, bound: &BoundMoveStateSlot) -> bool {
    // IDA 0x6321dc: `basic_vtable2::assign_to` function-obj-tag overload
    // (same shape as 0x4a463c): store a clone, report success.
    func.bound = Some(bound.clone());
    true
}

// 0x6322c0 — __ZNK5boost6detail8function13basic_vtable2IvN3RBX18SkateboardPlatform9MoveStateES5_E14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_SF_EENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable2<void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x6322c0(bound: &BoundMoveStateSlot) -> Box<BoundMoveStateSlot> {
    // IDA 0x6322c0: `basic_vtable2::assign_functor` (same shape as
    // 0x4a4720): heap-copy the bound triple.
    Box::new(bound.clone())
}

// 0x632394 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS4_18SkateboardPlatform9MoveStateESA_EENS0_5list3INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEENSH_ILi2EEEEEEclIS8_S8_EEvRT_RT0_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>(RBX::SkateboardPlatform::MoveState &,RBX::SkateboardPlatform::MoveState &)")]
pub fn stub_0x632394(bound: &BoundMoveStateSlot, a: i32, b: i32) {
    // IDA 0x632394: `bind_t::operator()<MoveState, MoveState>`: run the
    // member triple — `GenericSlotWrapper::execute2` on the bound wrapper
    // with the two args (same shape as 0x4a47f4).
    stub_0x631c68(&bound.target, a, b);
}

// 0x6323b0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_18SkateboardPlatform9MoveStateESD_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSK_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::SkateboardPlatform::MoveState const&,RBX::SkateboardPlatform::MoveState const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x6323b0(op: MoveStateFunctorOp, src: &BoundMoveStateSlot, slot: &mut Option<BoundMoveStateSlot>) -> bool {
    // IDA 0x6323b0: `functor_manager<...>::manager` for the MoveState
    // triple (same shape as 0x4a4810): clone/move/destroy/check on the
    // bound slot.
    match op {
        MoveStateFunctorOp::Clone => {
            *slot = Some(src.clone());
            true
        }
        MoveStateFunctorOp::Move => {
            *slot = Some(src.clone());
            true
        }
        MoveStateFunctorOp::Destroy => {
            *slot = None;
            true
        }
        MoveStateFunctorOp::CheckNoCopy => true,
        MoveStateFunctorOp::GetType => false,
    }
}

// 0x6332b0 — __ZN3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEEC2IMS2_KFS3_vEMS2_FvRKS3_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::EnumPropDescriptor<RBX::SkateboardPlatform::MoveState (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(RBX::SkateboardPlatform::MoveState const&)>(char const*,char const*,RBX::SkateboardPlatform::MoveState (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(RBX::SkateboardPlatform::MoveState const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x6332b0(
    name: &str,
    category: &str,
    initial: i32,
    attributes: u32,
    permissions: u32,
) -> EnumProp {
    // IDA 0x6332b0: `EnumPropDescriptor<SkateboardPlatform, MoveState>`
    // ctor: `new` the GetSetImpl, link the `EnumDesc<MoveState>` singleton
    // (same shape as 0x5f9d30/0x4a5834).
    EnumProp::new(name, category, initial, MOVE_STATE_DESC.clone(), attributes, permissions)
}

// 0x633464 — __ZN3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::~EnumPropDescriptor()")]
pub fn stub_0x633464() {
    // IDA 0x633464: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x633490 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::isReadOnly(void)const")]
pub fn stub_0x633490() {
    // IDA 0x633490: EnumPropDescriptor::isReadOnly/isWriteOnly -- forwards through the bound member descriptor at +44 (`(**(this+44))(this+44)`, decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

/// `RBX::SurfaceEnumPropDescriptor<face, E>` cutover (IDA 0x65e484): the
/// `EnumProp` shape plus the hardcoded `(NormalId)face` template face
/// (`Bottom` = 4, `Top` = 1, cf. `NORMAL_ID_DESC` in descriptor.rs). The
/// per-face getter/setter pair folds into direct field access.
#[derive(Debug, Clone)]
pub struct SurfaceEnumProp {
    pub name: String,
    pub category: String,
    pub attributes: u32,
    pub permissions: u32,
    pub face: u8,
    pub value: i32,
    pub enum_desc: crate::enum_desc::EnumDesc,
}

impl SurfaceEnumProp {
    pub fn new(
        name: &str,
        category: &str,
        face: u8,
        initial: i32,
        enum_desc: crate::enum_desc::EnumDesc,
        attributes: u32,
        permissions: u32,
    ) -> Self {
        Self {
            name: name.to_owned(),
            category: category.to_owned(),
            attributes,
            permissions,
            face,
            value: initial,
            enum_desc,
        }
    }

    /// `EnumDesc<T>::convertToIndex` (same shape as 0x4aa47c/0x60a9f8).
    pub fn convert_to_index(&self, value: i32) -> i32 {
        assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
        usize::try_from(value)
            .ok()
            .and_then(|slot| self.enum_desc.value_ordinals.get(slot).copied())
            .unwrap_or(-1)
    }
}

/// `RBX::SurfaceGetSet<face, T, ...>` bound storage (IDA 0x65ec4c/0x65edd8):
/// name/category/attributes/permissions, the hardcoded face, and the live
/// value. The `getter(face)`/`setter(face, v)` member pair collapses into
/// direct field access.
#[derive(Debug, Clone, PartialEq)]
pub struct SurfaceProp<T: Clone + PartialEq + std::fmt::Debug> {
    pub name: String,
    pub category: String,
    pub attributes: u32,
    pub permissions: u32,
    pub face: u8,
    pub value: T,
}

impl<T: Clone + PartialEq + std::fmt::Debug> SurfaceProp<T> {
    pub fn new(
        name: &str,
        category: &str,
        face: u8,
        value: T,
        attributes: u32,
        permissions: u32,
    ) -> Self {
        Self {
            name: name.to_owned(),
            category: category.to_owned(),
            attributes,
            permissions,
            face,
            value,
        }
    }
}

/// `Singleton<EnumDesc<SurfaceType>>` / `Singleton<EnumDesc<InputType>>`
/// links: guard-once tables; item pairs register in the singleton C2s.
static SURFACE_TYPE_DESC: std::sync::LazyLock<crate::enum_desc::EnumDesc> =
    std::sync::LazyLock::new(|| crate::enum_desc::EnumDesc::new("SurfaceType"));
static SURFACE_INPUT_DESC: std::sync::LazyLock<crate::enum_desc::EnumDesc> =
    std::sync::LazyLock::new(|| crate::enum_desc::EnumDesc::new("InputType"));

// 0x6334a0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::isWriteOnly(void)const")]
pub fn stub_0x6334a0(member_write_only: bool) -> bool {
    // IDA 0x6334a0: `EnumPropDescriptor<MoveState>::isWriteOnly` forwards
    // through the bound member descriptor at +44 (`(**(this+44))(this+44)`,
    // 0x6334ac). Member descriptors unmodeled: forward the member answer.
    member_write_only
}
// 0x65df90 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_65df90(prop: &mut SurfaceEnumProp, name: &str) -> bool {
    // IDA 0x65df90: `setStringValue` for (4, InputType).
    match prop.enum_desc.lookup_value(name) {
        Some(v) => {
            prop.value = v;
            true
        }
        None => false,
    }
}

// 0x65dff4 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_65dff4(prop: &SurfaceEnumProp) -> i32 {
    // IDA 0x65dff4: `writeValue` for (4, InputType).
    prop.value
}

// 0x65e014 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_65e014(prop: &mut SurfaceEnumProp, text: &str) -> bool {
    // IDA 0x65e014: `readValue` for (4, InputType).
    match prop.enum_desc.lookup_value(text) {
        Some(v) => {
            prop.value = v;
            true
        }
        None => false,
    }
}

// 0x65e26c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_65e26c(prop: &SurfaceEnumProp) -> i32 {
    // IDA 0x65e26c: `getIndexValue` for (4, InputType).
    prop.convert_to_index(prop.value)
}

// 0x65e2b4 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_65e2b4(prop: &mut SurfaceEnumProp, index: usize) -> bool {
    // IDA 0x65e2b4: `setIndexValue` for (4, InputType).
    match prop.enum_desc.values.get(index) {
        Some(&v) => {
            prop.value = v;
            true
        }
        None => false,
    }
}

// 0x65e310 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_65e310(prop: &SurfaceEnumProp) -> i32 {
    // IDA 0x65e310: `getEnumValue` for (4, InputType).
    prop.value
}

// 0x65e318 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_65e318(prop: &mut SurfaceEnumProp, value: i32) -> bool {
    // IDA 0x65e318: `setEnumValue` for (4, InputType).
    if prop.enum_desc.items.iter().any(|it| it.value == value) {
        prop.value = value;
        true
    } else {
        false
    }
}

// 0x65e38c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_65e38c(prop: &SurfaceEnumProp) -> Option<crate::enum_desc::EnumItem> {
    // IDA 0x65e38c: `getEnumItem` for (4, InputType).
    usize::try_from(prop.value)
        .ok()
        .and_then(|slot| prop.enum_desc.items_by_value.get(slot).copied().flatten())
        .and_then(|idx| prop.enum_desc.items.get(idx).cloned())
}

// 0x65e3dc — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_65e3dc(prop: &mut SurfaceEnumProp, name: &str) -> bool {
    // IDA 0x65e3dc: `setStringValue` (`Name` overload) for (4, InputType).
    match prop.enum_desc.lookup_value(name) {
        Some(v) => {
            prop.value = v;
            true
        }
        None => false,
    }
}

// 0x65e440 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE4ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE8getValueEPKNS_10Reflection13DescribedBaseE
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)4,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_65e440(prop: &SurfaceProp<i32>) -> i32 {
    // IDA 0x65e440: `SurfaceGetSet<4, InputType>::getValue`: invoke
    // `getter(Bottom)` — face hardcoded.
    prop.value
}

// 0x65e460 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE4ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE8setValueEPNS_10Reflection13DescribedBaseERKS3_
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)4,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::setValue(RBX::Reflection::DescribedBase *,RBX::LegacyController::InputType const&)const")]
pub fn stub_65e460(prop: &mut SurfaceProp<i32>, value: i32) {
    // IDA 0x65e460: `SurfaceGetSet<4, InputType>::setValue`: invoke
    // `setter(Bottom, value)` — face hardcoded.
    prop.value = value;
}

// 0x65e484 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEEC2IMNS_12PartInstanceEKFS2_S1_EMS5_FvS1_S2_EEEPKcSB_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::SurfaceEnumPropDescriptor<RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>(char const*,char const*,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType),RBX::Reflection::PropertyDescriptor::Functionality)")]
pub fn stub_65e484(
    name: &str,
    category: &str,
    initial: i32,
    attributes: u32,
    permissions: u32,
) -> SurfaceEnumProp {
    // IDA 0x65e484: `SurfaceEnumPropDescriptor<4, SurfaceType>` ctor:
    // `new` the `SurfaceGetSet<4>` holding the per-face getter/setter pair,
    // link the `EnumDesc<SurfaceType>` singleton (same shape as 0x5f9d30).
    SurfaceEnumProp::new(name, category, 4, initial, SURFACE_TYPE_DESC.clone(), attributes, permissions)
}

// 0x65e57c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES7_
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_65e57c(a: &SurfaceEnumProp, b: &SurfaceEnumProp) -> bool {
    // IDA 0x65e57c: `equalValues`: get both sides via the +44 member
    // (0x65e58c/0x65e5a2), compare.
    a.value == b.value
}

// 0x65e5a4 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS4_7VariantE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_65e5a4(prop: &SurfaceEnumProp) -> Value {
    // IDA 0x65e5a4: `getVariant`: get via the +44 member (0x65e5b4), tag
    // `Type::getSingleton<SurfaceType>` (0x65e5ba), `placement_any` pack
    // (0x65e5c8).
    Value::Int(prop.value)
}

// 0x65e5cc — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS4_7VariantE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_65e5cc(prop: &mut SurfaceEnumProp, value: &Value) {
    // IDA 0x65e5cc: `setVariant`: `any_cast<SurfaceType>` on a matching
    // payload (typeinfo + `"N3RBX11SurfaceTypeE"` check, 0x65e656), else
    // `Variant::convert<SurfaceType>`, then `setValue` via slot 12.
    prop.value = match value {
        Value::Int(v) => *v,
        Value::EnumValue(v) => *v,
        other => panic!("Variant::convert<SurfaceType> on {other:?} (IDA 0x65e5cc)"),
    };
}

// 0x65e724 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS5_
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_65e724(dst: &mut SurfaceEnumProp, src: &SurfaceEnumProp) {
    // IDA 0x65e724: `copyValue` get-then-set.
    dst.value = src.value;
}

// 0x65e74c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_65e74c(prop: &SurfaceEnumProp) -> String {
    // IDA 0x65e74c: `getStringValue` via `convertToString` (same shape as
    // 0x4aa1a0).
    prop.enum_desc.lookup_name(prop.value).unwrap_or_default().to_owned()
}

// 0x65e79c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_65e79c(prop: &mut SurfaceEnumProp, name: &str) -> bool {
    // IDA 0x65e79c: `setStringValue` lookup-and-set (same shape as
    // 0x4aa1c4).
    match prop.enum_desc.lookup_value(name) {
        Some(v) => {
            prop.value = v;
            true
        }
        None => false,
    }
}

// 0x65e800 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_65e800(prop: &SurfaceEnumProp) -> i32 {
    // IDA 0x65e800: `writeValue`: get via the +44 member (0x65e80e),
    // `clearValue` (0x65e814), store int tag 5 + value (0x65e81a-0x65e81c).
    prop.value
}

// 0x65e820 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_65e820(prop: &mut SurfaceEnumProp, text: &str) -> bool {
    // IDA 0x65e820: `readValue`: Xml pair text into a string, `Name::lookup`,
    // `convertToValue`; success sets. Empty/missing text leaves the object
    // untouched (same shape as 0x4aa224).
    match prop.enum_desc.lookup_value(text) {
        Some(v) => {
            prop.value = v;
            true
        }
        None => false,
    }
}

// 0x65ea78 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_65ea78(prop: &SurfaceEnumProp) -> i32 {
    // IDA 0x65ea78: `getIndexValue` via `convertToIndex` (same shape as
    // 0x4aa464).
    prop.convert_to_index(prop.value)
}

// 0x65eac0 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_65eac0(prop: &mut SurfaceEnumProp, index: usize) -> bool {
    // IDA 0x65eac0: `setIndexValue` bounds-check + set (same shape as
    // 0x4aa480).
    match prop.enum_desc.values.get(index) {
        Some(&v) => {
            prop.value = v;
            true
        }
        None => false,
    }
}

// 0x65eb1c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_65eb1c(prop: &SurfaceEnumProp) -> i32 {
    // IDA 0x65eb1c: `getEnumValue` (same shape as 0x4aa4b4).
    prop.value
}

// 0x65eb24 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_65eb24(prop: &mut SurfaceEnumProp, value: i32) -> bool {
    // IDA 0x65eb24: `setEnumValue` find-and-set (same shape as 0x4aa4bc).
    if prop.enum_desc.items.iter().any(|it| it.value == value) {
        prop.value = value;
        true
    } else {
        false
    }
}

// 0x65eb98 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_65eb98(prop: &SurfaceEnumProp) -> Option<crate::enum_desc::EnumItem> {
    // IDA 0x65eb98: `getEnumItem` via `convertToItem` (same shape as
    // 0x4aa508).
    usize::try_from(prop.value)
        .ok()
        .and_then(|slot| prop.enum_desc.items_by_value.get(slot).copied().flatten())
        .and_then(|idx| prop.enum_desc.items.get(idx).cloned())
}

// 0x65ebe8 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_65ebe8(prop: &mut SurfaceEnumProp, name: &str) -> bool {
    // IDA 0x65ebe8: `setStringValue` (`Name` overload, same shape as
    // 0x4aa528).
    match prop.enum_desc.lookup_value(name) {
        Some(v) => {
            prop.value = v;
            true
        }
        None => false,
    }
}

// 0x65ec4c — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE4ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE8getValueEPKNS_10Reflection13DescribedBaseE
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)4,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_65ec4c(prop: &SurfaceProp<i32>) -> i32 {
    // IDA 0x65ec4c: `SurfaceGetSet<4, SurfaceType>::getValue`: invoke
    // `getter(Bottom)` — face hardcoded (0x65ec66).
    prop.value
}

// 0x65ec6c — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE4ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE8setValueEPNS_10Reflection13DescribedBaseERKS2_
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)4,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::setValue(RBX::Reflection::DescribedBase *,RBX::SurfaceType const&)const")]
pub fn stub_65ec6c(prop: &mut SurfaceProp<i32>, value: i32) {
    // IDA 0x65ec6c: `SurfaceGetSet<4, SurfaceType>::setValue`: invoke
    // `setter(Bottom, value)` — face hardcoded (0x65ec88).
    prop.value = value;
}

// 0x65ec90 — __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE1EfEC2IMNS_12PartInstanceEKFfS1_EMS4_FvS1_fEEEPKcSA_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityENS_8Security11PermissionsE
#[doc(alias = "RBX::SurfacePropDescriptor<(RBX::NormalId)1,float>::SurfacePropDescriptor<float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>(char const*,char const*,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float),RBX::Reflection::PropertyDescriptor::Functionality,RBX::Security::Permissions)")]
pub fn stub_65ec90(
    name: &str,
    category: &str,
    initial: f32,
    attributes: u32,
    permissions: u32,
) -> SurfaceProp<f32> {
    // IDA 0x65ec90: `SurfacePropDescriptor<1, float>` ctor: `new` the
    // `SurfaceGetSet<1>` holding the per-face getter/setter pair, forward
    // into the typed-descriptor ctor (same shape as 0x5f0cec).
    SurfaceProp::new(name, category, 1, initial, attributes, permissions)
}

// 0x65edd8 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE1EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE8getValueEPKNS_10Reflection13DescribedBaseE
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)1,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_65edd8(prop: &SurfaceProp<f32>) -> f32 {
    // IDA 0x65edd8: `SurfaceGetSet<1, float>::getValue`: header strip,
    // getter decode, invoke `getter(Top)` — face hardcoded (0x65edf2).
    prop.value
}

// 0x65edf8 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE1EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE8setValueEPNS_10Reflection13DescribedBaseERKf
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)1,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
pub fn stub_65edf8(prop: &mut SurfaceProp<f32>, value: f32) {
    // IDA 0x65edf8: `SurfaceGetSet<1, float>::setValue`: header strip,
    // setter decode, invoke `setter(Top, value)` — face hardcoded (0x65ee14).
    prop.value = value;
}

// 0x65ee1c — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEEC2IMNS_12PartInstanceEKFS3_S1_EMS6_FvS1_S3_EEEPKcSC_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::SurfaceEnumPropDescriptor<RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>(char const*,char const*,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType),RBX::Reflection::PropertyDescriptor::Functionality)")]
pub fn stub_65ee1c(
    name: &str,
    category: &str,
    initial: i32,
    attributes: u32,
    permissions: u32,
) -> SurfaceEnumProp {
    // IDA 0x65ee1c: `SurfaceEnumPropDescriptor<1, InputType>` ctor: `new`
    // the `SurfaceGetSet<1>`, link the `EnumDesc<InputType>` singleton
    // (same shape as 0x65e484).
    SurfaceEnumProp::new(name, category, 1, initial, SURFACE_INPUT_DESC.clone(), attributes, permissions)
}

// 0x65ef14 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES8_
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_65ef14(a: &SurfaceEnumProp, b: &SurfaceEnumProp) -> bool {
    // IDA 0x65ef14: `equalValues` for (1, InputType).
    a.value == b.value
}

// 0x65ef3c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS5_7VariantE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_65ef3c(prop: &SurfaceEnumProp) -> Value {
    // IDA 0x65ef3c: `getVariant` for (1, InputType).
    Value::Int(prop.value)
}

// 0x65ef64 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS5_7VariantE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_65ef64(prop: &mut SurfaceEnumProp, value: &Value) {
    // IDA 0x65ef64: `setVariant` for (1, InputType).
    prop.value = match value {
        Value::Int(v) => *v,
        Value::EnumValue(v) => *v,
        other => panic!("Variant::convert<InputType> on {other:?} (IDA 0x65ef64)"),
    };
}

// 0x65f0bc — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS6_
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_65f0bc(dst: &mut SurfaceEnumProp, src: &SurfaceEnumProp) {
    // IDA 0x65f0bc: `copyValue` for (1, InputType).
    dst.value = src.value;
}

// 0x65f0e4 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_65f0e4(prop: &SurfaceEnumProp) -> String {
    // IDA 0x65f0e4: `getStringValue` for (1, InputType).
    prop.enum_desc.lookup_name(prop.value).unwrap_or_default().to_owned()
}

// 0x65f134 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_65f134(prop: &mut SurfaceEnumProp, name: &str) -> bool {
    // IDA 0x65f134: `setStringValue` for (1, InputType).
    match prop.enum_desc.lookup_value(name) {
        Some(v) => {
            prop.value = v;
            true
        }
        None => false,
    }
}

// 0x65f198 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_65f198(prop: &SurfaceEnumProp) -> i32 {
    // IDA 0x65f198: `writeValue` for (1, InputType).
    prop.value
}

// 0x65f1b8 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_65f1b8(prop: &mut SurfaceEnumProp, text: &str) -> bool {
    // IDA 0x65f1b8: `readValue` for (1, InputType).
    match prop.enum_desc.lookup_value(text) {
        Some(v) => {
            prop.value = v;
            true
        }
        None => false,
    }
}

// 0x65f410 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_65f410(prop: &SurfaceEnumProp) -> i32 {
    // IDA 0x65f410: `getIndexValue` for (1, InputType).
    prop.convert_to_index(prop.value)
}

// 0x65f458 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_65f458(prop: &mut SurfaceEnumProp, index: usize) -> bool {
    // IDA 0x65f458: `setIndexValue` for (1, InputType).
    match prop.enum_desc.values.get(index) {
        Some(&v) => {
            prop.value = v;
            true
        }
        None => false,
    }
}

// 0x65f4b4 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_65f4b4(prop: &SurfaceEnumProp) -> i32 {
    // IDA 0x65f4b4: `getEnumValue` for (1, InputType).
    prop.value
}

// 0x65f4bc — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_65f4bc(prop: &mut SurfaceEnumProp, value: i32) -> bool {
    // IDA 0x65f4bc: `setEnumValue` for (1, InputType).
    if prop.enum_desc.items.iter().any(|it| it.value == value) {
        prop.value = value;
        true
    } else {
        false
    }
}

// 0x65f530 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_65f530(prop: &SurfaceEnumProp) -> Option<crate::enum_desc::EnumItem> {
    // IDA 0x65f530: `getEnumItem` for (1, InputType).
    usize::try_from(prop.value)
        .ok()
        .and_then(|slot| prop.enum_desc.items_by_value.get(slot).copied().flatten())
        .and_then(|idx| prop.enum_desc.items.get(idx).cloned())
}

// 0x65f580 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_65f580(prop: &mut SurfaceEnumProp, name: &str) -> bool {
    // IDA 0x65f580: `setStringValue` (`Name` overload) for (1, InputType).
    match prop.enum_desc.lookup_value(name) {
        Some(v) => {
            prop.value = v;
            true
        }
        None => false,
    }
}

// 0x65f5e4 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE1ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE8getValueEPKNS_10Reflection13DescribedBaseE
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)1,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_65f5e4(prop: &SurfaceProp<i32>) -> i32 {
    // IDA 0x65f5e4: `SurfaceGetSet<1, InputType>::getValue`: invoke
    // `getter(Top)` — face hardcoded.
    prop.value
}

// 0x65f604 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE1ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE8setValueEPNS_10Reflection13DescribedBaseERKS3_
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)1,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::setValue(RBX::Reflection::DescribedBase *,RBX::LegacyController::InputType const&)const")]
pub fn stub_65f604(prop: &mut SurfaceProp<i32>, value: i32) {
    // IDA 0x65f604: `SurfaceGetSet<1, InputType>::setValue`: invoke
    // `setter(Top, value)` — face hardcoded.
    prop.value = value;
}

// 0x65f628 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEEC2IMNS_12PartInstanceEKFS2_S1_EMS5_FvS1_S2_EEEPKcSB_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::SurfaceEnumPropDescriptor<RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>(char const*,char const*,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType),RBX::Reflection::PropertyDescriptor::Functionality)")]
pub fn stub_65f628(
    name: &str,
    category: &str,
    initial: i32,
    attributes: u32,
    permissions: u32,
) -> SurfaceEnumProp {
    // IDA 0x65f628: `SurfaceEnumPropDescriptor<1, SurfaceType>` ctor: `new`
    // the `SurfaceGetSet<1>`, link the `EnumDesc<SurfaceType>` singleton
    // (same shape as 0x65e484).
    SurfaceEnumProp::new(name, category, 1, initial, SURFACE_TYPE_DESC.clone(), attributes, permissions)
}

// 0x65f720 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES7_
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_65f720(a: &SurfaceEnumProp, b: &SurfaceEnumProp) -> bool {
    // IDA 0x65f720: `equalValues` for (1, SurfaceType).
    a.value == b.value
}

// 0x65f748 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS4_7VariantE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_65f748(prop: &SurfaceEnumProp) -> Value {
    // IDA 0x65f748: `getVariant` for (1, SurfaceType).
    Value::Int(prop.value)
}

// 0x65f770 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS4_7VariantE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_65f770(prop: &mut SurfaceEnumProp, value: &Value) {
    // IDA 0x65f770: `setVariant` for (1, SurfaceType).
    prop.value = match value {
        Value::Int(v) => *v,
        Value::EnumValue(v) => *v,
        other => panic!("Variant::convert<SurfaceType> on {other:?} (IDA 0x65f770)"),
    };
}

// 0x65f8c8 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS5_
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_65f8c8(dst: &mut SurfaceEnumProp, src: &SurfaceEnumProp) {
    // IDA 0x65f8c8: `copyValue` for (1, SurfaceType).
    dst.value = src.value;
}

// 0x65f8f0 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_65f8f0(prop: &SurfaceEnumProp) -> String {
    // IDA 0x65f8f0: `getStringValue` for (1, SurfaceType).
    prop.enum_desc.lookup_name(prop.value).unwrap_or_default().to_owned()
}

// 0x65f940 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_65f940(prop: &mut SurfaceEnumProp, name: &str) -> bool {
    // IDA 0x65f940: `setStringValue` for (1, SurfaceType).
    match prop.enum_desc.lookup_value(name) {
        Some(v) => {
            prop.value = v;
            true
        }
        None => false,
    }
}

// 0x65f9a4 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_65f9a4(prop: &SurfaceEnumProp) -> i32 {
    // IDA 0x65f9a4: `writeValue` for (1, SurfaceType).
    prop.value
}

// 0x65f9c4 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_65f9c4(prop: &mut SurfaceEnumProp, text: &str) -> bool {
    // IDA 0x65f9c4: `readValue` for (1, SurfaceType).
    match prop.enum_desc.lookup_value(text) {
        Some(v) => {
            prop.value = v;
            true
        }
        None => false,
    }
}

// 0x65fc1c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_65fc1c(prop: &SurfaceEnumProp) -> i32 {
    // IDA 0x65fc1c: `getIndexValue` for (1, SurfaceType).
    prop.convert_to_index(prop.value)
}

// 0x65fc64 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_65fc64(prop: &mut SurfaceEnumProp, index: usize) -> bool {
    // IDA 0x65fc64: `setIndexValue` for (1, SurfaceType).
    match prop.enum_desc.values.get(index) {
        Some(&v) => {
            prop.value = v;
            true
        }
        None => false,
    }
}

// 0x65fcc0 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_65fcc0(prop: &SurfaceEnumProp) -> i32 {
    // IDA 0x65fcc0: `getEnumValue` for (1, SurfaceType).
    prop.value
}

// 0x65fcc8 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_65fcc8(prop: &mut SurfaceEnumProp, value: i32) -> bool {
    // IDA 0x65fcc8: `setEnumValue` for (1, SurfaceType).
    if prop.enum_desc.items.iter().any(|it| it.value == value) {
        prop.value = value;
        true
    } else {
        false
    }
}

// 0x65fd3c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_65fd3c(prop: &SurfaceEnumProp) -> Option<crate::enum_desc::EnumItem> {
    // IDA 0x65fd3c: `getEnumItem` for (1, SurfaceType).
    usize::try_from(prop.value)
        .ok()
        .and_then(|slot| prop.enum_desc.items_by_value.get(slot).copied().flatten())
        .and_then(|idx| prop.enum_desc.items.get(idx).cloned())
}

// 0x65fd8c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_65fd8c(prop: &mut SurfaceEnumProp, name: &str) -> bool {
    // IDA 0x65fd8c: `setStringValue` (`Name` overload) for (1, SurfaceType).
    match prop.enum_desc.lookup_value(name) {
        Some(v) => {
            prop.value = v;
            true
        }
        None => false,
    }
}

// 0x65fdf0 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE1ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE8getValueEPKNS_10Reflection13DescribedBaseE
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)1,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_65fdf0(prop: &SurfaceProp<i32>) -> i32 {
    // IDA 0x65fdf0: `SurfaceGetSet<1, SurfaceType>::getValue`: invoke
    // `getter(Top)` — face hardcoded.
    prop.value
}

// 0x65fe10 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE1ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE8setValueEPNS_10Reflection13DescribedBaseERKS2_
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)1,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::setValue(RBX::Reflection::DescribedBase *,RBX::SurfaceType const&)const")]
pub fn stub_65fe10(prop: &mut SurfaceProp<i32>, value: i32) {
    // IDA 0x65fe10: `SurfaceGetSet<1, SurfaceType>::setValue`: invoke
    // `setter(Top, value)` — face hardcoded.
    prop.value = value;
}

// 0x65fee4 — __ZN3RBX10Reflection4TypeC2INS_7SurfaceEEEPKcS5_PT_
#[doc(alias = "RBX::Reflection::Type::Type<RBX::Surface>(char const*,char const*,RBX::Surface *)")]
pub fn stub_65fee4(name: &str, category: &str) -> ClassType {
    // IDA 0x65fee4: `Type::Type<Surface>`: `Descriptor` init, vtable
    // install, `typeinfo for'Surface` (0x65ff1c), `Name::declare` the tag
    // (0x65ff24-0x65ff2e), `ReleaseAssert(!tag.empty())` (Type.h:77), then
    // `addToAllTypes` (0x65ff80, same shape as 0x62ffac).
    assert!(!name.is_empty(), "!this->tag.empty() include/reflection/Type.h:77 (IDA 0x65fee4)");
    register_type(name);
    ClassType { name: name.to_owned(), category: category.to_owned() }
}

// 0x65ff90 — __ZN3RBX10Reflection5TTypeINS_7SurfaceEED0Ev
#[doc(alias = "RBX::Reflection::TType<RBX::Surface>::~TType()")]
pub fn stub_65ff90() {
    // IDA 0x65ff90: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x660be8 — __ZN3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::~EnumPropDescriptor()")]
pub fn stub_660be8() {
    // IDA 0x660be8: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x661740 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_16SurfaceSelectionES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::SurfaceSelection,RBX::SurfaceSelection>(rbx_core::SharedPtr<RBX::SurfaceSelection> const*,RBX::SurfaceSelection *)const")]
pub fn stub_661740() {
    // IDA 0x661740: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0x662440 — __ZN3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::EnumPropDescriptor<RBX::NormalId (RBX::SurfaceSelection::*)(void)const,void (RBX::SurfaceSelection::*)(RBX::NormalId)>(char const*,char const*,RBX::NormalId (RBX::SurfaceSelection::*)(void)const,void (RBX::SurfaceSelection::*)(RBX::NormalId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_662440(
    name: &str,
    category: &str,
    initial: i32,
    attributes: u32,
    permissions: u32,
) -> EnumProp {
    // IDA 0x662440: `EnumPropDescriptor<SurfaceSelection, NormalId>` ctor
    // (same shape as 0x4a9de0/0x5f9d30). Links the grounded `NormalId`
    // singleton (`normal_id_enum_desc`, pairs at 0x6f2a52+).
    EnumProp::new(name, category, initial, crate::descriptor::normal_id_enum_desc().clone(), attributes, permissions)
}

// 0x6625f4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::~EnumPropDescriptor()")]
pub fn stub_6625f4() {
    // IDA 0x6625f4: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x662620 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::isReadOnly(void)const")]
pub fn stub_662620() {
    // IDA 0x662620: EnumPropDescriptor::isReadOnly/isWriteOnly -- forwards through the bound member descriptor at +44 (`(**(this+44))(this+44)`, decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x662630 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::isWriteOnly(void)const")]
pub fn stub_662630() {
    // IDA 0x662630: EnumPropDescriptor::isReadOnly/isWriteOnly -- forwards through the bound member descriptor at +44 (`(**(this+44))(this+44)`, decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x662640 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_662640(a: &EnumProp, b: &EnumProp) -> bool {
    // IDA 0x662640: `equalValues` (same shape as 0x4a9fe0).
    a.value == b.value
}

// 0x662668 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_662668(prop: &EnumProp) -> Value {
    // IDA 0x662668: `getVariant` (same shape as 0x4aa008).
    Value::Int(prop.value)
}

// 0x66268c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_66268c(prop: &mut EnumProp, value: &Value) {
    // IDA 0x66268c: `setVariant` (same shape as 0x4aa02c).
    prop.value = match value {
        Value::Int(v) => *v,
        Value::EnumValue(v) => *v,
        Value::Float(v) => *v as i32,
        Value::Bool(v) => *v as i32,
        other => panic!("Variant::convert<int> on {other:?} (IDA 0x66268c)"),
    };
}

// 0x6627d8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_6627d8(dst: &mut EnumProp, src: &EnumProp) {
    // IDA 0x6627d8: `copyValue` (same shape as 0x4aa178).
    dst.value = src.value;
}

// 0x6627fc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::hasStringValue(void)const")]
pub fn stub_6627fc() -> bool {
    // IDA 0x6627fc: EnumPropDescriptor::hasStringValue -- hardcoded `return 1` (decompiled 0x10244/0x10dc8/0x11650).
    true
}

// 0x662800 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_662800(prop: &EnumProp) -> String {
    // IDA 0x662800: `getStringValue` (same shape as 0x4aa1a0).
    prop.enum_desc.lookup_name(prop.value).unwrap_or_default().to_owned()
}

// 0x662824 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_662824(prop: &mut EnumProp, name: &str) -> bool {
    // IDA 0x662824: `setStringValue` (same shape as 0x4aa1c4).
    match prop.enum_desc.lookup_value(name) {
        Some(v) => {
            prop.value = v;
            true
        }
        None => false,
    }
}

// 0x662864 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_662864(prop: &EnumProp) -> i32 {
    // IDA 0x662864: `writeValue` (same shape as 0x4aa204).
    prop.value
}

// 0x662884 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_662884(prop: &mut EnumProp, text: &str) -> bool {
    // IDA 0x662884: `readValue` (same shape as 0x4aa224).
    match prop.enum_desc.lookup_value(text) {
        Some(v) => {
            prop.value = v;
            true
        }
        None => false,
    }
}

// 0x662ac4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_662ac4(prop: &EnumProp) -> i32 {
    // IDA 0x662ac4: `getIndexValue` (same shape as 0x4aa464).
    prop.convert_to_index(prop.value)
}

// 0x662ae0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_662ae0(prop: &mut EnumProp, index: usize) -> bool {
    // IDA 0x662ae0: `setIndexValue` (same shape as 0x4aa480).
    match prop.enum_desc.values.get(index) {
        Some(&v) => {
            prop.value = v;
            true
        }
        None => false,
    }
}

// 0x662b14 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_662b14(prop: &EnumProp) -> i32 {
    // IDA 0x662b14: `getEnumValue` (same shape as 0x4aa4b4).
    prop.value
}

// 0x662b1c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_662b1c(prop: &mut EnumProp, value: i32) -> bool {
    // IDA 0x662b1c: `setEnumValue` (same shape as 0x4aa4bc).
    if prop.enum_desc.items.iter().any(|it| it.value == value) {
        prop.value = value;
        true
    } else {
        false
    }
}

// 0x662b68 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_662b68(prop: &EnumProp) -> Option<crate::enum_desc::EnumItem> {
    // IDA 0x662b68: `getEnumItem` (same shape as 0x4aa508).
    usize::try_from(prop.value)
        .ok()
        .and_then(|slot| prop.enum_desc.items_by_value.get(slot).copied().flatten())
        .and_then(|idx| prop.enum_desc.items.get(idx).cloned())
}

// 0x662b88 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_662b88(prop: &mut EnumProp, name: &str) -> bool {
    // IDA 0x662b88: `setStringValue` (`Name` overload, same shape as
    // 0x4aa528).
    match prop.enum_desc.lookup_value(name) {
        Some(v) => {
            prop.value = v;
            true
        }
        None => false,
    }
}

// 0x662bbc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_662bbc(prop: &mut EnumProp, value: i32) -> bool {
    // IDA 0x662bbc: `setIntValue` (same shape as 0x4aa55c).
    match usize::try_from(value)
        .ok()
        .and_then(|slot| prop.enum_desc.value_to_value.get(slot).copied())
    {
        Some(mapped) if mapped != -1 => {
            prop.value = mapped;
            true
        }
        _ => false,
    }
}

// 0x662bfc — __ZNK3RBX10Reflection14PropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::GetSetImpl<RBX::NormalId (RBX::SurfaceSelection::*)(void)const,void (RBX::SurfaceSelection::*)(RBX::NormalId)>::isReadOnly(void)const")]
pub fn stub_662bfc() -> bool {
    // IDA 0x662bfc: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0x662c00 — __ZNK3RBX10Reflection14PropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::GetSetImpl<RBX::NormalId (RBX::SurfaceSelection::*)(void)const,void (RBX::SurfaceSelection::*)(RBX::NormalId)>::isWriteOnly(void)const")]
pub fn stub_662c00() -> bool {
    // IDA 0x662c00: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0x662c04 — __ZNK3RBX10Reflection14PropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::GetSetImpl<RBX::NormalId (RBX::SurfaceSelection::*)(void)const,void (RBX::SurfaceSelection::*)(RBX::NormalId)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_662c04(prop: &EnumProp) -> i32 {
    // IDA 0x662c04: `GetSetImpl<NormalId>::getValue` for SurfaceSelection
    // (same shape as 0x4aa5a4).
    prop.value
}

// 0x662c24 — __ZNK3RBX10Reflection14PropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::GetSetImpl<RBX::NormalId (RBX::SurfaceSelection::*)(void)const,void (RBX::SurfaceSelection::*)(RBX::NormalId)>::setValue(RBX::Reflection::DescribedBase *,RBX::NormalId const&)const")]
pub fn stub_662c24(prop: &mut EnumProp, value: i32) {
    // IDA 0x662c24: `GetSetImpl<NormalId>::setValue` for SurfaceSelection
    // (same shape as 0x4aa5c4).
    prop.value = value;
}

// 0x6632e0 — __ZN3RBX10Reflection14PropDescriptorINS_4TeamEiED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,int>::~PropDescriptor()")]
pub fn stub_6632e0() {
    // IDA 0x6632e0: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x663304 — __ZN3RBX10Reflection14PropDescriptorINS_4TeamENS_10BrickColorEED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,RBX::BrickColor>::~PropDescriptor()")]
pub fn stub_663304() {
    // IDA 0x663304: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x663328 — __ZN3RBX10Reflection14PropDescriptorINS_4TeamEbED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,bool>::~PropDescriptor()")]
pub fn stub_663328() {
    // IDA 0x663328: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

/// `RBX::BrickColor` cutover: 4-byte payload (the 0x664040 getter returns it
/// in a register, 0x664064 — not a multi-lane struct like `Vector3`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BrickColor(pub u32);

/// `RBX::Reflection::BoundProp<bool, Mutable>` cutover for `Team`
/// (IDA 0x663b74): unlike `GetSetImpl` (member-function pair), the
/// `BoundPropGetSet` reads/writes a bound field byte directly
/// (0x663d14/0x663d34). The field offset folds into storage; the change
/// callback + `raisePropertyChanged` fold into `on_change`/`changed`.
#[derive(Clone, Default)]
pub struct BoundBoolProp {
    pub name: String,
    pub category: String,
    pub attributes: u32,
    pub permissions: u32,
    pub value: bool,
    pub on_change: Option<SharedPtr<dyn Fn(bool) + Send + Sync>>,
    pub changed: bool,
}

// 0x663b74 — __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_4TeamEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Team>(char const*,char const*,bool RBX::Team::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_663b74(
    name: &str,
    category: &str,
    initial: bool,
    on_change: Option<SharedPtr<dyn Fn(bool) + Send + Sync>>,
    attributes: u32,
    permissions: u32,
) -> BoundBoolProp {
    // IDA 0x663b74: `BoundProp<bool, Mutable>::BoundProp<Team>` ctor:
    // class-descriptor fetch, `new` the `BoundPropGetSet` holding the field
    // offset + change member, forward into the typed-descriptor ctor. The
    // field offset folds into direct storage; the change member folds into
    // `on_change`.
    BoundBoolProp {
        name: name.to_owned(),
        category: category.to_owned(),
        attributes,
        permissions,
        value: initial,
        on_change,
        changed: false,
    }
}

// 0x663d04 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_4TeamEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Team>::isReadOnly(void)const")]
pub fn stub_663d04() -> bool {
    // IDA 0x663d04: BoundPropGetSet::isReadOnly -- hardcoded `return 0` (decompiled 0x659d38/0x659d3c SurfaceGetSet, 0x6ba528/0x6ba52c BoundPropGetSet).
    false
}

// 0x663d08 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_4TeamEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Team>::isWriteOnly(void)const")]
pub fn stub_663d08() -> bool {
    // IDA 0x663d08: BoundPropGetSet::isWriteOnly -- hardcoded `return 0` (decompiled 0x659d38/0x659d3c SurfaceGetSet, 0x6ba528/0x6ba52c BoundPropGetSet).
    false
}

// 0x663d0c — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_4TeamEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Team>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_663d0c(prop: &BoundBoolProp) -> bool {
    // IDA 0x663d0c: `BoundPropGetSet::getValue`: direct bound-field byte
    // read at `*(this + 8) + obj - 36` (0x663d14) — no member-function hop,
    // unlike `GetSetImpl`.
    prop.value
}

// 0x663d18 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_4TeamEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Team>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
pub fn stub_663d18(prop: &mut BoundBoolProp, value: bool) {
    // IDA 0x663d18: `BoundPropGetSet::setValue`: header strip (`a2 - 36`,
    // 0x663d22); on change store the byte (0x663d34), run the bound change
    // member when present (0x663d36-0x663d54), then
    // `Instance::raisePropertyChanged` (collapsed into `changed`).
    if prop.value != value {
        prop.value = value;
        if let Some(cb) = prop.on_change.clone() {
            cb(value);
        }
        prop.changed = true;
    }
}

// 0x663d68 — __ZN3RBX10Reflection14PropDescriptorINS_4TeamEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,bool>::PropDescriptor<bool (RBX::Team::*)(void)const,void (RBX::Team::*)(bool)>(char const*,char const*,bool (RBX::Team::*)(void)const,void (RBX::Team::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_663d68(
    name: &str,
    category: &str,
    initial: bool,
    attributes: u32,
    permissions: u32,
) -> Prop<bool> {
    // IDA 0x663d68: `PropDescriptor<Team, bool>` get/set ctor: `new` the
    // GetSetImpl, forward into the `TypedPropertyDescriptor` ctor (same
    // shape as 0x5f0cec).
    Prop::new(name, category, initial, attributes, permissions)
}

// 0x663e7c — __ZN3RBX10Reflection14PropDescriptorINS_4TeamEbED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,bool>::~PropDescriptor()")]
pub fn stub_663e7c() {
    // IDA 0x663e7c: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x663ea8 — __ZNK3RBX10Reflection14PropDescriptorINS_4TeamEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,bool>::GetSetImpl<bool (RBX::Team::*)(void)const,void (RBX::Team::*)(bool)>::isReadOnly(void)const")]
pub fn stub_663ea8() -> bool {
    // IDA 0x663ea8: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0x663eac — __ZNK3RBX10Reflection14PropDescriptorINS_4TeamEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,bool>::GetSetImpl<bool (RBX::Team::*)(void)const,void (RBX::Team::*)(bool)>::isWriteOnly(void)const")]
pub fn stub_663eac() -> bool {
    // IDA 0x663eac: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0x663eb0 — __ZNK3RBX10Reflection14PropDescriptorINS_4TeamEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,bool>::GetSetImpl<bool (RBX::Team::*)(void)const,void (RBX::Team::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_663eb0(prop: &Prop<bool>) -> bool {
    // IDA 0x663eb0: `GetSetImpl<bool>::getValue` for Team: header strip,
    // getter member-pointer decode, invoke.
    prop.value
}

// 0x663ed4 — __ZNK3RBX10Reflection14PropDescriptorINS_4TeamEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,bool>::GetSetImpl<bool (RBX::Team::*)(void)const,void (RBX::Team::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
pub fn stub_663ed4(prop: &mut Prop<bool>, value: bool) {
    // IDA 0x663ed4: `GetSetImpl<bool>::setValue` for Team: header strip,
    // setter member-pointer decode, invoke.
    prop.value = value;
}

// 0x663ef8 — __ZN3RBX10Reflection14PropDescriptorINS_4TeamENS_10BrickColorEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,RBX::BrickColor>::PropDescriptor<RBX::BrickColor (RBX::Team::*)(void)const,void (RBX::Team::*)(RBX::BrickColor)>(char const*,char const*,RBX::BrickColor (RBX::Team::*)(void)const,void (RBX::Team::*)(RBX::BrickColor),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_663ef8(
    name: &str,
    category: &str,
    initial: BrickColor,
    attributes: u32,
    permissions: u32,
) -> Prop<BrickColor> {
    // IDA 0x663ef8: `PropDescriptor<Team, BrickColor>` get/set ctor: `new`
    // the GetSetImpl, forward into the `TypedPropertyDescriptor` ctor
    // (same shape as 0x5f0cec).
    Prop::new(name, category, initial, attributes, permissions)
}

// 0x66400c — __ZN3RBX10Reflection14PropDescriptorINS_4TeamENS_10BrickColorEED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,RBX::BrickColor>::~PropDescriptor()")]
pub fn stub_66400c() {
    // IDA 0x66400c: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x664038 — __ZNK3RBX10Reflection14PropDescriptorINS_4TeamENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::Team::*)(void)const,void (RBX::Team::*)(RBX::BrickColor)>::isReadOnly(void)const")]
pub fn stub_664038() -> bool {
    // IDA 0x664038: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0x66403c — __ZNK3RBX10Reflection14PropDescriptorINS_4TeamENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::Team::*)(void)const,void (RBX::Team::*)(RBX::BrickColor)>::isWriteOnly(void)const")]
pub fn stub_66403c() -> bool {
    // IDA 0x66403c: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0x664040 — __ZNK3RBX10Reflection14PropDescriptorINS_4TeamENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::Team::*)(void)const,void (RBX::Team::*)(RBX::BrickColor)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_664040(prop: &Prop<BrickColor>) -> BrickColor {
    // IDA 0x664040: `GetSetImpl<BrickColor>::getValue` for Team: header
    // strip (`a3 - 36`, 0x664048), member-pointer decode
    // (`offset >> 1`, virtual bit, 0x664058-0x664060), invoke (0x664064).
    prop.value
}

// 0x664068 — __ZNK3RBX10Reflection14PropDescriptorINS_4TeamENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::Team::*)(void)const,void (RBX::Team::*)(RBX::BrickColor)>::setValue(RBX::Reflection::DescribedBase *,RBX::BrickColor const&)const")]
pub fn stub_664068(prop: &mut Prop<BrickColor>, value: BrickColor) {
    // IDA 0x664068: `GetSetImpl<BrickColor>::setValue` for Team: header
    // strip, setter member-pointer decode, invoke.
    prop.value = value;
}

// 0x66408c — __ZN3RBX10Reflection14PropDescriptorINS_4TeamEiEC2IMS2_KFivEMS2_FviEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,int>::PropDescriptor<int (RBX::Team::*)(void)const,void (RBX::Team::*)(int)>(char const*,char const*,int (RBX::Team::*)(void)const,void (RBX::Team::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_66408c(
    name: &str,
    category: &str,
    initial: i32,
    attributes: u32,
    permissions: u32,
) -> Prop<i32> {
    // IDA 0x66408c: `PropDescriptor<Team, int>` get/set ctor: `new` the
    // GetSetImpl, forward into the `TypedPropertyDescriptor` ctor (same
    // shape as 0x5f0cec).
    Prop::new(name, category, initial, attributes, permissions)
}

// 0x6641a0 — __ZN3RBX10Reflection14PropDescriptorINS_4TeamEiED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,int>::~PropDescriptor()")]
pub fn stub_6641a0() {
    // IDA 0x6641a0: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x6641cc — __ZNK3RBX10Reflection14PropDescriptorINS_4TeamEiE10GetSetImplIMS2_KFivEMS2_FviEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,int>::GetSetImpl<int (RBX::Team::*)(void)const,void (RBX::Team::*)(int)>::isReadOnly(void)const")]
pub fn stub_6641cc() -> bool {
    // IDA 0x6641cc: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0x6641d0 — __ZNK3RBX10Reflection14PropDescriptorINS_4TeamEiE10GetSetImplIMS2_KFivEMS2_FviEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,int>::GetSetImpl<int (RBX::Team::*)(void)const,void (RBX::Team::*)(int)>::isWriteOnly(void)const")]
pub fn stub_6641d0() -> bool {
    // IDA 0x6641d0: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0x6641d4 — __ZNK3RBX10Reflection14PropDescriptorINS_4TeamEiE10GetSetImplIMS2_KFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,int>::GetSetImpl<int (RBX::Team::*)(void)const,void (RBX::Team::*)(int)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_6641d4(prop: &Prop<i32>) -> i32 {
    // IDA 0x6641d4: `GetSetImpl<int>::getValue` for Team: header strip,
    // getter member-pointer decode, invoke.
    prop.value
}

// 0x6641f4 — __ZNK3RBX10Reflection14PropDescriptorINS_4TeamEiE10GetSetImplIMS2_KFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,int>::GetSetImpl<int (RBX::Team::*)(void)const,void (RBX::Team::*)(int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
pub fn stub_6641f4(prop: &mut Prop<i32>, value: i32) {
    // IDA 0x6641f4: `GetSetImpl<int>::setValue` for Team: header strip,
    // setter member-pointer decode, invoke.
    prop.value = value;
}

// 0x665008 — __ZN3RBX10Reflection13BoundFuncDescINS_5TeamsEFvvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Teams,void ()(void),0>::~BoundFuncDesc()")]
pub fn stub_665008() {
    // IDA 0x665008: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}
// 0x665040 — __ZN3RBX10Reflection13BoundFuncDescINS_5TeamsEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Teams,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()")]
pub fn stub_0x665040() {
    // IDA 0x665040: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x6654bc — __ZN3RBX10Reflection13BoundFuncDescINS_5TeamsEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EEC2EMS2_FSB_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Teams,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Teams::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x6654bc() -> ! {
    todo!("0x6654bc RBX::Reflection::BoundFuncDesc<RBX::Teams,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::BoundFuncDesc(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::Teams::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x6655c0 — __ZN3RBX10Reflection13BoundFuncDescINS_5TeamsEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Teams,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()")]
pub fn stub_0x6655c0() {
    // IDA 0x6655c0: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x665674 — __ZNK3RBX10Reflection13BoundFuncDescINS_5TeamsEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Teams,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x665674() -> ! {
    todo!("0x665674 RBX::Reflection::BoundFuncDesc<RBX::Teams,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x665698 — __ZN3RBX10Reflection11Call0HelperINS_5TeamsEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvESB_E4callEPS2_SD_RNS0_7VariantE
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::Teams,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Teams::*)(void),rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::call(RBX::Teams*,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Teams::*)(void),RBX::Reflection::Variant &)")]
pub fn stub_0x665698() -> ! {
    todo!("0x665698 RBX::Reflection::Call0Helper<RBX::Teams,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::Teams::*)(void),boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>>::call(RBX::Teams*,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::Teams::*)(void),RBX::Reflection::Variant &)")
}

// 0x665780 — __ZN3RBX10Reflection13BoundFuncDescINS_5TeamsEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Teams,void ()(void),0>::BoundFuncDesc(void (RBX::Teams::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x665780() -> ! {
    todo!("0x665780 RBX::Reflection::BoundFuncDesc<RBX::Teams,void ()(void),0>::BoundFuncDesc(void (RBX::Teams::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x665884 — __ZN3RBX10Reflection13BoundFuncDescINS_5TeamsEFvvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Teams,void ()(void),0>::~BoundFuncDesc()")]
pub fn stub_0x665884() {
    // IDA 0x665884: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x665938 — __ZNK3RBX10Reflection13BoundFuncDescINS_5TeamsEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Teams,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x665938() -> ! {
    todo!("0x665938 RBX::Reflection::BoundFuncDesc<RBX::Teams,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x667558 — __ZN3RBX7TextBox17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
#[doc(alias = "RBX::TextBox::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_0x667558() -> ! {
    todo!("0x667558 RBX::TextBox::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")
}

// 0x66857c — __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEbED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::~PropDescriptor()")]
pub fn stub_0x66857c() {
    // IDA 0x66857c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x6685a8 — __ZN3RBX10Reflection13BoundFuncDescINS_7TextBoxEFvvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TextBox,void ()(void),0>::~BoundFuncDesc()")]
pub fn stub_0x6685a8() {
    // IDA 0x6685a8: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x6685cc — __ZN3RBX10Reflection9EventDescINS_7TextBoxEFvbEN3rbx6signalIS3_EEMS2_S6_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::TextBox,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::TextBox::*>::~EventDesc()")]
pub fn stub_0x6685cc() {
    // IDA 0x6685cc: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x6685fc — __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxESsED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,std::string>::~PropDescriptor()")]
pub fn stub_0x6685fc() {
    // IDA 0x6685fc: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x668624 — __ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::~EnumPropDescriptor()")]
pub fn stub_0x668624() {
    // IDA 0x668624: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x66864c — __ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::~EnumPropDescriptor()")]
pub fn stub_0x66864c() {
    // IDA 0x66864c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x66868c — __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxENS_10BrickColorEED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::BrickColor>::~PropDescriptor()")]
pub fn stub_0x66868c() {
    // IDA 0x66868c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x6686c0 — __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D6Color3EED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Color3>::~PropDescriptor()")]
pub fn stub_0x6686c0() {
    // IDA 0x6686c0: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x6686e8 — __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEfED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,float>::~PropDescriptor()")]
pub fn stub_0x6686e8() {
    // IDA 0x6686e8: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x668720 — __ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::~EnumPropDescriptor()")]
pub fn stub_0x668720() {
    // IDA 0x668720: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x668748 — __ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::~EnumPropDescriptor()")]
pub fn stub_0x668748() {
    // IDA 0x668748: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x66876c — __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D7Vector2EED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Vector2>::~PropDescriptor()")]
pub fn stub_0x66876c() {
    // IDA 0x66876c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x66939c — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7TextBoxES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TextBox,RBX::TextBox>(rbx_core::SharedPtr<RBX::TextBox> const*,RBX::TextBox *)const")]
pub fn stub_0x66939c() {
    // IDA 0x66939c: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0x66c194 — __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEbEC2IMS2_KFbvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::PropDescriptor<bool (RBX::TextBox::*)(void)const,int>(char const*,char const*,bool (RBX::TextBox::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x66c194() -> ! {
    todo!("0x66c194 RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::PropDescriptor<bool (RBX::TextBox::*)(void)const,int>(char const*,char const*,bool (RBX::TextBox::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x66c2a0 — __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEbED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::~PropDescriptor()")]
pub fn stub_0x66c2a0() {
    // IDA 0x66c2a0: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x66c2cc — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE7GetImplIMS2_KFbvEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetImpl<bool (RBX::TextBox::*)(void)const>::isReadOnly(void)const")]
pub fn stub_0x66c2cc() {
    // IDA 0x66c2cc: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x66c2d0 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE7GetImplIMS2_KFbvEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetImpl<bool (RBX::TextBox::*)(void)const>::isWriteOnly(void)const")]
pub fn stub_0x66c2d0() {
    // IDA 0x66c2d0: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x66c2d4 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE7GetImplIMS2_KFbvEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetImpl<bool (RBX::TextBox::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x66c2d4() -> ! {
    todo!("0x66c2d4 RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetImpl<bool (RBX::TextBox::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x66c2f8 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE7GetImplIMS2_KFbvEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetImpl<bool (RBX::TextBox::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
pub fn stub_0x66c2f8() -> ! {
    todo!("0x66c2f8 RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetImpl<bool (RBX::TextBox::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")
}

// 0x66c418 — __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D7Vector2EEC2IMS2_KFS4_vEiEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Vector2>::PropDescriptor<G3D::Vector2 (RBX::TextBox::*)(void)const,int>(char const*,char const*,G3D::Vector2 (RBX::TextBox::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x66c418() -> ! {
    todo!("0x66c418 RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Vector2>::PropDescriptor<G3D::Vector2 (RBX::TextBox::*)(void)const,int>(char const*,char const*,G3D::Vector2 (RBX::TextBox::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x66c524 — __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D7Vector2EED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Vector2>::~PropDescriptor()")]
pub fn stub_0x66c524() {
    // IDA 0x66c524: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x66c550 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector2EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::isReadOnly(void)const")]
pub fn stub_0x66c550() {
    // IDA 0x66c550: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x66c560 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector2EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::isWriteOnly(void)const")]
pub fn stub_0x66c560() {
    // IDA 0x66c560: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x66c570 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector2EE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x66c570() -> ! {
    todo!("0x66c570 RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

// 0x66c5c0 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector2EE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x66c5c0() -> ! {
    todo!("0x66c5c0 RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}

// 0x66c5ec — __ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector2EE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x66c5ec() -> ! {
    todo!("0x66c5ec RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}

// 0x66c614 — __ZN3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector2EED1Ev
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::~TypedPropertyDescriptor()")]
pub fn stub_0x66c614() {
    // IDA 0x66c614: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x66c638 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D7Vector2EE7GetImplIMS2_KFS4_vEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Vector2>::GetImpl<G3D::Vector2 (RBX::TextBox::*)(void)const>::isReadOnly(void)const")]
pub fn stub_0x66c638() {
    // IDA 0x66c638: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x66c63c — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D7Vector2EE7GetImplIMS2_KFS4_vEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Vector2>::GetImpl<G3D::Vector2 (RBX::TextBox::*)(void)const>::isWriteOnly(void)const")]
pub fn stub_0x66c63c() {
    // IDA 0x66c63c: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x66c640 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D7Vector2EE7GetImplIMS2_KFS4_vEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Vector2>::GetImpl<G3D::Vector2 (RBX::TextBox::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x66c640() -> ! {
    todo!("0x66c640 RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Vector2>::GetImpl<G3D::Vector2 (RBX::TextBox::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x66c668 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEN3G3D7Vector2EE7GetImplIMS2_KFS4_vEE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Vector2>::GetImpl<G3D::Vector2 (RBX::TextBox::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector2 const&)const")]
pub fn stub_0x66c668() -> ! {
    todo!("0x66c668 RBX::Reflection::PropDescriptor<RBX::TextBox,G3D::Vector2>::GetImpl<G3D::Vector2 (RBX::TextBox::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector2 const&)const")
}

// 0x66c788 — __ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::EnumPropDescriptor<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::YAlignment)>(char const*,char const*,RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::YAlignment),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x66c788() -> ! {
    todo!("0x66c788 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::EnumPropDescriptor<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::YAlignment)>(char const*,char const*,RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::YAlignment),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x66c93c — __ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::~EnumPropDescriptor()")]
pub fn stub_0x66c93c() {
    // IDA 0x66c93c: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x66c968 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::isReadOnly(void)const")]
pub fn stub_0x66c968() {
    // IDA 0x66c968: EnumPropDescriptor::isReadOnly/isWriteOnly -- forwards through the bound member descriptor at +44 (`(**(this+44))(this+44)`, decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x66c978 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::isWriteOnly(void)const")]
pub fn stub_0x66c978() {
    // IDA 0x66c978: EnumPropDescriptor::isReadOnly/isWriteOnly -- forwards through the bound member descriptor at +44 (`(**(this+44))(this+44)`, decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x66c988 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x66c988() -> ! {
    todo!("0x66c988 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

// 0x66c9b0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x66c9b0() -> ! {
    todo!("0x66c9b0 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}

// 0x66c9d4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x66c9d4() -> ! {
    todo!("0x66c9d4 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}

// 0x66cb20 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE9copyValueEPKNS0_13DescribedBaseEPS6_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x66cb20() -> ! {
    todo!("0x66cb20 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}

// 0x66cb44 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::hasStringValue(void)const")]
pub fn stub_0x66cb44() -> bool {
    // IDA 0x66cb44: EnumPropDescriptor::hasStringValue -- hardcoded `return 1` (decompiled 0x10244/0x10dc8/0x11650).
    true
}

// 0x66cb48 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x66cb48() -> ! {
    todo!("0x66cb48 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::getStringValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x66cb6c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_0x66cb6c() -> ! {
    todo!("0x66cb6c RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x66cbac — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_0x66cbac() -> ! {
    todo!("0x66cbac RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

// 0x66cbcc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x66cbcc() -> ! {
    todo!("0x66cbcc RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

// 0x66ce0c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x66ce0c() -> ! {
    todo!("0x66ce0c RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::getIndexValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x66ce28 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_0x66ce28() -> ! {
    todo!("0x66ce28 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")
}

// 0x66ce5c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x66ce5c() -> ! {
    todo!("0x66ce5c RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::getEnumValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x66ce64 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x66ce64() -> ! {
    todo!("0x66ce64 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x66ceb0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x66ceb0() -> ! {
    todo!("0x66ceb0 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::getEnumItem(RBX::Reflection::DescribedBase const*)const")
}

// 0x66ced0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_0x66ced0() -> ! {
    todo!("0x66ced0 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")
}

// 0x66cf04 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::convertToValue(RBX::Name const&,RBX::TextService::YAlignment&)const")]
pub fn stub_0x66cf04(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0x66cf04: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0x66cf80 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::convertToItem(RBX::TextService::YAlignment const&)const")]
pub fn stub_0x66cf80(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0x66cf80: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0x66d04c — __ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::convertToIndex(RBX::TextService::YAlignment)const")]
pub fn stub_0x66d04c(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0x66d04c: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0x66d0bc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x66d0bc() -> ! {
    todo!("0x66d0bc RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x66d0fc — __ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::convertToString(RBX::TextService::YAlignment const&)const")]
pub fn stub_0x66d0fc(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0x66d0fc: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0x66d29c — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::GetSetImpl<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::YAlignment)>::isReadOnly(void)const")]
pub fn stub_0x66d29c() -> bool {
    // IDA 0x66d29c: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0x66d2a0 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::GetSetImpl<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::YAlignment)>::isWriteOnly(void)const")]
pub fn stub_0x66d2a0() -> bool {
    // IDA 0x66d2a0: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0x66d2a4 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::GetSetImpl<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::YAlignment)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x66d2a4() -> ! {
    todo!("0x66d2a4 RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::GetSetImpl<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::YAlignment)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x66d2d0 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::GetSetImpl<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::YAlignment)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextService::YAlignment const&)const")]
pub fn stub_0x66d2d0() -> ! {
    todo!("0x66d2d0 RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::GetSetImpl<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::YAlignment)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextService::YAlignment const&)const")
}

// 0x66d2f4 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService10YAlignmentEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TextService::YAlignment> const>::initSingleton(void)")]
pub fn stub_0x66d2f4() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0x66d2f4: Singleton<EnumDesc<T>>::initSingleton -- thunk to doGetSingleton (decompiled 0x4a60b8). Rust: forward to the singleton.
    crate::generated_shard_a::stub_0x66d2f8()
}

// 0x66d2f8 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService10YAlignmentEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TextService::YAlignment> const>::doGetSingleton(void)")]
pub fn stub_0x66d2f8() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0x66d2f8: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated_shard_aa::stub_0x7d8720)
}

// 0x66d3e8 — __ZN3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::~EnumDesc()")]
pub fn stub_0x66d3e8() {
    // IDA 0x66d3e8: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x66d3ec — __ZN3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::~EnumDesc()")]
pub fn stub_0x66d3ec() {
    // IDA 0x66d3ec: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x66d5c0 — __ZN3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::~EnumDesc()")]
pub fn stub_0x66d5c0() {
    // IDA 0x66d5c0: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x66d660 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::lookup(char const*)const")]
pub fn stub_0x66d660(desc: &crate::enum_desc::EnumDesc, name: &str) -> usize {
    // IDA 0x66d660: EnumDesc<T>::lookup(char const*) -- Name::lookup intern, search name_to_value then legacy_names; hit: return convertToItem(value); miss: return 0 (decompiled 0x957a18).
    desc.lookup_value(name).and_then(|v| usize::try_from(v).ok()).and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0x66d690 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_0x66d690(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0x66d690: EnumDesc<T>::lookup(Variant) -- rbx::any_cast<T> the payload, then convertToItem (decompiled 0xb97c). Variant is unmodeled in this crate; the caller passes the already-cast enum value, and this is convertToItem exactly.
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0x66d6b0 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_0x66d6b0() {
    // IDA 0x66d6b0: EnumDesc<T>::convertToValue(index, Variant&) -- writes the converted value into a Variant out-param; Variant is unmodeled in this crate: cutover no-op. See the (desc, name, &mut i32) sibling for the lookup semantics.
}

// 0x66d6e4 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::convertToString(unsigned long,std::string &)const")]
pub fn stub_0x66d6e4(desc: &crate::enum_desc::EnumDesc, index: usize, out: &mut String) -> bool {
    // IDA 0x66d6e4: EnumDesc<T>::convertToString(index, string&) -- if index < items.size(): out = items[index].name, return true; else return false, out untouched (decompiled 0x957bd4).
    if let Some(item) = desc.items.get(index) {
        *out = item.name.clone();
        true
    } else {
        false
    }
}

// 0x66da0c — __ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::EnumPropDescriptor<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::XAlignment)>(char const*,char const*,RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::XAlignment),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x66da0c() -> ! {
    todo!("0x66da0c RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::EnumPropDescriptor<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::XAlignment)>(char const*,char const*,RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::XAlignment),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x66dbc0 — __ZN3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::~EnumPropDescriptor()")]
pub fn stub_0x66dbc0() {
    // IDA 0x66dbc0: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x66dbec — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::isReadOnly(void)const")]
pub fn stub_0x66dbec() {
    // IDA 0x66dbec: EnumPropDescriptor::isReadOnly/isWriteOnly -- forwards through the bound member descriptor at +44 (`(**(this+44))(this+44)`, decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x66dbfc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::isWriteOnly(void)const")]
pub fn stub_0x66dbfc() {
    // IDA 0x66dbfc: EnumPropDescriptor::isReadOnly/isWriteOnly -- forwards through the bound member descriptor at +44 (`(**(this+44))(this+44)`, decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x66dc0c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x66dc0c() -> ! {
    todo!("0x66dc0c RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

// 0x66dc34 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x66dc34() -> ! {
    todo!("0x66dc34 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}

// 0x66dc58 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x66dc58() -> ! {
    todo!("0x66dc58 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}

// 0x66dda4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE9copyValueEPKNS0_13DescribedBaseEPS6_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x66dda4() -> ! {
    todo!("0x66dda4 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}

// 0x66ddc8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::hasStringValue(void)const")]
pub fn stub_0x66ddc8() -> bool {
    // IDA 0x66ddc8: EnumPropDescriptor::hasStringValue -- hardcoded `return 1` (decompiled 0x10244/0x10dc8/0x11650).
    true
}

// 0x66ddcc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x66ddcc() -> ! {
    todo!("0x66ddcc RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::getStringValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x66ddf0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_0x66ddf0() -> ! {
    todo!("0x66ddf0 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x66de30 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_0x66de30() -> ! {
    todo!("0x66de30 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

// 0x66de50 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x66de50() -> ! {
    todo!("0x66de50 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

// 0x66e090 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x66e090() -> ! {
    todo!("0x66e090 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::getIndexValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x66e0ac — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_0x66e0ac() -> ! {
    todo!("0x66e0ac RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")
}

// 0x66e0e0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x66e0e0() -> ! {
    todo!("0x66e0e0 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::getEnumValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x66e0e8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x66e0e8() -> ! {
    todo!("0x66e0e8 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x66e134 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x66e134() -> ! {
    todo!("0x66e134 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::getEnumItem(RBX::Reflection::DescribedBase const*)const")
}

// 0x66e154 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_0x66e154() -> ! {
    todo!("0x66e154 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")
}

// 0x66e188 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::convertToValue(RBX::Name const&,RBX::TextService::XAlignment&)const")]
pub fn stub_0x66e188(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0x66e188: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0x66e204 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::convertToItem(RBX::TextService::XAlignment const&)const")]
pub fn stub_0x66e204(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0x66e204: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0x66e2d0 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::convertToIndex(RBX::TextService::XAlignment)const")]
pub fn stub_0x66e2d0(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0x66e2d0: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0x66e340 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x66e340() -> ! {
    todo!("0x66e340 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x66e380 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::convertToString(RBX::TextService::XAlignment const&)const")]
pub fn stub_0x66e380(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0x66e380: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0x66e520 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::GetSetImpl<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::XAlignment)>::isReadOnly(void)const")]
pub fn stub_0x66e520() -> bool {
    // IDA 0x66e520: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0x66e524 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::GetSetImpl<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::XAlignment)>::isWriteOnly(void)const")]
pub fn stub_0x66e524() -> bool {
    // IDA 0x66e524: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0x66e528 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::GetSetImpl<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::XAlignment)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x66e528() -> ! {
    todo!("0x66e528 RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::GetSetImpl<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::XAlignment)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x66e554 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::GetSetImpl<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::XAlignment)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextService::XAlignment const&)const")]
pub fn stub_0x66e554() -> ! {
    todo!("0x66e554 RBX::Reflection::PropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::GetSetImpl<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(RBX::TextService::XAlignment)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextService::XAlignment const&)const")
}

// 0x66e578 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService10XAlignmentEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TextService::XAlignment> const>::initSingleton(void)")]
pub fn stub_0x66e578() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0x66e578: Singleton<EnumDesc<T>>::initSingleton -- thunk to doGetSingleton (decompiled 0x4a60b8). Rust: forward to the singleton.
    crate::generated_shard_a::stub_0x66e57c()
}

// 0x66e57c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService10XAlignmentEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TextService::XAlignment> const>::doGetSingleton(void)")]
pub fn stub_0x66e57c() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0x66e57c: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated_shard_aa::stub_0x7d8544)
}

// 0x66e66c — __ZN3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::~EnumDesc()")]
pub fn stub_0x66e66c() {
    // IDA 0x66e66c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x66e670 — __ZN3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::~EnumDesc()")]
pub fn stub_0x66e670() {
    // IDA 0x66e670: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x66e844 — __ZN3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::~EnumDesc()")]
pub fn stub_0x66e844() {
    // IDA 0x66e844: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x66e8e4 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::lookup(char const*)const")]
pub fn stub_0x66e8e4(desc: &crate::enum_desc::EnumDesc, name: &str) -> usize {
    // IDA 0x66e8e4: EnumDesc<T>::lookup(char const*) -- Name::lookup intern, search name_to_value then legacy_names; hit: return convertToItem(value); miss: return 0 (decompiled 0x957a18).
    desc.lookup_value(name).and_then(|v| usize::try_from(v).ok()).and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0x66e914 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_0x66e914(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0x66e914: EnumDesc<T>::lookup(Variant) -- rbx::any_cast<T> the payload, then convertToItem (decompiled 0xb97c). Variant is unmodeled in this crate; the caller passes the already-cast enum value, and this is convertToItem exactly.
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0x66e934 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_0x66e934() {
    // IDA 0x66e934: EnumDesc<T>::convertToValue(index, Variant&) -- writes the converted value into a Variant out-param; Variant is unmodeled in this crate: cutover no-op. See the (desc, name, &mut i32) sibling for the lookup semantics.
}

// 0x66e968 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::convertToString(unsigned long,std::string &)const")]
pub fn stub_0x66e968(desc: &crate::enum_desc::EnumDesc, index: usize, out: &mut String) -> bool {
    // IDA 0x66e968: EnumDesc<T>::convertToString(index, string&) -- if index < items.size(): out = items[index].name, return true; else return false, out untouched (decompiled 0x957bd4).
    if let Some(item) = desc.items.get(index) {
        *out = item.name.clone();
        true
    } else {
        false
    }
}

// 0x66ec90 — __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEbEC2IMNS_12GuiTextMixinEKFbvEMS2_FvbEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::PropDescriptor<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(bool)>(char const*,char const*,bool (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x66ec90() -> ! {
    todo!("0x66ec90 RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::PropDescriptor<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(bool)>(char const*,char const*,bool (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x66eda4 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMNS_12GuiTextMixinEKFbvEMS2_FvbEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetSetImpl<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(bool)>::isReadOnly(void)const")]
pub fn stub_0x66eda4() -> bool {
    // IDA 0x66eda4: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0x66eda8 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMNS_12GuiTextMixinEKFbvEMS2_FvbEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetSetImpl<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(bool)>::isWriteOnly(void)const")]
pub fn stub_0x66eda8() -> bool {
    // IDA 0x66eda8: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0x66edac — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMNS_12GuiTextMixinEKFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetSetImpl<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x66edac() -> ! {
    todo!("0x66edac RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetSetImpl<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x66ede0 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMNS_12GuiTextMixinEKFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetSetImpl<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
pub fn stub_0x66ede0() -> ! {
    todo!("0x66ede0 RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetSetImpl<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")
}

// 0x66ee04 — __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEfEC2IMNS_12GuiTextMixinEKFfvEMS2_FvfEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,float>::PropDescriptor<float (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(float)>(char const*,char const*,float (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x66ee04() -> ! {
    todo!("0x66ee04 RBX::Reflection::PropDescriptor<RBX::TextBox,float>::PropDescriptor<float (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(float)>(char const*,char const*,float (RBX::GuiTextMixin::*)(void)const,void (RBX::TextBox::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}
// 0xa3e960 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11StarterGearES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::StarterGear,RBX::StarterGear>(rbx_core::SharedPtr<RBX::StarterGear> const*,RBX::StarterGear *)const")]
pub fn stub_0xa3e960() {
    // IDA 0xa3e960: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xa3ec64 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Network6PlayerES7_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Network::Player,RBX::Network::Player>(rbx_core::SharedPtr<RBX::Network::Player> const*,RBX::Network::Player *)const")]
pub fn stub_0xa3ec64() {
    // IDA 0xa3ec64: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xa3fc30 — __ZNK3RBX10Reflection7Variant3getIbEET_v
#[doc(alias = "bool RBX::Reflection::Variant::get<bool>(void)const")]
pub fn stub_0xa3fc30() -> ! {
    todo!("0xa3fc30 bool RBX::Reflection::Variant::get<bool>(void)const")
}

// 0xa40330 — __ZN3RBX10Reflection9ArgHelper6getArgINS_7Network7Players10ChatOptionELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS6_EEPNSA_10disable_ifINSA_7is_sameIS6_NSA_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "RBX::Network::Players::ChatOption RBX::Reflection::ArgHelper::getArg<RBX::Network::Players::ChatOption,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Network::Players::ChatOption> const&,boost::disable_if<boost::is_same<RBX::Network::Players::ChatOption,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
pub fn stub_0xa40330() -> ! {
    todo!("0xa40330 RBX::Network::Players::ChatOption RBX::Reflection::ArgHelper::getArg<RBX::Network::Players::ChatOption,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Network::Players::ChatOption> const&,boost::disable_if<boost::is_same<RBX::Network::Players::ChatOption,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0xa413f8 — __ZN3RBX10Reflection18GenericSlotWrapper8execute1IN5boost10shared_ptrINS_8InstanceEEEEEvRKT_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute1<rbx_core::SharedPtr<RBX::Instance>>(rbx_core::SharedPtr<RBX::Instance> const&)")]
pub fn stub_0xa413f8() -> ! {
    todo!("0xa413f8 void RBX::Reflection::GenericSlotWrapper::execute1<boost::shared_ptr<RBX::Instance>>(boost::shared_ptr<RBX::Instance> const&)")
}

// 0xa41a90 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueINSA_IS9_EEEENS_3argILi1EEEEEEEvSC_E6invokeERNS1_15function_bufferESC_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,void,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0xa41a90() -> ! {
    todo!("0xa41a90 boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,void,boost::shared_ptr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>)")
}

// 0xa41ab0 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS4_10Reflection18GenericSlotWrapperERKS6_EENS9_5list2INS9_5valueINS3_ISE_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0xa41ab0() -> ! {
    todo!("0xa41ab0 bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0xa41d98 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueINSA_IS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0xa41d98() -> ! {
    todo!("0xa41d98 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0xa43684 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS1_8InstanceEEES8_RKNS1_13FriendService15FriendEventTypeENS4_IS3_EENS_3argILi1EEENSE_ILi2EEENSE_ILi3EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf3ISK_T0_T1_T2_T3_EENSI_9list_av_4IT4_T5_T6_T7_E4typeEEEMSN_FSK_SO_SP_SQ_EST_SU_SV_SW_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&>,boost::_bi::list_av_4<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>(void (RBX::Reflection::GenericSlotWrapper::*)(rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
pub fn stub_0xa43684() -> ! {
    todo!("0xa43684 boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&>,boost::_bi::list_av_4<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>(void (RBX::Reflection::GenericSlotWrapper::*)(boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")
}

// 0xa43ce8 — __ZN3RBX10Reflection18GenericSlotWrapper8execute3IN5boost10shared_ptrINS_8InstanceEEES6_NS_13FriendService15FriendEventTypeEEEvRKT_RKT0_RKT1_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute3<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType>(rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&)")]
pub fn stub_0xa43ce8() -> ! {
    todo!("0xa43ce8 void RBX::Reflection::GenericSlotWrapper::execute3<boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType>(boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&)")
}

// 0xa43f98 — __ZN5boost3_bi8storage4INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEC2ES8_SA_SB_SC_
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::storage4(boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
pub fn stub_0xa43f98() -> ! {
    todo!("0xa43f98 boost::_bi::storage4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::storage4(boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")
}

// 0xa44854 — __ZN5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEES4_NS2_13FriendService15FriendEventTypeEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKS4_SG_RKS6_EENS9_5list4INS9_5valueINS1_ISE_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEEvT_
#[doc(alias = "void boost::function3<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)")]
pub fn stub_0xa44854() -> ! {
    todo!("0xa44854 void boost::function3<void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)")
}

// 0xa44ccc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEESE_RKNS7_13FriendService15FriendEventTypeEEENS3_5list4INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEE6manageERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0xa44ccc() -> ! {
    todo!("0xa44ccc boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0xa44cf0 — __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEESE_RKNS7_13FriendService15FriendEventTypeEEENS3_5list4INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEvSC_SC_SG_E6invokeERNS1_15function_bufferESC_SC_SG_
#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)")]
pub fn stub_0xa44cf0() -> ! {
    todo!("0xa44cf0 boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)")
}

// 0xa44d1c — __ZNK5boost6detail8function13basic_vtable3IvNS_10shared_ptrIN3RBX8InstanceEEES6_NS4_13FriendService15FriendEventTypeEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKS6_SI_RKS8_EENSB_5list4INSB_5valueINS3_ISG_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0xa44d1c() -> ! {
    todo!("0xa44d1c bool boost::detail::function::basic_vtable3<void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0xa45004 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEESE_RKNS7_13FriendService15FriendEventTypeEEENS3_5list4INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEE7managerERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0xa45004() -> ! {
    todo!("0xa45004 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendEventType const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0xa48e90 — __ZN3RBX10Reflection18GenericSlotWrapper8execute1ISsEEvRKT_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute1<std::string>(std::string const&)")]
pub fn stub_0xa48e90() -> ! {
    todo!("0xa48e90 void RBX::Reflection::GenericSlotWrapper::execute1<std::string>(std::string const&)")
}

// 0xa490f0 — __ZN5boost9function1IvSsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKSsEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEEvT_
#[doc(alias = "void boost::function1<void,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,std::string const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,std::string const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)")]
pub fn stub_0xa490f0() -> ! {
    todo!("0xa490f0 void boost::function1<void,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,std::string const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,std::string const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)")
}

// 0xa49568 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKSsEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,std::string const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0xa49568() -> ! {
    todo!("0xa49568 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,std::string const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0xa4b0f4 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS1_7Network7Players14PlayerChatTypeERKNS_10shared_ptrINS1_8InstanceEEERKSsSD_NS9_IS3_EENS_3argILi1EEENSH_ILi2EEENSH_ILi3EEENSH_ILi4EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf4ISO_T0_T1_T2_T3_T4_EENSM_9list_av_5IT5_T6_T7_T8_T9_E4typeEEEMSR_FSO_SS_ST_SU_SV_ESY_SZ_S10_S11_S12_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list_av_5<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)")]
pub fn stub_0xa4b0f4() -> ! {
    todo!("0xa4b0f4 boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,boost::shared_ptr<RBX::Instance> const&,std::string const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list_av_5<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,boost::shared_ptr<RBX::Instance> const&,std::string const&,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::Network::Players::PlayerChatType const&,boost::shared_ptr<RBX::Instance> const&,std::string const&,boost::shared_ptr<RBX::Instance> const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)")
}

// 0xa4b560 — __ZN3RBX10Reflection18GenericSlotWrapper8execute4INS_7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS_8InstanceEEESsS9_EEvRKT_RKT0_RKT1_RKT2_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute4<RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>(RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&)")]
pub fn stub_0xa4b560() -> ! {
    todo!("0xa4b560 void RBX::Reflection::GenericSlotWrapper::execute4<RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>>(RBX::Network::Players::PlayerChatType const&,boost::shared_ptr<RBX::Instance> const&,std::string const&,boost::shared_ptr<RBX::Instance> const&)")
}

// 0xa4bd28 — __ZN5boost9function4IvN3RBX7Network7Players14PlayerChatTypeENS_10shared_ptrINS1_8InstanceEEESsS7_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvNS1_10Reflection18GenericSlotWrapperERKS4_RKS7_RKSsSJ_EENSA_5list5INSA_5valueINS5_ISF_EEEENS_3argILi1EEENSR_ILi2EEENSR_ILi3EEENSR_ILi4EEEEEEEEEvT_
#[doc(alias = "void boost::function4<void,RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>)")]
pub fn stub_0xa4bd28() -> ! {
    todo!("0xa4bd28 void boost::function4<void,RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,boost::shared_ptr<RBX::Instance> const&,std::string const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,boost::shared_ptr<RBX::Instance> const&,std::string const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>)")
}

// 0xa4c1a0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKNS7_7Network7Players14PlayerChatTypeERKNS_10shared_ptrINS7_8InstanceEEERKSsSJ_EENS3_5list5INS3_5valueINSF_IS9_EEEENS_3argILi1EEENSR_ILi2EEENSR_ILi3EEENSR_ILi4EEEEEEEE6manageERKNS1_15function_bufferERSZ_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0xa4c1a0() -> ! {
    todo!("0xa4c1a0 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,boost::shared_ptr<RBX::Instance> const&,std::string const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0xa4c1c4 — __ZN5boost6detail8function26void_function_obj_invoker4INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKNS7_7Network7Players14PlayerChatTypeERKNS_10shared_ptrINS7_8InstanceEEERKSsSJ_EENS3_5list5INS3_5valueINSF_IS9_EEEENS_3argILi1EEENSR_ILi2EEENSR_ILi3EEENSR_ILi4EEEEEEEvSC_SH_SsSH_E6invokeERNS1_15function_bufferESC_SH_SsSH_
#[doc(alias = "boost::detail::function::void_function_obj_invoker4<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,void,RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0xa4c1c4() -> ! {
    todo!("0xa4c1c4 boost::detail::function::void_function_obj_invoker4<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,boost::shared_ptr<RBX::Instance> const&,std::string const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,void,RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)")
}

// 0xa4c1f8 — __ZNK5boost6detail8function13basic_vtable4IvN3RBX7Network7Players14PlayerChatTypeENS_10shared_ptrINS3_8InstanceEEESsS9_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvNS3_10Reflection18GenericSlotWrapperERKS6_RKS9_RKSsSL_EENSC_5list5INSC_5valueINS7_ISH_EEEENS_3argILi1EEENST_ILi2EEENST_ILi3EEENST_ILi4EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable4<void,RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0xa4c1f8() -> ! {
    todo!("0xa4c1f8 bool boost::detail::function::basic_vtable4<void,RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,boost::shared_ptr<RBX::Instance> const&,std::string const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,boost::shared_ptr<RBX::Instance> const&,std::string const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0xa4c4e0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKNS7_7Network7Players14PlayerChatTypeERKNS_10shared_ptrINS7_8InstanceEEERKSsSJ_EENS3_5list5INS3_5valueINSF_IS9_EEEENS_3argILi1EEENSR_ILi2EEENSR_ILi3EEENSR_ILi4EEEEEEEE7managerERKNS1_15function_bufferERSZ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0xa4c4e0() -> ! {
    todo!("0xa4c4e0 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,RBX::Network::Players::PlayerChatType const&,boost::shared_ptr<RBX::Instance> const&,std::string const&,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0xa50340 — __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_7Network7PlayersENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
#[doc(alias = "non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::Network::Players,RBX::Instance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
pub fn stub_0xa50340() {
    // IDA 0xa50340: non-virtual thunk to `RBX::Reflection::RefPropDescriptor<RBX::Network::Players,RBX::Instance>::assignIDREF( int a1, int a2, _DWORD *a3) { RBX::Reflecti` — this/arg-adjust + tail-call (arg a1 -= 40) (decompiled). Rust uses static dispatch; no thunk needed. Target unmodeled: cutover no-op.
}

// 0xa50350 — __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIN3RBX10Reflection13DescribedBaseEEEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<rbx_core::SharedPtr<RBX::Reflection::DescribedBase>>::construct_func(char const*,char *)")]
pub fn stub_0xa50350() -> ! {
    todo!("0xa50350 rbx::implementation::typed_holder<boost::shared_ptr<RBX::Reflection::DescribedBase>>::construct_func(char const*,char *)")
}

// 0xa87e84 — __ZN3RBX7Network6Player20LoadDataResultHelperEN5boost8weak_ptrIS1_EENS2_10shared_ptrIKSt3mapISsNS_10Reflection7VariantESt4lessISsESaISt4pairIKSsS8_EEEEE
#[doc(alias = "RBX::Network::Player::LoadDataResultHelper(Weak<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)")]
pub fn stub_0xa87e84() -> ! {
    todo!("0xa87e84 RBX::Network::Player::LoadDataResultHelper(boost::weak_ptr<RBX::Network::Player>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)")
}

// 0xa88274 — __ZN3RBX7Network6Player14loadDataResultEN5boost10shared_ptrIKSt3mapISsNS_10Reflection7VariantESt4lessISsESaISt4pairIKSsS6_EEEEE
#[doc(alias = "RBX::Network::Player::loadDataResult(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)")]
pub fn stub_0xa88274() -> ! {
    todo!("0xa88274 RBX::Network::Player::loadDataResult(boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)")
}

// 0xa934b0 — __ZN3RBX7Network6Player16getFriendsOnlineEiN5boost8functionIFvNS2_10shared_ptrIKNS2_9unordered13unordered_mapISsNS_10Reflection7VariantENS2_4hashISsEESt8equal_toISsESaISt4pairIKSsS8_EEEEEEEEENS3_IFvSsEEE
#[doc(alias = "RBX::Network::Player::getFriendsOnline(int,boost::function<void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>)")]
pub fn stub_0xa934b0() -> ! {
    todo!("0xa934b0 RBX::Network::Player::getFriendsOnline(int,boost::function<void ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>)")
}

// 0xa981c8 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX7Network6PlayerEEENS_10shared_ptrIKSt3mapISsNS2_10Reflection7VariantESt4lessISsESaISt4pairIKSsS9_EEEEES5_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSN_T0_T1_ENSL_9list_av_2IT2_T3_E4typeEEESR_ST_SU_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(Weak<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list_av_2<Weak<RBX::Network::Player>,boost::arg<1>>::type> boost::bind<void,Weak<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,Weak<RBX::Network::Player>,boost::arg<1>>(void (*)(Weak<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),Weak<RBX::Network::Player>,boost::arg<1>)")]
pub fn stub_0xa981c8() -> ! {
    todo!("0xa981c8 boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list_av_2<boost::weak_ptr<RBX::Network::Player>,boost::arg<1>>::type> boost::bind<void,boost::weak_ptr<RBX::Network::Player>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,boost::weak_ptr<RBX::Network::Player>,boost::arg<1>>(void (*)(boost::weak_ptr<RBX::Network::Player>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::weak_ptr<RBX::Network::Player>,boost::arg<1>)")
}

// 0xa9b5e4 — __ZN3RBX10Reflection7Variant14genericConvertINS_7Network6Player14MembershipTypeEEERT_v
#[doc(alias = "RBX::Network::Player::MembershipType & RBX::Reflection::Variant::genericConvert<RBX::Network::Player::MembershipType>(void)")]
pub fn stub_0xa9b5e4() -> ! {
    todo!("0xa9b5e4 RBX::Network::Player::MembershipType & RBX::Reflection::Variant::genericConvert<RBX::Network::Player::MembershipType>(void)")
}

// 0xaa1b6c — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11PlayerMouseES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::PlayerMouse,RBX::PlayerMouse>(rbx_core::SharedPtr<RBX::PlayerMouse> const*,RBX::PlayerMouse *)const")]
pub fn stub_0xaa1b6c() {
    // IDA 0xaa1b6c: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xaa2a04 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_8BackpackES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Backpack,RBX::Backpack>(rbx_core::SharedPtr<RBX::Backpack> const*,RBX::Backpack *)const")]
pub fn stub_0xaa2a04() {
    // IDA 0xaa2a04: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xaa4bf0 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvPFvRNSA_8weak_ptrINS2_7Network6PlayerEEEPKNS2_15ServiceProviderEENSB_5list2INSB_5valueISG_EENSO_ISK_EEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(Weak<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>>::~callable_slot()")]
pub fn stub_0xaa4bf0() {
    // IDA 0xaa4bf0: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0xaa4bfc — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvPFvRNSA_8weak_ptrINS2_7Network6PlayerEEEPKNS2_15ServiceProviderEENSB_5list2INSB_5valueISG_EENSO_ISK_EEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(Weak<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>>::~callable_slot()")]
pub fn stub_0xaa4bfc() {
    // IDA 0xaa4bfc: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0xaa4cb0 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvRNSB_8weak_ptrINS3_7Network6PlayerEEEPKNS3_15ServiceProviderEENSC_5list2INSC_5valueISH_EENSP_ISL_EEEEEELi1ES8_E4callES7_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(Weak<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_0xaa4cb0() -> ! {
    todo!("0xaa4cb0 rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")
}

// 0xaa4cc0 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvRNSB_8weak_ptrINS3_7Network6PlayerEEEPKNS3_15ServiceProviderEENSC_5list2INSC_5valueISH_EENSP_ISL_EEEEEELi1ES8_E4callES7_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(Weak<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_0xaa4cc0() {
    // IDA 0xaa4cc0: non-virtual thunk to `rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Pla` — this/arg-adjust + tail-call (this += 12) (decompiled). Rust uses static dispatch; no thunk needed. Target unmodeled: cutover no-op.
}

// 0xaa4cd0 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvRNSB_8weak_ptrINS3_7Network6PlayerEEEPKNS3_15ServiceProviderEENSC_5list2INSC_5valueISH_EENSP_ISL_EEEEEELi1ES8_ED2Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(Weak<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
pub fn stub_0xaa4cd0() {
    // IDA 0xaa4cd0: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0xaa4ea8 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvRNSB_8weak_ptrINS3_7Network6PlayerEEEPKNS3_15ServiceProviderEENSC_5list2INSC_5valueISH_EENSP_ISL_EEEEEELi1ES8_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(Weak<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
pub fn stub_0xaa4ea8() {
    // IDA 0xaa4ea8: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0xaa4eb4 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvRNSB_8weak_ptrINS3_7Network6PlayerEEEPKNS3_15ServiceProviderEENSC_5list2INSC_5valueISH_EENSP_ISL_EEEEEELi1ES8_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(Weak<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
pub fn stub_0xaa4eb4() {
    // IDA 0xaa4eb4: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0xaaa378 — __ZN5boost9function1IvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_7Network6PlayerEEESE_ENSH_5list2INSH_5valueISM_EENS_3argILi1EEEEEEEEEvT_
#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::assign_to<boost::_bi::bind_t<void,void (*)(Weak<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<Weak<RBX::Network::Player>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(Weak<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<Weak<RBX::Network::Player>>,boost::arg<1>>>)")]
pub fn stub_0xaaa378() -> ! {
    todo!("0xaaa378 void boost::function1<void,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>>>)")
}

// 0xaaa55c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX7Network6PlayerEEENS_10shared_ptrIKSt3mapISsNS6_10Reflection7VariantESt4lessISsESaISt4pairIKSsSD_EEEEEENS3_5list2INS3_5valueIS9_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSX_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(Weak<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<Weak<RBX::Network::Player>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0xaaa55c() -> ! {
    todo!("0xaaa55c boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0xaaa580 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX7Network6PlayerEEENS_10shared_ptrIKSt3mapISsNS6_10Reflection7VariantESt4lessISsESaISt4pairIKSsSD_EEEEEENS3_5list2INS3_5valueIS9_EENS_3argILi1EEEEEEEvSM_E6invokeERNS1_15function_bufferESM_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(Weak<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<Weak<RBX::Network::Player>>,boost::arg<1>>>,void,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)")]
pub fn stub_0xaaa580() -> ! {
    todo!("0xaaa580 boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>>>,void,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)")
}

// 0xaaa598 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS5_7Network6PlayerEEESG_ENSJ_5list2INSJ_5valueISO_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::assign_to<boost::_bi::bind_t<void,void (*)(Weak<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<Weak<RBX::Network::Player>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(Weak<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<Weak<RBX::Network::Player>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0xaaa598() -> ! {
    todo!("0xaaa598 bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")
}

// 0xaaa764 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS5_7Network6PlayerEEESG_ENSJ_5list2INSJ_5valueISO_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::assign_to<boost::_bi::bind_t<void,void (*)(Weak<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<Weak<RBX::Network::Player>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(Weak<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<Weak<RBX::Network::Player>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0xaaa764() -> ! {
    todo!("0xaaa764 bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0xaaa960 — __ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS_3argILi1EEEEclIPFvS7_NS_10shared_ptrIKSt3mapISsNS4_10Reflection7VariantESt4lessISsESaISt4pairIKSsSG_EEEEEENS0_5list1IRSP_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<Weak<RBX::Network::Player>>,boost::arg<1>>::operator()<void (*)(Weak<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list1<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&>>(boost::_bi::type<void>,void (*)(Weak<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>) &,boost::_bi::list1<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&> &,int)")]
pub fn stub_0xaaa960() -> ! {
    todo!("0xaaa960 void boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>>::operator()<void (*)(boost::weak_ptr<RBX::Network::Player>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list1<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::Network::Player>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>) &,boost::_bi::list1<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&> &,int)")
}

// 0xaaad08 — __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX7Network6PlayerEEENS_10shared_ptrIKSt3mapISsNS6_10Reflection7VariantESt4lessISsESaISt4pairIKSsSD_EEEEEENS3_5list2INS3_5valueIS9_EENS_3argILi1EEEEEEEE12manage_smallERKNS1_15function_bufferERSX_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager_common<boost::_bi::bind_t<void,void (*)(Weak<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<Weak<RBX::Network::Player>>,boost::arg<1>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0xaaad08() -> ! {
    todo!("0xaaad08 boost::detail::function::functor_manager_common<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0xaadb98 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_0xaadb98() -> ! {
    todo!("0xaadb98 RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")
}

// 0xaadc2c — __ZNK3RBX10Reflection14PropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE10GetSetImplIMS3_KFS5_vEMS3_FvS5_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::GetSetImpl<RBX::Camera::CameraMode (RBX::Network::Player::*)(void)const,void (RBX::Network::Player::*)(RBX::Camera::CameraMode)>::isReadOnly(void)const")]
pub fn stub_0xaadc2c() -> bool {
    // IDA 0xaadc2c: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0xaadc30 — __ZNK3RBX10Reflection14PropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE10GetSetImplIMS3_KFS5_vEMS3_FvS5_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::GetSetImpl<RBX::Camera::CameraMode (RBX::Network::Player::*)(void)const,void (RBX::Network::Player::*)(RBX::Camera::CameraMode)>::isWriteOnly(void)const")]
pub fn stub_0xaadc30() -> bool {
    // IDA 0xaadc30: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0xaadc34 — __ZNK3RBX10Reflection14PropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE10GetSetImplIMS3_KFS5_vEMS3_FvS5_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::GetSetImpl<RBX::Camera::CameraMode (RBX::Network::Player::*)(void)const,void (RBX::Network::Player::*)(RBX::Camera::CameraMode)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0xaadc34() -> ! {
    todo!("0xaadc34 RBX::Reflection::PropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::GetSetImpl<RBX::Camera::CameraMode (RBX::Network::Player::*)(void)const,void (RBX::Network::Player::*)(RBX::Camera::CameraMode)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0xaadc58 — __ZNK3RBX10Reflection14PropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE10GetSetImplIMS3_KFS5_vEMS3_FvS5_EE8setValueEPNS0_13DescribedBaseERKS5_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::GetSetImpl<RBX::Camera::CameraMode (RBX::Network::Player::*)(void)const,void (RBX::Network::Player::*)(RBX::Camera::CameraMode)>::setValue(RBX::Reflection::DescribedBase *,RBX::Camera::CameraMode const&)const")]
pub fn stub_0xaadc58() -> ! {
    todo!("0xaadc58 RBX::Reflection::PropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::GetSetImpl<RBX::Camera::CameraMode (RBX::Network::Player::*)(void)const,void (RBX::Network::Player::*)(RBX::Camera::CameraMode)>::setValue(RBX::Reflection::DescribedBase *,RBX::Camera::CameraMode const&)const")
}

// 0xaadc80 — __ZN3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvSsEN3rbx13remote_signalIS4_EEED0Ev
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::~RemoteEventDesc()")]
pub fn stub_0xaadc80() {
    // IDA 0xaadc80: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0xaadd5c — __ZNK3RBX10Reflection13EventDescImplILi1ENS_7Network6PlayerEFvSsEN3rbx13remote_signalIS4_EEMS3_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::Network::Player::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_0xaadd5c() -> ! {
    todo!("0xaadd5c RBX::Reflection::EventDescImpl<1,RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::Network::Player::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0xaae1f4 — __ZNK3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvSsEN3rbx13remote_signalIS4_EEE12isScriptableEv
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::isScriptable(void)const")]
pub fn stub_0xaae1f4() {
    // IDA 0xaae1f4: RemoteEventDesc::isScriptable -- `return *(this+48) & 1` permission-flags read (decompiled 0x39f7ac/0x39fdf8). Flags word unmodeled: cutover no-op.
}

// 0xaae1fc — __ZNK3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvSsEN3rbx13remote_signalIS4_EEE11isBroadcastEv
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::isBroadcast(void)const")]
pub fn stub_0xaae1fc() -> ! {
    todo!("0xaae1fc RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::isBroadcast(void)const")
}

// 0xaae204 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_7Network6PlayerEFvSsEN3rbx13remote_signalIS4_EEMS3_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::Network::Player::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0xaae204() -> ! {
    todo!("0xaae204 RBX::Reflection::EventDescImpl<1,RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::Network::Player::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0xaae40c — __ZNK3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvSsEN3rbx13remote_signalIS4_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0xaae40c() -> ! {
    todo!("0xaae40c RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0xaae424 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network6PlayerEFvSsEN3rbx13remote_signalIS4_EEMS3_S7_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::Network::Player::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0xaae424() -> ! {
    todo!("0xaae424 RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::Network::Player::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0xaae600 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvSsEN3rbx13remote_signalIS4_EEMS3_S7_EC2ES8_PKcSB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::Network::Player::*>::EventDesc(rbx::remote_signal<void ()(std::string)> RBX::Network::Player::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0xaae600() -> ! {
    todo!("0xaae600 RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::Network::Player::*>::EventDesc(rbx::remote_signal<void ()(std::string)> RBX::Network::Player::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xaae8b0 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvSsEN3rbx13remote_signalIS4_EEMS3_S7_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::Network::Player::*>::~EventDesc()")]
pub fn stub_0xaae8b0() {
    // IDA 0xaae8b0: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0xaae8f8 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvSsEN3rbx13remote_signalIS4_EEMS3_S7_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::Network::Player::*>::~EventDesc()")]
pub fn stub_0xaae8f8() {
    // IDA 0xaae8f8: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0xaae9d4 — __ZN3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvSsN3G3D7Vector3EEN3rbx13remote_signalIS6_EEED0Ev
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>>::~RemoteEventDesc()")]
pub fn stub_0xaae9d4() {
    // IDA 0xaae9d4: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0xaaeab0 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_7Network6PlayerEFvSsN3G3D7Vector3EEN3rbx13remote_signalIS6_EEMS3_S9_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>,rbx::remote_signal<void ()(std::string,G3D::Vector3)> RBX::Network::Player::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_0xaaeab0() -> ! {
    todo!("0xaaeab0 RBX::Reflection::EventDescImpl<2,RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>,rbx::remote_signal<void ()(std::string,G3D::Vector3)> RBX::Network::Player::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0xaaef48 — __ZNK3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvSsN3G3D7Vector3EEN3rbx13remote_signalIS6_EEE12isScriptableEv
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>>::isScriptable(void)const")]
pub fn stub_0xaaef48() {
    // IDA 0xaaef48: RemoteEventDesc::isScriptable -- `return *(this+48) & 1` permission-flags read (decompiled 0x39f7ac/0x39fdf8). Flags word unmodeled: cutover no-op.
}

// 0xaaef50 — __ZNK3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvSsN3G3D7Vector3EEN3rbx13remote_signalIS6_EEE11isBroadcastEv
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>>::isBroadcast(void)const")]
pub fn stub_0xaaef50() -> ! {
    todo!("0xaaef50 RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>>::isBroadcast(void)const")
}

// 0xaaef58 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_7Network6PlayerEFvSsN3G3D7Vector3EEN3rbx13remote_signalIS6_EEMS3_S9_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISF_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>,rbx::remote_signal<void ()(std::string,G3D::Vector3)> RBX::Network::Player::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0xaaef58() -> ! {
    todo!("0xaaef58 RBX::Reflection::EventDescImpl<2,RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>,rbx::remote_signal<void ()(std::string,G3D::Vector3)> RBX::Network::Player::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0xaaf1cc — __ZNK3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvSsN3G3D7Vector3EEN3rbx13remote_signalIS6_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISE_EE
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0xaaf1cc() -> ! {
    todo!("0xaaf1cc RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0xaaf1e4 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network6PlayerEFvSsN3G3D7Vector3EEN3rbx13remote_signalIS6_EEMS3_S9_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>,rbx::remote_signal<void ()(std::string,G3D::Vector3)> RBX::Network::Player::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0xaaf1e4() -> ! {
    todo!("0xaaf1e4 RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>,rbx::remote_signal<void ()(std::string,G3D::Vector3)> RBX::Network::Player::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0xaaf1fc — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKSsRKN3G3D7Vector3ENS_10shared_ptrIS3_EENS_3argILi1EEENSC_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISH_T0_T1_T2_EENSF_9list_av_3IT3_T4_T5_E4typeEEEMSK_FSH_SL_SM_ESP_SQ_SR_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,G3D::Vector3 const&>,boost::_bi::list_av_3<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,std::string const&,G3D::Vector3 const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(std::string const&,G3D::Vector3 const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")]
pub fn stub_0xaaf1fc() -> ! {
    todo!("0xaaf1fc boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,G3D::Vector3 const&>,boost::_bi::list_av_3<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,std::string const&,G3D::Vector3 const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(std::string const&,G3D::Vector3 const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")
}

// 0xaaf668 — __ZN3RBX10Reflection18GenericSlotWrapper8execute2ISsN3G3D7Vector3EEEvRKT_RKT0_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute2<std::string,G3D::Vector3>(std::string const&,G3D::Vector3 const&)")]
pub fn stub_0xaaf668() -> ! {
    todo!("0xaaf668 void RBX::Reflection::GenericSlotWrapper::execute2<std::string,G3D::Vector3>(std::string const&,G3D::Vector3 const&)")
}

// 0xaaf988 — __ZN5boost3_bi5list3INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS_3argILi1EEENS9_ILi2EEEEC1ES8_SA_SB_
#[doc(alias = "boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>::list3(boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>)")]
pub fn stub_0xaaf988() -> ! {
    todo!("0xaaf988 boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>::list3(boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>)")
}

// 0xab005c — __ZN5boost9function2IvSsN3G3D7Vector3EE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsRKS2_EENS5_5list3INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEvT_
#[doc(alias = "void boost::function2<void,std::string,G3D::Vector3>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,G3D::Vector3 const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,G3D::Vector3 const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")]
pub fn stub_0xab005c() -> ! {
    todo!("0xab005c void boost::function2<void,std::string,G3D::Vector3>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,G3D::Vector3 const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,G3D::Vector3 const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")
}

// 0xab04d4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsRKN3G3D7Vector3EEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,G3D::Vector3 const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0xab04d4() -> ! {
    todo!("0xab04d4 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,G3D::Vector3 const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0xab04f8 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsRKN3G3D7Vector3EEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEvSsSD_E6invokeERNS1_15function_bufferESsSD_
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,G3D::Vector3 const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,std::string,G3D::Vector3>::invoke(boost::detail::function::function_buffer &,std::string,G3D::Vector3)")]
pub fn stub_0xab04f8() -> ! {
    todo!("0xab04f8 boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,G3D::Vector3 const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,std::string,G3D::Vector3>::invoke(boost::detail::function::function_buffer &,std::string,G3D::Vector3)")
}

// 0xab0524 — __ZNK5boost6detail8function13basic_vtable2IvSsN3G3D7Vector3EE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsRKS4_EENS7_5list3INS7_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string,G3D::Vector3>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,G3D::Vector3 const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,G3D::Vector3 const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0xab0524() -> ! {
    todo!("0xab0524 bool boost::detail::function::basic_vtable2<void,std::string,G3D::Vector3>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,G3D::Vector3 const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,G3D::Vector3 const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0xab080c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsRKN3G3D7Vector3EEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,G3D::Vector3 const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0xab080c() -> ! {
    todo!("0xab080c boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,G3D::Vector3 const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0xab12d4 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvSsN3G3D7Vector3EEN3rbx13remote_signalIS6_EEMS3_S9_EC2ESA_PKcSD_SD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>,rbx::remote_signal<void ()(std::string,G3D::Vector3)> RBX::Network::Player::*>::EventDesc(rbx::remote_signal<void ()(std::string,G3D::Vector3)> RBX::Network::Player::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0xab12d4() -> ! {
    todo!("0xab12d4 RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>,rbx::remote_signal<void ()(std::string,G3D::Vector3)> RBX::Network::Player::*>::EventDesc(rbx::remote_signal<void ()(std::string,G3D::Vector3)> RBX::Network::Player::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xab1670 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvSsN3G3D7Vector3EEN3rbx13remote_signalIS6_EEMS3_S9_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>,rbx::remote_signal<void ()(std::string,G3D::Vector3)> RBX::Network::Player::*>::~EventDesc()")]
pub fn stub_0xab1670() {
    // IDA 0xab1670: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0xab16b8 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvSsN3G3D7Vector3EEN3rbx13remote_signalIS6_EEMS3_S9_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>,rbx::remote_signal<void ()(std::string,G3D::Vector3)> RBX::Network::Player::*>::~EventDesc()")]
pub fn stub_0xab16b8() {
    // IDA 0xab16b8: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0xab1794 — __ZN3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvSsSsSsEN3rbx13remote_signalIS4_EEED0Ev
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>>::~RemoteEventDesc()")]
pub fn stub_0xab1794() {
    // IDA 0xab1794: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0xab1870 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_7Network6PlayerEFvSsSsSsEN3rbx13remote_signalIS4_EEMS3_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>,rbx::remote_signal<void ()(std::string,std::string,std::string)> RBX::Network::Player::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_0xab1870() -> ! {
    todo!("0xab1870 RBX::Reflection::EventDescImpl<3,RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>,rbx::remote_signal<void ()(std::string,std::string,std::string)> RBX::Network::Player::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0xab1d08 — __ZNK3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvSsSsSsEN3rbx13remote_signalIS4_EEE12isScriptableEv
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>>::isScriptable(void)const")]
pub fn stub_0xab1d08() {
    // IDA 0xab1d08: RemoteEventDesc::isScriptable -- `return *(this+48) & 1` permission-flags read (decompiled 0x39f7ac/0x39fdf8). Flags word unmodeled: cutover no-op.
}

// 0xab1d10 — __ZNK3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvSsSsSsEN3rbx13remote_signalIS4_EEE11isBroadcastEv
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>>::isBroadcast(void)const")]
pub fn stub_0xab1d10() -> ! {
    todo!("0xab1d10 RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>>::isBroadcast(void)const")
}

// 0xab1d18 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_7Network6PlayerEFvSsSsSsEN3rbx13remote_signalIS4_EEMS3_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>,rbx::remote_signal<void ()(std::string,std::string,std::string)> RBX::Network::Player::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0xab1d18() -> ! {
    todo!("0xab1d18 RBX::Reflection::EventDescImpl<3,RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>,rbx::remote_signal<void ()(std::string,std::string,std::string)> RBX::Network::Player::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0xab2108 — __ZNK3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvSsSsSsEN3rbx13remote_signalIS4_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0xab2108() -> ! {
    todo!("0xab2108 RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0xab2120 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network6PlayerEFvSsSsSsEN3rbx13remote_signalIS4_EEMS3_S7_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>,rbx::remote_signal<void ()(std::string,std::string,std::string)> RBX::Network::Player::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0xab2120() -> ! {
    todo!("0xab2120 RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>,rbx::remote_signal<void ()(std::string,std::string,std::string)> RBX::Network::Player::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0xab2138 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKSsS5_S5_NS_10shared_ptrIS3_EENS_3argILi1EEENS8_ILi2EEENS8_ILi3EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf3ISE_T0_T1_T2_T3_EENSC_9list_av_4IT4_T5_T6_T7_E4typeEEEMSH_FSE_SI_SJ_SK_ESN_SO_SP_SQ_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,std::string const&>,boost::_bi::list_av_4<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>(void (RBX::Reflection::GenericSlotWrapper::*)(std::string const&,std::string const&,std::string const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
pub fn stub_0xab2138() -> ! {
    todo!("0xab2138 boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,std::string const&>,boost::_bi::list_av_4<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,std::string const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>(void (RBX::Reflection::GenericSlotWrapper::*)(std::string const&,std::string const&,std::string const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")
}

// 0xab25a4 — __ZN3RBX10Reflection18GenericSlotWrapper8execute3ISsSsSsEEvRKT_RKT0_RKT1_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute3<std::string,std::string,std::string>(std::string const&,std::string const&,std::string const&)")]
pub fn stub_0xab25a4() -> ! {
    todo!("0xab25a4 void RBX::Reflection::GenericSlotWrapper::execute3<std::string,std::string,std::string>(std::string const&,std::string const&,std::string const&)")
}

// 0xab2dd0 — __ZN5boost9function3IvSsSsSsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsSB_SB_EENS3_5list4INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEEEEvT_
#[doc(alias = "void boost::function3<void,std::string,std::string,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,std::string const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,std::string const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)")]
pub fn stub_0xab2dd0() -> ! {
    todo!("0xab2dd0 void boost::function3<void,std::string,std::string,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,std::string const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,std::string const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)")
}

// 0xab3248 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsSB_SB_EENS3_5list4INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEEE6manageERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,std::string const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0xab3248() -> ! {
    todo!("0xab3248 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,std::string const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0xab326c — __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsSB_SB_EENS3_5list4INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEEvSsSsSsE6invokeERNS1_15function_bufferESsSsSs
#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,std::string const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,void,std::string,std::string,std::string>::invoke(boost::detail::function::function_buffer &,std::string,std::string,std::string)")]
pub fn stub_0xab326c() -> ! {
    todo!("0xab326c boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,std::string const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,void,std::string,std::string,std::string>::invoke(boost::detail::function::function_buffer &,std::string,std::string,std::string)")
}

// 0xab3290 — __ZNK5boost6detail8function13basic_vtable3IvSsSsSsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsSD_SD_EENS5_5list4INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,std::string,std::string,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,std::string const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,std::string const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0xab3290() -> ! {
    todo!("0xab3290 bool boost::detail::function::basic_vtable3<void,std::string,std::string,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,std::string const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,std::string const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0xab3578 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsSB_SB_EENS3_5list4INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,std::string const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0xab3578() -> ! {
    todo!("0xab3578 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,std::string const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0xab412c — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvSsSsSsEN3rbx13remote_signalIS4_EEMS3_S7_EC2ES8_PKcSB_SB_SB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>,rbx::remote_signal<void ()(std::string,std::string,std::string)> RBX::Network::Player::*>::EventDesc(rbx::remote_signal<void ()(std::string,std::string,std::string)> RBX::Network::Player::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0xab412c() -> ! {
    todo!("0xab412c RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>,rbx::remote_signal<void ()(std::string,std::string,std::string)> RBX::Network::Player::*>::EventDesc(rbx::remote_signal<void ()(std::string,std::string,std::string)> RBX::Network::Player::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xab45b4 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvSsSsSsEN3rbx13remote_signalIS4_EEMS3_S7_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>,rbx::remote_signal<void ()(std::string,std::string,std::string)> RBX::Network::Player::*>::~EventDesc()")]
pub fn stub_0xab45b4() {
    // IDA 0xab45b4: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0xab45fc — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvSsSsSsEN3rbx13remote_signalIS4_EEMS3_S7_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>,rbx::remote_signal<void ()(std::string,std::string,std::string)> RBX::Network::Player::*>::~EventDesc()")]
pub fn stub_0xab45fc() {
    // IDA 0xab45fc: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0xab46d8 — __ZN3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvvEN3rbx13remote_signalIS4_EEED0Ev
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(void),rbx::remote_signal<void ()(void)>>::~RemoteEventDesc()")]
pub fn stub_0xab46d8() {
    // IDA 0xab46d8: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0xab47b4 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_7Network6PlayerEFvvEN3rbx13remote_signalIS4_EEMS3_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::Network::Player,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::Network::Player::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_0xab47b4() -> ! {
    todo!("0xab47b4 RBX::Reflection::EventDescImpl<0,RBX::Network::Player,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::Network::Player::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0xab4fd4 — __ZNK3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvvEN3rbx13remote_signalIS4_EEE12isScriptableEv
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(void),rbx::remote_signal<void ()(void)>>::isScriptable(void)const")]
pub fn stub_0xab4fd4() {
    // IDA 0xab4fd4: RemoteEventDesc::isScriptable -- `return *(this+48) & 1` permission-flags read (decompiled 0x39f7ac/0x39fdf8). Flags word unmodeled: cutover no-op.
}

// 0xab4fdc — __ZNK3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvvEN3rbx13remote_signalIS4_EEE11isBroadcastEv
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(void),rbx::remote_signal<void ()(void)>>::isBroadcast(void)const")]
pub fn stub_0xab4fdc() -> ! {
    todo!("0xab4fdc RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(void),rbx::remote_signal<void ()(void)>>::isBroadcast(void)const")
}

// 0xab4fe4 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_7Network6PlayerEFvvEN3rbx13remote_signalIS4_EEMS3_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::Network::Player,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::Network::Player::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0xab4fe4() -> ! {
    todo!("0xab4fe4 RBX::Reflection::EventDescImpl<0,RBX::Network::Player,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::Network::Player::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0xab5058 — __ZNK3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvvEN3rbx13remote_signalIS4_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(void),rbx::remote_signal<void ()(void)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0xab5058() -> ! {
    todo!("0xab5058 RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(void),rbx::remote_signal<void ()(void)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0xab5070 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network6PlayerEFvvEN3rbx13remote_signalIS4_EEMS3_S7_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::Network::Player::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0xab5070() -> ! {
    todo!("0xab5070 RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::Network::Player::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0xab524c — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEENS_13FriendService12FriendStatusEEN3rbx6signalISA_EEMS3_SD_EC2ESE_PKcSH_SH_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0xab524c() -> ! {
    todo!("0xab524c RBX::Reflection::EventDesc<RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xab55e8 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEENS_13FriendService12FriendStatusEEN3rbx6signalISA_EEMS3_SD_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::~EventDesc()")]
pub fn stub_0xab55e8() {
    // IDA 0xab55e8: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0xab56c4 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEENS_13FriendService12FriendStatusEEN3rbx6signalISA_EEMS3_SD_E14connectGenericEPNS0_11EventSourceENS5_INS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_0xab56c4() -> ! {
    todo!("0xab56c4 RBX::Reflection::EventDescImpl<2,RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0xab5b48 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEENS_13FriendService12FriendStatusEEN3rbx6signalISA_EEMS3_SD_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISJ_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0xab5b48() -> ! {
    todo!("0xab5b48 RBX::Reflection::EventDescImpl<2,RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0xab5f20 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEENS_13FriendService12FriendStatusEEN3rbx6signalISA_EEMS3_SD_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0xab5f20() -> ! {
    todo!("0xab5f20 RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0xab60e4 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEENS_13FriendService12FriendStatusEEN3rbx6signalISA_EEMS3_SD_E7connectEPNS0_11EventSourceERKNS4_8functionISA_EE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)> const&)const")]
pub fn stub_0xab60e4() -> ! {
    todo!("0xab60e4 RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)> const&)const")
}

// 0xab62b8 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS1_8InstanceEEERKNS1_13FriendService12FriendStatusENS4_IS3_EENS_3argILi1EEENSE_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISJ_T0_T1_T2_EENSH_9list_av_3IT3_T4_T5_E4typeEEEMSM_FSJ_SN_SO_ESR_SS_ST_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&>,boost::_bi::list_av_3<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")]
pub fn stub_0xab62b8() -> ! {
    todo!("0xab62b8 boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&>,boost::_bi::list_av_3<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")
}

// 0xab6724 — __ZN3RBX10Reflection18GenericSlotWrapper8execute2IN5boost10shared_ptrINS_8InstanceEEENS_13FriendService12FriendStatusEEEvRKT_RKT0_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute2<rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus>(rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&)")]
pub fn stub_0xab6724() -> ! {
    todo!("0xab6724 void RBX::Reflection::GenericSlotWrapper::execute2<boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus>(boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&)")
}

// 0xab6e04 — __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEENS2_13FriendService12FriendStatusEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_RKS6_EENS9_5list3INS9_5valueINS1_ISE_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEvT_
#[doc(alias = "void boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")]
pub fn stub_0xab6e04() -> ! {
    todo!("0xab6e04 void boost::function2<void,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")
}

// 0xab727c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKNS7_13FriendService12FriendStatusEEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSO_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0xab727c() -> ! {
    todo!("0xab727c boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0xab72a0 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKNS7_13FriendService12FriendStatusEEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSO_ILi2EEEEEEEvSC_SG_E6invokeERNS1_15function_bufferESC_SG_
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)")]
pub fn stub_0xab72a0() -> ! {
    todo!("0xab72a0 boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)")
}

// 0xab72c8 — __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEENS4_13FriendService12FriendStatusEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS4_10Reflection18GenericSlotWrapperERKS6_RKS8_EENSB_5list3INSB_5valueINS3_ISG_EEEENS_3argILi1EEENSQ_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0xab72c8() -> ! {
    todo!("0xab72c8 bool boost::detail::function::basic_vtable2<void,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0xab75b0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKNS7_13FriendService12FriendStatusEEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSO_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0xab75b0() -> ! {
    todo!("0xab75b0 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::FriendService::FriendStatus const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0xab8950 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvdEN3rbx6signalIS4_EEMS3_S7_EC2ES8_PKcSB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(double),rbx::signal<void ()(double)>,rbx::signal<void ()(double)> RBX::Network::Player::*>::EventDesc(rbx::signal<void ()(double)> RBX::Network::Player::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0xab8950() -> ! {
    todo!("0xab8950 RBX::Reflection::EventDesc<RBX::Network::Player,void ()(double),rbx::signal<void ()(double)>,rbx::signal<void ()(double)> RBX::Network::Player::*>::EventDesc(rbx::signal<void ()(double)> RBX::Network::Player::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

