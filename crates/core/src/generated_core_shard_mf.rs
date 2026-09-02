//! core shard mf — 100 core stubs EA-sorted asc global gap filler not yet in core (fallback filter).
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 100 not yet in rbx_core (fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound; 33887 fallback, 3173 uncovered before -> 3073 after, rbx_core::SharedPtr not boost).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "-[FlurryConnectionDelegate body]")]
// 0xf0a9a8 — -[FlurryConnectionDelegate body]
// type: NSMutableData *__cdecl(FlurryConnectionDelegate *self, SEL)
pub fn stub_0xf0a9a8() -> ! { todo!("0xf0a9a8 -[FlurryConnectionDelegate body]") }

#[doc(alias = "-[FlurryConnectionDelegate error]")]
// 0xf0a9b8 — -[FlurryConnectionDelegate error]
// type: NSError *__cdecl(FlurryConnectionDelegate *self, SEL)
pub fn stub_0xf0a9b8() -> ! { todo!("0xf0a9b8 -[FlurryConnectionDelegate error]") }

#[doc(alias = "-[FlurryConnectionDelegate httpTaskDelegate]")]
// 0xf0a9c8 — -[FlurryConnectionDelegate httpTaskDelegate]
// type: FlurryHttpAsyncTaskDelegate *__cdecl(FlurryConnectionDelegate *self, SEL)
pub fn stub_0xf0a9c8() -> ! { todo!("0xf0a9c8 -[FlurryConnectionDelegate httpTaskDelegate]") }

#[doc(alias = "-[FlurryConnectionDelegate setHttpTaskDelegate:]")]
// 0xf0a9d8 — -[FlurryConnectionDelegate setHttpTaskDelegate:]
// type: void __cdecl(FlurryConnectionDelegate *self, SEL, id)
pub fn stub_0xf0a9d8() -> ! { todo!("0xf0a9d8 -[FlurryConnectionDelegate setHttpTaskDelegate:]") }

#[doc(alias = "-[FlurryConnectionDelegate httpTask]")]
// 0xf0a9fc — -[FlurryConnectionDelegate httpTask]
// type: FlurryHttpAsyncTask *__cdecl(FlurryConnectionDelegate *self, SEL)
pub fn stub_0xf0a9fc() -> ! { todo!("0xf0a9fc -[FlurryConnectionDelegate httpTask]") }

#[doc(alias = "-[FlurryConnectionDelegate setHttpTask:]")]
// 0xf0aa0c — -[FlurryConnectionDelegate setHttpTask:]
// type: void __cdecl(FlurryConnectionDelegate *self, SEL, id)
pub fn stub_0xf0aa0c() -> ! { todo!("0xf0aa0c -[FlurryConnectionDelegate setHttpTask:]") }

#[doc(alias = "-[FlurryHTTPResponse initWithResponse:body:error:]")]
// 0xf0aa30 — -[FlurryHTTPResponse initWithResponse:body:error:]
// type: FlurryHTTPResponse *__cdecl(FlurryHTTPResponse *self, SEL, id, id, id)
pub fn stub_0xf0aa30() -> ! { todo!("0xf0aa30 -[FlurryHTTPResponse initWithResponse:body:error:]") }

#[doc(alias = "-[FlurryHTTPResponse initWithBody:statusCode:headers:error:]")]
// 0xf0ac38 — -[FlurryHTTPResponse initWithBody:statusCode:headers:error:]
// type: FlurryHTTPResponse *__cdecl(FlurryHTTPResponse *self, SEL, id, int, id, id)
pub fn stub_0xf0ac38() -> ! { todo!("0xf0ac38 -[FlurryHTTPResponse initWithBody:statusCode:headers:error:]") }

#[doc(alias = "-[FlurryHTTPResponse dealloc]")]
// 0xf0ae04 — -[FlurryHTTPResponse dealloc]
// type: void __cdecl(FlurryHTTPResponse *self, SEL)
pub fn stub_0xf0ae04() -> ! { todo!("0xf0ae04 -[FlurryHTTPResponse dealloc]") }

#[doc(alias = "-[FlurryHTTPResponse isSuccess]")]
// 0xf0ae7c — -[FlurryHTTPResponse isSuccess]
// type: char __cdecl(FlurryHTTPResponse *self, SEL)
pub fn stub_0xf0ae7c() -> ! { todo!("0xf0ae7c -[FlurryHTTPResponse isSuccess]") }

#[doc(alias = "-[FlurryHTTPResponse isError]")]
// 0xf0ae9c — -[FlurryHTTPResponse isError]
// type: char __cdecl(FlurryHTTPResponse *self, SEL)
pub fn stub_0xf0ae9c() -> ! { todo!("0xf0ae9c -[FlurryHTTPResponse isError]") }

