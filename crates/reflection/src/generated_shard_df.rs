// Auto-generated shard DF — next 100 RBX::Reflection stubs — EA-sorted ascending 0x3f2860..0x43b058 (remaining 254) — starts 0x3f2860
// Source: ida/export.json filtered demangled contains RBX::Reflection (16171 total, 15817->15917 covered, 254 remaining)
// Format: // 0xADDR — mangled + #[doc(alias = "RBX::...")] + todo!("0xADDR") using rbx_core::SharedPtr (was boost::shared_ptr)
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(clippy::all)]
use rbx_core::SharedPtr;
use crate::descriptor::{GenericSlotWrapper, InstanceHandle, Variant};
use crate::enum_desc::EnumDesc;
use rbx_core::signal::Signal;

/// Signature argument kinds of `EventDesc<CollectionService, void(SharedPtr<Instance>)>`
/// (IDA 0x3f4622 `Type::getSingleton<SharedPtr<Instance>>`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectionEventArg {
    Instance,
}

/// `RBX::Reflection::EventDesc<CollectionService, void(SharedPtr<Instance>), ...>`
/// (IDA 0x3f4578): base `EventDescriptor` init, member-signal pointer stored at +40
/// (`v36[10] = a2`), one-item signature list appended.
#[derive(Debug, Clone)]
pub struct CollectionServiceEventDesc {
    pub name: String,
    pub category: String,
    pub member: usize,
    pub signature: Vec<(String, CollectionEventArg)>,
    pub permissions: u32,
    pub attributes: u32,
}

/// Connected slot: the original signal owns its slots until `disconnectAll`; the strong
/// refs live in `holders` because `Signal::connect` keeps only weak refs.
type InstanceSlot = SharedPtr<dyn Fn(SharedPtr<InstanceHandle>) + Send + Sync>;

/// `RBX::Reflection::EventSource` for the single-instance CollectionService signal:
/// owns the connected slots. Backed by `rbx_core::signal::Signal` (IDA 0x3f4850/0x3f49fa).
#[derive(Default)]
pub struct InstanceEventSource {
    signal: Signal<SharedPtr<InstanceHandle>>,
    holders: parking_lot::Mutex<Vec<(SharedPtr<GenericSlotWrapper>, InstanceSlot)>>,
}

impl InstanceEventSource {
    pub fn connect_slot(&self, wrapper: SharedPtr<GenericSlotWrapper>) {
        let w = SharedPtr::clone(&wrapper);
        let slot = std::sync::Arc::new(move |instance: SharedPtr<InstanceHandle>| {
            // IDA 0x3f4828 `bind(execute1<SharedPtr<Instance>>, wrapper, _1)`: pack the
            // single instance into a 1-Variant vector and dispatch the stored callable
            // (cf. execute2 at 0x4a40c8).
            (w.invoke)(&[Variant::Instance(SharedPtr::clone(&instance))]);
        });
        self.signal.connect(SharedPtr::clone(&slot));
        let slot: InstanceSlot = slot;
        self.holders.lock().push((wrapper, slot));
    }

    pub fn fire(&self, instance: &SharedPtr<InstanceHandle>) {
        self.signal.fire(SharedPtr::clone(instance));
    }

    pub fn disconnect_all(&self) {
        self.holders.lock().clear();
        self.signal.disconnect_all();
    }
}

/// `boost::_bi::bind_t<mf1<GenericSlotWrapper, const SharedPtr<Instance>&>>`
/// (IDA 0x3f4828): stores the bound wrapper; the member function is fixed
/// (`execute1`), so the triple folds into the target.
#[derive(Clone)]
pub struct BoundInstanceSlot {
    pub target: SharedPtr<GenericSlotWrapper>,
}

impl BoundInstanceSlot {
    pub fn call(&self, instance: &SharedPtr<InstanceHandle>) {
        // IDA mf1 operator(): pack the instance and dispatch (cf. 0x4a40c8).
        (self.target.invoke)(&[Variant::Instance(SharedPtr::clone(instance))]);
    }
}

/// `boost::function1<void, SharedPtr<Instance>>` holding one bound slot
/// (IDA 0x3f4834).
#[derive(Default, Clone)]
pub struct InstanceSlotFunction {
    bound: Option<BoundInstanceSlot>,
}

impl InstanceSlotFunction {
    pub fn is_empty(&self) -> bool {
        self.bound.is_none()
    }

    pub fn invoke(&self, instance: &SharedPtr<InstanceHandle>) {
        // Calling an empty `boost::function` throws `bad_function_call`; panic mirrors it.
        self.bound.as_ref().expect("bad_function_call").call(instance);
    }
}

