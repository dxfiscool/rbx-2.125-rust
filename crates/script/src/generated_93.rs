// Auto-generated skeletons for rbx-script — Lua|Script|Yield|lua filtered batch
// Filter: Lua|Script|Yield|lua (5401 filtered, 100 in this batch)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x831ff4..0x8e901c | 4011->4111 covered, 1290 remaining, rbx_core::SharedPtr not boost
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use std::collections::HashMap;
use std::sync::Weak;

use rbx_core::SharedPtr;

// ---- FriendService scriptable flags + PersonalServerService/BindableFunction yield-desc cluster (IDA 0x84527c..0x89f9a4) ----
// Ground truth per stub: `decompile(ea)` + `disasm(ea)` via IDA MCP.
// Boost mapping (AGENTS.md section 4): boost::shared_ptr -> rbx_core::SharedPtr
// (Arc); boost::function/bind -> Box<dyn Fn> closures; boost::unordered_map ->
// HashMap.
// Unmodeled throughout: C++ vtable installs, RTTI unwind tables, std::string
// copy-on-write internals (String covers the observable copy), and the
// Name::declare interning pool (names kept raw).

/// was: `RBX::Reflection::RemoteEventDesc<...>` flag word at +0x30 whose bit 0
/// is the scriptable bit (IDA 0x84527c/0x846a1c: `LDR R0,[R0,#0x30]; AND #1`).
#[derive(Debug, Clone, Copy, Default)]
pub struct RemoteEventFlags {
    /// Flag word at +0x30.
    pub word_0x30: u32,
}

/// Shared isScriptable body (IDA 0x84527c, 0x846a1c): bit 0 of +0x30.
fn remote_event_is_scriptable(flags: &RemoteEventFlags) -> bool {
    flags.word_0x30 & 1 != 0
}

/// was: `RBX::Reflection::SignatureDescriptor::Item` — one entry of the
/// signature list cleared at D1/D0 (IDA 0x892a8a/0x894366 `_M_clear` on +8).
#[derive(Clone, Debug, Default)]
pub struct YieldSignatureItem {
    /// Argument name (was: interned Name; kept raw).
    pub name: String,
    /// Argument type tag (was: Type singleton ref).
    pub type_tag: u8,
}

/// was: `RBX::Reflection::BoundYieldFuncDesc<RBX::PersonalServerService,
/// std::string ()(int), std::string, 1>` — yield descriptor: member-fn pair at
/// +0x28 (IDA 0x8941b8 `STRD R10,R11,[R0,#0x28]`), owned member at +0x30
/// (IDA 0x892a6a `LDR [R4,#0x30]`, deleted when non-null), signature list at
/// +8 (IDA 0x892a8a `_M_clear(a1+8)`).
#[derive(Default)]
pub struct PersonalServerYieldDesc {
    /// Bound member fn (was: member pair (void*, adjust) at +0x28).
    pub method: Option<fn(i32) -> String>,
    /// Owned member at +0x30 (deleted when non-null).
    pub owned: Option<Box<u8>>,
    /// Signature items (+8 list).
    pub signature: Vec<YieldSignatureItem>,
    /// Return-type tag (IDA 0x8942c4: Type<string> stored at +0x1C).
    pub return_type_tag: u8,
}

/// Shared D1 body (IDA 0x892a54): vtable reset, delete +0x30 member when
/// non-null (0x892a6a..0x892a70), base vtable reset + signature clear
/// (0x892a86/0x892a8a).
fn personal_server_yield_destroy(desc: &mut PersonalServerYieldDesc) {
    // IDA 0x892a68/0x892a86: vtable resets (unmodeled words).
    // IDA 0x892a6a..0x892a70.
    desc.owned.take();
    // IDA 0x892a8a.
    desc.signature.clear();
}

/// was: `RBX::Reflection::BoundYieldFuncDesc<RBX::BindableFunction,
/// shared_ptr<Tuple const> ()(shared_ptr<Tuple const>), ...>` — yield
/// descriptor: scoped member at +0x30 (IDA 0x89fa08 `scoped_ptr dtor`),
/// signature list at +8 (IDA 0x89fa28 `_M_clear`).
#[derive(Default)]
pub struct BindableFunctionYieldDesc {
    /// Scoped member at +0x30 (was: scoped_ptr<shared_ptr<Tuple const>>).
    pub member: Option<SharedPtr<Vec<u8>>>,
    /// Bound member fn (was: pair at +0x28; set by C2 0x8a1acc).
    pub member_method: Option<fn(Option<SharedPtr<Vec<u8>>>) -> Option<SharedPtr<Vec<u8>>>>,
    /// Signature items (+8 list).
    pub signature: Vec<YieldSignatureItem>,
    /// Return-type tag (tuple type, IDA 0x8a1c58).
    pub return_type_tag: u8,
}
/// Type tags for `YieldSignatureItem::type_tag` (was: Type singleton refs).
/// 1 = string, 2 = int (see 0x8942b4); 3 = bool (IDA 0x8adc1e/0x8d904a
/// `Type::getSingleton<bool>`); 4 = Instance (IDA 0x8adc2e/0x8d905a
/// `Type::getSingleton<shared_ptr<Instance>>`); 5 = Tuple (IDA 0x8a1c58/0x8a1c64
/// `Type::getSingleton<shared_ptr<Tuple const>>`); 6 = Variant map (IDA
/// 0x8d9920 `Type::getSingleton<shared_ptr<map<string, Variant> const>>`).
pub const TYPE_TAG_BOOL: u8 = 3;
/// See type-tag list on [`TYPE_TAG_BOOL`]: Instance tag.
pub const TYPE_TAG_INSTANCE: u8 = 4;
/// See type-tag list on [`TYPE_TAG_BOOL`]: Tuple tag.
pub const TYPE_TAG_TUPLE: u8 = 5;
/// See type-tag list on [`TYPE_TAG_BOOL`]: Variant-map tag.
pub const TYPE_TAG_VARIANT_MAP: u8 = 6;

