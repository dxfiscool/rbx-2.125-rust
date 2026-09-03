//! datamodel — generated_watchdog_datamodel_w12b — 120 stubs (watchdog w12b datamodel2)
//! Source: ida/export.json (85545 funcs) EA-sorted asc, strict RBX:: datamodel filter (datamodel2)
//! Filter: RBX:: + Instance|DataModel|Workspace|Part|Joint|Keyframe|Lighting|Selection|Gui|Controller|etc, SKIP /tmp/global_eas.txt, UNIQUE vs datamodel stubs
//! Each stub preserves IDA ea + mangled + demangled for rg. Uses rbx_core::SharedPtr not boost::shared_ptr.
//! Range: 0xf46b64..0xf4d244 | watchdog w12b datamodel2 (datamodel, not core gap)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xf46b64 — j___ZN3rbx7signals6signalIFvRKN3RBX16SelectionChangedEEE4slot24safe_static_do_get_mutexEv
// type: int __fastcall(int, int, int, int, int)
// rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::slot::safe_static_do_get_mutex(void)
#[doc(alias = "rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvRKN3RBX16SelectionChangedEEE4slot24safe_static_do_get_mutexEv")]
pub fn stub_0xf46b64() -> ! {
    todo!("0xf46b64 rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::slot::safe_static_do_get_mutex(void)")
}

// 0xf46b74 — j___ZN3rbx7signals6signalIFvRKN3RBX16SelectionChangedEEE6insertEPNS7_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
// rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::insert(rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::slot *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::insert(rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvRKN3RBX16SelectionChangedEEE6insertEPNS7_4slotE")]
pub fn stub_0xf46b74() -> ! {
    todo!("0xf46b74 rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::insert(rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::slot *)")
}

// 0xf46b84 — j___ZN3rbx7signals6signalIFvRKN3RBX16SelectionChangedEEE6removeEPNS7_4slotE
// type: int __fastcall(int, char *)
// rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::remove(rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::slot *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::remove(rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvRKN3RBX16SelectionChangedEEE6removeEPNS7_4slotE")]
pub fn stub_0xf46b84() -> ! {
    todo!("0xf46b84 rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::remove(rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::slot *)")
}

// 0xf46b94 — j___ZN3rbx7signals6signalIFvRKN3RBX16SelectionChangedEEE7connectIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_9SelectionES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int()
// rbx::signals::connection rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Selection,RBX::SelectionChanged const&>,boost::_bi::list2<boost::_bi::value<RBX::Selection*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Selection,RBX::SelectionChanged const&>,boost::_bi::list2<boost::_bi::value<RBX::Selection*>,boost::arg<1>>> const&)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Selection,RBX::SelectionChanged const&>,boost::_bi::list2<boost::_bi::value<RBX::Selection*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Selection,RBX::SelectionChanged const&>,boost::_bi::list2<boost::_bi::value<RBX::Selection*>,boost::arg<1>>> const&)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvRKN3RBX16SelectionChangedEEE7connectIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_9SelectionES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEEENS0_10connectionERKT_")]
pub fn stub_0xf46b94() -> ! {
    todo!("0xf46b94 rbx::signals::connection rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Selection,RBX::SelectionChanged const&>,boost::_bi::list2<boost::_bi::value<RBX::Selection*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Selection,RBX::SelectionChanged const&>,boost::_bi::list2<boost::_bi::value<RBX::Selection*>,boost::arg<1>>> const&)")
}

// 0xf46ba4 — j___ZN3rbx7signals6signalIFvRKN3RBX16SelectionChangedEEE8on_errorERSt9exception
// type: int()
// rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::on_error(std::exception &)
#[doc(alias = "rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::on_error(std::exception &)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvRKN3RBX16SelectionChangedEEE8on_errorERSt9exception")]
pub fn stub_0xf46ba4() -> ! {
    todo!("0xf46ba4 rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::on_error(std::exception &)")
}

// 0xf46bb4 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX16SelectionChangedEEE4slotEEaSEPSA_
// type: int()
// rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::slot*)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::slot*)")]
#[doc(alias = "j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX16SelectionChangedEEE4slotEEaSEPSA_")]
pub fn stub_0xf46bb4() -> ! {
    todo!("0xf46bb4 rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::slot*)")
}

// 0xf46bc4 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX16SelectionChangedEEE4slotEEaSERKSB_
// type: int()
// rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::slot> const&)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::slot> const&)")]
#[doc(alias = "j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX16SelectionChangedEEE4slotEEaSERKSB_")]
pub fn stub_0xf46bc4() -> ! {
    todo!("0xf46bc4 rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::slot> const&)")
}

// 0xf46be4 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX9SelectionERKNS4_16SelectionChangedEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_
// type: int()
// void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Selection,RBX::SelectionChanged const&>,boost::_bi::list2<boost::_bi::value<RBX::Selection*>,boost::arg<1>>>::operator()<RBX::SelectionChanged>(RBX::SelectionChanged const&)
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Selection,RBX::SelectionChanged const&>,boost::_bi::list2<boost::_bi::value<RBX::Selection*>,boost::arg<1>>>::operator()<RBX::SelectionChanged>(RBX::SelectionChanged const&)")]
#[doc(alias = "j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX9SelectionERKNS4_16SelectionChangedEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_")]
pub fn stub_0xf46be4() -> ! {
    todo!("0xf46be4 void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Selection,RBX::SelectionChanged const&>,boost::_bi::list2<boost::_bi::value<RBX::Selection*>,boost::arg<1>>>::operator()<RBX::SelectionChanged>(RBX::SelectionChanged const&)")
}

// 0xf46bf4 — j___ZNSt12_Vector_baseIPN3RBX14ISelectionBaseESaIS2_EE11_M_allocateEm
// type: int()
// std::_Vector_base<RBX::ISelectionBase *,std::allocator<RBX::ISelectionBase *>>::_M_allocate(unsigned long)
#[doc(alias = "std::_Vector_base<RBX::ISelectionBase *,std::allocator<RBX::ISelectionBase *>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIPN3RBX14ISelectionBaseESaIS2_EE11_M_allocateEm")]
pub fn stub_0xf46bf4() -> ! {
    todo!("0xf46bf4 std::_Vector_base<RBX::ISelectionBase *,std::allocator<RBX::ISelectionBase *>>::_M_allocate(unsigned long)")
}

// 0xf46c14 — j___ZNSt6vectorIPN3RBX14ISelectionBaseESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, void *__src)
// std::vector<RBX::ISelectionBase *,std::allocator<RBX::ISelectionBase *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::ISelectionBase **,std::vector<RBX::ISelectionBase *,std::allocator<RBX::ISelectionBase *>>>,RBX::ISelectionBase * const&)
#[doc(alias = "std::vector<RBX::ISelectionBase *,std::allocator<RBX::ISelectionBase *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::ISelectionBase **,std::vector<RBX::ISelectionBase *,std::allocator<RBX::ISelectionBase *>>>,RBX::ISelectionBase * const&)")]
#[doc(alias = "j___ZNSt6vectorIPN3RBX14ISelectionBaseESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_0xf46c14() -> ! {
    todo!("0xf46c14 std::vector<RBX::ISelectionBase *,std::allocator<RBX::ISelectionBase *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::ISelectionBase **,std::vector<RBX::ISelectionBase *,std::allocator<RBX::ISelectionBase *>>>,RBX::ISelectionBase * const&)")
}

// 0xf46c24 — j___ZNSt6vectorIPN3RBX14ISelectionBaseESaIS2_EE9push_backERKS2_
// type: int()
// std::vector<RBX::ISelectionBase *,std::allocator<RBX::ISelectionBase *>>::push_back(RBX::ISelectionBase * const&)
#[doc(alias = "std::vector<RBX::ISelectionBase *,std::allocator<RBX::ISelectionBase *>>::push_back(RBX::ISelectionBase * const&)")]
#[doc(alias = "j___ZNSt6vectorIPN3RBX14ISelectionBaseESaIS2_EE9push_backERKS2_")]
pub fn stub_0xf46c24() -> ! {
    todo!("0xf46c24 std::vector<RBX::ISelectionBase *,std::allocator<RBX::ISelectionBase *>>::push_back(RBX::ISelectionBase * const&)")
}

// 0xf46c44 — j___ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX14ISelectionBaseESt6vectorIS4_SaIS4_EEEES4_ET_SA_SA_RKT0_St26random_access_iterator_tag
// type: int()
// __gnu_cxx::__normal_iterator<RBX::ISelectionBase **,std::vector<RBX::ISelectionBase *,std::allocator<RBX::ISelectionBase *>>> std::__find<__gnu_cxx::__normal_iterator<RBX::ISelectionBase **,std::vector<RBX::ISelectionBase *,std::allocator<RBX::ISelectionBase *>>>,RBX::ISelectionBase *>(__gnu_cxx::__normal_iterator<RBX::ISelectionBase **,std::vector<RBX::ISelectionBase *,std::allocator<RBX::ISelectionBase *>>>,__gnu_cxx::__normal_iterator<RBX::ISelectionBase **,std::vector<RBX::ISelectionBase *,std::allocator<RBX::ISelectionBase *>>>,RBX::ISelectionBase * const&,std::random_access_iterator_tag)
#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::ISelectionBase **,std::vector<RBX::ISelectionBase *,std::allocator<RBX::ISelectionBase *>>> std::__find<__gnu_cxx::__normal_iterator<RBX::ISelectionBase **,std::vector<RBX::ISelectionBase *,std::allocator<RBX::ISelectionBase *>>>,RBX::ISelectionBase *>(__gnu_cxx::__normal_iterator<RBX::ISelectionBase **,std::vector<RBX::ISelectionBase *,std::allocator<RBX::ISelectionBase *>>>,__gnu_cxx::__normal_iterator<RBX::ISelectionBase **,std::vector<RBX::ISelectionBase *,std::allocator<RBX::ISelectionBase *>>>,RBX::ISelectionBase * const&,std::random_access_iterator_tag)")]
#[doc(alias = "j___ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX14ISelectionBaseESt6vectorIS4_SaIS4_EEEES4_ET_SA_SA_RKT0_St26random_access_iterator_tag")]
pub fn stub_0xf46c44() -> ! {
    todo!("0xf46c44 __gnu_cxx::__normal_iterator<RBX::ISelectionBase **,std::vector<RBX::ISelectionBase *,std::allocator<RBX::ISelectionBase *>>> std::__find<__gnu_cxx::__normal_iterator<RBX::ISelectionBase **,std::vector<RBX::ISelectionBase *,std::allocator<RBX::ISelectionBase *>>>,RBX::ISelectionBase *>(__gnu_cxx::__normal_iterator<RBX::ISelectionBase **,std::vector<RBX::ISelectionBase *,std::allocator<RBX::ISelectionBase *>>>,__gnu_cxx::__normal_iterator<RBX::ISelectionBase **,std::vector<RBX::ISelectionBase *,std::allocator<RBX::ISelectionBase *>>>,RBX::ISelectionBase * const&,std::random_access_iterator_tag)")
}

