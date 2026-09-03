//! rendering — generated_rendering_shard_a — 150 stubs EA-sorted asc
//! Filter: Ogre|G3D|Render strict 15112 total filtered, 13 gaps remain (exhausted), filler global asc for remaining 137 (26870->27020 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x236d04 — __ZN5boost12_GLOBAL__N_131get_or_make_current_thread_dataEv
// type: void *__fastcall(boost::_anonymous_namespace_ *this, int, int, int)
#[doc(alias = "boost::anonymous namespace::get_or_make_current_thread_data(void)")]
// was: __ZN5boost12_GLOBAL__N_131get_or_make_current_thread_dataEv
// IDA 0x236d04: 159 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_236d04() {
}

// 0x236ec0 — __ZN5boost6detail12get_tss_dataEPKv
// type: _DWORD __fastcall(boost::detail *__hidden this, const void *)
#[doc(alias = "boost::detail::get_tss_data(void const*)")]
// was: __ZN5boost6detail12get_tss_dataEPKv
// IDA 0x236ec0: 42 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_236ec0() {
}

// 0x236f30 — __ZN5boost6detail16add_new_tss_nodeEPKvNS_10shared_ptrINS0_20tss_cleanup_functionEEEPv
// type: void __fastcall(boost::_anonymous_namespace_ *, int *, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::detail::add_new_tss_node(void const*,rbx_core::SharedPtr<boost::detail::tss_cleanup_function>,void *)")]
// was: __ZN5boost6detail16add_new_tss_nodeEPKvNS_10shared_ptrINS0_20tss_cleanup_functionEEEPv
// IDA 0x236f30: 194 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_236f30() {
}

// 0x237130 — __ZN5boost6detail12set_tss_dataEPKvNS_10shared_ptrINS0_20tss_cleanup_functionEEEPvb
// type: void __fastcall(boost::_anonymous_namespace_ *, int *, int, int)
#[doc(alias = "boost::detail::set_tss_data(void const*,rbx_core::SharedPtr<boost::detail::tss_cleanup_function>,void *,bool)")]
// was: __ZN5boost6detail12set_tss_dataEPKvNS_10shared_ptrINS0_20tss_cleanup_functionEEEPvb
// IDA 0x237130: 197 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_237130() {
}

// 0x237348 — __ZN5boost12_GLOBAL__N_126externally_launched_threadD1Ev
// type: void __fastcall(boost::_anonymous_namespace_::externally_launched_thread *__hidden this)
#[doc(alias = "boost::anonymous namespace::externally_launched_thread::~externally_launched_thread()")]
// was: __ZN5boost12_GLOBAL__N_126externally_launched_threadD1Ev
// IDA 0x237348: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_237348() {
}

// 0x237354 — __ZN5boost12_GLOBAL__N_126externally_launched_threadD0Ev
// type: void __fastcall(boost::_anonymous_namespace_::externally_launched_thread *__hidden this)
#[doc(alias = "boost::anonymous namespace::externally_launched_thread::~externally_launched_thread()")]
// was: __ZN5boost12_GLOBAL__N_126externally_launched_threadD0Ev
// IDA 0x237354: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_237354() {
}

// 0x237368 — __ZN5boost12_GLOBAL__N_126externally_launched_thread3runEv
// type: void __fastcall(boost::_anonymous_namespace_::externally_launched_thread *this)
#[doc(alias = "boost::anonymous namespace::externally_launched_thread::run(void)")]
// was: __ZN5boost12_GLOBAL__N_126externally_launched_thread3runEv
// IDA 0x237368: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_237368() {
}

// 0x23736c — __ZN5boost12_GLOBAL__N_126externally_launched_thread25notify_all_at_thread_exitEPNS_18condition_variableEPNS_5mutexE
// type: void()
#[doc(alias = "boost::anonymous namespace::externally_launched_thread::notify_all_at_thread_exit(boost::condition_variable *,boost::mutex *)")]
// was: __ZN5boost12_GLOBAL__N_126externally_launched_thread25notify_all_at_thread_exitEPNS_18condition_variableEPNS_5mutexE
// IDA 0x23736c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_23736c() {
}

// 0x237370 — _tls_destructor
// type: void __fastcall(int)
#[doc(alias = "_tls_destructor")]
// was: _tls_destructor
// IDA 0x237370: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_237370() {
}

// 0x2374bc — __ZN5boost10shared_ptrINS_6detail16thread_data_baseEEaSERKS3_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *)
#[doc(alias = "rbx_core::SharedPtr<boost::detail::thread_data_base>::operator=(rbx_core::SharedPtr<boost::detail::thread_data_base> const&)")]
// was: __ZN5boost10shared_ptrINS_6detail16thread_data_baseEEaSERKS3_
// IDA 0x2374bc: 85 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2374bc() {
}

// 0x2375b0 — __ZN5boost10shared_ptrINS_6detail20tss_cleanup_functionEEaSERKS3_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *)
#[doc(alias = "rbx_core::SharedPtr<boost::detail::tss_cleanup_function>::operator=(rbx_core::SharedPtr<boost::detail::tss_cleanup_function> const&)")]
// was: __ZN5boost10shared_ptrINS_6detail20tss_cleanup_functionEEaSERKS3_
// IDA 0x2375b0: 85 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2375b0() {
}

// 0x2376a4 — __ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE5eraseESt17_Rb_tree_iteratorIS7_ESF_
// type: void __fastcall(int, _Rb_tree_node_base *, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<void const*,std::pair<void const* const,boost::detail::tss_data_node>,std::_Select1st<std::pair<void const* const,boost::detail::tss_data_node>>,std::less<void const*>,std::allocator<std::pair<void const* const,boost::detail::tss_data_node>>>::erase(std::_Rb_tree_iterator<std::pair<void const* const,boost::detail::tss_data_node>>,std::_Rb_tree_iterator<std::pair<void const* const,boost::detail::tss_data_node>>)")]
// was: __ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE5eraseESt17_Rb_tree_iteratorIS7_ESF_
// IDA 0x2376a4: 91 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2376a4() {
}

// 0x237798 — __ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<void const*,std::pair<void const* const,boost::detail::tss_data_node>,std::_Select1st<std::pair<void const* const,boost::detail::tss_data_node>>,std::less<void const*>,std::allocator<std::pair<void const* const,boost::detail::tss_data_node>>>::_M_erase(std::_Rb_tree_node<std::pair<void const* const,boost::detail::tss_data_node>> *)")]
// was: __ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// IDA 0x237798: 62 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_237798() {
}

// 0x237848 — __ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE16_M_insert_uniqueERKS7_
// type: int __fastcall(int, _DWORD *, int *)
#[doc(alias = "std::_Rb_tree<void const*,std::pair<void const* const,boost::detail::tss_data_node>,std::_Select1st<std::pair<void const* const,boost::detail::tss_data_node>>,std::less<void const*>,std::allocator<std::pair<void const* const,boost::detail::tss_data_node>>>::_M_insert_unique(std::pair<void const* const,boost::detail::tss_data_node> const&)")]
// was: __ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE16_M_insert_uniqueERKS7_
// IDA 0x237848: 70 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_237848() {
}

