//! rendering — generated_502 — 100 stubs global dedup (rendering filtered Ogre/G3D/Render/Adorn/Gfx, EA-sorted asc, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo)
//! Source: ida/export.json (85545 funcs) NOT in /tmp/global_eas.txt — next 100 uncovered EA-sorted asc 0xe49a3c..0xe4f6e4 (1636 candidates fresh, 92643 global EAs)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr). Sanitized: single quotes removed, boost::shared_ptr -> rbx_core::SharedPtr.

use rbx_core::SharedPtr;
use crate::ogre::{ColourValue, SceneBlendFactor};

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

/// was: `Ogre::TextureUnitState::ContentType` — named texture vs shadow content.
#[doc(alias = "Ogre::TextureUnitState::ContentType")]
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ContentType {
    #[default]
    Named = 0,
    Shadow = 1,
    Unknown = 0xffff_ffff,
}

impl ContentType {
    pub fn from_raw(v: u32) -> Self {
        match v {
            0 => ContentType::Named,
            1 => ContentType::Shadow,
            _ => ContentType::Unknown,
        }
    }
}

/// was: `Ogre::TextureUnitState::BindingType` — fragment vs vertex texture unit.
#[doc(alias = "Ogre::TextureUnitState::BindingType")]
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BindingType {
    #[default]
    Fragment = 0,
    Vertex = 1,
}

/// was: `Ogre::TextureUnitState::mTextureType` dimensionality.
/// Values observed in IDA: 2 = 2D (`0xe4ab3e`), 4 = 3D (`0xe4ab3c`, `0xe4aca8`).
pub const TEXTURE_TYPE_2D: u32 = 2;
/// was: `Ogre::TextureUnitState::mTextureType` 3D value (`0xe4aca8`: `is3D = (type == 4)`).
pub const TEXTURE_TYPE_3D: u32 = 4;

/// was: `Ogre::TextureUnitState::TextureEffect` (value of the `mEffects` map).
/// Flat layout from IDA `0xe4bd68`/`0xe4c2d4`: type@0, subtype@4, arg1@8,
/// arg2@12, waveType@16, base@20, frequency@24, phase@28, amplitude@32,
/// controller@36 (`LDR R0,[R4,#0x24]` at `0xe4bd72`).
#[doc(alias = "Ogre::TextureUnitState::TextureEffect")]
#[derive(Clone, Debug, Default)]
pub struct TextureEffect {
    pub effect_type: u32,
    pub subtype: i32,
    pub arg1: f32,
    pub arg2: f32,
    pub wave_type: u32,
    pub base: f32,
    pub frequency: f32,
    pub phase: f32,
    pub amplitude: f32,
    /// ControllerManager handle from `createEffectController`; None = destroyed.
    pub controller: Option<u32>,
}

/// was: `Ogre::TextureUnitState::TextureEffectType` (`0xe4bc46`: `type <= 5`
/// replaces the existing entry; 6 = `ET_TRANSFORM` allows several subtypes).
/// Values from `setScrollAnimation` (`0xe4c20c`: 2/3/4), `setRotateAnimation`
/// (`0xe4c2a0`: 5), `setTransformAnimation` (`0xe4c2d4`: 6), `setEnvironmentMap`
/// (`0xe4be2c`: 0).
#[doc(alias = "Ogre::TextureUnitState::TextureEffectType")]
pub mod effect_type {
    /// `ET_ENVIRONMENT_MAP` (IDA `0xe4be2c`).
    pub const ENVIRONMENT_MAP: u32 = 0;
    /// `ET_PROJECTIVE_TEXTURE` (no controller, IDA `0xe4bd8e` default arm).
    pub const PROJECTIVE_TEXTURE: u32 = 1;
    /// `ET_UVSCROLL` (IDA `0xe4c25a`).
    pub const UV_SCROLL: u32 = 2;
    /// `ET_USCROLL` (IDA `0xe4c270`).
    pub const U_SCROLL: u32 = 3;
    /// `ET_VSCROLL` (IDA `0xe4c288`).
    pub const V_SCROLL: u32 = 4;
    /// `ET_ROTATE` (IDA `0xe4c2c4`).
    pub const ROTATE: u32 = 5;
    /// `ET_TRANSFORM` (IDA `0xe4c368`).
    pub const TRANSFORM: u32 = 6;
}

/// was: `Ogre::LayerBlendModeEx` — one blend-mode block (60 bytes).
/// Colour block at +56, alpha block at +124 (IDA `0xe4be02`/`0xe4be06`).
#[doc(alias = "Ogre::LayerBlendModeEx")]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LayerBlendModeEx {
    /// +0 blend type (`LBT_COLOUR`/`LBT_ALPHA`).
    pub blend_type: u32,
    /// +4 operation (`mAlphaBlendMode.operation`, IDA `0xe4bc14`).
    pub operation: u32,
    /// +8 first source (IDA `0xe4bc14`).
    pub source1: u32,
    /// +12 second source (IDA `0xe4bc14`).
    pub source2: u32,
    /// +16 colour arg 1 (`mColourBlendMode.colourArg1`, IDA `0xe4bbda`).
    pub colour_arg1: ColourValue,
    /// +32 colour arg 2 (IDA `0xe4bbea`).
    pub colour_arg2: ColourValue,
    /// +48 alpha arg 1 (`mAlphaBlendMode.alphaArg1`, IDA `0xe4bc18`).
    pub alpha_arg1: f32,
    /// +52 alpha arg 2 (IDA `0xe4bc1c`).
    pub alpha_arg2: f32,
    /// +56 manual factor (`mAlphaBlendMode.factor`, IDA `0xe4bc20`).
    pub factor: f32,
}

/// was: `Ogre::TextureUnitState::UVWAddressingMode` — per-axis wrap mode.
/// Returned by reference at +28 (IDA `0xe4be0a`).
#[doc(alias = "Ogre::TextureUnitState::UVWAddressingMode")]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UvwAddressingMode {
    pub u: u32,
    pub v: u32,
    pub w: u32,
}

/// was: `Ogre::Matrix4` — row-major 4x4 (`mTexModMatrix` at +208, IDA `0xe4bf58`).
#[doc(alias = "Ogre::Matrix4")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix4 {
    pub m: [[f32; 4]; 4],
}

impl Matrix4 {
    /// `Ogre::Matrix4::IDENTITY` (copied in at IDA `0xe4bfa8`..`0xe4bfb4`).
    pub const IDENTITY: Self = Self {
        m: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };

    /// `Ogre::Matrix4::concatenate` (`0xd14830`): `out = a * b`, row-major.
    /// IDA `0xd14888`: `out[0][j] = a[0][0]*b[0][j] + a[0][1]*b[1][j] + ...`.
    pub fn concatenate(a: &Self, b: &Self) -> Self {
        let mut m = [[0.0f32; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                m[i][j] = a.m[i][0] * b.m[0][j]
                    + a.m[i][1] * b.m[1][j]
                    + a.m[i][2] * b.m[2][j]
                    + a.m[i][3] * b.m[3][j];
            }
        }
        Self { m }
    }
}

impl Default for Matrix4 {
    fn default() -> Self {
        Self::IDENTITY
    }
}

/// was: `Ogre::TexturePtr` (`boost::shared_ptr<Ogre::Texture>`, 16 bytes each in `mTextures`).
/// Only the loaded/prepared latches are modelled; the GPU resource itself is opaque.
/// `boost::shared_ptr` maps to `rbx_core::SharedPtr` per AGENTS.md §4.
#[derive(Clone, Debug, Default)]
pub struct TextureSlot {
    pub loaded: bool,
    /// `Texture::prepare` latch for `ensurePrepared` (IDA `0xe4c3a6`).
    pub prepared: bool,
}

/// was: `Ogre::TextureUnitState` (OgreMain/src/OgreTextureUnitState.cpp, ogre-v1-6-4).
/// Byte offsets are the IDA `(this + N)` word offsets mapped to bytes.
#[doc(alias = "Ogre::TextureUnitState")]
#[derive(Clone, Debug)]
pub struct TextureUnitState {
    /// +0 current frame index (`mCurrentFrame`, IDA `0xe4bb9a`).
    pub current_frame: u32,
    /// +4 animation duration, 0 = not animated (`mAnimDuration`, IDA `0xe49b06`).
    pub anim_duration: f32,
    /// +8 set to 1 by `setCubicTextureName`, 0 by `setAnimatedTextureName` (IDA `0xe4ab34`/`0xe4af9e`).
    pub flag_08: u8,
    /// +12 texture dimensionality (`mTextureType`, IDA `0xe4acb6`).
    pub texture_type: u32,
    /// +16 desired pixel format (`mDesiredFormat`, raw `Ogre::PixelFormat`, IDA `0xe4bba8`).
    pub desired_format: u32,
    /// +20 mipmap count (`mNumMipmaps`, IDA `0xe4bbac`).
    pub num_mipmaps: i32,
    /// +24 texture coordinate set (`mTextureCoordSetIndex`, IDA `0xe4bbc2`).
    pub texture_coord_set: u32,
    /// +28 addressing mode (`mAddressMode`, `UVWAddressingMode`, IDA `0xe4be0a`).
    pub addressing: UvwAddressingMode,
    /// +40 border colour (`mBorderColour`, IDA `0xe4be22`).
    pub border_colour: ColourValue,
    /// +56 colour blend block (`mColourBlendMode`, IDA `0xe4be02`).
    pub colour_blend: LayerBlendModeEx,
    /// +116 multipass fallback src/dst (IDA `0xe4bbfc`).
    pub colour_fallback_src: SceneBlendFactor,
    pub colour_fallback_dst: SceneBlendFactor,
    /// +124 alpha blend block (`mAlphaBlendMode`, IDA `0xe4be06`).
    pub alpha_blend: LayerBlendModeEx,
    /// +184 load-failed latch, cleared by the name setters (IDA `0xe4a96c`).
    pub load_failed: bool,
    /// +185 texture has alpha (`mIsAlpha`, IDA `0xe4bbb0`).
    pub is_alpha: bool,
    /// +186 hardware gamma (`mHwGammaEnabled`, IDA `0xe4bbb8`).
    pub hw_gamma_enabled: bool,
    /// +187 matrix-recalc latch (`mRecalcTexMatrix`, IDA `0xe4bf04`).
    pub recalc_tex_matrix: bool,
    /// +188 scroll offsets (`mUMod`/`mVMod`, IDA `0xe4bf12`).
    pub scroll: [f32; 2],
    /// +196 scale (`mUScale`/`mVScale`, IDA `0xe4bf22`).
    pub scale: [f32; 2],
    /// +204 rotation (`mRotate`, `Radian`, IDA `0xe4bf30`).
    pub rotate: f32,
    /// +208 texture matrix (`mTexModMatrix`, IDA `0xe4bf54`).
    pub tex_mod_matrix: Matrix4,
    /// +296 binding type (`mBindingType`, IDA `0xe4a90c`/`0xe4a918`).
    pub binding_type: BindingType,
    /// +300 content type (`mContentType`, IDA `0xe49ba8`/`0xe4a920`).
    pub content_type: u32,
    /// +312 frame texture names (`mFrames`, `vector<string>`).
    pub frames: Vec<String>,
    /// +328 loaded textures (`mTextures`, `vector<TexturePtr>`, 16 bytes each).
    pub textures: Vec<TextureSlot>,
    /// +356 effect list (original is a keyed `std::map`; insertion order kept).
    pub effects: Vec<TextureEffect>,
    /// +384 parent pass loaded latch (models `Pass::isLoaded(mParent)`, IDA `0xe49ad0`).
    pub parent_loaded: bool,
    /// +388 animation controller handle (`mAnimController`, IDA `0xe49a4a`).
    pub anim_controller: Option<u32>,
    /// Parent needs recompile/hash-dirty (`Pass::_notifyNeedsRecompile`/`_dirtyHash`, IDA `0xe4abb0`/`0xe4b1cc`).
    pub parent_dirty: bool,
    /// Local allocator for controller handles (no original address; glue only).
    next_handle: u32,
}

impl Default for TextureUnitState {
    /// Matches the Ogre ctor defaults that IDA shows in use: unit scale,
    /// identity texture matrix, everything else zero/empty.
    fn default() -> Self {
        Self {
            current_frame: 0,
            anim_duration: 0.0,
            flag_08: 0,
            texture_type: 0,
            desired_format: 0,
            num_mipmaps: 0,
            texture_coord_set: 0,
            addressing: UvwAddressingMode::default(),
            border_colour: ColourValue::default(),
            colour_blend: LayerBlendModeEx::default(),
            colour_fallback_src: SceneBlendFactor::default(),
            colour_fallback_dst: SceneBlendFactor::default(),
            alpha_blend: LayerBlendModeEx::default(),
            load_failed: false,
            is_alpha: false,
            hw_gamma_enabled: false,
            recalc_tex_matrix: false,
            scroll: [0.0, 0.0],
            scale: [1.0, 1.0],
            rotate: 0.0,
            tex_mod_matrix: Matrix4::IDENTITY,
            binding_type: BindingType::default(),
            content_type: 0,
            frames: Vec::new(),
            textures: Vec::new(),
            effects: Vec::new(),
            parent_loaded: false,
            anim_controller: None,
            parent_dirty: false,
            next_handle: 0,
        }
    }
}

impl TextureUnitState {
    fn alloc_handle(&mut self) -> u32 {
        let h = self.next_handle.max(1);
        self.next_handle = h.wrapping_add(1).max(1);
        h
    }

    /// IDA `0xe49a3c`: destroy the anim controller, every effect controller,
    /// then release all texture references.
    pub fn unload(&mut self) {
        // IDA 0xe49a50..0xe49a5e: ControllerManager::destroyController(mAnimController); mAnimController = 0
        self.anim_controller = None;
        // IDA 0xe49a62..0xe49a8a: walk the mEffects rb-tree destroying each effect controller
        for effect in self.effects.iter_mut() {
            effect.controller = None;
        }
        // IDA 0xe49a92..0xe49ab0: reset every TexturePtr in mTextures (shared_ptr release)
        for slot in self.textures.iter_mut() {
            slot.loaded = false;
        }
    }

