//! core shard oa — 120 core stubs EA-sorted asc gap filler global not yet in crates (global).
//! Source: `ida/export.json` (85545 funcs) EA-sorted asc, next 120 not yet in crates as stub_0x (global distinct 33042 before -> 32922 after, batch 0x5dbca8..0x60005c).
//! Filter: global EA-sorted asc next uncovered (no namespace filter), rbx_core::SharedPtr not boost.
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "anonymous namespace::computeNetworkOwnerIsSomeoneElseImpl(RBX::SystemAddress const&,RBX::SystemAddress const&)")]
#[doc(alias = "__ZN12_GLOBAL__N_136computeNetworkOwnerIsSomeoneElseImplERKN3RBX13SystemAddressES3_")]
// 0x5dbca8 — __ZN12_GLOBAL__N_136computeNetworkOwnerIsSomeoneElseImplERKN3RBX13SystemAddressES3_
// type: int __fastcall(int, int)
pub fn stub_0x5dbca8() -> ! {
    todo!("0x5dbca8 __ZN12_GLOBAL__N_136computeNetworkOwnerIsSomeoneElseImplERKN3RBX13SystemAddressES3_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::insert(rbx::signals::signal<void ()(RBX::Primitive *)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE6insertEPNS6_4slotE")]
// 0x5f7e64 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE6insertEPNS6_4slotE
// type: void __fastcall(int *, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0x5f7e64() -> ! {
    todo!("0x5f7e64 __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE6insertEPNS6_4slotE")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Primitive *)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Primitive *)>::slot*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotEEaSEPS9_")]
// 0x5f8070 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotEEaSEPS9_
// type: int(void)
pub fn stub_0x5f8070() -> ! {
    todo!("0x5f8070 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotEEaSEPS9_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Primitive *)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Primitive *)>::slot> const&)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotEEaSERKSA_")]
// 0x5f8094 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotEEaSERKSA_
// type: int(void)
pub fn stub_0x5f8094() -> ! {
    todo!("0x5f8094 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotEEaSERKSA_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE22safe_static_init_mutexEv")]
// 0x5f80b8 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE22safe_static_init_mutexEv
pub fn stub_0x5f80b8() -> ! {
    todo!("0x5f80b8 __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE24safe_static_do_get_mutexEv")]
// 0x5f80bc — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE24safe_static_do_get_mutexEv
pub fn stub_0x5f80bc() -> ! {
    todo!("0x5f80bc __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_14PhysicsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEED1Ev")]
// 0x5f81b4 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_14PhysicsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEED1Ev
pub fn stub_0x5f81b4() -> ! {
    todo!("0x5f81b4 __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_14PhysicsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_14PhysicsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEED0Ev")]
// 0x5f81e0 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_14PhysicsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEED0Ev
pub fn stub_0x5f81e0() -> ! {
    todo!("0x5f81e0 __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_14PhysicsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::slot::disconnect(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot10disconnectEv")]
// 0x5f82b4 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot10disconnectEv
pub fn stub_0x5f82b4() -> ! {
    todo!("0x5f82b4 __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot10disconnectEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::slot::connected(void)const")]
#[doc(alias = "__ZNK3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot9connectedEv")]
// 0x5f83c4 — __ZNK3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot9connectedEv
pub fn stub_0x5f83c4() -> ! {
    todo!("0x5f83c4 __ZNK3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot9connectedEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Primitive *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>>,1,void ()(RBX::Primitive *)>::call(RBX::Primitive *)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_14PhysicsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_")]
// 0x5f83d0 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_14PhysicsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_
pub fn stub_0x5f83d0() -> ! {
    todo!("0x5f83d0 __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_14PhysicsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Primitive *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>>,1,void ()(RBX::Primitive *)>::call(RBX::Primitive *)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_14PhysicsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_")]
// 0x5f83e4 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_14PhysicsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_
pub fn stub_0x5f83e4() -> ! {
    todo!("0x5f83e4 __ZThn4_N3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_14PhysicsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_")
}

#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>>::operator()<RBX::Primitive *>(RBX::Primitive * &)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX14PhysicsServiceEPNS4_9PrimitiveEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS7_EEvRT_")]
// 0x5f83f8 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX14PhysicsServiceEPNS4_9PrimitiveEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS7_EEvRT_
// type: int(void)
pub fn stub_0x5f83f8() -> ! {
    todo!("0x5f83f8 __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX14PhysicsServiceEPNS4_9PrimitiveEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS7_EEvRT_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::remove(rbx::signals::signal<void ()(RBX::Primitive *)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE6removeEPNS6_4slotE")]
// 0x5f8410 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE6removeEPNS6_4slotE
// type: int __fastcall(int, char *)
pub fn stub_0x5f8410() -> ! {
    todo!("0x5f8410 __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE6removeEPNS6_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::slot::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot22safe_static_init_mutexEv")]
// 0x5f8500 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot22safe_static_init_mutexEv
pub fn stub_0x5f8500() -> ! {
    todo!("0x5f8500 __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot24safe_static_do_get_mutexEv")]
// 0x5f8504 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot24safe_static_do_get_mutexEv
pub fn stub_0x5f8504() -> ! {
    todo!("0x5f8504 __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotD1Ev")]
// 0x5f85f4 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotD1Ev
pub fn stub_0x5f85f4() -> ! {
    todo!("0x5f85f4 __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotD1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotD0Ev")]
// 0x5f8620 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotD0Ev
pub fn stub_0x5f8620() -> ! {
    todo!("0x5f8620 __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotD0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Primitive *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>>,1,void ()(RBX::Primitive *)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_14PhysicsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_ED1Ev")]
// 0x5f86f4 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_14PhysicsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_ED1Ev
pub fn stub_0x5f86f4() -> ! {
    todo!("0x5f86f4 __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_14PhysicsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Primitive *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>>,1,void ()(RBX::Primitive *)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_14PhysicsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_ED0Ev")]
// 0x5f8720 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_14PhysicsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_ED0Ev
pub fn stub_0x5f8720() -> ! {
    todo!("0x5f8720 __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_14PhysicsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_ED0Ev")
}

#[doc(alias = "RBX::PhysicsSettings::getShowAnchoredParts(void)const")]
#[doc(alias = "__ZNK3RBX15PhysicsSettings20getShowAnchoredPartsEv")]
// 0x5f8a64 — __ZNK3RBX15PhysicsSettings20getShowAnchoredPartsEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
pub fn stub_0x5f8a64() -> ! {
    todo!("0x5f8a64 __ZNK3RBX15PhysicsSettings20getShowAnchoredPartsEv")
}

#[doc(alias = "RBX::PhysicsSettings::setShowAnchoredParts(bool)")]
#[doc(alias = "__ZN3RBX15PhysicsSettings20setShowAnchoredPartsEb")]
// 0x5f8a74 — __ZN3RBX15PhysicsSettings20setShowAnchoredPartsEb
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this, bool)
pub fn stub_0x5f8a74() -> ! {
    todo!("0x5f8a74 __ZN3RBX15PhysicsSettings20setShowAnchoredPartsEb")
}

