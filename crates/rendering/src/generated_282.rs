//! rendering shard 282 — 100 stubs EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Render 15112/15112 complete, 30720->30820 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 30720 before -> 30820 after; global gap filler)
//! Filter: Ogre|G3D|Render exhausted (0 remaining), filler global asc next 100 after 0x3d9d30

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0x3d9d3c — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS5_10Reflection18PropertyDescriptorEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf2IvNS5_20ChangeHistoryServiceES7_SB_EENSF_5list3INSF_5valueIPSJ_EENS3_3argILi1EEENSP_ILi2EEEEEEELi2ESC_E4callES7_SB_
// type: int __fastcall(int, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list3<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::call(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS5_10Reflection18PropertyDescriptorEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf2IvNS5_20ChangeHistoryServiceES7_SB_EENSF_5list3INSF_5valueIPSJ_EENS3_3argILi1EEENSP_ILi2EEEEEEELi2ESC_E4callES7_SB_
pub fn stub_3d9d3c() -> ! {
    todo!("0x3d9d3c rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list3<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::call(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)")
}

// 0x3d9d60 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS5_10Reflection18PropertyDescriptorEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf2IvNS5_20ChangeHistoryServiceES7_SB_EENSF_5list3INSF_5valueIPSJ_EENS3_3argILi1EEENSP_ILi2EEEEEEELi2ESC_E4callES7_SB_
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS5_10Reflection18PropertyDescriptorEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf2IvNS5_20ChangeHistoryServiceES7_SB_EENSF_5list3INSF_5valueIPSJ_EENS3_3argILi1EEENSP_ILi2EEEEEEELi2ESC_E4callES7_SB_")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS5_10Reflection18PropertyDescriptorEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf2IvNS5_20ChangeHistoryServiceES7_SB_EENSF_5list3INSF_5valueIPSJ_EENS3_3argILi1EEENSP_ILi2EEEEEEELi2ESC_E4callES7_SB_
pub fn stub_3d9d60() -> ! {
    todo!("0x3d9d60 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list3<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::call(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)")
}

// 0x3d9d84 — __ZN5boost3_bi5list3INS0_5valueIPN3RBX20ChangeHistoryServiceEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_NS_10shared_ptrINS3_8InstanceEEEPKNS3_10Reflection18PropertyDescriptorEEENS0_5list2IRSG_RSK_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, int, const shared_count **)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::ChangeHistoryService *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::shared_ptr<RBX::Instance>&,RBX::Reflection::PropertyDescriptor const*&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*> &,boost::_bi::list2<boost::shared_ptr<RBX::Instance>&,RBX::Reflection::PropertyDescriptor const*&> &,int)")]
// was: __ZN5boost3_bi5list3INS0_5valueIPN3RBX20ChangeHistoryServiceEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_NS_10shared_ptrINS3_8InstanceEEEPKNS3_10Reflection18PropertyDescriptorEEENS0_5list2IRSG_RSK_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_3d9d84() -> ! {
    todo!("0x3d9d84 void boost::_bi::list3<boost::_bi::value<RBX::ChangeHistoryService *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::shared_ptr<RBX::Instance>&,RBX::Reflection::PropertyDescriptor const*&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*> &,boost::_bi::list2<boost::shared_ptr<RBX::Instance>&,RBX::Reflection::PropertyDescriptor const*&> &,int)")
}

// 0x3d9e64 — __ZNK5boost4_mfi3mf2IvN3RBX20ChangeHistoryServiceENS_10shared_ptrINS2_8InstanceEEEPKNS2_10Reflection18PropertyDescriptorEEclEPS3_S6_SA_
// type: void __fastcall(char **, int, const shared_count *, int)
#[doc(alias = "boost::_mfi::mf2<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>::operator()(RBX::ChangeHistoryService*,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)const")]
// was: __ZNK5boost4_mfi3mf2IvN3RBX20ChangeHistoryServiceENS_10shared_ptrINS2_8InstanceEEEPKNS2_10Reflection18PropertyDescriptorEEclEPS3_S6_SA_
pub fn stub_3d9e64() -> ! {
    todo!("0x3d9e64 boost::_mfi::mf2<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>::operator()(RBX::ChangeHistoryService*,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)const")
}

// 0x3d9f50 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE6removeEPNSC_4slotE
// type: int __fastcall(char **, char *, int, const void *)
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::remove(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot *)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE6removeEPNSC_4slotE
pub fn stub_3d9f50() -> ! {
    todo!("0x3d9f50 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::remove(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot *)")
}

// 0x3da040 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE4slot22safe_static_init_mutexEv
// type: 
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE4slot22safe_static_init_mutexEv
pub fn stub_3da040() -> ! {
    todo!("0x3da040 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot::safe_static_init_mutex(void)")
}

// 0x3da048 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE4slotD0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE4slotD0Ev
pub fn stub_3da048() -> ! {
    todo!("0x3da048 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot::~slot()")
}

// 0x3da11c — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS5_10Reflection18PropertyDescriptorEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf2IvNS5_20ChangeHistoryServiceES7_SB_EENSF_5list3INSF_5valueIPSJ_EENS3_3argILi1EEENSP_ILi2EEEEEEELi2ESC_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list3<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS5_10Reflection18PropertyDescriptorEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf2IvNS5_20ChangeHistoryServiceES7_SB_EENSF_5list3INSF_5valueIPSJ_EENS3_3argILi1EEENSP_ILi2EEEEEEELi2ESC_ED1Ev
pub fn stub_3da11c() -> ! {
    todo!("0x3da11c rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list3<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::~callable()")
}

// 0x3da148 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS5_10Reflection18PropertyDescriptorEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf2IvNS5_20ChangeHistoryServiceES7_SB_EENSF_5list3INSF_5valueIPSJ_EENS3_3argILi1EEENSP_ILi2EEEEEEELi2ESC_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list3<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS5_10Reflection18PropertyDescriptorEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf2IvNS5_20ChangeHistoryServiceES7_SB_EENSF_5list3INSF_5valueIPSJ_EENS3_3argILi1EEENSP_ILi2EEEEEEELi2ESC_ED0Ev
pub fn stub_3da148() -> ! {
    todo!("0x3da148 rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list3<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::~callable()")
}

// 0x3da220 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEEEE4slotEEaSEPSA_
// type: int *__fastcall(int *, int)
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot>::operator=(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot*)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEEEE4slotEEaSEPSA_
pub fn stub_3da220() -> ! {
    todo!("0x3da220 boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot>::operator=(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot*)")
}

