//! core shard me — 100 core stubs EA-sorted asc global gap filler not yet in core (fallback filter).
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 100 not yet in rbx_core (fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound; 33260 fallback, 3244 uncovered before -> 3144 after, rbx_core::SharedPtr not boost).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "-[FlurrySession eventLog]")]
// 0xf071d4 — -[FlurrySession eventLog]
// type: NSMutableArray *__cdecl(FlurrySession *self, SEL)
pub fn stub_0xf071d4() -> ! { todo!("0xf071d4 -[FlurrySession eventLog]") }

#[doc(alias = "-[FlurrySession setEventLog:]")]
// 0xf071e4 — -[FlurrySession setEventLog:]
// type: void __cdecl(FlurrySession *self, SEL, id)
pub fn stub_0xf071e4() -> ! { todo!("0xf071e4 -[FlurrySession setEventLog:]") }

#[doc(alias = "-[FlurrySession eventLogComplete]")]
// 0xf07208 — -[FlurrySession eventLogComplete]
// type: char __cdecl(FlurrySession *self, SEL)
pub fn stub_0xf07208() -> ! { todo!("0xf07208 -[FlurrySession eventLogComplete]") }

#[doc(alias = "-[FlurrySession setEventLogComplete:]")]
// 0xf07218 — -[FlurrySession setEventLogComplete:]
// type: void __cdecl(FlurrySession *self, SEL, char)
pub fn stub_0xf07218() -> ! { todo!("0xf07218 -[FlurrySession setEventLogComplete:]") }

#[doc(alias = "-[FlurrySession errors]")]
// 0xf07228 — -[FlurrySession errors]
// type: NSMutableArray *__cdecl(FlurrySession *self, SEL)
pub fn stub_0xf07228() -> ! { todo!("0xf07228 -[FlurrySession errors]") }

#[doc(alias = "-[FlurrySession setErrors:]")]
// 0xf07238 — -[FlurrySession setErrors:]
// type: void __cdecl(FlurrySession *self, SEL, id)
pub fn stub_0xf07238() -> ! { todo!("0xf07238 -[FlurrySession setErrors:]") }

#[doc(alias = "-[FlurrySession totalErrorCount]")]
// 0xf0725c — -[FlurrySession totalErrorCount]
// type: int __cdecl(FlurrySession *self, SEL)
pub fn stub_0xf0725c() -> ! { todo!("0xf0725c -[FlurrySession totalErrorCount]") }

#[doc(alias = "-[FlurrySession setTotalErrorCount:]")]
// 0xf0726c — -[FlurrySession setTotalErrorCount:]
// type: void __cdecl(FlurrySession *self, SEL, int)
pub fn stub_0xf0726c() -> ! { todo!("0xf0726c -[FlurrySession setTotalErrorCount:]") }

#[doc(alias = "-[FlurrySession locale]")]
// 0xf0727c — -[FlurrySession locale]
// type: NSString *__cdecl(FlurrySession *self, SEL)
pub fn stub_0xf0727c() -> ! { todo!("0xf0727c -[FlurrySession locale]") }

#[doc(alias = "-[FlurrySession setLocale:]")]
// 0xf0728c — -[FlurrySession setLocale:]
// type: void __cdecl(FlurrySession *self, SEL, id)
pub fn stub_0xf0728c() -> ! { todo!("0xf0728c -[FlurrySession setLocale:]") }

#[doc(alias = "-[FlurrySession timeZone]")]
// 0xf072b0 — -[FlurrySession timeZone]
// type: NSString *__cdecl(FlurrySession *self, SEL)
pub fn stub_0xf072b0() -> ! { todo!("0xf072b0 -[FlurrySession timeZone]") }

#[doc(alias = "-[FlurrySession setTimeZone:]")]
// 0xf072c0 — -[FlurrySession setTimeZone:]
// type: void __cdecl(FlurrySession *self, SEL, id)
pub fn stub_0xf072c0() -> ! { todo!("0xf072c0 -[FlurrySession setTimeZone:]") }

#[doc(alias = "-[FlurrySession gender]")]
// 0xf072e4 — -[FlurrySession gender]
// type: int __cdecl(FlurrySession *self, SEL)
pub fn stub_0xf072e4() -> ! { todo!("0xf072e4 -[FlurrySession gender]") }