// 0xf46c64 — j___ZN3RBX12SelectionBoxD1Ev
// type: void __fastcall(RBX::SelectionBox *__hidden this)
// RBX::SelectionBox::~SelectionBox()
#[doc(alias = "RBX::SelectionBox::~SelectionBox()")]
#[doc(alias = "j___ZN3RBX12SelectionBoxD1Ev")]
pub fn stub_0xf46c64() -> ! {
    todo!("0xf46c64 RBX::SelectionBox::~SelectionBox()")
}

// 0xf46db4 — j___ZN3RBX14SelectionLassoD1Ev
// type: void __fastcall(RBX::SelectionLasso *__hidden this)
// RBX::SelectionLasso::~SelectionLasso()
#[doc(alias = "RBX::SelectionLasso::~SelectionLasso()")]
#[doc(alias = "j___ZN3RBX14SelectionLassoD1Ev")]
pub fn stub_0xf46db4() -> ! {
    todo!("0xf46db4 RBX::SelectionLasso::~SelectionLasso()")
}

// 0xf46dd4 — j___ZN3RBX19SelectionPointLassoD1Ev
// type: void __fastcall(RBX::SelectionPointLasso *__hidden this)
// RBX::SelectionPointLasso::~SelectionPointLasso()
#[doc(alias = "RBX::SelectionPointLasso::~SelectionPointLasso()")]
#[doc(alias = "j___ZN3RBX19SelectionPointLassoD1Ev")]
pub fn stub_0xf46dd4() -> ! {
    todo!("0xf46dd4 RBX::SelectionPointLasso::~SelectionPointLasso()")
}

// 0xf46f44 — j___ZN3RBX20SkateboardControllerD1Ev
// type: void __fastcall(RBX::SkateboardController *__hidden this)
// RBX::SkateboardController::~SkateboardController()
#[doc(alias = "RBX::SkateboardController::~SkateboardController()")]
#[doc(alias = "j___ZN3RBX20SkateboardControllerD1Ev")]
pub fn stub_0xf46f44() -> ! {
    todo!("0xf46f44 RBX::SkateboardController::~SkateboardController()")
}

// 0xf470d4 — j___ZN3RBX11shared_fromINS_20SkateboardControllerEEEN5boost10shared_ptrIT_EEPS4_
// type: void __fastcall(_QWORD *, int)
// was: boost::shared_ptr<RBX::SkateboardController> RBX::shared_from<RBX::SkateboardController>(RBX::SkateboardController*)
#[doc(alias = "rbx_core::SharedPtr<RBX::SkateboardController> RBX::shared_from<RBX::SkateboardController>(RBX::SkateboardController*)")]
#[doc(alias = "j___ZN3RBX11shared_fromINS_20SkateboardControllerEEEN5boost10shared_ptrIT_EEPS4_")]
pub fn stub_0xf470d4() -> ! {
    todo!("0xf470d4 rbx_core::SharedPtr<RBX::SkateboardController> RBX::shared_from<RBX::SkateboardController>(RBX::SkateboardController*)")
}

// 0xf472f4 — j___ZN5boost10shared_ptrIN3RBX20SkateboardControllerEEaSERKS3_
// type: int()
// was: boost::shared_ptr<RBX::SkateboardController>::operator=(boost::shared_ptr<RBX::SkateboardController> const&)
#[doc(alias = "rbx_core::SharedPtr<RBX::SkateboardController>::operator=(rbx_core::SharedPtr<RBX::SkateboardController> const&)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX20SkateboardControllerEEaSERKS3_")]
pub fn stub_0xf472f4() -> ! {
    todo!("0xf472f4 rbx_core::SharedPtr<RBX::SkateboardController>::operator=(rbx_core::SharedPtr<RBX::SkateboardController> const&)")
}

// 0xf47914 — j___ZN3RBX8SparklesD0Ev
// type: void __fastcall(RBX::Sparkles *__hidden this)
// RBX::Sparkles::~Sparkles()
#[doc(alias = "RBX::Sparkles::~Sparkles()")]
#[doc(alias = "j___ZN3RBX8SparklesD0Ev")]
pub fn stub_0xf47914() -> ! {
    todo!("0xf47914 RBX::Sparkles::~Sparkles()")
}

// 0xf47924 — j___ZN3RBX8SparklesD2Ev
// type: void __fastcall(RBX::Sparkles *__hidden this)
// RBX::Sparkles::~Sparkles()
#[doc(alias = "RBX::Sparkles::~Sparkles()")]
#[doc(alias = "j___ZN3RBX8SparklesD2Ev")]
pub fn stub_0xf47924() -> ! {
    todo!("0xf47924 RBX::Sparkles::~Sparkles()")
}

// 0xf47ac4 — j___ZNSt12_Vector_baseIPN3RBX13SpawnLocationESaIS2_EE11_M_allocateEm
// type: int()
// std::_Vector_base<RBX::SpawnLocation *,std::allocator<RBX::SpawnLocation *>>::_M_allocate(unsigned long)
#[doc(alias = "std::_Vector_base<RBX::SpawnLocation *,std::allocator<RBX::SpawnLocation *>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIPN3RBX13SpawnLocationESaIS2_EE11_M_allocateEm")]
pub fn stub_0xf47ac4() -> ! {
    todo!("0xf47ac4 std::_Vector_base<RBX::SpawnLocation *,std::allocator<RBX::SpawnLocation *>>::_M_allocate(unsigned long)")
}

// 0xf47ad4 — j___ZNSt4listIPN3RBX13SpawnLocationESaIS2_EE6removeERKS2_
// type: int()
// std::list<RBX::SpawnLocation *,std::allocator<RBX::SpawnLocation *>>::remove(RBX::SpawnLocation * const&)
#[doc(alias = "std::list<RBX::SpawnLocation *,std::allocator<RBX::SpawnLocation *>>::remove(RBX::SpawnLocation * const&)")]
#[doc(alias = "j___ZNSt4listIPN3RBX13SpawnLocationESaIS2_EE6removeERKS2_")]
pub fn stub_0xf47ad4() -> ! {
    todo!("0xf47ad4 std::list<RBX::SpawnLocation *,std::allocator<RBX::SpawnLocation *>>::remove(RBX::SpawnLocation * const&)")
}

// 0xf47ae4 — j___ZNSt6vectorIPN3RBX13SpawnLocationESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, void *__src)
// std::vector<RBX::SpawnLocation *,std::allocator<RBX::SpawnLocation *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SpawnLocation **,std::vector<RBX::SpawnLocation *,std::allocator<RBX::SpawnLocation *>>>,RBX::SpawnLocation * const&)
#[doc(alias = "std::vector<RBX::SpawnLocation *,std::allocator<RBX::SpawnLocation *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SpawnLocation **,std::vector<RBX::SpawnLocation *,std::allocator<RBX::SpawnLocation *>>>,RBX::SpawnLocation * const&)")]
#[doc(alias = "j___ZNSt6vectorIPN3RBX13SpawnLocationESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_0xf47ae4() -> ! {
    todo!("0xf47ae4 std::vector<RBX::SpawnLocation *,std::allocator<RBX::SpawnLocation *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SpawnLocation **,std::vector<RBX::SpawnLocation *,std::allocator<RBX::SpawnLocation *>>>,RBX::SpawnLocation * const&)")
}

// 0xf47af4 — j___ZNSt6vectorIPN3RBX13SpawnLocationESaIS2_EE9push_backERKS2_
// type: int()
// std::vector<RBX::SpawnLocation *,std::allocator<RBX::SpawnLocation *>>::push_back(RBX::SpawnLocation * const&)
#[doc(alias = "std::vector<RBX::SpawnLocation *,std::allocator<RBX::SpawnLocation *>>::push_back(RBX::SpawnLocation * const&)")]
#[doc(alias = "j___ZNSt6vectorIPN3RBX13SpawnLocationESaIS2_EE9push_backERKS2_")]
pub fn stub_0xf47af4() -> ! {
    todo!("0xf47af4 std::vector<RBX::SpawnLocation *,std::allocator<RBX::SpawnLocation *>>::push_back(RBX::SpawnLocation * const&)")
}