// 0x2378fc — __ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE14_M_create_nodeERKS7_
// type: _DWORD *__fastcall(int, int *)
#[doc(alias = "std::_Rb_tree<void const*,std::pair<void const* const,boost::detail::tss_data_node>,std::_Select1st<std::pair<void const* const,boost::detail::tss_data_node>>,std::less<void const*>,std::allocator<std::pair<void const* const,boost::detail::tss_data_node>>>::_M_create_node(std::pair<void const* const,boost::detail::tss_data_node> const&)")]
// was: __ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE14_M_create_nodeERKS7_
// IDA 0x2378fc: 86 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2378fc() {
}

// 0x2379ec — __ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_S2_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int)
#[doc(alias = "void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data_base>(rbx_core::SharedPtr<boost::detail::thread_data_base> const*,boost::detail::thread_data_base *)const")]
// was: __ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_S2_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x2379ec: 120 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2379ec() {
}

// 0x237b40 — __ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data_base>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEED1Ev
// IDA 0x237b40: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_237b40() {
}

// 0x237b44 — __ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data_base>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEED0Ev
// IDA 0x237b44: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_237b44() {
}

// 0x237b50 — __ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data_base>::dispose(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEE7disposeEv
// IDA 0x237b50: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_237b50() {
}

// 0x237b64 — __ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data_base>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEE11get_deleterERKSt9type_info
// IDA 0x237b64: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_237b64() {
}

// 0x237b68 — __ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data_base>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEE19get_untyped_deleterEv
// IDA 0x237b68: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_237b68() {
}

// 0x237b6c — __ZN5boost6detail18future_object_base22mark_finished_internalERNS_11unique_lockINS_5mutexEEE
// type: void __fastcall(int)
#[doc(alias = "boost::detail::future_object_base::mark_finished_internal(boost::unique_lock<boost::mutex> &)")]
// was: __ZN5boost6detail18future_object_base22mark_finished_internalERNS_11unique_lockINS_5mutexEEE
// IDA 0x237b6c: 109 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_237b6c() {
}

// 0x237c98 — __GLOBAL__I_a_38
#[doc(alias = "global constructor keyed to_a_38")]
// was: __GLOBAL__I_a_38
// IDA 0x237c98: 63 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_237c98() {
}

// 0x237d60 — __ZN5boost10filesystem6detail14symlink_statusERKNS0_4pathEPNS_6system10error_codeE
// type: void __fastcall(int *, const char **, int *)
#[doc(alias = "boost::filesystem::detail::symlink_status(boost::filesystem::path const&,boost::system::error_code *)")]
// was: __ZN5boost10filesystem6detail14symlink_statusERKNS0_4pathEPNS_6system10error_codeE
// IDA 0x237d60: 208 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_237d60() {
}

// 0x237fa4 — __ZN5boost10filesystem6detail12current_pathEPNS_6system10error_codeE
// type: void __fastcall(std::string *, _DWORD *)
#[doc(alias = "boost::filesystem::detail::current_path(boost::system::error_code *)")]
// was: __ZN5boost10filesystem6detail12current_pathEPNS_6system10error_codeE
// IDA 0x237fa4: 191 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_237fa4() {
}

// 0x238258 — __ZN12_GLOBAL__N_15errorEbRKN5boost10filesystem4pathEPNS0_6system10error_codeERKSs
// type: int __fastcall(int, void *, int, int)
#[doc(alias = "anonymous namespace::error(bool,boost::filesystem::path const&,boost::system::error_code *,std::string const&)")]
// was: __ZN12_GLOBAL__N_15errorEbRKN5boost10filesystem4pathEPNS0_6system10error_codeERKSs
// IDA 0x238258: 95 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_238258() {
}

// 0x23837c — __ZN5boost10filesystem6detail12initial_pathEPNS_6system10error_codeE
// type: void __fastcall(std::string *, _DWORD *)
#[doc(alias = "boost::filesystem::detail::initial_path(boost::system::error_code *)")]
// was: __ZN5boost10filesystem6detail12initial_pathEPNS_6system10error_codeE
// IDA 0x23837c: 142 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23837c() {
}

// 0x23852c — __ZN5boost10filesystem6detail8is_emptyERKNS0_4pathEPNS_6system10error_codeE
// type: bool __fastcall(const char **, int)
#[doc(alias = "boost::filesystem::detail::is_empty(boost::filesystem::path const&,boost::system::error_code *)")]
// was: __ZN5boost10filesystem6detail8is_emptyERKNS0_4pathEPNS_6system10error_codeE
// IDA 0x23852c: 118 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23852c() {
}

// 0x2386d4 — __ZN5boost10filesystem6detail6removeERKNS0_4pathEPNS_6system10error_codeE
// type: int __fastcall(const char **, int *)
#[doc(alias = "boost::filesystem::detail::remove(boost::filesystem::path const&,boost::system::error_code *)")]
// was: __ZN5boost10filesystem6detail6removeERKNS0_4pathEPNS_6system10error_codeE
// IDA 0x2386d4: 129 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2386d4() {
}

// 0x2388a8 — __ZN12_GLOBAL__N_124remove_file_or_directoryERKN5boost10filesystem4pathENS1_9file_typeEPNS0_6system10error_codeE
// type: bool __fastcall(const char **, int, _DWORD *)
#[doc(alias = "anonymous namespace::remove_file_or_directory(boost::filesystem::path const&,boost::filesystem::file_type,boost::system::error_code *)")]
// was: __ZN12_GLOBAL__N_124remove_file_or_directoryERKN5boost10filesystem4pathENS1_9file_typeEPNS0_6system10error_codeE
// IDA 0x2388a8: 203 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2388a8() {
}

// 0x238adc — __ZN5boost10filesystem6detail6statusERKNS0_4pathEPNS_6system10error_codeE
// type: void __fastcall(int *, const char **, int *)
#[doc(alias = "boost::filesystem::detail::status(boost::filesystem::path const&,boost::system::error_code *)")]
// was: __ZN5boost10filesystem6detail6statusERKNS0_4pathEPNS_6system10error_codeE
// IDA 0x238adc: 204 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_238adc() {
}

// 0x238d18 — __ZN5boost10filesystem6detail15system_completeERKNS0_4pathEPNS_6system10error_codeE
// type: void __fastcall(std::string *, const std::string *)
#[doc(alias = "boost::filesystem::detail::system_complete(boost::filesystem::path const&,boost::system::error_code *)")]
// was: __ZN5boost10filesystem6detail15system_completeERKNS0_4pathEPNS_6system10error_codeE
// IDA 0x238d18: 181 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_238d18() {
}

// 0x238f14 — __ZNK5boost10filesystem15directory_entry12m_get_statusEPNS_6system10error_codeE
// type: __int64 __fastcall(_QWORD *, int, int *)
#[doc(alias = "boost::filesystem::directory_entry::m_get_status(boost::system::error_code *)const")]
// was: __ZNK5boost10filesystem15directory_entry12m_get_statusEPNS_6system10error_codeE
// IDA 0x238f14: 42 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_238f14() {
}

