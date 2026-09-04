//! audio generated_audio_wd_watchdog6 — 120 stubs EA-sorted asc gap filler not yet in audio (FMOD|Sound|Audio exhausted, global gap filler)
//! Source: ida/export.json (85545 funcs) EA-sorted asc next 120 not in audio after 0x062c19c | rbx_core::SharedPtr not boost
//! Range 0x062c208..0x0630bf4 | existing 35962 -> 36082 distinct
//! Batch: 120 stubs | // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR mangled")

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use parking_lot::Mutex;
use rbx_core::SharedPtr;
use std::collections::HashMap;
use std::sync::OnceLock;
const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };
// IDA 0x62c208..0x62eda8 host-seam model types.
/// Live slot of the 1-arg touched signal (IDA 0x62c208): the bound
/// (PlatformImpl*, shared Instance) pair behind the callable_slot rides in
/// `target`/`instance`; `live` is the +12 link word.
pub struct TouchedSlot {
    pub live: bool,
    pub target: u32,
    pub instance: u32,
}
/// RBX::ActionStation<BasicPartInstance> host state (IDA 0x62eda8): the
/// touch-regulator stamp at +336 (word 42, seconds).
pub struct ActionStationState {
    pub touch_stamp: f64,
}
/// Host side of `std::map<RBX::Name const*, MoveState>` (IDA 0x62f064):
/// Name keys are u32 ids, MoveState values ride as i32 (cf. the
/// `EnumDescState` index in generated_audio_wd_watchdog5). HashMap stands in
/// for the Rb_tree; insertion uniqueness matches (cf. `SoundMap` in
/// generated.rs).
pub type MoveStateMap = HashMap<u32, i32>;
/// Function-static mutex behind the touched signal's static lock
/// (see `stub_062c208`); distinct static per signal family, same convention
/// as `MOVE_STATE_SIGNAL_MUTEX` in generated_audio_wd_watchdog5.
static TOUCHED_SIGNAL_MUTEX: Mutex<()> = Mutex::new(());
/// Host side of `BoundFuncDesc<SkateboardPlatform, void(Vector3), 1>`
/// (IDA 0x62f4f8): name/doc/permissions/attributes plus the declared
/// argument name once `declareSignature` runs (IDA 0x62f5b4). The
/// member-function pair at +40 (IDA 0x62f574) folds into the caller's
/// dispatch closure.
pub struct SkateboardBoundFunc {
    pub name: String,
    pub doc: String,
    pub permissions: u32,
    pub attributes: u32,
    pub signature_arg: Option<String>,
}
/// Host side of `RefPropDescriptor<SkateboardPlatform, T>` (IDA 0x62f7b8 /
/// 0x63005c): name/category/attributes/permissions. The GetImpl box at +44
/// (IDA 0x62f82c-0x62f846) folds into the caller's getter/setter closures.
pub struct SkateboardRefProp {
    pub name: String,
    pub category: String,
    pub attributes: u32,
    pub permissions: u32,
}
/// Host side of `Reflection::Type<T*>` (IDA 0x62ffac): the declared tag
/// name (IDA 0x62ffec) registered in the all-types list (IDA 0x630048).
pub struct SkateboardTypeDesc {
    pub name: String,
}
/// Function-static `RefType<Humanoid *>` instance (see `stub_062f85c`).
/// Runtime input (the init closure) required — OnceLock, not LazyLock.
static HUMANOID_REF_TYPE: OnceLock<u32> = OnceLock::new();
/// Function-static `RefType<SkateboardController *>` instance (see
/// `stub_0630100`).
static CONTROLLER_REF_TYPE: OnceLock<u32> = OnceLock::new();

// 0x062c208 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_12PlatformImplINS4_17BasicPartInstanceEEES6_EENSA_5list2INSA_5valueIPSG_EENS2_3argILi1EEEEEEEEENS0_10connectionERKT_
// demangled: rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PlatformImpl<RBX::BasicPartInstance>,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PlatformImpl<RBX::BasicPartInstance>,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>,boost::arg<1>>> const&)
// type: int(void)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PlatformImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PlatformImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>,boost::arg<1>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_12PlatformImplINS4_17BasicPartInstanceEEES6_EENSA_5list2INSA_5valueIPSG_EENS2_3argILi1EEEEEEEEENS0_10connectionERKT_")]
pub fn stub_062c208(slots: &mut Vec<TouchedSlot>, target: u32, instance: u32) -> u32 {
    // IDA 0x62c208 (signal<void(shared)>::connect): operator new(28) the
    // callable_slot (0x62c220), install the slot/callable/bind vtables plus
    // the bound (PlatformImpl*, shared Instance) triple (0x62c238-0x62c25e),
    // signal::insert under the static lock (0x62c262), connection <= slot
    // (0x62c268) with add_weak_ref (0x62c270). Same shape as the void()
    // twin stub_062b798 in generated_audio_wd_watchdog5.
    // was: rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<...>(...).
    let _static = TOUCHED_SIGNAL_MUTEX.lock();
    slots.push(TouchedSlot { live: true, target, instance });
    (slots.len() - 1) as u32
}

// 0x062c27c — __ZN3RBX12PlatformImplINS_17BasicPartInstanceEE23onEvent_platformTouchedEN5boost10shared_ptrINS_8InstanceEEE
// demangled: RBX::PlatformImpl<RBX::BasicPartInstance>::onEvent_platformTouched(boost::shared_ptr<RBX::Instance>)
#[doc(alias = "RBX::PlatformImpl<RBX::BasicPartInstance>::onEvent_platformTouched(rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX12PlatformImplINS_17BasicPartInstanceEE23onEvent_platformTouchedEN5boost10shared_ptrINS_8InstanceEEE")]
#[allow(clippy::too_many_arguments)]
pub fn stub_062c27c(
    this: u32,
    touched: u32,
    humanoid_of: impl FnOnce(u32) -> u32,
    torso_of: impl FnOnce(u32) -> u32,
    seconds_since_touch: impl FnOnce() -> f64,
    find_motor: impl FnOnce() -> u32,
    standing_flag: impl FnOnce(u32) -> bool,
    is_dead: impl FnOnce(u32) -> bool,
    owned_elsewhere: impl FnOnce(u32) -> bool,
    in_workspace: impl Fn(u32) -> bool,
    same_assembly: bool,
    up_y: impl FnOnce() -> f32,
    create_motor: impl FnOnce(),
) -> bool {
    // IDA 0x62c27c (onEvent_platformTouched): h = humanoidFromBodyPart
    // (0x62c28a); !h -> out (0x62c28e). torso = getTorsoSlow (0x62c298);
    // !torso -> out (0x62c29c). now - stamp(+336) <= 3.0 -> out
    // (0x62c2a8-0x62c2b4). findPlatformMotor6D (0x62c2cc); found -> out.
    // standing flag (0x62c2d4), getDead (0x62c2de), network-owner-elsewhere
    // (0x62c2e8), both contextInWorkspace (0x62c2f2-0x62c2fc) gate; assert
    // torso/platform assemblies differ (Platform.h:36, 0x62c316-0x62c33c);
    // up-column y <= 0.7 -> out (0x62c36c-0x62c38a); createPlatformMotor6D
    // (0x62c390).
    let humanoid = humanoid_of(touched);
    if humanoid == 0 {
        return false;
    }
    let torso = torso_of(humanoid);
    if torso == 0 {
        return false;
    }
    if seconds_since_touch() <= 3.0 {
        return false;
    }
    if find_motor() != 0 {
        return false;
    }
    if standing_flag(humanoid) {
        return false;
    }
    if is_dead(humanoid) {
        return false;
    }
    if owned_elsewhere(torso) {
        return false;
    }
    if !in_workspace(torso) || !in_workspace(this) {
        return false;
    }
    debug_assert!(
        !same_assembly,
        "h->getTorsoSlow()->getPartPrimitive()->getAssembly() != this->getPartPrimitive()->getAssembly() Platform.h:36"
    );
    if up_y() <= 0.7 {
        return false;
    }
    create_motor();
    true
}

// 0x062c3a0 — __ZN3RBX12PlatformImplINS_17BasicPartInstanceEE21createPlatformMotor6DEPNS_8HumanoidE
// demangled: RBX::PlatformImpl<RBX::BasicPartInstance>::createPlatformMotor6D(RBX::Humanoid *)
// type: int(void)
#[doc(alias = "RBX::PlatformImpl<RBX::BasicPartInstance>::createPlatformMotor6D(RBX::Humanoid *)")]
#[doc(alias = "__ZN3RBX12PlatformImplINS_17BasicPartInstanceEE21createPlatformMotor6DEPNS_8HumanoidE")]
pub fn stub_062c3a0(
    this: u32,
    humanoid: u32,
    use_visible_torso: bool,
    torso_of: impl FnOnce(u32, bool) -> u32,
    frame_matches: bool,
    torso_assembly: u32,
    platform_assembly: u32,
    root_is_torso: bool,
    build_motor: impl FnOnce(u32, u32),
) {
    // IDA 0x62c3a0 (createPlatformMotor6D): torso = DFFlag
    // ? getVisibleTorsoSlow : getTorsoSlow (0x62c402-0x62c412); assert
    // pTorsoPart (Platform.h:75, 0x62c426-0x62c474); assert
    // getCoordinateFrame() == part-primitive frame (Platform.h:78,
    // 0x62c4b8-0x62c510); lookAt C0/C1 frames (0x62c54e-0x62c5cc); assert
    // torso/platform assemblies differ (Platform.h:89, 0x62c5e6-0x62c608);
    // root = getAssemblyPrimitive (0x62c652), assert root == torso unless
    // the flag moves the root (Platform.h:93, 0x62c656-0x62c6a8); zero the
    // assembly velocity (0x62c6ac-0x62c6c6); create<Motor6D>, name it
    // "PlatformMotor6D", setPart0/1 + C0/C1, parent under this
    // (0x62c6d0-0x62c77a, the build_motor seam).
    let torso = torso_of(humanoid, use_visible_torso);
    debug_assert!(torso != 0, "pTorsoPart Platform.h:75");
    debug_assert!(
        frame_matches,
        "this->getCoordinateFrame() == this->getPartPrimitive()->getCoordinateFrame() Platform.h:78"
    );
    debug_assert!(
        torso_assembly != platform_assembly,
        "torso->getAssembly() != this->getPartPrimitive()->getAssembly() Platform.h:89"
    );
    if !use_visible_torso {
        debug_assert!(root_is_torso, "root == torso Platform.h:93");
    }
    build_motor(this, torso);
}

