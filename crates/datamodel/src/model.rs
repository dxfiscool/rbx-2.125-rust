// Auto-generated skeletons for rbx-datamodel split 2 — Part/Model/Workspace
// Filter: demangled contains RBX::Part|RBX::Model|RBX::Workspace | not in batch 1 | sorted by EA
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr

#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use rbx_core::SharedPtr;
use rbx_core::WeakPtr;
use rbx_core::signal::Signal;
use crate::generated_05::Instance;
use crate::instance::{
    AccoutrementBind, MouseCommand, PartAdornment, PartInstance, PartRefPropDescriptor, PVInstance,
    PVRefExtra, Vector3,
};
use crate::workspace::Workspace;
use std::any::Any;
use std::sync::Arc;

/// Rust model of `RBX::PartDragTool` (IDA `0x2f15ec` family): the
/// `enable_shared_from_this` weak owner behind `shared_from<PartDragTool>`
/// plus the rotate-grab flag behind the cursor-name branch (IDA `0x2f184c`).
/// The C2 stores (IDA `0x2f094c`) are the drag part, the grab point, the
/// workspace link, and the associated host instance (4th ctor arg, whose
/// exact role lands with the drag batch).
pub struct PartDragTool {
    pub weak_owner: WeakPtr<PartDragTool>,
    pub grab_rotate: bool,
    pub drag_part: *const PartInstance,
    pub grab_point: Vector3,
    pub workspace: *mut Workspace,
    pub host: Option<SharedPtr<Instance>>,
}

/// Rust model of `RBX::RunDragger` (IDA `0x2f2bf0`/`0x2f2ff8`): the workspace
/// link (`+40`), the drag-part weak (`+24`), the grab point and the
/// upright-applied flag behind the `turnUpright` tail call. Body/frame/
/// velocity words land with the physics batch.
pub struct RunDragger {
    pub workspace: *const Workspace,
    pub drag_part: WeakPtr<PartInstance>,
    pub hit: Vector3,
    pub upright: bool,
}

/// Rust model of `RBX::BoxSelectCommand` (IDA `0x2f6ff4`): the workspace link
/// (`+16`) plus the selection corners (the `+92` region, empty at
/// construction); the `MouseCommand` base C2 collapses (compiler-managed).
pub struct BoxSelectCommand {
    pub workspace: *const Workspace,
    pub anchor: Option<Vector3>,
    pub current: Option<Vector3>,
}

/// Rust model of `RBX::InterpolatedCFrame::FrameInfo` (IDA `0x3252f8`): one
/// history-ring entry — sample time plus position. Rotation/velocity words
/// land with the CFrame batch.
pub struct FrameSample {
    pub time: f64,
    pub pos: Vector3,
}

/// Rust model of `RBX::InterpolatedCFrame` (IDA `0x3252f8`/`0x325998`): the
/// `circular_buffer<FrameInfo>` history (`+96`), the previous-frame local
/// time and the adaptive rate (`+64` accumulation). The `NUM_HISTORY`
/// capacity assert (IDA `0x3253d0`) rides `Vec` growth.
pub struct InterpolatedCFrame {
    pub samples: Vec<FrameSample>,
    pub local_time: f64,
    pub rate: f64,
}

/// Connection handle returned by the `TouchedSignal::connect<Accoutrement>`
/// instantiation (IDA `0x390270`): owns the slot closure's strong ref so the
/// `Signal`'s weak slot stays live — same discipline as `HeartbeatConnection`.
pub struct TouchedConnection {
    pub keep: SharedPtr<dyn Any + Send + Sync>,
}

/// Rust model of `RBX::RbxRay` (IDA `0x2e67a4`): ray origin + direction behind
/// `getSearchRay`/`intersectRayPlane`; the scratch `Plane` lands here as plain math.
#[derive(Clone, Copy, Default)]
pub struct RbxRay {
    pub origin: Vector3,
    pub direction: Vector3,
}

/// Rust model of `RBX::UIEvent` (IDA `0x2ef364`): the input event behind
/// `getIndicatedPart`; button/modifier words land with the input batch.
#[derive(Default)]
pub struct UIEvent {
    _opaque: (),
}

/// Rust model of `RBX::RootInstance` (IDA `0x2eaea4`): only the workspace link
/// is modeled so far; the world/arbiter words land with the physics batch.
pub struct RootInstance {
    pub workspace: *const Workspace,
}

impl Default for RootInstance {
    fn default() -> Self {
        Self { workspace: core::ptr::null() }
    }
}

/// `RBX::DRAG::JoinType` (IDA `0x2eaea4`, word `+24`): how dragged parts join;
/// stored as the original word until the drag-joint batch enumerates it.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct JoinType(pub i32);

/// Rust model of `RBX::NewNullTool` (IDA `0x2ef130`): the `MouseCommand` base
/// (workspace link) plus the cursor string at `+64` (`"ArrowCursor"`,
/// 0x2ef1ba) and the zeroed flag/words at `+68`/`+18..+20`.
pub struct NewNullTool {
    pub workspace: *const Workspace,
    pub cursor: String,
    pub flag: bool,
}

impl Default for NewNullTool {
    fn default() -> Self {
        Self { workspace: core::ptr::null(), cursor: String::new(), flag: false }
    }
}

/// Mutable drag state behind `RBX::LuaDragger::mouseDown` (IDA `0x2e6070`):
/// the drag word at `+23`, the `jointsIMade`/`runDragger` assert pair
/// (LuaDragger.cpp:95-96), the mouse part (`+34`/`+35`), the weak part list
/// (`+31`), the workspace (`+34`... see `mouseDown`) and grab point (`+36..+38`).
#[derive(Default)]
pub struct LuaDraggerState {
    /// Word `+23`: 0 idle, 1 dragging (`0x2e6222` sets 1; `0x2e613a` throws when 1 or 2).
    pub drag_state: u32,
    /// `jointsIMade` size, asserted 0 on entry (IDA `0x2e6146`, LuaDragger.cpp:95).
    pub joints_made: u32,
    /// Whether `runDragger` is live, asserted null on entry (IDA `0x2e619a`, :96).
    pub has_run_dragger: bool,
    /// Mouse part: px at `+34`, weak at `+35` (IDA `0x2e61f8`-`0x2e6204`).
    pub mouse_part: WeakPtr<PartInstance>,
    /// Weak part list stored by `vector::operator=` (IDA `0x2e61e4`, words `+31`).
    pub mouse_parts: Vec<WeakPtr<PartInstance>>,
    /// Workspace stored at `+34` (IDA `0x2e61f8`).
    pub workspace: *const Workspace,
    /// Grab point stored at `+36..+38` (IDA `0x2e620c`-`0x2e621c`).
    pub hit: Vector3,
}

/// Rust model of `RBX::LuaDragTool` (IDA `0x2e9f84`): the workspace link, the
/// part-local grip from the frame-row dots (stored world-space until the
/// `CoordinateFrame` batch) and the live dragger state from `mouseDown`.
pub struct LuaDragTool {
    pub workspace: *const Workspace,
    pub grip: Vector3,
    pub dragger: LuaDraggerState,
}

/// Rust model of `RBX::MegaDragger` (IDA `0x2eaea4`/`0x2eafd8`): the drag-part
/// weak (`shared_from`, 0x2eaecc), the part list (`pvsToParts` into `+8` at
/// 0x2eaf4e, or the weak-vector copy at 0x2eb052), the active byte at `+20`
/// (`1`, 0x2eaf2e), the join word at `+24` (0x2eaf32), the root at `+28`
/// (0x2eaf38) and the arbiter word at `+32` (0x2eaf44).
pub struct MegaDragger {
    pub drag_part: WeakPtr<PartInstance>,
    pub parts: Vec<WeakPtr<PartInstance>>,
    pub active: bool,
    pub join: JoinType,
    pub root: *const RootInstance,
    /// Unretained arbiter word (`root + 312` then `+ 184`); dangerous.
    pub arbiter: *const (),
}

// 135 stubs in this file | batch range 0x2e6070..0x6d1334 (60 existing + 75 new slice C)

// 0x2e6070 — __ZN3RBX10LuaDragger9mouseDownEN5boost10shared_ptrINS_12PartInstanceEEERKN3G3D7Vector3ESt6vectorINS1_8weak_ptrIS3_EESaISB_EE
#[doc(alias = "RBX::LuaDragger::mouseDown(rbx_core::SharedPtr<RBX::PartInstance>,G3D::Vector3 const&,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>)")]
// was: RBX::LuaDragger::mouseDown(boost::shared_ptr<RBX::PartInstance>,G3D::Vector3 const&,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>)
pub fn stub_0x2e6070(
    state: &mut LuaDraggerState,
    part: &SharedPtr<PartInstance>,
    hit: Vector3,
    extra: Vec<WeakPtr<PartInstance>>,
    part_in_workspace: bool,
) {
    // IDA 0x2e6070: `getWorkspaceIfInWorkspace(_mousePart)` assert (LuaDragger.cpp:87,
    // 0x2e60a2-0x2e6130) plus the `GameBasicSettings` byte write (0x2e6130, collapses —
    // no settings model); `Call to LuaDragger::mouseDown when already dragging` throw when
    // `a1[23] - 1 <= 1` (0x2e613a-0x2e6294); `jointsIMade.size() == 0` (:95, 0x2e6146-0x2e6192)
    // and `runDragger.get() == NULL` (:96, 0x2e619a-0x2e61e0) asserts; weak-vector
    // `operator=` into `+31` (0x2e61e4), mouse-part px/weak into `+34`/`+35`
    // (0x2e61f8-0x2e6204), hit into `+36..+38` (0x2e620c-0x2e621c), `a1[23] = 1` (0x2e6222).
    // boost::shared_ptr/weak_ptr -> SharedPtr/WeakPtr (Arc/Weak).
    assert!(part_in_workspace, "0x2e6070 mouseDown: Workspace::getWorkspaceIfInWorkspace(_mousePart.get())");
    if state.drag_state == 1 || state.drag_state == 2 {
        panic!("0x2e6070 mouseDown: Call to LuaDragger::mouseDown when already dragging");
    }
    assert!(state.joints_made == 0, "0x2e6070 mouseDown: jointsIMade.size() == 0");
    assert!(!state.has_run_dragger, "0x2e6070 mouseDown: runDragger.get() == NULL");
    state.mouse_parts = extra;
    state.mouse_part = Arc::downgrade(part);
    state.hit = hit;
    state.drag_state = 1;
}

// 0x2e67a4 — __ZN3RBX10LuaDragger15getSnapHitPointEPNS_12PartInstanceERKNS_6RbxRayERN3G3D7Vector3E
#[doc(alias = "RBX::LuaDragger::getSnapHitPoint(RBX::PartInstance *,RBX::RbxRay const&,G3D::Vector3 &)")]
// was: RBX::LuaDragger::getSnapHitPoint(RBX::PartInstance *,RBX::RbxRay const&,G3D::Vector3 &)
pub fn stub_0x2e67a4(ray: &RbxRay, out: &mut Vector3) -> bool {
    // IDA 0x2e67a4: `getSearchRay` (0x2e67cc); plane through `unitY` (0x2e6802) and zero
    // (0x2e6808) built at 0x2e6816; `intersectRayPlane` (0x2e682c) decides — miss returns 0
    // (0x2e6828). On hit (0x2e683e): `findWorkspace` (0x2e6840), `hitObjectOrPlane`
    // (0x2e6852) and the `toGrid` snap (0x2e6872-0x2e6880); returns 1 (0x2e689e). The
    // workspace/object/grid stages collapse (no Workspace/DragUtilities model): the
    // horizontal-plane intersection round-trips the snap.
    let denom = ray.direction.y;
    if denom.abs() < f32::EPSILON {
        return false;
    }
    let t = -ray.origin.y / denom;
    if t < 0.0 {
        return false;
    }
    out.x = ray.origin.x + ray.direction.x * t;
    out.y = 0.0;
    out.z = ray.origin.z + ray.direction.z * t;
    true
}

// 0x2e6d94 — __ZN3RBXL7addPartEN5boost10shared_ptrINS_8InstanceEEEPSt6vectorINS0_8weak_ptrINS_12PartInstanceEEESaIS7_EE
#[doc(alias = "RBX::addPart(rbx_core::SharedPtr<RBX::Instance>,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> *)")]
// was: RBX::addPart(boost::shared_ptr<RBX::Instance>,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> *)
pub fn stub_0x2e6d94(
    part: Option<&SharedPtr<PartInstance>>,
    out: &mut Vec<WeakPtr<PartInstance>>,
    in_workspace: bool,
) {
    // IDA 0x2e6d94: `dynamic_pointer_cast<PartInstance, Instance>` (0x2e6db6); null throws
    // `runtime_error("Only Part objects should be passed to a Dragger:MouseDown function")`
    // (0x2e6e52-0x2e6e98); null `getWorkspaceIfInWorkspace` throws `"...in the Workspace..."`
    // (0x2e6df4-0x2e6ef2); else a default weak plus `push_back` (0x2e6dfe-0x2e6e0a, releases
    // at 0x2e6e10-0x2e6e24 collapse into Vec/Arc). boost::exception -> panic with the message.
    let part = part.expect("0x2e6d94 addPart: Only Part objects should be passed to a Dragger:MouseDown function");
    assert!(in_workspace, "0x2e6d94 addPart: Only Part objects in the Workspace should be passed to a Dragger:MouseDown function");
    out.push(Arc::downgrade(part));
}

// 0x2e716c — __ZN5boost20dynamic_pointer_castIN3RBX12PartInstanceENS1_8InstanceEEENS_10shared_ptrIT_EERKNS4_IT0_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::PartInstance> boost::dynamic_pointer_cast<RBX::PartInstance,RBX::Instance>(rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: boost::shared_ptr<RBX::PartInstance> boost::dynamic_pointer_cast<RBX::PartInstance,RBX::Instance>(boost::shared_ptr<RBX::Instance> const&)
pub fn stub_0x2e716c(part: Option<SharedPtr<PartInstance>>) -> Option<SharedPtr<PartInstance>> {
    // IDA 0x2e716c: null `pi_` (0x2e7198) yields the empty `shared_ptr` (0x2e71ac); else
    // `__dynamic_cast` Instance -> PartInstance (0x2e719a) with shared ownership
    // (0x2e71a2). boost::shared_ptr<T> -> rbx_core::SharedPtr<T> (Arc<T>); typed model
    // space resolves the cast at the call site, so Some passes through and None stays empty.
    part
}

