//! rendering high — shard 2 (high EA >= 0xC00000)
//! Filter: Ogre|Gfx|Render|G3D (11144 total, 4927 prior, 100 this batch) — 0xc03acc..0xc8af30
//! Each stub preserves IDA ea + mangled + demangled for rg.
//!
//! IDA-grounded mirror types live below the imports; the small-EA stubs
//! further down delegate to them. Offsets and control flow mirror the ARMv7
//! disassembly/decompilation noted per item.

#![allow(
    non_snake_case,
    dead_code,
    unused_variables,
    unused_imports,
    clippy::all
)]

use std::cell::Cell;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::OnceLock;

use rbx_core::SharedPtr;

use crate::movable::{AxisAlignedBox, Entity as OgreEntity, MovableObject};

/// Ogre 4x4 matrix row-major (`Ogre::Matrix4`, 64 bytes).
pub type Matrix4 = [[f32; 4]; 4];

/// Ogre::StringConverter::parseReal with default 0.0 (0xc6a064).
#[inline]
fn parse_real(text: &str) -> f32 {
    text.parse::<f32>().unwrap_or(0.0)
}

/// RBX::GfxPart — renderable part binding (`GfxBase/GfxPart.h`).
#[derive(Clone, Debug, Default)]
pub struct GfxPart {
    pub fuzzy_min: [f32; 3],
    pub fuzzy_max: [f32; 3],
}

impl GfxPart {
    /// IDA 0xc0b4cc: `MOVS R0,#1; BX LR` — decompile `return 1;`.
    #[inline]
    pub fn part_count(&self) -> u32 {
        1
    }

    /// IDA 0xc0b43c: `ReleaseAssert(false, "false", GfxPart.h:109)` while
    /// `FLog::Asserts`, then min=+maxFinite, max=-maxFinite. No `FLog` in
    /// this crate, so the assert is a `debug_assert`.
    pub fn fast_fuzzy_extents(&mut self) -> [[f32; 3]; 2] {
        debug_assert!(
            false,
            "false file: ../GfxBase/include/GfxBase/GfxPart.h line: 109"
        );
        self.fuzzy_min = [f32::MAX; 3];
        self.fuzzy_max = [-f32::MAX; 3];
        [self.fuzzy_min, self.fuzzy_max]
    }
}

/// Ogre::MaterialPtr — shared material handle (`boost::shared_ptr` →
/// [`SharedPtr`], no boost shims).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MaterialHandle {
    pub name: String,
    pub group: String,
}

/// RBX::RenderEntity — Ogre-mapped renderable (`RbxOgre/RbxRenderNode.cpp`).
///
/// Layout mirrors the ARMv7 original: material slot at `+52`
/// (0xc35838/0xc359ec: `return this + 52`), scene link at `+48` (0xc35aa8).
#[derive(Clone, Debug, Default)]
pub struct RenderEntity {
    pub material: MaterialHandle,
    pub lights: Vec<u32>,
}

impl RenderEntity {
    /// IDA 0xc35838: `return this + 52` — the live material slot.
    #[inline]
    pub fn actual_material(&self) -> &MaterialHandle {
        &self.material
    }

    /// IDA 0xc359ec: `return this + 52` — same slot as the live material.
    #[inline]
    pub fn debug_material(&self) -> &MaterialHandle {
        &self.material
    }

    /// IDA 0xc359e8: `MOVS R0,#0; BX LR` — hardcoded null technique.
    #[inline]
    pub fn technique(&self) -> u32 {
        0
    }

    /// IDA 0xc35aa8: light list reached through the scene link at `+48`.
    #[inline]
    pub fn lights(&self) -> &[u32] {
        &self.lights
    }
}

/// IDA 0xc35980: function-static default `Ogre::MaterialPtr` (guarded init
/// plus `__cxa_atexit` dtor). `OnceLock` runs the init once; Rust statics
/// never run destructors, so there is no manual state to tear down.
static DEFAULT_MATERIAL: OnceLock<MaterialHandle> = OnceLock::new();

/// IDA 0xc35980: returns the process-wide default material.
#[inline]
pub fn default_material() -> &'static MaterialHandle {
    DEFAULT_MATERIAL.get_or_init(MaterialHandle::default)
}

/// RBX::RenderNode — entity container (`RbxOgre/RbxRenderNode.cpp:113,120`).
/// Entity vector at `+464` (`std::vector<RBX::RenderEntity *>`).
#[derive(Clone, Debug, Default)]
pub struct RenderNode {
    pub entities: Vec<SharedPtr<RenderEntity>>,
    pub bounds: AxisAlignedBox,
}

impl RenderNode {
    /// IDA 0xc35d9c: `ReleaseAssert(entity != 0)` (line 113), then
    /// `push_back` (`_M_insert_aux` on the `+464` vector). Non-null is
    /// carried by the type here, so the assert holds by construction.
    pub fn add_entity(&mut self, entity: SharedPtr<RenderEntity>) {
        self.entities.push(entity);
    }

    /// IDA 0xc35e2c: unrolled linear find plus `memmove` tail shift and a
    /// `ReleaseAssert(it != mEntities.end())` (line 120).
    /// // BUG: original at 0xc35e2c — with asserts off and a missing entity
    /// the original shifts nothing yet still shrinks the vector by one;
    /// this port panics instead of corrupting the list.
    pub fn remove_entity(&mut self, entity: &SharedPtr<RenderEntity>) {
        let pos = self
            .entities
            .iter()
            .position(|slot| SharedPtr::ptr_eq(slot, entity))
            .expect("it != mEntities.end() RbxRenderNode.cpp:120");
        self.entities.remove(pos);
    }

    /// IDA 0xc35f64: `Ogre::ToExtents(box at +312)`. The mirror box uses the
    /// same min/max layout, so the conversion is the identity.
    #[inline]
    pub fn fast_fuzzy_extents(&self) -> AxisAlignedBox {
        self.bounds
    }
}

/// Ogre::Animation::InterpolationMode (`OgreAnimation.h`: linear then
/// spline) [INFERENCE: Ogre 1.6 header — IDA shows only store+return].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u32)]
pub enum InterpolationMode {
    #[default]
    Linear = 0,
    Spline = 1,
}

/// Ogre::Animation::msDefaultInterpolationMode (0xc50e64).
static DEFAULT_INTERPOLATION_MODE: AtomicU32 = AtomicU32::new(0);

/// IDA 0xc50e64: stores the mode and returns it.
#[inline]
pub fn set_default_interpolation_mode(mode: InterpolationMode) -> InterpolationMode {
    DEFAULT_INTERPOLATION_MODE.store(mode as u32, Ordering::Relaxed);
    mode
}

/// Ogre::AutoParamDataSource world-matrix view (0xc5738c: pointer at
/// `+16392`, count at `+16388`, dirty byte at `+19280` cleared).
#[derive(Clone, Debug, Default)]
pub struct AutoParamDataSource {
    pub world_matrix_ptr: usize,
    pub world_matrix_count: usize,
    pub world_matrices_dirty: bool,
}

