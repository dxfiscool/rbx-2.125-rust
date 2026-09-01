//! core shard mw — 100 core stubs EA-sorted asc fallback not yet in rbx_core.
//! Source: `ida/export.json` (85545 funcs) EA-sorted asc, next 100 not yet in rbx_core (fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound; fallback 33887, 1823 uncovered before -> 1723 after, batch 0xf241e4..0xf248c8).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "__ZN3RBX10FixedArrayIPNS_13PolyConnectorELm40EEixEm$shim")]
// 0xf241e4 — __ZN3RBX10FixedArrayIPNS_13PolyConnectorELm40EEixEm$shim
// type: int __fastcall(int, int)
pub fn stub_0xf241e4() -> ! { todo!("0xf241e4 __ZN3RBX10FixedArrayIPNS_13PolyConnectorELm40EEixEm$shim") }

#[doc(alias = "__ZNK3RBX8EdgeList7getEdgeEi$shim")]
// 0xf241f0 — __ZNK3RBX8EdgeList7getEdgeEi$shim
// type: int __fastcall(RBX::EdgeList *, int)
pub fn stub_0xf241f0() -> ! { todo!("0xf241f0 __ZNK3RBX8EdgeList7getEdgeEi$shim") }

#[doc(alias = "__ZN3RBX4Body15getPV_Spin_LockEv$shim")]
// 0xf241fc — __ZN3RBX4Body15getPV_Spin_LockEv$shim
// type: int __fastcall(RBX::Body *)
pub fn stub_0xf241fc() -> ! { todo!("0xf241fc __ZN3RBX4Body15getPV_Spin_LockEv$shim") }

#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE29safe_static_do_get_staticDataEv$shim")]
// 0xf24208 — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE29safe_static_do_get_staticDataEv$shim
// type: int()
pub fn stub_0xf24208() -> ! { todo!("0xf24208 __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE29safe_static_do_get_staticDataEv$shim") }

#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE29safe_static_do_get_staticDataEv$shim")]
// 0xf24214 — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE29safe_static_do_get_staticDataEv$shim
// type: int()
pub fn stub_0xf24214() -> ! { todo!("0xf24214 __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE29safe_static_do_get_staticDataEv$shim") }

#[doc(alias = "__ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf2422c — __ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int __fastcall(int, void *)
pub fn stub_0xf2422c() -> ! { todo!("0xf2422c __ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZN3RBX11IWorldStage9getMetricENS0_10MetricTypeE$shim")]
// 0xf24238 — __ZN3RBX11IWorldStage9getMetricENS0_10MetricTypeE$shim
// type: int()
pub fn stub_0xf24238() -> ! { todo!("0xf24238 __ZN3RBX11IWorldStage9getMetricENS0_10MetricTypeE$shim") }

#[doc(alias = "__ZNSt5dequeIPN3RBX8AssemblyESaIS2_EE16_M_push_back_auxERKS2_$shim")]
// 0xf24244 — __ZNSt5dequeIPN3RBX8AssemblyESaIS2_EE16_M_push_back_auxERKS2_$shim
// type: int()
pub fn stub_0xf24244() -> ! { todo!("0xf24244 __ZNSt5dequeIPN3RBX8AssemblyESaIS2_EE16_M_push_back_auxERKS2_$shim") }