// 0xf47c74 — j___ZNSt12_Vector_baseIN3RBX12SpecialShape8MeshTypeESaIS2_EE11_M_allocateEm
// type: int()
// std::_Vector_base<RBX::SpecialShape::MeshType,std::allocator<RBX::SpecialShape::MeshType>>::_M_allocate(unsigned long)
#[doc(alias = "std::_Vector_base<RBX::SpecialShape::MeshType,std::allocator<RBX::SpecialShape::MeshType>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIN3RBX12SpecialShape8MeshTypeESaIS2_EE11_M_allocateEm")]
pub fn stub_0xf47c74() -> ! {
    todo!("0xf47c74 std::_Vector_base<RBX::SpecialShape::MeshType,std::allocator<RBX::SpecialShape::MeshType>>::_M_allocate(unsigned long)")
}

// 0xf47c84 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX12SpecialShape8MeshTypeES6_EET0_T_S8_S7_
// type: int()
// RBX::SpecialShape::MeshType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::SpecialShape::MeshType *,RBX::SpecialShape::MeshType *>(RBX::SpecialShape::MeshType *,RBX::SpecialShape::MeshType *,RBX::SpecialShape::MeshType *)
#[doc(alias = "RBX::SpecialShape::MeshType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::SpecialShape::MeshType *,RBX::SpecialShape::MeshType *>(RBX::SpecialShape::MeshType *,RBX::SpecialShape::MeshType *,RBX::SpecialShape::MeshType *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX12SpecialShape8MeshTypeES6_EET0_T_S8_S7_")]
pub fn stub_0xf47c84() -> ! {
    todo!("0xf47c84 RBX::SpecialShape::MeshType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::SpecialShape::MeshType *,RBX::SpecialShape::MeshType *>(RBX::SpecialShape::MeshType *,RBX::SpecialShape::MeshType *,RBX::SpecialShape::MeshType *)")
}

// 0xf47c94 — j___ZNSt3mapIPKN3RBX4NameENS0_12SpecialShape8MeshTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int()
// std::map<RBX::Name const*,RBX::SpecialShape::MeshType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>>::operator[](RBX::Name const* const&)
#[doc(alias = "std::map<RBX::Name const*,RBX::SpecialShape::MeshType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "j___ZNSt3mapIPKN3RBX4NameENS0_12SpecialShape8MeshTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
pub fn stub_0xf47c94() -> ! {
    todo!("0xf47c94 std::map<RBX::Name const*,RBX::SpecialShape::MeshType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>>::operator[](RBX::Name const* const&)")
}

// 0xf47ca4 — j___ZNSt6vectorIN3RBX12SpecialShape8MeshTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int()
// std::vector<RBX::SpecialShape::MeshType,std::allocator<RBX::SpecialShape::MeshType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SpecialShape::MeshType*,std::vector<RBX::SpecialShape::MeshType,std::allocator<RBX::SpecialShape::MeshType>>>,RBX::SpecialShape::MeshType const&)
#[doc(alias = "std::vector<RBX::SpecialShape::MeshType,std::allocator<RBX::SpecialShape::MeshType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SpecialShape::MeshType*,std::vector<RBX::SpecialShape::MeshType,std::allocator<RBX::SpecialShape::MeshType>>>,RBX::SpecialShape::MeshType const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX12SpecialShape8MeshTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_0xf47ca4() -> ! {
    todo!("0xf47ca4 std::vector<RBX::SpecialShape::MeshType,std::allocator<RBX::SpecialShape::MeshType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SpecialShape::MeshType*,std::vector<RBX::SpecialShape::MeshType,std::allocator<RBX::SpecialShape::MeshType>>>,RBX::SpecialShape::MeshType const&)")
}

// 0xf47cb4 — j___ZNSt6vectorIN3RBX12SpecialShape8MeshTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int()
// std::vector<RBX::SpecialShape::MeshType,std::allocator<RBX::SpecialShape::MeshType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SpecialShape::MeshType*,std::vector<RBX::SpecialShape::MeshType,std::allocator<RBX::SpecialShape::MeshType>>>,unsigned long,RBX::SpecialShape::MeshType const&)
#[doc(alias = "std::vector<RBX::SpecialShape::MeshType,std::allocator<RBX::SpecialShape::MeshType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SpecialShape::MeshType*,std::vector<RBX::SpecialShape::MeshType,std::allocator<RBX::SpecialShape::MeshType>>>,unsigned long,RBX::SpecialShape::MeshType const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX12SpecialShape8MeshTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_0xf47cb4() -> ! {
    todo!("0xf47cb4 std::vector<RBX::SpecialShape::MeshType,std::allocator<RBX::SpecialShape::MeshType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SpecialShape::MeshType*,std::vector<RBX::SpecialShape::MeshType,std::allocator<RBX::SpecialShape::MeshType>>>,unsigned long,RBX::SpecialShape::MeshType const&)")
}

// 0xf47cc4 — j___ZNSt6vectorIN3RBX12SpecialShape8MeshTypeESaIS2_EE6resizeEmS2_
// type: int()
// std::vector<RBX::SpecialShape::MeshType,std::allocator<RBX::SpecialShape::MeshType>>::resize(unsigned long,RBX::SpecialShape::MeshType)
#[doc(alias = "std::vector<RBX::SpecialShape::MeshType,std::allocator<RBX::SpecialShape::MeshType>>::resize(unsigned long,RBX::SpecialShape::MeshType)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX12SpecialShape8MeshTypeESaIS2_EE6resizeEmS2_")]
pub fn stub_0xf47cc4() -> ! {
    todo!("0xf47cc4 std::vector<RBX::SpecialShape::MeshType,std::allocator<RBX::SpecialShape::MeshType>>::resize(unsigned long,RBX::SpecialShape::MeshType)")
}

// 0xf47cd4 — j___ZNSt6vectorIN3RBX12SpecialShape8MeshTypeESaIS2_EE9push_backERKS2_
// type: int()
// std::vector<RBX::SpecialShape::MeshType,std::allocator<RBX::SpecialShape::MeshType>>::push_back(RBX::SpecialShape::MeshType const&)
#[doc(alias = "std::vector<RBX::SpecialShape::MeshType,std::allocator<RBX::SpecialShape::MeshType>>::push_back(RBX::SpecialShape::MeshType const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX12SpecialShape8MeshTypeESaIS2_EE9push_backERKS2_")]
pub fn stub_0xf47cd4() -> ! {
    todo!("0xf47cd4 std::vector<RBX::SpecialShape::MeshType,std::allocator<RBX::SpecialShape::MeshType>>::push_back(RBX::SpecialShape::MeshType const&)")
}

// 0xf47ce4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12SpecialShape8MeshTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int()
// std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType> const&)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12SpecialShape8MeshTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
pub fn stub_0xf47ce4() -> ! {
    todo!("0xf47ce4 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType> const&)")
}

// 0xf47cf4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12SpecialShape8MeshTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
// std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>,std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType> const&)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>,std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12SpecialShape8MeshTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
pub fn stub_0xf47cf4() -> ! {
    todo!("0xf47cf4 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>,std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType> const&)")
}

// 0xf47d04 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12SpecialShape8MeshTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int()
// std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType> const&)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12SpecialShape8MeshTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
pub fn stub_0xf47d04() -> ! {
    todo!("0xf47d04 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType> const&)")
}

// 0xf48664 — j___ZN3rbx8any_castIN3RBX16LegacyController9InputTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: int()
// RBX::LegacyController::InputType * rbx::any_cast<RBX::LegacyController::InputType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)
#[doc(alias = "RBX::LegacyController::InputType * rbx::any_cast<RBX::LegacyController::InputType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "j___ZN3rbx8any_castIN3RBX16LegacyController9InputTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
pub fn stub_0xf48664() -> ! {
    todo!("0xf48664 RBX::LegacyController::InputType * rbx::any_cast<RBX::LegacyController::InputType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")
}

// 0xf486e4 — j___ZN3RBX16SurfaceSelectionD1Ev
// type: void __fastcall(RBX::SurfaceSelection *__hidden this)
// RBX::SurfaceSelection::~SurfaceSelection()
#[doc(alias = "RBX::SurfaceSelection::~SurfaceSelection()")]
#[doc(alias = "j___ZN3RBX16SurfaceSelectionD1Ev")]
pub fn stub_0xf486e4() -> ! {
    todo!("0xf486e4 RBX::SurfaceSelection::~SurfaceSelection()")
}

// 0xf48824 — j___ZN3RBX11shared_fromINS_4TeamEEEN5boost10shared_ptrIT_EEPS4_
// type: int()
// was: boost::shared_ptr<RBX::Team> RBX::shared_from<RBX::Team>(RBX::Team*)
#[doc(alias = "rbx_core::SharedPtr<RBX::Team> RBX::shared_from<RBX::Team>(RBX::Team*)")]
#[doc(alias = "j___ZN3RBX11shared_fromINS_4TeamEEEN5boost10shared_ptrIT_EEPS4_")]
pub fn stub_0xf48824() -> ! {
    todo!("0xf48824 rbx_core::SharedPtr<RBX::Team> RBX::shared_from<RBX::Team>(RBX::Team*)")
}

// 0xf48ac4 — j___ZN3RBX9GuiObject15convertFontSizeENS_11TextService8FontSizeE
// type: int __fastcall(int)
// RBX::GuiObject::convertFontSize(RBX::TextService::FontSize)
#[doc(alias = "RBX::GuiObject::convertFontSize(RBX::TextService::FontSize)")]
#[doc(alias = "j___ZN3RBX9GuiObject15convertFontSizeENS_11TextService8FontSizeE")]
pub fn stub_0xf48ac4() -> ! {
    todo!("0xf48ac4 RBX::GuiObject::convertFontSize(RBX::TextService::FontSize)")
}

// 0xf48ad4 — j___ZN3RBX9GuiObjectD2Ev
// type: void __fastcall(RBX::GuiObject *__hidden this)
// RBX::GuiObject::~GuiObject()
#[doc(alias = "RBX::GuiObject::~GuiObject()")]
#[doc(alias = "j___ZN3RBX9GuiObjectD2Ev")]
pub fn stub_0xf48ad4() -> ! {
    todo!("0xf48ad4 RBX::GuiObject::~GuiObject()")
}