/// was: `RBX::Reflection::BoundYieldFuncDesc<RBX::GamePassService, bool
/// ()(shared_ptr<RBX::Instance>, int), bool, 2>` — bound member pair at
/// +0x28 (IDA 0x8ada9a `*((_QWORD *)v32 + 5) = v17`), scoped member at +0x30
/// (IDA 0x8ad150..0x8ad166 release + delete), owned word at +0x34 (IDA
/// 0x8ad126..0x8ad148 delete when non-null), signature list at +8 (IDA
/// 0x8ad186 `_M_clear`).
#[derive(Default)]
pub struct GamePassYieldDesc {
    /// Bound member fn (was: member pair (void*, adjust) at +0x28).
    pub method: Option<fn(Option<SharedPtr<Vec<u8>>>, i32) -> bool>,
    /// Scoped member at +0x30 (was: scoped_ptr<shared_ptr<Instance>>).
    pub member: Option<SharedPtr<Vec<u8>>>,
    /// Owned word at +0x34 (deleted when non-null).
    pub owned: Option<Box<u8>>,
    /// Signature items (+8 list).
    pub signature: Vec<YieldSignatureItem>,
    /// Return-type tag (IDA 0x8adc1e: Type<bool> stored at +0x1C).
    pub return_type_tag: u8,
}

/// Shared D1 body (IDA 0x8ad0e4): vtable reset, delete +0x34 owned when
/// non-null (0x8ad126..0x8ad148), release + delete +0x30 member
/// (0x8ad150..0x8ad166), base vtable reset + signature clear
/// (0x8ad17a/0x8ad186).
fn game_pass_yield_destroy(desc: &mut GamePassYieldDesc) {
    // IDA 0x8ad11c/0x8ad17a: vtable resets (unmodeled words).
    // IDA 0x8ad126..0x8ad148.
    desc.owned.take();
    // IDA 0x8ad150..0x8ad166.
    desc.member.take();
    // IDA 0x8ad186.
    desc.signature.clear();
}

/// was: `RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService, bool
/// ()(shared_ptr<RBX::Instance>, int), bool, 2>` — same layout as
/// [`GamePassYieldDesc`] (IDA 0x8d8ee2 pair at +0x28; 0x8d8ef8/0x8d8f02 zero
/// +0x30/+0x34; D1 0x8cd124/0x8d9088 delete +0x34, scoped dtor +0x30, clear
/// +8 at 0x8cd1a8/0x8cd1b4 and 0x8d910c/0x8d9116).
#[derive(Default)]
pub struct MarketplaceBoolYieldDesc {
    /// Bound member fn (was: member pair (void*, adjust) at +0x28).
    pub method: Option<fn(Option<SharedPtr<Vec<u8>>>, i32) -> bool>,
    /// Scoped member at +0x30 (was: scoped_ptr<shared_ptr<Instance>>).
    pub member: Option<SharedPtr<Vec<u8>>>,
    /// Owned word at +0x34 (deleted when non-null).
    pub owned: Option<Box<u8>>,
    /// Signature items (+8 list).
    pub signature: Vec<YieldSignatureItem>,
    /// Return-type tag (IDA 0x8d904a: Type<bool> stored at +0x1C).
    pub return_type_tag: u8,
}

/// Shared D1 body (IDA 0x8cd124, 0x8d9088): vtable reset, delete +0x34
/// owned when non-null, scoped dtor at +0x30, base vtable reset +
/// signature clear.
fn marketplace_bool_yield_destroy(desc: &mut MarketplaceBoolYieldDesc) {
    // IDA 0x8cd15c/0x8cd1a8: vtable resets (unmodeled words).
    // IDA 0x8cd166..0x8cd188.
    desc.owned.take();
    // IDA 0x8cd194.
    desc.member.take();
    // IDA 0x8cd1b4.
    desc.signature.clear();
}

/// was: `boost::unordered::unordered_map<std::string, RBX::Reflection::Variant,
/// ...>` result/arg map (IDA 0x8d9920). MODEL: unordered_map -> HashMap;
/// Variant payloads kept opaque (Variant unmodeled crate-wide).
pub type VariantMap = HashMap<String, Vec<u8>>;

/// was: `RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,
/// shared_ptr<map<string, Variant> const> ()(int), ..., 1>` — bound member
/// pair at +0x28 (IDA 0x8d9814 `*((_QWORD *)v26 + 5) = v16`), owned member
/// at +0x30 (IDA 0x8cd0fa..0x8cd100 / 0x8d9982..0x8d99a4 delete when
/// non-null), signature list at +8 (IDA 0x8cd11a/0x8d99c2 `_M_clear`).
#[derive(Default)]
pub struct MarketplaceMapYieldDesc {
    /// Bound member fn (was: member pair (void*, adjust) at +0x28).
    pub method: Option<fn(i32) -> SharedPtr<VariantMap>>,
    /// Owned member at +0x30 (deleted when non-null).
    pub owned: Option<Box<u8>>,
    /// Signature items (+8 list).
    pub signature: Vec<YieldSignatureItem>,
    /// Return-type tag (IDA 0x8d9920: Type<map> stored at +0x1C).
    pub return_type_tag: u8,
}