#[doc(alias = "-[FlurrySession setGender:]")]
// 0xf072f4 — -[FlurrySession setGender:]
// type: void __cdecl(FlurrySession *self, SEL, int)
pub fn stub_0xf072f4() -> ! { todo!("0xf072f4 -[FlurrySession setGender:]") }

#[doc(alias = "-[FlurrySession pushToken]")]
// 0xf07304 — -[FlurrySession pushToken]
// type: NSString *__cdecl(FlurrySession *self, SEL)
pub fn stub_0xf07304() -> ! { todo!("0xf07304 -[FlurrySession pushToken]") }

#[doc(alias = "-[FlurrySession serializationVersion]")]
// 0xf07314 — -[FlurrySession serializationVersion]
// type: int __cdecl(FlurrySession *self, SEL)
pub fn stub_0xf07314() -> ! { todo!("0xf07314 -[FlurrySession serializationVersion]") }

#[doc(alias = "-[FlurrySession setSerializationVersion:]")]
// 0xf07324 — -[FlurrySession setSerializationVersion:]
// type: void __cdecl(FlurrySession *self, SEL, int)
pub fn stub_0xf07324() -> ! { todo!("0xf07324 -[FlurrySession setSerializationVersion:]") }

#[doc(alias = "-[FlurrySession eventLoggingEnabled]")]
// 0xf07334 — -[FlurrySession eventLoggingEnabled]
// type: char __cdecl(FlurrySession *self, SEL)
pub fn stub_0xf07334() -> ! { todo!("0xf07334 -[FlurrySession eventLoggingEnabled]") }

#[doc(alias = "-[FlurrySession setAge:]")]
// 0xf07344 — -[FlurrySession setAge:]
// type: void __cdecl(FlurrySession *self, SEL, id)
pub fn stub_0xf07344() -> ! { todo!("0xf07344 -[FlurrySession setAge:]") }

#[doc(alias = "+[FlurryUtil assertThreadIsNotMain]")]
// 0xf07368 — +[FlurryUtil assertThreadIsNotMain]
// type: void __cdecl(id, SEL)
pub fn stub_0xf07368() -> ! { todo!("0xf07368 +[FlurryUtil assertThreadIsNotMain]") }

#[doc(alias = "+[FlurryUtil setLogLevel:]")]
// 0xf0738c — +[FlurryUtil setLogLevel:]
// type: void __cdecl(id, SEL, int)
pub fn stub_0xf0738c() -> ! { todo!("0xf0738c +[FlurryUtil setLogLevel:]") }

#[doc(alias = "+[FlurryUtil logLevel]")]
// 0xf0739c — +[FlurryUtil logLevel]
// type: int __cdecl(id, SEL)
pub fn stub_0xf0739c() -> ! { todo!("0xf0739c +[FlurryUtil logLevel]") }

#[doc(alias = "+[FlurryUtil setShowErrorInLogEnabled:]")]
// 0xf073ac — +[FlurryUtil setShowErrorInLogEnabled:]
// type: void __cdecl(id, SEL, char)
pub fn stub_0xf073ac() -> ! { todo!("0xf073ac +[FlurryUtil setShowErrorInLogEnabled:]") }

#[doc(alias = "+[FlurryUtil handleException:]")]
// 0xf073bc — +[FlurryUtil handleException:]
// type: void __cdecl(id, SEL, id)
pub fn stub_0xf073bc() -> ! { todo!("0xf073bc +[FlurryUtil handleException:]") }

#[doc(alias = "+[FlurryUtil safeUnarchiveObjectWithFile:]")]
// 0xf075c0 — +[FlurryUtil safeUnarchiveObjectWithFile:]
// type: id __cdecl(id, SEL, id)
pub fn stub_0xf075c0() -> ! { todo!("0xf075c0 +[FlurryUtil safeUnarchiveObjectWithFile:]") }

#[doc(alias = "+[FlurryUtil getOrientationStrFromInterfaceOrientation:]")]
// 0xf07724 — +[FlurryUtil getOrientationStrFromInterfaceOrientation:]
// type: id __cdecl(id, SEL, int)
pub fn stub_0xf07724() -> ! { todo!("0xf07724 +[FlurryUtil getOrientationStrFromInterfaceOrientation:]") }

#[doc(alias = "+[FlurryUtil screenBounds:]")]
// 0xf07768 — +[FlurryUtil screenBounds:]
// type: CGRect *__cdecl(CGRect *__return_ptr __struct_ptr retstr, id, SEL, id)
pub fn stub_0xf07768() -> ! { todo!("0xf07768 +[FlurryUtil screenBounds:]") }

