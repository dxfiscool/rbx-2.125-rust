//! datamodel -- generated_watchdog_dm_w12a -- 120 stubs (watchdog w12a datamodel gap-filler)
//! Source: ida/export.json (85545 funcs) stub-EA global dedup, EA-sorted asc
//! Filter: strict RBX::Instance|DataModel|Workspace|Part uncovered = 0 -> fallback next 120 uncovered containing RBX:: (broad)
//! Each stub preserves IDA ea + mangled + demangled for rg. Uses rbx_core::SharedPtr not boost::shared_ptr.
//! Range: 0xf43834..0xf46174 | global 84896 stub-EAs skipped | datamodel stub-EAs 34310 before batch | broad uncovered 1413 -> 1293 after

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xf43834 -- j___ZN5boost10shared_ptrIN3RBX19IAdornableCollectorEEaSERKS3_
// type: int()
// was: boost::shared_ptr<RBX::IAdornableCollector>::operator=(boost::shared_ptr<RBX::IAdornableCollector> const&)
#[doc(alias = "rbx_core::SharedPtr<RBX::IAdornableCollector>::operator=(rbx_core::SharedPtr<RBX::IAdornableCollector> const&)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX19IAdornableCollectorEEaSERKS3_")]
pub fn stub_0xf43834() -> ! {
    todo!("0xf43834 rbx_core::SharedPtr<RBX::IAdornableCollector>::operator=(rbx_core::SharedPtr<RBX::IAdornableCollector> const&)")
}

// 0xf43894 -- j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX5JointEEE4slotEEaSEPS9_
// type: int()
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Joint *)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Joint *)>::slot*)")]
#[doc(alias = "j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX5JointEEE4slotEEaSEPS9_")]
pub fn stub_0xf43894() -> ! {
    todo!("0xf43894 boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Joint *)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Joint *)>::slot*)")
}

// 0xf438a4 -- j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX5JointEEE4slotEEaSERKSA_
// type: int()
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Joint *)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Joint *)>::slot> const&)")]
#[doc(alias = "j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX5JointEEE4slotEEaSERKSA_")]
pub fn stub_0xf438a4() -> ! {
    todo!("0xf438a4 boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Joint *)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Joint *)>::slot> const&)")
}

// 0xf43f54 -- j___ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEE23removeLeastRecentlyUsedEv
// type: int()
// was: RBX::LRUCache<std::string,boost::shared_ptr<RBX::KeyframeSequence>>::removeLeastRecentlyUsed(void)
#[doc(alias = "RBX::LRUCache<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>>::removeLeastRecentlyUsed(void)")]
#[doc(alias = "j___ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEE23removeLeastRecentlyUsedEv")]
pub fn stub_0xf43f54() -> ! {
    todo!("0xf43f54 RBX::LRUCache<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>>::removeLeastRecentlyUsed(void)")
}

// 0xf43f64 -- j___ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEE6insertERKSsRKS4_m
// type: int()
// was: RBX::LRUCache<std::string,boost::shared_ptr<RBX::KeyframeSequence>>::insert(std::string const&,boost::shared_ptr<RBX::KeyframeSequence> const&,unsigned long)
#[doc(alias = "RBX::LRUCache<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>>::insert(std::string const&,rbx_core::SharedPtr<RBX::KeyframeSequence> const&,unsigned long)")]
#[doc(alias = "j___ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEE6insertERKSsRKS4_m")]
pub fn stub_0xf43f64() -> ! {
    todo!("0xf43f64 RBX::LRUCache<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>>::insert(std::string const&,rbx_core::SharedPtr<RBX::KeyframeSequence> const&,unsigned long)")
}

// 0xf43f74 -- j___ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEE6removeERKSs
// type: int()
// was: RBX::LRUCache<std::string,boost::shared_ptr<RBX::KeyframeSequence>>::remove(std::string const&)
#[doc(alias = "RBX::LRUCache<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>>::remove(std::string const&)")]
#[doc(alias = "j___ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEE6removeERKSs")]
pub fn stub_0xf43f74() -> ! {
    todo!("0xf43f74 RBX::LRUCache<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>>::remove(std::string const&)")
}

// 0xf43f84 -- j___ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEEC2Ev
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
// was: RBX::LRUCache<std::string,boost::shared_ptr<RBX::KeyframeSequence>>::LRUCache(void)
#[doc(alias = "RBX::LRUCache<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>>::LRUCache(void)")]
#[doc(alias = "j___ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEEC2Ev")]
pub fn stub_0xf43f84() -> ! {
    todo!("0xf43f84 RBX::LRUCache<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>>::LRUCache(void)")
}

// 0xf43f94 -- j___ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEED2Ev
// type: int __fastcall(int, int, int, int, int, int)
// was: RBX::LRUCache<std::string,boost::shared_ptr<RBX::KeyframeSequence>>::~LRUCache()
#[doc(alias = "RBX::LRUCache<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>>::~LRUCache()")]
#[doc(alias = "j___ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEED2Ev")]
pub fn stub_0xf43f94() -> ! {
    todo!("0xf43f94 RBX::LRUCache<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>>::~LRUCache()")
}

// 0xf43fb4 -- j___ZN3RBX9weak_fromINS_24KeyframeSequenceProviderEEEN5boost8weak_ptrIT_EEPS4_
// type: int()
#[doc(alias = "boost::weak_ptr<RBX::KeyframeSequenceProvider> RBX::weak_from<RBX::KeyframeSequenceProvider>(RBX::KeyframeSequenceProvider*)")]
#[doc(alias = "j___ZN3RBX9weak_fromINS_24KeyframeSequenceProviderEEEN5boost8weak_ptrIT_EEPS4_")]
pub fn stub_0xf43fb4() -> ! {
    todo!("0xf43fb4 boost::weak_ptr<RBX::KeyframeSequenceProvider> RBX::weak_from<RBX::KeyframeSequenceProvider>(RBX::KeyframeSequenceProvider*)")
}

// 0xf43fe4 -- j___ZN5boost10shared_ptrIN3RBX16KeyframeSequenceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// type: int()
// was: boost::shared_ptr<RBX::KeyframeSequence>::shared_ptr<RBX::KeyframeSequence>(boost::weak_ptr<RBX::KeyframeSequence> const&,boost::detail::sp_nothrow_tag)
#[doc(alias = "rbx_core::SharedPtr<RBX::KeyframeSequence>::shared_ptr<RBX::KeyframeSequence>(boost::weak_ptr<RBX::KeyframeSequence> const&,boost::detail::sp_nothrow_tag)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX16KeyframeSequenceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")]
pub fn stub_0xf43fe4() -> ! {
    todo!("0xf43fe4 rbx_core::SharedPtr<RBX::KeyframeSequence>::shared_ptr<RBX::KeyframeSequence>(boost::weak_ptr<RBX::KeyframeSequence> const&,boost::detail::sp_nothrow_tag)")
}