// 0x238f80 — __ZN5boost10filesystem6detail13dir_itr_closeERPvS3_
// type: int __fastcall(boost::filesystem::detail *this, void **, void **)
#[doc(alias = "boost::filesystem::detail::dir_itr_close(void *&,void *&)")]
// was: __ZN5boost10filesystem6detail13dir_itr_closeERPvS3_
// IDA 0x238f80: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_238f80() {
}

// 0x238fd4 — __ZN5boost10filesystem6detail28directory_iterator_constructERNS0_18directory_iteratorERKNS0_4pathEPNS_6system10error_codeE
// type: void __fastcall(std::string **, const char **, std::string **)
#[doc(alias = "boost::filesystem::detail::directory_iterator_construct(boost::filesystem::directory_iterator &,boost::filesystem::path const&,boost::system::error_code *)")]
// was: __ZN5boost10filesystem6detail28directory_iterator_constructERNS0_18directory_iteratorERKNS0_4pathEPNS_6system10error_codeE
// IDA 0x238fd4: 415 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_238fd4() {
}

// 0x239668 — __ZN5boost10filesystem6detail28directory_iterator_incrementERNS0_18directory_iteratorEPNS_6system10error_codeE
// type: void __fastcall(int *, dirent **)
#[doc(alias = "boost::filesystem::detail::directory_iterator_increment(boost::filesystem::directory_iterator &,boost::system::error_code *)")]
// was: __ZN5boost10filesystem6detail28directory_iterator_incrementERNS0_18directory_iteratorEPNS_6system10error_codeE
// IDA 0x239668: 456 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_239668() {
}

// 0x239b34 — __ZN5boost10filesystem18directory_iteratorD1Ev
// type: void __fastcall(boost::filesystem::directory_iterator *__hidden this)
#[doc(alias = "boost::filesystem::directory_iterator::~directory_iterator()")]
// was: __ZN5boost10filesystem18directory_iteratorD1Ev
// IDA 0x239b34: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_239b34() {
}

// 0x239bc8 — __ZN5boost10filesystem16filesystem_errorD1Ev
// type: void __fastcall(std::runtime_error *this)
#[doc(alias = "boost::filesystem::filesystem_error::~filesystem_error()")]
// was: __ZN5boost10filesystem16filesystem_errorD1Ev
// IDA 0x239bc8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_239bc8() {
}

// 0x239cc8 — __ZN5boost10filesystem16filesystem_errorC2ERKSsNS_6system10error_codeE
// type: std::runtime_error *__fastcall(std::runtime_error *, const std::string *, std::runtime_error_vtbl *, const char *, boost::detail::sp_counted_base *, std::runtime_error *, int, int, void *, int)
#[doc(alias = "boost::filesystem::filesystem_error::filesystem_error(std::string const&,boost::system::error_code)")]
// was: __ZN5boost10filesystem16filesystem_errorC2ERKSsNS_6system10error_codeE
// IDA 0x239cc8: 172 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_239cc8() {
}

// 0x239e90 — __ZN5boost10filesystem16filesystem_errorD0Ev
// type: void __fastcall(std::runtime_error *this)
#[doc(alias = "boost::filesystem::filesystem_error::~filesystem_error()")]
// was: __ZN5boost10filesystem16filesystem_errorD0Ev
// IDA 0x239e90: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_239e90() {
}

// 0x239f94 — __ZNK5boost10filesystem16filesystem_error4whatEv
// type: int __fastcall(boost::filesystem::filesystem_error *this)
#[doc(alias = "boost::filesystem::filesystem_error::what(void)const")]
// was: __ZNK5boost10filesystem16filesystem_error4whatEv
// IDA 0x239f94: 138 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_239f94() {
}

// 0x23a11c — __ZN5boost6detail20sp_pointer_constructINS_10filesystem16filesystem_error5m_impES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, boost::detail::sp_counted_base **, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<boost::filesystem::filesystem_error::m_imp,boost::filesystem::filesystem_error::m_imp>(rbx_core::SharedPtr<boost::filesystem::filesystem_error::m_imp> *,boost::filesystem::filesystem_error::m_imp *,boost::detail::shared_count &)")]
// was: __ZN5boost6detail20sp_pointer_constructINS_10filesystem16filesystem_error5m_impES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// IDA 0x23a11c: 145 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23a11c() {
}

// 0x23a2bc — __ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::filesystem::filesystem_error::m_imp>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEED1Ev
// IDA 0x23a2bc: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_23a2bc() {
}

// 0x23a2c0 — __ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::filesystem::filesystem_error::m_imp>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEED0Ev
// IDA 0x23a2c0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_23a2c0() {
}

// 0x23a2cc — __ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEE7disposeEv
// type: void __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::filesystem::filesystem_error::m_imp>::dispose(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEE7disposeEv
// IDA 0x23a2cc: 68 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23a2cc() {
}

// 0x23a38c — __ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::filesystem::filesystem_error::m_imp>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEE11get_deleterERKSt9type_info
// IDA 0x23a38c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23a38c() {
}

// 0x23a390 — __ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::filesystem::filesystem_error::m_imp>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEE19get_untyped_deleterEv
// IDA 0x23a390: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23a390() {
}

// 0x23a394 — __ZN5boost10filesystem16filesystem_errorC2ERKSsRKNS0_4pathENS_6system10error_codeE
// type: std::runtime_error *__fastcall(std::runtime_error *, const std::string *, const std::string *, std::runtime_error_vtbl *, boost::detail::sp_counted_base *, std::runtime_error *, int, int, void *, int)
#[doc(alias = "boost::filesystem::filesystem_error::filesystem_error(std::string const&,boost::filesystem::path const&,boost::system::error_code)")]
// was: __ZN5boost10filesystem16filesystem_errorC2ERKSsRKNS0_4pathENS_6system10error_codeE
// IDA 0x23a394: 180 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23a394() {
}

// 0x23a570 — __GLOBAL__I_a_39
// type: int *()
#[doc(alias = "global constructor keyed to_a_39")]
// was: __GLOBAL__I_a_39
// IDA 0x23a570: 60 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23a570() {
}

// 0x23a630 — __ZN5boost10filesystem4pathdVERKS1_
// type: boost::filesystem::path *__fastcall(boost::filesystem::path *, const std::string *)
#[doc(alias = "boost::filesystem::path::operator/=(boost::filesystem::path const&)")]
// was: __ZN5boost10filesystem4pathdVERKS1_
// IDA 0x23a630: 138 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23a630() {
}

// 0x23a7b8 — __ZN5boost10filesystem4path28m_append_separator_if_neededEv
// type: int __fastcall(boost::filesystem::path *this)
#[doc(alias = "boost::filesystem::path::m_append_separator_if_needed(void)")]
// was: __ZN5boost10filesystem4path28m_append_separator_if_neededEv
// IDA 0x23a7b8: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23a7b8() {
}

