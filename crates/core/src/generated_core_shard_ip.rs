//! core shard IP — 100 core stubs EA-sorted, 0x2b980..0x248f10 (strict RBX|boost excluding Reflection|Instance|DataModel|Ogre|G3D|Rendering|Adorn|RakNet|Network|Replicat|Socket|Sound|Audio|FMOD|Script|Lua|ViewController|UIApplication|Platform|iOS, EA-sorted ascending, next 100 uncovered after 0x2b980 prior 100 remaining).
//! Source: ida/export.json filtered where demangled NOT containing Reflection|Instance|DataModel|Ogre|G3D|Rendering|Adorn|RakNet|Network|Replicat|Socket|Sound|Audio|FMOD|Script|Lua|lua|ViewController|UIApplication|Platform|iOS but containing RBX:: or boost::, EA-sorted ascending, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "____ZL20finishTeleportHelperP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEE_block_invoke")]
// 0x2b980 — ____ZL20finishTeleportHelperP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEE_block_invoke
// was: ____ZL20finishTeleportHelperP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEE_block_invoke
pub fn stub_0x2b980() -> ! {
    todo!("0x2b980 ____ZL20finishTeleportHelperP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEE_block_invoke")
}

#[doc(alias = "____ZL20finishTeleportHelperP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEE_block_invoke428")]
// 0x2ba14 — ____ZL20finishTeleportHelperP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEE_block_invoke428
// was: ____ZL20finishTeleportHelperP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEE_block_invoke428
pub fn stub_0x2ba14() -> ! {
    todo!("0x2ba14 ____ZL20finishTeleportHelperP10RobloxViewN5boost10shared_ptrIN3RBX4GameEEE_block_invoke428")
}

#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIiEENSC_IS8_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE")]
// 0x2f0f0 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIiEENSC_IS8_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIiEENSC_IS8_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE
pub fn stub_0x2f0f0() -> ! {
    todo!("0x2f0f0 __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIiEENSC_IS8_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS3_5list3INS3_5valueIiEENSD_IS8_EENSD_IS9_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE")]
