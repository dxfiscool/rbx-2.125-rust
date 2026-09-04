//! core shard FU — 100 core stubs EA-sorted, 0xf3ce24..0xf3e604 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after FT 0xf3ce14).
//! Source: ida/export.json filtered where demangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered after 0xf3ce14.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]


#[doc(alias = "RBX::GuiObject::TweenEasingStyle const& rbx::any_cast<RBX::GuiObject::TweenEasingStyle const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf3ce24 — j___ZN3rbx8any_castIRKN3RBX9GuiObject16TweenEasingStyleENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f3ce24() {
    // IDA 0xf3ce24: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "RBX::GuiObject::TweenEasingDirection const& rbx::any_cast<RBX::GuiObject::TweenEasingDirection const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf3ce34 — j___ZN3rbx8any_castIRKN3RBX9GuiObject20TweenEasingDirectionENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f3ce34() {
    // IDA 0xf3ce34: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "RBX::HopperBin::BinType const& rbx::any_cast<RBX::HopperBin::BinType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf3ce44 — j___ZN3rbx8any_castIRKN3RBX9HopperBin7BinTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f3ce44() {
    // IDA 0xf3ce44: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::CustomEventReceiver>::shared_ptr<RBX::CustomEventReceiver>(rbx_core::WeakPtr<RBX::CustomEventReceiver> const&,boost::detail::sp_nothrow_tag)")]
