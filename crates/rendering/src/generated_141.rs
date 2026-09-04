//! rendering shard 141 — 27 stubs EA-sorted filtered wide (15586 total, 15559->15586 covered, 0 remaining) — 0xc6eb18..0xf6ad84
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Batch-2 impl: all 27 ported from IDA decompile+disasm (see per-EA notes).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// ---- Ogre compositor/renderable/controller ports (IDA 0xc6eb18..0xc85874) ----

// 0xc6eb18 — __ZNK4Ogre10Renderable19setRenderSystemDataEPNS0_16RenderSystemDataE
#[doc(alias = "Ogre::Renderable::setRenderSystemData(Ogre::Renderable::RenderSystemData *)const")]
// was: Ogre::Renderable::setRenderSystemData(Ogre::Renderable::RenderSystemData *)const
// IDA 0xc6eb18: `STR R1, [R0,#0x2C]` then `BX LR` — stores the
// RenderSystemData* word at +44 and returns `this` (IDA decompile:
// `*(_DWORD *)(result + 44) = a2; return result;`).
// Maps onto `crate::movable::Renderable::render_system_data`; a null word
// becomes `None`.
pub fn stub_c6eb18(renderable: &mut crate::movable::Renderable, data: usize) {
    // IDA 0xc6eb18: store word.
    renderable.render_system_data = if data == 0 { None } else { Some(data) };
}

/// was: `Ogre::CompositionPass::InputTex` — one entry of `mInputs`.
/// 8-byte stride in the binary (`ADD R0, R6, R5, LSL #3` at IDA `0xc7092a`):
/// a 4-byte string plus a 4-byte MRT index.
#[doc(alias = "Ogre::CompositionPass::InputTex")]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CompositionInput {
    pub name: String,
    pub mrt_index: u32,
}

/// was: `Ogre::CompositionPass` — parent at `+0`, pass type at `+4`, id at
/// `+8`, material at `+16`, render queues at `+28`/`+29`, material scheme at
/// `+32`, clear buffers at `+36`, clear colour at `+40`, clear depth at `+56`,
/// clear stencil at `+60`, inputs at `+64`, stencil block at `+192`..`+220`,
/// quad corners at `+224`, custom type at `+244` (IDA `0xc70234`..`0xc70394`).
#[doc(alias = "Ogre::CompositionPass")]
#[derive(Clone, Debug)]
pub struct CompositionPass {
    /// Owning `CompositionTargetPass *` at `+0` (IDA `0xc70256`).
    pub parent: usize,
    /// Pass type at `+4` (init `3`, IDA `0xc7025c`).
    pub pass_type: u32,
    /// Identifier at `+8` (init `0`, IDA `0xc70260`).
    pub identifier: u32,
    /// Resolved material handle at `+16` (IDA `0xc70772`); the name itself
    /// is not retained (IDA `0xc706e4` stores only the handle).
    pub material: Option<usize>,
    /// Custom type string at `+244` (IDA `0xc70394`, `0xc70a6e`).
    pub custom_type: String,
    /// Material scheme string at `+32` (IDA `0xc70a04`).
    pub material_scheme: String,
    /// First render queue byte at `+28` (init `0`, IDA `0xc70284`).
    pub first_render_queue: u8,
    /// Last render queue byte at `+29` (init `95`, IDA `0xc70292`).
    pub last_render_queue: u8,
    /// Clear-buffer bits at `+36` (init `3`, IDA `0xc702dc`).
    pub clear_buffers: u32,
    /// Clear colour words at `+40` (init black, IDA `0xc702e4`).
    pub clear_colour: crate::movable::ColourValue,
    /// Clear depth float at `+56` (init `1.0`, IDA `0xc702ec`).
    pub clear_depth: f32,
    /// Clear stencil word at `+60` (IDA `0xc70a14`).
    pub clear_stencil: u32,
    /// Stencil check byte at `+192` (IDA `0xc7032a`, `0xc70a18`).
    pub stencil_check: bool,
    /// Stencil compare function at `+196` (init `1`, IDA `0xc70330`).
    pub stencil_func: u32,
    /// Stencil reference value at `+200` (IDA `0xc7033a`).
    pub stencil_ref_value: u32,
    /// Stencil mask at `+204` (init all-bits, IDA `0xc70340`).
    pub stencil_mask: u32,
    /// Stencil fail op at `+208` (IDA `0xc70348`).
    pub stencil_fail_op: u32,
    /// Stencil depth-fail op at `+212` (IDA `0xc7034c`).
    pub stencil_depth_fail_op: u32,
    /// Stencil pass op at `+216` (IDA `0xc70350`).
    pub stencil_pass_op: u32,
    /// Two-sided stencil byte at `+220` (IDA `0xc70a50`).
    pub stencil_two_sided: bool,
    /// Quad corner floats at `+224` (init `-1,1,1,-1`, IDA
    /// `0xc7035e`..`0xc70374`).
    pub quad_corners: [f32; 4],
    /// Quad far-corner flags at `+240`/`+241` (IDA `0xc7037a`..`0xc7038a`,
    /// `0xc70a58`).
    pub quad_far_corners: (bool, bool),
    /// `mInputs` entries at `+64`, 8-byte stride, 16 slots
    /// (IDA `0xc702fa`..`0xc7032a`, `0xc7092a`).
    pub inputs: Vec<CompositionInput>,
}

impl Default for CompositionPass {
    fn default() -> Self {
        Self::new(0)
    }
}

impl CompositionPass {
    /// `CompositionPass::CompositionPass(parent)` (IDA `0xc70234`, via C1 at
    /// `0xc70228`): parent at `+0`, type `3`, id `0`, queues `0`/`95`,
    /// scheme/custom blank, clear buffers `3`, colour black, depth `1.0`,
    /// 16 blank inputs, stencil func `1` / mask all-bits, quad corners
    /// `-1,1,1,-1` (IDA `0xc70330`..`0xc70374`).
    pub fn new(parent: usize) -> Self {
        Self {
            parent,
            pass_type: 3,
            identifier: 0,
            material: None,
            custom_type: String::new(),
            material_scheme: String::new(),
            first_render_queue: 0,
            last_render_queue: 95,
            clear_buffers: 3,
            clear_colour: crate::movable::ColourValue::default(),
            clear_depth: 1.0,
            clear_stencil: 0,
            stencil_check: false,
            stencil_func: 1,
            stencil_ref_value: 0,
            stencil_mask: u32::MAX,
            stencil_fail_op: 0,
            stencil_depth_fail_op: 0,
            stencil_pass_op: 0,
            stencil_two_sided: false,
            quad_corners: [-1.0, 1.0, 1.0, -1.0],
            quad_far_corners: (false, false),
            inputs: vec![CompositionInput::default(); 16],
        }
    }

    /// `CompositionPass::setType` (IDA `0xc706dc`): store word at `+4`.
    pub fn set_type(&mut self, pass_type: u32) {
        self.pass_type = pass_type;
    }

    /// `CompositionPass::setIdentifier` (IDA `0xc706e0`): store word at `+8`.
    pub fn set_identifier(&mut self, identifier: u32) {
        self.identifier = identifier;
    }

    /// `CompositionPass::setMaterialName` (IDA `0xc706e4`): resolve through
    /// the material manager and store the handle at `+16` (releasing the
    /// previous one); the name itself is not retained.
    pub fn set_material_name(
        &mut self,
        name: &str,
        resolver: &dyn crate::movable::MaterialResolver,
    ) {
        self.material = resolver.load_material(name);
    }

    /// `CompositionPass::setClearBuffers` (IDA `0xc708b8`): store at `+36`.
    pub fn set_clear_buffers(&mut self, buffers: u32) {
        self.clear_buffers = buffers;
    }

