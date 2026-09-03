//! reflection — generated_bg_1 — 100 stubs EA-sorted asc global gap filler 0x84e0..0x14e00 not yet in crates/reflection (global all covered, 64601 gaps reflection; 20945 distinct before, 21045 after)
//! Source: ida/export.json (85545 funcs) global EA asc not in crates/reflection/src — next 100 uncovered for reflection-bg sorted asc
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;
use rbx_core::signal::Signal;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

// 0x84e0 — start
// type: void __fastcall __noreturn(int, int, int, int, int argc, char *argv)
#[doc(alias = "start")]
pub fn stub_0x84e0() -> ! {
    todo!("0x84e0 start")
}

/// IDA 0x9608..0x9794: `CRenderSettingsItem` slots touched by this shard's leaves.
/// The original object is larger; only IDA-observed slots are modelled, with
/// byte offsets from the ARM disassembly noted per field.
#[derive(Default)]
pub struct CRenderSettingsItem {
    /// +0x64 dword. IDA 0x9608 `LDR R2,[R0,#0x64]` / `STR R1,[R0,#0x64]`.
    pub graphics_mode: i32,
    /// +0x68 dword. IDA 0x971c `LDR R2,[R0,#0x68]` / `STR R1,[R0,#0x68]`.
    pub antialiasing_mode: i32,
    /// +0x6C dword. IDA 0x96fc `LDR R2,[R0,#0x6C]` / `STR R1,[R0,#0x6C]`.
    pub shadow_mode: i32,
    /// +0x70 dword. IDA 0x9628 `LDR R2,[R0,#0x70]` / `STR R1,[R0,#0x70]`.
    pub frame_rate_manager_mode: i32,
    /// +0x74 dword. IDA 0x9648 `LDR R2,[R0,#0x74]` / `STR R1,[R0,#0x74]`.
    pub quality_level: i32,
    /// +0x88 byte. IDA 0x973c `LDRB.W R2,[R0,#0x88]` / `STRB.W R1,[R0,#0x88]`.
    pub debug_show_bounding_boxes: bool,
    /// +0x89 byte. IDA 0x9760 `LDRB.W R2,[R0,#0x89]` / `STRB.W R1,[R0,#0x89]`.
    pub enable_frm: bool,
    /// +0x9A byte. IDA 0x96ac `LDRB.W R2,[R0,#0x9A]` / `STRB.W R1,[R0,#0x9A]`.
    pub show_aggregation: bool,
    /// +0x9B byte: the stored `AlwaysDrawConnectors` value. IDA 0x9668 `STRB.W R1,[R0,#0x9B]`.
    pub always_draw_connectors: bool,
    /// +0x9C byte: second input of the 0x9668 effective-value compare.
    /// Role inferred from the compare logic (override clear => effective tracks this byte).
    pub always_draw_connectors_base: bool,
    /// +0xC0: `rbx::signals::signal_with_args<1, void(const PropertyDescriptor*)>`.
    /// Every setter below tail-calls it (`ADDS R0,#0xC0`) with its own
    /// `PropertyDescriptor` (`unk_130Cxxx`); modelled by descriptor name.
    pub property_changed: Signal<&'static str>,
}

/// IDA 0x96d0: `RBX::CRenderSettings::aaSamples` — a dword global, not an item
/// field (`LDR R2,[R2]; RBX::CRenderSettings::aaSamples` via `_ptr` slot).
pub static AA_SAMPLES: AtomicI32 = AtomicI32::new(0);
/// IDA 0x9784/0x9794: `RBX::PartInstance::disableInterpolation` — a byte global.
pub static DISABLE_INTERPOLATION: AtomicBool = AtomicBool::new(false);

/// IDA 0x9668: `LDRB` + `CBNZ`/`MOVNE` folds any nonzero flag byte to 1.
/// Fields here are already `bool`, so this documents the original fold.
fn normalize_flag(value: bool) -> i32 {
    i32::from(value)
}

// 0x9608 — __ZN19CRenderSettingsItem15setGraphicsModeEN3RBX15CRenderSettings12GraphicsModeE
// type: int __fastcall(int result, int)
// IDA 0x9608: store +0x64 then fire(+0xC0, &unk_130C244) iff changed; return this.
#[doc(alias = "CRenderSettingsItem::setGraphicsMode(RBX::CRenderSettings::GraphicsMode)")]
#[doc(alias = "__ZN19CRenderSettingsItem15setGraphicsModeEN3RBX15CRenderSettings12GraphicsModeE")]
pub fn stub_0x9608(this: *mut CRenderSettingsItem, value: i32) -> *mut CRenderSettingsItem {
    // SAFETY: `this` must point to a valid `CRenderSettingsItem`.
    unsafe {
        let item = &mut *this;
        if item.graphics_mode != value {
            item.graphics_mode = value;
            item.property_changed.fire("GraphicsMode");
        }
        this
    }
}

// 0x9628 — __ZN19CRenderSettingsItem23setFrameRateManagerModeEN3RBX15CRenderSettings20FrameRateManagerModeE
// type: int __fastcall(int result, int)
// IDA 0x9628: store +0x70 then fire(+0xC0, &unk_130C278) iff changed; return this.
#[doc(alias = "CRenderSettingsItem::setFrameRateManagerMode(RBX::CRenderSettings::FrameRateManagerMode)")]
#[doc(alias = "__ZN19CRenderSettingsItem23setFrameRateManagerModeEN3RBX15CRenderSettings20FrameRateManagerModeE")]
pub fn stub_0x9628(this: *mut CRenderSettingsItem, value: i32) -> *mut CRenderSettingsItem {
    // SAFETY: `this` must point to a valid `CRenderSettingsItem`.
    unsafe {
        let item = &mut *this;
        if item.frame_rate_manager_mode != value {
            item.frame_rate_manager_mode = value;
            item.property_changed.fire("FrameRateManagerMode");
        }
        this
    }
}