// 0x2e71b4 — __ZNSt6vectorIN5boost8weak_ptrIN3RBX12PartInstanceEEESaIS4_EEaSERKS6_
#[doc(alias = "std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>::operator=(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&)")]
// was: std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>::operator=(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&)
pub fn stub_0x2e71b4(dst: &mut Vec<WeakPtr<PartInstance>>, src: &[WeakPtr<PartInstance>]) {
    // IDA 0x2e71b4: `vector<weak_ptr<PartInstance>>::operator=` — self-assign guard, then
    // `_M_allocate_and_copy` when capacity is short, elementwise `weak_ptr` copy-assign
    // and surplus destroy. `std::vector` -> Vec, `boost::weak_ptr` -> Weak; the clone-then-assign
    // keeps the strong exception guarantee the original's temp buffer provides.
    let tmp: Vec<WeakPtr<PartInstance>> = src.to_vec();
    *dst = tmp;
}

// 0x2e7eb4 — __ZNSt6vectorIN5boost8weak_ptrIN3RBX12PartInstanceEEESaIS4_EE20_M_allocate_and_copyIN9__gnu_cxx17__normal_iteratorIPKS4_S6_EEEEPS4_mT_SE_
#[doc(alias = "rbx_core::WeakPtr<RBX::PartInstance>* std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>::_M_allocate_and_copy<__gnu_cxx::__normal_iterator<rbx_core::WeakPtr<RBX::PartInstance> const*,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>>>(unsigned long,__gnu_cxx::__normal_iterator<rbx_core::WeakPtr<RBX::PartInstance> const*,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>>,__gnu_cxx::__normal_iterator<rbx_core::WeakPtr<RBX::PartInstance> const*,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>>)")]
// was: boost::weak_ptr<RBX::PartInstance>* std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>::_M_allocate_and_copy<__gnu_cxx::__normal_iterator<boost::weak_ptr<RBX::PartInstance> const*,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>>>(unsigned long,__gnu_cxx::__normal_iterator<boost::weak_ptr<RBX::PartInstance> const*,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>>,__gnu_cxx::__normal_iterator<boost::weak_ptr<RBX::PartInstance> const*,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>>)
pub fn stub_0x2e7eb4(src: &[WeakPtr<PartInstance>]) -> Vec<WeakPtr<PartInstance>> {
    // IDA 0x2e7eb4: `vector<weak_ptr>::_M_allocate_and_copy` — `_M_allocate` (0x2e7ee4) then
    // the per-element `weak_ptr` copy-construct under the block mutex (loop 0x2e7f08-0x2e7fa8).
    // `std::vector` -> Vec: `to_vec` allocates and clones in one pass.
    src.to_vec()
}

// 0x2e8078 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost8weak_ptrIN3RBX12PartInstanceEEES8_EET0_T_SA_S9_
#[doc(alias = "rbx_core::WeakPtr<RBX::PartInstance> * std::__copy<false,std::random_access_iterator_tag>::copy<rbx_core::WeakPtr<RBX::PartInstance> *,rbx_core::WeakPtr<RBX::PartInstance> *>(rbx_core::WeakPtr<RBX::PartInstance> *,rbx_core::WeakPtr<RBX::PartInstance> *,rbx_core::WeakPtr<RBX::PartInstance> *)")]
// was: boost::weak_ptr<RBX::PartInstance> * std::__copy<false,std::random_access_iterator_tag>::copy<boost::weak_ptr<RBX::PartInstance> *,boost::weak_ptr<RBX::PartInstance> *>(boost::weak_ptr<RBX::PartInstance> *,boost::weak_ptr<RBX::PartInstance> *,boost::weak_ptr<RBX::PartInstance> *)
pub fn stub_0x2e8078(dst: &mut [WeakPtr<PartInstance>], src: &[WeakPtr<PartInstance>]) -> usize {
    // IDA 0x2e8078: `__copy<false, random_access>::copy<weak_ptr*, weak_ptr*>` — the
    // memmove-vectorized loop (0x2e8080 count, 0x2e808a unroll shift, 0x2e80a0-0x2e80c2 body)
    // storing 8-byte weak_ptrs. Elementwise `clone` into the fixed-range overlap collapses it.
    let n = dst.len().min(src.len());
    for (d, s) in dst[..n].iter_mut().zip(src[..n].iter()) {
        *d = s.clone();
    }
    n
}

// 0x2e80d0 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKN5boost8weak_ptrIN3RBX12PartInstanceEEEPS7_EET0_T_SC_SB_
#[doc(alias = "rbx_core::WeakPtr<RBX::PartInstance>* std::__copy<false,std::random_access_iterator_tag>::copy<rbx_core::WeakPtr<RBX::PartInstance> const*,rbx_core::WeakPtr<RBX::PartInstance>*>(rbx_core::WeakPtr<RBX::PartInstance> const*,rbx_core::WeakPtr<RBX::PartInstance> const*,rbx_core::WeakPtr<RBX::PartInstance>*)")]
// was: boost::weak_ptr<RBX::PartInstance>* std::__copy<false,std::random_access_iterator_tag>::copy<boost::weak_ptr<RBX::PartInstance> const*,boost::weak_ptr<RBX::PartInstance>*>(boost::weak_ptr<RBX::PartInstance> const*,boost::weak_ptr<RBX::PartInstance> const*,boost::weak_ptr<RBX::PartInstance>*)
pub fn stub_0x2e80d0(dst: &mut [WeakPtr<PartInstance>], src: &[WeakPtr<PartInstance>]) -> usize {
    // IDA 0x2e80d0: same `__copy` loop as 0x2e8078 for the const-source overload
    // (`copy<const weak_ptr*, weak_ptr*>`, 0x2e80d8-0x2e811a). Constness collapses in slice
    // space; identical elementwise clone.
    stub_0x2e8078(dst, src)
}

// 0x2e9870 — __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEENS2_3_bi6bind_tIvPFvS6_PS9_INS2_8weak_ptrINS4_12PartInstanceEEESaISH_EEENSD_5list2INS2_3argILi1EEENSD_5valueISK_EEEEEEET0_T_SV_SU_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector*<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector*<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>>>> std::for_each<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector*<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector*<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector*<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector*<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>>>>)")]
// was: boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>,std::vector*<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector*<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>>>> std::for_each<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>,std::vector*<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector*<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>,std::vector*<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector*<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>>>>)
pub fn stub_0x2e9870(
    items: &[SharedPtr<Instance>],
    out: &mut Vec<WeakPtr<PartInstance>>,
    f: fn(&SharedPtr<Instance>, &mut Vec<WeakPtr<PartInstance>>),
) {
    // IDA 0x2e9870: `std::for_each` over the `shared_ptr<Instance>` range invoking the
    // `addPart` `bind_t` per element (register save 0x2e987a, per-element `list2::operator()`
    // at 0x2e98b8). `boost::bind`/`list2` -> plain fn; the loop is `Iterator::for_each`.
    items.iter().for_each(|it| f(it, out));
}

// 0x2e98b8 — __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueIPSt6vectorINS_8weak_ptrIN3RBX12PartInstanceEEESaIS9_EEEEEclIPFvNS_10shared_ptrINS7_8InstanceEEESC_ENS0_5list1IRKSI_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> *>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> *),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> *) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
// was: void boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> *>>::operator()<void (*)(boost::shared_ptr<RBX::Instance>,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> *),boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::Instance>,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> *) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)
pub fn stub_0x2e98b8(
    f: fn(&SharedPtr<Instance>, &mut Vec<WeakPtr<PartInstance>>),
    arg: &SharedPtr<Instance>,
    bound: &mut Vec<WeakPtr<PartInstance>>,
) {
    // IDA 0x2e98b8: `list2<arg<1>, value<vector*>>::operator()` — forwards the `list1` head
    // (the `shared_ptr<Instance>`) as arg 1 and the stored `vector*` as arg 2, then tail-calls
    // `addPart`. `boost::bind`/`function` -> `Box<dyn Fn>`/closures; here a plain fn call.
    f(arg, bound);
}

// 0x2e9f80 — __ZN3RBX11LuaDragToolC1EPNS_12PartInstanceERKN3G3D7Vector3ERKSt6vectorIN5boost8weak_ptrIS1_EESaISA_EEPNS_9WorkspaceENS8_10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::LuaDragTool::LuaDragTool(RBX::PartInstance *,G3D::Vector3 const&,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::LuaDragTool::LuaDragTool(RBX::PartInstance *,G3D::Vector3 const&,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)
pub fn stub_0x2e9f80(
    part: &SharedPtr<PartInstance>,
    hit: Vector3,
    extra: Vec<WeakPtr<PartInstance>>,
    workspace: *const Workspace,
    scope: Option<SharedPtr<Instance>>,
) -> LuaDragTool {
    // IDA 0x2e9f80: C1 thunk tail-calling the C2 ctor (decomp is a single tail call).
    stub_0x2e9f84(part, hit, extra, workspace, scope)
}

// 0x2e9f84 — __ZN3RBX11LuaDragToolC2EPNS_12PartInstanceERKN3G3D7Vector3ERKSt6vectorIN5boost8weak_ptrIS1_EESaISA_EEPNS_9WorkspaceENS8_10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::LuaDragTool::LuaDragTool(RBX::PartInstance *,G3D::Vector3 const&,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::LuaDragTool::LuaDragTool(RBX::PartInstance *,G3D::Vector3 const&,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)
pub fn stub_0x2e9f84(
    part: &SharedPtr<PartInstance>,
    hit: Vector3,
    extra: Vec<WeakPtr<PartInstance>>,
    workspace: *const Workspace,
    _scope: Option<SharedPtr<Instance>>,
) -> LuaDragTool {
    // IDA 0x2e9f84: `MouseCommand` base C2 (0x2e9fac), vtable stores (0x2e9fc4-0x2e9fce),
    // words `+16`/`+17` zeroed (0x2e9fde-0x2e9fea), `Instance` weak at `+72` (0x2ea014),
    // `FastLog("LuaDragTool created: %p")`, `LuaDragger` create + shared assign
    // (0x2ea042-0x2ea04e, releases 0x2ea054-0x2ea05a collapse), `shared_from<PartInstance>`
    // (0x2ea06c), part-local grip via the frame-row dots (0x2ea07c-0x2ea10a), weak-vector copy
    // (0x2ea118), `mouseDown` (0x2ea128) and vector dtor (0x2ea132-0x2ea13e collapse).
    // The grip math collapses (no CoordinateFrame model): the world hit round-trips it.
    // Workspace membership was pre-validated by `addPart` (0x2e6df4).
    let mut dragger = LuaDraggerState::default();
    dragger.workspace = workspace;
    stub_0x2e6070(&mut dragger, part, hit, extra, true);
    LuaDragTool { workspace, grip: hit, dragger }
}

// 0x2eaea0 — __ZN3RBX11MegaDraggerC1EPNS_12PartInstanceERKSt6vectorIPNS_10PVInstanceESaIS5_EEPNS_12RootInstanceENS_4DRAG8JoinTypeE
#[doc(alias = "RBX::MegaDragger::MegaDragger(RBX::PartInstance *,std::vector<RBX::PVInstance *,std::allocator<RBX::PVInstance *>> const&,RBX::RootInstance *,RBX::DRAG::JoinType)")]
// was: RBX::MegaDragger::MegaDragger(RBX::PartInstance *,std::vector<RBX::PVInstance *,std::allocator<RBX::PVInstance *>> const&,RBX::RootInstance *,RBX::DRAG::JoinType)
pub fn stub_0x2eaea0(
    part: &SharedPtr<PartInstance>,
    loose: Vec<*const PVInstance>,
    root: *const RootInstance,
    join: JoinType,
) -> MegaDragger {
    // IDA 0x2eaea0: C1 thunk tail-calling the C2 ctor for the `PVInstance*` overload.
    stub_0x2eaea4(part, loose, root, join)
}

// 0x2eaea4 — __ZN3RBX11MegaDraggerC2EPNS_12PartInstanceERKSt6vectorIPNS_10PVInstanceESaIS5_EEPNS_12RootInstanceENS_4DRAG8JoinTypeE
#[doc(alias = "RBX::MegaDragger::MegaDragger(RBX::PartInstance *,std::vector<RBX::PVInstance *,std::allocator<RBX::PVInstance *>> const&,RBX::RootInstance *,RBX::DRAG::JoinType)")]
// was: RBX::MegaDragger::MegaDragger(RBX::PartInstance *,std::vector<RBX::PVInstance *,std::allocator<RBX::PVInstance *>> const&,RBX::RootInstance *,RBX::DRAG::JoinType)
pub fn stub_0x2eaea4(
    part: &SharedPtr<PartInstance>,
    loose: Vec<*const PVInstance>,
    root: *const RootInstance,
    join: JoinType,
) -> MegaDragger {
    // IDA 0x2eaea4: `shared_from<PartInstance>` (0x2eaecc), default weak (0x2eaf02, release
    // 0x2eaf0a-0x2eaf12 collapses), words `+8`/`+12`/`+16` zeroed (0x2eaf20-0x2eaf28), byte
    // `+20 = 1` (0x2eaf2e), join at `+24` (0x2eaf32), root at `+28` (0x2eaf38), arbiter word
    // `*(root + 312) + 184` at `+32` (0x2eaf44) and `pvsToParts(a3 -> +8)` (0x2eaf4e-0x2eaf70).
    // `pvsToParts` collapses (no physics model): raw links are carried as weak-or-null.
    // SAFETY: `root` must be non-null; `loose` entries must outlive the drag.
    assert!(!root.is_null(), "0x2eaea4 MegaDragger: root");
    let parts: Vec<WeakPtr<PartInstance>> = loose.iter().map(|_| Arc::downgrade(part)).collect();
    MegaDragger { drag_part: Arc::downgrade(part), parts, active: true, join, root, arbiter: core::ptr::null() }
}

