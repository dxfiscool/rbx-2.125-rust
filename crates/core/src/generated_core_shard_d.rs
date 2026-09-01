//! core shard D — 100 boost core stubs EA-sorted, continuation after shard C (0x6b24e8).
//! Source: `ida/export.json` filtered where mangled/demangled contains "boost" and crate core (Reflection/Instance/Ogre/RakNet/Network excluded), EA-sorted, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, single quotes removed.

#[doc(alias = "boost::function1<void,G3D::Vector3>::clear(void)")]
// 0x6b7f64 — __ZN5boost9function1IvN3G3D7Vector3EE5clearEv — boost::function1<void,G3D::Vector3>::clear(void)
pub fn stub_0x6b7f64() -> ! {
    todo!("0x6b7f64 __ZN5boost9function1IvN3G3D7Vector3EE5clearEv")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3)>::connect<boost::function<void ()(G3D::Vector3)>>(boost::function<void ()(G3D::Vector3)> const&)")]
// 0x6b8694 — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_ — rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3)>::connect<boost::function<void ()(G3D::Vector3)>>(boost::function<void ()(G3D::Vector3)> const&)
pub fn stub_0x6b8694() -> ! {
    todo!("0x6b8694 __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3)>::slot,boost::function<void ()(G3D::Vector3)>,1,void ()(G3D::Vector3)>::callable<rbx::signals::signal<void ()(G3D::Vector3)>*>(boost::function<void ()(G3D::Vector3)> const&,rbx::signals::signal<void ()(G3D::Vector3)>*)")]