/// Shared D1 body (IDA 0x8cd0e4): vtable reset, delete +0x30 member when
/// non-null (0x8cd0fa..0x8cd100), base vtable reset + signature clear
/// (0x8cd116/0x8cd11a).
fn marketplace_map_yield_destroy(desc: &mut MarketplaceMapYieldDesc) {
    // IDA 0x8cd0f8/0x8cd116: vtable resets (unmodeled words).
    // IDA 0x8cd0fa..0x8cd100.
    desc.owned.take();
    // IDA 0x8cd11a.
    desc.signature.clear();
}

/// was: `boost::enable_shared_from_this<RBX::Reflection::DescribedBase>`
/// weak slot adopted by `_internal_accept_owner<LuaWebService>` (IDA
/// 0x8d0c00): owner word + weak count. MODEL: Arc/Weak pair; the +36
/// pointer adjust (IDA 0x8d0c58..0x8d0c5a) is unmodeled.
#[derive(Default)]
pub struct DescribedWeakSlot {
    /// Owner word (was: adjusted LuaWebService* at [a1]).
    pub owner: Option<SharedPtr<Vec<u8>>>,
    /// Weak count side (was: weak_count at [a1+1]).
    pub weak: Option<Weak<Vec<u8>>>,
}

/// was: `rbx::callable<signal<shared_ptr<Instance>>::slot, bind(mf1
/// ScriptService ...)>` connection slot destroyed at D1 0x8e901c: vtable
/// reset (0x8e902e..0x8e9036, unmodeled) + intrusive_ptr_release of the +8
/// slot when non-null (0x8e903a..0x8e9040). MODEL: dropping the Arc covers
/// the release; in-place destroy keeps the allocation (D1 frees nothing).
#[derive(Default)]
pub struct ScriptServiceSlotCallable {
    /// Connection slot at +8 (was: intrusive_ptr<slot>).
    pub slot: Option<SharedPtr<Vec<u8>>>,
}

// 0x84527c — __ZNK3RBX10Reflection15RemoteEventDescINS_13FriendServiceEFviiNS2_12FriendStatusEEN3rbx13remote_signalIS4_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::FriendService,void ()(int,int,RBX::FriendService::FriendStatus),rbx::remote_signal<void ()(int,int,RBX::FriendService::FriendStatus)>>::isScriptable(void)const")]
// IDA 0x84527c: `LDR R0,[R0,#0x30]; AND #1; BX LR` — scriptable bit.
pub fn stub_0x84527c(flags: &RemoteEventFlags) -> bool {
    // IDA 0x84527c
    remote_event_is_scriptable(flags)
}

// 0x846a1c — __ZNK3RBX10Reflection15RemoteEventDescINS_13FriendServiceEFviiNS2_15FriendEventTypeEEN3rbx13remote_signalIS4_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::FriendService,void ()(int,int,RBX::FriendService::FriendEventType),rbx::remote_signal<void ()(int,int,RBX::FriendService::FriendEventType)>>::isScriptable(void)const")]
// IDA 0x846a1c: identical shape to 0x84527c (`LDR [R0,#0x30]; AND #1`).
pub fn stub_0x846a1c(flags: &RemoteEventFlags) -> bool {
    // IDA 0x846a1c
    remote_event_is_scriptable(flags)
}

// 0x892a54 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_21PersonalServerServiceEFSsiESsLi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::PersonalServerService,std::string ()(int),std::string,1>::~BoundYieldFuncDesc()")]
// IDA 0x892a54 (D1): destroy in place, no free (returns a1, 0x892a90).
pub fn stub_0x892a54(desc: &mut PersonalServerYieldDesc) {
    // IDA 0x892a54
    personal_server_yield_destroy(desc);
}

// 0x89413c — __ZN3RBX10Reflection18BoundYieldFuncDescINS_21PersonalServerServiceEFSsiESsLi1EEC2EMS2_FviN5boost8functionIFvSsEEES8_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, unsigned int, int, int, int, int, int, int)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::PersonalServerService,std::string ()(int),std::string,1>::BoundYieldFuncDesc(void (RBX::PersonalServerService::*)(int,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::PersonalServerService,std::string ()(int),std::string,1>::BoundYieldFuncDesc(void (RBX::PersonalServerService::*)(int,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// IDA 0x89413c (C2): base Described + YieldFunctionDescriptor init
// (0x894174/0x894194); member pair stored at +0x28 (0x8941b8); +0x30 zeroed
// (0x8941c2); return-type void singleton + declareSignature(a5) (0x8941e8..0x8941f8).
// MODEL: vtable installs and the describedClassDescriptor statics unmodeled;
// the (a2,a3) member pair becomes the resolved fn.
pub fn stub_0x89413c(method: fn(i32) -> String, arg_name: String) -> PersonalServerYieldDesc {
    // IDA 0x89413c
    let mut desc = PersonalServerYieldDesc { method: Some(method), ..PersonalServerYieldDesc::default() };
    stub_0x8942b4(&mut desc, arg_name);
    desc
}