// 0x9648 — __ZN19CRenderSettingsItem15setQualityLevelEN3RBX15CRenderSettings12QualityLevelE
// type: int __fastcall(int result, int)
// IDA 0x9648: store +0x74 then fire(+0xC0, &unk_130C2AC) iff changed; return this.
#[doc(alias = "CRenderSettingsItem::setQualityLevel(RBX::CRenderSettings::QualityLevel)")]
#[doc(alias = "__ZN19CRenderSettingsItem15setQualityLevelEN3RBX15CRenderSettings12QualityLevelE")]
pub fn stub_0x9648(this: *mut CRenderSettingsItem, value: i32) -> *mut CRenderSettingsItem {
    // SAFETY: `this` must point to a valid `CRenderSettingsItem`.
    unsafe {
        let item = &mut *this;
        if item.quality_level != value {
            item.quality_level = value;
            item.property_changed.fire("QualityLevel");
        }
        this
    }
}

// 0x9668 — __ZN19CRenderSettingsItem23setAlwaysDrawConnectorsEb
// type: int __fastcall(int this, int)
// IDA 0x9668: effective = +0x9B ? 1 : normalize(+0x9C); store +0x9B, then fire
// (+0xC0, &unk_130C030) iff the effective value changed; return this.
#[doc(alias = "CRenderSettingsItem::setAlwaysDrawConnectors(bool)")]
#[doc(alias = "__ZN19CRenderSettingsItem23setAlwaysDrawConnectorsEb")]
pub fn stub_0x9668(this: *mut CRenderSettingsItem, value: bool) -> *mut CRenderSettingsItem {
    // SAFETY: `this` must point to a valid `CRenderSettingsItem`.
    unsafe {
        let item = &mut *this;
        let old_effective = if item.always_draw_connectors {
            1
        } else {
            normalize_flag(item.always_draw_connectors_base)
        };
        item.always_draw_connectors = value;
        if value {
            // IDA 0x9694: `CMP R2,#0` / `BXNE LR` — set override with a
            // previously nonzero effective value is a no-op signal-wise.
            if old_effective != 0 {
                return this;
            }
        } else {
            // IDA 0x968c: `TEQ.W R2,R1` / `BNE fire` — clearing the override
            // fires iff the base value differs from the old effective value.
            let new_effective = normalize_flag(item.always_draw_connectors_base);
            if old_effective == new_effective {
                return this;
            }
        }
        item.property_changed.fire("AlwaysDrawConnectors");
        this
    }
}

// 0x96ac — __ZN19CRenderSettingsItem18setShowAggregationEb
// type: int __fastcall(int this, int)
// IDA 0x96ac: store +0x9A then fire(+0xC0, &unk_130C05C) iff changed; return this.
#[doc(alias = "CRenderSettingsItem::setShowAggregation(bool)")]
#[doc(alias = "__ZN19CRenderSettingsItem18setShowAggregationEb")]
pub fn stub_0x96ac(this: *mut CRenderSettingsItem, value: bool) -> *mut CRenderSettingsItem {
    // SAFETY: `this` must point to a valid `CRenderSettingsItem`.
    unsafe {
        let item = &mut *this;
        if item.show_aggregation != value {
            item.show_aggregation = value;
            item.property_changed.fire("ShowAggregation");
        }
        this
    }
}

// 0x96d0 — __ZN19CRenderSettingsItem12setAASamplesEN3RBX15CRenderSettings9AASamplesE
// type: int __fastcall(int result, int)
// IDA 0x96d0: compares/stores the `RBX::CRenderSettings::aaSamples` GLOBAL
// (not an item field) but still fires the item's +0xC0 signal (&unk_130C2E0).
#[doc(alias = "CRenderSettingsItem::setAASamples(RBX::CRenderSettings::AASamples)")]
#[doc(alias = "__ZN19CRenderSettingsItem12setAASamplesEN3RBX15CRenderSettings9AASamplesE")]
pub fn stub_0x96d0(this: *mut CRenderSettingsItem, value: i32) -> *mut CRenderSettingsItem {
    if AA_SAMPLES.load(Ordering::SeqCst) != value {
        AA_SAMPLES.store(value, Ordering::SeqCst);
        // SAFETY: `this` must point to a valid `CRenderSettingsItem`.
        unsafe {
            (*this).property_changed.fire("AASamples");
        }
    }
    this
}

// 0x96fc — __ZN19CRenderSettingsItem13setShadowModeEN3RBX15CRenderSettings10ShadowModeE
// type: int __fastcall(int result, int)
// IDA 0x96fc: store +0x6C then fire(+0xC0, &unk_130C314) iff changed; return this.
#[doc(alias = "CRenderSettingsItem::setShadowMode(RBX::CRenderSettings::ShadowMode)")]
#[doc(alias = "__ZN19CRenderSettingsItem13setShadowModeEN3RBX15CRenderSettings10ShadowModeE")]
pub fn stub_0x96fc(this: *mut CRenderSettingsItem, value: i32) -> *mut CRenderSettingsItem {
    // SAFETY: `this` must point to a valid `CRenderSettingsItem`.
    unsafe {
        let item = &mut *this;
        if item.shadow_mode != value {
            item.shadow_mode = value;
            item.property_changed.fire("ShadowMode");
        }
        this
    }
}