/// `RBX::Reflection::BoundFuncDesc<CollectionService,
/// SharedPtr<vector<SharedPtr<Instance>> const>(string), 1>` (IDA 0x3f4a78): base
/// `FunctionDescriptor` init, member-function pair stored at +40, return-type plus
/// one `string` argument declared via `declareSignature` (IDA 0x3f4b2a-0x3f4b3a).
#[derive(Debug, Clone, Default)]
pub struct CollectionTagsFuncDesc {
    pub name: String,
    pub category: String,
    pub member: u64,
    pub return_type: &'static str,
    pub args: Vec<(String, &'static str)>,
    pub permissions: u32,
    pub attributes: u32,
}

/// One `RBX::TaskScheduler::Job` row as read by `appendJobInfo` (IDA 0x42fb68):
/// name string at +112, duty-cycle/step/error averages, and the state word at +136
/// (`v37 = *(job + 34)`; active iff `== 3`, 0x42fde4-0x42fe06).
#[derive(Debug, Clone, Default)]
pub struct JobSample {
    pub name: String,
    pub average_duty_cycle: f64,
    pub average_steps_per_second: f64,
    pub average_step_time: f64,
    pub average_error: f64,
    pub state: i32,
    /// Owning-DataModel tag (`*(job + 23)`); the append only runs when it matches
    /// the model's tag (`v14 == a1 + 184`, 0x42fbfc).
    pub owner_tag: u32,
}

/// Duty-cycle stats block filled by `WindowAverageDutyCycle::getStats(job[99])`
/// (IDA 0x4300d0) and pushed as seven doubles by `appendJobExtendedStats`.
#[derive(Debug, Clone, Default)]
pub struct JobDutyStats {
    pub name: String,
    pub values: [f64; 7],
    /// Owning-DataModel tag, same guard as `appendJobInfo` (IDA 0x4300b2).
    pub owner_tag: u32,
}


// 0x3f2860 — __ZN3RBX10Reflection9EventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::ClickDetector,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ClickDetector::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_ED1Ev")]
// was: RBX::Reflection::EventDesc<RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ClickDetector::*>::~EventDesc()
pub fn stub_3f2860() {
    // IDA 0x3f2860: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3f2884 — __ZN3RBX10Reflection9EventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::ClickDetector,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ClickDetector::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_ED0Ev")]
// was: RBX::Reflection::EventDesc<RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ClickDetector::*>::~EventDesc()
pub fn stub_3f2884() {
    // IDA 0x3f2884: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3f351c — __ZN3RBX10Reflection13BoundFuncDescINS_17CollectionServiceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEESsELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::CollectionService,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(std::string),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_17CollectionServiceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEESsELi1EED1Ev")]
// was: RBX::Reflection::BoundFuncDesc<RBX::CollectionService,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(std::string),1>::~BoundFuncDesc()
pub fn stub_3f351c() {
    // IDA 0x3f351c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3f355c — __ZN3RBX10Reflection9EventDescINS_17CollectionServiceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::CollectionService,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::CollectionService::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_17CollectionServiceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED1Ev")]
// was: RBX::Reflection::EventDesc<RBX::CollectionService,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::CollectionService::*>::~EventDesc()
pub fn stub_3f355c() {
    // IDA 0x3f355c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3f4578 — __ZN3RBX10Reflection9EventDescINS_17CollectionServiceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::CollectionService,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::CollectionService::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::CollectionService::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_17CollectionServiceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: RBX::Reflection::EventDesc<RBX::CollectionService,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::CollectionService::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::CollectionService::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_3f4578(
    member: usize,
    name: &str,
    category: &str,
    permissions: u32,
    attributes: u32,
) -> CollectionServiceEventDesc {
    // IDA 0x3f4578: classDescriptor guard-once (0x3f45b0), `EventDescriptor` base init
    // (0x3f45ce), member-signal pointer stored at +40 (`v36[10] = a2`, 0x3f45f2), vtable
    // off_1243048 (0x3f45f6), `Name::declare` (0x3f461c),
    // `Type::getSingleton<SharedPtr<Instance>>` (0x3f4622), one signature item
    // appended (0x3f4630-0x3f464a). Same shape as the Explosion twin at 0x4a38b8.
    CollectionServiceEventDesc {
        name: name.to_owned(),
        category: category.to_owned(),
        member,
        signature: vec![(category.to_owned(), CollectionEventArg::Instance)],
        permissions,
        attributes,
    }
}

// 0x3f46fc — __ZN3RBX10Reflection9EventDescINS_17CollectionServiceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::CollectionService,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::CollectionService::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_17CollectionServiceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED0Ev")]
// was: RBX::Reflection::EventDesc<RBX::CollectionService,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::CollectionService::*>::~EventDesc()
pub fn stub_3f46fc() {
    // IDA 0x3f46fc: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3f47b0 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_17CollectionServiceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::CollectionService,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::CollectionService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_17CollectionServiceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE")]
// was: RBX::Reflection::EventDescImpl<1,RBX::CollectionService,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::CollectionService::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_3f47b0(source: Option<&InstanceEventSource>, wrapper: SharedPtr<GenericSlotWrapper>) {
    // IDA 0x3f47b0: builds `bind(execute1<SharedPtr<Instance>>, wrapper, _1)` (0x3f4828),
    // wraps it in a `boost::function` (0x3f4834), then `signal::connect(member-signal
    // of source, fn)` (0x3f4850). Null source stores an empty connection (`*v44 = 0`,
    // 0x3f485a). Temp clear (0x3f4862) and shared-count releases (0x3f4868-0x3f487c) are
    // `Arc` drop glue. Same shape as the Explosion twin at 0x4a3b5c.
    if let Some(source) = source {
        source.connect_slot(wrapper);
    }
    // `function1::clear()` drops the temp; `Arc` drop glue covers it.
}

// 0x3f4904 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_17CollectionServiceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::CollectionService,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::CollectionService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_17CollectionServiceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE")]
// was: RBX::Reflection::EventDescImpl<1,RBX::CollectionService,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::CollectionService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
pub fn stub_3f4904(source: &InstanceEventSource, args: &[Variant]) {
    // IDA 0x3f4904: `ReleaseAssert(args.size() == 1)` (Event.h:320, 0x3f4940-0x3f49b4), then
    // `any_cast<SharedPtr<Instance>>(args[0])` (0x3f49d4) and
    // `signal_with_args<1>::operator()` (0x3f49fa). Same shape as the Explosion twin
    // at 0x4a3cb0.
    assert!(args.len() == 1, "args.size() == 1 include/Reflection/Event.h:320");
    let Variant::Instance(instance) = &args[0] else {
        panic!("any_cast<SharedPtr<Instance>> failed (IDA 0x3f49d4)");
    };
    source.fire(instance);
}

// 0x3f4a64 — __ZNK3RBX10Reflection13EventDescBaseINS_17CollectionServiceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::CollectionService,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::CollectionService::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_17CollectionServiceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE")]
// was: RBX::Reflection::EventDescBase<RBX::CollectionService,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::CollectionService::*>::disconnectAll(RBX::Reflection::EventSource *)const
pub fn stub_3f4a64(source: &InstanceEventSource) {
    // IDA 0x3f4a64: member-offset adjust (`a2 ? a2 - 36 : 0`, 0x3f4a68-0x3f4a6a), then
    // `signal::disconnectAll(member)`. The adjust is member-pointer mechanics; the
    // observable effect is dropping every slot. Same shape as the Explosion twin at
    // 0x4a3e20.
    source.disconnect_all();
}

// 0x3f4a78 — __ZN3RBX10Reflection13BoundFuncDescINS_17CollectionServiceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEESsELi1EEC2EMS2_FSB_SsEPKcSH_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::CollectionService,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(std::string),1>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::CollectionService::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_17CollectionServiceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEESsELi1EEC2EMS2_FSB_SsEPKcSH_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: RBX::Reflection::BoundFuncDesc<RBX::CollectionService,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(std::string),1>::BoundFuncDesc(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::CollectionService::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_3f4a78(
    member: u64,
    name: &str,
    arg_name: &str,
    category: &str,
    permissions: u32,
    attributes: u32,
) -> CollectionTagsFuncDesc {
    // IDA 0x3f4a78: classDescriptor guard-once (0x3f4ab0), `FunctionDescriptor` base init
    // (0x3f4ad0), vtable off_12430A8 (0x3f4ae6), member-function pair stored at +40
    // (`*(_QWORD *)(v27 + 40) = v16`, 0x3f4aea), signature storage init (0x3f4afc-0x3f4b06),
    // `getSingleton<void>` + `declareSignature` (0x3f4b2a-0x3f4b3a), holder teardown
    // (0x3f4b40-0x3f4b4c).
    let mut desc = CollectionTagsFuncDesc {
        name: name.to_owned(),
        category: category.to_owned(),
        member,
        return_type: "SharedPtr<vector<SharedPtr<Instance>> const>",
        args: Vec::new(),
        permissions,
        attributes,
    };
    stub_3f4bf0(&mut desc, arg_name);
    desc
}

// 0x3f4bf0 — __ZN3RBX10Reflection13BoundFuncDescINS_17CollectionServiceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEESsELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::CollectionService,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_17CollectionServiceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEESsELi1EE16declareSignatureEPKcNS0_7VariantE")]
// was: RBX::Reflection::BoundFuncDesc<RBX::CollectionService,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)
pub fn stub_3f4bf0(desc: &mut CollectionTagsFuncDesc, arg_name: &str) {
    // IDA 0x3f4bf0: return-type singleton
    // `getSingleton<SharedPtr<vector<SharedPtr<Instance>> const>>` stored at +28
    // (0x3f4bfc-0x3f4c00), `Name::declare` (0x3f4c0a), `getSingleton<string>` (0x3f4c0c),
    // `SignatureDescriptor::addArgument` (0x3f4c1e).
    desc.return_type = "SharedPtr<vector<SharedPtr<Instance>> const>";
    desc.args.push((arg_name.to_owned(), "string"));
}

// 0x3f4c20 — __ZN3RBX10Reflection13BoundFuncDescINS_17CollectionServiceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEESsELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::CollectionService,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(std::string),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_17CollectionServiceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEESsELi1EED0Ev")]
// was: RBX::Reflection::BoundFuncDesc<RBX::CollectionService,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(std::string),1>::~BoundFuncDesc()
pub fn stub_3f4c20() {
    // IDA 0x3f4c20: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3f4cec — __ZNK3RBX10Reflection13BoundFuncDescINS_17CollectionServiceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEESsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::CollectionService,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_17CollectionServiceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEESsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// was: RBX::Reflection::BoundFuncDesc<RBX::CollectionService,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
pub fn stub_3f4cec(
    get_tags: &dyn Fn(&str) -> Vec<SharedPtr<InstanceHandle>>,
    args: &[Variant],
) -> Vec<SharedPtr<InstanceHandle>> {
    // IDA 0x3f4cec: member-offset adjust (`a2 ? a2 - 36 : 0`, 0x3f4d3c-0x3f4d3e), member
    // pointer at +40 (0x3f4d42-0x3f4d48), `ArgHelper::getArg<string, 1>` (0x3f4d58),
    // `Call1Helper::call` (0x3f4d6c), argument string teardown (0x3f4d7e-0x3f4dc4).
    assert!(args.len() == 1, "BoundFuncDesc<..., 1>: one argument");
    let Variant::Text(filter) = &args[0] else {
        panic!("getArg<string, 1> failed (IDA 0x3f4d58)");
    };
    stub_3f4e2c(get_tags, filter)
}

// 0x3f4e2c — __ZN3RBX10Reflection11Call1HelperINS_17CollectionServiceEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEESsESsSB_E4callEPS2_SD_RNS0_7VariantERKSs
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::CollectionService,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::CollectionService::*)(std::string),std::string,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::call(RBX::CollectionService*,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::CollectionService::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call1HelperINS_17CollectionServiceEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEESsESsSB_E4callEPS2_SD_RNS0_7VariantERKSs")]
// was: RBX::Reflection::Call1Helper<RBX::CollectionService,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::CollectionService::*)(std::string),std::string,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>>::call(RBX::CollectionService*,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::CollectionService::*)(std::string),RBX::Reflection::Variant &,std::string const&)
pub fn stub_3f4e2c(
    get_tags: &dyn Fn(&str) -> Vec<SharedPtr<InstanceHandle>>,
    filter: &str,
) -> Vec<SharedPtr<InstanceHandle>> {
    // IDA 0x3f4e2c: virtual member-pointer adjust (`a3 & 1`: vtable lookup, 0x3f4e7a-0x3f4e8a),
    // argument string copy (0x3f4e94), member call (0x3f4ea2), result wrap:
    // `getSingleton<SharedPtr<vector<...>>>` + `placement_any::operator=` (0x3f4eae-0x3f4eba),
    // shared-count release (0x3f4ec0-0x3f4ec6), string teardown (0x3f4ed8-0x3f4f1e).
    // BUG: the out-`Variant` (`SharedPtr<vector<SharedPtr<Instance>> const>`) has no
    // payload in this crate's `Variant`; the row vector is returned directly.
    get_tags(filter)
}

// 0x4004bc — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17FilteredSelectionINS1_8InstanceEEES8_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::FilteredSelection<RBX::Instance>,RBX::FilteredSelection<RBX::Instance>>(rbx_core::SharedPtr<RBX::FilteredSelection<RBX::Instance>> const*,RBX::FilteredSelection<RBX::Instance> *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17FilteredSelectionINS1_8InstanceEEES8_EEvPKNS_10shared_ptrIT_EEPT0_")]
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::FilteredSelection<RBX::Instance>,RBX::FilteredSelection<RBX::Instance>>(boost::shared_ptr<RBX::FilteredSelection<RBX::Instance>> const*,RBX::FilteredSelection<RBX::Instance> *)const
pub fn stub_4004bc() {
    // IDA 0x4004bc: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0x4014dc — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17FilteredSelectionINS1_13ModelInstanceEEES8_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::FilteredSelection<RBX::ModelInstance>,RBX::FilteredSelection<RBX::ModelInstance>>(rbx_core::SharedPtr<RBX::FilteredSelection<RBX::ModelInstance>> const*,RBX::FilteredSelection<RBX::ModelInstance> *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17FilteredSelectionINS1_13ModelInstanceEEES8_EEvPKNS_10shared_ptrIT_EEPT0_")]
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::FilteredSelection<RBX::ModelInstance>,RBX::FilteredSelection<RBX::ModelInstance>>(boost::shared_ptr<RBX::FilteredSelection<RBX::ModelInstance>> const*,RBX::FilteredSelection<RBX::ModelInstance> *)const
pub fn stub_4014dc() {
    // IDA 0x4014dc: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0x4019f8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13ModelInstanceES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ModelInstance,RBX::ModelInstance>(rbx_core::SharedPtr<RBX::ModelInstance> const*,RBX::ModelInstance *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13ModelInstanceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ModelInstance,RBX::ModelInstance>(boost::shared_ptr<RBX::ModelInstance> const*,RBX::ModelInstance *)const
pub fn stub_4019f8() {
    // IDA 0x4019f8: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0x402c00 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17FilteredSelectionINS1_10PVInstanceEEES8_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::FilteredSelection<RBX::PVInstance>,RBX::FilteredSelection<RBX::PVInstance>>(rbx_core::SharedPtr<RBX::FilteredSelection<RBX::PVInstance>> const*,RBX::FilteredSelection<RBX::PVInstance> *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17FilteredSelectionINS1_10PVInstanceEEES8_EEvPKNS_10shared_ptrIT_EEPT0_")]
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::FilteredSelection<RBX::PVInstance>,RBX::FilteredSelection<RBX::PVInstance>>(boost::shared_ptr<RBX::FilteredSelection<RBX::PVInstance>> const*,RBX::FilteredSelection<RBX::PVInstance> *)const
pub fn stub_402c00() {
    // IDA 0x402c00: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0x416f78 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13ConfigurationES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Configuration,RBX::Configuration>(rbx_core::SharedPtr<RBX::Configuration> const*,RBX::Configuration *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13ConfigurationES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Configuration,RBX::Configuration>(boost::shared_ptr<RBX::Configuration> const*,RBX::Configuration *)const
pub fn stub_416f78() {
    // IDA 0x416f78: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0x418690 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19CornerWedgeInstanceES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CornerWedgeInstance,RBX::CornerWedgeInstance>(rbx_core::SharedPtr<RBX::CornerWedgeInstance> const*,RBX::CornerWedgeInstance *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19CornerWedgeInstanceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CornerWedgeInstance,RBX::CornerWedgeInstance>(boost::shared_ptr<RBX::CornerWedgeInstance> const*,RBX::CornerWedgeInstance *)const
pub fn stub_418690() {
    // IDA 0x418690: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0x41e040 — __ZN3RBX9DataModel15serverSavePlaceENS_8Instance10SaveFilterEN5boost8functionIFvNS3_10shared_ptrIKNS_10Reflection5TupleEEEEEENS4_IFvSsEEE
#[doc(alias = "RBX::DataModel::serverSavePlace(RBX::Instance::SaveFilter,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)")]
#[doc(alias = "__ZN3RBX9DataModel15serverSavePlaceENS_8Instance10SaveFilterEN5boost8functionIFvNS3_10shared_ptrIKNS_10Reflection5TupleEEEEEENS4_IFvSsEEE")]
// was: RBX::DataModel::serverSavePlace(RBX::Instance::SaveFilter,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)
pub fn stub_41e040(
    place_id: i32,
    base_url: &str,
    on_response: &dyn Fn(),
    on_error: &dyn Fn(&str),
    upload: &dyn Fn(&str, &dyn Fn(), &dyn Fn(&str)),
) -> String {
    // IDA 0x41e040: stringify the place id (`lcast_put_unsigned`, 0x41e0d8, negated with
    // `-` prefix, 0x41e0de-0x41e0e2), build `"?assetId=<id>"` (0x41e110) plus
    // `"&isAppCreation=true"` (0x41e134), resolve `ContentProvider::getBaseUrl`
    // (0x41e14c), append `"ide/publish/UploadExistingAsset"` (0x41e17a), clone both
    // `boost::function` callbacks (`assign_to_own`, 0x41e198-0x41e1a8), then
    // `DataModel::uploadPlace(url, ...)` (0x41e1c0) with teardown (0x41e1ca+).
    // BUG: `uploadPlace` and the `SharedPtr<Tuple const>` response payload live in
    // datamodel (unmodeled); the port exposes the upload as a sink and the response
    // callback as an opaque unit callback.
    let url = format!("{base_url}ide/publish/UploadExistingAsset?assetId={place_id}&isAppCreation=true");
    upload(&url, on_response, on_error);
    url
}

// 0x41e51c — __ZN3RBX9DataModel14savePlaceAsyncENS_8Instance10SaveFilterEN5boost8functionIFvNS3_10shared_ptrIKNS_10Reflection5TupleEEEEEENS4_IFvSsEEE
#[doc(alias = "RBX::DataModel::savePlaceAsync(RBX::Instance::SaveFilter,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)")]
#[doc(alias = "__ZN3RBX9DataModel14savePlaceAsyncENS_8Instance10SaveFilterEN5boost8functionIFvNS3_10shared_ptrIKNS_10Reflection5TupleEEEEEENS4_IFvSsEEE")]
// was: RBX::DataModel::savePlaceAsync(RBX::Instance::SaveFilter,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)
pub fn stub_41e51c(
    place_id: i32,
    is_server_script: Option<bool>,
    filter: i32,
    on_response: &dyn Fn(),
    on_error: &dyn Fn(&str),
    server_save: &dyn Fn(i32, &dyn Fn(), &dyn Fn(&str)),
) {
    // IDA 0x41e51c: invalid place id (`a1[863] <= 0`, 0x41e570) reports
    // `"Game:SavePlace placeID is not valid!"` (0x41e5f6-0x41e602); client context
    // (`frontendProcessing(...) == 1`, 0x41e582) reports `"Game:SavePlace can only be
    // called from a server script, aborting save function"` (0x41e596-0x41e5a2);
    // backend context (`backendProcessing(...) == 1`, 0x41e64c) clones both callbacks
    // (0x41e654-0x41e66c) and tail-calls `serverSavePlace` (0x41e680) with teardown
    // (0x41e686-0x41e6a2); undetermined context reports `"Game:SavePlace could not
    // determine if client or server"` (0x41e6ba-0x41e6c6).
    // BUG: place-id storage, `Players` processing checks, and the string refcount
    // teardown fold into the `place_id` / `is_server_script` params (`None` =
    // undetermined).
    if place_id <= 0 {
        on_error("Game:SavePlace placeID is not valid!");
    } else if is_server_script == Some(false) {
        on_error("Game:SavePlace can only be called from a server script, aborting save function");
    } else if is_server_script == Some(true) {
        server_save(filter, on_response, on_error);
    } else {
        on_error("Game:SavePlace could not determine if client or server");
    }
}

// 0x42fb68 — __ZN3RBXL13appendJobInfoEPNS_9DataModelEN5boost10shared_ptrIKNS_13TaskScheduler3JobEEEPSt6vectorINS_10Reflection7VariantESaISA_EE
#[doc(alias = "RBX::appendJobInfo(RBX::DataModel *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *)")]
#[doc(alias = "__ZN3RBXL13appendJobInfoEPNS_9DataModelEN5boost10shared_ptrIKNS_13TaskScheduler3JobEEEPSt6vectorINS_10Reflection7VariantESaISA_EE")]
// was: RBX::appendJobInfo(RBX::DataModel *,boost::shared_ptr<RBX::TaskScheduler::Job const>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *)
pub fn stub_42fb68(model_tag: u32, job: &JobSample, out: &mut Vec<Vec<Variant>>) {
    // IDA 0x42fb68: owner guard (`*(job + 23) == model + 184`, 0x42fbfc); on hit build a
    // fresh row (`make_shared<vector<Variant>>`, 0x42fc08) and push: name string at
    // +112 (0x42fc1e-0x42fc52), `averageDutyCycle` (0x42fc74-0x42fcb8),
    // `averageStepsPerSecond` (0x42fcda-0x42fd12), `averageStepTime` (0x42fd34-0x42fd6c),
    // `averageError` (0x42fd8e-0x42fdc6), active flag (`*(job + 34) == 3`, 0x42fde4-0x42fe22),
    // then wrap the row (`typed_holder<SharedPtr<vector<Variant> const>>`,
    // 0x42fe48-0x42fe88) and push it into the out vector (0x42fe92-0x42fea2).
    // BUG: doubles narrow to `Variant::Float` (this crate's `Variant` has no `Double`
    // payload) and the shared row pushes as an owned `Vec`, not a `SharedPtr` holder.
    if job.owner_tag != model_tag {
        return;
    }
    out.push(vec![
        Variant::Text(job.name.clone()),
        Variant::Float(job.average_duty_cycle as f32),
        Variant::Float(job.average_steps_per_second as f32),
        Variant::Float(job.average_step_time as f32),
        Variant::Float(job.average_error as f32),
        Variant::Bool(job.state == 3),
    ]);
}

// 0x43001c — __ZN3RBXL22appendJobExtendedStatsEPNS_9DataModelEN5boost10shared_ptrIKNS_13TaskScheduler3JobEEEPSt6vectorINS_10Reflection7VariantESaISA_EE
#[doc(alias = "RBX::appendJobExtendedStats(RBX::DataModel *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *)")]
#[doc(alias = "__ZN3RBXL22appendJobExtendedStatsEPNS_9DataModelEN5boost10shared_ptrIKNS_13TaskScheduler3JobEEEPSt6vectorINS_10Reflection7VariantESaISA_EE")]
// was: RBX::appendJobExtendedStats(RBX::DataModel *,boost::shared_ptr<RBX::TaskScheduler::Job const>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *)
pub fn stub_43001c(model_tag: u32, stats: &JobDutyStats, out: &mut Vec<Vec<Variant>>) {
    // IDA 0x43001c: owner guard (`*(job + 23) == model + 184`, 0x4300b2); on hit build a
    // fresh row (`make_shared<vector<Variant>>`, 0x4300be), fill duty stats via
    // `WindowAverageDutyCycle::getStats(job[99], -1)` (0x4300d0), push the name string
    // at +112 (0x4300e6-0x43011a) plus seven doubles (0x430138-0x430334), then wrap the
    // row and push it into the out vector (0x43034a-0x430394). Same wrap/push shape as
    // `appendJobInfo` at 0x42fe48-0x42fe92.
    // BUG: see `stub_42fb68` — doubles narrow to `Variant::Float`, row pushes owned.
    if stats.owner_tag != model_tag {
        return;
    }
    let mut row = Vec::with_capacity(8);
    row.push(Variant::Text(stats.name.clone()));
    for value in stats.values {
        row.push(Variant::Float(value as f32));
    }
    out.push(row);
}

// 0x431100 — __ZN3RBX10ReflectionL14resume_adapterIN5boost10shared_ptrIKNS0_5TupleEEEEEvNS2_8functionIFvNS0_7VariantEEEET_
#[doc(alias = "void RBX::Reflection::resume_adapter<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)")]
#[doc(alias = "__ZN3RBX10ReflectionL14resume_adapterIN5boost10shared_ptrIKNS0_5TupleEEEEEvNS2_8functionIFvNS0_7VariantEEEET_")]
// was: void RBX::Reflection::resume_adapter<boost::shared_ptr<RBX::Reflection::Tuple const>>(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>)
pub fn stub_431100(callback: &dyn Fn(Variant), value: Variant) {
    // IDA 0x431100: `getSingleton<SharedPtr<Tuple const>>` (0x431136), copy the shared
    // count into the holder (0x43114e-0x431156), wrap into a `Variant` through
    // `typed_holder<SharedPtr<Tuple const>>` (0x431158-0x4311ac), invoke
    // `function1<void, Variant>` (0x4311ba), then tear down both holders
    // (0x4311c0-0x4311dc). Same shape as the bool/string/instance twins at 0x430e54,
    // 0x430fa8, and 0x702f60.
    // BUG: `Tuple` has no `Variant` payload in this crate; the caller passes the
    // already-wrapped value.
    callback(value);
}

// 0x4312ac — __ZN3RBX10Reflection9EventDescINS_9DataModelEFvN5boost10shared_ptrINS_8InstanceEEEPKNS0_18PropertyDescriptorEEN3rbx6signalISA_EEMS2_SD_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::DataModel,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> RBX::DataModel::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9DataModelEFvN5boost10shared_ptrINS_8InstanceEEEPKNS0_18PropertyDescriptorEEN3rbx6signalISA_EEMS2_SD_ED1Ev")]
// was: RBX::Reflection::EventDesc<RBX::DataModel,void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> RBX::DataModel::*>::~EventDesc()
pub fn stub_4312ac() {
    // IDA 0x4312ac: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x4312d0 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(RBX::ContentId),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEELi1EED1Ev")]
// was: RBX::Reflection::BoundFuncDesc<RBX::DataModel,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(RBX::ContentId),1>::~BoundFuncDesc()
pub fn stub_4312d0() {
    // IDA 0x4312d0: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x431310 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFN5boost10shared_ptrIKNS0_5TupleEEENS_8Instance10SaveFilterEES7_Li1EED1Ev
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(RBX::Instance::SaveFilter),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,1>::~BoundYieldFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFN5boost10shared_ptrIKNS0_5TupleEEENS_8Instance10SaveFilterEES7_Li1EED1Ev")]
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,boost::shared_ptr<RBX::Reflection::Tuple const> ()(RBX::Instance::SaveFilter),boost::shared_ptr<RBX::Reflection::Tuple const>,1>::~BoundYieldFuncDesc()
pub fn stub_431310() {
    // IDA 0x431310: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x431390 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS_9ContentIdEELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::ContentId),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS_9ContentIdEELi1EED1Ev")]
pub fn stub_431390() {
    // IDA 0x431390: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x431434 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsELi1EED1Ev")]
pub fn stub_431434() {
    // IDA 0x431434: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x431474 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFSsSsESsLi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,std::string ()(std::string),std::string,1>::~BoundYieldFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFSsSsESsLi1EED1Ev")]
pub fn stub_431474() {
    // IDA 0x431474: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x4314b4 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFSsSsSsESsLi2EED1Ev
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,std::string ()(std::string,std::string),std::string,2>::~BoundYieldFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFSsSsSsESsLi2EED1Ev")]
pub fn stub_4314b4() {
    // IDA 0x4314b4: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x4314fc — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFSsSsbELi2EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,std::string ()(std::string,bool),2>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFSsSsbELi2EED1Ev")]
pub fn stub_4314fc() {
    // IDA 0x4314fc: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x431544 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFSsSsSsbELi3EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,std::string ()(std::string,std::string,bool),3>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFSsSsSsbELi3EED1Ev")]
pub fn stub_431544() {
    // IDA 0x431544: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x431594 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EED1Ev")]
// was: RBX::Reflection::BoundFuncDesc<RBX::DataModel,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::~BoundFuncDesc()
pub fn stub_431594() {
    // IDA 0x431594: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x4315b8 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsSsSsSsSsELi5EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string,std::string,std::string,std::string,std::string),5>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsSsSsSsSsELi5EED1Ev")]
pub fn stub_4315b8() {
    // IDA 0x4315b8: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x43164c — __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFbvEbLi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,bool ()(void),bool,0>::~BoundYieldFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFbvEbLi0EED1Ev")]
pub fn stub_43164c() {
    // IDA 0x43164c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x43177c — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFdSsdELi2EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,double ()(std::string,double),2>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFdSsdELi2EED1Ev")]
pub fn stub_43177c() {
    // IDA 0x43177c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x4317c4 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvdELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(double),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvdELi1EED1Ev")]
pub fn stub_4317c4() {
    // IDA 0x4317c4: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x431804 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvibELi2EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,bool),2>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvibELi2EED1Ev")]
pub fn stub_431804() {
    // IDA 0x431804: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x43184c — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFviNS2_11CreatorTypeEELi2EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,RBX::DataModel::CreatorType),2>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFviNS2_11CreatorTypeEELi2EED1Ev")]
pub fn stub_43184c() {
    // IDA 0x43184c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x431894 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_5GenreEELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::DataModel::Genre),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_5GenreEELi1EED1Ev")]
pub fn stub_431894() {
    // IDA 0x431894: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x4318d4 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_16GearGenreSettingEiELi2EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::DataModel::GearGenreSetting,int),2>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_16GearGenreSettingEiELi2EED1Ev")]
pub fn stub_4318d4() {
    // IDA 0x4318d4: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x431924 — __ZN3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEED1Ev
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::~RefPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEED1Ev")]
pub fn stub_431924() {
    // IDA 0x431924: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x431950 — __ZN3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEED1Ev
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::~RefPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEED1Ev")]
pub fn stub_431950() {
    // IDA 0x431950: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x4319c0 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEED1Ev")]
pub fn stub_4319c0() {
    // IDA 0x4319c0: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x4319ec — __ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEED1Ev")]
pub fn stub_4319ec() {
    // IDA 0x4319ec: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x431a18 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEED1Ev")]
pub fn stub_431a18() {
    // IDA 0x431a18: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x431a3c — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFbNS2_8GearTypeEELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,bool ()(RBX::DataModel::GearType),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFbNS2_8GearTypeEELi1EED1Ev")]
pub fn stub_431a3c() {
    // IDA 0x431a3c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x431a7c — __ZN3RBX10Reflection9EventDescINS_9DataModelEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::DataModel,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::DataModel::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9DataModelEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev")]
pub fn stub_431a7c() {
    // IDA 0x431a7c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x431ad4 — __ZN3RBX10Reflection9EventDescINS_9DataModelEFvbEN3rbx6signalIS3_EEMS2_S6_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::DataModel,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::DataModel::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9DataModelEFvbEN3rbx6signalIS3_EEMS2_S6_ED1Ev")]
pub fn stub_431ad4() {
    // IDA 0x431ad4: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x431b00 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsSsELi2EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string,std::string),2>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsSsELi2EED1Ev")]
pub fn stub_431b00() {
    // IDA 0x431b00: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x431b48 — __ZN3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::addPair(RBX::DataModel::CreatorType,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE7addPairES3_PKc")]
pub fn stub_431b48(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0x431b48: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0x431ea8 — __ZN3RBX10Reflection7Variant14genericConvertINS_9DataModel11CreatorTypeEEERT_v
#[doc(alias = "RBX::DataModel::CreatorType & RBX::Reflection::Variant::genericConvert<RBX::DataModel::CreatorType>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_9DataModel11CreatorTypeEEERT_v")]
pub fn stub_431ea8(desc: &EnumDesc, variant: &Variant) -> i32 {
    // IDA 0x431ea8: `any_cast<CreatorType>` hit returns the stored value (0x431efc);
    // string payload goes through `StringConverter<CreatorType>::convertToValue`
    // (0x431f32-0x431f44), is stored back via `placement_any::operator=` with the type
    // singleton update (0x431f6e-0x431f78), and returns (0x431f82); anything else throws
    // `runtime_error("Unable to cast %s to %s")` (0x431fd0-0x432028).
    // BUG: the `any` cell is this crate's `Variant`; a direct enum payload arrives as
    // `Int`, string names resolve through the shared `EnumDesc` (missing names throw,
    // mirroring `convertToValue != 1` at 0x431f62).
    match variant {
        Variant::Int(v) => *v,
        Variant::Text(s) => desc.lookup_value(s).unwrap_or_else(|| {
            panic!("Unable to cast string to CreatorType (IDA 0x431fd0)")
        }),
        _ => panic!("Unable to cast Variant to CreatorType (IDA 0x431fd0)"),
    }
}

// 0x432094 — __ZN3RBX10Reflection8EnumDescINS_9DataModel5GenreEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::addPair(RBX::DataModel::Genre,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9DataModel5GenreEE7addPairES3_PKc")]
pub fn stub_432094(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0x432094: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0x4323f4 — __ZN3RBX10Reflection7Variant14genericConvertINS_9DataModel5GenreEEERT_v
#[doc(alias = "RBX::DataModel::Genre & RBX::Reflection::Variant::genericConvert<RBX::DataModel::Genre>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_9DataModel5GenreEEERT_v")]
pub fn stub_4323f4(desc: &EnumDesc, variant: &Variant) -> i32 {
    // IDA 0x4323f4: `genericConvert<Genre>` — same shape as the CreatorType twin at
    // 0x431ea8: `any_cast` hit, string via `StringConverter<Genre>::convertToValue`,
    // else `runtime_error("Unable to cast %s to %s")`.
    // BUG: see `stub_431ea8`.
    match variant {
        Variant::Int(v) => *v,
        Variant::Text(s) => desc.lookup_value(s).unwrap_or_else(|| {
            panic!("Unable to cast string to Genre (IDA 0x4323f4)")
        }),
        _ => panic!("Unable to cast Variant to Genre (IDA 0x4323f4)"),
    }
}

// 0x4325e0 — __ZN3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::addPair(RBX::DataModel::GearGenreSetting,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE7addPairES3_PKc")]
pub fn stub_4325e0(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0x4325e0: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0x432940 — __ZN3RBX10Reflection7Variant14genericConvertINS_9DataModel16GearGenreSettingEEERT_v
#[doc(alias = "RBX::DataModel::GearGenreSetting & RBX::Reflection::Variant::genericConvert<RBX::DataModel::GearGenreSetting>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_9DataModel16GearGenreSettingEEERT_v")]
pub fn stub_432940(desc: &EnumDesc, variant: &Variant) -> i32 {
    // IDA 0x432940: `genericConvert<GearGenreSetting>` — same shape as the CreatorType
    // twin at 0x431ea8: `any_cast` hit, string via `StringConverter`, else
    // `runtime_error("Unable to cast %s to %s")`.
    // BUG: see `stub_431ea8`.
    match variant {
        Variant::Int(v) => *v,
        Variant::Text(s) => desc.lookup_value(s).unwrap_or_else(|| {
            panic!("Unable to cast string to GearGenreSetting (IDA 0x432940)")
        }),
        _ => panic!("Unable to cast Variant to GearGenreSetting (IDA 0x432940)"),
    }
}

// 0x432b2c — __ZN3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::addPair(RBX::DataModel::GearType,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE7addPairES3_PKc")]
pub fn stub_432b2c(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0x432b2c: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0x432e8c — __ZN3RBX10Reflection7Variant14genericConvertINS_9DataModel8GearTypeEEERT_v
#[doc(alias = "RBX::DataModel::GearType & RBX::Reflection::Variant::genericConvert<RBX::DataModel::GearType>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_9DataModel8GearTypeEEERT_v")]
pub fn stub_432e8c(desc: &EnumDesc, variant: &Variant) -> i32 {
    // IDA 0x432e8c: `genericConvert<GearType>` — same shape as the CreatorType twin at
    // 0x431ea8: `any_cast` hit, string via `StringConverter`, else
    // `runtime_error("Unable to cast %s to %s")`.
    // BUG: see `stub_431ea8`.
    match variant {
        Variant::Int(v) => *v,
        Variant::Text(s) => desc.lookup_value(s).unwrap_or_else(|| {
            panic!("Unable to cast string to GearType (IDA 0x432e8c)")
        }),
        _ => panic!("Unable to cast Variant to GearType (IDA 0x432e8c)"),
    }
}

// 0x433078 — __ZN3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::addPair(RBX::Instance::SaveFilter,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEE7addPairES3_PKc")]
pub fn stub_433078(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0x433078: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0x4333d8 — __ZN3RBX10Reflection7Variant14genericConvertINS_8Instance10SaveFilterEEERT_v
#[doc(alias = "RBX::Instance::SaveFilter & RBX::Reflection::Variant::genericConvert<RBX::Instance::SaveFilter>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_8Instance10SaveFilterEEERT_v")]
pub fn stub_4333d8(desc: &EnumDesc, variant: &Variant) -> i32 {
    // IDA 0x4333d8: `genericConvert<SaveFilter>` — same shape as the CreatorType twin at
    // 0x431ea8: `any_cast` hit, string via `StringConverter`, else
    // `runtime_error("Unable to cast %s to %s")`.
    // BUG: see `stub_431ea8`.
    match variant {
        Variant::Int(v) => *v,
        Variant::Text(s) => desc.lookup_value(s).unwrap_or_else(|| {
            panic!("Unable to cast string to SaveFilter (IDA 0x4333d8)")
        }),
        _ => panic!("Unable to cast Variant to SaveFilter (IDA 0x4333d8)"),
    }
}

// 0x437b50 — __ZN3RBX32shared_from_polymorphic_downcastINS_5Stats4ItemENS_10Reflection13DescribedBaseEEEN5boost10shared_ptrIT_EEPNS5_23enable_shared_from_thisIT0_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::Stats::Item> RBX::shared_from_polymorphic_downcast<RBX::Stats::Item,RBX::Reflection::DescribedBase>(boost::enable_shared_from_this<RBX::Reflection::DescribedBase> *)")]
#[doc(alias = "__ZN3RBX32shared_from_polymorphic_downcastINS_5Stats4ItemENS_10Reflection13DescribedBaseEEEN5boost10shared_ptrIT_EEPNS5_23enable_shared_from_thisIT0_EE")]
// was: boost::shared_ptr<RBX::Stats::Item> RBX::shared_from_polymorphic_downcast<RBX::Stats::Item,RBX::Reflection::DescribedBase>(boost::enable_shared_from_this<RBX::Reflection::DescribedBase> *)
pub fn stub_437b50(base: Option<&SharedPtr<InstanceHandle>>) -> Option<SharedPtr<InstanceHandle>> {
    // IDA 0x437b50: null in writes a null out (`*out = 0`, 0x437bdc); else
    // `enable_shared_from_this<DescribedBase>::shared_from_this` (0x437ba8), member
    // adjust (`v12 - 36`, 0x437bb4), shared-count copy (0x437bc4), release the temp
    // (0x437bca-0x437bd2). The `Stats::Item` downcast itself is a
    // `polymorphic_downcast` (unchecked in release).
    // BUG: `Stats::Item` is unmodeled in this crate; the identity clone stands in for
    // the downcast.
    base.map(SharedPtr::clone)
}

// 0x437ef0 — __ZN3rbx11make_sharedISt6vectorIN3RBX10Reflection7VariantESaIS4_EEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>> rbx::make_shared<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>(void)")]
#[doc(alias = "__ZN3rbx11make_sharedISt6vectorIN3RBX10Reflection7VariantESaIS4_EEEEN5boost10shared_ptrIT_EEv")]
// was: boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>> rbx::make_shared<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>(void)
pub fn stub_437ef0() -> SharedPtr<Vec<Variant>> {
    // IDA 0x437ef0: single-alloc `make_shared` (0x437f5a): fetch the `sp_ms_deleter`
    // (0x437f7e), init the empty vector (begin = end = 0, capacity = 0, 0x437f88-0x437f8e),
    // set the uses-allocator marker (`*v9 = 1`, 0x437f90), publish the pointer
    // (0x437f94) with a shared-count copy (0x437fa0) and temp release (0x437fa6-0x437fae).
    // Rust: one `Arc` allocation around an empty vector covers it.
    SharedPtr::new(Vec::new())
}

// 0x438048 — __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESt6vectorIS8_SaIS8_EEEENS2_3_bi6bind_tIvPFvPNS4_9DataModelES8_PSA_INS4_10Reflection7VariantESaISJ_EEENSE_5list3INSE_5valueISH_EENS2_3argILi1EEENSQ_ISM_EEEEEEET0_T_SY_SX_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(RBX::DataModel *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::vector*<RBX::Reflection::Variant,std::allocator<RBX::Reflection>>),boost::_bi::list3<boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::vector*<RBX::Reflection::Variant,std::allocator<RBX::Reflection>>>>> std::for_each<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>>,boost::_bi::bind_t<void,void (*)(RBX::DataModel *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::vector*<RBX::Reflection::Variant,std::allocator<RBX::Reflection>>),boost::_bi::list3<boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::vector*<RBX::Reflection::Variant,std::allocator<RBX::Reflection>>>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>>,boost::_bi::bind_t<void,void (*)(RBX::DataModel *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::vector*<RBX::Reflection::Variant,std::allocator<RBX::Reflection>>),boost::_bi::list3<boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::vector*<RBX::Reflection::Variant,std::allocator<RBX::Reflection>>>>>)")]
#[doc(alias = "__ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESt6vectorIS8_SaIS8_EEEENS2_3_bi6bind_tIvPFvPNS4_9DataModelES8_PSA_INS4_10Reflection7VariantESaISJ_EEENSE_5list3INSE_5valueISH_EENS2_3argILi1EEENSQ_ISM_EEEEEEET0_T_SY_SX_")]
// was: boost::_bi::bind_t<void,void (*)(RBX::DataModel *,boost::shared_ptr<RBX::TaskScheduler::Job const>,std::vector*<RBX::Reflection::Variant,std::allocator<RBX::Reflection>>),boost::_bi::list3<boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::vector*<RBX::Reflection::Variant,std::allocator<RBX::Reflection>>>>> std::for_each<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TaskScheduler::Job const> *,std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>>>,boost::_bi::bind_t<void,void (*)(RBX::DataModel *,boost::shared_ptr<RBX::TaskScheduler::Job const>,std::vector*<RBX::Reflection::Variant,std::allocator<RBX::Reflection>>),boost::_bi::list3<boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::vector*<RBX::Reflection::Variant,std::allocator<RBX::Reflection>>>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TaskScheduler::Job const> *,std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TaskScheduler::Job const> *,std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>>>,boost::_bi::bind_t<void,void (*)(RBX::DataModel *,boost::shared_ptr<RBX::TaskScheduler::Job const>,std::vector*<RBX::Reflection::Variant,std::allocator<RBX::Reflection>>),boost::_bi::list3<boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::vector*<RBX::Reflection::Variant,std::allocator<RBX::Reflection>>>>>)
pub fn stub_438048(model_tag: u32, jobs: &[JobSample], out: &mut Vec<Vec<Variant>>) {
    // IDA 0x438048: `for_each` over the `vector<SharedPtr<Job const>>` (0x438064):
    // each element runs the bound triple `appendJobInfo(model, _1, out-vec)`
    // (`list3::operator()`, 0x438078); the bound copy is returned through the out
    // functor param (0x438082-0x43808c). Rust: the triple folds into a direct call.
    for job in jobs {
        stub_42fb68(model_tag, job, out);
    }
}

// 0x4382c8 — __ZN3rbx7signals16signal_with_argsILi2EFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEEclES6_SA_
#[doc(alias = "rbx::signals::signal_with_args<2,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::operator()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)")]
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi2EFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEEclES6_SA_")]
// was: rbx::signals::signal_with_args<2,void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::operator()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)
pub fn stub_4382c8(
    signal: &Signal<(SharedPtr<InstanceHandle>, u32)>,
    instance: &SharedPtr<InstanceHandle>,
    prop_desc: u32,
) {
    // IDA 0x4382c8: no-op when the signal has no slots (`*a1 == 0`, 0x4382fa); else
    // optional `FastLog("Signal with 2 args executed")` (0x43832c-0x438340), then loop
    // `signal::next` (0x43836a): shared-count copy of the instance arg (0x43837e),
    // invoke each slot's invoker with `(instance, propdesc)` (0x43838a-0x4383b8) with
    // per-slot releases (0x4383bc-0x4383d0), slot-list release at the end
    // (0x43847c-0x438486). `Signal::fire` covers the iteration; the log is observability
    // only.
    signal.fire((SharedPtr::clone(instance), prop_desc));
}

// 0x4388c4 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE6lookupERKNS0_7VariantE")]
pub fn stub_4388c4(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0x4388c4: EnumDesc<T>::lookup(Variant) -- rbx::any_cast<T> the payload, then convertToItem (decompiled 0xb97c). Variant is unmodeled in this crate; the caller passes the already-cast enum value, and this is convertToItem exactly.
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0x4388e4 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_4388e4() {
    // IDA 0x4388e4: EnumDesc<T>::convertToValue(index, Variant&) -- writes the converted value into a Variant out-param; Variant is unmodeled in this crate: cutover no-op. See the (desc, name, &mut i32) sibling for the lookup semantics.
}

// 0x438918 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE15convertToStringEmRSs")]
pub fn stub_438918(desc: &crate::enum_desc::EnumDesc, index: usize, out: &mut String) -> bool {
    // IDA 0x438918: EnumDesc<T>::convertToString(index, string&) -- if index < items.size(): out = items[index].name, return true; else return false, out untouched (decompiled 0x957bd4).
    if let Some(item) = desc.items.get(index) {
        *out = item.name.clone();
        true
    } else {
        false
    }
}

// 0x438b30 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE6lookupERKNS0_7VariantE")]
pub fn stub_438b30(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0x438b30: EnumDesc<T>::lookup(Variant) -- rbx::any_cast<T> the payload, then convertToItem (decompiled 0xb97c). Variant is unmodeled in this crate; the caller passes the already-cast enum value, and this is convertToItem exactly.
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0x438b50 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_438b50() {
    // IDA 0x438b50: EnumDesc<T>::convertToValue(index, Variant&) -- writes the converted value into a Variant out-param; Variant is unmodeled in this crate: cutover no-op. See the (desc, name, &mut i32) sibling for the lookup semantics.
}

// 0x438b84 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE15convertToStringEmRSs")]
pub fn stub_438b84(desc: &crate::enum_desc::EnumDesc, index: usize, out: &mut String) -> bool {
    // IDA 0x438b84: EnumDesc<T>::convertToString(index, string&) -- if index < items.size(): out = items[index].name, return true; else return false, out untouched (decompiled 0x957bd4).
    if let Some(item) = desc.items.get(index) {
        *out = item.name.clone();
        true
    } else {
        false
    }
}

// 0x438ccc — __ZN3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEED0Ev")]
pub fn stub_438ccc() {
    // IDA 0x438ccc: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x438d6c — __ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE6lookupEPKc")]
pub fn stub_438d6c(desc: &crate::enum_desc::EnumDesc, name: &str) -> usize {
    // IDA 0x438d6c: EnumDesc<T>::lookup(char const*) -- Name::lookup intern, search name_to_value then legacy_names; hit: return convertToItem(value); miss: return 0 (decompiled 0x957a18).
    desc.lookup_value(name).and_then(|v| usize::try_from(v).ok()).and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0x438d9c — __ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE6lookupERKNS0_7VariantE")]
pub fn stub_438d9c(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0x438d9c: EnumDesc<T>::lookup(Variant) -- rbx::any_cast<T> the payload, then convertToItem (decompiled 0xb97c). Variant is unmodeled in this crate; the caller passes the already-cast enum value, and this is convertToItem exactly.
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0x438dbc — __ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_438dbc() {
    // IDA 0x438dbc: EnumDesc<T>::convertToValue(index, Variant&) -- writes the converted value into a Variant out-param; Variant is unmodeled in this crate: cutover no-op. See the (desc, name, &mut i32) sibling for the lookup semantics.
}

// 0x438df0 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE15convertToStringEmRSs")]
pub fn stub_438df0(desc: &crate::enum_desc::EnumDesc, index: usize, out: &mut String) -> bool {
    // IDA 0x438df0: EnumDesc<T>::convertToString(index, string&) -- if index < items.size(): out = items[index].name, return true; else return false, out untouched (decompiled 0x957bd4).
    if let Some(item) = desc.items.get(index) {
        *out = item.name.clone();
        true
    } else {
        false
    }
}

// 0x438fd8 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE6lookupEPKc")]
pub fn stub_438fd8(desc: &crate::enum_desc::EnumDesc, name: &str) -> usize {
    // IDA 0x438fd8: EnumDesc<T>::lookup(char const*) -- Name::lookup intern, search name_to_value then legacy_names; hit: return convertToItem(value); miss: return 0 (decompiled 0x957a18).
    desc.lookup_value(name).and_then(|v| usize::try_from(v).ok()).and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0x439008 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE6lookupERKNS0_7VariantE")]
pub fn stub_439008(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0x439008: EnumDesc<T>::lookup(Variant) -- rbx::any_cast<T> the payload, then convertToItem (decompiled 0xb97c). Variant is unmodeled in this crate; the caller passes the already-cast enum value, and this is convertToItem exactly.
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0x439028 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_439028() {
    // IDA 0x439028: EnumDesc<T>::convertToValue(index, Variant&) -- writes the converted value into a Variant out-param; Variant is unmodeled in this crate: cutover no-op. See the (desc, name, &mut i32) sibling for the lookup semantics.
}

// 0x43905c — __ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE15convertToStringEmRSs")]
pub fn stub_43905c(desc: &crate::enum_desc::EnumDesc, index: usize, out: &mut String) -> bool {
    // IDA 0x43905c: EnumDesc<T>::convertToString(index, string&) -- if index < items.size(): out = items[index].name, return true; else return false, out untouched (decompiled 0x957bd4).
    if let Some(item) = desc.items.get(index) {
        *out = item.name.clone();
        true
    } else {
        false
    }
}

// 0x439244 — __ZNK3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEE6lookupEPKc")]
pub fn stub_439244(desc: &crate::enum_desc::EnumDesc, name: &str) -> usize {
    // IDA 0x439244: EnumDesc<T>::lookup(char const*) -- Name::lookup intern, search name_to_value then legacy_names; hit: return convertToItem(value); miss: return 0 (decompiled 0x957a18).
    desc.lookup_value(name).and_then(|v| usize::try_from(v).ok()).and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0x439274 — __ZNK3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEE6lookupERKNS0_7VariantE")]
pub fn stub_439274(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0x439274: EnumDesc<T>::lookup(Variant) -- rbx::any_cast<T> the payload, then convertToItem (decompiled 0xb97c). Variant is unmodeled in this crate; the caller passes the already-cast enum value, and this is convertToItem exactly.
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0x439294 — __ZNK3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_439294() {
    // IDA 0x439294: EnumDesc<T>::convertToValue(index, Variant&) -- writes the converted value into a Variant out-param; Variant is unmodeled in this crate: cutover no-op. See the (desc, name, &mut i32) sibling for the lookup semantics.
}

// 0x4392c8 — __ZNK3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEE15convertToStringEmRSs")]
pub fn stub_4392c8(desc: &crate::enum_desc::EnumDesc, index: usize, out: &mut String) -> bool {
    // IDA 0x4392c8: EnumDesc<T>::convertToString(index, string&) -- if index < items.size(): out = items[index].name, return true; else return false, out untouched (decompiled 0x957bd4).
    if let Some(item) = desc.items.get(index) {
        *out = item.name.clone();
        true
    } else {
        false
    }
}

// 0x43940c — __ZNK3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::convertToString(RBX::Instance::SaveFilter const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEE15convertToStringERKS3_")]
pub fn stub_43940c(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0x43940c: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0x439678 — __ZNK3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::convertToItem(RBX::Instance::SaveFilter const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEE13convertToItemERKS3_")]
pub fn stub_439678(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0x439678: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0x439834 — __ZNK3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::convertToValue(RBX::Name const&,RBX::Instance::SaveFilter&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_439834(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0x439834: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0x439a84 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::convertToString(RBX::DataModel::GearType const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE15convertToStringERKS3_")]
pub fn stub_439a84(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0x439a84: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0x439cf0 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::convertToItem(RBX::DataModel::GearType const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE13convertToItemERKS3_")]
pub fn stub_439cf0(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0x439cf0: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0x439eac — __ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::convertToValue(RBX::Name const&,RBX::DataModel::GearType&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_439eac(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0x439eac: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0x43a0fc — __ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::convertToString(RBX::DataModel::GearGenreSetting const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE15convertToStringERKS3_")]
pub fn stub_43a0fc(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0x43a0fc: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0x43a368 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::convertToItem(RBX::DataModel::GearGenreSetting const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE13convertToItemERKS3_")]
pub fn stub_43a368(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0x43a368: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0x43a524 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::convertToValue(RBX::Name const&,RBX::DataModel::GearGenreSetting&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_43a524(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0x43a524: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0x43a5a0 — __ZN3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEED2Ev")]
pub fn stub_43a5a0() {
    // IDA 0x43a5a0: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x43a774 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::convertToString(RBX::DataModel::Genre const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE15convertToStringERKS3_")]
pub fn stub_43a774(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0x43a774: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0x43a9e0 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::convertToItem(RBX::DataModel::Genre const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE13convertToItemERKS3_")]
pub fn stub_43a9e0(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0x43a9e0: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0x43ab9c — __ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::convertToValue(RBX::Name const&,RBX::DataModel::Genre&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_43ab9c(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0x43ab9c: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0x43adec — __ZNK3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::convertToString(RBX::DataModel::CreatorType const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE15convertToStringERKS3_")]
pub fn stub_43adec(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0x43adec: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0x43b058 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::convertToItem(RBX::DataModel::CreatorType const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE13convertToItemERKS3_")]
pub fn stub_43b058(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0x43b058: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}
