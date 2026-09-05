// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: RBX::Instance|DataModel|Workspace strict (60) + RBX::Part|Model|Humanoid extras (28); EA-sorted asc, NOT stubbed in any crate
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 88 stubs | range 0xf59bc4..0xf47364 | strict filter now EXHAUSTED (all 10774 covered)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use rbx_core::WeakPtr;
use rbx_core::signal::Signal;
use crate::data_model::{DataModel, FunctorOp};
use crate::generated_05::{Instance, PairConnection, SignatureItem, instance_is_a};
use crate::generated_b::BoundMethod;
use std::collections::BTreeMap;
use crate::instance::{EnumSlot, LegacyPartType};

/// Rust model of `G3D::Vector2` bound by the `0xf59db4` callback: the plain
/// float pair.
#[derive(Clone, Copy, Default)]
pub struct DmVec2 {
    pub x: f32,
    pub y: f32,
}

/// Rust model of `RBX::UIEvent` bound by the `0xf59d94`/`0xf5dc4` callbacks.
/// // BUG: the event payload layout is unrecovered; opaque by-value carrier.
#[derive(Clone, Copy, Default)]
pub struct UiEvent {
    _opaque: (),
}

/// Rust model of `RBX::Camera::CameraPanMode` bound by `0xf59bf4`/`0xf59e14`.
/// // BUG: enumerants land with the camera batch; the word travels as-is.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct CameraPanMode(pub u32);

/// Rust model of `boost::_bi::bind_t<void, void(*)(weak_ptr<DataModel>, bool),
/// list2<value<weak>, value<bool>>>` (IDA `0xf59bc4`, `0xf59de4`): the
/// retained weak plus the bound flag.
#[derive(Clone)]
pub struct DmWeakBoolBind {
    pub weak: WeakPtr<DataModel>,
    pub flag: bool,
}
/// Invoker installed for the weak/flag bind (cf. `ViewGameFn` at 0x2d544).
pub type DmWeakBoolFn = fn(WeakPtr<DataModel>, bool);

/// Rust model of `boost::_bi::bind_t<void, void(*)(float, weak_ptr<DataModel>),
/// list2<value<float>, value<weak>>>` (IDA `0xf59bd4`, `0xf59df4`).
#[derive(Clone)]
pub struct DmFloatWeakBind {
    pub value: f32,
    pub weak: WeakPtr<DataModel>,
}
/// Invoker installed for the float/weak bind.
pub type DmFloatWeakFn = fn(f32, WeakPtr<DataModel>);

/// Rust model of `boost::_bi::bind_t<void, void(*)(float, float,
/// weak_ptr<DataModel>), list3<...>>` (IDA `0xf59be4`, `0xf59e04`).
#[derive(Clone)]
pub struct DmFloat2WeakBind {
    pub x: f32,
    pub y: f32,
    pub weak: WeakPtr<DataModel>,
}
/// Invoker installed for the float-pair/weak bind.
pub type DmFloat2WeakFn = fn(f32, f32, WeakPtr<DataModel>);

/// Rust model of `boost::_bi::bind_t<void, void(*)(float, float, bool,
/// CameraPanMode, weak_ptr<DataModel>), list5<...>>` (IDA `0xf59bf4`,
/// `0xf59e14`).
#[derive(Clone)]
pub struct DmCameraBind {
    pub x: f32,
    pub y: f32,
    pub flag: bool,
    pub pan: CameraPanMode,
    pub weak: WeakPtr<DataModel>,
}
/// Invoker installed for the camera bind.
pub type DmCameraFn = fn(f32, f32, bool, CameraPanMode, WeakPtr<DataModel>);

/// Rust model of `boost::_bi::bind_t<void, void(*)(UIEvent, void*,
/// weak_ptr<DataModel>), list3<...>>` (IDA `0xf59d94`): the by-value event,
/// the unretained data pointer, and the weak.
#[derive(Clone)]
pub struct DmUiEventBind {
    pub event: UiEvent,
    pub data: *mut (),
    pub weak: WeakPtr<DataModel>,
}
/// Invoker installed for the UI-event bind.
pub type DmUiEventFn = fn(UiEvent, *mut (), WeakPtr<DataModel>);

/// Rust model of `boost::_bi::bind_t<void, void(*)(weak_ptr<DataModel>),
/// list1<...>>` (IDA `0xf59da4`).
#[derive(Clone)]
pub struct DmWeakBind {
    pub weak: WeakPtr<DataModel>,
}
/// Invoker installed for the bare-weak bind.
pub type DmWeakFn = fn(WeakPtr<DataModel>);

/// Rust model of `boost::_bi::bind_t<void, void(*)(weak_ptr<DataModel>,
/// Vector2, float), list3<...>>` (IDA `0xf59db4`).
#[derive(Clone)]
pub struct DmWeakVec2Bind {
    pub weak: WeakPtr<DataModel>,
    pub point: DmVec2,
    pub value: f32,
}
/// Invoker installed for the weak/point bind.
pub type DmWeakVec2Fn = fn(WeakPtr<DataModel>, DmVec2, f32);

/// Rust model of `boost::_bi::bind_t<void, void(*)(weak_ptr<DataModel>,
/// UIEvent), list2<...>>` (IDA `0xf59dc4`).
#[derive(Clone)]
pub struct DmWeakUiBind {
    pub weak: WeakPtr<DataModel>,
    pub event: UiEvent,
}
/// Invoker installed for the weak/event bind.
pub type DmWeakUiFn = fn(WeakPtr<DataModel>, UiEvent);

/// Rust model of `boost::_bi::bind_t<void, void(*)(weak_ptr<DataModel>,
/// std::string, bool), list3<...>>` (IDA `0xf59dd4`): the weak plus the
/// copied string and flag.
#[derive(Clone)]
pub struct DmWeakStringBind {
    pub weak: WeakPtr<DataModel>,
    pub text: String,
    pub flag: bool,
}
/// Invoker installed for the weak/string bind.
pub type DmWeakStringFn = fn(WeakPtr<DataModel>, String, bool);

/// Rust model of the `boost::function<void(DataModel*)>` slot shared by the
/// thirteen `0xf59bc4`-`0xf59e14` binds: every bind is fully bound (no
/// `arg<1>`), so the late `DataModel*` is discarded at call time. The stored
/// invoker (the installed `stored_vtable`, cf. IDA `0x2d5ba`) rides each
/// variant; empty is the cleared state (cf. `DataModelCallback`).
#[derive(Clone)]
pub enum DmVoidBind {
    WeakBool(DmWeakBoolBind, DmWeakBoolFn),
    FloatWeak(DmFloatWeakBind, DmFloatWeakFn),
    Float2Weak(DmFloat2WeakBind, DmFloat2WeakFn),
    Camera(DmCameraBind, DmCameraFn),
    UiEvent(DmUiEventBind, DmUiEventFn),
    Weak(DmWeakBind, DmWeakFn),
    WeakVec2(DmWeakVec2Bind, DmWeakVec2Fn),
    WeakUi(DmWeakUiBind, DmWeakUiFn),
    WeakString(DmWeakStringBind, DmWeakStringFn),
}
/// Nullable `function<void(DataModel*)>` holding one of the thirteen binds.
#[derive(Clone, Default)]
pub struct DmVoidCallback {
    bind: Option<DmVoidBind>,
}
impl DmVoidCallback {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn is_empty(&self) -> bool {
        self.bind.is_none()
    }
    pub fn call(&self, dm: *mut DataModel) {
        // No `arg<1>` appears in any of the thirteen bind lists, so the late
        // arg is discarded by every arm; cloning the weak re-arms the same
        // `weak_add_ref` the original `shared_count` copy ran.
        let _ = dm;
        match &self.bind {
            None => {}
            Some(DmVoidBind::WeakBool(bind, invoke)) => invoke(bind.weak.clone(), bind.flag),
            Some(DmVoidBind::FloatWeak(bind, invoke)) => invoke(bind.value, bind.weak.clone()),
            Some(DmVoidBind::Float2Weak(bind, invoke)) => {
                invoke(bind.x, bind.y, bind.weak.clone())
            }
            Some(DmVoidBind::Camera(bind, invoke)) => {
                invoke(bind.x, bind.y, bind.flag, bind.pan, bind.weak.clone())
            }
            Some(DmVoidBind::UiEvent(bind, invoke)) => {
                invoke(bind.event, bind.data, bind.weak.clone())
            }
            Some(DmVoidBind::Weak(bind, invoke)) => invoke(bind.weak.clone()),
            Some(DmVoidBind::WeakVec2(bind, invoke)) => {
                invoke(bind.weak.clone(), bind.point, bind.value)
            }
            Some(DmVoidBind::WeakUi(bind, invoke)) => invoke(bind.weak.clone(), bind.event),
            Some(DmVoidBind::WeakString(bind, invoke)) => {
                invoke(bind.weak.clone(), bind.text.clone(), bind.flag)
            }
        }
    }
}

/// Rust model of `RBX::OverlayDataModel` for the dtor/weak ports below (IDA
/// `0xf5bc74`/`0xf5bd34`): only the embedded `enable_shared_from_this` weak
/// owner is modeled; the wider `DataModel` base lands with a later batch.
pub struct OverlayDataModel {
    pub weak_owner: WeakPtr<OverlayDataModel>,
}
/// A `void (OverlayDataModel::*)(int)` implementation behind
/// `OverlayVoidIntFunc::method` once resolved.
pub type OverlayVoidIntMethod = fn(*mut OverlayDataModel, i32);
/// Rust model of `BoundFuncDesc<OverlayDataModel, void(int), 1>` (IDA
/// `0xf5bb14`): signature items at `+8`, bound name (`scoped_ptr<string>`)
/// at `+12`, stored member pointer at `+10/+11` — twin of `HumanoidFuncVoid1`
/// in generated_b.
pub struct OverlayVoidIntFunc {
    pub items: Vec<SignatureItem>,
    pub method: BoundMethod,
    pub method_fn: Option<OverlayVoidIntMethod>,
    pub bound_name: Option<String>,
}
/// Rust model of `PropDescriptor<OverlayDataModel, int>` (IDA `0xf5bb24`):
/// the getter member encoding plus the stored default value.
pub struct OverlayIntProp {
    pub getter: BoundMethod,
    pub default: i32,
    pub class_name: String,
    pub name: String,
    pub permissions: u32,
    pub attributes: u32,
}

// 0xf59bc4 — j___ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EEbENS7_5list2INS7_5valueISA_EENSE_IbEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE
#[doc(alias = "j___ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EEbENS7_5list2INS7_5valueISA_EENSE_IbEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf59bc4(slot: &mut DmVoidCallback, bind: DmWeakBoolBind, invoke: DmWeakBoolFn) {
    // IDA 0xf59bc4: __picsymbolstub4 into `function<void(DataModel*)>::C2`
    // from `bind_t<void, void(*)(weak_ptr<DataModel>, bool),
    // list2<value<weak>, value<bool>>>` — installs the bound (weak, flag)
    // pair plus its invoker (the `stored_vtable`, cf. 0x2d544); the late
    // `DataModel*` has no `arg<1>` slot and is discarded at call time.
    slot.bind = Some(DmVoidBind::WeakBool(bind, invoke));
}

// 0xf59bd4 — j___ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvfNS_8weak_ptrIS2_EEENS7_5list2INS7_5valueIfEENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE
#[doc(alias = "j___ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvfNS_8weak_ptrIS2_EEENS7_5list2INS7_5valueIfEENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf59bd4(slot: &mut DmVoidCallback, bind: DmFloatWeakBind, invoke: DmFloatWeakFn) {
    // IDA 0xf59bd4: __picsymbolstub4 into `function<void(DataModel*)>::C2`
    // from `bind_t<void, void(*)(float, weak_ptr<DataModel>),
    // list2<value<float>, value<weak>>>` — same install as 0xf59bc4 for the
    // (float, weak) pair.
    slot.bind = Some(DmVoidBind::FloatWeak(bind, invoke));
}

// 0xf59be4 — j___ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvffNS_8weak_ptrIS2_EEENS7_5list3INS7_5valueIfEESF_NSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE
#[doc(alias = "j___ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvffNS_8weak_ptrIS2_EEENS7_5list3INS7_5valueIfEESF_NSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf59be4(slot: &mut DmVoidCallback, bind: DmFloat2WeakBind, invoke: DmFloat2WeakFn) {
    // IDA 0xf59be4: __picsymbolstub4 into `function<void(DataModel*)>::C2`
    // from `bind_t<void, void(*)(float, float, weak_ptr<DataModel>),
    // list3<value<float>, value<float>, value<weak>>>` — same install as
    // 0xf59bc4 for the float pair plus weak.
    slot.bind = Some(DmVoidBind::Float2Weak(bind, invoke));
}

