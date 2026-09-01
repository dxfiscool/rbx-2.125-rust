//! core shard kr — 120 stubs EA-sorted asc global gap filler not yet in core (fallback filter).
//! Source: `ida/export.json` (85545 funcs) EA-sorted asc, next 120 smallest not yet in rbx_core after kq 0xec5d84 (fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound; 33260 filtered, 4583 remaining before -> 4463 after, rbx_core::SharedPtr not boost).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + #[doc(alias = mangled)] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "-[GAIDispatcher dispatchTimer]")]
#[doc(alias = "-[GAIDispatcher dispatchTimer]")]
// 0xec5d98 — -[GAIDispatcher dispatchTimer]
// type: NSTimer *__cdecl(GAIDispatcher *self, SEL)
pub fn stub_0xec5d98() -> ! {
    todo!("0xec5d98 -[GAIDispatcher dispatchTimer]")
}

#[doc(alias = "-[GAIDispatcher setDispatchTimer:]")]
#[doc(alias = "-[GAIDispatcher setDispatchTimer:]")]
// 0xec5da8 — -[GAIDispatcher setDispatchTimer:]
// type: void __cdecl(GAIDispatcher *self, SEL, id)
pub fn stub_0xec5da8() -> ! {
    todo!("0xec5da8 -[GAIDispatcher setDispatchTimer:]")
}

#[doc(alias = "-[GAIDispatcher urlConnectionClass]")]
#[doc(alias = "-[GAIDispatcher urlConnectionClass]")]
// 0xec5dcc — -[GAIDispatcher urlConnectionClass]
// type: Class __cdecl(GAIDispatcher *self, SEL)
pub fn stub_0xec5dcc() -> ! {
    todo!("0xec5dcc -[GAIDispatcher urlConnectionClass]")
}

#[doc(alias = "-[GAIDispatcher setUrlConnectionClass:]")]
#[doc(alias = "-[GAIDispatcher setUrlConnectionClass:]")]
// 0xec5ddc — -[GAIDispatcher setUrlConnectionClass:]
// type: void __cdecl(GAIDispatcher *self, SEL, Class)
pub fn stub_0xec5ddc() -> ! {
    todo!("0xec5ddc -[GAIDispatcher setUrlConnectionClass:]")
}

#[doc(alias = "_tf_am_i_being_debugged")]
#[doc(alias = "_tf_am_i_being_debugged")]
// 0xec5dec — _tf_am_i_being_debugged
pub fn stub_0xec5dec() -> ! {
    todo!("0xec5dec _tf_am_i_being_debugged")
}

#[doc(alias = "+[TFApplicationInformation bundleInformation]")]
#[doc(alias = "+[TFApplicationInformation bundleInformation]")]
// 0xec5e70 — +[TFApplicationInformation bundleInformation]
// type: id __cdecl(id, SEL)
pub fn stub_0xec5e70() -> ! {
    todo!("0xec5e70 +[TFApplicationInformation bundleInformation]")
}

#[doc(alias = "+[TFApplicationInformation setApplicationToken:]")]
#[doc(alias = "+[TFApplicationInformation setApplicationToken:]")]
// 0xec61d8 — +[TFApplicationInformation setApplicationToken:]
// type: void __cdecl(id, SEL, id)
pub fn stub_0xec61d8() -> ! {
    todo!("0xec61d8 +[TFApplicationInformation setApplicationToken:]")
}

#[doc(alias = "_tf_get_udid_from_mach_header")]
#[doc(alias = "_tf_get_udid_from_mach_header")]
// 0xec6258 — _tf_get_udid_from_mach_header
// type: id __fastcall(int)
pub fn stub_0xec6258() -> ! {
    todo!("0xec6258 _tf_get_udid_from_mach_header")
}

#[doc(alias = "___tf_get_udid_from_mach_header_block_invoke")]
#[doc(alias = "___tf_get_udid_from_mach_header_block_invoke")]
// 0xec62b8 — ___tf_get_udid_from_mach_header_block_invoke
pub fn stub_0xec62b8() -> ! {
    todo!("0xec62b8 ___tf_get_udid_from_mach_header_block_invoke")
}

#[doc(alias = "+[TFApplicationInformation bundleUUID]")]
#[doc(alias = "+[TFApplicationInformation bundleUUID]")]
// 0xec6360 — +[TFApplicationInformation bundleUUID]
// type: id __cdecl(id, SEL)
pub fn stub_0xec6360() -> ! {
    todo!("0xec6360 +[TFApplicationInformation bundleUUID]")
}

#[doc(alias = "+[TFApplicationInformation getApplicationVersion]")]
#[doc(alias = "+[TFApplicationInformation getApplicationVersion]")]
// 0xec6370 — +[TFApplicationInformation getApplicationVersion]
// type: id __cdecl(id, SEL)
pub fn stub_0xec6370() -> ! {
    todo!("0xec6370 +[TFApplicationInformation getApplicationVersion]")
}

#[doc(alias = "___49+[TFApplicationInformation getApplicationVersion]_block_invoke")]
#[doc(alias = "___49+[TFApplicationInformation getApplicationVersion]_block_invoke")]
// 0xec63a4 — ___49+[TFApplicationInformation getApplicationVersion]_block_invoke
// type: void __cdecl(id)
pub fn stub_0xec63a4() -> ! {
    todo!("0xec63a4 ___49+[TFApplicationInformation getApplicationVersion]_block_invoke")
}