// 0x971c — __ZN19CRenderSettingsItem19setAntialiasingModeEN3RBX15CRenderSettings16AntialiasingModeE
// type: int __fastcall(int result, int)
// IDA 0x971c: store +0x68 then fire(+0xC0, &unk_130C348) iff changed; return this.
#[doc(alias = "CRenderSettingsItem::setAntialiasingMode(RBX::CRenderSettings::AntialiasingMode)")]
#[doc(alias = "__ZN19CRenderSettingsItem19setAntialiasingModeEN3RBX15CRenderSettings16AntialiasingModeE")]
pub fn stub_0x971c(this: *mut CRenderSettingsItem, value: i32) -> *mut CRenderSettingsItem {
    // SAFETY: `this` must point to a valid `CRenderSettingsItem`.
    unsafe {
        let item = &mut *this;
        if item.antialiasing_mode != value {
            item.antialiasing_mode = value;
            item.property_changed.fire("AntialiasingMode");
        }
        this
    }
}

// 0x973c — __ZN19CRenderSettingsItem25setDebugShowBoundingBoxesEb
// type: int __fastcall(int this, int)
// IDA 0x973c: store +0x88 then fire(+0xC0, &unk_130C0E0) iff changed; return this.
#[doc(alias = "CRenderSettingsItem::setDebugShowBoundingBoxes(bool)")]
#[doc(alias = "__ZN19CRenderSettingsItem25setDebugShowBoundingBoxesEb")]
pub fn stub_0x973c(this: *mut CRenderSettingsItem, value: bool) -> *mut CRenderSettingsItem {
    // SAFETY: `this` must point to a valid `CRenderSettingsItem`.
    unsafe {
        let item = &mut *this;
        if item.debug_show_bounding_boxes != value {
            item.debug_show_bounding_boxes = value;
            item.property_changed.fire("DebugShowBoundingBoxes");
        }
        this
    }
}

// 0x9760 — __ZN19CRenderSettingsItem12setEnableFRMEb
// type: int __fastcall(int this, int)
// IDA 0x9760: store +0x89 then fire(+0xC0, &unk_130C138) iff changed; return this.
#[doc(alias = "CRenderSettingsItem::setEnableFRM(bool)")]
#[doc(alias = "__ZN19CRenderSettingsItem12setEnableFRMEb")]
pub fn stub_0x9760(this: *mut CRenderSettingsItem, value: bool) -> *mut CRenderSettingsItem {
    // SAFETY: `this` must point to a valid `CRenderSettingsItem`.
    unsafe {
        let item = &mut *this;
        if item.enable_frm != value {
            item.enable_frm = value;
            item.property_changed.fire("EnableFRM");
        }
        this
    }
}

// 0x9784 — __ZNK19CRenderSettingsItem28getDebugDisableInterpolationEv
// type: int __fastcall(CRenderSettingsItem *this)
// IDA 0x9784: ignores `this`; returns the `RBX::PartInstance::disableInterpolation` global byte.
#[doc(alias = "CRenderSettingsItem::getDebugDisableInterpolation(void)const")]
#[doc(alias = "__ZNK19CRenderSettingsItem28getDebugDisableInterpolationEv")]
pub fn stub_0x9784(this: *const CRenderSettingsItem) -> bool {
    let _ = this;
    DISABLE_INTERPOLATION.load(Ordering::SeqCst)
}

// 0x9794 — __ZN19CRenderSettingsItem28setDebugDisableInterpolationEb
// type: char *__fastcall(CRenderSettingsItem *this, char)
// IDA 0x9794: sets the `disableInterpolation` global and returns its address;
// no signal fires and `this` is unused.
#[doc(alias = "CRenderSettingsItem::setDebugDisableInterpolation(bool)")]
#[doc(alias = "__ZN19CRenderSettingsItem28setDebugDisableInterpolationEb")]
pub fn stub_0x9794(this: *mut CRenderSettingsItem, value: bool) -> *mut bool {
    let _ = this;
    DISABLE_INTERPOLATION.store(value, Ordering::SeqCst);
    DISABLE_INTERPOLATION.as_ptr()
}

// 0x97a4 — __ZN19CRenderSettingsItem23setResolutionPreferenceEN3RBX15CRenderSettings16ResolutionPresetE
// type: int __fastcall(int result, int)
#[doc(alias = "CRenderSettingsItem::setResolutionPreference(RBX::CRenderSettings::ResolutionPreset)")]
#[doc(alias = "__ZN19CRenderSettingsItem23setResolutionPreferenceEN3RBX15CRenderSettings16ResolutionPresetE")]
pub fn stub_0x97a4() -> ! {
    todo!("0x97a4 CRenderSettingsItem::setResolutionPreference(RBX::CRenderSettings::ResolutionPreset)")
}

// 0x97c0 — __ZN19CRenderSettingsItem19setTextureCacheSizeEj
// type: int __fastcall(int this, unsigned int)
#[doc(alias = "CRenderSettingsItem::setTextureCacheSize(unsigned int)")]
#[doc(alias = "__ZN19CRenderSettingsItem19setTextureCacheSizeEj")]
pub fn stub_0x97c0() -> ! {
    todo!("0x97c0 CRenderSettingsItem::setTextureCacheSize(unsigned int)")
}

// 0x97c8 — __ZN19CRenderSettingsItem16setMeshCacheSizeEj
// type: int __fastcall(int this, unsigned int)
#[doc(alias = "CRenderSettingsItem::setMeshCacheSize(unsigned int)")]
#[doc(alias = "__ZN19CRenderSettingsItem16setMeshCacheSizeEj")]
pub fn stub_0x97c8() -> ! {
    todo!("0x97c8 CRenderSettingsItem::setMeshCacheSize(unsigned int)")
}

// 0x97d0 — __ZN19CRenderSettingsItemC2Ev
// type: void __fastcall(CRenderSettingsItem *this)
#[doc(alias = "CRenderSettingsItem::CRenderSettingsItem(void)")]
#[doc(alias = "__ZN19CRenderSettingsItemC2Ev")]
pub fn stub_0x97d0() -> ! {
    todo!("0x97d0 CRenderSettingsItem::CRenderSettingsItem(void)")
}

