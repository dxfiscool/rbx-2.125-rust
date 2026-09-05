//! audio generated_audio_wd_watchdog19 — 100 stubs EA-sorted asc gap filler not yet in audio (FMOD|Sound|Audio exhausted, global gap filler)
//! Source: ida/export.json (85545 funcs) EA-sorted asc next 100 not in audio after 0x0662800 | rbx_core::SharedPtr not boost
//! Range 0x662824..0x66606c | existing 36802 -> 36902 distinct
//! Batch: 100 stubs | // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR mangled")

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
use std::sync::atomic::{AtomicU32, Ordering};
use crate::generated_audio_wd_watchdog13::SpawnPlayerRef;
use crate::generated::flog_asserts;
use crate::generated_134::{XmlIntSlot, XmlReadValue};
use crate::generated_audio_wd_watchdog17::{NORMAL_ID_ITEMS, SurfaceSelectionState, normal_id_name};
use crate::generated_audio_wd_watchdog18::TextBoxState;
const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };


/// Player entry for the `Teams` queries needing character linkage
/// (IDA 0x664cb0): neutrality/team-color plus whether the player's
/// character contains the queried humanoid (the humanoid pointer
/// compare folds into the flag; null characters fold into false).
#[derive(Debug, Clone, Copy)]
pub struct PlayerTeamRef {
    pub neutral: bool,
    pub team_color: u32,
    pub owns_humanoid: bool,
}
/// `RBX::Team` cutover (IDA 0x662fcc): the score at +92 (word 23,
/// init 0), the team color at +96 (word 24, init 194 overwritten to
/// 1 at 0x6630fc), the auto-assignable flag at +100 (init 1,
/// 0x6630b6) and the auto-color-characters flag at +101 (init 1,
/// 0x6630c6). The `Instance`/`Described` bases and the
/// `setName("Team")` fold away.
#[derive(Debug, Clone)]
pub struct TeamState {
    pub score: i32,
    pub team_color: u32,
    pub auto_assignable: bool,
    pub auto_color_characters: bool,
}
/// `RBX::Reflection::BoundProp<bool>` cutover for `Team` (IDA
/// 0x663b74, `AutoColorCharacters`): name/category plus the live
/// value. The member cell (offset 101 = 0x65, IDA a_268 0x664374)
/// folds into the field (same shape as `SparklesBoolProp` at
/// 0x63cc8c).
#[derive(Debug, Clone)]
pub struct TeamBoolProp {
    pub name: String,
    pub category: String,
    pub attributes: u32,
    pub permissions: u32,
    pub value: bool,
}
impl TeamBoolProp {
    pub fn new(
        name: &str,
        category: &str,
        initial: bool,
        attributes: u32,
        permissions: u32,
    ) -> Self {
        Self {
            name: name.to_owned(),
            category: category.to_owned(),
            attributes,
            permissions,
            value: initial,
        }
    }
}
/// `RBX::Reflection::PropDescriptor<Team, _>` cutover (IDA 0x663d68
/// bool `AutoAssignable`, 0x663ef8 `BrickColor` `TeamColor`,
/// 0x66408c int `Score`, all IDA a_268 0x6642ac-0x664346):
/// name/category/attributes/permissions. The getter/setter
/// member-pointer pairs fold into direct field access.
#[derive(Debug, Clone)]
pub struct TeamPropDesc {
    pub name: String,
    pub category: String,
    pub attributes: u32,
    pub permissions: u32,
}
impl TeamPropDesc {
    pub fn new(name: &str, category: &str, attributes: u32, permissions: u32) -> Self {
        Self {
            name: name.to_owned(),
            category: category.to_owned(),
            attributes,
            permissions,
        }
    }
}
/// `RBX::Reflection::BoundFuncDesc<Teams, void()>` cutover (IDA
/// 0x665780): the bound member name. The single object binds
/// `rebalanceTeams` ("RebalanceTeams", IDA a_269 0x6659a6-0x6659ca);
/// the member pair folds into the name.
#[derive(Debug, Clone)]
pub struct TeamsVoidFuncDesc {
    pub name: String,
}
impl TeamsVoidFuncDesc {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }
}
/// `RBX::Teams` cutover (IDA 0x6645dc): the +92 flag (init 1) plus
/// the team list at +96 (copy_on_write vector, init empty). The
/// name registration folds away (same shape as `SpawnerServiceState`
/// at 0x63db8c).
#[derive(Debug, Clone, Default)]
pub struct TeamsState {
    pub flag_92: bool,
    pub teams: Vec<SharedPtr<TeamState>>,
}
/// Process-wide static-init run count behind the `__GLOBAL__I_a_*`
/// ctors in this file (IDA 0x662c48). The category/ios/descriptor/
/// pool/guard stores fold into host statics (initialized on use),
/// so only the run is recorded.
static WATCHDOG19_STATIC_INITS: AtomicU32 = AtomicU32::new(0);
/// Records one `__GLOBAL__I_a_*` run in this file.
pub fn watchdog19_static_init() {
    WATCHDOG19_STATIC_INITS.fetch_add(1, Ordering::SeqCst);
}
/// Returns the recorded static-init run count (test hook).
pub fn watchdog19_static_inits() -> u32 {
    WATCHDOG19_STATIC_INITS.load(Ordering::SeqCst)
}
// 0x662824 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE14setStringValueEPNS0_13DescribedBaseERKSs
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE14setStringValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_662824(state: &mut SurfaceSelectionState, name: &str) -> bool {
    // IDA 0x662824 (`EnumPropDescriptor<NormalId>::setStringValue`):
    // `Name::lookup` + `EnumDesc::convertToValue`; on a hit the
    // inner `setValue` runs and 1 returns, else 0. Same shape as
    // the `InputType` twin at 0x659820. Host: table position
    // decides.
    match NORMAL_ID_ITEMS.iter().position(|(n, _)| *n == name) {
        Some(index) => {
            state.surface = NORMAL_ID_ITEMS[index].1;
            true
        }
        None => false,
    }
}

// 0x662864 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
pub fn stub_662864(state: &SurfaceSelectionState, out: &mut XmlIntSlot) -> i32 {
    // IDA 0x662864 (`EnumPropDescriptor<NormalId>::writeValue`):
    // inner `getValue`, `clearValue`, int tag `5` at +16, value at
    // +20, returns 5. Same shape as the `InputType` twin at
    // 0x659884.
    out.value_type = 0;
    out.value_type = 5;
    out.int_value = state.surface as i32;
    5
}

// 0x662884 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_662884(state: &mut SurfaceSelectionState, xml: &XmlReadValue) {
    // IDA 0x662884 (`EnumPropDescriptor<NormalId>::readValue`):
    // xsi:nil early-out (0x6628a8); an int pair runs `setIntValue`
    // (index→value with -1 rejection, 0x6628f0-0x662900, host:
    // stub_662bbc) and returns on success; a string pair runs
    // lookup + convert + inner set (0x662908-0x66296a), a miss
    // falling through; anything else hits `ReleaseAssert(false)`
    // (Reflection.h line 359, host seam). Unlike the 0x6598a4
    // shape, the int path here is validated, not raw.
    match xml {
        XmlReadValue::Nil => {}
        XmlReadValue::Int(value) => {
            if stub_662bbc(state, *value) {
                return;
            }
            if flog_asserts() {
                panic!("false file: include/Reflection/Reflection.h line: 359 (IDA 0x662884)");
            }
        }
        XmlReadValue::Text(text) => {
            if stub_662824(state, text) {
                return;
            }
            if flog_asserts() {
                panic!("false file: include/Reflection/Reflection.h line: 359 (IDA 0x662884)");
            }
        }
        XmlReadValue::Other => {
            if flog_asserts() {
                panic!("false file: include/Reflection/Reflection.h line: 359 (IDA 0x662884)");
            }
        }
    }
}

// 0x662ac4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE13getIndexValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::getIndexValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE13getIndexValueEPKNS0_13DescribedBaseE")]
pub fn stub_662ac4(state: &SurfaceSelectionState) -> i32 {
    // IDA 0x662ac4 (`EnumPropDescriptor<NormalId>::getIndexValue`):
    // inner `getValue` + `EnumDesc::convertToIndex`. Host: the item
    // index of the live face.
    NORMAL_ID_ITEMS
        .iter()
        .position(|(_, v)| *v == state.surface)
        .map(|i| i as i32)
        .unwrap_or(-1)
}

// 0x662ae0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE13setIndexValueEPNS0_13DescribedBaseEm
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE13setIndexValueEPNS0_13DescribedBaseEm")]
pub fn stub_662ae0(state: &mut SurfaceSelectionState, index: u32) -> bool {
    // IDA 0x662ae0 (`EnumPropDescriptor<NormalId>::setIndexValue`):
    // `count > index` (0x662af2) gates reading the indexed item's
    // value (0x662afc) and storing it via the inner `setValue`,
    // returning 1 (else 0). Host: table read decides.
    match NORMAL_ID_ITEMS.get(index as usize) {
        Some((_, value)) => {
            state.surface = *value;
            true
        }
        None => false,
    }
}