impl AutoParamDataSource {
    /// IDA 0xc5738c: latches the matrix range and clears the dirty flag.
    #[inline]
    pub fn set_world_matrices(&mut self, matrices: &[Matrix4]) {
        self.world_matrix_ptr = matrices.as_ptr() as usize;
        self.world_matrix_count = matrices.len();
        self.world_matrices_dirty = false;
    }
}

/// Ogre::Renderable render-system slot (0xc6eb18: `STR R1,[R0,#44]` from a
/// `const` method — hence the `Cell`).
#[derive(Clone, Debug, Default)]
pub struct RenderableData {
    render_system_data: Cell<usize>,
}

impl RenderableData {
    /// IDA 0xc6eb18: stores the render-system cookie at `+44`.
    #[inline]
    pub fn set_render_system_data(&self, data: usize) {
        self.render_system_data.set(data);
    }

    #[inline]
    pub fn render_system_data(&self) -> usize {
        self.render_system_data.get()
    }
}

/// Ogre::SceneManager::ENTITY_TYPE_MASK (0xc8a67c: double-LDR of the
/// static). Value [INFERENCE: Ogre 1.6 `OgreSceneManager.h`] — IDA bridge
/// read of the static was unavailable when this was written.
pub const ENTITY_TYPE_MASK: u32 = 0x8000_0000;

/// IDA 0xc0dbe0: `sp_counted_impl_p<RenderNode>::dispose` — destroys the
/// owned node when non-null; maps to `Arc` drop (no boost shims).
#[inline]
pub fn dispose_render_node(node: Option<SharedPtr<RenderNode>>) {
    drop(node);
}

/// IDA 0xc8ad84: null-checked scalar-deleting destructor on the movable;
/// maps to `Arc` drop, where `None` is the null case.
#[inline]
pub fn destroy_instance(object: Option<SharedPtr<MovableObject>>) {
    drop(object);
}

/// Ogre::CompositionPass input slot (0xc708cc: `inputs[slot] = (name, index)`
/// with a string copy).
#[derive(Clone, Debug, Default)]
pub struct CompositionPass {
    pub inputs: Vec<(String, u32)>,
}

impl CompositionPass {
    /// IDA 0xc708cc: stores the copied name and index at the slot.
    pub fn set_input(&mut self, slot: usize, name: &str, index: u32) {
        if self.inputs.len() <= slot {
            self.inputs.resize(slot + 1, (String::new(), 0));
        }
        self.inputs[slot] = (name.to_string(), index);
    }
}

/// Ogre::CompositionTargetPass (0xc70ae4: C2 constructor; 0xc70ad8: C1
/// forwards to C2).
#[derive(Clone, Debug, Default)]
pub struct CompositionTargetPass {
    pub technique: usize,
    pub input: u32,
    pub output_name: String,
    pub visibility_mask: u32,
    pub lod_bias: f32,
    pub material_scheme: String,
    pub shadows_enabled: bool,
}

impl CompositionTargetPass {
    /// IDA 0xc70ae4: links the parent technique, zeroes the pass lists,
    /// `visibilityMask = -1`, `lodBias = 1.0`, scheme from the render-system
    /// capabilities when present else `MaterialManager::DEFAULT_SCHEME_NAME`
    /// ("Default" [INFERENCE: Ogre 1.6 header]), byte `+44 = 1`.
    pub fn new(technique: usize, default_scheme: &str) -> Self {
        Self {
            technique,
            input: 0,
            output_name: String::new(),
            visibility_mask: u32::MAX,
            lod_bias: 1.0,
            material_scheme: default_scheme.to_string(),
            shadows_enabled: true,
        }
    }
}

/// Ogre::BorderPanelOverlayElement UV cell rect. Exact getter text follows
/// 0xc68818 (`getCellUVString`, 670 insns); the observed call sites pass
/// cell 5 (bottom-left, 0xc6a554) and cell 7 (bottom-right, 0xc6a7e8).
#[derive(Clone, Copy, Debug, Default)]
pub struct BorderUvCell {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl BorderUvCell {
    #[inline]
    pub fn uv_string(&self) -> String {
        format!("{} {} {} {}", self.left, self.top, self.right, self.bottom)
    }
}

/// Ogre::BorderPanelOverlayElement sizing state (0xc6a064).
#[derive(Clone, Debug, Default)]
pub struct BorderPanelOverlayElement {
    pub cells: [BorderUvCell; 8],
    pub border_size: [f32; 4],
    pub pixel_mode: bool,
    pub geometry_dirty: bool,
}

impl BorderPanelOverlayElement {
    /// IDA 0xc6a064: `split(value, "\t\n ")` then four
    /// `StringConverter::parseReal(part, 0.0)`; pixel mode (`+38` set)
    /// truncates through ushorts (`vcvt_s32_f32`), float mode stores at
    /// `+130..133`; always sets the geometry-dirty byte (`+205 = 1`).
    pub fn set_border_size(&mut self, value: &str) {
        let mut parts = value.split_whitespace().map(parse_real);
        for slot in &mut self.border_size {
            *slot = parts.next().unwrap_or(0.0);
        }
        if self.pixel_mode {
            for slot in &mut self.border_size {
                *slot = (*slot as i32) as f32;
            }
        }
        self.geometry_dirty = true;
    }

    /// IDA 0xc6a554: `getCellUVString(cell 5)` (bottom-left).
    #[inline]
    pub fn border_bottom_left_uv(&self) -> String {
        self.cells[5].uv_string()
    }

    /// IDA 0xc6a7e8: `getCellUVString(cell 7)` (bottom-right).
    #[inline]
    pub fn border_bottom_right_uv(&self) -> String {
        self.cells[7].uv_string()
    }
}

/// Ogre::DefaultHardwareVertexBuffer CPU-side store
/// (0xc7f2cc: `memcpy(dst, base + offset, len)`).
#[derive(Clone, Debug, Default)]
pub struct DefaultHardwareVertexBuffer {
    pub data: Vec<u8>,
}

impl DefaultHardwareVertexBuffer {
    /// IDA 0xc7f2cc: copies `[offset, offset + dst.len())` into `dst`.
    #[inline]
    pub fn read_data(&self, offset: usize, dst: &mut [u8]) {
        dst.copy_from_slice(&self.data[offset..offset + dst.len()]);
    }
}

/// Ogre::HardwareIndexBufferSharedPtr payload (0xc8a5e0 target).
#[derive(Clone, Debug, Default)]
pub struct HardwareIndexBuffer {
    pub indices: Vec<u16>,
}

/// Ogre::Entity::EntityShadowRenderable (0xc8a580/0xc8a5c8/0xc8a5e0).
#[derive(Clone, Debug, Default)]
pub struct EntityShadowRenderable {
    pub parent_world: Matrix4,
    pub parent_visible: Option<bool>,
    pub index_buffer: Option<SharedPtr<HardwareIndexBuffer>>,
}

impl EntityShadowRenderable {
    /// IDA 0xc8a580: copies the parent world matrix (64 bytes) to the caller.
    #[inline]
    pub fn world_transform(&self) -> Matrix4 {
        self.parent_world
    }