// 0x9ac8 — __ZN19CRenderSettingsItem19setAutoQualityLevelEi
// type: int __fastcall(int this, int)
#[doc(alias = "CRenderSettingsItem::setAutoQualityLevel(int)")]
#[doc(alias = "__ZN19CRenderSettingsItem19setAutoQualityLevelEi")]
pub fn stub_0x9ac8() -> ! {
    todo!("0x9ac8 CRenderSettingsItem::setAutoQualityLevel(int)")
}

// 0x9ae8 — __ZThn96_N19CRenderSettingsItem19setAutoQualityLevelEi
// type: int __fastcall(int this, int)
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::setAutoQualityLevel(int)")]
#[doc(alias = "__ZThn96_N19CRenderSettingsItem19setAutoQualityLevelEi")]
pub fn stub_0x9ae8() -> ! {
    todo!("0x9ae8 non-virtual thunk toCRenderSettingsItem::setAutoQualityLevel(int)")
}

// 0x9b08 — __ZN19CRenderSettingsItem21setEagerBulkExecutionEb
// type: int __fastcall(int this, int)
#[doc(alias = "CRenderSettingsItem::setEagerBulkExecution(bool)")]
#[doc(alias = "__ZN19CRenderSettingsItem21setEagerBulkExecutionEb")]
pub fn stub_0x9b08() -> ! {
    todo!("0x9b08 CRenderSettingsItem::setEagerBulkExecution(bool)")
}

// 0x9b2c — __ZNSt12length_errorD1Ev
// type: void __cdecl(std::length_error *__hidden this)
#[doc(alias = "std::length_error::~length_error()")]
#[doc(alias = "__ZNSt12length_errorD1Ev")]
pub fn stub_0x9b2c() -> ! {
    todo!("0x9b2c std::length_error::~length_error()")
}

// 0x9b30 — __ZNSt12out_of_rangeD0Ev
// type: void __cdecl(std::out_of_range *__hidden this)
#[doc(alias = "std::out_of_range::~out_of_range()")]
#[doc(alias = "__ZNSt12out_of_rangeD0Ev")]
pub fn stub_0x9b30() -> ! {
    todo!("0x9b30 std::out_of_range::~out_of_range()")
}

// 0x9b44 — __ZNSt12out_of_rangeD2Ev
// type: void __cdecl(std::out_of_range *__hidden this)
#[doc(alias = "std::out_of_range::~out_of_range()")]
#[doc(alias = "__ZNSt12out_of_rangeD2Ev")]
pub fn stub_0x9b44() -> ! {
    todo!("0x9b44 std::out_of_range::~out_of_range()")
}

// 0xb33c — __ZNK3RBX15CRenderSettings15getGraphicsModeEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getGraphicsMode(void)const")]
#[doc(alias = "__ZNK3RBX15CRenderSettings15getGraphicsModeEv")]
pub fn stub_0xb33c() -> ! {
    todo!("0xb33c RBX::CRenderSettings::getGraphicsMode(void)const")
}

// 0xb364 — __ZNK3RBX15CRenderSettings23getFrameRateManagerModeEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getFrameRateManagerMode(void)const")]
#[doc(alias = "__ZNK3RBX15CRenderSettings23getFrameRateManagerModeEv")]
pub fn stub_0xb364() -> ! {
    todo!("0xb364 RBX::CRenderSettings::getFrameRateManagerMode(void)const")
}

// 0xb38c — __ZNK3RBX15CRenderSettings15getQualityLevelEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getQualityLevel(void)const")]
#[doc(alias = "__ZNK3RBX15CRenderSettings15getQualityLevelEv")]
pub fn stub_0xb38c() -> ! {
    todo!("0xb38c RBX::CRenderSettings::getQualityLevel(void)const")
}

// 0xb3b4 — __ZNK3RBX15CRenderSettings23getAlwaysDrawConnectorsEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getAlwaysDrawConnectors(void)const")]
#[doc(alias = "__ZNK3RBX15CRenderSettings23getAlwaysDrawConnectorsEv")]
pub fn stub_0xb3b4() -> ! {
    todo!("0xb3b4 RBX::CRenderSettings::getAlwaysDrawConnectors(void)const")
}

// 0xb3e0 — __ZNK3RBX15CRenderSettings18getShowAggregationEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getShowAggregation(void)const")]
#[doc(alias = "__ZNK3RBX15CRenderSettings18getShowAggregationEv")]
pub fn stub_0xb3e0() -> ! {
    todo!("0xb3e0 RBX::CRenderSettings::getShowAggregation(void)const")
}

// 0xb3e8 — __ZNK3RBX15CRenderSettings12getAASamplesEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getAASamples(void)const")]
#[doc(alias = "__ZNK3RBX15CRenderSettings12getAASamplesEv")]
pub fn stub_0xb3e8() -> ! {
    todo!("0xb3e8 RBX::CRenderSettings::getAASamples(void)const")
}

// 0xb41c — __ZNK3RBX15CRenderSettings13getShadowModeEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getShadowMode(void)const")]
#[doc(alias = "__ZNK3RBX15CRenderSettings13getShadowModeEv")]
pub fn stub_0xb41c() -> ! {
    todo!("0xb41c RBX::CRenderSettings::getShadowMode(void)const")
}

// 0xb444 — __ZNK3RBX15CRenderSettings19getAntialiasingModeEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getAntialiasingMode(void)const")]
#[doc(alias = "__ZNK3RBX15CRenderSettings19getAntialiasingModeEv")]
pub fn stub_0xb444() -> ! {
    todo!("0xb444 RBX::CRenderSettings::getAntialiasingMode(void)const")
}