#[doc(alias = "-[FlurryHTTPResponse description]")]
// 0xf0aec0 — -[FlurryHTTPResponse description]
// type: id __cdecl(FlurryHTTPResponse *self, SEL)
pub fn stub_0xf0aec0() -> ! { todo!("0xf0aec0 -[FlurryHTTPResponse description]") }

#[doc(alias = "-[FlurryHTTPResponse saveToDisk]")]
// 0xf0af7c — -[FlurryHTTPResponse saveToDisk]
// type: void __cdecl(FlurryHTTPResponse *self, SEL)
pub fn stub_0xf0af7c() -> ! { todo!("0xf0af7c -[FlurryHTTPResponse saveToDisk]") }

#[doc(alias = "+[FlurryHTTPResponse readFromURL:]")]
// 0xf0b100 — +[FlurryHTTPResponse readFromURL:]
// type: id __cdecl(id, SEL, id)
pub fn stub_0xf0b100() -> ! { todo!("0xf0b100 +[FlurryHTTPResponse readFromURL:]") }

#[doc(alias = "+[FlurryHTTPResponse filePath:]")]
// 0xf0b210 — +[FlurryHTTPResponse filePath:]
// type: id __cdecl(id, SEL, int)
pub fn stub_0xf0b210() -> ! { todo!("0xf0b210 +[FlurryHTTPResponse filePath:]") }

#[doc(alias = "-[FlurryHTTPResponse body]")]
// 0xf0b374 — -[FlurryHTTPResponse body]
// type: NSData *__cdecl(FlurryHTTPResponse *self, SEL)
pub fn stub_0xf0b374() -> ! { todo!("0xf0b374 -[FlurryHTTPResponse body]") }

#[doc(alias = "-[FlurryHTTPResponse statusCode]")]
// 0xf0b384 — -[FlurryHTTPResponse statusCode]
// type: int __cdecl(FlurryHTTPResponse *self, SEL)
pub fn stub_0xf0b384() -> ! { todo!("0xf0b384 -[FlurryHTTPResponse statusCode]") }

#[doc(alias = "-[FlurryHTTPResponse error]")]
// 0xf0b394 — -[FlurryHTTPResponse error]
// type: NSError *__cdecl(FlurryHTTPResponse *self, SEL)
pub fn stub_0xf0b394() -> ! { todo!("0xf0b394 -[FlurryHTTPResponse error]") }

#[doc(alias = "-[FlurryHTTPResponse headers]")]
// 0xf0b3a4 — -[FlurryHTTPResponse headers]
// type: NSMutableDictionary *__cdecl(FlurryHTTPResponse *self, SEL)
pub fn stub_0xf0b3a4() -> ! { todo!("0xf0b3a4 -[FlurryHTTPResponse headers]") }

#[doc(alias = "+[FlurryAutoIncrement instance]")]
// 0xf0b3b4 — +[FlurryAutoIncrement instance]
// type: id __cdecl(id, SEL)
pub fn stub_0xf0b3b4() -> ! { todo!("0xf0b3b4 +[FlurryAutoIncrement instance]") }

#[doc(alias = "-[FlurryAutoIncrement init]")]
// 0xf0b3fc — -[FlurryAutoIncrement init]
// type: FlurryAutoIncrement *__cdecl(FlurryAutoIncrement *self, SEL)
pub fn stub_0xf0b3fc() -> ! { todo!("0xf0b3fc -[FlurryAutoIncrement init]") }

#[doc(alias = "-[FlurryAutoIncrement nextCountFor:]")]
// 0xf0b47c — -[FlurryAutoIncrement nextCountFor:]
// type: int __cdecl(FlurryAutoIncrement *self, SEL, int)
pub fn stub_0xf0b47c() -> ! { todo!("0xf0b47c -[FlurryAutoIncrement nextCountFor:]") }

#[doc(alias = "-[FlurryAutoIncrement resetAll]")]
// 0xf0b71c — -[FlurryAutoIncrement resetAll]
// type: void __cdecl(FlurryAutoIncrement *self, SEL)
pub fn stub_0xf0b71c() -> ! { todo!("0xf0b71c -[FlurryAutoIncrement resetAll]") }

#[doc(alias = "-[FlurryAutoIncrement dealloc]")]
// 0xf0b844 — -[FlurryAutoIncrement dealloc]
// type: void __cdecl(FlurryAutoIncrement *self, SEL)
pub fn stub_0xf0b844() -> ! { todo!("0xf0b844 -[FlurryAutoIncrement dealloc]") }

#[doc(alias = "-[FlurryAutoIncrement counters]")]
// 0xf0b888 — -[FlurryAutoIncrement counters]
// type: NSMutableDictionary *__cdecl(FlurryAutoIncrement *self, SEL)
pub fn stub_0xf0b888() -> ! { todo!("0xf0b888 -[FlurryAutoIncrement counters]") }