// 0x3da244 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_20ChangeHistoryServiceES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_20ChangeHistoryServiceES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEED1Ev
pub fn stub_3da244() -> ! {
    todo!("0x3da244 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x3da270 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_20ChangeHistoryServiceES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_20ChangeHistoryServiceES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEED0Ev
pub fn stub_3da270() -> ! {
    todo!("0x3da270 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x3da348 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_20ChangeHistoryServiceES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
// type: int __fastcall(int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_20ChangeHistoryServiceES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
pub fn stub_3da348() -> ! {
    todo!("0x3da348 rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)")
}

// 0x3da364 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_20ChangeHistoryServiceES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
// type: int __fastcall(int, int)
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_20ChangeHistoryServiceES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_20ChangeHistoryServiceES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
pub fn stub_3da364() -> ! {
    todo!("0x3da364 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)")
}

// 0x3da380 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX20ChangeHistoryServiceEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, int, const shared_count **)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&> &,int)")]
// was: __ZN5boost3_bi5list2INS0_5valueIPN3RBX20ChangeHistoryServiceEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_3da380() -> ! {
    todo!("0x3da380 void boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&> &,int)")
}

// 0x3da45c — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_20ChangeHistoryServiceES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_20ChangeHistoryServiceES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_ED1Ev
pub fn stub_3da45c() -> ! {
    todo!("0x3da45c rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()")
}

// 0x3da488 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_20ChangeHistoryServiceES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_20ChangeHistoryServiceES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_ED0Ev
pub fn stub_3da488() -> ! {
    todo!("0x3da488 rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()")
}

// 0x3da560 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ChangeHistoryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ChangeHistoryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_3da560() -> ! {
    todo!("0x3da560 boost::detail::sp_counted_impl_pd<RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x3da568 — __ZN3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E15isNullClassNameEv
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E15isNullClassNameEv")]
// was: __ZN3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E15isNullClassNameEv
pub fn stub_3da568() -> ! {
    todo!("0x3da568 __ZN3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E15isNullClassNameEv")
}

// 0x3da5d0 — __ZN3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3da5d0() -> ! {
    todo!("0x3da5d0 __ZN3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x3da5d4 — __ZN3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3da5d4() -> ! {
    todo!("0x3da5d4 __ZN3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x3da674 — __ZThn32_N3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3da674() -> ! {
    todo!("0x3da674 __ZThn32_N3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x3da67c — __ZThn32_N3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3da67c() -> ! {
    todo!("0x3da67c __ZThn32_N3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x3da720 — __ZThn36_N3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3da720() -> ! {
    todo!("0x3da720 __ZThn36_N3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x3da728 — __ZThn36_N3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3da728() -> ! {
    todo!("0x3da728 __ZThn36_N3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x3da7cc — __ZNSt5dequeIPN3RBX20ChangeHistoryService4ItemESaIS3_EE8pop_backEv
// type: int __fastcall(int)
#[doc(alias = "std::deque<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::pop_back(void)")]
// was: __ZNSt5dequeIPN3RBX20ChangeHistoryService4ItemESaIS3_EE8pop_backEv
pub fn stub_3da7cc() -> ! {
    todo!("0x3da7cc std::deque<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::pop_back(void)")
}

// 0x3da7fc — __ZNSt5dequeIPN3RBX20ChangeHistoryService4ItemESaIS3_EE9push_backERKS3_
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::deque<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::push_back(RBX::ChangeHistoryService::Item * const&)")]
// was: __ZNSt5dequeIPN3RBX20ChangeHistoryService4ItemESaIS3_EE9push_backERKS3_
pub fn stub_3da7fc() -> ! {
    todo!("0x3da7fc std::deque<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::push_back(RBX::ChangeHistoryService::Item * const&)")
}

// 0x3da81c — __ZNSt5dequeIPN3RBX20ChangeHistoryService4ItemESaIS3_EE16_M_push_back_auxERKS3_
// type: int __fastcall(_DWORD *, int *)
#[doc(alias = "std::deque<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::_M_push_back_aux(RBX::ChangeHistoryService::Item * const&)")]
// was: __ZNSt5dequeIPN3RBX20ChangeHistoryService4ItemESaIS3_EE16_M_push_back_auxERKS3_
pub fn stub_3da81c() -> ! {
    todo!("0x3da81c std::deque<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::_M_push_back_aux(RBX::ChangeHistoryService::Item * const&)")
}

// 0x3da854 — __ZNSt5dequeIPN3RBX20ChangeHistoryService4ItemESaIS3_EE22_M_reserve_map_at_backEm
// type: _DWORD *__fastcall(_DWORD *result, int)
#[doc(alias = "std::deque<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::_M_reserve_map_at_back(unsigned long)")]
// was: __ZNSt5dequeIPN3RBX20ChangeHistoryService4ItemESaIS3_EE22_M_reserve_map_at_backEm
pub fn stub_3da854() -> ! {
    todo!("0x3da854 std::deque<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::_M_reserve_map_at_back(unsigned long)")
}

// 0x3da870 — __ZNSt5dequeIPN3RBX20ChangeHistoryService4ItemESaIS3_EE17_M_reallocate_mapEmb
// type: char *__fastcall(void **, unsigned int, int)
#[doc(alias = "std::deque<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::_M_reallocate_map(unsigned long,bool)")]
// was: __ZNSt5dequeIPN3RBX20ChangeHistoryService4ItemESaIS3_EE17_M_reallocate_mapEmb
pub fn stub_3da870() -> ! {
    todo!("0x3da870 std::deque<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::_M_reallocate_map(unsigned long,bool)")
}

// 0x3da948 — __ZNSt11_Deque_baseIPN3RBX20ChangeHistoryService4ItemESaIS3_EE15_M_allocate_mapEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Deque_base<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::_M_allocate_map(unsigned long)")]
// was: __ZNSt11_Deque_baseIPN3RBX20ChangeHistoryService4ItemESaIS3_EE15_M_allocate_mapEm
pub fn stub_3da948() -> ! {
    todo!("0x3da948 std::_Deque_base<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::_M_allocate_map(unsigned long)")
}

// 0x3da960 — __ZNSt5dequeIPN3RBX20ChangeHistoryService4ItemESaIS3_EEC2ERKS5_
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "std::deque<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::deque(std::deque<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>> const&)")]
// was: __ZNSt5dequeIPN3RBX20ChangeHistoryService4ItemESaIS3_EEC2ERKS5_
pub fn stub_3da960() -> ! {
    todo!("0x3da960 std::deque<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::deque(std::deque<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>> const&)")
}

// 0x3da9f4 — __ZNSt11_Deque_baseIPN3RBX20ChangeHistoryService4ItemESaIS3_EED2Ev
// type: int __fastcall(int)
#[doc(alias = "std::_Deque_base<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::~_Deque_base()")]
// was: __ZNSt11_Deque_baseIPN3RBX20ChangeHistoryService4ItemESaIS3_EED2Ev
pub fn stub_3da9f4() -> ! {
    todo!("0x3da9f4 std::_Deque_base<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::~_Deque_base()")
}

// 0x3daa20 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt15_Deque_iteratorIPN3RBX20ChangeHistoryService4ItemERKS7_PS8_ES3_IS7_RS7_PS7_EEET0_T_SG_SF_
// type: _DWORD *__fastcall(_DWORD *result, int *, int, int *, int, int, int, int, int, _DWORD *)
#[doc(alias = "std::_Deque_iterator<RBX::ChangeHistoryService::Item *,RBX::ChangeHistoryService::Item *&,RBX::ChangeHistoryService::Item **> std::__copy<false,std::random_access_iterator_tag>::copy<std::_Deque_iterator<RBX::ChangeHistoryService::Item *,RBX::ChangeHistoryService::Item * const&,RBX::ChangeHistoryService::Item * const*>,std::_Deque_iterator<RBX::ChangeHistoryService::Item *,RBX::ChangeHistoryService::Item *&,RBX::ChangeHistoryService::Item **>>(std::_Deque_iterator<RBX::ChangeHistoryService::Item *,RBX::ChangeHistoryService::Item * const&,RBX::ChangeHistoryService::Item * const*>,std::_Deque_iterator<RBX::ChangeHistoryService::Item *,RBX::ChangeHistoryService::Item * const&,RBX::ChangeHistoryService::Item * const*>,std::_Deque_iterator<RBX::ChangeHistoryService::Item *,RBX::ChangeHistoryService::Item *&,RBX::ChangeHistoryService::Item **>)")]
// was: __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt15_Deque_iteratorIPN3RBX20ChangeHistoryService4ItemERKS7_PS8_ES3_IS7_RS7_PS7_EEET0_T_SG_SF_
pub fn stub_3daa20() -> ! {
    todo!("0x3daa20 std::_Deque_iterator<RBX::ChangeHistoryService::Item *,RBX::ChangeHistoryService::Item *&,RBX::ChangeHistoryService::Item **> std::__copy<false,std::random_access_iterator_tag>::copy<std::_Deque_iterator<RBX::ChangeHistoryService::Item *,RBX::ChangeHistoryService::Item * const&,RBX::ChangeHistoryService::Item * const*>,std::_Deque_iterator<RBX::ChangeHistoryService::Item *,RBX::ChangeHistoryService::Item *&,RBX::ChangeHistoryService::Item **>>(std::_Deque_iterator<RBX::ChangeHistoryService::Item *,RBX::ChangeHistoryService::Item * const&,RBX::ChangeHistoryService::Item * const*>,std::_Deque_iterator<RBX::ChangeHistoryService::Item *,RBX::ChangeHistoryService::Item * const&,RBX::ChangeHistoryService::Item * const*>,std::_Deque_iterator<RBX::ChangeHistoryService::Item *,RBX::ChangeHistoryService::Item *&,RBX::ChangeHistoryService::Item **>)")
}

// 0x3daabc — __ZNSt11_Deque_baseIPN3RBX20ChangeHistoryService4ItemESaIS3_EE17_M_initialize_mapEm
// type: void __fastcall(int *, unsigned int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, void *, int)
#[doc(alias = "std::_Deque_base<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::_M_initialize_map(unsigned long)")]
// was: __ZNSt11_Deque_baseIPN3RBX20ChangeHistoryService4ItemESaIS3_EE17_M_initialize_mapEm
pub fn stub_3daabc() -> ! {
    todo!("0x3daabc std::_Deque_base<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::_M_initialize_map(unsigned long)")
}

// 0x3dac14 — __ZNSt11_Deque_baseIPN3RBX20ChangeHistoryService4ItemESaIS3_EE15_M_create_nodesEPPS3_S7_
// type: void __fastcall(int, _DWORD *, unsigned int, int, void *, int)
#[doc(alias = "std::_Deque_base<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::_M_create_nodes(RBX::ChangeHistoryService::Item ***,RBX::ChangeHistoryService::Item ***)")]
// was: __ZNSt11_Deque_baseIPN3RBX20ChangeHistoryService4ItemESaIS3_EE15_M_create_nodesEPPS3_S7_
pub fn stub_3dac14() -> ! {
    todo!("0x3dac14 std::_Deque_base<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::_M_create_nodes(RBX::ChangeHistoryService::Item ***,RBX::ChangeHistoryService::Item ***)")
}

// 0x3dad08 — __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE4findERS6_
// type: _DWORD *__fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::find(RBX::Reflection::PropertyDescriptor const* const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE4findERS6_
pub fn stub_3dad08() -> ! {
    todo!("0x3dad08 std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::find(RBX::Reflection::PropertyDescriptor const* const&)")
}

// 0x3dad48 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX20ChangeHistoryService4ItemEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS5_RKSt4pairIPKNS3_10Reflection18PropertyDescriptorENSF_7VariantEEEENS0_5list1IRSE_IKSI_SJ_EEEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int *, int *, _DWORD **)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant> const&>,boost::_bi::list1<std::pair&<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant> const&> &,boost::_bi::list1<std::pair&<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>> &,int)")]
// was: __ZN5boost3_bi5list2INS0_5valueIPN3RBX20ChangeHistoryService4ItemEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS5_RKSt4pairIPKNS3_10Reflection18PropertyDescriptorENSF_7VariantEEEENS0_5list1IRSE_IKSI_SJ_EEEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_3dad48() -> ! {
    todo!("0x3dad48 void boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant> const&>,boost::_bi::list1<std::pair&<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant> const&> &,boost::_bi::list1<std::pair&<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>> &,int)")
}

// 0x3dae68 — __ZN3RBX5Voxel6RegionINS0_4Grid5ChunkEE8iteratorC2ERKS4_
// type: int __fastcall(int, unsigned __int16 *)
#[doc(alias = "RBX::Voxel::Region<RBX::Voxel::Grid::Chunk>::iterator::iterator(RBX::Voxel::Region<RBX::Voxel::Grid::Chunk> const&)")]
// was: __ZN3RBX5Voxel6RegionINS0_4Grid5ChunkEE8iteratorC2ERKS4_
pub fn stub_3dae68() -> ! {
    todo!("0x3dae68 RBX::Voxel::Region<RBX::Voxel::Grid::Chunk>::iterator::iterator(RBX::Voxel::Region<RBX::Voxel::Grid::Chunk> const&)")
}

// 0x3dafa0 — __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFN5boost10shared_ptrIKNS0_5TupleEEEvELi0EEC2EMS2_FS7_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, unsigned int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,boost::shared_ptr<RBX::Reflection::Tuple const> ()(void),0>::BoundFuncDesc(boost::shared_ptr<RBX::Reflection::Tuple const> (RBX::ChangeHistoryService::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFN5boost10shared_ptrIKNS0_5TupleEEEvELi0EEC2EMS2_FS7_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_3dafa0() -> ! {
    todo!("0x3dafa0 RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,boost::shared_ptr<RBX::Reflection::Tuple const> ()(void),0>::BoundFuncDesc(boost::shared_ptr<RBX::Reflection::Tuple const> (RBX::ChangeHistoryService::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x3db0a4 — __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFN5boost10shared_ptrIKNS0_5TupleEEEvELi0EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,boost::shared_ptr<RBX::Reflection::Tuple const> ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFN5boost10shared_ptrIKNS0_5TupleEEEvELi0EED0Ev
pub fn stub_3db0a4() -> ! {
    todo!("0x3db0a4 RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,boost::shared_ptr<RBX::Reflection::Tuple const> ()(void),0>::~BoundFuncDesc()")
}

// 0x3db158 — __ZNK3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFN5boost10shared_ptrIKNS0_5TupleEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,boost::shared_ptr<RBX::Reflection::Tuple const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFN5boost10shared_ptrIKNS0_5TupleEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_3db158() -> ! {
    todo!("0x3db158 RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,boost::shared_ptr<RBX::Reflection::Tuple const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x3db17c — __ZN3RBX10Reflection11Call0HelperINS_20ChangeHistoryServiceEMS2_FN5boost10shared_ptrIKNS0_5TupleEEEvES7_E4callEPS2_S9_RNS0_7VariantE
// type: void __fastcall(int, char *, int, _DWORD *)
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::ChangeHistoryService,boost::shared_ptr<RBX::Reflection::Tuple const> (RBX::ChangeHistoryService::*)(void),boost::shared_ptr<RBX::Reflection::Tuple const>>::call(RBX::ChangeHistoryService*,boost::shared_ptr<RBX::Reflection::Tuple const> (RBX::ChangeHistoryService::*)(void),RBX::Reflection::Variant &)")]
// was: __ZN3RBX10Reflection11Call0HelperINS_20ChangeHistoryServiceEMS2_FN5boost10shared_ptrIKNS0_5TupleEEEvES7_E4callEPS2_S9_RNS0_7VariantE
pub fn stub_3db17c() -> ! {
    todo!("0x3db17c RBX::Reflection::Call0Helper<RBX::ChangeHistoryService,boost::shared_ptr<RBX::Reflection::Tuple const> (RBX::ChangeHistoryService::*)(void),boost::shared_ptr<RBX::Reflection::Tuple const>>::call(RBX::ChangeHistoryService*,boost::shared_ptr<RBX::Reflection::Tuple const> (RBX::ChangeHistoryService::*)(void),RBX::Reflection::Variant &)")
}

// 0x3db268 — __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, unsigned int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(void),0>::BoundFuncDesc(void (RBX::ChangeHistoryService::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_3db268() -> ! {
    todo!("0x3db268 RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(void),0>::BoundFuncDesc(void (RBX::ChangeHistoryService::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x3db36c — __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvvELi0EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvvELi0EED0Ev
pub fn stub_3db36c() -> ! {
    todo!("0x3db36c RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(void),0>::~BoundFuncDesc()")
}

// 0x3db420 — __ZNK3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_3db420() -> ! {
    todo!("0x3db420 RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x3db440 — __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvSsELi1EEC2EMS2_FvSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(std::string),1>::BoundFuncDesc(void (RBX::ChangeHistoryService::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvSsELi1EEC2EMS2_FvSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_3db440() -> ! {
    todo!("0x3db440 RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(std::string),1>::BoundFuncDesc(void (RBX::ChangeHistoryService::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x3db5b8 — __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvSsELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvSsELi1EE16declareSignatureEPKcNS0_7VariantE
pub fn stub_3db5b8() -> ! {
    todo!("0x3db5b8 RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x3db5e8 — __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvSsELi1EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(std::string),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvSsELi1EED0Ev
pub fn stub_3db5e8() -> ! {
    todo!("0x3db5e8 RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(std::string),1>::~BoundFuncDesc()")
}

// 0x3db6b4 — __ZNK3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_3db6b4() -> ! {
    todo!("0x3db6b4 RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x3db7f0 — __ZN3RBX10Reflection11Call1HelperINS_20ChangeHistoryServiceEMS2_FvSsESsvE4callEPS2_S4_RNS0_7VariantERKSs
// type: void __fastcall(int, char *, int, int, std::string *)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::ChangeHistoryService,void (RBX::ChangeHistoryService::*)(std::string),std::string,void>::call(RBX::ChangeHistoryService*,void (RBX::ChangeHistoryService::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
// was: __ZN3RBX10Reflection11Call1HelperINS_20ChangeHistoryServiceEMS2_FvSsESsvE4callEPS2_S4_RNS0_7VariantERKSs
pub fn stub_3db7f0() -> ! {
    todo!("0x3db7f0 RBX::Reflection::Call1Helper<RBX::ChangeHistoryService,void (RBX::ChangeHistoryService::*)(std::string),std::string,void>::call(RBX::ChangeHistoryService*,void (RBX::ChangeHistoryService::*)(std::string),RBX::Reflection::Variant &,std::string const&)")
}

// 0x3db920 — __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvbELi1EEC2EMS2_FvbEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(bool),1>::BoundFuncDesc(void (RBX::ChangeHistoryService::*)(bool),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvbELi1EEC2EMS2_FvbEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_3db920() -> ! {
    todo!("0x3db920 RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(bool),1>::BoundFuncDesc(void (RBX::ChangeHistoryService::*)(bool),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x3dba98 — __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvbELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvbELi1EE16declareSignatureEPKcNS0_7VariantE
pub fn stub_3dba98() -> ! {
    todo!("0x3dba98 RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x3dbac8 — __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvbELi1EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(bool),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvbELi1EED0Ev
pub fn stub_3dbac8() -> ! {
    todo!("0x3dbac8 RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(bool),1>::~BoundFuncDesc()")
}

// 0x3dbb9c — __ZNK3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_3dbb9c() -> ! {
    todo!("0x3dbb9c RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x3dbbd0 — __ZNSt6vectorIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorESaIS2_EE6resizeEmS2_
// type: int __fastcall(int result, unsigned int, int)
#[doc(alias = "std::vector<RBX::ChangeHistoryService::RuntimeUndoBehavior,std::allocator<RBX::ChangeHistoryService::RuntimeUndoBehavior>>::resize(unsigned long,RBX::ChangeHistoryService::RuntimeUndoBehavior)")]
// was: __ZNSt6vectorIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorESaIS2_EE6resizeEmS2_
pub fn stub_3dbbd0() -> ! {
    todo!("0x3dbbd0 std::vector<RBX::ChangeHistoryService::RuntimeUndoBehavior,std::allocator<RBX::ChangeHistoryService::RuntimeUndoBehavior>>::resize(unsigned long,RBX::ChangeHistoryService::RuntimeUndoBehavior)")
}

// 0x3dbc08 — __ZNSt6vectorIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorESaIS2_EE9push_backERKS2_
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<RBX::ChangeHistoryService::RuntimeUndoBehavior,std::allocator<RBX::ChangeHistoryService::RuntimeUndoBehavior>>::push_back(RBX::ChangeHistoryService::RuntimeUndoBehavior const&)")]
// was: __ZNSt6vectorIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorESaIS2_EE9push_backERKS2_
pub fn stub_3dbc08() -> ! {
    todo!("0x3dbc08 std::vector<RBX::ChangeHistoryService::RuntimeUndoBehavior,std::allocator<RBX::ChangeHistoryService::RuntimeUndoBehavior>>::push_back(RBX::ChangeHistoryService::RuntimeUndoBehavior const&)")
}

// 0x3dbc34 — __ZNSt3mapIPKN3RBX4NameENS0_20ChangeHistoryService19RuntimeUndoBehaviorESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: _Rb_tree_node_base **__fastcall(int, int *)
#[doc(alias = "std::map<RBX::Name const*,RBX::ChangeHistoryService::RuntimeUndoBehavior,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>>::operator[](RBX::Name const* const&)")]
// was: __ZNSt3mapIPKN3RBX4NameENS0_20ChangeHistoryService19RuntimeUndoBehaviorESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_3dbc34() -> ! {
    todo!("0x3dbc34 std::map<RBX::Name const*,RBX::ChangeHistoryService::RuntimeUndoBehavior,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>>::operator[](RBX::Name const* const&)")
}

// 0x3dbc8c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ChangeHistoryService19RuntimeUndoBehaviorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>,std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ChangeHistoryService19RuntimeUndoBehaviorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_3dbc8c() -> ! {
    todo!("0x3dbc8c std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>,std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior> const&)")
}

// 0x3dbd40 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ChangeHistoryService19RuntimeUndoBehaviorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ChangeHistoryService19RuntimeUndoBehaviorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_3dbd40() -> ! {
    todo!("0x3dbd40 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior> const&)")
}

// 0x3dbd98 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ChangeHistoryService19RuntimeUndoBehaviorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ChangeHistoryService19RuntimeUndoBehaviorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_3dbd98() -> ! {
    todo!("0x3dbd98 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior> const&)")
}

// 0x3dbe04 — __ZNSt6vectorIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, char *, _DWORD *)
#[doc(alias = "std::vector<RBX::ChangeHistoryService::RuntimeUndoBehavior,std::allocator<RBX::ChangeHistoryService::RuntimeUndoBehavior>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::ChangeHistoryService::RuntimeUndoBehavior*,std::vector<RBX::ChangeHistoryService::RuntimeUndoBehavior,std::allocator<RBX::ChangeHistoryService::RuntimeUndoBehavior>>>,RBX::ChangeHistoryService::RuntimeUndoBehavior const&)")]
// was: __ZNSt6vectorIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_3dbe04() -> ! {
    todo!("0x3dbe04 std::vector<RBX::ChangeHistoryService::RuntimeUndoBehavior,std::allocator<RBX::ChangeHistoryService::RuntimeUndoBehavior>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::ChangeHistoryService::RuntimeUndoBehavior*,std::vector<RBX::ChangeHistoryService::RuntimeUndoBehavior,std::allocator<RBX::ChangeHistoryService::RuntimeUndoBehavior>>>,RBX::ChangeHistoryService::RuntimeUndoBehavior const&)")
}

// 0x3dbee8 — __ZNSt12_Vector_baseIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorESaIS2_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::ChangeHistoryService::RuntimeUndoBehavior,std::allocator<RBX::ChangeHistoryService::RuntimeUndoBehavior>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorESaIS2_EE11_M_allocateEm
pub fn stub_3dbee8() -> ! {
    todo!("0x3dbee8 std::_Vector_base<RBX::ChangeHistoryService::RuntimeUndoBehavior,std::allocator<RBX::ChangeHistoryService::RuntimeUndoBehavior>>::_M_allocate(unsigned long)")
}

// 0x3dbf00 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX20ChangeHistoryService19RuntimeUndoBehaviorES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::ChangeHistoryService::RuntimeUndoBehavior * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::ChangeHistoryService::RuntimeUndoBehavior *,RBX::ChangeHistoryService::RuntimeUndoBehavior *>(RBX::ChangeHistoryService::RuntimeUndoBehavior *,RBX::ChangeHistoryService::RuntimeUndoBehavior *,RBX::ChangeHistoryService::RuntimeUndoBehavior *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX20ChangeHistoryService19RuntimeUndoBehaviorES6_EET0_T_S8_S7_
pub fn stub_3dbf00() -> ! {
    todo!("0x3dbf00 RBX::ChangeHistoryService::RuntimeUndoBehavior * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::ChangeHistoryService::RuntimeUndoBehavior *,RBX::ChangeHistoryService::RuntimeUndoBehavior *>(RBX::ChangeHistoryService::RuntimeUndoBehavior *,RBX::ChangeHistoryService::RuntimeUndoBehavior *,RBX::ChangeHistoryService::RuntimeUndoBehavior *)")
}

// 0x3dbf40 — __ZNSt6vectorIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(int result, char *, unsigned int, int *)
#[doc(alias = "std::vector<RBX::ChangeHistoryService::RuntimeUndoBehavior,std::allocator<RBX::ChangeHistoryService::RuntimeUndoBehavior>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::ChangeHistoryService::RuntimeUndoBehavior*,std::vector<RBX::ChangeHistoryService::RuntimeUndoBehavior,std::allocator<RBX::ChangeHistoryService::RuntimeUndoBehavior>>>,unsigned long,RBX::ChangeHistoryService::RuntimeUndoBehavior const&)")]
// was: __ZNSt6vectorIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_3dbf40() -> ! {
    todo!("0x3dbf40 std::vector<RBX::ChangeHistoryService::RuntimeUndoBehavior,std::allocator<RBX::ChangeHistoryService::RuntimeUndoBehavior>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::ChangeHistoryService::RuntimeUndoBehavior*,std::vector<RBX::ChangeHistoryService::RuntimeUndoBehavior,std::allocator<RBX::ChangeHistoryService::RuntimeUndoBehavior>>>,unsigned long,RBX::ChangeHistoryService::RuntimeUndoBehavior const&)")
}

// 0x3dc0d0 — __ZN5boost10flyweights19static_holder_classINS0_6detail14flyweight_coreINS2_20default_value_policyIN3RBX15ProtectedStringEEEN4mpl_2naENS0_10refcountedENS0_14hashed_factoryIS9_S9_S9_Li0EEENS0_14simple_lockingENS0_13static_holderEE10holder_argEE3getEv
// type: void *()
#[doc(alias = "boost::flyweights::static_holder_class<boost::flyweights::detail::flyweight_core<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>,mpl_::na,boost::flyweights::refcounted,boost::flyweights::hashed_factory<mpl_::na,mpl_::na,mpl_::na,0>,boost::flyweights::simple_locking,boost::flyweights::static_holder>::holder_arg>::get(void)")]
// was: __ZN5boost10flyweights19static_holder_classINS0_6detail14flyweight_coreINS2_20default_value_policyIN3RBX15ProtectedStringEEEN4mpl_2naENS0_10refcountedENS0_14hashed_factoryIS9_S9_S9_Li0EEENS0_14simple_lockingENS0_13static_holderEE10holder_argEE3getEv
pub fn stub_3dc0d0() -> ! {
    todo!("0x3dc0d0 boost::flyweights::static_holder_class<boost::flyweights::detail::flyweight_core<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>,mpl_::na,boost::flyweights::refcounted,boost::flyweights::hashed_factory<mpl_::na,mpl_::na,mpl_::na,0>,boost::flyweights::simple_locking,boost::flyweights::static_holder>::holder_arg>::get(void)")
}

// 0x3dc1f8 — __ZN5boost11multi_index21multi_index_containerINS_10flyweights6detail16refcounted_valueINS3_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES7_EENS2_20hashed_factory_classISA_S7_N4mpl_2naESD_SD_E10index_listESaISA_EED2Ev
// type: int __fastcall(int)
#[doc(alias = "boost::multi_index::multi_index_container<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>::~multi_index_container()")]
// was: __ZN5boost11multi_index21multi_index_containerINS_10flyweights6detail16refcounted_valueINS3_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES7_EENS2_20hashed_factory_classISA_S7_N4mpl_2naESD_SD_E10index_listESaISA_EED2Ev
pub fn stub_3dc1f8() -> ! {
    todo!("0x3dc1f8 boost::multi_index::multi_index_container<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>::~multi_index_container()")
}

// 0x3dc2c0 — __ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE17delete_all_nodes_Ev
// type: _DWORD *__fastcall(_DWORD *result)
#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>,boost::hash<RBX::ProtectedString>,std::equal_to<RBX::ProtectedString>,boost::multi_index::detail::nth_layer<1,boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>,boost::mpl::vector0<mpl_::na>,boost::multi_index::detail::hashed_unique_tag>::delete_all_nodes_(void)")]
// was: __ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE17delete_all_nodes_Ev
pub fn stub_3dc2c0() -> ! {
    todo!("0x3dc2c0 boost::multi_index::detail::hashed_index<boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>,boost::hash<RBX::ProtectedString>,std::equal_to<RBX::ProtectedString>,boost::multi_index::detail::nth_layer<1,boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>,boost::mpl::vector0<mpl_::na>,boost::multi_index::detail::hashed_unique_tag>::delete_all_nodes_(void)")
}

// 0x3dc308 — __ZNSt10_List_baseIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EE8_M_clearEv
// type: void __fastcall(_DWORD **)
#[doc(alias = "std::_List_base<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>::_M_clear(void)")]
// was: __ZNSt10_List_baseIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EE8_M_clearEv
pub fn stub_3dc308() -> ! {
    todo!("0x3dc308 std::_List_base<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>::_M_clear(void)")
}

// 0x3dc330 — __ZN3RBX20ChangeHistoryService4Item12unplayChangeEv
// type: void __fastcall(RBX::ChangeHistoryService::Item *this)
#[doc(alias = "RBX::ChangeHistoryService::Item::unplayChange(void)")]
// was: __ZN3RBX20ChangeHistoryService4Item12unplayChangeEv
pub fn stub_3dc330() -> ! {
    todo!("0x3dc330 RBX::ChangeHistoryService::Item::unplayChange(void)")
}

// 0x3dc500 — __ZNK5boost9function2IvNS_8functionIFvvEEESsEclES3_Ss
// type: void __fastcall(_DWORD *, int, const std::string *)
#[doc(alias = "boost::function2<void,boost::function<void ()(void)>,std::string>::operator()(boost::function<void ()(void)>,std::string)const")]
// was: __ZNK5boost9function2IvNS_8functionIFvvEEESsEclES3_Ss
pub fn stub_3dc500() -> ! {
    todo!("0x3dc500 boost::function2<void,boost::function<void ()(void)>,std::string>::operator()(boost::function<void ()(void)>,std::string)const")
}

// 0x3dc698 — __ZN3RBX20ChangeHistoryService4Item19unplayClusterChangeEv
// type: int __fastcall(RBX::ChangeHistoryService::Item *this)
#[doc(alias = "RBX::ChangeHistoryService::Item::unplayClusterChange(void)")]
// was: __ZN3RBX20ChangeHistoryService4Item19unplayClusterChangeEv
pub fn stub_3dc698() -> ! {
    todo!("0x3dc698 RBX::ChangeHistoryService::Item::unplayClusterChange(void)")
}

// 0x3dc6d0 — __ZSt8for_eachISt17_Rb_tree_iteratorISt4pairIKjSt6vectorIjSaIjEEEEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX20ChangeHistoryService4ItemERKS1_IjS5_EEENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEEET0_T_ST_SS_
// type: int __fastcall(int, _Rb_tree_node_base *, _Rb_tree_node_base *, unsigned int, unsigned int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>> std::for_each<std::_Rb_tree_iterator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>>(std::_Rb_tree_iterator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::_Rb_tree_iterator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>)")]
// was: __ZSt8for_eachISt17_Rb_tree_iteratorISt4pairIKjSt6vectorIjSaIjEEEEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX20ChangeHistoryService4ItemERKS1_IjS5_EEENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEEET0_T_ST_SS_
pub fn stub_3dc6d0() -> ! {
    todo!("0x3dc6d0 boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>> std::for_each<std::_Rb_tree_iterator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>>(std::_Rb_tree_iterator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::_Rb_tree_iterator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>)")
}

// 0x3dc72c — __ZN5boost3_bi5list2INS0_5valueIPN3RBX20ChangeHistoryService4ItemEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS5_RKSt4pairIjSt6vectorIjSaIjEEEEENS0_5list1IRSE_IKjSH_EEEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, char **, int **)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>> const&>,boost::_bi::list1<std::pair&<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>> const&> &,boost::_bi::list1<std::pair&<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>> &,int)")]
// was: __ZN5boost3_bi5list2INS0_5valueIPN3RBX20ChangeHistoryService4ItemEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS5_RKSt4pairIjSt6vectorIjSaIjEEEEENS0_5list1IRSE_IKjSH_EEEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_3dc72c() -> ! {
    todo!("0x3dc72c void boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>> const&>,boost::_bi::list1<std::pair&<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>> const&> &,boost::_bi::list1<std::pair&<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>> &,int)")
}

// 0x3dc7f4 — __ZNSt6vectorIjSaIjEEC2ERKS1_
// type: void **__fastcall(void **, _DWORD *)
#[doc(alias = "std::vector<unsigned int,std::allocator<unsigned int>>::vector(std::vector<unsigned int,std::allocator<unsigned int>> const&)")]
// was: __ZNSt6vectorIjSaIjEEC2ERKS1_
pub fn stub_3dc7f4() -> ! {
    todo!("0x3dc7f4 std::vector<unsigned int,std::allocator<unsigned int>>::vector(std::vector<unsigned int,std::allocator<unsigned int>> const&)")
}

// 0x3dc82c — __ZNSt12_Vector_baseIjSaIjEEC2EmRKS0_
// type: int __fastcall(int, int)
#[doc(alias = "std::_Vector_base<unsigned int,std::allocator<unsigned int>>::_Vector_base(unsigned long,std::allocator<unsigned int> const&)")]
// was: __ZNSt12_Vector_baseIjSaIjEEC2EmRKS0_
pub fn stub_3dc82c() -> ! {
    todo!("0x3dc82c std::_Vector_base<unsigned int,std::allocator<unsigned int>>::_Vector_base(unsigned long,std::allocator<unsigned int> const&)")
}

// 0x3dc85c — __ZNSt12_Vector_baseIjSaIjEE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<unsigned int,std::allocator<unsigned int>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIjSaIjEE11_M_allocateEm
pub fn stub_3dc85c() -> ! {
    todo!("0x3dc85c std::_Vector_base<unsigned int,std::allocator<unsigned int>>::_M_allocate(unsigned long)")
}

// 0x3dc874 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX20ChangeHistoryService4ItemEEENS3_5list1INS3_5valueIPS9_EEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ChangeHistoryService::Item>,boost::_bi::list1<boost::_bi::value<RBX::ChangeHistoryService::Item*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX20ChangeHistoryService4ItemEEENS3_5list1INS3_5valueIPS9_EEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE
pub fn stub_3dc874() -> ! {
    todo!("0x3dc874 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ChangeHistoryService::Item>,boost::_bi::list1<boost::_bi::value<RBX::ChangeHistoryService::Item*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x3dc8d4 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX20ChangeHistoryService4ItemEEENS3_5list1INS3_5valueIPS9_EEEEEEvE6invokeERNS1_15function_bufferE
// type: int()
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ChangeHistoryService::Item>,boost::_bi::list1<boost::_bi::value<RBX::ChangeHistoryService::Item*>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX20ChangeHistoryService4ItemEEENS3_5list1INS3_5valueIPS9_EEEEEEvE6invokeERNS1_15function_bufferE
pub fn stub_3dc8d4() -> ! {
    todo!("0x3dc8d4 boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ChangeHistoryService::Item>,boost::_bi::list1<boost::_bi::value<RBX::ChangeHistoryService::Item*>>>,void>::invoke(boost::detail::function::function_buffer &)")
}

// 0x3dc8d8 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX20ChangeHistoryService4ItemEEENS0_5list1INS0_5valueIPS6_EEEEEclEv
// type: int __fastcall(int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ChangeHistoryService::Item>,boost::_bi::list1<boost::_bi::value<RBX::ChangeHistoryService::Item*>>>::operator()(void)")]
// was: __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX20ChangeHistoryService4ItemEEENS0_5list1INS0_5valueIPS6_EEEEEclEv
pub fn stub_3dc8d8() -> ! {
    todo!("0x3dc8d8 boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ChangeHistoryService::Item>,boost::_bi::list1<boost::_bi::value<RBX::ChangeHistoryService::Item*>>>::operator()(void)")
}

// 0x3dc8f0 — __ZN5boost9function2IvNS_8functionIFvvEEESsE5dummy7nonnullEv
// type: void()
#[doc(alias = "boost::function2<void,boost::function<void ()(void)>,std::string>::dummy::nonnull(void)")]
// was: __ZN5boost9function2IvNS_8functionIFvvEEESsE5dummy7nonnullEv
pub fn stub_3dc8f0() -> ! {
    todo!("0x3dc8f0 boost::function2<void,boost::function<void ()(void)>,std::string>::dummy::nonnull(void)")
}

// 0x3dc8f4 — __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>> *)")]
// was: __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_3dc8f4() -> ! {
    todo!("0x3dc8f4 std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>> *)")
}

// 0x3dc928 — __ZN3RBX20ChangeHistoryService4Item8addValueERKNS_10Reflection18PropertyDescriptorE
// type: int __fastcall(RBX::ChangeHistoryService::Item *this, const RBX::Reflection::PropertyDescriptor *)
#[doc(alias = "RBX::ChangeHistoryService::Item::addValue(RBX::Reflection::PropertyDescriptor const&)")]
// was: __ZN3RBX20ChangeHistoryService4Item8addValueERKNS_10Reflection18PropertyDescriptorE
pub fn stub_3dc928() -> ! {
    todo!("0x3dc928 RBX::ChangeHistoryService::Item::addValue(RBX::Reflection::PropertyDescriptor const&)")
}

// 0x3dcb74 — __ZNSt3mapIPKN3RBX10Reflection18PropertyDescriptorENS1_7VariantESt4lessIS4_ESaISt4pairIKS4_S5_EEEixERS9_
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "std::map<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::operator[](RBX::Reflection::PropertyDescriptor const* const&)")]
// was: __ZNSt3mapIPKN3RBX10Reflection18PropertyDescriptorENS1_7VariantESt4lessIS4_ESaISt4pairIKS4_S5_EEEixERS9_
pub fn stub_3dcb74() -> ! {
    todo!("0x3dcb74 std::map<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::operator[](RBX::Reflection::PropertyDescriptor const* const&)")
}

// 0x3dccdc — __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: _Rb_tree_node_base *__fastcall(_DWORD *, _Rb_tree_node_base *, unsigned int *)
#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_3dccdc() -> ! {
    todo!("0x3dccdc std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant> const&)")
}

// 0x3dcd90 — __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: _Rb_tree_node_base *__fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_3dcd90() -> ! {
    todo!("0x3dcd90 std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant> const&)")
}

// 0x3dcddc — __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::_M_insert_unique(std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_3dcddc() -> ! {
    todo!("0x3dcddc std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::_M_insert_unique(std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant> const&)")
}

// 0x3dce44 — __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE14_M_create_nodeERKS8_
// type: _DWORD *__fastcall(int, int *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::_M_create_node(std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE14_M_create_nodeERKS8_
pub fn stub_3dce44() -> ! {
    todo!("0x3dce44 std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::_M_create_node(std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant> const&)")
}

// 0x3dcf44 — __ZN3RBX20ChangeHistoryService4Item25recordClusterDataGetChunkEi
// type: char *__fastcall(RBX::ChangeHistoryService::Item *this, void *)
#[doc(alias = "RBX::ChangeHistoryService::Item::recordClusterDataGetChunk(int)")]
// was: __ZN3RBX20ChangeHistoryService4Item25recordClusterDataGetChunkEi
pub fn stub_3dcf44() -> ! {
    todo!("0x3dcf44 RBX::ChangeHistoryService::Item::recordClusterDataGetChunk(int)")
}

// 0x3dd084 — __ZNSt3mapIjSt6vectorIjSaIjEESt4lessIjESaISt4pairIKjS2_EEEixERS6_
// type: int __fastcall(int, int *, int, int, int, int, int, void *, int, int, int, int, int, int)
#[doc(alias = "std::map<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::operator[](unsigned int const&)")]
// was: __ZNSt3mapIjSt6vectorIjSaIjEESt4lessIjESaISt4pairIKjS2_EEEixERS6_
pub fn stub_3dd084() -> ! {
    todo!("0x3dd084 std::map<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::operator[](unsigned int const&)")
}

// 0x3dd1a4 — __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: _Rb_tree_node_base *__fastcall(_DWORD *, _Rb_tree_node_base *, unsigned int *)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>> const&)")]
// was: __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
pub fn stub_3dd1a4() -> ! {
    todo!("0x3dd1a4 std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>> const&)")
}

// 0x3dd258 — __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
// type: _Rb_tree_node_base *__fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>> const&)")]
// was: __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
pub fn stub_3dd258() -> ! {
    todo!("0x3dd258 std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>> const&)")
}

// 0x3dd2a4 — __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_insert_unique(std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>> const&)")]
// was: __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE16_M_insert_uniqueERKS5_
pub fn stub_3dd2a4() -> ! {
    todo!("0x3dd2a4 std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_insert_unique(std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>> const&)")
}

// 0x3dd30c — __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE14_M_create_nodeERKS5_
// type: int __fastcall(int, _DWORD *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_create_node(std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>> const&)")]
// was: __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE14_M_create_nodeERKS5_
pub fn stub_3dd30c() -> ! {
    todo!("0x3dd30c std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_create_node(std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>> const&)")
}

// 0x3dd3f0 — __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE4findERS1_
// type: _DWORD *__fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::find(unsigned int const&)")]
// was: __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE4findERS1_
pub fn stub_3dd3f0() -> ! {
    todo!("0x3dd3f0 std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::find(unsigned int const&)")
}

// 0x3dd430 — __ZNK3RBX5Voxel6RegionINS0_4Grid5ChunkEEeqERKS4_
// type: bool __fastcall(int, int)
#[doc(alias = "RBX::Voxel::Region<RBX::Voxel::Grid::Chunk>::operator==(RBX::Voxel::Region<RBX::Voxel::Grid::Chunk> const&)const")]
// was: __ZNK3RBX5Voxel6RegionINS0_4Grid5ChunkEEeqERKS4_
pub fn stub_3dd430() -> ! {
    todo!("0x3dd430 RBX::Voxel::Region<RBX::Voxel::Grid::Chunk>::operator==(RBX::Voxel::Region<RBX::Voxel::Grid::Chunk> const&)const")
}

// 0x3dd488 — __ZSt8for_eachIN3RBX10Reflection25MemberDescriptorContainerINS1_18PropertyDescriptorEE8IteratorEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvNS0_20ChangeHistoryService4ItemERKNS1_8PropertyEEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEEET0_T_SQ_SP_
// type: _DWORD *__fastcall(_DWORD *, RBX::Reflection::DescribedBase *, RBX::Reflection::MemberDescriptor **, int, RBX::Reflection::MemberDescriptor **, char *, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,RBX::Reflection::Property const&>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>> std::for_each<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,RBX::Reflection::Property const&>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>>(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::Iterator,RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,RBX::Reflection::Property const&>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>)")]
// was: __ZSt8for_eachIN3RBX10Reflection25MemberDescriptorContainerINS1_18PropertyDescriptorEE8IteratorEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvNS0_20ChangeHistoryService4ItemERKNS1_8PropertyEEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEEET0_T_SQ_SP_
pub fn stub_3dd488() -> ! {
    todo!("0x3dd488 boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,RBX::Reflection::Property const&>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>> std::for_each<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,RBX::Reflection::Property const&>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>>(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::Iterator,RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,RBX::Reflection::Property const&>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>)")
}

// 0x3dd54c — __ZN3RBX20ChangeHistoryService4Item27addValueIfNotParentPropertyERKNS_10Reflection8PropertyE
// type: int __fastcall(int, void **)
#[doc(alias = "RBX::ChangeHistoryService::Item::addValueIfNotParentProperty(RBX::Reflection::Property const&)")]
// was: __ZN3RBX20ChangeHistoryService4Item27addValueIfNotParentPropertyERKNS_10Reflection8PropertyE
pub fn stub_3dd54c() -> ! {
    todo!("0x3dd54c RBX::ChangeHistoryService::Item::addValueIfNotParentProperty(RBX::Reflection::Property const&)")
}

// 0x3dd564 — __ZNSt3mapIPN3RBX8InstanceEjSt4lessIS2_ESaISt4pairIKS2_jEEEixERS6_
// type: _Rb_tree_node_base **__fastcall(int, int *)
#[doc(alias = "std::map<RBX::Instance *,unsigned int,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,unsigned int>>>::operator[](RBX::Instance * const&)")]
// was: __ZNSt3mapIPN3RBX8InstanceEjSt4lessIS2_ESaISt4pairIKS2_jEEEixERS6_
pub fn stub_3dd564() -> ! {
    todo!("0x3dd564 std::map<RBX::Instance *,unsigned int,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,unsigned int>>>::operator[](RBX::Instance * const&)")
}

// 0x3dd5bc — __ZNSt8_Rb_treeIPN3RBX8InstanceESt4pairIKS2_jESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::Instance *,std::pair<RBX::Instance * const,unsigned int>,std::_Select1st<std::pair<RBX::Instance * const,unsigned int>>,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,unsigned int>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Instance * const,unsigned int>>,std::pair<RBX::Instance * const,unsigned int> const&)")]
// was: __ZNSt8_Rb_treeIPN3RBX8InstanceESt4pairIKS2_jESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
pub fn stub_3dd5bc() -> ! {
    todo!("0x3dd5bc std::_Rb_tree<RBX::Instance *,std::pair<RBX::Instance * const,unsigned int>,std::_Select1st<std::pair<RBX::Instance * const,unsigned int>>,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,unsigned int>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Instance * const,unsigned int>>,std::pair<RBX::Instance * const,unsigned int> const&)")
}
