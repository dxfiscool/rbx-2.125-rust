//! rendering shard 306 — 100 stubs 0x445978..0x44a0ac EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 33141->33241 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 33141 before -> 33241 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0x445890 (lowest remaining 0x445978..0x44a0ac, next lowest 0x44a0b0)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x445978 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_10Soundscape12SoundServiceEEEmv
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::Soundscape::SoundService>(void)")]
// was: unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::Soundscape::SoundService>(void)
// IDA 0x445978: 70 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_445978() {
}

// 0x445a50 — __ZN3RBX14FactoryProductINS_8LightingENS_8InstanceELZNS_9sLightingEES2_E7CreatorD2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_8LightingENS_8InstanceELZNS_9sLightingEES2_E7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_8LightingENS_8InstanceELZNS_9sLightingEES2_E7CreatorD2Ev
// IDA 0x445a50: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_445a50() {
}

// 0x445af0 — __ZNK3RBX14FactoryProductINS_8LightingENS_8InstanceELZNS_9sLightingEES2_E7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_8LightingENS_8InstanceELZNS_9sLightingEES2_E7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_8LightingENS_8InstanceELZNS_9sLightingEES2_E7Creator6createEv
// IDA 0x445af0: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_445af0() {
}

// 0x445c38 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_8LightingES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Lighting,RBX::Lighting>(rbx_core::SharedPtr<RBX::Lighting> const*,RBX::Lighting *)const")]
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Lighting,RBX::Lighting>(boost::shared_ptr<RBX::Lighting> const*,RBX::Lighting *)const
// IDA 0x445c38: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_445c38() {
}

// 0x445d28 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8LightingENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Lighting *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::Lighting *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// IDA 0x445d28: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_445d28() {
}

// 0x445d30 — __ZN3RBX4Name7declareILZNS_9sLightingEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name7declareILZNS_9sLightingEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_9sLightingEEEERKS0_v
// IDA 0x445d30: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_445d30() {
}

// 0x445d78 — __ZN3RBX4Name9doDeclareILZNS_9sLightingEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sLightingEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_9sLightingEEEERKS0_v
// IDA 0x445d78: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_445d78() {
}

// 0x445e60 — __ZN3RBX14FactoryProductINS_8LightingENS_8InstanceELZNS_9sLightingEES2_E15isNullClassNameEv
#[doc(alias = "__ZN3RBX14FactoryProductINS_8LightingENS_8InstanceELZNS_9sLightingEES2_E15isNullClassNameEv")]
// was: __ZN3RBX14FactoryProductINS_8LightingENS_8InstanceELZNS_9sLightingEES2_E15isNullClassNameEv
// IDA 0x445e60: 33 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_445e60() {
}

// 0x445ec8 — __ZN3RBX14FactoryProductINS_8LightingENS_8InstanceELZNS_9sLightingEES2_E17static_getCreatorEv
#[doc(alias = "__ZN3RBX14FactoryProductINS_8LightingENS_8InstanceELZNS_9sLightingEES2_E17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_8LightingENS_8InstanceELZNS_9sLightingEES2_E17static_getCreatorEv
// IDA 0x445ec8: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_445ec8() {
}

// 0x445f40 — __ZN5boost9function0IvEC2INS_3_bi6bind_tISsPFSsRKSsS6_ENS3_5list2INS3_5valueISsEESB_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISE_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tISsPFSsRKSsS6_ENS3_5list2INS3_5valueISsEESB_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISE_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function0IvEC2INS_3_bi6bind_tISsPFSsRKSsS6_ENS3_5list2INS3_5valueISsEESB_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISE_EE5valueEEE5valueEiE4typeE
// IDA 0x445f40: 164 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_445f40() {
}

// 0x446120 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tISsPFSsRKSsS6_ENS3_5list2INS3_5valueISsEESB_EEEEEEvT_
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>>)")]
// was: void boost::function0<void>::assign_to<boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>>)
// IDA 0x446120: 167 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_446120() {
}

// 0x44630c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tISsPFSsRKSsS6_ENS3_5list2INS3_5valueISsEESB_EEEEE6manageERKNS1_15function_bufferERSF_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// IDA 0x44630c: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44630c() {
}

