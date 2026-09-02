//! rendering shard 468 — 120 stubs 0x71789c..0x71e87c EA-sorted asc global gap filler not yet in rbx_rendering (global gap filler, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Global gap filler fallback EA asc not yet in rbx_rendering (50510->50630 distinct, fallback after 0x71789c).
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc global gap filler not yet in rbx_rendering

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x71789c — __ZN3RBX8Assembly16getAssemblyEdgesEv
// type: _DWORD __fastcall(RBX::Assembly *__hidden this)
#[doc(alias = "RBX::Assembly::getAssemblyEdges(void)")]
#[doc(alias = "__ZN3RBX8Assembly16getAssemblyEdgesEv")]
pub fn stub_71789c() -> ! {
    todo!("0x71789c RBX::Assembly::getAssemblyEdges(void)")
}

// 0x717978 — __ZN3RBX8Assembly27computeIsGroundingPrimitiveEPKNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::Assembly *__hidden this, const RBX::Primitive *)
#[doc(alias = "RBX::Assembly::computeIsGroundingPrimitive(RBX::Primitive const*)")]
#[doc(alias = "__ZN3RBX8Assembly27computeIsGroundingPrimitiveEPKNS_9PrimitiveE")]
pub fn stub_717978() -> ! {
    todo!("0x717978 RBX::Assembly::computeIsGroundingPrimitive(RBX::Primitive const*)")
}

// 0x71798c — __ZNK3RBX8Assembly17computeIsGroundedEv
// type: _DWORD __fastcall(RBX::Assembly *__hidden this)
#[doc(alias = "RBX::Assembly::computeIsGrounded(void)const")]
#[doc(alias = "__ZNK3RBX8Assembly17computeIsGroundedEv")]
pub fn stub_71798c() -> ! {
    todo!("0x71798c RBX::Assembly::computeIsGrounded(void)const")
}

// 0x717a6c — __ZN3RBX28notifyAssemblyPrimitiveMovedEPNS_9PrimitiveEb
// type: _DWORD __fastcall(RBX *__hidden this, RBX::Primitive *, bool)
#[doc(alias = "RBX::notifyAssemblyPrimitiveMoved(RBX::Primitive *,bool)")]
#[doc(alias = "__ZN3RBX28notifyAssemblyPrimitiveMovedEPNS_9PrimitiveEb")]
pub fn stub_717a6c() -> ! {
    todo!("0x717a6c RBX::notifyAssemblyPrimitiveMoved(RBX::Primitive *,bool)")
}

// 0x717b7c — __ZN3RBX8Assembly30notifyMovedFromInternalPhysicsEv
// type: _DWORD __fastcall(RBX::Assembly *__hidden this)
#[doc(alias = "RBX::Assembly::notifyMovedFromInternalPhysics(void)")]
#[doc(alias = "__ZN3RBX8Assembly30notifyMovedFromInternalPhysicsEv")]
pub fn stub_717b7c() -> ! {
    todo!("0x717b7c RBX::Assembly::notifyMovedFromInternalPhysics(void)")
}

// 0x717bec — __ZNK3RBX2PVeqERKS0_
#[doc(alias = "RBX::PV::operator==(RBX::PV const&)const")]
#[doc(alias = "__ZNK3RBX2PVeqERKS0_")]
pub fn stub_717bec() -> ! {
    todo!("0x717bec RBX::PV::operator==(RBX::PV const&)const")
}

// 0x717cac — __ZN3RBX8Assembly19visitPrimitivesImplIN5boost3_bi6bind_tIvPFvPNS_9PrimitiveEbENS3_5list2INS2_3argILi1EEENS3_5valueIbEEEEEEEEvT_S6_
#[doc(alias = "void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,void (*)(RBX::Primitive *,bool),boost::_bi::list2<boost::arg<1>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(RBX::Primitive *,bool),boost::_bi::list2<boost::arg<1>,boost::_bi::value<bool>>>,RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX8Assembly19visitPrimitivesImplIN5boost3_bi6bind_tIvPFvPNS_9PrimitiveEbENS3_5list2INS2_3argILi1EEENS3_5valueIbEEEEEEEEvT_S6_")]
pub fn stub_717cac() -> ! {
    todo!("0x717cac void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,void (*)(RBX::Primitive *,bool),boost::_bi::list2<boost::arg<1>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(RBX::Primitive *,bool),boost::_bi::list2<boost::arg<1>,boost::_bi::value<bool>>>,RBX::Primitive *)")
}

// 0x717d64 — __ZN3RBX8Assembly19visitPrimitivesImplIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvS0_PNS_9PrimitiveEEENS3_5list2INS3_5valueIPS0_EENS2_3argILi1EEEEEEEEEvT_S8_
#[doc(alias = "void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Assembly,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Assembly*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Assembly,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Assembly*>,boost::arg<1>>>,RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX8Assembly19visitPrimitivesImplIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvS0_PNS_9PrimitiveEEENS3_5list2INS3_5valueIPS0_EENS2_3argILi1EEEEEEEEEvT_S8_")]
pub fn stub_717d64() -> ! {
    todo!("0x717d64 void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Assembly,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Assembly*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Assembly,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Assembly*>,boost::arg<1>>>,RBX::Primitive *)")
}

// 0x718070 — __ZSt16__introsort_loopIPPKN3RBX5JointEiPFbS3_S3_EEvT_S7_T0_T1_
#[doc(alias = "void std::__introsort_loop<RBX::Joint const**,int,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint const**,RBX::Joint const**,int,bool (*)(RBX::Joint const*,RBX::Joint const*))")]
#[doc(alias = "__ZSt16__introsort_loopIPPKN3RBX5JointEiPFbS3_S3_EEvT_S7_T0_T1_")]
pub fn stub_718070() -> ! {
    todo!("0x718070 void std::__introsort_loop<RBX::Joint const**,int,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint const**,RBX::Joint const**,int,bool (*)(RBX::Joint const*,RBX::Joint const*))")
}

// 0x71812c — __ZSt22__final_insertion_sortIPPKN3RBX5JointEPFbS3_S3_EEvT_S7_T0_
#[doc(alias = "void std::__final_insertion_sort<RBX::Joint const**,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint const**,RBX::Joint const**,bool (*)(RBX::Joint const*,RBX::Joint const*))")]
#[doc(alias = "__ZSt22__final_insertion_sortIPPKN3RBX5JointEPFbS3_S3_EEvT_S7_T0_")]
pub fn stub_71812c() -> ! {
    todo!("0x71812c void std::__final_insertion_sort<RBX::Joint const**,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint const**,RBX::Joint const**,bool (*)(RBX::Joint const*,RBX::Joint const*))")
}

// 0x718198 — __ZSt16__insertion_sortIPPKN3RBX5JointEPFbS3_S3_EEvT_S7_T0_
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *, int (__fastcall *)(int, _DWORD))
#[doc(alias = "void std::__insertion_sort<RBX::Joint const**,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint const**,RBX::Joint const**,bool (*)(RBX::Joint const*,RBX::Joint const*))")]
#[doc(alias = "__ZSt16__insertion_sortIPPKN3RBX5JointEPFbS3_S3_EEvT_S7_T0_")]
pub fn stub_718198() -> ! {
    todo!("0x718198 void std::__insertion_sort<RBX::Joint const**,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint const**,RBX::Joint const**,bool (*)(RBX::Joint const*,RBX::Joint const*))")
}

