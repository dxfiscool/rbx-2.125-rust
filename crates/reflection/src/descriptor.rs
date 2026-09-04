//! RBX::Reflection::Descriptor skeletons from `ida/export.json`.
//! Remaining batch — compile-only cutover points.

// --- remaining batch (150) from ida/export.json: RBX::Reflection not yet stubbed, sorted by ea ---
use rbx_core::SharedPtr;
use rbx_core::signal::Signal;

/// Opaque `RBX::Instance` handle. Reflection only forwards it through signals and
/// variants; the real type lives in datamodel (which depends on this crate).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct InstanceHandle {
    pub id: u32,
}

/// Minimal `RBX::Reflection::Variant` covering the payloads used below.
#[derive(Debug, Clone)]
pub enum Variant {
    Int(i32),
    Float(f32),
    Instance(SharedPtr<InstanceHandle>),
}

impl Variant {
    /// `RBX::Reflection::Variant::convert<int>` as used at 0x4a5b2c: int payloads pass
    /// through, floats truncate; anything else threw in the original and panics here.
    pub fn convert_to_int(&self) -> i32 {
        match self {
            Variant::Int(v) => *v,
            Variant::Float(v) => *v as i32,
            Variant::Instance(_) => panic!("Variant::convert<int> on non-numeric payload (IDA 0x4a5a80)"),
        }
    }
}

/// `RBX::Reflection::GenericSlotWrapper`: wraps one generic slot. `execute2` packs
/// `(instance, value)` into a 2-Variant vector and dispatches the stored callable
/// (IDA 0x4a40c8: vector fill, `vfptr+8` call, vector teardown).
pub struct GenericSlotWrapper {
    pub invoke: Box<dyn Fn(&[Variant]) + Send + Sync>,
}

impl GenericSlotWrapper {
    pub fn execute2(&self, instance: &SharedPtr<InstanceHandle>, value: f32) {
        // IDA 0x4a40c8: `vector<Variant>{ (Instance, arg0), (float, arg1) }`, virtual
        // dispatch into the wrapped slot, then destroy the vector.
        (self.invoke)(&[
            Variant::Instance(SharedPtr::clone(instance)),
            Variant::Float(value),
        ]);
    }
}


/// Signature argument kinds of `EventDesc<Explosion, void(SharedPtr<Instance>, float)>`
/// (IDA 0x4a3966 `Type::getSingleton<SharedPtr<Instance>>`, 0x4a39a2 `getSingleton<float>`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExplosionEventArg {
    Instance,
    Float,
}

/// Connected slot: the original signal owns its slots until `disconnectAll`; the strong
/// refs live in `holders` because `Signal::connect` keeps only weak refs.
type ExplosionSlot = SharedPtr<dyn Fn((SharedPtr<InstanceHandle>, f32)) + Send + Sync>;

/// `RBX::Reflection::EventDesc<Explosion, void(SharedPtr<Instance>, float), ...>`
/// (IDA 0x4a38b8): base `EventDescriptor` init, member-signal pointer stored at +40
/// (`v54[10] = a2`), two-item signature list appended.
#[derive(Debug, Clone)]
pub struct ExplosionEventDesc {
    pub name: String,
    pub category: String,
    pub title: String,
    pub member: usize,
    pub signature: Vec<(String, ExplosionEventArg)>,
    pub permissions: u32,
    pub attributes: u32,
}

/// `RBX::Reflection::EventSource` for the Explosion signal: owns the connected slots.
/// Backed by `rbx_core::signal::Signal` (IDA 0x4a3b5c/0x4a3e20).
#[derive(Default)]
pub struct EventSource {
    signal: Signal<(SharedPtr<InstanceHandle>, f32)>,
    holders: parking_lot::Mutex<Vec<(SharedPtr<GenericSlotWrapper>, ExplosionSlot)>>,
}

impl EventSource {
    pub fn connect_slot(&self, wrapper: SharedPtr<GenericSlotWrapper>) {
        let w = SharedPtr::clone(&wrapper);
        let slot = std::sync::Arc::new(
            move |payload: (SharedPtr<InstanceHandle>, f32)| {
                w.execute2(&payload.0, payload.1);
            },
        );
        self.signal.connect(SharedPtr::clone(&slot));
        let slot: ExplosionSlot = slot;
        self.holders.lock().push((wrapper, slot));
    }

    pub fn fire(&self, instance: &SharedPtr<InstanceHandle>, value: f32) {
        self.signal.fire((SharedPtr::clone(instance), value));
    }

    pub fn disconnect_all(&self) {
        self.holders.lock().clear();
        self.signal.disconnect_all();
    }
}
/// `boost::_bi::bind_t<mf2<GenericSlotWrapper, ...>, list3<value<SharedPtr<GenericSlotWrapper>>, arg<1>, arg<2>>>`
/// (IDA 0x4a3fac): stores the member-function triple plus the bound wrapper and the
/// two placeholders. The member function is fixed (`execute2`), so the triple folds
/// into the target.
#[derive(Clone)]
pub struct BoundExplosionSlot {
    pub target: SharedPtr<GenericSlotWrapper>,
}

impl BoundExplosionSlot {
    /// `bind_t::operator()<SharedPtr<Instance>, float>` (IDA 0x4a47f4): member-pointer
    /// dispatch `(target->*mf)(args)`. The `(v1 & 1)` virtual-adjust branch is
    /// member-pointer mechanics with no Rust equivalent.
    pub fn call(&self, instance: &SharedPtr<InstanceHandle>, value: f32) {
        self.target.execute2(instance, value);
    }
}

/// `boost::function2<void, SharedPtr<Instance>, float>` holding one bound slot
/// (IDA 0x4a442c/0x4a4554/0x4a463c).
#[derive(Default, Clone)]
pub struct ExplosionSlotFunction {
    bound: Option<BoundExplosionSlot>,
}

impl ExplosionSlotFunction {
    pub fn is_empty(&self) -> bool {
        self.bound.is_none()
    }

    pub fn invoke(&self, instance: &SharedPtr<InstanceHandle>, value: f32) {
        // Calling an empty `boost::function` throws `bad_function_call`; panic mirrors it.
        self.bound
            .as_ref()
            .expect("bad_function_call")
            .call(instance, value);
    }
}

/// `boost::detail::function::functor_manager_operation_type` cases as switched at 0x4a4810.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctorOp {
    CloneFunctor,
    MoveFunctor,
    DestroyFunctor,
    CheckFunctorType,
    GetFunctorTypeInfo,
}

/// typeinfo name compared by `manager` case 3 (IDA 0x4a490a `strcmp` literal).
pub const EXPLOSION_BIND_T_TYPEINFO: &str = "N5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS4_8InstanceEEERKfEENS0_5list3INS0_5valueINS7_IS6_EEEENS_3argILi1EEENSJ_ILi2EEEEEEE";

/// Minimal `RBX::Explosion` state visible to its enum descriptor. The real type lives
/// in datamodel; the descriptor only reads/writes the reflected field through the
/// bound pair below.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ExplosionState {
    pub explosion_type: i32,
}

/// Get/set pair behind `EnumPropDescriptor<Explosion, ExplosionType>` (the +44 member
/// desc: IDA 0x4a5938 `new(0x14)` holding the getter/setter member pointers).
pub struct ExplosionTypeAccess {
    pub get: Box<dyn Fn(&ExplosionState) -> i32 + Send + Sync>,
    pub set: Box<dyn Fn(&mut ExplosionState, i32) + Send + Sync>,
}
/// `RBX::Reflection::EnumPropDescriptor<Explosion, ExplosionType>` (IDA 0x4a5834).
pub struct ExplosionEnumPropDesc {
    pub name: String,
    pub category: String,
    pub access: ExplosionTypeAccess,
    /// Singleton link stored at +40/+48 (IDA 0x4a58ea/0x4a5954).
    pub enum_desc: &'static crate::enum_desc::EnumDesc,
    pub attributes: u32,
    pub permissions: u32,
}
/// `Singleton<EnumDesc<ExplosionType>>::doGetSingleton` (IDA 0x4b6a3c: guard-once
/// construct + `__cxa_atexit`; C2 at 0x49f614 registers the pairs). Rust: `LazyLock`;
/// the destructor runs at process exit.
/// Items grounded in disasm 0x49f6f6/0x49f70c/0x49f722 (`MOVS R1, #0/#1/#2` into `addPair`).
static EXPLOSION_TYPE_DESC: std::sync::LazyLock<crate::enum_desc::EnumDesc> =
    std::sync::LazyLock::new(|| {
        let mut d = crate::enum_desc::EnumDesc::new("ExplosionType");
        d.add_pair(0, "NoCraters");
        d.add_pair(1, "Craters");
        d.add_pair(2, "CratersAndDebris");
        d
    });

pub fn explosion_type_enum_desc() -> &'static crate::enum_desc::EnumDesc {
    &EXPLOSION_TYPE_DESC
}

