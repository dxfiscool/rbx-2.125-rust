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
#[doc(alias = "Ogre::TextureUnitState::TextureEffect")]
#[derive(Clone, Debug, Default)]
pub struct TextureEffect {
    pub effect_type: u32,
    pub arg1: f32,
    pub arg2: f32,
    /// ControllerManager handle from `createEffectController`; None = destroyed.
    pub controller: Option<u32>,
}

/// was: `Ogre::TexturePtr` (`boost::shared_ptr<Ogre::Texture>`, 16 bytes each in `mTextures`).
/// Only the loaded flag is modelled; the GPU resource itself is opaque.
/// `boost::shared_ptr` maps to `rbx_core::SharedPtr` per AGENTS.md §4.
#[derive(Clone, Debug, Default)]
pub struct TextureSlot {
    pub loaded: bool,
}

/// was: `Ogre::TextureUnitState` (OgreMain/src/OgreTextureUnitState.cpp, ogre-v1-6-4).
/// Byte offsets are the IDA `(this + N)` word offsets mapped to bytes.
#[doc(alias = "Ogre::TextureUnitState")]
#[derive(Clone, Debug, Default)]
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
    /// +60/+64/+68 colour blend op + sources (IDA `0xe4bbc8`..`0xe4bbd0`).
    pub colour_op_ex: u32,
    pub colour_src1: u32,
    pub colour_src2: u32,
    /// +72/+88 blend colour/alpha args (IDA `0xe4bbda`/`0xe4bbea`).
    pub colour_arg1: ColourValue,
    pub colour_arg2: ColourValue,
    /// +112 manual blend factor (IDA `0xe4bbee`).
    pub colour_blend_factor: f32,
    /// +116 multipass fallback src/dst (IDA `0xe4bbfc`).
    pub colour_fallback_src: SceneBlendFactor,
    pub colour_fallback_dst: SceneBlendFactor,
    /// +184 load-failed latch, cleared by the name setters (IDA `0xe4a96c`).
    pub load_failed: bool,
    /// +185 texture has alpha (`mIsAlpha`, IDA `0xe4bbb0`).
    pub is_alpha: bool,
    /// +186 hardware gamma (`mHwGammaEnabled`, IDA `0xe4bbb8`).
    pub hw_gamma_enabled: bool,
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
            let h = self.alloc_handle();
            self.effects[i].controller = Some(h);
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
        self.colour_op_ex = op;
        self.colour_src1 = src1;
        self.colour_src2 = src2;
        self.colour_arg1 = arg1;
        self.colour_arg2 = arg2;
        self.colour_blend_factor = factor;
    }

    /// IDA `0xe4bbfc`: store the multipass fallback pair (+116).
    pub fn set_colour_op_multipass_fallback(&mut self, src: SceneBlendFactor, dst: SceneBlendFactor) {
        self.colour_fallback_src = src;
        self.colour_fallback_dst = dst;
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
// IDA 0xe4bc04: 10 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4bc04() {
}

// 0xe4bc2c — __ZN4Ogre16TextureUnitState9addEffectERNS0_13TextureEffectE
#[doc(alias = "Ogre::TextureUnitState::addEffect(Ogre::TextureUnitState::TextureEffect &)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState9addEffectERNS0_13TextureEffectE")]
// was: Ogre::TextureUnitState::addEffect(Ogre::TextureUnitState::TextureEffect &)
// IDA 0xe4bc2c: 113 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4bc2c() {
}

// 0xe4bd68 — __ZN4Ogre16TextureUnitState22createEffectControllerERNS0_13TextureEffectE
#[doc(alias = "Ogre::TextureUnitState::createEffectController(Ogre::TextureUnitState::TextureEffect &)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState22createEffectControllerERNS0_13TextureEffectE")]
// was: Ogre::TextureUnitState::createEffectController(Ogre::TextureUnitState::TextureEffect &)
// IDA 0xe4bd68: 50 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4bd68() {
}

// 0xe4bdf8 — __ZNK4Ogre16TextureUnitState25getColourBlendFallbackSrcEv
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this)
#[doc(alias = "Ogre::TextureUnitState::getColourBlendFallbackSrc(void)const")]
#[doc(alias = "__ZNK4Ogre16TextureUnitState25getColourBlendFallbackSrcEv")]
// was: Ogre::TextureUnitState::getColourBlendFallbackSrc(void)const
// IDA 0xe4bdf8: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4bdf8() {
}

// 0xe4bdfc — __ZNK4Ogre16TextureUnitState26getColourBlendFallbackDestEv
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this)
#[doc(alias = "Ogre::TextureUnitState::getColourBlendFallbackDest(void)const")]
#[doc(alias = "__ZNK4Ogre16TextureUnitState26getColourBlendFallbackDestEv")]
// was: Ogre::TextureUnitState::getColourBlendFallbackDest(void)const
// IDA 0xe4bdfc: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4bdfc() {
}