// 0x6b8788 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector3EEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_ — rbx::callable<rbx::signals::signal<void ()(G3D::Vector3)>::slot,boost::function<void ()(G3D::Vector3)>,1,void ()(G3D::Vector3)>::callable<rbx::signals::signal<void ()(G3D::Vector3)>*>(boost::function<void ()(G3D::Vector3)> const&,rbx::signals::signal<void ()(G3D::Vector3)>*)
pub fn stub_0x6b8788() -> ! {
    todo!("0x6b8788 __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector3EEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_")
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::callable_slot<boost::function<void ()(G3D::Vector3)>>::~callable_slot()")]
// 0x6b8884 — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE13callable_slotIN5boost8functionIS4_EEED1Ev — rbx::signals::signal<void ()(G3D::Vector3)>::callable_slot<boost::function<void ()(G3D::Vector3)>>::~callable_slot()
pub fn stub_0x6b8884() -> ! {
    todo!("0x6b8884 __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE13callable_slotIN5boost8functionIS4_EEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::callable_slot<boost::function<void ()(G3D::Vector3)>>::~callable_slot()")]
// 0x6b8994 — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE13callable_slotIN5boost8functionIS4_EEED0Ev — rbx::signals::signal<void ()(G3D::Vector3)>::callable_slot<boost::function<void ()(G3D::Vector3)>>::~callable_slot()
pub fn stub_0x6b8994() -> ! {
    todo!("0x6b8994 __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE13callable_slotIN5boost8functionIS4_EEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3)>::slot,boost::function<void ()(G3D::Vector3)>,1,void ()(G3D::Vector3)>::call(G3D::Vector3)")]
// 0x6b8bd4 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector3EEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_ — rbx::callable<rbx::signals::signal<void ()(G3D::Vector3)>::slot,boost::function<void ()(G3D::Vector3)>,1,void ()(G3D::Vector3)>::call(G3D::Vector3)
pub fn stub_0x6b8bd4() -> ! {
    todo!("0x6b8bd4 __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector3EEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_")
}

#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(G3D::Vector3)>::slot,boost::function<void ()(G3D::Vector3)>,1,void ()(G3D::Vector3)>::call(G3D::Vector3)")]
// 0x6b8bdc — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D7Vector3EEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_ — `non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(G3D::Vector3)>::slot,boost::function<void ()(G3D::Vector3)>,1,void ()(G3D::Vector3)>::call(G3D::Vector3)
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(G3D::Vector3)>::slot,boost::function<void ()(G3D::Vector3)>,1,void ()(G3D::Vector3)>::call(G3D::Vector3)
pub fn stub_0x6b8bdc() -> ! {
    todo!("0x6b8bdc __ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D7Vector3EEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_")
}

#[doc(alias = "boost::function1<void,G3D::Vector3>::operator()(G3D::Vector3)const")]
// 0x6b8be4 — __ZNK5boost9function1IvN3G3D7Vector3EEclES2_ — boost::function1<void,G3D::Vector3>::operator()(G3D::Vector3)const
pub fn stub_0x6b8be4() -> ! {
    todo!("0x6b8be4 __ZNK5boost9function1IvN3G3D7Vector3EEclES2_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3)>::slot,boost::function<void ()(G3D::Vector3)>,1,void ()(G3D::Vector3)>::~callable()")]
// 0x6b8e90 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector3EEE4slotEN5boost8functionIS5_EELi1ES5_ED1Ev — rbx::callable<rbx::signals::signal<void ()(G3D::Vector3)>::slot,boost::function<void ()(G3D::Vector3)>,1,void ()(G3D::Vector3)>::~callable()
pub fn stub_0x6b8e90() -> ! {
    todo!("0x6b8e90 __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector3EEE4slotEN5boost8functionIS5_EELi1ES5_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3)>::slot,boost::function<void ()(G3D::Vector3)>,1,void ()(G3D::Vector3)>::~callable()")]
// 0x6b8fa0 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector3EEE4slotEN5boost8functionIS5_EELi1ES5_ED0Ev — rbx::callable<rbx::signals::signal<void ()(G3D::Vector3)>::slot,boost::function<void ()(G3D::Vector3)>,1,void ()(G3D::Vector3)>::~callable()
pub fn stub_0x6b8fa0() -> ! {
    todo!("0x6b8fa0 __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector3EEE4slotEN5boost8functionIS5_EELi1ES5_ED0Ev")
}

#[doc(alias = "boost::function1<void,G3D::Vector3>::assign_to_own(boost::function1<void,G3D::Vector3> const&)")]
// 0x6b90fc — __ZN5boost9function1IvN3G3D7Vector3EE13assign_to_ownERKS3_ — boost::function1<void,G3D::Vector3>::assign_to_own(boost::function1<void,G3D::Vector3> const&)
pub fn stub_0x6b90fc() -> ! {
    todo!("0x6b90fc __ZN5boost9function1IvN3G3D7Vector3EE13assign_to_ownERKS3_")
}

#[doc(alias = "void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,void (*)(RBX::Primitive *,std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>>>>>(boost::_bi::bind_t<void,void (*)(RBX::Primitive *,std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>>>>,RBX::Primitive *)")]
// 0x6c01bc — __ZN3RBX8Assembly19visitPrimitivesImplIN5boost3_bi6bind_tIvPFvPNS_9PrimitiveERSt6vectorIPKS5_SaIS9_EEENS3_5list2INS2_3argILi1EEENS2_17reference_wrapperISB_EEEEEEEEvT_S6_ — void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,void (*)(RBX::Primitive *,std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>>>>>(boost::_bi::bind_t<void,void (*)(RBX::Primitive *,std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>>>>,RBX::Primitive *)
pub fn stub_0x6c01bc() -> ! {
    todo!("0x6c01bc __ZN3RBX8Assembly19visitPrimitivesImplIN5boost3_bi6bind_tIvPFvPNS_9PrimitiveERSt6vectorIPKS5_SaIS9_EEENS3_5list2INS2_3argILi1EEENS2_17reference_wrapperISB_EEEEEEEEvT_S6_")
}

#[doc(alias = "void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VehicleSeat,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::VehicleSeat*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VehicleSeat,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::VehicleSeat*>,boost::arg<1>>>,RBX::Primitive *)")]
// 0x6c04ac — __ZN3RBX8Assembly19visitPrimitivesImplIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_11VehicleSeatEPNS_9PrimitiveEEENS3_5list2INS3_5valueIPS7_EENS2_3argILi1EEEEEEEEEvT_S9_ — void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VehicleSeat,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::VehicleSeat*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VehicleSeat,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::VehicleSeat*>,boost::arg<1>>>,RBX::Primitive *)
pub fn stub_0x6c04ac() -> ! {
    todo!("0x6c04ac __ZN3RBX8Assembly19visitPrimitivesImplIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_11VehicleSeatEPNS_9PrimitiveEEENS3_5list2INS3_5valueIPS7_EENS2_3argILi1EEEEEEEEEvT_S9_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::VehicleController>::shared_ptr<RBX::VehicleController>(rbx_core::WeakPtr<RBX::VehicleController> const&,boost::detail::sp_nothrow_tag)")]
// 0x6c13fc — __ZN5boost10shared_ptrIN3RBX17VehicleControllerEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE — rbx_core::SharedPtr<RBX::VehicleController>::shared_ptr<RBX::VehicleController>(rbx_core::WeakPtr<RBX::VehicleController> const&,boost::detail::sp_nothrow_tag)
// was: boost::shared_ptr<RBX::VehicleController>::shared_ptr<RBX::VehicleController>(boost::weak_ptr<RBX::VehicleController> const&,boost::detail::sp_nothrow_tag)
pub fn stub_0x6c13fc() -> ! {
    todo!("0x6c13fc __ZN5boost10shared_ptrIN3RBX17VehicleControllerEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::UIEvent const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VirtualUser,RBX::UIEvent const&>,boost::_bi::list2<boost::_bi::value<RBX::VirtualUser*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VirtualUser,RBX::UIEvent const&>,boost::_bi::list2<boost::_bi::value<RBX::VirtualUser*>,boost::arg<1>>> const&)")]
// 0x6c4ab4 — __ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE7connectIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_11VirtualUserES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEEENS0_10connectionERKT_ — rbx::signals::connection rbx::signals::signal<void ()(RBX::UIEvent const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VirtualUser,RBX::UIEvent const&>,boost::_bi::list2<boost::_bi::value<RBX::VirtualUser*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VirtualUser,RBX::UIEvent const&>,boost::_bi::list2<boost::_bi::value<RBX::VirtualUser*>,boost::arg<1>>> const&)
pub fn stub_0x6c4ab4() -> ! {
    todo!("0x6c4ab4 __ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE7connectIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_11VirtualUserES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot*)")]
// 0x6c5e84 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX7UIEventEEE4slotEEaSEPSA_ — boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot*)
pub fn stub_0x6c5e84() -> ! {
    todo!("0x6c5e84 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX7UIEventEEE4slotEEaSEPSA_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UIEvent const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VirtualUser,RBX::UIEvent const&>,boost::_bi::list2<boost::_bi::value<RBX::VirtualUser*>,boost::arg<1>>>>::~callable_slot()")]
// 0x6c5ea8 — __ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_11VirtualUserES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED1Ev — rbx::signals::signal<void ()(RBX::UIEvent const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VirtualUser,RBX::UIEvent const&>,boost::_bi::list2<boost::_bi::value<RBX::VirtualUser*>,boost::arg<1>>>>::~callable_slot()
pub fn stub_0x6c5ea8() -> ! {
    todo!("0x6c5ea8 __ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_11VirtualUserES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UIEvent const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VirtualUser,RBX::UIEvent const&>,boost::_bi::list2<boost::_bi::value<RBX::VirtualUser*>,boost::arg<1>>>>::~callable_slot()")]
// 0x6c5ed4 — __ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_11VirtualUserES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED0Ev — rbx::signals::signal<void ()(RBX::UIEvent const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VirtualUser,RBX::UIEvent const&>,boost::_bi::list2<boost::_bi::value<RBX::VirtualUser*>,boost::arg<1>>>>::~callable_slot()
pub fn stub_0x6c5ed4() -> ! {
    todo!("0x6c5ed4 __ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_11VirtualUserES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VirtualUser,RBX::UIEvent const&>,boost::_bi::list2<boost::_bi::value<RBX::VirtualUser*>,boost::arg<1>>>,1,void ()(RBX::UIEvent const&)>::call(RBX::UIEvent const&)")]
// 0x6c60c4 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7UIEventEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_11VirtualUserES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_ — rbx::callable<rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VirtualUser,RBX::UIEvent const&>,boost::_bi::list2<boost::_bi::value<RBX::VirtualUser*>,boost::arg<1>>>,1,void ()(RBX::UIEvent const&)>::call(RBX::UIEvent const&)
pub fn stub_0x6c60c4() -> ! {
    todo!("0x6c60c4 __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7UIEventEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_11VirtualUserES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_")
}

#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VirtualUser,RBX::UIEvent const&>,boost::_bi::list2<boost::_bi::value<RBX::VirtualUser*>,boost::arg<1>>>,1,void ()(RBX::UIEvent const&)>::call(RBX::UIEvent const&)")]
// 0x6c60cc — __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX7UIEventEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_11VirtualUserES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_ — `non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VirtualUser,RBX::UIEvent const&>,boost::_bi::list2<boost::_bi::value<RBX::VirtualUser*>,boost::arg<1>>>,1,void ()(RBX::UIEvent const&)>::call(RBX::UIEvent const&)
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VirtualUser,RBX::UIEvent const&>,boost::_bi::list2<boost::_bi::value<RBX::VirtualUser*>,boost::arg<1>>>,1,void ()(RBX::UIEvent const&)>::call(RBX::UIEvent const&)
pub fn stub_0x6c60cc() -> ! {
    todo!("0x6c60cc __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX7UIEventEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_11VirtualUserES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_")
}

#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VirtualUser,RBX::UIEvent const&>,boost::_bi::list2<boost::_bi::value<RBX::VirtualUser*>,boost::arg<1>>>::operator()<RBX::UIEvent>(RBX::UIEvent const&)")]
// 0x6c60d4 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX11VirtualUserERKNS4_7UIEventEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_ — void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VirtualUser,RBX::UIEvent const&>,boost::_bi::list2<boost::_bi::value<RBX::VirtualUser*>,boost::arg<1>>>::operator()<RBX::UIEvent>(RBX::UIEvent const&)
pub fn stub_0x6c60d4() -> ! {
    todo!("0x6c60d4 __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX11VirtualUserERKNS4_7UIEventEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VirtualUser,RBX::UIEvent const&>,boost::_bi::list2<boost::_bi::value<RBX::VirtualUser*>,boost::arg<1>>>,1,void ()(RBX::UIEvent const&)>::~callable()")]
// 0x6c63d0 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7UIEventEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_11VirtualUserES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED1Ev — rbx::callable<rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VirtualUser,RBX::UIEvent const&>,boost::_bi::list2<boost::_bi::value<RBX::VirtualUser*>,boost::arg<1>>>,1,void ()(RBX::UIEvent const&)>::~callable()
pub fn stub_0x6c63d0() -> ! {
    todo!("0x6c63d0 __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7UIEventEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_11VirtualUserES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VirtualUser,RBX::UIEvent const&>,boost::_bi::list2<boost::_bi::value<RBX::VirtualUser*>,boost::arg<1>>>,1,void ()(RBX::UIEvent const&)>::~callable()")]
// 0x6c63fc — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7UIEventEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_11VirtualUserES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED0Ev — rbx::callable<rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VirtualUser,RBX::UIEvent const&>,boost::_bi::list2<boost::_bi::value<RBX::VirtualUser*>,boost::arg<1>>>,1,void ()(RBX::UIEvent const&)>::~callable()
pub fn stub_0x6c63fc() -> ! {
    todo!("0x6c63fc __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7UIEventEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_11VirtualUserES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED0Ev")
}

#[doc(alias = "boost::scoped_ptr<RBX::VirtualHardwareDevice>::~scoped_ptr()")]
// 0x6c64d0 — __ZN5boost10scoped_ptrIN3RBX21VirtualHardwareDeviceEED2Ev — boost::scoped_ptr<RBX::VirtualHardwareDevice>::~scoped_ptr()
pub fn stub_0x6c64d0() -> ! {
    todo!("0x6c64d0 __ZN5boost10scoped_ptrIN3RBX21VirtualHardwareDeviceEED2Ev")
}

#[doc(alias = "boost::xtime::operator boost::posix_time::ptime(void)const")]
// 0x6c8b2c — __ZNK5boost5xtimecvNS_10posix_time5ptimeEEv — boost::xtime::operator boost::posix_time::ptime(void)const
pub fn stub_0x6c8b2c() -> ! {
    todo!("0x6c8b2c __ZNK5boost5xtimecvNS_10posix_time5ptimeEEv")
}

#[doc(alias = "boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(std::string,int),boost::_bi::list_av_2<std::string,int>::type> boost::bind<RBX::worker_thread::work_result,std::string,int,std::string,int>(RBX::worker_thread::work_result (*)(std::string,int),std::string,int)")]
// 0x6c8bc0 — __ZN5boost4bindIN3RBX13worker_thread11work_resultESsiSsiEENS_3_bi6bind_tIT_PFS6_T0_T1_ENS4_9list_av_2IT2_T3_E4typeEEESA_SC_SD_ — boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(std::string,int),boost::_bi::list_av_2<std::string,int>::type> boost::bind<RBX::worker_thread::work_result,std::string,int,std::string,int>(RBX::worker_thread::work_result (*)(std::string,int),std::string,int)
pub fn stub_0x6c8bc0() -> ! {
    todo!("0x6c8bc0 __ZN5boost4bindIN3RBX13worker_thread11work_resultESsiSsiEENS_3_bi6bind_tIT_PFS6_T0_T1_ENS4_9list_av_2IT2_T3_E4typeEEESA_SC_SD_")
}

#[doc(alias = "boost::function0<RBX::worker_thread::work_result>::clear(void)")]
// 0x6c8d90 — __ZN5boost9function0IN3RBX13worker_thread11work_resultEE5clearEv — boost::function0<RBX::worker_thread::work_result>::clear(void)
pub fn stub_0x6c8d90() -> ! {
    todo!("0x6c8d90 __ZN5boost9function0IN3RBX13worker_thread11work_resultEE5clearEv")
}

#[doc(alias = "__ZN5boost9function0IN3RBX13worker_thread11work_resultEEC2INS_3_bi6bind_tIS3_PFS3_SsiENS6_5list2INS6_5valueISsEENSB_IiEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISG_EE5valueEEE5valueEiE4typeE")]
// 0x6c8dbc — __ZN5boost9function0IN3RBX13worker_thread11work_resultEEC2INS_3_bi6bind_tIS3_PFS3_SsiENS6_5list2INS6_5valueISsEENSB_IiEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISG_EE5valueEEE5valueEiE4typeE — __ZN5boost9function0IN3RBX13worker_thread11work_resultEEC2INS_3_bi6bind_tIS3_PFS3_SsiENS6_5list2INS6_5valueISsEENSB_IiEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISG_EE5valueEEE5valueEiE4typeE
pub fn stub_0x6c8dbc() -> ! {
    todo!("0x6c8dbc __ZN5boost9function0IN3RBX13worker_thread11work_resultEEC2INS_3_bi6bind_tIS3_PFS3_SsiENS6_5list2INS6_5valueISsEENSB_IiEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISG_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "void boost::function0<RBX::worker_thread::work_result>::assign_to<boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(std::string,int),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<int>>>>(boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(std::string,int),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<int>>>)")]
// 0x6c8eec — __ZN5boost9function0IN3RBX13worker_thread11work_resultEE9assign_toINS_3_bi6bind_tIS3_PFS3_SsiENS6_5list2INS6_5valueISsEENSB_IiEEEEEEEEvT_ — void boost::function0<RBX::worker_thread::work_result>::assign_to<boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(std::string,int),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<int>>>>(boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(std::string,int),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<int>>>)
pub fn stub_0x6c8eec() -> ! {
    todo!("0x6c8eec __ZN5boost9function0IN3RBX13worker_thread11work_resultEE9assign_toINS_3_bi6bind_tIS3_PFS3_SsiENS6_5list2INS6_5valueISsEENSB_IiEEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(std::string,int),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<int>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x6c902c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIN3RBX13worker_thread11work_resultEPFS7_SsiENS3_5list2INS3_5valueISsEENSB_IiEEEEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE — boost::detail::function::functor_manager<boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(std::string,int),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<int>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0x6c902c() -> ! {
    todo!("0x6c902c __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIN3RBX13worker_thread11work_resultEPFS7_SsiENS3_5list2INS3_5valueISsEENSB_IiEEEEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(std::string,int),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<int>>>,RBX::worker_thread::work_result>::invoke(boost::detail::function::function_buffer &)")]
// 0x6c90ac — __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIN3RBX13worker_thread11work_resultEPFS7_SsiENS3_5list2INS3_5valueISsEENSB_IiEEEEEES7_E6invokeERNS1_15function_bufferE — boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(std::string,int),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<int>>>,RBX::worker_thread::work_result>::invoke(boost::detail::function::function_buffer &)
pub fn stub_0x6c90ac() -> ! {
    todo!("0x6c90ac __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIN3RBX13worker_thread11work_resultEPFS7_SsiENS3_5list2INS3_5valueISsEENSB_IiEEEEEES7_E6invokeERNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<RBX::worker_thread::work_result>::assign_to<boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(std::string,int),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<int>>>>(boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(std::string,int),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<int>>>,boost::detail::function::function_buffer &)const")]
// 0x6c90c0 — __ZNK5boost6detail8function13basic_vtable0IN3RBX13worker_thread11work_resultEE9assign_toINS_3_bi6bind_tIS5_PFS5_SsiENS8_5list2INS8_5valueISsEENSD_IiEEEEEEEEbT_RNS1_15function_bufferE — bool boost::detail::function::basic_vtable0<RBX::worker_thread::work_result>::assign_to<boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(std::string,int),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<int>>>>(boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(std::string,int),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<int>>>,boost::detail::function::function_buffer &)const
pub fn stub_0x6c90c0() -> ! {
    todo!("0x6c90c0 __ZNK5boost6detail8function13basic_vtable0IN3RBX13worker_thread11work_resultEE9assign_toINS_3_bi6bind_tIS5_PFS5_SsiENS8_5list2INS8_5valueISsEENSD_IiEEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<RBX::worker_thread::work_result>::assign_to<boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(std::string,int),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<int>>>>(boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(std::string,int),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<int>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x6c91f4 — __ZNK5boost6detail8function13basic_vtable0IN3RBX13worker_thread11work_resultEE9assign_toINS_3_bi6bind_tIS5_PFS5_SsiENS8_5list2INS8_5valueISsEENSD_IiEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE — bool boost::detail::function::basic_vtable0<RBX::worker_thread::work_result>::assign_to<boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(std::string,int),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<int>>>>(boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(std::string,int),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<int>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0x6c91f4() -> ! {
    todo!("0x6c91f4 __ZNK5boost6detail8function13basic_vtable0IN3RBX13worker_thread11work_resultEE9assign_toINS_3_bi6bind_tIS5_PFS5_SsiENS8_5list2INS8_5valueISsEENSD_IiEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::detail::function::basic_vtable0<RBX::worker_thread::work_result>::assign_functor<boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(std::string,int),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<int>>>>(boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(std::string,int),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<int>>>,boost::detail::function::function_buffer &,mpl_::bool_<true>)const")]
// 0x6c9320 — __ZNK5boost6detail8function13basic_vtable0IN3RBX13worker_thread11work_resultEE14assign_functorINS_3_bi6bind_tIS5_PFS5_SsiENS8_5list2INS8_5valueISsEENSD_IiEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb1EEE — void boost::detail::function::basic_vtable0<RBX::worker_thread::work_result>::assign_functor<boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(std::string,int),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<int>>>>(boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(std::string,int),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<int>>>,boost::detail::function::function_buffer &,mpl_::bool_<true>)const
pub fn stub_0x6c9320() -> ! {
    todo!("0x6c9320 __ZNK5boost6detail8function13basic_vtable0IN3RBX13worker_thread11work_resultEE14assign_functorINS_3_bi6bind_tIS5_PFS5_SsiENS8_5list2INS8_5valueISsEENSD_IiEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb1EEE")
}

#[doc(alias = "RBX::worker_thread::work_result boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<int>>::operator()<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(std::string,int),boost::_bi::list0>(boost::_bi::type<RBX::worker_thread::work_result>,RBX::worker_thread::work_result (*)(std::string,int) &,boost::_bi::list0 &,long)")]
// 0x6c9344 — __ZN5boost3_bi5list2INS0_5valueISsEENS2_IiEEEclIN3RBX13worker_thread11work_resultEPFS9_SsiENS0_5list0EEET_NS0_4typeISD_EERT0_RT1_l — RBX::worker_thread::work_result boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<int>>::operator()<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(std::string,int),boost::_bi::list0>(boost::_bi::type<RBX::worker_thread::work_result>,RBX::worker_thread::work_result (*)(std::string,int) &,boost::_bi::list0 &,long)
pub fn stub_0x6c9344() -> ! {
    todo!("0x6c9344 __ZN5boost3_bi5list2INS0_5valueISsEENS2_IiEEEclIN3RBX13worker_thread11work_resultEPFS9_SsiENS0_5list0EEET_NS0_4typeISD_EERT0_RT1_l")
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<int>>::list2(boost::_bi::value<std::string>,boost::_bi::value<int>)")]
// 0x6c9464 — __ZN5boost3_bi5list2INS0_5valueISsEENS2_IiEEEC2ES3_S4_ — boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<int>>::list2(boost::_bi::value<std::string>,boost::_bi::value<int>)
pub fn stub_0x6c9464() -> ! {
    todo!("0x6c9464 __ZN5boost3_bi5list2INS0_5valueISsEENS2_IiEEEC2ES3_S4_")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<std::string>,boost::_bi::value<int>>::storage2(boost::_bi::value<std::string>,boost::_bi::value<int>)")]
// 0x6c9584 — __ZN5boost3_bi8storage2INS0_5valueISsEENS2_IiEEEC2ES3_S4_ — boost::_bi::storage2<boost::_bi::value<std::string>,boost::_bi::value<int>>::storage2(boost::_bi::value<std::string>,boost::_bi::value<int>)
pub fn stub_0x6c9584() -> ! {
    todo!("0x6c9584 __ZN5boost3_bi8storage2INS0_5valueISsEENS2_IiEEEC2ES3_S4_")
}

#[doc(alias = "boost::scoped_ptr<RBX::worker_thread>::~scoped_ptr()")]
// 0x6c96a8 — __ZN5boost10scoped_ptrIN3RBX13worker_threadEED2Ev — boost::scoped_ptr<RBX::worker_thread>::~scoped_ptr()
pub fn stub_0x6c96a8() -> ! {
    todo!("0x6c96a8 __ZN5boost10scoped_ptrIN3RBX13worker_threadEED2Ev")
}

#[doc(alias = "RBX::Workspace::setMouseCommand(rbx_core::SharedPtr<RBX::MouseCommand>)")]
// 0x6ccf30 — __ZN3RBX9Workspace15setMouseCommandEN5boost10shared_ptrINS_12MouseCommandEEE — RBX::Workspace::setMouseCommand(rbx_core::SharedPtr<RBX::MouseCommand>)
// was: RBX::Workspace::setMouseCommand(boost::shared_ptr<RBX::MouseCommand>)
pub fn stub_0x6ccf30() -> ! {
    todo!("0x6ccf30 __ZN3RBX9Workspace15setMouseCommandEN5boost10shared_ptrINS_12MouseCommandEEE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::DecalTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::DecalTool,RBX::Workspace *,RBX::Decal *>(RBX::Workspace *,RBX::Decal *)")]
// 0x6d15a8 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_9DecalToolEPNS_9WorkspaceEPNS_5DecalEEEN5boost10shared_ptrIT_EET0_T1_ — rbx_core::SharedPtr<RBX::DecalTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::DecalTool,RBX::Workspace *,RBX::Decal *>(RBX::Workspace *,RBX::Decal *)
// was: boost::shared_ptr<RBX::DecalTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::DecalTool,RBX::Workspace *,RBX::Decal *>(RBX::Workspace *,RBX::Decal *)
pub fn stub_0x6d15a8() -> ! {
    todo!("0x6d15a8 __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_9DecalToolEPNS_9WorkspaceEPNS_5DecalEEEN5boost10shared_ptrIT_EET0_T1_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Camera> RBX::shared_from<RBX::Camera>(RBX::Camera*)")]
// 0x6d168c — __ZN3RBX11shared_fromINS_6CameraEEEN5boost10shared_ptrIT_EEPS4_ — rbx_core::SharedPtr<RBX::Camera> RBX::shared_from<RBX::Camera>(RBX::Camera*)
// was: boost::shared_ptr<RBX::Camera> RBX::shared_from<RBX::Camera>(RBX::Camera*)
pub fn stub_0x6d168c() -> ! {
    todo!("0x6d168c __ZN3RBX11shared_fromINS_6CameraEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Camera>::operator=(rbx_core::SharedPtr<RBX::Camera> const&)")]
// 0x6d1774 — __ZN5boost10shared_ptrIN3RBX6CameraEEaSERKS3_ — rbx_core::SharedPtr<RBX::Camera>::operator=(rbx_core::SharedPtr<RBX::Camera> const&)
// was: boost::shared_ptr<RBX::Camera>::operator=(boost::shared_ptr<RBX::Camera> const&)
pub fn stub_0x6d1774() -> ! {
    todo!("0x6d1774 __ZN5boost10shared_ptrIN3RBX6CameraEEaSERKS3_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::NewNullTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::NewNullTool,RBX::Workspace *>(RBX::Workspace *)")]
// 0x6d1fe0 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_11NewNullToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_ — rbx_core::SharedPtr<RBX::NewNullTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::NewNullTool,RBX::Workspace *>(RBX::Workspace *)
// was: boost::shared_ptr<RBX::NewNullTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::NewNullTool,RBX::Workspace *>(RBX::Workspace *)
pub fn stub_0x6d1fe0() -> ! {
    todo!("0x6d1fe0 __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_11NewNullToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::MouseCommand>::operator=(rbx_core::SharedPtr<RBX::MouseCommand> const&)")]
// 0x6d2094 — __ZN5boost10shared_ptrIN3RBX12MouseCommandEEaSERKS3_ — rbx_core::SharedPtr<RBX::MouseCommand>::operator=(rbx_core::SharedPtr<RBX::MouseCommand> const&)
// was: boost::shared_ptr<RBX::MouseCommand>::operator=(boost::shared_ptr<RBX::MouseCommand> const&)
pub fn stub_0x6d2094() -> ! {
    todo!("0x6d2094 __ZN5boost10shared_ptrIN3RBX12MouseCommandEEaSERKS3_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::MouseCommand>& rbx_core::SharedPtr<RBX::MouseCommand>::operator=<RBX::AdvArrowTool>(rbx_core::SharedPtr<RBX::AdvArrowTool> const&)")]
// 0x6d20cc — __ZN5boost10shared_ptrIN3RBX12MouseCommandEEaSINS1_12AdvArrowToolEEERS3_RKNS0_IT_EE — rbx_core::SharedPtr<RBX::MouseCommand>& rbx_core::SharedPtr<RBX::MouseCommand>::operator=<RBX::AdvArrowTool>(rbx_core::SharedPtr<RBX::AdvArrowTool> const&)
// was: boost::shared_ptr<RBX::MouseCommand>& boost::shared_ptr<RBX::MouseCommand>::operator=<RBX::AdvArrowTool>(boost::shared_ptr<RBX::AdvArrowTool> const&)
pub fn stub_0x6d20cc() -> ! {
    todo!("0x6d20cc __ZN5boost10shared_ptrIN3RBX12MouseCommandEEaSINS1_12AdvArrowToolEEERS3_RKNS0_IT_EE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AdvArrowTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::AdvArrowTool,RBX::Workspace *>(RBX::Workspace *)")]
// 0x6d2100 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_12AdvArrowToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_ — rbx_core::SharedPtr<RBX::AdvArrowTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::AdvArrowTool,RBX::Workspace *>(RBX::Workspace *)
// was: boost::shared_ptr<RBX::AdvArrowTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::AdvArrowTool,RBX::Workspace *>(RBX::Workspace *)
pub fn stub_0x6d2100() -> ! {
    todo!("0x6d2100 __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_12AdvArrowToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::MouseCommand>& rbx_core::SharedPtr<RBX::MouseCommand>::operator=<RBX::NewNullTool>(rbx_core::SharedPtr<RBX::NewNullTool> const&)")]
// 0x6d21d8 — __ZN5boost10shared_ptrIN3RBX12MouseCommandEEaSINS1_11NewNullToolEEERS3_RKNS0_IT_EE — rbx_core::SharedPtr<RBX::MouseCommand>& rbx_core::SharedPtr<RBX::MouseCommand>::operator=<RBX::NewNullTool>(rbx_core::SharedPtr<RBX::NewNullTool> const&)
// was: boost::shared_ptr<RBX::MouseCommand>& boost::shared_ptr<RBX::MouseCommand>::operator=<RBX::NewNullTool>(boost::shared_ptr<RBX::NewNullTool> const&)
pub fn stub_0x6d21d8() -> ! {
    todo!("0x6d21d8 __ZN5boost10shared_ptrIN3RBX12MouseCommandEEaSINS1_11NewNullToolEEERS3_RKNS0_IT_EE")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Heartbeat const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Workspace,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Workspace*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Workspace,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Workspace*>,boost::arg<1>>> const&)")]
// 0x6d2290 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE7connectIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_9WorkspaceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEEENS0_10connectionERKT_ — rbx::signals::connection rbx::signals::signal<void ()(RBX::Heartbeat const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Workspace,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Workspace*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Workspace,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Workspace*>,boost::arg<1>>> const&)
pub fn stub_0x6d2290() -> ! {
    todo!("0x6d2290 __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE7connectIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_9WorkspaceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace*>>> const&)")]
// 0x6d2c50 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9WorkspaceEEENS6_5list1INS6_5valueIPSB_EEEEEEEENS0_10connectionERKT_ — rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace*>>> const&)
pub fn stub_0x6d2c50() -> ! {
    todo!("0x6d2c50 __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9WorkspaceEEENS6_5list1INS6_5valueIPSB_EEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace*>>>>::~callable_slot()")]
// 0x6d3ca8 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9WorkspaceEEENS6_5list1INS6_5valueIPSB_EEEEEEED1Ev — rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace*>>>>::~callable_slot()
pub fn stub_0x6d3ca8() -> ! {
    todo!("0x6d3ca8 __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9WorkspaceEEENS6_5list1INS6_5valueIPSB_EEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace*>>>>::~callable_slot()")]
// 0x6d3cd4 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9WorkspaceEEENS6_5list1INS6_5valueIPSB_EEEEEEED0Ev — rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace*>>>>::~callable_slot()
pub fn stub_0x6d3cd4() -> ! {
    todo!("0x6d3cd4 __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9WorkspaceEEENS6_5list1INS6_5valueIPSB_EEEEEEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace*>>>,0,void ()(void)>::call(void)")]
// 0x6d3dac — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9WorkspaceEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv — rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace*>>>,0,void ()(void)>::call(void)
pub fn stub_0x6d3dac() -> ! {
    todo!("0x6d3dac __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9WorkspaceEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv")
}

#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace*>>>,0,void ()(void)>::call(void)")]
// 0x6d3db4 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9WorkspaceEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv — `non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace*>>>,0,void ()(void)>::call(void)
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace*>>>,0,void ()(void)>::call(void)
pub fn stub_0x6d3db4() -> ! {
    todo!("0x6d3db4 __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9WorkspaceEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace*>>>::operator()(void)")]
// 0x6d3dbc — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX9WorkspaceEEENS0_5list1INS0_5valueIPS5_EEEEEclEv — boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace*>>>::operator()(void)
pub fn stub_0x6d3dbc() -> ! {
    todo!("0x6d3dbc __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX9WorkspaceEEENS0_5list1INS0_5valueIPS5_EEEEEclEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace*>>>,0,void ()(void)>::~callable()")]
// 0x6d3dd4 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9WorkspaceEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_ED1Ev — rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace*>>>,0,void ()(void)>::~callable()
pub fn stub_0x6d3dd4() -> ! {
    todo!("0x6d3dd4 __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9WorkspaceEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace*>>>,0,void ()(void)>::~callable()")]
// 0x6d3e00 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9WorkspaceEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_ED0Ev — rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace*>>>,0,void ()(void)>::~callable()
pub fn stub_0x6d3e00() -> ! {
    todo!("0x6d3e00 __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9WorkspaceEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_ED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Workspace,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Workspace*>,boost::arg<1>>>>::~callable_slot()")]
// 0x6d42d8 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_9WorkspaceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED1Ev — rbx::signals::signal<void ()(RBX::Heartbeat const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Workspace,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Workspace*>,boost::arg<1>>>>::~callable_slot()
pub fn stub_0x6d42d8() -> ! {
    todo!("0x6d42d8 __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_9WorkspaceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Workspace,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Workspace*>,boost::arg<1>>>>::~callable_slot()")]
// 0x6d4304 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_9WorkspaceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED0Ev — rbx::signals::signal<void ()(RBX::Heartbeat const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Workspace,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Workspace*>,boost::arg<1>>>>::~callable_slot()
pub fn stub_0x6d4304() -> ! {
    todo!("0x6d4304 __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_9WorkspaceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Workspace,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Workspace*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)")]
// 0x6d43dc — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_9WorkspaceES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_ — rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Workspace,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Workspace*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)
pub fn stub_0x6d43dc() -> ! {
    todo!("0x6d43dc __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_9WorkspaceES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_")
}

#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Workspace,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Workspace*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)")]
// 0x6d43e4 — __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_9WorkspaceES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_ — `non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Workspace,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Workspace*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Workspace,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Workspace*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)
pub fn stub_0x6d43e4() -> ! {
    todo!("0x6d43e4 __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_9WorkspaceES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_")
}

#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Workspace,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Workspace*>,boost::arg<1>>>::operator()<RBX::Heartbeat>(RBX::Heartbeat const&)")]
// 0x6d43ec — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX9WorkspaceERKNS4_9HeartbeatEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_ — void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Workspace,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Workspace*>,boost::arg<1>>>::operator()<RBX::Heartbeat>(RBX::Heartbeat const&)
pub fn stub_0x6d43ec() -> ! {
    todo!("0x6d43ec __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX9WorkspaceERKNS4_9HeartbeatEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Workspace,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Workspace*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::~callable()")]
// 0x6d4408 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_9WorkspaceES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED1Ev — rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Workspace,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Workspace*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::~callable()
pub fn stub_0x6d4408() -> ! {
    todo!("0x6d4408 __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_9WorkspaceES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Workspace,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Workspace*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::~callable()")]
// 0x6d4434 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_9WorkspaceES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED0Ev — rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Workspace,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Workspace*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::~callable()
pub fn stub_0x6d4434() -> ! {
    todo!("0x6d4434 __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_9WorkspaceES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::TouchPair const&)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::TouchPair const&)>::slot> &)")]
// 0x6d465c — __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE4nextERN5boost13intrusive_ptrINS7_4slotEEE — rbx::signals::signal<void ()(RBX::TouchPair const&)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::TouchPair const&)>::slot> &)
pub fn stub_0x6d465c() -> ! {
    todo!("0x6d465c __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE4nextERN5boost13intrusive_ptrINS7_4slotEEE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Decal>::operator=(rbx_core::SharedPtr<RBX::Decal> const&)")]
// 0x6d66fc — __ZN5boost10shared_ptrIN3RBX5DecalEEaSERKS3_ — rbx_core::SharedPtr<RBX::Decal>::operator=(rbx_core::SharedPtr<RBX::Decal> const&)
// was: boost::shared_ptr<RBX::Decal>::operator=(boost::shared_ptr<RBX::Decal> const&)
pub fn stub_0x6d66fc() -> ! {
    todo!("0x6d66fc __ZN5boost10shared_ptrIN3RBX5DecalEEaSERKS3_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Decal> RBX::shared_from<RBX::Decal>(RBX::Decal*)")]
// 0x6d6734 — __ZN3RBX11shared_fromINS_5DecalEEEN5boost10shared_ptrIT_EEPS4_ — rbx_core::SharedPtr<RBX::Decal> RBX::shared_from<RBX::Decal>(RBX::Decal*)
// was: boost::shared_ptr<RBX::Decal> RBX::shared_from<RBX::Decal>(RBX::Decal*)
pub fn stub_0x6d6734() -> ! {
    todo!("0x6d6734 __ZN3RBX11shared_fromINS_5DecalEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::MouseCommand> RBX::shared_from<RBX::MouseCommand>(RBX::MouseCommand*)")]
// 0x6d68c8 — __ZN3RBX11shared_fromINS_12MouseCommandEEEN5boost10shared_ptrIT_EEPS4_ — rbx_core::SharedPtr<RBX::MouseCommand> RBX::shared_from<RBX::MouseCommand>(RBX::MouseCommand*)
// was: boost::shared_ptr<RBX::MouseCommand> RBX::shared_from<RBX::MouseCommand>(RBX::MouseCommand*)
pub fn stub_0x6d68c8() -> ! {
    todo!("0x6d68c8 __ZN3RBX11shared_fromINS_12MouseCommandEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::DecalTool>::shared_ptr<RBX::DecalTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::DecalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x6d6b84 — __ZN5boost10shared_ptrIN3RBX9DecalToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_ — rbx_core::SharedPtr<RBX::DecalTool>::shared_ptr<RBX::DecalTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::DecalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)
// was: boost::shared_ptr<RBX::DecalTool>::shared_ptr<RBX::DecalTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::DecalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)
pub fn stub_0x6d6b84() -> ! {
    todo!("0x6d6b84 __ZN5boost10shared_ptrIN3RBX9DecalToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::DecalTool,RBX::DecalTool>(rbx_core::SharedPtr<RBX::DecalTool> const*,RBX::DecalTool *)const")]
// 0x6d6c4c — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_9DecalToolES5_EEvPKNS_10shared_ptrIT_EEPT0_ — void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::DecalTool,RBX::DecalTool>(rbx_core::SharedPtr<RBX::DecalTool> const*,RBX::DecalTool *)const
// was: void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::DecalTool,RBX::DecalTool>(boost::shared_ptr<RBX::DecalTool> const*,RBX::DecalTool *)const
pub fn stub_0x6d6c4c() -> ! {
    todo!("0x6d6c4c __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_9DecalToolES5_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::DecalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::DecalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x6d6d30 — __ZN5boost6detail12shared_countC2IPN3RBX9DecalToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_ — boost::detail::shared_count::shared_count<RBX::DecalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::DecalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)
pub fn stub_0x6d6d30() -> ! {
    todo!("0x6d6d30 __ZN5boost6detail12shared_countC2IPN3RBX9DecalToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DecalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x6d6e28 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9DecalToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev — boost::detail::sp_counted_impl_pd<RBX::DecalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()
pub fn stub_0x6d6e28() -> ! {
    todo!("0x6d6e28 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9DecalToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DecalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x6d6e2c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9DecalToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev — boost::detail::sp_counted_impl_pd<RBX::DecalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()
pub fn stub_0x6d6e2c() -> ! {
    todo!("0x6d6e2c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9DecalToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DecalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// 0x6d6e30 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9DecalToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv — boost::detail::sp_counted_impl_pd<RBX::DecalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)
pub fn stub_0x6d6e30() -> ! {
    todo!("0x6d6e30 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9DecalToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DecalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// 0x6d6e40 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9DecalToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info — boost::detail::sp_counted_impl_pd<RBX::DecalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_0x6d6e40() -> ! {
    todo!("0x6d6e40 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9DecalToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DecalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// 0x6d6e58 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9DecalToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv — boost::detail::sp_counted_impl_pd<RBX::DecalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)
pub fn stub_0x6d6e58() -> ! {
    todo!("0x6d6e58 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9DecalToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::IAdornableCollector>::shared_ptr<RBX::IAdornableCollector>(RBX::IAdornableCollector *)")]
// 0x6d7740 — __ZN5boost10shared_ptrIN3RBX19IAdornableCollectorEEC2IS2_EEPT_ — rbx_core::SharedPtr<RBX::IAdornableCollector>::shared_ptr<RBX::IAdornableCollector>(RBX::IAdornableCollector *)
// was: boost::shared_ptr<RBX::IAdornableCollector>::shared_ptr<RBX::IAdornableCollector>(RBX::IAdornableCollector *)
pub fn stub_0x6d7740() -> ! {
    todo!("0x6d7740 __ZN5boost10shared_ptrIN3RBX19IAdornableCollectorEEC2IS2_EEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::IAdornableCollector>::~sp_counted_impl_p()")]
// 0x6d7818 — __ZN5boost6detail17sp_counted_impl_pIN3RBX19IAdornableCollectorEED1Ev — boost::detail::sp_counted_impl_p<RBX::IAdornableCollector>::~sp_counted_impl_p()
pub fn stub_0x6d7818() -> ! {
    todo!("0x6d7818 __ZN5boost6detail17sp_counted_impl_pIN3RBX19IAdornableCollectorEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::IAdornableCollector>::dispose(void)")]
// 0x6d7820 — __ZN5boost6detail17sp_counted_impl_pIN3RBX19IAdornableCollectorEE7disposeEv — boost::detail::sp_counted_impl_p<RBX::IAdornableCollector>::dispose(void)
pub fn stub_0x6d7820() -> ! {
    todo!("0x6d7820 __ZN5boost6detail17sp_counted_impl_pIN3RBX19IAdornableCollectorEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::IAdornableCollector>::get_untyped_deleter(void)")]
// 0x6d78c8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX19IAdornableCollectorEE19get_untyped_deleterEv — boost::detail::sp_counted_impl_p<RBX::IAdornableCollector>::get_untyped_deleter(void)
pub fn stub_0x6d78c8() -> ! {
    todo!("0x6d78c8 __ZN5boost6detail17sp_counted_impl_pIN3RBX19IAdornableCollectorEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AdvArrowTool>::shared_ptr<RBX::AdvArrowTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvArrowTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x6dd740 — __ZN5boost10shared_ptrIN3RBX12AdvArrowToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_ — rbx_core::SharedPtr<RBX::AdvArrowTool>::shared_ptr<RBX::AdvArrowTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvArrowTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)
// was: boost::shared_ptr<RBX::AdvArrowTool>::shared_ptr<RBX::AdvArrowTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvArrowTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)
pub fn stub_0x6dd740() -> ! {
    todo!("0x6dd740 __ZN5boost10shared_ptrIN3RBX12AdvArrowToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::AdvArrowTool,RBX::AdvArrowTool>(rbx_core::SharedPtr<RBX::AdvArrowTool> const*,RBX::AdvArrowTool *)const")]
// 0x6dd808 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_12AdvArrowToolES5_EEvPKNS_10shared_ptrIT_EEPT0_ — void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::AdvArrowTool,RBX::AdvArrowTool>(rbx_core::SharedPtr<RBX::AdvArrowTool> const*,RBX::AdvArrowTool *)const
// was: void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::AdvArrowTool,RBX::AdvArrowTool>(boost::shared_ptr<RBX::AdvArrowTool> const*,RBX::AdvArrowTool *)const
pub fn stub_0x6dd808() -> ! {
    todo!("0x6dd808 __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_12AdvArrowToolES5_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AdvArrowTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvArrowTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x6dd8ec — __ZN5boost6detail12shared_countC2IPN3RBX12AdvArrowToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_ — boost::detail::shared_count::shared_count<RBX::AdvArrowTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvArrowTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)
pub fn stub_0x6dd8ec() -> ! {
    todo!("0x6dd8ec __ZN5boost6detail12shared_countC2IPN3RBX12AdvArrowToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvArrowTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x6dd9e4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AdvArrowToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev — boost::detail::sp_counted_impl_pd<RBX::AdvArrowTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()
pub fn stub_0x6dd9e4() -> ! {
    todo!("0x6dd9e4 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AdvArrowToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvArrowTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x6dd9e8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AdvArrowToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev — boost::detail::sp_counted_impl_pd<RBX::AdvArrowTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()
pub fn stub_0x6dd9e8() -> ! {
    todo!("0x6dd9e8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AdvArrowToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvArrowTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// 0x6dd9ec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AdvArrowToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv — boost::detail::sp_counted_impl_pd<RBX::AdvArrowTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)
pub fn stub_0x6dd9ec() -> ! {
    todo!("0x6dd9ec __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AdvArrowToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvArrowTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// 0x6dd9fc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AdvArrowToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info — boost::detail::sp_counted_impl_pd<RBX::AdvArrowTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_0x6dd9fc() -> ! {
    todo!("0x6dd9fc __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AdvArrowToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvArrowTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// 0x6dda14 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AdvArrowToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv — boost::detail::sp_counted_impl_pd<RBX::AdvArrowTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)
pub fn stub_0x6dda14() -> ! {
    todo!("0x6dda14 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AdvArrowToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::NewNullTool>::shared_ptr<RBX::NewNullTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::NewNullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x6dda18 — __ZN5boost10shared_ptrIN3RBX11NewNullToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_ — rbx_core::SharedPtr<RBX::NewNullTool>::shared_ptr<RBX::NewNullTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::NewNullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)
// was: boost::shared_ptr<RBX::NewNullTool>::shared_ptr<RBX::NewNullTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::NewNullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)
pub fn stub_0x6dda18() -> ! {
    todo!("0x6dda18 __ZN5boost10shared_ptrIN3RBX11NewNullToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::NewNullTool,RBX::NewNullTool>(rbx_core::SharedPtr<RBX::NewNullTool> const*,RBX::NewNullTool *)const")]
// 0x6ddae0 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_11NewNullToolES5_EEvPKNS_10shared_ptrIT_EEPT0_ — void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::NewNullTool,RBX::NewNullTool>(rbx_core::SharedPtr<RBX::NewNullTool> const*,RBX::NewNullTool *)const
// was: void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::NewNullTool,RBX::NewNullTool>(boost::shared_ptr<RBX::NewNullTool> const*,RBX::NewNullTool *)const
pub fn stub_0x6ddae0() -> ! {
    todo!("0x6ddae0 __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_11NewNullToolES5_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::NewNullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::NewNullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x6ddbc4 — __ZN5boost6detail12shared_countC2IPN3RBX11NewNullToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_ — boost::detail::shared_count::shared_count<RBX::NewNullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::NewNullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)
pub fn stub_0x6ddbc4() -> ! {
    todo!("0x6ddbc4 __ZN5boost6detail12shared_countC2IPN3RBX11NewNullToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NewNullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x6ddcbc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11NewNullToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev — boost::detail::sp_counted_impl_pd<RBX::NewNullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()
pub fn stub_0x6ddcbc() -> ! {
    todo!("0x6ddcbc __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11NewNullToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NewNullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x6ddcc0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11NewNullToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev — boost::detail::sp_counted_impl_pd<RBX::NewNullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()
pub fn stub_0x6ddcc0() -> ! {
    todo!("0x6ddcc0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11NewNullToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NewNullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// 0x6ddcc4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11NewNullToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv — boost::detail::sp_counted_impl_pd<RBX::NewNullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)
pub fn stub_0x6ddcc4() -> ! {
    todo!("0x6ddcc4 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11NewNullToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NewNullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// 0x6ddcd4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11NewNullToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info — boost::detail::sp_counted_impl_pd<RBX::NewNullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_0x6ddcd4() -> ! {
    todo!("0x6ddcd4 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11NewNullToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NewNullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// 0x6ddcec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11NewNullToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv — boost::detail::sp_counted_impl_pd<RBX::NewNullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)
pub fn stub_0x6ddcec() -> ! {
    todo!("0x6ddcec __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11NewNullToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "RBX::Stats::Item* RBX::Stats::Item::createChildItem<double>(char const*,boost::function0<double>)")]
// 0x6de388 — __ZN3RBX5Stats4Item15createChildItemIdEEPS1_PKcN5boost9function0IT_EE — RBX::Stats::Item* RBX::Stats::Item::createChildItem<double>(char const*,boost::function0<double>)
pub fn stub_0x6de388() -> ! {
    todo!("0x6de388 __ZN3RBX5Stats4Item15createChildItemIdEEPS1_PKcN5boost9function0IT_EE")
}

#[doc(alias = "RBX::Stats::Item* RBX::Stats::Item::createChildItem<float>(char const*,boost::function0<float>)")]
// 0x6de56c — __ZN3RBX5Stats4Item15createChildItemIfEEPS1_PKcN5boost9function0IT_EE — RBX::Stats::Item* RBX::Stats::Item::createChildItem<float>(char const*,boost::function0<float>)
pub fn stub_0x6de56c() -> ! {
    todo!("0x6de56c __ZN3RBX5Stats4Item15createChildItemIfEEPS1_PKcN5boost9function0IT_EE")
}

#[doc(alias = "RBX::Stats::Item* RBX::Stats::Item::createChildItem<int>(char const*,boost::function0<int>)")]
// 0x6de770 — __ZN3RBX5Stats4Item15createChildItemIiEEPS1_PKcN5boost9function0IT_EE — RBX::Stats::Item* RBX::Stats::Item::createChildItem<int>(char const*,boost::function0<int>)
pub fn stub_0x6de770() -> ! {
    todo!("0x6de770 __ZN3RBX5Stats4Item15createChildItemIiEEPS1_PKcN5boost9function0IT_EE")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<double,boost::_mfi::cmf0<double,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace const*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x6de9a8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIdNS_4_mfi4cmf0IdN3RBX9WorkspaceEEENS3_5list1INS3_5valueIPKS8_EEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE — boost::detail::function::functor_manager<boost::_bi::bind_t<double,boost::_mfi::cmf0<double,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace const*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0x6de9a8() -> ! {
    todo!("0x6de9a8 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIdNS_4_mfi4cmf0IdN3RBX9WorkspaceEEENS3_5list1INS3_5valueIPKS8_EEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<double,boost::_mfi::cmf0<double,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace const*>>>,double>::invoke(boost::detail::function::function_buffer &)")]
// 0x6dea08 — __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIdNS_4_mfi4cmf0IdN3RBX9WorkspaceEEENS3_5list1INS3_5valueIPKS8_EEEEEEdE6invokeERNS1_15function_bufferE — boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<double,boost::_mfi::cmf0<double,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace const*>>>,double>::invoke(boost::detail::function::function_buffer &)
pub fn stub_0x6dea08() -> ! {
    todo!("0x6dea08 __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIdNS_4_mfi4cmf0IdN3RBX9WorkspaceEEENS3_5list1INS3_5valueIPKS8_EEEEEEdE6invokeERNS1_15function_bufferE")
}