    /// IDA 0xc8a5c8: parent `isVisible` (`+35` vtable call), default true
    /// when unbound.
    #[inline]
    pub fn is_visible(&self) -> bool {
        self.parent_visible.unwrap_or(true)
    }

    /// IDA 0xc8a5e0: rebinds the shared index buffer.
    #[inline]
    pub fn rebind_index_buffer(&mut self, buffer: Option<SharedPtr<HardwareIndexBuffer>>) {
        self.index_buffer = buffer;
    }
}

// 0xc03acc — __ZN19ResourceGroupHelper23updateOnEveryRenderableEv
#[doc(alias = "ResourceGroupHelper::updateOnEveryRenderable(void)")]
// was: ResourceGroupHelper::updateOnEveryRenderable(void)
// IDA 0xc03acc: 294 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c03acc() {
}

// 0xc04c88 — __ZN19ResourceGroupHelper31UpdateMaterialRenderableVisitorD1Ev
#[doc(
    alias = "ResourceGroupHelper::UpdateMaterialRenderableVisitor::~UpdateMaterialRenderableVisitor()"
)]
// was: ResourceGroupHelper::UpdateMaterialRenderableVisitor::~UpdateMaterialRenderableVisitor()
// IDA 0xc04c88: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c04c88() {
}

// 0xc04c8c — __ZN19ResourceGroupHelper31UpdateMaterialRenderableVisitorD0Ev
#[doc(
    alias = "ResourceGroupHelper::UpdateMaterialRenderableVisitor::~UpdateMaterialRenderableVisitor()"
)]
// was: ResourceGroupHelper::UpdateMaterialRenderableVisitor::~UpdateMaterialRenderableVisitor()
// IDA 0xc04c8c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_c04c8c() {
}

// 0xc08ddc — __ZN3RBX11MegaCluster19createSolidGeometryEPNS_10RenderNodeERKNS_13SpatialRegion2IdEPj
#[doc(
    alias = "RBX::MegaCluster::createSolidGeometry(RBX::RenderNode *,RBX::SpatialRegion::Id const&,unsigned int *)"
)]
// was: RBX::MegaCluster::createSolidGeometry(RBX::RenderNode *,RBX::SpatialRegion::Id const&,unsigned int *)
// IDA 0xc08ddc: 208 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c08ddc() {
}

// 0xc08fec — __ZN3RBX11MegaCluster19createWaterGeometryEPNS_10RenderNodeERKNS_13SpatialRegion2IdEPj
#[doc(
    alias = "RBX::MegaCluster::createWaterGeometry(RBX::RenderNode *,RBX::SpatialRegion::Id const&,unsigned int *)"
)]
// was: RBX::MegaCluster::createWaterGeometry(RBX::RenderNode *,RBX::SpatialRegion::Id const&,unsigned int *)
// IDA 0xc08fec: 230 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c08fec() {
}

// 0xc0a4ec — __ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_20SolidTerrainRendererIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(
    alias = "RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)"
)]
// was: RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)
// IDA 0xc0a4ec: 759 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c0a4ec() {
}

// 0xc0acec — __ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_17WaterFaceRendererIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(
    alias = "RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::WaterFaceRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)"
)]
// was: RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::WaterFaceRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)
// IDA 0xc0acec: 696 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c0acec() {
}

// 0xc0b430 — __ZN3RBX10GfxBinding16invalidateEntityEv
#[doc(alias = "RBX::GfxBinding::invalidateEntity(void)")]
// was: RBX::GfxBinding::invalidateEntity(void)
// IDA 0xc0b430: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c0b430() {
}

// 0xc0b434 — __ZN3RBX10GfxBinding24onCoordinateFrameChangedEv
#[doc(alias = "RBX::GfxBinding::onCoordinateFrameChanged(void)")]
// was: RBX::GfxBinding::onCoordinateFrameChanged(void)
// IDA 0xc0b434: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c0b434() {
}

// 0xc0b438 — __ZN3RBX7GfxPart21updateCoordinateFrameEb
#[doc(alias = "RBX::GfxPart::updateCoordinateFrame(bool)")]
// was: RBX::GfxPart::updateCoordinateFrame(bool)
// IDA 0xc0b438: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c0b438() {
}

// 0xc0b43c — __ZN3RBX7GfxPart19getFastFuzzyExtentsEv
#[doc(alias = "RBX::GfxPart::getFastFuzzyExtents(void)")]
// was: RBX::GfxPart::getFastFuzzyExtents(void)
// IDA 0xc0b43c: decompile asserts `false` (GfxPart.h:109) under FLog::Asserts, then min=+maxFinite, max=-maxFinite.
pub fn stub_c0b43c(part: &mut GfxPart) -> [[f32; 3]; 2] {
    part.fast_fuzzy_extents()
}

// 0xc0b4cc — __ZN3RBX7GfxPart12getPartCountEv
#[doc(alias = "RBX::GfxPart::getPartCount(void)")]
// was: RBX::GfxPart::getPartCount(void)
// IDA 0xc0b4cc: `MOVS R0,#1; BX LR` — decompile `return 1;`.
pub fn stub_c0b4cc(part: &GfxPart) -> u32 {
    part.part_count()
}

// 0xc0b4d0 — __ZN3RBX7GfxPart17onSleepingChangedEbPNS_12PartInstanceE
#[doc(alias = "RBX::GfxPart::onSleepingChanged(bool,RBX::PartInstance *)")]
// was: RBX::GfxPart::onSleepingChanged(bool,RBX::PartInstance *)
// IDA 0xc0b4d0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c0b4d0() {
}

// 0xc0b4d4 — __ZN3RBX7GfxPart14onClumpChangedEv
#[doc(alias = "RBX::GfxPart::onClumpChanged(void)")]
// was: RBX::GfxPart::onClumpChanged(void)
// IDA 0xc0b4d4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c0b4d4() {
}

// 0xc0b4d8 — __ZNK3RBX20WaterRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE8internalERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionE
#[doc(
    alias = "RBX::WaterRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>::internal(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection)const"
)]
// was: RBX::WaterRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>::internal(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection)const
// IDA 0xc0b4d8: 139 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c0b4d8() {
}

// 0xc0b66c — __ZN3RBX17WaterFaceRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE5applyERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionENS_16RenderPredStatusE
#[doc(
    alias = "RBX::WaterFaceRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)"
)]
// was: RBX::WaterFaceRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)
// IDA 0xc0b66c: 274 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c0b66c() {
}

// 0xc0bf18 — __ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_11FaceCounterIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(
    alias = "RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::FaceCounter<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)"
)]
// was: RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::FaceCounter<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)
// IDA 0xc0bf18: 683 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c0bf18() {
}