// 0xf59bf4 — j___ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvffbNS1_6Camera13CameraPanModeENS_8weak_ptrIS2_EEENS7_5list5INS7_5valueIfEESH_NSG_IbEENSG_ISA_EENSG_ISC_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
#[doc(alias = "j___ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvffbNS1_6Camera13CameraPanModeENS_8weak_ptrIS2_EEENS7_5list5INS7_5valueIfEESH_NSG_IbEENSG_ISA_EENSG_ISC_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf59bf4(slot: &mut DmVoidCallback, bind: DmCameraBind, invoke: DmCameraFn) {
    // IDA 0xf59bf4: __picsymbolstub4 into `function<void(DataModel*)>::C2`
    // from `bind_t<void, void(*)(float, float, bool, CameraPanMode,
    // weak_ptr<DataModel>), list5<...>>` — same install as 0xf59bc4 for the
    // camera 5-tuple.
    slot.bind = Some(DmVoidBind::Camera(bind, invoke));
}

// 0xf59d94 — j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS1_7UIEventEPvNS_8weak_ptrIS2_EEENS6_5list3INS6_5valueIS8_EENSF_IS9_EENSF_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
#[doc(alias = "j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS1_7UIEventEPvNS_8weak_ptrIS2_EEENS6_5list3INS6_5valueIS8_EENSF_IS9_EENSF_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf59d94(slot: &mut DmVoidCallback, bind: DmUiEventBind, invoke: DmUiEventFn) {
    // IDA 0xf59d94: __picsymbolstub4 into `function1<void, DataModel*>::C2`
    // from `bind_t<void, void(*)(UIEvent, void*, weak_ptr<DataModel>),
    // list3<value<UIEvent>, value<void*>, value<weak>>>` — same install as
    // 0xf59bc4 for the (event, data, weak) triple.
    slot.bind = Some(DmVoidBind::UiEvent(bind, invoke));
}

// 0xf59da4 — j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EEENS6_5list1INS6_5valueIS9_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE
#[doc(alias = "j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EEENS6_5list1INS6_5valueIS9_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf59da4(slot: &mut DmVoidCallback, bind: DmWeakBind, invoke: DmWeakFn) {
    // IDA 0xf59da4: __picsymbolstub4 into `function1<void, DataModel*>::C2`
    // from `bind_t<void, void(*)(weak_ptr<DataModel>), list1<value<weak>>>`
    // — same install as 0xf59bc4 for the bare weak.
    slot.bind = Some(DmVoidBind::Weak(bind, invoke));
}

// 0xf59db4 — j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EEN3G3D7Vector2EfENS6_5list3INS6_5valueIS9_EENSF_ISB_EENSF_IfEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
#[doc(alias = "j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EEN3G3D7Vector2EfENS6_5list3INS6_5valueIS9_EENSF_ISB_EENSF_IfEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf59db4(slot: &mut DmVoidCallback, bind: DmWeakVec2Bind, invoke: DmWeakVec2Fn) {
    // IDA 0xf59db4: __picsymbolstub4 into `function1<void, DataModel*>::C2`
    // from `bind_t<void, void(*)(weak_ptr<DataModel>, Vector2, float),
    // list3<...>>` — same install as 0xf59bc4 for the (weak, point, value)
    // triple.
    slot.bind = Some(DmVoidBind::WeakVec2(bind, invoke));
}

// 0xf59dc4 — j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_7UIEventEENS6_5list2INS6_5valueIS9_EENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE
#[doc(alias = "j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_7UIEventEENS6_5list2INS6_5valueIS9_EENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf59dc4(slot: &mut DmVoidCallback, bind: DmWeakUiBind, invoke: DmWeakUiFn) {
    // IDA 0xf59dc4: __picsymbolstub4 into `function1<void, DataModel*>::C2`
    // from `bind_t<void, void(*)(weak_ptr<DataModel>, UIEvent), list2<...>>`
    // — same install as 0xf59bc4 for the (weak, event) pair.
    slot.bind = Some(DmVoidBind::WeakUi(bind, invoke));
}

// 0xf59dd4 — j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EESsbENS6_5list3INS6_5valueIS9_EENSD_ISsEENSD_IbEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE
#[doc(alias = "j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EESsbENS6_5list3INS6_5valueIS9_EENSD_ISsEENSD_IbEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf59dd4(slot: &mut DmVoidCallback, bind: DmWeakStringBind, invoke: DmWeakStringFn) {
    // IDA 0xf59dd4: __picsymbolstub4 into `function1<void, DataModel*>::C2`
    // from `bind_t<void, void(*)(weak_ptr<DataModel>, std::string, bool),
    // list3<...>>` — same install as 0xf59bc4; moving the bind copies the
    // string like the `std::string::string` copy in the 0x282ab8 precedent.
    slot.bind = Some(DmVoidBind::WeakString(bind, invoke));
}

// 0xf59de4 — j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EEbENS6_5list2INS6_5valueIS9_EENSD_IbEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE
#[doc(alias = "j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EEbENS6_5list2INS6_5valueIS9_EENSD_IbEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf59de4(slot: &mut DmVoidCallback, bind: DmWeakBoolBind, invoke: DmWeakBoolFn) {
    // IDA 0xf59de4: __picsymbolstub4 into `function1<void, DataModel*>::C2`
    // from `bind_t<void, void(*)(weak_ptr<DataModel>, bool), list2<...>>` —
    // same (weak, flag) shape as 0xf59bc4.
    slot.bind = Some(DmVoidBind::WeakBool(bind, invoke));
}

// 0xf59df4 — j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvfNS_8weak_ptrIS2_EEENS6_5list2INS6_5valueIfEENSD_IS9_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE
#[doc(alias = "j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvfNS_8weak_ptrIS2_EEENS6_5list2INS6_5valueIfEENSD_IS9_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf59df4(slot: &mut DmVoidCallback, bind: DmFloatWeakBind, invoke: DmFloatWeakFn) {
    // IDA 0xf59df4: __picsymbolstub4 into `function1<void, DataModel*>::C2`
    // from `bind_t<void, void(*)(float, weak_ptr<DataModel>), list2<...>>` —
    // same (float, weak) shape as 0xf59bd4.
    slot.bind = Some(DmVoidBind::FloatWeak(bind, invoke));
}

// 0xf59e04 — j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvffNS_8weak_ptrIS2_EEENS6_5list3INS6_5valueIfEESE_NSD_IS9_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE
#[doc(alias = "j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvffNS_8weak_ptrIS2_EEENS6_5list3INS6_5valueIfEESE_NSD_IS9_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf59e04(slot: &mut DmVoidCallback, bind: DmFloat2WeakBind, invoke: DmFloat2WeakFn) {
    // IDA 0xf59e04: __picsymbolstub4 into `function1<void, DataModel*>::C2`
    // from `bind_t<void, void(*)(float, float, weak_ptr<DataModel>),
    // list3<...>>` — same float-pair shape as 0xf59be4.
    slot.bind = Some(DmVoidBind::Float2Weak(bind, invoke));
}

// 0xf59e14 — j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvffbNS1_6Camera13CameraPanModeENS_8weak_ptrIS2_EEENS6_5list5INS6_5valueIfEESG_NSF_IbEENSF_IS9_EENSF_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
#[doc(alias = "j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvffbNS1_6Camera13CameraPanModeENS_8weak_ptrIS2_EEENS6_5list5INS6_5valueIfEESG_NSF_IbEENSF_IS9_EENSF_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf59e14(slot: &mut DmVoidCallback, bind: DmCameraBind, invoke: DmCameraFn) {
    // IDA 0xf59e14: __picsymbolstub4 into `function1<void, DataModel*>::C2`
    // from `bind_t<void, void(*)(float, float, bool, CameraPanMode,
    // weak_ptr<DataModel>), list5<...>>` — same camera shape as 0xf59bf4.
    slot.bind = Some(DmVoidBind::Camera(bind, invoke));
}

// 0xf5bb04 — j___ZN3RBX10Reflection13BoundFuncDescINS_16OverlayDataModelEFviELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::OverlayDataModel,void ()(int),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0xf5bb04(desc: *mut OverlayVoidIntFunc, name: &str) {
    // IDA 0xf5bb04: __picsymbolstub4 into
    // BoundFuncDesc<OverlayDataModel, void(int), 1>::declareSignature —
    // `void` return singleton plus the `int` arg declared + added, the same
    // shape as the Humanoid twin 0x7c7490 (singleton/Name::declare/addArgument
    // at 0x7c74a0-0x7c74be). The declared name is kept as the bound name;
    // interning has no global table yet.
    // SAFETY: `desc` must point to a valid `OverlayVoidIntFunc`.
    unsafe {
        (*desc).items.push(SignatureItem { type_name: "int" });
        (*desc).bound_name = Some(name.to_string());
    }
}

// 0xf5bb14 — j___ZN3RBX10Reflection13BoundFuncDescINS_16OverlayDataModelEFviELi1EEC2EMS2_FviEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::OverlayDataModel,void ()(int),1>::BoundFuncDesc(void (RBX::OverlayDataModel::*)(int),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0xf5bb14(
    method_hi: u32,
    method_lo: u32,
    name: &str,
    permissions: u32,
    attributes: u32,
) -> OverlayVoidIntFunc {
    // IDA 0xf5bb14: __picsymbolstub4 into
    // BoundFuncDesc<OverlayDataModel, void(int), 1>::C2 — base
    // `FunctionDescriptor` init, vtable set, member-pointer pair at `+10/+11`,
    // `scoped_ptr` at `+12` nulled, then `declareSignature(name)` (0xf5bb04)
    // over the `void` return — same shape as the Humanoid twin 0x7c7314.
    let mut desc = OverlayVoidIntFunc {
        items: Vec::new(),
        method: BoundMethod {
            raw: ((method_hi as u64) << 32) | method_lo as u64,
        },
        method_fn: None,
        bound_name: None,
    };
    stub_0xf5bb04(&mut desc as *mut _, name);
    let _ = (permissions, attributes);
    desc
}

// 0xf5bb24 — j___ZN3RBX10Reflection14PropDescriptorINS_16OverlayDataModelEiEC2IMS2_KFivEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::OverlayDataModel,int>::PropDescriptor<int (RBX::OverlayDataModel::*)(void)const,int>(char const*,char const*,int (RBX::OverlayDataModel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0xf5bb24(
    class_name: &str,
    name: &str,
    getter_hi: u32,
    getter_lo: u32,
    default: i32,
    permissions: u32,
    attributes: u32,
) -> OverlayIntProp {
    // IDA 0xf5bb24: __picsymbolstub4 into
    // PropDescriptor<OverlayDataModel, int>::C2<int
    // (OverlayDataModel::*)(void)const, int> — class link plus
    // `PropertyDescriptor` base init (compiler-managed here), the getter
    // member pair (same `+10/+11` encoding as the BoundFuncDesc C2s, cf.
    // 0x7c3986), and the stored default value.
    OverlayIntProp {
        getter: BoundMethod {
            raw: ((getter_hi as u64) << 32) | getter_lo as u64,
        },
        default,
        class_name: class_name.to_string(),
        name: name.to_string(),
        permissions,
        attributes,
    }
}

// 0xf5bc74 — j___ZN3RBX16OverlayDataModelD2Ev
#[doc(alias = "RBX::OverlayDataModel::~OverlayDataModel()")]
pub fn stub_0xf5bc74(this: *mut OverlayDataModel) {
    // IDA 0xf5bc74: __picsymbolstub4 into OverlayDataModel::D2 (non-deleting)
    // — member teardown; the modeled half is the embedded weak-owner release
    // (same shape as the ChildRemovedSignalData D1 at 0x703da4).
    // // BUG: the wider DataModel/Instance base teardown has no model yet.
    // SAFETY: `this` must point to a valid `OverlayDataModel`.
    unsafe {
        (*this).weak_owner = WeakPtr::new();
    }
}

// 0xf5bd04 — j___ZN3RBX4Name9doDeclareILZNS_17sOverlayDataModelEEEERKS0_v
#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_17sOverlayDataModelEEEERKS0_v")]
pub fn stub_0xf5bd04() -> &'static str {
    // IDA 0xf5bd04: __picsymbolstub4 into
    // Name::doDeclare<sOverlayDataModel> — returns the interned static name;
    // interning has no global table yet (cf. 0x7c3aaa), so this is the
    // literal.
    "OverlayDataModel"
}

