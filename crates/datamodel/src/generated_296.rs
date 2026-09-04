// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: RBX::Instance|DataModel|Workspace strict (60) + RBX::Part|Model|Humanoid extras (28); EA-sorted asc, NOT stubbed in any crate
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 88 stubs | range 0xf59bc4..0xf47364 | strict filter now EXHAUSTED (all 10774 covered)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use rbx_core::WeakPtr;
use crate::data_model::DataModel;
use crate::generated_05::SignatureItem;
use crate::generated_b::BoundMethod;

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

// 0xf5bd74 — j___ZN5boost10shared_ptrIN3RBX16OverlayDataModelEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
#[doc(alias = "SharedPtr<RBX::OverlayDataModel>::shared_ptr<RBX::OverlayDataModel>(rbx_core::Weak<RBX::OverlayDataModel> const&,boost::detail::sp_nothrow_tag)")]
// was: boost::shared_ptr<RBX::OverlayDataModel>::shared_ptr<RBX::OverlayDataModel>(boost::weak_ptr<RBX::OverlayDataModel> const&,boost::detail::sp_nothrow_tag)
pub fn stub_0xf5bd74() -> ! {
    todo!("0xf5bd74 SharedPtr<RBX::OverlayDataModel>::shared_ptr<RBX::OverlayDataModel>(rbx_core::Weak<RBX::OverlayDataModel> const&,boost::detail::sp_nothrow_tag)")
}

// 0xf5bdf4 — j___ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX16OverlayDataModelEEEEENS2_ISsEEEC2ES7_S8_
#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>::list2(boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>)")]
// was: boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>::list2(boost::_bi::value<boost::weak_ptr<RBX::OverlayDataModel>>,boost::_bi::value<std::string>)
pub fn stub_0xf5bdf4() -> ! {
    todo!("0xf5bdf4 boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>::list2(boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>)")
}

// 0xf5be04 — j___ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX16OverlayDataModelEEEEENS2_ISsEEEclIPFvS6_SsENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>::operator()<void (*)(rbx_core::Weak<RBX::OverlayDataModel>,std::string),boost::_bi::list0>(boost::_bi::type<void>,void (*)(rbx_core::Weak<RBX::OverlayDataModel>,std::string) &,boost::_bi::list0 &,int)")]
// was: void boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>::operator()<void (*)(boost::weak_ptr<RBX::OverlayDataModel>,std::string),boost::_bi::list0>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::OverlayDataModel>,std::string) &,boost::_bi::list0 &,int)
pub fn stub_0xf5be04() -> ! {
    todo!("0xf5be04 void boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>::operator()<void (*)(rbx_core::Weak<RBX::OverlayDataModel>,std::string),boost::_bi::list0>(boost::_bi::type<void>,void (*)(rbx_core::Weak<RBX::OverlayDataModel>,std::string) &,boost::_bi::list0 &,int)")
}

// 0xf5be14 — j___ZN5boost3_bi5list3INS0_5valueINS_10shared_ptrIN3RBX16OverlayDataModelEEEEENS_3argILi1EEENS8_ILi2EEEEC2ES7_S9_SA_
#[doc(alias = "boost::_bi::list3<boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>::list3(boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>)")]
// was: boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>::list3(boost::_bi::value<boost::shared_ptr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>)
pub fn stub_0xf5be14() -> ! {
    todo!("0xf5be14 boost::_bi::list3<boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>::list3(boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>)")
}

// 0xf5be24 — j___ZN5boost3_bi5list3INS0_5valueINS_10shared_ptrIN3RBX16OverlayDataModelEEEEENS_3argILi1EEENS8_ILi2EEEEclINS_4_mfi3mf2IvS5_PKSsPKSt9exceptionEENS0_5list2IRPSsRPSH_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list2<std::string *&,std::exception*&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*> &,boost::_bi::list2<std::string *&,std::exception*&> &,int)")]
// was: void boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list2<std::string *&,std::exception*&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*> &,boost::_bi::list2<std::string *&,std::exception*&> &,int)
pub fn stub_0xf5be24() -> ! {
    todo!("0xf5be24 void boost::_bi::list3<boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list2<std::string *&,std::exception*&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*> &,boost::_bi::list2<std::string *&,std::exception*&> &,int)")
}

// 0xf5be34 — j___ZN5boost3_bi5list3INS0_5valueIPN3RBX16OverlayDataModelEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_PKSsPKSt9exceptionEENS0_5list2IRPSsRPSG_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::OverlayDataModel *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list2<std::string *&,std::exception*&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*> &,boost::_bi::list2<std::string *&,std::exception*&> &,int)")]
pub fn stub_0xf5be34() -> ! {
    todo!("0xf5be34 void boost::_bi::list3<boost::_bi::value<RBX::OverlayDataModel *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list2<std::string *&,std::exception*&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*> &,boost::_bi::list2<std::string *&,std::exception*&> &,int)")
}

