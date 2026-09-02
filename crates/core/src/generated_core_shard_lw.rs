//! core shard lw — 100 core stubs EA-sorted, next uncovered after shard lv (0xd98a8) / shard ls (0x82f854) — filtered core fallback.
//! Source: ida/export.json filtered where demangled/mangled excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound|Gfx|Render|Adorn, EA-sorted asc, next 100 uncovered (lowest EA first).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.
//! Preserves IDA ea + mangled + demangled for rg; uses rbx_core::SharedPtr not boost.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "str_rep(lua_State *)")]
// 0x82f85c — __ZL7str_repP9lua_State
pub fn stub_82f85c() -> ! {
    todo!("0x82f85c __ZL7str_repP9lua_State")
}

#[doc(alias = "str_reverse(lua_State *)")]
// 0x82f8e0 — __ZL11str_reverseP9lua_State
pub fn stub_82f8e0() -> ! {
    todo!("0x82f8e0 __ZL11str_reverseP9lua_State")
}

#[doc(alias = "str_sub(lua_State *)")]
// 0x82f97c — __ZL7str_subP9lua_State
pub fn stub_82f97c() -> ! {
    todo!("0x82f97c __ZL7str_subP9lua_State")
}

#[doc(alias = "str_upper(lua_State *)")]
// 0x82fa00 — __ZL9str_upperP9lua_State
pub fn stub_82fa00() -> ! {
    todo!("0x82fa00 __ZL9str_upperP9lua_State")
}

#[doc(alias = "str_find_aux(lua_State *,int)")]
// 0x82fa98 — __ZL12str_find_auxP9lua_Statei
pub fn stub_82fa98() -> ! {
    todo!("0x82fa98 __ZL12str_find_auxP9lua_Statei")
}

#[doc(alias = "gmatch_aux(lua_State *)")]
// 0x830358 — __ZL10gmatch_auxP9lua_State
pub fn stub_830358() -> ! {
    todo!("0x830358 __ZL10gmatch_auxP9lua_State")
}

#[doc(alias = "writer(lua_State *,void const*,unsigned long,void *)")]
// 0x830400 — __ZL6writerP9lua_StatePKvmPv
pub fn stub_830400() -> ! {
    todo!("0x830400 __ZL6writerP9lua_StatePKvmPv")
}

#[doc(alias = "luaH_next(lua_State *,Table *,lua_TValue *)")]
// 0x8304d8 — __Z9luaH_nextP9lua_StateP5TableP10lua_TValue
// type: int __fastcall(int, int, double *)
pub fn stub_8304d8() -> ! {
    todo!("0x8304d8 __Z9luaH_nextP9lua_StateP5TableP10lua_TValue")
}

#[doc(alias = "luaH_resizearray(lua_State *,Table *,int)")]
// 0x83061c — __Z16luaH_resizearrayP9lua_StateP5Tablei
pub fn stub_83061c() -> ! {
    todo!("0x83061c __Z16luaH_resizearrayP9lua_StateP5Tablei")
}

#[doc(alias = "resize(lua_State *,Table *,int,int)")]
// 0x830640 — __ZL6resizeP9lua_StateP5Tableii
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_830640() -> ! {
    todo!("0x830640 __ZL6resizeP9lua_StateP5Tableii")
}

#[doc(alias = "luaH_new(lua_State *,int,int)")]
// 0x830768 — __Z8luaH_newP9lua_Stateii
pub fn stub_830768() -> ! {
    todo!("0x830768 __Z8luaH_newP9lua_Stateii")
}

#[doc(alias = "setarrayvector(lua_State *,Table *,int)")]
// 0x8307c4 — __ZL14setarrayvectorP9lua_StateP5Tablei
pub fn stub_8307c4() -> ! {
    todo!("0x8307c4 __ZL14setarrayvectorP9lua_StateP5Tablei")
}

#[doc(alias = "setnodevector(lua_State *,Table *,int)")]
// 0x830828 — __ZL13setnodevectorP9lua_StateP5Tablei
pub fn stub_830828() -> ! {
    todo!("0x830828 __ZL13setnodevectorP9lua_StateP5Tablei")
}

