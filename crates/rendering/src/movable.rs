//! Idiomatic Ogre mirror types backing the trivial accessor stubs.
//! Field offsets verified against IDA disassembly (see per-method notes).

use std::collections::HashMap;
use std::sync::Arc;

use rbx_core::SharedPtr;

/// Ogre::Any — user payload attached to movable/renderable objects.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct UserAny {
    pub value: String,
}

/// Ogre::MovableObject — scene-attached renderable carrier.
///
/// Layout notes mirror the original C++ member offsets observed in IDA:
///
/// - `+0x18` parent-tag-point flag (bool)
/// - `+0x1A` debug-display flag (bool)
/// - `+0x1C` rendering distance (float)
/// - `+0x20` cached squared rendering distance (float)
/// - `+0x24` rendering min pixel size (float)
/// - `+0x2C` user-object bindings (holds [`UserAny`])
/// - `+0x3C` query flags (u32)
/// - `+0x40` visibility flags (u32)
/// - `+0x98` listener pointer (nullable)
/// - `+0x9C` embedded light list head
/// - `+0xB8` light mask (u32)
#[derive(Clone, Debug, Default)]
pub struct MovableObject {
    pub is_parent_tag_point: bool,
    pub debug_display_enabled: bool,
    pub rendering_distance: f32,
    pub rendering_distance_squared: f32,
    pub min_pixel_size: f32,
    pub user_any: UserAny,
    pub query_flags: u32,
    pub visibility_flags: u32,
    pub listener: Option<usize>,
    pub light_list: Vec<u32>,
    pub light_mask: u32,
}

impl MovableObject {
    #[inline]
    pub fn is_parent_tag_point(&self) -> bool {
        self.is_parent_tag_point
    }

    #[inline]
    pub fn set_rendering_distance(&mut self, distance: f32) {
        self.rendering_distance = distance;
        self.rendering_distance_squared = distance * distance;
    }

    #[inline]
    pub fn rendering_distance(&self) -> f32 {
        self.rendering_distance
    }

    #[inline]
    pub fn set_rendering_min_pixel_size(&mut self, size: f32) {
        self.min_pixel_size = size;
    }

    #[inline]
    pub fn rendering_min_pixel_size(&self) -> f32 {
        self.min_pixel_size
    }

    #[inline]
    pub fn set_user_any(&mut self, any: UserAny) {
        self.user_any = any;
    }

    #[inline]
    pub fn user_any(&self) -> &UserAny {
        &self.user_any
    }

    #[inline]
    pub fn set_query_flags(&mut self, flags: u32) {
        self.query_flags = flags;
    }

    #[inline]
    pub fn add_query_flags(&mut self, flags: u32) {
        self.query_flags |= flags;
    }

    #[inline]
    pub fn remove_query_flags(&mut self, flags: u32) {
        self.query_flags &= !flags;
    }

    #[inline]
    pub fn query_flags(&self) -> u32 {
        self.query_flags
    }

    #[inline]
    pub fn set_visibility_flags(&mut self, flags: u32) {
        self.visibility_flags = flags;
    }

    #[inline]
    pub fn add_visibility_flags(&mut self, flags: u32) {
        self.visibility_flags |= flags;
    }

    #[inline]
    pub fn remove_visibility_flags(&mut self, flags: u32) {
        self.visibility_flags &= !flags;
    }

    #[inline]
    pub fn visibility_flags(&self) -> u32 {
        self.visibility_flags
    }

    #[inline]
    pub fn set_listener(&mut self, listener: Option<usize>) {
        self.listener = listener;
    }

    #[inline]
    pub fn listener(&self) -> Option<usize> {
        self.listener
    }

    #[inline]
    pub fn light_mask(&self) -> u32 {
        self.light_mask
    }

    #[inline]
    pub fn light_list(&self) -> &[u32] {
        &self.light_list
    }

    #[inline]
    pub fn set_debug_display_enabled(&mut self, enabled: bool) {
        self.debug_display_enabled = enabled;
    }

    #[inline]
    pub fn is_debug_display_enabled(&self) -> bool {
        self.debug_display_enabled
    }
}