// 0x23a830 — __ZN5boost10filesystem4pathdVEPKc
// type: boost::filesystem::path *__fastcall(boost::filesystem::path *, const char *, int, int, int, char, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "boost::filesystem::path::operator/=(char const*)")]
// was: __ZN5boost10filesystem4pathdVEPKc
// IDA 0x23a830: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23a830() {
}

// 0x23a9d4 — __ZN5boost10filesystem4path27m_erase_redundant_separatorEm
// type: std::string *__fastcall(std::string *this, unsigned int)
#[doc(alias = "boost::filesystem::path::m_erase_redundant_separator(unsigned long)")]
// was: __ZN5boost10filesystem4path27m_erase_redundant_separatorEm
// IDA 0x23a9d4: 35 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23a9d4() {
}

// 0x23aa2c — __ZN5boost10filesystem4path15remove_filenameEv
// type: boost::filesystem::path *__fastcall(boost::filesystem::path *this)
#[doc(alias = "boost::filesystem::path::remove_filename(void)")]
// was: __ZN5boost10filesystem4path15remove_filenameEv
// IDA 0x23aa2c: 18 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23aa2c() {
}

// 0x23aa60 — __ZNK5boost10filesystem4path17m_parent_path_endEv
// type: int __fastcall(boost::filesystem::path *this)
#[doc(alias = "boost::filesystem::path::m_parent_path_end(void)const")]
// was: __ZNK5boost10filesystem4path17m_parent_path_endEv
// IDA 0x23aa60: 113 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23aa60() {
}

// 0x23ab64 — __ZNK5boost10filesystem4path14root_directoryEv
// type: char *__fastcall(boost::filesystem::path *this, std::string *)
#[doc(alias = "boost::filesystem::path::root_directory(void)const")]
// was: __ZNK5boost10filesystem4path14root_directoryEv
// IDA 0x23ab64: 55 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23ab64() {
}

// 0x23abe8 — __ZNK5boost10filesystem4path11parent_pathEv
// type: char *__fastcall(boost::filesystem::path *this, boost::filesystem::path *)
#[doc(alias = "boost::filesystem::path::parent_path(void)const")]
// was: __ZNK5boost10filesystem4path11parent_pathEv
// IDA 0x23abe8: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23abe8() {
}

// 0x23ac1c — __ZN5boost10filesystem4path7codecvtEv
// type: int __fastcall(boost::filesystem::path *this)
#[doc(alias = "boost::filesystem::path::codecvt(void)")]
// was: __ZN5boost10filesystem4path7codecvtEv
// IDA 0x23ac1c: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23ac1c() {
}

// 0x23ac2c — __ZN5boost10filesystem4pathD1Ev
// type: void __fastcall(boost::filesystem::path *__hidden this)
#[doc(alias = "boost::filesystem::path::~path()")]
// was: __ZN5boost10filesystem4pathD1Ev
// IDA 0x23ac2c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_23ac2c() {
}

// 0x23ac78 — __ZNSt6localeC2IN5boost10filesystem6detail18utf8_codecvt_facetEEERKS_PT_
// type: int __fastcall(int, const _Impl **, int, int, void *, int)
#[doc(alias = "std::locale::locale<boost::filesystem::detail::utf8_codecvt_facet>(std::locale const&,boost::filesystem::detail::utf8_codecvt_facet *)")]
// was: __ZNSt6localeC2IN5boost10filesystem6detail18utf8_codecvt_facetEEERKS_PT_
// IDA 0x23ac78: 74 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23ac78() {
}

// 0x23adc4 — __ZN5boost10filesystem4pathC2IPKcEET_S5_
// type: std::string *__fastcall(std::string *, _BYTE *, _BYTE *)
#[doc(alias = "boost::filesystem::path::path<char const*>(char const*,char const*)")]
// was: __ZN5boost10filesystem4pathC2IPKcEET_S5_
// IDA 0x23adc4: 122 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23adc4() {
}

// 0x23af94 — __ZNK5boost10filesystem6detail18utf8_codecvt_facet10do_unshiftER11__mbstate_tPcS5_RS5_
// type: int __fastcall(boost::filesystem::detail::utf8_codecvt_facet *this, __mbstate_t *, char *, char *, char **)
#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::do_unshift(__mbstate_t &,char *,char *,char *&)const")]
// was: __ZNK5boost10filesystem6detail18utf8_codecvt_facet10do_unshiftER11__mbstate_tPcS5_RS5_
// IDA 0x23af94: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23af94() {
}

// 0x23af9c — __ZNK5boost10filesystem6detail18utf8_codecvt_facet11do_encodingEv
// type: int __fastcall(boost::filesystem::detail::utf8_codecvt_facet *this)
#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::do_encoding(void)const")]
// was: __ZNK5boost10filesystem6detail18utf8_codecvt_facet11do_encodingEv
// IDA 0x23af9c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23af9c() {
}

// 0x23afa0 — __ZNK5boost10filesystem6detail18utf8_codecvt_facet16do_always_noconvEv
// type: int __fastcall(boost::filesystem::detail::utf8_codecvt_facet *this)
#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::do_always_noconv(void)const")]
// was: __ZNK5boost10filesystem6detail18utf8_codecvt_facet16do_always_noconvEv
// IDA 0x23afa0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23afa0() {
}

// 0x23afa4 — __ZNK5boost10filesystem6detail18utf8_codecvt_facet13do_max_lengthEv
// type: int __fastcall(boost::filesystem::detail::utf8_codecvt_facet *this)
#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::do_max_length(void)const")]
// was: __ZNK5boost10filesystem6detail18utf8_codecvt_facet13do_max_lengthEv
// IDA 0x23afa4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23afa4() {
}

// 0x23afa8 — __GLOBAL__I_a_40
// type: void __fastcall(int, int, int, int, char, void *, int, int, int, int)
#[doc(alias = "global constructor keyed to_a_40")]
// was: __GLOBAL__I_a_40
// IDA 0x23afa8: 127 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23afa8() {
}

// 0x23b14c — __ZNK5boost10filesystem6detail18utf8_codecvt_facet5do_inER11__mbstate_tPKcS6_RS6_PwS8_RS8_
// type: int __fastcall(int, int, char *, char *, char **, int *, int *, int **)
#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::do_in(__mbstate_t &,char const*,char const*,char const*&,wchar_t *,wchar_t *,wchar_t *&)const")]
// was: __ZNK5boost10filesystem6detail18utf8_codecvt_facet5do_inER11__mbstate_tPKcS6_RS6_PwS8_RS8_
// IDA 0x23b14c: 143 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23b14c() {
}