// 0xb46c — __ZNK3RBX15CRenderSettings25getDebugShowBoundingBoxesEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getDebugShowBoundingBoxes(void)const")]
#[doc(alias = "__ZNK3RBX15CRenderSettings25getDebugShowBoundingBoxesEv")]
pub fn stub_0xb46c() -> ! {
    todo!("0xb46c RBX::CRenderSettings::getDebugShowBoundingBoxes(void)const")
}

// 0xb474 — __ZNK3RBX15CRenderSettings19getAutoQualityLevelEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getAutoQualityLevel(void)const")]
#[doc(alias = "__ZNK3RBX15CRenderSettings19getAutoQualityLevelEv")]
pub fn stub_0xb474() -> ! {
    todo!("0xb474 RBX::CRenderSettings::getAutoQualityLevel(void)const")
}

// 0xb49c — __ZNK3RBX15CRenderSettings12getEnableFRMEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getEnableFRM(void)const")]
#[doc(alias = "__ZNK3RBX15CRenderSettings12getEnableFRMEv")]
pub fn stub_0xb49c() -> ! {
    todo!("0xb49c RBX::CRenderSettings::getEnableFRM(void)const")
}

// 0xb4a4 — __ZNK3RBX15CRenderSettings23getResolutionPreferenceEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getResolutionPreference(void)const")]
#[doc(alias = "__ZNK3RBX15CRenderSettings23getResolutionPreferenceEv")]
pub fn stub_0xb4a4() -> ! {
    todo!("0xb4a4 RBX::CRenderSettings::getResolutionPreference(void)const")
}

// 0xb4cc — __ZN3RBX15CRenderSettings18getMaxQualityLevelEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getMaxQualityLevel(void)")]
#[doc(alias = "__ZN3RBX15CRenderSettings18getMaxQualityLevelEv")]
pub fn stub_0xb4cc() -> ! {
    todo!("0xb4cc RBX::CRenderSettings::getMaxQualityLevel(void)")
}

// 0xb4f4 — __ZNK3RBX15CRenderSettings19getTextureCacheSizeEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getTextureCacheSize(void)const")]
#[doc(alias = "__ZNK3RBX15CRenderSettings19getTextureCacheSizeEv")]
pub fn stub_0xb4f4() -> ! {
    todo!("0xb4f4 RBX::CRenderSettings::getTextureCacheSize(void)const")
}

// 0xb4f8 — __ZNK3RBX15CRenderSettings16getMeshCacheSizeEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getMeshCacheSize(void)const")]
#[doc(alias = "__ZNK3RBX15CRenderSettings16getMeshCacheSizeEv")]
pub fn stub_0xb4f8() -> ! {
    todo!("0xb4f8 RBX::CRenderSettings::getMeshCacheSize(void)const")
}

// 0xb4fc — __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEEC2Ev
// type: RBX::Instance *__fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEEC2Ev")]
pub fn stub_0xb4fc() -> ! {
    todo!("0xb4fc __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEEC2Ev")
}

// 0xb740 — __ZNSt6vectorIN3G3D12Vector2int16ESaIS1_EE9push_backERKS1_
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::push_back(G3D::Vector2int16 const&)")]
#[doc(alias = "__ZNSt6vectorIN3G3D12Vector2int16ESaIS1_EE9push_backERKS1_")]
pub fn stub_0xb740() -> ! {
    todo!("0xb740 std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::push_back(G3D::Vector2int16 const&)")
}

// 0xb8b0 — __ZNK3RBX15CRenderSettings21getEagerBulkExecutionEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getEagerBulkExecution(void)const")]
#[doc(alias = "__ZNK3RBX15CRenderSettings21getEagerBulkExecutionEv")]
pub fn stub_0xb8b0() -> ! {
    todo!("0xb8b0 RBX::CRenderSettings::getEagerBulkExecution(void)const")
}

// 0xb8b8 — __ZN19CRenderSettingsItemD1Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "CRenderSettingsItem::~CRenderSettingsItem()")]
#[doc(alias = "__ZN19CRenderSettingsItemD1Ev")]
pub fn stub_0xb8b8() -> ! {
    todo!("0xb8b8 CRenderSettingsItem::~CRenderSettingsItem()")
}

// 0xb8bc — __ZN19CRenderSettingsItemD0Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "CRenderSettingsItem::~CRenderSettingsItem()")]
#[doc(alias = "__ZN19CRenderSettingsItemD0Ev")]
pub fn stub_0xb8bc() -> ! {
    todo!("0xb8bc CRenderSettingsItem::~CRenderSettingsItem()")
}

// 0xb8d0 — __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv")]
pub fn stub_0xb8d0() -> ! {
    todo!("0xb8d0 __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv")
}

// 0xb8e0 — __ZThn32_N19CRenderSettingsItemD1Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
#[doc(alias = "__ZThn32_N19CRenderSettingsItemD1Ev")]
pub fn stub_0xb8e0() -> ! {
    todo!("0xb8e0 non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")
}

// 0xb8e8 — __ZThn32_N19CRenderSettingsItemD0Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
#[doc(alias = "__ZThn32_N19CRenderSettingsItemD0Ev")]
pub fn stub_0xb8e8() -> ! {
    todo!("0xb8e8 non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")
}

// 0xb900 — __ZThn32_NK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv")]
pub fn stub_0xb900() -> ! {
    todo!("0xb900 __ZThn32_NK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv")
}

// 0xb910 — __ZThn36_N19CRenderSettingsItemD1Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
#[doc(alias = "__ZThn36_N19CRenderSettingsItemD1Ev")]
pub fn stub_0xb910() -> ! {
    todo!("0xb910 non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")
}

// 0xb918 — __ZThn36_N19CRenderSettingsItemD0Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
#[doc(alias = "__ZThn36_N19CRenderSettingsItemD0Ev")]
pub fn stub_0xb918() -> ! {
    todo!("0xb918 non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")
}