// 0x718214 — __ZSt21__unguarded_partitionIPPKN3RBX5JointES3_PFbS3_S3_EET_S7_S7_T0_T1_
#[doc(alias = "RBX::Joint const** std::__unguarded_partition<RBX::Joint const**,RBX::Joint const*,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint const**,RBX::Joint const**,RBX::Joint const*,bool (*)(RBX::Joint const*,RBX::Joint const*))")]
#[doc(alias = "__ZSt21__unguarded_partitionIPPKN3RBX5JointES3_PFbS3_S3_EET_S7_S7_T0_T1_")]
pub fn stub_718214() -> ! {
    todo!("0x718214 RBX::Joint const** std::__unguarded_partition<RBX::Joint const**,RBX::Joint const*,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint const**,RBX::Joint const**,RBX::Joint const*,bool (*)(RBX::Joint const*,RBX::Joint const*))")
}

// 0x71825c — __ZSt13__heap_selectIPPKN3RBX5JointEPFbS3_S3_EEvT_S7_S7_T0_
#[doc(alias = "void std::__heap_select<RBX::Joint const**,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint const**,RBX::Joint const**,RBX::Joint const**,bool (*)(RBX::Joint const*,RBX::Joint const*))")]
#[doc(alias = "__ZSt13__heap_selectIPPKN3RBX5JointEPFbS3_S3_EEvT_S7_S7_T0_")]
pub fn stub_71825c() -> ! {
    todo!("0x71825c void std::__heap_select<RBX::Joint const**,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint const**,RBX::Joint const**,RBX::Joint const**,bool (*)(RBX::Joint const*,RBX::Joint const*))")
}

// 0x7182cc — __ZSt9sort_heapIPPKN3RBX5JointEPFbS3_S3_EEvT_S7_T0_
#[doc(alias = "void std::sort_heap<RBX::Joint const**,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint const**,RBX::Joint const**,bool (*)(RBX::Joint const*,RBX::Joint const*))")]
#[doc(alias = "__ZSt9sort_heapIPPKN3RBX5JointEPFbS3_S3_EEvT_S7_T0_")]
pub fn stub_7182cc() -> ! {
    todo!("0x7182cc void std::sort_heap<RBX::Joint const**,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint const**,RBX::Joint const**,bool (*)(RBX::Joint const*,RBX::Joint const*))")
}

// 0x7182f4 — __ZSt8pop_heapIPPKN3RBX5JointEPFbS3_S3_EEvT_S7_T0_
#[doc(alias = "void std::pop_heap<RBX::Joint const**,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint const**,RBX::Joint const**,bool (*)(RBX::Joint const*,RBX::Joint const*))")]
#[doc(alias = "__ZSt8pop_heapIPPKN3RBX5JointEPFbS3_S3_EEvT_S7_T0_")]
pub fn stub_7182f4() -> ! {
    todo!("0x7182f4 void std::pop_heap<RBX::Joint const**,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint const**,RBX::Joint const**,bool (*)(RBX::Joint const*,RBX::Joint const*))")
}

// 0x718318 — __ZSt13__adjust_heapIPPKN3RBX5JointEiS3_PFbS3_S3_EEvT_T0_S8_T1_T2_
#[doc(alias = "void std::__adjust_heap<RBX::Joint const**,int,RBX::Joint const*,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint const**,int,int,RBX::Joint const*,bool (*)(RBX::Joint const*,RBX::Joint const*))")]
#[doc(alias = "__ZSt13__adjust_heapIPPKN3RBX5JointEiS3_PFbS3_S3_EEvT_T0_S8_T1_T2_")]
pub fn stub_718318() -> ! {
    todo!("0x718318 void std::__adjust_heap<RBX::Joint const**,int,RBX::Joint const*,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint const**,int,int,RBX::Joint const*,bool (*)(RBX::Joint const*,RBX::Joint const*))")
}

// 0x7183c4 — __ZSt16__introsort_loopIPPN3RBX5JointEiPFbPKS1_S5_EEvT_S8_T0_T1_
#[doc(alias = "void std::__introsort_loop<RBX::Joint **,int,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint **,RBX::Joint **,int,bool (*)(RBX::Joint const*,RBX::Joint const*))")]
#[doc(alias = "__ZSt16__introsort_loopIPPN3RBX5JointEiPFbPKS1_S5_EEvT_S8_T0_T1_")]
pub fn stub_7183c4() -> ! {
    todo!("0x7183c4 void std::__introsort_loop<RBX::Joint **,int,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint **,RBX::Joint **,int,bool (*)(RBX::Joint const*,RBX::Joint const*))")
}

// 0x718480 — __ZSt22__final_insertion_sortIPPN3RBX5JointEPFbPKS1_S5_EEvT_S8_T0_
#[doc(alias = "void std::__final_insertion_sort<RBX::Joint **,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint **,RBX::Joint **,bool (*)(RBX::Joint const*,RBX::Joint const*))")]
#[doc(alias = "__ZSt22__final_insertion_sortIPPN3RBX5JointEPFbPKS1_S5_EEvT_S8_T0_")]
pub fn stub_718480() -> ! {
    todo!("0x718480 void std::__final_insertion_sort<RBX::Joint **,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint **,RBX::Joint **,bool (*)(RBX::Joint const*,RBX::Joint const*))")
}

// 0x7184ec — __ZSt16__insertion_sortIPPN3RBX5JointEPFbPKS1_S5_EEvT_S8_T0_
#[doc(alias = "void std::__insertion_sort<RBX::Joint **,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint **,RBX::Joint **,bool (*)(RBX::Joint const*,RBX::Joint const*))")]
#[doc(alias = "__ZSt16__insertion_sortIPPN3RBX5JointEPFbPKS1_S5_EEvT_S8_T0_")]
pub fn stub_7184ec() -> ! {
    todo!("0x7184ec void std::__insertion_sort<RBX::Joint **,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint **,RBX::Joint **,bool (*)(RBX::Joint const*,RBX::Joint const*))")
}

// 0x718568 — __ZSt21__unguarded_partitionIPPN3RBX5JointES2_PFbPKS1_S5_EET_S8_S8_T0_T1_
#[doc(alias = "RBX::Joint ** std::__unguarded_partition<RBX::Joint **,RBX::Joint *,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint **,RBX::Joint **,RBX::Joint *,bool (*)(RBX::Joint const*,RBX::Joint const*))")]
#[doc(alias = "__ZSt21__unguarded_partitionIPPN3RBX5JointES2_PFbPKS1_S5_EET_S8_S8_T0_T1_")]
pub fn stub_718568() -> ! {
    todo!("0x718568 RBX::Joint ** std::__unguarded_partition<RBX::Joint **,RBX::Joint *,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint **,RBX::Joint **,RBX::Joint *,bool (*)(RBX::Joint const*,RBX::Joint const*))")
}

// 0x7185b0 — __ZSt13__heap_selectIPPN3RBX5JointEPFbPKS1_S5_EEvT_S8_S8_T0_
#[doc(alias = "void std::__heap_select<RBX::Joint **,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint **,RBX::Joint **,RBX::Joint **,bool (*)(RBX::Joint const*,RBX::Joint const*))")]
#[doc(alias = "__ZSt13__heap_selectIPPN3RBX5JointEPFbPKS1_S5_EEvT_S8_S8_T0_")]
pub fn stub_7185b0() -> ! {
    todo!("0x7185b0 void std::__heap_select<RBX::Joint **,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint **,RBX::Joint **,RBX::Joint **,bool (*)(RBX::Joint const*,RBX::Joint const*))")
}

// 0x718620 — __ZSt9sort_heapIPPN3RBX5JointEPFbPKS1_S5_EEvT_S8_T0_
#[doc(alias = "void std::sort_heap<RBX::Joint **,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint **,RBX::Joint **,bool (*)(RBX::Joint const*,RBX::Joint const*))")]
#[doc(alias = "__ZSt9sort_heapIPPN3RBX5JointEPFbPKS1_S5_EEvT_S8_T0_")]
pub fn stub_718620() -> ! {
    todo!("0x718620 void std::sort_heap<RBX::Joint **,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint **,RBX::Joint **,bool (*)(RBX::Joint const*,RBX::Joint const*))")
}