// 0xf48c74 — j___ZN5boost10scoped_ptrIN3RBX9GuiObject5TweenEED2Ev
// type: int __fastcall(int, int, int, int, int)
// boost::scoped_ptr<RBX::GuiObject::Tween>::~scoped_ptr()
#[doc(alias = "boost::scoped_ptr<RBX::GuiObject::Tween>::~scoped_ptr()")]
#[doc(alias = "j___ZN5boost10scoped_ptrIN3RBX9GuiObject5TweenEED2Ev")]
pub fn stub_0xf48c74() -> ! {
    todo!("0xf48c74 boost::scoped_ptr<RBX::GuiObject::Tween>::~scoped_ptr()")
}

// 0xf48d04 — j___ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE5clearEv
// type: int __fastcall(int)
// boost::function1<void,RBX::GuiObject::TweenStatus>::clear(void)
#[doc(alias = "boost::function1<void,RBX::GuiObject::TweenStatus>::clear(void)")]
#[doc(alias = "j___ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE5clearEv")]
pub fn stub_0xf48d04() -> ! {
    todo!("0xf48d04 boost::function1<void,RBX::GuiObject::TweenStatus>::clear(void)")
}

// 0xf48e74 — j___ZNK3RBX9GuiBase2d9isVisibleERKN3G3D6Rect2DE
// type: int()
// RBX::GuiBase2d::isVisible(G3D::Rect2D const&)const
#[doc(alias = "RBX::GuiBase2d::isVisible(G3D::Rect2D const&)const")]
#[doc(alias = "j___ZNK3RBX9GuiBase2d9isVisibleERKN3G3D6Rect2DE")]
pub fn stub_0xf48e74() -> ! {
    todo!("0xf48e74 RBX::GuiBase2d::isVisible(G3D::Rect2D const&)const")
}

// 0xf49824 — j___ZN3RBX11shared_fromINS_10ControllerEEEN5boost10shared_ptrIT_EEPS4_
// type: int()
// was: boost::shared_ptr<RBX::Controller> RBX::shared_from<RBX::Controller>(RBX::Controller*)
#[doc(alias = "rbx_core::SharedPtr<RBX::Controller> RBX::shared_from<RBX::Controller>(RBX::Controller*)")]
#[doc(alias = "j___ZN3RBX11shared_fromINS_10ControllerEEEN5boost10shared_ptrIT_EEPS4_")]
pub fn stub_0xf49824() -> ! {
    todo!("0xf49824 rbx_core::SharedPtr<RBX::Controller> RBX::shared_from<RBX::Controller>(RBX::Controller*)")
}

// 0xf49844 — j___ZN3RBX12GuiDrawImageC2Ev
// type: _DWORD __fastcall(RBX::GuiDrawImage *__hidden this)
// RBX::GuiDrawImage::GuiDrawImage(void)
#[doc(alias = "RBX::GuiDrawImage::GuiDrawImage(void)")]
#[doc(alias = "j___ZN3RBX12GuiDrawImageC2Ev")]
pub fn stub_0xf49844() -> ! {
    todo!("0xf49844 RBX::GuiDrawImage::GuiDrawImage(void)")
}

// 0xf49994 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10Controller6ButtonEEERS3_RKT_
// type: int __fastcall(_DWORD, _DWORD)
// rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Controller::Button>(RBX::Controller::Button const&)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Controller::Button>(RBX::Controller::Button const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10Controller6ButtonEEERS3_RKT_")]
pub fn stub_0xf49994() -> ! {
    todo!("0xf49994 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Controller::Button>(RBX::Controller::Button const&)")
}

// 0xf499b4 — j___ZN3rbx7signals16signal_with_argsILi1EFvN3RBX10Controller6ButtonEEEclES4_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
// rbx::signals::signal_with_args<1,void ()(RBX::Controller::Button)>::operator()(RBX::Controller::Button)
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Controller::Button)>::operator()(RBX::Controller::Button)")]
#[doc(alias = "j___ZN3rbx7signals16signal_with_argsILi1EFvN3RBX10Controller6ButtonEEEclES4_")]
pub fn stub_0xf499b4() -> ! {
    todo!("0xf499b4 rbx::signals::signal_with_args<1,void ()(RBX::Controller::Button)>::operator()(RBX::Controller::Button)")
}

// 0xf499c4 — j___ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
// rbx::signals::signal<void ()(RBX::Controller::Button)>::disconnectAll(void)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Controller::Button)>::disconnectAll(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE13disconnectAllEv")]
pub fn stub_0xf499c4() -> ! {
    todo!("0xf499c4 rbx::signals::signal<void ()(RBX::Controller::Button)>::disconnectAll(void)")
}

// 0xf499d4 — j___ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE24safe_static_do_get_mutexEv
// type: int __fastcall(_DWORD)
// rbx::signals::signal<void ()(RBX::Controller::Button)>::safe_static_do_get_mutex(void)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Controller::Button)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE24safe_static_do_get_mutexEv")]
pub fn stub_0xf499d4() -> ! {
    todo!("0xf499d4 rbx::signals::signal<void ()(RBX::Controller::Button)>::safe_static_do_get_mutex(void)")
}

// 0xf499e4 — j___ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
// rbx::signals::signal<void ()(RBX::Controller::Button)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot> &)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Controller::Button)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot> &)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE")]
pub fn stub_0xf499e4() -> ! {
    todo!("0xf499e4 rbx::signals::signal<void ()(RBX::Controller::Button)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot> &)")
}

// 0xf499f4 — j___ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE4slot24safe_static_do_get_mutexEv
// type: int(void)
// rbx::signals::signal<void ()(RBX::Controller::Button)>::slot::safe_static_do_get_mutex(void)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Controller::Button)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE4slot24safe_static_do_get_mutexEv")]
pub fn stub_0xf499f4() -> ! {
    todo!("0xf499f4 rbx::signals::signal<void ()(RBX::Controller::Button)>::slot::safe_static_do_get_mutex(void)")
}

// 0xf49a04 — j___ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE6insertEPNS6_4slotE
// type: void __fastcall(int *, int, int, int, boost::mutex *, char, int, int, int, int)
// rbx::signals::signal<void ()(RBX::Controller::Button)>::insert(rbx::signals::signal<void ()(RBX::Controller::Button)>::slot *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Controller::Button)>::insert(rbx::signals::signal<void ()(RBX::Controller::Button)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE6insertEPNS6_4slotE")]
pub fn stub_0xf49a04() -> ! {
    todo!("0xf49a04 rbx::signals::signal<void ()(RBX::Controller::Button)>::insert(rbx::signals::signal<void ()(RBX::Controller::Button)>::slot *)")
}

// 0xf49a14 — j___ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE6removeEPNS6_4slotE
// type: int __fastcall(int, char *)
// rbx::signals::signal<void ()(RBX::Controller::Button)>::remove(rbx::signals::signal<void ()(RBX::Controller::Button)>::slot *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Controller::Button)>::remove(rbx::signals::signal<void ()(RBX::Controller::Button)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE6removeEPNS6_4slotE")]
pub fn stub_0xf49a14() -> ! {
    todo!("0xf49a14 rbx::signals::signal<void ()(RBX::Controller::Button)>::remove(rbx::signals::signal<void ()(RBX::Controller::Button)>::slot *)")
}

// 0xf49a24 — j___ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int, int)
// rbx::signals::connection rbx::signals::signal<void ()(RBX::Controller::Button)>::connect<boost::function<void ()(RBX::Controller::Button)>>(boost::function<void ()(RBX::Controller::Button)> const&)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Controller::Button)>::connect<boost::function<void ()(RBX::Controller::Button)>>(boost::function<void ()(RBX::Controller::Button)> const&)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_")]
pub fn stub_0xf49a24() -> ! {
    todo!("0xf49a24 rbx::signals::connection rbx::signals::signal<void ()(RBX::Controller::Button)>::connect<boost::function<void ()(RBX::Controller::Button)>>(boost::function<void ()(RBX::Controller::Button)> const&)")
}

// 0xf49a34 — j___ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE8on_errorERSt9exception
// rbx::signals::signal<void ()(RBX::Controller::Button)>::on_error(std::exception &)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Controller::Button)>::on_error(std::exception &)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE8on_errorERSt9exception")]
pub fn stub_0xf49a34() -> ! {
    todo!("0xf49a34 rbx::signals::signal<void ()(RBX::Controller::Button)>::on_error(std::exception &)")
}

// 0xf49a44 — j___ZN3rbx8any_castIN3RBX10Controller6ButtonENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// RBX::Controller::Button * rbx::any_cast<RBX::Controller::Button,RBX::Region3>(rbx::placement_any<RBX::Region3> *)
#[doc(alias = "RBX::Controller::Button * rbx::any_cast<RBX::Controller::Button,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "j___ZN3rbx8any_castIN3RBX10Controller6ButtonENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
pub fn stub_0xf49a44() -> ! {
    todo!("0xf49a44 RBX::Controller::Button * rbx::any_cast<RBX::Controller::Button,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")
}

