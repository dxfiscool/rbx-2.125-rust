// Auto-generated skeletons for rbx-script — Lua|Script|Yield|lua filtered batch
// Filter: Lua|Script|Yield|lua (5401 filtered, 100 in this batch)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x831ff4..0x8e901c | 4011->4111 covered, 1290 remaining, rbx_core::SharedPtr not boost
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

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
pub fn stub_0x8a1acc() -> ! {
    todo!("0x8a1acc __ZN3RBX10Reflection18BoundYieldFuncDescINS_16BindableFunctionEFN5boost10shared_ptrIKNS0_5TupleEEES7_ES7_Li1EEC2EMS2_FvS7_NS3_8functionIFvS7_EEENSA_IFvSsEEEEPKcSI_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

// 0x8a1c48 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_16BindableFunctionEFN5boost10shared_ptrIKNS0_5TupleEEES7_ES7_Li1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int *)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::BindableFunction,boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>),boost::shared_ptr<RBX::Reflection::Tuple const>,1>::declareSignature(char const*,RBX::Reflection::Variant)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::BindableFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x8a1c48() -> ! {
    todo!("0x8a1c48 __ZN3RBX10Reflection18BoundYieldFuncDescINS_16BindableFunctionEFN5boost10shared_ptrIKNS0_5TupleEEES7_ES7_Li1EE16declareSignatureEPKcNS0_7VariantE")
}

// 0x8a1c78 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_16BindableFunctionEFN5boost10shared_ptrIKNS0_5TupleEEES7_ES7_Li1EED0Ev
// type: void __fastcall(_DWORD *)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::BindableFunction,boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>),boost::shared_ptr<RBX::Reflection::Tuple const>,1>::~BoundYieldFuncDesc()
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::BindableFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,1>::~BoundYieldFuncDesc() [0x8a1c78]")]
pub fn stub_0x8a1c78() -> ! {
    todo!("0x8a1c78 __ZN3RBX10Reflection18BoundYieldFuncDescINS_16BindableFunctionEFN5boost10shared_ptrIKNS0_5TupleEEES7_ES7_Li1EED0Ev")
}

// 0x8a1d80 — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_16BindableFunctionEFN5boost10shared_ptrIKNS0_5TupleEEES7_ES7_Li1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvNS0_7VariantEEEENSF_IFvSsEEE
// type: void __fastcall(int, int, int, int, int, int, int, char, int, int, int, char, int, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::BindableFunction,boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>),boost::shared_ptr<RBX::Reflection::Tuple const>,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::BindableFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
pub fn stub_0x8a1d80() -> ! {
    todo!("0x8a1d80 __ZNK3RBX10Reflection18BoundYieldFuncDescINS_16BindableFunctionEFN5boost10shared_ptrIKNS0_5TupleEEES7_ES7_Li1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvNS0_7VariantEEEENSF_IFvSsEEE")
}

// 0x8ad0e4 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_15GamePassServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::GamePassService,bool ()(boost::shared_ptr<RBX::Instance>,int),bool,2>::~BoundYieldFuncDesc()
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::GamePassService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::~BoundYieldFuncDesc()")]
pub fn stub_0x8ad0e4() -> ! {
    todo!("0x8ad0e4 __ZN3RBX10Reflection18BoundYieldFuncDescINS_15GamePassServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EED1Ev")
}

// 0x8ada28 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_15GamePassServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EEC2EMS2_FvS6_iNS3_8functionIFvbEEENS9_IFvSsEEEEPKcSH_SH_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, unsigned int, int, int, int, int, int, int, int)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::GamePassService,bool ()(boost::shared_ptr<RBX::Instance>,int),bool,2>::BoundYieldFuncDesc(void (RBX::GamePassService::*)(boost::shared_ptr<RBX::Instance>,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::GamePassService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::BoundYieldFuncDesc(void (RBX::GamePassService::*)(rbx_core::SharedPtr<RBX::Instance>,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x8ada28() -> ! {
    todo!("0x8ada28 __ZN3RBX10Reflection18BoundYieldFuncDescINS_15GamePassServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EEC2EMS2_FvS6_iNS3_8functionIFvbEEENS9_IFvSsEEEEPKcSH_SH_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

// 0x8adc10 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_15GamePassServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EE16declareSignatureEPKcNS0_7VariantESA_SB_
// type: int __fastcall(int, int, int *, int, int *)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::GamePassService,bool ()(boost::shared_ptr<RBX::Instance>,int),bool,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::GamePassService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_0x8adc10() -> ! {
    todo!("0x8adc10 __ZN3RBX10Reflection18BoundYieldFuncDescINS_15GamePassServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EE16declareSignatureEPKcNS0_7VariantESA_SB_")
}

// 0x8adc5c — __ZN3RBX10Reflection18BoundYieldFuncDescINS_15GamePassServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EED0Ev
// type: void __fastcall(_DWORD *)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::GamePassService,bool ()(boost::shared_ptr<RBX::Instance>,int),bool,2>::~BoundYieldFuncDesc()
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::GamePassService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::~BoundYieldFuncDesc() [0x8adc5c]")]
pub fn stub_0x8adc5c() -> ! {
    todo!("0x8adc5c __ZN3RBX10Reflection18BoundYieldFuncDescINS_15GamePassServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EED0Ev")
}

// 0x8add88 — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_15GamePassServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvNS0_7VariantEEEENSE_IFvSsEEE
// type: void __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, char, int, int, int, char, int, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::GamePassService,bool ()(boost::shared_ptr<RBX::Instance>,int),bool,2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::GamePassService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
pub fn stub_0x8add88() -> ! {
    todo!("0x8add88 __ZNK3RBX10Reflection18BoundYieldFuncDescINS_15GamePassServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvNS0_7VariantEEEENSE_IFvSsEEE")
}

// 0x8cd0e4 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_18MarketplaceServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEiESI_Li1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::~BoundYieldFuncDesc()
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::~BoundYieldFuncDesc()")]
pub fn stub_0x8cd0e4() -> ! {
    todo!("0x8cd0e4 __ZN3RBX10Reflection18BoundYieldFuncDescINS_18MarketplaceServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEiESI_Li1EED1Ev")
}

// 0x8cd124 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_18MarketplaceServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(boost::shared_ptr<RBX::Instance>,int),bool,2>::~BoundYieldFuncDesc()
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::~BoundYieldFuncDesc()")]
pub fn stub_0x8cd124() -> ! {
    todo!("0x8cd124 __ZN3RBX10Reflection18BoundYieldFuncDescINS_18MarketplaceServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EED1Ev")
}

// 0x8d0c00 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13LuaWebServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LuaWebService,RBX::LuaWebService>(boost::shared_ptr<RBX::LuaWebService> const*,RBX::LuaWebService *)const
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LuaWebService,RBX::LuaWebService>(rbx_core::SharedPtr<RBX::LuaWebService> const*,RBX::LuaWebService *)const")]
pub fn stub_0x8d0c00() -> ! {
    todo!("0x8d0c00 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13LuaWebServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

// 0x8d4958 — __ZNK3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFviibEN3rbx13remote_signalIS3_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(int,int,bool),rbx::remote_signal<void ()(int,int,bool)>>::isScriptable(void)const")]
pub fn stub_0x8d4958() -> ! {
    todo!("0x8d4958 __ZNK3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFviibEN3rbx13remote_signalIS3_EEE12isScriptableEv")
}

// 0x8d6808 — __ZNK3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibEN3rbx13remote_signalIS7_EEE12isScriptableEv
// type: int __fastcall(int)
// was: RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool)>>::isScriptable(void)const
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool)>>::isScriptable(void)const")]
pub fn stub_0x8d6808() -> ! {
    todo!("0x8d6808 __ZNK3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibEN3rbx13remote_signalIS7_EEE12isScriptableEv")
}

// 0x8d8e70 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_18MarketplaceServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EEC2EMS2_FvS6_iNS3_8functionIFvbEEENS9_IFvSsEEEEPKcSH_SH_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, unsigned int, int, int, int, int, int, int, int)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(boost::shared_ptr<RBX::Instance>,int),bool,2>::BoundYieldFuncDesc(void (RBX::MarketplaceService::*)(boost::shared_ptr<RBX::Instance>,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::BoundYieldFuncDesc(void (RBX::MarketplaceService::*)(rbx_core::SharedPtr<RBX::Instance>,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x8d8e70() -> ! {
    todo!("0x8d8e70 __ZN3RBX10Reflection18BoundYieldFuncDescINS_18MarketplaceServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EEC2EMS2_FvS6_iNS3_8functionIFvbEEENS9_IFvSsEEEEPKcSH_SH_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

// 0x8d903c — __ZN3RBX10Reflection18BoundYieldFuncDescINS_18MarketplaceServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EE16declareSignatureEPKcNS0_7VariantESA_SB_
// type: int __fastcall(int, int, int *, int, int *)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(boost::shared_ptr<RBX::Instance>,int),bool,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_0x8d903c() -> ! {
    todo!("0x8d903c __ZN3RBX10Reflection18BoundYieldFuncDescINS_18MarketplaceServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EE16declareSignatureEPKcNS0_7VariantESA_SB_")
}

// 0x8d9088 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_18MarketplaceServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EED0Ev
// type: void __fastcall(_DWORD *)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(boost::shared_ptr<RBX::Instance>,int),bool,2>::~BoundYieldFuncDesc()
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::~BoundYieldFuncDesc() [0x8d9088]")]
pub fn stub_0x8d9088() -> ! {
    todo!("0x8d9088 __ZN3RBX10Reflection18BoundYieldFuncDescINS_18MarketplaceServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EED0Ev")
}

// 0x8d919c — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_18MarketplaceServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvNS0_7VariantEEEENSE_IFvSsEEE
// type: void __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, char, int, int, int, char, int, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(boost::shared_ptr<RBX::Instance>,int),bool,2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
pub fn stub_0x8d919c() -> ! {
    todo!("0x8d919c __ZNK3RBX10Reflection18BoundYieldFuncDescINS_18MarketplaceServiceEFbN5boost10shared_ptrINS_8InstanceEEEiEbLi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvNS0_7VariantEEEENSE_IFvSsEEE")
}

// 0x8d9798 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_18MarketplaceServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEiESI_Li1EEC2EMS2_FviNS3_8functionIFvSI_EEENSL_IFvSsEEEEPKcST_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, unsigned int, int, int, int, int, int, int)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::BoundYieldFuncDesc(void (RBX::MarketplaceService::*)(int,boost::function<void ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::BoundYieldFuncDesc(void (RBX::MarketplaceService::*)(int,boost::function<void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x8d9798() -> ! {
    todo!("0x8d9798 __ZN3RBX10Reflection18BoundYieldFuncDescINS_18MarketplaceServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEiESI_Li1EEC2EMS2_FviNS3_8functionIFvSI_EEENSL_IFvSsEEEEPKcST_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

// 0x8d9910 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_18MarketplaceServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEiESI_Li1EE16declareSignatureEPKcS7_
// type: int __fastcall(int, int, int *)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::declareSignature(char const*,RBX::Reflection::Variant)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x8d9910() -> ! {
    todo!("0x8d9910 __ZN3RBX10Reflection18BoundYieldFuncDescINS_18MarketplaceServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEiESI_Li1EE16declareSignatureEPKcS7_")
}

// 0x8d9940 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_18MarketplaceServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEiESI_Li1EED0Ev
// type: void __fastcall(_DWORD *)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::~BoundYieldFuncDesc()
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::~BoundYieldFuncDesc() [0x8d9940]")]
pub fn stub_0x8d9940() -> ! {
    todo!("0x8d9940 __ZN3RBX10Reflection18BoundYieldFuncDescINS_18MarketplaceServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEiESI_Li1EED0Ev")
}

// 0x8d9a14 — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_18MarketplaceServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEiESI_Li1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvS7_EEENSQ_IFvSsEEE
// type: void __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, char, int, int, int, char, int, int, int, int, char, int, int, int, int, int, int, int)
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
pub fn stub_0x8d9a14() -> ! {
    todo!("0x8d9a14 __ZNK3RBX10Reflection18BoundYieldFuncDescINS_18MarketplaceServiceEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEiESI_Li1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvS7_EEENSQ_IFvSsEEE")
}

// 0x8d9fd4 — __ZNK3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEEN3rbx13remote_signalISJ_EEE12isScriptableEv
// type: int __fastcall(int)
// was: RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),rbx::remote_signal<void ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>::isScriptable(void)const
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),rbx::remote_signal<void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>::isScriptable(void)const")]
pub fn stub_0x8d9fd4() -> ! {
    todo!("0x8d9fd4 __ZNK3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEEN3rbx13remote_signalISJ_EEE12isScriptableEv")
}

// 0x8dbcac — __ZNK3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFvSsiiEN3rbx13remote_signalIS3_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(std::string,int,int),rbx::remote_signal<void ()(std::string,int,int)>>::isScriptable(void)const")]
pub fn stub_0x8dbcac() -> ! {
    todo!("0x8dbcac __ZNK3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFvSsiiEN3rbx13remote_signalIS3_EEE12isScriptableEv")
}

// 0x8dd444 — __ZNK3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEE12isScriptableEv
// type: int __fastcall(int)
// was: RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::isScriptable(void)const
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::isScriptable(void)const")]
pub fn stub_0x8dd444() -> ! {
    todo!("0x8dd444 __ZNK3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEE12isScriptableEv")
}

// 0x8e8f34 — __ZNK5boost4_mfi3mf1IvN3RBX13ScriptServiceENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_
// type: void __fastcall(char **, int, const shared_count *)
// was: boost::_mfi::mf1<void,RBX::ScriptService,boost::shared_ptr<RBX::Instance>>::operator()(RBX::ScriptService*,boost::shared_ptr<RBX::Instance>)const
#[doc(alias = "boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::ScriptService*,rbx_core::SharedPtr<RBX::Instance>)const")]
pub fn stub_0x8e8f34() -> ! {
    todo!("0x8e8f34 __ZNK5boost4_mfi3mf1IvN3RBX13ScriptServiceENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_")
}

// 0x8e901c — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_13ScriptServiceES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_ED1Ev
// type: int __fastcall(int)
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
pub fn stub_0x8e901c() -> ! {
    todo!("0x8e901c __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_13ScriptServiceES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_ED1Ev")
}
