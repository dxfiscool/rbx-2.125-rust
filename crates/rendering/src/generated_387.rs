//! rendering shard 387 — 100 stubs 0x56d6ac..0x573744 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn 15618/15618 complete, 41910->42010 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Filter Ogre|G3D|Gfx|Render|Adorn 15618/15618 filtered complete; this batch is pure gap filler EA asc not yet in rbx_rendering.
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc gap not yet in rbx_rendering 0x56d6ac..0x573744 (100 stubs)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x56d6ac — __ZNK5boost6detail8function13basic_vtable1IvN3RBX8NormalIdEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvN3RBX8NormalIdEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::NormalId>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvN3RBX8NormalIdEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
pub fn stub_56d6ac() -> ! {
    todo!("0x56d6ac void boost::detail::function::basic_vtable1<void,RBX::NormalId>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x56d780 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS4_8NormalIdEEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIS7_EEvRT_
// type: int(void)
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS4_8NormalIdEEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIS7_EEvRT_")]
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>::operator()<RBX::NormalId>(RBX::NormalId &)")]
// was: __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS4_8NormalIdEEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIS7_EEvRT_
pub fn stub_56d780() -> ! {
    todo!("0x56d780 void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>::operator()<RBX::NormalId>(RBX::NormalId &)")
}

// 0x56d798 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_8NormalIdEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_8NormalIdEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_8NormalIdEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_56d798() -> ! {
    todo!("0x56d798 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x56d8f0 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int, int)
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_")]
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::NormalId)>::connect<boost::function<void ()(RBX::NormalId)>>(boost::function<void ()(RBX::NormalId)> const&)")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_
pub fn stub_56d8f0() -> ! {
    todo!("0x56d8f0 rbx::signals::connection rbx::signals::signal<void ()(RBX::NormalId)>::connect<boost::function<void ()(RBX::NormalId)>>(boost::function<void ()(RBX::NormalId)> const&)")
}

// 0x56d9e4 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,boost::function<void ()(RBX::NormalId)>,1,void ()(RBX::NormalId)>::callable<rbx::signals::signal<void ()(RBX::NormalId)>*>(boost::function<void ()(RBX::NormalId)> const&,rbx::signals::signal<void ()(RBX::NormalId)>*)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_
pub fn stub_56d9e4() -> ! {
    todo!("0x56d9e4 rbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,boost::function<void ()(RBX::NormalId)>,1,void ()(RBX::NormalId)>::callable<rbx::signals::signal<void ()(RBX::NormalId)>*>(boost::function<void ()(RBX::NormalId)> const&,rbx::signals::signal<void ()(RBX::NormalId)>*)")
}

// 0x56dae0 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13callable_slotIN5boost8functionIS4_EEED1Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13callable_slotIN5boost8functionIS4_EEED1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::callable_slot<boost::function<void ()(RBX::NormalId)>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13callable_slotIN5boost8functionIS4_EEED1Ev
pub fn stub_56dae0() -> ! {
    todo!("0x56dae0 rbx::signals::signal<void ()(RBX::NormalId)>::callable_slot<boost::function<void ()(RBX::NormalId)>>::~callable_slot()")
}

// 0x56dbf0 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13callable_slotIN5boost8functionIS4_EEED0Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13callable_slotIN5boost8functionIS4_EEED0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::callable_slot<boost::function<void ()(RBX::NormalId)>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13callable_slotIN5boost8functionIS4_EEED0Ev
pub fn stub_56dbf0() -> ! {
    todo!("0x56dbf0 rbx::signals::signal<void ()(RBX::NormalId)>::callable_slot<boost::function<void ()(RBX::NormalId)>>::~callable_slot()")
}

// 0x56dd20 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,boost::function<void ()(RBX::NormalId)>,1,void ()(RBX::NormalId)>::call(RBX::NormalId)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_
pub fn stub_56dd20() -> ! {
    todo!("0x56dd20 rbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,boost::function<void ()(RBX::NormalId)>,1,void ()(RBX::NormalId)>::call(RBX::NormalId)")
}

// 0x56dd28 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_")]
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,boost::function<void ()(RBX::NormalId)>,1,void ()(RBX::NormalId)>::call(RBX::NormalId)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_
pub fn stub_56dd28() -> ! {
    todo!("0x56dd28 non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,boost::function<void ()(RBX::NormalId)>,1,void ()(RBX::NormalId)>::call(RBX::NormalId)")
}

// 0x56dd30 — __ZNK5boost9function1IvN3RBX8NormalIdEEclES2_
// type: int(void)
#[doc(alias = "__ZNK5boost9function1IvN3RBX8NormalIdEEclES2_")]
#[doc(alias = "boost::function1<void,RBX::NormalId>::operator()(RBX::NormalId)const")]
// was: __ZNK5boost9function1IvN3RBX8NormalIdEEclES2_
pub fn stub_56dd30() -> ! {
    todo!("0x56dd30 boost::function1<void,RBX::NormalId>::operator()(RBX::NormalId)const")
}

// 0x56ddf4 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_ED1Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_ED1Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,boost::function<void ()(RBX::NormalId)>,1,void ()(RBX::NormalId)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_ED1Ev
pub fn stub_56ddf4() -> ! {
    todo!("0x56ddf4 rbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,boost::function<void ()(RBX::NormalId)>,1,void ()(RBX::NormalId)>::~callable()")
}

// 0x56df04 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_ED0Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_ED0Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,boost::function<void ()(RBX::NormalId)>,1,void ()(RBX::NormalId)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_ED0Ev
pub fn stub_56df04() -> ! {
    todo!("0x56df04 rbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,boost::function<void ()(RBX::NormalId)>,1,void ()(RBX::NormalId)>::~callable()")
}

// 0x56e034 — __ZN5boost9function1IvN3RBX8NormalIdEE13assign_to_ownERKS3_
// type: int(void)
#[doc(alias = "__ZN5boost9function1IvN3RBX8NormalIdEE13assign_to_ownERKS3_")]
#[doc(alias = "boost::function1<void,RBX::NormalId>::assign_to_own(boost::function1<void,RBX::NormalId> const&)")]
// was: __ZN5boost9function1IvN3RBX8NormalIdEE13assign_to_ownERKS3_
pub fn stub_56e034() -> ! {
    todo!("0x56e034 boost::function1<void,RBX::NormalId>::assign_to_own(boost::function1<void,RBX::NormalId> const&)")
}

// 0x56e064 — __ZN3RBX10Reflection9EventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEMS2_S7_EC2ES8_PKcSB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEMS2_S7_EC2ES8_PKcSB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>,rbx::remote_signal<void ()(RBX::NormalId)> RBX::Handles::*>::EventDesc(rbx::remote_signal<void ()(RBX::NormalId)> RBX::Handles::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection9EventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEMS2_S7_EC2ES8_PKcSB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_56e064() -> ! {
    todo!("0x56e064 RBX::Reflection::EventDesc<RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>,rbx::remote_signal<void ()(RBX::NormalId)> RBX::Handles::*>::EventDesc(rbx::remote_signal<void ()(RBX::NormalId)> RBX::Handles::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x56e1e8 — __ZN3RBX10Reflection9EventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEMS2_S7_ED1Ev
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEMS2_S7_ED1Ev")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>,rbx::remote_signal<void ()(RBX::NormalId)> RBX::Handles::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEMS2_S7_ED1Ev
pub fn stub_56e1e8() -> ! {
    todo!("0x56e1e8 RBX::Reflection::EventDesc<RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>,rbx::remote_signal<void ()(RBX::NormalId)> RBX::Handles::*>::~EventDesc()")
}

// 0x56e20c — __ZN3RBX10Reflection9EventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEMS2_S7_ED0Ev
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEMS2_S7_ED0Ev")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>,rbx::remote_signal<void ()(RBX::NormalId)> RBX::Handles::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEMS2_S7_ED0Ev
pub fn stub_56e20c() -> ! {
    todo!("0x56e20c RBX::Reflection::EventDesc<RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>,rbx::remote_signal<void ()(RBX::NormalId)> RBX::Handles::*>::~EventDesc()")
}

// 0x56e2c0 — __ZN3RBX10Reflection14PropDescriptorINS_7HandlesENS_5FacesEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7HandlesENS_5FacesEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Faces>::PropDescriptor<RBX::Faces (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Faces)>(char const*,char const*,RBX::Faces (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Faces),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_7HandlesENS_5FacesEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_56e2c0() -> ! {
    todo!("0x56e2c0 RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Faces>::PropDescriptor<RBX::Faces (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Faces)>(char const*,char const*,RBX::Faces (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Faces),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x56e3d4 — __ZN3RBX10Reflection14PropDescriptorINS_7HandlesENS_5FacesEED0Ev
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7HandlesENS_5FacesEED0Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Faces>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_7HandlesENS_5FacesEED0Ev
pub fn stub_56e3d4() -> ! {
    todo!("0x56e3d4 RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Faces>::~PropDescriptor()")
}

// 0x56e400 — __ZNK3RBX10Reflection14PropDescriptorINS_7HandlesENS_5FacesEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7HandlesENS_5FacesEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Faces>::GetSetImpl<RBX::Faces (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Faces)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7HandlesENS_5FacesEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
pub fn stub_56e400() -> ! {
    todo!("0x56e400 RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Faces>::GetSetImpl<RBX::Faces (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Faces)>::isReadOnly(void)const")
}

// 0x56e404 — __ZNK3RBX10Reflection14PropDescriptorINS_7HandlesENS_5FacesEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7HandlesENS_5FacesEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Faces>::GetSetImpl<RBX::Faces (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Faces)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7HandlesENS_5FacesEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
pub fn stub_56e404() -> ! {
    todo!("0x56e404 RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Faces>::GetSetImpl<RBX::Faces (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Faces)>::isWriteOnly(void)const")
}

// 0x56e408 — __ZNK3RBX10Reflection14PropDescriptorINS_7HandlesENS_5FacesEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7HandlesENS_5FacesEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Faces>::GetSetImpl<RBX::Faces (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Faces)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7HandlesENS_5FacesEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
pub fn stub_56e408() -> ! {
    todo!("0x56e408 RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Faces>::GetSetImpl<RBX::Faces (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Faces)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x56e428 — __ZNK3RBX10Reflection14PropDescriptorINS_7HandlesENS_5FacesEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7HandlesENS_5FacesEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Faces>::GetSetImpl<RBX::Faces (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Faces)>::setValue(RBX::Reflection::DescribedBase *,RBX::Faces const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7HandlesENS_5FacesEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
pub fn stub_56e428() -> ! {
    todo!("0x56e428 RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Faces>::GetSetImpl<RBX::Faces (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Faces)>::setValue(RBX::Reflection::DescribedBase *,RBX::Faces const&)const")
}

// 0x56e44c — __ZN3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::EnumPropDescriptor<RBX::Handles::VisualStyle (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Handles::VisualStyle)>(char const*,char const*,RBX::Handles::VisualStyle (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Handles::VisualStyle),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_56e44c() -> ! {
    todo!("0x56e44c RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::EnumPropDescriptor<RBX::Handles::VisualStyle (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Handles::VisualStyle)>(char const*,char const*,RBX::Handles::VisualStyle (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Handles::VisualStyle),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x56e600 — __ZN3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEED0Ev
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEED0Ev")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEED0Ev
pub fn stub_56e600() -> ! {
    todo!("0x56e600 RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::~EnumPropDescriptor()")
}

// 0x56e62c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE10isReadOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE10isReadOnlyEv
pub fn stub_56e62c() -> ! {
    todo!("0x56e62c RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::isReadOnly(void)const")
}

// 0x56e63c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE11isWriteOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE11isWriteOnlyEv
pub fn stub_56e63c() -> ! {
    todo!("0x56e63c RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::isWriteOnly(void)const")
}

// 0x56e64c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE11equalValuesEPKNS0_13DescribedBaseES7_")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE11equalValuesEPKNS0_13DescribedBaseES7_
pub fn stub_56e64c() -> ! {
    todo!("0x56e64c RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

// 0x56e674 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
pub fn stub_56e674() -> ! {
    todo!("0x56e674 RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}

// 0x56e698 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
pub fn stub_56e698() -> ! {
    todo!("0x56e698 RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}

// 0x56e7e4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE9copyValueEPKNS0_13DescribedBaseEPS5_")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE9copyValueEPKNS0_13DescribedBaseEPS5_
pub fn stub_56e7e4() -> ! {
    todo!("0x56e7e4 RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}

// 0x56e808 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE14hasStringValueEv
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE14hasStringValueEv")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::hasStringValue(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE14hasStringValueEv
pub fn stub_56e808() -> ! {
    todo!("0x56e808 RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::hasStringValue(void)const")
}

// 0x56e80c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE14getStringValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE14getStringValueEPKNS0_13DescribedBaseE
pub fn stub_56e80c() -> ! {
    todo!("0x56e80c RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::getStringValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x56e830 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE14setStringValueEPNS0_13DescribedBaseERKSs")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE14setStringValueEPNS0_13DescribedBaseERKSs
pub fn stub_56e830() -> ! {
    todo!("0x56e830 RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x56e870 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
pub fn stub_56e870() -> ! {
    todo!("0x56e870 RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

// 0x56e890 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
pub fn stub_56e890() -> ! {
    todo!("0x56e890 RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

// 0x56ead0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE13getIndexValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE13getIndexValueEPKNS0_13DescribedBaseE
pub fn stub_56ead0() -> ! {
    todo!("0x56ead0 RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::getIndexValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x56eaec — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE13setIndexValueEPNS0_13DescribedBaseEm")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE13setIndexValueEPNS0_13DescribedBaseEm
pub fn stub_56eaec() -> ! {
    todo!("0x56eaec RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")
}

// 0x56eb20 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE12getEnumValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE12getEnumValueEPKNS0_13DescribedBaseE
pub fn stub_56eb20() -> ! {
    todo!("0x56eb20 RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::getEnumValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x56eb28 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE12setEnumValueEPNS0_13DescribedBaseEi")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE12setEnumValueEPNS0_13DescribedBaseEi
pub fn stub_56eb28() -> ! {
    todo!("0x56eb28 RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x56eb74 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE11getEnumItemEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE11getEnumItemEPKNS0_13DescribedBaseE
pub fn stub_56eb74() -> ! {
    todo!("0x56eb74 RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::getEnumItem(RBX::Reflection::DescribedBase const*)const")
}

// 0x56eb94 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
pub fn stub_56eb94() -> ! {
    todo!("0x56eb94 RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")
}

// 0x56ebc8 — __ZNK3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEE14convertToIndexES3_
// type: int(void)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEE14convertToIndexES3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle>::convertToIndex(RBX::Handles::VisualStyle)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEE14convertToIndexES3_
pub fn stub_56ebc8() -> ! {
    todo!("0x56ebc8 RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle>::convertToIndex(RBX::Handles::VisualStyle)const")
}

// 0x56ec38 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE11setIntValueEPNS0_13DescribedBaseEi
// type: int(void)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE11setIntValueEPNS0_13DescribedBaseEi")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_7HandlesENS2_11VisualStyleEE11setIntValueEPNS0_13DescribedBaseEi
pub fn stub_56ec38() -> ! {
    todo!("0x56ec38 RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x56ec78 — __ZNK3RBX10Reflection14PropDescriptorINS_7HandlesENS2_11VisualStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7HandlesENS2_11VisualStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::GetSetImpl<RBX::Handles::VisualStyle (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Handles::VisualStyle)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7HandlesENS2_11VisualStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
pub fn stub_56ec78() -> ! {
    todo!("0x56ec78 RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::GetSetImpl<RBX::Handles::VisualStyle (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Handles::VisualStyle)>::isReadOnly(void)const")
}

// 0x56ec7c — __ZNK3RBX10Reflection14PropDescriptorINS_7HandlesENS2_11VisualStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7HandlesENS2_11VisualStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::GetSetImpl<RBX::Handles::VisualStyle (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Handles::VisualStyle)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7HandlesENS2_11VisualStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
pub fn stub_56ec7c() -> ! {
    todo!("0x56ec7c RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::GetSetImpl<RBX::Handles::VisualStyle (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Handles::VisualStyle)>::isWriteOnly(void)const")
}

// 0x56ec80 — __ZNK3RBX10Reflection14PropDescriptorINS_7HandlesENS2_11VisualStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7HandlesENS2_11VisualStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::GetSetImpl<RBX::Handles::VisualStyle (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Handles::VisualStyle)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7HandlesENS2_11VisualStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
pub fn stub_56ec80() -> ! {
    todo!("0x56ec80 RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::GetSetImpl<RBX::Handles::VisualStyle (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Handles::VisualStyle)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x56eca0 — __ZNK3RBX10Reflection14PropDescriptorINS_7HandlesENS2_11VisualStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7HandlesENS2_11VisualStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::GetSetImpl<RBX::Handles::VisualStyle (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Handles::VisualStyle)>::setValue(RBX::Reflection::DescribedBase *,RBX::Handles::VisualStyle const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7HandlesENS2_11VisualStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
pub fn stub_56eca0() -> ! {
    todo!("0x56eca0 RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::GetSetImpl<RBX::Handles::VisualStyle (RBX::Handles::*)(void)const,void (RBX::Handles::*)(RBX::Handles::VisualStyle)>::setValue(RBX::Reflection::DescribedBase *,RBX::Handles::VisualStyle const&)const")
}

// 0x56ecc4 — __ZN3RBX7HandlesD2Ev
// type: void __fastcall(RBX::Handles *__hidden this)
#[doc(alias = "__ZN3RBX7HandlesD2Ev")]
#[doc(alias = "RBX::Handles::~Handles()")]
// was: __ZN3RBX7HandlesD2Ev
pub fn stub_56ecc4() -> ! {
    todo!("0x56ecc4 RBX::Handles::~Handles()")
}

// 0x56eef8 — __ZN3rbx13remote_signalIFvN3RBX8NormalIdEfEED2Ev
// type: int __fastcall(int, int, int, int, char, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvN3RBX8NormalIdEfEED2Ev")]
#[doc(alias = "rbx::remote_signal<void ()(RBX::NormalId,float)>::~remote_signal()")]
// was: __ZN3rbx13remote_signalIFvN3RBX8NormalIdEfEED2Ev
pub fn stub_56eef8() -> ! {
    todo!("0x56eef8 rbx::remote_signal<void ()(RBX::NormalId,float)>::~remote_signal()")
}

// 0x56f044 — __ZN3rbx13remote_signalIFvN3RBX8NormalIdEEED2Ev
// type: int __fastcall(int, int, int, int, char, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvN3RBX8NormalIdEEED2Ev")]
#[doc(alias = "rbx::remote_signal<void ()(RBX::NormalId)>::~remote_signal()")]
// was: __ZN3rbx13remote_signalIFvN3RBX8NormalIdEEED2Ev
pub fn stub_56f044() -> ! {
    todo!("0x56f044 rbx::remote_signal<void ()(RBX::NormalId)>::~remote_signal()")
}

// 0x56f190 — __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEfEED2Ev
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEfEED2Ev")]
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>::~EventReplicatorBase()")]
// was: __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEfEED2Ev
pub fn stub_56f190() -> ! {
    todo!("0x56f190 RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>::~EventReplicatorBase()")
}

// 0x56f2c0 — __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEED2Ev
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEED2Ev")]
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>::~EventReplicatorBase()")]
// was: __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEED2Ev
pub fn stub_56f2c0() -> ! {
    todo!("0x56f2c0 RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>::~EventReplicatorBase()")
}

// 0x56f3f0 — __GLOBAL__I_a_210
#[doc(alias = "__GLOBAL__I_a_210")]
#[doc(alias = "global constructor keyed to_a_210")]
// was: __GLOBAL__I_a_210
pub fn stub_56f3f0() -> ! {
    todo!("0x56f3f0 global constructor keyed to _a_210")
}

// 0x56f9fc — __ZN3RBX11HandlesBaseC2EPKc
// type: _DWORD __fastcall(RBX::HandlesBase *__hidden this, const char *)
#[doc(alias = "__ZN3RBX11HandlesBaseC2EPKc")]
#[doc(alias = "RBX::HandlesBase::HandlesBase(char const*)")]
// was: __ZN3RBX11HandlesBaseC2EPKc
pub fn stub_56f9fc() -> ! {
    todo!("0x56f9fc RBX::HandlesBase::HandlesBase(char const*)")
}

// 0x5703e0 — __ZNK3RBX11HandlesBase26canProcessMeAndDescendantsEv
// type: _DWORD __fastcall(RBX::HandlesBase *__hidden this)
#[doc(alias = "__ZNK3RBX11HandlesBase26canProcessMeAndDescendantsEv")]
#[doc(alias = "RBX::HandlesBase::canProcessMeAndDescendants(void)const")]
// was: __ZNK3RBX11HandlesBase26canProcessMeAndDescendantsEv
pub fn stub_5703e0() -> ! {
    todo!("0x5703e0 RBX::HandlesBase::canProcessMeAndDescendants(void)const")
}

// 0x5707a4 — __ZN3RBX11HandlesBase18setServerGuiObjectEv
// type: _DWORD __fastcall(RBX::HandlesBase *__hidden this)
#[doc(alias = "__ZN3RBX11HandlesBase18setServerGuiObjectEv")]
#[doc(alias = "RBX::HandlesBase::setServerGuiObject(void)")]
// was: __ZN3RBX11HandlesBase18setServerGuiObjectEv
pub fn stub_5707a4() -> ! {
    todo!("0x5707a4 RBX::HandlesBase::setServerGuiObject(void)")
}

// 0x5707ac — __ZN3RBX11HandlesBase17onAncestorChangedERKNS_15AncestorChangedE
#[doc(alias = "__ZN3RBX11HandlesBase17onAncestorChangedERKNS_15AncestorChangedE")]
#[doc(alias = "RBX::HandlesBase::onAncestorChanged(RBX::AncestorChanged const&)")]
// was: __ZN3RBX11HandlesBase17onAncestorChangedERKNS_15AncestorChangedE
pub fn stub_5707ac() -> ! {
    todo!("0x5707ac RBX::HandlesBase::onAncestorChanged(RBX::AncestorChanged const&)")
}

// 0x570f0c — __GLOBAL__I_a_211
#[doc(alias = "__GLOBAL__I_a_211")]
#[doc(alias = "global constructor keyed to_a_211")]
// was: __GLOBAL__I_a_211
pub fn stub_570f0c() -> ! {
    todo!("0x570f0c global constructor keyed to _a_211")
}

// 0x57117c — __ZN3RBX10Reflection8EnumDescINS_9HopperBin7BinTypeEEC1Ev
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9HopperBin7BinTypeEEC1Ev")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::HopperBin::BinType>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_9HopperBin7BinTypeEEC1Ev
pub fn stub_57117c() -> ! {
    todo!("0x57117c RBX::Reflection::EnumDesc<RBX::HopperBin::BinType>::EnumDesc(void)")
}

// 0x571180 — __ZN3RBX10Reflection8EnumDescINS_9HopperBin7BinTypeEEC2Ev
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9HopperBin7BinTypeEEC2Ev")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::HopperBin::BinType>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_9HopperBin7BinTypeEEC2Ev
pub fn stub_571180() -> ! {
    todo!("0x571180 RBX::Reflection::EnumDesc<RBX::HopperBin::BinType>::EnumDesc(void)")
}

// 0x5713d0 — __ZNK3RBX12BackpackItem12getTextureIdEv
// type: _DWORD __fastcall(RBX::BackpackItem *__hidden this)
#[doc(alias = "__ZNK3RBX12BackpackItem12getTextureIdEv")]
#[doc(alias = "RBX::BackpackItem::getTextureId(void)const")]
// was: __ZNK3RBX12BackpackItem12getTextureIdEv
pub fn stub_5713d0() -> ! {
    todo!("0x5713d0 RBX::BackpackItem::getTextureId(void)const")
}

// 0x5713e8 — __ZN3RBX12BackpackItem12setTextureIdERKNS_9TextureIdE
// type: int __fastcall(RBX::BackpackItem *this, const RBX::TextureId *)
#[doc(alias = "__ZN3RBX12BackpackItem12setTextureIdERKNS_9TextureIdE")]
#[doc(alias = "RBX::BackpackItem::setTextureId(RBX::TextureId const&)")]
// was: __ZN3RBX12BackpackItem12setTextureIdERKNS_9TextureIdE
pub fn stub_5713e8() -> ! {
    todo!("0x5713e8 RBX::BackpackItem::setTextureId(RBX::TextureId const&)")
}

// 0x571428 — __ZN3RBX9HopperBin10setBinTypeENS0_7BinTypeE
#[doc(alias = "__ZN3RBX9HopperBin10setBinTypeENS0_7BinTypeE")]
#[doc(alias = "RBX::HopperBin::setBinType(RBX::HopperBin::BinType)")]
// was: __ZN3RBX9HopperBin10setBinTypeENS0_7BinTypeE
pub fn stub_571428() -> ! {
    todo!("0x571428 RBX::HopperBin::setBinType(RBX::HopperBin::BinType)")
}

// 0x5715a8 — __ZN3RBX9HopperBin11dataChangedERKNS_10Reflection18PropertyDescriptorE
// type: _DWORD __fastcall(RBX::HopperBin *__hidden this, const RBX::Reflection::PropertyDescriptor *)
#[doc(alias = "__ZN3RBX9HopperBin11dataChangedERKNS_10Reflection18PropertyDescriptorE")]
#[doc(alias = "RBX::HopperBin::dataChanged(RBX::Reflection::PropertyDescriptor const&)")]
// was: __ZN3RBX9HopperBin11dataChangedERKNS_10Reflection18PropertyDescriptorE
pub fn stub_5715a8() -> ! {
    todo!("0x5715a8 RBX::HopperBin::dataChanged(RBX::Reflection::PropertyDescriptor const&)")
}

// 0x5715ac — __ZN3RBX9HopperBin7disableEv
// type: _DWORD __fastcall(RBX::HopperBin *__hidden this)
#[doc(alias = "__ZN3RBX9HopperBin7disableEv")]
#[doc(alias = "RBX::HopperBin::disable(void)")]
// was: __ZN3RBX9HopperBin7disableEv
pub fn stub_5715ac() -> ! {
    todo!("0x5715ac RBX::HopperBin::disable(void)")
}

// 0x5715f8 — __ZN3RBX9HopperBin16setLegacyCommandERKSs
// type: _DWORD __fastcall(RBX::HopperBin *__hidden this, const std::string *)
#[doc(alias = "__ZN3RBX9HopperBin16setLegacyCommandERKSs")]
#[doc(alias = "RBX::HopperBin::setLegacyCommand(std::string const&)")]
// was: __ZN3RBX9HopperBin16setLegacyCommandERKSs
pub fn stub_5715f8() -> ! {
    todo!("0x5715f8 RBX::HopperBin::setLegacyCommand(std::string const&)")
}

// 0x571654 — __ZN3RBX9HopperBin20setLegacyTextureNameERKSs
// type: _DWORD __fastcall(RBX::HopperBin *__hidden this, const std::string *)
#[doc(alias = "__ZN3RBX9HopperBin20setLegacyTextureNameERKSs")]
#[doc(alias = "RBX::HopperBin::setLegacyTextureName(std::string const&)")]
// was: __ZN3RBX9HopperBin20setLegacyTextureNameERKSs
pub fn stub_571654() -> ! {
    todo!("0x571654 RBX::HopperBin::setLegacyTextureName(std::string const&)")
}

// 0x57195c — __ZN3RBX11StarterGearC1Ev
// type: _DWORD __fastcall(RBX::StarterGear *__hidden this)
#[doc(alias = "__ZN3RBX11StarterGearC1Ev")]
#[doc(alias = "RBX::StarterGear::StarterGear(void)")]
// was: __ZN3RBX11StarterGearC1Ev
pub fn stub_57195c() -> ! {
    todo!("0x57195c RBX::StarterGear::StarterGear(void)")
}

// 0x571960 — __ZN3RBX11StarterGearC2Ev
// type: _DWORD __fastcall(RBX::StarterGear *__hidden this)
#[doc(alias = "__ZN3RBX11StarterGearC2Ev")]
#[doc(alias = "RBX::StarterGear::StarterGear(void)")]
// was: __ZN3RBX11StarterGearC2Ev
pub fn stub_571960() -> ! {
    todo!("0x571960 RBX::StarterGear::StarterGear(void)")
}

// 0x571b54 — __ZNK3RBX11StarterGear12askSetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::StarterGear *__hidden this, const RBX::Instance *)
#[doc(alias = "__ZNK3RBX11StarterGear12askSetParentEPKNS_8InstanceE")]
#[doc(alias = "RBX::StarterGear::askSetParent(RBX::Instance const*)const")]
// was: __ZNK3RBX11StarterGear12askSetParentEPKNS_8InstanceE
pub fn stub_571b54() -> ! {
    todo!("0x571b54 RBX::StarterGear::askSetParent(RBX::Instance const*)const")
}

// 0x571b58 — __ZNK3RBX11StarterGear11askAddChildEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::StarterGear *__hidden this, const RBX::Instance *)
#[doc(alias = "__ZNK3RBX11StarterGear11askAddChildEPKNS_8InstanceE")]
#[doc(alias = "RBX::StarterGear::askAddChild(RBX::Instance const*)const")]
// was: __ZNK3RBX11StarterGear11askAddChildEPKNS_8InstanceE
pub fn stub_571b58() -> ! {
    todo!("0x571b58 RBX::StarterGear::askAddChild(RBX::Instance const*)const")
}

// 0x571b94 — __ZN3RBX12BackpackItem7setNameERKSs
// type: _DWORD __fastcall(RBX::BackpackItem *__hidden this, const std::string *)
#[doc(alias = "__ZN3RBX12BackpackItem7setNameERKSs")]
#[doc(alias = "RBX::BackpackItem::setName(std::string const&)")]
// was: __ZN3RBX12BackpackItem7setNameERKSs
pub fn stub_571b94() -> ! {
    todo!("0x571b94 RBX::BackpackItem::setName(std::string const&)")
}

// 0x571bb4 — __ZNK3RBX12BackpackItem8getBinIdEv
// type: _DWORD __fastcall(RBX::BackpackItem *__hidden this)
#[doc(alias = "__ZNK3RBX12BackpackItem8getBinIdEv")]
#[doc(alias = "RBX::BackpackItem::getBinId(void)const")]
// was: __ZNK3RBX12BackpackItem8getBinIdEv
pub fn stub_571bb4() -> ! {
    todo!("0x571bb4 RBX::BackpackItem::getBinId(void)const")
}

// 0x571c18 — __ZN3RBX12BackpackItem10inBackpackEv
// type: _DWORD __fastcall(RBX::BackpackItem *__hidden this)
#[doc(alias = "__ZN3RBX12BackpackItem10inBackpackEv")]
#[doc(alias = "RBX::BackpackItem::inBackpack(void)")]
// was: __ZN3RBX12BackpackItem10inBackpackEv
pub fn stub_571c18() -> ! {
    todo!("0x571c18 RBX::BackpackItem::inBackpack(void)")
}

// 0x571c54 — __ZNK3RBX12BackpackItem11askAddChildEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::BackpackItem *__hidden this, const RBX::Instance *)
#[doc(alias = "__ZNK3RBX12BackpackItem11askAddChildEPKNS_8InstanceE")]
#[doc(alias = "RBX::BackpackItem::askAddChild(RBX::Instance const*)const")]
// was: __ZNK3RBX12BackpackItem11askAddChildEPKNS_8InstanceE
pub fn stub_571c54() -> ! {
    todo!("0x571c54 RBX::BackpackItem::askAddChild(RBX::Instance const*)const")
}

// 0x571c58 — __ZNK3RBX12BackpackItem12askSetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::BackpackItem *__hidden this, const RBX::Instance *)
#[doc(alias = "__ZNK3RBX12BackpackItem12askSetParentEPKNS_8InstanceE")]
#[doc(alias = "RBX::BackpackItem::askSetParent(RBX::Instance const*)const")]
// was: __ZNK3RBX12BackpackItem12askSetParentEPKNS_8InstanceE
pub fn stub_571c58() -> ! {
    todo!("0x571c58 RBX::BackpackItem::askSetParent(RBX::Instance const*)const")
}

// 0x571c5c — __ZNK3RBX12BackpackItem7getSizeENS_6CanvasE
#[doc(alias = "__ZNK3RBX12BackpackItem7getSizeENS_6CanvasE")]
#[doc(alias = "RBX::BackpackItem::getSize(RBX::Canvas)const")]
// was: __ZNK3RBX12BackpackItem7getSizeENS_6CanvasE
pub fn stub_571c5c() -> ! {
    todo!("0x571c5c RBX::BackpackItem::getSize(RBX::Canvas)const")
}

// 0x5721a8 — __ZN3RBX9HopperBinC2Ev
// type: _DWORD __fastcall(RBX::HopperBin *__hidden this)
#[doc(alias = "__ZN3RBX9HopperBinC2Ev")]
#[doc(alias = "RBX::HopperBin::HopperBin(void)")]
// was: __ZN3RBX9HopperBinC2Ev
pub fn stub_5721a8() -> ! {
    todo!("0x5721a8 RBX::HopperBin::HopperBin(void)")
}

// 0x572710 — __ZN3RBX9HopperBin30selectedConnectionShimFunctionEv
// type: _DWORD __fastcall(RBX::HopperBin *__hidden this)
#[doc(alias = "__ZN3RBX9HopperBin30selectedConnectionShimFunctionEv")]
#[doc(alias = "RBX::HopperBin::selectedConnectionShimFunction(void)")]
// was: __ZN3RBX9HopperBin30selectedConnectionShimFunctionEv
pub fn stub_572710() -> ! {
    todo!("0x572710 RBX::HopperBin::selectedConnectionShimFunction(void)")
}

// 0x572714 — __ZN3RBX9HopperBin14onSelectScriptEv
// type: _DWORD __fastcall(RBX::HopperBin *__hidden this)
#[doc(alias = "__ZN3RBX9HopperBin14onSelectScriptEv")]
#[doc(alias = "RBX::HopperBin::onSelectScript(void)")]
// was: __ZN3RBX9HopperBin14onSelectScriptEv
pub fn stub_572714() -> ! {
    todo!("0x572714 RBX::HopperBin::onSelectScript(void)")
}

// 0x5728a4 — __ZN3RBX9HopperBin37reverseSelectedConnectionShimFunctionERN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "__ZN3RBX9HopperBin37reverseSelectedConnectionShimFunctionERN5boost10shared_ptrINS_8InstanceEEE")]
#[doc(alias = "RBX::HopperBin::reverseSelectedConnectionShimFunction(rbx_core::SharedPtr<RBX::Instance> &)")]
// was: __ZN3RBX9HopperBin37reverseSelectedConnectionShimFunctionERN5boost10shared_ptrINS_8InstanceEEE
pub fn stub_5728a4() -> ! {
    todo!("0x5728a4 RBX::HopperBin::reverseSelectedConnectionShimFunction(boost::shared_ptr<RBX::Instance> &)")
}

// 0x5728bc — __ZN3RBX9HopperBin17onAncestorChangedERKNS_15AncestorChangedE
#[doc(alias = "__ZN3RBX9HopperBin17onAncestorChangedERKNS_15AncestorChangedE")]
#[doc(alias = "RBX::HopperBin::onAncestorChanged(RBX::AncestorChanged const&)")]
// was: __ZN3RBX9HopperBin17onAncestorChangedERKNS_15AncestorChangedE
pub fn stub_5728bc() -> ! {
    todo!("0x5728bc RBX::HopperBin::onAncestorChanged(RBX::AncestorChanged const&)")
}

// 0x572b14 — __ZN3RBX9HopperBin15onSelectCommandEv
// type: _DWORD __fastcall(RBX::HopperBin *__hidden this)
#[doc(alias = "__ZN3RBX9HopperBin15onSelectCommandEv")]
#[doc(alias = "RBX::HopperBin::onSelectCommand(void)")]
// was: __ZN3RBX9HopperBin15onSelectCommandEv
pub fn stub_572b14() -> ! {
    todo!("0x572b14 RBX::HopperBin::onSelectCommand(void)")
}

// 0x572e98 — __ZN3RBX9HopperBin14onLocalClickedEv
// type: _DWORD __fastcall(RBX::HopperBin *__hidden this)
#[doc(alias = "__ZN3RBX9HopperBin14onLocalClickedEv")]
#[doc(alias = "RBX::HopperBin::onLocalClicked(void)")]
// was: __ZN3RBX9HopperBin14onLocalClickedEv
pub fn stub_572e98() -> ! {
    todo!("0x572e98 RBX::HopperBin::onLocalClicked(void)")
}

// 0x572ef8 — __ZN3RBX9HopperBin19onLocalOtherClickedEv
// type: _DWORD __fastcall(RBX::HopperBin *__hidden this)
#[doc(alias = "__ZN3RBX9HopperBin19onLocalOtherClickedEv")]
#[doc(alias = "RBX::HopperBin::onLocalOtherClicked(void)")]
// was: __ZN3RBX9HopperBin19onLocalOtherClickedEv
pub fn stub_572ef8() -> ! {
    todo!("0x572ef8 RBX::HopperBin::onLocalOtherClicked(void)")
}

// 0x572efc — __ZN3RBX6HopperC2Ev
// type: _DWORD __fastcall(RBX::Hopper *__hidden this)
#[doc(alias = "__ZN3RBX6HopperC2Ev")]
#[doc(alias = "RBX::Hopper::Hopper(void)")]
// was: __ZN3RBX6HopperC2Ev
pub fn stub_572efc() -> ! {
    todo!("0x572efc RBX::Hopper::Hopper(void)")
}

// 0x572f38 — __ZNK3RBX6Hopper12askSetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Hopper *__hidden this, const RBX::Instance *)
#[doc(alias = "__ZNK3RBX6Hopper12askSetParentEPKNS_8InstanceE")]
#[doc(alias = "RBX::Hopper::askSetParent(RBX::Instance const*)const")]
// was: __ZNK3RBX6Hopper12askSetParentEPKNS_8InstanceE
pub fn stub_572f38() -> ! {
    todo!("0x572f38 RBX::Hopper::askSetParent(RBX::Instance const*)const")
}

// 0x572f3c — __ZNK3RBX6Hopper11askAddChildEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Hopper *__hidden this, const RBX::Instance *)
#[doc(alias = "__ZNK3RBX6Hopper11askAddChildEPKNS_8InstanceE")]
#[doc(alias = "RBX::Hopper::askAddChild(RBX::Instance const*)const")]
// was: __ZNK3RBX6Hopper11askAddChildEPKNS_8InstanceE
pub fn stub_572f3c() -> ! {
    todo!("0x572f3c RBX::Hopper::askAddChild(RBX::Instance const*)const")
}

// 0x573090 — __ZN3RBX18StarterPackServiceC1Ev
// type: _DWORD __fastcall(RBX::StarterPackService *__hidden this)
#[doc(alias = "__ZN3RBX18StarterPackServiceC1Ev")]
#[doc(alias = "RBX::StarterPackService::StarterPackService(void)")]
// was: __ZN3RBX18StarterPackServiceC1Ev
pub fn stub_573090() -> ! {
    todo!("0x573090 RBX::StarterPackService::StarterPackService(void)")
}

// 0x573094 — __ZN3RBX18StarterPackServiceC2Ev
// type: _DWORD __fastcall(RBX::StarterPackService *__hidden this)
#[doc(alias = "__ZN3RBX18StarterPackServiceC2Ev")]
#[doc(alias = "RBX::StarterPackService::StarterPackService(void)")]
// was: __ZN3RBX18StarterPackServiceC2Ev
pub fn stub_573094() -> ! {
    todo!("0x573094 RBX::StarterPackService::StarterPackService(void)")
}

// 0x5732b0 — __ZN3RBX10Reflection8EnumDescINS_9HopperBin7BinTypeEE7addPairES3_PKc
// type: int(void)
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9HopperBin7BinTypeEE7addPairES3_PKc")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::HopperBin::BinType>::addPair(RBX::HopperBin::BinType,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_9HopperBin7BinTypeEE7addPairES3_PKc
pub fn stub_5732b0() -> ! {
    todo!("0x5732b0 RBX::Reflection::EnumDesc<RBX::HopperBin::BinType>::addPair(RBX::HopperBin::BinType,char const*)")
}

// 0x573610 — __ZN3RBX10Reflection8EnumDescINS_9HopperBin7BinTypeEE9addLegacyEiPKcS3_
// type: int(void)
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9HopperBin7BinTypeEE9addLegacyEiPKcS3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::HopperBin::BinType>::addLegacy(int,char const*,RBX::HopperBin::BinType)")]
// was: __ZN3RBX10Reflection8EnumDescINS_9HopperBin7BinTypeEE9addLegacyEiPKcS3_
pub fn stub_573610() -> ! {
    todo!("0x573610 RBX::Reflection::EnumDesc<RBX::HopperBin::BinType>::addLegacy(int,char const*,RBX::HopperBin::BinType)")
}

// 0x573664 — __ZN3RBX10Reflection14PropDescriptorINS_12BackpackItemENS_9TextureIdEED1Ev
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_12BackpackItemENS_9TextureIdEED1Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BackpackItem,RBX::TextureId>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_12BackpackItemENS_9TextureIdEED1Ev
pub fn stub_573664() -> ! {
    todo!("0x573664 RBX::Reflection::PropDescriptor<RBX::BackpackItem,RBX::TextureId>::~PropDescriptor()")
}

// 0x573688 — __ZNK3RBX9HopperBin10getBinTypeEv
// type: _DWORD __fastcall(RBX::HopperBin *__hidden this)
#[doc(alias = "__ZNK3RBX9HopperBin10getBinTypeEv")]
#[doc(alias = "RBX::HopperBin::getBinType(void)const")]
// was: __ZNK3RBX9HopperBin10getBinTypeEv
pub fn stub_573688() -> ! {
    todo!("0x573688 RBX::HopperBin::getBinType(void)const")
}

// 0x573690 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEED1Ev
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEED1Ev")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEED1Ev
pub fn stub_573690() -> ! {
    todo!("0x573690 RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::~EnumPropDescriptor()")
}

// 0x5736b4 — __ZN3RBX10Reflection9EventDescINS_9HopperBinEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_ED1Ev
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9HopperBinEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_ED1Ev")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::HopperBin,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::HopperBin::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_9HopperBinEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_ED1Ev
pub fn stub_5736b4() -> ! {
    todo!("0x5736b4 RBX::Reflection::EventDesc<RBX::HopperBin,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::HopperBin::*>::~EventDesc()")
}

// 0x5736d8 — __ZN3RBX10Reflection15RemoteEventDescINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEED1Ev
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEED1Ev")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>>::~RemoteEventDesc()")]
// was: __ZN3RBX10Reflection15RemoteEventDescINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEED1Ev
pub fn stub_5736d8() -> ! {
    todo!("0x5736d8 RBX::Reflection::RemoteEventDesc<RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>>::~RemoteEventDesc()")
}

// 0x5736fc — __ZN3RBX10Reflection9EventDescINS_9HopperBinEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9HopperBinEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::HopperBin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::HopperBin::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_9HopperBinEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev
pub fn stub_5736fc() -> ! {
    todo!("0x5736fc RBX::Reflection::EventDesc<RBX::HopperBin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::HopperBin::*>::~EventDesc()")
}

// 0x573720 — __ZN3RBX10Reflection13BoundFuncDescINS_9HopperBinEFvvELi0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9HopperBinEFvvELi0EED1Ev")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HopperBin,void ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9HopperBinEFvvELi0EED1Ev
pub fn stub_573720() -> ! {
    todo!("0x573720 RBX::Reflection::BoundFuncDesc<RBX::HopperBin,void ()(void),0>::~BoundFuncDesc()")
}

// 0x573744 — __ZN3RBX10Reflection14PropDescriptorINS_9HopperBinESsED1Ev
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9HopperBinESsED1Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::HopperBin,std::string>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_9HopperBinESsED1Ev
pub fn stub_573744() -> ! {
    todo!("0x573744 RBX::Reflection::PropDescriptor<RBX::HopperBin,std::string>::~PropDescriptor()")
}