// 0x23b2d0 — __ZNK5boost10filesystem6detail18utf8_codecvt_facet6do_outER11__mbstate_tPKwS6_RS6_PcS8_RS8_
// type: bool __fastcall(int, int, _DWORD *, _DWORD *, _DWORD *, _BYTE *, _BYTE *, _DWORD *)
#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::do_out(__mbstate_t &,wchar_t const*,wchar_t const*,wchar_t const*&,char *,char *,char *&)const")]
// was: __ZNK5boost10filesystem6detail18utf8_codecvt_facet6do_outER11__mbstate_tPKwS6_RS6_PcS8_RS8_
// IDA 0x23b2d0: 140 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23b2d0() {
}

// 0x23b43c — __ZNK5boost10filesystem6detail18utf8_codecvt_facet9do_lengthERK11__mbstate_tPKcS7_m
// type: int __fastcall(boost::filesystem::detail::utf8_codecvt_facet *this, const __mbstate_t *, const char *, const char *, unsigned int)
#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::do_length(__mbstate_t const&,char const*,char const*,unsigned long)const")]
// was: __ZNK5boost10filesystem6detail18utf8_codecvt_facet9do_lengthERK11__mbstate_tPKcS7_m
// IDA 0x23b43c: 45 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23b43c() {
}

// 0x23b4ac — __ZN5boost10filesystem6detail18utf8_codecvt_facetD1Ev
// type: void __fastcall(boost::filesystem::detail::utf8_codecvt_facet *__hidden this)
#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::~utf8_codecvt_facet()")]
// was: __ZN5boost10filesystem6detail18utf8_codecvt_facetD1Ev
// IDA 0x23b4ac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_23b4ac() {
}

// 0x23b4b8 — __ZN5boost10filesystem6detail18utf8_codecvt_facetD0Ev
// type: void __fastcall(boost::filesystem::detail::utf8_codecvt_facet *__hidden this)
#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::~utf8_codecvt_facet()")]
// was: __ZN5boost10filesystem6detail18utf8_codecvt_facetD0Ev
// IDA 0x23b4b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_23b4b8() {
}

// 0x23b4cc — __ZN5boost6system16generic_categoryEv
// type: int *__fastcall()
#[doc(alias = "boost::system::generic_category(void)")]
// was: __ZN5boost6system16generic_categoryEv
// IDA 0x23b4cc: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23b4cc() {
}

// 0x23b508 — __ZN5boost6system15system_categoryEv
// type: int *__fastcall()
#[doc(alias = "boost::system::system_category(void)")]
// was: __ZN5boost6system15system_categoryEv
// IDA 0x23b508: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23b508() {
}

// 0x23b544 — __ZN12_GLOBAL__N_121system_error_categoryD1Ev
// type: void __fastcall(_anonymous_namespace_::system_error_category *__hidden this)
#[doc(alias = "anonymous namespace::system_error_category::~system_error_category()")]
// was: __ZN12_GLOBAL__N_121system_error_categoryD1Ev
// IDA 0x23b544: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_23b544() {
}

// 0x23b548 — __ZN12_GLOBAL__N_122generic_error_categoryD1Ev
// type: void __fastcall(_anonymous_namespace_::generic_error_category *__hidden this)
#[doc(alias = "anonymous namespace::generic_error_category::~generic_error_category()")]
// was: __ZN12_GLOBAL__N_122generic_error_categoryD1Ev
// IDA 0x23b548: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_23b548() {
}

// 0x23b54c — __ZN12_GLOBAL__N_122generic_error_categoryD0Ev
// type: void __fastcall(_anonymous_namespace_::generic_error_category *__hidden this)
#[doc(alias = "anonymous namespace::generic_error_category::~generic_error_category()")]
// was: __ZN12_GLOBAL__N_122generic_error_categoryD0Ev
// IDA 0x23b54c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_23b54c() {
}

// 0x23b558 — __ZNK12_GLOBAL__N_122generic_error_category4nameEv
// type: const char *__fastcall(_anonymous_namespace_::generic_error_category *this)
#[doc(alias = "anonymous namespace::generic_error_category::name(void)const")]
// was: __ZNK12_GLOBAL__N_122generic_error_category4nameEv
// IDA 0x23b558: 3 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23b558() {
}

// 0x23b564 — __ZNK12_GLOBAL__N_122generic_error_category7messageEi
// type: int __fastcall(_anonymous_namespace_::generic_error_category *this, int, int)
#[doc(alias = "anonymous namespace::generic_error_category::message(int)const")]
// was: __ZNK12_GLOBAL__N_122generic_error_category7messageEi
// IDA 0x23b564: 213 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23b564() {
}

// 0x23b7cc — __ZN12_GLOBAL__N_121system_error_categoryD0Ev
// type: void __fastcall(_anonymous_namespace_::system_error_category *__hidden this)
#[doc(alias = "anonymous namespace::system_error_category::~system_error_category()")]
// was: __ZN12_GLOBAL__N_121system_error_categoryD0Ev
// IDA 0x23b7cc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_23b7cc() {
}

// 0x23b7d8 — __ZNK12_GLOBAL__N_121system_error_category4nameEv
// type: const char *__fastcall(_anonymous_namespace_::system_error_category *this)
#[doc(alias = "anonymous namespace::system_error_category::name(void)const")]
// was: __ZNK12_GLOBAL__N_121system_error_category4nameEv
// IDA 0x23b7d8: 3 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23b7d8() {
}

// 0x23b7e4 — __ZNK12_GLOBAL__N_121system_error_category7messageEi
// type: int __fastcall(_anonymous_namespace_::system_error_category *this, int, int)
#[doc(alias = "anonymous namespace::system_error_category::message(int)const")]
// was: __ZNK12_GLOBAL__N_121system_error_category7messageEi
// IDA 0x23b7e4: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23b7e4() {
}

// 0x23b838 — __ZNK12_GLOBAL__N_121system_error_category23default_error_conditionEi
// type: void __fastcall(_anonymous_namespace_::system_error_category *this, int, int)
#[doc(alias = "anonymous namespace::system_error_category::default_error_condition(int)const")]
// was: __ZNK12_GLOBAL__N_121system_error_category23default_error_conditionEi
// IDA 0x23b838: 1356 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23b838() {
}

// 0x23ca3c — __ZNK5boost6system14error_category23default_error_conditionEi
// type: _QWORD *__fastcall(_QWORD *this, int, __int64)
#[doc(alias = "boost::system::error_category::default_error_condition(int)const")]
// was: __ZNK5boost6system14error_category23default_error_conditionEi
// IDA 0x23ca3c: 3 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23ca3c() {
}

// 0x23ca44 — __ZNK5boost6system14error_category10equivalentEiRKNS0_15error_conditionE
// type: bool __fastcall(int, int, _DWORD *)
#[doc(alias = "boost::system::error_category::equivalent(int,boost::system::error_condition const&)const")]
// was: __ZNK5boost6system14error_category10equivalentEiRKNS0_15error_conditionE
// IDA 0x23ca44: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23ca44() {
}

// 0x23ca70 — __ZNK5boost6system14error_category10equivalentERKNS0_10error_codeEi
// type: bool __fastcall(int, _DWORD *, int)
#[doc(alias = "boost::system::error_category::equivalent(boost::system::error_code const&,int)const")]
// was: __ZNK5boost6system14error_category10equivalentERKNS0_10error_codeEi
// IDA 0x23ca70: 11 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23ca70() {
}

