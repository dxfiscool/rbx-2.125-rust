//! platform generated_28 — next 100 stubs EA-sorted desc high-EA filler not yet in any crate
//! Filter: high-EA filler EA-sorted desc, rbx_core::SharedPtr not boost
//! Batch: 100 stubs EA-sorted desc | skeleton batch shard BG14 | range 0xf637c4..0xf64454 (rbx_core::SharedPtr not boost)

#![allow(
    non_snake_case,
    dead_code,
    unused_variables,
    unused_imports,
    clippy::all
)]

use rbx_core::SharedPtr;

// Ensure SharedPtr is seen as used — mirrors boost::shared_ptr<T> -> rbx_core::SharedPtr<T>
const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0xf64454 — j___ZN5boost12shared_mutex23unlock_upgrade_and_lockEv
// type: _DWORD __fastcall(boost::shared_mutex *__hidden this)
#[doc(alias = "boost::shared_mutex::unlock_upgrade_and_lock(void)")]
pub fn stub_f64454() -> ! {
    todo!("0xf64454 boost::shared_mutex::unlock_upgrade_and_lock(void)")
}

// 0xf64444 — j___ZN5boost12shared_mutex14unlock_upgradeEv
// type: _DWORD __fastcall(boost::shared_mutex *__hidden this)
#[doc(alias = "boost::shared_mutex::unlock_upgrade(void)")]
pub fn stub_f64444() -> ! {
    todo!("0xf64444 boost::shared_mutex::unlock_upgrade(void)")
}

// 0xf64434 — j___ZN5boost12shared_mutex13unlock_sharedEv
// type: _DWORD __fastcall(boost::shared_mutex *__hidden this)
#[doc(alias = "boost::shared_mutex::unlock_shared(void)")]
pub fn stub_f64434() -> ! {
    todo!("0xf64434 boost::shared_mutex::unlock_shared(void)")
}

// 0xf64424 — j___ZN5boost12shared_mutex12lock_upgradeEv
// type: void __fastcall(boost::shared_mutex *this, int, int, int)
#[doc(alias = "boost::shared_mutex::lock_upgrade(void)")]
pub fn stub_f64424() -> ! {
    todo!("0xf64424 boost::shared_mutex::lock_upgrade(void)")
}

// 0xf64414 — j___ZN5boost11unique_lockINS_12shared_mutexEE4lockEv
// type: int __fastcall(_DWORD)
#[doc(alias = "boost::unique_lock<boost::shared_mutex>::lock(void)")]
pub fn stub_f64414() -> ! {
    todo!("0xf64414 boost::unique_lock<boost::shared_mutex>::lock(void)")
}

// 0xf64404 — j___ZN5boost11shared_lockINS_12shared_mutexEE4lockEv
// type: int __fastcall(_DWORD)
#[doc(alias = "boost::shared_lock<boost::shared_mutex>::lock(void)")]
pub fn stub_f64404() -> ! {
    todo!("0xf64404 boost::shared_lock<boost::shared_mutex>::lock(void)")
}

// 0xf643d4 — j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE6insertEPNS8_4slotE
// type: void __fastcall(int32_t **, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::insert(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot *)")]
pub fn stub_f643d4() -> ! {
    todo!("0xf643d4 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::insert(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot *)")
}

// 0xf643c4 — j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE5mutexEv
// type: int __fastcall(_DWORD)
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::mutex(void)")]
pub fn stub_f643c4() -> ! {
    todo!("0xf643c4 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::mutex(void)")
}

// 0xf643b4 — j___ZN3RBX9Intrusive3SetINS_12PartInstanceENS_14PhysicsServiceEE8IteratordeEv
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator::operator*(void)")]
pub fn stub_f643b4() -> ! {
    todo!("0xf643b4 RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator::operator*(void)")
}

// 0xf643a4 — j___ZN3RBX11shared_fromINS_14PhysicsServiceEEEN5boost10shared_ptrIT_EEPS4_
// type: void __fastcall(int, int)
#[doc(alias = "boost::shared_ptr<RBX::PhysicsService> RBX::shared_from<RBX::PhysicsService>(RBX::PhysicsService*)")]
pub fn stub_f643a4() -> ! {
    todo!("0xf643a4 boost::shared_ptr<RBX::PhysicsService> RBX::shared_from<RBX::PhysicsService>(RBX::PhysicsService*)")
}

// 0xf64384 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10Reflection7VariantEESt10_Select1stIS5_ESt4lessISsESaIS5_EE7_M_copyEPKSt13_Rb_tree_nodeIS5_EPSD_
// type: int __fastcall(int, void *, int, int, int, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::Reflection::Variant>,std::_Select1st<std::pair<std::string const,RBX::Reflection::Variant>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>::_M_copy(std::_Rb_tree_node<std::pair<std::string const,RBX::Reflection::Variant>> const*,std::_Rb_tree_node<std::pair<std::string const,RBX::Reflection::Variant>>*)")]
pub fn stub_f64384() -> ! {
    todo!("0xf64384 std::_Rb_tree<std::string,std::pair<std::string const,RBX::Reflection::Variant>,std::_Select1st<std::pair<std::string const,RBX::Reflection::Variant>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>::_M_copy(std::_Rb_tree_node<std::pair<std::string const,RBX::Reflection::Variant>> const*,std::_Rb_tree_node<std::pair<std::string const,RBX::Reflection::Variant>>*)")
}

// 0xf64374 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10Reflection7VariantEESt10_Select1stIS5_ESt4lessISsESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_E
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::Reflection::Variant>,std::_Select1st<std::pair<std::string const,RBX::Reflection::Variant>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,RBX::Reflection::Variant>>)")]
pub fn stub_f64374() -> ! {
    todo!("0xf64374 std::_Rb_tree<std::string,std::pair<std::string const,RBX::Reflection::Variant>,std::_Select1st<std::pair<std::string const,RBX::Reflection::Variant>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,RBX::Reflection::Variant>>)")
}

// 0xf64364 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10Reflection7VariantEESt10_Select1stIS5_ESt4lessISsESaIS5_EE11equal_rangeERS1_
// type: 
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::Reflection::Variant>,std::_Select1st<std::pair<std::string const,RBX::Reflection::Variant>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>::equal_range(std::string const&)")]
pub fn stub_f64364() -> ! {
    todo!("0xf64364 std::_Rb_tree<std::string,std::pair<std::string const,RBX::Reflection::Variant>,std::_Select1st<std::pair<std::string const,RBX::Reflection::Variant>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>::equal_range(std::string const&)")
}

// 0xf64354 — j___ZNK3RBX10Reflection7Variant3getIN5boost10shared_ptrIKSt3mapISsS1_St4lessISsESaISt4pairIKSsS1_EEEEEEET_v
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> RBX::Reflection::Variant::get<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>(void)const")]
pub fn stub_f64354() -> ! {
    todo!("0xf64354 boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> RBX::Reflection::Variant::get<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>(void)const")
}