#[doc(alias = "luaH_free(lua_State *,Table *)")]
// 0x8308cc — __Z9luaH_freeP9lua_StateP5Table
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_8308cc() -> ! {
    todo!("0x8308cc __Z9luaH_freeP9lua_StateP5Table")
}

#[doc(alias = "luaH_getnum(Table *,int)")]
// 0x830914 — __Z11luaH_getnumP5Tablei
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_830914() -> ! {
    todo!("0x830914 __Z11luaH_getnumP5Tablei")
}

#[doc(alias = "luaH_getstr(Table *,TString *)")]
// 0x8309d0 — __Z11luaH_getstrP5TableP7TString
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_8309d0() -> ! {
    todo!("0x8309d0 __Z11luaH_getstrP5TableP7TString")
}

#[doc(alias = "luaH_get(Table *,lua_TValue const*)")]
// 0x830a0c — __Z8luaH_getP5TablePK10lua_TValue
// type: void *__fastcall(int, _DWORD *)
pub fn stub_830a0c() -> ! {
    todo!("0x830a0c __Z8luaH_getP5TablePK10lua_TValue")
}

#[doc(alias = "mainposition(Table const*,lua_TValue const*)")]
// 0x830a78 — __ZL12mainpositionPK5TablePK10lua_TValue
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_830a78() -> ! {
    todo!("0x830a78 __ZL12mainpositionPK5TablePK10lua_TValue")
}

#[doc(alias = "luaH_set(lua_State *,Table *,lua_TValue const*)")]
// 0x830ae4 — __Z8luaH_setP9lua_StateP5TablePK10lua_TValue
// type: void *__fastcall(int, int, int)
pub fn stub_830ae4() -> ! {
    todo!("0x830ae4 __Z8luaH_setP9lua_StateP5TablePK10lua_TValue")
}

#[doc(alias = "newkey(lua_State *,Table *,lua_TValue const*)")]
// 0x830b4c — __ZL6newkeyP9lua_StateP5TablePK10lua_TValue
// type: void *__fastcall(int, int, _DWORD *)
pub fn stub_830b4c() -> ! {
    todo!("0x830b4c __ZL6newkeyP9lua_StateP5TablePK10lua_TValue")
}

#[doc(alias = "luaH_setnum(lua_State *,Table *,int)")]
// 0x830d84 — __Z11luaH_setnumP9lua_StateP5Tablei
pub fn stub_830d84() -> ! {
    todo!("0x830d84 __Z11luaH_setnumP9lua_StateP5Tablei")
}

#[doc(alias = "luaH_setstr(lua_State *,Table *,TString *)")]
// 0x830dc8 — __Z11luaH_setstrP9lua_StateP5TableP7TString
pub fn stub_830dc8() -> ! {
    todo!("0x830dc8 __Z11luaH_setstrP9lua_StateP5TableP7TString")
}

#[doc(alias = "luaH_getn(Table *)")]
// 0x830e00 — __Z9luaH_getnP5Table
pub fn stub_830e00() -> ! {
    todo!("0x830e00 __Z9luaH_getnP5Table")
}

#[doc(alias = "countint(lua_TValue const*,int *)")]
// 0x830ecc — __ZL8countintPK10lua_TValuePi
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_830ecc() -> ! {
    todo!("0x830ecc __ZL8countintPK10lua_TValuePi")
}

#[doc(alias = "luaopen_table(lua_State *)")]
// 0x830fe0 — __Z13luaopen_tableP9lua_State
pub fn stub_830fe0() -> ! {
    todo!("0x830fe0 __Z13luaopen_tableP9lua_State")
}

#[doc(alias = "tconcat(lua_State *)")]
// 0x831000 — __ZL7tconcatP9lua_State
pub fn stub_831000() -> ! {
    todo!("0x831000 __ZL7tconcatP9lua_State")
}

#[doc(alias = "foreach(lua_State *)")]
// 0x8310d0 — __ZL7foreachP9lua_State
pub fn stub_8310d0() -> ! {
    todo!("0x8310d0 __ZL7foreachP9lua_State")
}

#[doc(alias = "foreachi(lua_State *)")]
// 0x831140 — __ZL8foreachiP9lua_State
pub fn stub_831140() -> ! {
    todo!("0x831140 __ZL8foreachiP9lua_State")
}

