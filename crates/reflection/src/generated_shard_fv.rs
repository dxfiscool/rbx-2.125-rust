// Auto-generated shard FV — next 100 RBX::Reflection stubs — EA-sorted asc 0xfa00..0x2e9144 (gap filler not yet in reflection)
// Source: ida/export.json filtered RBX::Reflection (fallback mangled RBX+Reflection where demangled fails) (19821 total, 12100 stubbed -> 12200 after, showing next 100, remaining 7621)
// Format: // 0xADDR — mangled + #[doc(alias = "RBX::...")] + todo!("0xADDR") using rbx_core::SharedPtr

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;
/// `RBX::Reflection::ClassDescriptor` singleton for `Described<CRenderSettingsItem>`
/// (IDA 0xfa00): guard-once init over the base `Described<Instance>` descriptor carrying
/// the `"RenderSettings"` tag; the `__cxa_atexit` teardown is drop glue.
#[derive(Debug, Clone)]
pub struct FvClassDescriptor {
    pub name: String,
    pub base: &'static str,
}

/// `RBX::Reflection::Type` metadata (IDA 0x2cbe58/0x2cbf08/0x2cc020): base `Descriptor`
/// init carrying the tag name, vtable + typeinfo install, `Name::lookup` tag with the
/// `!tag.empty()` assert (type.h:66), registered in the all-types list.
#[derive(Debug, Clone)]
pub struct FvReflectionType {
    pub name: String,
}

/// `boost::function<void(std::string,std::string,SharedPtr<Instance>)>` bound to a
/// `GenericSlotWrapper` (IDA 0x2b948c/0x2b9570): the `bind_t` triple plus the bound wrapper
/// fold into the stored target; the `arg<1..3>` placeholders forward the call args.
#[derive(Clone)]
pub struct StrInstanceSlotFn {
    pub target: SharedPtr<crate::descriptor::GenericSlotWrapper>,
}

/// `boost::function<void(SharedPtr<Instance>,std::string,std::string)>` bound to a
/// `GenericSlotWrapper` (IDA 0x2bf150/0x2bf234): same shape with the instance first.
#[derive(Clone)]
pub struct InstanceStrSlotFn {
    pub target: SharedPtr<crate::descriptor::GenericSlotWrapper>,
}

/// `SharedPtr<RBX::Reflection::Tuple const>` payload (IDA 0x2cdd74/0x2cc0cc): the tuple
/// holds one Variant per item; the const-ness is borrow mechanics.
#[derive(Debug, Clone, Default)]
pub struct LuaTuple(pub Vec<crate::descriptor::Variant>);

/// `boost::function<SharedPtr<Tuple>(SharedPtr<Tuple>)>` target (IDA 0x2cdd74): `None`
/// means the vtable invoker slot is null and the call throws `bad_function_call`.
pub struct SyncTupleFn {
    pub invoke: Option<Box<dyn Fn(SharedPtr<LuaTuple>) -> SharedPtr<LuaTuple> + Send + Sync>>,
}

/// `RBX::Lua::IAsyncResult` completion payload (IDA 0x2cc0cc): the async result object the
/// stored completion callable receives.
#[derive(Debug, Clone, Default)]
pub struct LuaAsyncResult {
    pub done: bool,
}

/// `boost::function<void(SharedPtr<Tuple>, function<void(IAsyncResult*)>)>` target
/// (IDA 0x2cc0cc): `None` means the invoker slot is null (`bad_function_call`).
pub struct AsyncTupleFn {
    pub invoke: Option<
        Box<dyn Fn(SharedPtr<LuaTuple>, Box<dyn Fn(SharedPtr<LuaAsyncResult>) + Send + Sync>) + Send + Sync>,
    >,
}

/// `RBX::RbxRay` argument of `LuaDragger` (IDA 0x2e8e80): 24-byte by-value ray; origin and
/// direction reuse the crate `Vector3` (`[f32; 3]`) model.
#[derive(Debug, Clone, Copy)]
pub struct DraggerRay {
    pub origin: crate::descriptor::Vector3,
    pub direction: crate::descriptor::Vector3,
}

// 0xfa00 — __ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: void *__fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0xfa00() -> &'static FvClassDescriptor {
    // IDA 0xfa00: guard-once init (0xfa5c/0xfac4), base `Described<Instance>::classDescriptor`
    // init (0xfa68), `ClassDescriptor(base, "RenderSettings")` (0xfaa0), `__cxa_atexit` dtor
    // (0xfabe), return the singleton (0xfaee). Rust: function-local `static`.
    static S: std::sync::LazyLock<FvClassDescriptor> = std::sync::LazyLock::new(|| FvClassDescriptor {
        name: "RenderSettings".to_owned(),
        base: "Instance",
    });
    &S
}