// 0xf64344 — j___ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueIPiEEEclIPFvRKSt4pairISsN3RBX10Reflection7VariantEES5_ENS0_5list1IRS9_IKSsSC_EEEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::_bi::value<int *>>::operator()<void (*)(std::pair<std::string,RBX::Reflection::Variant> const&,int *),boost::_bi::list1<std::pair&<std::string const,RBX::Reflection::Variant>>>(boost::_bi::type<void>,void (*)(std::pair<std::string,RBX::Reflection::Variant> const&,int *) &,boost::_bi::list1<std::pair&<std::string const,RBX::Reflection::Variant>> &,int)")]
pub fn stub_f64344() -> ! {
    todo!("0xf64344 void boost::_bi::list2<boost::arg<1>,boost::_bi::value<int *>>::operator()<void (*)(std::pair<std::string,RBX::Reflection::Variant> const&,int *),boost::_bi::list1<std::pair&<std::string const,RBX::Reflection::Variant>>>(boost::_bi::type<void>,void (*)(std::pair<std::string,RBX::Reflection::Variant> const&,int *) &,boost::_bi::list1<std::pair&<std::string const,RBX::Reflection::Variant>> &,int)")
}

// 0xf64334 — j___ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueIPiEEEclIPFvRKSt4pairISsN3RBX10Reflection7VariantEES5_ENS0_5list1IRKS9_IKSsSC_EEEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::_bi::value<int *>>::operator()<void (*)(std::pair<std::string,RBX::Reflection::Variant> const&,int *),boost::_bi::list1<std::pair const&<std::string const,RBX::Reflection::Variant>>>(boost::_bi::type<void>,void (*)(std::pair<std::string,RBX::Reflection::Variant> const&,int *) &,boost::_bi::list1<std::pair const&<std::string const,RBX::Reflection::Variant>> &,int)")]
pub fn stub_f64334() -> ! {
    todo!("0xf64334 void boost::_bi::list2<boost::arg<1>,boost::_bi::value<int *>>::operator()<void (*)(std::pair<std::string,RBX::Reflection::Variant> const&,int *),boost::_bi::list1<std::pair const&<std::string const,RBX::Reflection::Variant>>>(boost::_bi::type<void>,void (*)(std::pair<std::string,RBX::Reflection::Variant> const&,int *) &,boost::_bi::list1<std::pair const&<std::string const,RBX::Reflection::Variant>> &,int)")
}

// 0xf64324 — j___ZN5boost14checked_deleteI10XmlElementEEvPT_
// type: void __fastcall(XmlElement *)
#[doc(alias = "void boost::checked_delete<XmlElement>(XmlElement *)")]
pub fn stub_f64324() -> ! {
    todo!("0xf64324 void boost::checked_delete<XmlElement>(XmlElement *)")
}

// 0xf64314 — j___ZSt24__uninitialized_fill_n_aIPN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEEmS7_S7_EvT_T0_RKT1_SaIT2_E
// type: int __fastcall(_DWORD)
#[doc(alias = "void std::__uninitialized_fill_n_a<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,unsigned long,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>(boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,unsigned long,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> const&,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>)")]
pub fn stub_f64314() -> ! {
    todo!("0xf64314 void std::__uninitialized_fill_n_a<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,unsigned long,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>(boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,unsigned long,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> const&,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>)")
}

// 0xf642f4 — j___ZSt24__uninitialized_copy_auxIN9__gnu_cxx17__normal_iteratorIPKN3RBX7UintSetESt6vectorIS3_SaIS3_EEEEPS3_ET0_T_SC_SB_St12__false_type
// type: int __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, void *, int)
#[doc(alias = "RBX::UintSet* std::__uninitialized_copy_aux<__gnu_cxx::__normal_iterator<RBX::UintSet const*,std::vector<RBX::UintSet,std::allocator<RBX::UintSet>>>,RBX::UintSet*>(__gnu_cxx::__normal_iterator<RBX::UintSet const*,std::vector<RBX::UintSet,std::allocator<RBX::UintSet>>>,__gnu_cxx::__normal_iterator<RBX::UintSet const*,std::vector<RBX::UintSet,std::allocator<RBX::UintSet>>>,RBX::UintSet*,std::__false_type)")]
pub fn stub_f642f4() -> ! {
    todo!("0xf642f4 RBX::UintSet* std::__uninitialized_copy_aux<__gnu_cxx::__normal_iterator<RBX::UintSet const*,std::vector<RBX::UintSet,std::allocator<RBX::UintSet>>>,RBX::UintSet*>(__gnu_cxx::__normal_iterator<RBX::UintSet const*,std::vector<RBX::UintSet,std::allocator<RBX::UintSet>>>,__gnu_cxx::__normal_iterator<RBX::UintSet const*,std::vector<RBX::UintSet,std::allocator<RBX::UintSet>>>,RBX::UintSet*,std::__false_type)")
}

// 0xf642e4 — j___ZSt22__uninitialized_copy_aIPN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEES8_S7_ET0_T_SA_S9_SaIT1_E
// type: int __fastcall(int, int, int)
#[doc(alias = "boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> * std::__uninitialized_copy_a<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>(boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>)")]
pub fn stub_f642e4() -> ! {
    todo!("0xf642e4 boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> * std::__uninitialized_copy_a<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>(boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>)")
}

// 0xf642d4 — j___ZNSt8_Rb_treeISsSt4pairIKSshESt10_Select1stIS2_ESt4lessISsESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned char>,std::_Select1st<std::pair<std::string const,unsigned char>>,std::less<std::string>,std::allocator<std::pair<std::string const,unsigned char>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,unsigned char>> *)")]
pub fn stub_f642d4() -> ! {
    todo!("0xf642d4 std::_Rb_tree<std::string,std::pair<std::string const,unsigned char>,std::_Select1st<std::pair<std::string const,unsigned char>>,std::less<std::string>,std::allocator<std::pair<std::string const,unsigned char>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,unsigned char>> *)")
}

// 0xf642c4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10Reflection7VariantEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: void __fastcall(int, int *, int, int, char, int, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::Reflection::Variant>,std::_Select1st<std::pair<std::string const,RBX::Reflection::Variant>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,RBX::Reflection::Variant>> *)")]
pub fn stub_f642c4() -> ! {
    todo!("0xf642c4 std::_Rb_tree<std::string,std::pair<std::string const,RBX::Reflection::Variant>,std::_Select1st<std::pair<std::string const,RBX::Reflection::Variant>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,RBX::Reflection::Variant>> *)")
}