// 0xe4be00 — __ZNK4Ogre16TextureUnitState18getColourBlendModeEv
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this)
#[doc(alias = "Ogre::TextureUnitState::getColourBlendMode(void)const")]
#[doc(alias = "__ZNK4Ogre16TextureUnitState18getColourBlendModeEv")]
// was: Ogre::TextureUnitState::getColourBlendMode(void)const
// IDA 0xe4be00: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4be00() {
}

// 0xe4be04 — __ZNK4Ogre16TextureUnitState17getAlphaBlendModeEv
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this)
#[doc(alias = "Ogre::TextureUnitState::getAlphaBlendMode(void)const")]
#[doc(alias = "__ZNK4Ogre16TextureUnitState17getAlphaBlendModeEv")]
// was: Ogre::TextureUnitState::getAlphaBlendMode(void)const
// IDA 0xe4be04: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4be04() {
}

// 0xe4be08 — __ZNK4Ogre16TextureUnitState24getTextureAddressingModeEv
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this)
#[doc(alias = "Ogre::TextureUnitState::getTextureAddressingMode(void)const")]
#[doc(alias = "__ZNK4Ogre16TextureUnitState24getTextureAddressingModeEv")]
// was: Ogre::TextureUnitState::getTextureAddressingMode(void)const
// IDA 0xe4be08: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4be08() {
}

// 0xe4be0c — __ZN4Ogre16TextureUnitState24setTextureAddressingModeERKNS0_17UVWAddressingModeE
#[doc(alias = "Ogre::TextureUnitState::setTextureAddressingMode(Ogre::TextureUnitState::UVWAddressingMode const&)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState24setTextureAddressingModeERKNS0_17UVWAddressingModeE")]
// was: Ogre::TextureUnitState::setTextureAddressingMode(Ogre::TextureUnitState::UVWAddressingMode const&)
// IDA 0xe4be0c: 5 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4be0c() {
}

// 0xe4be1c — __ZN4Ogre16TextureUnitState22setTextureBorderColourERKNS_11ColourValueE
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, const Ogre::ColourValue *)
#[doc(alias = "Ogre::TextureUnitState::setTextureBorderColour(Ogre::ColourValue const&)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState22setTextureBorderColourERKNS_11ColourValueE")]
// was: Ogre::TextureUnitState::setTextureBorderColour(Ogre::ColourValue const&)
// IDA 0xe4be1c: 4 insns (VLD1.32..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4be1c() {
}

// 0xe4be28 — __ZNK4Ogre16TextureUnitState22getTextureBorderColourEv
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this)
#[doc(alias = "Ogre::TextureUnitState::getTextureBorderColour(void)const")]
#[doc(alias = "__ZNK4Ogre16TextureUnitState22getTextureBorderColourEv")]
// was: Ogre::TextureUnitState::getTextureBorderColour(void)const
// IDA 0xe4be28: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4be28() {
}

// 0xe4be2c — __ZN4Ogre16TextureUnitState17setEnvironmentMapEbNS0_10EnvMapTypeE
#[doc(alias = "Ogre::TextureUnitState::setEnvironmentMap(bool,Ogre::TextureUnitState::EnvMapType)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState17setEnvironmentMapEbNS0_10EnvMapTypeE")]
// was: Ogre::TextureUnitState::setEnvironmentMap(bool,Ogre::TextureUnitState::EnvMapType)
// IDA 0xe4be2c: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4be2c() {
}

// 0xe4be50 — __ZN4Ogre16TextureUnitState12removeEffectENS0_17TextureEffectTypeE
#[doc(alias = "Ogre::TextureUnitState::removeEffect(Ogre::TextureUnitState::TextureEffectType)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState12removeEffectENS0_17TextureEffectTypeE")]
// was: Ogre::TextureUnitState::removeEffect(Ogre::TextureUnitState::TextureEffectType)
// IDA 0xe4be50: 46 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4be50() {
}

// 0xe4bec8 — __ZN4Ogre16TextureUnitState19setTextureTransformERKNS_7Matrix4E
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, const Ogre::Matrix4 *)
#[doc(alias = "Ogre::TextureUnitState::setTextureTransform(Ogre::Matrix4 const&)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState19setTextureTransformERKNS_7Matrix4E")]
// was: Ogre::TextureUnitState::setTextureTransform(Ogre::Matrix4 const&)
// IDA 0xe4bec8: 18 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4bec8() {
}

// 0xe4bf0c — __ZN4Ogre16TextureUnitState16setTextureScrollEff
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, float, float)
#[doc(alias = "Ogre::TextureUnitState::setTextureScroll(float,float)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState16setTextureScrollEff")]
// was: Ogre::TextureUnitState::setTextureScroll(float,float)
// IDA 0xe4bf0c: 6 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4bf0c() {
}