// 0xb930 — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD1Ev
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0xb930() -> ! {
    todo!("0xb930 __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD1Ev")
}

// 0xcb94 — __ZN5boost16exception_detail12refcount_ptrINS0_20error_info_containerEED2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "boost::exception_detail::refcount_ptr<boost::exception_detail::error_info_container>::~refcount_ptr()")]
#[doc(alias = "__ZN5boost16exception_detail12refcount_ptrINS0_20error_info_containerEED2Ev")]
pub fn stub_0xcb94() -> ! {
    todo!("0xcb94 boost::exception_detail::refcount_ptr<boost::exception_detail::error_info_container>::~refcount_ptr()")
}

// 0xeccc — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD2Ev
// type: int __fastcall(int)
#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0xeccc() -> ! {
    todo!("0xeccc __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD2Ev")
}

// 0xedfc — __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0xedfc() -> ! {
    todo!("0xedfc __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator12getClassNameEv")
}

// 0xee84 — __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator6createEv
// type: int __fastcall(int *)
#[doc(alias = "__ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator6createEv")]
pub fn stub_0xee84() -> ! {
    todo!("0xee84 __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator6createEv")
}

// 0xef04 — __ZN3RBX9CreatableINS_8InstanceEE6createI19CRenderSettingsItemEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<CRenderSettingsItem> RBX::Creatable<RBX::Instance>::create<CRenderSettingsItem>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createI19CRenderSettingsItemEEN5boost10shared_ptrIT_EEv")]
pub fn stub_0xef04() -> ! {
    todo!("0xef04 boost::shared_ptr<CRenderSettingsItem> RBX::Creatable<RBX::Instance>::create<CRenderSettingsItem>(void)")
}

// 0xefb4 — __ZN5boost10shared_ptrI19CRenderSettingsItemEC2IS1_N3RBX9CreatableINS4_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<CRenderSettingsItem>::shared_ptr<CRenderSettingsItem,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrI19CRenderSettingsItemEC2IS1_N3RBX9CreatableINS4_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_0xefb4() -> ! {
    todo!("0xefb4 boost::shared_ptr<CRenderSettingsItem>::shared_ptr<CRenderSettingsItem,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xefd8 — __ZNK5boost6detail15sp_counted_base9use_countEv
// type: int __fastcall(boost::detail::sp_counted_base *this)
#[doc(alias = "boost::detail::sp_counted_base::use_count(void)const")]
#[doc(alias = "__ZNK5boost6detail15sp_counted_base9use_countEv")]
pub fn stub_0xefd8() -> ! {
    todo!("0xefd8 boost::detail::sp_counted_base::use_count(void)const")
}

// 0xf098 — __ZN5boost6detail12shared_countC2IP19CRenderSettingsItemN3RBX9CreatableINS5_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IP19CRenderSettingsItemN3RBX9CreatableINS5_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_0xf098() -> ! {
    todo!("0xf098 boost::detail::shared_count::shared_count<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf198 — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0xf198() -> ! {
    todo!("0xf198 boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0xf19c — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0xf19c() -> ! {
    todo!("0xf19c boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0xf1bc — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0xf1bc() -> ! {
    todo!("0xf1bc boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0xf1d4 — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_0xf1d4() -> ! {
    todo!("0xf1d4 boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0xf1d8 — __ZN3RBX4Name13callDoDeclareILZ15sRenderSettingsEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZ15sRenderSettingsEEEvv")]
pub fn stub_0xf1d8() -> ! {
    todo!("0xf1d8 __ZN3RBX4Name13callDoDeclareILZ15sRenderSettingsEEEvv")
}

// 0xf1dc — __ZN3RBX4Name9doDeclareILZ15sRenderSettingsEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZ15sRenderSettingsEEERKS0_v")]
pub fn stub_0xf1dc() -> ! {
    todo!("0xf1dc __ZN3RBX4Name9doDeclareILZ15sRenderSettingsEEERKS0_v")
}

// 0xf2bc — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0xf2bc() -> ! {
    todo!("0xf2bc __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorC2Ev")
}

// 0xf500 — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0xf500() -> ! {
    todo!("0xf500 __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE17static_getCreatorEv")
}

// 0xf704 — __ZNSt6vectorIN3G3D12Vector2int16ESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: int __fastcall(int, char *, _DWORD *)
#[doc(alias = "std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::_M_insert_aux(__gnu_cxx::__normal_iterator<G3D::Vector2int16*,std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>>,G3D::Vector2int16 const&)")]
#[doc(alias = "__ZNSt6vectorIN3G3D12Vector2int16ESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")]
pub fn stub_0xf704() -> ! {
    todo!("0xf704 std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::_M_insert_aux(__gnu_cxx::__normal_iterator<G3D::Vector2int16*,std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>>,G3D::Vector2int16 const&)")
}

// 0xf7e8 — __ZNSt12_Vector_baseIN3G3D12Vector2int16ESaIS1_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3G3D12Vector2int16ESaIS1_EE11_M_allocateEm")]
pub fn stub_0xf7e8() -> ! {
    todo!("0xf7e8 std::_Vector_base<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::_M_allocate(unsigned long)")
}

// 0xf800 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D12Vector2int16ES5_EET0_T_S7_S6_
// type: int __fastcall(int, int, int)
#[doc(alias = "G3D::Vector2int16 * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Vector2int16 *,G3D::Vector2int16 *>(G3D::Vector2int16 *,G3D::Vector2int16 *,G3D::Vector2int16 *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D12Vector2int16ES5_EET0_T_S7_S6_")]
pub fn stub_0xf800() -> ! {
    todo!("0xf800 G3D::Vector2int16 * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Vector2int16 *,G3D::Vector2int16 *>(G3D::Vector2int16 *,G3D::Vector2int16 *,G3D::Vector2int16 *)")
}

// 0xf83c — __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev")]
pub fn stub_0xf83c() -> ! {
    todo!("0xf83c __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev")
}

// 0xf87c — __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev
// type: int __fastcall(int)
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev")]
pub fn stub_0xf87c() -> ! {
    todo!("0xf87c __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev")
}

// 0xf8c8 — __ZThn32_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev
// type: void __fastcall(_QWORD *)
#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev")]
pub fn stub_0xf8c8() -> ! {
    todo!("0xf8c8 __ZThn32_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev")
}

// 0xf90c — __ZThn32_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev
// type: int __fastcall(_QWORD *)
#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev")]
pub fn stub_0xf90c() -> ! {
    todo!("0xf90c __ZThn32_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev")
}

// 0xf964 — __ZThn36_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev")]
pub fn stub_0xf964() -> ! {
    todo!("0xf964 __ZThn36_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev")
}

// 0xf9a8 — __ZThn36_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev
// type: int __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev")]
pub fn stub_0xf9a8() -> ! {
    todo!("0xf9a8 __ZThn36_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev")
}

// 0x142b8 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings16ResolutionPresetESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: _Rb_tree_node_base **__fastcall(int, int *)
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::ResolutionPreset,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings16ResolutionPresetESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
pub fn stub_0x142b8() -> ! {
    todo!("0x142b8 std::map<RBX::Name const*,RBX::CRenderSettings::ResolutionPreset,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::operator[](RBX::Name const* const&)")
}

// 0x14310 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
pub fn stub_0x14310() -> ! {
    todo!("0x14310 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::C")
}

// 0x143c4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
pub fn stub_0x143c4() -> ! {
    todo!("0x143c4 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Na")
}

// 0x1441c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
pub fn stub_0x1441c() -> ! {
    todo!("0x1441c std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::Resolut")
}

// 0x14484 — __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE6resizeEmS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::resize(unsigned long,RBX::CRenderSettings::ResolutionPreset)")]
#[doc(alias = "__ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE6resizeEmS2_")]
pub fn stub_0x14484() -> ! {
    todo!("0x14484 std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::resize(unsigned long,RBX::CRenderSettings::ResolutionPreset)")
}

// 0x144b8 — __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE9push_backERKS2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::push_back(RBX::CRenderSettings::ResolutionPreset const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE9push_backERKS2_")]
pub fn stub_0x144b8() -> ! {
    todo!("0x144b8 std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::push_back(RBX::CRenderSettings::ResolutionPreset const&)")
}

// 0x144e0 — __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ResolutionPreset*,std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>>,RBX::CRenderSettings::ResolutionPreset const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_0x144e0() -> ! {
    todo!("0x144e0 std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ResolutionPreset*,std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>>,RBX::CRenderSettings::ResolutionPreset const&)")
}

// 0x145c4 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE11_M_allocateEm")]
pub fn stub_0x145c4() -> ! {
    todo!("0x145c4 std::_Vector_base<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_allocate(unsigned long)")
}

// 0x145dc — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings16ResolutionPresetES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::CRenderSettings::ResolutionPreset * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *>(RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings16ResolutionPresetES6_EET0_T_S8_S7_")]
pub fn stub_0x145dc() -> ! {
    todo!("0x145dc RBX::CRenderSettings::ResolutionPreset * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *>(RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *)")
}

// 0x14618 — __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ResolutionPreset*,std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>>,unsigned long,RBX::CRenderSettings::ResolutionPreset const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_0x14618() -> ! {
    todo!("0x14618 std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ResolutionPreset*,std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>>,unsigned long,RBX::CRenderSettings::ResolutionPreset const&)")
}