// 0x662b14 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE12getEnumValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::getEnumValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE12getEnumValueEPKNS0_13DescribedBaseE")]
pub fn stub_662b14(state: &SurfaceSelectionState) -> u32 {
    // IDA 0x662b14 (`EnumPropDescriptor<NormalId>::getEnumValue`):
    // inner `getValue` through the +44 `GetSet`. Host: the live
    // face.
    state.surface
}

// 0x662b1c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE12setEnumValueEPNS0_13DescribedBaseEi
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::setEnumValue(RBX::Reflection::DescribedBase *,int)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE12setEnumValueEPNS0_13DescribedBaseEi")]
pub fn stub_662b1c(state: &mut SurfaceSelectionState, value: u32) -> bool {
    // IDA 0x662b1c (`EnumPropDescriptor<NormalId>::setEnumValue`):
    // `find_if` with `equalValue(value)` over the items; found
    // stores via the inner `setValue` and returns 1, else 0.
    // Host: table membership decides.
    if NORMAL_ID_ITEMS.iter().any(|(_, v)| *v == value) {
        state.surface = value;
        true
    } else {
        false
    }
}

// 0x662b68 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE11getEnumItemEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::getEnumItem(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE11getEnumItemEPKNS0_13DescribedBaseE")]
pub fn stub_662b68(state: &SurfaceSelectionState) -> Option<u32> {
    // IDA 0x662b68 (`EnumPropDescriptor<NormalId>::getEnumItem`):
    // inner `getValue` + `EnumDesc::convertToItem`. Host: the item
    // index (`None` when absent).
    NORMAL_ID_ITEMS
        .iter()
        .position(|(_, v)| *v == state.surface)
        .map(|i| i as u32)
}

// 0x662b88 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
pub fn stub_662b88(state: &mut SurfaceSelectionState, name: &str) -> bool {
    // IDA 0x662b88 (`EnumPropDescriptor<NormalId>::setStringValue(Name)`):
    // `EnumDesc::convertToValue(name)`; hit stores via the inner
    // `setValue` and returns 1, else 0. Host: the string twin in
    // this file.
    stub_662824(state, name)
}

// 0x662bbc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE11setIntValueEPNS0_13DescribedBaseEi
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::setIntValue(RBX::Reflection::DescribedBase *,int)const
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_662bbc(state: &mut SurfaceSelectionState, raw: i32) -> bool {
    // IDA 0x662bbc (`EnumPropDescriptor<NormalId>::setIntValue`):
    // `raw >= 0` and in the `enumToItem`-shaped map with a non--1
    // value gates the inner `setValue`, returning 1 (else 0). The
    // map is dense identity for `NormalId`, so the index reads the
    // table. Same shape as the `Font` twin at 0x66fc30.
    if raw >= 0 {
        if let Some((_, value)) = NORMAL_ID_ITEMS.get(raw as usize) {
            state.surface = *value;
            return true;
        }
    }
    false
}

// 0x662bfc — __ZNK3RBX10Reflection14PropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::GetSetImpl<RBX::NormalId (RBX::SurfaceSelection::*)(void)const,void (RBX::SurfaceSelection::*)(RBX::NormalId)>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::GetSetImpl<RBX::NormalId (RBX::SurfaceSelection::*)(void)const,void (RBX::SurfaceSelection::*)(RBX::NormalId)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv")]
pub fn stub_662bfc() -> bool {
    // IDA 0x662bfc (`GetSetImpl<NormalId>::isReadOnly`): `MOVS R0,
    // #0; BX LR` — always readable.
    false
}

// 0x662c00 — __ZNK3RBX10Reflection14PropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::GetSetImpl<RBX::NormalId (RBX::SurfaceSelection::*)(void)const,void (RBX::SurfaceSelection::*)(RBX::NormalId)>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::GetSetImpl<RBX::NormalId (RBX::SurfaceSelection::*)(void)const,void (RBX::SurfaceSelection::*)(RBX::NormalId)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv")]
pub fn stub_662c00() -> bool {
    // IDA 0x662c00 (`GetSetImpl<NormalId>::isWriteOnly`): `MOVS R0,
    // #0; BX LR` — always writable.
    false
}

// 0x662c04 — __ZNK3RBX10Reflection14PropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::GetSetImpl<RBX::NormalId (RBX::SurfaceSelection::*)(void)const,void (RBX::SurfaceSelection::*)(RBX::NormalId)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::GetSetImpl<RBX::NormalId (RBX::SurfaceSelection::*)(void)const,void (RBX::SurfaceSelection::*)(RBX::NormalId)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_662c04(state: &SurfaceSelectionState) -> u32 {
    // IDA 0x662c04 (`GetSetImpl<NormalId>::getValue`): the
    // member-pointer resolve tail-calling the getter. The member is
    // `getSurface` (IDA 0x660be0, the only `NormalId` getter on
    // `SurfaceSelection`); the pointer folds into the field.
    state.surface
}

// 0x662c24 — __ZNK3RBX10Reflection14PropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// demangled: RBX::Reflection::PropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::GetSetImpl<RBX::NormalId (RBX::SurfaceSelection::*)(void)const,void (RBX::SurfaceSelection::*)(RBX::NormalId)>::setValue(RBX::Reflection::DescribedBase *,RBX::NormalId const&)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SurfaceSelection,RBX::NormalId>::GetSetImpl<RBX::NormalId (RBX::SurfaceSelection::*)(void)const,void (RBX::SurfaceSelection::*)(RBX::NormalId)>::setValue(RBX::Reflection::DescribedBase *,RBX::NormalId const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_16SurfaceSelectionENS_8NormalIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_")]
pub fn stub_662c24(state: &mut SurfaceSelectionState, value: u32) {
    // IDA 0x662c24 (`GetSetImpl<NormalId>::setValue`): the
    // member-pointer resolve tail-calling the setter with the input
    // word. The member is `setSurface` (IDA 0x660890); the pointer
    // folds into the field.
    state.surface = value;
}

// 0x662c48 — __GLOBAL__I_a_267
// demangled: global constructor keyed to_a_267
#[doc(alias = "global constructor keyed to_a_267")]
#[doc(alias = "__GLOBAL__I_a_267")]
pub fn stub_662c48() {
    // IDA 0x662c48 (`__GLOBAL__I_a_267`): `generic_category` x2 +
    // `system_category` stores (0x662c52-0x662c6c),
    // `ios_base::Init` + `__cxa_atexit` (0x662c6e-0x662c92), the
    // `SurfaceSelection` `TargetSurface`/`Data`
    // `EnumPropDescriptor` (getter `getSurface`, setter
    // `setSurface`, 0x662c96-0x662cd6), the `boost::exception`
    // statics, the `singleton_pool` guards and the `Camera` +
    // `SurfaceSelection` creators (0x662d10-0x662f36). Host statics
    // initialize on use; only the run is recorded.
    watchdog19_static_init();
}

// 0x662f5c — __ZNK3RBX4Team8getScoreEv
// demangled: RBX::Team::getScore(void)const
// type: _DWORD __fastcall(RBX::Team *__hidden this)
#[doc(alias = "RBX::Team::getScore(void)const")]
#[doc(alias = "__ZNK3RBX4Team8getScoreEv")]
pub fn stub_662f5c(state: &TeamState) -> i32 {
    // IDA 0x662f5c (`RBX::Team::getScore`): loads word 23 (+92,
    // 0x662f5e). Host: direct field read.
    state.score
}

// 0x662f60 — __ZN3RBX4Team8setScoreEi
// demangled: RBX::Team::setScore(int)
// type: _DWORD __fastcall(RBX::Team *__hidden this, int)
#[doc(alias = "RBX::Team::setScore(int)")]
#[doc(alias = "__ZN3RBX4Team8setScoreEi")]
pub fn stub_662f60(state: &mut TeamState, score: i32) -> bool {
    // IDA 0x662f60 (`RBX::Team::setScore`): compares word 23
    // (0x662f64); on change stores it (0x662f70) and raises
    // `raisePropertyChanged` (0x662f78), else returns unchanged
    // (0x662f66). The raise folds into the changed flag (same shape
    // as `StudioTool::setEnabled` at 0x65793c).
    if state.score == score {
        return false;
    }
    state.score = score;
    true
}

// 0x662f7c — __ZNK3RBX4Team12getTeamColorEv
// demangled: RBX::Team::getTeamColor(void)const
// type: _DWORD __fastcall(RBX::Team *__hidden this)
#[doc(alias = "RBX::Team::getTeamColor(void)const")]
#[doc(alias = "__ZNK3RBX4Team12getTeamColorEv")]
pub fn stub_662f7c(state: &TeamState) -> u32 {
    // IDA 0x662f7c (`RBX::Team::getTeamColor`): copies word 24 (+96,
    // 0x662f7e) into the hidden return slot. Host: direct field
    // read.
    state.team_color
}

// 0x662f84 — __ZN3RBX4Team12setTeamColorENS_10BrickColorE
// demangled: RBX::Team::setTeamColor(RBX::BrickColor)
#[doc(alias = "RBX::Team::setTeamColor(RBX::BrickColor)")]
#[doc(alias = "__ZN3RBX4Team12setTeamColorENS_10BrickColorE")]
pub fn stub_662f84(state: &mut TeamState, team_color: u32) -> bool {
    // IDA 0x662f84 (`RBX::Team::setTeamColor`): compares word 24
    // (0x662f88); on change stores it (0x662f94) and raises
    // `raisePropertyChanged` (0x662f9c), else returns unchanged
    // (0x662f8a). The raise folds into the changed flag.
    if state.team_color == team_color {
        return false;
    }
    state.team_color = team_color;
    true
}