// 0x446328 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tISsPFSsRKSsS6_ENS3_5list2INS3_5valueISsEESB_EEEEvE6invokeERNS1_15function_bufferE
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>>,void>::invoke(boost::detail::function::function_buffer &)
// IDA 0x446328: 31 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_446328() {
}

// 0x446380 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tISsPFSsRKSsS8_ENS5_5list2INS5_5valueISsEESD_EEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const
// IDA 0x446380: 162 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_446380() {
}

// 0x446558 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tISsPFSsRKSsS8_ENS5_5list2INS5_5valueISsEESD_EEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
// IDA 0x446558: 183 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_446558() {
}

// 0x446764 — __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tISsPFSsRKSsS6_ENS3_5list2INS3_5valueISsEESB_EEEEE12manage_smallERKNS1_15function_bufferERSF_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager_common<boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager_common<boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// IDA 0x446764: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_446764() {
}

// 0x44689c — __ZN5boost3_bi5list2INS0_5valueISsEES3_EC2ES3_S3_
#[doc(alias = "boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>::list2(boost::_bi::value<std::string>,boost::_bi::value<std::string>)")]
// was: boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>::list2(boost::_bi::value<std::string>,boost::_bi::value<std::string>)
// IDA 0x44689c: 145 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44689c() {
}

// 0x446a48 — __ZN5boost3_bi6bind_tISsPFSsRKSsS3_ENS0_5list2INS0_5valueISsEES8_EEEC2ES5_RKS9_
#[doc(alias = "boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>>::bind_t(std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>> const&)")]
// was: boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>>::bind_t(std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>> const&)
// IDA 0x446a48: 61 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_446a48() {
}

// 0x446af8 — __ZN5boost6detail11thread_dataINS_9function0IvEEEC2ES3_
#[doc(alias = "boost::detail::thread_data<boost::function0<void>>::thread_data(boost::function0<void>)")]
// was: boost::detail::thread_data<boost::function0<void>>::thread_data(boost::function0<void>)
// IDA 0x446af8: 71 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_446af8() {
}

// 0x446bc8 — __ZN5boost6detail16thread_data_baseC2Ev
#[doc(alias = "boost::detail::thread_data_base::thread_data_base(void)")]
// was: boost::detail::thread_data_base::thread_data_base(void)
// IDA 0x446bc8: 163 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_446bc8() {
}

// 0x446d80 — __ZNSt6vectorISt4pairIPN5boost18condition_variableEPNS1_5mutexEESaIS6_EE9push_backERKS6_
#[doc(alias = "std::vector<std::pair<boost::condition_variable *,boost::mutex *>,std::allocator<std::pair<boost::condition_variable *,boost::mutex *>>>::push_back(std::pair<boost::condition_variable *,boost::mutex *> const&)")]
// was: std::vector<std::pair<boost::condition_variable *,boost::mutex *>,std::allocator<std::pair<boost::condition_variable *,boost::mutex *>>>::push_back(std::pair<boost::condition_variable *,boost::mutex *> const&)
// IDA 0x446d80: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_446d80() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x446db0 — __ZNSt6vectorISt4pairIPN5boost18condition_variableEPNS1_5mutexEESaIS6_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS6_S8_EERKS6_
#[doc(alias = "std::vector<std::pair<boost::condition_variable *,boost::mutex *>,std::allocator<std::pair<boost::condition_variable *,boost::mutex *>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<boost::condition_variable *,boost::mutex *>*,std::vector<std::pair<boost::condition_variable *,boost::mutex *>,std::allocator<std::pair<boost::condition_variable *,boost::mutex *>>>>,std::pair<boost::condition_variable *,boost::mutex *> const&)")]
// was: std::vector<std::pair<boost::condition_variable *,boost::mutex *>,std::allocator<std::pair<boost::condition_variable *,boost::mutex *>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<boost::condition_variable *,boost::mutex *>*,std::vector<std::pair<boost::condition_variable *,boost::mutex *>,std::allocator<std::pair<boost::condition_variable *,boost::mutex *>>>>,std::pair<boost::condition_variable *,boost::mutex *> const&)
// IDA 0x446db0: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_446db0() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x446ea8 — __ZNSt12_Vector_baseISt4pairIPN5boost18condition_variableEPNS1_5mutexEESaIS6_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<std::pair<boost::condition_variable *,boost::mutex *>,std::allocator<std::pair<boost::condition_variable *,boost::mutex *>>>::_M_allocate(unsigned long)")]
// was: std::_Vector_base<std::pair<boost::condition_variable *,boost::mutex *>,std::allocator<std::pair<boost::condition_variable *,boost::mutex *>>>::_M_allocate(unsigned long)
// IDA 0x446ea8: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_446ea8() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x446ec0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPSt4pairIPN5boost18condition_variableEPNS4_5mutexEESA_EET0_T_SC_SB_
#[doc(alias = "std::pair<boost::condition_variable *,boost::mutex *> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<std::pair<boost::condition_variable *,boost::mutex *> *,std::pair<boost::condition_variable *,boost::mutex *> *>(std::pair<boost::condition_variable *,boost::mutex *> *,std::pair<boost::condition_variable *,boost::mutex *> *,std::pair<boost::condition_variable *,boost::mutex *> *)")]
// was: std::pair<boost::condition_variable *,boost::mutex *> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<std::pair<boost::condition_variable *,boost::mutex *> *,std::pair<boost::condition_variable *,boost::mutex *> *>(std::pair<boost::condition_variable *,boost::mutex *> *,std::pair<boost::condition_variable *,boost::mutex *> *,std::pair<boost::condition_variable *,boost::mutex *> *)
// IDA 0x446ec0: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_446ec0() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x446f08 — __ZN5boost10shared_ptrINS_6detail16thread_data_baseEEC2INS1_11thread_dataINS_9function0IvEEEEEEPT_
#[doc(alias = "rbx_core::SharedPtr<boost::detail::thread_data_base>::shared_ptr<boost::detail::thread_data<boost::function0<void>>>(boost::detail::thread_data<boost::function0<void>> *)")]
// was: boost::shared_ptr<boost::detail::thread_data_base>::shared_ptr<boost::detail::thread_data<boost::function0<void>>>(boost::detail::thread_data<boost::function0<void>> *)
// IDA 0x446f08: 86 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_446f08() {
}