// 0x147a8 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings12QualityLevelESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::QualityLevel,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings12QualityLevelESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
pub fn stub_0x147a8() -> ! {
    todo!("0x147a8 std::map<RBX::Name const*,RBX::CRenderSettings::QualityLevel,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::operator[](RBX::Name const* const&)")
}

// 0x14800 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
pub fn stub_0x14800() -> ! {
    todo!("0x14800 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettin")
}

// 0x148b4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
pub fn stub_0x148b4() -> ! {
    todo!("0x148b4 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* co")
}

// 0x1490c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
pub fn stub_0x1490c() -> ! {
    todo!("0x1490c std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const")
}

// 0x14974 — __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE6resizeEmS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::resize(unsigned long,RBX::CRenderSettings::QualityLevel)")]
#[doc(alias = "__ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE6resizeEmS2_")]
pub fn stub_0x14974() -> ! {
    todo!("0x14974 std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::resize(unsigned long,RBX::CRenderSettings::QualityLevel)")
}

// 0x149a8 — __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE9push_backERKS2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::push_back(RBX::CRenderSettings::QualityLevel const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE9push_backERKS2_")]
pub fn stub_0x149a8() -> ! {
    todo!("0x149a8 std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::push_back(RBX::CRenderSettings::QualityLevel const&)")
}

// 0x149d0 — __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::QualityLevel*,std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>>,RBX::CRenderSettings::QualityLevel const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_0x149d0() -> ! {
    todo!("0x149d0 std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::QualityLevel*,std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>>,RBX::CRenderSettings::QualityLevel const&)")
}

// 0x14ab4 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings12QualityLevelESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX15CRenderSettings12QualityLevelESaIS2_EE11_M_allocateEm")]
pub fn stub_0x14ab4() -> ! {
    todo!("0x14ab4 std::_Vector_base<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_allocate(unsigned long)")
}