// 0x2eafd4 — __ZN3RBX11MegaDraggerC1EPNS_12PartInstanceERKSt6vectorIN5boost8weak_ptrIS1_EESaIS6_EEPNS_12RootInstanceENS_4DRAG8JoinTypeE
#[doc(alias = "RBX::MegaDragger::MegaDragger(RBX::PartInstance *,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&,RBX::RootInstance *,RBX::DRAG::JoinType)")]
// was: RBX::MegaDragger::MegaDragger(RBX::PartInstance *,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,RBX::RootInstance *,RBX::DRAG::JoinType)
pub fn stub_0x2eafd4(
    part: &SharedPtr<PartInstance>,
    weak_parts: Vec<WeakPtr<PartInstance>>,
    root: *const RootInstance,
    join: JoinType,
) -> MegaDragger {
    // IDA 0x2eafd4: C1 thunk tail-calling the C2 ctor for the `weak_ptr` overload.
    stub_0x2eafd8(part, weak_parts, root, join)
}

// 0x2eafd8 — __ZN3RBX11MegaDraggerC2EPNS_12PartInstanceERKSt6vectorIN5boost8weak_ptrIS1_EESaIS6_EEPNS_12RootInstanceENS_4DRAG8JoinTypeE
#[doc(alias = "RBX::MegaDragger::MegaDragger(RBX::PartInstance *,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&,RBX::RootInstance *,RBX::DRAG::JoinType)")]
// was: RBX::MegaDragger::MegaDragger(RBX::PartInstance *,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,RBX::RootInstance *,RBX::DRAG::JoinType)
pub fn stub_0x2eafd8(
    part: &SharedPtr<PartInstance>,
    weak_parts: Vec<WeakPtr<PartInstance>>,
    root: *const RootInstance,
    join: JoinType,
) -> MegaDragger {
    // IDA 0x2eafd8: same shape as 0x2eaea4 — `shared_from` (0x2eb000), default weak
    // (0x2eb036-0x2eb044 collapse), except the weak vector is copied into `+8` directly
    // (0x2eb052) instead of `pvsToParts`; byte `+20 = 1` (0x2eb05c), join/root/arbiter
    // words (0x2eb060-0x2eb072).
    // SAFETY: same contract as 0x2eaea4.
    assert!(!root.is_null(), "0x2eafd8 MegaDragger: root");
    MegaDragger { drag_part: Arc::downgrade(part), parts: weak_parts, active: true, join, root, arbiter: core::ptr::null() }
}

// 0x2eee88 — __ZN3RBX8NullToolC1EPNS_9WorkspaceE
#[doc(alias = "RBX::NullTool::NullTool(RBX::Workspace *)")]
// was: RBX::NullTool::NullTool(RBX::Workspace *)
pub fn stub_0x2eee88(workspace: *const Workspace) -> crate::instance::NullTool {
    // IDA 0x2eee88: C1 thunk tail-calling the C2 ctor (single tail call in decomp).
    stub_0x2eee8c(workspace)
}

// 0x2eee8c — __ZN3RBX8NullToolC2EPNS_9WorkspaceE
#[doc(alias = "RBX::NullTool::NullTool(RBX::Workspace *)")]
// was: RBX::NullTool::NullTool(RBX::Workspace *)
pub fn stub_0x2eee8c(workspace: *const Workspace) -> crate::instance::NullTool {
    // IDA 0x2eee8c: `MouseCommand` base C2 (0x2eeeac), vtable stores (0x2eede-0x2eeeea) and
    // `FastLog("NullTool created: %p")` under `MouseCommandLifetime`. Vtables/logs collapse;
    // the workspace link is the modeled state.
    crate::instance::NullTool { workspace }
}

// 0x2ef12c — __ZN3RBX11NewNullToolC1EPNS_9WorkspaceE
#[doc(alias = "RBX::NewNullTool::NewNullTool(RBX::Workspace *)")]
// was: RBX::NewNullTool::NewNullTool(RBX::Workspace *)
pub fn stub_0x2ef12c(workspace: *const Workspace) -> NewNullTool {
    // IDA 0x2ef12c: C1 thunk tail-calling the C2 ctor (single tail call in decomp).
    stub_0x2ef130(workspace)
}

// 0x2ef130 — __ZN3RBX11NewNullToolC2EPNS_9WorkspaceE
#[doc(alias = "RBX::NewNullTool::NewNullTool(RBX::Workspace *)")]
// was: RBX::NewNullTool::NewNullTool(RBX::Workspace *)
pub fn stub_0x2ef130(workspace: *const Workspace) -> NewNullTool {
    // IDA 0x2ef130: `MouseCommand` base C2 (0x2ef150), vtable stores (0x2ef178-0x2ef184),
    // cursor `std::string(+64, "ArrowCursor")` (0x2ef1ba), flag byte `+68 = 0` (0x2ef1c2)
    // and words `+18..+20 = 0` (0x2ef1c8-0x2ef1d0).
    NewNullTool { workspace, cursor: "ArrowCursor".to_string(), flag: false }
}

// 0x2ef364 — __ZN3RBX11NewNullTool16getIndicatedPartERKNS_7UIEventERKbPPNS_12PartInstanceEPbPN3G3D7Vector3E
#[doc(alias = "RBX::NewNullTool::getIndicatedPart(RBX::UIEvent const&,bool const&,RBX::PartInstance **,bool *,G3D::Vector3 *)")]
// was: RBX::NewNullTool::getIndicatedPart(RBX::UIEvent const&,bool const&,RBX::PartInstance **,bool *,G3D::Vector3 *)
pub fn stub_0x2ef364(_tool: &NewNullTool, _evt: &UIEvent) -> (Option<SharedPtr<PartInstance>>, bool) {
    // IDA 0x2ef364: `FilterInvisibleNonColliding` build (0x2ef38c), `DataModel::get` Players
    // (0x2ef3c2), `findLocalPlayer` (0x2ef3ca), `getPartByLocalCharacter` (0x2ef3dc),
    // `shared_from<PartInstance>` (0x2ef3ec), `distanceToCharacter` (0x2ef3f8) and
    // `ClickDetector::isClickable` into `*a5` (0x2ef416-0x2ef420 collapse). No Players /
    // character model exists yet, so the indicated part is absent and nothing is clickable.
    (None, false)
}

// 0x2f0948 — __ZN3RBX12PartDragToolC1EPNS_12PartInstanceERKN3G3D7Vector3EPNS_9WorkspaceEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::PartDragTool::PartDragTool(RBX::PartInstance *,G3D::Vector3 const&,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::PartDragTool::PartDragTool(RBX::PartInstance *,G3D::Vector3 const&,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)
pub fn stub_0x2f0948(
    part: *const PartInstance,
    point: Vector3,
    workspace: *mut Workspace,
    host: Option<SharedPtr<Instance>>,
) -> PartDragTool {
    // IDA 0x2f0948 (C1): delegates to C2 (0x2f094c); the vtable fixup
    // between the two is compiler-owned.
    stub_0x2f094c(part, point, workspace, host)
}

// 0x2f094c — __ZN3RBX12PartDragToolC2EPNS_12PartInstanceERKN3G3D7Vector3EPNS_9WorkspaceEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::PartDragTool::PartDragTool(RBX::PartInstance *,G3D::Vector3 const&,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::PartDragTool::PartDragTool(RBX::PartInstance *,G3D::Vector3 const&,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)
pub fn stub_0x2f094c(
    part: *const PartInstance,
    point: Vector3,
    workspace: *mut Workspace,
    host: Option<SharedPtr<Instance>>,
) -> PartDragTool {
    // IDA 0x2f094c (C2): base `MouseCommand` ctor threading the workspace
    // (0x2f0974), vtable installs, then the member stores — the drag part,
    // the grab point, the workspace link, and the retained host instance.
    // The `enable_shared_from_this` weak arms when the `Creatable::create`
    // wrapper (0x2dbe5c) retains the result, so it starts empty here.
    PartDragTool {
        weak_owner: WeakPtr::new(),
        grab_rotate: false,
        drag_part: part,
        grab_point: point,
        workspace,
        host,
    }
}

// 0x2f0bb8 — __ZN3RBX12PartDragTool11onMouseDownERKNS_7UIEventE
#[doc(alias = "RBX::PartDragTool::onMouseDown(RBX::UIEvent const&)")]
// was: RBX::PartDragTool::onMouseDown(RBX::UIEvent const&)
pub fn stub_0x2f0bb8() -> ! {
    todo!("0x2f0bb8 RBX::PartDragTool::onMouseDown(RBX::UIEvent const&)")
}

// 0x2f0cb0 — __ZN3RBX12PartDragTool11onMouseMoveERKNS_7UIEventE
#[doc(alias = "RBX::PartDragTool::onMouseMove(RBX::UIEvent const&)")]
// was: RBX::PartDragTool::onMouseMove(RBX::UIEvent const&)
pub fn stub_0x2f0cb0() -> ! {
    todo!("0x2f0cb0 RBX::PartDragTool::onMouseMove(RBX::UIEvent const&)")
}

// 0x2f0d60 — __ZN3RBX12PartDragTool12onMouseDeltaERKNS_7UIEventE
#[doc(alias = "RBX::PartDragTool::onMouseDelta(RBX::UIEvent const&)")]
// was: RBX::PartDragTool::onMouseDelta(RBX::UIEvent const&)
pub fn stub_0x2f0d60() -> ! {
    todo!("0x2f0d60 RBX::PartDragTool::onMouseDelta(RBX::UIEvent const&)")
}

// 0x2f0ecc — __ZN3RBX12PartDragTool11onMouseIdleERKNS_7UIEventE
#[doc(alias = "RBX::PartDragTool::onMouseIdle(RBX::UIEvent const&)")]
// was: RBX::PartDragTool::onMouseIdle(RBX::UIEvent const&)
pub fn stub_0x2f0ecc() -> ! {
    todo!("0x2f0ecc RBX::PartDragTool::onMouseIdle(RBX::UIEvent const&)")
}

// 0x2f0f68 — __ZN3RBX12PartDragTool9onMouseUpERKNS_7UIEventE
#[doc(alias = "RBX::PartDragTool::onMouseUp(RBX::UIEvent const&)")]
// was: RBX::PartDragTool::onMouseUp(RBX::UIEvent const&)
pub fn stub_0x2f0f68() -> ! {
    todo!("0x2f0f68 RBX::PartDragTool::onMouseUp(RBX::UIEvent const&)")
}

// 0x2f1134 — __ZN3RBX12PartDragTool9onKeyDownERKNS_7UIEventE
#[doc(alias = "RBX::PartDragTool::onKeyDown(RBX::UIEvent const&)")]
// was: RBX::PartDragTool::onKeyDown(RBX::UIEvent const&)
pub fn stub_0x2f1134() -> ! {
    todo!("0x2f1134 RBX::PartDragTool::onKeyDown(RBX::UIEvent const&)")
}

// 0x2f12c0 — __ZN3RBX12PartDragTool13render3dAdornEPNS_5AdornE
#[doc(alias = "RBX::PartDragTool::render3dAdorn(RBX::Adorn *)")]
// was: RBX::PartDragTool::render3dAdorn(RBX::Adorn *)
pub fn stub_0x2f12c0() -> ! {
    todo!("0x2f12c0 RBX::PartDragTool::render3dAdorn(RBX::Adorn *)")
}

// 0x2f13d0 — __ZThn4_N3RBX12PartDragTool13render3dAdornEPNS_5AdornE
#[doc(alias = "non-virtual thunk toRBX::PartDragTool::render3dAdorn(RBX::Adorn *)")]
// was: non-virtual thunk toRBX::PartDragTool::render3dAdorn(RBX::Adorn *)
pub fn stub_0x2f13d0() -> ! {
    todo!("0x2f13d0 non-virtual thunk toRBX::PartDragTool::render3dAdorn(RBX::Adorn *)")
}

// 0x2f13d8 — __ZN3RBX12PartDragToolD0Ev
#[doc(alias = "RBX::PartDragTool::~PartDragTool()")]
// was: RBX::PartDragTool::~PartDragTool()
pub fn stub_0x2f13d8(tool: PartDragTool) {
    // IDA 0x2f13d8 (D0): D2 plus `operator delete` — `drop` of the owned
    // value is exactly that.
    stub_0x2f1484(tool);
}

// 0x2f1478 — __ZN3RBX12PartDragToolD1Ev
#[doc(alias = "RBX::PartDragTool::~PartDragTool()")]
// was: RBX::PartDragTool::~PartDragTool()
pub fn stub_0x2f1478(tool: PartDragTool) {
    // IDA 0x2f1478 (D1 `~PartDragTool`): runs member destructors
    // (`weak_owner`, `host`, base `MouseCommand`); Rust drops `tool` at
    // scope end — the same sequence.
    drop(tool);
}

// 0x2f147c — __ZThn36_N3RBX12PartDragToolD0Ev
#[doc(alias = "non-virtual thunk toRBX::PartDragTool::~PartDragTool()")]
// was: non-virtual thunk toRBX::PartDragTool::~PartDragTool()
pub fn stub_0x2f147c(tool: PartDragTool) {
    // IDA 0x2f147c (`Thn36` to D0): non-virtual thunk with a
    // compiler-owned `this` adjustment; forwards to the D0 body.
    stub_0x2f13d8(tool);
}

// 0x2f1484 — __ZN3RBX12PartDragToolD2Ev
#[doc(alias = "RBX::PartDragTool::~PartDragTool()")]
// was: RBX::PartDragTool::~PartDragTool()
pub fn stub_0x2f1484(tool: PartDragTool) {
    // IDA 0x2f1484 (D2 `~PartDragTool`): vtable resets are
    // compiler-managed here; member teardown is `drop` — the twin of D1
    // (0x2f1478) for the deleting-destructor path.
    drop(tool);
}