// 0xf49a54 — j___ZN3rbx8any_castIRKN3RBX10Controller6ButtonENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// RBX::Controller::Button const& rbx::any_cast<RBX::Controller::Button const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
#[doc(alias = "RBX::Controller::Button const& rbx::any_cast<RBX::Controller::Button const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRKN3RBX10Controller6ButtonENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0xf49a54() -> ! {
    todo!("0xf49a54 RBX::Controller::Button const& rbx::any_cast<RBX::Controller::Button const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0xf49a64 — j___ZN3rbx8any_castIRN3RBX10Controller6ButtonENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// RBX::Controller::Button & rbx::any_cast<RBX::Controller::Button &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
#[doc(alias = "RBX::Controller::Button & rbx::any_cast<RBX::Controller::Button &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRN3RBX10Controller6ButtonENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0xf49a64() -> ! {
    todo!("0xf49a64 RBX::Controller::Button & rbx::any_cast<RBX::Controller::Button &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0xf49a74 — j___ZN3rbx8callableINS_7signals6signalIFvN3RBX10Controller6ButtonEEE4slotEN5boost8functionIS6_EELi1ES6_EC2IPS7_EERKSB_T_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
// rbx::callable<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot,boost::function<void ()(RBX::Controller::Button)>,1,void ()(RBX::Controller::Button)>::callable<rbx::signals::signal<void ()(RBX::Controller::Button)>*>(boost::function<void ()(RBX::Controller::Button)> const&,rbx::signals::signal<void ()(RBX::Controller::Button)>*)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot,boost::function<void ()(RBX::Controller::Button)>,1,void ()(RBX::Controller::Button)>::callable<rbx::signals::signal<void ()(RBX::Controller::Button)>*>(boost::function<void ()(RBX::Controller::Button)> const&,rbx::signals::signal<void ()(RBX::Controller::Button)>*)")]
#[doc(alias = "j___ZN3rbx8callableINS_7signals6signalIFvN3RBX10Controller6ButtonEEE4slotEN5boost8functionIS6_EELi1ES6_EC2IPS7_EERKSB_T_")]
pub fn stub_0xf49a74() -> ! {
    todo!("0xf49a74 rbx::callable<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot,boost::function<void ()(RBX::Controller::Button)>,1,void ()(RBX::Controller::Button)>::callable<rbx::signals::signal<void ()(RBX::Controller::Button)>*>(boost::function<void ()(RBX::Controller::Button)> const&,rbx::signals::signal<void ()(RBX::Controller::Button)>*)")
}

// 0xf49a84 — j___ZN5boost10shared_ptrIN3RBX10ControllerEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// was: boost::shared_ptr<RBX::Controller>::shared_ptr<RBX::Controller>(boost::weak_ptr<RBX::Controller> const&,boost::detail::sp_nothrow_tag)
#[doc(alias = "rbx_core::SharedPtr<RBX::Controller>::shared_ptr<RBX::Controller>(rbx_core::Weak<RBX::Controller> const&,boost::detail::sp_nothrow_tag)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX10ControllerEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")]
pub fn stub_0xf49a84() -> ! {
    todo!("0xf49a84 rbx_core::SharedPtr<RBX::Controller>::shared_ptr<RBX::Controller>(rbx_core::Weak<RBX::Controller> const&,boost::detail::sp_nothrow_tag)")
}

// 0xf49af4 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE4slotEEaSEPS9_
// type: int()
// rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Controller::Button)>::slot*)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Controller::Button)>::slot*)")]
#[doc(alias = "j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE4slotEEaSEPS9_")]
pub fn stub_0xf49af4() -> ! {
    todo!("0xf49af4 rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Controller::Button)>::slot*)")
}

// 0xf49b04 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE4slotEEaSERKSA_
// type: int __fastcall(_DWORD, _DWORD)
// rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot> const&)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot> const&)")]
#[doc(alias = "j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE4slotEEaSERKSA_")]
pub fn stub_0xf49b04() -> ! {
    todo!("0xf49b04 rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot> const&)")
}

// 0xf49b94 — j___ZN5boost9function1IvN3RBX10Controller6ButtonEE13assign_to_ownERKS4_
// boost::function1<void,RBX::Controller::Button>::assign_to_own(boost::function1<void,RBX::Controller::Button> const&)
#[doc(alias = "boost::function1<void,RBX::Controller::Button>::assign_to_own(boost::function1<void,RBX::Controller::Button> const&)")]
#[doc(alias = "j___ZN5boost9function1IvN3RBX10Controller6ButtonEE13assign_to_ownERKS4_")]
pub fn stub_0xf49b94() -> ! {
    todo!("0xf49b94 boost::function1<void,RBX::Controller::Button>::assign_to_own(boost::function1<void,RBX::Controller::Button> const&)")
}

// 0xf49ba4 — j___ZN5boost9function1IvN3RBX10Controller6ButtonEE5clearEv
// boost::function1<void,RBX::Controller::Button>::clear(void)
#[doc(alias = "boost::function1<void,RBX::Controller::Button>::clear(void)")]
#[doc(alias = "j___ZN5boost9function1IvN3RBX10Controller6ButtonEE5clearEv")]
pub fn stub_0xf49ba4() -> ! {
    todo!("0xf49ba4 boost::function1<void,RBX::Controller::Button>::clear(void)")
}

// 0xf49bd4 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE11erase_nodesEPNS1_8ptr_nodeISA_EESK_
// boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>> *,boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>> *)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>> *,boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>> *)")]
#[doc(alias = "j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE11erase_nodesEPNS1_8ptr_nodeISA_EESK_")]
pub fn stub_0xf49bd4() -> ! {
    todo!("0xf49bd4 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>> *,boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>> *)")
}

// 0xf49be4 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE11rehash_implEm
// type: int(void)
// boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::rehash_impl(unsigned long)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::rehash_impl(unsigned long)")]
#[doc(alias = "j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE11rehash_implEm")]
pub fn stub_0xf49be4() -> ! {
    todo!("0xf49be4 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::rehash_impl(unsigned long)")
}

// 0xf49bf4 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE12emplace_implINS1_13emplace_args1ISA_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEbERS8_RKT_
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, std::string *, int, int, int, int)
// std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>>(RBX::Controller::Button const&,boost::unordered::detail::emplace_args1<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>> const&)
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>>(RBX::Controller::Button const&,boost::unordered::detail::emplace_args1<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>> const&)")]
#[doc(alias = "j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE12emplace_implINS1_13emplace_args1ISA_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEbERS8_RKT_")]
pub fn stub_0xf49bf4() -> ! {
    todo!("0xf49bf4 std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>>(RBX::Controller::Button const&,boost::unordered::detail::emplace_args1<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>> const&)")
}

// 0xf49c04 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE
// type: int()
// boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>> &,boost::unordered::detail::ptr_bucket *)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>> &,boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE")]
pub fn stub_0xf49c04() -> ! {
    todo!("0xf49c04 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>> &,boost::unordered::detail::ptr_bucket *)")
}

// 0xf49c14 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEEEEE9constructEv
// type: int __fastcall(_DWORD)
// boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>>>::construct(void)
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>>>::construct(void)")]
#[doc(alias = "j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEEEEE9constructEv")]
pub fn stub_0xf49c14() -> ! {
    todo!("0xf49c14 boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>>>::construct(void)")
}

// 0xf49c24 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEEEEED2Ev
// type: int __fastcall(_DWORD)
// boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>>>::~node_constructor()
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>>>::~node_constructor()")]
#[doc(alias = "j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEEEEED2Ev")]
pub fn stub_0xf49c24() -> ! {
    todo!("0xf49c24 boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>>>::~node_constructor()")
}

// 0xf49c34 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE10fix_bucketEmPNS1_10ptr_bucketE
// boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE10fix_bucketEmPNS1_10ptr_bucketE")]
pub fn stub_0xf49c34() -> ! {
    todo!("0xf49c34 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")
}

// 0xf49c44 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE11delete_nodeEPNS1_10ptr_bucketE
// boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::delete_node(boost::unordered::detail::ptr_bucket *)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE11delete_nodeEPNS1_10ptr_bucketE")]
pub fn stub_0xf49c44() -> ! {
    todo!("0xf49c44 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::delete_node(boost::unordered::detail::ptr_bucket *)")
}

// 0xf49c54 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm
// type: int(void)
// boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::create_buckets(unsigned long)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::create_buckets(unsigned long)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm")]
pub fn stub_0xf49c54() -> ! {
    todo!("0xf49c54 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::create_buckets(unsigned long)")
}

// 0xf49c64 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE14delete_bucketsEv
// boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::delete_buckets(void)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::delete_buckets(void)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE14delete_bucketsEv")]
pub fn stub_0xf49c64() -> ! {
    todo!("0xf49c64 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::delete_buckets(void)")
}

// 0xf49c74 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE18reserve_for_insertEm
// type: int(void)
// boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::reserve_for_insert(unsigned long)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::reserve_for_insert(unsigned long)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE18reserve_for_insertEm")]
pub fn stub_0xf49c74() -> ! {
    todo!("0xf49c74 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::reserve_for_insert(unsigned long)")
}

// 0xf49c84 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEEC2EmRKSD_RKSF_RKSaINS1_8ptr_nodeISA_EEE
// boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::table(unsigned long,boost::hash<RBX::Controller::Button> const&,std::equal_to<RBX::Controller::Button> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>> const&)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::table(unsigned long,boost::hash<RBX::Controller::Button> const&,std::equal_to<RBX::Controller::Button> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>> const&)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEEC2EmRKSD_RKSF_RKSaINS1_8ptr_nodeISA_EEE")]
pub fn stub_0xf49c84() -> ! {
    todo!("0xf49c84 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::table(unsigned long,boost::hash<RBX::Controller::Button> const&,std::equal_to<RBX::Controller::Button> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>> const&)")
}

// 0xf49d64 — j___ZNK5boost9function1IvN3RBX10Controller6ButtonEEclES3_
// type: int(void)
// boost::function1<void,RBX::Controller::Button>::operator()(RBX::Controller::Button)const
#[doc(alias = "boost::function1<void,RBX::Controller::Button>::operator()(RBX::Controller::Button)const")]
#[doc(alias = "j___ZNK5boost9function1IvN3RBX10Controller6ButtonEEclES3_")]
pub fn stub_0xf49d64() -> ! {
    todo!("0xf49d64 boost::function1<void,RBX::Controller::Button>::operator()(RBX::Controller::Button)const")
}

// 0xf49d74 — j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE14find_node_implIS7_SF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEmRKT_RKT0_
// type: int __fastcall(_DWORD)
// boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::find_node_impl<RBX::Controller::Button,std::equal_to<RBX::Controller::Button>>(unsigned long,RBX::Controller::Button const&,std::equal_to<RBX::Controller::Button> const&)const
#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::find_node_impl<RBX::Controller::Button,std::equal_to<RBX::Controller::Button>>(unsigned long,RBX::Controller::Button const&,std::equal_to<RBX::Controller::Button> const&)const")]
#[doc(alias = "j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE14find_node_implIS7_SF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEmRKT_RKT0_")]
pub fn stub_0xf49d74() -> ! {
    todo!("0xf49d74 boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::find_node_impl<RBX::Controller::Button,std::equal_to<RBX::Controller::Button>>(unsigned long,RBX::Controller::Button const&,std::equal_to<RBX::Controller::Button> const&)const")
}

// 0xf49d84 — j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE20min_buckets_for_sizeEm
// boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::min_buckets_for_size(unsigned long)const
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::min_buckets_for_size(unsigned long)const")]
#[doc(alias = "j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE20min_buckets_for_sizeEm")]
pub fn stub_0xf49d84() -> ! {
    todo!("0xf49d84 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::min_buckets_for_size(unsigned long)const")
}

// 0xf49d94 — j___ZNSt12_Vector_baseIN3RBX10Controller6ButtonESaIS2_EE11_M_allocateEm
// std::_Vector_base<RBX::Controller::Button,std::allocator<RBX::Controller::Button>>::_M_allocate(unsigned long)
#[doc(alias = "std::_Vector_base<RBX::Controller::Button,std::allocator<RBX::Controller::Button>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIN3RBX10Controller6ButtonESaIS2_EE11_M_allocateEm")]
pub fn stub_0xf49d94() -> ! {
    todo!("0xf49d94 std::_Vector_base<RBX::Controller::Button,std::allocator<RBX::Controller::Button>>::_M_allocate(unsigned long)")
}

// 0xf49dc4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10Controller6ButtonES6_EET0_T_S8_S7_
// type: int()
// RBX::Controller::Button * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Controller::Button *,RBX::Controller::Button *>(RBX::Controller::Button *,RBX::Controller::Button *,RBX::Controller::Button *)
#[doc(alias = "RBX::Controller::Button * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Controller::Button *,RBX::Controller::Button *>(RBX::Controller::Button *,RBX::Controller::Button *,RBX::Controller::Button *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10Controller6ButtonES6_EET0_T_S8_S7_")]
pub fn stub_0xf49dc4() -> ! {
    todo!("0xf49dc4 RBX::Controller::Button * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Controller::Button *,RBX::Controller::Button *>(RBX::Controller::Button *,RBX::Controller::Button *,RBX::Controller::Button *)")
}

// 0xf49de4 — j___ZNSt3mapIPKN3RBX4NameENS0_10Controller6ButtonESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// std::map<RBX::Name const*,RBX::Controller::Button,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Controller::Button>>>::operator[](RBX::Name const* const&)
#[doc(alias = "std::map<RBX::Name const*,RBX::Controller::Button,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Controller::Button>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "j___ZNSt3mapIPKN3RBX4NameENS0_10Controller6ButtonESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
pub fn stub_0xf49de4() -> ! {
    todo!("0xf49de4 std::map<RBX::Name const*,RBX::Controller::Button,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Controller::Button>>>::operator[](RBX::Name const* const&)")
}

// 0xf49df4 — j___ZNSt6vectorIN3RBX10Controller6ButtonESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
// std::vector<RBX::Controller::Button,std::allocator<RBX::Controller::Button>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Controller::Button*,std::vector<RBX::Controller::Button,std::allocator<RBX::Controller::Button>>>,RBX::Controller::Button const&)
#[doc(alias = "std::vector<RBX::Controller::Button,std::allocator<RBX::Controller::Button>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Controller::Button*,std::vector<RBX::Controller::Button,std::allocator<RBX::Controller::Button>>>,RBX::Controller::Button const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX10Controller6ButtonESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_0xf49df4() -> ! {
    todo!("0xf49df4 std::vector<RBX::Controller::Button,std::allocator<RBX::Controller::Button>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Controller::Button*,std::vector<RBX::Controller::Button,std::allocator<RBX::Controller::Button>>>,RBX::Controller::Button const&)")
}

