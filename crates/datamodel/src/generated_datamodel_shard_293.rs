// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: demangled contains RBX::Instance|DataModel|Workspace (gap-filler distinct not yet in datamodel, EA-sorted asc)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 100 stubs | range 0x578d50..0x57f894 | total filtered gap-filler, remaining 43556 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use super::generated_datamodel_shard_291::{
    BackpackTextureId, BackpackTextureProp, HOPPER_BIN_TYPE_ITEMS, stub_0x5713d0, stub_0x5713e8,
    stub_0x571428, stub_0x573688,
};
use crate::instance::Weld;

/// Rust model of `RBX::IEquipable` (IDA `0x57bf9c`): the equipable mixin
/// holding the shared `Weld` at `+8` (released by the `D2`, IDA `0x57c058`,
/// with a `ReleaseAssert(!weld)` at `IEquipable.cpp:18`); the vtable word
/// collapses.
#[derive(Default)]
pub struct IEquipable {
    pub weld: Option<SharedPtr<Weld>>,
}

// 0x578d50 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_0x578d50(bin: &mut crate::instance::HopperBin, index: usize) -> bool {
    // IDA 0x578d50 `EnumPropDescriptor<BinType>::setIndexValue`:
    // bounds-checks the index, reads the value, and sets; out-of-range
    // sets nothing. Same shape as 0x56eaec over `HOPPER_BIN_TYPE_ITEMS`.
    if let Some((value, _)) = HOPPER_BIN_TYPE_ITEMS.get(index) {
        stub_0x571428(bin, *value);
        true
    } else {
        false
    }
}

// 0x578d84 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x578d84(bin: &crate::instance::HopperBin) -> i32 {
    // IDA 0x578d84 `EnumPropDescriptor<BinType>::getEnumValue`: reads the
    // value through the member getter. Same shape as 0x56eb20; delegates
    // to `getBinType` (0x573688).
    stub_0x573688(bin)
}

// 0x578d8c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x578d8c(bin: &mut crate::instance::HopperBin, value: i32) -> bool {
    // IDA 0x578d8c `EnumPropDescriptor<BinType>::setEnumValue`: validates
    // the value against the table, sets on hit and returns true, false on
    // miss. Same shape as 0x56eb28; the store routes through `setBinType`
    // (0x571428, with its legacy-texture refresh).
    if HOPPER_BIN_TYPE_ITEMS.iter().any(|(v, _)| *v == value) {
        stub_0x571428(bin, value);
        true
    } else {
        false
    }
}

// 0x578dd8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x578dd8(bin: &crate::instance::HopperBin) -> Option<(i32, &'static str)> {
    // IDA 0x578dd8 `EnumPropDescriptor<BinType>::getEnumItem`: reads the
    // value through the member getter, then the item search. Same shape as
    // 0x56eb74.
    let current = stub_0x573688(bin);
    HOPPER_BIN_TYPE_ITEMS
        .iter()
        .find(|(v, _)| *v == current)
        .copied()
}

// 0x578df8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_0x578df8(bin: &mut crate::instance::HopperBin, name: &str) -> bool {
    // IDA 0x578df8 `EnumPropDescriptor<BinType>::setStringValue(Name)`:
    // converts via the desc table and sets on hit, false on miss. `Name`
    // collapses to the stored bytes. Same shape as 0x56eb94.
    if let Some(value) = HOPPER_BIN_TYPE_ITEMS
        .iter()
        .find(|(_, text)| *text == name)
        .map(|(v, _)| *v)
    {
        stub_0x571428(bin, value);
        true
    } else {
        false
    }
}

// 0x578e2c — __ZNK3RBX10Reflection8EnumDescINS_9HopperBin7BinTypeEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::HopperBin::BinType>::convertToIndex(RBX::HopperBin::BinType)const")]
pub use rbx_reflection::generated::stub_0x578e2c as stub_0x578e2c;

// 0x578e9c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x578e9c(bin: &mut crate::instance::HopperBin, value: i32) -> bool {
    // IDA 0x578e9c `EnumPropDescriptor<BinType>::setIntValue`: negative
    // values return false at once; values past the table size return false;
    // table entries holding the `-1` sentinel return false; else the member
    // setter runs and the result is true. The `BinType` table holds no
    // sentinels, so the bounds check covers all three rejections. Same
    // shape as 0x56ec38.
    if value < 0 || (value as usize) >= HOPPER_BIN_TYPE_ITEMS.len() {
        return false;
    }
    stub_0x571428(bin, value);
    true
}

// 0x578edc — __ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinENS2_7BinTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::GetSetImpl<RBX::HopperBin::BinType (RBX::HopperBin::*)(void)const,void (RBX::HopperBin::*)(RBX::HopperBin::BinType)>::isReadOnly(void)const")]
pub use rbx_reflection::generated::stub_0x578edc as stub_0x578edc;

// 0x578ee0 — __ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinENS2_7BinTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::GetSetImpl<RBX::HopperBin::BinType (RBX::HopperBin::*)(void)const,void (RBX::HopperBin::*)(RBX::HopperBin::BinType)>::isWriteOnly(void)const")]
pub use rbx_reflection::generated::stub_0x578ee0 as stub_0x578ee0;

// 0x578ee4 — __ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinENS2_7BinTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::GetSetImpl<RBX::HopperBin::BinType (RBX::HopperBin::*)(void)const,void (RBX::HopperBin::*)(RBX::HopperBin::BinType)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x578ee4(bin: &crate::instance::HopperBin) -> i32 {
    // IDA 0x578ee4 `GetSetImpl<getBinType, setBinType>::getValue`: invokes
    // the member getter (`getBinType`, 0x573688) through the bound
    // member-function pointer. Same shape as 0x56ec80.
    stub_0x573688(bin)
}

// 0x578f04 — __ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinENS2_7BinTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::GetSetImpl<RBX::HopperBin::BinType (RBX::HopperBin::*)(void)const,void (RBX::HopperBin::*)(RBX::HopperBin::BinType)>::setValue(RBX::Reflection::DescribedBase *,RBX::HopperBin::BinType const&)const")]
pub fn stub_0x578f04(bin: &mut crate::instance::HopperBin, value: i32) {
    // IDA 0x578f04 `GetSetImpl<getBinType, setBinType>::setValue`: invokes
    // the member setter (`setBinType`, 0x571428) through the bound
    // member-function pointer. Same shape as 0x56eca0.
    stub_0x571428(bin, value);
}