// 0x2f15e4 — __ZThn36_N3RBX12PartDragToolD1Ev
#[doc(alias = "non-virtual thunk toRBX::PartDragTool::~PartDragTool()")]
// was: non-virtual thunk toRBX::PartDragTool::~PartDragTool()
pub fn stub_0x2f15e4(tool: PartDragTool) {
    // IDA 0x2f15e4 (`Thn36` to D1): non-virtual thunk with a
    // compiler-owned `this` adjustment; forwards to the D1 body.
    stub_0x2f1478(tool);
}

// 0x2f15ec — __ZN3RBX11shared_fromINS_12PartDragToolEEEN5boost10shared_ptrIT_EEPS4_
#[doc(alias = "rbx_core::SharedPtr<RBX::PartDragTool> RBX::shared_from<RBX::PartDragTool>(RBX::PartDragTool*)")]
// was: boost::shared_ptr<RBX::PartDragTool> RBX::shared_from<RBX::PartDragTool>(RBX::PartDragTool*)
pub fn stub_0x2f15ec(this: *const PartDragTool) -> Option<SharedPtr<PartDragTool>> {
    // IDA 0x2f15ec: null yields the empty `shared_ptr` (0x2f1638-0x2f16be);
    // otherwise the weak at `+32` is locked under the use-count check
    // (0x2f163c-0x2f169a) and an expired owner throws `bad_weak_ptr`
    // (0x2f1642-0x2f16fe). The spinlock retain collapses into `Arc`.
    // SAFETY: `this` must be null or point to a `PartDragTool` whose weak
    // owner was armed by a live `SharedPtr<PartDragTool>`.
    if this.is_null() {
        return None;
    }
    match unsafe { (*this).weak_owner.upgrade() } {
        Some(owned) => Some(owned),
        None => panic!("0x2f15ec shared_from: bad_weak_ptr"),
    }
}

// 0x2f1830 — __ZNK3RBX12PartDragTool14drawConnectorsEv
#[doc(alias = "RBX::PartDragTool::drawConnectors(void)const")]
// was: RBX::PartDragTool::drawConnectors(void)const
pub fn stub_0x2f1830(_tool: &PartDragTool) -> bool {
    // IDA 0x2f1830: single `MOVS R0, #1` (0x2f1832) — connectors always drawn.
    true
}

// 0x2f1834 — __ZNK3RBX12PartDragTool13getCursorNameEv
#[doc(alias = "RBX::PartDragTool::getCursorName(void)const")]
// was: RBX::PartDragTool::getCursorName(void)const
pub fn stub_0x2f1834(tool: &PartDragTool) -> &'static str {
    // IDA 0x2f1834: the flag at `+88` selects the cursor name (0x2f184c-0x2f185a);
    // the `std::string` copy (0x2f1862) collapses into the borrow.
    if tool.grab_rotate {
        "GrabRotateCursor"
    } else {
        "DragCursor"
    }
}

// 0x2f2bf0 — __ZN3RBX10RunDragger9initLocalEPNS_9WorkspaceEN5boost8weak_ptrINS_12PartInstanceEEERKN3G3D7Vector3E
#[doc(alias = "RBX::RunDragger::initLocal(RBX::Workspace *,rbx_core::WeakPtr<RBX::PartInstance>,G3D::Vector3 const&)")]
// was: RBX::RunDragger::initLocal(RBX::Workspace *,boost::weak_ptr<RBX::PartInstance>,G3D::Vector3 const&)
pub fn stub_0x2f2bf0(
    workspace: *const Workspace,
    part: &WeakPtr<PartInstance>,
    hit: Vector3,
) -> RunDragger {
    // IDA 0x2f2bf0: `ReleaseAssert(!_dragPart.expired())` (0x2f2c4e-0x2f2c6e) plus
    // the `nonNullInWorkspace` assert (0x2f2cc2-0x2f2cde); the workspace store
    // (`+40`, 0x2f2d2e) and drag-part weak store (`+24`, 0x2f2d36-0x2f2d3e);
    // body/frame capture (0x2f2d4c-0x2f2d8c), velocity zeroing
    // (0x2f2d98-0x2f2dae), `inf` bounds with the `-1`/`0` sentinels
    // (0x2f2db6-0x2f2df4) and the `turnUpright` tail call (0x2f2e20) collapse —
    // body/frame/velocity words land with the physics batch. The world-match
    // assert (0x2f2e34-0x2f2e5a) collapses (no World model).
    if part.upgrade().is_none() {
        panic!("0x2f2bf0 initLocal: !_dragPart.expired()");
    }
    RunDragger { workspace, drag_part: part.clone(), hit, upright: true }
}

// 0x2f2f3c — __ZN3RBX10RunDragger11turnUprightEPNS_12PartInstanceE
#[doc(alias = "RBX::RunDragger::turnUpright(RBX::PartInstance *)")]
// was: RBX::RunDragger::turnUpright(RBX::PartInstance *)
pub fn stub_0x2f2f3c(part: *const PartInstance) -> bool {
    // IDA 0x2f2f3c: `ReleaseAssert(part)` (0x2f2f50-0x2f2f82); non-standard parts
    // return early (0x2f2f92-0x2f2f9a); standard parts read the frame (0x2f2fa2),
    // pick the closest object-normal id against `unitY` (0x2f2fba-0x2f2fc0) and,
    // unless already Y-up (`== 1`, 0x2f2fc6), reset rotation to identity through
    // the `+208` setter (0x2f2fc8-0x2f2ff2). Frame/normal math collapses (no
    // CFrame model): the already-upright path, `false` = no realignment.
    // SAFETY: `part` must be non-null.
    if part.is_null() {
        panic!("0x2f2f3c turnUpright: part");
    }
    false
}

// 0x2f2ff8 — __ZN3RBX10RunDragger4initEPNS_9WorkspaceEN5boost8weak_ptrINS_12PartInstanceEEERKN3G3D7Vector3E
#[doc(alias = "RBX::RunDragger::init(RBX::Workspace *,rbx_core::WeakPtr<RBX::PartInstance>,G3D::Vector3 const&)")]
// was: RBX::RunDragger::init(RBX::Workspace *,boost::weak_ptr<RBX::PartInstance>,G3D::Vector3 const&)
pub fn stub_0x2f2ff8(
    workspace: *const Workspace,
    part: &WeakPtr<PartInstance>,
    hit: Vector3,
) -> RunDragger {
    // IDA 0x2f2ff8: same expired/nonNull asserts as `initLocal` (0x2f305a-0x2f3126)
    // and workspace/weak stores (0x2f313a-0x2f314a); additionally the grab point
    // is rotated into part-local space with the frame-row dot products
    // (0x2f316e-0x2f31fe) before the shared frame capture, velocity zeroing,
    // `inf` bounds and `turnUpright` tail (0x2f320a-0x2f32c0). The local-offset
    // math collapses (no CoordinateFrame model); the stored world-space hit
    // round-trips it.
    if part.upgrade().is_none() {
        panic!("0x2f2ff8 init: !_dragPart.expired()");
    }
    RunDragger { workspace, drag_part: part.clone(), hit, upright: true }
}

// 0x2f61c0 — __ZN3RBX13ArrowToolBase9findDecalEPNS_12PartInstanceERKNS_7UIEventE
#[doc(alias = "RBX::ArrowToolBase::findDecal(RBX::PartInstance *,RBX::UIEvent const&)")]
// was: RBX::ArrowToolBase::findDecal(RBX::PartInstance *,RBX::UIEvent const&)
pub fn stub_0x2f61c0(
    children: &[SharedPtr<Instance>],
    surface: i32,
    _face: i32,
) -> Option<SharedPtr<Instance>> {
    // IDA 0x2f61c0: `getSurface` resolves the hit (disasm 0x2f61de); a zero
    // surface returns null (0x2f61e2-0x2f61e4); otherwise the part children are
    // scanned (0x2f6206-0x2f6246) for the first `isA<Decal>` (0x2f622a) whose
    // face word (`+0x74`) matches the hit face (0x2f623a-0x2f623e). The
    // face-word compare collapses — `Instance` models no face word — so the
    // first Decal wins. Same class-name match as 0x392c78.
    if surface == 0 {
        return None;
    }
    children.iter().find(|c| c.class_name == "Decal").cloned()
}

// 0x2f6ff4 — __ZN3RBX16BoxSelectCommandC2EPNS_9WorkspaceE
#[doc(alias = "RBX::BoxSelectCommand::BoxSelectCommand(RBX::Workspace *)")]
// was: RBX::BoxSelectCommand::BoxSelectCommand(RBX::Workspace *)
pub fn stub_0x2f6ff4(workspace: *const Workspace) -> BoxSelectCommand {
    // IDA 0x2f6ff4: `MouseCommand` C2 (decomp 0x2f7016, compiler-managed), vtable
    // installs (0x2f702e-0x2f703a), workspace store (`+16`, 0x2f7042), zeroed
    // words (`+17/+18/+27`, corners `+92..+108`) and the empty-vector begin/end
    // self-pointers (`+25/+26 = +92`, 0x2f707a-0x2f7084). The lifetime `FastLog`
    // collapses. `None` corners are the empty `+92` vector pair.
    BoxSelectCommand { workspace, anchor: None, current: None }
}

// 0x2f79c8 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_16BoxSelectCommandEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
#[doc(alias = "rbx_core::SharedPtr<RBX::BoxSelectCommand> RBX::Creatable<RBX::MouseCommand>::create<RBX::BoxSelectCommand,RBX::Workspace *>(RBX::Workspace *)")]
// was: boost::shared_ptr<RBX::BoxSelectCommand> RBX::Creatable<RBX::MouseCommand>::create<RBX::BoxSelectCommand,RBX::Workspace *>(RBX::Workspace *)
pub fn stub_0x2f79c8(workspace: *const Workspace) -> SharedPtr<BoxSelectCommand> {
    // IDA 0x2f79c8: `operator new(0x70)` (0x2f79fe), the C2 above (0x2f7a24), then
    // the `shared_ptr` with the `Creatable` deleter (0x2f7a32) — the deleter
    // collapses into `Arc`.
    SharedPtr::new(stub_0x2f6ff4(workspace))
}

// 0x3252f8 — __ZN3RBX18InterpolatedCFrame8setValueEPNS_12PartInstanceERKN3G3D15CoordinateFrameERKNS_10RemoteTimeE
#[doc(alias = "RBX::InterpolatedCFrame::setValue(RBX::PartInstance *,G3D::CoordinateFrame const&,RBX::RemoteTime const&)")]
// was: RBX::InterpolatedCFrame::setValue(RBX::PartInstance *,G3D::CoordinateFrame const&,RBX::RemoteTime const&)
pub fn stub_0x3252f8(frame: &mut InterpolatedCFrame, pos: Vector3, remote: f64, now: f64) {
    // IDA 0x3252f8: refresh flag set (0x325306), `notifyMoved` (0x325312),
    // `dt = now - remote` stored (`+80`, 0x32531a-0x32532c). Empty history
    // appends (0x325330-0x325334). Otherwise `remote - latest.time` decides
    // (0x325338-0x325348): stale (`< 0`) samples drop (0x325532 fallthrough);
    // equal overwrites the latest slot in place (0x325360-0x3253ae); newer
    // appends to the ring (0x325468-0x32549c), seeding base fields below two
    // frames (0x3254a4-0x32552a) or accumulating the interval rate above
    // (0x3254a6-0x3254ee). Rotation/ring-capacity words collapse (no CFrame
    // model); the `NUM_HISTORY` assert (0x3253c8-0x32540c) rides `Vec` growth.
    frame.local_time = now;
    match frame.samples.last().map(|s| s.time) {
        Some(t) if remote < t => {}
        Some(t) if remote == t => {
            frame.samples.last_mut().expect("0x3252f8 history").pos = pos;
        }
        _ => {
            if let Some(prev) = frame.samples.last() {
                frame.rate += remote - prev.time;
            }
            frame.samples.push(FrameSample { time: remote, pos });
        }
    }
}

// 0x325998 — __ZN3RBX18InterpolatedCFrame12computeValueEPNS_12PartInstanceE
#[doc(alias = "RBX::InterpolatedCFrame::computeValue(RBX::PartInstance *)")]
// was: RBX::InterpolatedCFrame::computeValue(RBX::PartInstance *)
pub fn stub_0x325998(frame: &mut InterpolatedCFrame, now: f64) -> Vector3 {
    // IDA 0x325998: `ReleaseAssert(now >= prevFrame.localTime)` (0x3259ca-0x325a18),
    // refresh flag cleared (0x325a1c). With 2+ samples (0x325a24) the ring is
    // scanned for the first sample at/after the sample-target time
    // (0x325a46-0x325a78; the target-time op, 0x325a2c, collapses — no
    // RemoteTime model, so `now` is the target): none → rate decays `*= 0.9`,
    // `prevLocal = now` (0x325a7e-0x32598); first-sample hit → rate `*= 0.5`
    // (0x325ab6); later hit below `1000.0` → rate `*= 1.1` (0x325ab0-0x325abe);
    // then `interpolate` runs (0x325ac2). The frame copy-out (0x325ad0-0x325ae0)
    // collapses into the returned position.
    assert!(
        now >= frame.local_time,
        "0x325998 computeValue: now >= prevFrame.localTime"
    );
    if frame.samples.len() > 1 {
        match frame.samples.iter().position(|s| s.time >= now) {
            None => {
                frame.rate *= 0.9;
                frame.local_time = now;
            }
            Some(idx) => {
                if idx == 0 {
                    frame.rate *= 0.5;
                } else {
                    if frame.rate < 1000.0 {
                        frame.rate *= 1.1;
                    }
                    let a = &frame.samples[idx - 1];
                    let b = &frame.samples[idx];
                    let span = (b.time - a.time).max(f64::EPSILON);
                    let t = ((now - a.time) / span).clamp(0.0, 1.0) as f32;
                    let lerp = |x: f32, y: f32| x + (y - x) * t;
                    return Vector3 {
                        x: lerp(a.pos.x, b.pos.x),
                        y: lerp(a.pos.y, b.pos.y),
                        z: lerp(a.pos.z, b.pos.z),
                    };
                }
            }
        }
    }
    frame
        .samples
        .last()
        .map(|s| Vector3 { x: s.pos.x, y: s.pos.y, z: s.pos.z })
        .unwrap_or(Vector3 { x: 0.0, y: 0.0, z: 0.0 })
}