// 0xf5be44 — j___ZN5boost3_bi5list4INS0_5valueIPN3RBX16OverlayDataModelEEENS2_IiEENS2_ISsEENS2_INS4_8JoinTypeEEEEC2ES6_S7_S8_SA_
#[doc(alias = "boost::_bi::list4<boost::_bi::value<RBX::OverlayDataModel *>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>>::list4(boost::_bi::value<RBX::OverlayDataModel *>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>)")]
pub fn stub_0xf5be44() -> ! {
    todo!("0xf5be44 boost::_bi::list4<boost::_bi::value<RBX::OverlayDataModel *>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>>::list4(boost::_bi::value<RBX::OverlayDataModel *>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>)")
}

// 0xf5be54 — j___ZN5boost3_bi5list4INS0_5valueIPN3RBX16OverlayDataModelEEENS2_IiEENS2_ISsEENS2_INS4_8JoinTypeEEEEclINS_4_mfi3mf3IvS4_iSsS9_EENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::OverlayDataModel *>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>>::operator()<boost::_mfi::mf3<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType> &,boost::_bi::list0 &,int)")]
pub fn stub_0xf5be54() -> ! {
    todo!("0xf5be54 void boost::_bi::list4<boost::_bi::value<RBX::OverlayDataModel *>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>>::operator()<boost::_mfi::mf3<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType> &,boost::_bi::list0 &,int)")
}

// 0xf5beb4 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX16OverlayDataModelEEENS0_5list1INS0_5valueIPS5_EEEEEclEv
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::OverlayDataModel>,boost::_bi::list1<boost::_bi::value<RBX::OverlayDataModel*>>>::operator()(void)")]
pub fn stub_0xf5beb4() -> ! {
    todo!("0xf5beb4 boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::OverlayDataModel>,boost::_bi::list1<boost::_bi::value<RBX::OverlayDataModel*>>>::operator()(void)")
}

// 0xf5bed4 — j___ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX16OverlayDataModelEEEEENS_3argILi1EEEEC2ES7_S9_
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>>::storage2(boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>)")]
// was: boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<RBX::OverlayDataModel>>,boost::arg<1>>::storage2(boost::_bi::value<boost::shared_ptr<RBX::OverlayDataModel>>,boost::arg<1>)
pub fn stub_0xf5bed4() -> ! {
    todo!("0xf5bed4 boost::_bi::storage2<boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>>::storage2(boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>)")
}

// 0xf5bef4 — j___ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX16OverlayDataModelEEEEENS2_ISsEEEC2ES7_S8_
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>)")]
// was: boost::_bi::storage2<boost::_bi::value<boost::weak_ptr<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<boost::weak_ptr<RBX::OverlayDataModel>>,boost::_bi::value<std::string>)
pub fn stub_0xf5bef4() -> ! {
    todo!("0xf5bef4 boost::_bi::storage2<boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>)")
}

// 0xf5bf04 — j___ZN5boost3_bi8storage3INS0_5valueINS_10shared_ptrIN3RBX16OverlayDataModelEEEEENS_3argILi1EEENS8_ILi2EEEEC2ES7_S9_SA_
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>::storage3(boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>)")]
// was: boost::_bi::storage3<boost::_bi::value<boost::shared_ptr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>::storage3(boost::_bi::value<boost::shared_ptr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>)
pub fn stub_0xf5bf04() -> ! {
    todo!("0xf5bf04 boost::_bi::storage3<boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>::storage3(boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>)")
}

// 0xf5bf14 — j___ZN5boost3_bi8storage4INS0_5valueIPN3RBX16OverlayDataModelEEENS2_IiEENS2_ISsEENS2_INS4_8JoinTypeEEEEC2ES6_S7_S8_SA_
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<RBX::OverlayDataModel *>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>>::storage4(boost::_bi::value<RBX::OverlayDataModel *>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>)")]
pub fn stub_0xf5bf14() -> ! {
    todo!("0xf5bf14 boost::_bi::storage4<boost::_bi::value<RBX::OverlayDataModel *>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>>::storage4(boost::_bi::value<RBX::OverlayDataModel *>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>)")
}