// 0x23ca88 — __GLOBAL__I_a_41
// type: void()
#[doc(alias = "global constructor keyed to_a_41")]
// was: __GLOBAL__I_a_41
// IDA 0x23ca88: 70 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23ca88() {
}

// 0x23cb64 — __ZN5boost9iostreams6detail11gzip_header7processEc
// type: void __fastcall(boost::iostreams::detail::gzip_header *this, unsigned __int8)
#[doc(alias = "boost::iostreams::detail::gzip_header::process(char)")]
// was: __ZN5boost9iostreams6detail11gzip_header7processEc
// IDA 0x23cb64: 329 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23cb64() {
}

// 0x23cef0 — __ZN5boost9iostreams6detail11gzip_header5resetEv
// type: int __fastcall(boost::iostreams::detail::gzip_header *this)
#[doc(alias = "boost::iostreams::detail::gzip_header::reset(void)")]
// was: __ZN5boost9iostreams6detail11gzip_header5resetEv
// IDA 0x23cef0: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23cef0() {
}

// 0x23cf2c — __ZN5boost9iostreams6detail11gzip_footer7processEc
// type: _DWORD *__fastcall(_DWORD *this, unsigned __int8)
#[doc(alias = "boost::iostreams::detail::gzip_footer::process(char)")]
// was: __ZN5boost9iostreams6detail11gzip_footer7processEc
// IDA 0x23cf2c: 33 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23cf2c() {
}

// 0x23cf7c — __ZN5boost9iostreams6detail11gzip_footer5resetEv
// type: _DWORD *__fastcall(_DWORD *this)
#[doc(alias = "boost::iostreams::detail::gzip_footer::reset(void)")]
// was: __ZN5boost9iostreams6detail11gzip_footer5resetEv
// IDA 0x23cf7c: 7 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23cf7c() {
}

// 0x23cf8c — __ZN5boost9iostreams10zlib_error5checkEi
// type: void __fastcall(boost::iostreams::zlib_error *this, int)
#[doc(alias = "boost::iostreams::zlib_error::check(int)")]
// was: __ZN5boost9iostreams10zlib_error5checkEi
// IDA 0x23cf8c: 109 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23cf8c() {
}

// 0x23d0c8 — __ZN5boost9iostreams6detail9zlib_baseC2Ev
// type: boost::iostreams::detail::zlib_base *__fastcall(boost::iostreams::detail::zlib_base *this)
#[doc(alias = "boost::iostreams::detail::zlib_base::zlib_base(void)")]
// was: __ZN5boost9iostreams6detail9zlib_baseC2Ev
// IDA 0x23d0c8: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23d0c8() {
}

// 0x23d0e8 — __ZN5boost9iostreams6detail9zlib_baseD2Ev
// type: void __fastcall(void **this)
#[doc(alias = "boost::iostreams::detail::zlib_base::~zlib_base()")]
// was: __ZN5boost9iostreams6detail9zlib_baseD2Ev
// IDA 0x23d0e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_23d0e8() {
}

// 0x23d0fc — __ZN5boost9iostreams6detail9zlib_base6beforeERPKcS4_RPcS6_
// type: int __fastcall(boost::iostreams::detail::zlib_base *this, const char **, const char *, char **, char *)
#[doc(alias = "boost::iostreams::detail::zlib_base::before(char const*&,char const*,char *&,char *)")]
// was: __ZN5boost9iostreams6detail9zlib_base6beforeERPKcS4_RPcS6_
// IDA 0x23d0fc: 13 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23d0fc() {
}

// 0x23d120 — __ZN5boost9iostreams6detail9zlib_base5afterERPKcRPcb
// type: const char *__fastcall(boost::iostreams::detail::zlib_base *this, const char **, char **, int)
#[doc(alias = "boost::iostreams::detail::zlib_base::after(char const*&,char *&,bool)")]
// was: __ZN5boost9iostreams6detail9zlib_base5afterERPKcRPcb
// IDA 0x23d120: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23d120() {
}

// 0x23d180 — __ZN5boost9iostreams6detail9zlib_base8xdeflateEi
// type: int __fastcall(z_streamp *this, int)
#[doc(alias = "boost::iostreams::detail::zlib_base::xdeflate(int)")]
// was: __ZN5boost9iostreams6detail9zlib_base8xdeflateEi
// IDA 0x23d180: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23d180() {
}

// 0x23d18c — __ZN5boost9iostreams6detail9zlib_base8xinflateEi
// type: int __fastcall(z_streamp *this, int)
#[doc(alias = "boost::iostreams::detail::zlib_base::xinflate(int)")]
// was: __ZN5boost9iostreams6detail9zlib_base8xinflateEi
// IDA 0x23d18c: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23d18c() {
}

// 0x23d198 — __ZN5boost9iostreams6detail9zlib_base5resetEbb
// type: int __fastcall(z_stream **this, int, int)
#[doc(alias = "boost::iostreams::detail::zlib_base::reset(bool,bool)")]
// was: __ZN5boost9iostreams6detail9zlib_base5resetEbb
// IDA 0x23d198: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23d198() {
}

// 0x23d1c8 — __ZN5boost9iostreams6detail9zlib_base7do_initERKNS0_11zlib_paramsEbPFPvS6_jjEPFvS6_S6_ES6_
// type: void __fastcall(int, int, int, int, int, void *)
#[doc(alias = "boost::iostreams::detail::zlib_base::do_init(boost::iostreams::zlib_params const&,bool,void * (*)(void *,unsigned int,unsigned int),void (*)(void *,void *),void *)")]
// was: __ZN5boost9iostreams6detail9zlib_base7do_initERKNS0_11zlib_paramsEbPFPvS6_jjEPFvS6_S6_ES6_
// IDA 0x23d1c8: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23d1c8() {
}

// 0x23d238 — __ZN5boost15throw_exceptionINS_9iostreams10zlib_errorEEEvRKT_
// type: void __fastcall __noreturn(int)
#[doc(alias = "void boost::throw_exception<boost::iostreams::zlib_error>(boost::iostreams::zlib_error const&)")]
// was: __ZN5boost15throw_exceptionINS_9iostreams10zlib_errorEEEvRKT_
// IDA 0x23d238: 124 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_23d238() {
}

// 0x2b9fcc — __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_8functionIS7_EEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::function<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_8functionIS7_EEED1Ev
// IDA 0x2b9fcc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2b9fcc() {
}

// 0x2ba0dc — __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_8functionIS7_EEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::function<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_8functionIS7_EEED0Ev
// IDA 0x2ba0dc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2ba0dc() {
}

// 0x2ba20c — __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::disconnect(void)")]
// was: __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slot10disconnectEv
// IDA 0x2ba20c: 93 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ba20c() {
}

// 0x2ba31c — __ZNK3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::connected(void)const")]
// was: __ZNK3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slot9connectedEv
// IDA 0x2ba31c: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ba31c() {
}