// 0x14acc — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings12QualityLevelES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::CRenderSettings::QualityLevel * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *>(RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings12QualityLevelES6_EET0_T_S8_S7_")]
pub fn stub_0x14acc() -> ! {
    todo!("0x14acc RBX::CRenderSettings::QualityLevel * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *>(RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *)")
}

// 0x14b08 — __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::QualityLevel*,std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>>,unsigned long,RBX::CRenderSettings::QualityLevel const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_0x14b08() -> ! {
    todo!("0x14b08 std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::QualityLevel*,std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>>,unsigned long,RBX::CRenderSettings::QualityLevel const&)")
}

// 0x14c98 — __ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE6resizeEmS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::resize(unsigned long,RBX::CRenderSettings::ShadowMode)")]
#[doc(alias = "__ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE6resizeEmS2_")]
pub fn stub_0x14c98() -> ! {
    todo!("0x14c98 std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::resize(unsigned long,RBX::CRenderSettings::ShadowMode)")
}

// 0x14ccc — __ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE9push_backERKS2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::push_back(RBX::CRenderSettings::ShadowMode const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE9push_backERKS2_")]
pub fn stub_0x14ccc() -> ! {
    todo!("0x14ccc std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::push_back(RBX::CRenderSettings::ShadowMode const&)")
}

// 0x14cf4 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings10ShadowModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::ShadowMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings10ShadowModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
pub fn stub_0x14cf4() -> ! {
    todo!("0x14cf4 std::map<RBX::Name const*,RBX::CRenderSettings::ShadowMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::operator[](RBX::Name const* const&)")
}

// 0x14d4c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
pub fn stub_0x14d4c() -> ! {
    todo!("0x14d4c std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::Sh")
}

// 0x14e00 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
pub fn stub_0x14e00() -> ! {
    todo!("0x14e00 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RB")
}

#[cfg(test)]
mod render_settings_item_tests {
    use super::*;
    use parking_lot::Mutex;
    use std::sync::Arc;

    fn connect_log(item: &CRenderSettingsItem) -> (Arc<Mutex<Vec<&'static str>>>, Arc<impl Fn(&'static str) + Send + Sync>) {
        let log = Arc::new(Mutex::new(Vec::new()));
        let inner = Arc::clone(&log);
        let slot = Arc::new(move |name: &'static str| {
            inner.lock().push(name);
        });
        item.property_changed.connect(slot.clone());
        // `Signal` holds only a weak ref: the returned `Arc` keeps the slot live.
        (log, slot)
    }

    #[test]
    fn dword_setter_stores_and_fires_once_on_change() {
        let mut item = CRenderSettingsItem::default();
        let (log, _slot) = connect_log(&item);
        let this = &mut item as *mut CRenderSettingsItem;
        unsafe {
            assert_eq!(stub_0x9608(this, 2), this);
            assert_eq!((*this).graphics_mode, 2);
            // Same value: no store, no signal (IDA 0x960c `IT EQ` / `BXEQ LR`).
            assert_eq!(stub_0x9608(this, 2), this);
            assert_eq!(stub_0x971c(this, 1), this);
            assert_eq!((*this).antialiasing_mode, 1);
        }
        assert_eq!(*log.lock(), vec!["GraphicsMode", "AntialiasingMode"]);
    }

    #[test]
    fn byte_setters_fire_with_own_descriptor_only_on_change() {
        let mut item = CRenderSettingsItem::default();
        let (log, _slot) = connect_log(&item);
        let this = &mut item as *mut CRenderSettingsItem;
        unsafe {
            stub_0x973c(this, true);
            stub_0x973c(this, true);
            stub_0x9760(this, true);
            stub_0x96ac(this, true);
            assert!((*this).debug_show_bounding_boxes);
            assert!((*this).enable_frm);
            assert!((*this).show_aggregation);
        }
        assert_eq!(
            *log.lock(),
            vec!["DebugShowBoundingBoxes", "EnableFRM", "ShowAggregation"]
        );
    }

    #[test]
    fn always_draw_connectors_fires_on_effective_change_only() {
        let mut item = CRenderSettingsItem::default();
        let (log, _slot) = connect_log(&item);
        let this = &mut item as *mut CRenderSettingsItem;
        unsafe {
            // Old effective 0 -> fires; override now set so repeat is silent.
            stub_0x9668(this, true);
            stub_0x9668(this, true);
            // Clearing with base 0: effective 1 -> 0 fires; repeat silent.
            stub_0x9668(this, false);
            stub_0x9668(this, false);
            assert!(!(*this).always_draw_connectors);
        }
        assert_eq!(*log.lock(), vec!["AlwaysDrawConnectors", "AlwaysDrawConnectors"]);
    }

    #[test]
    fn aa_samples_uses_global_but_fires_item_signal() {
        let mut item = CRenderSettingsItem::default();
        let (log, _slot) = connect_log(&item);
        let this = &mut item as *mut CRenderSettingsItem;
        unsafe {
            stub_0x96d0(this, 4);
            assert_eq!(AA_SAMPLES.load(Ordering::SeqCst), 4);
            stub_0x96d0(this, 4);
            // Restore the shared global so sibling tests see the default.
            AA_SAMPLES.store(0, Ordering::SeqCst);
        }
        assert_eq!(*log.lock(), vec!["AASamples"]);
    }

    #[test]
    fn disable_interpolation_round_trips_through_global() {
        let mut item = CRenderSettingsItem::default();
        let this = &mut item as *mut CRenderSettingsItem;
        unsafe {
            let addr = stub_0x9794(this, true);
            assert!(stub_0x9784(this));
            assert!(!addr.is_null());
            assert_eq!(addr, DISABLE_INTERPOLATION.as_ptr());
            stub_0x9794(this, false);
            assert!(!stub_0x9784(this));
        }
    }
}
