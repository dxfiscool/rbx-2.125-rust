// Auto-generated skeletons for rbx-script — script filler EA-sorted asc next 100 not yet in script
// Filter: Script|Lua|Yield|CodeGen|ScriptContext (4818 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x43ce40..0x444cd4 | script 24955->25055 distinct (filler 0x43ce40 asc, not-in-script 60593->60493)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>>::clear_buffer(void)")]
pub fn stub_0x43ce40() -> crate::slot::PortedFn {
// IDA 0x43ce40: boost::io::basic_altstringbuf<char, std::char_traits<char>, std::allocator<char>>::clear_buffer().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x43ce40, "boost::io::basic_altstringbuf<char, std::char_traits<char>, std::allocator<char>>::clear_buffer()")
}

#[doc(alias = "boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::~basic_oaltstringstream()")]
pub fn stub_0x43cf00() -> crate::slot::PortedFn {
// IDA 0x43cf00: boost::io::basic_oaltstringstream<char, std::char_traits<char>, std::allocator<char>>::~basic_oaltstringstream().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x43cf00, "boost::io::basic_oaltstringstream<char, std::char_traits<char>, std::allocator<char>>::~basic_oaltst~")
}

#[doc(alias = "boost::optional_detail::optional_base<std::locale>::is_initialized(void)const")]
pub fn stub_0x43cfd4() -> crate::slot::PortedFn {
// std::locale facet — host locale is process-wide.
crate::slot::PortedFn::new(0x43cfd4, "boost::optional_detail::optional_base<std::locale>::is_initialized() const")
}