#[doc(alias = "+[FlurryUtil screenBounds]")]
// 0xf07880 — +[FlurryUtil screenBounds]
// type: CGRect *__cdecl(CGRect *__return_ptr __struct_ptr retstr, id, SEL)
pub fn stub_0xf07880() -> ! { todo!("0xf07880 +[FlurryUtil screenBounds]") }

#[doc(alias = "+[FlurryUtil canvasInLandscape:]")]
// 0xf0795c — +[FlurryUtil canvasInLandscape:]
// type: char __cdecl(id, SEL, id)
pub fn stub_0xf0795c() -> ! { todo!("0xf0795c +[FlurryUtil canvasInLandscape:]") }

#[doc(alias = "+[FlurryUtil canvasInLandscapeRight:]")]
// 0xf079c4 — +[FlurryUtil canvasInLandscapeRight:]
// type: char __cdecl(id, SEL, id)
pub fn stub_0xf079c4() -> ! { todo!("0xf079c4 +[FlurryUtil canvasInLandscapeRight:]") }

#[doc(alias = "+[FlurryUtil canvasInLandscapeLeft:]")]
// 0xf07a14 — +[FlurryUtil canvasInLandscapeLeft:]
// type: char __cdecl(id, SEL, id)
pub fn stub_0xf07a14() -> ! { todo!("0xf07a14 +[FlurryUtil canvasInLandscapeLeft:]") }

#[doc(alias = "+[FlurryUtil canvasInPortraitUpsideDown:]")]
// 0xf07a64 — +[FlurryUtil canvasInPortraitUpsideDown:]
// type: char __cdecl(id, SEL, id)
pub fn stub_0xf07a64() -> ! { todo!("0xf07a64 +[FlurryUtil canvasInPortraitUpsideDown:]") }

#[doc(alias = "+[FlurryUtil getCanvasOrientation]")]
// 0xf07ab4 — +[FlurryUtil getCanvasOrientation]
// type: id __cdecl(id, SEL)
pub fn stub_0xf07ab4() -> ! { todo!("0xf07ab4 +[FlurryUtil getCanvasOrientation]") }

#[doc(alias = "+[FlurryUtil canvasInLandscape]")]
// 0xf07b40 — +[FlurryUtil canvasInLandscape]
// type: char __cdecl(id, SEL)
pub fn stub_0xf07b40() -> ! { todo!("0xf07b40 +[FlurryUtil canvasInLandscape]") }

#[doc(alias = "+[FlurryUtil canvasInLandscapeRight]")]
// 0xf07b8c — +[FlurryUtil canvasInLandscapeRight]
// type: char __cdecl(id, SEL)
pub fn stub_0xf07b8c() -> ! { todo!("0xf07b8c +[FlurryUtil canvasInLandscapeRight]") }

#[doc(alias = "+[FlurryUtil canvasInLandscapeLeft]")]
// 0xf07bc8 — +[FlurryUtil canvasInLandscapeLeft]
// type: char __cdecl(id, SEL)
pub fn stub_0xf07bc8() -> ! { todo!("0xf07bc8 +[FlurryUtil canvasInLandscapeLeft]") }

#[doc(alias = "+[FlurryUtil canvasInPortraitUpsideDown]")]
// 0xf07c04 — +[FlurryUtil canvasInPortraitUpsideDown]
// type: char __cdecl(id, SEL)
pub fn stub_0xf07c04() -> ! { todo!("0xf07c04 +[FlurryUtil canvasInPortraitUpsideDown]") }

#[doc(alias = "+[FlurryUtil currentInterfaceOrientation]")]
// 0xf07c40 — +[FlurryUtil currentInterfaceOrientation]
// type: int __cdecl(id, SEL)
pub fn stub_0xf07c40() -> ! { todo!("0xf07c40 +[FlurryUtil currentInterfaceOrientation]") }

#[doc(alias = "+[FlurryUtil removeViewFromSuperview:]")]
// 0xf07ca8 — +[FlurryUtil removeViewFromSuperview:]
// type: void __cdecl(id, SEL, id)
pub fn stub_0xf07ca8() -> ! { todo!("0xf07ca8 +[FlurryUtil removeViewFromSuperview:]") }