// 0xf49e04 — j___ZNSt6vectorIN3RBX10Controller6ButtonESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// std::vector<RBX::Controller::Button,std::allocator<RBX::Controller::Button>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Controller::Button*,std::vector<RBX::Controller::Button,std::allocator<RBX::Controller::Button>>>,unsigned long,RBX::Controller::Button const&)
#[doc(alias = "std::vector<RBX::Controller::Button,std::allocator<RBX::Controller::Button>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Controller::Button*,std::vector<RBX::Controller::Button,std::allocator<RBX::Controller::Button>>>,unsigned long,RBX::Controller::Button const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX10Controller6ButtonESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_0xf49e04() -> ! {
    todo!("0xf49e04 std::vector<RBX::Controller::Button,std::allocator<RBX::Controller::Button>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Controller::Button*,std::vector<RBX::Controller::Button,std::allocator<RBX::Controller::Button>>>,unsigned long,RBX::Controller::Button const&)")
}

// 0xf49e14 — j___ZNSt6vectorIN3RBX10Controller6ButtonESaIS2_EE6resizeEmS2_
// std::vector<RBX::Controller::Button,std::allocator<RBX::Controller::Button>>::resize(unsigned long,RBX::Controller::Button)
#[doc(alias = "std::vector<RBX::Controller::Button,std::allocator<RBX::Controller::Button>>::resize(unsigned long,RBX::Controller::Button)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX10Controller6ButtonESaIS2_EE6resizeEmS2_")]
pub fn stub_0xf49e14() -> ! {
    todo!("0xf49e14 std::vector<RBX::Controller::Button,std::allocator<RBX::Controller::Button>>::resize(unsigned long,RBX::Controller::Button)")
}

// 0xf49e24 — j___ZNSt6vectorIN3RBX10Controller6ButtonESaIS2_EE9push_backERKS2_
// std::vector<RBX::Controller::Button,std::allocator<RBX::Controller::Button>>::push_back(RBX::Controller::Button const&)
#[doc(alias = "std::vector<RBX::Controller::Button,std::allocator<RBX::Controller::Button>>::push_back(RBX::Controller::Button const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX10Controller6ButtonESaIS2_EE9push_backERKS2_")]
pub fn stub_0xf49e24() -> ! {
    todo!("0xf49e24 std::vector<RBX::Controller::Button,std::allocator<RBX::Controller::Button>>::push_back(RBX::Controller::Button const&)")
}

// 0xf49e54 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Controller6ButtonEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Controller::Button>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Controller::Button>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Controller::Button>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Controller::Button> const&)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Controller::Button>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Controller::Button>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Controller::Button>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Controller::Button> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Controller6ButtonEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
pub fn stub_0xf49e54() -> ! {
    todo!("0xf49e54 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Controller::Button>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Controller::Button>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Controller::Button>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Controller::Button> const&)")
}

// 0xf49e64 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Controller6ButtonEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
// std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Controller::Button>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Controller::Button>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Controller::Button>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Controller::Button>>,std::pair<RBX::Name const* const,RBX::Controller::Button> const&)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Controller::Button>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Controller::Button>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Controller::Button>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Controller::Button>>,std::pair<RBX::Name const* const,RBX::Controller::Button> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Controller6ButtonEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
pub fn stub_0xf49e64() -> ! {
    todo!("0xf49e64 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Controller::Button>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Controller::Button>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Controller::Button>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Controller::Button>>,std::pair<RBX::Name const* const,RBX::Controller::Button> const&)")
}

// 0xf49e74 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Controller6ButtonEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Controller::Button>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Controller::Button>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Controller::Button>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Controller::Button>> *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Controller::Button>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Controller::Button>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Controller::Button>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Controller::Button>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Controller6ButtonEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_0xf49e74() -> ! {
    todo!("0xf49e74 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Controller::Button>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Controller::Button>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Controller::Button>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Controller::Button>> *)")
}

// 0xf49e84 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Controller6ButtonEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Controller::Button>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Controller::Button>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Controller::Button>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Controller::Button> const&)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Controller::Button>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Controller::Button>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Controller::Button>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Controller::Button> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Controller6ButtonEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
pub fn stub_0xf49e84() -> ! {
    todo!("0xf49e84 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Controller::Button>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Controller::Button>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Controller::Button>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Controller::Button> const&)")
}

// 0xf4af74 — j___ZN3G3D5ArrayIPN3RBX11RotateJointELi10ELm32EE6appendERKS3_
// G3D::Array<RBX::RotateJoint *,10,32ul>::append(RBX::RotateJoint * const&)
#[doc(alias = "G3D::Array<RBX::RotateJoint *,10,32ul>::append(RBX::RotateJoint * const&)")]
#[doc(alias = "j___ZN3G3D5ArrayIPN3RBX11RotateJointELi10ELm32EE6appendERKS3_")]
pub fn stub_0xf4af74() -> ! {
    todo!("0xf4af74 G3D::Array<RBX::RotateJoint *,10,32ul>::append(RBX::RotateJoint * const&)")
}