// 0x8942b4 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_21PersonalServerServiceEFSsiESsLi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int *)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::PersonalServerService,std::string ()(int),std::string,1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// IDA 0x8942b4: return type = Type<string> (0x8942c4); Name::declare(a2)
// (0x8942ce); arg type = Type<int> (0x8942d0); addArgument (0x8942e2).
// MODEL: type singletons become tags (1 = string, 2 = int).
pub fn stub_0x8942b4(desc: &mut PersonalServerYieldDesc, arg_name: String) {
    // IDA 0x8942b4
    desc.return_type_tag = 1;
    desc.signature.push(YieldSignatureItem { name: arg_name, type_tag: 2 });
}

// 0x8942e4 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_21PersonalServerServiceEFSsiESsLi1EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::PersonalServerService,std::string ()(int),std::string,1>::~BoundYieldFuncDesc() [0x8942e4]")]
// IDA 0x8942e4 (D0): D1 body (0x89431c..0x894366) + operator delete (0x89436c).
// MODEL: consuming Box drops members and frees — same observable.
pub fn stub_0x8942e4(desc: Box<PersonalServerYieldDesc>) {
    // IDA 0x8942e4
    drop(desc);
}

// 0x8943b8 — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_21PersonalServerServiceEFSsiESsLi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsEN5boost8functionIFvNS0_7VariantEEEENSB_IFvSsEEE
// type: void __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, char, int, int, int, char, int, int, int, int, char, int, int, int, int, int, int, int)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::PersonalServerService,std::string ()(int),std::string,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::PersonalServerService,std::string ()(int),std::string,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
// IDA 0x8943b8: target = a2 ? a2-36 : 0 (0x894412..0x894414); member pair at
// +0x28/+0x2C with virtual adjust (0x8943ec..0x89442a); getArg<int,1>
// (0x894440); resume_adapter<string> bound into the string continuation
// (0x894464..0x894470); invoke (0x894492); all four function temps cleared
// (0x89449a..0x8944bc). MODEL: null target is skipped (the original would
// fault the member call); the resume adapter applies `resume` to the result.
pub fn stub_0x8943b8(
    desc: &PersonalServerYieldDesc,
    target_present: bool,
    arg0: i32,
    resume: &dyn Fn(String),
) {
    // IDA 0x8943b8
    if !target_present {
        return;
    }
    if let Some(method) = desc.method {
        let result = method(arg0);
        resume(result);
    }
}

// 0x89f9a4 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_16BindableFunctionEFN5boost10shared_ptrIKNS0_5TupleEEES7_ES7_Li1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::BindableFunction,boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>),boost::shared_ptr<RBX::Reflection::Tuple const>,1>::~BoundYieldFuncDesc()
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::BindableFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,1>::~BoundYieldFuncDesc()")]
// IDA 0x89f9a4 (D1): vtable reset (0x89f9e2); scoped member dtor at +0x30
// (0x89fa08); base vtable reset + signature _M_clear at +8 (0x89fa1c/0x89fa28).
pub fn stub_0x89f9a4(desc: &mut BindableFunctionYieldDesc) {
    // IDA 0x89f9a4
    desc.member.take();
    desc.signature.clear();
}

// 0x8a1acc — __ZN3RBX10Reflection18BoundYieldFuncDescINS_16BindableFunctionEFN5boost10shared_ptrIKNS0_5TupleEEES7_ES7_Li1EEC2EMS2_FvS7_NS3_8functionIFvS7_EEENSA_IFvSsEEEEPKcSI_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, unsigned int, int, int, int, int, int, int)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::BindableFunction,boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>),boost::shared_ptr<RBX::Reflection::Tuple const>,1>::BoundYieldFuncDesc(void (RBX::BindableFunction::*)(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::BindableFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,1>::BoundYieldFuncDesc(void (RBX::BindableFunction::*)(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// IDA 0x8a1acc (C2): base Described + YieldFunctionDescriptor init (0x8a1b04/0x8a1b24); member pair stored at +0x28 (0x8a1b3e); +0x30 zeroed (0x8a1b5a); return-type void singleton + declareSignature(a5) (0x8a1b7e..0x8a1b8e).
pub fn stub_0x8a1acc(
    method: fn(Option<SharedPtr<Vec<u8>>>) -> Option<SharedPtr<Vec<u8>>>,
    arg_name: String,
) -> BindableFunctionYieldDesc {
    // IDA 0x8a1acc
    let mut desc = BindableFunctionYieldDesc { member_method: Some(method), ..BindableFunctionYieldDesc::default() };
    stub_0x8a1c48(&mut desc, arg_name);
    desc
}

// 0x8a1c48 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_16BindableFunctionEFN5boost10shared_ptrIKNS0_5TupleEEES7_ES7_Li1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int *)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::BindableFunction,boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>),boost::shared_ptr<RBX::Reflection::Tuple const>,1>::declareSignature(char const*,RBX::Reflection::Variant)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::BindableFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// IDA 0x8a1c48: return type = Type<Tuple> (0x8a1c58); Name::declare(a2) (0x8a1c62); arg type = Type<Tuple> (0x8a1c64); addArgument (0x8a1c76). MODEL: type singletons become tags (5 = Tuple).
pub fn stub_0x8a1c48(desc: &mut BindableFunctionYieldDesc, arg_name: String) {
    // IDA 0x8a1c48
    desc.return_type_tag = TYPE_TAG_TUPLE;
    desc.signature.push(YieldSignatureItem { name: arg_name, type_tag: TYPE_TAG_TUPLE });
}