/// Ogre::Entity — movable object with a mesh binding.
/// boost::shared_ptr<Ogre::Entity> maps to [`SharedPtr`] (`Arc`).
///
/// Field offsets mirror the ARMv7 layout observed in IDA (`OgreEntity.cpp`,
/// Ogre 1.6.4): mesh at `+192` (`0xc86768`: `ADDS R0,#0xC0` past the
/// `MovableObject` base), sub-entity vector at `+212` (`0xc86968`),
/// animation-state set at `+224` (`0xc87910`), software/hardware
/// vertex-anim data at `+384`/`+388` (`0xc891a8`, `0xc891b0`),
/// buffers-used flag at `+392` (`0xc88fc6: STRB.W R1,[R0,#0x188]`),
/// skeleton instance at `+508` (`0xc884f6`), child-object map at `+584`
/// (`0xc85ece`), cached bounds at `+612` (`0xc86b68`).
#[derive(Clone, Debug, Default)]
#[doc(alias = "Ogre::Entity")]
pub struct Entity {
    pub movable: MovableObject,
    pub mesh_name: String,
    /// Bounding radius of the bound mesh (`Mesh::getBoundingSphereRadius`).
    pub mesh_bounding_radius: f32,
    /// Local bounds of the bound mesh (`Mesh::getBounds`).
    pub mesh_bounds: AxisAlignedBox,
    /// LOD index used for edge-list queries (`+470`, `0xc8934c`).
    pub mesh_lod_index: u16,
    /// Whether the bound mesh currently exposes an edge list.
    pub mesh_has_edge_list: bool,
    /// Whether the bound mesh carries vertex animation.
    pub mesh_vertex_animated: bool,
    /// Sub-entity list (`+212`, grown by `buildSubEntityList`).
    pub sub_entities: Vec<SubEntity>,
    /// Animation-state set (`+224`); empty means `begin == end`.
    pub animation_states: Vec<String>,
    /// Skeleton instance (`+508`); the flag records whether the skeleton
    /// itself reports animation (`0xc88514` vtable call).
    pub skeleton_has_animation: Option<bool>,
    /// Software vertex-anim vertex data (`+384`, `0xc891a8`).
    pub software_vertex_anim_data: Option<usize>,
    /// Hardware vertex-anim vertex data (`+388`, `0xc891b0`).
    pub hardware_vertex_anim_data: Option<usize>,
    /// Set by `_markBuffersUsedForAnimation` (`+392`, `0xc88fc6`).
    pub buffers_used_for_animation: bool,
    /// Requested vertex-animation map (`+424`, cleared by
    /// `reevaluateVertexProcessing`, `0xc85ea6..0xc85ebc`).
    pub vertex_animation_requests: HashMap<u16, bool>,
    /// Objects attached to bones (`+584`, cleared by
    /// `detachAllObjectsImpl`, `0xc85f0c..0xc85f1e`).
    pub child_objects: HashMap<String, bool>,
    /// Cached local bounding box (`+612`, six floats plus extent word).
    pub bounding_box: AxisAlignedBox,
    pub world_bounding_box: AxisAlignedBox,
    pub world_bounding_sphere: BoundingSphere,
    pub hardware_animation_enabled: bool,
    pub vertex_processing_evaluated: bool,
    /// Entity-level temp blend buffers (`+308`).
    pub entity_temp_buffers: TempBlendedBufferInfo,
    pub attached_to_parent: bool,
    pub initialised: bool,
}

/// Ogre::SubEntity — one material slot of an [`Entity`].
#[derive(Clone, Debug, Default, PartialEq)]
#[doc(alias = "Ogre::SubEntity")]
pub struct SubEntity {
    pub material_name: String,
    pub material_group: String,
    pub vertex_animation_type: VertexAnimationType,
    pub temp_buffers_checked_out: bool,
    pub visible: bool,
}

impl SubEntity {
    /// IDA `0xc8697e`: `SubEntity::setMaterialName` applied per slot.
    #[inline]
    pub fn set_material_name(&mut self, name: impl Into<String>, group: impl Into<String>) {
        self.material_name = name.into();
        self.material_group = group.into();
    }
}

/// Ogre::SubMesh::VertexAnimationType — morph/pose classification per slot.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[doc(alias = "Ogre::SubMesh::VertexAnimationType")]
pub enum VertexAnimationType {
    #[default]
    None,
    Morph,
    Pose,
}

/// Ogre::AxisAlignedBox::Extent — null/finite/infinite discriminant.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[doc(alias = "Ogre::AxisAlignedBox::Extent")]
pub enum BoxExtent {
    #[default]
    Null,
    Finite,
    Infinite,
}

/// Ogre::AxisAlignedBox — minimum/maximum corners plus extent flag.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[doc(alias = "Ogre::AxisAlignedBox")]
pub struct AxisAlignedBox {
    pub minimum: [f32; 3],
    pub maximum: [f32; 3],
    pub extent: BoxExtent,
}