// 0xf5bd34 — j___ZN3RBX9weak_fromINS_16OverlayDataModelEEEN5boost8weak_ptrIT_EEPS4_
#[doc(alias = "rbx_core::Weak<RBX::OverlayDataModel> RBX::weak_from<RBX::OverlayDataModel>(RBX::OverlayDataModel*)")]
// was: boost::weak_ptr<RBX::OverlayDataModel> RBX::weak_from<RBX::OverlayDataModel>(RBX::OverlayDataModel*)
pub fn stub_0xf5bd34(out: *mut WeakPtr<OverlayDataModel>, this: *const OverlayDataModel) {
    // IDA 0xf5bd34: __picsymbolstub4 into weak_from<OverlayDataModel> — same
    // shape as weak_from<Instance> (0x7039e4): null yields an empty weak,
    // else the embedded weak is cloned with a locked `weak_add_ref`; a dead
    // (never-owned or expired) owner throws `boost::bad_weak_ptr`, mapped to
    // a panic.
    // SAFETY: `out` must be writable; `this` must be null or valid.
    unsafe {
        let weak = match this.as_ref() {
            None => WeakPtr::new(),
            Some(model) => model.weak_owner.clone(),
        };
        if !this.is_null() && weak.upgrade().is_none() {
            panic!("0xf5bd34 RBX::weak_from<RBX::OverlayDataModel>: bad_weak_ptr");
        }
        core::ptr::write(out, weak);
    }
}
/// Rust model of `RBX::OverlayDataModel::JoinType` (IDA `0xf5be44`): the join
/// word travels as-is.
/// // BUG: enumerants land with a later batch; the word is opaque here.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct OverlayJoinType(pub u32);
/// Rust model of `boost::_bi::list2<value<weak_ptr<OverlayDataModel>>,
/// value<string>>` (IDA `0xf5bdf4`, storage twin `0xf5bef4`): the retained
/// weak plus the copied string.
#[derive(Clone)]
pub struct OverlayWeakStringList {
    pub weak: WeakPtr<OverlayDataModel>,
    pub text: String,
}
/// Invoker stored alongside the (weak, string) pair (cf. `0xf5bfa4`).
pub type OverlayWeakStringFn = fn(WeakPtr<OverlayDataModel>, String);
/// A `void (OverlayDataModel::*)(string const*, exception const*)`
/// implementation behind `OverlaySharedStrExc::method_fn` once resolved. Both
/// pointer args are opaque carriers — the exception layout has no model yet.
pub type OverlayStrExcMethod = fn(*mut OverlayDataModel, *const String, *const ());
/// Rust model of `boost::_bi::list3<value<shared_ptr<OverlayDataModel>>,
/// arg<1>, arg<2>>` (IDA `0xf5be14`, storage twin `0xf5bf04`, `bind` twin
/// `0xf5bf64`): the retained shared owner plus the member encoding; the list
/// ctors leave the member default, `bind` fills it — twin of
/// `OverlayVoidIntFunc`.
#[derive(Clone)]
pub struct OverlaySharedStrExc {
    pub shared: SharedPtr<OverlayDataModel>,
    pub method: BoundMethod,
    pub method_fn: Option<OverlayStrExcMethod>,
}
/// Rust model of `boost::_bi::list3<value<OverlayDataModel*>, arg<1>, arg<2>>`
/// (IDA `0xf5be34`): the unretained target plus the member encoding.
#[derive(Clone, Copy)]
pub struct OverlayRawStrExc {
    pub target: *mut OverlayDataModel,
    pub method: BoundMethod,
    pub method_fn: Option<OverlayStrExcMethod>,
}
/// A `void (OverlayDataModel::*)(int, string, JoinType)` implementation
/// behind `OverlayJoinList::method_fn` once resolved.
pub type OverlayJoinMethod = fn(*mut OverlayDataModel, i32, String, OverlayJoinType);
/// Rust model of `boost::_bi::list4<value<OverlayDataModel*>, value<int>,
/// value<string>, value<JoinType>>` (IDA `0xf5be44`, storage twin `0xf5bf14`,
/// `bind` twin `0xf5bf74`): the unretained target plus the copied args; the
/// list/storage ctors leave the member default, `bind` fills it.
#[derive(Clone)]
pub struct OverlayJoinList {
    pub target: *mut OverlayDataModel,
    pub id: i32,
    pub text: String,
    pub join: OverlayJoinType,
    pub method: BoundMethod,
    pub method_fn: Option<OverlayJoinMethod>,
}
/// A `void (OverlayDataModel::*)()` implementation behind
/// `OverlayNullaryBind::method_fn` once resolved.
pub type OverlayNullaryMethod = fn(*mut OverlayDataModel);
/// Rust model of `boost::_bi::bind_t<void, mf0<void, OverlayDataModel>,
/// list1<value<OverlayDataModel*>>>` (IDA `0xf5beb4`).
#[derive(Clone, Copy)]
pub struct OverlayNullaryBind {
    pub target: *mut OverlayDataModel,
    pub method: BoundMethod,
    pub method_fn: Option<OverlayNullaryMethod>,
}
/// Rust model of `boost::_bi::storage2<value<shared_ptr<OverlayDataModel>>,
/// arg<1>>` (IDA `0xf5bed4`): the retained shared owner.
#[derive(Clone)]
pub struct OverlaySharedArgStore {
    pub shared: SharedPtr<OverlayDataModel>,
}
/// Rust model of the `bind_t` built by `boost::bind` over the free
/// `(weak_ptr<OverlayDataModel>, string)` function (IDA `0xf5bfa4`).
#[derive(Clone)]
pub struct OverlayWeakStringBind {
    pub args: OverlayWeakStringList,
    pub invoke: OverlayWeakStringFn,
}
/// Done-slot invokers behind `DmWeakDoneBind` (IDA `0xf5c0c4`): the map
/// response and the error travel as opaque pointers.
/// // BUG: payload layouts land with the DataModel async batch.
pub type DmMapDoneFn = fn(WeakPtr<DataModel>, *const ());
pub type DmErrorFn = fn(WeakPtr<DataModel>, *const ());
/// Rust model of the `bind_t` behind `0xf5c0c4`: the retained weak plus the
/// map/error done-slots.
#[derive(Clone)]
pub struct DmWeakDoneBind {
    pub weak: WeakPtr<DataModel>,
    pub on_map: Option<DmMapDoneFn>,
    pub on_error: Option<DmErrorFn>,
}
/// Rust model of the `boost::function<void(string*, exception*)>` slot shared
/// by the `0xf5c0b4`/`0xf5c0c4` binds.
#[derive(Clone)]
pub enum StrExcBind {
    OverlayShared(OverlaySharedStrExc),
    DmWeakDone(DmWeakDoneBind),
}
/// Nullable `function<void(string*, exception*)>` holding one Overlay bind.
#[derive(Clone, Default)]
pub struct StrExcCallback {
    bind: Option<StrExcBind>,
}
impl StrExcCallback {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn is_empty(&self) -> bool {
        self.bind.is_none()
    }
    pub fn call(&self, s: *const String, e: *const ()) {
        // A null exception selects the map done-slot, mirroring the free
        // function dispatch behind 0xf5c0c4.
        match &self.bind {
            None => {}
            Some(StrExcBind::OverlayShared(bind)) => {
                if let Some(method) = bind.method_fn {
                    method(
                        std::sync::Arc::as_ptr(&bind.shared) as *mut OverlayDataModel,
                        s,
                        e,
                    );
                }
            }
            Some(StrExcBind::DmWeakDone(bind)) => {
                let weak = bind.weak.clone();
                if e.is_null() {
                    if let Some(done) = bind.on_map {
                        done(weak, s as *const ());
                    }
                } else if let Some(failed) = bind.on_error {
                    failed(weak, e);
                }
            }
        }
    }
}
/// Rust model of the `boost::function0<void>` slot shared by the
/// `0xf5c144`/`0xf5c154` binds (built by `0xf5c184`/`0xf5c194`).
#[derive(Clone)]
pub enum OverlayVoid0Bind {
    Join(OverlayJoinList),
    WeakString(OverlayWeakStringList, OverlayWeakStringFn),
}
/// Nullable `function0<void>` holding one Overlay bind.
#[derive(Clone, Default)]
pub struct OverlayVoid0Callback {
    bind: Option<OverlayVoid0Bind>,
}
impl OverlayVoid0Callback {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn is_empty(&self) -> bool {
        self.bind.is_none()
    }
    pub fn call(&self) {
        match &self.bind {
            None => {}
            Some(OverlayVoid0Bind::Join(bind)) => {
                if let Some(method) = bind.method_fn {
                    method(bind.target, bind.id, bind.text.clone(), bind.join);
                }
            }
            Some(OverlayVoid0Bind::WeakString(args, invoke)) => {
                invoke(args.weak.clone(), args.text.clone());
            }
        }
    }
}

// 0xf5bd74 — j___ZN5boost10shared_ptrIN3RBX16OverlayDataModelEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
#[doc(alias = "SharedPtr<RBX::OverlayDataModel>::shared_ptr<RBX::OverlayDataModel>(rbx_core::Weak<RBX::OverlayDataModel> const&,boost::detail::sp_nothrow_tag)")]
// was: boost::shared_ptr<RBX::OverlayDataModel>::shared_ptr<RBX::OverlayDataModel>(boost::weak_ptr<RBX::OverlayDataModel> const&,boost::detail::sp_nothrow_tag)
pub fn stub_0xf5bd74(weak: &WeakPtr<OverlayDataModel>) -> Option<SharedPtr<OverlayDataModel>> {
    // IDA 0xf5bd74: __picsymbolstub4 into
    // shared_ptr<OverlayDataModel>::shared_ptr(weak_ptr const&, sp_nothrow_tag)
    // — the nothrow twin of the throwing 0xf5bd34 path: an expired weak yields
    // an empty shared instead of throwing `bad_weak_ptr`, mapped to `None`.
    weak.upgrade()
}

// 0xf5bdf4 — j___ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX16OverlayDataModelEEEEENS2_ISsEEEC2ES7_S8_
#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>::list2(boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>)")]
// was: boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>::list2(boost::_bi::value<boost::weak_ptr<RBX::OverlayDataModel>>,boost::_bi::value<std::string>)
pub fn stub_0xf5bdf4(weak: WeakPtr<OverlayDataModel>, text: String) -> OverlayWeakStringList {
    // IDA 0xf5bdf4: __picsymbolstub4 into list2<value<weak>, value<string>>::C2
    // — memberwise move of the weak (shared_count copy) plus the string copy.
    OverlayWeakStringList { weak, text }
}

// 0xf5be04 — j___ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX16OverlayDataModelEEEEENS2_ISsEEEclIPFvS6_SsENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>::operator()<void (*)(rbx_core::Weak<RBX::OverlayDataModel>,std::string),boost::_bi::list0>(boost::_bi::type<void>,void (*)(rbx_core::Weak<RBX::OverlayDataModel>,std::string) &,boost::_bi::list0 &,int)")]
// was: void boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>::operator()<void (*)(boost::weak_ptr<RBX::OverlayDataModel>,std::string),boost::_bi::list0>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::OverlayDataModel>,std::string) &,boost::_bi::list0 &,int)
pub fn stub_0xf5be04(list: &OverlayWeakStringList, invoke: OverlayWeakStringFn) {
    // IDA 0xf5be04: __picsymbolstub4 into
    // list2<value<weak>, value<string>>::operator()<void(*)(weak, string), list0>
    // — forwards the stored (weak, string) to the free function; cloning the
    // weak re-arms the same weak_add_ref the list copy ran.
    invoke(list.weak.clone(), list.text.clone());
}

// 0xf5be14 — j___ZN5boost3_bi5list3INS0_5valueINS_10shared_ptrIN3RBX16OverlayDataModelEEEEENS_3argILi1EEENS8_ILi2EEEEC2ES7_S9_SA_
#[doc(alias = "boost::_bi::list3<boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>::list3(boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>)")]
// was: boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>::list3(boost::_bi::value<boost::shared_ptr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>)
pub fn stub_0xf5be14(shared: SharedPtr<OverlayDataModel>) -> OverlaySharedStrExc {
    // IDA 0xf5be14: __picsymbolstub4 into list3<value<shared>, arg<1>,
    // arg<2>>::C2 — retains the shared owner; the member encoding lands with
    // the `bind` twin 0xf5bf64.
    OverlaySharedStrExc { shared, method: BoundMethod::default(), method_fn: None }
}

