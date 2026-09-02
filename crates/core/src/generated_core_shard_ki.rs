//! core shard ki — 100 stubs EA-sorted asc global gap filler not yet in core (fallback filter).
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 100 not yet in rbx_core after kh 0xc45460 (fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, boost; 26188 filtered, 5591->5491 gaps, 35679->35779 distinct, rbx_core::SharedPtr not boost).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + #[doc(alias = mangled)] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;


#[doc(alias = "__zip_dirent_read")]
// 0xc45490 — __zip_dirent_read
// type: int __fastcall(int, FILE *__stream, int, int, int, int)
pub fn stub_0xc45490() -> ! {
    todo!("0xc45490 __zip_dirent_read")
}

#[doc(alias = "__zip_read2")]
// 0xc457f8 — __zip_read2
pub fn stub_0xc457f8() -> ! {
    todo!("0xc457f8 __zip_read2")
}

#[doc(alias = "__zip_read4")]
// 0xc45808 — __zip_read4
pub fn stub_0xc45808() -> ! {
    todo!("0xc45808 __zip_read4")
}

#[doc(alias = "__zip_readfpstr")]
// 0xc45828 — __zip_readfpstr
pub fn stub_0xc45828() -> ! {
    todo!("0xc45828 __zip_readfpstr")
}

#[doc(alias = "__zip_dirent_torrent_normalize")]
// 0xc458a8 — __zip_dirent_torrent_normalize
pub fn stub_0xc458a8() -> ! {
    todo!("0xc458a8 __zip_dirent_torrent_normalize")
}

#[doc(alias = "__zip_entry_free")]
// 0xc45924 — __zip_entry_free
pub fn stub_0xc45924() -> ! {
    todo!("0xc45924 __zip_entry_free")
}

#[doc(alias = "__zip_entry_new")]
// 0xc45958 — __zip_entry_new
// type: char *__fastcall(int)
pub fn stub_0xc45958() -> ! {
    todo!("0xc45958 __zip_entry_new")
}

#[doc(alias = "__zip_error_copy")]
// 0xc459d0 — __zip_error_copy
pub fn stub_0xc459d0() -> ! {
    todo!("0xc459d0 __zip_error_copy")
}

#[doc(alias = "__zip_error_get")]
// 0xc459dc — __zip_error_get
pub fn stub_0xc459dc() -> ! {
    todo!("0xc459dc __zip_error_get")
}

#[doc(alias = "__zip_error_init")]
// 0xc45a04 — __zip_error_init
pub fn stub_0xc45a04() -> ! {
    todo!("0xc45a04 __zip_error_init")
}

#[doc(alias = "__zip_error_set")]
// 0xc45a14 — __zip_error_set
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xc45a14() -> ! {
    todo!("0xc45a14 __zip_error_set")
}

#[doc(alias = "_zip_error_get_sys_type")]
// 0xc45a24 — _zip_error_get_sys_type
pub fn stub_0xc45a24() -> ! {
    todo!("0xc45a24 _zip_error_get_sys_type")
}

#[doc(alias = "_zip_fclose")]
// 0xc45a54 — _zip_fclose
pub fn stub_0xc45a54() -> ! {
    todo!("0xc45a54 _zip_fclose")
}

#[doc(alias = "_zip_file_error_get")]
// 0xc45ac4 — _zip_file_error_get
pub fn stub_0xc45ac4() -> ! {
    todo!("0xc45ac4 _zip_file_error_get")
}

#[doc(alias = "__zip_file_get_offset")]
// 0xc45ad0 — __zip_file_get_offset
pub fn stub_0xc45ad0() -> ! {
    todo!("0xc45ad0 __zip_file_get_offset")
}

#[doc(alias = "__zip_filerange_crc")]
// 0xc45b40 — __zip_filerange_crc
pub fn stub_0xc45b40() -> ! {
    todo!("0xc45b40 __zip_filerange_crc")
}