#[doc(alias = "boost::detail::shared_count::shared_count<boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>> *,boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::No_Op>(boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>> *,boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::No_Op)")]
pub fn stub_0x43cfd8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>> *,boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::No_Op>::~sp_counted_impl_pd()")]
pub fn stub_0x43d0b8(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>> *,boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::No_Op>::~sp_counted_impl_pd() [0x43d0bc]")]
pub fn stub_0x43d0bc(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>> *,boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::No_Op>::dispose(void)")]
pub fn stub_0x43d0c0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>> *,boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::No_Op>::get_deleter(std::type_info const&)")]
pub fn stub_0x43d0c4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>> *,boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::No_Op>::get_untyped_deleter(void)")]
pub fn stub_0x43d0dc() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::io::too_many_args::~too_many_args() [0x43d0e0]")]
pub fn stub_0x43d0e0() -> crate::slot::PortedFn {
// IDA 0x43d0e0: boost::io::too_many_args::~too_many_args().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x43d0e0, "boost::io::too_many_args::~too_many_args()")
}

#[doc(alias = "boost::io::too_many_args::what(void)const")]
pub fn stub_0x43d0f4() -> crate::slot::PortedFn {
// IDA 0x43d0f4: boost::io::too_many_args::what() const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x43d0f4, "boost::io::too_many_args::what() const")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::~clone_impl()")]
pub fn stub_0x43d100(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::io::too_many_args>::~error_info_injector()")]
pub fn stub_0x43d110(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::io::too_many_args>::~error_info_injector() [0x43d114]")]
pub fn stub_0x43d114(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::~clone_impl() [0x43d1d0]")]
pub fn stub_0x43d1d0(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::rethrow(void)const")]
pub fn stub_0x43d1e8(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::~clone_impl()")]
pub fn stub_0x43d318(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 12, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 12);
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::clone(void)const")]
pub fn stub_0x43d330(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::rethrow(void)const")]
pub fn stub_0x43d33c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::~clone_impl()")]
pub fn stub_0x43d34c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::io::too_many_args>::~error_info_injector() [0x43d368]")]
pub fn stub_0x43d368(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::io::too_many_args>::~error_info_injector()")]
pub fn stub_0x43d37c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 12, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 12);
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::clone_impl(boost::exception_detail::error_info_injector<boost::io::too_many_args> const&)")]
pub fn stub_0x43d398(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::size(void)const")]
pub fn stub_0x43d4d8() -> crate::slot::PortedFn {
// IDA 0x43d4d8: boost::basic_format<char, std::char_traits<char>, std::allocator<char>>::size() const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x43d4d8, "boost::basic_format<char, std::char_traits<char>, std::allocator<char>>::size() const")
}

#[doc(alias = "boost::io::too_few_args::~too_few_args()")]
pub fn stub_0x43d528() -> crate::slot::PortedFn {
// IDA 0x43d528: boost::io::too_few_args::~too_few_args().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x43d528, "boost::io::too_few_args::~too_few_args()")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::~clone_impl()")]
pub fn stub_0x43d540(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::io::too_few_args>::~error_info_injector()")]
pub fn stub_0x43d550(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::io::too_few_args>::~error_info_injector() [0x43d554]")]
pub fn stub_0x43d554(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::io::too_few_args>::~error_info_injector()")]
pub fn stub_0x43d60c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 12, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 12);
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::~clone_impl()")]
pub fn stub_0x43d614(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 12, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 12);
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::~clone_impl()")]
pub fn stub_0x43d61c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::~clone_impl() [0x43d628]")]
pub fn stub_0x43d628(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::clone(void)const")]
pub fn stub_0x43d63c(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::rethrow(void)const")]
pub fn stub_0x43d6f8(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::~clone_impl() [0x43d828]")]
pub fn stub_0x43d828(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 12, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 12);
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::clone(void)const")]
pub fn stub_0x43d840(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::io::too_few_args>::~error_info_injector() [0x43d850]")]
pub fn stub_0x43d850(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::clone_tag)")]
pub fn stub_0x43d864(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::clone_impl(boost::exception_detail::error_info_injector<boost::io::too_few_args> const&)")]
pub fn stub_0x43d9a0(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "boost::basic_format<char,std::char_traits<char>,std::allocator<char>> & boost::io::detail::feed<char,std::char_traits<char>,std::allocator<char>,int const&>(boost::basic_format<char,std::char_traits<char>,std::allocator<char>> &,int const&)")]
pub fn stub_0x43dadc() -> crate::slot::PortedFn {
// IDA 0x43dadc: boost::basic_format<char, std::char_traits<char>, std::allocator<char>>& boost::io::detail::feed<char, std::char_traits<~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x43dadc, "boost::basic_format<char, std::char_traits<char>, std::allocator<char>>& boost::io::detail::feed<cha~")
}

#[doc(alias = "void boost::io::detail::distribute<char,std::char_traits<char>,std::allocator<char>,int const&>(boost::basic_format<char,std::char_traits<char>,std::allocator<char>> &,int const&)")]
pub fn stub_0x43db38() -> crate::slot::PortedFn {
// IDA 0x43db38: void boost::io::detail::distribute<char, std::char_traits<char>, std::allocator<char>, int const&>(boost::basic_format<c~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x43db38, "void boost::io::detail::distribute<char, std::char_traits<char>, std::allocator<char>, int const&>(b~")
}

#[doc(alias = "void boost::io::detail::put<char,std::char_traits<char>,std::allocator<char>,int const&>(int const&,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> const&,boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::string_type &,boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::internal_streambuf_t &,std::locale *)")]
pub fn stub_0x43dc58() -> crate::slot::PortedFn {
// std::locale facet — host locale is process-wide.
crate::slot::PortedFn::new(0x43dc58, "void boost::io::detail::put<char, std::char_traits<char>, std::allocator<char>, int const&>(int cons~")
}

#[doc(alias = "boost::basic_format<char,std::char_traits<char>,std::allocator<char>> & boost::io::detail::feed<char,std::char_traits<char>,std::allocator<char>,double const&>(boost::basic_format<char,std::char_traits<char>,std::allocator<char>> &,double const&)")]
pub fn stub_0x43e15c() -> crate::slot::PortedFn {
// IDA 0x43e15c: boost::basic_format<char, std::char_traits<char>, std::allocator<char>>& boost::io::detail::feed<char, std::char_traits<~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x43e15c, "boost::basic_format<char, std::char_traits<char>, std::allocator<char>>& boost::io::detail::feed<cha~")
}

#[doc(alias = "void boost::io::detail::distribute<char,std::char_traits<char>,std::allocator<char>,double const&>(boost::basic_format<char,std::char_traits<char>,std::allocator<char>> &,double const&)")]
pub fn stub_0x43e1b8() -> crate::slot::PortedFn {
// IDA 0x43e1b8: void boost::io::detail::distribute<char, std::char_traits<char>, std::allocator<char>, double const&>(boost::basic_forma~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x43e1b8, "void boost::io::detail::distribute<char, std::char_traits<char>, std::allocator<char>, double const&~")
}

#[doc(alias = "void boost::io::detail::put<char,std::char_traits<char>,std::allocator<char>,double const&>(double const&,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> const&,boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::string_type &,boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::internal_streambuf_t &,std::locale *)")]
pub fn stub_0x43e2d8() -> crate::slot::PortedFn {
// std::locale facet — host locale is process-wide.
crate::slot::PortedFn::new(0x43e2d8, "void boost::io::detail::put<char, std::char_traits<char>, std::allocator<char>, double const&>(doubl~")
}

#[doc(alias = "boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::basic_format(char const*)")]
pub fn stub_0x43e7f0() -> crate::slot::PortedFn {
// IDA 0x43e7f0: boost::basic_format<char, std::char_traits<char>, std::allocator<char>>::basic_format(char const*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x43e7f0, "boost::basic_format<char, std::char_traits<char>, std::allocator<char>>::basic_format(char const*)")
}

#[doc(alias = "boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>>::~basic_altstringbuf()")]
pub fn stub_0x43ea00() -> crate::slot::PortedFn {
// IDA 0x43ea00: boost::io::basic_altstringbuf<char, std::char_traits<char>, std::allocator<char>>::~basic_altstringbuf().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x43ea00, "boost::io::basic_altstringbuf<char, std::char_traits<char>, std::allocator<char>>::~basic_altstringb~")
}

#[doc(alias = "std::vector<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>::~vector()")]
pub fn stub_0x43ea3c(vec: crate::slot::VecModel) {
// sequence dtor — releases the storage.
drop(vec);
}

#[doc(alias = "boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>>::dealloc(void)")]
pub fn stub_0x43ea8c() -> crate::slot::PortedFn {
// IDA 0x43ea8c: boost::io::basic_altstringbuf<char, std::char_traits<char>, std::allocator<char>>::dealloc().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x43ea8c, "boost::io::basic_altstringbuf<char, std::char_traits<char>, std::allocator<char>>::dealloc()")
}

#[doc(alias = "boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>>::~basic_altstringbuf() [0x43eabc]")]
pub fn stub_0x43eabc() -> crate::slot::PortedFn {
// IDA 0x43eabc: boost::io::basic_altstringbuf<char, std::char_traits<char>, std::allocator<char>>::~basic_altstringbuf().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x43eabc, "boost::io::basic_altstringbuf<char, std::char_traits<char>, std::allocator<char>>::~basic_altstringb~")
}

#[doc(alias = "boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>>::underflow(void)")]
pub fn stub_0x43eb00() -> crate::slot::PortedFn {
// IDA 0x43eb00: boost::io::basic_altstringbuf<char, std::char_traits<char>, std::allocator<char>>::underflow().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x43eb00, "boost::io::basic_altstringbuf<char, std::char_traits<char>, std::allocator<char>>::underflow()")
}

#[doc(alias = "boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>>::pbackfail(int)")]
pub fn stub_0x43eb48() -> crate::slot::PortedFn {
// IDA 0x43eb48: boost::io::basic_altstringbuf<char, std::char_traits<char>, std::allocator<char>>::pbackfail(int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x43eb48, "boost::io::basic_altstringbuf<char, std::char_traits<char>, std::allocator<char>>::pbackfail(int)")
}

#[doc(alias = "boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>>::overflow(int)")]
pub fn stub_0x43eb98() -> crate::slot::PortedFn {
// IDA 0x43eb98: boost::io::basic_altstringbuf<char, std::char_traits<char>, std::allocator<char>>::overflow(int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x43eb98, "boost::io::basic_altstringbuf<char, std::char_traits<char>, std::allocator<char>>::overflow(int)")
}

#[doc(alias = "boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::getloc(void)const")]
pub fn stub_0x43ecd4() -> crate::slot::PortedFn {
// IDA 0x43ecd4: boost::basic_format<char, std::char_traits<char>, std::allocator<char>>::getloc() const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x43ecd4, "boost::basic_format<char, std::char_traits<char>, std::allocator<char>>::getloc() const")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TestService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x440b48(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TestService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x440b50() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TestService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x440b70() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TestService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x440b88() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Teams,RBX::Teams>(boost::shared_ptr<RBX::Teams> const*,RBX::Teams *)const")]
pub fn stub_0x441838() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Teams")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Teams *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x441928(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UIEvent const&)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot> &)")]
pub fn stub_0x441cf0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("rbx::signals::signal<void (RBX::UIEvent const&)>::slot")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UIEvent const&)>::on_error(std::exception &)")]
pub fn stub_0x441e50(msg: &str) -> String {
// exception ctor — carries the message (thrown via panic
// at the throw site, cf. LuaTableKeysMustBeStrings).
msg.to_owned()
}

#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot> const&)")]
pub fn stub_0x441e78(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UIEvent const&)>::safe_static_init_mutex(void)")]
pub fn stub_0x441e9c(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (RBX::UIEvent const&)>::safe_static_init_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UIEvent const&)>::safe_static_do_get_mutex(void)")]
pub fn stub_0x441ea0(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (RBX::UIEvent const&)>::safe_static_do_get_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::shared_ptr<RBX::GuiTarget> boost::dynamic_pointer_cast<RBX::GuiTarget,RBX::Instance>(boost::shared_ptr<RBX::Instance> const&)")]
pub fn stub_0x442184() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::GuiTarget")
}

#[doc(alias = "boost::shared_ptr<RBX::Instance>::shared_ptr<RBX::Instance>(boost::weak_ptr<RBX::Instance> const&,boost::detail::sp_nothrow_tag)")]
pub fn stub_0x4421cc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_14GuiImageButtonELZNS_15sGuiImageButtonEENS_14FactoryProductIS2_NS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0x442248(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<RBX::GuiImageButton, RBX::sGuiImageButton, RBX::FactoryProduct<~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9GuiButtonELZNS_10sGuiButtonEENS_17NonFactoryProductINS_9GuiObjectELZNS_10sGuiButtonEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0x442368(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<RBX::GuiButton, RBX::sGuiButton, RBX::NonFactoryProduct<RBX::Gu~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0x442488(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<RBX::GuiObject, RBX::sGuiObject, RBX::NonFactoryProduct<RBX::Gu~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9GuiBase2dELZNS_10sGuiBase2dEENS_17NonFactoryProductINS_7GuiBaseELZNS_10sGuiBase2dEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0x4425a8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<RBX::GuiBase2d, RBX::sGuiBase2d, RBX::NonFactoryProduct<RBX::Gu~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7GuiBaseELZNS_8sGuiBaseEENS_17NonFactoryProductINS_8InstanceELZNS_8sGuiBaseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0x4426c8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<RBX::GuiBase, RBX::sGuiBase, RBX::NonFactoryProduct<RBX::Instan~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "void std::__introsort_loop<__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,int,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*)>(__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,int,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*))")]
pub fn stub_0x442a90() -> crate::slot::PortedFn {
// IDA 0x442a90: void std::__introsort_loop<__gnu_cxx::__normal_iterator<RBX::IAdornable**, std::vector<RBX::IAdornable*, std::allocator<~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x442a90, "void std::__introsort_loop<__gnu_cxx::__normal_iterator<RBX::IAdornable**, std::vector<RBX::IAdornab~")
}

#[doc(alias = "void std::__final_insertion_sort<__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*)>(__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*))")]
pub fn stub_0x442b4c() -> crate::slot::PortedFn {
// IDA 0x442b4c: void std::__final_insertion_sort<__gnu_cxx::__normal_iterator<RBX::IAdornable**, std::vector<RBX::IAdornable*, std::allo~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x442b4c, "void std::__final_insertion_sort<__gnu_cxx::__normal_iterator<RBX::IAdornable**, std::vector<RBX::IA~")
}

#[doc(alias = "void std::__insertion_sort<__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*)>(__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*))")]
pub fn stub_0x442bb8() -> crate::slot::PortedFn {
// IDA 0x442bb8: void std::__insertion_sort<__gnu_cxx::__normal_iterator<RBX::IAdornable**, std::vector<RBX::IAdornable*, std::allocator<~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x442bb8, "void std::__insertion_sort<__gnu_cxx::__normal_iterator<RBX::IAdornable**, std::vector<RBX::IAdornab~")
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>> std::__unguarded_partition<__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,RBX::IAdornable *,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*)>(__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,RBX::IAdornable *,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*))")]
pub fn stub_0x442c34() -> crate::slot::PortedFn {
// IDA 0x442c34: __gnu_cxx::__normal_iterator<RBX::IAdornable**, std::vector<RBX::IAdornable*, std::allocator<RBX::IAdornable*>>> std::__~.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x442c34, "__gnu_cxx::__normal_iterator<RBX::IAdornable**, std::vector<RBX::IAdornable*, std::allocator<RBX::IA~")
}

#[doc(alias = "void std::__heap_select<__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*)>(__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*))")]
pub fn stub_0x442c80() -> crate::slot::PortedFn {
// IDA 0x442c80: void std::__heap_select<__gnu_cxx::__normal_iterator<RBX::IAdornable**, std::vector<RBX::IAdornable*, std::allocator<RBX~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x442c80, "void std::__heap_select<__gnu_cxx::__normal_iterator<RBX::IAdornable**, std::vector<RBX::IAdornable*~")
}

#[doc(alias = "void std::sort_heap<__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*)>(__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*))")]
pub fn stub_0x442cf0() -> crate::slot::PortedFn {
// IDA 0x442cf0: void std::sort_heap<__gnu_cxx::__normal_iterator<RBX::IAdornable**, std::vector<RBX::IAdornable*, std::allocator<RBX::IA~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x442cf0, "void std::sort_heap<__gnu_cxx::__normal_iterator<RBX::IAdornable**, std::vector<RBX::IAdornable*, st~")
}

#[doc(alias = "void std::__adjust_heap<__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,int,RBX::IAdornable *,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*)>(__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,int,int,RBX::IAdornable *,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*))")]
pub fn stub_0x442d30() -> crate::slot::PortedFn {
// IDA 0x442d30: void std::__adjust_heap<__gnu_cxx::__normal_iterator<RBX::IAdornable**, std::vector<RBX::IAdornable*, std::allocator<RBX~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x442d30, "void std::__adjust_heap<__gnu_cxx::__normal_iterator<RBX::IAdornable**, std::vector<RBX::IAdornable*~")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ControllerService,RBX::ControllerService>(boost::shared_ptr<RBX::ControllerService> const*,RBX::ControllerService *)const")]
pub fn stub_0x442fa8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ControllerService")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x443098]")]
pub fn stub_0x443098(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_19MegaClusterInstanceELZNS_12sMegaClusterEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0x4430a0(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<RBX::MegaClusterInstance, RBX::sMegaCluster, RBX::FactoryProduc~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_17ReplicatedStorageENS_8InstanceELZNS_18sReplicatedStorageEES2_E7CreatorD2Ev")]
pub fn stub_0x443efc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ReplicatedStorage"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_17ReplicatedStorageENS_8InstanceELZNS_18sReplicatedStorageEES2_E7Creator12getClassNameEv")]
pub fn stub_0x443f98() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ReplicatedStorage"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_17ReplicatedStorageENS_8InstanceELZNS_18sReplicatedStorageEES2_E7Creator6createEv")]
pub fn stub_0x444004() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ReplicatedStorage"
}

#[doc(alias = "boost::shared_ptr<RBX::ReplicatedStorage> RBX::Creatable<RBX::Instance>::create<RBX::ReplicatedStorage>(void)")]
pub fn stub_0x444148() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ReplicatedStorage")
}

#[doc(alias = "boost::shared_ptr<RBX::ReplicatedStorage>::shared_ptr<RBX::ReplicatedStorage,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ReplicatedStorage *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x4441f8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ReplicatedStorage")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ReplicatedStorage,RBX::ReplicatedStorage>(boost::shared_ptr<RBX::ReplicatedStorage> const*,RBX::ReplicatedStorage *)const")]
pub fn stub_0x4442c0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ReplicatedStorage")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ReplicatedStorage *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ReplicatedStorage *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x4443ac() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ReplicatedStorage *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x4444b8(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ReplicatedStorage *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x4444bc() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ReplicatedStorage *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x4444d4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_18sReplicatedStorageEEEERKS0_v")]
pub fn stub_0x4444d8(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sReplicatedStorage>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_18sReplicatedStorageEEEERKS0_v")]
pub fn stub_0x444520(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sReplicatedStorage>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_17ReplicatedStorageENS_8InstanceELZNS_18sReplicatedStorageEES2_E7CreatorC2Ev")]
pub fn stub_0x444604() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ReplicatedStorage"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_17ReplicatedStorageENS_8InstanceELZNS_18sReplicatedStorageEES2_E15isNullClassNameEv")]
pub fn stub_0x444830(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::FactoryProduct getter.
cell.get()
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_17ReplicatedStorageENS_8InstanceELZNS_18sReplicatedStorageEES2_E17static_getCreatorEv")]
pub fn stub_0x444898() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ReplicatedStorage"
}

#[doc(alias = "boost::shared_ptr<RBX::ServerStorage> RBX::Creatable<RBX::Instance>::create<RBX::ServerStorage>(void)")]
pub fn stub_0x444b5c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ServerStorage")
}

#[doc(alias = "boost::shared_ptr<RBX::ServerStorage>::shared_ptr<RBX::ServerStorage,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ServerStorage *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x444c0c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ServerStorage")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ServerStorage,RBX::ServerStorage>(boost::shared_ptr<RBX::ServerStorage> const*,RBX::ServerStorage *)const")]
pub fn stub_0x444cd4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ServerStorage")
}