#[doc(alias = "+[TFApplicationInformation hasEncryptionSet:]")]
#[doc(alias = "+[TFApplicationInformation hasEncryptionSet:]")]
// 0xec6528 — +[TFApplicationInformation hasEncryptionSet:]
// type: char __cdecl(id, SEL, id *)
pub fn stub_0xec6528() -> ! {
    todo!("0xec6528 +[TFApplicationInformation hasEncryptionSet:]")
}

#[doc(alias = "__tf_str_seach_cmp")]
#[doc(alias = "__tf_str_seach_cmp")]
// 0xec65e4 — __tf_str_seach_cmp
pub fn stub_0xec65e4() -> ! {
    todo!("0xec65e4 __tf_str_seach_cmp")
}

#[doc(alias = "+[TFApplicationInformation getDistributionMethod:]")]
#[doc(alias = "+[TFApplicationInformation getDistributionMethod:]")]
// 0xec6644 — +[TFApplicationInformation getDistributionMethod:]
// type: id __cdecl(id, SEL, id *)
pub fn stub_0xec6644() -> ! {
    todo!("0xec6644 +[TFApplicationInformation getDistributionMethod:]")
}

#[doc(alias = "___50+[TFApplicationInformation getDistributionMethod:]_block_invoke")]
#[doc(alias = "___50+[TFApplicationInformation getDistributionMethod:]_block_invoke")]
// 0xec6680 — ___50+[TFApplicationInformation getDistributionMethod:]_block_invoke
// type: void __cdecl(id)
pub fn stub_0xec6680() -> ! {
    todo!("0xec6680 ___50+[TFApplicationInformation getDistributionMethod:]_block_invoke")
}

#[doc(alias = "+[TFApplicationInformation allowInAppUpdate]")]
#[doc(alias = "+[TFApplicationInformation allowInAppUpdate]")]
// 0xec6b18 — +[TFApplicationInformation allowInAppUpdate]
// type: char __cdecl(id, SEL)
pub fn stub_0xec6b18() -> ! {
    todo!("0xec6b18 +[TFApplicationInformation allowInAppUpdate]")
}

#[doc(alias = "___44+[TFApplicationInformation allowInAppUpdate]_block_invoke")]
#[doc(alias = "___44+[TFApplicationInformation allowInAppUpdate]_block_invoke")]
// 0xec6b50 — ___44+[TFApplicationInformation allowInAppUpdate]_block_invoke
// type: void __cdecl(id)
pub fn stub_0xec6b50() -> ! {
    todo!("0xec6b50 ___44+[TFApplicationInformation allowInAppUpdate]_block_invoke")
}

#[doc(alias = "+[TFApplicationInformation allowDeviceIdentifier]")]
#[doc(alias = "+[TFApplicationInformation allowDeviceIdentifier]")]
// 0xec6c3c — +[TFApplicationInformation allowDeviceIdentifier]
// type: char __cdecl(id, SEL)
pub fn stub_0xec6c3c() -> ! {
    todo!("0xec6c3c +[TFApplicationInformation allowDeviceIdentifier]")
}

#[doc(alias = "___49+[TFApplicationInformation allowDeviceIdentifier]_block_invoke")]
#[doc(alias = "___49+[TFApplicationInformation allowDeviceIdentifier]_block_invoke")]
// 0xec6c74 — ___49+[TFApplicationInformation allowDeviceIdentifier]_block_invoke
// type: void __cdecl(id)
pub fn stub_0xec6c74() -> ! {
    todo!("0xec6c74 ___49+[TFApplicationInformation allowDeviceIdentifier]_block_invoke")
}

#[doc(alias = "_tf_application_cracked_info")]
#[doc(alias = "_tf_application_cracked_info")]
// 0xec6d60 — _tf_application_cracked_info
pub fn stub_0xec6d60() -> ! {
    todo!("0xec6d60 _tf_application_cracked_info")
}

#[doc(alias = "+[TFCompatibility allowedToRun_5_0]")]
#[doc(alias = "+[TFCompatibility allowedToRun_5_0]")]
// 0xec72b4 — +[TFCompatibility allowedToRun_5_0]
// type: char __cdecl(id, SEL)
pub fn stub_0xec72b4() -> ! {
    todo!("0xec72b4 +[TFCompatibility allowedToRun_5_0]")
}

#[doc(alias = "___35+[TFCompatibility allowedToRun_5_0]_block_invoke")]
#[doc(alias = "___35+[TFCompatibility allowedToRun_5_0]_block_invoke")]
// 0xec72f4 — ___35+[TFCompatibility allowedToRun_5_0]_block_invoke
// type: void __cdecl(id)
pub fn stub_0xec72f4() -> ! {
    todo!("0xec72f4 ___35+[TFCompatibility allowedToRun_5_0]_block_invoke")
}

#[doc(alias = "+[TFCompatibility allowedToRun_6_0]")]
#[doc(alias = "+[TFCompatibility allowedToRun_6_0]")]
// 0xec7378 — +[TFCompatibility allowedToRun_6_0]
// type: char __cdecl(id, SEL)
pub fn stub_0xec7378() -> ! {
    todo!("0xec7378 +[TFCompatibility allowedToRun_6_0]")
}

#[doc(alias = "___35+[TFCompatibility allowedToRun_6_0]_block_invoke")]
#[doc(alias = "___35+[TFCompatibility allowedToRun_6_0]_block_invoke")]
// 0xec73b0 — ___35+[TFCompatibility allowedToRun_6_0]_block_invoke
// type: void __cdecl(id)
pub fn stub_0xec73b0() -> ! {
    todo!("0xec73b0 ___35+[TFCompatibility allowedToRun_6_0]_block_invoke")
}