#[doc(alias = "RBX::PhysicsSettings::getShowPartCoordinateFrames(void)const")]
#[doc(alias = "__ZNK3RBX15PhysicsSettings27getShowPartCoordinateFramesEv")]
// 0x5f8aa4 — __ZNK3RBX15PhysicsSettings27getShowPartCoordinateFramesEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
pub fn stub_0x5f8aa4() -> ! {
    todo!("0x5f8aa4 __ZNK3RBX15PhysicsSettings27getShowPartCoordinateFramesEv")
}

#[doc(alias = "RBX::PhysicsSettings::setShowPartCoordinateFrames(bool)")]
#[doc(alias = "__ZN3RBX15PhysicsSettings27setShowPartCoordinateFramesEb")]
// 0x5f8ab4 — __ZN3RBX15PhysicsSettings27setShowPartCoordinateFramesEb
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this, bool)
pub fn stub_0x5f8ab4() -> ! {
    todo!("0x5f8ab4 __ZN3RBX15PhysicsSettings27setShowPartCoordinateFramesEb")
}

#[doc(alias = "RBX::PhysicsSettings::getShowUnalignedParts(void)const")]
#[doc(alias = "__ZNK3RBX15PhysicsSettings21getShowUnalignedPartsEv")]
// 0x5f8ae4 — __ZNK3RBX15PhysicsSettings21getShowUnalignedPartsEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
pub fn stub_0x5f8ae4() -> ! {
    todo!("0x5f8ae4 __ZNK3RBX15PhysicsSettings21getShowUnalignedPartsEv")
}

#[doc(alias = "RBX::PhysicsSettings::setShowUnalignedParts(bool)")]
#[doc(alias = "__ZN3RBX15PhysicsSettings21setShowUnalignedPartsEb")]
// 0x5f8af4 — __ZN3RBX15PhysicsSettings21setShowUnalignedPartsEb
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this, bool)
pub fn stub_0x5f8af4() -> ! {
    todo!("0x5f8af4 __ZN3RBX15PhysicsSettings21setShowUnalignedPartsEb")
}

#[doc(alias = "RBX::PhysicsSettings::getShowModelCoordinateFrames(void)const")]
#[doc(alias = "__ZNK3RBX15PhysicsSettings28getShowModelCoordinateFramesEv")]
// 0x5f8b24 — __ZNK3RBX15PhysicsSettings28getShowModelCoordinateFramesEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
pub fn stub_0x5f8b24() -> ! {
    todo!("0x5f8b24 __ZNK3RBX15PhysicsSettings28getShowModelCoordinateFramesEv")
}

#[doc(alias = "RBX::PhysicsSettings::setShowModelCoordinateFrames(bool)")]
#[doc(alias = "__ZN3RBX15PhysicsSettings28setShowModelCoordinateFramesEb")]
// 0x5f8b34 — __ZN3RBX15PhysicsSettings28setShowModelCoordinateFramesEb
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this, bool)
pub fn stub_0x5f8b34() -> ! {
    todo!("0x5f8b34 __ZN3RBX15PhysicsSettings28setShowModelCoordinateFramesEb")
}

#[doc(alias = "RBX::PhysicsSettings::getShowWorldCoordinateFrame(void)const")]
#[doc(alias = "__ZNK3RBX15PhysicsSettings27getShowWorldCoordinateFrameEv")]
// 0x5f8b64 — __ZNK3RBX15PhysicsSettings27getShowWorldCoordinateFrameEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
pub fn stub_0x5f8b64() -> ! {
    todo!("0x5f8b64 __ZNK3RBX15PhysicsSettings27getShowWorldCoordinateFrameEv")
}

#[doc(alias = "RBX::PhysicsSettings::setShowWorldCoordinateFrame(bool)")]
#[doc(alias = "__ZN3RBX15PhysicsSettings27setShowWorldCoordinateFrameEb")]
// 0x5f8b74 — __ZN3RBX15PhysicsSettings27setShowWorldCoordinateFrameEb
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this, bool)
pub fn stub_0x5f8b74() -> ! {
    todo!("0x5f8b74 __ZN3RBX15PhysicsSettings27setShowWorldCoordinateFrameEb")
}

#[doc(alias = "RBX::PhysicsSettings::getShowEPhysicsOwners(void)const")]
#[doc(alias = "__ZNK3RBX15PhysicsSettings21getShowEPhysicsOwnersEv")]
// 0x5f8ba4 — __ZNK3RBX15PhysicsSettings21getShowEPhysicsOwnersEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
pub fn stub_0x5f8ba4() -> ! {
    todo!("0x5f8ba4 __ZNK3RBX15PhysicsSettings21getShowEPhysicsOwnersEv")
}

#[doc(alias = "RBX::PhysicsSettings::setShowEPhysicsOwners(bool)")]
#[doc(alias = "__ZN3RBX15PhysicsSettings21setShowEPhysicsOwnersEb")]
// 0x5f8bb4 — __ZN3RBX15PhysicsSettings21setShowEPhysicsOwnersEb
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this, bool)
pub fn stub_0x5f8bb4() -> ! {
    todo!("0x5f8bb4 __ZN3RBX15PhysicsSettings21setShowEPhysicsOwnersEb")
}

#[doc(alias = "RBX::PhysicsSettings::getShowEPhysicsRegions(void)const")]
#[doc(alias = "__ZNK3RBX15PhysicsSettings22getShowEPhysicsRegionsEv")]
// 0x5f8be4 — __ZNK3RBX15PhysicsSettings22getShowEPhysicsRegionsEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
pub fn stub_0x5f8be4() -> ! {
    todo!("0x5f8be4 __ZNK3RBX15PhysicsSettings22getShowEPhysicsRegionsEv")
}

#[doc(alias = "RBX::PhysicsSettings::setShowEPhysicsRegions(bool)")]
#[doc(alias = "__ZN3RBX15PhysicsSettings22setShowEPhysicsRegionsEb")]
// 0x5f8bf4 — __ZN3RBX15PhysicsSettings22setShowEPhysicsRegionsEb
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this, bool)
pub fn stub_0x5f8bf4() -> ! {
    todo!("0x5f8bf4 __ZN3RBX15PhysicsSettings22setShowEPhysicsRegionsEb")
}

#[doc(alias = "RBX::PhysicsSettings::getHighlightAwakeParts(void)const")]
#[doc(alias = "__ZNK3RBX15PhysicsSettings22getHighlightAwakePartsEv")]
// 0x5f8c24 — __ZNK3RBX15PhysicsSettings22getHighlightAwakePartsEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
pub fn stub_0x5f8c24() -> ! {
    todo!("0x5f8c24 __ZNK3RBX15PhysicsSettings22getHighlightAwakePartsEv")
}