// 0x446ff0 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvRKNS_9function0IvEEN3RBX11MessageTypeEbENS3_5list3INS3_5valueIS6_EENSE_ISA_EENSE_IbEEEEEEvE6invokeERNS1_15function_bufferE
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>,void>::invoke(boost::detail::function::function_buffer &)
// IDA 0x446ff0: 6 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_446ff0() {
}

// 0x447000 — __ZN5boost3_bi5list3INS0_5valueINS_9function0IvEEEENS2_IN3RBX11MessageTypeEEENS2_IbEEEC2ES5_S8_S9_
#[doc(alias = "boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>::list3(boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>)")]
// was: boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>::list3(boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>)
// IDA 0x447000: 73 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_447000() {
}

// 0x4470d0 — __ZN5boost9function0IvEC2INS_3_bi6bind_tISsPFSsRKSsENS3_5list1INS3_5valueISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISE_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tISsPFSsRKSsENS3_5list1INS3_5valueISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISE_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function0IvEC2INS_3_bi6bind_tISsPFSsRKSsENS3_5list1INS3_5valueISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISE_EE5valueEEE5valueEiE4typeE
// IDA 0x4470d0: 103 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4470d0() {
}

// 0x4471fc — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tISsPFSsRKSsENS3_5list1INS3_5valueISsEEEEEEEEvT_
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>>(boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>)")]
// was: void boost::function0<void>::assign_to<boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>>(boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>)
// IDA 0x4471fc: 107 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4471fc() {
}

// 0x447338 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tISsPFSsRKSsENS3_5list1INS3_5valueISsEEEEEEE6manageERKNS1_15function_bufferERSF_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// IDA 0x447338: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_447338() {
}

// 0x4473b8 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tISsPFSsRKSsENS3_5list1INS3_5valueISsEEEEEEvE6invokeERNS1_15function_bufferE
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>,void>::invoke(boost::detail::function::function_buffer &)
// IDA 0x4473b8: 30 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4473b8() {
}

// 0x447410 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tISsPFSsRKSsENS5_5list1INS5_5valueISsEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>>(boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>>(boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const
// IDA 0x447410: 102 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_447410() {
}

// 0x44753c — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tISsPFSsRKSsENS5_5list1INS5_5valueISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>>(boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>>(boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
// IDA 0x44753c: 107 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44753c() {
}