// 0xf5be24 — j___ZN5boost3_bi5list3INS0_5valueINS_10shared_ptrIN3RBX16OverlayDataModelEEEEENS_3argILi1EEENS8_ILi2EEEEclINS_4_mfi3mf2IvS5_PKSsPKSt9exceptionEENS0_5list2IRPSsRPSH_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list2<std::string *&,std::exception*&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*> &,boost::_bi::list2<std::string *&,std::exception*&> &,int)")]
// was: void boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list2<std::string *&,std::exception*&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*> &,boost::_bi::list2<std::string *&,std::exception*&> &,int)
pub fn stub_0xf5be24(list: &OverlaySharedStrExc, s: *const String, e: *const ()) {
    // IDA 0xf5be24: __picsymbolstub4 into list3<value<shared>, arg<1>,
    // arg<2>>::operator()<mf2<void, OverlayDataModel, string const*, exception
    // const*>, list2<string*&, exception*&>> — `(shared.get()->*mf)(s, e)`;
    // the late (string, exception) args ride `arg<1>/<2>`.
    if let Some(method) = list.method_fn {
        method(std::sync::Arc::as_ptr(&list.shared) as *mut OverlayDataModel, s, e);
    }
}

// 0xf5be34 — j___ZN5boost3_bi5list3INS0_5valueIPN3RBX16OverlayDataModelEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_PKSsPKSt9exceptionEENS0_5list2IRPSsRPSG_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::OverlayDataModel *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list2<std::string *&,std::exception*&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*> &,boost::_bi::list2<std::string *&,std::exception*&> &,int)")]
pub fn stub_0xf5be34(list: &OverlayRawStrExc, s: *const String, e: *const ()) {
    // IDA 0xf5be34: __picsymbolstub4 into list3<value<OverlayDataModel*>,
    // arg<1>, arg<2>>::operator()<mf2<...>, list2<...>> — same shape as
    // 0xf5be24 over the unretained target.
    if let Some(method) = list.method_fn {
        method(list.target, s, e);
    }
}

// 0xf5be44 — j___ZN5boost3_bi5list4INS0_5valueIPN3RBX16OverlayDataModelEEENS2_IiEENS2_ISsEENS2_INS4_8JoinTypeEEEEC2ES6_S7_S8_SA_
#[doc(alias = "boost::_bi::list4<boost::_bi::value<RBX::OverlayDataModel *>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>>::list4(boost::_bi::value<RBX::OverlayDataModel *>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>)")]
pub fn stub_0xf5be44(
    target: *mut OverlayDataModel,
    id: i32,
    text: String,
    join: OverlayJoinType,
) -> OverlayJoinList {
    // IDA 0xf5be44: __picsymbolstub4 into list4<value<Overlay*>, value<int>,
    // value<string>, value<JoinType>>::C2 — memberwise copy of the four words;
    // the member encoding lands with the `bind` twin 0xf5bf74.
    OverlayJoinList { target, id, text, join, method: BoundMethod::default(), method_fn: None }
}

// 0xf5be54 — j___ZN5boost3_bi5list4INS0_5valueIPN3RBX16OverlayDataModelEEENS2_IiEENS2_ISsEENS2_INS4_8JoinTypeEEEEclINS_4_mfi3mf3IvS4_iSsS9_EENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::OverlayDataModel *>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>>::operator()<boost::_mfi::mf3<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType> &,boost::_bi::list0 &,int)")]
pub fn stub_0xf5be54(list: &OverlayJoinList) {
    // IDA 0xf5be54: __picsymbolstub4 into list4<...>::operator()<mf3<void,
    // OverlayDataModel, int, string, JoinType>, list0> — `(target->*mf)(id,
    // text, join)` over the fully bound list.
    if let Some(method) = list.method_fn {
        method(list.target, list.id, list.text.clone(), list.join);
    }
}

// 0xf5beb4 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX16OverlayDataModelEEENS0_5list1INS0_5valueIPS5_EEEEEclEv
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::OverlayDataModel>,boost::_bi::list1<boost::_bi::value<RBX::OverlayDataModel*>>>::operator()(void)")]
pub fn stub_0xf5beb4(bind: &OverlayNullaryBind) {
    // IDA 0xf5beb4: __picsymbolstub4 into bind_t<void, mf0<void,
    // OverlayDataModel>, list1<value<Overlay*>>>::operator() — `(target->*mf)()`.
    if let Some(method) = bind.method_fn {
        method(bind.target);
    }
}

// 0xf5bed4 — j___ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX16OverlayDataModelEEEEENS_3argILi1EEEEC2ES7_S9_
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>>::storage2(boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>)")]
// was: boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<RBX::OverlayDataModel>>,boost::arg<1>>::storage2(boost::_bi::value<boost::shared_ptr<RBX::OverlayDataModel>>,boost::arg<1>)
pub fn stub_0xf5bed4(shared: SharedPtr<OverlayDataModel>) -> OverlaySharedArgStore {
    // IDA 0xf5bed4: __picsymbolstub4 into storage2<value<shared>,
    // arg<1>>::C2 — retains the shared owner; the `arg<1>` slot binds late.
    OverlaySharedArgStore { shared }
}

// 0xf5bef4 — j___ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX16OverlayDataModelEEEEENS2_ISsEEEC2ES7_S8_
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>)")]
// was: boost::_bi::storage2<boost::_bi::value<boost::weak_ptr<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<boost::weak_ptr<RBX::OverlayDataModel>>,boost::_bi::value<std::string>)
pub fn stub_0xf5bef4(weak: WeakPtr<OverlayDataModel>, text: String) -> OverlayWeakStringList {
    // IDA 0xf5bef4: __picsymbolstub4 into storage2<value<weak>,
    // value<string>>::C2 — same (weak, string) pair as the list2 twin 0xf5bdf4.
    OverlayWeakStringList { weak, text }
}

// 0xf5bf04 — j___ZN5boost3_bi8storage3INS0_5valueINS_10shared_ptrIN3RBX16OverlayDataModelEEEEENS_3argILi1EEENS8_ILi2EEEEC2ES7_S9_SA_
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>::storage3(boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>)")]
// was: boost::_bi::storage3<boost::_bi::value<boost::shared_ptr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>::storage3(boost::_bi::value<boost::shared_ptr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>)
pub fn stub_0xf5bf04(shared: SharedPtr<OverlayDataModel>) -> OverlaySharedStrExc {
    // IDA 0xf5bf04: __picsymbolstub4 into storage3<value<shared>, arg<1>,
    // arg<2>>::C2 — same retained owner as the list3 twin 0xf5be14.
    OverlaySharedStrExc { shared, method: BoundMethod::default(), method_fn: None }
}

// 0xf5bf14 — j___ZN5boost3_bi8storage4INS0_5valueIPN3RBX16OverlayDataModelEEENS2_IiEENS2_ISsEENS2_INS4_8JoinTypeEEEEC2ES6_S7_S8_SA_
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<RBX::OverlayDataModel *>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>>::storage4(boost::_bi::value<RBX::OverlayDataModel *>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>)")]
pub fn stub_0xf5bf14(
    target: *mut OverlayDataModel,
    id: i32,
    text: String,
    join: OverlayJoinType,
) -> OverlayJoinList {
    // IDA 0xf5bf14: __picsymbolstub4 into storage4<value<Overlay*>, value<int>,
    // value<string>, value<JoinType>>::C2 — same four words as the list4 twin
    // 0xf5be44.
    OverlayJoinList { target, id, text, join, method: BoundMethod::default(), method_fn: None }
}

// 0xf5bf64 — j___ZN5boost4bindIvN3RBX16OverlayDataModelEPKSsPKSt9exceptionNS_10shared_ptrIS2_EENS_3argILi1EEENSA_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISF_T0_T1_T2_EENSD_9list_av_3IT3_T4_T5_E4typeEEEMSI_FSF_SJ_SK_ESN_SO_SP_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list_av_3<SharedPtr<RBX::OverlayDataModel>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::OverlayDataModel,std::string const*,std::exception const*,SharedPtr<RBX::OverlayDataModel>,boost::arg<1>,boost::arg<2>>(void (RBX::OverlayDataModel::*)(std::string const*,std::exception const*),SharedPtr<RBX::OverlayDataModel>,boost::arg<1>,boost::arg<2>)")]
// was: boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list_av_3<boost::shared_ptr<RBX::OverlayDataModel>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::OverlayDataModel,std::string const*,std::exception const*,boost::shared_ptr<RBX::OverlayDataModel>,boost::arg<1>,boost::arg<2>>(void (RBX::OverlayDataModel::*)(std::string const*,std::exception const*),boost::shared_ptr<RBX::OverlayDataModel>,boost::arg<1>,boost::arg<2>)
pub fn stub_0xf5bf64(
    method_hi: u32,
    method_lo: u32,
    method_fn: OverlayStrExcMethod,
    shared: SharedPtr<OverlayDataModel>,
) -> OverlaySharedStrExc {
    // IDA 0xf5bf64: __picsymbolstub4 into bind<void, OverlayDataModel, string
    // const*, exception const*, shared_ptr, arg<1>, arg<2>> — member-pointer
    // pair at the BoundMethod encoding (cf. 0xf5bb14) plus the retained owner.
    OverlaySharedStrExc {
        shared,
        method: BoundMethod { raw: ((method_hi as u64) << 32) | method_lo as u64 },
        method_fn: Some(method_fn),
    }
}

// 0xf5bf74 — j___ZN5boost4bindIvN3RBX16OverlayDataModelEiSsNS2_8JoinTypeEPS2_iSsS3_EENS_3_bi6bind_tIT_NS_4_mfi3mf3IS7_T0_T1_T2_T3_EENS5_9list_av_4IT4_T5_T6_T7_E4typeEEEMSA_FS7_SB_SC_SD_ESG_SH_SI_SJ_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType>,boost::_bi::list_av_4<RBX::OverlayDataModel*,int,std::string,RBX::OverlayDataModel::JoinType>::type> boost::bind<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType,RBX::OverlayDataModel*,int,std::string,RBX::OverlayDataModel::JoinType>(void (RBX::OverlayDataModel::*)(int,std::string,RBX::OverlayDataModel::JoinType),RBX::OverlayDataModel*,int,std::string,RBX::OverlayDataModel::JoinType)")]
pub fn stub_0xf5bf74(
    method_hi: u32,
    method_lo: u32,
    method_fn: OverlayJoinMethod,
    target: *mut OverlayDataModel,
    id: i32,
    text: String,
    join: OverlayJoinType,
) -> OverlayJoinList {
    // IDA 0xf5bf74: __picsymbolstub4 into bind<void, OverlayDataModel, int,
    // string, JoinType, Overlay*, int, string, JoinType> — same member-pair
    // encoding as 0xf5bf64 over the fully bound (target, id, text, join).
    OverlayJoinList {
        target,
        id,
        text,
        join,
        method: BoundMethod { raw: ((method_hi as u64) << 32) | method_lo as u64 },
        method_fn: Some(method_fn),
    }
}

// 0xf5bfa4 — j___ZN5boost4bindIvNS_8weak_ptrIN3RBX16OverlayDataModelEEESsS4_SsEENS_3_bi6bind_tIT_PFS7_T0_T1_ENS5_9list_av_2IT2_T3_E4typeEEESB_SD_SE_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::OverlayDataModel>,std::string),boost::_bi::list_av_2<rbx_core::Weak<RBX::OverlayDataModel>,std::string>::type> boost::bind<void,rbx_core::Weak<RBX::OverlayDataModel>,std::string,rbx_core::Weak<RBX::OverlayDataModel>,std::string>(void (*)(rbx_core::Weak<RBX::OverlayDataModel>,std::string),rbx_core::Weak<RBX::OverlayDataModel>,std::string)")]
// was: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::OverlayDataModel>,std::string),boost::_bi::list_av_2<boost::weak_ptr<RBX::OverlayDataModel>,std::string>::type> boost::bind<void,boost::weak_ptr<RBX::OverlayDataModel>,std::string,boost::weak_ptr<RBX::OverlayDataModel>,std::string>(void (*)(boost::weak_ptr<RBX::OverlayDataModel>,std::string),boost::weak_ptr<RBX::OverlayDataModel>,std::string)
pub fn stub_0xf5bfa4(
    weak: WeakPtr<OverlayDataModel>,
    text: String,
    invoke: OverlayWeakStringFn,
) -> OverlayWeakStringBind {
    // IDA 0xf5bfa4: __picsymbolstub4 into bind<void, weak_ptr, string, ...>
    // over the free (weak, string) function — retains the pair plus its invoker.
    OverlayWeakStringBind { args: OverlayWeakStringList { weak, text }, invoke }
}