#[doc(alias = "+[TFCompatibility isIdiomIPhone]")]
#[doc(alias = "+[TFCompatibility isIdiomIPhone]")]
// 0xec7434 — +[TFCompatibility isIdiomIPhone]
// type: char __cdecl(id, SEL)
pub fn stub_0xec7434() -> ! {
    todo!("0xec7434 +[TFCompatibility isIdiomIPhone]")
}

#[doc(alias = "___32+[TFCompatibility isIdiomIPhone]_block_invoke")]
#[doc(alias = "___32+[TFCompatibility isIdiomIPhone]_block_invoke")]
// 0xec746c — ___32+[TFCompatibility isIdiomIPhone]_block_invoke
// type: void __cdecl(id)
pub fn stub_0xec746c() -> ! {
    todo!("0xec746c ___32+[TFCompatibility isIdiomIPhone]_block_invoke")
}

#[doc(alias = "+[TFCompatibility isIdiomIPad]")]
#[doc(alias = "+[TFCompatibility isIdiomIPad]")]
// 0xec74c4 — +[TFCompatibility isIdiomIPad]
// type: char __cdecl(id, SEL)
pub fn stub_0xec74c4() -> ! {
    todo!("0xec74c4 +[TFCompatibility isIdiomIPad]")
}

#[doc(alias = "___30+[TFCompatibility isIdiomIPad]_block_invoke")]
#[doc(alias = "___30+[TFCompatibility isIdiomIPad]_block_invoke")]
// 0xec74fc — ___30+[TFCompatibility isIdiomIPad]_block_invoke
// type: void __cdecl(id)
pub fn stub_0xec74fc() -> ! {
    todo!("0xec74fc ___30+[TFCompatibility isIdiomIPad]_block_invoke")
}

#[doc(alias = "+[TFCrypto sha1:]")]
#[doc(alias = "+[TFCrypto sha1:]")]
// 0xec7550 — +[TFCrypto sha1:]
// type: id __cdecl(id, SEL, id)
pub fn stub_0xec7550() -> ! {
    todo!("0xec7550 +[TFCrypto sha1:]")
}

#[doc(alias = "+[TFDeviceInfo initialize]")]
#[doc(alias = "+[TFDeviceInfo initialize]")]
// 0xec769c — +[TFDeviceInfo initialize]
// type: void __cdecl(id, SEL)
pub fn stub_0xec769c() -> ! {
    todo!("0xec769c +[TFDeviceInfo initialize]")
}

#[doc(alias = "___26+[TFDeviceInfo initialize]_block_invoke")]
#[doc(alias = "___26+[TFDeviceInfo initialize]_block_invoke")]
// 0xec76d0 — ___26+[TFDeviceInfo initialize]_block_invoke
// type: void __cdecl(id)
pub fn stub_0xec76d0() -> ! {
    todo!("0xec76d0 ___26+[TFDeviceInfo initialize]_block_invoke")
}

#[doc(alias = "+[TFDeviceInfo addCustomEnvironmentInformation:forKey:]")]
#[doc(alias = "+[TFDeviceInfo addCustomEnvironmentInformation:forKey:]")]
// 0xec7718 — +[TFDeviceInfo addCustomEnvironmentInformation:forKey:]
// type: void __cdecl(id, SEL, id, id)
pub fn stub_0xec7718() -> ! {
    todo!("0xec7718 +[TFDeviceInfo addCustomEnvironmentInformation:forKey:]")
}

#[doc(alias = "+[TFDeviceInfo setDeviceIdentifier:]")]
#[doc(alias = "+[TFDeviceInfo setDeviceIdentifier:]")]
// 0xec77fc — +[TFDeviceInfo setDeviceIdentifier:]
// type: void __cdecl(id, SEL, id)
pub fn stub_0xec77fc() -> ! {
    todo!("0xec77fc +[TFDeviceInfo setDeviceIdentifier:]")
}

#[doc(alias = "+[TFDeviceInfo deviceInformation]")]
#[doc(alias = "+[TFDeviceInfo deviceInformation]")]
// 0xec78b0 — +[TFDeviceInfo deviceInformation]
// type: id __cdecl(id, SEL)
pub fn stub_0xec78b0() -> ! {
    todo!("0xec78b0 +[TFDeviceInfo deviceInformation]")
}

#[doc(alias = "+[TFDeviceInfo machine]")]
#[doc(alias = "+[TFDeviceInfo machine]")]
// 0xec7ef8 — +[TFDeviceInfo machine]
// type: id __cdecl(id, SEL)
pub fn stub_0xec7ef8() -> ! {
    todo!("0xec7ef8 +[TFDeviceInfo machine]")
}

#[doc(alias = "+[TFDeviceInfo architecture]")]
#[doc(alias = "+[TFDeviceInfo architecture]")]
// 0xec7f70 — +[TFDeviceInfo architecture]
// type: id __cdecl(id, SEL)
pub fn stub_0xec7f70() -> ! {
    todo!("0xec7f70 +[TFDeviceInfo architecture]")
}

#[doc(alias = "+[TFDeviceInfo carrier]")]
#[doc(alias = "+[TFDeviceInfo carrier]")]
// 0xec7fa4 — +[TFDeviceInfo carrier]
// type: id __cdecl(id, SEL)
pub fn stub_0xec7fa4() -> ! {
    todo!("0xec7fa4 +[TFDeviceInfo carrier]")
}

