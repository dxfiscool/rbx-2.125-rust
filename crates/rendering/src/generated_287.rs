//! rendering shard 287 — 100 stubs EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Render 15586/15586 complete, 31240->31340 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 31240 before -> 31340 after; global gap filler)
//! Filter: Ogre|G3D|Render exhausted (0 remaining), filler global asc next 100 after 0x3f3c38

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0x3f3cdc — __ZN5boost10shared_ptrIN3RBX17copy_on_write_ptrISt6vectorINS0_INS1_8InstanceEEESaIS5_EEEEEC2IS8_EEPT_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::shared_ptr<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>(RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> *)")]
// was: __ZN5boost10shared_ptrIN3RBX17copy_on_write_ptrISt6vectorINS0_INS1_8InstanceEEESaIS5_EEEEEC2IS8_EEPT_
pub fn stub_3f3cdc() -> ! {
    todo!("0x3f3cdc boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>(RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> *)")
}

// 0x3f3db0 — __ZN5boost6detail12shared_countC2IN3RBX17copy_on_write_ptrISt6vectorINS_10shared_ptrINS3_8InstanceEEESaIS8_EEEEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>(RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> *)")]
// was: __ZN5boost6detail12shared_countC2IN3RBX17copy_on_write_ptrISt6vectorINS_10shared_ptrINS3_8InstanceEEESaIS8_EEEEEEPT_
pub fn stub_3f3db0() -> ! {
    todo!("0x3f3db0 boost::detail::shared_count::shared_count<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>(RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> *)")
}

// 0x3f3ec0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX17copy_on_write_ptrISt6vectorINS_10shared_ptrINS2_8InstanceEEESaIS7_EEEEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX17copy_on_write_ptrISt6vectorINS_10shared_ptrINS2_8InstanceEEESaIS7_EEEEED1Ev
pub fn stub_3f3ec0() -> ! {
    todo!("0x3f3ec0 boost::detail::sp_counted_impl_p<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::~sp_counted_impl_p()")
}

// 0x3f3ec4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX17copy_on_write_ptrISt6vectorINS_10shared_ptrINS2_8InstanceEEESaIS7_EEEEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX17copy_on_write_ptrISt6vectorINS_10shared_ptrINS2_8InstanceEEESaIS7_EEEEED0Ev
pub fn stub_3f3ec4() -> ! {
    todo!("0x3f3ec4 boost::detail::sp_counted_impl_p<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::~sp_counted_impl_p()")
}

// 0x3f3ec8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX17copy_on_write_ptrISt6vectorINS_10shared_ptrINS2_8InstanceEEESaIS7_EEEEE7disposeEv
// type: void __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::dispose(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX17copy_on_write_ptrISt6vectorINS_10shared_ptrINS2_8InstanceEEESaIS7_EEEEE7disposeEv
pub fn stub_3f3ec8() -> ! {
    todo!("0x3f3ec8 boost::detail::sp_counted_impl_p<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::dispose(void)")
}

// 0x3f3f70 — __ZN5boost6detail17sp_counted_impl_pIN3RBX17copy_on_write_ptrISt6vectorINS_10shared_ptrINS2_8InstanceEEESaIS7_EEEEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX17copy_on_write_ptrISt6vectorINS_10shared_ptrINS2_8InstanceEEESaIS7_EEEEE11get_deleterERKSt9type_info
pub fn stub_3f3f70() -> ! {
    todo!("0x3f3f70 boost::detail::sp_counted_impl_p<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::get_deleter(std::type_info const&)")
}

// 0x3f3f74 — __ZN5boost6detail17sp_counted_impl_pIN3RBX17copy_on_write_ptrISt6vectorINS_10shared_ptrINS2_8InstanceEEESaIS7_EEEEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX17copy_on_write_ptrISt6vectorINS_10shared_ptrINS2_8InstanceEEESaIS7_EEEEE19get_untyped_deleterEv
pub fn stub_3f3f74() -> ! {
    todo!("0x3f3f74 boost::detail::sp_counted_impl_p<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::get_untyped_deleter(void)")
}

// 0x3f3f78 — __ZNSt4pairIKSsN5boost10shared_ptrIN3RBX17copy_on_write_ptrISt6vectorINS2_INS3_8InstanceEEESaIS7_EEEEEEEC2ERS0_RKSB_
// type: _DWORD *__fastcall(_DWORD *, const std::string *, const shared_count *)
#[doc(alias = "std::pair<std::string const,rbx_core::SharedPtr<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>::pair(std::string const&,rbx_core::SharedPtr<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>> const&)")]
// was: __ZNSt4pairIKSsN5boost10shared_ptrIN3RBX17copy_on_write_ptrISt6vectorINS2_INS3_8InstanceEEESaIS7_EEEEEEEC2ERS0_RKSB_
pub fn stub_3f3f78() -> ! {
    todo!("0x3f3f78 std::pair<std::string const,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>::pair(std::string const&,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>> const&)")
}

// 0x3f4034 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17copy_on_write_ptrISt6vectorINS3_INS4_8InstanceEEESaIS8_EEEEEEESt10_Select1stISD_ESt4lessISsESaISD_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISD_ERKSD_
// type: _Rb_tree_node_base *__fastcall(_DWORD *, _Rb_tree_node_base *, const std::string *)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,rbx_core::SharedPtr<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>,std::pair<std::string const,rbx_core::SharedPtr<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17copy_on_write_ptrISt6vectorINS3_INS4_8InstanceEEESaIS8_EEEEEEESt10_Select1stISD_ESt4lessISsESaISD_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISD_ERKSD_
pub fn stub_3f4034() -> ! {
    todo!("0x3f4034 std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>>,std::pair<std::string const,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>> const&)")
}

// 0x3f4120 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17copy_on_write_ptrISt6vectorINS3_INS4_8InstanceEEESaIS8_EEEEEEESt10_Select1stISD_ESt4lessISsESaISD_EE9_M_insertEPSt18_Rb_tree_node_baseSL_RKSD_
// type: _Rb_tree_node_base *__fastcall(int, int, _Rb_tree_node_base *, std::string *)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,rbx_core::SharedPtr<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17copy_on_write_ptrISt6vectorINS3_INS4_8InstanceEEESaIS8_EEEEEEESt10_Select1stISD_ESt4lessISsESaISD_EE9_M_insertEPSt18_Rb_tree_node_baseSL_RKSD_
pub fn stub_3f4120() -> ! {
    todo!("0x3f4120 std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>> const&)")
}