// 0x578f28 — __ZN3RBX10Reflection14PropDescriptorINS_12BackpackItemENS_9TextureIdEEC2IMS2_KFKS3_vEMS2_FvRS6_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BackpackItem,RBX::TextureId>::PropDescriptor<RBX::TextureId const (RBX::BackpackItem::*)(void)const,void (RBX::BackpackItem::*)(RBX::TextureId const&)>(char const*,char const*,RBX::TextureId const (RBX::BackpackItem::*)(void)const,void (RBX::BackpackItem::*)(RBX::TextureId const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x578f28(name: &str, category: &str) -> BackpackTextureProp {
    // IDA 0x578f28 `PropDescriptor<BackpackItem, TextureId>::C2`: stores the
    // name/category words with the `getTextureId`/`setTextureId` member
    // pair; the pair collapses into direct `BackpackItem` texture access,
    // as in 0x573664.
    BackpackTextureProp {
        name: name.to_string(),
        category: category.to_string(),
    }
}

// 0x57903c — __ZN3RBX10Reflection14PropDescriptorINS_12BackpackItemENS_9TextureIdEED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BackpackItem,RBX::TextureId>::~PropDescriptor()")]
pub use rbx_reflection::generated::stub_0x57903c as stub_0x57903c;

// 0x579068 — __ZNK3RBX10Reflection14PropDescriptorINS_12BackpackItemENS_9TextureIdEE10GetSetImplIMS2_KFKS3_vEMS2_FvRS6_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BackpackItem,RBX::TextureId>::GetSetImpl<RBX::TextureId const (RBX::BackpackItem::*)(void)const,void (RBX::BackpackItem::*)(RBX::TextureId const&)>::isReadOnly(void)const")]
pub use rbx_reflection::generated::stub_0x579068 as stub_0x579068;

// 0x57906c — __ZNK3RBX10Reflection14PropDescriptorINS_12BackpackItemENS_9TextureIdEE10GetSetImplIMS2_KFKS3_vEMS2_FvRS6_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BackpackItem,RBX::TextureId>::GetSetImpl<RBX::TextureId const (RBX::BackpackItem::*)(void)const,void (RBX::BackpackItem::*)(RBX::TextureId const&)>::isWriteOnly(void)const")]
pub use rbx_reflection::generated::stub_0x57906c as stub_0x57906c;

// 0x579070 — __ZNK3RBX10Reflection14PropDescriptorINS_12BackpackItemENS_9TextureIdEE10GetSetImplIMS2_KFKS3_vEMS2_FvRS6_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BackpackItem,RBX::TextureId>::GetSetImpl<RBX::TextureId const (RBX::BackpackItem::*)(void)const,void (RBX::BackpackItem::*)(RBX::TextureId const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x579070(item: &crate::instance::BackpackItem) -> BackpackTextureId {
    // IDA 0x579070 `GetSetImpl<getTextureId, setTextureId>::getValue`:
    // invokes the member getter (`getTextureId`, 0x5713d0) through the
    // bound member-function pointer; delegates to it.
    stub_0x5713d0(item)
}

// 0x579098 — __ZNK3RBX10Reflection14PropDescriptorINS_12BackpackItemENS_9TextureIdEE10GetSetImplIMS2_KFKS3_vEMS2_FvRS6_EE8setValueEPNS0_13DescribedBaseES9_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BackpackItem,RBX::TextureId>::GetSetImpl<RBX::TextureId const (RBX::BackpackItem::*)(void)const,void (RBX::BackpackItem::*)(RBX::TextureId const&)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextureId const&)const")]
pub fn stub_0x579098(item: &mut crate::instance::BackpackItem, texture: &BackpackTextureId) -> bool {
    // IDA 0x579098 `GetSetImpl<getTextureId, setTextureId>::setValue`:
    // invokes the member setter (`setTextureId`, 0x5713e8) through the
    // bound member-function pointer; delegates to it.
    stub_0x5713e8(item, texture)
}

// 0x5790bc — __ZNSt6vectorIN3RBX9HopperBin7BinTypeESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>::resize(unsigned long,RBX::HopperBin::BinType)")]
pub fn stub_0x5790bc() -> ! {
    // POOL-CORE: `std::vector<BinType>` resize machinery backing
    // reflection's `EnumDesc<BinType>` runtime table; collapses into `Vec`
    // in the owning crate.
    todo!("0x5790bc std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>::resize(unsigned long,RBX::HopperBin::BinType)")
}

// 0x5790f0 — __ZNSt3mapIPKN3RBX4NameENS0_9HopperBin7BinTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::HopperBin::BinType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x5790f0() -> ! {
    // POOL-CORE: `std::map<Name const*, BinType>` subscript backing
    // reflection's `EnumDesc<BinType>` name table; collapses into map ops
    // in the owning crate.
    todo!("0x5790f0 std::map<RBX::Name const*,RBX::HopperBin::BinType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>>::operator[](RBX::Name const* const&)")
}

// 0x579148 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9HopperBin7BinTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HopperBin::BinType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>,std::pair<RBX::Name const* const,RBX::HopperBin::BinType> const&)")]
pub fn stub_0x579148() -> ! {
    // POOL-CORE: `std::_Rb_tree` node insert backing the `BinType` name
    // map; tree machinery lives in the owning crate.
    todo!("0x579148 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HopperBin::BinType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>,std::pair<RBX::Name const* const,RBX::HopperBin::BinType> const&)")
}

// 0x5791fc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9HopperBin7BinTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HopperBin::BinType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::HopperBin::BinType> const&)")]
pub fn stub_0x5791fc() -> ! {
    // POOL-CORE: `std::_Rb_tree` positional insert; see 0x579148.
    todo!("0x5791fc std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HopperBin::BinType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::HopperBin::BinType> const&)")
}