// 0x4a15b0 — __ZN3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::~EnumDesc()")]
pub fn stub_0x4a15b0() {
    // IDA 0x4a15b0: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x4a2ae8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12TimerServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TimerService,RBX::TimerService>(rbx_core::SharedPtr<RBX::TimerService> const*,RBX::TimerService *)const")]
pub fn stub_0x4a2ae8() {
    // IDA 0x4a2ae8: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0x4a38b8 — __ZN3RBX10Reflection9EventDescINS_9ExplosionEFvN5boost10shared_ptrINS_8InstanceEEEfEN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Explosion,void ()(rbx_core::SharedPtr<RBX::Instance>,float),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)> RBX::Explosion::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)> RBX::Explosion::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x4a38b8(
    member: usize,
    name: &str,
    category: &str,
    title: &str,
    permissions: u32,
    attributes: u32,
) -> ExplosionEventDesc {
    // IDA 0x4a38b8: base `EventDescriptor` init, member-signal pointer stored at +40
    // (`v54[10] = a2`), vtable install, then two signature items appended:
    // `(arg0_name, SharedPtr<Instance>)` (0x4a3966) and `(arg1_name, float)` (0x4a39a2).
    ExplosionEventDesc {
        name: name.to_owned(),
        category: category.to_owned(),
        title: title.to_owned(),
        member,
        signature: vec![
            (category.to_owned(), ExplosionEventArg::Instance),
            (title.to_owned(), ExplosionEventArg::Float),
        ],
        permissions,
        attributes,
    }
}

// 0x4a3aa8 — __ZN3RBX10Reflection9EventDescINS_9ExplosionEFvN5boost10shared_ptrINS_8InstanceEEEfEN3rbx6signalIS7_EEMS2_SA_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Explosion,void ()(rbx_core::SharedPtr<RBX::Instance>,float),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)> RBX::Explosion::*>::~EventDesc()")]
pub fn stub_0x4a3aa8() {
    // IDA 0x4a3aa8: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x4a3b5c — __ZNK3RBX10Reflection13EventDescImplILi2ENS_9ExplosionEFvN5boost10shared_ptrINS_8InstanceEEEfEN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::Explosion,void ()(rbx_core::SharedPtr<RBX::Instance>,float),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)> RBX::Explosion::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_0x4a3b5c(source: Option<&EventSource>, wrapper: SharedPtr<GenericSlotWrapper>) {
    // IDA 0x4a3b5c: builds `bind(execute2, wrapper, _1, _2)` (0x4a3bd4), wraps it in a
    // `boost::function` (0x4a3be0), then `signal::connect(member-signal-of-source, fn)`
    // (0x4a3bfc). Null source stores an empty connection (`*v44 = 0`, 0x4a3c06).
    if let Some(source) = source {
        source.connect_slot(wrapper);
    }
    // `function2::clear()` (0x4a3c0e) drops the temp; `Arc` drop glue covers it.
}

// 0x4a3cb0 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_9ExplosionEFvN5boost10shared_ptrINS_8InstanceEEEfEN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
pub fn stub_0x4a3cb0(source: &EventSource, args: &[Variant]) {
    // IDA 0x4a3cb0: `ReleaseAssert(args.size() == 2)` (Event.h:349, 0x4a3d14), then
    // `any_cast<SharedPtr<Instance>>(args[0])` (0x4a3d6c), `any_cast<float>(args[1])`
    // (0x4a3da2), and `signal_with_args<2>::operator()` (0x4a3db4).
    assert!(args.len() == 2, "args.size() == 2 include/Reflection/Event.h:349");
    let Variant::Instance(instance) = &args[0] else {
        panic!("any_cast<SharedPtr<Instance>> failed (IDA 0x4a3d6c)");
    };
    let Variant::Float(value) = &args[1] else {
        panic!("any_cast<float> failed (IDA 0x4a3da2)");
    };
    source.fire(instance, *value);
}

// 0x4a3e20 — __ZNK3RBX10Reflection13EventDescBaseINS_9ExplosionEFvN5boost10shared_ptrINS_8InstanceEEEfEN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Explosion,void ()(rbx_core::SharedPtr<RBX::Instance>,float),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)> RBX::Explosion::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x4a3e20(source: &EventSource) {
    // IDA 0x4a3e20: member-offset adjust (`a2 ? a2 - 36 : 0`, 0x4a3e24-0x4a3e26), then
    // `signal::disconnectAll(member)`. The adjust is member-pointer mechanics; the
    // observable effect is dropping every slot.
    source.disconnect_all();
}

// 0x4a3fac — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS1_8InstanceEEERKfNS4_IS3_EENS_3argILi1EEENSC_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISH_T0_T1_T2_EENSF_9list_av_3IT3_T4_T5_E4typeEEEMSK_FSH_SL_SM_ESP_SQ_SR_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list_av_3<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(rbx_core::SharedPtr<RBX::Instance> const&,float const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")]
pub fn stub_0x4a3fac(wrapper: SharedPtr<GenericSlotWrapper>) -> BoundExplosionSlot {
    // IDA 0x4a3fac: `list3(value(wrapper-shared), arg<1>, arg<2>)` (0x4a4016) plus the
    // member-function triple stored into the bind_t out (0x4a401e-0x4a4034). The member
    // function is fixed (`execute2`), so the triple folds into the bound target.
    BoundExplosionSlot { target: wrapper }
}

// 0x4a40c8 — __ZN3RBX10Reflection18GenericSlotWrapper8execute2IN5boost10shared_ptrINS_8InstanceEEEfEEvRKT_RKT0_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute2<rbx_core::SharedPtr<RBX::Instance>,float>(rbx_core::SharedPtr<RBX::Instance> const&,float const&)")]
pub fn stub_0x4a40c8(
    wrapper: &GenericSlotWrapper,
    instance: &SharedPtr<InstanceHandle>,
    value: f32,
) {
    // IDA 0x4a40c8: packs `vector<Variant>{ (Instance, arg0), (float, arg1) }`
    // (0x4a413c-0x4a418a), dispatches the wrapped slot (`vfptr+8`, 0x4a419a), destroys
    // the vector (0x4a41a4).
    wrapper.execute2(instance, value);
}

// 0x4a442c — __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_RKfEENS7_5list3INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEvT_
#[doc(alias = "void boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")]
pub fn stub_0x4a442c(func: &mut ExplosionSlotFunction, bound: &BoundExplosionSlot) {
    // IDA 0x4a442c: copies the bind_t triple plus shared count into a temp (0x4a4450-0x4a4464),
    // delegates to `basic_vtable2::assign_to(stored_vtable, tmp, buf)` (0x4a44b4), releases
    // the temp (0x4a44ba). Net effect: the function object owns a clone of the functor.
    stub_0x4a4554(func, bound);
}

// 0x4a4524 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKfEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x4a4524(
    op: FunctorOp,
    src: &BoundExplosionSlot,
    slot: &mut Option<Box<BoundExplosionSlot>>,
) -> &'static str {
    // IDA 0x4a4524: any op but 4 delegates to `manager()` (0x4a4528); op 4 answers the
    // bind_t typeinfo without touching the buffers (0x4a453a-0x4a453e). Either way the
    // call reports the functor type.
    if op != FunctorOp::GetFunctorTypeInfo {
        stub_0x4a4810(op, src, slot);
    }
    EXPLOSION_BIND_T_TYPEINFO
}

// 0x4a4540 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKfEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEvSC_fE6invokeERNS1_15function_bufferESC_f
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,rbx_core::SharedPtr<RBX::Instance>,float>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>,float)")]
pub fn stub_0x4a4540(
    bound: &BoundExplosionSlot,
    instance: &SharedPtr<InstanceHandle>,
    value: f32,
) {
    // IDA 0x4a4540: tail-jumps to `bind_t::operator()<SharedPtr<Instance>, float>` (0x4a4552).
    stub_0x4a47f4(bound, instance, value);
}

// 0x4a4554 — __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEEfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS4_10Reflection18GenericSlotWrapperERKS6_RKfEENS9_5list3INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,rbx_core::SharedPtr<RBX::Instance>,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x4a4554(func: &mut ExplosionSlotFunction, bound: &BoundExplosionSlot) -> bool {
    // IDA 0x4a4554: copies the functor triple (0x4a4574-0x4a458e), delegates to the
    // tag-dispatch overload (0x4a45d2), releases the temp, returns 1 (0x4a4600).
    stub_0x4a463c(func, bound)
}

// 0x4a463c — __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEEfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS4_10Reflection18GenericSlotWrapperERKS6_RKfEENS9_5list3INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,rbx_core::SharedPtr<RBX::Instance>,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x4a463c(func: &mut ExplosionSlotFunction, bound: &BoundExplosionSlot) -> bool {
    // IDA 0x4a463c: copies the functor triple (0x4a465c-0x4a468a), heap-clones it via
    // `assign_functor` (0x4a46b4), releases the temp, returns 1 (0x4a46e2).
    func.bound = Some(*stub_0x4a4720(bound));
    true
}

// 0x4a4720 — __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEEfE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS4_10Reflection18GenericSlotWrapperERKS6_RKfEENS9_5list3INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable2<void,rbx_core::SharedPtr<RBX::Instance>,float>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x4a4720(bound: &BoundExplosionSlot) -> Box<BoundExplosionSlot> {
    // IDA 0x4a4720 (`mpl::bool_<false>` = not-small-object): `operator new(0x10)`
    // (0x4a4748), 16-byte functor copy plus shared-count bump (0x4a475a-0x4a47a2),
    // out-ptr store (0x4a47aa). Rust: the heap clone is `Box::new`.
    Box::new(bound.clone())
}

// 0x4a47f4 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS4_8InstanceEEERKfEENS0_5list3INS0_5valueINS7_IS6_EEEENS_3argILi1EEENSJ_ILi2EEEEEEclIS9_fEEvRT_RT0_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<rbx_core::SharedPtr<RBX::Instance>,float>(rbx_core::SharedPtr<RBX::Instance> &,float &)")]
pub fn stub_0x4a47f4(
    bound: &BoundExplosionSlot,
    instance: &SharedPtr<InstanceHandle>,
    value: f32,
) {
    // IDA 0x4a47f4: member-function dispatch out of the bind_t triple (0x4a47f4-0x4a4808):
    // adjust the stored object (`v1 >> 1`, virtual via `v1 & 1`), call through it,
    // forwarding the `(instance, value)` call args. Rust folds the triple into the target.
    bound.call(instance, value);
}

// 0x4a4810 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKfEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x4a4810(
    op: FunctorOp,
    src: &BoundExplosionSlot,
    slot: &mut Option<Box<BoundExplosionSlot>>,
) -> bool {
    // IDA 0x4a4810 (`mpl::bool_<false>` = heap functor): 0 clone (`new(0x10)` copy,
    // 0x4a488e-0x4a48c0), 1 move (copy + zero the source, 0x4a48c6-0x4a48cc), 2 destroy
    // (release + `operator delete`, out = 0, 0x4a48d0-0x4a48ee), 3 get (`strcmp` the
    // bind_t typeinfo name: hit copies, miss writes 0, 0x4a490a-0x4a4914), default
    // answers the typeinfo (0x4a486e-0x4a4870). The model is monomorphic, so the
    // checked name always matches. Returns whether a live functor was stored.
    match op {
        FunctorOp::CloneFunctor | FunctorOp::MoveFunctor | FunctorOp::CheckFunctorType => {
            *slot = Some(Box::new(src.clone()));
            true
        }
        FunctorOp::DestroyFunctor => {
            *slot = None;
            false
        }
        FunctorOp::GetFunctorTypeInfo => true,
    }
}

// 0x4a5834 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::EnumPropDescriptor<RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType)>(char const*,char const*,RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x4a5834(
    name: &str,
    category: &str,
    get: Box<dyn Fn(&ExplosionState) -> i32 + Send + Sync>,
    set: Box<dyn Fn(&mut ExplosionState, i32) + Send + Sync>,
    mut attributes: u32,
    permissions: u32,
) -> ExplosionEnumPropDesc {
    // IDA 0x4a5834: `Singleton<EnumDesc<ExplosionType>>` via `call_once` + `doGetSingleton`
    // (0x4a5878-0x4a587c), base `PropertyDescriptor` init (0x4a58c6), enum-desc links at
    // +40/+48 (0x4a58ea/0x4a5954), `new(0x14)` member desc at +44 holding
    // (getter, setter) (0x4a5912-0x4a5938). Then `if (isReadOnly() == 1) attrs &= ~0x14`
    // (0x4a5964-0x4a596e) and `if (isWriteOnly() == 1) attrs &= ~0x0C` (0x4a5980-0x4a598a);
    // both query the GetSetImpl member desc, which hardcodes 0 (see stub_0x4a606c and
    // stub_0x4a6070), so the masks never fire.
    if stub_0x4a606c() {
        attributes &= !0x14;
    }
    ExplosionEnumPropDesc {
        name: name.to_owned(),
        category: category.to_owned(),
        access: ExplosionTypeAccess { get, set },
        enum_desc: explosion_type_enum_desc(),
        attributes,
        permissions,
    }
}