    /// `CompositionPass::setClearColour` (IDA `0xc708bc`): store the four
    /// channel words at `+40`.
    pub fn set_clear_colour(&mut self, colour: crate::movable::ColourValue) {
        self.clear_colour = colour;
    }

    /// `CompositionPass::setInput` (IDA `0xc708cc`): assign name + MRT index
    /// at slot `id`.
    /// // BUG: original at `0xc708cc` performs no bounds check.
    pub fn set_input(&mut self, id: usize, name: &str, mrt_index: u32) {
        let slot = &mut self.inputs[id];
        slot.name = name.to_owned();
        slot.mrt_index = mrt_index;
    }

    /// `CompositionPass::setFirstRenderQueue` (IDA `0xc709fc`): byte at `+28`.
    pub fn set_first_render_queue(&mut self, queue: u8) {
        self.first_render_queue = queue;
    }

    /// `CompositionPass::setLastRenderQueue` (IDA `0xc70a00`): byte at `+29`.
    pub fn set_last_render_queue(&mut self, queue: u8) {
        self.last_render_queue = queue;
    }

    /// `CompositionPass::setMaterialScheme` (IDA `0xc70a04`): assign the
    /// string at `+32`.
    pub fn set_material_scheme(&mut self, scheme: &str) {
        self.material_scheme = scheme.to_string();
    }

    /// `CompositionPass::setClearDepth` (IDA `0xc70a10`): float at `+56`.
    pub fn set_clear_depth(&mut self, depth: f32) {
        self.clear_depth = depth;
    }

    /// `CompositionPass::setClearStencil` (IDA `0xc70a14`): word at `+60`.
    pub fn set_clear_stencil(&mut self, stencil: u32) {
        self.clear_stencil = stencil;
    }

    /// `CompositionPass::setStencilCheck` (IDA `0xc70a18`): byte at `+192`.
    pub fn set_stencil_check(&mut self, check: bool) {
        self.stencil_check = check;
    }

    /// `CompositionPass::setStencilFunc` (IDA `0xc70a20`): word at `+196`.
    pub fn set_stencil_func(&mut self, func: u32) {
        self.stencil_func = func;
    }

    /// `CompositionPass::setStencilRefValue` (IDA `0xc70a28`): word at `+200`.
    pub fn set_stencil_ref_value(&mut self, value: u32) {
        self.stencil_ref_value = value;
    }

    /// `CompositionPass::setStencilMask` (IDA `0xc70a30`): word at `+204`.
    pub fn set_stencil_mask(&mut self, mask: u32) {
        self.stencil_mask = mask;
    }

    /// `CompositionPass::setStencilFailOp` (IDA `0xc70a38`): word at `+208`.
    pub fn set_stencil_fail_op(&mut self, op: u32) {
        self.stencil_fail_op = op;
    }

    /// `CompositionPass::setStencilDepthFailOp` (IDA `0xc70a40`): word at
    /// `+212`.
    pub fn set_stencil_depth_fail_op(&mut self, op: u32) {
        self.stencil_depth_fail_op = op;
    }

    /// `CompositionPass::setStencilPassOp` (IDA `0xc70a48`): word at `+216`.
    pub fn set_stencil_pass_op(&mut self, op: u32) {
        self.stencil_pass_op = op;
    }

    /// `CompositionPass::setStencilTwoSidedOperation` (IDA `0xc70a50`):
    /// byte at `+220`.
    pub fn set_stencil_two_sided(&mut self, two_sided: bool) {
        self.stencil_two_sided = two_sided;
    }

    /// `CompositionPass::setQuadFarCorners` (IDA `0xc70a58`): bytes at
    /// `+240`/`+241`.
    pub fn set_quad_far_corners(&mut self, a: bool, b: bool) {
        self.quad_far_corners = (a, b);
    }

    /// `CompositionPass::setCustomType` (IDA `0xc70a64`): assign the string
    /// at `+244`.
    pub fn set_custom_type(&mut self, custom_type: &str) {
        self.custom_type = custom_type.to_string();
    }

    /// `CompositionPass::_isSupported` (IDA `0xc70a70`): non-`3` types need
    /// no material and pass; type `3` compiles the material and requires at
    /// least one supported technique (null material fails).
    pub fn is_supported(&self, support: &dyn crate::movable::PassMaterialSupport) -> bool {
        if self.pass_type != 3 {
            return true;
        }
        match self.material {
            Some(handle) => {
                support.compile_material(handle);
                support.supported_techniques(handle) != 0
            }
            None => false,
        }
    }
}

/// was: `Ogre::CompositionTargetPass` — technique at `+0`, input mode at
/// `+4`, output name at `+8`, passes at `+12`, only-initial at `+28`,
/// visibility mask at `+32`, lod bias at `+36`, material scheme at `+40`,
/// shadows flag at `+44` (IDA `0xc70ae4`..`0xc70bfc`).
#[doc(alias = "Ogre::CompositionTargetPass")]
#[derive(Clone, Debug)]
pub struct CompositionTargetPass {
    /// Owning `CompositionTechnique *` at `+0` (IDA `0xc70b0e`).
    pub parent: usize,
    /// Input mode at `+4` (init `0`, IDA `0xc70b18`).
    pub input_mode: u32,
    /// Output name at `+8` (init blank, IDA `0xc70b28`).
    pub output_name: String,
    /// Pass list at `+12` (IDA `0xc70e9a`..`0xc70ebe`).
    pub passes: Vec<CompositionPass>,
    /// Only-initial byte at `+28` (IDA `0xc70e18`).
    pub only_initial: bool,
    /// Visibility mask at `+32` (init all-bits, IDA `0xc70b4a`).
    pub visibility_mask: u32,
    /// Lod bias float at `+36` (init `1.0`, IDA `0xc70b5c`).
    pub lod_bias: f32,
    /// Material scheme at `+40` (init `DEFAULT_SCHEME_NAME`, overridden by
    /// the render-system name when present, IDA `0xc70b66`..`0xc70bda`).
    pub material_scheme: String,
    /// Shadows flag at `+44` (init true, IDA `0xc70ba0`).
    pub shadows_enabled: bool,
}

impl CompositionTargetPass {
    /// `CompositionTargetPass::CompositionTargetPass(technique)` (IDA
    /// `0xc70ae4`, via C1 at `0xc70ad8`).
    pub fn new(parent: usize, render_system_scheme: Option<&str>) -> Self {
        Self {
            parent,
            input_mode: 0,
            output_name: String::new(),
            passes: Vec::new(),
            only_initial: false,
            visibility_mask: u32::MAX,
            lod_bias: 1.0,
            material_scheme: render_system_scheme
                .unwrap_or("Default")
                .to_string(),
            shadows_enabled: true,
        }
    }

    /// `CompositionTargetPass::setInputMode` (IDA `0xc70e08`): word at `+4`.
    pub fn set_input_mode(&mut self, mode: u32) {
        self.input_mode = mode;
    }

    /// `CompositionTargetPass::setOutputName` (IDA `0xc70e0c`): string at `+8`.
    pub fn set_output_name(&mut self, name: &str) {
        self.output_name = name.to_string();
    }

    /// `CompositionTargetPass::setOnlyInitial` (IDA `0xc70e18`): byte at `+28`.
    pub fn set_only_initial(&mut self, only: bool) {
        self.only_initial = only;
    }

    /// `CompositionTargetPass::setVisibilityMask` (IDA `0xc70e1c`): word at
    /// `+32`.
    pub fn set_visibility_mask(&mut self, mask: u32) {
        self.visibility_mask = mask;
    }

    /// `CompositionTargetPass::setLodBias` (IDA `0xc70e20`): float at `+36`.
    pub fn set_lod_bias(&mut self, bias: f32) {
        self.lod_bias = bias;
    }

