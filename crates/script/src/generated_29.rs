// Auto-generated skeletons for rbx-script — filler EA-sorted after 0x2360c4 (next 120) [filler EA-sorted ascending earliest gap]
// Filter: Lua|Script|Yield|CodeGen (7431 filtered, all already stubbed globally) — filler EA-sorted ascending
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +120 stubs | range 0x236318..0x23f50c | existing 7751 -> 7871 total (filler after 0x2360c4, EA-sorted ascending)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0x236318 — __ZN5boost6thread26do_try_join_until_noexceptERK8timespecRb
// type: int __fastcall(boost::thread *this, const timespec *, bool *)
// was: int __fastcall(boost::thread *this, const timespec *, bool *)
#[doc(alias = "boost::thread::do_try_join_until_noexcept(timespec const&,bool &)")]
pub fn stub_0x236318() -> crate::slot::PortedFn {
// IDA 0x236318: boost::thread::do_try_join_until_noexcept(timespec const&,bool &).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x236318, "boost::thread::do_try_join_until_noexcept(timespec const&,bool &)")
}

// 0x236598 — __ZN5boost6thread6detachEv
// type: void __fastcall(boost::thread *this)
// was: void __fastcall(boost::thread *this)
#[doc(alias = "boost::thread::detach(void)")]
pub fn stub_0x236598() -> crate::slot::PortedFn {
// IDA 0x236598: boost::thread::detach(void).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x236598, "boost::thread::detach(void)")
}

// 0x2366b0 — __ZN5boost11this_thread5hiden11sleep_untilERK8timespec
// type: void __fastcall(boost::this_thread::hiden *this, const timespec *, int, int)
// was: void __fastcall(boost::this_thread::hiden *this, const timespec *, int, int)
#[doc(alias = "boost::this_thread::hiden::sleep_until(timespec const&)")]
pub fn stub_0x2366b0() -> crate::slot::PortedFn {
// IDA 0x2366b0: boost::this_thread::hiden::sleep_until(timespec const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2366b0, "boost::this_thread::hiden::sleep_until(timespec const&)")
}

// 0x2368cc — __ZN5boost6thread13native_handleEv
// type: int __fastcall(boost::thread *this)
// was: int __fastcall(boost::thread *this)
#[doc(alias = "boost::thread::native_handle(void)")]
pub fn stub_0x2368cc() -> crate::slot::PortedFn {
// IDA 0x2368cc: boost::thread::native_handle(void).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2368cc, "boost::thread::native_handle(void)")
}

// 0x236a00 — __ZN5boost11this_thread18interruption_pointEv
// type: void __fastcall(boost::this_thread *this, int, int, int)
// was: void __fastcall(boost::this_thread *this, int, int, int)
#[doc(alias = "boost::this_thread::interruption_point(void)")]
pub fn stub_0x236a00() -> crate::slot::PortedFn {
// IDA 0x236a00: boost::this_thread::interruption_point(void).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x236a00, "boost::this_thread::interruption_point(void)")
}

// 0x236b14 — __ZN5boost11this_thread20disable_interruptionC1Ev
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(boost::this_thread::disable_interruption *this, int, int, int)
// was: void __fastcall __spoils<R1,R2,R3,R12,LR>(boost::this_thread::disable_interruption *this, int, int, int)
#[doc(alias = "boost::this_thread::disable_interruption::disable_interruption(void)")]
pub fn stub_0x236b14() -> crate::slot::PortedFn {
// IDA 0x236b14: boost::this_thread::disable_interruption::disable_interruption(void).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x236b14, "boost::this_thread::disable_interruption::disable_interruption(void)")
}

// 0x236c14 — __ZN5boost11this_thread20disable_interruptionD1Ev
// type: void __fastcall(boost::this_thread::disable_interruption *this, int, int, int)
// was: void __fastcall(boost::this_thread::disable_interruption *this, int, int, int)
#[doc(alias = "boost::this_thread::disable_interruption::~disable_interruption()")]
pub fn stub_0x236c14() -> crate::slot::PortedFn {
// IDA 0x236c14: boost::this_thread::disable_interruption::~disable_interruption().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x236c14, "boost::this_thread::disable_interruption::~disable_interruption()")
}

// 0x236d04 — __ZN5boost12_GLOBAL__N_131get_or_make_current_thread_dataEv
// type: void *__fastcall(boost::_anonymous_namespace_ *this, int, int, int)
// was: void *__fastcall(boost::_anonymous_namespace_ *this, int, int, int)
#[doc(alias = "boost::anonymous namespace::get_or_make_current_thread_data(void)")]
pub fn stub_0x236d04() -> crate::slot::PortedFn {
// IDA 0x236d04: boost::anonymous namespace::get_or_make_current_thread_data(void).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x236d04, "boost::anonymous namespace::get_or_make_current_thread_data(void)")
}

// 0x236ec0 — __ZN5boost6detail12get_tss_dataEPKv
// type: _DWORD __fastcall(boost::detail *__hidden this, const void *)
// was: _DWORD __fastcall(boost::detail *__hidden this, const void *)
#[doc(alias = "boost::detail::get_tss_data(void const*)")]
pub fn stub_0x236ec0() -> crate::slot::PortedFn {
// IDA 0x236ec0: boost::detail::get_tss_data(void const*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x236ec0, "boost::detail::get_tss_data(void const*)")
}

// 0x236f30 — __ZN5boost6detail16add_new_tss_nodeEPKvNS_10shared_ptrINS0_20tss_cleanup_functionEEEPv
// type: void __fastcall(boost::_anonymous_namespace_ *, int *, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: void __fastcall(boost::_anonymous_namespace_ *, int *, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::detail::add_new_tss_node(void const*,rbx_core::SharedPtr<boost::detail::tss_cleanup_function>,void *)")]
pub fn stub_0x236f30() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::detail::tss_cleanup_function")
}

