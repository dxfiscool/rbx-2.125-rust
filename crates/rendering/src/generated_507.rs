//! rendering — generated_507 — 100 stubs global dedup (rendering filtered, EA-sorted asc, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo)
//! Source: ida/export.json (85545 funcs) NOT in /tmp/global_eas.txt — 100 uncovered EA-sorted asc 0xe486e8..0xf67bf4 (359 candidates remaining, 94160 global EAs)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr). Sanitized: single quotes removed, boost::shared_ptr -> rbx_core::SharedPtr.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;
use crate::generated_502::{TextureSlot, TextureUnitState};

/// was: `Ogre::TextureManager` (OgreMain/src/OgreTextureManager.cpp, ogre-v1-6-4).
/// Byte offsets are the IDA `(this + N)` accesses: preferred integer depth +152
/// (`0xe486ec`), preferred float depth +154 (`0xe48768`), default mipmaps +156
/// (`0xe487f0`/`0xe48840`), resources-loaded flag +40, resource map +52/+36.
#[doc(alias = "Ogre::TextureManager")]
#[derive(Clone, Debug, Default)]
pub struct TextureManager {
    /// +152 preferred integer bit depth (`mPreferredIntBitDepth`, IDA `0xe486ec`).
    pub preferred_integer_bit_depth: u16,
    /// +154 preferred float bit depth (`mPreferredFloatBitDepth`, IDA `0xe48768`).
    pub preferred_float_bit_depth: u16,
    /// +156 default mipmap count (`mDefaultNumMipmaps`, IDA `0xe487f0`).
    pub default_num_mipmaps: u32,
    /// +40 resources-created latch guarding the reload loops (IDA `0xe486fe`/`0xe48784`).
    pub resources_loaded: bool,
    /// Managed textures visited by the `setPreferred*BitDepth` reload loops.
    pub textures: Vec<ManagedTexture>,
    /// Test hook for the `getNativeFormat` virtual (IDA `0xe48812`/`0xe48822`);
    /// `None` = native format equals the requested format.
    pub native_format_override: Option<u32>,
}

/// was: a `Texture` in the manager map (`0xe4870a`/`0xe4878c` walk), modelled with
/// the two virtual answers the reload loops query: `isLoaded` (+104) and
/// `isReloadable` (+72).
#[derive(Clone, Debug, Default)]
pub struct ManagedTexture {
    /// `isLoaded` answer.
    pub loaded: bool,
    /// `isReloadable` answer.
    pub reloadable: bool,
    /// Per-texture integer depth (`setPreferredBitDepths`, IDA `0xe487da`/`0xe4879e`).
    pub integer_bit_depth: u16,
    /// Per-texture float depth (`setPreferredFloatBitDepth`, IDA `0xe4874a`).
    pub float_bit_depth: u16,
    /// `load()` invocations (vtable +64, IDA `0xe48750`/`0xe487e4`).
    pub load_calls: u32,
}

/// was: `Ogre::PixelFormat` element bit counts from `Ogre::PixelUtil::getNumElemBits`,
/// used by `isEquivalentFormatSupported` (IDA `0xe48828`..`0xe4883a`).
/// Only formats with unambiguous widths are listed; unknown formats count 0 so an
/// identity native format still compares equivalent.
pub fn pixel_elem_bits(format: u32) -> u32 {
    match format {
        1 => 8,   // PF_L8
        2 => 16,  // PF_L16
        3 => 8,   // PF_A8
        4 => 8,   // PF_A4L4
        5 => 16,  // PF_BYTE_LA
        6 => 16,  // PF_R5G6B5
        7 => 16,  // PF_B5G6R5
        8 => 8,   // PF_R3G3B2
        9 => 16,  // PF_A4R4G4B4
        10 => 16, // PF_A1R5G5B5
        11 => 32, // PF_A8R8G8B8
        12 => 24, // PF_R8G8B8
        13 => 32, // PF_X8R8G8B8
        14 => 32, // PF_A2R10G10B10
        15 => 32, // PF_A2B10G10R10
        21 => 16, // PF_FLOAT16_R
        22 => 48, // PF_FLOAT16_RGB
        23 => 64, // PF_FLOAT16_RGBA
        24 => 32, // PF_FLOAT16_GR
        25 => 32, // PF_FLOAT32_R
        26 => 96, // PF_FLOAT32_RGB
        27 => 128, // PF_FLOAT32_RGBA
        28 => 64, // PF_FLOAT32_GR
        29 => 24, // PF_DEPTH
        _ => 0,
    }
}

impl TextureManager {
    /// IDA `0xe486e8`: `return *(u16 *)(this + 152)` (`0xe486ec`).
    pub fn preferred_integer_bit_depth(&self) -> u16 {
        self.preferred_integer_bit_depth
    }

    /// IDA `0xe486f0`: store `a2` at +154 (`0xe486f6`); when `reload` and resources
    /// exist (`0xe486fe`), walk the map (`0xe4870a`..`0xe4871c`): loaded + reloadable
    /// textures unload (+80), take the depth (+320) and load (+64); the rest just
    /// take the depth (`0xe4875a`).
    pub fn set_preferred_float_bit_depth(&mut self, depth: u16, reload: bool) {
        // IDA 0xe486f6: mPreferredFloatBitDepth = a2.
        self.preferred_float_bit_depth = depth;
        if reload && self.resources_loaded {
            for texture in self.textures.iter_mut() {
                if texture.loaded && texture.reloadable {
                    // IDA 0xe4873e: unload; 0xe4874a: setPreferredFloatBitDepth; 0xe48750: load.
                    texture.loaded = false;
                    texture.float_bit_depth = depth;
                    texture.loaded = true;
                    texture.load_calls += 1;
                } else {
                    // IDA 0xe4875a: setPreferredFloatBitDepth only.
                    texture.float_bit_depth = depth;
                }
            }
        }
    }

    /// IDA `0xe48764`: `return *(u16 *)(this + 154)` (`0xe48768`).
    pub fn preferred_float_bit_depth(&self) -> u16 {
        self.preferred_float_bit_depth
    }

    /// IDA `0xe4876c`: store both depths (+152 at `0xe48778`, +154 at `0xe4877e`);
    /// when `reload` and resources exist, walk the map: loaded + reloadable textures
    /// unload (+80), take both depths (+328) and `load(false)` (+64); the rest just
    /// take both depths (`0xe4879e`).
    pub fn set_preferred_bit_depths(&mut self, integer: u16, float: u16, reload: bool) {
        // IDA 0xe48778/0xe4877e.
        self.preferred_integer_bit_depth = integer;
        self.preferred_float_bit_depth = float;
        if reload && self.resources_loaded {
            for texture in self.textures.iter_mut() {
                if texture.loaded && texture.reloadable {
                    // IDA 0xe487cc: unload; 0xe487da: setPreferredBitDepths; 0xe487e4: load(0).
                    texture.loaded = false;
                    texture.integer_bit_depth = integer;
                    texture.float_bit_depth = float;
                    texture.loaded = true;
                    texture.load_calls += 1;
                } else {
                    // IDA 0xe4879e: setPreferredBitDepths only.
                    texture.integer_bit_depth = integer;
                    texture.float_bit_depth = float;
                }
            }
        }
    }

    /// IDA `0xe487f0`: `*(this + 156) = a2` (`0xe487f0`).
    pub fn set_default_num_mipmaps(&mut self, count: u32) {
        self.default_num_mipmaps = count;
    }

    /// IDA `0xe487f8`: `getNativeFormat(type, format, usage) == format` (`0xe48812`).
    pub fn is_format_supported(&self, texture_type: u32, format: u32, usage: i32) -> bool {
        self.native_format(texture_type, format, usage) == format
    }

    /// IDA `0xe48814`: `getNumElemBits(native) >= getNumElemBits(format)`
    /// (`0xe48828`..`0xe4883a`).
    pub fn is_equivalent_format_supported(&self, texture_type: u32, format: u32, usage: i32) -> bool {
        let native = self.native_format(texture_type, format, usage);
        pixel_elem_bits(native) >= pixel_elem_bits(format)
    }

    /// Models the `getNativeFormat` virtual (vtable +196, IDA `0xe48812`/`0xe48822`).
    fn native_format(&self, _texture_type: u32, format: u32, _usage: i32) -> u32 {
        self.native_format_override.unwrap_or(format)
    }

    /// IDA `0xe4883c`: `return *(this + 156)` (`0xe48840`).
    pub fn default_num_mipmaps(&self) -> u32 {
        self.default_num_mipmaps
    }
}