    /// `CompositionTargetPass::setMaterialScheme` (IDA `0xc70e24`): string
    /// at `+40`.
    pub fn set_material_scheme(&mut self, scheme: &str) {
        self.material_scheme = scheme.to_string();
    }

    /// `CompositionTargetPass::setShadowsEnabled` (IDA `0xc70e30`): byte at
    /// `+44`.
    pub fn set_shadows_enabled(&mut self, enabled: bool) {
        self.shadows_enabled = enabled;
    }

    /// `CompositionTargetPass::createPass` (IDA `0xc70e38`): construct a
    /// `CompositionPass` owned by `self`, append it, return its index.
    pub fn create_pass(&mut self) -> usize {
        let parent = self as *const Self as usize;
        self.passes.push(CompositionPass::new(parent));
        self.passes.len() - 1
    }

    /// `CompositionTargetPass::_isSupported` (IDA `0xc70f2c`): logical AND
    /// over the passes; empty passes yield true.
    pub fn is_supported(&self, support: &dyn crate::movable::PassMaterialSupport) -> bool {
        self.passes.iter().all(|p| p.is_supported(support))
    }
}

/// Texture support probe behind `CompositionTechnique::isSupported` (IDA
/// `0xc717c8`..`0xc71846`): render-system MRT capacity plus per-format
/// validation through the texture manager.
pub trait TechniqueSupport: crate::movable::PassMaterialSupport {
    /// Max simultaneous MRT buffers (`+816` caps word, IDA `0xc717f8`).
    fn max_mrt_buffers(&self) -> u16;
    /// Format validation; `srgb` selects the `isEquivalentFormat` (`a2 ==
    /// 1`, IDA `0xc71826`) vs `isSupportedFormat` (IDA `0xc71840`) path.
    fn texture_format_ok(&self, format: u32, srgb: bool) -> bool;
}

/// was: `Ogre::CompositionTechnique::TextureDefinition` — name at `+0`,
/// width/height at `+12`/`+16`, size factors at `+20`/`+24`, format list at
/// `+32` (IDA `0xc7161a`..`0xc7164e`, `0xc717de`..`0xc7180c`).
#[doc(alias = "Ogre::CompositionTechnique::TextureDefinition")]
#[derive(Clone, Debug)]
pub struct TextureDefinition {
    /// Texture name assigned at creation (IDA `0xc7164e`).
    pub name: String,
    /// Explicit width/height (`0` = sized by factor, IDA `0xc71626`..).
    pub width: u32,
    pub height: u32,
    /// Size factors (init `1.0`, IDA `0xc7162a`..`0xc7162c`).
    pub width_factor: f32,
    pub height_factor: f32,
    /// Pixel formats probed by `isSupported` (IDA `0xc71802`..`0xc71850`).
    pub format_ids: Vec<u32>,
}

impl TextureDefinition {
    /// `createTextureDefinition` field init (IDA `0xc7161a`..`0xc7164a`).
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            width: 0,
            height: 0,
            width_factor: 1.0,
            height_factor: 1.0,
            format_ids: Vec::new(),
        }
    }
}

/// was: `Ogre::CompositionTechnique` — compositor at `+4`, texture
/// definitions at `+8`, target passes at `+24`, output target pass at `+40`,
/// scheme name at `+44` (IDA `0xc71094`..`0xc71188`).
#[doc(alias = "Ogre::CompositionTechnique")]
#[derive(Clone, Debug)]
pub struct CompositionTechnique {
    /// Owning `Compositor *` at `+4` (IDA `0xc710ce`).
    pub parent: usize,
    /// Texture definitions at `+8` (IDA `0xc71652`..`0xc7166a`).
    pub texture_definitions: Vec<TextureDefinition>,
    /// Target passes at `+24` (IDA `0xc717aa`..`0xc717ba`).
    pub target_passes: Vec<CompositionTargetPass>,
    /// Output target pass at `+40`, created by the ctor (IDA
    /// `0xc71158`..`0xc71168`).
    pub output_target_pass: CompositionTargetPass,
    /// Scheme name at `+44` (IDA `0xc71120`, `0xc718da`).
    pub scheme_name: String,
}

impl CompositionTechnique {
    /// `CompositionTechnique::CompositionTechnique(compositor)` (IDA
    /// `0xc71094`, via C1 at `0xc71088`): empty definition/pass lists,
    /// blank scheme, fresh output target pass owned by `self`.
    pub fn new(parent: usize) -> Self {
        let mut technique = Self {
            parent,
            texture_definitions: Vec::new(),
            target_passes: Vec::new(),
            output_target_pass: CompositionTargetPass::new(0, None),
            scheme_name: String::new(),
        };
        let addr = &technique as *const Self as usize;
        technique.output_target_pass.parent = addr;
        technique
    }

    /// `CompositionTechnique::removeAllTextureDefinitions` (IDA `0xc71474`):
    /// destroy every definition, leave the list empty.
    pub fn remove_all_texture_definitions(&mut self) {
        self.texture_definitions.clear();
    }

    /// `CompositionTechnique::createTextureDefinition(name)` (IDA `0xc715e0`):
    /// default-init a definition, assign the name, append it, return its
    /// index.
    pub fn create_texture_definition(&mut self, name: &str) -> usize {
        self.texture_definitions.push(TextureDefinition::new(name));
        self.texture_definitions.len() - 1
    }

    /// `CompositionTechnique::getTextureDefinitionIterator` (IDA `0xc71688`):
    /// begin/end view over the definition list.
    pub fn texture_definitions(&self) -> &[TextureDefinition] {
        &self.texture_definitions
    }

    /// `CompositionTechnique::createTargetPass` (IDA `0xc71694`):
    /// construct a target pass owned by `self`, append it, return its index.
    pub fn create_target_pass(&mut self) -> usize {
        let parent = self as *const Self as usize;
        self.target_passes
            .push(CompositionTargetPass::new(parent, None));
        self.target_passes.len() - 1
    }

    /// `CompositionTechnique::getOutputTargetPass` (IDA `0xc71788`): word at
    /// `+40`.
    pub fn output_target_pass(&self) -> &CompositionTargetPass {
        &self.output_target_pass
    }

    /// `CompositionTechnique::isSupported(srgb)` (IDA `0xc7178c`): the
    /// output target pass must pass, then every target pass, then every
    /// texture definition's format list must fit the MRT caps and validate.
    pub fn is_supported(&self, srgb: bool, support: &dyn TechniqueSupport) -> bool {
        if !self.output_target_pass.is_supported(support) {
            return false; // IDA `0xc717a0`..`0xc717a6`
        }
        if !self
            .target_passes
            .iter()
            .all(|p| p.is_supported(support))
        {
            return false; // IDA `0xc717aa`..`0xc717c0`
        }
        for def in &self.texture_definitions {
            // IDA `0xc717de`..`0xc71800`: format count must fit the caps.
            if (def.format_ids.len() as u32) > support.max_mrt_buffers() as u32 {
                return false;
            }
            for format in &def.format_ids {
                if !support.texture_format_ok(*format, srgb) {
                    return false; // IDA `0xc71812`..`0xc71846`
                }
            }
        }
        true
    }

    /// `CompositionTechnique::setSchemeName` (IDA `0xc718d0`): assign the
    /// string at `+44`.
    pub fn set_scheme_name(&mut self, name: &str) {
        self.scheme_name = name.to_string();
    }
}