// 0x718648 — __ZSt8pop_heapIPPN3RBX5JointEPFbPKS1_S5_EEvT_S8_T0_
#[doc(alias = "void std::pop_heap<RBX::Joint **,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint **,RBX::Joint **,bool (*)(RBX::Joint const*,RBX::Joint const*))")]
#[doc(alias = "__ZSt8pop_heapIPPN3RBX5JointEPFbPKS1_S5_EEvT_S8_T0_")]
pub fn stub_718648() -> ! {
    todo!("0x718648 void std::pop_heap<RBX::Joint **,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint **,RBX::Joint **,bool (*)(RBX::Joint const*,RBX::Joint const*))")
}

// 0x71866c — __ZSt13__adjust_heapIPPN3RBX5JointEiS2_PFbPKS1_S5_EEvT_T0_S9_T1_T2_
#[doc(alias = "void std::__adjust_heap<RBX::Joint **,int,RBX::Joint *,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint **,int,int,RBX::Joint *,bool (*)(RBX::Joint const*,RBX::Joint const*))")]
#[doc(alias = "__ZSt13__adjust_heapIPPN3RBX5JointEiS2_PFbPKS1_S5_EEvT_T0_S9_T1_T2_")]
pub fn stub_71866c() -> ! {
    todo!("0x71866c void std::__adjust_heap<RBX::Joint **,int,RBX::Joint *,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint **,int,int,RBX::Joint *,bool (*)(RBX::Joint const*,RBX::Joint const*))")
}

// 0x718c64 — __ZNK3RBX8VelocityeqERKS0_
#[doc(alias = "RBX::Velocity::operator==(RBX::Velocity const&)const")]
#[doc(alias = "__ZNK3RBX8VelocityeqERKS0_")]
pub fn stub_718c64() -> ! {
    todo!("0x718c64 RBX::Velocity::operator==(RBX::Velocity const&)const")
}

// 0x718ce0 — __GLOBAL__I_a_302
#[doc(alias = "global constructor keyed to_a_302")]
#[doc(alias = "__GLOBAL__I_a_302")]
pub fn stub_718ce0() -> ! {
    todo!("0x718ce0 global constructor keyed to_a_302")
}

// 0x718e44 — __ZN3RBX15AssemblyHistoryC1ERNS_8AssemblyE
// type: _DWORD __fastcall(RBX::AssemblyHistory *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::AssemblyHistory::AssemblyHistory(RBX::Assembly &)")]
#[doc(alias = "__ZN3RBX15AssemblyHistoryC1ERNS_8AssemblyE")]
pub fn stub_718e44() -> ! {
    todo!("0x718e44 RBX::AssemblyHistory::AssemblyHistory(RBX::Assembly &)")
}

// 0x718e70 — __ZN3RBX15AssemblyHistory23getAssemblyPhysicsCoordERNS_8AssemblyE
// type: _DWORD __fastcall(RBX::AssemblyHistory *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::AssemblyHistory::getAssemblyPhysicsCoord(RBX::Assembly &)")]
#[doc(alias = "__ZN3RBX15AssemblyHistory23getAssemblyPhysicsCoordERNS_8AssemblyE")]
pub fn stub_718e70() -> ! {
    todo!("0x718e70 RBX::AssemblyHistory::getAssemblyPhysicsCoord(RBX::Assembly &)")
}

// 0x718f0c — __ZN3RBX15AssemblyHistoryD1Ev
// type: void __fastcall(RBX::AssemblyHistory *__hidden this)
#[doc(alias = "RBX::AssemblyHistory::~AssemblyHistory()")]
#[doc(alias = "__ZN3RBX15AssemblyHistoryD1Ev")]
pub fn stub_718f0c() -> ! {
    todo!("0x718f0c RBX::AssemblyHistory::~AssemblyHistory()")
}

// 0x718f20 — __ZN3RBX15AssemblyHistory18sampleAndNotMovingERNS_8AssemblyE
// type: _DWORD __fastcall(RBX::AssemblyHistory *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::AssemblyHistory::sampleAndNotMoving(RBX::Assembly &)")]
#[doc(alias = "__ZN3RBX15AssemblyHistory18sampleAndNotMovingERNS_8AssemblyE")]
pub fn stub_718f20() -> ! {
    todo!("0x718f20 RBX::AssemblyHistory::sampleAndNotMoving(RBX::Assembly &)")
}

// 0x718f8c — __ZN3RBX15AssemblyHistory19maxDeviationSquaredEv
// type: _DWORD __fastcall(RBX::AssemblyHistory *__hidden this)
#[doc(alias = "RBX::AssemblyHistory::maxDeviationSquared(void)")]
#[doc(alias = "__ZN3RBX15AssemblyHistory19maxDeviationSquaredEv")]
pub fn stub_718f8c() -> ! {
    todo!("0x718f8c RBX::AssemblyHistory::maxDeviationSquared(void)")
}

// 0x719068 — __ZN3RBX15AssemblyHistory20preventNeighborSleepEv
// type: _DWORD __fastcall(RBX::AssemblyHistory *__hidden this)
#[doc(alias = "RBX::AssemblyHistory::preventNeighborSleep(void)")]
#[doc(alias = "__ZN3RBX15AssemblyHistory20preventNeighborSleepEv")]
pub fn stub_719068() -> ! {
    todo!("0x719068 RBX::AssemblyHistory::preventNeighborSleep(void)")
}

// 0x71908c — __ZN3RBX15AssemblyHistory6wakeUpEv
// type: _DWORD __fastcall(RBX::AssemblyHistory *__hidden this)
#[doc(alias = "RBX::AssemblyHistory::wakeUp(void)")]
#[doc(alias = "__ZN3RBX15AssemblyHistory6wakeUpEv")]
pub fn stub_71908c() -> ! {
    todo!("0x71908c RBX::AssemblyHistory::wakeUp(void)")
}

// 0x719094 — __ZN3RBX7AverageINS_12PhysicsCoordEE6sampleES1_b
#[doc(alias = "RBX::Average<RBX::PhysicsCoord>::sample(RBX::PhysicsCoord,bool)")]
#[doc(alias = "__ZN3RBX7AverageINS_12PhysicsCoordEE6sampleES1_b")]
pub fn stub_719094() -> ! {
    todo!("0x719094 RBX::Average<RBX::PhysicsCoord>::sample(RBX::PhysicsCoord,bool)")
}

// 0x7190dc — __ZNK3RBX7AverageINS_12PhysicsCoordEE10getAverageEv
#[doc(alias = "RBX::Average<RBX::PhysicsCoord>::getAverage(void)const")]
#[doc(alias = "__ZNK3RBX7AverageINS_12PhysicsCoordEE10getAverageEv")]
pub fn stub_7190dc() -> ! {
    todo!("0x7190dc RBX::Average<RBX::PhysicsCoord>::getAverage(void)const")
}

// 0x7191d0 — __ZN3RBX7AverageINS_12PhysicsCoordEEC2EmS1_
#[doc(alias = "RBX::Average<RBX::PhysicsCoord>::Average(unsigned long,RBX::PhysicsCoord)")]
#[doc(alias = "__ZN3RBX7AverageINS_12PhysicsCoordEEC2EmS1_")]
pub fn stub_7191d0() -> ! {
    todo!("0x7191d0 RBX::Average<RBX::PhysicsCoord>::Average(unsigned long,RBX::PhysicsCoord)")
}