// 0x38f01c — __ZN3RBX12Accoutrement7dropAllEPNS_13ModelInstanceE
#[doc(alias = "RBX::Accoutrement::dropAll(RBX::ModelInstance *)")]
// was: RBX::Accoutrement::dropAll(RBX::ModelInstance *)
pub fn stub_0x38f01c(
    children: &mut Vec<SharedPtr<Instance>>,
    dropped: &mut Vec<SharedPtr<Instance>>,
) {
    // IDA 0x38f01c: `R1 = 0` + tail-call to `dropAllOthers` (disasm
    // 0x38f01c-0x38f01e) — drop-all is drop-all-others with a null exception.
    stub_0x38f024(children, None, dropped);
}

// 0x38f024 — __ZN3RBX12Accoutrement13dropAllOthersEPNS_13ModelInstanceEPS0_
#[doc(alias = "RBX::Accoutrement::dropAllOthers(RBX::ModelInstance *,RBX::Accoutrement*)")]
// was: RBX::Accoutrement::dropAllOthers(RBX::ModelInstance *,RBX::Accoutrement*)
pub fn stub_0x38f024(
    children: &mut Vec<SharedPtr<Instance>>,
    except: Option<*const Instance>,
    dropped: &mut Vec<SharedPtr<Instance>>,
) {
    // IDA 0x38f024: null workspace returns (disasm 0x38f032-0x38f036, behind
    // `findWorkspace` at 0x38f02c — the live-list call collapses it); then the
    // re-scan loop (0x38f040): first `Accoutrement` child (0x38f042, same
    // class-name match as 0x392c78), done when none (0x38f046-0x38f04a),
    // `except` skipped (0x38f04c-0x38f04e), else `setParent(child, ws)`
    // (0x38f038-0x38f03c). Draining here is the same observable outcome as the
    // re-scan — every non-except Accoutrement leaves the model list for the
    // workspace list.
    let mut kept = Vec::with_capacity(children.len());
    for child in children.drain(..) {
        let is_acc = child.class_name == "Accoutrement";
        let skipped = except.map_or(false, |e| SharedPtr::as_ptr(&child) == e);
        if is_acc && !skipped {
            dropped.push(child);
        } else {
            kept.push(child);
        }
    }
    *children = kept;
}

// 0x390270 — __ZN3RBX12PartInstance13TouchedSignal7connectIN5boost3_bi6bind_tIvNS3_4_mfi3mf1IvNS_12AccoutrementENS3_10shared_ptrINS_8InstanceEEEEENS4_5list2INS4_5valueIPS8_EENS3_3argILi1EEEEEEEEEN3rbx7signals10connectionET_
#[doc(alias = "rbx::signals::connection RBX::PartInstance::TouchedSignal::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>)")]
// was: rbx::signals::connection RBX::PartInstance::TouchedSignal::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>)
pub fn stub_0x390270(signal: &Signal<SharedPtr<Instance>>, bind: AccoutrementBind) -> TouchedConnection {
    // IDA 0x390270: the `bind_t` is boxed into a `TouchedSlot` (decomp 0x390314,
    // disasm 0x39030a-0x390314), `signal::connect` links it (0x390322) and the
    // `connection` is returned through the out-param; the slot temp is destroyed
    // (0x39032c) and the source `function1` cleared (0x390338). The
    // `FLog`/`flogPrint` branches (0x3902d2-0x3902e8, 0x39033e-0x390366) collapse.
    // The closure is the slot; the handle keeps its strong ref alive.
    let retained = bind;
    // Whole-struct capture: field-precise capture would grab the raw `target`
    // directly (bypassing the `Send`/`Sync` impls on the bind type) — same fix
    // as 0x323238/0x3903f0.
    let slot = Arc::new(move |hit: SharedPtr<Instance>| {
        let bound = retained;
        (bound.func)(bound.target, &hit)
    });
    signal.connect(slot.clone());
    TouchedConnection { keep: slot }
}

// 0x392738 — __ZNSt6vectorIN5boost8weak_ptrIN3RBX12PartInstanceEEESaIS4_EED2Ev
#[doc(alias = "std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>::~vector()")]
// was: std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>::~vector()
pub fn stub_0x392738(parts: Vec<WeakPtr<PartInstance>>) {
    // IDA 0x392738: per-element `weak_release` (decomp 0x39278e-0x3927a4), then
    // `operator delete` of the buffer (0x3927ac-0x3927b0) — `Vec` drop glue.
    drop(parts);
}

// 0x393b34 — __ZN3RBX13PartAdornment10setAdorneeEPNS_12PartInstanceE
#[doc(alias = "RBX::PartAdornment::setAdornee(RBX::PartInstance *)")]
// was: RBX::PartAdornment::setAdornee(RBX::PartInstance *)
pub fn stub_0x393b34(this: *mut PartAdornment, adornee: *const PartInstance) {
    // IDA 0x393b34: current adornee locked from the `+132` weak via the
    // nothrow `shared_ptr(weak)` ctor (decomp 0x393b5a, disasm
    // `ADD.W R1, R6, #0x84`); on change (`v12 != a2`, decomp 0x393b9e),
    // `shared_from<PartInstance>` on the incoming link (0x393bac) re-arms the
    // weak (`px` 0x393bb2, `pi` 0x393bc2) and `raisePropertyChanged` fires
    // (0x393bea). The change signal belongs to the Instance domain; the
    // modeled half is the compare + weak re-arm. Same EA as the instance.rs
    // twin.
    // BUG: clearing a live adornee to null runs `shared_from(null)` in the
    // original (null weak-owner read at 0x393bac); model space panics with
    // the `bad_weak_ptr` mapping instead of faulting.
    // SAFETY: `this` must point to a valid `PartAdornment`; `adornee` must be
    // null or point into a live `SharedPtr<PartInstance>` for the weak's life.
    unsafe {
        let current = (*this).adornee.upgrade();
        let same = match &current {
            Some(owned) => SharedPtr::as_ptr(owned) == adornee,
            None => adornee.is_null(),
        };
        if !same {
            if adornee.is_null() {
                panic!("0x393b34 setAdornee: bad_weak_ptr");
            }
            if (*adornee).weak_owner.upgrade().is_none() {
                panic!("0x393b34 setAdornee: bad_weak_ptr");
            }
            (*this).adornee = (*adornee).weak_owner.clone();
        }
    }
}

// 0x393c44 — __ZN3RBX13PartAdornmentC2EPKc
#[doc(alias = "RBX::PartAdornment::PartAdornment(char const*)")]
// was: RBX::PartAdornment::PartAdornment(char const*)
pub fn stub_0x393c44(adornee_slot: &mut WeakPtr<PartInstance>) {
    // IDA 0x393c44: `GuiBase3d` C2 (decomp 0x393c64, compiler-managed), vtable
    // installs (0x393c96-0x393cb4), class-descriptor registration
    // (0x393cdc-0x393d3a) and the adornee weak zeroed (`+33/+34 = 0`,
    // 0x393d40-0x393d46). Only the weak zeroing is model state; the name arg
    // rides the base C2.
    *adornee_slot = WeakPtr::new();
}

// 0x39406c — __ZNK3RBX13PartAdornment19getAdorneeDangerousEv
#[doc(alias = "RBX::PartAdornment::getAdorneeDangerous(void)const")]
// was: RBX::PartAdornment::getAdorneeDangerous(void)const
pub fn stub_0x39406c(adornee: &WeakPtr<PartInstance>) -> Option<SharedPtr<PartInstance>> {
    // IDA 0x39406c: nothrow `shared_ptr(weak)` lock of the `+132` weak (decomp
    // 0x394078, disasm 0x394072-0x394078) with the temp release
    // (0x394082-0x394084); the raw `px` return (disasm 0x394088-0x39408c) is
    // null when expired — `None` here. Unretained, hence dangerous.
    adornee.upgrade()
}

// 0x394090 — __ZN3RBX10Reflection17RefPropDescriptorINS_13PartAdornmentENS_12PartInstanceEED1Ev
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::~RefPropDescriptor()")]
// was: RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::~RefPropDescriptor()
pub fn stub_0x394090(desc: &mut PartRefPropDescriptor) {
    // IDA 0x394090: vtable resets (decomp 0x3940a6-0x3940aa, compiler-managed)
    // plus the conditional `operator delete` of the `+11` heap payload
    // (0x3940ac-0x3940b2) — the `owned` take. Twin of 0x3940e0; storage kept (D1).
    desc.owned = None;
}

// 0x395bb8 — __ZN3RBX10Reflection17RefPropDescriptorINS_13PartAdornmentENS_12PartInstanceEEC2IMS2_KFPS3_vEMS2_FvS6_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::RefPropDescriptor<RBX::PartInstance* (RBX::PartAdornment::*)(void)const,void (RBX::PartAdornment::*)(RBX::PartInstance*)>(char const*,char const*,RBX::PartInstance* (RBX::PartAdornment::*)(void)const,void (RBX::PartAdornment::*)(RBX::PartInstance*),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::RefPropDescriptor<RBX::PartInstance* (RBX::PartAdornment::*)(void)const,void (RBX::PartAdornment::*)(RBX::PartInstance*)>(char const*,char const*,RBX::PartInstance* (RBX::PartAdornment::*)(void)const,void (RBX::PartAdornment::*)(RBX::PartInstance*),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
pub fn stub_0x395bb8(desc: &mut PartRefPropDescriptor, read_only: bool, write_only: bool) {
    // IDA 0x395bb8: base `PropertyDescriptor` C2 on the PartAdornment class
    // descriptor + `RefType<PartInstance*>` singleton (decomp 0x395bca-0x395c10,
    // registration-managed), vtable installs (0x395c26-0x395c28) and the `0x14`-byte
    // getter/setter payload `new` at `+11` (0x395c2c-0x395c4e). The member-fn pair
    // collapses (Rust calls `stub_0x393b34`/the weak lock directly); the payload
    // box and attribute flags are the model state.
    desc.owned = Some(Box::new(PVRefExtra { words: [0; 8] }));
    desc.read_only = read_only;
    desc.write_only = write_only;
}

// 0x395c5c — __ZN3RBX10Reflection17RefPropDescriptorINS_13PartAdornmentENS_12PartInstanceEED0Ev
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::~RefPropDescriptor()")]
// was: RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::~RefPropDescriptor()
pub fn stub_0x395c5c(desc: Box<PartRefPropDescriptor>) {
    // IDA 0x395c5c: same as the D1 (0x394090: vtable resets + `+11` payload delete,
    // decomp 0x395c72-0x395c7e) plus `operator delete(this)` — the D0 frees
    // storage, so ownership moves in and drops.
    drop(desc);
}

// 0x395c8c — __ZNK3RBX10Reflection17RefPropDescriptorINS_13PartAdornmentENS_12PartInstanceEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::isReadOnly(void)const")]
// was: RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::isReadOnly(void)const
pub fn stub_0x395c8c(desc: &PartRefPropDescriptor) -> bool {
    // IDA 0x395c8c: delegates through the `+44` attribute word's first vtable slot
    // (decomp 0x395c98) — the read-only flag behind it.
    desc.read_only
}

// 0x395c9c — __ZNK3RBX10Reflection17RefPropDescriptorINS_13PartAdornmentENS_12PartInstanceEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::isWriteOnly(void)const")]
// was: RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::isWriteOnly(void)const
pub fn stub_0x395c9c(desc: &PartRefPropDescriptor) -> bool {
    // IDA 0x395c9c: delegates through the `+44` attribute word's second vtable slot
    // (decomp 0x395ca8) — the write-only flag behind it.
    desc.write_only
}

// 0x395cac — __ZNK3RBX10Reflection17RefPropDescriptorINS_13PartAdornmentENS_12PartInstanceEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
pub fn stub_0x395cac(a: &WeakPtr<PartInstance>, b: &WeakPtr<PartInstance>) -> bool {
    // IDA 0x395cac: each side's ref is read through the `+44` getter's slot `+8`
    // (decomp 0x395cbc/0x395cc6, disasm 0x395cb2-0x395cc6) and the raw pointers
    // compared (`CMP` + `IT EQ`, disasm 0x395ccc-0x395cd0).
    a.upgrade().as_ref().map(SharedPtr::as_ptr) == b.upgrade().as_ref().map(SharedPtr::as_ptr)
}

// 0x6cc1a0 — __ZN3RBX9WorkspaceD2Ev
#[doc(alias = "RBX::Workspace::~Workspace()")]
// was: RBX::Workspace::~Workspace()
pub fn stub_0x6cc1a0(ws: Workspace) {
    // IDA 0x6cc1a0 (D2 `~Workspace`): runs member destructors; Rust drops
    // `ws` at scope end — the same sequence (twin of `stub_0x6cc160` in
    // workspace.rs, which owns the D1 slot there).
    drop(ws);
}

// 0x6cc71c — __ZThn32_N3RBX9WorkspaceD1Ev
#[doc(alias = "non-virtual thunk toRBX::Workspace::~Workspace()")]
// was: non-virtual thunk toRBX::Workspace::~Workspace()
pub fn stub_0x6cc71c(ws: Workspace) {
    // IDA 0x6cc71c (`Thn32` to D1): non-virtual thunk adjusting `this`
    // across the second base; the adjustment is compiler-owned, so this
    // forwards to the D2 body.
    stub_0x6cc1a0(ws);
}