// 0xf642b4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10Reflection7VariantEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::Reflection::Variant>,std::_Select1st<std::pair<std::string const,RBX::Reflection::Variant>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,RBX::Reflection::Variant>>,std::pair<std::string const,RBX::Reflection::Variant> const&)")]
pub fn stub_f642b4() -> ! {
    todo!("0xf642b4 std::_Rb_tree<std::string,std::pair<std::string const,RBX::Reflection::Variant>,std::_Select1st<std::pair<std::string const,RBX::Reflection::Variant>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,RBX::Reflection::Variant>>,std::pair<std::string const,RBX::Reflection::Variant> const&)")
}

// 0xf642a4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10Reflection7VariantEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, _DWORD *, const void **, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::Reflection::Variant>,std::_Select1st<std::pair<std::string const,RBX::Reflection::Variant>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>::_M_insert_unique(std::pair<std::string const,RBX::Reflection::Variant> const&)")]
pub fn stub_f642a4() -> ! {
    todo!("0xf642a4 std::_Rb_tree<std::string,std::pair<std::string const,RBX::Reflection::Variant>,std::_Select1st<std::pair<std::string const,RBX::Reflection::Variant>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>::_M_insert_unique(std::pair<std::string const,RBX::Reflection::Variant> const&)")
}

// 0xf64294 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10Reflection7VariantEESt10_Select1stIS5_ESt4lessISsESaIS5_EE14_M_create_nodeERKS5_
// type: int __fastcall(int, int, int, int, void *, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::Reflection::Variant>,std::_Select1st<std::pair<std::string const,RBX::Reflection::Variant>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>::_M_create_node(std::pair<std::string const,RBX::Reflection::Variant> const&)")]
pub fn stub_f64294() -> ! {
    todo!("0xf64294 std::_Rb_tree<std::string,std::pair<std::string const,RBX::Reflection::Variant>,std::_Select1st<std::pair<std::string const,RBX::Reflection::Variant>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>::_M_create_node(std::pair<std::string const,RBX::Reflection::Variant> const&)")
}

// 0xf64284 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N5boost10shared_ptrINS0_8InstanceEEEESt10_Select1stISA_ESt4lessIS3_ESaISA_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISA_ERKSA_
// type: int __fastcall(int, _Rb_tree_node_base *, int)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>,std::_Select1st<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>,std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>> const&)")]
pub fn stub_f64284() -> ! {
    todo!("0xf64284 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>,std::_Select1st<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>,std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>> const&)")
}

// 0xf64274 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N5boost10shared_ptrINS0_8InstanceEEEESt10_Select1stISA_ESt4lessIS3_ESaISA_EE16_M_insert_uniqueERKSA_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>,std::_Select1st<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>>::_M_insert_unique(std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>> const&)")]
pub fn stub_f64274() -> ! {
    todo!("0xf64274 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>,std::_Select1st<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>>::_M_insert_unique(std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>> const&)")
}

// 0xf64264 — j___ZNSt8_Rb_treeIPKN3RBX10Reflection4TypeESt4pairIKS4_jESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::Reflection::Type const*,std::pair<RBX::Reflection::Type const* const,unsigned int>,std::_Select1st<std::pair<RBX::Reflection::Type const* const,unsigned int>>,std::less<RBX::Reflection::Type const*>,std::allocator<std::pair<RBX::Reflection::Type const* const,unsigned int>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Reflection::Type const* const,unsigned int>> *)")]
pub fn stub_f64264() -> ! {
    todo!("0xf64264 std::_Rb_tree<RBX::Reflection::Type const*,std::pair<RBX::Reflection::Type const* const,unsigned int>,std::_Select1st<std::pair<RBX::Reflection::Type const* const,unsigned int>>,std::less<RBX::Reflection::Type const*>,std::allocator<std::pair<RBX::Reflection::Type const* const,unsigned int>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Reflection::Type const* const,unsigned int>> *)")
}

// 0xf64254 — j___ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_jESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,unsigned int>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,unsigned int>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,unsigned int>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Reflection::PropertyDescriptor const* const,unsigned int>> *)")]
pub fn stub_f64254() -> ! {
    todo!("0xf64254 std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,unsigned int>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,unsigned int>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,unsigned int>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Reflection::PropertyDescriptor const* const,unsigned int>> *)")
}

// 0xf641c4 — j___ZNSt8_Rb_treeIPKN3RBX10Reflection15EventDescriptorESt4pairIKS4_jESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::Reflection::EventDescriptor const*,std::pair<RBX::Reflection::EventDescriptor const* const,unsigned int>,std::_Select1st<std::pair<RBX::Reflection::EventDescriptor const* const,unsigned int>>,std::less<RBX::Reflection::EventDescriptor const*>,std::allocator<std::pair<RBX::Reflection::EventDescriptor const* const,unsigned int>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Reflection::EventDescriptor const* const,unsigned int>> *)")]
pub fn stub_f641c4() -> ! {
    todo!("0xf641c4 std::_Rb_tree<RBX::Reflection::EventDescriptor const*,std::pair<RBX::Reflection::EventDescriptor const* const,unsigned int>,std::_Select1st<std::pair<RBX::Reflection::EventDescriptor const* const,unsigned int>>,std::less<RBX::Reflection::EventDescriptor const*>,std::allocator<std::pair<RBX::Reflection::EventDescriptor const* const,unsigned int>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Reflection::EventDescriptor const* const,unsigned int>> *)")
}

// 0xf64174 — j___ZNSt8_Rb_treeIPKN3RBX10Reflection15ClassDescriptorESt4pairIKS4_jESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Reflection::ClassDescriptor const*,std::pair<RBX::Reflection::ClassDescriptor const* const,unsigned int>,std::_Select1st<std::pair<RBX::Reflection::ClassDescriptor const* const,unsigned int>>,std::less<RBX::Reflection::ClassDescriptor const*>,std::allocator<std::pair<RBX::Reflection::ClassDescriptor const* const,unsigned int>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Reflection::ClassDescriptor const* const,unsigned int>> *)")]
pub fn stub_f64174() -> ! {
    todo!("0xf64174 std::_Rb_tree<RBX::Reflection::ClassDescriptor const*,std::pair<RBX::Reflection::ClassDescriptor const* const,unsigned int>,std::_Select1st<std::pair<RBX::Reflection::ClassDescriptor const* const,unsigned int>>,std::less<RBX::Reflection::ClassDescriptor const*>,std::allocator<std::pair<RBX::Reflection::ClassDescriptor const* const,unsigned int>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Reflection::ClassDescriptor const* const,unsigned int>> *)")
}

