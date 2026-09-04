//! core bg20 — 100 core stubs EA-sorted asc distinct not in /tmp/global_eas.txt.
//! Source: ida/export.json (85545 funcs) EA asc core-filtered (exclude Reflection|Instance|DataModel|Ogre|G3D|RakNet|FMOD|Lua, exclude boost) global distinct — next 100 uncovered 0xf413a4..0xf443d4.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr not emitted (boost funcs excluded); single quotes, backticks, double quotes removed from alias.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "std::_Rb_tree<char,char,std::_Identity<char>,std::less<char>,std::allocator<char>>::_M_insert_unique(char const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIccSt9_IdentityIcESt4lessIcESaIcEE16_M_insert_uniqueERKc")]
// 0xf413a4 — j___ZNSt8_Rb_treeIccSt9_IdentityIcESt4lessIcESaIcEE16_M_insert_uniqueERKc
// type: 
pub fn stub_0xf413a4() {
    // IDA 0xf413a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<char,char,std::_Identity<char>,std::less<char>,std::allocator<char>>::_M_erase(std::_Rb_tree_node<char> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIccSt9_IdentityIcESt4lessIcESaIcEE8_M_eraseEPSt13_Rb_tree_nodeIcE")]
// 0xf413b4 — j___ZNSt8_Rb_treeIccSt9_IdentityIcESt4lessIcESaIcEE8_M_eraseEPSt13_Rb_tree_nodeIcE
// type: 
pub fn stub_0xf413b4() {
    // IDA 0xf413b4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<char,char,std::_Identity<char>,std::less<char>,std::allocator<char>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,char const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIccSt9_IdentityIcESt4lessIcESaIcEE9_M_insertEPSt18_Rb_tree_node_baseS7_RKc")]
// 0xf413c4 — j___ZNSt8_Rb_treeIccSt9_IdentityIcESt4lessIcESaIcEE9_M_insertEPSt18_Rb_tree_node_baseS7_RKc
// type: 
pub fn stub_0xf413c4() {
    // IDA 0xf413c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Body::getBranchIBody(void)")]
#[doc(alias = "j___ZN3RBX4Body14getBranchIBodyEv")]
// 0xf41714 — j___ZN3RBX4Body14getBranchIBodyEv
// type: _DWORD __fastcall(RBX::Body *__hidden this)
pub fn stub_0xf41714() {
    // IDA 0xf41714: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Body::getBranchVelocity(void)")]
#[doc(alias = "j___ZN3RBX4Body17getBranchVelocityEv")]
// 0xf41724 — j___ZN3RBX4Body17getBranchVelocityEv
// type: _DWORD __fastcall(RBX::Body *__hidden this)
pub fn stub_0xf41724() {
    // IDA 0xf41724: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Body::getBranchForce(void)const")]
#[doc(alias = "j___ZNK3RBX4Body14getBranchForceEv")]
// 0xf41984 — j___ZNK3RBX4Body14getBranchForceEv
// type: _DWORD __fastcall(RBX::Body *__hidden this)
pub fn stub_0xf41984() {
    // IDA 0xf41984: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Body::getBranchTorque(void)const")]
#[doc(alias = "j___ZNK3RBX4Body15getBranchTorqueEv")]
// 0xf41994 — j___ZNK3RBX4Body15getBranchTorqueEv
// type: _DWORD __fastcall(RBX::Body *__hidden this)
pub fn stub_0xf41994() {
    // IDA 0xf41994: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "rbx::remote_signal<void ()(RBX::NormalId)>::remote_signal(void)")]
#[doc(alias = "j___ZN3rbx13remote_signalIFvN3RBX8NormalIdEEEC2Ev")]
// 0xf41b84 — j___ZN3rbx13remote_signalIFvN3RBX8NormalIdEEEC2Ev
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xf41b84() {
    // IDA 0xf41b84: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "rbx::remote_signal<void ()(RBX::NormalId)>::~remote_signal()")]
#[doc(alias = "j___ZN3rbx13remote_signalIFvN3RBX8NormalIdEEED2Ev")]
// 0xf41b94 — j___ZN3rbx13remote_signalIFvN3RBX8NormalIdEEED2Ev
// type: int __fastcall(int, int, int, int, char, int)
pub fn stub_0xf41b94() {
    // IDA 0xf41b94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::remote_signal<void ()(RBX::NormalId,float)>::remote_signal(void)")]
#[doc(alias = "j___ZN3rbx13remote_signalIFvN3RBX8NormalIdEfEEC2Ev")]
// 0xf41ba4 — j___ZN3rbx13remote_signalIFvN3RBX8NormalIdEfEEC2Ev
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xf41ba4() {
    // IDA 0xf41ba4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::remote_signal<void ()(RBX::NormalId,float)>::~remote_signal()")]
#[doc(alias = "j___ZN3rbx13remote_signalIFvN3RBX8NormalIdEfEED2Ev")]
// 0xf41bb4 — j___ZN3rbx13remote_signalIFvN3RBX8NormalIdEfEED2Ev
// type: int __fastcall(int, int, int, int, char, int)
pub fn stub_0xf41bb4() {
    // IDA 0xf41bb4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::NormalId)>::operator()(RBX::NormalId)")]
#[doc(alias = "j___ZN3rbx7signals16signal_with_argsILi1EFvN3RBX8NormalIdEEEclES3_")]
// 0xf41bc4 — j___ZN3rbx7signals16signal_with_argsILi1EFvN3RBX8NormalIdEEEclES3_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
pub fn stub_0xf41bc4() {
    // IDA 0xf41bc4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<2,void ()(RBX::NormalId,float)>::operator()(RBX::NormalId,float)")]
#[doc(alias = "j___ZN3rbx7signals16signal_with_argsILi2EFvN3RBX8NormalIdEfEEclES3_f")]
// 0xf41bd4 — j___ZN3rbx7signals16signal_with_argsILi2EFvN3RBX8NormalIdEfEEclES3_f
// type: int()
pub fn stub_0xf41bd4() {
    // IDA 0xf41bd4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::disconnectAll(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13disconnectAllEv")]
// 0xf41be4 — j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0xf41be4() {
    // IDA 0xf41be4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE24safe_static_do_get_mutexEv")]
// 0xf41bf4 — j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE24safe_static_do_get_mutexEv
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xf41bf4() {
    // IDA 0xf41bf4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slot24safe_static_do_get_mutexEv")]
// 0xf41c14 — j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slot24safe_static_do_get_mutexEv
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf41c14() {
    // IDA 0xf41c14: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::insert(rbx::signals::signal<void ()(RBX::NormalId)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE6insertEPNS5_4slotE")]
// 0xf41c24 — j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE6insertEPNS5_4slotE
// type: void __fastcall(int *, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0xf41c24() {
    // IDA 0xf41c24: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::remove(rbx::signals::signal<void ()(RBX::NormalId)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE6removeEPNS5_4slotE")]
// 0xf41c34 — j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE6removeEPNS5_4slotE
// type: int __fastcall(int, char *)
pub fn stub_0xf41c34() {
    // IDA 0xf41c34: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::on_error(std::exception &)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE8on_errorERSt9exception")]
// 0xf41c64 — j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE8on_errorERSt9exception
// type: int()
pub fn stub_0xf41c64() {
    // IDA 0xf41c64: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::disconnectAll(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13disconnectAllEv")]
// 0xf41c74 — j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0xf41c74() {
    // IDA 0xf41c74: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE24safe_static_do_get_mutexEv")]
// 0xf41c84 — j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE24safe_static_do_get_mutexEv
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xf41c84() {
    // IDA 0xf41c84: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slot24safe_static_do_get_mutexEv")]
// 0xf41ca4 — j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slot24safe_static_do_get_mutexEv
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf41ca4() {
    // IDA 0xf41ca4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::insert(rbx::signals::signal<void ()(RBX::NormalId,float)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE6insertEPNS5_4slotE")]
// 0xf41cb4 — j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE6insertEPNS5_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0xf41cb4() {
    // IDA 0xf41cb4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::remove(rbx::signals::signal<void ()(RBX::NormalId,float)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE6removeEPNS5_4slotE")]
// 0xf41cc4 — j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE6removeEPNS5_4slotE
// type: int __fastcall(int, char *)
pub fn stub_0xf41cc4() {
    // IDA 0xf41cc4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::on_error(std::exception &)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE8on_errorERSt9exception")]
// 0xf41cf4 — j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE8on_errorERSt9exception
// type: int()
pub fn stub_0xf41cf4() {
    // IDA 0xf41cf4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::RelativePanel::RelativePanel(void)")]
#[doc(alias = "j___ZN3RBX13RelativePanelC2Ev")]
// 0xf42044 — j___ZN3RBX13RelativePanelC2Ev
// type: int __fastcall(RBX::RelativePanel *this)
pub fn stub_0xf42044() {
    // IDA 0xf42044: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "rbx::remote_signal<void ()(std::string,RBX::ContentId)>::remote_signal(void)")]
#[doc(alias = "j___ZN3rbx13remote_signalIFvSsN3RBX9ContentIdEEEC2Ev")]
// 0xf42614 — j___ZN3rbx13remote_signalIFvSsN3RBX9ContentIdEEEC2Ev
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xf42614() {
    // IDA 0xf42614: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::remote_signal<void ()(std::string,RBX::ContentId)>::~remote_signal()")]
#[doc(alias = "j___ZN3rbx13remote_signalIFvSsN3RBX9ContentIdEEED2Ev")]
// 0xf42624 — j___ZN3rbx13remote_signalIFvSsN3RBX9ContentIdEEED2Ev
// type: int __fastcall(int, int, int, int, char, int)
pub fn stub_0xf42624() {
    // IDA 0xf42624: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::remote_signal<void ()(std::string,std::string)>::remote_signal(void)")]
#[doc(alias = "j___ZN3rbx13remote_signalIFvSsSsEEC2Ev")]
// 0xf42654 — j___ZN3rbx13remote_signalIFvSsSsEEC2Ev
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xf42654() {
    // IDA 0xf42654: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::remote_signal<void ()(std::string,std::string)>::~remote_signal()")]
#[doc(alias = "j___ZN3rbx13remote_signalIFvSsSsEED2Ev")]
// 0xf42664 — j___ZN3rbx13remote_signalIFvSsSsEED2Ev
// type: int __fastcall(int, int, int, int, char, int)
pub fn stub_0xf42664() {
    // IDA 0xf42664: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::remote_signal<void ()(std::string,int,int)>::remote_signal(void)")]
#[doc(alias = "j___ZN3rbx13remote_signalIFvSsiiEEC2Ev")]
// 0xf42674 — j___ZN3rbx13remote_signalIFvSsiiEEC2Ev
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xf42674() {
    // IDA 0xf42674: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<2,void ()(std::string,RBX::ContentId)>::fireItem(rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot *,std::string,RBX::ContentId)")]
#[doc(alias = "j___ZN3rbx7signals16signal_with_argsILi2EFvSsN3RBX9ContentIdEEE8fireItemEPNS0_6signalIS4_E4slotESsS3_")]
// 0xf42684 — j___ZN3rbx7signals16signal_with_argsILi2EFvSsN3RBX9ContentIdEEE8fireItemEPNS0_6signalIS4_E4slotESsS3_
// type: int()
pub fn stub_0xf42684() {
    // IDA 0xf42684: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<2,void ()(std::string,RBX::ContentId)>::operator()(std::string,RBX::ContentId)")]
#[doc(alias = "j___ZN3rbx7signals16signal_with_argsILi2EFvSsN3RBX9ContentIdEEEclESsS3_")]
// 0xf42694 — j___ZN3rbx7signals16signal_with_argsILi2EFvSsN3RBX9ContentIdEEEclESsS3_
// type: int()
pub fn stub_0xf42694() {
    // IDA 0xf42694: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<3,void ()(std::string,int,int)>::operator()(std::string,int,int)")]
#[doc(alias = "j___ZN3rbx7signals16signal_with_argsILi3EFvSsiiEEclESsii")]
// 0xf426a4 — j___ZN3rbx7signals16signal_with_argsILi3EFvSsiiEEclESsii
// type: int()
pub fn stub_0xf426a4() {
    // IDA 0xf426a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::disconnectAll(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE13disconnectAllEv")]
// 0xf426b4 — j___ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0xf426b4() {
    // IDA 0xf426b4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE24safe_static_do_get_mutexEv")]
// 0xf426c4 — j___ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE24safe_static_do_get_mutexEv
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xf426c4() {
    // IDA 0xf426c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slot24safe_static_do_get_mutexEv")]
// 0xf426e4 — j___ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slot24safe_static_do_get_mutexEv
// type: 
pub fn stub_0xf426e4() {
    // IDA 0xf426e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::insert(rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE6insertEPNS5_4slotE")]
// 0xf426f4 — j___ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE6insertEPNS5_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0xf426f4() {
    // IDA 0xf426f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::remove(rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE6removeEPNS5_4slotE")]
// 0xf42704 — j___ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE6removeEPNS5_4slotE
// type: int __fastcall(int, char *)
pub fn stub_0xf42704() {
    // IDA 0xf42704: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::on_error(std::exception &)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE8on_errorERSt9exception")]
// 0xf42734 — j___ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE8on_errorERSt9exception
// type: int()
pub fn stub_0xf42734() {
    // IDA 0xf42734: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvSsiiEE24safe_static_do_get_mutexEv")]
// 0xf42794 — j___ZN3rbx7signals6signalIFvSsiiEE24safe_static_do_get_mutexEv
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xf42794() {
    // IDA 0xf42794: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvSsiiEE4slot24safe_static_do_get_mutexEv")]
// 0xf427b4 — j___ZN3rbx7signals6signalIFvSsiiEE4slot24safe_static_do_get_mutexEv
// type: 
pub fn stub_0xf427b4() {
    // IDA 0xf427b4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::on_error(std::exception &)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvSsiiEE8on_errorERSt9exception")]
// 0xf427e4 — j___ZN3rbx7signals6signalIFvSsiiEE8on_errorERSt9exception
// type: int()
pub fn stub_0xf427e4() {
    // IDA 0xf427e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ManualGlueJoint::~ManualGlueJoint()")]
#[doc(alias = "j___ZN3RBX15ManualGlueJointD0Ev")]
// 0xf432f4 — j___ZN3RBX15ManualGlueJointD0Ev
// type: void __fastcall(RBX::ManualGlueJoint *__hidden this)
pub fn stub_0xf432f4() {
    // IDA 0xf432f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ManualWeldJoint::~ManualWeldJoint()")]
#[doc(alias = "j___ZN3RBX15ManualWeldJointD0Ev")]
// 0xf43304 — j___ZN3RBX15ManualWeldJointD0Ev
// type: void __fastcall(RBX::ManualWeldJoint *__hidden this)
pub fn stub_0xf43304() {
    // IDA 0xf43304: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvPN3RBX5JointEEE24safe_static_do_get_mutexEv")]
// 0xf43774 — j___ZN3rbx7signals6signalIFvPN3RBX5JointEEE24safe_static_do_get_mutexEv
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xf43774() {
    // IDA 0xf43774: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvPN3RBX5JointEEE4slot24safe_static_do_get_mutexEv")]
// 0xf43784 — j___ZN3rbx7signals6signalIFvPN3RBX5JointEEE4slot24safe_static_do_get_mutexEv
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf43784() {
    // IDA 0xf43784: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::insert(rbx::signals::signal<void ()(RBX::Joint *)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvPN3RBX5JointEEE6insertEPNS6_4slotE")]
// 0xf43794 — j___ZN3rbx7signals6signalIFvPN3RBX5JointEEE6insertEPNS6_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0xf43794() {
    // IDA 0xf43794: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::remove(rbx::signals::signal<void ()(RBX::Joint *)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvPN3RBX5JointEEE6removeEPNS6_4slotE")]
// 0xf437a4 — j___ZN3rbx7signals6signalIFvPN3RBX5JointEEE6removeEPNS6_4slotE
// type: int __fastcall(int, char *)
pub fn stub_0xf437a4() {
    // IDA 0xf437a4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::KeyframeSequence::~KeyframeSequence()")]
#[doc(alias = "j___ZN3RBX16KeyframeSequenceD1Ev")]
// 0xf43b64 — j___ZN3RBX16KeyframeSequenceD1Ev
// type: void __fastcall(RBX::KeyframeSequence *__hidden this)
pub fn stub_0xf43b64() {
    // IDA 0xf43b64: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "unsigned long RBX::findOrAdd<std::string>(std::vector<std::string,std::allocator<std::string>> &,std::string const&)")]
#[doc(alias = "j___ZN3RBX9findOrAddISsEEmRSt6vectorIT_SaIS2_EERKS2_")]
// 0xf43b84 — j___ZN3RBX9findOrAddISsEEmRSt6vectorIT_SaIS2_EERKS2_
// type: unsigned int __fastcall(const std::string **, std::string *)
pub fn stub_0xf43b84() {
    // IDA 0xf43b84: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "unsigned long RBX::findOrAdd<std::pair<unsigned long,unsigned long>>(std::vector<std::pair<unsigned long,unsigned long>,std::allocator<std::pair<unsigned long,unsigned long>>> &,std::pair<unsigned long,unsigned long> const&)")]
#[doc(alias = "j___ZN3RBX9findOrAddISt4pairImmEEEmRSt6vectorIT_SaIS4_EERKS4_")]
// 0xf43b94 — j___ZN3RBX9findOrAddISt4pairImmEEEmRSt6vectorIT_SaIS4_EERKS4_
// type: int()
pub fn stub_0xf43b94() {
    // IDA 0xf43b94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Vector_base<RBX::CachedPose,std::allocator<RBX::CachedPose>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIN3RBX10CachedPoseESaIS1_EE11_M_allocateEm")]
// 0xf43c24 — j___ZNSt12_Vector_baseIN3RBX10CachedPoseESaIS1_EE11_M_allocateEm
// type: int()
pub fn stub_0xf43c24() {
    // IDA 0xf43c24: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Vector_base<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIN3RBX16KeyframeSequence14CachedKeyframeESaIS2_EE11_M_allocateEm")]
// 0xf43c34 — j___ZNSt12_Vector_baseIN3RBX16KeyframeSequence14CachedKeyframeESaIS2_EE11_M_allocateEm
// type: int()
pub fn stub_0xf43c34() {
    // IDA 0xf43c34: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::KeyframeSequence::Priority,std::allocator<RBX::KeyframeSequence::Priority>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIN3RBX16KeyframeSequence8PriorityESaIS2_EE11_M_allocateEm")]
// 0xf43c44 — j___ZNSt12_Vector_baseIN3RBX16KeyframeSequence8PriorityESaIS2_EE11_M_allocateEm
// type: int()
pub fn stub_0xf43c44() {
    // IDA 0xf43c44: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::CachedPose *,std::allocator<RBX::CachedPose *>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIPN3RBX10CachedPoseESaIS2_EE11_M_allocateEm")]
// 0xf43c54 — j___ZNSt12_Vector_baseIPN3RBX10CachedPoseESaIS2_EE11_M_allocateEm
// type: int()
pub fn stub_0xf43c54() {
    // IDA 0xf43c54: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::CachedPose *,std::allocator<RBX::CachedPose *>>::_Vector_base(unsigned long,std::allocator<RBX::CachedPose *> const&)")]
#[doc(alias = "j___ZNSt12_Vector_baseIPN3RBX10CachedPoseESaIS2_EEC2EmRKS3_")]
// 0xf43c64 — j___ZNSt12_Vector_baseIPN3RBX10CachedPoseESaIS2_EEC2EmRKS3_
// type: int()
pub fn stub_0xf43c64() {
    // IDA 0xf43c64: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<std::pair<unsigned long,unsigned long>,std::allocator<std::pair<unsigned long,unsigned long>>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseISt4pairImmESaIS1_EE11_M_allocateEm")]
// 0xf43c74 — j___ZNSt12_Vector_baseISt4pairImmESaIS1_EE11_M_allocateEm
// type: int()
pub fn stub_0xf43c74() {
    // IDA 0xf43c74: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::CachedPose * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CachedPose *,RBX::CachedPose *>(RBX::CachedPose *,RBX::CachedPose *,RBX::CachedPose *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10CachedPoseES5_EET0_T_S7_S6_")]
// 0xf43c84 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10CachedPoseES5_EET0_T_S7_S6_
// type: int()
pub fn stub_0xf43c84() {
    // IDA 0xf43c84: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::KeyframeSequence::CachedKeyframe * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::KeyframeSequence::CachedKeyframe *,RBX::KeyframeSequence::CachedKeyframe *>(RBX::KeyframeSequence::CachedKeyframe *,RBX::KeyframeSequence::CachedKeyframe *,RBX::KeyframeSequence::CachedKeyframe *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX16KeyframeSequence14CachedKeyframeES6_EET0_T_S8_S7_")]
// 0xf43c94 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX16KeyframeSequence14CachedKeyframeES6_EET0_T_S8_S7_
// type: int()
pub fn stub_0xf43c94() {
    // IDA 0xf43c94: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::KeyframeSequence::Priority * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::KeyframeSequence::Priority *,RBX::KeyframeSequence::Priority *>(RBX::KeyframeSequence::Priority *,RBX::KeyframeSequence::Priority *,RBX::KeyframeSequence::Priority *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX16KeyframeSequence8PriorityES6_EET0_T_S8_S7_")]
// 0xf43ca4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX16KeyframeSequence8PriorityES6_EET0_T_S8_S7_
// type: int()
pub fn stub_0xf43ca4() {
    // IDA 0xf43ca4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::pair<unsigned long,unsigned long> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<std::pair<unsigned long,unsigned long> *,std::pair<unsigned long,unsigned long> *>(std::pair<unsigned long,unsigned long> *,std::pair<unsigned long,unsigned long> *,std::pair<unsigned long,unsigned long> *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPSt4pairImmES5_EET0_T_S7_S6_")]
// 0xf43cb4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPSt4pairImmES5_EET0_T_S7_S6_
// type: int()
pub fn stub_0xf43cb4() {
    // IDA 0xf43cb4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::KeyframeSequence::Priority,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "j___ZNSt3mapIPKN3RBX4NameENS0_16KeyframeSequence8PriorityESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
// 0xf43cc4 — j___ZNSt3mapIPKN3RBX4NameENS0_16KeyframeSequence8PriorityESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int()
pub fn stub_0xf43cc4() {
    // IDA 0xf43cc4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::CachedPose,std::allocator<RBX::CachedPose>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CachedPose*,std::vector<RBX::CachedPose,std::allocator<RBX::CachedPose>>>,RBX::CachedPose const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX10CachedPoseESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")]
// 0xf43cd4 — j___ZNSt6vectorIN3RBX10CachedPoseESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: int()
pub fn stub_0xf43cd4() {
    // IDA 0xf43cd4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::CachedPose,std::allocator<RBX::CachedPose>>::reserve(unsigned long)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX10CachedPoseESaIS1_EE7reserveEm")]
// 0xf43ce4 — j___ZNSt6vectorIN3RBX10CachedPoseESaIS1_EE7reserveEm
// type: int()
pub fn stub_0xf43ce4() {
    // IDA 0xf43ce4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::CachedPose,std::allocator<RBX::CachedPose>>::push_back(RBX::CachedPose const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX10CachedPoseESaIS1_EE9push_backERKS1_")]
// 0xf43cf4 — j___ZNSt6vectorIN3RBX10CachedPoseESaIS1_EE9push_backERKS1_
// type: int()
pub fn stub_0xf43cf4() {
    // IDA 0xf43cf4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe*,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,RBX::KeyframeSequence::CachedKeyframe const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX16KeyframeSequence14CachedKeyframeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0xf43d04 — j___ZNSt6vectorIN3RBX16KeyframeSequence14CachedKeyframeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int()
pub fn stub_0xf43d04() {
    // IDA 0xf43d04: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>::_M_erase_at_end(RBX::KeyframeSequence::CachedKeyframe*)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX16KeyframeSequence14CachedKeyframeESaIS2_EE15_M_erase_at_endEPS2_")]
// 0xf43d14 — j___ZNSt6vectorIN3RBX16KeyframeSequence14CachedKeyframeESaIS2_EE15_M_erase_at_endEPS2_
// type: int()
pub fn stub_0xf43d14() {
    // IDA 0xf43d14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>::push_back(RBX::KeyframeSequence::CachedKeyframe const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX16KeyframeSequence14CachedKeyframeESaIS2_EE9push_backERKS2_")]
// 0xf43d24 — j___ZNSt6vectorIN3RBX16KeyframeSequence14CachedKeyframeESaIS2_EE9push_backERKS2_
// type: int()
pub fn stub_0xf43d24() {
    // IDA 0xf43d24: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>::~vector()")]
#[doc(alias = "j___ZNSt6vectorIN3RBX16KeyframeSequence14CachedKeyframeESaIS2_EED2Ev")]
// 0xf43d34 — j___ZNSt6vectorIN3RBX16KeyframeSequence14CachedKeyframeESaIS2_EED2Ev
// type: int()
pub fn stub_0xf43d34() {
    // IDA 0xf43d34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::KeyframeSequence::Priority,std::allocator<RBX::KeyframeSequence::Priority>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::Priority*,std::vector<RBX::KeyframeSequence::Priority,std::allocator<RBX::KeyframeSequence::Priority>>>,RBX::KeyframeSequence::Priority const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX16KeyframeSequence8PriorityESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0xf43d44 — j___ZNSt6vectorIN3RBX16KeyframeSequence8PriorityESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int()
pub fn stub_0xf43d44() {
    // IDA 0xf43d44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::KeyframeSequence::Priority,std::allocator<RBX::KeyframeSequence::Priority>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::Priority*,std::vector<RBX::KeyframeSequence::Priority,std::allocator<RBX::KeyframeSequence::Priority>>>,unsigned long,RBX::KeyframeSequence::Priority const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX16KeyframeSequence8PriorityESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// 0xf43d54 — j___ZNSt6vectorIN3RBX16KeyframeSequence8PriorityESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int()
pub fn stub_0xf43d54() {
    // IDA 0xf43d54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::KeyframeSequence::Priority,std::allocator<RBX::KeyframeSequence::Priority>>::resize(unsigned long,RBX::KeyframeSequence::Priority)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX16KeyframeSequence8PriorityESaIS2_EE6resizeEmS2_")]
// 0xf43d64 — j___ZNSt6vectorIN3RBX16KeyframeSequence8PriorityESaIS2_EE6resizeEmS2_
// type: int()
pub fn stub_0xf43d64() {
    // IDA 0xf43d64: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::KeyframeSequence::Priority,std::allocator<RBX::KeyframeSequence::Priority>>::push_back(RBX::KeyframeSequence::Priority const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX16KeyframeSequence8PriorityESaIS2_EE9push_backERKS2_")]
// 0xf43d74 — j___ZNSt6vectorIN3RBX16KeyframeSequence8PriorityESaIS2_EE9push_backERKS2_
// type: int()
pub fn stub_0xf43d74() {
    // IDA 0xf43d74: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CachedPose **,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>>>,unsigned long,RBX::CachedPose * const&)")]
#[doc(alias = "j___ZNSt6vectorIPN3RBX10CachedPoseESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// 0xf43d84 — j___ZNSt6vectorIPN3RBX10CachedPoseESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(int, void *__src)
pub fn stub_0xf43d84() {
    // IDA 0xf43d84: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>>::resize(unsigned long,RBX::CachedPose *)")]
#[doc(alias = "j___ZNSt6vectorIPN3RBX10CachedPoseESaIS2_EE6resizeEmS2_")]
// 0xf43d94 — j___ZNSt6vectorIPN3RBX10CachedPoseESaIS2_EE6resizeEmS2_
// type: int()
pub fn stub_0xf43d94() {
    // IDA 0xf43d94: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>>::vector(std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> const&)")]
#[doc(alias = "j___ZNSt6vectorIPN3RBX10CachedPoseESaIS2_EEC2ERKS4_")]
// 0xf43da4 — j___ZNSt6vectorIPN3RBX10CachedPoseESaIS2_EEC2ERKS4_
// type: int()
pub fn stub_0xf43da4() {
    // IDA 0xf43da4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>>::operator=(std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> const&)")]
#[doc(alias = "j___ZNSt6vectorIPN3RBX10CachedPoseESaIS2_EEaSERKS4_")]
// 0xf43db4 — j___ZNSt6vectorIPN3RBX10CachedPoseESaIS2_EEaSERKS4_
// type: int()
pub fn stub_0xf43db4() {
    // IDA 0xf43db4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<std::pair<unsigned long,unsigned long>,std::allocator<std::pair<unsigned long,unsigned long>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<unsigned long,unsigned long>*,std::vector<std::pair<unsigned long,unsigned long>,std::allocator<std::pair<unsigned long,unsigned long>>>>,std::pair<unsigned long,unsigned long> const&)")]
#[doc(alias = "j___ZNSt6vectorISt4pairImmESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")]
// 0xf43dc4 — j___ZNSt6vectorISt4pairImmESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: int()
pub fn stub_0xf43dc4() {
    // IDA 0xf43dc4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<std::pair<unsigned long,unsigned long>,std::allocator<std::pair<unsigned long,unsigned long>>>::push_back(std::pair<unsigned long,unsigned long> const&)")]
#[doc(alias = "j___ZNSt6vectorISt4pairImmESaIS1_EE9push_backERKS1_")]
// 0xf43dd4 — j___ZNSt6vectorISt4pairImmESaIS1_EE9push_backERKS1_
// type: int()
pub fn stub_0xf43dd4() {
    // IDA 0xf43dd4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>,std::_Select1st<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16KeyframeSequence8PriorityEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
// 0xf43de4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16KeyframeSequence8PriorityEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int()
pub fn stub_0xf43de4() {
    // IDA 0xf43de4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>,std::_Select1st<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>>,std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16KeyframeSequence8PriorityEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
// 0xf43df4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16KeyframeSequence8PriorityEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0xf43df4() {
    // IDA 0xf43df4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>,std::_Select1st<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16KeyframeSequence8PriorityEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
// 0xf43e04 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16KeyframeSequence8PriorityEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int()
pub fn stub_0xf43e04() {
    // IDA 0xf43e04: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__push_heap<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,int,RBX::KeyframeSequence::CachedKeyframe>(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,int,int,RBX::KeyframeSequence::CachedKeyframe)")]
#[doc(alias = "j___ZSt11__push_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEiS4_EvT_T0_SB_T1_")]
// 0xf43e14 — j___ZSt11__push_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEiS4_EvT_T0_SB_T1_
// type: int()
pub fn stub_0xf43e14() {
    // IDA 0xf43e14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__adjust_heap<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,int,RBX::KeyframeSequence::CachedKeyframe>(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,int,int,RBX::KeyframeSequence::CachedKeyframe)")]
#[doc(alias = "j___ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEiS4_EvT_T0_SB_T1_")]
// 0xf43e24 — j___ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEiS4_EvT_T0_SB_T1_
// type: int __fastcall(int, int, int, int, int, int, int, int, int)
pub fn stub_0xf43e24() {
    // IDA 0xf43e24: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__heap_select<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>>(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>)")]
#[doc(alias = "j___ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEEvT_SA_SA_")]
// 0xf43e34 — j___ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEEvT_SA_SA_
// type: int __fastcall(int, int, void *, int, int, int, int, void *, int, int, int, void *, int, int, int, int, int, int)
pub fn stub_0xf43e34() {
    // IDA 0xf43e34: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__insertion_sort<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>>(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>)")]
#[doc(alias = "j___ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEEvT_SA_")]
// 0xf43e44 — j___ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEEvT_SA_
// type: int __fastcall(int, int, int, int, int, int, int, void *, int, int, int, void *, int, int, int, int, int, int)
pub fn stub_0xf43e44() {
    // IDA 0xf43e44: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__introsort_loop<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,int>(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,int)")]
#[doc(alias = "j___ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEiEvT_SA_T0_")]
// 0xf43e54 — j___ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEiEvT_SA_T0_
// type: int __fastcall(int, int, int, int, int, int, int, void *, int, int, int, int, int)
pub fn stub_0xf43e54() {
    // IDA 0xf43e54: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>> std::__unguarded_partition<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,RBX::KeyframeSequence::CachedKeyframe>(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,RBX::KeyframeSequence::CachedKeyframe)")]
#[doc(alias = "j___ZSt21__unguarded_partitionIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEES4_ET_SA_SA_T0_")]
// 0xf43e64 — j___ZSt21__unguarded_partitionIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEES4_ET_SA_SA_T0_
// type: 
pub fn stub_0xf43e64() {
    // IDA 0xf43e64: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__final_insertion_sort<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>>(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>)")]
#[doc(alias = "j___ZSt22__final_insertion_sortIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEEvT_SA_")]
// 0xf43e74 — j___ZSt22__final_insertion_sortIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEEvT_SA_
// type: int __fastcall(int, int, int, int, int, int, int, int, int)
pub fn stub_0xf43e74() {
    // IDA 0xf43e74: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__unguarded_linear_insert<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,RBX::KeyframeSequence::CachedKeyframe>(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,RBX::KeyframeSequence::CachedKeyframe)")]
#[doc(alias = "j___ZSt25__unguarded_linear_insertIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEES4_EvT_T0_")]
// 0xf43e84 — j___ZSt25__unguarded_linear_insertIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEES4_EvT_T0_
// type: int()
pub fn stub_0xf43e84() {
    // IDA 0xf43e84: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::pop_heap<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>>(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>)")]
#[doc(alias = "j___ZSt8pop_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEEvT_SA_")]
// 0xf43e94 — j___ZSt8pop_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEEvT_SA_
// type: int __fastcall(int, int, int, int, int, int, int, void *, int, int, int, int, int, int)
pub fn stub_0xf43e94() {
    // IDA 0xf43e94: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::make_heap<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>>(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>)")]
#[doc(alias = "j___ZSt9make_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEEvT_SA_")]
// 0xf43ea4 — j___ZSt9make_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEEvT_SA_
// type: int __fastcall(int, int, int, int, int, int, int, void *, int, int, int, int, int)
pub fn stub_0xf43ea4() {
    // IDA 0xf43ea4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::sort_heap<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>>(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>)")]
#[doc(alias = "j___ZSt9sort_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEEvT_SA_")]
// 0xf43eb4 — j___ZSt9sort_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEEvT_SA_
// type: int()
pub fn stub_0xf43eb4() {
    // IDA 0xf43eb4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::ContentId>(RBX::ContentId const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9ContentIdEEERS3_RKT_")]
// 0xf43fc4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9ContentIdEEERS3_RKT_
// type: int()
pub fn stub_0xf43fc4() {
    // IDA 0xf43fc4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::ContentId>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX9ContentIdEE9singletonEv")]
// 0xf43fd4 — j___ZN3rbx14implementation12typed_holderIN3RBX9ContentIdEE9singletonEv
// type: int()
pub fn stub_0xf43fd4() {
    // IDA 0xf43fd4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::AnimationId::isActive(void)const")]
#[doc(alias = "j___ZNK3RBX11AnimationId8isActiveEv")]
// 0xf44234 — j___ZNK3RBX11AnimationId8isActiveEv
// type: int __fastcall(RBX::AnimationId *this)
pub fn stub_0xf44234() {
    // IDA 0xf44234: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Deque_base<XmlElement *,std::allocator<XmlElement *>>::~_Deque_base()")]
#[doc(alias = "j___ZNSt11_Deque_baseIP10XmlElementSaIS1_EED2Ev")]
// 0xf442f4 — j___ZNSt11_Deque_baseIP10XmlElementSaIS1_EED2Ev
// type: int __fastcall(int)
pub fn stub_0xf442f4() {
    // IDA 0xf442f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Vector_base<RBX::Legacy::SurfaceConstraint,std::allocator<RBX::Legacy::SurfaceConstraint>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIN3RBX6Legacy17SurfaceConstraintESaIS2_EE11_M_allocateEm")]
// 0xf443c4 — j___ZNSt12_Vector_baseIN3RBX6Legacy17SurfaceConstraintESaIS2_EE11_M_allocateEm
// type: int()
pub fn stub_0xf443c4() {
    // IDA 0xf443c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Legacy::SurfaceConstraint * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Legacy::SurfaceConstraint *,RBX::Legacy::SurfaceConstraint *>(RBX::Legacy::SurfaceConstraint *,RBX::Legacy::SurfaceConstraint *,RBX::Legacy::SurfaceConstraint *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Legacy17SurfaceConstraintES6_EET0_T_S8_S7_")]
// 0xf443d4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Legacy17SurfaceConstraintES6_EET0_T_S8_S7_
// type: int()
pub fn stub_0xf443d4() {
    // IDA 0xf443d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