// 0xf5bf64 — j___ZN5boost4bindIvN3RBX16OverlayDataModelEPKSsPKSt9exceptionNS_10shared_ptrIS2_EENS_3argILi1EEENSA_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISF_T0_T1_T2_EENSD_9list_av_3IT3_T4_T5_E4typeEEEMSI_FSF_SJ_SK_ESN_SO_SP_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list_av_3<SharedPtr<RBX::OverlayDataModel>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::OverlayDataModel,std::string const*,std::exception const*,SharedPtr<RBX::OverlayDataModel>,boost::arg<1>,boost::arg<2>>(void (RBX::OverlayDataModel::*)(std::string const*,std::exception const*),SharedPtr<RBX::OverlayDataModel>,boost::arg<1>,boost::arg<2>)")]
// was: boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list_av_3<boost::shared_ptr<RBX::OverlayDataModel>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::OverlayDataModel,std::string const*,std::exception const*,boost::shared_ptr<RBX::OverlayDataModel>,boost::arg<1>,boost::arg<2>>(void (RBX::OverlayDataModel::*)(std::string const*,std::exception const*),boost::shared_ptr<RBX::OverlayDataModel>,boost::arg<1>,boost::arg<2>)
pub fn stub_0xf5bf64() -> ! {
    todo!("0xf5bf64 boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list_av_3<SharedPtr<RBX::OverlayDataModel>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::OverlayDataModel,std::string const*,std::exception const*,SharedPtr<RBX::OverlayDataModel>,boost::arg<1>,boost::arg<2>>(void (RBX::OverlayDataModel::*)(std::string const*,std::exception const*),SharedPtr<RBX::OverlayDataModel>,boost::arg<1>,boost::arg<2>)")
}

// 0xf5bf74 — j___ZN5boost4bindIvN3RBX16OverlayDataModelEiSsNS2_8JoinTypeEPS2_iSsS3_EENS_3_bi6bind_tIT_NS_4_mfi3mf3IS7_T0_T1_T2_T3_EENS5_9list_av_4IT4_T5_T6_T7_E4typeEEEMSA_FS7_SB_SC_SD_ESG_SH_SI_SJ_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType>,boost::_bi::list_av_4<RBX::OverlayDataModel*,int,std::string,RBX::OverlayDataModel::JoinType>::type> boost::bind<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType,RBX::OverlayDataModel*,int,std::string,RBX::OverlayDataModel::JoinType>(void (RBX::OverlayDataModel::*)(int,std::string,RBX::OverlayDataModel::JoinType),RBX::OverlayDataModel*,int,std::string,RBX::OverlayDataModel::JoinType)")]
pub fn stub_0xf5bf74() -> ! {
    todo!("0xf5bf74 boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType>,boost::_bi::list_av_4<RBX::OverlayDataModel*,int,std::string,RBX::OverlayDataModel::JoinType>::type> boost::bind<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType,RBX::OverlayDataModel*,int,std::string,RBX::OverlayDataModel::JoinType>(void (RBX::OverlayDataModel::*)(int,std::string,RBX::OverlayDataModel::JoinType),RBX::OverlayDataModel*,int,std::string,RBX::OverlayDataModel::JoinType)")
}

// 0xf5bfa4 — j___ZN5boost4bindIvNS_8weak_ptrIN3RBX16OverlayDataModelEEESsS4_SsEENS_3_bi6bind_tIT_PFS7_T0_T1_ENS5_9list_av_2IT2_T3_E4typeEEESB_SD_SE_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::OverlayDataModel>,std::string),boost::_bi::list_av_2<rbx_core::Weak<RBX::OverlayDataModel>,std::string>::type> boost::bind<void,rbx_core::Weak<RBX::OverlayDataModel>,std::string,rbx_core::Weak<RBX::OverlayDataModel>,std::string>(void (*)(rbx_core::Weak<RBX::OverlayDataModel>,std::string),rbx_core::Weak<RBX::OverlayDataModel>,std::string)")]
// was: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::OverlayDataModel>,std::string),boost::_bi::list_av_2<boost::weak_ptr<RBX::OverlayDataModel>,std::string>::type> boost::bind<void,boost::weak_ptr<RBX::OverlayDataModel>,std::string,boost::weak_ptr<RBX::OverlayDataModel>,std::string>(void (*)(boost::weak_ptr<RBX::OverlayDataModel>,std::string),boost::weak_ptr<RBX::OverlayDataModel>,std::string)
pub fn stub_0xf5bfa4() -> ! {
    todo!("0xf5bfa4 boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::OverlayDataModel>,std::string),boost::_bi::list_av_2<rbx_core::Weak<RBX::OverlayDataModel>,std::string>::type> boost::bind<void,rbx_core::Weak<RBX::OverlayDataModel>,std::string,rbx_core::Weak<RBX::OverlayDataModel>,std::string>(void (*)(rbx_core::Weak<RBX::OverlayDataModel>,std::string),rbx_core::Weak<RBX::OverlayDataModel>,std::string)")
}