#[doc(alias = "-[FlurryAutoIncrement setCounters:]")]
// 0xf0b898 — -[FlurryAutoIncrement setCounters:]
// type: void __cdecl(FlurryAutoIncrement *self, SEL, id)
pub fn stub_0xf0b898() -> ! { todo!("0xf0b898 -[FlurryAutoIncrement setCounters:]") }

#[doc(alias = "-[FlurryHttpAsyncTask dealloc]")]
// 0xf0c42c — -[FlurryHttpAsyncTask dealloc]
// type: void __cdecl(FlurryHttpAsyncTask *self, SEL)
pub fn stub_0xf0c42c() -> ! { todo!("0xf0c42c -[FlurryHttpAsyncTask dealloc]") }

#[doc(alias = "-[FlurryHttpAsyncTask init]")]
// 0xf0c4dc — -[FlurryHttpAsyncTask init]
// type: FlurryHttpAsyncTask *__cdecl(FlurryHttpAsyncTask *self, SEL)
pub fn stub_0xf0c4dc() -> ! { todo!("0xf0c4dc -[FlurryHttpAsyncTask init]") }

#[doc(alias = "+[FlurryHttpAsyncTask get:delegate:]")]
// 0xf0c5b0 — +[FlurryHttpAsyncTask get:delegate:]
// type: id __cdecl(id, SEL, id, id)
pub fn stub_0xf0c5b0() -> ! { todo!("0xf0c5b0 +[FlurryHttpAsyncTask get:delegate:]") }

#[doc(alias = "+[FlurryHttpAsyncTask get:headerFields:delegate:]")]
// 0xf0c5d0 — +[FlurryHttpAsyncTask get:headerFields:delegate:]
// type: id __cdecl(id, SEL, id, id, id)
pub fn stub_0xf0c5d0() -> ! { todo!("0xf0c5d0 +[FlurryHttpAsyncTask get:headerFields:delegate:]") }

#[doc(alias = "+[FlurryHttpAsyncTask post:body:delegate:]")]
// 0xf0c668 — +[FlurryHttpAsyncTask post:body:delegate:]
// type: id __cdecl(id, SEL, id, id, id)
pub fn stub_0xf0c668() -> ! { todo!("0xf0c668 +[FlurryHttpAsyncTask post:body:delegate:]") }

#[doc(alias = "+[FlurryHttpAsyncTask post:body:headerFields:delegate:]")]
// 0xf0c68c — +[FlurryHttpAsyncTask post:body:headerFields:delegate:]
// type: id __cdecl(id, SEL, id, id, id, id)
pub fn stub_0xf0c68c() -> ! { todo!("0xf0c68c +[FlurryHttpAsyncTask post:body:headerFields:delegate:]") }

#[doc(alias = "+[FlurryHttpAsyncTask sendMethod:to:body:headerFields:delegate:]")]
// 0xf0c728 — +[FlurryHttpAsyncTask sendMethod:to:body:headerFields:delegate:]
// type: id __cdecl(id, SEL, id, id, id, id, id)
pub fn stub_0xf0c728() -> ! { todo!("0xf0c728 +[FlurryHttpAsyncTask sendMethod:to:body:headerFields:delegate:]") }

#[doc(alias = "+[FlurryHttpAsyncTask sendMethod:to:body:headerFields:delegate:startImmediately:]")]
// 0xf0c760 — +[FlurryHttpAsyncTask sendMethod:to:body:headerFields:delegate:startImmediately:]
// type: id __cdecl(id, SEL, id, id, id, id, id, char)
pub fn stub_0xf0c760() -> ! { todo!("0xf0c760 +[FlurryHttpAsyncTask sendMethod:to:body:headerFields:delegate:startImmediately:]") }

#[doc(alias = "+[FlurryHttpAsyncTask sendMethod:to:body:headerFields:timeoutInterval:useCachePolicy:delegate:startImmediately:]")]
// 0xf0c7a8 — +[FlurryHttpAsyncTask sendMethod:to:body:headerFields:timeoutInterval:useCachePolicy:delegate:startImmediately:]
// type: id __cdecl(id, SEL, id, id, id, id, int, id, id, char)
pub fn stub_0xf0c7a8() -> ! { todo!("0xf0c7a8 +[FlurryHttpAsyncTask sendMethod:to:body:headerFields:timeoutInterval:useCachePolicy:delegate:startImmediately:]") }

#[doc(alias = "-[FlurryHttpAsyncTask startInRunLoop:]")]
// 0xf0c9bc — -[FlurryHttpAsyncTask startInRunLoop:]
// type: void __cdecl(FlurryHttpAsyncTask *self, SEL, id)
pub fn stub_0xf0c9bc() -> ! { todo!("0xf0c9bc -[FlurryHttpAsyncTask startInRunLoop:]") }