#[doc(alias = "___23+[TFDeviceInfo carrier]_block_invoke")]
#[doc(alias = "___23+[TFDeviceInfo carrier]_block_invoke")]
// 0xec7fd8 — ___23+[TFDeviceInfo carrier]_block_invoke
// type: void __cdecl(id)
pub fn stub_0xec7fd8() -> ! {
    todo!("0xec7fd8 ___23+[TFDeviceInfo carrier]_block_invoke")
}

#[doc(alias = "+[TFDeviceInfo orientation]")]
#[doc(alias = "+[TFDeviceInfo orientation]")]
// 0xec80d8 — +[TFDeviceInfo orientation]
// type: id __cdecl(id, SEL)
pub fn stub_0xec80d8() -> ! {
    todo!("0xec80d8 +[TFDeviceInfo orientation]")
}

#[doc(alias = "+[TFDeviceInfo batteryState]")]
#[doc(alias = "+[TFDeviceInfo batteryState]")]
// 0xec818c — +[TFDeviceInfo batteryState]
// type: id __cdecl(id, SEL)
pub fn stub_0xec818c() -> ! {
    todo!("0xec818c +[TFDeviceInfo batteryState]")
}

#[doc(alias = "+[TFDeviceInfo connectivityState]")]
#[doc(alias = "+[TFDeviceInfo connectivityState]")]
// 0xec8218 — +[TFDeviceInfo connectivityState]
// type: id __cdecl(id, SEL)
pub fn stub_0xec8218() -> ! {
    todo!("0xec8218 +[TFDeviceInfo connectivityState]")
}

#[doc(alias = "_tf_device_jail_broken_info")]
#[doc(alias = "_tf_device_jail_broken_info")]
// 0xec8294 — _tf_device_jail_broken_info
pub fn stub_0xec8294() -> ! {
    todo!("0xec8294 _tf_device_jail_broken_info")
}

#[doc(alias = "_tf_remote_log_info_open_file")]
#[doc(alias = "_tf_remote_log_info_open_file")]
// 0xec856c — _tf_remote_log_info_open_file
pub fn stub_0xec856c() -> ! {
    todo!("0xec856c _tf_remote_log_info_open_file")
}

#[doc(alias = "_tf_remote_log_file_new_uncompressed_file")]
#[doc(alias = "_tf_remote_log_file_new_uncompressed_file")]
// 0xec85f0 — _tf_remote_log_file_new_uncompressed_file
// type: int(void)
pub fn stub_0xec85f0() -> ! {
    todo!("0xec85f0 _tf_remote_log_file_new_uncompressed_file")
}

#[doc(alias = "_tf_remote_log_info_init")]
#[doc(alias = "_tf_remote_log_info_init")]
// 0xec8664 — _tf_remote_log_info_init
pub fn stub_0xec8664() -> ! {
    todo!("0xec8664 _tf_remote_log_info_init")
}

#[doc(alias = "_tf_remote_log_get_log_path")]
#[doc(alias = "_tf_remote_log_get_log_path")]
// 0xec86e8 — _tf_remote_log_get_log_path
pub fn stub_0xec86e8() -> ! {
    todo!("0xec86e8 _tf_remote_log_get_log_path")
}

#[doc(alias = "_tf_remote_log_info_open")]
#[doc(alias = "_tf_remote_log_info_open")]
// 0xec8744 — _tf_remote_log_info_open
// type: int()
pub fn stub_0xec8744() -> ! {
    todo!("0xec8744 _tf_remote_log_info_open")
}

#[doc(alias = "_tf_remote_log_info_close")]
#[doc(alias = "_tf_remote_log_info_close")]
// 0xec8758 — _tf_remote_log_info_close
pub fn stub_0xec8758() -> ! {
    todo!("0xec8758 _tf_remote_log_info_close")
}

#[doc(alias = "__tf_remote_log_compress_data")]
#[doc(alias = "__tf_remote_log_compress_data")]
// 0xec8780 — __tf_remote_log_compress_data
pub fn stub_0xec8780() -> ! {
    todo!("0xec8780 __tf_remote_log_compress_data")
}

#[doc(alias = "_tf_remote_log_add_entry")]
#[doc(alias = "_tf_remote_log_add_entry")]
// 0xec889c — _tf_remote_log_add_entry
// type: int __fastcall(int, void *__buf, size_t __nbyte)
pub fn stub_0xec889c() -> ! {
    todo!("0xec889c _tf_remote_log_add_entry")
}

#[doc(alias = "_tf_remote_log_finalize_to_file_safe")]
#[doc(alias = "_tf_remote_log_finalize_to_file_safe")]
// 0xec8984 — _tf_remote_log_finalize_to_file_safe
pub fn stub_0xec8984() -> ! {
    todo!("0xec8984 _tf_remote_log_finalize_to_file_safe")
}

#[doc(alias = "_tf_remote_log_delete_safe")]
#[doc(alias = "_tf_remote_log_delete_safe")]
// 0xec8b68 — _tf_remote_log_delete_safe
pub fn stub_0xec8b68() -> ! {
    todo!("0xec8b68 _tf_remote_log_delete_safe")
}

#[doc(alias = "_tf_log_console")]
#[doc(alias = "_tf_log_console")]
// 0xec8be4 — _tf_log_console
pub fn stub_0xec8be4() -> ! {
    todo!("0xec8be4 _tf_log_console")
}

#[doc(alias = "___tf_log_console_block_invoke")]
#[doc(alias = "___tf_log_console_block_invoke")]
// 0xec8c2c — ___tf_log_console_block_invoke
// type: void __cdecl(id)
pub fn stub_0xec8c2c() -> ! {
    todo!("0xec8c2c ___tf_log_console_block_invoke")
}

