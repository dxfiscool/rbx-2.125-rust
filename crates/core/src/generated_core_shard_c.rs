//! core shard C — 120 core stubs EA-sorted after 0x692234.
//! Source: ida/export.json filtered where demangled contains boost|rbx::signals|Ogre, excluding Reflection/Instance/DataModel/Ogre/RakNet/RBX::Network, EA-sorted, next 120 after highest existing EA in generated_*.rs.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, single quotes removed.

#[doc(alias = "RBX::VehicleController::onSteppedTouchInput(rbx_core::SharedPtr<RBX::VehicleSeat>)")]
// 0x692320 — __ZN3RBX17VehicleController19onSteppedTouchInputEN5boost10shared_ptrINS_11VehicleSeatEEE — RBX::VehicleController::onSteppedTouchInput(rbx_core::SharedPtr<RBX::VehicleSeat>)
// was: RBX::VehicleController::onSteppedTouchInput(boost::shared_ptr<RBX::VehicleSeat>)
pub fn stub_0x692320() {
    // IDA 0x692320: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Controller> RBX::shared_from<RBX::Controller>(RBX::Controller*)")]
// 0x693454 — __ZN3RBX11shared_fromINS_10ControllerEEEN5boost10shared_ptrIT_EEPS4_ — rbx_core::SharedPtr<RBX::Controller> RBX::shared_from<RBX::Controller>(RBX::Controller*)
// was: boost::shared_ptr<RBX::Controller> RBX::shared_from<RBX::Controller>(RBX::Controller*)
pub fn stub_0x693454() {
    // IDA 0x693454: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Controller::Button)>::operator()(RBX::Controller::Button)")]
// 0x6936bc — __ZN3rbx7signals16signal_with_argsILi1EFvN3RBX10Controller6ButtonEEEclES4_ — rbx::signals::signal_with_args<1,void ()(RBX::Controller::Button)>::operator()(RBX::Controller::Button)
pub fn stub_0x6936bc() {
    // IDA 0x6936bc: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ButtonBindingWidget>::operator=(rbx_core::SharedPtr<RBX::ButtonBindingWidget> const&)")]
// 0x693800 — __ZN5boost10shared_ptrIN3RBX19ButtonBindingWidgetEEaSERKS3_ — rbx_core::SharedPtr<RBX::ButtonBindingWidget>::operator=(rbx_core::SharedPtr<RBX::ButtonBindingWidget> const&)
// was: boost::shared_ptr<RBX::ButtonBindingWidget>::operator=(boost::shared_ptr<RBX::ButtonBindingWidget> const&)
pub fn stub_0x693800() {
    // IDA 0x693800: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::VehicleSeat> RBX::shared_from<RBX::VehicleSeat>(RBX::VehicleSeat*)")]
// 0x693a90 — __ZN3RBX11shared_fromINS_11VehicleSeatEEEN5boost10shared_ptrIT_EEPS4_ — rbx_core::SharedPtr<RBX::VehicleSeat> RBX::shared_from<RBX::VehicleSeat>(RBX::VehicleSeat*)
// was: boost::shared_ptr<RBX::VehicleSeat> RBX::shared_from<RBX::VehicleSeat>(RBX::VehicleSeat*)
pub fn stub_0x693a90() {
    // IDA 0x693a90: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::system::system_error::~system_error()")]
// 0x697330 — __ZN5boost6system12system_errorD2Ev — boost::system::system_error::~system_error()
pub fn stub_0x697330() {
    // IDA 0x697330: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::VehicleSeat>::shared_ptr<RBX::VehicleSeat>(rbx_core::WeakPtr<RBX::VehicleSeat> const&,boost::detail::sp_nothrow_tag)")]
// 0x697b38 — __ZN5boost10shared_ptrIN3RBX11VehicleSeatEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE — rbx_core::SharedPtr<RBX::VehicleSeat>::shared_ptr<RBX::VehicleSeat>(rbx_core::WeakPtr<RBX::VehicleSeat> const&,boost::detail::sp_nothrow_tag)
// was: boost::shared_ptr<RBX::VehicleSeat>::shared_ptr<RBX::VehicleSeat>(boost::weak_ptr<RBX::VehicleSeat> const&,boost::detail::sp_nothrow_tag)
pub fn stub_0x697b38() {
    // IDA 0x697b38: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Controller::Button)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot> &)")]
// 0x698370 — __ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE — rbx::signals::signal<void ()(RBX::Controller::Button)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot> &)
pub fn stub_0x698370() {
    // IDA 0x698370: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Controller::Button)>::on_error(std::exception &)")]
// 0x6984d0 — __ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE8on_errorERSt9exception — rbx::signals::signal<void ()(RBX::Controller::Button)>::on_error(std::exception &)
pub fn stub_0x6984d0() {
    // IDA 0x6984d0: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot> const&)")]
// 0x6984f8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE4slotEEaSERKSA_ — boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot> const&)
pub fn stub_0x6984f8() {
    // IDA 0x6984f8: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Controller::Button)>::safe_static_init_mutex(void)")]
// 0x69851c — __ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE22safe_static_init_mutexEv — rbx::signals::signal<void ()(RBX::Controller::Button)>::safe_static_init_mutex(void)
pub fn stub_0x69851c() {
    // IDA 0x69851c: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Controller::Button)>::safe_static_do_get_mutex(void)")]
// 0x698520 — __ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE24safe_static_do_get_mutexEv — rbx::signals::signal<void ()(RBX::Controller::Button)>::safe_static_do_get_mutex(void)
pub fn stub_0x698520() {
    // IDA 0x698520: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>> *,boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>> *)")]
// 0x69861c — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE11erase_nodesEPNS1_8ptr_nodeISA_EESK_ — boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>> *,boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>> *)
pub fn stub_0x69861c() {
    // IDA 0x69861c: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
// 0x698678 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE11delete_nodeEPNS1_10ptr_bucketE — boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::delete_node(boost::unordered::detail::ptr_bucket *)
pub fn stub_0x698678() {
    // IDA 0x698678: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")]
// 0x698758 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE10fix_bucketEmPNS1_10ptr_bucketE — boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)
pub fn stub_0x698758() {
    // IDA 0x698758: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::find_node_impl<RBX::Controller::Button,std::equal_to<RBX::Controller::Button>>(unsigned long,RBX::Controller::Button const&,std::equal_to<RBX::Controller::Button> const&)const")]
// 0x698798 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE14find_node_implIS7_SF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEmRKT_RKT0_ — boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::find_node_impl<RBX::Controller::Button,std::equal_to<RBX::Controller::Button>>(unsigned long,RBX::Controller::Button const&,std::equal_to<RBX::Controller::Button> const&)const
pub fn stub_0x698798() {
    // IDA 0x698798: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>>(RBX::Controller::Button const&,boost::unordered::detail::emplace_args1<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>> const&)")]
// 0x698804 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE12emplace_implINS1_13emplace_args1ISA_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEbERS8_RKT_ — std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>>(RBX::Controller::Button const&,boost::unordered::detail::emplace_args1<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>> const&)
pub fn stub_0x698804() {
    // IDA 0x698804: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::reserve_for_insert(unsigned long)")]
// 0x6989f0 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE18reserve_for_insertEm — boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::reserve_for_insert(unsigned long)
pub fn stub_0x6989f0() {
    // IDA 0x6989f0: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>>>::~node_constructor()")]
// 0x698a40 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEEEEED2Ev — boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>>>::~node_constructor()
pub fn stub_0x698a40() {
    // IDA 0x698a40: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::create_buckets(unsigned long)")]
// 0x698b18 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm — boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::create_buckets(unsigned long)
pub fn stub_0x698b18() {
    // IDA 0x698b18: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::min_buckets_for_size(unsigned long)const")]
// 0x698c40 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE20min_buckets_for_sizeEm — boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::min_buckets_for_size(unsigned long)const
pub fn stub_0x698c40() {
    // IDA 0x698c40: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::rehash_impl(unsigned long)")]
// 0x698cd0 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE11rehash_implEm — boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::rehash_impl(unsigned long)
pub fn stub_0x698cd0() {
    // IDA 0x698cd0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>> &,boost::unordered::detail::ptr_bucket *)")]
// 0x698cfc — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE — boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>> &,boost::unordered::detail::ptr_bucket *)
pub fn stub_0x698cfc() {
    // IDA 0x698cfc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>>>::construct(void)")]
// 0x698d54 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEEEEE9constructEv — boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>>>::construct(void)
pub fn stub_0x698d54() {
    // IDA 0x698d54: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::delete_buckets(void)")]
// 0x698e80 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE14delete_bucketsEv — boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::delete_buckets(void)
pub fn stub_0x698e80() {
    // IDA 0x698e80: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::table(unsigned long,boost::hash<RBX::Controller::Button> const&,std::equal_to<RBX::Controller::Button> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>> const&)")]
// 0x698eb8 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEEC2EmRKSD_RKSF_RKSaINS1_8ptr_nodeISA_EEE — boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>,RBX::Controller::Button,RBX::Controller::BoundButton,boost::hash<RBX::Controller::Button>,std::equal_to<RBX::Controller::Button>>>::table(unsigned long,boost::hash<RBX::Controller::Button> const&,std::equal_to<RBX::Controller::Button> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Controller::Button const,RBX::Controller::BoundButton>>> const&)
pub fn stub_0x698eb8() {
    // IDA 0x698eb8: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Controller>::shared_ptr<RBX::Controller>(rbx_core::WeakPtr<RBX::Controller> const&,boost::detail::sp_nothrow_tag)")]
// 0x6992a4 — __ZN5boost10shared_ptrIN3RBX10ControllerEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE — rbx_core::SharedPtr<RBX::Controller>::shared_ptr<RBX::Controller>(rbx_core::WeakPtr<RBX::Controller> const&,boost::detail::sp_nothrow_tag)
// was: boost::shared_ptr<RBX::Controller>::shared_ptr<RBX::Controller>(boost::weak_ptr<RBX::Controller> const&,boost::detail::sp_nothrow_tag)
pub fn stub_0x6992a4() {
    // IDA 0x6992a4: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Controller::Button)>::disconnectAll(void)")]
// 0x69a558 — __ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE13disconnectAllEv — rbx::signals::signal<void ()(RBX::Controller::Button)>::disconnectAll(void)
pub fn stub_0x69a558() {
    // IDA 0x69a558: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "boost::function1<void,RBX::Controller::Button>::clear(void)")]
// 0x69a930 — __ZN5boost9function1IvN3RBX10Controller6ButtonEE5clearEv — boost::function1<void,RBX::Controller::Button>::clear(void)
pub fn stub_0x69a930() {
    // IDA 0x69a930: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Controller::Button)>::connect<boost::function<void ()(RBX::Controller::Button)>>(boost::function<void ()(RBX::Controller::Button)> const&)")]
// 0x69b060 — __ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_ — rbx::signals::connection rbx::signals::signal<void ()(RBX::Controller::Button)>::connect<boost::function<void ()(RBX::Controller::Button)>>(boost::function<void ()(RBX::Controller::Button)> const&)
pub fn stub_0x69b060() {
    // IDA 0x69b060: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Controller::Button)>::insert(rbx::signals::signal<void ()(RBX::Controller::Button)>::slot *)")]
// 0x69b154 — __ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE6insertEPNS6_4slotE — rbx::signals::signal<void ()(RBX::Controller::Button)>::insert(rbx::signals::signal<void ()(RBX::Controller::Button)>::slot *)
pub fn stub_0x69b154() {
    // IDA 0x69b154: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Controller::Button)>::slot*)")]
// 0x69b360 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE4slotEEaSEPS9_ — boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Controller::Button)>::slot*)
pub fn stub_0x69b360() {
    // IDA 0x69b360: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot,boost::function<void ()(RBX::Controller::Button)>,1,void ()(RBX::Controller::Button)>::callable<rbx::signals::signal<void ()(RBX::Controller::Button)>*>(boost::function<void ()(RBX::Controller::Button)> const&,rbx::signals::signal<void ()(RBX::Controller::Button)>*)")]
// 0x69b384 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX10Controller6ButtonEEE4slotEN5boost8functionIS6_EELi1ES6_EC2IPS7_EERKSB_T_ — rbx::callable<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot,boost::function<void ()(RBX::Controller::Button)>,1,void ()(RBX::Controller::Button)>::callable<rbx::signals::signal<void ()(RBX::Controller::Button)>*>(boost::function<void ()(RBX::Controller::Button)> const&,rbx::signals::signal<void ()(RBX::Controller::Button)>*)
pub fn stub_0x69b384() {
    // IDA 0x69b384: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Controller::Button)>::callable_slot<boost::function<void ()(RBX::Controller::Button)>>::~callable_slot()")]
// 0x69b480 — __ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE13callable_slotIN5boost8functionIS5_EEED1Ev — rbx::signals::signal<void ()(RBX::Controller::Button)>::callable_slot<boost::function<void ()(RBX::Controller::Button)>>::~callable_slot()
pub fn stub_0x69b480() {
    // IDA 0x69b480: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Controller::Button)>::callable_slot<boost::function<void ()(RBX::Controller::Button)>>::~callable_slot()")]
// 0x69b590 — __ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE13callable_slotIN5boost8functionIS5_EEED0Ev — rbx::signals::signal<void ()(RBX::Controller::Button)>::callable_slot<boost::function<void ()(RBX::Controller::Button)>>::~callable_slot()
pub fn stub_0x69b590() {
    // IDA 0x69b590: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Controller::Button)>::slot::disconnect(void)")]
// 0x69b6c0 — __ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE4slot10disconnectEv — rbx::signals::signal<void ()(RBX::Controller::Button)>::slot::disconnect(void)
pub fn stub_0x69b6c0() {
    // IDA 0x69b6c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Controller::Button)>::slot::connected(void)const")]
// 0x69b7d0 — __ZNK3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE4slot9connectedEv — rbx::signals::signal<void ()(RBX::Controller::Button)>::slot::connected(void)const
pub fn stub_0x69b7d0() {
    // IDA 0x69b7d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot,boost::function<void ()(RBX::Controller::Button)>,1,void ()(RBX::Controller::Button)>::call(RBX::Controller::Button)")]
// 0x69b7dc — __ZN3rbx8callableINS_7signals6signalIFvN3RBX10Controller6ButtonEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_ — rbx::callable<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot,boost::function<void ()(RBX::Controller::Button)>,1,void ()(RBX::Controller::Button)>::call(RBX::Controller::Button)
pub fn stub_0x69b7dc() {
    // IDA 0x69b7dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot,boost::function<void ()(RBX::Controller::Button)>,1,void ()(RBX::Controller::Button)>::call(RBX::Controller::Button)")]
// 0x69b7e4 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX10Controller6ButtonEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_ — non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot,boost::function<void ()(RBX::Controller::Button)>,1,void ()(RBX::Controller::Button)>::call(RBX::Controller::Button)
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot,boost::function<void ()(RBX::Controller::Button)>,1,void ()(RBX::Controller::Button)>::call(RBX::Controller::Button)
pub fn stub_0x69b7e4() {
    // IDA 0x69b7e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function1<void,RBX::Controller::Button>::operator()(RBX::Controller::Button)const")]
// 0x69b7ec — __ZNK5boost9function1IvN3RBX10Controller6ButtonEEclES3_ — boost::function1<void,RBX::Controller::Button>::operator()(RBX::Controller::Button)const
pub fn stub_0x69b7ec() {
    // IDA 0x69b7ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Controller::Button)>::remove(rbx::signals::signal<void ()(RBX::Controller::Button)>::slot *)")]
// 0x69b8b0 — __ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE6removeEPNS6_4slotE — rbx::signals::signal<void ()(RBX::Controller::Button)>::remove(rbx::signals::signal<void ()(RBX::Controller::Button)>::slot *)
pub fn stub_0x69b8b0() {
    // IDA 0x69b8b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Controller::Button)>::slot::safe_static_init_mutex(void)")]
// 0x69b9a0 — __ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE4slot22safe_static_init_mutexEv — rbx::signals::signal<void ()(RBX::Controller::Button)>::slot::safe_static_init_mutex(void)
pub fn stub_0x69b9a0() {
    // IDA 0x69b9a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Controller::Button)>::slot::safe_static_do_get_mutex(void)")]
// 0x69b9a4 — __ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE4slot24safe_static_do_get_mutexEv — rbx::signals::signal<void ()(RBX::Controller::Button)>::slot::safe_static_do_get_mutex(void)
pub fn stub_0x69b9a4() {
    // IDA 0x69b9a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot,boost::function<void ()(RBX::Controller::Button)>,1,void ()(RBX::Controller::Button)>::~callable()")]
// 0x69ba98 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX10Controller6ButtonEEE4slotEN5boost8functionIS6_EELi1ES6_ED1Ev — rbx::callable<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot,boost::function<void ()(RBX::Controller::Button)>,1,void ()(RBX::Controller::Button)>::~callable()
pub fn stub_0x69ba98() {
    // IDA 0x69ba98: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot,boost::function<void ()(RBX::Controller::Button)>,1,void ()(RBX::Controller::Button)>::~callable()")]
// 0x69bba8 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX10Controller6ButtonEEE4slotEN5boost8functionIS6_EELi1ES6_ED0Ev — rbx::callable<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot,boost::function<void ()(RBX::Controller::Button)>,1,void ()(RBX::Controller::Button)>::~callable()
pub fn stub_0x69bba8() {
    // IDA 0x69bba8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Controller::Button)>::slot::~slot()")]
// 0x69bcd8 — __ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE4slotD1Ev — rbx::signals::signal<void ()(RBX::Controller::Button)>::slot::~slot()
pub fn stub_0x69bcd8() {
    // IDA 0x69bcd8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Controller::Button)>::slot::~slot()")]
// 0x69bd04 — __ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE4slotD0Ev — rbx::signals::signal<void ()(RBX::Controller::Button)>::slot::~slot()
pub fn stub_0x69bd04() {
    // IDA 0x69bd04: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function1<void,RBX::Controller::Button>::assign_to_own(boost::function1<void,RBX::Controller::Button> const&)")]
// 0x69bdd8 — __ZN5boost9function1IvN3RBX10Controller6ButtonEE13assign_to_ownERKS4_ — boost::function1<void,RBX::Controller::Button>::assign_to_own(boost::function1<void,RBX::Controller::Button> const&)
pub fn stub_0x69bdd8() {
    // IDA 0x69bdd8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::RbxRay)>::disconnectAll(void)")]
// 0x6a1c5c — __ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE13disconnectAllEv — rbx::signals::signal<void ()(RBX::RbxRay)>::disconnectAll(void)
pub fn stub_0x6a1c5c() {
    // IDA 0x6a1c5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::RbxRay)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::RbxRay)>::slot> const&)")]
// 0x6a1dd4 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX6RbxRayEEE4slotEEaSERKS9_ — boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::RbxRay)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::RbxRay)>::slot> const&)
pub fn stub_0x6a1dd4() {
    // IDA 0x6a1dd4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::RbxRay)>::safe_static_init_mutex(void)")]
// 0x6a1df8 — __ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE22safe_static_init_mutexEv — rbx::signals::signal<void ()(RBX::RbxRay)>::safe_static_init_mutex(void)
pub fn stub_0x6a1df8() {
    // IDA 0x6a1df8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::RbxRay)>::safe_static_do_get_mutex(void)")]
// 0x6a1dfc — __ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE24safe_static_do_get_mutexEv — rbx::signals::signal<void ()(RBX::RbxRay)>::safe_static_do_get_mutex(void)
pub fn stub_0x6a1dfc() {
    // IDA 0x6a1dfc: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::BrickColor)>::disconnectAll(void)")]
// 0x6a335c — __ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE13disconnectAllEv — rbx::signals::signal<void ()(RBX::BrickColor)>::disconnectAll(void)
pub fn stub_0x6a335c() {
    // IDA 0x6a335c: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::BrickColor)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::BrickColor)>::slot> const&)")]
// 0x6a34d4 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX10BrickColorEEE4slotEEaSERKS9_ — boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::BrickColor)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::BrickColor)>::slot> const&)
pub fn stub_0x6a34d4() {
    // IDA 0x6a34d4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::BrickColor)>::safe_static_init_mutex(void)")]
// 0x6a34f8 — __ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE22safe_static_init_mutexEv — rbx::signals::signal<void ()(RBX::BrickColor)>::safe_static_init_mutex(void)
pub fn stub_0x6a34f8() {
    // IDA 0x6a34f8: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::BrickColor)>::safe_static_do_get_mutex(void)")]
// 0x6a34fc — __ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE24safe_static_do_get_mutexEv — rbx::signals::signal<void ()(RBX::BrickColor)>::safe_static_do_get_mutex(void)
pub fn stub_0x6a34fc() {
    // IDA 0x6a34fc: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Color3)>::disconnectAll(void)")]
// 0x6a4a78 — __ZN3rbx7signals6signalIFvN3G3D6Color3EEE13disconnectAllEv — rbx::signals::signal<void ()(G3D::Color3)>::disconnectAll(void)
pub fn stub_0x6a4a78() {
    // IDA 0x6a4a78: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Color3)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Color3)>::slot> const&)")]
// 0x6a4bf0 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3G3D6Color3EEE4slotEEaSERKS9_ — boost::intrusive_ptr<rbx::signals::signal<void ()(G3D::Color3)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(G3D::Color3)>::slot> const&)
pub fn stub_0x6a4bf0() {
    // IDA 0x6a4bf0: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Color3)>::safe_static_init_mutex(void)")]
// 0x6a4c14 — __ZN3rbx7signals6signalIFvN3G3D6Color3EEE22safe_static_init_mutexEv — rbx::signals::signal<void ()(G3D::Color3)>::safe_static_init_mutex(void)
pub fn stub_0x6a4c14() {
    // IDA 0x6a4c14: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Color3)>::safe_static_do_get_mutex(void)")]
// 0x6a4c18 — __ZN3rbx7signals6signalIFvN3G3D6Color3EEE24safe_static_do_get_mutexEv — rbx::signals::signal<void ()(G3D::Color3)>::safe_static_do_get_mutex(void)
pub fn stub_0x6a4c18() {
    // IDA 0x6a4c18: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::CoordinateFrame)>::disconnectAll(void)")]
// 0x6a61b4 — __ZN3rbx7signals6signalIFvN3G3D15CoordinateFrameEEE13disconnectAllEv — rbx::signals::signal<void ()(G3D::CoordinateFrame)>::disconnectAll(void)
pub fn stub_0x6a61b4() {
    // IDA 0x6a61b4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::CoordinateFrame)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::CoordinateFrame)>::slot> const&)")]
// 0x6a632c — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3G3D15CoordinateFrameEEE4slotEEaSERKS9_ — boost::intrusive_ptr<rbx::signals::signal<void ()(G3D::CoordinateFrame)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(G3D::CoordinateFrame)>::slot> const&)
pub fn stub_0x6a632c() {
    // IDA 0x6a632c: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::CoordinateFrame)>::safe_static_init_mutex(void)")]
// 0x6a6350 — __ZN3rbx7signals6signalIFvN3G3D15CoordinateFrameEEE22safe_static_init_mutexEv — rbx::signals::signal<void ()(G3D::CoordinateFrame)>::safe_static_init_mutex(void)
pub fn stub_0x6a6350() {
    // IDA 0x6a6350: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::CoordinateFrame)>::safe_static_do_get_mutex(void)")]
// 0x6a6354 — __ZN3rbx7signals6signalIFvN3G3D15CoordinateFrameEEE24safe_static_do_get_mutexEv — rbx::signals::signal<void ()(G3D::CoordinateFrame)>::safe_static_do_get_mutex(void)
pub fn stub_0x6a6354() {
    // IDA 0x6a6354: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::disconnectAll(void)")]
// 0x6a78bc — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE13disconnectAllEv — rbx::signals::signal<void ()(G3D::Vector3)>::disconnectAll(void)
pub fn stub_0x6a78bc() {
    // IDA 0x6a78bc: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector3)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector3)>::slot> const&)")]
// 0x6a7a34 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3G3D7Vector3EEE4slotEEaSERKS9_ — boost::intrusive_ptr<rbx::signals::signal<void ()(G3D::Vector3)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(G3D::Vector3)>::slot> const&)
pub fn stub_0x6a7a34() {
    // IDA 0x6a7a34: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::safe_static_do_get_mutex(void)")]
// 0x6a7a58 — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE24safe_static_do_get_mutexEv — rbx::signals::signal<void ()(G3D::Vector3)>::safe_static_do_get_mutex(void)
pub fn stub_0x6a7a58() {
    // IDA 0x6a7a58: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(int)>::operator()(int)")]
// 0x6ad758 — __ZN3rbx7signals16signal_with_argsILi1EFviEEclEi — rbx::signals::signal_with_args<1,void ()(int)>::operator()(int)
pub fn stub_0x6ad758() {
    // IDA 0x6ad758: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(int)>::slot> &)")]
// 0x6ad89c — __ZN3rbx7signals6signalIFviEE4nextERN5boost13intrusive_ptrINS3_4slotEEE — rbx::signals::signal<void ()(int)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(int)>::slot> &)
pub fn stub_0x6ad89c() {
    // IDA 0x6ad89c: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int)>::on_error(std::exception &)")]
// 0x6ad9fc — __ZN3rbx7signals6signalIFviEE8on_errorERSt9exception — rbx::signals::signal<void ()(int)>::on_error(std::exception &)
pub fn stub_0x6ad9fc() {
    // IDA 0x6ad9fc: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int)>::slot,boost::function<void ()(int)>,1,void ()(int)>::callable<rbx::signals::signal<void ()(int)>*>(boost::function<void ()(int)> const&,rbx::signals::signal<void ()(int)>*)")]
// 0x6add34 — __ZN3rbx8callableINS_7signals6signalIFviEE4slotEN5boost8functionIS3_EELi1ES3_EC2IPS4_EERKS8_T_ — rbx::callable<rbx::signals::signal<void ()(int)>::slot,boost::function<void ()(int)>,1,void ()(int)>::callable<rbx::signals::signal<void ()(int)>*>(boost::function<void ()(int)> const&,rbx::signals::signal<void ()(int)>*)
pub fn stub_0x6add34() {
    // IDA 0x6add34: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int)>::slot,boost::function<void ()(int)>,1,void ()(int)>::call(int)")]
// 0x6ade30 — __ZN3rbx8callableINS_7signals6signalIFviEE4slotEN5boost8functionIS3_EELi1ES3_E4callEi — rbx::callable<rbx::signals::signal<void ()(int)>::slot,boost::function<void ()(int)>,1,void ()(int)>::call(int)
pub fn stub_0x6ade30() {
    // IDA 0x6ade30: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(int)>::slot,boost::function<void ()(int)>,1,void ()(int)>::call(int)")]
// 0x6ade38 — __ZThn4_N3rbx8callableINS_7signals6signalIFviEE4slotEN5boost8functionIS3_EELi1ES3_E4callEi — non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(int)>::slot,boost::function<void ()(int)>,1,void ()(int)>::call(int)
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(int)>::slot,boost::function<void ()(int)>,1,void ()(int)>::call(int)
pub fn stub_0x6ade38() {
    // IDA 0x6ade38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int)>::slot,boost::function<void ()(int)>,1,void ()(int)>::~callable()")]
// 0x6ade40 — __ZN3rbx8callableINS_7signals6signalIFviEE4slotEN5boost8functionIS3_EELi1ES3_ED1Ev — rbx::callable<rbx::signals::signal<void ()(int)>::slot,boost::function<void ()(int)>,1,void ()(int)>::~callable()
pub fn stub_0x6ade40() {
    // IDA 0x6ade40: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int)>::slot,boost::function<void ()(int)>,1,void ()(int)>::~callable()")]
// 0x6adf50 — __ZN3rbx8callableINS_7signals6signalIFviEE4slotEN5boost8functionIS3_EELi1ES3_ED0Ev — rbx::callable<rbx::signals::signal<void ()(int)>::slot,boost::function<void ()(int)>,1,void ()(int)>::~callable()
pub fn stub_0x6adf50() {
    // IDA 0x6adf50: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::BrickColor)>::operator()(RBX::BrickColor)")]
// 0x6ae9b0 — __ZN3rbx7signals16signal_with_argsILi1EFvN3RBX10BrickColorEEEclES3_ — rbx::signals::signal_with_args<1,void ()(RBX::BrickColor)>::operator()(RBX::BrickColor)
pub fn stub_0x6ae9b0() {
    // IDA 0x6ae9b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::BrickColor)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::BrickColor)>::slot> &)")]
// 0x6aeaf4 — __ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE4nextERN5boost13intrusive_ptrINS5_4slotEEE — rbx::signals::signal<void ()(RBX::BrickColor)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::BrickColor)>::slot> &)
pub fn stub_0x6aeaf4() {
    // IDA 0x6aeaf4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::BrickColor)>::on_error(std::exception &)")]
// 0x6aec54 — __ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE8on_errorERSt9exception — rbx::signals::signal<void ()(RBX::BrickColor)>::on_error(std::exception &)
pub fn stub_0x6aec54() {
    // IDA 0x6aec54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function1<void,RBX::BrickColor>::clear(void)")]
// 0x6aeedc — __ZN5boost9function1IvN3RBX10BrickColorEE5clearEv — boost::function1<void,RBX::BrickColor>::clear(void)
pub fn stub_0x6aeedc() {
    // IDA 0x6aeedc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::BrickColor)>::connect<boost::function<void ()(RBX::BrickColor)>>(boost::function<void ()(RBX::BrickColor)> const&)")]
// 0x6af60c — __ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_ — rbx::signals::connection rbx::signals::signal<void ()(RBX::BrickColor)>::connect<boost::function<void ()(RBX::BrickColor)>>(boost::function<void ()(RBX::BrickColor)> const&)
pub fn stub_0x6af60c() {
    // IDA 0x6af60c: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::BrickColor)>::insert(rbx::signals::signal<void ()(RBX::BrickColor)>::slot *)")]
// 0x6af700 — __ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE6insertEPNS5_4slotE — rbx::signals::signal<void ()(RBX::BrickColor)>::insert(rbx::signals::signal<void ()(RBX::BrickColor)>::slot *)
pub fn stub_0x6af700() {
    // IDA 0x6af700: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::BrickColor)>::slot>::operator=(rbx::signals::signal<void ()(RBX::BrickColor)>::slot*)")]
// 0x6af90c — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX10BrickColorEEE4slotEEaSEPS8_ — boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::BrickColor)>::slot>::operator=(rbx::signals::signal<void ()(RBX::BrickColor)>::slot*)
pub fn stub_0x6af90c() {
    // IDA 0x6af90c: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::BrickColor)>::slot,boost::function<void ()(RBX::BrickColor)>,1,void ()(RBX::BrickColor)>::callable<rbx::signals::signal<void ()(RBX::BrickColor)>*>(boost::function<void ()(RBX::BrickColor)> const&,rbx::signals::signal<void ()(RBX::BrickColor)>*)")]
// 0x6af930 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX10BrickColorEEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_ — rbx::callable<rbx::signals::signal<void ()(RBX::BrickColor)>::slot,boost::function<void ()(RBX::BrickColor)>,1,void ()(RBX::BrickColor)>::callable<rbx::signals::signal<void ()(RBX::BrickColor)>*>(boost::function<void ()(RBX::BrickColor)> const&,rbx::signals::signal<void ()(RBX::BrickColor)>*)
pub fn stub_0x6af930() {
    // IDA 0x6af930: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::BrickColor)>::callable_slot<boost::function<void ()(RBX::BrickColor)>>::~callable_slot()")]
// 0x6afa2c — __ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE13callable_slotIN5boost8functionIS4_EEED1Ev — rbx::signals::signal<void ()(RBX::BrickColor)>::callable_slot<boost::function<void ()(RBX::BrickColor)>>::~callable_slot()
pub fn stub_0x6afa2c() {
    // IDA 0x6afa2c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::BrickColor)>::callable_slot<boost::function<void ()(RBX::BrickColor)>>::~callable_slot()")]
// 0x6afb3c — __ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE13callable_slotIN5boost8functionIS4_EEED0Ev — rbx::signals::signal<void ()(RBX::BrickColor)>::callable_slot<boost::function<void ()(RBX::BrickColor)>>::~callable_slot()
pub fn stub_0x6afb3c() {
    // IDA 0x6afb3c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::BrickColor)>::slot::disconnect(void)")]
// 0x6afc6c — __ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE4slot10disconnectEv — rbx::signals::signal<void ()(RBX::BrickColor)>::slot::disconnect(void)
pub fn stub_0x6afc6c() {
    // IDA 0x6afc6c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::BrickColor)>::slot::connected(void)const")]
// 0x6afd7c — __ZNK3rbx7signals6signalIFvN3RBX10BrickColorEEE4slot9connectedEv — rbx::signals::signal<void ()(RBX::BrickColor)>::slot::connected(void)const
pub fn stub_0x6afd7c() {
    // IDA 0x6afd7c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::BrickColor)>::slot,boost::function<void ()(RBX::BrickColor)>,1,void ()(RBX::BrickColor)>::call(RBX::BrickColor)")]
// 0x6afd88 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX10BrickColorEEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_ — rbx::callable<rbx::signals::signal<void ()(RBX::BrickColor)>::slot,boost::function<void ()(RBX::BrickColor)>,1,void ()(RBX::BrickColor)>::call(RBX::BrickColor)
pub fn stub_0x6afd88() {
    // IDA 0x6afd88: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::BrickColor)>::slot,boost::function<void ()(RBX::BrickColor)>,1,void ()(RBX::BrickColor)>::call(RBX::BrickColor)")]
// 0x6afd90 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX10BrickColorEEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_ — non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::BrickColor)>::slot,boost::function<void ()(RBX::BrickColor)>,1,void ()(RBX::BrickColor)>::call(RBX::BrickColor)
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::BrickColor)>::slot,boost::function<void ()(RBX::BrickColor)>,1,void ()(RBX::BrickColor)>::call(RBX::BrickColor)
pub fn stub_0x6afd90() {
    // IDA 0x6afd90: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function1<void,RBX::BrickColor>::operator()(RBX::BrickColor)const")]
// 0x6afd98 — __ZNK5boost9function1IvN3RBX10BrickColorEEclES2_ — boost::function1<void,RBX::BrickColor>::operator()(RBX::BrickColor)const
pub fn stub_0x6afd98() {
    // IDA 0x6afd98: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::BrickColor)>::remove(rbx::signals::signal<void ()(RBX::BrickColor)>::slot *)")]
// 0x6afe5c — __ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE6removeEPNS5_4slotE — rbx::signals::signal<void ()(RBX::BrickColor)>::remove(rbx::signals::signal<void ()(RBX::BrickColor)>::slot *)
pub fn stub_0x6afe5c() {
    // IDA 0x6afe5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::BrickColor)>::slot::safe_static_init_mutex(void)")]
// 0x6aff4c — __ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE4slot22safe_static_init_mutexEv — rbx::signals::signal<void ()(RBX::BrickColor)>::slot::safe_static_init_mutex(void)
pub fn stub_0x6aff4c() {
    // IDA 0x6aff4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::BrickColor)>::slot::safe_static_do_get_mutex(void)")]
// 0x6aff50 — __ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE4slot24safe_static_do_get_mutexEv — rbx::signals::signal<void ()(RBX::BrickColor)>::slot::safe_static_do_get_mutex(void)
pub fn stub_0x6aff50() {
    // IDA 0x6aff50: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::BrickColor)>::slot,boost::function<void ()(RBX::BrickColor)>,1,void ()(RBX::BrickColor)>::~callable()")]
// 0x6b0040 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX10BrickColorEEE4slotEN5boost8functionIS5_EELi1ES5_ED1Ev — rbx::callable<rbx::signals::signal<void ()(RBX::BrickColor)>::slot,boost::function<void ()(RBX::BrickColor)>,1,void ()(RBX::BrickColor)>::~callable()
pub fn stub_0x6b0040() {
    // IDA 0x6b0040: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::BrickColor)>::slot,boost::function<void ()(RBX::BrickColor)>,1,void ()(RBX::BrickColor)>::~callable()")]
// 0x6b0150 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX10BrickColorEEE4slotEN5boost8functionIS5_EELi1ES5_ED0Ev — rbx::callable<rbx::signals::signal<void ()(RBX::BrickColor)>::slot,boost::function<void ()(RBX::BrickColor)>,1,void ()(RBX::BrickColor)>::~callable()
pub fn stub_0x6b0150() {
    // IDA 0x6b0150: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::BrickColor)>::slot::~slot()")]
// 0x6b0280 — __ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE4slotD1Ev — rbx::signals::signal<void ()(RBX::BrickColor)>::slot::~slot()
pub fn stub_0x6b0280() {
    // IDA 0x6b0280: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::BrickColor)>::slot::~slot()")]
// 0x6b02ac — __ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE4slotD0Ev — rbx::signals::signal<void ()(RBX::BrickColor)>::slot::~slot()
pub fn stub_0x6b02ac() {
    // IDA 0x6b02ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function1<void,RBX::BrickColor>::assign_to_own(boost::function1<void,RBX::BrickColor> const&)")]
// 0x6b0380 — __ZN5boost9function1IvN3RBX10BrickColorEE13assign_to_ownERKS3_ — boost::function1<void,RBX::BrickColor>::assign_to_own(boost::function1<void,RBX::BrickColor> const&)
pub fn stub_0x6b0380() {
    // IDA 0x6b0380: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(G3D::Color3)>::operator()(G3D::Color3)")]
// 0x6b0b90 — __ZN3rbx7signals16signal_with_argsILi1EFvN3G3D6Color3EEEclES3_ — rbx::signals::signal_with_args<1,void ()(G3D::Color3)>::operator()(G3D::Color3)
pub fn stub_0x6b0b90() {
    // IDA 0x6b0b90: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Color3)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Color3)>::slot> &)")]
// 0x6b0cf8 — __ZN3rbx7signals6signalIFvN3G3D6Color3EEE4nextERN5boost13intrusive_ptrINS5_4slotEEE — rbx::signals::signal<void ()(G3D::Color3)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(G3D::Color3)>::slot> &)
pub fn stub_0x6b0cf8() {
    // IDA 0x6b0cf8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Color3)>::on_error(std::exception &)")]
// 0x6b0e58 — __ZN3rbx7signals6signalIFvN3G3D6Color3EEE8on_errorERSt9exception — rbx::signals::signal<void ()(G3D::Color3)>::on_error(std::exception &)
pub fn stub_0x6b0e58() {
    // IDA 0x6b0e58: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function1<void,G3D::Color3>::clear(void)")]
// 0x6b10e0 — __ZN5boost9function1IvN3G3D6Color3EE5clearEv — boost::function1<void,G3D::Color3>::clear(void)
pub fn stub_0x6b10e0() {
    // IDA 0x6b10e0: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(G3D::Color3)>::connect<boost::function<void ()(G3D::Color3)>>(boost::function<void ()(G3D::Color3)> const&)")]
// 0x6b1804 — __ZN3rbx7signals6signalIFvN3G3D6Color3EEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_ — rbx::signals::connection rbx::signals::signal<void ()(G3D::Color3)>::connect<boost::function<void ()(G3D::Color3)>>(boost::function<void ()(G3D::Color3)> const&)
pub fn stub_0x6b1804() {
    // IDA 0x6b1804: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Color3)>::insert(rbx::signals::signal<void ()(G3D::Color3)>::slot *)")]
// 0x6b18f8 — __ZN3rbx7signals6signalIFvN3G3D6Color3EEE6insertEPNS5_4slotE — rbx::signals::signal<void ()(G3D::Color3)>::insert(rbx::signals::signal<void ()(G3D::Color3)>::slot *)
pub fn stub_0x6b18f8() {
    // IDA 0x6b18f8: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Color3)>::slot>::operator=(rbx::signals::signal<void ()(G3D::Color3)>::slot*)")]
// 0x6b1b04 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3G3D6Color3EEE4slotEEaSEPS8_ — boost::intrusive_ptr<rbx::signals::signal<void ()(G3D::Color3)>::slot>::operator=(rbx::signals::signal<void ()(G3D::Color3)>::slot*)
pub fn stub_0x6b1b04() {
    // IDA 0x6b1b04: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Color3)>::slot,boost::function<void ()(G3D::Color3)>,1,void ()(G3D::Color3)>::callable<rbx::signals::signal<void ()(G3D::Color3)>*>(boost::function<void ()(G3D::Color3)> const&,rbx::signals::signal<void ()(G3D::Color3)>*)")]
// 0x6b1b28 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D6Color3EEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_ — rbx::callable<rbx::signals::signal<void ()(G3D::Color3)>::slot,boost::function<void ()(G3D::Color3)>,1,void ()(G3D::Color3)>::callable<rbx::signals::signal<void ()(G3D::Color3)>*>(boost::function<void ()(G3D::Color3)> const&,rbx::signals::signal<void ()(G3D::Color3)>*)
pub fn stub_0x6b1b28() {
    // IDA 0x6b1b28: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Color3)>::callable_slot<boost::function<void ()(G3D::Color3)>>::~callable_slot()")]
// 0x6b1c24 — __ZN3rbx7signals6signalIFvN3G3D6Color3EEE13callable_slotIN5boost8functionIS4_EEED1Ev — rbx::signals::signal<void ()(G3D::Color3)>::callable_slot<boost::function<void ()(G3D::Color3)>>::~callable_slot()
pub fn stub_0x6b1c24() {
    // IDA 0x6b1c24: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Color3)>::callable_slot<boost::function<void ()(G3D::Color3)>>::~callable_slot()")]
// 0x6b1d34 — __ZN3rbx7signals6signalIFvN3G3D6Color3EEE13callable_slotIN5boost8functionIS4_EEED0Ev — rbx::signals::signal<void ()(G3D::Color3)>::callable_slot<boost::function<void ()(G3D::Color3)>>::~callable_slot()
pub fn stub_0x6b1d34() {
    // IDA 0x6b1d34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Color3)>::slot::disconnect(void)")]
// 0x6b1e64 — __ZN3rbx7signals6signalIFvN3G3D6Color3EEE4slot10disconnectEv — rbx::signals::signal<void ()(G3D::Color3)>::slot::disconnect(void)
pub fn stub_0x6b1e64() {
    // IDA 0x6b1e64: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Color3)>::slot::connected(void)const")]
// 0x6b1f74 — __ZNK3rbx7signals6signalIFvN3G3D6Color3EEE4slot9connectedEv — rbx::signals::signal<void ()(G3D::Color3)>::slot::connected(void)const
pub fn stub_0x6b1f74() {
    // IDA 0x6b1f74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Color3)>::slot,boost::function<void ()(G3D::Color3)>,1,void ()(G3D::Color3)>::call(G3D::Color3)")]
// 0x6b1f80 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D6Color3EEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_ — rbx::callable<rbx::signals::signal<void ()(G3D::Color3)>::slot,boost::function<void ()(G3D::Color3)>,1,void ()(G3D::Color3)>::call(G3D::Color3)
pub fn stub_0x6b1f80() {
    // IDA 0x6b1f80: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(G3D::Color3)>::slot,boost::function<void ()(G3D::Color3)>,1,void ()(G3D::Color3)>::call(G3D::Color3)")]
// 0x6b1fa0 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D6Color3EEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_ — non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(G3D::Color3)>::slot,boost::function<void ()(G3D::Color3)>,1,void ()(G3D::Color3)>::call(G3D::Color3)
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(G3D::Color3)>::slot,boost::function<void ()(G3D::Color3)>,1,void ()(G3D::Color3)>::call(G3D::Color3)
pub fn stub_0x6b1fa0() {
    // IDA 0x6b1fa0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function1<void,G3D::Color3>::operator()(G3D::Color3)const")]
// 0x6b1fc0 — __ZNK5boost9function1IvN3G3D6Color3EEclES2_ — boost::function1<void,G3D::Color3>::operator()(G3D::Color3)const
pub fn stub_0x6b1fc0() {
    // IDA 0x6b1fc0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Color3)>::remove(rbx::signals::signal<void ()(G3D::Color3)>::slot *)")]
// 0x6b2098 — __ZN3rbx7signals6signalIFvN3G3D6Color3EEE6removeEPNS5_4slotE — rbx::signals::signal<void ()(G3D::Color3)>::remove(rbx::signals::signal<void ()(G3D::Color3)>::slot *)
pub fn stub_0x6b2098() {
    // IDA 0x6b2098: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Color3)>::slot::safe_static_init_mutex(void)")]
// 0x6b2188 — __ZN3rbx7signals6signalIFvN3G3D6Color3EEE4slot22safe_static_init_mutexEv — rbx::signals::signal<void ()(G3D::Color3)>::slot::safe_static_init_mutex(void)
pub fn stub_0x6b2188() {
    // IDA 0x6b2188: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Color3)>::slot::safe_static_do_get_mutex(void)")]
// 0x6b218c — __ZN3rbx7signals6signalIFvN3G3D6Color3EEE4slot24safe_static_do_get_mutexEv — rbx::signals::signal<void ()(G3D::Color3)>::slot::safe_static_do_get_mutex(void)
pub fn stub_0x6b218c() {
    // IDA 0x6b218c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Color3)>::slot,boost::function<void ()(G3D::Color3)>,1,void ()(G3D::Color3)>::~callable()")]
// 0x6b227c — __ZN3rbx8callableINS_7signals6signalIFvN3G3D6Color3EEE4slotEN5boost8functionIS5_EELi1ES5_ED1Ev — rbx::callable<rbx::signals::signal<void ()(G3D::Color3)>::slot,boost::function<void ()(G3D::Color3)>,1,void ()(G3D::Color3)>::~callable()
pub fn stub_0x6b227c() {
    // IDA 0x6b227c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Color3)>::slot,boost::function<void ()(G3D::Color3)>,1,void ()(G3D::Color3)>::~callable()")]
// 0x6b238c — __ZN3rbx8callableINS_7signals6signalIFvN3G3D6Color3EEE4slotEN5boost8functionIS5_EELi1ES5_ED0Ev — rbx::callable<rbx::signals::signal<void ()(G3D::Color3)>::slot,boost::function<void ()(G3D::Color3)>,1,void ()(G3D::Color3)>::~callable()
pub fn stub_0x6b238c() {
    // IDA 0x6b238c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Color3)>::slot::~slot()")]
// 0x6b24bc — __ZN3rbx7signals6signalIFvN3G3D6Color3EEE4slotD1Ev — rbx::signals::signal<void ()(G3D::Color3)>::slot::~slot()
pub fn stub_0x6b24bc() {
    // IDA 0x6b24bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Color3)>::slot::~slot()")]
// 0x6b24e8 — __ZN3rbx7signals6signalIFvN3G3D6Color3EEE4slotD0Ev — rbx::signals::signal<void ()(G3D::Color3)>::slot::~slot()
pub fn stub_0x6b24e8() {
    // IDA 0x6b24e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