/// was: `Ogre::TextureUnitState::TextureType` cubic dispatch value.
/// IDA `0xe49316` routes `setTextureName` to `setCubicTextureName` when the type
/// is 4; the same value is what `is3D` tests at `0xe4aca8` in this build.
pub const TEXTURE_TYPE_CUBE_MAP: u32 = 4;

impl TextureUnitState {
    /// IDA `0xe492bc`: `setTextureName(name, texType)`.
    /// `mContentType = CONTENT_NAMED` (+300 at `0xe492f2`), clear `mLoadFailed`
    /// (+184 at `0xe492f6`); cube-map type delegates to `setCubicTextureName`
    /// (`0xe49324`); otherwise `mFrames`/`mTextures` shrink to one slot
    /// (`0xe4933e`..`0xe49418`), `mFrames[0] = name` (`0xe494d4`), the texture slot
    /// is nulled (`0xe494dc`..`0xe494fe`), frame/flag/type update (`0xe49504`..`0xe4950a`),
    /// and a non-empty name loads now when the parent pass is loaded plus dirties
    /// the pass hash (`0xe4950e`..`0xe49552`).
    pub fn set_texture_name(&mut self, name: &str, texture_type: u32) {
        // IDA 0xe492f2/0xe492f6.
        self.content_type = 0;
        self.load_failed = false;
        if texture_type == TEXTURE_TYPE_CUBE_MAP {
            // IDA 0xe49316..0xe49324.
            self.set_cubic_texture_name(&[name.to_owned()], true);
            return;
        }
        // IDA 0xe4933e..0xe49418: mFrames.resize(1), mTextures.resize(1).
        self.frames.resize(1, String::new());
        self.textures.resize(1, TextureSlot::default());
        // IDA 0xe494d4: mFrames[0] = name; 0xe494dc..0xe494fe: mTextures[0].setNull().
        self.frames[0] = name.to_owned();
        self.textures[0] = TextureSlot::default();
        // IDA 0xe49504..0xe4950a: mCurrentFrame = 0; flag = 0; mTextureType = a3.
        self.current_frame = 0;
        self.flag_08 = 0;
        self.texture_type = texture_type;
        // IDA 0xe4950e: non-empty name…
        if !name.is_empty() {
            // IDA 0xe49524..0xe4952a: parent loaded → _load().
            if self.parent_loaded {
                self.load();
            }
            // IDA 0xe4953a..0xe49552: builtin hash → Pass::_dirtyHash(mParent).
            self.parent_dirty = true;
        }
    }

    /// IDA `0xe4964c`: `STR R1, [R0, #0x18]` — store the coordinate set index.
    pub fn set_texture_coord_set(&mut self, set: u32) {
        self.texture_coord_set = set;
    }
}

// 0xe486e8 — __ZNK4Ogre14TextureManager27getPreferredIntegerBitDepthEv
// type: _DWORD __fastcall(Ogre::TextureManager *__hidden this)
#[doc(alias = "Ogre::TextureManager::getPreferredIntegerBitDepth(void)const")]
#[doc(alias = "__ZNK4Ogre14TextureManager27getPreferredIntegerBitDepthEv")]
// was: Ogre::TextureManager::getPreferredIntegerBitDepth(void)const
// IDA 0xe486e8: LDRH.W R0, [R0, #152] (0xe486ec) — return mPreferredIntBitDepth.
pub fn stub_0xe486e8(manager: &TextureManager) -> u16 {
    manager.preferred_integer_bit_depth()
}

// 0xe486f0 — __ZN4Ogre14TextureManager25setPreferredFloatBitDepthEtb
// type: _DWORD __fastcall(Ogre::TextureManager *__hidden this, unsigned __int16, bool)
#[doc(alias = "Ogre::TextureManager::setPreferredFloatBitDepth(unsigned short,bool)")]
#[doc(alias = "__ZN4Ogre14TextureManager25setPreferredFloatBitDepthEtb")]
// was: Ogre::TextureManager::setPreferredFloatBitDepth(unsigned short,bool)
// IDA 0xe486f0: store depth at +154, reload loop over textures (0xe486fe..0xe4871c).
pub fn stub_0xe486f0(manager: &mut TextureManager, depth: u16, reload: bool) {
    manager.set_preferred_float_bit_depth(depth, reload)
}
// 0xe48764 — __ZNK4Ogre14TextureManager25getPreferredFloatBitDepthEv
// type: _DWORD __fastcall(Ogre::TextureManager *__hidden this)
#[doc(alias = "Ogre::TextureManager::getPreferredFloatBitDepth(void)const")]
#[doc(alias = "__ZNK4Ogre14TextureManager25getPreferredFloatBitDepthEv")]
// was: Ogre::TextureManager::getPreferredFloatBitDepth(void)const
// IDA 0xe48764: LDRH.W R0, [R0, #154] (0xe48768) — return mPreferredFloatBitDepth.
pub fn stub_0xe48764(manager: &TextureManager) -> u16 {
    manager.preferred_float_bit_depth()
}

// 0xe4876c — __ZN4Ogre14TextureManager21setPreferredBitDepthsEttb
// type: _DWORD __fastcall(Ogre::TextureManager *__hidden this, unsigned __int16, unsigned __int16, bool)
#[doc(alias = "Ogre::TextureManager::setPreferredBitDepths(unsigned short,unsigned short,bool)")]
#[doc(alias = "__ZN4Ogre14TextureManager21setPreferredBitDepthsEttb")]
// IDA 0xe4876c: store both depths (+152/+154), reload loop over textures (0xe48784..0xe487aa).
pub fn stub_0xe4876c(manager: &mut TextureManager, integer: u16, float: u16, reload: bool) {
    manager.set_preferred_bit_depths(integer, float, reload)
}

// 0xe487f0 — __ZN4Ogre14TextureManager20setDefaultNumMipmapsEm
// type: _DWORD __fastcall(Ogre::TextureManager *__hidden this, unsigned int)
#[doc(alias = "Ogre::TextureManager::setDefaultNumMipmaps(unsigned long)")]
#[doc(alias = "__ZN4Ogre14TextureManager20setDefaultNumMipmapsEm")]
// IDA 0xe487f0: STR.W R1, [R0, #156] (0xe487f0) — store mDefaultNumMipmaps.
pub fn stub_0xe487f0(manager: &mut TextureManager, count: u32) {
    manager.set_default_num_mipmaps(count)
}

// 0xe487f8 — __ZN4Ogre14TextureManager17isFormatSupportedENS_11TextureTypeENS_11PixelFormatEi
#[doc(alias = "Ogre::TextureManager::isFormatSupported(Ogre::TextureType,Ogre::PixelFormat,int)")]
#[doc(alias = "__ZN4Ogre14TextureManager17isFormatSupportedENS_11TextureTypeENS_11PixelFormatEi")]
// IDA 0xe487f8: getNativeFormat(type, format, usage) == format (0xe48812).
pub fn stub_0xe487f8(manager: &TextureManager, texture_type: u32, format: u32, usage: i32) -> bool {
    manager.is_format_supported(texture_type, format, usage)
}

// 0xe48814 — __ZN4Ogre14TextureManager27isEquivalentFormatSupportedENS_11TextureTypeENS_11PixelFormatEi
#[doc(alias = "Ogre::TextureManager::isEquivalentFormatSupported(Ogre::TextureType,Ogre::PixelFormat,int)")]
#[doc(alias = "__ZN4Ogre14TextureManager27isEquivalentFormatSupportedENS_11TextureTypeENS_11PixelFormatEi")]
// IDA 0xe48814: getNumElemBits(native) >= getNumElemBits(format) (0xe48828..0xe4883a).
pub fn stub_0xe48814(manager: &TextureManager, texture_type: u32, format: u32, usage: i32) -> bool {
    manager.is_equivalent_format_supported(texture_type, format, usage)
}

// 0xe4883c — __ZN4Ogre14TextureManager20getDefaultNumMipmapsEv
// type: _DWORD __fastcall(Ogre::TextureManager *__hidden this)
#[doc(alias = "Ogre::TextureManager::getDefaultNumMipmaps(void)")]
// IDA 0xe4883c: LDR.W R0, [R0, #156] (0xe48840) — return mDefaultNumMipmaps.
pub fn stub_0xe4883c(manager: &TextureManager) -> u32 {
    manager.default_num_mipmaps()
}