#[doc(alias = "+[TFLogManager sharedManager]")]
#[doc(alias = "+[TFLogManager sharedManager]")]
// 0xec8c50 — +[TFLogManager sharedManager]
// type: id __cdecl(id, SEL)
pub fn stub_0xec8c50() -> ! {
    todo!("0xec8c50 +[TFLogManager sharedManager]")
}

#[doc(alias = "___29+[TFLogManager sharedManager]_block_invoke")]
#[doc(alias = "___29+[TFLogManager sharedManager]_block_invoke")]
// 0xec8c80 — ___29+[TFLogManager sharedManager]_block_invoke
// type: void __cdecl(id)
pub fn stub_0xec8c80() -> ! {
    todo!("0xec8c80 ___29+[TFLogManager sharedManager]_block_invoke")
}

#[doc(alias = "-[TFLogManager init]")]
#[doc(alias = "-[TFLogManager init]")]
// 0xec8cb8 — -[TFLogManager init]
// type: TFLogManager *__cdecl(TFLogManager *self, SEL)
pub fn stub_0xec8cb8() -> ! {
    todo!("0xec8cb8 -[TFLogManager init]")
}

#[doc(alias = "-[TFLogManager dealloc]")]
#[doc(alias = "-[TFLogManager dealloc]")]
// 0xec8d54 — -[TFLogManager dealloc]
// type: void __cdecl(TFLogManager *self, SEL)
pub fn stub_0xec8d54() -> ! {
    todo!("0xec8d54 -[TFLogManager dealloc]")
}

#[doc(alias = "-[TFLogManager setCurrentSessionID:]")]
#[doc(alias = "-[TFLogManager setCurrentSessionID:]")]
// 0xec8da4 — -[TFLogManager setCurrentSessionID:]
// type: void __cdecl(TFLogManager *self, SEL, id)
pub fn stub_0xec8da4() -> ! {
    todo!("0xec8da4 -[TFLogManager setCurrentSessionID:]")
}

#[doc(alias = "___36-[TFLogManager setCurrentSessionID:]_block_invoke")]
#[doc(alias = "___36-[TFLogManager setCurrentSessionID:]_block_invoke")]
// 0xec8e48 — ___36-[TFLogManager setCurrentSessionID:]_block_invoke
pub fn stub_0xec8e48() -> ! {
    todo!("0xec8e48 ___36-[TFLogManager setCurrentSessionID:]_block_invoke")
}

#[doc(alias = "___copy_helper_block__27")]
#[doc(alias = "___copy_helper_block__27")]
// 0xec8f3c — ___copy_helper_block__27
pub fn stub_0xec8f3c() -> ! {
    todo!("0xec8f3c ___copy_helper_block__27")
}

#[doc(alias = "___destroy_helper_block__27")]
#[doc(alias = "___destroy_helper_block__27")]
// 0xec8f50 — ___destroy_helper_block__27
pub fn stub_0xec8f50() -> ! {
    todo!("0xec8f50 ___destroy_helper_block__27")
}

#[doc(alias = "-[TFLogManager setLogToConsole:]")]
#[doc(alias = "-[TFLogManager setLogToConsole:]")]
// 0xec8f64 — -[TFLogManager setLogToConsole:]
// type: void __cdecl(TFLogManager *self, SEL, char)
pub fn stub_0xec8f64() -> ! {
    todo!("0xec8f64 -[TFLogManager setLogToConsole:]")
}

#[doc(alias = "___32-[TFLogManager setLogToConsole:]_block_invoke")]
#[doc(alias = "___32-[TFLogManager setLogToConsole:]_block_invoke")]
// 0xec8fe4 — ___32-[TFLogManager setLogToConsole:]_block_invoke
pub fn stub_0xec8fe4() -> ! {
    todo!("0xec8fe4 ___32-[TFLogManager setLogToConsole:]_block_invoke")
}

#[doc(alias = "___copy_helper_block_16")]
#[doc(alias = "___copy_helper_block_16")]
// 0xec8ff8 — ___copy_helper_block_16
pub fn stub_0xec8ff8() -> ! {
    todo!("0xec8ff8 ___copy_helper_block_16")
}

#[doc(alias = "___destroy_helper_block_17")]
#[doc(alias = "___destroy_helper_block_17")]
// 0xec9004 — ___destroy_helper_block_17
pub fn stub_0xec9004() -> ! {
    todo!("0xec9004 ___destroy_helper_block_17")
}

#[doc(alias = "-[TFLogManager setLogToSTDERR:]")]
#[doc(alias = "-[TFLogManager setLogToSTDERR:]")]
// 0xec9010 — -[TFLogManager setLogToSTDERR:]
// type: void __cdecl(TFLogManager *self, SEL, char)
pub fn stub_0xec9010() -> ! {
    todo!("0xec9010 -[TFLogManager setLogToSTDERR:]")
}

#[doc(alias = "___31-[TFLogManager setLogToSTDERR:]_block_invoke")]
#[doc(alias = "___31-[TFLogManager setLogToSTDERR:]_block_invoke")]
// 0xec9090 — ___31-[TFLogManager setLogToSTDERR:]_block_invoke
pub fn stub_0xec9090() -> ! {
    todo!("0xec9090 ___31-[TFLogManager setLogToSTDERR:]_block_invoke")
}

#[doc(alias = "___copy_helper_block_20")]
#[doc(alias = "___copy_helper_block_20")]
// 0xec90a4 — ___copy_helper_block_20
pub fn stub_0xec90a4() -> ! {
    todo!("0xec90a4 ___copy_helper_block_20")
}