// 0x7192a8 — __ZNSt6vectorIN3RBX12PhysicsCoordESaIS1_EE6resizeEmS1_
#[doc(alias = "std::vector<RBX::PhysicsCoord,std::allocator<RBX::PhysicsCoord>>::resize(unsigned long,RBX::PhysicsCoord)")]
#[doc(alias = "__ZNSt6vectorIN3RBX12PhysicsCoordESaIS1_EE6resizeEmS1_")]
pub fn stub_7192a8() -> ! {
    todo!("0x7192a8 std::vector<RBX::PhysicsCoord,std::allocator<RBX::PhysicsCoord>>::resize(unsigned long,RBX::PhysicsCoord)")
}

// 0x7192f4 — __ZNSt6vectorIN3RBX12PhysicsCoordESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
#[doc(alias = "std::vector<RBX::PhysicsCoord,std::allocator<RBX::PhysicsCoord>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::PhysicsCoord*,std::vector<RBX::PhysicsCoord,std::allocator<RBX::PhysicsCoord>>>,unsigned long,RBX::PhysicsCoord const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX12PhysicsCoordESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_")]
pub fn stub_7192f4() -> ! {
    todo!("0x7192f4 std::vector<RBX::PhysicsCoord,std::allocator<RBX::PhysicsCoord>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::PhysicsCoord*,std::vector<RBX::PhysicsCoord,std::allocator<RBX::PhysicsCoord>>>,unsigned long,RBX::PhysicsCoord const&)")
}

// 0x7195c4 — __ZSt4fillIPN3RBX12PhysicsCoordES1_EvT_S3_RKT0_
#[doc(alias = "void std::fill<RBX::PhysicsCoord *,RBX::PhysicsCoord>(RBX::PhysicsCoord *,RBX::PhysicsCoord *,RBX::PhysicsCoord const&)")]
#[doc(alias = "__ZSt4fillIPN3RBX12PhysicsCoordES1_EvT_S3_RKT0_")]
pub fn stub_7195c4() -> ! {
    todo!("0x7195c4 void std::fill<RBX::PhysicsCoord *,RBX::PhysicsCoord>(RBX::PhysicsCoord *,RBX::PhysicsCoord *,RBX::PhysicsCoord const&)")
}

// 0x7195fc — __ZNSt12_Vector_baseIN3RBX12PhysicsCoordESaIS1_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::PhysicsCoord,std::allocator<RBX::PhysicsCoord>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX12PhysicsCoordESaIS1_EE11_M_allocateEm")]
pub fn stub_7195fc() -> ! {
    todo!("0x7195fc std::_Vector_base<RBX::PhysicsCoord,std::allocator<RBX::PhysicsCoord>>::_M_allocate(unsigned long)")
}

// 0x719620 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX12PhysicsCoordES5_EET0_T_S7_S6_
#[doc(alias = "RBX::PhysicsCoord * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::PhysicsCoord *,RBX::PhysicsCoord *>(RBX::PhysicsCoord *,RBX::PhysicsCoord *,RBX::PhysicsCoord *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX12PhysicsCoordES5_EET0_T_S7_S6_")]
pub fn stub_719620() -> ! {
    todo!("0x719620 RBX::PhysicsCoord * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::PhysicsCoord *,RBX::PhysicsCoord *>(RBX::PhysicsCoord *,RBX::PhysicsCoord *,RBX::PhysicsCoord *)")
}

// 0x71968c — __GLOBAL__I_a_303
#[doc(alias = "global constructor keyed to_a_303")]
#[doc(alias = "__GLOBAL__I_a_303")]
pub fn stub_71968c() -> ! {
    todo!("0x71968c global constructor keyed to_a_303")
}

// 0x719788 — __ZN3RBX13AssemblyStageC1EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::AssemblyStage *__hidden this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::AssemblyStage::AssemblyStage(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX13AssemblyStageC1EPNS_6IStageEPNS_5WorldE")]
pub fn stub_719788() -> ! {
    todo!("0x719788 RBX::AssemblyStage::AssemblyStage(RBX::IStage *,RBX::World *)")
}

// 0x71978c — __ZN3RBX13AssemblyStageC2EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::AssemblyStage *__hidden this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::AssemblyStage::AssemblyStage(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX13AssemblyStageC2EPNS_6IStageEPNS_5WorldE")]
pub fn stub_71978c() -> ! {
    todo!("0x71978c RBX::AssemblyStage::AssemblyStage(RBX::IStage *,RBX::World *)")
}

// 0x719874 — __ZN3RBX13AssemblyStageD0Ev
// type: void __fastcall(RBX::AssemblyStage *__hidden this)
#[doc(alias = "RBX::AssemblyStage::~AssemblyStage()")]
#[doc(alias = "__ZN3RBX13AssemblyStageD0Ev")]
pub fn stub_719874() -> ! {
    todo!("0x719874 RBX::AssemblyStage::~AssemblyStage()")
}

// 0x719914 — __ZN3RBX13AssemblyStageD1Ev
// type: void __fastcall(RBX::AssemblyStage *__hidden this)
#[doc(alias = "RBX::AssemblyStage::~AssemblyStage()")]
#[doc(alias = "__ZN3RBX13AssemblyStageD1Ev")]
pub fn stub_719914() -> ! {
    todo!("0x719914 RBX::AssemblyStage::~AssemblyStage()")
}

// 0x719918 — __ZN3RBX13AssemblyStage16onEngineChangingEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::AssemblyStage *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::AssemblyStage::onEngineChanging(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX13AssemblyStage16onEngineChangingEPNS_9PrimitiveE")]
pub fn stub_719918() -> ! {
    todo!("0x719918 RBX::AssemblyStage::onEngineChanging(RBX::Primitive *)")
}

// 0x71995c — __ZN3RBX13AssemblyStage36onSimulateAssemblyDescendentRemovingEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::AssemblyStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::AssemblyStage::onSimulateAssemblyDescendentRemoving(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX13AssemblyStage36onSimulateAssemblyDescendentRemovingEPNS_8AssemblyE")]
pub fn stub_71995c() -> ! {
    todo!("0x71995c RBX::AssemblyStage::onSimulateAssemblyDescendentRemoving(RBX::Assembly *)")
}

// 0x71997c — __ZN3RBX13AssemblyStage15onEngineChangedEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::AssemblyStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::AssemblyStage::onEngineChanged(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX13AssemblyStage15onEngineChangedEPNS_8AssemblyE")]
pub fn stub_71997c() -> ! {
    todo!("0x71997c RBX::AssemblyStage::onEngineChanged(RBX::Assembly *)")
}

// 0x7199dc — __ZN3RBX13AssemblyStage33onSimulateAssemblyDescendentAddedEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::AssemblyStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::AssemblyStage::onSimulateAssemblyDescendentAdded(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX13AssemblyStage33onSimulateAssemblyDescendentAddedEPNS_8AssemblyE")]
pub fn stub_7199dc() -> ! {
    todo!("0x7199dc RBX::AssemblyStage::onSimulateAssemblyDescendentAdded(RBX::Assembly *)")
}

// 0x719a00 — __ZN3RBX13AssemblyStage27onSimulateAssemblyRootAddedEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::AssemblyStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::AssemblyStage::onSimulateAssemblyRootAdded(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX13AssemblyStage27onSimulateAssemblyRootAddedEPNS_8AssemblyE")]
pub fn stub_719a00() -> ! {
    todo!("0x719a00 RBX::AssemblyStage::onSimulateAssemblyRootAdded(RBX::Assembly *)")
}

// 0x719a24 — __ZN3RBX13AssemblyStage30onSimulateAssemblyRootRemovingEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::AssemblyStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::AssemblyStage::onSimulateAssemblyRootRemoving(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX13AssemblyStage30onSimulateAssemblyRootRemovingEPNS_8AssemblyE")]
pub fn stub_719a24() -> ! {
    todo!("0x719a24 RBX::AssemblyStage::onSimulateAssemblyRootRemoving(RBX::Assembly *)")
}