// 0xe48878 — __ZN4Ogre16TextureUnitStateC1EPNS_4PassE
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, Ogre::Pass *)
#[doc(alias = "Ogre::TextureUnitState::TextureUnitState(Ogre::Pass *)")]
#[doc(alias = "__ZN4Ogre16TextureUnitStateC1EPNS_4PassE")]
// was: Ogre::TextureUnitState::TextureUnitState(Ogre::Pass *)
// IDA 0xe48878: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe48878() {
}

// 0xe48884 — __ZN4Ogre16TextureUnitStateC2EPNS_4PassE
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, Ogre::Pass *)
#[doc(alias = "Ogre::TextureUnitState::TextureUnitState(Ogre::Pass *)")]
#[doc(alias = "__ZN4Ogre16TextureUnitStateC2EPNS_4PassE")]
// was: Ogre::TextureUnitState::TextureUnitState(Ogre::Pass *)
// IDA 0xe48884: 425 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe48884() {
}

// 0xe48d44 — __ZN4Ogre16TextureUnitState18setColourOperationENS_19LayerBlendOperationE
// type: int __fastcall(int result, int)
#[doc(alias = "Ogre::TextureUnitState::setColourOperation(Ogre::LayerBlendOperation)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState18setColourOperationENS_19LayerBlendOperationE")]
// was: Ogre::TextureUnitState::setColourOperation(Ogre::LayerBlendOperation)
// IDA 0xe48d44: 83 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe48d44() {
}

// 0xe48e4c — __ZN4Ogre16TextureUnitState24setTextureAddressingModeENS0_21TextureAddressingModeE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "Ogre::TextureUnitState::setTextureAddressingMode(Ogre::TextureUnitState::TextureAddressingMode)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState24setTextureAddressingModeENS0_21TextureAddressingModeE")]
// was: Ogre::TextureUnitState::setTextureAddressingMode(Ogre::TextureUnitState::TextureAddressingMode)
// IDA 0xe48e4c: 4 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe48e4c() {
}

// 0xe48e54 — __ZN4Ogre16TextureUnitStateC1EPNS_4PassERKS0_
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, Ogre::Pass *, const Ogre::TextureUnitState *)
#[doc(alias = "Ogre::TextureUnitState::TextureUnitState(Ogre::Pass *,Ogre::TextureUnitState const&)")]
#[doc(alias = "__ZN4Ogre16TextureUnitStateC1EPNS_4PassERKS0_")]
// was: Ogre::TextureUnitState::TextureUnitState(Ogre::Pass *,Ogre::TextureUnitState const&)
// IDA 0xe48e54: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe48e54() {
}

// 0xe48e60 — __ZN4Ogre16TextureUnitStateC2EPNS_4PassERKS0_
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, Ogre::Pass *, const Ogre::TextureUnitState *)
#[doc(alias = "Ogre::TextureUnitState::TextureUnitState(Ogre::Pass *,Ogre::TextureUnitState const&)")]
#[doc(alias = "__ZN4Ogre16TextureUnitStateC2EPNS_4PassERKS0_")]
// was: Ogre::TextureUnitState::TextureUnitState(Ogre::Pass *,Ogre::TextureUnitState const&)
// IDA 0xe48e60: 297 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe48e60() {
}

// 0xe491a8 — __ZN4Ogre16TextureUnitStateaSERKS0_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "Ogre::TextureUnitState::operator=(Ogre::TextureUnitState const&)")]
#[doc(alias = "__ZN4Ogre16TextureUnitStateaSERKS0_")]
// was: Ogre::TextureUnitState::operator=(Ogre::TextureUnitState const&)
// IDA 0xe491a8: 87 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xe491a8() {
}

// 0xe492bc — __ZN4Ogre16TextureUnitState14setTextureNameERKSsNS_11TextureTypeE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, Ogre::NedPoolingImpl *, int, Ogre::NedPoolingImpl *, int, int, int, int, char, int, int, int, int)
#[doc(alias = "Ogre::TextureUnitState::setTextureName(std::string const&,Ogre::TextureType)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState14setTextureNameERKSsNS_11TextureTypeE")]
// was: Ogre::TextureUnitState::setTextureName(std::string const&,Ogre::TextureType)
// IDA 0xe492bc: named content, single frame slot, parent load + hash-dirty (see set_texture_name).
pub fn stub_0xe492bc(state: &mut TextureUnitState, name: &str, texture_type: u32) {
    state.set_texture_name(name, texture_type)
}

// 0xe4964c — __ZN4Ogre16TextureUnitState18setTextureCoordSetEj
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this, unsigned int)
#[doc(alias = "Ogre::TextureUnitState::setTextureCoordSet(unsigned int)")]
#[doc(alias = "__ZN4Ogre16TextureUnitState18setTextureCoordSetEj")]
// IDA 0xe4964c: STR R1, [R0, #0x18] — store mTextureCoordSetIndex.
pub fn stub_0xe4964c(state: &mut TextureUnitState, set: u32) {
    state.set_texture_coord_set(set)
}

// 0xe49650 — __ZN4Ogre16TextureUnitStateD1Ev
// type: void __fastcall(Ogre::TextureUnitState *__hidden this)
#[doc(alias = "Ogre::TextureUnitState::~TextureUnitState()")]
#[doc(alias = "__ZN4Ogre16TextureUnitStateD1Ev")]
// was: Ogre::TextureUnitState::~TextureUnitState()
// IDA 0xe49650: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe49650() {
}

// 0xe4965c — __ZN4Ogre16TextureUnitStateD2Ev
// type: void __fastcall(Ogre::TextureUnitState *__hidden this)
#[doc(alias = "Ogre::TextureUnitState::~TextureUnitState()")]
#[doc(alias = "__ZN4Ogre16TextureUnitStateD2Ev")]
// was: Ogre::TextureUnitState::~TextureUnitState()
// IDA 0xe4965c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xe4965c() {
}

// 0xf67554 — j___ZNSt8_Rb_treeIPN4Ogre17VertexDeclarationES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_
// type: int __fastcall(char *)
#[doc(alias = "std::_Rb_tree<Ogre::VertexDeclaration *,Ogre::VertexDeclaration *,std::_Identity<Ogre::VertexDeclaration *>,std::less<Ogre::VertexDeclaration *>,Ogre::STLAllocator<Ogre::VertexDeclaration *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::VertexDeclaration * const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre17VertexDeclarationES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_")]
// was: std::_Rb_tree<Ogre::VertexDeclaration *,Ogre::VertexDeclaration *,std::_Identity<Ogre::VertexDeclaration *>,std::less<Ogre::VertexDeclaration *>,Ogre::STLAllocator<Ogre::VertexDeclaration *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::VertexDeclaration * const&)
// IDA 0xf67554: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67554() {
}

// 0xf67564 — j___ZNSt8_Rb_treeIPN4Ogre17VertexDeclarationES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS2_ESE_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<Ogre::VertexDeclaration *,Ogre::VertexDeclaration *,std::_Identity<Ogre::VertexDeclaration *>,std::less<Ogre::VertexDeclaration *>,Ogre::STLAllocator<Ogre::VertexDeclaration *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<Ogre::VertexDeclaration *>,std::_Rb_tree_iterator<Ogre::VertexDeclaration *>)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre17VertexDeclarationES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS2_ESE_")]
// was: std::_Rb_tree<Ogre::VertexDeclaration *,Ogre::VertexDeclaration *,std::_Identity<Ogre::VertexDeclaration *>,std::less<Ogre::VertexDeclaration *>,Ogre::STLAllocator<Ogre::VertexDeclaration *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<Ogre::VertexDeclaration *>,std::_Rb_tree_iterator<Ogre::VertexDeclaration *>)
// IDA 0xf67564: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67564() {
}

// 0xf67574 — j___ZNSt8_Rb_treeIPN4Ogre17VertexDeclarationES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<Ogre::VertexDeclaration *,Ogre::VertexDeclaration *,std::_Identity<Ogre::VertexDeclaration *>,std::less<Ogre::VertexDeclaration *>,Ogre::STLAllocator<Ogre::VertexDeclaration *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::VertexDeclaration *> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre17VertexDeclarationES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")]
// was: std::_Rb_tree<Ogre::VertexDeclaration *,Ogre::VertexDeclaration *,std::_Identity<Ogre::VertexDeclaration *>,std::less<Ogre::VertexDeclaration *>,Ogre::STLAllocator<Ogre::VertexDeclaration *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::VertexDeclaration *> *)
// IDA 0xf67574: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67574() {
}