    /// IDA `0xe49ac4`: `return Ogre::Pass::isLoaded(mParent)`.
    pub fn is_loaded(&self) -> bool {
        self.parent_loaded
    }

    /// IDA `0xe49ad4`: ensure every frame loaded; rebuild the animator when
    /// animated; create one controller per effect.
    pub fn load(&mut self) {
        // IDA 0xe49ada..0xe49af8: for each frame index: ensureLoaded(i)
        for i in 0..self.frames.len() as u32 {
            self.ensure_loaded(i);
        }
        // IDA 0xe49b06..0xe49b2c: mAnimDuration != 0 → destroy old controller, createTextureAnimator
        if self.anim_duration != 0.0 {
            self.anim_controller = None;
            let h = self.alloc_handle();
            self.anim_controller = Some(h);
        }
        // IDA 0xe49b30..0xe49b46: for each effect: createEffectController
        for i in 0..self.effects.len() {
            self.create_effect_controller(i);
        }
    }

    /// Models `TextureUnitState::ensurePrepared` (called at IDA `0xe4c3a6`).
    /// Original is const and calls `Texture::prepare` after gamma/checks;
    /// modelled as a latch like `ensure_loaded`.
    pub fn ensure_prepared(&mut self, index: u32) {
        if self.load_failed {
            return;
        }
        if let Some(slot) = self.textures.get_mut(index as usize) {
            slot.prepared = true;
        }
    }

    /// Models `TextureUnitState::ensureLoaded` (called at IDA `0xe49ae8`, `0xe4b97c`).
    pub fn ensure_loaded(&mut self, index: u32) {
        if let Some(slot) = self.textures.get_mut(index as usize) {
            slot.loaded = true;
        }
    }

    /// IDA `0xe49b54`: an out-of-range current frame yields `StringUtil::BLANK`.
    pub fn texture_name(&self) -> &str {
        self.frames
            .get(self.current_frame as usize)
            .map(String::as_str)
            .unwrap_or("")
    }

    /// IDA `0xe49b7c`: store the content type; shadow-family values drop every frame name.
    pub fn set_content_type(&mut self, content: u32) {
        // IDA 0xe49ba8: mContentType = a2
        self.content_type = content;
        // IDA 0xe49bce: `(a2 - 1) <= 1` unsigned → shadow content destroys the mFrames strings
        if content.wrapping_sub(1) <= 1 {
            self.frames.clear();
            self.textures.clear();
        }
    }

    /// IDA `0xe49dec` prefix: split `name` at the last `.` and build the six cube
    /// faces in binary order `_fr _bk _lf _rt _up _dn` (rodata refs at IDA `0xe49e6e`..`0xe49f0c`,
    /// `.` literal at `0xe49f44`, substr throw sites at `0xe4a41a`).
    pub fn cubic_face_names(name: &str) -> [String; 6] {
        let (stem, ext) = match name.rfind('.') {
            Some(pos) => name.split_at(pos),
            None => (name, ""),
        };
        ["_fr", "_bk", "_lf", "_rt", "_up", "_dn"].map(|s| format!("{stem}{s}{ext}"))
    }

    /// IDA `0xe4a924`: install cube names; `for_uvw` selects the single 3D-volume slot.
    pub fn set_cubic_texture_name(&mut self, names: &[String], for_uvw: bool) {
        // IDA 0xe4a966: mContentType = CONTENT_NAMED
        self.content_type = ContentType::Named as u32;
        // IDA 0xe4a96c: clear the load-failed latch
        self.load_failed = false;
        // IDA 0xe4a99a..0xe4a9a0 + fill_insert: 6 face slots, or 1 volume slot for UVW
        let want = if for_uvw { 1 } else { 6 };
        self.frames.resize(want, String::new());
        self.textures.resize(want, TextureSlot::default());
        // IDA 0xe4ab2a..0xe4ab3e: mAnimDuration = 0; mCurrentFrame = 0; flag = 1; type = forUVW ? 3D : 2D
        self.anim_duration = 0.0;
        self.current_frame = 0;
        self.flag_08 = 1;
        self.texture_type = if for_uvw { TEXTURE_TYPE_3D } else { TEXTURE_TYPE_2D };
        // IDA 0xe4ab5e..0xe4aba4: assign names over mFrames, releasing replaced TexturePtrs
        for (i, name) in names.iter().enumerate().take(want) {
            self.frames[i] = name.clone();
            self.textures[i] = TextureSlot::default();
        }
        // IDA 0xe4abb0: Pass::_notifyNeedsRecompile(mParent)
        self.parent_dirty = true;
    }

    /// IDA `0xe4acb8` prefix: frame name is `stem + "_" + i + ext` with a 0-based
    /// counter (init 0 at `0xe4b004`, streamed at `0xe4b092`, `"_"` literal at `0xe4b07a`).
    pub fn animated_frame_names(name: &str, num_frames: u32) -> Vec<String> {
        // IDA 0xe4ad48: find_last_of("."); substr stem/ext
        let (stem, ext) = match name.rfind('.') {
            Some(pos) => name.split_at(pos),
            None => (name, ""),
        };
        (0..num_frames).map(|i| format!("{stem}_{i}{ext}")).collect()
    }

    /// Shared tail of `0xe4b538`/`0xe4acb8`: install animated names plus duration.
    pub fn set_animated_texture_names(&mut self, names: &[String], num_frames: u32, duration: f32) {
        // IDA 0xe4b57c/0xe4acf4: mContentType = CONTENT_NAMED
        self.content_type = ContentType::Named as u32;
        // IDA 0xe4b582/0xe4acfc: clear the load-failed latch
        self.load_failed = false;
        // IDA fill_insert at 0xe4ae2c/0xe4b69e: grow both vectors to numFrames (default tail)
        let n = num_frames as usize;
        self.frames.resize(n, String::new());
        self.textures.resize(n, TextureSlot::default());
        // IDA 0xe4b736..0xe4b73e / 0xe4af98..0xe4af9e: duration, current frame 0, flag 0
        self.anim_duration = duration;
        self.current_frame = 0;
        self.flag_08 = 0;
        for (i, name) in names.iter().enumerate().take(n) {
            self.frames[i] = name.clone();
            self.textures[i] = TextureSlot::default();
        }
        // IDA 0xe4b196..0xe4b1cc: reload now when the parent pass is already loaded, then dirty the pass hash
        if self.parent_loaded {
            self.load();
        }
        self.parent_dirty = true;
    }

    /// IDA `0xe4b8f0`: content textures index directly; named frames lazy-load;
    /// anything else yields the guard-initialised static blank TexturePtr (None here).
    pub fn texture_ptr(&mut self, index: u32) -> Option<&TextureSlot> {
        // IDA 0xe4b8f6..0xe4b8fe: non-named content → mTextures[index]
        if self.content_type != ContentType::Named as u32 {
            return self.textures.get(index as usize);
        }
        // IDA 0xe4b912..0xe4b988: in-range, never-failed frame → ensureLoaded(i), then mTextures[i]
        if (index as usize) < self.frames.len() && !self.load_failed {
            self.ensure_loaded(index);
            return self.textures.get(index as usize);
        }
        // IDA 0xe4b928..0xe4b976: static blank TexturePtr
        None
    }

    /// IDA `0xe4b98c`: out-of-range throws `InvalidParametersException`; Rust panics with its text.
    pub fn set_current_frame(&mut self, frame: u32) {
        // IDA 0xe4b9e6..0xe4ba92: frameNumber >= numFrames → Ogre::Exception(InvalidParametersException,
        // "frameNumber parameter value exceeds number of stored frames.", "TextureUnitState::setCurrentFrame")
        if (frame as usize) >= self.frames.len() {
            panic!(
                "Ogre::InvalidParametersException at 0xe4b98c: frameNumber parameter value exceeds number of stored frames."
            );
        }
        self.current_frame = frame;
    }

    /// IDA `0xe4bb98`: `return mCurrentFrame`.
    pub fn get_current_frame(&self) -> u32 {
        self.current_frame
    }

    /// IDA `0xe4bba4`: `(end - begin) >> 2` = frame count.
    pub fn num_frames(&self) -> u32 {
        self.frames.len() as u32
    }

    /// IDA `0xe4aca8`: `return mTextureType == 4`.
    pub fn is_3d(&self) -> bool {
        self.texture_type == TEXTURE_TYPE_3D
    }

    /// IDA `0xe4bbc4`: store colour blend op/sources/args/factor (+60..+112).
    pub fn set_colour_operation_ex(
        &mut self,
        op: u32,
        src1: u32,
        src2: u32,
        arg1: ColourValue,
        arg2: ColourValue,
        factor: f32,
    ) {
        self.colour_blend.operation = op;
        self.colour_blend.source1 = src1;
        self.colour_blend.source2 = src2;
        self.colour_blend.colour_arg1 = arg1;
        self.colour_blend.colour_arg2 = arg2;
        self.colour_blend.factor = factor;
    }

    /// IDA `0xe4bbfc`: store the multipass fallback pair (+116).
    pub fn set_colour_op_multipass_fallback(&mut self, src: SceneBlendFactor, dst: SceneBlendFactor) {
        self.colour_fallback_src = src;
        self.colour_fallback_dst = dst;
    }
    /// IDA `0xe4bc04`: store alpha blend op/sources at +128..+136 and the
    /// alpha args + manual factor at +172..+180 (`mAlphaBlendMode`).
    pub fn set_alpha_operation(
        &mut self,
        op: u32,
        src1: u32,
        src2: u32,
        arg1: f32,
        arg2: f32,
        factor: f32,
    ) {
        // IDA 0xe4bc14: STRD over +128/+132/+136 (operation, source1, source2)
        self.alpha_blend.operation = op;
        self.alpha_blend.source1 = src1;
        self.alpha_blend.source2 = src2;
        // IDA 0xe4bc18..0xe4bc20: VSTR over +172/+176/+180
        self.alpha_blend.alpha_arg1 = arg1;
        self.alpha_blend.alpha_arg2 = arg2;
        self.alpha_blend.factor = factor;
    }

    /// IDA `0xe4bc2c`: null the incoming controller, replace any same-type
    /// entry (non-transform only), create the controller when loaded, insert.
    pub fn add_effect(&mut self, mut effect: TextureEffect) {
        // IDA 0xe4bc3c: effect.controller = 0
        effect.controller = None;
        // IDA 0xe4bc40..0xe4bc9a: type <= 5 → find + destroy controller + erase
        // the existing node (upstream: `effect.type != ET_TRANSFORM`).
        if effect.effect_type <= effect_type::ROTATE {
            self.remove_effect(effect.effect_type);
        }
        // IDA 0xe4bca8..0xe4bcae: isLoaded(mParent) → createEffectController
        if self.parent_loaded {
            let index = self.effects.len();
            self.effects.push(effect);
            self.create_effect_controller(index);
        } else {
            // IDA 0xe4bcd6..0xe4bd5a: rb-tree insert without a controller
            self.effects.push(effect);
        }
    }

    /// IDA `0xe4bd68`: destroy the old controller, then allocate one per the
    /// effect type via the ControllerManager singleton (modelled as a handle).
    /// Types 0/1 (`ET_ENVIRONMENT_MAP`/`ET_PROJECTIVE_TEXTURE`) take the
    /// `TBB` default arm at `0xe4bd8e`/`0xe4bdf2`, which skips the store, so
    /// the controller stays null.
    pub fn create_effect_controller(&mut self, index: usize) {
        // IDA 0xe4bd72..0xe4bd82: controller → destroyController → 0
        if let Some(effect) = self.effects.get_mut(index) {
            effect.controller = None;
        }
        let kind = match self.effects.get(index) {
            Some(effect) => effect.effect_type,
            None => return,
        };
        // IDA 0xe4bd88..0xe4bdf0: switch (type - 2) over 2..6
        let handle = match kind {
            effect_type::UV_SCROLL
            | effect_type::U_SCROLL
            | effect_type::V_SCROLL
            | effect_type::ROTATE
            | effect_type::TRANSFORM => Some(self.alloc_handle()),
            _ => None,
        };
        if let Some(effect) = self.effects.get_mut(index) {
            effect.controller = handle;
        }
    }

    /// IDA `0xe4bdf8`: `return mColourBlendFallbackSrc` (+116).
    pub fn colour_blend_fallback_src(&self) -> SceneBlendFactor {
        self.colour_fallback_src
    }

    /// IDA `0xe4bdfc`: `return mColourBlendFallbackDest` (+120).
    pub fn colour_blend_fallback_dst(&self) -> SceneBlendFactor {
        self.colour_fallback_dst
    }

    /// IDA `0xe4be00`: `return &mColourBlendMode` (+56).
    pub fn colour_blend_mode(&self) -> &LayerBlendModeEx {
        &self.colour_blend
    }

    /// IDA `0xe4be04`: `return &mAlphaBlendMode` (+124).
    pub fn alpha_blend_mode(&self) -> &LayerBlendModeEx {
        &self.alpha_blend
    }

    /// IDA `0xe4be08`: `return &mAddressMode` (+28).
    pub fn texture_addressing_mode(&self) -> &UvwAddressingMode {
        &self.addressing
    }

    /// IDA `0xe4be0c`: 12-byte copy of the `UVWAddressingMode` over +28.
    pub fn set_texture_addressing_mode(&mut self, mode: UvwAddressingMode) {
        // IDA 0xe4be12..0xe4be14: STR u/v/w over +28/+32/+36
        self.addressing = mode;
    }

    /// IDA `0xe4be1c`: 16-byte copy of the `ColourValue` over +40.
    pub fn set_texture_border_colour(&mut self, colour: ColourValue) {
        // IDA 0xe4be22: VLD1.32/vst pair over +40..+56
        self.border_colour = colour;
    }