// 0xf5c014 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX16OverlayDataModelEPKSsPKSt9exceptionEENS3_5list3INS3_5valueINS_10shared_ptrIS8_EEEENS_3argILi1EEENSK_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_0xf5c014(
    src: &OverlaySharedStrExc,
    dst: &mut Option<Box<OverlaySharedStrExc>>,
    op: FunctorOp,
) -> bool {
    // IDA 0xf5c014: __picsymbolstub4 into functor_manager<bind_t<void, mf2,
    // list3<value<shared>, arg<1>, arg<2>>>>::manager (mpl::bool_<false>,
    // heap-only) — clone/move (ops 0/1) box the bind, destroy (op 2) clears
    // the box; check/get (ops 3/4) always match this single-type manager
    // (same arms as the GenericSlotWrapper manager 0x708ab0).
    match op {
        FunctorOp::Clone => {
            *dst = Some(Box::new(src.clone()));
            true
        }
        FunctorOp::Destroy => {
            *dst = None;
            true
        }
        FunctorOp::CheckType | FunctorOp::GetType => true,
    }
}

// 0xf5c024 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX16OverlayDataModelEiSsNS8_8JoinTypeEEENS3_5list4INS3_5valueIPS8_EENSC_IiEENSC_ISsEENSC_IS9_EEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType>,boost::_bi::list4<boost::_bi::value<RBX::OverlayDataModel*>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0xf5c024(
    src: &OverlayJoinList,
    dst: &mut Option<Box<OverlayJoinList>>,
    op: FunctorOp,
) -> bool {
    // IDA 0xf5c024: __picsymbolstub4 into functor_manager<bind_t<void, mf3,
    // list4<value<Overlay*>, value<int>, value<string>, value<JoinType>>>>::manager
    // — same heap-box arms as 0xf5c014.
    match op {
        FunctorOp::Clone => {
            *dst = Some(Box::new(src.clone()));
            true
        }
        FunctorOp::Destroy => {
            *dst = None;
            true
        }
        FunctorOp::CheckType | FunctorOp::GetType => true,
    }
}

// 0xf5c054 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16OverlayDataModelEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_0xf5c054(
    src: &OverlayWeakStringBind,
    dst: &mut Option<Box<OverlayWeakStringBind>>,
    op: FunctorOp,
) -> bool {
    // IDA 0xf5c054: __picsymbolstub4 into functor_manager<bind_t<void,
    // void(*)(weak<Overlay>, string), list2<value<weak>, value<string>>>>::manager
    // — same heap-box arms as 0xf5c014.
    match op {
        FunctorOp::Clone => {
            *dst = Some(Box::new(src.clone()));
            true
        }
        FunctorOp::Destroy => {
            *dst = None;
            true
        }
        FunctorOp::CheckType | FunctorOp::GetType => true,
    }
}

// 0xf5c0b4 — j___ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX16OverlayDataModelEPKSsPKS2_EENS7_5list3INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE
#[doc(alias = "j___ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX16OverlayDataModelEPKSsPKS2_EENS7_5list3INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf5c0b4(slot: &mut StrExcCallback, bind: OverlaySharedStrExc) {
    // IDA 0xf5c0b4: __picsymbolstub4 into function<void(string*,
    // exception*)>::C2 from bind_t<void, mf2, list3<value<shared>, arg<1>,
    // arg<2>>> — installs the bound (shared, member) pair plus the late
    // (string, exception) args, same install shape as the DmVoidCallback C2s
    // (0xf59bc4 et al.).
    slot.bind = Some(StrExcBind::OverlayShared(bind));
}

// 0xf5c0c4 — j___ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvPKSsPKS2_NS_8weak_ptrIN3RBX9DataModelEEENS0_IFvNS_10shared_ptrIKNS_9unordered13unordered_mapISsNSE_10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIS9_SL_EEEEEEEEENS0_IFvSsEEEENS7_5list5INS_3argILi1EEENS13_ILi2EEENS7_5valueINSH_ISF_EEEENS16_ISX_EENS16_ISZ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS1D_EE5valueEEE5valueEiE4typeE
#[doc(alias = "j___ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvPKSsPKS2_NS_8weak_ptrIN3RBX9DataModelEEENS0_IFvNS_10shared_ptrIKNS_9unordered13unordered_mapISsNSE_10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIS9_SL_EEEEEEEEENS0_IFvSsEEEENS7_5list5INS_3argILi1EEENS13_ILi2EEENS7_5valueINSH_ISF_EEEENS16_ISX_EENS16_ISZ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS1D_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf5c0c4(slot: &mut StrExcCallback, bind: DmWeakDoneBind) {
    // IDA 0xf5c0c4: __picsymbolstub4 into function<void(string*,
    // exception*)>::C2 from bind_t<void, void(*)(string const*, exception
    // const*, weak<DataModel>, function<void(shared<map>)>,
    // function<void(string)>), list5<arg<1>, arg<2>, value<weak>, value<mapfn>,
    // value<strfn>>> — installs the DataModel async done-pair; the response
    // map is built from the body by the free function (see
    // StrExcCallback::call).
    slot.bind = Some(StrExcBind::DmWeakDone(bind));
}

// 0xf5c104 — j___ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16OverlayDataModelEEESsENS4_5list2INS4_5valueIS9_EENSD_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE
#[doc(alias = "j___ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16OverlayDataModelEEESsENS4_5list2INS4_5valueIS9_EENSD_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf5c104(
    slot: &mut OverlayVoid0Callback,
    args: OverlayWeakStringList,
    invoke: OverlayWeakStringFn,
) {
    // IDA 0xf5c104: __picsymbolstub4 into function0<void>::C2 from
    // bind_t<void, void(*)(weak<Overlay>, string), list2<...>> — same install
    // as 0xf59bc4 for the Overlay (weak, string) pair.
    slot.bind = Some(OverlayVoid0Bind::WeakString(args, invoke));
}

// 0xf5c144 — j___ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX16OverlayDataModelEiSsNS8_8JoinTypeEEENS3_5list4INS3_5valueIPS8_EENSC_IiEENSC_ISsEENSC_IS9_EEEEEEEEvT_
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType>,boost::_bi::list4<boost::_bi::value<RBX::OverlayDataModel*>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType>,boost::_bi::list4<boost::_bi::value<RBX::OverlayDataModel*>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>>>)")]
pub fn stub_0xf5c144(slot: &mut OverlayVoid0Callback, bind: OverlayJoinList) {
    // IDA 0xf5c144: function0<void>::assign_to over the mf3-join bind —
    // overwrites the slot with the fully bound (target, id, text, join).
    slot.bind = Some(OverlayVoid0Bind::Join(bind));
}

// 0xf5c154 — j___ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16OverlayDataModelEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEEEvT_
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>)")]
// was: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>)
pub fn stub_0xf5c154(
    slot: &mut OverlayVoid0Callback,
    args: OverlayWeakStringList,
    invoke: OverlayWeakStringFn,
) {
    // IDA 0xf5c154: function0<void>::assign_to over the free (weak, string)
    // bind — same overwrite as 0xf5c144 for the Overlay weak/string pair.
    slot.bind = Some(OverlayVoid0Bind::WeakString(args, invoke));
}

// 0xf5c184 — j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX16OverlayDataModelEiSsNS8_8JoinTypeEEENS3_5list4INS3_5valueIPS8_EENSC_IiEENSC_ISsEENSC_IS9_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
#[doc(alias = "j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX16OverlayDataModelEiSsNS8_8JoinTypeEEENS3_5list4INS3_5valueIPS8_EENSC_IiEENSC_ISsEENSC_IS9_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf5c184(slot: &mut OverlayVoid0Callback, bind: OverlayJoinList) {
    // IDA 0xf5c184: __picsymbolstub4 into function0<void>::C2 from the
    // mf3-join bind — same install as the assign_to twin 0xf5c144.
    slot.bind = Some(OverlayVoid0Bind::Join(bind));
}

// 0xf5c194 — j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16OverlayDataModelEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE
#[doc(alias = "j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16OverlayDataModelEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf5c194(
    slot: &mut OverlayVoid0Callback,
    args: OverlayWeakStringList,
    invoke: OverlayWeakStringFn,
) {
    // IDA 0xf5c194: __picsymbolstub4 into function0<void>::C2 from the free
    // (weak, string) bind — same install as the assign_to twin 0xf5c154.
    slot.bind = Some(OverlayVoid0Bind::WeakString(args, invoke));
}

// 0xf5c254 — j___ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX16OverlayDataModelEPKSsPKS2_EENS6_5list3INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEvT_
#[doc(alias = "void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>)")]
// was: void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>)
pub fn stub_0xf5c254(slot: &mut StrExcCallback, bind: OverlaySharedStrExc) {
    // IDA 0xf5c254: function2<void(string*, exception*)>::assign_to over the
    // mf2-shared bind — overwrites the slot with the bound (shared, member)
    // pair plus the late (string, exception) args; same overwrite as 0xf5c144
    // for the function0 slot.
    slot.bind = Some(StrExcBind::OverlayShared(bind));
}

// 0xf5c284 — j___ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX16OverlayDataModelEPKSsPKS2_EENS6_5list3INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE
#[doc(alias = "j___ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX16OverlayDataModelEPKSsPKS2_EENS6_5list3INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf5c284(slot: &mut StrExcCallback, bind: OverlaySharedStrExc) {
    // IDA 0xf5c284: __picsymbolstub4 into function2<void(string*,
    // exception*)>::C2 from the mf2-shared bind — same install as the
    // assign_to twin 0xf5c254 (cf. the C2 twin 0xf5c0b4).
    slot.bind = Some(StrExcBind::OverlayShared(bind));
}

// 0xf5c354 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_16OverlayDataModelES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::OverlayDataModel,RBX::OverlayDataModel>(SharedPtr<RBX::OverlayDataModel> const*,RBX::OverlayDataModel *)const")]
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::OverlayDataModel,RBX::OverlayDataModel>(boost::shared_ptr<RBX::OverlayDataModel> const*,RBX::OverlayDataModel *)const
pub fn stub_0xf5c354(this: *mut OverlayDataModel, owner: &WeakPtr<OverlayDataModel>) {
    // IDA 0xf5c354: enable_shared_from_this::accept_owner over OverlayDataModel
    // — expired check on the incoming weak; a live owner is linked into the
    // model, an expired one leaves it untouched. Same shape as 0x3a69c4.
    // SAFETY: `this` must point to a valid `OverlayDataModel`.
    unsafe {
        if owner.upgrade().is_some() {
            (*this).weak_owner = owner.clone();
        }
    }
}

// 0xf5c364 — j___ZNK5boost4_mfi3mf3IvN3RBX16OverlayDataModelEiSsNS3_8JoinTypeEEclEPS3_iSsS4_
#[doc(alias = "boost::_mfi::mf3<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType>::operator()(RBX::OverlayDataModel*,int,std::string,RBX::OverlayDataModel::JoinType)const")]
pub fn stub_0xf5c364(
    target: *mut OverlayDataModel,
    id: i32,
    text: String,
    join: OverlayJoinType,
    method_fn: OverlayJoinMethod,
) {
    // IDA 0xf5c364: __picsymbolstub4 into mf3<void, OverlayDataModel, int,
    // string, JoinType>::operator() — `(target->*mf)(id, text, join)`; the
    // member encoding rides the `bind` twin 0xf5bf74, same call shape as the
    // list4 operator() 0xf5be54.
    method_fn(target, id, text, join);
}

// 0xf5c394 — j___ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX16OverlayDataModelEiSsNSA_8JoinTypeEEENS5_5list4INS5_5valueIPSA_EENSE_IiEENSE_ISsEENSE_ISB_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType>,boost::_bi::list4<boost::_bi::value<RBX::OverlayDataModel*>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType>,boost::_bi::list4<boost::_bi::value<RBX::OverlayDataModel*>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0xf5c394(src: &OverlayJoinList, dst: &mut Option<Box<OverlayJoinList>>) {
    // IDA 0xf5c394: basic_vtable0<void>::assign_functor over the mf3-join bind
    // (mpl::bool_<false>, heap-only) — boxes the bind into the buffer; the
    // Clone arm of the manager twin 0xf5c024.
    *dst = Some(Box::new(src.clone()));
}