// 0x579254 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9HopperBin7BinTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HopperBin::BinType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::HopperBin::BinType> const&)")]
pub fn stub_0x579254() -> ! {
    // POOL-CORE: `std::_Rb_tree` value insert; see 0x579148.
    todo!("0x579254 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HopperBin::BinType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>,std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::HopperBin::BinType> const&)")
}

// 0x5792bc — __ZNSt6vectorIN3RBX9HopperBin7BinTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::HopperBin::BinType*,std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>>,unsigned long,RBX::HopperBin::BinType const&)")]
pub fn stub_0x5792bc() -> ! {
    // POOL-CORE: `std::vector<BinType>` fill-insert; see 0x5790bc.
    todo!("0x5792bc std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::HopperBin::BinType*,std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>>,unsigned long,RBX::HopperBin::BinType const&)")
}

// 0x57944c — __ZNSt12_Vector_baseIN3RBX9HopperBin7BinTypeESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>::_M_allocate(unsigned long)")]
pub fn stub_0x57944c() -> ! {
    // POOL-CORE: `std::_Vector_base<BinType>` allocate; see 0x5790bc.
    todo!("0x57944c std::_Vector_base<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>::_M_allocate(unsigned long)")
}

// 0x579464 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9HopperBin7BinTypeES6_EET0_T_S8_S7_
#[doc(alias = "RBX::HopperBin::BinType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::HopperBin::BinType *,RBX::HopperBin::BinType *>(RBX::HopperBin::BinType *,RBX::HopperBin::BinType *,RBX::HopperBin::BinType *)")]
pub fn stub_0x579464() -> ! {
    // POOL-CORE: `std::__copy_backward` over `BinType`; see 0x5790bc.
    todo!("0x579464 RBX::HopperBin::BinType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::HopperBin::BinType *,RBX::HopperBin::BinType *,RBX::HopperBin::BinType *>(RBX::HopperBin::BinType *,RBX::HopperBin::BinType *,RBX::HopperBin::BinType *)")
}

// 0x5794a0 — __ZNSt6vectorIN3RBX9HopperBin7BinTypeESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>::push_back(RBX::HopperBin::BinType const&)")]
pub fn stub_0x5794a0() -> ! {
    // POOL-CORE: `std::vector<BinType>` push-back; see 0x5790bc.
    todo!("0x5794a0 std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>::push_back(RBX::HopperBin::BinType const&)")
}

// 0x5794c8 — __ZNSt6vectorIN3RBX9HopperBin7BinTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::HopperBin::BinType*,std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>>,RBX::HopperBin::BinType const&)")]
pub fn stub_0x5794c8() -> ! {
    // POOL-CORE: `std::vector<BinType>` insert-aux; see 0x5790bc.
    todo!("0x5794c8 std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::HopperBin::BinType*,std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>>,RBX::HopperBin::BinType const&)")
}

// 0x5795ac — __ZN3RBX9HopperBinD2Ev
#[doc(alias = "RBX::HopperBin::~HopperBin()")]
pub fn stub_0x5795ac(_bin: &mut crate::instance::HopperBin) {
    // IDA 0x5795ac (decompiled): `HopperBin::D2` — vtable resets
    // (0x5795da-0x579604), `signal<void()>::disconnectAll` + release
    // (0x579636-0x579644), the two `remote_signal` member dtors
    // (0x579652-0x579660), weak release (0x579666-0x579670), then the
    // `BackpackItem`/`GuiItem` teardown (0x579684-0x5796d2). The modeled
    // members are plain words; the replicator connections live in core.
    // Drop glue — no-op. The `D1` (0x573a5c) forwards here.
}

// 0x579f70 — __ZN3RBX17ICharacterSubjectC2Ev
#[doc(alias = "RBX::ICharacterSubject::ICharacterSubject(void)")]
pub fn stub_0x579f70() -> ! {
    // BLOCKED: needs camera-subject (G3D/Workspace) infra — initializes the
    // focus/distance state words (decompiled 0x579f70)
    todo!("0x579f70 RBX::ICharacterSubject::ICharacterSubject(void)")
}

// 0x579fcc — __ZN3RBX17ICharacterSubject10initCameraERN3G3D7Vector3ERNS1_15CoordinateFrameE
#[doc(alias = "RBX::ICharacterSubject::initCamera(G3D::Vector3 &,G3D::CoordinateFrame &)")]
pub fn stub_0x579fcc() -> ! {
    // BLOCKED: needs camera (G3D::Vector3/CoordinateFrame) + Workspace infra
    todo!("0x579fcc RBX::ICharacterSubject::initCamera(G3D::Vector3 &,G3D::CoordinateFrame &)")
}

// 0x57a09c — __ZNK3RBX17ICharacterSubject13isFirstPersonEv
#[doc(alias = "RBX::ICharacterSubject::isFirstPerson(void)const")]
pub fn stub_0x57a09c() -> ! {
    // BLOCKED: needs camera-subject infra — reads float at `+28 < 4.5`
    // (decompiled 0x57a09c); the subject state word is unmodeled
    todo!("0x57a09c RBX::ICharacterSubject::isFirstPerson(void)const")
}

// 0x57a0b4 — __ZN3RBX17ICharacterSubject20stepLocationAndFocusERN3G3D7Vector3ERNS1_15CoordinateFrameEd
#[doc(alias = "RBX::ICharacterSubject::stepLocationAndFocus(G3D::Vector3 &,G3D::CoordinateFrame &,double)")]
pub fn stub_0x57a0b4() -> ! {
    // BLOCKED: needs camera (G3D) + Workspace infra
    todo!("0x57a0b4 RBX::ICharacterSubject::stepLocationAndFocus(G3D::Vector3 &,G3D::CoordinateFrame &,double)")
}

// 0x57a17c — __ZN3RBX17ICharacterSubject12doCameraMoveERN3G3D7Vector3ERNS1_15CoordinateFrameEd
#[doc(alias = "RBX::ICharacterSubject::doCameraMove(G3D::Vector3 &,G3D::CoordinateFrame &,double)")]
pub fn stub_0x57a17c() -> ! {
    // BLOCKED: needs camera (G3D) + Workspace infra
    todo!("0x57a17c RBX::ICharacterSubject::doCameraMove(G3D::Vector3 &,G3D::CoordinateFrame &,double)")
}

