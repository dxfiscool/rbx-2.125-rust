//! audio generated_audio_wd_watchdog12 — 100 stubs EA-sorted asc gap filler not yet in audio (FMOD|Sound|Audio exhausted, global gap filler)
//! Source: ida/export.json (85545 funcs) EA-sorted asc next 100 not in audio after 0x0637320 | rbx_core::SharedPtr not boost
//! Range 0x63736c..0x63c450 | existing 36203 -> 36303 distinct
//! Batch: 100 stubs | // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR mangled")

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
use std::sync::Arc;

/// `RBX::Smoke` cutover (IDA 0x637478): the `Color3` at +0x64..+0x6c,
/// the size at +0x70, the opacity at +0x74 and the rise velocity at
/// +0x78. The `Instance`/`Described`/`Effect` bases fold away.
#[derive(Debug, Clone)]
pub struct SmokeState {
    pub color: [f32; 3],
    pub size: f32,
    pub opacity: f32,
    pub rise_velocity: f32,
    pub flag_60: bool,
}
/// Float member selector for the `GetSetImpl<float getter, float
/// setter>` pairs below (IDA 0x638818/0x638838): the getter/setter
/// member pointers fold into the selector.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SmokeFloatField {
    Size,
    Opacity,
    RiseVelocity,
}
/// `RBX::Reflection::PropDescriptor<Smoke, float>` cutover
/// (IDA 0x6386d0): name/category/attributes/permissions, the bound
/// member selector and the live value.
#[derive(Debug, Clone)]
pub struct SmokeFloatProp {
    pub name: String,
    pub category: String,
    pub attributes: u32,
    pub permissions: u32,
    pub field: SmokeFloatField,
    pub value: f32,
}
impl SmokeFloatProp {
    pub fn new(
        name: &str,
        category: &str,
        field: SmokeFloatField,
        initial: f32,
        attributes: u32,
        permissions: u32,
    ) -> Self {
        Self {
            name: name.to_owned(),
            category: category.to_owned(),
            attributes,
            permissions,
            field,
            value: initial,
        }
    }
}
/// `RBX::Reflection::PropDescriptor<Smoke, G3D::Color3>` cutover
/// (IDA 0x63885c): same shape with a `Color3` value.
#[derive(Debug, Clone)]
pub struct SmokeColorProp {
    pub name: String,
    pub category: String,
    pub attributes: u32,
    pub permissions: u32,
    pub value: [f32; 3],
}
impl SmokeColorProp {
    pub fn new(
        name: &str,
        category: &str,
        initial: [f32; 3],
        attributes: u32,
        permissions: u32,
    ) -> Self {
        Self {
            name: name.to_owned(),
            category: category.to_owned(),
            attributes,
            permissions,
            value: initial,
        }
    }
}
/// `RBX::Reflection::BoundProp<bool>` cutover for `Smoke`
/// (IDA 0x638a08): name/category plus the live value. The member cell
/// folds into direct field access.
#[derive(Debug, Clone)]
pub struct SmokeBoolProp {
    pub name: String,
    pub category: String,
    pub attributes: u32,
    pub permissions: u32,
    pub value: bool,
}
impl SmokeBoolProp {
    pub fn new(
        name: &str,
        category: &str,
        initial: bool,
        attributes: u32,
        permissions: u32,
    ) -> Self {
        Self {
            name: name.to_owned(),
            category: category.to_owned(),
            attributes,
            permissions,
            value: initial,
        }
    }
}

// 0x063736c — __ZN3RBX5Smoke17setRiseVelocityUiEf
// demangled: RBX::Smoke::setRiseVelocityUi(float)
// type: _DWORD __fastcall(RBX::Smoke *__hidden this, float)
#[doc(alias = "RBX::Smoke::setRiseVelocityUi(float)")]
#[doc(alias = "__ZN3RBX5Smoke17setRiseVelocityUiEf")]
pub fn stub_063736c(smoke: &mut SmokeState, value: f32) -> bool {
    // IDA 0x63736c (`RBX::Smoke::setRiseVelocityUi`): clamps below-or-at
    // -25.0 up to -25.0 and above 25.0 down via `VMIN` (0x63736c-
    // 0x637382); when the clamped value differs from +0x78 it tail-calls
    // `setRiseVelocity` (0x637386-0x637398, which stores and raises
    // both props); when it differs from the raw input it tail-calls
    // `raisePropertyChanged` for the Ui prop (0x63739c-0x6373b4). Both
    // raises fold into the flag.
    let clamped = if value > -25.0 {
        value.min(25.0)
    } else {
        -25.0
    };
    let mut changed = false;
    if clamped != smoke.rise_velocity {
        smoke.rise_velocity = clamped;
        changed = true;
    }
    if clamped != value {
        changed = true;
    }
    changed
}

// 0x06373b8 — __ZN3RBX5Smoke7setSizeEf
// demangled: RBX::Smoke::setSize(float)
// type: _DWORD __fastcall(RBX::Smoke *__hidden this, float)
#[doc(alias = "RBX::Smoke::setSize(float)")]
#[doc(alias = "__ZN3RBX5Smoke7setSizeEf")]
pub fn stub_06373b8(smoke: &mut SmokeState, value: f32) -> bool {
    // IDA 0x6373b8 (`RBX::Smoke::setSize`): returns early when +0x70
    // matches (0x6373c0-0x6373d0), else stores (0x6373dc) and raises
    // the Size prop plus the SizeUi prop (0x6373e2-0x6373f4). Both
    // raises fold into the flag.
    if smoke.size == value {
        return false;
    }
    smoke.size = value;
    true
}

// 0x06373f8 — __ZN3RBX5Smoke10setOpacityEf
// demangled: RBX::Smoke::setOpacity(float)
// type: _DWORD __fastcall(RBX::Smoke *__hidden this, float)
#[doc(alias = "RBX::Smoke::setOpacity(float)")]
#[doc(alias = "__ZN3RBX5Smoke10setOpacityEf")]
pub fn stub_06373f8(smoke: &mut SmokeState, value: f32) -> bool {
    // IDA 0x6373f8 (`RBX::Smoke::setOpacity`): same compare-store shape
    // as `setSize` above over +0x74, raising the Opacity prop plus the
    // OpacityUi prop (0x6373f8-0x637434).
    if smoke.opacity == value {
        return false;
    }
    smoke.opacity = value;
    true
}

// 0x0637438 — __ZN3RBX5Smoke15setRiseVelocityEf
// demangled: RBX::Smoke::setRiseVelocity(float)
// type: _DWORD __fastcall(RBX::Smoke *__hidden this, float)
#[doc(alias = "RBX::Smoke::setRiseVelocity(float)")]
#[doc(alias = "__ZN3RBX5Smoke15setRiseVelocityEf")]
pub fn stub_0637438(smoke: &mut SmokeState, value: f32) -> bool {
    // IDA 0x637438 (`RBX::Smoke::setRiseVelocity`): same
    // compare-store shape over +0x78, raising the RiseVelocity prop
    // plus the RiseVelocityUi prop (0x637438-0x637474).
    if smoke.rise_velocity == value {
        return false;
    }
    smoke.rise_velocity = value;
    true
}

// 0x0637478 — __ZN3RBX5SmokeC2Ev
// demangled: RBX::Smoke::Smoke(void)
// type: _DWORD __fastcall(RBX::Smoke *__hidden this)
#[doc(alias = "RBX::Smoke::Smoke(void)")]
#[doc(alias = "__ZN3RBX5SmokeC2Ev")]
pub fn stub_0637478() -> SmokeState {
    // IDA 0x637478 (`RBX::Smoke::Smoke`): `Instance::C2("Smoke")` +
    // `Effect::C2` at +0x5C + vtable installs + class registration
    // (0x637494-0x63756a); the +0x60 flag byte is set to 1 (0x63759e-
    // 0x6375a2); the color at +0x64..+0x6c loads `G3D::Color3::white()`
    // (0x6375a6-0x6375cc); +0x70 (size) = 1.0, +0x74 (opacity) = 0.5,
    // +0x78 (rise velocity) = 1.0 (0x6375b2-0x6375de).
    SmokeState {
        color: [1.0, 1.0, 1.0],
        size: 1.0,
        opacity: 0.5,
        rise_velocity: 1.0,
        flag_60: true,
    }
}