// 0xc708cc — __ZN4Ogre15CompositionPass8setInputEmRKSsm
#[doc(alias = "Ogre::CompositionPass::setInput(unsigned long,std::string const&,unsigned long)")]
// was: Ogre::CompositionPass::setInput(unsigned long,std::string const&,unsigned long)
// IDA 0xc708cc: copies the input string into a local (`0xc708f6`), indexes
// `&mInputs[id]` as `this + id*8 + 0x40` (`0xc7092a`/`0xc70930`, no bounds
// check or resize), assigns the name (`0xc70934`), stores the MRT index word.
pub fn stub_c708cc(pass: &mut CompositionPass, id: usize, input: &str, mrt_index: u32) {
    pass.set_input(id, input, mrt_index);
}

/// was: `Ogre::MaterialManager::DEFAULT_SCHEME_NAME` — loaded by pointer at
/// IDA `0xc70b8a`..`0xc70b98`. Value `"Default"` is the Ogre
/// `MaterialManager::DEFAULT_SCHEME_NAME` convention; IDA only shows the load.
#[doc(alias = "Ogre::MaterialManager::DEFAULT_SCHEME_NAME")]
pub const DEFAULT_SCHEME_NAME: &str = "Default";

// 0xc70ae4 — __ZN4Ogre21CompositionTargetPassC2EPNS_20CompositionTechniqueE
#[doc(alias = "Ogre::CompositionTargetPass::CompositionTargetPass(Ogre::CompositionTechnique *)")]
// was: Ogre::CompositionTargetPass::CompositionTargetPass(Ogre::CompositionTechnique *)
// IDA 0xc70ae4 (C2, 168 insns): stores the parent, zeroes the flag word,
// default-constructs the output-name string and the passes vector, sets
// visibility mask `0xFFFFFFFF` and `1.0f` bias, copies
// `MaterialManager::DEFAULT_SCHEME_NAME` into the scheme string, sets the
// +44 shadows byte, then — only if `Root::getSingleton().getRenderSystem()`
// is non-null — reassigns the scheme from the render system's name
// (`0xc70ba8`..`0xc70bda`). Remainder is SjLj landing pads.
pub fn stub_c70ae4(parent: usize, render_system_name: Option<&str>) -> CompositionTargetPass {
    CompositionTargetPass::new(parent, render_system_name)
}

// 0xc70ad8 — __ZN4Ogre21CompositionTargetPassC1EPNS_20CompositionTechniqueE
#[doc(alias = "Ogre::CompositionTargetPass::CompositionTargetPass(Ogre::CompositionTechnique *)")]
// was: Ogre::CompositionTargetPass::CompositionTargetPass(Ogre::CompositionTechnique *)
// IDA 0xc70ad8: complete-object ctor veneer — `PUSH; BL C2 (0xc70ae4); POP`.
// Forwards to C2.
pub fn stub_c70ad8(parent: usize, render_system_name: Option<&str>) -> CompositionTargetPass {
    // IDA 0xc70adc: tail-branch to C2.
    stub_c70ae4(parent, render_system_name)
}

/// was: `Ogre::FrameTimeControllerValue` — the manager-owned frame-time
/// source shared into every texture scroller (IDA `0xc7972e`..`0xc79760`:
/// the controller copies the manager's source `shared_ptr` triple at
/// `this+32..+43` with a use-count bump). Payload is opaque here.
#[doc(alias = "Ogre::FrameTimeControllerValue")]
#[derive(Clone, Debug, Default)]
pub struct FrameTimeValue;

/// was: `Ogre::TexCoordModifierControllerValue` — destination value driving
/// a texture layer's UV transform. Ctor args observed at IDA `0xc79708`
/// (UV: `1, 1`), `0xc79a18` (U: `1, 0`), `0xc79d28` (V: `0, 1`).
#[doc(alias = "Ogre::TexCoordModifierControllerValue")]
#[derive(Clone, Debug)]
pub struct TexCoordModifierValue {
    /// Raw `TextureUnitState*` layer (IDA: stored without refcounting).
    pub layer: usize,
    pub translate_u: bool,
    pub translate_v: bool,
    pub scale_u: bool,
    pub scale_v: bool,
    pub rotate: bool,
}

/// was: `Ogre::ScaleControllerFunction` — `(factor, deltaInput)`.
/// IDA `0xc7976e`/`0xc79a32`/`0xc79d42`: factor is the *negated* speed
/// (`LODWORD(speed) ^ 0x80000000`), delta input `1`.
#[doc(alias = "Ogre::ScaleControllerFunction")]
#[derive(Clone, Debug)]
pub struct ScaleControllerFunction {
    pub factor: f32,
    pub delta_input: bool,
}

/// was: `Ogre::Controller<float>` (`0x38` bytes at IDA `0xc7977e`..`0xc79830`:
/// vtable, source `shared_ptr` (manager frame-time copy), destination
/// `shared_ptr`, function `shared_ptr`, enabled byte `1` at `+52`).
/// `boost`/`Ogre::SharedPtr` pieces map to `SharedPtr` (AGENTS.md §4).
#[doc(alias = "Ogre::Controller<float>")]
#[derive(Clone, Debug)]
pub struct TextureScroller {
    pub source: SharedPtr<FrameTimeValue>,
    pub dest: SharedPtr<TexCoordModifierValue>,
    pub func: SharedPtr<ScaleControllerFunction>,
    pub enabled: bool,
}

/// was: `Ogre::ControllerManager` — the scroller-relevant slice: the shared
/// frame-time source plus the owned controller set (`std::set` insert at
/// IDA line 113; original stores raw pointers, modelled shared).
#[doc(alias = "Ogre::ControllerManager")]
#[derive(Clone, Debug, Default)]
pub struct TextureScrollerManager {
    pub frame_time: SharedPtr<FrameTimeValue>,
    pub controllers: Vec<SharedPtr<TextureScroller>>,
}

impl TextureScrollerManager {
    /// Shared scroller core behind `createTextureUVScroller` /
    /// `createTextureUScroller` / `createTextureVScroller` (IDA
    /// `0xc7967c`/`0xc7998c`/`0xc79c9c`, 138 decompile lines each, identical
    /// except the `TexCoordModifier` translate flags): null when
    /// `speed == 0.0` (`v29 = 0`, `if (a3 != 0.0)` at `0xc796bc`..`0xc796d8`);
    /// otherwise news the dest value (`0x10` pool bytes), the function
    /// (`0x10` pool bytes, factor `-speed`), wires a `0x38`-byte controller
    /// to the manager's frame-time source + dest + func with enabled `1`,
    /// inserts it into the manager set, and returns it (`v29 = v30`).
    fn create_texture_scroller(
        &mut self,
        layer: usize,
        speed: f32,
        translate_u: bool,
        translate_v: bool,
    ) -> Option<SharedPtr<TextureScroller>> {
        // IDA 0xc796bc..0xc796d8: `v29 = 0; if (a3 != 0.0) { ... }`.
        if speed == 0.0 {
            return None;
        }
        // IDA 0xc796f4..0xc79708: dest = new TexCoordModifier(layer, u, v, 0, 0, 0).
        let dest = SharedPtr::new(TexCoordModifierValue {
            layer,
            translate_u,
            translate_v,
            scale_u: false,
            scale_v: false,
            rotate: false,
        });
        // IDA 0xc7976e: func = new ScaleControllerFunction(-speed, true).
        // The negation is `LODWORD(a3) ^ 0x80000000` — preserve it 1:1.
        let func = SharedPtr::new(ScaleControllerFunction {
            factor: -speed,
            delta_input: true,
        });
        // IDA 0xc7977e..0xc79830: controller{src = frame-time copy, dest, func, enabled = 1}.
        let controller = SharedPtr::new(TextureScroller {
            source: self.frame_time.clone(),
            dest,
            func,
            enabled: true,
        });
        // IDA line 113: insert into the manager controller set; `v29 = v30`.
        self.controllers.push(controller.clone());
        Some(controller)
    }
}