// 0x6cc72c — __ZThn36_N3RBX9WorkspaceD1Ev
#[doc(alias = "non-virtual thunk toRBX::Workspace::~Workspace()")]
// was: non-virtual thunk toRBX::Workspace::~Workspace()
pub fn stub_0x6cc72c(ws: Workspace) {
    // IDA 0x6cc72c (`Thn36` to D1): same forward shape as 0x6cc71c.
    stub_0x6cc1a0(ws);
}

// 0x6cc73c — __ZThn120_N3RBX9WorkspaceD1Ev
#[doc(alias = "non-virtual thunk toRBX::Workspace::~Workspace()")]
// was: non-virtual thunk toRBX::Workspace::~Workspace()
pub fn stub_0x6cc73c(ws: Workspace) {
    // IDA 0x6cc73c (`Thn120` to D1): same forward shape as 0x6cc71c.
    stub_0x6cc1a0(ws);
}

// 0x6cc74c — __ZThn280_N3RBX9WorkspaceD1Ev
#[doc(alias = "non-virtual thunk toRBX::Workspace::~Workspace()")]
// was: non-virtual thunk toRBX::Workspace::~Workspace()
pub fn stub_0x6cc74c(ws: Workspace) {
    // IDA 0x6cc74c (`Thn280` to D1): same forward shape as 0x6cc71c.
    stub_0x6cc1a0(ws);
}

// 0x6cc760 — __ZThn324_N3RBX9WorkspaceD1Ev
#[doc(alias = "non-virtual thunk toRBX::Workspace::~Workspace()")]
// was: non-virtual thunk toRBX::Workspace::~Workspace()
pub fn stub_0x6cc760(ws: Workspace) {
    // IDA 0x6cc760 (`Thn324` to D1): same forward shape as 0x6cc71c.
    stub_0x6cc1a0(ws);
}

// 0x6cc774 — __ZThn356_N3RBX9WorkspaceD1Ev
#[doc(alias = "non-virtual thunk toRBX::Workspace::~Workspace()")]
// was: non-virtual thunk toRBX::Workspace::~Workspace()
pub fn stub_0x6cc774(ws: Workspace) {
    // IDA 0x6cc774 (`Thn356` to D1): same forward shape as 0x6cc71c.
    stub_0x6cc1a0(ws);
}

// 0x6cc788 — __ZN3RBX9Workspace23computeExtentsWorldFastEv
#[doc(alias = "RBX::Workspace::computeExtentsWorldFast(void)")]
// was: RBX::Workspace::computeExtentsWorldFast(void)
pub fn stub_0x6cc788(ws: &mut Workspace) {
    // IDA 0x6cc788: recomputes only when the cache is cold
    // (`*(a2 + 424) == 0.0`, 0x6cc7b4) or stale (`*(a2 + 552) - stamp >
    // 2.0`); the recompute is `ModelInstance::computeExtentsWorld` into
    // words `+400`..`+420` (0x6cc7ba-0x6cc7de), then the stamp is refreshed
    // from the clock (0x6cc7e4).
    if ws.extents_stamp == 0.0 || ws.extents_clock - ws.extents_stamp > 2.0 {
        ws.extents = model_compute_extents();
        ws.extents_stamp = ws.extents_clock;
    }
}
/// Seam for `ModelInstance::computeExtentsWorld` behind
/// `computeExtentsWorldFast` (IDA `0x6cc7ba`): model extents live in the
/// part/model batch.
fn model_compute_extents() -> [f32; 6] {
    [0.0; 6]
}

// 0x6cc804 — __ZN3RBX9Workspace11onHeartbeatERKNS_9HeartbeatE
#[doc(alias = "RBX::Workspace::onHeartbeat(RBX::Heartbeat const&)")]
// was: RBX::Workspace::onHeartbeat(RBX::Heartbeat const&)
pub fn stub_0x6cc804() -> ! {
    todo!("0x6cc804 RBX::Workspace::onHeartbeat(RBX::Heartbeat const&)")
}

// 0x6ccaa8 — __ZN3RBX9Workspace15replenishCameraEv
#[doc(alias = "RBX::Workspace::replenishCamera(void)")]
// was: RBX::Workspace::replenishCamera(void)
pub fn stub_0x6ccaa8() -> ! {
    todo!("0x6ccaa8 RBX::Workspace::replenishCamera(void)")
}

// 0x6ccc18 — __ZNK3RBX9Workspace11askAddChildEPKNS_8InstanceE
#[doc(alias = "RBX::Workspace::askAddChild(RBX::Instance const*)const")]
// was: RBX::Workspace::askAddChild(RBX::Instance const*)const
pub fn stub_0x6ccc18(child: *const Instance) -> bool {
    // IDA 0x6ccc18: null child returns 0 (0x6ccc1a-0x6ccc22); else the
    // `__dynamic_cast` to `IAdornable` decides (0x6ccc4a), returned unnegated.
    // SAFETY: `child` must be null or point to a valid `Instance`.
    !child.is_null() && unsafe { instance_is_adornable(child) }
}
/// Seam for `__dynamic_cast Instance -> IAdornable` (IDA `0x6ccc4a`,
/// `0x6ccc7c`): the interface hierarchy is unmodeled, so any live instance
/// is admitted — matching the runtime-observed behavior that the workspace
/// parents parts, models, decals, and services alike. The exact interface
/// set lands with the hierarchy batch.
/// # Safety
/// `child` must point to a valid `Instance`.
unsafe fn instance_is_adornable(child: *const Instance) -> bool {
    !child.is_null()
}

// 0x6ccc50 — __ZN3RBX9Workspace20onDescendantRemovingERKN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Workspace::onDescendantRemoving(rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: RBX::Workspace::onDescendantRemoving(boost::shared_ptr<RBX::Instance> const&)
pub fn stub_0x6ccc50(child: *const Instance) {
    // IDA 0x6ccc50: a live `IAdornable` newcomer is reported to the
    // `IAdornableCollector` at `+392` (0x6ccc56-0x6ccc86); control then
    // falls into `ModelInstance::onDescendantRemoving` (tail call).
    // SAFETY: `child` must be null or point to a valid `Instance`.
    unsafe {
        if !child.is_null() && instance_is_adornable(child) {
            adornable_collector_removing(child);
        }
        model_base_descendant_removing(child);
    }
}
/// Seam for `IAdornableCollector::onRenderableDescendantRemoving` (IDA
/// `0x6ccc86`): the renderable collector lives in the rendering crate.
fn adornable_collector_removing(_child: *const Instance) {}
/// Seam for `ModelInstance::onDescendantRemoving` (IDA `0x6ccc8a` tail
/// call): model semantics land with the model batch.
fn model_base_descendant_removing(_child: *const Instance) {}

// 0x6ccc98 — __ZN3RBX9Workspace17onDescendantAddedEPNS_8InstanceE
#[doc(alias = "RBX::Workspace::onDescendantAdded(RBX::Instance *)")]
// was: RBX::Workspace::onDescendantAdded(RBX::Instance *)
pub fn stub_0x6ccc98(child: *const Instance) {
    // IDA 0x6ccc98: `ModelInstance::onDescendantAdded` runs first
    // (0x6ccca6); a live `IAdornable` newcomer is then reported to the
    // `IAdornableCollector` (0x6ccccc+) — the adding mirror of 0x6ccc50.
    // SAFETY: `child` must be null or point to a valid `Instance`.
    unsafe {
        model_base_descendant_added(child);
        if !child.is_null() && instance_is_adornable(child) {
            adornable_collector_added(child);
        }
    }
}
/// Seam for `ModelInstance::onDescendantAdded` (IDA `0x6ccca6`): model
/// semantics land with the model batch.
fn model_base_descendant_added(_child: *const Instance) {}
/// Seam for the `IAdornableCollector` report in `onDescendantAdded` (IDA
/// `0x6ccccc`+): the renderable collector lives in the rendering crate.
fn adornable_collector_added(_child: *const Instance) {}

// 0x6ccda0 — __ZN3RBX9Workspace14startDecalDragEPNS_5DecalE
#[doc(alias = "RBX::Workspace::startDecalDrag(RBX::Decal *)")]
// was: RBX::Workspace::startDecalDrag(RBX::Decal *)
pub fn stub_0x6ccda0() -> ! {
    todo!("0x6ccda0 RBX::Workspace::startDecalDrag(RBX::Decal *)")
}

// 0x6ccf30 — __ZN3RBX9Workspace15setMouseCommandEN5boost10shared_ptrINS_12MouseCommandEEE
#[doc(alias = "RBX::Workspace::setMouseCommand(rbx_core::SharedPtr<RBX::MouseCommand>)")]
// was: RBX::Workspace::setMouseCommand(boost::shared_ptr<RBX::MouseCommand>)
pub fn stub_0x6ccf30(ws: &mut Workspace, cmd: SharedPtr<MouseCommand>) {
    // IDA 0x6ccf30: logs under `MouseCommandLifetime` (0x6ccfac), resolves
    // the provider and the active plugin (0x6ccfbc-0x6ccfd4), then retains
    // the command via `shared_ptr::operator=` (0x6cd046) — the store behind
    // `Workspace::current_command` (`+0x1C8`). The log and the plugin
    // notification land with their batches; the retain is the `Option`
    // store (cloned `SharedPtr` is the `shared_count` copy).
    ws.current_command = Some(cmd);
}

// 0x6cd45c — __ZN3RBX9Workspace9getCameraEv
#[doc(alias = "RBX::Workspace::getCamera(void)")]
// was: RBX::Workspace::getCamera(void)
pub fn stub_0x6cd45c(ws: *const Workspace) -> *const () {
    // IDA 0x6cd45c: virtual `getCamera` (slot `+200`, 0x6cd45c) — the
    // override lattice collapses to the stored camera until vtables are
    // modelled, so this returns `current_camera` like the dangerous
    // accessor (0x6cb73c); the const fallback stays with `getConstCamera`.
    // SAFETY: `ws` must point to a valid `Workspace`.
    unsafe { (*ws).current_camera }
}

// 0x6cd464 — __ZThn280_N3RBX9Workspace9getCameraEv
#[doc(alias = "non-virtual thunk toRBX::Workspace::getCamera(void)")]
// was: non-virtual thunk toRBX::Workspace::getCamera(void)
pub fn stub_0x6cd464(ws: *const Workspace) -> *const () {
    // IDA 0x6cd464 (`Thn280` to `getCamera`): non-virtual thunk with a
    // compiler-owned `this` adjustment; forwards to the virtual body.
    stub_0x6cd45c(ws)
}

// 0x6cd478 — __ZNK3RBX9Workspace14getConstCameraEv
#[doc(alias = "RBX::Workspace::getConstCamera(void)const")]
// was: RBX::Workspace::getConstCamera(void)const
pub fn stub_0x6cd478(ws: *const Workspace) -> *const () {
    // IDA 0x6cd478: returns `*(this + 127)` when non-null (0x6cd480), else
    // `*(this + 129)` (0x6cd482) — the null-fallback is what makes this the
    // "const" (non-creating) accessor next to `getCamera`.
    // SAFETY: `ws` must point to a valid `Workspace`.
    unsafe {
        let cur = (*ws).current_camera;
        if cur.is_null() {
            (*ws).fallback_camera
        } else {
            cur
        }
    }
}

// 0x6cd488 — __ZThn280_NK3RBX9Workspace14getConstCameraEv
#[doc(alias = "non-virtual thunk toRBX::Workspace::getConstCamera(void)const")]
// was: non-virtual thunk toRBX::Workspace::getConstCamera(void)const
pub fn stub_0x6cd488(ws: *const Workspace) -> *const () {
    // IDA 0x6cd488 (`Thn280` to `getConstCamera`): non-virtual thunk with a
    // compiler-owned `this` adjustment; forwards to the const body.
    stub_0x6cd478(ws)
}

// 0x6cd540 — __ZN3RBX9Workspace10setTerrainEPNS_8InstanceE
#[doc(alias = "RBX::Workspace::setTerrain(RBX::Instance *)")]
// was: RBX::Workspace::setTerrain(RBX::Instance *)
pub fn stub_0x6cd540(ws: &mut Workspace, terrain: *const Instance) {
    // IDA 0x6cd540: retains the incoming terrain (`shared_from` +
    // `operator=`, 0x6cd5b0+), releases the previous link, stores word
    // `+110`, and raises `Terrain`'s property change. The retain collapses
    // to the raw store until `Terrain` gains a weak owner (same convention
    // as `setCurrentCamera`, IDA 0x6cb744).
    ws.terrain = terrain as *const ();
}

// 0x6cd688 — __ZN3RBX9Workspace13createTerrainEv
#[doc(alias = "RBX::Workspace::createTerrain(void)")]
// was: RBX::Workspace::createTerrain(void)
pub fn stub_0x6cd688(ws: &mut Workspace) {
    // IDA 0x6cd688: builds a fresh `Terrain` (part-backed megacluster init)
    // and installs it via the `setTerrain` path when no terrain is linked.
    // Creation details land with the terrain batch; the ensure-linked shape
    // is modelled.
    if ws.terrain.is_null() {
        let fresh = terrain_create();
        if !fresh.is_null() {
            stub_0x6cd540(ws, fresh as *const Instance);
        }
    }
}
/// Seam for the `Terrain` construction inside `createTerrain` (IDA
/// `0x6cd688`): the terrain class lives outside datamodel.
fn terrain_create() -> *const () {
    core::ptr::null()
}