// 0xf64154 — j___ZNSt8_Rb_treeIN3RBX13SystemAddressESt4pairIKS1_hESt10_Select1stIS4_ESt4lessIS1_ESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::SystemAddress,std::pair<RBX::SystemAddress const,unsigned char>,std::_Select1st<std::pair<RBX::SystemAddress const,unsigned char>>,std::less<RBX::SystemAddress>,std::allocator<std::pair<RBX::SystemAddress const,unsigned char>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::SystemAddress const,unsigned char>> *)")]
pub fn stub_f64154() -> ! {
    todo!("0xf64154 std::_Rb_tree<RBX::SystemAddress,std::pair<RBX::SystemAddress const,unsigned char>,std::_Select1st<std::pair<RBX::SystemAddress const,unsigned char>>,std::less<RBX::SystemAddress>,std::allocator<std::pair<RBX::SystemAddress const,unsigned char>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::SystemAddress const,unsigned char>> *)")
}

// 0xf64144 — j___ZNSt8_Rb_treeIN3RBX13SystemAddressESt4pairIKS1_hESt10_Select1stIS4_ESt4lessIS1_ESaIS4_EE5eraseERS3_
// type: int __fastcall(int, unsigned int *)
#[doc(alias = "std::_Rb_tree<RBX::SystemAddress,std::pair<RBX::SystemAddress const,unsigned char>,std::_Select1st<std::pair<RBX::SystemAddress const,unsigned char>>,std::less<RBX::SystemAddress>,std::allocator<std::pair<RBX::SystemAddress const,unsigned char>>>::erase(RBX::SystemAddress const&)")]
pub fn stub_f64144() -> ! {
    todo!("0xf64144 std::_Rb_tree<RBX::SystemAddress,std::pair<RBX::SystemAddress const,unsigned char>,std::_Select1st<std::pair<RBX::SystemAddress const,unsigned char>>,std::less<RBX::SystemAddress>,std::allocator<std::pair<RBX::SystemAddress const,unsigned char>>>::erase(RBX::SystemAddress const&)")
}

// 0xf64134 — j___ZNSt8_Rb_treeIN3RBX13SystemAddressESt4pairIKS1_hESt10_Select1stIS4_ESt4lessIS1_ESaIS4_EE16_M_insert_uniqueERKS4_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::SystemAddress,std::pair<RBX::SystemAddress const,unsigned char>,std::_Select1st<std::pair<RBX::SystemAddress const,unsigned char>>,std::less<RBX::SystemAddress>,std::allocator<std::pair<RBX::SystemAddress const,unsigned char>>>::_M_insert_unique(std::pair<RBX::SystemAddress const,unsigned char> const&)")]
pub fn stub_f64134() -> ! {
    todo!("0xf64134 std::_Rb_tree<RBX::SystemAddress,std::pair<RBX::SystemAddress const,unsigned char>,std::_Select1st<std::pair<RBX::SystemAddress const,unsigned char>>,std::less<RBX::SystemAddress>,std::allocator<std::pair<RBX::SystemAddress const,unsigned char>>>::_M_insert_unique(std::pair<RBX::SystemAddress const,unsigned char> const&)")
}

// 0xf64124 — j___ZNSt6vectorIjSaIjEEaSERKS1_
// type: int __fastcall(int, __int64 *)
#[doc(alias = "std::vector<unsigned int,std::allocator<unsigned int>>::operator=(std::vector<unsigned int,std::allocator<unsigned int>> const&)")]
pub fn stub_f64124() -> ! {
    todo!("0xf64124 std::vector<unsigned int,std::allocator<unsigned int>>::operator=(std::vector<unsigned int,std::allocator<unsigned int>> const&)")
}

// 0xf64114 — j___ZNSt6vectorIPN3RBX8InstanceESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<RBX::Instance *,std::allocator<RBX::Instance *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Instance **,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>>>,RBX::Instance * const&)")]
pub fn stub_f64114() -> ! {
    todo!("0xf64114 std::vector<RBX::Instance *,std::allocator<RBX::Instance *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Instance **,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>>>,RBX::Instance * const&)")
}

// 0xf64104 — j___ZNSt6vectorIPKN3RBX10Reflection4TypeESaIS4_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS4_S6_EEmRKS4_
// type: char *__fastcall(char *result, _DWORD *__src, unsigned int, int *)
#[doc(alias = "std::vector<RBX::Reflection::Type const*,std::allocator<RBX::Reflection::Type const*>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Reflection::Type const**,std::vector<RBX::Reflection::Type const*,std::allocator<RBX::Reflection::Type const*>>>,unsigned long,RBX::Reflection::Type const* const&)")]
pub fn stub_f64104() -> ! {
    todo!("0xf64104 std::vector<RBX::Reflection::Type const*,std::allocator<RBX::Reflection::Type const*>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Reflection::Type const**,std::vector<RBX::Reflection::Type const*,std::allocator<RBX::Reflection::Type const*>>>,unsigned long,RBX::Reflection::Type const* const&)")
}

// 0xf640f4 — j___ZNSt6vectorIPKN3RBX10Reflection18PropertyDescriptorESaIS4_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS4_S6_EEmRKS4_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<RBX::Reflection::PropertyDescriptor const*,std::allocator<RBX::Reflection::PropertyDescriptor const*>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Reflection::PropertyDescriptor const**,std::vector<RBX::Reflection::PropertyDescriptor const*,std::allocator<RBX::Reflection::PropertyDescriptor const*>>>,unsigned long,RBX::Reflection::PropertyDescriptor const* const&)")]
pub fn stub_f640f4() -> ! {
    todo!("0xf640f4 std::vector<RBX::Reflection::PropertyDescriptor const*,std::allocator<RBX::Reflection::PropertyDescriptor const*>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Reflection::PropertyDescriptor const**,std::vector<RBX::Reflection::PropertyDescriptor const*,std::allocator<RBX::Reflection::PropertyDescriptor const*>>>,unsigned long,RBX::Reflection::PropertyDescriptor const* const&)")
}

// 0xf640e4 — j___ZNSt6vectorIPKN3RBX10Reflection15EventDescriptorESaIS4_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS4_S6_EEmRKS4_
// type: char *__fastcall(char *result, _DWORD *__src, unsigned int, int *)
#[doc(alias = "std::vector<RBX::Reflection::EventDescriptor const*,std::allocator<RBX::Reflection::EventDescriptor const*>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Reflection::EventDescriptor const**,std::vector<RBX::Reflection::EventDescriptor const*,std::allocator<RBX::Reflection::EventDescriptor const*>>>,unsigned long,RBX::Reflection::EventDescriptor const* const&)")]
pub fn stub_f640e4() -> ! {
    todo!("0xf640e4 std::vector<RBX::Reflection::EventDescriptor const*,std::allocator<RBX::Reflection::EventDescriptor const*>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Reflection::EventDescriptor const**,std::vector<RBX::Reflection::EventDescriptor const*,std::allocator<RBX::Reflection::EventDescriptor const*>>>,unsigned long,RBX::Reflection::EventDescriptor const* const&)")
}