// 0x57a6c8 — __ZN3RBX17ICharacterSubject15doCameraOccludeERN3G3D7Vector3ERKNS1_15CoordinateFrameEd
#[doc(alias = "RBX::ICharacterSubject::doCameraOcclude(G3D::Vector3 &,G3D::CoordinateFrame const&,double)")]
pub fn stub_0x57a6c8() -> ! {
    // BLOCKED: needs camera occlusion (G3D) + Workspace infra
    todo!("0x57a6c8 RBX::ICharacterSubject::doCameraOcclude(G3D::Vector3 &,G3D::CoordinateFrame const&,double)")
}

// 0x57ab88 — __ZNK3RBX17ICharacterSubject19getNearPlaneCornersERN5boost5arrayIN3G3D7Vector3ELm4EEE
// was: RBX::ICharacterSubject::getNearPlaneCorners(boost::array<G3D::Vector3,4ul> &)const
#[doc(alias = "RBX::ICharacterSubject::getNearPlaneCorners(boost::array<G3D::Vector3,4ul> &)const")]
pub fn stub_0x57ab88() -> ! {
    // BLOCKED: needs camera frustum (G3D) infra
    todo!("0x57ab88 RBX::ICharacterSubject::getNearPlaneCorners(boost::array<G3D::Vector3,4ul> &)const")
}

// 0x57ad58 — __ZN3RBX17ICharacterSubject16getHalfDistancesERN5boost5arrayIfLm4EEERKN3G3D7Vector3ERKNS5_15CoordinateFrameE
// was: RBX::ICharacterSubject::getHalfDistances(boost::array<float,4ul> &,G3D::Vector3 const&,G3D::CoordinateFrame const&)
#[doc(alias = "RBX::ICharacterSubject::getHalfDistances(boost::array<float,4ul> &,G3D::Vector3 const&,G3D::CoordinateFrame const&)")]
pub fn stub_0x57ad58() -> ! {
    // BLOCKED: needs camera (G3D) infra
    todo!("0x57ad58 RBX::ICharacterSubject::getHalfDistances(boost::array<float,4ul> &,G3D::Vector3 const&,G3D::CoordinateFrame const&)")
}

// 0x57b03c — __ZN3RBX17ICharacterSubject22characterOcclusionTestERKN3G3D7Vector3ERKNS1_15CoordinateFrameE
#[doc(alias = "RBX::ICharacterSubject::characterOcclusionTest(G3D::Vector3 const&,G3D::CoordinateFrame const&)")]
pub fn stub_0x57b03c() -> ! {
    // BLOCKED: needs camera occlusion (G3D) + Workspace infra
    todo!("0x57b03c RBX::ICharacterSubject::characterOcclusionTest(G3D::Vector3 const&,G3D::CoordinateFrame const&)")
}

// 0x57b5e4 — __ZN3RBX17ICharacterSubject4zoomEfRN3G3D15CoordinateFrameES3_
#[doc(alias = "RBX::ICharacterSubject::zoom(float,G3D::CoordinateFrame &,G3D::CoordinateFrame &)")]
pub fn stub_0x57b5e4() -> ! {
    // BLOCKED: needs camera (G3D) infra
    todo!("0x57b5e4 RBX::ICharacterSubject::zoom(float,G3D::CoordinateFrame &,G3D::CoordinateFrame &)")
}

// 0x57bbe0 — __ZN3RBX17ICharacterSubject17onCameraHeartbeatERKN3G3D7Vector3ES4_
#[doc(alias = "RBX::ICharacterSubject::onCameraHeartbeat(G3D::Vector3 const&,G3D::Vector3 const&)")]
pub fn stub_0x57bbe0() -> ! {
    // BLOCKED: needs camera heartbeat (Workspace) infra
    todo!("0x57bbe0 RBX::ICharacterSubject::onCameraHeartbeat(G3D::Vector3 const&,G3D::Vector3 const&)")
}

// 0x57bd7c — __ZN3RBX17ICharacterSubject13setCameraModeENS_6Camera10CameraModeE
#[doc(alias = "RBX::ICharacterSubject::setCameraMode(RBX::Camera::CameraMode)")]
pub fn stub_0x57bd7c() -> ! {
    // BLOCKED: needs camera-mode (Camera) + subject infra
    todo!("0x57bd7c RBX::ICharacterSubject::setCameraMode(RBX::Camera::CameraMode)")
}

// 0x57bf9c — __ZN3RBX10IEquipableC2Ev
#[doc(alias = "RBX::IEquipable::IEquipable(void)")]
pub fn stub_0x57bf9c() -> IEquipable {
    // IDA 0x57bf9c (decompiled): `IEquipable::C2` — installs the vtable
    // word (0x57bfac) and zeroes the weld words at `+4`/`+8`
    // (0x57bfae-0x57bfb0). The vtable collapses; the observable state is
    // the empty weld.
    IEquipable::default()
}

// 0x57bfb4 — __ZN3RBX10IEquipableD0Ev
#[doc(alias = "RBX::IEquipable::~IEquipable()")]
pub fn stub_0x57bfb4(_equip: &mut IEquipable) {
    // IDA 0x57bfb4: `IEquipable::D0` — runs the `D1` body then releases
    // storage; same release as 0x57c054 — the weld link drops.
}

// 0x57c054 — __ZN3RBX10IEquipableD1Ev
#[doc(alias = "RBX::IEquipable::~IEquipable()")]
pub fn stub_0x57c054(equip: &mut IEquipable) {
    // IDA 0x57c054: `IEquipable::D1` — same vtable-reset + weld-release
    // shape as the `D2` (0x57c058); dropping the link is the same release.
    equip.weld = None;
}