#[doc(alias = "RBX::PhysicsSettings::setHighlightAwakeParts(bool)")]
#[doc(alias = "__ZN3RBX15PhysicsSettings22setHighlightAwakePartsEb")]
// 0x5f8c34 — __ZN3RBX15PhysicsSettings22setHighlightAwakePartsEb
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this, bool)
pub fn stub_0x5f8c34() -> ! {
    todo!("0x5f8c34 __ZN3RBX15PhysicsSettings22setHighlightAwakePartsEb")
}

#[doc(alias = "RBX::PhysicsSettings::getShowReceiveAge(void)const")]
#[doc(alias = "__ZNK3RBX15PhysicsSettings17getShowReceiveAgeEv")]
// 0x5f8ca4 — __ZNK3RBX15PhysicsSettings17getShowReceiveAgeEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
pub fn stub_0x5f8ca4() -> ! {
    todo!("0x5f8ca4 __ZNK3RBX15PhysicsSettings17getShowReceiveAgeEv")
}

#[doc(alias = "RBX::PhysicsSettings::setShowReceiveAge(bool)")]
#[doc(alias = "__ZN3RBX15PhysicsSettings17setShowReceiveAgeEb")]
// 0x5f8cb4 — __ZN3RBX15PhysicsSettings17setShowReceiveAgeEb
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this, bool)
pub fn stub_0x5f8cb4() -> ! {
    todo!("0x5f8cb4 __ZN3RBX15PhysicsSettings17setShowReceiveAgeEb")
}

#[doc(alias = "RBX::PhysicsSettings::getShowContactPoints(void)const")]
#[doc(alias = "__ZNK3RBX15PhysicsSettings20getShowContactPointsEv")]
// 0x5f8ce4 — __ZNK3RBX15PhysicsSettings20getShowContactPointsEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
pub fn stub_0x5f8ce4() -> ! {
    todo!("0x5f8ce4 __ZNK3RBX15PhysicsSettings20getShowContactPointsEv")
}

#[doc(alias = "RBX::PhysicsSettings::setShowContactPoints(bool)")]
#[doc(alias = "__ZN3RBX15PhysicsSettings20setShowContactPointsEb")]
// 0x5f8cf4 — __ZN3RBX15PhysicsSettings20setShowContactPointsEb
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this, bool)
pub fn stub_0x5f8cf4() -> ! {
    todo!("0x5f8cf4 __ZN3RBX15PhysicsSettings20setShowContactPointsEb")
}

#[doc(alias = "RBX::PhysicsSettings::getShowJointCoordinates(void)const")]
#[doc(alias = "__ZNK3RBX15PhysicsSettings23getShowJointCoordinatesEv")]
// 0x5f8d24 — __ZNK3RBX15PhysicsSettings23getShowJointCoordinatesEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
pub fn stub_0x5f8d24() -> ! {
    todo!("0x5f8d24 __ZNK3RBX15PhysicsSettings23getShowJointCoordinatesEv")
}

#[doc(alias = "RBX::PhysicsSettings::setShowJointCoordinates(bool)")]
#[doc(alias = "__ZN3RBX15PhysicsSettings23setShowJointCoordinatesEb")]
// 0x5f8d34 — __ZN3RBX15PhysicsSettings23setShowJointCoordinatesEb
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this, bool)
pub fn stub_0x5f8d34() -> ! {
    todo!("0x5f8d34 __ZN3RBX15PhysicsSettings23setShowJointCoordinatesEb")
}

#[doc(alias = "RBX::PhysicsSettings::getShowMechanisms(void)const")]
#[doc(alias = "__ZNK3RBX15PhysicsSettings17getShowMechanismsEv")]
// 0x5f8d64 — __ZNK3RBX15PhysicsSettings17getShowMechanismsEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
pub fn stub_0x5f8d64() -> ! {
    todo!("0x5f8d64 __ZNK3RBX15PhysicsSettings17getShowMechanismsEv")
}

#[doc(alias = "RBX::PhysicsSettings::setShowMechanisms(bool)")]
#[doc(alias = "__ZN3RBX15PhysicsSettings17setShowMechanismsEb")]
// 0x5f8d74 — __ZN3RBX15PhysicsSettings17setShowMechanismsEb
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this, bool)
pub fn stub_0x5f8d74() -> ! {
    todo!("0x5f8d74 __ZN3RBX15PhysicsSettings17setShowMechanismsEb")
}

#[doc(alias = "RBX::PhysicsSettings::getShowAssemblies(void)const")]
#[doc(alias = "__ZNK3RBX15PhysicsSettings17getShowAssembliesEv")]
// 0x5f8da4 — __ZNK3RBX15PhysicsSettings17getShowAssembliesEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
pub fn stub_0x5f8da4() -> ! {
    todo!("0x5f8da4 __ZNK3RBX15PhysicsSettings17getShowAssembliesEv")
}

#[doc(alias = "RBX::PhysicsSettings::setShowAssemblies(bool)")]
#[doc(alias = "__ZN3RBX15PhysicsSettings17setShowAssembliesEb")]
// 0x5f8db4 — __ZN3RBX15PhysicsSettings17setShowAssembliesEb
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this, bool)
pub fn stub_0x5f8db4() -> ! {
    todo!("0x5f8db4 __ZN3RBX15PhysicsSettings17setShowAssembliesEb")
}

#[doc(alias = "RBX::PhysicsSettings::getShowSpanningTree(void)const")]
#[doc(alias = "__ZNK3RBX15PhysicsSettings19getShowSpanningTreeEv")]
// 0x5f8de4 — __ZNK3RBX15PhysicsSettings19getShowSpanningTreeEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
pub fn stub_0x5f8de4() -> ! {
    todo!("0x5f8de4 __ZNK3RBX15PhysicsSettings19getShowSpanningTreeEv")
}

#[doc(alias = "RBX::PhysicsSettings::setShowSpanningTree(bool)")]
#[doc(alias = "__ZN3RBX15PhysicsSettings19setShowSpanningTreeEb")]
// 0x5f8df4 — __ZN3RBX15PhysicsSettings19setShowSpanningTreeEb
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this, bool)
pub fn stub_0x5f8df4() -> ! {
    todo!("0x5f8df4 __ZN3RBX15PhysicsSettings19setShowSpanningTreeEb")
}

#[doc(alias = "RBX::PhysicsSettings::getAllowSleep(void)const")]
#[doc(alias = "__ZNK3RBX15PhysicsSettings13getAllowSleepEv")]
// 0x5f8e24 — __ZNK3RBX15PhysicsSettings13getAllowSleepEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
pub fn stub_0x5f8e24() -> ! {
    todo!("0x5f8e24 __ZNK3RBX15PhysicsSettings13getAllowSleepEv")
}

#[doc(alias = "RBX::PhysicsSettings::setAllowSleep(bool)")]
#[doc(alias = "__ZN3RBX15PhysicsSettings13setAllowSleepEb")]
// 0x5f8e34 — __ZN3RBX15PhysicsSettings13setAllowSleepEb
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this, bool)
pub fn stub_0x5f8e34() -> ! {
    todo!("0x5f8e34 __ZN3RBX15PhysicsSettings13setAllowSleepEb")
}