#[doc(alias = "__ZNSt6vectorIPN3RBX7ContactESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf24250 — __ZNSt6vectorIPN3RBX7ContactESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int __fastcall(int, void *)
pub fn stub_0xf24250() -> ! { todo!("0xf24250 __ZNSt6vectorIPN3RBX7ContactESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZNSt6vectorIPN3RBX8AssemblyESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf2425c — __ZNSt6vectorIPN3RBX8AssemblyESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int __fastcall(int, void *)
pub fn stub_0xf2425c() -> ! { todo!("0xf2425c __ZNSt6vectorIPN3RBX8AssemblyESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZNSt5dequeIPN3RBX8AssemblyESaIS2_EE17_M_reallocate_mapEmb$shim")]
// 0xf24274 — __ZNSt5dequeIPN3RBX8AssemblyESaIS2_EE17_M_reallocate_mapEmb$shim
// type: int()
pub fn stub_0xf24274() -> ! { todo!("0xf24274 __ZNSt5dequeIPN3RBX8AssemblyESaIS2_EE17_M_reallocate_mapEmb$shim") }

#[doc(alias = "__ZNK3RBX6Kernel9numBodiesEv$shim")]
// 0xf2428c — __ZNK3RBX6Kernel9numBodiesEv$shim
// type: int __fastcall(RBX::Kernel *)
pub fn stub_0xf2428c() -> ! { todo!("0xf2428c __ZNK3RBX6Kernel9numBodiesEv$shim") }

#[doc(alias = "___divsi3$shim")]
// 0xf24298 — ___divsi3$shim
// type: int __fastcall(int, int)
pub fn stub_0xf24298() -> ! { todo!("0xf24298 ___divsi3$shim") }

#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi1EFvSt4pairIPN3RBX9PrimitiveES5_EEEclES6_$shim")]
// 0xf242b0 — __ZN3rbx7signals16signal_with_argsILi1EFvSt4pairIPN3RBX9PrimitiveES5_EEEclES6_$shim
// type: int()
pub fn stub_0xf242b0() -> ! { todo!("0xf242b0 __ZN3rbx7signals16signal_with_argsILi1EFvSt4pairIPN3RBX9PrimitiveES5_EEEclES6_$shim") }

#[doc(alias = "__ZNSt6vectorIPN3RBX9Profiling12CodeProfilerESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_$shim")]
// 0xf242bc — __ZNSt6vectorIPN3RBX9Profiling12CodeProfilerESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_$shim
// type: int __fastcall(int, void *)
pub fn stub_0xf242bc() -> ! { todo!("0xf242bc __ZNSt6vectorIPN3RBX9Profiling12CodeProfilerESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_$shim") }

#[doc(alias = "__ZNSt6vectorIPN3RBX5JointESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf242c8 — __ZNSt6vectorIPN3RBX5JointESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int __fastcall(int, void *)
pub fn stub_0xf242c8() -> ! { todo!("0xf242c8 __ZNSt6vectorIPN3RBX5JointESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE24safe_static_do_get_mutexEv$shim")]
// 0xf242e0 — __ZN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE24safe_static_do_get_mutexEv$shim
// type: int __fastcall(int)
pub fn stub_0xf242e0() -> ! { todo!("0xf242e0 __ZN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE24safe_static_do_get_mutexEv$shim") }

#[doc(alias = "__ZNSt6vectorIiSaIiEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPiS1_EERKi$shim")]
// 0xf242f8 — __ZNSt6vectorIiSaIiEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPiS1_EERKi$shim
// type: int __fastcall(int, void *)
pub fn stub_0xf242f8() -> ! { todo!("0xf242f8 __ZNSt6vectorIiSaIiEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPiS1_EERKi$shim") }

#[doc(alias = "__ZN3rbx7signals6signalIFvP9lua_StateEE24safe_static_do_get_mutexEv$shim")]
// 0xf24370 — __ZN3rbx7signals6signalIFvP9lua_StateEE24safe_static_do_get_mutexEv$shim
// type: int()
pub fn stub_0xf24370() -> ! { todo!("0xf24370 __ZN3rbx7signals6signalIFvP9lua_StateEE24safe_static_do_get_mutexEv$shim") }

#[doc(alias = "__ZN3rbx7signals6signalIFvP9lua_StateEE4slot24safe_static_do_get_mutexEv$shim")]
// 0xf2437c — __ZN3rbx7signals6signalIFvP9lua_StateEE4slot24safe_static_do_get_mutexEv$shim
// type: int()
pub fn stub_0xf2437c() -> ! { todo!("0xf2437c __ZN3rbx7signals6signalIFvP9lua_StateEE4slot24safe_static_do_get_mutexEv$shim") }

#[doc(alias = "__ZNSt6vectorIN12MemoryBinder9IDREFItemESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim")]
// 0xf2440c — __ZNSt6vectorIN12MemoryBinder9IDREFItemESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim
// type: int __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *, int, int, int, boost::detail::sp_counted_base *, void *, int, int, int, int, int, int, int, void *, int)
pub fn stub_0xf2440c() -> ! { todo!("0xf2440c __ZNSt6vectorIN12MemoryBinder9IDREFItemESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim") }

#[doc(alias = "__ZNSt6vectorImSaImEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPmS1_EERKm$shim")]
// 0xf24418 — __ZNSt6vectorImSaImEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPmS1_EERKm$shim
// type: int __fastcall(int, void *)
pub fn stub_0xf24418() -> ! { todo!("0xf24418 __ZNSt6vectorImSaImEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPmS1_EERKm$shim") }

#[doc(alias = "__ZN5boost16exception_detail19error_info_injectorINS_6spirit7classic12parser_errorISsN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEEEEED2Ev$shim")]
// 0xf24424 — __ZN5boost16exception_detail19error_info_injectorINS_6spirit7classic12parser_errorISsN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEEEEED2Ev$shim
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0xf24424() -> ! { todo!("0xf24424 __ZN5boost16exception_detail19error_info_injectorINS_6spirit7classic12parser_errorISsN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEEEEED2Ev$shim") }

#[doc(alias = "__ZNSt6vectorIPN5boost6spirit7classic4impl19grammar_helper_baseINS2_7grammarINS0_13property_tree11json_parser12json_grammarINS6_11basic_ptreeISsSsSt4lessISsEEEEENS2_14parser_contextINS2_5nil_tEEEEEEESaISJ_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPSJ_SL_EERKSJ_$shim")]
// 0xf24430 — __ZNSt6vectorIPN5boost6spirit7classic4impl19grammar_helper_baseINS2_7grammarINS0_13property_tree11json_parser12json_grammarINS6_11basic_ptreeISsSsSt4lessISsEEEEENS2_14parser_contextINS2_5nil_tEEEEEEESaISJ_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPSJ_SL_EERKSJ_$shim
// type: int __fastcall(int, void *)
pub fn stub_0xf24430() -> ! { todo!("0xf24430 __ZNSt6vectorIPN5boost6spirit7classic4impl19grammar_helper_baseINS2_7grammarINS0_13property_tree11json_parser12json_grammarINS6_11basic_ptreeISsSsSt4lessISsEEEEENS2_14parser_contextINS2_5nil_tEEEEEEESaISJ_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPSJ_SL_EERKSJ_$shim") }

#[doc(alias = "__ZNSs9push_backEc$shim")]
// 0xf2443c — __ZNSs9push_backEc$shim
// type: int __fastcall(std::string *, char)
pub fn stub_0xf2443c() -> ! { todo!("0xf2443c __ZNSs9push_backEc$shim") }

#[doc(alias = "__ZNK5boost6spirit7classic11alternativeINS1_6actionINS1_10differenceINS4_INS1_14anychar_parserENS1_6strlitIPKcEEEES9_EENS_13property_tree11json_parser7contextINSC_11basic_ptreeISsSsSt4lessISsEEEE6a_charEEENS1_8sequenceINS1_5chlitIcEENS1_16assertive_parserISsNS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS2_INS2_INS1_12space_parserENS1_13confix_parserIS9_NS1_11kleene_starIS5_EENS2_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENS13_IS9_S15_S9_S19_S1A_S1B_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES1N_EEEEEEE5parseIS1M_EENS1_13parser_resultIS1R_T_E4typeERKS1U_$shim")]
// 0xf24448 — __ZNK5boost6spirit7classic11alternativeINS1_6actionINS1_10differenceINS4_INS1_14anychar_parserENS1_6strlitIPKcEEEES9_EENS_13property_tree11json_parser7contextINSC_11basic_ptreeISsSsSt4lessISsEEEE6a_charEEENS1_8sequenceINS1_5chlitIcEENS1_16assertive_parserISsNS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS2_INS2_INS1_12space_parserENS1_13confix_parserIS9_NS1_11kleene_starIS5_EENS2_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENS13_IS9_S15_S9_S19_S1A_S1B_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES1N_EEEEEEE5parseIS1M_EENS1_13parser_resultIS1R_T_E4typeERKS1U_$shim
// type: int()
pub fn stub_0xf24448() -> ! { todo!("0xf24448 __ZNK5boost6spirit7classic11alternativeINS1_6actionINS1_10differenceINS4_INS1_14anychar_parserENS1_6strlitIPKcEEEES9_EENS_13property_tree11json_parser7contextINSC_11basic_ptreeISsSsSt4lessISsEEEE6a_charEEENS1_8sequenceINS1_5chlitIcEENS1_16assertive_parserISsNS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS2_INS2_INS1_12space_parserENS1_13confix_parserIS9_NS1_11kleene_starIS5_EENS2_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENS13_IS9_S15_S9_S19_S1A_S1B_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES1N_EEEEEEE5parseIS1M_EENS1_13parser_resultIS1R_T_E4typeERKS1U_$shim") }

#[doc(alias = "__ZN5boost6spirit7classic4impl15concrete_parserINS1_8sequenceINS4_INS4_INS1_8optionalINS1_5chlitIcEEEENS1_11alternativeIS7_NS4_INS1_5rangeIcEENS1_11kleene_starINS1_12digit_parserEEEEEEEEENS5_INS4_IS7_NS1_8positiveISD_EEEEEEEENS5_INS4_INS4_INS1_5chsetIcEENS5_ISO_EEEESJ_EEEEEENS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyINS9_INS9_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENSC_INS1_14anychar_parserEEENS9_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENS15_IS19_S1B_S19_S1F_S1G_S1H_EEEENS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tEED2Ev$shim")]
// 0xf24454 — __ZN5boost6spirit7classic4impl15concrete_parserINS1_8sequenceINS4_INS4_INS1_8optionalINS1_5chlitIcEEEENS1_11alternativeIS7_NS4_INS1_5rangeIcEENS1_11kleene_starINS1_12digit_parserEEEEEEEEENS5_INS4_IS7_NS1_8positiveISD_EEEEEEEENS5_INS4_INS4_INS1_5chsetIcEENS5_ISO_EEEESJ_EEEEEENS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyINS9_INS9_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENSC_INS1_14anychar_parserEEENS9_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENS15_IS19_S1B_S19_S1F_S1G_S1H_EEEENS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tEED2Ev$shim
// type: int()
pub fn stub_0xf24454() -> ! { todo!("0xf24454 __ZN5boost6spirit7classic4impl15concrete_parserINS1_8sequenceINS4_INS4_INS1_8optionalINS1_5chlitIcEEEENS1_11alternativeIS7_NS4_INS1_5rangeIcEENS1_11kleene_starINS1_12digit_parserEEEEEEEEENS5_INS4_IS7_NS1_8positiveISD_EEEEEEEENS5_INS4_INS4_INS1_5chsetIcEENS5_ISO_EEEESJ_EEEEEENS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyINS9_INS9_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENSC_INS1_14anychar_parserEEENS9_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENS15_IS19_S1B_S19_S1F_S1G_S1H_EEEENS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tEED2Ev$shim") }

#[doc(alias = "__ZN5boost11multi_index6detail13ordered_indexINS0_6memberISt4pairIKSsNS_13property_tree11basic_ptreeISsSsSt4lessISsEEEES5_XadL_ZNSB_5firstEEEEES9_NS1_9nth_layerILi2ESB_NS0_10indexed_byINS0_9sequencedINS0_3tagIN4mpl_2naESI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_EEEENS0_18ordered_non_uniqueINSG_INSA_4subs7by_nameESI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_EESC_S9_EESI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_EESaISB_EEENS_3mpl6v_itemISN_NST_7vector0ISI_EELi0EEENS1_22ordered_non_unique_tagEE5copy_ERKSZ_RKNS1_8copy_mapINS1_20sequenced_index_nodeINS1_18ordered_index_nodeINS1_15index_node_baseISB_SR_EEEEEESR_EE$shim")]
// 0xf24460 — __ZN5boost11multi_index6detail13ordered_indexINS0_6memberISt4pairIKSsNS_13property_tree11basic_ptreeISsSsSt4lessISsEEEES5_XadL_ZNSB_5firstEEEEES9_NS1_9nth_layerILi2ESB_NS0_10indexed_byINS0_9sequencedINS0_3tagIN4mpl_2naESI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_EEEENS0_18ordered_non_uniqueINSG_INSA_4subs7by_nameESI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_EESC_S9_EESI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_EESaISB_EEENS_3mpl6v_itemISN_NST_7vector0ISI_EELi0EEENS1_22ordered_non_unique_tagEE5copy_ERKSZ_RKNS1_8copy_mapINS1_20sequenced_index_nodeINS1_18ordered_index_nodeINS1_15index_node_baseISB_SR_EEEEEESR_EE$shim
// type: int()
pub fn stub_0xf24460() -> ! { todo!("0xf24460 __ZN5boost11multi_index6detail13ordered_indexINS0_6memberISt4pairIKSsNS_13property_tree11basic_ptreeISsSsSt4lessISsEEEES5_XadL_ZNSB_5firstEEEEES9_NS1_9nth_layerILi2ESB_NS0_10indexed_byINS0_9sequencedINS0_3tagIN4mpl_2naESI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_EEEENS0_18ordered_non_uniqueINSG_INSA_4subs7by_nameESI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_EESC_S9_EESI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_SI_EESaISB_EEENS_3mpl6v_itemISN_NST_7vector0ISI_EELi0EEENS1_22ordered_non_unique_tagEE5copy_ERKSZ_RKNS1_8copy_mapINS1_20sequenced_index_nodeINS1_18ordered_index_nodeINS1_15index_node_baseISB_SR_EEEEEESR_EE$shim") }

#[doc(alias = "__ZN5boost11multi_index6detail23ordered_index_node_implISaIcEE9rebalanceEPS4_NS1_34ordered_index_node_compressed_baseIS3_E10parent_refE$shim")]
// 0xf2446c — __ZN5boost11multi_index6detail23ordered_index_node_implISaIcEE9rebalanceEPS4_NS1_34ordered_index_node_compressed_baseIS3_E10parent_refE$shim
// type: int __fastcall(int)
pub fn stub_0xf2446c() -> ! { todo!("0xf2446c __ZN5boost11multi_index6detail23ordered_index_node_implISaIcEE9rebalanceEPS4_NS1_34ordered_index_node_compressed_baseIS3_E10parent_refE$shim") }

#[doc(alias = "__ZNK5boost6spirit7classic16assertive_parserISsNS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyINS1_11alternativeINSE_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSE_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSG_ISK_SN_SK_SR_SS_ST_EEEENS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES14_EEE5parseIS13_EENS1_13parser_resultIS16_T_E4typeERKS19_$shim")]
// 0xf24478 — __ZNK5boost6spirit7classic16assertive_parserISsNS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyINS1_11alternativeINSE_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSE_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSG_ISK_SN_SK_SR_SS_ST_EEEENS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES14_EEE5parseIS13_EENS1_13parser_resultIS16_T_E4typeERKS19_$shim
// type: int()
pub fn stub_0xf24478() -> ! { todo!("0xf24478 __ZNK5boost6spirit7classic16assertive_parserISsNS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyINS1_11alternativeINSE_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSE_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSG_ISK_SN_SK_SR_SS_ST_EEEENS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES14_EEE5parseIS13_EENS1_13parser_resultIS16_T_E4typeERKS19_$shim") }

#[doc(alias = "__ZNK5boost6spirit7classic8sequenceINS2_INS1_16assertive_parserISsNS1_6actionINS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyINS1_11alternativeINSG_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSG_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSI_ISM_SP_SM_ST_SU_SV_EEEENS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES16_EENS_13property_tree11json_parser7contextINS18_11basic_ptreeISsSsSt4lessISsEEEE6a_nameEEEEENS3_ISsNS1_5chlitIcEEEEEENS3_ISsS17_EEE5parseIS15_EENS1_13parser_resultIS1O_T_E4typeERKS1R_$shim")]
// 0xf24484 — __ZNK5boost6spirit7classic8sequenceINS2_INS1_16assertive_parserISsNS1_6actionINS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyINS1_11alternativeINSG_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSG_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSI_ISM_SP_SM_ST_SU_SV_EEEENS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES16_EENS_13property_tree11json_parser7contextINS18_11basic_ptreeISsSsSt4lessISsEEEE6a_nameEEEEENS3_ISsNS1_5chlitIcEEEEEENS3_ISsS17_EEE5parseIS15_EENS1_13parser_resultIS1O_T_E4typeERKS1R_$shim
// type: int()
pub fn stub_0xf24484() -> ! { todo!("0xf24484 __ZNK5boost6spirit7classic8sequenceINS2_INS1_16assertive_parserISsNS1_6actionINS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyINS1_11alternativeINSG_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSG_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSI_ISM_SP_SM_ST_SU_SV_EEEENS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES16_EENS_13property_tree11json_parser7contextINS18_11basic_ptreeISsSsSt4lessISsEEEE6a_nameEEEEENS3_ISsNS1_5chlitIcEEEEEENS3_ISsS17_EEE5parseIS15_EENS1_13parser_resultIS1O_T_E4typeERKS1R_$shim") }

#[doc(alias = "__ZNK5boost6spirit7classic8sequenceINS1_6actionINS1_5chlitIcEENS_13property_tree11json_parser7contextINS6_11basic_ptreeISsSsSt4lessISsEEEE10a_object_sEEENS1_11alternativeINS3_IS5_NSD_10a_object_eEEENS2_INS1_11list_parserINS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyINSG_INSG_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSG_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSW_IS10_S13_S10_S17_S18_S19_EEEENS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES1K_EES5_NS1_16no_list_endtokenENS1_21plain_parser_categoryEEENS1_16assertive_parserISsSI_EEEEEEE5parseIS1J_EENS1_13parser_resultIS1T_T_E4typeERKS1W_$shim")]
// 0xf24490 — __ZNK5boost6spirit7classic8sequenceINS1_6actionINS1_5chlitIcEENS_13property_tree11json_parser7contextINS6_11basic_ptreeISsSsSt4lessISsEEEE10a_object_sEEENS1_11alternativeINS3_IS5_NSD_10a_object_eEEENS2_INS1_11list_parserINS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyINSG_INSG_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSG_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSW_IS10_S13_S10_S17_S18_S19_EEEENS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES1K_EES5_NS1_16no_list_endtokenENS1_21plain_parser_categoryEEENS1_16assertive_parserISsSI_EEEEEEE5parseIS1J_EENS1_13parser_resultIS1T_T_E4typeERKS1W_$shim
// type: int()
pub fn stub_0xf24490() -> ! { todo!("0xf24490 __ZNK5boost6spirit7classic8sequenceINS1_6actionINS1_5chlitIcEENS_13property_tree11json_parser7contextINS6_11basic_ptreeISsSsSt4lessISsEEEE10a_object_sEEENS1_11alternativeINS3_IS5_NSD_10a_object_eEEENS2_INS1_11list_parserINS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyINSG_INSG_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSG_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSW_IS10_S13_S10_S17_S18_S19_EEEENS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES1K_EES5_NS1_16no_list_endtokenENS1_21plain_parser_categoryEEENS1_16assertive_parserISsSI_EEEEEEE5parseIS1J_EENS1_13parser_resultIS1T_T_E4typeERKS1W_$shim") }

#[doc(alias = "__ZNSt6vectorIPN5boost13property_tree11basic_ptreeISsSsSt4lessISsEEESaIS6_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS6_S8_EERKS6_$shim")]
// 0xf2449c — __ZNSt6vectorIPN5boost13property_tree11basic_ptreeISsSsSt4lessISsEEESaIS6_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS6_S8_EERKS6_$shim
// type: int __fastcall(int, void *)
pub fn stub_0xf2449c() -> ! { todo!("0xf2449c __ZNSt6vectorIPN5boost13property_tree11basic_ptreeISsSsSt4lessISsEEESaIS6_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS6_S8_EERKS6_$shim") }

#[doc(alias = "__ZN5boost16exception_detail19error_info_injectorINS_13property_tree11json_parser17json_parser_errorEED2Ev$shim")]
// 0xf244a8 — __ZN5boost16exception_detail19error_info_injectorINS_13property_tree11json_parser17json_parser_errorEED2Ev$shim
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0xf244a8() -> ! { todo!("0xf244a8 __ZN5boost16exception_detail19error_info_injectorINS_13property_tree11json_parser17json_parser_errorEED2Ev$shim") }

#[doc(alias = "__ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_13property_tree11json_parser17json_parser_errorEEEE5cloneEv$shim")]
// 0xf244b4 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_13property_tree11json_parser17json_parser_errorEEEE5cloneEv$shim
// type: int()
pub fn stub_0xf244b4() -> ! { todo!("0xf244b4 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_13property_tree11json_parser17json_parser_errorEEEE5cloneEv$shim") }

#[doc(alias = "__ZNSt6vectorIcSaIcEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPcS1_EERKc$shim")]
// 0xf244c0 — __ZNSt6vectorIcSaIcEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPcS1_EERKc$shim
// type: int __fastcall(int, void *)
pub fn stub_0xf244c0() -> ! { todo!("0xf244c0 __ZNSt6vectorIcSaIcEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPcS1_EERKc$shim") }

#[doc(alias = "__ZN5boost16exception_detail19error_info_injectorINS_13property_tree14ptree_bad_dataEED2Ev$shim")]
// 0xf244cc — __ZN5boost16exception_detail19error_info_injectorINS_13property_tree14ptree_bad_dataEED2Ev$shim
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0xf244cc() -> ! { todo!("0xf244cc __ZN5boost16exception_detail19error_info_injectorINS_13property_tree14ptree_bad_dataEED2Ev$shim") }

#[doc(alias = "__ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_13property_tree14ptree_bad_dataEEEE5cloneEv$shim")]
// 0xf244d8 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_13property_tree14ptree_bad_dataEEEE5cloneEv$shim
// type: char *__fastcall(int)
pub fn stub_0xf244d8() -> ! { todo!("0xf244d8 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_13property_tree14ptree_bad_dataEEEE5cloneEv$shim") }

#[doc(alias = "__ZSt2wsIcSt11char_traitsIcEERSt13basic_istreamIT_T0_ES6_$shim")]
// 0xf244e4 — __ZSt2wsIcSt11char_traitsIcEERSt13basic_istreamIT_T0_ES6_$shim
// type: int()
pub fn stub_0xf244e4() -> ! { todo!("0xf244e4 __ZSt2wsIcSt11char_traitsIcEERSt13basic_istreamIT_T0_ES6_$shim") }

#[doc(alias = "__ZNSt5dequeIP10XmlElementSaIS1_EE16_M_push_back_auxERKS1_$shim")]
// 0xf244f0 — __ZNSt5dequeIP10XmlElementSaIS1_EE16_M_push_back_auxERKS1_$shim
// type: int()
pub fn stub_0xf244f0() -> ! { todo!("0xf244f0 __ZNSt5dequeIP10XmlElementSaIS1_EE16_M_push_back_auxERKS1_$shim") }

#[doc(alias = "__ZNSt5dequeIP10XmlElementSaIS1_EE17_M_reallocate_mapEmb$shim")]
// 0xf244fc — __ZNSt5dequeIP10XmlElementSaIS1_EE17_M_reallocate_mapEmb$shim
// type: int()
pub fn stub_0xf244fc() -> ! { todo!("0xf244fc __ZNSt5dequeIP10XmlElementSaIS1_EE17_M_reallocate_mapEmb$shim") }

#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_5TeamsEEEPT_v$shim")]
// 0xf24508 — __ZNK3RBX15ServiceProvider4findINS_5TeamsEEEPT_v$shim
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *, int, int, int, int, int, int, int, int, int)
pub fn stub_0xf24508() -> ! { todo!("0xf24508 __ZNK3RBX15ServiceProvider4findINS_5TeamsEEEPT_v$shim") }

#[doc(alias = "__ZNSt5dequeIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EE16_M_pop_front_auxEv$shim")]
// 0xf24514 — __ZNSt5dequeIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EE16_M_pop_front_auxEv$shim
// type: int()
pub fn stub_0xf24514() -> ! { todo!("0xf24514 __ZNSt5dequeIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EE16_M_pop_front_auxEv$shim") }

#[doc(alias = "__ZNSt5dequeIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EE16_M_push_back_auxERKS4_$shim")]
// 0xf24520 — __ZNSt5dequeIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EE16_M_push_back_auxERKS4_$shim
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, void *, int)
pub fn stub_0xf24520() -> ! { todo!("0xf24520 __ZNSt5dequeIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EE16_M_push_back_auxERKS4_$shim") }