// 0x4a59e8 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::~EnumPropDescriptor()")]
pub fn stub_0x4a59e8() {
    // IDA 0x4a59e8: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x4a5a14 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::isReadOnly(void)const")]
pub fn stub_0x4a5a14() {
    // IDA 0x4a5a14: EnumPropDescriptor::isReadOnly/isWriteOnly -- forwards through the bound member descriptor at +44 (`(**(this+44))(this+44)`, decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x4a5a24 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::isWriteOnly(void)const")]
pub fn stub_0x4a5a24() {
    // IDA 0x4a5a24: EnumPropDescriptor::isReadOnly/isWriteOnly -- forwards through the bound member descriptor at +44 (`(**(this+44))(this+44)`, decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x4a5a34 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4a5a34(desc: &ExplosionEnumPropDesc, a: &ExplosionState, b: &ExplosionState) -> bool {
    // IDA 0x4a5a34: `v = member(+44)->get(a)` then `return v == member->get(b)`
    // (both through vf+8, 0x4a5a44-0x4a5a5a).
    (desc.access.get)(a) == (desc.access.get)(b)
}

// 0x4a5a5c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x4a5a5c(desc: &ExplosionEnumPropDesc, obj: &ExplosionState) -> Variant {
    // IDA 0x4a5a5c: `v = getEnumValue(obj)` (vf+68, 0x4a5a6a); out = `Variant(int, v)`
    // (0x4a5a70-0x4a5a7e).
    Variant::Int((desc.access.get)(obj))
}

// 0x4a5a80 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x4a5a80(desc: &ExplosionEnumPropDesc, obj: &mut ExplosionState, value: &Variant) {
    // IDA 0x4a5a80: int-typed payloads use `any_cast<int>` directly (0x4a5b4c); anything
    // else goes through `Variant::convert<int>` (0x4a5b00-0x4a5b3c); then
    // `setEnumValue(obj, v)` (vf+72, 0x4a5b5c).
    let v = match value {
        Variant::Int(v) => *v,
        other => other.convert_to_int(),
    };
    (desc.access.set)(obj, v);
}

// 0x4a5bcc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x4a5bcc(desc: &ExplosionEnumPropDesc, src: &ExplosionState, dst: &mut ExplosionState) {
    // IDA 0x4a5bcc: `v = member(+44)->get(src)` (vf+8, 0x4a5bde), then
    // `member->set(dst, v)` (vf+12, 0x4a5bee).
    let v = (desc.access.get)(src);
    (desc.access.set)(dst, v);
}

// 0x4a5bf4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::hasStringValue(void)const")]
pub fn stub_0x4a5bf4() -> bool {
    // IDA 0x4a5bf4: EnumPropDescriptor::hasStringValue -- hardcoded `return 1` (decompiled 0x10244/0x10dc8/0x11650).
    true
}

// 0x4a5bf8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4a5bf8(desc: &ExplosionEnumPropDesc, obj: &ExplosionState) -> String {
    // IDA 0x4a5bf8: `v = member(+44)->get(obj)` (0x4a5c0a), then
    // `EnumDesc<ExplosionType>::convertToString(enumdesc@+48, v)` (0x4a5c1a).
    let v = (desc.access.get)(obj);
    desc.enum_desc.lookup_name(v).unwrap_or_default().to_owned()
}

/// `G3D::Vector3` as stored on `Explosion` (12 bytes; IDA 0x4a640a copies `*v4` + `*(v4+8)`).
pub type Vector3 = [f32; 3];

/// Get/set pair behind `BoundProp<Vector3, Explosion>` (IDA 0x4a60bc ctor stores the member
/// offset; `BoundPropGetSet` dispatches through it at 0x4a63fc/0x4a6418).
pub struct ExplosionVector3Access {
    pub get: Box<dyn Fn(&ExplosionState) -> Vector3 + Send + Sync>,
    pub set: Box<dyn Fn(&mut ExplosionState, Vector3) + Send + Sync>,
}

/// Get/set pair behind the float Explosion props (IDA 0x4a64ac/0x4a66dc ctors).
pub struct ExplosionFloatAccess {
    pub get: Box<dyn Fn(&ExplosionState) -> f32 + Send + Sync>,
    pub set: Box<dyn Fn(&mut ExplosionState, f32) + Send + Sync>,
}

/// `RBX::Reflection::BoundProp<Vector3, Mutability::Mutable>` bound to `Explosion`
/// (IDA 0x4a60bc): base `TypedPropertyDescriptor<Vector3>` init plus the member offset.
pub struct BoundVector3Prop {
    pub name: String,
    pub category: String,
    pub access: ExplosionVector3Access,
    pub attributes: u32,
    pub permissions: u32,
}

/// `RBX::Reflection::BoundProp<float, Mutability::Mutable>` bound to `Explosion` (IDA 0x4a64ac).
pub struct BoundFloatProp {
    pub name: String,
    pub category: String,
    pub access: ExplosionFloatAccess,
    pub attributes: u32,
    pub permissions: u32,
}

/// `RBX::Reflection::PropDescriptor<Explosion, float>` with a getter/setter member-pointer
/// pair (IDA 0x4a66dc); `GetSetImpl` dispatches through it at 0x4a6824/0x4a6844.
pub struct ExplosionFloatPropDesc {
    pub name: String,
    pub category: String,
    pub access: ExplosionFloatAccess,
    pub attributes: u32,
    pub permissions: u32,
}

// 0x4a5c1c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_0x4a5c1c(desc: &ExplosionEnumPropDesc, obj: &mut ExplosionState, name: &str) -> bool {
    // IDA 0x4a5c1c: `Name::lookup(&name, str)` (0x4a5c2e), `convertToValue(enumdesc@+48, name, &out)`
    // (0x4a5c3c); on 1, `member(+44)->set(obj, out)` (0x4a5c52) and return 1, else 0. `&str`
    // folds the lookup step; `lookup_value` covers `convertToValue` including legacy names.
    match desc.enum_desc.lookup_value(name) {
        Some(v) => {
            (desc.access.set)(obj, v);
            true
        }
        None => false,
    }
}

// 0x4a5c5c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_0x4a5c5c(desc: &ExplosionEnumPropDesc, obj: &ExplosionState) -> i32 {
    // IDA 0x4a5c5c: `v = member(+44)->get(obj)` (vf+8, 0x4a5c6a), `clearValue(pair)` then store
    // int tag 5 + value (0x4a5c70-0x4a5c78), return 5. The tag is the Xml int type code; the
    // payload is the enum int, which is what the model returns.
    (desc.access.get)(obj)
}

// 0x4a5c7c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x4a5c7c(desc: &ExplosionEnumPropDesc, obj: &mut ExplosionState, text: &str) -> bool {
    // IDA 0x4a5c7c: extract the element text into a string, `Name::lookup`, `convertToValue`
    // (0x4a5d3c-0x4a5d4e); on success `member(+44)->set(obj, v)` (0x4a5d62). Empty/missing text
    // takes early-out paths that leave the object untouched. `&str` is the extracted text;
    // unknown names leave `obj` untouched and report false.
    match desc.enum_desc.lookup_value(text) {
        Some(v) => {
            (desc.access.set)(obj, v);
            true
        }
        None => false,
    }
}

// 0x4a5ebc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4a5ebc(desc: &ExplosionEnumPropDesc, obj: &ExplosionState) -> i32 {
    // IDA 0x4a5ebc: `v = member(+44)->get(obj)` (vf+8, 0x4a5ecc), return
    // `convertToIndex(enumdesc@+48, v)`. Same conversion as stub_0x4a5fb8.
    stub_0x4a5fb8(desc.enum_desc, (desc.access.get)(obj))
}

// 0x4a5ed8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_0x4a5ed8(desc: &ExplosionEnumPropDesc, obj: &mut ExplosionState, index: usize) -> bool {
    // IDA 0x4a5ed8: `if (*(enumdesc+40) > index)` (0x4a5eea) load `values[index]` (0x4a5ef4),
    // `member(+44)->set(obj, v)` (0x4a5efe), return 1; else return 0.
    match desc.enum_desc.values.get(index) {
        Some(&v) => {
            (desc.access.set)(obj, v);
            true
        }
        None => false,
    }
}

// 0x4a5f0c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4a5f0c(desc: &ExplosionEnumPropDesc, obj: &ExplosionState) -> i32 {
    // IDA 0x4a5f0c: tail-jump to `member(+44)->get(obj)` (vf+8); the whole body is the forward.
    (desc.access.get)(obj)
}

// 0x4a5f14 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x4a5f14(desc: &ExplosionEnumPropDesc, obj: &mut ExplosionState, value: i32) -> bool {
    // IDA 0x4a5f14: `find_if(items, bind(equalValue, _1, value))` (0x4a5f3e); miss returns 0
    // (0x4a5f44), hit runs `member(+44)->set(obj, value)` (0x4a5f52) and returns 1.
    if desc.enum_desc.items.iter().any(|it| it.value == value) {
        (desc.access.set)(obj, value);
        true
    } else {
        false
    }
}

// 0x4a5f60 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4a5f60(desc: &ExplosionEnumPropDesc, obj: &ExplosionState) -> Option<crate::enum_desc::EnumItem> {
    // IDA 0x4a5f60: `v = member(+44)->get(obj)` (0x4a5f72), return
    // `convertToItem(enumdesc@+48, &v)` (0x4a5f7e): the `Item*` for the value, or null.
    let v = (desc.access.get)(obj);
    usize::try_from(v)
        .ok()
        .and_then(|slot| desc.enum_desc.items_by_value.get(slot).copied().flatten())
        .and_then(|idx| desc.enum_desc.items.get(idx).cloned())
}

// 0x4a5f80 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_0x4a5f80(desc: &ExplosionEnumPropDesc, obj: &mut ExplosionState, name: &str) -> bool {
    // IDA 0x4a5f80 (`Name` overload): `convertToValue(enumdesc@+48, name, &out)` (0x4a5f96);
    // success runs `member(+44)->set(obj, out)` (0x4a5fac) and returns 1, else 0. Same shape as
    // stub_0x4a5c1c with the `Name::lookup` step already done by the caller.
    match desc.enum_desc.lookup_value(name) {
        Some(v) => {
            (desc.access.set)(obj, v);
            true
        }
        None => false,
    }
}