// 0xf67584 — j___ZNSt8_Rb_treeIPN4Ogre19HardwareIndexBufferES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<Ogre::HardwareIndexBuffer *,Ogre::HardwareIndexBuffer *,std::_Identity<Ogre::HardwareIndexBuffer *>,std::less<Ogre::HardwareIndexBuffer *>,Ogre::STLAllocator<Ogre::HardwareIndexBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::HardwareIndexBuffer *> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre19HardwareIndexBufferES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")]
// was: std::_Rb_tree<Ogre::HardwareIndexBuffer *,Ogre::HardwareIndexBuffer *,std::_Identity<Ogre::HardwareIndexBuffer *>,std::less<Ogre::HardwareIndexBuffer *>,Ogre::STLAllocator<Ogre::HardwareIndexBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::HardwareIndexBuffer *> *)
// IDA 0xf67584: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67584() {
}

// 0xf67594 — j___ZNSt8_Rb_treeIPN4Ogre19VertexBufferBindingES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_
// type: int __fastcall(char *)
#[doc(alias = "std::_Rb_tree<Ogre::VertexBufferBinding *,Ogre::VertexBufferBinding *,std::_Identity<Ogre::VertexBufferBinding *>,std::less<Ogre::VertexBufferBinding *>,Ogre::STLAllocator<Ogre::VertexBufferBinding *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::VertexBufferBinding * const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre19VertexBufferBindingES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_")]
// was: std::_Rb_tree<Ogre::VertexBufferBinding *,Ogre::VertexBufferBinding *,std::_Identity<Ogre::VertexBufferBinding *>,std::less<Ogre::VertexBufferBinding *>,Ogre::STLAllocator<Ogre::VertexBufferBinding *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::VertexBufferBinding * const&)
// IDA 0xf67594: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67594() {
}

// 0xf675a4 — j___ZNSt8_Rb_treeIPN4Ogre19VertexBufferBindingES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS2_ESE_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<Ogre::VertexBufferBinding *,Ogre::VertexBufferBinding *,std::_Identity<Ogre::VertexBufferBinding *>,std::less<Ogre::VertexBufferBinding *>,Ogre::STLAllocator<Ogre::VertexBufferBinding *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<Ogre::VertexBufferBinding *>,std::_Rb_tree_iterator<Ogre::VertexBufferBinding *>)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre19VertexBufferBindingES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS2_ESE_")]
// was: std::_Rb_tree<Ogre::VertexBufferBinding *,Ogre::VertexBufferBinding *,std::_Identity<Ogre::VertexBufferBinding *>,std::less<Ogre::VertexBufferBinding *>,Ogre::STLAllocator<Ogre::VertexBufferBinding *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<Ogre::VertexBufferBinding *>,std::_Rb_tree_iterator<Ogre::VertexBufferBinding *>)
// IDA 0xf675a4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf675a4() {
}

// 0xf675b4 — j___ZNSt8_Rb_treeIPN4Ogre19VertexBufferBindingES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<Ogre::VertexBufferBinding *,Ogre::VertexBufferBinding *,std::_Identity<Ogre::VertexBufferBinding *>,std::less<Ogre::VertexBufferBinding *>,Ogre::STLAllocator<Ogre::VertexBufferBinding *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::VertexBufferBinding *> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre19VertexBufferBindingES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")]
// was: std::_Rb_tree<Ogre::VertexBufferBinding *,Ogre::VertexBufferBinding *,std::_Identity<Ogre::VertexBufferBinding *>,std::less<Ogre::VertexBufferBinding *>,Ogre::STLAllocator<Ogre::VertexBufferBinding *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::VertexBufferBinding *> *)
// IDA 0xf675b4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf675b4() {
}

// 0xf675c4 — j___ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,Ogre::HardwareVertexBuffer *,std::_Identity<Ogre::HardwareVertexBuffer *>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<Ogre::HardwareVertexBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::HardwareVertexBuffer *> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")]
// was: std::_Rb_tree<Ogre::HardwareVertexBuffer *,Ogre::HardwareVertexBuffer *,std::_Identity<Ogre::HardwareVertexBuffer *>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<Ogre::HardwareVertexBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::HardwareVertexBuffer *> *)
// IDA 0xf675c4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf675c4() {
}

// 0xf675d4 — j___ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_25HardwareBufferManagerBase19VertexBufferLicenseEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS7_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_25HardwareBufferManagerBase19VertexBufferLicenseEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS7_E")]
// was: std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>> *)
// IDA 0xf675d4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf675d4() {
}

// 0xf675e4 — j___ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_25HardwareBufferManagerBase19VertexBufferLicenseEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS7_
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_25HardwareBufferManagerBase19VertexBufferLicenseEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS7_")]
// was: std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense> const&)
// IDA 0xf675e4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf675e4() {
}

// 0xf675f4 — j___ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_25HardwareBufferManagerBase19VertexBufferLicenseEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_25HardwareBufferManagerBase19VertexBufferLicenseEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")]
// was: std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>> *)
// IDA 0xf675f4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf675f4() {
}

// 0xf67604 — j___ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_25HardwareBufferManagerBase19VertexBufferLicenseEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSJ_RKS7_
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_25HardwareBufferManagerBase19VertexBufferLicenseEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSJ_RKS7_")]
// was: std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense> const&)
// IDA 0xf67604: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67604() {
}

// 0xf67614 — j___ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_29HardwareVertexBufferSharedPtrEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS6_E
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_29HardwareVertexBufferSharedPtrEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS6_E")]
// was: std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>> *)
// IDA 0xf67614: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67614() {
}

// 0xf67624 — j___ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_29HardwareVertexBufferSharedPtrEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS6_ESI_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>,std::_Rb_tree_iterator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_29HardwareVertexBufferSharedPtrEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS6_ESI_")]
// was: std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>,std::_Rb_tree_iterator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>)
// IDA 0xf67624: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67624() {
}

// 0xf67634 — j___ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_29HardwareVertexBufferSharedPtrEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_29HardwareVertexBufferSharedPtrEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E")]
// was: std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>> *)
// IDA 0xf67634: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67634() {
}

// 0xf67644 — j___ZNSt3mapItN4Ogre29HardwareVertexBufferSharedPtrESt4lessItENS0_12STLAllocatorISt4pairIKtS1_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS6_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<unsigned short,Ogre::HardwareVertexBufferSharedPtr,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](unsigned short const&)")]
#[doc(alias = "j___ZNSt3mapItN4Ogre29HardwareVertexBufferSharedPtrESt4lessItENS0_12STLAllocatorISt4pairIKtS1_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS6_")]
// was: std::map<unsigned short,Ogre::HardwareVertexBufferSharedPtr,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](unsigned short const&)
// IDA 0xf67644: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67644() {
}

// 0xf67654 — j___ZNSt4listIN4Ogre13VertexElementENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE4sortIPFbRKS1_SA_EEEvT_
#[doc(alias = "void std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::sort<bool (*)(Ogre::VertexElement const&,Ogre::VertexElement const&)>(bool (*)(Ogre::VertexElement const&,Ogre::VertexElement const&))")]
#[doc(alias = "j___ZNSt4listIN4Ogre13VertexElementENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE4sortIPFbRKS1_SA_EEEvT_")]
// was: void std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::sort<bool (*)(Ogre::VertexElement const&,Ogre::VertexElement const&)>(bool (*)(Ogre::VertexElement const&,Ogre::VertexElement const&))
// IDA 0xf67654: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67654() {
}

// 0xf67664 — j___ZNSt8_Rb_treeItSt4pairIKtN4Ogre29HardwareVertexBufferSharedPtrEESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS4_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeItSt4pairIKtN4Ogre29HardwareVertexBufferSharedPtrEESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS4_E")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>> *)
// IDA 0xf67664: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67664() {
}

// 0xf67674 — j___ZNSt8_Rb_treeItSt4pairIKtN4Ogre29HardwareVertexBufferSharedPtrEESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeItSt4pairIKtN4Ogre29HardwareVertexBufferSharedPtrEESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr> const&)
// IDA 0xf67674: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67674() {
}

// 0xf67684 — j___ZNSt8_Rb_treeItSt4pairIKtN4Ogre29HardwareVertexBufferSharedPtrEESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>>,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeItSt4pairIKtN4Ogre29HardwareVertexBufferSharedPtrEESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>>,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr> const&)
// IDA 0xf67684: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67684() {
}

// 0xf67694 — j___ZNSt8_Rb_treeItSt4pairIKtN4Ogre29HardwareVertexBufferSharedPtrEESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeItSt4pairIKtN4Ogre29HardwareVertexBufferSharedPtrEESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>> *)
// IDA 0xf67694: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67694() {
}