// 0x719a44 — __ZN3RBX13AssemblyStage24onFixedAssemblyRootAddedEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::AssemblyStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::AssemblyStage::onFixedAssemblyRootAdded(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX13AssemblyStage24onFixedAssemblyRootAddedEPNS_8AssemblyE")]
pub fn stub_719a44() -> ! {
    todo!("0x719a44 RBX::AssemblyStage::onFixedAssemblyRootAdded(RBX::Assembly *)")
}

// 0x719a60 — __ZN3RBX13AssemblyStage27onFixedAssemblyRootRemovingEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::AssemblyStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::AssemblyStage::onFixedAssemblyRootRemoving(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX13AssemblyStage27onFixedAssemblyRootRemovingEPNS_8AssemblyE")]
pub fn stub_719a60() -> ! {
    todo!("0x719a60 RBX::AssemblyStage::onFixedAssemblyRootRemoving(RBX::Assembly *)")
}

// 0x719a78 — __ZN3RBX13AssemblyStage35onNoSimulateAssemblyDescendentAddedEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::AssemblyStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::AssemblyStage::onNoSimulateAssemblyDescendentAdded(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX13AssemblyStage35onNoSimulateAssemblyDescendentAddedEPNS_8AssemblyE")]
pub fn stub_719a78() -> ! {
    todo!("0x719a78 RBX::AssemblyStage::onNoSimulateAssemblyDescendentAdded(RBX::Assembly *)")
}

// 0x719a94 — __ZN3RBX13AssemblyStage38onNoSimulateAssemblyDescendentRemovingEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::AssemblyStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::AssemblyStage::onNoSimulateAssemblyDescendentRemoving(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX13AssemblyStage38onNoSimulateAssemblyDescendentRemovingEPNS_8AssemblyE")]
pub fn stub_719a94() -> ! {
    todo!("0x719a94 RBX::AssemblyStage::onNoSimulateAssemblyDescendentRemoving(RBX::Assembly *)")
}

// 0x719aac — __ZNK3RBX13AssemblyStage12getStageTypeEv
// type: _DWORD __fastcall(RBX::AssemblyStage *__hidden this)
#[doc(alias = "RBX::AssemblyStage::getStageType(void)const")]
#[doc(alias = "__ZNK3RBX13AssemblyStage12getStageTypeEv")]
pub fn stub_719aac() -> ! {
    todo!("0x719aac RBX::AssemblyStage::getStageType(void)const")
}

// 0x719ab0 — __GLOBAL__I_a_304
#[doc(alias = "global constructor keyed to_a_304")]
#[doc(alias = "__GLOBAL__I_a_304")]
pub fn stub_719ab0() -> ! {
    todo!("0x719ab0 global constructor keyed to_a_304")
}

// 0x719be0 — __ZNK3RBX4Ball14getMomentSolidEf
// type: _DWORD __fastcall(RBX::Ball *__hidden this, float)
#[doc(alias = "RBX::Ball::getMomentSolid(float)const")]
#[doc(alias = "__ZNK3RBX4Ball14getMomentSolidEf")]
pub fn stub_719be0() -> ! {
    todo!("0x719be0 RBX::Ball::getMomentSolid(float)const")
}

// 0x719c28 — __ZNK3RBX4Ball9getVolumeEv
// type: _DWORD __fastcall(RBX::Ball *__hidden this)
#[doc(alias = "RBX::Ball::getVolume(void)const")]
#[doc(alias = "__ZNK3RBX4Ball9getVolumeEv")]
pub fn stub_719c28() -> ! {
    todo!("0x719c28 RBX::Ball::getVolume(void)const")
}

// 0x719e7c — __ZNK3RBX4Ball19getPlaneFromSurfaceEm
// type: _DWORD __fastcall(RBX::Ball *__hidden this, unsigned int)
#[doc(alias = "RBX::Ball::getPlaneFromSurface(unsigned long)const")]
#[doc(alias = "__ZNK3RBX4Ball19getPlaneFromSurfaceEm")]
pub fn stub_719e7c() -> ! {
    todo!("0x719e7c RBX::Ball::getPlaneFromSurface(unsigned long)const")
}

// 0x719f5c — __ZNK3RBX4Ball22getSurfaceNormalInBodyEm
// type: _DWORD __fastcall(RBX::Ball *__hidden this, unsigned int)
#[doc(alias = "RBX::Ball::getSurfaceNormalInBody(unsigned long)const")]
#[doc(alias = "__ZNK3RBX4Ball22getSurfaceNormalInBodyEm")]
pub fn stub_719f5c() -> ! {
    todo!("0x719f5c RBX::Ball::getSurfaceNormalInBody(unsigned long)const")
}

// 0x719fb4 — __ZNK3RBX4Ball20getSurfaceVertInBodyEmi
// type: _DWORD __fastcall(RBX::Ball *__hidden this, unsigned int, int)
#[doc(alias = "RBX::Ball::getSurfaceVertInBody(unsigned long,int)const")]
#[doc(alias = "__ZNK3RBX4Ball20getSurfaceVertInBodyEmi")]
pub fn stub_719fb4() -> ! {
    todo!("0x719fb4 RBX::Ball::getSurfaceVertInBody(unsigned long,int)const")
}

// 0x71a194 — __ZNK3RBX4Ball20getNumVertsInSurfaceEm
// type: _DWORD __fastcall(RBX::Ball *__hidden this, unsigned int)
#[doc(alias = "RBX::Ball::getNumVertsInSurface(unsigned long)const")]
#[doc(alias = "__ZNK3RBX4Ball20getNumVertsInSurfaceEm")]
pub fn stub_71a194() -> ! {
    todo!("0x71a194 RBX::Ball::getNumVertsInSurface(unsigned long)const")
}

// 0x71a230 — __ZNK3RBX4Ball21getSurfaceCoordInBodyEm
// type: _DWORD __fastcall(RBX::Ball *__hidden this, unsigned int)
#[doc(alias = "RBX::Ball::getSurfaceCoordInBody(unsigned long)const")]
#[doc(alias = "__ZNK3RBX4Ball21getSurfaceCoordInBodyEm")]
pub fn stub_71a230() -> ! {
    todo!("0x71a230 RBX::Ball::getSurfaceCoordInBody(unsigned long)const")
}

// 0x71a2fc — __GLOBAL__I_a_305
#[doc(alias = "global constructor keyed to_a_305")]
#[doc(alias = "__GLOBAL__I_a_305")]
pub fn stub_71a2fc() -> ! {
    todo!("0x71a2fc global constructor keyed to_a_305")
}

// 0x71a334 — __ZN3RBX15BallPolyContactC1EPNS_9PrimitiveES2_
// type: _DWORD __fastcall(RBX::BallPolyContact *__hidden this, RBX::Primitive *, RBX::Primitive *)
#[doc(alias = "RBX::BallPolyContact::BallPolyContact(RBX::Primitive *,RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX15BallPolyContactC1EPNS_9PrimitiveES2_")]
pub fn stub_71a334() -> ! {
    todo!("0x71a334 RBX::BallPolyContact::BallPolyContact(RBX::Primitive *,RBX::Primitive *)")
}

// 0x71a338 — __ZN3RBX15BallPolyContactC2EPNS_9PrimitiveES2_
// type: _DWORD __fastcall(RBX::BallPolyContact *__hidden this, RBX::Primitive *, RBX::Primitive *)
#[doc(alias = "RBX::BallPolyContact::BallPolyContact(RBX::Primitive *,RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX15BallPolyContactC2EPNS_9PrimitiveES2_")]
pub fn stub_71a338() -> ! {
    todo!("0x71a338 RBX::BallPolyContact::BallPolyContact(RBX::Primitive *,RBX::Primitive *)")
}