// 0xf5c3a4 — j___ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16OverlayDataModelEEESsENS5_5list2INS5_5valueISA_EENSE_ISsEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_0xf5c3a4(src: &OverlayWeakStringList, dst: &mut Option<Box<OverlayWeakStringList>>) {
    // IDA 0xf5c3a4: basic_vtable0<void>::assign_functor over the free (weak,
    // string) bind — same heap-box as 0xf5c394 for the Overlay weak/string
    // pair (cf. the manager twin 0xf5c054).
    *dst = Some(Box::new(src.clone()));
}

// 0xf5c3e4 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX16OverlayDataModelEiSsNSA_8JoinTypeEEENS5_5list4INS5_5valueIPSA_EENSE_IiEENSE_ISsEENSE_ISB_EEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType>,boost::_bi::list4<boost::_bi::value<RBX::OverlayDataModel*>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType>,boost::_bi::list4<boost::_bi::value<RBX::OverlayDataModel*>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0xf5c3e4(slot: &mut OverlayVoid0Callback, bind: OverlayJoinList) -> bool {
    // IDA 0xf5c3e4: basic_vtable0<void>::assign_to over the mf3-join bind —
    // same copy as 0xf5c144, reporting whether the functor fit the small
    // buffer; the fully bound list always fits, so this always reports success.
    stub_0xf5c144(slot, bind);
    true
}

// 0xf5c3f4 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX16OverlayDataModelEiSsNSA_8JoinTypeEEENS5_5list4INS5_5valueIPSA_EENSE_IiEENSE_ISsEENSE_ISB_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType>,boost::_bi::list4<boost::_bi::value<RBX::OverlayDataModel*>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType>,boost::_bi::list4<boost::_bi::value<RBX::OverlayDataModel*>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0xf5c3f4(slot: &mut OverlayVoid0Callback, bind: OverlayJoinList) -> bool {
    // IDA 0xf5c3f4: the `function_obj_tag` overload of 0xf5c3e4 — identical body.
    stub_0xf5c3e4(slot, bind)
}

// 0xf5c404 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16OverlayDataModelEEESsENS5_5list2INS5_5valueISA_EENSE_ISsEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const
pub fn stub_0xf5c404(
    slot: &mut OverlayVoid0Callback,
    args: OverlayWeakStringList,
    invoke: OverlayWeakStringFn,
) -> bool {
    // IDA 0xf5c404: basic_vtable0<void>::assign_to over the free (weak, string)
    // bind — same copy as 0xf5c154; the pair always fits, so this always
    // reports success.
    stub_0xf5c154(slot, args, invoke);
    true
}

// 0xf5c414 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16OverlayDataModelEEESsENS5_5list2INS5_5valueISA_EENSE_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0xf5c414(
    slot: &mut OverlayVoid0Callback,
    args: OverlayWeakStringList,
    invoke: OverlayWeakStringFn,
) -> bool {
    // IDA 0xf5c414: the `function_obj_tag` overload of 0xf5c404 — identical body.
    stub_0xf5c404(slot, args, invoke)
}

// 0xf5c4a4 — j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX16OverlayDataModelEPKSsPKS4_EENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_0xf5c4a4(src: &OverlaySharedStrExc, dst: &mut Option<Box<OverlaySharedStrExc>>) {
    // IDA 0xf5c4a4: basic_vtable2<void(string*, exception*)>::assign_functor
    // over the mf2-shared bind — same heap-box as 0xf5c394 (cf. the manager
    // twin 0xf5c014).
    *dst = Some(Box::new(src.clone()));
}

// 0xf5c4d4 — j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX16OverlayDataModelEPKSsPKS4_EENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const
pub fn stub_0xf5c4d4(slot: &mut StrExcCallback, bind: OverlaySharedStrExc) -> bool {
    // IDA 0xf5c4d4: basic_vtable2<void(string*, exception*)>::assign_to over
    // the mf2-shared bind — same copy as 0xf5c254; the retained owner always
    // fits, so this always reports success.
    stub_0xf5c254(slot, bind);
    true
}

// 0xf5c4e4 — j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX16OverlayDataModelEPKSsPKS4_EENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0xf5c4e4(slot: &mut StrExcCallback, bind: OverlaySharedStrExc) -> bool {
    // IDA 0xf5c4e4: the `function_obj_tag` overload of 0xf5c4d4 — identical body.
    stub_0xf5c4d4(slot, bind)
}

/// Invoker stored alongside the thread `bind_t` (IDA `0xf5ea84`): the bound
/// `void(*)(SharedPtr<DataModel>)` free function.
pub type DmSharedThreadFn = fn(SharedPtr<DataModel>);
/// Invoker stored alongside the thread `bind_t` (IDA `0xf5ea94`): the bound
/// `void(*)(WeakPtr<DataModel>)` free function.
pub type DmWeakThreadFn = fn(WeakPtr<DataModel>);

// 0xf5ea84 — j___ZN5boost6threadC2INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX9DataModelEEEENS2_5list1INS2_5valueIS7_EEEEEEEET_NS_12disable_if_cIXsr5boost13thread_detail14is_convertibleIRSF_NS_6detail13thread_move_tISF_EEEE5valueEPNS0_5dummyEE4typeE
#[doc(alias = "j___ZN5boost6threadC2INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX9DataModelEEEENS2_5list1INS2_5valueIS7_EEEEEEEET_NS_12disable_if_cIXsr5boost13thread_detail14is_convertibleIRSF_NS_6detail13thread_move_tISF_EEEE5valueEPNS0_5dummyEE4typeE")]
pub fn stub_0xf5ea84(
    invoke: DmSharedThreadFn,
    shared: SharedPtr<DataModel>,
) -> std::thread::JoinHandle<()> {
    // IDA 0xf5ea84: __picsymbolstub4 into thread::C2 from
    // bind<void(*)(shared<DataModel>), list1<value<shared>>> — retains the
    // shared owner into the new thread and invokes it there; cloning the
    // shared re-arms the same shared_count copy the list1 copy ran.
    std::thread::spawn(move || invoke(shared))
}

// 0xf5ea94 — j___ZN5boost6threadC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9DataModelEEEENS2_5list1INS2_5valueIS7_EEEEEEEET_NS_12disable_if_cIXsr5boost13thread_detail14is_convertibleIRSF_NS_6detail13thread_move_tISF_EEEE5valueEPNS0_5dummyEE4typeE
#[doc(alias = "j___ZN5boost6threadC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9DataModelEEEENS2_5list1INS2_5valueIS7_EEEEEEEET_NS_12disable_if_cIXsr5boost13thread_detail14is_convertibleIRSF_NS_6detail13thread_move_tISF_EEEE5valueEPNS0_5dummyEE4typeE")]
pub fn stub_0xf5ea94(
    invoke: DmWeakThreadFn,
    weak: WeakPtr<DataModel>,
) -> std::thread::JoinHandle<()> {
    // IDA 0xf5ea94: __picsymbolstub4 into thread::C2 from
    // bind<void(*)(weak<DataModel>), list1<value<weak>>> — same spawn as
    // 0xf5ea84 for the weak; the entry upgrades (or drops) the weak.
    std::thread::spawn(move || invoke(weak))
}

// 0xf3a614 — j___ZNSt12_Vector_baseIN3RBX17BasicPartInstance14LegacyPartTypeESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::BasicPartInstance::LegacyPartType,std::allocator<RBX::BasicPartInstance::LegacyPartType>>::_M_allocate(unsigned long)")]
pub fn stub_0xf3a614(capacity: usize) -> Vec<LegacyPartType> {
    // IDA 0xf3a614 (`_Vector_base<LegacyPartType>::_M_allocate`): allocates raw
    // storage without constructing any; same safe allocation as 0x45ab98.
    Vec::with_capacity(capacity)
}

// 0xf3a674 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX17BasicPartInstance14LegacyPartTypeES6_EET0_T_S8_S7_
#[doc(alias = "RBX::BasicPartInstance::LegacyPartType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::BasicPartInstance::LegacyPartType *,RBX::BasicPartInstance::LegacyPartType *>(RBX::BasicPartInstance::LegacyPartType *,RBX::BasicPartInstance::LegacyPartType *,RBX::BasicPartInstance::LegacyPartType *)")]
pub fn stub_0xf3a674(items: &mut Vec<LegacyPartType>, first: usize, last: usize, result: usize) {
    // IDA 0xf3a674 (`__copy_backward` over the `LegacyPartType` range): copies
    // `[first, last)` to end at `result`; `copy_within` handles the overlap
    // the same way (the raw pointers collapse into offsets). Same shape as
    // 0x45abb0.
    let len = last.saturating_sub(first);
    items.copy_within(first..last, result.saturating_sub(len));
}

// 0xf3a6d4 — j___ZNSt3mapIPKN3RBX4NameENS0_17BasicPartInstance14LegacyPartTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::BasicPartInstance::LegacyPartType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0xf3a6d4<'a>(map: &'a mut BTreeMap<String, LegacyPartType>, key: &str) -> &'a mut LegacyPartType {
    // IDA 0xf3a6d4 (`map<Name, LegacyPartType>::operator[]`): tree search with
    // a value-initialized (Ball, 0) slot on miss; `entry().or_insert` is the
    // same lookup-or-create (the `Name` key collapses into its text, same
    // shape as 0x45a8e4).
    map.entry(key.to_owned()).or_insert(LegacyPartType::Ball)
}

// 0xf3a794 — j___ZNSt6vectorIN3RBX17BasicPartInstance14LegacyPartTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::BasicPartInstance::LegacyPartType,std::allocator<RBX::BasicPartInstance::LegacyPartType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::BasicPartInstance::LegacyPartType*,std::vector<RBX::BasicPartInstance::LegacyPartType,std::allocator<RBX::BasicPartInstance::LegacyPartType>>>,RBX::BasicPartInstance::LegacyPartType const&)")]
pub fn stub_0xf3a794(items: &mut Vec<LegacyPartType>, index: usize, value: LegacyPartType) {
    // IDA 0xf3a794 (`vector<LegacyPartType>::_M_insert_aux`): shifts the tail
    // and copies the value into the hole; same splice as 0x45aab4 (the index
    // stands in for the iterator position).
    let at = index.min(items.len());
    items.insert(at, value);
}

// 0xf3a7a4 — j___ZNSt6vectorIN3RBX17BasicPartInstance14LegacyPartTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::BasicPartInstance::LegacyPartType,std::allocator<RBX::BasicPartInstance::LegacyPartType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::BasicPartInstance::LegacyPartType*,std::vector<RBX::BasicPartInstance::LegacyPartType,std::allocator<RBX::BasicPartInstance::LegacyPartType>>>,unsigned long,RBX::BasicPartInstance::LegacyPartType const&)")]
pub fn stub_0xf3a7a4(items: &mut Vec<LegacyPartType>, index: usize, count: usize, value: LegacyPartType) {
    // IDA 0xf3a7a4 (`vector<LegacyPartType>::_M_fill_insert`): splices `count`
    // copies of the value at the position; same splice as 0x45abf0.
    let at = index.min(items.len());
    items.splice(at..at, std::iter::repeat(value).take(count));
}

// 0xf3a7b4 — j___ZNSt6vectorIN3RBX17BasicPartInstance14LegacyPartTypeESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::BasicPartInstance::LegacyPartType,std::allocator<RBX::BasicPartInstance::LegacyPartType>>::resize(unsigned long,RBX::BasicPartInstance::LegacyPartType)")]
pub fn stub_0xf3a7b4(items: &mut Vec<LegacyPartType>, len: usize, value: LegacyPartType) {
    // IDA 0xf3a7b4 (`vector<LegacyPartType>::resize`): same shape as 0x45aecc.
    items.resize(len, value);
}

// 0xf3a7c4 — j___ZNSt6vectorIN3RBX17BasicPartInstance14LegacyPartTypeESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::BasicPartInstance::LegacyPartType,std::allocator<RBX::BasicPartInstance::LegacyPartType>>::push_back(RBX::BasicPartInstance::LegacyPartType const&)")]
pub fn stub_0xf3a7c4(items: &mut Vec<LegacyPartType>, value: LegacyPartType) {
    // IDA 0xf3a7c4 (`vector<LegacyPartType>::push_back`): copies the value into
    // the tail; `push` is the same append. Same shape as 0x45a8b8.
    items.push(value);
}

