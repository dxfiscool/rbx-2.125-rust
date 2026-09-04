//! audio generated_28 — next 100 stubs EA-sorted, from ida/export.json
//! Filter: FMOD|Sound|Audio (2454) exhausted — filler workspace EA-sorted asc after 0x2f829c, skip existing, rbx_core::SharedPtr not boost
//! Batch: 100 stubs | skeleton batch | range 0x2f82e8..0x2ffbfc EA-sorted asc after 0x2f829c, skip existing, rbx_core::SharedPtr not boost
//! Generated: 2026-09-01

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// Ensure SharedPtr is seen as used — mirrors boost::shared_ptr<T> -> rbx_core::SharedPtr<T>
const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x2f82e8 — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX8InstanceEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE7_M_copyEPKSt13_Rb_tree_nodeIS4_EPSC_
#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_copy(std::_Rb_tree_node<rbx_core::SharedPtr<RBX::Instance>> const*,std::_Rb_tree_node<rbx_core::SharedPtr<RBX::Instance>>*)")]
pub fn stub_2f82e8() -> ! {
    todo!("0x2f82e8 std::_Rb_tree<boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,std::_Identity<boost::shared_ptr<RBX::Instance>>,std::less<boost::shared_ptr<RBX::Instance>>,std::allocator<boost::shared_ptr<RBX::Instance>>>::_M_copy(std::_Rb_tree_node<boost::shared_ptr<RBX::Instance>> const*,std::_Rb")
}

// 0x2f843c — __ZNSt6__copyILb0ESt26bidirectional_iterator_tagE4copyISt23_Rb_tree_const_iteratorIN5boost10shared_ptrIN3RBX8InstanceEEEENS6_9Selection14RemoveIteratorEEET0_T_SD_SC_
#[doc(alias = "RBX::Selection::RemoveIterator std::__copy<false,std::bidirectional_iterator_tag>::copy<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::RemoveIterator>(std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::RemoveIterator)")]
pub fn stub_2f843c() -> ! {
    todo!("0x2f843c RBX::Selection::RemoveIterator std::__copy<false,std::bidirectional_iterator_tag>::copy<std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::Instance>>,RBX::Selection::RemoveIterator>(std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::Instance>>,std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::In")
}

// 0x2f8530 — __ZNSt6__copyILb0ESt26bidirectional_iterator_tagE4copyISt23_Rb_tree_const_iteratorIN5boost10shared_ptrIN3RBX8InstanceEEEENS6_9Selection11AddIteratorEEET0_T_SD_SC_
#[doc(alias = "RBX::Selection::AddIterator std::__copy<false,std::bidirectional_iterator_tag>::copy<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::AddIterator>(std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::AddIterator)")]
pub fn stub_2f8530() -> ! {
    todo!("0x2f8530 RBX::Selection::AddIterator std::__copy<false,std::bidirectional_iterator_tag>::copy<std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::Instance>>,RBX::Selection::AddIterator>(std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::Instance>>,std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::Instance")
}

// 0x2f8624 — __ZN5boost10shared_ptrIN3RBX16BoxSelectCommandEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::BoxSelectCommand>::shared_ptr<RBX::BoxSelectCommand,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
pub fn stub_2f8624() -> ! {
    todo!("0x2f8624 boost::shared_ptr<RBX::BoxSelectCommand>::shared_ptr<RBX::BoxSelectCommand,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter)")
}

