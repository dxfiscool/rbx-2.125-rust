// Auto-generated shard B — next 120 RBX::Reflection stubs — EA-sorted, offset +120 from shard A (alternative EA window)
// Source: ida/export.json filtered demangled contains RBX::Reflection (16171 total)
// Format: // 0xADDR — mangled + #[doc(alias = "RBX::...")] + todo!("0xADDR") using rbx_core::SharedPtr
#![allow(unused_imports)]
use rbx_core::SharedPtr;
use rbx_core::signal::Signal;

/// `RBX::StarterGuiService::CoreGuiType` as seen through reflection: an int-backed enum
/// (IDA 0x5fd070 `genericConvert<CoreGuiType>`, 0x601012 `Type::getSingleton<CoreGuiType>`).
pub type CoreGuiType = i32;

/// Signature argument kinds of `EventDesc<StarterGuiService, void(CoreGuiType, bool)>`
/// (IDA 0x601012 `Type::getSingleton<CoreGuiType>`, 0x60104e `getSingleton<bool>`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoreGuiChangedArg {
    CoreGuiType,
    Bool,
}

/// `RBX::Reflection::EventDesc<StarterGuiService, void(CoreGuiType, bool), ...>`
/// (IDA 0x600f64): base `EventDescriptor` init, member-signal pointer at +40, two-item
/// signature list appended. Mirrors `crate::descriptor::ExplosionEventDesc`.
#[derive(Debug, Clone)]
pub struct CoreGuiChangedEventDesc {
    pub name: String,
    pub category: String,
    pub title: String,
    pub member: usize,
    pub signature: Vec<(String, CoreGuiChangedArg)>,
    pub permissions: u32,
    pub attributes: u32,
}

/// `RBX::Reflection::GenericSlotWrapper` for `(CoreGuiType, bool)` slots. Wraps one generic
/// slot; `execute2` packs the args into a 2-Variant vector and dispatches the stored callable
/// (IDA 0x6016a0: vector fill, `vfptr+8` call, vector teardown). Mirrors
/// `crate::descriptor::GenericSlotWrapper`, which is fixed to `(Instance, f32)`.
pub struct CoreGuiSlotWrapper {
    pub invoke: Box<dyn Fn(&[crate::descriptor::Variant]) + Send + Sync>,
}

impl CoreGuiSlotWrapper {
    pub fn execute2(&self, value: CoreGuiType, flag: bool) {
        // IDA 0x6016a0: `vector<Variant>{ (CoreGuiType, arg0), (bool, arg1) }`, virtual
        // dispatch into the wrapped slot, then destroy the vector.
        (self.invoke)(&[
            crate::descriptor::Variant::Int(value),
            crate::descriptor::Variant::Bool(flag),
        ]);
    }
}

/// Connected slot: the original signal owns its slots until `disconnectAll`; the strong
/// refs live in `holders` because `Signal::connect` keeps only weak refs.
type CoreGuiSlot = SharedPtr<dyn Fn((CoreGuiType, bool)) + Send + Sync>;

/// `RBX::Reflection::EventSource` for the `CoreGuiChanged` signal: owns the connected slots.
/// Backed by `rbx_core::signal::Signal` (IDA 0x6012a8/0x6013f8).
#[derive(Default)]
pub struct CoreGuiEventSource {
    signal: Signal<(CoreGuiType, bool)>,
    holders: parking_lot::Mutex<Vec<(SharedPtr<CoreGuiSlotWrapper>, CoreGuiSlot)>>,
}

impl CoreGuiEventSource {
    pub fn connect_slot(&self, wrapper: SharedPtr<CoreGuiSlotWrapper>) {
        let w = SharedPtr::clone(&wrapper);
        let slot = std::sync::Arc::new(move |payload: (CoreGuiType, bool)| {
            w.execute2(payload.0, payload.1);
        });
        self.signal.connect(SharedPtr::clone(&slot));
        let slot: CoreGuiSlot = slot;
        self.holders.lock().push((wrapper, slot));
    }

    pub fn fire(&self, value: CoreGuiType, flag: bool) {
        self.signal.fire((value, flag));
    }

    pub fn disconnect_all(&self) {
        self.holders.lock().clear();
        self.signal.disconnect_all();
    }
}

/// `boost::_bi::bind_t<mf2<GenericSlotWrapper, CoreGuiType, bool>, list3<value<SharedPtr<GenericSlotWrapper>>, arg<1>, arg<2>>>`
/// (IDA 0x601584): stores the member-function triple plus the bound wrapper and the two
/// placeholders. The member function is fixed (`execute2`), so the triple folds into the target.
#[derive(Clone)]
pub struct BoundCoreGuiSlot {
    pub target: SharedPtr<CoreGuiSlotWrapper>,
}

impl BoundCoreGuiSlot {
    /// `bind_t::operator()<CoreGuiType, bool>` (IDA 0x601dd0): member-pointer dispatch
    /// `(target->*mf)(args)`. The `(v1 & 1)` virtual-adjust branch is member-pointer
    /// mechanics with no Rust equivalent.
    pub fn call(&self, value: CoreGuiType, flag: bool) {
        self.target.execute2(value, flag);
    }
}

/// `boost::function2<void, CoreGuiType, bool>` holding one bound slot
/// (IDA 0x601a00/0x601b30/0x601c18/0x601cfc).
#[derive(Default, Clone)]
pub struct CoreGuiSlotFunction {
    bound: Option<BoundCoreGuiSlot>,
}

impl CoreGuiSlotFunction {
    pub fn is_empty(&self) -> bool {
        self.bound.is_none()
    }

    pub fn invoke(&self, value: CoreGuiType, flag: bool) {
        // Calling an empty `boost::function` throws `bad_function_call`; panic mirrors it.
        self.bound
            .as_ref()
            .expect("bad_function_call")
            .call(value, flag);
    }
}

/// typeinfo name compared by `manager` case 3 (IDA 0x601dec `strcmp` literal).
pub const COREGUI_BIND_T_TYPEINFO: &str = "N5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS4_17StarterGuiService11CoreGuiTypeERKbEENS0_5list3INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEENSJ_ILi2EEEEEEE";

/// `RBX::Reflection::PropDescriptor<RBX::CoreGuiService, int>` access pair: the GetImpl
/// member descriptor holds the const getter plus the default value (IDA 0x5ffc78
/// `new(0xc)` at 0x5ffca4-0x5fcccc).
pub struct CoreGuiServiceIntAccess {
    pub get: Box<dyn Fn() -> i32 + Send + Sync>,
    pub default: i32,
}

/// `RBX::Reflection::PropDescriptor<RBX::CoreGuiService, int>` (IDA 0x5ffc78).
pub struct CoreGuiServiceIntPropDesc {
    pub name: String,
    pub category: String,
    pub access: CoreGuiServiceIntAccess,
    pub attributes: u32,
    pub permissions: u32,
}

/// One `SignatureDescriptor` item of a `BoundFuncDesc`: argument name + type name
/// (IDA 0x602e68 `Name::declare` + `Type::getSingleton<CoreGuiType>` + `addArgument`).
#[derive(Debug, Clone)]
pub struct CoreGuiBoundFuncSigItem {
    pub name: String,
    pub type_name: &'static str,
}

/// `RBX::Reflection::BoundFuncDesc<RBX::StarterGuiService, ...>` (IDA 0x602cf0/0x6031c8):
/// base `FunctionDescriptor` init, member-function pair at +40, void-typed defaults staged,
/// `declareSignature()` fixing the return type and argument list.
#[derive(Debug, Clone)]
pub struct CoreGuiBoundFuncDesc {
    pub name: String,
    pub category: String,
    pub member: (usize, usize),
    pub return_type: &'static str,
    pub args: Vec<CoreGuiBoundFuncSigItem>,
    pub permissions: u32,
    pub attributes: u32,
}

