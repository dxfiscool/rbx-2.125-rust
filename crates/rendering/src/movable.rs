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
            ..Default::default()
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
    /// User-object bindings at `+36` (`UserObjectBindings::getUserAny`,
    /// IDA `0xc6eb08`..`0xc6eb12`).
    pub user_any: UserAny,
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

    /// `Renderable::getUserAny` (IDA `0xc6eb08`): bindings at `this + 36`.
    #[inline]
    pub fn user_any(&self) -> &UserAny {
        &self.user_any
    }
}

/// Ogre::Exception — source string at `+0x14`, line at `+0x04`.
/// Full construction is `(number, description, source, type, file, line)`
/// (IDA `0xc6ea4a`).
#[derive(Clone, Debug, Default)]
pub struct OgreException {
    pub line: i32,
    pub source: String,
    pub message: String,
    /// Numeric code (`5` for `ItemIdentityException`, IDA `0xc6ea4a`).
    pub number: i32,
    /// Long description (also mirrored in `message`).
    pub description: String,
    /// Throwing translation unit (`OgreAnimable.h`, IDA `0xc6ea4a`).
    pub file: String,
    /// Concrete C++ type (`ItemIdentityException`, IDA `0xc6ea4a`).
    pub type_name: String,
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
impl OgreException {
    /// `Ogre::Exception::Exception(number, description, source, type,
    /// file, line)` (IDA `0xc6ea4a`).
    pub fn new(
        number: i32,
        description: &str,
        source: &str,
        type_name: &str,
        file: &str,
        line: i32,
    ) -> Self {
        Self {
            line,
            source: source.to_string(),
            message: description.to_string(),
            number,
            description: description.to_string(),
            file: file.to_string(),
            type_name: type_name.to_string(),
        }
    }
}

/// `Ogre::ColourValue` — four float channels `r, g, b, a` at `+0..+12`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ColourValue {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl ColourValue {
    /// `ColourValue::getAsRGBA` (IDA `0xc6fee4`): each channel scaled by
    /// `255.0` (`0x437F0000`, IDA `0xc6fee8`), truncated (`vcvt_s32_f32`),
    /// packed `(r << 24) | (g << 16) | (b << 8) | a`.
    pub fn get_as_rgba(&self) -> u32 {
        pack_channel(self.r) << 24
            | pack_channel(self.g) << 16
            | pack_channel(self.b) << 8
            | pack_channel(self.a)
    }

    /// `ColourValue::getAsARGB` (IDA `0xc6ff3c`): `(a << 24) | (r << 16) |
    /// (g << 8) | b`.
    pub fn get_as_argb(&self) -> u32 {
        pack_channel(self.a) << 24
            | pack_channel(self.r) << 16
            | pack_channel(self.g) << 8
            | pack_channel(self.b)
    }

    /// `ColourValue::getAsABGR` (IDA `0xc6ff94`): `(a << 24) | (b << 16) |
    /// (g << 8) | r`.
    pub fn get_as_abgr(&self) -> u32 {
        pack_channel(self.a) << 24
            | pack_channel(self.b) << 16
            | pack_channel(self.g) << 8
            | pack_channel(self.r)
    }
}

/// `vcvt_s32_f32(c * 255.0)`: truncation toward zero. Rust `as u32`
/// saturates on overflow, so truncate via `trunc()` first; identical for
/// in-range `[0, 1]` channels.
#[inline]
fn pack_channel(c: f32) -> u32 {
    (c * 255.0).trunc() as u32
}

/// `Ogre::FastHash(data, length, seed, _)` (IDA `0xc70150`): Paul-Hsieh-style
/// hash over 4-byte chunks, then 3/2/1-byte tail cases, then avalanche.
/// `seed == 0` selects `length` (IDA `0xc70164`); null/empty input with no
/// chunks yields `0` (IDA `0xc7015c`, `0xc701f2`). All arithmetic wraps.
pub fn fast_hash(data: &[u8], seed: u32) -> u32 {
    if data.is_empty() {
        return 0;
    }
    let mut hash = if seed == 0 { data.len() as u32 } else { seed };
    let mut chunks = data.chunks_exact(4);
    for word in &mut chunks {
        let lo = u16::from_le_bytes([word[0], word[1]]) as u32;
        let hi = u16::from_le_bytes([word[2], word[3]]) as u32;
        // IDA `0xc7018e`: t = (hash + lo) ^ (hi << 11) ^ ((hash + lo) << 16);
        // hash = t + (t >> 11).
        let t = (hash.wrapping_add(lo) ^ hi.wrapping_shl(11))
            ^ (hash.wrapping_add(lo).wrapping_shl(16));
        hash = t.wrapping_add(t >> 11);
    }
    let tail = chunks.remainder();
    match tail.len() {
        // IDA `0xc701b6`..`0xc701ba`.
        1 => {
            let t = (tail[0] as u32).wrapping_add(hash);
            let t = t ^ t.wrapping_shl(10);
            hash = t.wrapping_add(t >> 1);
        }
        // IDA `0xc701a6`..`0xc701aa`.
        2 => {
            let base = u16::from_le_bytes([tail[0], tail[1]]) as u32;
            let t = base.wrapping_add(hash);
            let t = t ^ t.wrapping_shl(11);
            hash = t.wrapping_add(t >> 17);
        }
        // IDA `0xc701d0`..`0xc701d4`.
        3 => {
            let base = u16::from_le_bytes([tail[0], tail[1]]) as u32;
            let t = base.wrapping_add(hash)
                ^ base.wrapping_add(hash).wrapping_shl(16)
                ^ (tail[2] as u32).wrapping_shl(18);
            hash = t.wrapping_add(t >> 11);
        }
        _ => {}
    }
    // IDA `0xc701dc`..`0xc701ec` avalanche.
    let t = hash ^ hash.wrapping_mul(8);
    let t = t.wrapping_add(t >> 5);
    let t = t ^ t.wrapping_mul(16);
    let t = t.wrapping_add(t >> 17);
    let t = t ^ t.wrapping_shl(25);
    t.wrapping_add(t >> 6)
}