// 0xf44004 -- j___ZN5boost10shared_ptrIN3RBX16KeyframeSequenceEEaSERKS3_
// type: int()
// was: boost::shared_ptr<RBX::KeyframeSequence>::operator=(boost::shared_ptr<RBX::KeyframeSequence> const&)
#[doc(alias = "rbx_core::SharedPtr<RBX::KeyframeSequence>::operator=(rbx_core::SharedPtr<RBX::KeyframeSequence> const&)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX16KeyframeSequenceEEaSERKS3_")]
pub fn stub_0xf44004() -> ! {
    todo!("0xf44004 rbx_core::SharedPtr<RBX::KeyframeSequence>::operator=(rbx_core::SharedPtr<RBX::KeyframeSequence> const&)")
}

// 0xf44014 -- j___ZN5boost10shared_ptrIN3RBX24KeyframeSequenceProviderEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// type: int()
// was: boost::shared_ptr<RBX::KeyframeSequenceProvider>::shared_ptr<RBX::KeyframeSequenceProvider>(boost::weak_ptr<RBX::KeyframeSequenceProvider> const&,boost::detail::sp_nothrow_tag)
#[doc(alias = "rbx_core::SharedPtr<RBX::KeyframeSequenceProvider>::shared_ptr<RBX::KeyframeSequenceProvider>(boost::weak_ptr<RBX::KeyframeSequenceProvider> const&,boost::detail::sp_nothrow_tag)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX24KeyframeSequenceProviderEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")]
pub fn stub_0xf44014() -> ! {
    todo!("0xf44014 rbx_core::SharedPtr<RBX::KeyframeSequenceProvider>::shared_ptr<RBX::KeyframeSequenceProvider>(boost::weak_ptr<RBX::KeyframeSequenceProvider> const&,boost::detail::sp_nothrow_tag)")
}

// 0xf44024 -- j___ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX16KeyframeSequenceEEEEENS2_INS_10shared_ptrIS5_EEEEEC2ES7_SA_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<boost::shared_ptr<RBX::KeyframeSequence>>>::list2(boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<boost::shared_ptr<RBX::KeyframeSequence>>)
#[doc(alias = "boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>::list2(boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>)")]
#[doc(alias = "j___ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX16KeyframeSequenceEEEEENS2_INS_10shared_ptrIS5_EEEEEC2ES7_SA_")]
pub fn stub_0xf44024() -> ! {
    todo!("0xf44024 boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>::list2(boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>)")
}

// 0xf44044 -- j___ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX24KeyframeSequenceProviderEEEEENS5_INS6_INS7_16KeyframeSequenceEEEEEEC2ES3_S4_SA_SD_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>>::list4(boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>)")]
#[doc(alias = "j___ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX24KeyframeSequenceProviderEEEEENS5_INS6_INS7_16KeyframeSequenceEEEEEEC2ES3_S4_SA_SD_")]
pub fn stub_0xf44044() -> ! {
    todo!("0xf44044 boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>>::list4(boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>)")
}

// 0xf44064 -- j___ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX16KeyframeSequenceEEEEENS2_INS_10shared_ptrIS5_EEEEEC2ES7_SA_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
// was: boost::_bi::storage2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<boost::shared_ptr<RBX::KeyframeSequence>>>::storage2(boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<boost::shared_ptr<RBX::KeyframeSequence>>)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>::storage2(boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>)")]
#[doc(alias = "j___ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX16KeyframeSequenceEEEEENS2_INS_10shared_ptrIS5_EEEEEC2ES7_SA_")]
pub fn stub_0xf44064() -> ! {
    todo!("0xf44064 boost::_bi::storage2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>::storage2(boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>)")
}

// 0xf44074 -- j___ZN5boost3_bi8storage4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX24KeyframeSequenceProviderEEEEENS5_INS6_INS7_16KeyframeSequenceEEEEEEC2ES3_S4_SA_SD_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>>::storage4(boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>)")]
#[doc(alias = "j___ZN5boost3_bi8storage4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX24KeyframeSequenceProviderEEEEENS5_INS6_INS7_16KeyframeSequenceEEEEEEC2ES3_S4_SA_SD_")]
pub fn stub_0xf44074() -> ! {
    todo!("0xf44074 boost::_bi::storage4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>>::storage4(boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>)")
}

// 0xf44094 -- j___ZN5boost4bindIvNS_8weak_ptrIN3RBX16KeyframeSequenceEEENS_10shared_ptrIS3_EES4_S6_EENS_3_bi6bind_tIT_PFS9_T0_T1_ENS7_9list_av_2IT2_T3_E4typeEEESD_SF_SG_
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
// was: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>),boost::_bi::list_av_2<boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>>::type> boost::bind<void,boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>,boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>>(void (*)(boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>),boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list_av_2<boost::weak_ptr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>>::type> boost::bind<void,boost::weak_ptr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>,boost::weak_ptr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>>(void (*)(boost::weak_ptr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::weak_ptr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>)")]
#[doc(alias = "j___ZN5boost4bindIvNS_8weak_ptrIN3RBX16KeyframeSequenceEEENS_10shared_ptrIS3_EES4_S6_EENS_3_bi6bind_tIT_PFS9_T0_T1_ENS7_9list_av_2IT2_T3_E4typeEEESD_SF_SG_")]
pub fn stub_0xf44094() -> ! {
    todo!("0xf44094 boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list_av_2<boost::weak_ptr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>>::type> boost::bind<void,boost::weak_ptr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>,boost::weak_ptr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>>(void (*)(boost::weak_ptr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::weak_ptr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>)")
}