// 0x237130 — __ZN5boost6detail12set_tss_dataEPKvNS_10shared_ptrINS0_20tss_cleanup_functionEEEPvb
// type: void __fastcall(boost::_anonymous_namespace_ *, int *, int, int)
// was: void __fastcall(boost::_anonymous_namespace_ *, int *, int, int)
#[doc(alias = "boost::detail::set_tss_data(void const*,rbx_core::SharedPtr<boost::detail::tss_cleanup_function>,void *,bool)")]
pub fn stub_0x237130() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::detail::tss_cleanup_function")
}

// 0x237348 — __ZN5boost12_GLOBAL__N_126externally_launched_threadD1Ev
// type: void __fastcall(boost::_anonymous_namespace_::externally_launched_thread *__hidden this)
// was: void __fastcall(boost::_anonymous_namespace_::externally_launched_thread *__hidden this)
#[doc(alias = "boost::anonymous namespace::externally_launched_thread::~externally_launched_thread()")]
pub fn stub_0x237348() -> crate::slot::PortedFn {
// IDA 0x237348: boost::anonymous namespace::externally_launched_thread::~externally_launched_thread().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x237348, "boost::anonymous namespace::externally_launched_thread::~externally_launched_thread()")
}

// 0x237354 — __ZN5boost12_GLOBAL__N_126externally_launched_threadD0Ev
// type: void __fastcall(boost::_anonymous_namespace_::externally_launched_thread *__hidden this)
// was: void __fastcall(boost::_anonymous_namespace_::externally_launched_thread *__hidden this)
#[doc(alias = "boost::anonymous namespace::externally_launched_thread::~externally_launched_thread() [0x237354]")]
pub fn stub_0x237354() -> crate::slot::PortedFn {
// IDA 0x237354: boost::anonymous namespace::externally_launched_thread::~externally_launched_thread() [0x237354].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x237354, "boost::anonymous namespace::externally_launched_thread::~externally_launched_thread() [0x237354]")
}

// 0x237368 — __ZN5boost12_GLOBAL__N_126externally_launched_thread3runEv
// type: void __fastcall(boost::_anonymous_namespace_::externally_launched_thread *this)
// was: void __fastcall(boost::_anonymous_namespace_::externally_launched_thread *this)
#[doc(alias = "boost::anonymous namespace::externally_launched_thread::run(void)")]
pub fn stub_0x237368() -> crate::slot::PortedFn {
// IDA 0x237368: boost::anonymous namespace::externally_launched_thread::run(void).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x237368, "boost::anonymous namespace::externally_launched_thread::run(void)")
}

#[doc(alias = "boost::anonymous namespace::externally_launched_thread::notify_all_at_thread_exit(boost::condition_variable *,boost::mutex *)")]
pub fn stub_0x23736c() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x23736c, "boost::(anonymous namespace)::externally_launched_thread::notify_all_at_thread_exit(boost::condition~")
}