// 0xc7967c — __ZN4Ogre17ControllerManager23createTextureUVScrollerEPNS_16TextureUnitStateEf
#[doc(alias = "Ogre::ControllerManager::createTextureUVScroller(Ogre::TextureUnitState *,float)")]
// was: Ogre::ControllerManager::createTextureUVScroller(Ogre::TextureUnitState *,float)
// IDA 0xc7967c: `TexCoordModifier(layer, 1, 1, 0, 0, 0)` at `0xc79708`.
pub fn stub_c7967c(
    manager: &mut TextureScrollerManager,
    layer: usize,
    speed: f32,
) -> Option<SharedPtr<TextureScroller>> {
    manager.create_texture_scroller(layer, speed, true, true)
}

// 0xc7998c — __ZN4Ogre17ControllerManager22createTextureUScrollerEPNS_16TextureUnitStateEf
#[doc(alias = "Ogre::ControllerManager::createTextureUScroller(Ogre::TextureUnitState *,float)")]
// was: Ogre::ControllerManager::createTextureUScroller(Ogre::TextureUnitState *,float)
// IDA 0xc7998c: byte-identical to UV except
// `TexCoordModifier(layer, 1, 0, 0, 0, 0)` (verified by decompile diff).
pub fn stub_c7998c(
    manager: &mut TextureScrollerManager,
    layer: usize,
    speed: f32,
) -> Option<SharedPtr<TextureScroller>> {
    manager.create_texture_scroller(layer, speed, true, false)
}

// 0xc79c9c — __ZN4Ogre17ControllerManager22createTextureVScrollerEPNS_16TextureUnitStateEf
#[doc(alias = "Ogre::ControllerManager::createTextureVScroller(Ogre::TextureUnitState *,float)")]
// was: Ogre::ControllerManager::createTextureVScroller(Ogre::TextureUnitState *,float)
// IDA 0xc79c9c: byte-identical to UV except
// `TexCoordModifier(layer, 0, 1, 0, 0, 0)` (verified by decompile diff).
pub fn stub_c79c9c(
    manager: &mut TextureScrollerManager,
    layer: usize,
    speed: f32,
) -> Option<SharedPtr<TextureScroller>> {
    manager.create_texture_scroller(layer, speed, false, true)
}

/// was: `Ogre::DefaultHardwareVertexBuffer` — system-memory shadow of the
/// GPU vertex buffer. IDA `0xc7f2cc` reads the base pointer at `+0x38`.
#[doc(alias = "Ogre::DefaultHardwareVertexBuffer")]
#[derive(Clone, Debug, Default)]
pub struct DefaultHardwareVertexBuffer {
    /// System-memory copy (`mData`, `+0x38`).
    pub sys_memory: Vec<u8>,
}

// 0xc7f2cc — __ZN4Ogre27DefaultHardwareVertexBuffer8readDataEmmPv
#[doc(alias = "Ogre::DefaultHardwareVertexBuffer::readData(unsigned long,unsigned long,void *)")]
// was: Ogre::DefaultHardwareVertexBuffer::readData(unsigned long,unsigned long,void *)
// IDA 0xc7f2cc: `LDR R0,[R0,#0x38]; ADD R1,R0; MOV R0,R3; BLX memcpy`
// (decompile: `memcpy(dst, offset + *(this+14), length)`).
// FIDELITY: the original is an unchecked `memcpy`; Rust panics on OOB
// instead of over-reading/writing.
pub fn stub_c7f2cc(
    buffer: &DefaultHardwareVertexBuffer,
    offset: usize,
    length: usize,
    dst: &mut [u8],
) {
    // IDA 0xc7f2d2..0xc7f2d6: `dst[0..length] = base[offset..]`.
    dst[..length].copy_from_slice(&buffer.sys_memory[offset..offset + length]);
}

/// Adjustment applied by the `Thn188` non-virtual thunk (IDA `0xc85880`:
/// `SUBS R0, #0xBC`).
#[doc(alias = "non-virtual thunk toOgre::Entity::backgroundLoadingComplete(Ogre::Resource *)")]
pub const ENTITY_THUNK_ADJUST: usize = 188;

// 0xc85874 — __ZThn188_N4Ogre6Entity25backgroundLoadingCompleteEPNS_8ResourceE
#[doc(alias = "non-virtual thunk toOgre::Entity::backgroundLoadingComplete(Ogre::Resource *)")]
// was: non-virtual thunk to Ogre::Entity::backgroundLoadingComplete(Ogre::Resource *)
// IDA 0xc85874: `LDR R2,[R0,#8]; CMP R2,R1; IT NE; POPNE` — if the pending
// background-load resource word (`thunk_this+8`) differs from `resource`,
// return without doing anything (`0xc8587e`); else adjust `this` by `-188`
// (`0xc85880`) and call `Entity::_initialise(entity, false)`
// (`MOVS R1,#0` at `0xc85882`, `BL` at `0xc85884`).
// Returns the adjusted entity pointer when initialisation is required
// (the `_initialise` call itself lives at its own EA); `None` = no-op path.
pub fn stub_c85874(thunk_this: usize, pending: usize, resource: usize) -> Option<usize> {
    // IDA 0xc85876..0xc8587e: mismatch → return, no call.
    if pending != resource {
        return None;
    }
    // IDA 0xc85880..0xc85884: `Entity::_initialise(this - 188, false)`.
    Some(thunk_this - ENTITY_THUNK_ADJUST)
}

// ---- Import-stub (`__picsymbolstub4`) forwarders (IDA 0xf52374..0xf6ad84) ----
//
// Every address below disassembles to the 3-insn PIC stub sequence
// (`LDR R12, =ptr; ADD R12, PC, R12; LDR PC, [R12]`) tail-jumping to the
// named symbol. Ports model the *target* semantics with the AGENTS.md §4
// Boost→Rust mappings (`boost::function`/`bind` → closures,
// `boost::shared_ptr`/`weak_ptr` → `SharedPtr`/`Option`, `std::vector`
// insertion → `Vec::insert`); the stub itself adds no state change.

/// was: `boost::function<void,RBX::BillboardGui*,RBX::Adorn*>` — the chat
/// adorn callback built by the `bind_t`/`mf3` machinery below.
/// `boost::function` → `Option` of a boxed closure (AGENTS.md §4); the two
/// call arguments are opaque handles (`BillboardGui*`, `Adorn*`).
#[doc(alias = "boost::function<void,RBX::BillboardGui *,RBX::Adorn *>")]
pub struct BillboardAdornCallback {
    inner: Option<Box<dyn Fn(usize, usize) + Send + Sync>>,
}

impl BillboardAdornCallback {
    pub fn empty() -> Self {
        Self { inner: None }
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_none()
    }

    /// Invoke the stored target (`mf3::operator()` path, IDA `0xf52434`).
    pub fn invoke(&self, gui: usize, adorn: usize) -> bool {
        match &self.inner {
            Some(f) => {
                f(gui, adorn);
                true
            }
            None => false,
        }
    }

    /// Store a bound `ChatOutput` member-function target
    /// (`assign_to`/`assign_functor` path, IDA `0xf523a4`/`0xf52454`/
    /// `0xf52474`/`0xf52484`): the `bind_t` captures `ChatOutput*` by value,
    /// `arg<2>` routes the second call argument (`Adorn*`), and the
    /// `weak_ptr<Instance const>` / `weak_ptr<PartInstance>` are locked at
    /// call time (`None` = expired).
    pub fn store_chat_output_binding(
        &mut self,
        chat_output: usize,
        instance: Option<usize>,
        part: Option<usize>,
        handler: Box<dyn Fn(usize, usize, Option<usize>, Option<usize>) + Send + Sync>,
    ) {
        self.inner = Some(Box::new(move |gui: usize, adorn: usize| {
            handler(chat_output, adorn, instance, part);
            let _ = gui;
        }));
    }