#[doc(alias = "RBX::PhysicsSettings::getParallelPhysics(void)const")]
#[doc(alias = "__ZNK3RBX15PhysicsSettings18getParallelPhysicsEv")]
// 0x5f8e64 — __ZNK3RBX15PhysicsSettings18getParallelPhysicsEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
pub fn stub_0x5f8e64() -> ! {
    todo!("0x5f8e64 __ZNK3RBX15PhysicsSettings18getParallelPhysicsEv")
}

#[doc(alias = "RBX::PhysicsSettings::setParallelPhysics(bool)")]
#[doc(alias = "__ZN3RBX15PhysicsSettings18setParallelPhysicsEb")]
// 0x5f8e74 — __ZN3RBX15PhysicsSettings18setParallelPhysicsEb
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this, bool)
pub fn stub_0x5f8e74() -> ! {
    todo!("0x5f8e74 __ZN3RBX15PhysicsSettings18setParallelPhysicsEb")
}

#[doc(alias = "RBX::PhysicsSettings::getEThrottle(void)const")]
#[doc(alias = "__ZNK3RBX15PhysicsSettings12getEThrottleEv")]
// 0x5f8ea4 — __ZNK3RBX15PhysicsSettings12getEThrottleEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
pub fn stub_0x5f8ea4() -> ! {
    todo!("0x5f8ea4 __ZNK3RBX15PhysicsSettings12getEThrottleEv")
}

#[doc(alias = "RBX::PhysicsSettings::setThrottleAdjustTime(double)")]
#[doc(alias = "__ZN3RBX15PhysicsSettings21setThrottleAdjustTimeEd")]
// 0x5f8edc — __ZN3RBX15PhysicsSettings21setThrottleAdjustTimeEd
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this, double)
pub fn stub_0x5f8edc() -> ! {
    todo!("0x5f8edc __ZN3RBX15PhysicsSettings21setThrottleAdjustTimeEd")
}

#[doc(alias = "RBX::PhysicsSettings::getThrottleAt30Fps(void)const")]
#[doc(alias = "__ZNK3RBX15PhysicsSettings18getThrottleAt30FpsEv")]
// 0x5f8f08 — __ZNK3RBX15PhysicsSettings18getThrottleAt30FpsEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
pub fn stub_0x5f8f08() -> ! {
    todo!("0x5f8f08 __ZNK3RBX15PhysicsSettings18getThrottleAt30FpsEv")
}

#[doc(alias = "RBX::PhysicsSettings::setThrottleAt30Fps(bool)")]
#[doc(alias = "__ZN3RBX15PhysicsSettings18setThrottleAt30FpsEb")]
// 0x5f8f18 — __ZN3RBX15PhysicsSettings18setThrottleAt30FpsEb
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this, bool)
pub fn stub_0x5f8f18() -> ! {
    todo!("0x5f8f18 __ZN3RBX15PhysicsSettings18setThrottleAt30FpsEb")
}

#[doc(alias = "RBX::PhysicsSettings::PhysicsSettings(void)")]
#[doc(alias = "__ZN3RBX15PhysicsSettingsC1Ev")]
// 0x5f8f38 — __ZN3RBX15PhysicsSettingsC1Ev
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
pub fn stub_0x5f8f38() -> ! {
    todo!("0x5f8f38 __ZN3RBX15PhysicsSettingsC1Ev")
}

#[doc(alias = "RBX::PhysicsSettings::PhysicsSettings(void)")]
#[doc(alias = "__ZN3RBX15PhysicsSettingsC2Ev")]
// 0x5f8f3c — __ZN3RBX15PhysicsSettingsC2Ev
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
pub fn stub_0x5f8f3c() -> ! {
    todo!("0x5f8f3c __ZN3RBX15PhysicsSettingsC2Ev")
}

#[doc(alias = "RBX::PhysicsSettings::getThrottleAdjustTime(void)const")]
#[doc(alias = "__ZNK3RBX15PhysicsSettings21getThrottleAdjustTimeEv")]
// 0x5f9150 — __ZNK3RBX15PhysicsSettings21getThrottleAdjustTimeEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
pub fn stub_0x5f9150() -> ! {
    todo!("0x5f9150 __ZNK3RBX15PhysicsSettings21getThrottleAdjustTimeEv")
}

#[doc(alias = "RBX::PhysicsSettings::~PhysicsSettings()")]
#[doc(alias = "__ZN3RBX15PhysicsSettingsD1Ev")]
// 0x5f93f0 — __ZN3RBX15PhysicsSettingsD1Ev
// type: void __fastcall(RBX::PhysicsSettings *__hidden this)
pub fn stub_0x5f93f0() -> ! {
    todo!("0x5f93f0 __ZN3RBX15PhysicsSettingsD1Ev")
}

#[doc(alias = "RBX::PhysicsSettings::~PhysicsSettings()")]
#[doc(alias = "__ZN3RBX15PhysicsSettingsD0Ev")]
// 0x5f9430 — __ZN3RBX15PhysicsSettingsD0Ev
// type: void __fastcall(RBX::PhysicsSettings *__hidden this)
pub fn stub_0x5f9430() -> ! {
    todo!("0x5f9430 __ZN3RBX15PhysicsSettingsD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::PhysicsSettings::~PhysicsSettings()")]
#[doc(alias = "__ZThn32_N3RBX15PhysicsSettingsD1Ev")]
// 0x5f9520 — __ZThn32_N3RBX15PhysicsSettingsD1Ev
// type: void __fastcall(RBX::PhysicsSettings *__hidden this)
pub fn stub_0x5f9520() -> ! {
    todo!("0x5f9520 __ZThn32_N3RBX15PhysicsSettingsD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::PhysicsSettings::~PhysicsSettings()")]
#[doc(alias = "__ZThn32_N3RBX15PhysicsSettingsD0Ev")]
// 0x5f9564 — __ZThn32_N3RBX15PhysicsSettingsD0Ev
// type: void __fastcall(RBX::PhysicsSettings *__hidden this)
pub fn stub_0x5f9564() -> ! {
    todo!("0x5f9564 __ZThn32_N3RBX15PhysicsSettingsD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::PhysicsSettings::~PhysicsSettings()")]
#[doc(alias = "__ZThn36_N3RBX15PhysicsSettingsD1Ev")]
// 0x5f9654 — __ZThn36_N3RBX15PhysicsSettingsD1Ev
// type: void __fastcall(RBX::PhysicsSettings *__hidden this)
pub fn stub_0x5f9654() -> ! {
    todo!("0x5f9654 __ZThn36_N3RBX15PhysicsSettingsD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::PhysicsSettings::~PhysicsSettings()")]