#[doc(alias = "_zip_fopen")]
// 0xc45c64 — _zip_fopen
pub fn stub_0xc45c64() -> ! {
    todo!("0xc45c64 _zip_fopen")
}

#[doc(alias = "_zip_fopen_index")]
// 0xc45c84 — _zip_fopen_index
pub fn stub_0xc45c84() -> ! {
    todo!("0xc45c84 _zip_fopen_index")
}

#[doc(alias = "__zip_file_fillbuf")]
// 0xc45e5c — __zip_file_fillbuf
pub fn stub_0xc45e5c() -> ! {
    todo!("0xc45e5c __zip_file_fillbuf")
}

#[doc(alias = "_zip_fread")]
// 0xc45f10 — _zip_fread
pub fn stub_0xc45f10() -> ! {
    todo!("0xc45f10 _zip_fread")
}

#[doc(alias = "__zip_free")]
// 0xc46054 — __zip_free
pub fn stub_0xc46054() -> ! {
    todo!("0xc46054 __zip_free")
}

#[doc(alias = "_zip_get_archive_flag")]
// 0xc460dc — _zip_get_archive_flag
pub fn stub_0xc460dc() -> ! {
    todo!("0xc460dc _zip_get_archive_flag")
}

#[doc(alias = "_zip_get_name")]
// 0xc460f4 — _zip_get_name
pub fn stub_0xc460f4() -> ! {
    todo!("0xc460f4 _zip_get_name")
}

#[doc(alias = "__zip_get_name")]
// 0xc46104 — __zip_get_name
pub fn stub_0xc46104() -> ! {
    todo!("0xc46104 __zip_get_name")
}

#[doc(alias = "_zip_get_num_files")]
// 0xc46164 — _zip_get_num_files
pub fn stub_0xc46164() -> ! {
    todo!("0xc46164 _zip_get_num_files")
}

#[doc(alias = "__zip_memdup")]
// 0xc46174 — __zip_memdup
// type: int __fastcall(int, size_t __size)
pub fn stub_0xc46174() -> ! {
    todo!("0xc46174 __zip_memdup")
}

#[doc(alias = "_zip_name_locate")]
// 0xc461b0 — _zip_name_locate
// type: int __fastcall(int, int, int)
pub fn stub_0xc461b0() -> ! {
    todo!("0xc461b0 _zip_name_locate")
}

#[doc(alias = "__zip_name_locate")]
// 0xc461c0 — __zip_name_locate
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0xc461c0() -> ! {
    todo!("0xc461c0 __zip_name_locate")
}

#[doc(alias = "__zip_new")]
// 0xc46280 — __zip_new
pub fn stub_0xc46280() -> ! {
    todo!("0xc46280 __zip_new")
}

#[doc(alias = "_zip_open")]
// 0xc462d8 — _zip_open
// type: int __fastcall(char *)
pub fn stub_0xc462d8() -> ! {
    todo!("0xc462d8 _zip_open")
}

#[doc(alias = "__zip_allocate_new")]
// 0xc46814 — __zip_allocate_new
pub fn stub_0xc46814() -> ! {
    todo!("0xc46814 __zip_allocate_new")
}

#[doc(alias = "__zip_checkcons")]
// 0xc46874 — __zip_checkcons
// type: int __fastcall(FILE *__stream)
pub fn stub_0xc46874() -> ! {
    todo!("0xc46874 __zip_checkcons")
}

#[doc(alias = "_zip_source_function")]
// 0xc469e4 — _zip_source_function
pub fn stub_0xc469e4() -> ! {
    todo!("0xc469e4 _zip_source_function")
}

#[doc(alias = "_zip_source_zip")]
// 0xc46a20 — _zip_source_zip
pub fn stub_0xc46a20() -> ! {
    todo!("0xc46a20 _zip_source_zip")
}

#[doc(alias = "_read_zip")]
// 0xc46b78 — _read_zip
pub fn stub_0xc46b78() -> ! {
    todo!("0xc46b78 _read_zip")
}

