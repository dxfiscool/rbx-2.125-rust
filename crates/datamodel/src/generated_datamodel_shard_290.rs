// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: RBX::Instance|RBX::DataModel|RBX::Workspace (strict/loose) — filtered 10215 total, 0 remaining globally (all covered), 0 remaining in datamodel (strict) — using global gap filler EA-sorted asc distinct not yet in datamodel crate
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 120 stubs | range 0x4f8874..0x56cbb0 | EA-sorted asc distinct not yet in datamodel/src (global stub gaps exhausted, datamodel gap filler)
// Shard: generated_datamodel_shard_290 EA-sorted ascending next 120 datamodel gap filler after stub coverage (filtered exhausted)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; stripped from alias where needed

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;
use parking_lot::Mutex;
use std::sync::Arc;
use crate::instance::Handles;

/// Raw `RBX::NormalId` face tag crossing the `Handles` remote events (IDA
/// `0x569b7c`, `0x56a77c`); same storage-only treatment as
/// `FaceInstance::face` (IDA `0x4a94fc`).
pub type HandlesNormalId = u32;

/// Slot callback behind the 2-arg `Handles` event `(NormalId, float)` (IDA
/// `0x56b7c8` `signal_with_args<2>`).
pub type HandlesHandler2 = Arc<dyn Fn(HandlesNormalId, f32) + Send + Sync>;

/// Slot callback behind the 1-arg `Handles` event `(NormalId,)` (IDA
/// `0x56a77c` replicate path).
pub type HandlesHandler1 = Arc<dyn Fn(HandlesNormalId) + Send + Sync>;

/// Rust model of `rbx::signals::signal<void ()(RBX::NormalId, float)>` (IDA
/// `0x56b874` `disconnectAll` target): the slot list behind
/// `EventDescImpl<2, Handles, ...>`; `Mutex` replaces the member-signal word.
#[derive(Default)]
pub struct HandlesSignal2 {
    slots: Mutex<Vec<HandlesHandler2>>,
}

impl HandlesSignal2 {
    pub fn connect(&self, handler: HandlesHandler2) {
        self.slots.lock().push(handler);
    }
    pub fn emit(&self, normal: HandlesNormalId, value: f32) {
        let live = self.slots.lock().clone();
        for slot in &live {
            slot(normal, value);
        }
    }
    pub fn disconnect_all(&self) {
        self.slots.lock().clear();
    }
    pub fn len(&self) -> usize {
        self.slots.lock().len()
    }
}

/// Rust model of `rbx::signals::signal<void ()(RBX::NormalId)>` (IDA
/// `0x56adc8` `remove` family): 1-arg twin of `HandlesSignal2`.
#[derive(Default)]
pub struct HandlesSignal1 {
    slots: Mutex<Vec<HandlesHandler1>>,
}

impl HandlesSignal1 {
    pub fn connect(&self, handler: HandlesHandler1) {
        self.slots.lock().push(handler);
    }
    pub fn emit(&self, normal: HandlesNormalId) {
        let live = self.slots.lock().clone();
        for slot in &live {
            slot(normal);
        }
    }
    pub fn disconnect_all(&self) {
        self.slots.lock().clear();
    }
    pub fn len(&self) -> usize {
        self.slots.lock().len()
    }
}

/// Rust model of `RBX::Reflection::EventDescImpl<2, Handles, void
/// ()(NormalId, float), remote_signal, member>` (IDA `0x56b7c0`
/// `isBroadcast`, `0x56c9c0` `EventDesc::C2`): the member signal (`+40`) and
/// the broadcast flag (`+44 & 1`) plus the replication half invoked by
/// `sendEvent`/`replicateEvent`.
#[derive(Default)]
pub struct HandlesEvent2Desc {
    pub name: String,
    pub broadcast: bool,
    pub signal: HandlesSignal2,
    pub remote: HandlesSignal2,
}

/// Rust model of `RBX::Reflection::EventDescImpl<1, Handles, void
/// ()(NormalId), remote_signal, member>` (IDA `0x56a77c`): 1-arg twin of
/// `HandlesEvent2Desc`.
#[derive(Default)]
pub struct HandlesEvent1Desc {
    pub name: String,
    pub broadcast: bool,
    pub signal: HandlesSignal1,
    pub remote: HandlesSignal1,
}

/// Rust model of `RBX::Reflection::GenericSlotWrapper` restricted to the
/// 2-arg `Handles` slot (IDA `0x56b9a4` `execute2<NormalId, float>`): the
/// native handler stands in for the Lua frame until the script bridge exists.
pub struct HandlesSlotWrapper2 {
    pub handler: HandlesHandler2,
}

impl HandlesSlotWrapper2 {
    /// IDA `0x56b9a4`: packs the 2-`Variant` vector with the `NormalId`/`float`
    /// singletons, dispatches the wrapped slot (`*a1 + 8`), destroys the vector.
    pub fn execute2(&self, normal: HandlesNormalId, value: f32) {
        (self.handler)(normal, value);
    }
}

/// Rust model of `RBX::Reflection::BoundProp<int, Mutability1>` over
/// `RBX::Handles` (IDA `0x56b3ac`): the name/category words; the bound member
/// is `Handles::int_value`, so the member offset collapses.
pub struct HandlesIntProp {
    pub name: String,
    pub category: String,
}

// 0x4f8874 — __ZThn116_N3RBX10ForceFieldD0Ev
// type: void __fastcall(RBX::ForceField *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ForceField::~ForceField()")]
#[doc(alias = "__ZThn116_N3RBX10ForceFieldD0Ev")]
pub use rbx_core::generated_core_shard_bn::stub_4f8874 as stub_0x4f8874;

// 0x4f8ce8 — __ZN3RBX10ForceFieldD2Ev
// type: void __fastcall(RBX::ForceField *__hidden this)
#[doc(alias = "RBX::ForceField::~ForceField()")]
#[doc(alias = "__ZN3RBX10ForceFieldD2Ev")]
pub use rbx_core::generated_core_shard_bn::stub_4f8ce8 as stub_0x4f8ce8;

// 0x4f90d8 — __ZN3RBX5Frame8setStyleENS0_5StyleE
// type: _DWORD __fastcall(RBX::Frame *__hidden this, Style)
#[doc(alias = "RBX::Frame::setStyle(RBX::Frame::Style)")]
#[doc(alias = "__ZN3RBX5Frame8setStyleENS0_5StyleE")]
pub use rbx_core::generated_core_shard_bn::stub_4f90d8 as stub_0x4f90d8;