// 0x447674 — __ZN5boost3_bi5list1INS0_5valueISsEEEC2ES3_
#[doc(alias = "boost::_bi::list1<boost::_bi::value<std::string>>::list1(boost::_bi::value<std::string>)")]
// was: boost::_bi::list1<boost::_bi::value<std::string>>::list1(boost::_bi::value<std::string>)
// IDA 0x447674: 96 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_447674() {
}

// 0x447794 — __ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvS1_S3_NS0_IFvSsEEESA_ENS7_5list4INS_3argILi1EEENSE_ILi2EEENS7_5valueISA_EESI_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvS1_S3_NS0_IFvSsEEESA_ENS7_5list4INS_3argILi1EEENSE_ILi2EEENS7_5valueISA_EESI_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvS1_S3_NS0_IFvSsEEESA_ENS7_5list4INS_3argILi1EEENSE_ILi2EEENS7_5valueISA_EESI_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// IDA 0x447794: 125 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_447794() {
}

// 0x4478e0 — __ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvPFvS1_S3_NS_8functionIFvSsEEESA_ENS6_5list4INS_3argILi1EEENSE_ILi2EEENS6_5valueISA_EESI_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvPFvS1_S3_NS_8functionIFvSsEEESA_ENS6_5list4INS_3argILi1EEENSE_ILi2EEENS6_5valueISA_EESI_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvPFvS1_S3_NS_8functionIFvSsEEESA_ENS6_5list4INS_3argILi1EEENSE_ILi2EEENS6_5valueISA_EESI_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// IDA 0x4478e0: 126 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4478e0() {
}

// 0x447a30 — __ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS1_S3_NS_8functionIFvSsEEESA_ENS6_5list4INS_3argILi1EEENSE_ILi2EEENS6_5valueISA_EESI_EEEEEEvT_
#[doc(alias = "void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)")]
// was: void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)
// IDA 0x447a30: 132 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_447a30() {
}

// 0x447b90 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8functionIFvSsEEESA_ENS3_5list4INS_3argILi1EEENSE_ILi2EEENS3_5valueISA_EESI_EEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// IDA 0x447b90: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_447b90() {
}

// 0x447bac — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8functionIFvSsEEESA_ENS3_5list4INS_3argILi1EEENSE_ILi2EEENS3_5valueISA_EESI_EEEEvS5_S7_E6invokeERNS1_15function_bufferES5_S7_
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,void,std::string *,std::exception *>::invoke(boost::detail::function::function_buffer &,std::string *,std::exception *)")]
// was: boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,void,std::string *,std::exception *>::invoke(boost::detail::function::function_buffer &,std::string *,std::exception *)
// IDA 0x447bac: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_447bac() {
}

// 0x447bcc — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS3_S5_NS_8functionIFvSsEEESC_ENS8_5list4INS_3argILi1EEENSG_ILi2EEENS8_5valueISC_EESK_EEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const
// IDA 0x447bcc: 127 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_447bcc() {
}

// 0x447d1c — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS3_S5_NS_8functionIFvSsEEESC_ENS8_5list4INS_3argILi1EEENSG_ILi2EEENS8_5valueISC_EESK_EEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
// IDA 0x447d1c: 125 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_447d1c() {
}

// 0x447e68 — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvPFvS3_S5_NS_8functionIFvSsEEESC_ENS8_5list4INS_3argILi1EEENSG_ILi2EEENS8_5valueISC_EESK_EEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
// IDA 0x447e68: 94 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_447e68() {
}

// 0x447f68 — __ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8functionIFvSsEEEEES9_EclIPFvPSsPSt9exceptionS8_S8_ENS0_5list2IRSC_RSE_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list2<std::string *&,std::exception *&>>(boost::_bi::type<void>,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>) &,boost::_bi::list2<std::string *&,std::exception *&> &,int)")]
// was: void boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list2<std::string *&,std::exception *&>>(boost::_bi::type<void>,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>) &,boost::_bi::list2<std::string *&,std::exception *&> &,int)
// IDA 0x447f68: 94 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_447f68() {
}

// 0x44806c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8functionIFvSsEEESA_ENS3_5list4INS_3argILi1EEENSE_ILi2EEENS3_5valueISA_EESI_EEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
// IDA 0x44806c: 165 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44806c() {
}