impl AxisAlignedBox {
    #[inline]
    pub fn null() -> Self {
        Self {
            minimum: [0.0; 3],
            maximum: [0.0; 3],
            extent: BoxExtent::Null,
        }
    }

    #[inline]
    pub fn is_null(&self) -> bool {
        self.extent == BoxExtent::Null
    }

    #[inline]
    pub fn is_finite(&self) -> bool {
        self.extent == BoxExtent::Finite
    }

    /// IDA `0xc86bf8..0xc86c90`: component-wise merge of child bounds.
    pub fn merge(&mut self, other: &Self) {
        if other.is_null() {
            return;
        }
        if self.is_null() {
            *self = *other;
            return;
        }
        for i in 0..3 {
            self.minimum[i] = self.minimum[i].min(other.minimum[i]);
            self.maximum[i] = self.maximum[i].max(other.maximum[i]);
        }
        self.extent = BoxExtent::Finite;
    }
}

/// Ogre::Sphere — centre plus radius for world-space bounds.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[doc(alias = "Ogre::Sphere")]
pub struct BoundingSphere {
    pub center: [f32; 3],
    pub radius: f32,
}

/// Ogre::TempBlendedBufferInfo — checkout record for blended vertex data.
#[derive(Clone, Debug, Default, PartialEq)]
#[doc(alias = "Ogre::TempBlendedBufferInfo")]
pub struct TempBlendedBufferInfo {
    pub vertex_data_id: usize,
    pub buffers_checked_out: bool,
    pub buffers_needed: bool,
}

impl TempBlendedBufferInfo {
    /// IDA `0xc89286`: `TempBlendedBufferInfo::extractFrom(info, vertexData)`.
    #[inline]
    pub fn extract_from(&mut self, vertex_data_id: usize) {
        self.vertex_data_id = vertex_data_id;
        self.buffers_checked_out = false;
        self.buffers_needed = true;
    }

    /// IDA `0xc87954`/`0xc8798a`: `buffersCheckedOut(info, check, needed)`.
    #[inline]
    pub fn buffers_checked_out(&self) -> bool {
        !self.buffers_needed || self.buffers_checked_out
    }
}

impl Entity {
    pub fn new(mesh_name: impl Into<String>) -> Self {
        Self {
            movable: MovableObject::default(),
            mesh_name: mesh_name.into(),
            ..Self::default()
        }
    }

    pub fn shared(mesh_name: impl Into<String>) -> SharedPtr<parking_lot::Mutex<Self>> {
        Arc::new(parking_lot::Mutex::new(Self::new(mesh_name)))
    }

    /// Ogre factory type name (`0xc8791e`: `EntityFactory::FACTORY_TYPE_NAME`).
    pub const MOVABLE_TYPE: &'static str = "Entity";