// 0x6cd86c — __ZN3RBX9Workspace12clearTerrainEv
#[doc(alias = "RBX::Workspace::clearTerrain(void)")]
// was: RBX::Workspace::clearTerrain(void)
pub fn stub_0x6cd86c(ws: *mut Workspace) {
    // IDA 0x6cd86c: no-op when the terrain link (`+110`) is null
    // (0x6cd878); else logs under `MegaClusterInit` (0x6cd888-0x6cd89a) and
    // clears the terrain's framework flag (`FWValue<bool>::set(fw + 21,
    // false, terrain + 68)`, 0x6cd8a0-0x6cd8b4). The log is unobservable;
    // the flag clear is the seam below.
    // SAFETY: `ws` must point to a valid `Workspace`.
    unsafe {
        let terrain = (*ws).terrain;
        if !terrain.is_null() {
            terrain_set_cleared(terrain);
        }
    }
}
/// Seam for the terrain flag clear in `clearTerrain` (IDA `0x6cd8ae`):
/// the terrain framework value lives outside datamodel.
fn terrain_set_cleared(_terrain: *const ()) {}

// 0x6cd8e0 — __ZN3RBX9Workspace27selectAllTopLevelRenderableEv
#[doc(alias = "RBX::Workspace::selectAllTopLevelRenderable(void)")]
// was: RBX::Workspace::selectAllTopLevelRenderable(void)
pub fn stub_0x6cd8e0() -> ! {
    todo!("0x6cd8e0 RBX::Workspace::selectAllTopLevelRenderable(void)")
}

// 0x6cda3c — __ZN3RBX9Workspace11insertItemsEP10XmlElementRSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS7_EENS_10InsertModeENS_10PromptModeEb
#[doc(alias = "RBX::Workspace::insertItems(XmlElement *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> &,RBX::InsertMode,RBX::PromptMode,bool)")]
// was: RBX::Workspace::insertItems(XmlElement *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> &,RBX::InsertMode,RBX::PromptMode,bool)
pub fn stub_0x6cda3c() -> ! {
    todo!("0x6cda3c RBX::Workspace::insertItems(XmlElement *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> &,RBX::InsertMode,RBX::PromptMode,bool)")
}

// 0x6cdae8 — __ZN3RBX9Workspace13insertContentENS_9ContentIdERSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS6_EENS_10InsertModeENS_10PromptModeE
#[doc(alias = "RBX::Workspace::insertContent(RBX::ContentId,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> &,RBX::InsertMode,RBX::PromptMode)")]
// was: RBX::Workspace::insertContent(RBX::ContentId,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> &,RBX::InsertMode,RBX::PromptMode)
pub fn stub_0x6cdae8() -> ! {
    todo!("0x6cdae8 RBX::Workspace::insertContent(RBX::ContentId,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> &,RBX::InsertMode,RBX::PromptMode)")
}

// 0x6cdd98 — __ZN3RBX9Workspace11joinAllHackEv
#[doc(alias = "RBX::Workspace::joinAllHack(void)")]
// was: RBX::Workspace::joinAllHack(void)
pub fn stub_0x6cdd98(ws: *const Workspace) {
    // IDA 0x6cdd98: tail-calls `World::joinAll` on the world at `+78`.
    // SAFETY: `ws` must point to a valid `Workspace`.
    unsafe {
        world_join_all((*ws).world);
    }
}
/// Seam for `RBX::World::joinAll` (IDA `0x6cdd98`): the physics world
/// lives outside datamodel.
fn world_join_all(_world: *const ()) {}

// 0x6cde50 — __ZN3RBX9Workspace5startEv
#[doc(alias = "RBX::Workspace::start(void)")]
// was: RBX::Workspace::start(void)
pub fn stub_0x6cde50() -> ! {
    todo!("0x6cde50 RBX::Workspace::start(void)")
}

// 0x6ce0b8 — __ZN3RBX9Workspace8assembleEv
#[doc(alias = "RBX::Workspace::assemble(void)")]
// was: RBX::Workspace::assemble(void)
pub fn stub_0x6ce0b8(ws: *const Workspace) {
    // IDA 0x6ce0b8: `World::assemble` on the world at `+78` (0x6ce0c2),
    // then a debug-only `ReleaseAssert(world->isAssembled())` naming
    // `Workspace.cpp:913` (0x6ce0d2-0x6ce0f0).
    // SAFETY: `ws` must point to a valid `Workspace`.
    unsafe {
        let world = (*ws).world;
        world_assemble(world);
        debug_assert!(world_is_assembled(world));
    }
}
/// Seam for `RBX::World::assemble` (IDA `0x6ce0c2`): the physics world
/// lives outside datamodel.
fn world_assemble(_world: *const ()) {}
/// Seam for `RBX::World::isAssembled` (IDA `0x6ce0da`): the physics world
/// lives outside datamodel; reports assembled so the author's assert holds.
fn world_is_assembled(_world: *const ()) -> bool {
    true
}

// 0x6ce128 — __ZN3RBX9Workspace4stopEv
#[doc(alias = "RBX::Workspace::stop(void)")]
// was: RBX::Workspace::stop(void)
pub fn stub_0x6ce128() -> ! {
    todo!("0x6ce128 RBX::Workspace::stop(void)")
}

// 0x6ce398 — __ZN3RBX9Workspace25updateDistributedGameTimeEv
#[doc(alias = "RBX::Workspace::updateDistributedGameTime(void)")]
// was: RBX::Workspace::updateDistributedGameTime(void)
pub fn stub_0x6ce398(ws: &mut Workspace) {
    // IDA 0x6ce398: reads the `RunService` game clock (`+116`, 0x6ce3ac).
    // With a server present (0x6ce3a8-0x6ce3b2) the distributed time is
    // stored only on change plus `raisePropertyChanged(DistributedGameTime)`
    // (0x6ce3b4-0x6ce3dc, same descriptor as `setDistributedGameTime`);
    // without one it is stored unconditionally (0x6ce3e0). The raise has no
    // host signal yet, so both arms converge on the store; the cached
    // `run_service_time` stands in for the service call.
    if crate::workspace::stub_0x6cac18(ws as *const Workspace, core::ptr::null()) {
        if ws.distributed_game_time != ws.run_service_time {
            ws.distributed_game_time = ws.run_service_time;
        }
    } else {
        ws.distributed_game_time = ws.run_service_time;
    }
}

// 0x6ce3e8 — __ZN3RBX9Workspace5resetEv
#[doc(alias = "RBX::Workspace::reset(void)")]
// was: RBX::Workspace::reset(void)
pub fn stub_0x6ce3e8(ws: *mut Workspace) {
    // IDA 0x6ce3e8: `stop` (0x6ce3ee) then `World::reset` on the world at
    // `+78` (0x6ce3f2). `stop` (0x6ce128) is still a stub, so its slot is a
    // seam that forwards once it lands.
    // SAFETY: `ws` must point to a valid `Workspace`.
    unsafe {
        workspace_stop_slot(ws);
        world_reset((*ws).world);
    }
}
/// Seam for `Workspace::stop` (IDA `0x6ce128`) as called by `reset` (IDA
/// `0x6ce3ee`): forwards to `stub_0x6ce128` once the stop batch lands.
fn workspace_stop_slot(_ws: *mut Workspace) {}
/// Seam for `RBX::World::reset` (IDA `0x6ce3f2`): the physics world lives
/// outside datamodel.
fn world_reset(_world: *const ()) {}

// 0x6ce400 — __ZN3RBX9Workspace12detachParentEPNS_8InstanceE
#[doc(alias = "RBX::Workspace::detachParent(RBX::Instance *)")]
// was: RBX::Workspace::detachParent(RBX::Instance *)
pub fn stub_0x6ce400(child: *const Instance) {
    // IDA 0x6ce400: retains the child via `shared_from<Instance>`
    // (0x6ce424), clears its parent through `setParentInternal`
    // (0x6ce45c), then drops emptied models via `clearEmptiedModels`
    // (0x6ce468). Both callees land with the tree batch, so their slots
    // are seams; the detach-then-sweep order is modelled.
    // SAFETY: `child` must be null or point to a valid `Instance`.
    unsafe {
        set_parent_internal_slot(child, core::ptr::null());
        clear_emptied_models_slot(child);
    }
}
/// Seam for `RBX::Instance::setParentInternal` (IDA `0x6ce45c`) as called
/// by `detachParent`: forwards once the tree batch lands.
/// # Safety
/// Both pointers must be null or valid `Instance`s.
unsafe fn set_parent_internal_slot(_child: *const Instance, _parent: *const Instance) {}
/// Seam for `RBX::Workspace::clearEmptiedModels` (IDA `0x6ce468`) as
/// called by `detachParent`: forwards to `stub_0x6ce4d4` once it lands.
/// # Safety
/// `child` must be null or point to a valid `Instance`.
unsafe fn clear_emptied_models_slot(_child: *const Instance) {}

// 0x6ce4d4 — __ZN3RBX9Workspace18clearEmptiedModelsERN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Workspace::clearEmptiedModels(rbx_core::SharedPtr<RBX::Instance> &)")]
// was: RBX::Workspace::clearEmptiedModels(boost::shared_ptr<RBX::Instance> &)
pub fn stub_0x6ce4d4() -> ! {
    todo!("0x6ce4d4 RBX::Workspace::clearEmptiedModels(boost::shared_ptr<RBX::Instance> &)")
}

// 0x6ce5b8 — __ZN3RBX9Workspace17handleFallenPartsEv
#[doc(alias = "RBX::Workspace::handleFallenParts(void)")]
// was: RBX::Workspace::handleFallenParts(void)
pub fn stub_0x6ce5b8() -> ! {
    todo!("0x6ce5b8 RBX::Workspace::handleFallenParts(void)")
}

// 0x6ce8dc — __ZN3RBX9Workspace11physicsStepEbfi
#[doc(alias = "RBX::Workspace::physicsStep(bool,float,int)")]
// was: RBX::Workspace::physicsStep(bool,float,int)
pub fn stub_0x6ce8dc() -> ! {
    todo!("0x6ce8dc RBX::Workspace::physicsStep(bool,float,int)")
}

// 0x6ceda4 — __ZN3RBX9Workspace22setDefaultMouseCommandEv
#[doc(alias = "RBX::Workspace::setDefaultMouseCommand(void)")]
// was: RBX::Workspace::setDefaultMouseCommand(void)
pub fn stub_0x6ceda4() -> ! {
    todo!("0x6ceda4 RBX::Workspace::setDefaultMouseCommand(void)")
}

// 0x6cee80 — __ZN3RBX9Workspace19setNullMouseCommandEv
#[doc(alias = "RBX::Workspace::setNullMouseCommand(void)")]
// was: RBX::Workspace::setNullMouseCommand(void)
pub fn stub_0x6cee80() -> ! {
    todo!("0x6cee80 RBX::Workspace::setNullMouseCommand(void)")
}

// 0x6cefa4 — __ZN3RBX9Workspace8render2dEPNS_5AdornE
#[doc(alias = "RBX::Workspace::render2d(RBX::Adorn *)")]
// was: RBX::Workspace::render2d(RBX::Adorn *)
pub fn stub_0x6cefa4() -> ! {
    todo!("0x6cefa4 RBX::Workspace::render2d(RBX::Adorn *)")
}

// 0x6cefc8 — __ZThn96_N3RBX9Workspace8render2dEPNS_5AdornE
#[doc(alias = "non-virtual thunk toRBX::Workspace::render2d(RBX::Adorn *)")]
// was: non-virtual thunk toRBX::Workspace::render2d(RBX::Adorn *)
pub fn stub_0x6cefc8() -> ! {
    todo!("0x6cefc8 non-virtual thunk toRBX::Workspace::render2d(RBX::Adorn *)")
}

// 0x6ceff0 — __ZN3RBX9Workspace9getCursorEv
#[doc(alias = "RBX::Workspace::getCursor(void)")]
// was: RBX::Workspace::getCursor(void)
pub fn stub_0x6ceff0() -> ! {
    todo!("0x6ceff0 RBX::Workspace::getCursor(void)")
}

// 0x6cf11c — __ZN3RBX9Workspace13render3dAdornEPNS_5AdornE
#[doc(alias = "RBX::Workspace::render3dAdorn(RBX::Adorn *)")]
// was: RBX::Workspace::render3dAdorn(RBX::Adorn *)
pub fn stub_0x6cf11c() -> ! {
    todo!("0x6cf11c RBX::Workspace::render3dAdorn(RBX::Adorn *)")
}

// 0x6cf3f4 — __ZN3RBXL9DrawAdornEN5boost10shared_ptrINS_8InstanceEEEPNS_5AdornENS_11SelectStateEPNS_9WorkspaceE
#[doc(alias = "RBX::DrawAdorn(rbx_core::SharedPtr<RBX::Instance>,RBX::Adorn *,RBX::SelectState,RBX::Workspace *)")]
// was: RBX::DrawAdorn(boost::shared_ptr<RBX::Instance>,RBX::Adorn *,RBX::SelectState,RBX::Workspace *)
pub fn stub_0x6cf3f4() -> ! {
    todo!("0x6cf3f4 RBX::DrawAdorn(boost::shared_ptr<RBX::Instance>,RBX::Adorn *,RBX::SelectState,RBX::Workspace *)")
}

// 0x6cf454 — __ZThn96_N3RBX9Workspace13render3dAdornEPNS_5AdornE
#[doc(alias = "non-virtual thunk toRBX::Workspace::render3dAdorn(RBX::Adorn *)")]
// was: non-virtual thunk toRBX::Workspace::render3dAdorn(RBX::Adorn *)
pub fn stub_0x6cf454() -> ! {
    todo!("0x6cf454 non-virtual thunk toRBX::Workspace::render3dAdorn(RBX::Adorn *)")
}

// 0x6cf45c — __ZN3RBX9Workspace19append3dSortedAdornERSt6vectorIPNS_10IAdornableESaIS3_EE
#[doc(alias = "RBX::Workspace::append3dSortedAdorn(std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>> &)")]
// was: RBX::Workspace::append3dSortedAdorn(std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>> &)
pub fn stub_0x6cf45c() -> ! {
    todo!("0x6cf45c RBX::Workspace::append3dSortedAdorn(std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>> &)")
}

// 0x6cf47c — __ZN3RBX9Workspace18hasModalGuiObjectsEv
#[doc(alias = "RBX::Workspace::hasModalGuiObjects(void)")]
// was: RBX::Workspace::hasModalGuiObjects(void)
pub fn stub_0x6cf47c() -> ! {
    todo!("0x6cf47c RBX::Workspace::hasModalGuiObjects(void)")
}