// 0xf676a4 — j___ZNSt8_Rb_treeItSt4pairIKtN4Ogre29HardwareVertexBufferSharedPtrEESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS4_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeItSt4pairIKtN4Ogre29HardwareVertexBufferSharedPtrEESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS4_")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr> const&)
// IDA 0xf676a4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf676a4() {
}

// 0xf676b4 — j___ZNSt8_Rb_treeItSt4pairIKttESt10_Select1stIS2_ESt4lessItEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_
// type: int __fastcall(char *)
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,unsigned short>,std::_Select1st<std::pair<unsigned short const,unsigned short>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,unsigned short>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,unsigned short> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeItSt4pairIKttESt10_Select1stIS2_ESt4lessItEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,unsigned short>,std::_Select1st<std::pair<unsigned short const,unsigned short>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,unsigned short>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,unsigned short> const&)
// IDA 0xf676b4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf676b4() {
}

// 0xf676c4 — j___ZNSt8_Rb_treeItSt4pairIKttESt10_Select1stIS2_ESt4lessItEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,unsigned short>,std::_Select1st<std::pair<unsigned short const,unsigned short>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,unsigned short>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,unsigned short>>,std::pair<unsigned short const,unsigned short> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeItSt4pairIKttESt10_Select1stIS2_ESt4lessItEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,unsigned short>,std::_Select1st<std::pair<unsigned short const,unsigned short>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,unsigned short>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,unsigned short>>,std::pair<unsigned short const,unsigned short> const&)
// IDA 0xf676c4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf676c4() {
}

// 0xf676d4 — j___ZN4Ogre9SharedPtrINS_19HighLevelGpuProgramEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<Ogre::HighLevelGpuProgram>::destroy(void)")]
#[doc(alias = "j___ZN4Ogre9SharedPtrINS_19HighLevelGpuProgramEE7destroyEv")]
// was: Ogre::SharedPtr<Ogre::HighLevelGpuProgram>::destroy(void)
// IDA 0xf676d4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf676d4() {
}

// 0xf676e4 — j___ZNSt3mapISsPN4Ogre26HighLevelGpuProgramFactoryESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_
#[doc(alias = "std::map<std::string,Ogre::HighLevelGpuProgramFactory *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
#[doc(alias = "j___ZNSt3mapISsPN4Ogre26HighLevelGpuProgramFactoryESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_")]
// was: std::map<std::string,Ogre::HighLevelGpuProgramFactory *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)
// IDA 0xf676e4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf676e4() {
}

// 0xf676f4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre26HighLevelGpuProgramFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,std::_Select1st<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre26HighLevelGpuProgramFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,std::_Select1st<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *> const&)
// IDA 0xf676f4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf676f4() {
}

// 0xf67704 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre26HighLevelGpuProgramFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,std::_Select1st<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>>,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre26HighLevelGpuProgramFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,std::_Select1st<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>>,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *> const&)
// IDA 0xf67704: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67704() {
}

// 0xf67714 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre26HighLevelGpuProgramFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,std::_Select1st<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre26HighLevelGpuProgramFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,std::_Select1st<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)
// IDA 0xf67714: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67714() {
}

// 0xf67724 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre26HighLevelGpuProgramFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,std::_Select1st<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre26HighLevelGpuProgramFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,std::_Select1st<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>> *)
// IDA 0xf67724: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67724() {
}

// 0xf67734 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre26HighLevelGpuProgramFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,std::_Select1st<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre26HighLevelGpuProgramFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,std::_Select1st<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *> const&)
// IDA 0xf67734: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67734() {
}

// 0xf67744 — j___ZN4Ogre15LinearResampler5scaleERKNS_8PixelBoxES3_
// type: _DWORD __fastcall(Ogre::LinearResampler *__hidden this, const Ogre::PixelBox *, const Ogre::PixelBox *)
#[doc(alias = "Ogre::LinearResampler::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
#[doc(alias = "j___ZN4Ogre15LinearResampler5scaleERKNS_8PixelBoxES3_")]
// was: Ogre::LinearResampler::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xf67744: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67744() {
}

// 0xf67754 — j___ZN4Ogre16NearestResamplerILj12EE5scaleERKNS_8PixelBoxES4_
#[doc(alias = "Ogre::NearestResampler<12u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
#[doc(alias = "j___ZN4Ogre16NearestResamplerILj12EE5scaleERKNS_8PixelBoxES4_")]
// was: Ogre::NearestResampler<12u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xf67754: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67754() {
}

// 0xf67764 — j___ZN4Ogre16NearestResamplerILj16EE5scaleERKNS_8PixelBoxES4_
#[doc(alias = "Ogre::NearestResampler<16u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
#[doc(alias = "j___ZN4Ogre16NearestResamplerILj16EE5scaleERKNS_8PixelBoxES4_")]
// was: Ogre::NearestResampler<16u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xf67764: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67764() {
}

// 0xf67774 — j___ZN4Ogre16NearestResamplerILj1EE5scaleERKNS_8PixelBoxES4_
#[doc(alias = "Ogre::NearestResampler<1u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
#[doc(alias = "j___ZN4Ogre16NearestResamplerILj1EE5scaleERKNS_8PixelBoxES4_")]
// was: Ogre::NearestResampler<1u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xf67774: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67774() {
}

// 0xf67784 — j___ZN4Ogre16NearestResamplerILj2EE5scaleERKNS_8PixelBoxES4_
#[doc(alias = "Ogre::NearestResampler<2u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
#[doc(alias = "j___ZN4Ogre16NearestResamplerILj2EE5scaleERKNS_8PixelBoxES4_")]
// was: Ogre::NearestResampler<2u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xf67784: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67784() {
}

// 0xf67794 — j___ZN4Ogre16NearestResamplerILj3EE5scaleERKNS_8PixelBoxES4_
#[doc(alias = "Ogre::NearestResampler<3u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
#[doc(alias = "j___ZN4Ogre16NearestResamplerILj3EE5scaleERKNS_8PixelBoxES4_")]
// was: Ogre::NearestResampler<3u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xf67794: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67794() {
}

// 0xf677a4 — j___ZN4Ogre16NearestResamplerILj4EE5scaleERKNS_8PixelBoxES4_
#[doc(alias = "Ogre::NearestResampler<4u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
#[doc(alias = "j___ZN4Ogre16NearestResamplerILj4EE5scaleERKNS_8PixelBoxES4_")]
// was: Ogre::NearestResampler<4u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xf677a4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf677a4() {
}

// 0xf677b4 — j___ZN4Ogre16NearestResamplerILj6EE5scaleERKNS_8PixelBoxES4_
#[doc(alias = "Ogre::NearestResampler<6u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
#[doc(alias = "j___ZN4Ogre16NearestResamplerILj6EE5scaleERKNS_8PixelBoxES4_")]
// was: Ogre::NearestResampler<6u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xf677b4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf677b4() {
}

// 0xf677c4 — j___ZN4Ogre16NearestResamplerILj8EE5scaleERKNS_8PixelBoxES4_
#[doc(alias = "Ogre::NearestResampler<8u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
#[doc(alias = "j___ZN4Ogre16NearestResamplerILj8EE5scaleERKNS_8PixelBoxES4_")]
// was: Ogre::NearestResampler<8u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xf677c4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf677c4() {
}

// 0xf677d4 — j___ZN4Ogre20LinearResampler_ByteILj1EE5scaleERKNS_8PixelBoxES4_
#[doc(alias = "Ogre::LinearResampler_Byte<1u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
#[doc(alias = "j___ZN4Ogre20LinearResampler_ByteILj1EE5scaleERKNS_8PixelBoxES4_")]
// was: Ogre::LinearResampler_Byte<1u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xf677d4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf677d4() {
}

// 0xf677e4 — j___ZN4Ogre20LinearResampler_ByteILj2EE5scaleERKNS_8PixelBoxES4_
#[doc(alias = "Ogre::LinearResampler_Byte<2u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
#[doc(alias = "j___ZN4Ogre20LinearResampler_ByteILj2EE5scaleERKNS_8PixelBoxES4_")]
// was: Ogre::LinearResampler_Byte<2u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xf677e4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf677e4() {
}

// 0xf677f4 — j___ZN4Ogre20LinearResampler_ByteILj3EE5scaleERKNS_8PixelBoxES4_
#[doc(alias = "Ogre::LinearResampler_Byte<3u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
#[doc(alias = "j___ZN4Ogre20LinearResampler_ByteILj3EE5scaleERKNS_8PixelBoxES4_")]
// was: Ogre::LinearResampler_Byte<3u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xf677f4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf677f4() {
}