// 0x4f910c — __ZN3RBX10Reflection8EnumDescINS_5Frame5StyleEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Frame::Style>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_5Frame5StyleEEC1Ev")]
pub use rbx_reflection::generated::stub_0x4f910c as stub_0x4f910c;

// 0x4f9110 — __ZN3RBX10Reflection8EnumDescINS_5Frame5StyleEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Frame::Style>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_5Frame5StyleEEC2Ev")]
pub use rbx_reflection::generated::stub_0x4f9110 as stub_0x4f9110;

// 0x4f932c — __ZN3RBX5FrameC1Ev
// type: _DWORD __fastcall(RBX::Frame *__hidden this)
#[doc(alias = "RBX::Frame::Frame(void)")]
#[doc(alias = "__ZN3RBX5FrameC1Ev")]
pub use rbx_core::generated_core_shard_bn::stub_4f932c as stub_0x4f932c;

// 0x4f9330 — __ZN3RBX5FrameC2Ev
// type: _DWORD __fastcall(RBX::Frame *__hidden this)
#[doc(alias = "RBX::Frame::Frame(void)")]
#[doc(alias = "__ZN3RBX5FrameC2Ev")]
pub use rbx_core::generated_core_shard_bn::stub_4f9330 as stub_0x4f9330;

// 0x4f94b8 — __ZNK3RBX5Frame14getChildRect2DEv
// type: _DWORD __fastcall(RBX::Frame *__hidden this)
#[doc(alias = "RBX::Frame::getChildRect2D(void)const")]
#[doc(alias = "__ZNK3RBX5Frame14getChildRect2DEv")]
pub use rbx_core::generated_core_shard_bn::stub_4f94b8 as stub_0x4f94b8;

// 0x4f956c — __ZN3RBX5Frame8render2dEPNS_5AdornE
// type: void __fastcall(RBX::Frame *this, RBX::Adorn *)
#[doc(alias = "RBX::Frame::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX5Frame8render2dEPNS_5AdornE")]
pub use rbx_core::generated_core_shard_bn::stub_4f956c as stub_0x4f956c;

// 0x4f9978 — __ZThn96_N3RBX5Frame8render2dEPNS_5AdornE
// type: _DWORD __fastcall(RBX::Frame *__hidden this, RBX::Adorn *)
#[doc(alias = "non-virtual thunk toRBX::Frame::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZThn96_N3RBX5Frame8render2dEPNS_5AdornE")]
pub use rbx_core::generated_core_shard_bn::stub_4f9978 as stub_0x4f9978;

// 0x4f9980 — __ZNK3RBX5Frame8getStyleEv
// type: _DWORD __fastcall(RBX::Frame *__hidden this)
#[doc(alias = "RBX::Frame::getStyle(void)const")]
#[doc(alias = "__ZNK3RBX5Frame8getStyleEv")]
pub use rbx_core::generated_core_shard_bn::stub_4f9980 as stub_0x4f9980;

// 0x4f9988 — __ZN3RBX10Reflection18EnumPropDescriptorINS_5FrameENS2_5StyleEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Frame,RBX::Frame::Style>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_5FrameENS2_5StyleEED1Ev")]
pub use rbx_reflection::generated::stub_0x4f9988 as stub_0x4f9988;

// 0x4f99ac — __ZN3RBX10Reflection8EnumDescINS_5Frame5StyleEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Frame::Style>::addPair(RBX::Frame::Style,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_5Frame5StyleEE7addPairES3_PKc")]
pub use rbx_reflection::generated::stub_0x4f99ac as stub_0x4f99ac;

// 0x4f9f08 — __ZNSt6vectorIN3RBX5Frame5StyleESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>::resize(unsigned long,RBX::Frame::Style)")]
#[doc(alias = "__ZNSt6vectorIN3RBX5Frame5StyleESaIS2_EE6resizeEmS2_")]
pub use rbx_core::generated_core_shard_bn::stub_4f9f08 as stub_0x4f9f08;

// 0x4f9f3c — __ZNSt6vectorIN3RBX5Frame5StyleESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>::push_back(RBX::Frame::Style const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX5Frame5StyleESaIS2_EE9push_backERKS2_")]
pub use rbx_core::generated_core_shard_bn::stub_4f9f3c as stub_0x4f9f3c;

// 0x4f9f64 — __ZNSt3mapIPKN3RBX4NameENS0_5Frame5StyleESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::Frame::Style,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Frame::Style>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_5Frame5StyleESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
pub use rbx_core::generated_core_shard_bn::stub_4f9f64 as stub_0x4f9f64;

// 0x4f9fbc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Frame5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Frame::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Frame::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Frame::Style>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Frame::Style>>,std::pair<RBX::Name const* const,RBX::Frame::Style> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Frame5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
pub use rbx_core::generated_core_shard_bn::stub_4f9fbc as stub_0x4f9fbc;

// 0x4fa070 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Frame5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Frame::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Frame::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Frame::Style>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Frame::Style> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Frame5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
pub use rbx_core::generated_core_shard_bn::stub_4fa070 as stub_0x4fa070;

// 0x4fa0c8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Frame5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Frame::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Frame::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Frame::Style>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Frame::Style> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Frame5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
pub use rbx_core::generated_core_shard_bn::stub_4fa0c8 as stub_0x4fa0c8;

// 0x4fa130 — __ZNSt6vectorIN3RBX5Frame5StyleESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Frame::Style*,std::vector<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>>,RBX::Frame::Style const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX5Frame5StyleESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub use rbx_core::generated_core_shard_bn::stub_4fa130 as stub_0x4fa130;

// 0x568f7c — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEfEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEfEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED0Ev")]
pub use rbx_core::boost_core_i::stub_568f7c as stub_0x568f7c;

// 0x569050 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv")]
pub use rbx_core::boost_core_i::stub_569050 as stub_0x569050;

// 0x569058 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv")]
pub use rbx_core::boost_core_i::stub_569058 as stub_0x569058;

// 0x569060 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_7HandlesEFvNS4_8NormalIdEfEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv
// type: int(void)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>::operator()(void)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_7HandlesEFvNS4_8NormalIdEfEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv")]
pub use rbx_core::boost_core_i::stub_569060 as stub_0x569060;

// 0x569078 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED1Ev")]
pub use rbx_core::boost_core_i::stub_569078 as stub_0x569078;

// 0x5690a4 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED0Ev")]
pub use rbx_core::boost_core_i::stub_5690a4 as stub_0x5690a4;

// 0x569178 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEEEEEENS6_5list1INS6_5valueIPSF_EEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEEEEEENS6_5list1INS6_5valueIPSF_EEEEEEEENS0_10connectionERKT_")]
pub use rbx_core::boost_core_i::stub_569178 as stub_0x569178;