    /// IDA `0xe4be28`: `return &mBorderColour` (+40).
    pub fn texture_border_colour(&self) -> &ColourValue {
        &self.border_colour
    }

    /// IDA `0xe4be2c`: enabled stores subtype and adds `ET_ENVIRONMENT_MAP`,
    /// disabled removes it.
    pub fn set_environment_map(&mut self, enable: bool, env_map_type: i32) {
        if !enable {
            // IDA 0xe4be34..0xe4be48: enable != 1 → removeEffect(ET_ENVIRONMENT_MAP)
            self.remove_effect(effect_type::ENVIRONMENT_MAP);
            return;
        }
        // IDA 0xe4be38..0xe4be44: eff = {ET_ENVIRONMENT_MAP, subtype}; addEffect
        self.add_effect(TextureEffect {
            effect_type: effect_type::ENVIRONMENT_MAP,
            subtype: env_map_type,
            ..TextureEffect::default()
        });
    }

    /// IDA `0xe4be50`: destroy every controller in the `equal_range(type)`
    /// span, then erase the span.
    pub fn remove_effect(&mut self, effect_type_: u32) {
        // IDA 0xe4be9a..0xe4bea4: destroyController per effect in range
        for effect in self.effects.iter_mut() {
            if effect.effect_type == effect_type_ {
                effect.controller = None;
            }
        }
        // IDA 0xe4bec4: rb-tree erase(first, last)
        self.effects.retain(|e| e.effect_type != effect_type_);
    }

    /// IDA `0xe4bec8`: 64-byte copy of the `Matrix4` over +208, latch clear.
    pub fn set_texture_transform(&mut self, xform: &Matrix4) {
        // IDA 0xe4bee6..0xe4befe: four VLD/VST row pairs over +208..+272
        self.tex_mod_matrix = *xform;
        // IDA 0xe4bf04: mRecalcTexMatrix = false
        self.recalc_tex_matrix = false;
    }

    /// IDA `0xe4bf0c`: store the scroll pair over +188, set the latch.
    pub fn set_texture_scroll(&mut self, u: f32, v: f32) {
        // IDA 0xe4bf12: STRD u,v over +188
        self.scroll = [u, v];
        // IDA 0xe4bf16: mRecalcTexMatrix = true
        self.recalc_tex_matrix = true;
    }

    /// IDA `0xe4bf1c`: store the scale pair over +196, set the latch.
    pub fn set_texture_scale(&mut self, u_scale: f32, v_scale: f32) {
        // IDA 0xe4bf22: STRD over +196
        self.scale = [u_scale, v_scale];
        // IDA 0xe4bf26: mRecalcTexMatrix = true
        self.recalc_tex_matrix = true;
    }

    /// IDA `0xe4bf2c`: store the `Radian` over +204, set the latch.
    pub fn set_texture_rotate(&mut self, angle: f32) {
        // IDA 0xe4bf30: STR over +204
        self.rotate = angle;
        // IDA 0xe4bf34: mRecalcTexMatrix = true
        self.recalc_tex_matrix = true;
    }

    /// IDA `0xe4bf3c`: recalc when the latch is set, return `mTexModMatrix`.
    pub fn texture_transform(&mut self) -> &Matrix4 {
        // IDA 0xe4bf42..0xe4bf4c: mRecalcTexMatrix → recalcTextureMatrix
        if self.recalc_tex_matrix {
            self.recalc_texture_matrix();
        }
        // IDA 0xe4bf54: return this + 208
        &self.tex_mod_matrix
    }

    /// IDA `0xe4bf58`: rebuild `mTexModMatrix` from scale/scroll/rotate.
    /// Matches upstream `OgreTextureUnitState.cpp`: scale-about-centre by
    /// inverse scale, then `xform = translate * xform`, then
    /// `xform = rotate-about-centre * xform` (disasm `0xe4c034`/`0xe4c0ec`:
    /// `R2 = accumulator`, `this = stage`, so `stage.concatenate(acc)`).
    /// The original calls double-precision `cos`/`sin` (`0xe4c0c8`/`0xe4c0da`);
    /// `&mut` stands in for the C++ mutable matrix member.
    pub fn recalc_texture_matrix(&mut self) {
        // IDA 0xe4bfa8..0xe4bfb4: xform = Matrix4::IDENTITY
        let mut xform = Matrix4::IDENTITY;
        // IDA 0xe4bfb8..0xe4bfd8: mUScale != 1 || mVScale != 1
        if self.scale[0] != 1.0 || self.scale[1] != 1.0 {
            // IDA 0xe4bfe6..0xe4c00a: inverse scale about the texture centre.
            // VDIV/VADD flow: m00 = 1/sx, m11 = 1/sy,
            // m03 = 0.5 - 0.5/sx, m13 = 0.5 - 0.5/sy (first matrix: direct).
            let sx = 1.0 / self.scale[0];
            let sy = 1.0 / self.scale[1];
            xform.m[0][0] = sx;
            xform.m[1][1] = sy;
            xform.m[0][3] = 0.5 - 0.5 * sx;
            xform.m[1][3] = 0.5 - 0.5 * sy;
        }
        // IDA 0xe4c012..0xe4c022: mUMod != 0 || mVMod != 0 (ITT EQ double test)
        if self.scroll[0] != 0.0 || self.scroll[1] != 0.0 {
            // IDA 0xe4c050..0xe4c076: stage = IDENTITY + (m03, m13) = scroll
            let mut stage = Matrix4::IDENTITY;
            stage.m[0][3] = self.scroll[0];
            stage.m[1][3] = self.scroll[1];
            xform = Matrix4::concatenate(&stage, &xform);
        }
        // IDA 0xe4c0a6..0xe4c0b4: mRotate != 0
        if self.rotate != 0.0 {
            // IDA 0xe4c0b6..0xe4c0d4: double cos/sin, back to float
            let (sin_theta, cos_theta) = (self.rotate as f64).sin_cos();
            let (sin_theta, cos_theta) = (sin_theta as f32, cos_theta as f32);
            // IDA 0xe4c114..0xe4c15e: rotate about the texture centre:
            // row0 = (cos, -sin, 0, 0.5 - 0.5cos + 0.5sin),
            // row1 = (sin, cos, 0, 0.5 - 0.5sin - 0.5cos).
            let mut stage = Matrix4::IDENTITY;
            stage.m[0][0] = cos_theta;
            stage.m[0][1] = -sin_theta;
            stage.m[1][0] = sin_theta;
            stage.m[1][1] = cos_theta;
            stage.m[0][3] = 0.5 - 0.5 * cos_theta + 0.5 * sin_theta;
            stage.m[1][3] = 0.5 - 0.5 * sin_theta - 0.5 * cos_theta;
            xform = Matrix4::concatenate(&stage, &xform);
        }
        // IDA 0xe4c1ac..0xe4c1ca: store rows over +208, mRecalcTexMatrix = false
        self.tex_mod_matrix = xform;
        self.recalc_tex_matrix = false;
    }

    /// IDA `0xe4c1dc`: `mUMod = value`, latch set.
    pub fn set_texture_u_scroll(&mut self, value: f32) {
        // IDA 0xe4c1de: STR over +188
        self.scroll[0] = value;
        // IDA 0xe4c1e2: mRecalcTexMatrix = true
        self.recalc_tex_matrix = true;
    }

    /// IDA `0xe4c1e8`: `mVMod = value`, latch set.
    pub fn set_texture_v_scroll(&mut self, value: f32) {
        // IDA 0xe4c1ea: STR over +192
        self.scroll[1] = value;
        // IDA 0xe4c1ee: mRecalcTexMatrix = true
        self.recalc_tex_matrix = true;
    }

    /// IDA `0xe4c1f4`: `mUScale = value`, latch set.
    pub fn set_texture_u_scale(&mut self, value: f32) {
        // IDA 0xe4c1f6: STR over +196
        self.scale[0] = value;
        // IDA 0xe4c1fa: mRecalcTexMatrix = true
        self.recalc_tex_matrix = true;
    }

    /// IDA `0xe4c200`: `mVScale = value`, latch set.
    pub fn set_texture_v_scale(&mut self, value: f32) {
        // IDA 0xe4c202: STR over +200
        self.scale[1] = value;
        // IDA 0xe4c206: mRecalcTexMatrix = true
        self.recalc_tex_matrix = true;
    }

    /// IDA `0xe4c20c`: drop UV/U/V scroll effects, then add one back from the
    /// speeds. Note the outer `a2 != 0` gate at `0xe4c242`: with `uSpeed == 0`
    /// nothing is added even when `vSpeed != 0` (differs from upstream master,
    /// preserved as observed).
    pub fn set_scroll_animation(&mut self, u_speed: f32, v_speed: f32) {
        // IDA 0xe4c21e..0xe4c22e: removeEffect(UV/USCROLL/VSCROLL)
        self.remove_effect(effect_type::UV_SCROLL);
        self.remove_effect(effect_type::U_SCROLL);
        self.remove_effect(effect_type::V_SCROLL);
        // IDA 0xe4c242: uSpeed != 0
        if u_speed != 0.0 {
            // IDA 0xe4c256..0xe4c25c: equal speeds → single UVSCROLL
            if u_speed == v_speed {
                self.add_effect(TextureEffect {
                    effect_type: effect_type::UV_SCROLL,
                    arg1: u_speed,
                    ..TextureEffect::default()
                });
                return;
            }
            // IDA 0xe4c26a..0xe4c278: uSpeed != 0 → USSCROLL
            if u_speed != 0.0 {
                self.add_effect(TextureEffect {
                    effect_type: effect_type::U_SCROLL,
                    arg1: u_speed,
                    ..TextureEffect::default()
                });
            }
            // IDA 0xe4c284..0xe4c28a: vSpeed != 0 → VSCROLL
            if v_speed != 0.0 {
                self.add_effect(TextureEffect {
                    effect_type: effect_type::V_SCROLL,
                    arg1: v_speed,
                    ..TextureEffect::default()
                });
            }
        }
    }

    /// IDA `0xe4c2a0`: drop `ET_ROTATE`, add it back unless the speed is 0.
    pub fn set_rotate_animation(&mut self, speed: f32) {
        // IDA 0xe4c2ae: removeEffect(ET_ROTATE)
        self.remove_effect(effect_type::ROTATE);
        // IDA 0xe4c2be..0xe4c2ca: speed != 0 → addEffect({ET_ROTATE, speed})
        if speed != 0.0 {
            self.add_effect(TextureEffect {
                effect_type: effect_type::ROTATE,
                arg1: speed,
                ..TextureEffect::default()
            });
        }
    }

    /// IDA `0xe4c2d4`: drop the `ET_TRANSFORM` entry with this subtype, then
    /// add the new wave entry unless the wave params are all 0.
    /// FIDELITY: the gate at `0xe4c362` tests base/phase/amplitude only —
    /// frequency alone does not trigger creation; preserved as observed.
    pub fn set_transform_animation(
        &mut self,
        ttype: i32,
        wave_type: u32,
        base: f32,
        frequency: f32,
        phase: f32,
        amplitude: f32,
    ) {
        // IDA 0xe4c2e4..0xe4c328: erase the (TRANSFORM, subtype) node, if any
        if let Some(pos) = self
            .effects
            .iter()
            .position(|e| e.effect_type == effect_type::TRANSFORM && e.subtype == ttype)
        {
            self.effects.remove(pos);
        }
        // IDA 0xe4c362: base || phase || amplitude
        if base != 0.0 || phase != 0.0 || amplitude != 0.0 {
            // IDA 0xe4c368..0xe4c382: eff = {TRANSFORM, ...}; addEffect
            self.add_effect(TextureEffect {
                effect_type: effect_type::TRANSFORM,
                subtype: ttype,
                wave_type,
                base,
                frequency,
                phase,
                amplitude,
                ..TextureEffect::default()
            });
        }
    }

    /// IDA `0xe4c390`: prepare every frame texture.
    pub fn prepare(&mut self) {
        // IDA 0xe4c396..0xe4c3b6: HIDWORD(size) != size → ensurePrepared(i++)
        for i in 0..self.frames.len() as u32 {
            self.ensure_prepared(i);
        }
    }
}

// 0xe49a3c — __ZN4Ogre16TextureUnitState7_unloadEv
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this)
#[doc(alias = "Ogre::TextureUnitState::_unload(void)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState7_unloadEv")]
// was: Ogre::TextureUnitState::_unload(void)
// IDA 0xe49a3c: destroys anim + effect controllers, releases texture refs (see TextureUnitState::unload).
pub fn stub_0xe49a3c(state: &mut TextureUnitState) {
    state.unload()
}

// 0xe49ac4 — __ZNK4Ogre16TextureUnitState8isLoadedEv
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this)
#[doc(alias = "Ogre::TextureUnitState::isLoaded(void)const")]
#[doc(alias = "__ZNK4Ogre16TextureUnitState8isLoadedEv")]
// was: Ogre::TextureUnitState::isLoaded(void)const
// IDA 0xe49ac4: Pass::isLoaded(mParent) (see TextureUnitState::is_loaded).
pub fn stub_0xe49ac4(state: &TextureUnitState) -> bool {
    state.is_loaded()
}

// 0xe49ad4 — __ZN4Ogre16TextureUnitState5_loadEv
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this)
#[doc(alias = "Ogre::TextureUnitState::_load(void)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState5_loadEv")]
// was: Ogre::TextureUnitState::_load(void)
// IDA 0xe49ad4: ensure-loads every frame, rebuilds the animator when animated (see TextureUnitState::load).
pub fn stub_0xe49ad4(state: &mut TextureUnitState) {
    state.load()
}

// 0xe49b54 — __ZNK4Ogre16TextureUnitState14getTextureNameEv
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this)
#[doc(alias = "Ogre::TextureUnitState::getTextureName(void)const")]
#[doc(alias = "__ZNK4Ogre16TextureUnitState14getTextureNameEv")]
// was: Ogre::TextureUnitState::getTextureName(void)const
// IDA 0xe49b54: out-of-range frame yields StringUtil::BLANK (see TextureUnitState::texture_name).
pub fn stub_0xe49b54(state: &TextureUnitState) -> &str {
    state.texture_name()
}