// 0xf640d4 — j___ZNSt6vectorIPKN3RBX10Reflection15ClassDescriptorESaIS4_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS4_S6_EEmRKS4_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<RBX::Reflection::ClassDescriptor const*,std::allocator<RBX::Reflection::ClassDescriptor const*>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Reflection::ClassDescriptor const**,std::vector<RBX::Reflection::ClassDescriptor const*,std::allocator<RBX::Reflection::ClassDescriptor const*>>>,unsigned long,RBX::Reflection::ClassDescriptor const* const&)")]
pub fn stub_f640d4() -> ! {
    todo!("0xf640d4 std::vector<RBX::Reflection::ClassDescriptor const*,std::allocator<RBX::Reflection::ClassDescriptor const*>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Reflection::ClassDescriptor const**,std::vector<RBX::Reflection::ClassDescriptor const*,std::allocator<RBX::Reflection::ClassDescriptor const*>>>,unsigned long,RBX::Reflection::ClassDescriptor const* const&)")
}

// 0xf640c4 — j___ZNSt6vectorIN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEESaIS7_EED1Ev
// type: 
#[doc(alias = "std::vector<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>::~vector()")]
pub fn stub_f640c4() -> ! {
    todo!("0xf640c4 std::vector<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>::~vector()")
}

// 0xf640b4 — j___ZNSt6vectorIN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEESaIS7_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS7_S9_EEmRKS7_
// type: 
#[doc(alias = "std::vector<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>*,std::vector<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>>,unsigned long,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> const&)")]
pub fn stub_f640b4() -> ! {
    todo!("0xf640b4 std::vector<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>*,std::vector<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>>,unsigned long,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> const&)")
}

// 0xf640a4 — j___ZNSt6vectorIN3RBX7UintSetESaIS1_EEaSERKS3_
// type: struct _Unwind_Exception *__fastcall(struct _Unwind_Exception *, int *)
#[doc(alias = "std::vector<RBX::UintSet,std::allocator<RBX::UintSet>>::operator=(std::vector<RBX::UintSet,std::allocator<RBX::UintSet>> const&)")]
pub fn stub_f640a4() -> ! {
    todo!("0xf640a4 std::vector<RBX::UintSet,std::allocator<RBX::UintSet>>::operator=(std::vector<RBX::UintSet,std::allocator<RBX::UintSet>> const&)")
}

// 0xf64044 — j___ZNSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS2_EEEixERS6_
// type: uint32_t *__fastcall(int, const void **)
#[doc(alias = "std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>::operator[](std::string const&)")]
pub fn stub_f64044() -> ! {
    todo!("0xf64044 std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>::operator[](std::string const&)")
}

// 0xf63fe4 — j___ZNK5boost9function2IvSsbEclESsb
// type: void __fastcall(_DWORD *, const std::string *, int)
#[doc(alias = "boost::function2<void,std::string,bool>::operator()(std::string,bool)const")]
pub fn stub_f63fe4() -> ! {
    todo!("0xf63fe4 boost::function2<void,std::string,bool>::operator()(std::string,bool)const")
}

// 0xf63fd4 — j___ZNK5boost6detail8function13basic_vtable2IvSsbE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsRKbEENS5_5list3INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, void *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string,bool>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,bool const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,bool const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_f63fd4() -> ! {
    todo!("0xf63fd4 bool boost::detail::function::basic_vtable2<void,std::string,bool>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,bool const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,bool const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0xf63fc4 — j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EEENS8_5list1INS8_5valueISB_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int *, int *, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_f63fc4() -> ! {
    todo!("0xf63fc4 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0xf63f24 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9BlockMeshES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BlockMesh,RBX::BlockMesh>(boost::shared_ptr<RBX::BlockMesh> const*,RBX::BlockMesh *)const")]
pub fn stub_f63f24() -> ! {
    todo!("0xf63f24 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BlockMesh,RBX::BlockMesh>(boost::shared_ptr<RBX::BlockMesh> const*,RBX::BlockMesh *)const")
}

// 0xf63f04 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12BadgeServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BadgeService,RBX::BadgeService>(boost::shared_ptr<RBX::BadgeService> const*,RBX::BadgeService *)const")]
pub fn stub_f63f04() -> ! {
    todo!("0xf63f04 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BadgeService,RBX::BadgeService>(boost::shared_ptr<RBX::BadgeService> const*,RBX::BadgeService *)const")
}

// 0xf63ef4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11TestServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TestService,RBX::TestService>(boost::shared_ptr<RBX::TestService> const*,RBX::TestService *)const")]
pub fn stub_f63ef4() -> ! {
    todo!("0xf63ef4 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TestService,RBX::TestService>(boost::shared_ptr<RBX::TestService> const*,RBX::TestService *)const")
}

// 0xf63ee4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11StringValueES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::StringValue,RBX::StringValue>(boost::shared_ptr<RBX::StringValue> const*,RBX::StringValue *)const")]
pub fn stub_f63ee4() -> ! {
    todo!("0xf63ee4 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::StringValue,RBX::StringValue>(boost::shared_ptr<RBX::StringValue> const*,RBX::StringValue *)const")
}

// 0xf63ed4 — j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEE5cloneEv
// type: 
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::clone(void)const")]
pub fn stub_f63ed4() -> ! {
    todo!("0xf63ed4 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::clone(void)const")
}

// 0xf63d84 — j___ZNK3RBX15ServiceProvider6createINS_8LightingEEEPT_v
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, RBX::Instance *, int, int, void *, int)
#[doc(alias = "RBX::Lighting * RBX::ServiceProvider::create<RBX::Lighting>(void)const")]
pub fn stub_f63d84() -> ! {
    todo!("0xf63d84 RBX::Lighting * RBX::ServiceProvider::create<RBX::Lighting>(void)const")
}

// 0xf63d64 — j___ZNK3RBX15ServiceProvider6createINS_5TeamsEEEPT_v
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, RBX::Instance *, int, int, void *, int)
#[doc(alias = "RBX::Teams * RBX::ServiceProvider::create<RBX::Teams>(void)const")]
pub fn stub_f63d64() -> ! {
    todo!("0xf63d64 RBX::Teams * RBX::ServiceProvider::create<RBX::Teams>(void)const")
}

// 0xf63d54 — j___ZNK3RBX15ServiceProvider6createINS_18MarketplaceServiceEEEPT_v
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, RBX::Instance *, int, int, void *, int)
#[doc(alias = "RBX::MarketplaceService * RBX::ServiceProvider::create<RBX::MarketplaceService>(void)const")]
pub fn stub_f63d54() -> ! {
    todo!("0xf63d54 RBX::MarketplaceService * RBX::ServiceProvider::create<RBX::MarketplaceService>(void)const")
}