#[doc(alias = "_zip_stat")]
// 0xc46d00 — _zip_stat
pub fn stub_0xc46d00() -> ! {
    todo!("0xc46d00 _zip_stat")
}

#[doc(alias = "_zip_stat_index")]
// 0xc46d28 — _zip_stat_index
pub fn stub_0xc46d28() -> ! {
    todo!("0xc46d28 _zip_stat_index")
}

#[doc(alias = "_zip_stat_init")]
// 0xc46df8 — _zip_stat_init
pub fn stub_0xc46df8() -> ! {
    todo!("0xc46df8 _zip_stat_init")
}

#[doc(alias = "__zip_unchange_data")]
// 0xc46e14 — __zip_unchange_data
pub fn stub_0xc46e14() -> ! {
    todo!("0xc46e14 __zip_unchange_data")
}

#[doc(alias = "global constructor keyed to_a_636")]
#[doc(alias = "__GLOBAL__I_a_636")]
// 0xc46e4c — __GLOBAL__I_a_636
pub fn stub_0xc46e4c() -> ! {
    todo!("0xc46e4c __GLOBAL__I_a_636")
}

#[doc(alias = "global constructor keyed to_a_637")]
#[doc(alias = "__GLOBAL__I_a_637")]
// 0xc46e80 — __GLOBAL__I_a_637
pub fn stub_0xc46e80() -> ! {
    todo!("0xc46e80 __GLOBAL__I_a_637")
}

#[doc(alias = "global constructor keyed to_a_638")]
#[doc(alias = "__GLOBAL__I_a_638")]
// 0xc46eb4 — __GLOBAL__I_a_638
pub fn stub_0xc46eb4() -> ! {
    todo!("0xc46eb4 __GLOBAL__I_a_638")
}

#[doc(alias = "global constructor keyed to_a_639")]
#[doc(alias = "__GLOBAL__I_a_639")]
// 0xc46ee8 — __GLOBAL__I_a_639
pub fn stub_0xc46ee8() -> ! {
    todo!("0xc46ee8 __GLOBAL__I_a_639")
}

#[doc(alias = "global constructor keyed to_a_640")]
#[doc(alias = "__GLOBAL__I_a_640")]
// 0xc46f1c — __GLOBAL__I_a_640
pub fn stub_0xc46f1c() -> ! {
    todo!("0xc46f1c __GLOBAL__I_a_640")
}

#[doc(alias = "global constructor keyed to_a_641")]
#[doc(alias = "__GLOBAL__I_a_641")]
// 0xc477b0 — __GLOBAL__I_a_641
pub fn stub_0xc477b0() -> ! {
    todo!("0xc477b0 __GLOBAL__I_a_641")
}

#[doc(alias = "global constructor keyed to_a_642")]
#[doc(alias = "__GLOBAL__I_a_642")]
// 0xc482a8 — __GLOBAL__I_a_642
pub fn stub_0xc482a8() -> ! {
    todo!("0xc482a8 __GLOBAL__I_a_642")
}

#[doc(alias = "sub_C4AD86")]
// 0xc4ad86 — sub_C4AD86
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, char)
pub fn stub_0xc4ad86() -> ! {
    todo!("0xc4ad86 sub_C4AD86")
}

#[doc(alias = "global constructor keyed to_a_643")]
#[doc(alias = "__GLOBAL__I_a_643")]
// 0xc4c118 — __GLOBAL__I_a_643
pub fn stub_0xc4c118() -> ! {
    todo!("0xc4c118 __GLOBAL__I_a_643")
}

#[doc(alias = "global constructor keyed to_a_644")]
#[doc(alias = "__GLOBAL__I_a_644")]
// 0xc4cd50 — __GLOBAL__I_a_644
pub fn stub_0xc4cd50() -> ! {
    todo!("0xc4cd50 __GLOBAL__I_a_644")
}