// 0xf67804 — j___ZN4Ogre20LinearResampler_ByteILj4EE5scaleERKNS_8PixelBoxES4_
#[doc(alias = "Ogre::LinearResampler_Byte<4u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
#[doc(alias = "j___ZN4Ogre20LinearResampler_ByteILj4EE5scaleERKNS_8PixelBoxES4_")]
// was: Ogre::LinearResampler_Byte<4u>::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xf67804: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67804() {
}

// 0xf67814 — j___ZN4Ogre23LinearResampler_Float325scaleERKNS_8PixelBoxES3_
#[doc(alias = "Ogre::LinearResampler_Float32::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)")]
#[doc(alias = "j___ZN4Ogre23LinearResampler_Float325scaleERKNS_8PixelBoxES3_")]
// was: Ogre::LinearResampler_Float32::scale(Ogre::PixelBox const&,Ogre::PixelBox const&)
// IDA 0xf67814: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67814() {
}

// 0xf67854 — j___ZNSt6vectorIPN4Ogre15RenderOperationENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<Ogre::RenderOperation *,Ogre::STLAllocator<Ogre::RenderOperation *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::RenderOperation **,std::vector<Ogre::RenderOperation *,Ogre::STLAllocator<Ogre::RenderOperation *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::RenderOperation * const&)")]
#[doc(alias = "j___ZNSt6vectorIPN4Ogre15RenderOperationENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")]
// was: std::vector<Ogre::RenderOperation *,Ogre::STLAllocator<Ogre::RenderOperation *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::RenderOperation **,std::vector<Ogre::RenderOperation *,Ogre::STLAllocator<Ogre::RenderOperation *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::RenderOperation * const&)
// IDA 0xf67854: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_0xf67854() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xf679a4 — j___ZNSt8_Rb_treeImSt4pairIKmN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessImENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_
// type: int __fastcall(char *)
#[doc(alias = "std::_Rb_tree<unsigned long,std::pair<unsigned long const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned long const,Ogre::Vector4>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned long const,Ogre::Vector4> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeImSt4pairIKmN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessImENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_")]
// was: std::_Rb_tree<unsigned long,std::pair<unsigned long const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned long const,Ogre::Vector4>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned long const,Ogre::Vector4> const&)
// IDA 0xf679a4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf679a4() {
}

// 0xf679b4 — j___ZNSt8_Rb_treeImSt4pairIKmN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessImENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<unsigned long,std::pair<unsigned long const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned long const,Ogre::Vector4>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::Vector4>>,std::pair<unsigned long const,Ogre::Vector4> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeImSt4pairIKmN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessImENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_")]
// was: std::_Rb_tree<unsigned long,std::pair<unsigned long const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned long const,Ogre::Vector4>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::Vector4>>,std::pair<unsigned long const,Ogre::Vector4> const&)
// IDA 0xf679b4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf679b4() {
}

// 0xf679f4 — j___ZNSt6vectorIN4Ogre18VertexPoseKeyFrame7PoseRefENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
#[doc(alias = "std::vector<Ogre::VertexPoseKeyFrame::PoseRef,Ogre::STLAllocator<Ogre::VertexPoseKeyFrame::PoseRef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::VertexPoseKeyFrame::PoseRef*,std::vector<Ogre::VertexPoseKeyFrame::PoseRef,Ogre::STLAllocator<Ogre::VertexPoseKeyFrame::PoseRef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::VertexPoseKeyFrame::PoseRef const&)")]
#[doc(alias = "j___ZNSt6vectorIN4Ogre18VertexPoseKeyFrame7PoseRefENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")]
// was: std::vector<Ogre::VertexPoseKeyFrame::PoseRef,Ogre::STLAllocator<Ogre::VertexPoseKeyFrame::PoseRef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::VertexPoseKeyFrame::PoseRef*,std::vector<Ogre::VertexPoseKeyFrame::PoseRef,Ogre::STLAllocator<Ogre::VertexPoseKeyFrame::PoseRef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::VertexPoseKeyFrame::PoseRef const&)
// IDA 0xf679f4: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_0xf679f4() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xf67a04 — j___ZNSt6vectorIN4Ogre18VertexPoseKeyFrame7PoseRefENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS8_
#[doc(alias = "std::vector<Ogre::VertexPoseKeyFrame::PoseRef,Ogre::STLAllocator<Ogre::VertexPoseKeyFrame::PoseRef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<Ogre::VertexPoseKeyFrame::PoseRef,Ogre::STLAllocator<Ogre::VertexPoseKeyFrame::PoseRef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
#[doc(alias = "j___ZNSt6vectorIN4Ogre18VertexPoseKeyFrame7PoseRefENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS8_")]
// was: std::vector<Ogre::VertexPoseKeyFrame::PoseRef,Ogre::STLAllocator<Ogre::VertexPoseKeyFrame::PoseRef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<Ogre::VertexPoseKeyFrame::PoseRef,Ogre::STLAllocator<Ogre::VertexPoseKeyFrame::PoseRef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
// IDA 0xf67a04: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67a04() {
}

// 0xf67a14 — j___ZN4Ogre9SharedPtrINS_13AnimableValueEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<Ogre::AnimableValue>::destroy(void)")]
#[doc(alias = "j___ZN4Ogre9SharedPtrINS_13AnimableValueEE7destroyEv")]
// was: Ogre::SharedPtr<Ogre::AnimableValue>::destroy(void)
// IDA 0xf67a14: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67a14() {
}

// 0xf67a24 — j___ZNSt6vectorIN4Ogre18PlaneBoundedVolumeENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_
// type: int __fastcall(int, int, void *)
#[doc(alias = "std::vector<Ogre::PlaneBoundedVolume,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::PlaneBoundedVolume*,std::vector<Ogre::PlaneBoundedVolume,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::PlaneBoundedVolume const&)")]
#[doc(alias = "j___ZNSt6vectorIN4Ogre18PlaneBoundedVolumeENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_")]
// was: std::vector<Ogre::PlaneBoundedVolume,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::PlaneBoundedVolume*,std::vector<Ogre::PlaneBoundedVolume,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::PlaneBoundedVolume const&)
// IDA 0xf67a24: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_0xf67a24() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xf67a34 — j___ZNSt8_Rb_treeItSt4pairIKtN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_
// type: int __fastcall(char *)
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned short const,Ogre::Vector4>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,Ogre::Vector4> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeItSt4pairIKtN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned short const,Ogre::Vector4>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,Ogre::Vector4> const&)
// IDA 0xf67a34: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67a34() {
}

// 0xf67a44 — j___ZNSt8_Rb_treeItSt4pairIKtN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned short const,Ogre::Vector4>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,Ogre::Vector4>>,std::pair<unsigned short const,Ogre::Vector4> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeItSt4pairIKtN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned short const,Ogre::Vector4>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,Ogre::Vector4>>,std::pair<unsigned short const,Ogre::Vector4> const&)
// IDA 0xf67a44: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67a44() {
}

// 0xf67a54 — j___ZNSt8_Rb_treeItSt4pairIKtN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned short const,Ogre::Vector4>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,Ogre::Vector4>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeItSt4pairIKtN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned short const,Ogre::Vector4>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,Ogre::Vector4>> *)
// IDA 0xf67a54: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67a54() {
}

// 0xf67a64 — j___ZSt22__uninitialized_copy_aIPN4Ogre18PlaneBoundedVolumeES2_NS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEET0_T_S9_S8_T1_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "Ogre::PlaneBoundedVolume * std::__uninitialized_copy_a<Ogre::PlaneBoundedVolume *,Ogre::PlaneBoundedVolume *,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(Ogre::PlaneBoundedVolume *,Ogre::PlaneBoundedVolume *,Ogre::PlaneBoundedVolume *,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")]
#[doc(alias = "j___ZSt22__uninitialized_copy_aIPN4Ogre18PlaneBoundedVolumeES2_NS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEET0_T_S9_S8_T1_")]
// was: Ogre::PlaneBoundedVolume * std::__uninitialized_copy_a<Ogre::PlaneBoundedVolume *,Ogre::PlaneBoundedVolume *,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(Ogre::PlaneBoundedVolume *,Ogre::PlaneBoundedVolume *,Ogre::PlaneBoundedVolume *,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)
// IDA 0xf67a64: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67a64() {
}