#[doc(alias = "-[FlurryHttpAsyncTask cancelTask]")]
// 0xf0ca14 — -[FlurryHttpAsyncTask cancelTask]
// type: void __cdecl(FlurryHttpAsyncTask *self, SEL)
pub fn stub_0xf0ca14() -> ! { todo!("0xf0ca14 -[FlurryHttpAsyncTask cancelTask]") }

#[doc(alias = "+[FlurryHttpAsyncTask queryDictionaryToString:]")]
// 0xf0ca3c — +[FlurryHttpAsyncTask queryDictionaryToString:]
// type: id __cdecl(id, SEL, id)
pub fn stub_0xf0ca3c() -> ! { todo!("0xf0ca3c +[FlurryHttpAsyncTask queryDictionaryToString:]") }

#[doc(alias = "+[FlurryHttpAsyncTask stringToQueryDictionary:]")]
// 0xf0cba0 — +[FlurryHttpAsyncTask stringToQueryDictionary:]
// type: id __cdecl(id, SEL, id)
pub fn stub_0xf0cba0() -> ! { todo!("0xf0cba0 +[FlurryHttpAsyncTask stringToQueryDictionary:]") }

#[doc(alias = "+[FlurryHttpAsyncTask urlWithProtocol:host:port:path:query:]")]
// 0xf0cd20 — +[FlurryHttpAsyncTask urlWithProtocol:host:port:path:query:]
// type: id __cdecl(id, SEL, id, id, int, id, id)
pub fn stub_0xf0cd20() -> ! { todo!("0xf0cd20 +[FlurryHttpAsyncTask urlWithProtocol:host:port:path:query:]") }

#[doc(alias = "-[FlurryHttpAsyncTask connection]")]
// 0xf0ce8c — -[FlurryHttpAsyncTask connection]
// type: NSURLConnection *__cdecl(FlurryHttpAsyncTask *self, SEL)
pub fn stub_0xf0ce8c() -> ! { todo!("0xf0ce8c -[FlurryHttpAsyncTask connection]") }

#[doc(alias = "-[FlurryHttpAsyncTask setConnection:]")]
// 0xf0ce9c — -[FlurryHttpAsyncTask setConnection:]
// type: void __cdecl(FlurryHttpAsyncTask *self, SEL, id)
pub fn stub_0xf0ce9c() -> ! { todo!("0xf0ce9c -[FlurryHttpAsyncTask setConnection:]") }

#[doc(alias = "-[FlurryHttpAsyncTask tag]")]
// 0xf0cec0 — -[FlurryHttpAsyncTask tag]
// type: NSString *__cdecl(FlurryHttpAsyncTask *self, SEL)
pub fn stub_0xf0cec0() -> ! { todo!("0xf0cec0 -[FlurryHttpAsyncTask tag]") }

#[doc(alias = "-[FlurryHttpAsyncTask setTag:]")]
// 0xf0ced0 — -[FlurryHttpAsyncTask setTag:]
// type: void __cdecl(FlurryHttpAsyncTask *self, SEL, id)
pub fn stub_0xf0ced0() -> ! { todo!("0xf0ced0 -[FlurryHttpAsyncTask setTag:]") }

#[doc(alias = "-[FlurryHttpAsyncTask taskParams]")]
// 0xf0cef4 — -[FlurryHttpAsyncTask taskParams]
// type: NSMutableDictionary *__cdecl(FlurryHttpAsyncTask *self, SEL)
pub fn stub_0xf0cef4() -> ! { todo!("0xf0cef4 -[FlurryHttpAsyncTask taskParams]") }

#[doc(alias = "-[FlurryHttpAsyncTask setTaskParams:]")]
// 0xf0cf04 — -[FlurryHttpAsyncTask setTaskParams:]
// type: void __cdecl(FlurryHttpAsyncTask *self, SEL, id)
pub fn stub_0xf0cf04() -> ! { todo!("0xf0cf04 -[FlurryHttpAsyncTask setTaskParams:]") }

#[doc(alias = "-[FlurryHttpAsyncTask backgroundTask]")]
// 0xf0cf28 — -[FlurryHttpAsyncTask backgroundTask]
// type: unsigned int __cdecl(FlurryHttpAsyncTask *self, SEL)
pub fn stub_0xf0cf28() -> ! { todo!("0xf0cf28 -[FlurryHttpAsyncTask backgroundTask]") }

#[doc(alias = "-[FlurryHttpAsyncTask setBackgroundTask:]")]
// 0xf0cf38 — -[FlurryHttpAsyncTask setBackgroundTask:]
// type: void __cdecl(FlurryHttpAsyncTask *self, SEL, unsigned int)
pub fn stub_0xf0cf38() -> ! { todo!("0xf0cf38 -[FlurryHttpAsyncTask setBackgroundTask:]") }