// 0xf63d44 — j___ZNK3RBX15ServiceProvider6createINS_17ReplicatedStorageEEEPT_v
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, RBX::Instance *, int, int, void *, int)
#[doc(alias = "RBX::ReplicatedStorage * RBX::ServiceProvider::create<RBX::ReplicatedStorage>(void)const")]
pub fn stub_f63d44() -> ! {
    todo!("0xf63d44 RBX::ReplicatedStorage * RBX::ServiceProvider::create<RBX::ReplicatedStorage>(void)const")
}

// 0xf63d34 — j___ZNK3RBX15ServiceProvider6createINS_11TestServiceEEEPT_v
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, RBX::Instance *, int, int, void *, int)
#[doc(alias = "RBX::TestService * RBX::ServiceProvider::create<RBX::TestService>(void)const")]
pub fn stub_f63d34() -> ! {
    todo!("0xf63d34 RBX::TestService * RBX::ServiceProvider::create<RBX::TestService>(void)const")
}

// 0xf63d24 — j___ZNK3RBX15ServiceProvider4findINS_8LightingEEEPT_v
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Lighting * RBX::ServiceProvider::find<RBX::Lighting>(void)const")]
pub fn stub_f63d24() -> ! {
    todo!("0xf63d24 RBX::Lighting * RBX::ServiceProvider::find<RBX::Lighting>(void)const")
}

// 0xf63cf4 — j___ZNK3RBX15ServiceProvider4findINS_5Stats12StatsServiceEEEPT_v
// type: int __fastcall(int, int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Stats::StatsService * RBX::ServiceProvider::find<RBX::Stats::StatsService>(void)const")]
pub fn stub_f63cf4() -> ! {
    todo!("0xf63cf4 RBX::Stats::StatsService * RBX::ServiceProvider::find<RBX::Stats::StatsService>(void)const")
}

// 0xf63ce4 — j___ZNK3RBX15ServiceProvider4findINS_17ReplicatedStorageEEEPT_v
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::ReplicatedStorage * RBX::ServiceProvider::find<RBX::ReplicatedStorage>(void)const")]
pub fn stub_f63ce4() -> ! {
    todo!("0xf63ce4 RBX::ReplicatedStorage * RBX::ServiceProvider::find<RBX::ReplicatedStorage>(void)const")
}

// 0xf63cd4 — j___ZNK3RBX14FactoryProductINS_8LightingENS_8InstanceELZNS_9sLightingEES2_E7Creator12getClassNameEv
// type: int __fastcall(_DWORD)
#[doc(alias = "j___ZNK3RBX14FactoryProductINS_8LightingENS_8InstanceELZNS_9sLightingEES2_E7Creator12getClassNameEv")]
pub fn stub_f63cd4() -> ! {
    todo!("0xf63cd4 j___ZNK3RBX14FactoryProductINS_8LightingENS_8InstanceELZNS_9sLightingEES2_E7Creator12getClassNameEv")
}

// 0xf63cc4 — j___ZNK3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E7Creator12getClassNameEv
// type: 
#[doc(alias = "j___ZNK3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E7Creator12getClassNameEv")]
pub fn stub_f63cc4() -> ! {
    todo!("0xf63cc4 j___ZNK3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E7Creator12getClassNameEv")
}

// 0xf63cb4 — j___ZNK3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7Creator12getClassNameEv
// type: 
#[doc(alias = "j___ZNK3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_f63cb4() -> ! {
    todo!("0xf63cb4 j___ZNK3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7Creator12getClassNameEv")
}

// 0xf63ca4 — j___ZNK3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE12getClassNameEv
// type: 
#[doc(alias = "j___ZNK3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE12getClassNameEv")]
pub fn stub_f63ca4() -> ! {
    todo!("0xf63ca4 j___ZNK3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE12getClassNameEv")
}

// 0xf63c94 — j___ZNK3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "j___ZNK3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E7Creator12getClassNameEv")]
pub fn stub_f63c94() -> ! {
    todo!("0xf63c94 j___ZNK3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E7Creator12getClassNameEv")
}

// 0xf63c84 — j___ZNK3RBX10Reflection7Variant3getIiEET_v
// type: 
#[doc(alias = "int RBX::Reflection::Variant::get<int>(void)const")]
pub fn stub_f63c84() -> ! {
    todo!("0xf63c84 int RBX::Reflection::Variant::get<int>(void)const")
}

// 0xf63c74 — j___ZNK3RBX10Reflection7Variant3getISsEET_v
// type: 
#[doc(alias = "std::string RBX::Reflection::Variant::get<std::string>(void)const")]
pub fn stub_f63c74() -> ! {
    todo!("0xf63c74 std::string RBX::Reflection::Variant::get<std::string>(void)const")
}

// 0xf63c54 — j___ZNK3RBX10Reflection13ConstProperty8getValueIiEET_v
// type: int __fastcall(_DWORD)
#[doc(alias = "int RBX::Reflection::ConstProperty::getValue<int>(void)const")]
pub fn stub_f63c54() -> ! {
    todo!("0xf63c54 int RBX::Reflection::ConstProperty::getValue<int>(void)const")
}

// 0xf63c44 — j___ZNK3RBX10Reflection13ConstProperty8getValueIfEET_v
// type: int __fastcall(_DWORD *, int, int)
#[doc(alias = "float RBX::Reflection::ConstProperty::getValue<float>(void)const")]
pub fn stub_f63c44() -> ! {
    todo!("0xf63c44 float RBX::Reflection::ConstProperty::getValue<float>(void)const")
}

// 0xf63c34 — j___ZNK3RBX10Reflection13ConstProperty8getValueIdEET_v
// type: __int64 __fastcall(_DWORD)
#[doc(alias = "double RBX::Reflection::ConstProperty::getValue<double>(void)const")]
pub fn stub_f63c34() -> ! {
    todo!("0xf63c34 double RBX::Reflection::ConstProperty::getValue<double>(void)const")
}

// 0xf63c24 — j___ZNK3RBX10Reflection13ConstProperty8getValueIbEET_v
// type: int __fastcall(_DWORD *, int)
#[doc(alias = "bool RBX::Reflection::ConstProperty::getValue<bool>(void)const")]
pub fn stub_f63c24() -> ! {
    todo!("0xf63c24 bool RBX::Reflection::ConstProperty::getValue<bool>(void)const")
}

// 0xf63c14 — j___ZNK3RBX10Reflection13ConstProperty8getValueINS_13SystemAddressEEET_v
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::SystemAddress RBX::Reflection::ConstProperty::getValue<RBX::SystemAddress>(void)const")]
pub fn stub_f63c14() -> ! {
    todo!("0xf63c14 RBX::SystemAddress RBX::Reflection::ConstProperty::getValue<RBX::SystemAddress>(void)const")
}