// 0xfb1c — __ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0xfb1c() {
    // IDA 0xfb1c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0xfb20 — __ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: int __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0xfb20() {
    // IDA 0xfb20: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0xfb34 — __ZThn32_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0xfb34() {
    // IDA 0xfb34: __ZThn32 thunk (D1 base dtor): `this -= 32`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0xfb3c — __ZThn32_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: int __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0xfb3c() {
    // IDA 0xfb3c: __ZThn32 thunk (D0 deleting dtor): `this -= 32`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0xfb54 — __ZThn36_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0xfb54() {
    // IDA 0xfb54: __ZThn36 thunk (D1 base dtor): `this -= 36`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0xfb5c — __ZThn36_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: int __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0xfb5c() {
    // IDA 0xfb5c: __ZThn36 thunk (D0 deleting dtor): `this -= 36`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x258c78 — __ZN3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x258c78() {
    // IDA 0x258c78: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x258c7c — __ZN3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x258c7c() {
    // IDA 0x258c7c: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x258d1c — __ZThn32_N3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x258d1c() {
    // IDA 0x258d1c: __ZThn32 thunk (D1 base dtor): `this -= 32`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x258d24 — __ZThn32_N3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x258d24() {
    // IDA 0x258d24: __ZThn32 thunk (D0 deleting dtor): `this -= 32`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x258dc8 — __ZThn36_N3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x258dc8() {
    // IDA 0x258dc8: __ZThn36 thunk (D1 base dtor): `this -= 36`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x258dd0 — __ZThn36_N3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_11HttpServiceELZNS_12sHttpServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sHttpServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x258dd0() {
    // IDA 0x258dd0: __ZThn36 thunk (D0 deleting dtor): `this -= 36`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x25d6ac — __ZN3RBX10Reflection9DescribedINS_9SpotLightELZNS_10sSpotLightEENS_14FactoryProductIS2_NS_5LightELZNS_10sSpotLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Light *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9SpotLightELZNS_10sSpotLightEENS_14FactoryProductIS2_NS_5LightELZNS_10sSpotLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x25d6ac() {
    // IDA 0x25d6ac: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x25d6b0 — __ZN3RBX10Reflection9DescribedINS_9SpotLightELZNS_10sSpotLightEENS_14FactoryProductIS2_NS_5LightELZNS_10sSpotLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Light *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9SpotLightELZNS_10sSpotLightEENS_14FactoryProductIS2_NS_5LightELZNS_10sSpotLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x25d6b0() {
    // IDA 0x25d6b0: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x25d750 — __ZThn32_N3RBX10Reflection9DescribedINS_9SpotLightELZNS_10sSpotLightEENS_14FactoryProductIS2_NS_5LightELZNS_10sSpotLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9SpotLightELZNS_10sSpotLightEENS_14FactoryProductIS2_NS_5LightELZNS_10sSpotLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x25d750() {
    // IDA 0x25d750: __ZThn32 thunk (D1 base dtor): `this -= 32`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x25d758 — __ZThn32_N3RBX10Reflection9DescribedINS_9SpotLightELZNS_10sSpotLightEENS_14FactoryProductIS2_NS_5LightELZNS_10sSpotLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9SpotLightELZNS_10sSpotLightEENS_14FactoryProductIS2_NS_5LightELZNS_10sSpotLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x25d758() {
    // IDA 0x25d758: __ZThn32 thunk (D0 deleting dtor): `this -= 32`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x25d7fc — __ZThn36_N3RBX10Reflection9DescribedINS_9SpotLightELZNS_10sSpotLightEENS_14FactoryProductIS2_NS_5LightELZNS_10sSpotLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9SpotLightELZNS_10sSpotLightEENS_14FactoryProductIS2_NS_5LightELZNS_10sSpotLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x25d7fc() {
    // IDA 0x25d7fc: __ZThn36 thunk (D1 base dtor): `this -= 36`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x25d804 — __ZThn36_N3RBX10Reflection9DescribedINS_9SpotLightELZNS_10sSpotLightEENS_14FactoryProductIS2_NS_5LightELZNS_10sSpotLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9SpotLightELZNS_10sSpotLightEENS_14FactoryProductIS2_NS_5LightELZNS_10sSpotLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x25d804() {
    // IDA 0x25d804: __ZThn36 thunk (D0 deleting dtor): `this -= 36`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x25d8a8 — __ZThn92_N3RBX10Reflection9DescribedINS_9SpotLightELZNS_10sSpotLightEENS_14FactoryProductIS2_NS_5LightELZNS_10sSpotLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_9SpotLightELZNS_10sSpotLightEENS_14FactoryProductIS2_NS_5LightELZNS_10sSpotLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x25d8a8() {
    // IDA 0x25d8a8: __ZThn92 thunk (D1 base dtor): `this -= 92`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x25d8b0 — __ZThn92_N3RBX10Reflection9DescribedINS_9SpotLightELZNS_10sSpotLightEENS_14FactoryProductIS2_NS_5LightELZNS_10sSpotLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_9SpotLightELZNS_10sSpotLightEENS_14FactoryProductIS2_NS_5LightELZNS_10sSpotLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x25d8b0() {
    // IDA 0x25d8b0: __ZThn92 thunk (D0 deleting dtor): `this -= 92`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x25d954 — __ZN3RBX10Reflection9DescribedINS_10PointLightELZNS_11sPointLightEENS_14FactoryProductIS2_NS_5LightELZNS_11sPointLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Light *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10PointLightELZNS_11sPointLightEENS_14FactoryProductIS2_NS_5LightELZNS_11sPointLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x25d954() {
    // IDA 0x25d954: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x25d958 — __ZN3RBX10Reflection9DescribedINS_10PointLightELZNS_11sPointLightEENS_14FactoryProductIS2_NS_5LightELZNS_11sPointLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Light *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10PointLightELZNS_11sPointLightEENS_14FactoryProductIS2_NS_5LightELZNS_11sPointLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x25d958() {
    // IDA 0x25d958: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x25d9f8 — __ZThn32_N3RBX10Reflection9DescribedINS_10PointLightELZNS_11sPointLightEENS_14FactoryProductIS2_NS_5LightELZNS_11sPointLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10PointLightELZNS_11sPointLightEENS_14FactoryProductIS2_NS_5LightELZNS_11sPointLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x25d9f8() {
    // IDA 0x25d9f8: __ZThn32 thunk (D1 base dtor): `this -= 32`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x25da00 — __ZThn32_N3RBX10Reflection9DescribedINS_10PointLightELZNS_11sPointLightEENS_14FactoryProductIS2_NS_5LightELZNS_11sPointLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10PointLightELZNS_11sPointLightEENS_14FactoryProductIS2_NS_5LightELZNS_11sPointLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x25da00() {
    // IDA 0x25da00: __ZThn32 thunk (D0 deleting dtor): `this -= 32`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x25daa4 — __ZThn36_N3RBX10Reflection9DescribedINS_10PointLightELZNS_11sPointLightEENS_14FactoryProductIS2_NS_5LightELZNS_11sPointLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10PointLightELZNS_11sPointLightEENS_14FactoryProductIS2_NS_5LightELZNS_11sPointLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x25daa4() {
    // IDA 0x25daa4: __ZThn36 thunk (D1 base dtor): `this -= 36`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x25daac — __ZThn36_N3RBX10Reflection9DescribedINS_10PointLightELZNS_11sPointLightEENS_14FactoryProductIS2_NS_5LightELZNS_11sPointLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10PointLightELZNS_11sPointLightEENS_14FactoryProductIS2_NS_5LightELZNS_11sPointLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x25daac() {
    // IDA 0x25daac: __ZThn36 thunk (D0 deleting dtor): `this -= 36`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x25db50 — __ZThn92_N3RBX10Reflection9DescribedINS_10PointLightELZNS_11sPointLightEENS_14FactoryProductIS2_NS_5LightELZNS_11sPointLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_10PointLightELZNS_11sPointLightEENS_14FactoryProductIS2_NS_5LightELZNS_11sPointLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x25db50() {
    // IDA 0x25db50: __ZThn92 thunk (D1 base dtor): `this -= 92`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x25db58 — __ZThn92_N3RBX10Reflection9DescribedINS_10PointLightELZNS_11sPointLightEENS_14FactoryProductIS2_NS_5LightELZNS_11sPointLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_10PointLightELZNS_11sPointLightEENS_14FactoryProductIS2_NS_5LightELZNS_11sPointLightEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x25db58() {
    // IDA 0x25db58: __ZThn92 thunk (D0 deleting dtor): `this -= 92`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x25dbfc — __ZN3RBX21DescribedNonCreatableINS_5LightENS_8InstanceELZNS_6sLightEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX21DescribedNonCreatableINS_5LightENS_8InstanceELZNS_6sLightEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x25dbfc() {
    // IDA 0x25dbfc: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x25dc00 — __ZN3RBX21DescribedNonCreatableINS_5LightENS_8InstanceELZNS_6sLightEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX21DescribedNonCreatableINS_5LightENS_8InstanceELZNS_6sLightEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x25dc00() {
    // IDA 0x25dc00: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x25dca0 — __ZThn32_N3RBX21DescribedNonCreatableINS_5LightENS_8InstanceELZNS_6sLightEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX21DescribedNonCreatableINS_5LightENS_8InstanceELZNS_6sLightEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x25dca0() {
    // IDA 0x25dca0: __ZThn32 thunk (D1 base dtor): `this -= 32`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x25dca8 — __ZThn32_N3RBX21DescribedNonCreatableINS_5LightENS_8InstanceELZNS_6sLightEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX21DescribedNonCreatableINS_5LightENS_8InstanceELZNS_6sLightEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x25dca8() {
    // IDA 0x25dca8: __ZThn32 thunk (D0 deleting dtor): `this -= 32`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x25dd4c — __ZThn36_N3RBX21DescribedNonCreatableINS_5LightENS_8InstanceELZNS_6sLightEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX21DescribedNonCreatableINS_5LightENS_8InstanceELZNS_6sLightEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x25dd4c() {
    // IDA 0x25dd4c: __ZThn36 thunk (D1 base dtor): `this -= 36`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x25dd54 — __ZThn36_N3RBX21DescribedNonCreatableINS_5LightENS_8InstanceELZNS_6sLightEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX21DescribedNonCreatableINS_5LightENS_8InstanceELZNS_6sLightEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x25dd54() {
    // IDA 0x25dd54: __ZThn36 thunk (D0 deleting dtor): `this -= 36`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x25ddf8 — __ZN3RBX10Reflection9DescribedINS_5LightELZNS_6sLightEENS_17NonFactoryProductINS_8InstanceELZNS_6sLightEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5LightELZNS_6sLightEENS_17NonFactoryProductINS_8InstanceELZNS_6sLightEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x25ddf8() {
    // IDA 0x25ddf8: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x25ddfc — __ZN3RBX10Reflection9DescribedINS_5LightELZNS_6sLightEENS_17NonFactoryProductINS_8InstanceELZNS_6sLightEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5LightELZNS_6sLightEENS_17NonFactoryProductINS_8InstanceELZNS_6sLightEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x25ddfc() {
    // IDA 0x25ddfc: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x25de9c — __ZThn32_N3RBX10Reflection9DescribedINS_5LightELZNS_6sLightEENS_17NonFactoryProductINS_8InstanceELZNS_6sLightEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_5LightELZNS_6sLightEENS_17NonFactoryProductINS_8InstanceELZNS_6sLightEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x25de9c() {
    // IDA 0x25de9c: __ZThn32 thunk (D1 base dtor): `this -= 32`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x25dea4 — __ZThn32_N3RBX10Reflection9DescribedINS_5LightELZNS_6sLightEENS_17NonFactoryProductINS_8InstanceELZNS_6sLightEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_5LightELZNS_6sLightEENS_17NonFactoryProductINS_8InstanceELZNS_6sLightEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x25dea4() {
    // IDA 0x25dea4: __ZThn32 thunk (D0 deleting dtor): `this -= 32`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x25df48 — __ZThn36_N3RBX10Reflection9DescribedINS_5LightELZNS_6sLightEENS_17NonFactoryProductINS_8InstanceELZNS_6sLightEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_5LightELZNS_6sLightEENS_17NonFactoryProductINS_8InstanceELZNS_6sLightEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x25df48() {
    // IDA 0x25df48: __ZThn36 thunk (D1 base dtor): `this -= 36`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x25df50 — __ZThn36_N3RBX10Reflection9DescribedINS_5LightELZNS_6sLightEENS_17NonFactoryProductINS_8InstanceELZNS_6sLightEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_5LightELZNS_6sLightEENS_17NonFactoryProductINS_8InstanceELZNS_6sLightEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x25df50() {
    // IDA 0x25df50: __ZThn36 thunk (D0 deleting dtor): `this -= 36`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x2b948c — __ZN5boost8functionIFvSsSsNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKSsSF_RKS4_EENS8_5list4INS8_5valueINS1_ISD_EEEENS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvSsSsNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKSsSF_RKS4_EENS8_5list4INS8_5valueINS1_ISD_EEEENS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x2b948c(
    target: &SharedPtr<crate::descriptor::GenericSlotWrapper>,
) -> StrInstanceSlotFn {
    // IDA 0x2b948c: `function<void(string,string,SharedPtr<Instance>)>::function<bind_t<...>>` --
    // the bind triple + shared_count copy into a temp (0x2b94b0-0x2b94c4), forward to
    // `function3::function3<bind_t>` (0x2b9506), temp release (0x2b950c-0x2b9514, drop glue).
    // The member-function triple folds into the stored target; `arg<1..3>` forward the args.
    StrInstanceSlotFn { target: SharedPtr::clone(target) }
}

