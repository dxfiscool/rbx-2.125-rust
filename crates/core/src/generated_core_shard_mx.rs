//! core shard mx — 100 core stubs EA-sorted asc global gap filler not yet in rbx_core.
//! Source: `ida/export.json` (85545 funcs) EA-sorted asc, next 100 not yet in rbx_core (fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound; fallback 33887, 1723 uncovered before -> 1623 after, batch 0xf248d4..0xf250cc).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;


#[doc(alias = "__ZNSt9basic_iosIcSt11char_traitsIcEE5rdbufEPSt15basic_streambufIcS1_E$shim")]
// 0xf248d4 — __ZNSt9basic_iosIcSt11char_traitsIcEE5rdbufEPSt15basic_streambufIcS1_E$shim
// type: int()
pub fn stub_0xf248d4() -> ! { todo!("0xf248d4 __ZNSt9basic_iosIcSt11char_traitsIcEE5rdbufEPSt15basic_streambufIcS1_E$shim") }

#[doc(alias = "__ZN5boost9iostreams16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EED0Ev$shim")]
// 0xf248e0 — __ZN5boost9iostreams16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EED0Ev$shim
// type: int()
pub fn stub_0xf248e0() -> ! { todo!("0xf248e0 __ZN5boost9iostreams16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EED0Ev$shim") }

#[doc(alias = "__ZNSt5dequeIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EE17_M_reallocate_mapEmb$shim")]
// 0xf248ec — __ZNSt5dequeIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EE17_M_reallocate_mapEmb$shim
// type: int()
pub fn stub_0xf248ec() -> ! { todo!("0xf248ec __ZNSt5dequeIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EE17_M_reallocate_mapEmb$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_20sMeshContentProviderEEEERKS0_v$shim")]
// 0xf248f8 — __ZN3RBX4Name9doDeclareILZNS_20sMeshContentProviderEEEERKS0_v$shim
// type: int()
pub fn stub_0xf248f8() -> ! { todo!("0xf248f8 __ZN3RBX4Name9doDeclareILZNS_20sMeshContentProviderEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX19MeshContentProviderD0Ev$shim")]
// 0xf24904 — __ZN3RBX19MeshContentProviderD0Ev$shim
// type: void __fastcall(RBX::MeshContentProvider *)
pub fn stub_0xf24904() -> ! { todo!("0xf24904 __ZN3RBX19MeshContentProviderD0Ev$shim") }

#[doc(alias = "__ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSK_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISF_EEEEmRKT_RKT0_$shim")]
// 0xf24910 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSK_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISF_EEEEmRKT_RKT0_$shim
// type: int __fastcall(int)
pub fn stub_0xf24910() -> ! { todo!("0xf24910 __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSK_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISF_EEEEmRKT_RKT0_$shim") }

#[doc(alias = "__ZN5boost8functionIFPN3RBX5ImageERSiRKSsEEaSERKS8_$shim")]
// 0xf2491c — __ZN5boost8functionIFPN3RBX5ImageERSiRKSsEEaSERKS8_$shim
// type: int __fastcall(int)
pub fn stub_0xf2491c() -> ! { todo!("0xf2491c __ZN5boost8functionIFPN3RBX5ImageERSiRKSsEEaSERKS8_$shim") }

#[doc(alias = "__ZN3RBX22TextureContentProviderD2Ev$shim")]
// 0xf24928 — __ZN3RBX22TextureContentProviderD2Ev$shim
// type: void __fastcall(RBX::TextureContentProvider *__hidden this)
pub fn stub_0xf24928() -> ! { todo!("0xf24928 __ZN3RBX22TextureContentProviderD2Ev$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_23sTextureContentProviderEEEERKS0_v$shim")]
// 0xf24934 — __ZN3RBX4Name9doDeclareILZNS_23sTextureContentProviderEEEERKS0_v$shim
// type: int()
pub fn stub_0xf24934() -> ! { todo!("0xf24934 __ZN3RBX4Name9doDeclareILZNS_23sTextureContentProviderEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX22TextureContentProviderD0Ev$shim")]
// 0xf24940 — __ZN3RBX22TextureContentProviderD0Ev$shim
// type: void __fastcall(RBX::TextureContentProvider *)
pub fn stub_0xf24940() -> ! { todo!("0xf24940 __ZN3RBX22TextureContentProviderD0Ev$shim") }

#[doc(alias = "__ZN3RBX14AsyncHttpCacheINS_15ContentProvider13CachedContentELb0EE12setCacheSizeEi$shim")]
// 0xf2497c — __ZN3RBX14AsyncHttpCacheINS_15ContentProvider13CachedContentELb0EE12setCacheSizeEi$shim
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0xf2497c() -> ! { todo!("0xf2497c __ZN3RBX14AsyncHttpCacheINS_15ContentProvider13CachedContentELb0EE12setCacheSizeEi$shim") }