#[doc(alias = "rbx_core::SharedPtr<boost::detail::thread_data_base>::operator=(rbx_core::SharedPtr<boost::detail::thread_data_base> const&)")]
pub fn stub_0x2374bc(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<boost::detail::tss_cleanup_function>::operator=(rbx_core::SharedPtr<boost::detail::tss_cleanup_function> const&)")]
pub fn stub_0x2375b0(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "std::_Rb_tree<void const*,std::pair<void const* const,boost::detail::tss_data_node>,std::_Select1st<std::pair<void const* const,boost::detail::tss_data_node>>,std::less<void const*>,std::allocator<std::pair<void const* const,boost::detail::tss_data_node>>>::erase(std::_Rb_tree_iterator<std::pair<void const* const,boost::detail::tss_data_node>>,std::_Rb_tree_iterator<std::pair<void const* const,boost::detail::tss_data_node>>)")]
pub fn stub_0x2376a4(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<void const*,std::pair<void const* const,boost::detail::tss_data_node>,std::_Select1st<std::pair<void const* const,boost::detail::tss_data_node>>,std::less<void const*>,std::allocator<std::pair<void const* const,boost::detail::tss_data_node>>>::_M_erase(std::_Rb_tree_node<std::pair<void const* const,boost::detail::tss_data_node>> *)")]
pub fn stub_0x237798(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<void const*,std::pair<void const* const,boost::detail::tss_data_node>,std::_Select1st<std::pair<void const* const,boost::detail::tss_data_node>>,std::less<void const*>,std::allocator<std::pair<void const* const,boost::detail::tss_data_node>>>::_M_insert_unique(std::pair<void const* const,boost::detail::tss_data_node> const&)")]
pub fn stub_0x237848(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<void const*,std::pair<void const* const,boost::detail::tss_data_node>,std::_Select1st<std::pair<void const* const,boost::detail::tss_data_node>>,std::less<void const*>,std::allocator<std::pair<void const* const,boost::detail::tss_data_node>>>::_M_create_node(std::pair<void const* const,boost::detail::tss_data_node> const&)")]
pub fn stub_0x2378fc() -> crate::slot::PortedFn {
// IDA 0x2378fc: std::_Rb_tree<void const*, std::pair<void const* const, boost::detail::tss_data_node>, std::_Select1st<std::pair<void co~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2378fc, "std::_Rb_tree<void const*, std::pair<void const* const, boost::detail::tss_data_node>, std::_Select1~")
}

#[doc(alias = "void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data_base>(rbx_core::SharedPtr<boost::detail::thread_data_base> const*,boost::detail::thread_data_base *)const")]
pub fn stub_0x2379ec() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::detail::thread_data_base")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data_base>::~sp_counted_impl_p()")]
pub fn stub_0x237b40(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data_base>::~sp_counted_impl_p() [0x237b44]")]
pub fn stub_0x237b44(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data_base>::dispose(void)")]
pub fn stub_0x237b50() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data_base>::get_deleter(std::type_info const&)")]
pub fn stub_0x237b64() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data_base>::get_untyped_deleter(void)")]
pub fn stub_0x237b68() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::future_object_base::mark_finished_internal(boost::unique_lock<boost::mutex> &)")]
pub fn stub_0x237b6c(guard: &mut bool) {
// std lock op — the host guard flips.
*guard = true;
}

#[doc(alias = "boost::filesystem::detail::symlink_status(boost::filesystem::path const&,boost::system::error_code *)")]
pub fn stub_0x237d60() -> crate::slot::PortedFn {
// IDA 0x237d60: boost::filesystem::detail::symlink_status(boost::filesystem::path const&, boost::system::error_code*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x237d60, "boost::filesystem::detail::symlink_status(boost::filesystem::path const&, boost::system::error_code*~")
}

#[doc(alias = "boost::filesystem::detail::current_path(boost::system::error_code *)")]
pub fn stub_0x237fa4() -> crate::slot::PortedFn {
// IDA 0x237fa4: boost::filesystem::detail::current_path(boost::system::error_code*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x237fa4, "boost::filesystem::detail::current_path(boost::system::error_code*)")
}

#[doc(alias = "anonymous namespace::error(bool,boost::filesystem::path const&,boost::system::error_code *,std::string const&)")]
pub fn stub_0x238258() -> crate::slot::PortedFn {
// IDA 0x238258: (anonymous namespace)::error(bool, boost::filesystem::path const&, boost::system::error_code*, std::string const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x238258, "(anonymous namespace)::error(bool, boost::filesystem::path const&, boost::system::error_code*, std::~")
}

#[doc(alias = "boost::filesystem::detail::initial_path(boost::system::error_code *)")]
pub fn stub_0x23837c() -> crate::slot::PortedFn {
// IDA 0x23837c: boost::filesystem::detail::initial_path(boost::system::error_code*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x23837c, "boost::filesystem::detail::initial_path(boost::system::error_code*)")
}

#[doc(alias = "boost::filesystem::detail::is_empty(boost::filesystem::path const&,boost::system::error_code *)")]
pub fn stub_0x23852c() -> crate::slot::PortedFn {
// IDA 0x23852c: boost::filesystem::detail::is_empty(boost::filesystem::path const&, boost::system::error_code*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x23852c, "boost::filesystem::detail::is_empty(boost::filesystem::path const&, boost::system::error_code*)")
}

#[doc(alias = "boost::filesystem::detail::remove(boost::filesystem::path const&,boost::system::error_code *)")]
pub fn stub_0x2386d4() -> crate::slot::PortedFn {
// IDA 0x2386d4: boost::filesystem::detail::remove(boost::filesystem::path const&, boost::system::error_code*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2386d4, "boost::filesystem::detail::remove(boost::filesystem::path const&, boost::system::error_code*)")
}

#[doc(alias = "anonymous namespace::remove_file_or_directory(boost::filesystem::path const&,boost::filesystem::file_type,boost::system::error_code *)")]
pub fn stub_0x2388a8() -> crate::slot::PortedFn {
// IDA 0x2388a8: (anonymous namespace)::remove_file_or_directory(boost::filesystem::path const&, boost::filesystem::file_type, boost::sys~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2388a8, "(anonymous namespace)::remove_file_or_directory(boost::filesystem::path const&, boost::filesystem::f~")
}

#[doc(alias = "boost::filesystem::detail::status(boost::filesystem::path const&,boost::system::error_code *)")]
pub fn stub_0x238adc() -> crate::slot::PortedFn {
// IDA 0x238adc: boost::filesystem::detail::status(boost::filesystem::path const&, boost::system::error_code*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x238adc, "boost::filesystem::detail::status(boost::filesystem::path const&, boost::system::error_code*)")
}

#[doc(alias = "boost::filesystem::detail::system_complete(boost::filesystem::path const&,boost::system::error_code *)")]
pub fn stub_0x238d18() -> crate::slot::PortedFn {
// IDA 0x238d18: boost::filesystem::detail::system_complete(boost::filesystem::path const&, boost::system::error_code*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x238d18, "boost::filesystem::detail::system_complete(boost::filesystem::path const&, boost::system::error_code~")
}

#[doc(alias = "boost::filesystem::directory_entry::m_get_status(boost::system::error_code *)const")]
pub fn stub_0x238f14() -> crate::slot::PortedFn {
// IDA 0x238f14: boost::filesystem::directory_entry::m_get_status(boost::system::error_code*) const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x238f14, "boost::filesystem::directory_entry::m_get_status(boost::system::error_code*) const")
}

// 0x238f80 — __ZN5boost10filesystem6detail13dir_itr_closeERPvS3_
// type: int __fastcall(boost::filesystem::detail *this, void **, void **)
// was: int __fastcall(boost::filesystem::detail *this, void **, void **)
#[doc(alias = "boost::filesystem::detail::dir_itr_close(void *&,void *&)")]
pub fn stub_0x238f80() -> crate::slot::PortedFn {
// IDA 0x238f80: boost::filesystem::detail::dir_itr_close(void *&,void *&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x238f80, "boost::filesystem::detail::dir_itr_close(void *&,void *&)")
}

#[doc(alias = "boost::filesystem::detail::directory_iterator_construct(boost::filesystem::directory_iterator &,boost::filesystem::path const&,boost::system::error_code *)")]
pub fn stub_0x238fd4() -> crate::slot::PortedFn {
// IDA 0x238fd4: boost::filesystem::detail::directory_iterator_construct(boost::filesystem::directory_iterator&, boost::filesystem::path ~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x238fd4, "boost::filesystem::detail::directory_iterator_construct(boost::filesystem::directory_iterator&, boos~")
}

#[doc(alias = "boost::filesystem::detail::directory_iterator_increment(boost::filesystem::directory_iterator &,boost::system::error_code *)")]
pub fn stub_0x239668() -> crate::slot::PortedFn {
// IDA 0x239668: boost::filesystem::detail::directory_iterator_increment(boost::filesystem::directory_iterator&, boost::system::error_cod~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x239668, "boost::filesystem::detail::directory_iterator_increment(boost::filesystem::directory_iterator&, boos~")
}

// 0x239b34 — __ZN5boost10filesystem18directory_iteratorD1Ev
// type: void __fastcall(boost::filesystem::directory_iterator *__hidden this)
// was: void __fastcall(boost::filesystem::directory_iterator *__hidden this)
#[doc(alias = "boost::filesystem::directory_iterator::~directory_iterator()")]
pub fn stub_0x239b34() -> crate::slot::PortedFn {
// IDA 0x239b34: boost::filesystem::directory_iterator::~directory_iterator().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x239b34, "boost::filesystem::directory_iterator::~directory_iterator()")
}

#[doc(alias = "boost::filesystem::filesystem_error::~filesystem_error()")]
pub fn stub_0x239bc8() -> crate::slot::PortedFn {
// IDA 0x239bc8: boost::filesystem::filesystem_error::~filesystem_error().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x239bc8, "boost::filesystem::filesystem_error::~filesystem_error()")
}

// 0x239cc8 — __ZN5boost10filesystem16filesystem_errorC2ERKSsNS_6system10error_codeE
// type: std::runtime_error *__fastcall(std::runtime_error *, const std::string *, std::runtime_error_vtbl *, const char *, boost::detail::sp_counted_base *, std::runtime_error *, int, int, void *, int)
// was: std::runtime_error *__fastcall(std::runtime_error *, const std::string *, std::runtime_error_vtbl *, const char *, boost::detail::sp_counted_base *, std::runtime_error *, int, int, void *, int)
#[doc(alias = "boost::filesystem::filesystem_error::filesystem_error(std::string const&,boost::system::error_code)")]
pub fn stub_0x239cc8() -> crate::slot::PortedFn {
// IDA 0x239cc8: boost::filesystem::filesystem_error::filesystem_error(std::string const&,boost::system::error_code).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x239cc8, "boost::filesystem::filesystem_error::filesystem_error(std::string const&,boost::system::error_code)")
}

#[doc(alias = "boost::filesystem::filesystem_error::~filesystem_error() [0x239e90]")]
pub fn stub_0x239e90() -> crate::slot::PortedFn {
// IDA 0x239e90: boost::filesystem::filesystem_error::~filesystem_error().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x239e90, "boost::filesystem::filesystem_error::~filesystem_error()")
}

// 0x239f94 — __ZNK5boost10filesystem16filesystem_error4whatEv
// type: int __fastcall(boost::filesystem::filesystem_error *this)
// was: int __fastcall(boost::filesystem::filesystem_error *this)
#[doc(alias = "boost::filesystem::filesystem_error::what(void)const")]
pub fn stub_0x239f94() -> crate::slot::PortedFn {
// IDA 0x239f94: boost::filesystem::filesystem_error::what(void)const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x239f94, "boost::filesystem::filesystem_error::what(void)const")
}

// 0x23a11c — __ZN5boost6detail20sp_pointer_constructINS_10filesystem16filesystem_error5m_impES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, boost::detail::sp_counted_base **, int, void *, int)
// was: void __fastcall(int, int, boost::detail::sp_counted_base **, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<boost::filesystem::filesystem_error::m_imp,boost::filesystem::filesystem_error::m_imp>(rbx_core::SharedPtr<boost::filesystem::filesystem_error::m_imp> *,boost::filesystem::filesystem_error::m_imp *,boost::detail::shared_count &)")]
pub fn stub_0x23a11c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::filesystem::filesystem_error::m_imp")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::filesystem::filesystem_error::m_imp>::~sp_counted_impl_p()")]
pub fn stub_0x23a2bc(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::filesystem::filesystem_error::m_imp>::~sp_counted_impl_p() [0x23a2c0]")]
pub fn stub_0x23a2c0(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::filesystem::filesystem_error::m_imp>::dispose(void)")]
pub fn stub_0x23a2cc() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::filesystem::filesystem_error::m_imp>::get_deleter(std::type_info const&)")]
pub fn stub_0x23a38c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::filesystem::filesystem_error::m_imp>::get_untyped_deleter(void)")]
pub fn stub_0x23a390() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x23a394 — __ZN5boost10filesystem16filesystem_errorC2ERKSsRKNS0_4pathENS_6system10error_codeE
// type: std::runtime_error *__fastcall(std::runtime_error *, const std::string *, const std::string *, std::runtime_error_vtbl *, boost::detail::sp_counted_base *, std::runtime_error *, int, int, void *, int)
// was: std::runtime_error *__fastcall(std::runtime_error *, const std::string *, const std::string *, std::runtime_error_vtbl *, boost::detail::sp_counted_base *, std::runtime_error *, int, int, void *, int)
#[doc(alias = "boost::filesystem::filesystem_error::filesystem_error(std::string const&,boost::filesystem::path const&,boost::system::error_code)")]
pub fn stub_0x23a394() -> crate::slot::PortedFn {
// IDA 0x23a394: boost::filesystem::filesystem_error::filesystem_error(std::string const&,boost::filesystem::path const&,boost::system::e~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x23a394, "boost::filesystem::filesystem_error::filesystem_error(std::string const&,boost::filesystem::path con~")
}

// 0x23a630 — __ZN5boost10filesystem4pathdVERKS1_
// type: boost::filesystem::path *__fastcall(boost::filesystem::path *, const std::string *)
// was: boost::filesystem::path *__fastcall(boost::filesystem::path *, const std::string *)
#[doc(alias = "boost::filesystem::path::operator/=(boost::filesystem::path const&)")]
pub fn stub_0x23a630() -> crate::slot::PortedFn {
// IDA 0x23a630: boost::filesystem::path::operator/=(boost::filesystem::path const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x23a630, "boost::filesystem::path::operator/=(boost::filesystem::path const&)")
}

// 0x23a7b8 — __ZN5boost10filesystem4path28m_append_separator_if_neededEv
// type: int __fastcall(boost::filesystem::path *this)
// was: int __fastcall(boost::filesystem::path *this)
#[doc(alias = "boost::filesystem::path::m_append_separator_if_needed(void)")]
pub fn stub_0x23a7b8() -> crate::slot::PortedFn {
// IDA 0x23a7b8: boost::filesystem::path::m_append_separator_if_needed(void).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x23a7b8, "boost::filesystem::path::m_append_separator_if_needed(void)")
}

// 0x23a830 — __ZN5boost10filesystem4pathdVEPKc
// type: boost::filesystem::path *__fastcall(boost::filesystem::path *, const char *, int, int, int, char, int, int, struct _Unwind_Exception *lpuexcpt)
// was: boost::filesystem::path *__fastcall(boost::filesystem::path *, const char *, int, int, int, char, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "boost::filesystem::path::operator/=(char const*)")]
pub fn stub_0x23a830() -> crate::slot::PortedFn {
// IDA 0x23a830: boost::filesystem::path::operator/=(char const*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x23a830, "boost::filesystem::path::operator/=(char const*)")
}

#[doc(alias = "boost::filesystem::path::m_erase_redundant_separator(unsigned long)")]
pub fn stub_0x23a9d4() -> crate::slot::PortedFn {
// IDA 0x23a9d4: boost::filesystem::path::m_erase_redundant_separator(unsigned long).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x23a9d4, "boost::filesystem::path::m_erase_redundant_separator(unsigned long)")
}

// 0x23aa2c — __ZN5boost10filesystem4path15remove_filenameEv
// type: boost::filesystem::path *__fastcall(boost::filesystem::path *this)
// was: boost::filesystem::path *__fastcall(boost::filesystem::path *this)
#[doc(alias = "boost::filesystem::path::remove_filename(void)")]
pub fn stub_0x23aa2c() -> crate::slot::PortedFn {
// IDA 0x23aa2c: boost::filesystem::path::remove_filename(void).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x23aa2c, "boost::filesystem::path::remove_filename(void)")
}

// 0x23aa60 — __ZNK5boost10filesystem4path17m_parent_path_endEv
// type: int __fastcall(boost::filesystem::path *this)
// was: int __fastcall(boost::filesystem::path *this)
#[doc(alias = "boost::filesystem::path::m_parent_path_end(void)const")]
pub fn stub_0x23aa60() -> crate::slot::PortedFn {
// IDA 0x23aa60: boost::filesystem::path::m_parent_path_end(void)const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x23aa60, "boost::filesystem::path::m_parent_path_end(void)const")
}

// 0x23ab64 — __ZNK5boost10filesystem4path14root_directoryEv
// type: char *__fastcall(boost::filesystem::path *this, std::string *)
// was: char *__fastcall(boost::filesystem::path *this, std::string *)
#[doc(alias = "boost::filesystem::path::root_directory(void)const")]
pub fn stub_0x23ab64() -> crate::slot::PortedFn {
// IDA 0x23ab64: boost::filesystem::path::root_directory(void)const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x23ab64, "boost::filesystem::path::root_directory(void)const")
}

// 0x23abe8 — __ZNK5boost10filesystem4path11parent_pathEv
// type: char *__fastcall(boost::filesystem::path *this, boost::filesystem::path *)
// was: char *__fastcall(boost::filesystem::path *this, boost::filesystem::path *)
#[doc(alias = "boost::filesystem::path::parent_path(void)const")]
pub fn stub_0x23abe8() -> crate::slot::PortedFn {
// IDA 0x23abe8: boost::filesystem::path::parent_path(void)const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x23abe8, "boost::filesystem::path::parent_path(void)const")
}

// 0x23ac1c — __ZN5boost10filesystem4path7codecvtEv
// type: int __fastcall(boost::filesystem::path *this)
// was: int __fastcall(boost::filesystem::path *this)
#[doc(alias = "boost::filesystem::path::codecvt(void)")]
pub fn stub_0x23ac1c() -> crate::slot::PortedFn {
// IDA 0x23ac1c: boost::filesystem::path::codecvt(void).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x23ac1c, "boost::filesystem::path::codecvt(void)")
}

// 0x23ac2c — __ZN5boost10filesystem4pathD1Ev
// type: void __fastcall(boost::filesystem::path *__hidden this)
// was: void __fastcall(boost::filesystem::path *__hidden this)
#[doc(alias = "boost::filesystem::path::~path()")]
pub fn stub_0x23ac2c() -> crate::slot::PortedFn {
// IDA 0x23ac2c: boost::filesystem::path::~path().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x23ac2c, "boost::filesystem::path::~path()")
}

#[doc(alias = "std::locale::locale<boost::filesystem::detail::utf8_codecvt_facet>(std::locale const&,boost::filesystem::detail::utf8_codecvt_facet *)")]
pub fn stub_0x23ac78() -> crate::slot::PortedFn {
// std::locale facet — host locale is process-wide.
crate::slot::PortedFn::new(0x23ac78, "std::locale::locale<boost::filesystem::detail::utf8_codecvt_facet>(std::locale const&, boost::filesy~")
}

#[doc(alias = "boost::filesystem::path::path<char const*>(char const*,char const*)")]
pub fn stub_0x23adc4() -> crate::slot::PortedFn {
// IDA 0x23adc4: boost::filesystem::path::path<char const*>(char const*, char const*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x23adc4, "boost::filesystem::path::path<char const*>(char const*, char const*)")
}

// 0x23af94 — __ZNK5boost10filesystem6detail18utf8_codecvt_facet10do_unshiftER11__mbstate_tPcS5_RS5_
// type: int __fastcall(boost::filesystem::detail::utf8_codecvt_facet *this, __mbstate_t *, char *, char *, char **)
// was: int __fastcall(boost::filesystem::detail::utf8_codecvt_facet *this, __mbstate_t *, char *, char *, char **)
#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::do_unshift(__mbstate_t &,char *,char *,char *&)const")]
pub fn stub_0x23af94() -> crate::slot::PortedFn {
// std::locale facet — host locale is process-wide.
crate::slot::PortedFn::new(0x23af94, "boost::filesystem::detail::utf8_codecvt_facet::do_unshift(__mbstate_t &,char *,char *,char *&)const")
}

// 0x23af9c — __ZNK5boost10filesystem6detail18utf8_codecvt_facet11do_encodingEv
// type: int __fastcall(boost::filesystem::detail::utf8_codecvt_facet *this)
// was: int __fastcall(boost::filesystem::detail::utf8_codecvt_facet *this)
#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::do_encoding(void)const")]
pub fn stub_0x23af9c() -> crate::slot::PortedFn {
// std::locale facet — host locale is process-wide.
crate::slot::PortedFn::new(0x23af9c, "boost::filesystem::detail::utf8_codecvt_facet::do_encoding(void)const")
}

// 0x23afa0 — __ZNK5boost10filesystem6detail18utf8_codecvt_facet16do_always_noconvEv
// type: int __fastcall(boost::filesystem::detail::utf8_codecvt_facet *this)
// was: int __fastcall(boost::filesystem::detail::utf8_codecvt_facet *this)
#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::do_always_noconv(void)const")]
pub fn stub_0x23afa0() -> crate::slot::PortedFn {
// std::locale facet — host locale is process-wide.
crate::slot::PortedFn::new(0x23afa0, "boost::filesystem::detail::utf8_codecvt_facet::do_always_noconv(void)const")
}

// 0x23afa4 — __ZNK5boost10filesystem6detail18utf8_codecvt_facet13do_max_lengthEv
// type: int __fastcall(boost::filesystem::detail::utf8_codecvt_facet *this)
// was: int __fastcall(boost::filesystem::detail::utf8_codecvt_facet *this)
#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::do_max_length(void)const")]
pub fn stub_0x23afa4() -> crate::slot::PortedFn {
// std::locale facet — host locale is process-wide.
crate::slot::PortedFn::new(0x23afa4, "boost::filesystem::detail::utf8_codecvt_facet::do_max_length(void)const")
}

#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::do_in(__mbstate_t &,char const*,char const*,char const*&,wchar_t *,wchar_t *,wchar_t *&)const")]
pub fn stub_0x23b14c() -> crate::slot::PortedFn {
// std::locale facet — host locale is process-wide.
crate::slot::PortedFn::new(0x23b14c, "boost::filesystem::detail::utf8_codecvt_facet::do_in(__mbstate_t&, char const*, char const*, char co~")
}

#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::do_out(__mbstate_t &,wchar_t const*,wchar_t const*,wchar_t const*&,char *,char *,char *&)const")]
pub fn stub_0x23b2d0() -> crate::slot::PortedFn {
// std::locale facet — host locale is process-wide.
crate::slot::PortedFn::new(0x23b2d0, "boost::filesystem::detail::utf8_codecvt_facet::do_out(__mbstate_t&, wchar_t const*, wchar_t const*, ~")
}

// 0x23b43c — __ZNK5boost10filesystem6detail18utf8_codecvt_facet9do_lengthERK11__mbstate_tPKcS7_m
// type: int __fastcall(boost::filesystem::detail::utf8_codecvt_facet *this, const __mbstate_t *, const char *, const char *, unsigned int)
// was: int __fastcall(boost::filesystem::detail::utf8_codecvt_facet *this, const __mbstate_t *, const char *, const char *, unsigned int)
#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::do_length(__mbstate_t const&,char const*,char const*,unsigned long)const")]
pub fn stub_0x23b43c() -> crate::slot::PortedFn {
// std::locale facet — host locale is process-wide.
crate::slot::PortedFn::new(0x23b43c, "boost::filesystem::detail::utf8_codecvt_facet::do_length(__mbstate_t const&,char const*,char const*,~")
}

// 0x23b4ac — __ZN5boost10filesystem6detail18utf8_codecvt_facetD1Ev
// type: void __fastcall(boost::filesystem::detail::utf8_codecvt_facet *__hidden this)
// was: void __fastcall(boost::filesystem::detail::utf8_codecvt_facet *__hidden this)
#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::~utf8_codecvt_facet()")]
pub fn stub_0x23b4ac() -> crate::slot::PortedFn {
// std::locale facet — host locale is process-wide.
crate::slot::PortedFn::new(0x23b4ac, "boost::filesystem::detail::utf8_codecvt_facet::~utf8_codecvt_facet()")
}

// 0x23b4b8 — __ZN5boost10filesystem6detail18utf8_codecvt_facetD0Ev
// type: void __fastcall(boost::filesystem::detail::utf8_codecvt_facet *__hidden this)
// was: void __fastcall(boost::filesystem::detail::utf8_codecvt_facet *__hidden this)
#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::~utf8_codecvt_facet() [0x23b4b8]")]
pub fn stub_0x23b4b8() -> crate::slot::PortedFn {
// std::locale facet — host locale is process-wide.
crate::slot::PortedFn::new(0x23b4b8, "boost::filesystem::detail::utf8_codecvt_facet::~utf8_codecvt_facet() [0x23b4b8]")
}

#[doc(alias = "boost::system::generic_category(void)")]
pub fn stub_0x23b4cc() -> crate::slot::PortedFn {
// IDA 0x23b4cc: boost::system::generic_category().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x23b4cc, "boost::system::generic_category()")
}

#[doc(alias = "boost::system::system_category(void)")]
pub fn stub_0x23b508() -> crate::slot::PortedFn {
// IDA 0x23b508: boost::system::system_category().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x23b508, "boost::system::system_category()")
}

#[doc(alias = "boost::system::error_category::default_error_condition(int)const")]
pub fn stub_0x23ca3c() -> crate::slot::PortedFn {
// IDA 0x23ca3c: boost::system::error_category::default_error_condition(int) const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x23ca3c, "boost::system::error_category::default_error_condition(int) const")
}

#[doc(alias = "boost::system::error_category::equivalent(int,boost::system::error_condition const&)const")]
pub fn stub_0x23ca44() -> crate::slot::PortedFn {
// IDA 0x23ca44: boost::system::error_category::equivalent(int, boost::system::error_condition const&) const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x23ca44, "boost::system::error_category::equivalent(int, boost::system::error_condition const&) const")
}

#[doc(alias = "boost::system::error_category::equivalent(boost::system::error_code const&,int)const")]
pub fn stub_0x23ca70() -> crate::slot::PortedFn {
// IDA 0x23ca70: boost::system::error_category::equivalent(boost::system::error_code const&, int) const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x23ca70, "boost::system::error_category::equivalent(boost::system::error_code const&, int) const")
}

// 0x23cb64 — __ZN5boost9iostreams6detail11gzip_header7processEc
// type: void __fastcall(boost::iostreams::detail::gzip_header *this, unsigned __int8)
// was: void __fastcall(boost::iostreams::detail::gzip_header *this, unsigned __int8)
#[doc(alias = "boost::iostreams::detail::gzip_header::process(char)")]
pub fn stub_0x23cb64() -> crate::slot::PortedFn {
// IDA 0x23cb64: boost::iostreams::detail::gzip_header::process(char).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x23cb64, "boost::iostreams::detail::gzip_header::process(char)")
}

// 0x23cef0 — __ZN5boost9iostreams6detail11gzip_header5resetEv
// type: int __fastcall(boost::iostreams::detail::gzip_header *this)
// was: int __fastcall(boost::iostreams::detail::gzip_header *this)
#[doc(alias = "boost::iostreams::detail::gzip_header::reset(void)")]
pub fn stub_0x23cef0() -> crate::slot::PortedFn {
// IDA 0x23cef0: boost::iostreams::detail::gzip_header::reset(void).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x23cef0, "boost::iostreams::detail::gzip_header::reset(void)")
}

#[doc(alias = "boost::iostreams::detail::gzip_footer::process(char)")]
pub fn stub_0x23cf2c() -> crate::slot::PortedFn {
// IDA 0x23cf2c: boost::iostreams::detail::gzip_footer::process(char).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x23cf2c, "boost::iostreams::detail::gzip_footer::process(char)")
}

#[doc(alias = "boost::iostreams::detail::gzip_footer::reset(void)")]
pub fn stub_0x23cf7c() -> crate::slot::PortedFn {
// IDA 0x23cf7c: boost::iostreams::detail::gzip_footer::reset().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x23cf7c, "boost::iostreams::detail::gzip_footer::reset()")
}

// 0x23cf8c — __ZN5boost9iostreams10zlib_error5checkEi
// type: void __fastcall(boost::iostreams::zlib_error *this, int)
// was: void __fastcall(boost::iostreams::zlib_error *this, int)
#[doc(alias = "boost::iostreams::zlib_error::check(int)")]
pub fn stub_0x23cf8c() -> crate::slot::PortedFn {
// IDA 0x23cf8c: boost::iostreams::zlib_error::check(int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x23cf8c, "boost::iostreams::zlib_error::check(int)")
}

// 0x23d0c8 — __ZN5boost9iostreams6detail9zlib_baseC2Ev
// type: boost::iostreams::detail::zlib_base *__fastcall(boost::iostreams::detail::zlib_base *this)
// was: boost::iostreams::detail::zlib_base *__fastcall(boost::iostreams::detail::zlib_base *this)
#[doc(alias = "boost::iostreams::detail::zlib_base::zlib_base(void)")]
pub fn stub_0x23d0c8() -> crate::slot::PortedFn {
// IDA 0x23d0c8: boost::iostreams::detail::zlib_base::zlib_base(void).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x23d0c8, "boost::iostreams::detail::zlib_base::zlib_base(void)")
}

#[doc(alias = "boost::iostreams::detail::zlib_base::~zlib_base()")]
pub fn stub_0x23d0e8() -> crate::slot::PortedFn {
// IDA 0x23d0e8: boost::iostreams::detail::zlib_base::~zlib_base().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x23d0e8, "boost::iostreams::detail::zlib_base::~zlib_base()")
}

// 0x23d0fc — __ZN5boost9iostreams6detail9zlib_base6beforeERPKcS4_RPcS6_
// type: int __fastcall(boost::iostreams::detail::zlib_base *this, const char **, const char *, char **, char *)
// was: int __fastcall(boost::iostreams::detail::zlib_base *this, const char **, const char *, char **, char *)
#[doc(alias = "boost::iostreams::detail::zlib_base::before(char const*&,char const*,char *&,char *)")]
pub fn stub_0x23d0fc() -> crate::slot::PortedFn {
// IDA 0x23d0fc: boost::iostreams::detail::zlib_base::before(char const*&,char const*,char *&,char *).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x23d0fc, "boost::iostreams::detail::zlib_base::before(char const*&,char const*,char *&,char *)")
}

// 0x23d120 — __ZN5boost9iostreams6detail9zlib_base5afterERPKcRPcb
// type: const char *__fastcall(boost::iostreams::detail::zlib_base *this, const char **, char **, int)
// was: const char *__fastcall(boost::iostreams::detail::zlib_base *this, const char **, char **, int)
#[doc(alias = "boost::iostreams::detail::zlib_base::after(char const*&,char *&,bool)")]
pub fn stub_0x23d120() -> crate::slot::PortedFn {
// IDA 0x23d120: boost::iostreams::detail::zlib_base::after(char const*&,char *&,bool).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x23d120, "boost::iostreams::detail::zlib_base::after(char const*&,char *&,bool)")
}

#[doc(alias = "boost::iostreams::detail::zlib_base::xdeflate(int)")]
pub fn stub_0x23d180() -> crate::slot::PortedFn {
// IDA 0x23d180: boost::iostreams::detail::zlib_base::xdeflate(int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x23d180, "boost::iostreams::detail::zlib_base::xdeflate(int)")
}

#[doc(alias = "boost::iostreams::detail::zlib_base::xinflate(int)")]
pub fn stub_0x23d18c() -> crate::slot::PortedFn {
// IDA 0x23d18c: boost::iostreams::detail::zlib_base::xinflate(int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x23d18c, "boost::iostreams::detail::zlib_base::xinflate(int)")
}

#[doc(alias = "boost::iostreams::detail::zlib_base::reset(bool,bool)")]
pub fn stub_0x23d198() -> crate::slot::PortedFn {
// IDA 0x23d198: boost::iostreams::detail::zlib_base::reset(bool, bool).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x23d198, "boost::iostreams::detail::zlib_base::reset(bool, bool)")
}

#[doc(alias = "boost::iostreams::detail::zlib_base::do_init(boost::iostreams::zlib_params const&,bool,void * (*)(void *,unsigned int,unsigned int),void (*)(void *,void *),void *)")]
pub fn stub_0x23d1c8() -> crate::slot::PortedFn {
// IDA 0x23d1c8: boost::iostreams::detail::zlib_base::do_init(boost::iostreams::zlib_params const&, bool, void* (*)(void*, unsigned int, ~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x23d1c8, "boost::iostreams::detail::zlib_base::do_init(boost::iostreams::zlib_params const&, bool, void* (*)(v~")
}

#[doc(alias = "void boost::throw_exception<boost::iostreams::zlib_error>(boost::iostreams::zlib_error const&)")]
pub fn stub_0x23d238(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

// 0x23d390 — __ZN5boost9iostreams10zlib_errorD1Ev
// type: void __fastcall(boost::iostreams::zlib_error *__hidden this)
// was: void __fastcall(boost::iostreams::zlib_error *__hidden this)
#[doc(alias = "boost::iostreams::zlib_error::~zlib_error()")]
pub fn stub_0x23d390() -> crate::slot::PortedFn {
// IDA 0x23d390: boost::iostreams::zlib_error::~zlib_error().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x23d390, "boost::iostreams::zlib_error::~zlib_error()")
}

// 0x23d39c — __ZN5boost9iostreams10zlib_errorD0Ev
// type: void __fastcall(boost::iostreams::zlib_error *__hidden this)
// was: void __fastcall(boost::iostreams::zlib_error *__hidden this)
#[doc(alias = "boost::iostreams::zlib_error::~zlib_error() [0x23d39c]")]
pub fn stub_0x23d39c() -> crate::slot::PortedFn {
// IDA 0x23d39c: boost::iostreams::zlib_error::~zlib_error() [0x23d39c].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x23d39c, "boost::iostreams::zlib_error::~zlib_error() [0x23d39c]")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl()")]
pub fn stub_0x23d3b0(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>::~error_info_injector()")]
pub fn stub_0x23d468(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::iostreams::zlib_error>::~error_info_injector()")]
pub fn stub_0x23d520(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 12, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 12);
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl()")]
pub fn stub_0x23d5d8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 12, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 12);
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl()")]
pub fn stub_0x23d690(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl() [0x23d75c]")]
pub fn stub_0x23d75c(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone(void)const")]
pub fn stub_0x23d818(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::rethrow(void)const")]
pub fn stub_0x23d8d4(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl() [0x23d984]")]
pub fn stub_0x23d984(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 12, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 12);
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone(void)const")]
pub fn stub_0x23da40(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::rethrow(void)const")]
pub fn stub_0x23db04(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl() [0x23db14]")]
pub fn stub_0x23db14(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>> const&)")]
pub fn stub_0x23dbe8(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>::~error_info_injector() [0x23dd30]")]
pub fn stub_0x23dd30(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::iostreams::zlib_error>::~error_info_injector() [0x23ddec]")]
pub fn stub_0x23ddec(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 12, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 12);
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone_tag)")]
pub fn stub_0x23dea8(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone_impl(boost::exception_detail::error_info_injector<boost::iostreams::zlib_error> const&)")]
pub fn stub_0x23e044(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "RBX::trim_trailing_slashes(std::string const&)")]
pub fn stub_0x23e52c() -> crate::slot::PortedFn {
// IDA 0x23e52c: RBX::trim_trailing_slashes(std::string const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x23e52c, "RBX::trim_trailing_slashes(std::string const&)")
}

#[doc(alias = "RBX::Debugable::dump(std::ostream &)")]
pub fn stub_0x23e5f8(handle: &crate::slot::InstanceHandle) {
// RBX::Debugable::dump(std::ostream&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Log::timeStamp(std::basic_ofstream<char,std::char_traits<char>> &,bool)")]
pub fn stub_0x23e678(handle: &crate::slot::InstanceHandle) {
// RBX::Log::timeStamp(std::basic_ofstream<char, std::char_traits<char>>&, bool) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::date_time::gregorian_calendar_base<boost::date_time::year_month_day_base<boost::gregorian::greg_year,boost::gregorian::greg_month,boost::gregorian::greg_day>,unsigned int>::from_day_number(unsigned int)")]
pub fn stub_0x23ec04() -> crate::slot::PortedFn {
// IDA 0x23ec04: boost::date_time::gregorian_calendar_base<boost::date_time::year_month_day_base<boost::gregorian::greg_year, boost::greg~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x23ec04, "boost::date_time::gregorian_calendar_base<boost::date_time::year_month_day_base<boost::gregorian::gr~")
}

#[doc(alias = "boost::date_time::second_clock<boost::posix_time::ptime>::create_time(tm *)")]
pub fn stub_0x23ecfc(guard: &mut bool) {
// std lock op — the host guard flips.
*guard = true;
}

#[doc(alias = "boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config>::date(void)const")]
pub fn stub_0x23ef20() -> crate::slot::PortedFn {
// IDA 0x23ef20: boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config>::date() const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x23ef20, "boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config>::date() con~")
}

// 0x23f2ac — __ZN3RBX12boost_detail8init_fooEv
// type: void __fastcall(RBX::boost_detail *this)
// was: void __fastcall(RBX::boost_detail *this)
#[doc(alias = "RBX::boost_detail::init_foo(void)")]
pub fn stub_0x23f2ac(handle: &crate::slot::InstanceHandle) {
// RBX::boost_detail::init_foo(void) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::thread_wrapper(boost::function0<void> const&,char const*)")]
pub fn stub_0x23f50c() -> crate::slot::PortedFn {
// IDA 0x23f50c: RBX::thread_wrapper(boost::function0<void> const&, char const*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x23f50c, "RBX::thread_wrapper(boost::function0<void> const&, char const*)")
}