// 0xc0c648 — __ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE5applyERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionENS_16RenderPredStatusE
#[doc(
    alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)"
)]
// was: RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)
// IDA 0xc0c648: 218 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c0c648() {
}

// 0xc0c904 — __ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE9wedgeFaceERKNS1_6RegionINS3_5ChunkEE8iteratorE
#[doc(
    alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::wedgeFace(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)"
)]
// was: RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::wedgeFace(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)
// IDA 0xc0c904: 330 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c0c904() {
}

// 0xc0cd30 — __ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE19detectWedgeOutlinesERKNS1_6RegionINS3_5ChunkEE8iteratorE
#[doc(
    alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::detectWedgeOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)"
)]
// was: RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::detectWedgeOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)
// IDA 0xc0cd30: 177 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c0cd30() {
}

// 0xc0cf1c — __ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE12wedgeUpEmptyERKNS1_6RegionINS3_5ChunkEE8iteratorE
#[doc(
    alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::wedgeUpEmpty(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)"
)]
// was: RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::wedgeUpEmpty(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)
// IDA 0xc0cf1c: 78 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c0cf1c() {
}

// 0xc0d000 — __ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE14detectOutlinesERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionENS_16RenderPredStatusE
#[doc(
    alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::detectOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)"
)]
// was: RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::detectOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)
// IDA 0xc0d000: 138 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c0d000() {
}

// 0xc0d418 — __ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_11FaceCounterIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(
    alias = "RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::FaceCounter<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)"
)]
// was: RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::FaceCounter<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)
// IDA 0xc0d418: 728 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c0d418() {
}

// 0xc0dbd8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::RenderNode>::~sp_counted_impl_p()")]
// was: boost::detail::sp_counted_impl_p<RBX::RenderNode>::~sp_counted_impl_p()
// IDA 0xc0dbd8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c0dbd8() {
}

// 0xc0dbdc — __ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::RenderNode>::~sp_counted_impl_p()")]
// was: boost::detail::sp_counted_impl_p<RBX::RenderNode>::~sp_counted_impl_p()
// IDA 0xc0dbdc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_c0dbdc() {
}

// 0xc0dbe0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::RenderNode>::dispose(void)")]
// was: boost::detail::sp_counted_impl_p<RBX::RenderNode>::dispose(void)
// IDA 0xc0dbe0: `dispose` destroys `*(a1 + 12)` via vtable `+24` when non-null; maps to `Arc` drop.
pub fn stub_c0dbe0(node: Option<SharedPtr<RenderNode>>) {
    dispose_render_node(node);
}

// 0xc0dbf0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEE11get_deleterERKSt9type_info
#[doc(
    alias = "boost::detail::sp_counted_impl_p<RBX::RenderNode>::get_deleter(std::type_info const&)"
)]
// was: boost::detail::sp_counted_impl_p<RBX::RenderNode>::get_deleter(std::type_info const&)
// IDA 0xc0dbf0: `get_deleter` returns 0 — `Arc` carries no user deleter.
pub fn stub_c0dbf0() -> Option<usize> {
    None
}

// 0xc0dbf4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::RenderNode>::get_untyped_deleter(void)")]
// was: boost::detail::sp_counted_impl_p<RBX::RenderNode>::get_untyped_deleter(void)
// IDA 0xc0dbf4: `get_untyped_deleter` returns 0 — `Arc` carries no user deleter.
pub fn stub_c0dbf4() -> Option<usize> {
    None
}

// 0xc10528 — __ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_19MegaClusterInstanceEEENS_20SolidTerrainRendererIS2_EES2_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(
    alias = "RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::MegaClusterInstance>,RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>,RBX::MegaClusterInstance>::handleCells(RBX::SpatialRegion::Id const&)"
)]
// was: RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::MegaClusterInstance>,RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>,RBX::MegaClusterInstance>::handleCells(RBX::SpatialRegion::Id const&)
// IDA 0xc10528: 738 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c10528() {
}

// 0xc10cec — __ZN3RBX20SolidTerrainRendererINS_19MegaClusterInstanceEE5applyERKNS_5Voxel6RegionINS1_9CellChunkEE8iteratorENS3_13FaceDirectionENS_16RenderPredStatusE
#[doc(
    alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::apply(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)"
)]
// was: RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::apply(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)
// IDA 0xc10cec: 235 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c10cec() {
}

// 0xc10fb8 — __ZN3RBX20SolidTerrainRendererINS_19MegaClusterInstanceEE9wedgeFaceERKNS_5Voxel6RegionINS1_9CellChunkEE8iteratorE
#[doc(
    alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::wedgeFace(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&)"
)]
// was: RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::wedgeFace(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&)
// IDA 0xc10fb8: 333 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c10fb8() {
}

// 0xc113f0 — __ZN3RBX20SolidTerrainRendererINS_19MegaClusterInstanceEE19detectWedgeOutlinesERKNS_5Voxel6RegionINS1_9CellChunkEE8iteratorE
#[doc(
    alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::detectWedgeOutlines(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&)"
)]
// was: RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::detectWedgeOutlines(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&)
// IDA 0xc113f0: 202 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c113f0() {
}

// 0xc11604 — __ZN3RBX20SolidTerrainRendererINS_19MegaClusterInstanceEE12wedgeUpEmptyERKNS_5Voxel6RegionINS1_9CellChunkEE8iteratorE
#[doc(
    alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::wedgeUpEmpty(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&)"
)]
// was: RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::wedgeUpEmpty(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&)
// IDA 0xc11604: 84 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c11604() {
}

// 0xc116f4 — __ZN3RBX20SolidTerrainRendererINS_19MegaClusterInstanceEE14detectOutlinesERKNS_5Voxel6RegionINS1_9CellChunkEE8iteratorENS3_13FaceDirectionENS_16RenderPredStatusE
#[doc(
    alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::detectOutlines(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)"
)]
// was: RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::detectOutlines(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)
// IDA 0xc116f4: 147 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c116f4() {
}

// 0xc11b14 — __ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_19MegaClusterInstanceEEENS_11FaceCounterIS2_EES2_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(
    alias = "RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::MegaClusterInstance>,RBX::FaceCounter<RBX::MegaClusterInstance>,RBX::MegaClusterInstance>::handleCells(RBX::SpatialRegion::Id const&)"
)]
// was: RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::MegaClusterInstance>,RBX::FaceCounter<RBX::MegaClusterInstance>,RBX::MegaClusterInstance>::handleCells(RBX::SpatialRegion::Id const&)
// IDA 0xc11b14: 709 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c11b14() {
}