#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISD_EESN_$shim")]
// 0xf24988 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISD_EESN_$shim
// type: int()
pub fn stub_0xf24988() -> ! { todo!("0xf24988 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISD_EESN_$shim") }

#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm$shim")]
// 0xf24994 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm$shim
// type: int()
pub fn stub_0xf24994() -> ! { todo!("0xf24994 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm$shim") }

#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm$shim")]
// 0xf249a0 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm$shim
// type: int()
pub fn stub_0xf249a0() -> ! { todo!("0xf249a0 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm$shim") }

#[doc(alias = "__ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSI_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEmRKT_RKT0_$shim")]
// 0xf249ac — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSI_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEmRKT_RKT0_$shim
// type: int()
pub fn stub_0xf249ac() -> ! { todo!("0xf249ac __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSI_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEmRKT_RKT0_$shim") }

#[doc(alias = "__ZNSt6vectorISsSaISsEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPSsS1_EERKSs$shim")]
// 0xf249b8 — __ZNSt6vectorISsSaISsEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPSsS1_EERKSs$shim
// type: void __fastcall(struct _Unwind_Exception **, std::string *, const std::string *)
pub fn stub_0xf249b8() -> ! { todo!("0xf249b8 __ZNSt6vectorISsSaISsEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPSsS1_EERKSs$shim") }

#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_9algorithm6detail13token_finderFINS4_10is_any_ofFIcEEEEE7managerERKNS1_15function_bufferERSA_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
// 0xf249c4 — __ZN5boost6detail8function15functor_managerINS_9algorithm6detail13token_finderFINS4_10is_any_ofFIcEEEEE7managerERKNS1_15function_bufferERSA_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
// type: int __fastcall(int, int, int, int, int, void *, int, int, int)
pub fn stub_0xf249c4() -> ! { todo!("0xf249c4 __ZN5boost6detail8function15functor_managerINS_9algorithm6detail13token_finderFINS4_10is_any_ofFIcEEEEE7managerERKNS1_15function_bufferERSA_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim") }

#[doc(alias = "__ZSt9sort_heapIPcEvT_S1_$shim")]
// 0xf249d0 — __ZSt9sort_heapIPcEvT_S1_$shim
// type: int()
pub fn stub_0xf249d0() -> ! { todo!("0xf249d0 __ZSt9sort_heapIPcEvT_S1_$shim") }

#[doc(alias = "__ZSt16__insertion_sortIPcEvT_S1_$shim")]
// 0xf249dc — __ZSt16__insertion_sortIPcEvT_S1_$shim
// type: int __fastcall(void *)
pub fn stub_0xf249dc() -> ! { todo!("0xf249dc __ZSt16__insertion_sortIPcEvT_S1_$shim") }

#[doc(alias = "__ZSt13__adjust_heapIPcicEvT_T0_S2_T1_$shim")]
// 0xf249e8 — __ZSt13__adjust_heapIPcicEvT_T0_S2_T1_$shim
// type: int()
pub fn stub_0xf249e8() -> ! { todo!("0xf249e8 __ZSt13__adjust_heapIPcicEvT_T0_S2_T1_$shim") }

#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESC_ENS3_5list2INS3_5valueISE_EENSI_ISC_EEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
// 0xf249f4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESC_ENS3_5list2INS3_5valueISE_EENSI_ISC_EEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
// type: int __fastcall(int, int, int, int, void *, int, int, int, int, int)
pub fn stub_0xf249f4() -> ! { todo!("0xf249f4 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESC_ENS3_5list2INS3_5valueISE_EENSI_ISC_EEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim") }

#[doc(alias = "__ZN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE4slot24safe_static_do_get_mutexEv$shim")]
// 0xf24a18 — __ZN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE4slot24safe_static_do_get_mutexEv$shim
// type: int()
pub fn stub_0xf24a18() -> ! { todo!("0xf24a18 __ZN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE4slot24safe_static_do_get_mutexEv$shim") }

#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_12TimerServiceEEEPT_v$shim")]
// 0xf24a30 — __ZNK3RBX15ServiceProvider4findINS_12TimerServiceEEEPT_v$shim
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0xf24a30() -> ! { todo!("0xf24a30 __ZNK3RBX15ServiceProvider4findINS_12TimerServiceEEEPT_v$shim") }

#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_10RunServiceEEEPT_v$shim")]
// 0xf24a3c — __ZNK3RBX15ServiceProvider4findINS_10RunServiceEEEPT_v$shim
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0xf24a3c() -> ! { todo!("0xf24a3c __ZNK3RBX15ServiceProvider4findINS_10RunServiceEEEPT_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sFunctionalTestEEEERKS0_v$shim")]
// 0xf24a6c — __ZN3RBX4Name9doDeclareILZNS_15sFunctionalTestEEEERKS0_v$shim
// type: int()
pub fn stub_0xf24a6c() -> ! { todo!("0xf24a6c __ZN3RBX4Name9doDeclareILZNS_15sFunctionalTestEEEERKS0_v$shim") }

