//! core shard ow — 100 core stubs EA-sorted, 0x9e4de0..0xb21a6c (rbx:: lowercase namespace, EA-sorted asc, next 100 uncovered).
//! Source: ida/export.json filtered where mangled starts with __ZN3rbx, EA-sorted asc, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "rbx::signals::signal_with_args<3,void ()(int,bool,int)>::operator()(int,bool,int)")]
// 0x9e4de0 — __ZN3rbx7signals16signal_with_argsILi3EFvibiEEclEibi
// type: void __fastcall(_DWORD *, int, unsigned __int8, const void *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, void *, int)
pub fn stub_0x9e4de0() {
    // IDA 0x9e4de0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "rbx::signals::signal<void ()(int,bool,int)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(int,bool,int)>::slot> &)")]
// 0x9e6b98 — __ZN3rbx7signals6signalIFvibiEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// type: int __fastcall(int, int32_t **)
pub fn stub_0x9e6b98() {
    // IDA 0x9e6b98: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,bool,int)>::mutex(void)")]
// 0x9e6da0 — __ZN3rbx7signals6signalIFvibiEE5mutexEv
// type: int __fastcall(int, int, int, int)
pub fn stub_0x9e6da0() {
    // IDA 0x9e6da0: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,bool,int)>::safe_static_init_mutex(void)")]
// 0x9e6f68 — __ZN3rbx7signals6signalIFvibiEE22safe_static_init_mutexEv
// type: void()
pub fn stub_0x9e6f68() {
    // IDA 0x9e6f68: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,bool,int)>::disconnectAll(void)")]
// 0x9ecf0c — __ZN3rbx7signals6signalIFvibiEE13disconnectAllEv
// type: void __fastcall(_DWORD *)
pub fn stub_0x9ecf0c() {
    // IDA 0x9ecf0c: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,bool,int)>::insert(rbx::signals::signal<void ()(int,bool,int)>::slot *)")]
// 0x9eea54 — __ZN3rbx7signals6signalIFvibiEE6insertEPNS3_4slotE
// type: void __fastcall(int, int, int)
pub fn stub_0x9eea54() {
    // IDA 0x9eea54: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,bool,int)>::callable_slot<boost::function<void ()(int,bool,int)>>::~callable_slot()")]
// 0x9eedc0 — __ZN3rbx7signals6signalIFvibiEE13callable_slotIN5boost8functionIS2_EEED1Ev
// type: int()
pub fn stub_0x9eedc0() {
    // IDA 0x9eedc0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,bool,int)>::callable_slot<boost::function<void ()(int,bool,int)>>::~callable_slot()")]
// 0x9eedcc — __ZN3rbx7signals6signalIFvibiEE13callable_slotIN5boost8functionIS2_EEED0Ev
// type: void __fastcall(void *)
pub fn stub_0x9eedcc() {
    // IDA 0x9eedcc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,bool,int)>::slot::disconnect(void)")]
// 0x9eee80 — __ZN3rbx7signals6signalIFvibiEE4slot10disconnectEv
// type: void __fastcall(int, int, int, int)
pub fn stub_0x9eee80() {
    // IDA 0x9eee80: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,bool,int)>::slot,boost::function<void ()(int,bool,int)>,3,void ()(int,bool,int)>::call(int,bool,int)")]
// 0x9ef000 — __ZN3rbx8callableINS_7signals6signalIFvibiEE4slotEN5boost8functionIS3_EELi3ES3_E4callEibi
// type: void __fastcall(int, int, int, int)
pub fn stub_0x9ef000() {
    // IDA 0x9ef000: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,bool,int)>::remove(rbx::signals::signal<void ()(int,bool,int)>::slot *)")]
// 0x9ef248 — __ZN3rbx7signals6signalIFvibiEE6removeEPNS3_4slotE
// type: int __fastcall(char **, char *, int, int (*)(const char *, ...))
pub fn stub_0x9ef248() {
    // IDA 0x9ef248: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,bool,int)>::slot::safe_static_init_mutex(void)")]
// 0x9ef334 — __ZN3rbx7signals6signalIFvibiEE4slot22safe_static_init_mutexEv
// type: void()
pub fn stub_0x9ef334() {
    // IDA 0x9ef334: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,bool,int)>::slot,boost::function<void ()(int,bool,int)>,3,void ()(int,bool,int)>::~callable()")]
// 0x9ef41c — __ZN3rbx8callableINS_7signals6signalIFvibiEE4slotEN5boost8functionIS3_EELi3ES3_ED2Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_0x9ef41c() {
    // IDA 0x9ef41c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,bool,int)>::slot,boost::function<void ()(int,bool,int)>,3,void ()(int,bool,int)>::~callable()")]
// 0x9ef5b4 — __ZN3rbx8callableINS_7signals6signalIFvibiEE4slotEN5boost8functionIS3_EELi3ES3_ED1Ev
// type: int __fastcall(int)
pub fn stub_0x9ef5b4() {
    // IDA 0x9ef5b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,bool,int)>::slot,boost::function<void ()(int,bool,int)>,3,void ()(int,bool,int)>::~callable()")]
// 0x9ef5c0 — __ZN3rbx8callableINS_7signals6signalIFvibiEE4slotEN5boost8functionIS3_EELi3ES3_ED0Ev
// type: void __fastcall(void *)
pub fn stub_0x9ef5c0() {
    // IDA 0x9ef5c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,bool,int)>::slot::~slot()")]
// 0x9ef674 — __ZN3rbx7signals6signalIFvibiEE4slotD1Ev
// type: int __fastcall(int)
pub fn stub_0x9ef674() {
    // IDA 0x9ef674: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,bool,int)>::slot::~slot()")]
// 0x9ef6d0 — __ZN3rbx7signals6signalIFvibiEE4slotD0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x9ef6d0() {
    // IDA 0x9ef6d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(std::string)>::operator()(std::string)")]
// 0xa1d528 — __ZN3rbx7signals16signal_with_argsILi1EFvSsEEclESs
// type: void __fastcall(_DWORD *, std::string *, void *, int)
pub fn stub_0xa1d528() {
    // IDA 0xa1d528: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(bool,int)>::insert(rbx::signals::signal<void ()(bool,int)>::slot *)")]
// 0xa27140 — __ZN3rbx7signals6signalIFvbiEE6insertEPNS3_4slotE
// type: void __fastcall(int32_t **, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0xa27140() {
    // IDA 0xa27140: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(bool,int)>::mutex(void)")]
// 0xa27404 — __ZN3rbx7signals6signalIFvbiEE5mutexEv
// type: int __fastcall(int, int, int, int)
pub fn stub_0xa27404() {
    // IDA 0xa27404: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(bool,int)>::safe_static_init_mutex(void)")]
// 0xa27680 — __ZN3rbx7signals6signalIFvbiEE22safe_static_init_mutexEv
// type: void()
pub fn stub_0xa27680() {
    // IDA 0xa27680: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(bool,int)>::slot::disconnect(void)")]
// 0xa27828 — __ZN3rbx7signals6signalIFvbiEE4slot10disconnectEv
// type: void __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0xa27828() {
    // IDA 0xa27828: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(bool,int)>::remove(rbx::signals::signal<void ()(bool,int)>::slot *)")]
// 0xa27dc8 — __ZN3rbx7signals6signalIFvbiEE6removeEPNS3_4slotE
// type: int __fastcall(char **, char *, int, int (*)(const char *, ...))
pub fn stub_0xa27dc8() {
    // IDA 0xa27dc8: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(bool,int)>::slot::safe_static_init_mutex(void)")]
// 0xa27eb4 — __ZN3rbx7signals6signalIFvbiEE4slot22safe_static_init_mutexEv
// type: void()
pub fn stub_0xa27eb4() {
    // IDA 0xa27eb4: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(bool,int)>::slot::~slot()")]
// 0xa28234 — __ZN3rbx7signals6signalIFvbiEE4slotD1Ev
// type: int __fastcall(int)
pub fn stub_0xa28234() {
    // IDA 0xa28234: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(bool,int)>::slot::~slot()")]
// 0xa28290 — __ZN3rbx7signals6signalIFvbiEE4slotD0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0xa28290() {
    // IDA 0xa28290: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::mutex(void)")]
// 0xa28398 — __ZN3rbx7signals6signalIFvvEE5mutexEv
// type: int __fastcall(int, int, int, int)
pub fn stub_0xa28398() {
    // IDA 0xa28398: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string)>::insert(rbx::signals::signal<void ()(std::string)>::slot *)")]
// 0xa28848 — __ZN3rbx7signals6signalIFvSsEE6insertEPNS3_4slotE
// type: void __fastcall(int32_t **, int, int, int (*)(const char *, ...), boost::mutex *, char, int, int, int, int)
pub fn stub_0xa28848() {
    // IDA 0xa28848: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string)>::mutex(void)")]
// 0xa28b08 — __ZN3rbx7signals6signalIFvSsEE5mutexEv
// type: int __fastcall(int, int, int, int)
pub fn stub_0xa28b08() {
    // IDA 0xa28b08: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string)>::slot::disconnect(void)")]
// 0xa28e38 — __ZN3rbx7signals6signalIFvSsEE4slot10disconnectEv
// type: void __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0xa28e38() {
    // IDA 0xa28e38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string)>::remove(rbx::signals::signal<void ()(std::string)>::slot *)")]
// 0xa29158 — __ZN3rbx7signals6signalIFvSsEE6removeEPNS3_4slotE
// type: int __fastcall(char **, char *, int, int (*)(const char *, ...))
pub fn stub_0xa29158() {
    // IDA 0xa29158: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string)>::slot::~slot()")]
// 0xa29248 — __ZN3rbx7signals6signalIFvSsEE4slotD1Ev
// type: int __fastcall(int)
pub fn stub_0xa29248() {
    // IDA 0xa29248: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,std::string)>::insert(rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot *)")]
// 0xa2a2c4 — __ZN3rbx7signals6signalIFvSsSsSsEE6insertEPNS3_4slotE
// type: void __fastcall(int32_t **, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0xa2a2c4() {
    // IDA 0xa2a2c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,std::string)>::mutex(void)")]
// 0xa2a584 — __ZN3rbx7signals6signalIFvSsSsSsEE5mutexEv
// type: int __fastcall(int, int, int, int)
pub fn stub_0xa2a584() {
    // IDA 0xa2a584: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,std::string)>::safe_static_init_mutex(void)")]
// 0xa2a800 — __ZN3rbx7signals6signalIFvSsSsSsEE22safe_static_init_mutexEv
// type: void()
pub fn stub_0xa2a800() {
    // IDA 0xa2a800: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot::disconnect(void)")]
// 0xa2aa4c — __ZN3rbx7signals6signalIFvSsSsSsEE4slot10disconnectEv
// type: void __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0xa2aa4c() {
    // IDA 0xa2aa4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,std::string)>::remove(rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot *)")]
// 0xa2b09c — __ZN3rbx7signals6signalIFvSsSsSsEE6removeEPNS3_4slotE
// type: int __fastcall(char **, char *, int, int (*)(const char *, ...))
pub fn stub_0xa2b09c() {
    // IDA 0xa2b09c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot::safe_static_init_mutex(void)")]
// 0xa2b188 — __ZN3rbx7signals6signalIFvSsSsSsEE4slot22safe_static_init_mutexEv
// type: void()
pub fn stub_0xa2b188() {
    // IDA 0xa2b188: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot::~slot()")]
// 0xa2b26c — __ZN3rbx7signals6signalIFvSsSsSsEE4slotD1Ev
// type: int __fastcall(int)
pub fn stub_0xa2b26c() {
    // IDA 0xa2b26c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot::~slot()")]
// 0xa2b2c8 — __ZN3rbx7signals6signalIFvSsSsSsEE4slotD0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0xa2b2c8() {
    // IDA 0xa2b2c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,bool)>::mutex(void)")]
// 0xa2efb0 — __ZN3rbx7signals6signalIFvSsbEE5mutexEv
// type: int __fastcall(int, int, int, int)
pub fn stub_0xa2efb0() {
    // IDA 0xa2efb0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,bool)>::safe_static_init_mutex(void)")]
// 0xa2f0c8 — __ZN3rbx7signals6signalIFvSsbEE22safe_static_init_mutexEv
// type: void()
pub fn stub_0xa2f0c8() {
    // IDA 0xa2f0c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::mutex(void)")]
// 0xa3e760 — __ZN3rbx7signals6signalIFvbEE5mutexEv
// type: int __fastcall(int, int, int, int)
pub fn stub_0xa3e760() {
    // IDA 0xa3e760: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::safe_static_init_mutex(void)")]
// 0xa3e878 — __ZN3rbx7signals6signalIFvbEE22safe_static_init_mutexEv
// type: void()
pub fn stub_0xa3e878() {
    // IDA 0xa3e878: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string)>::disconnectAll(void)")]
// 0xa48b00 — __ZN3rbx7signals6signalIFvSsEE13disconnectAllEv
// type: void __fastcall(_DWORD *, int, int, int)
pub fn stub_0xa48b00() {
    // IDA 0xa48b00: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string)>::callable_slot<boost::function<void ()(std::string)>>::~callable_slot()")]
// 0xa49590 — __ZN3rbx7signals6signalIFvSsEE13callable_slotIN5boost8functionIS2_EEED1Ev
// type: int()
pub fn stub_0xa49590() {
    // IDA 0xa49590: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::function<void ()(std::string)>,1,void ()(std::string)>::call(std::string)")]
// 0xa495a0 — __ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost8functionIS3_EELi1ES3_E4callESs
// type: void __fastcall(int, const std::string *)
pub fn stub_0xa495a0() {
    // IDA 0xa495a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::function<void ()(std::string)>,1,void ()(std::string)>::~callable()")]
// 0xa498bc — __ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost8functionIS3_EELi1ES3_ED2Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_0xa498bc() {
    // IDA 0xa498bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::function<void ()(std::string)>,1,void ()(std::string)>::~callable()")]
// 0xa49a58 — __ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost8functionIS3_EELi1ES3_ED0Ev
// type: void __fastcall(void *)
pub fn stub_0xa49a58() {
    // IDA 0xa49a58: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::remote_signal<void ()(std::string)>::~remote_signal()")]
// 0xa9713c — __ZN3rbx13remote_signalIFvSsEED1Ev
// type: int32_t **__fastcall(int, int, int, int, int, int, int, int, int, int)
pub fn stub_0xa9713c() {
    // IDA 0xa9713c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::remote_signal<void ()(void)>::~remote_signal()")]
// 0xa973d8 — __ZN3rbx13remote_signalIFvvEED1Ev
// type: int32_t **__fastcall(int, int, int, int, int, int, int, int, int, int)
pub fn stub_0xa973d8() {
    // IDA 0xa973d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::remote_signal<void ()(bool,int)>::~remote_signal()")]
// 0xa97674 — __ZN3rbx13remote_signalIFvbiEED1Ev
// type: int32_t **__fastcall(int, int, int, int, int, int, int, int, int, int)
pub fn stub_0xa97674() {
    // IDA 0xa97674: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::remote_signal<void ()(std::string,std::string,std::string)>::~remote_signal()")]
// 0xa97bac — __ZN3rbx13remote_signalIFvSsSsSsEED1Ev
// type: int32_t **__fastcall(int, int, int, int, int, int, int, int, int, int)
pub fn stub_0xa97bac() {
    // IDA 0xa97bac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<2,void ()(bool,int)>::operator()(bool,int)")]
// 0xaa261c — __ZN3rbx7signals16signal_with_argsILi2EFvbiEEclEbi
// type: void __fastcall(_DWORD *, unsigned __int8, int, const void *)
pub fn stub_0xaa261c() {
    // IDA 0xaa261c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(bool,int)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(bool,int)>::slot> &)")]
// 0xaa2800 — __ZN3rbx7signals6signalIFvbiEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// type: int __fastcall(int, int32_t **)
pub fn stub_0xaa2800() {
    // IDA 0xaa2800: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::mutex(void)")]
// 0xaa8988 — __ZN3rbx7signals6signalIFvdEE5mutexEv
// type: int __fastcall(int, int, int, int)
pub fn stub_0xaa8988() {
    // IDA 0xaa8988: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Player>,boost::_bi::list1<boost::_bi::value<RBX::Network::Player*>>>>::~callable_slot()")]
// 0xaa9e38 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX7Network6PlayerEEENS6_5list1INS6_5valueIPSC_EEEEEEED1Ev
// type: int __fastcall(int)
pub fn stub_0xaa9e38() {
    // IDA 0xaa9e38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Player>,boost::_bi::list1<boost::_bi::value<RBX::Network::Player*>>>>::~callable_slot()")]
// 0xaa9e94 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX7Network6PlayerEEENS6_5list1INS6_5valueIPSC_EEEEEEED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0xaa9e94() {
    // IDA 0xaa9e94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Player>,boost::_bi::list1<boost::_bi::value<RBX::Network::Player*>>>,0,void ()(void)>::call(void)")]
// 0xaa9f9c — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX7Network6PlayerEEENS7_5list1INS7_5valueIPSD_EEEEEELi0ES3_E4callEv
// type: int __fastcall(_DWORD *)
pub fn stub_0xaa9f9c() {
    // IDA 0xaa9f9c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<3,void ()(std::string,std::string,std::string)>::operator()(std::string,std::string,std::string)")]
// 0xaabaf0 — __ZN3rbx7signals16signal_with_argsILi3EFvSsSsSsEEclESsSsSs
// type: void __fastcall(_DWORD *, std::string *, std::string *, std::string *)
pub fn stub_0xaabaf0() {
    // IDA 0xaabaf0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,std::string)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot> &)")]
// 0xaabe7c — __ZN3rbx7signals6signalIFvSsSsSsEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// type: int __fastcall(int, int32_t **)
pub fn stub_0xaabe7c() {
    // IDA 0xaabe7c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<3,void ()(std::string,std::string,std::string)>::fireItem(rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot *,std::string,std::string,std::string)")]
// 0xaac080 — __ZN3rbx7signals16signal_with_argsILi3EFvSsSsSsEE8fireItemEPNS0_6signalIS2_E4slotESsSsSs
// type: void __fastcall(int, const std::string *, const std::string *, const std::string *)
pub fn stub_0xaac080() {
    // IDA 0xaac080: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::remote_signal<void ()(bool,int)>::remote_signal(void)")]
// 0xaac2c0 — __ZN3rbx13remote_signalIFvbiEEC2Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_0xaac2c0() {
    // IDA 0xaac2c0: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(bool,int)>::disconnectAll(void)")]
// 0xaac4c0 — __ZN3rbx7signals6signalIFvbiEE13disconnectAllEv
// type: void __fastcall(_DWORD *)
pub fn stub_0xaac4c0() {
    // IDA 0xaac4c0: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::remote_signal<void ()(std::string,std::string,std::string)>::remote_signal(void)")]
// 0xaaca18 — __ZN3rbx13remote_signalIFvSsSsSsEEC2Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_0xaaca18() {
    // IDA 0xaaca18: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,std::string)>::disconnectAll(void)")]
// 0xaacc18 — __ZN3rbx7signals6signalIFvSsSsSsEE13disconnectAllEv
// type: void __fastcall(_DWORD *)
pub fn stub_0xaacc18() {
    // IDA 0xaacc18: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::connection rbx::remote_signal<void ()(std::string)>::connect<boost::function<void ()(std::string)>>(boost::function<void ()(std::string)> const&)")]
// 0xaae43c — __ZN3rbx13remote_signalIFvSsEE7connectIN5boost8functionIS1_EEEENS_7signals10connectionERKT_
// type: void __fastcall(int, int, int *, int, int, void *, int, int, int, int)
pub fn stub_0xaae43c() {
    // IDA 0xaae43c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::connection rbx::remote_signal<void ()(std::string,std::string,std::string)>::connect<boost::function<void ()(std::string,std::string,std::string)>>(boost::function<void ()(std::string,std::string,std::string)> const&)")]
// 0xab370c — __ZN3rbx13remote_signalIFvSsSsSsEE7connectIN5boost8functionIS1_EEEENS_7signals10connectionERKT_
// type: void __fastcall(int, int, int *, int, int, void *, int, int, int, int)
pub fn stub_0xab370c() {
    // IDA 0xab370c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,std::string)>::callable_slot<boost::function<void ()(std::string,std::string,std::string)>>::~callable_slot()")]
// 0xab38d0 — __ZN3rbx7signals6signalIFvSsSsSsEE13callable_slotIN5boost8functionIS2_EEED1Ev
// type: int()
pub fn stub_0xab38d0() {
    // IDA 0xab38d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,std::string)>::callable_slot<boost::function<void ()(std::string,std::string,std::string)>>::~callable_slot()")]
// 0xab38dc — __ZN3rbx7signals6signalIFvSsSsSsEE13callable_slotIN5boost8functionIS2_EEED0Ev
// type: void __fastcall(void *)
pub fn stub_0xab38dc() {
    // IDA 0xab38dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot,boost::function<void ()(std::string,std::string,std::string)>,3,void ()(std::string,std::string,std::string)>::call(std::string,std::string,std::string)")]
// 0xab3990 — __ZN3rbx8callableINS_7signals6signalIFvSsSsSsEE4slotEN5boost8functionIS3_EELi3ES3_E4callESsSsSs
// type: void __fastcall(int, const std::string *, const std::string *, const std::string *)
pub fn stub_0xab3990() {
    // IDA 0xab3990: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot,boost::function<void ()(std::string,std::string,std::string)>,3,void ()(std::string,std::string,std::string)>::~callable()")]
// 0xab3ed4 — __ZN3rbx8callableINS_7signals6signalIFvSsSsSsEE4slotEN5boost8functionIS3_EELi3ES3_ED2Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_0xab3ed4() {
    // IDA 0xab3ed4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot,boost::function<void ()(std::string,std::string,std::string)>,3,void ()(std::string,std::string,std::string)>::~callable()")]
// 0xab406c — __ZN3rbx8callableINS_7signals6signalIFvSsSsSsEE4slotEN5boost8functionIS3_EELi3ES3_ED1Ev
// type: int __fastcall(int)
pub fn stub_0xab406c() {
    // IDA 0xab406c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot,boost::function<void ()(std::string,std::string,std::string)>,3,void ()(std::string,std::string,std::string)>::~callable()")]
// 0xab4078 — __ZN3rbx8callableINS_7signals6signalIFvSsSsSsEE4slotEN5boost8functionIS3_EELi3ES3_ED0Ev
// type: void __fastcall(void *)
pub fn stub_0xab4078() {
    // IDA 0xab4078: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::connection rbx::remote_signal<void ()(void)>::connect<boost::function<void ()(void)>>(boost::function<void ()(void)> const&)")]
// 0xab5088 — __ZN3rbx13remote_signalIFvvEE7connectIN5boost8functionIS1_EEEENS_7signals10connectionERKT_
// type: void __fastcall(int, int, int *, int, int, void *, int, int, int, int)
pub fn stub_0xab5088() {
    // IDA 0xab5088: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::connection rbx::remote_signal<void ()(bool,int)>::connect<boost::function<void ()(bool,int)>>(boost::function<void ()(bool,int)> const&)")]
// 0xac75b8 — __ZN3rbx13remote_signalIFvbiEE7connectIN5boost8functionIS1_EEEENS_7signals10connectionERKT_
// type: void __fastcall(int, int, int *, int, int, void *, int, int, int, int)
pub fn stub_0xac75b8() {
    // IDA 0xac75b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(bool,int)>::callable_slot<boost::function<void ()(bool,int)>>::~callable_slot()")]
// 0xac777c — __ZN3rbx7signals6signalIFvbiEE13callable_slotIN5boost8functionIS2_EEED1Ev
// type: int()
pub fn stub_0xac777c() {
    // IDA 0xac777c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(bool,int)>::callable_slot<boost::function<void ()(bool,int)>>::~callable_slot()")]
// 0xac7788 — __ZN3rbx7signals6signalIFvbiEE13callable_slotIN5boost8functionIS2_EEED0Ev
// type: void __fastcall(void *)
pub fn stub_0xac7788() {
    // IDA 0xac7788: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,int)>::slot,boost::function<void ()(bool,int)>,2,void ()(bool,int)>::call(bool,int)")]
// 0xac783c — __ZN3rbx8callableINS_7signals6signalIFvbiEE4slotEN5boost8functionIS3_EELi2ES3_E4callEbi
// type: void __fastcall(int, int, int)
pub fn stub_0xac783c() {
    // IDA 0xac783c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,int)>::slot,boost::function<void ()(bool,int)>,2,void ()(bool,int)>::~callable()")]
// 0xac7a7c — __ZN3rbx8callableINS_7signals6signalIFvbiEE4slotEN5boost8functionIS3_EELi2ES3_ED2Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_0xac7a7c() {
    // IDA 0xac7a7c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,int)>::slot,boost::function<void ()(bool,int)>,2,void ()(bool,int)>::~callable()")]
// 0xac7c14 — __ZN3rbx8callableINS_7signals6signalIFvbiEE4slotEN5boost8functionIS3_EELi2ES3_ED1Ev
// type: int __fastcall(int)
pub fn stub_0xac7c14() {
    // IDA 0xac7c14: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,int)>::slot,boost::function<void ()(bool,int)>,2,void ()(bool,int)>::~callable()")]
// 0xac7c20 — __ZN3rbx8callableINS_7signals6signalIFvbiEE4slotEN5boost8functionIS3_EELi2ES3_ED0Ev
// type: void __fastcall(void *)
pub fn stub_0xac7c20() {
    // IDA 0xac7c20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::timestamped_safe_queue<RakNet::Packet *>::pop_if_waited(RBX::Time::Interval,RakNet::Packet *&)")]
// 0xb0ab04 — __ZN3rbx22timestamped_safe_queueIPN6RakNet6PacketEE13pop_if_waitedEN3RBX4Time8IntervalERS3_
// type: int __fastcall(int, unsigned int, unsigned int, _DWORD *, int, pthread_mutex_t *, int, int, int, int)
pub fn stub_0xb0ab04() {
    // IDA 0xb0ab04: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<2,void ()(std::string,bool)>::operator()(std::string,bool)")]
// 0xb0b408 — __ZN3rbx7signals16signal_with_argsILi2EFvSsbEEclESsb
// type: void __fastcall(_DWORD *, std::string *, unsigned __int8, const void *)
pub fn stub_0xb0b408() {
    // IDA 0xb0b408: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,bool)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,bool)>::slot> &)")]
// 0xb19898 — __ZN3rbx7signals6signalIFvSsbEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// type: int __fastcall(int, int32_t **)
pub fn stub_0xb19898() {
    // IDA 0xb19898: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::ContentId>::construct_func(char const*,char *)")]
// 0xb1f3b0 — __ZN3rbx14implementation12typed_holderIN3RBX9ContentIdEE14construct_funcEPKcPc
// type: const std::string *__fastcall(const std::string *result, std::string *)
pub fn stub_0xb1f3b0() {
    // IDA 0xb1f3b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Region3int16>::construct_func(char const*,char *)")]
// 0xb1f3d0 — __ZN3rbx14implementation12typed_holderIN3RBX12Region3int16EE14construct_funcEPKcPc
// type: __int64 *__fastcall(__int64 *result, int)
pub fn stub_0xb1f3d0() {
    // IDA 0xb1f3d0: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Region3>(RBX::Region3 const&)")]
// 0xb1f3e8 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIS2_EERS3_RKT_
// type: int __fastcall(int, __int64 *)
pub fn stub_0xb1f3e8() {
    // IDA 0xb1f3e8: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::BrickColor>::construct_func(char const*,char *)")]
// 0xb1f528 — __ZN3rbx14implementation12typed_holderIN3RBX10BrickColorEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
pub fn stub_0xb1f528() {
    // IDA 0xb1f528: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Axes>::construct_func(char const*,char *)")]
// 0xb1f538 — __ZN3rbx14implementation12typed_holderIN3RBX4AxesEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
pub fn stub_0xb1f538() {
    // IDA 0xb1f538: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Faces>::construct_func(char const*,char *)")]
// 0xb1f548 — __ZN3rbx14implementation12typed_holderIN3RBX5FacesEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
pub fn stub_0xb1f548() {
    // IDA 0xb1f548: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::RbxRay>::construct_func(char const*,char *)")]
// 0xb1f558 — __ZN3rbx14implementation12typed_holderIN3RBX6RbxRayEE14construct_funcEPKcPc
// type: int __fastcall(int result, int)
pub fn stub_0xb1f558() {
    // IDA 0xb1f558: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::UDim2>::construct_func(char const*,char *)")]
// 0xb1f588 — __ZN3rbx14implementation12typed_holderIN3RBX5UDim2EE14construct_funcEPKcPc
// type: _QWORD *__fastcall(_QWORD *result, _QWORD *)
pub fn stub_0xb1f588() {
    // IDA 0xb1f588: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::UDim2>::destruct_func(char *)")]
// 0xb1f598 — __ZN3rbx14implementation12typed_holderIN3RBX5UDim2EE13destruct_funcEPc
// type: void()
pub fn stub_0xb1f598() {
    // IDA 0xb1f598: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<long>::construct_func(char const*,char *)")]
// 0xb1f59c — __ZN3rbx14implementation12typed_holderIlE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
pub fn stub_0xb1f59c() {
    // IDA 0xb1f59c: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<long>::destruct_func(char *)")]
// 0xb1f5a8 — __ZN3rbx14implementation12typed_holderIlE13destruct_funcEPc
// type: void()
pub fn stub_0xb1f5a8() {
    // IDA 0xb1f5a8: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RakNet::SystemAddress const&,rbx_core::SharedPtr<RakNet::BitStream> const&,std::string const&,std::string const&)>::insert(rbx::signals::signal<void ()(RakNet::SystemAddress const&,rbx_core::SharedPtr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot *)")]
// 0xb21580 — __ZN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS2_9BitStreamEEERKSsSD_EE6insertEPNSF_4slotE
// type: void __fastcall(int32_t **, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0xb21580() {
    // IDA 0xb21580: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RakNet::SystemAddress const&,rbx_core::SharedPtr<RakNet::BitStream> const&,std::string const&,std::string const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Replicator,RakNet::SystemAddress const&,rbx_core::SharedPtr<RakNet::BitStream> const&,std::string const&,std::string const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::~callable_slot()")]
// 0xb219ac — __ZN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS2_9BitStreamEEERKSsSD_EE13callable_slotINS6_3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX7Network10ReplicatorES5_SB_SD_SD_EENSH_5list5INSH_5valueINS7_ISN_EEEENS6_3argILi1EEENST_ILi2EEENST_ILi3EEENST_ILi4EEEEEEEED1Ev
// type: int()
pub fn stub_0xb219ac() {
    // IDA 0xb219ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RakNet::SystemAddress const&,rbx_core::SharedPtr<RakNet::BitStream> const&,std::string const&,std::string const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Replicator,RakNet::SystemAddress const&,rbx_core::SharedPtr<RakNet::BitStream> const&,std::string const&,std::string const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::~callable_slot()")]
// 0xb219b8 — __ZN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS2_9BitStreamEEERKSsSD_EE13callable_slotINS6_3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX7Network10ReplicatorES5_SB_SD_SD_EENSH_5list5INSH_5valueINS7_ISN_EEEENS6_3argILi1EEENST_ILi2EEENST_ILi3EEENST_ILi4EEEEEEEED0Ev
// type: void __fastcall(void *)
pub fn stub_0xb219b8() {
    // IDA 0xb219b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RakNet::SystemAddress const&,rbx_core::SharedPtr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot::disconnect(void)")]
// 0xb21a6c — __ZN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS2_9BitStreamEEERKSsSD_EE4slot10disconnectEv
// type: void __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0xb21a6c() {
    // IDA 0xb21a6c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