    /// Clear the stored target (`function2::clear`, IDA `0xf52394`).
    pub fn clear(&mut self) {
        self.inner = None;
    }
}

// 0xf52374 — j___ZN5boost8functionIFvPN3RBX12BillboardGuiEPNS1_5AdornEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS1_10ChatOutputES5_NS_8weak_ptrIKNS1_8InstanceEEENSE_INS1_12PartInstanceEEEEENS9_5list4INS9_5valueIPSD_EENS_3argILi2EEENSM_ISH_EENSM_ISJ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISV_EE5valueEEE5valueEiE4typeE
#[doc(alias = "j___ZN5boost8functionIFvPN3RBX12BillboardGuiEPNS1_5AdornEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS1_10ChatOutputES5_NS_8weak_ptrIKNS1_8InstanceEEENSE_INS1_12PartInstanceEEEEENS9_5list4INS9_5valueIPSD_EENS_3argILi2EEENSM_ISH_EENSM_ISJ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISV_EE5valueEEE5valueEiE4typeE")]
// was: j___ZN5boost8functionIFvPN3RBX12BillboardGuiEPNS1_5AdornEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS1_10ChatOutputES5_NS_8weak_ptrIKNS1_8InstanceEEENSE_INS1_12PartInstanceEEEEENS9_5list4INS9_5valueIPSD_EENS_3argILi2EEENSM_ISH_EENSM_ISJ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISV_EE5valueEEE5valueEiE4typeE
// IDA 0xf52374: `__picsymbolstub4` forwarder to the `boost::function` ctor
// from the `ChatOutput`/`mf3` bind expression. Constructs the callback slot.
pub fn stub_f52374(
    chat_output: usize,
    instance: Option<usize>,
    part: Option<usize>,
    handler: Box<dyn Fn(usize, usize, Option<usize>, Option<usize>) + Send + Sync>,
) -> BillboardAdornCallback {
    let mut slot = BillboardAdornCallback::empty();
    slot.store_chat_output_binding(chat_output, instance, part, handler);
    slot
}

// 0xf52394 — j___ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEE5clearEv
#[doc(alias = "boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::clear(void)")]
// was: boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::clear(void)
// IDA 0xf52394: `__picsymbolstub4` forwarder to `clear` — empties the slot.
pub fn stub_f52394(slot: &mut BillboardAdornCallback) {
    slot.clear();
}

// 0xf523a4 — j___ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS1_10ChatOutputES5_NS_8weak_ptrIKNS1_8InstanceEEENSD_INS1_12PartInstanceEEEEENS8_5list4INS8_5valueIPSC_EENS_3argILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEvT_
#[doc(alias = "void boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ChatOutput,RBX::Adorn *,rbx_core::WeakPtr<RBX::Instance const>,rbx_core::WeakPtr<RBX::PartInstance>>,boost::_bi::list4<boost::_bi::value<RBX::ChatOutput*>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::Instance const>>,boost::_bi::value<rbx_core::WeakPtr<RBX::PartInstance>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ChatOutput,RBX::Adorn *,rbx_core::WeakPtr<RBX::Instance const>,rbx_core::WeakPtr<RBX::PartInstance>>,boost::_bi::list4<boost::_bi::value<RBX::ChatOutput*>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::Instance const>>,boost::_bi::value<rbx_core::WeakPtr<RBX::PartInstance>>>>)")]
// was: void boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ChatOutput,RBX::Adorn *,boost::weak_ptr<RBX::Instance const>,boost::weak_ptr<RBX::PartInstance>>,boost::_bi::list4<boost::_bi::value<RBX::ChatOutput*>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ChatOutput,RBX::Adorn *,boost::weak_ptr<RBX::Instance const>,boost::weak_ptr<RBX::PartInstance>>,boost::_bi::list4<boost::_bi::value<RBX::ChatOutput*>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>>>)
// IDA 0xf523a4: `__picsymbolstub4` forwarder to the `void assign_to` overload
// — stores the bind expression into an existing slot.
pub fn stub_f523a4(
    slot: &mut BillboardAdornCallback,
    chat_output: usize,
    instance: Option<usize>,
    part: Option<usize>,
    handler: Box<dyn Fn(usize, usize, Option<usize>, Option<usize>) + Send + Sync>,
) {
    slot.store_chat_output_binding(chat_output, instance, part, handler);
}

// 0xf523c4 — j___ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS1_10ChatOutputES5_NS_8weak_ptrIKNS1_8InstanceEEENSD_INS1_12PartInstanceEEEEENS8_5list4INS8_5valueIPSC_EENS_3argILi2EEENSL_ISG_EENSL_ISI_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE
#[doc(alias = "j___ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS1_10ChatOutputES5_NS_8weak_ptrIKNS1_8InstanceEEENSD_INS1_12PartInstanceEEEEENS8_5list4INS8_5valueIPSC_EENS_3argILi2EEENSL_ISG_EENSL_ISI_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")]
// was: j___ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS1_10ChatOutputES5_NS_8weak_ptrIKNS1_8InstanceEEENSD_INS1_12PartInstanceEEEEENS8_5list4INS8_5valueIPSC_EENS_3argILi2EEENSL_ISG_EENSL_ISI_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE
// IDA 0xf523c4: `__picsymbolstub4` forwarder to the `function2` ctor from the
// same bind shape as `0xf52374`. Same construction semantics.
pub fn stub_f523c4(
    chat_output: usize,
    instance: Option<usize>,
    part: Option<usize>,
    handler: Box<dyn Fn(usize, usize, Option<usize>, Option<usize>) + Send + Sync>,
) -> BillboardAdornCallback {
    stub_f52374(chat_output, instance, part, handler)
}

// 0xf52434 — j___ZNK5boost4_mfi3mf3IvN3RBX10ChatOutputEPNS2_5AdornENS_8weak_ptrIKNS2_8InstanceEEENS6_INS2_12PartInstanceEEEEclEPS3_S5_S9_SB_
#[doc(alias = "boost::_mfi::mf3<void,RBX::ChatOutput,RBX::Adorn *,rbx_core::WeakPtr<RBX::Instance const>,rbx_core::WeakPtr<RBX::PartInstance>>::operator()(RBX::ChatOutput*,RBX::Adorn *,rbx_core::WeakPtr<RBX::Instance const>,rbx_core::WeakPtr<RBX::PartInstance>)const")]
// was: boost::_mfi::mf3<void,RBX::ChatOutput,RBX::Adorn *,boost::weak_ptr<RBX::Instance const>,boost::weak_ptr<RBX::PartInstance>>::operator()(RBX::ChatOutput*,RBX::Adorn *,boost::weak_ptr<RBX::Instance const>,boost::weak_ptr<RBX::PartInstance>)const
// IDA 0xf52434: `__picsymbolstub4` forwarder to the bound-member invocation —
// calls `ChatOutput::method(chat, adorn, lockedInstance, lockedPart)`.
// The `weak_ptr` locks are modelled by the `Option` args (`None` = expired).
pub fn stub_f52434(
    handler: &dyn Fn(usize, usize, Option<usize>, Option<usize>),
    chat_output: usize,
    adorn: usize,
    instance: Option<usize>,
    part: Option<usize>,
) {
    handler(chat_output, adorn, instance, part);
}