#[doc(alias = "__ZThn36_N3RBX15PhysicsSettingsD0Ev")]
// 0x5f9698 — __ZThn36_N3RBX15PhysicsSettingsD0Ev
// type: void __fastcall(RBX::PhysicsSettings *__hidden this)
pub fn stub_0x5f9698() -> ! {
    todo!("0x5f9698 __ZThn36_N3RBX15PhysicsSettingsD0Ev")
}

#[doc(alias = "RBX::BasePlayerGui::BasePlayerGui(void)")]
#[doc(alias = "__ZN3RBX13BasePlayerGuiC2Ev")]
// 0x5fb3d4 — __ZN3RBX13BasePlayerGuiC2Ev
// type: _DWORD __fastcall(RBX::BasePlayerGui *__hidden this)
pub fn stub_0x5fb3d4() -> ! {
    todo!("0x5fb3d4 __ZN3RBX13BasePlayerGuiC2Ev")
}

#[doc(alias = "RBX::BasePlayerGui::~BasePlayerGui()")]
#[doc(alias = "__ZN3RBX13BasePlayerGuiD0Ev")]
// 0x5fb5b4 — __ZN3RBX13BasePlayerGuiD0Ev
// type: void __fastcall(RBX::BasePlayerGui *__hidden this)
pub fn stub_0x5fb5b4() -> ! {
    todo!("0x5fb5b4 __ZN3RBX13BasePlayerGuiD0Ev")
}

#[doc(alias = "RBX::BasePlayerGui::~BasePlayerGui()")]
#[doc(alias = "__ZN3RBX13BasePlayerGuiD1Ev")]
// 0x5fb654 — __ZN3RBX13BasePlayerGuiD1Ev
// type: void __fastcall(RBX::BasePlayerGui *__hidden this)
pub fn stub_0x5fb654() -> ! {
    todo!("0x5fb654 __ZN3RBX13BasePlayerGuiD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::BasePlayerGui::~BasePlayerGui()")]
#[doc(alias = "__ZThn32_N3RBX13BasePlayerGuiD0Ev")]
// 0x5fb658 — __ZThn32_N3RBX13BasePlayerGuiD0Ev
// type: void __fastcall(RBX::BasePlayerGui *__hidden this)
pub fn stub_0x5fb658() -> ! {
    todo!("0x5fb658 __ZThn32_N3RBX13BasePlayerGuiD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::BasePlayerGui::~BasePlayerGui()")]
#[doc(alias = "__ZThn36_N3RBX13BasePlayerGuiD0Ev")]
// 0x5fb660 — __ZThn36_N3RBX13BasePlayerGuiD0Ev
// type: void __fastcall(RBX::BasePlayerGui *__hidden this)
pub fn stub_0x5fb660() -> ! {
    todo!("0x5fb660 __ZThn36_N3RBX13BasePlayerGuiD0Ev")
}

#[doc(alias = "RBX::BasePlayerGui::~BasePlayerGui()")]
#[doc(alias = "__ZN3RBX13BasePlayerGuiD2Ev")]
// 0x5fb668 — __ZN3RBX13BasePlayerGuiD2Ev
// type: void __fastcall(RBX::BasePlayerGui *__hidden this)
pub fn stub_0x5fb668() -> ! {
    todo!("0x5fb668 __ZN3RBX13BasePlayerGuiD2Ev")
}

#[doc(alias = "non-virtual thunk toRBX::BasePlayerGui::~BasePlayerGui()")]
#[doc(alias = "__ZThn32_N3RBX13BasePlayerGuiD1Ev")]
// 0x5fb7a4 — __ZThn32_N3RBX13BasePlayerGuiD1Ev
// type: void __fastcall(RBX::BasePlayerGui *__hidden this)
pub fn stub_0x5fb7a4() -> ! {
    todo!("0x5fb7a4 __ZThn32_N3RBX13BasePlayerGuiD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::BasePlayerGui::~BasePlayerGui()")]
#[doc(alias = "__ZThn36_N3RBX13BasePlayerGuiD1Ev")]
// 0x5fb7ac — __ZThn36_N3RBX13BasePlayerGuiD1Ev
// type: void __fastcall(RBX::BasePlayerGui *__hidden this)
pub fn stub_0x5fb7ac() -> ! {
    todo!("0x5fb7ac __ZThn36_N3RBX13BasePlayerGuiD1Ev")
}

#[doc(alias = "RBX::BasePlayerGui::findModalGuiObject(void)")]
#[doc(alias = "__ZN3RBX13BasePlayerGui18findModalGuiObjectEv")]
// 0x5fb7b4 — __ZN3RBX13BasePlayerGui18findModalGuiObjectEv
// type: _DWORD __fastcall(RBX::BasePlayerGui *__hidden this)
pub fn stub_0x5fb7b4() -> ! {
    todo!("0x5fb7b4 __ZN3RBX13BasePlayerGui18findModalGuiObjectEv")
}

#[doc(alias = "RBX::BasePlayerGui::render3dAdorn(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX13BasePlayerGui13render3dAdornEPNS_5AdornE")]
// 0x5fbb18 — __ZN3RBX13BasePlayerGui13render3dAdornEPNS_5AdornE
// type: _DWORD __fastcall(RBX::BasePlayerGui *__hidden this, RBX::Adorn *)
pub fn stub_0x5fbb18() -> ! {
    todo!("0x5fbb18 __ZN3RBX13BasePlayerGui13render3dAdornEPNS_5AdornE")
}

#[doc(alias = "RBX::BasePlayerGui::append3dSortedAdorn(std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>> &,RBX::Camera const*)const")]
#[doc(alias = "__ZNK3RBX13BasePlayerGui19append3dSortedAdornERSt6vectorIPNS_10IAdornableESaIS3_EEPKNS_6CameraE")]
// 0x5fbb20 — __ZNK3RBX13BasePlayerGui19append3dSortedAdornERSt6vectorIPNS_10IAdornableESaIS3_EEPKNS_6CameraE
pub fn stub_0x5fbb20() -> ! {
    todo!("0x5fbb20 __ZNK3RBX13BasePlayerGui19append3dSortedAdornERSt6vectorIPNS_10IAdornableESaIS3_EEPKNS_6CameraE")
}

#[doc(alias = "RBX::BasePlayerGui::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX13BasePlayerGui8render2dEPNS_5AdornE")]
// 0x5fbb28 — __ZN3RBX13BasePlayerGui8render2dEPNS_5AdornE
// type: _DWORD __fastcall(RBX::BasePlayerGui *__hidden this, RBX::Adorn *)
pub fn stub_0x5fbb28() -> ! {
    todo!("0x5fbb28 __ZN3RBX13BasePlayerGui8render2dEPNS_5AdornE")
}

#[doc(alias = "RBX::BasePlayerGui::process(RBX::GuiEvent const&)")]
#[doc(alias = "__ZN3RBX13BasePlayerGui7processERKNS_8GuiEventE")]
// 0x5fbb30 — __ZN3RBX13BasePlayerGui7processERKNS_8GuiEventE
pub fn stub_0x5fbb30() -> ! {
    todo!("0x5fbb30 __ZN3RBX13BasePlayerGui7processERKNS_8GuiEventE")
}