// 0x71a4d0 — __ZN3RBX15BallPolyContact19findClosestFeaturesERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE
#[doc(alias = "RBX::BallPolyContact::findClosestFeatures(RBX::FixedArray<RBX::PolyConnector *,40ul> &)")]
#[doc(alias = "__ZN3RBX15BallPolyContact19findClosestFeaturesERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE")]
pub fn stub_71a4d0() -> ! {
    todo!("0x71a4d0 RBX::BallPolyContact::findClosestFeatures(RBX::FixedArray<RBX::PolyConnector *,40ul> &)")
}

// 0x71a7b0 — __ZN3RBX15BallPolyContact21newBallPlaneConnectorEPKNS_4POLY4FaceE
// type: _DWORD __fastcall(RBX::BallPolyContact *__hidden this, const RBX::POLY::Face *)
#[doc(alias = "RBX::BallPolyContact::newBallPlaneConnector(RBX::POLY::Face const*)")]
#[doc(alias = "__ZN3RBX15BallPolyContact21newBallPlaneConnectorEPKNS_4POLY4FaceE")]
pub fn stub_71a7b0() -> ! {
    todo!("0x71a7b0 RBX::BallPolyContact::newBallPlaneConnector(RBX::POLY::Face const*)")
}

// 0x71aa04 — __ZN3RBX15BallPolyContact20newBallEdgeConnectorEPKNS_4POLY4EdgeE
// type: _DWORD __fastcall(RBX::BallPolyContact *__hidden this, const RBX::POLY::Edge *)
#[doc(alias = "RBX::BallPolyContact::newBallEdgeConnector(RBX::POLY::Edge const*)")]
#[doc(alias = "__ZN3RBX15BallPolyContact20newBallEdgeConnectorEPKNS_4POLY4EdgeE")]
pub fn stub_71aa04() -> ! {
    todo!("0x71aa04 RBX::BallPolyContact::newBallEdgeConnector(RBX::POLY::Edge const*)")
}

// 0x71ad7c — __ZN3RBX15BallPolyContact22newBallVertexConnectorEPKNS_4POLY6VertexE
// type: int __fastcall(int, int, int, int, boost::mutex *, int, int, int, int, int)
#[doc(alias = "RBX::BallPolyContact::newBallVertexConnector(RBX::POLY::Vertex const*)")]
#[doc(alias = "__ZN3RBX15BallPolyContact22newBallVertexConnectorEPKNS_4POLY6VertexE")]
pub fn stub_71ad7c() -> ! {
    todo!("0x71ad7c RBX::BallPolyContact::newBallVertexConnector(RBX::POLY::Vertex const*)")
}

// 0x71af10 — __ZN3RBX15BallPolyContact34generateDataForMovingAssemblyStageEv
// type: _DWORD __fastcall(RBX::BallPolyContact *__hidden this)
#[doc(alias = "RBX::BallPolyContact::generateDataForMovingAssemblyStage(void)")]
#[doc(alias = "__ZN3RBX15BallPolyContact34generateDataForMovingAssemblyStageEv")]
pub fn stub_71af10() -> ! {
    todo!("0x71af10 RBX::BallPolyContact::generateDataForMovingAssemblyStage(void)")
}

// 0x71af14 — __ZN3RBX9AllocatorINS_15BallPolyContactEEC2Ev
#[doc(alias = "RBX::Allocator<RBX::BallPolyContact>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_15BallPolyContactEEC2Ev")]
pub fn stub_71af14() -> ! {
    todo!("0x71af14 RBX::Allocator<RBX::BallPolyContact>::Allocator(void)")
}

// 0x71af78 — __ZN3RBX15BallPolyContactD1Ev
// type: void __fastcall(RBX::BallPolyContact *__hidden this)
#[doc(alias = "RBX::BallPolyContact::~BallPolyContact()")]
#[doc(alias = "__ZN3RBX15BallPolyContactD1Ev")]
pub fn stub_71af78() -> ! {
    todo!("0x71af78 RBX::BallPolyContact::~BallPolyContact()")
}

// 0x71af7c — __ZN3RBX15BallPolyContactD0Ev
// type: void __fastcall(RBX::BallPolyContact *__hidden this)
#[doc(alias = "RBX::BallPolyContact::~BallPolyContact()")]
#[doc(alias = "__ZN3RBX15BallPolyContactD0Ev")]
pub fn stub_71af7c() -> ! {
    todo!("0x71af7c RBX::BallPolyContact::~BallPolyContact()")
}

// 0x71b030 — __ZNK3RBX11PolyContact13numConnectorsEv
// type: _DWORD __fastcall(RBX::PolyContact *__hidden this)
#[doc(alias = "RBX::PolyContact::numConnectors(void)const")]
#[doc(alias = "__ZNK3RBX11PolyContact13numConnectorsEv")]
pub fn stub_71b030() -> ! {
    todo!("0x71b030 RBX::PolyContact::numConnectors(void)const")
}

// 0x71b038 — __ZN3RBX9AllocatorINS_15BallPolyContactEE13releaseMemoryEv
#[doc(alias = "RBX::Allocator<RBX::BallPolyContact>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_15BallPolyContactEE13releaseMemoryEv")]
pub fn stub_71b038() -> ! {
    todo!("0x71b038 RBX::Allocator<RBX::BallPolyContact>::releaseMemory(void)")
}

// 0x71b054 — __ZN5boost14singleton_poolIN3RBX15BallPolyContactELj212ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
#[doc(alias = "boost::singleton_pool<RBX::BallPolyContact,212u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX15BallPolyContactELj212ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
pub fn stub_71b054() -> ! {
    todo!("0x71b054 boost::singleton_pool<RBX::BallPolyContact,212u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")
}

// 0x71b084 — __ZN3RBX9AllocatorINS_15BallPolyContactEEdlEPv
// type: void __fastcall(void *)
#[doc(alias = "RBX::Allocator<RBX::BallPolyContact>::operator delete(void *)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_15BallPolyContactEEdlEPv")]
pub fn stub_71b084() -> ! {
    todo!("0x71b084 RBX::Allocator<RBX::BallPolyContact>::operator delete(void *)")
}

// 0x71b0c0 — __GLOBAL__I_a_306
#[doc(alias = "global constructor keyed to_a_306")]
#[doc(alias = "__GLOBAL__I_a_306")]
pub fn stub_71b0c0() -> ! {
    todo!("0x71b0c0 global constructor keyed to_a_306")
}

// 0x71b460 — __ZN3RBX5Block4initEv
// type: _DWORD __fastcall(RBX::Block *__hidden this)
#[doc(alias = "RBX::Block::init(void)")]
#[doc(alias = "__ZN3RBX5Block4initEv")]
pub fn stub_71b460() -> ! {
    todo!("0x71b460 RBX::Block::init(void)")
}

// 0x71b4a8 — __ZN3RBX5Block9buildMeshEv
// type: _DWORD __fastcall(RBX::Block *__hidden this)
#[doc(alias = "RBX::Block::buildMesh(void)")]
#[doc(alias = "__ZN3RBX5Block9buildMeshEv")]
pub fn stub_71b4a8() -> ! {
    todo!("0x71b4a8 RBX::Block::buildMesh(void)")
}

// 0x71b72c — __ZNK3RBX5Block15getMomentHollowEf
// type: _DWORD __fastcall(RBX::Block *__hidden this, float)
#[doc(alias = "RBX::Block::getMomentHollow(float)const")]
#[doc(alias = "__ZNK3RBX5Block15getMomentHollowEf")]
pub fn stub_71b72c() -> ! {
    todo!("0x71b72c RBX::Block::getMomentHollow(float)const")
}

// 0x71bb08 — __ZNK3RBX5Block9getVolumeEv
// type: _DWORD __fastcall(RBX::Block *__hidden this)
#[doc(alias = "RBX::Block::getVolume(void)const")]
#[doc(alias = "__ZNK3RBX5Block9getVolumeEv")]
pub fn stub_71bb08() -> ! {
    todo!("0x71bb08 RBX::Block::getVolume(void)const")
}