// 0xf63c04 — j___ZNK3RBX10Reflection13ConstProperty8getValueIN3G3D7Vector3EEET_v
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "G3D::Vector3 RBX::Reflection::ConstProperty::getValue<G3D::Vector3>(void)const")]
pub fn stub_f63c04() -> ! {
    todo!("0xf63c04 G3D::Vector3 RBX::Reflection::ConstProperty::getValue<G3D::Vector3>(void)const")
}

// 0xf63bf4 — j___ZNK3RBX10Reflection13ConstProperty8getValueIN3G3D7Vector2EEET_v
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "G3D::Vector2 RBX::Reflection::ConstProperty::getValue<G3D::Vector2>(void)const")]
pub fn stub_f63bf4() -> ! {
    todo!("0xf63bf4 G3D::Vector2 RBX::Reflection::ConstProperty::getValue<G3D::Vector2>(void)const")
}

// 0xf63be4 — j___ZNK3RBX10Reflection13ConstProperty8getValueIN3G3D6Color3EEET_v
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "G3D::Color3 RBX::Reflection::ConstProperty::getValue<G3D::Color3>(void)const")]
pub fn stub_f63be4() -> ! {
    todo!("0xf63be4 G3D::Color3 RBX::Reflection::ConstProperty::getValue<G3D::Color3>(void)const")
}

// 0xf63bd4 — j___ZNK3RBX10Reflection13ConstProperty8getValueIN3G3D15CoordinateFrameEEET_v
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "G3D::CoordinateFrame RBX::Reflection::ConstProperty::getValue<G3D::CoordinateFrame>(void)const")]
pub fn stub_f63bd4() -> ! {
    todo!("0xf63bd4 G3D::CoordinateFrame RBX::Reflection::ConstProperty::getValue<G3D::CoordinateFrame>(void)const")
}

// 0xf63bc4 — j___ZNK3RBX10Reflection13ConstProperty8getValueIN3G3D12Vector2int16EEET_v
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "G3D::Vector2int16 RBX::Reflection::ConstProperty::getValue<G3D::Vector2int16>(void)const")]
pub fn stub_f63bc4() -> ! {
    todo!("0xf63bc4 G3D::Vector2int16 RBX::Reflection::ConstProperty::getValue<G3D::Vector2int16>(void)const")
}

// 0xf63bb4 — j___ZN5boost9unordered6detail5tableINS1_3setISaIPKN3RBX8InstanceEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE18reserve_for_insertEm
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance const*>,RBX::Instance const*,boost::hash<RBX::Instance const*>,std::equal_to<RBX::Instance const*>>>::reserve_for_insert(unsigned long)")]
pub fn stub_f63bb4() -> ! {
    todo!("0xf63bb4 boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance const*>,RBX::Instance const*,boost::hash<RBX::Instance const*>,std::equal_to<RBX::Instance const*>>>::reserve_for_insert(unsigned long)")
}

// 0xf63ba4 — j___ZN5boost9unordered6detail5tableINS1_3setISaIPKN3RBX8InstanceEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm
// type: unsigned int __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance const*>,RBX::Instance const*,boost::hash<RBX::Instance const*>,std::equal_to<RBX::Instance const*>>>::create_buckets(unsigned long)")]
pub fn stub_f63ba4() -> ! {
    todo!("0xf63ba4 boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance const*>,RBX::Instance const*,boost::hash<RBX::Instance const*>,std::equal_to<RBX::Instance const*>>>::create_buckets(unsigned long)")
}

// 0xf63b44 — j___ZN5boost9unordered6detail17array_constructorINS_19fast_pool_allocatorINS1_10ptr_bucketENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEEE9constructIS4_EEvRKT_m
// type: void __fastcall(_DWORD *, _DWORD *, int)
#[doc(alias = "void boost::unordered::detail::array_constructor<boost::fast_pool_allocator<boost::unordered::detail::ptr_bucket,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>>::construct<boost::unordered::detail::ptr_bucket>(boost::unordered::detail::ptr_bucket const&,unsigned long)")]
pub fn stub_f63b44() -> ! {
    todo!("0xf63b44 void boost::unordered::detail::array_constructor<boost::fast_pool_allocator<boost::unordered::detail::ptr_bucket,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>>::construct<boost::unordered::detail::ptr_bucket>(boost::unordered::detail::ptr_bucket const&,unsigned long)")
}

// 0xf63b04 — j___ZN5boost9unordered6detail10table_implINS1_3setISaIPKN3RBX8InstanceEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE9erase_keyERKS7_
// type: int __fastcall(_DWORD *, unsigned int *)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Instance const*>,RBX::Instance const*,boost::hash<RBX::Instance const*>,std::equal_to<RBX::Instance const*>>>::erase_key(RBX::Instance const* const&)")]
pub fn stub_f63b04() -> ! {
    todo!("0xf63b04 boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Instance const*>,RBX::Instance const*,boost::hash<RBX::Instance const*>,std::equal_to<RBX::Instance const*>>>::erase_key(RBX::Instance const* const&)")
}

// 0xf63af4 — j___ZN5boost9unordered6detail10table_implINS1_3setISaIPKN3RBX8InstanceEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE12emplace_implINS1_13emplace_args1IS7_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEEbERKS7_RKT_
// type: int __fastcall(int, int, int, int, int, void *, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::Instance const*>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Instance const*>,RBX::Instance const*,boost::hash<RBX::Instance const*>,std::equal_to<RBX::Instance const*>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::Instance const*>>(RBX::Instance const* const&,boost::unordered::detail::emplace_args1<RBX::Instance const*> const&)")]
pub fn stub_f63af4() -> ! {
    todo!("0xf63af4 std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::Instance const*>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Instance const*>,RBX::Instance const*,boost::hash<RBX::Instance const*>,std::equal_to<RBX::Instance const*>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::Instance const*>>(RBX::Instance const* const&,boost::unordered::detail::emplace_args1<RBX::Instance const*> const&)")
}

// 0xf63aa4 — j___ZN5boost9function2IvSsbE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsRKbEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSK_ILi2EEEEEEEEEvT_
// type: void __fastcall(int, int *, int, int, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void boost::function2<void,std::string,bool>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,bool const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,bool const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")]
pub fn stub_f63aa4() -> ! {
    todo!("0xf63aa4 void boost::function2<void,std::string,bool>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,bool const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,bool const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")
}

// 0xf63a94 — j___ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EEENS6_5list1INS6_5valueIS9_EEEEEEEEvT_
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>)")]
pub fn stub_f63a94() -> ! {
    todo!("0xf63a94 void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>)")
}

// 0xf63a74 — j___ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEE4swapERS5_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "boost::function1<void,boost::shared_ptr<RBX::Instance>>::swap(boost::function1<void,boost::shared_ptr<RBX::Instance>>&)")]
pub fn stub_f63a74() -> ! {
    todo!("0xf63a74 boost::function1<void,boost::shared_ptr<RBX::Instance>>::swap(boost::function1<void,boost::shared_ptr<RBX::Instance>>&)")
}