#[doc(alias = "+[FlurryDataSender instance]")]
// 0xf0cf48 — +[FlurryDataSender instance]
// type: id __cdecl(id, SEL)
pub fn stub_0xf0cf48() -> ! { todo!("0xf0cf48 +[FlurryDataSender instance]") }

#[doc(alias = "+[FlurryDataSender httpAsyncTaskClass]")]
// 0xf0d020 — +[FlurryDataSender httpAsyncTaskClass]
// type: Class __cdecl(id, SEL)
pub fn stub_0xf0d020() -> ! { todo!("0xf0d020 +[FlurryDataSender httpAsyncTaskClass]") }

#[doc(alias = "-[FlurryDataSender init]")]
// 0xf0d044 — -[FlurryDataSender init]
// type: FlurryDataSender *__cdecl(FlurryDataSender *self, SEL)
pub fn stub_0xf0d044() -> ! { todo!("0xf0d044 -[FlurryDataSender init]") }

#[doc(alias = "-[FlurryDataSender dealloc]")]
// 0xf0d124 — -[FlurryDataSender dealloc]
// type: void __cdecl(FlurryDataSender *self, SEL)
pub fn stub_0xf0d124() -> ! { todo!("0xf0d124 -[FlurryDataSender dealloc]") }

#[doc(alias = "+[FlurryDataSender dataKey]")]
// 0xf0d170 — +[FlurryDataSender dataKey]
// type: id __cdecl(id, SEL)
pub fn stub_0xf0d170() -> ! { todo!("0xf0d170 +[FlurryDataSender dataKey]") }

#[doc(alias = "-[FlurryDataSender setup]")]
// 0xf0d1e4 — -[FlurryDataSender setup]
// type: void __cdecl(FlurryDataSender *self, SEL)
pub fn stub_0xf0d1e4() -> ! { todo!("0xf0d1e4 -[FlurryDataSender setup]") }

#[doc(alias = "-[FlurryDataSender storeData:]")]
// 0xf0d274 — -[FlurryDataSender storeData:]
// type: id __cdecl(FlurryDataSender *self, SEL, id)
pub fn stub_0xf0d274() -> ! { todo!("0xf0d274 -[FlurryDataSender storeData:]") }

#[doc(alias = "-[FlurryDataSender removeStoredDataWithIdentifier:]")]
// 0xf0d330 — -[FlurryDataSender removeStoredDataWithIdentifier:]
// type: void __cdecl(FlurryDataSender *self, SEL, id)
pub fn stub_0xf0d330() -> ! { todo!("0xf0d330 -[FlurryDataSender removeStoredDataWithIdentifier:]") }

#[doc(alias = "-[FlurryDataSender everSentReportSuccessfully]")]
// 0xf0d384 — -[FlurryDataSender everSentReportSuccessfully]
// type: char __cdecl(FlurryDataSender *self, SEL)
pub fn stub_0xf0d384() -> ! { todo!("0xf0d384 -[FlurryDataSender everSentReportSuccessfully]") }

#[doc(alias = "-[FlurryDataSender sendData:withIdentifier:]")]
// 0xf0d3ac — -[FlurryDataSender sendData:withIdentifier:]
// type: void __cdecl(FlurryDataSender *self, SEL, id, id)
pub fn stub_0xf0d3ac() -> ! { todo!("0xf0d3ac -[FlurryDataSender sendData:withIdentifier:]") }

#[doc(alias = "___44-[FlurryDataSender sendData:withIdentifier:]_block_invoke_0")]
// 0xf0d5b0 — ___44-[FlurryDataSender sendData:withIdentifier:]_block_invoke_0
pub fn stub_0xf0d5b0() -> ! { todo!("0xf0d5b0 ___44-[FlurryDataSender sendData:withIdentifier:]_block_invoke_0") }

#[doc(alias = "___copy_helper_block__40")]
// 0xf0d5e8 — ___copy_helper_block__40
pub fn stub_0xf0d5e8() -> ! { todo!("0xf0d5e8 ___copy_helper_block__40") }

#[doc(alias = "___destroy_helper_block__40")]
// 0xf0d5f8 — ___destroy_helper_block__40
pub fn stub_0xf0d5f8() -> ! { todo!("0xf0d5f8 ___destroy_helper_block__40") }

#[doc(alias = "-[FlurryDataSender sendSessionsData:]")]
// 0xf0d608 — -[FlurryDataSender sendSessionsData:]
// type: void __cdecl(FlurryDataSender *self, SEL, id)
pub fn stub_0xf0d608() -> ! { todo!("0xf0d608 -[FlurryDataSender sendSessionsData:]") }