// 0x71c050 — __ZNK3RBX5Block21getSurfaceCoordInBodyEm
// type: _DWORD __fastcall(RBX::Block *__hidden this, unsigned int)
#[doc(alias = "RBX::Block::getSurfaceCoordInBody(unsigned long)const")]
#[doc(alias = "__ZNK3RBX5Block21getSurfaceCoordInBodyEm")]
pub fn stub_71c050() -> ! {
    todo!("0x71c050 RBX::Block::getSurfaceCoordInBody(unsigned long)const")
}

// 0x71c3f0 — __ZN3RBX5BlockD1Ev
// type: void __fastcall(RBX::Block *__hidden this)
#[doc(alias = "RBX::Block::~Block()")]
#[doc(alias = "__ZN3RBX5BlockD1Ev")]
pub fn stub_71c3f0() -> ! {
    todo!("0x71c3f0 RBX::Block::~Block()")
}

// 0x71c3f4 — __ZN3RBX5BlockD0Ev
// type: void __fastcall(RBX::Block *__hidden this)
#[doc(alias = "RBX::Block::~Block()")]
#[doc(alias = "__ZN3RBX5BlockD0Ev")]
pub fn stub_71c3f4() -> ! {
    todo!("0x71c3f4 RBX::Block::~Block()")
}

// 0x71c494 — __ZNK3RBX5Block15getGeometryTypeEv
// type: _DWORD __fastcall(RBX::Block *__hidden this)
#[doc(alias = "RBX::Block::getGeometryType(void)const")]
#[doc(alias = "__ZNK3RBX5Block15getGeometryTypeEv")]
pub fn stub_71c494() -> ! {
    todo!("0x71c494 RBX::Block::getGeometryType(void)const")
}

// 0x71c498 — __ZNK3RBX5Block14getCollideTypeEv
// type: _DWORD __fastcall(RBX::Block *__hidden this)
#[doc(alias = "RBX::Block::getCollideType(void)const")]
#[doc(alias = "__ZNK3RBX5Block14getCollideTypeEv")]
pub fn stub_71c498() -> ! {
    todo!("0x71c498 RBX::Block::getCollideType(void)const")
}

// 0x71c49c — __ZN3RBX8Geometry20setGeometryParameterERKSsi
// type: _DWORD __fastcall(RBX::Geometry *__hidden this, const std::string *, int)
#[doc(alias = "RBX::Geometry::setGeometryParameter(std::string const&,int)")]
#[doc(alias = "__ZN3RBX8Geometry20setGeometryParameterERKSsi")]
pub fn stub_71c49c() -> ! {
    todo!("0x71c49c RBX::Geometry::setGeometryParameter(std::string const&,int)")
}

// 0x71c4f4 — __ZNK3RBX8Geometry20getGeometryParameterERKSs
// type: _DWORD __fastcall(RBX::Geometry *__hidden this, const std::string *)
#[doc(alias = "RBX::Geometry::getGeometryParameter(std::string const&)const")]
#[doc(alias = "__ZNK3RBX8Geometry20getGeometryParameterERKSs")]
pub fn stub_71c4f4() -> ! {
    todo!("0x71c4f4 RBX::Geometry::getGeometryParameter(std::string const&)const")
}

// 0x71c548 — __ZNK3RBX4Poly9getRadiusEv
// type: _DWORD __fastcall(RBX::Poly *__hidden this)
#[doc(alias = "RBX::Poly::getRadius(void)const")]
#[doc(alias = "__ZNK3RBX4Poly9getRadiusEv")]
pub fn stub_71c548() -> ! {
    todo!("0x71c548 RBX::Poly::getRadius(void)const")
}

// 0x71c54c — __ZNK3RBX4Poly14getNumSurfacesEv
// type: _DWORD __fastcall(RBX::Poly *__hidden this)
#[doc(alias = "RBX::Poly::getNumSurfaces(void)const")]
#[doc(alias = "__ZNK3RBX4Poly14getNumSurfacesEv")]
pub fn stub_71c54c() -> ! {
    todo!("0x71c54c RBX::Poly::getNumSurfaces(void)const")
}

// 0x71c564 — __ZNK3RBX8Geometry25getFaceFromLegacyNormalIdENS_8NormalIdE
#[doc(alias = "RBX::Geometry::getFaceFromLegacyNormalId(RBX::NormalId)const")]
#[doc(alias = "__ZNK3RBX8Geometry25getFaceFromLegacyNormalIdENS_8NormalIdE")]
pub fn stub_71c564() -> ! {
    todo!("0x71c564 RBX::Geometry::getFaceFromLegacyNormalId(RBX::NormalId)const")
}

// 0x71c568 — __ZNK3RBX8Geometry20isGeometryOrthogonalEv
// type: _DWORD __fastcall(RBX::Geometry *__hidden this)
#[doc(alias = "RBX::Geometry::isGeometryOrthogonal(void)const")]
#[doc(alias = "__ZNK3RBX8Geometry20isGeometryOrthogonalEv")]
pub fn stub_71c568() -> ! {
    todo!("0x71c568 RBX::Geometry::isGeometryOrthogonal(void)const")
}

// 0x71c56c — __ZNK3RBX5Block9getMomentEf
// type: _DWORD __fastcall(RBX::Block *__hidden this, float)
#[doc(alias = "RBX::Block::getMoment(float)const")]
#[doc(alias = "__ZNK3RBX5Block9getMomentEf")]
pub fn stub_71c56c() -> ! {
    todo!("0x71c56c RBX::Block::getMoment(float)const")
}

// 0x71ca14 — __ZN3RBX9AllocatorINS_4POLY12BlockCornersEEdlEPv
// type: void __fastcall(void *)
#[doc(alias = "RBX::Allocator<RBX::POLY::BlockCorners>::operator delete(void *)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY12BlockCornersEEdlEPv")]
pub fn stub_71ca14() -> ! {
    todo!("0x71ca14 RBX::Allocator<RBX::POLY::BlockCorners>::operator delete(void *)")
}

// 0x71d050 — __ZN3RBX9AllocatorINS_4POLY12BlockCornersEEnwEm
#[doc(alias = "RBX::Allocator<RBX::POLY::BlockCorners>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY12BlockCornersEEnwEm")]
pub fn stub_71d050() -> ! {
    todo!("0x71d050 RBX::Allocator<RBX::POLY::BlockCorners>::operator new(unsigned long)")
}

// 0x71d14c — __ZN3RBX9AllocatorINS_4POLY12BlockCornersEEC2Ev
#[doc(alias = "RBX::Allocator<RBX::POLY::BlockCorners>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY12BlockCornersEEC2Ev")]
pub fn stub_71d14c() -> ! {
    todo!("0x71d14c RBX::Allocator<RBX::POLY::BlockCorners>::Allocator(void)")
}

// 0x71d1b0 — __ZN3RBX9AllocatorINS_4POLY12BlockCornersEE13releaseMemoryEv
#[doc(alias = "RBX::Allocator<RBX::POLY::BlockCorners>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY12BlockCornersEE13releaseMemoryEv")]
pub fn stub_71d1b0() -> ! {
    todo!("0x71d1b0 RBX::Allocator<RBX::POLY::BlockCorners>::releaseMemory(void)")
}

// 0x71d1cc — __ZN5boost14singleton_poolIN3RBX4POLY12BlockCornersELj96ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
#[doc(alias = "boost::singleton_pool<RBX::POLY::BlockCorners,96u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX4POLY12BlockCornersELj96ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
pub fn stub_71d1cc() -> ! {
    todo!("0x71d1cc boost::singleton_pool<RBX::POLY::BlockCorners,96u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")
}