// 0xf63a64 — j___ZN5boost8functionIFvSsbEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsRKbEENS4_5list3INS4_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEENSL_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *, int *, int, int, pthread_mutex_t *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "j___ZN5boost8functionIFvSsbEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsRKbEENS4_5list3INS4_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEENSL_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_f63a64() -> ! {
    todo!("0xf63a64 j___ZN5boost8functionIFvSsbEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsRKbEENS4_5list3INS4_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEENSL_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")
}

// 0xf63a34 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsRKbEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSK_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,bool const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_f63a34() -> ! {
    todo!("0xf63a34 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,bool const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0xf63a24 — j___ZN5boost6detail20sp_pointer_constructISt6vectorIN3RBX10Reflection7VariantESaIS5_EES7_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>> *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *,boost::detail::shared_count &)")]
pub fn stub_f63a24() -> ! {
    todo!("0xf63a24 void boost::detail::sp_pointer_construct<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>> *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *,boost::detail::shared_count &)")
}

// 0xf63a14 — j___ZN5boost6detail20sp_pointer_constructISt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEESC_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>,std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>(boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> *,std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,boost::detail::shared_count &)")]
pub fn stub_f63a14() -> ! {
    todo!("0xf63a14 void boost::detail::sp_pointer_construct<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>,std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>(boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> *,std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,boost::detail::shared_count &)")
}

// 0xf63a04 — j___ZN5boost6detail20sp_pointer_constructINS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEESF_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>,boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> *,boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,boost::detail::shared_count &)")]
pub fn stub_f63a04() -> ! {
    todo!("0xf63a04 void boost::detail::sp_pointer_construct<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>,boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> *,boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,boost::detail::shared_count &)")
}

// 0xf63944 — j___ZN5boost6detail20sp_pointer_constructIN3RBX10Reflection5TupleES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Reflection::Tuple,RBX::Reflection::Tuple>(boost::shared_ptr<RBX::Reflection::Tuple> *,RBX::Reflection::Tuple *,boost::detail::shared_count &)")]
pub fn stub_f63944() -> ! {
    todo!("0xf63944 void boost::detail::sp_pointer_construct<RBX::Reflection::Tuple,RBX::Reflection::Tuple>(boost::shared_ptr<RBX::Reflection::Tuple> *,RBX::Reflection::Tuple *,boost::detail::shared_count &)")
}

// 0xf63934 — j___ZN5boost6detail12shared_countD1Ev
// type: void __fastcall(boost::detail::shared_count *__hidden this)
#[doc(alias = "boost::detail::shared_count::~shared_count()")]
pub fn stub_f63934() -> ! {
    todo!("0xf63934 boost::detail::shared_count::~shared_count()")
}

// 0xf63924 — j___ZN5boost4poolINS_33default_user_allocator_new_deleteEE18malloc_need_resizeEv
// type: char *__fastcall(char ***)
#[doc(alias = "boost::pool<boost::default_user_allocator_new_delete>::malloc_need_resize(void)")]
pub fn stub_f63924() -> ! {
    todo!("0xf63924 boost::pool<boost::default_user_allocator_new_delete>::malloc_need_resize(void)")
}

// 0xf63914 — j___ZN5boost4poolINS_33default_user_allocator_new_deleteEE14ordered_mallocEm
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "boost::pool<boost::default_user_allocator_new_delete>::ordered_malloc(unsigned long)")]
pub fn stub_f63914() -> ! {
    todo!("0xf63914 boost::pool<boost::default_user_allocator_new_delete>::ordered_malloc(unsigned long)")
}

// 0xf63904 — j___ZN5boost4bindIvNS_8weak_ptrIN3RBX9DataModelEEES4_EENS_3_bi6bind_tIT_PFS7_T0_ENS5_9list_av_1IT1_E4typeEEESA_SC_
// type: void __fastcall(struct _Unwind_Exception **, int, int *, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>),boost::_bi::list_av_1<boost::weak_ptr<RBX::DataModel>>::type> boost::bind<void,boost::weak_ptr<RBX::DataModel>,boost::weak_ptr<RBX::DataModel>>(void (*)(boost::weak_ptr<RBX::DataModel>),boost::weak_ptr<RBX::DataModel>)")]
pub fn stub_f63904() -> ! {
    todo!("0xf63904 boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>),boost::_bi::list_av_1<boost::weak_ptr<RBX::DataModel>>::type> boost::bind<void,boost::weak_ptr<RBX::DataModel>,boost::weak_ptr<RBX::DataModel>>(void (*)(boost::weak_ptr<RBX::DataModel>),boost::weak_ptr<RBX::DataModel>)")
}

// 0xf638b4 — j___ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKSsRKbNS_10shared_ptrIS3_EENS_3argILi1EEENSA_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISF_T0_T1_T2_EENSD_9list_av_3IT3_T4_T5_E4typeEEEMSI_FSF_SJ_SK_ESN_SO_SP_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,bool const&>,boost::_bi::list_av_3<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,std::string const&,bool const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(std::string const&,bool const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")]
pub fn stub_f638b4() -> ! {
    todo!("0xf638b4 boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,bool const&>,boost::_bi::list_av_3<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,std::string const&,bool const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(std::string const&,bool const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")
}

// 0xf637f4 — j___ZN5boost2io6detail24upper_bound_from_fstringISsSt5ctypeIcEEEiRKT_NS5_10value_typeERKT0_h
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "int boost::io::detail::upper_bound_from_fstring<std::string,std::ctype<char>>(std::string const&,std::string::value_type,std::ctype<char> const&,unsigned char)")]
pub fn stub_f637f4() -> ! {
    todo!("0xf637f4 int boost::io::detail::upper_bound_from_fstring<std::string,std::ctype<char>>(std::string const&,std::string::value_type,std::ctype<char> const&,unsigned char)")
}

// 0xf637e4 — j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt9bad_allocEEEC1ERKS5_NS5_9clone_tagE
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::bad_alloc>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::bad_alloc>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::bad_alloc>>::clone_tag)")]
pub fn stub_f637e4() -> ! {
    todo!("0xf637e4 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::bad_alloc>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::bad_alloc>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::bad_alloc>>::clone_tag)")
}

// 0xf637d4 — j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt9bad_allocEEEC1ERKS4_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::bad_alloc>>::clone_impl(boost::exception_detail::error_info_injector<std::bad_alloc> const&)")]
pub fn stub_f637d4() -> ! {
    todo!("0xf637d4 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::bad_alloc>>::clone_impl(boost::exception_detail::error_info_injector<std::bad_alloc> const&)")
}

// 0xf637c4 — j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEEC1ERKS6_NS6_9clone_tagE
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::clone_tag)")]
pub fn stub_f637c4() -> ! {
    todo!("0xf637c4 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::clone_tag)")
}