// 0x4a5fb8 — __ZNK3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::convertToIndex(RBX::Explosion::ExplosionType)const")]
pub fn stub_0x4a5fb8(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0x4a5fb8: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0x4a6028 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x4a6028(desc: &ExplosionEnumPropDesc, obj: &mut ExplosionState, value: i32) -> bool {
    // IDA 0x4a6028: `if (value >= 0)` (0x4a6032) and `value < value_to_value.size` (0x4a6044)
    // load `mapped = value_to_value[value]` (0x4a6046); `mapped == -1` returns 0 (0x4a6050),
    // else `member(+44)->set(obj, mapped)` (0x4a605c) and return 1.
    match usize::try_from(value)
        .ok()
        .and_then(|slot| desc.enum_desc.value_to_value.get(slot).copied())
    {
        Some(mapped) if mapped != -1 => {
            (desc.access.set)(obj, mapped);
            true
        }
        _ => false,
    }
}

// 0x4a606c — __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::GetSetImpl<RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType)>::isReadOnly(void)const")]
pub fn stub_0x4a606c() -> bool {
    // IDA 0x4a606c: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0x4a6070 — __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::GetSetImpl<RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType)>::isWriteOnly(void)const")]
pub fn stub_0x4a6070() -> bool {
    // IDA 0x4a6070: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0x4a6074 — __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::GetSetImpl<RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4a6074(access: &ExplosionTypeAccess, obj: &ExplosionState) -> i32 {
    // IDA 0x4a6074: null→`obj-36` member adjust (0x4a6078-0x4a607a), split the member pointer
    // (offset at +8, encoding at +4), virtual-adjust if the low bit is set (0x4a608a-0x4a608e),
    // call the getter. The adjust/encoding is member-pointer mechanics with no Rust equivalent;
    // the observable effect is the get.
    (access.get)(obj)
}

// 0x4a6094 — __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::GetSetImpl<RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType)>::setValue(RBX::Reflection::DescribedBase *,RBX::Explosion::ExplosionType const&)const")]
pub fn stub_0x4a6094(access: &ExplosionTypeAccess, obj: &mut ExplosionState, value: i32) {
    // IDA 0x4a6094: same member-pointer dispatch as stub_0x4a6074 through the setter at +12/+16
    // (0x4a60a0-0x4a60b0); the observable effect is the set.
    (access.set)(obj, value);
}

// 0x4a60b8 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9Explosion13ExplosionTypeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType> const>::initSingleton(void)")]
pub fn stub_0x4a60b8() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0x4a60b8: Singleton<EnumDesc<T>>::initSingleton -- thunk to doGetSingleton (decompiled 0x4a60b8). Rust: forward to the singleton.
    crate::generated::stub_0x4b6a3c()
}

// 0x4a60bc — __ZN3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EEC2INS_9ExplosionEEEPKcS9_MT_S3_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Explosion>(char const*,char const*,G3D::Vector3 RBX::Explosion::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x4a60bc(
    name: &str,
    category: &str,
    get: Box<dyn Fn(&ExplosionState) -> Vector3 + Send + Sync>,
    set: Box<dyn Fn(&mut ExplosionState, Vector3) + Send + Sync>,
    attributes: u32,
    permissions: u32,
) -> BoundVector3Prop {
    // IDA 0x4a60bc: base `TypedPropertyDescriptor<Vector3>` init, vtable installs, member-offset
    // store plus name/category/attribute wiring (same ctor shape as the float twin at 0x4a64ac).
    BoundVector3Prop {
        name: name.to_owned(),
        category: category.to_owned(),
        access: ExplosionVector3Access { get, set },
        attributes,
        permissions,
    }
}

// 0x4a6250 — __ZN3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EED0Ev
#[doc(alias = "RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::~BoundProp()")]
pub fn stub_0x4a6250() {
    // IDA 0x4a6250: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x4a6280 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector3EE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector3>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x4a6280(prop: &BoundVector3Prop, obj: &ExplosionState) -> Vector3 {
    // IDA 0x4a6280: `v = member(+40)->get(obj)` into a 12-byte temp (vf+8, 0x4a6294), tag the out
    // `Variant` with `Type::getSingleton<Vector3>` (0x4a629a), placement-move the temp in
    // (0x4a62a8). The model returns the payload; the tag is implied by the return type.
    stub_0x4a63fc(&prop.access, obj)
}

// 0x4a62b0 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector3EE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector3>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x4a62b0(prop: &BoundVector3Prop, src: &ExplosionState, dst: &mut ExplosionState) {
    // IDA 0x4a62b0: `member(+40)->get(src)` into a temp (0x4a62c6), then
    // `member(+40)->set(dst, temp)` (0x4a62d6).
    let v = stub_0x4a63fc(&prop.access, src);
    stub_0x4a6418(&prop.access, dst, &v);
}

// 0x4a63c8 — __ZN3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector3EED0Ev
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector3>::~TypedPropertyDescriptor()")]
pub fn stub_0x4a63c8() {
    // IDA 0x4a63c8: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x4a63f4 — __ZNK3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::isReadOnly(void)const")]
pub fn stub_0x4a63f4() -> bool {
    // IDA 0x4a63f4: BoundPropGetSet::isReadOnly -- hardcoded `return 0` (decompiled 0x659d38/0x659d3c SurfaceGetSet, 0x6ba528/0x6ba52c BoundPropGetSet).
    false
}

// 0x4a63f8 — __ZNK3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::isWriteOnly(void)const")]
pub fn stub_0x4a63f8() -> bool {
    // IDA 0x4a63f8: BoundPropGetSet::isWriteOnly -- hardcoded `return 0` (decompiled 0x659d38/0x659d3c SurfaceGetSet, 0x6ba528/0x6ba52c BoundPropGetSet).
    false
}

// 0x4a63fc — __ZNK3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4a63fc(access: &ExplosionVector3Access, obj: &ExplosionState) -> Vector3 {
    // IDA 0x4a63fc: null→`obj-36` adjust (0x4a6400-0x4a6402), add the member offset at +8
    // (0x4a6408), copy 12 bytes out (0x4a640a-0x4a6412). The adjust/offset is member-pointer
    // mechanics; the observable effect is the field load.
    (access.get)(obj)
}

// 0x4a6418 — __ZNK3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector3 const&)const")]
pub fn stub_0x4a6418(access: &ExplosionVector3Access, obj: &mut ExplosionState, value: &Vector3) {
    // IDA 0x4a6418: member adjust + offset (0x4a6420-0x4a642e), component-wise compare with
    // early-out when all three match (0x4a643c-0x4a646a), else store (0x4a646e-0x4a6478) and,
    // when the notify bits at +12/+16 are set, `raisePropertyChanged` (0x4a647a-0x4a6498). The
    // signal lives on `Instance` (datamodel side); the model keeps the compare-and-store.
    if (access.get)(obj) != *value {
        (access.set)(obj, *value);
    }
}

// 0x4a64ac — __ZN3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EEC2INS_9ExplosionEEEPKcS7_MT_fNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Explosion>(char const*,char const*,float RBX::Explosion::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x4a64ac(
    name: &str,
    category: &str,
    get: Box<dyn Fn(&ExplosionState) -> f32 + Send + Sync>,
    set: Box<dyn Fn(&mut ExplosionState, f32) + Send + Sync>,
    attributes: u32,
    permissions: u32,
) -> BoundFloatProp {
    // IDA 0x4a64ac: base `TypedPropertyDescriptor<float>` init, vtable installs, member-offset
    // store plus name/category/attribute wiring (same ctor shape as the Vector3 twin at 0x4a60bc).
    BoundFloatProp {
        name: name.to_owned(),
        category: category.to_owned(),
        access: ExplosionFloatAccess { get, set },
        attributes,
        permissions,
    }
}

// 0x4a6640 — __ZN3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EED0Ev
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::~BoundProp()")]
pub fn stub_0x4a6640() {
    // IDA 0x4a6640: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x4a666c — __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::isReadOnly(void)const")]
pub fn stub_0x4a666c() -> bool {
    // IDA 0x4a666c: BoundPropGetSet::isReadOnly -- hardcoded `return 0` (decompiled 0x659d38/0x659d3c SurfaceGetSet, 0x6ba528/0x6ba52c BoundPropGetSet).
    false
}

// 0x4a6670 — __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::isWriteOnly(void)const")]
pub fn stub_0x4a6670() -> bool {
    // IDA 0x4a6670: BoundPropGetSet::isWriteOnly -- hardcoded `return 0` (decompiled 0x659d38/0x659d3c SurfaceGetSet, 0x6ba528/0x6ba52c BoundPropGetSet).
    false
}

// 0x4a6674 — __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4a6674(access: &ExplosionFloatAccess, obj: &ExplosionState) -> f32 {
    // IDA 0x4a6674: single load `*(member_offset(a1+8) + obj - 36)` (0x4a667c) — a direct
    // data-member binding with no virtual adjust.
    (access.get)(obj)
}

// 0x4a6680 — __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE8setValueEPNS0_13DescribedBaseERKf
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
pub fn stub_0x4a6680(access: &ExplosionFloatAccess, obj: &mut ExplosionState, value: f32) {
    // IDA 0x4a6680: member adjust + offset (0x4a6684-0x4a6696), early-out when equal (0x4a66a4),
    // else store (0x4a66a8) and `raisePropertyChanged` when the notify bits are set
    // (0x4a66ac-0x4a66d8). Same compare-and-store shape as stub_0x4a6418.
    if (access.get)(obj) != value {
        (access.set)(obj, value);
    }
}

// 0x4a66dc — __ZN3RBX10Reflection14PropDescriptorINS_9ExplosionEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,float>::PropDescriptor<float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float)>(char const*,char const*,float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x4a66dc(
    name: &str,
    category: &str,
    get: Box<dyn Fn(&ExplosionState) -> f32 + Send + Sync>,
    set: Box<dyn Fn(&mut ExplosionState, f32) + Send + Sync>,
    attributes: u32,
    permissions: u32,
) -> ExplosionFloatPropDesc {
    // IDA 0x4a66dc: base `PropertyDescriptor` init, vtable installs, getter/setter member-pointer
    // pair stored into the `GetSetImpl` (same shape as the enum twin at 0x4a5834, whose
    // `new(0x14)` member desc holding (getter, setter) is described there).
    ExplosionFloatPropDesc {
        name: name.to_owned(),
        category: category.to_owned(),
        access: ExplosionFloatAccess { get, set },
        attributes,
        permissions,
    }
}