#[doc(alias = "global constructor keyed to_a_645")]
#[doc(alias = "__GLOBAL__I_a_645")]
// 0xc4cdbc — __GLOBAL__I_a_645
pub fn stub_0xc4cdbc() -> ! {
    todo!("0xc4cdbc __GLOBAL__I_a_645")
}

#[doc(alias = "global constructor keyed to_a_646")]
#[doc(alias = "__GLOBAL__I_a_646")]
// 0xc4f3a8 — __GLOBAL__I_a_646
pub fn stub_0xc4f3a8() -> ! {
    todo!("0xc4f3a8 __GLOBAL__I_a_646")
}

#[doc(alias = "global constructor keyed to_a_647")]
#[doc(alias = "__GLOBAL__I_a_647")]
// 0xc51d4c — __GLOBAL__I_a_647
// type: int()
pub fn stub_0xc51d4c() -> ! {
    todo!("0xc51d4c __GLOBAL__I_a_647")
}

#[doc(alias = "global constructor keyed to_a_648")]
#[doc(alias = "__GLOBAL__I_a_648")]
// 0xc5335c — __GLOBAL__I_a_648
// type: int()
pub fn stub_0xc5335c() -> ! {
    todo!("0xc5335c __GLOBAL__I_a_648")
}

#[doc(alias = "global constructor keyed to_a_649")]
#[doc(alias = "__GLOBAL__I_a_649")]
// 0xc556f4 — __GLOBAL__I_a_649
// type: int()
pub fn stub_0xc556f4() -> ! {
    todo!("0xc556f4 __GLOBAL__I_a_649")
}

#[doc(alias = "global constructor keyed to_a_650")]
#[doc(alias = "__GLOBAL__I_a_650")]
// 0xc56b84 — __GLOBAL__I_a_650
pub fn stub_0xc56b84() -> ! {
    todo!("0xc56b84 __GLOBAL__I_a_650")
}

#[doc(alias = "global constructor keyed to_a_651")]
#[doc(alias = "__GLOBAL__I_a_651")]
// 0xc59b44 — __GLOBAL__I_a_651
pub fn stub_0xc59b44() -> ! {
    todo!("0xc59b44 __GLOBAL__I_a_651")
}

#[doc(alias = "global constructor keyed to_a_652")]
#[doc(alias = "__GLOBAL__I_a_652")]
// 0xc59c50 — __GLOBAL__I_a_652
pub fn stub_0xc59c50() -> ! {
    todo!("0xc59c50 __GLOBAL__I_a_652")
}

#[doc(alias = "global constructor keyed to_a_653")]
#[doc(alias = "__GLOBAL__I_a_653")]
// 0xc5dc9c — __GLOBAL__I_a_653
pub fn stub_0xc5dc9c() -> ! {
    todo!("0xc5dc9c __GLOBAL__I_a_653")
}

#[doc(alias = "global constructor keyed to_a_654")]
#[doc(alias = "__GLOBAL__I_a_654")]
// 0xc60608 — __GLOBAL__I_a_654
pub fn stub_0xc60608() -> ! {
    todo!("0xc60608 __GLOBAL__I_a_654")
}

#[doc(alias = "global constructor keyed to_a_655")]
#[doc(alias = "__GLOBAL__I_a_655")]
// 0xc65ae4 — __GLOBAL__I_a_655
pub fn stub_0xc65ae4() -> ! {
    todo!("0xc65ae4 __GLOBAL__I_a_655")
}

#[doc(alias = "global constructor keyed to_a_656")]
#[doc(alias = "__GLOBAL__I_a_656")]
// 0xc65f2c — __GLOBAL__I_a_656
pub fn stub_0xc65f2c() -> ! {
    todo!("0xc65f2c __GLOBAL__I_a_656")
}

#[doc(alias = "global constructor keyed to_a_657")]
#[doc(alias = "__GLOBAL__I_a_657")]
// 0xc6be74 — __GLOBAL__I_a_657
pub fn stub_0xc6be74() -> ! {
    todo!("0xc6be74 __GLOBAL__I_a_657")
}