#[doc(alias = "___destroy_helper_block_21")]
#[doc(alias = "___destroy_helper_block_21")]
// 0xec90b0 — ___destroy_helper_block_21
pub fn stub_0xec90b0() -> ! {
    todo!("0xec90b0 ___destroy_helper_block_21")
}

#[doc(alias = "-[TFLogManager log:]")]
#[doc(alias = "-[TFLogManager log:]")]
// 0xec90bc — -[TFLogManager log:]
// type: void __cdecl(TFLogManager *self, SEL, id)
pub fn stub_0xec90bc() -> ! {
    todo!("0xec90bc -[TFLogManager log:]")
}

#[doc(alias = "___20-[TFLogManager log:]_block_invoke")]
#[doc(alias = "___20-[TFLogManager log:]_block_invoke")]
// 0xec91b0 — ___20-[TFLogManager log:]_block_invoke
pub fn stub_0xec91b0() -> ! {
    todo!("0xec91b0 ___20-[TFLogManager log:]_block_invoke")
}

#[doc(alias = "__tf_log")]
#[doc(alias = "__tf_log")]
// 0xec91c0 — __tf_log
pub fn stub_0xec91c0() -> ! {
    todo!("0xec91c0 __tf_log")
}

#[doc(alias = "___copy_helper_block_27")]
#[doc(alias = "___copy_helper_block_27")]
// 0xec93bc — ___copy_helper_block_27
pub fn stub_0xec93bc() -> ! {
    todo!("0xec93bc ___copy_helper_block_27")
}

#[doc(alias = "___destroy_helper_block_28")]
#[doc(alias = "___destroy_helper_block_28")]
// 0xec93d8 — ___destroy_helper_block_28
pub fn stub_0xec93d8() -> ! {
    todo!("0xec93d8 ___destroy_helper_block_28")
}

#[doc(alias = "-[TFLogManager logAsync:]")]
#[doc(alias = "-[TFLogManager logAsync:]")]
// 0xec93f4 — -[TFLogManager logAsync:]
// type: void __cdecl(TFLogManager *self, SEL, id)
pub fn stub_0xec93f4() -> ! {
    todo!("0xec93f4 -[TFLogManager logAsync:]")
}

#[doc(alias = "___25-[TFLogManager logAsync:]_block_invoke")]
#[doc(alias = "___25-[TFLogManager logAsync:]_block_invoke")]
// 0xec94e8 — ___25-[TFLogManager logAsync:]_block_invoke
pub fn stub_0xec94e8() -> ! {
    todo!("0xec94e8 ___25-[TFLogManager logAsync:]_block_invoke")
}

#[doc(alias = "___copy_helper_block_31")]
#[doc(alias = "___copy_helper_block_31")]
// 0xec94f8 — ___copy_helper_block_31
pub fn stub_0xec94f8() -> ! {
    todo!("0xec94f8 ___copy_helper_block_31")
}

#[doc(alias = "___destroy_helper_block_32")]
#[doc(alias = "___destroy_helper_block_32")]
// 0xec9514 — ___destroy_helper_block_32
pub fn stub_0xec9514() -> ! {
    todo!("0xec9514 ___destroy_helper_block_32")
}

#[doc(alias = "____tf_log_block_invoke")]
#[doc(alias = "____tf_log_block_invoke")]
// 0xec9530 — ____tf_log_block_invoke
pub fn stub_0xec9530() -> ! {
    todo!("0xec9530 ____tf_log_block_invoke")
}

#[doc(alias = "___copy_helper_block_43")]
#[doc(alias = "___copy_helper_block_43")]
// 0xec958c — ___copy_helper_block_43
pub fn stub_0xec958c() -> ! {
    todo!("0xec958c ___copy_helper_block_43")
}

#[doc(alias = "___destroy_helper_block_44")]
#[doc(alias = "___destroy_helper_block_44")]
// 0xec9598 — ___destroy_helper_block_44
pub fn stub_0xec9598() -> ! {
    todo!("0xec9598 ___destroy_helper_block_44")
}

#[doc(alias = "____tf_log_block_invoke48")]
#[doc(alias = "____tf_log_block_invoke48")]
// 0xec95a4 — ____tf_log_block_invoke48
pub fn stub_0xec95a4() -> ! {
    todo!("0xec95a4 ____tf_log_block_invoke48")
}

#[doc(alias = "___copy_helper_block_51")]
#[doc(alias = "___copy_helper_block_51")]
// 0xec9600 — ___copy_helper_block_51
pub fn stub_0xec9600() -> ! {
    todo!("0xec9600 ___copy_helper_block_51")
}

#[doc(alias = "___destroy_helper_block_52")]
#[doc(alias = "___destroy_helper_block_52")]
// 0xec960c — ___destroy_helper_block_52
pub fn stub_0xec960c() -> ! {
    todo!("0xec960c ___destroy_helper_block_52")
}

#[doc(alias = "____tf_log_block_invoke56")]
#[doc(alias = "____tf_log_block_invoke56")]
// 0xec9618 — ____tf_log_block_invoke56
pub fn stub_0xec9618() -> ! {
    todo!("0xec9618 ____tf_log_block_invoke56")
}

#[doc(alias = "___copy_helper_block_59")]
#[doc(alias = "___copy_helper_block_59")]
// 0xec9674 — ___copy_helper_block_59
pub fn stub_0xec9674() -> ! {
    todo!("0xec9674 ___copy_helper_block_59")
}

#[doc(alias = "___destroy_helper_block_60")]
#[doc(alias = "___destroy_helper_block_60")]
// 0xec9680 — ___destroy_helper_block_60
pub fn stub_0xec9680() -> ! {
    todo!("0xec9680 ___destroy_helper_block_60")
}