#[doc(alias = "+[FlurryUtil convertStrToNum:]")]
// 0xf07cc8 — +[FlurryUtil convertStrToNum:]
// type: id __cdecl(id, SEL, id)
pub fn stub_0xf07cc8() -> ! { todo!("0xf07cc8 +[FlurryUtil convertStrToNum:]") }

#[doc(alias = "+[FlurryUtil isIPad]")]
// 0xf07d3c — +[FlurryUtil isIPad]
// type: char __cdecl(id, SEL)
pub fn stub_0xf07d3c() -> ! { todo!("0xf07d3c +[FlurryUtil isIPad]") }

#[doc(alias = "+[FlurryUtil isRetina]")]
// 0xf07da0 — +[FlurryUtil isRetina]
// type: char __cdecl(id, SEL)
pub fn stub_0xf07da0() -> ! { todo!("0xf07da0 +[FlurryUtil isRetina]") }

#[doc(alias = "+[FlurryUtil addToSet:obj:]")]
// 0xf07e30 — +[FlurryUtil addToSet:obj:]
// type: char __cdecl(id, SEL, id, id)
pub fn stub_0xf07e30() -> ! { todo!("0xf07e30 +[FlurryUtil addToSet:obj:]") }

#[doc(alias = "+[FlurryUtil getSystemVersionAsAnInteger]")]
// 0xf07e74 — +[FlurryUtil getSystemVersionAsAnInteger]
// type: int __cdecl(id, SEL)
pub fn stub_0xf07e74() -> ! { todo!("0xf07e74 +[FlurryUtil getSystemVersionAsAnInteger]") }

#[doc(alias = "+[FlurryUtil addSkipBackupAttributeToItemAtURL:]")]
// 0xf0803c — +[FlurryUtil addSkipBackupAttributeToItemAtURL:]
// type: char __cdecl(id, SEL, id)
pub fn stub_0xf0803c() -> ! { todo!("0xf0803c +[FlurryUtil addSkipBackupAttributeToItemAtURL:]") }

#[doc(alias = "+[FlurryUtil decodeFromPercentEscapeString:]")]
// 0xf080b4 — +[FlurryUtil decodeFromPercentEscapeString:]
// type: id __cdecl(id, SEL, id)
pub fn stub_0xf080b4() -> ! { todo!("0xf080b4 +[FlurryUtil decodeFromPercentEscapeString:]") }

#[doc(alias = "+[FlurryUtil encodeToPercentEscapeString:]")]
// 0xf080e4 — +[FlurryUtil encodeToPercentEscapeString:]
// type: id __cdecl(id, SEL, id)
pub fn stub_0xf080e4() -> ! { todo!("0xf080e4 +[FlurryUtil encodeToPercentEscapeString:]") }

#[doc(alias = "+[FlurryUtil viewIsVisible:]")]
// 0xf0811c — +[FlurryUtil viewIsVisible:]
// type: char __cdecl(id, SEL, id)
pub fn stub_0xf0811c() -> ! { todo!("0xf0811c +[FlurryUtil viewIsVisible:]") }

#[doc(alias = "+[FlurryUtil theAppIsActive]")]
// 0xf08374 — +[FlurryUtil theAppIsActive]
// type: char __cdecl(id, SEL)
pub fn stub_0xf08374() -> ! { todo!("0xf08374 +[FlurryUtil theAppIsActive]") }

#[doc(alias = "+[FlurryUtil generateChecksumTable]")]
// 0xf08420 — +[FlurryUtil generateChecksumTable]
// type: void __cdecl(id, SEL)
pub fn stub_0xf08420() -> ! { todo!("0xf08420 +[FlurryUtil generateChecksumTable]") }

#[doc(alias = "+[FlurryUtil dataChecksum:]")]
// 0xf084c0 — +[FlurryUtil dataChecksum:]
// type: int __cdecl(id, SEL, id)
pub fn stub_0xf084c0() -> ! { todo!("0xf084c0 +[FlurryUtil dataChecksum:]") }

#[doc(alias = "+[FlurryUtil createParamKeysDictFromUrlParams:paramsHeader:paramsKVString:]")]
// 0xf0854c — +[FlurryUtil createParamKeysDictFromUrlParams:paramsHeader:paramsKVString:]
// type: id __cdecl(id, SEL, id, id *, id *)
pub fn stub_0xf0854c() -> ! { todo!("0xf0854c +[FlurryUtil createParamKeysDictFromUrlParams:paramsHeader:paramsKVString:]") }