// 0x71d1fc — __ZN5boost14singleton_poolIN3RBX4POLY12BlockCornersELj96ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
#[doc(alias = "boost::singleton_pool<RBX::POLY::BlockCorners,96u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX4POLY12BlockCornersELj96ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")]
pub fn stub_71d1fc() -> ! {
    todo!("0x71d1fc boost::singleton_pool<RBX::POLY::BlockCorners,96u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")
}

// 0x71da2c — __ZN3RBX9AllocatorINS_4POLY9BlockMeshEEdlEPv
// type: void __fastcall(void *)
#[doc(alias = "RBX::Allocator<RBX::POLY::BlockMesh>::operator delete(void *)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY9BlockMeshEEdlEPv")]
pub fn stub_71da2c() -> ! {
    todo!("0x71da2c RBX::Allocator<RBX::POLY::BlockMesh>::operator delete(void *)")
}

// 0x71da68 — __ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EED2Ev
#[doc(alias = "std::vector<RBX::POLY::Vertex,std::allocator<RBX::POLY::Vertex>>::~vector()")]
#[doc(alias = "__ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EED2Ev")]
pub fn stub_71da68() -> ! {
    todo!("0x71da68 std::vector<RBX::POLY::Vertex,std::allocator<RBX::POLY::Vertex>>::~vector()")
}

// 0x71daa0 — __ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EED2Ev
#[doc(alias = "std::vector<RBX::POLY::Face,std::allocator<RBX::POLY::Face>>::~vector()")]
#[doc(alias = "__ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EED2Ev")]
pub fn stub_71daa0() -> ! {
    todo!("0x71daa0 std::vector<RBX::POLY::Face,std::allocator<RBX::POLY::Face>>::~vector()")
}

// 0x71e0b0 — __ZN3RBX9AllocatorINS_4POLY9BlockMeshEEnwEm
#[doc(alias = "RBX::Allocator<RBX::POLY::BlockMesh>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY9BlockMeshEEnwEm")]
pub fn stub_71e0b0() -> ! {
    todo!("0x71e0b0 RBX::Allocator<RBX::POLY::BlockMesh>::operator new(unsigned long)")
}

// 0x71e230 — __ZN3RBX9AllocatorINS_4POLY9BlockMeshEEC2Ev
#[doc(alias = "RBX::Allocator<RBX::POLY::BlockMesh>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY9BlockMeshEEC2Ev")]
pub fn stub_71e230() -> ! {
    todo!("0x71e230 RBX::Allocator<RBX::POLY::BlockMesh>::Allocator(void)")
}

// 0x71e294 — __ZN3RBX9AllocatorINS_4POLY9BlockMeshEE13releaseMemoryEv
#[doc(alias = "RBX::Allocator<RBX::POLY::BlockMesh>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY9BlockMeshEE13releaseMemoryEv")]
pub fn stub_71e294() -> ! {
    todo!("0x71e294 RBX::Allocator<RBX::POLY::BlockMesh>::releaseMemory(void)")
}

// 0x71e2b0 — __ZN5boost14singleton_poolIN3RBX4POLY9BlockMeshELj36ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
#[doc(alias = "boost::singleton_pool<RBX::POLY::BlockMesh,36u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX4POLY9BlockMeshELj36ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
pub fn stub_71e2b0() -> ! {
    todo!("0x71e2b0 boost::singleton_pool<RBX::POLY::BlockMesh,36u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")
}

// 0x71e2e0 — __ZN5boost14singleton_poolIN3RBX4POLY9BlockMeshELj36ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
#[doc(alias = "boost::singleton_pool<RBX::POLY::BlockMesh,36u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX4POLY9BlockMeshELj36ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")]
pub fn stub_71e2e0() -> ! {
    todo!("0x71e2e0 boost::singleton_pool<RBX::POLY::BlockMesh,36u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")
}

// 0x71e3bc — __ZN3RBX5BlockD2Ev
// type: void __fastcall(RBX::Block *__hidden this)
#[doc(alias = "RBX::Block::~Block()")]
#[doc(alias = "__ZN3RBX5BlockD2Ev")]
pub fn stub_71e3bc() -> ! {
    todo!("0x71e3bc RBX::Block::~Block()")
}

// 0x71e49c — __GLOBAL__I_a_307
#[doc(alias = "global constructor keyed to_a_307")]
#[doc(alias = "__GLOBAL__I_a_307")]
pub fn stub_71e49c() -> ! {
    todo!("0x71e49c global constructor keyed to_a_307")
}

// 0x71e5cc — __ZN3RBX10CleanStageC1EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::CleanStage *__hidden this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::CleanStage::CleanStage(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX10CleanStageC1EPNS_6IStageEPNS_5WorldE")]
pub fn stub_71e5cc() -> ! {
    todo!("0x71e5cc RBX::CleanStage::CleanStage(RBX::IStage *,RBX::World *)")
}

// 0x71e5d0 — __ZN3RBX10CleanStageC2EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::CleanStage *__hidden this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::CleanStage::CleanStage(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX10CleanStageC2EPNS_6IStageEPNS_5WorldE")]
pub fn stub_71e5d0() -> ! {
    todo!("0x71e5d0 RBX::CleanStage::CleanStage(RBX::IStage *,RBX::World *)")
}

// 0x71e6a4 — __ZN3RBX10CleanStage16onPrimitiveAddedEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::CleanStage *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::CleanStage::onPrimitiveAdded(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX10CleanStage16onPrimitiveAddedEPNS_9PrimitiveE")]
pub fn stub_71e6a4() -> ! {
    todo!("0x71e6a4 RBX::CleanStage::onPrimitiveAdded(RBX::Primitive *)")
}

// 0x71e6c0 — __ZN3RBX10CleanStage19onPrimitiveRemovingEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::CleanStage *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::CleanStage::onPrimitiveRemoving(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX10CleanStage19onPrimitiveRemovingEPNS_9PrimitiveE")]
pub fn stub_71e6c0() -> ! {
    todo!("0x71e6c0 RBX::CleanStage::onPrimitiveRemoving(RBX::Primitive *)")
}

// 0x71e6dc — __ZN3RBX10CleanStage23onJointPrimitiveNullingEPNS_5JointEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::CleanStage *__hidden this, RBX::Joint *, RBX::Primitive *)
#[doc(alias = "RBX::CleanStage::onJointPrimitiveNulling(RBX::Joint *,RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX10CleanStage23onJointPrimitiveNullingEPNS_5JointEPNS_9PrimitiveE")]
pub fn stub_71e6dc() -> ! {
    todo!("0x71e6dc RBX::CleanStage::onJointPrimitiveNulling(RBX::Joint *,RBX::Primitive *)")
}

// 0x71e7fc — __ZN3RBX10CleanStage19onJointPrimitiveSetEPNS_5JointEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::CleanStage *__hidden this, RBX::Joint *, RBX::Primitive *)
#[doc(alias = "RBX::CleanStage::onJointPrimitiveSet(RBX::Joint *,RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX10CleanStage19onJointPrimitiveSetEPNS_5JointEPNS_9PrimitiveE")]
pub fn stub_71e7fc() -> ! {
    todo!("0x71e7fc RBX::CleanStage::onJointPrimitiveSet(RBX::Joint *,RBX::Primitive *)")
}

// 0x71e87c — __ZN3RBX10CleanStage11onEdgeAddedEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::CleanStage *__hidden this, RBX::Edge *)
#[doc(alias = "RBX::CleanStage::onEdgeAdded(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX10CleanStage11onEdgeAddedEPNS_4EdgeE")]
pub fn stub_71e87c() -> ! {
    todo!("0x71e87c RBX::CleanStage::onEdgeAdded(RBX::Edge *)")
}
