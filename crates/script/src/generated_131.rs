// Auto-generated skeletons for rbx-script — filler EA-sorted asc next 100 uncovered (global)
// Filter: Script|Lua|Yield|lua (5401 filtered, all stubbed, 0 remaining)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x5c2874..0x9e9be4 EA-sorted asc next 100 uncovered (workspace uncovered 168->68, filler 7764->7864, rbx_core::SharedPtr not boost) [skeleton batch]
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "RBX::Lighting::setTime(boost::posix_time::time_duration const&)")]
pub fn stub_0x5c2874(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Lighting setter.
cell.set(value)
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Sky>::operator=(rbx_core::SharedPtr<RBX::Sky> const&)")]
pub fn stub_0x5c2eb4(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Sky> RBX::shared_from<RBX::Sky>(RBX::Sky*)")]
pub fn stub_0x5c2eec() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Sky")
}

#[doc(alias = "std::basic_string<char,std::char_traits<char>,std::allocator<char>> boost::posix_time::to_simple_string_type<char>(boost::posix_time::time_duration)")]
pub fn stub_0x5c501c(s: &String) -> &str {
// std::string::c_str.
s.as_str()
}

#[doc(alias = "boost::date_time::int_adapter<long long>::compare(boost::date_time::int_adapter<long long> const&)const")]
pub fn stub_0x5c5354() -> crate::slot::PortedFn {
// IDA 0x5c5354: boost::date_time::int_adapter<long long>::compare(boost::date_time::int_adapter<long long> const&) const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5c5354, "boost::date_time::int_adapter<long long>::compare(boost::date_time::int_adapter<long long> const&) c~")
}

#[doc(alias = "boost::posix_time::time_duration boost::date_time::str_from_delimited_time_duration<boost::posix_time::time_duration,char>(std::basic_string<char,std::char_traits<char>,std::allocator<char>> const&)")]
pub fn stub_0x5c549c(s: &String) -> &str {
// std::string::c_str.
s.as_str()
}

#[doc(alias = "boost::date_time::int_adapter<long long>::operator*(int)const")]
pub fn stub_0x5c5d80() -> crate::slot::PortedFn {
// IDA 0x5c5d80: boost::date_time::int_adapter<long long>::operator*(int) const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5c5d80, "boost::date_time::int_adapter<long long>::operator*(int) const")
}

#[doc(alias = "boost::date_time::int_adapter<long long>::mult_div_specials(int const&)const")]
pub fn stub_0x5c5de4() -> crate::slot::PortedFn {
// IDA 0x5c5de4: boost::date_time::int_adapter<long long>::mult_div_specials(int const&) const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5c5de4, "boost::date_time::int_adapter<long long>::mult_div_specials(int const&) const")
}

#[doc(alias = "boost::char_separator<char,std::char_traits<char>>::is_kept(char)const")]
pub fn stub_0x5c5e9c() -> crate::slot::PortedFn {
// IDA 0x5c5e9c: boost::char_separator<char, std::char_traits<char>>::is_kept(char) const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5c5e9c, "boost::char_separator<char, std::char_traits<char>>::is_kept(char) const")
}

#[doc(alias = "boost::tokenizer_detail::traits_extension_details<std::char_traits<char>,1>::ispunct(char)")]
pub fn stub_0x5c5ecc() -> crate::slot::PortedFn {
// IDA 0x5c5ecc: boost::tokenizer_detail::traits_extension_details<std::char_traits<char>, 1>::ispunct(char).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5c5ecc, "boost::tokenizer_detail::traits_extension_details<std::char_traits<char>, 1>::ispunct(char)")
}

#[doc(alias = "boost::tokenizer_detail::traits_extension_details<std::char_traits<char>,1>::isspace(char)")]
pub fn stub_0x5c5efc() -> crate::slot::PortedFn {
// IDA 0x5c5efc: boost::tokenizer_detail::traits_extension_details<std::char_traits<char>, 1>::isspace(char).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5c5efc, "boost::tokenizer_detail::traits_extension_details<std::char_traits<char>, 1>::isspace(char)")
}

#[doc(alias = "bool boost::detail::lexical_stream_limited_src<char,std::char_traits<char>,false>::shr_signed<long long>(long long &)")]
pub fn stub_0x5c5f2c() -> crate::slot::PortedFn {
// IDA 0x5c5f2c: bool boost::detail::lexical_stream_limited_src<char, std::char_traits<char>, false>::shr_signed<long long>(long long&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5c5f2c, "bool boost::detail::lexical_stream_limited_src<char, std::char_traits<char>, false>::shr_signed<long~")
}

#[doc(alias = "bool boost::detail::lcast_ret_unsigned<std::char_traits<char>,unsigned long long,char>(unsigned long long &,char const*,char const*)")]
pub fn stub_0x5c5fb8() -> crate::slot::PortedFn {
// IDA 0x5c5fb8: bool boost::detail::lcast_ret_unsigned<std::char_traits<char>, unsigned long long, char>(unsigned long long&, char const~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5c5fb8, "bool boost::detail::lcast_ret_unsigned<std::char_traits<char>, unsigned long long, char>(unsigned lo~")
}

#[doc(alias = "bool boost::detail::lexical_stream_limited_src<char,std::char_traits<char>,false>::shr_unsigned<unsigned short>(unsigned short &)")]
pub fn stub_0x5c641c() -> crate::slot::PortedFn {
// IDA 0x5c641c: bool boost::detail::lexical_stream_limited_src<char, std::char_traits<char>, false>::shr_unsigned<unsigned short>(unsign~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5c641c, "bool boost::detail::lexical_stream_limited_src<char, std::char_traits<char>, false>::shr_unsigned<un~")
}

#[doc(alias = "bool boost::detail::lcast_ret_unsigned<std::char_traits<char>,unsigned short,char>(unsigned short &,char const*,char const*)")]
pub fn stub_0x5c6460() -> crate::slot::PortedFn {
// IDA 0x5c6460: bool boost::detail::lcast_ret_unsigned<std::char_traits<char>, unsigned short, char>(unsigned short&, char const*, char ~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5c6460, "bool boost::detail::lcast_ret_unsigned<std::char_traits<char>, unsigned short, char>(unsigned short&~")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::component_impl(void)")]
pub fn stub_0x7debfc() -> crate::slot::PortedFn {
// IDA 0x7debfc: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char, boost::iostreams::output>, std::c~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x7debfc, "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char, boost::iostre~")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::init_get_area(void)")]
pub fn stub_0x7dec00() -> crate::slot::PortedFn {
// IDA 0x7dec00: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char, boost::iostreams::output>, std::c~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x7dec00, "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char, boost::iostre~")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::init_put_area(void)")]
pub fn stub_0x7dec0c() -> crate::slot::PortedFn {
// IDA 0x7dec0c: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char, boost::iostreams::output>, std::c~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x7dec0c, "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char, boost::iostre~")
}

#[doc(alias = "void boost::throw_exception<std::ios_base::failure>(std::ios_base::failure const&)")]
pub fn stub_0x7dec30(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::~clone_impl()")]
pub fn stub_0x7ded0c(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "boost::exception_detail::error_info_injector<std::ios_base::failure>::~error_info_injector()")]
pub fn stub_0x7ded1c(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "boost::exception_detail::error_info_injector<std::ios_base::failure>::~error_info_injector() [0x7ded20]")]
pub fn stub_0x7ded20(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12TweenServiceELZNS_13sTweenServiceEENS_17NonFactoryProductINS_8InstanceELZNS_13sTweenServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x8349f8(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12TweenServiceELZNS_13sTweenServiceEENS_17NonFactoryProductINS_8InstanceELZNS_13sTweenServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x8349fc(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12TweenServiceELZNS_13sTweenServiceEENS_17NonFactoryProductINS_8InstanceELZNS_13sTweenServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x834a9c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12TweenServiceELZNS_13sTweenServiceEENS_17NonFactoryProductINS_8InstanceELZNS_13sTweenServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x834aa4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12TweenServiceELZNS_13sTweenServiceEENS_17NonFactoryProductINS_8InstanceELZNS_13sTweenServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x834b48(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12TweenServiceELZNS_13sTweenServiceEENS_17NonFactoryProductINS_8InstanceELZNS_13sTweenServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x834b50(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_15NotificationBoxELZNS_16sNotificationBoxEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_16sNotificationBoxEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x8366a4(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_15NotificationBoxELZNS_16sNotificationBoxEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_16sNotificationBoxEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x8366a8(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_15NotificationBoxELZNS_16sNotificationBoxEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_16sNotificationBoxEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x836748(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_15NotificationBoxELZNS_16sNotificationBoxEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_16sNotificationBoxEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x836750(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_15NotificationBoxELZNS_16sNotificationBoxEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_16sNotificationBoxEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x8367f4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_15NotificationBoxELZNS_16sNotificationBoxEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_16sNotificationBoxEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x8367fc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_18NotificationObjectELZNS_19sNotificationObjectEENS_14FactoryProductIS2_NS_5FrameELZNS_19sNotificationObjectEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x8383b8(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_18NotificationObjectELZNS_19sNotificationObjectEENS_14FactoryProductIS2_NS_5FrameELZNS_19sNotificationObjectEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x8384b0(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_18NotificationObjectELZNS_19sNotificationObjectEENS_14FactoryProductIS2_NS_5FrameELZNS_19sNotificationObjectEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x8385b8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_18NotificationObjectELZNS_19sNotificationObjectEENS_14FactoryProductIS2_NS_5FrameELZNS_19sNotificationObjectEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x8386ac(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_18NotificationObjectELZNS_19sNotificationObjectEENS_14FactoryProductIS2_NS_5FrameELZNS_19sNotificationObjectEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x8387b8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_18NotificationObjectELZNS_19sNotificationObjectEENS_14FactoryProductIS2_NS_5FrameELZNS_19sNotificationObjectEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x8388ac(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX13FriendServiceEiS1_PKS2_NS0_IFvNS_10shared_ptrIKNS_9unordered13unordered_mapISsNSB_10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsSJ_EEEEEEEEENS0_IFvSsEEEEENS7_5list6INS7_5valueIPSC_EENS11_IiEENS_3argILi1EEENS15_ILi2EEENS11_ISW_EENS11_ISY_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS1C_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x8439d8() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 6 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(6)
}

#[doc(alias = "__ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX13FriendServiceEiS1_PKS2_NS_8functionIFvNS_10shared_ptrIKNS_9unordered13unordered_mapISsNSA_10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsSJ_EEEEEEEEENSE_IFvSsEEEEENS6_5list6INS6_5valueIPSB_EENS11_IiEENS_3argILi1EEENS15_ILi2EEENS11_ISW_EENS11_ISY_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS1C_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x843b38() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 6 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(6)
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13FriendServiceELZNS_14sFriendServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sFriendServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x844e68(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13FriendServiceELZNS_14sFriendServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sFriendServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x844e6c(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13FriendServiceELZNS_14sFriendServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sFriendServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x844f0c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13FriendServiceELZNS_14sFriendServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sFriendServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x844f14(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13FriendServiceELZNS_14sFriendServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sFriendServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x844fb8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13FriendServiceELZNS_14sFriendServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sFriendServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x844fc0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN5boost8functionIFviiN3RBX13FriendService12FriendStatusEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS1_10Reflection18GenericSlotWrapperERKiSE_RKS3_EENS7_5list4INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x84562c() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 8 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(8)
}

#[doc(alias = "__ZN5boost9function3IviiN3RBX13FriendService12FriendStatusEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS1_10Reflection18GenericSlotWrapperERKiSD_RKS3_EENS6_5list4INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x845710() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 8 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(8)
}

#[doc(alias = "RBX::Network::ServerReplicator::createStatsItem(void)")]
pub fn stub_0x9d7028(handle: &crate::slot::InstanceHandle) {
// RBX::Network::ServerReplicator::createStatsItem() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Network::ServerReplicator::canUseProtocolVersion(int)const")]
pub fn stub_0x9d7414(handle: &crate::slot::InstanceHandle) {
// RBX::Network::ServerReplicator::canUseProtocolVersion(int) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Network::ServerReplicator::ServerReplicator(RakNet::SystemAddress,RBX::Network::Server *,RBX::NetworkSettings *)")]
pub fn stub_0x9d7430() -> crate::slot::InstanceHandle {
// RBX::Network::ServerReplicator ctor.
crate::slot::InstanceHandle::new("RBX::Network::ServerReplicator")
}

#[doc(alias = "RBX::Network::ServerReplicator::ServerReplicator(RakNet::SystemAddress,RBX::Network::Server *,RBX::NetworkSettings *) [0x9d744c]")]
pub fn stub_0x9d744c() -> crate::slot::InstanceHandle {
// RBX::Network::ServerReplicator ctor.
crate::slot::InstanceHandle::new("RBX::Network::ServerReplicator")
}

#[doc(alias = "RBX::Network::ServerReplicator::~ServerReplicator()")]
pub fn stub_0x9d7e54(handle: crate::slot::InstanceHandle) {
// RBX::Network::ServerReplicator dtor.
drop(handle);
}

#[doc(alias = "RBX::Network::ServerReplicator::~ServerReplicator() [0x9d7ef4]")]
pub fn stub_0x9d7ef4(handle: crate::slot::InstanceHandle) {
// RBX::Network::ServerReplicator dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::Network::ServerReplicator::~ServerReplicator()")]
pub fn stub_0x9d7f00(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::Network::ServerReplicator::~ServerReplicator() [0x9d7fa4]")]
pub fn stub_0x9d7fa4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::Network::ServerReplicator::~ServerReplicator() [0x9d8048]")]
pub fn stub_0x9d8048(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 1180, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 1180);
}

#[doc(alias = "non-virtual thunk toRBX::Network::ServerReplicator::~ServerReplicator() [0x9d80ec]")]
pub fn stub_0x9d80ec(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 1192, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 1192);
}

#[doc(alias = "RBX::Network::ServerReplicator::~ServerReplicator() [0x9d8190]")]
pub fn stub_0x9d8190(handle: crate::slot::InstanceHandle) {
// RBX::Network::ServerReplicator dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::Network::ServerReplicator::~ServerReplicator() [0x9d86b4]")]
pub fn stub_0x9d86b4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::Network::ServerReplicator::~ServerReplicator() [0x9d86c0]")]
pub fn stub_0x9d86c0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::Network::ServerReplicator::~ServerReplicator() [0x9d86cc]")]
pub fn stub_0x9d86cc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 1180, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 1180);
}

#[doc(alias = "non-virtual thunk toRBX::Network::ServerReplicator::~ServerReplicator() [0x9d86dc]")]
pub fn stub_0x9d86dc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 1192, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 1192);
}

#[doc(alias = "RBX::Network::ServerReplicator::readPlayerSimulationRegion(RBX::Region2::WeightedPoint &)")]
pub fn stub_0x9d8700(handle: &crate::slot::InstanceHandle) {
// RBX::Network::ServerReplicator::readPlayerSimulationRegion(RBX::Region2::WeightedPoint&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Network::ServerReplicator::onSentMarker(long)")]
pub fn stub_0x9dbd20(handle: &crate::slot::InstanceHandle) {
// RBX::Network::ServerReplicator::onSentMarker(long) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Network::ServerReplicator::sendTop(RakNet::RakPeerInterface *)")]
pub fn stub_0x9dbe34(handle: &crate::slot::InstanceHandle) {
// RBX::Network::ServerReplicator::sendTop(RakNet::RakPeerInterface*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Network::ServerReplicator::installRemotePlayer(std::string)")]
pub fn stub_0x9dc8e4(handle: &crate::slot::InstanceHandle) {
// RBX::Network::ServerReplicator::installRemotePlayer(std::string) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Network::ServerReplicator::OnReceive(RakNet::Packet *)")]
pub fn stub_0x9dca6c(handle: &crate::slot::InstanceHandle) {
// RBX::Network::ServerReplicator::OnReceive(RakNet::Packet*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "non-virtual thunk toRBX::Network::ServerReplicator::OnReceive(RakNet::Packet *)")]
pub fn stub_0x9dcbc8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 1180, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 1180);
}

#[doc(alias = "RBX::Network::ServerReplicator::sendItemsPacket(void)")]
pub fn stub_0x9dcbd8(handle: &crate::slot::InstanceHandle) {
// RBX::Network::ServerReplicator::sendItemsPacket() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Network::ServerReplicator::readItem(RakNet::BitStream &,RBX::Network::Item::ItemType)")]
pub fn stub_0x9dcc34(handle: &crate::slot::InstanceHandle) {
// RBX::Network::ServerReplicator::readItem(RakNet::BitStream&, RBX::Network::Item::ItemType) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Network::ServerReplicator::readRequestCharacter(RakNet::BitStream &)")]
pub fn stub_0x9dcfb8(handle: &crate::slot::InstanceHandle) {
// RBX::Network::ServerReplicator::readRequestCharacter(RakNet::BitStream&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Network::ServerReplicator::readPropAcknowledgement(RakNet::BitStream &)")]
pub fn stub_0x9dd5f8(handle: &crate::slot::InstanceHandle) {
// RBX::Network::ServerReplicator::readPropAcknowledgement(RakNet::BitStream&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Network::ServerReplicator::dataOutStep(void)")]
pub fn stub_0x9e0098(handle: &crate::slot::InstanceHandle) {
// RBX::Network::ServerReplicator::dataOutStep() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Network::ServerReplicator::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
pub fn stub_0x9e16cc(handle: &crate::slot::InstanceHandle) {
// RBX::Network::ServerReplicator::onServiceProvider(RBX::ServiceProvider*, RBX::ServiceProvi~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Network::ServerReplicator::serializeSFFlags(RakNet::BitStream &)")]
pub fn stub_0x9e2024(handle: &crate::slot::InstanceHandle) {
// RBX::Network::ServerReplicator::serializeSFFlags(RakNet::BitStream&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::ServerReplicator> RBX::shared_from<RBX::Network::ServerReplicator>(RBX::Network::ServerReplicator*)")]
pub fn stub_0x9e34e0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Network::ServerReplicator")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Replicator::StreamJob>::reset(void)")]
pub fn stub_0x9e5a18(handle: &mut crate::slot::InstanceHandle) {
// shared_ptr::reset — release the owned ref.
let _ = handle;
}

#[doc(alias = "RBX::Network::ServerReplicator::canSendItems(void)")]
pub fn stub_0x9e5bc0(handle: &crate::slot::InstanceHandle) {
// RBX::Network::ServerReplicator::canSendItems() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RakNet::PluginInterface2::OnDetach(void)")]
pub fn stub_0x9e5cc0() -> crate::slot::PortedFn {
// IDA 0x9e5cc0: RakNet::PluginInterface2::OnDetach().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x9e5cc0, "RakNet::PluginInterface2::OnDetach()")
}

#[doc(alias = "RakNet::PluginInterface2::OnPushBackPacket(char const*,unsigned int,RakNet::SystemAddress)")]
pub fn stub_0x9e5cc8() -> crate::slot::PortedFn {
// IDA 0x9e5cc8: RakNet::PluginInterface2::OnPushBackPacket(char const*, unsigned int, RakNet::SystemAddress).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x9e5cc8, "RakNet::PluginInterface2::OnPushBackPacket(char const*, unsigned int, RakNet::SystemAddress)")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Network::FilterResult>::destruct_func(char *)")]
pub fn stub_0x9e5ef8(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::Network::FilterResult>::destruct_func(char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::Replicator::StreamJob,RBX::Network::Replicator::StreamJob>(rbx_core::SharedPtr<RBX::Network::Replicator::StreamJob> *,RBX::Network::Replicator::StreamJob *,boost::detail::shared_count &)")]
pub fn stub_0x9e63f8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Network::Replicator::StreamJob")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::Replicator::StreamJob,RBX::Network::Replicator::StreamJob>(rbx_core::SharedPtr<RBX::Network::Replicator::StreamJob> const*,RBX::Network::Replicator::StreamJob *)const")]
pub fn stub_0x9e65a8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Network::Replicator::StreamJob")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::StreamJob>::~sp_counted_impl_p()")]
pub fn stub_0x9e6854(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::StreamJob>::~sp_counted_impl_p() [0x9e6858]")]
pub fn stub_0x9e6858(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::StreamJob>::dispose(void)")]
pub fn stub_0x9e6864() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::StreamJob>::get_deleter(std::type_info const&)")]
pub fn stub_0x9e6878() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::StreamJob>::get_untyped_deleter(void)")]
pub fn stub_0x9e687c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network16ServerReplicatorELZNS2_17sServerReplicatorEENS_17NonFactoryProductINS2_10ReplicatorELZNS2_17sServerReplicatorEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EEC2IN6RakNet13SystemAddressEN5boost10shared_ptrINS2_17ConcurrentRakPeerEEEPNS_15NetworkSettingsEbEET_T0_T1_T2_")]
pub fn stub_0x9e7928() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Network::ConcurrentRakPeer")
}

#[doc(alias = "RBX::Network::ServerReplicator::ServerStatsItem::ServerStatsItem(rbx_core::SharedPtr<RBX::Network::ServerReplicator const> const&)")]
pub fn stub_0x9e8a8c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Network::ServerReplicator const")
}

#[doc(alias = "RBX::Network::Replicator::StatsItem::~StatsItem()")]
pub fn stub_0x9e9460(handle: crate::slot::InstanceHandle) {
// RBX::Network::Replicator::StatsItem dtor.
drop(handle);
}

#[doc(alias = "RBX::Network::ServerReplicator::ServerStatsItem::~ServerStatsItem()")]
pub fn stub_0x9e967c(handle: crate::slot::InstanceHandle) {
// RBX::Network::ServerReplicator::ServerStatsItem dtor.
drop(handle);
}

#[doc(alias = "RBX::Network::ServerReplicator::ServerStatsItem::~ServerStatsItem() [0x9e9688]")]
pub fn stub_0x9e9688(handle: crate::slot::InstanceHandle) {
// RBX::Network::ServerReplicator::ServerStatsItem dtor.
drop(handle);
}

#[doc(alias = "RBX::Network::ServerReplicator::ServerStatsItem::update(void)")]
pub fn stub_0x9e9728(handle: &crate::slot::InstanceHandle) {
// RBX::Network::ServerReplicator::ServerStatsItem::update() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "non-virtual thunk toRBX::Network::ServerReplicator::ServerStatsItem::~ServerStatsItem()")]
pub fn stub_0x9e9b30(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::Network::ServerReplicator::ServerStatsItem::~ServerStatsItem() [0x9e9b3c]")]
pub fn stub_0x9e9b3c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::Network::ServerReplicator::ServerStatsItem::~ServerStatsItem() [0x9e9be4]")]
pub fn stub_0x9e9be4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}