#[doc(alias = "+[FlurryUtil flurryRangeOfData:inData:]")]
// 0xf086f8 — +[FlurryUtil flurryRangeOfData:inData:]
// type: _NSRange *__cdecl(_NSRange *__return_ptr __struct_ptr retstr, id, SEL, id, id)
pub fn stub_0xf086f8() -> ! { todo!("0xf086f8 +[FlurryUtil flurryRangeOfData:inData:]") }

#[doc(alias = "+[FlurryUtil isKeyWindowAlertView]")]
// 0xf087e4 — +[FlurryUtil isKeyWindowAlertView]
// type: char __cdecl(id, SEL)
pub fn stub_0xf087e4() -> ! { todo!("0xf087e4 +[FlurryUtil isKeyWindowAlertView]") }

#[doc(alias = "+[FlurryUtil oldFilePathDirectoryUptilAgentVersion109]")]
// 0xf08868 — +[FlurryUtil oldFilePathDirectoryUptilAgentVersion109]
// type: id __cdecl(id, SEL)
pub fn stub_0xf08868() -> ! { todo!("0xf08868 +[FlurryUtil oldFilePathDirectoryUptilAgentVersion109]") }

#[doc(alias = "+[FlurryUtil filePathDirectory]")]
// 0xf08978 — +[FlurryUtil filePathDirectory]
// type: id __cdecl(id, SEL)
pub fn stub_0xf08978() -> ! { todo!("0xf08978 +[FlurryUtil filePathDirectory]") }

#[doc(alias = "+[FlurryUtil devicePlatform]")]
// 0xf08ad8 — +[FlurryUtil devicePlatform]
// type: id __cdecl(id, SEL)
pub fn stub_0xf08ad8() -> ! { todo!("0xf08ad8 +[FlurryUtil devicePlatform]") }

#[doc(alias = "+[FlurryUtil deviceMemory]")]
// 0xf08b48 — +[FlurryUtil deviceMemory]
// type: int __cdecl(id, SEL)
pub fn stub_0xf08b48() -> ! { todo!("0xf08b48 +[FlurryUtil deviceMemory]") }

#[doc(alias = "+[FlurryUtil deviceCPUFrequency]")]
// 0xf08b78 — +[FlurryUtil deviceCPUFrequency]
// type: int __cdecl(id, SEL)
pub fn stub_0xf08b78() -> ! { todo!("0xf08b78 +[FlurryUtil deviceCPUFrequency]") }

#[doc(alias = "+[FlurryUtil deviceIsJailbroken]")]
// 0xf08ba0 — +[FlurryUtil deviceIsJailbroken]
// type: char __cdecl(id, SEL)
pub fn stub_0xf08ba0() -> ! { todo!("0xf08ba0 +[FlurryUtil deviceIsJailbroken]") }

#[doc(alias = "+[FlurryUtil appIsCracked]")]
// 0xf08ba8 — +[FlurryUtil appIsCracked]
// type: char __cdecl(id, SEL)
pub fn stub_0xf08ba8() -> ! { todo!("0xf08ba8 +[FlurryUtil appIsCracked]") }

#[doc(alias = "+[FlurryUtil iTunesMetadataPlist]")]
// 0xf08c80 — +[FlurryUtil iTunesMetadataPlist]
// type: id __cdecl(id, SEL)
pub fn stub_0xf08c80() -> ! { todo!("0xf08c80 +[FlurryUtil iTunesMetadataPlist]") }

#[doc(alias = "+[FlurryUtil hashData:]")]
// 0xf08f24 — +[FlurryUtil hashData:]
// type: id __cdecl(id, SEL, id)
pub fn stub_0xf08f24() -> ! { todo!("0xf08f24 +[FlurryUtil hashData:]") }

#[doc(alias = "+[FlurryUtil hashDataToHexString:length:]")]
// 0xf08fa0 — +[FlurryUtil hashDataToHexString:length:]
// type: id __cdecl(id, SEL, char *, int)
pub fn stub_0xf08fa0() -> ! { todo!("0xf08fa0 +[FlurryUtil hashDataToHexString:length:]") }

#[doc(alias = "_flurryGetMacAddress")]
// 0xf09020 — _flurryGetMacAddress
pub fn stub_0xf09020() -> ! { todo!("0xf09020 _flurryGetMacAddress") }