// 0xf4af84 — j___ZN3G3D5ArrayIPN3RBX11RotateJointELi10ELm32EE6resizeEib
// G3D::Array<RBX::RotateJoint *,10,32ul>::resize(int,bool)
#[doc(alias = "G3D::Array<RBX::RotateJoint *,10,32ul>::resize(int,bool)")]
#[doc(alias = "j___ZN3G3D5ArrayIPN3RBX11RotateJointELi10ELm32EE6resizeEib")]
pub fn stub_0xf4af84() -> ! {
    todo!("0xf4af84 G3D::Array<RBX::RotateJoint *,10,32ul>::resize(int,bool)")
}

// 0xf4af94 — j___ZN3G3D5ArrayIPN3RBX11RotateJointELi10ELm32EE7reallocEi
// type: int(void)
// G3D::Array<RBX::RotateJoint *,10,32ul>::realloc(int)
#[doc(alias = "G3D::Array<RBX::RotateJoint *,10,32ul>::realloc(int)")]
#[doc(alias = "j___ZN3G3D5ArrayIPN3RBX11RotateJointELi10ELm32EE7reallocEi")]
pub fn stub_0xf4af94() -> ! {
    todo!("0xf4af94 G3D::Array<RBX::RotateJoint *,10,32ul>::realloc(int)")
}

// 0xf4afa4 — j___ZN3G3D5ArrayIPN3RBX11RotateJointELi10ELm32EEC2Ev
// G3D::Array<RBX::RotateJoint *,10,32ul>::Array(void)
#[doc(alias = "G3D::Array<RBX::RotateJoint *,10,32ul>::Array(void)")]
#[doc(alias = "j___ZN3G3D5ArrayIPN3RBX11RotateJointELi10ELm32EEC2Ev")]
pub fn stub_0xf4afa4() -> ! {
    todo!("0xf4afa4 G3D::Array<RBX::RotateJoint *,10,32ul>::Array(void)")
}

// 0xf4afb4 — j___ZN3G3D5ArrayIPN3RBX11RotateJointELi10ELm32EED2Ev
// G3D::Array<RBX::RotateJoint *,10,32ul>::~Array()
#[doc(alias = "G3D::Array<RBX::RotateJoint *,10,32ul>::~Array()")]
#[doc(alias = "j___ZN3G3D5ArrayIPN3RBX11RotateJointELi10ELm32EED2Ev")]
pub fn stub_0xf4afb4() -> ! {
    todo!("0xf4afb4 G3D::Array<RBX::RotateJoint *,10,32ul>::~Array()")
}

// 0xf4b054 — j___ZN3RBX11IndexedTree13getTypedChildINS_9PrimitiveEEEPT_i
// RBX::Primitive * RBX::IndexedTree::getTypedChild<RBX::Primitive>(int)
#[doc(alias = "RBX::Primitive * RBX::IndexedTree::getTypedChild<RBX::Primitive>(int)")]
#[doc(alias = "j___ZN3RBX11IndexedTree13getTypedChildINS_9PrimitiveEEEPT_i")]
pub fn stub_0xf4b054() -> ! {
    todo!("0xf4b054 RBX::Primitive * RBX::IndexedTree::getTypedChild<RBX::Primitive>(int)")
}

// 0xf4b064 — j___ZN3RBX11KernelJoint7getBodyENS_9Connector9BodyIndexE
// type: int(void)
// RBX::KernelJoint::getBody(RBX::Connector::BodyIndex)
#[doc(alias = "RBX::KernelJoint::getBody(RBX::Connector::BodyIndex)")]
#[doc(alias = "j___ZN3RBX11KernelJoint7getBodyENS_9Connector9BodyIndexE")]
pub fn stub_0xf4b064() -> ! {
    todo!("0xf4b064 RBX::KernelJoint::getBody(RBX::Connector::BodyIndex)")
}

// 0xf4b074 — j___ZN3RBX11KernelJointD0Ev
// type: void __fastcall(RBX::KernelJoint *__hidden this)
// RBX::KernelJoint::~KernelJoint()
#[doc(alias = "RBX::KernelJoint::~KernelJoint()")]
#[doc(alias = "j___ZN3RBX11KernelJointD0Ev")]
pub fn stub_0xf4b074() -> ! {
    todo!("0xf4b074 RBX::KernelJoint::~KernelJoint()")
}

// 0xf4b104 — j___ZN3RBX8Assembly19visitPrimitivesImplIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_11VehicleSeatEPNS_9PrimitiveEEENS3_5list2INS3_5valueIPS7_EENS2_3argILi1EEEEEEEEEvT_S9_
// void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VehicleSeat,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::VehicleSeat*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VehicleSeat,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::VehicleSeat*>,boost::arg<1>>>,RBX::Primitive *)
#[doc(alias = "void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VehicleSeat,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::VehicleSeat*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VehicleSeat,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::VehicleSeat*>,boost::arg<1>>>,RBX::Primitive *)")]
#[doc(alias = "j___ZN3RBX8Assembly19visitPrimitivesImplIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_11VehicleSeatEPNS_9PrimitiveEEENS3_5list2INS3_5valueIPS7_EENS2_3argILi1EEEEEEEEEvT_S9_")]
pub fn stub_0xf4b104() -> ! {
    todo!("0xf4b104 void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VehicleSeat,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::VehicleSeat*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::VehicleSeat,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::VehicleSeat*>,boost::arg<1>>>,RBX::Primitive *)")
}

// 0xf4b114 — j___ZN3RBX8Assembly19visitPrimitivesImplIN5boost3_bi6bind_tIvPFvPNS_9PrimitiveERSt6vectorIPKS5_SaIS9_EEENS3_5list2INS2_3argILi1EEENS2_17reference_wrapperISB_EEEEEEEEvT_S6_
// type: int(void)
// void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,void (*)(RBX::Primitive *,std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>>>>>(boost::_bi::bind_t<void,void (*)(RBX::Primitive *,std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>>>>,RBX::Primitive *)
#[doc(alias = "void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,void (*)(RBX::Primitive *,std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>>>>>(boost::_bi::bind_t<void,void (*)(RBX::Primitive *,std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>>>>,RBX::Primitive *)")]
#[doc(alias = "j___ZN3RBX8Assembly19visitPrimitivesImplIN5boost3_bi6bind_tIvPFvPNS_9PrimitiveERSt6vectorIPKS5_SaIS9_EEENS3_5list2INS2_3argILi1EEENS2_17reference_wrapperISB_EEEEEEEEvT_S6_")]
pub fn stub_0xf4b114() -> ! {
    todo!("0xf4b114 void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,void (*)(RBX::Primitive *,std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>>>>>(boost::_bi::bind_t<void,void (*)(RBX::Primitive *,std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>>>>,RBX::Primitive *)")
}

// 0xf4b1f4 — j___ZN5boost10shared_ptrIN3RBX17VehicleControllerEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// was: boost::shared_ptr<RBX::VehicleController>::shared_ptr<RBX::VehicleController>(boost::weak_ptr<RBX::VehicleController> const&,boost::detail::sp_nothrow_tag)
#[doc(alias = "rbx_core::SharedPtr<RBX::VehicleController>::shared_ptr<RBX::VehicleController>(rbx_core::Weak<RBX::VehicleController> const&,boost::detail::sp_nothrow_tag)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX17VehicleControllerEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")]
pub fn stub_0xf4b1f4() -> ! {
    todo!("0xf4b1f4 rbx_core::SharedPtr<RBX::VehicleController>::shared_ptr<RBX::VehicleController>(rbx_core::Weak<RBX::VehicleController> const&,boost::detail::sp_nothrow_tag)")
}

// 0xf4b904 — j___ZN3RBX11shared_fromINS_5DecalEEEN5boost10shared_ptrIT_EEPS4_
// was: boost::shared_ptr<RBX::Decal> RBX::shared_from<RBX::Decal>(RBX::Decal*)
#[doc(alias = "rbx_core::SharedPtr<RBX::Decal> RBX::shared_from<RBX::Decal>(RBX::Decal*)")]
#[doc(alias = "j___ZN3RBX11shared_fromINS_5DecalEEEN5boost10shared_ptrIT_EEPS4_")]
pub fn stub_0xf4b904() -> ! {
    todo!("0xf4b904 rbx_core::SharedPtr<RBX::Decal> RBX::shared_from<RBX::Decal>(RBX::Decal*)")
}

// 0xf4b914 — j___ZN3RBX11shared_fromINS_6CameraEEEN5boost10shared_ptrIT_EEPS4_
// was: boost::shared_ptr<RBX::Camera> RBX::shared_from<RBX::Camera>(RBX::Camera*)
#[doc(alias = "rbx_core::SharedPtr<RBX::Camera> RBX::shared_from<RBX::Camera>(RBX::Camera*)")]
#[doc(alias = "j___ZN3RBX11shared_fromINS_6CameraEEEN5boost10shared_ptrIT_EEPS4_")]
pub fn stub_0xf4b914() -> ! {
    todo!("0xf4b914 rbx_core::SharedPtr<RBX::Camera> RBX::shared_from<RBX::Camera>(RBX::Camera*)")
}

// 0xf4bd64 — j___ZN5boost10shared_ptrIN3RBX5DecalEEaSERKS3_
// was: boost::shared_ptr<RBX::Decal>::operator=(boost::shared_ptr<RBX::Decal> const&)
#[doc(alias = "rbx_core::SharedPtr<RBX::Decal>::operator=(rbx_core::SharedPtr<RBX::Decal> const&)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX5DecalEEaSERKS3_")]
pub fn stub_0xf4bd64() -> ! {
    todo!("0xf4bd64 rbx_core::SharedPtr<RBX::Decal>::operator=(rbx_core::SharedPtr<RBX::Decal> const&)")
}