// 0x662fa0 — __ZNK3RBX4Team17getAutoAssignableEv
// demangled: RBX::Team::getAutoAssignable(void)const
// type: _DWORD __fastcall(RBX::Team *__hidden this)
#[doc(alias = "RBX::Team::getAutoAssignable(void)const")]
#[doc(alias = "__ZNK3RBX4Team17getAutoAssignableEv")]
pub fn stub_662fa0(state: &TeamState) -> bool {
    // IDA 0x662fa0 (`RBX::Team::getAutoAssignable`): loads the +100
    // flag byte (0x662fa4). Host: direct field read.
    state.auto_assignable
}

// 0x662fa8 — __ZN3RBX4Team17setAutoAssignableEb
// demangled: RBX::Team::setAutoAssignable(bool)
// type: _DWORD __fastcall(RBX::Team *__hidden this, bool)
#[doc(alias = "RBX::Team::setAutoAssignable(bool)")]
#[doc(alias = "__ZN3RBX4Team17setAutoAssignableEb")]
pub fn stub_662fa8(state: &mut TeamState, auto_assignable: bool) -> bool {
    // IDA 0x662fa8 (`RBX::Team::setAutoAssignable`): compares +100
    // (0x662fae); on change stores it (0x662fba) and raises
    // `raisePropertyChanged` (0x662fc4), else returns unchanged
    // (0x662fb0). The raise folds into the changed flag.
    if state.auto_assignable == auto_assignable {
        return false;
    }
    state.auto_assignable = auto_assignable;
    true
}

// 0x662fc8 — __ZN3RBX4TeamC1Ev
// demangled: RBX::Team::Team(void)
// type: _DWORD __fastcall(RBX::Team *__hidden this)
#[doc(alias = "RBX::Team::Team(void)")]
#[doc(alias = "__ZN3RBX4TeamC1Ev")]
pub fn stub_662fc8() -> TeamState {
    // IDA 0x662fc8 (`RBX::Team::Team` C1): thunk tail-calling the C2
    // (host: stub_662fcc).
    stub_662fcc()
}

// 0x662fcc — __ZN3RBX4TeamC2Ev
// demangled: RBX::Team::Team(void)
// type: _DWORD __fastcall(RBX::Team *__hidden this)
#[doc(alias = "RBX::Team::Team(void)")]
#[doc(alias = "__ZN3RBX4TeamC2Ev")]
pub fn stub_662fcc() -> TeamState {
    // IDA 0x662fcc (`RBX::Team::Team` C2): `Instance` base + vtable
    // installs + class registration (0x662fee-0x663088); word 23
    // (+92, score) = 0 (0x6630ac); word 24 (+96, team color) = 194
    // then overwritten to 1 (0x6630b2 then 0x6630fc); +100
    // (auto-assignable) = 1 (0x6630b6) and +101
    // (auto-color-characters) = 1 (0x6630c6);
    // `setName("Team")` (0x6630d2-0x6630de). Host: the grounded
    // cutover.
    TeamState {
        score: 0,
        team_color: 1,
        auto_assignable: true,
        auto_color_characters: true,
    }
}

