//! rendering shard 258 — 100 stubs EA-sorted asc global gap filler after 0x31e458 not yet in rendering (Ogre|G3D|Render 14876/14876 complete, 27720->27820 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x31e63c — __ZN5boost15circular_bufferIdSaIdEE7destroyEv
// type: int(void)
#[doc(alias = "boost::circular_buffer<double,std::allocator<double>>::destroy(void)")]
// was: __ZN5boost15circular_bufferIdSaIdEE7destroyEv
pub fn stub_31e63c() -> ! {
    todo!("0x31e63c boost::circular_buffer<double,std::allocator<double>>::destroy(void)")
}

// 0x322ec8 — __ZN3RBX14InstanceHandleC1EPNS_10Reflection13DescribedBaseE
// type: _DWORD __fastcall(RBX::InstanceHandle *__hidden this, RBX::Reflection::DescribedBase *)
#[doc(alias = "RBX::InstanceHandle::InstanceHandle(RBX::Reflection::DescribedBase *)")]
// was: __ZN3RBX14InstanceHandleC1EPNS_10Reflection13DescribedBaseE
pub fn stub_322ec8() -> ! {
    todo!("0x322ec8 RBX::InstanceHandle::InstanceHandle(RBX::Reflection::DescribedBase *)")
}

// 0x322ed8 — __ZNK3RBX14InstanceHandle12operatorLessERKS0_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::InstanceHandle::operatorLess(RBX::InstanceHandle const&)const")]
// was: __ZNK3RBX14InstanceHandle12operatorLessERKS0_
pub fn stub_322ed8() -> ! {
    todo!("0x322ed8 RBX::InstanceHandle::operatorLess(RBX::InstanceHandle const&)const")
}

// 0x322ee8 — __ZNK3RBX14InstanceHandle5emptyEv
// type: _DWORD __fastcall(RBX::InstanceHandle *__hidden this)
#[doc(alias = "RBX::InstanceHandle::empty(void)const")]
// was: __ZNK3RBX14InstanceHandle5emptyEv
pub fn stub_322ee8() -> ! {
    todo!("0x322ee8 RBX::InstanceHandle::empty(void)const")
}

// 0x322ef4 — __ZN3RBX14InstanceHandle6linkToEN5boost10shared_ptrINS_10Reflection13DescribedBaseEEE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::InstanceHandle::linkTo(rbx_core::SharedPtr<RBX::Reflection::DescribedBase>)")]
// was: __ZN3RBX14InstanceHandle6linkToEN5boost10shared_ptrINS_10Reflection13DescribedBaseEEE
pub fn stub_322ef4() -> ! {
    todo!("0x322ef4 RBX::InstanceHandle::linkTo(rbx_core::SharedPtr<RBX::Reflection::DescribedBase>)")
}

// 0x32305c — __ZN3RBX17HeartbeatInstance34onServiceProviderHeartbeatInstanceEPNS_15ServiceProviderES2_
// type: _DWORD __fastcall(RBX::HeartbeatInstance *__hidden this, RBX::ServiceProvider *, RBX::ServiceProvider *)
#[doc(alias = "RBX::HeartbeatInstance::onServiceProviderHeartbeatInstance(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// was: __ZN3RBX17HeartbeatInstance34onServiceProviderHeartbeatInstanceEPNS_15ServiceProviderES2_
pub fn stub_32305c() -> ! {
    todo!("0x32305c RBX::HeartbeatInstance::onServiceProviderHeartbeatInstance(RBX::ServiceProvider *,RBX::ServiceProvider *)")
}

// 0x323238 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE7connectIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_17HeartbeatInstanceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Heartbeat const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HeartbeatInstance,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::HeartbeatInstance*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HeartbeatInstance,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::HeartbeatInstance*>,boost::arg<1>>> const&)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE7connectIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_17HeartbeatInstanceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEEENS0_10connectionERKT_
pub fn stub_323238() -> ! {
    todo!("0x323238 rbx::signals::connection rbx::signals::signal<void ()(RBX::Heartbeat const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HeartbeatInstance,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::HeartbeatInstance*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HeartbeatInstance,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::HeartbeatInstance*>,boost::arg<1>>> const&)")
}

// 0x3232ac — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_17HeartbeatInstanceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HeartbeatInstance,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::HeartbeatInstance*>,boost::arg<1>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_17HeartbeatInstanceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED1Ev
pub fn stub_3232ac() -> ! {
    todo!("0x3232ac rbx::signals::signal<void ()(RBX::Heartbeat const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HeartbeatInstance,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::HeartbeatInstance*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x3232d8 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_17HeartbeatInstanceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HeartbeatInstance,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::HeartbeatInstance*>,boost::arg<1>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_17HeartbeatInstanceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED0Ev
pub fn stub_3232d8() -> ! {
    todo!("0x3232d8 rbx::signals::signal<void ()(RBX::Heartbeat const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HeartbeatInstance,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::HeartbeatInstance*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x3233ac — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_17HeartbeatInstanceES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HeartbeatInstance,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::HeartbeatInstance*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_17HeartbeatInstanceES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_
pub fn stub_3233ac() -> ! {
    todo!("0x3233ac rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HeartbeatInstance,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::HeartbeatInstance*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)")
}

// 0x3233b4 — __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_17HeartbeatInstanceES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HeartbeatInstance,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::HeartbeatInstance*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_17HeartbeatInstanceES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_
pub fn stub_3233b4() -> ! {
    todo!("0x3233b4 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HeartbeatInstance,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::HeartbeatInstance*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)")
}

// 0x3233bc — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX17HeartbeatInstanceERKNS4_9HeartbeatEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HeartbeatInstance,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::HeartbeatInstance*>,boost::arg<1>>>::operator()<RBX::Heartbeat>(RBX::Heartbeat const&)")]
// was: __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX17HeartbeatInstanceERKNS4_9HeartbeatEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_
pub fn stub_3233bc() -> ! {
    todo!("0x3233bc void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HeartbeatInstance,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::HeartbeatInstance*>,boost::arg<1>>>::operator()<RBX::Heartbeat>(RBX::Heartbeat const&)")
}

// 0x3233d4 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_17HeartbeatInstanceES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HeartbeatInstance,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::HeartbeatInstance*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_17HeartbeatInstanceES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED1Ev
pub fn stub_3233d4() -> ! {
    todo!("0x3233d4 rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HeartbeatInstance,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::HeartbeatInstance*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::~callable()")
}

// 0x323400 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_17HeartbeatInstanceES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HeartbeatInstance,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::HeartbeatInstance*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_17HeartbeatInstanceES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED0Ev
pub fn stub_323400() -> ! {
    todo!("0x323400 rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HeartbeatInstance,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::HeartbeatInstance*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::~callable()")
}

// 0x325998 — __ZN3RBX18InterpolatedCFrame12computeValueEPNS_12PartInstanceE
// type: _DWORD __fastcall(RBX::InterpolatedCFrame *__hidden this, RBX::PartInstance *)
#[doc(alias = "RBX::InterpolatedCFrame::computeValue(RBX::PartInstance *)")]
// was: __ZN3RBX18InterpolatedCFrame12computeValueEPNS_12PartInstanceE
pub fn stub_325998() -> ! {
    todo!("0x325998 RBX::InterpolatedCFrame::computeValue(RBX::PartInstance *)")
}

// 0x325bb4 — __ZN5boost31circular_buffer_space_optimizedIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE5clearEv
#[doc(alias = "boost::circular_buffer_space_optimized<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::clear(void)")]
// was: __ZN5boost31circular_buffer_space_optimizedIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE5clearEv
pub fn stub_325bb4() -> ! {
    todo!("0x325bb4 boost::circular_buffer_space_optimized<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::clear(void)")
}

// 0x325be0 — __ZN5boost31circular_buffer_space_optimizedIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE18check_low_capacityEm
#[doc(alias = "boost::circular_buffer_space_optimized<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::check_low_capacity(unsigned long)")]
// was: __ZN5boost31circular_buffer_space_optimizedIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE18check_low_capacityEm
pub fn stub_325be0() -> ! {
    todo!("0x325be0 boost::circular_buffer_space_optimized<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::check_low_capacity(unsigned long)")
}

// 0x325ce4 — __ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE12set_capacityEm
#[doc(alias = "boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::set_capacity(unsigned long)")]
// was: __ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE12set_capacityEm
pub fn stub_325ce4() -> ! {
    todo!("0x325ce4 boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::set_capacity(unsigned long)")
}

// 0x325e8c — __ZN5boost10cb_details8iteratorINS_15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS5_EEENS0_15nonconst_traitsIS6_EEEpLEi
#[doc(alias = "boost::cb_details::iterator<boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>,boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>>::operator+=(int)")]
// was: __ZN5boost10cb_details8iteratorINS_15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS5_EEENS0_15nonconst_traitsIS6_EEEpLEi
pub fn stub_325e8c() -> ! {
    todo!("0x325e8c boost::cb_details::iterator<boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>,boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>>::operator+=(int)")
}

// 0x325ed4 — __ZN5boost10cb_details8iteratorINS_15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS5_EEENS0_15nonconst_traitsIS6_EEEmIEi
#[doc(alias = "boost::cb_details::iterator<boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>,boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>>::operator-=(int)")]
// was: __ZN5boost10cb_details8iteratorINS_15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS5_EEENS0_15nonconst_traitsIS6_EEEmIEi
pub fn stub_325ed4() -> ! {
    todo!("0x325ed4 boost::cb_details::iterator<boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>,boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>>::operator-=(int)")
}

// 0x325f14 — __ZN5boost31circular_buffer_space_optimizedIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE5eraseENS_10cb_details8iteratorINS_15circular_bufferIS3_S4_EENS6_15nonconst_traitsIS4_EEEESC_
#[doc(alias = "boost::circular_buffer_space_optimized<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::erase(boost::cb_details::iterator<boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>,boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>>,boost::cb_details::iterator<boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>,boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>>)")]
// was: __ZN5boost31circular_buffer_space_optimizedIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE5eraseENS_10cb_details8iteratorINS_15circular_bufferIS3_S4_EENS6_15nonconst_traitsIS4_EEEESC_
pub fn stub_325f14() -> ! {
    todo!("0x325f14 boost::circular_buffer_space_optimized<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::erase(boost::cb_details::iterator<boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>,boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>>,boost::cb_details::iterator<boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>,boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>>)")
}

// 0x325f94 — __ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE5eraseENS_10cb_details8iteratorIS5_NS6_15nonconst_traitsIS4_EEEESA_
#[doc(alias = "boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::erase(boost::cb_details::iterator<boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>,boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>>,boost::cb_details::iterator<boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>,boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>>)")]
// was: __ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE5eraseENS_10cb_details8iteratorIS5_NS6_15nonconst_traitsIS4_EEEESA_
pub fn stub_325f94() -> ! {
    todo!("0x325f94 boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::erase(boost::cb_details::iterator<boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>,boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>>,boost::cb_details::iterator<boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>,boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>>)")
}

// 0x32607c — __ZN5boost31circular_buffer_space_optimizedIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE19check_high_capacityEv
#[doc(alias = "boost::circular_buffer_space_optimized<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::check_high_capacity(void)")]
// was: __ZN5boost31circular_buffer_space_optimizedIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE19check_high_capacityEv
pub fn stub_32607c() -> ! {
    todo!("0x32607c boost::circular_buffer_space_optimized<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::check_high_capacity(void)")
}

// 0x3260d8 — __ZNK5boost10cb_details8iteratorINS_15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS5_EEENS0_15nonconst_traitsIS6_EEE17linearize_pointerIS9_EENT_7pointerERKNS1_IS7_SC_EE
// type: int __fastcall(_DWORD **, int)
#[doc(alias = "boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::pointer boost::cb_details::iterator<boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>,boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>>::linearize_pointer<boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>>(boost::cb_details::iterator<boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>,boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::pointer> const&)const")]
// was: __ZNK5boost10cb_details8iteratorINS_15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS5_EEENS0_15nonconst_traitsIS6_EEE17linearize_pointerIS9_EENT_7pointerERKNS1_IS7_SC_EE
pub fn stub_3260d8() -> ! {
    todo!("0x3260d8 boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::pointer boost::cb_details::iterator<boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>,boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>>::linearize_pointer<boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>>(boost::cb_details::iterator<boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>,boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::pointer> const&)const")
}

// 0x326378 — __ZN3RBX10Reflection8EnumDescINS_17KeywordFilterTypeEEC1Ev
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::KeywordFilterType>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_17KeywordFilterTypeEEC1Ev
pub fn stub_326378() -> ! {
    todo!("0x326378 RBX::Reflection::EnumDesc<RBX::KeywordFilterType>::EnumDesc(void)")
}

// 0x32637c — __ZN3RBX10Reflection8EnumDescINS_17KeywordFilterTypeEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::KeywordFilterType>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_17KeywordFilterTypeEEC2Ev
pub fn stub_32637c() -> ! {
    todo!("0x32637c RBX::Reflection::EnumDesc<RBX::KeywordFilterType>::EnumDesc(void)")
}

// 0x32653c — __ZN3RBX10Reflection8EnumDescINS_17KeywordFilterTypeEE7addPairES2_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::KeywordFilterType>::addPair(RBX::KeywordFilterType,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_17KeywordFilterTypeEE7addPairES2_PKc
pub fn stub_32653c() -> ! {
    todo!("0x32653c RBX::Reflection::EnumDesc<RBX::KeywordFilterType>::addPair(RBX::KeywordFilterType,char const*)")
}

// 0x326a5c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17KeywordFilterTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::KeywordFilterType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::KeywordFilterType> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17KeywordFilterTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_
pub fn stub_326a5c() -> ! {
    todo!("0x326a5c std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::KeywordFilterType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::KeywordFilterType> const&)")
}

// 0x326ac4 — __ZNSt6vectorIN3RBX17KeywordFilterTypeESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: int __fastcall(int, char *, _DWORD *)
#[doc(alias = "std::vector<RBX::KeywordFilterType,std::allocator<RBX::KeywordFilterType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::KeywordFilterType*,std::vector<RBX::KeywordFilterType,std::allocator<RBX::KeywordFilterType>>>,RBX::KeywordFilterType const&)")]
// was: __ZNSt6vectorIN3RBX17KeywordFilterTypeESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
pub fn stub_326ac4() -> ! {
    todo!("0x326ac4 std::vector<RBX::KeywordFilterType,std::allocator<RBX::KeywordFilterType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::KeywordFilterType*,std::vector<RBX::KeywordFilterType,std::allocator<RBX::KeywordFilterType>>>,RBX::KeywordFilterType const&)")
}

// 0x326ba8 — __ZNSt12_Vector_baseIN3RBX17KeywordFilterTypeESaIS1_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::KeywordFilterType,std::allocator<RBX::KeywordFilterType>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN3RBX17KeywordFilterTypeESaIS1_EE11_M_allocateEm
pub fn stub_326ba8() -> ! {
    todo!("0x326ba8 std::_Vector_base<RBX::KeywordFilterType,std::allocator<RBX::KeywordFilterType>>::_M_allocate(unsigned long)")
}

// 0x326bc0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX17KeywordFilterTypeES5_EET0_T_S7_S6_
#[doc(alias = "RBX::KeywordFilterType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::KeywordFilterType *,RBX::KeywordFilterType *>(RBX::KeywordFilterType *,RBX::KeywordFilterType *,RBX::KeywordFilterType *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX17KeywordFilterTypeES5_EET0_T_S7_S6_
pub fn stub_326bc0() -> ! {
    todo!("0x326bc0 RBX::KeywordFilterType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::KeywordFilterType *,RBX::KeywordFilterType *>(RBX::KeywordFilterType *,RBX::KeywordFilterType *,RBX::KeywordFilterType *)")
}

// 0x326bfc — __ZNSt6vectorIN3RBX17KeywordFilterTypeESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
#[doc(alias = "std::vector<RBX::KeywordFilterType,std::allocator<RBX::KeywordFilterType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::KeywordFilterType*,std::vector<RBX::KeywordFilterType,std::allocator<RBX::KeywordFilterType>>>,unsigned long,RBX::KeywordFilterType const&)")]
// was: __ZNSt6vectorIN3RBX17KeywordFilterTypeESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
pub fn stub_326bfc() -> ! {
    todo!("0x326bfc std::vector<RBX::KeywordFilterType,std::allocator<RBX::KeywordFilterType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::KeywordFilterType*,std::vector<RBX::KeywordFilterType,std::allocator<RBX::KeywordFilterType>>>,unsigned long,RBX::KeywordFilterType const&)")
}

// 0x326d8c — __GLOBAL__I_a_125
#[doc(alias = "global constructor keyed to_a_125")]
// was: __GLOBAL__I_a_125
pub fn stub_326d8c() -> ! {
    todo!("0x326d8c global constructor keyed to_a_125")
}

// 0x326e54 — __ZN3RBX18LegacyContentTableC1Ev
// type: _DWORD __fastcall(RBX::LegacyContentTable *__hidden this)
#[doc(alias = "RBX::LegacyContentTable::LegacyContentTable(void)")]
// was: __ZN3RBX18LegacyContentTableC1Ev
pub fn stub_326e54() -> ! {
    todo!("0x326e54 RBX::LegacyContentTable::LegacyContentTable(void)")
}

// 0x326e58 — __ZN3RBX18LegacyContentTableC2Ev
// type: _DWORD __fastcall(RBX::LegacyContentTable *__hidden this)
#[doc(alias = "RBX::LegacyContentTable::LegacyContentTable(void)")]
// was: __ZN3RBX18LegacyContentTableC2Ev
pub fn stub_326e58() -> ! {
    todo!("0x326e58 RBX::LegacyContentTable::LegacyContentTable(void)")
}

// 0x3378c6 — sub_3378C6
// type: void __fastcall __noreturn(int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpta, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "sub_3378C6")]
// was: sub_3378C6
pub fn stub_3378c6() -> ! {
    todo!("0x3378c6 sub_3378C6")
}

// 0x34581c — __ZN3RBX18LegacyContentTable8AddEntryERKSsS2_
// type: _DWORD __fastcall(RBX::LegacyContentTable *__hidden this, const std::string *, const std::string *)
#[doc(alias = "RBX::LegacyContentTable::AddEntry(std::string const&,std::string const&)")]
// was: __ZN3RBX18LegacyContentTable8AddEntryERKSsS2_
pub fn stub_34581c() -> ! {
    todo!("0x34581c RBX::LegacyContentTable::AddEntry(std::string const&,std::string const&)")
}

// 0x345950 — __ZN12_GLOBAL__N_112normalizeUrlERSs
// type: _DWORD __fastcall(_anonymous_namespace_ *__hidden this, std::string *)
#[doc(alias = "anonymous namespace::normalizeUrl(std::string &)")]
// was: __ZN12_GLOBAL__N_112normalizeUrlERSs
pub fn stub_345950() -> ! {
    todo!("0x345950 anonymous namespace::normalizeUrl(std::string &)")
}

// 0x3459d4 — __ZN3RBX18LegacyContentTable9FindEntryERKSs
// type: _DWORD __fastcall(RBX::LegacyContentTable *__hidden this, const std::string *)
#[doc(alias = "RBX::LegacyContentTable::FindEntry(std::string const&)")]
// was: __ZN3RBX18LegacyContentTable9FindEntryERKSs
pub fn stub_3459d4() -> ! {
    todo!("0x3459d4 RBX::LegacyContentTable::FindEntry(std::string const&)")
}

// 0x345b08 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::string>>,std::string,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")]
// was: __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_
pub fn stub_345b08() -> ! {
    todo!("0x345b08 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::string>>,std::string,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")
}

// 0x345b48 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSB_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEmRKT_RKT0_
// type: int __fastcall(int, unsigned int, std::string *)
#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::string>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::string>>,std::string,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
// was: __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSB_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEmRKT_RKT0_
pub fn stub_345b48() -> ! {
    todo!("0x345b48 boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::string>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::string>>,std::string,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")
}

// 0x345bb4 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEEC2EmRKS9_RKSB_RKSaINS1_8ptr_nodeIS6_EEE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::string>>,std::string,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::string>>> const&)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEEC2EmRKS9_RKSB_RKSaINS1_8ptr_nodeIS6_EEE
pub fn stub_345bb4() -> ! {
    todo!("0x345bb4 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::string>>,std::string,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::string>>> const&)")
}

// 0x345c20 — __ZN3RBX13findLocalFileERKSsPSs
#[doc(alias = "RBX::findLocalFile(std::string const&,std::string *)")]
// was: __ZN3RBX13findLocalFileERKSsPSs
pub fn stub_345c20() -> ! {
    todo!("0x345c20 RBX::findLocalFile(std::string const&,std::string *)")
}

// 0x345c24 — __ZN3RBX13LuaWebService26CachedRawLuaWebServiceInfoC2EN5boost10shared_ptrIKSsEES5_
#[doc(alias = "RBX::LuaWebService::CachedRawLuaWebServiceInfo::CachedRawLuaWebServiceInfo(rbx_core::SharedPtr<std::string const>,rbx_core::SharedPtr<std::string const>)")]
// was: __ZN3RBX13LuaWebService26CachedRawLuaWebServiceInfoC2EN5boost10shared_ptrIKSsEES5_
pub fn stub_345c24() -> ! {
    todo!("0x345c24 RBX::LuaWebService::CachedRawLuaWebServiceInfo::CachedRawLuaWebServiceInfo(rbx_core::SharedPtr<std::string const>,rbx_core::SharedPtr<std::string const>)")
}

// 0x345cf4 — __ZN3RBX13LuaWebService23CachedLuaWebServiceInfoC2EN5boost10shared_ptrIKSsEES5_
#[doc(alias = "RBX::LuaWebService::CachedLuaWebServiceInfo::CachedLuaWebServiceInfo(rbx_core::SharedPtr<std::string const>,rbx_core::SharedPtr<std::string const>)")]
// was: __ZN3RBX13LuaWebService23CachedLuaWebServiceInfoC2EN5boost10shared_ptrIKSsEES5_
pub fn stub_345cf4() -> ! {
    todo!("0x345cf4 RBX::LuaWebService::CachedLuaWebServiceInfo::CachedLuaWebServiceInfo(rbx_core::SharedPtr<std::string const>,rbx_core::SharedPtr<std::string const>)")
}

// 0x345ed0 — __ZN3RBX13LuaWebServiceC1Ev
// type: _DWORD __fastcall(RBX::LuaWebService *__hidden this)
#[doc(alias = "RBX::LuaWebService::LuaWebService(void)")]
// was: __ZN3RBX13LuaWebServiceC1Ev
pub fn stub_345ed0() -> ! {
    todo!("0x345ed0 RBX::LuaWebService::LuaWebService(void)")
}

// 0x345ed4 — __ZN3RBX13LuaWebServiceC2Ev
// type: _DWORD __fastcall(RBX::LuaWebService *__hidden this)
#[doc(alias = "RBX::LuaWebService::LuaWebService(void)")]
// was: __ZN3RBX13LuaWebServiceC2Ev
pub fn stub_345ed4() -> ! {
    todo!("0x345ed4 RBX::LuaWebService::LuaWebService(void)")
}

// 0x346168 — __ZN3RBX13LuaWebService11RawCallbackEN5boost8weak_ptrIS0_EENS_14AsyncHttpQueue13RequestResultESsNS1_8functionIFvSsEEES8_
#[doc(alias = "RBX::LuaWebService::RawCallback(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
// was: __ZN3RBX13LuaWebService11RawCallbackEN5boost8weak_ptrIS0_EENS_14AsyncHttpQueue13RequestResultESsNS1_8functionIFvSsEEES8_
pub fn stub_346168() -> ! {
    todo!("0x346168 RBX::LuaWebService::RawCallback(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")
}

// 0x34636c — __ZN3RBX13LuaWebService12asyncRequestERKSsfN5boost8functionIFvNS3_10shared_ptrIKSt6vectorINS_10Reflection7VariantESaIS8_EEEEEEENS4_IFvSsEEE
#[doc(alias = "RBX::LuaWebService::asyncRequest(std::string const&,float,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)")]
// was: __ZN3RBX13LuaWebService12asyncRequestERKSsfN5boost8functionIFvNS3_10shared_ptrIKSt6vectorINS_10Reflection7VariantESaIS8_EEEEEEENS4_IFvSsEEE
pub fn stub_34636c() -> ! {
    todo!("0x34636c RBX::LuaWebService::asyncRequest(std::string const&,float,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)")
}

// 0x346620 — __ZN3RBX13LuaWebService19asyncRequestNoCacheERKSsfN5boost8functionIFvNS3_10shared_ptrIKSt3mapISsNS_10Reflection7VariantESt4lessISsESaISt4pairIS1_S8_EEEEEEEENS_14AsyncHttpQueue9ResultJobE
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, int)
#[doc(alias = "RBX::LuaWebService::asyncRequestNoCache(std::string const&,float,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,RBX::AsyncHttpQueue::ResultJob)")]
// was: __ZN3RBX13LuaWebService19asyncRequestNoCacheERKSsfN5boost8functionIFvNS3_10shared_ptrIKSt3mapISsNS_10Reflection7VariantESt4lessISsESaISt4pairIS1_S8_EEEEEEEENS_14AsyncHttpQueue9ResultJobE
pub fn stub_346620() -> ! {
    todo!("0x346620 RBX::LuaWebService::asyncRequestNoCache(std::string const&,float,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,RBX::AsyncHttpQueue::ResultJob)")
}

// 0x34695c — __ZN3RBX13LuaWebService12asyncRequestERKSsfN5boost8functionIFvbEEENS4_IFvSsEEE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::LuaWebService::asyncRequest(std::string const&,float,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// was: __ZN3RBX13LuaWebService12asyncRequestERKSsfN5boost8functionIFvbEEENS4_IFvSsEEE
pub fn stub_34695c() -> ! {
    todo!("0x34695c RBX::LuaWebService::asyncRequest(std::string const&,float,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")
}

// 0x346c10 — __ZN3RBX13LuaWebService12asyncRequestERKSsfN5boost8functionIFviEEENS4_IFvSsEEE
// type: void __fastcall(int, const std::string *, int, int, int)
#[doc(alias = "RBX::LuaWebService::asyncRequest(std::string const&,float,boost::function<void ()(int)>,boost::function<void ()(std::string)>)")]
// was: __ZN3RBX13LuaWebService12asyncRequestERKSsfN5boost8functionIFviEEENS4_IFvSsEEE
pub fn stub_346c10() -> ! {
    todo!("0x346c10 RBX::LuaWebService::asyncRequest(std::string const&,float,boost::function<void ()(int)>,boost::function<void ()(std::string)>)")
}

// 0x346ec4 — __ZN3RBX13LuaWebService12asyncRequestERKSsfN5boost8functionIFvSsEEES6_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::LuaWebService::asyncRequest(std::string const&,float,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
// was: __ZN3RBX13LuaWebService12asyncRequestERKSsfN5boost8functionIFvSsEEES6_
pub fn stub_346ec4() -> ! {
    todo!("0x346ec4 RBX::LuaWebService::asyncRequest(std::string const&,float,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")
}

// 0x347178 — __ZN5boost10shared_ptrIN3RBX14AsyncHttpCacheINS1_13LuaWebService23CachedLuaWebServiceInfoELb1EEEE5resetIS5_EEvPT_
#[doc(alias = "void rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::reset<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *)")]
// was: __ZN5boost10shared_ptrIN3RBX14AsyncHttpCacheINS1_13LuaWebService23CachedLuaWebServiceInfoELb1EEEE5resetIS5_EEvPT_
pub fn stub_347178() -> ! {
    todo!("0x347178 void rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::reset<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *)")
}

// 0x3471a4 — __ZN5boost10shared_ptrIN3RBX14AsyncHttpCacheINS1_13LuaWebService26CachedRawLuaWebServiceInfoELb1EEEE5resetIS5_EEvPT_
#[doc(alias = "void rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::reset<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true> *)")]
// was: __ZN5boost10shared_ptrIN3RBX14AsyncHttpCacheINS1_13LuaWebService26CachedRawLuaWebServiceInfoELb1EEEE5resetIS5_EEvPT_
pub fn stub_3471a4() -> ! {
    todo!("0x3471a4 void rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::reset<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true> *)")
}

// 0x3471d0 — __ZN3RBX13LuaWebService21TryRawDispatchRequestISsEEbPNS_14AsyncHttpCacheINS0_26CachedRawLuaWebServiceInfoELb1EEERKSsN5boost8functionIFvT_EEENS9_IFvSsEEE
// type: int __fastcall(int, int, int, int)
#[doc(alias = "bool RBX::LuaWebService::TryRawDispatchRequest<std::string>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true> *,std::string const&,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
// was: __ZN3RBX13LuaWebService21TryRawDispatchRequestISsEEbPNS_14AsyncHttpCacheINS0_26CachedRawLuaWebServiceInfoELb1EEERKSsN5boost8functionIFvT_EEENS9_IFvSsEEE
pub fn stub_3471d0() -> ! {
    todo!("0x3471d0 bool RBX::LuaWebService::TryRawDispatchRequest<std::string>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true> *,std::string const&,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")
}

// 0x347418 — __ZN3RBX13LuaWebService10checkCacheIN5boost10shared_ptrIKSt6vectorINS_10Reflection7VariantESaIS6_EEEEEEbRKSsNS2_8functionIFvT_EEENSD_IFvSsEEE
#[doc(alias = "bool RBX::LuaWebService::checkCache<rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>(std::string const&,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)")]
// was: __ZN3RBX13LuaWebService10checkCacheIN5boost10shared_ptrIKSt6vectorINS_10Reflection7VariantESaIS6_EEEEEEbRKSsNS2_8functionIFvT_EEENSD_IFvSsEEE
pub fn stub_347418() -> ! {
    todo!("0x347418 bool RBX::LuaWebService::checkCache<rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>(std::string const&,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)")
}

// 0x347518 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS2_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt6vectorINS2_10Reflection7VariantESaISB_EEEEEEENS7_IFvSsEEES4_NS_3argILi1EEESsSH_SJ_EENS_3_bi6bind_tIT_PFSO_T0_T1_T2_T3_T4_ENSM_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESV_SX_SY_SZ_S10_S11_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list_av_5<rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>,rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>>(void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)")]
// was: __ZN5boost4bindIvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS2_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt6vectorINS2_10Reflection7VariantESaISB_EEEEEEENS7_IFvSsEEES4_NS_3argILi1EEESsSH_SJ_EENS_3_bi6bind_tIT_PFSO_T0_T1_T2_T3_T4_ENSM_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESV_SX_SY_SZ_S10_S11_
pub fn stub_347518() -> ! {
    todo!("0x347518 boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list_av_5<rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>,rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>>(void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)")
}

// 0x347a14 — __ZN3RBX13LuaWebService8CallbackIN5boost10shared_ptrIKSt6vectorINS_10Reflection7VariantESaIS6_EEEEEEvNS2_8weak_ptrIS0_EENS_14AsyncHttpQueue13RequestResultESsNS2_8functionIFvT_EEENSF_IFvSsEEE
#[doc(alias = "void RBX::LuaWebService::Callback<rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)")]
// was: __ZN3RBX13LuaWebService8CallbackIN5boost10shared_ptrIKSt6vectorINS_10Reflection7VariantESaIS6_EEEEEEvNS2_8weak_ptrIS0_EENS_14AsyncHttpQueue13RequestResultESsNS2_8functionIFvT_EEENSF_IFvSsEEE
pub fn stub_347a14() -> ! {
    todo!("0x347a14 void RBX::LuaWebService::Callback<rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)")
}

// 0x347c18 — __ZN3RBX9weak_fromINS_13LuaWebServiceEEEN5boost8weak_ptrIT_EEPS4_
#[doc(alias = "rbx_core::WeakPtr<RBX::LuaWebService> RBX::weak_from<RBX::LuaWebService>(RBX::LuaWebService*)")]
// was: __ZN3RBX9weak_fromINS_13LuaWebServiceEEEN5boost8weak_ptrIT_EEPS4_
pub fn stub_347c18() -> ! {
    todo!("0x347c18 rbx_core::WeakPtr<RBX::LuaWebService> RBX::weak_from<RBX::LuaWebService>(RBX::LuaWebService*)")
}

// 0x347e10 — __ZN5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS3_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt6vectorINS3_10Reflection7VariantESaISC_EEEEEEENS8_IFvSsEEEENS0_5list5INS0_5valueIS5_EENS_3argILi1EEENSO_ISsEENSO_ISI_EENSO_ISK_EEEEED1Ev
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::~bind_t()")]
// was: __ZN5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS3_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt6vectorINS3_10Reflection7VariantESaISC_EEEEEEENS8_IFvSsEEEENS0_5list5INS0_5valueIS5_EENS_3argILi1EEENSO_ISsEENSO_ISI_EENSO_ISK_EEEEED1Ev
pub fn stub_347e10() -> ! {
    todo!("0x347e10 boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::~bind_t()")
}

// 0x347fc8 — __ZN5boost4bindINS_8functionIFvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS6_EEEEEEEESF_EENS_3_bi6bind_tINSI_11unspecifiedET_NSI_9list_av_1IT0_E4typeEEESL_SN_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::_bi::list_av_1<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::type> boost::bind<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>(boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)")]
// was: __ZN5boost4bindINS_8functionIFvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS6_EEEEEEEESF_EENS_3_bi6bind_tINSI_11unspecifiedET_NSI_9list_av_1IT0_E4typeEEESL_SN_
pub fn stub_347fc8() -> ! {
    todo!("0x347fc8 boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::_bi::list_av_1<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::type> boost::bind<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>(boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)")
}

// 0x3480fc — __ZN5boost4bindIvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS2_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt3mapISsNS2_10Reflection7VariantESt4lessISsESaISt4pairIKSsSB_EEEEEEEENS7_IFvSsEEES4_NS_3argILi1EEESsSM_SO_EENS_3_bi6bind_tIT_PFST_T0_T1_T2_T3_T4_ENSR_9list_av_5IT5_T6_T7_T8_T9_E4typeEEES10_S12_S13_S14_S15_S16_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list_av_5<rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>,rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>>(void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>)")]
// was: __ZN5boost4bindIvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS2_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt3mapISsNS2_10Reflection7VariantESt4lessISsESaISt4pairIKSsSB_EEEEEEEENS7_IFvSsEEES4_NS_3argILi1EEESsSM_SO_EENS_3_bi6bind_tIT_PFST_T0_T1_T2_T3_T4_ENSR_9list_av_5IT5_T6_T7_T8_T9_E4typeEEES10_S12_S13_S14_S15_S16_
pub fn stub_3480fc() -> ! {
    todo!("0x3480fc boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list_av_5<rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>,rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>>(void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>)")
}

// 0x3485f8 — __ZN3RBX13LuaWebService8CallbackIN5boost10shared_ptrIKSt3mapISsNS_10Reflection7VariantESt4lessISsESaISt4pairIKSsS6_EEEEEEEvNS2_8weak_ptrIS0_EENS_14AsyncHttpQueue13RequestResultESsNS2_8functionIFvT_EEENSK_IFvSsEEE
// type: void __fastcall(int, int, int, int, int)
#[doc(alias = "void RBX::LuaWebService::Callback<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>)")]
// was: __ZN3RBX13LuaWebService8CallbackIN5boost10shared_ptrIKSt3mapISsNS_10Reflection7VariantESt4lessISsESaISt4pairIKSsS6_EEEEEEEvNS2_8weak_ptrIS0_EENS_14AsyncHttpQueue13RequestResultESsNS2_8functionIFvT_EEENSK_IFvSsEEE
pub fn stub_3485f8() -> ! {
    todo!("0x3485f8 void RBX::LuaWebService::Callback<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>)")
}

// 0x3487fc — __ZN5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS3_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt3mapISsNS3_10Reflection7VariantESt4lessISsESaISt4pairIKSsSC_EEEEEEEENS8_IFvSsEEEENS0_5list5INS0_5valueIS5_EENS_3argILi1EEENST_ISsEENST_ISN_EENST_ISP_EEEEED1Ev
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::~bind_t()")]
// was: __ZN5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS3_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt3mapISsNS3_10Reflection7VariantESt4lessISsESaISt4pairIKSsSC_EEEEEEEENS8_IFvSsEEEENS0_5list5INS0_5valueIS5_EENS_3argILi1EEENST_ISsEENST_ISN_EENST_ISP_EEEEED1Ev
pub fn stub_3487fc() -> ! {
    todo!("0x3487fc boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::~bind_t()")
}

// 0x3489b4 — __ZN3RBX13LuaWebService10checkCacheIbEEbRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE
// type: int __fastcall(int, int, int, int)
#[doc(alias = "bool RBX::LuaWebService::checkCache<bool>(std::string const&,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// was: __ZN3RBX13LuaWebService10checkCacheIbEEbRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE
pub fn stub_3489b4() -> ! {
    todo!("0x3489b4 bool RBX::LuaWebService::checkCache<bool>(std::string const&,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")
}

// 0x348ab4 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS2_14AsyncHttpQueue13RequestResultESsNS_8functionIFvbEEENS7_IFvSsEEES4_NS_3argILi1EEESsS9_SB_EENS_3_bi6bind_tIT_PFSG_T0_T1_T2_T3_T4_ENSE_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESN_SP_SQ_SR_SS_ST_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list_av_5<rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>,rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>(void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// was: __ZN5boost4bindIvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS2_14AsyncHttpQueue13RequestResultESsNS_8functionIFvbEEENS7_IFvSsEEES4_NS_3argILi1EEESsS9_SB_EENS_3_bi6bind_tIT_PFSG_T0_T1_T2_T3_T4_ENSE_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESN_SP_SQ_SR_SS_ST_
pub fn stub_348ab4() -> ! {
    todo!("0x348ab4 boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list_av_5<rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>,rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>(void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")
}

// 0x348fb0 — __ZN3RBX13LuaWebService8CallbackIbEEvN5boost8weak_ptrIS0_EENS_14AsyncHttpQueue13RequestResultESsNS2_8functionIFvT_EEENS7_IFvSsEEE
#[doc(alias = "void RBX::LuaWebService::Callback<bool>(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// was: __ZN3RBX13LuaWebService8CallbackIbEEvN5boost8weak_ptrIS0_EENS_14AsyncHttpQueue13RequestResultESsNS2_8functionIFvT_EEENS7_IFvSsEEE
pub fn stub_348fb0() -> ! {
    todo!("0x348fb0 void RBX::LuaWebService::Callback<bool>(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")
}

// 0x3491b4 — __ZN5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS3_14AsyncHttpQueue13RequestResultESsNS_8functionIFvbEEENS8_IFvSsEEEENS0_5list5INS0_5valueIS5_EENS_3argILi1EEENSG_ISsEENSG_ISA_EENSG_ISC_EEEEED1Ev
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::~bind_t()")]
// was: __ZN5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS3_14AsyncHttpQueue13RequestResultESsNS_8functionIFvbEEENS8_IFvSsEEEENS0_5list5INS0_5valueIS5_EENS_3argILi1EEENSG_ISsEENSG_ISA_EENSG_ISC_EEEEED1Ev
pub fn stub_3491b4() -> ! {
    todo!("0x3491b4 boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::~bind_t()")
}

// 0x34936c — __ZN3RBX13LuaWebService10checkCacheIiEEbRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE
#[doc(alias = "bool RBX::LuaWebService::checkCache<int>(std::string const&,boost::function<void ()(int)>,boost::function<void ()(std::string)>)")]
// was: __ZN3RBX13LuaWebService10checkCacheIiEEbRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE
pub fn stub_34936c() -> ! {
    todo!("0x34936c bool RBX::LuaWebService::checkCache<int>(std::string const&,boost::function<void ()(int)>,boost::function<void ()(std::string)>)")
}

// 0x34946c — __ZN5boost4bindIvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS2_14AsyncHttpQueue13RequestResultESsNS_8functionIFviEEENS7_IFvSsEEES4_NS_3argILi1EEESsS9_SB_EENS_3_bi6bind_tIT_PFSG_T0_T1_T2_T3_T4_ENSE_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESN_SP_SQ_SR_SS_ST_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list_av_5<rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>,rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>>(void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>)")]
// was: __ZN5boost4bindIvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS2_14AsyncHttpQueue13RequestResultESsNS_8functionIFviEEENS7_IFvSsEEES4_NS_3argILi1EEESsS9_SB_EENS_3_bi6bind_tIT_PFSG_T0_T1_T2_T3_T4_ENSE_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESN_SP_SQ_SR_SS_ST_
pub fn stub_34946c() -> ! {
    todo!("0x34946c boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list_av_5<rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>,rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>>(void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>)")
}

// 0x349968 — __ZN3RBX13LuaWebService8CallbackIiEEvN5boost8weak_ptrIS0_EENS_14AsyncHttpQueue13RequestResultESsNS2_8functionIFvT_EEENS7_IFvSsEEE
#[doc(alias = "void RBX::LuaWebService::Callback<int>(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>)")]
// was: __ZN3RBX13LuaWebService8CallbackIiEEvN5boost8weak_ptrIS0_EENS_14AsyncHttpQueue13RequestResultESsNS2_8functionIFvT_EEENS7_IFvSsEEE
pub fn stub_349968() -> ! {
    todo!("0x349968 void RBX::LuaWebService::Callback<int>(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>)")
}

// 0x349b6c — __ZN5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS3_14AsyncHttpQueue13RequestResultESsNS_8functionIFviEEENS8_IFvSsEEEENS0_5list5INS0_5valueIS5_EENS_3argILi1EEENSG_ISsEENSG_ISA_EENSG_ISC_EEEEED1Ev
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::~bind_t()")]
// was: __ZN5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS3_14AsyncHttpQueue13RequestResultESsNS_8functionIFviEEENS8_IFvSsEEEENS0_5list5INS0_5valueIS5_EENS_3argILi1EEENSG_ISsEENSG_ISA_EENSG_ISC_EEEEED1Ev
pub fn stub_349b6c() -> ! {
    todo!("0x349b6c boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::~bind_t()")
}

// 0x349d24 — __ZN3RBX13LuaWebService10checkCacheISsEEbRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE
#[doc(alias = "bool RBX::LuaWebService::checkCache<std::string>(std::string const&,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
// was: __ZN3RBX13LuaWebService10checkCacheISsEEbRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE
pub fn stub_349d24() -> ! {
    todo!("0x349d24 bool RBX::LuaWebService::checkCache<std::string>(std::string const&,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")
}

// 0x349e24 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS2_14AsyncHttpQueue13RequestResultESsNS_8functionIFvSsEEES9_S4_NS_3argILi1EEESsS9_S9_EENS_3_bi6bind_tIT_PFSE_T0_T1_T2_T3_T4_ENSC_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESL_SN_SO_SP_SQ_SR_
// type: void __fastcall(int, int, int, const std::string *, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list_av_5<rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>,rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>>(void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
// was: __ZN5boost4bindIvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS2_14AsyncHttpQueue13RequestResultESsNS_8functionIFvSsEEES9_S4_NS_3argILi1EEESsS9_S9_EENS_3_bi6bind_tIT_PFSE_T0_T1_T2_T3_T4_ENSC_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESL_SN_SO_SP_SQ_SR_
pub fn stub_349e24() -> ! {
    todo!("0x349e24 boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list_av_5<rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>,rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>>(void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")
}

// 0x34a320 — __ZN5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS3_14AsyncHttpQueue13RequestResultESsNS_8functionIFvSsEEESA_ENS0_5list5INS0_5valueIS5_EENS_3argILi1EEENSE_ISsEENSE_ISA_EESJ_EEED1Ev
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::~bind_t()")]
// was: __ZN5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS3_14AsyncHttpQueue13RequestResultESsNS_8functionIFvSsEEESA_ENS0_5list5INS0_5valueIS5_EENS_3argILi1EEENSE_ISsEENSE_ISA_EESJ_EEED1Ev
pub fn stub_34a320() -> ! {
    todo!("0x34a320 boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::~bind_t()")
}

// 0x34a4dc — __ZN3RBX13LuaWebServiceD1Ev
// type: void __fastcall(RBX::LuaWebService *__hidden this)
#[doc(alias = "RBX::LuaWebService::~LuaWebService()")]
// was: __ZN3RBX13LuaWebServiceD1Ev
pub fn stub_34a4dc() -> ! {
    todo!("0x34a4dc RBX::LuaWebService::~LuaWebService()")
}

// 0x34a5ec — __ZN3RBX13LuaWebServiceD0Ev
// type: void __fastcall(RBX::LuaWebService *__hidden this)
#[doc(alias = "RBX::LuaWebService::~LuaWebService()")]
// was: __ZN3RBX13LuaWebServiceD0Ev
pub fn stub_34a5ec() -> ! {
    todo!("0x34a5ec RBX::LuaWebService::~LuaWebService()")
}

// 0x34a714 — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEE12getClassNameEv
pub fn stub_34a714() -> ! {
    todo!("0x34a714 __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEE12getClassNameEv")
}

// 0x34a740 — __ZThn32_N3RBX13LuaWebServiceD1Ev
// type: void __fastcall(RBX::LuaWebService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::LuaWebService::~LuaWebService()")]
// was: __ZThn32_N3RBX13LuaWebServiceD1Ev
pub fn stub_34a740() -> ! {
    todo!("0x34a740 non-virtual thunk toRBX::LuaWebService::~LuaWebService()")
}

// 0x34a84c — __ZThn32_N3RBX13LuaWebServiceD0Ev
// type: void __fastcall(RBX::LuaWebService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::LuaWebService::~LuaWebService()")]
// was: __ZThn32_N3RBX13LuaWebServiceD0Ev
pub fn stub_34a84c() -> ! {
    todo!("0x34a84c non-virtual thunk toRBX::LuaWebService::~LuaWebService()")
}

// 0x34a970 — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEE12getClassNameEv
pub fn stub_34a970() -> ! {
    todo!("0x34a970 __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEE12getClassNameEv")
}

// 0x34a998 — __ZThn36_N3RBX13LuaWebServiceD1Ev
// type: void __fastcall(RBX::LuaWebService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::LuaWebService::~LuaWebService()")]
// was: __ZThn36_N3RBX13LuaWebServiceD1Ev
pub fn stub_34a998() -> ! {
    todo!("0x34a998 non-virtual thunk toRBX::LuaWebService::~LuaWebService()")
}

// 0x34aaa4 — __ZThn36_N3RBX13LuaWebServiceD0Ev
// type: void __fastcall(RBX::LuaWebService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::LuaWebService::~LuaWebService()")]
// was: __ZThn36_N3RBX13LuaWebServiceD0Ev
pub fn stub_34aaa4() -> ! {
    todo!("0x34aaa4 non-virtual thunk toRBX::LuaWebService::~LuaWebService()")
}

// 0x34abc8 — __ZN3RBX4Name13callDoDeclareILZNS_14sLuaWebServiceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sLuaWebServiceEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_14sLuaWebServiceEEEEvv
pub fn stub_34abc8() -> ! {
    todo!("0x34abc8 __ZN3RBX4Name13callDoDeclareILZNS_14sLuaWebServiceEEEEvv")
}

// 0x34abcc — __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS0_IFvSsEEESH_ENSB_5list5INSB_5valueISF_EENS_3argILi1EEENSL_ISsEENSL_ISH_EESQ_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int)
#[doc(alias = "__ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS0_IFvSsEEESH_ENSB_5list5INSB_5valueISF_EENS_3argILi1EEENSL_ISsEENSL_ISH_EESQ_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS0_IFvSsEEESH_ENSB_5list5INSB_5valueISF_EENS_3argILi1EEENSL_ISsEENSL_ISH_EESQ_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE
pub fn stub_34abcc() -> ! {
    todo!("0x34abcc __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS0_IFvSsEEESH_ENSB_5list5INSB_5valueISF_EENS_3argILi1EEENSL_ISsEENSL_ISH_EESQ_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE")
}

// 0x34ac8c — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS_8functionIFvSsEEESH_ENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSL_ISsEENSL_ISH_EESQ_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS_8functionIFvSsEEESH_ENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSL_ISsEENSL_ISH_EESQ_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS_8functionIFvSsEEESH_ENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSL_ISsEENSL_ISH_EESQ_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE
pub fn stub_34ac8c() -> ! {
    todo!("0x34ac8c __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS_8functionIFvSsEEESH_ENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSL_ISsEENSL_ISH_EESQ_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE")
}

// 0x34ad50 — __ZN5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS3_14AsyncHttpQueue13RequestResultESsNS_8functionIFvSsEEESA_ENS0_5list5INS0_5valueIS5_EENS_3argILi1EEENSE_ISsEENSE_ISA_EESJ_EEEC1ERKSL_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::bind_t(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>> const&)")]
// was: __ZN5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS3_14AsyncHttpQueue13RequestResultESsNS_8functionIFvSsEEESA_ENS0_5list5INS0_5valueIS5_EENS_3argILi1EEENSE_ISsEENSE_ISA_EESJ_EEEC1ERKSL_
pub fn stub_34ad50() -> ! {
    todo!("0x34ad50 boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::bind_t(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>> const&)")
}

// 0x34aea8 — __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvSsEEEEEEC2ERKSF_
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage4(boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>> const&)")]
// was: __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvSsEEEEEEC2ERKSF_
pub fn stub_34aea8() -> ! {
    todo!("0x34aea8 boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage4(boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>> const&)")
}

// 0x34b020 — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS_8functionIFvSsEEESH_ENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSL_ISsEENSL_ISH_EESQ_EEEEEEvT_
#[doc(alias = "void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)")]
// was: __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS_8functionIFvSsEEESH_ENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSL_ISsEENSL_ISH_EESQ_EEEEEEvT_
pub fn stub_34b020() -> ! {
    todo!("0x34b020 void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)")
}

// 0x34b0f4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvSsEEESD_ENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSH_ISsEENSH_ISD_EESM_EEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvSsEEESD_ENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSH_ISsEENSH_ISD_EESM_EEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE
pub fn stub_34b0f4() -> ! {
    todo!("0x34b0f4 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x34b110 — __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvSsEEESD_ENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSH_ISsEENSH_ISD_EESM_EEEEvSA_PSiNS_10shared_ptrIKSsEEE6invokeERNS1_15function_bufferESA_SP_SS_
#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvSsEEESD_ENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSH_ISsEENSH_ISD_EESM_EEEEvSA_PSiNS_10shared_ptrIKSsEEE6invokeERNS1_15function_bufferESA_SP_SS_
pub fn stub_34b110() -> ! {
    todo!("0x34b110 boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)")
}

// 0x34b134 — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13LuaWebServiceEEES5_SsNS_8functionIFvSsEEESJ_ENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSN_ISsEENSN_ISJ_EESS_EEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13LuaWebServiceEEES5_SsNS_8functionIFvSsEEESJ_ENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSN_ISsEENSN_ISJ_EESS_EEEEEEbT_RNS1_15function_bufferE
pub fn stub_34b134() -> ! {
    todo!("0x34b134 bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")
}

// 0x34b1f8 — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13LuaWebServiceEEES5_SsNS_8functionIFvSsEEESJ_ENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSN_ISsEENSN_ISJ_EESS_EEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13LuaWebServiceEEES5_SsNS_8functionIFvSsEEESJ_ENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSN_ISsEENSN_ISJ_EESS_EEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_34b1f8() -> ! {
    todo!("0x34b1f8 bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x34b2b8 — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13LuaWebServiceEEES5_SsNS_8functionIFvSsEEESJ_ENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSN_ISsEENSN_ISJ_EESS_EEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13LuaWebServiceEEES5_SsNS_8functionIFvSsEEESJ_ENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSN_ISsEENSN_ISJ_EESS_EEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
pub fn stub_34b2b8() -> ! {
    todo!("0x34b2b8 void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x34b364 — __ZN5boost3_bi5list5INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvSsEEEEESE_EclIPFvS6_NS4_14AsyncHttpQueue13RequestResultESsSD_SD_ENS0_5list3IRSI_RPSiRNS_10shared_ptrIKSsEEEEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &> &,int)")]
// was: __ZN5boost3_bi5list5INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvSsEEEEESE_EclIPFvS6_NS4_14AsyncHttpQueue13RequestResultESsSD_SD_ENS0_5list3IRSI_RPSiRNS_10shared_ptrIKSsEEEEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_34b364() -> ! {
    todo!("0x34b364 void boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &> &,int)")
}

// 0x34b584 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvSsEEESD_ENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSH_ISsEENSH_ISD_EESM_EEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvSsEEESD_ENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSH_ISsEENSH_ISD_EESM_EEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_34b584() -> ! {
    todo!("0x34b584 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x34b6b0 — __ZN5boost3_bi5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEC1ERKS5_
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
#[doc(alias = "boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>::value(rbx_core::WeakPtr<RBX::LuaWebService> const&)")]
// was: __ZN5boost3_bi5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEC1ERKS5_
pub fn stub_34b6b0() -> ! {
    todo!("0x34b6b0 boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>::value(rbx_core::WeakPtr<RBX::LuaWebService> const&)")
}

// 0x34b714 — __ZN5boost3_bi5list5INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvSsEEEEESE_EC2ES7_S9_SA_SE_SE_
#[doc(alias = "boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list5(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// was: __ZN5boost3_bi5list5INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvSsEEEEESE_EC2ES7_S9_SA_SE_SE_
pub fn stub_34b714() -> ! {
    todo!("0x34b714 boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list5(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")
}

// 0x34b924 — __ZN5boost3_bi8storage5INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvSsEEEEESE_EC2ES7_S9_SA_SE_SE_
#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage5(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// was: __ZN5boost3_bi8storage5INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvSsEEEEESE_EC2ES7_S9_SA_SE_SE_
pub fn stub_34b924() -> ! {
    todo!("0x34b924 boost::_bi::storage5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage5(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")
}