#[doc(alias = "getn(lua_State *)")]
// 0x8311bc — __ZL4getnP9lua_State
pub fn stub_8311bc() -> ! {
    todo!("0x8311bc __ZL4getnP9lua_State")
}

#[doc(alias = "maxn(lua_State *)")]
// 0x8311e0 — __ZL4maxnP9lua_State
pub fn stub_8311e0() -> ! {
    todo!("0x8311e0 __ZL4maxnP9lua_State")
}

#[doc(alias = "tinsert(lua_State *)")]
// 0x831280 — __ZL7tinsertP9lua_State
pub fn stub_831280() -> ! {
    todo!("0x831280 __ZL7tinsertP9lua_State")
}

#[doc(alias = "tremove(lua_State *)")]
// 0x831304 — __ZL7tremoveP9lua_State
pub fn stub_831304() -> ! {
    todo!("0x831304 __ZL7tremoveP9lua_State")
}

#[doc(alias = "setn(lua_State *)")]
// 0x831380 — __ZL4setnP9lua_State
pub fn stub_831380() -> ! {
    todo!("0x831380 __ZL4setnP9lua_State")
}

#[doc(alias = "sort(lua_State *)")]
// 0x8313ac — __ZL4sortP9lua_State
pub fn stub_8313ac() -> ! {
    todo!("0x8313ac __ZL4sortP9lua_State")
}

#[doc(alias = "auxsort(lua_State *,int,int)")]
// 0x831404 — __ZL7auxsortP9lua_Stateii
pub fn stub_831404() -> ! {
    todo!("0x831404 __ZL7auxsortP9lua_Stateii")
}

#[doc(alias = "sort_comp(lua_State *,int,int)")]
// 0x831690 — __ZL9sort_compP9lua_Stateii
pub fn stub_831690() -> ! {
    todo!("0x831690 __ZL9sort_compP9lua_Stateii")
}

#[doc(alias = "addfield(lua_State *,luaL_Buffer *,int)")]
// 0x8316ec — __ZL8addfieldP9lua_StateP11luaL_Bufferi
pub fn stub_8316ec() -> ! {
    todo!("0x8316ec __ZL8addfieldP9lua_StateP11luaL_Bufferi")
}

#[doc(alias = "luaT_init(lua_State *)")]
// 0x831800 — __Z9luaT_initP9lua_State
pub fn stub_831800() -> ! {
    todo!("0x831800 __Z9luaT_initP9lua_State")
}

#[doc(alias = "luaT_gettm(Table *,TMS,TString *)")]
// 0x831854 — __Z10luaT_gettmP5Table3TMSP7TString
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_831854() -> ! {
    todo!("0x831854 __Z10luaT_gettmP5Table3TMSP7TString")
}

#[doc(alias = "luaT_gettmbyobj(lua_State *,lua_TValue const*,TMS)")]
// 0x831878 — __Z15luaT_gettmbyobjP9lua_StatePK10lua_TValue3TMS
// type: void *__fastcall(int, _DWORD *, int)
pub fn stub_831878() -> ! {
    todo!("0x831878 __Z15luaT_gettmbyobjP9lua_StatePK10lua_TValue3TMS")
}

#[doc(alias = "luaU_undump(lua_State *,Zio *,Mbuffer *,char const*)")]
// 0x831984 — __Z11luaU_undumpP9lua_StateP3ZioP7MbufferPKc
pub fn stub_831984() -> ! {
    todo!("0x831984 __Z11luaU_undumpP9lua_StateP3ZioP7MbufferPKc")
}

#[doc(alias = "luaU_header(char *)")]
// 0x831e50 — __Z11luaU_headerPc
// type: _DWORD __fastcall(char *)
pub fn stub_831e50() -> ! {
    todo!("0x831e50 __Z11luaU_headerPc")
}

#[doc(alias = "luaV_tonumber(lua_TValue const*,lua_TValue*)")]
// 0x831ff4 — __Z13luaV_tonumberPK10lua_TValuePS_
// type: _DWORD *__fastcall(_DWORD *, int)
pub fn stub_831ff4() -> ! {
    todo!("0x831ff4 __Z13luaV_tonumberPK10lua_TValuePS_")
}

#[doc(alias = "luaV_tostring(lua_State *,lua_TValue *)")]
// 0x832038 — __Z13luaV_tostringP9lua_StateP10lua_TValue
// type: int __fastcall(int, int)
pub fn stub_832038() -> ! {
    todo!("0x832038 __Z13luaV_tostringP9lua_StateP10lua_TValue")
}

#[doc(alias = "luaV_gettable(lua_State *,lua_TValue const*,lua_TValue*,lua_TValue*)")]
// 0x8320a4 — __Z13luaV_gettableP9lua_StatePK10lua_TValuePS1_S4_
// type: int __fastcall(int, _DWORD *, int, _DWORD *)
pub fn stub_8320a4() -> ! {
    todo!("0x8320a4 __Z13luaV_gettableP9lua_StatePK10lua_TValuePS1_S4_")
}

#[doc(alias = "callTMres(lua_State *,lua_TValue *,lua_TValue const*,lua_TValue const*,lua_TValue const*)")]
// 0x832164 — __ZL9callTMresP9lua_StateP10lua_TValuePKS1_S4_S4_
// type: int __fastcall(_DWORD *, int, int *, int *, int *)
pub fn stub_832164() -> ! {
    todo!("0x832164 __ZL9callTMresP9lua_StateP10lua_TValuePKS1_S4_S4_")
}

#[doc(alias = "luaV_settable(lua_State *,lua_TValue const*,lua_TValue*,lua_TValue*)")]
// 0x8321e8 — __Z13luaV_settableP9lua_StatePK10lua_TValuePS1_S4_
// type: int __fastcall(_DWORD *, int *, int *, _DWORD *)
pub fn stub_8321e8() -> ! {
    todo!("0x8321e8 __Z13luaV_settableP9lua_StatePK10lua_TValuePS1_S4_")
}

#[doc(alias = "luaV_lessthan(lua_State *,lua_TValue const*,lua_TValue const*)")]
// 0x83234c — __Z13luaV_lessthanP9lua_StatePK10lua_TValueS3_
// type: unsigned int __fastcall(int, _DWORD *, _DWORD *)
pub fn stub_83234c() -> ! {
    todo!("0x83234c __Z13luaV_lessthanP9lua_StatePK10lua_TValueS3_")
}

#[doc(alias = "call_orderTM(lua_State *,lua_TValue const*,lua_TValue const*,TMS)")]
// 0x83240c — __ZL12call_orderTMP9lua_StatePK10lua_TValueS3_3TMS
// type: int __fastcall(_DWORD *, int *, int *, int)
pub fn stub_83240c() -> ! {
    todo!("0x83240c __ZL12call_orderTMP9lua_StatePK10lua_TValueS3_3TMS")
}

#[doc(alias = "luaV_equalval(lua_State *,lua_TValue const*,lua_TValue const*)")]
// 0x832478 — __Z13luaV_equalvalP9lua_StatePK10lua_TValueS3_
// type: int __fastcall(int, double *, int)
pub fn stub_832478() -> ! {
    todo!("0x832478 __Z13luaV_equalvalP9lua_StatePK10lua_TValueS3_")
}

#[doc(alias = "get_compTM(lua_State *,Table *,Table *,TMS)")]
// 0x832514 — __ZL10get_compTMP9lua_StateP5TableS2_3TMS
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_832514() -> ! {
    todo!("0x832514 __ZL10get_compTMP9lua_StateP5TableS2_3TMS")
}

#[doc(alias = "luaV_concat(lua_State *,int,int)")]
// 0x832584 — __Z11luaV_concatP9lua_Stateii
pub fn stub_832584() -> ! {
    todo!("0x832584 __Z11luaV_concatP9lua_Stateii")
}

#[doc(alias = "call_binTM(lua_State *,lua_TValue const*,lua_TValue const*,lua_TValue*,TMS)")]
// 0x8326f8 — __ZL10call_binTMP9lua_StatePK10lua_TValueS3_PS1_3TMS
// type: int __fastcall(_DWORD *, int *, int *, int, int)
pub fn stub_8326f8() -> ! {
    todo!("0x8326f8 __ZL10call_binTMP9lua_StatePK10lua_TValueS3_PS1_3TMS")
}