// 0x5fccec — __ZN3RBX10Reflection9EventDescINS_17StarterGuiServiceEFvNS2_11CoreGuiTypeEbEN3rbx6signalIS4_EEMS2_S7_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::StarterGuiService,void ()(RBX::StarterGuiService::CoreGuiType,bool),rbx::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>,rbx::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)> RBX::StarterGuiService::*>::~EventDesc()")]
pub fn stub_0x5fccec() {
    // IDA 0x5fccec: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x5fcd10 — __ZN3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::StarterGuiService::CoreGuiType>::addPair(RBX::StarterGuiService::CoreGuiType,char const*)")]
pub fn stub_0x5fcd10(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0x5fcd10: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0x5fd070 — __ZN3RBX10Reflection7Variant14genericConvertINS_17StarterGuiService11CoreGuiTypeEEERT_v
#[doc(alias = "RBX::StarterGuiService::CoreGuiType & RBX::Reflection::Variant::genericConvert<RBX::StarterGuiService::CoreGuiType>(void)")]
pub fn stub_0x5fd070(variant: &crate::descriptor::Variant) -> CoreGuiType {
    // IDA 0x5fd070: `any_cast<CoreGuiType>` direct hit returns as-is (0x5fd09a/0x5fd168);
    // else the variant must hold a string ("Ss" typeinfo check, 0x5fd0d4-0x5fd0ea),
    // converted via `StringConverter<CoreGuiType>::convertToValue` (0x5fd10c) into a
    // placement_any + type-singleton store (0x5fd136-0x5fd14a). Anything else throws
    // `runtime_error("Unable to cast %s to %s")` (0x5fd198-0x5fd1f0); panic mirrors it.
    match variant {
        crate::descriptor::Variant::Int(v) => *v,
        crate::descriptor::Variant::Text(s) => {
            match crate::generated::stub_0x4b3cb8().lookup_value(s) {
                Some(v) => v,
                None => panic!("Unable to cast {s} to CoreGuiType (IDA 0x5fd070)"),
            }
        }
        _ => panic!("Unable to cast non-string variant to CoreGuiType (IDA 0x5fd070)"),
    }
}

// 0x5fd3ac — __ZN3RBX10Reflection14PropDescriptorINS_14CoreGuiServiceEiED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::CoreGuiService,int>::~PropDescriptor()")]
pub fn stub_0x5fd3ac() {
    // IDA 0x5fd3ac: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x5fe180 — __ZN3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::StarterGuiService::CoreGuiType>::~EnumDesc()")]
pub fn stub_0x5fe180() {
    // IDA 0x5fe180: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x5fe220 — __ZNK3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::StarterGuiService::CoreGuiType>::lookup(char const*)const")]
pub fn stub_0x5fe220(desc: &crate::enum_desc::EnumDesc, name: &str) -> usize {
    // IDA 0x5fe220: EnumDesc<T>::lookup(char const*) -- Name::lookup intern, search name_to_value then legacy_names; hit: return convertToItem(value); miss: return 0 (decompiled 0x957a18).
    desc.lookup_value(name).and_then(|v| usize::try_from(v).ok()).and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0x5fe250 — __ZNK3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::StarterGuiService::CoreGuiType>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_0x5fe250(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0x5fe250: EnumDesc<T>::lookup(Variant) -- rbx::any_cast<T> the payload, then convertToItem (decompiled 0xb97c). Variant is unmodeled in this crate; the caller passes the already-cast enum value, and this is convertToItem exactly.
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0x5fe270 — __ZNK3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::StarterGuiService::CoreGuiType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_0x5fe270() {
    // IDA 0x5fe270: EnumDesc<T>::convertToValue(index, Variant&) -- writes the converted value into a Variant out-param; Variant is unmodeled in this crate: cutover no-op. See the (desc, name, &mut i32) sibling for the lookup semantics.
}

// 0x5fe2a8 — __ZNK3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::StarterGuiService::CoreGuiType>::convertToString(RBX::StarterGuiService::CoreGuiType const&)const")]
pub fn stub_0x5fe2a8(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0x5fe2a8: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0x5fe600 — __ZNK3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::StarterGuiService::CoreGuiType>::convertToValue(RBX::Name const&,RBX::StarterGuiService::CoreGuiType&)const")]
pub fn stub_0x5fe600(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0x5fe600: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0x5fea60 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9PlayerGuiES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::PlayerGui,RBX::PlayerGui>(rbx_core::SharedPtr<RBX::PlayerGui> const*,RBX::PlayerGui *)const")]
pub fn stub_0x5fea60() {
    // IDA 0x5fea60: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0x5ff548 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9TextLabelES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TextLabel,RBX::TextLabel>(rbx_core::SharedPtr<RBX::TextLabel> const*,RBX::TextLabel *)const")]
pub fn stub_0x5ff548() {
    // IDA 0x5ff548: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0x5ff844 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9ScreenGuiES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ScreenGui,RBX::ScreenGui>(rbx_core::SharedPtr<RBX::ScreenGui> const*,RBX::ScreenGui *)const")]
pub fn stub_0x5ff844() {
    // IDA 0x5ff844: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0x5ffc78 — __ZN3RBX10Reflection14PropDescriptorINS_14CoreGuiServiceEiEC2IMS2_KFivEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::CoreGuiService,int>::PropDescriptor<int (RBX::CoreGuiService::*)(void)const,int>(char const*,char const*,int (RBX::CoreGuiService::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x5ffc78(
    name: &str,
    category: &str,
    get: Box<dyn Fn() -> i32 + Send + Sync>,
    default: i32,
    attributes: u32,
    permissions: u32,
) -> CoreGuiServiceIntPropDesc {
    // IDA 0x5ffc78: base `Described<CoreGuiService>::classDescriptor` (0x5ffc9e),
    // `new(0xc)` GetImpl member desc holding (getter, default) (0x5ffca4-0x5fcccc),
    // `TypedPropertyDescriptor<int>` init (0x5ffd16), temp release (0x5ffd1e-0x5ffd20),
    // vtable off_1271D68 (0x5ffd34).
    CoreGuiServiceIntPropDesc {
        name: name.to_owned(),
        category: category.to_owned(),
        access: CoreGuiServiceIntAccess { get, default },
        attributes,
        permissions,
    }
}

// 0x5ffd88 — __ZN3RBX10Reflection14PropDescriptorINS_14CoreGuiServiceEiED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::CoreGuiService,int>::~PropDescriptor()")]
pub fn stub_0x5ffd88() {
    // IDA 0x5ffd88: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x5ffdb4 — __ZNK3RBX10Reflection14PropDescriptorINS_14CoreGuiServiceEiE7GetImplIMS2_KFivEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::CoreGuiService,int>::GetImpl<int (RBX::CoreGuiService::*)(void)const>::isReadOnly(void)const")]
pub fn stub_0x5ffdb4() {
    // IDA 0x5ffdb4: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x5ffdb8 — __ZNK3RBX10Reflection14PropDescriptorINS_14CoreGuiServiceEiE7GetImplIMS2_KFivEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::CoreGuiService,int>::GetImpl<int (RBX::CoreGuiService::*)(void)const>::isWriteOnly(void)const")]
pub fn stub_0x5ffdb8() {
    // IDA 0x5ffdb8: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x5ffdbc — __ZNK3RBX10Reflection14PropDescriptorINS_14CoreGuiServiceEiE7GetImplIMS2_KFivEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::CoreGuiService,int>::GetImpl<int (RBX::CoreGuiService::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x5ffdbc(getter: &dyn Fn() -> i32) -> i32 {
    // IDA 0x5ffdbc: member-pointer dispatch out of the GetImpl pair: adjust the instance
    // (`a2 ? a2 - 36 : 0`, 0x5ffdc0-0x5ffdc2), load member fn + encoded adjust
    // (0x5ffdc6-0x5ffdce), virtual-adjust when the low bit is set (0x5ffdd2-0x5ffdd6),
    // call through (0x5ffdd6). The adjust is member-pointer mechanics over an unmodeled
    // instance; the call is the observable effect.
    getter()
}

// 0x5ffddc — __ZNK3RBX10Reflection14PropDescriptorINS_14CoreGuiServiceEiE7GetImplIMS2_KFivEE8setValueEPNS0_13DescribedBaseERKi
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::CoreGuiService,int>::GetImpl<int (RBX::CoreGuiService::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
pub fn stub_0x5ffddc() -> ! {
    // IDA 0x5ffddc (`__noreturn`): get-only binding -- unconditionally builds
    // `runtime_error("can't set value")` (0x5ffe3c-0x5ffec4) and `__cxa_throw`s it
    // (0x5ffeec); panic mirrors the throw.
    panic!("can't set value (IDA 0x5ffddc)")
}

// 0x600910 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_17StarterGuiService11CoreGuiTypeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::StarterGuiService::CoreGuiType> const>::initSingleton(void)")]
pub fn stub_0x600910() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0x600910: Singleton<EnumDesc<T>>::initSingleton -- thunk to doGetSingleton (decompiled 0x4a60b8). Rust: forward to the singleton.
    crate::generated::stub_0x4b3cb8()
}

// 0x600f64 — __ZN3RBX10Reflection9EventDescINS_17StarterGuiServiceEFvNS2_11CoreGuiTypeEbEN3rbx6signalIS4_EEMS2_S7_EC2ES8_PKcSB_SB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::StarterGuiService,void ()(RBX::StarterGuiService::CoreGuiType,bool),rbx::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>,rbx::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)> RBX::StarterGuiService::*>::EventDesc(rbx::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)> RBX::StarterGuiService::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x600f64(
    member: usize,
    name: &str,
    category: &str,
    title: &str,
    permissions: u32,
    attributes: u32,
) -> CoreGuiChangedEventDesc {
    // IDA 0x600f64: base `EventDescriptor` init (0x600fbe), member-signal pointer at +40
    // (0x600fe2), vtable off_1271F08 (0x600fe6), then two signature items appended:
    // `(arg0_name, CoreGuiType)` (0x60100c-0x60103a) and `(arg1_name, bool)`
    // (0x601048-0x601072). As in stub_0x4a38b8, the declared names are the
    // category/title strings.
    CoreGuiChangedEventDesc {
        name: name.to_owned(),
        category: category.to_owned(),
        title: title.to_owned(),
        member,
        signature: vec![
            (category.to_owned(), CoreGuiChangedArg::CoreGuiType),
            (title.to_owned(), CoreGuiChangedArg::Bool),
        ],
        permissions,
        attributes,
    }
}

// 0x601154 — __ZN3RBX10Reflection9EventDescINS_17StarterGuiServiceEFvNS2_11CoreGuiTypeEbEN3rbx6signalIS4_EEMS2_S7_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::StarterGuiService,void ()(RBX::StarterGuiService::CoreGuiType,bool),rbx::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>,rbx::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)> RBX::StarterGuiService::*>::~EventDesc()")]
pub fn stub_0x601154() {
    // IDA 0x601154: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x601208 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_17StarterGuiServiceEFvNS2_11CoreGuiTypeEbEN3rbx6signalIS4_EEMS2_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::StarterGuiService,void ()(RBX::StarterGuiService::CoreGuiType,bool),rbx::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>,rbx::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)> RBX::StarterGuiService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_0x601208(source: Option<&CoreGuiEventSource>, wrapper: SharedPtr<CoreGuiSlotWrapper>) {
    // IDA 0x601208: builds `bind(execute2<CoreGuiType, bool>, wrapper, _1, _2)` (0x601280),
    // wraps it in a `boost::function` (0x60128c), then `signal::connect(member-signal-of-source,
    // fn)` (0x6012a8). Null member stores an empty connection (`*v44 = 0`, 0x6012b2); the
    // temp is cleared (0x6012ba) and the shared counts released (0x6012c0-0x6012d4, drop glue).
    if let Some(source) = source {
        source.connect_slot(wrapper);
    }
    // `function2::clear()` (0x6012ba) drops the temp; `Arc` drop glue covers it.
}

// 0x60135c — __ZNK3RBX10Reflection13EventDescImplILi2ENS_17StarterGuiServiceEFvNS2_11CoreGuiTypeEbEN3rbx6signalIS4_EEMS2_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::StarterGuiService,void ()(RBX::StarterGuiService::CoreGuiType,bool),rbx::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>,rbx::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)> RBX::StarterGuiService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x60135c(source: &CoreGuiEventSource, args: &[crate::descriptor::Variant]) {
    // IDA 0x60135c: `ReleaseAssert(args.size() == 2)` (Event.h:349, 0x601390-0x6013b0, gated
    // by FLog::Asserts + _debugHook at 0x601376-0x6013ae), then `any_cast<CoreGuiType>(args[0])`
    // (0x6013da), `any_cast<bool>(args[1])` (0x6013e2), and `signal_with_args<2>::operator()`.
    assert!(args.len() == 2, "args.size() == 2 include/Reflection/Event.h:349");
    let crate::descriptor::Variant::Int(value) = &args[0] else {
        panic!("any_cast<CoreGuiType> failed (IDA 0x6013da)");
    };
    let crate::descriptor::Variant::Bool(flag) = &args[1] else {
        panic!("any_cast<bool> failed (IDA 0x6013e2)");
    };
    source.fire(*value, *flag);
}

// 0x6013f8 — __ZNK3RBX10Reflection13EventDescBaseINS_17StarterGuiServiceEFvNS2_11CoreGuiTypeEbEN3rbx6signalIS4_EEMS2_S7_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::StarterGuiService,void ()(RBX::StarterGuiService::CoreGuiType,bool),rbx::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>,rbx::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)> RBX::StarterGuiService::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x6013f8(source: &CoreGuiEventSource) {
    // IDA 0x6013f8: member-offset adjust (`a2 ? a2 - 36 : 0`, 0x6013fc-0x6013fe), then
    // `signal::disconnectAll(member)`. The adjust is member-pointer mechanics; the
    // observable effect is dropping every slot.
    source.disconnect_all();
}

// 0x601584 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS1_17StarterGuiService11CoreGuiTypeERKbNS_10shared_ptrIS3_EENS_3argILi1EEENSC_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISH_T0_T1_T2_EENSF_9list_av_3IT3_T4_T5_E4typeEEEMSK_FSH_SL_SM_ESP_SQ_SR_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::StarterGuiService::CoreGuiType const&,bool const&>,boost::_bi::list_av_3<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::StarterGuiService::CoreGuiType const&,bool const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::StarterGuiService::CoreGuiType const&,bool const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")]
pub fn stub_0x601584(wrapper: SharedPtr<CoreGuiSlotWrapper>) -> BoundCoreGuiSlot {
    // IDA 0x601584: `list3(value(wrapper-shared), arg<1>, arg<2>)` (0x6015ee) plus the
    // member-function triple stored into the bind_t out (0x6015f6-0x601614); the shared
    // counts are released (0x601618-0x60162c, drop glue). The member function is fixed
    // (`execute2`), so the triple folds into the bound target.
    BoundCoreGuiSlot { target: wrapper }
}

// 0x6016a0 — __ZN3RBX10Reflection18GenericSlotWrapper8execute2INS_17StarterGuiService11CoreGuiTypeEbEEvRKT_RKT0_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute2<RBX::StarterGuiService::CoreGuiType,bool>(RBX::StarterGuiService::CoreGuiType const&,bool const&)")]
pub fn stub_0x6016a0(wrapper: &CoreGuiSlotWrapper, value: CoreGuiType, flag: bool) {
    // IDA 0x6016a0: packs `vector<Variant>{ (CoreGuiType, arg0), (bool, arg1) }`
    // (0x601714-0x601762), dispatches the wrapped slot (`vfptr+8`, 0x601772), destroys
    // the vector (0x60177c, drop glue).
    wrapper.execute2(value, flag);
}

// 0x601a00 — __ZN5boost9function2IvN3RBX17StarterGuiService11CoreGuiTypeEbE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS3_RKbEENS6_5list3INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEvT_
#[doc(alias = "void boost::function2<void,RBX::StarterGuiService::CoreGuiType,bool>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::StarterGuiService::CoreGuiType const&,bool const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::StarterGuiService::CoreGuiType const&,bool const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")]
pub fn stub_0x601a00(func: &mut CoreGuiSlotFunction, bound: &BoundCoreGuiSlot) {
    // IDA 0x601a00: copies the bind_t triple plus shared count into a temp (0x601a24-0x601a38),
    // delegates to `basic_vtable2::assign_to(stored_vtable, tmp, buf)` (0x601a88), releases the
    // temp (0x601a8e-0x601a96), stores the vtable (`*v25 = v23`, 0x601a9e). Net effect: the
    // function object owns a clone of the functor.
    stub_0x601b30(func, bound);
}

// 0x601af8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_17StarterGuiService11CoreGuiTypeERKbEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::StarterGuiService::CoreGuiType const&,bool const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x601af8(
    op: crate::descriptor::FunctorOp,
    src: &BoundCoreGuiSlot,
    slot: &mut Option<Box<BoundCoreGuiSlot>>,
) -> &'static str {
    // IDA 0x601af8: any op but 4 delegates to `manager()` (0x601afa-0x601afc); op 4 answers
    // the bind_t typeinfo without touching the buffers (0x601b0e-0x601b12). Either way the
    // call reports the functor type.
    if op != crate::descriptor::FunctorOp::GetFunctorTypeInfo {
        stub_0x601dec(op, src, slot);
    }
    COREGUI_BIND_T_TYPEINFO
}

// 0x601b14 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_17StarterGuiService11CoreGuiTypeERKbEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEvSB_bE6invokeERNS1_15function_bufferESB_b
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::StarterGuiService::CoreGuiType const&,bool const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,RBX::StarterGuiService::CoreGuiType,bool>::invoke(boost::detail::function::function_buffer &,RBX::StarterGuiService::CoreGuiType,bool)")]
pub fn stub_0x601b14(bound: &BoundCoreGuiSlot, value: CoreGuiType, flag: bool) {
    // IDA 0x601b14: tail-jumps to `bind_t::operator()<CoreGuiType, bool>` (0x601b2c).
    stub_0x601dd0(bound, value, flag);
}

// 0x601b30 — __ZNK5boost6detail8function13basic_vtable2IvN3RBX17StarterGuiService11CoreGuiTypeEbE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_RKbEENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,RBX::StarterGuiService::CoreGuiType,bool>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::StarterGuiService::CoreGuiType const&,bool const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::StarterGuiService::CoreGuiType const&,bool const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x601b30(func: &mut CoreGuiSlotFunction, bound: &BoundCoreGuiSlot) -> bool {
    // IDA 0x601b30: copies the functor triple (0x601b50-0x601b6a), delegates to the
    // tag-dispatch overload (0x601bae), releases the temp (0x601bb4-0x601bbc),
    // returns 1 (0x601bdc).
    stub_0x601c18(func, bound)
}

// 0x601c18 — __ZNK5boost6detail8function13basic_vtable2IvN3RBX17StarterGuiService11CoreGuiTypeEbE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_RKbEENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,RBX::StarterGuiService::CoreGuiType,bool>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::StarterGuiService::CoreGuiType const&,bool const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::StarterGuiService::CoreGuiType const&,bool const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x601c18(func: &mut CoreGuiSlotFunction, bound: &BoundCoreGuiSlot) -> bool {
    // IDA 0x601c18: copies the functor triple + shared count (0x601c38-0x601c66),
    // heap-clones it via `assign_functor` (0x601c90), releases the temp (0x601c96-0x601c9e),
    // returns 1 (0x601cbe).
    stub_0x601cfc(func, bound);
    true
}

// 0x601cfc — __ZNK5boost6detail8function13basic_vtable2IvN3RBX17StarterGuiService11CoreGuiTypeEbE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_RKbEENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable2<void,RBX::StarterGuiService::CoreGuiType,bool>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::StarterGuiService::CoreGuiType const&,bool const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::StarterGuiService::CoreGuiType const&,bool const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x601cfc(func: &mut CoreGuiSlotFunction, bound: &BoundCoreGuiSlot) {
    // IDA 0x601cfc (`mpl::bool_<false>` = not-small-object): `operator new(0x10)`
    // (0x601d24), 16-byte functor copy plus shared-count bump (0x601d36-0x601d7a),
    // out-ptr store (0x601d86). Rust: the heap clone is a bound `Some`.
    func.bound = Some(bound.clone());
}

// 0x601dd0 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS4_17StarterGuiService11CoreGuiTypeERKbEENS0_5list3INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEENSJ_ILi2EEEEEEclIS8_bEEvRT_RT0_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::StarterGuiService::CoreGuiType const&,bool const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<RBX::StarterGuiService::CoreGuiType,bool>(RBX::StarterGuiService::CoreGuiType &,bool &)")]
pub fn stub_0x601dd0(bound: &BoundCoreGuiSlot, value: CoreGuiType, flag: bool) {
    // IDA 0x601dd0: member-function dispatch out of the bind_t triple (0x601dd0-0x601de4):
    // adjust the stored object (`v1 >> 1`, virtual via `v1 & 1`), call through it,
    // forwarding the `(CoreGuiType, bool)` call args. Rust folds the triple into the target.
    bound.call(value, flag);
}

// 0x601dec — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_17StarterGuiService11CoreGuiTypeERKbEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::StarterGuiService::CoreGuiType const&,bool const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x601dec(
    op: crate::descriptor::FunctorOp,
    src: &BoundCoreGuiSlot,
    slot: &mut Option<Box<BoundCoreGuiSlot>>,
) -> bool {
    // IDA 0x601dec (`mpl::bool_<false>`): case 0 heap-clones the triple + shared count
    // (`new(0x10)`, 0x601e6a-0x601e9c); case 1 moves the pointer and nulls the source
    // (0x601ea2-0x601ea8); case 2 releases the shared count, deletes, nulls
    // (0x601eac-0x601ec8); case 3 answers the stored pointer on `strcmp` typeinfo match,
    // else null (0x601ee6-0x601ef0); default reports the bind_t typeinfo (0x601e4a-0x601e4c).
    // The move source is `&`-borrowed here, so move degrades to clone; only one functor
    // type exists, so the type check always hits.
    use crate::descriptor::FunctorOp::*;
    match op {
        CloneFunctor | MoveFunctor | CheckFunctorType => {
            *slot = Some(Box::new(src.clone()));
            true
        }
        DestroyFunctor => {
            *slot = None;
            false
        }
        GetFunctorTypeInfo => true,
    }
}

// 0x602cf0 — __ZN3RBX10Reflection13BoundFuncDescINS_17StarterGuiServiceEFbNS2_11CoreGuiTypeEELi1EEC2EMS2_FbS3_EPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::StarterGuiService,bool ()(RBX::StarterGuiService::CoreGuiType),1>::BoundFuncDesc(bool (RBX::StarterGuiService::*)(RBX::StarterGuiService::CoreGuiType),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x602cf0(
    name: &str,
    category: &str,
    member0: usize,
    member1: usize,
    permissions: u32,
    attributes: u32,
) -> CoreGuiBoundFuncDesc {
    // IDA 0x602cf0: base `FunctionDescriptor` init (0x602d48), vtable off_1272088
    // (0x602d5e), member-function pair stored at +40 (0x602d6c), reserved word at +48
    // zeroed (0x602d76), `Type::getSingleton<void>` (0x602d98), then `declareSignature()`
    // (0x602dac) fixing the return type to bool with one `CoreGuiType` argument
    // (see stub_0x602e68). The declared arg name follows the stub_0x4a38b8 convention.
    CoreGuiBoundFuncDesc {
        name: name.to_owned(),
        category: category.to_owned(),
        member: (member0, member1),
        return_type: "bool",
        args: vec![CoreGuiBoundFuncSigItem {
            name: category.to_owned(),
            type_name: "CoreGuiType",
        }],
        permissions,
        attributes,
    }
}

// 0x602e68 — __ZN3RBX10Reflection13BoundFuncDescINS_17StarterGuiServiceEFbNS2_11CoreGuiTypeEELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::StarterGuiService,bool ()(RBX::StarterGuiService::CoreGuiType),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x602e68(desc: &mut CoreGuiBoundFuncDesc, arg_name: &str, _default: &crate::descriptor::Variant) {
    // IDA 0x602e68: return type fixed to `Type::getSingleton<bool>` at +28
    // (0x602e74-0x602e78), `Name::declare(arg name)` (0x602e82), argument type
    // `Type::getSingleton<CoreGuiType>` (0x602e84), `SignatureDescriptor::addArgument`
    // (0x602e96). The default-value Variant is carried by the signature item in C++;
    // unmodeled here.
    desc.return_type = "bool";
    desc.args.push(CoreGuiBoundFuncSigItem {
        name: arg_name.to_owned(),
        type_name: "CoreGuiType",
    });
}

// 0x602e98 — __ZN3RBX10Reflection13BoundFuncDescINS_17StarterGuiServiceEFbNS2_11CoreGuiTypeEELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::StarterGuiService,bool ()(RBX::StarterGuiService::CoreGuiType),1>::~BoundFuncDesc()")]
pub fn stub_0x602e98() {
    // IDA 0x602e98: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x602f6c — __ZNK3RBX10Reflection13BoundFuncDescINS_17StarterGuiServiceEFbNS2_11CoreGuiTypeEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::StarterGuiService,bool ()(RBX::StarterGuiService::CoreGuiType),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x602f6c(target: &dyn Fn(CoreGuiType) -> bool, arg: CoreGuiType) -> bool {
    // IDA 0x602f6c: member-offset adjust (0x602f7a-0x602f7c), member pair load (0x602f86),
    // `ArgHelper::getArg<CoreGuiType, 1>` (0x602f90), `Call1Helper::call` (0x602fa8). The
    // adjust is member-pointer mechanics over an unmodeled instance; the caller passes the
    // already-extracted argument and the bound member closure.
    stub_0x602fac(target, arg)
}

// 0x602fac — __ZN3RBX10Reflection11Call1HelperINS_17StarterGuiServiceEMS2_FbNS2_11CoreGuiTypeEES3_bE4callEPS2_S5_RNS0_7VariantERKS3_
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::StarterGuiService,bool (RBX::StarterGuiService::*)(RBX::StarterGuiService::CoreGuiType),RBX::StarterGuiService::CoreGuiType,bool>::call(RBX::StarterGuiService*,bool (RBX::StarterGuiService::*)(RBX::StarterGuiService::CoreGuiType),RBX::Reflection::Variant &,RBX::StarterGuiService::CoreGuiType const&)")]
pub fn stub_0x602fac(target: &dyn Fn(CoreGuiType) -> bool, arg: CoreGuiType) -> bool {
    // IDA 0x602fac: member-pointer dispatch (`adj >> 1`, virtual via `adj & 1`,
    // 0x602fb4-0x602fc4), invoke `(target.*member)(arg)` (0x602fc8), wrap the bool result
    // (`Type::getSingleton<bool>` + `placement_any<bool>`, 0x602fd2-0x602fe0). The Variant
    // out-param is unmodeled; the observable result is the bool.
    target(arg)
}

// 0x602fe4 — __ZN3RBX10Reflection9ArgHelper6getArgINS_17StarterGuiService11CoreGuiTypeELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "RBX::StarterGuiService::CoreGuiType RBX::Reflection::ArgHelper::getArg<RBX::StarterGuiService::CoreGuiType,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::StarterGuiService::CoreGuiType> const&,boost::disable_if<boost::is_same<RBX::StarterGuiService::CoreGuiType,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
pub fn stub_0x602fe4(args: &[crate::descriptor::Variant], default: Option<CoreGuiType>) -> CoreGuiType {
    // IDA 0x602fe4: empty-arguments check (0x60301c); `try_enum<1, CoreGuiType>` first
    // (0x60305c-0x603060); else the numeric-convertible path via
    // `genericConvert<CoreGuiType>` (0x603068-0x60308c), which throws on failure. Missing
    // with no default throws `runtime_error("Argument %d missing or nil")`
    // (0x6030ac-0x603146); otherwise the default is returned (0x6030ae). Index 1 in the
    // original's 1-based convention is the first slice element.
    if let Some(arg) = args.first() {
        let mut out = 0;
        if stub_0x603174(arg, &mut out) {
            return out;
        }
        return stub_0x5fd070(arg);
    }
    match default {
        Some(d) => d,
        None => panic!("Argument 1 missing or nil (IDA 0x602fe4)"),
    }
}

// 0x603174 — __ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_17StarterGuiService11CoreGuiTypeEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<1,RBX::StarterGuiService::CoreGuiType>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::StarterGuiService::CoreGuiType &,boost::enable_if<boost::is_enum<RBX::StarterGuiService::CoreGuiType>,void>::type *)")]
pub fn stub_0x603174(arg: &crate::descriptor::Variant, out: &mut CoreGuiType) -> bool {
    // IDA 0x603174: `Singleton<EnumDesc<CoreGuiType>>` via `call_once` + `doGetSingleton`
    // (0x60319a-0x60319e), then the Arguments virtual at +44 attempts the enum conversion
    // against that singleton (0x6031b6); on success stores the value and returns 1
    // (0x6031bc-0x6031c2). Int payloads convert directly; text goes through
    // `EnumDesc::convertToValue` (see stub_0x5fe600); anything else misses.
    let desc = crate::generated::stub_0x4b3cb8();
    match arg {
        crate::descriptor::Variant::Int(v) => {
            *out = *v;
            true
        }
        crate::descriptor::Variant::Text(s) => match desc.lookup_value(s) {
            Some(v) => {
                *out = v;
                true
            }
            None => false,
        },
        _ => false,
    }
}

// 0x6031c8 — __ZN3RBX10Reflection13BoundFuncDescINS_17StarterGuiServiceEFvNS2_11CoreGuiTypeEbELi2EEC2EMS2_FvS3_bEPKcS9_S9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::StarterGuiService,void ()(RBX::StarterGuiService::CoreGuiType,bool),2>::BoundFuncDesc(void (RBX::StarterGuiService::*)(RBX::StarterGuiService::CoreGuiType,bool),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x6031c8(
    name: &str,
    category: &str,
    title: &str,
    member0: usize,
    member1: usize,
    permissions: u32,
    attributes: u32,
) -> CoreGuiBoundFuncDesc {
    // IDA 0x6031c8: base `FunctionDescriptor` init (0x603220), vtable off_12720C8
    // (0x603236), member-function pair at +40 (0x60323a), reserved words at +48/+52
    // zeroed (0x60324a-0x603254), void-typed defaults staged (0x603278-0x60328c), then
    // `declareSignature()` (0x60329c) fixing a void return with `(CoreGuiType, bool)`
    // arguments (see stub_0x603390). Declared arg names follow the stub_0x4a38b8 convention.
    CoreGuiBoundFuncDesc {
        name: name.to_owned(),
        category: category.to_owned(),
        member: (member0, member1),
        return_type: "void",
        args: vec![
            CoreGuiBoundFuncSigItem {
                name: category.to_owned(),
                type_name: "CoreGuiType",
            },
            CoreGuiBoundFuncSigItem {
                name: title.to_owned(),
                type_name: "bool",
            },
        ],
        permissions,
        attributes,
    }
}

