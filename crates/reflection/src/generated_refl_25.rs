//! Auto-generated refl 25 — 100 stubs EA-sorted asc 0x237798..0x23da40 (global gap filler, RBX::Reflection exhausted 16171/16171, crate-local gaps 57662->57562 , rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo)
//! Source: ida/export.json (85545 funcs) global EA asc not yet in crates/reflection/src — next 100 uncovered
//! Format: // 0xADDR — mangled + #[doc(alias = "RBX::...")] + todo!("0xADDR") using rbx_core::SharedPtr not boost

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x237798 — __ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<void const*,std::pair<void const* const,boost::detail::tss_data_node>,std::_Select1st<std::pair<void const* const,boost::detail::tss_data_node>>,std::less<void const*>,std::allocator<std::pair<void const* const,boost::detail::tss_data_node>>>::_M_erase(std::_Rb_tree_node<std::pair<void const* const,boost::detail::tss_data_node>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")]
pub fn stub_0x237798() -> ! {
    todo!("0x237798 std::_Rb_tree<void const*,std::pair<void const* const,boost::detail::tss_data_node>,std::_Select1st<std::pair<void const* const,boost::detail::tss_data_node>>,std::less<void const*>,std::allocator<std::pair<void const* const,boost::detail::tss_data_node>>>::_M_erase(std::_Rb_tree_node<std::pair<void const* const,boost::detail::tss_data_node>> *)")
}

// 0x237848 — __ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE16_M_insert_uniqueERKS7_
// type: int __fastcall(int, _DWORD *, int *)
#[doc(alias = "std::_Rb_tree<void const*,std::pair<void const* const,boost::detail::tss_data_node>,std::_Select1st<std::pair<void const* const,boost::detail::tss_data_node>>,std::less<void const*>,std::allocator<std::pair<void const* const,boost::detail::tss_data_node>>>::_M_insert_unique(std::pair<void const* const,boost::detail::tss_data_node> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE16_M_insert_uniqueERKS7_")]
pub fn stub_0x237848() -> ! {
    todo!("0x237848 std::_Rb_tree<void const*,std::pair<void const* const,boost::detail::tss_data_node>,std::_Select1st<std::pair<void const* const,boost::detail::tss_data_node>>,std::less<void const*>,std::allocator<std::pair<void const* const,boost::detail::tss_data_node>>>::_M_insert_unique(std::pair<void const* const,boost::detail::tss_data_node> const&)")
}

// 0x2378fc — __ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE14_M_create_nodeERKS7_
// type: _DWORD *__fastcall(int, int *)
#[doc(alias = "std::_Rb_tree<void const*,std::pair<void const* const,boost::detail::tss_data_node>,std::_Select1st<std::pair<void const* const,boost::detail::tss_data_node>>,std::less<void const*>,std::allocator<std::pair<void const* const,boost::detail::tss_data_node>>>::_M_create_node(std::pair<void const* const,boost::detail::tss_data_node> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE14_M_create_nodeERKS7_")]
pub fn stub_0x2378fc() -> ! {
    todo!("0x2378fc std::_Rb_tree<void const*,std::pair<void const* const,boost::detail::tss_data_node>,std::_Select1st<std::pair<void const* const,boost::detail::tss_data_node>>,std::less<void const*>,std::allocator<std::pair<void const* const,boost::detail::tss_data_node>>>::_M_create_node(std::pair<void const* const,boost::detail::tss_data_node> const&)")
}

// 0x2379ec — __ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_S2_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int)
#[doc(alias = "void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data_base>(boost::shared_ptr<boost::detail::thread_data_base> const*,boost::detail::thread_data_base *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_S2_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0x2379ec() {
    // IDA 0x2379ec: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0x237b40 — __ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data_base>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEED1Ev")]
pub fn stub_0x237b40() {
    // IDA 0x237b40: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x237b44 — __ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data_base>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEED0Ev")]
pub fn stub_0x237b44() {
    // IDA 0x237b44: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x237b50 — __ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data_base>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEE7disposeEv")]
pub fn stub_0x237b50() -> ! {
    todo!("0x237b50 boost::detail::sp_counted_impl_p<boost::detail::thread_data_base>::dispose(void)")
}

// 0x237b64 — __ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data_base>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEE11get_deleterERKSt9type_info")]
pub fn stub_0x237b64() -> ! {
    todo!("0x237b64 boost::detail::sp_counted_impl_p<boost::detail::thread_data_base>::get_deleter(std::type_info const&)")
}

// 0x237b68 — __ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data_base>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEE19get_untyped_deleterEv")]
pub fn stub_0x237b68() -> ! {
    todo!("0x237b68 boost::detail::sp_counted_impl_p<boost::detail::thread_data_base>::get_untyped_deleter(void)")
}

// 0x237b6c — __ZN5boost6detail18future_object_base22mark_finished_internalERNS_11unique_lockINS_5mutexEEE
// type: void __fastcall(int)
#[doc(alias = "boost::detail::future_object_base::mark_finished_internal(boost::unique_lock<boost::mutex> &)")]
#[doc(alias = "__ZN5boost6detail18future_object_base22mark_finished_internalERNS_11unique_lockINS_5mutexEEE")]
pub fn stub_0x237b6c() -> ! {
    todo!("0x237b6c boost::detail::future_object_base::mark_finished_internal(boost::unique_lock<boost::mutex> &)")
}

// 0x237c98 — __GLOBAL__I_a_38
#[doc(alias = "global constructor keyed to_a_38")]
#[doc(alias = "__GLOBAL__I_a_38")]
pub fn stub_0x237c98() -> ! {
    todo!("0x237c98 __GLOBAL__I_a_38")
}

// 0x237d60 — __ZN5boost10filesystem6detail14symlink_statusERKNS0_4pathEPNS_6system10error_codeE
// type: void __fastcall(int *, const char **, int *)
#[doc(alias = "boost::filesystem::detail::symlink_status(boost::filesystem::path const&,boost::system::error_code *)")]
#[doc(alias = "__ZN5boost10filesystem6detail14symlink_statusERKNS0_4pathEPNS_6system10error_codeE")]
pub fn stub_0x237d60() -> ! {
    todo!("0x237d60 boost::filesystem::detail::symlink_status(boost::filesystem::path const&,boost::system::error_code *)")
}

// 0x237fa4 — __ZN5boost10filesystem6detail12current_pathEPNS_6system10error_codeE
// type: void __fastcall(std::string *, _DWORD *)
#[doc(alias = "boost::filesystem::detail::current_path(boost::system::error_code *)")]
#[doc(alias = "__ZN5boost10filesystem6detail12current_pathEPNS_6system10error_codeE")]
pub fn stub_0x237fa4() -> ! {
    todo!("0x237fa4 boost::filesystem::detail::current_path(boost::system::error_code *)")
}

// 0x238258 — __ZN12_GLOBAL__N_15errorEbRKN5boost10filesystem4pathEPNS0_6system10error_codeERKSs
// type: int __fastcall(int, void *, int, int)
#[doc(alias = "anonymous namespace::error(bool,boost::filesystem::path const&,boost::system::error_code *,std::string const&)")]
#[doc(alias = "__ZN12_GLOBAL__N_15errorEbRKN5boost10filesystem4pathEPNS0_6system10error_codeERKSs")]
pub fn stub_0x238258() -> ! {
    todo!("0x238258 anonymous namespace::error(bool,boost::filesystem::path const&,boost::system::error_code *,std::string const&)")
}

// 0x23837c — __ZN5boost10filesystem6detail12initial_pathEPNS_6system10error_codeE
// type: void __fastcall(std::string *, _DWORD *)
#[doc(alias = "boost::filesystem::detail::initial_path(boost::system::error_code *)")]
#[doc(alias = "__ZN5boost10filesystem6detail12initial_pathEPNS_6system10error_codeE")]
pub fn stub_0x23837c() -> ! {
    todo!("0x23837c boost::filesystem::detail::initial_path(boost::system::error_code *)")
}

// 0x23852c — __ZN5boost10filesystem6detail8is_emptyERKNS0_4pathEPNS_6system10error_codeE
// type: bool __fastcall(const char **, int)
#[doc(alias = "boost::filesystem::detail::is_empty(boost::filesystem::path const&,boost::system::error_code *)")]
#[doc(alias = "__ZN5boost10filesystem6detail8is_emptyERKNS0_4pathEPNS_6system10error_codeE")]
pub fn stub_0x23852c() -> ! {
    todo!("0x23852c boost::filesystem::detail::is_empty(boost::filesystem::path const&,boost::system::error_code *)")
}

// 0x2386d4 — __ZN5boost10filesystem6detail6removeERKNS0_4pathEPNS_6system10error_codeE
// type: int __fastcall(const char **, int *)
#[doc(alias = "boost::filesystem::detail::remove(boost::filesystem::path const&,boost::system::error_code *)")]
#[doc(alias = "__ZN5boost10filesystem6detail6removeERKNS0_4pathEPNS_6system10error_codeE")]
pub fn stub_0x2386d4() -> ! {
    todo!("0x2386d4 boost::filesystem::detail::remove(boost::filesystem::path const&,boost::system::error_code *)")
}

// 0x2388a8 — __ZN12_GLOBAL__N_124remove_file_or_directoryERKN5boost10filesystem4pathENS1_9file_typeEPNS0_6system10error_codeE
// type: bool __fastcall(const char **, int, _DWORD *)
#[doc(alias = "anonymous namespace::remove_file_or_directory(boost::filesystem::path const&,boost::filesystem::file_type,boost::system::error_code *)")]
#[doc(alias = "__ZN12_GLOBAL__N_124remove_file_or_directoryERKN5boost10filesystem4pathENS1_9file_typeEPNS0_6system10error_codeE")]
pub fn stub_0x2388a8() -> ! {
    todo!("0x2388a8 anonymous namespace::remove_file_or_directory(boost::filesystem::path const&,boost::filesystem::file_type,boost::system::error_code *)")
}

// 0x238adc — __ZN5boost10filesystem6detail6statusERKNS0_4pathEPNS_6system10error_codeE
// type: void __fastcall(int *, const char **, int *)
#[doc(alias = "boost::filesystem::detail::status(boost::filesystem::path const&,boost::system::error_code *)")]
#[doc(alias = "__ZN5boost10filesystem6detail6statusERKNS0_4pathEPNS_6system10error_codeE")]
pub fn stub_0x238adc() -> ! {
    todo!("0x238adc boost::filesystem::detail::status(boost::filesystem::path const&,boost::system::error_code *)")
}

// 0x238d18 — __ZN5boost10filesystem6detail15system_completeERKNS0_4pathEPNS_6system10error_codeE
// type: void __fastcall(std::string *, const std::string *)
#[doc(alias = "boost::filesystem::detail::system_complete(boost::filesystem::path const&,boost::system::error_code *)")]
#[doc(alias = "__ZN5boost10filesystem6detail15system_completeERKNS0_4pathEPNS_6system10error_codeE")]
pub fn stub_0x238d18() -> ! {
    todo!("0x238d18 boost::filesystem::detail::system_complete(boost::filesystem::path const&,boost::system::error_code *)")
}

// 0x238f14 — __ZNK5boost10filesystem15directory_entry12m_get_statusEPNS_6system10error_codeE
// type: __int64 __fastcall(_QWORD *, int, int *)
#[doc(alias = "boost::filesystem::directory_entry::m_get_status(boost::system::error_code *)const")]
#[doc(alias = "__ZNK5boost10filesystem15directory_entry12m_get_statusEPNS_6system10error_codeE")]
pub fn stub_0x238f14() -> ! {
    todo!("0x238f14 boost::filesystem::directory_entry::m_get_status(boost::system::error_code *)const")
}

// 0x238f80 — __ZN5boost10filesystem6detail13dir_itr_closeERPvS3_
// type: int __fastcall(boost::filesystem::detail *this, void **, void **)
#[doc(alias = "boost::filesystem::detail::dir_itr_close(void *&,void *&)")]
#[doc(alias = "__ZN5boost10filesystem6detail13dir_itr_closeERPvS3_")]
pub fn stub_0x238f80() -> ! {
    todo!("0x238f80 boost::filesystem::detail::dir_itr_close(void *&,void *&)")
}

// 0x238fd4 — __ZN5boost10filesystem6detail28directory_iterator_constructERNS0_18directory_iteratorERKNS0_4pathEPNS_6system10error_codeE
// type: void __fastcall(std::string **, const char **, std::string **)
#[doc(alias = "boost::filesystem::detail::directory_iterator_construct(boost::filesystem::directory_iterator &,boost::filesystem::path const&,boost::system::error_code *)")]
#[doc(alias = "__ZN5boost10filesystem6detail28directory_iterator_constructERNS0_18directory_iteratorERKNS0_4pathEPNS_6system10error_codeE")]
pub fn stub_0x238fd4() -> ! {
    todo!("0x238fd4 boost::filesystem::detail::directory_iterator_construct(boost::filesystem::directory_iterator &,boost::filesystem::path const&,boost::system::error_code *)")
}

// 0x239668 — __ZN5boost10filesystem6detail28directory_iterator_incrementERNS0_18directory_iteratorEPNS_6system10error_codeE
// type: void __fastcall(int *, dirent **)
#[doc(alias = "boost::filesystem::detail::directory_iterator_increment(boost::filesystem::directory_iterator &,boost::system::error_code *)")]
#[doc(alias = "__ZN5boost10filesystem6detail28directory_iterator_incrementERNS0_18directory_iteratorEPNS_6system10error_codeE")]
pub fn stub_0x239668() -> ! {
    todo!("0x239668 boost::filesystem::detail::directory_iterator_increment(boost::filesystem::directory_iterator &,boost::system::error_code *)")
}

// 0x239b34 — __ZN5boost10filesystem18directory_iteratorD1Ev
// type: void __fastcall(boost::filesystem::directory_iterator *__hidden this)
#[doc(alias = "boost::filesystem::directory_iterator::~directory_iterator()")]
#[doc(alias = "__ZN5boost10filesystem18directory_iteratorD1Ev")]
pub fn stub_0x239b34() {
    // IDA 0x239b34: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x239bc8 — __ZN5boost10filesystem16filesystem_errorD1Ev
// type: void __fastcall(std::runtime_error *this)
#[doc(alias = "boost::filesystem::filesystem_error::~filesystem_error()")]
#[doc(alias = "__ZN5boost10filesystem16filesystem_errorD1Ev")]
pub fn stub_0x239bc8() {
    // IDA 0x239bc8: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x239cc8 — __ZN5boost10filesystem16filesystem_errorC2ERKSsNS_6system10error_codeE
// type: std::runtime_error *__fastcall(std::runtime_error *, const std::string *, std::runtime_error_vtbl *, const char *, boost::detail::sp_counted_base *, std::runtime_error *, int, int, void *, int)
#[doc(alias = "boost::filesystem::filesystem_error::filesystem_error(std::string const&,boost::system::error_code)")]
#[doc(alias = "__ZN5boost10filesystem16filesystem_errorC2ERKSsNS_6system10error_codeE")]
pub fn stub_0x239cc8() -> ! {
    todo!("0x239cc8 boost::filesystem::filesystem_error::filesystem_error(std::string const&,boost::system::error_code)")
}

// 0x239e90 — __ZN5boost10filesystem16filesystem_errorD0Ev
// type: void __fastcall(std::runtime_error *this)
#[doc(alias = "boost::filesystem::filesystem_error::~filesystem_error()")]
#[doc(alias = "__ZN5boost10filesystem16filesystem_errorD0Ev")]
pub fn stub_0x239e90() {
    // IDA 0x239e90: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x239f94 — __ZNK5boost10filesystem16filesystem_error4whatEv
// type: int __fastcall(boost::filesystem::filesystem_error *this)
#[doc(alias = "boost::filesystem::filesystem_error::what(void)const")]
#[doc(alias = "__ZNK5boost10filesystem16filesystem_error4whatEv")]
pub fn stub_0x239f94() -> ! {
    todo!("0x239f94 boost::filesystem::filesystem_error::what(void)const")
}

// 0x23a11c — __ZN5boost6detail20sp_pointer_constructINS_10filesystem16filesystem_error5m_impES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, boost::detail::sp_counted_base **, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<boost::filesystem::filesystem_error::m_imp,boost::filesystem::filesystem_error::m_imp>(boost::shared_ptr<boost::filesystem::filesystem_error::m_imp> *,boost::filesystem::filesystem_error::m_imp *,boost::detail::shared_count &)")]
#[doc(alias = "__ZN5boost6detail20sp_pointer_constructINS_10filesystem16filesystem_error5m_impES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")]
pub fn stub_0x23a11c() -> ! {
    todo!("0x23a11c void boost::detail::sp_pointer_construct<boost::filesystem::filesystem_error::m_imp,boost::filesystem::filesystem_error::m_imp>(boost::shared_ptr<boost::filesystem::filesystem_error::m_imp> *,boost::filesystem::filesystem_error::m_imp *,boost::detail::shared_count &)")
}

// 0x23a2bc — __ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::filesystem::filesystem_error::m_imp>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEED1Ev")]
pub fn stub_0x23a2bc() {
    // IDA 0x23a2bc: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x23a2c0 — __ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::filesystem::filesystem_error::m_imp>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEED0Ev")]
pub fn stub_0x23a2c0() {
    // IDA 0x23a2c0: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x23a2cc — __ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEE7disposeEv
// type: void __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::filesystem::filesystem_error::m_imp>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEE7disposeEv")]
pub fn stub_0x23a2cc() -> ! {
    todo!("0x23a2cc boost::detail::sp_counted_impl_p<boost::filesystem::filesystem_error::m_imp>::dispose(void)")
}

// 0x23a38c — __ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::filesystem::filesystem_error::m_imp>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEE11get_deleterERKSt9type_info")]
pub fn stub_0x23a38c() -> ! {
    todo!("0x23a38c boost::detail::sp_counted_impl_p<boost::filesystem::filesystem_error::m_imp>::get_deleter(std::type_info const&)")
}

// 0x23a390 — __ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::filesystem::filesystem_error::m_imp>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEE19get_untyped_deleterEv")]
pub fn stub_0x23a390() -> ! {
    todo!("0x23a390 boost::detail::sp_counted_impl_p<boost::filesystem::filesystem_error::m_imp>::get_untyped_deleter(void)")
}

// 0x23a394 — __ZN5boost10filesystem16filesystem_errorC2ERKSsRKNS0_4pathENS_6system10error_codeE
// type: std::runtime_error *__fastcall(std::runtime_error *, const std::string *, const std::string *, std::runtime_error_vtbl *, boost::detail::sp_counted_base *, std::runtime_error *, int, int, void *, int)
#[doc(alias = "boost::filesystem::filesystem_error::filesystem_error(std::string const&,boost::filesystem::path const&,boost::system::error_code)")]
#[doc(alias = "__ZN5boost10filesystem16filesystem_errorC2ERKSsRKNS0_4pathENS_6system10error_codeE")]
pub fn stub_0x23a394() -> ! {
    todo!("0x23a394 boost::filesystem::filesystem_error::filesystem_error(std::string const&,boost::filesystem::path const&,boost::system::error_code)")
}

// 0x23a570 — __GLOBAL__I_a_39
// type: int *()
#[doc(alias = "global constructor keyed to_a_39")]
#[doc(alias = "__GLOBAL__I_a_39")]
pub fn stub_0x23a570() -> ! {
    todo!("0x23a570 __GLOBAL__I_a_39")
}

// 0x23a630 — __ZN5boost10filesystem4pathdVERKS1_
// type: boost::filesystem::path *__fastcall(boost::filesystem::path *, const std::string *)
#[doc(alias = "boost::filesystem::path::operator/=(boost::filesystem::path const&)")]
#[doc(alias = "__ZN5boost10filesystem4pathdVERKS1_")]
pub fn stub_0x23a630() -> ! {
    todo!("0x23a630 boost::filesystem::path::operator/=(boost::filesystem::path const&)")
}

// 0x23a7b8 — __ZN5boost10filesystem4path28m_append_separator_if_neededEv
// type: int __fastcall(boost::filesystem::path *this)
#[doc(alias = "boost::filesystem::path::m_append_separator_if_needed(void)")]
#[doc(alias = "__ZN5boost10filesystem4path28m_append_separator_if_neededEv")]
pub fn stub_0x23a7b8() -> ! {
    todo!("0x23a7b8 boost::filesystem::path::m_append_separator_if_needed(void)")
}

// 0x23a830 — __ZN5boost10filesystem4pathdVEPKc
// type: boost::filesystem::path *__fastcall(boost::filesystem::path *, const char *, int, int, int, char, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "boost::filesystem::path::operator/=(char const*)")]
#[doc(alias = "__ZN5boost10filesystem4pathdVEPKc")]
pub fn stub_0x23a830() -> ! {
    todo!("0x23a830 boost::filesystem::path::operator/=(char const*)")
}

// 0x23a9d4 — __ZN5boost10filesystem4path27m_erase_redundant_separatorEm
// type: std::string *__fastcall(std::string *this, unsigned int)
#[doc(alias = "boost::filesystem::path::m_erase_redundant_separator(unsigned long)")]
#[doc(alias = "__ZN5boost10filesystem4path27m_erase_redundant_separatorEm")]
pub fn stub_0x23a9d4() -> ! {
    todo!("0x23a9d4 boost::filesystem::path::m_erase_redundant_separator(unsigned long)")
}

// 0x23aa2c — __ZN5boost10filesystem4path15remove_filenameEv
// type: boost::filesystem::path *__fastcall(boost::filesystem::path *this)
#[doc(alias = "boost::filesystem::path::remove_filename(void)")]
#[doc(alias = "__ZN5boost10filesystem4path15remove_filenameEv")]
pub fn stub_0x23aa2c() -> ! {
    todo!("0x23aa2c boost::filesystem::path::remove_filename(void)")
}

// 0x23aa60 — __ZNK5boost10filesystem4path17m_parent_path_endEv
// type: int __fastcall(boost::filesystem::path *this)
#[doc(alias = "boost::filesystem::path::m_parent_path_end(void)const")]
#[doc(alias = "__ZNK5boost10filesystem4path17m_parent_path_endEv")]
pub fn stub_0x23aa60() -> ! {
    todo!("0x23aa60 boost::filesystem::path::m_parent_path_end(void)const")
}

// 0x23ab64 — __ZNK5boost10filesystem4path14root_directoryEv
// type: char *__fastcall(boost::filesystem::path *this, std::string *)
#[doc(alias = "boost::filesystem::path::root_directory(void)const")]
#[doc(alias = "__ZNK5boost10filesystem4path14root_directoryEv")]
pub fn stub_0x23ab64() -> ! {
    todo!("0x23ab64 boost::filesystem::path::root_directory(void)const")
}

// 0x23abe8 — __ZNK5boost10filesystem4path11parent_pathEv
// type: char *__fastcall(boost::filesystem::path *this, boost::filesystem::path *)
#[doc(alias = "boost::filesystem::path::parent_path(void)const")]
#[doc(alias = "__ZNK5boost10filesystem4path11parent_pathEv")]
pub fn stub_0x23abe8() -> ! {
    todo!("0x23abe8 boost::filesystem::path::parent_path(void)const")
}

// 0x23ac1c — __ZN5boost10filesystem4path7codecvtEv
// type: int __fastcall(boost::filesystem::path *this)
#[doc(alias = "boost::filesystem::path::codecvt(void)")]
#[doc(alias = "__ZN5boost10filesystem4path7codecvtEv")]
pub fn stub_0x23ac1c() -> ! {
    todo!("0x23ac1c boost::filesystem::path::codecvt(void)")
}

// 0x23ac2c — __ZN5boost10filesystem4pathD1Ev
// type: void __fastcall(boost::filesystem::path *__hidden this)
#[doc(alias = "boost::filesystem::path::~path()")]
#[doc(alias = "__ZN5boost10filesystem4pathD1Ev")]
pub fn stub_0x23ac2c() {
    // IDA 0x23ac2c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x23ac78 — __ZNSt6localeC2IN5boost10filesystem6detail18utf8_codecvt_facetEEERKS_PT_
// type: int __fastcall(int, const _Impl **, int, int, void *, int)
#[doc(alias = "std::locale::locale<boost::filesystem::detail::utf8_codecvt_facet>(std::locale const&,boost::filesystem::detail::utf8_codecvt_facet *)")]
#[doc(alias = "__ZNSt6localeC2IN5boost10filesystem6detail18utf8_codecvt_facetEEERKS_PT_")]
pub fn stub_0x23ac78() -> ! {
    todo!("0x23ac78 std::locale::locale<boost::filesystem::detail::utf8_codecvt_facet>(std::locale const&,boost::filesystem::detail::utf8_codecvt_facet *)")
}

// 0x23adc4 — __ZN5boost10filesystem4pathC2IPKcEET_S5_
// type: std::string *__fastcall(std::string *, _BYTE *, _BYTE *)
#[doc(alias = "boost::filesystem::path::path<char const*>(char const*,char const*)")]
#[doc(alias = "__ZN5boost10filesystem4pathC2IPKcEET_S5_")]
pub fn stub_0x23adc4() -> ! {
    todo!("0x23adc4 boost::filesystem::path::path<char const*>(char const*,char const*)")
}

// 0x23af94 — __ZNK5boost10filesystem6detail18utf8_codecvt_facet10do_unshiftER11__mbstate_tPcS5_RS5_
// type: int __fastcall(boost::filesystem::detail::utf8_codecvt_facet *this, __mbstate_t *, char *, char *, char **)
#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::do_unshift(__mbstate_t &,char *,char *,char *&)const")]
#[doc(alias = "__ZNK5boost10filesystem6detail18utf8_codecvt_facet10do_unshiftER11__mbstate_tPcS5_RS5_")]
pub fn stub_0x23af94() -> ! {
    todo!("0x23af94 boost::filesystem::detail::utf8_codecvt_facet::do_unshift(__mbstate_t &,char *,char *,char *&)const")
}

// 0x23af9c — __ZNK5boost10filesystem6detail18utf8_codecvt_facet11do_encodingEv
// type: int __fastcall(boost::filesystem::detail::utf8_codecvt_facet *this)
#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::do_encoding(void)const")]
#[doc(alias = "__ZNK5boost10filesystem6detail18utf8_codecvt_facet11do_encodingEv")]
pub fn stub_0x23af9c() -> ! {
    todo!("0x23af9c boost::filesystem::detail::utf8_codecvt_facet::do_encoding(void)const")
}

// 0x23afa0 — __ZNK5boost10filesystem6detail18utf8_codecvt_facet16do_always_noconvEv
// type: int __fastcall(boost::filesystem::detail::utf8_codecvt_facet *this)
#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::do_always_noconv(void)const")]
#[doc(alias = "__ZNK5boost10filesystem6detail18utf8_codecvt_facet16do_always_noconvEv")]
pub fn stub_0x23afa0() -> ! {
    todo!("0x23afa0 boost::filesystem::detail::utf8_codecvt_facet::do_always_noconv(void)const")
}

// 0x23afa4 — __ZNK5boost10filesystem6detail18utf8_codecvt_facet13do_max_lengthEv
// type: int __fastcall(boost::filesystem::detail::utf8_codecvt_facet *this)
#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::do_max_length(void)const")]
#[doc(alias = "__ZNK5boost10filesystem6detail18utf8_codecvt_facet13do_max_lengthEv")]
pub fn stub_0x23afa4() -> ! {
    todo!("0x23afa4 boost::filesystem::detail::utf8_codecvt_facet::do_max_length(void)const")
}

// 0x23afa8 — __GLOBAL__I_a_40
// type: void __fastcall(int, int, int, int, char, void *, int, int, int, int)
#[doc(alias = "global constructor keyed to_a_40")]
#[doc(alias = "__GLOBAL__I_a_40")]
pub fn stub_0x23afa8() -> ! {
    todo!("0x23afa8 __GLOBAL__I_a_40")
}

// 0x23b14c — __ZNK5boost10filesystem6detail18utf8_codecvt_facet5do_inER11__mbstate_tPKcS6_RS6_PwS8_RS8_
// type: int __fastcall(int, int, char *, char *, char **, int *, int *, int **)
#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::do_in(__mbstate_t &,char const*,char const*,char const*&,wchar_t *,wchar_t *,wchar_t *&)const")]
#[doc(alias = "__ZNK5boost10filesystem6detail18utf8_codecvt_facet5do_inER11__mbstate_tPKcS6_RS6_PwS8_RS8_")]
pub fn stub_0x23b14c() -> ! {
    todo!("0x23b14c boost::filesystem::detail::utf8_codecvt_facet::do_in(__mbstate_t &,char const*,char const*,char const*&,wchar_t *,wchar_t *,wchar_t *&)const")
}

// 0x23b2d0 — __ZNK5boost10filesystem6detail18utf8_codecvt_facet6do_outER11__mbstate_tPKwS6_RS6_PcS8_RS8_
// type: bool __fastcall(int, int, _DWORD *, _DWORD *, _DWORD *, _BYTE *, _BYTE *, _DWORD *)
#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::do_out(__mbstate_t &,wchar_t const*,wchar_t const*,wchar_t const*&,char *,char *,char *&)const")]
#[doc(alias = "__ZNK5boost10filesystem6detail18utf8_codecvt_facet6do_outER11__mbstate_tPKwS6_RS6_PcS8_RS8_")]
pub fn stub_0x23b2d0() -> ! {
    todo!("0x23b2d0 boost::filesystem::detail::utf8_codecvt_facet::do_out(__mbstate_t &,wchar_t const*,wchar_t const*,wchar_t const*&,char *,char *,char *&)const")
}

// 0x23b43c — __ZNK5boost10filesystem6detail18utf8_codecvt_facet9do_lengthERK11__mbstate_tPKcS7_m
// type: int __fastcall(boost::filesystem::detail::utf8_codecvt_facet *this, const __mbstate_t *, const char *, const char *, unsigned int)
#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::do_length(__mbstate_t const&,char const*,char const*,unsigned long)const")]
#[doc(alias = "__ZNK5boost10filesystem6detail18utf8_codecvt_facet9do_lengthERK11__mbstate_tPKcS7_m")]
pub fn stub_0x23b43c() -> ! {
    todo!("0x23b43c boost::filesystem::detail::utf8_codecvt_facet::do_length(__mbstate_t const&,char const*,char const*,unsigned long)const")
}

// 0x23b4ac — __ZN5boost10filesystem6detail18utf8_codecvt_facetD1Ev
// type: void __fastcall(boost::filesystem::detail::utf8_codecvt_facet *__hidden this)
#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::~utf8_codecvt_facet()")]
#[doc(alias = "__ZN5boost10filesystem6detail18utf8_codecvt_facetD1Ev")]
pub fn stub_0x23b4ac() {
    // IDA 0x23b4ac: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x23b4b8 — __ZN5boost10filesystem6detail18utf8_codecvt_facetD0Ev
// type: void __fastcall(boost::filesystem::detail::utf8_codecvt_facet *__hidden this)
#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::~utf8_codecvt_facet()")]
#[doc(alias = "__ZN5boost10filesystem6detail18utf8_codecvt_facetD0Ev")]
pub fn stub_0x23b4b8() {
    // IDA 0x23b4b8: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x23b4cc — __ZN5boost6system16generic_categoryEv
// type: int *__fastcall()
#[doc(alias = "boost::system::generic_category(void)")]
#[doc(alias = "__ZN5boost6system16generic_categoryEv")]
pub fn stub_0x23b4cc() -> ! {
    todo!("0x23b4cc boost::system::generic_category(void)")
}

// 0x23b508 — __ZN5boost6system15system_categoryEv
// type: int *__fastcall()
#[doc(alias = "boost::system::system_category(void)")]
#[doc(alias = "__ZN5boost6system15system_categoryEv")]
pub fn stub_0x23b508() -> ! {
    todo!("0x23b508 boost::system::system_category(void)")
}

// 0x23b544 — __ZN12_GLOBAL__N_121system_error_categoryD1Ev
// type: void __fastcall(_anonymous_namespace_::system_error_category *__hidden this)
#[doc(alias = "anonymous namespace::system_error_category::~system_error_category()")]
#[doc(alias = "__ZN12_GLOBAL__N_121system_error_categoryD1Ev")]
pub fn stub_0x23b544() {
    // IDA 0x23b544: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x23b548 — __ZN12_GLOBAL__N_122generic_error_categoryD1Ev
// type: void __fastcall(_anonymous_namespace_::generic_error_category *__hidden this)
#[doc(alias = "anonymous namespace::generic_error_category::~generic_error_category()")]
#[doc(alias = "__ZN12_GLOBAL__N_122generic_error_categoryD1Ev")]
pub fn stub_0x23b548() {
    // IDA 0x23b548: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x23b54c — __ZN12_GLOBAL__N_122generic_error_categoryD0Ev
// type: void __fastcall(_anonymous_namespace_::generic_error_category *__hidden this)
#[doc(alias = "anonymous namespace::generic_error_category::~generic_error_category()")]
#[doc(alias = "__ZN12_GLOBAL__N_122generic_error_categoryD0Ev")]
pub fn stub_0x23b54c() {
    // IDA 0x23b54c: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x23b558 — __ZNK12_GLOBAL__N_122generic_error_category4nameEv
// type: const char *__fastcall(_anonymous_namespace_::generic_error_category *this)
#[doc(alias = "anonymous namespace::generic_error_category::name(void)const")]
#[doc(alias = "__ZNK12_GLOBAL__N_122generic_error_category4nameEv")]
pub fn stub_0x23b558() -> ! {
    todo!("0x23b558 anonymous namespace::generic_error_category::name(void)const")
}

// 0x23b564 — __ZNK12_GLOBAL__N_122generic_error_category7messageEi
// type: int __fastcall(_anonymous_namespace_::generic_error_category *this, int, int)
#[doc(alias = "anonymous namespace::generic_error_category::message(int)const")]
#[doc(alias = "__ZNK12_GLOBAL__N_122generic_error_category7messageEi")]
pub fn stub_0x23b564() -> ! {
    todo!("0x23b564 anonymous namespace::generic_error_category::message(int)const")
}

// 0x23b7cc — __ZN12_GLOBAL__N_121system_error_categoryD0Ev
// type: void __fastcall(_anonymous_namespace_::system_error_category *__hidden this)
#[doc(alias = "anonymous namespace::system_error_category::~system_error_category()")]
#[doc(alias = "__ZN12_GLOBAL__N_121system_error_categoryD0Ev")]
pub fn stub_0x23b7cc() {
    // IDA 0x23b7cc: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x23b7d8 — __ZNK12_GLOBAL__N_121system_error_category4nameEv
// type: const char *__fastcall(_anonymous_namespace_::system_error_category *this)
#[doc(alias = "anonymous namespace::system_error_category::name(void)const")]
#[doc(alias = "__ZNK12_GLOBAL__N_121system_error_category4nameEv")]
pub fn stub_0x23b7d8() -> ! {
    todo!("0x23b7d8 anonymous namespace::system_error_category::name(void)const")
}

// 0x23b7e4 — __ZNK12_GLOBAL__N_121system_error_category7messageEi
// type: int __fastcall(_anonymous_namespace_::system_error_category *this, int, int)
#[doc(alias = "anonymous namespace::system_error_category::message(int)const")]
#[doc(alias = "__ZNK12_GLOBAL__N_121system_error_category7messageEi")]
pub fn stub_0x23b7e4() -> ! {
    todo!("0x23b7e4 anonymous namespace::system_error_category::message(int)const")
}

// 0x23b838 — __ZNK12_GLOBAL__N_121system_error_category23default_error_conditionEi
// type: void __fastcall(_anonymous_namespace_::system_error_category *this, int, int)
#[doc(alias = "anonymous namespace::system_error_category::default_error_condition(int)const")]
#[doc(alias = "__ZNK12_GLOBAL__N_121system_error_category23default_error_conditionEi")]
pub fn stub_0x23b838() -> ! {
    todo!("0x23b838 anonymous namespace::system_error_category::default_error_condition(int)const")
}

// 0x23ca3c — __ZNK5boost6system14error_category23default_error_conditionEi
// type: _QWORD *__fastcall(_QWORD *this, int, __int64)
#[doc(alias = "boost::system::error_category::default_error_condition(int)const")]
#[doc(alias = "__ZNK5boost6system14error_category23default_error_conditionEi")]
pub fn stub_0x23ca3c() -> ! {
    todo!("0x23ca3c boost::system::error_category::default_error_condition(int)const")
}

// 0x23ca44 — __ZNK5boost6system14error_category10equivalentEiRKNS0_15error_conditionE
// type: bool __fastcall(int, int, _DWORD *)
#[doc(alias = "boost::system::error_category::equivalent(int,boost::system::error_condition const&)const")]
#[doc(alias = "__ZNK5boost6system14error_category10equivalentEiRKNS0_15error_conditionE")]
pub fn stub_0x23ca44() -> ! {
    todo!("0x23ca44 boost::system::error_category::equivalent(int,boost::system::error_condition const&)const")
}

// 0x23ca70 — __ZNK5boost6system14error_category10equivalentERKNS0_10error_codeEi
// type: bool __fastcall(int, _DWORD *, int)
#[doc(alias = "boost::system::error_category::equivalent(boost::system::error_code const&,int)const")]
#[doc(alias = "__ZNK5boost6system14error_category10equivalentERKNS0_10error_codeEi")]
pub fn stub_0x23ca70() -> ! {
    todo!("0x23ca70 boost::system::error_category::equivalent(boost::system::error_code const&,int)const")
}

// 0x23ca88 — __GLOBAL__I_a_41
// type: void()
#[doc(alias = "global constructor keyed to_a_41")]
#[doc(alias = "__GLOBAL__I_a_41")]
pub fn stub_0x23ca88() -> ! {
    todo!("0x23ca88 __GLOBAL__I_a_41")
}

// 0x23cb64 — __ZN5boost9iostreams6detail11gzip_header7processEc
// type: void __fastcall(boost::iostreams::detail::gzip_header *this, unsigned __int8)
#[doc(alias = "boost::iostreams::detail::gzip_header::process(char)")]
#[doc(alias = "__ZN5boost9iostreams6detail11gzip_header7processEc")]
pub fn stub_0x23cb64() -> ! {
    todo!("0x23cb64 boost::iostreams::detail::gzip_header::process(char)")
}

// 0x23cef0 — __ZN5boost9iostreams6detail11gzip_header5resetEv
// type: int __fastcall(boost::iostreams::detail::gzip_header *this)
#[doc(alias = "boost::iostreams::detail::gzip_header::reset(void)")]
#[doc(alias = "__ZN5boost9iostreams6detail11gzip_header5resetEv")]
pub fn stub_0x23cef0() -> ! {
    todo!("0x23cef0 boost::iostreams::detail::gzip_header::reset(void)")
}

// 0x23cf2c — __ZN5boost9iostreams6detail11gzip_footer7processEc
// type: _DWORD *__fastcall(_DWORD *this, unsigned __int8)
#[doc(alias = "boost::iostreams::detail::gzip_footer::process(char)")]
#[doc(alias = "__ZN5boost9iostreams6detail11gzip_footer7processEc")]
pub fn stub_0x23cf2c() -> ! {
    todo!("0x23cf2c boost::iostreams::detail::gzip_footer::process(char)")
}

// 0x23cf7c — __ZN5boost9iostreams6detail11gzip_footer5resetEv
// type: _DWORD *__fastcall(_DWORD *this)
#[doc(alias = "boost::iostreams::detail::gzip_footer::reset(void)")]
#[doc(alias = "__ZN5boost9iostreams6detail11gzip_footer5resetEv")]
pub fn stub_0x23cf7c() -> ! {
    todo!("0x23cf7c boost::iostreams::detail::gzip_footer::reset(void)")
}

// 0x23cf8c — __ZN5boost9iostreams10zlib_error5checkEi
// type: void __fastcall(boost::iostreams::zlib_error *this, int)
#[doc(alias = "boost::iostreams::zlib_error::check(int)")]
#[doc(alias = "__ZN5boost9iostreams10zlib_error5checkEi")]
pub fn stub_0x23cf8c() -> ! {
    todo!("0x23cf8c boost::iostreams::zlib_error::check(int)")
}

// 0x23d0c8 — __ZN5boost9iostreams6detail9zlib_baseC2Ev
// type: boost::iostreams::detail::zlib_base *__fastcall(boost::iostreams::detail::zlib_base *this)
#[doc(alias = "boost::iostreams::detail::zlib_base::zlib_base(void)")]
#[doc(alias = "__ZN5boost9iostreams6detail9zlib_baseC2Ev")]
pub fn stub_0x23d0c8() -> ! {
    todo!("0x23d0c8 boost::iostreams::detail::zlib_base::zlib_base(void)")
}

// 0x23d0e8 — __ZN5boost9iostreams6detail9zlib_baseD2Ev
// type: void __fastcall(void **this)
#[doc(alias = "boost::iostreams::detail::zlib_base::~zlib_base()")]
#[doc(alias = "__ZN5boost9iostreams6detail9zlib_baseD2Ev")]
pub fn stub_0x23d0e8() {
    // IDA 0x23d0e8: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x23d0fc — __ZN5boost9iostreams6detail9zlib_base6beforeERPKcS4_RPcS6_
// type: int __fastcall(boost::iostreams::detail::zlib_base *this, const char **, const char *, char **, char *)
#[doc(alias = "boost::iostreams::detail::zlib_base::before(char const*&,char const*,char *&,char *)")]
#[doc(alias = "__ZN5boost9iostreams6detail9zlib_base6beforeERPKcS4_RPcS6_")]
pub fn stub_0x23d0fc() -> ! {
    todo!("0x23d0fc boost::iostreams::detail::zlib_base::before(char const*&,char const*,char *&,char *)")
}

// 0x23d120 — __ZN5boost9iostreams6detail9zlib_base5afterERPKcRPcb
// type: const char *__fastcall(boost::iostreams::detail::zlib_base *this, const char **, char **, int)
#[doc(alias = "boost::iostreams::detail::zlib_base::after(char const*&,char *&,bool)")]
#[doc(alias = "__ZN5boost9iostreams6detail9zlib_base5afterERPKcRPcb")]
pub fn stub_0x23d120() -> ! {
    todo!("0x23d120 boost::iostreams::detail::zlib_base::after(char const*&,char *&,bool)")
}

// 0x23d180 — __ZN5boost9iostreams6detail9zlib_base8xdeflateEi
// type: int __fastcall(z_streamp *this, int)
#[doc(alias = "boost::iostreams::detail::zlib_base::xdeflate(int)")]
#[doc(alias = "__ZN5boost9iostreams6detail9zlib_base8xdeflateEi")]
pub fn stub_0x23d180() -> ! {
    todo!("0x23d180 boost::iostreams::detail::zlib_base::xdeflate(int)")
}

// 0x23d18c — __ZN5boost9iostreams6detail9zlib_base8xinflateEi
// type: int __fastcall(z_streamp *this, int)
#[doc(alias = "boost::iostreams::detail::zlib_base::xinflate(int)")]
#[doc(alias = "__ZN5boost9iostreams6detail9zlib_base8xinflateEi")]
pub fn stub_0x23d18c() -> ! {
    todo!("0x23d18c boost::iostreams::detail::zlib_base::xinflate(int)")
}

// 0x23d198 — __ZN5boost9iostreams6detail9zlib_base5resetEbb
// type: int __fastcall(z_stream **this, int, int)
#[doc(alias = "boost::iostreams::detail::zlib_base::reset(bool,bool)")]
#[doc(alias = "__ZN5boost9iostreams6detail9zlib_base5resetEbb")]
pub fn stub_0x23d198() -> ! {
    todo!("0x23d198 boost::iostreams::detail::zlib_base::reset(bool,bool)")
}

// 0x23d1c8 — __ZN5boost9iostreams6detail9zlib_base7do_initERKNS0_11zlib_paramsEbPFPvS6_jjEPFvS6_S6_ES6_
// type: void __fastcall(int, int, int, int, int, void *)
#[doc(alias = "boost::iostreams::detail::zlib_base::do_init(boost::iostreams::zlib_params const&,bool,void * (*)(void *,unsigned int,unsigned int),void (*)(void *,void *),void *)")]
#[doc(alias = "__ZN5boost9iostreams6detail9zlib_base7do_initERKNS0_11zlib_paramsEbPFPvS6_jjEPFvS6_S6_ES6_")]
pub fn stub_0x23d1c8() -> ! {
    todo!("0x23d1c8 boost::iostreams::detail::zlib_base::do_init(boost::iostreams::zlib_params const&,bool,void * (*)(void *,unsigned int,unsigned int),void (*)(void *,void *),void *)")
}

// 0x23d238 — __ZN5boost15throw_exceptionINS_9iostreams10zlib_errorEEEvRKT_
// type: void __fastcall __noreturn(int)
#[doc(alias = "void boost::throw_exception<boost::iostreams::zlib_error>(boost::iostreams::zlib_error const&)")]
#[doc(alias = "__ZN5boost15throw_exceptionINS_9iostreams10zlib_errorEEEvRKT_")]
pub fn stub_0x23d238() -> ! {
    todo!("0x23d238 void boost::throw_exception<boost::iostreams::zlib_error>(boost::iostreams::zlib_error const&)")
}

// 0x23d390 — __ZN5boost9iostreams10zlib_errorD1Ev
// type: void __fastcall(boost::iostreams::zlib_error *__hidden this)
#[doc(alias = "boost::iostreams::zlib_error::~zlib_error()")]
#[doc(alias = "__ZN5boost9iostreams10zlib_errorD1Ev")]
pub fn stub_0x23d390() {
    // IDA 0x23d390: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x23d39c — __ZN5boost9iostreams10zlib_errorD0Ev
// type: void __fastcall(boost::iostreams::zlib_error *__hidden this)
#[doc(alias = "boost::iostreams::zlib_error::~zlib_error()")]
#[doc(alias = "__ZN5boost9iostreams10zlib_errorD0Ev")]
pub fn stub_0x23d39c() {
    // IDA 0x23d39c: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x23d3b0 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED1Ev
// type: std::ios_base::failure *__fastcall(std::ios_base::failure *)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl()")]
#[doc(alias = "__ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED1Ev")]
pub fn stub_0x23d3b0() {
    // IDA 0x23d3b0: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x23d468 — __ZN5boost16exception_detail19error_info_injectorINS_9iostreams10zlib_errorEED1Ev
// type: int __fastcall(std::ios_base::failure *)
#[doc(alias = "boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>::~error_info_injector()")]
#[doc(alias = "__ZN5boost16exception_detail19error_info_injectorINS_9iostreams10zlib_errorEED1Ev")]
pub fn stub_0x23d468() {
    // IDA 0x23d468: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x23d520 — __ZThn12_N5boost16exception_detail19error_info_injectorINS_9iostreams10zlib_errorEED1Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::iostreams::zlib_error>::~error_info_injector()")]
#[doc(alias = "__ZThn12_N5boost16exception_detail19error_info_injectorINS_9iostreams10zlib_errorEED1Ev")]
pub fn stub_0x23d520() {
    // IDA 0x23d520: __ZThn12 thunk (D1 base dtor): `this -= 12`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x23d5d8 — __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED1Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl()")]
#[doc(alias = "__ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED1Ev")]
pub fn stub_0x23d5d8() {
    // IDA 0x23d5d8: __ZThn12 thunk (D1 base dtor): `this -= 12`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x23d690 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED1Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl()")]
#[doc(alias = "__ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED1Ev")]
pub fn stub_0x23d690() {
    // IDA 0x23d690: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x23d75c — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED0Ev
// type: void __fastcall(std::ios_base::failure *)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl()")]
#[doc(alias = "__ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED0Ev")]
pub fn stub_0x23d75c() {
    // IDA 0x23d75c: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x23d818 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEE5cloneEv
// type: char *__fastcall(int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone(void)const")]
#[doc(alias = "__ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEE5cloneEv")]
pub fn stub_0x23d818() -> ! {
    todo!("0x23d818 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone(void)const")
}

// 0x23d8d4 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEE7rethrowEv
// type: void __fastcall __noreturn(int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::rethrow(void)const")]
#[doc(alias = "__ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEE7rethrowEv")]
pub fn stub_0x23d8d4() -> ! {
    todo!("0x23d8d4 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::rethrow(void)const")
}

// 0x23d984 — __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl()")]
#[doc(alias = "__ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED0Ev")]
pub fn stub_0x23d984() {
    // IDA 0x23d984: __ZThn12 thunk (D0 deleting dtor): `this -= 12`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x23da40 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEE5cloneEv
// type: char *__fastcall(_DWORD *)
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone(void)const")]
#[doc(alias = "__ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEE5cloneEv")]
pub fn stub_0x23da40() -> ! {
    todo!("0x23da40 virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone(void)const")
}