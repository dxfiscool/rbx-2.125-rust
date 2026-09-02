//! rendering shard 464 — 100 stubs 0x7038f8..0x707998 EA-sorted asc global gap filler not yet in rbx_rendering (global gap filler, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Global gap filler fallback EA asc not yet in rbx_rendering (50122->50222 distinct, fallback after 0x7038f8).
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc global gap filler not yet in rbx_rendering

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x7038f8 — __ZN10XmlElement8addChildERKN3RBX4NameE
// type: _DWORD __fastcall(XmlElement *__hidden this, const RBX::Name *)
#[doc(alias = "XmlElement::addChild(RBX::Name const&)")]
#[doc(alias = "__ZN10XmlElement8addChildERKN3RBX4NameE")]
pub fn stub_7038f8() -> ! {
    todo!("0x7038f8 XmlElement::addChild(RBX::Name const&)")
}

// 0x7039cc — __ZN3RBX15ServiceProvider4findINS_13ScriptServiceEEEPT_PKNS_8InstanceE
#[doc(alias = "RBX::ScriptService * RBX::ServiceProvider::find<RBX::ScriptService>(RBX::Instance const*)")]
#[doc(alias = "__ZN3RBX15ServiceProvider4findINS_13ScriptServiceEEEPT_PKNS_8InstanceE")]
pub fn stub_7039cc() -> ! {
    todo!("0x7039cc RBX::ScriptService * RBX::ServiceProvider::find<RBX::ScriptService>(RBX::Instance const*)")
}

// 0x7039e4 — __ZN3RBX9weak_fromINS_8InstanceEEEN5boost8weak_ptrIT_EEPS4_
#[doc(alias = "rbx_core::WeakPtr<RBX::Instance> RBX::weak_from<RBX::Instance>(RBX::Instance*)")]
#[doc(alias = "__ZN3RBX9weak_fromINS_8InstanceEEEN5boost8weak_ptrIT_EEPS4_")]
// was: boost::weak_ptr<RBX::Instance> RBX::weak_from<RBX::Instance>(RBX::Instance*)
pub fn stub_7039e4() -> ! {
    todo!("0x7039e4 boost::weak_ptr<RBX::Instance> RBX::weak_from<RBX::Instance>(RBX::Instance*)")
}

// 0x703be0 — __ZNK3RBX8Security7Context17requirePermissionENS0_11PermissionsEPKc
#[doc(alias = "RBX::Security::Context::requirePermission(RBX::Security::Permissions,char const*)const")]
#[doc(alias = "__ZNK3RBX8Security7Context17requirePermissionENS0_11PermissionsEPKc")]
pub fn stub_703be0() -> ! {
    todo!("0x703be0 RBX::Security::Context::requirePermission(RBX::Security::Permissions,char const*)const")
}

// 0x703cc0 — __ZN3RBX8Instance18childRemovedSignalERN5boost10shared_ptrIS0_EE
#[doc(alias = "RBX::Instance::childRemovedSignal(rbx_core::SharedPtr<RBX::Instance> &)")]
#[doc(alias = "__ZN3RBX8Instance18childRemovedSignalERN5boost10shared_ptrIS0_EE")]
// was: RBX::Instance::childRemovedSignal(boost::shared_ptr<RBX::Instance> &)
pub fn stub_703cc0() -> ! {
    todo!("0x703cc0 RBX::Instance::childRemovedSignal(boost::shared_ptr<RBX::Instance> &)")
}

// 0x703da4 — __ZN3RBX8Instance22ChildRemovedSignalDataD1Ev
// type: void __fastcall(RBX::Instance::ChildRemovedSignalData *__hidden this)
#[doc(alias = "RBX::Instance::ChildRemovedSignalData::~ChildRemovedSignalData()")]
#[doc(alias = "__ZN3RBX8Instance22ChildRemovedSignalDataD1Ev")]
pub fn stub_703da4() -> ! {
    todo!("0x703da4 RBX::Instance::ChildRemovedSignalData::~ChildRemovedSignalData()")
}

// 0x703dc8 — __ZN3RBX8Instance16childAddedSignalERN5boost10shared_ptrIS0_EE
#[doc(alias = "RBX::Instance::childAddedSignal(rbx_core::SharedPtr<RBX::Instance> &)")]
#[doc(alias = "__ZN3RBX8Instance16childAddedSignalERN5boost10shared_ptrIS0_EE")]
// was: RBX::Instance::childAddedSignal(boost::shared_ptr<RBX::Instance> &)
pub fn stub_703dc8() -> ! {
    todo!("0x703dc8 RBX::Instance::childAddedSignal(boost::shared_ptr<RBX::Instance> &)")
}

// 0x703eac — __ZN3RBX8Instance20ChildAddedSignalDataD1Ev
// type: void __fastcall(RBX::Instance::ChildAddedSignalData *__hidden this)
#[doc(alias = "RBX::Instance::ChildAddedSignalData::~ChildAddedSignalData()")]
#[doc(alias = "__ZN3RBX8Instance20ChildAddedSignalDataD1Ev")]
pub fn stub_703eac() -> ! {
    todo!("0x703eac RBX::Instance::ChildAddedSignalData::~ChildAddedSignalData()")
}

// 0x703ed0 — __ZN3RBX8Instance25AncestryChangedSignalDataD1Ev
// type: void __fastcall(RBX::Instance::AncestryChangedSignalData *__hidden this)
#[doc(alias = "RBX::Instance::AncestryChangedSignalData::~AncestryChangedSignalData()")]
#[doc(alias = "__ZN3RBX8Instance25AncestryChangedSignalDataD1Ev")]
pub fn stub_703ed0() -> ! {
    todo!("0x703ed0 RBX::Instance::AncestryChangedSignalData::~AncestryChangedSignalData()")
}

// 0x703fb0 — __ZN3rbx7signals16signal_with_argsILi2EFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EEclES6_S6_
#[doc(alias = "rbx::signals::signal_with_args<2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::operator()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi2EFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EEclES6_S6_")]
// was: rbx::signals::signal_with_args<2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::operator()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)
pub fn stub_703fb0() -> ! {
    todo!("0x703fb0 rbx::signals::signal_with_args<2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::operator()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)")
}

// 0x704228 — __ZN3RBX8Instance24descendantRemovingSignalERKN5boost10shared_ptrIS0_EE
#[doc(alias = "RBX::Instance::descendantRemovingSignal(rbx_core::SharedPtr<RBX::Instance> const&)")]
#[doc(alias = "__ZN3RBX8Instance24descendantRemovingSignalERKN5boost10shared_ptrIS0_EE")]
// was: RBX::Instance::descendantRemovingSignal(boost::shared_ptr<RBX::Instance> const&)
pub fn stub_704228() -> ! {
    todo!("0x704228 RBX::Instance::descendantRemovingSignal(boost::shared_ptr<RBX::Instance> const&)")
}