// 0x3f4170 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17copy_on_write_ptrISt6vectorINS3_INS4_8InstanceEEESaIS8_EEEEEEESt10_Select1stISD_ESt4lessISsESaISD_EE16_M_insert_uniqueERKSD_
// type: int __fastcall(int, int, std::string *)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>>::_M_insert_unique(std::pair<std::string const,rbx_core::SharedPtr<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17copy_on_write_ptrISt6vectorINS3_INS4_8InstanceEEESaIS8_EEEEEEESt10_Select1stISD_ESt4lessISsESaISD_EE16_M_insert_uniqueERKSD_
pub fn stub_3f4170() -> ! {
    todo!("0x3f4170 std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>>>::_M_insert_unique(std::pair<std::string const,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>> const&)")
}

// 0x3f41f4 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17copy_on_write_ptrISt6vectorINS3_INS4_8InstanceEEESaIS8_EEEEEEESt10_Select1stISD_ESt4lessISsESaISD_EE14_M_create_nodeERKSD_
// type: _DWORD *__fastcall(int, const shared_count *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>>::_M_create_node(std::pair<std::string const,rbx_core::SharedPtr<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17copy_on_write_ptrISt6vectorINS3_INS4_8InstanceEEESaIS8_EEEEEEESt10_Select1stISD_ESt4lessISsESaISD_EE14_M_create_nodeERKSD_
pub fn stub_3f41f4() -> ! {
    todo!("0x3f41f4 std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>>>::_M_create_node(std::pair<std::string const,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>> const&)")
}

// 0x3f42fc — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17copy_on_write_ptrISt6vectorINS3_INS4_8InstanceEEESaIS8_EEEEEEESt10_Select1stISD_ESt4lessISsESaISD_EE11lower_boundERS1_
// type: _DWORD *__fastcall(int, std::string *)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>>::lower_bound(std::string const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17copy_on_write_ptrISt6vectorINS3_INS4_8InstanceEEESaIS8_EEEEEEESt10_Select1stISD_ESt4lessISsESaISD_EE11lower_boundERS1_
pub fn stub_3f42fc() -> ! {
    todo!("0x3f42fc std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>>>::lower_bound(std::string const&)")
}

// 0x3f432c — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17copy_on_write_ptrISt6vectorINS3_INS4_8InstanceEEESaIS8_EEEEEEESt10_Select1stISD_ESt4lessISsESaISD_EE4findERS1_
// type: _DWORD *__fastcall(int, std::string *this)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>>::find(std::string const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17copy_on_write_ptrISt6vectorINS3_INS4_8InstanceEEESaIS8_EEEEEEESt10_Select1stISD_ESt4lessISsESaISD_EE4findERS1_
pub fn stub_3f432c() -> ! {
    todo!("0x3f432c std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>>>::find(std::string const&)")
}