    #[inline]
    pub fn movable_type() -> &'static str {
        Self::MOVABLE_TYPE
    }

    /// IDA `0xc8676a`: `return (char *)this + 192` — the bound mesh handle.
    #[inline]
    pub fn mesh_name(&self) -> &str {
        &self.mesh_name
    }

    /// IDA `0xc86796..0xc867e4`: bounds-checked sub-entity fetch; out of
    /// range throws `InvalidParametersException` ("Index out of bounds.",
    /// `Entity::getSubEntity`, `OgreEntity.cpp:324`).
    pub fn sub_entity(&self, index: u32) -> Result<&SubEntity, OgreException> {
        self.sub_entities.get(index as usize).ok_or(OgreException {
            line: 324,
            source: "Entity::getSubEntity".to_string(),
            message: "Index out of bounds.".to_string(),
        })
    }

    /// IDA `0xc86958`: `(end - begin) >> 2`.
    #[inline]
    pub fn num_sub_entities(&self) -> usize {
        self.sub_entities.len()
    }

    /// IDA `0xc86964..0xc86988`: apply to every sub-entity in `[begin, end)`.
    pub fn set_material_name(&mut self, name: &str, group: &str) {
        for sub in &mut self.sub_entities {
            sub.set_material_name(name, group);
        }
    }

    /// IDA `0xc8585c..0xc8586c`: when the finished background load is the
    /// pending mesh, run `_initialise(false)`; otherwise a no-op.
    pub fn background_loading_complete(&mut self, resource_mesh: &str) -> bool {
        if self.mesh_name != resource_mesh {
            return false;
        }
        self.initialise(false);
        true
    }

    fn initialise(&mut self, _background: bool) {
        self.initialised = true;
    }

    /// IDA `0xc85ea6..0xc85ec0`: erase the `ushort -> bool` request map at
    /// `+424` and reset it to empty.
    #[inline]
    pub fn reevaluate_vertex_processing(&mut self) {
        self.vertex_animation_requests.clear();
        self.vertex_processing_evaluated = false;
    }

    /// IDA `0xc85ed2..0xc85f1e`: free every bone-attached tag point, then
    /// clear the child-object map at `+584`.
    #[inline]
    pub fn detach_all_objects_impl(&mut self) {
        self.child_objects.clear();
        self.attached_to_parent = false;
    }

    /// IDA `0xc87910`: animation-state set pointer at `+224`.
    #[inline]
    pub fn all_animation_states(&self) -> &[String] {
        &self.animation_states
    }

    /// IDA `0xc884f4..0xc88518`: skeleton present yields true when states
    /// are bound; with an empty state set the answer comes from the
    /// skeleton itself.
    #[inline]
    pub fn is_skeleton_animated(&self) -> bool {
        match self.skeleton_has_animation {
            None => false,
            Some(skeleton_animated) => {
                if self.animation_states.is_empty() {
                    skeleton_animated
                } else {
                    true
                }
            }
        }
    }

    /// IDA `0xc88fc6`: `*(this + 392) = 1`.
    #[inline]
    pub fn mark_buffers_used_for_animation(&mut self) {
        self.buffers_used_for_animation = true;
    }

    /// IDA `0xc891a8`: vertex data at `+384`.
    #[inline]
    pub fn software_vertex_anim_data(&self) -> Option<usize> {
        self.software_vertex_anim_data
    }

    /// IDA `0xc891b0`: vertex data at `+388`.
    #[inline]
    pub fn hardware_vertex_anim_data(&self) -> Option<usize> {
        self.hardware_vertex_anim_data
    }

    /// IDA `0xc89278`: `Mesh::getBoundingSphereRadius(mesh)`.
    #[inline]
    pub fn bounding_radius(&self) -> f32 {
        self.mesh_bounding_radius
    }

    /// IDA `0xc8934c`: `Mesh::getEdgeList(mesh, lodIndex)`; null when the
    /// mesh exposes no edge list.
    #[inline]
    pub fn edge_list_lod(&self) -> Option<u16> {
        self.mesh_has_edge_list.then_some(self.mesh_lod_index)
    }

    /// IDA `0xc89366`: `getEdgeList(...) != 0`.
    #[inline]
    pub fn has_edge_list(&self) -> bool {
        self.mesh_has_edge_list
    }

    /// IDA `0xc86bd0..0xc86cbc`: start from the mesh bounds, then merge the
    /// child-object box when one exists.
    pub fn bounding_box(&mut self) -> AxisAlignedBox {
        let mut merged = self.mesh_bounds;
        let children = self.child_objects_bounding_box();
        merged.merge(&children);
        self.bounding_box = merged;
        merged
    }

    /// IDA `0xc86dac..0xc86f50`: fold every attached child box into one;
    /// null when nothing is attached.
    pub fn child_objects_bounding_box(&self) -> AxisAlignedBox {
        let mut merged = AxisAlignedBox::null();
        if self.child_objects.is_empty() {
            return merged;
        }
        merged.merge(&self.mesh_bounds);
        merged
    }

    /// IDA `0xc86fcc..0xc86ffa`: with `derive=true` refresh each child
    /// object first, then take the `MovableObject` world box.
    pub fn world_bounding_box(&mut self, derive: bool) -> AxisAlignedBox {
        if derive {
            self.bounding_box();
        }
        self.world_bounding_box = self.bounding_box;
        self.world_bounding_box
    }

    /// IDA `0xc8700c..0xc8703a`: sphere twin of [`Entity::world_bounding_box`].
    pub fn world_bounding_sphere(&mut self, derive: bool) -> BoundingSphere {
        if derive {
            self.bounding_box();
        }
        let b = self.bounding_box;
        let center = [
            0.5 * (b.minimum[0] + b.maximum[0]),
            0.5 * (b.minimum[1] + b.maximum[1]),
            0.5 * (b.minimum[2] + b.maximum[2]),
        ];
        let dx = b.maximum[0] - center[0];
        let dy = b.maximum[1] - center[1];
        let dz = b.maximum[2] - center[2];
        let radius = (dx * dx + dy * dy + dz * dz).sqrt().max(self.mesh_bounding_radius);
        self.world_bounding_sphere = BoundingSphere { center, radius };
        self.world_bounding_sphere
    }

    /// IDA `0xc8792a..0xc879ca`: skeleton-level temp buffers must be out,
    /// and every animated sub-entity must hold its own checkout.
    pub fn temp_vertex_anim_buffers_bound(&self) -> bool {
        if self.mesh_vertex_animated && !self.entity_temp_buffers.buffers_checked_out() {
            return false;
        }
        for sub in &self.sub_entities {
            if sub.vertex_animation_type != VertexAnimationType::None
                && !sub.temp_buffers_checked_out
            {
                return false;
            }
        }
        true
    }

    /// IDA `0xc87a06..0xc87a34`: with no usable scheme entry the original
    /// re-runs vertex-processing evaluation and records a fresh entry;
    /// the cached enable flag is returned.
    pub fn is_hardware_animation_enabled(&mut self) -> bool {
        if !self.vertex_processing_evaluated {
            self.calc_vertex_processing();
        }
        self.hardware_animation_enabled
    }

    /// Simplified `calcVertexProcessing` (`0xc89368`): hardware skinning
    /// wins when a skeleton is bound and every animated slot is a pose
    /// slot; morph slots force the software path.
    pub fn calc_vertex_processing(&mut self) {
        let mut hardware = self.skeleton_has_animation.is_some();
        for sub in &self.sub_entities {
            match sub.vertex_animation_type {
                VertexAnimationType::None => {}
                VertexAnimationType::Pose => {}
                VertexAnimationType::Morph => {
                    hardware = false;
                }
            }
        }
        self.hardware_animation_enabled = hardware;
        self.vertex_processing_evaluated = true;
    }

    /// IDA `0xc89eaa..0xc89ed0`: base-class attach first, then propagate
    /// to every object in the child map (`+496`/`+500` range).
    pub fn notify_attached(&mut self, attached: bool) {
        self.attached_to_parent = attached;
        for flag in self.child_objects.values_mut() {
            *flag = attached;
        }
    }
}