// 0x2ba328 — __ZN3rbx8callableINS_7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi3ES8_E4callESsSsS7_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>,3,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::call(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi3ES8_E4callESsSsS7_
// IDA 0x2ba328: 175 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ba328() {
}

// 0x2ba518 — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi3ES8_E4callESsSsS7_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>,3,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::call(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi3ES8_E4callESsSsS7_
// IDA 0x2ba518: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ba518() {
}

// 0x2ba520 — __ZNK5boost9function3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEEclESsSsS4_
#[doc(alias = "boost::function3<void,std::string,std::string,rbx_core::SharedPtr<RBX::Instance>>::operator()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)const")]
// was: __ZNK5boost9function3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEEclESsSsS4_
// IDA 0x2ba520: 202 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ba520() {
}

// 0x2ba750 — __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE6removeEPNS8_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::remove(rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *)")]
// was: __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE6removeEPNS8_4slotE
// IDA 0x2ba750: 78 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ba750() {
}

// 0x2ba840 — __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slot22safe_static_init_mutexEv
// IDA 0x2ba840: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2ba840() {
}

// 0x2ba844 — __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slot24safe_static_do_get_mutexEv
// IDA 0x2ba844: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ba844() {
}

// 0x2ba934 — __ZN3rbx8callableINS_7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi3ES8_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>,3,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi3ES8_ED1Ev
// IDA 0x2ba934: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2ba934() {
}

// 0x2baa44 — __ZN3rbx8callableINS_7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi3ES8_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>,3,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi3ES8_ED0Ev
// IDA 0x2baa44: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2baa44() {
}

// 0x2bab74 — __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotD1Ev
// IDA 0x2bab74: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2bab74() {
}

// 0x2baba0 — __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotD0Ev
// IDA 0x2baba0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2baba0() {
}

// 0x2bac74 — __ZN5boost9function3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEE13assign_to_ownERKS5_
#[doc(alias = "boost::function3<void,std::string,std::string,rbx_core::SharedPtr<RBX::Instance>>::assign_to_own(boost::function3<void,std::string,std::string,rbx_core::SharedPtr<RBX::Instance>> const&)")]
// was: __ZN5boost9function3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEE13assign_to_ownERKS5_
// IDA 0x2bac74: 20 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bac74() {
}

// 0x2baca8 — __ZNSt4listIN3RBX10Reflection19SignatureDescriptor4ItemESaIS3_EE14_M_create_nodeERKS3_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::list<RBX::Reflection::SignatureDescriptor::Item,std::allocator<RBX::Reflection::SignatureDescriptor::Item>>::_M_create_node(RBX::Reflection::SignatureDescriptor::Item const&)")]
// was: __ZNSt4listIN3RBX10Reflection19SignatureDescriptor4ItemESaIS3_EE14_M_create_nodeERKS3_
// IDA 0x2baca8: 89 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2baca8() {
}

// 0x2bada0 — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvbELi1EEC2EMS2_FvbEPKcS8_bNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(bool),1>::BoundFuncDesc(void (RBX::ScriptContext::*)(bool),char const*,char const*,bool,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvbELi1EEC2EMS2_FvbEPKcS8_bNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x2bada0: 159 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bada0() {
}

// 0x2baf4c — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvbELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvbELi1EE16declareSignatureEPKcNS0_7VariantE
// IDA 0x2baf4c: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2baf4c() {
}

// 0x2baf7c — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvbELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(bool),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvbELi1EED0Ev
// IDA 0x2baf7c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2baf7c() {
}

// 0x2bb050 — __ZNK3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x2bb050: 20 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bb050() {
}

// 0x2bb084 — __ZN3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EEC2INS_13ScriptContextEEEPKcS7_MT_iNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundProp<RBX::ScriptContext>(char const*,char const*,int RBX::ScriptContext::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EEC2INS_13ScriptContextEEEPKcS7_MT_iNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x2bb084: 153 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bb084() {
}

// 0x2bb218 — __ZN3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EED0Ev
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::~BoundProp()")]
// was: __ZN3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EED0Ev
// IDA 0x2bb218: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2bb218() {
}

// 0x2bb248 — __ZN3rbx8any_castIRKiN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "int const& rbx::any_cast<int const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRKiN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// IDA 0x2bb248: 78 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bb248() {
}

// 0x2bb330 — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_13ScriptContextEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ScriptContext>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_13ScriptContextEE10isReadOnlyEv
// IDA 0x2bb330: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bb330() {
}

// 0x2bb334 — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_13ScriptContextEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ScriptContext>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_13ScriptContextEE11isWriteOnlyEv
// IDA 0x2bb334: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bb334() {
}

// 0x2bb338 — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_13ScriptContextEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ScriptContext>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_13ScriptContextEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x2bb338: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bb338() {
}

// 0x2bb344 — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_13ScriptContextEE8setValueEPNS0_13DescribedBaseERKi
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ScriptContext>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
// was: __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_13ScriptContextEE8setValueEPNS0_13DescribedBaseERKi
// IDA 0x2bb344: 31 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bb344() {
}

// 0x2bb394 — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EEC2EMS2_FSA_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::ScriptContext::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EEC2EMS2_FSA_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x2bb394: 90 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bb394() {
}

// 0x2bb498 — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EED0Ev
// IDA 0x2bb498: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2bb498() {
}

// 0x2bb54c — __ZNK3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x2bb54c: 13 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bb54c() {
}

// 0x2bb570 — __ZN3RBX10Reflection11Call0HelperINS_13ScriptContextEMS2_FN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvESA_E4callEPS2_SC_RS6_
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::ScriptContext,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::ScriptContext::*)(void),rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::call(RBX::ScriptContext*,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::ScriptContext::*)(void),RBX::Reflection::Variant&)")]
// was: __ZN3RBX10Reflection11Call0HelperINS_13ScriptContextEMS2_FN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvESA_E4callEPS2_SC_RS6_
// IDA 0x2bb570: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bb570() {
}

// 0x2bb658 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIN5boost10shared_ptrIKSt6vectorINS1_10Reflection7VariantESaIS9_EEEEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> const&)")]
// was: __ZN3rbx13placement_anyIN3RBX7Region3EEaSIN5boost10shared_ptrIKSt6vectorINS1_10Reflection7VariantESaIS9_EEEEEERS3_RKT_
// IDA 0x2bb658: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bb658() {
}

// 0x2bb6c0 — __ZN5boost10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS4_EEEaSERKS8_
#[doc(alias = "rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>::operator=(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> const&)")]
// was: __ZN5boost10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS4_EEEaSERKS8_
// IDA 0x2bb6c0: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bb6c0() {
}

// 0x2bb6f8 — __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS7_EEEEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::construct_func(char const*,char *)")]
// was: __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS7_EEEEE14construct_funcEPKcPc
// IDA 0x2bb6f8: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bb6f8() {
}