#[doc(alias = "+[FlurryUtil MACString]")]
// 0xf090dc — +[FlurryUtil MACString]
// type: id __cdecl(id, SEL)
pub fn stub_0xf090dc() -> ! { todo!("0xf090dc +[FlurryUtil MACString]") }

#[doc(alias = "+[FlurryUtil getMACBasedUID]")]
// 0xf091ac — +[FlurryUtil getMACBasedUID]
// type: id __cdecl(id, SEL)
pub fn stub_0xf091ac() -> ! { todo!("0xf091ac +[FlurryUtil getMACBasedUID]") }

#[doc(alias = "+[FlurryUtil getMACUID]")]
// 0xf09288 — +[FlurryUtil getMACUID]
// type: id __cdecl(id, SEL)
pub fn stub_0xf09288() -> ! { todo!("0xf09288 +[FlurryUtil getMACUID]") }

#[doc(alias = "+[FlurryUtil getIdentifierForAdvertiser]")]
// 0xf09344 — +[FlurryUtil getIdentifierForAdvertiser]
// type: id __cdecl(id, SEL)
pub fn stub_0xf09344() -> ! { todo!("0xf09344 +[FlurryUtil getIdentifierForAdvertiser]") }

#[doc(alias = "+[FlurryUtil getIdentifierForVendor]")]
// 0xf093a4 — +[FlurryUtil getIdentifierForVendor]
// type: id __cdecl(id, SEL)
pub fn stub_0xf093a4() -> ! { todo!("0xf093a4 +[FlurryUtil getIdentifierForVendor]") }

#[doc(alias = "+[FlurryUtil getStoredIdentifierForVendor]")]
// 0xf09420 — +[FlurryUtil getStoredIdentifierForVendor]
// type: id __cdecl(id, SEL)
pub fn stub_0xf09420() -> ! { todo!("0xf09420 +[FlurryUtil getStoredIdentifierForVendor]") }

#[doc(alias = "+[FlurryUtil storeIdentifierForVendor:]")]
// 0xf094d8 — +[FlurryUtil storeIdentifierForVendor:]
// type: void __cdecl(id, SEL, id)
pub fn stub_0xf094d8() -> ! { todo!("0xf094d8 +[FlurryUtil storeIdentifierForVendor:]") }

#[doc(alias = "+[FlurryUtil isAdTrackingEnabled]")]
// 0xf09524 — +[FlurryUtil isAdTrackingEnabled]
// type: char __cdecl(id, SEL)
pub fn stub_0xf09524() -> ! { todo!("0xf09524 +[FlurryUtil isAdTrackingEnabled]") }

#[doc(alias = "+[FlurryUtil generateCFUUIDBasedUIDChecksum:]")]
// 0xf09570 — +[FlurryUtil generateCFUUIDBasedUIDChecksum:]
// type: unsigned int __cdecl(id, SEL, id)
pub fn stub_0xf09570() -> ! { todo!("0xf09570 +[FlurryUtil generateCFUUIDBasedUIDChecksum:]") }

#[doc(alias = "+[FlurryUtil generateCFUUID]")]
// 0xf09760 — +[FlurryUtil generateCFUUID]
// type: id __cdecl(id, SEL)
pub fn stub_0xf09760() -> ! { todo!("0xf09760 +[FlurryUtil generateCFUUID]") }

#[doc(alias = "+[FlurryUtil getCFUUIDBasedUIDAndStatus:]")]
// 0xf097a8 — +[FlurryUtil getCFUUIDBasedUIDAndStatus:]
// type: id __cdecl(id, SEL, int *)
pub fn stub_0xf097a8() -> ! { todo!("0xf097a8 +[FlurryUtil getCFUUIDBasedUIDAndStatus:]") }

#[doc(alias = "+[FlurryUtil generateCFUUIDBasedUID]")]
// 0xf09ac4 — +[FlurryUtil generateCFUUIDBasedUID]
// type: id __cdecl(id, SEL)
pub fn stub_0xf09ac4() -> ! { todo!("0xf09ac4 +[FlurryUtil generateCFUUIDBasedUID]") }

#[doc(alias = "+[FlurryUtil getCFUUIDBasedUIDWithChecksum:]")]
// 0xf09b14 — +[FlurryUtil getCFUUIDBasedUIDWithChecksum:]
// type: id __cdecl(id, SEL, id)
pub fn stub_0xf09b14() -> ! { todo!("0xf09b14 +[FlurryUtil getCFUUIDBasedUIDWithChecksum:]") }