// 0x8a1c78 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_16BindableFunctionEFN5boost10shared_ptrIKNS0_5TupleEEES7_ES7_Li1EED0Ev
// type: void __fastcall(_DWORD *)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::BindableFunction,boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>),boost::shared_ptr<RBX::Reflection::Tuple const>,1>::~BoundYieldFuncDesc()
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::BindableFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,1>::~BoundYieldFuncDesc() [0x8a1c78]")]
// IDA 0x8a1c78 (D0): vtable reset (0x8a1cb6); scoped member dtor at +0x30 (0x8a1cdc); base vtable reset + signature clear at +8 (0x8a1cf0/0x8a1cfa); operator delete (0x8a1d00). MODEL: consuming Box drops members and frees — same observable.
pub fn stub_0x8a1c78(desc: Box<BindableFunctionYieldDesc>) {
    // IDA 0x8a1c78
    drop(desc);
}

// 0x8a1d80 — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_16BindableFunctionEFN5boost10shared_ptrIKNS0_5TupleEEES7_ES7_Li1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvNS0_7VariantEEEENSF_IFvSsEEE
// type: void __fastcall(int, int, int, int, int, int, int, char, int, int, int, char, int, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::BindableFunction,boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>),boost::shared_ptr<RBX::Reflection::Tuple const>,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::BindableFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
// IDA 0x8a1d80: target = a2 ? a2-36 : 0 (0x8a1dd6..0x8a1dd8); member pair at +0x28/+0x2C with virtual adjust (0x8a1dac..0x8a1dea); getArg<Tuple,1> (0x8a1dfc); resume_adapter<Tuple> bound into the tuple continuation (0x8a1e22..0x8a1e2e); invoke (0x8a1e50); all four function temps cleared (0x8a1e58..0x8a1e86). MODEL: as stub_0x8943b8.
pub fn stub_0x8a1d80(
    desc: &BindableFunctionYieldDesc,
    target_present: bool,
    arg0: Option<SharedPtr<Vec<u8>>>,
    resume: &dyn Fn(Option<SharedPtr<Vec<u8>>>),
) {
    // IDA 0x8a1d80
    if !target_present {
        return;
    }
    if let Some(method) = desc.member_method {
        let result = method(arg0);
        resume(result);
    }
}

// 0x8ad0e4 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_15GamePassServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::GamePassService,bool ()(boost::shared_ptr<RBX::Instance>,int),bool,2>::~BoundYieldFuncDesc()
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::GamePassService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::~BoundYieldFuncDesc()")]
// IDA 0x8ad0e4 (D1): destroy in place, no free (returns a1, 0x8ad1a8).
pub fn stub_0x8ad0e4(desc: &mut GamePassYieldDesc) {
    // IDA 0x8ad0e4
    game_pass_yield_destroy(desc);
}

// 0x8ada28 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_15GamePassServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EEC2EMS2_FvS6_iNS3_8functionIFvbEEENS9_IFvSsEEEEPKcSH_SH_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, unsigned int, int, int, int, int, int, int, int)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::GamePassService,bool ()(boost::shared_ptr<RBX::Instance>,int),bool,2>::BoundYieldFuncDesc(void (RBX::GamePassService::*)(boost::shared_ptr<RBX::Instance>,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::GamePassService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::BoundYieldFuncDesc(void (RBX::GamePassService::*)(rbx_core::SharedPtr<RBX::Instance>,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// IDA 0x8ada28 (C2): base Described + YieldFunctionDescriptor init; member pair stored at +0x28 (0x8ada9a); +0x30/+0x34 zeroed (0x8adaaa/0x8adab4); return-type void singleton + declareSignature(a5, a6) (0x8adad8..0x8adafc).
pub fn stub_0x8ada28(
    method: fn(Option<SharedPtr<Vec<u8>>>, i32) -> bool,
    arg0_name: String,
    arg1_name: String,
) -> GamePassYieldDesc {
    // IDA 0x8ada28
    let mut desc = GamePassYieldDesc { method: Some(method), ..GamePassYieldDesc::default() };
    stub_0x8adc10(&mut desc, arg0_name, arg1_name);
    desc
}

// 0x8adc10 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_15GamePassServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EE16declareSignatureEPKcNS0_7VariantESA_SB_
// type: int __fastcall(int, int, int *, int, int *)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::GamePassService,bool ()(boost::shared_ptr<RBX::Instance>,int),bool,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::GamePassService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// IDA 0x8adc10: return type = Type<bool> (0x8adc1e); Name::declare(a2) + arg type = Type<Instance> (0x8adc2c..0x8adc2e); addArgument (0x8adc3a); Name::declare(a4) + Type<int> (0x8adc44..0x8adc46); addArgument (0x8adc58). MODEL: tags (3 = bool, 4 = Instance, 2 = int).
pub fn stub_0x8adc10(desc: &mut GamePassYieldDesc, arg0_name: String, arg1_name: String) {
    // IDA 0x8adc10
    desc.return_type_tag = TYPE_TAG_BOOL;
    desc.signature.push(YieldSignatureItem { name: arg0_name, type_tag: TYPE_TAG_INSTANCE });
    desc.signature.push(YieldSignatureItem { name: arg1_name, type_tag: 2 });
}