/// `Ogre::AnimableObject` base: the dictionary name is always blank and
/// value creation always throws (IDA `0xc6e7d0`, `0xc6e7e4`).
#[derive(Clone, Debug, Default)]
pub struct AnimableObject;

/// Placeholder for the `Ok` side of `create_animable_value`; the base
/// implementation never produces one.
#[derive(Clone, Debug)]
pub struct AnimableValue {
    pub name: String,
}

impl AnimableObject {
    /// `AnimableObject::getAnimableDictionaryName` (IDA `0xc6e7d0`):
    /// returns `&StringUtil::BLANK`.
    pub fn animable_dictionary_name(&self) -> &'static str {
        ""
    }

    /// `AnimableObject::createAnimableValue(name)` (IDA `0xc6e7e4`):
    /// always throws `ItemIdentityException` (`5`, `"No animable value
    /// named '<name>' present."`, `AnimableObject::createAnimableValue`,
    /// `OgreAnimable.h:323`); Rust reports it as `Err`.
    pub fn create_animable_value(&self, name: &str) -> Result<AnimableValue, OgreException> {
        Err(OgreException::new(
            5,
            &format!("No animable value named '{name}' present."),
            "AnimableObject::createAnimableValue",
            "ItemIdentityException",
            "../../OgreMain/include/OgreAnimable.h",
            323,
        ))
    }
}

/// One entry of `Ogre::Codec::msMapCodecs`: extensions plus the magic-byte
/// prefix probed by `magicNumberToFileExt`.
#[derive(Clone, Debug, Default)]
pub struct CodecEntry {
    pub extensions: Vec<String>,
    pub magic_prefix: Vec<u8>,
}

impl CodecEntry {
    /// `magicNumberToFileExt` probe: true when `magic` starts with the
    /// registered prefix (empty prefix never matches).
    pub fn matches_magic(&self, magic: &[u8]) -> bool {
        !self.magic_prefix.is_empty() && magic.starts_with(&self.magic_prefix)
    }
}

/// `Ogre::Codec` static registry (`msMapCodecs`, IDA `0xc6f40e`).
#[derive(Clone, Debug, Default)]
pub struct CodecRegistry {
    entries: Vec<(String, CodecEntry)>,
}

impl CodecRegistry {
    /// Register a codec under its lower-cased type name.
    pub fn register(&mut self, type_name: &str, entry: CodecEntry) {
        let key = type_name.to_lowercase();
        if let Some(slot) = self.entries.iter_mut().find(|(name, _)| *name == key) {
            slot.1 = entry;
        } else {
            self.entries.push((key, entry));
        }
    }

    /// `Codec::getCodec(extension)` (IDA `0xc6f3a0`): lower-case lookup in
    /// `msMapCodecs`; miss throws with `"Supported formats are: <exts>."`.
    pub fn get_codec(&self, type_name: &str) -> Result<usize, OgreException> {
        let key = type_name.to_lowercase();
        if let Some(index) = self.entries.iter().position(|(name, _)| *name == key) {
            return Ok(index);
        }
        Err(OgreException::new(
            0,
            &format!("Supported formats are: {}.", self.extensions().join(" ")),
            "Codec::getCodec",
            "Exception",
            "../../OgreMain/src/OgreCodec.cpp",
            0,
        ))
    }

    /// `Codec::getCodec(magicNumber, maxbytes)` (IDA `0xc6fbcc`): first
    /// codec whose magic probe matches; miss throws like the string form.
    pub fn get_codec_for_magic(&self, magic: &[u8]) -> Result<usize, OgreException> {
        if let Some(index) = self.entries.iter().position(|(_, e)| e.matches_magic(magic)) {
            return Ok(index);
        }
        self.get_codec("")
    }

    /// `Codec::getExtensions` (IDA `0xc6f1f4`): every registered extension.
    pub fn extensions(&self) -> Vec<String> {
        self.entries
            .iter()
            .flat_map(|(_, e)| e.extensions.iter().cloned())
            .collect()
    }

    /// Codec name behind a registry index returned by `get_codec`.
    pub fn codec_name(&self, index: usize) -> Option<&str> {
        self.entries.get(index).map(|(name, _)| name.as_str())
    }
}

/// Material lookup behind `CompositionPass::setMaterialName` (IDA
/// `0xc70704`..`0xc7071e`): `MaterialManager::getSingleton` + load in
/// `AUTODETECT_RESOURCE_GROUP_NAME`.
pub trait MaterialResolver {
    /// Load `name`, returning the material handle (`None` = load failed,
    /// stored as null at `+16`, IDA `0xc70772`..`0xc707e2`).
    fn load_material(&self, name: &str) -> Option<usize>;
}

/// Material support probe behind `CompositionPass::_isSupported` (IDA
/// `0xc70a8a`..`0xc70a9c`): compile the pass material, then count its
/// supported techniques.
pub trait PassMaterialSupport {
    /// `Material::compile(handle, true)` (IDA `0xc70a8a`).
    fn compile_material(&self, handle: usize);
    /// `Material::getNumSupportedTechniques(handle)` (IDA `0xc70a9c`).
    fn supported_techniques(&self, handle: usize) -> usize;
}