// 0xe4bf1c — __ZN4Ogre16TextureUnitState15setTextureScaleEff
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, float, float)
#[doc(alias = "Ogre::TextureUnitState::setTextureScale(float,float)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState15setTextureScaleEff")]
// was: Ogre::TextureUnitState::setTextureScale(float,float)
// IDA 0xe4bf1c: 6 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4bf1c() {
}

// 0xe4bf2c — __ZN4Ogre16TextureUnitState16setTextureRotateERKNS_6RadianE
#[doc(alias = "Ogre::TextureUnitState::setTextureRotate(Ogre::Radian const&)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState16setTextureRotateERKNS_6RadianE")]
// was: Ogre::TextureUnitState::setTextureRotate(Ogre::Radian const&)
// IDA 0xe4bf2c: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4bf2c() {
}

// 0xe4bf3c — __ZNK4Ogre16TextureUnitState19getTextureTransformEv
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this)
#[doc(alias = "Ogre::TextureUnitState::getTextureTransform(void)const")]
#[doc(alias = "__ZNK4Ogre16TextureUnitState19getTextureTransformEv")]
// was: Ogre::TextureUnitState::getTextureTransform(void)const
// IDA 0xe4bf3c: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4bf3c() {
}

// 0xe4bf58 — __ZNK4Ogre16TextureUnitState19recalcTextureMatrixEv
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this)
#[doc(alias = "Ogre::TextureUnitState::recalcTextureMatrix(void)const")]
#[doc(alias = "__ZNK4Ogre16TextureUnitState19recalcTextureMatrixEv")]
// was: Ogre::TextureUnitState::recalcTextureMatrix(void)const
// IDA 0xe4bf58: 183 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4bf58() {
}

// 0xe4c1dc — __ZN4Ogre16TextureUnitState17setTextureUScrollEf
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, float)
#[doc(alias = "Ogre::TextureUnitState::setTextureUScroll(float)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState17setTextureUScrollEf")]
// was: Ogre::TextureUnitState::setTextureUScroll(float)
// IDA 0xe4c1dc: 4 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4c1dc() {
}

// 0xe4c1e8 — __ZN4Ogre16TextureUnitState17setTextureVScrollEf
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, float)
#[doc(alias = "Ogre::TextureUnitState::setTextureVScroll(float)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState17setTextureVScrollEf")]
// was: Ogre::TextureUnitState::setTextureVScroll(float)
// IDA 0xe4c1e8: 4 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4c1e8() {
}

// 0xe4c1f4 — __ZN4Ogre16TextureUnitState16setTextureUScaleEf
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, float)
#[doc(alias = "Ogre::TextureUnitState::setTextureUScale(float)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState16setTextureUScaleEf")]
// was: Ogre::TextureUnitState::setTextureUScale(float)
// IDA 0xe4c1f4: 4 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4c1f4() {
}

// 0xe4c200 — __ZN4Ogre16TextureUnitState16setTextureVScaleEf
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, float)
#[doc(alias = "Ogre::TextureUnitState::setTextureVScale(float)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState16setTextureVScaleEf")]
// was: Ogre::TextureUnitState::setTextureVScale(float)
// IDA 0xe4c200: 4 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4c200() {
}

// 0xe4c20c — __ZN4Ogre16TextureUnitState18setScrollAnimationEff
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, float, float)
#[doc(alias = "Ogre::TextureUnitState::setScrollAnimation(float,float)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState18setScrollAnimationEff")]
// was: Ogre::TextureUnitState::setScrollAnimation(float,float)
// IDA 0xe4c20c: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4c20c() {
}

// 0xe4c2a0 — __ZN4Ogre16TextureUnitState18setRotateAnimationEf
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, float)
#[doc(alias = "Ogre::TextureUnitState::setRotateAnimation(float)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState18setRotateAnimationEf")]
// was: Ogre::TextureUnitState::setRotateAnimation(float)
// IDA 0xe4c2a0: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4c2a0() {
}

// 0xe4c2d4 — __ZN4Ogre16TextureUnitState21setTransformAnimationENS0_20TextureTransformTypeENS_12WaveformTypeEffff
// type: int __fastcall(int, int, int, int, float, float, float)
#[doc(alias = "Ogre::TextureUnitState::setTransformAnimation(Ogre::TextureUnitState::TextureTransformType,Ogre::WaveformType,float,float,float,float)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState21setTransformAnimationENS0_20TextureTransformTypeENS_12WaveformTypeEffff")]
// was: Ogre::TextureUnitState::setTransformAnimation(Ogre::TextureUnitState::TextureTransformType,Ogre::WaveformType,float,float,float,float)
// IDA 0xe4c2d4: 63 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4c2d4() {
}

// 0xe4c390 — __ZN4Ogre16TextureUnitState8_prepareEv
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this)
#[doc(alias = "Ogre::TextureUnitState::_prepare(void)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState8_prepareEv")]
// was: Ogre::TextureUnitState::_prepare(void)
// IDA 0xe4c390: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe4c390() {
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
}