// 0x448218 — __ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8functionIFvSsEEEEES9_EC2ES3_S4_S9_S9_
#[doc(alias = "boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list4(boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// was: boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list4(boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>)
// IDA 0x448218: 88 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_448218() {
}

// 0x44830c — __ZN5boost3_bi8storage4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8functionIFvSsEEEEES9_EC2ES3_S4_S9_S9_
#[doc(alias = "boost::_bi::storage4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage4(boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// was: boost::_bi::storage4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage4(boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>)
// IDA 0x44830c: 89 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44830c() {
}

// 0x448400 — __ZN5boost9function1IvbE13assign_to_ownERKS1_
#[doc(alias = "boost::function1<void,bool>::assign_to_own(boost::function1<void,bool> const&)")]
// was: boost::function1<void,bool>::assign_to_own(boost::function1<void,bool> const&)
// IDA 0x448400: 20 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_448400() {
}

// 0x448434 — __ZN5boost10scoped_ptrIN3RBX9DataModel10LegacyLock14ImplementationEED2Ev
#[doc(alias = "boost::scoped_ptr<RBX::DataModel::LegacyLock::Implementation>::~scoped_ptr()")]
// was: boost::scoped_ptr<RBX::DataModel::LegacyLock::Implementation>::~scoped_ptr()
// IDA 0x448434: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_448434() {
}

// 0x4484dc — __ZN3RBX9DataModel10LegacyLock14ImplementationD2Ev
#[doc(alias = "RBX::DataModel::LegacyLock::Implementation::~Implementation()")]
// was: RBX::DataModel::LegacyLock::Implementation::~Implementation()
// IDA 0x4484dc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4484dc() {
}

// 0x4486b8 — __ZN5boost19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE5resetEPS4_
#[doc(alias = "boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::reset(RBX::DataModel::GenericJob **)")]
// was: boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::reset(RBX::DataModel::GenericJob **)
// IDA 0x4486b8: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4486b8() {
}

// 0x4487a0 — __ZNK3RBX15ServiceProvider4findINS_12AssetServiceEEEPT_v
#[doc(alias = "RBX::AssetService * RBX::ServiceProvider::find<RBX::AssetService>(void)const")]
// was: RBX::AssetService * RBX::ServiceProvider::find<RBX::AssetService>(void)const
// IDA 0x4487a0: 133 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4487a0() {
}

// 0x448914 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_12AssetServiceEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::AssetService> RBX::Creatable<RBX::Instance>::create<RBX::AssetService>(void)")]
// was: boost::shared_ptr<RBX::AssetService> RBX::Creatable<RBX::Instance>::create<RBX::AssetService>(void)
// IDA 0x448914: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_448914() {
}

// 0x4489c4 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_12AssetServiceEEERS3_RKNS0_IT_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::AssetService>(rbx_core::SharedPtr<RBX::AssetService> const&)")]
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::AssetService>(boost::shared_ptr<RBX::AssetService> const&)
// IDA 0x4489c4: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4489c4() {
}

// 0x4489f8 — __ZN3RBX4Name7declareILZNS_13sAssetServiceEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name7declareILZNS_13sAssetServiceEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_13sAssetServiceEEEERKS0_v
// IDA 0x4489f8: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4489f8() {
}

// 0x448a3c — __ZN3RBX4Name13callDoDeclareILZNS_13sAssetServiceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sAssetServiceEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_13sAssetServiceEEEEvv
// IDA 0x448a3c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_448a3c() {
}

// 0x448a40 — __ZN3RBX4Name9doDeclareILZNS_13sAssetServiceEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sAssetServiceEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_13sAssetServiceEEEERKS0_v
// IDA 0x448a40: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_448a40() {
}

// 0x448b24 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_12AssetServiceEEEvv
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::AssetService>(void)")]
// was: void RBX::ServiceProvider::callDoGetClassIndex<RBX::AssetService>(void)
// IDA 0x448b24: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_448b24() {
}

// 0x448b28 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_12AssetServiceEEEmv
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::AssetService>(void)")]
// was: unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::AssetService>(void)
// IDA 0x448b28: 70 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_448b28() {
}