// 0x8adc5c — __ZN3RBX10Reflection18BoundYieldFuncDescINS_15GamePassServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EED0Ev
// type: void __fastcall(_DWORD *)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::GamePassService,bool ()(boost::shared_ptr<RBX::Instance>,int),bool,2>::~BoundYieldFuncDesc()
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::GamePassService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::~BoundYieldFuncDesc() [0x8adc5c]")]
// IDA 0x8adc5c (D0): D1 body (0x8adc94..0x8adcfc) + operator delete (0x8add02). MODEL: consuming Box drops members and frees — same observable.
pub fn stub_0x8adc5c(desc: Box<GamePassYieldDesc>) {
    // IDA 0x8adc5c
    drop(desc);
}

// 0x8add88 — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_15GamePassServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvNS0_7VariantEEEENSE_IFvSsEEE
// type: void __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, char, int, int, int, char, int, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::GamePassService,bool ()(boost::shared_ptr<RBX::Instance>,int),bool,2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::GamePassService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
// IDA 0x8add88: target = a2 ? a2-36 : 0 (0x8adde2..0x8adde4); member pair at +0x28/+0x2C with virtual adjust (0x8addb6..0x8addf6); getArg<Instance,1> (0x8ade0c) + getArg<int,2> (0x8ade1e); resume_adapter<bool> (0x8ade42..0x8ade4e); invoke (0x8ade74); temps cleared (0x8ade7c..0x8adeaa). MODEL: as stub_0x8943b8.
pub fn stub_0x8add88(
    desc: &GamePassYieldDesc,
    target_present: bool,
    arg0: Option<SharedPtr<Vec<u8>>>,
    arg1: i32,
    resume: &dyn Fn(bool),
) {
    // IDA 0x8add88
    if !target_present {
        return;
    }
    if let Some(method) = desc.method {
        let result = method(arg0, arg1);
        resume(result);
    }
}

// 0x8cd0e4 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_18MarketplaceServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEiESI_Li1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::~BoundYieldFuncDesc()
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::~BoundYieldFuncDesc()")]
// IDA 0x8cd0e4 (D1): vtable reset (0x8cd0f8); delete +0x30 member when non-null (0x8cd0fa..0x8cd100); base vtable reset + signature clear (0x8cd116/0x8cd11a). Returns a1, no free (0x8cd120).
pub fn stub_0x8cd0e4(desc: &mut MarketplaceMapYieldDesc) {
    // IDA 0x8cd0e4
    marketplace_map_yield_destroy(desc);
}

// 0x8cd124 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_18MarketplaceServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(boost::shared_ptr<RBX::Instance>,int),bool,2>::~BoundYieldFuncDesc()
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::~BoundYieldFuncDesc()")]
// IDA 0x8cd124 (D1): vtable reset (0x8cd15c); delete +0x34 owned when non-null (0x8cd166..0x8cd188); scoped dtor at +0x30 (0x8cd194); base vtable reset + signature clear (0x8cd1a8/0x8cd1b4). Returns a1, no free (0x8cd1d6).
pub fn stub_0x8cd124(desc: &mut MarketplaceBoolYieldDesc) {
    // IDA 0x8cd124
    marketplace_bool_yield_destroy(desc);
}

// 0x8d0c00 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13LuaWebServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LuaWebService,RBX::LuaWebService>(boost::shared_ptr<RBX::LuaWebService> const*,RBX::LuaWebService *)const
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LuaWebService,RBX::LuaWebService>(rbx_core::SharedPtr<RBX::LuaWebService> const*,RBX::LuaWebService *)const")]
// IDA 0x8d0c00: if weak expired (0x8d0c54), store owner (0x8d0c58..0x8d0c5a; the +36 adjust unmodeled) + copy shared count into the weak slot (0x8d0c6e..0x8d0c80).
pub fn stub_0x8d0c00(slot: &mut DescribedWeakSlot, owner: Option<SharedPtr<Vec<u8>>>) {
    // IDA 0x8d0c00
    if slot.weak.as_ref().and_then(Weak::upgrade).is_some() {
        return;
    }
    slot.owner = owner;
    slot.weak = slot.owner.as_ref().map(SharedPtr::downgrade);
}

// 0x8d4958 — __ZNK3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFviibEN3rbx13remote_signalIS3_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(int,int,bool),rbx::remote_signal<void ()(int,int,bool)>>::isScriptable(void)const")]
// IDA 0x8d4958: `LDR R0,[R0,#0x30]; AND #1` — scriptable bit (same shape as 0x84527c).
pub fn stub_0x8d4958(flags: &RemoteEventFlags) -> bool {
    // IDA 0x8d4958
    remote_event_is_scriptable(flags)
}

// 0x8d6808 — __ZNK3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibEN3rbx13remote_signalIS7_EEE12isScriptableEv
// type: int __fastcall(int)
// was: RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool)>>::isScriptable(void)const
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool)>>::isScriptable(void)const")]
// IDA 0x8d6808: `LDR R0,[R0,#0x30]; AND #1` — scriptable bit (same shape as 0x84527c).
pub fn stub_0x8d6808(flags: &RemoteEventFlags) -> bool {
    // IDA 0x8d6808
    remote_event_is_scriptable(flags)
}

