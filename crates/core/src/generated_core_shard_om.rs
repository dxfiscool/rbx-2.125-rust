//! core shard om — 100 core stubs EA-sorted, 0x716570..0x71af78 (RBX not Reflection|Instance|DataModel|Ogre|G3D|RakNet|Sound|Audio|FMOD|Script|Lua, EA-sorted asc, next 100 after 0x716570 global dedup).
//! Source: ida/export.json filtered where demangled contains RBX and not Reflection|Instance|DataModel|Ogre|G3D|RakNet|Sound|Audio|FMOD|Script|Lua, EA-sorted asc, next 100 uncovered not in /tmp/global_eas.txt.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Verb *>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Verb *>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Verb *>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Verb *> const&)")]
// 0x716570 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PNS0_4VerbEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_0x716570() -> ! {
    todo!("0x716570 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PNS0_4VerbEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "RBX::Assembly::Assembly(void)")]
// 0x7166a0 — __ZN3RBX8AssemblyC1Ev
pub fn stub_0x7166a0() -> ! {
    todo!("0x7166a0 __ZN3RBX8AssemblyC1Ev")
}

#[doc(alias = "RBX::Assembly::Assembly(void)")]
// 0x7166a4 — __ZN3RBX8AssemblyC2Ev
pub fn stub_0x7166a4() -> ! {
    todo!("0x7166a4 __ZN3RBX8AssemblyC2Ev")
}

#[doc(alias = "RBX::Assembly::computeAssemblyMaxRadius(void)")]
// 0x716824 — __ZN3RBX8Assembly24computeAssemblyMaxRadiusEv
pub fn stub_0x716824() -> ! {
    todo!("0x716824 __ZN3RBX8Assembly24computeAssemblyMaxRadiusEv")
}

#[doc(alias = "RBX::Assembly::~Assembly()")]
// 0x7168bc — __ZN3RBX8AssemblyD0Ev
pub fn stub_0x7168bc() -> ! {
    todo!("0x7168bc __ZN3RBX8AssemblyD0Ev")
}

#[doc(alias = "RBX::Assembly::~Assembly()")]
// 0x71695c — __ZN3RBX8AssemblyD1Ev
pub fn stub_0x71695c() -> ! {
    todo!("0x71695c __ZN3RBX8AssemblyD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Assembly::~Assembly()")]
// 0x716960 — __ZThn8_N3RBX8AssemblyD0Ev
pub fn stub_0x716960() -> ! {
    todo!("0x716960 __ZThn8_N3RBX8AssemblyD0Ev")
}

#[doc(alias = "RBX::Assembly::~Assembly()")]
// 0x716968 — __ZN3RBX8AssemblyD2Ev
pub fn stub_0x716968() -> ! {
    todo!("0x716968 __ZN3RBX8AssemblyD2Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Assembly::~Assembly()")]
// 0x716bf8 — __ZThn8_N3RBX8AssemblyD1Ev
pub fn stub_0x716bf8() -> ! {
    todo!("0x716bf8 __ZThn8_N3RBX8AssemblyD1Ev")
}

#[doc(alias = "RBX::Assembly::reset(RBX::Sim::AssemblyState)")]
// 0x716c00 — __ZN3RBX8Assembly5resetENS_3Sim13AssemblyStateE
pub fn stub_0x716c00() -> ! {
    todo!("0x716c00 __ZN3RBX8Assembly5resetENS_3Sim13AssemblyStateE")
}

#[doc(alias = "RBX::Assembly::sampleAndNotMoving(void)")]
// 0x716df8 — __ZN3RBX8Assembly18sampleAndNotMovingEv
pub fn stub_0x716df8() -> ! {
    todo!("0x716df8 __ZN3RBX8Assembly18sampleAndNotMovingEv")
}

#[doc(alias = "RBX::Assembly::preventNeighborSleep(void)")]
// 0x716e08 — __ZN3RBX8Assembly20preventNeighborSleepEv
pub fn stub_0x716e08() -> ! {
    todo!("0x716e08 __ZN3RBX8Assembly20preventNeighborSleepEv")
}

#[doc(alias = "RBX::Assembly::wakeUp(void)")]
// 0x716e14 — __ZN3RBX8Assembly6wakeUpEv
pub fn stub_0x716e14() -> ! {
    todo!("0x716e14 __ZN3RBX8Assembly6wakeUpEv")
}

#[doc(alias = "RBX::Assembly::getAssemblyPrimitive(void)")]
// 0x716e34 — __ZN3RBX8Assembly20getAssemblyPrimitiveEv
pub fn stub_0x716e34() -> ! {
    todo!("0x716e34 __ZN3RBX8Assembly20getAssemblyPrimitiveEv")
}

#[doc(alias = "RBX::Assembly::getAssemblyState(void)const")]
// 0x716e98 — __ZNK3RBX8Assembly16getAssemblyStateEv
pub fn stub_0x716e98() -> ! {
    todo!("0x716e98 __ZNK3RBX8Assembly16getAssemblyStateEv")
}

#[doc(alias = "RBX::Assembly::getConstAssemblyPrimitive(void)const")]
// 0x716f00 — __ZNK3RBX8Assembly25getConstAssemblyPrimitiveEv
pub fn stub_0x716f00() -> ! {
    todo!("0x716f00 __ZNK3RBX8Assembly25getConstAssemblyPrimitiveEv")
}

#[doc(alias = "RBX::Assembly::getPrimitiveAssemblyFast(RBX::Primitive *)")]
// 0x716f64 — __ZN3RBX8Assembly24getPrimitiveAssemblyFastEPNS_9PrimitiveE
pub fn stub_0x716f64() -> ! {
    todo!("0x716f64 __ZN3RBX8Assembly24getPrimitiveAssemblyFastEPNS_9PrimitiveE")
}

#[doc(alias = "RBX::Assembly::getPrimitiveAssembly(RBX::Primitive *)")]
// 0x716fd0 — __ZN3RBX8Assembly20getPrimitiveAssemblyEPNS_9PrimitiveE
pub fn stub_0x716fd0() -> ! {
    todo!("0x716fd0 __ZN3RBX8Assembly20getPrimitiveAssemblyEPNS_9PrimitiveE")
}

#[doc(alias = "RBX::Assembly::getConstPrimitiveAssembly(RBX::Primitive const*)")]
// 0x716ff8 — __ZN3RBX8Assembly25getConstPrimitiveAssemblyEPKNS_9PrimitiveE
pub fn stub_0x716ff8() -> ! {
    todo!("0x716ff8 __ZN3RBX8Assembly25getConstPrimitiveAssemblyEPKNS_9PrimitiveE")
}

#[doc(alias = "RBX::Assembly::onLowersChanged(void)")]
// 0x717020 — __ZN3RBX8Assembly15onLowersChangedEv
pub fn stub_0x717020() -> ! {
    todo!("0x717020 __ZN3RBX8Assembly15onLowersChangedEv")
}

#[doc(alias = "non-virtual thunk toRBX::Assembly::onLowersChanged(void)")]
// 0x717028 — __ZThn8_N3RBX8Assembly15onLowersChangedEv
pub fn stub_0x717028() -> ! {
    todo!("0x717028 __ZThn8_N3RBX8Assembly15onLowersChangedEv")
}

#[doc(alias = "RBX::Assembly::getAssemblyClump(void)")]
// 0x717030 — __ZN3RBX8Assembly16getAssemblyClumpEv
pub fn stub_0x717030() -> ! {
    todo!("0x717030 __ZN3RBX8Assembly16getAssemblyClumpEv")
}

#[doc(alias = "RBX::Assembly::getConstAssemblyClump(void)const")]
// 0x717088 — __ZNK3RBX8Assembly21getConstAssemblyClumpEv
pub fn stub_0x717088() -> ! {
    todo!("0x717088 __ZNK3RBX8Assembly21getConstAssemblyClumpEv")
}

#[doc(alias = "RBX::lessMotor(RBX::Joint const*,RBX::Joint const*)")]
// 0x7170e0 — __ZN3RBX9lessMotorEPKNS_5JointES2_
pub fn stub_0x7170e0() -> ! {
    todo!("0x7170e0 __ZN3RBX9lessMotorEPKNS_5JointES2_")
}

#[doc(alias = "RBX::Assembly::notifyMovedFromExternal(void)")]
// 0x71760c — __ZN3RBX8Assembly23notifyMovedFromExternalEv
pub fn stub_0x71760c() -> ! {
    todo!("0x71760c __ZN3RBX8Assembly23notifyMovedFromExternalEv")
}

#[doc(alias = "RBX::Assembly::isAssemblyRootPrimitive(RBX::Primitive const*)")]
// 0x71767c — __ZN3RBX8Assembly23isAssemblyRootPrimitiveEPKNS_9PrimitiveE
pub fn stub_0x71767c() -> ! {
    todo!("0x71767c __ZN3RBX8Assembly23isAssemblyRootPrimitiveEPKNS_9PrimitiveE")
}

#[doc(alias = "RBX::Assembly::otherAssembly(RBX::Edge *)")]
// 0x717710 — __ZN3RBX8Assembly13otherAssemblyEPNS_4EdgeE
pub fn stub_0x717710() -> ! {
    todo!("0x717710 __ZN3RBX8Assembly13otherAssemblyEPNS_4EdgeE")
}

#[doc(alias = "RBX::Assembly::getCanThrottle(void)const")]
// 0x717790 — __ZNK3RBX8Assembly14getCanThrottleEv
pub fn stub_0x717790() -> ! {
    todo!("0x717790 __ZNK3RBX8Assembly14getCanThrottleEv")
}

#[doc(alias = "RBX::Assembly::computeCanThrottle(RBX::Edge *)")]
// 0x7177a0 — __ZN3RBX8Assembly18computeCanThrottleEPNS_4EdgeE
pub fn stub_0x7177a0() -> ! {
    todo!("0x7177a0 __ZN3RBX8Assembly18computeCanThrottleEPNS_4EdgeE")
}

#[doc(alias = "RBX::Assembly::get2dPosition(void)const")]
// 0x7177d8 — __ZNK3RBX8Assembly13get2dPositionEv
pub fn stub_0x7177d8() -> ! {
    todo!("0x7177d8 __ZNK3RBX8Assembly13get2dPositionEv")
}

#[doc(alias = "RBX::Assembly::gatherPrimitiveExternalEdges(RBX::Primitive *)")]
// 0x7177f4 — __ZN3RBX8Assembly28gatherPrimitiveExternalEdgesEPNS_9PrimitiveE
pub fn stub_0x7177f4() -> ! {
    todo!("0x7177f4 __ZN3RBX8Assembly28gatherPrimitiveExternalEdgesEPNS_9PrimitiveE")
}

#[doc(alias = "RBX::Assembly::getAssemblyEdges(void)")]
// 0x71789c — __ZN3RBX8Assembly16getAssemblyEdgesEv
pub fn stub_0x71789c() -> ! {
    todo!("0x71789c __ZN3RBX8Assembly16getAssemblyEdgesEv")
}

#[doc(alias = "RBX::Assembly::computeIsGroundingPrimitive(RBX::Primitive const*)")]
// 0x717978 — __ZN3RBX8Assembly27computeIsGroundingPrimitiveEPKNS_9PrimitiveE
pub fn stub_0x717978() -> ! {
    todo!("0x717978 __ZN3RBX8Assembly27computeIsGroundingPrimitiveEPKNS_9PrimitiveE")
}

#[doc(alias = "RBX::Assembly::computeIsGrounded(void)const")]
// 0x71798c — __ZNK3RBX8Assembly17computeIsGroundedEv
pub fn stub_0x71798c() -> ! {
    todo!("0x71798c __ZNK3RBX8Assembly17computeIsGroundedEv")
}

#[doc(alias = "RBX::notifyAssemblyPrimitiveMoved(RBX::Primitive *,bool)")]
// 0x717a6c — __ZN3RBX28notifyAssemblyPrimitiveMovedEPNS_9PrimitiveEb
pub fn stub_0x717a6c() -> ! {
    todo!("0x717a6c __ZN3RBX28notifyAssemblyPrimitiveMovedEPNS_9PrimitiveEb")
}

#[doc(alias = "RBX::Assembly::notifyMovedFromInternalPhysics(void)")]
// 0x717b7c — __ZN3RBX8Assembly30notifyMovedFromInternalPhysicsEv
pub fn stub_0x717b7c() -> ! {
    todo!("0x717b7c __ZN3RBX8Assembly30notifyMovedFromInternalPhysicsEv")
}

#[doc(alias = "RBX::PV::operator==(RBX::PV const&)const")]
// 0x717bec — __ZNK3RBX2PVeqERKS0_
pub fn stub_0x717bec() -> ! {
    todo!("0x717bec __ZNK3RBX2PVeqERKS0_")
}

#[doc(alias = "void std::__introsort_loop<RBX::Joint const**,int,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint const**,RBX::Joint const**,int,bool (*)(RBX::Joint const*,RBX::Joint const*))")]
// 0x718070 — __ZSt16__introsort_loopIPPKN3RBX5JointEiPFbS3_S3_EEvT_S7_T0_T1_
pub fn stub_0x718070() -> ! {
    todo!("0x718070 __ZSt16__introsort_loopIPPKN3RBX5JointEiPFbS3_S3_EEvT_S7_T0_T1_")
}

#[doc(alias = "void std::__final_insertion_sort<RBX::Joint const**,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint const**,RBX::Joint const**,bool (*)(RBX::Joint const*,RBX::Joint const*))")]
// 0x71812c — __ZSt22__final_insertion_sortIPPKN3RBX5JointEPFbS3_S3_EEvT_S7_T0_
pub fn stub_0x71812c() -> ! {
    todo!("0x71812c __ZSt22__final_insertion_sortIPPKN3RBX5JointEPFbS3_S3_EEvT_S7_T0_")
}

#[doc(alias = "void std::__insertion_sort<RBX::Joint const**,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint const**,RBX::Joint const**,bool (*)(RBX::Joint const*,RBX::Joint const*))")]
// 0x718198 — __ZSt16__insertion_sortIPPKN3RBX5JointEPFbS3_S3_EEvT_S7_T0_
pub fn stub_0x718198() -> ! {
    todo!("0x718198 __ZSt16__insertion_sortIPPKN3RBX5JointEPFbS3_S3_EEvT_S7_T0_")
}

#[doc(alias = "RBX::Joint const** std::__unguarded_partition<RBX::Joint const**,RBX::Joint const*,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint const**,RBX::Joint const**,RBX::Joint const*,bool (*)(RBX::Joint const*,RBX::Joint const*))")]
// 0x718214 — __ZSt21__unguarded_partitionIPPKN3RBX5JointES3_PFbS3_S3_EET_S7_S7_T0_T1_
pub fn stub_0x718214() -> ! {
    todo!("0x718214 __ZSt21__unguarded_partitionIPPKN3RBX5JointES3_PFbS3_S3_EET_S7_S7_T0_T1_")
}

#[doc(alias = "void std::__heap_select<RBX::Joint const**,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint const**,RBX::Joint const**,RBX::Joint const**,bool (*)(RBX::Joint const*,RBX::Joint const*))")]
// 0x71825c — __ZSt13__heap_selectIPPKN3RBX5JointEPFbS3_S3_EEvT_S7_S7_T0_
pub fn stub_0x71825c() -> ! {
    todo!("0x71825c __ZSt13__heap_selectIPPKN3RBX5JointEPFbS3_S3_EEvT_S7_S7_T0_")
}

#[doc(alias = "void std::sort_heap<RBX::Joint const**,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint const**,RBX::Joint const**,bool (*)(RBX::Joint const*,RBX::Joint const*))")]
// 0x7182cc — __ZSt9sort_heapIPPKN3RBX5JointEPFbS3_S3_EEvT_S7_T0_
pub fn stub_0x7182cc() -> ! {
    todo!("0x7182cc __ZSt9sort_heapIPPKN3RBX5JointEPFbS3_S3_EEvT_S7_T0_")
}

#[doc(alias = "void std::pop_heap<RBX::Joint const**,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint const**,RBX::Joint const**,bool (*)(RBX::Joint const*,RBX::Joint const*))")]
// 0x7182f4 — __ZSt8pop_heapIPPKN3RBX5JointEPFbS3_S3_EEvT_S7_T0_
pub fn stub_0x7182f4() -> ! {
    todo!("0x7182f4 __ZSt8pop_heapIPPKN3RBX5JointEPFbS3_S3_EEvT_S7_T0_")
}

#[doc(alias = "void std::__adjust_heap<RBX::Joint const**,int,RBX::Joint const*,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint const**,int,int,RBX::Joint const*,bool (*)(RBX::Joint const*,RBX::Joint const*))")]
// 0x718318 — __ZSt13__adjust_heapIPPKN3RBX5JointEiS3_PFbS3_S3_EEvT_T0_S8_T1_T2_
pub fn stub_0x718318() -> ! {
    todo!("0x718318 __ZSt13__adjust_heapIPPKN3RBX5JointEiS3_PFbS3_S3_EEvT_T0_S8_T1_T2_")
}

#[doc(alias = "void std::__introsort_loop<RBX::Joint **,int,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint **,RBX::Joint **,int,bool (*)(RBX::Joint const*,RBX::Joint const*))")]
// 0x7183c4 — __ZSt16__introsort_loopIPPN3RBX5JointEiPFbPKS1_S5_EEvT_S8_T0_T1_
pub fn stub_0x7183c4() -> ! {
    todo!("0x7183c4 __ZSt16__introsort_loopIPPN3RBX5JointEiPFbPKS1_S5_EEvT_S8_T0_T1_")
}

#[doc(alias = "void std::__final_insertion_sort<RBX::Joint **,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint **,RBX::Joint **,bool (*)(RBX::Joint const*,RBX::Joint const*))")]
// 0x718480 — __ZSt22__final_insertion_sortIPPN3RBX5JointEPFbPKS1_S5_EEvT_S8_T0_
pub fn stub_0x718480() -> ! {
    todo!("0x718480 __ZSt22__final_insertion_sortIPPN3RBX5JointEPFbPKS1_S5_EEvT_S8_T0_")
}

#[doc(alias = "void std::__insertion_sort<RBX::Joint **,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint **,RBX::Joint **,bool (*)(RBX::Joint const*,RBX::Joint const*))")]
// 0x7184ec — __ZSt16__insertion_sortIPPN3RBX5JointEPFbPKS1_S5_EEvT_S8_T0_
pub fn stub_0x7184ec() -> ! {
    todo!("0x7184ec __ZSt16__insertion_sortIPPN3RBX5JointEPFbPKS1_S5_EEvT_S8_T0_")
}

#[doc(alias = "RBX::Joint ** std::__unguarded_partition<RBX::Joint **,RBX::Joint *,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint **,RBX::Joint **,RBX::Joint *,bool (*)(RBX::Joint const*,RBX::Joint const*))")]
// 0x718568 — __ZSt21__unguarded_partitionIPPN3RBX5JointES2_PFbPKS1_S5_EET_S8_S8_T0_T1_
pub fn stub_0x718568() -> ! {
    todo!("0x718568 __ZSt21__unguarded_partitionIPPN3RBX5JointES2_PFbPKS1_S5_EET_S8_S8_T0_T1_")
}

#[doc(alias = "void std::__heap_select<RBX::Joint **,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint **,RBX::Joint **,RBX::Joint **,bool (*)(RBX::Joint const*,RBX::Joint const*))")]
// 0x7185b0 — __ZSt13__heap_selectIPPN3RBX5JointEPFbPKS1_S5_EEvT_S8_S8_T0_
pub fn stub_0x7185b0() -> ! {
    todo!("0x7185b0 __ZSt13__heap_selectIPPN3RBX5JointEPFbPKS1_S5_EEvT_S8_S8_T0_")
}

#[doc(alias = "void std::sort_heap<RBX::Joint **,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint **,RBX::Joint **,bool (*)(RBX::Joint const*,RBX::Joint const*))")]
// 0x718620 — __ZSt9sort_heapIPPN3RBX5JointEPFbPKS1_S5_EEvT_S8_T0_
pub fn stub_0x718620() -> ! {
    todo!("0x718620 __ZSt9sort_heapIPPN3RBX5JointEPFbPKS1_S5_EEvT_S8_T0_")
}

#[doc(alias = "void std::pop_heap<RBX::Joint **,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint **,RBX::Joint **,bool (*)(RBX::Joint const*,RBX::Joint const*))")]
// 0x718648 — __ZSt8pop_heapIPPN3RBX5JointEPFbPKS1_S5_EEvT_S8_T0_
pub fn stub_0x718648() -> ! {
    todo!("0x718648 __ZSt8pop_heapIPPN3RBX5JointEPFbPKS1_S5_EEvT_S8_T0_")
}

#[doc(alias = "void std::__adjust_heap<RBX::Joint **,int,RBX::Joint *,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint **,int,int,RBX::Joint *,bool (*)(RBX::Joint const*,RBX::Joint const*))")]
// 0x71866c — __ZSt13__adjust_heapIPPN3RBX5JointEiS2_PFbPKS1_S5_EEvT_T0_S9_T1_T2_
pub fn stub_0x71866c() -> ! {
    todo!("0x71866c __ZSt13__adjust_heapIPPN3RBX5JointEiS2_PFbPKS1_S5_EEvT_T0_S9_T1_T2_")
}

#[doc(alias = "RBX::Velocity::operator==(RBX::Velocity const&)const")]
// 0x718c64 — __ZNK3RBX8VelocityeqERKS0_
pub fn stub_0x718c64() -> ! {
    todo!("0x718c64 __ZNK3RBX8VelocityeqERKS0_")
}

#[doc(alias = "RBX::AssemblyHistory::AssemblyHistory(RBX::Assembly &)")]
// 0x718e44 — __ZN3RBX15AssemblyHistoryC1ERNS_8AssemblyE
pub fn stub_0x718e44() -> ! {
    todo!("0x718e44 __ZN3RBX15AssemblyHistoryC1ERNS_8AssemblyE")
}

#[doc(alias = "RBX::AssemblyHistory::getAssemblyPhysicsCoord(RBX::Assembly &)")]
// 0x718e70 — __ZN3RBX15AssemblyHistory23getAssemblyPhysicsCoordERNS_8AssemblyE
pub fn stub_0x718e70() -> ! {
    todo!("0x718e70 __ZN3RBX15AssemblyHistory23getAssemblyPhysicsCoordERNS_8AssemblyE")
}

#[doc(alias = "RBX::AssemblyHistory::~AssemblyHistory()")]
// 0x718f0c — __ZN3RBX15AssemblyHistoryD1Ev
pub fn stub_0x718f0c() -> ! {
    todo!("0x718f0c __ZN3RBX15AssemblyHistoryD1Ev")
}

#[doc(alias = "RBX::AssemblyHistory::sampleAndNotMoving(RBX::Assembly &)")]
// 0x718f20 — __ZN3RBX15AssemblyHistory18sampleAndNotMovingERNS_8AssemblyE
pub fn stub_0x718f20() -> ! {
    todo!("0x718f20 __ZN3RBX15AssemblyHistory18sampleAndNotMovingERNS_8AssemblyE")
}

#[doc(alias = "RBX::AssemblyHistory::maxDeviationSquared(void)")]
// 0x718f8c — __ZN3RBX15AssemblyHistory19maxDeviationSquaredEv
pub fn stub_0x718f8c() -> ! {
    todo!("0x718f8c __ZN3RBX15AssemblyHistory19maxDeviationSquaredEv")
}

#[doc(alias = "RBX::AssemblyHistory::preventNeighborSleep(void)")]
// 0x719068 — __ZN3RBX15AssemblyHistory20preventNeighborSleepEv
pub fn stub_0x719068() -> ! {
    todo!("0x719068 __ZN3RBX15AssemblyHistory20preventNeighborSleepEv")
}

#[doc(alias = "RBX::AssemblyHistory::wakeUp(void)")]
// 0x71908c — __ZN3RBX15AssemblyHistory6wakeUpEv
pub fn stub_0x71908c() -> ! {
    todo!("0x71908c __ZN3RBX15AssemblyHistory6wakeUpEv")
}

#[doc(alias = "RBX::Average<RBX::PhysicsCoord>::sample(RBX::PhysicsCoord,bool)")]
// 0x719094 — __ZN3RBX7AverageINS_12PhysicsCoordEE6sampleES1_b
pub fn stub_0x719094() -> ! {
    todo!("0x719094 __ZN3RBX7AverageINS_12PhysicsCoordEE6sampleES1_b")
}

#[doc(alias = "RBX::Average<RBX::PhysicsCoord>::getAverage(void)const")]
// 0x7190dc — __ZNK3RBX7AverageINS_12PhysicsCoordEE10getAverageEv
pub fn stub_0x7190dc() -> ! {
    todo!("0x7190dc __ZNK3RBX7AverageINS_12PhysicsCoordEE10getAverageEv")
}

#[doc(alias = "RBX::Average<RBX::PhysicsCoord>::Average(unsigned long,RBX::PhysicsCoord)")]
// 0x7191d0 — __ZN3RBX7AverageINS_12PhysicsCoordEEC2EmS1_
pub fn stub_0x7191d0() -> ! {
    todo!("0x7191d0 __ZN3RBX7AverageINS_12PhysicsCoordEEC2EmS1_")
}

#[doc(alias = "std::vector<RBX::PhysicsCoord,std::allocator<RBX::PhysicsCoord>>::resize(unsigned long,RBX::PhysicsCoord)")]
// 0x7192a8 — __ZNSt6vectorIN3RBX12PhysicsCoordESaIS1_EE6resizeEmS1_
pub fn stub_0x7192a8() -> ! {
    todo!("0x7192a8 __ZNSt6vectorIN3RBX12PhysicsCoordESaIS1_EE6resizeEmS1_")
}

#[doc(alias = "std::vector<RBX::PhysicsCoord,std::allocator<RBX::PhysicsCoord>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::PhysicsCoord*,std::vector<RBX::PhysicsCoord,std::allocator<RBX::PhysicsCoord>>>,unsigned long,RBX::PhysicsCoord const&)")]
// 0x7192f4 — __ZNSt6vectorIN3RBX12PhysicsCoordESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
pub fn stub_0x7192f4() -> ! {
    todo!("0x7192f4 __ZNSt6vectorIN3RBX12PhysicsCoordESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_")
}

#[doc(alias = "void std::fill<RBX::PhysicsCoord *,RBX::PhysicsCoord>(RBX::PhysicsCoord *,RBX::PhysicsCoord *,RBX::PhysicsCoord const&)")]
// 0x7195c4 — __ZSt4fillIPN3RBX12PhysicsCoordES1_EvT_S3_RKT0_
pub fn stub_0x7195c4() -> ! {
    todo!("0x7195c4 __ZSt4fillIPN3RBX12PhysicsCoordES1_EvT_S3_RKT0_")
}

#[doc(alias = "std::_Vector_base<RBX::PhysicsCoord,std::allocator<RBX::PhysicsCoord>>::_M_allocate(unsigned long)")]
// 0x7195fc — __ZNSt12_Vector_baseIN3RBX12PhysicsCoordESaIS1_EE11_M_allocateEm
pub fn stub_0x7195fc() -> ! {
    todo!("0x7195fc __ZNSt12_Vector_baseIN3RBX12PhysicsCoordESaIS1_EE11_M_allocateEm")
}

#[doc(alias = "RBX::PhysicsCoord * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::PhysicsCoord *,RBX::PhysicsCoord *>(RBX::PhysicsCoord *,RBX::PhysicsCoord *,RBX::PhysicsCoord *)")]
// 0x719620 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX12PhysicsCoordES5_EET0_T_S7_S6_
pub fn stub_0x719620() -> ! {
    todo!("0x719620 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX12PhysicsCoordES5_EET0_T_S7_S6_")
}

#[doc(alias = "RBX::AssemblyStage::AssemblyStage(RBX::IStage *,RBX::World *)")]
// 0x719788 — __ZN3RBX13AssemblyStageC1EPNS_6IStageEPNS_5WorldE
pub fn stub_0x719788() -> ! {
    todo!("0x719788 __ZN3RBX13AssemblyStageC1EPNS_6IStageEPNS_5WorldE")
}

#[doc(alias = "RBX::AssemblyStage::AssemblyStage(RBX::IStage *,RBX::World *)")]
// 0x71978c — __ZN3RBX13AssemblyStageC2EPNS_6IStageEPNS_5WorldE
pub fn stub_0x71978c() -> ! {
    todo!("0x71978c __ZN3RBX13AssemblyStageC2EPNS_6IStageEPNS_5WorldE")
}

#[doc(alias = "RBX::AssemblyStage::~AssemblyStage()")]
// 0x719874 — __ZN3RBX13AssemblyStageD0Ev
pub fn stub_0x719874() -> ! {
    todo!("0x719874 __ZN3RBX13AssemblyStageD0Ev")
}

#[doc(alias = "RBX::AssemblyStage::~AssemblyStage()")]
// 0x719914 — __ZN3RBX13AssemblyStageD1Ev
pub fn stub_0x719914() -> ! {
    todo!("0x719914 __ZN3RBX13AssemblyStageD1Ev")
}

#[doc(alias = "RBX::AssemblyStage::onEngineChanging(RBX::Primitive *)")]
// 0x719918 — __ZN3RBX13AssemblyStage16onEngineChangingEPNS_9PrimitiveE
pub fn stub_0x719918() -> ! {
    todo!("0x719918 __ZN3RBX13AssemblyStage16onEngineChangingEPNS_9PrimitiveE")
}

#[doc(alias = "RBX::AssemblyStage::onSimulateAssemblyDescendentRemoving(RBX::Assembly *)")]
// 0x71995c — __ZN3RBX13AssemblyStage36onSimulateAssemblyDescendentRemovingEPNS_8AssemblyE
pub fn stub_0x71995c() -> ! {
    todo!("0x71995c __ZN3RBX13AssemblyStage36onSimulateAssemblyDescendentRemovingEPNS_8AssemblyE")
}

#[doc(alias = "RBX::AssemblyStage::onEngineChanged(RBX::Assembly *)")]
// 0x71997c — __ZN3RBX13AssemblyStage15onEngineChangedEPNS_8AssemblyE
pub fn stub_0x71997c() -> ! {
    todo!("0x71997c __ZN3RBX13AssemblyStage15onEngineChangedEPNS_8AssemblyE")
}

#[doc(alias = "RBX::AssemblyStage::onSimulateAssemblyDescendentAdded(RBX::Assembly *)")]
// 0x7199dc — __ZN3RBX13AssemblyStage33onSimulateAssemblyDescendentAddedEPNS_8AssemblyE
pub fn stub_0x7199dc() -> ! {
    todo!("0x7199dc __ZN3RBX13AssemblyStage33onSimulateAssemblyDescendentAddedEPNS_8AssemblyE")
}

#[doc(alias = "RBX::AssemblyStage::onSimulateAssemblyRootAdded(RBX::Assembly *)")]
// 0x719a00 — __ZN3RBX13AssemblyStage27onSimulateAssemblyRootAddedEPNS_8AssemblyE
pub fn stub_0x719a00() -> ! {
    todo!("0x719a00 __ZN3RBX13AssemblyStage27onSimulateAssemblyRootAddedEPNS_8AssemblyE")
}

#[doc(alias = "RBX::AssemblyStage::onSimulateAssemblyRootRemoving(RBX::Assembly *)")]
// 0x719a24 — __ZN3RBX13AssemblyStage30onSimulateAssemblyRootRemovingEPNS_8AssemblyE
pub fn stub_0x719a24() -> ! {
    todo!("0x719a24 __ZN3RBX13AssemblyStage30onSimulateAssemblyRootRemovingEPNS_8AssemblyE")
}

#[doc(alias = "RBX::AssemblyStage::onFixedAssemblyRootAdded(RBX::Assembly *)")]
// 0x719a44 — __ZN3RBX13AssemblyStage24onFixedAssemblyRootAddedEPNS_8AssemblyE
pub fn stub_0x719a44() -> ! {
    todo!("0x719a44 __ZN3RBX13AssemblyStage24onFixedAssemblyRootAddedEPNS_8AssemblyE")
}

#[doc(alias = "RBX::AssemblyStage::onFixedAssemblyRootRemoving(RBX::Assembly *)")]
// 0x719a60 — __ZN3RBX13AssemblyStage27onFixedAssemblyRootRemovingEPNS_8AssemblyE
pub fn stub_0x719a60() -> ! {
    todo!("0x719a60 __ZN3RBX13AssemblyStage27onFixedAssemblyRootRemovingEPNS_8AssemblyE")
}

#[doc(alias = "RBX::AssemblyStage::onNoSimulateAssemblyDescendentAdded(RBX::Assembly *)")]
// 0x719a78 — __ZN3RBX13AssemblyStage35onNoSimulateAssemblyDescendentAddedEPNS_8AssemblyE
pub fn stub_0x719a78() -> ! {
    todo!("0x719a78 __ZN3RBX13AssemblyStage35onNoSimulateAssemblyDescendentAddedEPNS_8AssemblyE")
}

#[doc(alias = "RBX::AssemblyStage::onNoSimulateAssemblyDescendentRemoving(RBX::Assembly *)")]
// 0x719a94 — __ZN3RBX13AssemblyStage38onNoSimulateAssemblyDescendentRemovingEPNS_8AssemblyE
pub fn stub_0x719a94() -> ! {
    todo!("0x719a94 __ZN3RBX13AssemblyStage38onNoSimulateAssemblyDescendentRemovingEPNS_8AssemblyE")
}

#[doc(alias = "RBX::AssemblyStage::getStageType(void)const")]
// 0x719aac — __ZNK3RBX13AssemblyStage12getStageTypeEv
pub fn stub_0x719aac() -> ! {
    todo!("0x719aac __ZNK3RBX13AssemblyStage12getStageTypeEv")
}

#[doc(alias = "RBX::Ball::getMomentSolid(float)const")]
// 0x719be0 — __ZNK3RBX4Ball14getMomentSolidEf
pub fn stub_0x719be0() -> ! {
    todo!("0x719be0 __ZNK3RBX4Ball14getMomentSolidEf")
}

#[doc(alias = "RBX::Ball::getVolume(void)const")]
// 0x719c28 — __ZNK3RBX4Ball9getVolumeEv
pub fn stub_0x719c28() -> ! {
    todo!("0x719c28 __ZNK3RBX4Ball9getVolumeEv")
}

#[doc(alias = "RBX::Ball::getPlaneFromSurface(unsigned long)const")]
// 0x719e7c — __ZNK3RBX4Ball19getPlaneFromSurfaceEm
pub fn stub_0x719e7c() -> ! {
    todo!("0x719e7c __ZNK3RBX4Ball19getPlaneFromSurfaceEm")
}

#[doc(alias = "RBX::Ball::getSurfaceNormalInBody(unsigned long)const")]
// 0x719f5c — __ZNK3RBX4Ball22getSurfaceNormalInBodyEm
pub fn stub_0x719f5c() -> ! {
    todo!("0x719f5c __ZNK3RBX4Ball22getSurfaceNormalInBodyEm")
}

#[doc(alias = "RBX::Ball::getSurfaceVertInBody(unsigned long,int)const")]
// 0x719fb4 — __ZNK3RBX4Ball20getSurfaceVertInBodyEmi
pub fn stub_0x719fb4() -> ! {
    todo!("0x719fb4 __ZNK3RBX4Ball20getSurfaceVertInBodyEmi")
}

#[doc(alias = "RBX::Ball::getNumVertsInSurface(unsigned long)const")]
// 0x71a194 — __ZNK3RBX4Ball20getNumVertsInSurfaceEm
pub fn stub_0x71a194() -> ! {
    todo!("0x71a194 __ZNK3RBX4Ball20getNumVertsInSurfaceEm")
}

#[doc(alias = "RBX::Ball::getSurfaceCoordInBody(unsigned long)const")]
// 0x71a230 — __ZNK3RBX4Ball21getSurfaceCoordInBodyEm
pub fn stub_0x71a230() -> ! {
    todo!("0x71a230 __ZNK3RBX4Ball21getSurfaceCoordInBodyEm")
}

#[doc(alias = "RBX::BallPolyContact::BallPolyContact(RBX::Primitive *,RBX::Primitive *)")]
// 0x71a334 — __ZN3RBX15BallPolyContactC1EPNS_9PrimitiveES2_
pub fn stub_0x71a334() -> ! {
    todo!("0x71a334 __ZN3RBX15BallPolyContactC1EPNS_9PrimitiveES2_")
}

#[doc(alias = "RBX::BallPolyContact::BallPolyContact(RBX::Primitive *,RBX::Primitive *)")]
// 0x71a338 — __ZN3RBX15BallPolyContactC2EPNS_9PrimitiveES2_
pub fn stub_0x71a338() -> ! {
    todo!("0x71a338 __ZN3RBX15BallPolyContactC2EPNS_9PrimitiveES2_")
}

#[doc(alias = "RBX::BallPolyContact::findClosestFeatures(RBX::FixedArray<RBX::PolyConnector *,40ul> &)")]
// 0x71a4d0 — __ZN3RBX15BallPolyContact19findClosestFeaturesERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE
pub fn stub_0x71a4d0() -> ! {
    todo!("0x71a4d0 __ZN3RBX15BallPolyContact19findClosestFeaturesERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE")
}

#[doc(alias = "RBX::BallPolyContact::newBallPlaneConnector(RBX::POLY::Face const*)")]
// 0x71a7b0 — __ZN3RBX15BallPolyContact21newBallPlaneConnectorEPKNS_4POLY4FaceE
pub fn stub_0x71a7b0() -> ! {
    todo!("0x71a7b0 __ZN3RBX15BallPolyContact21newBallPlaneConnectorEPKNS_4POLY4FaceE")
}

#[doc(alias = "RBX::BallPolyContact::newBallEdgeConnector(RBX::POLY::Edge const*)")]
// 0x71aa04 — __ZN3RBX15BallPolyContact20newBallEdgeConnectorEPKNS_4POLY4EdgeE
pub fn stub_0x71aa04() -> ! {
    todo!("0x71aa04 __ZN3RBX15BallPolyContact20newBallEdgeConnectorEPKNS_4POLY4EdgeE")
}

#[doc(alias = "RBX::BallPolyContact::newBallVertexConnector(RBX::POLY::Vertex const*)")]
// 0x71ad7c — __ZN3RBX15BallPolyContact22newBallVertexConnectorEPKNS_4POLY6VertexE
pub fn stub_0x71ad7c() -> ! {
    todo!("0x71ad7c __ZN3RBX15BallPolyContact22newBallVertexConnectorEPKNS_4POLY6VertexE")
}

#[doc(alias = "RBX::BallPolyContact::generateDataForMovingAssemblyStage(void)")]
// 0x71af10 — __ZN3RBX15BallPolyContact34generateDataForMovingAssemblyStageEv
pub fn stub_0x71af10() -> ! {
    todo!("0x71af10 __ZN3RBX15BallPolyContact34generateDataForMovingAssemblyStageEv")
}

#[doc(alias = "RBX::Allocator<RBX::BallPolyContact>::Allocator(void)")]
// 0x71af14 — __ZN3RBX9AllocatorINS_15BallPolyContactEEC2Ev
pub fn stub_0x71af14() -> ! {
    todo!("0x71af14 __ZN3RBX9AllocatorINS_15BallPolyContactEEC2Ev")
}

#[doc(alias = "RBX::BallPolyContact::~BallPolyContact()")]
// 0x71af78 — __ZN3RBX15BallPolyContactD1Ev
pub fn stub_0x71af78() -> ! {
    todo!("0x71af78 __ZN3RBX15BallPolyContactD1Ev")
}