// 0x6631e0 — __ZN3RBX4TeamD0Ev
// demangled: RBX::Team::~Team()
// type: void __fastcall(RBX::Team *__hidden this)
#[doc(alias = "RBX::Team::~Team()")]
#[doc(alias = "__ZN3RBX4TeamD0Ev")]
pub fn stub_6631e0() {
    // IDA 0x6631e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x663280 — __ZN3RBX4TeamD1Ev
// demangled: RBX::Team::~Team()
// type: void __fastcall(RBX::Team *__hidden this)
#[doc(alias = "RBX::Team::~Team()")]
#[doc(alias = "__ZN3RBX4TeamD1Ev")]
pub fn stub_663280() {
    // IDA 0x663280: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x663284 — __ZThn32_N3RBX4TeamD0Ev
// demangled: non-virtual thunk toRBX::Team::~Team()
// type: void __fastcall(RBX::Team *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Team::~Team()")]
#[doc(alias = "__ZThn32_N3RBX4TeamD0Ev")]
pub fn stub_663284() {
    // IDA 0x663284: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x66328c — __ZThn36_N3RBX4TeamD0Ev
// demangled: non-virtual thunk toRBX::Team::~Team()
// type: void __fastcall(RBX::Team *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Team::~Team()")]
#[doc(alias = "__ZThn36_N3RBX4TeamD0Ev")]
pub fn stub_66328c() {
    // IDA 0x66328c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x663294 — __ZThn32_N3RBX4TeamD1Ev
// demangled: non-virtual thunk toRBX::Team::~Team()
// type: void __fastcall(RBX::Team *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Team::~Team()")]
#[doc(alias = "__ZThn32_N3RBX4TeamD1Ev")]
pub fn stub_663294() {
    // IDA 0x663294: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x66329c — __ZThn36_N3RBX4TeamD1Ev
// demangled: non-virtual thunk toRBX::Team::~Team()
// type: void __fastcall(RBX::Team *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Team::~Team()")]
#[doc(alias = "__ZThn36_N3RBX4TeamD1Ev")]
pub fn stub_66329c() {
    // IDA 0x66329c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6632a4 — __ZNK3RBX4Team12askSetParentEPKNS_8InstanceE
// demangled: RBX::Team::askSetParent(RBX::Instance const*)const
// type: _DWORD __fastcall(RBX::Team *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Team::askSetParent(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX4Team12askSetParentEPKNS_8InstanceE")]
pub fn stub_6632a4(parent_is_teams: Option<bool>) -> bool {
    // IDA 0x6632a4 (`RBX::Team::askSetParent`): null parent returns
    // false (0x6632a8-0x6632b4); else the parent (adjusted +0x24)
    // must `isA(Teams)` (0x6632b6-0x6632d6), returning true only
    // then (0x6632d8-0x6632de). Host: presence + kind flags (same
    // `Option` shape as `Sparkles::askSetParent` at 0x63c52c).
    matches!(parent_is_teams, Some(true))
}

// 0x6632e0 — __ZN3RBX10Reflection14PropDescriptorINS_4TeamEiED1Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::Team,int>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,int>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_4TeamEiED1Ev")]
pub fn stub_6632e0() {
    // IDA 0x6632e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x663304 — __ZN3RBX10Reflection14PropDescriptorINS_4TeamENS_10BrickColorEED1Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::Team,RBX::BrickColor>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,RBX::BrickColor>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_4TeamENS_10BrickColorEED1Ev")]
pub fn stub_663304() {
    // IDA 0x663304: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x663328 — __ZN3RBX10Reflection14PropDescriptorINS_4TeamEbED1Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::Team,bool>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,bool>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_4TeamEbED1Ev")]
pub fn stub_663328() {
    // IDA 0x663328: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x66334c — __ZNK3RBX4Team11askAddChildEPKNS_8InstanceE
// demangled: RBX::Team::askAddChild(RBX::Instance const*)const
// type: _DWORD __fastcall(RBX::Team *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Team::askAddChild(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX4Team11askAddChildEPKNS_8InstanceE")]
pub fn stub_66334c() -> bool {
    // IDA 0x66334c (`RBX::Team::askAddChild`): `MOVS R0, #1; BX LR`
    // — any child is accepted (same shape as `Sparkles` at
    // 0x63c528).
    true
}

// 0x663350 — __ZNK3RBX14FactoryProductINS_4TeamENS_8InstanceELZNS_5sTeamEES2_E12getClassNameEv
// demangled: __ZNK3RBX14FactoryProductINS_4TeamENS_8InstanceELZNS_5sTeamEES2_E12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_4TeamENS_8InstanceELZNS_5sTeamEES2_E12getClassNameEv")]
pub fn stub_663350() -> &'static str {
    // IDA 0x663350 (`FactoryProduct<Team>::getClassName`):
    // `static_getCreator` (0x663354) then the `Creator::getClassName`
    // shim. Host: the declared name directly (grounded by the
    // `setName("Team")` in the ctor at 0x6630d2).
    "Team"
}

// 0x663360 — __ZThn32_NK3RBX14FactoryProductINS_4TeamENS_8InstanceELZNS_5sTeamEES2_E12getClassNameEv
// demangled: __ZThn32_NK3RBX14FactoryProductINS_4TeamENS_8InstanceELZNS_5sTeamEES2_E12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_4TeamENS_8InstanceELZNS_5sTeamEES2_E12getClassNameEv")]
pub fn stub_663360() {
    // IDA 0x663360: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x663b74 — __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_4TeamEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Team>(char const*,char const*,bool RBX::Team::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Team>(char const*,char const*,bool RBX::Team::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_4TeamEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_663b74(
    name: &str,
    category: &str,
    initial: bool,
    attributes: u32,
    permissions: u32,
) -> TeamBoolProp {
    // IDA 0x663b74 (`BoundProp<bool>::BoundProp<Team>`): the `Team`
    // `classDescriptor` call + `TypedPropertyDescriptor<bool>`
    // member-cell init (name/category/member/attributes/
    // permissions). The single object is `AutoColorCharacters`
    // (static `prop_AutoColorCharacters`, member offset 101 = 0x65,
    // IDA a_268 0x664368-0x664382). The cell folds into the field
    // (same shape as `SparklesBoolProp` at 0x63cc8c).
    TeamBoolProp::new(name, category, initial, attributes, permissions)
}

// 0x663d04 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_4TeamEE10isReadOnlyEv
// demangled: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Team>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Team>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_4TeamEE10isReadOnlyEv")]
pub fn stub_663d04() -> bool {
    // IDA 0x663d04 (`BoundPropGetSet<Team>::isReadOnly`): `MOVS R0,
    // #0; BX LR` — always readable.
    false
}

// 0x663d08 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_4TeamEE11isWriteOnlyEv
// demangled: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Team>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Team>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_4TeamEE11isWriteOnlyEv")]
pub fn stub_663d08() -> bool {
    // IDA 0x663d08 (`BoundPropGetSet<Team>::isWriteOnly`): `MOVS R0,
    // #0; BX LR` — always writable.
    false
}

// 0x663d0c — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_4TeamEE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Team>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Team>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_4TeamEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_663d0c(state: &TeamState) -> bool {
    // IDA 0x663d0c (`BoundPropGetSet<Team>::getValue`): loads the
    // member offset at +8, adjusts the described (`R1 - 36` when
    // non-null) and returns the byte there. The member is the +101
    // `AutoColorCharacters` cell; the offset folds into the field
    // (same shape as `Sparkles` at 0x63ce24).
    state.auto_color_characters
}

// 0x663d18 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_4TeamEE8setValueEPNS0_13DescribedBaseERKb
// demangled: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Team>::setValue(RBX::Reflection::DescribedBase *,bool const&)const
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Team>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_4TeamEE8setValueEPNS0_13DescribedBaseERKb")]
pub fn stub_663d18(state: &mut TeamState, value: bool) -> bool {
    // IDA 0x663d18 (`BoundPropGetSet<Team>::setValue`): adjusts the
    // described, returns early on match, else stores (0x663d30-
    // 0x663d34), runs the member hook when set and tail-calls
    // `raisePropertyChanged` (0x663d36-0x663d62). The raise folds
    // into the changed flag (same shape as `Sparkles` at 0x63ce30).
    if state.auto_color_characters == value {
        return false;
    }
    state.auto_color_characters = value;
    true
}

// 0x663d68 — __ZN3RBX10Reflection14PropDescriptorINS_4TeamEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::PropDescriptor<RBX::Team,bool>::PropDescriptor<bool (RBX::Team::*)(void)const,void (RBX::Team::*)(bool)>(char const*,char const*,bool (RBX::Team::*)(void)const,void (RBX::Team::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,bool>::PropDescriptor<bool (RBX::Team::*)(void)const,void (RBX::Team::*)(bool)>(char const*,char const*,bool (RBX::Team::*)(void)const,void (RBX::Team::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_4TeamEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_663d68(
    name: &str,
    category: &str,
    attributes: u32,
    permissions: u32,
) -> TeamPropDesc {
    // IDA 0x663d68 (`PropDescriptor<Team, bool>` ctor): the `Team`
    // `classDescriptor` call + `operator new` impl holding the
    // getter/setter member-pointer pair, then the
    // `TypedPropertyDescriptor<bool>` base init. The single object
    // is `AutoAssignable` (setter `setAutoAssignable`, IDA a_268
    // 0x664346). The pair folds into direct field access.
    TeamPropDesc::new(name, category, attributes, permissions)
}

// 0x663e7c — __ZN3RBX10Reflection14PropDescriptorINS_4TeamEbED0Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::Team,bool>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,bool>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_4TeamEbED0Ev")]
pub fn stub_663e7c() {
    // IDA 0x663e7c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x663ea8 — __ZNK3RBX10Reflection14PropDescriptorINS_4TeamEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::Team,bool>::GetSetImpl<bool (RBX::Team::*)(void)const,void (RBX::Team::*)(bool)>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,bool>::GetSetImpl<bool (RBX::Team::*)(void)const,void (RBX::Team::*)(bool)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_4TeamEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv")]
pub fn stub_663ea8() -> bool {
    // IDA 0x663ea8 (`GetSetImpl<bool>::isReadOnly`): `MOVS R0, #0;
    // BX LR` — always readable.
    false
}

// 0x663eac — __ZNK3RBX10Reflection14PropDescriptorINS_4TeamEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::Team,bool>::GetSetImpl<bool (RBX::Team::*)(void)const,void (RBX::Team::*)(bool)>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,bool>::GetSetImpl<bool (RBX::Team::*)(void)const,void (RBX::Team::*)(bool)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_4TeamEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv")]
pub fn stub_663eac() -> bool {
    // IDA 0x663eac (`GetSetImpl<bool>::isWriteOnly`): `MOVS R0, #0;
    // BX LR` — always writable.
    false
}

// 0x663eb0 — __ZNK3RBX10Reflection14PropDescriptorINS_4TeamEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::Team,bool>::GetSetImpl<bool (RBX::Team::*)(void)const,void (RBX::Team::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,bool>::GetSetImpl<bool (RBX::Team::*)(void)const,void (RBX::Team::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_4TeamEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_663eb0(state: &TeamState) -> bool {
    // IDA 0x663eb0 (`GetSetImpl<bool>::getValue`): the
    // member-pointer resolve tail-calling the getter. The member is
    // `getAutoAssignable` (IDA a_268 0x664346); the pointer folds
    // into the field.
    state.auto_assignable
}

// 0x663ed4 — __ZNK3RBX10Reflection14PropDescriptorINS_4TeamEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
// demangled: RBX::Reflection::PropDescriptor<RBX::Team,bool>::GetSetImpl<bool (RBX::Team::*)(void)const,void (RBX::Team::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,bool>::GetSetImpl<bool (RBX::Team::*)(void)const,void (RBX::Team::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_4TeamEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb")]
pub fn stub_663ed4(state: &mut TeamState, value: bool) -> bool {
    // IDA 0x663ed4 (`GetSetImpl<bool>::setValue`): the
    // member-pointer resolve tail-calling the setter with the input
    // byte. The member is `setAutoAssignable` (host:
    // stub_662fa8); the pointer folds into it.
    stub_662fa8(state, value)
}

// 0x663ef8 — __ZN3RBX10Reflection14PropDescriptorINS_4TeamENS_10BrickColorEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::PropDescriptor<RBX::Team,RBX::BrickColor>::PropDescriptor<RBX::BrickColor (RBX::Team::*)(void)const,void (RBX::Team::*)(RBX::BrickColor)>(char const*,char const*,RBX::BrickColor (RBX::Team::*)(void)const,void (RBX::Team::*)(RBX::BrickColor),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,RBX::BrickColor>::PropDescriptor<RBX::BrickColor (RBX::Team::*)(void)const,void (RBX::Team::*)(RBX::BrickColor)>(char const*,char const*,RBX::BrickColor (RBX::Team::*)(void)const,void (RBX::Team::*)(RBX::BrickColor),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_4TeamENS_10BrickColorEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_663ef8(
    name: &str,
    category: &str,
    attributes: u32,
    permissions: u32,
) -> TeamPropDesc {
    // IDA 0x663ef8 (`PropDescriptor<Team, BrickColor>` ctor): same
    // `classDescriptor` + impl + base-init shape for the single
    // object (`TeamColor`, setter `setTeamColor`, IDA a_268
    // 0x6642fa). The pair folds into direct field access.
    TeamPropDesc::new(name, category, attributes, permissions)
}

// 0x66400c — __ZN3RBX10Reflection14PropDescriptorINS_4TeamENS_10BrickColorEED0Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::Team,RBX::BrickColor>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,RBX::BrickColor>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_4TeamENS_10BrickColorEED0Ev")]
pub fn stub_66400c() {
    // IDA 0x66400c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x664038 — __ZNK3RBX10Reflection14PropDescriptorINS_4TeamENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::Team,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::Team::*)(void)const,void (RBX::Team::*)(RBX::BrickColor)>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::Team::*)(void)const,void (RBX::Team::*)(RBX::BrickColor)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_4TeamENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv")]
pub fn stub_664038() -> bool {
    // IDA 0x664038 (`GetSetImpl<BrickColor>::isReadOnly`): `MOVS
    // R0, #0; BX LR` — always readable.
    false
}

// 0x66403c — __ZNK3RBX10Reflection14PropDescriptorINS_4TeamENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::Team,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::Team::*)(void)const,void (RBX::Team::*)(RBX::BrickColor)>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::Team::*)(void)const,void (RBX::Team::*)(RBX::BrickColor)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_4TeamENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv")]
pub fn stub_66403c() -> bool {
    // IDA 0x66403c (`GetSetImpl<BrickColor>::isWriteOnly`): `MOVS
    // R0, #0; BX LR` — always writable.
    false
}

// 0x664040 — __ZNK3RBX10Reflection14PropDescriptorINS_4TeamENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::Team,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::Team::*)(void)const,void (RBX::Team::*)(RBX::BrickColor)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::Team::*)(void)const,void (RBX::Team::*)(RBX::BrickColor)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_4TeamENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_664040(state: &TeamState) -> u32 {
    // IDA 0x664040 (`GetSetImpl<BrickColor>::getValue`): the
    // member-pointer resolve tail-calling the getter. The member is
    // `getTeamColor`; the pointer folds into the field.
    state.team_color
}

// 0x664068 — __ZNK3RBX10Reflection14PropDescriptorINS_4TeamENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// demangled: RBX::Reflection::PropDescriptor<RBX::Team,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::Team::*)(void)const,void (RBX::Team::*)(RBX::BrickColor)>::setValue(RBX::Reflection::DescribedBase *,RBX::BrickColor const&)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::Team::*)(void)const,void (RBX::Team::*)(RBX::BrickColor)>::setValue(RBX::Reflection::DescribedBase *,RBX::BrickColor const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_4TeamENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_")]
pub fn stub_664068(state: &mut TeamState, value: u32) -> bool {
    // IDA 0x664068 (`GetSetImpl<BrickColor>::setValue`): the
    // member-pointer resolve tail-calling the setter with the input
    // word. The member is `setTeamColor` (host: stub_662f84); the
    // pointer folds into it.
    stub_662f84(state, value)
}

// 0x66408c — __ZN3RBX10Reflection14PropDescriptorINS_4TeamEiEC2IMS2_KFivEMS2_FviEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::PropDescriptor<RBX::Team,int>::PropDescriptor<int (RBX::Team::*)(void)const,void (RBX::Team::*)(int)>(char const*,char const*,int (RBX::Team::*)(void)const,void (RBX::Team::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,int>::PropDescriptor<int (RBX::Team::*)(void)const,void (RBX::Team::*)(int)>(char const*,char const*,int (RBX::Team::*)(void)const,void (RBX::Team::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_4TeamEiEC2IMS2_KFivEMS2_FviEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_66408c(
    name: &str,
    category: &str,
    attributes: u32,
    permissions: u32,
) -> TeamPropDesc {
    // IDA 0x66408c (`PropDescriptor<Team, int>` ctor): same
    // `classDescriptor` + impl + base-init shape for the single
    // object (`Score`, setter `setScore`, IDA a_268 0x6642ac). The
    // pair folds into direct field access.
    TeamPropDesc::new(name, category, attributes, permissions)
}

// 0x6641a0 — __ZN3RBX10Reflection14PropDescriptorINS_4TeamEiED0Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::Team,int>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,int>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_4TeamEiED0Ev")]
pub fn stub_6641a0() {
    // IDA 0x6641a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6641cc — __ZNK3RBX10Reflection14PropDescriptorINS_4TeamEiE10GetSetImplIMS2_KFivEMS2_FviEE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::Team,int>::GetSetImpl<int (RBX::Team::*)(void)const,void (RBX::Team::*)(int)>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,int>::GetSetImpl<int (RBX::Team::*)(void)const,void (RBX::Team::*)(int)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_4TeamEiE10GetSetImplIMS2_KFivEMS2_FviEE10isReadOnlyEv")]
pub fn stub_6641cc() -> bool {
    // IDA 0x6641cc (`GetSetImpl<int>::isReadOnly`): `MOVS R0, #0;
    // BX LR` — always readable.
    false
}

// 0x6641d0 — __ZNK3RBX10Reflection14PropDescriptorINS_4TeamEiE10GetSetImplIMS2_KFivEMS2_FviEE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::Team,int>::GetSetImpl<int (RBX::Team::*)(void)const,void (RBX::Team::*)(int)>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,int>::GetSetImpl<int (RBX::Team::*)(void)const,void (RBX::Team::*)(int)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_4TeamEiE10GetSetImplIMS2_KFivEMS2_FviEE11isWriteOnlyEv")]
pub fn stub_6641d0() -> bool {
    // IDA 0x6641d0 (`GetSetImpl<int>::isWriteOnly`): `MOVS R0, #0;
    // BX LR` — always writable.
    false
}

// 0x6641d4 — __ZNK3RBX10Reflection14PropDescriptorINS_4TeamEiE10GetSetImplIMS2_KFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::Team,int>::GetSetImpl<int (RBX::Team::*)(void)const,void (RBX::Team::*)(int)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,int>::GetSetImpl<int (RBX::Team::*)(void)const,void (RBX::Team::*)(int)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_4TeamEiE10GetSetImplIMS2_KFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_6641d4(state: &TeamState) -> i32 {
    // IDA 0x6641d4 (`GetSetImpl<int>::getValue`): the
    // member-pointer resolve tail-calling the getter. The member is
    // `getScore`; the pointer folds into the field.
    state.score
}

// 0x6641f4 — __ZNK3RBX10Reflection14PropDescriptorINS_4TeamEiE10GetSetImplIMS2_KFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi
// demangled: RBX::Reflection::PropDescriptor<RBX::Team,int>::GetSetImpl<int (RBX::Team::*)(void)const,void (RBX::Team::*)(int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Team,int>::GetSetImpl<int (RBX::Team::*)(void)const,void (RBX::Team::*)(int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_4TeamEiE10GetSetImplIMS2_KFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi")]
pub fn stub_6641f4(state: &mut TeamState, value: i32) -> bool {
    // IDA 0x6641f4 (`GetSetImpl<int>::setValue`): the
    // member-pointer resolve tail-calling the setter with the input
    // word. The member is `setScore` (host: stub_662f60); the
    // pointer folds into it.
    stub_662f60(state, value)
}

// 0x6645d4 — __ZN3RBX5Teams14rebalanceTeamsEv
// demangled: RBX::Teams::rebalanceTeams(void)
// type: _DWORD __fastcall(RBX::Teams *__hidden this)
#[doc(alias = "RBX::Teams::rebalanceTeams(void)")]
#[doc(alias = "__ZN3RBX5Teams14rebalanceTeamsEv")]
pub fn stub_6645d4() {
    // IDA 0x6645d4 (`RBX::Teams::rebalanceTeams`): empty body —
    // no-op.
}

// 0x6645d8 — __ZN3RBX5TeamsC1Ev
// demangled: RBX::Teams::Teams(void)
// type: _DWORD __fastcall(RBX::Teams *__hidden this)
#[doc(alias = "RBX::Teams::Teams(void)")]
#[doc(alias = "__ZN3RBX5TeamsC1Ev")]
pub fn stub_6645d8() -> TeamsState {
    // IDA 0x6645d8 (`RBX::Teams::Teams` C1): thunk tail-calling the
    // C2 (host: stub_6645dc).
    stub_6645dc()
}

// 0x6645dc — __ZN3RBX5TeamsC2Ev
// demangled: RBX::Teams::Teams(void)
// type: _DWORD __fastcall(RBX::Teams *__hidden this)
#[doc(alias = "RBX::Teams::Teams(void)")]
#[doc(alias = "__ZN3RBX5TeamsC2Ev")]
pub fn stub_6645dc() -> TeamsState {
    // IDA 0x6645dc (`RBX::Teams::Teams` C2): `ServiceProvider` base
    // + vtable installs + class registration (0x664668-0x664690);
    // the +92 flag byte is set to 1 (0x66469e); the team list at
    // +96 is an empty copy_on_write vector (0x6646c2-0x6646da);
    // `setName("Teams")` (0x6646f8-0x664706). Host: the cleared
    // cutover (same shape as `SpawnerServiceState` at 0x63db8c).
    TeamsState {
        flag_92: true,
        teams: Vec::new(),
    }
}

// 0x66482c — __ZN3RBX5TeamsD0Ev
// demangled: RBX::Teams::~Teams()
// type: void __fastcall(RBX::Teams *__hidden this)
#[doc(alias = "RBX::Teams::~Teams()")]
#[doc(alias = "__ZN3RBX5TeamsD0Ev")]
pub fn stub_66482c() {
    // IDA 0x66482c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6648cc — __ZN3RBX5TeamsD1Ev
// demangled: RBX::Teams::~Teams()
// type: void __fastcall(RBX::Teams *__hidden this)
#[doc(alias = "RBX::Teams::~Teams()")]
#[doc(alias = "__ZN3RBX5TeamsD1Ev")]
pub fn stub_6648cc() {
    // IDA 0x6648cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6648d0 — __ZThn32_N3RBX5TeamsD0Ev
// demangled: non-virtual thunk toRBX::Teams::~Teams()
// type: void __fastcall(RBX::Teams *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Teams::~Teams()")]
#[doc(alias = "__ZThn32_N3RBX5TeamsD0Ev")]
pub fn stub_6648d0() {
    // IDA 0x6648d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6648d8 — __ZThn36_N3RBX5TeamsD0Ev
// demangled: non-virtual thunk toRBX::Teams::~Teams()
// type: void __fastcall(RBX::Teams *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Teams::~Teams()")]
#[doc(alias = "__ZThn36_N3RBX5TeamsD0Ev")]
pub fn stub_6648d8() {
    // IDA 0x6648d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6648e0 — __ZN3RBX5TeamsD2Ev
// demangled: RBX::Teams::~Teams()
// type: void __fastcall(RBX::Teams *__hidden this)
#[doc(alias = "RBX::Teams::~Teams()")]
#[doc(alias = "__ZN3RBX5TeamsD2Ev")]
pub fn stub_6648e0() {
    // IDA 0x6648e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6649c4 — __ZThn32_N3RBX5TeamsD1Ev
// demangled: non-virtual thunk toRBX::Teams::~Teams()
// type: void __fastcall(RBX::Teams *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Teams::~Teams()")]
#[doc(alias = "__ZThn32_N3RBX5TeamsD1Ev")]
pub fn stub_6649c4() {
    // IDA 0x6649c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6649cc — __ZThn36_N3RBX5TeamsD1Ev
// demangled: non-virtual thunk toRBX::Teams::~Teams()
// type: void __fastcall(RBX::Teams *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Teams::~Teams()")]
#[doc(alias = "__ZThn36_N3RBX5TeamsD1Ev")]
pub fn stub_6649cc() {
    // IDA 0x6649cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6649d4 — __ZN3RBX5Teams10isTeamGameEv
// demangled: RBX::Teams::isTeamGame(void)
// type: _DWORD __fastcall(RBX::Teams *__hidden this)
#[doc(alias = "RBX::Teams::isTeamGame(void)")]
#[doc(alias = "__ZN3RBX5Teams10isTeamGameEv")]
pub fn stub_6649d4(players_present: bool, players: &[SpawnPlayerRef]) -> bool {
    // IDA 0x6649d4 (`RBX::Teams::isTeamGame`): a null `Players`
    // service returns 0 (0x6649e4-0x6649e8); zero children returns 0
    // (0x6649f2-0x6649f4); the child slots (8-byte `shared_ptr`
    // elements, 0x664a0a-0x664a16) skip nulls (0x664a16-0x664a1c)
    // and non-`Player`s (`isA`, 0x664a1e-0x664a2e); the first
    // non-neutral (+104) child returns 1 immediately
    // (0x664a30-0x664a3c); exhausting the list returns 0
    // (0x664a3e-0x664a4c). (The decompiler misreads the loop-bottom
    // `MOVS R0, #0` as a post-loop reset — the disasm shows it feeds
    // the next compare/return.) Host: the slice holds the `Player`
    // children; nulls/non-players fold out.
    if !players_present {
        return false;
    }
    players.iter().any(|player| !player.neutral)
}

// 0x664a54 — __ZN3RBX5Teams21assignNewPlayerToTeamEPNS_7Network6PlayerE
// demangled: RBX::Teams::assignNewPlayerToTeam(RBX::Network::Player *)
// type: _DWORD __fastcall(RBX::Teams *__hidden this, RBX::Network::Player *)
#[doc(alias = "RBX::Teams::assignNewPlayerToTeam(RBX::Network::Player *)")]
#[doc(alias = "__ZN3RBX5Teams21assignNewPlayerToTeamEPNS_7Network6PlayerE")]
pub fn stub_664a54(
    teams: &TeamsState,
    players: &[SpawnPlayerRef],
    player: &mut SpawnPlayerRef,
) -> bool {
    // IDA 0x664a54 (`RBX::Teams::assignNewPlayerToTeam`): walks the
    // team children (`isA Team`, 0x664aa4-0x664ac4); only
    // auto-assignable teams qualify (0x664ac4); each is scored by
    // `getNumPlayersInTeam` (0x664ad6, host: stub_664b24) tracking
    // the minimum from 10000 (0x664adc-0x664aee); on a hit the
    // player's team color is set and neutrality cleared
    // (0x664b0e-0x664b16), else false (0x664b00-0x664b20). Host:
    // the teams vector replaces the child walk.
    let mut best: Option<(u32, usize)> = None;
    for team in &teams.teams {
        if !team.auto_assignable {
            continue;
        }
        let count = stub_664b24(true, players, team.team_color);
        if count < best.map(|(_, c)| c).unwrap_or(10000) {
            best = Some((team.team_color, count));
        }
    }
    match best {
        Some((team_color, _)) => {
            player.team_color = team_color;
            player.neutral = false;
            true
        }
        None => false,
    }
}

// 0x664b24 — __ZN3RBX5Teams19getNumPlayersInTeamENS_10BrickColorE
// demangled: RBX::Teams::getNumPlayersInTeam(RBX::BrickColor)
#[doc(alias = "RBX::Teams::getNumPlayersInTeam(RBX::BrickColor)")]
#[doc(alias = "__ZN3RBX5Teams19getNumPlayersInTeamENS_10BrickColorE")]
pub fn stub_664b24(players_present: bool, players: &[SpawnPlayerRef], team_color: u32) -> usize {
    // IDA 0x664b24 (`RBX::Teams::getNumPlayersInTeam`):
    // `ReleaseAssert(players)` gated on `FLog::Asserts` with the
    // debug hook (Teams.cpp line 97, 0x664b42-0x664b82 — a host
    // seam), then counts the non-null `Player` children (`isA`,
    // 0x664be6) that are not neutral (+104) and whose team color
    // (+100) equals the arg (0x664ba8-0x664bf6). Host: the player
    // slice replaces the child walk; nulls/non-players fold out.
    if flog_asserts() {
        assert!(
            players_present,
            "players file: Client/App/v8datamodel/Teams.cpp line: 97 (IDA 0x664b24)"
        );
    }
    if !players_present {
        return 0;
    }
    players
        .iter()
        .filter(|player| !player.neutral && player.team_color == team_color)
        .count()
}

// 0x664c04 — __ZN3RBX5Teams10teamExistsENS_10BrickColorE
// demangled: RBX::Teams::teamExists(RBX::BrickColor)
#[doc(alias = "RBX::Teams::teamExists(RBX::BrickColor)")]
#[doc(alias = "__ZN3RBX5Teams10teamExistsENS_10BrickColorE")]
pub fn stub_664c04(teams: &TeamsState, color: u32) -> bool {
    // IDA 0x664c04 (`RBX::Teams::teamExists`): returns
    // `getTeamFromTeamColor() != 0` (host: stub_664c14).
    stub_664c14(teams, color).is_some()
}

// 0x664c14 — __ZN3RBX5Teams20getTeamFromTeamColorENS_10BrickColorE
// demangled: RBX::Teams::getTeamFromTeamColor(RBX::BrickColor)
#[doc(alias = "RBX::Teams::getTeamFromTeamColor(RBX::BrickColor)")]
#[doc(alias = "__ZN3RBX5Teams20getTeamFromTeamColorENS_10BrickColorE")]
pub fn stub_664c14(teams: &TeamsState, team_color: u32) -> Option<SharedPtr<TeamState>> {
    // IDA 0x664c14 (`RBX::Teams::getTeamFromTeamColor`): walks the
    // children for `isA Team` (0x664c56-0x664c68); the first whose
    // `getTeamColor` equals the arg breaks out (0x664c74-0x664c7e),
    // else null (0x664c82-0x664c98). Host: find the matching team
    // in the vector.
    teams
        .teams
        .iter()
        .find(|team| team.team_color == team_color)
        .cloned()
}

// 0x664c9c — __ZN3RBX5Teams17getTeamFromPlayerEPNS_7Network6PlayerE
// demangled: RBX::Teams::getTeamFromPlayer(RBX::Network::Player *)
// type: _DWORD __fastcall(RBX::Teams *__hidden this, RBX::Network::Player *)
#[doc(alias = "RBX::Teams::getTeamFromPlayer(RBX::Network::Player *)")]
#[doc(alias = "__ZN3RBX5Teams17getTeamFromPlayerEPNS_7Network6PlayerE")]
pub fn stub_664c9c(teams: &TeamsState, player: &SpawnPlayerRef) -> Option<SharedPtr<TeamState>> {
    // IDA 0x664c9c (`RBX::Teams::getTeamFromPlayer`): a neutral
    // player (+104) yields null (0x664c9c-0x664ca4); else
    // `getTeamFromTeamColor(player[25])` — the +100 team color
    // (host: stub_664c14).
    if player.neutral {
        return None;
    }
    stub_664c14(teams, player.team_color)
}

// 0x664cb0 — __ZN3RBX5Teams23getTeamColorForHumanoidEPNS_8HumanoidE
// demangled: RBX::Teams::getTeamColorForHumanoid(RBX::Humanoid *)
// type: _DWORD __fastcall(RBX::Teams *__hidden this, RBX::Humanoid *)
#[doc(alias = "RBX::Teams::getTeamColorForHumanoid(RBX::Humanoid *)")]
#[doc(alias = "__ZN3RBX5Teams23getTeamColorForHumanoidEPNS_8HumanoidE")]
pub fn stub_664cb0(players_present: bool, players: &[PlayerTeamRef]) -> Option<u32> {
    // IDA 0x664cb0 (`RBX::Teams::getTeamColorForHumanoid`):
    // `ReleaseAssert(players)` (Teams.cpp line 160, host seam);
    // null service or zero children yields white (0x664d16-0x664d22,
    // host: `None`); the player-child scan skips nulls
    // (0x664d44-0x664d4e), non-`Player`s (`isA`, 0x664d60-0x664d62),
    // neutrals (+0x68, 0x664d6a-0x664d6e) and null characters
    // (0x664d70-0x664d72); the first whose character contains the
    // humanoid (`findConstFirstChildOfType`, 0x664d74-0x664d7c)
    // returns `BrickColor::color3(teamcolor)` (0x664da6-0x664db2);
    // exhausting the list yields white (0x664d8c-0x664da4). Host:
    // the pointer walk folds into flags; the runtime-`BrickMap`
    // RGB conversion folds into the id (`None` = white).
    if flog_asserts() {
        assert!(
            players_present,
            "players file: Client/App/v8datamodel/Teams.cpp line: 160 (IDA 0x664cb0)"
        );
    }
    if !players_present {
        return None;
    }
    players
        .iter()
        .find(|player| !player.neutral && player.owns_humanoid)
        .map(|player| player.team_color)
}

// 0x664db4 — __ZN3RBX5Teams12onChildAddedEPNS_8InstanceE
// demangled: RBX::Teams::onChildAdded(RBX::Instance *)
// type: _DWORD __fastcall(RBX::Teams *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::Teams::onChildAdded(RBX::Instance *)")]
#[doc(alias = "__ZN3RBX5Teams12onChildAddedEPNS_8InstanceE")]
pub fn stub_664db4(teams: &mut TeamsState, team: SharedPtr<TeamState>, is_team: bool) {
    // IDA 0x664db4 (`RBX::Teams::onChildAdded`): a non-null child
    // that `isA Team` (0x664e2c) is `shared_from`'d and pushed into
    // the +96 vector via the copy_on_write `write` gate
    // (0x664e40-0x664e66); others are ignored. Host: push on the
    // kind flag (same hook/unhook discipline as `SpawnerService`
    // at 0x63da9c).
    if is_team {
        teams.teams.push(team);
    }
}

// 0x664ef4 — __ZN3RBX5Teams15onChildRemovingEPNS_8InstanceE
// demangled: RBX::Teams::onChildRemoving(RBX::Instance *)
// type: _DWORD __fastcall(RBX::Teams *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::Teams::onChildRemoving(RBX::Instance *)")]
#[doc(alias = "__ZN3RBX5Teams15onChildRemovingEPNS_8InstanceE")]
pub fn stub_664ef4(teams: &mut TeamsState, team: &SharedPtr<TeamState>) {
    // IDA 0x664ef4 (`RBX::Teams::onChildRemoving`): a child that
    // `isA Team` (0x664f6c) is located with `__find` (0x664f94,
    // host: stub_665230) and erased from the +96 vector (0x664fa0).
    // Host: retain all but the removed link (same shape as
    // `list<SpawnLocation*>::remove` at 0x63e66c).
    teams.teams.retain(|slot| !SharedPtr::ptr_eq(slot, team));
}

// 0x665008 — __ZN3RBX10Reflection13BoundFuncDescINS_5TeamsEFvvELi0EED1Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Teams,void ()(void),0>::~BoundFuncDesc()
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Teams,void ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_5TeamsEFvvELi0EED1Ev")]
pub fn stub_665008() {
    // IDA 0x665008: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x66502c — __ZN3RBX5Teams8getTeamsEv
// demangled: RBX::Teams::getTeams(void)
// type: _DWORD __fastcall(RBX::Teams *__hidden this)
#[doc(alias = "RBX::Teams::getTeams(void)")]
#[doc(alias = "__ZN3RBX5Teams8getTeamsEv")]
pub fn stub_66502c(teams: &TeamsState) -> Vec<SharedPtr<TeamState>> {
    // IDA 0x66502c (`RBX::Teams::getTeams`): copies the +96
    // copy_on_write vector's `shared_count` pair into the out slot
    // (0x665032-0x66503c, shared ownership). Host: clone the
    // vector.
    teams.teams.clone()
}

// 0x665040 — __ZN3RBX10Reflection13BoundFuncDescINS_5TeamsEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EED1Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Teams,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Teams,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_5TeamsEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EED1Ev")]
pub fn stub_665040() {
    // IDA 0x665040: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x665064 — __ZN3RBX11shared_fromINS_4TeamEEEN5boost10shared_ptrIT_EEPS4_
// demangled: boost::shared_ptr<RBX::Team> RBX::shared_from<RBX::Team>(RBX::Team*)
// type: int(void)
#[doc(alias = "boost::shared_ptr<RBX::Team> RBX::shared_from<RBX::Team>(RBX::Team*)")]
#[doc(alias = "__ZN3RBX11shared_fromINS_4TeamEEEN5boost10shared_ptrIT_EEPS4_")]
pub fn stub_665064(team: Option<SharedPtr<TeamState>>) -> Option<SharedPtr<TeamState>> {
    // IDA 0x665064 (`shared_from<Team>`): a null input yields null
    // (0x6650b2-0x665140); else the weak-owner dance re-locks the
    // control block (0x6650b4-0x665168, throwing `bad_weak_ptr` on
    // expiry). Arc ownership never expires while held — the move is
    // identity.
    team
}

// 0x6651d4 — __ZNK3RBX5Teams11askAddChildEPKNS_8InstanceE
// demangled: RBX::Teams::askAddChild(RBX::Instance const*)const
// type: _DWORD __fastcall(RBX::Teams *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Teams::askAddChild(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX5Teams11askAddChildEPKNS_8InstanceE")]
pub fn stub_6651d4(child_is_team: Option<bool>) -> bool {
    // IDA 0x6651d4 (`RBX::Teams::askAddChild`): null child returns
    // false (0x6651d8-0x6651e4); else the child (adjusted +0x24)
    // must `isA Team` (0x6651e6-0x665206), returning true only then
    // (0x665208-0x66520e). Same shape as `Team::askSetParent` at
    // 0x6632a4.
    matches!(child_is_team, Some(true))
}

// 0x665230 — __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEENS3_INS4_4TeamEEEET_SE_SE_RKT0_St26random_access_iterator_tag
// demangled: __gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> std::__find<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::shared_ptr<RBX::Team>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::shared_ptr<RBX::Team> const&,std::random_access_iterator_tag)
// type: int(void)
#[doc(alias = "__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> std::__find<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::shared_ptr<RBX::Team>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::shared_ptr<RBX::Team> const&,std::random_access_iterator_tag)")]
#[doc(alias = "__ZSt6__findIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEENS3_INS4_4TeamEEEET_SE_SE_RKT0_St26random_access_iterator_tag")]
pub fn stub_665230(teams: &[SharedPtr<TeamState>], team: &SharedPtr<TeamState>) -> Option<usize> {
    // IDA 0x665230 (`__find` over the team vector): pointer-equality
    // scan over the 8-byte `shared_ptr` elements (0x665248-0x66526c
    // unrolled, 0x665274-0x6652be tails), returning the hit or the
    // end iterator. Host: the position (`None` past the end).
    teams.iter().position(|slot| SharedPtr::ptr_eq(slot, team))
}

// 0x6654bc — __ZN3RBX10Reflection13BoundFuncDescINS_5TeamsEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EEC2EMS2_FSB_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Teams,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::BoundFuncDesc(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::Teams::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Teams,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::BoundFuncDesc(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::Teams::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_5TeamsEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EEC2EMS2_FSB_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_6654bc() {
    // IDA 0x6654bc: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x6655c0 — __ZN3RBX10Reflection13BoundFuncDescINS_5TeamsEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EED0Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Teams,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Teams,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_5TeamsEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EED0Ev")]
pub fn stub_6655c0() {
    // IDA 0x6655c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x665674 — __ZNK3RBX10Reflection13BoundFuncDescINS_5TeamsEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Teams,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Teams,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_5TeamsEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_665674(teams: &TeamsState) -> Vec<SharedPtr<TeamState>> {
    // IDA 0x665674 (`BoundFuncDesc<Teams, vector>(...)::execute`):
    // tail-calls `Call0Helper::call` (host: stub_665698), which
    // invokes the bound `getTeams` member and fills the out
    // variant. Host: the team list.
    stub_665698(teams)
}

// 0x665698 — __ZN3RBX10Reflection11Call0HelperINS_5TeamsEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvESB_E4callEPS2_SD_RNS0_7VariantE
// demangled: RBX::Reflection::Call0Helper<RBX::Teams,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::Teams::*)(void),boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>>::call(RBX::Teams*,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::Teams::*)(void),RBX::Reflection::Variant &)
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::Teams,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::Teams::*)(void),boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>>::call(RBX::Teams*,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::Teams::*)(void),RBX::Reflection::Variant &)")]
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_5TeamsEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvESB_E4callEPS2_SD_RNS0_7VariantE")]
pub fn stub_665698(teams: &TeamsState) -> Vec<SharedPtr<TeamState>> {
    // IDA 0x665698 (`Call0Helper<Teams, getTeams>::call`): the
    // member-pointer resolve tail-calling the bound member, whose
    // return fills the out `Variant` (host: stub_66502c). The pair
    // folds into the call.
    stub_66502c(teams)
}

// 0x665780 — __ZN3RBX10Reflection13BoundFuncDescINS_5TeamsEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Teams,void ()(void),0>::BoundFuncDesc(void (RBX::Teams::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Teams,void ()(void),0>::BoundFuncDesc(void (RBX::Teams::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_5TeamsEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_665780(name: &str) -> TeamsVoidFuncDesc {
    // IDA 0x665780 (`BoundFuncDesc<Teams, void()>::BoundFuncDesc`):
    // the `Teams` `classDescriptor` call (0x6657a6), the
    // `FunctionDescriptor` base init and the member pair at +40
    // (0x6657c6-0x6657ee). The single object binds
    // `rebalanceTeams` ("RebalanceTeams", IDA a_269 0x6659a6-0x6659ca).
    // The pair folds into the name.
    TeamsVoidFuncDesc::new(name)
}

// 0x665884 — __ZN3RBX10Reflection13BoundFuncDescINS_5TeamsEFvvELi0EED0Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Teams,void ()(void),0>::~BoundFuncDesc()
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Teams,void ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_5TeamsEFvvELi0EED0Ev")]
pub fn stub_665884() {
    // IDA 0x665884: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x665938 — __ZNK3RBX10Reflection13BoundFuncDescINS_5TeamsEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Teams,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Teams,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_5TeamsEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_665938() {
    // IDA 0x665938 (`BoundFuncDesc<Teams, void()>::execute`): the
    // member-pointer resolve tail-calling the +40 member — the
    // `rebalanceTeams` binding (host: stub_6645d4).
    stub_6645d4();
}

// 0x665c58 — __ZN3RBX7TextBox12setMultiLineEb
// demangled: RBX::TextBox::setMultiLine(bool)
// type: _DWORD __fastcall(RBX::TextBox *__hidden this, bool)
#[doc(alias = "RBX::TextBox::setMultiLine(bool)")]
#[doc(alias = "__ZN3RBX7TextBox12setMultiLineEb")]
pub fn stub_665c58(state: &mut TextBoxState, multi_line: bool) -> bool {
    // IDA 0x665c58 (`RBX::TextBox::setMultiLine`): compares +652
    // (0x665c5e); on change stores it (0x665c6a) and raises
    // `raisePropertyChanged` (0x665c74), else returns unchanged
    // (0x665c60). The raise folds into the changed flag.
    if state.multi_line == multi_line {
        return false;
    }
    state.multi_line = multi_line;
    true
}

// 0x665c78 — __ZN3RBX7TextBox19setClearTextOnFocusEb
// demangled: RBX::TextBox::setClearTextOnFocus(bool)
// type: _DWORD __fastcall(RBX::TextBox *__hidden this, bool)
#[doc(alias = "RBX::TextBox::setClearTextOnFocus(bool)")]
#[doc(alias = "__ZN3RBX7TextBox19setClearTextOnFocusEb")]
pub fn stub_665c78(state: &mut TextBoxState, clear_text_on_focus: bool) -> bool {
    // IDA 0x665c78 (`RBX::TextBox::setClearTextOnFocus`): compares
    // +607 (0x665c7e); on change stores it (0x665c8a) and raises
    // `raisePropertyChanged` (0x665c94), else returns unchanged
    // (0x665c80). The raise folds into the changed flag.
    if state.clear_text_on_focus == clear_text_on_focus {
        return false;
    }
    state.clear_text_on_focus = clear_text_on_focus;
    true
}

// 0x665c98 — __ZN3RBX7TextBox12captureFocusEv
// demangled: RBX::TextBox::captureFocus(void)
// type: _DWORD __fastcall(RBX::TextBox *__hidden this)
#[doc(alias = "RBX::TextBox::captureFocus(void)")]
#[doc(alias = "__ZN3RBX7TextBox12captureFocusEv")]
pub fn stub_665c98(state: &mut TextBoxState, input_service_created: bool, fire_focused: impl Fn()) {
    // IDA 0x665c98 (`RBX::TextBox::captureFocus`): the +155/+152
    // cursor dance folds away (0x665cce); +605 is set (0x665cd8),
    // the +608 string is cleared (0x665cdc), +155 is zeroed
    // (0x665ce2) and the +604 focused flag is set (0x665ce6);
    // `create<UserInputService>` (0x665cea) gates `shared_from`
    // plus the `Focused` signal fire (0x665d28-0x665d44). Host:
    // the flag, the cleared focus text and the gated closure call.
    state.focused = true;
    state.focus_text.clear();
    if input_service_created {
        fire_focused();
    }
}

// 0x665da0 — __ZN3RBX7TextBox7setTextESs
// demangled: RBX::TextBox::setText(std::string)
#[doc(alias = "RBX::TextBox::setText(std::string)")]
#[doc(alias = "__ZN3RBX7TextBox7setTextESs")]
pub fn stub_665da0(state: &mut TextBoxState, text: &str, filter_pass: bool) {
    // IDA 0x665da0 (`RBX::TextBox::setText`): over-0x400 inputs are
    // cut down (0x665e00-0x665ef4, host: byte-truncate floored to a
    // char boundary); a profanity hit without the fw+22 override
    // skips silently (0x665e56); on difference from the +540 text
    // (0x665e62) it assigns it, zeroes word 134 (+536, 0x665e82)
    // and raises three descriptors (0x665e92-0x665eae, folds into
    // the mutation). Host: mutate on change only.
    if !filter_pass {
        return;
    }
    let mut clipped = text.to_owned();
    if clipped.len() > 0x400 {
        let mut end = 0x400;
        while !clipped.is_char_boundary(end) {
            end -= 1;
        }
        clipped.truncate(end);
    }
    if state.text == clipped {
        return;
    }
    state.text = clipped;
}

// 0x665f5c — __ZN3RBX7TextBox11setFontSizeENS_11TextService8FontSizeE
// demangled: RBX::TextBox::setFontSize(RBX::TextService::FontSize)
#[doc(alias = "RBX::TextBox::setFontSize(RBX::TextService::FontSize)")]
#[doc(alias = "__ZN3RBX7TextBox11setFontSizeENS_11TextService8FontSizeE")]
pub fn stub_665f5c(state: &mut TextBoxState, font_size: u32) -> bool {
    // IDA 0x665f5c (`RBX::TextBox::setFontSize`): compares word 136
    // (+544, 0x665f68); on change stores it (0x665f76) and raises
    // twice (0x665f80-0x665f8e), else returns unchanged. The raises
    // fold into the changed flag. The member is the `FontSize`
    // enum id (same field the `FontSize` descriptor binds).
    if state.font_size == font_size {
        return false;
    }
    state.font_size = font_size;
    true
}

// 0x665f94 — __ZN3RBX7TextBox7setFontENS_11TextService4FontE
// demangled: RBX::TextBox::setFont(RBX::TextService::Font)
#[doc(alias = "RBX::TextBox::setFont(RBX::TextService::Font)")]
#[doc(alias = "__ZN3RBX7TextBox7setFontENS_11TextService4FontE")]
pub fn stub_665f94(state: &mut TextBoxState, font: u32) -> bool {
    // IDA 0x665f94 (`RBX::TextBox::setFont`): compares word 148
    // (+592, 0x665fa0); on change stores it (0x665fae) and raises
    // twice (0x665fb8-0x665fc6), else returns unchanged. The raises
    // fold into the changed flag (same field the `Font` descriptor
    // binds).
    if state.font == font {
        return false;
    }
    state.font = font;
    true
}

// 0x665fcc — __ZN3RBX7TextBox12setTextColorENS_10BrickColorE
// demangled: RBX::TextBox::setTextColor(RBX::BrickColor)
#[doc(alias = "RBX::TextBox::setTextColor(RBX::BrickColor)")]
#[doc(alias = "__ZN3RBX7TextBox12setTextColorENS_10BrickColorE")]
pub fn stub_665fcc(state: &mut TextBoxState, text_color: u32) {
    // IDA 0x665fcc (`RBX::TextBox::setTextColor`): converts the
    // `BrickColor` id via `BrickColor::color3` (0x665fdc, runtime
    // `BrickMap` palette — ungrounded in this range) and delegates
    // to `setTextColor3` (0x665fea, host: stub_665fec). The id
    // itself is cached (the binary derives reads via `closest`;
    // same gap).
    state.text_color = text_color;
}

// 0x665fec — __ZN3RBX7TextBox13setTextColor3EN3G3D6Color3E
// demangled: RBX::TextBox::setTextColor3(G3D::Color3)
#[doc(alias = "RBX::TextBox::setTextColor3(G3D::Color3)")]
#[doc(alias = "__ZN3RBX7TextBox13setTextColor3EN3G3D6Color3E")]
pub fn stub_665fec(state: &mut TextBoxState, value: [f32; 3]) -> bool {
    // IDA 0x665fec (`RBX::TextBox::setTextColor3`): compares words
    // 137-139 (+548, 0x666002) with early-outs; on any difference
    // stores all three (0x66603a-0x666052) and raises twice
    // (0x666058-0x666066). The raises fold into the changed flag
    // (same field the `TextColor3` descriptor binds).
    if state.text_color3 == value {
        return false;
    }
    state.text_color3 = value;
    true
}

// 0x66606c — __ZN3RBX7TextBox19setTextTransparencyEf
// demangled: RBX::TextBox::setTextTransparency(float)
// type: _DWORD __fastcall(RBX::TextBox *__hidden this, float)
#[doc(alias = "RBX::TextBox::setTextTransparency(float)")]
#[doc(alias = "__ZN3RBX7TextBox19setTextTransparencyEf")]
pub fn stub_66606c(state: &mut TextBoxState, value: f32) -> bool {
    // IDA 0x66606c (`RBX::TextBox::setTextTransparency`): compares
    // word 140 (+560, 0x66607c); on change stores it (0x666088) and
    // raises (0x666090), else returns unchanged. The raise folds
    // into the changed flag (same field the `TextTransparency`
    // descriptor binds).
    if state.text_transparency == value {
        return false;
    }
    state.text_transparency = value;
    true
}