#[doc(alias = "-[FlurryDataSender requestDidFail:withResponse:]")]
// 0xf0d780 — -[FlurryDataSender requestDidFail:withResponse:]
// type: void __cdecl(FlurryDataSender *self, SEL, id, id)
pub fn stub_0xf0d780() -> ! { todo!("0xf0d780 -[FlurryDataSender requestDidFail:withResponse:]") }

#[doc(alias = "___48-[FlurryDataSender requestDidFail:withResponse:]_block_invoke_0")]
// 0xf0d85c — ___48-[FlurryDataSender requestDidFail:withResponse:]_block_invoke_0
pub fn stub_0xf0d85c() -> ! { todo!("0xf0d85c ___48-[FlurryDataSender requestDidFail:withResponse:]_block_invoke_0") }

#[doc(alias = "___copy_helper_block_108")]
// 0xf0d8c0 — ___copy_helper_block_108
pub fn stub_0xf0d8c0() -> ! { todo!("0xf0d8c0 ___copy_helper_block_108") }

#[doc(alias = "___destroy_helper_block_109")]
// 0xf0d8f0 — ___destroy_helper_block_109
pub fn stub_0xf0d8f0() -> ! { todo!("0xf0d8f0 ___destroy_helper_block_109") }

#[doc(alias = "-[FlurryDataSender requestDidCancel:withResponse:]")]
// 0xf0d910 — -[FlurryDataSender requestDidCancel:withResponse:]
// type: void __cdecl(FlurryDataSender *self, SEL, id, id)
pub fn stub_0xf0d910() -> ! { todo!("0xf0d910 -[FlurryDataSender requestDidCancel:withResponse:]") }

#[doc(alias = "___50-[FlurryDataSender requestDidCancel:withResponse:]_block_invoke_0")]
// 0xf0d9a8 — ___50-[FlurryDataSender requestDidCancel:withResponse:]_block_invoke_0
pub fn stub_0xf0d9a8() -> ! { todo!("0xf0d9a8 ___50-[FlurryDataSender requestDidCancel:withResponse:]_block_invoke_0") }

#[doc(alias = "___copy_helper_block_111")]
// 0xf0d9c8 — ___copy_helper_block_111
pub fn stub_0xf0d9c8() -> ! { todo!("0xf0d9c8 ___copy_helper_block_111") }

#[doc(alias = "___destroy_helper_block_112")]
// 0xf0d9ec — ___destroy_helper_block_112
pub fn stub_0xf0d9ec() -> ! { todo!("0xf0d9ec ___destroy_helper_block_112") }

#[doc(alias = "-[FlurryDataSender requestSuccessful:withResponse:]")]
// 0xf0da04 — -[FlurryDataSender requestSuccessful:withResponse:]
// type: void __cdecl(FlurryDataSender *self, SEL, id, id)
pub fn stub_0xf0da04() -> ! { todo!("0xf0da04 -[FlurryDataSender requestSuccessful:withResponse:]") }

#[doc(alias = "___51-[FlurryDataSender requestSuccessful:withResponse:]_block_invoke_0")]
// 0xf0daa0 — ___51-[FlurryDataSender requestSuccessful:withResponse:]_block_invoke_0
pub fn stub_0xf0daa0() -> ! { todo!("0xf0daa0 ___51-[FlurryDataSender requestSuccessful:withResponse:]_block_invoke_0") }

#[doc(alias = "___copy_helper_block_116")]
// 0xf0dac0 — ___copy_helper_block_116
pub fn stub_0xf0dac0() -> ! { todo!("0xf0dac0 ___copy_helper_block_116") }

#[doc(alias = "___destroy_helper_block_117")]
// 0xf0daf0 — ___destroy_helper_block_117
pub fn stub_0xf0daf0() -> ! { todo!("0xf0daf0 ___destroy_helper_block_117") }

#[doc(alias = "-[FlurryDataSender requestSuccessComplete:withResponse:]")]
// 0xf0db10 — -[FlurryDataSender requestSuccessComplete:withResponse:]
// type: void __cdecl(FlurryDataSender *self, SEL, id, id)
pub fn stub_0xf0db10() -> ! { todo!("0xf0db10 -[FlurryDataSender requestSuccessComplete:withResponse:]") }

#[doc(alias = "-[FlurryDataSender didCompleteAllTasks]")]
// 0xf0dc70 — -[FlurryDataSender didCompleteAllTasks]
// type: void __cdecl(FlurryDataSender *self, SEL)
pub fn stub_0xf0dc70() -> ! { todo!("0xf0dc70 -[FlurryDataSender didCompleteAllTasks]") }