// 0xf5c014 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX16OverlayDataModelEPKSsPKSt9exceptionEENS3_5list3INS3_5valueINS_10shared_ptrIS8_EEEENS_3argILi1EEENSK_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_0xf5c014() -> ! {
    todo!("0xf5c014 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0xf5c024 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX16OverlayDataModelEiSsNS8_8JoinTypeEEENS3_5list4INS3_5valueIPS8_EENSC_IiEENSC_ISsEENSC_IS9_EEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType>,boost::_bi::list4<boost::_bi::value<RBX::OverlayDataModel*>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0xf5c024() -> ! {
    todo!("0xf5c024 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType>,boost::_bi::list4<boost::_bi::value<RBX::OverlayDataModel*>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0xf5c054 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16OverlayDataModelEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_0xf5c054() -> ! {
    todo!("0xf5c054 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0xf5c0b4 — j___ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX16OverlayDataModelEPKSsPKS2_EENS7_5list3INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE
#[doc(alias = "j___ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX16OverlayDataModelEPKSsPKS2_EENS7_5list3INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf5c0b4() -> ! {
    todo!("0xf5c0b4 j___ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX16OverlayDataModelEPKSsPKS2_EENS7_5list3INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")
}

// 0xf5c0c4 — j___ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvPKSsPKS2_NS_8weak_ptrIN3RBX9DataModelEEENS0_IFvNS_10shared_ptrIKNS_9unordered13unordered_mapISsNSE_10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIS9_SL_EEEEEEEEENS0_IFvSsEEEENS7_5list5INS_3argILi1EEENS13_ILi2EEENS7_5valueINSH_ISF_EEEENS16_ISX_EENS16_ISZ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS1D_EE5valueEEE5valueEiE4typeE
#[doc(alias = "j___ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvPKSsPKS2_NS_8weak_ptrIN3RBX9DataModelEEENS0_IFvNS_10shared_ptrIKNS_9unordered13unordered_mapISsNSE_10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIS9_SL_EEEEEEEEENS0_IFvSsEEEENS7_5list5INS_3argILi1EEENS13_ILi2EEENS7_5valueINSH_ISF_EEEENS16_ISX_EENS16_ISZ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS1D_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf5c0c4() -> ! {
    todo!("0xf5c0c4 j___ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvPKSsPKS2_NS_8weak_ptrIN3RBX9DataModelEEENS0_IFvNS_10shared_ptrIKNS_9unordered13unordered_mapISsNSE_10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIS9_SL_EEEEEEEEENS0_IFvSsEEEENS7_5list5INS_3argILi1EEENS13_ILi2EEENS7_5valueINSH_ISF_EEEENS16_ISX_EENS16_ISZ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS1D_EE5valueEEE5valueEiE4typeE")
}

// 0xf5c104 — j___ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16OverlayDataModelEEESsENS4_5list2INS4_5valueIS9_EENSD_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE
#[doc(alias = "j___ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16OverlayDataModelEEESsENS4_5list2INS4_5valueIS9_EENSD_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf5c104() -> ! {
    todo!("0xf5c104 j___ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16OverlayDataModelEEESsENS4_5list2INS4_5valueIS9_EENSD_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE")
}

// 0xf5c144 — j___ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX16OverlayDataModelEiSsNS8_8JoinTypeEEENS3_5list4INS3_5valueIPS8_EENSC_IiEENSC_ISsEENSC_IS9_EEEEEEEEvT_
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType>,boost::_bi::list4<boost::_bi::value<RBX::OverlayDataModel*>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType>,boost::_bi::list4<boost::_bi::value<RBX::OverlayDataModel*>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>>>)")]
pub fn stub_0xf5c144() -> ! {
    todo!("0xf5c144 void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType>,boost::_bi::list4<boost::_bi::value<RBX::OverlayDataModel*>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType>,boost::_bi::list4<boost::_bi::value<RBX::OverlayDataModel*>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>>>)")
}

// 0xf5c154 — j___ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16OverlayDataModelEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEEEvT_
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>)")]
// was: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>)
pub fn stub_0xf5c154() -> ! {
    todo!("0xf5c154 void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>)")
}

// 0xf5c184 — j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX16OverlayDataModelEiSsNS8_8JoinTypeEEENS3_5list4INS3_5valueIPS8_EENSC_IiEENSC_ISsEENSC_IS9_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
#[doc(alias = "j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX16OverlayDataModelEiSsNS8_8JoinTypeEEENS3_5list4INS3_5valueIPS8_EENSC_IiEENSC_ISsEENSC_IS9_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf5c184() -> ! {
    todo!("0xf5c184 j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX16OverlayDataModelEiSsNS8_8JoinTypeEEENS3_5list4INS3_5valueIPS8_EENSC_IiEENSC_ISsEENSC_IS9_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")
}

// 0xf5c194 — j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16OverlayDataModelEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE
#[doc(alias = "j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16OverlayDataModelEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf5c194() -> ! {
    todo!("0xf5c194 j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16OverlayDataModelEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE")
}

// 0xf5c254 — j___ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX16OverlayDataModelEPKSsPKS2_EENS6_5list3INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEvT_
#[doc(alias = "void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>)")]
// was: void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>)
pub fn stub_0xf5c254() -> ! {
    todo!("0xf5c254 void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>)")
}

// 0xf5c284 — j___ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX16OverlayDataModelEPKSsPKS2_EENS6_5list3INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE
#[doc(alias = "j___ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX16OverlayDataModelEPKSsPKS2_EENS6_5list3INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf5c284() -> ! {
    todo!("0xf5c284 j___ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX16OverlayDataModelEPKSsPKS2_EENS6_5list3INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")
}