#[doc(alias = "global constructor keyed to_a_658")]
#[doc(alias = "__GLOBAL__I_a_658")]
// 0xc6f184 — __GLOBAL__I_a_658
pub fn stub_0xc6f184() -> ! {
    todo!("0xc6f184 __GLOBAL__I_a_658")
}

#[doc(alias = "global constructor keyed to_a_659")]
#[doc(alias = "__GLOBAL__I_a_659")]
// 0xc6fe6c — __GLOBAL__I_a_659
pub fn stub_0xc6fe6c() -> ! {
    todo!("0xc6fe6c __GLOBAL__I_a_659")
}

#[doc(alias = "global constructor keyed to_a_660")]
#[doc(alias = "__GLOBAL__I_a_660")]
// 0xc70098 — __GLOBAL__I_a_660
pub fn stub_0xc70098() -> ! {
    todo!("0xc70098 __GLOBAL__I_a_660")
}

#[doc(alias = "global constructor keyed to_a_661")]
#[doc(alias = "__GLOBAL__I_a_661")]
// 0xc701f4 — __GLOBAL__I_a_661
pub fn stub_0xc701f4() -> ! {
    todo!("0xc701f4 __GLOBAL__I_a_661")
}

#[doc(alias = "global constructor keyed to_a_662")]
#[doc(alias = "__GLOBAL__I_a_662")]
// 0xc70aa4 — __GLOBAL__I_a_662
pub fn stub_0xc70aa4() -> ! {
    todo!("0xc70aa4 __GLOBAL__I_a_662")
}

#[doc(alias = "global constructor keyed to_a_663")]
#[doc(alias = "__GLOBAL__I_a_663")]
// 0xc71054 — __GLOBAL__I_a_663
pub fn stub_0xc71054() -> ! {
    todo!("0xc71054 __GLOBAL__I_a_663")
}

#[doc(alias = "global constructor keyed to_a_664")]
#[doc(alias = "__GLOBAL__I_a_664")]
// 0xc71afc — __GLOBAL__I_a_664
pub fn stub_0xc71afc() -> ! {
    todo!("0xc71afc __GLOBAL__I_a_664")
}

#[doc(alias = "global constructor keyed to_a_665")]
#[doc(alias = "__GLOBAL__I_a_665")]
// 0xc74ff8 — __GLOBAL__I_a_665
pub fn stub_0xc74ff8() -> ! {
    todo!("0xc74ff8 __GLOBAL__I_a_665")
}

#[doc(alias = "global constructor keyed to_a_666")]
#[doc(alias = "__GLOBAL__I_a_666")]
// 0xc7509c — __GLOBAL__I_a_666
pub fn stub_0xc7509c() -> ! {
    todo!("0xc7509c __GLOBAL__I_a_666")
}

#[doc(alias = "global constructor keyed to_a_667")]
#[doc(alias = "__GLOBAL__I_a_667")]
// 0xc7542c — __GLOBAL__I_a_667
pub fn stub_0xc7542c() -> ! {
    todo!("0xc7542c __GLOBAL__I_a_667")
}

#[doc(alias = "global constructor keyed to_a_668")]
#[doc(alias = "__GLOBAL__I_a_668")]
// 0xc7699c — __GLOBAL__I_a_668
pub fn stub_0xc7699c() -> ! {
    todo!("0xc7699c __GLOBAL__I_a_668")
}

#[doc(alias = "global constructor keyed to_a_669")]
#[doc(alias = "__GLOBAL__I_a_669")]
// 0xc78cc0 — __GLOBAL__I_a_669
pub fn stub_0xc78cc0() -> ! {
    todo!("0xc78cc0 __GLOBAL__I_a_669")
}

