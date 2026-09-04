//! Auto-generated skeletons for rbx-script — watchdog w12 Script/Lua/ScriptContext
//! Filter: demangled/mangled contains Lua|Script|ScriptContext (case-sensitive), EA-sorted asc, slice [1200:1320)
//! Source: ida/export.json (85545 funcs, base 0x4000) — SKIP global dedup per task
//! Batch: +120 stubs | range 0x2bbd78..0x2c864c | EA-sorted asc, UNIQUE within file
//! SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; // 0xADDR mangled + #[doc(alias)] + todo!("0xADDR")

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
use rbx_core::signal::Signal;
use parking_lot::Mutex;
use std::sync::{Arc, Weak};
const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

// ---- Batch model: ScriptContext reflection descriptors + Lua thread refs ----
// IDA ground truth per stub below (decompile + disasm via IDA MCP).
// Unmodeled throughout: C++ vtable installs/virtual-adjust thunks, RTTI
// dynamic_cast plumbing, std::string copy-on-write internals (Clone covers
// the observable copy), and Lua error raises on bad arg types (defaults).

/// was: RBX::Instance — opaque payload of the bound methods below.
#[derive(Debug, Default)]
pub struct ScriptInstance;

/// was: RBX::ScriptContext — dispatch target of the bound methods below.
#[derive(Debug, Default)]
pub struct ScriptContextState;

/// was: RBX::Lua::detail::LiveThreadRef — opaque; counter at +0 (IDA 0x2c190c).
#[derive(Debug, Default)]
pub struct LiveThreadRefDetail;

/// was: RBX::Lua::WeakThreadRef::Node — opaque; counter at +4 (IDA 0x2c5ec8).
#[derive(Debug, Default)]
pub struct WeakThreadRefNode;

/// was: boost::function<SharedPtr<Tuple const>(SharedPtr<Tuple const>)> — opaque.
#[derive(Debug, Default)]
pub struct GenericFunction;

/// was: boost::function<void(SharedPtr<Tuple const>,function<void(IAsyncResult*)>)> — opaque.
#[derive(Debug, Default)]
pub struct GenericAsyncFunction;

/// was: RBX::LuaStatsItem — Lua stats node owned by its ScriptContext.
/// IDA 0x2c1af8 news 0x80 bytes and runs LuaStatsItem::LuaStatsItem(ctx);
/// only the owning context is modeled.
#[derive(Debug)]
pub struct LuaStatsItem {
    pub context: SharedPtr<ScriptContextState>,
}

/// was: RBX::DataModel — opaque owner held by the jobs below.
#[derive(Debug, Default)]
pub struct DataModelMarker;

/// Lua string-push sink: the lua_State side of Bridge::on_tostring
/// (lua_pushlstring target, IDA 0x2c59b0/0x2c59cc/0x2c59e8).
#[derive(Debug, Default)]
pub struct LuaPushStack {
    pub pushed: Vec<Vec<u8>>,
}

impl LuaPushStack {
    pub fn push_lstring(&mut self, s: &[u8]) {
        self.pushed.push(s.to_vec());
    }
}

/// was: RBX::Reflection::Type id of a descriptor argument.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScriptArgKind {
    Void,
    Int,
    Instance,
    Str,
}

/// was: SignatureDescriptor::Item (name + type).
#[derive(Clone, Debug)]
pub struct ScriptArg {
    pub name: String,
    pub kind: ScriptArgKind,
}

/// was: FunctionDescriptor::Arguments item (untyped Variant slot).
#[derive(Clone, Debug, Default)]
pub enum ScriptVariant {
    #[default]
    Nil,
    Int(i32),
    Instance(Option<SharedPtr<ScriptInstance>>),
    Str(String),
}