// 0xf5c354 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_16OverlayDataModelES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::OverlayDataModel,RBX::OverlayDataModel>(SharedPtr<RBX::OverlayDataModel> const*,RBX::OverlayDataModel *)const")]
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::OverlayDataModel,RBX::OverlayDataModel>(boost::shared_ptr<RBX::OverlayDataModel> const*,RBX::OverlayDataModel *)const
pub fn stub_0xf5c354() -> ! {
    todo!("0xf5c354 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::OverlayDataModel,RBX::OverlayDataModel>(SharedPtr<RBX::OverlayDataModel> const*,RBX::OverlayDataModel *)const")
}

// 0xf5c364 — j___ZNK5boost4_mfi3mf3IvN3RBX16OverlayDataModelEiSsNS3_8JoinTypeEEclEPS3_iSsS4_
#[doc(alias = "boost::_mfi::mf3<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType>::operator()(RBX::OverlayDataModel*,int,std::string,RBX::OverlayDataModel::JoinType)const")]
pub fn stub_0xf5c364() -> ! {
    todo!("0xf5c364 boost::_mfi::mf3<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType>::operator()(RBX::OverlayDataModel*,int,std::string,RBX::OverlayDataModel::JoinType)const")
}

// 0xf5c394 — j___ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX16OverlayDataModelEiSsNSA_8JoinTypeEEENS5_5list4INS5_5valueIPSA_EENSE_IiEENSE_ISsEENSE_ISB_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType>,boost::_bi::list4<boost::_bi::value<RBX::OverlayDataModel*>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType>,boost::_bi::list4<boost::_bi::value<RBX::OverlayDataModel*>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0xf5c394() -> ! {
    todo!("0xf5c394 void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType>,boost::_bi::list4<boost::_bi::value<RBX::OverlayDataModel*>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType>,boost::_bi::list4<boost::_bi::value<RBX::OverlayDataModel*>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0xf5c3a4 — j___ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16OverlayDataModelEEESsENS5_5list2INS5_5valueISA_EENSE_ISsEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_0xf5c3a4() -> ! {
    todo!("0xf5c3a4 void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0xf5c3e4 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX16OverlayDataModelEiSsNSA_8JoinTypeEEENS5_5list4INS5_5valueIPSA_EENSE_IiEENSE_ISsEENSE_ISB_EEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType>,boost::_bi::list4<boost::_bi::value<RBX::OverlayDataModel*>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType>,boost::_bi::list4<boost::_bi::value<RBX::OverlayDataModel*>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0xf5c3e4() -> ! {
    todo!("0xf5c3e4 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType>,boost::_bi::list4<boost::_bi::value<RBX::OverlayDataModel*>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType>,boost::_bi::list4<boost::_bi::value<RBX::OverlayDataModel*>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>>>,boost::detail::function::function_buffer &)const")
}

// 0xf5c3f4 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX16OverlayDataModelEiSsNSA_8JoinTypeEEENS5_5list4INS5_5valueIPSA_EENSE_IiEENSE_ISsEENSE_ISB_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType>,boost::_bi::list4<boost::_bi::value<RBX::OverlayDataModel*>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType>,boost::_bi::list4<boost::_bi::value<RBX::OverlayDataModel*>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0xf5c3f4() -> ! {
    todo!("0xf5c3f4 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType>,boost::_bi::list4<boost::_bi::value<RBX::OverlayDataModel*>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::OverlayDataModel,int,std::string,RBX::OverlayDataModel::JoinType>,boost::_bi::list4<boost::_bi::value<RBX::OverlayDataModel*>,boost::_bi::value<int>,boost::_bi::value<std::string>,boost::_bi::value<RBX::OverlayDataModel::JoinType>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0xf5c404 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16OverlayDataModelEEESsENS5_5list2INS5_5valueISA_EENSE_ISsEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const
pub fn stub_0xf5c404() -> ! {
    todo!("0xf5c404 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")
}

// 0xf5c414 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16OverlayDataModelEEESsENS5_5list2INS5_5valueISA_EENSE_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0xf5c414() -> ! {
    todo!("0xf5c414 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::OverlayDataModel>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::OverlayDataModel>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0xf5c4a4 — j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX16OverlayDataModelEPKSsPKS4_EENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_0xf5c4a4() -> ! {
    todo!("0xf5c4a4 void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0xf5c4d4 — j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX16OverlayDataModelEPKSsPKS4_EENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const
pub fn stub_0xf5c4d4() -> ! {
    todo!("0xf5c4d4 bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")
}

// 0xf5c4e4 — j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX16OverlayDataModelEPKSsPKS4_EENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0xf5c4e4() -> ! {
    todo!("0xf5c4e4 bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::OverlayDataModel,std::string const*,std::exception const*>,boost::_bi::list3<boost::_bi::value<SharedPtr<RBX::OverlayDataModel>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0xf5ea84 — j___ZN5boost6threadC2INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX9DataModelEEEENS2_5list1INS2_5valueIS7_EEEEEEEET_NS_12disable_if_cIXsr5boost13thread_detail14is_convertibleIRSF_NS_6detail13thread_move_tISF_EEEE5valueEPNS0_5dummyEE4typeE
#[doc(alias = "j___ZN5boost6threadC2INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX9DataModelEEEENS2_5list1INS2_5valueIS7_EEEEEEEET_NS_12disable_if_cIXsr5boost13thread_detail14is_convertibleIRSF_NS_6detail13thread_move_tISF_EEEE5valueEPNS0_5dummyEE4typeE")]
pub fn stub_0xf5ea84() -> ! {
    todo!("0xf5ea84 j___ZN5boost6threadC2INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX9DataModelEEEENS2_5list1INS2_5valueIS7_EEEEEEEET_NS_12disable_if_cIXsr5boost13thread_detail14is_convertibleIRSF_NS_6detail13thread_move_tISF_EEEE5valueEPNS0_5dummyEE4typeE")
}

// 0xf5ea94 — j___ZN5boost6threadC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9DataModelEEEENS2_5list1INS2_5valueIS7_EEEEEEEET_NS_12disable_if_cIXsr5boost13thread_detail14is_convertibleIRSF_NS_6detail13thread_move_tISF_EEEE5valueEPNS0_5dummyEE4typeE
#[doc(alias = "j___ZN5boost6threadC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9DataModelEEEENS2_5list1INS2_5valueIS7_EEEEEEEET_NS_12disable_if_cIXsr5boost13thread_detail14is_convertibleIRSF_NS_6detail13thread_move_tISF_EEEE5valueEPNS0_5dummyEE4typeE")]
pub fn stub_0xf5ea94() -> ! {
    todo!("0xf5ea94 j___ZN5boost6threadC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9DataModelEEEENS2_5list1INS2_5valueIS7_EEEEEEEET_NS_12disable_if_cIXsr5boost13thread_detail14is_convertibleIRSF_NS_6detail13thread_move_tISF_EEEE5valueEPNS0_5dummyEE4typeE")
}

// 0xf3a614 — j___ZNSt12_Vector_baseIN3RBX17BasicPartInstance14LegacyPartTypeESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::BasicPartInstance::LegacyPartType,std::allocator<RBX::BasicPartInstance::LegacyPartType>>::_M_allocate(unsigned long)")]
pub fn stub_0xf3a614() -> ! {
    todo!("0xf3a614 std::_Vector_base<RBX::BasicPartInstance::LegacyPartType,std::allocator<RBX::BasicPartInstance::LegacyPartType>>::_M_allocate(unsigned long)")
}

// 0xf3a674 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX17BasicPartInstance14LegacyPartTypeES6_EET0_T_S8_S7_
#[doc(alias = "RBX::BasicPartInstance::LegacyPartType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::BasicPartInstance::LegacyPartType *,RBX::BasicPartInstance::LegacyPartType *>(RBX::BasicPartInstance::LegacyPartType *,RBX::BasicPartInstance::LegacyPartType *,RBX::BasicPartInstance::LegacyPartType *)")]
pub fn stub_0xf3a674() -> ! {
    todo!("0xf3a674 RBX::BasicPartInstance::LegacyPartType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::BasicPartInstance::LegacyPartType *,RBX::BasicPartInstance::LegacyPartType *>(RBX::BasicPartInstance::LegacyPartType *,RBX::BasicPartInstance::LegacyPartType *,RBX::BasicPartInstance::LegacyPartType *)")
}

// 0xf3a6d4 — j___ZNSt3mapIPKN3RBX4NameENS0_17BasicPartInstance14LegacyPartTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::BasicPartInstance::LegacyPartType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0xf3a6d4() -> ! {
    todo!("0xf3a6d4 std::map<RBX::Name const*,RBX::BasicPartInstance::LegacyPartType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>>::operator[](RBX::Name const* const&)")
}

// 0xf3a794 — j___ZNSt6vectorIN3RBX17BasicPartInstance14LegacyPartTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::BasicPartInstance::LegacyPartType,std::allocator<RBX::BasicPartInstance::LegacyPartType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::BasicPartInstance::LegacyPartType*,std::vector<RBX::BasicPartInstance::LegacyPartType,std::allocator<RBX::BasicPartInstance::LegacyPartType>>>,RBX::BasicPartInstance::LegacyPartType const&)")]
pub fn stub_0xf3a794() -> ! {
    todo!("0xf3a794 std::vector<RBX::BasicPartInstance::LegacyPartType,std::allocator<RBX::BasicPartInstance::LegacyPartType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::BasicPartInstance::LegacyPartType*,std::vector<RBX::BasicPartInstance::LegacyPartType,std::allocator<RBX::BasicPartInstance::LegacyPartType>>>,RBX::BasicPartInstance::LegacyPartType const&)")
}

// 0xf3a7a4 — j___ZNSt6vectorIN3RBX17BasicPartInstance14LegacyPartTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::BasicPartInstance::LegacyPartType,std::allocator<RBX::BasicPartInstance::LegacyPartType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::BasicPartInstance::LegacyPartType*,std::vector<RBX::BasicPartInstance::LegacyPartType,std::allocator<RBX::BasicPartInstance::LegacyPartType>>>,unsigned long,RBX::BasicPartInstance::LegacyPartType const&)")]
pub fn stub_0xf3a7a4() -> ! {
    todo!("0xf3a7a4 std::vector<RBX::BasicPartInstance::LegacyPartType,std::allocator<RBX::BasicPartInstance::LegacyPartType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::BasicPartInstance::LegacyPartType*,std::vector<RBX::BasicPartInstance::LegacyPartType,std::allocator<RBX::BasicPartInstance::LegacyPartType>>>,unsigned long,RBX::BasicPartInstance::LegacyPartType const&)")
}

// 0xf3a7b4 — j___ZNSt6vectorIN3RBX17BasicPartInstance14LegacyPartTypeESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::BasicPartInstance::LegacyPartType,std::allocator<RBX::BasicPartInstance::LegacyPartType>>::resize(unsigned long,RBX::BasicPartInstance::LegacyPartType)")]
pub fn stub_0xf3a7b4() -> ! {
    todo!("0xf3a7b4 std::vector<RBX::BasicPartInstance::LegacyPartType,std::allocator<RBX::BasicPartInstance::LegacyPartType>>::resize(unsigned long,RBX::BasicPartInstance::LegacyPartType)")
}

// 0xf3a7c4 — j___ZNSt6vectorIN3RBX17BasicPartInstance14LegacyPartTypeESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::BasicPartInstance::LegacyPartType,std::allocator<RBX::BasicPartInstance::LegacyPartType>>::push_back(RBX::BasicPartInstance::LegacyPartType const&)")]
pub fn stub_0xf3a7c4() -> ! {
    todo!("0xf3a7c4 std::vector<RBX::BasicPartInstance::LegacyPartType,std::allocator<RBX::BasicPartInstance::LegacyPartType>>::push_back(RBX::BasicPartInstance::LegacyPartType const&)")
}

// 0xf3a8f4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17BasicPartInstance14LegacyPartTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType> const&)")]
pub fn stub_0xf3a8f4() -> ! {
    todo!("0xf3a8f4 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType> const&)")
}

// 0xf3a904 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17BasicPartInstance14LegacyPartTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>,std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType> const&)")]
pub fn stub_0xf3a904() -> ! {
    todo!("0xf3a904 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>,std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType> const&)")
}

// 0xf3a914 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17BasicPartInstance14LegacyPartTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType> const&)")]
pub fn stub_0xf3a914() -> ! {
    todo!("0xf3a914 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType> const&)")
}

// 0xf3c564 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_17BasicPartInstance14LegacyPartTypeEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::BasicPartInstance::LegacyPartType>(RBX::BasicPartInstance::LegacyPartType const&)")]
pub fn stub_0xf3c564() -> ! {
    todo!("0xf3c564 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::BasicPartInstance::LegacyPartType>(RBX::BasicPartInstance::LegacyPartType const&)")
}

// 0xf3c8b4 — j___ZN3rbx14implementation12typed_holderIN3RBX17BasicPartInstance14LegacyPartTypeEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::BasicPartInstance::LegacyPartType>::singleton(void)")]
pub fn stub_0xf3c8b4() -> ! {
    todo!("0xf3c8b4 rbx::implementation::typed_holder<RBX::BasicPartInstance::LegacyPartType>::singleton(void)")
}

// 0xf3cc74 — j___ZN3rbx8any_castIRKN3RBX17BasicPartInstance14LegacyPartTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::BasicPartInstance::LegacyPartType const& rbx::any_cast<RBX::BasicPartInstance::LegacyPartType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0xf3cc74() -> ! {
    todo!("0xf3cc74 RBX::BasicPartInstance::LegacyPartType const& rbx::any_cast<RBX::BasicPartInstance::LegacyPartType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0xf3dd24 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17BasicPartInstance14LegacyPartTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>> *)")]
pub fn stub_0xf3dd24() -> ! {
    todo!("0xf3dd24 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>> *)")
}

// 0xf46974 — j___ZN3RBX8SeatImplINS_17BasicPartInstanceEE11setDisabledERKb
#[doc(alias = "RBX::SeatImpl<RBX::BasicPartInstance>::setDisabled(bool const&)")]
pub fn stub_0xf46974() -> ! {
    todo!("0xf46974 RBX::SeatImpl<RBX::BasicPartInstance>::setDisabled(bool const&)")
}

// 0xf46984 — j___ZN3RBX8SeatImplINS_17BasicPartInstanceEE12findSeatWeldEv
#[doc(alias = "RBX::SeatImpl<RBX::BasicPartInstance>::findSeatWeld(void)")]
pub fn stub_0xf46984() -> ! {
    todo!("0xf46984 RBX::SeatImpl<RBX::BasicPartInstance>::findSeatWeld(void)")
}

// 0xf469b4 — j___ZN3RBX8SeatImplINS_17BasicPartInstanceEE16humanoidFromWeldEPNS_4WeldE
#[doc(alias = "RBX::SeatImpl<RBX::BasicPartInstance>::humanoidFromWeld(RBX::Weld *)")]
pub fn stub_0xf469b4() -> ! {
    todo!("0xf469b4 RBX::SeatImpl<RBX::BasicPartInstance>::humanoidFromWeld(RBX::Weld *)")
}

// 0xf469c4 — j___ZN3RBX8SeatImplINS_17BasicPartInstanceEED2Ev
#[doc(alias = "RBX::SeatImpl<RBX::BasicPartInstance>::~SeatImpl()")]
pub fn stub_0xf469c4() -> ! {
    todo!("0xf469c4 RBX::SeatImpl<RBX::BasicPartInstance>::~SeatImpl()")
}

// 0xf469f4 — j___ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX8SeatImplINSA_17BasicPartInstanceEEEEENS6_5list1INS6_5valueIPSD_EEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>>> const&)")]
pub fn stub_0xf469f4() -> ! {
    todo!("0xf469f4 rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>>> const&)")
}

// 0xf46a34 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX8SeatImplINS4_17BasicPartInstanceEEEEENS0_5list1INS0_5valueIPS7_EEEEEclEv
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>>>::operator()(void)")]
pub fn stub_0xf46a34() -> ! {
    todo!("0xf46a34 boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::BasicPartInstance>*>>>::operator()(void)")
}

// 0xf470e4 — j___ZN3RBX12PlatformImplINS_17BasicPartInstanceEE17onServiceProviderEPNS_15ServiceProviderES4_
#[doc(alias = "RBX::PlatformImpl<RBX::BasicPartInstance>::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
pub fn stub_0xf470e4() -> ! {
    todo!("0xf470e4 RBX::PlatformImpl<RBX::BasicPartInstance>::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")
}

// 0xf470f4 — j___ZN3RBX12PlatformImplINS_17BasicPartInstanceEE19findPlatformMotor6DEv
#[doc(alias = "RBX::PlatformImpl<RBX::BasicPartInstance>::findPlatformMotor6D(void)")]
pub fn stub_0xf470f4() -> ! {
    todo!("0xf470f4 RBX::PlatformImpl<RBX::BasicPartInstance>::findPlatformMotor6D(void)")
}

// 0xf47104 — j___ZN3RBX12PlatformImplINS_17BasicPartInstanceEE19humanoidFromMotor6DEPNS_7Motor6DE
#[doc(alias = "RBX::PlatformImpl<RBX::BasicPartInstance>::humanoidFromMotor6D(RBX::Motor6D *)")]
pub fn stub_0xf47104() -> ! {
    todo!("0xf47104 RBX::PlatformImpl<RBX::BasicPartInstance>::humanoidFromMotor6D(RBX::Motor6D *)")
}

// 0xf47134 — j___ZN3RBX12PlatformImplINS_17BasicPartInstanceEED2Ev
#[doc(alias = "RBX::PlatformImpl<RBX::BasicPartInstance>::~PlatformImpl()")]
pub fn stub_0xf47134() -> ! {
    todo!("0xf47134 RBX::PlatformImpl<RBX::BasicPartInstance>::~PlatformImpl()")
}

// 0xf47144 — j___ZN3RBX13ActionStationINS_17BasicPartInstanceEEC2Ev
#[doc(alias = "RBX::ActionStation<RBX::BasicPartInstance>::ActionStation(void)")]
pub fn stub_0xf47144() -> ! {
    todo!("0xf47144 RBX::ActionStation<RBX::BasicPartInstance>::ActionStation(void)")
}

// 0xf47154 — j___ZN3RBX13ActionStationINS_17BasicPartInstanceEED0Ev
#[doc(alias = "RBX::ActionStation<RBX::BasicPartInstance>::~ActionStation()")]
pub fn stub_0xf47154() -> ! {
    todo!("0xf47154 RBX::ActionStation<RBX::BasicPartInstance>::~ActionStation()")
}

// 0xf472a4 — j___ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX12PlatformImplINSA_17BasicPartInstanceEEEEENS6_5list1INS6_5valueIPSD_EEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::PlatformImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::PlatformImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>>> const&)")]
pub fn stub_0xf472a4() -> ! {
    todo!("0xf472a4 rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::PlatformImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::PlatformImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>>> const&)")
}

// 0xf47364 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX12PlatformImplINS4_17BasicPartInstanceEEEEENS0_5list1INS0_5valueIPS7_EEEEEclEv
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::PlatformImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>>>::operator()(void)")]
pub fn stub_0xf47364() -> ! {
    todo!("0xf47364 boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::PlatformImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>>>::operator()(void)")
}