// 0xf440c4 -- j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16KeyframeSequenceEEENS_10shared_ptrIS7_EEENS3_5list2INS3_5valueIS8_EENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<boost::shared_ptr<RBX::KeyframeSequence>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16KeyframeSequenceEEENS_10shared_ptrIS7_EEENS3_5list2INS3_5valueIS8_EENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_0xf440c4() -> ! {
    todo!("0xf440c4 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0xf44134 -- j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISE_EESO_
// type: int()
// was: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>> *)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>> *)")]
#[doc(alias = "j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISE_EESO_")]
pub fn stub_0xf44134() -> ! {
    todo!("0xf44134 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>> *)")
}

// 0xf44144 -- j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm
// type: int()
// was: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)")]
#[doc(alias = "j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm")]
pub fn stub_0xf44144() -> ! {
    todo!("0xf44144 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)")
}

// 0xf44154 -- j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE12emplace_implINS1_13emplace_args1ISE_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISE_EEEEbERS5_RKT_
// type: int __fastcall(int, int, int, int, int, int, int, int, int)
// was: std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>>(std::string const&,boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>> const&)
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>>(std::string const&,boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>> const&)")]
#[doc(alias = "j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE12emplace_implINS1_13emplace_args1ISE_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISE_EEEEbERS5_RKT_")]
pub fn stub_0xf44154() -> ! {
    todo!("0xf44154 std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>>(std::string const&,boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>> const&)")
}

// 0xf44164 -- j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISK_EEPNS1_10ptr_bucketE
// type: int()
// was: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISK_EEPNS1_10ptr_bucketE")]
pub fn stub_0xf44164() -> ! {
    todo!("0xf44164 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")
}

// 0xf44174 -- j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEEEEE20construct_with_valueINS1_13emplace_args1ISE_EEEEvRKT_
// type: int()
// was: void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>>(boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>> const&)
#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>>(boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>> const&)")]
#[doc(alias = "j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEEEEE20construct_with_valueINS1_13emplace_args1ISE_EEEEvRKT_")]
pub fn stub_0xf44174() -> ! {
    todo!("0xf44174 void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>>(boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>> const&)")
}

// 0xf44184 -- j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEEEEE9constructEv
// type: int()
// was: boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>>>::construct(void)
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>>>::construct(void)")]
#[doc(alias = "j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEEEEE9constructEv")]
pub fn stub_0xf44184() -> ! {
    todo!("0xf44184 boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>>>::construct(void)")
}

// 0xf44194 -- j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEEEEED2Ev
// type: int()
// was: boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>>>::~node_constructor()
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>>>::~node_constructor()")]
#[doc(alias = "j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEEEEED2Ev")]
pub fn stub_0xf44194() -> ! {
    todo!("0xf44194 boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>>>::~node_constructor()")
}

// 0xf441a4 -- j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE10fix_bucketEmPNS1_10ptr_bucketE
// type: int()
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE10fix_bucketEmPNS1_10ptr_bucketE")]
pub fn stub_0xf441a4() -> ! {
    todo!("0xf441a4 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")
}

// 0xf441b4 -- j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
// type: int()
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE")]
pub fn stub_0xf441b4() -> ! {
    todo!("0xf441b4 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")
}

// 0xf441c4 -- j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
// type: int __fastcall(int, int, int, int, int, int, int, int, int)
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm")]
pub fn stub_0xf441c4() -> ! {
    todo!("0xf441c4 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")
}

// 0xf441d4 -- j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
// type: int()
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv")]
pub fn stub_0xf441d4() -> ! {
    todo!("0xf441d4 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")
}

// 0xf441e4 -- j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
// type: int()
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm")]
pub fn stub_0xf441e4() -> ! {
    todo!("0xf441e4 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")
}

// 0xf441f4 -- j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE5clearEv
// type: int()
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE5clearEv")]
pub fn stub_0xf441f4() -> ! {
    todo!("0xf441f4 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)")
}

// 0xf44204 -- j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSH_RKSJ_RKSaINS1_8ptr_nodeISE_EEE
// type: int()
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>> const&)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>> const&)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSH_RKSJ_RKSaINS1_8ptr_nodeISE_EEE")]
pub fn stub_0xf44204() -> ! {
    todo!("0xf44204 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>> const&)")
}

// 0xf44214 -- j___ZN9__gnu_cxx13new_allocatorISt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEE7destroyEPS8_
// type: int __fastcall(int, int, int, int, int)
// was: __gnu_cxx::new_allocator<std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>>::destroy(std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>*)
#[doc(alias = "__gnu_cxx::new_allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>::destroy(std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>*)")]
#[doc(alias = "j___ZN9__gnu_cxx13new_allocatorISt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEE7destroyEPS8_")]
pub fn stub_0xf44214() -> ! {
    todo!("0xf44214 __gnu_cxx::new_allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>::destroy(std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>*)")
}

// 0xf44224 -- j___ZN9__gnu_cxx13new_allocatorISt4pairISsS1_ImN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEEE7destroyEPS8_
// type: int __fastcall(int, int, int, int, int)
// was: __gnu_cxx::new_allocator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>::destroy(std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>*)
#[doc(alias = "__gnu_cxx::new_allocator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::destroy(std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>*)")]
#[doc(alias = "j___ZN9__gnu_cxx13new_allocatorISt4pairISsS1_ImN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEEE7destroyEPS8_")]
pub fn stub_0xf44224() -> ! {
    todo!("0xf44224 __gnu_cxx::new_allocator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::destroy(std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>*)")
}

// 0xf442b4 -- j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSJ_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISE_EEEEmRKT_RKT0_
// type: int()
// was: boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const
#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
#[doc(alias = "j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSJ_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISE_EEEEmRKT_RKT0_")]
pub fn stub_0xf442b4() -> ! {
    todo!("0xf442b4 boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")
}

// 0xf442c4 -- j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm
// type: int()
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const")]
#[doc(alias = "j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm")]
pub fn stub_0xf442c4() -> ! {
    todo!("0xf442c4 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const")
}

// 0xf442d4 -- j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_
// type: int()
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")]
#[doc(alias = "j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_")]
pub fn stub_0xf442d4() -> ! {
    todo!("0xf442d4 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")
}

// 0xf442e4 -- j___ZNSt10_List_baseISt4pairISsS0_ImN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEESaIS7_EE8_M_clearEv
// type: int()
// was: std::_List_base<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>,std::allocator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>::_M_clear(void)
#[doc(alias = "std::_List_base<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::allocator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>::_M_clear(void)")]
#[doc(alias = "j___ZNSt10_List_baseISt4pairISsS0_ImN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEESaIS7_EE8_M_clearEv")]
pub fn stub_0xf442e4() -> ! {
    todo!("0xf442e4 std::_List_base<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::allocator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>::_M_clear(void)")
}

// 0xf44304 -- j___ZNSt3mapISsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEESt4lessISsESaISt4pairIKSsS4_EEEixERS8_
// type: int()
// was: std::map<std::string,boost::shared_ptr<RBX::KeyframeSequence>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>>>::operator[](std::string const&)
#[doc(alias = "std::map<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::operator[](std::string const&)")]
#[doc(alias = "j___ZNSt3mapISsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEESt4lessISsESaISt4pairIKSsS4_EEEixERS8_")]
pub fn stub_0xf44304() -> ! {
    todo!("0xf44304 std::map<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::operator[](std::string const&)")
}

// 0xf44314 -- j___ZNSt4listISt4pairISsS0_ImN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEESaIS7_EE14_M_create_nodeERKS7_
// type: int __fastcall(int, int, int, int, void *, int)
// was: std::list<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>,std::allocator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>> const&)
#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::allocator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>> const&)")]
#[doc(alias = "j___ZNSt4listISt4pairISsS0_ImN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEESaIS7_EE14_M_create_nodeERKS7_")]
pub fn stub_0xf44314() -> ! {
    todo!("0xf44314 std::list<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::allocator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>> const&)")
}

// 0xf44324 -- j___ZNSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEC2ERS0_RKS5_
// type: int __fastcall(int, int, int, int, int)
// was: std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>::pair(std::string const&,boost::shared_ptr<RBX::KeyframeSequence> const&)
#[doc(alias = "std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>::pair(std::string const&,rbx_core::SharedPtr<RBX::KeyframeSequence> const&)")]
#[doc(alias = "j___ZNSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEC2ERS0_RKS5_")]
pub fn stub_0xf44324() -> ! {
    todo!("0xf44324 std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>::pair(std::string const&,rbx_core::SharedPtr<RBX::KeyframeSequence> const&)")
}

