//! boost_core_m — 100 boost stubs (EA-ordered, next uncovered after boost_core_l).
//! Source: `ida/export.json` filtered where mangled/demangled contains "boost", sorted by EA, next 100 uncovered.
//! Each stub preserves IDA address, mangled symbol, and demangled spelling; sanitized alias uses `rbx_core::SharedPtr` not `boost::`.
//! Sanitized: single quotes removed, boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr.


#[doc(alias = "rbx_core::SharedPtr<RBX::FWBase>& rbx_core::SharedPtr<RBX::FWBase>::operator=<RBX::FWPartInstance>(rbx_core::SharedPtr<RBX::FWPartInstance> const&)")]
// 0x5e60b8 — __ZN5boost10shared_ptrIN3RBX6FWBaseEEaSINS1_14FWPartInstanceEEERS3_RKNS0_IT_EE
// was: boost::shared_ptr<RBX::FWBase>& boost::shared_ptr<RBX::FWBase>::operator=<RBX::FWPartInstance>(boost::shared_ptr<RBX::FWPartInstance> const&)
pub fn stub_5e60b8() {
    // IDA 0x5e60b8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FWPartInstance> RBX::shared_from<RBX::FWPartInstance>(RBX::FWPartInstance*)")]
// 0x5e60ec — __ZN3RBX11shared_fromINS_14FWPartInstanceEEEN5boost10shared_ptrIT_EEPS4_
// was: boost::shared_ptr<RBX::FWPartInstance> RBX::shared_from<RBX::FWPartInstance>(RBX::FWPartInstance*)
pub fn stub_5e60ec() {
    // IDA 0x5e60ec: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::DumbPtr<RBX::FWPartInstance>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::DumbPtr<RBX::FWPartInstance>>,RBX::DumbPtr<RBX::FWPartInstance>,boost::hash<RBX::DumbPtr<RBX::FWPartInstance>>,std::equal_to<RBX::DumbPtr<RBX::FWPartInstance>>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::DumbPtr<RBX::FWPartInstance>>>(RBX::DumbPtr<RBX::FWPartInstance> const&,boost::unordered::detail::emplace_args1<RBX::DumbPtr<RBX::FWPartInstance>> const&)")]
// 0x5e6568 — __ZN5boost9unordered6detail10table_implINS1_3setISaIN3RBX7DumbPtrINS4_14FWPartInstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE12emplace_implINS1_13emplace_args1IS7_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEEbERKS7_RKT_
pub fn stub_5e6568() {
    // IDA 0x5e6568: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::DumbPtr<RBX::FWPartInstance>>,RBX::DumbPtr<RBX::FWPartInstance>,boost::hash<RBX::DumbPtr<RBX::FWPartInstance>>,std::equal_to<RBX::DumbPtr<RBX::FWPartInstance>>>>::create_buckets(unsigned long)")]
// 0x5e6718 — __ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX7DumbPtrINS4_14FWPartInstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm
pub fn stub_5e6718() {
    // IDA 0x5e6718: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::DumbPtr<RBX::FWPartInstance>>,RBX::DumbPtr<RBX::FWPartInstance>,boost::hash<RBX::DumbPtr<RBX::FWPartInstance>>,std::equal_to<RBX::DumbPtr<RBX::FWPartInstance>>>>::min_buckets_for_size(unsigned long)const")]
// 0x5e6840 — __ZNK5boost9unordered6detail5tableINS1_3setISaIN3RBX7DumbPtrINS4_14FWPartInstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE20min_buckets_for_sizeEm
pub fn stub_5e6840() {
    // IDA 0x5e6840: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::DumbPtr<RBX::FWPartInstance>>,RBX::DumbPtr<RBX::FWPartInstance>,boost::hash<RBX::DumbPtr<RBX::FWPartInstance>>,std::equal_to<RBX::DumbPtr<RBX::FWPartInstance>>>>::rehash_impl(unsigned long)")]
// 0x5e68d0 — __ZN5boost9unordered6detail10table_implINS1_3setISaIN3RBX7DumbPtrINS4_14FWPartInstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE11rehash_implEm
pub fn stub_5e68d0() {
    // IDA 0x5e68d0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::DumbPtr<RBX::FWPartInstance>>,RBX::DumbPtr<RBX::FWPartInstance>,boost::hash<RBX::DumbPtr<RBX::FWPartInstance>>,std::equal_to<RBX::DumbPtr<RBX::FWPartInstance>>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::DumbPtr<RBX::FWPartInstance>>,RBX::DumbPtr<RBX::FWPartInstance>,boost::hash<RBX::DumbPtr<RBX::FWPartInstance>>,std::equal_to<RBX::DumbPtr<RBX::FWPartInstance>>>> &,boost::unordered::detail::ptr_bucket *)")]
// 0x5e68fc — __ZN5boost9unordered6detail10table_implINS1_3setISaIN3RBX7DumbPtrINS4_14FWPartInstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE15place_in_bucketERNS1_5tableISD_EEPNS1_10ptr_bucketE
pub fn stub_5e68fc() {
    // IDA 0x5e68fc: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<RBX::DumbPtr<RBX::FWPartInstance>>>>::construct(void)")]
// 0x5e6950 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeIN3RBX7DumbPtrINS4_14FWPartInstanceEEEEEEE9constructEv
pub fn stub_5e6950() {
    // IDA 0x5e6950: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::DumbPtr<RBX::FWPartInstance>>> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::DumbPtr<RBX::FWPartInstance>>,RBX::DumbPtr<RBX::FWPartInstance>,boost::hash<RBX::DumbPtr<RBX::FWPartInstance>>,std::equal_to<RBX::DumbPtr<RBX::FWPartInstance>>>>::find_node_impl<RBX::DumbPtr<RBX::FWPartInstance>,std::equal_to<RBX::DumbPtr<RBX::FWPartInstance>>>(unsigned long,RBX::DumbPtr<RBX::FWPartInstance> const&,std::equal_to<RBX::DumbPtr<RBX::FWPartInstance>> const&)const")]
// 0x5e6988 — __ZNK5boost9unordered6detail10table_implINS1_3setISaIN3RBX7DumbPtrINS4_14FWPartInstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14find_node_implIS7_SC_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEEmRKT_RKT0_
pub fn stub_5e6988() {
    // IDA 0x5e6988: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::WeakPtr<RBX::PartInstance>*,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>>,rbx_core::WeakPtr<RBX::PartInstance> const&)")]
// 0x5e75dc — __ZNSt6vectorIN5boost8weak_ptrIN3RBX12PartInstanceEEESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_
// was: std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::weak_ptr<RBX::PartInstance>*,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>>,boost::weak_ptr<RBX::PartInstance> const&)
pub fn stub_5e75dc() {
    // IDA 0x5e75dc: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::_Vector_base<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>::_M_allocate(unsigned long)")]
// 0x5e7b24 — __ZNSt12_Vector_baseIN5boost8weak_ptrIN3RBX12PartInstanceEEESaIS4_EE11_M_allocateEm
// was: std::_Vector_base<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>::_M_allocate(unsigned long)
pub fn stub_5e7b24() {
    // IDA 0x5e7b24: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "rbx_core::WeakPtr<RBX::PartInstance> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::WeakPtr<RBX::PartInstance> *,rbx_core::WeakPtr<RBX::PartInstance> *>(rbx_core::WeakPtr<RBX::PartInstance> *,rbx_core::WeakPtr<RBX::PartInstance> *,rbx_core::WeakPtr<RBX::PartInstance> *)")]
// 0x5e7b40 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost8weak_ptrIN3RBX12PartInstanceEEES8_EET0_T_SA_S9_
// was: boost::weak_ptr<RBX::PartInstance> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::weak_ptr<RBX::PartInstance> *,boost::weak_ptr<RBX::PartInstance> *>(boost::weak_ptr<RBX::PartInstance> *,boost::weak_ptr<RBX::PartInstance> *,boost::weak_ptr<RBX::PartInstance> *)
pub fn stub_5e7b40() {
    // IDA 0x5e7b40: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::PartInstance>,std::allocator<rbx_core::SharedPtr<RBX::PartInstance>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::PartInstance>*,std::vector<rbx_core::SharedPtr<RBX::PartInstance>,std::allocator<rbx_core::SharedPtr<RBX::PartInstance>>>>,rbx_core::SharedPtr<RBX::PartInstance> const&)")]
// 0x5e7b98 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX12PartInstanceEEESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_
// was: std::vector<boost::shared_ptr<RBX::PartInstance>,std::allocator<boost::shared_ptr<RBX::PartInstance>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::PartInstance>*,std::vector<boost::shared_ptr<RBX::PartInstance>,std::allocator<boost::shared_ptr<RBX::PartInstance>>>>,boost::shared_ptr<RBX::PartInstance> const&)
pub fn stub_5e7b98() {
    // IDA 0x5e7b98: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::FWPartInstance,56u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0x5e8728 — __ZN5boost14singleton_poolIN3RBX14FWPartInstanceELj56ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_5e8728() {
    // IDA 0x5e8728: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::PartInstance::OnDemandPartInstance,200u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0x5e885c — __ZN5boost14singleton_poolIN3RBX12PartInstance20OnDemandPartInstanceELj200ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_5e885c() {
    // IDA 0x5e885c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TouchTransmitter>::shared_ptr<RBX::TouchTransmitter,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TouchTransmitter *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x5e8890 — __ZN5boost10shared_ptrIN3RBX16TouchTransmitterEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::TouchTransmitter>::shared_ptr<RBX::TouchTransmitter,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TouchTransmitter *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_5e8890() {
    // IDA 0x5e8890: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TouchTransmitter,RBX::TouchTransmitter>(rbx_core::SharedPtr<RBX::TouchTransmitter> const*,RBX::TouchTransmitter *)const")]
// 0x5e8958 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_16TouchTransmitterES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TouchTransmitter,RBX::TouchTransmitter>(boost::shared_ptr<RBX::TouchTransmitter> const*,RBX::TouchTransmitter *)const
pub fn stub_5e8958() {
    // IDA 0x5e8958: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TouchTransmitter *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TouchTransmitter *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x5e8a40 — __ZN5boost6detail12shared_countC2IPN3RBX16TouchTransmitterENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_5e8a40() {
    // IDA 0x5e8a40: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TouchTransmitter *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x5e8b48 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16TouchTransmitterENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_5e8b48() {
    // IDA 0x5e8b48: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TouchTransmitter *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x5e8b4c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16TouchTransmitterENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_5e8b4c() {
    // IDA 0x5e8b4c: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TouchTransmitter *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x5e8b50 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16TouchTransmitterENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_5e8b50() {
    // IDA 0x5e8b50: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TouchTransmitter *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x5e8b70 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16TouchTransmitterENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_5e8b70() {
    // IDA 0x5e8b70: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TouchTransmitter *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x5e8b88 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16TouchTransmitterENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_5e8b88() {
    // IDA 0x5e8b88: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::PartInstance,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)>& (RBX::PartInstance::*)(void)>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// 0x5e8c40 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_12PartInstanceEFvvEN3rbx6signalIS3_EEMS2_FRS6_vEE14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// was: RBX::Reflection::EventDescImpl<0,RBX::PartInstance,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)>& (RBX::PartInstance::*)(void)>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_5e8c40() {
    // IDA 0x5e8c40: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>(void (RBX::Reflection::GenericSlotWrapper::*)(std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>)")]
// 0x5e8f88 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKSt6vectorINS2_7VariantESaIS5_EENS_10shared_ptrIS3_EES7_EENS_3_bi6bind_tIT_NS_4_mfi3mf1ISE_T0_T1_EENSC_9list_av_2IT2_T3_E4typeEEEMSH_FSE_SI_ESL_SM_
// was: boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&>,boost::_bi::list_av_2<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>(void (RBX::Reflection::GenericSlotWrapper::*)(std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>)
pub fn stub_5e8f88() {
    // IDA 0x5e8f88: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>::list2(boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>)")]
// 0x5e9268 — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS2_ISt6vectorINS5_7VariantESaISA_EEEEEC2ES8_SD_
// was: boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>::list2(boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>)
pub fn stub_5e9268() {
    // IDA 0x5e9268: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>::storage2(boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>)")]
// 0x5e9368 — __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS2_ISt6vectorINS5_7VariantESaISA_EEEEEC2ES8_SD_
// was: boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>::storage2(boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>)
pub fn stub_5e9368() {
    // IDA 0x5e9368: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "__ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKSt6vectorINS9_7VariantESaISC_EEEENS4_5list2INS4_5valueINS_10shared_ptrISA_EEEENSJ_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
// 0x5e9470 — __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKSt6vectorINS9_7VariantESaISC_EEEENS4_5list2INS4_5valueINS_10shared_ptrISA_EEEENSJ_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
pub fn stub_5e9470() {
    // IDA 0x5e9470: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKSt6vectorINS8_7VariantESaISB_EEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENSI_ISD_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
// 0x5e95d0 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKSt6vectorINS8_7VariantESaISB_EEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENSI_ISD_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
pub fn stub_5e95d0() {
    // IDA 0x5e95d0: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x5e9738 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKSt6vectorINS8_7VariantESaISB_EEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENSI_ISD_EEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_5e9738() {
    // IDA 0x5e9738: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x5e9758 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKSt6vectorINSA_7VariantESaISD_EEEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENSK_ISF_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_5e9758() {
    // IDA 0x5e9758: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x5e98b8 — __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKSt6vectorINSA_7VariantESaISD_EEEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENSK_ISF_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_5e98b8() {
    // IDA 0x5e98b8: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>>::operator()(void)")]
// 0x5e99c8 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKSt6vectorINS5_7VariantESaIS8_EEEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENSF_ISA_EEEEEclEv
// was: boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>>::operator()(void)
pub fn stub_5e99c8() {
    // IDA 0x5e99c8: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x5e99e0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKSt6vectorINS8_7VariantESaISB_EEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENSI_ISD_EEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_5e99e0() {
    // IDA 0x5e99e0: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::EventDesc(RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void),char const*,char const*,RBX::Reflection::Descriptor::Attributes)")]
// 0x5ea1cc — __ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEEC2ESB_PKcSE_NS0_10Descriptor10AttributesE
// was: RBX::Reflection::EventDesc<RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::EventDesc(RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void),char const*,char const*,RBX::Reflection::Descriptor::Attributes)
pub fn stub_5ea1cc() {
    // IDA 0x5ea1cc: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::~EventDesc()")]
// 0x5ea35c — __ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEED0Ev
// was: RBX::Reflection::EventDesc<RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::~EventDesc()
pub fn stub_5ea35c() {
    // IDA 0x5ea35c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// 0x5ea410 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEE14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// was: RBX::Reflection::EventDescImpl<1,RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_5ea410() {
    // IDA 0x5ea410: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// 0x5ea5b0 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEE9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
// was: RBX::Reflection::EventDescImpl<1,RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
pub fn stub_5ea5b0() {
    // IDA 0x5ea5b0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Reflection::EventDescBase<RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::disconnectAll(RBX::Reflection::EventSource *)const")]
// 0x5ea70c — __ZNK3RBX10Reflection13EventDescBaseINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEE13disconnectAllEPNS0_11EventSourceE
// was: RBX::Reflection::EventDescBase<RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::disconnectAll(RBX::Reflection::EventSource *)const
pub fn stub_5ea70c() {
    // IDA 0x5ea70c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::safe_static_init_mutex(void)")]
// 0x5ea748 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE22safe_static_init_mutexEv
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::safe_static_init_mutex(void)
pub fn stub_5ea748() {
    // IDA 0x5ea748: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Reflection::EventDescBase<RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::getSignal(RBX::PartInstance*)const")]
// 0x5ea74c — __ZNK3RBX10Reflection13EventDescBaseINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEE9getSignalEPS2_
// was: RBX::Reflection::EventDescBase<RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::getSignal(RBX::PartInstance*)const
pub fn stub_5ea74c() {
    // IDA 0x5ea74c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance> const& rbx::any_cast<rbx_core::SharedPtr<RBX::Instance> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x5ea7c0 — __ZN3rbx8any_castIRKN5boost10shared_ptrIN3RBX8InstanceEEENS3_7Region3EEET_RNS_13placement_anyIT0_EE
// was: boost::shared_ptr<RBX::Instance> const& rbx::any_cast<boost::shared_ptr<RBX::Instance> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_5ea7c0() {
    // IDA 0x5ea7c0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(rbx_core::SharedPtr<RBX::Instance> const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)")]
// 0x5ea8b0 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS1_8InstanceEEENS4_IS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISE_T0_T1_EENSC_9list_av_2IT2_T3_E4typeEEEMSH_FSE_SI_ESL_SM_
// was: boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list_av_2<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(boost::shared_ptr<RBX::Instance> const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)
pub fn stub_5ea8b0() {
    // IDA 0x5ea8b0: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::Instance>>::clear(void)")]
// 0x5ea9d0 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEE5clearEv
// was: boost::function1<void,boost::shared_ptr<RBX::Instance>>::clear(void)
pub fn stub_5ea9d0() {
    // IDA 0x5ea9d0: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<rbx_core::SharedPtr<RBX::Instance>>::singleton(void)")]
// 0x5eaa00 — __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIN3RBX8InstanceEEEE9singletonEv
// was: rbx::implementation::typed_holder<boost::shared_ptr<RBX::Instance>>::singleton(void)
pub fn stub_5eaa00() {
    // IDA 0x5eaa00: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<rbx_core::SharedPtr<RBX::Instance>>::destruct_func(char *)")]
// 0x5eaa70 — __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIN3RBX8InstanceEEEE13destruct_funcEPc
// was: rbx::implementation::typed_holder<boost::shared_ptr<RBX::Instance>>::destruct_func(char *)
pub fn stub_5eaa70() {
    // IDA 0x5eaa70: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)")]
// 0x5eaa80 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS1_ISC_EEEENS_3argILi1EEEEEEEEEvT_
// was: void boost::function1<void,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)
pub fn stub_5eaa80() {
    // IDA 0x5eaa80: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x5eab78 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueINSA_IS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_5eab78() {
    // IDA 0x5eab78: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
// 0x5eab94 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS4_10Reflection18GenericSlotWrapperERKS6_EENS9_5list2INS9_5valueINS3_ISE_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const
pub fn stub_5eab94() {
    // IDA 0x5eab94: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "rbx::signals::connection RBX::PartInstance::TouchedSignal::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>)")]
// 0x5eac7c — __ZN3RBX12PartInstance13TouchedSignal7connectIN5boost8functionIFvNS3_10shared_ptrINS_8InstanceEEEEEEEEN3rbx7signals10connectionET_
// was: rbx::signals::connection RBX::PartInstance::TouchedSignal::connect<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>(boost::function<void ()(boost::shared_ptr<RBX::Instance>)>)
pub fn stub_5eac7c() {
    // IDA 0x5eac7c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to_own(boost::function1<void,rbx_core::SharedPtr<RBX::Instance>> const&)")]
// 0x5eadc0 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEE13assign_to_ownERKS5_
// was: boost::function1<void,boost::shared_ptr<RBX::Instance>>::assign_to_own(boost::function1<void,boost::shared_ptr<RBX::Instance>> const&)
pub fn stub_5eadc0() {
    // IDA 0x5eadc0: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<RBX::PartInstance::TouchedSignal::TouchedSlot>(RBX::PartInstance::TouchedSignal::TouchedSlot const&)")]
// 0x5eadf0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS4_12PartInstance13TouchedSignal11TouchedSlotEEENS0_10connectionERKT_
// was: rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::connect<RBX::PartInstance::TouchedSignal::TouchedSlot>(RBX::PartInstance::TouchedSignal::TouchedSlot const&)
pub fn stub_5eadf0() {
    // IDA 0x5eadf0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::flogPrint(void)")]
// 0x5eafd0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE9flogPrintEv
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::flogPrint(void)
pub fn stub_5eafd0() {
    // IDA 0x5eafd0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::PartInstance>::shared_ptr<RBX::PartInstance>(rbx_core::WeakPtr<RBX::PartInstance> const&,boost::detail::sp_nothrow_tag)")]
// 0x5eb1d0 — __ZN5boost10shared_ptrIN3RBX12PartInstanceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// was: boost::shared_ptr<RBX::PartInstance>::shared_ptr<RBX::PartInstance>(boost::weak_ptr<RBX::PartInstance> const&,boost::detail::sp_nothrow_tag)
pub fn stub_5eb1d0() {
    // IDA 0x5eb1d0: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "RBX::PartInstance::TouchedSignal::TouchedSlot::TouchedSlot(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)> const&,RBX::PartInstance*)")]
// 0x5eb24c — __ZN3RBX12PartInstance13TouchedSignal11TouchedSlotC2ERKN5boost8functionIFvNS3_10shared_ptrINS_8InstanceEEEEEEPS0_
// was: RBX::PartInstance::TouchedSignal::TouchedSlot::TouchedSlot(boost::function<void ()(boost::shared_ptr<RBX::Instance>)> const&,RBX::PartInstance*)
pub fn stub_5eb24c() {
    // IDA 0x5eb24c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::insert(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot *)")]
// 0x5eb360 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE6insertEPNS8_4slotE
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::insert(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot *)
pub fn stub_5eb360() {
    // IDA 0x5eb360: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,RBX::PartInstance::TouchedSignal::TouchedSlot,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>*>(RBX::PartInstance::TouchedSignal::TouchedSlot const&,rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>*)")]
// 0x5eb570 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS5_12PartInstance13TouchedSignal11TouchedSlotELi1ES8_EC2IPS9_EERKSD_T_
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,RBX::PartInstance::TouchedSignal::TouchedSlot,1,void ()(boost::shared_ptr<RBX::Instance>)>::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>*>(RBX::PartInstance::TouchedSignal::TouchedSlot const&,rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>*)
pub fn stub_5eb570() {
    // IDA 0x5eb570: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<RBX::PartInstance::TouchedSignal::TouchedSlot>::~callable_slot()")]
// 0x5eb66c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS4_12PartInstance13TouchedSignal11TouchedSlotEED1Ev
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<RBX::PartInstance::TouchedSignal::TouchedSlot>::~callable_slot()
pub fn stub_5eb66c() {
    // IDA 0x5eb66c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<RBX::PartInstance::TouchedSignal::TouchedSlot>::~callable_slot()")]
// 0x5eb77c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS4_12PartInstance13TouchedSignal11TouchedSlotEED0Ev
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<RBX::PartInstance::TouchedSignal::TouchedSlot>::~callable_slot()
pub fn stub_5eb77c() {
    // IDA 0x5eb77c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,RBX::PartInstance::TouchedSignal::TouchedSlot,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
// 0x5eb8b0 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS5_12PartInstance13TouchedSignal11TouchedSlotELi1ES8_E4callES7_
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,RBX::PartInstance::TouchedSignal::TouchedSlot,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)
pub fn stub_5eb8b0() {
    // IDA 0x5eb8b0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,RBX::PartInstance::TouchedSignal::TouchedSlot,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
// 0x5eb980 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS5_12PartInstance13TouchedSignal11TouchedSlotELi1ES8_E4callES7_
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,RBX::PartInstance::TouchedSignal::TouchedSlot,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)
pub fn stub_5eb980() {
    // IDA 0x5eb980: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PartInstance::TouchedSignal::TouchedSlot::operator()(rbx_core::SharedPtr<RBX::Instance>)")]
// 0x5eb988 — __ZN3RBX12PartInstance13TouchedSignal11TouchedSlotclEN5boost10shared_ptrINS_8InstanceEEE
// was: RBX::PartInstance::TouchedSignal::TouchedSlot::operator()(boost::shared_ptr<RBX::Instance>)
pub fn stub_5eb988() {
    // IDA 0x5eb988: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,RBX::PartInstance::TouchedSignal::TouchedSlot,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// 0x5eba58 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS5_12PartInstance13TouchedSignal11TouchedSlotELi1ES8_ED1Ev
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,RBX::PartInstance::TouchedSignal::TouchedSlot,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_5eba58() {
    // IDA 0x5eba58: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,RBX::PartInstance::TouchedSignal::TouchedSlot,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// 0x5ebb68 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS5_12PartInstance13TouchedSignal11TouchedSlotELi1ES8_ED0Ev
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,RBX::PartInstance::TouchedSignal::TouchedSlot,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_5ebb68() {
    // IDA 0x5ebb68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot::~slot()")]
// 0x5ebc98 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotD1Ev
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot::~slot()
pub fn stub_5ebc98() {
    // IDA 0x5ebc98: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::EventDesc(RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x5ebe34 — __ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEEC2ESB_PKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::EventDesc<RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::EventDesc(RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_5ebe34() {
    // IDA 0x5ebe34: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::PartInstance::*)(void)>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::PartInstance::*)(void),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x5ebfc4 — __ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_FRSA_vEEC2ESD_PKcSG_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::EventDesc<RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::PartInstance::*)(void)>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::PartInstance::*)(void),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_5ebfc4() {
    // IDA 0x5ebfc4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::PartInstance::*)(void)>::~EventDesc()")]
// 0x5ec154 — __ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_FRSA_vEED0Ev
// was: RBX::Reflection::EventDesc<RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::PartInstance::*)(void)>::~EventDesc()
pub fn stub_5ec154() {
    // IDA 0x5ec154: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::PartInstance::*)(void)>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// 0x5ec208 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_FRSA_vEE14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// was: RBX::Reflection::EventDescImpl<1,RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::PartInstance::*)(void)>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_5ec208() {
    // IDA 0x5ec208: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::PartInstance::*)(void)>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// 0x5ec374 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_FRSA_vEE9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISI_EE
// was: RBX::Reflection::EventDescImpl<1,RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::PartInstance::*)(void)>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
pub fn stub_5ec374() {
    // IDA 0x5ec374: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Reflection::EventDescBase<RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::PartInstance::*)(void)>::disconnectAll(RBX::Reflection::EventSource *)const")]
// 0x5ec4d0 — __ZNK3RBX10Reflection13EventDescBaseINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_FRSA_vEE13disconnectAllEPNS0_11EventSourceE
// was: RBX::Reflection::EventDescBase<RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::PartInstance::*)(void)>::disconnectAll(RBX::Reflection::EventSource *)const
pub fn stub_5ec4d0() {
    // IDA 0x5ec4d0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Reflection::EventDescBase<RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::PartInstance::*)(void)>::getSignal(RBX::PartInstance*)const")]
// 0x5ec508 — __ZNK3RBX10Reflection13EventDescBaseINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_FRSA_vEE9getSignalEPS2_
// was: RBX::Reflection::EventDescBase<RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::PartInstance::*)(void)>::getSignal(RBX::PartInstance*)const
pub fn stub_5ec508() {
    // IDA 0x5ec508: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::on_error(std::exception &)")]
// 0x5ec580 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE8on_errorERSt9exception
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::on_error(std::exception &)
pub fn stub_5ec580() {
    // IDA 0x5ec580: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)> const&)")]
// 0x5ec5a8 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_
// was: rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::connect<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>(boost::function<void ()(boost::shared_ptr<RBX::Instance>)> const&)
pub fn stub_5ec5a8() {
    // IDA 0x5ec5a8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>::~callable_slot()")]
// 0x5ec6a0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_8functionIS7_EEED1Ev
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>::~callable_slot()
pub fn stub_5ec6a0() {
    // IDA 0x5ec6a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>::~callable_slot()")]
// 0x5ec7b0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_8functionIS7_EEED0Ev
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>::~callable_slot()
pub fn stub_5ec7b0() {
    // IDA 0x5ec7b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
// 0x5ec8e0 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi1ES8_E4callES7_
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)
pub fn stub_5ec8e0() {
    // IDA 0x5ec8e0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
// 0x5ec9b0 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi1ES8_E4callES7_
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)
pub fn stub_5ec9b0() {
    // IDA 0x5ec9b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// 0x5ec9b8 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi1ES8_ED1Ev
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_5ec9b8() {
    // IDA 0x5ec9b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// 0x5ecac8 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi1ES8_ED0Ev
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_5ecac8() {
    // IDA 0x5ecac8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::NormalId RBX::Reflection::ArgHelper::getArg<RBX::NormalId,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::NormalId> const&,boost::disable_if<boost::is_same<RBX::NormalId,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// 0x5ed670 — __ZN3RBX10Reflection9ArgHelper6getArgINS_8NormalIdELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS4_EEPNS8_10disable_ifINS8_7is_sameIS4_NS8_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// was: RBX::NormalId RBX::Reflection::ArgHelper::getArg<RBX::NormalId,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::NormalId> const&,boost::disable_if<boost::is_same<RBX::NormalId,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)
pub fn stub_5ed670() {
    // IDA 0x5ed670: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "int RBX::Reflection::ArgHelper::getArg<int,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<int> const&,boost::disable_if<boost::is_same<int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// 0x5ed800 — __ZN3RBX10Reflection9ArgHelper6getArgIiLi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// was: int RBX::Reflection::ArgHelper::getArg<int,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<int> const&,boost::disable_if<boost::is_same<int,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)
pub fn stub_5ed800() {
    // IDA 0x5ed800: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<1,RBX::NormalId>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::NormalId &,boost::enable_if<boost::is_enum<RBX::NormalId>,void>::type *)")]
// 0x5ed998 — __ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_8NormalIdEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINS9_7is_enumIS7_EEvE4typeE
pub fn stub_5ed998() {
    // IDA 0x5ed998: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::Reflection::EnumDescriptor::Item const* const*,std::vector<RBX::Reflection::EnumDescriptor::Item const*,std::allocator<RBX::Reflection::EnumDescriptor::Item const*>>> std::__find_if<__gnu_cxx::__normal_iterator<RBX::Reflection::EnumDescriptor::Item const* const*,std::vector<RBX::Reflection::EnumDescriptor::Item const*,std::allocator<RBX::Reflection::EnumDescriptor::Item const*>>>,boost::_bi::bind_t<bool,bool (*)(RBX::Reflection::EnumDescriptor::Item const*,int),boost::_bi::list2<boost::arg<1>,boost::_bi::value<int>>>>(__gnu_cxx::__normal_iterator<RBX::Reflection::EnumDescriptor::Item const* const*,std::vector<RBX::Reflection::EnumDescriptor::Item const*,std::allocator<RBX::Reflection::EnumDescriptor::Item const*>>>,__gnu_cxx::__normal_iterator<RBX::Reflection::EnumDescriptor::Item const* const*,std::vector<RBX::Reflection::EnumDescriptor::Item const*,std::allocator<RBX::Reflection::EnumDescriptor::Item const*>>>,boost::_bi::bind_t<bool,bool (*)(RBX::Reflection::EnumDescriptor::Item const*,int),boost::_bi::list2<boost::arg<1>,boost::_bi::value<int>>>,std::random_access_iterator_tag)")]
// 0x5ef918 — __ZSt9__find_ifIN9__gnu_cxx17__normal_iteratorIPKPKN3RBX10Reflection14EnumDescriptor4ItemESt6vectorIS7_SaIS7_EEEEN5boost3_bi6bind_tIbPFbS7_iENSF_5list2INSE_3argILi1EEENSF_5valueIiEEEEEEET_SQ_SQ_T0_St26random_access_iterator_tag
pub fn stub_5ef918() {
    // IDA 0x5ef918: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PartInstance,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(bool),1>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::PartInstance::*)(bool),char const*,char const*,bool,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x5f1434 — __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEbELi1EEC2EMS2_FSB_bEPKcSH_bNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::BoundFuncDesc<RBX::PartInstance,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(bool),1>::BoundFuncDesc(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::PartInstance::*)(bool),char const*,char const*,bool,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_5f1434() {
    // IDA 0x5f1434: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PartInstance,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// 0x5f15e0 — __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEbELi1EE16declareSignatureEPKcNS0_7VariantE
// was: RBX::Reflection::BoundFuncDesc<RBX::PartInstance,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)
pub fn stub_5f15e0() {
    // IDA 0x5f15e0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PartInstance,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(bool),1>::~BoundFuncDesc()")]
// 0x5f1610 — __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEbELi1EED0Ev
// was: RBX::Reflection::BoundFuncDesc<RBX::PartInstance,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(bool),1>::~BoundFuncDesc()
pub fn stub_5f1610() {
    // IDA 0x5f1610: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PartInstance,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x5f16e4 — __ZNK3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// was: RBX::Reflection::BoundFuncDesc<RBX::PartInstance,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
pub fn stub_5f16e4() {
    // IDA 0x5f16e4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Reflection::Call1Helper<RBX::PartInstance,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::PartInstance::*)(bool),bool,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::call(RBX::PartInstance*,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::PartInstance::*)(bool),RBX::Reflection::Variant &,bool const&)")]
// 0x5f1724 — __ZN3RBX10Reflection11Call1HelperINS_12PartInstanceEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEbEbSB_E4callEPS2_SD_RNS0_7VariantERKb
// was: RBX::Reflection::Call1Helper<RBX::PartInstance,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::PartInstance::*)(bool),bool,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>>::call(RBX::PartInstance*,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::PartInstance::*)(bool),RBX::Reflection::Variant &,bool const&)
pub fn stub_5f1724() {
    // IDA 0x5f1724: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "bool RBX::Reflection::ArgHelper::getArg<bool,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<bool> const&,boost::disable_if<boost::is_same<bool,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// 0x5f1810 — __ZN3RBX10Reflection9ArgHelper6getArgIbLi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// was: bool RBX::Reflection::ArgHelper::getArg<bool,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<bool> const&,boost::disable_if<boost::is_same<bool,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)
pub fn stub_5f1810() {
    // IDA 0x5f1810: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::destruct_func(char *)")]
// 0x5f19b8 — __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIKSt6vectorINS3_IN3RBX8InstanceEEESaIS7_EEEEE13destruct_funcEPc
// was: rbx::implementation::typed_holder<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>>::destruct_func(char *)
pub fn stub_5f19b8() {
    // IDA 0x5f19b8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::WeakPtr<RBX::PartInstance>::weak_ptr<RBX::PartInstance>(rbx_core::SharedPtr<RBX::PartInstance> const&,boost::detail::sp_enable_if_convertible<RBX::PartInstance,RBX::PartInstance>::type)")]
// 0x5f3c08 — __ZN5boost8weak_ptrIN3RBX12PartInstanceEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
// was: boost::weak_ptr<RBX::PartInstance>::weak_ptr<RBX::PartInstance>(boost::shared_ptr<RBX::PartInstance> const&,boost::detail::sp_enable_if_convertible<RBX::PartInstance,RBX::PartInstance>::type)
pub fn stub_5f3c08() {
    // IDA 0x5f3c08: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::OnDemandPVInstance,24u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0x5f3c58 — __ZN5boost14singleton_poolIN3RBX18OnDemandPVInstanceELj24ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_5f3c58() {
    // IDA 0x5f3c58: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::FWInstance,28u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0x5f3cb0 — __ZN5boost14singleton_poolIN3RBX10FWInstanceELj28ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_5f3cb0() {
    // IDA 0x5f3cb0: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>::operator*(void)const")]
// 0x5f45d0 — __ZNK3RBX17copy_on_write_ptrISt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EEEdeEv
// was: RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>::operator*(void)const
pub fn stub_5f45d0() {
    // IDA 0x5f45d0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::PartInstance::OnDemandPartInstance,200u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0x5f4698 — __ZN5boost14singleton_poolIN3RBX12PartInstance20OnDemandPartInstanceELj200ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_5f4698() {
    // IDA 0x5f4698: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::OnDemandPVInstance,24u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0x5f55e4 — __ZN5boost14singleton_poolIN3RBX18OnDemandPVInstanceELj24ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_5f55e4() {
    // IDA 0x5f55e4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::BasePlayerGui::onDescendantRemoving(rbx_core::SharedPtr<RBX::Instance> const&)")]
// 0x5fbad4 — __ZN3RBX13BasePlayerGui20onDescendantRemovingERKN5boost10shared_ptrINS_8InstanceEEE
// was: RBX::BasePlayerGui::onDescendantRemoving(boost::shared_ptr<RBX::Instance> const&)
pub fn stub_5fbad4() {
    // IDA 0x5fbad4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ScreenGui>(rbx_core::SharedPtr<RBX::ScreenGui> const&)")]
// 0x5fd3d4 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_9ScreenGuiEEERS3_RKNS0_IT_EE
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::ScreenGui>(boost::shared_ptr<RBX::ScreenGui> const&)
pub fn stub_5fd3d4() {
    // IDA 0x5fd3d4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScreenGui> RBX::Creatable<RBX::Instance>::create<RBX::ScreenGui>(void)")]
// 0x5fd408 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_9ScreenGuiEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::ScreenGui> RBX::Creatable<RBX::Instance>::create<RBX::ScreenGui>(void)
pub fn stub_5fd408() {
    // IDA 0x5fd408: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}