#[doc(alias = "___tf_log_dispatch_queue_block_invoke")]
#[doc(alias = "___tf_log_dispatch_queue_block_invoke")]
// 0xec968c — ___tf_log_dispatch_queue_block_invoke
// type: void __cdecl(id)
pub fn stub_0xec968c() -> ! {
    todo!("0xec968c ___tf_log_dispatch_queue_block_invoke")
}

#[doc(alias = "__tf_log_folder_path")]
#[doc(alias = "__tf_log_folder_path")]
// 0xec96b0 — __tf_log_folder_path
pub fn stub_0xec96b0() -> ! {
    todo!("0xec96b0 __tf_log_folder_path")
}

#[doc(alias = "____tf_log_folder_path_block_invoke")]
#[doc(alias = "____tf_log_folder_path_block_invoke")]
// 0xec96e4 — ____tf_log_folder_path_block_invoke
// type: void __cdecl(id)
pub fn stub_0xec96e4() -> ! {
    todo!("0xec96e4 ____tf_log_folder_path_block_invoke")
}

#[doc(alias = "-[TFLogManager _pathForSessionID:]")]
#[doc(alias = "-[TFLogManager _pathForSessionID:]")]
// 0xec9734 — -[TFLogManager _pathForSessionID:]
// type: id __cdecl(TFLogManager *self, SEL, id)
pub fn stub_0xec9734() -> ! {
    todo!("0xec9734 -[TFLogManager _pathForSessionID:]")
}

#[doc(alias = "___copy_helper_block_90")]
#[doc(alias = "___copy_helper_block_90")]
// 0xec9b28 — ___copy_helper_block_90
pub fn stub_0xec9b28() -> ! {
    todo!("0xec9b28 ___copy_helper_block_90")
}

#[doc(alias = "___destroy_helper_block_91")]
#[doc(alias = "___destroy_helper_block_91")]
// 0xec9b4c — ___destroy_helper_block_91
pub fn stub_0xec9b4c() -> ! {
    todo!("0xec9b4c ___destroy_helper_block_91")
}

#[doc(alias = "-[TFLogManager sessionIDsWithOldLogs]")]
#[doc(alias = "-[TFLogManager sessionIDsWithOldLogs]")]
// 0xec9b68 — -[TFLogManager sessionIDsWithOldLogs]
// type: id __cdecl(TFLogManager *self, SEL)
pub fn stub_0xec9b68() -> ! {
    todo!("0xec9b68 -[TFLogManager sessionIDsWithOldLogs]")
}

#[doc(alias = "___37-[TFLogManager sessionIDsWithOldLogs]_block_invoke")]
#[doc(alias = "___37-[TFLogManager sessionIDsWithOldLogs]_block_invoke")]
// 0xec9dd0 — ___37-[TFLogManager sessionIDsWithOldLogs]_block_invoke
pub fn stub_0xec9dd0() -> ! {
    todo!("0xec9dd0 ___37-[TFLogManager sessionIDsWithOldLogs]_block_invoke")
}

#[doc(alias = "___copy_helper_block_110")]
#[doc(alias = "___copy_helper_block_110")]
// 0xec9e30 — ___copy_helper_block_110
pub fn stub_0xec9e30() -> ! {
    todo!("0xec9e30 ___copy_helper_block_110")
}

#[doc(alias = "___destroy_helper_block_111")]
#[doc(alias = "___destroy_helper_block_111")]
// 0xec9e44 — ___destroy_helper_block_111
pub fn stub_0xec9e44() -> ! {
    todo!("0xec9e44 ___destroy_helper_block_111")
}

#[doc(alias = "-[TFLogManager deleteLogFor:]")]
#[doc(alias = "-[TFLogManager deleteLogFor:]")]
// 0xec9e58 — -[TFLogManager deleteLogFor:]
// type: void __cdecl(TFLogManager *self, SEL, id)
pub fn stub_0xec9e58() -> ! {
    todo!("0xec9e58 -[TFLogManager deleteLogFor:]")
}

#[doc(alias = "___29-[TFLogManager deleteLogFor:]_block_invoke")]
#[doc(alias = "___29-[TFLogManager deleteLogFor:]_block_invoke")]
// 0xec9efc — ___29-[TFLogManager deleteLogFor:]_block_invoke
pub fn stub_0xec9efc() -> ! {
    todo!("0xec9efc ___29-[TFLogManager deleteLogFor:]_block_invoke")
}

#[doc(alias = "___copy_helper_block_114")]
#[doc(alias = "___copy_helper_block_114")]
// 0xec9f1c — ___copy_helper_block_114
pub fn stub_0xec9f1c() -> ! {
    todo!("0xec9f1c ___copy_helper_block_114")
}

#[doc(alias = "___destroy_helper_block_115")]
#[doc(alias = "___destroy_helper_block_115")]
// 0xec9f30 — ___destroy_helper_block_115
pub fn stub_0xec9f30() -> ! {
    todo!("0xec9f30 ___destroy_helper_block_115")
}

#[doc(alias = "-[TFLogManager internalDeleteLogFor:]")]
#[doc(alias = "-[TFLogManager internalDeleteLogFor:]")]
// 0xec9f44 — -[TFLogManager internalDeleteLogFor:]
// type: void __cdecl(TFLogManager *self, SEL, id)
pub fn stub_0xec9f44() -> ! {
    todo!("0xec9f44 -[TFLogManager internalDeleteLogFor:]")
}