impl ScriptVariant {
    pub fn as_int(&self) -> Option<i32> {
        if let ScriptVariant::Int(v) = self {
            Some(*v)
        } else {
            None
        }
    }
    pub fn as_instance(&self) -> Option<SharedPtr<ScriptInstance>> {
        if let ScriptVariant::Instance(v) = self {
            v.clone()
        } else {
            None
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        if let ScriptVariant::Str(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

/// was: void (ScriptContext::*)(int, shared_ptr<Instance>, string).
pub type ScriptMethod3 =
    fn(&mut ScriptContextState, i32, Option<SharedPtr<ScriptInstance>>, String);

/// was: void (ScriptContext::*)(string, shared_ptr<Instance>).
pub type ScriptMethod2 =
    fn(&mut ScriptContextState, String, Option<SharedPtr<ScriptInstance>>);

/// was: BoundFuncDesc<ScriptContext, void(int,shared_ptr<Instance>,string), 3>.
pub struct BoundFuncDesc3 {
    pub name: String,
    pub permissions: u32,
    pub attributes: u32,
    pub method: ScriptMethod3,
    pub signature: Vec<ScriptArg>,
}

/// was: BoundFuncDesc<ScriptContext, void(string,shared_ptr<Instance>), 2>.
pub struct BoundFuncDesc2 {
    pub name: String,
    pub permissions: u32,
    pub attributes: u32,
    pub method: ScriptMethod2,
    pub signature: Vec<ScriptArg>,
}

/// was: (shared_ptr<Instance>, string, shared_ptr<Instance>) event triple.
#[derive(Clone, Debug, Default)]
pub struct ScriptContextEvent(
    pub Option<SharedPtr<ScriptInstance>>,
    pub String,
    pub Option<SharedPtr<ScriptInstance>>,
);

/// was: RBX::Reflection::GenericSlotWrapper (execute3 target, IDA 0x2be3f8).
/// MODEL: records each delivery; the real wrapper forwards into Lua.
#[derive(Debug, Default)]
pub struct GenericSlot3 {
    pub delivered: Mutex<Vec<ScriptContextEvent>>,
}

/// was: RBX::Reflection::GenericSlotWrapper::execute3<...>.
pub fn generic_slot_execute3(slot: &GenericSlot3, args: ScriptContextEvent) {
    slot.delivered.lock().push(args);
}

/// was: EventDesc<ScriptContext, void(shared_ptr<Instance>,string,shared_ptr<Instance>), ...>.
pub struct ScriptEventDesc {
    pub name: String,
    pub permissions: u32,
    pub attributes: u32,
    pub signature: Vec<ScriptArg>,
    pub signal: SharedPtr<Signal<ScriptContextEvent>>,
    /// Strong slot handles: rbx_core::Signal keeps only Weak refs, so the
    /// descriptor retains each connected slot (the stored connection in the
    /// original plays this role, IDA 0x2be420/0x2be42a).
    pub live_slots: Mutex<Vec<SharedPtr<dyn Fn(ScriptContextEvent) + Send + Sync>>>,
}

/// was: RBX::WaitingScriptsJob.
pub struct WaitingScriptsJob {
    pub name: &'static str,
    pub recurring: bool,
    pub data_model: Option<SharedPtr<DataModelMarker>>,
    pub period_secs: f64,
    pub context: Option<Weak<ScriptContextState>>,
}

/// was: RBX::GcJob.
pub struct GcJob {
    pub name: &'static str,
    pub recurring: bool,
    pub data_model: Option<SharedPtr<DataModelMarker>>,
    pub period_secs: f64,
    pub context: Option<Weak<ScriptContextState>>,
    /// was: +488 — 30.0 / (fps + 1.0) (IDA 0x2c4624..0x2c4642).
    pub frame_divisor: f32,
}

// 0x2bc2a8 — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviN5boost10shared_ptrINS_8InstanceEEESsELi3EEC2EMS2_FviS6_SsEPKcSC_SC_SC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(int,boost::shared_ptr<RBX::Instance>,std::string),3>::BoundFuncDesc(void (RBX::ScriptContext::*)(int,boost::shared_ptr<RBX::Instance>,std::string),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(int,boost::shared_ptr<RBX::Instance>,std::string),3>::BoundFuncDesc(void (RBX::ScriptContext::*)(int,boost::shared_ptr<RBX::Instance>,std::string),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviN5boost10shared_ptrINS_8InstanceEEESsELi3EEC2EMS2_FviS6_SsEPKcSC_SC_SC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// IDA 0x2bc2a8: base Described + FunctionDescriptor init (0x2bc2e0/0x2bc300);
// member-fn pair stored at +10 (0x2bc31a); three scoped default-arg slots
// zeroed (0x2bc320..0x2bc344); declareSignature (0x2bc3a2). MODEL: vtable
// installs and the describedClassDescriptor statics are unmodeled.
pub fn stub_0x2bc2a8(
    name: String,
    method: ScriptMethod3,
    arg_names: (String, String, String),
    permissions: u32,
    attributes: u32,
) -> BoundFuncDesc3 {
    let mut desc = BoundFuncDesc3 {
        name,
        permissions,
        attributes,
        method,
        signature: Vec::new(),
    };
    stub_0x2bc4c4(&mut desc, arg_names);
    desc
}

// 0x2bc4c4 — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviN5boost10shared_ptrINS_8InstanceEEESsELi3EE16declareSignatureEPKcNS0_7VariantESA_SB_SA_SB_
// RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(int,boost::shared_ptr<RBX::Instance>,std::string),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(int,boost::shared_ptr<RBX::Instance>,std::string),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviN5boost10shared_ptrINS_8InstanceEEESsELi3EE16declareSignatureEPKcNS0_7VariantESA_SB_SA_SB_")]
// IDA 0x2bc4c4: return type void (0x2bc4da); addArgument(int) (0x2bc4f2),
// addArgument(shared_ptr<Instance>) (0x2bc50a), addArgument(string)
// (0x2bc528). MODEL: Name::declare interning is unmodeled; names kept raw.
pub fn stub_0x2bc4c4(desc: &mut BoundFuncDesc3, arg_names: (String, String, String)) {
    desc.signature.clear();
    desc.signature.push(ScriptArg {
        name: arg_names.0,
        kind: ScriptArgKind::Int,
    });
    desc.signature.push(ScriptArg {
        name: arg_names.1,
        kind: ScriptArgKind::Instance,
    });
    desc.signature.push(ScriptArg {
        name: arg_names.2,
        kind: ScriptArgKind::Str,
    });
}

// 0x2bc530 — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviN5boost10shared_ptrINS_8InstanceEEESsELi3EED0Ev
// RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(int,boost::shared_ptr<RBX::Instance>,std::string),3>::~BoundFuncDesc()
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(int,boost::shared_ptr<RBX::Instance>,std::string),3>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviN5boost10shared_ptrINS_8InstanceEEESsELi3EED0Ev")]
// IDA 0x2bc530 (D0): scoped member dtors (0x2bc564..0x2bc5a8), base signature
// list clear (0x2bc5c6), operator delete (0x2bc5cc). MODEL: consuming Box
// drops members and frees — same observable.
pub fn stub_0x2bc530(_obj: Box<BoundFuncDesc3>) {}

// 0x2bc658 — __ZNK3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviN5boost10shared_ptrINS_8InstanceEEESsELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(int,boost::shared_ptr<RBX::Instance>,std::string),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(int,boost::shared_ptr<RBX::Instance>,std::string),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviN5boost10shared_ptrINS_8InstanceEEESsELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// IDA 0x2bc658: target = obj ? obj-36 : 0 (0x2bc6a8..0x2bc6aa); method pair
// (0x2bc6b4/0x2bc6bc); getArg<int,1> (0x2bc6c8), getArg<Instance,2>
// (0x2bc6d4), getArg<string,3> (0x2bc6e4); Call3Helper::call (0x2bc6fe).
// MODEL: a null target would crash on the member call in the original, so it
// is skipped; mistyped args default (the original raises a Lua arg error).
pub fn stub_0x2bc658(
    desc: &BoundFuncDesc3,
    target: Option<&mut ScriptContextState>,
    args: &[ScriptVariant],
) {
    let a0 = args.first().and_then(ScriptVariant::as_int).unwrap_or(0);
    let a1 = args.get(1).and_then(|v| v.as_instance());
    let a2 = args
        .get(2)
        .and_then(ScriptVariant::as_str)
        .unwrap_or("")
        .to_string();
    if let Some(obj) = target {
        stub_0x2bc7f0(obj, desc.method, a0, a1, a2);
    }
}

// 0x2bc7f0 — __ZN3RBX10Reflection11Call3HelperINS_13ScriptContextEMS2_FviN5boost10shared_ptrINS_8InstanceEEESsEiS6_SsvE4callEPS2_S8_RNS0_7VariantERKiRKS6_RKSs
// RBX::Reflection::Call3Helper<RBX::ScriptContext,void (RBX::ScriptContext::*)(int,boost::shared_ptr<RBX::Instance>,std::string),int,boost::shared_ptr<RBX::Instance>,std::string,void>::call(RBX::ScriptContext*,void (RBX::ScriptContext::*)(int,boost::shared_ptr<RBX::Instance>,std::string),RBX::Reflection::Variant &,int const&,boost::shared_ptr<RBX::Instance> const&,std::string const&)
#[doc(alias = "RBX::Reflection::Call3Helper<RBX::ScriptContext,void (RBX::ScriptContext::*)(int,boost::shared_ptr<RBX::Instance>,std::string),int,boost::shared_ptr<RBX::Instance>,std::string,void>::call(RBX::ScriptContext*,void (RBX::ScriptContext::*)(int,boost::shared_ptr<RBX::Instance>,std::string),RBX::Reflection::Variant &,int const&,boost::shared_ptr<RBX::Instance> const&,std::string const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call3HelperINS_13ScriptContextEMS2_FviN5boost10shared_ptrINS_8InstanceEEESsEiS6_SsvE4callEPS2_S8_RNS0_7VariantERKiRKS6_RKSs")]
// IDA 0x2bc7f0: virtual-adjust obj+(bit>>1), odd bit resolves the vtbl entry
// (0x2bc84e..0x2bc854); shared arg addref (0x2bc872), string copy (0x2bc884);
// invoke (0x2bc896); temp string/shared released (0x2bc8b0..0x2bc8fa).
// MODEL: method is the already-resolved entry; Clone calls are the
// addref/copy and the temps drop after invoke — same order.
pub fn stub_0x2bc7f0(
    obj: &mut ScriptContextState,
    method: ScriptMethod3,
    a0: i32,
    a1: Option<SharedPtr<ScriptInstance>>,
    a2: String,
) {
    let b1 = a1.clone();
    let b2 = a2.clone();
    method(obj, a0, b1, b2);
}

// 0x2bceb4 — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EEC2EMS2_FvSsS6_EPKcSC_SC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(std::string,boost::shared_ptr<RBX::Instance>),2>::BoundFuncDesc(void (RBX::ScriptContext::*)(std::string,boost::shared_ptr<RBX::Instance>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(std::string,boost::shared_ptr<RBX::Instance>),2>::BoundFuncDesc(void (RBX::ScriptContext::*)(std::string,boost::shared_ptr<RBX::Instance>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EEC2EMS2_FvSsS6_EPKcSC_SC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// IDA 0x2bceb4: same shape as 0x2bc2a8 with two args — base init
// (0x2bceec/0x2bcf0c), member-fn pair at +10 (0x2bcf26), two scoped slots
// zeroed (0x2bcf2e..0x2bcf4c), declareSignature (0x2bcf94).
pub fn stub_0x2bceb4(
    name: String,
    method: ScriptMethod2,
    arg_names: (String, String),
    permissions: u32,
    attributes: u32,
) -> BoundFuncDesc2 {
    let mut desc = BoundFuncDesc2 {
        name,
        permissions,
        attributes,
        method,
        signature: Vec::new(),
    };
    stub_0x2bd080(&mut desc, arg_names);
    desc
}

// 0x2bd080 — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EE16declareSignatureEPKcNS0_7VariantESA_SB_
// RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(std::string,boost::shared_ptr<RBX::Instance>),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(std::string,boost::shared_ptr<RBX::Instance>),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EE16declareSignatureEPKcNS0_7VariantESA_SB_")]
// IDA 0x2bd080: return type void (0x2bd092); addArgument(string) (0x2bd0aa),
// addArgument(shared_ptr<Instance>) (0x2bd0c8).
pub fn stub_0x2bd080(desc: &mut BoundFuncDesc2, arg_names: (String, String)) {
    desc.signature.clear();
    desc.signature.push(ScriptArg {
        name: arg_names.0,
        kind: ScriptArgKind::Str,
    });
    desc.signature.push(ScriptArg {
        name: arg_names.1,
        kind: ScriptArgKind::Instance,
    });
}

// 0x2bd0cc — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EED0Ev
// RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(std::string,boost::shared_ptr<RBX::Instance>),2>::~BoundFuncDesc()
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(std::string,boost::shared_ptr<RBX::Instance>),2>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EED0Ev")]
// IDA 0x2bd0cc (D0): scoped member dtors (0x2bd130/0x2bd13a), base list
// clear (0x2bd156), operator delete (0x2bd15c). MODEL: consuming Box, same
// observable as 0x2bc530.
pub fn stub_0x2bd0cc(_obj: Box<BoundFuncDesc2>) {}

// 0x2bd1e4 — __ZNK3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(std::string,boost::shared_ptr<RBX::Instance>),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(std::string,boost::shared_ptr<RBX::Instance>),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// IDA 0x2bd1e4: target = obj ? obj-36 : 0 (0x2bd234..0x2bd236); method pair
// (0x2bd240/0x2bd246); getArg<string,1> (0x2bd252),
// getArg<Instance,2> (0x2bd262); Call2Helper::call (0x2bd278); arg temps
// released (0x2bd27e..0x2bd2dc). MODEL: same null-target/mistype policy as
// 0x2bc658.
pub fn stub_0x2bd1e4(
    desc: &BoundFuncDesc2,
    target: Option<&mut ScriptContextState>,
    args: &[ScriptVariant],
) {
    let a0 = args
        .first()
        .and_then(ScriptVariant::as_str)
        .unwrap_or("")
        .to_string();
    let a1 = args.get(1).and_then(|v| v.as_instance());
    if let Some(obj) = target {
        stub_0x2bd368(obj, desc.method, a0, a1);
    }
}

// 0x2bd368 — __ZN3RBX10Reflection11Call2HelperINS_13ScriptContextEMS2_FvSsN5boost10shared_ptrINS_8InstanceEEEESsS6_vE4callEPS2_S8_RNS0_7VariantERKSsRKS6_
// RBX::Reflection::Call2Helper<RBX::ScriptContext,void (RBX::ScriptContext::*)(std::string,boost::shared_ptr<RBX::Instance>),std::string,boost::shared_ptr<RBX::Instance>,void>::call(RBX::ScriptContext*,void (RBX::ScriptContext::*)(std::string,boost::shared_ptr<RBX::Instance>),RBX::Reflection::Variant &,std::string const&,boost::shared_ptr<RBX::Instance> const&)
// type: int __fastcall(int, int, int, int, std::string *, int)
#[doc(alias = "RBX::Reflection::Call2Helper<RBX::ScriptContext,void (RBX::ScriptContext::*)(std::string,boost::shared_ptr<RBX::Instance>),std::string,boost::shared_ptr<RBX::Instance>,void>::call(RBX::ScriptContext*,void (RBX::ScriptContext::*)(std::string,boost::shared_ptr<RBX::Instance>),RBX::Reflection::Variant &,std::string const&,boost::shared_ptr<RBX::Instance> const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call2HelperINS_13ScriptContextEMS2_FvSsN5boost10shared_ptrINS_8InstanceEEEESsS6_vE4callEPS2_S8_RNS0_7VariantERKSsRKS6_")]
// IDA 0x2bd368: virtual-adjust obj+(bit>>1), odd bit resolves the vtbl entry
// (0x2bd3b8..0x2bd3c8); string copy (0x2bd3d8), shared arg addref
// (0x2bd3e2..0x2bd3f6); invoke (0x2bd402); temps released (0x2bd406..0x2bd464).
// MODEL: same policy as 0x2bc7f0, copies in original order.
pub fn stub_0x2bd368(
    obj: &mut ScriptContextState,
    method: ScriptMethod2,
    a0: String,
    a1: Option<SharedPtr<ScriptInstance>>,
) {
    let b0 = a0.clone();
    let b1 = a1.clone();
    method(obj, b0, b1);
}

// 0x2be070 — __ZN3RBX10Reflection9EventDescINS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_SE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// RBX::Reflection::EventDesc<RBX::ScriptContext,void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)> RBX::ScriptContext::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)> RBX::ScriptContext::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::ScriptContext,void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)> RBX::ScriptContext::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)> RBX::ScriptContext::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_SE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// IDA 0x2be070: base Described + EventDescriptor init (0x2be0b0/0x2be0ce);
// member-signal offset stored (0x2be0f2); three signature items —
// shared_ptr<Instance> (0x2be130), string (0x2be16c), shared_ptr<Instance>
// (0x2be1a4) — list-hooked (0x2be140/0x2be178/0x2be1b0).
pub fn stub_0x2be070(
    name: String,
    arg_names: (String, String, String),
    permissions: u32,
    attributes: u32,
) -> ScriptEventDesc {
    ScriptEventDesc {
        name,
        permissions,
        attributes,
        signature: vec![
            ScriptArg {
                name: arg_names.0,
                kind: ScriptArgKind::Instance,
            },
            ScriptArg {
                name: arg_names.1,
                kind: ScriptArgKind::Str,
            },
            ScriptArg {
                name: arg_names.2,
                kind: ScriptArgKind::Instance,
            },
        ],
        signal: SharedPtr::new(Signal::new()),
        live_slots: Mutex::new(Vec::new()),
    }
}

// 0x2be2cc — __ZN3RBX10Reflection9EventDescINS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_ED0Ev
// RBX::Reflection::EventDesc<RBX::ScriptContext,void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)> RBX::ScriptContext::*>::~EventDesc()
#[doc(alias = "RBX::Reflection::EventDesc<RBX::ScriptContext,void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)> RBX::ScriptContext::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_ED0Ev")]
// IDA 0x2be2cc (D0): signature list clear (0x2be330), operator delete
// (0x2be336). MODEL: consuming Box, same observable as 0x2bc530.
pub fn stub_0x2be2cc(_obj: Box<ScriptEventDesc>) {}

// 0x2be380 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// RBX::Reflection::EventDescImpl<3,RBX::ScriptContext,void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)> RBX::ScriptContext::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::ScriptContext,void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)> RBX::ScriptContext::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi3ENS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE")]
// IDA 0x2be380: bind GenericSlotWrapper::execute3(wrapper,_1,_2,_3) (0x2be3f8)
// into a boost::function (0x2be404); signal::connect (0x2be420), or a null
// connection when the wrapper is null (0x2be42a); bind/function temps
// released (0x2be432..0x2be44c). MODEL: the bound slot is a closure over the
// wrapper; the descriptor retains it (see live_slots).
pub fn stub_0x2be380(desc: &ScriptEventDesc, slot: Option<SharedPtr<GenericSlot3>>) -> bool {
    match slot {
        None => false,
        Some(w) => {
            let cb = SharedPtr::new(move |args: ScriptContextEvent| {
                generic_slot_execute3(&w, args);
            });
            // Concrete closure type into connect (F: Sized); the erased
            // Arc<dyn Fn> is what the descriptor retains.
            desc.signal.connect(SharedPtr::clone(&cb));
            desc.live_slots.lock().push(cb);
            true
        }
    }
}

// 0x2be4d4 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
// RBX::Reflection::EventDescImpl<3,RBX::ScriptContext,void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)> RBX::ScriptContext::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::ScriptContext,void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)> RBX::ScriptContext::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi3ENS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE")]
// IDA 0x2be4d4: ReleaseAssert(args.size()==3) (Event.h:380,
// 0x2be510..0x2be580); any_cast Instance/68-byte slot 0 (0x2be5a6), string
// slot 1 (0x2be5c8), Instance slot 2 (0x2be5e2); signal_with_args<3> invoke
// (0x2be614); arg temps released (0x2be61a..0x2be686). MODEL: mistyped slots
// coerce to null/empty (the original throws bad_any_cast into Lua).
pub fn stub_0x2be4d4(desc: &ScriptEventDesc, args: &[ScriptVariant]) {
    assert!(args.len() == 3, "args.size() == 3");
    let event = ScriptContextEvent(
        args[0].as_instance(),
        args[1].as_str().unwrap_or("").to_string(),
        args[2].as_instance(),
    );
    desc.signal.fire(event);
}

// 0x2be728 — __ZNK3RBX10Reflection13EventDescBaseINS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
// RBX::Reflection::EventDescBase<RBX::ScriptContext,void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)> RBX::ScriptContext::*>::disconnectAll(RBX::Reflection::EventSource *)const
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::ScriptContext,void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)> RBX::ScriptContext::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE")]
// IDA 0x2be728: member-signal at *(a1+40)+(a2 ? a2-36 : 0);
// rbx::signal::disconnectAll on it. MODEL: plus dropping the retained slots.
pub fn stub_0x2be728(desc: &ScriptEventDesc) {
    desc.signal.disconnect_all();
    desc.live_slots.lock().clear();
}

// 0x2c190c — __ZN5boost13intrusive_ptrIN3RBX3Lua6detail13LiveThreadRefEEaSERKS5_
// boost::intrusive_ptr<RBX::Lua::detail::LiveThreadRef>::operator=(boost::intrusive_ptr<RBX::Lua::detail::LiveThreadRef> const&)
#[doc(alias = "boost::intrusive_ptr<RBX::Lua::detail::LiveThreadRef>::operator=(boost::intrusive_ptr<RBX::Lua::detail::LiveThreadRef> const&)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3RBX3Lua6detail13LiveThreadRefEEaSERKS5_")]
// IDA 0x2c190c: addref the new pointer first (OSAtomicAdd32(1), 0x2c191a),
// store it (0x2c1920), then release the old one (0x2c1926). MODEL:
// intrusive_ptr<T> is Option<SharedPtr<T>>; clone-then-assign bumps the new
// owner before dropping the old — same order, self-assign safe.
pub fn stub_0x2c190c(
    dst: &mut Option<SharedPtr<LiveThreadRefDetail>>,
    src: &Option<SharedPtr<LiveThreadRefDetail>>,
) {
    *dst = src.clone();
}

// 0x2c1930 — __ZN5boost21intrusive_ptr_releaseIN3RBX3Lua6detail13LiveThreadRefEiLi0EEEvPKN3rbx26quick_intrusive_ptr_targetIT_T0_XT1_EEE
// void boost::intrusive_ptr_release<RBX::Lua::detail::LiveThreadRef,int,0>(rbx::quick_intrusive_ptr_target<RBX::Lua::detail::LiveThreadRef,int,0> const*)
#[doc(alias = "void boost::intrusive_ptr_release<RBX::Lua::detail::LiveThreadRef,int,0>(rbx::quick_intrusive_ptr_target<RBX::Lua::detail::LiveThreadRef,int,0> const*)")]
#[doc(alias = "__ZN5boost21intrusive_ptr_releaseIN3RBX3Lua6detail13LiveThreadRefEiLi0EEEvPKN3rbx26quick_intrusive_ptr_targetIT_T0_XT1_EEE")]
// IDA 0x2c1930: ReleaseAssert(p->refs > 0) (intrusive_ptr_target.h:163,
// 0x2c196c..0x2c19e2); OSAtomicAdd32(-1) (0x2c19e4); zero count runs
// ~LiveThreadRef + operator delete (0x2c19f4/0x2c19fa). MODEL: releasing one
// SharedPtr owner; the last drop runs the same destroy+free, and the
// refs>0 assert is structural (a live owner always has count >= 1).
pub fn stub_0x2c1930(_owner: Option<SharedPtr<LiveThreadRefDetail>>) {}

// 0x2c1af8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_12LuaStatsItemEPNS_13ScriptContextEEEN5boost10shared_ptrIT_EET0_
// boost::shared_ptr<RBX::LuaStatsItem> RBX::Creatable<RBX::Instance>::create<RBX::LuaStatsItem,RBX::ScriptContext *>(RBX::ScriptContext *)
#[doc(alias = "boost::shared_ptr<RBX::LuaStatsItem> RBX::Creatable<RBX::Instance>::create<RBX::LuaStatsItem,RBX::ScriptContext *>(RBX::ScriptContext *)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_12LuaStatsItemEPNS_13ScriptContextEEEN5boost10shared_ptrIT_EET0_")]
// IDA 0x2c1af8: operator new(0x80) (0x2c1b2e), LuaStatsItem::LuaStatsItem(ctx)
// (0x2c1b54), adopt into a shared_ptr with Creatable<Instance>::Deleter
// (0x2c1b62, IDA 0x2c2270). MODEL: Box->Arc adoption; the deleter tag and
// the unmodeled stat fields are dropped.
pub fn stub_0x2c1af8(ctx: SharedPtr<ScriptContextState>) -> SharedPtr<LuaStatsItem> {
    SharedPtr::new(LuaStatsItem { context: ctx })
}

// 0x2c2270 — __ZN5boost10shared_ptrIN3RBX12LuaStatsItemEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// boost::shared_ptr<RBX::LuaStatsItem>::shared_ptr<RBX::LuaStatsItem,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "boost::shared_ptr<RBX::LuaStatsItem>::shared_ptr<RBX::LuaStatsItem,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX12LuaStatsItemEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// IDA 0x2c2270: px = p (0x2c2290); shared_count(p, deleter) (0x2c2298); if
// (p) _internal_accept_owner (0x2c22d6, IDA 0x2c2338). MODEL: adopt plus the
// weak-owner install below.
pub fn stub_0x2c2270(
    item: LuaStatsItem,
    slot: &mut Option<Weak<LuaStatsItem>>,
) -> SharedPtr<LuaStatsItem> {
    let shared = SharedPtr::new(item);
    stub_0x2c2338(slot, &shared);
    shared
}

// 0x2c2338 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12LuaStatsItemES6_EEvPKNS_10shared_ptrIT_EEPT0_
// void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LuaStatsItem,RBX::LuaStatsItem>(boost::shared_ptr<RBX::LuaStatsItem> const*,RBX::LuaStatsItem *)const
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LuaStatsItem,RBX::LuaStatsItem>(boost::shared_ptr<RBX::LuaStatsItem> const*,RBX::LuaStatsItem *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12LuaStatsItemES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// IDA 0x2c2338: if the weak slot has no shared owners (0x2c2360), install
// this shared owner into it (0x2c23b0..0x2c23ba); the old control block is
// released (0x2c23c8). MODEL: Weak::strong_count()==0 installs the
// downgrade; a live slot is left alone.
pub fn stub_0x2c2338(slot: &mut Option<Weak<LuaStatsItem>>, owner: &SharedPtr<LuaStatsItem>) {
    let live = slot.as_ref().map(Weak::strong_count).unwrap_or(0);
    if live == 0 {
        *slot = Some(Arc::downgrade(owner));
    }
}

// 0x2c3fb0 — __ZN3RBX17WaitingScriptsJobC2EN5boost10shared_ptrINS_13ScriptContextEEE
// RBX::WaitingScriptsJob::WaitingScriptsJob(boost::shared_ptr<RBX::ScriptContext>)
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, RBX::TaskScheduler::Job *, int, int, int, int)
#[doc(alias = "RBX::WaitingScriptsJob::WaitingScriptsJob(boost::shared_ptr<RBX::ScriptContext>)")]
#[doc(alias = "__ZN3RBX17WaitingScriptsJobC2EN5boost10shared_ptrINS_13ScriptContextEEE")]
// IDA 0x2c3fb0: DataModel via shared_from_dynamic_cast on the context owner
// (0x2c3fde); DataModelJob base ("LuaResumeWaitingScripts", recurring=1,
// period = LuaSettings(+124)/60.0) (0x2c4032..0x2c405e); weak ScriptContext
// at +480 (0x2c409a). MODEL: the cast chain is unmodeled (None); the period
// takes the settings rate as a parameter.
pub fn stub_0x2c3fb0(
    ctx: &SharedPtr<ScriptContextState>,
    resumes_per_minute: f32,
) -> WaitingScriptsJob {
    WaitingScriptsJob {
        name: "LuaResumeWaitingScripts",
        recurring: true,
        data_model: None,
        period_secs: (resumes_per_minute / 60.0) as f64,
        context: Some(Arc::downgrade(ctx)),
    }
}

// 0x2c44c0 — __ZN5boost10shared_ptrIN3RBX13ScriptContextEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// boost::shared_ptr<RBX::ScriptContext>::shared_ptr<RBX::ScriptContext>(boost::weak_ptr<RBX::ScriptContext> const&,boost::detail::sp_nothrow_tag)
#[doc(alias = "boost::shared_ptr<RBX::ScriptContext>::shared_ptr<RBX::ScriptContext>(boost::weak_ptr<RBX::ScriptContext> const&,boost::detail::sp_nothrow_tag)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX13ScriptContextEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")]
// IDA 0x2c44c0 (nothrow): start null (0x2c44ce); under the pool spinlock,
// bump only when use_count > 0 and alias the pointer (0x2c44fe..0x2c4524),
// else stay null (0x2c452a..0x2c4530) — never throws on expiry. MODEL:
// Weak::upgrade is exactly this: Some when live, None when expired/empty.
pub fn stub_0x2c44c0(
    weak: &Option<Weak<ScriptContextState>>,
) -> Option<SharedPtr<ScriptContextState>> {
    weak.as_ref().and_then(Weak::upgrade)
}

// 0x2c453c — __ZN3RBX5GcJobC2EN5boost10shared_ptrINS_13ScriptContextEEE
// RBX::GcJob::GcJob(boost::shared_ptr<RBX::ScriptContext>)
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, RBX::TaskScheduler::Job *, int, int, int, int)
#[doc(alias = "RBX::GcJob::GcJob(boost::shared_ptr<RBX::ScriptContext>)")]
#[doc(alias = "__ZN3RBX5GcJobC2EN5boost10shared_ptrINS_13ScriptContextEEE")]
// IDA 0x2c453c: same DataModelJob shape as 0x2c3fb0 with name "LuaGc" and a
// fixed 0.003s period (double words 0xBC6A7EFA/1063818100 = 0x3F689374…,
// 0x2c45dc); weak ScriptContext at +480 (0x2c4618); +488 holds
// 30.0/(fps+1.0) with fps from the DataModel (0x2c4624..0x2c4642).
pub fn stub_0x2c453c(ctx: &SharedPtr<ScriptContextState>, data_model_fps: i32) -> GcJob {
    GcJob {
        name: "LuaGc",
        recurring: true,
        data_model: None,
        period_secs: 0.003,
        context: Some(Arc::downgrade(ctx)),
        frame_divisor: 30.0 / (data_model_fps as f32 + 1.0),
    }
}

// 0x2c59b0 — __ZN3RBX3Lua6BridgeIN5boost13intrusive_ptrINS0_13WeakThreadRef4NodeEEELb1EE11on_tostringERKS6_P9lua_State
// RBX::Lua::Bridge<boost::intrusive_ptr<RBX::Lua::WeakThreadRef::Node>,true>::on_tostring(boost::intrusive_ptr<RBX::Lua::WeakThreadRef::Node> const&,lua_State *)
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Lua::Bridge<boost::intrusive_ptr<RBX::Lua::WeakThreadRef::Node>,true>::on_tostring(boost::intrusive_ptr<RBX::Lua::WeakThreadRef::Node> const&,lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost13intrusive_ptrINS0_13WeakThreadRef4NodeEEELb1EE11on_tostringERKS6_P9lua_State")]
// IDA 0x2c59b0: lua_pushlstring(L, "WeakThreadRef", 13) (0x2c59c4);
// return 1 (0x2c59ca).
pub fn stub_0x2c59b0(_node: &WeakThreadRefNode, l: &mut LuaPushStack) -> i32 {
    debug_assert_eq!(b"WeakThreadRef".len(), 13);
    l.push_lstring(b"WeakThreadRef");
    1
}

// 0x2c59cc — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFNS3_IKNS_10Reflection5TupleEEES8_EEEEELb1EE11on_tostringERKSB_P9lua_State
// RBX::Lua::Bridge<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,true>::on_tostring(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>> const&,lua_State *)
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Lua::Bridge<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,true>::on_tostring(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>> const&,lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFNS3_IKNS_10Reflection5TupleEEES8_EEEEELb1EE11on_tostringERKSB_P9lua_State")]
// IDA 0x2c59cc: lua_pushlstring(L, "GenericFunction", 15) (0x2c59e0);
// return 1 (0x2c59e6).
pub fn stub_0x2c59cc(_func: &GenericFunction, l: &mut LuaPushStack) -> i32 {
    debug_assert_eq!(b"GenericFunction".len(), 15);
    l.push_lstring(b"GenericFunction");
    1
}

// 0x2c59e8 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFvNS3_IKNS_10Reflection5TupleEEENS4_IFvPNS0_12IAsyncResultEEEEEEEEELb1EE11on_tostringERKSF_P9lua_State
// RBX::Lua::Bridge<boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>,true>::on_tostring(boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>> const&,lua_State *)
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Lua::Bridge<boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>,true>::on_tostring(boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>> const&,lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFvNS3_IKNS_10Reflection5TupleEEENS4_IFvPNS0_12IAsyncResultEEEEEEEEELb1EE11on_tostringERKSF_P9lua_State")]
// IDA 0x2c59e8: lua_pushlstring(L, "GenericAsyncFunction", 20) (0x2c59fc);
// return 1 (0x2c5a02).
pub fn stub_0x2c59e8(_func: &GenericAsyncFunction, l: &mut LuaPushStack) -> i32 {
    debug_assert_eq!(b"GenericAsyncFunction".len(), 20);
    l.push_lstring(b"GenericAsyncFunction");
    1
}

// 0x2c5ec8 — __ZN5boost13intrusive_ptrIN3RBX3Lua13WeakThreadRefEEaSERKS4_
// boost::intrusive_ptr<RBX::Lua::WeakThreadRef>::operator=(boost::intrusive_ptr<RBX::Lua::WeakThreadRef> const&)
#[doc(alias = "boost::intrusive_ptr<RBX::Lua::WeakThreadRef>::operator=(boost::intrusive_ptr<RBX::Lua::WeakThreadRef> const&)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3RBX3Lua13WeakThreadRefEEaSERKS4_")]
// IDA 0x2c5ec8: same addref-store-release shape as 0x2c190c, except the
// counter lives at ptr+4 (0x2c5ed2..0x2c5ee4) — a layout detail of
// quick_intrusive_ptr_target. MODEL: same Option<SharedPtr> assignment.
pub fn stub_0x2c5ec8(
    dst: &mut Option<SharedPtr<WeakThreadRefNode>>,
    src: &Option<SharedPtr<WeakThreadRefNode>>,
) {
    *dst = src.clone();
}