#[doc(alias = "global constructor keyed to_a_670")]
#[doc(alias = "__GLOBAL__I_a_670")]
// 0xc7b0dc — __GLOBAL__I_a_670
pub fn stub_0xc7b0dc() -> ! {
    todo!("0xc7b0dc __GLOBAL__I_a_670")
}

#[doc(alias = "global constructor keyed to_a_671")]
#[doc(alias = "__GLOBAL__I_a_671")]
// 0xc7b3ec — __GLOBAL__I_a_671
pub fn stub_0xc7b3ec() -> ! {
    todo!("0xc7b3ec __GLOBAL__I_a_671")
}

#[doc(alias = "global constructor keyed to_a_672")]
#[doc(alias = "__GLOBAL__I_a_672")]
// 0xc7cd90 — __GLOBAL__I_a_672
pub fn stub_0xc7cd90() -> ! {
    todo!("0xc7cd90 __GLOBAL__I_a_672")
}

#[doc(alias = "global constructor keyed to_a_673")]
#[doc(alias = "__GLOBAL__I_a_673")]
// 0xc7f034 — __GLOBAL__I_a_673
pub fn stub_0xc7f034() -> ! {
    todo!("0xc7f034 __GLOBAL__I_a_673")
}

#[doc(alias = "global constructor keyed to_a_674")]
#[doc(alias = "__GLOBAL__I_a_674")]
// 0xc7f570 — __GLOBAL__I_a_674
pub fn stub_0xc7f570() -> ! {
    todo!("0xc7f570 __GLOBAL__I_a_674")
}

#[doc(alias = "global constructor keyed to_a_675")]
#[doc(alias = "__GLOBAL__I_a_675")]
// 0xc80820 — __GLOBAL__I_a_675
pub fn stub_0xc80820() -> ! {
    todo!("0xc80820 __GLOBAL__I_a_675")
}

#[doc(alias = "global constructor keyed to_a_676")]
#[doc(alias = "__GLOBAL__I_a_676")]
// 0xc80bf0 — __GLOBAL__I_a_676
pub fn stub_0xc80bf0() -> ! {
    todo!("0xc80bf0 __GLOBAL__I_a_676")
}

#[doc(alias = "global constructor keyed to_a_677")]
#[doc(alias = "__GLOBAL__I_a_677")]
// 0xc814b0 — __GLOBAL__I_a_677
pub fn stub_0xc814b0() -> ! {
    todo!("0xc814b0 __GLOBAL__I_a_677")
}

#[doc(alias = "global constructor keyed to_a_678")]
#[doc(alias = "__GLOBAL__I_a_678")]
// 0xc8203c — __GLOBAL__I_a_678
pub fn stub_0xc8203c() -> ! {
    todo!("0xc8203c __GLOBAL__I_a_678")
}

#[doc(alias = "global constructor keyed to_a_679")]
#[doc(alias = "__GLOBAL__I_a_679")]
// 0xc84a6c — __GLOBAL__I_a_679
pub fn stub_0xc84a6c() -> ! {
    todo!("0xc84a6c __GLOBAL__I_a_679")
}

#[doc(alias = "global constructor keyed to_a_680")]
#[doc(alias = "__GLOBAL__I_a_680")]
// 0xc8b57c — __GLOBAL__I_a_680
pub fn stub_0xc8b57c() -> ! {
    todo!("0xc8b57c __GLOBAL__I_a_680")
}

#[doc(alias = "global constructor keyed to_a_681")]
#[doc(alias = "__GLOBAL__I_a_681")]
// 0xc8be10 — __GLOBAL__I_a_681
pub fn stub_0xc8be10() -> ! {
    todo!("0xc8be10 __GLOBAL__I_a_681")
}

#[doc(alias = "global constructor keyed to_a_682")]
#[doc(alias = "__GLOBAL__I_a_682")]
// 0xc8c1ac — __GLOBAL__I_a_682
// type: int()
pub fn stub_0xc8c1ac() -> ! {
    todo!("0xc8c1ac __GLOBAL__I_a_682")
}

