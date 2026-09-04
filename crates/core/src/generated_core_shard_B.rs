//! core shard B — 100 core stubs EA-sorted, 0x251058..0x363e78 (RBX::SystemAddress|RBX::TaskScheduler|boost residual, EA-sorted ascending, next 100 uncovered globally, rbx_core::SharedPtr not boost).
//! Source: ida/export.json filtered where demangled/mangled contains SystemAddress|TaskScheduler|boost, EA-sorted ascending, next 100 uncovered (11920 remaining before -> 11820 after, 0x251058..0x363e78).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "boost::gregorian::bad_day_of_month::bad_day_of_month(void)")]
// 0x251058 — __ZN5boost9gregorian16bad_day_of_monthC2Ev
// type: std::out_of_range *__fastcall(std::out_of_range *this)
// was: boost::gregorian::bad_day_of_month::bad_day_of_month(void)
pub fn stub_0x251058() {
    // IDA 0x251058: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::~clone_impl()")]
// 0x2511a0 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEED1Ev
// type: std::logic_error *__fastcall(std::logic_error *, int, int, int, void *, int)
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::~clone_impl()
pub fn stub_0x2511a0() {
    // IDA 0x2511a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>::~error_info_injector()")]
// 0x251258 — __ZN5boost16exception_detail19error_info_injectorINS_9gregorian16bad_day_of_monthEED1Ev
// type: int __fastcall(std::logic_error *, int, int, int, void *, int)
// was: boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>::~error_info_injector()
pub fn stub_0x251258() {
    // IDA 0x251258: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>::~error_info_injector()")]
// 0x251310 — __ZThn8_N5boost16exception_detail19error_info_injectorINS_9gregorian16bad_day_of_monthEED1Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
// was: non-virtual thunk toboost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>::~error_info_injector()
pub fn stub_0x251310() {
    // IDA 0x251310: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::~clone_impl()")]
// 0x2513c8 — __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEED1Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
// was: non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::~clone_impl()
pub fn stub_0x2513c8() {
    // IDA 0x2513c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::~clone_impl()")]
// 0x251480 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEED1Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
// was: virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::~clone_impl()
pub fn stub_0x251480() {
    // IDA 0x251480: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::clone(void)const")]
// 0x251550 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEE5cloneEv
// type: char *__fastcall(int)
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::clone(void)const
pub fn stub_0x251550() {
    // IDA 0x251550: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::~clone_impl()")]
// 0x25160c — __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEED0Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
// was: non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::~clone_impl()
pub fn stub_0x25160c() {
    // IDA 0x25160c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::clone(void)const")]
// 0x2516c8 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEE5cloneEv
// type: char *__fastcall(_DWORD *)
// was: virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::clone(void)const
pub fn stub_0x2516c8() {
    // IDA 0x2516c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::rethrow(void)const")]
// 0x25178c — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEE7rethrowEv
// type: int __fastcall(_DWORD *, int, int, int, void *, int)
// was: virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::rethrow(void)const
pub fn stub_0x25178c() {
    // IDA 0x25178c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::~clone_impl()")]
// 0x25179c — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEED0Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
// was: virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::~clone_impl()
pub fn stub_0x25179c() {
    // IDA 0x25179c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>::~error_info_injector()")]
// 0x251870 — __ZN5boost16exception_detail19error_info_injectorINS_9gregorian16bad_day_of_monthEED0Ev
// type: void __fastcall(std::logic_error *, int, int, int, void *, int)
// was: boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>::~error_info_injector()
pub fn stub_0x251870() {
    // IDA 0x251870: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>::~error_info_injector()")]
// 0x25192c — __ZThn8_N5boost16exception_detail19error_info_injectorINS_9gregorian16bad_day_of_monthEED0Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
// was: non-virtual thunk toboost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>::~error_info_injector()
pub fn stub_0x25192c() {
    // IDA 0x25192c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::clone_tag)")]
// 0x2519e8 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEEC1ERKS6_NS6_9clone_tagE
// type: int __fastcall(int, int, int, int, void *, int)
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::clone_tag)
pub fn stub_0x2519e8() {
    // IDA 0x2519e8: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::clone_impl(boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month> const&)")]
// 0x251b7c — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEEC1ERKS5_
// type: int __fastcall(int, int, int, int, void *, int)
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::clone_impl(boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month> const&)
pub fn stub_0x251b7c() {
    // IDA 0x251b7c: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "boost::CV::simple_exception_policy<unsigned short,(unsigned short)1,(unsigned short)12,boost::gregorian::bad_month>::on_error(unsigned short,unsigned short,boost::CV::violation_enum)")]
// 0x251d10 — __ZN5boost2CV23simple_exception_policyItLt1ELt12ENS_9gregorian9bad_monthEE8on_errorEttNS0_14violation_enumE
// type: void __noreturn()
// was: boost::CV::simple_exception_policy<unsigned short,(unsigned short)1,(unsigned short)12,boost::gregorian::bad_month>::on_error(unsigned short,unsigned short,boost::CV::violation_enum)
pub fn stub_0x251d10() {
    // IDA 0x251d10: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "void boost::throw_exception<boost::gregorian::bad_month>(boost::gregorian::bad_month const&)")]
// 0x251d94 — __ZN5boost15throw_exceptionINS_9gregorian9bad_monthEEEvRKT_
// type: void __fastcall __noreturn(int)
// was: void boost::throw_exception<boost::gregorian::bad_month>(boost::gregorian::bad_month const&)
pub fn stub_0x251d94() {
    // IDA 0x251d94: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "boost::gregorian::bad_month::bad_month(void)")]
// 0x251ee8 — __ZN5boost9gregorian9bad_monthC2Ev
// type: std::out_of_range *__fastcall(std::out_of_range *this)
// was: boost::gregorian::bad_month::bad_month(void)
pub fn stub_0x251ee8() {
    // IDA 0x251ee8: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "boost::gregorian::bad_month::~bad_month()")]
// 0x25202c — __ZN5boost9gregorian9bad_monthD0Ev
// type: void __fastcall(std::logic_error *this)
// was: boost::gregorian::bad_month::~bad_month()
pub fn stub_0x25202c() {
    // IDA 0x25202c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::~clone_impl()")]
// 0x252040 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEED1Ev
// type: std::logic_error *__fastcall(std::logic_error *, int, int, int, void *, int)
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::~clone_impl()
pub fn stub_0x252040() {
    // IDA 0x252040: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::gregorian::bad_month>::~error_info_injector()")]
// 0x2520f8 — __ZN5boost16exception_detail19error_info_injectorINS_9gregorian9bad_monthEED1Ev
// type: int __fastcall(std::logic_error *, int, int, int, void *, int)
// was: boost::exception_detail::error_info_injector<boost::gregorian::bad_month>::~error_info_injector()
pub fn stub_0x2520f8() {
    // IDA 0x2520f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::gregorian::bad_month>::~error_info_injector()")]
// 0x2521b0 — __ZThn8_N5boost16exception_detail19error_info_injectorINS_9gregorian9bad_monthEED1Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
// was: non-virtual thunk toboost::exception_detail::error_info_injector<boost::gregorian::bad_month>::~error_info_injector()
pub fn stub_0x2521b0() {
    // IDA 0x2521b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::~clone_impl()")]
// 0x252268 — __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEED1Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
// was: non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::~clone_impl()
pub fn stub_0x252268() {
    // IDA 0x252268: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::~clone_impl()")]
// 0x252320 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEED1Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
// was: virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::~clone_impl()
pub fn stub_0x252320() {
    // IDA 0x252320: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::~clone_impl()")]
// 0x2523ec — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEED0Ev
// type: void __fastcall(std::logic_error *, int, int, int, void *, int)
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::~clone_impl()
pub fn stub_0x2523ec() {
    // IDA 0x2523ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::clone(void)const")]
// 0x2524a8 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEE5cloneEv
// type: char *__fastcall(int)
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::clone(void)const
pub fn stub_0x2524a8() {
    // IDA 0x2524a8: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::rethrow(void)const")]
// 0x252564 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEE7rethrowEv
// type: void __fastcall __noreturn(int)
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::rethrow(void)const
pub fn stub_0x252564() {
    // IDA 0x252564: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::clone(void)const")]
// 0x252618 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEE5cloneEv
// type: char *__fastcall(_DWORD *)
// was: virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::clone(void)const
pub fn stub_0x252618() {
    // IDA 0x252618: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>> const&)")]
// 0x2526e0 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEEC1ERKS6_
// type: int __fastcall(int, int)
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>> const&)
pub fn stub_0x2526e0() {
    // IDA 0x2526e0: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::gregorian::bad_month>::~error_info_injector()")]
// 0x252820 — __ZN5boost16exception_detail19error_info_injectorINS_9gregorian9bad_monthEED0Ev
// type: void __fastcall(std::logic_error *, int, int, int, void *, int)
// was: boost::exception_detail::error_info_injector<boost::gregorian::bad_month>::~error_info_injector()
pub fn stub_0x252820() {
    // IDA 0x252820: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::clone_tag)")]
// 0x2528e0 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEEC1ERKS6_NS6_9clone_tagE
// type: int __fastcall(int, int, int, int, void *, int)
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::clone_tag)
pub fn stub_0x2528e0() {
    // IDA 0x2528e0: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "boost::CV::simple_exception_policy<unsigned short,(unsigned short)1400,(unsigned short)10000,boost::gregorian::bad_year>::on_error(unsigned short,unsigned short,boost::CV::violation_enum)")]
// 0x252a78 — __ZN5boost2CV23simple_exception_policyItLt1400ELt10000ENS_9gregorian8bad_yearEE8on_errorEttNS0_14violation_enumE
// type: void __noreturn()
// was: boost::CV::simple_exception_policy<unsigned short,(unsigned short)1400,(unsigned short)10000,boost::gregorian::bad_year>::on_error(unsigned short,unsigned short,boost::CV::violation_enum)
pub fn stub_0x252a78() {
    // IDA 0x252a78: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "boost::gregorian::bad_year::~bad_year()")]
// 0x252afc — __ZN5boost9gregorian8bad_yearD1Ev
// type: void __fastcall(std::logic_error *this)
// was: boost::gregorian::bad_year::~bad_year()
pub fn stub_0x252afc() {
    // IDA 0x252afc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::gregorian::bad_year::bad_year(void)")]
// 0x252b08 — __ZN5boost9gregorian8bad_yearC2Ev
// type: std::out_of_range *__fastcall(std::out_of_range *this)
// was: boost::gregorian::bad_year::bad_year(void)
pub fn stub_0x252b08() {
    // IDA 0x252b08: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::~clone_impl()")]
// 0x252c50 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEED1Ev
// type: std::logic_error *__fastcall(std::logic_error *, int, int, int, void *, int)
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::~clone_impl()
pub fn stub_0x252c50() {
    // IDA 0x252c50: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::gregorian::bad_year>::~error_info_injector()")]
// 0x252d08 — __ZN5boost16exception_detail19error_info_injectorINS_9gregorian8bad_yearEED1Ev
// type: int __fastcall(std::logic_error *, int, int, int, void *, int)
// was: boost::exception_detail::error_info_injector<boost::gregorian::bad_year>::~error_info_injector()
pub fn stub_0x252d08() {
    // IDA 0x252d08: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::gregorian::bad_year>::~error_info_injector()")]
// 0x252dc0 — __ZThn8_N5boost16exception_detail19error_info_injectorINS_9gregorian8bad_yearEED1Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
// was: non-virtual thunk toboost::exception_detail::error_info_injector<boost::gregorian::bad_year>::~error_info_injector()
pub fn stub_0x252dc0() {
    // IDA 0x252dc0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::~clone_impl()")]
// 0x252e78 — __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEED1Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
// was: non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::~clone_impl()
pub fn stub_0x252e78() {
    // IDA 0x252e78: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::~clone_impl()")]
// 0x252f30 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEED1Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
// was: virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::~clone_impl()
pub fn stub_0x252f30() {
    // IDA 0x252f30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::clone(void)const")]
// 0x253000 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEE5cloneEv
// type: char *__fastcall(int)
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::clone(void)const
pub fn stub_0x253000() {
    // IDA 0x253000: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::~clone_impl()")]
// 0x2530bc — __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEED0Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
// was: non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::~clone_impl()
pub fn stub_0x2530bc() {
    // IDA 0x2530bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::clone(void)const")]
// 0x253178 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEE5cloneEv
// type: char *__fastcall(_DWORD *)
// was: virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::clone(void)const
pub fn stub_0x253178() {
    // IDA 0x253178: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::rethrow(void)const")]
// 0x25323c — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEE7rethrowEv
// type: int __fastcall(_DWORD *, int, int, int, void *, int)
// was: virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::rethrow(void)const
pub fn stub_0x25323c() {
    // IDA 0x25323c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::~clone_impl()")]
// 0x25324c — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEED0Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
// was: virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::~clone_impl()
pub fn stub_0x25324c() {
    // IDA 0x25324c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::gregorian::bad_year>::~error_info_injector()")]
// 0x253320 — __ZN5boost16exception_detail19error_info_injectorINS_9gregorian8bad_yearEED0Ev
// type: void __fastcall(std::logic_error *, int, int, int, void *, int)
// was: boost::exception_detail::error_info_injector<boost::gregorian::bad_year>::~error_info_injector()
pub fn stub_0x253320() {
    // IDA 0x253320: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::gregorian::bad_year>::~error_info_injector()")]
// 0x2533dc — __ZThn8_N5boost16exception_detail19error_info_injectorINS_9gregorian8bad_yearEED0Ev
// type: void __fastcall(_DWORD *, int, int, int, void *, int)
// was: non-virtual thunk toboost::exception_detail::error_info_injector<boost::gregorian::bad_year>::~error_info_injector()
pub fn stub_0x2533dc() {
    // IDA 0x2533dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::clone_tag)")]
// 0x253498 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEEC1ERKS6_NS6_9clone_tagE
// type: int __fastcall(int, int, int, int, void *, int)
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::clone_tag)
pub fn stub_0x253498() {
    // IDA 0x253498: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::clone_impl(boost::exception_detail::error_info_injector<boost::gregorian::bad_year> const&)")]
// 0x25362c — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEEC1ERKS5_
// type: int __fastcall(int, int, int, int, void *, int)
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::clone_impl(boost::exception_detail::error_info_injector<boost::gregorian::bad_year> const&)
pub fn stub_0x25362c() {
    // IDA 0x25362c: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "boost::gregorian::date::date(boost::gregorian::greg_year,boost::gregorian::greg_month,boost::gregorian::greg_day)")]
// 0x2537c0 — __ZN5boost9gregorian4dateC2ENS0_9greg_yearENS0_10greg_monthENS0_8greg_dayE
// type: _DWORD *__fastcall(_DWORD *, unsigned __int16, unsigned __int16, unsigned __int16)
// was: boost::gregorian::date::date(boost::gregorian::greg_year,boost::gregorian::greg_month,boost::gregorian::greg_day)
pub fn stub_0x2537c0() {
    // IDA 0x2537c0: boost::date_time. std::time-style duration — carrier no-op.
}

#[doc(alias = "boost::thread::do_try_join_until(timespec const&)")]
// 0x2539a0 — __ZN5boost6thread17do_try_join_untilERK8timespec
// type: int __fastcall(boost::thread *this, const timespec *)
// was: boost::thread::do_try_join_until(timespec const&)
pub fn stub_0x2539a0() {
    // IDA 0x2539a0: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "boost::date_time::counted_time_system<boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config>>::subtract_times(boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config> const&,boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config> const&)")]
// 0x253af0 — __ZN5boost9date_time19counted_time_systemINS0_16counted_time_repINS_10posix_time33millisec_posix_time_system_configEEEE14subtract_timesERKS5_S8_
// type: int __fastcall(int result, unsigned int *, __int64 *)
// was: boost::date_time::counted_time_system<boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config>>::subtract_times(boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config> const&,boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config> const&)
pub fn stub_0x253af0() {
    // IDA 0x253af0: boost::date_time. std::time-style duration — carrier no-op.
}

#[doc(alias = "void boost::algorithm::trim_if<std::string,bool (*)(char)>(std::string &,bool (*)(char))")]
// 0x255cc8 — __ZN5boost9algorithm7trim_ifISsPFbcEEEvRT_T0_
// type: int __fastcall(std::string *this, int (__fastcall *)(int))
// was: void boost::algorithm::trim_if<std::string,bool (*)(char)>(std::string &,bool (*)(char))
pub fn stub_0x255cc8() {
    // IDA 0x255cc8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::HttpService::userHttpGetAsync(std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
// 0x255d98 — __ZN3RBX11HttpService16userHttpGetAsyncESsN5boost8functionIFvSsEEES4_
// type: void __fastcall(int, int, int, int)
// was: RBX::HttpService::userHttpGetAsync(std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)
pub fn stub_0x255d98() {
    // IDA 0x255d98: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::HttpService::userHttpPostAsync(std::string,std::string,RBX::HttpService::HttpContentType,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
// 0x25620c — __ZN3RBX11HttpService17userHttpPostAsyncESsSsNS0_15HttpContentTypeEN5boost8functionIFvSsEEES5_
// type: void __fastcall(int, int, int, int, int, int)
// was: RBX::HttpService::userHttpPostAsync(std::string,std::string,RBX::HttpService::HttpContentType,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)
pub fn stub_0x25620c() {
    // IDA 0x25620c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::HttpService::checkEverything(std::string &,boost::function<void ()(std::string)>)")]
// 0x257338 — __ZN3RBX11HttpService15checkEverythingERSsN5boost8functionIFvSsEEE
// type: int __fastcall(RBX::HttpService *, _DWORD *, int, int, int, int, int, int, int, int, int, int, char, int, int, char, char, int, int, char, char, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
// was: RBX::HttpService::checkEverything(std::string &,boost::function<void ()(std::string)>)
pub fn stub_0x257338() {
    // IDA 0x257338: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::operator[](std::string const&)")]
// 0x35c508 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEEixERS5_
// was: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::operator[](std::string const&)
pub fn stub_0x35c508() {
    // IDA 0x35c508: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Name *>>>>::construct_with_value<boost::unordered::detail::emplace_args3<boost::unordered::piecewise_construct_t,boost::tuples::tuple<std::string,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::tuple<boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>>>(boost::unordered::detail::emplace_args3<boost::unordered::piecewise_construct_t,boost::tuples::tuple<std::string,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::tuple<boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>> const&)")]
// 0x35c740 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsPN3RBX4NameEEEEEE20construct_with_valueINS1_13emplace_args3INS0_21piecewise_construct_tENS_6tuples5tupleISsNSG_9null_typeESI_SI_SI_SI_SI_SI_SI_SI_EENSH_ISI_SI_SI_SI_SI_SI_SI_SI_SI_SI_EEEEEEvRKT_
// was: void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Name *>>>>::construct_with_value<boost::unordered::detail::emplace_args3<boost::unordered::piecewise_construct_t,boost::tuples::tuple<std::string,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::tuple<boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>>>(boost::unordered::detail::emplace_args3<boost::unordered::piecewise_construct_t,boost::tuples::tuple<std::string,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::tuple<boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>> const&)
pub fn stub_0x35c740() {
    // IDA 0x35c740: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
// 0x35c764 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)
pub fn stub_0x35c764() {
    // IDA 0x35c764: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Name *>>>>::~node_constructor()")]
// 0x35c7b4 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsPN3RBX4NameEEEEEED2Ev
// was: boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Name *>>>>::~node_constructor()
pub fn stub_0x35c7b4() {
    // IDA 0x35c7b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
// 0x35c7d0 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)
pub fn stub_0x35c7d0() {
    // IDA 0x35c7d0: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const")]
// 0x35c8f8 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const
pub fn stub_0x35c8f8() {
    // IDA 0x35c8f8: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)")]
// 0x35c988 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm
// type: int __fastcall(int)
// was: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)
pub fn stub_0x35c988() {
    // IDA 0x35c988: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")]
// 0x35c9b4 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISF_EEPNS1_10ptr_bucketE
// was: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)
pub fn stub_0x35c9b4() {
    // IDA 0x35c9b4: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Name *>>>>::construct(void)")]
// 0x35ca0c — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsPN3RBX4NameEEEEEE9constructEv
// was: boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Name *>>>>::construct(void)
pub fn stub_0x35ca0c() {
    // IDA 0x35ca0c: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Name *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
// 0x35ca48 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSE_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS9_EEEEmRKT_RKT0_
// was: boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Name *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const
pub fn stub_0x35ca48() {
    // IDA 0x35ca48: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")]
// 0x35cab4 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const
pub fn stub_0x35cab4() {
    // IDA 0x35cab4: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
// 0x35cbec — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)
pub fn stub_0x35cbec() {
    // IDA 0x35cbec: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
// 0x35cc24 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)
pub fn stub_0x35cc24() {
    // IDA 0x35cc24: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Name *>>> const&)")]
// 0x35cc54 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSC_RKSE_RKSaINS1_8ptr_nodeIS9_EEE
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Name *>>> const&)
pub fn stub_0x35cc54() {
    // IDA 0x35cc54: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::PhysicsJob>::operator=(rbx_core::SharedPtr<RBX::PhysicsJob> const&)")]
// 0x3618e0 — __ZN5boost10shared_ptrIN3RBX10PhysicsJobEEaSERKS3_
// was: boost::shared_ptr<RBX::PhysicsJob>::operator=(boost::shared_ptr<RBX::PhysicsJob> const&)
pub fn stub_0x3618e0() {
    // IDA 0x3618e0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::HeartbeatTask>::operator=(rbx_core::SharedPtr<RBX::HeartbeatTask> const&)")]
// 0x361a78 — __ZN5boost10shared_ptrIN3RBX13HeartbeatTaskEEaSERKS3_
// was: boost::shared_ptr<RBX::HeartbeatTask>::operator=(boost::shared_ptr<RBX::HeartbeatTask> const&)
pub fn stub_0x361a78() {
    // IDA 0x361a78: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::RunService> RBX::shared_from<RBX::RunService>(RBX::RunService*)")]
// 0x361ab0 — __ZN3RBX11shared_fromINS_10RunServiceEEEN5boost10shared_ptrIT_EEPS4_
// was: boost::shared_ptr<RBX::RunService> RBX::shared_from<RBX::RunService>(RBX::RunService*)
pub fn stub_0x361ab0() {
    // IDA 0x361ab0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::weak_count::operator=(boost::detail::shared_count const&)")]
// 0x362510 — __ZN5boost6detail10weak_countaSERKNS0_12shared_countE
// was: boost::detail::weak_count::operator=(boost::detail::shared_count const&)
pub fn stub_0x362510() {
    // IDA 0x362510: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::RunTransition)>::slot> &)")]
// 0x3627c4 — __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
// was: rbx::signals::signal<void ()(RBX::RunTransition)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::RunTransition)>::slot> &)
pub fn stub_0x3627c4() {
    // IDA 0x3627c4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::bad_function_call::bad_function_call(void)")]
// 0x362950 — __ZN5boost17bad_function_callC2Ev
// type: _DWORD __fastcall(boost::bad_function_call *__hidden this)
// was: boost::bad_function_call::bad_function_call(void)
pub fn stub_0x362950() {
    // IDA 0x362950: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::~clone_impl()")]
// 0x362a98 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEED1Ev
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::~clone_impl()
pub fn stub_0x362a98() {
    // IDA 0x362a98: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::bad_function_call>::~error_info_injector()")]
// 0x362aa8 — __ZN5boost16exception_detail19error_info_injectorINS_17bad_function_callEED1Ev
// was: boost::exception_detail::error_info_injector<boost::bad_function_call>::~error_info_injector()
pub fn stub_0x362aa8() {
    // IDA 0x362aa8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::~clone_impl()")]
// 0x362ab0 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEED0Ev
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::~clone_impl()
pub fn stub_0x362ab0() {
    // IDA 0x362ab0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::clone_tag)")]
// 0x362ac8 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEEC1ERKS5_NS5_9clone_tagE
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::clone_tag)
pub fn stub_0x362ac8() {
    // IDA 0x362ac8: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::clone_impl(boost::exception_detail::error_info_injector<boost::bad_function_call> const&)")]
// 0x362c30 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEEC1ERKS4_
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::clone_impl(boost::exception_detail::error_info_injector<boost::bad_function_call> const&)
pub fn stub_0x362c30() {
    // IDA 0x362c30: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::RunTransition)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::RunTransition)>::slot> const&)")]
// 0x362d98 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX13RunTransitionEEE4slotEEaSERKS9_
// type: int *__fastcall(int *, int *)
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::RunTransition)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::RunTransition)>::slot> const&)
pub fn stub_0x362d98() {
    // IDA 0x362d98: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::rethrow(void)const")]
// 0x362dc0 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEE7rethrowEv
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::rethrow(void)const
pub fn stub_0x362dc0() {
    // IDA 0x362dc0: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::rethrow(void)const")]
// 0x362f88 — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEE7rethrowEv
// was: virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::rethrow(void)const
pub fn stub_0x362f88() {
    // IDA 0x362f88: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::~clone_impl()")]
// 0x362f98 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEED0Ev
// was: virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::~clone_impl()
pub fn stub_0x362f98() {
    // IDA 0x362f98: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::lock_error>::~error_info_injector()")]
// 0x362fb8 — __ZThn20_N5boost16exception_detail19error_info_injectorINS_10lock_errorEED0Ev
// was: non-virtual thunk toboost::exception_detail::error_info_injector<boost::lock_error>::~error_info_injector()
pub fn stub_0x362fb8() {
    // IDA 0x362fb8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::clone_tag)")]
// 0x362fd0 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEEC1ERKS5_NS5_9clone_tagE
// type: int __fastcall(int, int, int, int, std::exception *, std::string *, int, int, int, int)
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::clone_tag)
pub fn stub_0x362fd0() {
    // IDA 0x362fd0: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "void rbx_core::SharedPtr_add_ref<rbx::signals::connection::islot,int,0,0>(rbx::intrusive_ptr_target<rbx::signals::connection::islot,int,0,0> const*)")]
// 0x3631b0 — __ZN5boost21intrusive_ptr_add_refIN3rbx7signals10connection5islotEiLi0ELi0EEEvPKNS1_20intrusive_ptr_targetIT_T0_XT1_EXT2_EEE
// was: void boost::intrusive_ptr_add_ref<rbx::signals::connection::islot,int,0,0>(rbx::intrusive_ptr_target<rbx::signals::connection::islot,int,0,0> const*)
pub fn stub_0x3631b0() {
    // IDA 0x3631b0: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(double,double)>::slot> &)")]
// 0x363224 — __ZN3rbx7signals6signalIFvddEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
// was: rbx::signals::signal<void ()(double,double)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(double,double)>::slot> &)
pub fn stub_0x363224() {
    // IDA 0x363224: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(double,double)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(double,double)>::slot> const&)")]
// 0x3633ac — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvddEE4slotEEaSERKS7_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(double,double)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(double,double)>::slot> const&)
pub fn stub_0x3633ac() {
    // IDA 0x3633ac: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot> &)")]
// 0x3634cc — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4nextERN5boost13intrusive_ptrINS7_4slotEEE
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
// was: rbx::signals::signal<void ()(RBX::Stepped const&)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot> &)
pub fn stub_0x3634cc() {
    // IDA 0x3634cc: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot> const&)")]
// 0x363654 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slotEEaSERKSB_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot> const&)
pub fn stub_0x363654() {
    // IDA 0x363654: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(double)>::slot> &)")]
// 0x363774 — __ZN3rbx7signals6signalIFvdEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
// was: rbx::signals::signal<void ()(double)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(double)>::slot> &)
pub fn stub_0x363774() {
    // IDA 0x363774: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(double)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(double)>::slot> const&)")]
// 0x3638fc — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvdEE4slotEEaSERKS7_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(double)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(double)>::slot> const&)
pub fn stub_0x3638fc() {
    // IDA 0x3638fc: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot> &)")]
// 0x363a1c — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4nextERN5boost13intrusive_ptrINS7_4slotEEE
// type: int __fastcall(int, int *, int, int, char, int, int, int, int, int)
// was: rbx::signals::signal<void ()(RBX::Heartbeat const&)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot> &)
pub fn stub_0x363a1c() {
    // IDA 0x363a1c: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::HeartbeatTask>::shared_ptr<RBX::HeartbeatTask>(RBX::HeartbeatTask *)")]
// 0x363bac — __ZN5boost10shared_ptrIN3RBX13HeartbeatTaskEEC2IS2_EEPT_
// type: int __fastcall(int, void *, int, int, int, int)
// was: boost::shared_ptr<RBX::HeartbeatTask>::shared_ptr<RBX::HeartbeatTask>(RBX::HeartbeatTask *)
pub fn stub_0x363bac() {
    // IDA 0x363bac: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::HeartbeatTask,RBX::HeartbeatTask>(rbx_core::SharedPtr<RBX::HeartbeatTask> const*,RBX::HeartbeatTask *)const")]
// 0x363c94 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_13HeartbeatTaskES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::HeartbeatTask,RBX::HeartbeatTask>(boost::shared_ptr<RBX::HeartbeatTask> const*,RBX::HeartbeatTask *)const
pub fn stub_0x363c94() {
    // IDA 0x363c94: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::HeartbeatTask>(RBX::HeartbeatTask *)")]
// 0x363d78 — __ZN5boost6detail12shared_countC2IN3RBX13HeartbeatTaskEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
// was: boost::detail::shared_count::shared_count<RBX::HeartbeatTask>(RBX::HeartbeatTask *)
pub fn stub_0x363d78() {
    // IDA 0x363d78: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HeartbeatTask>::~sp_counted_impl_p()")]
// 0x363e70 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13HeartbeatTaskEED1Ev
// was: boost::detail::sp_counted_impl_p<RBX::HeartbeatTask>::~sp_counted_impl_p()
pub fn stub_0x363e70() {
    // IDA 0x363e70: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HeartbeatTask>::~sp_counted_impl_p()")]
// 0x363e74 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13HeartbeatTaskEED0Ev
// was: boost::detail::sp_counted_impl_p<RBX::HeartbeatTask>::~sp_counted_impl_p()
pub fn stub_0x363e74() {
    // IDA 0x363e74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HeartbeatTask>::dispose(void)")]
// 0x363e78 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13HeartbeatTaskEE7disposeEv
// was: boost::detail::sp_counted_impl_p<RBX::HeartbeatTask>::dispose(void)
pub fn stub_0x363e78() {
    // IDA 0x363e78: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

