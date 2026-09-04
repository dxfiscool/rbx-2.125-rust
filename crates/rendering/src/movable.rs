//! Idiomatic Ogre mirror types backing the trivial accessor stubs.
//! Field offsets verified against IDA disassembly (see per-method notes).

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
#[derive(Clone, Debug, Default)]
#[doc(alias = "Ogre::Entity")]
pub struct Entity {
    pub movable: MovableObject,
    pub mesh_name: String,
}

impl Entity {
    pub fn new(mesh_name: impl Into<String>) -> Self {
        Self {
            movable: MovableObject::default(),
            mesh_name: mesh_name.into(),
        }
    }

    pub fn shared(mesh_name: impl Into<String>) -> SharedPtr<parking_lot::Mutex<Self>> {
        Arc::new(parking_lot::Mutex::new(Self::new(mesh_name)))
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