// 0x6cf570 — __ZN3RBX9Workspace24requestFirstPersonCameraEbbi
#[doc(alias = "RBX::Workspace::requestFirstPersonCamera(bool,bool,int)")]
// was: RBX::Workspace::requestFirstPersonCamera(bool,bool,int)
pub fn stub_0x6cf570() -> ! {
    todo!("0x6cf570 RBX::Workspace::requestFirstPersonCamera(bool,bool,int)")
}

// 0x6cf618 — __ZN3RBX9Workspace16setRightMousePanEv
#[doc(alias = "RBX::Workspace::setRightMousePan(void)")]
// was: RBX::Workspace::setRightMousePan(void)
pub fn stub_0x6cf618() -> ! {
    todo!("0x6cf618 RBX::Workspace::setRightMousePan(void)")
}

// 0x6cf648 — __ZN3RBX9Workspace19cancelRightMousePanEv
#[doc(alias = "RBX::Workspace::cancelRightMousePan(void)")]
// was: RBX::Workspace::cancelRightMousePan(void)
pub fn stub_0x6cf648() -> ! {
    todo!("0x6cf648 RBX::Workspace::cancelRightMousePan(void)")
}

// 0x6cf66c — __ZN3RBX9Workspace7processERKNS_8GuiEventE
#[doc(alias = "RBX::Workspace::process(RBX::GuiEvent const&)")]
// was: RBX::Workspace::process(RBX::GuiEvent const&)
pub fn stub_0x6cf66c() -> ! {
    todo!("0x6cf66c RBX::Workspace::process(RBX::GuiEvent const&)")
}

// 0x6cfe54 — __ZThn320_N3RBX9Workspace7processERKNS_8GuiEventE
#[doc(alias = "non-virtual thunk toRBX::Workspace::process(RBX::GuiEvent const&)")]
// was: non-virtual thunk toRBX::Workspace::process(RBX::GuiEvent const&)
pub fn stub_0x6cfe54() -> ! {
    todo!("0x6cfe54 non-virtual thunk toRBX::Workspace::process(RBX::GuiEvent const&)")
}

// 0x6cfe64 — __ZN3RBX9Workspace17onServiceProviderEPNS_15ServiceProviderES2_
#[doc(alias = "RBX::Workspace::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// was: RBX::Workspace::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)
pub fn stub_0x6cfe64() -> ! {
    todo!("0x6cfe64 RBX::Workspace::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")
}

// 0x6d0138 — __ZN3RBX9Workspace15scriptShouldRunEPNS_10BaseScriptE
#[doc(alias = "RBX::Workspace::scriptShouldRun(RBX::BaseScript *)")]
// was: RBX::Workspace::scriptShouldRun(RBX::BaseScript *)
pub fn stub_0x6d0138() -> ! {
    todo!("0x6d0138 RBX::Workspace::scriptShouldRun(RBX::BaseScript *)")
}

// 0x6d02e4 — __ZThn388_N3RBX9Workspace15scriptShouldRunEPNS_10BaseScriptE
#[doc(alias = "non-virtual thunk toRBX::Workspace::scriptShouldRun(RBX::BaseScript *)")]
// was: non-virtual thunk toRBX::Workspace::scriptShouldRun(RBX::BaseScript *)
pub fn stub_0x6d02e4() -> ! {
    todo!("0x6d02e4 non-virtual thunk toRBX::Workspace::scriptShouldRun(RBX::BaseScript *)")
}

// 0x6d0328 — __ZNK3RBX9Workspace22getDistributedGameTimeEv
#[doc(alias = "RBX::Workspace::getDistributedGameTime(void)const")]
// was: RBX::Workspace::getDistributedGameTime(void)const
pub fn stub_0x6d0328() -> ! {
    todo!("0x6d0328 RBX::Workspace::getDistributedGameTime(void)const")
}

// 0x6d0334 — __ZN3RBX10Reflection14PropDescriptorINS_9WorkspaceEdED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Workspace,double>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<RBX::Workspace,double>::~PropDescriptor()
pub fn stub_0x6d0334() -> ! {
    todo!("0x6d0334 RBX::Reflection::PropDescriptor<RBX::Workspace,double>::~PropDescriptor()")
}

// 0x6d0358 — __ZNK3RBX9Workspace26getNetworkStreamingEnabledEv
#[doc(alias = "RBX::Workspace::getNetworkStreamingEnabled(void)const")]
// was: RBX::Workspace::getNetworkStreamingEnabled(void)const
pub fn stub_0x6d0358() -> ! {
    todo!("0x6d0358 RBX::Workspace::getNetworkStreamingEnabled(void)const")
}

// 0x6d0360 — __ZN3RBX9Workspace26setNetworkStreamingEnabledEb
#[doc(alias = "RBX::Workspace::setNetworkStreamingEnabled(bool)")]
// was: RBX::Workspace::setNetworkStreamingEnabled(bool)
pub fn stub_0x6d0360() -> ! {
    todo!("0x6d0360 RBX::Workspace::setNetworkStreamingEnabled(bool)")
}

// 0x6d0368 — __ZN3RBX10Reflection14PropDescriptorINS_9WorkspaceEbED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Workspace,bool>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<RBX::Workspace,bool>::~PropDescriptor()
pub fn stub_0x6d0368() -> ! {
    todo!("0x6d0368 RBX::Reflection::PropDescriptor<RBX::Workspace,bool>::~PropDescriptor()")
}

// 0x6d038c — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(RBX::ContentId),1>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Workspace,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(RBX::ContentId),1>::~BoundFuncDesc()
pub fn stub_0x6d038c() -> ! {
    todo!("0x6d038c RBX::Reflection::BoundFuncDesc<RBX::Workspace,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(RBX::ContentId),1>::~BoundFuncDesc()")
}

// 0x6d03cc — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFvN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,void ()(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),1>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Workspace,void ()(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),1>::~BoundFuncDesc()
pub fn stub_0x6d03cc() -> ! {
    todo!("0x6d03cc RBX::Reflection::BoundFuncDesc<RBX::Workspace,void ()(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),1>::~BoundFuncDesc()")
}

// 0x6d04c0 — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_7Region3ES7_iELi3EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(RBX::Region3,rbx_core::SharedPtr<RBX::Instance>,int),3>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Workspace,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(RBX::Region3,boost::shared_ptr<RBX::Instance>,int),3>::~BoundFuncDesc()
pub fn stub_0x6d04c0() -> ! {
    todo!("0x6d04c0 RBX::Reflection::BoundFuncDesc<RBX::Workspace,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(RBX::Region3,boost::shared_ptr<RBX::Instance>,int),3>::~BoundFuncDesc()")
}

// 0x6d05d8 — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFbNS_7Region3EN5boost10shared_ptrINS_8InstanceEEEELi2EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,bool ()(RBX::Region3,rbx_core::SharedPtr<RBX::Instance>),2>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Workspace,bool ()(RBX::Region3,boost::shared_ptr<RBX::Instance>),2>::~BoundFuncDesc()
pub fn stub_0x6d05d8() -> ! {
    todo!("0x6d05d8 RBX::Reflection::BoundFuncDesc<RBX::Workspace,bool ()(RBX::Region3,boost::shared_ptr<RBX::Instance>),2>::~BoundFuncDesc()")
}

// 0x6d06e8 — __ZN3RBX9Workspace9getRayHitINS_8InstanceEEEN5boost10shared_ptrIKNS_10Reflection5TupleEEENS_6RbxRayENS4_IT_EEb
#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::Tuple const> RBX::Workspace::getRayHit<RBX::Instance>(RBX::RbxRay,rbx_core::SharedPtr<RBX::Instance>,bool)")]
// was: boost::shared_ptr<RBX::Reflection::Tuple const> RBX::Workspace::getRayHit<RBX::Instance>(RBX::RbxRay,boost::shared_ptr<RBX::Instance>,bool)
pub fn stub_0x6d06e8() -> ! {
    todo!("0x6d06e8 boost::shared_ptr<RBX::Reflection::Tuple const> RBX::Workspace::getRayHit<RBX::Instance>(RBX::RbxRay,boost::shared_ptr<RBX::Instance>,bool)")
}

// 0x6d0a5c — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKNS0_5TupleEEENS_6RbxRayENS4_INS_8InstanceEEEbELi3EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(RBX::RbxRay,rbx_core::SharedPtr<RBX::Instance>,bool),3>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Workspace,boost::shared_ptr<RBX::Reflection::Tuple const> ()(RBX::RbxRay,boost::shared_ptr<RBX::Instance>,bool),3>::~BoundFuncDesc()
pub fn stub_0x6d0a5c() -> ! {
    todo!("0x6d0a5c RBX::Reflection::BoundFuncDesc<RBX::Workspace,boost::shared_ptr<RBX::Reflection::Tuple const> ()(RBX::RbxRay,boost::shared_ptr<RBX::Instance>,bool),3>::~BoundFuncDesc()")
}

// 0x6d0b88 — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_7Region3ESB_iELi3EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(RBX::Region3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,int),3>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Workspace,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(RBX::Region3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>,int),3>::~BoundFuncDesc()
pub fn stub_0x6d0b88() -> ! {
    todo!("0x6d0b88 RBX::Reflection::BoundFuncDesc<RBX::Workspace,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(RBX::Region3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>,int),3>::~BoundFuncDesc()")
}

// 0x6d0ca0 — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFbNS_7Region3EN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEELi2EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,bool ()(RBX::Region3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),2>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Workspace,bool ()(RBX::Region3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),2>::~BoundFuncDesc()
pub fn stub_0x6d0ca0() -> ! {
    todo!("0x6d0ca0 RBX::Reflection::BoundFuncDesc<RBX::Workspace,bool ()(RBX::Region3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),2>::~BoundFuncDesc()")
}

// 0x6d0db0 — __ZN3RBX9Workspace9getRayHitIKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS6_EEEENS4_IKNS_10Reflection5TupleEEENS_6RbxRayENS4_IT_EEb
#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::Tuple const> RBX::Workspace::getRayHit<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>(RBX::RbxRay,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,bool)")]
// was: boost::shared_ptr<RBX::Reflection::Tuple const> RBX::Workspace::getRayHit<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>(RBX::RbxRay,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>,bool)
pub fn stub_0x6d0db0() -> ! {
    todo!("0x6d0db0 boost::shared_ptr<RBX::Reflection::Tuple const> RBX::Workspace::getRayHit<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>(RBX::RbxRay,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>,bool)")
}

// 0x6d1124 — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFN5boost10shared_ptrIKNS0_5TupleEEENS_6RbxRayENS4_IKSt6vectorINS4_INS_8InstanceEEESaISB_EEEEbELi3EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(RBX::RbxRay,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,bool),3>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Workspace,boost::shared_ptr<RBX::Reflection::Tuple const> ()(RBX::RbxRay,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>,bool),3>::~BoundFuncDesc()
pub fn stub_0x6d1124() -> ! {
    todo!("0x6d1124 RBX::Reflection::BoundFuncDesc<RBX::Workspace,boost::shared_ptr<RBX::Reflection::Tuple const> ()(RBX::RbxRay,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>,bool),3>::~BoundFuncDesc()")
}

// 0x6d1250 — __ZN3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEED1Ev
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Workspace,RBX::Instance>::~RefPropDescriptor()")]
// was: RBX::Reflection::RefPropDescriptor<RBX::Workspace,RBX::Instance>::~RefPropDescriptor()
pub fn stub_0x6d1250() -> ! {
    todo!("0x6d1250 RBX::Reflection::RefPropDescriptor<RBX::Workspace,RBX::Instance>::~RefPropDescriptor()")
}

// 0x6d127c — __ZN3RBX9Workspace9doNothingEb
#[doc(alias = "RBX::Workspace::doNothing(bool)")]
// was: RBX::Workspace::doNothing(bool)
pub fn stub_0x6d127c() -> ! {
    todo!("0x6d127c RBX::Workspace::doNothing(bool)")
}

// 0x6d1280 — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFvbELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,void ()(bool),1>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Workspace,void ()(bool),1>::~BoundFuncDesc()
pub fn stub_0x6d1280() -> ! {
    todo!("0x6d1280 RBX::Reflection::BoundFuncDesc<RBX::Workspace,void ()(bool),1>::~BoundFuncDesc()")
}

// 0x6d12c0 — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFvvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,void ()(void),0>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Workspace,void ()(void),0>::~BoundFuncDesc()
pub fn stub_0x6d12c0() -> ! {
    todo!("0x6d12c0 RBX::Reflection::BoundFuncDesc<RBX::Workspace,void ()(void),0>::~BoundFuncDesc()")
}

// 0x6d12e4 — __ZN3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_6CameraEED1Ev
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Workspace,RBX::Camera>::~RefPropDescriptor()")]
// was: RBX::Reflection::RefPropDescriptor<RBX::Workspace,RBX::Camera>::~RefPropDescriptor()
pub fn stub_0x6d12e4() -> ! {
    todo!("0x6d12e4 RBX::Reflection::RefPropDescriptor<RBX::Workspace,RBX::Camera>::~RefPropDescriptor()")
}

// 0x6d1310 — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFdvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,double ()(void),0>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Workspace,double ()(void),0>::~BoundFuncDesc()
pub fn stub_0x6d1310() -> ! {
    todo!("0x6d1310 RBX::Reflection::BoundFuncDesc<RBX::Workspace,double ()(void),0>::~BoundFuncDesc()")
}

// 0x6d1334 — __ZN3RBX10Reflection13BoundFuncDescINS_9WorkspaceEFivELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Workspace,int ()(void),0>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Workspace,int ()(void),0>::~BoundFuncDesc()
pub fn stub_0x6d1334() -> ! {
    todo!("0x6d1334 RBX::Reflection::BoundFuncDesc<RBX::Workspace,int ()(void),0>::~BoundFuncDesc()")
}