#[doc(alias = "non-virtual thunk toRBX::BasePlayerGui::process(RBX::GuiEvent const&)")]
#[doc(alias = "__ZThn96_N3RBX13BasePlayerGui7processERKNS_8GuiEventE")]
// 0x5fbcc4 — __ZThn96_N3RBX13BasePlayerGui7processERKNS_8GuiEventE
pub fn stub_0x5fbcc4() -> ! {
    todo!("0x5fbcc4 __ZThn96_N3RBX13BasePlayerGui7processERKNS_8GuiEventE")
}

#[doc(alias = "RBX::PlayerGui::PlayerGui(void)")]
#[doc(alias = "__ZN3RBX9PlayerGuiC1Ev")]
// 0x5fbcd0 — __ZN3RBX9PlayerGuiC1Ev
// type: _DWORD __fastcall(RBX::PlayerGui *__hidden this)
pub fn stub_0x5fbcd0() -> ! {
    todo!("0x5fbcd0 __ZN3RBX9PlayerGuiC1Ev")
}

#[doc(alias = "RBX::PlayerGui::PlayerGui(void)")]
#[doc(alias = "__ZN3RBX9PlayerGuiC2Ev")]
// 0x5fbcd4 — __ZN3RBX9PlayerGuiC2Ev
// type: _DWORD __fastcall(RBX::PlayerGui *__hidden this)
pub fn stub_0x5fbcd4() -> ! {
    todo!("0x5fbcd4 __ZN3RBX9PlayerGuiC2Ev")
}

#[doc(alias = "RBX::StarterGuiService::setShowGui(bool)")]
#[doc(alias = "__ZN3RBX17StarterGuiService10setShowGuiEb")]
// 0x5fbf2c — __ZN3RBX17StarterGuiService10setShowGuiEb
// type: _DWORD __fastcall(RBX::StarterGuiService *__hidden this, bool)
pub fn stub_0x5fbf2c() -> ! {
    todo!("0x5fbf2c __ZN3RBX17StarterGuiService10setShowGuiEb")
}

#[doc(alias = "RBX::StarterGuiService::setResetPlayerGui(bool)")]
#[doc(alias = "__ZN3RBX17StarterGuiService17setResetPlayerGuiEb")]
// 0x5fbf4c — __ZN3RBX17StarterGuiService17setResetPlayerGuiEb
// type: _DWORD __fastcall(RBX::StarterGuiService *__hidden this, bool)
pub fn stub_0x5fbf4c() -> ! {
    todo!("0x5fbf4c __ZN3RBX17StarterGuiService17setResetPlayerGuiEb")
}

#[doc(alias = "RBX::StarterGuiService::StarterGuiService(void)")]
#[doc(alias = "__ZN3RBX17StarterGuiServiceC1Ev")]
// 0x5fc44c — __ZN3RBX17StarterGuiServiceC1Ev
// type: _DWORD __fastcall(RBX::StarterGuiService *__hidden this)
pub fn stub_0x5fc44c() -> ! {
    todo!("0x5fc44c __ZN3RBX17StarterGuiServiceC1Ev")
}

#[doc(alias = "RBX::StarterGuiService::StarterGuiService(void)")]
#[doc(alias = "__ZN3RBX17StarterGuiServiceC2Ev")]
// 0x5fc450 — __ZN3RBX17StarterGuiServiceC2Ev
// type: _DWORD __fastcall(RBX::StarterGuiService *__hidden this)
pub fn stub_0x5fc450() -> ! {
    todo!("0x5fc450 __ZN3RBX17StarterGuiServiceC2Ev")
}

#[doc(alias = "RBX::StarterGuiService::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX17StarterGuiService8render2dEPNS_5AdornE")]
// 0x5fc754 — __ZN3RBX17StarterGuiService8render2dEPNS_5AdornE
// type: _DWORD __fastcall(RBX::StarterGuiService *__hidden this, RBX::Adorn *)
pub fn stub_0x5fc754() -> ! {
    todo!("0x5fc754 __ZN3RBX17StarterGuiService8render2dEPNS_5AdornE")
}

#[doc(alias = "RBX::StarterGuiService::render3dAdorn(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX17StarterGuiService13render3dAdornEPNS_5AdornE")]
// 0x5fc764 — __ZN3RBX17StarterGuiService13render3dAdornEPNS_5AdornE
// type: _DWORD __fastcall(RBX::StarterGuiService *__hidden this, RBX::Adorn *)
pub fn stub_0x5fc764() -> ! {
    todo!("0x5fc764 __ZN3RBX17StarterGuiService13render3dAdornEPNS_5AdornE")
}

#[doc(alias = "RBX::StarterGuiService::append3dSortedAdorn(std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>> &,RBX::Camera const*)const")]
#[doc(alias = "__ZNK3RBX17StarterGuiService19append3dSortedAdornERSt6vectorIPNS_10IAdornableESaIS3_EEPKNS_6CameraE")]
// 0x5fc774 — __ZNK3RBX17StarterGuiService19append3dSortedAdornERSt6vectorIPNS_10IAdornableESaIS3_EEPKNS_6CameraE
pub fn stub_0x5fc774() -> ! {
    todo!("0x5fc774 __ZNK3RBX17StarterGuiService19append3dSortedAdornERSt6vectorIPNS_10IAdornableESaIS3_EEPKNS_6CameraE")
}

#[doc(alias = "RBX::StarterGuiService::process(RBX::GuiEvent const&)")]
#[doc(alias = "__ZN3RBX17StarterGuiService7processERKNS_8GuiEventE")]
// 0x5fc784 — __ZN3RBX17StarterGuiService7processERKNS_8GuiEventE
pub fn stub_0x5fc784() -> ! {
    todo!("0x5fc784 __ZN3RBX17StarterGuiService7processERKNS_8GuiEventE")
}

#[doc(alias = "non-virtual thunk toRBX::StarterGuiService::process(RBX::GuiEvent const&)")]
#[doc(alias = "__ZThn96_N3RBX17StarterGuiService7processERKNS_8GuiEventE")]
// 0x5fc7a0 — __ZThn96_N3RBX17StarterGuiService7processERKNS_8GuiEventE
pub fn stub_0x5fc7a0() -> ! {
    todo!("0x5fc7a0 __ZThn96_N3RBX17StarterGuiService7processERKNS_8GuiEventE")
}

#[doc(alias = "RBX::CoreGuiService::getGuiVersion(void)const")]
#[doc(alias = "__ZNK3RBX14CoreGuiService13getGuiVersionEv")]
// 0x5fc7bc — __ZNK3RBX14CoreGuiService13getGuiVersionEv
// type: _DWORD __fastcall(RBX::CoreGuiService *__hidden this)
pub fn stub_0x5fc7bc() -> ! {
    todo!("0x5fc7bc __ZNK3RBX14CoreGuiService13getGuiVersionEv")
}

