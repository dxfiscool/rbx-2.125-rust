//! reflection — generated_refl_wdog_1788370696 — 120 stubs EA-sorted asc 0x3907ac..0x3992dc (reflection gap filler distinct not yet in crates/reflection/src — next 120 uncovered; RBX::Reflection filter yielded 0 remaining so global reflection-gap asc, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo)
//! Source: ida/export.json (85545 funcs) EA asc not yet in crates/reflection/src — next 120 uncovered sorted asc (RBX::Reflection strict filter exhausted 0 remaining -> fallback to global reflection-gap)
//! Format: // 0xADDR — mangled + #[doc(alias = "RBX::...")] + todo!("0xADDR") using rbx_core::SharedPtr not boost

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x3907ac — __ZThn32_N3RBX3HatD1Ev
#[doc(alias = "non-virtual thunk toRBX::Hat::~Hat()")]
#[doc(alias = "__ZThn32_N3RBX3HatD1Ev")]
pub fn stub_0x3907ac() {
    // IDA 0x3907ac: __ZThn32 thunk (D1 base dtor): `this -= 32`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x3907c0 — __ZThn32_N3RBX3HatD0Ev
#[doc(alias = "non-virtual thunk toRBX::Hat::~Hat()")]
#[doc(alias = "__ZThn32_N3RBX3HatD0Ev")]
pub fn stub_0x3907c0() {
    // IDA 0x3907c0: __ZThn32 thunk (D0 deleting dtor): `this -= 32`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x390874 — __ZThn32_NK3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x390874() -> &'static str {
    // IDA 0x390874: __ZThn getClassName — `Creator = static_getCreator(); return Creator::getClassName_shim(Creator)` (decompiled 0x390874 family; e.g. 0x28e128). The Creator name is the class name. Rust: no vtable/Creator needed.
    "Hat"
}

// 0x390884 — __ZThn36_N3RBX3HatD1Ev
#[doc(alias = "non-virtual thunk toRBX::Hat::~Hat()")]
#[doc(alias = "__ZThn36_N3RBX3HatD1Ev")]
pub fn stub_0x390884() {
    // IDA 0x390884: __ZThn36 thunk (D1 base dtor): `this -= 36`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x390898 — __ZThn36_N3RBX3HatD0Ev
#[doc(alias = "non-virtual thunk toRBX::Hat::~Hat()")]
#[doc(alias = "__ZThn36_N3RBX3HatD0Ev")]
pub fn stub_0x390898() {
    // IDA 0x390898: __ZThn36 thunk (D0 deleting dtor): `this -= 36`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x39094c — __ZThn92_N3RBX3HatD1Ev
#[doc(alias = "non-virtual thunk toRBX::Hat::~Hat()")]
#[doc(alias = "__ZThn92_N3RBX3HatD1Ev")]
pub fn stub_0x39094c() {
    // IDA 0x39094c: __ZThn92 thunk (D1 base dtor): `this -= 92`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x390960 — __ZThn92_N3RBX3HatD0Ev
#[doc(alias = "non-virtual thunk toRBX::Hat::~Hat()")]
#[doc(alias = "__ZThn92_N3RBX3HatD0Ev")]
pub fn stub_0x390960() {
    // IDA 0x390960: __ZThn92 thunk (D0 deleting dtor): `this -= 92`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x390a14 — __ZThn128_N3RBX3HatD1Ev
#[doc(alias = "non-virtual thunk toRBX::Hat::~Hat()")]
#[doc(alias = "__ZThn128_N3RBX3HatD1Ev")]
pub fn stub_0x390a14() {
    // IDA 0x390a14: __ZThn128 thunk (D1 base dtor): `this -= 128`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x390a28 — __ZThn128_N3RBX3HatD0Ev
#[doc(alias = "non-virtual thunk toRBX::Hat::~Hat()")]
#[doc(alias = "__ZThn128_N3RBX3HatD0Ev")]
pub fn stub_0x390a28() {
    // IDA 0x390a28: __ZThn128 thunk (D0 deleting dtor): `this -= 128`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x390df4 — __ZN3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED1Ev")]
pub fn stub_0x390df4() {
    // IDA 0x390df4: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x390e08 — __ZN3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED0Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED0Ev")]
pub fn stub_0x390e08() {
    // IDA 0x390e08: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x390eb8 — __ZThn128_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED1Ev
#[doc(alias = "__ZThn128_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED1Ev")]
pub fn stub_0x390eb8() {
    // IDA 0x390eb8: __ZThn128 thunk (D1 base dtor): `this -= 128`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x390ecc — __ZThn128_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED0Ev
#[doc(alias = "__ZThn128_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED0Ev")]
pub fn stub_0x390ecc() {
    // IDA 0x390ecc: __ZThn128 thunk (D0 deleting dtor): `this -= 128`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x390ed4 — __ZThn32_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED0Ev
#[doc(alias = "__ZThn32_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED0Ev")]
pub fn stub_0x390ed4() {
    // IDA 0x390ed4: __ZThn32 thunk (D0 deleting dtor): `this -= 32`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x390edc — __ZThn36_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED0Ev
#[doc(alias = "__ZThn36_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED0Ev")]
pub fn stub_0x390edc() {
    // IDA 0x390edc: __ZThn36 thunk (D0 deleting dtor): `this -= 36`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x390ee4 — __ZThn92_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED0Ev
#[doc(alias = "__ZThn92_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED0Ev")]
pub fn stub_0x390ee4() {
    // IDA 0x390ee4: __ZThn92 thunk (D0 deleting dtor): `this -= 92`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x390eec — __ZThn32_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED1Ev
#[doc(alias = "__ZThn32_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED1Ev")]
pub fn stub_0x390eec() {
    // IDA 0x390eec: __ZThn32 thunk (D1 base dtor): `this -= 32`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x390f00 — __ZThn36_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED1Ev
#[doc(alias = "__ZThn36_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED1Ev")]
pub fn stub_0x390f00() {
    // IDA 0x390f00: __ZThn36 thunk (D1 base dtor): `this -= 36`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x390f14 — __ZThn92_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED1Ev
#[doc(alias = "__ZThn92_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED1Ev")]
pub fn stub_0x390f14() {
    // IDA 0x390f14: __ZThn92 thunk (D1 base dtor): `this -= 92`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x3913d8 — __ZN3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE17static_getCreatorEv
#[doc(alias = "__ZN3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x3913d8() -> ! {
    todo!("0x3913d8 __ZN3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE17static_getCreatorEv")
}

// 0x39144c — __ZNK3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x39144c() -> ! {
    todo!("0x39144c __ZNK3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE7Creator12getClassNameEv")
}

// 0x3914d4 — __ZN3RBX4Name13callDoDeclareILZNS_4sHatEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_4sHatEEEEvv")]
pub fn stub_0x3914d4() -> ! {
    todo!("0x3914d4 __ZN3RBX4Name13callDoDeclareILZNS_4sHatEEEEvv")
}

// 0x3914d8 — __ZN3RBX4Name9doDeclareILZNS_4sHatEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_4sHatEEEERKS0_v")]
pub fn stub_0x3914d8() -> ! {
    todo!("0x3914d8 __ZN3RBX4Name9doDeclareILZNS_4sHatEEEERKS0_v")
}

// 0x3915b8 — __ZN3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE7CreatorD2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x3915b8() {
    // IDA 0x3915b8: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x391654 — __ZNK3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x391654() -> ! {
    todo!("0x391654 __ZNK3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE7Creator6createEv")
}

// 0x391798 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_3HatEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::Hat> RBX::Creatable<RBX::Instance>::create<RBX::Hat>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_3HatEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_0x391798() -> ! {
    todo!("0x391798 boost::shared_ptr<RBX::Hat> RBX::Creatable<RBX::Instance>::create<RBX::Hat>(void)")
}

// 0x391848 — __ZN5boost10shared_ptrIN3RBX3HatEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::Hat>::shared_ptr<RBX::Hat,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX3HatEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_0x391848() -> ! {
    todo!("0x391848 boost::shared_ptr<RBX::Hat>::shared_ptr<RBX::Hat,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x3919f8 — __ZN5boost6detail12shared_countC2IPN3RBX3HatENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX3HatENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_0x3919f8() -> ! {
    todo!("0x3919f8 boost::detail::shared_count::shared_count<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x391b00 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX3HatENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX3HatENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0x391b00() {
    // IDA 0x391b00: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x391b04 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX3HatENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX3HatENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_0x391b04() {
    // IDA 0x391b04: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x391b08 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX3HatENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX3HatENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0x391b08() -> ! {
    todo!("0x391b08 boost::detail::sp_counted_impl_pd<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x391b28 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX3HatENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX3HatENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0x391b28() -> ! {
    todo!("0x391b28 boost::detail::sp_counted_impl_pd<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x391b40 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX3HatENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX3HatENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_0x391b40() -> ! {
    todo!("0x391b40 boost::detail::sp_counted_impl_pd<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x391b44 — __ZN3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE7CreatorC2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x391b44() -> ! {
    todo!("0x391b44 __ZN3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE7CreatorC2Ev")
}

// 0x391d88 — __ZN3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E7CreatorD2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E7CreatorD2Ev")]
pub fn stub_0x391d88() {
    // IDA 0x391d88: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x391e24 — __ZNK3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E7Creator12getClassNameEv")]
pub fn stub_0x391e24() -> ! {
    todo!("0x391e24 __ZNK3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E7Creator12getClassNameEv")
}

// 0x391eac — __ZNK3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E7Creator6createEv")]
pub fn stub_0x391eac() -> ! {
    todo!("0x391eac __ZNK3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E7Creator6createEv")
}

// 0x391ff0 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_12AccoutrementEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::Accoutrement> RBX::Creatable<RBX::Instance>::create<RBX::Accoutrement>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_12AccoutrementEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_0x391ff0() -> ! {
    todo!("0x391ff0 boost::shared_ptr<RBX::Accoutrement> RBX::Creatable<RBX::Instance>::create<RBX::Accoutrement>(void)")
}

// 0x3920a0 — __ZN5boost10shared_ptrIN3RBX12AccoutrementEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::Accoutrement>::shared_ptr<RBX::Accoutrement,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX12AccoutrementEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_0x3920a0() -> ! {
    todo!("0x3920a0 boost::shared_ptr<RBX::Accoutrement>::shared_ptr<RBX::Accoutrement,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x392250 — __ZN5boost6detail12shared_countC2IPN3RBX12AccoutrementENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX12AccoutrementENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_0x392250() -> ! {
    todo!("0x392250 boost::detail::shared_count::shared_count<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x392358 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AccoutrementENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AccoutrementENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0x392358() {
    // IDA 0x392358: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x39235c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AccoutrementENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AccoutrementENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_0x39235c() {
    // IDA 0x39235c: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x392360 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AccoutrementENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AccoutrementENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0x392360() -> ! {
    todo!("0x392360 boost::detail::sp_counted_impl_pd<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x392380 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AccoutrementENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AccoutrementENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0x392380() -> ! {
    todo!("0x392380 boost::detail::sp_counted_impl_pd<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x392398 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AccoutrementENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AccoutrementENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_0x392398() -> ! {
    todo!("0x392398 boost::detail::sp_counted_impl_pd<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x39239c — __ZN3RBX4Name13callDoDeclareILZNS_13sAccoutrementEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sAccoutrementEEEEvv")]
pub fn stub_0x39239c() -> ! {
    todo!("0x39239c __ZN3RBX4Name13callDoDeclareILZNS_13sAccoutrementEEEEvv")
}

// 0x3923a0 — __ZN3RBX4Name9doDeclareILZNS_13sAccoutrementEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sAccoutrementEEEERKS0_v")]
pub fn stub_0x3923a0() -> ! {
    todo!("0x3923a0 __ZN3RBX4Name9doDeclareILZNS_13sAccoutrementEEEERKS0_v")
}

// 0x392480 — __ZN3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E7CreatorC2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E7CreatorC2Ev")]
pub fn stub_0x392480() -> ! {
    todo!("0x392480 __ZN3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E7CreatorC2Ev")
}

// 0x3926c4 — __ZN3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E17static_getCreatorEv
#[doc(alias = "__ZN3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E17static_getCreatorEv")]
pub fn stub_0x3926c4() -> ! {
    todo!("0x3926c4 __ZN3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E17static_getCreatorEv")
}

// 0x392738 — __ZNSt6vectorIN5boost8weak_ptrIN3RBX12PartInstanceEEESaIS4_EED2Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>::~vector()")]
#[doc(alias = "__ZNSt6vectorIN5boost8weak_ptrIN3RBX12PartInstanceEEESaIS4_EED2Ev")]
pub fn stub_0x392738() {
    // IDA 0x392738: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x392804 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_12AccoutrementES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEED1Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_12AccoutrementES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEED1Ev")]
pub fn stub_0x392804() {
    // IDA 0x392804: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x392830 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_12AccoutrementES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEED0Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_12AccoutrementES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEED0Ev")]
pub fn stub_0x392830() {
    // IDA 0x392830: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x392904 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_12AccoutrementES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_12AccoutrementES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_")]
pub fn stub_0x392904() -> ! {
    todo!("0x392904 rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)")
}

// 0x392920 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_12AccoutrementES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_12AccoutrementES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_")]
pub fn stub_0x392920() {
    // IDA 0x392920: non-virtual thunk to `rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::share` — this/arg-adjust + tail-call (this += 20) (decompiled). Rust uses static dispatch; no thunk needed. Target unmodeled: cutover no-op.
}

// 0x39293c — __ZN5boost3_bi5list2INS0_5valueIPN3RBX12AccoutrementEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::Accoutrement *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueIPN3RBX12AccoutrementEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0x39293c() -> ! {
    todo!("0x39293c void boost::_bi::list2<boost::_bi::value<RBX::Accoutrement *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&> &,int)")
}

// 0x392a14 — __ZNK5boost4_mfi3mf1IvN3RBX12AccoutrementENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>::operator()(RBX::Accoutrement*,boost::shared_ptr<RBX::Instance>)const")]
#[doc(alias = "__ZNK5boost4_mfi3mf1IvN3RBX12AccoutrementENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_")]
pub fn stub_0x392a14() -> ! {
    todo!("0x392a14 boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>::operator()(RBX::Accoutrement*,boost::shared_ptr<RBX::Instance>)const")
}

// 0x392afc — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_12AccoutrementES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_ED1Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_12AccoutrementES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_ED1Ev")]
pub fn stub_0x392afc() {
    // IDA 0x392afc: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x392b28 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_12AccoutrementES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_ED0Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_12AccoutrementES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_ED0Ev")]
pub fn stub_0x392b28() {
    // IDA 0x392b28: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x392bfc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX12AccoutrementENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueIPS8_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX12AccoutrementENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueIPS8_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE")]
pub fn stub_0x392bfc() -> ! {
    todo!("0x392bfc boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x392c5c — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX12AccoutrementENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueIPS8_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,void,boost::shared_ptr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX12AccoutrementENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueIPS8_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_")]
pub fn stub_0x392c5c() -> ! {
    todo!("0x392c5c boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,void,boost::shared_ptr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>)")
}

// 0x392c78 — __ZNK3RBX8Instance25findConstFirstChildOfTypeINS_12AccoutrementEEEPKT_v
#[doc(alias = "RBX::Accoutrement const* RBX::Instance::findConstFirstChildOfType<RBX::Accoutrement>(void)const")]
#[doc(alias = "__ZNK3RBX8Instance25findConstFirstChildOfTypeINS_12AccoutrementEEEPKT_v")]
pub fn stub_0x392c78() -> ! {
    todo!("0x392c78 RBX::Accoutrement const* RBX::Instance::findConstFirstChildOfType<RBX::Accoutrement>(void)const")
}

// 0x3935a0 — __GLOBAL__I_a_152
#[doc(alias = "global constructor keyed to_a_152")]
#[doc(alias = "__GLOBAL__I_a_152")]
pub fn stub_0x3935a0() -> ! {
    todo!("0x3935a0 global constructor keyed to _a_152")
}

// 0x393b34 — __ZN3RBX13PartAdornment10setAdorneeEPNS_12PartInstanceE
#[doc(alias = "RBX::PartAdornment::setAdornee(RBX::PartInstance *)")]
#[doc(alias = "__ZN3RBX13PartAdornment10setAdorneeEPNS_12PartInstanceE")]
pub fn stub_0x393b34() -> ! {
    todo!("0x393b34 RBX::PartAdornment::setAdornee(RBX::PartInstance *)")
}

// 0x393c44 — __ZN3RBX13PartAdornmentC2EPKc
#[doc(alias = "RBX::PartAdornment::PartAdornment(char const*)")]
#[doc(alias = "__ZN3RBX13PartAdornmentC2EPKc")]
pub fn stub_0x393c44() -> ! {
    todo!("0x393c44 RBX::PartAdornment::PartAdornment(char const*)")
}

// 0x393dd0 — __ZN3RBX11PVAdornment10setAdorneeEPNS_10PVInstanceE
#[doc(alias = "RBX::PVAdornment::setAdornee(RBX::PVInstance *)")]
#[doc(alias = "__ZN3RBX11PVAdornment10setAdorneeEPNS_10PVInstanceE")]
pub fn stub_0x393dd0() -> ! {
    todo!("0x393dd0 RBX::PVAdornment::setAdornee(RBX::PVInstance *)")
}

// 0x393ee0 — __ZN3RBX11PVAdornmentC2EPKc
#[doc(alias = "RBX::PVAdornment::PVAdornment(char const*)")]
#[doc(alias = "__ZN3RBX11PVAdornmentC2EPKc")]
pub fn stub_0x393ee0() -> ! {
    todo!("0x393ee0 RBX::PVAdornment::PVAdornment(char const*)")
}

// 0x39406c — __ZNK3RBX13PartAdornment19getAdorneeDangerousEv
#[doc(alias = "RBX::PartAdornment::getAdorneeDangerous(void)const")]
#[doc(alias = "__ZNK3RBX13PartAdornment19getAdorneeDangerousEv")]
pub fn stub_0x39406c() -> ! {
    todo!("0x39406c RBX::PartAdornment::getAdorneeDangerous(void)const")
}

// 0x3940bc — __ZNK3RBX11PVAdornment19getAdorneeDangerousEv
#[doc(alias = "RBX::PVAdornment::getAdorneeDangerous(void)const")]
#[doc(alias = "__ZNK3RBX11PVAdornment19getAdorneeDangerousEv")]
pub fn stub_0x3940bc() -> ! {
    todo!("0x3940bc RBX::PVAdornment::getAdorneeDangerous(void)const")
}

// 0x39410c — __ZN3RBX11shared_fromINS_10PVInstanceEEEN5boost10shared_ptrIT_EEPS4_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::PVInstance> RBX::shared_from<RBX::PVInstance>(RBX::PVInstance*)")]
#[doc(alias = "__ZN3RBX11shared_fromINS_10PVInstanceEEEN5boost10shared_ptrIT_EEPS4_")]
pub fn stub_0x39410c() -> ! {
    todo!("0x39410c boost::shared_ptr<RBX::PVInstance> RBX::shared_from<RBX::PVInstance>(RBX::PVInstance*)")
}

// 0x39427c — __ZN3RBX11PVAdornmentD1Ev
#[doc(alias = "RBX::PVAdornment::~PVAdornment()")]
#[doc(alias = "__ZN3RBX11PVAdornmentD1Ev")]
pub fn stub_0x39427c() {
    // IDA 0x39427c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3943c4 — __ZN3RBX11PVAdornmentD0Ev
#[doc(alias = "RBX::PVAdornment::~PVAdornment()")]
#[doc(alias = "__ZN3RBX11PVAdornmentD0Ev")]
pub fn stub_0x3943c4() {
    // IDA 0x3943c4: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x394464 — __ZNK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEE12getClassNameEv")]
pub fn stub_0x394464() -> ! {
    todo!("0x394464 __ZNK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEE12getClassNameEv")
}

// 0x39448c — __ZThn32_N3RBX11PVAdornmentD1Ev
#[doc(alias = "non-virtual thunk toRBX::PVAdornment::~PVAdornment()")]
#[doc(alias = "__ZThn32_N3RBX11PVAdornmentD1Ev")]
pub fn stub_0x39448c() {
    // IDA 0x39448c: __ZThn32 thunk (D1 base dtor): `this -= 32`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x3945d4 — __ZThn32_N3RBX11PVAdornmentD0Ev
#[doc(alias = "non-virtual thunk toRBX::PVAdornment::~PVAdornment()")]
#[doc(alias = "__ZThn32_N3RBX11PVAdornmentD0Ev")]
pub fn stub_0x3945d4() {
    // IDA 0x3945d4: __ZThn32 thunk (D0 deleting dtor): `this -= 32`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x394730 — __ZThn32_NK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEE12getClassNameEv")]
pub fn stub_0x394730() -> &'static str {
    // IDA 0x394730: __ZThn getClassName — `Creator = static_getCreator(); return Creator::getClassName_shim(Creator)` (decompiled 0x394730 family; e.g. 0x28e128). The Creator name is the class name. Rust: no vtable/Creator needed.
    "PVAdornment"
}

// 0x394758 — __ZThn36_N3RBX11PVAdornmentD1Ev
#[doc(alias = "non-virtual thunk toRBX::PVAdornment::~PVAdornment()")]
#[doc(alias = "__ZThn36_N3RBX11PVAdornmentD1Ev")]
pub fn stub_0x394758() {
    // IDA 0x394758: __ZThn36 thunk (D1 base dtor): `this -= 36`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x3948a0 — __ZThn36_N3RBX11PVAdornmentD0Ev
#[doc(alias = "non-virtual thunk toRBX::PVAdornment::~PVAdornment()")]
#[doc(alias = "__ZThn36_N3RBX11PVAdornmentD0Ev")]
pub fn stub_0x3948a0() {
    // IDA 0x3948a0: __ZThn36 thunk (D0 deleting dtor): `this -= 36`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3949fc — __ZN3RBX4Name13callDoDeclareILZNS_12sPVAdornmentEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sPVAdornmentEEEEvv")]
pub fn stub_0x3949fc() -> ! {
    todo!("0x3949fc __ZN3RBX4Name13callDoDeclareILZNS_12sPVAdornmentEEEEvv")
}

// 0x394a00 — __ZN3RBX4Name9doDeclareILZNS_12sPVAdornmentEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sPVAdornmentEEEERKS0_v")]
pub fn stub_0x394a00() -> ! {
    todo!("0x394a00 __ZN3RBX4Name9doDeclareILZNS_12sPVAdornmentEEEERKS0_v")
}

// 0x3961b4 — __ZN5boost10shared_ptrIN3RBX10PVInstanceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::PVInstance>::shared_ptr<RBX::PVInstance>(boost::weak_ptr<RBX::PVInstance> const&,boost::detail::sp_nothrow_tag)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10PVInstanceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")]
pub fn stub_0x3961b4() -> ! {
    todo!("0x3961b4 boost::shared_ptr<RBX::PVInstance>::shared_ptr<RBX::PVInstance>(boost::weak_ptr<RBX::PVInstance> const&,boost::detail::sp_nothrow_tag)")
}

// 0x396230 — __GLOBAL__I_a_153
#[doc(alias = "global constructor keyed to_a_153")]
#[doc(alias = "__GLOBAL__I_a_153")]
pub fn stub_0x396230() -> ! {
    todo!("0x396230 global constructor keyed to _a_153")
}

// 0x396554 — __ZN3RBX19AnimatableRootJoint13getParentNameEv
#[doc(alias = "RBX::AnimatableRootJoint::getParentName(void)")]
#[doc(alias = "__ZN3RBX19AnimatableRootJoint13getParentNameEv")]
pub fn stub_0x396554() -> ! {
    todo!("0x396554 RBX::AnimatableRootJoint::getParentName(void)")
}

// 0x396564 — __ZN3RBX19AnimatableRootJoint11getPartNameEv
#[doc(alias = "RBX::AnimatableRootJoint::getPartName(void)")]
#[doc(alias = "__ZN3RBX19AnimatableRootJoint11getPartNameEv")]
pub fn stub_0x396564() -> ! {
    todo!("0x396564 RBX::AnimatableRootJoint::getPartName(void)")
}

// 0x396574 — __ZN3RBX19AnimatableRootJoint9applyPoseERKNS_10CachedPoseE
#[doc(alias = "RBX::AnimatableRootJoint::applyPose(RBX::CachedPose const&)")]
#[doc(alias = "__ZN3RBX19AnimatableRootJoint9applyPoseERKNS_10CachedPoseE")]
pub fn stub_0x396574() -> ! {
    todo!("0x396574 RBX::AnimatableRootJoint::applyPose(RBX::CachedPose const&)")
}

// 0x39672c — __GLOBAL__I_a_154
#[doc(alias = "global constructor keyed to_a_154")]
#[doc(alias = "__GLOBAL__I_a_154")]
pub fn stub_0x39672c() -> ! {
    todo!("0x39672c global constructor keyed to _a_154")
}

// 0x39699c — __ZN3RBX9Animation10setAssetIdENS_11AnimationIdE
#[doc(alias = "RBX::Animation::setAssetId(RBX::AnimationId)")]
#[doc(alias = "__ZN3RBX9Animation10setAssetIdENS_11AnimationIdE")]
pub fn stub_0x39699c() -> ! {
    todo!("0x39699c RBX::Animation::setAssetId(RBX::AnimationId)")
}

// 0x3969d8 — __ZN3RBX9AnimationC1Ev
#[doc(alias = "RBX::Animation::Animation(void)")]
#[doc(alias = "__ZN3RBX9AnimationC1Ev")]
pub fn stub_0x3969d8() -> ! {
    todo!("0x3969d8 RBX::Animation::Animation(void)")
}

// 0x3969dc — __ZN3RBX9AnimationC2Ev
#[doc(alias = "RBX::Animation::Animation(void)")]
#[doc(alias = "__ZN3RBX9AnimationC2Ev")]
pub fn stub_0x3969dc() -> ! {
    todo!("0x3969dc RBX::Animation::Animation(void)")
}

// 0x396c00 — __ZNK3RBX9Animation15isEmbeddedAssetEv
#[doc(alias = "RBX::Animation::isEmbeddedAsset(void)const")]
#[doc(alias = "__ZNK3RBX9Animation15isEmbeddedAssetEv")]
pub fn stub_0x396c00() -> ! {
    todo!("0x396c00 RBX::Animation::isEmbeddedAsset(void)const")
}

// 0x396c40 — __ZNK3RBX9Animation19getKeyframeSequenceEPKNS_8InstanceE
#[doc(alias = "RBX::Animation::getKeyframeSequence(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX9Animation19getKeyframeSequenceEPKNS_8InstanceE")]
pub fn stub_0x396c40() -> ! {
    todo!("0x396c40 RBX::Animation::getKeyframeSequence(RBX::Instance const*)const")
}

// 0x396e08 — __ZNK3RBX9Animation10getAssetIdEv
#[doc(alias = "RBX::Animation::getAssetId(void)const")]
#[doc(alias = "__ZNK3RBX9Animation10getAssetIdEv")]
pub fn stub_0x396e08() -> ! {
    todo!("0x396e08 RBX::Animation::getAssetId(void)const")
}

// 0x396e44 — __ZN3RBX15ServiceProvider6createINS_24KeyframeSequenceProviderEEEPT_PKNS_8InstanceE
#[doc(alias = "RBX::KeyframeSequenceProvider * RBX::ServiceProvider::create<RBX::KeyframeSequenceProvider>(RBX::Instance const*)")]
#[doc(alias = "__ZN3RBX15ServiceProvider6createINS_24KeyframeSequenceProviderEEEPT_PKNS_8InstanceE")]
pub fn stub_0x396e44() -> ! {
    todo!("0x396e44 RBX::KeyframeSequenceProvider * RBX::ServiceProvider::create<RBX::KeyframeSequenceProvider>(RBX::Instance const*)")
}

// 0x396e5c — __ZN3RBX9AnimationD1Ev
#[doc(alias = "RBX::Animation::~Animation()")]
#[doc(alias = "__ZN3RBX9AnimationD1Ev")]
pub fn stub_0x396e5c() {
    // IDA 0x396e5c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x396f40 — __ZN3RBX9AnimationD0Ev
#[doc(alias = "RBX::Animation::~Animation()")]
#[doc(alias = "__ZN3RBX9AnimationD0Ev")]
pub fn stub_0x396f40() {
    // IDA 0x396f40: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x397038 — __ZNK3RBX9Animation21getPersistentDataCostEv
#[doc(alias = "RBX::Animation::getPersistentDataCost(void)const")]
#[doc(alias = "__ZNK3RBX9Animation21getPersistentDataCostEv")]
pub fn stub_0x397038() -> ! {
    todo!("0x397038 RBX::Animation::getPersistentDataCost(void)const")
}

// 0x3970bc — __ZNK3RBX9Animation12askSetParentEPKNS_8InstanceE
#[doc(alias = "RBX::Animation::askSetParent(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX9Animation12askSetParentEPKNS_8InstanceE")]
pub fn stub_0x3970bc() -> ! {
    todo!("0x3970bc RBX::Animation::askSetParent(RBX::Instance const*)const")
}

// 0x3970c0 — __ZNK3RBX14FactoryProductINS_9AnimationENS_8InstanceELZNS_10sAnimationEES2_E12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_9AnimationENS_8InstanceELZNS_10sAnimationEES2_E12getClassNameEv")]
pub fn stub_0x3970c0() -> ! {
    todo!("0x3970c0 __ZNK3RBX14FactoryProductINS_9AnimationENS_8InstanceELZNS_10sAnimationEES2_E12getClassNameEv")
}

// 0x3970d0 — __ZThn32_N3RBX9AnimationD1Ev
#[doc(alias = "non-virtual thunk toRBX::Animation::~Animation()")]
#[doc(alias = "__ZThn32_N3RBX9AnimationD1Ev")]
pub fn stub_0x3970d0() {
    // IDA 0x3970d0: __ZThn32 thunk (D1 base dtor): `this -= 32`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x3971b4 — __ZThn32_N3RBX9AnimationD0Ev
#[doc(alias = "non-virtual thunk toRBX::Animation::~Animation()")]
#[doc(alias = "__ZThn32_N3RBX9AnimationD0Ev")]
pub fn stub_0x3971b4() {
    // IDA 0x3971b4: __ZThn32 thunk (D0 deleting dtor): `this -= 32`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3972ac — __ZThn32_NK3RBX14FactoryProductINS_9AnimationENS_8InstanceELZNS_10sAnimationEES2_E12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_9AnimationENS_8InstanceELZNS_10sAnimationEES2_E12getClassNameEv")]
pub fn stub_0x3972ac() -> &'static str {
    // IDA 0x3972ac: __ZThn getClassName — `Creator = static_getCreator(); return Creator::getClassName_shim(Creator)` (decompiled 0x3972ac family; e.g. 0x28e128). The Creator name is the class name. Rust: no vtable/Creator needed.
    "Animation"
}

// 0x3972bc — __ZThn36_N3RBX9AnimationD1Ev
#[doc(alias = "non-virtual thunk toRBX::Animation::~Animation()")]
#[doc(alias = "__ZThn36_N3RBX9AnimationD1Ev")]
pub fn stub_0x3972bc() {
    // IDA 0x3972bc: __ZThn36 thunk (D1 base dtor): `this -= 36`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x3973a0 — __ZThn36_N3RBX9AnimationD0Ev
#[doc(alias = "non-virtual thunk toRBX::Animation::~Animation()")]
#[doc(alias = "__ZThn36_N3RBX9AnimationD0Ev")]
pub fn stub_0x3973a0() {
    // IDA 0x3973a0: __ZThn36 thunk (D0 deleting dtor): `this -= 36`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x397498 — __ZN3RBX14FactoryProductINS_9AnimationENS_8InstanceELZNS_10sAnimationEES2_E17static_getCreatorEv
#[doc(alias = "__ZN3RBX14FactoryProductINS_9AnimationENS_8InstanceELZNS_10sAnimationEES2_E17static_getCreatorEv")]
pub fn stub_0x397498() -> ! {
    todo!("0x397498 __ZN3RBX14FactoryProductINS_9AnimationENS_8InstanceELZNS_10sAnimationEES2_E17static_getCreatorEv")
}

// 0x397f88 — __ZN3rbx8any_castIRKN3RBX11AnimationIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::AnimationId const& rbx::any_cast<RBX::AnimationId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX11AnimationIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0x397f88() -> ! {
    todo!("0x397f88 RBX::AnimationId const& rbx::any_cast<RBX::AnimationId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x398240 — __GLOBAL__I_a_155
#[doc(alias = "global constructor keyed to_a_155")]
#[doc(alias = "__GLOBAL__I_a_155")]
pub fn stub_0x398240() -> ! {
    todo!("0x398240 global constructor keyed to _a_155")
}

// 0x398554 — __ZN3RBX14AnimationTrack4playEfff
#[doc(alias = "RBX::AnimationTrack::play(float,float,float)")]
#[doc(alias = "__ZN3RBX14AnimationTrack4playEfff")]
pub fn stub_0x398554() -> ! {
    todo!("0x398554 RBX::AnimationTrack::play(float,float,float)")
}

// 0x398694 — __ZN3RBX14AnimationTrack4stopEf
#[doc(alias = "RBX::AnimationTrack::stop(float)")]
#[doc(alias = "__ZN3RBX14AnimationTrack4stopEf")]
pub fn stub_0x398694() -> ! {
    todo!("0x398694 RBX::AnimationTrack::stop(float)")
}

// 0x39869c — __ZN3RBX14AnimationTrack12adjustWeightEff
#[doc(alias = "RBX::AnimationTrack::adjustWeight(float,float)")]
#[doc(alias = "__ZN3RBX14AnimationTrack12adjustWeightEff")]
pub fn stub_0x39869c() -> ! {
    todo!("0x39869c RBX::AnimationTrack::adjustWeight(float,float)")
}

// 0x3987cc — __ZN3RBX14AnimationTrack11adjustSpeedEf
#[doc(alias = "RBX::AnimationTrack::adjustSpeed(float)")]
#[doc(alias = "__ZN3RBX14AnimationTrack11adjustSpeedEf")]
pub fn stub_0x3987cc() -> ! {
    todo!("0x3987cc RBX::AnimationTrack::adjustSpeed(float)")
}

// 0x3988f0 — __ZN3RBX14AnimationTrackC1EN5boost10shared_ptrINS_19AnimationTrackStateEEENS1_8weak_ptrINS_8AnimatorEEE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::AnimationTrack::AnimationTrack(boost::shared_ptr<RBX::AnimationTrackState>,boost::weak_ptr<RBX::Animator>)")]
#[doc(alias = "__ZN3RBX14AnimationTrackC1EN5boost10shared_ptrINS_19AnimationTrackStateEEENS1_8weak_ptrINS_8AnimatorEEE")]
pub fn stub_0x3988f0() -> ! {
    todo!("0x3988f0 RBX::AnimationTrack::AnimationTrack(boost::shared_ptr<RBX::AnimationTrackState>,boost::weak_ptr<RBX::Animator>)")
}

// 0x3988f4 — __ZN3RBX14AnimationTrackC2EN5boost10shared_ptrINS_19AnimationTrackStateEEENS1_8weak_ptrINS_8AnimatorEEE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::AnimationTrack::AnimationTrack(boost::shared_ptr<RBX::AnimationTrackState>,boost::weak_ptr<RBX::Animator>)")]
#[doc(alias = "__ZN3RBX14AnimationTrackC2EN5boost10shared_ptrINS_19AnimationTrackStateEEENS1_8weak_ptrINS_8AnimatorEEE")]
pub fn stub_0x3988f4() -> ! {
    todo!("0x3988f4 RBX::AnimationTrack::AnimationTrack(boost::shared_ptr<RBX::AnimationTrackState>,boost::weak_ptr<RBX::Animator>)")
}

// 0x398d64 — __ZN3RBX14AnimationTrack22forwardKeyframeReachedESs
#[doc(alias = "RBX::AnimationTrack::forwardKeyframeReached(std::string)")]
#[doc(alias = "__ZN3RBX14AnimationTrack22forwardKeyframeReachedESs")]
pub fn stub_0x398d64() -> ! {
    todo!("0x398d64 RBX::AnimationTrack::forwardKeyframeReached(std::string)")
}

// 0x398e80 — __ZN3RBX14AnimationTrackD0Ev
#[doc(alias = "RBX::AnimationTrack::~AnimationTrack()")]
#[doc(alias = "__ZN3RBX14AnimationTrackD0Ev")]
pub fn stub_0x398e80() {
    // IDA 0x398e80: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x398f20 — __ZN3RBX14AnimationTrackD1Ev
#[doc(alias = "RBX::AnimationTrack::~AnimationTrack()")]
#[doc(alias = "__ZN3RBX14AnimationTrackD1Ev")]
pub fn stub_0x398f20() {
    // IDA 0x398f20: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x398f24 — __ZThn32_N3RBX14AnimationTrackD0Ev
#[doc(alias = "non-virtual thunk toRBX::AnimationTrack::~AnimationTrack()")]
#[doc(alias = "__ZThn32_N3RBX14AnimationTrackD0Ev")]
pub fn stub_0x398f24() {
    // IDA 0x398f24: __ZThn32 thunk (D0 deleting dtor): `this -= 32`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x398f2c — __ZThn36_N3RBX14AnimationTrackD0Ev
#[doc(alias = "non-virtual thunk toRBX::AnimationTrack::~AnimationTrack()")]
#[doc(alias = "__ZThn36_N3RBX14AnimationTrackD0Ev")]
pub fn stub_0x398f2c() {
    // IDA 0x398f2c: __ZThn36 thunk (D0 deleting dtor): `this -= 36`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x398f34 — __ZN3RBX14AnimationTrackD2Ev
#[doc(alias = "RBX::AnimationTrack::~AnimationTrack()")]
#[doc(alias = "__ZN3RBX14AnimationTrackD2Ev")]
pub fn stub_0x398f34() {
    // IDA 0x398f34: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x3991a8 — __ZThn32_N3RBX14AnimationTrackD1Ev
#[doc(alias = "non-virtual thunk toRBX::AnimationTrack::~AnimationTrack()")]
#[doc(alias = "__ZThn32_N3RBX14AnimationTrackD1Ev")]
pub fn stub_0x3991a8() {
    // IDA 0x3991a8: __ZThn32 thunk (D1 base dtor): `this -= 32`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x3991b0 — __ZThn36_N3RBX14AnimationTrackD1Ev
#[doc(alias = "non-virtual thunk toRBX::AnimationTrack::~AnimationTrack()")]
#[doc(alias = "__ZThn36_N3RBX14AnimationTrackD1Ev")]
pub fn stub_0x3991b0() {
    // IDA 0x3991b0: __ZThn36 thunk (D1 base dtor): `this -= 36`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x3992dc — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEE12getClassNameEv")]
pub fn stub_0x3992dc() -> ! {
    todo!("0x3992dc __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEE12getClassNameEv")
}