#[doc(alias = "luaV_execute(lua_State *,int)")]
// 0x832744 — __Z12luaV_executeP9lua_Statei
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_832744() -> ! {
    todo!("0x832744 __Z12luaV_executeP9lua_Statei")
}

#[doc(alias = "Arith(lua_State *,lua_TValue *,lua_TValue const*,lua_TValue const*,TMS)")]
// 0x833730 — __ZL5ArithP9lua_StateP10lua_TValuePKS1_S4_3TMS
// type: int __fastcall(_DWORD *, int, int *, int *, int)
pub fn stub_833730() -> ! {
    todo!("0x833730 __ZL5ArithP9lua_StateP10lua_TValuePKS1_S4_3TMS")
}

#[doc(alias = "luaZ_fill(Zio *)")]
// 0x8338c8 — __Z9luaZ_fillP3Zio
pub fn stub_8338c8() -> ! {
    todo!("0x8338c8 __Z9luaZ_fillP3Zio")
}

#[doc(alias = "luaZ_lookahead(Zio *)")]
// 0x8338fc — __Z14luaZ_lookaheadP3Zio
pub fn stub_8338fc() -> ! {
    todo!("0x8338fc __Z14luaZ_lookaheadP3Zio")
}

#[doc(alias = "luaZ_init(lua_State *,Zio *,char const* (*)(lua_State *,void *,unsigned long *),void *)")]
// 0x833930 — __Z9luaZ_initP9lua_StateP3ZioPFPKcS0_PvPmES5_
pub fn stub_833930() -> ! {
    todo!("0x833930 __Z9luaZ_initP9lua_StateP3ZioPFPKcS0_PvPmES5_")
}

#[doc(alias = "luaZ_read(Zio *,void *,unsigned long)")]
// 0x833940 — __Z9luaZ_readP3ZioPvm
pub fn stub_833940() -> ! {
    todo!("0x833940 __Z9luaZ_readP3ZioPvm")
}

#[doc(alias = "luaZ_openspace(lua_State *,Mbuffer *,unsigned long)")]
// 0x8339a0 — __Z14luaZ_openspaceP9lua_StateP7Mbufferm
pub fn stub_8339a0() -> ! {
    todo!("0x8339a0 __Z14luaZ_openspaceP9lua_StateP7Mbufferm")
}

#[doc(alias = "-[EAGLView description]")]
// 0xe5eee8 — -[EAGLView description]
// type: id __cdecl(EAGLView *self, SEL)
pub fn stub_e5eee8() -> ! {
    todo!("0xe5eee8 -[EAGLView description]")
}

#[doc(alias = "-[EAGL2View description]")]
// 0xe87e38 — -[EAGL2View description]
// type: id __cdecl(EAGL2View *self, SEL)
pub fn stub_e87e38() -> ! {
    todo!("0xe87e38 -[EAGL2View description]")
}

#[doc(alias = "-[GAITrackerImpl trackException:withDescription:]")]
// 0xeb823c — -[GAITrackerImpl trackException:withDescription:]
// type: char __cdecl(GAITrackerImpl *self, SEL, char, id)
pub fn stub_eb823c() -> ! {
    todo!("0xeb823c -[GAITrackerImpl trackException:withDescription:]")
}

#[doc(alias = "-[GAITrackerImpl sendException:withDescription:]")]
// 0xeb82b4 — -[GAITrackerImpl sendException:withDescription:]
// type: char __cdecl(GAITrackerImpl *self, SEL, char, id)
pub fn stub_eb82b4() -> ! {
    todo!("0xeb82b4 -[GAITrackerImpl sendException:withDescription:]")
}

#[doc(alias = "+[GAIHit entityDescription]")]
// 0xebc408 — +[GAIHit entityDescription]
// type: id __cdecl(id, SEL)
pub fn stub_ebc408() -> ! {
    todo!("0xebc408 +[GAIHit entityDescription]")
}