// 0x57c058 — __ZN3RBX10IEquipableD2Ev
#[doc(alias = "RBX::IEquipable::~IEquipable()")]
pub fn stub_0x57c058(equip: &mut IEquipable) {
    // IDA 0x57c058 (decompiled): `IEquipable::D2` — vtable reset (0x57c09c),
    // `ReleaseAssert(!weld)` (`IEquipable.cpp:18`, 0x57c0a2-0x57c10e), then
    // releases the shared weld at `+8` (0x57c10e-0x57c11a). The live build
    // keeps the link empty at destroy; clearing it is the same release
    // (the assert is a debug-only gate on model state).
    equip.weld = None;
}

// 0x57c39c — __ZN5boost10shared_ptrIN3RBX4WeldEEaSERKS3_
// was: boost::shared_ptr<RBX::Weld>::operator=(boost::shared_ptr<RBX::Weld> const&)
#[doc(alias = "rbx_core::SharedPtr<RBX::Weld>::operator=(rbx_core::SharedPtr<RBX::Weld> const&)")]
pub fn stub_0x57c39c(dst: &mut SharedPtr<Weld>, src: &SharedPtr<Weld>) {
    // IDA 0x57c39c (decompiled): `shared_ptr<Weld>::operator=` — retains
    // the new count (0x57c3b0), installs it (0x57c3ba-0x57c3c2), releases
    // the old (0x57c3c6-0x57c3c8). Clone-assign is the same retain/release.
    *dst = SharedPtr::clone(src);
}

// 0x57c644 — __ZN3RBX14GuiImageButtonC2Ev
#[doc(alias = "RBX::GuiImageButton::GuiImageButton(void)")]
pub fn stub_0x57c644() -> ! {
    todo!("0x57c644 RBX::GuiImageButton::GuiImageButton(void)")
}

// 0x57c894 — __ZN3RBX14GuiImageButtonC1EPNS_4VerbE
#[doc(alias = "RBX::GuiImageButton::GuiImageButton(RBX::Verb *)")]
pub fn stub_0x57c894() -> ! {
    todo!("0x57c894 RBX::GuiImageButton::GuiImageButton(RBX::Verb *)")
}

// 0x57c898 — __ZN3RBX14GuiImageButtonC2EPNS_4VerbE
#[doc(alias = "RBX::GuiImageButton::GuiImageButton(RBX::Verb *)")]
pub fn stub_0x57c898() -> ! {
    todo!("0x57c898 RBX::GuiImageButton::GuiImageButton(RBX::Verb *)")
}

// 0x57caf4 — __ZN3RBX14GuiImageButton8setImageENS_9TextureIdE
#[doc(alias = "RBX::GuiImageButton::setImage(RBX::TextureId)")]
pub fn stub_0x57caf4() -> ! {
    todo!("0x57caf4 RBX::GuiImageButton::setImage(RBX::TextureId)")
}

// 0x57cb34 — __ZThn800_N3RBX14GuiImageButton8setImageENS_9TextureIdE
#[doc(alias = "non-virtual thunk toRBX::GuiImageButton::setImage(RBX::TextureId)")]
pub fn stub_0x57cb34() -> ! {
    todo!("0x57cb34 non-virtual thunk toRBX::GuiImageButton::setImage(RBX::TextureId)")
}

// 0x57cb3c — __ZN3RBX14GuiImageButton18setImageRectOffsetEN3G3D7Vector2E
#[doc(alias = "RBX::GuiImageButton::setImageRectOffset(G3D::Vector2)")]
pub fn stub_0x57cb3c() -> ! {
    todo!("0x57cb3c RBX::GuiImageButton::setImageRectOffset(G3D::Vector2)")
}

// 0x57cb88 — __ZThn800_N3RBX14GuiImageButton18setImageRectOffsetEN3G3D7Vector2E
#[doc(alias = "non-virtual thunk toRBX::GuiImageButton::setImageRectOffset(G3D::Vector2)")]
pub fn stub_0x57cb88() -> ! {
    todo!("0x57cb88 non-virtual thunk toRBX::GuiImageButton::setImageRectOffset(G3D::Vector2)")
}

// 0x57cb90 — __ZN3RBX14GuiImageButton16setImageRectSizeEN3G3D7Vector2E
#[doc(alias = "RBX::GuiImageButton::setImageRectSize(G3D::Vector2)")]
pub fn stub_0x57cb90() -> ! {
    todo!("0x57cb90 RBX::GuiImageButton::setImageRectSize(G3D::Vector2)")
}

// 0x57cbdc — __ZThn800_N3RBX14GuiImageButton16setImageRectSizeEN3G3D7Vector2E
#[doc(alias = "non-virtual thunk toRBX::GuiImageButton::setImageRectSize(G3D::Vector2)")]
pub fn stub_0x57cbdc() -> ! {
    todo!("0x57cbdc non-virtual thunk toRBX::GuiImageButton::setImageRectSize(G3D::Vector2)")
}

// 0x57cbe4 — __ZN3RBX14GuiImageButton8render2dEPNS_5AdornE
#[doc(alias = "RBX::GuiImageButton::render2d(RBX::Adorn *)")]
pub fn stub_0x57cbe4() -> ! {
    todo!("0x57cbe4 RBX::GuiImageButton::render2d(RBX::Adorn *)")
}

// 0x57cd38 — __ZThn96_N3RBX14GuiImageButton8render2dEPNS_5AdornE
#[doc(alias = "non-virtual thunk toRBX::GuiImageButton::render2d(RBX::Adorn *)")]
pub fn stub_0x57cd38() -> ! {
    todo!("0x57cd38 non-virtual thunk toRBX::GuiImageButton::render2d(RBX::Adorn *)")
}

// 0x57cd40 — __ZN3RBX10Reflection14PropDescriptorINS_14GuiImageButtonENS_9TextureIdEED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiImageButton,RBX::TextureId>::~PropDescriptor()")]
pub fn stub_0x57cd40() -> ! {
    todo!("0x57cd40 RBX::Reflection::PropDescriptor<RBX::GuiImageButton,RBX::TextureId>::~PropDescriptor()")
}

// 0x57cd64 — __ZN3RBX14GuiImageButtonD1Ev
#[doc(alias = "RBX::GuiImageButton::~GuiImageButton()")]
pub fn stub_0x57cd64() -> ! {
    todo!("0x57cd64 RBX::GuiImageButton::~GuiImageButton()")
}