#[doc(alias = "RBX::CoreGuiService::CoreGuiService(void)")]
#[doc(alias = "__ZN3RBX14CoreGuiServiceC1Ev")]
// 0x5fc7c0 — __ZN3RBX14CoreGuiServiceC1Ev
// type: _DWORD __fastcall(RBX::CoreGuiService *__hidden this)
pub fn stub_0x5fc7c0() -> ! {
    todo!("0x5fc7c0 __ZN3RBX14CoreGuiServiceC1Ev")
}

#[doc(alias = "RBX::CoreGuiService::CoreGuiService(void)")]
#[doc(alias = "__ZN3RBX14CoreGuiServiceC2Ev")]
// 0x5fc7c4 — __ZN3RBX14CoreGuiServiceC2Ev
// type: _DWORD __fastcall(RBX::CoreGuiService *__hidden this)
pub fn stub_0x5fc7c4() -> ! {
    todo!("0x5fc7c4 __ZN3RBX14CoreGuiServiceC2Ev")
}

#[doc(alias = "RBX::CoreGuiService::createRobloxScreenGui(void)")]
#[doc(alias = "__ZN3RBX14CoreGuiService21createRobloxScreenGuiEv")]
// 0x5fca54 — __ZN3RBX14CoreGuiService21createRobloxScreenGuiEv
// type: _DWORD __fastcall(RBX::CoreGuiService *__hidden this)
pub fn stub_0x5fca54() -> ! {
    todo!("0x5fca54 __ZN3RBX14CoreGuiService21createRobloxScreenGuiEv")
}

#[doc(alias = "RBX::StarterGuiService::getShowGui(void)const")]
#[doc(alias = "__ZNK3RBX17StarterGuiService10getShowGuiEv")]
// 0x5fcc30 — __ZNK3RBX17StarterGuiService10getShowGuiEv
// type: _DWORD __fastcall(RBX::StarterGuiService *__hidden this)
pub fn stub_0x5fcc30() -> ! {
    todo!("0x5fcc30 __ZNK3RBX17StarterGuiService10getShowGuiEv")
}

#[doc(alias = "RBX::StarterGuiService::getResetPlayerGui(void)const")]
#[doc(alias = "__ZNK3RBX17StarterGuiService17getResetPlayerGuiEv")]
// 0x5fcc5c — __ZNK3RBX17StarterGuiService17getResetPlayerGuiEv
// type: _DWORD __fastcall(RBX::StarterGuiService *__hidden this)
pub fn stub_0x5fcc5c() -> ! {
    todo!("0x5fcc5c __ZNK3RBX17StarterGuiService17getResetPlayerGuiEv")
}

#[doc(alias = "RBX::PlayerGui::~PlayerGui()")]
#[doc(alias = "__ZN3RBX9PlayerGuiD1Ev")]
// 0x5fd5c8 — __ZN3RBX9PlayerGuiD1Ev
// type: void __fastcall(RBX::PlayerGui *__hidden this)
pub fn stub_0x5fd5c8() -> ! {
    todo!("0x5fd5c8 __ZN3RBX9PlayerGuiD1Ev")
}

#[doc(alias = "RBX::PlayerGui::~PlayerGui()")]
#[doc(alias = "__ZN3RBX9PlayerGuiD0Ev")]
// 0x5fd5cc — __ZN3RBX9PlayerGuiD0Ev
// type: void __fastcall(RBX::PlayerGui *__hidden this)
pub fn stub_0x5fd5cc() -> ! {
    todo!("0x5fd5cc __ZN3RBX9PlayerGuiD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::PlayerGui::~PlayerGui()")]
#[doc(alias = "__ZThn32_N3RBX9PlayerGuiD1Ev")]
// 0x5fd67c — __ZThn32_N3RBX9PlayerGuiD1Ev
// type: void __fastcall(RBX::PlayerGui *__hidden this)
pub fn stub_0x5fd67c() -> ! {
    todo!("0x5fd67c __ZThn32_N3RBX9PlayerGuiD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::PlayerGui::~PlayerGui()")]
#[doc(alias = "__ZThn32_N3RBX9PlayerGuiD0Ev")]
// 0x5fd684 — __ZThn32_N3RBX9PlayerGuiD0Ev
// type: void __fastcall(RBX::PlayerGui *__hidden this)
pub fn stub_0x5fd684() -> ! {
    todo!("0x5fd684 __ZThn32_N3RBX9PlayerGuiD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::PlayerGui::~PlayerGui()")]
#[doc(alias = "__ZThn36_N3RBX9PlayerGuiD1Ev")]
// 0x5fd738 — __ZThn36_N3RBX9PlayerGuiD1Ev
// type: void __fastcall(RBX::PlayerGui *__hidden this)
pub fn stub_0x5fd738() -> ! {
    todo!("0x5fd738 __ZThn36_N3RBX9PlayerGuiD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::PlayerGui::~PlayerGui()")]
#[doc(alias = "__ZThn36_N3RBX9PlayerGuiD0Ev")]
// 0x5fd740 — __ZThn36_N3RBX9PlayerGuiD0Ev
// type: void __fastcall(RBX::PlayerGui *__hidden this)
pub fn stub_0x5fd740() -> ! {
    todo!("0x5fd740 __ZThn36_N3RBX9PlayerGuiD0Ev")
}

#[doc(alias = "RBX::StarterGuiService::~StarterGuiService()")]
#[doc(alias = "__ZN3RBX17StarterGuiServiceD1Ev")]
// 0x5fd7e4 — __ZN3RBX17StarterGuiServiceD1Ev
// type: void __fastcall(RBX::StarterGuiService *__hidden this)
pub fn stub_0x5fd7e4() -> ! {
    todo!("0x5fd7e4 __ZN3RBX17StarterGuiServiceD1Ev")
}

#[doc(alias = "RBX::StarterGuiService::~StarterGuiService()")]
#[doc(alias = "__ZN3RBX17StarterGuiServiceD0Ev")]
// 0x5fd7e8 — __ZN3RBX17StarterGuiServiceD0Ev
// type: void __fastcall(RBX::StarterGuiService *__hidden this)
pub fn stub_0x5fd7e8() -> ! {
    todo!("0x5fd7e8 __ZN3RBX17StarterGuiServiceD0Ev")
}

#[doc(alias = "RBX::StarterGuiService::canClientCreate(void)")]
#[doc(alias = "__ZN3RBX17StarterGuiService15canClientCreateEv")]
// 0x5fd888 — __ZN3RBX17StarterGuiService15canClientCreateEv
// type: _DWORD __fastcall(RBX::StarterGuiService *__hidden this)
pub fn stub_0x5fd888() -> ! {
    todo!("0x5fd888 __ZN3RBX17StarterGuiService15canClientCreateEv")
}