// 0xc12294 — __ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_19MegaClusterInstanceEEENS_17WaterFaceRendererIS2_EES2_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(
    alias = "RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::MegaClusterInstance>,RBX::WaterFaceRenderer<RBX::MegaClusterInstance>,RBX::MegaClusterInstance>::handleCells(RBX::SpatialRegion::Id const&)"
)]
// was: RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::MegaClusterInstance>,RBX::WaterFaceRenderer<RBX::MegaClusterInstance>,RBX::MegaClusterInstance>::handleCells(RBX::SpatialRegion::Id const&)
// IDA 0xc12294: 670 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c12294() {
}

// 0xc129a8 — __ZNK3RBX20WaterRenderPredicateINS_19MegaClusterInstanceEE8internalERKNS_5Voxel6RegionINS1_9CellChunkEE8iteratorENS3_13FaceDirectionE
#[doc(
    alias = "RBX::WaterRenderPredicate<RBX::MegaClusterInstance>::internal(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&,RBX::Voxel::FaceDirection)const"
)]
// was: RBX::WaterRenderPredicate<RBX::MegaClusterInstance>::internal(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&,RBX::Voxel::FaceDirection)const
// IDA 0xc129a8: 138 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c129a8() {
}

// 0xc12b3c — __ZN3RBX17WaterFaceRendererINS_19MegaClusterInstanceEE5applyERKNS_5Voxel6RegionINS1_9CellChunkEE8iteratorENS3_13FaceDirectionENS_16RenderPredStatusE
#[doc(
    alias = "RBX::WaterFaceRenderer<RBX::MegaClusterInstance>::apply(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)"
)]
// was: RBX::WaterFaceRenderer<RBX::MegaClusterInstance>::apply(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)
// IDA 0xc12b3c: 274 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c12b3c() {
}

// 0xc12ee0 — __ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_19MegaClusterInstanceEEENS_11FaceCounterIS2_EES2_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(
    alias = "RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::MegaClusterInstance>,RBX::FaceCounter<RBX::MegaClusterInstance>,RBX::MegaClusterInstance>::handleCells(RBX::SpatialRegion::Id const&)"
)]
// was: RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::MegaClusterInstance>,RBX::FaceCounter<RBX::MegaClusterInstance>,RBX::MegaClusterInstance>::handleCells(RBX::SpatialRegion::Id const&)
// IDA 0xc12ee0: 660 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c12ee0() {
}

// 0xc14f78 — __ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_20SolidTerrainRendererIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(
    alias = "RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)"
)]
// was: RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)
// IDA 0xc14f78: 761 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c14f78() {
}

// 0xc15780 — __ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE5applyERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionENS_16RenderPredStatusE
#[doc(
    alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)"
)]
// was: RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)
// IDA 0xc15780: 218 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c15780() {
}

// 0xc15a3c — __ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE9wedgeFaceERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorE
#[doc(
    alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::wedgeFace(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)"
)]
// was: RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::wedgeFace(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)
// IDA 0xc15a3c: 330 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c15a3c() {
}

// 0xc15e68 — __ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE19detectWedgeOutlinesERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorE
#[doc(
    alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::detectWedgeOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)"
)]
// was: RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::detectWedgeOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)
// IDA 0xc15e68: 177 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c15e68() {
}

// 0xc16054 — __ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE12wedgeUpEmptyERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorE
#[doc(
    alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::wedgeUpEmpty(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)"
)]
// was: RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::wedgeUpEmpty(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)
// IDA 0xc16054: 78 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c16054() {
}

// 0xc16138 — __ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE14detectOutlinesERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionENS_16RenderPredStatusE
#[doc(
    alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::detectOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)"
)]
// was: RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::detectOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)
// IDA 0xc16138: 138 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c16138() {
}

// 0xc16550 — __ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_11FaceCounterIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(
    alias = "RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::FaceCounter<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)"
)]
// was: RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::FaceCounter<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)
// IDA 0xc16550: 730 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c16550() {
}

// 0xc16d18 — __ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_17WaterFaceRendererIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(
    alias = "RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::WaterFaceRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)"
)]
// was: RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::WaterFaceRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)
// IDA 0xc16d18: 698 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c16d18() {
}

// 0xc17464 — __ZNK3RBX20WaterRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEE8internalERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionE
#[doc(
    alias = "RBX::WaterRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>::internal(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection)const"
)]
// was: RBX::WaterRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>::internal(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection)const
// IDA 0xc17464: 139 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c17464() {
}

// 0xc175f8 — __ZN3RBX17WaterFaceRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE5applyERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionENS_16RenderPredStatusE
#[doc(
    alias = "RBX::WaterFaceRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)"
)]
// was: RBX::WaterFaceRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)
// IDA 0xc175f8: 274 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c175f8() {
}

// 0xc1799c — __ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_11FaceCounterIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(
    alias = "RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::FaceCounter<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)"
)]
// was: RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::FaceCounter<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)
// IDA 0xc1799c: 685 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c1799c() {
}

// 0xc18ea8 — __ZN3RBX10GfxBinding12updateEntityEb
#[doc(alias = "RBX::GfxBinding::updateEntity(bool)")]
// was: RBX::GfxBinding::updateEntity(bool)
// IDA 0xc18ea8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c18ea8() {
}

// 0xc35418 — __ZN3RBX12RenderEntityD0Ev
#[doc(alias = "RBX::RenderEntity::~RenderEntity()")]
// was: RBX::RenderEntity::~RenderEntity()
// IDA 0xc35418: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c35418() {
}

// 0xc354b8 — __ZN3RBX12RenderEntityD1Ev
#[doc(alias = "RBX::RenderEntity::~RenderEntity()")]
// was: RBX::RenderEntity::~RenderEntity()
// IDA 0xc354b8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_c354b8() {
}

// 0xc354bc — __ZN3RBX12RenderEntityD2Ev
#[doc(alias = "RBX::RenderEntity::~RenderEntity()")]
// was: RBX::RenderEntity::~RenderEntity()
// IDA 0xc354bc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c354bc() {
}

// 0xc35838 — __ZNK3RBX12RenderEntity17getActualMaterialEv
#[doc(alias = "RBX::RenderEntity::getActualMaterial(void)const")]
// was: RBX::RenderEntity::getActualMaterial(void)const
// IDA 0xc35838: `return (char *)this + 52` — the live material slot.
pub fn stub_c35838(entity: &RenderEntity) -> &MaterialHandle {
    entity.actual_material()
}

// 0xc35980 — __ZNK3RBX12RenderEntity11getMaterialEv
#[doc(alias = "RBX::RenderEntity::getMaterial(void)const")]
// was: RBX::RenderEntity::getMaterial(void)const
// IDA 0xc35980: guarded function-static default `Ogre::MaterialPtr` (`__cxa_atexit` dtor); `OnceLock` init-once map.
pub fn stub_c35980() -> &'static MaterialHandle {
    default_material()
}

// 0xc359e8 — __ZNK3RBX12RenderEntity12getTechniqueEv
#[doc(alias = "RBX::RenderEntity::getTechnique(void)const")]
// was: RBX::RenderEntity::getTechnique(void)const
// IDA 0xc359e8: `MOVS R0,#0; BX LR` — hardcoded null technique.
pub fn stub_c359e8(entity: &RenderEntity) -> u32 {
    entity.technique()
}