// 0x4a67f0 — __ZN3RBX10Reflection14PropDescriptorINS_9ExplosionEfED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,float>::~PropDescriptor()")]
pub fn stub_0x4a67f0() {
    // IDA 0x4a67f0: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x4a681c — __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,float>::GetSetImpl<float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float)>::isReadOnly(void)const")]
pub fn stub_0x4a681c() -> bool {
    // IDA 0x4a681c: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0x4a6820 — __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,float>::GetSetImpl<float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float)>::isWriteOnly(void)const")]
pub fn stub_0x4a6820() -> bool {
    // IDA 0x4a6820: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0x4a6824 — __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,float>::GetSetImpl<float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4a6824(access: &ExplosionFloatAccess, obj: &ExplosionState) -> f32 {
    // IDA 0x4a6824: same member-pointer dispatch as stub_0x4a6074 through the float getter at
    // +4/+8 (0x4a682e-0x4a683e); the observable effect is the get.
    (access.get)(obj)
}

// 0x4a6844 — __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,float>::GetSetImpl<float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
pub fn stub_0x4a6844(access: &ExplosionFloatAccess, obj: &mut ExplosionState, value: f32) {
    // IDA 0x4a6844: same member-pointer dispatch as stub_0x4a6094 through the float setter at
    // +12/+16 (0x4a6850-0x4a6860); the observable effect is the set.
    (access.set)(obj, value);
}

// 0x4a7734 — __ZN3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::~EnumPropDescriptor()")]
pub fn stub_0x4a7734() {
    // IDA 0x4a7734: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x4a7f5c — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_20ExtrudedPartInstanceES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance>(rbx_core::SharedPtr<RBX::ExtrudedPartInstance> const*,RBX::ExtrudedPartInstance *)const")]
pub fn stub_0x4a7f5c() {
    // IDA 0x4a7f5c: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

/// Minimal `RBX::ExtrudedPartInstance` state visible to its enum descriptor (IDA 0x4a88f0).
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ExtrudedPartState {
    pub visual_truss_style: i32,
}

/// Get/set pair behind `EnumPropDescriptor<ExtrudedPartInstance, VisualTrussStyle>`.
pub struct TrussStyleAccess {
    pub get: Box<dyn Fn(&ExtrudedPartState) -> i32 + Send + Sync>,
    pub set: Box<dyn Fn(&mut ExtrudedPartState, i32) + Send + Sync>,
}

/// `RBX::Reflection::EnumPropDescriptor<ExtrudedPartInstance, VisualTrussStyle>` (IDA 0x4a88f0).
pub struct TrussStylePropDesc {
    pub name: String,
    pub category: String,
    pub access: TrussStyleAccess,
    /// Singleton link stored at +40/+48 (same layout as the Explosion twin at 0x4a5834).
    pub enum_desc: &'static crate::enum_desc::EnumDesc,
    pub attributes: u32,
    pub permissions: u32,
}

/// `Singleton<EnumDesc<VisualTrussStyle>>::doGetSingleton`: pairs grounded in disasm 0x49b7f4
/// (`MOVS R1, #0` + "AlternatingSupports"), 0x49b80a (`#1` + "BridgeStyleSupports"), 0x49b820
/// (`#2` + "NoSupports"); legacy display names mapped at 0x49b838-0x49b89c.
static TRUSS_STYLE_DESC: std::sync::LazyLock<crate::enum_desc::EnumDesc> =
    std::sync::LazyLock::new(|| {
        let mut d = crate::enum_desc::EnumDesc::new("Style");
        d.add_pair(0, "AlternatingSupports");
        d.add_pair(1, "BridgeStyleSupports");
        d.add_pair(2, "NoSupports");
        d.add_legacy(0, "Alternating Supports", 0);
        d.add_legacy(1, "Bridge Style Supports", 1);
        d.add_legacy(2, "No Supports", 2);
        d
    });

pub fn truss_style_enum_desc() -> &'static crate::enum_desc::EnumDesc {
    &TRUSS_STYLE_DESC
}

/// Minimal `RBX::FaceInstance` state visible to its enum descriptor (IDA 0x4a9de0).
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct FaceInstanceState {
    pub normal_id: i32,
}

/// Get/set pair behind `EnumPropDescriptor<FaceInstance, NormalId>`.
pub struct FaceNormalAccess {
    pub get: Box<dyn Fn(&FaceInstanceState) -> i32 + Send + Sync>,
    pub set: Box<dyn Fn(&mut FaceInstanceState, i32) + Send + Sync>,
}

/// `RBX::Reflection::EnumPropDescriptor<FaceInstance, NormalId>` (IDA 0x4a9de0).
pub struct FaceNormalPropDesc {
    pub name: String,
    pub category: String,
    pub access: FaceNormalAccess,
    /// Singleton link stored at +40/+48 (same layout as the Explosion twin at 0x4a5834).
    pub enum_desc: &'static crate::enum_desc::EnumDesc,
    pub attributes: u32,
    pub permissions: u32,
}

/// `Singleton<EnumDesc<NormalId>>::doGetSingleton`: pairs grounded in disasm 0x6f2a52
/// (`MOVS R1, #1` + "Top"), 0x6f2a68 (`#4` + "Bottom"), 0x6f2a7e (`#2` + "Back"), 0x6f2a94
/// (`#5` + "Front"), 0x6f2aaa (`#0` + "Right"), 0x6f2ac0 (`#3` + "Left").
static NORMAL_ID_DESC: std::sync::LazyLock<crate::enum_desc::EnumDesc> =
    std::sync::LazyLock::new(|| {
        let mut d = crate::enum_desc::EnumDesc::new("NormalId");
        d.add_pair(1, "Top");
        d.add_pair(4, "Bottom");
        d.add_pair(2, "Back");
        d.add_pair(5, "Front");
        d.add_pair(0, "Right");
        d.add_pair(3, "Left");
        d
    });

pub fn normal_id_enum_desc() -> &'static crate::enum_desc::EnumDesc {
    &NORMAL_ID_DESC
}

// 0x4a88f0 — __ZN3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::EnumPropDescriptor<RBX::ExtrudedPartInstance::VisualTrussStyle (RBX::ExtrudedPartInstance::*)(void)const,void (RBX::ExtrudedPartInstance::*)(RBX::ExtrudedPartInstance::VisualTrussStyle)>(char const*,char const*,RBX::ExtrudedPartInstance::VisualTrussStyle (RBX::ExtrudedPartInstance::*)(void)const,void (RBX::ExtrudedPartInstance::*)(RBX::ExtrudedPartInstance::VisualTrussStyle),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x4a88f0(
    name: &str,
    category: &str,
    get: Box<dyn Fn(&ExtrudedPartState) -> i32 + Send + Sync>,
    set: Box<dyn Fn(&mut ExtrudedPartState, i32) + Send + Sync>,
    mut attributes: u32,
    permissions: u32,
) -> TrussStylePropDesc {
    // IDA 0x4a88f0: same ctor shape as the Explosion twin at 0x4a5834 — singleton link at
    // +40/+48, `new(0x14)` member desc at +44 holding (getter, setter), then the
    // read-only/write-only attribute masks, which query the GetSetImpl member desc that
    // hardcodes 0 (see stub_0x4a911c/stub_0x4a9120), so the masks never fire.
    if stub_0x4a911c() {
        attributes &= !0x14;
    }
    TrussStylePropDesc {
        name: name.to_owned(),
        category: category.to_owned(),
        access: TrussStyleAccess { get, set },
        enum_desc: truss_style_enum_desc(),
        attributes,
        permissions,
    }
}

// 0x4a8aa4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::~EnumPropDescriptor()")]
pub fn stub_0x4a8aa4() {
    // IDA 0x4a8aa4: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x4a8ad0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::isReadOnly(void)const")]
pub fn stub_0x4a8ad0() {
    // IDA 0x4a8ad0: EnumPropDescriptor::isReadOnly/isWriteOnly -- forwards through the bound member descriptor at +44 (`(**(this+44))(this+44)`, decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x4a8ae0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::isWriteOnly(void)const")]
pub fn stub_0x4a8ae0() {
    // IDA 0x4a8ae0: EnumPropDescriptor::isReadOnly/isWriteOnly -- forwards through the bound member descriptor at +44 (`(**(this+44))(this+44)`, decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x4a8af0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4a8af0(desc: &TrussStylePropDesc, a: &ExtrudedPartState, b: &ExtrudedPartState) -> bool {
    // IDA 0x4a8af0: `v = member(+44)->get(a)` (vf+8, 0x4a8b00), `return v == member->get(b)`
    // (0x4a8b16). Same shape as the Explosion twin at 0x4a5a34.
    (desc.access.get)(a) == (desc.access.get)(b)
}

// 0x4a8b18 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x4a8b18(desc: &TrussStylePropDesc, obj: &ExtrudedPartState) -> Variant {
    // IDA 0x4a8b18: `v = getEnumValue(obj)` (vf+68, 0x4a8b26); out = `Variant(int, v)` via the
    // int singleton + placement copy (0x4a8b2c-0x4a8b3a). Same shape as 0x4a5a5c.
    Variant::Int((desc.access.get)(obj))
}

// 0x4a8b3c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x4a8b3c(desc: &TrussStylePropDesc, obj: &mut ExtrudedPartState, value: &Variant) {
    // IDA 0x4a8b3c: same shape as the Explosion twin at 0x4a5a80 — int payloads via
    // `any_cast<int>`, anything else through `Variant::convert<int>`, then setEnumValue.
    let v = match value {
        Variant::Int(v) => *v,
        other => other.convert_to_int(),
    };
    (desc.access.set)(obj, v);
}

// 0x4a8c88 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x4a8c88(desc: &TrussStylePropDesc, src: &ExtrudedPartState, dst: &mut ExtrudedPartState) {
    // IDA 0x4a8c88: `v = member(+44)->get(src)`, then `member->set(dst, v)`. Same as 0x4a5bcc.
    let v = (desc.access.get)(src);
    (desc.access.set)(dst, v);
}

// 0x4a8cac — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::hasStringValue(void)const")]
pub fn stub_0x4a8cac() -> bool {
    // IDA 0x4a8cac: EnumPropDescriptor::hasStringValue -- hardcoded `return 1` (decompiled 0x10244/0x10dc8/0x11650).
    true
}

// 0x4a8cb0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4a8cb0(desc: &TrussStylePropDesc, obj: &ExtrudedPartState) -> String {
    // IDA 0x4a8cb0: `v = member(+44)->get(obj)`, then `EnumDesc<VisualTrussStyle>::convertToString`.
    // Same shape as the Explosion twin at 0x4a5bf8.
    desc.enum_desc.lookup_name((desc.access.get)(obj)).unwrap_or_default().to_owned()
}

