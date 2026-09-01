// Auto-generated skeletons for rbx-script — filler EA-sorted after 0x2360c4 (next 120) [filler EA-sorted ascending earliest gap]
// Filter: Lua|Script|Yield|CodeGen (7431 filtered, all already stubbed globally) — filler EA-sorted ascending
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +120 stubs | range 0x236318..0x23f50c | existing 7751 -> 7871 total (filler after 0x2360c4, EA-sorted ascending)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; ` stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0x236318 — __ZN5boost6thread26do_try_join_until_noexceptERK8timespecRb
// type: int __fastcall(boost::thread *this, const timespec *, bool *)
// was: int __fastcall(boost::thread *this, const timespec *, bool *)
#[doc(alias = "boost::thread::do_try_join_until_noexcept(timespec const&,bool &)")]
pub fn stub_0x236318() -> ! {
    todo!("0x236318 __ZN5boost6thread26do_try_join_until_noexceptERK8timespecRb")
}

// 0x236598 — __ZN5boost6thread6detachEv
// type: void __fastcall(boost::thread *this)
// was: void __fastcall(boost::thread *this)
#[doc(alias = "boost::thread::detach(void)")]
pub fn stub_0x236598() -> ! {
    todo!("0x236598 __ZN5boost6thread6detachEv")
}

// 0x2366b0 — __ZN5boost11this_thread5hiden11sleep_untilERK8timespec
// type: void __fastcall(boost::this_thread::hiden *this, const timespec *, int, int)
// was: void __fastcall(boost::this_thread::hiden *this, const timespec *, int, int)
#[doc(alias = "boost::this_thread::hiden::sleep_until(timespec const&)")]
pub fn stub_0x2366b0() -> ! {
    todo!("0x2366b0 __ZN5boost11this_thread5hiden11sleep_untilERK8timespec")
}

// 0x2368cc — __ZN5boost6thread13native_handleEv
// type: int __fastcall(boost::thread *this)
// was: int __fastcall(boost::thread *this)
#[doc(alias = "boost::thread::native_handle(void)")]
pub fn stub_0x2368cc() -> ! {
    todo!("0x2368cc __ZN5boost6thread13native_handleEv")
}

// 0x236a00 — __ZN5boost11this_thread18interruption_pointEv
// type: void __fastcall(boost::this_thread *this, int, int, int)
// was: void __fastcall(boost::this_thread *this, int, int, int)
#[doc(alias = "boost::this_thread::interruption_point(void)")]
pub fn stub_0x236a00() -> ! {
    todo!("0x236a00 __ZN5boost11this_thread18interruption_pointEv")
}

// 0x236b14 — __ZN5boost11this_thread20disable_interruptionC1Ev
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(boost::this_thread::disable_interruption *this, int, int, int)
// was: void __fastcall __spoils<R1,R2,R3,R12,LR>(boost::this_thread::disable_interruption *this, int, int, int)
#[doc(alias = "boost::this_thread::disable_interruption::disable_interruption(void)")]
pub fn stub_0x236b14() -> ! {
    todo!("0x236b14 __ZN5boost11this_thread20disable_interruptionC1Ev")
}

// 0x236c14 — __ZN5boost11this_thread20disable_interruptionD1Ev
// type: void __fastcall(boost::this_thread::disable_interruption *this, int, int, int)
// was: void __fastcall(boost::this_thread::disable_interruption *this, int, int, int)
#[doc(alias = "boost::this_thread::disable_interruption::~disable_interruption()")]
pub fn stub_0x236c14() -> ! {
    todo!("0x236c14 __ZN5boost11this_thread20disable_interruptionD1Ev")
}

// 0x236d04 — __ZN5boost12_GLOBAL__N_131get_or_make_current_thread_dataEv
// type: void *__fastcall(boost::_anonymous_namespace_ *this, int, int, int)
// was: void *__fastcall(boost::_anonymous_namespace_ *this, int, int, int)
#[doc(alias = "boost::anonymous namespace::get_or_make_current_thread_data(void)")]
pub fn stub_0x236d04() -> ! {
    todo!("0x236d04 __ZN5boost12_GLOBAL__N_131get_or_make_current_thread_dataEv")
}

// 0x236ec0 — __ZN5boost6detail12get_tss_dataEPKv
// type: _DWORD __fastcall(boost::detail *__hidden this, const void *)
// was: _DWORD __fastcall(boost::detail *__hidden this, const void *)
#[doc(alias = "boost::detail::get_tss_data(void const*)")]
pub fn stub_0x236ec0() -> ! {
    todo!("0x236ec0 __ZN5boost6detail12get_tss_dataEPKv")
}

// 0x236f30 — __ZN5boost6detail16add_new_tss_nodeEPKvNS_10shared_ptrINS0_20tss_cleanup_functionEEEPv
// type: void __fastcall(boost::_anonymous_namespace_ *, int *, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: void __fastcall(boost::_anonymous_namespace_ *, int *, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::detail::add_new_tss_node(void const*,rbx_core::SharedPtr<boost::detail::tss_cleanup_function>,void *)")]
pub fn stub_0x236f30() -> ! {
    todo!("0x236f30 __ZN5boost6detail16add_new_tss_nodeEPKvNS_10shared_ptrINS0_20tss_cleanup_functionEEEPv")
}

// 0x237130 — __ZN5boost6detail12set_tss_dataEPKvNS_10shared_ptrINS0_20tss_cleanup_functionEEEPvb
// type: void __fastcall(boost::_anonymous_namespace_ *, int *, int, int)
// was: void __fastcall(boost::_anonymous_namespace_ *, int *, int, int)
#[doc(alias = "boost::detail::set_tss_data(void const*,rbx_core::SharedPtr<boost::detail::tss_cleanup_function>,void *,bool)")]
pub fn stub_0x237130() -> ! {
    todo!("0x237130 __ZN5boost6detail12set_tss_dataEPKvNS_10shared_ptrINS0_20tss_cleanup_functionEEEPvb")
}

// 0x237348 — __ZN5boost12_GLOBAL__N_126externally_launched_threadD1Ev
// type: void __fastcall(boost::_anonymous_namespace_::externally_launched_thread *__hidden this)
// was: void __fastcall(boost::_anonymous_namespace_::externally_launched_thread *__hidden this)
#[doc(alias = "boost::anonymous namespace::externally_launched_thread::~externally_launched_thread()")]
pub fn stub_0x237348() -> ! {
    todo!("0x237348 __ZN5boost12_GLOBAL__N_126externally_launched_threadD1Ev")
}

// 0x237354 — __ZN5boost12_GLOBAL__N_126externally_launched_threadD0Ev
// type: void __fastcall(boost::_anonymous_namespace_::externally_launched_thread *__hidden this)
// was: void __fastcall(boost::_anonymous_namespace_::externally_launched_thread *__hidden this)
#[doc(alias = "boost::anonymous namespace::externally_launched_thread::~externally_launched_thread()")]
pub fn stub_0x237354() -> ! {
    todo!("0x237354 __ZN5boost12_GLOBAL__N_126externally_launched_threadD0Ev")
}

// 0x237368 — __ZN5boost12_GLOBAL__N_126externally_launched_thread3runEv
// type: void __fastcall(boost::_anonymous_namespace_::externally_launched_thread *this)
// was: void __fastcall(boost::_anonymous_namespace_::externally_launched_thread *this)
#[doc(alias = "boost::anonymous namespace::externally_launched_thread::run(void)")]
pub fn stub_0x237368() -> ! {
    todo!("0x237368 __ZN5boost12_GLOBAL__N_126externally_launched_thread3runEv")
}

// 0x23736c — __ZN5boost12_GLOBAL__N_126externally_launched_thread25notify_all_at_thread_exitEPNS_18condition_variableEPNS_5mutexE
// type: void()
#[doc(alias = "boost::anonymous namespace::externally_launched_thread::notify_all_at_thread_exit(boost::condition_variable *,boost::mutex *)")]
pub fn stub_0x23736c() -> ! {
    todo!("0x23736c __ZN5boost12_GLOBAL__N_126externally_launched_thread25notify_all_at_thread_exitEPNS_18condition_variableEPNS_5mutexE")
}

// 0x2374bc — __ZN5boost10shared_ptrINS_6detail16thread_data_baseEEaSERKS3_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *)
#[doc(alias = "rbx_core::SharedPtr<boost::detail::thread_data_base>::operator=(rbx_core::SharedPtr<boost::detail::thread_data_base> const&)")]
pub fn stub_0x2374bc() -> ! {
    todo!("0x2374bc __ZN5boost10shared_ptrINS_6detail16thread_data_baseEEaSERKS3_")
}

// 0x2375b0 — __ZN5boost10shared_ptrINS_6detail20tss_cleanup_functionEEaSERKS3_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *)
#[doc(alias = "rbx_core::SharedPtr<boost::detail::tss_cleanup_function>::operator=(rbx_core::SharedPtr<boost::detail::tss_cleanup_function> const&)")]
pub fn stub_0x2375b0() -> ! {
    todo!("0x2375b0 __ZN5boost10shared_ptrINS_6detail20tss_cleanup_functionEEaSERKS3_")
}

// 0x2376a4 — __ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE5eraseESt17_Rb_tree_iteratorIS7_ESF_
// type: void __fastcall(int, _Rb_tree_node_base *, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<void const*,std::pair<void const* const,boost::detail::tss_data_node>,std::_Select1st<std::pair<void const* const,boost::detail::tss_data_node>>,std::less<void const*>,std::allocator<std::pair<void const* const,boost::detail::tss_data_node>>>::erase(std::_Rb_tree_iterator<std::pair<void const* const,boost::detail::tss_data_node>>,std::_Rb_tree_iterator<std::pair<void const* const,boost::detail::tss_data_node>>)")]
pub fn stub_0x2376a4() -> ! {
    todo!("0x2376a4 __ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE5eraseESt17_Rb_tree_iteratorIS7_ESF_")
}

// 0x237798 — __ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<void const*,std::pair<void const* const,boost::detail::tss_data_node>,std::_Select1st<std::pair<void const* const,boost::detail::tss_data_node>>,std::less<void const*>,std::allocator<std::pair<void const* const,boost::detail::tss_data_node>>>::_M_erase(std::_Rb_tree_node<std::pair<void const* const,boost::detail::tss_data_node>> *)")]
pub fn stub_0x237798() -> ! {
    todo!("0x237798 __ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")
}

// 0x237848 — __ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE16_M_insert_uniqueERKS7_
// type: int __fastcall(int, _DWORD *, int *)
#[doc(alias = "std::_Rb_tree<void const*,std::pair<void const* const,boost::detail::tss_data_node>,std::_Select1st<std::pair<void const* const,boost::detail::tss_data_node>>,std::less<void const*>,std::allocator<std::pair<void const* const,boost::detail::tss_data_node>>>::_M_insert_unique(std::pair<void const* const,boost::detail::tss_data_node> const&)")]
pub fn stub_0x237848() -> ! {
    todo!("0x237848 __ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE16_M_insert_uniqueERKS7_")
}

// 0x2378fc — __ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE14_M_create_nodeERKS7_
// type: _DWORD *__fastcall(int, int *)
#[doc(alias = "std::_Rb_tree<void const*,std::pair<void const* const,boost::detail::tss_data_node>,std::_Select1st<std::pair<void const* const,boost::detail::tss_data_node>>,std::less<void const*>,std::allocator<std::pair<void const* const,boost::detail::tss_data_node>>>::_M_create_node(std::pair<void const* const,boost::detail::tss_data_node> const&)")]
pub fn stub_0x2378fc() -> ! {
    todo!("0x2378fc __ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE14_M_create_nodeERKS7_")
}

// 0x2379ec — __ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_S2_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int)
#[doc(alias = "void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data_base>(rbx_core::SharedPtr<boost::detail::thread_data_base> const*,boost::detail::thread_data_base *)const")]
pub fn stub_0x2379ec() -> ! {
    todo!("0x2379ec __ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_S2_EEvPKNS_10shared_ptrIT_EEPT0_")
}

// 0x237b40 — __ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data_base>::~sp_counted_impl_p()")]
pub fn stub_0x237b40() -> ! {
    todo!("0x237b40 __ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEED1Ev")
}

// 0x237b44 — __ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data_base>::~sp_counted_impl_p()")]
pub fn stub_0x237b44() -> ! {
    todo!("0x237b44 __ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEED0Ev")
}

// 0x237b50 — __ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data_base>::dispose(void)")]
pub fn stub_0x237b50() -> ! {
    todo!("0x237b50 __ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEE7disposeEv")
}

// 0x237b64 — __ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data_base>::get_deleter(std::type_info const&)")]
pub fn stub_0x237b64() -> ! {
    todo!("0x237b64 __ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEE11get_deleterERKSt9type_info")
}

// 0x237b68 — __ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data_base>::get_untyped_deleter(void)")]
pub fn stub_0x237b68() -> ! {
    todo!("0x237b68 __ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEE19get_untyped_deleterEv")
}

// 0x237b6c — __ZN5boost6detail18future_object_base22mark_finished_internalERNS_11unique_lockINS_5mutexEEE
// type: void __fastcall(int)
#[doc(alias = "boost::detail::future_object_base::mark_finished_internal(boost::unique_lock<boost::mutex> &)")]
pub fn stub_0x237b6c() -> ! {
    todo!("0x237b6c __ZN5boost6detail18future_object_base22mark_finished_internalERNS_11unique_lockINS_5mutexEEE")
}

// 0x237d60 — __ZN5boost10filesystem6detail14symlink_statusERKNS0_4pathEPNS_6system10error_codeE
// type: void __fastcall(int *, const char **, int *)
#[doc(alias = "boost::filesystem::detail::symlink_status(boost::filesystem::path const&,boost::system::error_code *)")]
pub fn stub_0x237d60() -> ! {
    todo!("0x237d60 __ZN5boost10filesystem6detail14symlink_statusERKNS0_4pathEPNS_6system10error_codeE")
}

// 0x237fa4 — __ZN5boost10filesystem6detail12current_pathEPNS_6system10error_codeE
// type: void __fastcall(std::string *, _DWORD *)
#[doc(alias = "boost::filesystem::detail::current_path(boost::system::error_code *)")]
pub fn stub_0x237fa4() -> ! {
    todo!("0x237fa4 __ZN5boost10filesystem6detail12current_pathEPNS_6system10error_codeE")
}

// 0x238258 — __ZN12_GLOBAL__N_15errorEbRKN5boost10filesystem4pathEPNS0_6system10error_codeERKSs
// type: int __fastcall(int, void *, int, int)
#[doc(alias = "anonymous namespace::error(bool,boost::filesystem::path const&,boost::system::error_code *,std::string const&)")]
pub fn stub_0x238258() -> ! {
    todo!("0x238258 __ZN12_GLOBAL__N_15errorEbRKN5boost10filesystem4pathEPNS0_6system10error_codeERKSs")
}

// 0x23837c — __ZN5boost10filesystem6detail12initial_pathEPNS_6system10error_codeE
// type: void __fastcall(std::string *, _DWORD *)
#[doc(alias = "boost::filesystem::detail::initial_path(boost::system::error_code *)")]
pub fn stub_0x23837c() -> ! {
    todo!("0x23837c __ZN5boost10filesystem6detail12initial_pathEPNS_6system10error_codeE")
}

// 0x23852c — __ZN5boost10filesystem6detail8is_emptyERKNS0_4pathEPNS_6system10error_codeE
// type: bool __fastcall(const char **, int)
#[doc(alias = "boost::filesystem::detail::is_empty(boost::filesystem::path const&,boost::system::error_code *)")]
pub fn stub_0x23852c() -> ! {
    todo!("0x23852c __ZN5boost10filesystem6detail8is_emptyERKNS0_4pathEPNS_6system10error_codeE")
}

// 0x2386d4 — __ZN5boost10filesystem6detail6removeERKNS0_4pathEPNS_6system10error_codeE
// type: int __fastcall(const char **, int *)
#[doc(alias = "boost::filesystem::detail::remove(boost::filesystem::path const&,boost::system::error_code *)")]
pub fn stub_0x2386d4() -> ! {
    todo!("0x2386d4 __ZN5boost10filesystem6detail6removeERKNS0_4pathEPNS_6system10error_codeE")
}

// 0x2388a8 — __ZN12_GLOBAL__N_124remove_file_or_directoryERKN5boost10filesystem4pathENS1_9file_typeEPNS0_6system10error_codeE
// type: bool __fastcall(const char **, int, _DWORD *)
#[doc(alias = "anonymous namespace::remove_file_or_directory(boost::filesystem::path const&,boost::filesystem::file_type,boost::system::error_code *)")]
pub fn stub_0x2388a8() -> ! {
    todo!("0x2388a8 __ZN12_GLOBAL__N_124remove_file_or_directoryERKN5boost10filesystem4pathENS1_9file_typeEPNS0_6system10error_codeE")
}

// 0x238adc — __ZN5boost10filesystem6detail6statusERKNS0_4pathEPNS_6system10error_codeE
// type: void __fastcall(int *, const char **, int *)
#[doc(alias = "boost::filesystem::detail::status(boost::filesystem::path const&,boost::system::error_code *)")]
pub fn stub_0x238adc() -> ! {
    todo!("0x238adc __ZN5boost10filesystem6detail6statusERKNS0_4pathEPNS_6system10error_codeE")
}

// 0x238d18 — __ZN5boost10filesystem6detail15system_completeERKNS0_4pathEPNS_6system10error_codeE
// type: void __fastcall(std::string *, const std::string *)
#[doc(alias = "boost::filesystem::detail::system_complete(boost::filesystem::path const&,boost::system::error_code *)")]
pub fn stub_0x238d18() -> ! {
    todo!("0x238d18 __ZN5boost10filesystem6detail15system_completeERKNS0_4pathEPNS_6system10error_codeE")
}

// 0x238f14 — __ZNK5boost10filesystem15directory_entry12m_get_statusEPNS_6system10error_codeE
// type: __int64 __fastcall(_QWORD *, int, int *)
#[doc(alias = "boost::filesystem::directory_entry::m_get_status(boost::system::error_code *)const")]
pub fn stub_0x238f14() -> ! {
    todo!("0x238f14 __ZNK5boost10filesystem15directory_entry12m_get_statusEPNS_6system10error_codeE")
}

// 0x238f80 — __ZN5boost10filesystem6detail13dir_itr_closeERPvS3_
// type: int __fastcall(boost::filesystem::detail *this, void **, void **)
// was: int __fastcall(boost::filesystem::detail *this, void **, void **)
#[doc(alias = "boost::filesystem::detail::dir_itr_close(void *&,void *&)")]
pub fn stub_0x238f80() -> ! {
    todo!("0x238f80 __ZN5boost10filesystem6detail13dir_itr_closeERPvS3_")
}

// 0x238fd4 — __ZN5boost10filesystem6detail28directory_iterator_constructERNS0_18directory_iteratorERKNS0_4pathEPNS_6system10error_codeE
// type: void __fastcall(std::string **, const char **, std::string **)
#[doc(alias = "boost::filesystem::detail::directory_iterator_construct(boost::filesystem::directory_iterator &,boost::filesystem::path const&,boost::system::error_code *)")]
pub fn stub_0x238fd4() -> ! {
    todo!("0x238fd4 __ZN5boost10filesystem6detail28directory_iterator_constructERNS0_18directory_iteratorERKNS0_4pathEPNS_6system10error_codeE")
}

// 0x239668 — __ZN5boost10filesystem6detail28directory_iterator_incrementERNS0_18directory_iteratorEPNS_6system10error_codeE
// type: void __fastcall(int *, dirent **)
#[doc(alias = "boost::filesystem::detail::directory_iterator_increment(boost::filesystem::directory_iterator &,boost::system::error_code *)")]
pub fn stub_0x239668() -> ! {
    todo!("0x239668 __ZN5boost10filesystem6detail28directory_iterator_incrementERNS0_18directory_iteratorEPNS_6system10error_codeE")
}

// 0x239b34 — __ZN5boost10filesystem18directory_iteratorD1Ev
// type: void __fastcall(boost::filesystem::directory_iterator *__hidden this)
// was: void __fastcall(boost::filesystem::directory_iterator *__hidden this)
#[doc(alias = "boost::filesystem::directory_iterator::~directory_iterator()")]
pub fn stub_0x239b34() -> ! {
    todo!("0x239b34 __ZN5boost10filesystem18directory_iteratorD1Ev")
}

// 0x239bc8 — __ZN5boost10filesystem16filesystem_errorD1Ev
// type: void __fastcall(std::runtime_error *this)
#[doc(alias = "boost::filesystem::filesystem_error::~filesystem_error()")]
pub fn stub_0x239bc8() -> ! {
    todo!("0x239bc8 __ZN5boost10filesystem16filesystem_errorD1Ev")
}

// 0x239cc8 — __ZN5boost10filesystem16filesystem_errorC2ERKSsNS_6system10error_codeE
// type: std::runtime_error *__fastcall(std::runtime_error *, const std::string *, std::runtime_error_vtbl *, const char *, boost::detail::sp_counted_base *, std::runtime_error *, int, int, void *, int)
// was: std::runtime_error *__fastcall(std::runtime_error *, const std::string *, std::runtime_error_vtbl *, const char *, boost::detail::sp_counted_base *, std::runtime_error *, int, int, void *, int)
#[doc(alias = "boost::filesystem::filesystem_error::filesystem_error(std::string const&,boost::system::error_code)")]
pub fn stub_0x239cc8() -> ! {
    todo!("0x239cc8 __ZN5boost10filesystem16filesystem_errorC2ERKSsNS_6system10error_codeE")
}

// 0x239e90 — __ZN5boost10filesystem16filesystem_errorD0Ev
// type: void __fastcall(std::runtime_error *this)
#[doc(alias = "boost::filesystem::filesystem_error::~filesystem_error()")]
pub fn stub_0x239e90() -> ! {
    todo!("0x239e90 __ZN5boost10filesystem16filesystem_errorD0Ev")
}

// 0x239f94 — __ZNK5boost10filesystem16filesystem_error4whatEv
// type: int __fastcall(boost::filesystem::filesystem_error *this)
// was: int __fastcall(boost::filesystem::filesystem_error *this)
#[doc(alias = "boost::filesystem::filesystem_error::what(void)const")]
pub fn stub_0x239f94() -> ! {
    todo!("0x239f94 __ZNK5boost10filesystem16filesystem_error4whatEv")
}

// 0x23a11c — __ZN5boost6detail20sp_pointer_constructINS_10filesystem16filesystem_error5m_impES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, boost::detail::sp_counted_base **, int, void *, int)
// was: void __fastcall(int, int, boost::detail::sp_counted_base **, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<boost::filesystem::filesystem_error::m_imp,boost::filesystem::filesystem_error::m_imp>(rbx_core::SharedPtr<boost::filesystem::filesystem_error::m_imp> *,boost::filesystem::filesystem_error::m_imp *,boost::detail::shared_count &)")]
pub fn stub_0x23a11c() -> ! {
    todo!("0x23a11c __ZN5boost6detail20sp_pointer_constructINS_10filesystem16filesystem_error5m_impES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

// 0x23a2bc — __ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::filesystem::filesystem_error::m_imp>::~sp_counted_impl_p()")]
pub fn stub_0x23a2bc() -> ! {
    todo!("0x23a2bc __ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEED1Ev")
}

// 0x23a2c0 — __ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::filesystem::filesystem_error::m_imp>::~sp_counted_impl_p()")]
pub fn stub_0x23a2c0() -> ! {
    todo!("0x23a2c0 __ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEED0Ev")
}

// 0x23a2cc — __ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEE7disposeEv
// type: void __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::filesystem::filesystem_error::m_imp>::dispose(void)")]
pub fn stub_0x23a2cc() -> ! {
    todo!("0x23a2cc __ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEE7disposeEv")
}

// 0x23a38c — __ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::filesystem::filesystem_error::m_imp>::get_deleter(std::type_info const&)")]
pub fn stub_0x23a38c() -> ! {
    todo!("0x23a38c __ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEE11get_deleterERKSt9type_info")
}

// 0x23a390 — __ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::filesystem::filesystem_error::m_imp>::get_untyped_deleter(void)")]
pub fn stub_0x23a390() -> ! {
    todo!("0x23a390 __ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEE19get_untyped_deleterEv")
}

// 0x23a394 — __ZN5boost10filesystem16filesystem_errorC2ERKSsRKNS0_4pathENS_6system10error_codeE
// type: std::runtime_error *__fastcall(std::runtime_error *, const std::string *, const std::string *, std::runtime_error_vtbl *, boost::detail::sp_counted_base *, std::runtime_error *, int, int, void *, int)
// was: std::runtime_error *__fastcall(std::runtime_error *, const std::string *, const std::string *, std::runtime_error_vtbl *, boost::detail::sp_counted_base *, std::runtime_error *, int, int, void *, int)
#[doc(alias = "boost::filesystem::filesystem_error::filesystem_error(std::string const&,boost::filesystem::path const&,boost::system::error_code)")]
pub fn stub_0x23a394() -> ! {
    todo!("0x23a394 __ZN5boost10filesystem16filesystem_errorC2ERKSsRKNS0_4pathENS_6system10error_codeE")
}

// 0x23a630 — __ZN5boost10filesystem4pathdVERKS1_
// type: boost::filesystem::path *__fastcall(boost::filesystem::path *, const std::string *)
// was: boost::filesystem::path *__fastcall(boost::filesystem::path *, const std::string *)
#[doc(alias = "boost::filesystem::path::operator/=(boost::filesystem::path const&)")]
pub fn stub_0x23a630() -> ! {
    todo!("0x23a630 __ZN5boost10filesystem4pathdVERKS1_")
}

// 0x23a7b8 — __ZN5boost10filesystem4path28m_append_separator_if_neededEv
// type: int __fastcall(boost::filesystem::path *this)
// was: int __fastcall(boost::filesystem::path *this)
#[doc(alias = "boost::filesystem::path::m_append_separator_if_needed(void)")]
pub fn stub_0x23a7b8() -> ! {
    todo!("0x23a7b8 __ZN5boost10filesystem4path28m_append_separator_if_neededEv")
}

// 0x23a830 — __ZN5boost10filesystem4pathdVEPKc
// type: boost::filesystem::path *__fastcall(boost::filesystem::path *, const char *, int, int, int, char, int, int, struct _Unwind_Exception *lpuexcpt)
// was: boost::filesystem::path *__fastcall(boost::filesystem::path *, const char *, int, int, int, char, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "boost::filesystem::path::operator/=(char const*)")]
pub fn stub_0x23a830() -> ! {
    todo!("0x23a830 __ZN5boost10filesystem4pathdVEPKc")
}

// 0x23a9d4 — __ZN5boost10filesystem4path27m_erase_redundant_separatorEm
// type: std::string *__fastcall(std::string *this, unsigned int)
#[doc(alias = "boost::filesystem::path::m_erase_redundant_separator(unsigned long)")]
pub fn stub_0x23a9d4() -> ! {
    todo!("0x23a9d4 __ZN5boost10filesystem4path27m_erase_redundant_separatorEm")
}

// 0x23aa2c — __ZN5boost10filesystem4path15remove_filenameEv
// type: boost::filesystem::path *__fastcall(boost::filesystem::path *this)
// was: boost::filesystem::path *__fastcall(boost::filesystem::path *this)
#[doc(alias = "boost::filesystem::path::remove_filename(void)")]
pub fn stub_0x23aa2c() -> ! {
    todo!("0x23aa2c __ZN5boost10filesystem4path15remove_filenameEv")
}

// 0x23aa60 — __ZNK5boost10filesystem4path17m_parent_path_endEv
// type: int __fastcall(boost::filesystem::path *this)
// was: int __fastcall(boost::filesystem::path *this)
#[doc(alias = "boost::filesystem::path::m_parent_path_end(void)const")]
pub fn stub_0x23aa60() -> ! {
    todo!("0x23aa60 __ZNK5boost10filesystem4path17m_parent_path_endEv")
}

// 0x23ab64 — __ZNK5boost10filesystem4path14root_directoryEv
// type: char *__fastcall(boost::filesystem::path *this, std::string *)
// was: char *__fastcall(boost::filesystem::path *this, std::string *)
#[doc(alias = "boost::filesystem::path::root_directory(void)const")]
pub fn stub_0x23ab64() -> ! {
    todo!("0x23ab64 __ZNK5boost10filesystem4path14root_directoryEv")
}

// 0x23abe8 — __ZNK5boost10filesystem4path11parent_pathEv
// type: char *__fastcall(boost::filesystem::path *this, boost::filesystem::path *)
// was: char *__fastcall(boost::filesystem::path *this, boost::filesystem::path *)
#[doc(alias = "boost::filesystem::path::parent_path(void)const")]
pub fn stub_0x23abe8() -> ! {
    todo!("0x23abe8 __ZNK5boost10filesystem4path11parent_pathEv")
}

// 0x23ac1c — __ZN5boost10filesystem4path7codecvtEv
// type: int __fastcall(boost::filesystem::path *this)
// was: int __fastcall(boost::filesystem::path *this)
#[doc(alias = "boost::filesystem::path::codecvt(void)")]
pub fn stub_0x23ac1c() -> ! {
    todo!("0x23ac1c __ZN5boost10filesystem4path7codecvtEv")
}

// 0x23ac2c — __ZN5boost10filesystem4pathD1Ev
// type: void __fastcall(boost::filesystem::path *__hidden this)
// was: void __fastcall(boost::filesystem::path *__hidden this)
#[doc(alias = "boost::filesystem::path::~path()")]
pub fn stub_0x23ac2c() -> ! {
    todo!("0x23ac2c __ZN5boost10filesystem4pathD1Ev")
}

// 0x23ac78 — __ZNSt6localeC2IN5boost10filesystem6detail18utf8_codecvt_facetEEERKS_PT_
// type: int __fastcall(int, const _Impl **, int, int, void *, int)
#[doc(alias = "std::locale::locale<boost::filesystem::detail::utf8_codecvt_facet>(std::locale const&,boost::filesystem::detail::utf8_codecvt_facet *)")]
pub fn stub_0x23ac78() -> ! {
    todo!("0x23ac78 __ZNSt6localeC2IN5boost10filesystem6detail18utf8_codecvt_facetEEERKS_PT_")
}

// 0x23adc4 — __ZN5boost10filesystem4pathC2IPKcEET_S5_
// type: std::string *__fastcall(std::string *, _BYTE *, _BYTE *)
#[doc(alias = "boost::filesystem::path::path<char const*>(char const*,char const*)")]
pub fn stub_0x23adc4() -> ! {
    todo!("0x23adc4 __ZN5boost10filesystem4pathC2IPKcEET_S5_")
}

// 0x23af94 — __ZNK5boost10filesystem6detail18utf8_codecvt_facet10do_unshiftER11__mbstate_tPcS5_RS5_
// type: int __fastcall(boost::filesystem::detail::utf8_codecvt_facet *this, __mbstate_t *, char *, char *, char **)
// was: int __fastcall(boost::filesystem::detail::utf8_codecvt_facet *this, __mbstate_t *, char *, char *, char **)
#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::do_unshift(__mbstate_t &,char *,char *,char *&)const")]
pub fn stub_0x23af94() -> ! {
    todo!("0x23af94 __ZNK5boost10filesystem6detail18utf8_codecvt_facet10do_unshiftER11__mbstate_tPcS5_RS5_")
}

// 0x23af9c — __ZNK5boost10filesystem6detail18utf8_codecvt_facet11do_encodingEv
// type: int __fastcall(boost::filesystem::detail::utf8_codecvt_facet *this)
// was: int __fastcall(boost::filesystem::detail::utf8_codecvt_facet *this)
#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::do_encoding(void)const")]
pub fn stub_0x23af9c() -> ! {
    todo!("0x23af9c __ZNK5boost10filesystem6detail18utf8_codecvt_facet11do_encodingEv")
}

// 0x23afa0 — __ZNK5boost10filesystem6detail18utf8_codecvt_facet16do_always_noconvEv
// type: int __fastcall(boost::filesystem::detail::utf8_codecvt_facet *this)
// was: int __fastcall(boost::filesystem::detail::utf8_codecvt_facet *this)
#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::do_always_noconv(void)const")]
pub fn stub_0x23afa0() -> ! {
    todo!("0x23afa0 __ZNK5boost10filesystem6detail18utf8_codecvt_facet16do_always_noconvEv")
}

// 0x23afa4 — __ZNK5boost10filesystem6detail18utf8_codecvt_facet13do_max_lengthEv
// type: int __fastcall(boost::filesystem::detail::utf8_codecvt_facet *this)
// was: int __fastcall(boost::filesystem::detail::utf8_codecvt_facet *this)
#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::do_max_length(void)const")]
pub fn stub_0x23afa4() -> ! {
    todo!("0x23afa4 __ZNK5boost10filesystem6detail18utf8_codecvt_facet13do_max_lengthEv")
}

// 0x23b14c — __ZNK5boost10filesystem6detail18utf8_codecvt_facet5do_inER11__mbstate_tPKcS6_RS6_PwS8_RS8_
// type: int __fastcall(int, int, char *, char *, char **, int *, int *, int **)
#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::do_in(__mbstate_t &,char const*,char const*,char const*&,wchar_t *,wchar_t *,wchar_t *&)const")]
pub fn stub_0x23b14c() -> ! {
    todo!("0x23b14c __ZNK5boost10filesystem6detail18utf8_codecvt_facet5do_inER11__mbstate_tPKcS6_RS6_PwS8_RS8_")
}

// 0x23b2d0 — __ZNK5boost10filesystem6detail18utf8_codecvt_facet6do_outER11__mbstate_tPKwS6_RS6_PcS8_RS8_
// type: bool __fastcall(int, int, _DWORD *, _DWORD *, _DWORD *, _BYTE *, _BYTE *, _DWORD *)
#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::do_out(__mbstate_t &,wchar_t const*,wchar_t const*,wchar_t const*&,char *,char *,char *&)const")]
pub fn stub_0x23b2d0() -> ! {
    todo!("0x23b2d0 __ZNK5boost10filesystem6detail18utf8_codecvt_facet6do_outER11__mbstate_tPKwS6_RS6_PcS8_RS8_")
}

// 0x23b43c — __ZNK5boost10filesystem6detail18utf8_codecvt_facet9do_lengthERK11__mbstate_tPKcS7_m
// type: int __fastcall(boost::filesystem::detail::utf8_codecvt_facet *this, const __mbstate_t *, const char *, const char *, unsigned int)
// was: int __fastcall(boost::filesystem::detail::utf8_codecvt_facet *this, const __mbstate_t *, const char *, const char *, unsigned int)
#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::do_length(__mbstate_t const&,char const*,char const*,unsigned long)const")]
pub fn stub_0x23b43c() -> ! {
    todo!("0x23b43c __ZNK5boost10filesystem6detail18utf8_codecvt_facet9do_lengthERK11__mbstate_tPKcS7_m")
}

// 0x23b4ac — __ZN5boost10filesystem6detail18utf8_codecvt_facetD1Ev
// type: void __fastcall(boost::filesystem::detail::utf8_codecvt_facet *__hidden this)
// was: void __fastcall(boost::filesystem::detail::utf8_codecvt_facet *__hidden this)
#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::~utf8_codecvt_facet()")]
pub fn stub_0x23b4ac() -> ! {
    todo!("0x23b4ac __ZN5boost10filesystem6detail18utf8_codecvt_facetD1Ev")
}

// 0x23b4b8 — __ZN5boost10filesystem6detail18utf8_codecvt_facetD0Ev
// type: void __fastcall(boost::filesystem::detail::utf8_codecvt_facet *__hidden this)
// was: void __fastcall(boost::filesystem::detail::utf8_codecvt_facet *__hidden this)
#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::~utf8_codecvt_facet()")]
pub fn stub_0x23b4b8() -> ! {
    todo!("0x23b4b8 __ZN5boost10filesystem6detail18utf8_codecvt_facetD0Ev")
}

// 0x23b4cc — __ZN5boost6system16generic_categoryEv
// type: int *__fastcall()
#[doc(alias = "boost::system::generic_category(void)")]
pub fn stub_0x23b4cc() -> ! {
    todo!("0x23b4cc __ZN5boost6system16generic_categoryEv")
}

// 0x23b508 — __ZN5boost6system15system_categoryEv
// type: int *__fastcall()
#[doc(alias = "boost::system::system_category(void)")]
pub fn stub_0x23b508() -> ! {
    todo!("0x23b508 __ZN5boost6system15system_categoryEv")
}

// 0x23ca3c — __ZNK5boost6system14error_category23default_error_conditionEi
// type: _QWORD *__fastcall(_QWORD *this, int, __int64)
#[doc(alias = "boost::system::error_category::default_error_condition(int)const")]
pub fn stub_0x23ca3c() -> ! {
    todo!("0x23ca3c __ZNK5boost6system14error_category23default_error_conditionEi")
}

// 0x23ca44 — __ZNK5boost6system14error_category10equivalentEiRKNS0_15error_conditionE
// type: bool __fastcall(int, int, _DWORD *)
#[doc(alias = "boost::system::error_category::equivalent(int,boost::system::error_condition const&)const")]
pub fn stub_0x23ca44() -> ! {
    todo!("0x23ca44 __ZNK5boost6system14error_category10equivalentEiRKNS0_15error_conditionE")
}

// 0x23ca70 — __ZNK5boost6system14error_category10equivalentERKNS0_10error_codeEi
// type: bool __fastcall(int, _DWORD *, int)
#[doc(alias = "boost::system::error_category::equivalent(boost::system::error_code const&,int)const")]
pub fn stub_0x23ca70() -> ! {
    todo!("0x23ca70 __ZNK5boost6system14error_category10equivalentERKNS0_10error_codeEi")
}

// 0x23cb64 — __ZN5boost9iostreams6detail11gzip_header7processEc
// type: void __fastcall(boost::iostreams::detail::gzip_header *this, unsigned __int8)
// was: void __fastcall(boost::iostreams::detail::gzip_header *this, unsigned __int8)
#[doc(alias = "boost::iostreams::detail::gzip_header::process(char)")]
pub fn stub_0x23cb64() -> ! {
    todo!("0x23cb64 __ZN5boost9iostreams6detail11gzip_header7processEc")
}

// 0x23cef0 — __ZN5boost9iostreams6detail11gzip_header5resetEv
// type: int __fastcall(boost::iostreams::detail::gzip_header *this)
// was: int __fastcall(boost::iostreams::detail::gzip_header *this)
#[doc(alias = "boost::iostreams::detail::gzip_header::reset(void)")]
pub fn stub_0x23cef0() -> ! {
    todo!("0x23cef0 __ZN5boost9iostreams6detail11gzip_header5resetEv")
}

// 0x23cf2c — __ZN5boost9iostreams6detail11gzip_footer7processEc
// type: _DWORD *__fastcall(_DWORD *this, unsigned __int8)
#[doc(alias = "boost::iostreams::detail::gzip_footer::process(char)")]
pub fn stub_0x23cf2c() -> ! {
    todo!("0x23cf2c __ZN5boost9iostreams6detail11gzip_footer7processEc")
}

// 0x23cf7c — __ZN5boost9iostreams6detail11gzip_footer5resetEv
// type: _DWORD *__fastcall(_DWORD *this)
#[doc(alias = "boost::iostreams::detail::gzip_footer::reset(void)")]
pub fn stub_0x23cf7c() -> ! {
    todo!("0x23cf7c __ZN5boost9iostreams6detail11gzip_footer5resetEv")
}

// 0x23cf8c — __ZN5boost9iostreams10zlib_error5checkEi
// type: void __fastcall(boost::iostreams::zlib_error *this, int)
// was: void __fastcall(boost::iostreams::zlib_error *this, int)
#[doc(alias = "boost::iostreams::zlib_error::check(int)")]
pub fn stub_0x23cf8c() -> ! {
    todo!("0x23cf8c __ZN5boost9iostreams10zlib_error5checkEi")
}

// 0x23d0c8 — __ZN5boost9iostreams6detail9zlib_baseC2Ev
// type: boost::iostreams::detail::zlib_base *__fastcall(boost::iostreams::detail::zlib_base *this)
// was: boost::iostreams::detail::zlib_base *__fastcall(boost::iostreams::detail::zlib_base *this)
#[doc(alias = "boost::iostreams::detail::zlib_base::zlib_base(void)")]
pub fn stub_0x23d0c8() -> ! {
    todo!("0x23d0c8 __ZN5boost9iostreams6detail9zlib_baseC2Ev")
}

// 0x23d0e8 — __ZN5boost9iostreams6detail9zlib_baseD2Ev
// type: void __fastcall(void **this)
#[doc(alias = "boost::iostreams::detail::zlib_base::~zlib_base()")]
pub fn stub_0x23d0e8() -> ! {
    todo!("0x23d0e8 __ZN5boost9iostreams6detail9zlib_baseD2Ev")
}

// 0x23d0fc — __ZN5boost9iostreams6detail9zlib_base6beforeERPKcS4_RPcS6_
// type: int __fastcall(boost::iostreams::detail::zlib_base *this, const char **, const char *, char **, char *)
// was: int __fastcall(boost::iostreams::detail::zlib_base *this, const char **, const char *, char **, char *)
#[doc(alias = "boost::iostreams::detail::zlib_base::before(char const*&,char const*,char *&,char *)")]
pub fn stub_0x23d0fc() -> ! {
    todo!("0x23d0fc __ZN5boost9iostreams6detail9zlib_base6beforeERPKcS4_RPcS6_")
}

// 0x23d120 — __ZN5boost9iostreams6detail9zlib_base5afterERPKcRPcb
// type: const char *__fastcall(boost::iostreams::detail::zlib_base *this, const char **, char **, int)
// was: const char *__fastcall(boost::iostreams::detail::zlib_base *this, const char **, char **, int)
#[doc(alias = "boost::iostreams::detail::zlib_base::after(char const*&,char *&,bool)")]
pub fn stub_0x23d120() -> ! {
    todo!("0x23d120 __ZN5boost9iostreams6detail9zlib_base5afterERPKcRPcb")
}

// 0x23d180 — __ZN5boost9iostreams6detail9zlib_base8xdeflateEi
// type: int __fastcall(z_streamp *this, int)
#[doc(alias = "boost::iostreams::detail::zlib_base::xdeflate(int)")]
pub fn stub_0x23d180() -> ! {
    todo!("0x23d180 __ZN5boost9iostreams6detail9zlib_base8xdeflateEi")
}

// 0x23d18c — __ZN5boost9iostreams6detail9zlib_base8xinflateEi
// type: int __fastcall(z_streamp *this, int)
#[doc(alias = "boost::iostreams::detail::zlib_base::xinflate(int)")]
pub fn stub_0x23d18c() -> ! {
    todo!("0x23d18c __ZN5boost9iostreams6detail9zlib_base8xinflateEi")
}

// 0x23d198 — __ZN5boost9iostreams6detail9zlib_base5resetEbb
// type: int __fastcall(z_stream **this, int, int)
#[doc(alias = "boost::iostreams::detail::zlib_base::reset(bool,bool)")]
pub fn stub_0x23d198() -> ! {
    todo!("0x23d198 __ZN5boost9iostreams6detail9zlib_base5resetEbb")
}

// 0x23d1c8 — __ZN5boost9iostreams6detail9zlib_base7do_initERKNS0_11zlib_paramsEbPFPvS6_jjEPFvS6_S6_ES6_
// type: void __fastcall(int, int, int, int, int, void *)
#[doc(alias = "boost::iostreams::detail::zlib_base::do_init(boost::iostreams::zlib_params const&,bool,void * (*)(void *,unsigned int,unsigned int),void (*)(void *,void *),void *)")]
pub fn stub_0x23d1c8() -> ! {
    todo!("0x23d1c8 __ZN5boost9iostreams6detail9zlib_base7do_initERKNS0_11zlib_paramsEbPFPvS6_jjEPFvS6_S6_ES6_")
}

// 0x23d238 — __ZN5boost15throw_exceptionINS_9iostreams10zlib_errorEEEvRKT_
// type: void __fastcall __noreturn(int)
#[doc(alias = "void boost::throw_exception<boost::iostreams::zlib_error>(boost::iostreams::zlib_error const&)")]
pub fn stub_0x23d238() -> ! {
    todo!("0x23d238 __ZN5boost15throw_exceptionINS_9iostreams10zlib_errorEEEvRKT_")
}

// 0x23d390 — __ZN5boost9iostreams10zlib_errorD1Ev
// type: void __fastcall(boost::iostreams::zlib_error *__hidden this)
// was: void __fastcall(boost::iostreams::zlib_error *__hidden this)
#[doc(alias = "boost::iostreams::zlib_error::~zlib_error()")]
pub fn stub_0x23d390() -> ! {
    todo!("0x23d390 __ZN5boost9iostreams10zlib_errorD1Ev")
}

// 0x23d39c — __ZN5boost9iostreams10zlib_errorD0Ev
// type: void __fastcall(boost::iostreams::zlib_error *__hidden this)
// was: void __fastcall(boost::iostreams::zlib_error *__hidden this)
#[doc(alias = "boost::iostreams::zlib_error::~zlib_error()")]
pub fn stub_0x23d39c() -> ! {
    todo!("0x23d39c __ZN5boost9iostreams10zlib_errorD0Ev")
}

// 0x23d3b0 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED1Ev
// type: std::ios_base::failure *__fastcall(std::ios_base::failure *)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl()")]
pub fn stub_0x23d3b0() -> ! {
    todo!("0x23d3b0 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED1Ev")
}

// 0x23d468 — __ZN5boost16exception_detail19error_info_injectorINS_9iostreams10zlib_errorEED1Ev
// type: int __fastcall(std::ios_base::failure *)
#[doc(alias = "boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>::~error_info_injector()")]
pub fn stub_0x23d468() -> ! {
    todo!("0x23d468 __ZN5boost16exception_detail19error_info_injectorINS_9iostreams10zlib_errorEED1Ev")
}

// 0x23d520 — __ZThn12_N5boost16exception_detail19error_info_injectorINS_9iostreams10zlib_errorEED1Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::iostreams::zlib_error>::~error_info_injector()")]
pub fn stub_0x23d520() -> ! {
    todo!("0x23d520 __ZThn12_N5boost16exception_detail19error_info_injectorINS_9iostreams10zlib_errorEED1Ev")
}

// 0x23d5d8 — __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED1Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl()")]
pub fn stub_0x23d5d8() -> ! {
    todo!("0x23d5d8 __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED1Ev")
}

// 0x23d690 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED1Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl()")]
pub fn stub_0x23d690() -> ! {
    todo!("0x23d690 __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED1Ev")
}

// 0x23d75c — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED0Ev
// type: void __fastcall(std::ios_base::failure *)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl()")]
pub fn stub_0x23d75c() -> ! {
    todo!("0x23d75c __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED0Ev")
}

// 0x23d818 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEE5cloneEv
// type: char *__fastcall(int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone(void)const")]
pub fn stub_0x23d818() -> ! {
    todo!("0x23d818 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEE5cloneEv")
}

// 0x23d8d4 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEE7rethrowEv
// type: void __fastcall __noreturn(int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::rethrow(void)const")]
pub fn stub_0x23d8d4() -> ! {
    todo!("0x23d8d4 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEE7rethrowEv")
}

// 0x23d984 — __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl()")]
pub fn stub_0x23d984() -> ! {
    todo!("0x23d984 __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED0Ev")
}

// 0x23da40 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEE5cloneEv
// type: char *__fastcall(_DWORD *)
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone(void)const")]
pub fn stub_0x23da40() -> ! {
    todo!("0x23da40 __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEE5cloneEv")
}

// 0x23db04 — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEE7rethrowEv
// type: void __fastcall __noreturn(_DWORD *)
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::rethrow(void)const")]
pub fn stub_0x23db04() -> ! {
    todo!("0x23db04 __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEE7rethrowEv")
}

// 0x23db14 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl()")]
pub fn stub_0x23db14() -> ! {
    todo!("0x23db14 __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED0Ev")
}

// 0x23dbe8 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEEC1ERKS6_
// type: int __fastcall(int, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>> const&)")]
pub fn stub_0x23dbe8() -> ! {
    todo!("0x23dbe8 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEEC1ERKS6_")
}

// 0x23dd30 — __ZN5boost16exception_detail19error_info_injectorINS_9iostreams10zlib_errorEED0Ev
// type: void __fastcall(std::ios_base::failure *)
#[doc(alias = "boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>::~error_info_injector()")]
pub fn stub_0x23dd30() -> ! {
    todo!("0x23dd30 __ZN5boost16exception_detail19error_info_injectorINS_9iostreams10zlib_errorEED0Ev")
}

// 0x23ddec — __ZThn12_N5boost16exception_detail19error_info_injectorINS_9iostreams10zlib_errorEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::iostreams::zlib_error>::~error_info_injector()")]
pub fn stub_0x23ddec() -> ! {
    todo!("0x23ddec __ZThn12_N5boost16exception_detail19error_info_injectorINS_9iostreams10zlib_errorEED0Ev")
}

// 0x23dea8 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEEC1ERKS6_NS6_9clone_tagE
// type: int __fastcall(int, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone_tag)")]
pub fn stub_0x23dea8() -> ! {
    todo!("0x23dea8 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEEC1ERKS6_NS6_9clone_tagE")
}

// 0x23e044 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEEC1ERKS5_
// type: int __fastcall(int, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone_impl(boost::exception_detail::error_info_injector<boost::iostreams::zlib_error> const&)")]
pub fn stub_0x23e044() -> ! {
    todo!("0x23e044 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEEC1ERKS5_")
}

// 0x23e52c — __ZN3RBX21trim_trailing_slashesERKSs
// type: int __fastcall(RBX *this, const std::string *)
#[doc(alias = "RBX::trim_trailing_slashes(std::string const&)")]
pub fn stub_0x23e52c() -> ! {
    todo!("0x23e52c __ZN3RBX21trim_trailing_slashesERKSs")
}

// 0x23e5f8 — __ZN3RBX9Debugable4dumpERSo
// type: int __fastcall(RBX::Debugable *this, std::ostream *)
#[doc(alias = "RBX::Debugable::dump(std::ostream &)")]
pub fn stub_0x23e5f8() -> ! {
    todo!("0x23e5f8 __ZN3RBX9Debugable4dumpERSo")
}

// 0x23e678 — __ZN3RBX3Log9timeStampERSt14basic_ofstreamIcSt11char_traitsIcEEb
// type: int __fastcall(std::ostream *, int)
#[doc(alias = "RBX::Log::timeStamp(std::basic_ofstream<char,std::char_traits<char>> &,bool)")]
pub fn stub_0x23e678() -> ! {
    todo!("0x23e678 __ZN3RBX3Log9timeStampERSt14basic_ofstreamIcSt11char_traitsIcEEb")
}

// 0x23ec04 — __ZN5boost9date_time23gregorian_calendar_baseINS0_19year_month_day_baseINS_9gregorian9greg_yearENS3_10greg_monthENS3_8greg_dayEEEjE15from_day_numberEj
// type: _WORD *__fastcall(_WORD *result, int)
#[doc(alias = "boost::date_time::gregorian_calendar_base<boost::date_time::year_month_day_base<boost::gregorian::greg_year,boost::gregorian::greg_month,boost::gregorian::greg_day>,unsigned int>::from_day_number(unsigned int)")]
pub fn stub_0x23ec04() -> ! {
    todo!("0x23ec04 __ZN5boost9date_time23gregorian_calendar_baseINS0_19year_month_day_baseINS_9gregorian9greg_yearENS3_10greg_monthENS3_8greg_dayEEEjE15from_day_numberEj")
}

// 0x23ecfc — __ZN5boost9date_time12second_clockINS_10posix_time5ptimeEE11create_timeEP2tm
// type: int __fastcall(_DWORD *, __int64 *)
#[doc(alias = "boost::date_time::second_clock<boost::posix_time::ptime>::create_time(tm *)")]
pub fn stub_0x23ecfc() -> ! {
    todo!("0x23ecfc __ZN5boost9date_time12second_clockINS_10posix_time5ptimeEE11create_timeEP2tm")
}

// 0x23ef20 — __ZNK5boost9date_time16counted_time_repINS_10posix_time33millisec_posix_time_system_configEE4dateEv
// type: unsigned int __fastcall(__int64 *)
#[doc(alias = "boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config>::date(void)const")]
pub fn stub_0x23ef20() -> ! {
    todo!("0x23ef20 __ZNK5boost9date_time16counted_time_repINS_10posix_time33millisec_posix_time_system_configEE4dateEv")
}

// 0x23f2ac — __ZN3RBX12boost_detail8init_fooEv
// type: void __fastcall(RBX::boost_detail *this)
// was: void __fastcall(RBX::boost_detail *this)
#[doc(alias = "RBX::boost_detail::init_foo(void)")]
pub fn stub_0x23f2ac() -> ! {
    todo!("0x23f2ac __ZN3RBX12boost_detail8init_fooEv")
}

// 0x23f50c — __ZN3RBX14thread_wrapperERKN5boost9function0IvEEPKc
// type: void __fastcall(_DWORD *, int *, int)
#[doc(alias = "RBX::thread_wrapper(boost::function0<void> const&,char const*)")]
pub fn stub_0x23f50c() -> ! {
    todo!("0x23f50c __ZN3RBX14thread_wrapperERKN5boost9function0IvEEPKc")
}