// 0xf52454 — j___ZNK5boost6detail8function13basic_vtable2IvPN3RBX12BillboardGuiEPNS3_5AdornEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS3_10ChatOutputES7_NS_8weak_ptrIKNS3_8InstanceEEENSF_INS3_12PartInstanceEEEEENSA_5list4INSA_5valueIPSE_EENS_3argILi2EEENSN_ISI_EENSN_ISK_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable2<void,RBX::BillboardGui *,RBX::Adorn *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ChatOutput,RBX::Adorn *,rbx_core::WeakPtr<RBX::Instance const>,rbx_core::WeakPtr<RBX::PartInstance>>,boost::_bi::list4<boost::_bi::value<RBX::ChatOutput*>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::Instance const>>,boost::_bi::value<rbx_core::WeakPtr<RBX::PartInstance>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ChatOutput,RBX::Adorn *,rbx_core::WeakPtr<RBX::Instance const>,rbx_core::WeakPtr<RBX::PartInstance>>,boost::_bi::list4<boost::_bi::value<RBX::ChatOutput*>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::Instance const>>,boost::_bi::value<rbx_core::WeakPtr<RBX::PartInstance>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: void boost::detail::function::basic_vtable2<void,RBX::BillboardGui *,RBX::Adorn *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ChatOutput,RBX::Adorn *,boost::weak_ptr<RBX::Instance const>,boost::weak_ptr<RBX::PartInstance>>,boost::_bi::list4<boost::_bi::value<RBX::ChatOutput*>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ChatOutput,RBX::Adorn *,boost::weak_ptr<RBX::Instance const>,boost::weak_ptr<RBX::PartInstance>>,boost::_bi::list4<boost::_bi::value<RBX::ChatOutput*>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
// IDA 0xf52454: `__picsymbolstub4` forwarder to the heap (`bool_<false>`)
// functor-assignment path — stores the bind expression into the buffer.
pub fn stub_f52454(
    slot: &mut BillboardAdornCallback,
    chat_output: usize,
    instance: Option<usize>,
    part: Option<usize>,
    handler: Box<dyn Fn(usize, usize, Option<usize>, Option<usize>) + Send + Sync>,
) {
    slot.store_chat_output_binding(chat_output, instance, part, handler);
}

// 0xf52474 — j___ZNK5boost6detail8function13basic_vtable2IvPN3RBX12BillboardGuiEPNS3_5AdornEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS3_10ChatOutputES7_NS_8weak_ptrIKNS3_8InstanceEEENSF_INS3_12PartInstanceEEEEENSA_5list4INSA_5valueIPSE_EENS_3argILi2EEENSN_ISI_EENSN_ISK_EEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,RBX::BillboardGui *,RBX::Adorn *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ChatOutput,RBX::Adorn *,rbx_core::WeakPtr<RBX::Instance const>,rbx_core::WeakPtr<RBX::PartInstance>>,boost::_bi::list4<boost::_bi::value<RBX::ChatOutput*>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ChatOutput,RBX::Adorn *,rbx_core::WeakPtr<RBX::Instance const>,rbx_core::WeakPtr<RBX::PartInstance>>,boost::_bi::list4<boost::_bi::value<RBX::ChatOutput*>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable2<void,RBX::BillboardGui *,RBX::Adorn *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ChatOutput,RBX::Adorn *,boost::weak_ptr<RBX::Instance const>,boost::weak_ptr<RBX::PartInstance>>,boost::_bi::list4<boost::_bi::value<RBX::ChatOutput*>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ChatOutput,RBX::Adorn *,boost::weak_ptr<RBX::Instance const>,boost::weak_ptr<RBX::PartInstance>>,boost::_bi::list4<boost::_bi::value<RBX::ChatOutput*>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>>>,boost::detail::function::function_buffer &)const
// IDA 0xf52474: `__picsymbolstub4` forwarder to the bool `assign_to` overload
// — stores the bind expression; reports success. The store cannot fail here,
// so this returns `true` (the original returns nonzero on success).
pub fn stub_f52474(
    slot: &mut BillboardAdornCallback,
    chat_output: usize,
    instance: Option<usize>,
    part: Option<usize>,
    handler: Box<dyn Fn(usize, usize, Option<usize>, Option<usize>) + Send + Sync>,
) -> bool {
    slot.store_chat_output_binding(chat_output, instance, part, handler);
    true
}

// 0xf52484 — j___ZNK5boost6detail8function13basic_vtable2IvPN3RBX12BillboardGuiEPNS3_5AdornEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS3_10ChatOutputES7_NS_8weak_ptrIKNS3_8InstanceEEENSF_INS3_12PartInstanceEEEEENSA_5list4INSA_5valueIPSE_EENS_3argILi2EEENSN_ISI_EENSN_ISK_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,RBX::BillboardGui *,RBX::Adorn *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ChatOutput,RBX::Adorn *,rbx_core::WeakPtr<RBX::Instance const>,rbx_core::WeakPtr<RBX::PartInstance>>,boost::_bi::list4<boost::_bi::value<RBX::ChatOutput*>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ChatOutput,RBX::Adorn *,rbx_core::WeakPtr<RBX::Instance const>,rbx_core::WeakPtr<RBX::PartInstance>>,boost::_bi::list4<boost::_bi::value<RBX::ChatOutput*>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable2<void,RBX::BillboardGui *,RBX::Adorn *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ChatOutput,RBX::Adorn *,boost::weak_ptr<RBX::Instance const>,boost::weak_ptr<RBX::PartInstance>>,boost::_bi::list4<boost::_bi::value<RBX::ChatOutput*>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ChatOutput,RBX::Adorn *,boost::weak_ptr<RBX::Instance const>,boost::weak_ptr<RBX::PartInstance>>,boost::_bi::list4<boost::_bi::value<RBX::ChatOutput*>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
// IDA 0xf52484: `__picsymbolstub4` forwarder to the tagged (`function_obj_tag`)
// `assign_to` overload — same store-then-report-success semantics as `0xf52474`.
pub fn stub_f52484(
    slot: &mut BillboardAdornCallback,
    chat_output: usize,
    instance: Option<usize>,
    part: Option<usize>,
    handler: Box<dyn Fn(usize, usize, Option<usize>, Option<usize>) + Send + Sync>,
) -> bool {
    stub_f52474(slot, chat_output, instance, part, handler)
}

// 0xf5b1a4 — j___ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueIPN3RBX5AdornEEEEclIPFvNS_10shared_ptrINS5_8InstanceEEES7_ENS0_5list1IRKSD_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Adorn *>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Adorn *),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Adorn *) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
// was: void boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Adorn *>>::operator()<void (*)(boost::shared_ptr<RBX::Instance>,RBX::Adorn *),boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::Instance>,RBX::Adorn *) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)
// IDA 0xf5b1a4: `__picsymbolstub4` forwarder to the bind-argument applier:
// `arg<1>` forwards the caller's `SharedPtr<Instance>` and the stored
// `value<Adorn*>` supplies the second argument, then the function pointer is
// invoked. `boost::bind` apply → closure call (AGENTS.md §4).
pub fn stub_f5b1a4(
    target: &dyn Fn(usize, usize),
    instance: usize,
    adorn: usize,
) {
    target(instance, adorn);
}

/// was: `RBX::Adorn` handle for the destructor forwarder below. The real
/// `Adorn` object lives outside this crate; the handle keeps the teardown
/// target identifiable.
#[doc(alias = "RBX::Adorn")]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AdornHandle {
    pub target: usize,
}

// 0xf65004 — j___ZN3RBX5AdornD2Ev
#[doc(alias = "RBX::Adorn::~Adorn()")]
// was: RBX::Adorn::~Adorn()
// IDA 0xf65004: `__picsymbolstub4` forwarder to the `Adorn` destructor
// (decompile: `Adorn::~Adorn(this)`). Destructor/thunk glue: the C++
// member teardown maps to dropping the owned handle (AGENTS.md §4).
pub fn stub_f65004(handle: AdornHandle) {
    drop(handle);
}