#[doc(alias = "__ZN3RBX8ChatLineD2Ev$shim")]
// 0xf2452c — __ZN3RBX8ChatLineD2Ev$shim
// type: void __fastcall(RBX::ChatLine *)
pub fn stub_0xf2452c() -> ! { todo!("0xf2452c __ZN3RBX8ChatLineD2Ev$shim") }

#[doc(alias = "__ZNSt5dequeIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EE17_M_reallocate_mapEmb$shim")]
// 0xf24550 — __ZNSt5dequeIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EE17_M_reallocate_mapEmb$shim
// type: int()
pub fn stub_0xf24550() -> ! { todo!("0xf24550 __ZNSt5dequeIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EE17_M_reallocate_mapEmb$shim") }

#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10ChatOutputERKNS4_9HeartbeatEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_$shim")]
// 0xf2458c — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10ChatOutputERKNS4_9HeartbeatEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_$shim
// type: int()
pub fn stub_0xf2458c() -> ! { todo!("0xf2458c __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10ChatOutputERKNS4_9HeartbeatEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_$shim") }

#[doc(alias = "_lrint$shim")]
// 0xf245b0 — _lrint$shim
// type: __int32 __cdecl(double)
pub fn stub_0xf245b0() -> ! { todo!("0xf245b0 _lrint$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sGuiItemEEEERKS0_v$shim")]
// 0xf245bc — __ZN3RBX4Name9doDeclareILZNS_8sGuiItemEEEERKS0_v$shim
// type: int()
pub fn stub_0xf245bc() -> ! { todo!("0xf245bc __ZN3RBX4Name9doDeclareILZNS_8sGuiItemEEEERKS0_v$shim") }