// 0xf67a74 — j___ZNSt11__iter_swapILb1EE9iter_swapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS5_NS4_12STLAllocatorIS5_NS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEEEESE_EEvT_T0_
#[doc(alias = "void std::__iter_swap<true>::iter_swap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>)")]
#[doc(alias = "j___ZNSt11__iter_swapILb1EE9iter_swapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS5_NS4_12STLAllocatorIS5_NS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEEEESE_EEvT_T0_")]
// was: void std::__iter_swap<true>::iter_swap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>)
// IDA 0xf67a74: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67a74() {
}

// 0xf67a84 — j___ZSt10__pop_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES3_NS2_16LodUsageSortLessEEvT_SE_SE_T0_T1_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "void std::__pop_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortLess)")]
#[doc(alias = "j___ZSt10__pop_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES3_NS2_16LodUsageSortLessEEvT_SE_SE_T0_T1_")]
// was: void std::__pop_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortLess)
// IDA 0xf67a84: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67a84() {
}

// 0xf67a94 — j___ZSt10__pop_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES3_NS2_19LodUsageSortGreaterEEvT_SE_SE_T0_T1_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "void std::__pop_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater)")]
#[doc(alias = "j___ZSt10__pop_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES3_NS2_19LodUsageSortGreaterEEvT_SE_SE_T0_T1_")]
// was: void std::__pop_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater)
// IDA 0xf67a94: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67a94() {
}

// 0xf67aa4 — j___ZSt11__push_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiS3_NS2_16LodUsageSortLessEEvT_T0_SF_T1_T2_
#[doc(alias = "void std::__push_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::MeshLodUsage,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,int,Ogre::MeshLodUsage,Ogre::LodUsageSortLess)")]
#[doc(alias = "j___ZSt11__push_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiS3_NS2_16LodUsageSortLessEEvT_T0_SF_T1_T2_")]
// was: void std::__push_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::MeshLodUsage,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,int,Ogre::MeshLodUsage,Ogre::LodUsageSortLess)
// IDA 0xf67aa4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67aa4() {
}

// 0xf67ab4 — j___ZSt11__push_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiS3_NS2_19LodUsageSortGreaterEEvT_T0_SF_T1_T2_
#[doc(alias = "void std::__push_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,int,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater)")]
#[doc(alias = "j___ZSt11__push_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiS3_NS2_19LodUsageSortGreaterEEvT_T0_SF_T1_T2_")]
// was: void std::__push_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,int,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater)
// IDA 0xf67ab4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67ab4() {
}

// 0xf67ac4 — j___ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiS3_NS2_16LodUsageSortLessEEvT_T0_SF_T1_T2_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "void std::__adjust_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::MeshLodUsage,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,int,Ogre::MeshLodUsage,Ogre::LodUsageSortLess)")]
#[doc(alias = "j___ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiS3_NS2_16LodUsageSortLessEEvT_T0_SF_T1_T2_")]
// was: void std::__adjust_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::MeshLodUsage,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,int,Ogre::MeshLodUsage,Ogre::LodUsageSortLess)
// IDA 0xf67ac4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67ac4() {
}

// 0xf67ad4 — j___ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiS3_NS2_19LodUsageSortGreaterEEvT_T0_SF_T1_T2_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "void std::__adjust_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,int,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater)")]
#[doc(alias = "j___ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiS3_NS2_19LodUsageSortGreaterEEvT_T0_SF_T1_T2_")]
// was: void std::__adjust_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,int,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater)
// IDA 0xf67ad4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67ad4() {
}

// 0xf67ae4 — j___ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_16LodUsageSortLessEEvT_SE_SE_T0_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, char, int, int, int, struct _Unwind_Exception *, int)
#[doc(alias = "void std::__heap_select<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess)")]
#[doc(alias = "j___ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_16LodUsageSortLessEEvT_SE_SE_T0_")]
// was: void std::__heap_select<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess)
// IDA 0xf67ae4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67ae4() {
}

// 0xf67af4 — j___ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_19LodUsageSortGreaterEEvT_SE_SE_T0_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, char, int, int, int, struct _Unwind_Exception *, int)
#[doc(alias = "void std::__heap_select<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater)")]
#[doc(alias = "j___ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_19LodUsageSortGreaterEEvT_SE_SE_T0_")]
// was: void std::__heap_select<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater)
// IDA 0xf67af4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67af4() {
}

// 0xf67b04 — j___ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_16LodUsageSortLessEEvT_SE_T0_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, int, int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, char, int, char, char, int, int, int, int, int)
#[doc(alias = "void std::__insertion_sort<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess)")]
#[doc(alias = "j___ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_16LodUsageSortLessEEvT_SE_T0_")]
// was: void std::__insertion_sort<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess)
// IDA 0xf67b04: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67b04() {
}

// 0xf67b14 — j___ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_19LodUsageSortGreaterEEvT_SE_T0_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, int, int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, char, int, char, char, int, int, int, int, int)
#[doc(alias = "void std::__insertion_sort<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater)")]
#[doc(alias = "j___ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_19LodUsageSortGreaterEEvT_SE_T0_")]
// was: void std::__insertion_sort<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater)
// IDA 0xf67b14: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67b14() {
}

// 0xf67b24 — j___ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiNS2_16LodUsageSortLessEEvT_SE_T0_T1_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, char, int, int, struct _Unwind_Exception *, int)
#[doc(alias = "void std::__introsort_loop<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::LodUsageSortLess)")]
#[doc(alias = "j___ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiNS2_16LodUsageSortLessEEvT_SE_T0_T1_")]
// was: void std::__introsort_loop<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::LodUsageSortLess)
// IDA 0xf67b24: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67b24() {
}

// 0xf67b34 — j___ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiNS2_19LodUsageSortGreaterEEvT_SE_T0_T1_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, char, int, int, struct _Unwind_Exception *, int)
#[doc(alias = "void std::__introsort_loop<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::LodUsageSortGreater)")]
#[doc(alias = "j___ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiNS2_19LodUsageSortGreaterEEvT_SE_T0_T1_")]
// was: void std::__introsort_loop<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::LodUsageSortGreater)
// IDA 0xf67b34: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67b34() {
}

// 0xf67b44 — j___ZSt22__final_insertion_sortIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_16LodUsageSortLessEEvT_SE_T0_
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, char, int, int, struct _Unwind_Exception *, int)
#[doc(alias = "void std::__final_insertion_sort<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess)")]
#[doc(alias = "j___ZSt22__final_insertion_sortIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_16LodUsageSortLessEEvT_SE_T0_")]
// was: void std::__final_insertion_sort<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess)
// IDA 0xf67b44: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67b44() {
}

// 0xf67b54 — j___ZSt22__final_insertion_sortIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_19LodUsageSortGreaterEEvT_SE_T0_
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, char, int, int, struct _Unwind_Exception *, int)
#[doc(alias = "void std::__final_insertion_sort<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater)")]
#[doc(alias = "j___ZSt22__final_insertion_sortIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_19LodUsageSortGreaterEEvT_SE_T0_")]
// was: void std::__final_insertion_sort<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater)
// IDA 0xf67b54: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67b54() {
}

// 0xf67b64 — j___ZSt25__unguarded_linear_insertIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES3_NS2_16LodUsageSortLessEEvT_T0_T1_
#[doc(alias = "void std::__unguarded_linear_insert<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortLess)")]
#[doc(alias = "j___ZSt25__unguarded_linear_insertIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES3_NS2_16LodUsageSortLessEEvT_T0_T1_")]
// was: void std::__unguarded_linear_insert<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortLess)
// IDA 0xf67b64: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67b64() {
}

// 0xf67b74 — j___ZSt25__unguarded_linear_insertIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES3_NS2_19LodUsageSortGreaterEEvT_T0_T1_
#[doc(alias = "void std::__unguarded_linear_insert<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater)")]
#[doc(alias = "j___ZSt25__unguarded_linear_insertIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES3_NS2_19LodUsageSortGreaterEEvT_T0_T1_")]
// was: void std::__unguarded_linear_insert<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater)
// IDA 0xf67b74: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67b74() {
}

// 0xf67b84 — j___ZSt8pop_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_16LodUsageSortLessEEvT_SE_T0_
#[doc(alias = "void std::pop_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess)")]
#[doc(alias = "j___ZSt8pop_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_16LodUsageSortLessEEvT_SE_T0_")]
// was: void std::pop_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess)
// IDA 0xf67b84: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67b84() {
}