// 0xf4bda4 — j___ZN5boost10shared_ptrIN3RBX6CameraEEaSERKS3_
// was: boost::shared_ptr<RBX::Camera>::operator=(boost::shared_ptr<RBX::Camera> const&)
#[doc(alias = "rbx_core::SharedPtr<RBX::Camera>::operator=(rbx_core::SharedPtr<RBX::Camera> const&)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX6CameraEEaSERKS3_")]
pub fn stub_0xf4bda4() -> ! {
    todo!("0xf4bda4 rbx_core::SharedPtr<RBX::Camera>::operator=(rbx_core::SharedPtr<RBX::Camera> const&)")
}

// 0xf4bdd4 — j___ZN5boost10shared_ptrIN3RBX9DecalToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::DecalTool>::shared_ptr<RBX::DecalTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::DecalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::DecalTool>::shared_ptr<RBX::DecalTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::DecalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX9DecalToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_")]
pub fn stub_0xf4bdd4() -> ! {
    todo!("0xf4bdd4 rbx_core::SharedPtr<RBX::DecalTool>::shared_ptr<RBX::DecalTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::DecalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")
}

// 0xf4bea4 — j___ZN5boost6detail12shared_countC2IPN3RBX11NewNullToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
// boost::detail::shared_count::shared_count<RBX::NewNullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::NewNullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::NewNullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::NewNullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IPN3RBX11NewNullToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_")]
pub fn stub_0xf4bea4() -> ! {
    todo!("0xf4bea4 boost::detail::shared_count::shared_count<RBX::NewNullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::NewNullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")
}

// 0xf4beb4 — j___ZN5boost6detail12shared_countC2IPN3RBX12AdvArrowToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
// boost::detail::shared_count::shared_count<RBX::AdvArrowTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvArrowTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AdvArrowTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvArrowTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IPN3RBX12AdvArrowToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_")]
pub fn stub_0xf4beb4() -> ! {
    todo!("0xf4beb4 boost::detail::shared_count::shared_count<RBX::AdvArrowTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvArrowTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")
}

// 0xf4bf24 — j___ZN5boost6detail12shared_countC2IPN3RBX9DecalToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
// boost::detail::shared_count::shared_count<RBX::DecalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::DecalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::DecalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::DecalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IPN3RBX9DecalToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_")]
pub fn stub_0xf4bf24() -> ! {
    todo!("0xf4bf24 boost::detail::shared_count::shared_count<RBX::DecalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::DecalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")
}

// 0xf4c014 — j___ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_11NewNullToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::NewNullTool,RBX::NewNullTool>(boost::shared_ptr<RBX::NewNullTool> const*,RBX::NewNullTool *)const
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::NewNullTool,RBX::NewNullTool>(rbx_core::SharedPtr<RBX::NewNullTool> const*,RBX::NewNullTool *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_11NewNullToolES5_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0xf4c014() -> ! {
    todo!("0xf4c014 void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::NewNullTool,RBX::NewNullTool>(rbx_core::SharedPtr<RBX::NewNullTool> const*,RBX::NewNullTool *)const")
}

// 0xf4c024 — j___ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_12AdvArrowToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::AdvArrowTool,RBX::AdvArrowTool>(boost::shared_ptr<RBX::AdvArrowTool> const*,RBX::AdvArrowTool *)const
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::AdvArrowTool,RBX::AdvArrowTool>(rbx_core::SharedPtr<RBX::AdvArrowTool> const*,RBX::AdvArrowTool *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_12AdvArrowToolES5_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0xf4c024() -> ! {
    todo!("0xf4c024 void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::AdvArrowTool,RBX::AdvArrowTool>(rbx_core::SharedPtr<RBX::AdvArrowTool> const*,RBX::AdvArrowTool *)const")
}

// 0xf4c034 — j___ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_9DecalToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::DecalTool,RBX::DecalTool>(boost::shared_ptr<RBX::DecalTool> const*,RBX::DecalTool *)const
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::DecalTool,RBX::DecalTool>(rbx_core::SharedPtr<RBX::DecalTool> const*,RBX::DecalTool *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_9DecalToolES5_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0xf4c034() -> ! {
    todo!("0xf4c034 void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::DecalTool,RBX::DecalTool>(rbx_core::SharedPtr<RBX::DecalTool> const*,RBX::DecalTool *)const")
}

// 0xf4c424 — j___ZN3RBX10IndexArrayINS_7SimBodyEXadL_ZNS1_17getJointBodyIndexEvEEE10fastRemoveEPS1_
// RBX::IndexArray<RBX::SimBody,&RBX::SimBody::getJointBodyIndex>::fastRemove(RBX::SimBody*)
#[doc(alias = "RBX::IndexArray<RBX::SimBody,&RBX::SimBody::getJointBodyIndex>::fastRemove(RBX::SimBody*)")]
#[doc(alias = "j___ZN3RBX10IndexArrayINS_7SimBodyEXadL_ZNS1_17getJointBodyIndexEvEEE10fastRemoveEPS1_")]
pub fn stub_0xf4c424() -> ! {
    todo!("0xf4c424 RBX::IndexArray<RBX::SimBody,&RBX::SimBody::getJointBodyIndex>::fastRemove(RBX::SimBody*)")
}

// 0xf4c464 — j___ZN3RBX10IndexArrayINS_9ConnectorEXadL_ZNS1_13getJointIndexEvEEE10fastRemoveEPS1_
// RBX::IndexArray<RBX::Connector,&RBX::Connector::getJointIndex>::fastRemove(RBX::Connector*)
#[doc(alias = "RBX::IndexArray<RBX::Connector,&RBX::Connector::getJointIndex>::fastRemove(RBX::Connector*)")]
#[doc(alias = "j___ZN3RBX10IndexArrayINS_9ConnectorEXadL_ZNS1_13getJointIndexEvEEE10fastRemoveEPS1_")]
pub fn stub_0xf4c464() -> ! {
    todo!("0xf4c464 RBX::IndexArray<RBX::Connector,&RBX::Connector::getJointIndex>::fastRemove(RBX::Connector*)")
}

// 0xf4c604 — j___ZN16XmlNameValuePair8setValueEN3RBX9ContentIdE
// type: int __fastcall(_DWORD, _DWORD)
// XmlNameValuePair::setValue(RBX::ContentId)
#[doc(alias = "XmlNameValuePair::setValue(RBX::ContentId)")]
#[doc(alias = "j___ZN16XmlNameValuePair8setValueEN3RBX9ContentIdE")]
pub fn stub_0xf4c604() -> ! {
    todo!("0xf4c604 XmlNameValuePair::setValue(RBX::ContentId)")
}

// 0xf4d0d4 — j___ZN3RBX13FWStringValueC1EPKc
// type: _DWORD __fastcall(RBX::FWStringValue *__hidden this, const char *__s)
// RBX::FWStringValue::FWStringValue(char const*)
#[doc(alias = "RBX::FWStringValue::FWStringValue(char const*)")]
#[doc(alias = "j___ZN3RBX13FWStringValueC1EPKc")]
pub fn stub_0xf4d0d4() -> ! {
    todo!("0xf4d0d4 RBX::FWStringValue::FWStringValue(char const*)")
}

// 0xf4d174 — j___ZN3RBX6FWBase4initINS_10FWInstanceEEEPT_S4_
// RBX::FWInstance * RBX::FWBase::init<RBX::FWInstance>(RBX::FWInstance *)
#[doc(alias = "RBX::FWInstance * RBX::FWBase::init<RBX::FWInstance>(RBX::FWInstance *)")]
#[doc(alias = "j___ZN3RBX6FWBase4initINS_10FWInstanceEEEPT_S4_")]
pub fn stub_0xf4d174() -> ! {
    todo!("0xf4d174 RBX::FWInstance * RBX::FWBase::init<RBX::FWInstance>(RBX::FWInstance *)")
}

// 0xf4d194 — j___ZN3RBX7FWFinalINS_10FWInstanceEED2Ev
// type: int(void)
// RBX::FWFinal<RBX::FWInstance>::~FWFinal()
#[doc(alias = "RBX::FWFinal<RBX::FWInstance>::~FWFinal()")]
#[doc(alias = "j___ZN3RBX7FWFinalINS_10FWInstanceEED2Ev")]
pub fn stub_0xf4d194() -> ! {
    todo!("0xf4d194 RBX::FWFinal<RBX::FWInstance>::~FWFinal()")
}

// 0xf4d1a4 — j___ZN3RBX7FWValueISsE3setERKSsPNS_5FWRefE
// type: int __fastcall(std::string *, std::string *this, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, boost::detail::sp_counted_base *, int, char, int, int, int, int)
// RBX::FWValue<std::string>::set(std::string const&,RBX::FWRef *)
#[doc(alias = "RBX::FWValue<std::string>::set(std::string const&,RBX::FWRef *)")]
#[doc(alias = "j___ZN3RBX7FWValueISsE3setERKSsPNS_5FWRefE")]
pub fn stub_0xf4d1a4() -> ! {
    todo!("0xf4d1a4 RBX::FWValue<std::string>::set(std::string const&,RBX::FWRef *)")
}

// 0xf4d234 — j___ZN3RBX9AllocatorINS_10FWInstanceEEC2Ev
// RBX::Allocator<RBX::FWInstance>::Allocator(void)
#[doc(alias = "RBX::Allocator<RBX::FWInstance>::Allocator(void)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_10FWInstanceEEC2Ev")]
pub fn stub_0xf4d234() -> ! {
    todo!("0xf4d234 RBX::Allocator<RBX::FWInstance>::Allocator(void)")
}

// 0xf4d244 — j___ZN3RBX9AllocatorINS_10FWInstanceEEnwEm
// RBX::Allocator<RBX::FWInstance>::operator new(unsigned long)
#[doc(alias = "RBX::Allocator<RBX::FWInstance>::operator new(unsigned long)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_10FWInstanceEEnwEm")]
pub fn stub_0xf4d244() -> ! {
    todo!("0xf4d244 RBX::Allocator<RBX::FWInstance>::operator new(unsigned long)")
}