// 0xf665f4 — j___ZNSt6vectorIPN3RBX12RenderEntityESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::RenderEntity *,std::allocator<RBX::RenderEntity *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::RenderEntity **,std::vector<RBX::RenderEntity *,std::allocator<RBX::RenderEntity *>>>,RBX::RenderEntity * const&)")]
// was: std::vector<RBX::RenderEntity *,std::allocator<RBX::RenderEntity *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::RenderEntity **,std::vector<RBX::RenderEntity *,std::allocator<RBX::RenderEntity *>>>,RBX::RenderEntity * const&)
// IDA 0xf665f4: `__picsymbolstub4` forwarder to `vector<RenderEntity*>`
// insertion (shift tail or realloc when full). Maps to `Vec::insert`.
pub fn stub_f665f4(items: &mut Vec<usize>, pos: usize, value: usize) {
    items.insert(pos, value);
}

/// was: `OgreRbxMutex` — Roblox's Ogre mutex wrapper. The real `C2` body
/// lives in the Ogre support library; this models the constructed state: the
/// integer flavor stored at construction plus the lock itself.
#[doc(alias = "OgreRbxMutex::OgreRbxMutex(int)")]
#[derive(Debug, Default)]
pub struct OgreRbxMutex {
    /// The `int` ctor argument (IDA `0xf67c14`).
    pub kind: i32,
    pub locked: parking_lot::Mutex<()>,
}

// 0xf67c14 — j___ZN12OgreRbxMutexC2Ei
#[doc(alias = "OgreRbxMutex::OgreRbxMutex(int)")]
// was: OgreRbxMutex::OgreRbxMutex(int)
// IDA 0xf67c14: `__picsymbolstub4` forwarder to the mutex ctor — records the
// flavor word and yields an unlocked mutex.
pub fn stub_f67c14(kind: i32) -> OgreRbxMutex {
    OgreRbxMutex {
        kind,
        locked: parking_lot::Mutex::new(()),
    }
}

// 0xf68fc4 — j___ZN5boost9unordered6detail16allocator_traitsISaINS1_8ptr_nodeISt4pairIKSsN4Ogre9SharedPtrINS6_8ResourceEEEEEEEE7destroyISA_EENS_12disable_if_cIXsr5boost9unordered6detail11has_destroyISC_T_EE5valueEvE4typeERSC_PSG_
#[doc(alias = "j___ZN5boost9unordered6detail16allocator_traitsISaINS1_8ptr_nodeISt4pairIKSsN4Ogre9SharedPtrINS6_8ResourceEEEEEEEE7destroyISA_EENS_12disable_if_cIXsr5boost9unordered6detail11has_destroyISC_T_EE5valueEvE4typeERSC_PSG_")]
// was: j___ZN5boost9unordered6detail16allocator_traitsISaINS1_8ptr_nodeISt4pairIKSsN4Ogre9SharedPtrINS6_8ResourceEEEEEEEE7destroyISA_EENS_12disable_if_cIXsr5boost9unordered6detail11has_destroyISC_T_EE5valueEvE4typeERSC_PSG_
// IDA 0xf68fc4: `__picsymbolstub4` forwarder to the `disable_if` (no
// `has_destroy`) `destroy` specialization for the
// `pair<string const, Ogre::SharedPtr<Resource>>` node — selected exactly
// when there is nothing to destroy, so the body is *correctly* empty.
// The `Ogre::SharedPtr` drop glue lives with the map itself.
pub fn stub_f68fc4() {}

// 0xf6ad04 — j___ZNK3RBX8Instance13visitChildrenIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_10GfxBindingERKNS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPS7_EENS2_3argILi1EEEEEEEEEvRKT_
#[doc(alias = "void RBX::Instance::visitChildren<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::GfxBinding,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<RBX::GfxBinding*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::GfxBinding,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<RBX::GfxBinding*>,boost::arg<1>>> const&)const")]
// was: void RBX::Instance::visitChildren<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::GfxBinding,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<RBX::GfxBinding*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::GfxBinding,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<RBX::GfxBinding*>,boost::arg<1>>> const&)const
// IDA 0xf6ad04: `__picsymbolstub4` forwarder to the child visitor — applies
// the bound `GfxBinding` member function (`mf1` with `value<GfxBinding*>` +
// `arg<1>`) to every child. `boost::bind` visitation → closure loop
// (AGENTS.md §4); children arrive as `SharedPtr<Instance>` handles.
pub fn stub_f6ad04(children: &[usize], visit: &dyn Fn(usize)) {
    for &child in children {
        visit(child);
    }
}

/// Shared `IndexArray<IAdornable, indexFunc*>::fastRemove` core behind IDA
/// `0xf6ad54`/`0xf6ad64`/`0xf6ad74` (the three differ only in the index
/// function template argument; removal mechanics are identical): swap-with-last
/// removal — order is *not* preserved. Returns whether `item` was present.
pub fn index_array_fast_remove(items: &mut Vec<usize>, item: usize) -> bool {
    match items.iter().position(|&x| x == item) {
        Some(pos) => {
            items.swap_remove(pos);
            true
        }
        None => false,
    }
}

// 0xf6ad54 — j___ZN3RBX10IndexArrayINS_10IAdornableEXadL_ZNS1_11indexFunc2dEvEEE10fastRemoveEPS1_
#[doc(alias = "RBX::IndexArray<RBX::IAdornable,&RBX::IAdornable::indexFunc2d>::fastRemove(RBX::IAdornable*)")]
// was: RBX::IndexArray<RBX::IAdornable,&RBX::IAdornable::indexFunc2d>::fastRemove(RBX::IAdornable*)
// IDA 0xf6ad54: `__picsymbolstub4` forwarder to the `indexFunc2d` removal.
pub fn stub_f6ad54(items: &mut Vec<usize>, item: usize) -> bool {
    index_array_fast_remove(items, item)
}

// 0xf6ad64 — j___ZN3RBX10IndexArrayINS_10IAdornableEXadL_ZNS1_11indexFunc3dEvEEE10fastRemoveEPS1_
#[doc(alias = "RBX::IndexArray<RBX::IAdornable,&RBX::IAdornable::indexFunc3d>::fastRemove(RBX::IAdornable*)")]
// was: RBX::IndexArray<RBX::IAdornable,&RBX::IAdornable::indexFunc3d>::fastRemove(RBX::IAdornable*)
// IDA 0xf6ad64: `__picsymbolstub4` forwarder to the `indexFunc3d` removal.
pub fn stub_f6ad64(items: &mut Vec<usize>, item: usize) -> bool {
    index_array_fast_remove(items, item)
}

// 0xf6ad74 — j___ZN3RBX10IndexArrayINS_10IAdornableEXadL_ZNS1_17indexFunc3dSortedEvEEE10fastRemoveEPS1_
#[doc(alias = "RBX::IndexArray<RBX::IAdornable,&RBX::IAdornable::indexFunc3dSorted>::fastRemove(RBX::IAdornable*)")]
// was: RBX::IndexArray<RBX::IAdornable,&RBX::IAdornable::indexFunc3dSorted>::fastRemove(RBX::IAdornable*)
// IDA 0xf6ad74: `__picsymbolstub4` forwarder to the `indexFunc3dSorted` removal.
pub fn stub_f6ad74(items: &mut Vec<usize>, item: usize) -> bool {
    index_array_fast_remove(items, item)
}

// 0xf6ad84 — j___ZNSt6vectorIPN3RBX10IAdornableESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,RBX::IAdornable * const&)")]
// was: std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,RBX::IAdornable * const&)
// IDA 0xf6ad84: `__picsymbolstub4` forwarder to `vector<IAdornable*>`
// insertion (shift tail or realloc when full). Maps to `Vec::insert`.
pub fn stub_f6ad84(items: &mut Vec<usize>, pos: usize, value: usize) {
    items.insert(pos, value);
}