// 0xc359ec — __ZNK3RBX12RenderEntity16getDebugMaterialEv
#[doc(alias = "RBX::RenderEntity::getDebugMaterial(void)const")]
// was: RBX::RenderEntity::getDebugMaterial(void)const
// IDA 0xc359ec: `return (char *)this + 52` — same slot as the live material.
pub fn stub_c359ec(entity: &RenderEntity) -> &MaterialHandle {
    entity.debug_material()
}

// 0xc35aa8 — __ZNK3RBX12RenderEntity9getLightsEv
#[doc(alias = "RBX::RenderEntity::getLights(void)const")]
// was: RBX::RenderEntity::getLights(void)const
// IDA 0xc35aa8: light list via the scene link (`*(this + 12) + 308`, vtable `+328`).
pub fn stub_c35aa8(entity: &RenderEntity) -> &[u32] {
    entity.lights()
}

// 0xc35b20 — __ZN3RBX10RenderNodeD0Ev
#[doc(alias = "RBX::RenderNode::~RenderNode()")]
// was: RBX::RenderNode::~RenderNode()
// IDA 0xc35b20: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c35b20() {
}

// 0xc35bd4 — __ZN3RBX10RenderNodeD1Ev
#[doc(alias = "RBX::RenderNode::~RenderNode()")]
// was: RBX::RenderNode::~RenderNode()
// IDA 0xc35bd4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_c35bd4() {
}

// 0xc35bd8 — __ZN3RBX10RenderNodeD2Ev
#[doc(alias = "RBX::RenderNode::~RenderNode()")]
// was: RBX::RenderNode::~RenderNode()
// IDA 0xc35bd8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c35bd8() {
}

// 0xc35d9c — __ZN3RBX10RenderNode9addEntityEPNS_12RenderEntityE
#[doc(alias = "RBX::RenderNode::addEntity(RBX::RenderEntity *)")]
// was: RBX::RenderNode::addEntity(RBX::RenderEntity *)
// IDA 0xc35d9c: `ReleaseAssert(entity)` (RbxRenderNode.cpp:113) then `push_back` on the `+464` vector.
pub fn stub_c35d9c(node: &mut RenderNode, entity: SharedPtr<RenderEntity>) {
    node.add_entity(entity);
}

// 0xc35e2c — __ZN3RBX10RenderNode12removeEntityEPNS_12RenderEntityE
#[doc(alias = "RBX::RenderNode::removeEntity(RBX::RenderEntity *)")]
// was: RBX::RenderNode::removeEntity(RBX::RenderEntity *)
// IDA 0xc35e2c: unrolled find plus `memmove` shift; `ReleaseAssert(it != end)` (RbxRenderNode.cpp:120).
pub fn stub_c35e2c(node: &mut RenderNode, entity: &SharedPtr<RenderEntity>) {
    node.remove_entity(entity);
}

// 0xc35f64 — __ZN3RBX10RenderNode19getFastFuzzyExtentsEv
#[doc(alias = "RBX::RenderNode::getFastFuzzyExtents(void)")]
// was: RBX::RenderNode::getFastFuzzyExtents(void)
// IDA 0xc35f64: `Ogre::ToExtents(box at +312)`; mirror layout makes it the identity.
pub fn stub_c35f64(node: &RenderNode) -> AxisAlignedBox {
    node.fast_fuzzy_extents()
}

// 0xc35f74 — __ZThn392_N3RBX10RenderNode19getFastFuzzyExtentsEv
#[doc(alias = "non-virtual thunk to RBX::RenderNode::getFastFuzzyExtents(void)")]
// was: non-virtual thunk to RBX::RenderNode::getFastFuzzyExtents(void)
// IDA 0xc35f74: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c35f74() {
}

// 0xc35f80 — __ZN3RBX10RenderNode13_updateBoundsEv
#[doc(alias = "RBX::RenderNode::_updateBounds(void)")]
// was: RBX::RenderNode::_updateBounds(void)
// IDA 0xc35f80: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c35f80() {
}

// 0xc3602c — __ZNSt6vectorIPN3RBX12RenderEntityESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(
    alias = "std::vector<RBX::RenderEntity *,std::allocator<RBX::RenderEntity *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::RenderEntity **,std::vector<RBX::RenderEntity *,std::allocator<RBX::RenderEntity *>>>,RBX::RenderEntity * const&)"
)]
// was: std::vector<RBX::RenderEntity *,std::allocator<RBX::RenderEntity *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::RenderEntity **,std::vector<RBX::RenderEntity *,std::allocator<RBX::RenderEntity *>>>,RBX::RenderEntity * const&)
// IDA 0xc3602c: 85 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c3602c() {
}

// 0xc50e64 — __ZN4Ogre9Animation27setDefaultInterpolationModeENS0_17InterpolationModeE
#[doc(alias = "Ogre::Animation::setDefaultInterpolationMode(Ogre::Animation::InterpolationMode)")]
// was: Ogre::Animation::setDefaultInterpolationMode(Ogre::Animation::InterpolationMode)
// IDA 0xc50e64: `msDefaultInterpolationMode = arg; return arg;`.
pub fn stub_c50e64(mode: InterpolationMode) -> InterpolationMode {
    set_default_interpolation_mode(mode)
}

// 0xc5738c — __ZN4Ogre19AutoParamDataSource16setWorldMatricesEPKNS_7Matrix4Em
#[doc(alias = "Ogre::AutoParamDataSource::setWorldMatrices(Ogre::Matrix4 const*,unsigned long)")]
// was: Ogre::AutoParamDataSource::setWorldMatrices(Ogre::Matrix4 const*,unsigned long)
// IDA 0xc5738c: pointer at `+16392`, count at `+16388`, dirty byte at `+19280` cleared.
pub fn stub_c5738c(source: &mut AutoParamDataSource, matrices: &[Matrix4]) {
    source.set_world_matrices(matrices);
}

// 0xc6a064 — __ZN4Ogre25BorderPanelOverlayElement13CmdBorderSize5doSetEPvRKSs
#[doc(alias = "Ogre::BorderPanelOverlayElement::CmdBorderSize::doSet(void *,std::string const&)")]
// was: Ogre::BorderPanelOverlayElement::CmdBorderSize::doSet(void *,std::string const&)
// IDA 0xc6a064: whitespace split, four `parseReal` (default 0.0), ushort/float branch, dirty byte `+205 = 1`.
pub fn stub_c6a064(element: &mut BorderPanelOverlayElement, value: &str) {
    element.set_border_size(value);
}