#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX11TestServiceEiEENS3_5list2INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
// 0xf24ab4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX11TestServiceEiEENS3_5list2INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
pub fn stub_0xf24ab4() -> ! { todo!("0xf24ab4 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX11TestServiceEiEENS3_5list2INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim") }

#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS3_5list3INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEENSB_IdEEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
// 0xf24acc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS3_5list3INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEENSB_IdEEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
pub fn stub_0xf24acc() -> ! { todo!("0xf24acc __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS3_5list3INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEENSB_IdEEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX14FunctionalTest6ResultESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf24b2c — __ZNSt6vectorIN3RBX14FunctionalTest6ResultESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int()
pub fn stub_0xf24b2c() -> ! { todo!("0xf24b2c __ZNSt6vectorIN3RBX14FunctionalTest6ResultESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZSt16__ostream_insertIcSt11char_traitsIcEERSt13basic_ostreamIT_T0_ES6_PKS3_i$shim")]
// 0xf24b38 — __ZSt16__ostream_insertIcSt11char_traitsIcEERSt13basic_ostreamIT_T0_ES6_PKS3_i$shim
// type: int __fastcall(int, int, int)
pub fn stub_0xf24b38() -> ! { todo!("0xf24b38 __ZSt16__ostream_insertIcSt11char_traitsIcEERSt13basic_ostreamIT_T0_ES6_PKS3_i$shim") }

#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiENS3_5list2INS3_5valueIS9_EENSD_IiEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
// 0xf24b5c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiENS3_5list2INS3_5valueIS9_EENSD_IiEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
pub fn stub_0xf24b5c() -> ! { todo!("0xf24b5c __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiENS3_5list2INS3_5valueIS9_EENSD_IiEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim") }

#[doc(alias = "__ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE16unchecked_rehashEm$shim")]
// 0xf24b68 — __ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE16unchecked_rehashEm$shim
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int, int, int, int, void *, int, int, struct _Unwind_Exception *, int)
pub fn stub_0xf24b68() -> ! { todo!("0xf24b68 __ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE16unchecked_rehashEm$shim") }

#[doc(alias = "__ZNK5boost9unordered6detail10table_implINS1_3setISaISsESsNS_4hashISsEESt8equal_toISsEEEE14find_node_implISsS8_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISsEEEEmRKT_RKT0_$shim")]
// 0xf24b74 — __ZNK5boost9unordered6detail10table_implINS1_3setISaISsESsNS_4hashISsEESt8equal_toISsEEEE14find_node_implISsS8_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISsEEEEmRKT_RKT0_$shim
// type: int()
pub fn stub_0xf24b74() -> ! { todo!("0xf24b74 __ZNK5boost9unordered6detail10table_implINS1_3setISaISsESsNS_4hashISsEESt8equal_toISsEEEE14find_node_implISsS8_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISsEEEEmRKT_RKT0_$shim") }

#[doc(alias = "__ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEENS3_5list1INS3_5valueIS9_EEEEEEE12manage_smallERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE$shim")]
// 0xf24b80 — __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEENS3_5list1INS3_5valueIS9_EEEEEEE12manage_smallERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE$shim
// type: int()
pub fn stub_0xf24b80() -> ! { todo!("0xf24b80 __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEENS3_5list1INS3_5valueIS9_EEEEEEE12manage_smallERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE$shim") }

#[doc(alias = "__ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEEEEclIPFvS7_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i$shim")]
// 0xf24b8c — __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEEEEclIPFvS7_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i$shim
// type: int()
pub fn stub_0xf24b8c() -> ! { todo!("0xf24b8c __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEEEEclIPFvS7_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i$shim") }

#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESsENS3_5list2INS3_5valueIS9_EENSD_ISsEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
// 0xf24b98 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESsENS3_5list2INS3_5valueIS9_EENSD_ISsEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
pub fn stub_0xf24b98() -> ! { todo!("0xf24b98 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESsENS3_5list2INS3_5valueIS9_EENSD_ISsEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim") }

#[doc(alias = "_realloc$shim")]
// 0xf24ba4 — _realloc$shim
// type: void *__cdecl(void *__ptr, size_t __size)
pub fn stub_0xf24ba4() -> ! { todo!("0xf24ba4 _realloc$shim") }

#[doc(alias = "__ZN13lua_exceptionD2Ev$shim")]
// 0xf24bb0 — __ZN13lua_exceptionD2Ev$shim
// type: void __fastcall(lua_exception *)
pub fn stub_0xf24bb0() -> ! { todo!("0xf24bb0 __ZN13lua_exceptionD2Ev$shim") }

#[doc(alias = "_strcat$shim")]
// 0xf24bbc — _strcat$shim
// type: char *__cdecl(char *__s1, const char *__s2)
pub fn stub_0xf24bbc() -> ! { todo!("0xf24bbc _strcat$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_16sNotificationBoxEEEERKS0_v$shim")]
// 0xf24be0 — __ZN3RBX4Name9doDeclareILZNS_16sNotificationBoxEEEERKS0_v$shim
// type: int()
pub fn stub_0xf24be0() -> ! { todo!("0xf24be0 __ZN3RBX4Name9doDeclareILZNS_16sNotificationBoxEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX18NotificationObjectD2Ev$shim")]
// 0xf24bec — __ZN3RBX18NotificationObjectD2Ev$shim
// type: void __fastcall(RBX::NotificationObject *)
pub fn stub_0xf24bec() -> ! { todo!("0xf24bec __ZN3RBX18NotificationObjectD2Ev$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_19sNotificationObjectEEEERKS0_v$shim")]
// 0xf24c1c — __ZN3RBX4Name9doDeclareILZNS_19sNotificationObjectEEEERKS0_v$shim
// type: int()
pub fn stub_0xf24c1c() -> ! { todo!("0xf24c1c __ZN3RBX4Name9doDeclareILZNS_19sNotificationObjectEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name7declareILZNS_6sFrameEEEERKS0_v$shim")]
// 0xf24c28 — __ZN3RBX4Name7declareILZNS_6sFrameEEEERKS0_v$shim
// type: int()
pub fn stub_0xf24c28() -> ! { todo!("0xf24c28 __ZN3RBX4Name7declareILZNS_6sFrameEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_6sFrameEEEERKS0_v$shim")]
// 0xf24c34 — __ZN3RBX4Name9doDeclareILZNS_6sFrameEEEERKS0_v$shim
// type: int()
pub fn stub_0xf24c34() -> ! { todo!("0xf24c34 __ZN3RBX4Name9doDeclareILZNS_6sFrameEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX13FriendServiceD2Ev$shim")]
// 0xf24c4c — __ZN3RBX13FriendServiceD2Ev$shim
// type: void __fastcall(RBX::FriendService *)
pub fn stub_0xf24c4c() -> ! { todo!("0xf24c4c __ZN3RBX13FriendServiceD2Ev$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX13FriendService15FriendEventTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf24c64 — __ZNSt6vectorIN3RBX13FriendService15FriendEventTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int()
pub fn stub_0xf24c64() -> ! { todo!("0xf24c64 __ZNSt6vectorIN3RBX13FriendService15FriendEventTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX13FriendService12FriendStatusESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf24c70 — __ZNSt6vectorIN3RBX13FriendService12FriendStatusESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int()
pub fn stub_0xf24c70() -> ! { todo!("0xf24c70 __ZNSt6vectorIN3RBX13FriendService12FriendStatusESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE24safe_static_do_get_mutexEv$shim")]
// 0xf24c7c — __ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE24safe_static_do_get_mutexEv$shim
// type: int __fastcall(int)
pub fn stub_0xf24c7c() -> ! { todo!("0xf24c7c __ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE24safe_static_do_get_mutexEv$shim") }

#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13FriendServiceEEEiSt3setIiSt4lessIiESaIiEEPSsPSt9exceptionENS3_5list5INS3_5valueIS8_EENSK_IiEENSK_ISD_EENS_3argILi1EEENSO_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
// 0xf24c88 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13FriendServiceEEEiSt3setIiSt4lessIiESaIiEEPSsPSt9exceptionENS3_5list5INS3_5valueIS8_EENSK_IiEENSK_ISD_EENS_3argILi1EEENSO_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
// type: int __fastcall(int, int, int, int, void *, void *, int, int, int, int)
pub fn stub_0xf24c88() -> ! { todo!("0xf24c88 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13FriendServiceEEEiSt3setIiSt4lessIiESaIiEEPSsPSt9exceptionENS3_5list5INS3_5valueIS8_EENSK_IiEENSK_ISD_EENS_3argILi1EEENSO_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim") }

#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13FriendServiceEEEiNS_10shared_ptrISt3mapIiNS7_12FriendStatusESt4lessIiESaISt4pairIKiSB_EEEEEENS3_5list3INS3_5valueIS8_EENSN_IiEENSN_ISJ_EEEEEEE7managerERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
// 0xf24c94 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13FriendServiceEEEiNS_10shared_ptrISt3mapIiNS7_12FriendStatusESt4lessIiESaISt4pairIKiSB_EEEEEENS3_5list3INS3_5valueIS8_EENSN_IiEENSN_ISJ_EEEEEEE7managerERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
pub fn stub_0xf24c94() -> ! { todo!("0xf24c94 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13FriendServiceEEEiNS_10shared_ptrISt3mapIiNS7_12FriendStatusESt4lessIiESaISt4pairIKiSB_EEEEEENS3_5list3INS3_5valueIS8_EENSN_IiEENSN_ISJ_EEEEEEE7managerERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim") }

#[doc(alias = "__ZN3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE24safe_static_do_get_mutexEv$shim")]
// 0xf24ca0 — __ZN3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE24safe_static_do_get_mutexEv$shim
// type: int()
pub fn stub_0xf24ca0() -> ! { todo!("0xf24ca0 __ZN3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE24safe_static_do_get_mutexEv$shim") }

#[doc(alias = "__ZN3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE4slot24safe_static_do_get_mutexEv$shim")]
// 0xf24cac — __ZN3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE4slot24safe_static_do_get_mutexEv$shim
// type: int __fastcall(int)
pub fn stub_0xf24cac() -> ! { todo!("0xf24cac __ZN3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE4slot24safe_static_do_get_mutexEv$shim") }

#[doc(alias = "__ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE4slot24safe_static_do_get_mutexEv$shim")]
// 0xf24cb8 — __ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE4slot24safe_static_do_get_mutexEv$shim
// type: int __fastcall(int)
pub fn stub_0xf24cb8() -> ! { todo!("0xf24cb8 __ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE4slot24safe_static_do_get_mutexEv$shim") }

#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi3EFviiN3RBX13FriendService12FriendStatusEEEclEiiS4_$shim")]
// 0xf24cd0 — __ZN3rbx7signals16signal_with_argsILi3EFviiN3RBX13FriendService12FriendStatusEEEclEiiS4_$shim
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
pub fn stub_0xf24cd0() -> ! { todo!("0xf24cd0 __ZN3rbx7signals16signal_with_argsILi3EFviiN3RBX13FriendService12FriendStatusEEEclEiiS4_$shim") }

#[doc(alias = "__ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE13disconnectAllEv$shim")]
// 0xf24cdc — __ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE13disconnectAllEv$shim
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0xf24cdc() -> ! { todo!("0xf24cdc __ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE13disconnectAllEv$shim") }

#[doc(alias = "__ZNK5boost9function3IviiN3RBX13FriendService12FriendStatusEEclEiiS3_$shim")]
// 0xf24cf4 — __ZNK5boost9function3IviiN3RBX13FriendService12FriendStatusEEclEiiS3_$shim
// type: int()
pub fn stub_0xf24cf4() -> ! { todo!("0xf24cf4 __ZNK5boost9function3IviiN3RBX13FriendService12FriendStatusEEclEiiS3_$shim") }

#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi3EFviiN3RBX13FriendService15FriendEventTypeEEEclEiiS4_$shim")]
// 0xf24d00 — __ZN3rbx7signals16signal_with_argsILi3EFviiN3RBX13FriendService15FriendEventTypeEEEclEiiS4_$shim
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
pub fn stub_0xf24d00() -> ! { todo!("0xf24d00 __ZN3rbx7signals16signal_with_argsILi3EFviiN3RBX13FriendService15FriendEventTypeEEEclEiiS4_$shim") }

#[doc(alias = "__ZN3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE13disconnectAllEv$shim")]
// 0xf24d0c — __ZN3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE13disconnectAllEv$shim
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0xf24d0c() -> ! { todo!("0xf24d0c __ZN3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE13disconnectAllEv$shim") }

#[doc(alias = "__ZNK5boost9function3IviiN3RBX13FriendService15FriendEventTypeEEclEiiS3_$shim")]
// 0xf24d24 — __ZNK5boost9function3IviiN3RBX13FriendService15FriendEventTypeEEclEiiS3_$shim
// type: int()
pub fn stub_0xf24d24() -> ! { todo!("0xf24d24 __ZNK5boost9function3IviiN3RBX13FriendService15FriendEventTypeEEclEiiS3_$shim") }

#[doc(alias = "__ZN3RBX17GameBasicSettingsD2Ev$shim")]
// 0xf24d30 — __ZN3RBX17GameBasicSettingsD2Ev$shim
// type: void __fastcall(RBX::GameBasicSettings *)
pub fn stub_0xf24d30() -> ! { todo!("0xf24d30 __ZN3RBX17GameBasicSettingsD2Ev$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_18sGameBasicSettingsEEEERKS0_v$shim")]
// 0xf24d54 — __ZN3RBX4Name9doDeclareILZNS_18sGameBasicSettingsEEEERKS0_v$shim
// type: int()
pub fn stub_0xf24d54() -> ! { todo!("0xf24d54 __ZN3RBX4Name9doDeclareILZNS_18sGameBasicSettingsEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEED0Ev$shim")]
// 0xf24d60 — __ZN3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEED0Ev$shim
// type: int()
pub fn stub_0xf24d60() -> ! { todo!("0xf24d60 __ZN3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEED0Ev$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX17GameBasicSettings20RenderQualitySettingESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf24d6c — __ZNSt6vectorIN3RBX17GameBasicSettings20RenderQualitySettingESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int()
pub fn stub_0xf24d6c() -> ! { todo!("0xf24d6c __ZNSt6vectorIN3RBX17GameBasicSettings20RenderQualitySettingESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX17GameBasicSettings11ControlModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf24d78 — __ZNSt6vectorIN3RBX17GameBasicSettings11ControlModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int()
pub fn stub_0xf24d78() -> ! { todo!("0xf24d78 __ZNSt6vectorIN3RBX17GameBasicSettings11ControlModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZN3RBX18RenderHooksServiceD2Ev$shim")]
// 0xf24da8 — __ZN3RBX18RenderHooksServiceD2Ev$shim
// type: void __fastcall(RBX::RenderHooksService *)
pub fn stub_0xf24da8() -> ! { todo!("0xf24da8 __ZN3RBX18RenderHooksServiceD2Ev$shim") }

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEE9singletonEv$shim")]
// 0xf24dc0 — __ZN3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEE9singletonEv$shim
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, boost::mutex *, char, int, int, int, int, int, int)
pub fn stub_0xf24dc0() -> ! { todo!("0xf24dc0 __ZN3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEE9singletonEv$shim") }

#[doc(alias = "__ZN3RBX17ClientAppSettingsD2Ev$shim")]
// 0xf24dcc — __ZN3RBX17ClientAppSettingsD2Ev$shim
// type: void __fastcall(RBX::ClientAppSettings *)
pub fn stub_0xf24dcc() -> ! { todo!("0xf24dcc __ZN3RBX17ClientAppSettingsD2Ev$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_16sFastLogSettingsEEEERKS0_v$shim")]
// 0xf24de4 — __ZN3RBX4Name9doDeclareILZNS_16sFastLogSettingsEEEERKS0_v$shim
// type: int()
pub fn stub_0xf24de4() -> ! { todo!("0xf24de4 __ZN3RBX4Name9doDeclareILZNS_16sFastLogSettingsEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEED0Ev$shim")]
// 0xf24dfc — __ZN3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEED0Ev$shim
// type: int()
pub fn stub_0xf24dfc() -> ! { todo!("0xf24dfc __ZN3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEED0Ev$shim") }

#[doc(alias = "__ZN3RBX19CustomEventReceiverD2Ev$shim")]
// 0xf24e14 — __ZN3RBX19CustomEventReceiverD2Ev$shim
// type: void __fastcall(RBX::CustomEventReceiver *)
pub fn stub_0xf24e14() -> ! { todo!("0xf24e14 __ZN3RBX19CustomEventReceiverD2Ev$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sTextureTrailEEEERKS0_v$shim")]
// 0xf24e50 — __ZN3RBX4Name9doDeclareILZNS_13sTextureTrailEEEERKS0_v$shim
// type: int()
pub fn stub_0xf24e50() -> ! { todo!("0xf24e50 __ZN3RBX4Name9doDeclareILZNS_13sTextureTrailEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sFloorWireEEEERKS0_v$shim")]
// 0xf24ea4 — __ZN3RBX4Name9doDeclareILZNS_10sFloorWireEEEERKS0_v$shim
// type: int()
pub fn stub_0xf24ea4() -> ! { todo!("0xf24ea4 __ZN3RBX4Name9doDeclareILZNS_10sFloorWireEEEERKS0_v$shim") }

#[doc(alias = "__ZNSt6vectorIPN3RBX5Voxel18CellChangeListenerESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_$shim")]
// 0xf24ed4 — __ZNSt6vectorIPN3RBX5Voxel18CellChangeListenerESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_$shim
// type: int __fastcall(int, void *)
pub fn stub_0xf24ed4() -> ! { todo!("0xf24ed4 __ZNSt6vectorIPN3RBX5Voxel18CellChangeListenerESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_$shim") }

#[doc(alias = "__ZN3RBX15writeCountValueINS_17StringWriteBufferEEEvRT_j$shim")]
// 0xf24ee0 — __ZN3RBX15writeCountValueINS_17StringWriteBufferEEEvRT_j$shim
// type: int __fastcall(std::string *)
pub fn stub_0xf24ee0() -> ! { todo!("0xf24ee0 __ZN3RBX15writeCountValueINS_17StringWriteBufferEEEvRT_j$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sMegaClusterEEEERKS0_v$shim")]
// 0xf24f10 — __ZN3RBX4Name9doDeclareILZNS_12sMegaClusterEEEERKS0_v$shim
// type: int()
pub fn stub_0xf24f10() -> ! { todo!("0xf24f10 __ZN3RBX4Name9doDeclareILZNS_12sMegaClusterEEEERKS0_v$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX5Voxel18WaterCellDirectionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf24f28 — __ZNSt6vectorIN3RBX5Voxel18WaterCellDirectionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int()
pub fn stub_0xf24f28() -> ! { todo!("0xf24f28 __ZNSt6vectorIN3RBX5Voxel18WaterCellDirectionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX5Voxel14WaterCellForceESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf24f34 — __ZNSt6vectorIN3RBX5Voxel14WaterCellForceESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int()
pub fn stub_0xf24f34() -> ! { todo!("0xf24f34 __ZNSt6vectorIN3RBX5Voxel14WaterCellForceESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX5Voxel15CellOrientationESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf24f40 — __ZNSt6vectorIN3RBX5Voxel15CellOrientationESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int()
pub fn stub_0xf24f40() -> ! { todo!("0xf24f40 __ZNSt6vectorIN3RBX5Voxel15CellOrientationESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX5Voxel9CellBlockESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf24f4c — __ZNSt6vectorIN3RBX5Voxel9CellBlockESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int()
pub fn stub_0xf24f4c() -> ! { todo!("0xf24f4c __ZNSt6vectorIN3RBX5Voxel9CellBlockESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX5Voxel12CellMaterialESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf24f58 — __ZNSt6vectorIN3RBX5Voxel12CellMaterialESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int()
pub fn stub_0xf24f58() -> ! { todo!("0xf24f58 __ZNSt6vectorIN3RBX5Voxel12CellMaterialESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZN3RBX13PluginManagerD2Ev$shim")]
// 0xf24f70 — __ZN3RBX13PluginManagerD2Ev$shim
// type: void __fastcall(RBX::PluginManager *)
pub fn stub_0xf24f70() -> ! { todo!("0xf24f70 __ZN3RBX13PluginManagerD2Ev$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sPluginManagerEEEERKS0_v$shim")]
// 0xf24f7c — __ZN3RBX4Name9doDeclareILZNS_14sPluginManagerEEEERKS0_v$shim
// type: int()
pub fn stub_0xf24f7c() -> ! { todo!("0xf24f7c __ZN3RBX4Name9doDeclareILZNS_14sPluginManagerEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sToolbarEEEERKS0_v$shim")]
// 0xf24fd0 — __ZN3RBX4Name9doDeclareILZNS_8sToolbarEEEERKS0_v$shim
// type: int()
pub fn stub_0xf24fd0() -> ! { todo!("0xf24fd0 __ZN3RBX4Name9doDeclareILZNS_8sToolbarEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_7sButtonEEEERKS0_v$shim")]
// 0xf24fdc — __ZN3RBX4Name9doDeclareILZNS_7sButtonEEEERKS0_v$shim
// type: int()
pub fn stub_0xf24fdc() -> ! { todo!("0xf24fdc __ZN3RBX4Name9doDeclareILZNS_7sButtonEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_7sPluginEEEERKS0_v$shim")]
// 0xf24fe8 — __ZN3RBX4Name9doDeclareILZNS_7sPluginEEEERKS0_v$shim
// type: int()
pub fn stub_0xf24fe8() -> ! { todo!("0xf24fe8 __ZN3RBX4Name9doDeclareILZNS_7sPluginEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sPluginMouseEEEERKS0_v$shim")]
// 0xf2500c — __ZN3RBX4Name9doDeclareILZNS_12sPluginMouseEEEERKS0_v$shim
// type: int()
pub fn stub_0xf2500c() -> ! { todo!("0xf2500c __ZN3RBX4Name9doDeclareILZNS_12sPluginMouseEEEERKS0_v$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX21PersonalServerService13PrivilegeTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf25018 — __ZNSt6vectorIN3RBX21PersonalServerService13PrivilegeTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int __fastcall(int, char *, _DWORD *)
pub fn stub_0xf25018() -> ! { todo!("0xf25018 __ZNSt6vectorIN3RBX21PersonalServerService13PrivilegeTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZNSt6vectorIS_IbSaIbEESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_$shim")]
// 0xf25024 — __ZNSt6vectorIS_IbSaIbEESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_$shim
// type: void __fastcall(_DWORD *, struct _Unwind_Exception *, unsigned int, int)
pub fn stub_0xf25024() -> ! { todo!("0xf25024 __ZNSt6vectorIS_IbSaIbEESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_$shim") }

#[doc(alias = "__ZNSt6vectorIS_IbSaIbEESaIS1_EE15_M_erase_at_endEPS1_$shim")]
// 0xf25030 — __ZNSt6vectorIS_IbSaIbEESaIS1_EE15_M_erase_at_endEPS1_$shim
// type: void __fastcall(int, void **)
pub fn stub_0xf25030() -> ! { todo!("0xf25030 __ZNSt6vectorIS_IbSaIbEESaIS1_EE15_M_erase_at_endEPS1_$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_25sCacheableContentProviderEEEERKS0_v$shim")]
// 0xf2503c — __ZN3RBX4Name9doDeclareILZNS_25sCacheableContentProviderEEEERKS0_v$shim
// type: int()
pub fn stub_0xf2503c() -> ! { todo!("0xf2503c __ZN3RBX4Name9doDeclareILZNS_25sCacheableContentProviderEEEERKS0_v$shim") }

#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX24CacheableContentProviderEEENS6_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEESsENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSI_ISsEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
// 0xf25048 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX24CacheableContentProviderEEENS6_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEESsENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSI_ISsEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
pub fn stub_0xf25048() -> ! { todo!("0xf25048 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX24CacheableContentProviderEEENS6_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEESsENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSI_ISsEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim") }

#[doc(alias = "__ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX24CacheableContentProviderEEERKSsENS3_5list2INS3_5valueIS8_EENS_3argILi1EEEEEEEE12manage_smallERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE$shim")]
// 0xf25054 — __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX24CacheableContentProviderEEERKSsENS3_5list2INS3_5valueIS8_EENS_3argILi1EEEEEEEE12manage_smallERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE$shim
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
pub fn stub_0xf25054() -> ! { todo!("0xf25054 __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX24CacheableContentProviderEEERKSsENS3_5list2INS3_5valueIS8_EENS_3argILi1EEEEEEEE12manage_smallERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE$shim") }

#[doc(alias = "__ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIN3RBX13TaskScheduler10StepResultEPFS7_NS_8weak_ptrINS5_24CacheableContentProviderEEERKSsNS_10shared_ptrISB_EEENS3_5list3INS3_5valueISA_EENS_3argILi1EEENSK_ILi2EEEEEEEE12manage_smallERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE$shim")]
// 0xf25060 — __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIN3RBX13TaskScheduler10StepResultEPFS7_NS_8weak_ptrINS5_24CacheableContentProviderEEERKSsNS_10shared_ptrISB_EEENS3_5list3INS3_5valueISA_EENS_3argILi1EEENSK_ILi2EEEEEEEE12manage_smallERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE$shim
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
pub fn stub_0xf25060() -> ! { todo!("0xf25060 __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIN3RBX13TaskScheduler10StepResultEPFS7_NS_8weak_ptrINS5_24CacheableContentProviderEEERKSsNS_10shared_ptrISB_EEENS3_5list3INS3_5valueISA_EENS_3argILi1EEENSK_ILi2EEEEEEEE12manage_smallERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE$shim") }

#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm$shim")]
// 0xf2506c — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm$shim
// type: int __fastcall(int, unsigned int)
pub fn stub_0xf2506c() -> ! { todo!("0xf2506c __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm$shim") }

#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm$shim")]
// 0xf25078 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm$shim
// type: void __fastcall(int, unsigned int)
pub fn stub_0xf25078() -> ! { todo!("0xf25078 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm$shim") }

#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISF_EESP_$shim")]
// 0xf25084 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISF_EESP_$shim
// type: int __fastcall(int, int, int)
pub fn stub_0xf25084() -> ! { todo!("0xf25084 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISF_EESP_$shim") }

#[doc(alias = "__ZNSt5dequeIN3RBX16BindableFunction10InvocationESaIS2_EE17_M_reallocate_mapEmb$shim")]
// 0xf25090 — __ZNSt5dequeIN3RBX16BindableFunction10InvocationESaIS2_EE17_M_reallocate_mapEmb$shim
// type: char *__fastcall(void **, unsigned int, int)
pub fn stub_0xf25090() -> ! { todo!("0xf25090 __ZNSt5dequeIN3RBX16BindableFunction10InvocationESaIS2_EE17_M_reallocate_mapEmb$shim") }

#[doc(alias = "__ZN3RBX10FixedArrayIPNS_17BuoyancyConnectorELm8EEixEm$shim")]
// 0xf250a8 — __ZN3RBX10FixedArrayIPNS_17BuoyancyConnectorELm8EEixEm$shim
// type: int __fastcall(int, int)
pub fn stub_0xf250a8() -> ! { todo!("0xf250a8 __ZN3RBX10FixedArrayIPNS_17BuoyancyConnectorELm8EEixEm$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN8sRagdollEEEERKS0_v$shim")]
// 0xf250b4 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN8sRagdollEEEERKS0_v$shim
// type: int()
pub fn stub_0xf250b4() -> ! { todo!("0xf250b4 __ZN3RBX4Name9doDeclareILZNS_5HUMAN8sRagdollEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN9sSwimmingEEEERKS0_v$shim")]
// 0xf250c0 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN9sSwimmingEEEERKS0_v$shim
// type: int()
pub fn stub_0xf250c0() -> ! { todo!("0xf250c0 __ZN3RBX4Name9doDeclareILZNS_5HUMAN9sSwimmingEEEERKS0_v$shim") }

#[doc(alias = "__ZNSt6vectorIPN3RBX21ConstraintSurfacePairESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf250cc — __ZNSt6vectorIPN3RBX21ConstraintSurfacePairESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int __fastcall(int, void *)
pub fn stub_0xf250cc() -> ! { todo!("0xf250cc __ZNSt6vectorIPN3RBX21ConstraintSurfacePairESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }
