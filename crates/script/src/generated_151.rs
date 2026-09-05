// Auto-generated skeletons for rbx-script — global filler EA-sorted asc continuation
// Filter: Script|Lua|lua|Yield (5401 filtered, all stubbed) — global EA-sorted asc filler
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +150 stubs | range 0x310b80..0x3234d4 | global filler EA-sorted asc after 0x3107c8 | rbx_core::SharedPtr not boost | max_stub was 0xf6fb4c (global max)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "__ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS1_S3_NS_8weak_ptrIN3RBX13ContentFilterEEESsENS6_5list4INS_3argILi1EEENSF_ILi2EEENS6_5valueISB_EENSI_ISsEEEEEEEEvT_")]
pub fn stub_0x310b80(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8weak_ptrIN3RBX13ContentFilterEEESsENS3_5list4INS_3argILi1EEENSF_ILi2EEENS3_5valueISB_EENSI_ISsEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE")]
pub fn stub_0x310d54(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8weak_ptrIN3RBX13ContentFilterEEESsENS3_5list4INS_3argILi1EEENSF_ILi2EEENS3_5valueISB_EENSI_ISsEEEEEEvS5_S7_E6invokeERNS1_15function_bufferES5_S7_")]
pub fn stub_0x310d70(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS3_S5_NS_8weak_ptrIN3RBX13ContentFilterEEESsENS8_5list4INS_3argILi1EEENSH_ILi2EEENS8_5valueISD_EENSK_ISsEEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_0x310d90(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS3_S5_NS_8weak_ptrIN3RBX13ContentFilterEEESsENS8_5list4INS_3argILi1EEENSH_ILi2EEENS8_5valueISD_EENSK_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_0x310f54(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvPFvS3_S5_NS_8weak_ptrIN3RBX13ContentFilterEEESsENS8_5list4INS_3argILi1EEENSH_ILi2EEENS8_5valueISD_EENSK_ISsEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_0x311114(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "__ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX13ContentFilterEEEEENS5_ISsEEEclIPFvPSsPSt9exceptionS9_SsENS0_5list2IRSE_RSG_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0x311258(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x311258: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8weak_ptrIN3RBX13ContentFilterEEESsENS3_5list4INS_3argILi1EEENSF_ILi2EEENS3_5valueISB_EENSI_ISsEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_0x31140c(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "__ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX13ContentFilterEEEEENS5_ISsEEEC2ES3_S4_SA_SB_")]
pub fn stub_0x3115e8() -> crate::slot::BindPiece {
// boost::bind fragment (list4) composing a host BoundCall.
crate::slot::BindPiece::new("list4")
}

#[doc(alias = "__ZN5boost3_bi8storage4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX13ContentFilterEEEEENS5_ISsEEEC2ES3_S4_SA_SB_")]
pub fn stub_0x311794() -> crate::slot::BindPiece {
// boost::bind fragment (storage4) composing a host BoundCall.
crate::slot::BindPiece::new("storage4")
}

#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ContentFilterEEESsENS6_5list2INS6_5valueISA_EENSE_ISsEEEEEEEEvT_")]
pub fn stub_0x311c8c(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ContentFilterEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE")]
pub fn stub_0x311e60(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ContentFilterEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEvPNS6_9DataModelEE6invokeERNS1_15function_bufferESI_")]
pub fn stub_0x311e7c(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ContentFilterEEESsENS8_5list2INS8_5valueISC_EENSG_ISsEEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_0x311e98(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ContentFilterEEESsENS8_5list2INS8_5valueISC_EENSG_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_0x31205c(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ContentFilterEEESsENS8_5list2INS8_5valueISC_EENSG_ISsEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_0x31221c(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX13ContentFilterEEEEENS2_ISsEEEclIPFvS6_SsENS0_5list1IRPNS4_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0x312360(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x312360: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ContentFilterEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_0x312508(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX13ContentFilterEEEEENS2_ISsEEEC2ES7_S8_")]
pub fn stub_0x3126e4() -> crate::slot::BindPiece {
// boost::bind fragment (list2) composing a host BoundCall.
crate::slot::BindPiece::new("list2")
}

#[doc(alias = "__ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE4findERKSs")]
pub fn stub_0x312a54(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE4findERS1_")]
pub fn stub_0x312aa4(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_E")]
pub fn stub_0x312af4(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13ContentFilterELZNS_14sContentFilterEENS_17NonFactoryProductINS_8InstanceELZNS_14sContentFilterEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x312b1c(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13ContentFilterELZNS_14sContentFilterEENS_17NonFactoryProductINS_8InstanceELZNS_14sContentFilterEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x312b20(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13ContentFilterELZNS_14sContentFilterEENS_17NonFactoryProductINS_8InstanceELZNS_14sContentFilterEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x312bc0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13ContentFilterELZNS_14sContentFilterEENS_17NonFactoryProductINS_8InstanceELZNS_14sContentFilterEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x312bc8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13ContentFilterELZNS_14sContentFilterEENS_17NonFactoryProductINS_8InstanceELZNS_14sContentFilterEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x312c6c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13ContentFilterELZNS_14sContentFilterEENS_17NonFactoryProductINS_8InstanceELZNS_14sContentFilterEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x312c74(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13ContentFilterEFviiELi2EEC2EMS2_FviiEPKcS8_S8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0x312d18() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ContentFilter", "void", 2)
}

#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13ContentFilterEFviiELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_")]
pub fn stub_0x312ee0() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ContentFilter", "void", 2)
}

#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13ContentFilterEFviiELi2EED0Ev")]
pub fn stub_0x312f2c(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_13ContentFilterEFviiELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0x31300c() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ContentFilter", "void", 2)
}

#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13ContentFilterEFvSsELi1EEC2EMS2_FvSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0x313060() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ContentFilter", "void", 1)
}

#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13ContentFilterEFvSsELi1EE16declareSignatureEPKcNS0_7VariantE")]
pub fn stub_0x3131d8() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ContentFilter", "void", 1)
}

#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13ContentFilterEFvSsELi1EED0Ev")]
pub fn stub_0x313208(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_13ContentFilterEFvSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0x3132d4() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ContentFilter", "void", 1)
}

#[doc(alias = "__ZN3RBX10Reflection11Call1HelperINS_13ContentFilterEMS2_FvSsESsvE4callEPS2_S4_RNS0_7VariantERKSs")]
pub fn stub_0x313410(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Call1Helper<RBX::ContentFilter, void (RBX::ContentFilter::*)(std::string)~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ContentFilterEEESsbENS6_5list3INS6_5valueISA_EENSE_ISsEENSE_IbEEEEEEEEvT_")]
pub fn stub_0x313f14(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ContentFilterEEESsbENS3_5list3INS3_5valueIS8_EENSC_ISsEENSC_IbEEEEEEvPNS6_9DataModelEE6invokeERNS1_15function_bufferESJ_")]
pub fn stub_0x31410c(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ContentFilterEEESsbENS8_5list3INS8_5valueISC_EENSG_ISsEENSG_IbEEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_0x314128(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ContentFilterEEESsbENS8_5list3INS8_5valueISC_EENSG_ISsEENSG_IbEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_0x3142f4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ContentFilterEEESsbENS8_5list3INS8_5valueISC_EENSG_ISsEENSG_IbEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_0x3144bc(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX13ContentFilterEEEEENS2_ISsEENS2_IbEEEclIPFvS6_SsbENS0_5list1IRPNS4_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0x314604(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x314604: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
pub fn stub_0x314a10(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "__GLOBAL__I_a_110")]
pub fn stub_0x314a40() -> crate::slot::PortedFn {
// IDA 0x314a40: __GLOBAL__I_a_110.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x314a40, "__GLOBAL__I_a_110")
}

#[doc(alias = "__ZN3RBXltERKNS_9ContentIdES2_")]
pub fn stub_0x314c84() -> crate::slot::PortedFn {
// IDA 0x314c84: RBX::operator<(RBX::ContentId const&, RBX::ContentId const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x314c84, "RBX::operator<(RBX::ContentId const&, RBX::ContentId const&)")
}

#[doc(alias = "__ZN3RBXneERKNS_9ContentIdES2_")]
pub fn stub_0x314c90() -> crate::slot::PortedFn {
// IDA 0x314c90: RBX::operator!=(RBX::ContentId const&, RBX::ContentId const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x314c90, "RBX::operator!=(RBX::ContentId const&, RBX::ContentId const&)")
}

#[doc(alias = "__ZN3RBXeqERKNS_9ContentIdES2_")]
pub fn stub_0x314ca8() -> crate::slot::PortedFn {
// IDA 0x314ca8: RBX::operator==(RBX::ContentId const&, RBX::ContentId const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x314ca8, "RBX::operator==(RBX::ContentId const&, RBX::ContentId const&)")
}

#[doc(alias = "__ZN3RBX9ContentId7fromUrlERKSs")]
pub fn stub_0x314cbc(handle: &crate::slot::InstanceHandle) {
// RBX::ContentId::fromUrl(std::string const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX9ContentId16CorrectBackslashERSs")]
pub fn stub_0x314cc8(handle: &crate::slot::InstanceHandle) {
// RBX::ContentId::CorrectBackslash(std::string&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX9ContentId14convertAssetIdERKSs")]
pub fn stub_0x314d14(handle: &crate::slot::InstanceHandle) {
// RBX::ContentId::convertAssetId(std::string const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN12_GLOBAL__N_111createIdUrlERSsRKSsS2_")]
pub fn stub_0x314f94() -> crate::slot::PortedFn {
// IDA 0x314f94: (anonymous namespace)::createIdUrl(std::string&, std::string const&, std::string const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x314f94, "(anonymous namespace)::createIdUrl(std::string&, std::string const&, std::string const&)")
}

#[doc(alias = "__ZN3RBX9ContentId22convertToLegacyContentERKSs")]
pub fn stub_0x315004(handle: &crate::slot::InstanceHandle) {
// RBX::ContentId::convertToLegacyContent(std::string const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX9ContentId10getAssetIdEv")]
pub fn stub_0x31507c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::ContentId getter.
cell.get()
}

#[doc(alias = "__ZN3RBX9ContentId10fromAssetsEPKc")]
pub fn stub_0x31530c(handle: &crate::slot::InstanceHandle) {
// RBX::ContentId::fromAssets(char const*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX18LegacyContentTableD1Ev")]
pub fn stub_0x315514(handle: crate::slot::InstanceHandle) {
// RBX::LegacyContentTable dtor.
drop(handle);
}

#[doc(alias = "__GLOBAL__I_a_111")]
pub fn stub_0x315594() -> crate::slot::PortedFn {
// IDA 0x315594: __GLOBAL__I_a_111.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x315594, "__GLOBAL__I_a_111")
}

#[doc(alias = "__ZN3RBX10FileSystem16getUserDirectoryEbNS_13FileSystemDirEPKc")]
pub fn stub_0x315680(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::FileSystem getter.
cell.get()
}

#[doc(alias = "__ZN3RBX10FileSystem19clearCacheDirectoryEPKci")]
pub fn stub_0x315ba4(map: &mut crate::slot::TreeMapModel) {
// map clear — releases every node.
map.clear();
}

#[doc(alias = "__ZN3RBX10FileSystem17getCacheDirectoryEbPKc")]
pub fn stub_0x315dc8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::FileSystem getter.
cell.get()
}

#[doc(alias = "__ZN3RBX10FileSystem21getBaseCacheDirectoryEb")]
pub fn stub_0x315dd4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::FileSystem getter.
cell.get()
}

#[doc(alias = "__GLOBAL__I_a_112")]
pub fn stub_0x3164c8() -> crate::slot::PortedFn {
// IDA 0x3164c8: __GLOBAL__I_a_112.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x3164c8, "__GLOBAL__I_a_112")
}

#[doc(alias = "__ZN3RBX4Http21getRobloxResponceLockEv")]
pub fn stub_0x316590(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Http getter.
cell.get()
}

#[doc(alias = "__ZN3RBX4Http18getCdnResponceLockEv")]
pub fn stub_0x3165a0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Http getter.
cell.get()
}

#[doc(alias = "__ZN3RBX4Http4initENS0_3APIE")]
pub fn stub_0x3165b0(handle: &crate::slot::InstanceHandle) {
// RBX::Http::init(RBX::Http::API) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Http14ThrowIfFailureEbPKcS2_")]
pub fn stub_0x316738(handle: &crate::slot::InstanceHandle) {
// RBX::Http::ThrowIfFailure(bool, char const*, char const*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Http15httpGetPostImplEbRSibRKSt3mapISsSsSt4lessISsESaISt4pairIKSsSsEEEbRSs")]
pub fn stub_0x316814(handle: &crate::slot::InstanceHandle) {
// RBX::Http::httpGetPostImpl(bool, std::istream&, bool, std::map<std::string, std::string, s~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Http4postERSibRSsb")]
pub fn stub_0x31688c(handle: &crate::slot::InstanceHandle) {
// RBX::Http::post(std::istream&, bool, std::string&, bool) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Http3getERSsb")]
pub fn stub_0x317de0(handle: &crate::slot::InstanceHandle) {
// RBX::Http::get(std::string&, bool) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Http12isRobloxSiteEPKc")]
pub fn stub_0x3180dc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Http getter.
cell.get()
}

#[doc(alias = "__ZN3RBXL14initTrustCheckEv")]
pub fn stub_0x3180ec() -> crate::slot::PortedFn {
// IDA 0x3180ec: RBX::initTrustCheck().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3180ec, "RBX::initTrustCheck()")
}

#[doc(alias = "__ZN3RBX13WindowAverageIddED1Ev")]
pub fn stub_0x3180f0(avg: crate::slot::WindowAverage) {
// WindowAverage dtor.
drop(avg);
}

#[doc(alias = "__ZN3RBX4Http10MutexGuardD1Ev")]
pub fn stub_0x318100(handle: crate::slot::InstanceHandle) {
// RBX::Http::MutexGuard dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX4Http10MutexGuardD2Ev")]
pub fn stub_0x31e45c(handle: crate::slot::InstanceHandle) {
// RBX::Http::MutexGuard dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX4Http10MutexGuardC2Ev")]
pub fn stub_0x31e558() -> crate::slot::InstanceHandle {
// RBX::Http::MutexGuard ctor.
crate::slot::InstanceHandle::new("RBX::Http::MutexGuard")
}

#[doc(alias = "__GLOBAL__I_a_113")]
pub fn stub_0x31e658() -> crate::slot::PortedFn {
// IDA 0x31e658: __GLOBAL__I_a_113.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x31e658, "__GLOBAL__I_a_113")
}

#[doc(alias = "__ZNK3RBX7Extents13clampInsideOfERKS0_")]
pub fn stub_0x31e8b0(handle: &crate::slot::InstanceHandle) {
// RBX::Extents::clampInsideOf(RBX::Extents const&) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX7Extents11closestFaceERKN3G3D7Vector3E")]
pub fn stub_0x31e9f4(handle: &crate::slot::InstanceHandle) {
// RBX::Extents::closestFace(G3D::Vector3 const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX7Extents14getCornerIndexEi")]
pub fn stub_0x31eae4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Extents getter.
cell.get()
}

#[doc(alias = "__ZNK3RBX7Extents9getCornerEi")]
pub fn stub_0x31eba8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Extents getter.
cell.get()
}

#[doc(alias = "__ZNK3RBX7Extents14getFaceCornersENS_8NormalIdERN3G3D7Vector3ES4_S4_S4_")]
pub fn stub_0x31ebfc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Extents getter.
cell.get()
}

#[doc(alias = "__ZNK3RBX7Extents7expressERKN3G3D15CoordinateFrameES4_")]
pub fn stub_0x31ee8c(handle: crate::slot::InstanceHandle) {
// RBX::Extents dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX7Extents12toWorldSpaceERKN3G3D15CoordinateFrameE")]
pub fn stub_0x31f1b4(handle: crate::slot::InstanceHandle) {
// RBX::Extents dtor.
drop(handle);
}

#[doc(alias = "__ZNK3RBX7Extents10faceCenterENS_8NormalIdE")]
pub fn stub_0x31f464(handle: &crate::slot::InstanceHandle) {
// RBX::Extents::faceCenter(RBX::NormalId) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX7Extents31computeClosestSqDistanceToPointERKN3G3D7Vector3E")]
pub fn stub_0x31f4d0(handle: &crate::slot::InstanceHandle) {
// RBX::Extents::computeClosestSqDistanceToPoint(G3D::Vector3 const&) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX7Extents19separatedByMoreThanERKS0_f")]
pub fn stub_0x31f5b4(handle: &crate::slot::InstanceHandle) {
// RBX::Extents::separatedByMoreThan(RBX::Extents const&, float) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX7Extents2vvERKN3G3D7Vector3ES4_")]
pub fn stub_0x31f68c(handle: &crate::slot::InstanceHandle) {
// RBX::Extents::vv(G3D::Vector3 const&, G3D::Vector3 const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__GLOBAL__I_a_114")]
pub fn stub_0x31f738() -> crate::slot::PortedFn {
// IDA 0x31f738: __GLOBAL__I_a_114.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x31f738, "__GLOBAL__I_a_114")
}

#[doc(alias = "__ZNK3RBX4FaceixEi")]
pub fn stub_0x31f90c(handle: &crate::slot::InstanceHandle) {
// RBX::Face::operator[](int) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4FaceixEi")]
pub fn stub_0x31f918(handle: &crate::slot::InstanceHandle) {
// RBX::Face::operator[](int) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Face10snapToGridEf")]
pub fn stub_0x31f924(handle: &crate::slot::InstanceHandle) {
// RBX::Face::snapToGrid(float) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Face19overlapWithinPlanesERKS0_S2_f")]
pub fn stub_0x31f964(handle: &crate::slot::InstanceHandle) {
// RBX::Face::overlapWithinPlanes(RBX::Face const&, RBX::Face const&, float) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX4Face18projectOverlapOnMeERKS0_")]
pub fn stub_0x31fa44(handle: &crate::slot::InstanceHandle) {
// RBX::Face::projectOverlapOnMe(RBX::Face const&) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX4Face24fuzzyContainsInExtrusionERKN3G3D7Vector3Ef")]
pub fn stub_0x31fcd4(handle: &crate::slot::InstanceHandle) {
// RBX::Face::fuzzyContainsInExtrusion(G3D::Vector3 const&, float) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX4Face6minMaxERKN3G3D7Vector3ES4_RfS5_")]
pub fn stub_0x31fdc4(handle: &crate::slot::InstanceHandle) {
// RBX::Face::minMax(G3D::Vector3 const&, G3D::Vector3 const&, float&, float&) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Face10hasOverlapERKS0_S2_f")]
pub fn stub_0x31fe6c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Face getter.
cell.get()
}

#[doc(alias = "__ZN3RBX4Face14cornersAlignedERKS0_S2_f")]
pub fn stub_0x31fefc(handle: &crate::slot::InstanceHandle) {
// RBX::Face::cornersAligned(RBX::Face const&, RBX::Face const&, float) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Face15fromExtentsSideERKNS_7ExtentsENS_8NormalIdE")]
pub fn stub_0x31ffe4(handle: &crate::slot::InstanceHandle) {
// RBX::Face::fromExtentsSide(RBX::Extents const&, RBX::NormalId) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX4Face12toWorldSpaceERKN3G3D15CoordinateFrameE")]
pub fn stub_0x320024(handle: crate::slot::InstanceHandle) {
// RBX::Face dtor.
drop(handle);
}

#[doc(alias = "__ZNK3RBX4Face13toObjectSpaceERKN3G3D15CoordinateFrameE")]
pub fn stub_0x32010c(handle: crate::slot::InstanceHandle) {
// RBX::Face dtor.
drop(handle);
}

#[doc(alias = "__ZNK3RBX4Face7getAxisEi")]
pub fn stub_0x3201f4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Face getter.
cell.get()
}

#[doc(alias = "__GLOBAL__I_a_115")]
pub fn stub_0x3202dc() -> crate::slot::PortedFn {
// IDA 0x3202dc: __GLOBAL__I_a_115.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x3202dc, "__GLOBAL__I_a_115")
}

#[doc(alias = "__ZN3RBX5FacesC1Ei")]
pub fn stub_0x320314() -> crate::slot::InstanceHandle {
// RBX::Faces ctor.
crate::slot::InstanceHandle::new("RBX::Faces")
}

#[doc(alias = "__ZN3RBX5Faces11setNormalIdENS_8NormalIdEb")]
pub fn stub_0x320318(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Faces setter.
cell.set(value)
}

#[doc(alias = "__ZNK3RBX5Faces11getNormalIdENS_8NormalIdE")]
pub fn stub_0x320338(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Faces getter.
cell.get()
}

#[doc(alias = "__ZN3RBX15StringConverterINS_5FacesEE15convertToStringERKS1_")]
pub fn stub_0x32034c(handle: &crate::slot::InstanceHandle) {
// RBX::StringConverter<RBX::Faces>::convertToString(RBX::Faces const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX15StringConverterINS_5FacesEE14convertToValueERKSsRS1_")]
pub fn stub_0x32059c(handle: &crate::slot::InstanceHandle) {
// RBX::StringConverter<RBX::Faces>::convertToValue(std::string const&, RBX::Faces&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__GLOBAL__I_a_116")]
pub fn stub_0x3207f8() -> crate::slot::PortedFn {
// IDA 0x3207f8: __GLOBAL__I_a_116.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x3207f8, "__GLOBAL__I_a_116")
}

#[doc(alias = "_gpc_free_polygon")]
pub fn stub_0x3208c0() -> crate::slot::PortedFn {
// IDA 0x3208c0: _gpc_free_polygon.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x3208c0, "_gpc_free_polygon")
}

#[doc(alias = "_gpc_polygon_clip")]
pub fn stub_0x320910() -> crate::slot::PortedFn {
// IDA 0x320910: _gpc_polygon_clip.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x320910, "_gpc_polygon_clip")
}

#[doc(alias = "_minimax_test")]
pub fn stub_0x321838() -> crate::slot::PortedFn {
// IDA 0x321838: _minimax_test.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x321838, "_minimax_test")
}

#[doc(alias = "_build_lmt")]
pub fn stub_0x321a18() -> crate::slot::PortedFn {
// IDA 0x321a18: _build_lmt.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x321a18, "_build_lmt")
}

#[doc(alias = "_build_sbt")]
pub fn stub_0x321fd4() -> crate::slot::PortedFn {
// IDA 0x321fd4: _build_sbt.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x321fd4, "_build_sbt")
}

#[doc(alias = "_free_sbtree")]
pub fn stub_0x322004() -> crate::slot::PortedFn {
// IDA 0x322004: _free_sbtree.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x322004, "_free_sbtree")
}

#[doc(alias = "_add_edge_to_aet")]
pub fn stub_0x322030() -> crate::slot::PortedFn {
// IDA 0x322030: _add_edge_to_aet.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x322030, "_add_edge_to_aet")
}

#[doc(alias = "_add_local_min")]
pub fn stub_0x322088() -> crate::slot::PortedFn {
// IDA 0x322088: _add_local_min.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x322088, "_add_local_min")
}

#[doc(alias = "_add_right")]
pub fn stub_0x322140() -> crate::slot::PortedFn {
// IDA 0x322140: _add_right.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x322140, "_add_right")
}

#[doc(alias = "_add_left")]
pub fn stub_0x3221b0() -> crate::slot::PortedFn {
// IDA 0x3221b0: _add_left.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x3221b0, "_add_left")
}

#[doc(alias = "_merge_right")]
pub fn stub_0x32221c() -> crate::slot::PortedFn {
// IDA 0x32221c: _merge_right.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x32221c, "_merge_right")
}

#[doc(alias = "_merge_left")]
pub fn stub_0x322268() -> crate::slot::PortedFn {
// IDA 0x322268: _merge_left.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x322268, "_merge_left")
}

#[doc(alias = "_build_intersection_table")]
pub fn stub_0x3222b8() -> crate::slot::PortedFn {
// IDA 0x3222b8: _build_intersection_table.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x3222b8, "_build_intersection_table")
}

#[doc(alias = "_insert_bound")]
pub fn stub_0x3224d8() -> crate::slot::PortedFn {
// IDA 0x3224d8: _insert_bound.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x3224d8, "_insert_bound")
}

#[doc(alias = "_bound_list")]
pub fn stub_0x322518() -> crate::slot::PortedFn {
// IDA 0x322518: _bound_list.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x322518, "_bound_list")
}

#[doc(alias = "_create_contour_bboxes")]
pub fn stub_0x3225b8() -> crate::slot::PortedFn {
// IDA 0x3225b8: _create_contour_bboxes.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x3225b8, "_create_contour_bboxes")
}

#[doc(alias = "__ZL14initLocalScopev")]
pub fn stub_0x3226f8() -> crate::slot::PortedFn {
// IDA 0x3226f8: initLocalScope().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3226f8, "initLocalScope()")
}

#[doc(alias = "__ZN3RBX4GuidC1Ev")]
pub fn stub_0x32281c() -> crate::slot::InstanceHandle {
// RBX::Guid ctor.
crate::slot::InstanceHandle::new("RBX::Guid")
}

#[doc(alias = "__ZN3RBX4Guid20generateStandardGUIDERSs")]
pub fn stub_0x322850(handle: &crate::slot::InstanceHandle) {
// RBX::Guid::generateStandardGUID(std::string&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Guid15generateRBXGUIDERSs")]
pub fn stub_0x32298c(handle: &crate::slot::InstanceHandle) {
// RBX::Guid::generateRBXGUID(std::string&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Guid6assignENS0_4DataE")]
pub fn stub_0x322b04(handle: &crate::slot::InstanceHandle) {
// RBX::Guid::assign(RBX::Guid::Data) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX4Guid4DataltERKS1_")]
pub fn stub_0x322b10(handle: &crate::slot::InstanceHandle) {
// RBX::Guid::Data::operator<(RBX::Guid::Data const&) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Guid7compareEPKS0_S2_")]
pub fn stub_0x322b38(handle: &crate::slot::InstanceHandle) {
// RBX::Guid::compare(RBX::Guid const*, RBX::Guid const*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Guid7compareEPKS0_S2_S2_S2_")]
pub fn stub_0x322b78(handle: &crate::slot::InstanceHandle) {
// RBX::Guid::compare(RBX::Guid const*, RBX::Guid const*, RBX::Guid const*, RBX::Guid const*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX4Guid4Data14readableStringEi")]
pub fn stub_0x322bdc(handle: &crate::slot::InstanceHandle) {
// RBX::Guid::Data::readableString(int) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__GLOBAL__I_a_117")]
pub fn stub_0x322e00() -> crate::slot::PortedFn {
// IDA 0x322e00: __GLOBAL__I_a_117.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x322e00, "__GLOBAL__I_a_117")
}

#[doc(alias = "__ZN3RBX14InstanceHandleC1EPNS_10Reflection13DescribedBaseE")]
pub fn stub_0x322ec8() -> crate::slot::InstanceHandle {
// RBX::InstanceHandle ctor.
crate::slot::InstanceHandle::new("RBX::InstanceHandle")
}

#[doc(alias = "__ZNK3RBX14InstanceHandle12operatorLessERKS0_")]
pub fn stub_0x322ed8(handle: &crate::slot::InstanceHandle) {
// RBX::InstanceHandle::operatorLess(RBX::InstanceHandle const&) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX14InstanceHandle5emptyEv")]
pub fn stub_0x322ee8(handle: &crate::slot::InstanceHandle) {
// RBX::InstanceHandle::empty() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14InstanceHandle6linkToEN5boost10shared_ptrINS_10Reflection13DescribedBaseEEE")]
pub fn stub_0x322ef4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::DescribedBase")
}

#[doc(alias = "__GLOBAL__I_a_118")]
pub fn stub_0x322ef8() -> crate::slot::PortedFn {
// IDA 0x322ef8: __GLOBAL__I_a_118.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x322ef8, "__GLOBAL__I_a_118")
}

#[doc(alias = "__ZN3RBX4Hash4hashERKSs")]
pub fn stub_0x323028(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Hash getter.
cell.get()
}

#[doc(alias = "__ZN3RBX17HeartbeatInstance34onServiceProviderHeartbeatInstanceEPNS_15ServiceProviderES2_")]
pub fn stub_0x32305c(handle: &crate::slot::InstanceHandle) {
// RBX::HeartbeatInstance::onServiceProviderHeartbeatInstance(RBX::ServiceProvider*, RBX::Ser~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE7connectIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_17HeartbeatInstanceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEEENS0_10connectionERKT_")]
pub fn stub_0x323238() -> crate::slot::SlotConnection {
// IDA 0x323238: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_17HeartbeatInstanceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED1Ev")]
pub fn stub_0x3232ac(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_17HeartbeatInstanceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED0Ev")]
pub fn stub_0x3232d8(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_17HeartbeatInstanceES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_")]
pub fn stub_0x3233ac(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3233ac: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_17HeartbeatInstanceES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_")]
pub fn stub_0x3233b4(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3233b4: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX17HeartbeatInstanceERKNS4_9HeartbeatEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_")]
pub fn stub_0x3233bc() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_17HeartbeatInstanceES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED1Ev")]
pub fn stub_0x3233d4(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3233d4: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_17HeartbeatInstanceES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED0Ev")]
pub fn stub_0x323400(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x323400: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "__GLOBAL__I_a_119")]
pub fn stub_0x3234d4() -> crate::slot::PortedFn {
// IDA 0x3234d4: __GLOBAL__I_a_119.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x3234d4, "__GLOBAL__I_a_119")
}