// 0x70430c — __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvPFvNS2_10shared_ptrIS0_EEPiENS3_5list2INS2_3argILi1EEENS3_5valueIS7_EEEEEEEEvRKT_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,int *),boost::_bi::list2<boost::arg<1>,boost::_bi::value<int *>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,int *),boost::_bi::list2<boost::arg<1>,boost::_bi::value<int *>>> const&)const")]
#[doc(alias = "__ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvPFvNS2_10shared_ptrIS0_EEPiENS3_5list2INS2_3argILi1EEENS3_5valueIS7_EEEEEEEEvRKT_")]
// was: void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>,int *),boost::_bi::list2<boost::arg<1>,boost::_bi::value<int *>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>,int *),boost::_bi::list2<boost::arg<1>,boost::_bi::value<int *>>> const&)const
pub fn stub_70430c() -> ! {
    todo!("0x70430c void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>,int *),boost::_bi::list2<boost::arg<1>,boost::_bi::value<int *>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>,int *),boost::_bi::list2<boost::arg<1>,boost::_bi::value<int *>>> const&)const")
}

// 0x704414 — __ZN3RBX8GuidItemINS_8InstanceEEC2Ev
#[doc(alias = "RBX::GuidItem<RBX::Instance>::GuidItem(void)")]
#[doc(alias = "__ZN3RBX8GuidItemINS_8InstanceEEC2Ev")]
pub fn stub_704414() -> ! {
    todo!("0x704414 RBX::GuidItem<RBX::Instance>::GuidItem(void)")
}

// 0x7044e4 — __ZN3RBX6FWBase4initINS_10FWInstanceEEEPT_S4_
#[doc(alias = "RBX::FWInstance * RBX::FWBase::init<RBX::FWInstance>(RBX::FWInstance *)")]
#[doc(alias = "__ZN3RBX6FWBase4initINS_10FWInstanceEEEPT_S4_")]
pub fn stub_7044e4() -> ! {
    todo!("0x7044e4 RBX::FWInstance * RBX::FWBase::init<RBX::FWInstance>(RBX::FWInstance *)")
}

// 0x7045b0 — __ZN3RBX8GuidItemINS_8InstanceEED2Ev
#[doc(alias = "RBX::GuidItem<RBX::Instance>::~GuidItem()")]
#[doc(alias = "__ZN3RBX8GuidItemINS_8InstanceEED2Ev")]
pub fn stub_7045b0() -> ! {
    todo!("0x7045b0 RBX::GuidItem<RBX::Instance>::~GuidItem()")
}

// 0x704688 — __ZN3RBX13FWStringValueC1EPKc
// type: _DWORD __fastcall(RBX::FWStringValue *__hidden this, const char *__s)
#[doc(alias = "RBX::FWStringValue::FWStringValue(char const*)")]
#[doc(alias = "__ZN3RBX13FWStringValueC1EPKc")]
pub fn stub_704688() -> ! {
    todo!("0x704688 RBX::FWStringValue::FWStringValue(char const*)")
}

// 0x704748 — __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEENS2_3_bi6bind_tIvNS2_4_mfi3mf0IvS5_EENSD_5list1INS2_3argILi1EEEEEEEET0_T_SO_SN_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Instance>,boost::_bi::list1<boost::arg<1>>> std::for_each<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Instance>,boost::_bi::list1<boost::arg<1>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Instance>,boost::_bi::list1<boost::arg<1>>>)")]
#[doc(alias = "__ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEENS2_3_bi6bind_tIvNS2_4_mfi3mf0IvS5_EENSD_5list1INS2_3argILi1EEEEEEEET0_T_SO_SN_")]
// was: boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Instance>,boost::_bi::list1<boost::arg<1>>> std::for_each<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Instance>,boost::_bi::list1<boost::arg<1>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Instance>,boost::_bi::list1<boost::arg<1>>>)
pub fn stub_704748() -> ! {
    todo!("0x704748 boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Instance>,boost::_bi::list1<boost::arg<1>>> std::for_each<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Instance>,boost::_bi::list1<boost::arg<1>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Instance>,boost::_bi::list1<boost::arg<1>>>)")
}

// 0x704794 — __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPKPN3RBX10Reflection15EventDescriptorESt6vectorIS5_SaIS5_EEEEN5boost3_bi6bind_tIvNSC_4_mfi4cmf1IvS4_PNS3_11EventSourceEEENSD_5list2INSC_3argILi1EEENSD_5valueIPNS2_8InstanceEEEEEEEET0_T_SU_ST_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::Reflection::EventDescriptor,RBX::Reflection::EventSource *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Instance *>>> std::for_each<__gnu_cxx::__normal_iterator<RBX::Reflection::EventDescriptor * const*,std::vector<RBX::Reflection::EventDescriptor *,std::allocator<RBX::Reflection::EventDescriptor *>>>,boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::Reflection::EventDescriptor,RBX::Reflection::EventSource *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Instance *>>>>(__gnu_cxx::__normal_iterator<RBX::Reflection::EventDescriptor * const*,std::vector<RBX::Reflection::EventDescriptor *,std::allocator<RBX::Reflection::EventDescriptor *>>>,__gnu_cxx::__normal_iterator<RBX::Reflection::EventDescriptor * const*,std::vector<RBX::Reflection::EventDescriptor *,std::allocator<RBX::Reflection::EventDescriptor *>>>,boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::Reflection::EventDescriptor,RBX::Reflection::EventSource *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Instance *>>>)")]
#[doc(alias = "__ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPKPN3RBX10Reflection15EventDescriptorESt6vectorIS5_SaIS5_EEEEN5boost3_bi6bind_tIvNSC_4_mfi4cmf1IvS4_PNS3_11EventSourceEEENSD_5list2INSC_3argILi1EEENSD_5valueIPNS2_8InstanceEEEEEEEET0_T_SU_ST_")]
pub fn stub_704794() -> ! {
    todo!("0x704794 boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::Reflection::EventDescriptor,RBX::Reflection::EventSource *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Instance *>>> std::for_each<__gnu_cxx::__normal_iterator<RBX::Reflection::EventDescriptor * const*,std::vector<RBX::Reflection::EventDescriptor *,std::allocator<RBX::Reflection::EventDescriptor *>>>,boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::Reflection::EventDescriptor,RBX::Reflection::EventSource *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Instance *>>>>(__gnu_cxx::__normal_iterator<RBX::Reflection::EventDescriptor * const*,std::vector<RBX::Reflection::EventDescriptor *,std::allocator<RBX::Reflection::EventDescriptor *>>>,__gnu_cxx::__normal_iterator<RBX::Reflection::EventDescriptor * const*,std::vector<RBX::Reflection::EventDescriptor *,std::allocator<RBX::Reflection::EventDescriptor *>>>,boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::Reflection::EventDescriptor,RBX::Reflection::EventSource *>,boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::Instance *>>>)")
}

// 0x7047ec — __ZN3RBX7FWValueISsE3setERKSsPNS_5FWRefE
// type: int __fastcall(std::string *, std::string *this, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, boost::detail::sp_counted_base *, int, char, int, int, int, int)
#[doc(alias = "RBX::FWValue<std::string>::set(std::string const&,RBX::FWRef *)")]
#[doc(alias = "__ZN3RBX7FWValueISsE3setERKSsPNS_5FWRefE")]
pub fn stub_7047ec() -> ! {
    todo!("0x7047ec RBX::FWValue<std::string>::set(std::string const&,RBX::FWRef *)")
}

// 0x704ab8 — __ZN3RBX9AllocatorINS_16OnDemandInstanceEEnwEm
#[doc(alias = "RBX::Allocator<RBX::OnDemandInstance>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_16OnDemandInstanceEEnwEm")]
pub fn stub_704ab8() -> ! {
    todo!("0x704ab8 RBX::Allocator<RBX::OnDemandInstance>::operator new(unsigned long)")
}

// 0x704b28 — __ZN3RBX9AllocatorINS_16OnDemandInstanceEEdlEPv
// type: void __fastcall(void *)
#[doc(alias = "RBX::Allocator<RBX::OnDemandInstance>::operator delete(void *)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_16OnDemandInstanceEEdlEPv")]
pub fn stub_704b28() -> ! {
    todo!("0x704b28 RBX::Allocator<RBX::OnDemandInstance>::operator delete(void *)")
}

// 0x704b68 — __ZN3RBX8Instance25PropertyChangedSignalDataD1Ev
// type: void __fastcall(RBX::Instance::PropertyChangedSignalData *__hidden this)
#[doc(alias = "RBX::Instance::PropertyChangedSignalData::~PropertyChangedSignalData()")]
#[doc(alias = "__ZN3RBX8Instance25PropertyChangedSignalDataD1Ev")]
pub fn stub_704b68() -> ! {
    todo!("0x704b68 RBX::Instance::PropertyChangedSignalData::~PropertyChangedSignalData()")
}

// 0x704b6c — __ZN3RBX6FWBaseC2Ev
// type: _DWORD __fastcall(RBX::FWBase *__hidden this)
#[doc(alias = "RBX::FWBase::FWBase(void)")]
#[doc(alias = "__ZN3RBX6FWBaseC2Ev")]
pub fn stub_704b6c() -> ! {
    todo!("0x704b6c RBX::FWBase::FWBase(void)")
}

// 0x704c70 — __ZN3RBX9AllocatorINS_10FWInstanceEEC2Ev
#[doc(alias = "RBX::Allocator<RBX::FWInstance>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_10FWInstanceEEC2Ev")]
pub fn stub_704c70() -> ! {
    todo!("0x704c70 RBX::Allocator<RBX::FWInstance>::Allocator(void)")
}

// 0x704cd8 — __ZNK3RBX6FWBaseeqERKS0_
// type: int __fastcall(char *)
#[doc(alias = "RBX::FWBase::operator==(RBX::FWBase const&)const")]
#[doc(alias = "__ZNK3RBX6FWBaseeqERKS0_")]
pub fn stub_704cd8() -> ! {
    todo!("0x704cd8 RBX::FWBase::operator==(RBX::FWBase const&)const")
}

// 0x704d88 — __ZN3RBX10FWInstanceD0Ev
// type: void __fastcall(RBX::FWInstance *__hidden this)
#[doc(alias = "RBX::FWInstance::~FWInstance()")]
#[doc(alias = "__ZN3RBX10FWInstanceD0Ev")]
pub fn stub_704d88() -> ! {
    todo!("0x704d88 RBX::FWInstance::~FWInstance()")
}

// 0x704e58 — __ZN3RBX9AllocatorINS_10FWInstanceEE13releaseMemoryEv
#[doc(alias = "RBX::Allocator<RBX::FWInstance>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_10FWInstanceEE13releaseMemoryEv")]
pub fn stub_704e58() -> ! {
    todo!("0x704e58 RBX::Allocator<RBX::FWInstance>::releaseMemory(void)")
}

// 0x704e78 — __ZN5boost14singleton_poolIN3RBX10FWInstanceELj28ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
#[doc(alias = "boost::singleton_pool<RBX::FWInstance,28u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX10FWInstanceELj28ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
pub fn stub_704e78() -> ! {
    todo!("0x704e78 boost::singleton_pool<RBX::FWInstance,28u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")
}

// 0x704ea8 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE22safe_static_init_mutexEv")]
pub fn stub_704ea8() -> ! {
    todo!("0x704ea8 rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::safe_static_init_mutex(void)")
}

// 0x704eac — __ZN5boost14singleton_poolIN3RBX16OnDemandInstanceELj20ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
#[doc(alias = "boost::singleton_pool<RBX::OnDemandInstance,20u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX16OnDemandInstanceELj20ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")]
pub fn stub_704eac() -> ! {
    todo!("0x704eac boost::singleton_pool<RBX::OnDemandInstance,20u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")
}

// 0x704ee8 — __ZN3RBX8GuidItemINS_8InstanceEE8Registry10unregisterEPS2_
// type: int __fastcall(int, int, int, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "RBX::GuidItem<RBX::Instance>::Registry::unregister(RBX::GuidItem<RBX::Instance>*)")]
#[doc(alias = "__ZN3RBX8GuidItemINS_8InstanceEE8Registry10unregisterEPS2_")]
pub fn stub_704ee8() -> ! {
    todo!("0x704ee8 RBX::GuidItem<RBX::Instance>::Registry::unregister(RBX::GuidItem<RBX::Instance>*)")
}

// 0x705088 — __ZNSt8_Rb_treeIN3RBX4Guid4DataESt4pairIKS2_PNS0_8InstanceEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE5eraseERS4_
#[doc(alias = "std::_Rb_tree<RBX::Guid::Data,std::pair<RBX::Guid::Data const,RBX::Instance *>,std::_Select1st<std::pair<RBX::Guid::Data const,RBX::Instance *>>,std::less<RBX::Guid::Data>,std::allocator<std::pair<RBX::Guid::Data const,RBX::Instance *>>>::erase(RBX::Guid::Data const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX4Guid4DataESt4pairIKS2_PNS0_8InstanceEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE5eraseERS4_")]
pub fn stub_705088() -> ! {
    todo!("0x705088 std::_Rb_tree<RBX::Guid::Data,std::pair<RBX::Guid::Data const,RBX::Instance *>,std::_Select1st<std::pair<RBX::Guid::Data const,RBX::Instance *>>,std::less<RBX::Guid::Data>,std::allocator<std::pair<RBX::Guid::Data const,RBX::Instance *>>>::erase(RBX::Guid::Data const&)")
}

// 0x7050b0 — __ZNSt8_Rb_treeIN3RBX4Guid4DataESt4pairIKS2_PNS0_8InstanceEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE11equal_rangeERS4_
#[doc(alias = "std::_Rb_tree<RBX::Guid::Data,std::pair<RBX::Guid::Data const,RBX::Instance *>,std::_Select1st<std::pair<RBX::Guid::Data const,RBX::Instance *>>,std::less<RBX::Guid::Data>,std::allocator<std::pair<RBX::Guid::Data const,RBX::Instance *>>>::equal_range(RBX::Guid::Data const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX4Guid4DataESt4pairIKS2_PNS0_8InstanceEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE11equal_rangeERS4_")]
pub fn stub_7050b0() -> ! {
    todo!("0x7050b0 std::_Rb_tree<RBX::Guid::Data,std::pair<RBX::Guid::Data const,RBX::Instance *>,std::_Select1st<std::pair<RBX::Guid::Data const,RBX::Instance *>>,std::less<RBX::Guid::Data>,std::allocator<std::pair<RBX::Guid::Data const,RBX::Instance *>>>::equal_range(RBX::Guid::Data const&)")
}

// 0x705110 — __ZNSt8_Rb_treeIN3RBX4Guid4DataESt4pairIKS2_PNS0_8InstanceEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE5eraseESt17_Rb_tree_iteratorIS7_ESF_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Guid::Data,std::pair<RBX::Guid::Data const,RBX::Instance *>,std::_Select1st<std::pair<RBX::Guid::Data const,RBX::Instance *>>,std::less<RBX::Guid::Data>,std::allocator<std::pair<RBX::Guid::Data const,RBX::Instance *>>>::erase(std::_Rb_tree_iterator<std::pair<RBX::Guid::Data const,RBX::Instance *>>,std::_Rb_tree_iterator<std::pair<RBX::Guid::Data const,RBX::Instance *>>)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX4Guid4DataESt4pairIKS2_PNS0_8InstanceEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE5eraseESt17_Rb_tree_iteratorIS7_ESF_")]
pub fn stub_705110() -> ! {
    todo!("0x705110 std::_Rb_tree<RBX::Guid::Data,std::pair<RBX::Guid::Data const,RBX::Instance *>,std::_Select1st<std::pair<RBX::Guid::Data const,RBX::Instance *>>,std::less<RBX::Guid::Data>,std::allocator<std::pair<RBX::Guid::Data const,RBX::Instance *>>>::erase(std::_Rb_tree_iterator<std::pair<RBX::Guid::Data const,RBX::Instance *>>,std::_Rb_tree_iterator<std::pair<RBX::Guid::Data const,RBX::Instance *>>)")
}

// 0x705170 — __ZNSt8_Rb_treeIN3RBX4Guid4DataESt4pairIKS2_PNS0_8InstanceEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
#[doc(alias = "std::_Rb_tree<RBX::Guid::Data,std::pair<RBX::Guid::Data const,RBX::Instance *>,std::_Select1st<std::pair<RBX::Guid::Data const,RBX::Instance *>>,std::less<RBX::Guid::Data>,std::allocator<std::pair<RBX::Guid::Data const,RBX::Instance *>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Guid::Data const,RBX::Instance *>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX4Guid4DataESt4pairIKS2_PNS0_8InstanceEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")]
pub fn stub_705170() -> ! {
    todo!("0x705170 std::_Rb_tree<RBX::Guid::Data,std::pair<RBX::Guid::Data const,RBX::Instance *>,std::_Select1st<std::pair<RBX::Guid::Data const,RBX::Instance *>>,std::less<RBX::Guid::Data>,std::allocator<std::pair<RBX::Guid::Data const,RBX::Instance *>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Guid::Data const,RBX::Instance *>> *)")
}

// 0x705198 — __ZN3RBX10Reflection13DescribedBaseC2Ev
// type: _DWORD __fastcall(RBX::Reflection::DescribedBase *__hidden this)
#[doc(alias = "RBX::Reflection::DescribedBase::DescribedBase(void)")]
#[doc(alias = "__ZN3RBX10Reflection13DescribedBaseC2Ev")]
pub fn stub_705198() -> ! {
    todo!("0x705198 RBX::Reflection::DescribedBase::DescribedBase(void)")
}

// 0x705288 — __ZN3RBX10Reflection9DescribedINS_8InstanceELZNS_9sInstanceEENS0_13DescribedBaseELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8InstanceELZNS_9sInstanceEENS0_13DescribedBaseELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_705288() -> ! {
    todo!("0x705288 __ZN3RBX10Reflection9DescribedINS_8InstanceELZNS_9sInstanceEENS0_13DescribedBaseELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x70529c — __ZN3RBX10Reflection9DescribedINS_8InstanceELZNS_9sInstanceEENS0_13DescribedBaseELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8InstanceELZNS_9sInstanceEENS0_13DescribedBaseELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_70529c() -> ! {
    todo!("0x70529c __ZN3RBX10Reflection9DescribedINS_8InstanceELZNS_9sInstanceEENS0_13DescribedBaseELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x705340 — __ZN3RBX10Reflection13DescribedBaseD1Ev
// type: void __fastcall(RBX::Reflection::DescribedBase *__hidden this)
#[doc(alias = "RBX::Reflection::DescribedBase::~DescribedBase()")]
#[doc(alias = "__ZN3RBX10Reflection13DescribedBaseD1Ev")]
pub fn stub_705340() -> ! {
    todo!("0x705340 RBX::Reflection::DescribedBase::~DescribedBase()")
}

// 0x705354 — __ZN3RBX10Reflection13DescribedBaseD0Ev
// type: void __fastcall(RBX::Reflection::DescribedBase *__hidden this)
#[doc(alias = "RBX::Reflection::DescribedBase::~DescribedBase()")]
#[doc(alias = "__ZN3RBX10Reflection13DescribedBaseD0Ev")]
pub fn stub_705354() -> ! {
    todo!("0x705354 RBX::Reflection::DescribedBase::~DescribedBase()")
}

// 0x7053f8 — __ZN3RBX22AbstractFactoryProductINS_8InstanceEED1Ev
#[doc(alias = "RBX::AbstractFactoryProduct<RBX::Instance>::~AbstractFactoryProduct()")]
#[doc(alias = "__ZN3RBX22AbstractFactoryProductINS_8InstanceEED1Ev")]
pub fn stub_7053f8() -> ! {
    todo!("0x7053f8 RBX::AbstractFactoryProduct<RBX::Instance>::~AbstractFactoryProduct()")
}

// 0x7053fc — __ZN3RBX22AbstractFactoryProductINS_8InstanceEED0Ev
#[doc(alias = "RBX::AbstractFactoryProduct<RBX::Instance>::~AbstractFactoryProduct()")]
#[doc(alias = "__ZN3RBX22AbstractFactoryProductINS_8InstanceEED0Ev")]
pub fn stub_7053fc() -> ! {
    todo!("0x7053fc RBX::AbstractFactoryProduct<RBX::Instance>::~AbstractFactoryProduct()")
}

// 0x705400 — __ZN3RBX9AllocatorINS_10FWInstanceEEnwEm
#[doc(alias = "RBX::Allocator<RBX::FWInstance>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_10FWInstanceEEnwEm")]
pub fn stub_705400() -> ! {
    todo!("0x705400 RBX::Allocator<RBX::FWInstance>::operator new(unsigned long)")
}

// 0x705470 — __ZN3RBX7FWFinalINS_10FWInstanceEED1Ev
#[doc(alias = "RBX::FWFinal<RBX::FWInstance>::~FWFinal()")]
#[doc(alias = "__ZN3RBX7FWFinalINS_10FWInstanceEED1Ev")]
pub fn stub_705470() -> ! {
    todo!("0x705470 RBX::FWFinal<RBX::FWInstance>::~FWFinal()")
}

// 0x705474 — __ZN3RBX7FWFinalINS_10FWInstanceEED0Ev
#[doc(alias = "RBX::FWFinal<RBX::FWInstance>::~FWFinal()")]
#[doc(alias = "__ZN3RBX7FWFinalINS_10FWInstanceEED0Ev")]
pub fn stub_705474() -> ! {
    todo!("0x705474 RBX::FWFinal<RBX::FWInstance>::~FWFinal()")
}

// 0x705528 — __ZN3RBX7FWFinalINS_10FWInstanceEED2Ev
#[doc(alias = "RBX::FWFinal<RBX::FWInstance>::~FWFinal()")]
#[doc(alias = "__ZN3RBX7FWFinalINS_10FWInstanceEED2Ev")]
pub fn stub_705528() -> ! {
    todo!("0x705528 RBX::FWFinal<RBX::FWInstance>::~FWFinal()")
}

// 0x705634 — __ZN5boost14singleton_poolIN3RBX10FWInstanceELj28ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
#[doc(alias = "boost::singleton_pool<RBX::FWInstance,28u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX10FWInstanceELj28ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")]
pub fn stub_705634() -> ! {
    todo!("0x705634 boost::singleton_pool<RBX::FWInstance,28u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")
}

// 0x70566c — __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueIPiEEEclIPFvNS_10shared_ptrIN3RBX8InstanceEEES5_ENS0_5list1IRKSC_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::_bi::value<int *>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,int *),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,int *) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueIPiEEEclIPFvNS_10shared_ptrIN3RBX8InstanceEEES5_ENS0_5list1IRKSC_EEEEvNS0_4typeIvEERT_RT0_i")]
// was: void boost::_bi::list2<boost::arg<1>,boost::_bi::value<int *>>::operator()<void (*)(boost::shared_ptr<RBX::Instance>,int *),boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::Instance>,int *) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)
pub fn stub_70566c() -> ! {
    todo!("0x70566c void boost::_bi::list2<boost::arg<1>,boost::_bi::value<int *>>::operator()<void (*)(boost::shared_ptr<RBX::Instance>,int *),boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::Instance>,int *) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)")
}

// 0x705740 — __ZN3RBX9AllocatorI12XmlAttributeEdlEPv
// type: void __fastcall(void *)
#[doc(alias = "RBX::Allocator<XmlAttribute>::operator delete(void *)")]
#[doc(alias = "__ZN3RBX9AllocatorI12XmlAttributeEdlEPv")]
pub fn stub_705740() -> ! {
    todo!("0x705740 RBX::Allocator<XmlAttribute>::operator delete(void *)")
}

// 0x705780 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIbPFbPN3RBX8InstanceES7_ENS3_5list2INS3_5valueIS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<bool,bool (*)(RBX::Instance *,RBX::Instance *),boost::_bi::list2<boost::_bi::value<RBX::Instance *>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIbPFbPN3RBX8InstanceES7_ENS3_5list2INS3_5valueIS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE")]
pub fn stub_705780() -> ! {
    todo!("0x705780 boost::detail::function::functor_manager<boost::_bi::bind_t<bool,bool (*)(RBX::Instance *,RBX::Instance *),boost::_bi::list2<boost::_bi::value<RBX::Instance *>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x7057e0 — __ZN5boost6detail8function21function_obj_invoker1INS_3_bi6bind_tIbPFbPN3RBX8InstanceES7_ENS3_5list2INS3_5valueIS7_EENS_3argILi1EEEEEEEbS7_E6invokeERNS1_15function_bufferES7_
#[doc(alias = "boost::detail::function::function_obj_invoker1<boost::_bi::bind_t<bool,bool (*)(RBX::Instance *,RBX::Instance *),boost::_bi::list2<boost::_bi::value<RBX::Instance *>,boost::arg<1>>>,bool,RBX::Instance *>::invoke(boost::detail::function::function_buffer &,RBX::Instance *)")]
#[doc(alias = "__ZN5boost6detail8function21function_obj_invoker1INS_3_bi6bind_tIbPFbPN3RBX8InstanceES7_ENS3_5list2INS3_5valueIS7_EENS_3argILi1EEEEEEEbS7_E6invokeERNS1_15function_bufferES7_")]
pub fn stub_7057e0() -> ! {
    todo!("0x7057e0 boost::detail::function::function_obj_invoker1<boost::_bi::bind_t<bool,bool (*)(RBX::Instance *,RBX::Instance *),boost::_bi::list2<boost::_bi::value<RBX::Instance *>,boost::arg<1>>>,bool,RBX::Instance *>::invoke(boost::detail::function::function_buffer &,RBX::Instance *)")
}

// 0x7057f0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE4nextERNS2_13intrusive_ptrINS8_4slotEEE
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot> &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE4nextERNS2_13intrusive_ptrINS8_4slotEEE")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot> &)
pub fn stub_7057f0() -> ! {
    todo!("0x7057f0 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot> &)")
}

// 0x705950 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE8on_errorERSt9exception
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::on_error(std::exception &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE8on_errorERSt9exception")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::on_error(std::exception &)
pub fn stub_705950() -> ! {
    todo!("0x705950 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::on_error(std::exception &)")
}

// 0x705978 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEES7_EE4slotEEaSERKSB_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot> const&)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEES7_EE4slotEEaSERKSB_")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot> const&)
pub fn stub_705978() -> ! {
    todo!("0x705978 boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot> const&)")
}

// 0x7059a0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE24safe_static_do_get_mutexEv")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::safe_static_do_get_mutex(void)
pub fn stub_7059a0() -> ! {
    todo!("0x7059a0 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::safe_static_do_get_mutex(void)")
}

// 0x705a98 — __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEES6_ET_SC_SC_RKT0_St26random_access_iterator_tag
#[doc(alias = "__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> std::__find<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,rbx_core::SharedPtr<RBX::Instance>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,rbx_core::SharedPtr<RBX::Instance> const&,std::random_access_iterator_tag)")]
#[doc(alias = "__ZSt6__findIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEES6_ET_SC_SC_RKT0_St26random_access_iterator_tag")]
// was: __gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> std::__find<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::shared_ptr<RBX::Instance>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::shared_ptr<RBX::Instance> const&,std::random_access_iterator_tag)
pub fn stub_705a98() -> ! {
    todo!("0x705a98 __gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> std::__find<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::shared_ptr<RBX::Instance>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::shared_ptr<RBX::Instance> const&,std::random_access_iterator_tag)")
}

// 0x705b28 — __ZN5boost10shared_ptrISt6vectorINS0_IN3RBX8InstanceEEESaIS4_EEEaSERKS7_
#[doc(alias = "rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>::operator=(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrISt6vectorINS0_IN3RBX8InstanceEEESaIS4_EEEaSERKS7_")]
// was: boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>::operator=(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> const&)
pub fn stub_705b28() -> ! {
    todo!("0x705b28 boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>::operator=(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> const&)")
}

// 0x705b60 — __ZN5boost8weak_ptrIN3RBX8InstanceEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
#[doc(alias = "rbx_core::WeakPtr<RBX::Instance>::weak_ptr<RBX::Instance>(rbx_core::SharedPtr<RBX::Instance> const&,boost::detail::sp_enable_if_convertible<RBX::Instance,RBX::Instance>::type)")]
#[doc(alias = "__ZN5boost8weak_ptrIN3RBX8InstanceEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE")]
// was: boost::weak_ptr<RBX::Instance>::weak_ptr<RBX::Instance>(boost::shared_ptr<RBX::Instance> const&,boost::detail::sp_enable_if_convertible<RBX::Instance,RBX::Instance>::type)
pub fn stub_705b60() -> ! {
    todo!("0x705b60 boost::weak_ptr<RBX::Instance>::weak_ptr<RBX::Instance>(boost::shared_ptr<RBX::Instance> const&,boost::detail::sp_enable_if_convertible<RBX::Instance,RBX::Instance>::type)")
}

// 0x705bb0 — __ZNK3RBX15ServiceProvider4findINS_13ScriptServiceEEEPT_v
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::ScriptService * RBX::ServiceProvider::find<RBX::ScriptService>(void)const")]
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_13ScriptServiceEEEPT_v")]
pub fn stub_705bb0() -> ! {
    todo!("0x705bb0 RBX::ScriptService * RBX::ServiceProvider::find<RBX::ScriptService>(void)const")
}

// 0x705d28 — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEE15isNullClassNameEv
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEE15isNullClassNameEv")]
pub fn stub_705d28() -> ! {
    todo!("0x705d28 __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEE15isNullClassNameEv")
}

// 0x705dc8 — __ZN3RBX4Name7declareILZNS_14sScriptServiceEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sScriptServiceEEEERKS0_v")]
pub fn stub_705dc8() -> ! {
    todo!("0x705dc8 __ZN3RBX4Name7declareILZNS_14sScriptServiceEEEERKS0_v")
}

// 0x705e10 — __ZN3RBX4Name9doDeclareILZNS_14sScriptServiceEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sScriptServiceEEEERKS0_v")]
pub fn stub_705e10() -> ! {
    todo!("0x705e10 __ZN3RBX4Name9doDeclareILZNS_14sScriptServiceEEEERKS0_v")
}

// 0x705ef8 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13ScriptServiceEEEmv
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ScriptService>(void)")]
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_13ScriptServiceEEEmv")]
pub fn stub_705ef8() -> ! {
    todo!("0x705ef8 unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ScriptService>(void)")
}

// 0x705fd0 — __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEENS3_IKNS4_10Reflection13DescribedBaseEEEET_SH_SH_RKT0_St26random_access_iterator_tag
#[doc(alias = "__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> std::__find<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,rbx_core::SharedPtr<RBX::Reflection::DescribedBase const>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,rbx_core::SharedPtr<RBX::Reflection::DescribedBase const> const&,std::random_access_iterator_tag)")]
#[doc(alias = "__ZSt6__findIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEENS3_IKNS4_10Reflection13DescribedBaseEEEET_SH_SH_RKT0_St26random_access_iterator_tag")]
// was: __gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> std::__find<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::shared_ptr<RBX::Reflection::DescribedBase const>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::shared_ptr<RBX::Reflection::DescribedBase const> const&,std::random_access_iterator_tag)
pub fn stub_705fd0() -> ! {
    todo!("0x705fd0 __gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> std::__find<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::shared_ptr<RBX::Reflection::DescribedBase const>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::shared_ptr<RBX::Reflection::DescribedBase const> const&,std::random_access_iterator_tag)")
}

// 0x706094 — __ZN12XmlAttributeC2IN3RBX14InstanceHandleEEERKNS1_4NameET_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, XmlNameValuePair *, int, int, int, int)
#[doc(alias = "XmlAttribute::XmlAttribute<RBX::InstanceHandle>(RBX::Name const&,RBX::InstanceHandle)")]
#[doc(alias = "__ZN12XmlAttributeC2IN3RBX14InstanceHandleEEERKNS1_4NameET_")]
pub fn stub_706094() -> ! {
    todo!("0x706094 XmlAttribute::XmlAttribute<RBX::InstanceHandle>(RBX::Name const&,RBX::InstanceHandle)")
}

// 0x706198 — __ZN16XmlNameValuePairC2ERKN3RBX4NameENS0_14InstanceHandleE
#[doc(alias = "XmlNameValuePair::XmlNameValuePair(RBX::Name const&,RBX::InstanceHandle)")]
#[doc(alias = "__ZN16XmlNameValuePairC2ERKN3RBX4NameENS0_14InstanceHandleE")]
pub fn stub_706198() -> ! {
    todo!("0x706198 XmlNameValuePair::XmlNameValuePair(RBX::Name const&,RBX::InstanceHandle)")
}

// 0x706260 — __ZN5boost14singleton_poolI12XmlAttributeLj20ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
#[doc(alias = "boost::singleton_pool<XmlAttribute,20u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
#[doc(alias = "__ZN5boost14singleton_poolI12XmlAttributeLj20ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")]
pub fn stub_706260() -> ! {
    todo!("0x706260 boost::singleton_pool<XmlAttribute,20u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")
}

// 0x706298 — __ZN5boost6detail8function15functor_managerIPFbPN3RBX8InstanceEEE6manageERKNS1_15function_bufferERS9_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<bool (*)(RBX::Instance *)>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerIPFbPN3RBX8InstanceEEE6manageERKNS1_15function_bufferERS9_NS1_30functor_manager_operation_typeE")]
pub fn stub_706298() -> ! {
    todo!("0x706298 boost::detail::function::functor_manager<bool (*)(RBX::Instance *)>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x7062f8 — __ZN5boost20dynamic_pointer_castIN3RBX8InstanceENS1_6ObjectEEENS_10shared_ptrIT_EERKNS4_IT0_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance> boost::dynamic_pointer_cast<RBX::Instance,RBX::Object>(rbx_core::SharedPtr<RBX::Object> const&)")]
#[doc(alias = "__ZN5boost20dynamic_pointer_castIN3RBX8InstanceENS1_6ObjectEEENS_10shared_ptrIT_EERKNS4_IT0_EE")]
// was: boost::shared_ptr<RBX::Instance> boost::dynamic_pointer_cast<RBX::Instance,RBX::Object>(boost::shared_ptr<RBX::Object> const&)
pub fn stub_7062f8() -> ! {
    todo!("0x7062f8 boost::shared_ptr<RBX::Instance> boost::dynamic_pointer_cast<RBX::Instance,RBX::Object>(boost::shared_ptr<RBX::Object> const&)")
}

// 0x70633c — __ZN3RBX10Reflection9EventDescINS_8InstanceEFvPKNS0_18PropertyDescriptorEEN3rbx6signalIS6_EEMS2_S9_EC2ESA_PKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Instance,void ()(RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)> RBX::Instance::*>::EventDesc(rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)> RBX::Instance::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_8InstanceEFvPKNS0_18PropertyDescriptorEEN3rbx6signalIS6_EEMS2_S9_EC2ESA_PKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_70633c() -> ! {
    todo!("0x70633c RBX::Reflection::EventDesc<RBX::Instance,void ()(RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)> RBX::Instance::*>::EventDesc(rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)> RBX::Instance::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x7064c0 — __ZN3RBX10Reflection9EventDescINS_8InstanceEFvPKNS0_18PropertyDescriptorEEN3rbx6signalIS6_EEMS2_S9_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Instance,void ()(RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)> RBX::Instance::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_8InstanceEFvPKNS0_18PropertyDescriptorEEN3rbx6signalIS6_EEMS2_S9_ED0Ev")]
pub fn stub_7064c0() -> ! {
    todo!("0x7064c0 RBX::Reflection::EventDesc<RBX::Instance,void ()(RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)> RBX::Instance::*>::~EventDesc()")
}

// 0x706574 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_8InstanceEFvPKNS0_18PropertyDescriptorEEN3rbx6signalIS6_EEMS2_S9_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Instance,void ()(RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)> RBX::Instance::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_8InstanceEFvPKNS0_18PropertyDescriptorEEN3rbx6signalIS6_EEMS2_S9_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
// was: RBX::Reflection::EventDescImpl<1,RBX::Instance,void ()(RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)> RBX::Instance::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_706574() -> ! {
    todo!("0x706574 RBX::Reflection::EventDescImpl<1,RBX::Instance,void ()(RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)> RBX::Instance::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x7066c8 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_8InstanceEFvPKNS0_18PropertyDescriptorEEN3rbx6signalIS6_EEMS2_S9_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISF_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Instance,void ()(RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)> RBX::Instance::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_8InstanceEFvPKNS0_18PropertyDescriptorEEN3rbx6signalIS6_EEMS2_S9_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISF_EE")]
pub fn stub_7066c8() -> ! {
    todo!("0x7066c8 RBX::Reflection::EventDescImpl<1,RBX::Instance,void ()(RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)> RBX::Instance::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x706754 — __ZNK3RBX10Reflection13EventDescBaseINS_8InstanceEFvPKNS0_18PropertyDescriptorEEN3rbx6signalIS6_EEMS2_S9_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Instance,void ()(RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)> RBX::Instance::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_8InstanceEFvPKNS0_18PropertyDescriptorEEN3rbx6signalIS6_EEMS2_S9_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_706754() -> ! {
    todo!("0x706754 RBX::Reflection::EventDescBase<RBX::Instance,void ()(RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(RBX::Reflection::PropertyDescriptor const*)> RBX::Instance::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x706768 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::disconnectAll(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13disconnectAllEv")]
pub fn stub_706768() -> ! {
    todo!("0x706768 rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::disconnectAll(void)")
}

// 0x7068e0 — __ZN3rbx8any_castIRKPKN3RBX10Reflection18PropertyDescriptorENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::Reflection::PropertyDescriptor const* const& rbx::any_cast<RBX::Reflection::PropertyDescriptor const* const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRKPKN3RBX10Reflection18PropertyDescriptorENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_7068e0() -> ! {
    todo!("0x7068e0 RBX::Reflection::PropertyDescriptor const* const& rbx::any_cast<RBX::Reflection::PropertyDescriptor const* const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x7069d4 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKPKNS2_18PropertyDescriptorENS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISF_T0_T1_EENSD_9list_av_2IT2_T3_E4typeEEEMSI_FSF_SJ_ESM_SN_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::Reflection::PropertyDescriptor const* const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKPKNS2_18PropertyDescriptorENS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISF_T0_T1_EENSD_9list_av_2IT2_T3_E4typeEEEMSI_FSF_SJ_ESM_SN_")]
// was: boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list_av_2<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::Reflection::PropertyDescriptor const* const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)
pub fn stub_7069d4() -> ! {
    todo!("0x7069d4 boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list_av_2<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::Reflection::PropertyDescriptor const* const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)")
}

// 0x706af0 — __ZN3RBX10Reflection18GenericSlotWrapper8execute1IPKNS0_18PropertyDescriptorEEEvRKT_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute1<RBX::Reflection::PropertyDescriptor const*>(RBX::Reflection::PropertyDescriptor const* const&)")]
#[doc(alias = "__ZN3RBX10Reflection18GenericSlotWrapper8execute1IPKNS0_18PropertyDescriptorEEEvRKT_")]
pub fn stub_706af0() -> ! {
    todo!("0x706af0 void RBX::Reflection::GenericSlotWrapper::execute1<RBX::Reflection::PropertyDescriptor const*>(RBX::Reflection::PropertyDescriptor const* const&)")
}

// 0x706c38 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIPKNS1_10Reflection18PropertyDescriptorEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Reflection::PropertyDescriptor const*>(RBX::Reflection::PropertyDescriptor const* const&)")]
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSIPKNS1_10Reflection18PropertyDescriptorEEERS3_RKT_")]
pub fn stub_706c38() -> ! {
    todo!("0x706c38 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Reflection::PropertyDescriptor const*>(RBX::Reflection::PropertyDescriptor const* const&)")
}

// 0x706c88 — __ZN3rbx14implementation12typed_holderIPKN3RBX10Reflection18PropertyDescriptorEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::Reflection::PropertyDescriptor const*>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIPKN3RBX10Reflection18PropertyDescriptorEE9singletonEv")]
pub fn stub_706c88() -> ! {
    todo!("0x706c88 rbx::implementation::typed_holder<RBX::Reflection::PropertyDescriptor const*>::singleton(void)")
}

// 0x706cf4 — __ZN3rbx14implementation12typed_holderIPKN3RBX10Reflection18PropertyDescriptorEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::Reflection::PropertyDescriptor const*>::construct_func(char const*,char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIPKN3RBX10Reflection18PropertyDescriptorEE14construct_funcEPKcPc")]
pub fn stub_706cf4() -> ! {
    todo!("0x706cf4 rbx::implementation::typed_holder<RBX::Reflection::PropertyDescriptor const*>::construct_func(char const*,char *)")
}

// 0x706d00 — __ZN3rbx14implementation12typed_holderIPKN3RBX10Reflection18PropertyDescriptorEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::Reflection::PropertyDescriptor const*>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIPKN3RBX10Reflection18PropertyDescriptorEE13destruct_funcEPc")]
pub fn stub_706d00() -> ! {
    todo!("0x706d00 rbx::implementation::typed_holder<RBX::Reflection::PropertyDescriptor const*>::destruct_func(char *)")
}

// 0x706d04 — __ZN5boost8functionIFvPKN3RBX10Reflection18PropertyDescriptorEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_18GenericSlotWrapperERKS5_EENS9_5list2INS9_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvPKN3RBX10Reflection18PropertyDescriptorEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_18GenericSlotWrapperERKS5_EENS9_5list2INS9_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_706d04() -> ! {
    todo!("0x706d04 __ZN5boost8functionIFvPKN3RBX10Reflection18PropertyDescriptorEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_18GenericSlotWrapperERKS5_EENS9_5list2INS9_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")
}

// 0x706de8 — __ZN5boost9function1IvPKN3RBX10Reflection18PropertyDescriptorEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_18GenericSlotWrapperERKS5_EENS8_5list2INS8_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function1IvPKN3RBX10Reflection18PropertyDescriptorEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_18GenericSlotWrapperERKS5_EENS8_5list2INS8_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
pub fn stub_706de8() -> ! {
    todo!("0x706de8 __ZN5boost9function1IvPKN3RBX10Reflection18PropertyDescriptorEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_18GenericSlotWrapperERKS5_EENS8_5list2INS8_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")
}

// 0x706ed0 — __ZN5boost9function1IvPKN3RBX10Reflection18PropertyDescriptorEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_18GenericSlotWrapperERKS5_EENS8_5list2INS8_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function1<void,RBX::Reflection::PropertyDescriptor const*>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)")]
#[doc(alias = "__ZN5boost9function1IvPKN3RBX10Reflection18PropertyDescriptorEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_18GenericSlotWrapperERKS5_EENS8_5list2INS8_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEvT_")]
// was: void boost::function1<void,RBX::Reflection::PropertyDescriptor const*>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)
pub fn stub_706ed0() -> ! {
    todo!("0x706ed0 void boost::function1<void,RBX::Reflection::PropertyDescriptor const*>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)")
}

// 0x706fc8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKPKNS8_18PropertyDescriptorEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKPKNS8_18PropertyDescriptorEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_706fc8() -> ! {
    todo!("0x706fc8 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x706fe4 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKPKNS8_18PropertyDescriptorEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEvSC_E6invokeERNS1_15function_bufferESC_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,void,RBX::Reflection::PropertyDescriptor const*>::invoke(boost::detail::function::function_buffer &,RBX::Reflection::PropertyDescriptor const*)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKPKNS8_18PropertyDescriptorEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEvSC_E6invokeERNS1_15function_bufferESC_")]
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,void,RBX::Reflection::PropertyDescriptor const*>::invoke(boost::detail::function::function_buffer &,RBX::Reflection::PropertyDescriptor const*)
pub fn stub_706fe4() -> ! {
    todo!("0x706fe4 boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,void,RBX::Reflection::PropertyDescriptor const*>::invoke(boost::detail::function::function_buffer &,RBX::Reflection::PropertyDescriptor const*)")
}

// 0x706ff8 — __ZNK5boost6detail8function13basic_vtable1IvPKN3RBX10Reflection18PropertyDescriptorEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS4_18GenericSlotWrapperERKS7_EENSA_5list2INSA_5valueINS_10shared_ptrISE_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::Reflection::PropertyDescriptor const*>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPKN3RBX10Reflection18PropertyDescriptorEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS4_18GenericSlotWrapperERKS7_EENSA_5list2INSA_5valueINS_10shared_ptrISE_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::Reflection::PropertyDescriptor const*>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const
pub fn stub_706ff8() -> ! {
    todo!("0x706ff8 bool boost::detail::function::basic_vtable1<void,RBX::Reflection::PropertyDescriptor const*>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")
}

// 0x7070e0 — __ZNK5boost6detail8function13basic_vtable1IvPKN3RBX10Reflection18PropertyDescriptorEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS4_18GenericSlotWrapperERKS7_EENSA_5list2INSA_5valueINS_10shared_ptrISE_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::Reflection::PropertyDescriptor const*>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPKN3RBX10Reflection18PropertyDescriptorEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS4_18GenericSlotWrapperERKS7_EENSA_5list2INSA_5valueINS_10shared_ptrISE_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::Reflection::PropertyDescriptor const*>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_7070e0() -> ! {
    todo!("0x7070e0 bool boost::detail::function::basic_vtable1<void,RBX::Reflection::PropertyDescriptor const*>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x7071c4 — __ZNK5boost6detail8function13basic_vtable1IvPKN3RBX10Reflection18PropertyDescriptorEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS4_18GenericSlotWrapperERKS7_EENSA_5list2INSA_5valueINS_10shared_ptrISE_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::Reflection::PropertyDescriptor const*>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPKN3RBX10Reflection18PropertyDescriptorEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS4_18GenericSlotWrapperERKS7_EENSA_5list2INSA_5valueINS_10shared_ptrISE_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
// was: void boost::detail::function::basic_vtable1<void,RBX::Reflection::PropertyDescriptor const*>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_7071c4() -> ! {
    todo!("0x7071c4 void boost::detail::function::basic_vtable1<void,RBX::Reflection::PropertyDescriptor const*>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x707298 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKPKNS5_18PropertyDescriptorEEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIS9_EEvRT_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>::operator()<RBX::Reflection::PropertyDescriptor const*>(RBX::Reflection::PropertyDescriptor const* &)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKPKNS5_18PropertyDescriptorEEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIS9_EEvRT_")]
// was: void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>::operator()<RBX::Reflection::PropertyDescriptor const*>(RBX::Reflection::PropertyDescriptor const* &)
pub fn stub_707298() -> ! {
    todo!("0x707298 void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>::operator()<RBX::Reflection::PropertyDescriptor const*>(RBX::Reflection::PropertyDescriptor const* &)")
}

// 0x7072b0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKPKNS8_18PropertyDescriptorEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKPKNS8_18PropertyDescriptorEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_7072b0() -> ! {
    todo!("0x7072b0 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x707408 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE7connectIN5boost8functionIS7_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::connect<boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>>(boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE7connectIN5boost8functionIS7_EEEENS0_10connectionERKT_")]
pub fn stub_707408() -> ! {
    todo!("0x707408 rbx::signals::connection rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::connect<boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>>(boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)> const&)")
}

// 0x707500 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEEaSEPSB_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEEaSEPSB_")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot*)
pub fn stub_707500() -> ! {
    todo!("0x707500 boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot*)")
}

// 0x707528 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost8functionIS7_EEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost8functionIS7_EEED1Ev")]
pub fn stub_707528() -> ! {
    todo!("0x707528 rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>>::~callable_slot()")
}

// 0x707638 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost8functionIS7_EEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost8functionIS7_EEED0Ev")]
pub fn stub_707638() -> ! {
    todo!("0x707638 rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>>::~callable_slot()")
}

// 0x707768 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot::disconnect(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slot10disconnectEv")]
pub fn stub_707768() -> ! {
    todo!("0x707768 rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot::disconnect(void)")
}

// 0x707878 — __ZNK3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot::connected(void)const")]
#[doc(alias = "__ZNK3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slot9connectedEv")]
pub fn stub_707878() -> ! {
    todo!("0x707878 rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot::connected(void)const")
}

// 0x707888 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost8functionIS8_EELi1ES8_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost8functionIS8_EELi1ES8_ED1Ev")]
pub fn stub_707888() -> ! {
    todo!("0x707888 rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")
}

// 0x707998 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost8functionIS8_EELi1ES8_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost8functionIS8_EELi1ES8_ED0Ev")]
pub fn stub_707998() -> ! {
    todo!("0x707998 rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(RBX::Reflection::PropertyDescriptor const*)>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")
}