#[doc(alias = "+[FlurryUtil preferredLanguage]")]
// 0xf09bbc — +[FlurryUtil preferredLanguage]
// type: id __cdecl(id, SEL)
pub fn stub_0xf09bbc() -> ! { todo!("0xf09bbc +[FlurryUtil preferredLanguage]") }

#[doc(alias = "+[FlurryUtil incrementCounter:locationName:]")]
// 0xf09ca8 — +[FlurryUtil incrementCounter:locationName:]
// type: void __cdecl(id, SEL, id, char *)
pub fn stub_0xf09ca8() -> ! { todo!("0xf09ca8 +[FlurryUtil incrementCounter:locationName:]") }

#[doc(alias = "+[FlurryUtil incrementCounter:]")]
// 0xf09d00 — +[FlurryUtil incrementCounter:]
// type: void __cdecl(id, SEL, id)
pub fn stub_0xf09d00() -> ! { todo!("0xf09d00 +[FlurryUtil incrementCounter:]") }

#[doc(alias = "+[FlurryUtil decrementCounter:locationName:]")]
// 0xf09e94 — +[FlurryUtil decrementCounter:locationName:]
// type: void __cdecl(id, SEL, id, char *)
pub fn stub_0xf09e94() -> ! { todo!("0xf09e94 +[FlurryUtil decrementCounter:locationName:]") }

#[doc(alias = "+[FlurryUtil decrementCounter:]")]
// 0xf09eec — +[FlurryUtil decrementCounter:]
// type: void __cdecl(id, SEL, id)
pub fn stub_0xf09eec() -> ! { todo!("0xf09eec +[FlurryUtil decrementCounter:]") }

#[doc(alias = "+[FlurryUtil printCounters]")]
// 0xf0a080 — +[FlurryUtil printCounters]
// type: void __cdecl(id, SEL)
pub fn stub_0xf0a080() -> ! { todo!("0xf0a080 +[FlurryUtil printCounters]") }

#[doc(alias = "+[FlurryUtil allCounters]")]
// 0xf0a204 — +[FlurryUtil allCounters]
// type: id __cdecl(id, SEL)
pub fn stub_0xf0a204() -> ! { todo!("0xf0a204 +[FlurryUtil allCounters]") }

#[doc(alias = "-[FlurryConnectionDelegate init]")]
// 0xf0a2f0 — -[FlurryConnectionDelegate init]
// type: FlurryConnectionDelegate *__cdecl(FlurryConnectionDelegate *self, SEL)
pub fn stub_0xf0a2f0() -> ! { todo!("0xf0a2f0 -[FlurryConnectionDelegate init]") }

#[doc(alias = "-[FlurryConnectionDelegate initWithTaskDelegate:taskParent:]")]
// 0xf0a364 — -[FlurryConnectionDelegate initWithTaskDelegate:taskParent:]
// type: FlurryConnectionDelegate *__cdecl(FlurryConnectionDelegate *self, SEL, id, id)
pub fn stub_0xf0a364() -> ! { todo!("0xf0a364 -[FlurryConnectionDelegate initWithTaskDelegate:taskParent:]") }

#[doc(alias = "-[FlurryConnectionDelegate dealloc]")]
// 0xf0a40c — -[FlurryConnectionDelegate dealloc]
// type: void __cdecl(FlurryConnectionDelegate *self, SEL)
pub fn stub_0xf0a40c() -> ! { todo!("0xf0a40c -[FlurryConnectionDelegate dealloc]") }

#[doc(alias = "-[FlurryConnectionDelegate isDone]")]
// 0xf0a4ac — -[FlurryConnectionDelegate isDone]
// type: char __cdecl(FlurryConnectionDelegate *self, SEL)
pub fn stub_0xf0a4ac() -> ! { todo!("0xf0a4ac -[FlurryConnectionDelegate isDone]") }

#[doc(alias = "-[FlurryConnectionDelegate connection:willSendRequest:redirectResponse:]")]
// 0xf0a4bc — -[FlurryConnectionDelegate connection:willSendRequest:redirectResponse:]
// type: id __cdecl(FlurryConnectionDelegate *self, SEL, id, id, id)
pub fn stub_0xf0a4bc() -> ! { todo!("0xf0a4bc -[FlurryConnectionDelegate connection:willSendRequest:redirectResponse:]") }