#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX12GuiDrawImageEEENS0_5list1INS0_5valueIPS5_EEEEEclEv$shim")]
// 0xf245c8 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX12GuiDrawImageEEENS0_5list1INS0_5valueIPS5_EEEEEclEv$shim
// type: int()
pub fn stub_0xf245c8() -> ! { todo!("0xf245c8 __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX12GuiDrawImageEEENS0_5list1INS0_5valueIPS5_EEEEEclEv$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN5sDeadEEEERKS0_v$shim")]
// 0xf245d4 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN5sDeadEEEERKS0_v$shim
// type: int()
pub fn stub_0xf245d4() -> ! { todo!("0xf245d4 __ZN3RBX4Name9doDeclareILZNS_5HUMAN5sDeadEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN12sFallingDownEEEERKS0_v$shim")]
// 0xf245e0 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN12sFallingDownEEEERKS0_v$shim
// type: int()
pub fn stub_0xf245e0() -> ! { todo!("0xf245e0 __ZN3RBX4Name9doDeclareILZNS_5HUMAN12sFallingDownEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN7sFlyingEEEERKS0_v$shim")]
// 0xf245ec — __ZN3RBX4Name9doDeclareILZNS_5HUMAN7sFlyingEEEERKS0_v$shim
// type: int()
pub fn stub_0xf245ec() -> ! { todo!("0xf245ec __ZN3RBX4Name9doDeclareILZNS_5HUMAN7sFlyingEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN9sFreefallEEEERKS0_v$shim")]
// 0xf245f8 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN9sFreefallEEEERKS0_v$shim
// type: int()
pub fn stub_0xf245f8() -> ! { todo!("0xf245f8 __ZN3RBX4Name9doDeclareILZNS_5HUMAN9sFreefallEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN10sGettingUpEEEERKS0_v$shim")]
// 0xf24604 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN10sGettingUpEEEERKS0_v$shim
// type: int()
pub fn stub_0xf24604() -> ! { todo!("0xf24604 __ZN3RBX4Name9doDeclareILZNS_5HUMAN10sGettingUpEEEERKS0_v$shim") }