// 0xf44334 -- j___ZNSt4pairISsS_ImN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEEC2ERKSsRKS5_
// type: 
// was: std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>::pair(std::string const&,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>> const&)
#[doc(alias = "std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>::pair(std::string const&,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>> const&)")]
#[doc(alias = "j___ZNSt4pairISsS_ImN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEEC2ERKSsRKS5_")]
pub fn stub_0xf44334() -> ! {
    todo!("0xf44334 std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>::pair(std::string const&,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>> const&)")
}

// 0xf44344 -- j___ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE11lower_boundERS1_
// type: _DWORD *__fastcall(int, std::string *)
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>>>::lower_bound(std::string const&)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::lower_bound(std::string const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE11lower_boundERS1_")]
pub fn stub_0xf44344() -> ! {
    todo!("0xf44344 std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::lower_bound(std::string const&)")
}

// 0xf44354 -- j___ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE14_M_create_nodeERKS7_
// type: int __fastcall(int, int, int, int, void *, int)
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>>>::_M_create_node(std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>> const&)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::_M_create_node(std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE14_M_create_nodeERKS7_")]
pub fn stub_0xf44354() -> ! {
    todo!("0xf44354 std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::_M_create_node(std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>> const&)")
}

// 0xf44364 -- j___ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE16_M_insert_uniqueERKS7_
// type: int __fastcall(int, int, int)
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>>>::_M_insert_unique(std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>> const&)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::_M_insert_unique(std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE16_M_insert_uniqueERKS7_")]
pub fn stub_0xf44364() -> ! {
    todo!("0xf44364 std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::_M_insert_unique(std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>> const&)")
}

// 0xf44374 -- j___ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
// type: int __fastcall(int, int, int)
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>>,std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>> const&)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_")]
pub fn stub_0xf44374() -> ! {
    todo!("0xf44374 std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>> const&)")
}

// 0xf44384 -- j___ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE4findERS1_
// type: int __fastcall(int, std::string *this)
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>>>::find(std::string const&)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::find(std::string const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE4findERS1_")]
pub fn stub_0xf44384() -> ! {
    todo!("0xf44384 std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::find(std::string const&)")
}

// 0xf44394 -- j___ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// type: int()
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>> *)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")]
pub fn stub_0xf44394() -> ! {
    todo!("0xf44394 std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>> *)")
}

// 0xf443a4 -- j___ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_
// type: int __fastcall(int, int, int, int)
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>> const&)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_")]
pub fn stub_0xf443a4() -> ! {
    todo!("0xf443a4 std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>> const&)")
}

// 0xf443e4 -- j___ZNSt3mapIPKN3RBX4NameENS0_6Legacy17SurfaceConstraintESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int()
#[doc(alias = "std::map<RBX::Name const*,RBX::Legacy::SurfaceConstraint,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "j___ZNSt3mapIPKN3RBX4NameENS0_6Legacy17SurfaceConstraintESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
pub fn stub_0xf443e4() -> ! {
    todo!("0xf443e4 std::map<RBX::Name const*,RBX::Legacy::SurfaceConstraint,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>>::operator[](RBX::Name const* const&)")
}

// 0xf443f4 -- j___ZNSt6vectorIN3RBX6Legacy17SurfaceConstraintESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int()
#[doc(alias = "std::vector<RBX::Legacy::SurfaceConstraint,std::allocator<RBX::Legacy::SurfaceConstraint>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Legacy::SurfaceConstraint*,std::vector<RBX::Legacy::SurfaceConstraint,std::allocator<RBX::Legacy::SurfaceConstraint>>>,RBX::Legacy::SurfaceConstraint const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX6Legacy17SurfaceConstraintESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_0xf443f4() -> ! {
    todo!("0xf443f4 std::vector<RBX::Legacy::SurfaceConstraint,std::allocator<RBX::Legacy::SurfaceConstraint>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Legacy::SurfaceConstraint*,std::vector<RBX::Legacy::SurfaceConstraint,std::allocator<RBX::Legacy::SurfaceConstraint>>>,RBX::Legacy::SurfaceConstraint const&)")
}

// 0xf44404 -- j___ZNSt6vectorIN3RBX6Legacy17SurfaceConstraintESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int()
#[doc(alias = "std::vector<RBX::Legacy::SurfaceConstraint,std::allocator<RBX::Legacy::SurfaceConstraint>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Legacy::SurfaceConstraint*,std::vector<RBX::Legacy::SurfaceConstraint,std::allocator<RBX::Legacy::SurfaceConstraint>>>,unsigned long,RBX::Legacy::SurfaceConstraint const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX6Legacy17SurfaceConstraintESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_0xf44404() -> ! {
    todo!("0xf44404 std::vector<RBX::Legacy::SurfaceConstraint,std::allocator<RBX::Legacy::SurfaceConstraint>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Legacy::SurfaceConstraint*,std::vector<RBX::Legacy::SurfaceConstraint,std::allocator<RBX::Legacy::SurfaceConstraint>>>,unsigned long,RBX::Legacy::SurfaceConstraint const&)")
}

// 0xf44414 -- j___ZNSt6vectorIN3RBX6Legacy17SurfaceConstraintESaIS2_EE6resizeEmS2_
// type: int()
#[doc(alias = "std::vector<RBX::Legacy::SurfaceConstraint,std::allocator<RBX::Legacy::SurfaceConstraint>>::resize(unsigned long,RBX::Legacy::SurfaceConstraint)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX6Legacy17SurfaceConstraintESaIS2_EE6resizeEmS2_")]
pub fn stub_0xf44414() -> ! {
    todo!("0xf44414 std::vector<RBX::Legacy::SurfaceConstraint,std::allocator<RBX::Legacy::SurfaceConstraint>>::resize(unsigned long,RBX::Legacy::SurfaceConstraint)")
}

// 0xf44424 -- j___ZNSt6vectorIN3RBX6Legacy17SurfaceConstraintESaIS2_EE9push_backERKS2_
// type: int()
#[doc(alias = "std::vector<RBX::Legacy::SurfaceConstraint,std::allocator<RBX::Legacy::SurfaceConstraint>>::push_back(RBX::Legacy::SurfaceConstraint const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX6Legacy17SurfaceConstraintESaIS2_EE9push_backERKS2_")]
pub fn stub_0xf44424() -> ! {
    todo!("0xf44424 std::vector<RBX::Legacy::SurfaceConstraint,std::allocator<RBX::Legacy::SurfaceConstraint>>::push_back(RBX::Legacy::SurfaceConstraint const&)")
}

// 0xf44434 -- j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Legacy17SurfaceConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int()
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Legacy17SurfaceConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
pub fn stub_0xf44434() -> ! {
    todo!("0xf44434 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint> const&)")
}

// 0xf44444 -- j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Legacy17SurfaceConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>,std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Legacy17SurfaceConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
pub fn stub_0xf44444() -> ! {
    todo!("0xf44444 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>,std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint> const&)")
}

// 0xf44454 -- j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Legacy17SurfaceConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int()
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Legacy17SurfaceConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
pub fn stub_0xf44454() -> ! {
    todo!("0xf44454 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint> const&)")
}