// 0x57ce5c — __ZN3RBX14GuiImageButtonD0Ev
#[doc(alias = "RBX::GuiImageButton::~GuiImageButton()")]
pub fn stub_0x57ce5c() -> ! {
    todo!("0x57ce5c RBX::GuiImageButton::~GuiImageButton()")
}

// 0x57cf74 — __ZThn32_N3RBX14GuiImageButtonD1Ev
#[doc(alias = "non-virtual thunk toRBX::GuiImageButton::~GuiImageButton()")]
pub fn stub_0x57cf74() -> ! {
    todo!("0x57cf74 non-virtual thunk toRBX::GuiImageButton::~GuiImageButton()")
}

// 0x57d06c — __ZThn32_N3RBX14GuiImageButtonD0Ev
#[doc(alias = "non-virtual thunk toRBX::GuiImageButton::~GuiImageButton()")]
pub fn stub_0x57d06c() -> ! {
    todo!("0x57d06c non-virtual thunk toRBX::GuiImageButton::~GuiImageButton()")
}

// 0x57d188 — __ZThn36_N3RBX14GuiImageButtonD1Ev
#[doc(alias = "non-virtual thunk toRBX::GuiImageButton::~GuiImageButton()")]
pub fn stub_0x57d188() -> ! {
    todo!("0x57d188 non-virtual thunk toRBX::GuiImageButton::~GuiImageButton()")
}

// 0x57d280 — __ZThn36_N3RBX14GuiImageButtonD0Ev
#[doc(alias = "non-virtual thunk toRBX::GuiImageButton::~GuiImageButton()")]
pub fn stub_0x57d280() -> ! {
    todo!("0x57d280 non-virtual thunk toRBX::GuiImageButton::~GuiImageButton()")
}

// 0x57d6ac — __ZN3RBX4Name13callDoDeclareILZNS_15sGuiImageButtonEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_15sGuiImageButtonEEEEvv")]
pub fn stub_0x57d6ac() -> ! {
    todo!("0x57d6ac __ZN3RBX4Name13callDoDeclareILZNS_15sGuiImageButtonEEEEvv")
}

// 0x57d6b0 — __ZN3RBX4Name9doDeclareILZNS_15sGuiImageButtonEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sGuiImageButtonEEEERKS0_v")]
pub fn stub_0x57d6b0() -> ! {
    todo!("0x57d6b0 __ZN3RBX4Name9doDeclareILZNS_15sGuiImageButtonEEEERKS0_v")
}

// 0x57da48 — __ZN3RBX10Reflection14PropDescriptorINS_14GuiImageButtonENS_9TextureIdEEC2IMNS_13GuiImageMixinEKFS3_vEMS2_FvS3_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiImageButton,RBX::TextureId>::PropDescriptor<RBX::TextureId (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(RBX::TextureId)>(char const*,char const*,RBX::TextureId (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(RBX::TextureId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x57da48() -> ! {
    todo!("0x57da48 RBX::Reflection::PropDescriptor<RBX::GuiImageButton,RBX::TextureId>::PropDescriptor<RBX::TextureId (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(RBX::TextureId)>(char const*,char const*,RBX::TextureId (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(RBX::TextureId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x57db5c — __ZN3RBX10Reflection14PropDescriptorINS_14GuiImageButtonENS_9TextureIdEED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiImageButton,RBX::TextureId>::~PropDescriptor()")]
pub fn stub_0x57db5c() -> ! {
    todo!("0x57db5c RBX::Reflection::PropDescriptor<RBX::GuiImageButton,RBX::TextureId>::~PropDescriptor()")
}

// 0x57db88 — __ZNK3RBX10Reflection14PropDescriptorINS_14GuiImageButtonENS_9TextureIdEE10GetSetImplIMNS_13GuiImageMixinEKFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiImageButton,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(RBX::TextureId)>::isReadOnly(void)const")]
pub fn stub_0x57db88() -> ! {
    todo!("0x57db88 RBX::Reflection::PropDescriptor<RBX::GuiImageButton,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(RBX::TextureId)>::isReadOnly(void)const")
}

// 0x57db8c — __ZNK3RBX10Reflection14PropDescriptorINS_14GuiImageButtonENS_9TextureIdEE10GetSetImplIMNS_13GuiImageMixinEKFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiImageButton,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(RBX::TextureId)>::isWriteOnly(void)const")]
pub fn stub_0x57db8c() -> ! {
    todo!("0x57db8c RBX::Reflection::PropDescriptor<RBX::GuiImageButton,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(RBX::TextureId)>::isWriteOnly(void)const")
}

// 0x57db90 — __ZNK3RBX10Reflection14PropDescriptorINS_14GuiImageButtonENS_9TextureIdEE10GetSetImplIMNS_13GuiImageMixinEKFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiImageButton,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(RBX::TextureId)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x57db90() -> ! {
    todo!("0x57db90 RBX::Reflection::PropDescriptor<RBX::GuiImageButton,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(RBX::TextureId)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x57dbc8 — __ZNK3RBX10Reflection14PropDescriptorINS_14GuiImageButtonENS_9TextureIdEE10GetSetImplIMNS_13GuiImageMixinEKFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiImageButton,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(RBX::TextureId)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextureId const&)const")]
pub fn stub_0x57dbc8() -> ! {
    todo!("0x57dbc8 RBX::Reflection::PropDescriptor<RBX::GuiImageButton,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(RBX::TextureId)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextureId const&)const")
}

// 0x57e37c — __ZN3RBX10ImageLabelC1Ev
#[doc(alias = "RBX::ImageLabel::ImageLabel(void)")]
pub fn stub_0x57e37c() -> ! {
    todo!("0x57e37c RBX::ImageLabel::ImageLabel(void)")
}

// 0x57e380 — __ZN3RBX10ImageLabelC2Ev
#[doc(alias = "RBX::ImageLabel::ImageLabel(void)")]
pub fn stub_0x57e380() -> ! {
    todo!("0x57e380 RBX::ImageLabel::ImageLabel(void)")
}