#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi1EFvfEEclEf$shim")]
// 0xf24610 — __ZN3rbx7signals16signal_with_argsILi1EFvfEEclEf$shim
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
pub fn stub_0xf24610() -> ! { todo!("0xf24610 __ZN3rbx7signals16signal_with_argsILi1EFvfEEclEf$shim") }

#[doc(alias = "__ZNK3RBX15ServiceProvider6createINS_13ContentFilterEEEPT_v$shim")]
// 0xf24634 — __ZNK3RBX15ServiceProvider6createINS_13ContentFilterEEEPT_v$shim
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0xf24634() -> ! { todo!("0xf24634 __ZNK3RBX15ServiceProvider6createINS_13ContentFilterEEEPT_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sHumanoidEEEERKS0_v$shim")]
// 0xf24664 — __ZN3RBX4Name9doDeclareILZNS_9sHumanoidEEEERKS0_v$shim
// type: int()
pub fn stub_0xf24664() -> ! { todo!("0xf24664 __ZN3RBX4Name9doDeclareILZNS_9sHumanoidEEEERKS0_v$shim") }

#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX8HumanoidEEENS0_5list1INS0_5valueIPS5_EEEEEclEv$shim")]
// 0xf24670 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX8HumanoidEEENS0_5list1INS0_5valueIPS5_EEEEEclEv$shim
// type: int()
pub fn stub_0xf24670() -> ! { todo!("0xf24670 __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX8HumanoidEEENS0_5list1INS0_5valueIPS5_EEEEEclEv$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX8Humanoid6StatusESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf2467c — __ZNSt6vectorIN3RBX8Humanoid6StatusESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int()
pub fn stub_0xf2467c() -> ! { todo!("0xf2467c __ZNSt6vectorIN3RBX8Humanoid6StatusESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX8Humanoid13NameOcclusionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf24688 — __ZNSt6vectorIN3RBX8Humanoid13NameOcclusionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int()
pub fn stub_0xf24688() -> ! { todo!("0xf24688 __ZNSt6vectorIN3RBX8Humanoid13NameOcclusionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE24safe_static_do_get_mutexEv$shim")]
// 0xf246c4 — __ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE24safe_static_do_get_mutexEv$shim
// type: int()
pub fn stub_0xf246c4() -> ! { todo!("0xf246c4 __ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE24safe_static_do_get_mutexEv$shim") }

#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi1EFvN3RBX8Humanoid6StatusEEEclES4_$shim")]
// 0xf246dc — __ZN3rbx7signals16signal_with_argsILi1EFvN3RBX8Humanoid6StatusEEEclES4_$shim
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
pub fn stub_0xf246dc() -> ! { todo!("0xf246dc __ZN3rbx7signals16signal_with_argsILi1EFvN3RBX8Humanoid6StatusEEEclES4_$shim") }

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE13disconnectAllEv$shim")]
// 0xf246e8 — __ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE13disconnectAllEv$shim
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0xf246e8() -> ! { todo!("0xf246e8 __ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE13disconnectAllEv$shim") }

#[doc(alias = "__ZNK5boost9function1IvN3RBX8Humanoid6StatusEEclES3_$shim")]
// 0xf24700 — __ZNK5boost9function1IvN3RBX8Humanoid6StatusEEclES3_$shim
// type: int()
pub fn stub_0xf24700() -> ! { todo!("0xf24700 __ZNK5boost9function1IvN3RBX8Humanoid6StatusEEclES3_$shim") }

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE4slot24safe_static_do_get_mutexEv$shim")]
// 0xf2470c — __ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE4slot24safe_static_do_get_mutexEv$shim
// type: int()
pub fn stub_0xf2470c() -> ! { todo!("0xf2470c __ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE4slot24safe_static_do_get_mutexEv$shim") }

#[doc(alias = "__ZN3rbx7signals6signalIFvfEE13disconnectAllEv$shim")]
// 0xf24724 — __ZN3rbx7signals6signalIFvfEE13disconnectAllEv$shim
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0xf24724() -> ! { todo!("0xf24724 __ZN3rbx7signals6signalIFvfEE13disconnectAllEv$shim") }

#[doc(alias = "__ZNK5boost9function1IvfEclEf$shim")]
// 0xf2473c — __ZNK5boost9function1IvfEclEf$shim
// type: int()
pub fn stub_0xf2473c() -> ! { todo!("0xf2473c __ZNK5boost9function1IvfEclEf$shim") }

#[doc(alias = "__ZNSt6vectorIPN3RBX9PrimitiveESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf24754 — __ZNSt6vectorIPN3RBX9PrimitiveESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int __fastcall(int, void *)
pub fn stub_0xf24754() -> ! { todo!("0xf24754 __ZNSt6vectorIPN3RBX9PrimitiveESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZN5boost10shared_ptrIN3RBX5HUMAN13HumanoidStateEE5resetIS3_EEvPT_$shim")]
// 0xf24760 — __ZN5boost10shared_ptrIN3RBX5HUMAN13HumanoidStateEE5resetIS3_EEvPT_$shim
// type: int()
pub fn stub_0xf24760() -> ! { todo!("0xf24760 __ZN5boost10shared_ptrIN3RBX5HUMAN13HumanoidStateEE5resetIS3_EEvPT_$shim") }

#[doc(alias = "__ZNSt6vectorIN3rbx7signals10connectionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf2476c — __ZNSt6vectorIN3rbx7signals10connectionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int()
pub fn stub_0xf2476c() -> ! { todo!("0xf2476c __ZNSt6vectorIN3rbx7signals10connectionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_15GeometryServiceEEEPT_v$shim")]
// 0xf24778 — __ZNK3RBX15ServiceProvider4findINS_15GeometryServiceEEEPT_v$shim
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0xf24778() -> ! { todo!("0xf24778 __ZNK3RBX15ServiceProvider4findINS_15GeometryServiceEEEPT_v$shim") }

#[doc(alias = "__ZN5boost26intrusive_ptr_add_weak_refIN3rbx7signals10connection5islotEiLi0ELi0EEEvPKNS1_20intrusive_ptr_targetIT_T0_XT1_EXT2_EEE$shim")]
// 0xf24784 — __ZN5boost26intrusive_ptr_add_weak_refIN3rbx7signals10connection5islotEiLi0ELi0EEEvPKNS1_20intrusive_ptr_targetIT_T0_XT1_EXT2_EEE$shim
// type: int __fastcall(int)
pub fn stub_0xf24784() -> ! { todo!("0xf24784 __ZN5boost26intrusive_ptr_add_weak_refIN3rbx7signals10connection5islotEiLi0ELi0EEEvPKNS1_20intrusive_ptr_targetIT_T0_XT1_EXT2_EEE$shim") }

#[doc(alias = "__ZN3RBX4Name7declareILZNS_5HUMAN9sClimbingEEEERKS0_v$shim")]
// 0xf24790 — __ZN3RBX4Name7declareILZNS_5HUMAN9sClimbingEEEERKS0_v$shim
// type: int()
pub fn stub_0xf24790() -> ! { todo!("0xf24790 __ZN3RBX4Name7declareILZNS_5HUMAN9sClimbingEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN9sClimbingEEEERKS0_v$shim")]
// 0xf2479c — __ZN3RBX4Name9doDeclareILZNS_5HUMAN9sClimbingEEEERKS0_v$shim
// type: int()
pub fn stub_0xf2479c() -> ! { todo!("0xf2479c __ZN3RBX4Name9doDeclareILZNS_5HUMAN9sClimbingEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN8sJumpingEEEERKS0_v$shim")]
// 0xf247a8 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN8sJumpingEEEERKS0_v$shim
// type: int()
pub fn stub_0xf247a8() -> ! { todo!("0xf247a8 __ZN3RBX4Name9doDeclareILZNS_5HUMAN8sJumpingEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN20sMovingNoPhysicsBaseEEEERKS0_v$shim")]
// 0xf247b4 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN20sMovingNoPhysicsBaseEEEERKS0_v$shim
// type: int()
pub fn stub_0xf247b4() -> ! { todo!("0xf247b4 __ZN3RBX4Name9doDeclareILZNS_5HUMAN20sMovingNoPhysicsBaseEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN8sRunningEEEERKS0_v$shim")]
// 0xf247c0 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN8sRunningEEEERKS0_v$shim
// type: int()
pub fn stub_0xf247c0() -> ! { todo!("0xf247c0 __ZN3RBX4Name9doDeclareILZNS_5HUMAN8sRunningEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN13sRunningSlaveEEEERKS0_v$shim")]
// 0xf247cc — __ZN3RBX4Name9doDeclareILZNS_5HUMAN13sRunningSlaveEEEERKS0_v$shim
// type: int()
pub fn stub_0xf247cc() -> ! { todo!("0xf247cc __ZN3RBX4Name9doDeclareILZNS_5HUMAN13sRunningSlaveEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN7sLandedEEEERKS0_v$shim")]
// 0xf247d8 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN7sLandedEEEERKS0_v$shim
// type: int()
pub fn stub_0xf247d8() -> ! { todo!("0xf247d8 __ZN3RBX4Name9doDeclareILZNS_5HUMAN7sLandedEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN17sRunningNoPhysicsEEEERKS0_v$shim")]
// 0xf247e4 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN17sRunningNoPhysicsEEEERKS0_v$shim
// type: int()
pub fn stub_0xf247e4() -> ! { todo!("0xf247e4 __ZN3RBX4Name9doDeclareILZNS_5HUMAN17sRunningNoPhysicsEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN7sSeatedEEEERKS0_v$shim")]
// 0xf247f0 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN7sSeatedEEEERKS0_v$shim
// type: int()
pub fn stub_0xf247f0() -> ! { todo!("0xf247f0 __ZN3RBX4Name9doDeclareILZNS_5HUMAN7sSeatedEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN17sPlatformStandingEEEERKS0_v$shim")]
// 0xf247fc — __ZN3RBX4Name9doDeclareILZNS_5HUMAN17sPlatformStandingEEEERKS0_v$shim
// type: int()
pub fn stub_0xf247fc() -> ! { todo!("0xf247fc __ZN3RBX4Name9doDeclareILZNS_5HUMAN17sPlatformStandingEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN18sStrafingNoPhysicsEEEERKS0_v$shim")]
// 0xf24808 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN18sStrafingNoPhysicsEEEERKS0_v$shim
// type: int()
pub fn stub_0xf24808() -> ! { todo!("0xf24808 __ZN3RBX4Name9doDeclareILZNS_5HUMAN18sStrafingNoPhysicsEEEERKS0_v$shim") }

#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10TypesetterEEaSERKS3_$shim")]
// 0xf24814 — __ZN5boost10shared_ptrIN3RBX10TypesetterEEaSERKS3_$shim
// type: int()
pub fn stub_0xf24814() -> ! { todo!("0xf24814 __ZN5boost10shared_ptrIN3RBX10TypesetterEEaSERKS3_$shim") }

#[doc(alias = "__ZN3RBX11TextServiceD1Ev$shim")]
// 0xf24820 — __ZN3RBX11TextServiceD1Ev$shim
// type: void __fastcall(RBX::TextService *)
pub fn stub_0xf24820() -> ! { todo!("0xf24820 __ZN3RBX11TextServiceD1Ev$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX11TextService10YAlignmentESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf2482c — __ZNSt6vectorIN3RBX11TextService10YAlignmentESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int()
pub fn stub_0xf2482c() -> ! { todo!("0xf2482c __ZNSt6vectorIN3RBX11TextService10YAlignmentESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX11TextService10XAlignmentESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf24838 — __ZNSt6vectorIN3RBX11TextService10XAlignmentESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int()
pub fn stub_0xf24838() -> ! { todo!("0xf24838 __ZNSt6vectorIN3RBX11TextService10XAlignmentESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX11TextService4FontESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf24844 — __ZNSt6vectorIN3RBX11TextService4FontESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int()
pub fn stub_0xf24844() -> ! { todo!("0xf24844 __ZNSt6vectorIN3RBX11TextService4FontESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX11TextService8FontSizeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf24850 — __ZNSt6vectorIN3RBX11TextService8FontSizeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int()
pub fn stub_0xf24850() -> ! { todo!("0xf24850 __ZNSt6vectorIN3RBX11TextService8FontSizeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E$shim")]
// 0xf2485c — __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E$shim
// type: int __fastcall(int, int)
pub fn stub_0xf2485c() -> ! { todo!("0xf2485c __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E$shim") }

#[doc(alias = "__ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E3popEv$shim")]
// 0xf24868 — __ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E3popEv$shim
// type: int()
pub fn stub_0xf24868() -> ! { todo!("0xf24868 __ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E3popEv$shim") }

#[doc(alias = "__ZN5boost9iostreams13stream_bufferINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES3_ED2Ev$shim")]
// 0xf24874 — __ZN5boost9iostreams13stream_bufferINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES3_ED2Ev$shim
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0xf24874() -> ! { todo!("0xf24874 __ZN5boost9iostreams13stream_bufferINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES3_ED2Ev$shim") }

#[doc(alias = "__ZN5boost16exception_detail19error_info_injectorINSt8ios_base7failureEED2Ev$shim")]
// 0xf24880 — __ZN5boost16exception_detail19error_info_injectorINSt8ios_base7failureEED2Ev$shim
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0xf24880() -> ! { todo!("0xf24880 __ZN5boost16exception_detail19error_info_injectorINSt8ios_base7failureEED2Ev$shim") }

#[doc(alias = "__ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINSt8ios_base7failureEEEE5cloneEv$shim")]
// 0xf2488c — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINSt8ios_base7failureEEEE5cloneEv$shim
// type: int()
pub fn stub_0xf2488c() -> ! { todo!("0xf2488c __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINSt8ios_base7failureEEEE5cloneEv$shim") }

#[doc(alias = "__ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEED2Ev$shim")]
// 0xf24898 — __ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEED2Ev$shim
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0xf24898() -> ! { todo!("0xf24898 __ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEED2Ev$shim") }

#[doc(alias = "__ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_6outputEED2Ev$shim")]
// 0xf248a4 — __ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_6outputEED2Ev$shim
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
pub fn stub_0xf248a4() -> ! { todo!("0xf248a4 __ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_6outputEED2Ev$shim") }

#[doc(alias = "__ZN5boost9iostreams5closeINS0_21basic_gzip_compressorISaIcEEENS0_6detail16linked_streambufIcSt11char_traitsIcEEEEEvRT_RT0_St13_Ios_Openmode$shim")]
// 0xf248b0 — __ZN5boost9iostreams5closeINS0_21basic_gzip_compressorISaIcEEENS0_6detail16linked_streambufIcSt11char_traitsIcEEEEEvRT_RT0_St13_Ios_Openmode$shim
// type: int __fastcall(int, void *)
pub fn stub_0xf248b0() -> ! { todo!("0xf248b0 __ZN5boost9iostreams5closeINS0_21basic_gzip_compressorISaIcEEENS0_6detail16linked_streambufIcSt11char_traitsIcEEEEEvRT_RT0_St13_Ios_Openmode$shim") }

#[doc(alias = "__ZN5boost9iostreams16symmetric_filterINS0_6detail20zlib_compressor_implISaIcEEES4_E5writeINS2_16linked_streambufIcSt11char_traitsIcEEEEEiRT_PKci$shim")]
// 0xf248bc — __ZN5boost9iostreams16symmetric_filterINS0_6detail20zlib_compressor_implISaIcEEES4_E5writeINS2_16linked_streambufIcSt11char_traitsIcEEEEEiRT_PKci$shim
// type: int()
pub fn stub_0xf248bc() -> ! { todo!("0xf248bc __ZN5boost9iostreams16symmetric_filterINS0_6detail20zlib_compressor_implISaIcEEES4_E5writeINS2_16linked_streambufIcSt11char_traitsIcEEEEEiRT_PKci$shim") }

#[doc(alias = "__ZN5boost9iostreams16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EED1Ev$shim")]
// 0xf248c8 — __ZN5boost9iostreams16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EED1Ev$shim
// type: int()
pub fn stub_0xf248c8() -> ! { todo!("0xf248c8 __ZN5boost9iostreams16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EED1Ev$shim") }