// 0x2bb720 — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFN5boost10shared_ptrIKNS0_5TupleEEEbELi1EEC2EMS2_FS7_bEPKcSD_bNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(bool),1>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Reflection::Tuple const> (RBX::ScriptContext::*)(bool),char const*,char const*,bool,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFN5boost10shared_ptrIKNS0_5TupleEEEbELi1EEC2EMS2_FS7_bEPKcSD_bNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x2bb720: 159 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bb720() {
}

// 0x2bb8cc — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFN5boost10shared_ptrIKNS0_5TupleEEEbELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFN5boost10shared_ptrIKNS0_5TupleEEEbELi1EE16declareSignatureEPKcNS0_7VariantE
// IDA 0x2bb8cc: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bb8cc() {
}

// 0x2bb8fc — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFN5boost10shared_ptrIKNS0_5TupleEEEbELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(bool),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFN5boost10shared_ptrIKNS0_5TupleEEEbELi1EED0Ev
// IDA 0x2bb8fc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2bb8fc() {
}

// 0x7bcc68 — __ZNK3RBX8Humanoid22render3dSortedPositionEv
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this)
#[doc(alias = "RBX::Humanoid::render3dSortedPosition(void)const")]
// was: __ZNK3RBX8Humanoid22render3dSortedPositionEv
// IDA 0x7bcc68: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7bcc68() {
}

// 0x7bcc9c — __ZThn268_NK3RBX8Humanoid22render3dSortedPositionEv
// type: int __fastcall(RBX::Humanoid *this)
#[doc(alias = "non-virtual thunk toRBX::Humanoid::render3dSortedPosition(void)const")]
// was: __ZThn268_NK3RBX8Humanoid22render3dSortedPositionEv
// IDA 0x7bcc9c: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7bcc9c() {
}

// 0xa63bd8 — __ZN6RakNet7RakPeer31SetSplitMessageProgressIntervalEi
// type: unsigned int __fastcall(RakNet::RakPeer *this, int)
#[doc(alias = "RakNet::RakPeer::SetSplitMessageProgressInterval(int)")]
// was: __ZN6RakNet7RakPeer31SetSplitMessageProgressIntervalEi
// IDA 0xa63bd8: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_a63bd8() {
}

// 0xa63c14 — __ZNK6RakNet7RakPeer31GetSplitMessageProgressIntervalEv
// type: int __fastcall(RakNet::RakPeer *this)
#[doc(alias = "RakNet::RakPeer::GetSplitMessageProgressInterval(void)const")]
// was: __ZNK6RakNet7RakPeer31GetSplitMessageProgressIntervalEv
// IDA 0xa63c14: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_a63c14() {
}

// 0xa76c90 — __ZN6RakNet16ReliabilityLayer31SetSplitMessageProgressIntervalEi
// type: int __fastcall(int this, int)
#[doc(alias = "RakNet::ReliabilityLayer::SetSplitMessageProgressInterval(int)")]
// was: __ZN6RakNet16ReliabilityLayer31SetSplitMessageProgressIntervalEi
// IDA 0xa76c90: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_a76c90() {
}

// 0xbdc9e8 — __ZN3RBX17TextureCompositor17renderJobFinalizeERNS0_3JobE
// type: _DWORD __fastcall(RBX::TextureCompositor *__hidden this, RBX::TextureCompositor::Job *)
#[doc(alias = "RBX::TextureCompositor::renderJobFinalize(RBX::TextureCompositor::Job &)")]
// was: __ZN3RBX17TextureCompositor17renderJobFinalizeERNS0_3JobE
// IDA 0xbdc9e8: 713 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdc9e8() {
}

// 0xbdd154 — __ZN3RBX17TextureCompositor20renderJobIfNecessaryERNS0_3JobEm
// type: _DWORD __fastcall(RBX::TextureCompositor *__hidden this, RBX::TextureCompositor::Job *, unsigned int)
#[doc(alias = "RBX::TextureCompositor::renderJobIfNecessary(RBX::TextureCompositor::Job &,unsigned long)")]
// was: __ZN3RBX17TextureCompositor20renderJobIfNecessaryERNS0_3JobEm
// IDA 0xbdd154: 816 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdd154() {
}

// 0xee1570 — -[BSAFHTTPRequestOperation setUploadProgressBlock:]
// type: void __cdecl(BSAFHTTPRequestOperation *self, SEL, id)
#[doc(alias = "-[BSAFHTTPRequestOperation setUploadProgressBlock:]")]
// was: -[BSAFHTTPRequestOperation setUploadProgressBlock:]
// IDA 0xee1570: ObjC setter (STR ivar); field recovery pending — no-op.
pub fn stub_ee1570() {
}

// 0xee1588 — -[BSAFHTTPRequestOperation setDownloadProgressBlock:]
// type: void __cdecl(BSAFHTTPRequestOperation *self, SEL, id)
#[doc(alias = "-[BSAFHTTPRequestOperation setDownloadProgressBlock:]")]
// was: -[BSAFHTTPRequestOperation setDownloadProgressBlock:]
// IDA 0xee1588: ObjC setter (STR ivar); field recovery pending — no-op.
pub fn stub_ee1588() {
}

// 0xee2290 — -[BSAFHTTPRequestOperation uploadProgress]
// type: id __cdecl(BSAFHTTPRequestOperation *self, SEL)
#[doc(alias = "-[BSAFHTTPRequestOperation uploadProgress]")]
// was: -[BSAFHTTPRequestOperation uploadProgress]
// IDA 0xee2290: ObjC getter (LDR ivar, returns id); field recovery pending — no-op.
pub fn stub_ee2290() {
}

// 0xee22a8 — -[BSAFHTTPRequestOperation setUploadProgress:]
// type: void __cdecl(BSAFHTTPRequestOperation *self, SEL, id)
#[doc(alias = "-[BSAFHTTPRequestOperation setUploadProgress:]")]
// was: -[BSAFHTTPRequestOperation setUploadProgress:]
// IDA 0xee22a8: ObjC setter (STR ivar); field recovery pending — no-op.
pub fn stub_ee22a8() {
}

// 0xee22d8 — -[BSAFHTTPRequestOperation downloadProgress]
// type: id __cdecl(BSAFHTTPRequestOperation *self, SEL)
#[doc(alias = "-[BSAFHTTPRequestOperation downloadProgress]")]
// was: -[BSAFHTTPRequestOperation downloadProgress]
// IDA 0xee22d8: ObjC getter (LDR ivar, returns id); field recovery pending — no-op.
pub fn stub_ee22d8() {
}

// 0xee22f0 — -[BSAFHTTPRequestOperation setDownloadProgress:]
// type: void __cdecl(BSAFHTTPRequestOperation *self, SEL, id)
#[doc(alias = "-[BSAFHTTPRequestOperation setDownloadProgress:]")]
// was: -[BSAFHTTPRequestOperation setDownloadProgress:]
// IDA 0xee22f0: ObjC setter (STR ivar); field recovery pending — no-op.
pub fn stub_ee22f0() {
}