// 0x2f7d0 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS3_5list3INS3_5valueIiEENSD_IS8_EENSD_IS9_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, struct _Unwind_Exception *lpuexcpt, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
// was: __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS3_5list3INS3_5valueIiEENSD_IS8_EENSD_IS9_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE
pub fn stub_0x2f7d0() -> ! {
    todo!("0x2f7d0 __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS3_5list3INS3_5valueIiEENSD_IS8_EENSD_IS9_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list3INS3_5valueIiEENSE_IPKcEENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
// 0x2ff94 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list3INS3_5valueIiEENSE_IPKcEENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
// type: boost::detail::sp_counted_base *__fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list3INS3_5valueIiEENSE_IPKcEENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
pub fn stub_0x2ff94() -> ! {
    todo!("0x2ff94 __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list3INS3_5valueIiEENSE_IPKcEENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
// 0x3093c — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
pub fn stub_0x3093c() -> ! {
    todo!("0x3093c __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS4_5list2INS4_5valueIS7_EENSF_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")]
// 0x32984 — __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS4_5list2INS4_5valueIS7_EENSF_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS4_5list2INS4_5valueIS7_EENSF_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
pub fn stub_0x32984() -> ! {
    todo!("0x32984 __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS4_5list2INS4_5valueIS7_EENSF_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE")]
// 0x32a68 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE
pub fn stub_0x32a68() -> ! {
    todo!("0x32a68 __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS4_5list4INS4_5valueIS7_EENSB_ISsEESD_SD_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISG_EE5valueEEE5valueEiE4typeE")]
// 0x342f4 — __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS4_5list4INS4_5valueIS7_EENSB_ISsEESD_SD_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISG_EE5valueEEE5valueEiE4typeE
// type: int(void)
// was: __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS4_5list4INS4_5valueIS7_EENSB_ISsEESD_SD_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISG_EE5valueEEE5valueEiE4typeE
pub fn stub_0x342f4() -> ! {
    todo!("0x342f4 __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS4_5list4INS4_5valueIS7_EENSB_ISsEESD_SD_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISG_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISF_EE5valueEEE5valueEiE4typeE")]
// 0x345b0 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISF_EE5valueEEE5valueEiE4typeE
// type: int(void)
// was: __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISF_EE5valueEEE5valueEiE4typeE
pub fn stub_0x345b0() -> ! {
    todo!("0x345b0 __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISF_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "RBX::CEvent::Wait(int)")]
// 0x24381c — __ZN3RBX6CEvent4WaitEi
// type: bool __fastcall(RBX::CEvent *this, int, int)
// was: RBX::CEvent::Wait(int)
pub fn stub_0x24381c() -> ! {
    todo!("0x24381c __ZN3RBX6CEvent4WaitEi")
}

#[doc(alias = "RBX::CEvent::~CEvent()")]
// 0x243830 — __ZN3RBX6CEventD1Ev
// type: void __fastcall(RBX::CEvent *__hidden this)
// was: RBX::CEvent::~CEvent()
pub fn stub_0x243830() -> ! {
    todo!("0x243830 __ZN3RBX6CEventD1Ev")
}

#[doc(alias = "RBX::CEvent::~CEvent()")]
// 0x24383c — __ZN3RBX6CEventD2Ev
// type: void __fastcall(RBX::CEvent *__hidden this)
// was: RBX::CEvent::~CEvent()
pub fn stub_0x24383c() -> ! {
    todo!("0x24383c __ZN3RBX6CEventD2Ev")
}

#[doc(alias = "RBX::CEvent::CEvent(bool)")]
// 0x243944 — __ZN3RBX6CEventC1Eb
// type: RBX::CEvent *__fastcall(RBX::CEvent *this, bool)
// was: RBX::CEvent::CEvent(bool)
pub fn stub_0x243944() -> ! {
    todo!("0x243944 __ZN3RBX6CEventC1Eb")
}

#[doc(alias = "RBX::CEvent::Set(void)")]
// 0x243a30 — __ZN3RBX6CEvent3SetEv
// type: void __fastcall(RBX::CEvent *this)
// was: RBX::CEvent::Set(void)
pub fn stub_0x243a30() -> ! {
    todo!("0x243a30 __ZN3RBX6CEvent3SetEv")
}

#[doc(alias = "boost::condition_variable::do_wait_until(boost::unique_lock<boost::mutex> &,timespec const&)")]
// 0x243b84 — __ZN5boost18condition_variable13do_wait_untilERNS_11unique_lockINS_5mutexEEERK8timespec
// type: int __fastcall(int, int, const timespec *)
// was: boost::condition_variable::do_wait_until(boost::unique_lock<boost::mutex> &,timespec const&)
pub fn stub_0x243b84() -> ! {
    todo!("0x243b84 __ZN5boost18condition_variable13do_wait_untilERNS_11unique_lockINS_5mutexEEERK8timespec")
}

#[doc(alias = "RBX::Limits::Countable::Countable(void)")]
// 0x243e98 — __ZN3RBX6Limits9CountableC2Ev
// type: RBX::Limits::Countable *__fastcall(RBX::Limits::Countable *this, int, int, int)
// was: RBX::Limits::Countable::Countable(void)
pub fn stub_0x243e98() -> ! {
    todo!("0x243e98 __ZN3RBX6Limits9CountableC2Ev")
}

#[doc(alias = "RBX::Limits::Counter::add(RBX::Limits::Countable *)")]
// 0x244088 — __ZN3RBX6Limits7Counter3addEPNS0_9CountableE
// type: void __fastcall(int32_t *, volatile int *)
// was: RBX::Limits::Counter::add(RBX::Limits::Countable *)
pub fn stub_0x244088() -> ! {
    todo!("0x244088 __ZN3RBX6Limits7Counter3addEPNS0_9CountableE")
}

#[doc(alias = "RBX::Limits::Countable::~Countable()")]
// 0x244200 — __ZN3RBX6Limits9CountableD2Ev
// type: void __fastcall(int32_t **this, volatile int *)
// was: RBX::Limits::Countable::~Countable()
pub fn stub_0x244200() -> ! {
    todo!("0x244200 __ZN3RBX6Limits9CountableD2Ev")
}

#[doc(alias = "RBX::Limits::Counter::getCurrentCount(void)")]
// 0x2442c4 — __ZN3RBX6Limits7Counter15getCurrentCountEv
// type: _DWORD __fastcall(RBX::Limits::Counter *__hidden this)
// was: RBX::Limits::Counter::getCurrentCount(void)
pub fn stub_0x2442c4() -> ! {
    todo!("0x2442c4 __ZN3RBX6Limits7Counter15getCurrentCountEv")
}

#[doc(alias = "RBX::Limits::Counter::canAdd(int)")]
// 0x244358 — __ZN3RBX6Limits7Counter6canAddEi
// type: bool __fastcall(RBX::Limits::Counter *this, int)
// was: RBX::Limits::Counter::canAdd(int)
pub fn stub_0x244358() -> ! {
    todo!("0x244358 __ZN3RBX6Limits7Counter6canAddEi")
}

#[doc(alias = "RBX::Limits::Counter::Activator::Activator(rbx_core::SharedPtr<RBX::Limits::Counter>)")]
// 0x244384 — __ZN3RBX6Limits7Counter9ActivatorC1EN5boost10shared_ptrIS1_EE
// was: RBX::Limits::Counter::Activator::Activator(rbx_core::SharedPtr<RBX::Limits::Counter>)
pub fn stub_0x244384() -> ! {
    todo!("0x244384 __ZN3RBX6Limits7Counter9ActivatorC1EN5boost10shared_ptrIS1_EE")
}

#[doc(alias = "RBX::Limits::Counter::Activator::Activator(rbx_core::SharedPtr<RBX::Limits::Counter>)")]
// 0x244390 — __ZN3RBX6Limits7Counter9ActivatorC2EN5boost10shared_ptrIS1_EE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, void *, int, int, int, int)
// was: RBX::Limits::Counter::Activator::Activator(rbx_core::SharedPtr<RBX::Limits::Counter>)
pub fn stub_0x244390() -> ! {
    todo!("0x244390 __ZN3RBX6Limits7Counter9ActivatorC2EN5boost10shared_ptrIS1_EE")
}

#[doc(alias = "RBX::Limits::Counter::Activator::~Activator()")]
// 0x2445fc — __ZN3RBX6Limits7Counter9ActivatorD1Ev
// type: void __fastcall(RBX::Limits::Counter::Activator *__hidden this)
// was: RBX::Limits::Counter::Activator::~Activator()
pub fn stub_0x2445fc() -> ! {
    todo!("0x2445fc __ZN3RBX6Limits7Counter9ActivatorD1Ev")
}

#[doc(alias = "RBX::Limits::Counter::Activator::~Activator()")]
// 0x244608 — __ZN3RBX6Limits7Counter9ActivatorD2Ev
// type: void __fastcall(RBX::Limits::Counter::Activator *this, int, int, int)
// was: RBX::Limits::Counter::Activator::~Activator()
pub fn stub_0x244608() -> ! {
    todo!("0x244608 __ZN3RBX6Limits7Counter9ActivatorD2Ev")
}

#[doc(alias = "boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::reset(rbx_core::SharedPtr<RBX::Limits::Counter>*)")]
// 0x24480c — __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE5resetEPS5_
// type: void __fastcall(int *, const void *)
// was: boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::reset(rbx_core::SharedPtr<RBX::Limits::Counter>*)
pub fn stub_0x24480c() -> ! {
    todo!("0x24480c __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE5resetEPS5_")
}

#[doc(alias = "RBX::Limits::Counter::safe_static_init_current(void)")]
// 0x244928 — __ZN3RBX6Limits7Counter24safe_static_init_currentEv
// type: int __fastcall(RBX::Limits::Counter *this)
// was: RBX::Limits::Counter::safe_static_init_current(void)
pub fn stub_0x244928() -> ! {
    todo!("0x244928 __ZN3RBX6Limits7Counter24safe_static_init_currentEv")
}

#[doc(alias = "RBX::Limits::Counter::safe_static_do_get_current(void)")]
// 0x244934 — __ZN3RBX6Limits7Counter26safe_static_do_get_currentEv
// type: int *__fastcall(RBX::Limits::Counter *this)
// was: RBX::Limits::Counter::safe_static_do_get_current(void)
pub fn stub_0x244934() -> ! {
    todo!("0x244934 __ZN3RBX6Limits7Counter26safe_static_do_get_currentEv")
}

#[doc(alias = "rbx::thread_specific_shared_ptr<RBX::Limits::Counter>::~thread_specific_shared_ptr()")]
// 0x244ab8 — __ZN3rbx26thread_specific_shared_ptrIN3RBX6Limits7CounterEED1Ev
// was: rbx::thread_specific_shared_ptr<RBX::Limits::Counter>::~thread_specific_shared_ptr()
pub fn stub_0x244ab8() -> ! {
    todo!("0x244ab8 __ZN3rbx26thread_specific_shared_ptrIN3RBX6Limits7CounterEED1Ev")
}

#[doc(alias = "boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::~thread_specific_ptr()")]
// 0x244ac8 — __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEED2Ev
// type: boost::_anonymous_namespace_ *__fastcall(boost::_anonymous_namespace_ *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
// was: boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::~thread_specific_ptr()
pub fn stub_0x244ac8() -> ! {
    todo!("0x244ac8 __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEED2Ev")
}

#[doc(alias = "boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data::~delete_data()")]
// 0x244bbc — __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataD1Ev
// type: void()
// was: boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data::~delete_data()
pub fn stub_0x244bbc() -> ! {
    todo!("0x244bbc __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataD1Ev")
}

#[doc(alias = "boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data::~delete_data()")]
// 0x244bc0 — __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataD0Ev
// type: void __fastcall(void *)
// was: boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data::~delete_data()
pub fn stub_0x244bc0() -> ! {
    todo!("0x244bc0 __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataD0Ev")
}

#[doc(alias = "boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data::operator()(void *)")]
// 0x244bcc — __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataclEPv
// type: void __fastcall(int, _DWORD *)
// was: boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data::operator()(void *)
pub fn stub_0x244bcc() -> ! {
    todo!("0x244bcc __ZN5boost19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataclEPv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data>>::~sp_counted_impl_pd()")]
// 0x244c74 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEED1Ev
// type: void()
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data>>::~sp_counted_impl_pd()
pub fn stub_0x244c74() -> ! {
    todo!("0x244c74 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data>>::~sp_counted_impl_pd()")]
// 0x244c78 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEED0Ev
// type: void __fastcall(void *)
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data>>::~sp_counted_impl_pd()
pub fn stub_0x244c78() -> ! {
    todo!("0x244c78 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data>>::dispose(void)")]
// 0x244c84 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEE7disposeEv
// type: int __fastcall(int)
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data>>::dispose(void)
pub fn stub_0x244c84() -> ! {
    todo!("0x244c84 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data>>::get_deleter(std::type_info const&)")]
// 0x244c98 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data>>::get_deleter(std::type_info const&)
pub fn stub_0x244c98() -> ! {
    todo!("0x244c98 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data>>::get_untyped_deleter(void)")]
// 0x244cb0 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEE19get_untyped_deleterEv
// type: int __fastcall(int)
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<rbx_core::SharedPtr<RBX::Limits::Counter>>::delete_data>>::get_untyped_deleter(void)
pub fn stub_0x244cb0() -> ! {
    todo!("0x244cb0 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrINS_10shared_ptrIN3RBX6Limits7CounterEEEE11delete_dataENS0_14do_heap_deleteIS9_EEE19get_untyped_deleterEv")
}

#[doc(alias = "RBX::roblox_allocator::malloc(unsigned long)")]
// 0x244d7c — __ZN3RBX16roblox_allocator6mallocEm
// type: void *__fastcall(size_t this, unsigned int)
// was: RBX::roblox_allocator::malloc(unsigned long)
pub fn stub_0x244d7c() -> ! {
    todo!("0x244d7c __ZN3RBX16roblox_allocator6mallocEm")
}

#[doc(alias = "RBX::roblox_allocator::free(char *)")]
// 0x244dac — __ZN3RBX16roblox_allocator4freeEPc
// type: void __fastcall(RBX::roblox_allocator *this, char *)
// was: RBX::roblox_allocator::free(char *)
pub fn stub_0x244dac() -> ! {
    todo!("0x244dac __ZN3RBX16roblox_allocator4freeEPc")
}

#[doc(alias = "boost::function<void ()(std::exception &)>::~function()")]
// 0x24551c — __ZN5boost8functionIFvRSt9exceptionEED1Ev
// type: int *__fastcall(int *)
// was: boost::function<void ()(std::exception &)>::~function()
pub fn stub_0x24551c() -> ! {
    todo!("0x24551c __ZN5boost8functionIFvRSt9exceptionEED1Ev")
}

#[doc(alias = "RBX::Tasks::SequenceBase::isInhibited(RBX::TaskScheduler::Job *)")]
// 0x2456a0 — __ZN3RBX5Tasks12SequenceBase11isInhibitedEPNS_13TaskScheduler3JobE
// type: bool __fastcall(RBX::Tasks::SequenceBase *this, RBX::TaskScheduler::Job *)
// was: RBX::Tasks::SequenceBase::isInhibited(RBX::TaskScheduler::Job *)
pub fn stub_0x2456a0() -> ! {
    todo!("0x2456a0 __ZN3RBX5Tasks12SequenceBase11isInhibitedEPNS_13TaskScheduler3JobE")
}

#[doc(alias = "RBX::Tasks::SequenceBase::advance(void)")]
// 0x2456d8 — __ZN3RBX5Tasks12SequenceBase7advanceEv
// type: int __fastcall(RBX::Tasks::SequenceBase *this)
// was: RBX::Tasks::SequenceBase::advance(void)
pub fn stub_0x2456d8() -> ! {
    todo!("0x2456d8 __ZN3RBX5Tasks12SequenceBase7advanceEv")
}

#[doc(alias = "RBX::Tasks::SequenceBase::onAdded(RBX::TaskScheduler::Job *)")]
// 0x245708 — __ZN3RBX5Tasks12SequenceBase7onAddedEPNS_13TaskScheduler3JobE
// type: void __fastcall(RBX::Tasks::SequenceBase *this, RBX::TaskScheduler::Job *)
// was: RBX::Tasks::SequenceBase::onAdded(RBX::TaskScheduler::Job *)
pub fn stub_0x245708() -> ! {
    todo!("0x245708 __ZN3RBX5Tasks12SequenceBase7onAddedEPNS_13TaskScheduler3JobE")
}

#[doc(alias = "RBX::Tasks::SequenceBase::onRemoved(RBX::TaskScheduler::Job *)")]
// 0x2457f0 — __ZN3RBX5Tasks12SequenceBase9onRemovedEPNS_13TaskScheduler3JobE
// type: int __fastcall(RBX::Tasks::SequenceBase *this, RBX::TaskScheduler::Job *)
// was: RBX::Tasks::SequenceBase::onRemoved(RBX::TaskScheduler::Job *)
pub fn stub_0x2457f0() -> ! {
    todo!("0x2457f0 __ZN3RBX5Tasks12SequenceBase9onRemovedEPNS_13TaskScheduler3JobE")
}

#[doc(alias = "std::vector<RBX::TaskScheduler::Job *,std::allocator<RBX::TaskScheduler::Job *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::Job **,std::vector<RBX::TaskScheduler::Job *,std::allocator<RBX::TaskScheduler::Job *>>>,RBX::TaskScheduler::Job * const&)")]
// 0x245848 — __ZNSt6vectorIPN3RBX13TaskScheduler3JobESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: void *__fastcall(int, char *__src, _DWORD *)
// was: std::vector<RBX::TaskScheduler::Job *,std::allocator<RBX::TaskScheduler::Job *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::Job **,std::vector<RBX::TaskScheduler::Job *,std::allocator<RBX::TaskScheduler::Job *>>>,RBX::TaskScheduler::Job * const&)
pub fn stub_0x245848() -> ! {
    todo!("0x245848 __ZNSt6vectorIPN3RBX13TaskScheduler3JobESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")
}

#[doc(alias = "RBX::TaskScheduler::getSchedulerDutyCyclePerThread(void)const")]
// 0x245a08 — __ZNK3RBX13TaskScheduler30getSchedulerDutyCyclePerThreadEv
// type: __int64 __fastcall(RBX::TaskScheduler *this)
// was: RBX::TaskScheduler::getSchedulerDutyCyclePerThread(void)const
pub fn stub_0x245a08() -> ! {
    todo!("0x245a08 __ZNK3RBX13TaskScheduler30getSchedulerDutyCyclePerThreadEv")
}

#[doc(alias = "RBX::ExclusiveArbiter::areExclusive(RBX::TaskScheduler::Job *,RBX::TaskScheduler::Job *)")]
// 0x245ab0 — __ZN3RBX16ExclusiveArbiter12areExclusiveEPNS_13TaskScheduler3JobES3_
// type: int __fastcall(RBX::ExclusiveArbiter *this, RBX::TaskScheduler::Job *, RBX::TaskScheduler::Job *)
// was: RBX::ExclusiveArbiter::areExclusive(RBX::TaskScheduler::Job *,RBX::TaskScheduler::Job *)
pub fn stub_0x245ab0() -> ! {
    todo!("0x245ab0 __ZN3RBX16ExclusiveArbiter12areExclusiveEPNS_13TaskScheduler3JobES3_")
}

#[doc(alias = "RBX::TaskScheduler::static_init(void)")]
// 0x245b68 — __ZN3RBX13TaskScheduler11static_initEv
// type: void __fastcall(RBX::TaskScheduler *this, int, int, int)
// was: RBX::TaskScheduler::static_init(void)
pub fn stub_0x245b68() -> ! {
    todo!("0x245b68 __ZN3RBX13TaskScheduler11static_initEv")
}

#[doc(alias = "RBX::TaskScheduler::~TaskScheduler()")]
// 0x245c64 — __ZN3RBX13TaskSchedulerD1Ev
// type: void __fastcall(RBX::TaskScheduler *__hidden this)
// was: RBX::TaskScheduler::~TaskScheduler()
pub fn stub_0x245c64() -> ! {
    todo!("0x245c64 __ZN3RBX13TaskSchedulerD1Ev")
}

#[doc(alias = "RBX::TaskScheduler::singleton(void)")]
// 0x245c70 — __ZN3RBX13TaskScheduler9singletonEv
// type: _DWORD __fastcall(RBX::TaskScheduler *__hidden this)
// was: RBX::TaskScheduler::singleton(void)
pub fn stub_0x245c70() -> ! {
    todo!("0x245c70 __ZN3RBX13TaskScheduler9singletonEv")
}

#[doc(alias = "RBX::TaskScheduler::TaskScheduler(void)")]
// 0x245c94 — __ZN3RBX13TaskSchedulerC2Ev
// type: int __fastcall(RBX::TaskScheduler *this, int, int)
// was: RBX::TaskScheduler::TaskScheduler(void)
pub fn stub_0x245c94() -> ! {
    todo!("0x245c94 __ZN3RBX13TaskSchedulerC2Ev")
}

#[doc(alias = "RBX::TaskScheduler::sampleRunningJobCount(void)")]
// 0x246308 — __ZN3RBX13TaskScheduler21sampleRunningJobCountEv
// type: bool __fastcall(RBX::TaskScheduler *this, int, int)
// was: RBX::TaskScheduler::sampleRunningJobCount(void)
pub fn stub_0x246308() -> ! {
    todo!("0x246308 __ZN3RBX13TaskScheduler21sampleRunningJobCountEv")
}

#[doc(alias = "RBX::TaskScheduler::~TaskScheduler()")]
// 0x246358 — __ZN3RBX13TaskSchedulerD2Ev
// type: void __fastcall(RBX::TaskScheduler *this, int, int, const void *)
// was: RBX::TaskScheduler::~TaskScheduler()
pub fn stub_0x246358() -> ! {
    todo!("0x246358 __ZN3RBX13TaskSchedulerD2Ev")
}

#[doc(alias = "RBX::TaskScheduler::remove(rbx_core::SharedPtr<RBX::TaskScheduler::Job>,bool,boost::function<void ()(void)>)")]
// 0x2467d0 — __ZN3RBX13TaskScheduler6removeEN5boost10shared_ptrINS0_3JobEEEbNS1_8functionIFvvEEE
// type: void __fastcall(int, int *, unsigned __int8, int)
// was: RBX::TaskScheduler::remove(rbx_core::SharedPtr<RBX::TaskScheduler::Job>,bool,boost::function<void ()(void)>)
pub fn stub_0x2467d0() -> ! {
    todo!("0x2467d0 __ZN3RBX13TaskScheduler6removeEN5boost10shared_ptrINS0_3JobEEEbNS1_8functionIFvvEEE")
}

#[doc(alias = "RBX::TaskScheduler::remove(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&,rbx_core::SharedPtr<RBX::CEvent>)")]
// 0x246a48 — __ZN3RBX13TaskScheduler6removeERKN5boost10shared_ptrINS0_3JobEEENS2_INS_6CEventEEE
// type: void __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int)
// was: RBX::TaskScheduler::remove(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&,rbx_core::SharedPtr<RBX::CEvent>)
pub fn stub_0x246a48() -> ! {
    todo!("0x246a48 __ZN3RBX13TaskScheduler6removeERKN5boost10shared_ptrINS0_3JobEEENS2_INS_6CEventEEE")
}

#[doc(alias = "RBX::TaskScheduler::reschedule(rbx_core::SharedPtr<RBX::TaskScheduler::Job>)")]
// 0x246da8 — __ZN3RBX13TaskScheduler10rescheduleEN5boost10shared_ptrINS0_3JobEEE
// type: void __fastcall(int, RBX::TaskScheduler::Job **)
// was: RBX::TaskScheduler::reschedule(rbx_core::SharedPtr<RBX::TaskScheduler::Job>)
pub fn stub_0x246da8() -> ! {
    todo!("0x246da8 __ZN3RBX13TaskScheduler10rescheduleEN5boost10shared_ptrINS0_3JobEEE")
}

#[doc(alias = "RBX::TaskScheduler::scheduleJob(RBX::TaskScheduler::Job &)")]
// 0x246e98 — __ZN3RBX13TaskScheduler11scheduleJobERNS0_3JobE
// type: int __fastcall(RBX::TaskScheduler *this, RBX::TaskScheduler::Job *)
// was: RBX::TaskScheduler::scheduleJob(RBX::TaskScheduler::Job &)
pub fn stub_0x246e98() -> ! {
    todo!("0x246e98 __ZN3RBX13TaskScheduler11scheduleJobERNS0_3JobE")
}

#[doc(alias = "RBX::TaskScheduler::add(rbx_core::SharedPtr<RBX::TaskScheduler::Job>)")]
// 0x246f90 — __ZN3RBX13TaskScheduler3addEN5boost10shared_ptrINS0_3JobEEE
// type: void __fastcall(int, int, int, int, int, pthread_mutex_t *, int, int, int, int)
// was: RBX::TaskScheduler::add(rbx_core::SharedPtr<RBX::TaskScheduler::Job>)
pub fn stub_0x246f90() -> ! {
    todo!("0x246f90 __ZN3RBX13TaskScheduler3addEN5boost10shared_ptrINS0_3JobEEE")
}

#[doc(alias = "RBX::TaskScheduler::incrementThreadCount(void)")]
// 0x24710c — __ZN3RBX13TaskScheduler20incrementThreadCountEv
// type: int __fastcall(int32_t *this, volatile int *)
// was: RBX::TaskScheduler::incrementThreadCount(void)
pub fn stub_0x24710c() -> ! {
    todo!("0x24710c __ZN3RBX13TaskScheduler20incrementThreadCountEv")
}

#[doc(alias = "RBX::TaskScheduler::decrementThreadCount(void)")]
// 0x24711c — __ZN3RBX13TaskScheduler20decrementThreadCountEv
// type: int __fastcall(int32_t *this, volatile int *)
// was: RBX::TaskScheduler::decrementThreadCount(void)
pub fn stub_0x24711c() -> ! {
    todo!("0x24711c __ZN3RBX13TaskScheduler20decrementThreadCountEv")
}

#[doc(alias = "RBX::TaskScheduler::getShortestSleepTime(void)const")]
// 0x247130 — __ZNK3RBX13TaskScheduler20getShortestSleepTimeEv
// type: int __fastcall(RBX::TaskScheduler *this, int)
// was: RBX::TaskScheduler::getShortestSleepTime(void)const
pub fn stub_0x247130() -> ! {
    todo!("0x247130 __ZNK3RBX13TaskScheduler20getShortestSleepTimeEv")
}

#[doc(alias = "RBX::TaskScheduler::wakeSleepingJobs(void)")]
// 0x247154 — __ZN3RBX13TaskScheduler16wakeSleepingJobsEv
// type: int __fastcall(RBX::TaskScheduler *this)
// was: RBX::TaskScheduler::wakeSleepingJobs(void)
pub fn stub_0x247154() -> ! {
    todo!("0x247154 __ZN3RBX13TaskScheduler16wakeSleepingJobsEv")
}

#[doc(alias = "RBX::TaskScheduler::findJobToRun(rbx_core::SharedPtr<RBX::TaskScheduler::Thread>)")]
// 0x247220 — __ZN3RBX13TaskScheduler12findJobToRunEN5boost10shared_ptrINS0_6ThreadEEE
// type: void __fastcall(RBX::TaskScheduler::Job **, int, int *, int, int, int, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
// was: RBX::TaskScheduler::findJobToRun(rbx_core::SharedPtr<RBX::TaskScheduler::Thread>)
pub fn stub_0x247220() -> ! {
    todo!("0x247220 __ZN3RBX13TaskScheduler12findJobToRunEN5boost10shared_ptrINS0_6ThreadEEE")
}

#[doc(alias = "rbx::thread_specific_reference<RBX::TaskScheduler::Job>::~thread_specific_reference()")]
// 0x247bd8 — __ZN3rbx25thread_specific_referenceIN3RBX13TaskScheduler3JobEED1Ev
// was: rbx::thread_specific_reference<RBX::TaskScheduler::Job>::~thread_specific_reference()
pub fn stub_0x247bd8() -> ! {
    todo!("0x247bd8 __ZN3rbx25thread_specific_referenceIN3RBX13TaskScheduler3JobEED1Ev")
}

#[doc(alias = "RBX::TaskScheduler::Job::getDebugName(void)const")]
// 0x247be8 — __ZNK3RBX13TaskScheduler3Job12getDebugNameEv
// type: void __fastcall(RBX::TaskScheduler::Job *this, int)
// was: RBX::TaskScheduler::Job::getDebugName(void)const
pub fn stub_0x247be8() -> ! {
    todo!("0x247be8 __ZNK3RBX13TaskScheduler3Job12getDebugNameEv")
}

#[doc(alias = "RBX::RunningAverage<int,double>::sample(int)")]
// 0x247db0 — __ZN3RBX14RunningAverageIidE6sampleEi
// type: _DWORD *__fastcall(int, int)
// was: RBX::RunningAverage<int,double>::sample(int)
pub fn stub_0x247db0() -> ! {
    todo!("0x247db0 __ZN3RBX14RunningAverageIidE6sampleEi")
}

#[doc(alias = "RBX::ExclusiveArbiter::arbiterName(void)")]
// 0x247e74 — __ZN3RBX16ExclusiveArbiter11arbiterNameEv
// type: int __fastcall(RBX::ExclusiveArbiter *this)
// was: RBX::ExclusiveArbiter::arbiterName(void)
pub fn stub_0x247e74() -> ! {
    todo!("0x247e74 __ZN3RBX16ExclusiveArbiter11arbiterNameEv")
}

#[doc(alias = "RBX::ExclusiveArbiter::isThrottled(void)")]
// 0x247e90 — __ZN3RBX16ExclusiveArbiter11isThrottledEv
// type: int __fastcall(RBX::ExclusiveArbiter *this)
// was: RBX::ExclusiveArbiter::isThrottled(void)
pub fn stub_0x247e90() -> ! {
    todo!("0x247e90 __ZN3RBX16ExclusiveArbiter11isThrottledEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Job>::shared_ptr<RBX::TaskScheduler::Job>(rbx_core::WeakPtr<RBX::TaskScheduler::Job> const&)")]
// 0x247e94 — __ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEC2IS3_EERKNS_8weak_ptrIT_EE
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
// was: rbx_core::SharedPtr<RBX::TaskScheduler::Job>::shared_ptr<RBX::TaskScheduler::Job>(rbx_core::WeakPtr<RBX::TaskScheduler::Job> const&)
pub fn stub_0x247e94() -> ! {
    todo!("0x247e94 __ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEC2IS3_EERKNS_8weak_ptrIT_EE")
}

#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::less<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::erase(std::_Rb_tree_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::_Rb_tree_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>)")]
// 0x247fac — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_ESD_
// type: _Rb_tree_node_base *__fastcall(_DWORD *, _Rb_tree_node_base *, _Rb_tree_node_base *)
// was: std::_Rb_tree<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::less<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::erase(std::_Rb_tree_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::_Rb_tree_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>)
pub fn stub_0x247fac() -> ! {
    todo!("0x247fac __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_ESD_")
}

#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::less<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::_M_erase(std::_Rb_tree_node<rbx_core::SharedPtr<RBX::TaskScheduler::Job>> *)")]
// 0x248020 — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: void __fastcall(int, _DWORD *)
// was: std::_Rb_tree<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::less<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::_M_erase(std::_Rb_tree_node<rbx_core::SharedPtr<RBX::TaskScheduler::Job>> *)
pub fn stub_0x248020() -> ! {
    todo!("0x248020 __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")
}

#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::less<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::_M_insert_unique(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&)")]
// 0x248050 — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, _DWORD *, unsigned int M_parent, int)
// was: std::_Rb_tree<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::less<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::_M_insert_unique(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&)
pub fn stub_0x248050() -> ! {
    todo!("0x248050 __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE16_M_insert_uniqueERKS5_")
}

#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::less<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::_M_create_node(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&)")]
// 0x248104 — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE14_M_create_nodeERKS5_
// type: int __fastcall(int, int *, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, void *, int)
// was: std::_Rb_tree<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::less<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::_M_create_node(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&)
pub fn stub_0x248104() -> ! {
    todo!("0x248104 __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE14_M_create_nodeERKS5_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::CEvent>(RBX::CEvent *)")]
// 0x248224 — __ZN5boost6detail12shared_countC2IN3RBX6CEventEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
// was: boost::detail::shared_count::shared_count<RBX::CEvent>(RBX::CEvent *)
pub fn stub_0x248224() -> ! {
    todo!("0x248224 __ZN5boost6detail12shared_countC2IN3RBX6CEventEEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CEvent>::~sp_counted_impl_p()")]
// 0x24831c — __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEED1Ev
// type: void()
// was: boost::detail::sp_counted_impl_p<RBX::CEvent>::~sp_counted_impl_p()
pub fn stub_0x24831c() -> ! {
    todo!("0x24831c __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CEvent>::~sp_counted_impl_p()")]
// 0x248320 — __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEED0Ev
// type: void __fastcall(void *)
// was: boost::detail::sp_counted_impl_p<RBX::CEvent>::~sp_counted_impl_p()
pub fn stub_0x248320() -> ! {
    todo!("0x248320 __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CEvent>::dispose(void)")]
// 0x24832c — __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEE7disposeEv
// type: void __fastcall(int)
// was: boost::detail::sp_counted_impl_p<RBX::CEvent>::dispose(void)
pub fn stub_0x24832c() -> ! {
    todo!("0x24832c __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CEvent>::get_deleter(std::type_info const&)")]
// 0x24834c — __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEE11get_deleterERKSt9type_info
// type: int()
// was: boost::detail::sp_counted_impl_p<RBX::CEvent>::get_deleter(std::type_info const&)
pub fn stub_0x24834c() -> ! {
    todo!("0x24834c __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CEvent>::get_untyped_deleter(void)")]
// 0x248350 — __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEE19get_untyped_deleterEv
// type: int()
// was: boost::detail::sp_counted_impl_p<RBX::CEvent>::get_untyped_deleter(void)
pub fn stub_0x248350() -> ! {
    todo!("0x248350 __ZN5boost6detail17sp_counted_impl_pIN3RBX6CEventEE19get_untyped_deleterEv")
}

#[doc(alias = "boost::detail::thread_data<boost::function0<void>>::~thread_data()")]
// 0x248358 — __ZN5boost6detail11thread_dataINS_9function0IvEEED1Ev
// type: int __fastcall(boost::detail::thread_data_base *)
// was: boost::detail::thread_data<boost::function0<void>>::~thread_data()
pub fn stub_0x248358() -> ! {
    todo!("0x248358 __ZN5boost6detail11thread_dataINS_9function0IvEEED1Ev")
}

#[doc(alias = "boost::condition_variable::condition_variable(void)")]
// 0x248448 — __ZN5boost18condition_variableC2Ev
// type: boost::condition_variable *__fastcall(boost::condition_variable *this)
// was: boost::condition_variable::condition_variable(void)
pub fn stub_0x248448() -> ! {
    todo!("0x248448 __ZN5boost18condition_variableC2Ev")
}

#[doc(alias = "void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data<boost::function0<void>>>(rbx_core::SharedPtr<boost::detail::thread_data_base> const*,boost::detail::thread_data<boost::function0<void>> *)const")]
// 0x248620 — __ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_NS1_11thread_dataINS_9function0IvEEEEEEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int)
// was: void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data<boost::function0<void>>>(rbx_core::SharedPtr<boost::detail::thread_data_base> const*,boost::detail::thread_data<boost::function0<void>> *)const
pub fn stub_0x248620() -> ! {
    todo!("0x248620 __ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_NS1_11thread_dataINS_9function0IvEEEEEEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::function0<void>>>::get_untyped_deleter(void)")]
// 0x248778 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_9function0IvEEEEE19get_untyped_deleterEv
// type: int()
// was: boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::function0<void>>>::get_untyped_deleter(void)
pub fn stub_0x248778() -> ! {
    todo!("0x248778 __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_9function0IvEEEEE19get_untyped_deleterEv")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler>,boost::_bi::list1<boost::_bi::value<RBX::TaskScheduler*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x24877c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskSchedulerEEENS3_5list1INS3_5valueIPS8_EEEEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler>,boost::_bi::list1<boost::_bi::value<RBX::TaskScheduler*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0x24877c() -> ! {
    todo!("0x24877c __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskSchedulerEEENS3_5list1INS3_5valueIPS8_EEEEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler>,boost::_bi::list1<boost::_bi::value<RBX::TaskScheduler*>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// 0x2487dc — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskSchedulerEEENS3_5list1INS3_5valueIPS8_EEEEEEvE6invokeERNS1_15function_bufferE
// type: int __fastcall(int)
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler>,boost::_bi::list1<boost::_bi::value<RBX::TaskScheduler*>>>,void>::invoke(boost::detail::function::function_buffer &)
pub fn stub_0x2487dc() -> ! {
    todo!("0x2487dc __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskSchedulerEEENS3_5list1INS3_5valueIPS8_EEEEEEvE6invokeERNS1_15function_bufferE")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>> const&)")]
// 0x2487f8 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEEC1ERKS5_
// type: int __fastcall(int, int)
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>> const&)
pub fn stub_0x2487f8() -> ! {
    todo!("0x2487f8 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEEC1ERKS5_")
}

#[doc(alias = "boost::function0<void>::dummy::nonnull(void)")]
// 0x248938 — __ZN5boost9function0IvE5dummy7nonnullEv
// type: void()
// was: boost::function0<void>::dummy::nonnull(void)
pub fn stub_0x248938() -> ! {
    todo!("0x248938 __ZN5boost9function0IvE5dummy7nonnullEv")
}

#[doc(alias = "boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::~thread_specific_ptr()")]
// 0x248a8c — __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEED2Ev
// type: boost::_anonymous_namespace_ *__fastcall(boost::_anonymous_namespace_ *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
// was: boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::~thread_specific_ptr()
pub fn stub_0x248a8c() -> ! {
    todo!("0x248a8c __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEED2Ev")
}

#[doc(alias = "boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data::~delete_data()")]
// 0x248b80 — __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataD1Ev
// type: void()
// was: boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data::~delete_data()
pub fn stub_0x248b80() -> ! {
    todo!("0x248b80 __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataD1Ev")
}

#[doc(alias = "boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data::~delete_data()")]
// 0x248b84 — __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataD0Ev
// type: void __fastcall(void *)
// was: boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data::~delete_data()
pub fn stub_0x248b84() -> ! {
    todo!("0x248b84 __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataD0Ev")
}

#[doc(alias = "boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data::operator()(void *)")]
// 0x248b90 — __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataclEPv
// type: void __fastcall(int, void *)
// was: boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data::operator()(void *)
pub fn stub_0x248b90() -> ! {
    todo!("0x248b90 __ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataclEPv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data>>::~sp_counted_impl_pd()")]
// 0x248ba0 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataENS0_14do_heap_deleteIS8_EEED1Ev
// type: void()
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data>>::~sp_counted_impl_pd()
pub fn stub_0x248ba0() -> ! {
    todo!("0x248ba0 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataENS0_14do_heap_deleteIS8_EEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data>>::~sp_counted_impl_pd()")]
// 0x248ba4 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataENS0_14do_heap_deleteIS8_EEED0Ev
// type: void __fastcall(void *)
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data>>::~sp_counted_impl_pd()
pub fn stub_0x248ba4() -> ! {
    todo!("0x248ba4 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataENS0_14do_heap_deleteIS8_EEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data>>::dispose(void)")]
// 0x248bb0 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataENS0_14do_heap_deleteIS8_EEE7disposeEv
// type: int __fastcall(int)
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data>>::dispose(void)
pub fn stub_0x248bb0() -> ! {
    todo!("0x248bb0 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataENS0_14do_heap_deleteIS8_EEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data>>::get_deleter(std::type_info const&)")]
// 0x248bc4 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataENS0_14do_heap_deleteIS8_EEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data>>::get_deleter(std::type_info const&)
pub fn stub_0x248bc4() -> ! {
    todo!("0x248bc4 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataENS0_14do_heap_deleteIS8_EEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data>>::get_untyped_deleter(void)")]
// 0x248bdc — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataENS0_14do_heap_deleteIS8_EEE19get_untyped_deleterEv
// type: int __fastcall(int)
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::delete_data>>::get_untyped_deleter(void)
pub fn stub_0x248bdc() -> ! {
    todo!("0x248bdc __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE11delete_dataENS0_14do_heap_deleteIS8_EEE19get_untyped_deleterEv")
}

#[doc(alias = "RBX::TaskScheduler::Job::averageDutyCycle(void)const")]
// 0x248e38 — __ZNK3RBX13TaskScheduler3Job16averageDutyCycleEv
// type: __int64 __fastcall(RBX::TaskScheduler::Job *this)
// was: RBX::TaskScheduler::Job::averageDutyCycle(void)const
pub fn stub_0x248e38() -> ! {
    todo!("0x248e38 __ZNK3RBX13TaskScheduler3Job16averageDutyCycleEv")
}

#[doc(alias = "RBX::TaskScheduler::Job::averageStepsPerSecond(void)const")]
// 0x248eb0 — __ZNK3RBX13TaskScheduler3Job21averageStepsPerSecondEv
// type: __int64 __fastcall(RBX::TaskScheduler::Job *this)
// was: RBX::TaskScheduler::Job::averageStepsPerSecond(void)const
pub fn stub_0x248eb0() -> ! {
    todo!("0x248eb0 __ZNK3RBX13TaskScheduler3Job21averageStepsPerSecondEv")
}

#[doc(alias = "RBX::TaskScheduler::Job::averageStepTime(void)const")]
// 0x248f10 — __ZNK3RBX13TaskScheduler3Job15averageStepTimeEv
// type: __int64 __fastcall(RBX::TaskScheduler::Job *this)
// was: RBX::TaskScheduler::Job::averageStepTime(void)const
pub fn stub_0x248f10() -> ! {
    todo!("0x248f10 __ZNK3RBX13TaskScheduler3Job15averageStepTimeEv")
}