// 0x2b9570 — __ZN5boost9function3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKSsSE_RKS4_EENS7_5list4INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKSsSE_RKS4_EENS7_5list4INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x2b9570(
    target: &SharedPtr<crate::descriptor::GenericSlotWrapper>,
) -> StrInstanceSlotFn {
    // IDA 0x2b9570: same `function3<void,string,string,SharedPtr<Instance>>` bind shape as
    // stub_0x2b948c (temp copy, forward, release). Same fold into the stored target.
    StrInstanceSlotFn { target: SharedPtr::clone(target) }
}

// 0x2bf150 — __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEESsS4_EEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKS4_RKSsSF_EENS8_5list4INS8_5valueINS1_ISD_EEEENS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEESsS4_EEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKS4_RKSsSF_EENS8_5list4INS8_5valueINS1_ISD_EEEENS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x2bf150(
    target: &SharedPtr<crate::descriptor::GenericSlotWrapper>,
) -> InstanceStrSlotFn {
    // IDA 0x2bf150: `function<void(SharedPtr<Instance),string,string)>::function<bind_t<...>>` --
    // bind triple + shared_count temp copy, forward to `function3::function3<bind_t>`, temp
    // release (same shape as decompiled 0x2b948c). Instance-first arg order folds into the type.
    InstanceStrSlotFn { target: SharedPtr::clone(target) }
}

// 0x2bf234 — __ZN5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsS4_EC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKS4_RKSsSE_EENS7_5list4INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsS4_EC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKS4_RKSsSE_EENS7_5list4INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x2bf234(
    target: &SharedPtr<crate::descriptor::GenericSlotWrapper>,
) -> InstanceStrSlotFn {
    // IDA 0x2bf234: same instance-first `function3` bind shape as stub_0x2bf150 (temp copy,
    // forward, release). Same fold into the stored target.
    InstanceStrSlotFn { target: SharedPtr::clone(target) }
}

// 0x2c2120 — __ZN3RBX10Reflection9DescribedINS_5Stats4ItemELZNS2_10sStatsItemEENS_17NonFactoryProductINS_8InstanceELZNS2_10sStatsItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5Stats4ItemELZNS2_10sStatsItemEENS_17NonFactoryProductINS_8InstanceELZNS2_10sStatsItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x2c2120() {
    // IDA 0x2c2120: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x2c21c0 — __ZThn32_N3RBX10Reflection9DescribedINS_5Stats4ItemELZNS2_10sStatsItemEENS_17NonFactoryProductINS_8InstanceELZNS2_10sStatsItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_5Stats4ItemELZNS2_10sStatsItemEENS_17NonFactoryProductINS_8InstanceELZNS2_10sStatsItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x2c21c0() {
    // IDA 0x2c21c0: __ZThn32 thunk (D1 base dtor): `this -= 32`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x2c21c8 — __ZThn32_N3RBX10Reflection9DescribedINS_5Stats4ItemELZNS2_10sStatsItemEENS_17NonFactoryProductINS_8InstanceELZNS2_10sStatsItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_5Stats4ItemELZNS2_10sStatsItemEENS_17NonFactoryProductINS_8InstanceELZNS2_10sStatsItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x2c21c8() {
    // IDA 0x2c21c8: __ZThn32 thunk (D0 deleting dtor): `this -= 32`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x2c7968 — __ZN3RBX10Reflection9DescribedINS_5Stats4ItemELZNS2_10sStatsItemEENS_17NonFactoryProductINS_8InstanceELZNS2_10sStatsItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5Stats4ItemELZNS2_10sStatsItemEENS_17NonFactoryProductINS_8InstanceELZNS2_10sStatsItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x2c7968() {
    // IDA 0x2c7968: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x2c7970 — __ZThn36_N3RBX10Reflection9DescribedINS_5Stats4ItemELZNS2_10sStatsItemEENS_17NonFactoryProductINS_8InstanceELZNS2_10sStatsItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_5Stats4ItemELZNS2_10sStatsItemEENS_17NonFactoryProductINS_8InstanceELZNS2_10sStatsItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x2c7970() {
    // IDA 0x2c7970: __ZThn36 thunk (D1 base dtor): `this -= 36`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x2c7978 — __ZThn36_N3RBX10Reflection9DescribedINS_5Stats4ItemELZNS2_10sStatsItemEENS_17NonFactoryProductINS_8InstanceELZNS2_10sStatsItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_5Stats4ItemELZNS2_10sStatsItemEENS_17NonFactoryProductINS_8InstanceELZNS2_10sStatsItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x2c7978() {
    // IDA 0x2c7978: __ZThn36 thunk (D0 deleting dtor): `this -= 36`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x2cb790 — __ZN3RBX10Reflection4Type12getSingletonIN5boost10shared_ptrINS3_8functionIFNS4_IKNS0_5TupleEEES8_EEEEEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonIN5boost10shared_ptrINS3_8functionIFNS4_IKNS0_5TupleEEES8_EEEEEEERKS1_v")]