#[doc(alias = "+[GAIError errorWithCode:withDescription:]")]
// 0xebc8e8 — +[GAIError errorWithCode:withDescription:]
// type: id __cdecl(id, SEL, int, id)
pub fn stub_ebc8e8() -> ! {
    todo!("0xebc8e8 +[GAIError errorWithCode:withDescription:]")
}

#[doc(alias = "+[GAIError errorWithCode:withFailedFilePath:withDescription:]")]
// 0xebc9cc — +[GAIError errorWithCode:withFailedFilePath:withDescription:]
// type: id __cdecl(id, SEL, int, id, id)
pub fn stub_ebc9cc() -> ! {
    todo!("0xebc9cc +[GAIError errorWithCode:withFailedFilePath:withDescription:]")
}

#[doc(alias = "+[GAIError errorWithCode:withUnderlyingError:withDescription:]")]
// 0xebcad0 — +[GAIError errorWithCode:withUnderlyingError:withDescription:]
// type: id __cdecl(id, SEL, int, id, id)
pub fn stub_ebcad0() -> ! {
    todo!("0xebcad0 +[GAIError errorWithCode:withUnderlyingError:withDescription:]")
}

#[doc(alias = "+[GAIProperty entityDescription]")]
// 0xebcbd4 — +[GAIProperty entityDescription]
// type: id __cdecl(id, SEL)
pub fn stub_ebcbd4() -> ! {
    todo!("0xebcbd4 +[GAIProperty entityDescription]")
}

#[doc(alias = "+[GAIExceptionParser exceptionDescription:withMaxLength:]")]
// 0xec2a0c — +[GAIExceptionParser exceptionDescription:withMaxLength:]
// type: id __cdecl(id, SEL, id, unsigned int)
pub fn stub_ec2a0c() -> ! {
    todo!("0xec2a0c +[GAIExceptionParser exceptionDescription:withMaxLength:]")
}

#[doc(alias = "-[TFLogManager writeLogFor:toFileDescriptor:inAHurry:bytesWritten:]")]
// 0xec9780 — -[TFLogManager writeLogFor:toFileDescriptor:inAHurry:bytesWritten:]
// type: char __cdecl(TFLogManager *self, SEL, id, int, char, int *)
pub fn stub_ec9780() -> ! {
    todo!("0xec9780 -[TFLogManager writeLogFor:toFileDescriptor:inAHurry:bytesWritten:]")
}

#[doc(alias = "___67-[TFLogManager writeLogFor:toFileDescriptor:inAHurry:bytesWritten:]_block_invoke")]
// 0xec987c — ___67-[TFLogManager writeLogFor:toFileDescriptor:inAHurry:bytesWritten:]_block_invoke
pub fn stub_ec987c() -> ! {
    todo!("0xec987c ___67-[TFLogManager writeLogFor:toFileDescriptor:inAHurry:bytesWritten:]_block_invoke")
}

#[doc(alias = "_tf_event_pack_network_start_data_safe")]
// 0xed53e8 — _tf_event_pack_network_start_data_safe
pub fn stub_ed53e8() -> ! {
    todo!("0xed53e8 _tf_event_pack_network_start_data_safe")
}

#[doc(alias = "+[TFURLConnectionOperation _networkThread]")]
// 0xed6e48 — +[TFURLConnectionOperation _networkThread]
// type: id __cdecl(id, SEL)
pub fn stub_ed6e48() -> ! {
    todo!("0xed6e48 +[TFURLConnectionOperation _networkThread]")
}

#[doc(alias = "+[BSAFHTTPRequestOperation networkRequestThread]")]
// 0xee1060 — +[BSAFHTTPRequestOperation networkRequestThread]
// type: id __cdecl(id, SEL)
pub fn stub_ee1060() -> ! {
    todo!("0xee1060 +[BSAFHTTPRequestOperation networkRequestThread]")
}

#[doc(alias = "___48+[BSAFHTTPRequestOperation networkRequestThread]_block_invoke")]
// 0xee10c4 — ___48+[BSAFHTTPRequestOperation networkRequestThread]_block_invoke
pub fn stub_ee10c4() -> ! {
    todo!("0xee10c4 ___48+[BSAFHTTPRequestOperation networkRequestThread]_block_invoke")
}