// 0x062c87c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_12PlatformImplINS4_17BasicPartInstanceEEES6_EENSA_5list2INSA_5valueIPSG_EENS2_3argILi1EEEEEEEED1Ev
// demangled: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PlatformImpl<RBX::BasicPartInstance>,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>>::~callable_slot()
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PlatformImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_12PlatformImplINS4_17BasicPartInstanceEEES6_EENSA_5list2INSA_5valueIPSG_EENS2_3argILi1EEEEEEEED1Ev")]
pub fn stub_062c87c() {
    // IDA 0x062c87c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062c8a8 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_12PlatformImplINS4_17BasicPartInstanceEEES6_EENSA_5list2INSA_5valueIPSG_EENS2_3argILi1EEEEEEEED0Ev
// demangled: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PlatformImpl<RBX::BasicPartInstance>,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>>::~callable_slot()
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PlatformImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_12PlatformImplINS4_17BasicPartInstanceEEES6_EENSA_5list2INSA_5valueIPSG_EENS2_3argILi1EEEEEEEED0Ev")]
pub fn stub_062c8a8() {
    // IDA 0x062c8a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062c97c — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_12PlatformImplINS5_17BasicPartInstanceEEES7_EENSB_5list2INSB_5valueIPSH_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
// demangled: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PlatformImpl<RBX::BasicPartInstance>,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PlatformImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_12PlatformImplINS5_17BasicPartInstanceEEES7_EENSB_5list2INSB_5valueIPSH_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_")]
pub fn stub_062c97c<T>(obj: u32, instance: &SharedPtr<T>, invoke: impl FnOnce(u32, &SharedPtr<T>)) {
    // IDA 0x62c97c (callable::call): forwards the bound pair (slot+24,
    // slot+16) into list2::operator() (0x62c996) — delegates directly.
    // was: rbx::callable<...shared...>::call(rbx_core::SharedPtr<RBX::Instance>).
    stub_062c9b4(obj, instance, invoke);
}

// 0x062c998 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_12PlatformImplINS5_17BasicPartInstanceEEES7_EENSB_5list2INSB_5valueIPSH_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
// demangled: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PlatformImpl<RBX::BasicPartInstance>,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PlatformImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_12PlatformImplINS5_17BasicPartInstanceEEES7_EENSB_5list2INSB_5valueIPSH_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_")]
pub fn stub_062c998() {
    // IDA 0x062c998: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062c9b4 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX12PlatformImplINS3_17BasicPartInstanceEEEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS6_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRSH_EEEEvNS0_4typeIvEERT_RT0_i
// demangled: void boost::_bi::list2<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance> *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::PlatformImpl<RBX::BasicPartInstance>,boost::shared_ptr<RBX::Instance>>,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::PlatformImpl<RBX::BasicPartInstance>,boost::shared_ptr<RBX::Instance>> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&> &,int)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance> *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::PlatformImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::PlatformImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueIPN3RBX12PlatformImplINS3_17BasicPartInstanceEEEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS6_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRSH_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_062c9b4<T>(obj: u32, instance: &SharedPtr<T>, invoke: impl FnOnce(u32, &SharedPtr<T>)) {
    // IDA 0x62c9b4 (list2 operator()): shared_count copy of the instance
    // (0x62c9e0-0x62c9e6); mf1::operator()(obj, shared) (0x62ca22); release
    // (0x62ca28-0x62ca30). Clone-then-drop is the same retain/release pair;
    // same shape as the mf2 twin stub_062ba7c in generated_audio_wd_watchdog5.
    // was: boost::_bi::list2<...>::operator()<...>(...).
    let owned = SharedPtr::clone(instance);
    invoke(obj, &owned);
}

// 0x062ca8c — __ZNK5boost4_mfi3mf1IvN3RBX12PlatformImplINS2_17BasicPartInstanceEEENS_10shared_ptrINS2_8InstanceEEEEclEPS5_S8_
// demangled: boost::_mfi::mf1<void,RBX::PlatformImpl<RBX::BasicPartInstance>,boost::shared_ptr<RBX::Instance>>::operator()(RBX::PlatformImpl<RBX::BasicPartInstance>*,boost::shared_ptr<RBX::Instance>)const
#[doc(alias = "boost::_mfi::mf1<void,RBX::PlatformImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::PlatformImpl<RBX::BasicPartInstance>*,rbx_core::SharedPtr<RBX::Instance>)const")]
#[doc(alias = "__ZNK5boost4_mfi3mf1IvN3RBX12PlatformImplINS2_17BasicPartInstanceEEENS_10shared_ptrINS2_8InstanceEEEEclEPS5_S8_")]
pub fn stub_062ca8c<T>(
    target: u32,
    adjust: u32,
    instance: &SharedPtr<T>,
    direct: impl FnOnce(u32, &SharedPtr<T>),
    virtual_call: impl FnOnce(u32, &SharedPtr<T>),
) {
    // IDA 0x62ca8c (mf1 operator()): resolve this+adjust with the odd-adjust
    // vtable step (0x62cab8-0x62caea, cf. the mf0 twin 0x62b93c);
    // shared_count copy (0x62caf0-0x62cb02); invoke (0x62cb0c); release
    // (0x62cb10-0x62cb18).
    // was: boost::_mfi::mf1<...>::operator()(...).
    let owned = SharedPtr::clone(instance);
    if (adjust & 1) != 0 {
        virtual_call(target, &owned);
    } else {
        direct(target, &owned);
    }
}

// 0x062cb74 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_12PlatformImplINS5_17BasicPartInstanceEEES7_EENSB_5list2INSB_5valueIPSH_EENS3_3argILi1EEEEEEELi1ES8_ED1Ev
// demangled: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PlatformImpl<RBX::BasicPartInstance>,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PlatformImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_12PlatformImplINS5_17BasicPartInstanceEEES7_EENSB_5list2INSB_5valueIPSH_EENS3_3argILi1EEEEEEELi1ES8_ED1Ev")]
pub fn stub_062cb74() {
    // IDA 0x062cb74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062cba0 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_12PlatformImplINS5_17BasicPartInstanceEEES7_EENSB_5list2INSB_5valueIPSH_EENS3_3argILi1EEEEEEELi1ES8_ED0Ev
// demangled: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PlatformImpl<RBX::BasicPartInstance>,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PlatformImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_12PlatformImplINS5_17BasicPartInstanceEEES7_EENSB_5list2INSB_5valueIPSH_EENS3_3argILi1EEEEEEELi1ES8_ED0Ev")]
pub fn stub_062cba0() {
    // IDA 0x062cba0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062cc74 — __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ModelInstanceEEESB_ENS7_5list2INS7_5valueISB_EESG_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ModelInstanceEEESB_ENS7_5list2INS7_5valueISB_EESG_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_062cc74<T>(
    first: &std::sync::Weak<T>,
    second: &std::sync::Weak<T>,
    assign: impl FnOnce(std::sync::Weak<T>, std::sync::Weak<T>),
) {
    // IDA 0x62cc74 (function<void(DataModel*)> ctor from bind_t): weak
    // addref both bound weak_ptrs under the spinlock pool (0x62ccb2-0x62cd78)
    // then delegates to the function1 ctor (0x62cd86); temp weak_releases
    // (0x62cd8c-0x62cda0) ride the dropped clones.
    // was: boost::function<void ()(RBX::DataModel *)>::function<...>(...).
    stub_062ce60(first, second, assign);
}

// 0x062ce60 — __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ModelInstanceEEESA_ENS6_5list2INS6_5valueISA_EESF_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ModelInstanceEEESA_ENS6_5list2INS6_5valueISA_EESF_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE")]
pub fn stub_062ce60<T>(
    first: &std::sync::Weak<T>,
    second: &std::sync::Weak<T>,
    assign: impl FnOnce(std::sync::Weak<T>, std::sync::Weak<T>),
) {
    // IDA 0x62ce60 (function1 ctor from bind_t): null the vtable
    // (0x62ce80); weak addref both bound weak_ptrs under the spinlock pool
    // (0x62cea2-0x62cf6a); assign_to the bound pair (0x62cf78); temp
    // weak_releases (0x62cf7e-0x62cf92) ride the dropped clones.
    // was: boost::function1<void,RBX::DataModel *>::function1<...>(...).
    assign(std::sync::Weak::clone(first), std::sync::Weak::clone(second));
}

// 0x062d050 — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ModelInstanceEEESA_ENS6_5list2INS6_5valueISA_EESF_EEEEEEvT_
// demangled: void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ModelInstance>,boost::weak_ptr<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>,boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ModelInstance>,boost::weak_ptr<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>,boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>>>)
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::ModelInstance>,rbx_core::Weak<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>,boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::ModelInstance>,rbx_core::Weak<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>,boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>>>)")]
#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ModelInstanceEEESA_ENS6_5list2INS6_5valueISA_EESF_EEEEEEvT_")]
pub fn stub_062d050() {
    // IDA 0x062d050: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

// 0x062d250 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ModelInstanceEEES8_ENS3_5list2INS3_5valueIS8_EESD_EEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ModelInstance>,boost::weak_ptr<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>,boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::ModelInstance>,rbx_core::Weak<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>,boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ModelInstanceEEES8_ENS3_5list2INS3_5valueIS8_EESD_EEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE")]
pub fn stub_062d250() {
    // IDA 0x062d250: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

// 0x062d26c — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ModelInstanceEEES8_ENS3_5list2INS3_5valueIS8_EESD_EEEEvPNS6_9DataModelEE6invokeERNS1_15function_bufferESH_
// demangled: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ModelInstance>,boost::weak_ptr<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>,boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::ModelInstance>,rbx_core::Weak<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>,boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ModelInstanceEEES8_ENS3_5list2INS3_5valueIS8_EESD_EEEEvPNS6_9DataModelEE6invokeERNS1_15function_bufferESH_")]
pub fn stub_062d26c() {
    // IDA 0x062d26c: invoker::invoke unpacked the buffer and called the bound functor. Closure call at the live site — carrier no-op.
}

// 0x062d288 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ModelInstanceEEESC_ENS8_5list2INS8_5valueISC_EESH_EEEEEEbT_RNS1_15function_bufferE
// demangled: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ModelInstance>,boost::weak_ptr<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>,boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ModelInstance>,boost::weak_ptr<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>,boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>>>,boost::detail::function::function_buffer &)const
// type: int __fastcall(int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::ModelInstance>,rbx_core::Weak<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>,boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::ModelInstance>,rbx_core::Weak<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>,boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ModelInstanceEEESC_ENS8_5list2INS8_5valueISC_EESH_EEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_062d288() {
    // IDA 0x062d288: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

// 0x062d478 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ModelInstanceEEESC_ENS8_5list2INS8_5valueISC_EESH_EEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// demangled: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ModelInstance>,boost::weak_ptr<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>,boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ModelInstance>,boost::weak_ptr<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>,boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
// type: int __fastcall(int, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::ModelInstance>,rbx_core::Weak<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>,boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::ModelInstance>,rbx_core::Weak<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>,boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ModelInstanceEEESC_ENS8_5list2INS8_5valueISC_EESH_EEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_062d478() {
    // IDA 0x062d478: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

// 0x062d664 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ModelInstanceEEESC_ENS8_5list2INS8_5valueISC_EESH_EEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// demangled: void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ModelInstance>,boost::weak_ptr<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>,boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ModelInstance>,boost::weak_ptr<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>,boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, int, int)
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::ModelInstance>,rbx_core::Weak<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>,boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::ModelInstance>,rbx_core::Weak<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>,boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ModelInstanceEEESC_ENS8_5list2INS8_5valueISC_EESH_EEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_062d664() {
    // IDA 0x062d664: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

// 0x062d7f4 — __ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX13ModelInstanceEEEEES7_EclIPFvS6_S6_ENS0_5list1IRPNS4_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i
// demangled: void boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>,boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>>::operator()<void (*)(boost::weak_ptr<RBX::ModelInstance>,boost::weak_ptr<RBX::ModelInstance>),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::ModelInstance>,boost::weak_ptr<RBX::ModelInstance>) &,boost::_bi::list1<RBX::DataModel *&> &,int)
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>,boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>>::operator()<void (*)(rbx_core::Weak<RBX::ModelInstance>,rbx_core::Weak<RBX::ModelInstance>),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(rbx_core::Weak<RBX::ModelInstance>,rbx_core::Weak<RBX::ModelInstance>) &,boost::_bi::list1<RBX::DataModel *&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX13ModelInstanceEEEEES7_EclIPFvS6_S6_ENS0_5list1IRPNS4_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_062d7f4<T>(
    first: &std::sync::Weak<T>,
    second: &std::sync::Weak<T>,
    invoke: impl FnOnce(std::sync::Weak<T>, std::sync::Weak<T>),
) {
    // IDA 0x62d7f4 (list2 operator() for the free function): weak addref
    // the first bound weak (0x62d848-0x62d894) and the second (0x62d8a0-
    // 0x62d8ee) under the spinlock pool; invoke f(a, b) (0x62d8fa); weak
    // release both (0x62d8fe-0x62d912). Cloned Weaks dropped at scope end
    // are the same pair.
    // was: boost::_bi::list2<...weak...>::operator()<...>(...).
    let a = std::sync::Weak::clone(first);
    let b = std::sync::Weak::clone(second);
    invoke(a, b);
}

// 0x062d988 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ModelInstanceEEES8_ENS3_5list2INS3_5valueIS8_EESD_EEEEE7managerERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ModelInstance>,boost::weak_ptr<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>,boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::ModelInstance>,rbx_core::Weak<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>,boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ModelInstanceEEES8_ENS3_5list2INS3_5valueIS8_EESD_EEEEE7managerERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_062d988() {
    // IDA 0x062d988: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

// 0x062dbc4 — __ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX13ModelInstanceEEEEES7_EC2ES7_S7_
// demangled: boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>,boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>>::list2(boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>,boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>)
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>,boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>>::list2(boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>,boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX13ModelInstanceEEEEES7_EC2ES7_S7_")]
pub fn stub_062dbc4() {
    // IDA 0x062dbc4: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

// 0x062dd60 — __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX13ModelInstanceEEEEES7_EC2ES7_S7_
// demangled: boost::_bi::storage2<boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>,boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>>::storage2(boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>,boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>)
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>,boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>>::storage2(boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>,boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>)")]
#[doc(alias = "__ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX13ModelInstanceEEEEES7_EC2ES7_S7_")]
pub fn stub_062dd60<T>(
    first: &(std::sync::Weak<T>, std::sync::Weak<T>),
    second: &(std::sync::Weak<T>, std::sync::Weak<T>),
) -> (
    (std::sync::Weak<T>, std::sync::Weak<T>),
    (std::sync::Weak<T>, std::sync::Weak<T>),
) {
    // IDA 0x62dd60 (storage2 ctor): copy the first pair with the
    // bump/bump/release dance (0x62ddf0-0x62de3c, net single weak addref;
    // null takes the plain copy at 0x62de42-0x62de4c) and the second pair
    // with a single bump (0x62de6a-0x62deac). Clone pairs are the same.
    // was: boost::_bi::storage2<...weak...>::storage2(...).
    (
        (std::sync::Weak::clone(&first.0), std::sync::Weak::clone(&first.1)),
        (std::sync::Weak::clone(&second.0), std::sync::Weak::clone(&second.1)),
    )
}

// 0x062df24 — __ZN5boost8weak_ptrIN3RBX13ModelInstanceEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
// demangled: boost::weak_ptr<RBX::ModelInstance>::weak_ptr<RBX::ModelInstance>(boost::shared_ptr<RBX::ModelInstance> const&,boost::detail::sp_enable_if_convertible<RBX::ModelInstance,RBX::ModelInstance>::type)
// type: int(void)
#[doc(alias = "rbx_core::Weak<RBX::ModelInstance>::weak_ptr<RBX::ModelInstance>(rbx_core::SharedPtr<RBX::ModelInstance> const&,boost::detail::sp_enable_if_convertible<RBX::ModelInstance,RBX::ModelInstance>::type)")]
#[doc(alias = "__ZN5boost8weak_ptrIN3RBX13ModelInstanceEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE")]
pub fn stub_062df24() {
    // IDA 0x062df24: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

// 0x062df74 — __ZN5boost10shared_ptrIN3RBX13ModelInstanceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// demangled: boost::shared_ptr<RBX::ModelInstance>::shared_ptr<RBX::ModelInstance>(boost::weak_ptr<RBX::ModelInstance> const&,boost::detail::sp_nothrow_tag)
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::ModelInstance>::shared_ptr<RBX::ModelInstance>(rbx_core::Weak<RBX::ModelInstance> const&,boost::detail::sp_nothrow_tag)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX13ModelInstanceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")]
pub fn stub_062df74() {
    // IDA 0x062df74: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x062dff0 — __ZN5boost10shared_ptrIN3RBX20SkateboardControllerEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// demangled: boost::shared_ptr<RBX::SkateboardController>::shared_ptr<RBX::SkateboardController,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SkateboardController *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::SkateboardController>::shared_ptr<RBX::SkateboardController,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SkateboardController *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX20SkateboardControllerEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_062dff0<T>(
    boxed: Option<Box<T>>,
    accept_owner: impl FnOnce(&SharedPtr<T>),
) -> Option<SharedPtr<T>> {
    // IDA 0x62dff0: store the raw pointer (0x62e010), shared_count adopt
    // (0x62e018); non-null wires the weak owner via _internal_accept_owner
    // (0x62e046-0x62e056). Instruction-identical to the SkateboardPlatform
    // twin stub_062b0b8 in generated_audio_wd_watchdog5.
    // was: boost::shared_ptr<RBX::SkateboardController>::shared_ptr<...>(RBX::SkateboardController *,RBX::Creatable<RBX::Instance>::Deleter).
    let shared = boxed.map(|b| SharedPtr::new(*b));
    if let Some(s) = shared.as_ref() {
        accept_owner(s);
    }
    shared
}

// 0x062e0b8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_20SkateboardControllerES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::SkateboardController,RBX::SkateboardController>(boost::shared_ptr<RBX::SkateboardController> const*,RBX::SkateboardController *)const
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::SkateboardController,RBX::SkateboardController>(rbx_core::SharedPtr<RBX::SkateboardController> const*,RBX::SkateboardController *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_20SkateboardControllerES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_062e0b8() {
    // IDA 0x062e0b8: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0x062e1a0 — __ZN5boost6detail12shared_countC2IPN3RBX20SkateboardControllerENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// demangled: boost::detail::shared_count::shared_count<RBX::SkateboardController *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SkateboardController *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::SkateboardController *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SkateboardController *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX20SkateboardControllerENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_062e1a0() {
    // IDA 0x062e1a0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x062e2a8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SkateboardControllerENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::SkateboardController *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SkateboardController *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SkateboardControllerENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_062e2a8() {
    // IDA 0x062e2a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062e2ac — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SkateboardControllerENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::SkateboardController *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SkateboardController *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SkateboardControllerENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_062e2ac() {
    // IDA 0x062e2ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062e2b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SkateboardControllerENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::SkateboardController *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SkateboardController *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SkateboardControllerENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_062e2b0() {
    // IDA 0x062e2b0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x062e2d0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SkateboardControllerENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_pd<RBX::SkateboardController *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SkateboardController *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SkateboardControllerENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_062e2d0() {
    // IDA 0x062e2d0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x062e2e8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SkateboardControllerENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::SkateboardController *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SkateboardController *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SkateboardControllerENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_062e2e8() {
    // IDA 0x062e2e8: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x062e2ec — __ZThn32_N3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEED1Ev
#[doc(alias = "__ZThn32_N3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEED1Ev")]
pub fn stub_062e2ec() {
    // IDA 0x062e2ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062e300 — __ZThn36_N3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEED1Ev
#[doc(alias = "__ZThn36_N3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEED1Ev")]
pub fn stub_062e300() {
    // IDA 0x062e300: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062e314 — __ZThn32_N3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEED0Ev
#[doc(alias = "__ZThn32_N3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEED0Ev")]
pub fn stub_062e314() {
    // IDA 0x062e314: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062e31c — __ZThn36_N3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEED0Ev
// type: int __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEED0Ev")]
pub fn stub_062e31c() {
    // IDA 0x062e31c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062e324 — __ZN3RBX12PlatformImplINS_17BasicPartInstanceEED2Ev
// demangled: RBX::PlatformImpl<RBX::BasicPartInstance>::~PlatformImpl()
// type: int __fastcall(int, int, int, int, int, rbx::signals::connection *, int, int, int, int)
#[doc(alias = "RBX::PlatformImpl<RBX::BasicPartInstance>::~PlatformImpl()")]
#[doc(alias = "__ZN3RBX12PlatformImplINS_17BasicPartInstanceEED2Ev")]
pub fn stub_062e324() {
    // IDA 0x062e324: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062e514 — __ZThn32_N3RBX12PlatformImplINS_17BasicPartInstanceEED1Ev
// demangled: non-virtual thunk toRBX::PlatformImpl<RBX::BasicPartInstance>::~PlatformImpl()
#[doc(alias = "non-virtual thunk toRBX::PlatformImpl<RBX::BasicPartInstance>::~PlatformImpl()")]
#[doc(alias = "__ZThn32_N3RBX12PlatformImplINS_17BasicPartInstanceEED1Ev")]
pub fn stub_062e514() {
    // IDA 0x062e514: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062e528 — __ZThn32_N3RBX12PlatformImplINS_17BasicPartInstanceEED0Ev
// demangled: non-virtual thunk toRBX::PlatformImpl<RBX::BasicPartInstance>::~PlatformImpl()
#[doc(alias = "non-virtual thunk toRBX::PlatformImpl<RBX::BasicPartInstance>::~PlatformImpl()")]
#[doc(alias = "__ZThn32_N3RBX12PlatformImplINS_17BasicPartInstanceEED0Ev")]
pub fn stub_062e528() {
    // IDA 0x062e528: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062e5d8 — __ZThn36_N3RBX12PlatformImplINS_17BasicPartInstanceEED1Ev
// demangled: non-virtual thunk toRBX::PlatformImpl<RBX::BasicPartInstance>::~PlatformImpl()
#[doc(alias = "non-virtual thunk toRBX::PlatformImpl<RBX::BasicPartInstance>::~PlatformImpl()")]
#[doc(alias = "__ZThn36_N3RBX12PlatformImplINS_17BasicPartInstanceEED1Ev")]
pub fn stub_062e5d8() {
    // IDA 0x062e5d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062e5ec — __ZThn36_N3RBX12PlatformImplINS_17BasicPartInstanceEED0Ev
// demangled: non-virtual thunk toRBX::PlatformImpl<RBX::BasicPartInstance>::~PlatformImpl()
#[doc(alias = "non-virtual thunk toRBX::PlatformImpl<RBX::BasicPartInstance>::~PlatformImpl()")]
#[doc(alias = "__ZThn36_N3RBX12PlatformImplINS_17BasicPartInstanceEED0Ev")]
pub fn stub_062e5ec() {
    // IDA 0x062e5ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062e69c — __ZN3G3D5ArrayIN3RBX18SkateboardPlatform5WheelELi10ELm32EED2Ev
// demangled: G3D::Array<RBX::SkateboardPlatform::Wheel,10,32ul>::~Array()
#[doc(alias = "G3D::Array<RBX::SkateboardPlatform::Wheel,10,32ul>::~Array()")]
#[doc(alias = "__ZN3G3D5ArrayIN3RBX18SkateboardPlatform5WheelELi10ELm32EED2Ev")]
pub fn stub_062e69c() {
    // IDA 0x062e69c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062e770 — __ZN3G3D5ArrayIN3RBX18SkateboardPlatform5WheelELi10ELm32EEC2Ev
// demangled: G3D::Array<RBX::SkateboardPlatform::Wheel,10,32ul>::Array(void)
#[doc(alias = "G3D::Array<RBX::SkateboardPlatform::Wheel,10,32ul>::Array(void)")]
#[doc(alias = "__ZN3G3D5ArrayIN3RBX18SkateboardPlatform5WheelELi10ELm32EEC2Ev")]
pub fn stub_062e770<T>() -> Vec<T> {
    // IDA 0x62e770 (G3D::Array<Wheel> C2): length 0 (0x62e798), null data
    // via MemoryManager::create (0x62e798-0x62e7ce), zeroPointer
    // (0x62e7ee). An empty Vec is the same state.
    Vec::new()
}

// 0x062e860 — __ZN3RBX10Reflection9DescribedINS_18SkateboardPlatformELZNS_19sSkateboardPlatformEENS_14FactoryProductIS2_NS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EEC2Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_18SkateboardPlatformELZNS_19sSkateboardPlatformEENS_14FactoryProductIS2_NS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EEC2Ev")]
pub fn stub_062e860(construct_base: impl FnOnce(), describe: impl FnOnce() -> u32, register: impl FnOnce()) -> u32 {
    // IDA 0x62e860 (Described<SkateboardPlatform> C2): ActionStation base
    // ctor (0x62e88c); vtable installs across words 0/3/8/9/23/24/27/33/37
    // plus the cross-vtable copies (0x62e892-0x62e9e2, unlinked host model);
    // classDescriptor + describedClassDescriptor install (0x62ea08-0x62ea26);
    // ClassRegistrar++ (0x62ea2c).
    construct_base();
    let descriptor = describe();
    register();
    descriptor
}

// 0x062ea88 — __ZThn32_N3RBX18DescribedCreatableINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_062ea88() {
    // IDA 0x062ea88: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062ea9c — __ZThn32_N3RBX18DescribedCreatableINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_062ea9c() {
    // IDA 0x062ea9c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062eb50 — __ZThn36_N3RBX18DescribedCreatableINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_062eb50() {
    // IDA 0x062eb50: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062eb64 — __ZThn36_N3RBX18DescribedCreatableINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_062eb64() {
    // IDA 0x062eb64: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062ec18 — __ZThn32_N3RBX10Reflection9DescribedINS_18SkateboardPlatformELZNS_19sSkateboardPlatformEENS_14FactoryProductIS2_NS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_18SkateboardPlatformELZNS_19sSkateboardPlatformEENS_14FactoryProductIS2_NS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_062ec18() {
    // IDA 0x062ec18: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062ec2c — __ZThn32_N3RBX10Reflection9DescribedINS_18SkateboardPlatformELZNS_19sSkateboardPlatformEENS_14FactoryProductIS2_NS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_18SkateboardPlatformELZNS_19sSkateboardPlatformEENS_14FactoryProductIS2_NS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_062ec2c() {
    // IDA 0x062ec2c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062ece0 — __ZThn36_N3RBX10Reflection9DescribedINS_18SkateboardPlatformELZNS_19sSkateboardPlatformEENS_14FactoryProductIS2_NS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_18SkateboardPlatformELZNS_19sSkateboardPlatformEENS_14FactoryProductIS2_NS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_062ece0() {
    // IDA 0x062ece0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062eda8 — __ZN3RBX13ActionStationINS_17BasicPartInstanceEEC2Ev
// demangled: RBX::ActionStation<RBX::BasicPartInstance>::ActionStation(void)
// type: int __fastcall(int, int, int, int, int, int, int, int, RBX::BasicPartInstance *, int, int, int, int, int)
#[doc(alias = "RBX::ActionStation<RBX::BasicPartInstance>::ActionStation(void)")]
#[doc(alias = "__ZN3RBX13ActionStationINS_17BasicPartInstanceEEC2Ev")]
pub fn stub_062eda8(
    state: &mut ActionStationState,
    construct_base: impl FnOnce(),
    now: impl FnOnce() -> f64,
    part_prim: u32,
    set_size_mult: impl FnOnce(u32),
) {
    // IDA 0x62eda8 (ActionStation C2): BasicPartInstance base (0x62edd0);
    // vtable installs (0x62edf0-0x62ee44, unlinked host model); touch stamp
    // (+336) = now - 4.0 (0x62ee6e-0x62ee94); assert sleepTimeUp, i.e. the
    // stamp reads older than 3s (ActionStation.h:37, 0x62eea4-0x62ef0e —
    // the second now() is assert scaffolding); assert getPartPrimitive()
    // non-null (ActionStation.h:38, 0x62ef26-0x62ef5a);
    // setSizeMultiplier(prim, 3) (0x62ef5e).
    construct_base();
    let t = now();
    state.touch_stamp = t - 4.0;
    debug_assert!(
        t - state.touch_stamp > 3.0,
        "this->sleepTimeUp() ActionStation.h:37"
    );
    debug_assert!(
        part_prim != 0,
        "this->getPartPrimitive() ActionStation.h:38"
    );
    set_size_mult(part_prim);
}

// 0x062efd0 — __ZThn32_N3RBX13ActionStationINS_17BasicPartInstanceEED1Ev
// demangled: non-virtual thunk toRBX::ActionStation<RBX::BasicPartInstance>::~ActionStation()
#[doc(alias = "non-virtual thunk toRBX::ActionStation<RBX::BasicPartInstance>::~ActionStation()")]
#[doc(alias = "__ZThn32_N3RBX13ActionStationINS_17BasicPartInstanceEED1Ev")]
pub fn stub_062efd0() {
    // IDA 0x062efd0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062efe4 — __ZThn32_N3RBX13ActionStationINS_17BasicPartInstanceEED0Ev
// demangled: non-virtual thunk toRBX::ActionStation<RBX::BasicPartInstance>::~ActionStation()
#[doc(alias = "non-virtual thunk toRBX::ActionStation<RBX::BasicPartInstance>::~ActionStation()")]
#[doc(alias = "__ZThn32_N3RBX13ActionStationINS_17BasicPartInstanceEED0Ev")]
pub fn stub_062efe4() {
    // IDA 0x062efe4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062efec — __ZThn36_N3RBX13ActionStationINS_17BasicPartInstanceEED1Ev
// demangled: non-virtual thunk toRBX::ActionStation<RBX::BasicPartInstance>::~ActionStation()
// type: void __fastcall(int)
#[doc(alias = "non-virtual thunk toRBX::ActionStation<RBX::BasicPartInstance>::~ActionStation()")]
#[doc(alias = "__ZThn36_N3RBX13ActionStationINS_17BasicPartInstanceEED1Ev")]
pub fn stub_062efec() {
    // IDA 0x062efec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062f000 — __ZThn36_N3RBX13ActionStationINS_17BasicPartInstanceEED0Ev
// demangled: non-virtual thunk toRBX::ActionStation<RBX::BasicPartInstance>::~ActionStation()
#[doc(alias = "non-virtual thunk toRBX::ActionStation<RBX::BasicPartInstance>::~ActionStation()")]
#[doc(alias = "__ZThn36_N3RBX13ActionStationINS_17BasicPartInstanceEED0Ev")]
pub fn stub_062f000() {
    // IDA 0x062f000: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062f008 — __ZNSt6vectorIN3RBX18SkateboardPlatform9MoveStateESaIS2_EE6resizeEmS2_
// demangled: std::vector<RBX::SkateboardPlatform::MoveState,std::allocator<RBX::SkateboardPlatform::MoveState>>::resize(unsigned long,RBX::SkateboardPlatform::MoveState)
// type: int(void)
#[doc(alias = "std::vector<RBX::SkateboardPlatform::MoveState,std::allocator<RBX::SkateboardPlatform::MoveState>>::resize(unsigned long,RBX::SkateboardPlatform::MoveState)")]
#[doc(alias = "__ZNSt6vectorIN3RBX18SkateboardPlatform9MoveStateESaIS2_EE6resizeEmS2_")]
pub fn stub_062f008(states: &mut Vec<i32>, new_len: usize, value: i32) {
    // IDA 0x62f008 (vector<MoveState>::resize): shrink via erase or grow
    // via _M_fill_insert; Vec::resize is the same fill/truncate.
    // was: std::vector<RBX::SkateboardPlatform::MoveState,...>::resize(unsigned long,RBX::SkateboardPlatform::MoveState).
    states.resize(new_len, value);
}

// 0x062f03c — __ZNSt6vectorIN3RBX18SkateboardPlatform9MoveStateESaIS2_EE9push_backERKS2_
// demangled: std::vector<RBX::SkateboardPlatform::MoveState,std::allocator<RBX::SkateboardPlatform::MoveState>>::push_back(RBX::SkateboardPlatform::MoveState const&)
// type: int(void)
#[doc(alias = "std::vector<RBX::SkateboardPlatform::MoveState,std::allocator<RBX::SkateboardPlatform::MoveState>>::push_back(RBX::SkateboardPlatform::MoveState const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX18SkateboardPlatform9MoveStateESaIS2_EE9push_backERKS2_")]
pub fn stub_062f03c(states: &mut Vec<i32>, value: i32) {
    // IDA 0x62f03c (vector<MoveState>::push_back): realloc when full
    // (_M_insert_aux path), else construct in place; Vec::push is the same.
    // was: std::vector<RBX::SkateboardPlatform::MoveState,...>::push_back(RBX::SkateboardPlatform::MoveState const&).
    states.push(value);
}

// 0x062f064 — __ZNSt3mapIPKN3RBX4NameENS0_18SkateboardPlatform9MoveStateESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// demangled: std::map<RBX::Name const*,RBX::SkateboardPlatform::MoveState,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>>>::operator[](RBX::Name const* const&)
// type: int(void)
#[doc(alias = "std::map<RBX::Name const*,RBX::SkateboardPlatform::MoveState,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_18SkateboardPlatform9MoveStateESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
pub fn stub_062f064(map: &mut MoveStateMap, key: u32) -> &mut i32 {
    // IDA 0x62f064 (map::operator[]): lower_bound on the Name key; miss ->
    // insert a value-initialized (0) node and return it. Entry::or_insert
    // is the same miss/value-init/hit path.
    // was: std::map<RBX::Name const*,RBX::SkateboardPlatform::MoveState,...>::operator[](RBX::Name const* const&).
    map.entry(key).or_insert(0)
}

// 0x062f0bc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_18SkateboardPlatform9MoveStateEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>>,std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState> const&)
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>>,std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_18SkateboardPlatform9MoveStateEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
pub fn stub_062f0bc(map: &mut MoveStateMap, key: u32, value: i32) -> bool {
    // IDA 0x62f0bc (_M_insert_unique with hint): the hint only seeds the
    // lower_bound walk; a duplicate key still inserts nothing. Host: hint
    // is meaningless for HashMap — same delegation as audio generated.rs
    // stub_3788dc.
    stub_062f1c8(map, key, value)
}

// 0x062f170 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_18SkateboardPlatform9MoveStateEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState> const&)
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_18SkateboardPlatform9MoveStateEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
pub fn stub_062f170(map: &mut MoveStateMap, key: u32, value: i32) -> bool {
    // IDA 0x62f170 (_M_insert): links the created node and rebalances; the
    // caller established the miss. Host: same Entry keep-first insert as
    // audio generated.rs stub_3789c4.
    stub_062f1c8(map, key, value)
}

// 0x062f1c8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_18SkateboardPlatform9MoveStateEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState> const&)
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_18SkateboardPlatform9MoveStateEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
pub fn stub_062f1c8(map: &mut MoveStateMap, key: u32, value: i32) -> bool {
    // IDA 0x62f1c8 (_M_insert_unique): lower_bound on the key; on a miss
    // create the node and link it, else return the existing node. Host:
    // HashMap Entry reports vacant (true) vs occupied (false); the occupied
    // slot keeps its value, as in the original (cf. generated.rs stub_378a14).
    use std::collections::hash_map::Entry;
    match map.entry(key) {
        Entry::Vacant(slot) => {
            slot.insert(value);
            true
        }
        Entry::Occupied(_) => false,
    }
}

// 0x062f230 — __ZNSt6vectorIN3RBX18SkateboardPlatform9MoveStateESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// demangled: std::vector<RBX::SkateboardPlatform::MoveState,std::allocator<RBX::SkateboardPlatform::MoveState>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SkateboardPlatform::MoveState*,std::vector<RBX::SkateboardPlatform::MoveState,std::allocator<RBX::SkateboardPlatform::MoveState>>>,RBX::SkateboardPlatform::MoveState const&)
// type: int(void)
#[doc(alias = "std::vector<RBX::SkateboardPlatform::MoveState,std::allocator<RBX::SkateboardPlatform::MoveState>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SkateboardPlatform::MoveState*,std::vector<RBX::SkateboardPlatform::MoveState,std::allocator<RBX::SkateboardPlatform::MoveState>>>,RBX::SkateboardPlatform::MoveState const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX18SkateboardPlatform9MoveStateESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_062f230(states: &mut Vec<i32>, index: usize, value: i32) {
    // IDA 0x62f230 (vector<MoveState>::_M_insert_aux): realloc when full,
    // shift the tail back, construct at position. Vec::insert is the same
    // (it dispatches to the insert-aux slow path exactly when full).
    // was: std::vector<RBX::SkateboardPlatform::MoveState,...>::_M_insert_aux(...).
    states.insert(index, value);
}

// 0x062f314 — __ZNSt12_Vector_baseIN3RBX18SkateboardPlatform9MoveStateESaIS2_EE11_M_allocateEm
// demangled: std::_Vector_base<RBX::SkateboardPlatform::MoveState,std::allocator<RBX::SkateboardPlatform::MoveState>>::_M_allocate(unsigned long)
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::SkateboardPlatform::MoveState,std::allocator<RBX::SkateboardPlatform::MoveState>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX18SkateboardPlatform9MoveStateESaIS2_EE11_M_allocateEm")]
pub fn stub_062f314(states: &mut Vec<i32>, count: usize) {
    // IDA 0x62f314 (_Vector_base::_M_allocate): raw allocate of count
    // elements (length unchanged). Vec::reserve_exact grows capacity the
    // same without touching the length.
    // was: std::_Vector_base<RBX::SkateboardPlatform::MoveState,...>::_M_allocate(unsigned long).
    states.reserve_exact(count);
}

// 0x062f32c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX18SkateboardPlatform9MoveStateES6_EET0_T_S8_S7_
// demangled: RBX::SkateboardPlatform::MoveState * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::SkateboardPlatform::MoveState *,RBX::SkateboardPlatform::MoveState *>(RBX::SkateboardPlatform::MoveState *,RBX::SkateboardPlatform::MoveState *,RBX::SkateboardPlatform::MoveState *)
// type: int(void)
#[doc(alias = "RBX::SkateboardPlatform::MoveState * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::SkateboardPlatform::MoveState *,RBX::SkateboardPlatform::MoveState *>(RBX::SkateboardPlatform::MoveState *,RBX::SkateboardPlatform::MoveState *,RBX::SkateboardPlatform::MoveState *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX18SkateboardPlatform9MoveStateES6_EET0_T_S8_S7_")]
pub fn stub_062f32c(states: &mut [i32], src: core::ops::Range<usize>, dst_end: usize) {
    // IDA 0x62f32c (__copy_backward for MoveState*): shifts [first, last)
    // to end at result, back to front for overlap. copy_within is the same
    // memmove.
    // was: std::__copy_backward<...>::__copy_b<MoveState*,MoveState*>(MoveState*,MoveState*,MoveState*).
    states.copy_within(src, dst_end);
}

// 0x062f368 — __ZNSt6vectorIN3RBX18SkateboardPlatform9MoveStateESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// demangled: std::vector<RBX::SkateboardPlatform::MoveState,std::allocator<RBX::SkateboardPlatform::MoveState>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SkateboardPlatform::MoveState*,std::vector<RBX::SkateboardPlatform::MoveState,std::allocator<RBX::SkateboardPlatform::MoveState>>>,unsigned long,RBX::SkateboardPlatform::MoveState const&)
// type: int(void)
#[doc(alias = "std::vector<RBX::SkateboardPlatform::MoveState,std::allocator<RBX::SkateboardPlatform::MoveState>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SkateboardPlatform::MoveState*,std::vector<RBX::SkateboardPlatform::MoveState,std::allocator<RBX::SkateboardPlatform::MoveState>>>,unsigned long,RBX::SkateboardPlatform::MoveState const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX18SkateboardPlatform9MoveStateESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_062f368(states: &mut Vec<i32>, index: usize, count: usize, value: i32) {
    // IDA 0x62f368 (vector<MoveState>::_M_fill_insert): realloc when short,
    // shift the tail, fill count copies at position. splice at the empty
    // range is the same shift + fill.
    // was: std::vector<RBX::SkateboardPlatform::MoveState,...>::_M_fill_insert(...).
    states.splice(index..index, core::iter::repeat(value).take(count));
}

// 0x062f4f8 — __ZN3RBX10Reflection13BoundFuncDescINS_18SkateboardPlatformEFvN3G3D7Vector3EELi1EEC2EMS2_FvS4_EPKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// demangled: RBX::Reflection::BoundFuncDesc<RBX::SkateboardPlatform,void ()(G3D::Vector3),1>::BoundFuncDesc(void (RBX::SkateboardPlatform::*)(G3D::Vector3),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::SkateboardPlatform,void ()(G3D::Vector3),1>::BoundFuncDesc(void (RBX::SkateboardPlatform::*)(G3D::Vector3),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_18SkateboardPlatformEFvN3G3D7Vector3EELi1EEC2EMS2_FvS4_EPKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_062f4f8(
    name: &str,
    doc: &str,
    permissions: u32,
    attributes: u32,
    declare: impl FnOnce(&mut SkateboardBoundFunc),
) -> SkateboardBoundFunc {
    // IDA 0x62f4f8 (BoundFuncDesc C2): classDescriptor base (0x62f530),
    // FunctionDescriptor base (0x62f550), vtable install (0x62f566), the
    // member-function pair at +40 (0x62f574, folds into dispatch closures),
    // +48 = 0 (0x62f57e), return-type getSingleton<void> (0x62f5a0),
    // declareSignature (0x62f5b4).
    let mut desc = SkateboardBoundFunc {
        name: name.to_owned(),
        doc: doc.to_owned(),
        permissions,
        attributes,
        signature_arg: None,
    };
    declare(&mut desc);
    desc
}

// 0x062f670 — __ZN3RBX10Reflection13BoundFuncDescINS_18SkateboardPlatformEFvN3G3D7Vector3EELi1EE16declareSignatureEPKcNS0_7VariantE
// demangled: RBX::Reflection::BoundFuncDesc<RBX::SkateboardPlatform,void ()(G3D::Vector3),1>::declareSignature(char const*,RBX::Reflection::Variant)
// type: int(void)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::SkateboardPlatform,void ()(G3D::Vector3),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_18SkateboardPlatformEFvN3G3D7Vector3EELi1EE16declareSignatureEPKcNS0_7VariantE")]
pub fn stub_062f670(desc: &mut SkateboardBoundFunc, arg_name: &str) {
    // IDA 0x62f670 (declareSignature): return-type getSingleton<void> at
    // +28 (0x62f680); Name::declare the argument (0x62f68a);
    // getSingleton<Vector3> (0x62f68c); SignatureDescriptor::addArgument
    // (0x62f69e).
    desc.signature_arg = Some(arg_name.to_owned());
}

// 0x062f6a0 — __ZN3RBX10Reflection13BoundFuncDescINS_18SkateboardPlatformEFvN3G3D7Vector3EELi1EED0Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::SkateboardPlatform,void ()(G3D::Vector3),1>::~BoundFuncDesc()
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::SkateboardPlatform,void ()(G3D::Vector3),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_18SkateboardPlatformEFvN3G3D7Vector3EELi1EED0Ev")]
pub fn stub_062f6a0() {
    // IDA 0x062f6a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062f774 — __ZNK3RBX10Reflection13BoundFuncDescINS_18SkateboardPlatformEFvN3G3D7Vector3EELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// demangled: RBX::Reflection::BoundFuncDesc<RBX::SkateboardPlatform,void ()(G3D::Vector3),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::SkateboardPlatform,void ()(G3D::Vector3),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_18SkateboardPlatformEFvN3G3D7Vector3EELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_062f774(
    target: u32,
    adjust: u32,
    arg: [f32; 3],
    direct: impl FnOnce(u32, [f32; 3]),
    virtual_call: impl FnOnce(u32, [f32; 3]),
) {
    // IDA 0x62f774 (BoundFuncDesc::execute): ArgHelper::getArg<Vector3>
    // (0x62f798, the arg seam); member resolve with the odd-adjust vtable
    // step (0x62f79c-0x62f7a8, cf. 0x62b93c); invoke (0x62f7b6).
    if (adjust & 1) != 0 {
        virtual_call(target, arg);
    } else {
        direct(target, arg);
    }
}

// 0x062f7b8 — __ZN3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEEC2IMS2_KFPS3_vEiEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::RefPropDescriptor<RBX::Humanoid* (RBX::SkateboardPlatform::*)(void)const,int>(char const*,char const*,RBX::Humanoid* (RBX::SkateboardPlatform::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::RefPropDescriptor<RBX::Humanoid* (RBX::SkateboardPlatform::*)(void)const,int>(char const*,char const*,RBX::Humanoid* (RBX::SkateboardPlatform::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEEC2IMS2_KFPS3_vEiEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_062f7b8(name: &str, category: &str, attributes: u32, permissions: u32) -> SkateboardRefProp {
    // IDA 0x62f7b8 (RefPropDescriptor<Humanoid> C2):
    // RefType<Humanoid*>::singleton (0x62f7ce); PropertyDescriptor base
    // (0x62f810); vtable installs (0x62f826-0x62f828); GetImpl new with the
    // member pair (0x62f82c-0x62f846, folds into getter/setter seams); flag
    // mask &= 0xF3 at +28 (0x62f84e, a descriptor-fixed flag — attributes
    // pass through unchanged).
    SkateboardRefProp {
        name: name.to_owned(),
        category: category.to_owned(),
        attributes,
        permissions,
    }
}

// 0x062f85c — __ZN3RBX10Reflection7RefTypeIPNS_8HumanoidEE9singletonEv
// demangled: RBX::Reflection::RefType<RBX::Humanoid *>::singleton(void)
#[doc(alias = "RBX::Reflection::RefType<RBX::Humanoid *>::singleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection7RefTypeIPNS_8HumanoidEE9singletonEv")]
pub fn stub_062f85c(init: impl FnOnce() -> u32) -> u32 {
    // IDA 0x62f85c (RefType<Humanoid*>::singleton): guarded once-init
    // (0x62f8b8); Type::Type<Humanoid*> (0x62f8e6); vtable install
    // (0x62f8fa); guard release (0x62f8fe). OnceLock is the same once.
    *HUMANOID_REF_TYPE.get_or_init(init)
}

// 0x062f954 — __ZN3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEED0Ev
// demangled: RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::~RefPropDescriptor()
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::~RefPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEED0Ev")]
pub fn stub_062f954() {
    // IDA 0x062f954: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062f984 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE10isReadOnlyEv
// demangled: RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE10isReadOnlyEv")]
pub fn stub_062f984(is_read_only: impl FnOnce() -> bool) -> bool {
    // IDA 0x62f984 (RefProp::isReadOnly): delegates to the GetImpl vtable
    // slot0 (0x62f990).
    is_read_only()
}

// 0x062f994 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE11isWriteOnlyEv
// demangled: RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE11isWriteOnlyEv")]
pub fn stub_062f994(is_write_only: impl FnOnce() -> bool) -> bool {
    // IDA 0x62f994 (RefProp::isWriteOnly): delegates to the GetImpl vtable
    // slot1 (0x62f9a0).
    is_write_only()
}

// 0x062f9a4 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE11equalValuesEPKNS0_13DescribedBaseES7_
// demangled: RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE11equalValuesEPKNS0_13DescribedBaseES7_")]
pub fn stub_062f9a4(get: impl Fn(u32) -> u32, a: u32, b: u32) -> bool {
    // IDA 0x62f9a4 (RefProp::equalValues): v = getter(a) (0x62f9b4);
    // return v == getter(b) (0x62f9ca).
    get(a) == get(b)
}

// 0x062f9cc — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// demangled: RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_062f9cc<T>(obj: u32, get: impl FnOnce(u32) -> Option<SharedPtr<T>>) -> Option<SharedPtr<T>> {
    // IDA 0x62f9cc (RefProp::getVariant): v = GetImpl getter(obj)
    // (0x62f9f0); shared_from (0x62f9f8, null stays null); the +36
    // subobject adjust plus the Variant type/singleton write
    // (0x62f9f8-0x62fa62) ride the caller's Variant; releases
    // (0x62fa3a-0x62fa70) ride the dropped clones.
    get(obj)
}

// 0x062fae4 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// demangled: RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_062fae4<T>(obj: u32, value: Option<SharedPtr<T>>, set: impl FnOnce(u32, Option<SharedPtr<T>>)) {
    // IDA 0x62fae4 (RefProp::setVariant): shared = Variant::get<shared>
    // (0x62fb08); setter dispatch through vtable+64 (0x62fb46); release
    // (0x62fb4a-0x62fb52, the dropped clone).
    set(obj, value);
}

// 0x062fbac — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE9copyValueEPKNS0_13DescribedBaseEPS5_
// demangled: RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE9copyValueEPKNS0_13DescribedBaseEPS5_")]
pub fn stub_062fbac(src: u32, dst: u32, get: impl FnOnce(u32) -> u32, set: impl FnOnce(u32, u32)) {
    // IDA 0x62fbac (RefProp::copyValue): v = getter(src) (0x62fbbe);
    // setter(dst, v) (0x62fbce).
    let v = get(src);
    set(dst, v);
}

// 0x062fbd0 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// demangled: RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
pub fn stub_062fbd0(get: impl FnOnce() -> Option<u32>, write: impl FnOnce(Option<u32>)) {
    // IDA 0x62fbd0 (RefProp::writeValue): v = getter() (0x62fbf4); the +36
    // DescribedBase adjust (0x62fbfc-0x62fbfe) collapses — host ids aren't
    // pointers; null stays null either way; InstanceHandle wrap (0x62fc02);
    // XmlNameValuePair::setValue (0x62fc3a); release (0x62fc40-0x62fc48 via
    // drop).
    write(get());
}

// 0x062fca4 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// demangled: RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_062fca4(read: impl FnOnce(Option<u32>) -> i32, handle: Option<u32>) -> i32 {
    // IDA 0x62fca4 (RefProp::readValue): element = handle ? handle+12 :
    // null (0x62fcb0-0x62fcb2); binder resolve dispatch (0x62fcb4). The +12
    // element step rides in the handle seam.
    read(handle)
}

// 0x062fcc8 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE11getRefValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::getRefValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::getRefValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE11getRefValueEPKNS0_13DescribedBaseE")]
pub fn stub_062fcc8(get: impl FnOnce() -> Option<u32>) -> Option<u32> {
    // IDA 0x62fcc8 (RefProp::getRefValue): v = getter() (0x62fcd2); the +36
    // DescribedBase adjust (0x62fcd6-0x62fcd8) collapses — host ids aren't
    // pointers; null stays null either way (0x62fcd6).
    get()
}

// 0x062fcdc — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE11setRefValueEPNS0_13DescribedBaseES6_
// demangled: RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const
// type: int __fastcall(int, int, void *lpsrc)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE11setRefValueEPNS0_13DescribedBaseES6_")]
pub fn stub_062fcdc(
    obj: u32,
    value: Option<u32>,
    is_humanoid: impl FnOnce(u32) -> bool,
    set: impl FnOnce(u32, Option<u32>),
) {
    // IDA 0x62fcdc (RefProp::setRefValue): null passes through
    // (0x62fce6-0x62fce8); __dynamic_cast to Humanoid (0x62fd0a); a failed
    // cast throws bad_cast (0x62fd24-0x62fd52); setter dispatch (0x62fd20).
    if let Some(v) = value {
        if !is_humanoid(v) {
            panic!("0x62fcdc std::bad_cast: setRefValue on non-Humanoid");
        }
    }
    set(obj, value);
}

// 0x062fd58 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_
// demangled: RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_")]
pub fn stub_062fd58(obj: u32, value: u32, set: impl FnOnce(u32, u32)) {
    // IDA 0x62fd58 (RefProp::setRefValueUnsafe): null stays null, else the
    // -36 DescribedBase adjust (0x62fd5e-0x62fd68); setter dispatch
    // (0x62fd76).
    set(obj, if value == 0 { 0 } else { value - 36 });
}

// 0x062fd78 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// demangled: RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE")]
pub fn stub_062fd78<T>(obj: u32, instance: &SharedPtr<T>, set: impl FnOnce(u32, &SharedPtr<T>)) {
    // IDA 0x62fd78 (RefProp::assignIDREF): shared_count copy (0x62fda6);
    // the null-stays-null/-36 adjust (0x62fdbc-0x62fde2) collapses — host
    // ids aren't pointers; setter dispatch (0x62fdf2); release
    // (0x62fdf6-0x62fdfe via drop).
    let owned = SharedPtr::clone(instance);
    set(obj, &owned);
}

// 0x062fe58 — __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// demangled: non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const
#[doc(alias = "non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
#[doc(alias = "__ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE")]
pub fn stub_062fe58() {
    // IDA 0x062fe58: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062fe60 — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEPNS_8HumanoidEE7GetImplIMS2_KFS4_vEE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid *>::GetImpl<RBX::Humanoid * (RBX::SkateboardPlatform::*)(void)const>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid *>::GetImpl<RBX::Humanoid * (RBX::SkateboardPlatform::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEPNS_8HumanoidEE7GetImplIMS2_KFS4_vEE10isReadOnlyEv")]
pub fn stub_062fe60() -> bool {
    // IDA 0x62fe60 (GetImpl::isReadOnly): hardcoded 1 (0x62fe62) — the
    // const member getter is never writable through this impl.
    true
}

// 0x062fe64 — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEPNS_8HumanoidEE7GetImplIMS2_KFS4_vEE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid *>::GetImpl<RBX::Humanoid * (RBX::SkateboardPlatform::*)(void)const>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid *>::GetImpl<RBX::Humanoid * (RBX::SkateboardPlatform::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEPNS_8HumanoidEE7GetImplIMS2_KFS4_vEE11isWriteOnlyEv")]
pub fn stub_062fe64() -> bool {
    // IDA 0x62fe64 (GetImpl::isWriteOnly): hardcoded 0 (0x62fe66).
    false
}

// 0x062fe68 — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEPNS_8HumanoidEE7GetImplIMS2_KFS4_vEE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid *>::GetImpl<RBX::Humanoid * (RBX::SkateboardPlatform::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid *>::GetImpl<RBX::Humanoid * (RBX::SkateboardPlatform::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEPNS_8HumanoidEE7GetImplIMS2_KFS4_vEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_062fe68(
    obj: u32,
    adjust: u32,
    direct: impl FnOnce(u32) -> u32,
    virtual_call: impl FnOnce(u32) -> u32,
) -> u32 {
    // IDA 0x62fe68 (GetImpl::getValue): the null-stays-null/-36 Described
    // adjust (0x62fe6c-0x62fe6e) collapses — host ids aren't pointers;
    // member resolve with the odd-adjust vtable step (0x62fe72-0x62fe82,
    // cf. 0x62b93c); invoke (0x62fe82).
    if (adjust & 1) != 0 {
        virtual_call(obj)
    } else {
        direct(obj)
    }
}

// 0x062fe88 — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEPNS_8HumanoidEE7GetImplIMS2_KFS4_vEE8setValueEPNS0_13DescribedBaseERKS4_
// demangled: RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid *>::GetImpl<RBX::Humanoid * (RBX::SkateboardPlatform::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::Humanoid * const&)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid *>::GetImpl<RBX::Humanoid * (RBX::SkateboardPlatform::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::Humanoid * const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEPNS_8HumanoidEE7GetImplIMS2_KFS4_vEE8setValueEPNS0_13DescribedBaseERKS4_")]
pub fn stub_062fe88() -> ! {
    // IDA 0x62fe88 (GetImpl::setValue): always throws
    // runtime_error("can't set value") (0x62feb4-0x62ff98) — the const
    // getter has no setter.
    panic!("can't set value");
}

// 0x062ffa8 — __ZN3RBX10Reflection7RefTypeIPNS_8HumanoidEED1Ev
// demangled: RBX::Reflection::RefType<RBX::Humanoid *>::~RefType()
#[doc(alias = "RBX::Reflection::RefType<RBX::Humanoid *>::~RefType()")]
#[doc(alias = "__ZN3RBX10Reflection7RefTypeIPNS_8HumanoidEED1Ev")]
pub fn stub_062ffa8() {
    // IDA 0x062ffa8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062ffac — __ZN3RBX10Reflection4TypeC2IPNS_8HumanoidEEEPKcS6_PT_
// demangled: RBX::Reflection::Type::Type<RBX::Humanoid *>(char const*,char const*,RBX::Humanoid * *)
// type: int(void)
#[doc(alias = "RBX::Reflection::Type::Type<RBX::Humanoid *>(char const*,char const*,RBX::Humanoid * *)")]
#[doc(alias = "__ZN3RBX10Reflection4TypeC2IPNS_8HumanoidEEEPKcS6_PT_")]
pub fn stub_062ffac(
    tag: &str,
    declare: impl FnOnce(&str) -> String,
    register: impl FnOnce(&str),
) -> SkateboardTypeDesc {
    // IDA 0x62ffac (Type<Humanoid*> C2): Descriptor base (0x62ffc2);
    // vtable + typeinfo installs (0x62ffe2-0x62ffe4); Name::declare the tag
    // (0x62ffec-0x62fff6); assert !tag.empty() (Type.h:77,
    // 0x62fffa-0x630018); addToAllTypes (0x630048).
    let name = declare(tag);
    assert!(!name.is_empty(), "!this->tag.empty() Type.h:77");
    register(&name);
    SkateboardTypeDesc { name }
}

// 0x0630058 — __ZN3RBX10Reflection7RefTypeIPNS_8HumanoidEED0Ev
// demangled: RBX::Reflection::RefType<RBX::Humanoid *>::~RefType()
#[doc(alias = "RBX::Reflection::RefType<RBX::Humanoid *>::~RefType()")]
#[doc(alias = "__ZN3RBX10Reflection7RefTypeIPNS_8HumanoidEED0Ev")]
pub fn stub_0630058() {
    // IDA 0x0630058: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063005c — __ZN3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEEC2IMS2_KFPS3_vEiEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::RefPropDescriptor<RBX::SkateboardController* (RBX::SkateboardPlatform::*)(void)const,int>(char const*,char const*,RBX::SkateboardController* (RBX::SkateboardPlatform::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::RefPropDescriptor<RBX::SkateboardController* (RBX::SkateboardPlatform::*)(void)const,int>(char const*,char const*,RBX::SkateboardController* (RBX::SkateboardPlatform::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEEC2IMS2_KFPS3_vEiEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_063005c(name: &str, category: &str, attributes: u32, permissions: u32) -> SkateboardRefProp {
    // IDA 0x63005c (RefPropDescriptor<SkateboardController> C2): same shape
    // as the Humanoid twin 0x62f7b8 — RefType<SkateboardController*>::
    // singleton, PropertyDescriptor base, vtable installs, GetImpl new
    // with the member pair, flag mask &= 0xF3 at +28.
    SkateboardRefProp {
        name: name.to_owned(),
        category: category.to_owned(),
        attributes,
        permissions,
    }
}

// 0x0630100 — __ZN3RBX10Reflection7RefTypeIPNS_20SkateboardControllerEE9singletonEv
// demangled: RBX::Reflection::RefType<RBX::SkateboardController *>::singleton(void)
#[doc(alias = "RBX::Reflection::RefType<RBX::SkateboardController *>::singleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection7RefTypeIPNS_20SkateboardControllerEE9singletonEv")]
pub fn stub_0630100(init: impl FnOnce() -> u32) -> u32 {
    // IDA 0x630100 (RefType<SkateboardController*>::singleton): same
    // guarded once-init shape as the Humanoid twin 0x62f85c.
    *CONTROLLER_REF_TYPE.get_or_init(init)
}

// 0x06301f8 — __ZN3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEED0Ev
// demangled: RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::~RefPropDescriptor()
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::~RefPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEED0Ev")]
pub fn stub_06301f8() {
    // IDA 0x06301f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0630228 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE10isReadOnlyEv
// demangled: RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE10isReadOnlyEv")]
pub fn stub_0630228(is_read_only: impl FnOnce() -> bool) -> bool {
    // IDA 0x630228: delegates to the GetImpl vtable slot0 (0x630234);
    // same shape as 0x62f984.
    is_read_only()
}

// 0x0630238 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE11isWriteOnlyEv
// demangled: RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE11isWriteOnlyEv")]
pub fn stub_0630238(is_write_only: impl FnOnce() -> bool) -> bool {
    // IDA 0x630238: delegates to the GetImpl vtable slot1 (0x630244);
    // same shape as 0x62f994.
    is_write_only()
}

// 0x0630248 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE11equalValuesEPKNS0_13DescribedBaseES7_
// demangled: RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE11equalValuesEPKNS0_13DescribedBaseES7_")]
pub fn stub_0630248(get: impl Fn(u32) -> u32, a: u32, b: u32) -> bool {
    // IDA 0x630248: v = getter(a) (0x630258); return v == getter(b)
    // (0x63026e); same shape as 0x62f9a4.
    get(a) == get(b)
}

// 0x0630270 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// demangled: RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
// type: void __fastcall(int, int, _DWORD *, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_0630270<T>(obj: u32, get: impl FnOnce(u32) -> Option<SharedPtr<T>>) -> Option<SharedPtr<T>> {
    // IDA 0x630270: v = GetImpl getter(obj) (0x630294); shared_from
    // (0x63029c); Variant write rides the caller; same shape as 0x62f9cc.
    get(obj)
}

// 0x0630388 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// demangled: RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_0630388<T>(obj: u32, value: Option<SharedPtr<T>>, set: impl FnOnce(u32, Option<SharedPtr<T>>)) {
    // IDA 0x630388: shared = Variant::get<shared> (0x6303ac); setter
    // dispatch (0x6303e8); release via drop; same shape as 0x62fae4.
    set(obj, value);
}

// 0x0630450 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE9copyValueEPKNS0_13DescribedBaseEPS5_
// demangled: RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE9copyValueEPKNS0_13DescribedBaseEPS5_")]
pub fn stub_0630450(src: u32, dst: u32, get: impl FnOnce(u32) -> u32, set: impl FnOnce(u32, u32)) {
    // IDA 0x630450: v = getter(src) (0x630462); setter(dst, v) (0x630472);
    // same shape as 0x62fbac.
    let v = get(src);
    set(dst, v);
}

// 0x0630474 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// demangled: RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
pub fn stub_0630474(get: impl FnOnce() -> Option<u32>, write: impl FnOnce(Option<u32>)) {
    // IDA 0x630474: v = getter() (0x630498); +36 collapses, null stays
    // null (0x6304a0-0x6304a2); InstanceHandle wrap (0x6304a6); setValue
    // (0x6304de); same shape as 0x62fbd0.
    write(get());
}

// 0x0630548 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// demangled: RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// type: int __fastcall(int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_0630548(read: impl FnOnce(Option<u32>) -> i32, handle: Option<u32>) -> i32 {
    // IDA 0x630548: element = handle ? handle+12 : null (0x630554);
    // binder resolve dispatch (0x63055a); same shape as 0x62fca4.
    read(handle)
}

// 0x063056c — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE11getRefValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::getRefValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::getRefValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE11getRefValueEPKNS0_13DescribedBaseE")]
pub fn stub_063056c(get: impl FnOnce() -> Option<u32>) -> Option<u32> {
    // IDA 0x63056c: v = getter() (0x630572); +36 collapses, null stays
    // null (0x63057a); same shape as 0x62fcc8.
    get()
}

// 0x0630580 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE11setRefValueEPNS0_13DescribedBaseES6_
// demangled: RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const
// type: int __fastcall(int, int, void *lpsrc)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE11setRefValueEPNS0_13DescribedBaseES6_")]
pub fn stub_0630580(
    obj: u32,
    value: Option<u32>,
    is_controller: impl FnOnce(u32) -> bool,
    set: impl FnOnce(u32, Option<u32>),
) {
    // IDA 0x630580: null passes through (0x63058a-0x63058c);
    // __dynamic_cast to SkateboardController (0x6305ae); failed cast throws
    // bad_cast (0x6305b2); setter dispatch (0x6305c4); same shape as
    // 0x62fcdc.
    if let Some(v) = value {
        if !is_controller(v) {
            panic!("0x630580 std::bad_cast: setRefValue on non-SkateboardController");
        }
    }
    set(obj, value);
}

// 0x06305fc — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_
// demangled: RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_")]
pub fn stub_06305fc(obj: u32, value: u32, set: impl FnOnce(u32, u32)) {
    // IDA 0x6305fc: null stays null, else -36 (cf. 0x62fd5e-0x62fd68);
    // setter dispatch; same shape as 0x62fd58.
    set(obj, if value == 0 { 0 } else { value - 36 });
}

// 0x063061c — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// demangled: RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const
// type: void __fastcall(int, int, const shared_count *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE")]
pub fn stub_063061c<T>(obj: u32, instance: &SharedPtr<T>, set: impl FnOnce(u32, &SharedPtr<T>)) {
    // IDA 0x63061c: shared_count copy; -36 collapses; setter dispatch;
    // release via drop; same shape as 0x62fd78.
    let owned = SharedPtr::clone(instance);
    set(obj, &owned);
}

// 0x06306fc — __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// demangled: non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
#[doc(alias = "__ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE")]
pub fn stub_06306fc() {
    // IDA 0x06306fc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0630704 — __ZN3RBX11shared_fromINS_20SkateboardControllerEEEN5boost10shared_ptrIT_EEPS4_
// demangled: boost::shared_ptr<RBX::SkateboardController> RBX::shared_from<RBX::SkateboardController>(RBX::SkateboardController*)
// type: void __fastcall(_QWORD *, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::SkateboardController> RBX::shared_from<RBX::SkateboardController>(RBX::SkateboardController*)")]
#[doc(alias = "__ZN3RBX11shared_fromINS_20SkateboardControllerEEEN5boost10shared_ptrIT_EEPS4_")]
pub fn stub_0630704<T>(slot: Option<&SharedPtr<T>>) -> Option<SharedPtr<T>> {
    // IDA 0x630704 (shared_from<SkateboardController>): null -> empty
    // (0x630752/0x6307e0); missing weak (0x63075c) or expired count
    // (0x630798) -> throw bad_weak_ptr (0x63080a-0x630860); else addref +
    // copy (0x63078a-0x6307c8). Same shape as stub_0629eec in
    // generated_audio_wd_watchdog5; a borrowed Arc is always live so the
    // throw is unrepresentable.
    slot.map(SharedPtr::clone)
}

// 0x0630874 — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEPNS_20SkateboardControllerEE7GetImplIMS2_KFS4_vEE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController *>::GetImpl<RBX::SkateboardController * (RBX::SkateboardPlatform::*)(void)const>::isReadOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController *>::GetImpl<RBX::SkateboardController * (RBX::SkateboardPlatform::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEPNS_20SkateboardControllerEE7GetImplIMS2_KFS4_vEE10isReadOnlyEv")]
pub fn stub_0630874() -> bool {
    // IDA 0x630874 (GetImpl::isReadOnly): hardcoded 1 — same shape as
    // 0x62fe60.
    true
}

// 0x0630878 — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEPNS_20SkateboardControllerEE7GetImplIMS2_KFS4_vEE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController *>::GetImpl<RBX::SkateboardController * (RBX::SkateboardPlatform::*)(void)const>::isWriteOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController *>::GetImpl<RBX::SkateboardController * (RBX::SkateboardPlatform::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEPNS_20SkateboardControllerEE7GetImplIMS2_KFS4_vEE11isWriteOnlyEv")]
pub fn stub_0630878() -> bool {
    // IDA 0x630878 (GetImpl::isWriteOnly): hardcoded 0 — same shape as
    // 0x62fe64.
    false
}

// 0x063087c — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEPNS_20SkateboardControllerEE7GetImplIMS2_KFS4_vEE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController *>::GetImpl<RBX::SkateboardController * (RBX::SkateboardPlatform::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController *>::GetImpl<RBX::SkateboardController * (RBX::SkateboardPlatform::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEPNS_20SkateboardControllerEE7GetImplIMS2_KFS4_vEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_063087c(
    obj: u32,
    adjust: u32,
    direct: impl FnOnce(u32) -> u32,
    virtual_call: impl FnOnce(u32) -> u32,
) -> u32 {
    // IDA 0x63087c (GetImpl::getValue): -36 collapses; member resolve with
    // the odd-adjust vtable step; invoke; same shape as 0x62fe68.
    if (adjust & 1) != 0 {
        virtual_call(obj)
    } else {
        direct(obj)
    }
}

// 0x063089c — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEPNS_20SkateboardControllerEE7GetImplIMS2_KFS4_vEE8setValueEPNS0_13DescribedBaseERKS4_
// demangled: RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController *>::GetImpl<RBX::SkateboardController * (RBX::SkateboardPlatform::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::SkateboardController * const&)const
// type: void __noreturn()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController *>::GetImpl<RBX::SkateboardController * (RBX::SkateboardPlatform::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::SkateboardController * const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEPNS_20SkateboardControllerEE7GetImplIMS2_KFS4_vEE8setValueEPNS0_13DescribedBaseERKS4_")]
pub fn stub_063089c() -> ! {
    // IDA 0x63089c (GetImpl::setValue): always throws
    // runtime_error("can't set value") — same shape as 0x62fe88.
    panic!("can't set value");
}

// 0x06309bc — __ZN3RBX10Reflection7RefTypeIPNS_20SkateboardControllerEED1Ev
// demangled: RBX::Reflection::RefType<RBX::SkateboardController *>::~RefType()
// type: void()
#[doc(alias = "RBX::Reflection::RefType<RBX::SkateboardController *>::~RefType()")]
#[doc(alias = "__ZN3RBX10Reflection7RefTypeIPNS_20SkateboardControllerEED1Ev")]
pub fn stub_06309bc() {
    // IDA 0x06309bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x06309c0 — __ZN3RBX10Reflection4TypeC2IPNS_20SkateboardControllerEEEPKcS6_PT_
// demangled: RBX::Reflection::Type::Type<RBX::SkateboardController *>(char const*,char const*,RBX::SkateboardController * *)
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::Type::Type<RBX::SkateboardController *>(char const*,char const*,RBX::SkateboardController * *)")]
#[doc(alias = "__ZN3RBX10Reflection4TypeC2IPNS_20SkateboardControllerEEEPKcS6_PT_")]
pub fn stub_06309c0(
    tag: &str,
    declare: impl FnOnce(&str) -> String,
    register: impl FnOnce(&str),
) -> SkateboardTypeDesc {
    // IDA 0x6309c0 (Type<SkateboardController*> C2): Descriptor base;
    // vtable + typeinfo installs; Name::declare the tag; assert
    // !tag.empty() (Type.h:77); addToAllTypes; same shape as 0x62ffac.
    let name = declare(tag);
    assert!(!name.is_empty(), "!this->tag.empty() Type.h:77");
    register(&name);
    SkateboardTypeDesc { name }
}

// 0x0630a6c — __ZN3RBX10Reflection7RefTypeIPNS_20SkateboardControllerEED0Ev
// demangled: RBX::Reflection::RefType<RBX::SkateboardController *>::~RefType()
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RefType<RBX::SkateboardController *>::~RefType()")]
#[doc(alias = "__ZN3RBX10Reflection7RefTypeIPNS_20SkateboardControllerEED0Ev")]
pub fn stub_0630a6c() {
    // IDA 0x0630a6c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0630a70 — __ZN3RBX10Reflection9EventDescINS_18SkateboardPlatformEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// demangled: RBX::Reflection::EventDesc<RBX::SkateboardPlatform,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::SkateboardPlatform::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::SkateboardPlatform::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::SkateboardPlatform,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::SkateboardPlatform::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::SkateboardPlatform::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_18SkateboardPlatformEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0630a70() {
    // IDA 0x0630a70: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x0630bf4 — __ZN3RBX10Reflection9EventDescINS_18SkateboardPlatformEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED0Ev
// demangled: RBX::Reflection::EventDesc<RBX::SkateboardPlatform,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::SkateboardPlatform::*>::~EventDesc()
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::SkateboardPlatform,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::SkateboardPlatform::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_18SkateboardPlatformEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED0Ev")]
pub fn stub_0630bf4() {
    // IDA 0x0630bf4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