#[doc(alias = "-[FlurryConnectionDelegate connection:didReceiveAuthenticationChallenge:]")]
// 0xf0a4c0 — -[FlurryConnectionDelegate connection:didReceiveAuthenticationChallenge:]
// type: void __cdecl(FlurryConnectionDelegate *self, SEL, id, id)
pub fn stub_0xf0a4c0() -> ! { todo!("0xf0a4c0 -[FlurryConnectionDelegate connection:didReceiveAuthenticationChallenge:]") }

#[doc(alias = "-[FlurryConnectionDelegate connection:didCancelAuthenticationChallenge:]")]
// 0xf0a504 — -[FlurryConnectionDelegate connection:didCancelAuthenticationChallenge:]
// type: void __cdecl(FlurryConnectionDelegate *self, SEL, id, id)
pub fn stub_0xf0a504() -> ! { todo!("0xf0a504 -[FlurryConnectionDelegate connection:didCancelAuthenticationChallenge:]") }

#[doc(alias = "-[FlurryConnectionDelegate connection:didReceiveResponse:]")]
// 0xf0a5b8 — -[FlurryConnectionDelegate connection:didReceiveResponse:]
// type: void __cdecl(FlurryConnectionDelegate *self, SEL, id, id)
pub fn stub_0xf0a5b8() -> ! { todo!("0xf0a5b8 -[FlurryConnectionDelegate connection:didReceiveResponse:]") }

#[doc(alias = "-[FlurryConnectionDelegate connection:didReceiveData:]")]
// 0xf0a614 — -[FlurryConnectionDelegate connection:didReceiveData:]
// type: void __cdecl(FlurryConnectionDelegate *self, SEL, id, id)
pub fn stub_0xf0a614() -> ! { todo!("0xf0a614 -[FlurryConnectionDelegate connection:didReceiveData:]") }

#[doc(alias = "-[FlurryConnectionDelegate connectionDidFinishLoading:]")]
// 0xf0a63c — -[FlurryConnectionDelegate connectionDidFinishLoading:]
// type: void __cdecl(FlurryConnectionDelegate *self, SEL, id)
pub fn stub_0xf0a63c() -> ! { todo!("0xf0a63c -[FlurryConnectionDelegate connectionDidFinishLoading:]") }

#[doc(alias = "-[FlurryConnectionDelegate connection:didFailWithError:]")]
// 0xf0a798 — -[FlurryConnectionDelegate connection:didFailWithError:]
// type: void __cdecl(FlurryConnectionDelegate *self, SEL, id, id)
pub fn stub_0xf0a798() -> ! { todo!("0xf0a798 -[FlurryConnectionDelegate connection:didFailWithError:]") }

#[doc(alias = "-[FlurryConnectionDelegate connection:willCacheResponse:]")]
// 0xf0a86c — -[FlurryConnectionDelegate connection:willCacheResponse:]
// type: id __cdecl(FlurryConnectionDelegate *self, SEL, id, id)
pub fn stub_0xf0a86c() -> ! { todo!("0xf0a86c -[FlurryConnectionDelegate connection:willCacheResponse:]") }

#[doc(alias = "-[FlurryConnectionDelegate connectionCanceled]")]
// 0xf0a870 — -[FlurryConnectionDelegate connectionCanceled]
// type: void __cdecl(FlurryConnectionDelegate *self, SEL)
pub fn stub_0xf0a870() -> ! { todo!("0xf0a870 -[FlurryConnectionDelegate connectionCanceled]") }

#[doc(alias = "-[FlurryConnectionDelegate constructResponse]")]
// 0xf0a904 — -[FlurryConnectionDelegate constructResponse]
// type: id __cdecl(FlurryConnectionDelegate *self, SEL)
pub fn stub_0xf0a904() -> ! { todo!("0xf0a904 -[FlurryConnectionDelegate constructResponse]") }

#[doc(alias = "-[FlurryConnectionDelegate response]")]
// 0xf0a998 — -[FlurryConnectionDelegate response]
// type: NSHTTPURLResponse *__cdecl(FlurryConnectionDelegate *self, SEL)
pub fn stub_0xf0a998() -> ! { todo!("0xf0a998 -[FlurryConnectionDelegate response]") }