// 0x57e5c8 — __ZN3RBX10ImageLabel8setImageENS_9TextureIdE
#[doc(alias = "RBX::ImageLabel::setImage(RBX::TextureId)")]
pub fn stub_0x57e5c8() -> ! {
    todo!("0x57e5c8 RBX::ImageLabel::setImage(RBX::TextureId)")
}

// 0x57e608 — __ZThn536_N3RBX10ImageLabel8setImageENS_9TextureIdE
#[doc(alias = "non-virtual thunk toRBX::ImageLabel::setImage(RBX::TextureId)")]
pub fn stub_0x57e608() -> ! {
    todo!("0x57e608 non-virtual thunk toRBX::ImageLabel::setImage(RBX::TextureId)")
}

// 0x57e610 — __ZN3RBX10ImageLabel18setImageRectOffsetEN3G3D7Vector2E
#[doc(alias = "RBX::ImageLabel::setImageRectOffset(G3D::Vector2)")]
pub fn stub_0x57e610() -> ! {
    todo!("0x57e610 RBX::ImageLabel::setImageRectOffset(G3D::Vector2)")
}

// 0x57e65c — __ZThn536_N3RBX10ImageLabel18setImageRectOffsetEN3G3D7Vector2E
#[doc(alias = "non-virtual thunk toRBX::ImageLabel::setImageRectOffset(G3D::Vector2)")]
pub fn stub_0x57e65c() -> ! {
    todo!("0x57e65c non-virtual thunk toRBX::ImageLabel::setImageRectOffset(G3D::Vector2)")
}

// 0x57e664 — __ZN3RBX10ImageLabel16setImageRectSizeEN3G3D7Vector2E
#[doc(alias = "RBX::ImageLabel::setImageRectSize(G3D::Vector2)")]
pub fn stub_0x57e664() -> ! {
    todo!("0x57e664 RBX::ImageLabel::setImageRectSize(G3D::Vector2)")
}

// 0x57e6b0 — __ZThn536_N3RBX10ImageLabel16setImageRectSizeEN3G3D7Vector2E
#[doc(alias = "non-virtual thunk toRBX::ImageLabel::setImageRectSize(G3D::Vector2)")]
pub fn stub_0x57e6b0() -> ! {
    todo!("0x57e6b0 non-virtual thunk toRBX::ImageLabel::setImageRectSize(G3D::Vector2)")
}

// 0x57e6b8 — __ZN3RBX10ImageLabel8render2dEPNS_5AdornE
#[doc(alias = "RBX::ImageLabel::render2d(RBX::Adorn *)")]
pub fn stub_0x57e6b8() -> ! {
    todo!("0x57e6b8 RBX::ImageLabel::render2d(RBX::Adorn *)")
}

// 0x57e7b4 — __ZThn96_N3RBX10ImageLabel8render2dEPNS_5AdornE
#[doc(alias = "non-virtual thunk toRBX::ImageLabel::render2d(RBX::Adorn *)")]
pub fn stub_0x57e7b4() -> ! {
    todo!("0x57e7b4 non-virtual thunk toRBX::ImageLabel::render2d(RBX::Adorn *)")
}

// 0x57e7bc — __ZN3RBX10ImageLabel18renderBackground2dEPNS_5AdornE
#[doc(alias = "RBX::ImageLabel::renderBackground2d(RBX::Adorn *)")]
pub fn stub_0x57e7bc() -> ! {
    todo!("0x57e7bc RBX::ImageLabel::renderBackground2d(RBX::Adorn *)")
}

// 0x57e7f0 — __ZThn96_N3RBX10ImageLabel18renderBackground2dEPNS_5AdornE
#[doc(alias = "non-virtual thunk toRBX::ImageLabel::renderBackground2d(RBX::Adorn *)")]
pub fn stub_0x57e7f0() -> ! {
    todo!("0x57e7f0 non-virtual thunk toRBX::ImageLabel::renderBackground2d(RBX::Adorn *)")
}

// 0x57e7f8 — __ZNK3RBX13GuiImageMixin8getImageEv
#[doc(alias = "RBX::GuiImageMixin::getImage(void)const")]
pub fn stub_0x57e7f8() -> ! {
    todo!("0x57e7f8 RBX::GuiImageMixin::getImage(void)const")
}

// 0x57e80c — __ZN3RBX10Reflection14PropDescriptorINS_10ImageLabelENS_9TextureIdEED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ImageLabel,RBX::TextureId>::~PropDescriptor()")]
pub fn stub_0x57e80c() -> ! {
    todo!("0x57e80c RBX::Reflection::PropDescriptor<RBX::ImageLabel,RBX::TextureId>::~PropDescriptor()")
}

// 0x57e830 — __ZN3RBX10ImageLabelD1Ev
#[doc(alias = "RBX::ImageLabel::~ImageLabel()")]
pub fn stub_0x57e830() -> ! {
    todo!("0x57e830 RBX::ImageLabel::~ImageLabel()")
}

// 0x57e928 — __ZN3RBX10ImageLabelD0Ev
#[doc(alias = "RBX::ImageLabel::~ImageLabel()")]
pub fn stub_0x57e928() -> ! {
    todo!("0x57e928 RBX::ImageLabel::~ImageLabel()")
}

// 0x57ea40 — __ZNK3RBX8GuiLabel9isGuiLeafEv
#[doc(alias = "RBX::GuiLabel::isGuiLeaf(void)const")]
pub fn stub_0x57ea40() -> ! {
    todo!("0x57ea40 RBX::GuiLabel::isGuiLeaf(void)const")
}

// 0x57ea44 — __ZThn32_N3RBX10ImageLabelD1Ev
#[doc(alias = "non-virtual thunk toRBX::ImageLabel::~ImageLabel()")]
pub fn stub_0x57ea44() -> ! {
    todo!("0x57ea44 non-virtual thunk toRBX::ImageLabel::~ImageLabel()")
}

// 0x57eb3c — __ZThn32_N3RBX10ImageLabelD0Ev
#[doc(alias = "non-virtual thunk toRBX::ImageLabel::~ImageLabel()")]
pub fn stub_0x57eb3c() -> ! {
    todo!("0x57eb3c non-virtual thunk toRBX::ImageLabel::~ImageLabel()")
}

// 0x57ec58 — __ZThn36_N3RBX10ImageLabelD1Ev
#[doc(alias = "non-virtual thunk toRBX::ImageLabel::~ImageLabel()")]
pub fn stub_0x57ec58() -> ! {
    todo!("0x57ec58 non-virtual thunk toRBX::ImageLabel::~ImageLabel()")
}

// 0x57ed50 — __ZThn36_N3RBX10ImageLabelD0Ev
#[doc(alias = "non-virtual thunk toRBX::ImageLabel::~ImageLabel()")]
pub fn stub_0x57ed50() -> ! {
    todo!("0x57ed50 non-virtual thunk toRBX::ImageLabel::~ImageLabel()")
}

// 0x57f0c8 — __ZN3RBX4Name13callDoDeclareILZNS_11sImageLabelEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sImageLabelEEEEvv")]
pub fn stub_0x57f0c8() -> ! {
    todo!("0x57f0c8 __ZN3RBX4Name13callDoDeclareILZNS_11sImageLabelEEEEvv")
}

// 0x57f0cc — __ZN3RBX4Name9doDeclareILZNS_11sImageLabelEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sImageLabelEEEERKS0_v")]
pub fn stub_0x57f0cc() -> ! {
    todo!("0x57f0cc __ZN3RBX4Name9doDeclareILZNS_11sImageLabelEEEERKS0_v")
}

// 0x57f464 — __ZN3RBX10Reflection14PropDescriptorINS_10ImageLabelENS_9TextureIdEEC2IMNS_13GuiImageMixinEKFS3_vEMS2_FvS3_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ImageLabel,RBX::TextureId>::PropDescriptor<RBX::TextureId (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(RBX::TextureId)>(char const*,char const*,RBX::TextureId (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(RBX::TextureId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x57f464() -> ! {
    todo!("0x57f464 RBX::Reflection::PropDescriptor<RBX::ImageLabel,RBX::TextureId>::PropDescriptor<RBX::TextureId (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(RBX::TextureId)>(char const*,char const*,RBX::TextureId (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(RBX::TextureId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x57f69c — __ZN3RBX10Reflection14PropDescriptorINS_10ImageLabelENS_9TextureIdEED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ImageLabel,RBX::TextureId>::~PropDescriptor()")]
pub fn stub_0x57f69c() -> ! {
    todo!("0x57f69c RBX::Reflection::PropDescriptor<RBX::ImageLabel,RBX::TextureId>::~PropDescriptor()")
}

// 0x57f6c8 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::isReadOnly(void)const")]
pub fn stub_0x57f6c8() -> ! {
    todo!("0x57f6c8 RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::isReadOnly(void)const")
}

// 0x57f6d8 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::isWriteOnly(void)const")]
pub fn stub_0x57f6d8() -> ! {
    todo!("0x57f6d8 RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::isWriteOnly(void)const")
}

// 0x57f6e8 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE11equalValuesEPKNS0_13DescribedBaseES6_
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x57f6e8() -> ! {
    todo!("0x57f6e8 RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

// 0x57f894 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x57f894() -> ! {
    todo!("0x57f894 RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}

#[cfg(test)]
mod batch_a_tests {
    use super::*;
    use crate::instance::{BackpackItem, HopperBin};

    #[test]
    fn bintype_tail_round_trip() {
        let mut bin = HopperBin::default();
        assert_eq!(stub_0x578d84(&bin), 0);
        assert_eq!(stub_0x578ee4(&bin), 0);
        assert!(stub_0x578d8c(&mut bin, 2));
        assert_eq!(bin.bin_type, 2);
        assert!(!stub_0x578d8c(&mut bin, 42));
        assert_eq!(bin.bin_type, 2);
        assert!(stub_0x578d50(&mut bin, 4));
        assert_eq!(bin.bin_type, 4);
        assert!(!stub_0x578d50(&mut bin, 9));
        assert_eq!(stub_0x578dd8(&bin), Some((4, "Hammer")));
        assert!(stub_0x578df8(&mut bin, "Grab"));
        assert_eq!(bin.bin_type, 2);
        assert!(!stub_0x578df8(&mut bin, "Nope"));
        assert!(stub_0x578e9c(&mut bin, 1));
        assert!(!stub_0x578e9c(&mut bin, -1));
        assert!(!stub_0x578e9c(&mut bin, 5));
        stub_0x578f04(&mut bin, 3);
        assert_eq!(bin.bin_type, 3);
        assert!(!stub_0x578edc());
        assert!(!stub_0x578ee0());
    }

    #[test]
    fn texture_getset_delegates() {
        let prop = stub_0x578f28("TextureId", "Appearance");
        assert_eq!(prop.name, "TextureId");
        let mut item = BackpackItem::default();
        let tex = BackpackTextureId {
            id: "rbxasset://Textures/sword.png".to_string(),
            tag: 7,
        };
        assert!(stub_0x579098(&mut item, &tex));
        assert_eq!(stub_0x579070(&item), tex);
        assert!(!stub_0x579068());
        assert!(!stub_0x57906c());
        stub_0x57903c();
        drop(prop);
    }

    #[test]
    fn hopperbin_d2_is_drop_glue() {
        let mut bin = HopperBin::default();
        bin.active = true;
        bin.bin_type = 2;
        stub_0x5795ac(&mut bin);
        assert!(bin.active);
    }
}

#[cfg(test)]
mod batch_b_tests {
    use super::*;
    use crate::instance::Weld;

    #[test]
    fn equipable_weld_link_lifecycle() {
        let mut equip = stub_0x57bf9c();
        assert!(equip.weld.is_none());
        let weld = SharedPtr::new(Weld::default());
        let mut slot = SharedPtr::new(Weld::default());
        stub_0x57c39c(&mut slot, &weld);
        assert!(SharedPtr::ptr_eq(&slot, &weld));
        equip.weld = Some(SharedPtr::clone(&weld));
        stub_0x57c054(&mut equip);
        assert!(equip.weld.is_none());
        equip.weld = Some(weld);
        stub_0x57c058(&mut equip);
        assert!(equip.weld.is_none());
        stub_0x57bfb4(&mut equip);
    }
}