// 0xf3ced4 — j___ZN5boost10shared_ptrIN3RBX19CustomEventReceiverEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// was: boost::shared_ptr<RBX::CustomEventReceiver>::shared_ptr<RBX::CustomEventReceiver>(boost::weak_ptr<RBX::CustomEventReceiver> const&,boost::detail::sp_nothrow_tag)
pub fn stub_f3ced4() {
    // IDA 0xf3ced4: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(float)>::slot>::operator=(rbx::signals::signal<void ()(float)>::slot*)")]
// 0xf3cf14 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvfEE4slotEEaSEPS6_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(float)>::slot>::operator=(rbx::signals::signal<void ()(float)>::slot*)
pub fn stub_f3cf14() {
    // IDA 0xf3cf14: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(float)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(float)>::slot> const&)")]
// 0xf3cf24 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvfEE4slotEEaSERKS7_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(float)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(float)>::slot> const&)
pub fn stub_f3cf24() {
    // IDA 0xf3cf24: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>::operator()<float>(float &)")]
// 0xf3cf34 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIfEEvRT_
// was: void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>::operator()<float>(float &)
pub fn stub_f3cf34() {
    // IDA 0xf3cf34: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::function1<void,std::exception &>::move_assign(boost::function1<void,std::exception &>&)")]
// 0xf3cff4 — j___ZN5boost9function1IvRSt9exceptionE11move_assignERS3_
// was: boost::function1<void,std::exception &>::move_assign(boost::function1<void,std::exception &>&)
pub fn stub_f3cff4() {
    // IDA 0xf3cff4: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "boost::function1<void,std::exception &>::swap(boost::function1<void,std::exception &>&)")]
// 0xf3d004 — j___ZN5boost9function1IvRSt9exceptionE4swapERS3_
// was: boost::function1<void,std::exception &>::swap(boost::function1<void,std::exception &>&)
pub fn stub_f3d004() {
    // IDA 0xf3d004: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "boost::function1<void,std::exception &>::clear(void)")]
// 0xf3d014 — j___ZN5boost9function1IvRSt9exceptionE5clearEv
// was: boost::function1<void,std::exception &>::clear(void)
pub fn stub_f3d014() {
    // IDA 0xf3d014: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "std::_List_base<rbx_core::WeakPtr<RBX::CustomEventReceiver>,std::allocator<rbx_core::WeakPtr<RBX::CustomEventReceiver>>>::_M_clear(void)")]
// 0xf3db14 — j___ZNSt10_List_baseIN5boost8weak_ptrIN3RBX19CustomEventReceiverEEESaIS4_EE8_M_clearEv
// was: std::_List_base<boost::weak_ptr<RBX::CustomEventReceiver>,std::allocator<boost::weak_ptr<RBX::CustomEventReceiver>>>::_M_clear(void)
pub fn stub_f3db14() {
    // IDA 0xf3db14: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::_Deque_base<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::_M_allocate_map(unsigned long)")]
// 0xf3db24 — j___ZNSt11_Deque_baseIN3RBX16BindableFunction10InvocationESaIS2_EE15_M_allocate_mapEm
pub fn stub_f3db24() {
    // IDA 0xf3db24: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::_Deque_base<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::_M_create_nodes(RBX::BindableFunction::Invocation**,RBX::BindableFunction::Invocation**)")]
// 0xf3db34 — j___ZNSt11_Deque_baseIN3RBX16BindableFunction10InvocationESaIS2_EE15_M_create_nodesEPPS2_S6_
pub fn stub_f3db34() {
    // IDA 0xf3db34: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::_Deque_base<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::_M_initialize_map(unsigned long)")]
// 0xf3db44 — j___ZNSt11_Deque_baseIN3RBX16BindableFunction10InvocationESaIS2_EE17_M_initialize_mapEm
pub fn stub_f3db44() {
    // IDA 0xf3db44: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::_Deque_base<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::~_Deque_base()")]
// 0xf3db54 — j___ZNSt11_Deque_baseIN3RBX16BindableFunction10InvocationESaIS2_EED2Ev
pub fn stub_f3db54() {
    // IDA 0xf3db54: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::list<rbx_core::WeakPtr<RBX::CustomEventReceiver>,std::allocator<rbx_core::WeakPtr<RBX::CustomEventReceiver>>>::_M_create_node(rbx_core::WeakPtr<RBX::CustomEventReceiver> const&)")]
// 0xf3db64 — j___ZNSt4listIN5boost8weak_ptrIN3RBX19CustomEventReceiverEEESaIS4_EE14_M_create_nodeERKS4_
// was: std::list<boost::weak_ptr<RBX::CustomEventReceiver>,std::allocator<boost::weak_ptr<RBX::CustomEventReceiver>>>::_M_create_node(boost::weak_ptr<RBX::CustomEventReceiver> const&)
pub fn stub_f3db64() {
    // IDA 0xf3db64: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "void std::list<rbx_core::WeakPtr<RBX::CustomEventReceiver>,std::allocator<rbx_core::WeakPtr<RBX::CustomEventReceiver>>>::_M_initialize_dispatch<std::_List_const_iterator<rbx_core::WeakPtr<RBX::CustomEventReceiver>>>(std::_List_const_iterator<rbx_core::WeakPtr<RBX::CustomEventReceiver>>,std::_List_const_iterator<rbx_core::WeakPtr<RBX::CustomEventReceiver>>,std::__false_type)")]
// 0xf3db74 — j___ZNSt4listIN5boost8weak_ptrIN3RBX19CustomEventReceiverEEESaIS4_EE22_M_initialize_dispatchISt20_List_const_iteratorIS4_EEEvT_SA_St12__false_type
// was: void std::list<boost::weak_ptr<RBX::CustomEventReceiver>,std::allocator<boost::weak_ptr<RBX::CustomEventReceiver>>>::_M_initialize_dispatch<std::_List_const_iterator<boost::weak_ptr<RBX::CustomEventReceiver>>>(std::_List_const_iterator<boost::weak_ptr<RBX::CustomEventReceiver>>,std::_List_const_iterator<boost::weak_ptr<RBX::CustomEventReceiver>>,std::__false_type)
pub fn stub_f3db74() {
    // IDA 0xf3db74: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::list<rbx_core::WeakPtr<RBX::CustomEventReceiver>,std::allocator<rbx_core::WeakPtr<RBX::CustomEventReceiver>>>::list(std::list<rbx_core::WeakPtr<RBX::CustomEventReceiver>,std::allocator<rbx_core::WeakPtr<RBX::CustomEventReceiver>>> const&)")]
// 0xf3db84 — j___ZNSt4listIN5boost8weak_ptrIN3RBX19CustomEventReceiverEEESaIS4_EEC2ERKS6_
// was: std::list<boost::weak_ptr<RBX::CustomEventReceiver>,std::allocator<boost::weak_ptr<RBX::CustomEventReceiver>>>::list(std::list<boost::weak_ptr<RBX::CustomEventReceiver>,std::allocator<boost::weak_ptr<RBX::CustomEventReceiver>>> const&)
pub fn stub_f3db84() {
    // IDA 0xf3db84: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::deque<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::_M_destroy_data_aux(std::_Deque_iterator<RBX::BindableFunction::Invocation,RBX::BindableFunction::Invocation&,RBX::BindableFunction::Invocation*>,std::_Deque_iterator<RBX::BindableFunction::Invocation,RBX::BindableFunction::Invocation&,RBX::BindableFunction::Invocation*>)")]
// 0xf3db94 — j___ZNSt5dequeIN3RBX16BindableFunction10InvocationESaIS2_EE19_M_destroy_data_auxESt15_Deque_iteratorIS2_RS2_PS2_ES8_
pub fn stub_f3db94() {
    // IDA 0xf3db94: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::deque<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::deque(std::deque<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>> const&)")]
// 0xf3dba4 — j___ZNSt5dequeIN3RBX16BindableFunction10InvocationESaIS2_EEC2ERKS4_
pub fn stub_f3dba4() {
    // IDA 0xf3dba4: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::deque<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::~deque()")]
// 0xf3dbb4 — j___ZNSt5dequeIN3RBX16BindableFunction10InvocationESaIS2_EED2Ev
pub fn stub_f3dbb4() {
    // IDA 0xf3dbb4: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>> *)")]
// 0xf3dbd4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot10DialogToneEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3dbd4() {
    // IDA 0xf3dbd4: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>> *)")]
// 0xf3dbe4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot13DialogPurposeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3dbe4() {
    // IDA 0xf3dbe4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>> *)")]
// 0xf3dbf4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11ChatService9ChatColorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3dbf4() {
    // IDA 0xf3dbf4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>> *)")]
// 0xf3dc04 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11HttpService15HttpContentTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3dc04() {
    // IDA 0xf3dc04: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>> *)")]
// 0xf3dc14 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11InputObject13UserInputTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3dc14() {
    // IDA 0xf3dc14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>,std::_Select1st<std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>> *)")]
// 0xf3dc24 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11InputObject14UserInputStateEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3dc24() {
    // IDA 0xf3dc24: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SurfaceType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SurfaceType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SurfaceType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::SurfaceType>> *)")]
// 0xf3dc34 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11SurfaceTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
pub fn stub_f3dc34() {
    // IDA 0xf3dc34: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::AssetService::AccessType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>> *)")]
// 0xf3dc44 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12AssetService10AccessTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3dc44() {
    // IDA 0xf3dc44: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>> *)")]
// 0xf3dc54 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12GameSettings12VideoQualityEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3dc54() {
    // IDA 0xf3dc54: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>> *)")]
// 0xf3dc64 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12GameSettings13UploadSettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3dc64() {
    // IDA 0xf3dc64: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>> *)")]
// 0xf3dc74 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12SpecialShape8MeshTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3dc74() {
    // IDA 0xf3dc74: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>> *)")]
// 0xf3dc84 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13CharacterMesh8BodyPartEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3dc84() {
    // IDA 0xf3dc84: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>> *)")]
// 0xf3dca4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13FriendService12FriendStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3dca4() {
    // IDA 0xf3dca4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>> *)")]
// 0xf3dcb4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13FriendService15FriendEventTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3dcb4() {
    // IDA 0xf3dcb4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SocialService::StuffType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>> *)")]
// 0xf3dcd4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13SocialService9StuffTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3dcd4() {
    // IDA 0xf3dcd4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>> *)")]
// 0xf3dce4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_14FunctionalTest6ResultEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3dce4() {
    // IDA 0xf3dce4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>,std::_Select1st<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>> *)")]
// 0xf3dd04 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16KeyframeSequence8PriorityEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3dd04() {
    // IDA 0xf3dd04: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::LegacyController::InputType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::LegacyController::InputType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::LegacyController::InputType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::LegacyController::InputType>> *)")]
// 0xf3dd14 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16LegacyController9InputTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3dd14() {
    // IDA 0xf3dd14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode>> *)")]
// 0xf3dd34 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17GameBasicSettings11ControlModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3dd34() {
    // IDA 0xf3dd34: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>> *)")]
// 0xf3dd44 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17GameBasicSettings20RenderQualitySettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3dd44() {
    // IDA 0xf3dd44: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::KeywordFilterType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::KeywordFilterType>> *)")]
// 0xf3dd54 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17KeywordFilterTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
pub fn stub_f3dd54() {
    // IDA 0xf3dd54: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>> *)")]
// 0xf3dd64 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_18SkateboardPlatform9MoveStateEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3dd64() {
    // IDA 0xf3dd64: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>> *)")]
// 0xf3dd84 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_21PersonalServerService13PrivilegeTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3dd84() {
    // IDA 0xf3dd84: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Frame::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Frame::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Frame::Style>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Frame::Style>> *)")]
// 0xf3dd94 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Frame5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3dd94() {
    // IDA 0xf3dd94: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Joint::JointType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Joint::JointType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Joint::JointType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Joint::JointType>> *)")]
// 0xf3dda4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Joint9JointTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3dda4() {
    // IDA 0xf3dda4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>> *)")]
// 0xf3ddb4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel12CellMaterialEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3ddb4() {
    // IDA 0xf3ddb4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>> *)")]
// 0xf3ddc4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel14WaterCellForceEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3ddc4() {
    // IDA 0xf3ddc4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>> *)")]
// 0xf3ddd4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel15CellOrientationEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3ddd4() {
    // IDA 0xf3ddd4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>> *)")]
// 0xf3dde4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel18WaterCellDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3dde4() {
    // IDA 0xf3dde4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>> *)")]
// 0xf3ddf4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel9CellBlockEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3ddf4() {
    // IDA 0xf3ddf4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Action::ActionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Action::ActionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Action::ActionType>> *)")]
// 0xf3de04 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Action10ActionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3de04() {
    // IDA 0xf3de04: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>> *)")]
// 0xf3de14 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Legacy17SurfaceConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3de14() {
    // IDA 0xf3de14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::InOut>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::InOut>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::InOut>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Feature::InOut>> *)")]
// 0xf3de24 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Feature5InOutEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3de24() {
    // IDA 0xf3de24: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::LeftRight>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>> *)")]
// 0xf3de34 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Feature9LeftRightEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3de34() {
    // IDA 0xf3de34: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::TopBottom>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>> *)")]
// 0xf3de44 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Feature9TopBottomEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3de44() {
    // IDA 0xf3de44: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>> *)")]
// 0xf3de54 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Handles11VisualStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3de54() {
    // IDA 0xf3de54: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>> *)")]
// 0xf3de64 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Humanoid13NameOcclusionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3de64() {
    // IDA 0xf3de64: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Humanoid::Status>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Humanoid::Status>> *)")]
// 0xf3de74 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Humanoid6StatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3de74() {
    // IDA 0xf3de74: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiButton::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GuiButton::Style>> *)")]
// 0xf3de84 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiButton5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3de84() {
    // IDA 0xf3de84: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>> *)")]
// 0xf3de94 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3de94() {
    // IDA 0xf3de94: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>> *)")]
// 0xf3dea4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject14SizeConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3dea4() {
    // IDA 0xf3dea4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>> *)")]
// 0xf3deb4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject16TweenEasingStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3deb4() {
    // IDA 0xf3deb4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>> *)")]
// 0xf3dec4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject20TweenEasingDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3dec4() {
    // IDA 0xf3dec4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HopperBin::BinType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>> *)")]
// 0xf3ded4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9HopperBin7BinTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3ded4() {
    // IDA 0xf3ded4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Deque_iterator<RBX::BindableFunction::Invocation,RBX::BindableFunction::Invocation&,RBX::BindableFunction::Invocation*> std::__uninitialized_copy_aux<std::_Deque_iterator<RBX::BindableFunction::Invocation,RBX::BindableFunction::Invocation const&,RBX::BindableFunction::Invocation const*>,std::_Deque_iterator<RBX::BindableFunction::Invocation,RBX::BindableFunction::Invocation&,RBX::BindableFunction::Invocation*>>(std::_Deque_iterator<RBX::BindableFunction::Invocation,RBX::BindableFunction::Invocation const&,RBX::BindableFunction::Invocation const*>,std::_Deque_iterator<RBX::BindableFunction::Invocation,RBX::BindableFunction::Invocation const&,RBX::BindableFunction::Invocation const*>,std::_Deque_iterator<RBX::BindableFunction::Invocation,RBX::BindableFunction::Invocation&,RBX::BindableFunction::Invocation*>,std::__false_type)")]
// 0xf3def4 — j___ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN3RBX16BindableFunction10InvocationERKS3_PS4_ES0_IS3_RS3_PS3_EET0_T_SC_SB_St12__false_type
pub fn stub_f3def4() {
    // IDA 0xf3def4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Hole> RBX::shared_from<RBX::Hole>(RBX::Hole*)")]
// 0xf3dfb4 — j___ZN3RBX11shared_fromINS_4HoleEEEN5boost10shared_ptrIT_EEPS4_
// was: boost::shared_ptr<RBX::Hole> RBX::shared_from<RBX::Hole>(RBX::Hole*)
pub fn stub_f3dfb4() {
    // IDA 0xf3dfb4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Hole>::operator=(rbx_core::SharedPtr<RBX::Hole> const&)")]
// 0xf3e104 — j___ZN5boost10shared_ptrIN3RBX4HoleEEaSERKS3_
// was: boost::shared_ptr<RBX::Hole>::operator=(boost::shared_ptr<RBX::Hole> const&)
pub fn stub_f3e104() {
    // IDA 0xf3e104: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Vector_base<RBX::Feature::InOut,std::allocator<RBX::Feature::InOut>>::_M_allocate(unsigned long)")]
// 0xf3e224 — j___ZNSt12_Vector_baseIN3RBX7Feature5InOutESaIS2_EE11_M_allocateEm
pub fn stub_f3e224() {
    // IDA 0xf3e224: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Vector_base<RBX::Feature::LeftRight,std::allocator<RBX::Feature::LeftRight>>::_M_allocate(unsigned long)")]
// 0xf3e234 — j___ZNSt12_Vector_baseIN3RBX7Feature9LeftRightESaIS2_EE11_M_allocateEm
pub fn stub_f3e234() {
    // IDA 0xf3e234: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Vector_base<RBX::Feature::TopBottom,std::allocator<RBX::Feature::TopBottom>>::_M_allocate(unsigned long)")]
// 0xf3e244 — j___ZNSt12_Vector_baseIN3RBX7Feature9TopBottomESaIS2_EE11_M_allocateEm
pub fn stub_f3e244() {
    // IDA 0xf3e244: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Feature::InOut * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Feature::InOut *,RBX::Feature::InOut *>(RBX::Feature::InOut *,RBX::Feature::InOut *,RBX::Feature::InOut *)")]
// 0xf3e254 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX7Feature5InOutES6_EET0_T_S8_S7_
pub fn stub_f3e254() {
    // IDA 0xf3e254: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Feature::LeftRight * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Feature::LeftRight *,RBX::Feature::LeftRight *>(RBX::Feature::LeftRight *,RBX::Feature::LeftRight *,RBX::Feature::LeftRight *)")]
// 0xf3e264 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX7Feature9LeftRightES6_EET0_T_S8_S7_
pub fn stub_f3e264() {
    // IDA 0xf3e264: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Feature::TopBottom * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Feature::TopBottom *,RBX::Feature::TopBottom *>(RBX::Feature::TopBottom *,RBX::Feature::TopBottom *,RBX::Feature::TopBottom *)")]
// 0xf3e274 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX7Feature9TopBottomES6_EET0_T_S8_S7_
pub fn stub_f3e274() {
    // IDA 0xf3e274: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Feature::InOut,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::InOut>>>::operator[](RBX::Name const* const&)")]
// 0xf3e284 — j___ZNSt3mapIPKN3RBX4NameENS0_7Feature5InOutESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f3e284() {
    // IDA 0xf3e284: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Feature::LeftRight,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>>::operator[](RBX::Name const* const&)")]
// 0xf3e294 — j___ZNSt3mapIPKN3RBX4NameENS0_7Feature9LeftRightESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f3e294() {
    // IDA 0xf3e294: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Feature::TopBottom,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>>::operator[](RBX::Name const* const&)")]
// 0xf3e2a4 — j___ZNSt3mapIPKN3RBX4NameENS0_7Feature9TopBottomESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f3e2a4() {
    // IDA 0xf3e2a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Feature::InOut,std::allocator<RBX::Feature::InOut>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Feature::InOut*,std::vector<RBX::Feature::InOut,std::allocator<RBX::Feature::InOut>>>,RBX::Feature::InOut const&)")]
// 0xf3e2b4 — j___ZNSt6vectorIN3RBX7Feature5InOutESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f3e2b4() {
    // IDA 0xf3e2b4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Feature::InOut,std::allocator<RBX::Feature::InOut>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Feature::InOut*,std::vector<RBX::Feature::InOut,std::allocator<RBX::Feature::InOut>>>,unsigned long,RBX::Feature::InOut const&)")]
// 0xf3e2c4 — j___ZNSt6vectorIN3RBX7Feature5InOutESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f3e2c4() {
    // IDA 0xf3e2c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Feature::InOut,std::allocator<RBX::Feature::InOut>>::resize(unsigned long,RBX::Feature::InOut)")]
// 0xf3e2d4 — j___ZNSt6vectorIN3RBX7Feature5InOutESaIS2_EE6resizeEmS2_
pub fn stub_f3e2d4() {
    // IDA 0xf3e2d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Feature::InOut,std::allocator<RBX::Feature::InOut>>::push_back(RBX::Feature::InOut const&)")]
// 0xf3e2e4 — j___ZNSt6vectorIN3RBX7Feature5InOutESaIS2_EE9push_backERKS2_
pub fn stub_f3e2e4() {
    // IDA 0xf3e2e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Feature::LeftRight,std::allocator<RBX::Feature::LeftRight>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Feature::LeftRight*,std::vector<RBX::Feature::LeftRight,std::allocator<RBX::Feature::LeftRight>>>,RBX::Feature::LeftRight const&)")]
// 0xf3e2f4 — j___ZNSt6vectorIN3RBX7Feature9LeftRightESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f3e2f4() {
    // IDA 0xf3e2f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Feature::LeftRight,std::allocator<RBX::Feature::LeftRight>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Feature::LeftRight*,std::vector<RBX::Feature::LeftRight,std::allocator<RBX::Feature::LeftRight>>>,unsigned long,RBX::Feature::LeftRight const&)")]
// 0xf3e304 — j___ZNSt6vectorIN3RBX7Feature9LeftRightESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f3e304() {
    // IDA 0xf3e304: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Feature::LeftRight,std::allocator<RBX::Feature::LeftRight>>::resize(unsigned long,RBX::Feature::LeftRight)")]
// 0xf3e314 — j___ZNSt6vectorIN3RBX7Feature9LeftRightESaIS2_EE6resizeEmS2_
pub fn stub_f3e314() {
    // IDA 0xf3e314: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Feature::LeftRight,std::allocator<RBX::Feature::LeftRight>>::push_back(RBX::Feature::LeftRight const&)")]
// 0xf3e324 — j___ZNSt6vectorIN3RBX7Feature9LeftRightESaIS2_EE9push_backERKS2_
pub fn stub_f3e324() {
    // IDA 0xf3e324: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Feature::TopBottom,std::allocator<RBX::Feature::TopBottom>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Feature::TopBottom*,std::vector<RBX::Feature::TopBottom,std::allocator<RBX::Feature::TopBottom>>>,RBX::Feature::TopBottom const&)")]
// 0xf3e334 — j___ZNSt6vectorIN3RBX7Feature9TopBottomESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f3e334() {
    // IDA 0xf3e334: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Feature::TopBottom,std::allocator<RBX::Feature::TopBottom>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Feature::TopBottom*,std::vector<RBX::Feature::TopBottom,std::allocator<RBX::Feature::TopBottom>>>,unsigned long,RBX::Feature::TopBottom const&)")]
// 0xf3e344 — j___ZNSt6vectorIN3RBX7Feature9TopBottomESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f3e344() {
    // IDA 0xf3e344: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Feature::TopBottom,std::allocator<RBX::Feature::TopBottom>>::resize(unsigned long,RBX::Feature::TopBottom)")]
// 0xf3e354 — j___ZNSt6vectorIN3RBX7Feature9TopBottomESaIS2_EE6resizeEmS2_
pub fn stub_f3e354() {
    // IDA 0xf3e354: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Feature::TopBottom,std::allocator<RBX::Feature::TopBottom>>::push_back(RBX::Feature::TopBottom const&)")]
// 0xf3e364 — j___ZNSt6vectorIN3RBX7Feature9TopBottomESaIS2_EE9push_backERKS2_
pub fn stub_f3e364() {
    // IDA 0xf3e364: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::InOut>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::InOut>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::InOut>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Feature::InOut> const&)")]
// 0xf3e374 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Feature5InOutEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f3e374() {
    // IDA 0xf3e374: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::InOut>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::InOut>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::InOut>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Feature::InOut>>,std::pair<RBX::Name const* const,RBX::Feature::InOut> const&)")]
// 0xf3e384 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Feature5InOutEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f3e384() {
    // IDA 0xf3e384: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::InOut>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::InOut>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::InOut>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Feature::InOut> const&)")]
// 0xf3e394 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Feature5InOutEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f3e394() {
    // IDA 0xf3e394: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::LeftRight>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Feature::LeftRight> const&)")]
// 0xf3e3a4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Feature9LeftRightEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f3e3a4() {
    // IDA 0xf3e3a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::LeftRight>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>,std::pair<RBX::Name const* const,RBX::Feature::LeftRight> const&)")]
// 0xf3e3b4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Feature9LeftRightEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f3e3b4() {
    // IDA 0xf3e3b4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::LeftRight>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Feature::LeftRight> const&)")]
// 0xf3e3c4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Feature9LeftRightEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f3e3c4() {
    // IDA 0xf3e3c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::TopBottom>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Feature::TopBottom> const&)")]
// 0xf3e3d4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Feature9TopBottomEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f3e3d4() {
    // IDA 0xf3e3d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::TopBottom>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>,std::pair<RBX::Name const* const,RBX::Feature::TopBottom> const&)")]
// 0xf3e3e4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Feature9TopBottomEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f3e3e4() {
    // IDA 0xf3e3e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::TopBottom>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Feature::TopBottom> const&)")]
// 0xf3e3f4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Feature9TopBottomEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f3e3f4() {
    // IDA 0xf3e3f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::MeshId const& rbx::any_cast<RBX::MeshId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf3e434 — j___ZN3rbx8any_castIRKN3RBX6MeshIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f3e434() {
    // IDA 0xf3e434: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Flag> RBX::shared_from<RBX::Flag>(RBX::Flag*)")]
// 0xf3e604 — j___ZN3RBX11shared_fromINS_4FlagEEEN5boost10shared_ptrIT_EEPS4_
// was: boost::shared_ptr<RBX::Flag> RBX::shared_from<RBX::Flag>(RBX::Flag*)
pub fn stub_f3e604() {
    // IDA 0xf3e604: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}