// 0xf67b94 — j___ZSt8pop_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_19LodUsageSortGreaterEEvT_SE_T0_
#[doc(alias = "void std::pop_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater)")]
#[doc(alias = "j___ZSt8pop_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_19LodUsageSortGreaterEEvT_SE_T0_")]
// was: void std::pop_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater)
// IDA 0xf67b94: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67b94() {
}

// 0xf67ba4 — j___ZSt9make_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_16LodUsageSortLessEEvT_SE_T0_
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, char, int, int, struct _Unwind_Exception *, int)
#[doc(alias = "void std::make_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess)")]
#[doc(alias = "j___ZSt9make_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_16LodUsageSortLessEEvT_SE_T0_")]
// was: void std::make_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess)
// IDA 0xf67ba4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67ba4() {
}

// 0xf67bb4 — j___ZSt9make_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_19LodUsageSortGreaterEEvT_SE_T0_
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, char, int, int, struct _Unwind_Exception *, int)
#[doc(alias = "void std::make_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater)")]
#[doc(alias = "j___ZSt9make_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_19LodUsageSortGreaterEEvT_SE_T0_")]
// was: void std::make_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater)
// IDA 0xf67bb4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67bb4() {
}

// 0xf67bc4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre11LodStrategyEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::LodStrategy *>,std::_Select1st<std::pair<std::string const,Ogre::LodStrategy *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::LodStrategy *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::LodStrategy *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre11LodStrategyEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::LodStrategy *>,std::_Select1st<std::pair<std::string const,Ogre::LodStrategy *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::LodStrategy *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::LodStrategy *> const&)
// IDA 0xf67bc4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67bc4() {
}

// 0xf67bd4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre11LodStrategyEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::LodStrategy *>,std::_Select1st<std::pair<std::string const,Ogre::LodStrategy *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::LodStrategy *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre11LodStrategyEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::LodStrategy *>,std::_Select1st<std::pair<std::string const,Ogre::LodStrategy *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::LodStrategy *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)
// IDA 0xf67bd4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67bd4() {
}

// 0xf67be4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre11LodStrategyEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::LodStrategy *>,std::_Select1st<std::pair<std::string const,Ogre::LodStrategy *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::LodStrategy *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::LodStrategy *>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre11LodStrategyEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::LodStrategy *>,std::_Select1st<std::pair<std::string const,Ogre::LodStrategy *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::LodStrategy *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::LodStrategy *>> *)
// IDA 0xf67be4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67be4() {
}

// 0xf67bf4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre11LodStrategyEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::LodStrategy *>,std::_Select1st<std::pair<std::string const,Ogre::LodStrategy *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::LodStrategy *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::LodStrategy *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre11LodStrategyEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::LodStrategy *>,std::_Select1st<std::pair<std::string const,Ogre::LodStrategy *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::LodStrategy *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::LodStrategy *> const&)
// IDA 0xf67bf4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xf67bf4() {
}

#[cfg(test)]
mod texture_manager_tests {
    use super::*;
    use crate::generated_502::TextureUnitState;

    #[test]
    fn preferred_depths_round_trip_per_ida_offsets() {
        let mut manager = TextureManager::default();
        // IDA 0xe486e8/0xe48764 read +152/+154; 0xe487f0/0xe4883c write/read +156.
        stub_0xe486f0(&mut manager, 16, false);
        assert_eq!(stub_0xe486e8(&manager), 0);
        assert_eq!(stub_0xe48764(&manager), 16);
        stub_0xe4876c(&mut manager, 32, 16, false);
        assert_eq!(stub_0xe486e8(&manager), 32);
        assert_eq!(stub_0xe48764(&manager), 16);
        stub_0xe487f0(&mut manager, 5);
        assert_eq!(stub_0xe4883c(&manager), 5);
    }

    #[test]
    fn float_depth_reload_cycles_loaded_textures() {
        let mut manager = TextureManager::default();
        manager.resources_loaded = true;
        manager.textures = vec![
            ManagedTexture { loaded: true, reloadable: true, ..ManagedTexture::default() },
            ManagedTexture { loaded: true, reloadable: false, ..ManagedTexture::default() },
            ManagedTexture::default(),
        ];
        // IDA 0xe486f0: loaded + reloadable cycles unload/set/load; rest only take the depth.
        stub_0xe486f0(&mut manager, 32, true);
        assert_eq!(manager.textures[0].float_bit_depth, 32);
        assert_eq!(manager.textures[0].load_calls, 1);
        assert!(manager.textures[0].loaded);
        assert_eq!(manager.textures[1].float_bit_depth, 32);
        assert_eq!(manager.textures[1].load_calls, 0);
        assert_eq!(manager.textures[2].float_bit_depth, 32);
        // No reload requested → the per-texture loop is skipped (IDA 0xe48784 guard);
        // only the manager defaults (checked above) store.
        stub_0xe4876c(&mut manager, 8, 16, false);
        assert_eq!(manager.textures[0].integer_bit_depth, 0);
        assert_eq!(manager.textures[0].load_calls, 1);
        // Reload with both depths cycles the loaded + reloadable entry with load(false).
        stub_0xe4876c(&mut manager, 16, 32, true);
        assert_eq!(manager.textures[0].integer_bit_depth, 16);
        assert_eq!(manager.textures[0].float_bit_depth, 32);
        assert_eq!(manager.textures[0].load_calls, 2);
        assert_eq!(manager.textures[1].load_calls, 0);
    }

    #[test]
    fn format_support_matches_ida_comparisons() {
        let manager = TextureManager::default();
        // IDA 0xe487f8: identity native format is supported.
        assert!(stub_0xe487f8(&manager, 1, 11, 0));
        // IDA 0xe48814: identity is always bit-equivalent.
        assert!(stub_0xe48814(&manager, 1, 6, 0));
        assert_eq!(pixel_elem_bits(11), 32);
        assert_eq!(pixel_elem_bits(6), 16);
        // Narrower native (R5G6B5 for an A8R8G8B8 request) is not equivalent.
        let mut narrow = TextureManager::default();
        narrow.native_format_override = Some(6);
        assert!(!stub_0xe487f8(&narrow, 1, 11, 0));
        assert!(!stub_0xe48814(&narrow, 1, 11, 0));
        // Wider native stays equivalent but not identical.
        narrow.native_format_override = Some(27);
        assert!(!stub_0xe487f8(&narrow, 1, 11, 0));
        assert!(stub_0xe48814(&narrow, 1, 11, 0));
    }

    #[test]
    fn set_texture_name_installs_single_frame() {
        let mut state = TextureUnitState::default();
        // IDA 0xe492bc non-cube path: named content, one frame, flag/type update.
        stub_0xe492bc(&mut state, "brick.png", 2);
        assert_eq!(state.content_type, 0);
        assert!(!state.load_failed);
        assert_eq!(state.frames, vec!["brick.png".to_owned()]);
        assert_eq!(state.textures.len(), 1);
        assert_eq!(state.current_frame, 0);
        assert_eq!(state.flag_08, 0);
        assert_eq!(state.texture_type, 2);
        // Non-empty name with unloaded parent: no load, but the pass hash dirties.
        assert!(state.parent_dirty);
        assert!(!state.textures[0].loaded);
        // Loaded parent → _load marks the slot loaded (IDA 0xe49524..0xe4952a).
        let mut loaded_parent = TextureUnitState::default();
        loaded_parent.parent_loaded = true;
        stub_0xe492bc(&mut loaded_parent, "brick.png", 2);
        assert!(loaded_parent.textures[0].loaded);
        // Empty name skips both the load and the hash-dirty (IDA 0xe4950e guard).
        let mut empty = TextureUnitState::default();
        empty.parent_loaded = true;
        stub_0xe492bc(&mut empty, "", 2);
        assert!(!empty.parent_dirty);
    }

    #[test]
    fn set_texture_name_cube_dispatches() {
        let mut state = TextureUnitState::default();
        // IDA 0xe49316..0xe49324: type 4 → setCubicTextureName(name, true).
        stub_0xe492bc(&mut state, "sky.png", TEXTURE_TYPE_CUBE_MAP);
        assert_eq!(state.frames[0], "sky.png".to_owned());
        assert_eq!(state.flag_08, 1);
        assert!(state.parent_dirty);
    }

    #[test]
    fn set_texture_coord_set_stores_index() {
        let mut state = TextureUnitState::default();
        // IDA 0xe4964c: STR R1, [R0, #0x18].
        stub_0xe4964c(&mut state, 3);
        assert_eq!(state.texture_coord_set, 3);
    }
}