// 0xf3a8f4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17BasicPartInstance14LegacyPartTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType> const&)")]
pub fn stub_0xf3a8f4(map: &mut BTreeMap<String, LegacyPartType>, key: &str, value: LegacyPartType) -> bool {
    // IDA 0xf3a8f4 (`_Rb_tree::_M_insert_unique` by value): search, then link
    // on miss. Same shape as 0x45aa48.
    use std::collections::btree_map::Entry;
    match map.entry(key.to_owned()) {
        Entry::Vacant(slot) => {
            slot.insert(value);
            true
        }
        Entry::Occupied(_) => false,
    }
}

// 0xf3a904 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17BasicPartInstance14LegacyPartTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>,std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType> const&)")]
pub fn stub_0xf3a904(map: &mut BTreeMap<String, LegacyPartType>, key: &str, value: LegacyPartType) -> bool {
    // IDA 0xf3a904 (`_Rb_tree::_M_insert_unique` with the position hint): the
    // hint only seeds the search, so the hinted insert collapses into a plain
    // unique insert. Same shape as 0x45a93c.
    use std::collections::btree_map::Entry;
    match map.entry(key.to_owned()) {
        Entry::Vacant(slot) => {
            slot.insert(value);
            true
        }
        Entry::Occupied(_) => false,
    }
}

// 0xf3a914 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17BasicPartInstance14LegacyPartTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType> const&)")]
pub fn stub_0xf3a914(map: &mut BTreeMap<String, LegacyPartType>, key: &str, value: LegacyPartType) {
    // IDA 0xf3a914 (`_Rb_tree::_M_insert`): links the already-uniqueness-
    // checked node into the tree; after the check the link is a plain insert.
    // Same shape as 0x45a9f0.
    map.insert(key.to_owned(), value);
}

/// Rust model of `rbx::implementation::typed_holder<LegacyPartType>` (IDA
/// `0x4c9868`, target of the `0xf3c8b4` jump stub): the empty per-type
/// holder whose singleton is a function-local static.
#[derive(Clone, Copy, Default)]
pub struct LegacyPartTypeHolder {
    _opaque: (),
}
/// Singleton holder behind `typed_holder<LegacyPartType>::singleton`.
static LEGACY_PART_TYPE_HOLDER: LegacyPartTypeHolder = LegacyPartTypeHolder { _opaque: () };

/// Rust model of `RBX::SeatImpl<BasicPartInstance>` (IDA `0x615bdc` ff.,
/// via the `0xf46974`-`0xf46a34` jump stubs): the seat part link, the
/// disabled flag behind `setDisabled`, the cached seat-weld link behind
/// `findSeatWeld`, and the notify connection torn down by `~SeatImpl`.
pub struct SeatImpl {
    pub part: *const Instance,
    pub disabled: bool,
    pub weld: *const Instance,
    pub conn: Option<PairConnection>,
}

/// Rust model of `boost::_bi::bind_t<void, mf0<void, SeatImpl>, ...>` (IDA
/// `0x616788`, via `0xf46a34`): the bound seat plus the nullary member.
#[derive(Clone, Copy)]
pub struct SeatNotifyBind {
    pub func: fn(*const SeatImpl),
    pub target: *const SeatImpl,
}
// The bound target travels inside signal slots; sound under the
// slot-lifetime contract like `AnimatorBind` (instance.rs).
unsafe impl Send for SeatNotifyBind {}
unsafe impl Sync for SeatNotifyBind {}

/// Rust model of `RBX::PlatformImpl<BasicPartInstance>` (IDA `0x62a0e4`
/// ff., via the `0xf470e4`-`0xf47364` jump stubs): same shape as
/// `SeatImpl` over the platform motor, plus the provider link behind
/// `onServiceProvider`.
pub struct PlatformImpl {
    pub part: *const Instance,
    pub motor: *const Instance,
    pub provider: *const Instance,
    pub conn: Option<PairConnection>,
}

/// Rust model of `boost::_bi::bind_t<void, mf0<void, PlatformImpl>, ...>`
/// (IDA `0x62b93c`, via `0xf47364`).
#[derive(Clone, Copy)]
pub struct PlatformNotifyBind {
    pub func: fn(*const PlatformImpl),
    pub target: *const PlatformImpl,
}
// Same slot-lifetime contract as `SeatNotifyBind`.
unsafe impl Send for PlatformNotifyBind {}
unsafe impl Sync for PlatformNotifyBind {}

/// Rust model of `RBX::ActionStation<BasicPartInstance>` (IDA `0x62eda8`,
/// via `0xf47144`): the part link installed by the ctor.
pub struct ActionStation {
    pub part: *const Instance,
}

// 0xf3c564 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_17BasicPartInstance14LegacyPartTypeEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::BasicPartInstance::LegacyPartType>(RBX::BasicPartInstance::LegacyPartType const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_17BasicPartInstance14LegacyPartTypeEEERS3_RKT_")]
// Jump thunk (`j__` import stub); canonical body lives in `crate::instance::stub_0x4c9818`;
// re-exported so the two addresses cannot drift.
pub use crate::instance::stub_0x4c9818 as stub_0xf3c564;

// 0xf3c8b4 — __ZN3rbx14implementation12typed_holderIN3RBX17BasicPartInstance14LegacyPartTypeEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::BasicPartInstance::LegacyPartType>::singleton(void)")]
pub fn stub_0xf3c8b4() -> &'static LegacyPartTypeHolder {
    // IDA 0xf3c8b4: jump stub into `typed_holder<LegacyPartType>::singleton`
    // (IDA 0x4c9868, export `_DWORD *()`): returns the function-local static
    // holder.
    &LEGACY_PART_TYPE_HOLDER
}

// 0xf3cc74 — __ZN3rbx8any_castIRKN3RBX17BasicPartInstance14LegacyPartTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::BasicPartInstance::LegacyPartType const& rbx::any_cast<RBX::BasicPartInstance::LegacyPartType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0xf3cc74(slot: &EnumSlot) -> LegacyPartType {
    // IDA 0xf3cc74: jump stub into the `LegacyPartType` any_cast (IDA
    // 0x4c99b0): type check against the stored holder, then the payload
    // past the tag. The word model collapses the check into the
    // discriminant range; mismatch is `bad_placement_any_cast` (cf.
    // `instance::stub_0x26ee14`).
    match slot.word {
        0 => LegacyPartType::Ball,
        1 => LegacyPartType::Block,
        2 => LegacyPartType::Cylinder,
        _ => panic!("0xf3cc74: bad_placement_any_cast"),
    }
}

// 0xf3dd24 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17BasicPartInstance14LegacyPartTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>> *)")]
pub fn stub_0xf3dd24(map: &mut BTreeMap<String, LegacyPartType>, key: &str) {
    // IDA 0xf3dd24: jump stub into `_M_erase` by node (IDA 0x4c9b1c):
    // unlinks and frees the node holding the key; `remove` is the same
    // keyed erase (cf. `instance::stub_0x3df534`).
    let _ = map.remove(key);
}


// 0xf46974 — __ZN3RBX8SeatImplINS_17BasicPartInstanceEE11setDisabledERKb
#[doc(alias = "RBX::SeatImpl<RBX::BasicPartInstance>::setDisabled(bool const&)")]
pub fn stub_0xf46974(seat: *mut SeatImpl, value: &bool) {
    // IDA 0xf46974: jump stub into `SeatImpl::setDisabled` (IDA 0x615bdc):
    // stores the flag word.
    // SAFETY: `seat` must point to a valid `SeatImpl`.
    unsafe {
        (*seat).disabled = *value;
    }
}

// 0xf46984 — __ZN3RBX8SeatImplINS_17BasicPartInstanceEE12findSeatWeldEv
#[doc(alias = "RBX::SeatImpl<RBX::BasicPartInstance>::findSeatWeld(void)")]
pub fn stub_0xf46984(seat: *mut SeatImpl) -> *const Instance {
    // IDA 0xf46984: jump stub into `SeatImpl::findSeatWeld` (IDA 0x616644,
    // export `(Instance*) -> ptr`): scans the seat part's children for the
    // seat weld; null when absent. The match is the `Weld` class check
    // (`instance_is_a`, cf. `instance::stub_0x3e11f4`); the cache mirrors
    // the stored link the lookup refreshes.
    // SAFETY: `seat` must point to a valid `SeatImpl` whose part is null or
    // a valid `Instance` outliving the result.
    unsafe {
        let part = (*seat).part;
        if part.is_null() {
            (*seat).weld = core::ptr::null();
            return core::ptr::null();
        }
        for child in &(*part).children {
            let ptr: *const Instance = SharedPtr::as_ptr(child);
            if instance_is_a(ptr, "Weld") {
                (*seat).weld = ptr;
                return ptr;
            }
        }
        (*seat).weld = core::ptr::null();
        core::ptr::null()
    }
}

// 0xf469b4 — __ZN3RBX8SeatImplINS_17BasicPartInstanceEE16humanoidFromWeldEPNS_4WeldE
#[doc(alias = "RBX::SeatImpl<RBX::BasicPartInstance>::humanoidFromWeld(RBX::Weld *)")]
pub fn stub_0xf469b4(weld: *const Instance) -> *const Instance {
    // IDA 0xf469b4: jump stub into `SeatImpl::humanoidFromWeld` (IDA
    // 0x616598, export `(int, JointInstance*) -> ptr`): the weld's part
    // climbs to the parent model, whose first `Humanoid` child is the
    // occupant; null when any link is missing. The joint Part0/Part1 read
    // rides `JointInstance` (instance.rs); the tree walk here is over the
    // modeled parent/children links.
    // SAFETY: `weld` must be null or point to a valid `Instance`.
    humanoid_in_parent_model(weld)
}

// 0xf469c4 — __ZN3RBX8SeatImplINS_17BasicPartInstanceEED2Ev
#[doc(alias = "RBX::SeatImpl<RBX::BasicPartInstance>::~SeatImpl()")]
pub fn stub_0xf469c4(seat: *mut SeatImpl) {
    // IDA 0xf469c4: jump stub into `~SeatImpl` (IDA 0x617ebc, export shows
    // the `connection` teardown): disconnects the notify connection;
    // dropping the handle expires the weak slot (see `PairConnection`).
    // SAFETY: `seat` must point to a valid `SeatImpl`.
    unsafe {
        (*seat).conn = None;
    }
}

// 0xf469f4 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX8SeatImplINSA_17BasicPartInstanceEEEEENS6_5list1INS6_5valueIPSD_EEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>>> const&)")]
pub fn stub_0xf469f4(sig: &Signal<()>, bind: &SeatNotifyBind) -> PairConnection {
    // IDA 0xf469f4: jump stub into the `SeatImpl` nullary `connect` (IDA
    // 0x6165b0): callable slot retaining the bind, slot insert, connection
    // return (cf. `generated_05::stub_0x708c08`).
    let retained = *bind;
    // Whole-struct capture: field-precise capture would grab the raw
    // `target` directly (bypassing the `Send`/`Sync` impls on the bind
    // type); cf. `instance::stub_0x323238`.
    let cb = SharedPtr::new(move |_: ()| {
        let bound = retained;
        (bound.func)(bound.target)
    });
    sig.connect(cb.clone());
    PairConnection { keep: cb }
}
// 0xf46a34 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX8SeatImplINS4_17BasicPartInstanceEEEEENS0_5list1INS0_5valueIPS7_EEEEEclEv
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>>>::operator()(void)")]
pub fn stub_0xf46a34(bind: &SeatNotifyBind) {
    // IDA 0xf46a34: jump stub into the `SeatImpl` bind call (IDA 0x616788):
    // applies the stored mf0 to the bound target (cf.
    // `instance::stub_0x3a5cd4`, nullary here).
    (bind.func)(bind.target);
}

// 0xf470e4 — __ZN3RBX12PlatformImplINS_17BasicPartInstanceEE17onServiceProviderEPNS_15ServiceProviderES4_
#[doc(alias = "RBX::PlatformImpl<RBX::BasicPartInstance>::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
pub fn stub_0xf470e4(this: *mut PlatformImpl, _old: *const Instance, new: *const Instance) {
    // IDA 0xf470e4: jump stub into `PlatformImpl::onServiceProvider` (IDA
    // 0x62a0e4): on a provider switch the motor listener is dropped and the
    // new provider recorded; re-subscribe rides the signal batch.
    // SAFETY: `this` must point to a valid `PlatformImpl`.
    unsafe {
        if (*this).provider != new {
            (*this).conn = None;
            (*this).provider = new;
        }
    }
}