#[doc(alias = "-[FlurryDataSender retransmitNotSentBlocks]")]
// 0xf0dcc4 — -[FlurryDataSender retransmitNotSentBlocks]
// type: void __cdecl(FlurryDataSender *self, SEL)
pub fn stub_0xf0dcc4() -> ! { todo!("0xf0dcc4 -[FlurryDataSender retransmitNotSentBlocks]") }

#[doc(alias = "-[FlurryDataSender dataIndex]")]
// 0xf0e0cc — -[FlurryDataSender dataIndex]
// type: FlurryDataSenderIndex *__cdecl(FlurryDataSender *self, SEL)
pub fn stub_0xf0e0cc() -> ! { todo!("0xf0e0cc -[FlurryDataSender dataIndex]") }

#[doc(alias = "-[FlurryDataSender setDataIndex:]")]
// 0xf0e0dc — -[FlurryDataSender setDataIndex:]
// type: void __cdecl(FlurryDataSender *self, SEL, id)
pub fn stub_0xf0e0dc() -> ! { todo!("0xf0e0dc -[FlurryDataSender setDataIndex:]") }

#[doc(alias = "-[FlurryDataSenderBlockInfo initWithData:]")]
// 0xf0e100 — -[FlurryDataSenderBlockInfo initWithData:]
// type: FlurryDataSenderBlockInfo *__cdecl(FlurryDataSenderBlockInfo *self, SEL, id)
pub fn stub_0xf0e100() -> ! { todo!("0xf0e100 -[FlurryDataSenderBlockInfo initWithData:]") }

#[doc(alias = "-[FlurryDataSenderBlockInfo initWithCoder:]")]
// 0xf0e290 — -[FlurryDataSenderBlockInfo initWithCoder:]
// type: FlurryDataSenderBlockInfo *__cdecl(FlurryDataSenderBlockInfo *self, SEL, id)
pub fn stub_0xf0e290() -> ! { todo!("0xf0e290 -[FlurryDataSenderBlockInfo initWithCoder:]") }

#[doc(alias = "-[FlurryDataSenderBlockInfo description]")]
// 0xf0e38c — -[FlurryDataSenderBlockInfo description]
// type: id __cdecl(FlurryDataSenderBlockInfo *self, SEL)
pub fn stub_0xf0e38c() -> ! { todo!("0xf0e38c -[FlurryDataSenderBlockInfo description]") }

#[doc(alias = "-[FlurryDataSenderBlockInfo dealloc]")]
// 0xf0e3ec — -[FlurryDataSenderBlockInfo dealloc]
// type: void __cdecl(FlurryDataSenderBlockInfo *self, SEL)
pub fn stub_0xf0e3ec() -> ! { todo!("0xf0e3ec -[FlurryDataSenderBlockInfo dealloc]") }

#[doc(alias = "-[FlurryDataSenderBlockInfo encodeWithCoder:]")]
// 0xf0e450 — -[FlurryDataSenderBlockInfo encodeWithCoder:]
// type: void __cdecl(FlurryDataSenderBlockInfo *self, SEL, id)
pub fn stub_0xf0e450() -> ! { todo!("0xf0e450 -[FlurryDataSenderBlockInfo encodeWithCoder:]") }

#[doc(alias = "-[FlurryDataSenderBlockInfo deletePersistentData]")]
// 0xf0e4e4 — -[FlurryDataSenderBlockInfo deletePersistentData]
// type: char __cdecl(FlurryDataSenderBlockInfo *self, SEL)
pub fn stub_0xf0e4e4() -> ! { todo!("0xf0e4e4 -[FlurryDataSenderBlockInfo deletePersistentData]") }

#[doc(alias = "-[FlurryDataSenderBlockInfo data]")]
// 0xf0e5f0 — -[FlurryDataSenderBlockInfo data]
// type: id __cdecl(FlurryDataSenderBlockInfo *self, SEL)
pub fn stub_0xf0e5f0() -> ! { todo!("0xf0e5f0 -[FlurryDataSenderBlockInfo data]") }

#[doc(alias = "-[FlurryDataSenderBlockInfo setData:]")]
// 0xf0e6bc — -[FlurryDataSenderBlockInfo setData:]
// type: char __cdecl(FlurryDataSenderBlockInfo *self, SEL, id)
pub fn stub_0xf0e6bc() -> ! { todo!("0xf0e6bc -[FlurryDataSenderBlockInfo setData:]") }

#[doc(alias = "-[FlurryDataSenderBlockInfo identifier]")]
// 0xf0e7dc — -[FlurryDataSenderBlockInfo identifier]
// type: NSString *__cdecl(FlurryDataSenderBlockInfo *self, SEL)
pub fn stub_0xf0e7dc() -> ! { todo!("0xf0e7dc -[FlurryDataSenderBlockInfo identifier]") }