// 0x8d8e70 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_18MarketplaceServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EEC2EMS2_FvS6_iNS3_8functionIFvbEEENS9_IFvSsEEEEPKcSH_SH_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, unsigned int, int, int, int, int, int, int, int)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(boost::shared_ptr<RBX::Instance>,int),bool,2>::BoundYieldFuncDesc(void (RBX::MarketplaceService::*)(boost::shared_ptr<RBX::Instance>,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::BoundYieldFuncDesc(void (RBX::MarketplaceService::*)(rbx_core::SharedPtr<RBX::Instance>,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// IDA 0x8d8e70 (C2): base Described + YieldFunctionDescriptor init; member pair stored at +0x28 (0x8d8ee2); +0x30/+0x34 zeroed (0x8d8ef8/0x8d8f02); return-type void singleton + declareSignature(a5, a6) (0x8d8f26..0x8d8f4a).
pub fn stub_0x8d8e70(
    method: fn(Option<SharedPtr<Vec<u8>>>, i32) -> bool,
    arg0_name: String,
    arg1_name: String,
) -> MarketplaceBoolYieldDesc {
    // IDA 0x8d8e70
    let mut desc = MarketplaceBoolYieldDesc { method: Some(method), ..MarketplaceBoolYieldDesc::default() };
    stub_0x8d903c(&mut desc, arg0_name, arg1_name);
    desc
}

// 0x8d903c — __ZN3RBX10Reflection18BoundYieldFuncDescINS_18MarketplaceServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EE16declareSignatureEPKcNS0_7VariantESA_SB_
// type: int __fastcall(int, int, int *, int, int *)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(boost::shared_ptr<RBX::Instance>,int),bool,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// IDA 0x8d903c: return type = Type<bool> (0x8d904a); Name::declare(a2) + arg type = Type<Instance> (0x8d9058..0x8d905a); addArgument (0x8d9066); Name::declare(a4) + Type<int> (0x8d9070..0x8d9072); addArgument (0x8d9084). MODEL: tags (3 = bool, 4 = Instance, 2 = int).
pub fn stub_0x8d903c(desc: &mut MarketplaceBoolYieldDesc, arg0_name: String, arg1_name: String) {
    // IDA 0x8d903c
    desc.return_type_tag = TYPE_TAG_BOOL;
    desc.signature.push(YieldSignatureItem { name: arg0_name, type_tag: TYPE_TAG_INSTANCE });
    desc.signature.push(YieldSignatureItem { name: arg1_name, type_tag: 2 });
}

// 0x8d9088 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_18MarketplaceServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EED0Ev
// type: void __fastcall(_DWORD *)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(boost::shared_ptr<RBX::Instance>,int),bool,2>::~BoundYieldFuncDesc()
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::~BoundYieldFuncDesc() [0x8d9088]")]
// IDA 0x8d9088 (D0): D1 body (0x8d90c0..0x8d9116) + operator delete (0x8d911c). MODEL: consuming Box drops members and frees — same observable.
pub fn stub_0x8d9088(desc: Box<MarketplaceBoolYieldDesc>) {
    // IDA 0x8d9088
    drop(desc);
}

// 0x8d919c — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_18MarketplaceServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvNS0_7VariantEEEENSE_IFvSsEEE
// type: void __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, char, int, int, int, char, int, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(boost::shared_ptr<RBX::Instance>,int),bool,2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
// IDA 0x8d919c: target = a2 ? a2-36 : 0 (0x8d91f6..0x8d91f8); member pair at +0x28/+0x2C with virtual adjust (0x8d91ca..0x8d920a); getArg<Instance,1> (0x8d9220) + getArg<int,2> (0x8d9232); resume_adapter<bool> (0x8d9256..0x8d9262); invoke (0x8d9288); temps cleared (0x8d9290..0x8d92be). MODEL: as stub_0x8943b8.
pub fn stub_0x8d919c(
    desc: &MarketplaceBoolYieldDesc,
    target_present: bool,
    arg0: Option<SharedPtr<Vec<u8>>>,
    arg1: i32,
    resume: &dyn Fn(bool),
) {
    // IDA 0x8d919c
    if !target_present {
        return;
    }
    if let Some(method) = desc.method {
        let result = method(arg0, arg1);
        resume(result);
    }
}

// 0x8d9798 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_18MarketplaceServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEiESI_Li1EEC2EMS2_FviNS3_8functionIFvSI_EEENSL_IFvSsEEEEPKcST_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, unsigned int, int, int, int, int, int, int)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::BoundYieldFuncDesc(void (RBX::MarketplaceService::*)(int,boost::function<void ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::BoundYieldFuncDesc(void (RBX::MarketplaceService::*)(int,boost::function<void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// IDA 0x8d9798 (C2): base Described + YieldFunctionDescriptor init; member pair stored at +0x28 (0x8d9814); +0x30 zeroed (0x8d981e); return-type void singleton + declareSignature(a5) (0x8d9844..0x8d9854).
pub fn stub_0x8d9798(method: fn(i32) -> SharedPtr<VariantMap>, arg_name: String) -> MarketplaceMapYieldDesc {
    // IDA 0x8d9798
    let mut desc = MarketplaceMapYieldDesc { method: Some(method), ..MarketplaceMapYieldDesc::default() };
    stub_0x8d9910(&mut desc, arg_name);
    desc
}

