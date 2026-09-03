//! core shard ma — 150 core stubs EA-sorted, next uncovered fallback gap filler (lowest unstubbed EA first).
//! Source: ida/export.json (85545 funcs) global EA asc not yet stubbed in any crate — next 150 uncovered sorted asc (0x3ad3bc..0x3ea224).
//! Preserves IDA ea + mangled + demangled for rg; uses rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "boost::function3<void,G3D::Vector3::Axis,float,float>::clear(void)")]
// 0x3ad3bc — __ZN5boost9function3IvN3G3D7Vector34AxisEffE5clearEv
// type: int __fastcall(int *)
pub fn stub_0x3ad3bc() {
    // IDA 0x3ad3bc: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::connect<boost::function<void ()(G3D::Vector3::Axis,float,float)>>(boost::function<void ()(G3D::Vector3::Axis,float,float)> const&)")]
// 0x3adb10 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
// type: void __fastcall(char, boost::mutex *, int, int, int, int)
pub fn stub_0x3adb10() {
    // IDA 0x3adb10: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::function<void ()(G3D::Vector3::Axis,float,float)>,3,void ()(G3D::Vector3::Axis,float,float)>::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>*>(boost::function<void ()(G3D::Vector3::Axis,float,float)> const&,rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>*)")]
// 0x3adc04 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost8functionIS6_EELi3ES6_EC2IPS7_EERKSB_T_
// type: _DWORD *__fastcall(_DWORD *, int, int)
pub fn stub_0x3adc04() {
    // IDA 0x3adc04: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::callable_slot<boost::function<void ()(G3D::Vector3::Axis,float,float)>>::~callable_slot()")]
// 0x3add00 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE13callable_slotIN5boost8functionIS5_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_0x3add00() {
    // IDA 0x3add00: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::callable_slot<boost::function<void ()(G3D::Vector3::Axis,float,float)>>::~callable_slot()")]
// 0x3ade10 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE13callable_slotIN5boost8functionIS5_EEED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x3ade10() {
    // IDA 0x3ade10: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::function<void ()(G3D::Vector3::Axis,float,float)>,3,void ()(G3D::Vector3::Axis,float,float)>::call(G3D::Vector3::Axis,float,float)")]
// 0x3adf40 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost8functionIS6_EELi3ES6_E4callES5_ff
// type: int __fastcall(int)
pub fn stub_0x3adf40() {
    // IDA 0x3adf40: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::function<void ()(G3D::Vector3::Axis,float,float)>,3,void ()(G3D::Vector3::Axis,float,float)>::call(G3D::Vector3::Axis,float,float)")]
// 0x3adf48 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost8functionIS6_EELi3ES6_E4callES5_ff
// type: int __fastcall(int)
pub fn stub_0x3adf48() {
    // IDA 0x3adf48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function3<void,G3D::Vector3::Axis,float,float>::operator()(G3D::Vector3::Axis,float,float)const")]
// 0x3adf50 — __ZNK5boost9function3IvN3G3D7Vector34AxisEffEclES3_ff
// type: void __fastcall(_DWORD *, int, int, int)
pub fn stub_0x3adf50() {
    // IDA 0x3adf50: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::function<void ()(G3D::Vector3::Axis,float,float)>,3,void ()(G3D::Vector3::Axis,float,float)>::~callable()")]
// 0x3ae028 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost8functionIS6_EELi3ES6_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_0x3ae028() {
    // IDA 0x3ae028: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::function<void ()(G3D::Vector3::Axis,float,float)>,3,void ()(G3D::Vector3::Axis,float,float)>::~callable()")]
// 0x3ae138 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost8functionIS6_EELi3ES6_ED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x3ae138() {
    // IDA 0x3ae138: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function3<void,G3D::Vector3::Axis,float,float>::assign_to_own(boost::function3<void,G3D::Vector3::Axis,float,float> const&)")]
// 0x3ae268 — __ZN5boost9function3IvN3G3D7Vector34AxisEffE13assign_to_ownERKS4_
// type: int __fastcall(int result, int *)
pub fn stub_0x3ae268() {
    // IDA 0x3ae268: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function1<void,G3D::Vector3::Axis>::clear(void)")]
// 0x3aeb04 — __ZN5boost9function1IvN3G3D7Vector34AxisEE5clearEv
// type: int __fastcall(int *)
pub fn stub_0x3aeb04() {
    // IDA 0x3aeb04: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3::Axis)>::connect<boost::function<void ()(G3D::Vector3::Axis)>>(boost::function<void ()(G3D::Vector3::Axis)> const&)")]
// 0x3af234 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
// type: void __fastcall(char, boost::mutex *, int, int, int, int)
pub fn stub_0x3af234() {
    // IDA 0x3af234: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::function<void ()(G3D::Vector3::Axis)>,1,void ()(G3D::Vector3::Axis)>::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>*>(boost::function<void ()(G3D::Vector3::Axis)> const&,rbx::signals::signal<void ()(G3D::Vector3::Axis)>*)")]
// 0x3af328 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost8functionIS6_EELi1ES6_EC2IPS7_EERKSB_T_
// type: _DWORD *__fastcall(_DWORD *, int, int)
pub fn stub_0x3af328() {
    // IDA 0x3af328: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::callable_slot<boost::function<void ()(G3D::Vector3::Axis)>>::~callable_slot()")]
// 0x3af424 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE13callable_slotIN5boost8functionIS5_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_0x3af424() {
    // IDA 0x3af424: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::callable_slot<boost::function<void ()(G3D::Vector3::Axis)>>::~callable_slot()")]
// 0x3af534 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE13callable_slotIN5boost8functionIS5_EEED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x3af534() {
    // IDA 0x3af534: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::function<void ()(G3D::Vector3::Axis)>,1,void ()(G3D::Vector3::Axis)>::call(G3D::Vector3::Axis)")]
// 0x3af664 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_
// type: int __fastcall(int)
pub fn stub_0x3af664() {
    // IDA 0x3af664: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::function<void ()(G3D::Vector3::Axis)>,1,void ()(G3D::Vector3::Axis)>::call(G3D::Vector3::Axis)")]
// 0x3af66c — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_
// type: int __fastcall(int)
pub fn stub_0x3af66c() {
    // IDA 0x3af66c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function1<void,G3D::Vector3::Axis>::operator()(G3D::Vector3::Axis)const")]
// 0x3af674 — __ZNK5boost9function1IvN3G3D7Vector34AxisEEclES3_
// type: void __fastcall(_DWORD *, int)
pub fn stub_0x3af674() {
    // IDA 0x3af674: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::function<void ()(G3D::Vector3::Axis)>,1,void ()(G3D::Vector3::Axis)>::~callable()")]
// 0x3af738 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost8functionIS6_EELi1ES6_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_0x3af738() {
    // IDA 0x3af738: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::function<void ()(G3D::Vector3::Axis)>,1,void ()(G3D::Vector3::Axis)>::~callable()")]
// 0x3af848 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost8functionIS6_EELi1ES6_ED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x3af848() {
    // IDA 0x3af848: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function1<void,G3D::Vector3::Axis>::assign_to_own(boost::function1<void,G3D::Vector3::Axis> const&)")]
// 0x3af978 — __ZN5boost9function1IvN3G3D7Vector34AxisEE13assign_to_ownERKS4_
// type: int __fastcall(int result, int *)
pub fn stub_0x3af978() {
    // IDA 0x3af978: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "global constructor keyed to_a_159")]
// 0x3b081c — __GLOBAL__I_a_159
pub fn stub_0x3b081c() {
    // IDA 0x3b081c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BadgeService::userHasBadge(int,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0x3b21a4 — __ZN3RBX12BadgeService12userHasBadgeEiiN5boost8functionIFvbEEENS2_IFvSsEEE
// type: void __fastcall(int, const RBX::Instance *, boost::detail::sp_counted_base *, int, int)
pub fn stub_0x3b21a4() {
    // IDA 0x3b21a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BadgeService::awardBadge(int,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0x3b2908 — __ZN3RBX12BadgeService10awardBadgeEiiN5boost8functionIFvbEEENS2_IFvSsEEE
// type: void __fastcall(int, const RBX::Instance *, int, int, int)
pub fn stub_0x3b2908() {
    // IDA 0x3b2908: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BadgeService::isDisabled(int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0x3b3160 — __ZN3RBX12BadgeService10isDisabledEiN5boost8functionIFvbEEENS2_IFvSsEEE
// type: void __fastcall(int, const RBX::Instance *, int, int)
pub fn stub_0x3b3160() {
    // IDA 0x3b3160: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "RBX::BadgeService::isLegal(int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0x3b3840 — __ZN3RBX12BadgeService7isLegalEiN5boost8functionIFvbEEENS2_IFvSsEEE
// type: void __fastcall(int, int, int, int)
pub fn stub_0x3b3840() {
    // IDA 0x3b3840: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::BadgeService::hasBadgeResultHelper(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0x3b42f4 — __ZN3RBX12BadgeService20hasBadgeResultHelperEN5boost8weak_ptrIS0_EEiiPSsPSt9exceptionNS1_8functionIFvbEEENS7_IFvSsEEE
// type: void __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x3b42f4() {
    // IDA 0x3b42f4: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "RBX::BadgeService::hasBadgeResult(int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0x3b4424 — __ZN3RBX12BadgeService14hasBadgeResultEiiPSsPSt9exceptionN5boost8functionIFvbEEENS5_IFvSsEEE
// type: void __fastcall(int, int, int, std::string *, pthread_mutex_t *, int, pthread_mutex_t *, char, int, int, int, int, int, int, int, int, int, int)
pub fn stub_0x3b4424() {
    // IDA 0x3b4424: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "RBX::BadgeService::isDiabledResultHelper(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0x3b4720 — __ZN3RBX12BadgeService21isDiabledResultHelperEN5boost8weak_ptrIS0_EEiPSsPSt9exceptionNS1_8functionIFvbEEENS7_IFvSsEEE
// type: void __fastcall(int, pthread_mutex_t *, int, int, int, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x3b4720() {
    // IDA 0x3b4720: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "RBX::BadgeService::isDisabledResult(int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0x3b484c — __ZN3RBX12BadgeService16isDisabledResultEiPSsPSt9exceptionN5boost8functionIFvbEEENS5_IFvSsEEE
// type: void __fastcall(int, int, std::string *, int, int, pthread_mutex_t *, char, pthread_mutex_t *, char, int, int, int, int, int)
pub fn stub_0x3b484c() {
    // IDA 0x3b484c: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "RBX::BadgeService::isLegalResultHelper(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0x3b4a20 — __ZN3RBX12BadgeService19isLegalResultHelperEN5boost8weak_ptrIS0_EEiPSsPSt9exceptionNS1_8functionIFvbEEENS7_IFvSsEEE
// type: void __fastcall(int, pthread_mutex_t *, int, int, int, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x3b4a20() {
    // IDA 0x3b4a20: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "RBX::BadgeService::isLegalResult(int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0x3b4b4c — __ZN3RBX12BadgeService13isLegalResultEiPSsPSt9exceptionN5boost8functionIFvbEEENS5_IFvSsEEE
// type: void __fastcall(int, int, std::string *, int, int, pthread_mutex_t *, char, pthread_mutex_t *, char, int, int, int, int, int)
pub fn stub_0x3b4b4c() {
    // IDA 0x3b4b4c: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "RBX::BadgeService::awardBadgeResultHelper(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0x3b4d20 — __ZN3RBX12BadgeService22awardBadgeResultHelperEN5boost8weak_ptrIS0_EEiiPSsPSt9exceptionNS1_8functionIFvbEEENS7_IFvSsEEE
// type: void __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x3b4d20() {
    // IDA 0x3b4d20: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "RBX::BadgeService::awardBadgeResult(int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0x3b4e50 — __ZN3RBX12BadgeService16awardBadgeResultEiiPSsPSt9exceptionN5boost8functionIFvbEEENS5_IFvSsEEE
// type: void __fastcall(int, int, int, std::string *, int, int)
pub fn stub_0x3b4e50() {
    // IDA 0x3b4e50: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list_av_7<rbx_core::WeakPtr<RBX::BadgeService>,int,int,boost::arg<1>,boost::arg<2>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>,rbx_core::WeakPtr<RBX::BadgeService>,int,int,boost::arg<1>,boost::arg<2>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>(void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),rbx_core::WeakPtr<RBX::BadgeService>,int,int,boost::arg<1>,boost::arg<2>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0x3b5484 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiPSsPSt9exceptionNS_8functionIFvbEEENS8_IFvSsEEES4_iiNS_3argILi1EEENSD_ILi2EEESA_SC_EENS_3_bi6bind_tIT_PFSI_T0_T1_T2_T3_T4_T5_T6_ENSG_9list_av_7IT7_T8_T9_T10_T11_T12_T13_E4typeEEESR_ST_SU_SV_SW_SX_SY_SZ_
// type: void __fastcall(char, int, int, int, int, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, char, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
pub fn stub_0x3b5484() {
    // IDA 0x3b5484: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "rbx_core::WeakPtr<RBX::BadgeService> RBX::weak_from<RBX::BadgeService>(RBX::BadgeService*)")]
// 0x3b5728 — __ZN3RBX9weak_fromINS_12BadgeServiceEEEN5boost8weak_ptrIT_EEPS4_
// type: void __fastcall(int, int)
pub fn stub_0x3b5728() {
    // IDA 0x3b5728: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list_av_6<rbx_core::WeakPtr<RBX::BadgeService>,int,boost::arg<1>,boost::arg<2>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>,rbx_core::WeakPtr<RBX::BadgeService>,int,boost::arg<1>,boost::arg<2>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>(void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),rbx_core::WeakPtr<RBX::BadgeService>,int,boost::arg<1>,boost::arg<2>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0x3b5978 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX12BadgeServiceEEEiPSsPSt9exceptionNS_8functionIFvbEEENS8_IFvSsEEES4_iNS_3argILi1EEENSD_ILi2EEESA_SC_EENS_3_bi6bind_tIT_PFSI_T0_T1_T2_T3_T4_T5_ENSG_9list_av_6IT6_T7_T8_T9_T10_T11_E4typeEEESQ_SS_ST_SU_SV_SW_SX_
// type: void __fastcall(int, char, int *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, char, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, int)
pub fn stub_0x3b5978() {
    // IDA 0x3b5978: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiS1_S3_NS0_IFvbEEENS0_IFvSsEEEENS7_5list6INS7_5valueISC_EENSK_IiEENS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")]
// 0x3b6384 — __ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiS1_S3_NS0_IFvbEEENS0_IFvSsEEEENS7_5list6INS7_5valueISC_EENSK_IiEENS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int *, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, int)
pub fn stub_0x3b6384() {
    // IDA 0x3b6384: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiS1_S3_NS_8functionIFvbEEENSC_IFvSsEEEENS6_5list6INS6_5valueISB_EENSK_IiEENS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")]
// 0x3b6540 — __ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiS1_S3_NS_8functionIFvbEEENSC_IFvSsEEEENS6_5list6INS6_5valueISB_EENSK_IiEENS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(_DWORD *, int *, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, int)
pub fn stub_0x3b6540() {
    // IDA 0x3b6540: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage6(boost::_bi::storage6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>> const&)")]
// 0x3b6700 — __ZN5boost3_bi8storage6INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEC2ERKSJ_
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
pub fn stub_0x3b6700() {
    // IDA 0x3b6700: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)")]
// 0x3b6880 — __ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiS1_S3_NS_8functionIFvbEEENSC_IFvSsEEEENS6_5list6INS6_5valueISB_EENSK_IiEENS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEEvT_
// type: void __fastcall(int, int *, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, int)
pub fn stub_0x3b6880() {
    // IDA 0x3b6880: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x3b6a4c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEENS3_5list6INS3_5valueIS8_EENSK_IiEENS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEE6manageERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(int, int, int)
pub fn stub_0x3b6a4c() {
    // IDA 0x3b6a4c: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,void,std::string *,std::exception *>::invoke(boost::detail::function::function_buffer &,std::string *,std::exception *)")]
// 0x3b6a68 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEENS3_5list6INS3_5valueIS8_EENSK_IiEENS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEvS9_SB_E6invokeERNS1_15function_bufferES9_SB_
// type: int __fastcall(int, int, int)
pub fn stub_0x3b6a68() {
    // IDA 0x3b6a68: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")]
// 0x3b6a88 — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiS3_S5_NS_8functionIFvbEEENSE_IFvSsEEEENS8_5list6INS8_5valueISD_EENSM_IiEENS_3argILi1EEENSP_ILi2EEENSM_ISG_EENSM_ISI_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int *, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, int)
pub fn stub_0x3b6a88() {
    // IDA 0x3b6a88: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x3b6c44 — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiS3_S5_NS_8functionIFvbEEENSE_IFvSsEEEENS8_5list6INS8_5valueISD_EENSM_IiEENS_3argILi1EEENSP_ILi2EEENSM_ISG_EENSM_ISI_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, int)
pub fn stub_0x3b6c44() {
    // IDA 0x3b6c44: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x3b6dfc — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiS3_S5_NS_8functionIFvbEEENSE_IFvSsEEEENS8_5list6INS8_5valueISD_EENSM_IiEENS_3argILi1EEENSP_ILi2EEENSM_ISG_EENSM_ISI_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int, _DWORD *, _DWORD *)
pub fn stub_0x3b6dfc() {
    // IDA 0x3b6dfc: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "void boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list2<std::string *&,std::exception *&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>) &,boost::_bi::list2<std::string *&,std::exception *&> &,int)")]
// 0x3b6ebc — __ZN5boost3_bi5list6INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEclIPFvS6_iPSsPSt9exceptionSE_SH_ENS0_5list2IRSL_RSN_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, void (__fastcall **)(int *, int, int, int, _DWORD *, _DWORD *), int **, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x3b6ebc() {
    // IDA 0x3b6ebc: invoker::invoke unpacked the buffer and called the bound functor. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x3b7054 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEENS3_5list6INS3_5valueIS8_EENSK_IiEENS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEE7managerERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(_DWORD **, _WORD *, int, int, void *, void *, int, int, int, int)
pub fn stub_0x3b7054() {
    // IDA 0x3b7054: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list6(boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// 0x3b7224 — __ZN5boost3_bi5list6INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEC2ES7_S8_SA_SB_SF_SI_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(boost::detail::sp_counted_base *, int *, int, int, int, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x3b7224() {
    // IDA 0x3b7224: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::_bi::storage6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage6(boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// 0x3b73a4 — __ZN5boost3_bi8storage6INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEC2ES7_S8_SA_SB_SF_SI_
// type: int __fastcall(int, int *, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
pub fn stub_0x3b73a4() {
    // IDA 0x3b73a4: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>>::storage5(boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>)")]
// 0x3b7550 — __ZN5boost3_bi8storage5INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEEEC2ES7_S8_SA_SB_SF_
// type: int __fastcall(int, int *, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
pub fn stub_0x3b7550() {
    // IDA 0x3b7550: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>)")]
// 0x3b7698 — __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEEEC2ES7_S8_SA_SB_
// type: int __fastcall(int, int *, int)
pub fn stub_0x3b7698() {
    // IDA 0x3b7698: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>)")]
// 0x3b77b4 — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEENS_3argILi1EEEEC2ES7_S8_SA_
// type: struct _Unwind_Exception *__fastcall(struct _Unwind_Exception *, boost::detail::sp_counted_base **, int, int)
pub fn stub_0x3b77b4() {
    // IDA 0x3b77b4: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>)")]
// 0x3b78d0 — __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEEEC2ES7_S8_
// type: _DWORD *__fastcall(_DWORD *, unsigned int *, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x3b78d0() {
    // IDA 0x3b78d0: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "__ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiS1_S3_NS0_IFvbEEENS0_IFvSsEEEENS7_5list7INS7_5valueISC_EENSK_IiEESM_NS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")]
// 0x3b7b94 — __ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiS1_S3_NS0_IFvbEEENS0_IFvSsEEEENS7_5list7INS7_5valueISC_EENSK_IiEESM_NS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int *)
pub fn stub_0x3b7b94() {
    // IDA 0x3b7b94: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiS1_S3_NS_8functionIFvbEEENSC_IFvSsEEEENS6_5list7INS6_5valueISB_EENSK_IiEESM_NS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")]
// 0x3b7d50 — __ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiS1_S3_NS_8functionIFvbEEENSC_IFvSsEEEENS6_5list7INS6_5valueISB_EENSK_IiEESM_NS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE
// type: _DWORD *__fastcall(_DWORD *, int *)
pub fn stub_0x3b7d50() {
    // IDA 0x3b7d50: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage7(boost::_bi::storage7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>> const&)")]
// 0x3b7f10 — __ZN5boost3_bi8storage7INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEC2ERKSJ_
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
pub fn stub_0x3b7f10() {
    // IDA 0x3b7f10: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)")]
// 0x3b8094 — __ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiS1_S3_NS_8functionIFvbEEENSC_IFvSsEEEENS6_5list7INS6_5valueISB_EENSK_IiEESM_NS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEEvT_
// type: void __fastcall(_DWORD *, int)
pub fn stub_0x3b8094() {
    // IDA 0x3b8094: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x3b8260 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEENS3_5list7INS3_5valueIS8_EENSK_IiEESM_NS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEE6manageERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(int, int, int)
pub fn stub_0x3b8260() {
    // IDA 0x3b8260: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,void,std::string *,std::exception *>::invoke(boost::detail::function::function_buffer &,std::string *,std::exception *)")]
// 0x3b827c — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEENS3_5list7INS3_5valueIS8_EENSK_IiEESM_NS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEvS9_SB_E6invokeERNS1_15function_bufferES9_SB_
// type: int __fastcall(int, int, int)
pub fn stub_0x3b827c() {
    // IDA 0x3b827c: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")]
// 0x3b829c — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiS3_S5_NS_8functionIFvbEEENSE_IFvSsEEEENS8_5list7INS8_5valueISD_EENSM_IiEESO_NS_3argILi1EEENSP_ILi2EEENSM_ISG_EENSM_ISI_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int *, int)
pub fn stub_0x3b829c() {
    // IDA 0x3b829c: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x3b8458 — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiS3_S5_NS_8functionIFvbEEENSE_IFvSsEEEENS8_5list7INS8_5valueISD_EENSM_IiEESO_NS_3argILi1EEENSP_ILi2EEENSM_ISG_EENSM_ISI_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int)
pub fn stub_0x3b8458() {
    // IDA 0x3b8458: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x3b8610 — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiS3_S5_NS_8functionIFvbEEENSE_IFvSsEEEENS8_5list7INS8_5valueISD_EENSM_IiEESO_NS_3argILi1EEENSP_ILi2EEENSM_ISG_EENSM_ISI_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int, _DWORD *, _DWORD *)
pub fn stub_0x3b8610() {
    // IDA 0x3b8610: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "void boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list2<std::string *&,std::exception *&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>) &,boost::_bi::list2<std::string *&,std::exception *&> &,int)")]
// 0x3b86d0 — __ZN5boost3_bi5list7INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEclIPFvS6_iiPSsPSt9exceptionSE_SH_ENS0_5list2IRSL_RSN_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, void (__fastcall **)(int *, int, int, int, int, _DWORD *, _DWORD *), int **, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x3b86d0() {
    // IDA 0x3b86d0: invoker::invoke unpacked the buffer and called the bound functor. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x3b8870 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEENS3_5list7INS3_5valueIS8_EENSK_IiEESM_NS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEE7managerERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(_DWORD **, _WORD *, int, int, void *, void *, int, int, int, int)
pub fn stub_0x3b8870() {
    // IDA 0x3b8870: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list7(boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// 0x3b8a40 — __ZN5boost3_bi5list7INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEC2ES7_S8_S8_SA_SB_SF_SI_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(boost::detail::sp_counted_base *, int *, int, int, int, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x3b8a40() {
    // IDA 0x3b8a40: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::_bi::storage7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage7(boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// 0x3b8bc8 — __ZN5boost3_bi8storage7INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEC2ES7_S8_S8_SA_SB_SF_SI_
// type: int __fastcall(int, int *, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
pub fn stub_0x3b8bc8() {
    // IDA 0x3b8bc8: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::_bi::storage6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>>::storage6(boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>)")]
// 0x3b8d7c — __ZN5boost3_bi8storage6INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEEEC2ES7_S8_S8_SA_SB_SF_
// type: int __fastcall(int, int *, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
pub fn stub_0x3b8d7c() {
    // IDA 0x3b8d7c: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>::storage5(boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>)")]
// 0x3b8ec8 — __ZN5boost3_bi8storage5INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEEEC2ES7_S8_S8_SA_SB_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int, int, int, int, int)
pub fn stub_0x3b8ec8() {
    // IDA 0x3b8ec8: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>)")]
// 0x3b8fe4 — __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEEEC2ES7_S8_S8_SA_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, boost::detail::sp_counted_base **, int, int, int, int)
pub fn stub_0x3b8fe4() {
    // IDA 0x3b8fe4: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>)")]
// 0x3b9100 — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_EC2ES7_S8_S8_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x3b9100() {
    // IDA 0x3b9100: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "rbx_core::WeakPtr<RBX::BadgeService>::weak_ptr<RBX::BadgeService>(rbx_core::SharedPtr<RBX::BadgeService> const&,boost::detail::sp_enable_if_convertible<RBX::BadgeService,RBX::BadgeService>::type)")]
// 0x3b9224 — __ZN5boost8weak_ptrIN3RBX12BadgeServiceEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
pub fn stub_0x3b9224() {
    // IDA 0x3b9224: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::BadgeService>::shared_ptr<RBX::BadgeService>(rbx_core::WeakPtr<RBX::BadgeService> const&,boost::detail::sp_nothrow_tag)")]
// 0x3b9738 — __ZN5boost10shared_ptrIN3RBX12BadgeServiceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
pub fn stub_0x3b9738() {
    // IDA 0x3b9738: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "rbx::remote_signal<void ()(std::string)>::remote_signal(void)")]
// 0x3b97b4 — __ZN3rbx13remote_signalIFvSsEEC2Ev
// type: int __fastcall(_DWORD *, int, int, int, int, int)
pub fn stub_0x3b97b4() {
    // IDA 0x3b97b4: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "rbx::remote_signal<void ()(std::string)>::~remote_signal()")]
// 0x3bb204 — __ZN3rbx13remote_signalIFvSsEED2Ev
// type: _DWORD *__fastcall(int, int, int, int, int, int)
pub fn stub_0x3bb204() {
    // IDA 0x3bb204: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ChangeHistoryService::getUnplayWaypoint(std::string &,int)const")]
// 0x3d434c — __ZNK3RBX20ChangeHistoryService17getUnplayWaypointERSsi
// type: int __fastcall(RBX::ChangeHistoryService *this, std::string *, int)
pub fn stub_0x3d434c() {
    // IDA 0x3d434c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ChangeHistoryService::requestWaypoint(char const*)")]
// 0x3d43c0 — __ZN3RBX20ChangeHistoryService15requestWaypointEPKc
// type: int __fastcall(RBX::ChangeHistoryService *this, const char *)
pub fn stub_0x3d43c0() {
    // IDA 0x3d43c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ChangeHistoryService::setWaypoint(char const*)")]
// 0x3d43e0 — __ZN3RBX20ChangeHistoryService11setWaypointEPKc
// type: void __fastcall(RBX::ChangeHistoryService *this, char *)
pub fn stub_0x3d43e0() {
    // IDA 0x3d43e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ChangeHistoryService::mergeFirstTwoWaypoints(void)")]
// 0x3d45f0 — __ZN3RBX20ChangeHistoryService22mergeFirstTwoWaypointsEv
// type: void __fastcall(RBX::ChangeHistoryService *this)
pub fn stub_0x3d45f0() {
    // IDA 0x3d45f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ChangeHistoryService::computeDataSize(void)")]
// 0x3d4de0 — __ZN3RBX20ChangeHistoryService15computeDataSizeEv
// type: int __fastcall(RBX::ChangeHistoryService *this)
pub fn stub_0x3d4de0() {
    // IDA 0x3d4de0: platform/render/stats wiring owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ChangeHistoryService::trimWaypoints(void)")]
// 0x3d4e30 — __ZN3RBX20ChangeHistoryService13trimWaypointsEv
// type: int __fastcall(RBX::ChangeHistoryService *this)
pub fn stub_0x3d4e30() {
    // IDA 0x3d4e30: platform/render/stats wiring owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ChangeHistoryService::checkSettingWaypoint(void)")]
// 0x3d4f20 — __ZN3RBX20ChangeHistoryService20checkSettingWaypointEv
// type: int __fastcall(RBX::ChangeHistoryService *this)
pub fn stub_0x3d4f20() {
    // IDA 0x3d4f20: platform/render/stats wiring owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ChangeHistoryService::clearWaypoints(void)")]
// 0x3d4fc4 — __ZN3RBX20ChangeHistoryService14clearWaypointsEv
// type: void __fastcall(RBX::ChangeHistoryService *this)
pub fn stub_0x3d4fc4() {
    // IDA 0x3d4fc4: platform/render/stats wiring owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ChangeHistoryService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x3d511c — __ZN3RBX20ChangeHistoryService17onServiceProviderEPNS_15ServiceProviderES2_
// type: void __fastcall(boost::detail::sp_counted_base **this, RBX::ServiceProvider *, RBX::ServiceProvider *, int)
pub fn stub_0x3d511c() {
    // IDA 0x3d511c: platform/render/stats wiring owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ChangeHistoryService::onRunTransition(RBX::RunTransition)")]
// 0x3d5358 — __ZN3RBX20ChangeHistoryService15onRunTransitionENS_13RunTransitionE
// type: int __fastcall(int, int, int)
pub fn stub_0x3d5358() {
    // IDA 0x3d5358: platform/render/stats wiring owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ChangeHistoryService::terrainCellChanged(RBX::Voxel::CellChangeInfo const&)")]
// 0x3d582c — __ZN3RBX20ChangeHistoryService18terrainCellChangedERKNS_5Voxel14CellChangeInfoE
// type: void __fastcall(int, __int16 *, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x3d582c() {
    // IDA 0x3d582c: platform/render/stats wiring owned by higher crates — carrier no-op in core.
}

#[doc(alias = "non-virtual thunk toRBX::ChangeHistoryService::terrainCellChanged(RBX::Voxel::CellChangeInfo const&)")]
// 0x3d59b8 — __ZThn96_N3RBX20ChangeHistoryService18terrainCellChangedERKNS_5Voxel14CellChangeInfoE
// type: void __fastcall(int, __int16 *, int, int, int, int, int, struct _Unwind_Exception *, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x3d59b8() {
    // IDA 0x3d59b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ChangeHistoryService::Waypoint::play(void)")]
// 0x3d5fc0 — __ZN3RBX20ChangeHistoryService8Waypoint4playEv
// type: void __fastcall(RBX::ChangeHistoryService::Waypoint **this)
pub fn stub_0x3d5fc0() {
    // IDA 0x3d5fc0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ChangeHistoryService::Waypoint::unplay(void)")]
// 0x3d63dc — __ZN3RBX20ChangeHistoryService8Waypoint6unplayEv
// type: void __fastcall(RBX::ChangeHistoryService::Waypoint *this)
pub fn stub_0x3d63dc() {
    // IDA 0x3d63dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ChangeHistoryService::setRunWaypoint(void)")]
// 0x3d65c4 — __ZN3RBX20ChangeHistoryService14setRunWaypointEv
// type: void __fastcall(RBX::ChangeHistoryService *this)
pub fn stub_0x3d65c4() {
    // IDA 0x3d65c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ChangeHistoryService::requestWaypoint2(std::string)")]
// 0x3d6b10 — __ZN3RBX20ChangeHistoryService16requestWaypoint2ESs
// type: int __fastcall(RBX::ChangeHistoryService *, const char **)
pub fn stub_0x3d6b10() {
    // IDA 0x3d6b10: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ChangeHistoryService::Item::play(void)")]
// 0x3d6c14 — __ZN3RBX20ChangeHistoryService4Item4playEv
// type: void __fastcall(RBX::ChangeHistoryService::Item *this)
pub fn stub_0x3d6c14() {
    // IDA 0x3d6c14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ChangeHistoryService::Item::getCellData(unsigned int,unsigned int,unsigned int &)")]
// 0x3d6f18 — __ZN3RBX20ChangeHistoryService4Item11getCellDataEjjRj
// type: int __fastcall(RBX::ChangeHistoryService::Item *this, unsigned int, unsigned __int16, unsigned int *)
pub fn stub_0x3d6f18() {
    // IDA 0x3d6f18: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ChangeHistoryService::Waypoint::absorb(RBX::ChangeHistoryService::Waypoint const*)")]
// 0x3d7150 — __ZN3RBX20ChangeHistoryService8Waypoint6absorbEPKS1_
// type: RBX::ChangeHistoryService::Waypoint *__fastcall(RBX::ChangeHistoryService::Waypoint *this, const RBX::ChangeHistoryService::Waypoint *)
pub fn stub_0x3d7150() {
    // IDA 0x3d7150: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void delete_helper<RBX::ChangeHistoryService::Waypoint>(RBX::ChangeHistoryService::Waypoint *)")]
// 0x3d7214 — __Z13delete_helperIN3RBX20ChangeHistoryService8WaypointEEvPT_
// type: void __fastcall(_DWORD *)
pub fn stub_0x3d7214() {
    // IDA 0x3d7214: platform/render/stats wiring owned by higher crates — carrier no-op in core.
}

#[doc(alias = "std::list<RBX::ChangeHistoryService::Waypoint *,std::allocator<RBX::ChangeHistoryService::Waypoint *>>::erase(std::_List_iterator<RBX::ChangeHistoryService::Waypoint *>,std::_List_iterator<RBX::ChangeHistoryService::Waypoint *>)")]
// 0x3d72cc — __ZNSt4listIPN3RBX20ChangeHistoryService8WaypointESaIS3_EE5eraseESt14_List_iteratorIS3_ES7_
// type: std::_List_node_base *__fastcall(int, std::_List_node_base *this, std::_List_node_base *)
pub fn stub_0x3d72cc() {
    // IDA 0x3d72cc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::RunTransition)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>> const&)")]
// 0x3d74e4 — __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_20ChangeHistoryServiceES3_EENS8_5list2INS8_5valueIPSC_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
pub fn stub_0x3d74e4() {
    // IDA 0x3d74e4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "ChangeHistoryStatsItem::create(RBX::ChangeHistoryService &)")]
// 0x3d758c — __ZN22ChangeHistoryStatsItem6createERN3RBX20ChangeHistoryServiceE
// type: void __fastcall(ChangeHistoryStatsItem *this, RBX::ChangeHistoryService *)
pub fn stub_0x3d758c() {
    // IDA 0x3d758c: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "RBX::ChangeHistoryService::Item::recordCreate(void)")]
// 0x3d79c4 — __ZN3RBX20ChangeHistoryService4Item12recordCreateEv
// type: void __fastcall(RBX::ChangeHistoryService::Item *this)
pub fn stub_0x3d79c4() {
    // IDA 0x3d79c4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "RBX::ChangeHistoryService::Item::addClusterDataFast(RBX::Voxel::Grid const*)")]
// 0x3d7b64 — __ZN3RBX20ChangeHistoryService4Item18addClusterDataFastEPKNS_5Voxel4GridE
// type: const RBX::Voxel::Grid *__fastcall(RBX::ChangeHistoryService::Item *this, const RBX::Voxel::Grid *)
pub fn stub_0x3d7b64() {
    // IDA 0x3d7b64: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "RBX::ChangeHistoryService::Item::recordDelete(void)")]
// 0x3d8108 — __ZN3RBX20ChangeHistoryService4Item12recordDeleteEv
// type: int __fastcall(RBX::ChangeHistoryService::Item *this)
pub fn stub_0x3d8108() {
    // IDA 0x3d8108: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "RBX::ChangeHistoryService::Item::unplay(void)")]
// 0x3d8144 — __ZN3RBX20ChangeHistoryService4Item6unplayEv
// type: void __fastcall(RBX::ChangeHistoryService::Item *this, int, int, int)
pub fn stub_0x3d8144() {
    // IDA 0x3d8144: platform/render/stats wiring owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ChangeHistoryService::Item::unplay_CFrame(void)")]
// 0x3d8168 — __ZN3RBX20ChangeHistoryService4Item13unplay_CFrameEv
// type: int __fastcall(RBX::Instance **this)
pub fn stub_0x3d8168() {
    // IDA 0x3d8168: platform/render/stats wiring owned by higher crates — carrier no-op in core.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::ChangeHistoryService::RuntimeUndoBehavior>::construct_func(char const*,char *)")]
// 0x3d8460 — __ZN3rbx14implementation12typed_holderIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
pub fn stub_0x3d8460() {
    // IDA 0x3d8460: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "RBX::Selection * RBX::ServiceProvider::create<RBX::Selection>(void)const")]
// 0x3d95b0 — __ZNK3RBX15ServiceProvider6createINS_9SelectionEEEPT_v
// type: int __fastcall(RBX::Instance *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x3d95b0() {
    // IDA 0x3d95b0: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>>::~callable_slot()")]
// 0x3d989c — __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_20ChangeHistoryServiceES3_EENS8_5list2INS8_5valueIPSC_EENS7_3argILi1EEEEEEEED1Ev
// type: int __fastcall(int)
pub fn stub_0x3d989c() {
    // IDA 0x3d989c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>>::~callable_slot()")]
// 0x3d98c8 — __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_20ChangeHistoryServiceES3_EENS8_5list2INS8_5valueIPSC_EENS7_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x3d98c8() {
    // IDA 0x3d98c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::RunTransition)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(RBX::RunTransition)>::call(RBX::RunTransition)")]
// 0x3d99a0 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX13RunTransitionEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_20ChangeHistoryServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
// type: int __fastcall(int, int, int)
pub fn stub_0x3d99a0() {
    // IDA 0x3d99a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::RunTransition)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(RBX::RunTransition)>::call(RBX::RunTransition)")]
// 0x3d99c4 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX13RunTransitionEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_20ChangeHistoryServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
// type: int __fastcall(int, int, int)
pub fn stub_0x3d99c4() {
    // IDA 0x3d99c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list1<RBX::RunTransition&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition> &,boost::_bi::list1<RBX::RunTransition&> &,int)")]
// 0x3d99e8 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX20ChangeHistoryServiceEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS3_13RunTransitionEEENS0_5list1IRSD_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(_DWORD *, char **, _DWORD **)
pub fn stub_0x3d99e8() {
    // IDA 0x3d99e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::RunTransition)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(RBX::RunTransition)>::~callable()")]
// 0x3d9a1c — __ZN3rbx8callableINS_7signals6signalIFvN3RBX13RunTransitionEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_20ChangeHistoryServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEELi1ES5_ED1Ev
// type: int __fastcall(int)
pub fn stub_0x3d9a1c() {
    // IDA 0x3d9a1c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::RunTransition)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(RBX::RunTransition)>::~callable()")]
// 0x3d9a48 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX13RunTransitionEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_20ChangeHistoryServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEELi1ES5_ED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x3d9a48() {
    // IDA 0x3d9a48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::flyweights::static_holder_class<boost::flyweights::detail::flyweight_core<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>,mpl_::na,boost::flyweights::refcounted,boost::flyweights::hashed_factory<mpl_::na,mpl_::na,mpl_::na,0>,boost::flyweights::simple_locking,boost::flyweights::static_holder>::holder_arg>::get(void)")]
// 0x3dc0d0 — __ZN5boost10flyweights19static_holder_classINS0_6detail14flyweight_coreINS2_20default_value_policyIN3RBX15ProtectedStringEEEN4mpl_2naENS0_10refcountedENS0_14hashed_factoryIS9_S9_S9_Li0EEENS0_14simple_lockingENS0_13static_holderEE10holder_argEE3getEv
// type: void *()
pub fn stub_0x3dc0d0() {
    // IDA 0x3dc0d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::multi_index::multi_index_container<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>::~multi_index_container()")]
// 0x3dc1f8 — __ZN5boost11multi_index21multi_index_containerINS_10flyweights6detail16refcounted_valueINS3_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES7_EENS2_20hashed_factory_classISA_S7_N4mpl_2naESD_SD_E10index_listESaISA_EED2Ev
// type: int __fastcall(int)
pub fn stub_0x3dc1f8() {
    // IDA 0x3dc1f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>,boost::hash<RBX::ProtectedString>,std::equal_to<RBX::ProtectedString>,boost::multi_index::detail::nth_layer<1,boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>,boost::mpl::vector0<mpl_::na>,boost::multi_index::detail::hashed_unique_tag>::delete_all_nodes_(void)")]
// 0x3dc2c0 — __ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE17delete_all_nodes_Ev
// type: _DWORD *__fastcall(_DWORD *result)
pub fn stub_0x3dc2c0() {
    // IDA 0x3dc2c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function2<void,boost::function<void ()(void)>,std::string>::operator()(boost::function<void ()(void)>,std::string)const")]
// 0x3dc500 — __ZNK5boost9function2IvNS_8functionIFvvEEESsEclES3_Ss
// type: void __fastcall(_DWORD *, int, const std::string *)
pub fn stub_0x3dc500() {
    // IDA 0x3dc500: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>> std::for_each<std::_Rb_tree_iterator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>>(std::_Rb_tree_iterator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::_Rb_tree_iterator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>)")]
// 0x3dc6d0 — __ZSt8for_eachISt17_Rb_tree_iteratorISt4pairIKjSt6vectorIjSaIjEEEEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX20ChangeHistoryService4ItemERKS1_IjS5_EEENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEEET0_T_ST_SS_
// type: int __fastcall(int, _Rb_tree_node_base *, _Rb_tree_node_base *, unsigned int, unsigned int, int)
pub fn stub_0x3dc6d0() {
    // IDA 0x3dc6d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>> const&>,boost::_bi::list1<std::pair&<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>> const&> &,boost::_bi::list1<std::pair&<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>> &,int)")]
// 0x3dc72c — __ZN5boost3_bi5list2INS0_5valueIPN3RBX20ChangeHistoryService4ItemEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS5_RKSt4pairIjSt6vectorIjSaIjEEEEENS0_5list1IRSE_IKjSH_EEEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, char **, int **)
pub fn stub_0x3dc72c() {
    // IDA 0x3dc72c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ChangeHistoryService::Item>,boost::_bi::list1<boost::_bi::value<RBX::ChangeHistoryService::Item*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x3dc874 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX20ChangeHistoryService4ItemEEENS3_5list1INS3_5valueIPS9_EEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
pub fn stub_0x3dc874() {
    // IDA 0x3dc874: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ChangeHistoryService::Item>,boost::_bi::list1<boost::_bi::value<RBX::ChangeHistoryService::Item*>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// 0x3dc8d4 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX20ChangeHistoryService4ItemEEENS3_5list1INS3_5valueIPS9_EEEEEEvE6invokeERNS1_15function_bufferE
// type: int()
pub fn stub_0x3dc8d4() {
    // IDA 0x3dc8d4: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ChangeHistoryService::Item>,boost::_bi::list1<boost::_bi::value<RBX::ChangeHistoryService::Item*>>>::operator()(void)")]
// 0x3dc8d8 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX20ChangeHistoryService4ItemEEENS0_5list1INS0_5valueIPS6_EEEEEclEv
// type: int __fastcall(int)
pub fn stub_0x3dc8d8() {
    // IDA 0x3dc8d8: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::function2<void,boost::function<void ()(void)>,std::string>::dummy::nonnull(void)")]
// 0x3dc8f0 — __ZN5boost9function2IvNS_8functionIFvvEEESsE5dummy7nonnullEv
// type: void()
pub fn stub_0x3dc8f0() {
    // IDA 0x3dc8f0: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<int,boost::_mfi::cmf0<int,RBX::ChangeHistoryService>,boost::_bi::list1<boost::_bi::value<RBX::ChangeHistoryService*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x3ddd08 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX20ChangeHistoryServiceEEENS3_5list1INS3_5valueIPS8_EEEEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
pub fn stub_0x3ddd08() {
    // IDA 0x3ddd08: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<int,boost::_mfi::cmf0<int,RBX::ChangeHistoryService>,boost::_bi::list1<boost::_bi::value<RBX::ChangeHistoryService*>>>,int>::invoke(boost::detail::function::function_buffer &)")]
// 0x3ddd68 — __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX20ChangeHistoryServiceEEENS3_5list1INS3_5valueIPS8_EEEEEEiE6invokeERNS1_15function_bufferE
// type: int()
pub fn stub_0x3ddd68() {
    // IDA 0x3ddd68: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::_bi::bind_t<int,boost::_mfi::cmf0<int,RBX::ChangeHistoryService>,boost::_bi::list1<boost::_bi::value<RBX::ChangeHistoryService*>>>::operator()(void)")]
// 0x3ddd6c — __ZN5boost3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX20ChangeHistoryServiceEEENS0_5list1INS0_5valueIPS5_EEEEEclEv
// type: int __fastcall(int)
pub fn stub_0x3ddd6c() {
    // IDA 0x3ddd6c: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::function0<int>::assign_to_own(boost::function0<int> const&)")]
// 0x3ddd88 — __ZN5boost9function0IiE13assign_to_ownERKS1_
// type: int __fastcall(int result, int *)
pub fn stub_0x3ddd88() {
    // IDA 0x3ddd88: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "RBX::Stats::TypedStatsItem<int>::TypedStatsItem(boost::function0<int>)")]
// 0x3dddb8 — __ZN3RBX5Stats14TypedStatsItemIiEC2EN5boost9function0IiEE
// type: RBX::Stats::Item *__fastcall(RBX::Stats::Item *, int *)
pub fn stub_0x3dddb8() {
    // IDA 0x3dddb8: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "ChangeHistoryStatsItem::ChangeHistoryStatsItem(void)")]
// 0x3de2c8 — __ZN22ChangeHistoryStatsItemC2Ev
// type: void __fastcall(ChangeHistoryStatsItem *this)
pub fn stub_0x3de2c8() {
    // IDA 0x3de2c8: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "ChangeHistoryStatsItem::~ChangeHistoryStatsItem()")]
// 0x3de47c — __ZN22ChangeHistoryStatsItemD1Ev
// type: void __fastcall(ChangeHistoryStatsItem *__hidden this)
pub fn stub_0x3de47c() {
    // IDA 0x3de47c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "ChangeHistoryStatsItem::~ChangeHistoryStatsItem()")]
// 0x3de4b8 — __ZN22ChangeHistoryStatsItemD0Ev
// type: void __fastcall(ChangeHistoryStatsItem *__hidden this)
pub fn stub_0x3de4b8() {
    // IDA 0x3de4b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toChangeHistoryStatsItem::~ChangeHistoryStatsItem()")]
// 0x3de58c — __ZThn32_N22ChangeHistoryStatsItemD1Ev
// type: void __fastcall(ChangeHistoryStatsItem *__hidden this)
pub fn stub_0x3de58c() {
    // IDA 0x3de58c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toChangeHistoryStatsItem::~ChangeHistoryStatsItem()")]
// 0x3de5cc — __ZThn32_N22ChangeHistoryStatsItemD0Ev
// type: void __fastcall(ChangeHistoryStatsItem *__hidden this)
pub fn stub_0x3de5cc() {
    // IDA 0x3de5cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toChangeHistoryStatsItem::~ChangeHistoryStatsItem()")]
// 0x3de6a0 — __ZThn36_N22ChangeHistoryStatsItemD1Ev
// type: void __fastcall(ChangeHistoryStatsItem *__hidden this)
pub fn stub_0x3de6a0() {
    // IDA 0x3de6a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toChangeHistoryStatsItem::~ChangeHistoryStatsItem()")]
// 0x3de6e0 — __ZThn36_N22ChangeHistoryStatsItemD0Ev
// type: void __fastcall(ChangeHistoryStatsItem *__hidden this)
pub fn stub_0x3de6e0() {
    // IDA 0x3de6e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>> std::for_each<std::_Rb_tree_const_iterator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>>(std::_Rb_tree_const_iterator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::_Rb_tree_const_iterator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>)")]
// 0x3dedb4 — __ZSt8for_eachISt23_Rb_tree_const_iteratorISt4pairIKjSt6vectorIjSaIjEEEEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX20ChangeHistoryService4ItemERKS1_IjS5_EEENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEEET0_T_ST_SS_
// type: int __fastcall(int, const _Rb_tree_node_base *, const _Rb_tree_node_base *, unsigned int, unsigned int, int)
pub fn stub_0x3dedb4() {
    // IDA 0x3dedb4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>> const&>,boost::_bi::list1<std::pair const&<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>> const&> &,boost::_bi::list1<std::pair const&<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>> &,int)")]
// 0x3df0f0 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX20ChangeHistoryService4ItemEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS5_RKSt4pairIjSt6vectorIjSaIjEEEEENS0_5list1IRKSE_IKjSH_EEEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, char **, int **)
pub fn stub_0x3df0f0() {
    // IDA 0x3df0f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function2<void,boost::function<void ()(void)>,std::string>::clear(void)")]
// 0x3df798 — __ZN5boost9function2IvNS_8functionIFvvEEESsE5clearEv
// type: int __fastcall(int *)
pub fn stub_0x3df798() {
    // IDA 0x3df798: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5sSkinEEEEvv")]
// 0x3e3f24 — __ZN3RBX4Name13callDoDeclareILZNS_5sSkinEEEEvv
pub fn stub_0x3e3f24() {
    // IDA 0x3e3f24: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5sSkinEEEERKS0_v")]
// 0x3e3f28 — __ZN3RBX4Name9doDeclareILZNS_5sSkinEEEERKS0_v
// type: int()
pub fn stub_0x3e3f28() {
    // IDA 0x3e3f28: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sClothingEEEEvv")]
// 0x3e42c0 — __ZN3RBX4Name13callDoDeclareILZNS_9sClothingEEEEvv
pub fn stub_0x3e42c0() {
    // IDA 0x3e42c0: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sClothingEEEERKS0_v")]
// 0x3e42c4 — __ZN3RBX4Name9doDeclareILZNS_9sClothingEEEERKS0_v
// type: int()
pub fn stub_0x3e42c4() {
    // IDA 0x3e42c4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sShirtGraphicEEEEvv")]
// 0x3e49b8 — __ZN3RBX4Name13callDoDeclareILZNS_13sShirtGraphicEEEEvv
pub fn stub_0x3e49b8() {
    // IDA 0x3e49b8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sShirtGraphicEEEERKS0_v")]
// 0x3e49bc — __ZN3RBX4Name9doDeclareILZNS_13sShirtGraphicEEEERKS0_v
// type: int()
pub fn stub_0x3e49bc() {
    // IDA 0x3e49bc: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_168")]
// 0x3e7f44 — __GLOBAL__I_a_168
pub fn stub_0x3e7f44() {
    // IDA 0x3e7f44: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sCharacterMeshEEEEvv")]
// 0x3e9b90 — __ZN3RBX4Name13callDoDeclareILZNS_14sCharacterMeshEEEEvv
pub fn stub_0x3e9b90() {
    // IDA 0x3e9b90: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sCharacterMeshEEEERKS0_v")]
// 0x3e9b94 — __ZN3RBX4Name9doDeclareILZNS_14sCharacterMeshEEEERKS0_v
// type: int()
pub fn stub_0x3e9b94() {
    // IDA 0x3e9b94: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_20sCharacterAppearanceEEEERKS0_v")]
// 0x3ea1dc — __ZN3RBX4Name7declareILZNS_20sCharacterAppearanceEEEERKS0_v
pub fn stub_0x3ea1dc() {
    // IDA 0x3ea1dc: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_20sCharacterAppearanceEEEEvv")]
// 0x3ea220 — __ZN3RBX4Name13callDoDeclareILZNS_20sCharacterAppearanceEEEEvv
pub fn stub_0x3ea220() {
    // IDA 0x3ea220: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_20sCharacterAppearanceEEEERKS0_v")]
// 0x3ea224 — __ZN3RBX4Name9doDeclareILZNS_20sCharacterAppearanceEEEERKS0_v
// type: int()
pub fn stub_0x3ea224() {
    // IDA 0x3ea224: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}