// 0xc6a554 — __ZNK4Ogre25BorderPanelOverlayElement21CmdBorderBottomLeftUV5doGetEPKv
#[doc(alias = "Ogre::BorderPanelOverlayElement::CmdBorderBottomLeftUV::doGet(void const*)const")]
// was: Ogre::BorderPanelOverlayElement::CmdBorderBottomLeftUV::doGet(void const*)const
// IDA 0xc6a554: `getCellUVString(cell 5)` (bottom-left).
pub fn stub_c6a554(element: &BorderPanelOverlayElement) -> String {
    element.border_bottom_left_uv()
}

// 0xc6a7e8 — __ZNK4Ogre25BorderPanelOverlayElement22CmdBorderBottomRightUV5doGetEPKv
#[doc(alias = "Ogre::BorderPanelOverlayElement::CmdBorderBottomRightUV::doGet(void const*)const")]
// was: Ogre::BorderPanelOverlayElement::CmdBorderBottomRightUV::doGet(void const*)const
// IDA 0xc6a7e8: `getCellUVString(cell 7)` (bottom-right).
pub fn stub_c6a7e8(element: &BorderPanelOverlayElement) -> String {
    element.border_bottom_right_uv()
}

// 0xc6ba1c — __ZN4Ogre25BorderPanelOverlayElement21CmdBorderBottomLeftUVD1Ev
#[doc(alias = "Ogre::BorderPanelOverlayElement::CmdBorderBottomLeftUV::~CmdBorderBottomLeftUV()")]
// was: Ogre::BorderPanelOverlayElement::CmdBorderBottomLeftUV::~CmdBorderBottomLeftUV()
// IDA 0xc6ba1c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c6ba1c() {
}

// 0xc6ba7c — __ZN4Ogre25BorderPanelOverlayElement21CmdBorderBottomLeftUVD0Ev
#[doc(alias = "Ogre::BorderPanelOverlayElement::CmdBorderBottomLeftUV::~CmdBorderBottomLeftUV()")]
// was: Ogre::BorderPanelOverlayElement::CmdBorderBottomLeftUV::~CmdBorderBottomLeftUV()
// IDA 0xc6ba7c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c6ba7c() {
}

// 0xc6eb18 — __ZNK4Ogre10Renderable19setRenderSystemDataEPNS0_16RenderSystemDataE
#[doc(alias = "Ogre::Renderable::setRenderSystemData(Ogre::Renderable::RenderSystemData *)const")]
// was: Ogre::Renderable::setRenderSystemData(Ogre::Renderable::RenderSystemData *)const
// IDA 0xc6eb18: `STR R1,[R0,#44]` — const setter over the `Cell` slot.
pub fn stub_c6eb18(renderable: &RenderableData, data: usize) {
    renderable.set_render_system_data(data);
}

// 0xc708cc — __ZN4Ogre15CompositionPass8setInputEmRKSsm
#[doc(alias = "Ogre::CompositionPass::setInput(unsigned long,std::string const&,unsigned long)")]
// was: Ogre::CompositionPass::setInput(unsigned long,std::string const&,unsigned long)
// IDA 0xc708cc: `inputs[slot] = (copied name, index)`.
pub fn stub_c708cc(pass: &mut CompositionPass, slot: usize, name: &str, index: u32) {
    pass.set_input(slot, name, index);
}

// 0xc70ad8 — __ZN4Ogre21CompositionTargetPassC1EPNS_20CompositionTechniqueE
#[doc(alias = "Ogre::CompositionTargetPass::CompositionTargetPass(Ogre::CompositionTechnique *)")]
// was: Ogre::CompositionTargetPass::CompositionTargetPass(Ogre::CompositionTechnique *)
// IDA 0xc70ad8: C1 delegates to the C2 constructor below.
pub fn stub_c70ad8(technique: usize, default_scheme: &str) -> CompositionTargetPass {
    stub_c70ae4(technique, default_scheme)
}

// 0xc70ae4 — __ZN4Ogre21CompositionTargetPassC2EPNS_20CompositionTechniqueE
#[doc(alias = "Ogre::CompositionTargetPass::CompositionTargetPass(Ogre::CompositionTechnique *)")]
// was: Ogre::CompositionTargetPass::CompositionTargetPass(Ogre::CompositionTechnique *)
// IDA 0xc70ae4: C2 links the technique, zeroes lists, mask -1, lodBias 1.0, render-system scheme or default.
pub fn stub_c70ae4(technique: usize, default_scheme: &str) -> CompositionTargetPass {
    CompositionTargetPass::new(technique, default_scheme)
}

// 0xc7967c — __ZN4Ogre17ControllerManager23createTextureUVScrollerEPNS_16TextureUnitStateEf
#[doc(alias = "Ogre::ControllerManager::createTextureUVScroller(Ogre::TextureUnitState *,float)")]
// was: Ogre::ControllerManager::createTextureUVScroller(Ogre::TextureUnitState *,float)
// IDA 0xc7967c: 327 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7967c() {
}

// 0xc7998c — __ZN4Ogre17ControllerManager22createTextureUScrollerEPNS_16TextureUnitStateEf
#[doc(alias = "Ogre::ControllerManager::createTextureUScroller(Ogre::TextureUnitState *,float)")]
// was: Ogre::ControllerManager::createTextureUScroller(Ogre::TextureUnitState *,float)
// IDA 0xc7998c: 327 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7998c() {
}

// 0xc79c9c — __ZN4Ogre17ControllerManager22createTextureVScrollerEPNS_16TextureUnitStateEf
#[doc(alias = "Ogre::ControllerManager::createTextureVScroller(Ogre::TextureUnitState *,float)")]
// was: Ogre::ControllerManager::createTextureVScroller(Ogre::TextureUnitState *,float)
// IDA 0xc79c9c: 327 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c79c9c() {
}

// 0xc7f2cc — __ZN4Ogre27DefaultHardwareVertexBuffer8readDataEmmPv
#[doc(alias = "Ogre::DefaultHardwareVertexBuffer::readData(unsigned long,unsigned long,void *)")]
// was: Ogre::DefaultHardwareVertexBuffer::readData(unsigned long,unsigned long,void *)
// IDA 0xc7f2cc: `memcpy(dst, base + offset, len)` over the CPU store.
pub fn stub_c7f2cc(buffer: &DefaultHardwareVertexBuffer, offset: usize, dst: &mut [u8]) {
    buffer.read_data(offset, dst);
}

// 0xc85874 — __ZThn188_N4Ogre6Entity25backgroundLoadingCompleteEPNS_8ResourceE
#[doc(alias = "non-virtual thunk toOgre::Entity::backgroundLoadingComplete(Ogre::Resource *)")]
// was: non-virtual thunk to Ogre::Entity::backgroundLoadingComplete(Ogre::Resource *)
// IDA 0xc85874: non-virtual thunk — `this - 188` to `Entity`; `_initialise` when the finished resource matches.
pub fn stub_c85874(entity: &mut OgreEntity, resource_mesh: &str) -> bool {
    entity.background_loading_complete(resource_mesh)
}