#[doc(alias = "-[FlurryDataSenderBlockInfo setIdentifier:]")]
// 0xf0e7ec — -[FlurryDataSenderBlockInfo setIdentifier:]
// type: void __cdecl(FlurryDataSenderBlockInfo *self, SEL, id)
pub fn stub_0xf0e7ec() -> ! { todo!("0xf0e7ec -[FlurryDataSenderBlockInfo setIdentifier:]") }

#[doc(alias = "-[FlurryDataSenderBlockInfo dataSize]")]
// 0xf0e810 — -[FlurryDataSenderBlockInfo dataSize]
// type: unsigned int __cdecl(FlurryDataSenderBlockInfo *self, SEL)
pub fn stub_0xf0e810() -> ! { todo!("0xf0e810 -[FlurryDataSenderBlockInfo dataSize]") }

#[doc(alias = "-[FlurryDataSenderBlockInfo setDataSize:]")]
// 0xf0e820 — -[FlurryDataSenderBlockInfo setDataSize:]
// type: void __cdecl(FlurryDataSenderBlockInfo *self, SEL, unsigned int)
pub fn stub_0xf0e820() -> ! { todo!("0xf0e820 -[FlurryDataSenderBlockInfo setDataSize:]") }

#[doc(alias = "-[FlurryDataSenderBlockInfo creationDate]")]
// 0xf0e830 — -[FlurryDataSenderBlockInfo creationDate]
// type: NSDate *__cdecl(FlurryDataSenderBlockInfo *self, SEL)
pub fn stub_0xf0e830() -> ! { todo!("0xf0e830 -[FlurryDataSenderBlockInfo creationDate]") }

#[doc(alias = "-[FlurryDataSenderBlockInfo setCreationDate:]")]
// 0xf0e840 — -[FlurryDataSenderBlockInfo setCreationDate:]
// type: void __cdecl(FlurryDataSenderBlockInfo *self, SEL, id)
pub fn stub_0xf0e840() -> ! { todo!("0xf0e840 -[FlurryDataSenderBlockInfo setCreationDate:]") }

#[doc(alias = "+[FlurryDataSenderIndex instantiatedIndex]")]
// 0xf0e864 — +[FlurryDataSenderIndex instantiatedIndex]
// type: id __cdecl(id, SEL)
pub fn stub_0xf0e864() -> ! { todo!("0xf0e864 +[FlurryDataSenderIndex instantiatedIndex]") }

#[doc(alias = "-[FlurryDataSenderIndex init]")]
// 0xf0e928 — -[FlurryDataSenderIndex init]
// type: FlurryDataSenderIndex *__cdecl(FlurryDataSenderIndex *self, SEL)
pub fn stub_0xf0e928() -> ! { todo!("0xf0e928 -[FlurryDataSenderIndex init]") }

#[doc(alias = "-[FlurryDataSenderIndex initWithCoder:]")]
// 0xf0e994 — -[FlurryDataSenderIndex initWithCoder:]
// type: FlurryDataSenderIndex *__cdecl(FlurryDataSenderIndex *self, SEL, id)
pub fn stub_0xf0e994() -> ! { todo!("0xf0e994 -[FlurryDataSenderIndex initWithCoder:]") }

#[doc(alias = "-[FlurryDataSenderIndex dealloc]")]
// 0xf0ea68 — -[FlurryDataSenderIndex dealloc]
// type: void __cdecl(FlurryDataSenderIndex *self, SEL)
pub fn stub_0xf0ea68() -> ! { todo!("0xf0ea68 -[FlurryDataSenderIndex dealloc]") }

#[doc(alias = "-[FlurryDataSenderIndex encodeWithCoder:]")]
// 0xf0eab8 — -[FlurryDataSenderIndex encodeWithCoder:]
// type: void __cdecl(FlurryDataSenderIndex *self, SEL, id)
pub fn stub_0xf0eab8() -> ! { todo!("0xf0eab8 -[FlurryDataSenderIndex encodeWithCoder:]") }

#[doc(alias = "-[FlurryDataSenderIndex saveState]")]
// 0xf0eb1c — -[FlurryDataSenderIndex saveState]
// type: void __cdecl(FlurryDataSenderIndex *self, SEL)
pub fn stub_0xf0eb1c() -> ! { todo!("0xf0eb1c -[FlurryDataSenderIndex saveState]") }

#[doc(alias = "-[FlurryDataSenderIndex addBlockInfo:forDataKey:]")]
// 0xf0eb94 — -[FlurryDataSenderIndex addBlockInfo:forDataKey:]
// type: void __cdecl(FlurryDataSenderIndex *self, SEL, id, id)
pub fn stub_0xf0eb94() -> ! { todo!("0xf0eb94 -[FlurryDataSenderIndex addBlockInfo:forDataKey:]") }