// 0xe49b7c — __ZN4Ogre16TextureUnitState14setContentTypeENS0_11ContentTypeE
#[doc(alias = "Ogre::TextureUnitState::setContentType(Ogre::TextureUnitState::ContentType)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState14setContentTypeENS0_11ContentTypeE")]
// was: Ogre::TextureUnitState::setContentType(Ogre::TextureUnitState::ContentType)
// IDA 0xe49b7c: stores content type; shadow values clear mFrames (see TextureUnitState::set_content_type).
pub fn stub_0xe49b7c(state: &mut TextureUnitState, content: ContentType) {
    state.set_content_type(content as u32)
}

// 0xe49dec — __ZN4Ogre16TextureUnitState19setCubicTextureNameERKSsb
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, const std::string *, bool)
#[doc(alias = "Ogre::TextureUnitState::setCubicTextureName(std::string const&,bool)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState19setCubicTextureNameERKSsb")]
// was: Ogre::TextureUnitState::setCubicTextureName(std::string const&,bool)
// IDA 0xe49dec: builds the six face names then delegates to setCubicTextureName(ptr) at 0xe49e5e/0xe4a0a4.
pub fn stub_0xe49dec(state: &mut TextureUnitState, name: &str, for_uvw: bool) {
    let faces = TextureUnitState::cubic_face_names(name);
    state.set_cubic_texture_name(&faces, for_uvw)
}

// 0xe4a90c — __ZN4Ogre16TextureUnitState14setBindingTypeENS0_11BindingTypeE
#[doc(alias = "Ogre::TextureUnitState::setBindingType(Ogre::TextureUnitState::BindingType)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState14setBindingTypeENS0_11BindingTypeE")]
// was: Ogre::TextureUnitState::setBindingType(Ogre::TextureUnitState::BindingType)
// IDA 0xe4a90c: STR R1,[R0,#296] (mBindingType).
pub fn stub_0xe4a90c(state: &mut TextureUnitState, binding: BindingType) {
    state.binding_type = binding
}

// 0xe4a914 — __ZNK4Ogre16TextureUnitState14getBindingTypeEv
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this)
#[doc(alias = "Ogre::TextureUnitState::getBindingType(void)const")]
#[doc(alias = "__ZNK4Ogre16TextureUnitState14getBindingTypeEv")]
// was: Ogre::TextureUnitState::getBindingType(void)const
// IDA 0xe4a918: LDR R0,[R0,#296] (mBindingType).
pub fn stub_0xe4a914(state: &TextureUnitState) -> BindingType {
    state.binding_type
}

// 0xe4a91c — __ZNK4Ogre16TextureUnitState14getContentTypeEv
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this)
#[doc(alias = "Ogre::TextureUnitState::getContentType(void)const")]
#[doc(alias = "__ZNK4Ogre16TextureUnitState14getContentTypeEv")]
// was: Ogre::TextureUnitState::getContentType(void)const
// IDA 0xe4a920: LDR R0,[R0,#300] (mContentType).
pub fn stub_0xe4a91c(state: &TextureUnitState) -> ContentType {
    ContentType::from_raw(state.content_type)
}

// 0xe4a924 — __ZN4Ogre16TextureUnitState19setCubicTextureNameEPKSsb
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, const std::string *, bool)
#[doc(alias = "Ogre::TextureUnitState::setCubicTextureName(std::string const*,bool)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState19setCubicTextureNameEPKSsb")]
// was: Ogre::TextureUnitState::setCubicTextureName(std::string const*,bool)
// IDA 0xe4a924: installs cube names, type = forUVW ? 3D : 2D, notifies parent (see set_cubic_texture_name).
pub fn stub_0xe4a924(state: &mut TextureUnitState, names: &[String], for_uvw: bool) {
    state.set_cubic_texture_name(names, for_uvw)
}

// 0xe4aca8 — __ZNK4Ogre16TextureUnitState4is3DEv
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this)
#[doc(alias = "Ogre::TextureUnitState::is3D(void)const")]
#[doc(alias = "__ZNK4Ogre16TextureUnitState4is3DEv")]
// was: Ogre::TextureUnitState::is3D(void)const
// IDA 0xe4acb2: return mTextureType == 4.
pub fn stub_0xe4aca8(state: &TextureUnitState) -> bool {
    state.is_3d()
}

// 0xe4acb4 — __ZNK4Ogre16TextureUnitState14getTextureTypeEv
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this)
#[doc(alias = "Ogre::TextureUnitState::getTextureType(void)const")]
#[doc(alias = "__ZNK4Ogre16TextureUnitState14getTextureTypeEv")]
// was: Ogre::TextureUnitState::getTextureType(void)const
// IDA 0xe4acb6: LDR R0,[R0,#12] (mTextureType).
pub fn stub_0xe4acb4(state: &TextureUnitState) -> u32 {
    state.texture_type
}

// 0xe4acb8 — __ZN4Ogre16TextureUnitState22setAnimatedTextureNameERKSsjf
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, const std::string *, unsigned int, float)
#[doc(alias = "Ogre::TextureUnitState::setAnimatedTextureName(std::string const&,unsigned int,float)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState22setAnimatedTextureNameERKSsjf")]
// was: Ogre::TextureUnitState::setAnimatedTextureName(std::string const&,unsigned int,float)
// IDA 0xe4acb8: builds stem_N names via stringstream, installs them, reloads when loaded (see animated_frame_names).
pub fn stub_0xe4acb8(state: &mut TextureUnitState, name: &str, num_frames: u32, duration: f32) {
    let names = TextureUnitState::animated_frame_names(name, num_frames);
    state.set_animated_texture_names(&names, num_frames, duration)
}

// 0xe4b538 — __ZN4Ogre16TextureUnitState22setAnimatedTextureNameEPKSsjf
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, const std::string *, unsigned int, float)
#[doc(alias = "Ogre::TextureUnitState::setAnimatedTextureName(std::string const*,unsigned int,float)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState22setAnimatedTextureNameEPKSsjf")]
// was: Ogre::TextureUnitState::setAnimatedTextureName(std::string const*,unsigned int,float)
// IDA 0xe4b538: installs animated names plus duration (see set_animated_texture_names).
pub fn stub_0xe4b538(state: &mut TextureUnitState, names: &[String], num_frames: u32, duration: f32) {
    state.set_animated_texture_names(names, num_frames, duration)
}

// 0xe4b8f0 — __ZNK4Ogre16TextureUnitState14_getTexturePtrEm
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, unsigned int)
#[doc(alias = "Ogre::TextureUnitState::_getTexturePtr(unsigned long)const")]
#[doc(alias = "__ZNK4Ogre16TextureUnitState14_getTexturePtrEm")]
// was: Ogre::TextureUnitState::_getTexturePtr(unsigned long)const
// IDA 0xe4b8f0: direct index for content textures, lazy ensureLoaded for frames, blank ptr else (see texture_ptr).
pub fn stub_0xe4b8f0(state: &mut TextureUnitState, index: u32) -> Option<&TextureSlot> {
    state.texture_ptr(index)
}

// 0xe4b98c — __ZN4Ogre16TextureUnitState15setCurrentFrameEj
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, unsigned int)
#[doc(alias = "Ogre::TextureUnitState::setCurrentFrame(unsigned int)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState15setCurrentFrameEj")]
// was: Ogre::TextureUnitState::setCurrentFrame(unsigned int)
// IDA 0xe4b98c: out-of-range throws InvalidParametersException (see set_current_frame).
pub fn stub_0xe4b98c(state: &mut TextureUnitState, frame: u32) {
    state.set_current_frame(frame)
}

// 0xe4bb98 — __ZNK4Ogre16TextureUnitState15getCurrentFrameEv
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this)
#[doc(alias = "Ogre::TextureUnitState::getCurrentFrame(void)const")]
#[doc(alias = "__ZNK4Ogre16TextureUnitState15getCurrentFrameEv")]
// was: Ogre::TextureUnitState::getCurrentFrame(void)const
// IDA 0xe4bb9a: LDR R0,[R0] (mCurrentFrame).
pub fn stub_0xe4bb98(state: &TextureUnitState) -> u32 {
    state.get_current_frame()
}

// 0xe4bb9c — __ZNK4Ogre16TextureUnitState12getNumFramesEv
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this)
#[doc(alias = "Ogre::TextureUnitState::getNumFrames(void)const")]
#[doc(alias = "__ZNK4Ogre16TextureUnitState12getNumFramesEv")]
// was: Ogre::TextureUnitState::getNumFrames(void)const
// IDA 0xe4bba4: (end - begin) >> 2 = frame count.
pub fn stub_0xe4bb9c(state: &TextureUnitState) -> u32 {
    state.num_frames()
}

// 0xe4bba8 — __ZN4Ogre16TextureUnitState16setDesiredFormatENS_11PixelFormatE
#[doc(alias = "Ogre::TextureUnitState::setDesiredFormat(Ogre::PixelFormat)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState16setDesiredFormatENS_11PixelFormatE")]
// was: Ogre::TextureUnitState::setDesiredFormat(Ogre::PixelFormat)
// IDA 0xe4bba8: STR R1,[R0,#16] (mDesiredFormat).
pub fn stub_0xe4bba8(state: &mut TextureUnitState, format: u32) {
    state.desired_format = format
}

// 0xe4bbac — __ZN4Ogre16TextureUnitState13setNumMipmapsEi
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, int)
#[doc(alias = "Ogre::TextureUnitState::setNumMipmaps(int)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState13setNumMipmapsEi")]
// was: Ogre::TextureUnitState::setNumMipmaps(int)
// IDA 0xe4bbac: STR R1,[R0,#20] (mNumMipmaps).
pub fn stub_0xe4bbac(state: &mut TextureUnitState, count: i32) {
    state.num_mipmaps = count
}

// 0xe4bbb0 — __ZN4Ogre16TextureUnitState10setIsAlphaEb
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, bool)
#[doc(alias = "Ogre::TextureUnitState::setIsAlpha(bool)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState10setIsAlphaEb")]
// was: Ogre::TextureUnitState::setIsAlpha(bool)
// IDA 0xe4bbb0: STRB R1,[R0,#185] (mIsAlpha).
pub fn stub_0xe4bbb0(state: &mut TextureUnitState, is_alpha: bool) {
    state.is_alpha = is_alpha
}

// 0xe4bbb8 — __ZN4Ogre16TextureUnitState23setHardwareGammaEnabledEb
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, bool)
#[doc(alias = "Ogre::TextureUnitState::setHardwareGammaEnabled(bool)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState23setHardwareGammaEnabledEb")]
// was: Ogre::TextureUnitState::setHardwareGammaEnabled(bool)
// IDA 0xe4bbb8: STRB R1,[R0,#186] (mHwGammaEnabled).
pub fn stub_0xe4bbb8(state: &mut TextureUnitState, enabled: bool) {
    state.hw_gamma_enabled = enabled
}

// 0xe4bbc0 — __ZNK4Ogre16TextureUnitState18getTextureCoordSetEv
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this)
#[doc(alias = "Ogre::TextureUnitState::getTextureCoordSet(void)const")]
#[doc(alias = "__ZNK4Ogre16TextureUnitState18getTextureCoordSetEv")]
// was: Ogre::TextureUnitState::getTextureCoordSet(void)const
// IDA 0xe4bbc2: LDR R0,[R0,#24] (mTextureCoordSetIndex).
pub fn stub_0xe4bbc0(state: &TextureUnitState) -> u32 {
    state.texture_coord_set
}

// 0xe4bbc4 — __ZN4Ogre16TextureUnitState20setColourOperationExENS_21LayerBlendOperationExENS_16LayerBlendSourceES2_RKNS_11ColourValueES5_f
// type: int __fastcall(int, int, int, int, int, int, float)
#[doc(alias = "Ogre::TextureUnitState::setColourOperationEx(Ogre::LayerBlendOperationEx,Ogre::LayerBlendSource,Ogre::LayerBlendSource,Ogre::ColourValue const&,Ogre::ColourValue const&,float)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState20setColourOperationExENS_21LayerBlendOperationExENS_16LayerBlendSourceES2_RKNS_11ColourValueES5_f")]
// was: Ogre::TextureUnitState::setColourOperationEx(Ogre::LayerBlendOperationEx,Ogre::LayerBlendSource,Ogre::LayerBlendSource,Ogre::ColourValue const&,Ogre::ColourValue const&,float)
// IDA 0xe4bbc4: stores op/sources/args/factor at +60..+112 (see set_colour_operation_ex).
pub fn stub_0xe4bbc4(
    state: &mut TextureUnitState,
    op: u32,
    src1: u32,
    src2: u32,
    arg1: ColourValue,
    arg2: ColourValue,
    factor: f32,
) {
    state.set_colour_operation_ex(op, src1, src2, arg1, arg2, factor)
}

// 0xe4bbf8 — __ZN4Ogre16TextureUnitState28setColourOpMultipassFallbackENS_16SceneBlendFactorES1_
#[doc(alias = "Ogre::TextureUnitState::setColourOpMultipassFallback(Ogre::SceneBlendFactor,Ogre::SceneBlendFactor)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState28setColourOpMultipassFallbackENS_16SceneBlendFactorES1_")]
// was: Ogre::TextureUnitState::setColourOpMultipassFallback(Ogre::SceneBlendFactor,Ogre::SceneBlendFactor)
// IDA 0xe4bbfc: STRD R1,R2,[R0,#116] (mColourBlendFallbackSrc/Dest).
pub fn stub_0xe4bbf8(state: &mut TextureUnitState, src: SceneBlendFactor, dst: SceneBlendFactor) {
    state.set_colour_op_multipass_fallback(src, dst)
}

// 0xe4bc04 — __ZN4Ogre16TextureUnitState17setAlphaOperationENS_21LayerBlendOperationExENS_16LayerBlendSourceES2_fff
// type: int __fastcall(int, int, int, int, float, float, float)
#[doc(alias = "Ogre::TextureUnitState::setAlphaOperation(Ogre::LayerBlendOperationEx,Ogre::LayerBlendSource,Ogre::LayerBlendSource,float,float,float)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState17setAlphaOperationENS_21LayerBlendOperationExENS_16LayerBlendSourceES2_fff")]
// was: Ogre::TextureUnitState::setAlphaOperation(Ogre::LayerBlendOperationEx,Ogre::LayerBlendSource,Ogre::LayerBlendSource,float,float,float)
// IDA 0xe4bc04: stores op/sources/args/factor at +128..+180 (see set_alpha_operation).
pub fn stub_0xe4bc04(
    state: &mut TextureUnitState,
    op: u32,
    src1: u32,
    src2: u32,
    arg1: f32,
    arg2: f32,
    factor: f32,
) {
    state.set_alpha_operation(op, src1, src2, arg1, arg2, factor)
}

// 0xe4bc2c — __ZN4Ogre16TextureUnitState9addEffectERNS0_13TextureEffectE
#[doc(alias = "Ogre::TextureUnitState::addEffect(Ogre::TextureUnitState::TextureEffect &)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState9addEffectERNS0_13TextureEffectE")]
// was: Ogre::TextureUnitState::addEffect(Ogre::TextureUnitState::TextureEffect &)
// IDA 0xe4bc2c: null controller, replace same-type entry, insert (see add_effect).
pub fn stub_0xe4bc2c(state: &mut TextureUnitState, effect: &TextureEffect) {
    state.add_effect(effect.clone())
}

// 0xe4bd68 — __ZN4Ogre16TextureUnitState22createEffectControllerERNS0_13TextureEffectE
#[doc(alias = "Ogre::TextureUnitState::createEffectController(Ogre::TextureUnitState::TextureEffect &)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState22createEffectControllerERNS0_13TextureEffectE")]
// was: Ogre::TextureUnitState::createEffectController(Ogre::TextureUnitState::TextureEffect &)
// IDA 0xe4bd68: destroy old controller, allocate per effect type (see create_effect_controller).
// Original takes `TextureEffect &`; the index selects the element in `mEffects`.
pub fn stub_0xe4bd68(state: &mut TextureUnitState, index: usize) {
    state.create_effect_controller(index)
}

// 0xe4bdf8 — __ZNK4Ogre16TextureUnitState25getColourBlendFallbackSrcEv
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this)
#[doc(alias = "Ogre::TextureUnitState::getColourBlendFallbackSrc(void)const")]
#[doc(alias = "__ZNK4Ogre16TextureUnitState25getColourBlendFallbackSrcEv")]
// was: Ogre::TextureUnitState::getColourBlendFallbackSrc(void)const
// IDA 0xe4bdf8: LDR R0,[R0,#116] (mColourBlendFallbackSrc).
pub fn stub_0xe4bdf8(state: &TextureUnitState) -> SceneBlendFactor {
    state.colour_blend_fallback_src()
}

// 0xe4bdfc — __ZNK4Ogre16TextureUnitState26getColourBlendFallbackDestEv
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this)
#[doc(alias = "Ogre::TextureUnitState::getColourBlendFallbackDest(void)const")]
#[doc(alias = "__ZNK4Ogre16TextureUnitState26getColourBlendFallbackDestEv")]
// was: Ogre::TextureUnitState::getColourBlendFallbackDest(void)const
// IDA 0xe4bdfc: LDR R0,[R0,#120] (mColourBlendFallbackDest).
pub fn stub_0xe4bdfc(state: &TextureUnitState) -> SceneBlendFactor {
    state.colour_blend_fallback_dst()
}

// 0xe4be00 — __ZNK4Ogre16TextureUnitState18getColourBlendModeEv
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this)
#[doc(alias = "Ogre::TextureUnitState::getColourBlendMode(void)const")]
#[doc(alias = "__ZNK4Ogre16TextureUnitState18getColourBlendModeEv")]
// was: Ogre::TextureUnitState::getColourBlendMode(void)const
// IDA 0xe4be00: ADDS R0,#56 (mColourBlendMode).
pub fn stub_0xe4be00(state: &TextureUnitState) -> &LayerBlendModeEx {
    state.colour_blend_mode()
}

// 0xe4be04 — __ZNK4Ogre16TextureUnitState17getAlphaBlendModeEv
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this)
#[doc(alias = "Ogre::TextureUnitState::getAlphaBlendMode(void)const")]
#[doc(alias = "__ZNK4Ogre16TextureUnitState17getAlphaBlendModeEv")]
// was: Ogre::TextureUnitState::getAlphaBlendMode(void)const
// IDA 0xe4be04: ADDS R0,#124 (mAlphaBlendMode).
pub fn stub_0xe4be04(state: &TextureUnitState) -> &LayerBlendModeEx {
    state.alpha_blend_mode()
}

// 0xe4be08 — __ZNK4Ogre16TextureUnitState24getTextureAddressingModeEv
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this)
#[doc(alias = "Ogre::TextureUnitState::getTextureAddressingMode(void)const")]
#[doc(alias = "__ZNK4Ogre16TextureUnitState24getTextureAddressingModeEv")]
// was: Ogre::TextureUnitState::getTextureAddressingMode(void)const
// IDA 0xe4be08: ADDS R0,#28 (mAddressMode).
pub fn stub_0xe4be08(state: &TextureUnitState) -> &UvwAddressingMode {
    state.texture_addressing_mode()
}

// 0xe4be0c — __ZN4Ogre16TextureUnitState24setTextureAddressingModeERKNS0_17UVWAddressingModeE
#[doc(alias = "Ogre::TextureUnitState::setTextureAddressingMode(Ogre::TextureUnitState::UVWAddressingMode const&)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState24setTextureAddressingModeERKNS0_17UVWAddressingModeE")]
// was: Ogre::TextureUnitState::setTextureAddressingMode(Ogre::TextureUnitState::UVWAddressingMode const&)
// IDA 0xe4be0c: VLDR pair + STR over +28/+32/+36 (see set_texture_addressing_mode).
pub fn stub_0xe4be0c(state: &mut TextureUnitState, mode: &UvwAddressingMode) {
    state.set_texture_addressing_mode(*mode)
}

// 0xe4be1c — __ZN4Ogre16TextureUnitState22setTextureBorderColourERKNS_11ColourValueE
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, const Ogre::ColourValue *)
#[doc(alias = "Ogre::TextureUnitState::setTextureBorderColour(Ogre::ColourValue const&)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState22setTextureBorderColourERKNS_11ColourValueE")]
// was: Ogre::TextureUnitState::setTextureBorderColour(Ogre::ColourValue const&)
// IDA 0xe4be1c: VLD1.32 pair over +40 (see set_texture_border_colour).
pub fn stub_0xe4be1c(state: &mut TextureUnitState, colour: &ColourValue) {
    state.set_texture_border_colour(*colour)
}

// 0xe4be28 — __ZNK4Ogre16TextureUnitState22getTextureBorderColourEv
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this)
#[doc(alias = "Ogre::TextureUnitState::getTextureBorderColour(void)const")]
#[doc(alias = "__ZNK4Ogre16TextureUnitState22getTextureBorderColourEv")]
// was: Ogre::TextureUnitState::getTextureBorderColour(void)const
// IDA 0xe4be28: ADDS R0,#40 (mBorderColour).
pub fn stub_0xe4be28(state: &TextureUnitState) -> &ColourValue {
    state.texture_border_colour()
}

// 0xe4be2c — __ZN4Ogre16TextureUnitState17setEnvironmentMapEbNS0_10EnvMapTypeE
#[doc(alias = "Ogre::TextureUnitState::setEnvironmentMap(bool,Ogre::TextureUnitState::EnvMapType)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState17setEnvironmentMapEbNS0_10EnvMapTypeE")]
// was: Ogre::TextureUnitState::setEnvironmentMap(bool,Ogre::TextureUnitState::EnvMapType)
// IDA 0xe4be2c: enable ? addEffect({ET_ENVIRONMENT_MAP, subtype}) : removeEffect(0).
pub fn stub_0xe4be2c(state: &mut TextureUnitState, enable: bool, env_map_type: i32) {
    state.set_environment_map(enable, env_map_type)
}

// 0xe4be50 — __ZN4Ogre16TextureUnitState12removeEffectENS0_17TextureEffectTypeE
#[doc(alias = "Ogre::TextureUnitState::removeEffect(Ogre::TextureUnitState::TextureEffectType)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState12removeEffectENS0_17TextureEffectTypeE")]
// was: Ogre::TextureUnitState::removeEffect(Ogre::TextureUnitState::TextureEffectType)
// IDA 0xe4be50: destroy controllers in equal_range(type), erase the span (see remove_effect).
pub fn stub_0xe4be50(state: &mut TextureUnitState, effect_type_: u32) {
    state.remove_effect(effect_type_)
}

// 0xe4bec8 — __ZN4Ogre16TextureUnitState19setTextureTransformERKNS_7Matrix4E
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, const Ogre::Matrix4 *)
#[doc(alias = "Ogre::TextureUnitState::setTextureTransform(Ogre::Matrix4 const&)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState19setTextureTransformERKNS_7Matrix4E")]
// was: Ogre::TextureUnitState::setTextureTransform(Ogre::Matrix4 const&)
// IDA 0xe4bec8: 64-byte copy over +208, latch clear (see set_texture_transform).
pub fn stub_0xe4bec8(state: &mut TextureUnitState, xform: &Matrix4) {
    state.set_texture_transform(xform)
}

// 0xe4bf0c — __ZN4Ogre16TextureUnitState16setTextureScrollEff
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, float, float)
#[doc(alias = "Ogre::TextureUnitState::setTextureScroll(float,float)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState16setTextureScrollEff")]
// was: Ogre::TextureUnitState::setTextureScroll(float,float)
// IDA 0xe4bf0c: STRD u,v over +188; latch set (see set_texture_scroll).
pub fn stub_0xe4bf0c(state: &mut TextureUnitState, u: f32, v: f32) {
    state.set_texture_scroll(u, v)
}

// 0xe4bf1c — __ZN4Ogre16TextureUnitState15setTextureScaleEff
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, float, float)
#[doc(alias = "Ogre::TextureUnitState::setTextureScale(float,float)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState15setTextureScaleEff")]
// was: Ogre::TextureUnitState::setTextureScale(float,float)
// IDA 0xe4bf1c: STRD over +196; latch set (see set_texture_scale).
pub fn stub_0xe4bf1c(state: &mut TextureUnitState, u_scale: f32, v_scale: f32) {
    state.set_texture_scale(u_scale, v_scale)
}

// 0xe4bf2c — __ZN4Ogre16TextureUnitState16setTextureRotateERKNS_6RadianE
#[doc(alias = "Ogre::TextureUnitState::setTextureRotate(Ogre::Radian const&)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState16setTextureRotateERKNS_6RadianE")]
// was: Ogre::TextureUnitState::setTextureRotate(Ogre::Radian const&)
// IDA 0xe4bf2c: STR Radian over +204; latch set (see set_texture_rotate).
pub fn stub_0xe4bf2c(state: &mut TextureUnitState, angle: f32) {
    state.set_texture_rotate(angle)
}

// 0xe4bf3c — __ZNK4Ogre16TextureUnitState19getTextureTransformEv
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this)
#[doc(alias = "Ogre::TextureUnitState::getTextureTransform(void)const")]
#[doc(alias = "__ZNK4Ogre16TextureUnitState19getTextureTransformEv")]
// was: Ogre::TextureUnitState::getTextureTransform(void)const
// IDA 0xe4bf3c: latch → recalcTextureMatrix; return this + 208 (see texture_transform).
pub fn stub_0xe4bf3c(state: &mut TextureUnitState) -> &Matrix4 {
    state.texture_transform()
}

// 0xe4bf58 — __ZNK4Ogre16TextureUnitState19recalcTextureMatrixEv
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this)
#[doc(alias = "Ogre::TextureUnitState::recalcTextureMatrix(void)const")]
#[doc(alias = "__ZNK4Ogre16TextureUnitState19recalcTextureMatrixEv")]
// was: Ogre::TextureUnitState::recalcTextureMatrix(void)const
// IDA 0xe4bf58: rebuild mTexModMatrix from scale/scroll/rotate (see recalc_texture_matrix).
// Original is const (mutable member); `&mut` stands in here.
pub fn stub_0xe4bf58(state: &mut TextureUnitState) {
    state.recalc_texture_matrix()
}

// 0xe4c1dc — __ZN4Ogre16TextureUnitState17setTextureUScrollEf
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, float)
#[doc(alias = "Ogre::TextureUnitState::setTextureUScroll(float)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState17setTextureUScrollEf")]
// was: Ogre::TextureUnitState::setTextureUScroll(float)
// IDA 0xe4c1dc: STR over +188 (mUMod); latch set.
pub fn stub_0xe4c1dc(state: &mut TextureUnitState, value: f32) {
    state.set_texture_u_scroll(value)
}

// 0xe4c1e8 — __ZN4Ogre16TextureUnitState17setTextureVScrollEf
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, float)
#[doc(alias = "Ogre::TextureUnitState::setTextureVScroll(float)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState17setTextureVScrollEf")]
// was: Ogre::TextureUnitState::setTextureVScroll(float)
// IDA 0xe4c1e8: STR over +192 (mVMod); latch set.
pub fn stub_0xe4c1e8(state: &mut TextureUnitState, value: f32) {
    state.set_texture_v_scroll(value)
}

// 0xe4c1f4 — __ZN4Ogre16TextureUnitState16setTextureUScaleEf
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, float)
#[doc(alias = "Ogre::TextureUnitState::setTextureUScale(float)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState16setTextureUScaleEf")]
// was: Ogre::TextureUnitState::setTextureUScale(float)
// IDA 0xe4c1f4: STR over +196 (mUScale); latch set.
pub fn stub_0xe4c1f4(state: &mut TextureUnitState, value: f32) {
    state.set_texture_u_scale(value)
}

// 0xe4c200 — __ZN4Ogre16TextureUnitState16setTextureVScaleEf
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, float)
#[doc(alias = "Ogre::TextureUnitState::setTextureVScale(float)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState16setTextureVScaleEf")]
// was: Ogre::TextureUnitState::setTextureVScale(float)
// IDA 0xe4c200: STR over +200 (mVScale); latch set.
pub fn stub_0xe4c200(state: &mut TextureUnitState, value: f32) {
    state.set_texture_v_scale(value)
}

// 0xe4c20c — __ZN4Ogre16TextureUnitState18setScrollAnimationEff
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, float, float)
#[doc(alias = "Ogre::TextureUnitState::setScrollAnimation(float,float)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState18setScrollAnimationEff")]
// was: Ogre::TextureUnitState::setScrollAnimation(float,float)
// IDA 0xe4c20c: drop UV/U/V effects, re-add from speeds (see set_scroll_animation).
pub fn stub_0xe4c20c(state: &mut TextureUnitState, u_speed: f32, v_speed: f32) {
    state.set_scroll_animation(u_speed, v_speed)
}

// 0xe4c2a0 — __ZN4Ogre16TextureUnitState18setRotateAnimationEf
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, float)
#[doc(alias = "Ogre::TextureUnitState::setRotateAnimation(float)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState18setRotateAnimationEf")]
// was: Ogre::TextureUnitState::setRotateAnimation(float)
// IDA 0xe4c2a0: drop ET_ROTATE, re-add unless speed is 0 (see set_rotate_animation).
pub fn stub_0xe4c2a0(state: &mut TextureUnitState, speed: f32) {
    state.set_rotate_animation(speed)
}

// 0xe4c2d4 — __ZN4Ogre16TextureUnitState21setTransformAnimationENS0_20TextureTransformTypeENS_12WaveformTypeEffff
// type: int __fastcall(int, int, int, int, float, float, float)
#[doc(alias = "Ogre::TextureUnitState::setTransformAnimation(Ogre::TextureUnitState::TextureTransformType,Ogre::WaveformType,float,float,float,float)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState21setTransformAnimationENS0_20TextureTransformTypeENS_12WaveformTypeEffff")]
// was: Ogre::TextureUnitState::setTransformAnimation(Ogre::TextureUnitState::TextureTransformType,Ogre::WaveformType,float,float,float,float)
// IDA 0xe4c2d4: drop (TRANSFORM, subtype), add wave entry unless gated (see set_transform_animation).
pub fn stub_0xe4c2d4(
    state: &mut TextureUnitState,
    ttype: i32,
    wave_type: u32,
    base: f32,
    frequency: f32,
    phase: f32,
    amplitude: f32,
) {
    state.set_transform_animation(ttype, wave_type, base, frequency, phase, amplitude)
}

// 0xe4c390 — __ZN4Ogre16TextureUnitState8_prepareEv
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this)
#[doc(alias = "Ogre::TextureUnitState::_prepare(void)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState8_prepareEv")]
// was: Ogre::TextureUnitState::_prepare(void)
// IDA 0xe4c390: for each frame: ensurePrepared(i) (see prepare).
pub fn stub_0xe4c390(state: &mut TextureUnitState) {
    state.prepare()
}

// 0xe4c3bc — __ZNK4Ogre16TextureUnitState14ensurePreparedEm
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, unsigned int)
#[doc(alias = "Ogre::TextureUnitState::ensurePrepared(unsigned long)const")]
#[doc(alias = "__ZNK4Ogre16TextureUnitState14ensurePreparedEm")]
// was: Ogre::TextureUnitState::ensurePrepared(unsigned long)const
// IDA 0xe4c3bc: 609 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4c3bc() {
}

// 0xe4ca5c — __ZNK4Ogre16TextureUnitState12ensureLoadedEm
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, unsigned int)
#[doc(alias = "Ogre::TextureUnitState::ensureLoaded(unsigned long)const")]
#[doc(alias = "__ZNK4Ogre16TextureUnitState12ensureLoadedEm")]
// was: Ogre::TextureUnitState::ensureLoaded(unsigned long)const
// IDA 0xe4ca5c: 609 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4ca5c() {
}

// 0xe4d0fc — __ZNK4Ogre16TextureUnitState14_getTexturePtrEv
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this)
#[doc(alias = "Ogre::TextureUnitState::_getTexturePtr(void)const")]
#[doc(alias = "__ZNK4Ogre16TextureUnitState14_getTexturePtrEv")]
// was: Ogre::TextureUnitState::_getTexturePtr(void)const
// IDA 0xe4d0fc: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4d0fc() {
}

// 0xe4d108 — __ZN4Ogre16TextureUnitState14_setTexturePtrERKNS_10TexturePtrE
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, const Ogre::TexturePtr *)
#[doc(alias = "Ogre::TextureUnitState::_setTexturePtr(Ogre::TexturePtr const&)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState14_setTexturePtrERKNS_10TexturePtrE")]
// was: Ogre::TextureUnitState::_setTexturePtr(Ogre::TexturePtr const&)
// IDA 0xe4d108: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4d108() {
}

// 0xe4d11c — __ZNK4Ogre16TextureUnitState10getEffectsEv
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this)
#[doc(alias = "Ogre::TextureUnitState::getEffects(void)const")]
#[doc(alias = "__ZNK4Ogre16TextureUnitState10getEffectsEv")]
// was: Ogre::TextureUnitState::getEffects(void)const
// IDA 0xe4d11c: 2 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4d11c() {
}

// 0xe4d124 — __ZN4Ogre16TextureUnitState19setTextureFilteringENS_20TextureFilterOptionsE
#[doc(alias = "Ogre::TextureUnitState::setTextureFiltering(Ogre::TextureFilterOptions)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState19setTextureFilteringENS_20TextureFilterOptionsE")]
// was: Ogre::TextureUnitState::setTextureFiltering(Ogre::TextureFilterOptions)
// IDA 0xe4d124: 28 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4d124() {
}

// 0xe4d178 — __ZN4Ogre16TextureUnitState19setTextureFilteringENS_13FilterOptionsES1_S1_
#[doc(alias = "Ogre::TextureUnitState::setTextureFiltering(Ogre::FilterOptions,Ogre::FilterOptions,Ogre::FilterOptions)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState19setTextureFilteringENS_13FilterOptionsES1_S1_")]
// was: Ogre::TextureUnitState::setTextureFiltering(Ogre::FilterOptions,Ogre::FilterOptions,Ogre::FilterOptions)
// IDA 0xe4d178: 5 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4d178() {
}

// 0xe4d188 — __ZNK4Ogre16TextureUnitState19getTextureFilteringENS_10FilterTypeE
#[doc(alias = "Ogre::TextureUnitState::getTextureFiltering(Ogre::FilterType)const")]
#[doc(alias = "__ZNK4Ogre16TextureUnitState19getTextureFilteringENS_10FilterTypeE")]
// was: Ogre::TextureUnitState::getTextureFiltering(Ogre::FilterType)const
// IDA 0xe4d188: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4d188() {
}

// 0xe4d1f0 — __ZN4Ogre16TextureUnitState20setTextureAnisotropyEj
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, unsigned int)
#[doc(alias = "Ogre::TextureUnitState::setTextureAnisotropy(unsigned int)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState20setTextureAnisotropyEj")]
// was: Ogre::TextureUnitState::setTextureAnisotropy(unsigned int)
// IDA 0xe4d1f0: 4 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4d1f0() {
}

// 0xe4d1fc — __ZNK4Ogre16TextureUnitState20getTextureAnisotropyEv
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this)
#[doc(alias = "Ogre::TextureUnitState::getTextureAnisotropy(void)const")]
#[doc(alias = "__ZNK4Ogre16TextureUnitState20getTextureAnisotropyEv")]
// was: Ogre::TextureUnitState::getTextureAnisotropy(void)const
// IDA 0xe4d1fc: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4d1fc() {
}

// 0xe4d218 — __ZN4Ogre16TextureUnitState10_unprepareEv
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this)
#[doc(alias = "Ogre::TextureUnitState::_unprepare(void)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState10_unprepareEv")]
// was: Ogre::TextureUnitState::_unprepare(void)
// IDA 0xe4d218: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4d218() {
}

// 0xe4d2d4 — __ZN4Ogre16TextureUnitState22setProjectiveTexturingEbPKNS_7FrustumE
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, bool, const Ogre::Frustum *)
#[doc(alias = "Ogre::TextureUnitState::setProjectiveTexturing(bool,Ogre::Frustum const*)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState22setProjectiveTexturingEbPKNS_7FrustumE")]
// was: Ogre::TextureUnitState::setProjectiveTexturing(bool,Ogre::Frustum const*)
// IDA 0xe4d2d4: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4d2d4() {
}

// 0xe4d2f8 — __ZN4Ogre16TextureUnitState7setNameERKSs
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, const std::string *)
#[doc(alias = "Ogre::TextureUnitState::setName(std::string const&)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState7setNameERKSs")]
// was: Ogre::TextureUnitState::setName(std::string const&)
// IDA 0xe4d2f8: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4d2f8() {
}

// 0xe4d324 — __ZN4Ogre16TextureUnitState19setTextureNameAliasERKSs
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, const std::string *)
#[doc(alias = "Ogre::TextureUnitState::setTextureNameAlias(std::string const&)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState19setTextureNameAliasERKSs")]
// was: Ogre::TextureUnitState::setTextureNameAlias(std::string const&)
// IDA 0xe4d324: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4d324() {
}

// 0xe4d334 — __ZN4Ogre16TextureUnitState19applyTextureAliasesERKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIKSsSsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEb
#[doc(alias = "Ogre::TextureUnitState::applyTextureAliases(std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&,bool)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState19applyTextureAliasesERKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIKSsSsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEb")]
// was: Ogre::TextureUnitState::applyTextureAliases(std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&,bool)
// IDA 0xe4d334: 49 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4d334() {
}

// 0xe4d3b4 — __ZN4Ogre16TextureUnitState13_notifyParentEPNS_4PassE
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, Ogre::Pass *)
#[doc(alias = "Ogre::TextureUnitState::_notifyParent(Ogre::Pass *)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState13_notifyParentEPNS_4PassE")]
// was: Ogre::TextureUnitState::_notifyParent(Ogre::Pass *)
// IDA 0xe4d3b4: 2 insns (STR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4d3b4() {
}

// 0xe4d3bc — __ZN4Ogre16TextureUnitState22setCompositorReferenceERKSsS2_m
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, const std::string *, const std::string *, unsigned int)
#[doc(alias = "Ogre::TextureUnitState::setCompositorReference(std::string const&,std::string const&,unsigned long)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState22setCompositorReferenceERKSsS2_m")]
// was: Ogre::TextureUnitState::setCompositorReference(std::string const&,std::string const&,unsigned long)
// IDA 0xe4d3bc: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4d3bc() {
}

// 0xe4d3e0 — __ZNSt6vectorIN4Ogre10TexturePtrENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS7_
// type: int __fastcall(int, Ogre::NedPoolingImpl *)
#[doc(alias = "std::vector<Ogre::TexturePtr,Ogre::STLAllocator<Ogre::TexturePtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<Ogre::TexturePtr,Ogre::STLAllocator<Ogre::TexturePtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
#[doc(alias = "__ZNSt6vectorIN4Ogre10TexturePtrENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS7_")]
// was: std::vector<Ogre::TexturePtr,Ogre::STLAllocator<Ogre::TexturePtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<Ogre::TexturePtr,Ogre::STLAllocator<Ogre::TexturePtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
// IDA 0xe4d3e0: 185 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4d3e0() {
}

// 0xe4d5d0 — __ZNSt8_Rb_treeIN4Ogre16TextureUnitState17TextureEffectTypeESt4pairIKS2_NS1_13TextureEffectEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS6_ESI_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<Ogre::TextureUnitState::TextureEffectType,std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>,std::_Select1st<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>>,std::less<Ogre::TextureUnitState::TextureEffectType>,Ogre::STLAllocator<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>>,std::_Rb_tree_iterator<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>>)")]
#[doc(alias = "__ZNSt8_Rb_treeIN4Ogre16TextureUnitState17TextureEffectTypeESt4pairIKS2_NS1_13TextureEffectEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS6_ESI_")]
// was: std::_Rb_tree<Ogre::TextureUnitState::TextureEffectType,std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>,std::_Select1st<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>>,std::less<Ogre::TextureUnitState::TextureEffectType>,Ogre::STLAllocator<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>>,std::_Rb_tree_iterator<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>>)
// IDA 0xe4d5d0: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4d5d0() {
}

// 0xe4d634 — __ZNSt8_Rb_treeIN4Ogre16TextureUnitState17TextureEffectTypeESt4pairIKS2_NS1_13TextureEffectEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
// type: int(void)
#[doc(alias = "std::_Rb_tree<Ogre::TextureUnitState::TextureEffectType,std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>,std::_Select1st<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>>,std::less<Ogre::TextureUnitState::TextureEffectType>,Ogre::STLAllocator<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIN4Ogre16TextureUnitState17TextureEffectTypeESt4pairIKS2_NS1_13TextureEffectEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E")]
// was: std::_Rb_tree<Ogre::TextureUnitState::TextureEffectType,std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>,std::_Select1st<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>>,std::less<Ogre::TextureUnitState::TextureEffectType>,Ogre::STLAllocator<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>> *)
// IDA 0xe4d634: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4d634() {
}

// 0xe4d65c — __ZNSt6vectorIN4Ogre10TexturePtrENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S7_EEmRKS1_
// type: int(void)
#[doc(alias = "std::vector<Ogre::TexturePtr,Ogre::STLAllocator<Ogre::TexturePtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::TexturePtr*,std::vector<Ogre::TexturePtr,Ogre::STLAllocator<Ogre::TexturePtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::TexturePtr const&)")]
#[doc(alias = "__ZNSt6vectorIN4Ogre10TexturePtrENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S7_EEmRKS1_")]
// was: std::vector<Ogre::TexturePtr,Ogre::STLAllocator<Ogre::TexturePtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::TexturePtr*,std::vector<Ogre::TexturePtr,Ogre::STLAllocator<Ogre::TexturePtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::TexturePtr const&)
// IDA 0xe4d65c: 498 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4d65c() {
}

// 0xe4db98 — __ZNSt6vectorISsN4Ogre12STLAllocatorISsNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPSsS6_EEmRKSs
// type: int __fastcall(int, int)
#[doc(alias = "std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<std::string *,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,std::string const&)")]
#[doc(alias = "__ZNSt6vectorISsN4Ogre12STLAllocatorISsNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPSsS6_EEmRKSs")]
// was: std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<std::string *,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,std::string const&)
// IDA 0xe4db98: 310 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4db98() {
}

// 0xe4dfc4 — __ZSt24__uninitialized_fill_n_aIPSsmSsN4Ogre12STLAllocatorISsNS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEEvT_T0_RKT1_T2_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "void std::__uninitialized_fill_n_a<std::string *,unsigned long,std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(std::string *,unsigned long,std::string const&,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")]
#[doc(alias = "__ZSt24__uninitialized_fill_n_aIPSsmSsN4Ogre12STLAllocatorISsNS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEEvT_T0_RKT1_T2_")]
// was: void std::__uninitialized_fill_n_a<std::string *,unsigned long,std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(std::string *,unsigned long,std::string const&,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)
// IDA 0xe4dfc4: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4dfc4() {
}

// 0xe4e0fc — __ZNSt8_Rb_treeIN4Ogre16TextureUnitState17TextureEffectTypeESt4pairIKS2_NS1_13TextureEffectEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE7_M_copyEPKSt13_Rb_tree_nodeIS6_EPSI_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<Ogre::TextureUnitState::TextureEffectType,std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>,std::_Select1st<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>>,std::less<Ogre::TextureUnitState::TextureEffectType>,Ogre::STLAllocator<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_copy(std::_Rb_tree_node<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>> const*,std::_Rb_tree_node<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>>*)")]
#[doc(alias = "__ZNSt8_Rb_treeIN4Ogre16TextureUnitState17TextureEffectTypeESt4pairIKS2_NS1_13TextureEffectEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE7_M_copyEPKSt13_Rb_tree_nodeIS6_EPSI_")]
// was: std::_Rb_tree<Ogre::TextureUnitState::TextureEffectType,std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>,std::_Select1st<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>>,std::less<Ogre::TextureUnitState::TextureEffectType>,Ogre::STLAllocator<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_copy(std::_Rb_tree_node<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>> const*,std::_Rb_tree_node<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>>*)
// IDA 0xe4e0fc: 129 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4e0fc() {
}

// 0xe4e2b4 — __ZNSt8_Rb_treeIN4Ogre16TextureUnitState17TextureEffectTypeESt4pairIKS2_NS1_13TextureEffectEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<Ogre::TextureUnitState::TextureEffectType,std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>,std::_Select1st<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>>,std::less<Ogre::TextureUnitState::TextureEffectType>,Ogre::STLAllocator<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::TextureUnitState::TextureEffectType>,false>::~_Rb_tree_impl()")]
#[doc(alias = "__ZNSt8_Rb_treeIN4Ogre16TextureUnitState17TextureEffectTypeESt4pairIKS2_NS1_13TextureEffectEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED1Ev")]
// was: std::_Rb_tree<Ogre::TextureUnitState::TextureEffectType,std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>,std::_Select1st<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>>,std::less<Ogre::TextureUnitState::TextureEffectType>,Ogre::STLAllocator<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::TextureUnitState::TextureEffectType>,false>::~_Rb_tree_impl()
// IDA 0xe4e2b4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xe4e2b4() {
}

// 0xe4e2b8 — __ZNSt8_Rb_treeIN4Ogre16TextureUnitState17TextureEffectTypeESt4pairIKS2_NS1_13TextureEffectEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<Ogre::TextureUnitState::TextureEffectType,std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>,std::_Select1st<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>>,std::less<Ogre::TextureUnitState::TextureEffectType>,Ogre::STLAllocator<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::TextureUnitState::TextureEffectType>,false>::~_Rb_tree_impl()")]
#[doc(alias = "__ZNSt8_Rb_treeIN4Ogre16TextureUnitState17TextureEffectTypeESt4pairIKS2_NS1_13TextureEffectEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED0Ev")]
// was: std::_Rb_tree<Ogre::TextureUnitState::TextureEffectType,std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>,std::_Select1st<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>>,std::less<Ogre::TextureUnitState::TextureEffectType>,Ogre::STLAllocator<std::pair<Ogre::TextureUnitState::TextureEffectType const,Ogre::TextureUnitState::TextureEffect>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::TextureUnitState::TextureEffectType>,false>::~_Rb_tree_impl()
// IDA 0xe4e2b8: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4e2b8() {
}

// 0xe4e2f8 — __ZN4Ogre26UnifiedHighLevelGpuProgramC2EPNS_15ResourceManagerERKSsyS4_bPNS_20ManualResourceLoaderE
// type: _DWORD __fastcall(Ogre::UnifiedHighLevelGpuProgram *__hidden this, Ogre::ResourceManager *, const std::string *, unsigned __int64, const std::string *, bool, Ogre::ManualResourceLoader *)
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgram::UnifiedHighLevelGpuProgram(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *)")]
#[doc(alias = "__ZN4Ogre26UnifiedHighLevelGpuProgramC2EPNS_15ResourceManagerERKSsyS4_bPNS_20ManualResourceLoaderE")]
// was: Ogre::UnifiedHighLevelGpuProgram::UnifiedHighLevelGpuProgram(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *)
// IDA 0xe4e2f8: 477 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4e2f8() {
}

// 0xe4e84c — __ZN4Ogre26UnifiedHighLevelGpuProgramD0Ev
// type: void __fastcall(Ogre::UnifiedHighLevelGpuProgram *__hidden this)
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgram::~UnifiedHighLevelGpuProgram()")]
#[doc(alias = "__ZN4Ogre26UnifiedHighLevelGpuProgramD0Ev")]
// was: Ogre::UnifiedHighLevelGpuProgram::~UnifiedHighLevelGpuProgram()
// IDA 0xe4e84c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe4e84c() {
}

// 0xe4e8dc — __ZN4Ogre26UnifiedHighLevelGpuProgramD1Ev
// type: void __fastcall(Ogre::UnifiedHighLevelGpuProgram *__hidden this)
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgram::~UnifiedHighLevelGpuProgram()")]
#[doc(alias = "__ZN4Ogre26UnifiedHighLevelGpuProgramD1Ev")]
// was: Ogre::UnifiedHighLevelGpuProgram::~UnifiedHighLevelGpuProgram()
// IDA 0xe4e8dc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe4e8dc() {
}

// 0xe4e8e8 — __ZN4Ogre26UnifiedHighLevelGpuProgramD2Ev
// type: void __fastcall(Ogre::UnifiedHighLevelGpuProgram *__hidden this)
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgram::~UnifiedHighLevelGpuProgram()")]
#[doc(alias = "__ZN4Ogre26UnifiedHighLevelGpuProgramD2Ev")]
// was: Ogre::UnifiedHighLevelGpuProgram::~UnifiedHighLevelGpuProgram()
// IDA 0xe4e8e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe4e8e8() {
}

// 0xe4ea78 — __ZNK4Ogre26UnifiedHighLevelGpuProgram14chooseDelegateEv
// type: _DWORD __fastcall(Ogre::UnifiedHighLevelGpuProgram *__hidden this)
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgram::chooseDelegate(void)const")]
#[doc(alias = "__ZNK4Ogre26UnifiedHighLevelGpuProgram14chooseDelegateEv")]
// was: Ogre::UnifiedHighLevelGpuProgram::chooseDelegate(void)const
// IDA 0xe4ea78: 806 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4ea78() {
}

// 0xe4f338 — __ZN4Ogre26UnifiedHighLevelGpuProgram18addDelegateProgramERKSs
// type: _DWORD __fastcall(Ogre::UnifiedHighLevelGpuProgram *__hidden this, const std::string *)
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgram::addDelegateProgram(std::string const&)")]
#[doc(alias = "__ZN4Ogre26UnifiedHighLevelGpuProgram18addDelegateProgramERKSs")]
// was: Ogre::UnifiedHighLevelGpuProgram::addDelegateProgram(std::string const&)
// IDA 0xe4f338: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4f338() {
}

// 0xe4f3a0 — __ZNK4Ogre26UnifiedHighLevelGpuProgram11getLanguageEv
// type: _DWORD __fastcall(Ogre::UnifiedHighLevelGpuProgram *__hidden this)
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgram::getLanguage(void)const")]
#[doc(alias = "__ZNK4Ogre26UnifiedHighLevelGpuProgram11getLanguageEv")]
// was: Ogre::UnifiedHighLevelGpuProgram::getLanguage(void)const
// IDA 0xe4f3a0: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4f3a0() {
}

// 0xe4f3b0 — __ZN4Ogre26UnifiedHighLevelGpuProgram16createParametersEv
// type: _DWORD __fastcall(Ogre::UnifiedHighLevelGpuProgram *__hidden this)
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgram::createParameters(void)")]
#[doc(alias = "__ZN4Ogre26UnifiedHighLevelGpuProgram16createParametersEv")]
// was: Ogre::UnifiedHighLevelGpuProgram::createParameters(void)
// IDA 0xe4f3b0: 100 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4f3b0() {
}

// 0xe4f4c8 — __ZN4Ogre26UnifiedHighLevelGpuProgram19_getBindingDelegateEv
// type: _DWORD __fastcall(Ogre::UnifiedHighLevelGpuProgram *__hidden this)
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgram::_getBindingDelegate(void)")]
#[doc(alias = "__ZN4Ogre26UnifiedHighLevelGpuProgram19_getBindingDelegateEv")]
// was: Ogre::UnifiedHighLevelGpuProgram::_getBindingDelegate(void)
// IDA 0xe4f4c8: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4f4c8() {
}

// 0xe4f4f0 — __ZNK4Ogre26UnifiedHighLevelGpuProgram11isSupportedEv
// type: _DWORD __fastcall(Ogre::UnifiedHighLevelGpuProgram *__hidden this)
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgram::isSupported(void)const")]
#[doc(alias = "__ZNK4Ogre26UnifiedHighLevelGpuProgram11isSupportedEv")]
// was: Ogre::UnifiedHighLevelGpuProgram::isSupported(void)const
// IDA 0xe4f4f0: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4f4f0() {
}

// 0xe4f510 — __ZNK4Ogre26UnifiedHighLevelGpuProgram27isSkeletalAnimationIncludedEv
// type: _DWORD __fastcall(Ogre::UnifiedHighLevelGpuProgram *__hidden this)
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgram::isSkeletalAnimationIncluded(void)const")]
#[doc(alias = "__ZNK4Ogre26UnifiedHighLevelGpuProgram27isSkeletalAnimationIncludedEv")]
// was: Ogre::UnifiedHighLevelGpuProgram::isSkeletalAnimationIncluded(void)const
// IDA 0xe4f510: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4f510() {
}

// 0xe4f538 — __ZNK4Ogre26UnifiedHighLevelGpuProgram24isMorphAnimationIncludedEv
// type: _DWORD __fastcall(Ogre::UnifiedHighLevelGpuProgram *__hidden this)
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgram::isMorphAnimationIncluded(void)const")]
#[doc(alias = "__ZNK4Ogre26UnifiedHighLevelGpuProgram24isMorphAnimationIncludedEv")]
// was: Ogre::UnifiedHighLevelGpuProgram::isMorphAnimationIncluded(void)const
// IDA 0xe4f538: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4f538() {
}

// 0xe4f560 — __ZNK4Ogre26UnifiedHighLevelGpuProgram23isPoseAnimationIncludedEv
// type: _DWORD __fastcall(Ogre::UnifiedHighLevelGpuProgram *__hidden this)
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgram::isPoseAnimationIncluded(void)const")]
#[doc(alias = "__ZNK4Ogre26UnifiedHighLevelGpuProgram23isPoseAnimationIncludedEv")]
// was: Ogre::UnifiedHighLevelGpuProgram::isPoseAnimationIncluded(void)const
// IDA 0xe4f560: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4f560() {
}

// 0xe4f588 — __ZNK4Ogre26UnifiedHighLevelGpuProgram28isVertexTextureFetchRequiredEv
// type: _DWORD __fastcall(Ogre::UnifiedHighLevelGpuProgram *__hidden this)
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgram::isVertexTextureFetchRequired(void)const")]
#[doc(alias = "__ZNK4Ogre26UnifiedHighLevelGpuProgram28isVertexTextureFetchRequiredEv")]
// was: Ogre::UnifiedHighLevelGpuProgram::isVertexTextureFetchRequired(void)const
// IDA 0xe4f588: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4f588() {
}

// 0xe4f5b0 — __ZN4Ogre26UnifiedHighLevelGpuProgram20getDefaultParametersEv
// type: _DWORD __fastcall(Ogre::UnifiedHighLevelGpuProgram *__hidden this)
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgram::getDefaultParameters(void)")]
#[doc(alias = "__ZN4Ogre26UnifiedHighLevelGpuProgram20getDefaultParametersEv")]
// was: Ogre::UnifiedHighLevelGpuProgram::getDefaultParameters(void)
// IDA 0xe4f5b0: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4f5b0() {
}