// 0x0637668 — __ZN3RBX5SmokeD0Ev
// demangled: RBX::Smoke::~Smoke()
// type: void __fastcall(RBX::Smoke *__hidden this)
#[doc(alias = "RBX::Smoke::~Smoke()")]
#[doc(alias = "__ZN3RBX5SmokeD0Ev")]
pub fn stub_0637668() {
    // IDA 0x0637668: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0637708 — __ZN3RBX5SmokeD1Ev
// demangled: RBX::Smoke::~Smoke()
// type: void __fastcall(RBX::Smoke *__hidden this)
#[doc(alias = "RBX::Smoke::~Smoke()")]
#[doc(alias = "__ZN3RBX5SmokeD1Ev")]
pub fn stub_0637708() {
    // IDA 0x0637708: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063770c — __ZThn32_N3RBX5SmokeD0Ev
// demangled: non-virtual thunk toRBX::Smoke::~Smoke()
// type: void __fastcall(RBX::Smoke *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Smoke::~Smoke()")]
#[doc(alias = "__ZThn32_N3RBX5SmokeD0Ev")]
pub fn stub_063770c() {
    // IDA 0x063770c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0637714 — __ZThn36_N3RBX5SmokeD0Ev
// demangled: non-virtual thunk toRBX::Smoke::~Smoke()
// type: void __fastcall(RBX::Smoke *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Smoke::~Smoke()")]
#[doc(alias = "__ZThn36_N3RBX5SmokeD0Ev")]
pub fn stub_0637714() {
    // IDA 0x0637714: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063771c — __ZThn92_N3RBX5SmokeD0Ev
// demangled: non-virtual thunk toRBX::Smoke::~Smoke()
// type: void __fastcall(RBX::Smoke *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Smoke::~Smoke()")]
#[doc(alias = "__ZThn92_N3RBX5SmokeD0Ev")]
pub fn stub_063771c() {
    // IDA 0x063771c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0637724 — __ZN3RBX5SmokeD2Ev
// demangled: RBX::Smoke::~Smoke()
// type: void __fastcall(RBX::Smoke *__hidden this)
#[doc(alias = "RBX::Smoke::~Smoke()")]
#[doc(alias = "__ZN3RBX5SmokeD2Ev")]
pub fn stub_0637724() {
    // IDA 0x0637724: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x06377e0 — __ZThn32_N3RBX5SmokeD1Ev
// demangled: non-virtual thunk toRBX::Smoke::~Smoke()
// type: void __fastcall(RBX::Smoke *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Smoke::~Smoke()")]
#[doc(alias = "__ZThn32_N3RBX5SmokeD1Ev")]
pub fn stub_06377e0() {
    // IDA 0x06377e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x06377e8 — __ZThn36_N3RBX5SmokeD1Ev
// demangled: non-virtual thunk toRBX::Smoke::~Smoke()
// type: void __fastcall(RBX::Smoke *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Smoke::~Smoke()")]
#[doc(alias = "__ZThn36_N3RBX5SmokeD1Ev")]
pub fn stub_06377e8() {
    // IDA 0x06377e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x06377f0 — __ZThn92_N3RBX5SmokeD1Ev
// demangled: non-virtual thunk toRBX::Smoke::~Smoke()
// type: void __fastcall(RBX::Smoke *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Smoke::~Smoke()")]
#[doc(alias = "__ZThn92_N3RBX5SmokeD1Ev")]
pub fn stub_06377f0() {
    // IDA 0x06377f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x06377f8 — __ZNK3RBX5Smoke14getClampedSizeEv
// demangled: RBX::Smoke::getClampedSize(void)const
// type: _DWORD __fastcall(RBX::Smoke *__hidden this)
#[doc(alias = "RBX::Smoke::getClampedSize(void)const")]
#[doc(alias = "__ZNK3RBX5Smoke14getClampedSizeEv")]
pub fn stub_06377f8(smoke: &SmokeState) -> f32 {
    // IDA 0x6377f8 (`RBX::Smoke::getClampedSize`): below-or-at 0.1
    // reads 0.1, above clamps via `VMIN` to 100.0 (0x6377f8-0x63780e;
    // 0.1 is `1036831949`, 100.0 is `1120403456`).
    if smoke.size > 0.1 {
        smoke.size.min(100.0)
    } else {
        0.1
    }
}

// 0x0637820 — __ZNK3RBX5Smoke17getClampedOpacityEv
// demangled: RBX::Smoke::getClampedOpacity(void)const
// type: _DWORD __fastcall(RBX::Smoke *__hidden this)
#[doc(alias = "RBX::Smoke::getClampedOpacity(void)const")]
#[doc(alias = "__ZNK3RBX5Smoke17getClampedOpacityEv")]
pub fn stub_0637820(smoke: &SmokeState) -> f32 {
    // IDA 0x637820 (`RBX::Smoke::getClampedOpacity`): zero or below
    // reads 0.0, above clamps via `VMIN` to 1.0 (0x637820-0x637836).
    if smoke.opacity > 0.0 {
        smoke.opacity.min(1.0)
    } else {
        0.0
    }
}

// 0x0637840 — __ZNK3RBX5Smoke22getClampedRiseVelocityEv
// demangled: RBX::Smoke::getClampedRiseVelocity(void)const
// type: _DWORD __fastcall(RBX::Smoke *__hidden this)
#[doc(alias = "RBX::Smoke::getClampedRiseVelocity(void)const")]
#[doc(alias = "__ZNK3RBX5Smoke22getClampedRiseVelocityEv")]
pub fn stub_0637840(smoke: &SmokeState) -> f32 {
    // IDA 0x637840 (`RBX::Smoke::getClampedRiseVelocity`): below-or-at
    // -25.0 reads -25.0, above clamps via `VMIN` to 25.0 (0x637840-
    // 0x637856).
    if smoke.rise_velocity > -25.0 {
        smoke.rise_velocity.min(25.0)
    } else {
        -25.0
    }
}

// 0x0637860 — __ZNK3RBX5Smoke8getColorEv
// demangled: RBX::Smoke::getColor(void)const
// type: _DWORD __fastcall(RBX::Smoke *__hidden this)
#[doc(alias = "RBX::Smoke::getColor(void)const")]
#[doc(alias = "__ZNK3RBX5Smoke8getColorEv")]
pub fn stub_0637860(smoke: &SmokeState) -> [f32; 3] {
    // IDA 0x637860 (`RBX::Smoke::getColor`): copies the three words at
    // +0x64/+0x68/+0x6c to the result (0x637860-0x63786a).
    smoke.color
}

// 0x0637870 — __ZN3RBX10Reflection14PropDescriptorINS_5SmokeEN3G3D6Color3EED1Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::Smoke,G3D::Color3>::~PropDescriptor()
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Smoke,G3D::Color3>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_5SmokeEN3G3D6Color3EED1Ev")]
pub fn stub_0637870() {
    // IDA 0x0637870: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0637894 — __ZNK3RBX5Smoke10getSizeRawEv
// demangled: RBX::Smoke::getSizeRaw(void)const
// type: _DWORD __fastcall(RBX::Smoke *__hidden this)
#[doc(alias = "RBX::Smoke::getSizeRaw(void)const")]
#[doc(alias = "__ZNK3RBX5Smoke10getSizeRawEv")]
pub fn stub_0637894(smoke: &SmokeState) -> f32 {
    // IDA 0x637894 (`RBX::Smoke::getSizeRaw`): loads +0x70
    // (0x637894-0x637896) — the raw value, unlike the clamped twin at
    // 0x6377f8.
    smoke.size
}

// 0x0637898 — __ZN3RBX10Reflection14PropDescriptorINS_5SmokeEfED1Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::Smoke,float>::~PropDescriptor()
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Smoke,float>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_5SmokeEfED1Ev")]
pub fn stub_0637898() {
    // IDA 0x0637898: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x06378bc — __ZNK3RBX5Smoke13getOpacityRawEv
// demangled: RBX::Smoke::getOpacityRaw(void)const
// type: _DWORD __fastcall(RBX::Smoke *__hidden this)
#[doc(alias = "RBX::Smoke::getOpacityRaw(void)const")]
#[doc(alias = "__ZNK3RBX5Smoke13getOpacityRawEv")]
pub fn stub_06378bc(smoke: &SmokeState) -> f32 {
    // IDA 0x6378bc (`RBX::Smoke::getOpacityRaw`): loads +0x74
    // (0x6378bc-0x6378be).
    smoke.opacity
}

// 0x06378c0 — __ZNK3RBX5Smoke18getRiseVelocityRawEv
// demangled: RBX::Smoke::getRiseVelocityRaw(void)const
// type: _DWORD __fastcall(RBX::Smoke *__hidden this)
#[doc(alias = "RBX::Smoke::getRiseVelocityRaw(void)const")]
#[doc(alias = "__ZNK3RBX5Smoke18getRiseVelocityRawEv")]
pub fn stub_06378c0(smoke: &SmokeState) -> f32 {
    // IDA 0x6378c0 (`RBX::Smoke::getRiseVelocityRaw`): loads +0x78
    // (0x6378c0-0x6378c2).
    smoke.rise_velocity
}

// 0x06378c4 — __ZNK3RBX5Smoke11askAddChildEPKNS_8InstanceE
// demangled: RBX::Smoke::askAddChild(RBX::Instance const*)const
// type: _DWORD __fastcall(RBX::Smoke *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Smoke::askAddChild(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX5Smoke11askAddChildEPKNS_8InstanceE")]
pub fn stub_06378c4() -> bool {
    // IDA 0x6378c4 (`RBX::Smoke::askAddChild`): `MOVS R0, #1; BX LR`
    // — any child is accepted.
    true
}

// 0x06378c8 — __ZNK3RBX5Smoke12askSetParentEPKNS_8InstanceE
// demangled: RBX::Smoke::askSetParent(RBX::Instance const*)const
// type: _DWORD __fastcall(RBX::Smoke *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Smoke::askSetParent(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX5Smoke12askSetParentEPKNS_8InstanceE")]
pub fn stub_06378c8(parent_is_part: Option<bool>) -> bool {
    // IDA 0x6378c8 (`RBX::Smoke::askSetParent`): null parent returns 0
    // (0x6378cc-0x6378d8); else the candidate's class descriptor must
    // `isA` `Part` (0x6378da-0x6378f6), returning 0 on mismatch and 1
    // otherwise (0x6378f8-0x637902). The hierarchy walk folds into the
    // `isA` answer; null folds into `None`.
    matches!(parent_is_part, Some(true))
}

// 0x0637b90 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_5SmokeEEEN5boost10shared_ptrIT_EEv
// demangled: boost::shared_ptr<RBX::Smoke> RBX::Creatable<RBX::Instance>::create<RBX::Smoke>(void)
// type: 
#[doc(alias = "rbx_core::SharedPtr<RBX::Smoke> RBX::Creatable<RBX::Instance>::create<RBX::Smoke>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_5SmokeEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_0637b90() -> SharedPtr<SmokeState> {
    // IDA 0x637b90 (`Creatable<Instance>::create<Smoke>`): `operator
    // new(0x7c)` (0x637bae-0x637bb0), `Smoke::Smoke` (0x637be6-
    // 0x637be8), then the `shared_ptr<Smoke>` ctor with the
    // `Creatable::Deleter` (0x637bec-0x637bf6). Same shape as the `Sky`
    // twin at 0x6360a4.
    stub_0637c40(stub_0637478())
}

// 0x0637c40 — __ZN5boost10shared_ptrIN3RBX5SmokeEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// demangled: boost::shared_ptr<RBX::Smoke>::shared_ptr<RBX::Smoke,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Smoke *,RBX::Creatable<RBX::Instance>::Deleter)
// type: 
#[doc(alias = "rbx_core::SharedPtr<RBX::Smoke>::shared_ptr<RBX::Smoke,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Smoke *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX5SmokeEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_0637c40(smoke: SmokeState) -> SharedPtr<SmokeState> {
    // IDA 0x637c40 (`shared_ptr<Smoke>::shared_ptr<Smoke, Creatable
    // Deleter>`): stores the pointer (0x637c5c-0x637c60), builds the
    // `shared_count` control block (0x637c66-0x637c68) and, when
    // non-null, wires the weak owner via `_internal_accept_owner`
    // (0x637c96-0x637ca6). Same shape as the `Sky` twin at 0x636154.
    SharedPtr::new(smoke)
}

// 0x0637d08 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5SmokeES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Smoke,RBX::Smoke>(boost::shared_ptr<RBX::Smoke> const*,RBX::Smoke *)const
// type: 
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Smoke,RBX::Smoke>(rbx_core::SharedPtr<RBX::Smoke> const*,RBX::Smoke *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5SmokeES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0637d08() {
    // IDA 0x0637d08: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0x0637df0 — __ZN5boost6detail12shared_countC2IPN3RBX5SmokeENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// demangled: boost::detail::shared_count::shared_count<RBX::Smoke *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Smoke *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Smoke *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Smoke *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX5SmokeENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_0637df0() {
    // IDA 0x0637df0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x0637ef8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5SmokeENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::Smoke *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: 
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Smoke *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5SmokeENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0637ef8() {
    // IDA 0x0637ef8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0637efc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5SmokeENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::Smoke *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: 
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Smoke *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5SmokeENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_0637efc() {
    // IDA 0x0637efc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0637f00 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5SmokeENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::Smoke *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
// type: 
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Smoke *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5SmokeENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0637f00() {
    // IDA 0x0637f00: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x0637f20 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5SmokeENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_pd<RBX::Smoke *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
// type: 
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Smoke *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5SmokeENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0637f20() {
    // IDA 0x0637f20: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x0637f38 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5SmokeENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::Smoke *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
// type: 
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Smoke *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5SmokeENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_0637f38() {
    // IDA 0x0637f38: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x06386d0 — __ZN3RBX10Reflection14PropDescriptorINS_5SmokeEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::PropDescriptor<RBX::Smoke,float>::PropDescriptor<float (RBX::Smoke::*)(void)const,void (RBX::Smoke::*)(float)>(char const*,char const*,float (RBX::Smoke::*)(void)const,void (RBX::Smoke::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Smoke,float>::PropDescriptor<float (RBX::Smoke::*)(void)const,void (RBX::Smoke::*)(float)>(char const*,char const*,float (RBX::Smoke::*)(void)const,void (RBX::Smoke::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_5SmokeEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_06386d0(
    name: &str,
    category: &str,
    field: SmokeFloatField,
    initial: f32,
    attributes: u32,
    permissions: u32,
) -> SmokeFloatProp {
    // IDA 0x6386d0 (`PropDescriptor<Smoke, float>::C2`): allocates the
    // `GetSetImpl` member triple (0x14 bytes, 0x6386fc-0x638738), runs
    // `TypedPropertyDescriptor<float>::C2` (0x638750-0x638790) and
    // installs the vtable (0x638792-0x6387a8) — same shape as the
    // Skateboard twin at 0x633cb8. The member triple folds into the
    // field selector.
    SmokeFloatProp::new(name, category, field, initial, attributes, permissions)
}

// 0x06387e4 — __ZN3RBX10Reflection14PropDescriptorINS_5SmokeEfED0Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::Smoke,float>::~PropDescriptor()
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Smoke,float>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_5SmokeEfED0Ev")]
pub fn stub_06387e4() {
    // IDA 0x06387e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0638810 — __ZNK3RBX10Reflection14PropDescriptorINS_5SmokeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::Smoke,float>::GetSetImpl<float (RBX::Smoke::*)(void)const,void (RBX::Smoke::*)(float)>::isReadOnly(void)const
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Smoke,float>::GetSetImpl<float (RBX::Smoke::*)(void)const,void (RBX::Smoke::*)(float)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_5SmokeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv")]
pub fn stub_0638810() -> bool {
    // IDA 0x638810 (`GetSetImpl<float getter, float
    // setter>::isReadOnly`): `MOVS R0, #0; BX LR` — always readable.
    false
}

// 0x0638814 — __ZNK3RBX10Reflection14PropDescriptorINS_5SmokeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::Smoke,float>::GetSetImpl<float (RBX::Smoke::*)(void)const,void (RBX::Smoke::*)(float)>::isWriteOnly(void)const
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Smoke,float>::GetSetImpl<float (RBX::Smoke::*)(void)const,void (RBX::Smoke::*)(float)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_5SmokeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv")]
pub fn stub_0638814() -> bool {
    // IDA 0x638814 (`GetSetImpl<float getter, float
    // setter>::isWriteOnly`): `MOVS R0, #0; BX LR` — always writable.
    false
}

// 0x0638818 — __ZNK3RBX10Reflection14PropDescriptorINS_5SmokeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::Smoke,float>::GetSetImpl<float (RBX::Smoke::*)(void)const,void (RBX::Smoke::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Smoke,float>::GetSetImpl<float (RBX::Smoke::*)(void)const,void (RBX::Smoke::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_5SmokeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0638818(state: &SmokeState, field: SmokeFloatField) -> f32 {
    // IDA 0x638818 (`GetSetImpl::getValue`): null described reads at
    // offset 0, else `a2 - 36` (0x638818-0x63881e); resolves the getter
    // member pointer (+4/+8, virtual when the low bit is set,
    // 0x638822-0x638832) and tail-calls it (0x638834). The member
    // pointer folds into the selector.
    match field {
        SmokeFloatField::Size => state.size,
        SmokeFloatField::Opacity => state.opacity,
        SmokeFloatField::RiseVelocity => state.rise_velocity,
    }
}

// 0x0638838 — __ZNK3RBX10Reflection14PropDescriptorINS_5SmokeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
// demangled: RBX::Reflection::PropDescriptor<RBX::Smoke,float>::GetSetImpl<float (RBX::Smoke::*)(void)const,void (RBX::Smoke::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Smoke,float>::GetSetImpl<float (RBX::Smoke::*)(void)const,void (RBX::Smoke::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_5SmokeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf")]
pub fn stub_0638838(state: &mut SmokeState, field: SmokeFloatField, value: f32) -> bool {
    // IDA 0x638838 (`GetSetImpl::setValue`): same member-pointer
    // resolve as 0x638818 above over +12/+16 (0x638838-0x638854),
    // tail-calling the setter with `*a3` (0x638856-0x638858). The
    // setter is one of `setSize`/`setOpacity`/`setRiseVelocity`, each
    // of which compares, stores and raises; the raises fold into the
    // flag.
    let slot = match field {
        SmokeFloatField::Size => &mut state.size,
        SmokeFloatField::Opacity => &mut state.opacity,
        SmokeFloatField::RiseVelocity => &mut state.rise_velocity,
    };
    if *slot == value {
        return false;
    }
    *slot = value;
    true
}

// 0x063885c — __ZN3RBX10Reflection14PropDescriptorINS_5SmokeEN3G3D6Color3EEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::PropDescriptor<RBX::Smoke,G3D::Color3>::PropDescriptor<G3D::Color3 (RBX::Smoke::*)(void)const,void (RBX::Smoke::*)(G3D::Color3)>(char const*,char const*,G3D::Color3 (RBX::Smoke::*)(void)const,void (RBX::Smoke::*)(G3D::Color3),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Smoke,G3D::Color3>::PropDescriptor<G3D::Color3 (RBX::Smoke::*)(void)const,void (RBX::Smoke::*)(G3D::Color3)>(char const*,char const*,G3D::Color3 (RBX::Smoke::*)(void)const,void (RBX::Smoke::*)(G3D::Color3),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_5SmokeEN3G3D6Color3EEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_063885c(
    name: &str,
    category: &str,
    initial: [f32; 3],
    attributes: u32,
    permissions: u32,
) -> SmokeColorProp {
    // IDA 0x63885c (`PropDescriptor<Smoke, Color3>::C2`): same
    // member-triple + `TypedPropertyDescriptor<Color3>::C2` + vtable
    // shape as the float twin at 0x6386d0 (0x63885c-0x638940).
    SmokeColorProp::new(name, category, initial, attributes, permissions)
}

// 0x0638970 — __ZN3RBX10Reflection14PropDescriptorINS_5SmokeEN3G3D6Color3EED0Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::Smoke,G3D::Color3>::~PropDescriptor()
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Smoke,G3D::Color3>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_5SmokeEN3G3D6Color3EED0Ev")]
pub fn stub_0638970() {
    // IDA 0x0638970: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063899c — __ZNK3RBX10Reflection14PropDescriptorINS_5SmokeEN3G3D6Color3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::Smoke,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Smoke::*)(void)const,void (RBX::Smoke::*)(G3D::Color3)>::isReadOnly(void)const
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Smoke,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Smoke::*)(void)const,void (RBX::Smoke::*)(G3D::Color3)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_5SmokeEN3G3D6Color3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv")]
pub fn stub_063899c() -> bool {
    // IDA 0x63899c (`GetSetImpl<Color3 getter, Color3
    // setter>::isReadOnly`): `MOVS R0, #0; BX LR` — always readable.
    false
}

// 0x06389a0 — __ZNK3RBX10Reflection14PropDescriptorINS_5SmokeEN3G3D6Color3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::Smoke,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Smoke::*)(void)const,void (RBX::Smoke::*)(G3D::Color3)>::isWriteOnly(void)const
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Smoke,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Smoke::*)(void)const,void (RBX::Smoke::*)(G3D::Color3)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_5SmokeEN3G3D6Color3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv")]
pub fn stub_06389a0() -> bool {
    // IDA 0x6389a0 (`GetSetImpl<Color3 getter, Color3
    // setter>::isWriteOnly`): `MOVS R0, #0; BX LR` — always writable.
    false
}

// 0x06389a4 — __ZNK3RBX10Reflection14PropDescriptorINS_5SmokeEN3G3D6Color3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::Smoke,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Smoke::*)(void)const,void (RBX::Smoke::*)(G3D::Color3)>::getValue(RBX::Reflection::DescribedBase const*)const
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Smoke,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Smoke::*)(void)const,void (RBX::Smoke::*)(G3D::Color3)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_5SmokeEN3G3D6Color3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_06389a4(state: &SmokeState) -> [f32; 3] {
    // IDA 0x6389a4 (`GetSetImpl::getValue`): same member-pointer
    // resolve as 0x638818 above (0x6389a4-0x6389c4), tail-calling the
    // getter (0x6389c6-0x6389c8). The member is `getColor`
    // (0x637860); the pointer folds into the field.
    state.color
}

// 0x06389cc — __ZNK3RBX10Reflection14PropDescriptorINS_5SmokeEN3G3D6Color3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// demangled: RBX::Reflection::PropDescriptor<RBX::Smoke,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Smoke::*)(void)const,void (RBX::Smoke::*)(G3D::Color3)>::setValue(RBX::Reflection::DescribedBase *,G3D::Color3 const&)const
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Smoke,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Smoke::*)(void)const,void (RBX::Smoke::*)(G3D::Color3)>::setValue(RBX::Reflection::DescribedBase *,G3D::Color3 const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_5SmokeEN3G3D6Color3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
pub fn stub_06389cc(state: &mut SmokeState, value: [f32; 3]) -> bool {
    // IDA 0x6389cc (`GetSetImpl::setValue`): same member-pointer
    // resolve over +12/+16 (0x6389cc-0x6389ee), copying the three
    // input words to the stack frame (0x6389f2-0x6389fc) for the
    // setter call. The member is `setColor` (0x637264, which compares,
    // stores and raises); the pointer folds into it.
    if state.color == value {
        return false;
    }
    state.color = value;
    true
}

/// `boost::function<void(int)>` cutover for the social callbacks
/// (IDA 0x6396c4): `assign_to_own`/`clear` fold into the `Arc` clone.
pub type SocialIntFn = Arc<dyn Fn(i32) + Send + Sync>;
/// `boost::function<void(bool)>` cutover (IDA 0x639b5c).
pub type SocialBoolFn = Arc<dyn Fn(bool) + Send + Sync>;
/// `boost::function<void(std::string)>` cutover (IDA 0x6396c4).
pub type SocialStringFn = Arc<dyn Fn(String) + Send + Sync>;
/// `LuaWebService::asyncRequest` payload (IDA 0x63a5e0/0x63a888/0x63ab30):
/// the formatted URL plus the copied value/error callbacks. The parse
/// (body to `int`/`bool`/`string`) lives in the web-service machinery,
/// so each variant carries its own value callback.
pub enum SocialRequest {
    Int {
        url: String,
        on_value: SocialIntFn,
        on_error: SocialStringFn,
    },
    Text {
        url: String,
        on_value: SocialStringFn,
        on_error: SocialStringFn,
    },
    Flag {
        url: String,
        on_value: SocialBoolFn,
        on_error: SocialStringFn,
    },
}
/// `RBX::SocialService` cutover (IDA 0x63944c): the +92 flag (init 1)
/// plus the seven URL templates at +96..+120 (friend, best friend,
/// group, group rank, group role, stuff, package contents). The
/// `Instance`/`Described` bases fold away.
#[derive(Debug, Clone, Default)]
pub struct SocialServiceState {
    pub flag_92: bool,
    pub friend_url: String,
    pub best_friend_url: String,
    pub group_url: String,
    pub group_rank_url: String,
    pub group_role_url: String,
    pub stuff_url: String,
    pub package_contents_url: String,
}
/// `RBX::format` cutover (IDA 0x63972a/0x639976/0x639bc2): printf-style
/// substitution of the two ids in order (`%d`/`i`/`u`/`s` take the next
/// arg, `%%` takes `%`, anything else is kept literally). The templates
/// come from game config, so only the substitution discipline is fixed.
pub fn social_format(template: &str, a: i32, b: i32) -> String {
    let mut out = String::with_capacity(template.len() + 16);
    let argv = [a, b];
    let mut args = argv.iter();
    let mut chars = template.chars();
    while let Some(c) = chars.next() {
        if c != '%' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('%') => out.push('%'),
            Some('d') | Some('i') | Some('u') | Some('s') => match args.next() {
                Some(v) => out.push_str(&v.to_string()),
                None => out.push_str("%?"),
            },
            Some(other) => {
                out.push('%');
                out.push(other);
            }
            None => out.push('%'),
        }
    }
    out
}
/// `RBX::SocialService::StuffType` items (IDA 0x639174 `EnumDesc::C2`:
/// `addPair` Heads=0 .. Costumes=13).
pub const STUFF_TYPE_ITEMS: [(&str, i32); 14] = [
    ("Heads", 0),
    ("Faces", 1),
    ("Hats", 2),
    ("TShirts", 3),
    ("Shirts", 4),
    ("Pants", 5),
    ("Gears", 6),
    ("Torsos", 7),
    ("LeftArms", 8),
    ("RightArms", 9),
    ("LeftLegs", 10),
    ("RightLegs", 11),
    ("Bodies", 12),
    ("Costumes", 13),
];
/// `EnumDesc<StuffType>` cutover (IDA 0x639174): the item table built
/// by the 14 `addPair` calls.
#[derive(Debug, Clone, Default)]
pub struct StuffTypeDesc {
    pub items: Vec<(String, i32)>,
}

// 0x0638a08 — __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_5SmokeEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Smoke>(char const*,char const*,bool RBX::Smoke::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: 
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Smoke>(char const*,char const*,bool RBX::Smoke::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_5SmokeEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0638a08(
    name: &str,
    category: &str,
    initial: bool,
    attributes: u32,
    permissions: u32,
) -> SmokeBoolProp {
    // IDA 0x638a08 (`BoundProp<bool>::BoundProp<Smoke>`): same
    // `TypedPropertyDescriptor<bool>::C2` + vtable + member-cell shape
    // as the `Sky` twin at 0x6368d0 (0x638a08-0x638b90). The member
    // cell folds into direct field access.
    SmokeBoolProp::new(name, category, initial, attributes, permissions)
}

// 0x0638b98 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_5SmokeEE10isReadOnlyEv
// demangled: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Smoke>::isReadOnly(void)const
// type: 
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Smoke>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_5SmokeEE10isReadOnlyEv")]
pub fn stub_0638b98() -> bool {
    // IDA 0x638b98 (`BoundPropGetSet<Smoke>::isReadOnly`): `MOVS R0,
    // #0; BX LR` — always readable.
    false
}

// 0x0638b9c — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_5SmokeEE11isWriteOnlyEv
// demangled: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Smoke>::isWriteOnly(void)const
// type: 
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Smoke>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_5SmokeEE11isWriteOnlyEv")]
pub fn stub_0638b9c() -> bool {
    // IDA 0x638b9c (`BoundPropGetSet<Smoke>::isWriteOnly`): `MOVS R0,
    // #0; BX LR` — always writable.
    false
}

// 0x0638ba0 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_5SmokeEE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Smoke>::getValue(RBX::Reflection::DescribedBase const*)const
// type: 
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Smoke>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_5SmokeEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0638ba0(smoke: &SmokeState) -> bool {
    // IDA 0x638ba0 (`BoundPropGetSet<Smoke>::getValue`): loads the
    // member offset at +8, adjusts the described (`R1 - 36` when
    // non-null, 0x638ba0-0x638ba2) and returns the byte there
    // (0x638ba4). The member is the +0x60 flag (set to 1 by
    // `Smoke::Smoke`, 0x63759e-0x6375a2); the offset folds into the
    // field.
    smoke.flag_60
}

// 0x0638bac — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_5SmokeEE8setValueEPNS0_13DescribedBaseERKb
// demangled: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Smoke>::setValue(RBX::Reflection::DescribedBase *,bool const&)const
// type: 
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Smoke>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_5SmokeEE8setValueEPNS0_13DescribedBaseERKb")]
pub fn stub_0638bac(smoke: &mut SmokeState, value: bool) -> bool {
    // IDA 0x638bac (`BoundPropGetSet<Smoke>::setValue`): adjusts the
    // described (0x638bb0-0x638bb6), returns early when the byte
    // already matches (0x638bbe-0x638bc6), else stores (0x638bc8),
    // runs the member hook when the +12/+16 pair is set
    // (0x638bca-0x638bea) and tail-calls `raisePropertyChanged`
    // (0x638bee-0x638bf6). Same shape as the `Sky` twin at 0x636a74.
    if smoke.flag_60 == value {
        return false;
    }
    smoke.flag_60 = value;
    true
}

// 0x0639138 — __ZN3RBX13SocialService12setFriendUrlESs
// demangled: RBX::SocialService::setFriendUrl(std::string)
// type: 
#[doc(alias = "RBX::SocialService::setFriendUrl(std::string)")]
#[doc(alias = "__ZN3RBX13SocialService12setFriendUrlESs")]
pub fn stub_0639138(state: &mut SocialServiceState, value: String) {
    // IDA 0x639138 (`RBX::SocialService::setFriendUrl`): `ADDS R0,
    // #0x60; B string::assign` — direct assign, no raise.
    state.friend_url = value;
}

// 0x0639140 — __ZN3RBX13SocialService16setBestFriendUrlESs
// demangled: RBX::SocialService::setBestFriendUrl(std::string)
// type: 
#[doc(alias = "RBX::SocialService::setBestFriendUrl(std::string)")]
#[doc(alias = "__ZN3RBX13SocialService16setBestFriendUrlESs")]
pub fn stub_0639140(state: &mut SocialServiceState, value: String) {
    // IDA 0x639140 (`RBX::SocialService::setBestFriendUrl`): assigns
    // at +0x64 (0x639140-0x639142).
    state.best_friend_url = value;
}

// 0x0639148 — __ZN3RBX13SocialService11setGroupUrlESs
// demangled: RBX::SocialService::setGroupUrl(std::string)
// type: 
#[doc(alias = "RBX::SocialService::setGroupUrl(std::string)")]
#[doc(alias = "__ZN3RBX13SocialService11setGroupUrlESs")]
pub fn stub_0639148(state: &mut SocialServiceState, value: String) {
    // IDA 0x639148 (`RBX::SocialService::setGroupUrl`): assigns at
    // +0x68 (0x639148-0x63914a).
    state.group_url = value;
}

// 0x0639150 — __ZN3RBX13SocialService15setGroupRankUrlESs
// demangled: RBX::SocialService::setGroupRankUrl(std::string)
// type: 
#[doc(alias = "RBX::SocialService::setGroupRankUrl(std::string)")]
#[doc(alias = "__ZN3RBX13SocialService15setGroupRankUrlESs")]
pub fn stub_0639150(state: &mut SocialServiceState, value: String) {
    // IDA 0x639150 (`RBX::SocialService::setGroupRankUrl`): assigns at
    // +0x6c (0x639150-0x639152).
    state.group_rank_url = value;
}

// 0x0639158 — __ZN3RBX13SocialService15setGroupRoleUrlESs
// demangled: RBX::SocialService::setGroupRoleUrl(std::string)
// type: 
#[doc(alias = "RBX::SocialService::setGroupRoleUrl(std::string)")]
#[doc(alias = "__ZN3RBX13SocialService15setGroupRoleUrlESs")]
pub fn stub_0639158(state: &mut SocialServiceState, value: String) {
    // IDA 0x639158 (`RBX::SocialService::setGroupRoleUrl`): assigns at
    // +0x70 (0x639158-0x63915a).
    state.group_role_url = value;
}

// 0x0639160 — __ZN3RBX13SocialService11setStuffUrlESs
// demangled: RBX::SocialService::setStuffUrl(std::string)
// type: 
#[doc(alias = "RBX::SocialService::setStuffUrl(std::string)")]
#[doc(alias = "__ZN3RBX13SocialService11setStuffUrlESs")]
pub fn stub_0639160(state: &mut SocialServiceState, value: String) {
    // IDA 0x639160 (`RBX::SocialService::setStuffUrl`): assigns at
    // +0x74 (0x639160-0x639162).
    state.stuff_url = value;
}

// 0x0639168 — __ZN3RBX13SocialService21setPackageContentsUrlESs
// demangled: RBX::SocialService::setPackageContentsUrl(std::string)
// type: 
#[doc(alias = "RBX::SocialService::setPackageContentsUrl(std::string)")]
#[doc(alias = "__ZN3RBX13SocialService21setPackageContentsUrlESs")]
pub fn stub_0639168(state: &mut SocialServiceState, value: String) {
    // IDA 0x639168 (`RBX::SocialService::setPackageContentsUrl`):
    // assigns at +0x78 (0x639168-0x63916a).
    state.package_contents_url = value;
}

// 0x0639170 — __ZN3RBX10Reflection8EnumDescINS_13SocialService9StuffTypeEEC1Ev
// demangled: RBX::Reflection::EnumDesc<RBX::SocialService::StuffType>::EnumDesc(void)
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SocialService::StuffType>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_13SocialService9StuffTypeEEC1Ev")]
pub fn stub_0639170() -> StuffTypeDesc {
    // IDA 0x639170 (`EnumDesc<StuffType>::C1`): thunk tail-calling the
    // `C2` below (0x639170-0x639173).
    stub_0639174()
}

// 0x0639174 — __ZN3RBX10Reflection8EnumDescINS_13SocialService9StuffTypeEEC2Ev
// demangled: RBX::Reflection::EnumDesc<RBX::SocialService::StuffType>::EnumDesc(void)
// type: 
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SocialService::StuffType>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_13SocialService9StuffTypeEEC2Ev")]
pub fn stub_0639174() -> StuffTypeDesc {
    // IDA 0x639174 (`EnumDesc<StuffType>::C2`): `EnumDescriptor::C2`
    // with "Stuff" + typeinfo (0x6391aa), vtable install + table
    // zeroing (0x6391be-0x639244), then the 14 `addPair` calls
    // (0x63924a-0x639376: Heads=0 .. Costumes=13).
    let mut desc = StuffTypeDesc::default();
    for &(name, value) in &STUFF_TYPE_ITEMS {
        stub_063a280(&mut desc, value, name);
    }
    desc
}

// 0x0639448 — __ZN3RBX13SocialServiceC1Ev
// demangled: RBX::SocialService::SocialService(void)
// type: _DWORD __fastcall(RBX::SocialService *__hidden this)
#[doc(alias = "RBX::SocialService::SocialService(void)")]
#[doc(alias = "__ZN3RBX13SocialServiceC1Ev")]
pub fn stub_0639448() -> SocialServiceState {
    // IDA 0x639448 (`RBX::SocialService::C1`): thunk tail-calling the
    // `C2` below (0x639448-0x63944b).
    stub_063944c()
}

// 0x063944c — __ZN3RBX13SocialServiceC2Ev
// demangled: RBX::SocialService::SocialService(void)
// type: _DWORD __fastcall(RBX::SocialService *__hidden this)
#[doc(alias = "RBX::SocialService::SocialService(void)")]
#[doc(alias = "__ZN3RBX13SocialServiceC2Ev")]
pub fn stub_063944c() -> SocialServiceState {
    // IDA 0x63944c (`RBX::SocialService::C2`): `Instance::C2` + vtable
    // installs + class-descriptor registration (0x63946e-0x6394d8);
    // the +92 flag byte is set to 1 (0x6394dc-0x6394e8); the seven
    // URL strings at +96..+120 start empty (0x6394ea-0x639560).
    SocialServiceState {
        flag_92: true,
        ..SocialServiceState::default()
    }
}

// 0x06396c4 — __ZN3RBX13SocialService14getRankInGroupEiiN5boost8functionIFviEEENS2_IFvSsEEE
// demangled: RBX::SocialService::getRankInGroup(int,int,boost::function<void ()(int)>,boost::function<void ()(std::string)>)
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::SocialService::getRankInGroup(int,int,boost::function<void ()(int)>,boost::function<void ()(std::string)>)")]
#[doc(alias = "__ZN3RBX13SocialService14getRankInGroupEiiN5boost8functionIFviEEENS2_IFvSsEEE")]
pub fn stub_06396c4(
    state: &SocialServiceState,
    a: i32,
    b: i32,
    on_value: SocialIntFn,
    on_error: SocialStringFn,
    issue: Option<impl FnOnce(SocialRequest)>,
) {
    // IDA 0x6396c4 (`RBX::SocialService::getRankInGroup`): loads the
    // +108 template (0x6396ee); empty URL calls the error callback
    // with "No groupRankUrl set" (0x6397ac-0x6397e8); else
    // `RBX::format(template, a, b)` (0x63971c-0x63972a) and
    // `dispatchRequest<int>` (0x639752-0x639758, at 0x63a5e0).
    if state.group_rank_url.is_empty() {
        on_error("No groupRankUrl set".to_owned());
        return;
    }
    stub_063a5e0(
        &social_format(&state.group_rank_url, a, b),
        on_value,
        on_error,
        issue,
    );
}

// 0x0639910 — __ZN3RBX13SocialService14getRoleInGroupEiiN5boost8functionIFvSsEEES4_
// demangled: RBX::SocialService::getRoleInGroup(int,int,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::SocialService::getRoleInGroup(int,int,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
#[doc(alias = "__ZN3RBX13SocialService14getRoleInGroupEiiN5boost8functionIFvSsEEES4_")]
pub fn stub_0639910(
    state: &SocialServiceState,
    a: i32,
    b: i32,
    on_value: SocialStringFn,
    on_error: SocialStringFn,
    issue: Option<impl FnOnce(SocialRequest)>,
) {
    // IDA 0x639910 (`RBX::SocialService::getRoleInGroup`): same shape
    // over the +112 template with "No groupRoleUrl set" on empty
    // (0x63993a-0x639a34), dispatching `dispatchRequest<std::string>`
    // (0x63999e-0x6399a4, at 0x63a888).
    if state.group_role_url.is_empty() {
        on_error("No groupRoleUrl set".to_owned());
        return;
    }
    stub_063a888(
        &social_format(&state.group_role_url, a, b),
        on_value,
        on_error,
        issue,
    );
}

// 0x0639b5c — __ZN3RBX13SocialService13isFriendsWithEiiN5boost8functionIFvbEEENS2_IFvSsEEE
// demangled: RBX::SocialService::isFriendsWith(int,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::SocialService::isFriendsWith(int,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
#[doc(alias = "__ZN3RBX13SocialService13isFriendsWithEiiN5boost8functionIFvbEEENS2_IFvSsEEE")]
pub fn stub_0639b5c(
    state: &SocialServiceState,
    a: i32,
    b: i32,
    on_value: SocialBoolFn,
    on_error: SocialStringFn,
    issue: Option<impl FnOnce(SocialRequest)>,
) {
    // IDA 0x639b5c (`RBX::SocialService::isFriendsWith`): same shape
    // over the +96 template with "No friendUrl set" on empty
    // (0x639b86-0x639c80), dispatching `dispatchRequest<bool>`
    // (0x639bea-0x639bf0, at 0x63ab30).
    if state.friend_url.is_empty() {
        on_error("No friendUrl set".to_owned());
        return;
    }
    stub_063ab30(
        &social_format(&state.friend_url, a, b),
        on_value,
        on_error,
        issue,
    );
}

// 0x0639da8 — __ZN3RBX13SocialService17isBestFriendsWithEiiN5boost8functionIFvbEEENS2_IFvSsEEE
// demangled: RBX::SocialService::isBestFriendsWith(int,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::SocialService::isBestFriendsWith(int,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
#[doc(alias = "__ZN3RBX13SocialService17isBestFriendsWithEiiN5boost8functionIFvbEEENS2_IFvSsEEE")]
pub fn stub_0639da8(
    state: &SocialServiceState,
    a: i32,
    b: i32,
    on_value: SocialBoolFn,
    on_error: SocialStringFn,
    issue: Option<impl FnOnce(SocialRequest)>,
) {
    // IDA 0x639da8 (`RBX::SocialService::isBestFriendsWith`): same
    // shape over the +100 template with "No bestFriendUrl set" on
    // empty (0x639dd2-0x639ecc), dispatching `dispatchRequest<bool>`
    // (0x639e36-0x639e3c, at 0x63ab30).
    if state.best_friend_url.is_empty() {
        on_error("No bestFriendUrl set".to_owned());
        return;
    }
    stub_063ab30(
        &social_format(&state.best_friend_url, a, b),
        on_value,
        on_error,
        issue,
    );
}

// 0x0639ff4 — __ZN3RBX13SocialService9isInGroupEiiN5boost8functionIFvbEEENS2_IFvSsEEE
// demangled: RBX::SocialService::isInGroup(int,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::SocialService::isInGroup(int,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
#[doc(alias = "__ZN3RBX13SocialService9isInGroupEiiN5boost8functionIFvbEEENS2_IFvSsEEE")]
pub fn stub_0639ff4(
    state: &SocialServiceState,
    a: i32,
    b: i32,
    on_value: SocialBoolFn,
    on_error: SocialStringFn,
    issue: Option<impl FnOnce(SocialRequest)>,
) {
    // IDA 0x639ff4 (`RBX::SocialService::isInGroup`): same shape over
    // the +104 template with "No groupUrl set" on empty
    // (0x63a01e-0x63a118), dispatching `dispatchRequest<bool>`
    // (0x63a082-0x63a088, at 0x63ab30).
    if state.group_url.is_empty() {
        on_error("No groupUrl set".to_owned());
        return;
    }
    stub_063ab30(
        &social_format(&state.group_url, a, b),
        on_value,
        on_error,
        issue,
    );
}

// 0x063a240 — __ZN3RBX10Reflection13BoundFuncDescINS_13SocialServiceEFvSsELi1EED1Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::SocialService,void ()(std::string),1>::~BoundFuncDesc()
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::SocialService,void ()(std::string),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13SocialServiceEFvSsELi1EED1Ev")]
pub fn stub_063a240() {
    // IDA 0x063a240: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063a280 — __ZN3RBX10Reflection8EnumDescINS_13SocialService9StuffTypeEE7addPairES3_PKc
// demangled: RBX::Reflection::EnumDesc<RBX::SocialService::StuffType>::addPair(RBX::SocialService::StuffType,char const*)
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SocialService::StuffType>::addPair(RBX::SocialService::StuffType,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_13SocialService9StuffTypeEE7addPairES3_PKc")]
pub fn stub_063a280(desc: &mut StuffTypeDesc, value: i32, name: &str) {
    // IDA 0x63a280 (`EnumDesc<StuffType>::addPair`): `operator new(0x1c)`
    // for the `Item` (0x63a2a0-0x63a2aa), `Descriptor::C2(name)`
    // (0x63a2e4-0x63a2ec), vtable install, value at +0x10
    // (0x63a30a-0x63a312) and linkage into the item vector/map
    // (0x63a314-0x63a400). The node allocation folds into the push.
    desc.items.push((name.to_owned(), value));
}

// 0x063a5e0 — __ZN3RBX13SocialService15dispatchRequestIiEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE
// demangled: void RBX::SocialService::dispatchRequest<int>(std::string const&,boost::function<void ()(int)>,boost::function<void ()(std::string)>)
// type: int(void)
#[doc(alias = "void RBX::SocialService::dispatchRequest<int>(std::string const&,boost::function<void ()(int)>,boost::function<void ()(std::string)>)")]
#[doc(alias = "__ZN3RBX13SocialService15dispatchRequestIiEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE")]
pub fn stub_063a5e0(
    url: &str,
    on_value: SocialIntFn,
    on_error: SocialStringFn,
    issue: Option<impl FnOnce(SocialRequest)>,
) {
    // IDA 0x63a5e0 (`SocialService::dispatchRequest<int>`):
    // `ServiceProvider::create<LuaWebService>` (0x63a5fc-0x63a602);
    // null service calls the error callback with "Shutting down"
    // (0x63a634-0x63a686); else both callbacks are copied
    // (`assign_to_own`, 0x63a63a-0x63a648) and
    // `LuaWebService::asyncRequest(url, 1112014848, ok, err)` runs
    // (0x63a64c-0x63a660). The service lookup folds into the `issue`
    // seam; the response parse lives in the web machinery.
    match issue {
        Some(issue) => issue(SocialRequest::Int {
            url: url.to_owned(),
            on_value,
            on_error,
        }),
        None => on_error("Shutting down".to_owned()),
    }
}

// 0x063a888 — __ZN3RBX13SocialService15dispatchRequestISsEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE
// demangled: void RBX::SocialService::dispatchRequest<std::string>(std::string const&,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)
// type: int(void)
#[doc(alias = "void RBX::SocialService::dispatchRequest<std::string>(std::string const&,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
#[doc(alias = "__ZN3RBX13SocialService15dispatchRequestISsEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE")]
pub fn stub_063a888(
    url: &str,
    on_value: SocialStringFn,
    on_error: SocialStringFn,
    issue: Option<impl FnOnce(SocialRequest)>,
) {
    // IDA 0x63a888 (`SocialService::dispatchRequest<std::string>`):
    // same create-or-"Shutting down" + `assign_to_own` +
    // `asyncRequest(url, 1112014848, ok, err)` shape as the `int` twin
    // at 0x63a5e0 (0x63a888-0x63aac).
    match issue {
        Some(issue) => issue(SocialRequest::Text {
            url: url.to_owned(),
            on_value,
            on_error,
        }),
        None => on_error("Shutting down".to_owned()),
    }
}

// 0x063ab30 — __ZN3RBX13SocialService15dispatchRequestIbEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE
// demangled: void RBX::SocialService::dispatchRequest<bool>(std::string const&,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)
// type: int(void)
#[doc(alias = "void RBX::SocialService::dispatchRequest<bool>(std::string const&,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
#[doc(alias = "__ZN3RBX13SocialService15dispatchRequestIbEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE")]
pub fn stub_063ab30(
    url: &str,
    on_value: SocialBoolFn,
    on_error: SocialStringFn,
    issue: Option<impl FnOnce(SocialRequest)>,
) {
    // IDA 0x63ab30 (`SocialService::dispatchRequest<bool>`): same
    // create-or-"Shutting down" + `assign_to_own` +
    // `asyncRequest(url, 1112014848, ok, err)` shape as the `int` twin
    // at 0x63a5e0 (0x63ab30-0x63c54).
    match issue {
        Some(issue) => issue(SocialRequest::Flag {
            url: url.to_owned(),
            on_value,
            on_error,
        }),
        None => on_error("Shutting down".to_owned()),
    }
}

// 0x063add8 — __ZN3RBX13SocialServiceD1Ev
// demangled: RBX::SocialService::~SocialService()
// type: void __fastcall(RBX::SocialService *__hidden this)
#[doc(alias = "RBX::SocialService::~SocialService()")]
#[doc(alias = "__ZN3RBX13SocialServiceD1Ev")]
pub fn stub_063add8() {
    // IDA 0x063add8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063ae44 — __ZN3RBX13SocialServiceD0Ev
// demangled: RBX::SocialService::~SocialService()
// type: void __fastcall(RBX::SocialService *__hidden this)
#[doc(alias = "RBX::SocialService::~SocialService()")]
#[doc(alias = "__ZN3RBX13SocialServiceD0Ev")]
pub fn stub_063ae44() {
    // IDA 0x063ae44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063af0c — __ZThn32_N3RBX13SocialServiceD1Ev
// demangled: non-virtual thunk toRBX::SocialService::~SocialService()
// type: void __fastcall(RBX::SocialService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::SocialService::~SocialService()")]
#[doc(alias = "__ZThn32_N3RBX13SocialServiceD1Ev")]
pub fn stub_063af0c() {
    // IDA 0x063af0c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063af80 — __ZThn32_N3RBX13SocialServiceD0Ev
// demangled: non-virtual thunk toRBX::SocialService::~SocialService()
// type: void __fastcall(RBX::SocialService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::SocialService::~SocialService()")]
#[doc(alias = "__ZThn32_N3RBX13SocialServiceD0Ev")]
pub fn stub_063af80() {
    // IDA 0x063af80: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063b0ac — __ZThn36_N3RBX13SocialServiceD1Ev
// demangled: non-virtual thunk toRBX::SocialService::~SocialService()
// type: void __fastcall(RBX::SocialService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::SocialService::~SocialService()")]
#[doc(alias = "__ZThn36_N3RBX13SocialServiceD1Ev")]
pub fn stub_063b0ac() {
    // IDA 0x063b0ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063b120 — __ZThn36_N3RBX13SocialServiceD0Ev
// demangled: non-virtual thunk toRBX::SocialService::~SocialService()
// type: void __fastcall(RBX::SocialService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::SocialService::~SocialService()")]
#[doc(alias = "__ZThn36_N3RBX13SocialServiceD0Ev")]
pub fn stub_063b120() {
    // IDA 0x063b120: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063b420 — __ZNSt6vectorIN3RBX13SocialService9StuffTypeESaIS2_EE6resizeEmS2_
// demangled: std::vector<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>::resize(unsigned long,RBX::SocialService::StuffType)
// type: int(void)
#[doc(alias = "std::vector<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>::resize(unsigned long,RBX::SocialService::StuffType)")]
#[doc(alias = "__ZNSt6vectorIN3RBX13SocialService9StuffTypeESaIS2_EE6resizeEmS2_")]
pub fn stub_063b420() -> ! {
    todo!("0x063b420 std::vector<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>::resize(unsigned long,RBX::SocialService::StuffType)")
}

// 0x063b454 — __ZNSt6vectorIN3RBX13SocialService9StuffTypeESaIS2_EE9push_backERKS2_
// demangled: std::vector<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>::push_back(RBX::SocialService::StuffType const&)
// type: int(void)
#[doc(alias = "std::vector<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>::push_back(RBX::SocialService::StuffType const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX13SocialService9StuffTypeESaIS2_EE9push_backERKS2_")]
pub fn stub_063b454() -> ! {
    todo!("0x063b454 std::vector<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>::push_back(RBX::SocialService::StuffType const&)")
}

// 0x063b47c — __ZNSt3mapIPKN3RBX4NameENS0_13SocialService9StuffTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// demangled: std::map<RBX::Name const*,RBX::SocialService::StuffType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>>::operator[](RBX::Name const* const&)
// type: int(void)
#[doc(alias = "std::map<RBX::Name const*,RBX::SocialService::StuffType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_13SocialService9StuffTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
pub fn stub_063b47c() -> ! {
    todo!("0x063b47c std::map<RBX::Name const*,RBX::SocialService::StuffType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>>::operator[](RBX::Name const* const&)")
}

// 0x063b4d4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13SocialService9StuffTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SocialService::StuffType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>,std::pair<RBX::Name const* const,RBX::SocialService::StuffType> const&)
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SocialService::StuffType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>,std::pair<RBX::Name const* const,RBX::SocialService::StuffType> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13SocialService9StuffTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
pub fn stub_063b4d4() -> ! {
    todo!("0x063b4d4 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SocialService::StuffType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>,std::pair<RBX::Name const* const,RBX::SocialService::StuffType> const&)")
}

// 0x063b588 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13SocialService9StuffTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SocialService::StuffType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::SocialService::StuffType> const&)
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SocialService::StuffType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::SocialService::StuffType> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13SocialService9StuffTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
pub fn stub_063b588() -> ! {
    todo!("0x063b588 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SocialService::StuffType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::SocialService::StuffType> const&)")
}

// 0x063b5e0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13SocialService9StuffTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SocialService::StuffType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::SocialService::StuffType> const&)
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SocialService::StuffType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::SocialService::StuffType> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13SocialService9StuffTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
pub fn stub_063b5e0() -> ! {
    todo!("0x063b5e0 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SocialService::StuffType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::SocialService::StuffType> const&)")
}

// 0x063b648 — __ZNSt6vectorIN3RBX13SocialService9StuffTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// demangled: std::vector<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SocialService::StuffType*,std::vector<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>>,RBX::SocialService::StuffType const&)
// type: int __fastcall(int, char *, _DWORD *)
#[doc(alias = "std::vector<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SocialService::StuffType*,std::vector<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>>,RBX::SocialService::StuffType const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX13SocialService9StuffTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_063b648() -> ! {
    todo!("0x063b648 std::vector<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SocialService::StuffType*,std::vector<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>>,RBX::SocialService::StuffType const&)")
}

// 0x063b72c — __ZNSt12_Vector_baseIN3RBX13SocialService9StuffTypeESaIS2_EE11_M_allocateEm
// demangled: std::_Vector_base<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>::_M_allocate(unsigned long)
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX13SocialService9StuffTypeESaIS2_EE11_M_allocateEm")]
pub fn stub_063b72c() -> ! {
    todo!("0x063b72c std::_Vector_base<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>::_M_allocate(unsigned long)")
}

// 0x063b744 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13SocialService9StuffTypeES6_EET0_T_S8_S7_
// demangled: RBX::SocialService::StuffType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::SocialService::StuffType *,RBX::SocialService::StuffType *>(RBX::SocialService::StuffType *,RBX::SocialService::StuffType *,RBX::SocialService::StuffType *)
// type: int(void)
#[doc(alias = "RBX::SocialService::StuffType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::SocialService::StuffType *,RBX::SocialService::StuffType *>(RBX::SocialService::StuffType *,RBX::SocialService::StuffType *,RBX::SocialService::StuffType *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13SocialService9StuffTypeES6_EET0_T_S8_S7_")]
pub fn stub_063b744() -> ! {
    todo!("0x063b744 RBX::SocialService::StuffType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::SocialService::StuffType *,RBX::SocialService::StuffType *>(RBX::SocialService::StuffType *,RBX::SocialService::StuffType *,RBX::SocialService::StuffType *)")
}

// 0x063b780 — __ZNSt6vectorIN3RBX13SocialService9StuffTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// demangled: std::vector<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SocialService::StuffType*,std::vector<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>>,unsigned long,RBX::SocialService::StuffType const&)
// type: int(void)
#[doc(alias = "std::vector<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SocialService::StuffType*,std::vector<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>>,unsigned long,RBX::SocialService::StuffType const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX13SocialService9StuffTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_063b780() -> ! {
    todo!("0x063b780 std::vector<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SocialService::StuffType*,std::vector<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>>,unsigned long,RBX::SocialService::StuffType const&)")
}

// 0x063b910 — __ZN3RBX10Reflection13BoundFuncDescINS_13SocialServiceEFvSsELi1EEC2EMS2_FvSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// demangled: RBX::Reflection::BoundFuncDesc<RBX::SocialService,void ()(std::string),1>::BoundFuncDesc(void (RBX::SocialService::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::SocialService,void ()(std::string),1>::BoundFuncDesc(void (RBX::SocialService::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13SocialServiceEFvSsELi1EEC2EMS2_FvSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_063b910() -> ! {
    todo!("0x063b910 RBX::Reflection::BoundFuncDesc<RBX::SocialService,void ()(std::string),1>::BoundFuncDesc(void (RBX::SocialService::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x063ba88 — __ZN3RBX10Reflection13BoundFuncDescINS_13SocialServiceEFvSsELi1EE16declareSignatureEPKcNS0_7VariantE
// demangled: RBX::Reflection::BoundFuncDesc<RBX::SocialService,void ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)
// type: int(void)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::SocialService,void ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13SocialServiceEFvSsELi1EE16declareSignatureEPKcNS0_7VariantE")]
pub fn stub_063ba88() -> ! {
    todo!("0x063ba88 RBX::Reflection::BoundFuncDesc<RBX::SocialService,void ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x063bab8 — __ZN3RBX10Reflection13BoundFuncDescINS_13SocialServiceEFvSsELi1EED0Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::SocialService,void ()(std::string),1>::~BoundFuncDesc()
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::SocialService,void ()(std::string),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13SocialServiceEFvSsELi1EED0Ev")]
pub fn stub_063bab8() {
    // IDA 0x063bab8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063bb84 — __ZNK3RBX10Reflection13BoundFuncDescINS_13SocialServiceEFvSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// demangled: RBX::Reflection::BoundFuncDesc<RBX::SocialService,void ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::SocialService,void ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_13SocialServiceEFvSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_063bb84() -> ! {
    todo!("0x063bb84 RBX::Reflection::BoundFuncDesc<RBX::SocialService,void ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x063bcc0 — __ZN3RBX10Reflection11Call1HelperINS_13SocialServiceEMS2_FvSsESsvE4callEPS2_S4_RNS0_7VariantERKSs
// demangled: RBX::Reflection::Call1Helper<RBX::SocialService,void (RBX::SocialService::*)(std::string),std::string,void>::call(RBX::SocialService*,void (RBX::SocialService::*)(std::string),RBX::Reflection::Variant &,std::string const&)
// type: int __fastcall(int, int, int, int, std::string *)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::SocialService,void (RBX::SocialService::*)(std::string),std::string,void>::call(RBX::SocialService*,void (RBX::SocialService::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call1HelperINS_13SocialServiceEMS2_FvSsESsvE4callEPS2_S4_RNS0_7VariantERKSs")]
pub fn stub_063bcc0() -> ! {
    todo!("0x063bcc0 RBX::Reflection::Call1Helper<RBX::SocialService,void (RBX::SocialService::*)(std::string),std::string,void>::call(RBX::SocialService*,void (RBX::SocialService::*)(std::string),RBX::Reflection::Variant &,std::string const&)")
}

// 0x063bdf0 — __ZN5boost9function1IvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEEEE5clearEv
// demangled: boost::function1<void,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::clear(void)
// type: int(void)
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::clear(void)")]
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEEEE5clearEv")]
pub fn stub_063bdf0() {
    // IDA 0x063bdf0: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

// 0x063c1a4 — __ZN3RBX8Sparkles8setColorEN3G3D6Color3E
// demangled: RBX::Sparkles::setColor(G3D::Color3)
// type: 
#[doc(alias = "RBX::Sparkles::setColor(G3D::Color3)")]
#[doc(alias = "__ZN3RBX8Sparkles8setColorEN3G3D6Color3E")]
pub fn stub_063c1a4() -> ! {
    todo!("0x063c1a4 RBX::Sparkles::setColor(G3D::Color3)")
}

// 0x063c208 — __ZNK3RBX8Sparkles14getLegacyColorEv
// demangled: RBX::Sparkles::getLegacyColor(void)const
// type: _DWORD __fastcall(RBX::Sparkles *__hidden this)
#[doc(alias = "RBX::Sparkles::getLegacyColor(void)const")]
#[doc(alias = "__ZNK3RBX8Sparkles14getLegacyColorEv")]
pub fn stub_063c208() -> ! {
    todo!("0x063c208 RBX::Sparkles::getLegacyColor(void)const")
}

// 0x063c248 — __ZN3RBX8Sparkles14setLegacyColorEN3G3D6Color3E
// demangled: RBX::Sparkles::setLegacyColor(G3D::Color3)
// type: 
#[doc(alias = "RBX::Sparkles::setLegacyColor(G3D::Color3)")]
#[doc(alias = "__ZN3RBX8Sparkles14setLegacyColorEN3G3D6Color3E")]
pub fn stub_063c248() -> ! {
    todo!("0x063c248 RBX::Sparkles::setLegacyColor(G3D::Color3)")
}

// 0x063c294 — __ZN3RBX8SparklesC1Ev
// demangled: RBX::Sparkles::Sparkles(void)
// type: _DWORD __fastcall(RBX::Sparkles *__hidden this)
#[doc(alias = "RBX::Sparkles::Sparkles(void)")]
#[doc(alias = "__ZN3RBX8SparklesC1Ev")]
pub fn stub_063c294() -> ! {
    todo!("0x063c294 RBX::Sparkles::Sparkles(void)")
}

// 0x063c298 — __ZN3RBX8SparklesC2Ev
// demangled: RBX::Sparkles::Sparkles(void)
// type: _DWORD __fastcall(RBX::Sparkles *__hidden this)
#[doc(alias = "RBX::Sparkles::Sparkles(void)")]
#[doc(alias = "__ZN3RBX8SparklesC2Ev")]
pub fn stub_063c298() -> ! {
    todo!("0x063c298 RBX::Sparkles::Sparkles(void)")
}

// 0x063c450 — __ZNK3RBX8Sparkles8getColorEv
// demangled: RBX::Sparkles::getColor(void)const
// type: _DWORD __fastcall(RBX::Sparkles *__hidden this)
#[doc(alias = "RBX::Sparkles::getColor(void)const")]
#[doc(alias = "__ZNK3RBX8Sparkles8getColorEv")]
pub fn stub_063c450() -> ! {
    todo!("0x063c450 RBX::Sparkles::getColor(void)const")
}