// 0xf44564 -- j___ZN3RBX11shared_fromINS_3SkyEEEN5boost10shared_ptrIT_EEPS4_
// type: int()
// was: boost::shared_ptr<RBX::Sky> RBX::shared_from<RBX::Sky>(RBX::Sky*)
#[doc(alias = "rbx_core::SharedPtr<RBX::Sky> RBX::shared_from<RBX::Sky>(RBX::Sky*)")]
#[doc(alias = "j___ZN3RBX11shared_fromINS_3SkyEEEN5boost10shared_ptrIT_EEPS4_")]
pub fn stub_0xf44564() -> ! {
    todo!("0xf44564 rbx_core::SharedPtr<RBX::Sky> RBX::shared_from<RBX::Sky>(RBX::Sky*)")
}

// 0xf44594 -- j___ZN5boost10shared_ptrIN3RBX3SkyEEaSERKS3_
// type: int()
// was: boost::shared_ptr<RBX::Sky>::operator=(boost::shared_ptr<RBX::Sky> const&)
#[doc(alias = "rbx_core::SharedPtr<RBX::Sky>::operator=(rbx_core::SharedPtr<RBX::Sky> const&)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX3SkyEEaSERKS3_")]
pub fn stub_0xf44594() -> ! {
    todo!("0xf44594 rbx_core::SharedPtr<RBX::Sky>::operator=(rbx_core::SharedPtr<RBX::Sky> const&)")
}

// 0xf44af4 -- j___ZN10XmlElementC2IfEERKN3RBX4NameET_
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "XmlElement::XmlElement<float>(RBX::Name const&,float)")]
#[doc(alias = "j___ZN10XmlElementC2IfEERKN3RBX4NameET_")]
pub fn stub_0xf44af4() -> ! {
    todo!("0xf44af4 XmlElement::XmlElement<float>(RBX::Name const&,float)")
}

// 0xf44b04 -- j___ZN3G3D5ArrayIPN3RBX9PrimitiveELi10ELm32EE6resizeEib
// type: int __fastcall(int, int, int)
#[doc(alias = "G3D::Array<RBX::Primitive *,10,32ul>::resize(int,bool)")]
#[doc(alias = "j___ZN3G3D5ArrayIPN3RBX9PrimitiveELi10ELm32EE6resizeEib")]
pub fn stub_0xf44b04() -> ! {
    todo!("0xf44b04 G3D::Array<RBX::Primitive *,10,32ul>::resize(int,bool)")
}

// 0xf44b14 -- j___ZN3G3D5ArrayIPN3RBX9PrimitiveELi10ELm32EE7reallocEi
// type: int()
#[doc(alias = "G3D::Array<RBX::Primitive *,10,32ul>::realloc(int)")]
#[doc(alias = "j___ZN3G3D5ArrayIPN3RBX9PrimitiveELi10ELm32EE7reallocEi")]
pub fn stub_0xf44b14() -> ! {
    todo!("0xf44b14 G3D::Array<RBX::Primitive *,10,32ul>::realloc(int)")
}

// 0xf44b24 -- j___ZN3G3D5ArrayIPN3RBX9PrimitiveELi10ELm32EED2Ev
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "G3D::Array<RBX::Primitive *,10,32ul>::~Array()")]
#[doc(alias = "j___ZN3G3D5ArrayIPN3RBX9PrimitiveELi10ELm32EED2Ev")]
pub fn stub_0xf44b24() -> ! {
    todo!("0xf44b24 G3D::Array<RBX::Primitive *,10,32ul>::~Array()")
}

// 0xf44e94 -- j___ZN3RBX11SurfaceData5emptyEv
// type: int __fastcall(RBX::SurfaceData *this)
#[doc(alias = "RBX::SurfaceData::empty(void)")]
#[doc(alias = "j___ZN3RBX11SurfaceData5emptyEv")]
pub fn stub_0xf44e94() -> ! {
    todo!("0xf44e94 RBX::SurfaceData::empty(void)")
}

// 0xf44fa4 -- j___ZN3RBX18InterpolatedCFrameC2Ev
// type: int __fastcall(RBX::InterpolatedCFrame *this)
#[doc(alias = "RBX::InterpolatedCFrame::InterpolatedCFrame(void)")]
#[doc(alias = "j___ZN3RBX18InterpolatedCFrameC2Ev")]
pub fn stub_0xf44fa4() -> ! {
    todo!("0xf44fa4 RBX::InterpolatedCFrame::InterpolatedCFrame(void)")
}

// 0xf44fd4 -- j___ZN3RBX4Body13getMeInParentEv
// type: int __fastcall(RBX::Body *this)
#[doc(alias = "RBX::Body::getMeInParent(void)")]
#[doc(alias = "j___ZN3RBX4Body13getMeInParentEv")]
pub fn stub_0xf44fd4() -> ! {
    todo!("0xf44fd4 RBX::Body::getMeInParent(void)")
}

// 0xf44fe4 -- j___ZN3RBX4Body8updatePVEv
// type: int __fastcall(RBX::Body *this)
#[doc(alias = "RBX::Body::updatePV(void)")]
#[doc(alias = "j___ZN3RBX4Body8updatePVEv")]
pub fn stub_0xf44fe4() -> ! {
    todo!("0xf44fe4 RBX::Body::updatePV(void)")
}

// 0xf45034 -- j___ZN3RBX7Dragger8dragSnapEv
// type: int __fastcall(RBX::Dragger *this)
#[doc(alias = "RBX::Dragger::dragSnap(void)")]
#[doc(alias = "j___ZN3RBX7Dragger8dragSnapEv")]
pub fn stub_0xf45034() -> ! {
    todo!("0xf45034 RBX::Dragger::dragSnap(void)")
}

// 0xf45074 -- j___ZN3RBX7FWValueIPNS_5WorldEE3setERKS2_PNS_5FWRefE
// type: int()
#[doc(alias = "RBX::FWValue<RBX::World *>::set(RBX::World * const&,RBX::FWRef *)")]
#[doc(alias = "j___ZN3RBX7FWValueIPNS_5WorldEE3setERKS2_PNS_5FWRefE")]
pub fn stub_0xf45074() -> ! {
    todo!("0xf45074 RBX::FWValue<RBX::World *>::set(RBX::World * const&,RBX::FWRef *)")
}

// 0xf45084 -- j___ZN3RBX7FWValueIfE3setERKfPNS_5FWRefE
// type: int()
#[doc(alias = "RBX::FWValue<float>::set(float const&,RBX::FWRef *)")]
#[doc(alias = "j___ZN3RBX7FWValueIfE3setERKfPNS_5FWRefE")]
pub fn stub_0xf45084() -> ! {
    todo!("0xf45084 RBX::FWValue<float>::set(float const&,RBX::FWRef *)")
}

// 0xf45094 -- j___ZN3RBX7IMovingD2Ev
// type: void __fastcall(RBX::IMoving *__hidden this)
#[doc(alias = "RBX::IMoving::~IMoving()")]
#[doc(alias = "j___ZN3RBX7IMovingD2Ev")]
pub fn stub_0xf45094() -> ! {
    todo!("0xf45094 RBX::IMoving::~IMoving()")
}