// 0xe4f5f4 — __ZNK4Ogre26UnifiedHighLevelGpuProgram20hasDefaultParametersEv
// type: _DWORD __fastcall(Ogre::UnifiedHighLevelGpuProgram *__hidden this)
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgram::hasDefaultParameters(void)const")]
#[doc(alias = "__ZNK4Ogre26UnifiedHighLevelGpuProgram20hasDefaultParametersEv")]
// was: Ogre::UnifiedHighLevelGpuProgram::hasDefaultParameters(void)const
// IDA 0xe4f5f4: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4f5f4() {
}

// 0xe4f61c — __ZNK4Ogre26UnifiedHighLevelGpuProgram28getPassSurfaceAndLightStatesEv
// type: _DWORD __fastcall(Ogre::UnifiedHighLevelGpuProgram *__hidden this)
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgram::getPassSurfaceAndLightStates(void)const")]
#[doc(alias = "__ZNK4Ogre26UnifiedHighLevelGpuProgram28getPassSurfaceAndLightStatesEv")]
// was: Ogre::UnifiedHighLevelGpuProgram::getPassSurfaceAndLightStates(void)const
// IDA 0xe4f61c: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4f61c() {
}

// 0xe4f644 — __ZNK4Ogre26UnifiedHighLevelGpuProgram16getPassFogStatesEv
// type: _DWORD __fastcall(Ogre::UnifiedHighLevelGpuProgram *__hidden this)
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgram::getPassFogStates(void)const")]
#[doc(alias = "__ZNK4Ogre26UnifiedHighLevelGpuProgram16getPassFogStatesEv")]
// was: Ogre::UnifiedHighLevelGpuProgram::getPassFogStates(void)const
// IDA 0xe4f644: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4f644() {
}

// 0xe4f66c — __ZNK4Ogre26UnifiedHighLevelGpuProgram22getPassTransformStatesEv
// type: _DWORD __fastcall(Ogre::UnifiedHighLevelGpuProgram *__hidden this)
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgram::getPassTransformStates(void)const")]
#[doc(alias = "__ZNK4Ogre26UnifiedHighLevelGpuProgram22getPassTransformStatesEv")]
// was: Ogre::UnifiedHighLevelGpuProgram::getPassTransformStates(void)const
// IDA 0xe4f66c: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4f66c() {
}

// 0xe4f694 — __ZNK4Ogre26UnifiedHighLevelGpuProgram15hasCompileErrorEv
// type: _DWORD __fastcall(Ogre::UnifiedHighLevelGpuProgram *__hidden this)
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgram::hasCompileError(void)const")]
#[doc(alias = "__ZNK4Ogre26UnifiedHighLevelGpuProgram15hasCompileErrorEv")]
// was: Ogre::UnifiedHighLevelGpuProgram::hasCompileError(void)const
// IDA 0xe4f694: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4f694() {
}

// 0xe4f6bc — __ZN4Ogre26UnifiedHighLevelGpuProgram17resetCompileErrorEv
// type: _DWORD __fastcall(Ogre::UnifiedHighLevelGpuProgram *__hidden this)
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgram::resetCompileError(void)")]
#[doc(alias = "__ZN4Ogre26UnifiedHighLevelGpuProgram17resetCompileErrorEv")]
// was: Ogre::UnifiedHighLevelGpuProgram::resetCompileError(void)
// IDA 0xe4f6bc: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4f6bc() {
}

// 0xe4f6e4 — __ZN4Ogre26UnifiedHighLevelGpuProgram4loadEb
// type: int __fastcall(Ogre::UnifiedHighLevelGpuProgram *this, int)
#[doc(alias = "Ogre::UnifiedHighLevelGpuProgram::load(bool)")]
#[doc(alias = "__ZN4Ogre26UnifiedHighLevelGpuProgram4loadEb")]
// was: Ogre::UnifiedHighLevelGpuProgram::load(bool)
// IDA 0xe4f6e4: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4f6e4() {
}

#[cfg(test)]
mod texture_unit_state_tests {
    use super::*;

    #[test]
    fn cubic_faces_split_extension_in_binary_order() {
        assert_eq!(
            TextureUnitState::cubic_face_names("brick.png"),
            [
                "brick_fr.png",
                "brick_bk.png",
                "brick_lf.png",
                "brick_rt.png",
                "brick_up.png",
                "brick_dn.png"
            ]
            .map(String::from)
        );
    }

    #[test]
    fn animated_names_are_zero_based_stem_index_ext() {
        assert_eq!(
            TextureUnitState::animated_frame_names("flame.tga", 3),
            vec!["flame_0.tga", "flame_1.tga", "flame_2.tga"]
        );
    }

    #[test]
    fn shadow_content_clears_frames_but_named_keeps() {
        let mut named = TextureUnitState::default();
        stub_0xe4acb8(&mut named, "flame.tga", 2, 1.5);
        assert_eq!(stub_0xe4bb9c(&named), 2);
        assert_eq!(stub_0xe49b54(&named), "flame_0.tga");
        stub_0xe49b7c(&mut named, ContentType::Shadow);
        assert_eq!(stub_0xe4bb9c(&named), 0);
        assert_eq!(stub_0xe49b54(&named), "");
    }

    #[test]
    fn cubic_uvw_selects_single_3d_slot() {
        let mut state = TextureUnitState::default();
        stub_0xe49dec(&mut state, "env.png", true);
        assert_eq!(stub_0xe4bb9c(&state), 1);
        assert!(stub_0xe4aca8(&state));
        stub_0xe49dec(&mut state, "env.png", false);
        assert_eq!(stub_0xe4bb9c(&state), 6);
        assert!(!stub_0xe4aca8(&state));
    }

    #[test]
    #[should_panic(expected = "frameNumber parameter value exceeds")]
    fn current_frame_out_of_range_panics_like_ogre() {
        let mut state = TextureUnitState::default();
        stub_0xe4acb8(&mut state, "flame.tga", 2, 1.0);
        stub_0xe4b98c(&mut state, 0);
        assert_eq!(stub_0xe4bb98(&state), 0);
        stub_0xe4b98c(&mut state, 2);
    }

    #[test]
    fn texture_ptr_lazy_loads_named_frames() {
        let mut state = TextureUnitState::default();
        stub_0xe4acb8(&mut state, "flame.tga", 2, 0.0);
        assert!(stub_0xe4b8f0(&mut state, 1).is_some());
        assert!(stub_0xe4b8f0(&mut state, 7).is_none());
    }

    #[test]
    fn alpha_operation_stores_op_sources_and_args() {
        let mut state = TextureUnitState::default();
        stub_0xe4bc04(&mut state, 3, 1, 2, 0.25, 0.75, 0.5);
        let mode = stub_0xe4be04(&state);
        assert_eq!(mode.operation, 3);
        assert_eq!(mode.source1, 1);
        assert_eq!(mode.source2, 2);
        assert_eq!(mode.alpha_arg1, 0.25);
        assert_eq!(mode.alpha_arg2, 0.75);
        assert_eq!(mode.factor, 0.5);
    }

    #[test]
    fn colour_blend_mode_and_fallback_round_trip() {
        let mut state = TextureUnitState::default();
        stub_0xe4bbc4(
            &mut state,
            7,
            1,
            2,
            ColourValue { r: 1.0, g: 0.0, b: 0.0, a: 1.0 },
            ColourValue { r: 0.0, g: 1.0, b: 0.0, a: 1.0 },
            0.5,
        );
        assert_eq!(stub_0xe4be00(&state).operation, 7);
        assert_eq!(stub_0xe4be00(&state).colour_arg1.r, 1.0);
        stub_0xe4bbf8(
            &mut state,
            SceneBlendFactor::SourceAlpha,
            SceneBlendFactor::OneMinusSourceAlpha,
        );
        assert_eq!(stub_0xe4bdf8(&state), SceneBlendFactor::SourceAlpha);
        assert_eq!(stub_0xe4bdfc(&state), SceneBlendFactor::OneMinusSourceAlpha);
    }

    #[test]
    fn addressing_and_border_colour_round_trip() {
        let mut state = TextureUnitState::default();
        let mode = UvwAddressingMode { u: 1, v: 2, w: 3 };
        stub_0xe4be0c(&mut state, &mode);
        assert_eq!(*stub_0xe4be08(&state), mode);
        let colour = ColourValue { r: 0.1, g: 0.2, b: 0.3, a: 0.4 };
        stub_0xe4be1c(&mut state, &colour);
        assert_eq!(*stub_0xe4be28(&state), colour);
    }

    #[test]
    fn add_effect_replaces_same_type_and_allocates_when_loaded() {
        let mut state = TextureUnitState::default();
        state.parent_loaded = true;
        stub_0xe4c20c(&mut state, 1.5, 1.5);
        assert_eq!(state.effects.len(), 1);
        assert_eq!(state.effects[0].effect_type, effect_type::UV_SCROLL);
        assert!(state.effects[0].controller.is_some());
        // Equal-type re-add destroys the old controller and keeps one entry.
        let first = state.effects[0].controller;
        stub_0xe4c20c(&mut state, 2.0, 2.0);
        assert_eq!(state.effects.len(), 1);
        assert_eq!(state.effects[0].arg1, 2.0);
        assert_ne!(state.effects[0].controller, first);
        // Split speeds produce U + V entries.
        stub_0xe4c20c(&mut state, 1.0, 2.0);
        assert_eq!(state.effects.len(), 2);
        // Zero uSpeed adds nothing (binary gate at 0xe4c242).
        stub_0xe4c20c(&mut state, 0.0, 2.0);
        assert!(state.effects.is_empty());
    }

    #[test]
    fn rotate_and_envmap_effects() {
        let mut state = TextureUnitState::default();
        state.parent_loaded = true;
        stub_0xe4c2a0(&mut state, 0.5);
        assert_eq!(state.effects.len(), 1);
        assert!(state.effects[0].controller.is_some());
        stub_0xe4c2a0(&mut state, 0.0);
        assert!(state.effects.is_empty());
        stub_0xe4be2c(&mut state, true, 2);
        assert_eq!(state.effects.len(), 1);
        assert_eq!(state.effects[0].effect_type, effect_type::ENVIRONMENT_MAP);
        assert_eq!(state.effects[0].subtype, 2);
        // Envmap takes the TBB default arm: no controller.
        assert!(state.effects[0].controller.is_none());
        stub_0xe4be50(&mut state, effect_type::ENVIRONMENT_MAP);
        assert!(state.effects.is_empty());
    }

    #[test]
    fn transform_animation_gates_and_subtypes() {
        let mut state = TextureUnitState::default();
        // Frequency alone does not trigger creation (gate at 0xe4c362).
        stub_0xe4c2d4(&mut state, 1, 0, 0.0, 2.0, 0.0, 0.0);
        assert!(state.effects.is_empty());
        stub_0xe4c2d4(&mut state, 1, 3, 0.5, 2.0, 0.25, 1.0);
        assert_eq!(state.effects.len(), 1);
        assert_eq!(state.effects[0].wave_type, 3);
        assert_eq!(state.effects[0].frequency, 2.0);
        // Same subtype replaces; other subtypes coexist.
        stub_0xe4c2d4(&mut state, 1, 3, 0.75, 2.0, 0.25, 1.0);
        assert_eq!(state.effects.len(), 1);
        assert_eq!(state.effects[0].base, 0.75);
        stub_0xe4c2d4(&mut state, 2, 3, 0.75, 2.0, 0.25, 1.0);
        assert_eq!(state.effects.len(), 2);
    }

    #[test]
    fn scroll_scale_rotate_set_latch_and_recalc() {
        let mut state = TextureUnitState::default();
        stub_0xe4bf0c(&mut state, 0.25, 0.5);
        stub_0xe4bf1c(&mut state, 2.0, 4.0);
        stub_0xe4bf2c(&mut state, 0.0);
        assert!(state.recalc_tex_matrix);
        stub_0xe4bf58(&mut state);
        assert!(!state.recalc_tex_matrix);
        let m = stub_0xe4bf3c(&mut state);
        assert_eq!(m.m[0][0], 0.5);
        assert_eq!(m.m[1][1], 0.25);
        // Scroll composes as translate * scale (IDA 0xe4c076):
        // m03 = 0.25 + 0.25, m13 = 0.375 + 0.5.
        assert_eq!(m.m[0][3], 0.5);
        assert_eq!(m.m[1][3], 0.875);
        // U/V leaves touch one lane each.
        stub_0xe4c1dc(&mut state, 0.0);
        stub_0xe4c1e8(&mut state, 0.0);
        stub_0xe4c1f4(&mut state, 1.0);
        stub_0xe4c200(&mut state, 1.0);
        stub_0xe4bf58(&mut state);
        assert_eq!(stub_0xe4bf3c(&mut state).m, Matrix4::IDENTITY.m);
    }

    #[test]
    fn rotate_90_degrees_about_centre() {
        let mut state = TextureUnitState::default();
        stub_0xe4bf2c(&mut state, std::f32::consts::FRAC_PI_2);
        let m = stub_0xe4bf3c(&mut state).m;
        assert!((m[0][0] - 0.0).abs() < 1e-6);
        assert!((m[0][1] + 1.0).abs() < 1e-6);
        assert!((m[1][0] - 1.0).abs() < 1e-6);
        assert!((m[1][1] - 0.0).abs() < 1e-6);
        assert!((m[0][3] - 1.0).abs() < 1e-6);
        assert!((m[1][3] - 0.0).abs() < 1e-6);
    }

    #[test]
    fn set_transform_skips_recalc_and_prepare_latches() {
        let mut state = TextureUnitState::default();
        let mut custom = Matrix4::IDENTITY;
        custom.m[0][3] = 2.0;
        stub_0xe4bec8(&mut state, &custom);
        assert!(!state.recalc_tex_matrix);
        assert_eq!(stub_0xe4bf3c(&mut state).m[0][3], 2.0);
        stub_0xe4acb8(&mut state, "flame.tga", 2, 0.0);
        stub_0xe4c390(&mut state);
        assert!(state.textures.iter().all(|s| s.prepared));
    }
}