#[doc(alias = "global constructor keyed to_a_683")]
#[doc(alias = "__GLOBAL__I_a_683")]
// 0xc8ecb0 — __GLOBAL__I_a_683
pub fn stub_0xc8ecb0() -> ! {
    todo!("0xc8ecb0 __GLOBAL__I_a_683")
}

#[doc(alias = "global constructor keyed to_a_684")]
#[doc(alias = "__GLOBAL__I_a_684")]
// 0xc946ec — __GLOBAL__I_a_684
pub fn stub_0xc946ec() -> ! {
    todo!("0xc946ec __GLOBAL__I_a_684")
}

#[doc(alias = "global constructor keyed to_a_685")]
#[doc(alias = "__GLOBAL__I_a_685")]
// 0xc96d1c — __GLOBAL__I_a_685
pub fn stub_0xc96d1c() -> ! {
    todo!("0xc96d1c __GLOBAL__I_a_685")
}

#[doc(alias = "global constructor keyed to_a_686")]
#[doc(alias = "__GLOBAL__I_a_686")]
// 0xc98c40 — __GLOBAL__I_a_686
pub fn stub_0xc98c40() -> ! {
    todo!("0xc98c40 __GLOBAL__I_a_686")
}

#[doc(alias = "global constructor keyed to_a_687")]
#[doc(alias = "__GLOBAL__I_a_687")]
// 0xc9c374 — __GLOBAL__I_a_687
pub fn stub_0xc9c374() -> ! {
    todo!("0xc9c374 __GLOBAL__I_a_687")
}

#[doc(alias = "global constructor keyed to_a_688")]
#[doc(alias = "__GLOBAL__I_a_688")]
// 0xc9f7e8 — __GLOBAL__I_a_688
pub fn stub_0xc9f7e8() -> ! {
    todo!("0xc9f7e8 __GLOBAL__I_a_688")
}

#[doc(alias = "global constructor keyed to_a_689")]
#[doc(alias = "__GLOBAL__I_a_689")]
// 0xca39e8 — __GLOBAL__I_a_689
pub fn stub_0xca39e8() -> ! {
    todo!("0xca39e8 __GLOBAL__I_a_689")
}

#[doc(alias = "___cxx_global_var_init1")]
// 0xcac054 — ___cxx_global_var_init1
pub fn stub_0xcac054() -> ! {
    todo!("0xcac054 ___cxx_global_var_init1")
}

#[doc(alias = "___cxx_global_array_dtor_2")]
// 0xcb42bc — ___cxx_global_array_dtor_2
pub fn stub_0xcb42bc() -> ! {
    todo!("0xcb42bc ___cxx_global_array_dtor_2")
}

#[doc(alias = "global constructor keyed to_a_690")]
#[doc(alias = "__GLOBAL__I_a_690")]
// 0xcb432c — __GLOBAL__I_a_690
pub fn stub_0xcb432c() -> ! {
    todo!("0xcb432c __GLOBAL__I_a_690")
}

#[doc(alias = "global constructor keyed to_a_691")]
#[doc(alias = "__GLOBAL__I_a_691")]
// 0xcb5c18 — __GLOBAL__I_a_691
pub fn stub_0xcb5c18() -> ! {
    todo!("0xcb5c18 __GLOBAL__I_a_691")
}

#[doc(alias = "global constructor keyed to_a_692")]
#[doc(alias = "__GLOBAL__I_a_692")]
// 0xcb8b2c — __GLOBAL__I_a_692
// type: int()
pub fn stub_0xcb8b2c() -> ! {
    todo!("0xcb8b2c __GLOBAL__I_a_692")
}

#[doc(alias = "global constructor keyed to_a_693")]
#[doc(alias = "__GLOBAL__I_a_693")]
// 0xcb8f3c — __GLOBAL__I_a_693
// type: int()
pub fn stub_0xcb8f3c() -> ! {
    todo!("0xcb8f3c __GLOBAL__I_a_693")
}