#[doc(alias = "non-virtual thunk toRBX::StarterGuiService::~StarterGuiService()")]
#[doc(alias = "__ZThn32_N3RBX17StarterGuiServiceD1Ev")]
// 0x5fd8b8 — __ZThn32_N3RBX17StarterGuiServiceD1Ev
// type: void __fastcall(RBX::StarterGuiService *__hidden this)
pub fn stub_0x5fd8b8() -> ! {
    todo!("0x5fd8b8 __ZThn32_N3RBX17StarterGuiServiceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::StarterGuiService::~StarterGuiService()")]
#[doc(alias = "__ZThn32_N3RBX17StarterGuiServiceD0Ev")]
// 0x5fd8c0 — __ZThn32_N3RBX17StarterGuiServiceD0Ev
// type: void __fastcall(RBX::StarterGuiService *__hidden this)
pub fn stub_0x5fd8c0() -> ! {
    todo!("0x5fd8c0 __ZThn32_N3RBX17StarterGuiServiceD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::StarterGuiService::~StarterGuiService()")]
#[doc(alias = "__ZThn36_N3RBX17StarterGuiServiceD1Ev")]
// 0x5fd98c — __ZThn36_N3RBX17StarterGuiServiceD1Ev
// type: void __fastcall(RBX::StarterGuiService *__hidden this)
pub fn stub_0x5fd98c() -> ! {
    todo!("0x5fd98c __ZThn36_N3RBX17StarterGuiServiceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::StarterGuiService::~StarterGuiService()")]
#[doc(alias = "__ZThn36_N3RBX17StarterGuiServiceD0Ev")]
// 0x5fd994 — __ZThn36_N3RBX17StarterGuiServiceD0Ev
// type: void __fastcall(RBX::StarterGuiService *__hidden this)
pub fn stub_0x5fd994() -> ! {
    todo!("0x5fd994 __ZThn36_N3RBX17StarterGuiServiceD0Ev")
}

#[doc(alias = "RBX::CoreGuiService::~CoreGuiService()")]
#[doc(alias = "__ZN3RBX14CoreGuiServiceD1Ev")]
// 0x5fda3c — __ZN3RBX14CoreGuiServiceD1Ev
// type: void __fastcall(RBX::CoreGuiService *__hidden this)
pub fn stub_0x5fda3c() -> ! {
    todo!("0x5fda3c __ZN3RBX14CoreGuiServiceD1Ev")
}

#[doc(alias = "RBX::CoreGuiService::~CoreGuiService()")]
#[doc(alias = "__ZN3RBX14CoreGuiServiceD0Ev")]
// 0x5fdb58 — __ZN3RBX14CoreGuiServiceD0Ev
// type: void __fastcall(RBX::CoreGuiService *__hidden this)
pub fn stub_0x5fdb58() -> ! {
    todo!("0x5fdb58 __ZN3RBX14CoreGuiServiceD0Ev")
}

#[doc(alias = "RBX::CoreGuiService::canClientCreate(void)")]
#[doc(alias = "__ZN3RBX14CoreGuiService15canClientCreateEv")]
// 0x5fdc88 — __ZN3RBX14CoreGuiService15canClientCreateEv
// type: _DWORD __fastcall(RBX::CoreGuiService *__hidden this)
pub fn stub_0x5fdc88() -> ! {
    todo!("0x5fdc88 __ZN3RBX14CoreGuiService15canClientCreateEv")
}

#[doc(alias = "non-virtual thunk toRBX::CoreGuiService::~CoreGuiService()")]
#[doc(alias = "__ZThn32_N3RBX14CoreGuiServiceD1Ev")]
// 0x5fdcb8 — __ZThn32_N3RBX14CoreGuiServiceD1Ev
// type: void __fastcall(RBX::CoreGuiService *__hidden this)
pub fn stub_0x5fdcb8() -> ! {
    todo!("0x5fdcb8 __ZThn32_N3RBX14CoreGuiServiceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::CoreGuiService::~CoreGuiService()")]
#[doc(alias = "__ZThn32_N3RBX14CoreGuiServiceD0Ev")]
// 0x5fddd0 — __ZThn32_N3RBX14CoreGuiServiceD0Ev
// type: void __fastcall(RBX::CoreGuiService *__hidden this)
pub fn stub_0x5fddd0() -> ! {
    todo!("0x5fddd0 __ZThn32_N3RBX14CoreGuiServiceD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::CoreGuiService::~CoreGuiService()")]
#[doc(alias = "__ZThn36_N3RBX14CoreGuiServiceD1Ev")]
// 0x5fdf28 — __ZThn36_N3RBX14CoreGuiServiceD1Ev
// type: void __fastcall(RBX::CoreGuiService *__hidden this)
pub fn stub_0x5fdf28() -> ! {
    todo!("0x5fdf28 __ZThn36_N3RBX14CoreGuiServiceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::CoreGuiService::~CoreGuiService()")]
#[doc(alias = "__ZThn36_N3RBX14CoreGuiServiceD0Ev")]
// 0x5fe040 — __ZThn36_N3RBX14CoreGuiServiceD0Ev
// type: void __fastcall(RBX::CoreGuiService *__hidden this)
pub fn stub_0x5fe040() -> ! {
    todo!("0x5fe040 __ZThn36_N3RBX14CoreGuiServiceD0Ev")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::StarterGuiService::CoreGuiType>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX17StarterGuiService11CoreGuiTypeEE13destruct_funcEPc")]
// 0x5fe508 — __ZN3rbx14implementation12typed_holderIN3RBX17StarterGuiService11CoreGuiTypeEE13destruct_funcEPc
pub fn stub_0x5fe508() -> ! {
    todo!("0x5fe508 __ZN3rbx14implementation12typed_holderIN3RBX17StarterGuiService11CoreGuiTypeEE13destruct_funcEPc")
}

#[doc(alias = "RBX::StarterGuiService::CoreGuiType const& rbx::any_cast<RBX::StarterGuiService::CoreGuiType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX17StarterGuiService11CoreGuiTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0x5fe510 — __ZN3rbx8any_castIRKN3RBX17StarterGuiService11CoreGuiTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int(void)
pub fn stub_0x5fe510() -> ! {
    todo!("0x5fe510 __ZN3rbx8any_castIRKN3RBX17StarterGuiService11CoreGuiTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot> &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4nextERN5boost13intrusive_ptrINS6_4slotEEE")]
// 0x5ffefc — __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4nextERN5boost13intrusive_ptrINS6_4slotEEE
// type: int __fastcall(int, int *, int, int, char, int, int, int, int, int)
pub fn stub_0x5ffefc() -> ! {
    todo!("0x5ffefc __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4nextERN5boost13intrusive_ptrINS6_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::on_error(std::exception &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE8on_errorERSt9exception")]
// 0x60005c — __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE8on_errorERSt9exception
// type: int(void)
pub fn stub_0x60005c() -> ! {
    todo!("0x60005c __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE8on_errorERSt9exception")
}