// 0xf45104 -- j___ZN3RBX8Security7Context3ptrEv
// type: int __fastcall(RBX::Security::Context *this)
#[doc(alias = "RBX::Security::Context::ptr(void)")]
#[doc(alias = "j___ZN3RBX8Security7Context3ptrEv")]
pub fn stub_0xf45104() -> ! {
    todo!("0xf45104 RBX::Security::Context::ptr(void)")
}

// 0xf45184 -- j___ZN3rbx13placement_anyIN3RBX7Region3EEaSIN3G3D15CoordinateFrameEEERS3_RKT_
// type: int __fastcall(int, int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<G3D::CoordinateFrame>(G3D::CoordinateFrame const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSIN3G3D15CoordinateFrameEEERS3_RKT_")]
pub fn stub_0xf45184() -> ! {
    todo!("0xf45184 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<G3D::CoordinateFrame>(G3D::CoordinateFrame const&)")
}

// 0xf45194 -- j___ZN3rbx13placement_anyIN3RBX7Region3EEaSIN3G3D6Color3EEERS3_RKT_
// type: int __fastcall(int, int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<G3D::Color3>(G3D::Color3 const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSIN3G3D6Color3EEERS3_RKT_")]
pub fn stub_0xf45194() -> ! {
    todo!("0xf45194 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<G3D::Color3>(G3D::Color3 const&)")
}