// 0x4a8cd4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_0x4a8cd4(desc: &TrussStylePropDesc, obj: &mut ExtrudedPartState, name: &str) -> bool {
    // IDA 0x4a8cd4: `Name::lookup`, `convertToValue(enumdesc@+48, ...)`; on 1 set + return 1,
    // else 0. Same shape as the Explosion twin at 0x4a5c1c.
    match desc.enum_desc.lookup_value(name) {
        Some(v) => {
            (desc.access.set)(obj, v);
            true
        }
        None => false,
    }
}

// 0x4a8d14 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_0x4a8d14(desc: &TrussStylePropDesc, obj: &ExtrudedPartState) -> i32 {
    // IDA 0x4a8d14: `v = member(+44)->get(obj)` (vf+8, 0x4a8d22), `clearValue(pair)`, store int
    // tag 5 + value (0x4a8d28-0x4a8d30), return 5. Same shape as 0x4a5c5c.
    (desc.access.get)(obj)
}

// 0x4a8d34 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x4a8d34(desc: &TrussStylePropDesc, obj: &mut ExtrudedPartState, text: &str) -> bool {
    // IDA 0x4a8d34: element-text extract, `Name::lookup`, `convertToValue`; on success set.
    // Same shape as the Explosion twin at 0x4a5c7c.
    match desc.enum_desc.lookup_value(text) {
        Some(v) => {
            (desc.access.set)(obj, v);
            true
        }
        None => false,
    }
}

// 0x4a8f74 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4a8f74(desc: &TrussStylePropDesc, obj: &ExtrudedPartState) -> i32 {
    // IDA 0x4a8f74: `v = member(+44)->get(obj)`, return `convertToIndex(enumdesc@+48, v)`.
    // Same shape as the Explosion twin at 0x4a5ebc.
    stub_0x4a906c(desc.enum_desc, (desc.access.get)(obj))
}

// 0x4a8f90 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_0x4a8f90(desc: &TrussStylePropDesc, obj: &mut ExtrudedPartState, index: usize) -> bool {
    // IDA 0x4a8f90: bounds-check against the enum count, load `values[index]`, set, return 1;
    // else 0. Same shape as the Explosion twin at 0x4a5ed8.
    match desc.enum_desc.values.get(index) {
        Some(&v) => {
            (desc.access.set)(obj, v);
            true
        }
        None => false,
    }
}

// 0x4a8fc4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4a8fc4(desc: &TrussStylePropDesc, obj: &ExtrudedPartState) -> i32 {
    // IDA 0x4a8fc4: tail-jump to `member(+44)->get(obj)` (vf+8). Same as 0x4a5f0c.
    (desc.access.get)(obj)
}

// 0x4a8fcc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x4a8fcc(desc: &TrussStylePropDesc, obj: &mut ExtrudedPartState, value: i32) -> bool {
    // IDA 0x4a8fcc: `find_if(items, bind(equalValue, _1, value))`; hit sets + returns 1, miss 0.
    // Same shape as the Explosion twin at 0x4a5f14.
    if desc.enum_desc.items.iter().any(|it| it.value == value) {
        (desc.access.set)(obj, value);
        true
    } else {
        false
    }
}

// 0x4a9018 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4a9018(desc: &TrussStylePropDesc, obj: &ExtrudedPartState) -> Option<crate::enum_desc::EnumItem> {
    // IDA 0x4a9018: `v = member(+44)->get(obj)`, return `convertToItem(enumdesc@+48, &v)`.
    // Same shape as the Explosion twin at 0x4a5f60.
    let v = (desc.access.get)(obj);
    usize::try_from(v)
        .ok()
        .and_then(|slot| desc.enum_desc.items_by_value.get(slot).copied().flatten())
        .and_then(|idx| desc.enum_desc.items.get(idx).cloned())
}

// 0x4a9038 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_0x4a9038(desc: &TrussStylePropDesc, obj: &mut ExtrudedPartState, name: &str) -> bool {
    // IDA 0x4a9038 (`Name` overload): `convertToValue` then conditional set. Same as 0x4a5f80.
    match desc.enum_desc.lookup_value(name) {
        Some(v) => {
            (desc.access.set)(obj, v);
            true
        }
        None => false,
    }
}

// 0x4a906c — __ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::convertToIndex(RBX::ExtrudedPartInstance::VisualTrussStyle)const")]
pub fn stub_0x4a906c(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0x4a906c: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0x4a90dc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x4a90dc(desc: &TrussStylePropDesc, obj: &mut ExtrudedPartState, value: i32) -> bool {
    // IDA 0x4a90dc: `value >= 0` + bounds check, `mapped = value_to_value[value]`; `-1`
    // returns 0, else set + return 1. Same shape as the Explosion twin at 0x4a6028.
    match usize::try_from(value)
        .ok()
        .and_then(|slot| desc.enum_desc.value_to_value.get(slot).copied())
    {
        Some(mapped) if mapped != -1 => {
            (desc.access.set)(obj, mapped);
            true
        }
        _ => false,
    }
}

// 0x4a911c — __ZNK3RBX10Reflection14PropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::GetSetImpl<RBX::ExtrudedPartInstance::VisualTrussStyle (RBX::ExtrudedPartInstance::*)(void)const,void (RBX::ExtrudedPartInstance::*)(RBX::ExtrudedPartInstance::VisualTrussStyle)>::isReadOnly(void)const")]
pub fn stub_0x4a911c() -> bool {
    // IDA 0x4a911c: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0x4a9120 — __ZNK3RBX10Reflection14PropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::GetSetImpl<RBX::ExtrudedPartInstance::VisualTrussStyle (RBX::ExtrudedPartInstance::*)(void)const,void (RBX::ExtrudedPartInstance::*)(RBX::ExtrudedPartInstance::VisualTrussStyle)>::isWriteOnly(void)const")]
pub fn stub_0x4a9120() -> bool {
    // IDA 0x4a9120: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0x4a9124 — __ZNK3RBX10Reflection14PropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::GetSetImpl<RBX::ExtrudedPartInstance::VisualTrussStyle (RBX::ExtrudedPartInstance::*)(void)const,void (RBX::ExtrudedPartInstance::*)(RBX::ExtrudedPartInstance::VisualTrussStyle)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4a9124(access: &TrussStyleAccess, obj: &ExtrudedPartState) -> i32 {
    // IDA 0x4a9124: null→`obj-36` member adjust + member-pointer dispatch through the getter.
    // Same shape as the Explosion twin at 0x4a6074.
    (access.get)(obj)
}

// 0x4a9144 — __ZNK3RBX10Reflection14PropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::GetSetImpl<RBX::ExtrudedPartInstance::VisualTrussStyle (RBX::ExtrudedPartInstance::*)(void)const,void (RBX::ExtrudedPartInstance::*)(RBX::ExtrudedPartInstance::VisualTrussStyle)>::setValue(RBX::Reflection::DescribedBase *,RBX::ExtrudedPartInstance::VisualTrussStyle const&)const")]
pub fn stub_0x4a9144(access: &TrussStyleAccess, obj: &mut ExtrudedPartState, value: i32) {
    // IDA 0x4a9144: same member-pointer dispatch as stub_0x4a9124 through the setter.
    // Same shape as the Explosion twin at 0x4a6094.
    (access.set)(obj, value);
}

// 0x4a9728 — __ZN3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::~EnumPropDescriptor()")]
pub fn stub_0x4a9728() {
    // IDA 0x4a9728: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x4a9de0 — __ZN3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::EnumPropDescriptor<RBX::NormalId (RBX::FaceInstance::*)(void)const,void (RBX::FaceInstance::*)(RBX::NormalId)>(char const*,char const*,RBX::NormalId (RBX::FaceInstance::*)(void)const,void (RBX::FaceInstance::*)(RBX::NormalId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x4a9de0(
    name: &str,
    category: &str,
    get: Box<dyn Fn(&FaceInstanceState) -> i32 + Send + Sync>,
    set: Box<dyn Fn(&mut FaceInstanceState, i32) + Send + Sync>,
    attributes: u32,
    permissions: u32,
) -> FaceNormalPropDesc {
    // IDA 0x4a9de0: same EnumPropDescriptor ctor shape as 0x4a5834/0x4a88f0 — singleton link
    // at +40/+48, `new(0x14)` member desc at +44 holding (getter, setter); the
    // read-only/write-only attribute masks query a GetSetImpl that hardcodes 0, so they
    // never fire and the model keeps `attributes` as passed.
    FaceNormalPropDesc {
        name: name.to_owned(),
        category: category.to_owned(),
        access: FaceNormalAccess { get, set },
        enum_desc: normal_id_enum_desc(),
        attributes,
        permissions,
    }
}

// 0x4a9f94 — __ZN3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::~EnumPropDescriptor()")]
pub fn stub_0x4a9f94() {
    // IDA 0x4a9f94: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x4a9fc0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::isReadOnly(void)const")]
pub fn stub_0x4a9fc0() {
    // IDA 0x4a9fc0: EnumPropDescriptor::isReadOnly/isWriteOnly -- forwards through the bound member descriptor at +44 (`(**(this+44))(this+44)`, decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x4a9fd0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::isWriteOnly(void)const")]
pub fn stub_0x4a9fd0() {
    // IDA 0x4a9fd0: EnumPropDescriptor::isReadOnly/isWriteOnly -- forwards through the bound member descriptor at +44 (`(**(this+44))(this+44)`, decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x4a9fe0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4a9fe0(desc: &FaceNormalPropDesc, a: &FaceInstanceState, b: &FaceInstanceState) -> bool {
    // IDA 0x4a9fe0: `v = member(+44)->get(a)`, `return v == member->get(b)`. Same shape as
    // the Explosion twin at 0x4a5a34 and the truss twin at 0x4a8af0.
    (desc.access.get)(a) == (desc.access.get)(b)
}

// 0x4aa008 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x4aa008() -> ! {
    todo!("0x4aa008 __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")
}

// 0x4aa02c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x4aa02c() -> ! {
    todo!("0x4aa02c __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")
}

// 0x4aa178 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x4aa178() -> ! {
    todo!("0x4aa178 __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE9copyValueEPKNS0_13DescribedBaseEPS5_")
}

// 0x4aa19c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::hasStringValue(void)const")]
pub fn stub_0x4aa19c() -> bool {
    // IDA 0x4aa19c: EnumPropDescriptor::hasStringValue -- hardcoded `return 1` (decompiled 0x10244/0x10dc8/0x11650).
    true
}

// 0x4aa1a0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4aa1a0() -> ! {
    todo!("0x4aa1a0 __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE14getStringValueEPKNS0_13DescribedBaseE")
}

// 0x4aa1c4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_0x4aa1c4() -> ! {
    todo!("0x4aa1c4 __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE14setStringValueEPNS0_13DescribedBaseERKSs")
}