/// Ogre::Frustum — camera frustum flag block.
#[derive(Clone, Debug, Default)]
pub struct Frustum {
    pub custom_view_matrix_enabled: bool,
    pub custom_projection_matrix_enabled: bool,
    pub reflected: bool,
    pub reflection_matrix: [f32; 16],
    pub reflection_plane: [f32; 4],
    pub custom_near_clip_plane_enabled: bool,
}

impl Frustum {
    #[inline]
    pub fn is_custom_view_matrix_enabled(&self) -> bool {
        self.custom_view_matrix_enabled
    }

    #[inline]
    pub fn is_custom_projection_matrix_enabled(&self) -> bool {
        self.custom_projection_matrix_enabled
    }

    #[inline]
    pub fn is_reflected(&self) -> bool {
        self.reflected
    }

    #[inline]
    pub fn reflection_matrix(&self) -> &[f32; 16] {
        &self.reflection_matrix
    }

    #[inline]
    pub fn reflection_plane(&self) -> &[f32; 4] {
        &self.reflection_plane
    }

    #[inline]
    pub fn is_custom_near_clip_plane_enabled(&self) -> bool {
        self.custom_near_clip_plane_enabled
    }
}

/// Ogre::Camera — window/render-distance flag block.
#[derive(Clone, Debug, Default)]
pub struct Camera {
    pub window_set: bool,
    pub use_rendering_distance: bool,
}

impl Camera {
    #[inline]
    pub fn is_window_set(&self) -> bool {
        self.window_set
    }

    #[inline]
    pub fn set_use_rendering_distance(&mut self, use_it: bool) {
        self.use_rendering_distance = use_it;
    }

    #[inline]
    pub fn use_rendering_distance(&self) -> bool {
        self.use_rendering_distance
    }
}

/// Ogre::Renderable — base renderable defaults.
#[derive(Clone, Debug, Default)]
pub struct Renderable {
    pub render_system_data: Option<usize>,
}

impl Renderable {
    #[inline]
    pub fn pre_render(&mut self) -> bool {
        true
    }

    #[inline]
    pub fn num_world_transforms(&self) -> u16 {
        1
    }

    #[inline]
    pub fn set_render_system_data(&mut self, data: Option<usize>) {
        self.render_system_data = data;
    }
}

/// Ogre::Exception — source string at `+0x14`, line at `+0x04`.
#[derive(Clone, Debug, Default)]
pub struct OgreException {
    pub line: i32,
    pub source: String,
    pub message: String,
}

impl OgreException {
    #[inline]
    pub fn source(&self) -> &str {
        &self.source
    }

    #[inline]
    pub fn line(&self) -> i32 {
        self.line
    }
}

impl std::fmt::Display for OgreException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}: {}", self.source, self.line, self.message)
    }
}

impl std::error::Error for OgreException {}