// 0xf451c4 -- j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13SystemAddressEEERS3_RKT_
// type: int()
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SystemAddress>(RBX::SystemAddress const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13SystemAddressEEERS3_RKT_")]
pub fn stub_0xf451c4() -> ! {
    todo!("0xf451c4 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SystemAddress>(RBX::SystemAddress const&)")
}

// 0xf451f4 -- j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_8NormalIdEEERS3_RKT_
// type: int()
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::NormalId>(RBX::NormalId const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_8NormalIdEEERS3_RKT_")]
pub fn stub_0xf451f4() -> ! {
    todo!("0xf451f4 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::NormalId>(RBX::NormalId const&)")
}

// 0xf45234 -- j___ZN3rbx14implementation12typed_holderIN3RBX13SystemAddressEE9singletonEv
// type: int()
#[doc(alias = "rbx::implementation::typed_holder<RBX::SystemAddress>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX13SystemAddressEE9singletonEv")]
pub fn stub_0xf45234() -> ! {
    todo!("0xf45234 rbx::implementation::typed_holder<RBX::SystemAddress>::singleton(void)")
}

// 0xf45264 -- j___ZN3rbx14implementation12typed_holderIN3RBX8NormalIdEE9singletonEv
// type: int()
#[doc(alias = "rbx::implementation::typed_holder<RBX::NormalId>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX8NormalIdEE9singletonEv")]
pub fn stub_0xf45264() -> ! {
    todo!("0xf45264 rbx::implementation::typed_holder<RBX::NormalId>::singleton(void)")
}

// 0xf45384 -- j___ZN3rbx8any_castIRKN3G3D15CoordinateFrameEN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(int)
#[doc(alias = "G3D::CoordinateFrame const& rbx::any_cast<G3D::CoordinateFrame const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRKN3G3D15CoordinateFrameEN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0xf45384() -> ! {
    todo!("0xf45384 G3D::CoordinateFrame const& rbx::any_cast<G3D::CoordinateFrame const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0xf45394 -- j___ZN3rbx8any_castIRKN3G3D6Color3EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: int()
#[doc(alias = "G3D::Color3 const& rbx::any_cast<G3D::Color3 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRKN3G3D6Color3EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0xf45394() -> ! {
    todo!("0xf45394 G3D::Color3 const& rbx::any_cast<G3D::Color3 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0xf453c4 -- j___ZN3rbx8any_castIRKN3RBX13SystemAddressENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: 
#[doc(alias = "RBX::SystemAddress const& rbx::any_cast<RBX::SystemAddress const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRKN3RBX13SystemAddressENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0xf453c4() -> ! {
    todo!("0xf453c4 RBX::SystemAddress const& rbx::any_cast<RBX::SystemAddress const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0xf453e4 -- j___ZN3rbx8any_castIRKN3RBX8MaterialENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int()
#[doc(alias = "RBX::Material const& rbx::any_cast<RBX::Material const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRKN3RBX8MaterialENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0xf453e4() -> ! {
    todo!("0xf453e4 RBX::Material const& rbx::any_cast<RBX::Material const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0xf453f4 -- j___ZN3rbx8any_castIRKN3RBX8NormalIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int()
#[doc(alias = "RBX::NormalId const& rbx::any_cast<RBX::NormalId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRKN3RBX8NormalIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0xf453f4() -> ! {
    todo!("0xf453f4 RBX::NormalId const& rbx::any_cast<RBX::NormalId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0xf45474 -- j___ZN5boost10shared_ptrIN3RBX9IFWHolderEEaSERKS3_
// type: int()
// was: boost::shared_ptr<RBX::IFWHolder>::operator=(boost::shared_ptr<RBX::IFWHolder> const&)
#[doc(alias = "rbx_core::SharedPtr<RBX::IFWHolder>::operator=(rbx_core::SharedPtr<RBX::IFWHolder> const&)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX9IFWHolderEEaSERKS3_")]
pub fn stub_0xf45474() -> ! {
    todo!("0xf45474 rbx_core::SharedPtr<RBX::IFWHolder>::operator=(rbx_core::SharedPtr<RBX::IFWHolder> const&)")
}

// 0xf454e4 -- j___ZN5boost14singleton_poolIN3RBX12RevoluteLinkELj208ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
// type: int()
#[doc(alias = "boost::singleton_pool<RBX::RevoluteLink,208u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
#[doc(alias = "j___ZN5boost14singleton_poolIN3RBX12RevoluteLinkELj208ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")]
pub fn stub_0xf454e4() -> ! {
    todo!("0xf454e4 boost::singleton_pool<RBX::RevoluteLink,208u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")
}

// 0xf45514 -- j___ZN5boost14singleton_poolIN3RBX16BallBlockContactELj52ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
// type: int(void)
#[doc(alias = "boost::singleton_pool<RBX::BallBlockContact,52u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
#[doc(alias = "j___ZN5boost14singleton_poolIN3RBX16BallBlockContactELj52ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")]
pub fn stub_0xf45514() -> ! {
    todo!("0xf45514 boost::singleton_pool<RBX::BallBlockContact,52u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")
}

// 0xf45524 -- j___ZN5boost14singleton_poolIN3RBX16GeoPairConnectorELj264ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
// type: int()
#[doc(alias = "boost::singleton_pool<RBX::GeoPairConnector,264u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
#[doc(alias = "j___ZN5boost14singleton_poolIN3RBX16GeoPairConnectorELj264ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")]
pub fn stub_0xf45524() -> ! {
    todo!("0xf45524 boost::singleton_pool<RBX::GeoPairConnector,264u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")
}

// 0xf45534 -- j___ZN5boost14singleton_poolIN3RBX17BallBallConnectorELj272ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
// type: int()
#[doc(alias = "boost::singleton_pool<RBX::BallBallConnector,272u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
#[doc(alias = "j___ZN5boost14singleton_poolIN3RBX17BallBallConnectorELj272ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")]
pub fn stub_0xf45534() -> ! {
    todo!("0xf45534 boost::singleton_pool<RBX::BallBallConnector,272u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")
}

// 0xf45544 -- j___ZN5boost14singleton_poolIN3RBX18BallBlockConnectorELj288ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
// type: int()
#[doc(alias = "boost::singleton_pool<RBX::BallBlockConnector,288u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
#[doc(alias = "j___ZN5boost14singleton_poolIN3RBX18BallBlockConnectorELj288ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")]
pub fn stub_0xf45544() -> ! {
    todo!("0xf45544 boost::singleton_pool<RBX::BallBlockConnector,288u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")
}

// 0xf45574 -- j___ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE7destroyEv
// type: int()
#[doc(alias = "boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::destroy(void)")]
#[doc(alias = "j___ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE7destroyEv")]
pub fn stub_0xf45574() -> ! {
    todo!("0xf45574 boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::destroy(void)")
}

// 0xf45584 -- j___ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE8allocateEm
// type: int()
#[doc(alias = "boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::allocate(unsigned long)")]
#[doc(alias = "j___ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE8allocateEm")]
pub fn stub_0xf45584() -> ! {
    todo!("0xf45584 boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::allocate(unsigned long)")
}

// 0xf45924 -- j___ZNK3RBX5Joint11getNormalIdEi
// type: int __fastcall(RBX::Joint *this, int)
#[doc(alias = "RBX::Joint::getNormalId(int)const")]
#[doc(alias = "j___ZNK3RBX5Joint11getNormalIdEi")]
pub fn stub_0xf45924() -> ! {
    todo!("0xf45924 RBX::Joint::getNormalId(int)const")
}

// 0xf45934 -- j___ZNK3RBX8EdgeList8getOtherEi
// type: int __fastcall(RBX::EdgeList *this, int)
#[doc(alias = "RBX::EdgeList::getOther(int)const")]
#[doc(alias = "j___ZNK3RBX8EdgeList8getOtherEi")]
pub fn stub_0xf45934() -> ! {
    todo!("0xf45934 RBX::EdgeList::getOther(int)const")
}

// 0xf45964 -- j___ZNK3RBX9Primitive15getExtentsWorldEv
// type: int __fastcall(RBX::Primitive *this)
#[doc(alias = "RBX::Primitive::getExtentsWorld(void)const")]
#[doc(alias = "j___ZNK3RBX9Primitive15getExtentsWorldEv")]
pub fn stub_0xf45964() -> ! {
    todo!("0xf45964 RBX::Primitive::getExtentsWorld(void)const")
}

// 0xf45a74 -- j___ZNSt12_Vector_baseIN3RBX8MaterialESaIS1_EE11_M_allocateEm
// type: int()
#[doc(alias = "std::_Vector_base<RBX::Material,std::allocator<RBX::Material>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIN3RBX8MaterialESaIS1_EE11_M_allocateEm")]
pub fn stub_0xf45a74() -> ! {
    todo!("0xf45a74 std::_Vector_base<RBX::Material,std::allocator<RBX::Material>>::_M_allocate(unsigned long)")
}

// 0xf45aa4 -- j___ZNSt12_Vector_baseIPKN3RBX9PrimitiveESaIS3_EE11_M_allocateEm
// type: int()
#[doc(alias = "std::_Vector_base<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIPKN3RBX9PrimitiveESaIS3_EE11_M_allocateEm")]
pub fn stub_0xf45aa4() -> ! {
    todo!("0xf45aa4 std::_Vector_base<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>::_M_allocate(unsigned long)")
}

// 0xf45ac4 -- j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX8MaterialES5_EET0_T_S7_S6_
// type: int()
#[doc(alias = "RBX::Material * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Material *,RBX::Material *>(RBX::Material *,RBX::Material *,RBX::Material *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX8MaterialES5_EET0_T_S7_S6_")]
pub fn stub_0xf45ac4() -> ! {
    todo!("0xf45ac4 RBX::Material * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Material *,RBX::Material *>(RBX::Material *,RBX::Material *,RBX::Material *)")
}

// 0xf45b04 -- j___ZNSt3mapIPKN3RBX4NameENS0_8MaterialESt4lessIS3_ESaISt4pairIKS3_S4_EEEixERS8_
// type: int()
#[doc(alias = "std::map<RBX::Name const*,RBX::Material,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Material>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "j___ZNSt3mapIPKN3RBX4NameENS0_8MaterialESt4lessIS3_ESaISt4pairIKS3_S4_EEEixERS8_")]
pub fn stub_0xf45b04() -> ! {
    todo!("0xf45b04 std::map<RBX::Name const*,RBX::Material,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Material>>>::operator[](RBX::Name const* const&)")
}

// 0xf45b64 -- j___ZNSt6vectorIN3RBX8MaterialESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: int()
#[doc(alias = "std::vector<RBX::Material,std::allocator<RBX::Material>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Material*,std::vector<RBX::Material,std::allocator<RBX::Material>>>,RBX::Material const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX8MaterialESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")]
pub fn stub_0xf45b64() -> ! {
    todo!("0xf45b64 std::vector<RBX::Material,std::allocator<RBX::Material>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Material*,std::vector<RBX::Material,std::allocator<RBX::Material>>>,RBX::Material const&)")
}

// 0xf45b74 -- j___ZNSt6vectorIN3RBX8MaterialESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
// type: int()
#[doc(alias = "std::vector<RBX::Material,std::allocator<RBX::Material>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Material*,std::vector<RBX::Material,std::allocator<RBX::Material>>>,unsigned long,RBX::Material const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX8MaterialESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_")]
pub fn stub_0xf45b74() -> ! {
    todo!("0xf45b74 std::vector<RBX::Material,std::allocator<RBX::Material>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Material*,std::vector<RBX::Material,std::allocator<RBX::Material>>>,unsigned long,RBX::Material const&)")
}

// 0xf45b84 -- j___ZNSt6vectorIN3RBX8MaterialESaIS1_EE6resizeEmS1_
// type: int()
#[doc(alias = "std::vector<RBX::Material,std::allocator<RBX::Material>>::resize(unsigned long,RBX::Material)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX8MaterialESaIS1_EE6resizeEmS1_")]
pub fn stub_0xf45b84() -> ! {
    todo!("0xf45b84 std::vector<RBX::Material,std::allocator<RBX::Material>>::resize(unsigned long,RBX::Material)")
}

// 0xf45b94 -- j___ZNSt6vectorIN3RBX8MaterialESaIS1_EE9push_backERKS1_
// type: int()
#[doc(alias = "std::vector<RBX::Material,std::allocator<RBX::Material>>::push_back(RBX::Material const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX8MaterialESaIS1_EE9push_backERKS1_")]
pub fn stub_0xf45b94() -> ! {
    todo!("0xf45b94 std::vector<RBX::Material,std::allocator<RBX::Material>>::push_back(RBX::Material const&)")
}

// 0xf45c14 -- j___ZNSt6vectorIPKN3RBX9PrimitiveESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Primitive const**,std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>>,RBX::Primitive const* const&)")]
#[doc(alias = "j___ZNSt6vectorIPKN3RBX9PrimitiveESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")]
pub fn stub_0xf45c14() -> ! {
    todo!("0xf45c14 std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Primitive const**,std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>>,RBX::Primitive const* const&)")
}

// 0xf45c24 -- j___ZNSt6vectorIPKN3RBX9PrimitiveESaIS3_EE9push_backERKS3_
// type: int()
#[doc(alias = "std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>::push_back(RBX::Primitive const* const&)")]
#[doc(alias = "j___ZNSt6vectorIPKN3RBX9PrimitiveESaIS3_EE9push_backERKS3_")]
pub fn stub_0xf45c24() -> ! {
    todo!("0xf45c24 std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>::push_back(RBX::Primitive const* const&)")
}

// 0xf45ca4 -- j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8MaterialEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_
// type: int()
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Material>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Material>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Material>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Material> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8MaterialEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_")]
pub fn stub_0xf45ca4() -> ! {
    todo!("0xf45ca4 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Material>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Material>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Material>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Material> const&)")
}

// 0xf45cb4 -- j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8MaterialEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Material>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Material>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Material>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Material>>,std::pair<RBX::Name const* const,RBX::Material> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8MaterialEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_")]
pub fn stub_0xf45cb4() -> ! {
    todo!("0xf45cb4 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Material>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Material>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Material>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Material>>,std::pair<RBX::Name const* const,RBX::Material> const&)")
}

// 0xf45cc4 -- j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8MaterialEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// type: int()
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Material>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Material>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Material>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Material>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8MaterialEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")]
pub fn stub_0xf45cc4() -> ! {
    todo!("0xf45cc4 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Material>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Material>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Material>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Material>> *)")
}

// 0xf45cd4 -- j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8MaterialEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_
// type: int()
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Material>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Material>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Material>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Material> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8MaterialEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_")]
pub fn stub_0xf45cd4() -> ! {
    todo!("0xf45cd4 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Material>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Material>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Material>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Material> const&)")
}

// 0xf45d14 -- j___ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE24safe_static_do_get_mutexEv
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE24safe_static_do_get_mutexEv")]
pub fn stub_0xf45d14() -> ! {
    todo!("0xf45d14 rbx::signals::signal<void ()(RBX::Primitive *)>::safe_static_do_get_mutex(void)")
}

// 0xf45d24 -- j___ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot24safe_static_do_get_mutexEv
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot24safe_static_do_get_mutexEv")]
pub fn stub_0xf45d24() -> ! {
    todo!("0xf45d24 rbx::signals::signal<void ()(RBX::Primitive *)>::slot::safe_static_do_get_mutex(void)")
}

// 0xf45d34 -- j___ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE6insertEPNS6_4slotE
// type: void __fastcall(int *, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::insert(rbx::signals::signal<void ()(RBX::Primitive *)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE6insertEPNS6_4slotE")]
pub fn stub_0xf45d34() -> ! {
    todo!("0xf45d34 rbx::signals::signal<void ()(RBX::Primitive *)>::insert(rbx::signals::signal<void ()(RBX::Primitive *)>::slot *)")
}

// 0xf45d44 -- j___ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE6removeEPNS6_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::remove(rbx::signals::signal<void ()(RBX::Primitive *)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE6removeEPNS6_4slotE")]
pub fn stub_0xf45d44() -> ! {
    todo!("0xf45d44 rbx::signals::signal<void ()(RBX::Primitive *)>::remove(rbx::signals::signal<void ()(RBX::Primitive *)>::slot *)")
}

// 0xf45d54 -- j___ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_14PhysicsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int()
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Primitive *)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>> const&)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_14PhysicsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEEENS0_10connectionERKT_")]
pub fn stub_0xf45d54() -> ! {
    todo!("0xf45d54 rbx::signals::connection rbx::signals::signal<void ()(RBX::Primitive *)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>> const&)")
}

// 0xf45d64 -- j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotEEaSEPS9_
// type: int()
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Primitive *)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Primitive *)>::slot*)")]
#[doc(alias = "j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotEEaSEPS9_")]
pub fn stub_0xf45d64() -> ! {
    todo!("0xf45d64 boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Primitive *)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Primitive *)>::slot*)")
}

// 0xf45d74 -- j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotEEaSERKSA_
// type: int()
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Primitive *)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Primitive *)>::slot> const&)")]
#[doc(alias = "j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotEEaSERKSA_")]
pub fn stub_0xf45d74() -> ! {
    todo!("0xf45d74 boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Primitive *)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Primitive *)>::slot> const&)")
}

