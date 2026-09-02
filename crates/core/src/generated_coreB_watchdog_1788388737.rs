//! coreB watchdog — 120 core/utility stubs EA-sorted, core/utility namespace fallback gap filler.
//! Source: ida/export.json (85545 funcs) filtered for core/utility namespace (demangled contains core|utility case-insensitive), SKIP EAs in global set (/tmp/global_eas.txt 68540 unique), EA-sorted asc next 120 uncovered.
//! Core-filtered 79 remaining, gap-filled to 120 with EA-sorted uncovered. Range: 0x8093a0..0xf51a24 | rbx_core::SharedPtr not boost.
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>,void>::invoke(boost::detail::function::function_buffer &)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS3_5list3INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEENSB_IdEEEEEEvE6invokeERNS1_15function_bufferE")]
// 0x8093a0 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS3_5list3INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEENSB_IdEEEEEEvE6invokeERNS1_15function_bufferE
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>,void>::invoke(boost::detail::function::function_buffer &)
pub fn stub_0x8093a0() -> ! {
    todo!("0x8093a0 __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS3_5list3INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEENSB_IdEEEEEEvE6invokeERNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENSD_IiEENSD_IdEEEEEEEEbT_RNS1_15function_bufferE")]
// 0x8093b8 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENSD_IiEENSD_IdEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int)
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>,boost::detail::function::function_buffer &)const
pub fn stub_0x8093b8() -> ! {
    todo!("0x8093b8 __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENSD_IiEENSD_IdEEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENSD_IiEENSD_IdEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// 0x8094ac — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENSD_IiEENSD_IdEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int)
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0x8094ac() -> ! {
    todo!("0x8094ac __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENSD_IiEENSD_IdEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENSD_IiEENSD_IdEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
// 0x80959c — __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENSD_IiEENSD_IdEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int, int, _DWORD *)
// was: void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_0x80959c() -> ! {
    todo!("0x80959c __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENSD_IiEENSD_IdEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>::operator()<boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::TestService,int,double> &,boost::_bi::list0 &,int)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS2_IdEEEclINS_4_mfi3mf2IvS5_idEENS0_5list0EEEvNS0_4typeIvEERT_RT0_i")]
// 0x809680 — __ZN5boost3_bi5list3INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS2_IdEEEclINS_4_mfi3mf2IvS5_idEENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>::operator()<boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::TestService,int,double> &,boost::_bi::list0 &,int)
pub fn stub_0x809680() -> ! {
    todo!("0x809680 __ZN5boost3_bi5list3INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS2_IdEEEclINS_4_mfi3mf2IvS5_idEENS0_5list0EEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS3_5list3INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEENSB_IdEEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// 0x8096a4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS3_5list3INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEENSB_IdEEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int *, _WORD *, int, int, int, void *, int, int, int, int)
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_0x8096a4() -> ! {
    todo!("0x8096a4 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS3_5list3INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEENSB_IdEEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>::list3(boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS2_IdEEEC2ES7_S8_S9_")]
// 0x80980c — __ZN5boost3_bi5list3INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS2_IdEEEC2ES7_S8_S9_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>::list3(boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>)
pub fn stub_0x80980c() -> ! {
    todo!("0x80980c __ZN5boost3_bi5list3INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS2_IdEEEC2ES7_S8_S9_")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>::storage3(boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>)")]
#[doc(alias = "__ZN5boost3_bi8storage3INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS2_IdEEEC2ES7_S8_S9_")]
// 0x8098ec — __ZN5boost3_bi8storage3INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS2_IdEEEC2ES7_S8_S9_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::_bi::storage3<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>::storage3(boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>)
pub fn stub_0x8098ec() -> ! {
    todo!("0x8098ec __ZN5boost3_bi8storage3INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS2_IdEEEC2ES7_S8_S9_")
}

#[doc(alias = "std::vector<RBX::FunctionalTest::Result,std::allocator<RBX::FunctionalTest::Result>>::resize(unsigned long,RBX::FunctionalTest::Result)")]
#[doc(alias = "__ZNSt6vectorIN3RBX14FunctionalTest6ResultESaIS2_EE6resizeEmS2_")]
// 0x8121c8 — __ZNSt6vectorIN3RBX14FunctionalTest6ResultESaIS2_EE6resizeEmS2_
pub fn stub_0x8121c8() -> ! {
    todo!("0x8121c8 __ZNSt6vectorIN3RBX14FunctionalTest6ResultESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::FunctionalTest::Result,std::allocator<RBX::FunctionalTest::Result>>::push_back(RBX::FunctionalTest::Result const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX14FunctionalTest6ResultESaIS2_EE9push_backERKS2_")]
// 0x8121fc — __ZNSt6vectorIN3RBX14FunctionalTest6ResultESaIS2_EE9push_backERKS2_
pub fn stub_0x8121fc() -> ! {
    todo!("0x8121fc __ZNSt6vectorIN3RBX14FunctionalTest6ResultESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::FunctionalTest::Result,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_14FunctionalTest6ResultESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
// 0x812224 — __ZNSt3mapIPKN3RBX4NameENS0_14FunctionalTest6ResultESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_0x812224() -> ! {
    todo!("0x812224 __ZNSt3mapIPKN3RBX4NameENS0_14FunctionalTest6ResultESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>,std::pair<RBX::Name const* const,RBX::FunctionalTest::Result> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_14FunctionalTest6ResultEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
// 0x81227c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_14FunctionalTest6ResultEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0x81227c() -> ! {
    todo!("0x81227c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_14FunctionalTest6ResultEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::FunctionalTest::Result> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_14FunctionalTest6ResultEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
// 0x812330 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_14FunctionalTest6ResultEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_0x812330() -> ! {
    todo!("0x812330 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_14FunctionalTest6ResultEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::FunctionalTest::Result> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_14FunctionalTest6ResultEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
// 0x812388 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_14FunctionalTest6ResultEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_0x812388() -> ! {
    todo!("0x812388 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_14FunctionalTest6ResultEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::FunctionalTest::Result,std::allocator<RBX::FunctionalTest::Result>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FunctionalTest::Result*,std::vector<RBX::FunctionalTest::Result,std::allocator<RBX::FunctionalTest::Result>>>,RBX::FunctionalTest::Result const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX14FunctionalTest6ResultESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0x8123f0 — __ZNSt6vectorIN3RBX14FunctionalTest6ResultESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0x8123f0() -> ! {
    todo!("0x8123f0 __ZNSt6vectorIN3RBX14FunctionalTest6ResultESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::FunctionalTest::Result,std::allocator<RBX::FunctionalTest::Result>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX14FunctionalTest6ResultESaIS2_EE11_M_allocateEm")]
// 0x8124d4 — __ZNSt12_Vector_baseIN3RBX14FunctionalTest6ResultESaIS2_EE11_M_allocateEm
pub fn stub_0x8124d4() -> ! {
    todo!("0x8124d4 __ZNSt12_Vector_baseIN3RBX14FunctionalTest6ResultESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::FunctionalTest::Result * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::FunctionalTest::Result *,RBX::FunctionalTest::Result *>(RBX::FunctionalTest::Result *,RBX::FunctionalTest::Result *,RBX::FunctionalTest::Result *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX14FunctionalTest6ResultES6_EET0_T_S8_S7_")]
// 0x8124ec — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX14FunctionalTest6ResultES6_EET0_T_S8_S7_
pub fn stub_0x8124ec() -> ! {
    todo!("0x8124ec __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX14FunctionalTest6ResultES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::FunctionalTest::Result,std::allocator<RBX::FunctionalTest::Result>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::FunctionalTest::Result*,std::vector<RBX::FunctionalTest::Result,std::allocator<RBX::FunctionalTest::Result>>>,unsigned long,RBX::FunctionalTest::Result const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX14FunctionalTest6ResultESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// 0x812528 — __ZNSt6vectorIN3RBX14FunctionalTest6ResultESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_0x812528() -> ! {
    todo!("0x812528 __ZNSt6vectorIN3RBX14FunctionalTest6ResultESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "RBX::MacroSubstituter::MacroSubstituter(std::string const&)")]
#[doc(alias = "__ZN3RBX16MacroSubstituterC2ERKSs")]
// 0x8126b8 — __ZN3RBX16MacroSubstituterC2ERKSs
// type: _DWORD __fastcall(RBX::MacroSubstituter *__hidden this, const std::string *)
pub fn stub_0x8126b8() -> ! {
    todo!("0x8126b8 __ZN3RBX16MacroSubstituterC2ERKSs")
}

#[doc(alias = "RBX::MacroSubstituter::processLine(int,std::string const&)")]
#[doc(alias = "__ZN3RBX16MacroSubstituter11processLineEiRKSs")]
// 0x812a08 — __ZN3RBX16MacroSubstituter11processLineEiRKSs
// type: _DWORD __fastcall(RBX::MacroSubstituter *__hidden this, int, const std::string *)
pub fn stub_0x812a08() -> ! {
    todo!("0x812a08 __ZN3RBX16MacroSubstituter11processLineEiRKSs")
}

#[doc(alias = "RBX::MacroSubstituter::doRBX_Test_Equality(int,std::string const&,char const*,char const*,char const*,char const*)")]
#[doc(alias = "__ZN3RBX16MacroSubstituter19doRBX_Test_EqualityEiRKSsPKcS4_S4_S4_")]
// 0x813180 — __ZN3RBX16MacroSubstituter19doRBX_Test_EqualityEiRKSsPKcS4_S4_S4_
// type: _DWORD __fastcall(RBX::MacroSubstituter *__hidden this, int, const std::string *, const char *__s, const char *, const char *, const char *)
pub fn stub_0x813180() -> ! {
    todo!("0x813180 __ZN3RBX16MacroSubstituter19doRBX_Test_EqualityEiRKSsPKcS4_S4_S4_")
}

#[doc(alias = "RBX::MacroSubstituter::doRBX_SimpleSubstitution(int,std::string const&,char const*,char const*)")]
#[doc(alias = "__ZN3RBX16MacroSubstituter24doRBX_SimpleSubstitutionEiRKSsPKcS4_")]
// 0x813924 — __ZN3RBX16MacroSubstituter24doRBX_SimpleSubstitutionEiRKSsPKcS4_
// type: _DWORD __fastcall(RBX::MacroSubstituter *__hidden this, int, const std::string *, const char *__s, const char *)
pub fn stub_0x813924() -> ! {
    todo!("0x813924 __ZN3RBX16MacroSubstituter24doRBX_SimpleSubstitutionEiRKSsPKcS4_")
}

#[doc(alias = "RBX::MacroSubstituter::doRBX_Test_Throw(int,std::string const&,char const*,char const*)")]
#[doc(alias = "__ZN3RBX16MacroSubstituter16doRBX_Test_ThrowEiRKSsPKcS4_")]
// 0x813d10 — __ZN3RBX16MacroSubstituter16doRBX_Test_ThrowEiRKSsPKcS4_
// type: _DWORD __fastcall(RBX::MacroSubstituter *__hidden this, int, const std::string *, const char *__s, const char *)
pub fn stub_0x813d10() -> ! {
    todo!("0x813d10 __ZN3RBX16MacroSubstituter16doRBX_Test_ThrowEiRKSsPKcS4_")
}

#[doc(alias = "RBX::MacroSubstituter::doRBX_Test_NoThrow(int,std::string const&,char const*,char const*)")]
#[doc(alias = "__ZN3RBX16MacroSubstituter18doRBX_Test_NoThrowEiRKSsPKcS4_")]
// 0x81412c — __ZN3RBX16MacroSubstituter18doRBX_Test_NoThrowEiRKSsPKcS4_
// type: _DWORD __fastcall(RBX::MacroSubstituter *__hidden this, int, const std::string *, const char *__s, const char *)
pub fn stub_0x81412c() -> ! {
    todo!("0x81412c __ZN3RBX16MacroSubstituter18doRBX_Test_NoThrowEiRKSsPKcS4_")
}

#[doc(alias = "RBX::MacroSubstituter::doRBX_Test(int,std::string const&,char const*,char const*)")]
#[doc(alias = "__ZN3RBX16MacroSubstituter10doRBX_TestEiRKSsPKcS4_")]
// 0x814548 — __ZN3RBX16MacroSubstituter10doRBX_TestEiRKSsPKcS4_
// type: _DWORD __fastcall(RBX::MacroSubstituter *__hidden this, int, const std::string *, const char *__s, const char *)
pub fn stub_0x814548() -> ! {
    todo!("0x814548 __ZN3RBX16MacroSubstituter10doRBX_TestEiRKSsPKcS4_")
}

#[doc(alias = "RBX::MacroSubstituter::appendArg(std::vector<std::string,std::allocator<std::string>> *,__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>)")]
#[doc(alias = "__ZN3RBX16MacroSubstituter9appendArgEPSt6vectorISsSaISsEEN9__gnu_cxx17__normal_iteratorIPKcSsEES9_")]
// 0x815108 — __ZN3RBX16MacroSubstituter9appendArgEPSt6vectorISsSaISsEEN9__gnu_cxx17__normal_iteratorIPKcSsEES9_
pub fn stub_0x815108() -> ! {
    todo!("0x815108 __ZN3RBX16MacroSubstituter9appendArgEPSt6vectorISsSaISsEEN9__gnu_cxx17__normal_iteratorIPKcSsEES9_")
}

#[doc(alias = "RBX::Region3::Region3(void)")]
#[doc(alias = "__ZN3RBX7Region3C1Ev")]
// 0x816d04 — __ZN3RBX7Region3C1Ev
// type: _DWORD __fastcall(RBX::Region3 *__hidden this)
pub fn stub_0x816d04() -> ! {
    todo!("0x816d04 __ZN3RBX7Region3C1Ev")
}

#[doc(alias = "RBX::Region3::init(RBX::Extents const&)")]
#[doc(alias = "__ZN3RBX7Region34initERKNS_7ExtentsE")]
// 0x816d64 — __ZN3RBX7Region34initERKNS_7ExtentsE
// type: _DWORD __fastcall(RBX::Region3 *__hidden this, const RBX::Extents *)
pub fn stub_0x816d64() -> ! {
    todo!("0x816d64 __ZN3RBX7Region34initERKNS_7ExtentsE")
}

#[doc(alias = "RBX::Region3::Region3(RBX::Extents const&)")]
#[doc(alias = "__ZN3RBX7Region3C1ERKNS_7ExtentsE")]
// 0x816e3c — __ZN3RBX7Region3C1ERKNS_7ExtentsE
// type: _DWORD __fastcall(RBX::Region3 *__hidden this, const RBX::Extents *)
pub fn stub_0x816e3c() -> ! {
    todo!("0x816e3c __ZN3RBX7Region3C1ERKNS_7ExtentsE")
}

#[doc(alias = "RBX::Region3::minPos(void)const")]
#[doc(alias = "__ZNK3RBX7Region36minPosEv")]
// 0x816e60 — __ZNK3RBX7Region36minPosEv
// type: _DWORD __fastcall(RBX::Region3 *__hidden this)
pub fn stub_0x816e60() -> ! {
    todo!("0x816e60 __ZNK3RBX7Region36minPosEv")
}

#[doc(alias = "RBX::Region3::maxPos(void)const")]
#[doc(alias = "__ZNK3RBX7Region36maxPosEv")]
// 0x816ea8 — __ZNK3RBX7Region36maxPosEv
// type: _DWORD __fastcall(RBX::Region3 *__hidden this)
pub fn stub_0x816ea8() -> ! {
    todo!("0x816ea8 __ZNK3RBX7Region36maxPosEv")
}

#[doc(alias = "RBX::LibraryService::queueExceptionThread(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string const&)")]
#[doc(alias = "__ZN3RBX14LibraryService20queueExceptionThreadEN5boost10shared_ptrINS0_18LibraryStateObjectEEERKSs")]
// 0x818074 — __ZN3RBX14LibraryService20queueExceptionThreadEN5boost10shared_ptrINS0_18LibraryStateObjectEEERKSs
// was: RBX::LibraryService::queueExceptionThread(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string const&)
pub fn stub_0x818074() -> ! {
    todo!("0x818074 __ZN3RBX14LibraryService20queueExceptionThreadEN5boost10shared_ptrINS0_18LibraryStateObjectEEERKSs")
}

#[doc(alias = "RBX::LibraryService::queueResumeThread(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>)")]
#[doc(alias = "__ZN3RBX14LibraryService17queueResumeThreadEN5boost10shared_ptrINS0_18LibraryStateObjectEEE")]
// 0x8182c4 — __ZN3RBX14LibraryService17queueResumeThreadEN5boost10shared_ptrINS0_18LibraryStateObjectEEE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
// was: RBX::LibraryService::queueResumeThread(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>)
pub fn stub_0x8182c4() -> ! {
    todo!("0x8182c4 __ZN3RBX14LibraryService17queueResumeThreadEN5boost10shared_ptrINS0_18LibraryStateObjectEEE")
}

#[doc(alias = "RBX::LibraryService::resumeAllThreadsWithException(std::string const&)")]
#[doc(alias = "__ZN3RBX14LibraryService29resumeAllThreadsWithExceptionERKSs")]
// 0x818408 — __ZN3RBX14LibraryService29resumeAllThreadsWithExceptionERKSs
// type: _DWORD __fastcall(RBX::LibraryService *__hidden this, const std::string *)
pub fn stub_0x818408() -> ! {
    todo!("0x818408 __ZN3RBX14LibraryService29resumeAllThreadsWithExceptionERKSs")
}

#[doc(alias = "RBX::LibraryService::contentReady(std::string const&,std::string const&,RBX::AsyncHttpQueue::RequestResult,std::string const*)")]
#[doc(alias = "__ZN3RBX14LibraryService12contentReadyERKSsS2_NS_14AsyncHttpQueue13RequestResultEPS1_")]
// 0x818804 — __ZN3RBX14LibraryService12contentReadyERKSsS2_NS_14AsyncHttpQueue13RequestResultEPS1_
pub fn stub_0x818804() -> ! {
    todo!("0x818804 __ZN3RBX14LibraryService12contentReadyERKSsS2_NS_14AsyncHttpQueue13RequestResultEPS1_")
}

#[doc(alias = "RBX::LibraryService::onHeartbeat(void)")]
#[doc(alias = "__ZN3RBX14LibraryService11onHeartbeatEv")]
// 0x819200 — __ZN3RBX14LibraryService11onHeartbeatEv
// type: _DWORD __fastcall(RBX::LibraryService *__hidden this)
pub fn stub_0x819200() -> ! {
    todo!("0x819200 __ZN3RBX14LibraryService11onHeartbeatEv")
}

#[doc(alias = "RBX::LibraryService::issueDelayedLibraryRequest(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>)")]
#[doc(alias = "__ZN3RBX14LibraryService26issueDelayedLibraryRequestEN5boost10shared_ptrINS0_18LibraryStateObjectEEE")]
// 0x81932c — __ZN3RBX14LibraryService26issueDelayedLibraryRequestEN5boost10shared_ptrINS0_18LibraryStateObjectEEE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, int, char, int, int, int, int, int, int, int, int)
// was: RBX::LibraryService::issueDelayedLibraryRequest(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>)
pub fn stub_0x81932c() -> ! {
    todo!("0x81932c __ZN3RBX14LibraryService26issueDelayedLibraryRequestEN5boost10shared_ptrINS0_18LibraryStateObjectEEE")
}

#[doc(alias = "RBX::DoIt(boost::function<void ()(void)>)")]
#[doc(alias = "__ZN3RBXL4DoItEN5boost8functionIFvvEEE")]
// 0x819570 — __ZN3RBXL4DoItEN5boost8functionIFvvEEE
pub fn stub_0x819570() -> ! {
    todo!("0x819570 __ZN3RBXL4DoItEN5boost8functionIFvvEEE")
}

#[doc(alias = "RBX::LibraryService::markLibrariesLoaded(void)")]
#[doc(alias = "__ZN3RBX14LibraryService19markLibrariesLoadedEv")]
// 0x819574 — __ZN3RBX14LibraryService19markLibrariesLoadedEv
// type: _DWORD __fastcall(RBX::LibraryService *__hidden this)
pub fn stub_0x819574() -> ! {
    todo!("0x819574 __ZN3RBX14LibraryService19markLibrariesLoadedEv")
}

#[doc(alias = "RBX::LibraryService::loadLocalLibrary(std::string const&)")]
#[doc(alias = "__ZN3RBX14LibraryService16loadLocalLibraryERKSs")]
// 0x81972c — __ZN3RBX14LibraryService16loadLocalLibraryERKSs
// type: _DWORD __fastcall(RBX::LibraryService *__hidden this, const std::string *)
pub fn stub_0x81972c() -> ! {
    todo!("0x81972c __ZN3RBX14LibraryService16loadLocalLibraryERKSs")
}

#[doc(alias = "RBX::LibraryService::registerLibrary(std::string const&,std::string const&,bool)")]
#[doc(alias = "__ZN3RBX14LibraryService15registerLibraryERKSsS2_b")]
// 0x819d48 — __ZN3RBX14LibraryService15registerLibraryERKSsS2_b
// type: _DWORD __fastcall(RBX::LibraryService *__hidden this, const std::string *, const std::string *, bool)
pub fn stub_0x819d48() -> ! {
    todo!("0x819d48 __ZN3RBX14LibraryService15registerLibraryERKSsS2_b")
}

#[doc(alias = "boost::multi_index::detail::ordered_index<boost::multi_index::member<boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>,long,&boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>::right>,std::less<long>,boost::multi_index::detail::nth_layer<1,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>::core_indices,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>,boost::mpl::v_item<boost::bimaps::relation::member_at::right,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_unique_tag>::insert_(boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true> const&,boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>>> *)")]
#[doc(alias = "__ZN5boost11multi_index6detail13ordered_indexINS0_6memberINS_6bimaps8relation6detail16relation_storageINS4_4tags6taggedIKSsNS5_9member_at4leftEEENS9_IKlNSB_5rightEEELb1EEElXadL_ZNSH_5rightEEEEESt4lessIlENS1_9nth_layerILi1ENS5_15mutant_relationISD_SG_N4mpl_2naELb1EEENS4_6detail10bimap_coreISslSO_SO_SO_E12core_indicesESaISP_EEENS_3mpl6v_itemISF_NSW_7vector0ISO_EELi0EEENS1_18ordered_unique_tagEE7insert_ERKSP_PNS1_18ordered_index_nodeINS15_INS1_15index_node_baseISP_SU_EEEEEE")]
// 0x9ce4b4 — __ZN5boost11multi_index6detail13ordered_indexINS0_6memberINS_6bimaps8relation6detail16relation_storageINS4_4tags6taggedIKSsNS5_9member_at4leftEEENS9_IKlNSB_5rightEEELb1EEElXadL_ZNSH_5rightEEEEESt4lessIlENS1_9nth_layerILi1ENS5_15mutant_relationISD_SG_N4mpl_2naELb1EEENS4_6detail10bimap_coreISslSO_SO_SO_E12core_indicesESaISP_EEENS_3mpl6v_itemISF_NSW_7vector0ISO_EELi0EEENS1_18ordered_unique_tagEE7insert_ERKSP_PNS1_18ordered_index_nodeINS15_INS1_15index_node_baseISP_SU_EEEEEE
// type: int __fastcall(int, int, _DWORD *)
pub fn stub_0x9ce4b4() -> ! {
    todo!("0x9ce4b4 __ZN5boost11multi_index6detail13ordered_indexINS0_6memberINS_6bimaps8relation6detail16relation_storageINS4_4tags6taggedIKSsNS5_9member_at4leftEEENS9_IKlNSB_5rightEEELb1EEElXadL_ZNSH_5rightEEEEESt4lessIlENS1_9nth_layerILi1ENS5_15mutant_relationISD_SG_N4mpl_2naELb1EEENS4_6detail10bimap_coreISslSO_SO_SO_E12core_indicesESaISP_EEENS_3mpl6v_itemISF_NSW_7vector0ISO_EELi0EEENS1_18ordered_unique_tagEE7insert_ERKSP_PNS1_18ordered_index_nodeINS15_INS1_15index_node_baseISP_SU_EEEEEE")
}

#[doc(alias = "boost::multi_index::detail::ordered_index<boost::multi_index::member<boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>,long,&boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>::right>,std::less<long>,boost::multi_index::detail::nth_layer<1,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>::core_indices,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>,boost::mpl::v_item<boost::bimaps::relation::member_at::right,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_unique_tag>::link_point(long,boost::multi_index::detail::ordered_index<boost::multi_index::member<boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>,long,&boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>::right>,std::less<long>,boost::multi_index::detail::nth_layer<1,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>::core_indices,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>,boost::mpl::v_item<boost::bimaps::relation::member_at::right,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_unique_tag>::link_info &,boost::multi_index::detail::ordered_unique_tag)")]
#[doc(alias = "__ZN5boost11multi_index6detail13ordered_indexINS0_6memberINS_6bimaps8relation6detail16relation_storageINS4_4tags6taggedIKSsNS5_9member_at4leftEEENS9_IKlNSB_5rightEEELb1EEElXadL_ZNSH_5rightEEEEESt4lessIlENS1_9nth_layerILi1ENS5_15mutant_relationISD_SG_N4mpl_2naELb1EEENS4_6detail10bimap_coreISslSO_SO_SO_E12core_indicesESaISP_EEENS_3mpl6v_itemISF_NSW_7vector0ISO_EELi0EEENS1_18ordered_unique_tagEE10link_pointElRNS12_9link_infoES11_")]
// 0x9ce548 — __ZN5boost11multi_index6detail13ordered_indexINS0_6memberINS_6bimaps8relation6detail16relation_storageINS4_4tags6taggedIKSsNS5_9member_at4leftEEENS9_IKlNSB_5rightEEELb1EEElXadL_ZNSH_5rightEEEEESt4lessIlENS1_9nth_layerILi1ENS5_15mutant_relationISD_SG_N4mpl_2naELb1EEENS4_6detail10bimap_coreISslSO_SO_SO_E12core_indicesESaISP_EEENS_3mpl6v_itemISF_NSW_7vector0ISO_EELi0EEENS1_18ordered_unique_tagEE10link_pointElRNS12_9link_infoES11_
// type: int __fastcall(int, int, int)
pub fn stub_0x9ce548() -> ! {
    todo!("0x9ce548 __ZN5boost11multi_index6detail13ordered_indexINS0_6memberINS_6bimaps8relation6detail16relation_storageINS4_4tags6taggedIKSsNS5_9member_at4leftEEENS9_IKlNSB_5rightEEELb1EEElXadL_ZNSH_5rightEEEEESt4lessIlENS1_9nth_layerILi1ENS5_15mutant_relationISD_SG_N4mpl_2naELb1EEENS4_6detail10bimap_coreISslSO_SO_SO_E12core_indicesESaISP_EEENS_3mpl6v_itemISF_NSW_7vector0ISO_EELi0EEENS1_18ordered_unique_tagEE10link_pointElRNS12_9link_infoES11_")
}

#[doc(alias = "boost::multi_index::detail::ordered_index<boost::multi_index::member<boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>,std::string,&boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>::left>,std::less<std::string>,boost::multi_index::detail::nth_layer<2,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>::core_indices,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>,boost::mpl::v_item<boost::bimaps::relation::member_at::left,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_unique_tag>::insert_(boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true> const&,boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>> *)")]
#[doc(alias = "__ZN5boost11multi_index6detail13ordered_indexINS0_6memberINS_6bimaps8relation6detail16relation_storageINS4_4tags6taggedIKSsNS5_9member_at4leftEEENS9_IKlNSB_5rightEEELb1EEESsXadL_ZNSH_4leftEEEEESt4lessISsENS1_9nth_layerILi2ENS5_15mutant_relationISD_SG_N4mpl_2naELb1EEENS4_6detail10bimap_coreISslSO_SO_SO_E12core_indicesESaISP_EEENS_3mpl6v_itemISC_NSW_7vector0ISO_EELi0EEENS1_18ordered_unique_tagEE7insert_ERKSP_PNS1_18ordered_index_nodeINS1_15index_node_baseISP_SU_EEEE")]
// 0x9ce620 — __ZN5boost11multi_index6detail13ordered_indexINS0_6memberINS_6bimaps8relation6detail16relation_storageINS4_4tags6taggedIKSsNS5_9member_at4leftEEENS9_IKlNSB_5rightEEELb1EEESsXadL_ZNSH_4leftEEEEESt4lessISsENS1_9nth_layerILi2ENS5_15mutant_relationISD_SG_N4mpl_2naELb1EEENS4_6detail10bimap_coreISslSO_SO_SO_E12core_indicesESaISP_EEENS_3mpl6v_itemISC_NSW_7vector0ISO_EELi0EEENS1_18ordered_unique_tagEE7insert_ERKSP_PNS1_18ordered_index_nodeINS1_15index_node_baseISP_SU_EEEE
// type: std::string *__fastcall(int, const std::string *, std::string *)
pub fn stub_0x9ce620() -> ! {
    todo!("0x9ce620 __ZN5boost11multi_index6detail13ordered_indexINS0_6memberINS_6bimaps8relation6detail16relation_storageINS4_4tags6taggedIKSsNS5_9member_at4leftEEENS9_IKlNSB_5rightEEELb1EEESsXadL_ZNSH_4leftEEEEESt4lessISsENS1_9nth_layerILi2ENS5_15mutant_relationISD_SG_N4mpl_2naELb1EEENS4_6detail10bimap_coreISslSO_SO_SO_E12core_indicesESaISP_EEENS_3mpl6v_itemISC_NSW_7vector0ISO_EELi0EEENS1_18ordered_unique_tagEE7insert_ERKSP_PNS1_18ordered_index_nodeINS1_15index_node_baseISP_SU_EEEE")
}

#[doc(alias = "boost::multi_index::detail::ordered_index<boost::multi_index::member<boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>,std::string,&boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>::left>,std::less<std::string>,boost::multi_index::detail::nth_layer<2,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>::core_indices,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>,boost::mpl::v_item<boost::bimaps::relation::member_at::left,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_unique_tag>::link_point(std::string const&,boost::multi_index::detail::ordered_index<boost::multi_index::member<boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>,std::string,&boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>::left>,std::less<std::string>,boost::multi_index::detail::nth_layer<2,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>::core_indices,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>,boost::mpl::v_item<boost::bimaps::relation::member_at::left,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_unique_tag>::link_info &,boost::multi_index::detail::ordered_unique_tag)")]
#[doc(alias = "__ZN5boost11multi_index6detail13ordered_indexINS0_6memberINS_6bimaps8relation6detail16relation_storageINS4_4tags6taggedIKSsNS5_9member_at4leftEEENS9_IKlNSB_5rightEEELb1EEESsXadL_ZNSH_4leftEEEEESt4lessISsENS1_9nth_layerILi2ENS5_15mutant_relationISD_SG_N4mpl_2naELb1EEENS4_6detail10bimap_coreISslSO_SO_SO_E12core_indicesESaISP_EEENS_3mpl6v_itemISC_NSW_7vector0ISO_EELi0EEENS1_18ordered_unique_tagEE10link_pointERSA_RNS12_9link_infoES11_")]
// 0x9ce6b8 — __ZN5boost11multi_index6detail13ordered_indexINS0_6memberINS_6bimaps8relation6detail16relation_storageINS4_4tags6taggedIKSsNS5_9member_at4leftEEENS9_IKlNSB_5rightEEELb1EEESsXadL_ZNSH_4leftEEEEESt4lessISsENS1_9nth_layerILi2ENS5_15mutant_relationISD_SG_N4mpl_2naELb1EEENS4_6detail10bimap_coreISslSO_SO_SO_E12core_indicesESaISP_EEENS_3mpl6v_itemISC_NSW_7vector0ISO_EELi0EEENS1_18ordered_unique_tagEE10link_pointERSA_RNS12_9link_infoES11_
// type: int __fastcall(int, const void **, int)
pub fn stub_0x9ce6b8() -> ! {
    todo!("0x9ce6b8 __ZN5boost11multi_index6detail13ordered_indexINS0_6memberINS_6bimaps8relation6detail16relation_storageINS4_4tags6taggedIKSsNS5_9member_at4leftEEENS9_IKlNSB_5rightEEELb1EEESsXadL_ZNSH_4leftEEEEESt4lessISsENS1_9nth_layerILi2ENS5_15mutant_relationISD_SG_N4mpl_2naELb1EEENS4_6detail10bimap_coreISslSO_SO_SO_E12core_indicesESaISP_EEENS_3mpl6v_itemISC_NSW_7vector0ISO_EELi0EEENS1_18ordered_unique_tagEE10link_pointERSA_RNS12_9link_infoES11_")
}

#[doc(alias = "boost::multi_index::detail::ordered_index<boost::multi_index::member<boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>,long,&boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>::right>,std::less<long>,boost::multi_index::detail::nth_layer<1,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>::core_indices,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>,boost::mpl::v_item<boost::bimaps::relation::member_at::right,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_unique_tag>::delete_all_nodes(boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>>> *)")]
#[doc(alias = "__ZN5boost11multi_index6detail13ordered_indexINS0_6memberINS_6bimaps8relation6detail16relation_storageINS4_4tags6taggedIKSsNS5_9member_at4leftEEENS9_IKlNSB_5rightEEELb1EEElXadL_ZNSH_5rightEEEEESt4lessIlENS1_9nth_layerILi1ENS5_15mutant_relationISD_SG_N4mpl_2naELb1EEENS4_6detail10bimap_coreISslSO_SO_SO_E12core_indicesESaISP_EEENS_3mpl6v_itemISF_NSW_7vector0ISO_EELi0EEENS1_18ordered_unique_tagEE16delete_all_nodesEPNS1_18ordered_index_nodeINS13_INS1_15index_node_baseISP_SU_EEEEEE")]
// 0x9ce810 — __ZN5boost11multi_index6detail13ordered_indexINS0_6memberINS_6bimaps8relation6detail16relation_storageINS4_4tags6taggedIKSsNS5_9member_at4leftEEENS9_IKlNSB_5rightEEELb1EEElXadL_ZNSH_5rightEEEEESt4lessIlENS1_9nth_layerILi1ENS5_15mutant_relationISD_SG_N4mpl_2naELb1EEENS4_6detail10bimap_coreISslSO_SO_SO_E12core_indicesESaISP_EEENS_3mpl6v_itemISF_NSW_7vector0ISO_EELi0EEENS1_18ordered_unique_tagEE16delete_all_nodesEPNS1_18ordered_index_nodeINS13_INS1_15index_node_baseISP_SU_EEEEEE
// type: void __fastcall(int, _DWORD *)
pub fn stub_0x9ce810() -> ! {
    todo!("0x9ce810 __ZN5boost11multi_index6detail13ordered_indexINS0_6memberINS_6bimaps8relation6detail16relation_storageINS4_4tags6taggedIKSsNS5_9member_at4leftEEENS9_IKlNSB_5rightEEELb1EEElXadL_ZNSH_5rightEEEEESt4lessIlENS1_9nth_layerILi1ENS5_15mutant_relationISD_SG_N4mpl_2naELb1EEENS4_6detail10bimap_coreISslSO_SO_SO_E12core_indicesESaISP_EEENS_3mpl6v_itemISF_NSW_7vector0ISO_EELi0EEENS1_18ordered_unique_tagEE16delete_all_nodesEPNS1_18ordered_index_nodeINS13_INS1_15index_node_baseISP_SU_EEEEEE")
}

#[doc(alias = "boost::bimaps::detail::map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>> boost::bimaps::container_adaptor::associative_container_adaptor<boost::multi_index::detail::ordered_index<boost::multi_index::member<boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>,std::string,&boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>::left>,std::less<std::string>,boost::multi_index::detail::nth_layer<2,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>::core_indices,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>,boost::mpl::v_item<boost::bimaps::relation::member_at::left,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_unique_tag>,boost::bimaps::detail::map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>,boost::bimaps::detail::const_map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>,std::string const,boost::bimaps::container_adaptor::support::iterator_facade_to_base<boost::bimaps::detail::map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>,boost::bimaps::detail::const_map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>>,mpl_::na,boost::bimaps::relation::detail::pair_to_relation_functor<boost::bimaps::relation::member_at::left,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>,boost::bimaps::relation::support::get_pair_functor<boost::bimaps::relation::member_at::left,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>,mpl_::na,boost::mpl::v_item<boost::bimaps::container_adaptor::detail::iterator_from_base_identity<boost::reverse_iterator<boost::multi_index::detail::bidir_node_iterator<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>>>>,boost::bimaps::detail::reverse_map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>,boost::reverse_iterator<boost::multi_index::detail::bidir_node_iterator<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>>>>,boost::bimaps::detail::const_reverse_map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>>,boost::mpl::vector<mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,1>>::find<std::string>(std::string const&)")]
#[doc(alias = "__ZN5boost6bimaps17container_adaptor29associative_container_adaptorINS_11multi_index6detail13ordered_indexINS3_6memberINS0_8relation6detail16relation_storageINS0_4tags6taggedIKSsNS7_9member_at4leftEEENSB_IKlNSD_5rightEEELb1EEESsXadL_ZNSJ_4leftEEEEESt4lessISsENS4_9nth_layerILi2ENS7_15mutant_relationISF_SI_N4mpl_2naELb1EEENS0_6detail10bimap_coreISslSQ_SQ_SQ_E12core_indicesESaISR_EEENS_3mpl6v_itemISE_NSY_7vector0ISQ_EELi0EEENS4_18ordered_unique_tagEEENSS_17map_view_iteratorISE_SU_EENSS_23const_map_view_iteratorISE_SU_EESC_NS1_7support23iterator_facade_to_baseIS16_S18_EESQ_NS8_24pair_to_relation_functorISE_SR_EENS7_7support16get_pair_functorISE_SR_EESQ_NSZ_INS1_6detail27iterator_from_base_identityINS_16reverse_iteratorINS4_19bidir_node_iteratorINS4_18ordered_index_nodeINS4_15index_node_baseISR_SW_EEEEEEEENSS_25reverse_map_view_iteratorISE_SU_EES1Q_NSS_31const_reverse_map_view_iteratorISE_SU_EEEENSY_6vectorISQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_EELi1EEEE4findISsEES16_RKT_")]
// 0x9e438c — __ZN5boost6bimaps17container_adaptor29associative_container_adaptorINS_11multi_index6detail13ordered_indexINS3_6memberINS0_8relation6detail16relation_storageINS0_4tags6taggedIKSsNS7_9member_at4leftEEENSB_IKlNSD_5rightEEELb1EEESsXadL_ZNSJ_4leftEEEEESt4lessISsENS4_9nth_layerILi2ENS7_15mutant_relationISF_SI_N4mpl_2naELb1EEENS0_6detail10bimap_coreISslSQ_SQ_SQ_E12core_indicesESaISR_EEENS_3mpl6v_itemISE_NSY_7vector0ISQ_EELi0EEENS4_18ordered_unique_tagEEENSS_17map_view_iteratorISE_SU_EENSS_23const_map_view_iteratorISE_SU_EESC_NS1_7support23iterator_facade_to_baseIS16_S18_EESQ_NS8_24pair_to_relation_functorISE_SR_EENS7_7support16get_pair_functorISE_SR_EESQ_NSZ_INS1_6detail27iterator_from_base_identityINS_16reverse_iteratorINS4_19bidir_node_iteratorINS4_18ordered_index_nodeINS4_15index_node_baseISR_SW_EEEEEEEENSS_25reverse_map_view_iteratorISE_SU_EES1Q_NSS_31const_reverse_map_view_iteratorISE_SU_EEEENSY_6vectorISQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_EELi1EEEE4findISsEES16_RKT_
// type: int __fastcall(const void ***, unsigned int *, std::string *)
pub fn stub_0x9e438c() -> ! {
    todo!("0x9e438c __ZN5boost6bimaps17container_adaptor29associative_container_adaptorINS_11multi_index6detail13ordered_indexINS3_6memberINS0_8relation6detail16relation_storageINS0_4tags6taggedIKSsNS7_9member_at4leftEEENSB_IKlNSD_5rightEEELb1EEESsXadL_ZNSJ_4leftEEEEESt4lessISsENS4_9nth_layerILi2ENS7_15mutant_relationISF_SI_N4mpl_2naELb1EEENS0_6detail10bimap_coreISslSQ_SQ_SQ_E12core_indicesESaISR_EEENS_3mpl6v_itemISE_NSY_7vector0ISQ_EELi0EEENS4_18ordered_unique_tagEEENSS_17map_view_iteratorISE_SU_EENSS_23const_map_view_iteratorISE_SU_EESC_NS1_7support23iterator_facade_to_baseIS16_S18_EESQ_NS8_24pair_to_relation_functorISE_SR_EENS7_7support16get_pair_functorISE_SR_EESQ_NSZ_INS1_6detail27iterator_from_base_identityINS_16reverse_iteratorINS4_19bidir_node_iteratorINS4_18ordered_index_nodeINS4_15index_node_baseISR_SW_EEEEEEEENSS_25reverse_map_view_iteratorISE_SU_EES1Q_NSS_31const_reverse_map_view_iteratorISE_SU_EEEENSY_6vectorISQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_EELi1EEEE4findISsEES16_RKT_")
}

#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_17StarterGuiService11CoreGuiTypeEEEE14doGetSingletonEv$shim")]
// 0xf22e64 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_17StarterGuiService11CoreGuiTypeEEEE14doGetSingletonEv$shim
// type: int()
pub fn stub_0xf22e64() -> ! {
    todo!("0xf22e64 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_17StarterGuiService11CoreGuiTypeEEEE14doGetSingletonEv$shim")
}

#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_17StarterGuiService11CoreGuiTypeERKbEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
// 0xf22e94 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_17StarterGuiService11CoreGuiTypeERKbEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
pub fn stub_0xf22e94() -> ! {
    todo!("0xf22e94 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_17StarterGuiService11CoreGuiTypeERKbEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::CoreGuiService> RBX::shared_from<RBX::CoreGuiService>(RBX::CoreGuiService*)")]
#[doc(alias = "j___ZN3RBX11shared_fromINS_14CoreGuiServiceEEEN5boost10shared_ptrIT_EEPS4_")]
// 0xf35d84 — j___ZN3RBX11shared_fromINS_14CoreGuiServiceEEEN5boost10shared_ptrIT_EEPS4_
// type: void __fastcall(_DWORD *, int)
// was: boost::shared_ptr<RBX::CoreGuiService> RBX::shared_from<RBX::CoreGuiService>(RBX::CoreGuiService*)
pub fn stub_0xf35d84() -> ! {
    todo!("0xf35d84 j___ZN3RBX11shared_fromINS_14CoreGuiServiceEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::CoreGuiService>::operator=(rbx_core::SharedPtr<RBX::CoreGuiService> const&)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX14CoreGuiServiceEEaSERKS3_")]
// 0xf36fe4 — j___ZN5boost10shared_ptrIN3RBX14CoreGuiServiceEEaSERKS3_
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
// was: boost::shared_ptr<RBX::CoreGuiService>::operator=(boost::shared_ptr<RBX::CoreGuiService> const&)
pub fn stub_0xf36fe4() -> ! {
    todo!("0xf36fe4 j___ZN5boost10shared_ptrIN3RBX14CoreGuiServiceEEaSERKS3_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CoreGuiService,RBX::CoreGuiService>(rbx_core::SharedPtr<RBX::CoreGuiService> const*,RBX::CoreGuiService *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_14CoreGuiServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// 0xf384b4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_14CoreGuiServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: int __fastcall(int, int, int, int, int, int)
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CoreGuiService,RBX::CoreGuiService>(boost::shared_ptr<RBX::CoreGuiService> const*,RBX::CoreGuiService *)const
pub fn stub_0xf384b4() -> ! {
    todo!("0xf384b4 j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_14CoreGuiServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::StarterGuiService::CoreGuiType>::~EnumDesc()")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEED2Ev")]
// 0xf3b0f4 — j___ZN3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEED2Ev
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xf3b0f4() -> ! {
    todo!("0xf3b0f4 j___ZN3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEED2Ev")
}

#[doc(alias = "j___ZN3RBX10Reflection9DescribedINS_14CoreGuiServiceELZNS_15sCoreGuiServiceEENS_17NonFactoryProductINS_13BasePlayerGuiELZNS_15sCoreGuiServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EE15classDescriptorEv")]
// 0xf3b684 — j___ZN3RBX10Reflection9DescribedINS_14CoreGuiServiceELZNS_15sCoreGuiServiceEENS_17NonFactoryProductINS_13BasePlayerGuiELZNS_15sCoreGuiServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
pub fn stub_0xf3b684() -> ! {
    todo!("0xf3b684 j___ZN3RBX10Reflection9DescribedINS_14CoreGuiServiceELZNS_15sCoreGuiServiceEENS_17NonFactoryProductINS_13BasePlayerGuiELZNS_15sCoreGuiServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EE15classDescriptorEv")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::StarterGuiService::CoreGuiType> const>::doGetSingleton(void)")]
#[doc(alias = "j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_17StarterGuiService11CoreGuiTypeEEEE14doGetSingletonEv")]
// 0xf3bf14 — j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_17StarterGuiService11CoreGuiTypeEEEE14doGetSingletonEv
// type: int(void)
pub fn stub_0xf3bf14() -> ! {
    todo!("0xf3bf14 j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_17StarterGuiService11CoreGuiTypeEEEE14doGetSingletonEv")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::StarterGuiService::CoreGuiType>::convertToItem(RBX::StarterGuiService::CoreGuiType const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEE13convertToItemERKS3_")]
// 0xf3d504 — j___ZNK3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEE13convertToItemERKS3_
pub fn stub_0xf3d504() -> ! {
    todo!("0xf3d504 j___ZNK3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEE13convertToItemERKS3_")
}

#[doc(alias = "RBX::Reflection::Call1Helper<RBX::StarterGuiService,bool (RBX::StarterGuiService::*)(RBX::StarterGuiService::CoreGuiType),RBX::StarterGuiService::CoreGuiType,bool>::call(RBX::StarterGuiService*,bool (RBX::StarterGuiService::*)(RBX::StarterGuiService::CoreGuiType),RBX::Reflection::Variant &,RBX::StarterGuiService::CoreGuiType const&)")]
#[doc(alias = "j___ZN3RBX10Reflection11Call1HelperINS_17StarterGuiServiceEMS2_FbNS2_11CoreGuiTypeEES3_bE4callEPS2_S5_RNS0_7VariantERKS3_")]
// 0xf45e44 — j___ZN3RBX10Reflection11Call1HelperINS_17StarterGuiServiceEMS2_FbNS2_11CoreGuiTypeEES3_bE4callEPS2_S5_RNS0_7VariantERKS3_
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf45e44() -> ! {
    todo!("0xf45e44 j___ZN3RBX10Reflection11Call1HelperINS_17StarterGuiServiceEMS2_FbNS2_11CoreGuiTypeEES3_bE4callEPS2_S5_RNS0_7VariantERKS3_")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::StarterGuiService,bool ()(RBX::StarterGuiService::CoreGuiType),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescINS_17StarterGuiServiceEFbNS2_11CoreGuiTypeEELi1EE16declareSignatureEPKcNS0_7VariantE")]
// 0xf45e54 — j___ZN3RBX10Reflection13BoundFuncDescINS_17StarterGuiServiceEFbNS2_11CoreGuiTypeEELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int()
pub fn stub_0xf45e54() -> ! {
    todo!("0xf45e54 j___ZN3RBX10Reflection13BoundFuncDescINS_17StarterGuiServiceEFbNS2_11CoreGuiTypeEELi1EE16declareSignatureEPKcNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::StarterGuiService,bool ()(RBX::StarterGuiService::CoreGuiType),1>::BoundFuncDesc(bool (RBX::StarterGuiService::*)(RBX::StarterGuiService::CoreGuiType),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescINS_17StarterGuiServiceEFbNS2_11CoreGuiTypeEELi1EEC2EMS2_FbS3_EPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// 0xf45e64 — j___ZN3RBX10Reflection13BoundFuncDescINS_17StarterGuiServiceEFbNS2_11CoreGuiTypeEELi1EEC2EMS2_FbS3_EPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, int, int, int, int)
pub fn stub_0xf45e64() -> ! {
    todo!("0xf45e64 j___ZN3RBX10Reflection13BoundFuncDescINS_17StarterGuiServiceEFbNS2_11CoreGuiTypeEELi1EEC2EMS2_FbS3_EPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::StarterGuiService,void ()(RBX::StarterGuiService::CoreGuiType,bool),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescINS_17StarterGuiServiceEFvNS2_11CoreGuiTypeEbELi2EE16declareSignatureEPKcNS0_7VariantES7_S8_")]
// 0xf45e74 — j___ZN3RBX10Reflection13BoundFuncDescINS_17StarterGuiServiceEFvNS2_11CoreGuiTypeEbELi2EE16declareSignatureEPKcNS0_7VariantES7_S8_
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf45e74() -> ! {
    todo!("0xf45e74 j___ZN3RBX10Reflection13BoundFuncDescINS_17StarterGuiServiceEFvNS2_11CoreGuiTypeEbELi2EE16declareSignatureEPKcNS0_7VariantES7_S8_")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::StarterGuiService,void ()(RBX::StarterGuiService::CoreGuiType,bool),2>::BoundFuncDesc(void (RBX::StarterGuiService::*)(RBX::StarterGuiService::CoreGuiType,bool),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescINS_17StarterGuiServiceEFvNS2_11CoreGuiTypeEbELi2EEC2EMS2_FvS3_bEPKcS9_S9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// 0xf45e84 — j___ZN3RBX10Reflection13BoundFuncDescINS_17StarterGuiServiceEFvNS2_11CoreGuiTypeEbELi2EEC2EMS2_FvS3_bEPKcS9_S9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, int, int, int, int, int)
pub fn stub_0xf45e84() -> ! {
    todo!("0xf45e84 j___ZN3RBX10Reflection13BoundFuncDescINS_17StarterGuiServiceEFvNS2_11CoreGuiTypeEbELi2EEC2EMS2_FvS3_bEPKcS9_S9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::CoreGuiService,int>::PropDescriptor<int (RBX::CoreGuiService::*)(void)const,int>(char const*,char const*,int (RBX::CoreGuiService::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_14CoreGuiServiceEiEC2IMS2_KFivEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// 0xf45e94 — j___ZN3RBX10Reflection14PropDescriptorINS_14CoreGuiServiceEiEC2IMS2_KFivEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
pub fn stub_0xf45e94() -> ! {
    todo!("0xf45e94 j___ZN3RBX10Reflection14PropDescriptorINS_14CoreGuiServiceEiEC2IMS2_KFivEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

#[doc(alias = "RBX::StarterGuiService::CoreGuiType & RBX::Reflection::Variant::genericConvert<RBX::StarterGuiService::CoreGuiType>(void)")]
#[doc(alias = "j___ZN3RBX10Reflection7Variant14genericConvertINS_17StarterGuiService11CoreGuiTypeEEERT_v")]
// 0xf45eb4 — j___ZN3RBX10Reflection7Variant14genericConvertINS_17StarterGuiService11CoreGuiTypeEEERT_v
// type: int()
pub fn stub_0xf45eb4() -> ! {
    todo!("0xf45eb4 j___ZN3RBX10Reflection7Variant14genericConvertINS_17StarterGuiService11CoreGuiTypeEEERT_v")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::StarterGuiService::CoreGuiType>::addPair(RBX::StarterGuiService::CoreGuiType,char const*)")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEE7addPairES3_PKc")]
// 0xf45ec4 — j___ZN3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEE7addPairES3_PKc
// type: int()
pub fn stub_0xf45ec4() -> ! {
    todo!("0xf45ec4 j___ZN3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEE7addPairES3_PKc")
}

#[doc(alias = "RBX::StarterGuiService::CoreGuiType RBX::Reflection::ArgHelper::getArg<RBX::StarterGuiService::CoreGuiType,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::StarterGuiService::CoreGuiType> const&,boost::disable_if<boost::is_same<RBX::StarterGuiService::CoreGuiType,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "j___ZN3RBX10Reflection9ArgHelper6getArgINS_17StarterGuiService11CoreGuiTypeELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")]
// 0xf45ed4 — j___ZN3RBX10Reflection9ArgHelper6getArgINS_17StarterGuiService11CoreGuiTypeELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type: int()
// was: RBX::StarterGuiService::CoreGuiType RBX::Reflection::ArgHelper::getArg<RBX::StarterGuiService::CoreGuiType,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::StarterGuiService::CoreGuiType> const&,boost::disable_if<boost::is_same<RBX::StarterGuiService::CoreGuiType,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)
pub fn stub_0xf45ed4() -> ! {
    todo!("0xf45ed4 j___ZN3RBX10Reflection9ArgHelper6getArgINS_17StarterGuiService11CoreGuiTypeELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")
}

#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<1,RBX::StarterGuiService::CoreGuiType>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::StarterGuiService::CoreGuiType &,boost::enable_if<boost::is_enum<RBX::StarterGuiService::CoreGuiType>,void>::type *)")]
#[doc(alias = "j___ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_17StarterGuiService11CoreGuiTypeEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE")]
// 0xf45ee4 — j___ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_17StarterGuiService11CoreGuiTypeEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
// type: int()
pub fn stub_0xf45ee4() -> ! {
    todo!("0xf45ee4 j___ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_17StarterGuiService11CoreGuiTypeEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::StarterGuiService,void ()(RBX::StarterGuiService::CoreGuiType,bool),rbx::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>,rbx::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)> RBX::StarterGuiService::*>::EventDesc(rbx::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)> RBX::StarterGuiService::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "j___ZN3RBX10Reflection9EventDescINS_17StarterGuiServiceEFvNS2_11CoreGuiTypeEbEN3rbx6signalIS4_EEMS2_S7_EC2ES8_PKcSB_SB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// 0xf45f34 — j___ZN3RBX10Reflection9EventDescINS_17StarterGuiServiceEFvNS2_11CoreGuiTypeEbEN3rbx6signalIS4_EEMS2_S7_EC2ES8_PKcSB_SB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, int, int, int, int)
pub fn stub_0xf45f34() -> ! {
    todo!("0xf45f34 j___ZN3RBX10Reflection9EventDescINS_17StarterGuiServiceEFvNS2_11CoreGuiTypeEbEN3rbx6signalIS4_EEMS2_S7_EC2ES8_PKcSB_SB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::StarterGuiService::CoreGuiType>(RBX::StarterGuiService::CoreGuiType const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_17StarterGuiService11CoreGuiTypeEEERS3_RKT_")]
// 0xf45fe4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_17StarterGuiService11CoreGuiTypeEEERS3_RKT_
pub fn stub_0xf45fe4() -> ! {
    todo!("0xf45fe4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_17StarterGuiService11CoreGuiTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::StarterGuiService::CoreGuiType>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX17StarterGuiService11CoreGuiTypeEE9singletonEv")]
// 0xf45ff4 — j___ZN3rbx14implementation12typed_holderIN3RBX17StarterGuiService11CoreGuiTypeEE9singletonEv
// type: int()
pub fn stub_0xf45ff4() -> ! {
    todo!("0xf45ff4 j___ZN3rbx14implementation12typed_holderIN3RBX17StarterGuiService11CoreGuiTypeEE9singletonEv")
}

#[doc(alias = "rbx::signals::signal_with_args<2,void ()(RBX::StarterGuiService::CoreGuiType,bool)>::operator()(RBX::StarterGuiService::CoreGuiType,bool)")]
#[doc(alias = "j___ZN3rbx7signals16signal_with_argsILi2EFvN3RBX17StarterGuiService11CoreGuiTypeEbEEclES4_b")]
// 0xf46004 — j___ZN3rbx7signals16signal_with_argsILi2EFvN3RBX17StarterGuiService11CoreGuiTypeEbEEclES4_b
// type: int __fastcall(int, int)
pub fn stub_0xf46004() -> ! {
    todo!("0xf46004 j___ZN3rbx7signals16signal_with_argsILi2EFvN3RBX17StarterGuiService11CoreGuiTypeEbEEclES4_b")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::disconnectAll(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE13disconnectAllEv")]
// 0xf46014 — j___ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0xf46014() -> ! {
    todo!("0xf46014 j___ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE24safe_static_do_get_mutexEv")]
// 0xf46024 — j___ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE24safe_static_do_get_mutexEv
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xf46024() -> ! {
    todo!("0xf46024 j___ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot> &)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4nextERN5boost13intrusive_ptrINS6_4slotEEE")]
// 0xf46034 — j___ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4nextERN5boost13intrusive_ptrINS6_4slotEEE
// type: int __fastcall(int, int *, int, int, char, int, int, int, int, int)
// was: rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot> &)
pub fn stub_0xf46034() -> ! {
    todo!("0xf46034 j___ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4nextERN5boost13intrusive_ptrINS6_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slot24safe_static_do_get_mutexEv")]
// 0xf46044 — j___ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slot24safe_static_do_get_mutexEv
pub fn stub_0xf46044() -> ! {
    todo!("0xf46044 j___ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::insert(rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE6insertEPNS6_4slotE")]
// 0xf46054 — j___ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE6insertEPNS6_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0xf46054() -> ! {
    todo!("0xf46054 j___ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE6insertEPNS6_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::remove(rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE6removeEPNS6_4slotE")]
// 0xf46064 — j___ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE6removeEPNS6_4slotE
// type: int __fastcall(int, char *)
pub fn stub_0xf46064() -> ! {
    todo!("0xf46064 j___ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE6removeEPNS6_4slotE")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::connect<boost::function<void ()(RBX::StarterGuiService::CoreGuiType,bool)>>(boost::function<void ()(RBX::StarterGuiService::CoreGuiType,bool)> const&)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_")]
// 0xf46074 — j___ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int, int)
pub fn stub_0xf46074() -> ! {
    todo!("0xf46074 j___ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::on_error(std::exception &)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE8on_errorERSt9exception")]
// 0xf46084 — j___ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE8on_errorERSt9exception
// type: int()
pub fn stub_0xf46084() -> ! {
    todo!("0xf46084 j___ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE8on_errorERSt9exception")
}

#[doc(alias = "RBX::StarterGuiService::CoreGuiType * rbx::any_cast<RBX::StarterGuiService::CoreGuiType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "j___ZN3rbx8any_castIN3RBX17StarterGuiService11CoreGuiTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// 0xf46094 — j___ZN3rbx8any_castIN3RBX17StarterGuiService11CoreGuiTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: int()
pub fn stub_0xf46094() -> ! {
    todo!("0xf46094 j___ZN3rbx8any_castIN3RBX17StarterGuiService11CoreGuiTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::StarterGuiService::CoreGuiType const& rbx::any_cast<RBX::StarterGuiService::CoreGuiType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRKN3RBX17StarterGuiService11CoreGuiTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf460a4 — j___ZN3rbx8any_castIRKN3RBX17StarterGuiService11CoreGuiTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int()
pub fn stub_0xf460a4() -> ! {
    todo!("0xf460a4 j___ZN3rbx8any_castIRKN3RBX17StarterGuiService11CoreGuiTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::StarterGuiService::CoreGuiType & rbx::any_cast<RBX::StarterGuiService::CoreGuiType &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRN3RBX17StarterGuiService11CoreGuiTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf460b4 — j___ZN3rbx8any_castIRN3RBX17StarterGuiService11CoreGuiTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int()
pub fn stub_0xf460b4() -> ! {
    todo!("0xf460b4 j___ZN3rbx8any_castIRN3RBX17StarterGuiService11CoreGuiTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot,boost::function<void ()(RBX::StarterGuiService::CoreGuiType,bool)>,2,void ()(RBX::StarterGuiService::CoreGuiType,bool)>::callable<rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>*>(boost::function<void ()(RBX::StarterGuiService::CoreGuiType,bool)> const&,rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>*)")]
#[doc(alias = "j___ZN3rbx8callableINS_7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotEN5boost8functionIS6_EELi2ES6_EC2IPS7_EERKSB_T_")]
// 0xf460c4 — j___ZN3rbx8callableINS_7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotEN5boost8functionIS6_EELi2ES6_EC2IPS7_EERKSB_T_
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xf460c4() -> ! {
    todo!("0xf460c4 j___ZN3rbx8callableINS_7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotEN5boost8functionIS6_EELi2ES6_EC2IPS7_EERKSB_T_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot>::operator=(rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot*)")]
#[doc(alias = "j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotEEaSEPS9_")]
// 0xf46134 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotEEaSEPS9_
// type: int()
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot>::operator=(rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot*)
pub fn stub_0xf46134() -> ! {
    todo!("0xf46134 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotEEaSEPS9_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot> const&)")]
#[doc(alias = "j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotEEaSERKSA_")]
// 0xf46144 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotEEaSERKSA_
// type: int(void)
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot> const&)
pub fn stub_0xf46144() -> ! {
    todo!("0xf46144 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotEEaSERKSA_")
}

#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::StarterGuiService::CoreGuiType const&,bool const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<RBX::StarterGuiService::CoreGuiType,bool>(RBX::StarterGuiService::CoreGuiType &,bool &)")]
#[doc(alias = "j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS4_17StarterGuiService11CoreGuiTypeERKbEENS0_5list3INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEENSJ_ILi2EEEEEEclIS8_bEEvRT_RT0_")]
// 0xf46154 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS4_17StarterGuiService11CoreGuiTypeERKbEENS0_5list3INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEENSJ_ILi2EEEEEEclIS8_bEEvRT_RT0_
// type: int()
// was: void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::StarterGuiService::CoreGuiType const&,bool const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<RBX::StarterGuiService::CoreGuiType,bool>(RBX::StarterGuiService::CoreGuiType &,bool &)
pub fn stub_0xf46154() -> ! {
    todo!("0xf46154 j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS4_17StarterGuiService11CoreGuiTypeERKbEENS0_5list3INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEENSJ_ILi2EEEEEEclIS8_bEEvRT_RT0_")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::StarterGuiService::CoreGuiType const&,bool const&>,boost::_bi::list_av_3<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::StarterGuiService::CoreGuiType const&,bool const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::StarterGuiService::CoreGuiType const&,bool const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")]
#[doc(alias = "j___ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS1_17StarterGuiService11CoreGuiTypeERKbNS_10shared_ptrIS3_EENS_3argILi1EEENSC_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISH_T0_T1_T2_EENSF_9list_av_3IT3_T4_T5_E4typeEEEMSK_FSH_SL_SM_ESP_SQ_SR_")]
// 0xf46164 — j___ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS1_17StarterGuiService11CoreGuiTypeERKbNS_10shared_ptrIS3_EENS_3argILi1EEENSC_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISH_T0_T1_T2_EENSF_9list_av_3IT3_T4_T5_E4typeEEEMSK_FSH_SL_SM_ESP_SQ_SR_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::StarterGuiService::CoreGuiType const&,bool const&>,boost::_bi::list_av_3<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::StarterGuiService::CoreGuiType const&,bool const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::StarterGuiService::CoreGuiType const&,bool const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)
pub fn stub_0xf46164() -> ! {
    todo!("0xf46164 j___ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS1_17StarterGuiService11CoreGuiTypeERKbNS_10shared_ptrIS3_EENS_3argILi1EEENSC_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISH_T0_T1_T2_EENSF_9list_av_3IT3_T4_T5_E4typeEEEMSK_FSH_SL_SM_ESP_SQ_SR_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::StarterGuiService::CoreGuiType const&,bool const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_17StarterGuiService11CoreGuiTypeERKbEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// 0xf461b4 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_17StarterGuiService11CoreGuiTypeERKbEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::StarterGuiService::CoreGuiType const&,bool const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_0xf461b4() -> ! {
    todo!("0xf461b4 j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_17StarterGuiService11CoreGuiTypeERKbEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "j___ZN5boost8functionIFvN3RBX17StarterGuiService11CoreGuiTypeEbEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS3_RKbEENS7_5list3INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
// 0xf461c4 — j___ZN5boost8functionIFvN3RBX17StarterGuiService11CoreGuiTypeEbEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS3_RKbEENS7_5list3INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0xf461c4() -> ! {
    todo!("0xf461c4 j___ZN5boost8functionIFvN3RBX17StarterGuiService11CoreGuiTypeEbEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS3_RKbEENS7_5list3INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "boost::function2<void,RBX::StarterGuiService::CoreGuiType,bool>::assign_to_own(boost::function2<void,RBX::StarterGuiService::CoreGuiType,bool> const&)")]
#[doc(alias = "j___ZN5boost9function2IvN3RBX17StarterGuiService11CoreGuiTypeEbE13assign_to_ownERKS4_")]
// 0xf461d4 — j___ZN5boost9function2IvN3RBX17StarterGuiService11CoreGuiTypeEbE13assign_to_ownERKS4_
// type: int()
pub fn stub_0xf461d4() -> ! {
    todo!("0xf461d4 j___ZN5boost9function2IvN3RBX17StarterGuiService11CoreGuiTypeEbE13assign_to_ownERKS4_")
}

#[doc(alias = "boost::function2<void,RBX::StarterGuiService::CoreGuiType,bool>::clear(void)")]
#[doc(alias = "j___ZN5boost9function2IvN3RBX17StarterGuiService11CoreGuiTypeEbE5clearEv")]
// 0xf461e4 — j___ZN5boost9function2IvN3RBX17StarterGuiService11CoreGuiTypeEbE5clearEv
// type: int()
pub fn stub_0xf461e4() -> ! {
    todo!("0xf461e4 j___ZN5boost9function2IvN3RBX17StarterGuiService11CoreGuiTypeEbE5clearEv")
}

#[doc(alias = "void boost::function2<void,RBX::StarterGuiService::CoreGuiType,bool>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::StarterGuiService::CoreGuiType const&,bool const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::StarterGuiService::CoreGuiType const&,bool const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")]
#[doc(alias = "j___ZN5boost9function2IvN3RBX17StarterGuiService11CoreGuiTypeEbE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS3_RKbEENS6_5list3INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEvT_")]
// 0xf461f4 — j___ZN5boost9function2IvN3RBX17StarterGuiService11CoreGuiTypeEbE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS3_RKbEENS6_5list3INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: void boost::function2<void,RBX::StarterGuiService::CoreGuiType,bool>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::StarterGuiService::CoreGuiType const&,bool const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::StarterGuiService::CoreGuiType const&,bool const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)
pub fn stub_0xf461f4() -> ! {
    todo!("0xf461f4 j___ZN5boost9function2IvN3RBX17StarterGuiService11CoreGuiTypeEbE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS3_RKbEENS6_5list3INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEvT_")
}

#[doc(alias = "j___ZN5boost9function2IvN3RBX17StarterGuiService11CoreGuiTypeEbEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS3_RKbEENS6_5list3INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")]
// 0xf46204 — j___ZN5boost9function2IvN3RBX17StarterGuiService11CoreGuiTypeEbEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS3_RKbEENS6_5list3INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0xf46204() -> ! {
    todo!("0xf46204 j___ZN5boost9function2IvN3RBX17StarterGuiService11CoreGuiTypeEbEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS3_RKbEENS6_5list3INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>,RBX::StarterGuiService::CoreGuiType,bool,boost::hash<RBX::StarterGuiService::CoreGuiType>,std::equal_to<RBX::StarterGuiService::CoreGuiType>>>::rehash_impl(unsigned long)")]
#[doc(alias = "j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE11rehash_implEm")]
// 0xf46214 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE11rehash_implEm
// type: int()
pub fn stub_0xf46214() -> ! {
    todo!("0xf46214 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE11rehash_implEm")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>,RBX::StarterGuiService::CoreGuiType,bool,boost::hash<RBX::StarterGuiService::CoreGuiType>,std::equal_to<RBX::StarterGuiService::CoreGuiType>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>,RBX::StarterGuiService::CoreGuiType,bool,boost::hash<RBX::StarterGuiService::CoreGuiType>,std::equal_to<RBX::StarterGuiService::CoreGuiType>>> &,boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE15place_in_bucketERNS1_5tableISF_EEPNS1_10ptr_bucketE")]
// 0xf46224 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE15place_in_bucketERNS1_5tableISF_EEPNS1_10ptr_bucketE
// type: int()
pub fn stub_0xf46224() -> ! {
    todo!("0xf46224 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE15place_in_bucketERNS1_5tableISF_EEPNS1_10ptr_bucketE")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>,RBX::StarterGuiService::CoreGuiType,bool,boost::hash<RBX::StarterGuiService::CoreGuiType>,std::equal_to<RBX::StarterGuiService::CoreGuiType>>>::operator[](RBX::StarterGuiService::CoreGuiType const&)")]
#[doc(alias = "j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEEixERS8_")]
// 0xf46234 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEEixERS8_
// type: int __fastcall(int, int, int, int, void *, int, int, int, int)
pub fn stub_0xf46234() -> ! {
    todo!("0xf46234 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEEixERS8_")
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>>>::construct(void)")]
#[doc(alias = "j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEEEEE9constructEv")]
// 0xf46244 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEEEEE9constructEv
// type: int()
pub fn stub_0xf46244() -> ! {
    todo!("0xf46244 j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEEEEE9constructEv")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>,RBX::StarterGuiService::CoreGuiType,bool,boost::hash<RBX::StarterGuiService::CoreGuiType>,std::equal_to<RBX::StarterGuiService::CoreGuiType>>>::create_buckets(unsigned long)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm")]
// 0xf46254 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm
// type: int __fastcall(int, int, int, int, int, int, int, int, int)
pub fn stub_0xf46254() -> ! {
    todo!("0xf46254 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>,RBX::StarterGuiService::CoreGuiType,bool,boost::hash<RBX::StarterGuiService::CoreGuiType>,std::equal_to<RBX::StarterGuiService::CoreGuiType>>>::delete_buckets(void)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE14delete_bucketsEv")]
// 0xf46264 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE14delete_bucketsEv
// type: int()
pub fn stub_0xf46264() -> ! {
    todo!("0xf46264 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE14delete_bucketsEv")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>,RBX::StarterGuiService::CoreGuiType,bool,boost::hash<RBX::StarterGuiService::CoreGuiType>,std::equal_to<RBX::StarterGuiService::CoreGuiType>>>::reserve_for_insert(unsigned long)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE18reserve_for_insertEm")]
// 0xf46274 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE18reserve_for_insertEm
// type: int()
pub fn stub_0xf46274() -> ! {
    todo!("0xf46274 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE18reserve_for_insertEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>,RBX::StarterGuiService::CoreGuiType,bool,boost::hash<RBX::StarterGuiService::CoreGuiType>,std::equal_to<RBX::StarterGuiService::CoreGuiType>>>::table(unsigned long,boost::hash<RBX::StarterGuiService::CoreGuiType> const&,std::equal_to<RBX::StarterGuiService::CoreGuiType> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>> const&)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEEC2EmRKSC_RKSE_RKSaINS1_8ptr_nodeIS9_EEE")]
// 0xf46284 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEEC2EmRKSC_RKSE_RKSaINS1_8ptr_nodeIS9_EEE
// type: int()
pub fn stub_0xf46284() -> ! {
    todo!("0xf46284 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEEC2EmRKSC_RKSE_RKSaINS1_8ptr_nodeIS9_EEE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::StarterGuiService::CoreGuiType>::convertToValue(RBX::Name const&,RBX::StarterGuiService::CoreGuiType&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEE14convertToValueERKNS_4NameERS3_")]
// 0xf46294 — j___ZNK3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEE14convertToValueERKNS_4NameERS3_
// type: int()
pub fn stub_0xf46294() -> ! {
    todo!("0xf46294 j___ZNK3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEE14convertToValueERKNS_4NameERS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::StarterGuiService::CoreGuiType>::convertToString(RBX::StarterGuiService::CoreGuiType const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEE15convertToStringERKS3_")]
// 0xf462a4 — j___ZNK3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0xf462a4() -> ! {
    todo!("0xf462a4 j___ZNK3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEE15convertToStringERKS3_")
}

#[doc(alias = "void boost::detail::function::basic_vtable2<void,RBX::StarterGuiService::CoreGuiType,bool>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::StarterGuiService::CoreGuiType const&,bool const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::StarterGuiService::CoreGuiType const&,bool const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable2IvN3RBX17StarterGuiService11CoreGuiTypeEbE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_RKbEENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
// 0xf462f4 — j___ZNK5boost6detail8function13basic_vtable2IvN3RBX17StarterGuiService11CoreGuiTypeEbE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_RKbEENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int)
// was: void boost::detail::function::basic_vtable2<void,RBX::StarterGuiService::CoreGuiType,bool>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::StarterGuiService::CoreGuiType const&,bool const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::StarterGuiService::CoreGuiType const&,bool const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_0xf462f4() -> ! {
    todo!("0xf462f4 j___ZNK5boost6detail8function13basic_vtable2IvN3RBX17StarterGuiService11CoreGuiTypeEbE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_RKbEENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,RBX::StarterGuiService::CoreGuiType,bool>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::StarterGuiService::CoreGuiType const&,bool const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::StarterGuiService::CoreGuiType const&,bool const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable2IvN3RBX17StarterGuiService11CoreGuiTypeEbE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_RKbEENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferE")]
// 0xf46304 — j___ZNK5boost6detail8function13basic_vtable2IvN3RBX17StarterGuiService11CoreGuiTypeEbE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_RKbEENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: bool boost::detail::function::basic_vtable2<void,RBX::StarterGuiService::CoreGuiType,bool>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::StarterGuiService::CoreGuiType const&,bool const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::StarterGuiService::CoreGuiType const&,bool const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const
pub fn stub_0xf46304() -> ! {
    todo!("0xf46304 j___ZNK5boost6detail8function13basic_vtable2IvN3RBX17StarterGuiService11CoreGuiTypeEbE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_RKbEENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,RBX::StarterGuiService::CoreGuiType,bool>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::StarterGuiService::CoreGuiType const&,bool const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::StarterGuiService::CoreGuiType const&,bool const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable2IvN3RBX17StarterGuiService11CoreGuiTypeEbE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_RKbEENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// 0xf46314 — j___ZNK5boost6detail8function13basic_vtable2IvN3RBX17StarterGuiService11CoreGuiTypeEbE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_RKbEENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: bool boost::detail::function::basic_vtable2<void,RBX::StarterGuiService::CoreGuiType,bool>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::StarterGuiService::CoreGuiType const&,bool const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::StarterGuiService::CoreGuiType const&,bool const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0xf46314() -> ! {
    todo!("0xf46314 j___ZNK5boost6detail8function13basic_vtable2IvN3RBX17StarterGuiService11CoreGuiTypeEbE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_RKbEENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "boost::function2<void,RBX::StarterGuiService::CoreGuiType,bool>::operator()(RBX::StarterGuiService::CoreGuiType,bool)const")]
#[doc(alias = "j___ZNK5boost9function2IvN3RBX17StarterGuiService11CoreGuiTypeEbEclES3_b")]
// 0xf46324 — j___ZNK5boost9function2IvN3RBX17StarterGuiService11CoreGuiTypeEbEclES3_b
// type: int()
pub fn stub_0xf46324() -> ! {
    todo!("0xf46324 j___ZNK5boost9function2IvN3RBX17StarterGuiService11CoreGuiTypeEbEclES3_b")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>,RBX::StarterGuiService::CoreGuiType,bool,boost::hash<RBX::StarterGuiService::CoreGuiType>,std::equal_to<RBX::StarterGuiService::CoreGuiType>>>::find_node_impl<RBX::StarterGuiService::CoreGuiType,std::equal_to<RBX::StarterGuiService::CoreGuiType>>(unsigned long,RBX::StarterGuiService::CoreGuiType const&,std::equal_to<RBX::StarterGuiService::CoreGuiType> const&)const")]
#[doc(alias = "j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE14find_node_implIS7_SE_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS9_EEEEmRKT_RKT0_")]
// 0xf46334 — j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE14find_node_implIS7_SE_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS9_EEEEmRKT_RKT0_
// type: int()
pub fn stub_0xf46334() -> ! {
    todo!("0xf46334 j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE14find_node_implIS7_SE_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS9_EEEEmRKT_RKT0_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>,RBX::StarterGuiService::CoreGuiType,bool,boost::hash<RBX::StarterGuiService::CoreGuiType>,std::equal_to<RBX::StarterGuiService::CoreGuiType>>>::min_buckets_for_size(unsigned long)const")]
#[doc(alias = "j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE20min_buckets_for_sizeEm")]
// 0xf46344 — j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE20min_buckets_for_sizeEm
// type: int()
pub fn stub_0xf46344() -> ! {
    todo!("0xf46344 j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE20min_buckets_for_sizeEm")
}

#[doc(alias = "std::_Vector_base<RBX::StarterGuiService::CoreGuiType,std::allocator<RBX::StarterGuiService::CoreGuiType>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIN3RBX17StarterGuiService11CoreGuiTypeESaIS2_EE11_M_allocateEm")]
// 0xf46354 — j___ZNSt12_Vector_baseIN3RBX17StarterGuiService11CoreGuiTypeESaIS2_EE11_M_allocateEm
// type: int()
pub fn stub_0xf46354() -> ! {
    todo!("0xf46354 j___ZNSt12_Vector_baseIN3RBX17StarterGuiService11CoreGuiTypeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::StarterGuiService::CoreGuiType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::StarterGuiService::CoreGuiType *,RBX::StarterGuiService::CoreGuiType *>(RBX::StarterGuiService::CoreGuiType *,RBX::StarterGuiService::CoreGuiType *,RBX::StarterGuiService::CoreGuiType *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX17StarterGuiService11CoreGuiTypeES6_EET0_T_S8_S7_")]
// 0xf46364 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX17StarterGuiService11CoreGuiTypeES6_EET0_T_S8_S7_
// type: int()
pub fn stub_0xf46364() -> ! {
    todo!("0xf46364 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX17StarterGuiService11CoreGuiTypeES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::StarterGuiService::CoreGuiType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "j___ZNSt3mapIPKN3RBX4NameENS0_17StarterGuiService11CoreGuiTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
// 0xf46374 — j___ZNSt3mapIPKN3RBX4NameENS0_17StarterGuiService11CoreGuiTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_0xf46374() -> ! {
    todo!("0xf46374 j___ZNSt3mapIPKN3RBX4NameENS0_17StarterGuiService11CoreGuiTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::vector<RBX::StarterGuiService::CoreGuiType,std::allocator<RBX::StarterGuiService::CoreGuiType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::StarterGuiService::CoreGuiType*,std::vector<RBX::StarterGuiService::CoreGuiType,std::allocator<RBX::StarterGuiService::CoreGuiType>>>,RBX::StarterGuiService::CoreGuiType const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX17StarterGuiService11CoreGuiTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0xf46384 — j___ZNSt6vectorIN3RBX17StarterGuiService11CoreGuiTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int()
pub fn stub_0xf46384() -> ! {
    todo!("0xf46384 j___ZNSt6vectorIN3RBX17StarterGuiService11CoreGuiTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::StarterGuiService::CoreGuiType,std::allocator<RBX::StarterGuiService::CoreGuiType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::StarterGuiService::CoreGuiType*,std::vector<RBX::StarterGuiService::CoreGuiType,std::allocator<RBX::StarterGuiService::CoreGuiType>>>,unsigned long,RBX::StarterGuiService::CoreGuiType const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX17StarterGuiService11CoreGuiTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// 0xf46394 — j___ZNSt6vectorIN3RBX17StarterGuiService11CoreGuiTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int()
pub fn stub_0xf46394() -> ! {
    todo!("0xf46394 j___ZNSt6vectorIN3RBX17StarterGuiService11CoreGuiTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::StarterGuiService::CoreGuiType,std::allocator<RBX::StarterGuiService::CoreGuiType>>::resize(unsigned long,RBX::StarterGuiService::CoreGuiType)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX17StarterGuiService11CoreGuiTypeESaIS2_EE6resizeEmS2_")]
// 0xf463a4 — j___ZNSt6vectorIN3RBX17StarterGuiService11CoreGuiTypeESaIS2_EE6resizeEmS2_
// type: int()
pub fn stub_0xf463a4() -> ! {
    todo!("0xf463a4 j___ZNSt6vectorIN3RBX17StarterGuiService11CoreGuiTypeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::StarterGuiService::CoreGuiType,std::allocator<RBX::StarterGuiService::CoreGuiType>>::push_back(RBX::StarterGuiService::CoreGuiType const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX17StarterGuiService11CoreGuiTypeESaIS2_EE9push_backERKS2_")]
// 0xf463b4 — j___ZNSt6vectorIN3RBX17StarterGuiService11CoreGuiTypeESaIS2_EE9push_backERKS2_
// type: int()
pub fn stub_0xf463b4() -> ! {
    todo!("0xf463b4 j___ZNSt6vectorIN3RBX17StarterGuiService11CoreGuiTypeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17StarterGuiService11CoreGuiTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
// 0xf463c4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17StarterGuiService11CoreGuiTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int()
pub fn stub_0xf463c4() -> ! {
    todo!("0xf463c4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17StarterGuiService11CoreGuiTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>>,std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17StarterGuiService11CoreGuiTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
// 0xf463d4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17StarterGuiService11CoreGuiTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0xf463d4() -> ! {
    todo!("0xf463d4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17StarterGuiService11CoreGuiTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17StarterGuiService11CoreGuiTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
// 0xf463e4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17StarterGuiService11CoreGuiTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int()
pub fn stub_0xf463e4() -> ! {
    todo!("0xf463e4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17StarterGuiService11CoreGuiTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::StarterGuiService::CoreGuiType> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17StarterGuiService11CoreGuiTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
// 0xf463f4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17StarterGuiService11CoreGuiTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int()
pub fn stub_0xf463f4() -> ! {
    todo!("0xf463f4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17StarterGuiService11CoreGuiTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "void boost::spirit::classic::utility::impl::construct_chset<char,char>(rbx_core::SharedPtr<boost::spirit::classic::basic_chset<char>> &,char const*)")]
#[doc(alias = "j___ZN5boost6spirit7classic7utility4impl15construct_chsetIccEEvRNS_10shared_ptrINS1_11basic_chsetIT_EEEEPKT0_")]
// 0xf51a24 — j___ZN5boost6spirit7classic7utility4impl15construct_chsetIccEEvRNS_10shared_ptrINS1_11basic_chsetIT_EEEEPKT0_
// was: void boost::spirit::classic::utility::impl::construct_chset<char,char>(boost::shared_ptr<boost::spirit::classic::basic_chset<char>> &,char const*)
pub fn stub_0xf51a24() -> ! {
    todo!("0xf51a24 j___ZN5boost6spirit7classic7utility4impl15construct_chsetIccEEvRNS_10shared_ptrINS1_11basic_chsetIT_EEEEPKT0_")
}