// was: RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>(void)
pub fn stub_0x2cb790() -> &'static FvReflectionType {
    // IDA 0x2cb790: `Type::getSingleton<SharedPtr<function<SharedPtr<Tuple>(SharedPtr<Tuple>)>>>` --
    // guard-once (0x2cb7ee/0x2cb826) `Type::Type<T>(storage, "GenericFunction")` (0x2cb806) with
    // the TType vtable install (0x2cb822). Rust: `LazyLock`; destructor at exit. The item set
    // lands with the `Type::Type` ctor (see stub_0x2cbf08).
    static S: std::sync::LazyLock<FvReflectionType> =
        std::sync::LazyLock::new(|| stub_0x2cbf08("GenericFunction"));
    &S
}

// 0x2cb874 — __ZN3RBX10Reflection4Type12getSingletonIN5boost10shared_ptrINS3_8functionIFvNS4_IKNS0_5TupleEEENS5_IFvPNS_3Lua12IAsyncResultEEEEEEEEEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonIN5boost10shared_ptrINS3_8functionIFvNS4_IKNS0_5TupleEEENS5_IFvPNS_3Lua12IAsyncResultEEEEEEEEEEERKS1_v")]
// was: RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>>(void)
pub fn stub_0x2cb874() -> &'static FvReflectionType {
    // IDA 0x2cb874: `Type::getSingleton<SharedPtr<function<void(SharedPtr<Tuple>,
    // function<void(IAsyncResult*)>)>>>` -- guard-once (0x2cb8d2/0x2cb90a)
    // `Type::Type<T>(storage, "GenericAsyncFunction")` (0x2cb8ea), TType vtable (0x2cb906).
    // Rust: `LazyLock`; destructor at exit (see stub_0x2cbe58).
    static S: std::sync::LazyLock<FvReflectionType> =
        std::sync::LazyLock::new(|| stub_0x2cbe58("GenericAsyncFunction"));
    &S
}

// 0x2cbd58 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFNS3_IKNS_10Reflection5TupleEEES8_EEEEELb1EE13pushNewObjectISB_EEPSB_P9lua_StateT_
#[doc(alias = "rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>* RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,true>::pushNewObject<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>(lua_State *,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFNS3_IKNS_10Reflection5TupleEEES8_EEEEELb1EE13pushNewObjectISB_EEPSB_P9lua_StateT_")]
// was: boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>* RBX::Lua::Bridge<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,true>::pushNewObject<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>(lua_State *,boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>)
pub fn stub_0x2cbd58(value: &SharedPtr<SyncTupleFn>) -> SharedPtr<SyncTupleFn> {
    // IDA 0x2cbd58: `Bridge<SharedPtr<function<SharedPtr<Tuple>(SharedPtr<Tuple>)>>,
    // true>::pushNewObject` -- `lua_newuserdata(8)` (0x2cbd62), copy the `SharedPtr` (pi_ +
    // `shared_count` copy, 0x2cbd72-0x2cbd7c) into the userdata, return its pointer. The Lua
    // stack slot has no Rust form; the cloned ownership is the observable effect.
    SharedPtr::clone(value)
}

// 0x2cbda8 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFvNS3_IKNS_10Reflection5TupleEEENS4_IFvPNS0_12IAsyncResultEEEEEEEEELb1EE13pushNewObjectISF_EEPSF_P9lua_StateT_
#[doc(alias = "rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>* RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>,true>::pushNewObject<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>>(lua_State *,rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFvNS3_IKNS_10Reflection5TupleEEENS4_IFvPNS0_12IAsyncResultEEEEEEEEELb1EE13pushNewObjectISF_EEPSF_P9lua_StateT_")]
// was: boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>* RBX::Lua::Bridge<boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>,true>::pushNewObject<boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>>(lua_State *,boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>)
pub fn stub_0x2cbda8(value: &SharedPtr<AsyncTupleFn>) -> SharedPtr<AsyncTupleFn> {
    // IDA 0x2cbda8: async `Bridge<...>::pushNewObject` -- same `lua_newuserdata(8)` +
    // `SharedPtr` copy shape as stub_0x2cbd58 (0x2cbdb2-0x2cbdc6). Same cloned-ownership effect.
    SharedPtr::clone(value)
}