#[doc(alias = "-[TFLogManager logCheckpoints]")]
#[doc(alias = "-[TFLogManager logCheckpoints]")]
// 0xeca020 — -[TFLogManager logCheckpoints]
// type: char __cdecl(TFLogManager *self, SEL)
pub fn stub_0xeca020() -> ! {
    todo!("0xeca020 -[TFLogManager logCheckpoints]")
}

#[doc(alias = "-[TFLogManager setLogCheckpoints:]")]
#[doc(alias = "-[TFLogManager setLogCheckpoints:]")]
// 0xeca038 — -[TFLogManager setLogCheckpoints:]
// type: void __cdecl(TFLogManager *self, SEL, char)
pub fn stub_0xeca038() -> ! {
    todo!("0xeca038 -[TFLogManager setLogCheckpoints:]")
}

#[doc(alias = "-[TFLogManager .cxx_destruct]")]
#[doc(alias = "-[TFLogManager .cxx_destruct]")]
// 0xeca050 — -[TFLogManager .cxx_destruct]
// type: void __cdecl(TFLogManager *self, SEL)
pub fn stub_0xeca050() -> ! {
    todo!("0xeca050 -[TFLogManager .cxx_destruct]")
}

#[doc(alias = "_tf_log_remote")]
#[doc(alias = "_tf_log_remote")]
// 0xeca080 — _tf_log_remote
// type: int __fastcall(int, char *__s)
pub fn stub_0xeca080() -> ! {
    todo!("0xeca080 _tf_log_remote")
}

#[doc(alias = "___tf_log_remote_block_invoke")]
#[doc(alias = "___tf_log_remote_block_invoke")]
// 0xeca1e0 — ___tf_log_remote_block_invoke
pub fn stub_0xeca1e0() -> ! {
    todo!("0xeca1e0 ___tf_log_remote_block_invoke")
}

#[doc(alias = "_tf_log_STDERR")]
#[doc(alias = "_tf_log_STDERR")]
// 0xeca234 — _tf_log_STDERR
// type: int __fastcall(int, id)
pub fn stub_0xeca234() -> ! {
    todo!("0xeca234 _tf_log_STDERR")
}

#[doc(alias = "___tf_log_STDERR_block_invoke")]
#[doc(alias = "___tf_log_STDERR_block_invoke")]
// 0xeca3bc — ___tf_log_STDERR_block_invoke
// type: void __cdecl(id)
pub fn stub_0xeca3bc() -> ! {
    todo!("0xeca3bc ___tf_log_STDERR_block_invoke")
}

#[doc(alias = "_tf_msgpack_unpack_init_msgpack_data")]
#[doc(alias = "_tf_msgpack_unpack_init_msgpack_data")]
// 0xeca538 — _tf_msgpack_unpack_init_msgpack_data
pub fn stub_0xeca538() -> ! {
    todo!("0xeca538 _tf_msgpack_unpack_init_msgpack_data")
}

#[doc(alias = "_tf_msgpack_unpack_next")]
#[doc(alias = "_tf_msgpack_unpack_next")]
// 0xeca544 — _tf_msgpack_unpack_next
pub fn stub_0xeca544() -> ! {
    todo!("0xeca544 _tf_msgpack_unpack_next")
}

#[doc(alias = "_tf_msgpack_unpack_pos_fix_num")]
#[doc(alias = "_tf_msgpack_unpack_pos_fix_num")]
// 0xeca650 — _tf_msgpack_unpack_pos_fix_num
pub fn stub_0xeca650() -> ! {
    todo!("0xeca650 _tf_msgpack_unpack_pos_fix_num")
}

#[doc(alias = "_tf_msgpack_unpack_neg_fix_num")]
#[doc(alias = "_tf_msgpack_unpack_neg_fix_num")]
// 0xeca660 — _tf_msgpack_unpack_neg_fix_num
pub fn stub_0xeca660() -> ! {
    todo!("0xeca660 _tf_msgpack_unpack_neg_fix_num")
}

#[doc(alias = "_tf_msgpack_unpack_int8")]
#[doc(alias = "_tf_msgpack_unpack_int8")]
// 0xeca670 — _tf_msgpack_unpack_int8
pub fn stub_0xeca670() -> ! {
    todo!("0xeca670 _tf_msgpack_unpack_int8")
}

#[doc(alias = "_tf_msgpack_unpack_uint8")]
#[doc(alias = "_tf_msgpack_unpack_uint8")]
// 0xeca67c — _tf_msgpack_unpack_uint8
pub fn stub_0xeca67c() -> ! {
    todo!("0xeca67c _tf_msgpack_unpack_uint8")
}

#[doc(alias = "_tf_msgpack_unpack_int16")]
#[doc(alias = "_tf_msgpack_unpack_int16")]
// 0xeca688 — _tf_msgpack_unpack_int16
pub fn stub_0xeca688() -> ! {
    todo!("0xeca688 _tf_msgpack_unpack_int16")
}

#[doc(alias = "_tf_msgpack_unpack_uint16")]
#[doc(alias = "_tf_msgpack_unpack_uint16")]
// 0xeca6ac — _tf_msgpack_unpack_uint16
pub fn stub_0xeca6ac() -> ! {
    todo!("0xeca6ac _tf_msgpack_unpack_uint16")
}

#[doc(alias = "_tf_msgpack_unpack_int32")]
#[doc(alias = "_tf_msgpack_unpack_int32")]
// 0xeca6e4 — _tf_msgpack_unpack_int32
pub fn stub_0xeca6e4() -> ! {
    todo!("0xeca6e4 _tf_msgpack_unpack_int32")
}