// 0xf470f4 — __ZN3RBX12PlatformImplINS_17BasicPartInstanceEE19findPlatformMotor6DEv
#[doc(alias = "RBX::PlatformImpl<RBX::BasicPartInstance>::findPlatformMotor6D(void)")]
pub fn stub_0xf470f4(this: *mut PlatformImpl) -> *const Instance {
    // IDA 0xf470f4: jump stub into `findPlatformMotor6D` (IDA 0x62a238,
    // export `(Instance*) -> ptr`): scans the platform part's children for
    // the motor joint; null when absent — the `Motor6D` twin of
    // `stub_0xf46984`.
    // SAFETY: `this` must point to a valid `PlatformImpl` whose part is null
    // or a valid `Instance` outliving the result.
    unsafe {
        let part = (*this).part;
        if part.is_null() {
            (*this).motor = core::ptr::null();
            return core::ptr::null();
        }
        for child in &(*part).children {
            let ptr: *const Instance = SharedPtr::as_ptr(child);
            if instance_is_a(ptr, "Motor6D") {
                (*this).motor = ptr;
                return ptr;
            }
        }
        (*this).motor = core::ptr::null();
        core::ptr::null()
    }
}

// 0xf47104 — __ZN3RBX12PlatformImplINS_17BasicPartInstanceEE19humanoidFromMotor6DEPNS_7Motor6DE
#[doc(alias = "RBX::PlatformImpl<RBX::BasicPartInstance>::humanoidFromMotor6D(RBX::Motor6D *)")]
pub fn stub_0xf47104(motor: *const Instance) -> *const Instance {
    // IDA 0xf47104: jump stub into `humanoidFromMotor6D` (IDA 0x62a26c,
    // export `(int, JointInstance*) -> ptr`): same parent-model Humanoid
    // walk as `stub_0xf469b4`.
    // SAFETY: `motor` must be null or point to a valid `Instance`.
    humanoid_in_parent_model(motor)
}

// 0xf47134 — __ZN3RBX12PlatformImplINS_17BasicPartInstanceEED2Ev
#[doc(alias = "RBX::PlatformImpl<RBX::BasicPartInstance>::~PlatformImpl()")]
pub fn stub_0xf47134(this: *mut PlatformImpl) {
    // IDA 0xf47134: jump stub into `~PlatformImpl` (IDA 0x62e324, export
    // shows the `connection` teardown): disconnects the motor listener.
    // SAFETY: `this` must point to a valid `PlatformImpl`.
    unsafe {
        (*this).conn = None;
    }
}

// 0xf47144 — __ZN3RBX13ActionStationINS_17BasicPartInstanceEEC2Ev
#[doc(alias = "RBX::ActionStation<RBX::BasicPartInstance>::ActionStation(void)")]
pub fn stub_0xf47144(part: *const Instance) -> ActionStation {
    // IDA 0xf47144: jump stub into the `ActionStation` ctor (IDA 0x62eda8,
    // export shows the `BasicPartInstance*` param): installs the part link.
    ActionStation { part }
}

// 0xf47154 — __ZN3RBX13ActionStationINS_17BasicPartInstanceEED0Ev
#[doc(alias = "RBX::ActionStation<RBX::BasicPartInstance>::~ActionStation()")]
pub fn stub_0xf47154(station: ActionStation) {
    // IDA 0xf47154: jump stub into the `ActionStation` deleting dtor (IDA
    // 0x62accd): D2 teardown (no members beyond the part link) plus free
    // collapse into drop.
    drop(station);
}

// 0xf472a4 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX12PlatformImplINSA_17BasicPartInstanceEEEEENS6_5list1INS6_5valueIPSD_EEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::PlatformImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::PlatformImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>>> const&)")]
pub fn stub_0xf472a4(sig: &Signal<()>, bind: &PlatformNotifyBind) -> PairConnection {
    // IDA 0xf472a4: jump stub into the `PlatformImpl` nullary `connect`
    // (IDA 0x62b798): same slot-retain + insert + connection shape as
    // `stub_0xf469f4`.
    let retained = *bind;
    // Whole-struct capture (cf. `instance::stub_0x323238`).
    let cb = SharedPtr::new(move |_: ()| {
        let bound = retained;
        (bound.func)(bound.target)
    });
    sig.connect(cb.clone());
    PairConnection { keep: cb }
}
// 0xf47364 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX12PlatformImplINS4_17BasicPartInstanceEEEEENS0_5list1INS0_5valueIPS7_EEEEEclEv
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::PlatformImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>>>::operator()(void)")]
pub fn stub_0xf47364(bind: &PlatformNotifyBind) {
    // IDA 0xf47364: jump stub into the `PlatformImpl` bind call (IDA
    // 0x62b93c): applies the stored mf0 to the bound target.
    (bind.func)(bind.target);
}

/// Shared walk behind `humanoidFromWeld` (IDA `0x616598`) and
/// `humanoidFromMotor6D` (IDA `0x62a26c`): the joint node climbs to the
/// parent model, whose first `Humanoid` child is returned; null when any
/// link is missing.
/// SAFETY: `joint` must be null or point to a valid `Instance`.
fn humanoid_in_parent_model(joint: *const Instance) -> *const Instance {
    unsafe {
        if joint.is_null() {
            return core::ptr::null();
        }
        let model = (*joint).parent;
        if model.is_null() {
            return core::ptr::null();
        }
        for child in &(*model).children {
            let ptr: *const Instance = SharedPtr::as_ptr(child);
            if instance_is_a(ptr, "Humanoid") {
                return ptr;
            }
        }
        core::ptr::null()
    }
}

#[cfg(test)]
mod seat_platform_tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static HITS: AtomicUsize = AtomicUsize::new(0);

    fn hit_seat(_: *const SeatImpl) {
        HITS.fetch_add(1, Ordering::SeqCst);
    }

    fn hit_platform(_: *const PlatformImpl) {
        HITS.fetch_add(1, Ordering::SeqCst);
    }

    fn tree_node(class: &'static str, children: Vec<SharedPtr<Instance>>) -> SharedPtr<Instance> {
        SharedPtr::new(Instance { class_name: class, children, ..Default::default() })
    }

    #[test]
    fn placement_any_reexport_stores_word() {
        let mut slot = EnumSlot::default();
        stub_0xf3c564(&mut slot, crate::instance::LegacyPartTypeTag(2));
        assert_eq!(slot.word, 2);
    }

    #[test]
    fn holder_singleton_is_stable() {
        let a = stub_0xf3c8b4() as *const LegacyPartTypeHolder;
        let b = stub_0xf3c8b4() as *const LegacyPartTypeHolder;
        assert!(!a.is_null());
        assert_eq!(a, b);
    }

    #[test]
    fn any_cast_maps_discriminants() {
        for (word, want) in [
            (0, LegacyPartType::Ball),
            (1, LegacyPartType::Block),
            (2, LegacyPartType::Cylinder),
        ] {
            assert_eq!(stub_0xf3cc74(&EnumSlot { word }), want);
        }
    }

    #[test]
    #[should_panic(expected = "bad_placement_any_cast")]
    fn any_cast_bad_word_panics() {
        let _ = stub_0xf3cc74(&EnumSlot { word: 9 });
    }

    #[test]
    fn erase_removes_inserted_key() {
        let mut map = BTreeMap::new();
        assert!(stub_0xf3a904(&mut map, "Ball", LegacyPartType::Ball));
        stub_0xf3dd24(&mut map, "Ball");
        assert!(!map.contains_key("Ball"));
    }

    #[test]
    fn seat_disabled_stores_flag() {
        let mut seat = SeatImpl {
            part: core::ptr::null(),
            disabled: false,
            weld: core::ptr::null(),
            conn: None,
        };
        stub_0xf46974(&mut seat, &true);
        assert!(seat.disabled);
    }

    #[test]
    fn find_seat_weld_caches_link() {
        let weld = tree_node("Weld", Vec::new());
        let part = tree_node("Seat", vec![SharedPtr::clone(&weld)]);
        let mut seat = SeatImpl {
            part: SharedPtr::as_ptr(&part),
            disabled: false,
            weld: core::ptr::null(),
            conn: None,
        };
        let found = stub_0xf46984(&mut seat);
        assert_eq!(found, SharedPtr::as_ptr(&weld));
        assert_eq!(seat.weld, found);
        let _ = (part, weld);
    }

    #[test]
    fn find_seat_weld_null_part_is_null() {
        let mut seat = SeatImpl {
            part: core::ptr::null(),
            disabled: false,
            weld: core::ptr::null(),
            conn: None,
        };
        assert!(stub_0xf46984(&mut seat).is_null());
    }

    #[test]
    fn humanoid_from_weld_walks_to_model() {
        let humanoid = tree_node("Humanoid", Vec::new());
        let model = tree_node("Model", vec![SharedPtr::clone(&humanoid)]);
        let mut weld_node = Instance { class_name: "Weld", ..Default::default() };
        weld_node.parent = SharedPtr::as_ptr(&model);
        let weld = SharedPtr::new(weld_node);
        assert_eq!(
            stub_0xf469b4(SharedPtr::as_ptr(&weld)),
            SharedPtr::as_ptr(&humanoid)
        );
        assert!(stub_0xf469b4(core::ptr::null()).is_null());
        let _ = (model, humanoid, weld);
    }

    #[test]
    fn seat_connect_fires_and_dtor_clears() {
        HITS.store(0, Ordering::SeqCst);
        let sig = Signal::<()>::new();
        let seat = SeatImpl {
            part: core::ptr::null(),
            disabled: false,
            weld: core::ptr::null(),
            conn: None,
        };
        let seat_ptr: *const SeatImpl = &seat;
        let bind = SeatNotifyBind { func: hit_seat, target: seat_ptr };
        let mut owned = seat;
        owned.conn = Some(stub_0xf469f4(&sig, &bind));
        sig.fire(());
        assert_eq!(HITS.load(Ordering::SeqCst), 1);
        stub_0xf469c4(&mut owned);
        assert!(owned.conn.is_none());
        stub_0xf46a34(&bind);
        assert_eq!(HITS.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn platform_motor_and_humanoid_walks() {
        let humanoid = tree_node("Humanoid", Vec::new());
        let model = tree_node("Model", vec![SharedPtr::clone(&humanoid)]);
        let mut motor_node = Instance { class_name: "Motor6D", ..Default::default() };
        motor_node.parent = SharedPtr::as_ptr(&model);
        let motor = SharedPtr::new(motor_node);
        let part = tree_node("Part", vec![SharedPtr::clone(&motor)]);
        let mut plat = PlatformImpl {
            part: SharedPtr::as_ptr(&part),
            motor: core::ptr::null(),
            provider: core::ptr::null(),
            conn: None,
        };
        assert_eq!(stub_0xf470f4(&mut plat), SharedPtr::as_ptr(&motor));
        assert_eq!(
            stub_0xf47104(SharedPtr::as_ptr(&motor)),
            SharedPtr::as_ptr(&humanoid)
        );
        let _ = (model, humanoid, motor, part);
    }

    #[test]
    fn platform_provider_switch_clears_and_records() {
        HITS.store(0, Ordering::SeqCst);
        let sig = Signal::<()>::new();
        let plat = PlatformImpl {
            part: core::ptr::null(),
            motor: core::ptr::null(),
            provider: core::ptr::null(),
            conn: None,
        };
        let plat_ptr: *const PlatformImpl = &plat;
        let bind = PlatformNotifyBind { func: hit_platform, target: plat_ptr };
        let mut owned = plat;
        owned.conn = Some(stub_0xf472a4(&sig, &bind));
        let next = tree_node("ServiceProvider", Vec::new());
        stub_0xf470e4(&mut owned, core::ptr::null(), SharedPtr::as_ptr(&next));
        assert!(owned.conn.is_none());
        assert_eq!(owned.provider, SharedPtr::as_ptr(&next));
        stub_0xf47134(&mut owned);
        stub_0xf47364(&bind);
        assert_eq!(HITS.load(Ordering::SeqCst), 1);
        let _ = next;
    }

    #[test]
    fn action_station_ctor_stores_part() {
        let part = tree_node("Part", Vec::new());
        let station = stub_0xf47144(SharedPtr::as_ptr(&part));
        assert_eq!(station.part, SharedPtr::as_ptr(&part));
        stub_0xf47154(station);
        let _ = part;
    }
}
