//! core shard lp — 150 core stubs EA-sorted, next uncovered after shard lo (0x7f9cec..0x825604, lowest EA first).
//! Source: ida/export.json filtered where demangled excludes Reflection|Instance|DataModel|Ogre|G3D|RakNet|FMOD|Lua|Script, EA-sorted, next 150 uncovered (lowest EA first, rbx_core::SharedPtr not boost) [skeleton batch].
//! Format: // 0xADDR — mangled + #[doc(alias = "mangled")] + pub fn stub_0xADDR todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::Soundscape::CollisionSoundManager::~CollisionSoundManager()")]
#[doc(alias = "__ZN3RBX10Soundscape21CollisionSoundManagerD2Ev")]
// 0x7f9cec — __ZN3RBX10Soundscape21CollisionSoundManagerD2Ev
// type: void __fastcall(RBX::Soundscape::CollisionSoundManager *__hidden this)
pub fn stub_0x7f9cec() {
    // IDA 0x7f9cec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Soundscape::CollisionSoundManager::PlaySound(std::pair<RBX::Primitive *,RBX::Primitive *>)")]
#[doc(alias = "__ZN3RBX10Soundscape21CollisionSoundManager9PlaySoundESt4pairIPNS_9PrimitiveES4_E")]
// 0x7fcd04 — __ZN3RBX10Soundscape21CollisionSoundManager9PlaySoundESt4pairIPNS_9PrimitiveES4_E
pub fn stub_0x7fcd04() {
    // IDA 0x7fcd04: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void rbx_core::SharedPtr<RBX::Soundscape::Sound>::reset<RBX::Soundscape::Sound>(RBX::Soundscape::Sound *)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10Soundscape5SoundEE5resetIS3_EEvPT_")]
// 0x7fcf70 — __ZN5boost10shared_ptrIN3RBX10Soundscape5SoundEE5resetIS3_EEvPT_
pub fn stub_0x7fcf70() {
    // IDA 0x7fcf70: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE7connectIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10Soundscape21CollisionSoundManagerES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEEEENS0_10connectionERKT_")]
// 0x7fcf9c — __ZN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE7connectIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10Soundscape21CollisionSoundManagerES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEEEENS0_10connectionERKT_
pub fn stub_0x7fcf9c() {
    // IDA 0x7fcf9c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Primitive *)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_10Soundscape21CollisionSoundManagerES4_EENS9_5list2INS9_5valueIPSE_EENS8_3argILi1EEEEEEEEENS0_10connectionERKT_")]
// 0x7fd010 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_10Soundscape21CollisionSoundManagerES4_EENS9_5list2INS9_5valueIPSE_EENS8_3argILi1EEEEEEEEENS0_10connectionERKT_
pub fn stub_0x7fd010() {
    // IDA 0x7fd010: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::map<RBX::Soundscape::CollisionSoundType,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>,std::less<RBX::Soundscape::CollisionSoundType>,std::allocator<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>>::operator[](RBX::Soundscape::CollisionSoundType const&)")]
#[doc(alias = "__ZNSt3mapIN3RBX10Soundscape18CollisionSoundTypeEN5boost10shared_ptrINS1_14CollisionSoundEEESt4lessIS2_ESaISt4pairIKS2_S6_EEEixERSA_")]
// 0x7fd084 — __ZNSt3mapIN3RBX10Soundscape18CollisionSoundTypeEN5boost10shared_ptrINS1_14CollisionSoundEEESt4lessIS2_ESaISt4pairIKS2_S6_EEEixERSA_
// type: int __fastcall(int, int *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: int __fastcall(int, int *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x7fd084() {
    // IDA 0x7fd084: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>::operator=(rbx_core::SharedPtr<RBX::Soundscape::CollisionSound> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10Soundscape14CollisionSoundEEaSERKS4_")]
// 0x7fd1cc — __ZN5boost10shared_ptrIN3RBX10Soundscape14CollisionSoundEEaSERKS4_
pub fn stub_0x7fd1cc() {
    // IDA 0x7fd1cc: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Soundscape::CollisionSoundType,std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>,std::_Select1st<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>,std::less<RBX::Soundscape::CollisionSoundType>,std::allocator<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>,std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX10Soundscape18CollisionSoundTypeESt4pairIKS2_N5boost10shared_ptrINS1_14CollisionSoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_")]
// 0x7fd204 — __ZNSt8_Rb_treeIN3RBX10Soundscape18CollisionSoundTypeESt4pairIKS2_N5boost10shared_ptrINS1_14CollisionSoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_
// type: int __fastcall(int, int, int)
pub fn stub_0x7fd204() {
    // IDA 0x7fd204: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Soundscape::CollisionSoundType,std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>,std::_Select1st<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>,std::less<RBX::Soundscape::CollisionSoundType>,std::allocator<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX10Soundscape18CollisionSoundTypeESt4pairIKS2_N5boost10shared_ptrINS1_14CollisionSoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_")]
// 0x7fd2b8 — __ZNSt8_Rb_treeIN3RBX10Soundscape18CollisionSoundTypeESt4pairIKS2_N5boost10shared_ptrINS1_14CollisionSoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_
// type: int __fastcall(int, int, int, int)
pub fn stub_0x7fd2b8() {
    // IDA 0x7fd2b8: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Soundscape::CollisionSoundType,std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>,std::_Select1st<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>,std::less<RBX::Soundscape::CollisionSoundType>,std::allocator<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>>::_M_insert_unique(std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX10Soundscape18CollisionSoundTypeESt4pairIKS2_N5boost10shared_ptrINS1_14CollisionSoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE16_M_insert_uniqueERKS9_")]
// 0x7fd304 — __ZNSt8_Rb_treeIN3RBX10Soundscape18CollisionSoundTypeESt4pairIKS2_N5boost10shared_ptrINS1_14CollisionSoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE16_M_insert_uniqueERKS9_
// type: int __fastcall(int, int, int)
pub fn stub_0x7fd304() {
    // IDA 0x7fd304: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Soundscape::CollisionSoundType,std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>,std::_Select1st<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>,std::less<RBX::Soundscape::CollisionSoundType>,std::allocator<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>>::_M_create_node(std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX10Soundscape18CollisionSoundTypeESt4pairIKS2_N5boost10shared_ptrINS1_14CollisionSoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE14_M_create_nodeERKS9_")]
// 0x7fd36c — __ZNSt8_Rb_treeIN3RBX10Soundscape18CollisionSoundTypeESt4pairIKS2_N5boost10shared_ptrINS1_14CollisionSoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE14_M_create_nodeERKS9_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0x7fd36c() {
    // IDA 0x7fd36c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>::shared_ptr<RBX::Soundscape::CollisionSound>(RBX::Soundscape::CollisionSound *)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10Soundscape14CollisionSoundEEC2IS3_EEPT_")]
// 0x7fd45c — __ZN5boost10shared_ptrIN3RBX10Soundscape14CollisionSoundEEC2IS3_EEPT_
pub fn stub_0x7fd45c() {
    // IDA 0x7fd45c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Soundscape::CollisionSound>(RBX::Soundscape::CollisionSound *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IN3RBX10Soundscape14CollisionSoundEEEPT_")]
// 0x7fd530 — __ZN5boost6detail12shared_countC2IN3RBX10Soundscape14CollisionSoundEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0x7fd530() {
    // IDA 0x7fd530: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::CollisionSound>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape14CollisionSoundEED1Ev")]
// 0x7fd640 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape14CollisionSoundEED1Ev
pub fn stub_0x7fd640() {
    // IDA 0x7fd640: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::CollisionSound>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape14CollisionSoundEED0Ev")]
// 0x7fd644 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape14CollisionSoundEED0Ev
pub fn stub_0x7fd644() {
    // IDA 0x7fd644: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::CollisionSound>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape14CollisionSoundEE7disposeEv")]
// 0x7fd648 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape14CollisionSoundEE7disposeEv
pub fn stub_0x7fd648() {
    // IDA 0x7fd648: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::CollisionSound>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape14CollisionSoundEE11get_deleterERKSt9type_info")]
// 0x7fd6f0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape14CollisionSoundEE11get_deleterERKSt9type_info
pub fn stub_0x7fd6f0() {
    // IDA 0x7fd6f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::CollisionSound>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape14CollisionSoundEE19get_untyped_deleterEv")]
// 0x7fd6f4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape14CollisionSoundEE19get_untyped_deleterEv
pub fn stub_0x7fd6f4() {
    // IDA 0x7fd6f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_10Soundscape21CollisionSoundManagerES4_EENS9_5list2INS9_5valueIPSE_EENS8_3argILi1EEEEEEEED1Ev")]
// 0x7fd6f8 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_10Soundscape21CollisionSoundManagerES4_EENS9_5list2INS9_5valueIPSE_EENS8_3argILi1EEEEEEEED1Ev
pub fn stub_0x7fd6f8() {
    // IDA 0x7fd6f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_10Soundscape21CollisionSoundManagerES4_EENS9_5list2INS9_5valueIPSE_EENS8_3argILi1EEEEEEEED0Ev")]
// 0x7fd724 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_10Soundscape21CollisionSoundManagerES4_EENS9_5list2INS9_5valueIPSE_EENS8_3argILi1EEEEEEEED0Ev
pub fn stub_0x7fd724() {
    // IDA 0x7fd724: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Primitive *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>,1,void ()(RBX::Primitive *)>::call(RBX::Primitive *)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_10Soundscape21CollisionSoundManagerES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_")]
// 0x7fd7f8 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_10Soundscape21CollisionSoundManagerES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_
pub fn stub_0x7fd7f8() {
    // IDA 0x7fd7f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Primitive *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>,1,void ()(RBX::Primitive *)>::call(RBX::Primitive *)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_10Soundscape21CollisionSoundManagerES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_")]
// 0x7fd80c — __ZThn4_N3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_10Soundscape21CollisionSoundManagerES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_
pub fn stub_0x7fd80c() {
    // IDA 0x7fd80c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>::operator()<RBX::Primitive *>(RBX::Primitive * &)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Soundscape21CollisionSoundManagerEPNS4_9PrimitiveEEENS0_5list2INS0_5valueIPS6_EENS_3argILi1EEEEEEclIS8_EEvRT_")]
// 0x7fd820 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Soundscape21CollisionSoundManagerEPNS4_9PrimitiveEEENS0_5list2INS0_5valueIPS6_EENS_3argILi1EEEEEEclIS8_EEvRT_
pub fn stub_0x7fd820() {
    // IDA 0x7fd820: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Primitive *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>,1,void ()(RBX::Primitive *)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_10Soundscape21CollisionSoundManagerES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEELi1ES6_ED1Ev")]
// 0x7fd838 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_10Soundscape21CollisionSoundManagerES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEELi1ES6_ED1Ev
pub fn stub_0x7fd838() {
    // IDA 0x7fd838: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Primitive *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>,1,void ()(RBX::Primitive *)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_10Soundscape21CollisionSoundManagerES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEELi1ES6_ED0Ev")]
// 0x7fd864 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_10Soundscape21CollisionSoundManagerES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEELi1ES6_ED0Ev
pub fn stub_0x7fd864() {
    // IDA 0x7fd864: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10Soundscape21CollisionSoundManagerES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEEED1Ev")]
// 0x7fdb68 — __ZN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10Soundscape21CollisionSoundManagerES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEEED1Ev
pub fn stub_0x7fdb68() {
    // IDA 0x7fdb68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10Soundscape21CollisionSoundManagerES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEEED0Ev")]
// 0x7fdb94 — __ZN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10Soundscape21CollisionSoundManagerES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEEED0Ev
pub fn stub_0x7fdb94() {
    // IDA 0x7fdb94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>,1,void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::call(std::pair<RBX::Primitive *,RBX::Primitive *>)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSt4pairIPN3RBX9PrimitiveES6_EEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS4_10Soundscape21CollisionSoundManagerES7_EENSC_5list2INSC_5valueIPSH_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_")]
// 0x7fdd84 — __ZN3rbx8callableINS_7signals6signalIFvSt4pairIPN3RBX9PrimitiveES6_EEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS4_10Soundscape21CollisionSoundManagerES7_EENSC_5list2INSC_5valueIPSH_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
pub fn stub_0x7fdd84() {
    // IDA 0x7fdd84: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>,1,void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::call(std::pair<RBX::Primitive *,RBX::Primitive *>)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvSt4pairIPN3RBX9PrimitiveES6_EEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS4_10Soundscape21CollisionSoundManagerES7_EENSC_5list2INSC_5valueIPSH_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_")]
// 0x7fdda8 — __ZThn4_N3rbx8callableINS_7signals6signalIFvSt4pairIPN3RBX9PrimitiveES6_EEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS4_10Soundscape21CollisionSoundManagerES7_EENSC_5list2INSC_5valueIPSH_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
pub fn stub_0x7fdda8() {
    // IDA 0x7fdda8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>>,boost::_bi::list1<std::pair<RBX::Primitive *,RBX::Primitive *>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>> &,boost::_bi::list1<std::pair<RBX::Primitive *,RBX::Primitive *>&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueIPN3RBX10Soundscape21CollisionSoundManagerEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS5_St4pairIPNS3_9PrimitiveESG_EEENS0_5list1IRSH_EEEEvNS0_4typeIvEERT_RT0_i")]
// 0x7fddcc — __ZN5boost3_bi5list2INS0_5valueIPN3RBX10Soundscape21CollisionSoundManagerEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS5_St4pairIPNS3_9PrimitiveESG_EEENS0_5list1IRSH_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_0x7fddcc() {
    // IDA 0x7fddcc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>,1,void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSt4pairIPN3RBX9PrimitiveES6_EEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS4_10Soundscape21CollisionSoundManagerES7_EENSC_5list2INSC_5valueIPSH_EENSB_3argILi1EEEEEEELi1ES8_ED1Ev")]
// 0x7fe0e4 — __ZN3rbx8callableINS_7signals6signalIFvSt4pairIPN3RBX9PrimitiveES6_EEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS4_10Soundscape21CollisionSoundManagerES7_EENSC_5list2INSC_5valueIPSH_EENSB_3argILi1EEEEEEELi1ES8_ED1Ev
pub fn stub_0x7fe0e4() {
    // IDA 0x7fe0e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>,1,void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSt4pairIPN3RBX9PrimitiveES6_EEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS4_10Soundscape21CollisionSoundManagerES7_EENSC_5list2INSC_5valueIPSH_EENSB_3argILi1EEEEEEELi1ES8_ED0Ev")]
// 0x7fe110 — __ZN3rbx8callableINS_7signals6signalIFvSt4pairIPN3RBX9PrimitiveES6_EEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS4_10Soundscape21CollisionSoundManagerES7_EENSC_5list2INSC_5valueIPSH_EENSB_3argILi1EEEEEEELi1ES8_ED0Ev
pub fn stub_0x7fe110() {
    // IDA 0x7fe110: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Soundscape::CollisionSoundType,std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>,std::_Select1st<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>,std::less<RBX::Soundscape::CollisionSoundType>,std::allocator<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX10Soundscape18CollisionSoundTypeESt4pairIKS2_N5boost10shared_ptrINS1_14CollisionSoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E")]
// 0x7fe1e4 — __ZNSt8_Rb_treeIN3RBX10Soundscape18CollisionSoundTypeESt4pairIKS2_N5boost10shared_ptrINS1_14CollisionSoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E
pub fn stub_0x7fe1e4() {
    // IDA 0x7fe1e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Soundscape::CollisionSoundType,std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>,std::_Select1st<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>,std::less<RBX::Soundscape::CollisionSoundType>,std::allocator<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX10Soundscape18CollisionSoundTypeESt4pairIKS2_N5boost10shared_ptrINS1_14CollisionSoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS9_E")]
// 0x7fe20c — __ZNSt8_Rb_treeIN3RBX10Soundscape18CollisionSoundTypeESt4pairIKS2_N5boost10shared_ptrINS1_14CollisionSoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS9_E
pub fn stub_0x7fe20c() {
    // IDA 0x7fe20c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "luaA_pushobject(lua_State *,lua_TValue const*)")]
#[doc(alias = "__Z15luaA_pushobjectP9lua_StatePK10lua_TValue")]
// 0x822960 — __Z15luaA_pushobjectP9lua_StatePK10lua_TValue
// type: int __fastcall(int result, int *)
pub fn stub_0x822960() {
    // IDA 0x822960: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "lua_checkstack(lua_State *,int)")]
#[doc(alias = "__Z14lua_checkstackP9lua_Statei")]
// 0x82297c — __Z14lua_checkstackP9lua_Statei
// type: int(void)
pub fn stub_0x82297c() {
    // IDA 0x82297c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "lua_xmove(lua_State *,lua_State *,int)")]
#[doc(alias = "__Z9lua_xmoveP9lua_StateS0_i")]
// 0x8229d8 — __Z9lua_xmoveP9lua_StateS0_i
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0x8229d8() {
    // IDA 0x8229d8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "lua_setlevel(lua_State *,lua_State *)")]
#[doc(alias = "__Z12lua_setlevelP9lua_StateS0_")]
// 0x822a2c — __Z12lua_setlevelP9lua_StateS0_
pub fn stub_0x822a2c() {
    // IDA 0x822a2c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_atpanic(lua_State *,int (*)(lua_State *))")]
#[doc(alias = "__Z11lua_atpanicP9lua_StatePFiS0_E")]
// 0x822a34 — __Z11lua_atpanicP9lua_StatePFiS0_E
pub fn stub_0x822a34() {
    // IDA 0x822a34: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_newthread(lua_State *)")]
#[doc(alias = "__Z13lua_newthreadP9lua_State")]
// 0x822a3c — __Z13lua_newthreadP9lua_State
// type: int __fastcall(_DWORD)
pub fn stub_0x822a3c() {
    // IDA 0x822a3c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_gettop(lua_State *)")]
#[doc(alias = "__Z10lua_gettopP9lua_State")]
// 0x822a80 — __Z10lua_gettopP9lua_State
// type: int __fastcall(_DWORD)
pub fn stub_0x822a80() {
    // IDA 0x822a80: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_settop(lua_State *,int)")]
#[doc(alias = "__Z10lua_settopP9lua_Statei")]
// 0x822a94 — __Z10lua_settopP9lua_Statei
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x822a94() {
    // IDA 0x822a94: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_remove(lua_State *,int)")]
#[doc(alias = "__Z10lua_removeP9lua_Statei")]
// 0x822ac8 — __Z10lua_removeP9lua_Statei
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x822ac8() {
    // IDA 0x822ac8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "index2adr(lua_State *,int)")]
#[doc(alias = "__ZL9index2adrP9lua_Statei")]
// 0x822af8 — __ZL9index2adrP9lua_Statei
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x822af8() {
    // IDA 0x822af8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_insert(lua_State *,int)")]
#[doc(alias = "__Z10lua_insertP9lua_Statei")]
// 0x822ba0 — __Z10lua_insertP9lua_Statei
pub fn stub_0x822ba0() {
    // IDA 0x822ba0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_replace(lua_State *,int)")]
#[doc(alias = "__Z11lua_replaceP9lua_Statei")]
// 0x822bdc — __Z11lua_replaceP9lua_Statei
pub fn stub_0x822bdc() {
    // IDA 0x822bdc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_pushvalue(lua_State *,int)")]
#[doc(alias = "__Z13lua_pushvalueP9lua_Statei")]
// 0x822c98 — __Z13lua_pushvalueP9lua_Statei
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x822c98() {
    // IDA 0x822c98: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_type(lua_State *,int)")]
#[doc(alias = "__Z8lua_typeP9lua_Statei")]
// 0x822cb8 — __Z8lua_typeP9lua_Statei
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x822cb8() {
    // IDA 0x822cb8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_typename(lua_State *,int)")]
#[doc(alias = "__Z12lua_typenameP9lua_Statei")]
// 0x822cdc — __Z12lua_typenameP9lua_Statei
// type: const char *__fastcall(int, int)
pub fn stub_0x822cdc() {
    // IDA 0x822cdc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_iscfunction(lua_State *,int)")]
#[doc(alias = "__Z15lua_iscfunctionP9lua_Statei")]
// 0x822d00 — __Z15lua_iscfunctionP9lua_Statei
pub fn stub_0x822d00() {
    // IDA 0x822d00: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_isnumber(lua_State *,int)")]
#[doc(alias = "__Z12lua_isnumberP9lua_Statei")]
// 0x822d20 — __Z12lua_isnumberP9lua_Statei
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x822d20() {
    // IDA 0x822d20: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_isstring(lua_State *,int)")]
#[doc(alias = "__Z12lua_isstringP9lua_Statei")]
// 0x822d48 — __Z12lua_isstringP9lua_Statei
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x822d48() {
    // IDA 0x822d48: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_rawequal(lua_State *,int,int)")]
#[doc(alias = "__Z12lua_rawequalP9lua_Stateii")]
// 0x822d74 — __Z12lua_rawequalP9lua_Stateii
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0x822d74() {
    // IDA 0x822d74: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_lessthan(lua_State *,int,int)")]
#[doc(alias = "__Z12lua_lessthanP9lua_Stateii")]
// 0x822db4 — __Z12lua_lessthanP9lua_Stateii
pub fn stub_0x822db4() {
    // IDA 0x822db4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_tonumber(lua_State *,int)")]
#[doc(alias = "__Z12lua_tonumberP9lua_Statei")]
// 0x822df0 — __Z12lua_tonumberP9lua_Statei
// type: __int64 __fastcall(_DWORD, _DWORD)
pub fn stub_0x822df0() {
    // IDA 0x822df0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_tointeger(lua_State *,int)")]
#[doc(alias = "__Z13lua_tointegerP9lua_Statei")]
// 0x822e28 — __Z13lua_tointegerP9lua_Statei
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x822e28() {
    // IDA 0x822e28: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_toboolean(lua_State *,int)")]
#[doc(alias = "__Z13lua_tobooleanP9lua_Statei")]
// 0x822e54 — __Z13lua_tobooleanP9lua_Statei
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x822e54() {
    // IDA 0x822e54: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_tolstring(lua_State *,int,unsigned long *)")]
#[doc(alias = "__Z13lua_tolstringP9lua_StateiPm")]
// 0x822e78 — __Z13lua_tolstringP9lua_StateiPm
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0x822e78() {
    // IDA 0x822e78: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_objlen(lua_State *,int)")]
#[doc(alias = "__Z10lua_objlenP9lua_Statei")]
// 0x822ed0 — __Z10lua_objlenP9lua_Statei
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x822ed0() {
    // IDA 0x822ed0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_touserdata(lua_State *,int)")]
#[doc(alias = "__Z14lua_touserdataP9lua_Statei")]
// 0x822f1c — __Z14lua_touserdataP9lua_Statei
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x822f1c() {
    // IDA 0x822f1c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_tothread(lua_State *,int)")]
#[doc(alias = "__Z12lua_tothreadP9lua_Statei")]
// 0x822f40 — __Z12lua_tothreadP9lua_Statei
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x822f40() {
    // IDA 0x822f40: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_topointer(lua_State *,int)")]
#[doc(alias = "__Z13lua_topointerP9lua_Statei")]
// 0x822f58 — __Z13lua_topointerP9lua_Statei
pub fn stub_0x822f58() {
    // IDA 0x822f58: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_pushnil(lua_State *)")]
#[doc(alias = "__Z11lua_pushnilP9lua_State")]
// 0x822fa0 — __Z11lua_pushnilP9lua_State
// type: int __fastcall(_DWORD)
pub fn stub_0x822fa0() {
    // IDA 0x822fa0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_pushnumber(lua_State *,double)")]
#[doc(alias = "__Z14lua_pushnumberP9lua_Stated")]
// 0x822fac — __Z14lua_pushnumberP9lua_Stated
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0x822fac() {
    // IDA 0x822fac: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_pushinteger(lua_State *,int)")]
#[doc(alias = "__Z15lua_pushintegerP9lua_Statei")]
// 0x822fc0 — __Z15lua_pushintegerP9lua_Statei
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x822fc0() {
    // IDA 0x822fc0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_pushlstring(lua_State *,char const*,unsigned long)")]
#[doc(alias = "__Z15lua_pushlstringP9lua_StatePKcm")]
// 0x822fd8 — __Z15lua_pushlstringP9lua_StatePKcm
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0x822fd8() {
    // IDA 0x822fd8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_pushstring(lua_State *,char const*)")]
#[doc(alias = "__Z14lua_pushstringP9lua_StatePKc")]
// 0x823014 — __Z14lua_pushstringP9lua_StatePKc
// type: int __fastcall(int, char *__s)
pub fn stub_0x823014() {
    // IDA 0x823014: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_pushvfstring(lua_State *,char const*,void *)")]
#[doc(alias = "__Z16lua_pushvfstringP9lua_StatePKcPv")]
// 0x823040 — __Z16lua_pushvfstringP9lua_StatePKcPv
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0x823040() {
    // IDA 0x823040: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_pushfstring(lua_State *,char const*,...)")]
#[doc(alias = "__Z15lua_pushfstringP9lua_StatePKcz")]
// 0x823068 — __Z15lua_pushfstringP9lua_StatePKcz
// type: int __fastcall(int, int)
pub fn stub_0x823068() {
    // IDA 0x823068: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_pushcclosure(lua_State *,int (*)(lua_State *),int)")]
#[doc(alias = "__Z16lua_pushcclosureP9lua_StatePFiS0_Ei")]
// 0x8230a0 — __Z16lua_pushcclosureP9lua_StatePFiS0_Ei
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0x8230a0() {
    // IDA 0x8230a0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_pushboolean(lua_State *,int)")]
#[doc(alias = "__Z15lua_pushbooleanP9lua_Statei")]
// 0x823134 — __Z15lua_pushbooleanP9lua_Statei
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x823134() {
    // IDA 0x823134: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_pushlightuserdata(lua_State *,void *)")]
#[doc(alias = "__Z21lua_pushlightuserdataP9lua_StatePv")]
// 0x82314c — __Z21lua_pushlightuserdataP9lua_StatePv
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x82314c() {
    // IDA 0x82314c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_pushthread(lua_State *)")]
#[doc(alias = "__Z14lua_pushthreadP9lua_State")]
// 0x82315c — __Z14lua_pushthreadP9lua_State
pub fn stub_0x82315c() {
    // IDA 0x82315c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_gettable(lua_State *,int)")]
#[doc(alias = "__Z12lua_gettableP9lua_Statei")]
// 0x82317c — __Z12lua_gettableP9lua_Statei
pub fn stub_0x82317c() {
    // IDA 0x82317c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_getfield(lua_State *,int,char const*)")]
#[doc(alias = "__Z12lua_getfieldP9lua_StateiPKc")]
// 0x82319c — __Z12lua_getfieldP9lua_StateiPKc
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0x82319c() {
    // IDA 0x82319c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_rawget(lua_State *,int)")]
#[doc(alias = "__Z10lua_rawgetP9lua_Statei")]
// 0x8231d8 — __Z10lua_rawgetP9lua_Statei
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x8231d8() {
    // IDA 0x8231d8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_rawgeti(lua_State *,int,int)")]
#[doc(alias = "__Z11lua_rawgetiP9lua_Stateii")]
// 0x823204 — __Z11lua_rawgetiP9lua_Stateii
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x823204() {
    // IDA 0x823204: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_createtable(lua_State *,int,int)")]
#[doc(alias = "__Z15lua_createtableP9lua_Stateii")]
// 0x823230 — __Z15lua_createtableP9lua_Stateii
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0x823230() {
    // IDA 0x823230: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_setreadonly(lua_State *,int,bool)")]
#[doc(alias = "__Z15lua_setreadonlyP9lua_Stateib")]
// 0x82326c — __Z15lua_setreadonlyP9lua_Stateib
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0x82326c() {
    // IDA 0x82326c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_getmetatable(lua_State *,int)")]
#[doc(alias = "__Z16lua_getmetatableP9lua_Statei")]
// 0x82327c — __Z16lua_getmetatableP9lua_Statei
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x82327c() {
    // IDA 0x82327c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_getfenv(lua_State *,int)")]
#[doc(alias = "__Z11lua_getfenvP9lua_Statei")]
// 0x8232c0 — __Z11lua_getfenvP9lua_Statei
pub fn stub_0x8232c0() {
    // IDA 0x8232c0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_settable(lua_State *,int)")]
#[doc(alias = "__Z12lua_settableP9lua_Statei")]
// 0x823304 — __Z12lua_settableP9lua_Statei
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x823304() {
    // IDA 0x823304: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_setfield(lua_State *,int,char const*)")]
#[doc(alias = "__Z12lua_setfieldP9lua_StateiPKc")]
// 0x823328 — __Z12lua_setfieldP9lua_StateiPKc
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0x823328() {
    // IDA 0x823328: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_rawset(lua_State *,int)")]
#[doc(alias = "__Z10lua_rawsetP9lua_Statei")]
// 0x823368 — __Z10lua_rawsetP9lua_Statei
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x823368() {
    // IDA 0x823368: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_rawseti(lua_State *,int,int)")]
#[doc(alias = "__Z11lua_rawsetiP9lua_Stateii")]
// 0x8233e8 — __Z11lua_rawsetiP9lua_Stateii
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0x8233e8() {
    // IDA 0x8233e8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_setmetatable(lua_State *,int)")]
#[doc(alias = "__Z16lua_setmetatableP9lua_Statei")]
// 0x82344c — __Z16lua_setmetatableP9lua_Statei
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x82344c() {
    // IDA 0x82344c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_setfenv(lua_State *,int)")]
#[doc(alias = "__Z11lua_setfenvP9lua_Statei")]
// 0x8234c8 — __Z11lua_setfenvP9lua_Statei
pub fn stub_0x8234c8() {
    // IDA 0x8234c8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_call(lua_State *,int,int)")]
#[doc(alias = "__Z8lua_callP9lua_Stateii")]
// 0x823534 — __Z8lua_callP9lua_Stateii
pub fn stub_0x823534() {
    // IDA 0x823534: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_pcall(lua_State *,int,int,int)")]
#[doc(alias = "__Z9lua_pcallP9lua_Stateiii")]
// 0x823564 — __Z9lua_pcallP9lua_Stateiii
// type: int __fastcall(_DWORD *, int, int, int)
pub fn stub_0x823564() {
    // IDA 0x823564: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "f_call(lua_State *,void *)")]
#[doc(alias = "__ZL6f_callP9lua_StatePv")]
// 0x8235c0 — __ZL6f_callP9lua_StatePv
pub fn stub_0x8235c0() {
    // IDA 0x8235c0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_load(lua_State *,char const* (*)(lua_State *,void *,unsigned long *),void *,char const*)")]
#[doc(alias = "__Z8lua_loadP9lua_StatePFPKcS0_PvPmES3_S2_")]
// 0x8235d0 — __Z8lua_loadP9lua_StatePFPKcS0_PvPmES3_S2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0x8235d0() {
    // IDA 0x8235d0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_dump(lua_State *,int (*)(lua_State *,void const*,unsigned long,void *),void *)")]
#[doc(alias = "__Z8lua_dumpP9lua_StatePFiS0_PKvmPvES3_")]
// 0x823604 — __Z8lua_dumpP9lua_StatePFiS0_PKvmPvES3_
pub fn stub_0x823604() {
    // IDA 0x823604: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_status(lua_State *)")]
#[doc(alias = "__Z10lua_statusP9lua_State")]
// 0x823638 — __Z10lua_statusP9lua_State
pub fn stub_0x823638() {
    // IDA 0x823638: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_gc(lua_State *,int,int)")]
#[doc(alias = "__Z6lua_gcP9lua_Stateii")]
// 0x82363c — __Z6lua_gcP9lua_Stateii
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0x82363c() {
    // IDA 0x82363c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_error(lua_State *)")]
#[doc(alias = "__Z9lua_errorP9lua_State")]
// 0x8236b8 — __Z9lua_errorP9lua_State
// type: void __fastcall __noreturn(_DWORD)
pub fn stub_0x8236b8() {
    // IDA 0x8236b8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_next(lua_State *,int)")]
#[doc(alias = "__Z8lua_nextP9lua_Statei")]
// 0x8236c4 — __Z8lua_nextP9lua_Statei
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x8236c4() {
    // IDA 0x8236c4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_concat(lua_State *,int)")]
#[doc(alias = "__Z10lua_concatP9lua_Statei")]
// 0x8236f0 — __Z10lua_concatP9lua_Statei
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x8236f0() {
    // IDA 0x8236f0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_newuserdata(lua_State *,unsigned long)")]
#[doc(alias = "__Z15lua_newuserdataP9lua_Statem")]
// 0x823764 — __Z15lua_newuserdataP9lua_Statem
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x823764() {
    // IDA 0x823764: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_getupvalue(lua_State *,int,int)")]
#[doc(alias = "__Z14lua_getupvalueP9lua_Stateii")]
// 0x8237a8 — __Z14lua_getupvalueP9lua_Stateii
pub fn stub_0x8237a8() {
    // IDA 0x8237a8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "aux_upvalue(lua_TValue *,int,lua_TValue **)")]
#[doc(alias = "__ZL11aux_upvalueP10lua_TValueiPS0_")]
// 0x8237dc — __ZL11aux_upvalueP10lua_TValueiPS0_
pub fn stub_0x8237dc() {
    // IDA 0x8237dc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "lua_setupvalue(lua_State *,int,int)")]
#[doc(alias = "__Z14lua_setupvalueP9lua_Stateii")]
// 0x823848 — __Z14lua_setupvalueP9lua_Stateii
pub fn stub_0x823848() {
    // IDA 0x823848: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "luaL_argerror(lua_State *,int,char const*)")]
#[doc(alias = "__Z13luaL_argerrorP9lua_StateiPKc")]
// 0x823fec — __Z13luaL_argerrorP9lua_StateiPKc
// type: void __fastcall __noreturn(int, const char *)
pub fn stub_0x823fec() {
    // IDA 0x823fec: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "luaL_error(lua_State *,char const*,...)")]
#[doc(alias = "__Z10luaL_errorP9lua_StatePKcz")]
// 0x8240a8 — __Z10luaL_errorP9lua_StatePKcz
// type: void __fastcall __noreturn(int, int)
pub fn stub_0x8240a8() {
    // IDA 0x8240a8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "luaL_typerror(lua_State *,int,char const*)")]
#[doc(alias = "__Z13luaL_typerrorP9lua_StateiPKc")]
// 0x8240e8 — __Z13luaL_typerrorP9lua_StateiPKc
// type: void __fastcall __noreturn(int, const char *)
pub fn stub_0x8240e8() {
    // IDA 0x8240e8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "luaL_where(lua_State *,int)")]
#[doc(alias = "__Z10luaL_whereP9lua_Statei")]
// 0x824120 — __Z10luaL_whereP9lua_Statei
// type: int __fastcall(int, int)
pub fn stub_0x824120() {
    // IDA 0x824120: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "luaL_checkoption(lua_State *,int,char const*,char const* const*)")]
#[doc(alias = "__Z16luaL_checkoptionP9lua_StateiPKcPKS2_")]
// 0x824194 — __Z16luaL_checkoptionP9lua_StateiPKcPKS2_
// type: int __fastcall(int, const char *, int, int)
pub fn stub_0x824194() {
    // IDA 0x824194: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "luaL_optlstring(lua_State *,int,char const*,unsigned long *)")]
#[doc(alias = "__Z15luaL_optlstringP9lua_StateiPKcPm")]
// 0x8241f4 — __Z15luaL_optlstringP9lua_StateiPKcPm
// type: int __fastcall(int, int, const char *, size_t *)
pub fn stub_0x8241f4() {
    // IDA 0x8241f4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "luaL_checklstring(lua_State *,int,unsigned long *)")]
#[doc(alias = "__Z17luaL_checklstringP9lua_StateiPm")]
// 0x82423c — __Z17luaL_checklstringP9lua_StateiPm
// type: int __fastcall(int, const char *, int)
pub fn stub_0x82423c() {
    // IDA 0x82423c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "luaL_newmetatable(lua_State *,char const*)")]
#[doc(alias = "__Z17luaL_newmetatableP9lua_StatePKc")]
// 0x824264 — __Z17luaL_newmetatableP9lua_StatePKc
// type: int __fastcall(int, int)
pub fn stub_0x824264() {
    // IDA 0x824264: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "luaL_checkudata(lua_State *,int,char const*)")]
#[doc(alias = "__Z15luaL_checkudataP9lua_StateiPKc")]
// 0x8242c0 — __Z15luaL_checkudataP9lua_StateiPKc
// type: int __fastcall(int, const char *, int)
pub fn stub_0x8242c0() {
    // IDA 0x8242c0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "luaL_checkstack(lua_State *,int,char const*)")]
#[doc(alias = "__Z15luaL_checkstackP9lua_StateiPKc")]
// 0x824320 — __Z15luaL_checkstackP9lua_StateiPKc
// type: int __fastcall(int)
pub fn stub_0x824320() {
    // IDA 0x824320: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "luaL_checktype(lua_State *,int,int)")]
#[doc(alias = "__Z14luaL_checktypeP9lua_Stateii")]
// 0x824348 — __Z14luaL_checktypeP9lua_Stateii
// type: int __fastcall(int, const char *, int)
pub fn stub_0x824348() {
    // IDA 0x824348: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "luaL_checkany(lua_State *,int)")]
#[doc(alias = "__Z13luaL_checkanyP9lua_Statei")]
// 0x824374 — __Z13luaL_checkanyP9lua_Statei
pub fn stub_0x824374() {
    // IDA 0x824374: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "luaL_checknumber(lua_State *,int)")]
#[doc(alias = "__Z16luaL_checknumberP9lua_Statei")]
// 0x8243a0 — __Z16luaL_checknumberP9lua_Statei
// type: double __fastcall(_DWORD, _DWORD)
pub fn stub_0x8243a0() {
    // IDA 0x8243a0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "luaL_checkinteger(lua_State *,int)")]
#[doc(alias = "__Z17luaL_checkintegerP9lua_Statei")]
// 0x8243e4 — __Z17luaL_checkintegerP9lua_Statei
pub fn stub_0x8243e4() {
    // IDA 0x8243e4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "luaL_optinteger(lua_State *,int,int)")]
#[doc(alias = "__Z15luaL_optintegerP9lua_Stateii")]
// 0x824414 — __Z15luaL_optintegerP9lua_Stateii
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0x824414() {
    // IDA 0x824414: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "luaL_getmetafield(lua_State *,int,char const*)")]
#[doc(alias = "__Z17luaL_getmetafieldP9lua_StateiPKc")]
// 0x824438 — __Z17luaL_getmetafieldP9lua_StateiPKc
pub fn stub_0x824438() {
    // IDA 0x824438: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "luaL_callmeta(lua_State *,int,char const*)")]
#[doc(alias = "__Z13luaL_callmetaP9lua_StateiPKc")]
// 0x824488 — __Z13luaL_callmetaP9lua_StateiPKc
pub fn stub_0x824488() {
    // IDA 0x824488: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "luaL_register(lua_State *,char const*,luaL_Reg const*)")]
#[doc(alias = "__Z13luaL_registerP9lua_StatePKcPK8luaL_Reg")]
// 0x8244cc — __Z13luaL_registerP9lua_StatePKcPK8luaL_Reg
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0x8244cc() {
    // IDA 0x8244cc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "luaI_openlib(lua_State *,char const*,luaL_Reg const*,int)")]
#[doc(alias = "__Z12luaI_openlibP9lua_StatePKcPK8luaL_Regi")]
// 0x8244d4 — __Z12luaI_openlibP9lua_StatePKcPK8luaL_Regi
pub fn stub_0x8244d4() {
    // IDA 0x8244d4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "luaL_findtable(lua_State *,int,char const*,int)")]
#[doc(alias = "__Z14luaL_findtableP9lua_StateiPKci")]
// 0x8245e8 — __Z14luaL_findtableP9lua_StateiPKci
pub fn stub_0x8245e8() {
    // IDA 0x8245e8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "luaL_buffinit(lua_State *,luaL_Buffer *)")]
#[doc(alias = "__Z13luaL_buffinitP9lua_StateP11luaL_Buffer")]
// 0x8246a8 — __Z13luaL_buffinitP9lua_StateP11luaL_Buffer
pub fn stub_0x8246a8() {
    // IDA 0x8246a8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "luaL_addlstring(luaL_Buffer *,char const*,unsigned long)")]
#[doc(alias = "__Z15luaL_addlstringP11luaL_BufferPKcm")]
// 0x8246b8 — __Z15luaL_addlstringP11luaL_BufferPKcm
pub fn stub_0x8246b8() {
    // IDA 0x8246b8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "luaL_pushresult(luaL_Buffer *)")]
#[doc(alias = "__Z15luaL_pushresultP11luaL_Buffer")]
// 0x8246f0 — __Z15luaL_pushresultP11luaL_Buffer
pub fn stub_0x8246f0() {
    // IDA 0x8246f0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "luaL_prepbuffer(luaL_Buffer *)")]
#[doc(alias = "__Z15luaL_prepbufferP11luaL_Buffer")]
// 0x82470c — __Z15luaL_prepbufferP11luaL_Buffer
// type: int __fastcall(int)
pub fn stub_0x82470c() {
    // IDA 0x82470c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "emptybuffer(luaL_Buffer *)")]
#[doc(alias = "__ZL11emptybufferP11luaL_Buffer")]
// 0x824728 — __ZL11emptybufferP11luaL_Buffer
// type: int(void)
pub fn stub_0x824728() {
    // IDA 0x824728: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "adjuststack(luaL_Buffer *)")]
#[doc(alias = "__ZL11adjuststackP11luaL_Buffer")]
// 0x824754 — __ZL11adjuststackP11luaL_Buffer
// type: int __fastcall(_DWORD)
pub fn stub_0x824754() {
    // IDA 0x824754: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "luaL_addvalue(luaL_Buffer *)")]
#[doc(alias = "__Z13luaL_addvalueP11luaL_Buffer")]
// 0x8247b8 — __Z13luaL_addvalueP11luaL_Buffer
pub fn stub_0x8247b8() {
    // IDA 0x8247b8: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "luaL_ref(lua_State *,int)")]
#[doc(alias = "__Z8luaL_refP9lua_Statei")]
// 0x824818 — __Z8luaL_refP9lua_Statei
pub fn stub_0x824818() {
    // IDA 0x824818: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "luaL_unref(lua_State *,int,int)")]
#[doc(alias = "__Z10luaL_unrefP9lua_Stateii")]
// 0x8248a0 — __Z10luaL_unrefP9lua_Stateii
pub fn stub_0x8248a0() {
    // IDA 0x8248a0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "luaL_loadfile(lua_State *,char const*)")]
#[doc(alias = "__Z13luaL_loadfileP9lua_StatePKc")]
// 0x8248f0 — __Z13luaL_loadfileP9lua_StatePKc
pub fn stub_0x8248f0() {
    // IDA 0x8248f0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "errfile(lua_State *,char const*,int)")]
#[doc(alias = "__ZL7errfileP9lua_StatePKci")]
// 0x824a68 — __ZL7errfileP9lua_StatePKci
pub fn stub_0x824a68() {
    // IDA 0x824a68: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "getF(lua_State *,void *,unsigned long *)")]
#[doc(alias = "__ZL4getFP9lua_StatePvPm")]
// 0x824ab8 — __ZL4getFP9lua_StatePvPm
pub fn stub_0x824ab8() {
    // IDA 0x824ab8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "getS(lua_State *,void *,unsigned long *)")]
#[doc(alias = "__ZL4getSP9lua_StatePvPm")]
// 0x824b04 — __ZL4getSP9lua_StatePvPm
pub fn stub_0x824b04() {
    // IDA 0x824b04: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "protReader(lua_State *,void *,unsigned long *)")]
#[doc(alias = "__ZL10protReaderP9lua_StatePvPm")]
// 0x824dac — __ZL10protReaderP9lua_StatePvPm
pub fn stub_0x824dac() {
    // IDA 0x824dac: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "luaL_newstate(void)")]
#[doc(alias = "__Z13luaL_newstatev")]
// 0x824fa8 — __Z13luaL_newstatev
// type: _DWORD __fastcall()
pub fn stub_0x824fa8() {
    // IDA 0x824fa8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL5panicP9lua_State_0")]
// 0x824ff0 — __ZL5panicP9lua_State_0
pub fn stub_0x824ff0() {
    // IDA 0x824ff0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "luaopen_base(lua_State *)")]
#[doc(alias = "__Z12luaopen_baseP9lua_State")]
// 0x8251c0 — __Z12luaopen_baseP9lua_State
pub fn stub_0x8251c0() {
    // IDA 0x8251c0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "luaB_cocreate(lua_State *)")]
#[doc(alias = "__ZL13luaB_cocreateP9lua_State")]
// 0x8252ec — __ZL13luaB_cocreateP9lua_State
pub fn stub_0x8252ec() {
    // IDA 0x8252ec: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "luaB_coresume(lua_State *)")]
#[doc(alias = "__ZL13luaB_coresumeP9lua_State")]
// 0x825338 — __ZL13luaB_coresumeP9lua_State
pub fn stub_0x825338() {
    // IDA 0x825338: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "luaB_corunning(lua_State *)")]
#[doc(alias = "__ZL14luaB_corunningP9lua_State")]
// 0x82539c — __ZL14luaB_corunningP9lua_State
pub fn stub_0x82539c() {
    // IDA 0x82539c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "luaB_costatus(lua_State *)")]
#[doc(alias = "__ZL13luaB_costatusP9lua_State")]
// 0x8253b4 — __ZL13luaB_costatusP9lua_State
// type: int __fastcall(int)
pub fn stub_0x8253b4() {
    // IDA 0x8253b4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "luaB_cowrap(lua_State *)")]
#[doc(alias = "__ZL11luaB_cowrapP9lua_State")]
// 0x8253f8 — __ZL11luaB_cowrapP9lua_State
pub fn stub_0x8253f8() {
    // IDA 0x8253f8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "luaB_yield(lua_State *)")]
#[doc(alias = "__ZL10luaB_yieldP9lua_State")]
// 0x825418 — __ZL10luaB_yieldP9lua_State
pub fn stub_0x825418() {
    // IDA 0x825418: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "luaB_auxwrap(lua_State *)")]
#[doc(alias = "__ZL12luaB_auxwrapP9lua_State")]
// 0x825430 — __ZL12luaB_auxwrapP9lua_State
pub fn stub_0x825430() {
    // IDA 0x825430: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "auxresume(lua_State *,lua_State *,int)")]
#[doc(alias = "__ZL9auxresumeP9lua_StateS0_i")]
// 0x82548c — __ZL9auxresumeP9lua_StateS0_i
pub fn stub_0x82548c() {
    // IDA 0x82548c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "costatus(lua_State *,lua_State *)")]
#[doc(alias = "__ZL8costatusP9lua_StateS0_")]
// 0x825540 — __ZL8costatusP9lua_StateS0_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x825540() {
    // IDA 0x825540: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "auxopen(lua_State *,char const*,int (*)(lua_State *),int (*)(lua_State *))")]
#[doc(alias = "__ZL7auxopenP9lua_StatePKcPFiS0_ES4_")]
// 0x8255a8 — __ZL7auxopenP9lua_StatePKcPFiS0_ES4_
pub fn stub_0x8255a8() {
    // IDA 0x8255a8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "luaB_ipairs(lua_State *)")]
#[doc(alias = "__ZL11luaB_ipairsP9lua_State")]
// 0x8255d4 — __ZL11luaB_ipairsP9lua_State
pub fn stub_0x8255d4() {
    // IDA 0x8255d4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "ipairsaux(lua_State *)")]
#[doc(alias = "__ZL9ipairsauxP9lua_State")]
// 0x825604 — __ZL9ipairsauxP9lua_State
pub fn stub_0x825604() {
    // IDA 0x825604: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}