// 0x4aa204 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_0x4aa204() -> ! {
    todo!("0x4aa204 __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")
}

// 0x4aa224 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x4aa224() -> ! {
    todo!("0x4aa224 __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")
}

// 0x4aa464 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4aa464() -> ! {
    todo!("0x4aa464 __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE13getIndexValueEPKNS0_13DescribedBaseE")
}

// 0x4aa480 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_0x4aa480() -> ! {
    todo!("0x4aa480 __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE13setIndexValueEPNS0_13DescribedBaseEm")
}

// 0x4aa4b4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4aa4b4() -> ! {
    todo!("0x4aa4b4 __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE12getEnumValueEPKNS0_13DescribedBaseE")
}

// 0x4aa4bc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x4aa4bc() -> ! {
    todo!("0x4aa4bc __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE12setEnumValueEPNS0_13DescribedBaseEi")
}

// 0x4aa508 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4aa508() -> ! {
    todo!("0x4aa508 __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE11getEnumItemEPKNS0_13DescribedBaseE")
}

// 0x4aa528 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_0x4aa528() -> ! {
    todo!("0x4aa528 __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")
}

// 0x4aa55c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x4aa55c() -> ! {
    todo!("0x4aa55c __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE11setIntValueEPNS0_13DescribedBaseEi")
}

