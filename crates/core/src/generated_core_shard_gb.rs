//! core shard GB — 100 core stubs EA-sorted, 0xf45604..0xf46b14 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after 0xf455f4).
//! Source: ida/export.json filtered where demangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered after 0xf455f4.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "boost::exception_detail::error_info_injector<std::length_error>::~error_info_injector()")]
// 0xf45604 — j___ZN5boost16exception_detail19error_info_injectorISt12length_errorED2Ev
pub fn stub_f45604() {
    // IDA 0xf45604: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void rbx_core::SharedPtr_release<rbx::signals::connection::islot,int,0,0>(rbx::intrusive_ptr_target<rbx::signals::connection::islot,int,0,0> const*)")]
// 0xf45614 — j___ZN5boost21intrusive_ptr_releaseIN3rbx7signals10connection5islotEiLi0ELi0EEEvPKNS1_20intrusive_ptr_targetIT_T0_XT1_EXT2_EEE
// was: void boost::intrusive_ptr_release<rbx::signals::connection::islot,int,0,0>(rbx::intrusive_ptr_target<rbx::signals::connection::islot,int,0,0> const*)
pub fn stub_f45614() {
    // IDA 0xf45614: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::pool<boost::default_user_allocator_malloc_free>::malloc_need_resize(void)")]
// 0xf45694 — j___ZN5boost4poolINS_34default_user_allocator_malloc_freeEE18malloc_need_resizeEv
pub fn stub_f45694() {
    // IDA 0xf45694: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::allocator_traits<std::allocator<boost::unordered::detail::ptr_bucket>>::allocate(std::allocator<boost::unordered::detail::ptr_bucket>&,unsigned long)")]
// 0xf45784 — j___ZN5boost9unordered6detail16allocator_traitsISaINS1_10ptr_bucketEEE8allocateERS4_m
pub fn stub_f45784() {
    // IDA 0xf45784: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "void boost::unordered::detail::array_constructor<std::allocator<boost::unordered::detail::ptr_bucket>>::construct<boost::unordered::detail::ptr_bucket>(boost::unordered::detail::ptr_bucket const&,unsigned long)")]
// 0xf457b4 — j___ZN5boost9unordered6detail17array_constructorISaINS1_10ptr_bucketEEE9constructIS3_EEvRKT_m
pub fn stub_f457b4() {
    // IDA 0xf457b4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "RBX::FWService * RBX::ServiceProvider::find<RBX::FWService>(void)const")]
// 0xf45904 — j___ZNK3RBX15ServiceProvider4findINS_9FWServiceEEEPT_v
pub fn stub_f45904() {
    // IDA 0xf45904: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "RBX::Joint::getNormalId(int)const")]
// 0xf45924 — j___ZNK3RBX5Joint11getNormalIdEi
pub fn stub_f45924() {
    // IDA 0xf45924: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::EdgeList::getOther(int)const")]
// 0xf45934 — j___ZNK3RBX8EdgeList8getOtherEi
pub fn stub_f45934() {
    // IDA 0xf45934: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Primitive::getExtentsWorld(void)const")]
// 0xf45964 — j___ZNK3RBX9Primitive15getExtentsWorldEv
pub fn stub_f45964() {
    // IDA 0xf45964: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::rethrow(void)const")]
// 0xf45974 — j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorIN3rbx22bad_placement_any_castEEEE7rethrowEv
pub fn stub_f45974() {
    // IDA 0xf45974: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::clone(void)const")]
// 0xf45984 — j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEE5cloneEv
pub fn stub_f45984() {
    // IDA 0xf45984: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::clone(void)const")]
// 0xf45994 — j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEE5cloneEv
pub fn stub_f45994() {
    // IDA 0xf45994: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::clone(void)const")]
// 0xf459a4 — j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEE5cloneEv
pub fn stub_f459a4() {
    // IDA 0xf459a4: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "boost::function1<void,std::exception &>::operator()(std::exception &)const")]
// 0xf45a04 — j___ZNK5boost9function1IvRSt9exceptionEclES2_
pub fn stub_f45a04() {
    // IDA 0xf45a04: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "std::_Vector_base<RBX::Material,std::allocator<RBX::Material>>::_M_allocate(unsigned long)")]
// 0xf45a74 — j___ZNSt12_Vector_baseIN3RBX8MaterialESaIS1_EE11_M_allocateEm
pub fn stub_f45a74() {
    // IDA 0xf45a74: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "std::_Vector_base<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>::_M_allocate(unsigned long)")]
// 0xf45aa4 — j___ZNSt12_Vector_baseIPKN3RBX9PrimitiveESaIS3_EE11_M_allocateEm
pub fn stub_f45aa4() {
    // IDA 0xf45aa4: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "RBX::Material * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Material *,RBX::Material *>(RBX::Material *,RBX::Material *,RBX::Material *)")]
// 0xf45ac4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX8MaterialES5_EET0_T_S7_S6_
pub fn stub_f45ac4() {
    // IDA 0xf45ac4: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Material,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Material>>>::operator[](RBX::Name const* const&)")]
// 0xf45b04 — j___ZNSt3mapIPKN3RBX4NameENS0_8MaterialESt4lessIS3_ESaISt4pairIKS3_S4_EEEixERS8_
pub fn stub_f45b04() {
    // IDA 0xf45b04: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "std::vector<RBX::Material,std::allocator<RBX::Material>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Material*,std::vector<RBX::Material,std::allocator<RBX::Material>>>,RBX::Material const&)")]
// 0xf45b64 — j___ZNSt6vectorIN3RBX8MaterialESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
pub fn stub_f45b64() {
    // IDA 0xf45b64: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Material,std::allocator<RBX::Material>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Material*,std::vector<RBX::Material,std::allocator<RBX::Material>>>,unsigned long,RBX::Material const&)")]
// 0xf45b74 — j___ZNSt6vectorIN3RBX8MaterialESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
pub fn stub_f45b74() {
    // IDA 0xf45b74: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Material,std::allocator<RBX::Material>>::resize(unsigned long,RBX::Material)")]
// 0xf45b84 — j___ZNSt6vectorIN3RBX8MaterialESaIS1_EE6resizeEmS1_
pub fn stub_f45b84() {
    // IDA 0xf45b84: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Material,std::allocator<RBX::Material>>::push_back(RBX::Material const&)")]
// 0xf45b94 — j___ZNSt6vectorIN3RBX8MaterialESaIS1_EE9push_backERKS1_
pub fn stub_f45b94() {
    // IDA 0xf45b94: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Primitive const**,std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>>,RBX::Primitive const* const&)")]
// 0xf45c14 — j___ZNSt6vectorIPKN3RBX9PrimitiveESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
pub fn stub_f45c14() {
    // IDA 0xf45c14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>::push_back(RBX::Primitive const* const&)")]
// 0xf45c24 — j___ZNSt6vectorIPKN3RBX9PrimitiveESaIS3_EE9push_backERKS3_
pub fn stub_f45c24() {
    // IDA 0xf45c24: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<std::string,std::allocator<std::string>>::resize(unsigned long,std::string)")]
// 0xf45c34 — j___ZNSt6vectorISsSaISsEE6resizeEmSs
pub fn stub_f45c34() {
    // IDA 0xf45c34: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<std::string,std::allocator<std::string>>::~vector()")]
// 0xf45c44 — j___ZNSt6vectorISsSaISsEED2Ev
pub fn stub_f45c44() {
    // IDA 0xf45c44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<unsigned long,std::allocator<unsigned long>>::resize(unsigned long,unsigned long)")]
// 0xf45c54 — j___ZNSt6vectorImSaImEE6resizeEmm
pub fn stub_f45c54() {
    // IDA 0xf45c54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Material>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Material>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Material>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Material> const&)")]
// 0xf45ca4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8MaterialEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_
pub fn stub_f45ca4() {
    // IDA 0xf45ca4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Material>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Material>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Material>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Material>>,std::pair<RBX::Name const* const,RBX::Material> const&)")]
// 0xf45cb4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8MaterialEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
pub fn stub_f45cb4() {
    // IDA 0xf45cb4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Material>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Material>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Material>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Material>> *)")]
// 0xf45cc4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8MaterialEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
pub fn stub_f45cc4() {
    // IDA 0xf45cc4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Material>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Material>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Material>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Material> const&)")]
// 0xf45cd4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8MaterialEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_
pub fn stub_f45cd4() {
    // IDA 0xf45cd4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<double,std::allocator<double>>::resize(unsigned long,double)")]
// 0xf45cf4 — j___ZNSt6vectorIdSaIdEE6resizeEmd
pub fn stub_f45cf4() {
    // IDA 0xf45cf4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::safe_static_do_get_mutex(void)")]
// 0xf45d14 — j___ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE24safe_static_do_get_mutexEv
pub fn stub_f45d14() {
    // IDA 0xf45d14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::slot::safe_static_do_get_mutex(void)")]
// 0xf45d24 — j___ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot24safe_static_do_get_mutexEv
pub fn stub_f45d24() {
    // IDA 0xf45d24: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::insert(rbx::signals::signal<void ()(RBX::Primitive *)>::slot *)")]
// 0xf45d34 — j___ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE6insertEPNS6_4slotE
pub fn stub_f45d34() {
    // IDA 0xf45d34: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::remove(rbx::signals::signal<void ()(RBX::Primitive *)>::slot *)")]
// 0xf45d44 — j___ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE6removeEPNS6_4slotE
pub fn stub_f45d44() {
    // IDA 0xf45d44: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Primitive *)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>> const&)")]
// 0xf45d54 — j___ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_14PhysicsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEEENS0_10connectionERKT_
pub fn stub_f45d54() {
    // IDA 0xf45d54: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Primitive *)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Primitive *)>::slot*)")]
// 0xf45d64 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotEEaSEPS9_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Primitive *)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Primitive *)>::slot*)
pub fn stub_f45d64() {
    // IDA 0xf45d64: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Primitive *)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Primitive *)>::slot> const&)")]
// 0xf45d74 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotEEaSERKSA_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Primitive *)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Primitive *)>::slot> const&)
pub fn stub_f45d74() {
    // IDA 0xf45d74: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>>::operator()<RBX::Primitive *>(RBX::Primitive * &)")]
// 0xf45d84 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX14PhysicsServiceEPNS4_9PrimitiveEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS7_EEvRT_
pub fn stub_f45d84() {
    // IDA 0xf45d84: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "RBX::StarterGuiService::~StarterGuiService()")]
// 0xf45f74 — j___ZN3RBX17StarterGuiServiceD2Ev
pub fn stub_f45f74() {
    // IDA 0xf45f74: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "RBX::IAdornableCollector::IAdornableCollector(void)")]
// 0xf45f84 — j___ZN3RBX19IAdornableCollectorC2Ev
pub fn stub_f45f84() {
    // IDA 0xf45f84: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::StarterGuiService::CoreGuiType>(RBX::StarterGuiService::CoreGuiType const&)")]
// 0xf45fe4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_17StarterGuiService11CoreGuiTypeEEERS3_RKT_
pub fn stub_f45fe4() {
    // IDA 0xf45fe4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::StarterGuiService::CoreGuiType>::singleton(void)")]
// 0xf45ff4 — j___ZN3rbx14implementation12typed_holderIN3RBX17StarterGuiService11CoreGuiTypeEE9singletonEv
pub fn stub_f45ff4() {
    // IDA 0xf45ff4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<2,void ()(RBX::StarterGuiService::CoreGuiType,bool)>::operator()(RBX::StarterGuiService::CoreGuiType,bool)")]
// 0xf46004 — j___ZN3rbx7signals16signal_with_argsILi2EFvN3RBX17StarterGuiService11CoreGuiTypeEbEEclES4_b
pub fn stub_f46004() {
    // IDA 0xf46004: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::disconnectAll(void)")]
// 0xf46014 — j___ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE13disconnectAllEv
pub fn stub_f46014() {
    // IDA 0xf46014: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::safe_static_do_get_mutex(void)")]
// 0xf46024 — j___ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE24safe_static_do_get_mutexEv
pub fn stub_f46024() {
    // IDA 0xf46024: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot> &)")]
// 0xf46034 — j___ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4nextERN5boost13intrusive_ptrINS6_4slotEEE
// was: rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot> &)
pub fn stub_f46034() {
    // IDA 0xf46034: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot::safe_static_do_get_mutex(void)")]
// 0xf46044 — j___ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slot24safe_static_do_get_mutexEv
pub fn stub_f46044() {
    // IDA 0xf46044: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::insert(rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot *)")]
// 0xf46054 — j___ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE6insertEPNS6_4slotE
pub fn stub_f46054() {
    // IDA 0xf46054: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::remove(rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot *)")]
// 0xf46064 — j___ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE6removeEPNS6_4slotE
pub fn stub_f46064() {
    // IDA 0xf46064: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::connect<boost::function<void ()(RBX::StarterGuiService::CoreGuiType,bool)>>(boost::function<void ()(RBX::StarterGuiService::CoreGuiType,bool)> const&)")]
// 0xf46074 — j___ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
pub fn stub_f46074() {
    // IDA 0xf46074: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::on_error(std::exception &)")]
// 0xf46084 — j___ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE8on_errorERSt9exception
pub fn stub_f46084() {
    // IDA 0xf46084: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::StarterGuiService::CoreGuiType * rbx::any_cast<RBX::StarterGuiService::CoreGuiType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0xf46094 — j___ZN3rbx8any_castIN3RBX17StarterGuiService11CoreGuiTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_f46094() {
    // IDA 0xf46094: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "RBX::StarterGuiService::CoreGuiType const& rbx::any_cast<RBX::StarterGuiService::CoreGuiType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf460a4 — j___ZN3rbx8any_castIRKN3RBX17StarterGuiService11CoreGuiTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f460a4() {
    // IDA 0xf460a4: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "RBX::StarterGuiService::CoreGuiType & rbx::any_cast<RBX::StarterGuiService::CoreGuiType &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf460b4 — j___ZN3rbx8any_castIRN3RBX17StarterGuiService11CoreGuiTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f460b4() {
    // IDA 0xf460b4: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot,boost::function<void ()(RBX::StarterGuiService::CoreGuiType,bool)>,2,void ()(RBX::StarterGuiService::CoreGuiType,bool)>::callable<rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>*>(boost::function<void ()(RBX::StarterGuiService::CoreGuiType,bool)> const&,rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>*)")]
// 0xf460c4 — j___ZN3rbx8callableINS_7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotEN5boost8functionIS6_EELi2ES6_EC2IPS7_EERKSB_T_
pub fn stub_f460c4() {
    // IDA 0xf460c4: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "boost::flyweights::detail::recursive_lightweight_mutex::recursive_lightweight_mutex(void)")]
// 0xf460d4 — j___ZN5boost10flyweights6detail27recursive_lightweight_mutexC2Ev
pub fn stub_f460d4() {
    // IDA 0xf460d4: flyweight interned-value holder. Arc<str>-style interning at the live site — carrier no-op.
}

#[doc(alias = "boost::multi_index::multi_index_container<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>::multi_index_container(boost::tuples::cons<boost::tuples::tuple<unsigned long,boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>,boost::hash<RBX::ProtectedString>,std::equal_to<RBX::ProtectedString>,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::null_type> const&,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>> const&)")]
// 0xf46124 — j___ZN5boost11multi_index21multi_index_containerINS_10flyweights6detail16refcounted_valueINS3_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES7_EENS2_20hashed_factory_classISA_S7_N4mpl_2naESD_SD_E10index_listESaISA_EEC2ERKNS_6tuples4consINSI_5tupleImNS0_8identityISA_EENS_4hashIS7_EESt8equal_toIS7_ENSI_9null_typeESR_SR_SR_SR_SR_EESR_EERKSG_
pub fn stub_f46124() {
    // IDA 0xf46124: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot>::operator=(rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot*)")]
// 0xf46134 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotEEaSEPS9_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot>::operator=(rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot*)
pub fn stub_f46134() {
    // IDA 0xf46134: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot> const&)")]
// 0xf46144 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotEEaSERKSA_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot> const&)
pub fn stub_f46144() {
    // IDA 0xf46144: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::IAdornableCollector>(RBX::IAdornableCollector *)")]
// 0xf46174 — j___ZN5boost6detail12shared_countC2IN3RBX19IAdornableCollectorEEEPT_
pub fn stub_f46174() {
    // IDA 0xf46174: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::function2<void,RBX::StarterGuiService::CoreGuiType,bool>::assign_to_own(boost::function2<void,RBX::StarterGuiService::CoreGuiType,bool> const&)")]
// 0xf461d4 — j___ZN5boost9function2IvN3RBX17StarterGuiService11CoreGuiTypeEbE13assign_to_ownERKS4_
pub fn stub_f461d4() {
    // IDA 0xf461d4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::function2<void,RBX::StarterGuiService::CoreGuiType,bool>::clear(void)")]
// 0xf461e4 — j___ZN5boost9function2IvN3RBX17StarterGuiService11CoreGuiTypeEbE5clearEv
pub fn stub_f461e4() {
    // IDA 0xf461e4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>,RBX::StarterGuiService::CoreGuiType,bool,boost::hash<RBX::StarterGuiService::CoreGuiType>,std::equal_to<RBX::StarterGuiService::CoreGuiType>>>::rehash_impl(unsigned long)")]
// 0xf46214 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE11rehash_implEm
pub fn stub_f46214() {
    // IDA 0xf46214: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>,RBX::StarterGuiService::CoreGuiType,bool,boost::hash<RBX::StarterGuiService::CoreGuiType>,std::equal_to<RBX::StarterGuiService::CoreGuiType>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>,RBX::StarterGuiService::CoreGuiType,bool,boost::hash<RBX::StarterGuiService::CoreGuiType>,std::equal_to<RBX::StarterGuiService::CoreGuiType>>> &,boost::unordered::detail::ptr_bucket *)")]
// 0xf46224 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE15place_in_bucketERNS1_5tableISF_EEPNS1_10ptr_bucketE
pub fn stub_f46224() {
    // IDA 0xf46224: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>,RBX::StarterGuiService::CoreGuiType,bool,boost::hash<RBX::StarterGuiService::CoreGuiType>,std::equal_to<RBX::StarterGuiService::CoreGuiType>>>::operator[](RBX::StarterGuiService::CoreGuiType const&)")]
// 0xf46234 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEEixERS8_
pub fn stub_f46234() {
    // IDA 0xf46234: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>>>::construct(void)")]
// 0xf46244 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEEEEE9constructEv
pub fn stub_f46244() {
    // IDA 0xf46244: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>,RBX::StarterGuiService::CoreGuiType,bool,boost::hash<RBX::StarterGuiService::CoreGuiType>,std::equal_to<RBX::StarterGuiService::CoreGuiType>>>::create_buckets(unsigned long)")]
// 0xf46254 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm
pub fn stub_f46254() {
    // IDA 0xf46254: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>,RBX::StarterGuiService::CoreGuiType,bool,boost::hash<RBX::StarterGuiService::CoreGuiType>,std::equal_to<RBX::StarterGuiService::CoreGuiType>>>::delete_buckets(void)")]
// 0xf46264 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE14delete_bucketsEv
pub fn stub_f46264() {
    // IDA 0xf46264: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>,RBX::StarterGuiService::CoreGuiType,bool,boost::hash<RBX::StarterGuiService::CoreGuiType>,std::equal_to<RBX::StarterGuiService::CoreGuiType>>>::reserve_for_insert(unsigned long)")]
// 0xf46274 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE18reserve_for_insertEm
pub fn stub_f46274() {
    // IDA 0xf46274: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>,RBX::StarterGuiService::CoreGuiType,bool,boost::hash<RBX::StarterGuiService::CoreGuiType>,std::equal_to<RBX::StarterGuiService::CoreGuiType>>>::table(unsigned long,boost::hash<RBX::StarterGuiService::CoreGuiType> const&,std::equal_to<RBX::StarterGuiService::CoreGuiType> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>> const&)")]
// 0xf46284 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEEC2EmRKSC_RKSE_RKSaINS1_8ptr_nodeIS9_EEE
pub fn stub_f46284() {
    // IDA 0xf46284: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::function2<void,RBX::StarterGuiService::CoreGuiType,bool>::operator()(RBX::StarterGuiService::CoreGuiType,bool)const")]
// 0xf46324 — j___ZNK5boost9function2IvN3RBX17StarterGuiService11CoreGuiTypeEbEclES3_b
pub fn stub_f46324() {
    // IDA 0xf46324: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>,RBX::StarterGuiService::CoreGuiType,bool,boost::hash<RBX::StarterGuiService::CoreGuiType>,std::equal_to<RBX::StarterGuiService::CoreGuiType>>>::find_node_impl<RBX::StarterGuiService::CoreGuiType,std::equal_to<RBX::StarterGuiService::CoreGuiType>>(unsigned long,RBX::StarterGuiService::CoreGuiType const&,std::equal_to<RBX::StarterGuiService::CoreGuiType> const&)const")]
// 0xf46334 — j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE14find_node_implIS7_SE_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS9_EEEEmRKT_RKT0_
pub fn stub_f46334() {
    // IDA 0xf46334: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>,RBX::StarterGuiService::CoreGuiType,bool,boost::hash<RBX::StarterGuiService::CoreGuiType>,std::equal_to<RBX::StarterGuiService::CoreGuiType>>>::min_buckets_for_size(unsigned long)const")]
// 0xf46344 — j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE20min_buckets_for_sizeEm
pub fn stub_f46344() {
    // IDA 0xf46344: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "std::_Vector_base<RBX::StarterGuiService::CoreGuiType,std::allocator<RBX::StarterGuiService::CoreGuiType>>::_M_allocate(unsigned long)")]
// 0xf46354 — j___ZNSt12_Vector_baseIN3RBX17StarterGuiService11CoreGuiTypeESaIS2_EE11_M_allocateEm
pub fn stub_f46354() {
    // IDA 0xf46354: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "RBX::StarterGuiService::CoreGuiType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::StarterGuiService::CoreGuiType *,RBX::StarterGuiService::CoreGuiType *>(RBX::StarterGuiService::CoreGuiType *,RBX::StarterGuiService::CoreGuiType *,RBX::StarterGuiService::CoreGuiType *)")]
// 0xf46364 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX17StarterGuiService11CoreGuiTypeES6_EET0_T_S8_S7_
pub fn stub_f46364() {
    // IDA 0xf46364: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::StarterGuiService::CoreGuiType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>>>::operator[](RBX::Name const* const&)")]
// 0xf46374 — j___ZNSt3mapIPKN3RBX4NameENS0_17StarterGuiService11CoreGuiTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f46374() {
    // IDA 0xf46374: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::StarterGuiService::CoreGuiType,std::allocator<RBX::StarterGuiService::CoreGuiType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::StarterGuiService::CoreGuiType*,std::vector<RBX::StarterGuiService::CoreGuiType,std::allocator<RBX::StarterGuiService::CoreGuiType>>>,RBX::StarterGuiService::CoreGuiType const&)")]
// 0xf46384 — j___ZNSt6vectorIN3RBX17StarterGuiService11CoreGuiTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f46384() {
    // IDA 0xf46384: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::StarterGuiService::CoreGuiType,std::allocator<RBX::StarterGuiService::CoreGuiType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::StarterGuiService::CoreGuiType*,std::vector<RBX::StarterGuiService::CoreGuiType,std::allocator<RBX::StarterGuiService::CoreGuiType>>>,unsigned long,RBX::StarterGuiService::CoreGuiType const&)")]
// 0xf46394 — j___ZNSt6vectorIN3RBX17StarterGuiService11CoreGuiTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f46394() {
    // IDA 0xf46394: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::StarterGuiService::CoreGuiType,std::allocator<RBX::StarterGuiService::CoreGuiType>>::resize(unsigned long,RBX::StarterGuiService::CoreGuiType)")]
// 0xf463a4 — j___ZNSt6vectorIN3RBX17StarterGuiService11CoreGuiTypeESaIS2_EE6resizeEmS2_
pub fn stub_f463a4() {
    // IDA 0xf463a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::StarterGuiService::CoreGuiType,std::allocator<RBX::StarterGuiService::CoreGuiType>>::push_back(RBX::StarterGuiService::CoreGuiType const&)")]
// 0xf463b4 — j___ZNSt6vectorIN3RBX17StarterGuiService11CoreGuiTypeESaIS2_EE9push_backERKS2_
pub fn stub_f463b4() {
    // IDA 0xf463b4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType> const&)")]
// 0xf463c4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17StarterGuiService11CoreGuiTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f463c4() {
    // IDA 0xf463c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>>,std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType> const&)")]
// 0xf463d4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17StarterGuiService11CoreGuiTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f463d4() {
    // IDA 0xf463d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>> *)")]
// 0xf463e4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17StarterGuiService11CoreGuiTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f463e4() {
    // IDA 0xf463e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType> const&)")]
// 0xf463f4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17StarterGuiService11CoreGuiTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f463f4() {
    // IDA 0xf463f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::auto_ptr<RBX::World>::~auto_ptr()")]
// 0xf46644 — j___ZNSt8auto_ptrIN3RBX5WorldEED2Ev
pub fn stub_f46644() {
    // IDA 0xf46644: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ChatOption::ChatOption(std::string)")]
// 0xf46664 — j___ZN3RBX10ChatOptionC2ESs
pub fn stub_f46664() {
    // IDA 0xf46664: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::scoped_ptr<RBX::ChatOption>::~scoped_ptr()")]
// 0xf46674 — j___ZN5boost10scoped_ptrIN3RBX10ChatOptionEED2Ev
pub fn stub_f46674() {
    // IDA 0xf46674: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::scoped_ptr<RBX::SafeChat>::~scoped_ptr()")]
// 0xf46684 — j___ZN5boost10scoped_ptrIN3RBX8SafeChatEED2Ev
pub fn stub_f46684() {
    // IDA 0xf46684: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Vector_base<RBX::ChatOption *,std::allocator<RBX::ChatOption *>>::_M_allocate(unsigned long)")]
// 0xf46694 — j___ZNSt12_Vector_baseIPN3RBX10ChatOptionESaIS2_EE11_M_allocateEm
pub fn stub_f46694() {
    // IDA 0xf46694: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::ChatOption *,std::allocator<RBX::ChatOption *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::ChatOption **,std::vector<RBX::ChatOption *,std::allocator<RBX::ChatOption *>>>,RBX::ChatOption * const&)")]
// 0xf466a4 — j___ZNSt6vectorIPN3RBX10ChatOptionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f466a4() {
    // IDA 0xf466a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::ChatOption *,std::allocator<RBX::ChatOption *>>::push_back(RBX::ChatOption * const&)")]
// 0xf466b4 — j___ZNSt6vectorIPN3RBX10ChatOptionESaIS2_EE9push_backERKS2_
pub fn stub_f466b4() {
    // IDA 0xf466b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GuiMain::~GuiMain()")]
// 0xf467a4 — j___ZN3RBX7GuiMainD1Ev
pub fn stub_f467a4() {
    // IDA 0xf467a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ScreenGui::~ScreenGui()")]
// 0xf467c4 — j___ZN3RBX9ScreenGuiD1Ev
pub fn stub_f467c4() {
    // IDA 0xf467c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Adorn::getUserGuiRect(void)const")]
// 0xf46834 — j___ZNK3RBX5Adorn14getUserGuiRectEv
pub fn stub_f46834() {
    // IDA 0xf46834: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Vector_base<RBX::GuiButton *,std::allocator<RBX::GuiButton *>>::_M_allocate(unsigned long)")]
// 0xf46854 — j___ZNSt12_Vector_baseIPN3RBX9GuiButtonESaIS2_EE11_M_allocateEm
pub fn stub_f46854() {
    // IDA 0xf46854: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::GuiButton *,std::allocator<RBX::GuiButton *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GuiButton **,std::vector<RBX::GuiButton *,std::allocator<RBX::GuiButton *>>>,RBX::GuiButton * const&)")]
// 0xf46864 — j___ZNSt6vectorIPN3RBX9GuiButtonESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f46864() {
    // IDA 0xf46864: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::GuiButton *,std::allocator<RBX::GuiButton *>>::push_back(RBX::GuiButton * const&)")]
// 0xf46874 — j___ZNSt6vectorIPN3RBX9GuiButtonESaIS2_EE9push_backERKS2_
pub fn stub_f46874() {
    // IDA 0xf46874: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::SelectionChanged const&)>::operator()(RBX::SelectionChanged const&)")]
// 0xf46b14 — j___ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX16SelectionChangedEEEclES5_
pub fn stub_f46b14() {
    // IDA 0xf46b14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