#[doc(alias = "+[BSAFHTTPRequestOperation networkRequestThreadEntryPoint:]")]
// 0xee114c — +[BSAFHTTPRequestOperation networkRequestThreadEntryPoint:]
// type: void __cdecl(id, SEL, id)
pub fn stub_ee114c() -> ! {
    todo!("0xee114c +[BSAFHTTPRequestOperation networkRequestThreadEntryPoint:]")
}

#[doc(alias = "-[BSAFHTTPRequestOperation setUploadProgressBlock:]")]
// 0xee1570 — -[BSAFHTTPRequestOperation setUploadProgressBlock:]
// type: void __cdecl(BSAFHTTPRequestOperation *self, SEL, id)
pub fn stub_ee1570() -> ! {
    todo!("0xee1570 -[BSAFHTTPRequestOperation setUploadProgressBlock:]")
}

#[doc(alias = "-[BSAFHTTPRequestOperation setDownloadProgressBlock:]")]
// 0xee1588 — -[BSAFHTTPRequestOperation setDownloadProgressBlock:]
// type: void __cdecl(BSAFHTTPRequestOperation *self, SEL, id)
pub fn stub_ee1588() -> ! {
    todo!("0xee1588 -[BSAFHTTPRequestOperation setDownloadProgressBlock:]")
}

#[doc(alias = "-[BSAFHTTPRequestOperation uploadProgress]")]
// 0xee2290 — -[BSAFHTTPRequestOperation uploadProgress]
// type: id __cdecl(BSAFHTTPRequestOperation *self, SEL)
pub fn stub_ee2290() -> ! {
    todo!("0xee2290 -[BSAFHTTPRequestOperation uploadProgress]")
}

#[doc(alias = "-[BSAFHTTPRequestOperation setUploadProgress:]")]
// 0xee22a8 — -[BSAFHTTPRequestOperation setUploadProgress:]
// type: void __cdecl(BSAFHTTPRequestOperation *self, SEL, id)
pub fn stub_ee22a8() -> ! {
    todo!("0xee22a8 -[BSAFHTTPRequestOperation setUploadProgress:]")
}

#[doc(alias = "-[BSAFHTTPRequestOperation downloadProgress]")]
// 0xee22d8 — -[BSAFHTTPRequestOperation downloadProgress]
// type: id __cdecl(BSAFHTTPRequestOperation *self, SEL)
pub fn stub_ee22d8() -> ! {
    todo!("0xee22d8 -[BSAFHTTPRequestOperation downloadProgress]")
}

#[doc(alias = "-[BSAFHTTPRequestOperation setDownloadProgress:]")]
// 0xee22f0 — -[BSAFHTTPRequestOperation setDownloadProgress:]
// type: void __cdecl(BSAFHTTPRequestOperation *self, SEL, id)
pub fn stub_ee22f0() -> ! {
    todo!("0xee22f0 -[BSAFHTTPRequestOperation setDownloadProgress:]")
}

#[doc(alias = "-[BSReachability networkStatusForFlags:]")]
// 0xeee6f0 — -[BSReachability networkStatusForFlags:]
// type: int __cdecl(BSReachability *self, SEL, unsigned int)
pub fn stub_eee6f0() -> ! {
    todo!("0xeee6f0 -[BSReachability networkStatusForFlags:]")
}

#[doc(alias = "_protobuf_c_enum_descriptor_get_value_by_name")]
// 0xef7a24 — _protobuf_c_enum_descriptor_get_value_by_name
// type: int __fastcall(int, char *__s2)
pub fn stub_ef7a24() -> ! {
    todo!("0xef7a24 _protobuf_c_enum_descriptor_get_value_by_name")
}

#[doc(alias = "_protobuf_c_enum_descriptor_get_value")]
// 0xef7aa4 — _protobuf_c_enum_descriptor_get_value
pub fn stub_ef7aa4() -> ! {
    todo!("0xef7aa4 _protobuf_c_enum_descriptor_get_value")
}

#[doc(alias = "_protobuf_c_message_descriptor_get_field_by_name")]
// 0xef7b34 — _protobuf_c_message_descriptor_get_field_by_name
// type: const char **__fastcall(_DWORD *, char *__s2)
pub fn stub_ef7b34() -> ! {
    todo!("0xef7b34 _protobuf_c_message_descriptor_get_field_by_name")
}