// 0x2f86ec — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_16BoxSelectCommandES5_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::BoxSelectCommand,RBX::BoxSelectCommand>(rbx_core::SharedPtr<RBX::BoxSelectCommand> const*,RBX::BoxSelectCommand *)const")]
pub fn stub_2f86ec() {
    // IDA 0x2f86ec: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0x2f87d0 — __ZN5boost6detail12shared_countC2IPN3RBX16BoxSelectCommandENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
pub fn stub_2f87d0() {
    // IDA 0x2f87d0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x2f88c8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16BoxSelectCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_2f88c8() {
    // IDA 0x2f88c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2f88cc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16BoxSelectCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_2f88cc() {
    // IDA 0x2f88cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2f88d0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16BoxSelectCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
pub fn stub_2f88d0() {
    // IDA 0x2f88d0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x2f88e0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16BoxSelectCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_2f88e0() {
    // IDA 0x2f88e0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x2f88f8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16BoxSelectCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_2f88f8() {
    // IDA 0x2f88f8: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x2f88fc — __ZNK3RBX13ServiceClientINS_17FilteredSelectionINS_8InstanceEEEE13createServiceEv
#[doc(alias = "RBX::ServiceClient<RBX::FilteredSelection<RBX::Instance>>::createService(void)const")]
pub fn stub_2f88fc() -> ! {
    todo!("0x2f88fc RBX::ServiceClient<RBX::FilteredSelection<RBX::Instance>>::createService(void)const")
}

// 0x2f89dc — __ZN5boost10shared_ptrIN3RBX17FilteredSelectionINS1_8InstanceEEEEaSERKS5_
#[doc(alias = "rbx_core::SharedPtr<RBX::FilteredSelection<RBX::Instance>>::operator=(rbx_core::SharedPtr<RBX::FilteredSelection<RBX::Instance>> const&)")]
pub fn stub_2f89dc() -> ! {
    todo!("0x2f89dc boost::shared_ptr<RBX::FilteredSelection<RBX::Instance>>::operator=(boost::shared_ptr<RBX::FilteredSelection<RBX::Instance>> const&)")
}

// 0x2f8a14 — __ZN3RBX11shared_fromINS_17FilteredSelectionINS_8InstanceEEEEEN5boost10shared_ptrIT_EEPS6_
#[doc(alias = "rbx_core::SharedPtr<RBX::FilteredSelection<RBX::Instance>> RBX::shared_from<RBX::FilteredSelection<RBX::Instance>>(RBX::FilteredSelection<RBX::Instance>*)")]
pub fn stub_2f8a14() -> ! {
    todo!("0x2f8a14 boost::shared_ptr<RBX::FilteredSelection<RBX::Instance>> RBX::shared_from<RBX::FilteredSelection<RBX::Instance>>(RBX::FilteredSelection<RBX::Instance>*)")
}

// 0x2f8b84 — __ZN3RBX15ServiceProvider6createINS_17FilteredSelectionINS_8InstanceEEEEEPT_PKS3_
#[doc(alias = "RBX::FilteredSelection<RBX::Instance> * RBX::ServiceProvider::create<RBX::FilteredSelection<RBX::Instance>>(RBX::Instance const*)")]
pub fn stub_2f8b84() -> ! {
    todo!("0x2f8b84 RBX::FilteredSelection<RBX::Instance> * RBX::ServiceProvider::create<RBX::FilteredSelection<RBX::Instance>>(RBX::Instance const*)")
}

// 0x2f8b9c — __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEENS3_IKS5_EEET_SF_SF_RKT0_St26random_access_iterator_tag
#[doc(alias = "__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> std::__find<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,rbx_core::SharedPtr<RBX::Instance const>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std:")]
pub fn stub_2f8b9c() -> ! {
    todo!("0x2f8b9c __gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> std::__find<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allo")
}

// 0x2f8c2c — __GLOBAL__I_a_103
#[doc(alias = "global constructor keyed to_a_103")]
pub fn stub_2f8c2c() -> ! {
    todo!("0x2f8c2c global constructor keyed to_a_103")
}

// 0x2f8f04 — __ZN3RBX10Reflection8EnumDescINS_6Action10ActionTypeEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Action::ActionType>::EnumDesc(void)")]
pub fn stub_2f8f04() -> ! {
    todo!("0x2f8f04 RBX::Reflection::EnumDesc<RBX::Action::ActionType>::EnumDesc(void)")
}

// 0x2f8f08 — __ZN3RBX10Reflection8EnumDescINS_6Action10ActionTypeEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Action::ActionType>::EnumDesc(void)")]
pub fn stub_2f8f08() -> ! {
    todo!("0x2f8f08 RBX::Reflection::EnumDesc<RBX::Action::ActionType>::EnumDesc(void)")
}

// 0x2f910c — __ZN3RBX10Reflection8EnumDescINS_6Action10ActionTypeEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Action::ActionType>::addPair(RBX::Action::ActionType,char const*)")]
pub fn stub_2f910c() -> ! {
    todo!("0x2f910c RBX::Reflection::EnumDesc<RBX::Action::ActionType>::addPair(RBX::Action::ActionType,char const*)")
}

// 0x2f946c — __ZNSt6vectorIN3RBX6Action10ActionTypeESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::resize(unsigned long,RBX::Action::ActionType)")]
pub fn stub_2f946c() -> ! {
    todo!("0x2f946c std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::resize(unsigned long,RBX::Action::ActionType)")
}

// 0x2f94a0 — __ZNSt6vectorIN3RBX6Action10ActionTypeESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::push_back(RBX::Action::ActionType const&)")]
pub fn stub_2f94a0() -> ! {
    todo!("0x2f94a0 std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::push_back(RBX::Action::ActionType const&)")
}

// 0x2f94c8 — __ZNSt3mapIPKN3RBX4NameENS0_6Action10ActionTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::Action::ActionType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>>::operator[](RBX::Name const* const&)")]
pub fn stub_2f94c8() -> ! {
    todo!("0x2f94c8 std::map<RBX::Name const*,RBX::Action::ActionType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>>::operator[](RBX::Name const* const&)")
}

// 0x2f9520 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Action10ActionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Action::ActionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Action::ActionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>,std::pair<RBX::Name const* const,RBX::Action::ActionType> const&)")]
pub fn stub_2f9520() -> ! {
    todo!("0x2f9520 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Action::ActionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Action::ActionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>>::_M_insert_unique(std::_Rb_tree_i")
}

// 0x2f95d4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Action10ActionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Action::ActionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Action::ActionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Action::ActionType> const&)")]
pub fn stub_2f95d4() -> ! {
    todo!("0x2f95d4 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Action::ActionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Action::ActionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>>::_M_insert(std::_Rb_tree_node_bas")
}

// 0x2f962c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Action10ActionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Action::ActionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Action::ActionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Action::ActionType> const&)")]
pub fn stub_2f962c() -> ! {
    todo!("0x2f962c std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Action::ActionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Action::ActionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>>::_M_insert_unique(std::pair<RBX::")
}

// 0x2f9694 — __ZNSt6vectorIN3RBX6Action10ActionTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Action::ActionType*,std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>>,RBX::Action::ActionType const&)")]
pub fn stub_2f9694() -> ! {
    todo!("0x2f9694 std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Action::ActionType*,std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>>,RBX::Action::ActionType const&)")
}

// 0x2f9778 — __ZNSt12_Vector_baseIN3RBX6Action10ActionTypeESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::_M_allocate(unsigned long)")]
pub fn stub_2f9778() -> ! {
    todo!("0x2f9778 std::_Vector_base<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::_M_allocate(unsigned long)")
}

// 0x2f9790 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Action10ActionTypeES6_EET0_T_S8_S7_
#[doc(alias = "RBX::Action::ActionType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Action::ActionType *,RBX::Action::ActionType *>(RBX::Action::ActionType *,RBX::Action::ActionType *,RBX::Action::ActionType *)")]
pub fn stub_2f9790() -> ! {
    todo!("0x2f9790 RBX::Action::ActionType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Action::ActionType *,RBX::Action::ActionType *>(RBX::Action::ActionType *,RBX::Action::ActionType *,RBX::Action::ActionType *)")
}

// 0x2f97cc — __ZNSt6vectorIN3RBX6Action10ActionTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Action::ActionType*,std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>>,unsigned long,RBX::Action::ActionType const&)")]
pub fn stub_2f97cc() -> ! {
    todo!("0x2f97cc std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Action::ActionType*,std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>>,unsigned long,RBX::Action::ActionType const&)")
}

// 0x2f995c — __GLOBAL__I_a_104
#[doc(alias = "global constructor keyed to_a_104")]
pub fn stub_2f995c() -> ! {
    todo!("0x2f995c global constructor keyed to_a_104")
}

// 0x2f9a24 — __ZN3RBX15StringConverterINS_11AnimationIdEE14convertToValueERKSsRS1_
#[doc(alias = "RBX::StringConverter<RBX::AnimationId>::convertToValue(std::string const&,RBX::AnimationId&)")]
pub fn stub_2f9a24() -> ! {
    todo!("0x2f9a24 RBX::StringConverter<RBX::AnimationId>::convertToValue(std::string const&,RBX::AnimationId&)")
}

// 0x2f9b48 — __ZN3RBX10Reflection4Type12getSingletonINS_11AnimationIdEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::AnimationId>(void)")]
pub fn stub_2f9b48() -> ! {
    todo!("0x2f9b48 RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::AnimationId>(void)")
}

// 0x2f9b4c — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_11AnimationIdEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_2f9b4c() -> ! {
    todo!("0x2f9b4c RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

// 0x2f9d34 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_11AnimationIdEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_2f9d34() -> ! {
    todo!("0x2f9d34 RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

// 0x2f9edc — __ZN3RBX10Reflection7Variant7convertINS_11AnimationIdEEERT_v
#[doc(alias = "RBX::AnimationId & RBX::Reflection::Variant::convert<RBX::AnimationId>(void)")]
pub fn stub_2f9edc() -> ! {
    todo!("0x2f9edc RBX::AnimationId & RBX::Reflection::Variant::convert<RBX::AnimationId>(void)")
}

// 0x2fa0c8 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_11AnimationIdEE11getDataSizeEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::getDataSize(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_2fa0c8() -> ! {
    todo!("0x2fa0c8 RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::getDataSize(RBX::Reflection::DescribedBase const*)const")
}

// 0x2fa124 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_11AnimationIdEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::hasStringValue(void)const")]
pub fn stub_2fa124() -> ! {
    todo!("0x2fa124 RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::hasStringValue(void)const")
}

// 0x2fa128 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_11AnimationIdEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_2fa128() -> ! {
    todo!("0x2fa128 RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::getStringValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x2fa244 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_11AnimationIdEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_2fa244() -> ! {
    todo!("0x2fa244 RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x2fa39c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11AnimationIdEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::AnimationId>(RBX::AnimationId const&)")]
pub fn stub_2fa39c() -> ! {
    todo!("0x2fa39c rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::AnimationId>(RBX::AnimationId const&)")
}

// 0x2fa3fc — __ZN3RBX10Reflection7Variant14genericConvertINS_11AnimationIdEEERT_v
#[doc(alias = "RBX::AnimationId & RBX::Reflection::Variant::genericConvert<RBX::AnimationId>(void)")]
pub fn stub_2fa3fc() -> ! {
    todo!("0x2fa3fc RBX::AnimationId & RBX::Reflection::Variant::genericConvert<RBX::AnimationId>(void)")
}

// 0x2fa6a8 — __ZN3rbx8any_castIN3RBX11AnimationIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
#[doc(alias = "RBX::AnimationId * rbx::any_cast<RBX::AnimationId,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
pub fn stub_2fa6a8() -> ! {
    todo!("0x2fa6a8 RBX::AnimationId * rbx::any_cast<RBX::AnimationId,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")
}

// 0x2fa700 — __ZN3rbx8any_castIRN3RBX11AnimationIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::AnimationId & rbx::any_cast<RBX::AnimationId &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_2fa700() -> ! {
    todo!("0x2fa700 RBX::AnimationId & rbx::any_cast<RBX::AnimationId &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x2fa7f0 — __ZN3rbx14implementation12typed_holderIN3RBX11AnimationIdEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::AnimationId>::singleton(void)")]
pub fn stub_2fa7f0() -> ! {
    todo!("0x2fa7f0 rbx::implementation::typed_holder<RBX::AnimationId>::singleton(void)")
}

// 0x2fa85c — __ZN3rbx14implementation12typed_holderIN3RBX11AnimationIdEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::AnimationId>::construct_func(char const*,char *)")]
pub fn stub_2fa85c() -> ! {
    todo!("0x2fa85c rbx::implementation::typed_holder<RBX::AnimationId>::construct_func(char const*,char *)")
}

// 0x2fa878 — __ZN3rbx14implementation12typed_holderIN3RBX11AnimationIdEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::AnimationId>::destruct_func(char *)")]
pub fn stub_2fa878() -> ! {
    todo!("0x2fa878 rbx::implementation::typed_holder<RBX::AnimationId>::destruct_func(char *)")
}

// 0x2fa87c — __GLOBAL__I_a_105
#[doc(alias = "global constructor keyed to_a_105")]
pub fn stub_2fa87c() -> ! {
    todo!("0x2fa87c global constructor keyed to_a_105")
}

// 0x2faa84 — __ZN3RBX14AsyncHttpQueueC2EPNS_8InstanceEN5boost8functionIFbRKSsPSsEEEi
#[doc(alias = "RBX::AsyncHttpQueue::AsyncHttpQueue(RBX::Instance *,boost::function<bool ()(std::string const&,std::string *)>,int)")]
pub fn stub_2faa84() -> ! {
    todo!("0x2faa84 RBX::AsyncHttpQueue::AsyncHttpQueue(RBX::Instance *,boost::function<bool ()(std::string const&,std::string *)>,int)")
}

// 0x2fad24 — __ZN3RBX14AsyncHttpQueue13setThreadPoolEi
#[doc(alias = "RBX::AsyncHttpQueue::setThreadPool(int)")]
pub fn stub_2fad24() -> ! {
    todo!("0x2fad24 RBX::AsyncHttpQueue::setThreadPool(int)")
}

// 0x2fae00 — __ZN3RBX14AsyncHttpQueue14resetStatsItemEPNS_15ServiceProviderE
#[doc(alias = "RBX::AsyncHttpQueue::resetStatsItem(RBX::ServiceProvider *)")]
pub fn stub_2fae00() -> ! {
    todo!("0x2fae00 RBX::AsyncHttpQueue::resetStatsItem(RBX::ServiceProvider *)")
}

// 0x2faf2c — __ZNK3RBX14AsyncHttpQueue19getRequestQueueSizeEv
#[doc(alias = "RBX::AsyncHttpQueue::getRequestQueueSize(void)const")]
pub fn stub_2faf2c() -> ! {
    todo!("0x2faf2c RBX::AsyncHttpQueue::getRequestQueueSize(void)const")
}

// 0x2faf68 — __ZN3RBX14AsyncHttpQueueD0Ev
#[doc(alias = "RBX::AsyncHttpQueue::~AsyncHttpQueue()")]
pub fn stub_2faf68() {
    // IDA 0x2faf68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2fb008 — __ZN3RBX14AsyncHttpQueueD1Ev
#[doc(alias = "RBX::AsyncHttpQueue::~AsyncHttpQueue()")]
pub fn stub_2fb008() {
    // IDA 0x2fb008: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2fb00c — __ZN3RBX14AsyncHttpQueueD2Ev
#[doc(alias = "RBX::AsyncHttpQueue::~AsyncHttpQueue()")]
pub fn stub_2fb00c() {
    // IDA 0x2fb00c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2fb2ac — __ZN3RBX14AsyncHttpQueue11onHeartbeatERKNS_9HeartbeatE
#[doc(alias = "RBX::AsyncHttpQueue::onHeartbeat(RBX::Heartbeat const&)")]
pub fn stub_2fb2ac() -> ! {
    todo!("0x2fb2ac RBX::AsyncHttpQueue::onHeartbeat(RBX::Heartbeat const&)")
}

// 0x2fb548 — __ZN3RBX14AsyncHttpQueue15processRequestsEN5boost8weak_ptrIS0_EESt14_List_iteratorINS0_7RequestEENS1_10shared_ptrINS_5mutexEEE
#[doc(alias = "RBX::AsyncHttpQueue::processRequests(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>)")]
pub fn stub_2fb548() -> ! {
    todo!("0x2fb548 RBX::AsyncHttpQueue::processRequests(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>)")
}

// 0x2fc440 — __ZN3RBX14AsyncHttpQueue23dispatchGenericCallbackEN5boost8functionIFvPNS_9DataModelEEEEPNS_8InstanceENS0_9ResultJobE
#[doc(alias = "RBX::AsyncHttpQueue::dispatchGenericCallback(boost::function<void ()(RBX::DataModel *)>,RBX::Instance *,RBX::AsyncHttpQueue::ResultJob)")]
pub fn stub_2fc440() -> ! {
    todo!("0x2fc440 RBX::AsyncHttpQueue::dispatchGenericCallback(boost::function<void ()(RBX::DataModel *)>,RBX::Instance *,RBX::AsyncHttpQueue::ResultJob)")
}

// 0x2fc6a8 — __ZN3RBX14AsyncHttpQueue16dispatchCallbackEN5boost8functionIFvNS0_13RequestResultEPSiNS1_10shared_ptrIKSsEEEEEPNS_8InstanceES3_S7_NS0_9ResultJobE
#[doc(alias = "RBX::AsyncHttpQueue::dispatchCallback(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>,RBX::AsyncHttpQueue::ResultJob)")]
pub fn stub_2fc6a8() -> ! {
    todo!("0x2fc6a8 RBX::AsyncHttpQueue::dispatchCallback(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>,RBX::AsyncHttpQueue::ResultJob)")
}

// 0x2fc874 — __ZN3RBXL19InvokeAsyncCallbackEN5boost8functionIFvNS_14AsyncHttpQueue13RequestResultEPSiNS0_10shared_ptrIKSsEEEEES3_S7_
#[doc(alias = "RBX::InvokeAsyncCallback(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>)")]
pub fn stub_2fc874() -> ! {
    todo!("0x2fc874 RBX::InvokeAsyncCallback(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>)")
}

// 0x2fca04 — __ZN3RBX14AsyncHttpQueue19isRequestQueueEmptyEv
#[doc(alias = "RBX::AsyncHttpQueue::isRequestQueueEmpty(void)")]
pub fn stub_2fca04() -> ! {
    todo!("0x2fca04 RBX::AsyncHttpQueue::isRequestQueueEmpty(void)")
}

// 0x2fca3c — __ZN3RBXL15checkContentUrlESs
#[doc(alias = "RBX::checkContentUrl(std::string)")]
pub fn stub_2fca3c() -> ! {
    todo!("0x2fca3c RBX::checkContentUrl(std::string)")
}

// 0x2fcfb4 — __ZN3RBXL8callbackENS_14AsyncHttpQueue15CallbackWrapperEPNS_8InstanceENS0_13RequestResultEN5boost10shared_ptrISsEE
#[doc(alias = "RBX::callback(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>)")]
pub fn stub_2fcfb4() -> ! {
    todo!("0x2fcfb4 RBX::callback(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>)")
}

// 0x2fd150 — __ZN3RBX14AsyncHttpQueue9FailedUrlC2EPKc
#[doc(alias = "RBX::AsyncHttpQueue::FailedUrl::FailedUrl(char const*)")]
pub fn stub_2fd150() -> ! {
    todo!("0x2fd150 RBX::AsyncHttpQueue::FailedUrl::FailedUrl(char const*)")
}

// 0x2fd220 — __ZN3RBX14AsyncHttpQueue8isUrlBadERKSs
#[doc(alias = "RBX::AsyncHttpQueue::isUrlBad(std::string const&)")]
pub fn stub_2fd220() -> ! {
    todo!("0x2fd220 RBX::AsyncHttpQueue::isUrlBad(std::string const&)")
}

// 0x2fd37c — __ZN3RBX14AsyncHttpQueue12asyncRequestERKSsfPN5boost8functionIFvNS0_13RequestResultEPSiNS3_10shared_ptrIS1_EEEEENS0_9ResultJobEb
#[doc(alias = "RBX::AsyncHttpQueue::asyncRequest(std::string const&,float,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)> *,RBX::AsyncHttpQueue::ResultJob,bool)")]
pub fn stub_2fd37c() -> ! {
    todo!("0x2fd37c RBX::AsyncHttpQueue::asyncRequest(std::string const&,float,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)> *,RBX::AsyncHttpQueue::ResultJob,bool)")
}

// 0x2fd910 — __ZN3RBX14AsyncHttpQueue11syncRequestERKSs
#[doc(alias = "RBX::AsyncHttpQueue::syncRequest(std::string const&)")]
pub fn stub_2fd910() -> ! {
    todo!("0x2fd910 RBX::AsyncHttpQueue::syncRequest(std::string const&)")
}

// 0x2fded0 — __ZN5boost10shared_ptrIN3RBX18HttpQueueStatsItemEEaSERKS3_
#[doc(alias = "rbx_core::SharedPtr<RBX::HttpQueueStatsItem>::operator=(rbx_core::SharedPtr<RBX::HttpQueueStatsItem> const&)")]
pub fn stub_2fded0() -> ! {
    todo!("0x2fded0 boost::shared_ptr<RBX::HttpQueueStatsItem>::operator=(boost::shared_ptr<RBX::HttpQueueStatsItem> const&)")
}

// 0x2fdf08 — __ZN3RBX18HttpQueueStatsItem6createEPNS_14AsyncHttpQueueEPNS_8InstanceE
#[doc(alias = "RBX::HttpQueueStatsItem::create(RBX::AsyncHttpQueue *,RBX::Instance *)")]
pub fn stub_2fdf08() -> ! {
    todo!("0x2fdf08 RBX::HttpQueueStatsItem::create(RBX::AsyncHttpQueue *,RBX::Instance *)")
}

// 0x2fdfbc — __ZN5boost4bindIvNS_8weak_ptrIN3RBX14AsyncHttpQueueEEESt14_List_iteratorINS3_7RequestEENS_10shared_ptrINS2_5mutexEEES4_S7_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSF_T0_T1_T2_ENSD_9list_av_3IT3_T4_T5_E4typeEEESK_SM_SN_SO_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list_av_3<rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::arg<1>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>,rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Reques")]
pub fn stub_2fdfbc() -> ! {
    todo!("0x2fdfbc boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list_av_3<boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::arg<1>>::type> boost::bind<void,b")
}

// 0x2fe168 — __ZN3RBX9weak_fromINS_14AsyncHttpQueueEEEN5boost8weak_ptrIT_EEPS4_
#[doc(alias = "rbx_core::WeakPtr<RBX::AsyncHttpQueue> RBX::weak_from<RBX::AsyncHttpQueue>(RBX::AsyncHttpQueue*)")]
pub fn stub_2fe168() -> ! {
    todo!("0x2fe168 boost::weak_ptr<RBX::AsyncHttpQueue> RBX::weak_from<RBX::AsyncHttpQueue>(RBX::AsyncHttpQueue*)")
}

// 0x2fe358 — __ZN5boost4bindIvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEES4_S8_SA_S4_S8_EENS_3_bi6bind_tIT_PFSD_T0_T1_T2_ENSB_9list_av_3IT3_T4_T5_E4typeEEESI_SK_SL_SM_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list_av_3<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>>::type> boost::bind<void,boost::function<void ()(RBX::AsyncHttpQueue::Requ")]
pub fn stub_2fe358() -> ! {
    todo!("0x2fe358 boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::_bi::list_av_3<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,")
}

// 0x2fe524 — __ZNK5boost9function2IbRKSsPSsEclES2_S3_
#[doc(alias = "boost::function2<bool,std::string const&,std::string *>::operator()(std::string const&,std::string *)const")]
pub fn stub_2fe524() -> ! {
    todo!("0x2fe524 boost::function2<bool,std::string const&,std::string *>::operator()(std::string const&,std::string *)const")
}

// 0x2fe5f0 — __ZN5boost10shared_ptrIN3RBX4HttpEEaSERKS3_
#[doc(alias = "rbx_core::SharedPtr<RBX::Http>::operator=(rbx_core::SharedPtr<RBX::Http> const&)")]
pub fn stub_2fe5f0() -> ! {
    todo!("0x2fe5f0 boost::shared_ptr<RBX::Http>::operator=(boost::shared_ptr<RBX::Http> const&)")
}

// 0x2fe628 — __ZN5boost10shared_ptrISsE5resetISsEEvPT_
#[doc(alias = "void rbx_core::SharedPtr<std::string>::reset<std::string>(std::string *)")]
pub fn stub_2fe628() -> ! {
    todo!("0x2fe628 void boost::shared_ptr<std::string>::reset<std::string>(std::string *)")
}

// 0x2fe654 — __ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EEaSERKS4_
#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::operator=(std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>> const&)")]
pub fn stub_2fe654() -> ! {
    todo!("0x2fe654 std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::operator=(std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>> const&)")
}

// 0x2fe884 — __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPN3RBX14AsyncHttpQueue15CallbackWrapperESt6vectorIS4_SaIS4_EEEEN5boost3_bi6bind_tIvPFvS4_PNS2_8InstanceENS3_13RequestResultENSA_10shared_ptrISsEEENSB_5list4INSA_3argILi1EEENSB_5valueISE_EENSN_ISF_EENSN_ISH_EEEEEEET0_T_SU_ST_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>),boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string>>>> std::for_each<__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper *,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::Callbac")]
pub fn stub_2fe884() -> ! {
    todo!("0x2fe884 boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>),boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::s")
}

// 0x2fe8f4 — __ZN5boost4bindIvN3RBX14AsyncHttpQueue15CallbackWrapperEPNS1_8InstanceENS2_13RequestResultENS_10shared_ptrISsEENS_3argILi1EEES5_S6_S8_EENS_3_bi6bind_tIT_PFSD_T0_T1_T2_T3_ENSB_9list_av_4IT4_T5_T6_T7_E4typeEEESJ_SL_SM_SN_SO_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>),boost::_bi::list_av_4<boost::arg<1>,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>>::type> boost::bind<void,RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>,boost::arg<1>,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<st")]
pub fn stub_2fe8f4() -> ! {
    todo!("0x2fe8f4 boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>),boost::_bi::list_av_4<boost::arg<1>,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>>::type> boost::bind<void,RBX:")
}

// 0x2fea20 — __ZNSt4listIN3RBX14AsyncHttpQueue9FailedUrlESaIS2_EE5eraseESt14_List_iteratorIS2_ES6_
#[doc(alias = "std::list<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::erase(std::_List_iterator<RBX::AsyncHttpQueue::FailedUrl>,std::_List_iterator<RBX::AsyncHttpQueue::FailedUrl>)")]
pub fn stub_2fea20() -> ! {
    todo!("0x2fea20 std::list<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::erase(std::_List_iterator<RBX::AsyncHttpQueue::FailedUrl>,std::_List_iterator<RBX::AsyncHttpQueue::FailedUrl>)")
}

// 0x2fea58 — __ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::push_back(RBX::AsyncHttpQueue::CallbackWrapper const&)")]
pub fn stub_2fea58() -> ! {
    todo!("0x2fea58 std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::push_back(RBX::AsyncHttpQueue::CallbackWrapper const&)")
}

// 0x2feaa8 — __ZN3RBX14AsyncHttpQueue15registerContentERKSsN5boost10shared_ptrIS1_EES5_
#[doc(alias = "RBX::AsyncHttpQueue::registerContent(std::string const&,rbx_core::SharedPtr<std::string const>,rbx_core::SharedPtr<std::string const>)")]
pub fn stub_2feaa8() -> ! {
    todo!("0x2feaa8 RBX::AsyncHttpQueue::registerContent(std::string const&,boost::shared_ptr<std::string const>,boost::shared_ptr<std::string const>)")
}

// 0x2feab0 — __ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,RBX::AsyncHttpQueue::CallbackWrapper const&)")]
pub fn stub_2feab0() -> ! {
    todo!("0x2feab0 std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,RBX::AsyncH")
}

// 0x2fee5c — __ZNSt12_Vector_baseIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_allocate(unsigned long)")]
pub fn stub_2fee5c() -> ! {
    todo!("0x2fee5c std::_Vector_base<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_allocate(unsigned long)")
}

// 0x2fee80 — __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEaSERKS9_
#[doc(alias = "boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>::operator=(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)> const&)")]
pub fn stub_2fee80() -> ! {
    todo!("0x2fee80 boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>::operator=(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)> const&)")
}

// 0x2fef44 — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE4swapERS8_
#[doc(alias = "boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::swap(boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>&)")]
pub fn stub_2fef44() {
    // IDA 0x2fef44: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

// 0x2ff020 — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE11move_assignERS8_
#[doc(alias = "boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::move_assign(boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>&)")]
pub fn stub_2ff020() {
    // IDA 0x2ff020: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

// 0x2ff128 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX14AsyncHttpQueue15CallbackWrapperES6_EET0_T_S8_S7_
#[doc(alias = "RBX::AsyncHttpQueue::CallbackWrapper * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *>(RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *)")]
pub fn stub_2ff128() -> ! {
    todo!("0x2ff128 RBX::AsyncHttpQueue::CallbackWrapper * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *>(RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapp")
}

// 0x2ff188 — __ZNSt4listIN3RBX14AsyncHttpQueue7RequestESaIS2_EE14_M_create_nodeERKS2_
#[doc(alias = "std::list<RBX::AsyncHttpQueue::Request,std::allocator<RBX::AsyncHttpQueue::Request>>::_M_create_node(RBX::AsyncHttpQueue::Request const&)")]
pub fn stub_2ff188() {
    // IDA 0x2ff188: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

// 0x2ff2d4 — __ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EEC2ERKS4_
#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::vector(std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>> const&)")]
pub fn stub_2ff2d4() -> ! {
    todo!("0x2ff2d4 std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::vector(std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>> const&)")
}

// 0x2ff43c — __ZNSt12_Vector_baseIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EEC2EmRKS3_
#[doc(alias = "std::_Vector_base<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_Vector_base(unsigned long,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper> const&)")]
pub fn stub_2ff43c() -> ! {
    todo!("0x2ff43c std::_Vector_base<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_Vector_base(unsigned long,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper> const&)")
}

// 0x2ff470 — __ZN5boost3_bi5list4INS_3argILi1EEENS0_5valueIPN3RBX8InstanceEEENS4_INS5_14AsyncHttpQueue13RequestResultEEENS4_INS_10shared_ptrISsEEEEEclIPFvNS9_15CallbackWrapperES7_SA_SD_ENS0_5list1IRSH_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string>>>::operator()<void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>),boost::_bi::list1<RBX::AsyncHttpQueue::CallbackWrapper&>>(boost::_bi::type<void>,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boos")]
pub fn stub_2ff470() -> ! {
    todo!("0x2ff470 void boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string>>>::operator()<void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_")
}

// 0x2ff588 — __ZN5boost3_bi5list4INS_3argILi1EEENS0_5valueIPN3RBX8InstanceEEENS4_INS5_14AsyncHttpQueue13RequestResultEEENS4_INS_10shared_ptrISsEEEEEC2ES3_S8_SB_SE_
#[doc(alias = "boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string>>>::list4(boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string>>)")]
pub fn stub_2ff588() {
    // IDA 0x2ff588: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

// 0x2ff674 — __ZNSt4listIN3RBX14AsyncHttpQueue7RequestESaIS2_EE8_M_eraseESt14_List_iteratorIS2_E
#[doc(alias = "std::list<RBX::AsyncHttpQueue::Request,std::allocator<RBX::AsyncHttpQueue::Request>>::_M_erase(std::_List_iterator<RBX::AsyncHttpQueue::Request>)")]
pub fn stub_2ff674() {
    // IDA 0x2ff674: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x2ff758 — __ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE20_M_allocate_and_copyIN9__gnu_cxx17__normal_iteratorIPKS2_S4_EEEEPS2_mT_SC_
#[doc(alias = "RBX::AsyncHttpQueue::CallbackWrapper* std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_allocate_and_copy<__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper const*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>>(unsigned long,__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper const*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::Asy")]
pub fn stub_2ff758() -> ! {
    todo!("0x2ff758 RBX::AsyncHttpQueue::CallbackWrapper* std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_allocate_and_copy<__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper const*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<")
}

// 0x2ff8c0 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN3RBX14AsyncHttpQueue15CallbackWrapperES6_EET0_T_S8_S7_
#[doc(alias = "RBX::AsyncHttpQueue::CallbackWrapper * std::__copy<false,std::random_access_iterator_tag>::copy<RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *>(RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *)")]
pub fn stub_2ff8c0() -> ! {
    todo!("0x2ff8c0 RBX::AsyncHttpQueue::CallbackWrapper * std::__copy<false,std::random_access_iterator_tag>::copy<RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *>(RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *)")
}

// 0x2ff91c — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKN3RBX14AsyncHttpQueue15CallbackWrapperEPS5_EET0_T_SA_S9_
#[doc(alias = "RBX::AsyncHttpQueue::CallbackWrapper* std::__copy<false,std::random_access_iterator_tag>::copy<RBX::AsyncHttpQueue::CallbackWrapper const*,RBX::AsyncHttpQueue::CallbackWrapper*>(RBX::AsyncHttpQueue::CallbackWrapper const*,RBX::AsyncHttpQueue::CallbackWrapper const*,RBX::AsyncHttpQueue::CallbackWrapper*)")]
pub fn stub_2ff91c() -> ! {
    todo!("0x2ff91c RBX::AsyncHttpQueue::CallbackWrapper* std::__copy<false,std::random_access_iterator_tag>::copy<RBX::AsyncHttpQueue::CallbackWrapper const*,RBX::AsyncHttpQueue::CallbackWrapper*>(RBX::AsyncHttpQueue::CallbackWrapper const*,RBX::AsyncHttpQueue::CallbackWrapper const*,RBX::AsyncHttpQueue::CallbackWrapp")
}

// 0x2ff978 — __ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EED2Ev
#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::~vector()")]
pub fn stub_2ff978() {
    // IDA 0x2ff978: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2ffa44 — __ZNSt4listIN3RBX14AsyncHttpQueue9FailedUrlESaIS2_EE14_M_create_nodeERKS2_
#[doc(alias = "std::list<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::_M_create_node(RBX::AsyncHttpQueue::FailedUrl const&)")]
pub fn stub_2ffa44() {
    // IDA 0x2ffa44: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

// 0x2ffb24 — __ZN5boost10shared_ptrIN3RBX4HttpEEC2IS2_EEPT_
#[doc(alias = "rbx_core::SharedPtr<RBX::Http>::shared_ptr<RBX::Http>(RBX::Http *)")]
pub fn stub_2ffb24() -> ! {
    todo!("0x2ffb24 boost::shared_ptr<RBX::Http>::shared_ptr<RBX::Http>(RBX::Http *)")
}

// 0x2ffbfc — __ZN5boost6detail12shared_countC2IN3RBX4HttpEEEPT_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Http>(RBX::Http *)")]
pub fn stub_2ffbfc() {
    // IDA 0x2ffbfc: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}