// 0x3f437c — __ZN3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3f437c() -> ! {
    todo!("0x3f437c __ZN3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x3f4380 — __ZN3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3f4380() -> ! {
    todo!("0x3f4380 __ZN3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x3f4420 — __ZThn32_N3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3f4420() -> ! {
    todo!("0x3f4420 __ZThn32_N3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x3f4428 — __ZThn32_N3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3f4428() -> ! {
    todo!("0x3f4428 __ZThn32_N3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x3f44cc — __ZThn36_N3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3f44cc() -> ! {
    todo!("0x3f44cc __ZThn36_N3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x3f44d4 — __ZThn36_N3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3f44d4() -> ! {
    todo!("0x3f44d4 __ZThn36_N3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x3f4578 — __ZN3RBX10Reflection9EventDescINS_17CollectionServiceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(int, int, int, int, int, void *, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::CollectionService,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::CollectionService::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::CollectionService::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection9EventDescINS_17CollectionServiceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_3f4578() -> ! {
    todo!("0x3f4578 RBX::Reflection::EventDesc<RBX::CollectionService,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::CollectionService::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::CollectionService::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x3f46fc — __ZN3RBX10Reflection9EventDescINS_17CollectionServiceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::CollectionService,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::CollectionService::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_17CollectionServiceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED0Ev
pub fn stub_3f46fc() -> ! {
    todo!("0x3f46fc RBX::Reflection::EventDesc<RBX::CollectionService,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::CollectionService::*>::~EventDesc()")
}

// 0x3f47b0 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_17CollectionServiceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::CollectionService,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::CollectionService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi1ENS_17CollectionServiceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
pub fn stub_3f47b0() -> ! {
    todo!("0x3f47b0 RBX::Reflection::EventDescImpl<1,RBX::CollectionService,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::CollectionService::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x3f4904 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_17CollectionServiceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::CollectionService,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::CollectionService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi1ENS_17CollectionServiceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
pub fn stub_3f4904() -> ! {
    todo!("0x3f4904 RBX::Reflection::EventDescImpl<1,RBX::CollectionService,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::CollectionService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x3f4a64 — __ZNK3RBX10Reflection13EventDescBaseINS_17CollectionServiceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::CollectionService,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::CollectionService::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_17CollectionServiceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
pub fn stub_3f4a64() -> ! {
    todo!("0x3f4a64 RBX::Reflection::EventDescBase<RBX::CollectionService,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::CollectionService::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x3f4a78 — __ZN3RBX10Reflection13BoundFuncDescINS_17CollectionServiceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEESsELi1EEC2EMS2_FSB_SsEPKcSH_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::CollectionService,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(std::string),1>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::CollectionService::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_17CollectionServiceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEESsELi1EEC2EMS2_FSB_SsEPKcSH_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_3f4a78() -> ! {
    todo!("0x3f4a78 RBX::Reflection::BoundFuncDesc<RBX::CollectionService,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(std::string),1>::BoundFuncDesc(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::CollectionService::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x3f4bf0 — __ZN3RBX10Reflection13BoundFuncDescINS_17CollectionServiceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEESsELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::CollectionService,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_17CollectionServiceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEESsELi1EE16declareSignatureEPKcNS0_7VariantE
pub fn stub_3f4bf0() -> ! {
    todo!("0x3f4bf0 RBX::Reflection::BoundFuncDesc<RBX::CollectionService,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x3f4c20 — __ZN3RBX10Reflection13BoundFuncDescINS_17CollectionServiceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEESsELi1EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::CollectionService,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(std::string),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_17CollectionServiceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEESsELi1EED0Ev
pub fn stub_3f4c20() -> ! {
    todo!("0x3f4c20 RBX::Reflection::BoundFuncDesc<RBX::CollectionService,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(std::string),1>::~BoundFuncDesc()")
}

// 0x3f4cec — __ZNK3RBX10Reflection13BoundFuncDescINS_17CollectionServiceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEESsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::CollectionService,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_17CollectionServiceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEESsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_3f4cec() -> ! {
    todo!("0x3f4cec RBX::Reflection::BoundFuncDesc<RBX::CollectionService,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x3f4e2c — __ZN3RBX10Reflection11Call1HelperINS_17CollectionServiceEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEESsESsSB_E4callEPS2_SD_RNS0_7VariantERKSs
// type: void __fastcall(int, char *, int, _DWORD *, std::string *)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::CollectionService,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::CollectionService::*)(std::string),std::string,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::call(RBX::CollectionService*,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::CollectionService::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
// was: __ZN3RBX10Reflection11Call1HelperINS_17CollectionServiceEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEESsESsSB_E4callEPS2_SD_RNS0_7VariantERKSs
pub fn stub_3f4e2c() -> ! {
    todo!("0x3f4e2c RBX::Reflection::Call1Helper<RBX::CollectionService,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::CollectionService::*)(std::string),std::string,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>>::call(RBX::CollectionService*,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::CollectionService::*)(std::string),RBX::Reflection::Variant &,std::string const&)")
}

// 0x3f4fac — __GLOBAL__I_a_172
// type: 
#[doc(alias = "global constructor keyed to_a_172")]
// was: __GLOBAL__I_a_172
pub fn stub_3f4fac() -> ! {
    todo!("0x3f4fac global constructor keyed to_a_172")
}

// 0x3f5208 — __ZN3RBX16BoolPropertyVerbC2ERKSsPNS_9DataModelEPKc
// type: char **__fastcall(char **this, const std::string *, RBX::DataModel *, const char *)
#[doc(alias = "RBX::BoolPropertyVerb::BoolPropertyVerb(std::string const&,RBX::DataModel *,char const*)")]
// was: __ZN3RBX16BoolPropertyVerbC2ERKSsPNS_9DataModelEPKc
pub fn stub_3f5208() -> ! {
    todo!("0x3f5208 RBX::BoolPropertyVerb::BoolPropertyVerb(std::string const&,RBX::DataModel *,char const*)")
}

// 0x3f5368 — __ZN3RBX17EditSelectionVerbC2ESsPNS_9DataModelE
// type: RBX::Verb *__fastcall(RBX::Verb *, const std::string *, _DWORD *)
#[doc(alias = "RBX::EditSelectionVerb::EditSelectionVerb(std::string,RBX::DataModel *)")]
// was: __ZN3RBX17EditSelectionVerbC2ESsPNS_9DataModelE
pub fn stub_3f5368() -> ! {
    todo!("0x3f5368 RBX::EditSelectionVerb::EditSelectionVerb(std::string,RBX::DataModel *)")
}

// 0x3f5454 — __ZN3RBX17EditSelectionVerbD2Ev
// type: void __fastcall(RBX::EditSelectionVerb *__hidden this)
#[doc(alias = "RBX::EditSelectionVerb::~EditSelectionVerb()")]
// was: __ZN3RBX17EditSelectionVerbD2Ev
pub fn stub_3f5454() -> ! {
    todo!("0x3f5454 RBX::EditSelectionVerb::~EditSelectionVerb()")
}

// 0x3f5548 — __ZNK3RBX16BoolPropertyVerb9isCheckedEv
// type: bool __fastcall(RBX::BoolPropertyVerb *this)
#[doc(alias = "RBX::BoolPropertyVerb::isChecked(void)const")]
// was: __ZNK3RBX16BoolPropertyVerb9isCheckedEv
pub fn stub_3f5548() -> ! {
    todo!("0x3f5548 RBX::BoolPropertyVerb::isChecked(void)const")
}

// 0x3f559c — __ZN3RBXL15HasTruePropertyEPKcN5boost10shared_ptrINS_8InstanceEEE
// type: bool __fastcall(int, int *)
#[doc(alias = "RBX::HasTrueProperty(char const*,rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBXL15HasTruePropertyEPKcN5boost10shared_ptrINS_8InstanceEEE
pub fn stub_3f559c() -> ! {
    todo!("0x3f559c RBX::HasTrueProperty(char const*,boost::shared_ptr<RBX::Instance>)")
}

// 0x3f569c — __ZN3RBX16BoolPropertyVerb4doItEPNS_10IDataStateE
// type: int __fastcall(int, RBX::DataModel *)
#[doc(alias = "RBX::BoolPropertyVerb::doIt(RBX::IDataState *)")]
// was: __ZN3RBX16BoolPropertyVerb4doItEPNS_10IDataStateE
pub fn stub_3f569c() -> ! {
    todo!("0x3f569c RBX::BoolPropertyVerb::doIt(RBX::IDataState *)")
}

// 0x3f5710 — __ZN3RBXL11requireEditEPNS_9DataModelE
// type: void __fastcall(RBX *this, RBX::DataModel *, bool)
#[doc(alias = "RBX::requireEdit(RBX::DataModel *)")]
// was: __ZN3RBXL11requireEditEPNS_9DataModelE
pub fn stub_3f5710() -> ! {
    todo!("0x3f5710 RBX::requireEdit(RBX::DataModel *)")
}

// 0x3f5868 — __ZN3RBX19CameraCenterCommandC1EPNS_9WorkspaceE
// type: int __fastcall(RBX::CameraCenterCommand *this, RBX::Workspace *)
#[doc(alias = "RBX::CameraCenterCommand::CameraCenterCommand(RBX::Workspace *)")]
// was: __ZN3RBX19CameraCenterCommandC1EPNS_9WorkspaceE
pub fn stub_3f5868() -> ! {
    todo!("0x3f5868 RBX::CameraCenterCommand::CameraCenterCommand(RBX::Workspace *)")
}

// 0x3f586c — __ZN3RBX19CameraCenterCommandC2EPNS_9WorkspaceE
// type: RBX::Verb *__fastcall(RBX::CameraCenterCommand *this, RBX::Workspace *)
#[doc(alias = "RBX::CameraCenterCommand::CameraCenterCommand(RBX::Workspace *)")]
// was: __ZN3RBX19CameraCenterCommandC2EPNS_9WorkspaceE
pub fn stub_3f586c() -> ! {
    todo!("0x3f586c RBX::CameraCenterCommand::CameraCenterCommand(RBX::Workspace *)")
}

// 0x3f5a4c — __ZN3RBX10CameraVerbC2ESsPNS_9WorkspaceE
// type: RBX::Verb *__fastcall(RBX::Verb *, const std::string *, int)
#[doc(alias = "RBX::CameraVerb::CameraVerb(std::string,RBX::Workspace *)")]
// was: __ZN3RBX10CameraVerbC2ESsPNS_9WorkspaceE
pub fn stub_3f5a4c() -> ! {
    todo!("0x3f5a4c RBX::CameraVerb::CameraVerb(std::string,RBX::Workspace *)")
}

// 0x3f5a80 — __ZNK3RBX19CameraCenterCommand9isEnabledEv
// type: bool __fastcall(RBX::CameraCenterCommand *this)
#[doc(alias = "RBX::CameraCenterCommand::isEnabled(void)const")]
// was: __ZNK3RBX19CameraCenterCommand9isEnabledEv
pub fn stub_3f5a80() -> ! {
    todo!("0x3f5a80 RBX::CameraCenterCommand::isEnabled(void)const")
}

// 0x3f5bcc — __ZN3RBX19CameraCenterCommand4doItEPNS_10IDataStateE
// type: void __fastcall(_DWORD *, void (__fastcall ***)(_DWORD, int), int, const void *, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::CameraCenterCommand::doIt(RBX::IDataState *)")]
// was: __ZN3RBX19CameraCenterCommand4doItEPNS_10IDataStateE
pub fn stub_3f5bcc() -> ! {
    todo!("0x3f5bcc RBX::CameraCenterCommand::doIt(RBX::IDataState *)")
}

// 0x3f5f00 — __ZN3RBX10CameraVerb4doItEPNS_10IDataStateE
// type: int __fastcall(int, int (__fastcall ***)(_DWORD, int))
#[doc(alias = "RBX::CameraVerb::doIt(RBX::IDataState *)")]
// was: __ZN3RBX10CameraVerb4doItEPNS_10IDataStateE
pub fn stub_3f5f00() -> ! {
    todo!("0x3f5f00 RBX::CameraVerb::doIt(RBX::IDataState *)")
}

// 0x3f5f64 — __ZN3RBX16SelectAllCommand4doItEPNS_10IDataStateE
// type: int __fastcall(int, RBX::DataModel *, bool, const void *)
#[doc(alias = "RBX::SelectAllCommand::doIt(RBX::IDataState *)")]
// was: __ZN3RBX16SelectAllCommand4doItEPNS_10IDataStateE
pub fn stub_3f5f64() -> ! {
    todo!("0x3f5f64 RBX::SelectAllCommand::doIt(RBX::IDataState *)")
}

// 0x3f5fa0 — __ZN3RBX19AllCanSelectCommand4doItEPNS_10IDataStateE
// type: int __fastcall(int, RBX::DataModel *, bool)
#[doc(alias = "RBX::AllCanSelectCommand::doIt(RBX::IDataState *)")]
// was: __ZN3RBX19AllCanSelectCommand4doItEPNS_10IDataStateE
pub fn stub_3f5fa0() -> ! {
    todo!("0x3f5fa0 RBX::AllCanSelectCommand::doIt(RBX::IDataState *)")
}

// 0x3f5fd4 — __ZN3RBX19CanNotSelectCommand4doItEPNS_10IDataStateE
// type: int __fastcall(int, RBX::DataModel *, bool)
#[doc(alias = "RBX::CanNotSelectCommand::doIt(RBX::IDataState *)")]
// was: __ZN3RBX19CanNotSelectCommand4doItEPNS_10IDataStateE
pub fn stub_3f5fd4() -> ! {
    todo!("0x3f5fd4 RBX::CanNotSelectCommand::doIt(RBX::IDataState *)")
}

// 0x3f604c — __ZN3RBXL15SetCanNotSelectEN5boost10shared_ptrINS_8InstanceEEE
// type: int __fastcall(RBX::PartInstance **, int, bool)
#[doc(alias = "RBX::SetCanNotSelect(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBXL15SetCanNotSelectEN5boost10shared_ptrINS_8InstanceEEE
pub fn stub_3f604c() -> ! {
    todo!("0x3f604c RBX::SetCanNotSelect(boost::shared_ptr<RBX::Instance>)")
}

// 0x3f6054 — __ZN3RBX18FirstPersonCommandC1EPNS_9DataModelE
// type: int __fastcall(RBX::FirstPersonCommand *this, RBX::DataModel *)
#[doc(alias = "RBX::FirstPersonCommand::FirstPersonCommand(RBX::DataModel *)")]
// was: __ZN3RBX18FirstPersonCommandC1EPNS_9DataModelE
pub fn stub_3f6054() -> ! {
    todo!("0x3f6054 RBX::FirstPersonCommand::FirstPersonCommand(RBX::DataModel *)")
}

// 0x3f6058 — __ZN3RBX18FirstPersonCommandC2EPNS_9DataModelE
// type: RBX::FirstPersonCommand *__fastcall(RBX::FirstPersonCommand *this, RBX::DataModel *)
#[doc(alias = "RBX::FirstPersonCommand::FirstPersonCommand(RBX::DataModel *)")]
// was: __ZN3RBX18FirstPersonCommandC2EPNS_9DataModelE
pub fn stub_3f6058() -> ! {
    todo!("0x3f6058 RBX::FirstPersonCommand::FirstPersonCommand(RBX::DataModel *)")
}

// 0x3f61b0 — __ZNK3RBX18FirstPersonCommand9isEnabledEv
// type: int __fastcall(RBX::Humanoid **this, RBX::Instance *)
#[doc(alias = "RBX::FirstPersonCommand::isEnabled(void)const")]
// was: __ZNK3RBX18FirstPersonCommand9isEnabledEv
pub fn stub_3f61b0() -> ! {
    todo!("0x3f61b0 RBX::FirstPersonCommand::isEnabled(void)const")
}

// 0x3f61cc — __ZN3RBX14ToggleViewModeC1EPNS_9DataModelE
// type: int __fastcall(RBX::ToggleViewMode *this, RBX::DataModel *)
#[doc(alias = "RBX::ToggleViewMode::ToggleViewMode(RBX::DataModel *)")]
// was: __ZN3RBX14ToggleViewModeC1EPNS_9DataModelE
pub fn stub_3f61cc() -> ! {
    todo!("0x3f61cc RBX::ToggleViewMode::ToggleViewMode(RBX::DataModel *)")
}

// 0x3f61d0 — __ZN3RBX14ToggleViewModeC2EPNS_9DataModelE
// type: RBX::ToggleViewMode *__fastcall(RBX::ToggleViewMode *this, RBX::DataModel *)
#[doc(alias = "RBX::ToggleViewMode::ToggleViewMode(RBX::DataModel *)")]
// was: __ZN3RBX14ToggleViewModeC2EPNS_9DataModelE
pub fn stub_3f61d0() -> ! {
    todo!("0x3f61d0 RBX::ToggleViewMode::ToggleViewMode(RBX::DataModel *)")
}

// 0x3f6328 — __ZNK3RBX14ToggleViewMode9isCheckedEv
// type: int __fastcall(RBX::ToggleViewMode *this)
#[doc(alias = "RBX::ToggleViewMode::isChecked(void)const")]
// was: __ZNK3RBX14ToggleViewMode9isCheckedEv
pub fn stub_3f6328() -> ! {
    todo!("0x3f6328 RBX::ToggleViewMode::isChecked(void)const")
}

// 0x3f632c — __ZNK3RBX14ToggleViewMode9isEnabledEv
// type: bool __fastcall(RBX::ToggleViewMode *this)
#[doc(alias = "RBX::ToggleViewMode::isEnabled(void)const")]
// was: __ZNK3RBX14ToggleViewMode9isEnabledEv
pub fn stub_3f632c() -> ! {
    todo!("0x3f632c RBX::ToggleViewMode::isEnabled(void)const")
}

// 0x3f6348 — __ZNK3RBX14ToggleViewMode10isSelectedEv
// type: int __fastcall(RBX::ToggleViewMode *this)
#[doc(alias = "RBX::ToggleViewMode::isSelected(void)const")]
// was: __ZNK3RBX14ToggleViewMode10isSelectedEv
pub fn stub_3f6348() -> ! {
    todo!("0x3f6348 RBX::ToggleViewMode::isSelected(void)const")
}

// 0x3f634c — __ZN3RBX14ToggleViewMode4doItEPNS_10IDataStateE
// type: void __fastcall(int, int, int, const void *)
#[doc(alias = "RBX::ToggleViewMode::doIt(RBX::IDataState *)")]
// was: __ZN3RBX14ToggleViewMode4doItEPNS_10IDataStateE
pub fn stub_3f634c() -> ! {
    todo!("0x3f634c RBX::ToggleViewMode::doIt(RBX::IDataState *)")
}

// 0x3f637c — __ZN3RBX12StatsCommandC1EPNS_9DataModelE
// type: int __fastcall(RBX::StatsCommand *this, RBX::DataModel *)
#[doc(alias = "RBX::StatsCommand::StatsCommand(RBX::DataModel *)")]
// was: __ZN3RBX12StatsCommandC1EPNS_9DataModelE
pub fn stub_3f637c() -> ! {
    todo!("0x3f637c RBX::StatsCommand::StatsCommand(RBX::DataModel *)")
}

// 0x3f6380 — __ZN3RBX12StatsCommandC2EPNS_9DataModelE
// type: RBX::StatsCommand *__fastcall(RBX::StatsCommand *this, RBX::DataModel *)
#[doc(alias = "RBX::StatsCommand::StatsCommand(RBX::DataModel *)")]
// was: __ZN3RBX12StatsCommandC2EPNS_9DataModelE
pub fn stub_3f6380() -> ! {
    todo!("0x3f6380 RBX::StatsCommand::StatsCommand(RBX::DataModel *)")
}

// 0x3f64d8 — __ZN3RBX12StatsCommand4doItEPNS_10IDataStateE
// type: void __fastcall(int, int, int, const void *)
#[doc(alias = "RBX::StatsCommand::doIt(RBX::IDataState *)")]
// was: __ZN3RBX12StatsCommand4doItEPNS_10IDataStateE
pub fn stub_3f64d8() -> ! {
    todo!("0x3f64d8 RBX::StatsCommand::doIt(RBX::IDataState *)")
}

// 0x3f6784 — __ZNK3RBX12StatsCommand9isEnabledEv
// type: bool __fastcall(RBX::StatsCommand *this)
#[doc(alias = "RBX::StatsCommand::isEnabled(void)const")]
// was: __ZNK3RBX12StatsCommand9isEnabledEv
pub fn stub_3f6784() -> ! {
    todo!("0x3f6784 RBX::StatsCommand::isEnabled(void)const")
}

// 0x3f6900 — __ZNK3RBX12StatsCommand9isCheckedEv
// type: int __fastcall(RBX::StatsCommand *this)
#[doc(alias = "RBX::StatsCommand::isChecked(void)const")]
// was: __ZNK3RBX12StatsCommand9isCheckedEv
pub fn stub_3f6900() -> ! {
    todo!("0x3f6900 RBX::StatsCommand::isChecked(void)const")
}

// 0x3f71b8 — __ZN3RBX19SummaryStatsCommandC1EPNS_9DataModelE
// type: int __fastcall(RBX::SummaryStatsCommand *this, RBX::DataModel *)
#[doc(alias = "RBX::SummaryStatsCommand::SummaryStatsCommand(RBX::DataModel *)")]
// was: __ZN3RBX19SummaryStatsCommandC1EPNS_9DataModelE
pub fn stub_3f71b8() -> ! {
    todo!("0x3f71b8 RBX::SummaryStatsCommand::SummaryStatsCommand(RBX::DataModel *)")
}

// 0x3f71bc — __ZN3RBX19SummaryStatsCommandC2EPNS_9DataModelE
// type: RBX::SummaryStatsCommand *__fastcall(RBX::SummaryStatsCommand *this, RBX::DataModel *)
#[doc(alias = "RBX::SummaryStatsCommand::SummaryStatsCommand(RBX::DataModel *)")]
// was: __ZN3RBX19SummaryStatsCommandC2EPNS_9DataModelE
pub fn stub_3f71bc() -> ! {
    todo!("0x3f71bc RBX::SummaryStatsCommand::SummaryStatsCommand(RBX::DataModel *)")
}

// 0x3f7314 — __ZN3RBX19SummaryStatsCommand4doItEPNS_10IDataStateE
// type: void __fastcall(int, int, int, const void *)
#[doc(alias = "RBX::SummaryStatsCommand::doIt(RBX::IDataState *)")]
// was: __ZN3RBX19SummaryStatsCommand4doItEPNS_10IDataStateE
pub fn stub_3f7314() -> ! {
    todo!("0x3f7314 RBX::SummaryStatsCommand::doIt(RBX::IDataState *)")
}

// 0x3f74cc — __ZNK3RBX19SummaryStatsCommand9isEnabledEv
// type: bool __fastcall(RBX::SummaryStatsCommand *this)
#[doc(alias = "RBX::SummaryStatsCommand::isEnabled(void)const")]
// was: __ZNK3RBX19SummaryStatsCommand9isEnabledEv
pub fn stub_3f74cc() -> ! {
    todo!("0x3f74cc RBX::SummaryStatsCommand::isEnabled(void)const")
}

// 0x3f7648 — __ZNK3RBX19SummaryStatsCommand9isCheckedEv
// type: int __fastcall(RBX::SummaryStatsCommand *this)
#[doc(alias = "RBX::SummaryStatsCommand::isChecked(void)const")]
// was: __ZNK3RBX19SummaryStatsCommand9isCheckedEv
pub fn stub_3f7648() -> ! {
    todo!("0x3f7648 RBX::SummaryStatsCommand::isChecked(void)const")
}

// 0x3f77d4 — __ZN3RBX18CustomStatsCommandC1EPNS_9DataModelE
// type: int __fastcall(RBX::CustomStatsCommand *this, RBX::DataModel *)
#[doc(alias = "RBX::CustomStatsCommand::CustomStatsCommand(RBX::DataModel *)")]
// was: __ZN3RBX18CustomStatsCommandC1EPNS_9DataModelE
pub fn stub_3f77d4() -> ! {
    todo!("0x3f77d4 RBX::CustomStatsCommand::CustomStatsCommand(RBX::DataModel *)")
}

// 0x3f77d8 — __ZN3RBX18CustomStatsCommandC2EPNS_9DataModelE
// type: RBX::CustomStatsCommand *__fastcall(RBX::CustomStatsCommand *this, RBX::DataModel *)
#[doc(alias = "RBX::CustomStatsCommand::CustomStatsCommand(RBX::DataModel *)")]
// was: __ZN3RBX18CustomStatsCommandC2EPNS_9DataModelE
pub fn stub_3f77d8() -> ! {
    todo!("0x3f77d8 RBX::CustomStatsCommand::CustomStatsCommand(RBX::DataModel *)")
}

// 0x3f7930 — __ZN3RBX18CustomStatsCommand4doItEPNS_10IDataStateE
// type: void __fastcall(int, int, int, const void *)
#[doc(alias = "RBX::CustomStatsCommand::doIt(RBX::IDataState *)")]
// was: __ZN3RBX18CustomStatsCommand4doItEPNS_10IDataStateE
pub fn stub_3f7930() -> ! {
    todo!("0x3f7930 RBX::CustomStatsCommand::doIt(RBX::IDataState *)")
}

// 0x3f7ae8 — __ZNK3RBX18CustomStatsCommand9isEnabledEv
// type: bool __fastcall(RBX::CustomStatsCommand *this)
#[doc(alias = "RBX::CustomStatsCommand::isEnabled(void)const")]
// was: __ZNK3RBX18CustomStatsCommand9isEnabledEv
pub fn stub_3f7ae8() -> ! {
    todo!("0x3f7ae8 RBX::CustomStatsCommand::isEnabled(void)const")
}

// 0x3f7c64 — __ZNK3RBX18CustomStatsCommand9isCheckedEv
// type: int __fastcall(RBX::CustomStatsCommand *this)
#[doc(alias = "RBX::CustomStatsCommand::isChecked(void)const")]
// was: __ZNK3RBX18CustomStatsCommand9isCheckedEv
pub fn stub_3f7c64() -> ! {
    todo!("0x3f7c64 RBX::CustomStatsCommand::isChecked(void)const")
}

// 0x3f7df0 — __ZN3RBX19NetworkStatsCommandC1EPNS_9DataModelE
// type: int __fastcall(RBX::NetworkStatsCommand *this, RBX::DataModel *)
#[doc(alias = "RBX::NetworkStatsCommand::NetworkStatsCommand(RBX::DataModel *)")]
// was: __ZN3RBX19NetworkStatsCommandC1EPNS_9DataModelE
pub fn stub_3f7df0() -> ! {
    todo!("0x3f7df0 RBX::NetworkStatsCommand::NetworkStatsCommand(RBX::DataModel *)")
}

// 0x3f7df4 — __ZN3RBX19NetworkStatsCommandC2EPNS_9DataModelE
// type: RBX::Verb *__fastcall(RBX::NetworkStatsCommand *this, RBX::DataModel *)
#[doc(alias = "RBX::NetworkStatsCommand::NetworkStatsCommand(RBX::DataModel *)")]
// was: __ZN3RBX19NetworkStatsCommandC2EPNS_9DataModelE
pub fn stub_3f7df4() -> ! {
    todo!("0x3f7df4 RBX::NetworkStatsCommand::NetworkStatsCommand(RBX::DataModel *)")
}

// 0x3f7f80 — __ZN3RBX19NetworkStatsCommand4doItEPNS_10IDataStateE
// type: void __fastcall(int, int, int, const void *)
#[doc(alias = "RBX::NetworkStatsCommand::doIt(RBX::IDataState *)")]
// was: __ZN3RBX19NetworkStatsCommand4doItEPNS_10IDataStateE
pub fn stub_3f7f80() -> ! {
    todo!("0x3f7f80 RBX::NetworkStatsCommand::doIt(RBX::IDataState *)")
}

// 0x3f8268 — __ZNK3RBX19NetworkStatsCommand9isEnabledEv
// type: bool __fastcall(RBX::NetworkStatsCommand *this)
#[doc(alias = "RBX::NetworkStatsCommand::isEnabled(void)const")]
// was: __ZNK3RBX19NetworkStatsCommand9isEnabledEv
pub fn stub_3f8268() -> ! {
    todo!("0x3f8268 RBX::NetworkStatsCommand::isEnabled(void)const")
}

// 0x3f83e4 — __ZNK3RBX19NetworkStatsCommand9isCheckedEv
// type: int __fastcall(RBX::NetworkStatsCommand *this)
#[doc(alias = "RBX::NetworkStatsCommand::isChecked(void)const")]
// was: __ZNK3RBX19NetworkStatsCommand9isCheckedEv
pub fn stub_3f83e4() -> ! {
    todo!("0x3f83e4 RBX::NetworkStatsCommand::isChecked(void)const")
}

// 0x3f8570 — __ZN3RBX19PhysicsStatsCommandC1EPNS_9DataModelE
// type: int __fastcall(RBX::PhysicsStatsCommand *this, RBX::DataModel *)
#[doc(alias = "RBX::PhysicsStatsCommand::PhysicsStatsCommand(RBX::DataModel *)")]
// was: __ZN3RBX19PhysicsStatsCommandC1EPNS_9DataModelE
pub fn stub_3f8570() -> ! {
    todo!("0x3f8570 RBX::PhysicsStatsCommand::PhysicsStatsCommand(RBX::DataModel *)")
}

// 0x3f8574 — __ZN3RBX19PhysicsStatsCommandC2EPNS_9DataModelE
// type: RBX::PhysicsStatsCommand *__fastcall(RBX::PhysicsStatsCommand *this, RBX::DataModel *)
#[doc(alias = "RBX::PhysicsStatsCommand::PhysicsStatsCommand(RBX::DataModel *)")]
// was: __ZN3RBX19PhysicsStatsCommandC2EPNS_9DataModelE
pub fn stub_3f8574() -> ! {
    todo!("0x3f8574 RBX::PhysicsStatsCommand::PhysicsStatsCommand(RBX::DataModel *)")
}

// 0x3f86cc — __ZN3RBX19PhysicsStatsCommand4doItEPNS_10IDataStateE
// type: void __fastcall(int, int, int, const void *)
#[doc(alias = "RBX::PhysicsStatsCommand::doIt(RBX::IDataState *)")]
// was: __ZN3RBX19PhysicsStatsCommand4doItEPNS_10IDataStateE
pub fn stub_3f86cc() -> ! {
    todo!("0x3f86cc RBX::PhysicsStatsCommand::doIt(RBX::IDataState *)")
}

// 0x3f8988 — __ZNK3RBX19PhysicsStatsCommand9isEnabledEv
// type: bool __fastcall(RBX::PhysicsStatsCommand *this)
#[doc(alias = "RBX::PhysicsStatsCommand::isEnabled(void)const")]
// was: __ZNK3RBX19PhysicsStatsCommand9isEnabledEv
pub fn stub_3f8988() -> ! {
    todo!("0x3f8988 RBX::PhysicsStatsCommand::isEnabled(void)const")
}

// 0x3f8be8 — __ZNK3RBX19PhysicsStatsCommand9isCheckedEv
// type: bool __fastcall(RBX::PhysicsStatsCommand *this)
#[doc(alias = "RBX::PhysicsStatsCommand::isChecked(void)const")]
// was: __ZNK3RBX19PhysicsStatsCommand9isCheckedEv
pub fn stub_3f8be8() -> ! {
    todo!("0x3f8be8 RBX::PhysicsStatsCommand::isChecked(void)const")
}

// 0x3f8e6c — __ZN3RBX18EngineStatsCommandC1EPNS_9DataModelE
// type: int __fastcall(RBX::EngineStatsCommand *this, RBX::DataModel *)
#[doc(alias = "RBX::EngineStatsCommand::EngineStatsCommand(RBX::DataModel *)")]
// was: __ZN3RBX18EngineStatsCommandC1EPNS_9DataModelE
pub fn stub_3f8e6c() -> ! {
    todo!("0x3f8e6c RBX::EngineStatsCommand::EngineStatsCommand(RBX::DataModel *)")
}

// 0x3f8e70 — __ZN3RBX18EngineStatsCommandC2EPNS_9DataModelE
// type: RBX::EngineStatsCommand *__fastcall(RBX::EngineStatsCommand *this, RBX::DataModel *)
#[doc(alias = "RBX::EngineStatsCommand::EngineStatsCommand(RBX::DataModel *)")]
// was: __ZN3RBX18EngineStatsCommandC2EPNS_9DataModelE
pub fn stub_3f8e70() -> ! {
    todo!("0x3f8e70 RBX::EngineStatsCommand::EngineStatsCommand(RBX::DataModel *)")
}

// 0x3f8fc8 — __ZN3RBX18EngineStatsCommand4doItEPNS_10IDataStateE
// type: int __fastcall(int, int, int, const void *)
#[doc(alias = "RBX::EngineStatsCommand::doIt(RBX::IDataState *)")]
// was: __ZN3RBX18EngineStatsCommand4doItEPNS_10IDataStateE
pub fn stub_3f8fc8() -> ! {
    todo!("0x3f8fc8 RBX::EngineStatsCommand::doIt(RBX::IDataState *)")
}

// 0x3f9004 — __ZN3RBX11JoinCommandC1EPNS_9DataModelE
// type: int __fastcall(RBX::JoinCommand *this, RBX::DataModel *)
#[doc(alias = "RBX::JoinCommand::JoinCommand(RBX::DataModel *)")]
// was: __ZN3RBX11JoinCommandC1EPNS_9DataModelE
pub fn stub_3f9004() -> ! {
    todo!("0x3f9004 RBX::JoinCommand::JoinCommand(RBX::DataModel *)")
}

// 0x3f9008 — __ZN3RBX11JoinCommandC2EPNS_9DataModelE
// type: RBX::JoinCommand *__fastcall(RBX::JoinCommand *this, RBX::DataModel *)
#[doc(alias = "RBX::JoinCommand::JoinCommand(RBX::DataModel *)")]
// was: __ZN3RBX11JoinCommandC2EPNS_9DataModelE
pub fn stub_3f9008() -> ! {
    todo!("0x3f9008 RBX::JoinCommand::JoinCommand(RBX::DataModel *)")
}

// 0x3f9160 — __ZNK3RBX11JoinCommand9isEnabledEv
// type: bool __fastcall(RBX::JoinCommand *this, int, int, int)
#[doc(alias = "RBX::JoinCommand::isEnabled(void)const")]
// was: __ZNK3RBX11JoinCommand9isEnabledEv
pub fn stub_3f9160() -> ! {
    todo!("0x3f9160 RBX::JoinCommand::isEnabled(void)const")
}

// 0x3f9294 — __ZN3RBX11JoinCommand4doItEPNS_10IDataStateE
// type: void __fastcall(int, RBX::DataModel *, bool, const void *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::JoinCommand::doIt(RBX::IDataState *)")]
// was: __ZN3RBX11JoinCommand4doItEPNS_10IDataStateE
pub fn stub_3f9294() -> ! {
    todo!("0x3f9294 RBX::JoinCommand::doIt(RBX::IDataState *)")
}

// 0x3f93e4 — __ZN3RBX12RunStateVerbC2ESsPNS_9DataModelE
// type: RBX::Verb *__fastcall(RBX::Verb *, const std::string *, int)
#[doc(alias = "RBX::RunStateVerb::RunStateVerb(std::string,RBX::DataModel *)")]
// was: __ZN3RBX12RunStateVerbC2ESsPNS_9DataModelE
pub fn stub_3f93e4() -> ! {
    todo!("0x3f93e4 RBX::RunStateVerb::RunStateVerb(std::string,RBX::DataModel *)")
}

// 0x3f9418 — __ZN3RBX12RunStateVerbD0Ev
// type: void __fastcall(RBX::RunStateVerb *__hidden this)
#[doc(alias = "RBX::RunStateVerb::~RunStateVerb()")]
// was: __ZN3RBX12RunStateVerbD0Ev
pub fn stub_3f9418() -> ! {
    todo!("0x3f9418 RBX::RunStateVerb::~RunStateVerb()")
}

// 0x3f94b8 — __ZN3RBX12RunStateVerbD1Ev
// type: void __fastcall(RBX::RunStateVerb *__hidden this)
#[doc(alias = "RBX::RunStateVerb::~RunStateVerb()")]
// was: __ZN3RBX12RunStateVerbD1Ev
pub fn stub_3f94b8() -> ! {
    todo!("0x3f94b8 RBX::RunStateVerb::~RunStateVerb()")
}

// 0x3f94bc — __ZN3RBX12RunStateVerbD2Ev
// type: void __fastcall(RBX::RunStateVerb *__hidden this)
#[doc(alias = "RBX::RunStateVerb::~RunStateVerb()")]
// was: __ZN3RBX12RunStateVerbD2Ev
pub fn stub_3f94bc() -> ! {
    todo!("0x3f94bc RBX::RunStateVerb::~RunStateVerb()")
}

// 0x3f9588 — __ZNK3RBX10RunCommand9isEnabledEv
// type: int __fastcall(RBX::RunCommand *this)
#[doc(alias = "RBX::RunCommand::isEnabled(void)const")]
// was: __ZNK3RBX10RunCommand9isEnabledEv
pub fn stub_3f9588() -> ! {
    todo!("0x3f9588 RBX::RunCommand::isEnabled(void)const")
}

// 0x3f95b0 — __ZN3RBX10RunCommand4doItEPNS_10IDataStateE
// type: int __fastcall(int, RBX::DataModel *, bool, const void *)
#[doc(alias = "RBX::RunCommand::doIt(RBX::IDataState *)")]
// was: __ZN3RBX10RunCommand4doItEPNS_10IDataStateE
pub fn stub_3f95b0() -> ! {
    todo!("0x3f95b0 RBX::RunCommand::doIt(RBX::IDataState *)")
}

// 0x3f9644 — __ZNK3RBX11StopCommand9isEnabledEv
// type: int __fastcall(RBX::StopCommand *this)
#[doc(alias = "RBX::StopCommand::isEnabled(void)const")]
// was: __ZNK3RBX11StopCommand9isEnabledEv
pub fn stub_3f9644() -> ! {
    todo!("0x3f9644 RBX::StopCommand::isEnabled(void)const")
}

// 0x3f9658 — __ZN3RBX11StopCommand4doItEPNS_10IDataStateE
// type: int __fastcall(int, RBX::DataModel *, bool, const void *)
#[doc(alias = "RBX::StopCommand::doIt(RBX::IDataState *)")]
// was: __ZN3RBX11StopCommand4doItEPNS_10IDataStateE
pub fn stub_3f9658() -> ! {
    todo!("0x3f9658 RBX::StopCommand::doIt(RBX::IDataState *)")
}

// 0x3f96ec — __ZNK3RBX12ResetCommand9isEnabledEv
// type: bool __fastcall(RBX::Network::Players **this)
#[doc(alias = "RBX::ResetCommand::isEnabled(void)const")]
// was: __ZNK3RBX12ResetCommand9isEnabledEv
pub fn stub_3f96ec() -> ! {
    todo!("0x3f96ec RBX::ResetCommand::isEnabled(void)const")
}

// 0x3f9714 — __ZN3RBX12ResetCommand4doItEPNS_10IDataStateE
// type: int __fastcall(int, RBX::DataModel *, bool, const void *)
#[doc(alias = "RBX::ResetCommand::doIt(RBX::IDataState *)")]
// was: __ZN3RBX12ResetCommand4doItEPNS_10IDataStateE
pub fn stub_3f9714() -> ! {
    todo!("0x3f9714 RBX::ResetCommand::doIt(RBX::IDataState *)")
}

// 0x3f97a8 — __ZN3RBX17EditSelectionVerbD0Ev
// type: void __fastcall(RBX::EditSelectionVerb *__hidden this)
#[doc(alias = "RBX::EditSelectionVerb::~EditSelectionVerb()")]
// was: __ZN3RBX17EditSelectionVerbD0Ev
pub fn stub_3f97a8() -> ! {
    todo!("0x3f97a8 RBX::EditSelectionVerb::~EditSelectionVerb()")
}