// 0x2cbdf8 — __ZN3RBX10Reflection5TTypeINS_3Lua15WeakFunctionRefEED1Ev
#[doc(alias = "RBX::Reflection::TType<RBX::Lua::WeakFunctionRef>::~TType()")]
#[doc(alias = "__ZN3RBX10Reflection5TTypeINS_3Lua15WeakFunctionRefEED1Ev")]
pub fn stub_0x2cbdf8() {
    // IDA 0x2cbdf8: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x2cbe50 — __ZN3RBX10Reflection5TTypeIN5boost10shared_ptrINS2_8functionIFNS3_IKNS0_5TupleEEES7_EEEEEED1Ev
#[doc(alias = "RBX::Reflection::TType<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>::~TType()")]
#[doc(alias = "__ZN3RBX10Reflection5TTypeIN5boost10shared_ptrINS2_8functionIFNS3_IKNS0_5TupleEEES7_EEEEEED1Ev")]
// was: RBX::Reflection::TType<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>::~TType()
pub fn stub_0x2cbe50() {
    // IDA 0x2cbe50: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x2cbe54 — __ZN3RBX10Reflection5TTypeIN5boost10shared_ptrINS2_8functionIFvNS3_IKNS0_5TupleEEENS4_IFvPNS_3Lua12IAsyncResultEEEEEEEEEED1Ev
#[doc(alias = "RBX::Reflection::TType<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>>::~TType()")]
#[doc(alias = "__ZN3RBX10Reflection5TTypeIN5boost10shared_ptrINS2_8functionIFvNS3_IKNS0_5TupleEEENS4_IFvPNS_3Lua12IAsyncResultEEEEEEEEEED1Ev")]
// was: RBX::Reflection::TType<boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>>::~TType()
pub fn stub_0x2cbe54() {
    // IDA 0x2cbe54: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x2cbe58 — __ZN3RBX10Reflection4TypeC2IN5boost10shared_ptrINS3_8functionIFvNS4_IKNS0_5TupleEEENS5_IFvPNS_3Lua12IAsyncResultEEEEEEEEEEEPKcPT_
#[doc(alias = "RBX::Reflection::Type::Type<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>>(char const*,rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>> *)")]
#[doc(alias = "__ZN3RBX10Reflection4TypeC2IN5boost10shared_ptrINS3_8functionIFvNS4_IKNS0_5TupleEEENS5_IFvPNS_3Lua12IAsyncResultEEEEEEEEEEEPKcPT_")]
// was: RBX::Reflection::Type::Type<boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>>(char const*,boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>> *)
pub fn stub_0x2cbe58(name: &str) -> FvReflectionType {
    // IDA 0x2cbe58: `Type::Type<SharedPtr<function<void(SharedPtr<Tuple>,
    // function<void(IAsyncResult*)>)>>>` -- base `Descriptor::Descriptor(name)` (0x2cbe6e),
    // vtable install (0x2cbe8e), typeinfo (0x2cbe90), `Name::lookup` tag (0x2cbe98-0x2cbea2),
    // `!tag.empty()` ReleaseAssert (type.h:66, 0x2cbec4), `addToAllTypes` (0x2cbef4). The tag
    // pointer `a2` selects the overload; drop glue covers the registry link.
    assert!(!name.is_empty(), "!this->tag.empty() include/reflection/type.h:66");
    FvReflectionType { name: name.to_owned() }
}

// 0x2cbf04 — __ZN3RBX10Reflection5TTypeIN5boost10shared_ptrINS2_8functionIFvNS3_IKNS0_5TupleEEENS4_IFvPNS_3Lua12IAsyncResultEEEEEEEEEED0Ev
#[doc(alias = "RBX::Reflection::TType<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>>::~TType()")]
#[doc(alias = "__ZN3RBX10Reflection5TTypeIN5boost10shared_ptrINS2_8functionIFvNS3_IKNS0_5TupleEEENS4_IFvPNS_3Lua12IAsyncResultEEEEEEEEEED0Ev")]
// was: RBX::Reflection::TType<boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>>::~TType()
pub fn stub_0x2cbf04() {
    // IDA 0x2cbf04: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x2cbf08 — __ZN3RBX10Reflection4TypeC2IN5boost10shared_ptrINS3_8functionIFNS4_IKNS0_5TupleEEES8_EEEEEEEPKcPT_
#[doc(alias = "RBX::Reflection::Type::Type<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>(char const*,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>> *)")]
#[doc(alias = "__ZN3RBX10Reflection4TypeC2IN5boost10shared_ptrINS3_8functionIFNS4_IKNS0_5TupleEEES8_EEEEEEEPKcPT_")]
// was: RBX::Reflection::Type::Type<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>(char const*,boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>> *)
pub fn stub_0x2cbf08(name: &str) -> FvReflectionType {
    // IDA 0x2cbf08: `Type::Type<SharedPtr<function<SharedPtr<Tuple>(SharedPtr<Tuple>)>>>` --
    // same ctor shape as stub_0x2cbe58 (base init 0x2cbf1e, vtable 0x2cbf3e, typeinfo 0x2cbf40,
    // tag lookup 0x2cbf48-0x2cbf52, empty-tag assert 0x2cbf74, addToAllTypes 0x2cbfa4).
    assert!(!name.is_empty(), "!this->tag.empty() include/reflection/type.h:66");
    FvReflectionType { name: name.to_owned() }
}

// 0x2cbfb4 — __ZN3RBX10Reflection5TTypeIN5boost10shared_ptrINS2_8functionIFNS3_IKNS0_5TupleEEES7_EEEEEED0Ev
#[doc(alias = "RBX::Reflection::TType<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>::~TType()")]
#[doc(alias = "__ZN3RBX10Reflection5TTypeIN5boost10shared_ptrINS2_8functionIFNS3_IKNS0_5TupleEEES7_EEEEEED0Ev")]
// was: RBX::Reflection::TType<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>::~TType()
pub fn stub_0x2cbfb4() {
    // IDA 0x2cbfb4: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x2cc020 — __ZN3RBX10Reflection4TypeC2INS_3Lua15WeakFunctionRefEEEPKcPT_
#[doc(alias = "RBX::Reflection::Type::Type<RBX::Lua::WeakFunctionRef>(char const*,RBX::Lua::WeakFunctionRef *)")]
#[doc(alias = "__ZN3RBX10Reflection4TypeC2INS_3Lua15WeakFunctionRefEEEPKcPT_")]
pub fn stub_0x2cc020(name: &str) -> FvReflectionType {
    // IDA 0x2cc020: `Type::Type<WeakFunctionRef>` -- same ctor shape (base init 0x2cc036,
    // vtable 0x2cc052, typeinfo 0x2cc054, tag lookup 0x2cc05c-0x2cc066, empty-tag assert
    // 0x2cc088, addToAllTypes 0x2cc0b8). The `WeakFunctionRef` sample selects the overload.
    assert!(!name.is_empty(), "!this->tag.empty() include/reflection/type.h:66");
    FvReflectionType { name: name.to_owned() }
}

// 0x2cc0c8 — __ZN3RBX10Reflection5TTypeINS_3Lua15WeakFunctionRefEED0Ev
#[doc(alias = "RBX::Reflection::TType<RBX::Lua::WeakFunctionRef>::~TType()")]
#[doc(alias = "__ZN3RBX10Reflection5TTypeINS_3Lua15WeakFunctionRefEED0Ev")]
pub fn stub_0x2cc0c8() {
    // IDA 0x2cc0c8: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x2cc0cc — __ZNK5boost9function2IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEENS_8functionIFvPNS2_3Lua12IAsyncResultEEEEEclES6_SC_
#[doc(alias = "boost::function2<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>>::operator()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)const")]
#[doc(alias = "__ZNK5boost9function2IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEENS_8functionIFvPNS2_3Lua12IAsyncResultEEEEEclES6_SC_")]
// was: boost::function2<void,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>>::operator()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)const
pub fn stub_0x2cc0cc(
    func: &AsyncTupleFn,
    arg: SharedPtr<LuaTuple>,
    done: Box<dyn Fn(SharedPtr<LuaAsyncResult>) + Send + Sync>,
) {
    // IDA 0x2cc0cc: `function2<void, SharedPtr<Tuple>, function<void(IAsyncResult*)>>::operator()` --
    // null invoker throws `bad_function_call` (0x2cc11c-0x2cc1a8); else the vtable invoker
    // (0x2cc128) runs over the copied tuple arg (0x2cc12a-0x2cc13e, drop glue) plus the
    // completion copied via `assign_to_own` (0x2cc142-0x2cc14c, cleared at 0x2cc166). Calling an
    // empty `boost::function` throws `bad_function_call`; expect mirrors it.
    match &func.invoke {
        Some(f) => f(arg, done),
        None => panic!("bad_function_call"),
    }
}

// 0x2cdd74 — __ZNK5boost9function1INS_10shared_ptrIKN3RBX10Reflection5TupleEEES6_EclES6_
#[doc(alias = "boost::function1<rbx_core::SharedPtr<RBX::Reflection::Tuple const>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::operator()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)const")]
#[doc(alias = "__ZNK5boost9function1INS_10shared_ptrIKN3RBX10Reflection5TupleEEES6_EclES6_")]
// was: boost::function1<boost::shared_ptr<RBX::Reflection::Tuple const>,boost::shared_ptr<RBX::Reflection::Tuple const>>::operator()(boost::shared_ptr<RBX::Reflection::Tuple const>)const
pub fn stub_0x2cdd74(func: &SyncTupleFn, arg: SharedPtr<LuaTuple>) -> SharedPtr<LuaTuple> {
    // IDA 0x2cdd74: `function1<SharedPtr<Tuple>, SharedPtr<Tuple>>::operator()` -- null invoker
    // throws `bad_function_call` (0x2cddc4-0x2cde34); else the vtable invoker (0x2cddd4) runs
    // over the copied arg (0x2cddd6-0x2cddf4, drop glue). The by-value move is the copy.
    match &func.invoke {
        Some(f) => f(arg),
        None => panic!("bad_function_call"),
    }
}

// 0x2d0be8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13AdvLuaDraggerES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::AdvLuaDragger,RBX::AdvLuaDragger>(rbx_core::SharedPtr<RBX::AdvLuaDragger> const*,RBX::AdvLuaDragger *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13AdvLuaDraggerES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::AdvLuaDragger,RBX::AdvLuaDragger>(boost::shared_ptr<RBX::AdvLuaDragger> const*,RBX::AdvLuaDragger *)const
pub fn stub_0x2d0be8() {
    // IDA 0x2d0be8: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0x2d1260 — __ZN3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x2d1260() {
    // IDA 0x2d1260: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x2d1264 — __ZN3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x2d1264() {
    // IDA 0x2d1264: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x2d1304 — __ZThn32_N3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x2d1304() {
    // IDA 0x2d1304: __ZThn32 thunk (D1 base dtor): `this -= 32`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x2d130c — __ZThn32_N3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x2d130c() {
    // IDA 0x2d130c: __ZThn32 thunk (D0 deleting dtor): `this -= 32`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x2d13b0 — __ZThn36_N3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x2d13b0() {
    // IDA 0x2d13b0: __ZThn36 thunk (D1 base dtor): `this -= 36`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x2d13b8 — __ZThn36_N3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x2d13b8() {
    // IDA 0x2d13b8: __ZThn36 thunk (D0 deleting dtor): `this -= 36`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x2e4cec — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9ExplosionES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Explosion,RBX::Explosion>(rbx_core::SharedPtr<RBX::Explosion> const*,RBX::Explosion *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9ExplosionES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Explosion,RBX::Explosion>(boost::shared_ptr<RBX::Explosion> const*,RBX::Explosion *)const
pub fn stub_0x2e4cec() {
    // IDA 0x2e4cec: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0x2e700c — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS4_IKSt6vectorIS6_SaIS6_EEEEELi3EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),3>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS4_IKSt6vectorIS6_SaIS6_EEEEELi3EED1Ev")]
// was: RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),3>::~BoundFuncDesc()
pub fn stub_0x2e700c() {
    // IDA 0x2e700c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x2e7010 — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvNS_6RbxRayEELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(RBX::RbxRay),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvNS_6RbxRayEELi1EED1Ev")]
pub fn stub_0x2e7010() {
    // IDA 0x2e7010: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x2e7108 — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvvELi0EED1Ev")]
pub fn stub_0x2e7108() {
    // IDA 0x2e7108: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x2e712c — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN3G3D7Vector34AxisEELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(G3D::Vector3::Axis),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN3G3D7Vector34AxisEELi1EED1Ev")]
pub fn stub_0x2e712c() {
    // IDA 0x2e712c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x2e78e4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10LuaDraggerES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LuaDragger,RBX::LuaDragger>(rbx_core::SharedPtr<RBX::LuaDragger> const*,RBX::LuaDragger *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10LuaDraggerES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LuaDragger,RBX::LuaDragger>(boost::shared_ptr<RBX::LuaDragger> const*,RBX::LuaDragger *)const
pub fn stub_0x2e78e4() {
    // IDA 0x2e78e4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0x2e832c — __ZN3RBX10Reflection9DescribedINS_10LuaDraggerELZNS_11sLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10LuaDraggerELZNS_11sLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x2e832c() {
    // IDA 0x2e832c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x2e8330 — __ZN3RBX10Reflection9DescribedINS_10LuaDraggerELZNS_11sLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10LuaDraggerELZNS_11sLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x2e8330() {
    // IDA 0x2e8330: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x2e83d0 — __ZThn32_N3RBX10Reflection9DescribedINS_10LuaDraggerELZNS_11sLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10LuaDraggerELZNS_11sLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x2e83d0() {
    // IDA 0x2e83d0: __ZThn32 thunk (D1 base dtor): `this -= 32`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x2e83d8 — __ZThn32_N3RBX10Reflection9DescribedINS_10LuaDraggerELZNS_11sLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10LuaDraggerELZNS_11sLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x2e83d8() {
    // IDA 0x2e83d8: __ZThn32 thunk (D0 deleting dtor): `this -= 32`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x2e847c — __ZThn36_N3RBX10Reflection9DescribedINS_10LuaDraggerELZNS_11sLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10LuaDraggerELZNS_11sLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x2e847c() {
    // IDA 0x2e847c: __ZThn36 thunk (D1 base dtor): `this -= 36`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x2e8484 — __ZThn36_N3RBX10Reflection9DescribedINS_10LuaDraggerELZNS_11sLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10LuaDraggerELZNS_11sLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x2e8484() {
    // IDA 0x2e8484: __ZThn36 thunk (D0 deleting dtor): `this -= 36`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x2e8528 — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN3G3D7Vector34AxisEELi1EEC2EMS2_FvS5_EPKcSB_S5_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(G3D::Vector3::Axis),1>::BoundFuncDesc(void (RBX::LuaDragger::*)(G3D::Vector3::Axis),char const*,char const*,G3D::Vector3::Axis,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN3G3D7Vector34AxisEELi1EEC2EMS2_FvS5_EPKcSB_S5_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0x2e8528(
    name: &str,
    category: &str,
    member0: usize,
    member1: usize,
    arg_name: &str,
    default: i32,
    permissions: u32,
    attributes: u32,
) -> crate::generated_09::CoreGuiBoundFuncDesc {
    // IDA 0x2e8528: base `FunctionDescriptor` init (0x2e8580), vtable off_1233378 (0x2e859c),
    // member-function pair at +40 (0x2e85a8), `new(4)` scoped Axis default at +48 (0x2e85cc-0x2e85d6),
    // `Type::getSingleton<Axis>` (0x2e85e0) + `typed_holder<Axis>` staging (0x2e85f6-0x2e8608),
    // `declareSignature()` (0x2e8612, see stub_0x2e86d4) fixing a void return with one Axis
    // argument. Same shape as stub_0x811644.
    let mut desc = crate::generated_09::CoreGuiBoundFuncDesc {
        name: name.to_owned(),
        category: category.to_owned(),
        member: (member0, member1),
        return_type: "void",
        args: vec![],
        permissions,
        attributes,
    };
    stub_0x2e86d4(&mut desc, arg_name, &crate::descriptor::Variant::Int(default));
    desc
    // NOTE: the +48 scoped default and the typed_holder staging are C++ lifetime machinery
    // with no Rust equivalent once the signature item carries the name; drop glue covers them.
}

// 0x2e86d4 — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN3G3D7Vector34AxisEELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(G3D::Vector3::Axis),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN3G3D7Vector34AxisEELi1EE16declareSignatureEPKcNS0_7VariantE")]
pub fn stub_0x2e86d4(
    desc: &mut crate::generated_09::CoreGuiBoundFuncDesc,
    arg_name: &str,
    _default: &crate::descriptor::Variant,
) {
    // IDA 0x2e86d4: return type fixed to `Type::getSingleton<void>` at +28 (0x2e86e4, disasm
    // `STR.W R0, [R5,#0x1C]`), `Name::declare(arg name)` (0x2e86ee), argument type
    // `Type::getSingleton<Axis>` (0x2e86f0), `SignatureDescriptor::addArgument` (0x2e8702).
    // Same shape as stub_0x811814.
    desc.return_type = "void";
    desc.args.push(crate::generated_09::CoreGuiBoundFuncSigItem {
        name: arg_name.to_owned(),
        type_name: "Axis",
    });
}

// 0x2e8704 — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN3G3D7Vector34AxisEELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(G3D::Vector3::Axis),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN3G3D7Vector34AxisEELi1EED0Ev")]
pub fn stub_0x2e8704() {
    // IDA 0x2e8704: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x2e87d8 — __ZNK3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN3G3D7Vector34AxisEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(G3D::Vector3::Axis),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN3G3D7Vector34AxisEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0x2e87d8(target: &dyn Fn(i32), arg: i32) {
    // IDA 0x2e87d8: member-offset adjust (`a2 ? a2-36 : 0`, 0x2e87de-0x2e87e0), member pair
    // load (0x2e87e8-0x2e87ec), `ArgHelper::getArg<Axis, 1>` (0x2e87f4, see stub_0x2e880c),
    // `Call1Helper::call` with member-pointer dispatch (`adj >> 1`, virtual via `adj & 1`,
    // 0x2e87f6-0x2e8802). The adjust is member-pointer mechanics; the caller passes the
    // extracted arg. Void return.
    target(arg);
}

// 0x2e880c — __ZN3RBX10Reflection9ArgHelper6getArgIN3G3D7Vector34AxisELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS6_EEPNSA_10disable_ifINSA_7is_sameIS6_NSA_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "G3D::Vector3::Axis RBX::Reflection::ArgHelper::getArg<G3D::Vector3::Axis,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<G3D::Vector3::Axis> const&,boost::disable_if<boost::is_same<G3D::Vector3::Axis,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper6getArgIN3G3D7Vector34AxisELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS6_EEPNSA_10disable_ifINSA_7is_sameIS6_NSA_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")]
// was: G3D::Vector3::Axis RBX::Reflection::ArgHelper::getArg<G3D::Vector3::Axis,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<G3D::Vector3::Axis> const&,boost::disable_if<boost::is_same<G3D::Vector3::Axis,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)
pub fn stub_0x2e880c(
    values: &[crate::descriptor::Variant],
    default: Option<i32>,
) -> i32 {
    // IDA 0x2e880c: `getArg<Axis, 1>` -- non-empty args (0x2e8844) try `try_enum` first
    // (0x2e8884, see stub_0x2e899c); on miss, the index-1 Variant against the void type
    // (0x2e8890-0x2e88a8) goes through `Variant::convert<Axis>` (0x2e88b4). Missing or
    // unconvertible (LABEL_10, 0x2e88ce) yields the scoped default or throws
    // `runtime_error("Argument %d missing or nil", 1)` (0x2e8918-0x2e896e). Numeric payloads
    // convert like `convert<int>` (truncate floats); anything else takes the LABEL_10 path
    // since `convert<Axis>` failure modes live outside this crate.
    use crate::descriptor::Variant;
    if let Some(first) = values.first() {
        let mut e = 0;
        if stub_0x2e899c(first, &mut e) {
            return e;
        }
        match first {
            Variant::Int(v) => return *v,
            Variant::Float(v) => return *v as i32,
            _ => {}
        }
    }
    match default {
        Some(d) => d,
        None => panic!("Argument 1 missing or nil (IDA 0x2e880c)"),
    }
}

// 0x2e899c — __ZN3RBX10Reflection9ArgHelper8try_enumILi1EN3G3D7Vector34AxisEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSB_7is_enumIS9_EEvE4typeE
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<1,G3D::Vector3::Axis>(RBX::Reflection::FunctionDescriptor::Arguments &,G3D::Vector3::Axis &,boost::enable_if<boost::is_enum<G3D::Vector3::Axis>,void>::type *)")]
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper8try_enumILi1EN3G3D7Vector34AxisEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSB_7is_enumIS9_EEvE4typeE")]
// was: bool RBX::Reflection::ArgHelper::try_enum<1,G3D::Vector3::Axis>(RBX::Reflection::FunctionDescriptor::Arguments &,G3D::Vector3::Axis &,boost::enable_if<boost::is_enum<G3D::Vector3::Axis>,void>::type *)
pub fn stub_0x2e899c(value: &crate::descriptor::Variant, out: &mut i32) -> bool {
    // IDA 0x2e899c: `try_enum<1, Axis>` -- `call_once` the `EnumDesc<Axis>` singleton
    // (0x2e89c2) via `doGetSingleton` (0x2e89c6), then the vf+44 convert against it
    // (0x2e89de); on 1 write `*out` and return 1 (0x2e89e4-0x2e89ea), else 0. `lookup_value`
    // covers the enum-name conversion including legacy names (cf. stub_0x4a5c1c); numeric
    // payloads convert like `convert<int>`, matching the `getArg` convert path.
    use crate::descriptor::Variant;
    match value {
        Variant::Text(name) => match crate::descriptor::stub_0x4aaf60().lookup_value(name) {
            Some(v) => {
                *out = v;
                true
            }
            None => false,
        },
        Variant::Int(v) => {
            *out = *v;
            true
        }
        Variant::Float(v) => {
            *out = *v as i32;
            true
        }
        _ => false,
    }
}

// 0x2e89f0 — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(void),0>::BoundFuncDesc(void (RBX::LuaDragger::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0x2e89f0(
    name: &str,
    category: &str,
    member0: usize,
    member1: usize,
    permissions: u32,
    attributes: u32,
) -> crate::generated_09::CoreGuiBoundFuncDesc {
    // IDA 0x2e89f0: base `FunctionDescriptor` init (0x2e8a36), vtable off_12333B8 (0x2e8a52),
    // member-function pair at +40 (0x2e8a5e), return type `Type::getSingleton<void>` at +28
    // (0x2e8a86). Arity 0, so no scoped defaults and no `declareSignature` call.
    crate::generated_09::CoreGuiBoundFuncDesc {
        name: name.to_owned(),
        category: category.to_owned(),
        member: (member0, member1),
        return_type: "void",
        args: vec![],
        permissions,
        attributes,
    }
}

// 0x2e8af4 — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvvELi0EED0Ev")]
pub fn stub_0x2e8af4() {
    // IDA 0x2e8af4: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x2e8ba8 — __ZNK3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0x2e8ba8(target: &dyn Fn()) {
    // IDA 0x2e8ba8: member-offset adjust (`a2 ? a2-36 : 0`, 0x2e8bac-0x2e8bae), member pair
    // load (0x2e8bb2-0x2e8bb4), `Call0Helper::call` with member-pointer dispatch (`adj >> 1`,
    // virtual via `adj & 1`, 0x2e8bba-0x2e8bc2). The adjust is member-pointer mechanics; the
    // observable effect is invoking the bound callable. Void return.
    target();
}

// 0x2e8bc8 — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvNS_6RbxRayEELi1EEC2EMS2_FvS3_EPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(RBX::RbxRay),1>::BoundFuncDesc(void (RBX::LuaDragger::*)(RBX::RbxRay),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvNS_6RbxRayEELi1EEC2EMS2_FvS3_EPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0x2e8bc8(
    name: &str,
    category: &str,
    member0: usize,
    member1: usize,
    arg_name: &str,
    permissions: u32,
    attributes: u32,
) -> crate::generated_09::CoreGuiBoundFuncDesc {
    // IDA 0x2e8bc8: base `FunctionDescriptor` init (0x2e8c20), vtable off_12333D8 (0x2e8c36),
    // member-function pair at +40 (0x2e8c44), void-typed default staging at +48 (0x2e8c4e),
    // `declareSignature()` (0x2e8c84, see stub_0x2e8d44) fixing a void return with one RbxRay
    // argument, staging teardown (0x2e8c8a-0x2e8c96, drop glue). The void-typed default has no
    // `Variant` form; the signature item carries the name.
    let mut desc = crate::generated_09::CoreGuiBoundFuncDesc {
        name: name.to_owned(),
        category: category.to_owned(),
        member: (member0, member1),
        return_type: "void",
        args: vec![],
        permissions,
        attributes,
    };
    stub_0x2e8d44(&mut desc, arg_name, &crate::descriptor::Variant::Bool(false));
    desc
}

// 0x2e8d44 — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvNS_6RbxRayEELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(RBX::RbxRay),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvNS_6RbxRayEELi1EE16declareSignatureEPKcNS0_7VariantE")]
pub fn stub_0x2e8d44(
    desc: &mut crate::generated_09::CoreGuiBoundFuncDesc,
    arg_name: &str,
    _default: &crate::descriptor::Variant,
) {
    // IDA 0x2e8d44: return type fixed to `Type::getSingleton<void>` at +28 (0x2e8d54),
    // `Name::declare(arg name)` (0x2e8d5e), argument type `Type::getSingleton<RbxRay>`
    // (0x2e8d60), `SignatureDescriptor::addArgument` (0x2e8d72). Same shape as stub_0x2e86d4.
    desc.return_type = "void";
    desc.args.push(crate::generated_09::CoreGuiBoundFuncSigItem {
        name: arg_name.to_owned(),
        type_name: "RbxRay",
    });
}

// 0x2e8d74 — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvNS_6RbxRayEELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(RBX::RbxRay),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvNS_6RbxRayEELi1EED0Ev")]
pub fn stub_0x2e8d74() {
    // IDA 0x2e8d74: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x2e8e80 — __ZNK3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvNS_6RbxRayEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(RBX::RbxRay),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvNS_6RbxRayEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0x2e8e80(target: &dyn Fn(DraggerRay), arg: DraggerRay) {
    // IDA 0x2e8e80: member-offset adjust (`a2 ? a2-36 : 0`, 0x2e8e8e-0x2e8e96), member pair
    // load (0x2e8e9c-0x2e8ea0), `ArgHelper::getArg<RbxRay, 1>` by-value fill of the 24-byte
    // stack ray (0x2e8ea4), `Call1Helper::call` with member-pointer dispatch (0x2e8ea8-0x2e8eb4)
    // and temp teardown (0x2e8ec8-0x2e8ee6, drop glue). By-value `Copy` is the fill semantics.
    target(arg);
}

// 0x2e8ee8 — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS4_IKSt6vectorIS6_SaIS6_EEEEELi3EEC2EMS2_FvS6_S8_SD_EPKcSJ_SJ_SJ_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),3>::BoundFuncDesc(void (RBX::LuaDragger::*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS4_IKSt6vectorIS6_SaIS6_EEEEELi3EEC2EMS2_FvS6_S8_SD_EPKcSJ_SJ_SJ_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),3>::BoundFuncDesc(void (RBX::LuaDragger::*)(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_0x2e8ee8(
    name: &str,
    category: &str,
    member0: usize,
    member1: usize,
    arg0_name: &str,
    arg1_name: &str,
    arg2_name: &str,
    permissions: u32,
    attributes: u32,
) -> crate::generated_09::CoreGuiBoundFuncDesc {
    // IDA 0x2e8ee8: base `FunctionDescriptor` init (0x2e8f40), vtable off_12333F8 (0x2e8f56),
    // member-function pair at +40 (0x2e8f5a), three void-typed default stagings at
    // +48/+52/+56 (0x2e8f60-0x2e8f78), `declareSignature()` (0x2e8fd6, see stub_0x2e9144)
    // fixing a void return with three arguments, staging teardowns (0x2e8fdc-0x2e9008, drop
    // glue). The void-typed defaults have no `Variant` form; the items carry the names.
    let mut desc = crate::generated_09::CoreGuiBoundFuncDesc {
        name: name.to_owned(),
        category: category.to_owned(),
        member: (member0, member1),
        return_type: "void",
        args: vec![],
        permissions,
        attributes,
    };
    let void_default = crate::descriptor::Variant::Bool(false);
    stub_0x2e9144(
        &mut desc,
        arg0_name,
        &void_default,
        arg1_name,
        &void_default,
        arg2_name,
        &void_default,
    );
    desc
}

// 0x2e9144 — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS4_IKSt6vectorIS6_SaIS6_EEEEELi3EE16declareSignatureEPKcNS0_7VariantESH_SI_SH_SI_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS4_IKSt6vectorIS6_SaIS6_EEEEELi3EE16declareSignatureEPKcNS0_7VariantESH_SI_SH_SI_")]
// was: RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)
pub fn stub_0x2e9144(
    desc: &mut crate::generated_09::CoreGuiBoundFuncDesc,
    arg0_name: &str,
    _default0: &crate::descriptor::Variant,
    arg1_name: &str,
    _default1: &crate::descriptor::Variant,
    arg2_name: &str,
    _default2: &crate::descriptor::Variant,
) {
    // IDA 0x2e9144: return type fixed to `Type::getSingleton<void>` at +28 (0x2e915a), then one
    // `Name::declare` + typed `getSingleton` + `addArgument` triple per argument (0x2e9164-0x2e9172,
    // 0x2e917c-0x2e918a, 0x2e9194-0x2e91a8): `SharedPtr<Instance>`, `Vector3`,
    // `SharedPtr<Vector<SharedPtr<Instance>>>`.
    desc.return_type = "void";
    desc.args.push(crate::generated_09::CoreGuiBoundFuncSigItem {
        name: arg0_name.to_owned(),
        type_name: "SharedPtr<Instance>",
    });
    desc.args.push(crate::generated_09::CoreGuiBoundFuncSigItem {
        name: arg1_name.to_owned(),
        type_name: "Vector3",
    });
    desc.args.push(crate::generated_09::CoreGuiBoundFuncSigItem {
        name: arg2_name.to_owned(),
        type_name: "SharedPtr<Vector<SharedPtr<Instance>>>",
    });
}