// 0x603390 — __ZN3RBX10Reflection13BoundFuncDescINS_17StarterGuiServiceEFvNS2_11CoreGuiTypeEbELi2EE16declareSignatureEPKcNS0_7VariantES7_S8_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::StarterGuiService,void ()(RBX::StarterGuiService::CoreGuiType,bool),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_0x603390() -> ! {
    todo!("0x603390 RBX::Reflection::BoundFuncDesc<RBX::StarterGuiService,void ()(RBX::StarterGuiService::CoreGuiType,bool),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0x6033dc — __ZN3RBX10Reflection13BoundFuncDescINS_17StarterGuiServiceEFvNS2_11CoreGuiTypeEbELi2EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::StarterGuiService,void ()(RBX::StarterGuiService::CoreGuiType,bool),2>::~BoundFuncDesc()")]
pub fn stub_0x6033dc() {
    // IDA 0x6033dc: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x6034bc — __ZNK3RBX10Reflection13BoundFuncDescINS_17StarterGuiServiceEFvNS2_11CoreGuiTypeEbELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::StarterGuiService,void ()(RBX::StarterGuiService::CoreGuiType,bool),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x6034bc() -> ! {
    todo!("0x6034bc RBX::Reflection::BoundFuncDesc<RBX::StarterGuiService,void ()(RBX::StarterGuiService::CoreGuiType,bool),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x603510 — __ZN3RBX10Reflection14PropDescriptorINS_17StarterGuiServiceEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::StarterGuiService,bool>::PropDescriptor<bool (RBX::StarterGuiService::*)(void)const,void (RBX::StarterGuiService::*)(bool)>(char const*,char const*,bool (RBX::StarterGuiService::*)(void)const,void (RBX::StarterGuiService::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x603510() -> ! {
    todo!("0x603510 RBX::Reflection::PropDescriptor<RBX::StarterGuiService,bool>::PropDescriptor<bool (RBX::StarterGuiService::*)(void)const,void (RBX::StarterGuiService::*)(bool)>(char const*,char const*,bool (RBX::StarterGuiService::*)(void)const,void (RBX::StarterGuiService::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x603628 — __ZN3RBX10Reflection14PropDescriptorINS_17StarterGuiServiceEbED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::StarterGuiService,bool>::~PropDescriptor()")]
pub fn stub_0x603628() {
    // IDA 0x603628: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x603658 — __ZN3RBX10Reflection23TypedPropertyDescriptorIbED1Ev
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<bool>::~TypedPropertyDescriptor()")]
pub fn stub_0x603658() {
    // IDA 0x603658: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x603680 — __ZNK3RBX10Reflection14PropDescriptorINS_17StarterGuiServiceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::StarterGuiService,bool>::GetSetImpl<bool (RBX::StarterGuiService::*)(void)const,void (RBX::StarterGuiService::*)(bool)>::isReadOnly(void)const")]
pub fn stub_0x603680() -> bool {
    // IDA 0x603680: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0x603684 — __ZNK3RBX10Reflection14PropDescriptorINS_17StarterGuiServiceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::StarterGuiService,bool>::GetSetImpl<bool (RBX::StarterGuiService::*)(void)const,void (RBX::StarterGuiService::*)(bool)>::isWriteOnly(void)const")]
pub fn stub_0x603684() -> bool {
    // IDA 0x603684: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0x603688 — __ZNK3RBX10Reflection14PropDescriptorINS_17StarterGuiServiceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::StarterGuiService,bool>::GetSetImpl<bool (RBX::StarterGuiService::*)(void)const,void (RBX::StarterGuiService::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x603688() -> ! {
    todo!("0x603688 RBX::Reflection::PropDescriptor<RBX::StarterGuiService,bool>::GetSetImpl<bool (RBX::StarterGuiService::*)(void)const,void (RBX::StarterGuiService::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x6036ac — __ZNK3RBX10Reflection14PropDescriptorINS_17StarterGuiServiceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::StarterGuiService,bool>::GetSetImpl<bool (RBX::StarterGuiService::*)(void)const,void (RBX::StarterGuiService::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
pub fn stub_0x6036ac() -> ! {
    todo!("0x6036ac RBX::Reflection::PropDescriptor<RBX::StarterGuiService,bool>::GetSetImpl<bool (RBX::StarterGuiService::*)(void)const,void (RBX::StarterGuiService::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")
}

// 0x605fa4 — __ZN3RBX10Reflection13BoundFuncDescINS_4PoseEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Pose,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()")]
pub fn stub_0x605fa4() {
    // IDA 0x605fa4: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x605fc8 — __ZN3RBX10Reflection13BoundFuncDescINS_4PoseEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Pose,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
pub fn stub_0x605fc8() {
    // IDA 0x605fc8: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x6060d8 — __ZN3RBX10Reflection14PropDescriptorINS_4PoseEN3G3D15CoordinateFrameEED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Pose,G3D::CoordinateFrame>::~PropDescriptor()")]
pub fn stub_0x6060d8() {
    // IDA 0x6060d8: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x606104 — __ZN3RBX10Reflection14PropDescriptorINS_4PoseEfED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Pose,float>::~PropDescriptor()")]
pub fn stub_0x606104() {
    // IDA 0x606104: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x606774 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4PoseES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Pose,RBX::Pose>(rbx_core::SharedPtr<RBX::Pose> const*,RBX::Pose *)const")]
pub fn stub_0x606774() {
    // IDA 0x606774: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0x606f40 — __ZN3RBX10Reflection14PropDescriptorINS_4PoseEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Pose,float>::PropDescriptor<float (RBX::Pose::*)(void)const,void (RBX::Pose::*)(float)>(char const*,char const*,float (RBX::Pose::*)(void)const,void (RBX::Pose::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x606f40() -> ! {
    todo!("0x606f40 RBX::Reflection::PropDescriptor<RBX::Pose,float>::PropDescriptor<float (RBX::Pose::*)(void)const,void (RBX::Pose::*)(float)>(char const*,char const*,float (RBX::Pose::*)(void)const,void (RBX::Pose::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x607054 — __ZN3RBX10Reflection14PropDescriptorINS_4PoseEfED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Pose,float>::~PropDescriptor()")]
pub fn stub_0x607054() {
    // IDA 0x607054: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x607080 — __ZNK3RBX10Reflection14PropDescriptorINS_4PoseEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Pose,float>::GetSetImpl<float (RBX::Pose::*)(void)const,void (RBX::Pose::*)(float)>::isReadOnly(void)const")]
pub fn stub_0x607080() -> bool {
    // IDA 0x607080: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0x607084 — __ZNK3RBX10Reflection14PropDescriptorINS_4PoseEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Pose,float>::GetSetImpl<float (RBX::Pose::*)(void)const,void (RBX::Pose::*)(float)>::isWriteOnly(void)const")]
pub fn stub_0x607084() -> bool {
    // IDA 0x607084: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0x607088 — __ZNK3RBX10Reflection14PropDescriptorINS_4PoseEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Pose,float>::GetSetImpl<float (RBX::Pose::*)(void)const,void (RBX::Pose::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x607088() -> ! {
    todo!("0x607088 RBX::Reflection::PropDescriptor<RBX::Pose,float>::GetSetImpl<float (RBX::Pose::*)(void)const,void (RBX::Pose::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x6070a8 — __ZNK3RBX10Reflection14PropDescriptorINS_4PoseEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Pose,float>::GetSetImpl<float (RBX::Pose::*)(void)const,void (RBX::Pose::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
pub fn stub_0x6070a8() -> ! {
    todo!("0x6070a8 RBX::Reflection::PropDescriptor<RBX::Pose,float>::GetSetImpl<float (RBX::Pose::*)(void)const,void (RBX::Pose::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")
}

// 0x6070cc — __ZN3RBX10Reflection14PropDescriptorINS_4PoseEN3G3D15CoordinateFrameEEC2IMS2_KFRKS4_vEMS2_FvS8_EEEPKcSE_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Pose,G3D::CoordinateFrame>::PropDescriptor<G3D::CoordinateFrame const& (RBX::Pose::*)(void)const,void (RBX::Pose::*)(G3D::CoordinateFrame const&)>(char const*,char const*,G3D::CoordinateFrame const& (RBX::Pose::*)(void)const,void (RBX::Pose::*)(G3D::CoordinateFrame const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x6070cc() -> ! {
    todo!("0x6070cc RBX::Reflection::PropDescriptor<RBX::Pose,G3D::CoordinateFrame>::PropDescriptor<G3D::CoordinateFrame const& (RBX::Pose::*)(void)const,void (RBX::Pose::*)(G3D::CoordinateFrame const&)>(char const*,char const*,G3D::CoordinateFrame const& (RBX::Pose::*)(void)const,void (RBX::Pose::*)(G3D::CoordinateFrame const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x6071e0 — __ZN3RBX10Reflection14PropDescriptorINS_4PoseEN3G3D15CoordinateFrameEED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Pose,G3D::CoordinateFrame>::~PropDescriptor()")]
pub fn stub_0x6071e0() {
    // IDA 0x6071e0: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x60720c — __ZNK3RBX10Reflection14PropDescriptorINS_4PoseEN3G3D15CoordinateFrameEE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Pose,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::Pose::*)(void)const,void (RBX::Pose::*)(G3D::CoordinateFrame const&)>::isReadOnly(void)const")]
pub fn stub_0x60720c() -> bool {
    // IDA 0x60720c: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0x607210 — __ZNK3RBX10Reflection14PropDescriptorINS_4PoseEN3G3D15CoordinateFrameEE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Pose,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::Pose::*)(void)const,void (RBX::Pose::*)(G3D::CoordinateFrame const&)>::isWriteOnly(void)const")]
pub fn stub_0x607210() -> bool {
    // IDA 0x607210: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0x607214 — __ZNK3RBX10Reflection14PropDescriptorINS_4PoseEN3G3D15CoordinateFrameEE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Pose,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::Pose::*)(void)const,void (RBX::Pose::*)(G3D::CoordinateFrame const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x607214() -> ! {
    todo!("0x607214 RBX::Reflection::PropDescriptor<RBX::Pose,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::Pose::*)(void)const,void (RBX::Pose::*)(G3D::CoordinateFrame const&)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x607250 — __ZNK3RBX10Reflection14PropDescriptorINS_4PoseEN3G3D15CoordinateFrameEE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE8setValueEPNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Pose,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::Pose::*)(void)const,void (RBX::Pose::*)(G3D::CoordinateFrame const&)>::setValue(RBX::Reflection::DescribedBase *,G3D::CoordinateFrame const&)const")]
pub fn stub_0x607250() -> ! {
    todo!("0x607250 RBX::Reflection::PropDescriptor<RBX::Pose,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::Pose::*)(void)const,void (RBX::Pose::*)(G3D::CoordinateFrame const&)>::setValue(RBX::Reflection::DescribedBase *,G3D::CoordinateFrame const&)const")
}

// 0x607274 — __ZN3RBX10Reflection13BoundFuncDescINS_4PoseEFvN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS2_FvS6_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Pose,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(void (RBX::Pose::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x607274() -> ! {
    todo!("0x607274 RBX::Reflection::BoundFuncDesc<RBX::Pose,void ()(boost::shared_ptr<RBX::Instance>),1>::BoundFuncDesc(void (RBX::Pose::*)(boost::shared_ptr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x60740c — __ZN3RBX10Reflection13BoundFuncDescINS_4PoseEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Pose,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x60740c() -> ! {
    todo!("0x60740c RBX::Reflection::BoundFuncDesc<RBX::Pose,void ()(boost::shared_ptr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x60743c — __ZN3RBX10Reflection13BoundFuncDescINS_4PoseEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Pose,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
pub fn stub_0x60743c() {
    // IDA 0x60743c: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x607558 — __ZNK3RBX10Reflection13BoundFuncDescINS_4PoseEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Pose,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x607558() -> ! {
    todo!("0x607558 RBX::Reflection::BoundFuncDesc<RBX::Pose,void ()(boost::shared_ptr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x60763c — __ZN3RBX10Reflection11Call1HelperINS_4PoseEMS2_FvN5boost10shared_ptrINS_8InstanceEEEES6_vE4callEPS2_S8_RNS0_7VariantERKS6_
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Pose,void (RBX::Pose::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::Pose*,void (RBX::Pose::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")]
pub fn stub_0x60763c() -> ! {
    todo!("0x60763c RBX::Reflection::Call1Helper<RBX::Pose,void (RBX::Pose::*)(boost::shared_ptr<RBX::Instance>),boost::shared_ptr<RBX::Instance>,void>::call(RBX::Pose*,void (RBX::Pose::*)(boost::shared_ptr<RBX::Instance>),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&)")
}

// 0x607724 — __ZN3RBX10Reflection13BoundFuncDescINS_4PoseEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EEC2EMS2_FSB_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Pose,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Pose::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x607724() -> ! {
    todo!("0x607724 RBX::Reflection::BoundFuncDesc<RBX::Pose,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::BoundFuncDesc(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::Pose::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x607828 — __ZN3RBX10Reflection13BoundFuncDescINS_4PoseEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Pose,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()")]
pub fn stub_0x607828() {
    // IDA 0x607828: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x6078dc — __ZNK3RBX10Reflection13BoundFuncDescINS_4PoseEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Pose,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x6078dc() -> ! {
    todo!("0x6078dc RBX::Reflection::BoundFuncDesc<RBX::Pose,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x607900 — __ZN3RBX10Reflection11Call0HelperINS_4PoseEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvESB_E4callEPS2_SD_RNS0_7VariantE
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::Pose,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Pose::*)(void),rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::call(RBX::Pose*,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Pose::*)(void),RBX::Reflection::Variant &)")]
pub fn stub_0x607900() -> ! {
    todo!("0x607900 RBX::Reflection::Call0Helper<RBX::Pose,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::Pose::*)(void),boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>>::call(RBX::Pose*,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::Pose::*)(void),RBX::Reflection::Variant &)")
}

// 0x60814c — __ZN3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::~EnumPropDescriptor()")]
pub fn stub_0x60814c() {
    // IDA 0x60814c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x608170 — __ZN3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::EnumPropDescriptor<RBX::PrismInstance::NumSidesEnum (RBX::PrismInstance::*)(void)const,void (RBX::PrismInstance::*)(RBX::PrismInstance::NumSidesEnum)>(char const*,char const*,RBX::PrismInstance::NumSidesEnum (RBX::PrismInstance::*)(void)const,void (RBX::PrismInstance::*)(RBX::PrismInstance::NumSidesEnum),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x608170() -> ! {
    todo!("0x608170 RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::EnumPropDescriptor<RBX::PrismInstance::NumSidesEnum (RBX::PrismInstance::*)(void)const,void (RBX::PrismInstance::*)(RBX::PrismInstance::NumSidesEnum)>(char const*,char const*,RBX::PrismInstance::NumSidesEnum (RBX::PrismInstance::*)(void)const,void (RBX::PrismInstance::*)(RBX::PrismInstance::NumSidesEnum),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x608324 — __ZN3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::~EnumPropDescriptor()")]
pub fn stub_0x608324() {
    // IDA 0x608324: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x608350 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::isReadOnly(void)const")]
pub fn stub_0x608350() {
    // IDA 0x608350: EnumPropDescriptor::isReadOnly/isWriteOnly -- forwards through the bound member descriptor at +44 (`(**(this+44))(this+44)`, decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x608360 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::isWriteOnly(void)const")]
pub fn stub_0x608360() {
    // IDA 0x608360: EnumPropDescriptor::isReadOnly/isWriteOnly -- forwards through the bound member descriptor at +44 (`(**(this+44))(this+44)`, decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x608370 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x608370() -> ! {
    todo!("0x608370 RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

// 0x608398 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x608398() -> ! {
    todo!("0x608398 RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}

// 0x6083bc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x6083bc() -> ! {
    todo!("0x6083bc RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}

// 0x608508 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x608508() -> ! {
    todo!("0x608508 RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}

// 0x60852c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::hasStringValue(void)const")]
pub fn stub_0x60852c() -> bool {
    // IDA 0x60852c: EnumPropDescriptor::hasStringValue -- hardcoded `return 1` (decompiled 0x10244/0x10dc8/0x11650).
    true
}

// 0x608530 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x608530() -> ! {
    todo!("0x608530 RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::getStringValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x608554 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_0x608554() -> ! {
    todo!("0x608554 RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x608594 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_0x608594() -> ! {
    todo!("0x608594 RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

// 0x6085b4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x6085b4() -> ! {
    todo!("0x6085b4 RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

// 0x6087f4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x6087f4() -> ! {
    todo!("0x6087f4 RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::getIndexValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x608810 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_0x608810() -> ! {
    todo!("0x608810 RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")
}

// 0x608844 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x608844() -> ! {
    todo!("0x608844 RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::getEnumValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x60884c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x60884c() -> ! {
    todo!("0x60884c RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x608898 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x608898() -> ! {
    todo!("0x608898 RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::getEnumItem(RBX::Reflection::DescribedBase const*)const")
}

// 0x6088b8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_0x6088b8() -> ! {
    todo!("0x6088b8 RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")
}

// 0x6088ec — __ZNK3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::convertToIndex(RBX::PrismInstance::NumSidesEnum)const")]
pub fn stub_0x6088ec(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0x6088ec: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0x60895c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x60895c() -> ! {
    todo!("0x60895c RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x60899c — __ZNK3RBX10Reflection14PropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::GetSetImpl<RBX::PrismInstance::NumSidesEnum (RBX::PrismInstance::*)(void)const,void (RBX::PrismInstance::*)(RBX::PrismInstance::NumSidesEnum)>::isReadOnly(void)const")]
pub fn stub_0x60899c() -> bool {
    // IDA 0x60899c: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0x6089a0 — __ZNK3RBX10Reflection14PropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::GetSetImpl<RBX::PrismInstance::NumSidesEnum (RBX::PrismInstance::*)(void)const,void (RBX::PrismInstance::*)(RBX::PrismInstance::NumSidesEnum)>::isWriteOnly(void)const")]
pub fn stub_0x6089a0() -> bool {
    // IDA 0x6089a0: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0x6089a4 — __ZNK3RBX10Reflection14PropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::GetSetImpl<RBX::PrismInstance::NumSidesEnum (RBX::PrismInstance::*)(void)const,void (RBX::PrismInstance::*)(RBX::PrismInstance::NumSidesEnum)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x6089a4() -> ! {
    todo!("0x6089a4 RBX::Reflection::PropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::GetSetImpl<RBX::PrismInstance::NumSidesEnum (RBX::PrismInstance::*)(void)const,void (RBX::PrismInstance::*)(RBX::PrismInstance::NumSidesEnum)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x6089c4 — __ZNK3RBX10Reflection14PropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::GetSetImpl<RBX::PrismInstance::NumSidesEnum (RBX::PrismInstance::*)(void)const,void (RBX::PrismInstance::*)(RBX::PrismInstance::NumSidesEnum)>::setValue(RBX::Reflection::DescribedBase *,RBX::PrismInstance::NumSidesEnum const&)const")]
pub fn stub_0x6089c4() -> ! {
    todo!("0x6089c4 RBX::Reflection::PropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::GetSetImpl<RBX::PrismInstance::NumSidesEnum (RBX::PrismInstance::*)(void)const,void (RBX::PrismInstance::*)(RBX::PrismInstance::NumSidesEnum)>::setValue(RBX::Reflection::DescribedBase *,RBX::PrismInstance::NumSidesEnum const&)const")
}

// 0x60939c — __ZN3RBX10Reflection14PropDescriptorINS_10PVInstanceEN3G3D15CoordinateFrameEED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PVInstance,G3D::CoordinateFrame>::~PropDescriptor()")]
pub fn stub_0x60939c() {
    // IDA 0x60939c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x6098d4 — __ZN3RBX10Reflection14PropDescriptorINS_10PVInstanceEN3G3D15CoordinateFrameEEC2IiMS2_FvRKS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PVInstance,G3D::CoordinateFrame>::PropDescriptor<int,void (RBX::PVInstance::*)(G3D::CoordinateFrame const&)>(char const*,char const*,int,void (RBX::PVInstance::*)(G3D::CoordinateFrame const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x6098d4() -> ! {
    todo!("0x6098d4 RBX::Reflection::PropDescriptor<RBX::PVInstance,G3D::CoordinateFrame>::PropDescriptor<int,void (RBX::PVInstance::*)(G3D::CoordinateFrame const&)>(char const*,char const*,int,void (RBX::PVInstance::*)(G3D::CoordinateFrame const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x6099e0 — __ZN3RBX10Reflection14PropDescriptorINS_10PVInstanceEN3G3D15CoordinateFrameEED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PVInstance,G3D::CoordinateFrame>::~PropDescriptor()")]
pub fn stub_0x6099e0() {
    // IDA 0x6099e0: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x609a0c — __ZNK3RBX10Reflection14PropDescriptorINS_10PVInstanceEN3G3D15CoordinateFrameEE7SetImplIMS2_FvRKS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PVInstance,G3D::CoordinateFrame>::SetImpl<void (RBX::PVInstance::*)(G3D::CoordinateFrame const&)>::isReadOnly(void)const")]
pub fn stub_0x609a0c() {
    // IDA 0x609a0c: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x609a10 — __ZNK3RBX10Reflection14PropDescriptorINS_10PVInstanceEN3G3D15CoordinateFrameEE7SetImplIMS2_FvRKS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PVInstance,G3D::CoordinateFrame>::SetImpl<void (RBX::PVInstance::*)(G3D::CoordinateFrame const&)>::isWriteOnly(void)const")]
pub fn stub_0x609a10() {
    // IDA 0x609a10: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x609a14 — __ZNK3RBX10Reflection14PropDescriptorINS_10PVInstanceEN3G3D15CoordinateFrameEE7SetImplIMS2_FvRKS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PVInstance,G3D::CoordinateFrame>::SetImpl<void (RBX::PVInstance::*)(G3D::CoordinateFrame const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x609a14() -> ! {
    todo!("0x609a14 RBX::Reflection::PropDescriptor<RBX::PVInstance,G3D::CoordinateFrame>::SetImpl<void (RBX::PVInstance::*)(G3D::CoordinateFrame const&)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x609b34 — __ZNK3RBX10Reflection14PropDescriptorINS_10PVInstanceEN3G3D15CoordinateFrameEE7SetImplIMS2_FvRKS4_EE8setValueEPNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PVInstance,G3D::CoordinateFrame>::SetImpl<void (RBX::PVInstance::*)(G3D::CoordinateFrame const&)>::setValue(RBX::Reflection::DescribedBase *,G3D::CoordinateFrame const&)const")]
pub fn stub_0x609b34() -> ! {
    todo!("0x609b34 RBX::Reflection::PropDescriptor<RBX::PVInstance,G3D::CoordinateFrame>::SetImpl<void (RBX::PVInstance::*)(G3D::CoordinateFrame const&)>::setValue(RBX::Reflection::DescribedBase *,G3D::CoordinateFrame const&)const")
}

// 0x60a258 — __ZN3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::~EnumPropDescriptor()")]
pub fn stub_0x60a258() {
    // IDA 0x60a258: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x60a27c — __ZN3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::EnumPropDescriptor<RBX::PyramidInstance::NumSidesEnum (RBX::PyramidInstance::*)(void)const,void (RBX::PyramidInstance::*)(RBX::PyramidInstance::NumSidesEnum)>(char const*,char const*,RBX::PyramidInstance::NumSidesEnum (RBX::PyramidInstance::*)(void)const,void (RBX::PyramidInstance::*)(RBX::PyramidInstance::NumSidesEnum),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x60a27c() -> ! {
    todo!("0x60a27c RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::EnumPropDescriptor<RBX::PyramidInstance::NumSidesEnum (RBX::PyramidInstance::*)(void)const,void (RBX::PyramidInstance::*)(RBX::PyramidInstance::NumSidesEnum)>(char const*,char const*,RBX::PyramidInstance::NumSidesEnum (RBX::PyramidInstance::*)(void)const,void (RBX::PyramidInstance::*)(RBX::PyramidInstance::NumSidesEnum),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x60a430 — __ZN3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::~EnumPropDescriptor()")]
pub fn stub_0x60a430() {
    // IDA 0x60a430: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x60a45c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::isReadOnly(void)const")]
pub fn stub_0x60a45c() {
    // IDA 0x60a45c: EnumPropDescriptor::isReadOnly/isWriteOnly -- forwards through the bound member descriptor at +44 (`(**(this+44))(this+44)`, decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x60a46c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::isWriteOnly(void)const")]
pub fn stub_0x60a46c() {
    // IDA 0x60a46c: EnumPropDescriptor::isReadOnly/isWriteOnly -- forwards through the bound member descriptor at +44 (`(**(this+44))(this+44)`, decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x60a47c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x60a47c() -> ! {
    todo!("0x60a47c RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

// 0x60a4a4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x60a4a4() -> ! {
    todo!("0x60a4a4 RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}

// 0x60a4c8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x60a4c8() -> ! {
    todo!("0x60a4c8 RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}