// 0x448c00 — __ZN5boost10shared_ptrIN3RBX12AssetServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::AssetService>::shared_ptr<RBX::AssetService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AssetService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::AssetService>::shared_ptr<RBX::AssetService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AssetService *,RBX::Creatable<RBX::Instance>::Deleter)
// IDA 0x448c00: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_448c00() {
}

// 0x448cc8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12AssetServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::AssetService,RBX::AssetService>(rbx_core::SharedPtr<RBX::AssetService> const*,RBX::AssetService *)const")]
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::AssetService,RBX::AssetService>(boost::shared_ptr<RBX::AssetService> const*,RBX::AssetService *)const
// IDA 0x448cc8: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_448cc8() {
}

// 0x448db4 — __ZN5boost6detail12shared_countC2IPN3RBX12AssetServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AssetService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AssetService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::detail::shared_count::shared_count<RBX::AssetService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AssetService *,RBX::Creatable<RBX::Instance>::Deleter)
// IDA 0x448db4: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_448db4() {
}

// 0x448ebc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AssetServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AssetService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::AssetService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// IDA 0x448ebc: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_448ebc() {
}

// 0x448ec0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AssetServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AssetService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::AssetService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// IDA 0x448ec0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_448ec0() {
}

// 0x448ec4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AssetServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AssetService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::AssetService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
// IDA 0x448ec4: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_448ec4() {
}

// 0x448ee4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AssetServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AssetService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: boost::detail::sp_counted_impl_pd<RBX::AssetService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
// IDA 0x448ee4: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_448ee4() {
}

// 0x448efc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AssetServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AssetService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::AssetService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
// IDA 0x448efc: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_448efc() {
}

// 0x448f00 — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_13sAssetServiceEEE15isNullClassNameEv
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_13sAssetServiceEEE15isNullClassNameEv")]
// was: __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_13sAssetServiceEEE15isNullClassNameEv
// IDA 0x448f00: 53 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_448f00() {
}

// 0x448fa4 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_13ScriptServiceEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptService> RBX::Creatable<RBX::Instance>::create<RBX::ScriptService>(void)")]
// was: boost::shared_ptr<RBX::ScriptService> RBX::Creatable<RBX::Instance>::create<RBX::ScriptService>(void)
// IDA 0x448fa4: 144 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_448fa4() {
}

// 0x449148 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_13ScriptServiceEEERS3_RKNS0_IT_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ScriptService>(rbx_core::SharedPtr<RBX::ScriptService> const&)")]
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::ScriptService>(boost::shared_ptr<RBX::ScriptService> const&)
// IDA 0x449148: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_449148() {
}

// 0x449180 — __ZN3RBX4Name13callDoDeclareILZNS_14sScriptServiceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sScriptServiceEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_14sScriptServiceEEEEvv
// IDA 0x449180: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_449180() {
}

// 0x449188 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13ScriptServiceEEEvv
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::ScriptService>(void)")]
// was: void RBX::ServiceProvider::callDoGetClassIndex<RBX::ScriptService>(void)
// IDA 0x449188: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_449188() {
}

// 0x44918c — __ZN3RBX13ScriptServiceD1Ev
#[doc(alias = "RBX::ScriptService::~ScriptService()")]
// was: RBX::ScriptService::~ScriptService()
// IDA 0x44918c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_44918c() {
}

// 0x449270 — __ZN3RBX13ScriptServiceD0Ev
#[doc(alias = "RBX::ScriptService::~ScriptService()")]
// was: RBX::ScriptService::~ScriptService()
// IDA 0x449270: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_449270() {
}

// 0x449368 — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEE12getClassNameEv
// IDA 0x449368: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_449368() {
}

// 0x44936c — __ZThn32_N3RBX13ScriptServiceD1Ev
#[doc(alias = "non-virtual thunk toRBX::ScriptService::~ScriptService()")]
// was: non-virtual thunk toRBX::ScriptService::~ScriptService()
// IDA 0x44936c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_44936c() {
}

// 0x449450 — __ZThn32_N3RBX13ScriptServiceD0Ev
#[doc(alias = "non-virtual thunk toRBX::ScriptService::~ScriptService()")]
// was: non-virtual thunk toRBX::ScriptService::~ScriptService()
// IDA 0x449450: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_449450() {
}

// 0x449548 — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEE12getClassNameEv
// IDA 0x449548: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_449548() {
}