// 0x8d9910 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_18MarketplaceServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEiESI_Li1EE16declareSignatureEPKcS7_
// type: int __fastcall(int, int, int *)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::declareSignature(char const*,RBX::Reflection::Variant)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// IDA 0x8d9910: return type = Type<map> (0x8d9920); Name::declare(a2) (0x8d992a); arg type = Type<int> (0x8d992c); addArgument (0x8d993e). MODEL: tags (6 = Variant map, 2 = int).
pub fn stub_0x8d9910(desc: &mut MarketplaceMapYieldDesc, arg_name: String) {
    // IDA 0x8d9910
    desc.return_type_tag = TYPE_TAG_VARIANT_MAP;
    desc.signature.push(YieldSignatureItem { name: arg_name, type_tag: 2 });
}

// 0x8d9940 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_18MarketplaceServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEiESI_Li1EED0Ev
// type: void __fastcall(_DWORD *)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::~BoundYieldFuncDesc()
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::~BoundYieldFuncDesc() [0x8d9940]")]
// IDA 0x8d9940 (D0): D1 body (0x8d9978..0x8d99c2) + operator delete (0x8d99c8). MODEL: consuming Box drops members and frees — same observable.
pub fn stub_0x8d9940(desc: Box<MarketplaceMapYieldDesc>) {
    // IDA 0x8d9940
    drop(desc);
}

// 0x8d9a14 — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_18MarketplaceServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEiESI_Li1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvS7_EEENSQ_IFvSsEEE
// type: void __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, char, int, int, int, char, int, int, int, int, char, int, int, int, int, int, int, int)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
// IDA 0x8d9a14: target = a2 ? a2-36 : 0 (0x8d9a6e..0x8d9a70); member pair at +0x28/+0x2C with virtual adjust (0x8d9a48..0x8d9a86); getArg<int,1> (0x8d9a9c); resume_adapter<map> bound into the map continuation (0x8d9ac0..0x8d9acc); invoke (0x8d9aee); all four function temps cleared (0x8d9af6..0x8d9b18). MODEL: as stub_0x8943b8.
pub fn stub_0x8d9a14(
    desc: &MarketplaceMapYieldDesc,
    target_present: bool,
    arg0: i32,
    resume: &dyn Fn(SharedPtr<VariantMap>),
) {
    // IDA 0x8d9a14
    if !target_present {
        return;
    }
    if let Some(method) = desc.method {
        let result = method(arg0);
        resume(result);
    }
}

// 0x8d9fd4 — __ZNK3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEEN3rbx13remote_signalISJ_EEE12isScriptableEv
// type: int __fastcall(int)
// was: RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),rbx::remote_signal<void ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>::isScriptable(void)const
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),rbx::remote_signal<void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>::isScriptable(void)const")]
// IDA 0x8d9fd4: `LDR R0,[R0,#0x30]; AND #1` — scriptable bit (same shape as 0x84527c).
pub fn stub_0x8d9fd4(flags: &RemoteEventFlags) -> bool {
    // IDA 0x8d9fd4
    remote_event_is_scriptable(flags)
}

// 0x8dbcac — __ZNK3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFvSsiiEN3rbx13remote_signalIS3_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(std::string,int,int),rbx::remote_signal<void ()(std::string,int,int)>>::isScriptable(void)const")]
// IDA 0x8dbcac: `LDR R0,[R0,#0x30]; AND #1` — scriptable bit (same shape as 0x84527c).
pub fn stub_0x8dbcac(flags: &RemoteEventFlags) -> bool {
    // IDA 0x8dbcac
    remote_event_is_scriptable(flags)
}

// 0x8dd444 — __ZNK3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEE12isScriptableEv
// type: int __fastcall(int)
// was: RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::isScriptable(void)const
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::isScriptable(void)const")]
// IDA 0x8dd444: `LDR R0,[R0,#0x30]; AND #1` — scriptable bit (same shape as 0x84527c).
pub fn stub_0x8dd444(flags: &RemoteEventFlags) -> bool {
    // IDA 0x8dd444
    remote_event_is_scriptable(flags)
}

// 0x8e8f34 — __ZNK5boost4_mfi3mf1IvN3RBX13ScriptServiceENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_
// type: void __fastcall(char **, int, const shared_count *)
// was: boost::_mfi::mf1<void,RBX::ScriptService,boost::shared_ptr<RBX::Instance>>::operator()(RBX::ScriptService*,boost::shared_ptr<RBX::Instance>)const
#[doc(alias = "boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::ScriptService*,rbx_core::SharedPtr<RBX::Instance>)const")]
// IDA 0x8e8f34: resolve member fn + virtual adjust (0x8e8f60..0x8e8f92, unmodeled); shared-copy arg (0x8e8f98..0x8e8faa); invoke (0x8e8fb4); release (0x8e8fb8..0x8e8fc0). MODEL: the copy/release is the Arc clone/drop.
pub fn stub_0x8e8f34(method: fn(Option<SharedPtr<Vec<u8>>>), arg: Option<SharedPtr<Vec<u8>>>) {
    // IDA 0x8e8f34
    method(arg);
}

// 0x8e901c — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_13ScriptServiceES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_ED1Ev
// type: int __fastcall(int)
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// IDA 0x8e901c (D1): vtable reset (0x8e902e..0x8e9036, unmodeled) + intrusive_ptr_release of the +8 slot when non-null (0x8e903a..0x8e9040). MODEL: dropping the Arc covers the release; D1 frees nothing.
pub fn stub_0x8e901c(callable: &mut ScriptServiceSlotCallable) {
    // IDA 0x8e901c
    callable.slot.take();
}