// 0x5691ec — __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE23listenerConnectionAddedEv
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>::listenerConnectionAdded(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE23listenerConnectionAddedEv")]
pub use rbx_core::generated_core_shard_ig::stub_5691ec as stub_0x5691ec;

// 0x569238 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED1Ev")]
pub use rbx_core::boost_core_i::stub_569238 as stub_0x569238;

// 0x569264 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED0Ev")]
pub use rbx_core::boost_core_i::stub_569264 as stub_0x569264;

// 0x569338 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv")]
pub use rbx_core::boost_core_i::stub_569338 as stub_0x569338;

// 0x569340 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv")]
pub use rbx_core::boost_core_i::stub_569340 as stub_0x569340;

// 0x569348 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_7HandlesEFvNS4_8NormalIdEEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv
// type: int(void)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>::operator()(void)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_7HandlesEFvNS4_8NormalIdEEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv")]
pub use rbx_core::boost_core_i::stub_569348 as stub_0x569348;

// 0x569360 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED1Ev")]
pub use rbx_core::boost_core_i::stub_569360 as stub_0x569360;

// 0x56938c — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED0Ev")]
pub use rbx_core::boost_core_i::stub_56938c as stub_0x56938c;

// 0x569460 — __ZN3rbx13remote_signalIFvN3RBX8NormalIdEfEEC2Ev
// type: int __fastcall(_DWORD *, int, int, int, int, int)
#[doc(alias = "rbx::remote_signal<void ()(RBX::NormalId,float)>::remote_signal(void)")]
#[doc(alias = "__ZN3rbx13remote_signalIFvN3RBX8NormalIdEfEEC2Ev")]
pub use rbx_core::generated_core_k::stub_569460 as stub_0x569460;

// 0x5695bc — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::disconnectAll(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13disconnectAllEv")]
pub use rbx_core::generated_core_i::stub_5695bc as stub_0x5695bc;

// 0x569734 — __ZN3rbx13remote_signalIFvN3RBX8NormalIdEEEC2Ev
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "rbx::remote_signal<void ()(RBX::NormalId)>::remote_signal(void)")]
#[doc(alias = "__ZN3rbx13remote_signalIFvN3RBX8NormalIdEEEC2Ev")]
pub use rbx_core::generated_core_k::stub_569734 as stub_0x569734;

// 0x569890 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::disconnectAll(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13disconnectAllEv")]
pub use rbx_core::generated_core_i::stub_569890 as stub_0x569890;

// 0x569a08 — __ZN3RBX19EventReplicatorImplILi2ENS_7HandlesEFvNS_8NormalIdEfEE21connectSignalListenerEv
#[doc(alias = "RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>::connectSignalListener(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi2ENS_7HandlesEFvNS_8NormalIdEfEE21connectSignalListenerEv")]
pub use rbx_core::generated_core_shard_ig::stub_569a08 as stub_0x569a08;

// 0x569afc — __ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEE12getSignalPtrEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>>::getSignalPtr(RBX::Reflection::EventSource *)")]
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEE12getSignalPtrEPNS0_11EventSourceE")]
pub use rbx_core::generated_core_watchdog_1788317087::stub_569afc as stub_0x569afc;

// 0x569b64 — __ZN3RBX19EventReplicatorImplILi2ENS_7HandlesEFvNS_8NormalIdEfEE25signalProducedIncrementedES2_f
#[doc(alias = "RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>::signalProducedIncremented(RBX::NormalId,float)")]
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi2ENS_7HandlesEFvNS_8NormalIdEfEE25signalProducedIncrementedES2_f")]
pub use rbx_core::generated_core_shard_ig::stub_569b64 as stub_0x569b64;

// 0x569b7c — __ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEE14replicateEventEPNS0_11EventSourceES3_f
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<2,RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>>::replicateEvent(RBX::Reflection::EventSource *,RBX::NormalId,float)")]
#[doc(alias = "__ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEE14replicateEventEPNS0_11EventSourceES3_f")]
pub fn stub_0x569b7c(desc: &HandlesEvent2Desc, normal: HandlesNormalId, value: f32) {
    // IDA 0x569b7c `RemoteEventDescImpl<2, Handles, void(NormalId,
    // float)>::replicateEvent`: packs the 2-`Variant` vector with the
    // `NormalId`/`float` type singletons (`getSingleton<NormalId>(2)`,
    // `getSingleton<float>(4)`, 0x569c16-0x569c40), fires the replication
    // half (`*a2 + 12`, 0x569c54), then destroys the vector (0x569c5e).
    // Emitting the typed remote signal is the same delivery.
    desc.remote.emit(normal, value);
}

// 0x569ce8 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_19EventReplicatorImplILi2ENS2_7HandlesES4_EES3_fEENS8_5list3INS8_5valueIPSE_EENS7_3argILi1EEENSK_ILi2EEEEEEEEENS0_10connectionERKT_
// type: int(void)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::NormalId,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_19EventReplicatorImplILi2ENS2_7HandlesES4_EES3_fEENS8_5list3INS8_5valueIPSE_EENS7_3argILi1EEENSK_ILi2EEEEEEEEENS0_10connectionERKT_")]
pub use rbx_core::boost_core_i::stub_569ce8 as stub_0x569ce8;

// 0x569d5c — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE6insertEPNS5_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::insert(rbx::signals::signal<void ()(RBX::NormalId,float)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE6insertEPNS5_4slotE")]
pub use rbx_core::generated_core_i::stub_569d5c as stub_0x569d5c;

// 0x569f68 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slotEEaSEPS8_
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot>::operator=(rbx::signals::signal<void ()(RBX::NormalId,float)>::slot*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slotEEaSEPS8_")]
pub use rbx_core::boost_core_i::stub_569f68 as stub_0x569f68;

// 0x569f8c — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_19EventReplicatorImplILi2ENS2_7HandlesES4_EES3_fEENS8_5list3INS8_5valueIPSE_EENS7_3argILi1EEENSK_ILi2EEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_19EventReplicatorImplILi2ENS2_7HandlesES4_EES3_fEENS8_5list3INS8_5valueIPSE_EENS7_3argILi1EEENSK_ILi2EEEEEEEED1Ev")]
pub use rbx_core::boost_core_i::stub_569f8c as stub_0x569f8c;

// 0x569fb8 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_19EventReplicatorImplILi2ENS2_7HandlesES4_EES3_fEENS8_5list3INS8_5valueIPSE_EENS7_3argILi1EEENSK_ILi2EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_19EventReplicatorImplILi2ENS2_7HandlesES4_EES3_fEENS8_5list3INS8_5valueIPSE_EENS7_3argILi1EEENSK_ILi2EEEEEEEED0Ev")]
pub use rbx_core::boost_core_i::stub_569fb8 as stub_0x569fb8;

// 0x56a08c — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::slot::disconnect(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slot10disconnectEv")]
pub use rbx_core::generated_core_i::stub_56a08c as stub_0x56a08c;

// 0x56a19c — __ZNK3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::slot::connected(void)const")]
#[doc(alias = "__ZNK3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slot9connectedEv")]
pub use rbx_core::generated_core_i::stub_56a19c as stub_0x56a19c;

// 0x56a1a8 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_19EventReplicatorImplILi2ENS3_7HandlesES5_EES4_fEENS9_5list3INS9_5valueIPSF_EENS8_3argILi1EEENSL_ILi2EEEEEEELi2ES5_E4callES4_f
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(RBX::NormalId,float)>::call(RBX::NormalId,float)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_19EventReplicatorImplILi2ENS3_7HandlesES5_EES4_fEENS9_5list3INS9_5valueIPSF_EENS8_3argILi1EEENSL_ILi2EEEEEEELi2ES5_E4callES4_f")]
pub use rbx_core::boost_core_i::stub_56a1a8 as stub_0x56a1a8;

// 0x56a1d0 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_19EventReplicatorImplILi2ENS3_7HandlesES5_EES4_fEENS9_5list3INS9_5valueIPSF_EENS8_3argILi1EEENSL_ILi2EEEEEEELi2ES5_E4callES4_f
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(RBX::NormalId,float)>::call(RBX::NormalId,float)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_19EventReplicatorImplILi2ENS3_7HandlesES5_EES4_fEENS9_5list3INS9_5valueIPSF_EENS8_3argILi1EEENSL_ILi2EEEEEEELi2ES5_E4callES4_f")]
pub use rbx_core::boost_core_i::stub_56a1d0 as stub_0x56a1d0;

// 0x56a1f8 — __ZN5boost3_bi5list3INS0_5valueIPN3RBX19EventReplicatorImplILi2ENS3_7HandlesEFvNS3_8NormalIdEfEEEEENS_3argILi1EEENSB_ILi2EEEEclINS_4_mfi3mf2IvS8_S6_fEENS0_5list2IRS6_RfEEEEvNS0_4typeIvEERT_RT0_i
// type: int(void)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)> *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list2<RBX::NormalId&,float &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float> &,boost::_bi::list2<RBX::NormalId&,float &> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueIPN3RBX19EventReplicatorImplILi2ENS3_7HandlesEFvNS3_8NormalIdEfEEEEENS_3argILi1EEENSB_ILi2EEEEclINS_4_mfi3mf2IvS8_S6_fEENS0_5list2IRS6_RfEEEEvNS0_4typeIvEERT_RT0_i")]
pub use rbx_core::boost_core_i::stub_56a1f8 as stub_0x56a1f8;

// 0x56a224 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE6removeEPNS5_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::remove(rbx::signals::signal<void ()(RBX::NormalId,float)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE6removeEPNS5_4slotE")]
pub use rbx_core::generated_core_i::stub_56a224 as stub_0x56a224;

// 0x56a314 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slot22safe_static_init_mutexEv
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::slot::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slot22safe_static_init_mutexEv")]
pub use rbx_core::generated_core_i::stub_56a314 as stub_0x56a314;

// 0x56a318 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slot24safe_static_do_get_mutexEv")]
pub use rbx_core::generated_core_i::stub_56a318 as stub_0x56a318;

// 0x56a408 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slotD1Ev")]
pub use rbx_core::generated_core_i::stub_56a408 as stub_0x56a408;

// 0x56a434 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slotD0Ev")]
pub use rbx_core::generated_core_i::stub_56a434 as stub_0x56a434;

// 0x56a508 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_19EventReplicatorImplILi2ENS3_7HandlesES5_EES4_fEENS9_5list3INS9_5valueIPSF_EENS8_3argILi1EEENSL_ILi2EEEEEEELi2ES5_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(RBX::NormalId,float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_19EventReplicatorImplILi2ENS3_7HandlesES5_EES4_fEENS9_5list3INS9_5valueIPSF_EENS8_3argILi1EEENSL_ILi2EEEEEEELi2ES5_ED1Ev")]
pub use rbx_core::boost_core_i::stub_56a508 as stub_0x56a508;

// 0x56a534 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_19EventReplicatorImplILi2ENS3_7HandlesES5_EES4_fEENS9_5list3INS9_5valueIPSF_EENS8_3argILi1EEENSL_ILi2EEEEEEELi2ES5_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(RBX::NormalId,float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_19EventReplicatorImplILi2ENS3_7HandlesES5_EES4_fEENS9_5list3INS9_5valueIPSF_EENS8_3argILi1EEENSL_ILi2EEEEEEELi2ES5_ED0Ev")]
pub use rbx_core::boost_core_i::stub_56a534 as stub_0x56a534;

// 0x56a608 — __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEfEE21connectSignalListenerEv
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>::connectSignalListener(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEfEE21connectSignalListenerEv")]
pub use rbx_core::generated_core_shard_ig::stub_56a608 as stub_0x56a608;

// 0x56a60c — __ZN3RBX19EventReplicatorImplILi1ENS_7HandlesEFvNS_8NormalIdEEE21connectSignalListenerEv
#[doc(alias = "RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>::connectSignalListener(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi1ENS_7HandlesEFvNS_8NormalIdEEE21connectSignalListenerEv")]
pub use rbx_core::generated_core_shard_ig::stub_56a60c as stub_0x56a60c;

// 0x56a700 — __ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEE12getSignalPtrEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>>::getSignalPtr(RBX::Reflection::EventSource *)")]
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEE12getSignalPtrEPNS0_11EventSourceE")]
pub use rbx_core::generated_core_watchdog_1788317087::stub_56a700 as stub_0x56a700;

// 0x56a768 — __ZN3RBX19EventReplicatorImplILi1ENS_7HandlesEFvNS_8NormalIdEEE25signalProducedIncrementedES2_
#[doc(alias = "RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>::signalProducedIncremented(RBX::NormalId)")]
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi1ENS_7HandlesEFvNS_8NormalIdEEE25signalProducedIncrementedES2_")]
pub use rbx_core::generated_core_shard_ig::stub_56a768 as stub_0x56a768;

// 0x56a77c — __ZN3RBX10Reflection19RemoteEventDescImplILi1ENS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEE14replicateEventEPNS0_11EventSourceES3_
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<1,RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>>::replicateEvent(RBX::Reflection::EventSource *,RBX::NormalId)")]
#[doc(alias = "__ZN3RBX10Reflection19RemoteEventDescImplILi1ENS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEE14replicateEventEPNS0_11EventSourceES3_")]
pub fn stub_0x56a77c(desc: &HandlesEvent1Desc, normal: HandlesNormalId) {
    // IDA 0x56a77c `RemoteEventDescImpl<1, Handles, void(NormalId)>::
    // replicateEvent`: 1-arg twin of 0x569b7c — packs the 1-`Variant` vector
    // with the `NormalId` singleton (`getSingleton<NormalId>(2)`, 0x56a814),
    // fires the replication half (`*a2 + 12`, 0x56a834), destroys the vector
    // (0x56a83e).
    desc.remote.emit(normal);
}

// 0x56a8c8 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_7HandlesES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int(void)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::NormalId)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,boost::arg<1>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_7HandlesES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_")]
pub use rbx_core::boost_core_i::stub_56a8c8 as stub_0x56a8c8;

// 0x56a93c — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE6insertEPNS5_4slotE
// type: void __fastcall(int *, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::insert(rbx::signals::signal<void ()(RBX::NormalId)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE6insertEPNS5_4slotE")]
pub use rbx_core::generated_core_i::stub_56a93c as stub_0x56a93c;

// 0x56ab48 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slotEEaSEPS8_
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::NormalId)>::slot>::operator=(rbx::signals::signal<void ()(RBX::NormalId)>::slot*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slotEEaSEPS8_")]
pub use rbx_core::boost_core_i::stub_56ab48 as stub_0x56ab48;

// 0x56ab6c — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_7HandlesES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_7HandlesES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED1Ev")]
pub use rbx_core::boost_core_i::stub_56ab6c as stub_0x56ab6c;

// 0x56ab98 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_7HandlesES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_7HandlesES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED0Ev")]
pub use rbx_core::boost_core_i::stub_56ab98 as stub_0x56ab98;

// 0x56ac6c — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::slot::disconnect(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slot10disconnectEv")]
pub use rbx_core::generated_core_i::stub_56ac6c as stub_0x56ac6c;

// 0x56ad7c — __ZNK3rbx7signals6signalIFvN3RBX8NormalIdEEE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::slot::connected(void)const")]
#[doc(alias = "__ZNK3rbx7signals6signalIFvN3RBX8NormalIdEEE4slot9connectedEv")]
pub use rbx_core::generated_core_i::stub_56ad7c as stub_0x56ad7c;

// 0x56ad88 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_7HandlesES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,boost::arg<1>>>,1,void ()(RBX::NormalId)>::call(RBX::NormalId)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_7HandlesES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_")]
pub use rbx_core::boost_core_i::stub_56ad88 as stub_0x56ad88;

// 0x56ad9c — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_7HandlesES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,boost::arg<1>>>,1,void ()(RBX::NormalId)>::call(RBX::NormalId)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_7HandlesES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_")]
pub use rbx_core::boost_core_i::stub_56ad9c as stub_0x56ad9c;

// 0x56adb0 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENS4_7HandlesEFvNS4_8NormalIdEEEES7_EENS0_5list2INS0_5valueIPS9_EENS_3argILi1EEEEEEclIS7_EEvRT_
// type: int(void)
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,boost::arg<1>>>::operator()<RBX::NormalId>(RBX::NormalId &)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENS4_7HandlesEFvNS4_8NormalIdEEEES7_EENS0_5list2INS0_5valueIPS9_EENS_3argILi1EEEEEEclIS7_EEvRT_")]
pub use rbx_core::boost_core_i::stub_56adb0 as stub_0x56adb0;

// 0x56adc8 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE6removeEPNS5_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::remove(rbx::signals::signal<void ()(RBX::NormalId)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE6removeEPNS5_4slotE")]
pub use rbx_core::generated_core_i::stub_56adc8 as stub_0x56adc8;

// 0x56aeb8 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::slot::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slot22safe_static_init_mutexEv")]
pub use rbx_core::generated_core_i::stub_56aeb8 as stub_0x56aeb8;

// 0x56aebc — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slot24safe_static_do_get_mutexEv")]
pub use rbx_core::generated_core_i::stub_56aebc as stub_0x56aebc;

// 0x56afac — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slotD1Ev")]
pub use rbx_core::generated_core_i::stub_56afac as stub_0x56afac;

// 0x56afd8 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slotD0Ev")]
pub use rbx_core::generated_core_i::stub_56afd8 as stub_0x56afd8;

// 0x56b0ac — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_7HandlesES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,boost::arg<1>>>,1,void ()(RBX::NormalId)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_7HandlesES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED1Ev")]
pub use rbx_core::boost_core_i::stub_56b0ac as stub_0x56b0ac;

// 0x56b0d8 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_7HandlesES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,boost::arg<1>>>,1,void ()(RBX::NormalId)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_7HandlesES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED0Ev")]
pub use rbx_core::boost_core_i::stub_56b0d8 as stub_0x56b0d8;

// 0x56b1ac — __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE21connectSignalListenerEv
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>::connectSignalListener(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE21connectSignalListenerEv")]
pub use rbx_core::generated_core_shard_ig::stub_56b1ac as stub_0x56b1ac;

// 0x56b3ac — __ZN3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EEC2INS_7HandlesEEEPKcS7_MT_iNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Handles>(char const*,char const*,int RBX::Handles::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EEC2INS_7HandlesEEEPKcS7_MT_iNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x56b3ac(name: &str, category: &str) -> HandlesIntProp {
    // IDA 0x56b3ac `BoundProp<int, Mutability1>::BoundProp<Handles>`: runs
    // the `Described<Handles>` + `TypedPropertyDescriptor<int>` bases
    // (0x56b3d2-0x56b434), installs the `BoundProp` vtable (`off_12324C8`,
    // 0x56b452), allocates the `BoundPropGetSet<Handles>` at `+40`
    // (`off_1263A28`, 0x56b460-0x56b496, old payload deleted), then clears
    // the read/write-only flag bits at `+28` through the `+12`/`+8` virtuals
    // (0x56b4a6-0x56b4cc). The member offset collapses into
    // `Handles::int_value`; flags are never read/write-only for a get/set
    // pair.
    HandlesIntProp { name: name.to_string(), category: category.to_string() }
}

// 0x56b53c — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_7HandlesEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Handles>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_7HandlesEE10isReadOnlyEv")]
pub use rbx_reflection::generated::stub_0x56b53c as stub_0x56b53c;

// 0x56b540 — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_7HandlesEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Handles>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_7HandlesEE11isWriteOnlyEv")]
pub use rbx_reflection::generated::stub_0x56b540 as stub_0x56b540;

// 0x56b544 — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_7HandlesEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Handles>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_7HandlesEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x56b544(_desc: &HandlesIntProp, handles: &Handles) -> i32 {
    // IDA 0x56b544 `BoundPropGetSet<Handles>::getValue`: `*(*(a1 + 8) + a2 -
    // 36)` (0x56b54c) — the bound `int` member on the adjusted instance.
    handles.int_value
}

// 0x56b550 — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_7HandlesEE8setValueEPNS0_13DescribedBaseERKi
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Handles>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_7HandlesEE8setValueEPNS0_13DescribedBaseERKi")]
pub fn stub_0x56b550(_desc: &HandlesIntProp, handles: &mut Handles, value: i32) {
    // IDA 0x56b550 `BoundPropGetSet<Handles>::setValue`: no-op when
    // `*(v4 + result) == *a3` (0x56b568), else stores the word (0x56b56c),
    // runs the change virtual when the flag words are set (0x56b576-0x56b58c),
    // and `raisePropertyChanged`s (0x56b59a). The store is the observable
    // state change; the notification collapses here, as in
    // `FaceInstance::setFace` (IDA `0x4a94fc`).
    if handles.int_value != value {
        handles.int_value = value;
    }
}

// 0x56b5a0 — __ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEED0Ev
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>>::~RemoteEventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEED0Ev")]
pub use rbx_reflection::generated::stub_0x56b5a0 as stub_0x56b5a0;

// 0x56b654 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEMS2_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>,rbx::remote_signal<void ()(RBX::NormalId,float)> RBX::Handles::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi2ENS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEMS2_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
// was: RBX::Reflection::EventDescImpl<2,RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>,rbx::remote_signal<void ()(RBX::NormalId,float)> RBX::Handles::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub use rbx_core::boost_core_i::stub_56b654 as stub_0x56b654;

// 0x56b7b8 — __ZNK3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEE12isScriptableEv
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>>::isScriptable(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEE12isScriptableEv")]
pub use rbx_reflection::generated::stub_0x56b7b8 as stub_0x56b7b8;

// 0x56b7c0 — __ZNK3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEE11isBroadcastEv
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>>::isBroadcast(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEE11isBroadcastEv")]
pub fn stub_0x56b7c0(desc: &HandlesEvent2Desc) -> bool {
    // IDA 0x56b7c0 `RemoteEventDesc<Handles, void(NormalId,
    // float)>::isBroadcast`: `*(a1 + 44) & 1` (0x56b7c6).
    desc.broadcast
}

// 0x56b7c8 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEMS2_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>,rbx::remote_signal<void ()(RBX::NormalId,float)> RBX::Handles::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi2ENS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEMS2_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE")]
pub fn stub_0x56b7c8(desc: &HandlesEvent2Desc, normal: HandlesNormalId, value: f32) {
    // IDA 0x56b7c8 `EventDescImpl<2, Handles, void(NormalId,
    // float)>::fireEvent`: asserts `args.size() == 2` (Event.h:349,
    // 0x56b7e2-0x56b828), `any_cast`s the `NormalId`/`float` args out of the
    // `Variant` vector (0x56b838-0x56b84e), then
    // `signal_with_args<2>::operator()` fans out to each connected wrapper's
    // `execute2`. `generated_05::Variant` has no `Float` arm, so the float
    // crosses as `Variant::Int` bits; the typed signature guarantees the
    // arity and the casts.
    desc.signal.emit(normal, value);
}

// 0x56b864 — __ZNK3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
pub fn stub_0x56b864(desc: &HandlesEvent2Desc, normal: HandlesNormalId, value: f32) {
    // IDA 0x56b864 `RemoteEventDesc<Handles, void(NormalId,
    // float)>::sendEvent`: tail-calls the remote half's virtual at `*a2 + 12`
    // with the `Variant` vector. Emitting the remote signal is that delivery.
    desc.remote.emit(normal, value);
}

// 0x56b874 — __ZNK3RBX10Reflection13EventDescBaseINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEMS2_S7_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>,rbx::remote_signal<void ()(RBX::NormalId,float)> RBX::Handles::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEMS2_S7_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_0x56b874(desc: &HandlesEvent2Desc) {
    // IDA 0x56b874 `EventDescBase<Handles, void(NormalId,
    // float)>::disconnectAll`: adjusts the source to the member signal
    // (`a2 ? a2 - 36 : 0`, 0x56b874-0x56b87a) and `signal::disconnectAll`s it
    // (`*(a1 + 40) + v10`). Collapses to clearing the payload-side list.
    desc.signal.disconnect_all();
}

// 0x56b888 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS1_8NormalIdERKfNS_10shared_ptrIS3_EENS_3argILi1EEENSB_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISG_T0_T1_T2_EENSE_9list_av_3IT3_T4_T5_E4typeEEEMSJ_FSG_SK_SL_ESO_SP_SQ_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list_av_3<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::NormalId const&,float const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")]
#[doc(alias = "__ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS1_8NormalIdERKfNS_10shared_ptrIS3_EENS_3argILi1EEENSB_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISG_T0_T1_T2_EENSE_9list_av_3IT3_T4_T5_E4typeEEEMSJ_FSG_SK_SL_ESO_SP_SQ_")]
// was: boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list_av_3<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::NormalId const&,float const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)
pub use rbx_core::boost_core_i::stub_56b888 as stub_0x56b888;

// 0x56b9a4 — __ZN3RBX10Reflection18GenericSlotWrapper8execute2INS_8NormalIdEfEEvRKT_RKT0_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute2<RBX::NormalId,float>(RBX::NormalId const&,float const&)")]
#[doc(alias = "__ZN3RBX10Reflection18GenericSlotWrapper8execute2INS_8NormalIdEfEEvRKT_RKT0_")]
pub fn stub_0x56b9a4(wrapper: &HandlesSlotWrapper2, normal: HandlesNormalId, value: f32) {
    // IDA 0x56b9a4 `GenericSlotWrapper::execute2<NormalId, float>`: packs the
    // 2-`Variant` vector with the `NormalId`/`float` singletons
    // (`getSingleton<NormalId>(2)`, `getSingleton<float>(4)`, 0x56ba3c-0x56ba66),
    // dispatches the wrapped slot (`*a1 + 8`, 0x56ba76), destroys the vector
    // (0x56ba80). The Lua frame underneath is the handler until the script
    // bridge exists.
    wrapper.execute2(normal, value);
}

// 0x56bb0c — __ZN5boost9function2IvN3RBX8NormalIdEfE5clearEv
// type: int(void)
#[doc(alias = "boost::function2<void,RBX::NormalId,float>::clear(void)")]
#[doc(alias = "__ZN5boost9function2IvN3RBX8NormalIdEfE5clearEv")]
pub use rbx_core::boost_core_i::stub_56bb0c as stub_0x56bb0c;

// 0x56bb38 — __ZN5boost8functionIFvN3RBX8NormalIdEfEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS2_RKfEENS6_5list3INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvN3RBX8NormalIdEfEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS2_RKfEENS6_5list3INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost8functionIFvN3RBX8NormalIdEfEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS2_RKfEENS6_5list3INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")]
pub use rbx_core::boost_core_i::stub_56bb38 as stub_0x56bb38;

// 0x56bc1c — __ZN5boost9function2IvN3RBX8NormalIdEfEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS2_RKfEENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEENSL_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function2IvN3RBX8NormalIdEfEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS2_RKfEENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEENSL_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost9function2IvN3RBX8NormalIdEfEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS2_RKfEENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEENSL_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
pub use rbx_core::boost_core_i::stub_56bc1c as stub_0x56bc1c;

// 0x56bd04 — __ZN5boost9function2IvN3RBX8NormalIdEfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS2_RKfEENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEENSL_ILi2EEEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function2<void,RBX::NormalId,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")]
#[doc(alias = "__ZN5boost9function2IvN3RBX8NormalIdEfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS2_RKfEENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEENSL_ILi2EEEEEEEEEvT_")]
// was: void boost::function2<void,RBX::NormalId,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)
pub use rbx_core::boost_core_i::stub_56bd04 as stub_0x56bd04;

// 0x56bdfc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_8NormalIdERKfEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSL_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_8NormalIdERKfEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSL_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeE")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub use rbx_core::boost_core_i::stub_56bdfc as stub_0x56bdfc;

// 0x56be18 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_8NormalIdERKfEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSL_ILi2EEEEEEEvSA_fE6invokeERNS1_15function_bufferESA_f
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,RBX::NormalId,float>::invoke(boost::detail::function::function_buffer &,RBX::NormalId,float)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_8NormalIdERKfEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSL_ILi2EEEEEEEvSA_fE6invokeERNS1_15function_bufferESA_f")]
// was: boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,RBX::NormalId,float>::invoke(boost::detail::function::function_buffer &,RBX::NormalId,float)
pub use rbx_core::boost_core_i::stub_56be18 as stub_0x56be18;

// 0x56be30 — __ZNK5boost6detail8function13basic_vtable2IvN3RBX8NormalIdEfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS4_RKfEENS7_5list3INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,RBX::NormalId,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvN3RBX8NormalIdEfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS4_RKfEENS7_5list3INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEEbT_RNS1_15function_bufferE")]
// was: bool boost::detail::function::basic_vtable2<void,RBX::NormalId,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const
pub use rbx_core::boost_core_i::stub_56be30 as stub_0x56be30;

// 0x56bf18 — __ZNK5boost6detail8function13basic_vtable2IvN3RBX8NormalIdEfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS4_RKfEENS7_5list3INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,RBX::NormalId,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvN3RBX8NormalIdEfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS4_RKfEENS7_5list3INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// was: bool boost::detail::function::basic_vtable2<void,RBX::NormalId,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub use rbx_core::boost_core_i::stub_56bf18 as stub_0x56bf18;

// 0x56bffc — __ZNK5boost6detail8function13basic_vtable2IvN3RBX8NormalIdEfE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS4_RKfEENS7_5list3INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable2<void,RBX::NormalId,float>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvN3RBX8NormalIdEfE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS4_RKfEENS7_5list3INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
// was: void boost::detail::function::basic_vtable2<void,RBX::NormalId,float>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub use rbx_core::boost_core_i::stub_56bffc as stub_0x56bffc;

// 0x56c0d0 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS4_8NormalIdERKfEENS0_5list3INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEENSI_ILi2EEEEEEclIS7_fEEvRT_RT0_
// type: int(void)
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<RBX::NormalId,float>(RBX::NormalId &,float &)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS4_8NormalIdERKfEENS0_5list3INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEENSI_ILi2EEEEEEclIS7_fEEvRT_RT0_")]
// was: void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<RBX::NormalId,float>(RBX::NormalId &,float &)
pub use rbx_core::boost_core_i::stub_56c0d0 as stub_0x56c0d0;

// 0x56c0ec — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_8NormalIdERKfEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSL_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_8NormalIdERKfEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSL_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub use rbx_core::boost_core_i::stub_56c0ec as stub_0x56c0ec;

// 0x56c244 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::NormalId,float)>::connect<boost::function<void ()(RBX::NormalId,float)>>(boost::function<void ()(RBX::NormalId,float)> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_")]
pub use rbx_core::boost_core_i::stub_56c244 as stub_0x56c244;

// 0x56c338 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_EC2IPS6_EERKSA_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,boost::function<void ()(RBX::NormalId,float)>,2,void ()(RBX::NormalId,float)>::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>*>(boost::function<void ()(RBX::NormalId,float)> const&,rbx::signals::signal<void ()(RBX::NormalId,float)>*)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_EC2IPS6_EERKSA_T_")]
pub use rbx_core::boost_core_i::stub_56c338 as stub_0x56c338;

// 0x56c434 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13callable_slotIN5boost8functionIS4_EEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::callable_slot<boost::function<void ()(RBX::NormalId,float)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13callable_slotIN5boost8functionIS4_EEED1Ev")]
pub use rbx_core::boost_core_i::stub_56c434 as stub_0x56c434;

// 0x56c544 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13callable_slotIN5boost8functionIS4_EEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::callable_slot<boost::function<void ()(RBX::NormalId,float)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13callable_slotIN5boost8functionIS4_EEED0Ev")]
pub use rbx_core::boost_core_i::stub_56c544 as stub_0x56c544;

// 0x56c674 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_E4callES4_f
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,boost::function<void ()(RBX::NormalId,float)>,2,void ()(RBX::NormalId,float)>::call(RBX::NormalId,float)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_E4callES4_f")]
pub use rbx_core::boost_core_i::stub_56c674 as stub_0x56c674;

// 0x56c67c — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_E4callES4_f
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,boost::function<void ()(RBX::NormalId,float)>,2,void ()(RBX::NormalId,float)>::call(RBX::NormalId,float)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_E4callES4_f")]
pub use rbx_core::boost_core_i::stub_56c67c as stub_0x56c67c;

// 0x56c684 — __ZNK5boost9function2IvN3RBX8NormalIdEfEclES2_f
// type: int(void)
#[doc(alias = "boost::function2<void,RBX::NormalId,float>::operator()(RBX::NormalId,float)const")]
#[doc(alias = "__ZNK5boost9function2IvN3RBX8NormalIdEfEclES2_f")]
pub use rbx_core::boost_core_i::stub_56c684 as stub_0x56c684;

// 0x56c750 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,boost::function<void ()(RBX::NormalId,float)>,2,void ()(RBX::NormalId,float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_ED1Ev")]
pub use rbx_core::boost_core_i::stub_56c750 as stub_0x56c750;

// 0x56c860 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,boost::function<void ()(RBX::NormalId,float)>,2,void ()(RBX::NormalId,float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_ED0Ev")]
pub use rbx_core::boost_core_i::stub_56c860 as stub_0x56c860;

// 0x56c990 — __ZN5boost9function2IvN3RBX8NormalIdEfE13assign_to_ownERKS3_
// type: int(void)
#[doc(alias = "boost::function2<void,RBX::NormalId,float>::assign_to_own(boost::function2<void,RBX::NormalId,float> const&)")]
#[doc(alias = "__ZN5boost9function2IvN3RBX8NormalIdEfE13assign_to_ownERKS3_")]
pub use rbx_core::boost_core_i::stub_56c990 as stub_0x56c990;

// 0x56c9c0 — __ZN3RBX10Reflection9EventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEMS2_S7_EC2ES8_PKcSB_SB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>,rbx::remote_signal<void ()(RBX::NormalId,float)> RBX::Handles::*>::EventDesc(rbx::remote_signal<void ()(RBX::NormalId,float)> RBX::Handles::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEMS2_S7_EC2ES8_PKcSB_SB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0x56c9c0(
    this: *mut HandlesEvent2Desc,
    name: &str,
    permissions: u32,
    attributes: u32,
) {
    // IDA 0x56c9c0 `EventDesc<Handles, void(NormalId, float)>::EventDesc` C2:
    // runs the `Described<Handles>` + `EventDescriptor` bases (0x56c9fc +
    // 0x56ca1a), stores the member signal pointer at `+40` (0x56ca3e),
    // installs the `EventDesc` vtable (`off_1263B58`, 0x56ca42), declares the
    // two `Name`s and appends the two signature items —
    // `NormalId` (`getSingleton<NormalId>(2)`, 0x56ca6e) and `float`
    // (`getSingleton<float>(7)`, 0x56caaa) — via `_M_create_node` + `hook`
    // (0x56ca8c-0x56cace). The member pointer collapses into the payload's
    // member signals; the item types are comment-only.
    // SAFETY: `this` must point to valid uninitialized `HandlesEvent2Desc` storage.
    let _ = permissions;
    let _ = attributes;
    unsafe {
        core::ptr::write(
            this,
            HandlesEvent2Desc { name: name.to_string(), ..Default::default() },
        );
    }
}

// 0x56cbb0 — __ZN3RBX10Reflection9EventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEMS2_S7_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>,rbx::remote_signal<void ()(RBX::NormalId,float)> RBX::Handles::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEMS2_S7_ED1Ev")]
pub use rbx_reflection::generated::stub_0x56cbb0 as stub_0x56cbb0;

#[cfg(test)]
mod handles_remote_tests {
    use super::*;
    use std::sync::atomic::{AtomicI32, Ordering};

    #[test]
    fn int_prop_round_trip() {
        let desc = stub_0x56b3ac("Style", "Appearance");
        assert_eq!(desc.name, "Style");
        let mut handles = Handles::default();
        assert_eq!(stub_0x56b544(&desc, &handles), 0);
        stub_0x56b550(&desc, &mut handles, 2);
        assert_eq!(stub_0x56b544(&desc, &handles), 2);
        stub_0x56b550(&desc, &mut handles, 2);
        assert_eq!(handles.int_value, 2);
    }

    #[test]
    fn broadcast_fire_send_disconnect() {
        let mut storage = HandlesEvent2Desc::default();
        stub_0x56c9c0(&mut storage as *mut HandlesEvent2Desc, "Changed", 0, 0);
        assert_eq!(storage.name, "Changed");
        assert!(!stub_0x56b7c0(&storage));
        let seen = Arc::new(AtomicI32::new(0));
        let probe = Arc::clone(&seen);
        storage.signal.connect(Arc::new(move |normal: u32, value: f32| {
            probe.store(normal as i32 + value as i32, Ordering::Relaxed);
        }));
        assert_eq!(storage.signal.len(), 1);
        stub_0x56b7c8(&storage, 3, 2.0);
        assert_eq!(seen.load(Ordering::Relaxed), 5);
        let remote = Arc::new(AtomicI32::new(0));
        let rp = Arc::clone(&remote);
        storage.remote.connect(Arc::new(move |normal: u32, value: f32| {
            rp.store(100 + normal as i32 + value as i32, Ordering::Relaxed);
        }));
        stub_0x56b864(&storage, 1, 4.0);
        assert_eq!(remote.load(Ordering::Relaxed), 105);
        stub_0x569b7c(&storage, 2, 6.0);
        assert_eq!(remote.load(Ordering::Relaxed), 108);
        stub_0x56b874(&storage);
        assert_eq!(storage.signal.len(), 0);
        stub_0x56b7c8(&storage, 9, 9.0);
        assert_eq!(seen.load(Ordering::Relaxed), 5);
    }

    #[test]
    fn replicate1_and_execute2() {
        let desc = HandlesEvent1Desc { name: "Touched".to_string(), ..Default::default() };
        assert!(!desc.broadcast);
        let remote = Arc::new(AtomicI32::new(0));
        let rp = Arc::clone(&remote);
        desc.remote.connect(Arc::new(move |normal: u32| {
            rp.store(normal as i32, Ordering::Relaxed);
        }));
        stub_0x56a77c(&desc, 4);
        assert_eq!(remote.load(Ordering::Relaxed), 4);
        let seen = Arc::new(AtomicI32::new(0));
        let probe = Arc::clone(&seen);
        let wrapper = HandlesSlotWrapper2 {
            handler: Arc::new(move |normal: u32, value: f32| {
                probe.store(normal as i32 + value as i32, Ordering::Relaxed);
            }),
        };
        stub_0x56b9a4(&wrapper, 5, 7.0);
        assert_eq!(seen.load(Ordering::Relaxed), 12);
    }
}
