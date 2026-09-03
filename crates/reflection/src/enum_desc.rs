// RBX::Reflection::EnumDesc<T> decomp from IDA export.
// Models App/include/reflection/enumconverter.h: EnumDesc<T> over EnumDescriptor.
use std::collections::HashMap;

/// IDA 0x9b48: `ReleaseAssert((int)value<=2304)`, enumconverter.h:211.
pub const MAX_ENUM_VALUE: i32 = 2304;
/// Fill for unmapped value slots (original resizes vectors with -1). IDA 0x9b48.
const UNMAPPED: i32 = -1;

/// IDA 0x9b48: `EnumDescriptor::Item` (Descriptor base + owning desc, value, index).
#[derive(Debug, Clone)]
pub struct EnumItem {
    pub name: String,
    pub value: i32,
    pub index: usize,
}

/// `RBX::Reflection::EnumDesc<T>` item tables. IDA 0x850c.
#[derive(Debug, Clone, Default)]
pub struct EnumDesc {
    pub enum_name: &'static str,
    pub items: Vec<EnumItem>,
    pub value_to_value: Vec<i32>,
    pub value_ordinals: Vec<i32>,
    pub values: Vec<i32>,
    pub names: Vec<Option<String>>,
    pub items_by_value: Vec<Option<usize>>,
    pub name_to_value: HashMap<String, i32>,
    pub legacy_values: Vec<i32>,
    pub legacy_names: HashMap<String, i32>,
    pub count_bits: u32,
}

impl EnumDesc {
    /// IDA 0x850c: `EnumDescriptor::EnumDescriptor(this, name, typeinfo)`, vtable install, empty tables.
    pub fn new(enum_name: &'static str) -> Self {
        Self {
            enum_name,
            ..Default::default()
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// IDA 0x9b48: push Item, grow value-indexed vectors, map name->value, recount bits.
    pub fn add_pair(&mut self, value: i32, name: &str) {
        assert!(
            value >= 0,
            "value>=0 ../App/include/reflection/enumconverter.h:210"
        );
        assert!(
            value <= MAX_ENUM_VALUE,
            "(int)value<=2304 ../App/include/reflection/enumconverter.h:211"
        );
        let slot = value as usize;
        let index = self.items.len();
        self.items.push(EnumItem {
            name: name.to_owned(),
            value,
            index,
        });
        if self.value_to_value.len() <= slot {
            self.value_to_value.resize(slot + 1, UNMAPPED);
        }
        self.value_to_value[slot] = value;
        if self.value_ordinals.len() <= slot {
            self.value_ordinals.resize(slot + 1, UNMAPPED);
        }
        self.value_ordinals[slot] = index as i32;
        self.values.push(value);
        if self.names.len() <= slot {
            self.names.resize(slot + 1, None);
        }
        self.names[slot] = Some(name.to_owned());
        if self.items_by_value.len() <= slot {
            self.items_by_value.resize(slot + 1, None);
        }
        self.items_by_value[slot] = Some(index);
        self.name_to_value.insert(name.to_owned(), value);
        let count = self.items.len() as u32;
        self.count_bits = 31 - count.leading_zeros();
    }

    /// IDA 0xa208: grow legacy vector, map legacy name->value.
    pub fn add_legacy(&mut self, index: usize, name: &str, value: i32) {
        if self.legacy_values.len() <= index {
            self.legacy_values.resize(index + 1, UNMAPPED);
        }
        self.legacy_values[index] = value;
        self.legacy_names.insert(name.to_owned(), value);
    }

    pub fn lookup_value(&self, name: &str) -> Option<i32> {
        self.name_to_value
            .get(name)
            .copied()
            .or_else(|| self.legacy_names.get(name).copied())
    }

    pub fn lookup_name(&self, value: i32) -> Option<&str> {
        usize::try_from(value)
            .ok()
            .and_then(|slot| self.names.get(slot)?.as_deref())
    }
}

// 0x850c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEEC2Ev
// IDA 0x850c, vtable off_1221308
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::EnumDesc")]
pub fn enum_desc_crender_settings_aa_samples_ctor() -> EnumDesc {
    EnumDesc::new("AASamples")
}

// 0x86d0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEEC2Ev
// IDA 0x86d0, vtable off_1221338
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::EnumDesc")]
pub fn enum_desc_crender_settings_graphics_mode_ctor() -> EnumDesc {
    EnumDesc::new("GraphicsMode")
}

// 0x88c4 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEEC2Ev
// IDA 0x88c4, vtable off_1221368; note original string uses "FramerateManagerMode".
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::EnumDesc")]
pub fn enum_desc_crender_settings_frame_rate_manager_mode_ctor() -> EnumDesc {
    EnumDesc::new("FramerateManagerMode")
}

// 0x8a88 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEEC2Ev
// IDA 0x8a88, vtable off_1221398
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::EnumDesc")]
pub fn enum_desc_crender_settings_antialiasing_mode_ctor() -> EnumDesc {
    EnumDesc::new("Antialiasing")
}

// 0x8c4c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEEC2Ev
// IDA 0x8c4c, vtable off_12213C8
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::EnumDesc")]
pub fn enum_desc_crender_settings_shadow_mode_ctor() -> EnumDesc {
    EnumDesc::new("Shadow")
}