// 0xf45d84 -- j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX14PhysicsServiceEPNS4_9PrimitiveEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS7_EEvRT_
// type: int()
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>>::operator()<RBX::Primitive *>(RBX::Primitive * &)")]
#[doc(alias = "j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX14PhysicsServiceEPNS4_9PrimitiveEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS7_EEvRT_")]
pub fn stub_0xf45d84() -> ! {
    todo!("0xf45d84 void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>>::operator()<RBX::Primitive *>(RBX::Primitive * &)")
}

// 0xf45e34 -- j___ZN3G3D5ArrayIPN3RBX10IAdornableELi10ELm32EEC2Ev
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "G3D::Array<RBX::IAdornable *,10,32ul>::Array(void)")]
#[doc(alias = "j___ZN3G3D5ArrayIPN3RBX10IAdornableELi10ELm32EEC2Ev")]
pub fn stub_0xf45e34() -> ! {
    todo!("0xf45e34 G3D::Array<RBX::IAdornable *,10,32ul>::Array(void)")
}

// 0xf45f74 -- j___ZN3RBX17StarterGuiServiceD2Ev
// type: void __fastcall(RBX::StarterGuiService *__hidden this)
#[doc(alias = "RBX::StarterGuiService::~StarterGuiService()")]
#[doc(alias = "j___ZN3RBX17StarterGuiServiceD2Ev")]
pub fn stub_0xf45f74() -> ! {
    todo!("0xf45f74 RBX::StarterGuiService::~StarterGuiService()")
}

// 0xf45f84 -- j___ZN3RBX19IAdornableCollectorC2Ev
// type: int __fastcall(RBX::IAdornableCollector *this)
#[doc(alias = "RBX::IAdornableCollector::IAdornableCollector(void)")]
#[doc(alias = "j___ZN3RBX19IAdornableCollectorC2Ev")]
pub fn stub_0xf45f84() -> ! {
    todo!("0xf45f84 RBX::IAdornableCollector::IAdornableCollector(void)")
}

// 0xf46174 -- j___ZN5boost6detail12shared_countC2IN3RBX19IAdornableCollectorEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::IAdornableCollector>(RBX::IAdornableCollector *)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IN3RBX19IAdornableCollectorEEEPT_")]
pub fn stub_0xf46174() -> ! {
    todo!("0xf46174 boost::detail::shared_count::shared_count<RBX::IAdornableCollector>(RBX::IAdornableCollector *)")
}