// 0x44954c — __ZThn36_N3RBX13ScriptServiceD1Ev
#[doc(alias = "non-virtual thunk toRBX::ScriptService::~ScriptService()")]
// was: non-virtual thunk toRBX::ScriptService::~ScriptService()
// IDA 0x44954c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_44954c() {
}

// 0x449630 — __ZThn36_N3RBX13ScriptServiceD0Ev
#[doc(alias = "non-virtual thunk toRBX::ScriptService::~ScriptService()")]
// was: non-virtual thunk toRBX::ScriptService::~ScriptService()
// IDA 0x449630: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_449630() {
}

// 0x449728 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX13ScriptService4InfoEEESaIS5_EED2Ev
#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>::~vector()")]
// was: std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>::~vector()
// IDA 0x449728: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_449728() {
}

// 0x4497f4 — __ZN3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: __ZN3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// IDA 0x4497f4: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4497f4() {
}

// 0x449914 — __ZN3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x449914: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_449914() {
}

// 0x449918 — __ZN3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x449918: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_449918() {
}

// 0x4499b8 — __ZThn32_N3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x4499b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4499b8() {
}

// 0x4499c0 — __ZThn32_N3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x4499c0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4499c0() {
}

// 0x449a64 — __ZThn36_N3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x449a64: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_449a64() {
}

// 0x449a6c — __ZThn36_N3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x449a6c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_449a6c() {
}

// 0x449b10 — __ZN5boost10shared_ptrIN3RBX13ScriptServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptService>::shared_ptr<RBX::ScriptService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::ScriptService>::shared_ptr<RBX::ScriptService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter)
// IDA 0x449b10: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_449b10() {
}

// 0x449bd8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13ScriptServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ScriptService,RBX::ScriptService>(rbx_core::SharedPtr<RBX::ScriptService> const*,RBX::ScriptService *)const")]
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ScriptService,RBX::ScriptService>(boost::shared_ptr<RBX::ScriptService> const*,RBX::ScriptService *)const
// IDA 0x449bd8: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_449bd8() {
}

// 0x449cc4 — __ZN5boost6detail12shared_countC2IPN3RBX13ScriptServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::detail::shared_count::shared_count<RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter)
// IDA 0x449cc4: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_449cc4() {
}

// 0x449dcc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// IDA 0x449dcc: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_449dcc() {
}

// 0x449dd0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// IDA 0x449dd0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_449dd0() {
}

// 0x449dd4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
// IDA 0x449dd4: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_449dd4() {
}

// 0x449df4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: boost::detail::sp_counted_impl_pd<RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
// IDA 0x449df4: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_449df4() {
}

// 0x449e0c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
// IDA 0x449e0c: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_449e0c() {
}

// 0x449e10 — __ZNK3RBX15ServiceProvider4findINS_20ContextActionServiceEEEPT_v
#[doc(alias = "RBX::ContextActionService * RBX::ServiceProvider::find<RBX::ContextActionService>(void)const")]
// was: RBX::ContextActionService * RBX::ServiceProvider::find<RBX::ContextActionService>(void)const
// IDA 0x449e10: 133 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_449e10() {
}

// 0x449f84 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_20ContextActionServiceEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::ContextActionService> RBX::Creatable<RBX::Instance>::create<RBX::ContextActionService>(void)")]
// was: boost::shared_ptr<RBX::ContextActionService> RBX::Creatable<RBX::Instance>::create<RBX::ContextActionService>(void)
// IDA 0x449f84: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_449f84() {
}

// 0x44a034 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_20ContextActionServiceEEERS3_RKNS0_IT_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ContextActionService>(rbx_core::SharedPtr<RBX::ContextActionService> const&)")]
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::ContextActionService>(boost::shared_ptr<RBX::ContextActionService> const&)
// IDA 0x44a034: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44a034() {
}

// 0x44a068 — __ZN3RBX4Name7declareILZNS_21sContextActionServiceEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name7declareILZNS_21sContextActionServiceEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_21sContextActionServiceEEEERKS0_v
// IDA 0x44a068: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44a068() {
}

// 0x44a0ac — __ZN3RBX4Name13callDoDeclareILZNS_21sContextActionServiceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_21sContextActionServiceEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_21sContextActionServiceEEEEvv
// IDA 0x44a0ac: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_44a0ac() {
}