// 0x4aa59c — __ZNK3RBX10Reflection14PropDescriptorINS_12FaceInstanceENS_8NormalIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FaceInstance,RBX::NormalId>::GetSetImpl<RBX::NormalId (RBX::FaceInstance::*)(void)const,void (RBX::FaceInstance::*)(RBX::NormalId)>::isReadOnly(void)const")]
pub fn stub_0x4aa59c() -> bool {
    // IDA 0x4aa59c: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0x4aa5a0 — __ZNK3RBX10Reflection14PropDescriptorINS_12FaceInstanceENS_8NormalIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FaceInstance,RBX::NormalId>::GetSetImpl<RBX::NormalId (RBX::FaceInstance::*)(void)const,void (RBX::FaceInstance::*)(RBX::NormalId)>::isWriteOnly(void)const")]
pub fn stub_0x4aa5a0() -> bool {
    // IDA 0x4aa5a0: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0x4aa5a4 — __ZNK3RBX10Reflection14PropDescriptorINS_12FaceInstanceENS_8NormalIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FaceInstance,RBX::NormalId>::GetSetImpl<RBX::NormalId (RBX::FaceInstance::*)(void)const,void (RBX::FaceInstance::*)(RBX::NormalId)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4aa5a4() -> ! {
    todo!("0x4aa5a4 __ZNK3RBX10Reflection14PropDescriptorINS_12FaceInstanceENS_8NormalIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE")
}

// 0x4aa5c4 — __ZNK3RBX10Reflection14PropDescriptorINS_12FaceInstanceENS_8NormalIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FaceInstance,RBX::NormalId>::GetSetImpl<RBX::NormalId (RBX::FaceInstance::*)(void)const,void (RBX::FaceInstance::*)(RBX::NormalId)>::setValue(RBX::Reflection::DescribedBase *,RBX::NormalId const&)const")]
pub fn stub_0x4aa5c4() -> ! {
    todo!("0x4aa5c4 __ZNK3RBX10Reflection14PropDescriptorINS_12FaceInstanceENS_8NormalIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_")
}

// 0x4aab84 — __ZN3RBX10Reflection4Type12getSingletonINS_13TaskScheduler14PriorityMethodEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::TaskScheduler::PriorityMethod>(void)")]
pub fn stub_0x4aab84() -> ! {
    todo!("0x4aab84 __ZN3RBX10Reflection4Type12getSingletonINS_13TaskScheduler14PriorityMethodEEERKS1_v")
}

// 0x4aabb8 — __ZN3RBX10Reflection4Type12getSingletonINS_13TaskScheduler3Job17SleepAdjustMethodEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::TaskScheduler::Job::SleepAdjustMethod>(void)")]
pub fn stub_0x4aabb8() -> ! {
    todo!("0x4aabb8 __ZN3RBX10Reflection4Type12getSingletonINS_13TaskScheduler3Job17SleepAdjustMethodEEERKS1_v")
}

// 0x4aabec — __ZN3RBX10Reflection4Type12getSingletonINS_13TaskScheduler16ThreadPoolConfigEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::TaskScheduler::ThreadPoolConfig>(void)")]
pub fn stub_0x4aabec() -> ! {
    todo!("0x4aabec __ZN3RBX10Reflection4Type12getSingletonINS_13TaskScheduler16ThreadPoolConfigEEERKS1_v")
}

// 0x4aac20 — __ZN3RBX10Reflection4Type12getSingletonINS_10Controller6ButtonEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Controller::Button>(void)")]
pub fn stub_0x4aac20() -> ! {
    todo!("0x4aac20 __ZN3RBX10Reflection4Type12getSingletonINS_10Controller6ButtonEEERKS1_v")
}

// 0x4aac54 — __ZN3RBX10Reflection4Type12getSingletonINS_9GuiObject16TweenEasingStyleEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::GuiObject::TweenEasingStyle>(void)")]
pub fn stub_0x4aac54() -> ! {
    todo!("0x4aac54 __ZN3RBX10Reflection4Type12getSingletonINS_9GuiObject16TweenEasingStyleEEERKS1_v")
}

// 0x4aac88 — __ZN3RBX10Reflection4Type12getSingletonINS_9GuiObject11TweenStatusEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::GuiObject::TweenStatus>(void)")]
pub fn stub_0x4aac88() -> ! {
    todo!("0x4aac88 __ZN3RBX10Reflection4Type12getSingletonINS_9GuiObject11TweenStatusEEERKS1_v")
}

// 0x4aacbc — __ZN3RBX10Reflection4Type12getSingletonINS_9GuiObject20TweenEasingDirectionEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::GuiObject::TweenEasingDirection>(void)")]
pub fn stub_0x4aacbc() -> ! {
    todo!("0x4aacbc __ZN3RBX10Reflection4Type12getSingletonINS_9GuiObject20TweenEasingDirectionEEERKS1_v")
}

// 0x4aacf0 — __ZN3RBX10Reflection4Type12getSingletonINS_11TextService10XAlignmentEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::TextService::XAlignment>(void)")]
pub fn stub_0x4aacf0() -> ! {
    todo!("0x4aacf0 __ZN3RBX10Reflection4Type12getSingletonINS_11TextService10XAlignmentEEERKS1_v")
}

// 0x4aad24 — __ZN3RBX10Reflection4Type12getSingletonINS_11TextService10YAlignmentEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::TextService::YAlignment>(void)")]
pub fn stub_0x4aad24() -> ! {
    todo!("0x4aad24 __ZN3RBX10Reflection4Type12getSingletonINS_11TextService10YAlignmentEEERKS1_v")
}

// 0x4aad58 — __ZN3RBX10Reflection4Type12getSingletonINS_11TextService8FontSizeEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::TextService::FontSize>(void)")]
pub fn stub_0x4aad58() -> ! {
    todo!("0x4aad58 __ZN3RBX10Reflection4Type12getSingletonINS_11TextService8FontSizeEEERKS1_v")
}

// 0x4aad8c — __ZN3RBX10Reflection4Type12getSingletonINS_11TextService4FontEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::TextService::Font>(void)")]
pub fn stub_0x4aad8c() -> ! {
    todo!("0x4aad8c __ZN3RBX10Reflection4Type12getSingletonINS_11TextService4FontEEERKS1_v")
}

// 0x4aadc0 — __ZN3RBX10Reflection4Type12getSingletonINS_6Camera10CameraTypeEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Camera::CameraType>(void)")]
pub fn stub_0x4aadc0() -> ! {
    todo!("0x4aadc0 __ZN3RBX10Reflection4Type12getSingletonINS_6Camera10CameraTypeEEERKS1_v")
}

// 0x4aadf4 — __ZN3RBX10Reflection4Type12getSingletonINS_6Camera10CameraModeEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Camera::CameraMode>(void)")]
pub fn stub_0x4aadf4() -> ! {
    todo!("0x4aadf4 __ZN3RBX10Reflection4Type12getSingletonINS_6Camera10CameraModeEEERKS1_v")
}

// 0x4aae28 — __ZN3RBX10Reflection4Type12getSingletonINS_6Camera13CameraPanModeEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Camera::CameraPanMode>(void)")]
pub fn stub_0x4aae28() -> ! {
    todo!("0x4aae28 __ZN3RBX10Reflection4Type12getSingletonINS_6Camera13CameraPanModeEEERKS1_v")
}

// 0x4aae5c — __ZN3RBX10Reflection4Type12getSingletonINS_16LegacyController9InputTypeEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::LegacyController::InputType>(void)")]
pub fn stub_0x4aae5c() -> ! {
    todo!("0x4aae5c __ZN3RBX10Reflection4Type12getSingletonINS_16LegacyController9InputTypeEEERKS1_v")
}

// 0x4aae90 — __ZN3RBX10Reflection4Type12getSingletonINS_16DataModelArbiter16ConcurrencyModelEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::DataModelArbiter::ConcurrencyModel>(void)")]
pub fn stub_0x4aae90() -> ! {
    todo!("0x4aae90 __ZN3RBX10Reflection4Type12getSingletonINS_16DataModelArbiter16ConcurrencyModelEEERKS1_v")
}

// 0x4aaec4 — __ZN3RBX10Reflection4Type12getSingletonINS_13DebugSettings14ErrorReportingEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::DebugSettings::ErrorReporting>(void)")]
pub fn stub_0x4aaec4() -> ! {
    todo!("0x4aaec4 __ZN3RBX10Reflection4Type12getSingletonINS_13DebugSettings14ErrorReportingEEERKS1_v")
}

// 0x4aaef8 — __ZN3RBX10Reflection4Type12getSingletonINS_9EThrottle13EThrottleTypeEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::EThrottle::EThrottleType>(void)")]
pub fn stub_0x4aaef8() -> ! {
    todo!("0x4aaef8 __ZN3RBX10Reflection4Type12getSingletonINS_9EThrottle13EThrottleTypeEEERKS1_v")
}

// 0x4aaf2c — __ZN3RBX10Reflection4Type12getSingletonINS_8NormalIdEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::NormalId>(void)")]
pub fn stub_0x4aaf2c() -> ! {
    todo!("0x4aaf2c __ZN3RBX10Reflection4Type12getSingletonINS_8NormalIdEEERKS1_v")
}

// 0x4aaf60 — __ZN3RBX10Reflection4Type12getSingletonIN3G3D7Vector34AxisEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<G3D::Vector3::Axis>(void)")]
pub fn stub_0x4aaf60() -> ! {
    todo!("0x4aaf60 __ZN3RBX10Reflection4Type12getSingletonIN3G3D7Vector34AxisEEERKS1_v")
}

// 0x4aaf94 — __ZN3RBX10Reflection4Type12getSingletonINS_8Humanoid6StatusEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Humanoid::Status>(void)")]
pub fn stub_0x4aaf94() -> ! {
    todo!("0x4aaf94 __ZN3RBX10Reflection4Type12getSingletonINS_8Humanoid6StatusEEERKS1_v")
}

// 0x4aafc8 — __ZN3RBX10Reflection4Type12getSingletonINS_9DataModel11CreatorTypeEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::DataModel::CreatorType>(void)")]
pub fn stub_0x4aafc8() -> ! {
    todo!("0x4aafc8 __ZN3RBX10Reflection4Type12getSingletonINS_9DataModel11CreatorTypeEEERKS1_v")
}

// 0x4aaffc — __ZN3RBX10Reflection4Type12getSingletonINS_9DataModel5GenreEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::DataModel::Genre>(void)")]
pub fn stub_0x4aaffc() -> ! {
    todo!("0x4aaffc __ZN3RBX10Reflection4Type12getSingletonINS_9DataModel5GenreEEERKS1_v")
}

// 0x4ab030 — __ZN3RBX10Reflection4Type12getSingletonINS_9DataModel16GearGenreSettingEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::DataModel::GearGenreSetting>(void)")]
pub fn stub_0x4ab030() -> ! {
    todo!("0x4ab030 __ZN3RBX10Reflection4Type12getSingletonINS_9DataModel16GearGenreSettingEEERKS1_v")
}

// 0x4ab064 — __ZN3RBX10Reflection4Type12getSingletonINS_9DataModel8GearTypeEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::DataModel::GearType>(void)")]
pub fn stub_0x4ab064() -> ! {
    todo!("0x4ab064 __ZN3RBX10Reflection4Type12getSingletonINS_9DataModel8GearTypeEEERKS1_v")
}

// 0x4ab098 — __ZN3RBX10Reflection4Type12getSingletonINS_8Instance10SaveFilterEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Instance::SaveFilter>(void)")]
pub fn stub_0x4ab098() -> ! {
    todo!("0x4ab098 __ZN3RBX10Reflection4Type12getSingletonINS_8Instance10SaveFilterEEERKS1_v")
}

// 0x4ab0cc — __ZN3RBX10Reflection4Type12getSingletonINS_13FriendService12FriendStatusEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::FriendService::FriendStatus>(void)")]
pub fn stub_0x4ab0cc() -> ! {
    todo!("0x4ab0cc __ZN3RBX10Reflection4Type12getSingletonINS_13FriendService12FriendStatusEEERKS1_v")
}

// 0x4ab100 — __ZN3RBX10Reflection4Type12getSingletonINS_13FriendService15FriendEventTypeEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::FriendService::FriendEventType>(void)")]
pub fn stub_0x4ab100() -> ! {
    todo!("0x4ab100 __ZN3RBX10Reflection4Type12getSingletonINS_13FriendService15FriendEventTypeEEERKS1_v")
}

// 0x4ab134 — __ZN3RBX10Reflection4Type12getSingletonINS_18SkateboardPlatform9MoveStateEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::SkateboardPlatform::MoveState>(void)")]
pub fn stub_0x4ab134() -> ! {
    todo!("0x4ab134 __ZN3RBX10Reflection4Type12getSingletonINS_18SkateboardPlatform9MoveStateEEERKS1_v")
}

// 0x4ab168 — __ZN3RBX10Reflection4Type12getSingletonINS_9SoundTypeEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::SoundType>(void)")]
pub fn stub_0x4ab168() -> ! {
    todo!("0x4ab168 __ZN3RBX10Reflection4Type12getSingletonINS_9SoundTypeEEERKS1_v")
}

// 0x4ab19c — __ZN3RBX10Reflection4Type12getSingletonINS_11SurfaceTypeEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::SurfaceType>(void)")]
pub fn stub_0x4ab19c() -> ! {
    todo!("0x4ab19c __ZN3RBX10Reflection4Type12getSingletonINS_11SurfaceTypeEEERKS1_v")
}

// 0x4ab1d0 — __ZN3RBX10Reflection4Type12getSingletonINS_12PartInstance10FormFactorEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::PartInstance::FormFactor>(void)")]
pub fn stub_0x4ab1d0() -> ! {
    todo!("0x4ab1d0 __ZN3RBX10Reflection4Type12getSingletonINS_12PartInstance10FormFactorEEERKS1_v")
}

// 0x4ab204 — __ZN3RBX10Reflection4Type12getSingletonINS_16UserInputService14SwipeDirectionEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::UserInputService::SwipeDirection>(void)")]
pub fn stub_0x4ab204() -> ! {
    todo!("0x4ab204 __ZN3RBX10Reflection4Type12getSingletonINS_16UserInputService14SwipeDirectionEEERKS1_v")
}

// 0x4ab238 — __ZN3RBX10Reflection4Type12getSingletonINS_8MaterialEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Material>(void)")]
pub fn stub_0x4ab238() -> ! {
    todo!("0x4ab238 __ZN3RBX10Reflection4Type12getSingletonINS_8MaterialEEERKS1_v")
}

#[cfg(test)]
mod descriptor_batch_tests {
    use super::*;
    use std::sync::atomic::{AtomicI32, Ordering};

    fn test_desc() -> ExplosionEnumPropDesc {
        stub_0x4a5834(
            "ExplosionType",
            "Behavior",
            Box::new(|s: &ExplosionState| s.explosion_type),
            Box::new(|s: &mut ExplosionState, v: i32| s.explosion_type = v),
            0,
            0,
        )
    }

    #[test]
    fn explosion_type_singleton_items_match_ida() {
        // Disasm 0x49f6f6/0x49f70c/0x49f722: addPair(0, NoCraters), (1, Craters), (2, CratersAndDebris).
        let d = explosion_type_enum_desc();
        assert_eq!(d.lookup_value("NoCraters"), Some(0));
        assert_eq!(d.lookup_value("Craters"), Some(1));
        assert_eq!(d.lookup_value("CratersAndDebris"), Some(2));
        assert_eq!(d.lookup_name(2), Some("CratersAndDebris"));
    }

    #[test]
    fn enum_value_round_trip_through_bound_pair() {
        let desc = test_desc();
        let mut state = ExplosionState::default();
        stub_0x4a5a80(&desc, &mut state, &Variant::Int(1));
        assert_eq!(state.explosion_type, 1);
        assert!(matches!(stub_0x4a5a5c(&desc, &state), Variant::Int(1)));
        assert_eq!(stub_0x4a5bf8(&desc, &state), "Craters");
        stub_0x4a5a80(&desc, &mut state, &Variant::Float(2.0));
        assert_eq!(state.explosion_type, 2);
        let mut other = ExplosionState::default();
        stub_0x4a5bcc(&desc, &state, &mut other);
        assert!(stub_0x4a5a34(&desc, &state, &other));
        other.explosion_type = 0;
        assert!(!stub_0x4a5a34(&desc, &state, &other));
    }

    #[test]
    fn event_connect_fire_disconnect() {
        let source = EventSource::default();
        let seen = std::sync::Arc::new(AtomicI32::new(-1));
        let seen2 = std::sync::Arc::clone(&seen);
        let wrapper: SharedPtr<GenericSlotWrapper> = std::sync::Arc::new(GenericSlotWrapper {
            invoke: Box::new(move |args: &[Variant]| {
                if let Variant::Float(v) = args[1] {
                    seen2.store(v as i32, Ordering::SeqCst);
                }
            }),
        });
        let inst: SharedPtr<InstanceHandle> = std::sync::Arc::new(InstanceHandle { id: 7 });
        stub_0x4a3b5c(Some(&source), SharedPtr::clone(&wrapper));
        stub_0x4a3cb0(
            &source,
            &[Variant::Instance(inst), Variant::Float(3.0)],
        );
        assert_eq!(seen.load(Ordering::SeqCst), 3);
        stub_0x4a3e20(&source);
        stub_0x4a3cb0(
            &source,
            &[
                Variant::Instance(std::sync::Arc::new(InstanceHandle { id: 7 })),
                Variant::Float(9.0),
            ],
        );
        assert_eq!(seen.load(Ordering::SeqCst), 3);
        // Null source stores an empty connection: no-op, no panic.
        stub_0x4a3b5c(None, wrapper);
    }

    #[test]
    fn functor_manager_lifecycle() {
        let wrapper: SharedPtr<GenericSlotWrapper> = std::sync::Arc::new(GenericSlotWrapper {
            invoke: Box::new(|_| {}),
        });
        let bound = stub_0x4a3fac(wrapper);
        let mut slot: Option<Box<BoundExplosionSlot>> = None;
        assert!(stub_0x4a4810(FunctorOp::CloneFunctor, &bound, &mut slot));
        assert!(slot.is_some());
        assert!(stub_0x4a4810(FunctorOp::CheckFunctorType, &bound, &mut slot));
        assert!(!stub_0x4a4810(FunctorOp::DestroyFunctor, &bound, &mut slot));
        assert!(slot.is_none());
        assert!(stub_0x4a4810(FunctorOp::GetFunctorTypeInfo, &bound, &mut slot));
        assert!(slot.is_none());
        let mut slot2: Option<Box<BoundExplosionSlot>> = None;
        assert_eq!(
            stub_0x4a4524(FunctorOp::MoveFunctor, &bound, &mut slot2),
            EXPLOSION_BIND_T_TYPEINFO
        );
        assert!(slot2.is_some());
    }

    #[test]
    fn function_assign_and_invoke() {
        let wrapper: SharedPtr<GenericSlotWrapper> = std::sync::Arc::new(GenericSlotWrapper {
            invoke: Box::new(|_| {}),
        });
        let bound = stub_0x4a3fac(wrapper);
        let mut func = ExplosionSlotFunction::default();
        assert!(func.is_empty());
        stub_0x4a442c(&mut func, &bound);
        assert!(!func.is_empty());
        let inst: SharedPtr<InstanceHandle> = std::sync::Arc::new(InstanceHandle::default());
        stub_0x4a4540(&bound, &inst, 1.0);
        func.invoke(&inst, 1.0);
        assert!(stub_0x4a4554(&mut func, &bound));
    }

    #[test]
    fn event_desc_ctor_builds_two_item_signature() {
        let d = stub_0x4a38b8(0, "Exploded", "Behavior", "Exploded", 0, 0);
        assert_eq!(
            d.signature,
            vec![
                ("Behavior".to_owned(), ExplosionEventArg::Instance),
                ("Exploded".to_owned(), ExplosionEventArg::Float),
            ]
        );
    }
}