// 0xc8a3e0 — __ZN4Ogre6Entity22EntityShadowRenderableD2Ev
#[doc(alias = "Ogre::Entity::EntityShadowRenderable::~EntityShadowRenderable()")]
// was: Ogre::Entity::EntityShadowRenderable::~EntityShadowRenderable()
// IDA 0xc8a3e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c8a3e0() {
}

// 0xc8a580 — __ZNK4Ogre6Entity22EntityShadowRenderable18getWorldTransformsEPNS_7Matrix4E
#[doc(alias = "Ogre::Entity::EntityShadowRenderable::getWorldTransforms(Ogre::Matrix4 *)const")]
// was: Ogre::Entity::EntityShadowRenderable::getWorldTransforms(Ogre::Matrix4 *)const
// IDA 0xc8a580: copies the 64-byte parent world matrix to the caller (original returns the out pointer + 2).
pub fn stub_c8a580(shadow: &EntityShadowRenderable, out: &mut Matrix4) {
    *out = shadow.world_transform();
}

// 0xc8a5c8 — __ZNK4Ogre6Entity22EntityShadowRenderable9isVisibleEv
#[doc(alias = "Ogre::Entity::EntityShadowRenderable::isVisible(void)const")]
// was: Ogre::Entity::EntityShadowRenderable::isVisible(void)const
// IDA 0xc8a5c8: parent `isVisible` at `+35`, default true when unbound.
pub fn stub_c8a5c8(shadow: &EntityShadowRenderable) -> bool {
    shadow.is_visible()
}

// 0xc8a5e0 — __ZN4Ogre6Entity22EntityShadowRenderable17rebindIndexBufferERKNS_28HardwareIndexBufferSharedPtrE
#[doc(
    alias = "Ogre::Entity::EntityShadowRenderable::rebindIndexBuffer(Ogre::HardwareIndexBufferSharedPtr const&)"
)]
// was: Ogre::Entity::EntityShadowRenderable::rebindIndexBuffer(Ogre::HardwareIndexBufferSharedPtr const&)
// IDA 0xc8a5e0: rebinds the `HardwareIndexBufferSharedPtr`.
pub fn stub_c8a5e0(
    shadow: &mut EntityShadowRenderable,
    buffer: Option<SharedPtr<HardwareIndexBuffer>>,
) {
    shadow.rebind_index_buffer(buffer);
}

// 0xc8a600 — __ZN4Ogre6Entity19setRenderQueueGroupEh
#[doc(alias = "Ogre::Entity::setRenderQueueGroup(unsigned char)")]
// was: Ogre::Entity::setRenderQueueGroup(unsigned char)
// IDA 0xc8a600: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8a600() {
}

// 0xc8a638 — __ZN4Ogre6Entity30setRenderQueueGroupAndPriorityEht
#[doc(alias = "Ogre::Entity::setRenderQueueGroupAndPriority(unsigned char,unsigned short)")]
// was: Ogre::Entity::setRenderQueueGroupAndPriority(unsigned char,unsigned short)
// IDA 0xc8a638: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8a638() {
}

// 0xc8a67c — __ZNK4Ogre6Entity12getTypeFlagsEv
#[doc(alias = "Ogre::Entity::getTypeFlags(void)const")]
// was: Ogre::Entity::getTypeFlags(void)const
// IDA 0xc8a67c: double-LDR of `Ogre::SceneManager::ENTITY_TYPE_MASK` (disasm confirms the static load).
pub fn stub_c8a67c(_entity: &OgreEntity) -> u32 {
    ENTITY_TYPE_MASK
}

// 0xc8a68c — __ZN4Ogre6Entity23getVertexDataForBindingEv
#[doc(alias = "Ogre::Entity::getVertexDataForBinding(void)")]
// was: Ogre::Entity::getVertexDataForBinding(void)
// IDA 0xc8a68c: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8a68c() {
}

// 0xc8a6f8 — __ZN4Ogre6Entity26chooseVertexDataForBindingEb
#[doc(alias = "Ogre::Entity::chooseVertexDataForBinding(bool)")]
// was: Ogre::Entity::chooseVertexDataForBinding(bool)
// IDA 0xc8a6f8: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8a6f8() {
}

// 0xc8a738 — __ZN4Ogre6Entity16visitRenderablesEPNS_10Renderable7VisitorEb
#[doc(alias = "Ogre::Entity::visitRenderables(Ogre::Renderable::Visitor *,bool)")]
// was: Ogre::Entity::visitRenderables(Ogre::Renderable::Visitor *,bool)
// IDA 0xc8a738: 61 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8a738() {
}

// 0xc8a7d4 — __ZNK4Ogre13EntityFactory7getTypeEv
#[doc(alias = "Ogre::EntityFactory::getType(void)const")]
// was: Ogre::EntityFactory::getType(void)const
// IDA 0xc8a7d4: PC-relative lea of `EntityFactory::FACTORY_TYPE_NAME` (disasm confirms; same shape as 0xc87914).
pub fn stub_c8a7d4() -> &'static str {
    OgreEntity::movable_type()
}

// 0xc8a7e0 — __ZN4Ogre13EntityFactory18createInstanceImplERKSsPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(
    alias = "Ogre::EntityFactory::createInstanceImpl(std::string const&,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)"
)]
// was: Ogre::EntityFactory::createInstanceImpl(std::string const&,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)
// IDA 0xc8a7e0: 533 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8a7e0() {
}

// 0xc8ad84 — __ZN4Ogre13EntityFactory15destroyInstanceEPNS_13MovableObjectE
#[doc(alias = "Ogre::EntityFactory::destroyInstance(Ogre::MovableObject *)")]
// was: Ogre::EntityFactory::destroyInstance(Ogre::MovableObject *)
// IDA 0xc8ad84: null-checked scalar-deleting destructor; maps to `Arc` drop.
pub fn stub_c8ad84(object: Option<SharedPtr<MovableObject>>) {
    destroy_instance(object);
}

// 0xc8ad98 — __ZN4Ogre14AxisAlignedBox15transformAffineERKNS_7Matrix4E
#[doc(alias = "Ogre::AxisAlignedBox::transformAffine(Ogre::Matrix4 const&)")]
// was: Ogre::AxisAlignedBox::transformAffine(Ogre::Matrix4 const&)
// IDA 0xc8ad98: 104 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8ad98() {
}

// 0xc8af24 — __ZN4Ogre25RuntimeAssertionExceptionD1Ev
#[doc(alias = "Ogre::RuntimeAssertionException::~RuntimeAssertionException()")]
// was: Ogre::RuntimeAssertionException::~RuntimeAssertionException()
// IDA 0xc8af24: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c8af24() {
}

// 0xc8af30 — __ZN4Ogre8Resource8Listener27backgroundPreparingCompleteEPS0_
#[doc(alias = "Ogre::Resource::Listener::backgroundPreparingComplete(Ogre::Resource*)")]
// was: Ogre::Resource::Listener::backgroundPreparingComplete(Ogre::Resource*)
// IDA 0xc8af30: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c8af30() {
}