#[doc(alias = "_protobuf_c_message_descriptor_get_field")]
// 0xef7bb4 — _protobuf_c_message_descriptor_get_field
pub fn stub_ef7bb4() -> ! {
    todo!("0xef7bb4 _protobuf_c_message_descriptor_get_field")
}

#[doc(alias = "_protobuf_c_service_descriptor_get_method_by_name")]
// 0xef7c44 — _protobuf_c_service_descriptor_get_method_by_name
// type: int __fastcall(int, char *__s2)
pub fn stub_ef7c44() -> ! {
    todo!("0xef7c44 _protobuf_c_service_descriptor_get_method_by_name")
}

#[doc(alias = "_plcrash_populate_posix_error")]
// 0xefa59c — _plcrash_populate_posix_error
pub fn stub_efa59c() -> ! {
    todo!("0xefa59c _plcrash_populate_posix_error")
}

#[doc(alias = "+[Flurry setAppVersion:]")]
// 0xefa5e8 — +[Flurry setAppVersion:]
// type: void __cdecl(id, SEL, id)
pub fn stub_efa5e8() -> ! {
    todo!("0xefa5e8 +[Flurry setAppVersion:]")
}

#[doc(alias = "+[Flurry setLaunchOptions:]")]
// 0xefa7c8 — +[Flurry setLaunchOptions:]
// type: void __cdecl(id, SEL, id)
pub fn stub_efa7c8() -> ! {
    todo!("0xefa7c8 +[Flurry setLaunchOptions:]")
}

#[doc(alias = "+[Flurry parsePropertyList:]")]
// 0xefab04 — +[Flurry parsePropertyList:]
// type: void __cdecl(id, SEL, id)
pub fn stub_efab04() -> ! {
    todo!("0xefab04 +[Flurry parsePropertyList:]")
}

#[doc(alias = "+[Flurry parseDictionary:]")]
// 0xefac18 — +[Flurry parseDictionary:]
// type: void __cdecl(id, SEL, id)
pub fn stub_efac18() -> ! {
    todo!("0xefac18 +[Flurry parseDictionary:]")
}

#[doc(alias = "+[Flurry getFlurryAgentVersion]")]
// 0xefb04c — +[Flurry getFlurryAgentVersion]
// type: id __cdecl(id, SEL)
pub fn stub_efb04c() -> ! {
    todo!("0xefb04c +[Flurry getFlurryAgentVersion]")
}

#[doc(alias = "+[Flurry setLogLevel:]")]
// 0xefb070 — +[Flurry setLogLevel:]
// type: void __cdecl(id, SEL, int)
pub fn stub_efb070() -> ! {
    todo!("0xefb070 +[Flurry setLogLevel:]")
}

#[doc(alias = "+[Flurry setDebugLogEnabled:]")]
// 0xefb094 — +[Flurry setDebugLogEnabled:]
// type: void __cdecl(id, SEL, char)
pub fn stub_efb094() -> ! {
    todo!("0xefb094 +[Flurry setDebugLogEnabled:]")
}

#[doc(alias = "+[Flurry sessionContinueSeconds]")]
// 0xefb0c0 — +[Flurry sessionContinueSeconds]
// type: int __cdecl(id, SEL)
pub fn stub_efb0c0() -> ! {
    todo!("0xefb0c0 +[Flurry sessionContinueSeconds]")
}

#[doc(alias = "+[Flurry setSessionContinueSeconds:]")]
// 0xefb0f4 — +[Flurry setSessionContinueSeconds:]
// type: void __cdecl(id, SEL, int)
pub fn stub_efb0f4() -> ! {
    todo!("0xefb0f4 +[Flurry setSessionContinueSeconds:]")
}

#[doc(alias = "+[Flurry setMaxSessionsSaved:]")]
// 0xefb180 — +[Flurry setMaxSessionsSaved:]
// type: void __cdecl(id, SEL, int)
pub fn stub_efb180() -> ! {
    todo!("0xefb180 +[Flurry setMaxSessionsSaved:]")
}
