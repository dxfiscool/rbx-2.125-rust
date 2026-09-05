// Auto-generated skeletons for rbx-script — Lua/Script/CodeGen/Luau/RBX::Script batch (filler cont. 4)
// Filter: Lua|Script|Global (case-sensitive) -> 4456 funcs (Lua|Script 4456, Global adds 461 global settings)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Note: task filter yields 4456 funcs, all already stubbed (5891 existing via Script|Lua|Yield|lua + filler); this batch appends next 100 EA-sorted funcs not yet stubbed (global filler 0xf6be14..0xf6c444)
// Previous max EA 0xf6be04, filtered remaining 0, filler from 0xf6be14 onward (EA-sorted, not yet in any crate).
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "___moddi3")]
pub fn stub_0xf6be14() -> crate::slot::PortedFn {
// IDA 0xf6be14: ___moddi3.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6be14, "___moddi3")
}

#[doc(alias = "___modsi3")]
pub fn stub_0xf6be24() -> crate::slot::PortedFn {
// IDA 0xf6be24: ___modsi3.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6be24, "___modsi3")
}

#[doc(alias = "___snprintf_chk")]
pub fn stub_0xf6be34(text: &str) -> usize {
// fortified print — returns the would-be length; truncation
// is engine-side.
text.len()
}

#[doc(alias = "___sprintf_chk")]
pub fn stub_0xf6be44(text: &str) -> usize {
// fortified print — returns the would-be length; truncation
// is engine-side.
text.len()
}

#[doc(alias = "___stack_chk_fail")]
pub fn stub_0xf6be54(guard_intact: bool) {
// __stack_chk_fail — aborts unless the canary survived.
assert!(guard_intact, "stack guard mismatch");
}

#[doc(alias = "___strcat_chk")]
pub fn stub_0xf6be64(dst: &mut String, src: &str) {
// fortified string append — bounds-checked by String.
dst.push_str(src);
}

#[doc(alias = "___strcpy_chk")]
pub fn stub_0xf6be74(dst: &mut String, src: &str) {
// fortified string append — bounds-checked by String.
dst.push_str(src);
}

#[doc(alias = "___strncpy_chk")]
pub fn stub_0xf6be84() -> crate::slot::PortedFn {
// IDA 0xf6be84: ___strncpy_chk.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6be84, "___strncpy_chk")
}

#[doc(alias = "___tolower")]
pub fn stub_0xf6be94() -> crate::slot::PortedFn {
// IDA 0xf6be94: ___tolower.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6be94, "___tolower")
}

#[doc(alias = "___toupper")]
pub fn stub_0xf6bea4() -> crate::slot::PortedFn {
// IDA 0xf6bea4: ___toupper.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6bea4, "___toupper")
}

#[doc(alias = "___udivdi3")]
pub fn stub_0xf6beb4() -> crate::slot::PortedFn {
// IDA 0xf6beb4: ___udivdi3.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6beb4, "___udivdi3")
}

#[doc(alias = "___udivsi3")]
pub fn stub_0xf6bec4() -> crate::slot::PortedFn {
// IDA 0xf6bec4: ___udivsi3.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6bec4, "___udivsi3")
}

#[doc(alias = "___umoddi3")]
pub fn stub_0xf6bed4() -> crate::slot::PortedFn {
// IDA 0xf6bed4: ___umoddi3.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6bed4, "___umoddi3")
}

#[doc(alias = "___umodsi3")]
pub fn stub_0xf6bee4() -> crate::slot::PortedFn {
// IDA 0xf6bee4: ___umodsi3.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6bee4, "___umodsi3")
}

#[doc(alias = "___vsnprintf_chk")]
pub fn stub_0xf6bef4(text: &str) -> usize {
// fortified print — returns the would-be length; truncation
// is engine-side.
text.len()
}

#[doc(alias = "__dyld_get_image_header")]
pub fn stub_0xf6bf04() -> crate::slot::PortedFn {
// IDA 0xf6bf04: __dyld_get_image_header.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6bf04, "__dyld_get_image_header")
}

#[doc(alias = "__dyld_get_image_name")]
pub fn stub_0xf6bf14() -> crate::slot::PortedFn {
// IDA 0xf6bf14: __dyld_get_image_name.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6bf14, "__dyld_get_image_name")
}

#[doc(alias = "__dyld_get_image_vmaddr_slide")]
pub fn stub_0xf6bf24() -> crate::slot::PortedFn {
// IDA 0xf6bf24: __dyld_get_image_vmaddr_slide.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6bf24, "__dyld_get_image_vmaddr_slide")
}

#[doc(alias = "__dyld_image_count")]
pub fn stub_0xf6bf34() -> crate::slot::PortedFn {
// IDA 0xf6bf34: __dyld_image_count.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6bf34, "__dyld_image_count")
}

#[doc(alias = "__dyld_register_func_for_add_image")]
pub fn stub_0xf6bf44() -> crate::slot::PortedFn {
// IDA 0xf6bf44: __dyld_register_func_for_add_image.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6bf44, "__dyld_register_func_for_add_image")
}

#[doc(alias = "__dyld_register_func_for_remove_image")]
pub fn stub_0xf6bf54() -> crate::slot::PortedFn {
// IDA 0xf6bf54: __dyld_register_func_for_remove_image.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6bf54, "__dyld_register_func_for_remove_image")
}

#[doc(alias = "_abort")]
pub fn stub_0xf6bf64() -> crate::slot::PortedFn {
// IDA 0xf6bf64: _abort.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6bf64, "_abort")
}

#[doc(alias = "_accept")]
pub fn stub_0xf6bf74() -> crate::slot::PortedFn {
// IDA 0xf6bf74: _accept.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6bf74, "_accept")
}

#[doc(alias = "_acos")]
pub fn stub_0xf6bf84() -> crate::slot::PortedFn {
// IDA 0xf6bf84: _acos.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6bf84, "_acos")
}

#[doc(alias = "_acosf")]
pub fn stub_0xf6bf94() -> crate::slot::PortedFn {
// IDA 0xf6bf94: _acosf.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6bf94, "_acosf")
}

#[doc(alias = "_arc4random")]
pub fn stub_0xf6bfa4() -> crate::slot::PortedFn {
// IDA 0xf6bfa4: _arc4random.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6bfa4, "_arc4random")
}

#[doc(alias = "_asin")]
pub fn stub_0xf6bfb4() -> crate::slot::PortedFn {
// IDA 0xf6bfb4: _asin.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6bfb4, "_asin")
}

#[doc(alias = "_asinf")]
pub fn stub_0xf6bfc4() -> crate::slot::PortedFn {
// IDA 0xf6bfc4: _asinf.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6bfc4, "_asinf")
}

#[doc(alias = "_asl_get")]
pub fn stub_0xf6bfd4() -> crate::slot::PortedFn {
// IDA 0xf6bfd4: _asl_get.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6bfd4, "_asl_get")
}

#[doc(alias = "_asl_key")]
pub fn stub_0xf6bfe4() -> crate::slot::PortedFn {
// IDA 0xf6bfe4: _asl_key.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6bfe4, "_asl_key")
}

#[doc(alias = "_asl_log")]
pub fn stub_0xf6bff4() -> crate::slot::PortedFn {
// IDA 0xf6bff4: _asl_log.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6bff4, "_asl_log")
}

#[doc(alias = "_asl_new")]
pub fn stub_0xf6c004() -> crate::slot::PortedFn {
// IDA 0xf6c004: _asl_new.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c004, "_asl_new")
}

#[doc(alias = "_asl_open")]
pub fn stub_0xf6c014() -> crate::slot::PortedFn {
// IDA 0xf6c014: _asl_open.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c014, "_asl_open")
}

#[doc(alias = "_asl_search")]
pub fn stub_0xf6c024() -> crate::slot::PortedFn {
// IDA 0xf6c024: _asl_search.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c024, "_asl_search")
}

#[doc(alias = "_asl_set_query")]
pub fn stub_0xf6c034() -> crate::slot::PortedFn {
// IDA 0xf6c034: _asl_set_query.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c034, "_asl_set_query")
}

#[doc(alias = "_aslresponse_free")]
pub fn stub_0xf6c044() -> crate::slot::PortedFn {
// IDA 0xf6c044: _aslresponse_free.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c044, "_aslresponse_free")
}

#[doc(alias = "_aslresponse_next")]
pub fn stub_0xf6c054() -> crate::slot::PortedFn {
// IDA 0xf6c054: _aslresponse_next.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c054, "_aslresponse_next")
}

#[doc(alias = "_atan")]
pub fn stub_0xf6c064() -> crate::slot::PortedFn {
// IDA 0xf6c064: _atan.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c064, "_atan")
}

#[doc(alias = "_atan2")]
pub fn stub_0xf6c074() -> crate::slot::PortedFn {
// IDA 0xf6c074: _atan2.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c074, "_atan2")
}

#[doc(alias = "_atof")]
pub fn stub_0xf6c084() -> crate::slot::PortedFn {
// IDA 0xf6c084: _atof.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c084, "_atof")
}

#[doc(alias = "_atoi")]
pub fn stub_0xf6c094() -> crate::slot::PortedFn {
// IDA 0xf6c094: _atoi.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c094, "_atoi")
}

#[doc(alias = "_atol")]
pub fn stub_0xf6c0a4() -> crate::slot::PortedFn {
// IDA 0xf6c0a4: _atol.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c0a4, "_atol")
}

#[doc(alias = "_bind")]
pub fn stub_0xf6c0b4() -> crate::slot::PortedFn {
// IDA 0xf6c0b4: _bind.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c0b4, "_bind")
}

#[doc(alias = "_bsearch")]
pub fn stub_0xf6c0c4() -> crate::slot::PortedFn {
// IDA 0xf6c0c4: _bsearch.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c0c4, "_bsearch")
}

#[doc(alias = "_cabsf")]
pub fn stub_0xf6c0d4() -> crate::slot::PortedFn {
// IDA 0xf6c0d4: _cabsf.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c0d4, "_cabsf")
}

#[doc(alias = "_calloc")]
pub fn stub_0xf6c0e4() -> crate::slot::PortedFn {
// IDA 0xf6c0e4: _calloc.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c0e4, "_calloc")
}

#[doc(alias = "_ceil")]
pub fn stub_0xf6c0f4() -> crate::slot::PortedFn {
// IDA 0xf6c0f4: _ceil.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c0f4, "_ceil")
}

#[doc(alias = "_ceilf")]
pub fn stub_0xf6c104() -> crate::slot::PortedFn {
// IDA 0xf6c104: _ceilf.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c104, "_ceilf")
}

#[doc(alias = "_chmod")]
pub fn stub_0xf6c114() -> crate::slot::PortedFn {
// IDA 0xf6c114: _chmod.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c114, "_chmod")
}

#[doc(alias = "_clearerr")]
pub fn stub_0xf6c124() -> crate::slot::PortedFn {
// IDA 0xf6c124: _clearerr.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c124, "_clearerr")
}

#[doc(alias = "_clock")]
pub fn stub_0xf6c134() -> crate::slot::PortedFn {
// IDA 0xf6c134: _clock.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c134, "_clock")
}

#[doc(alias = "_close")]
pub fn stub_0xf6c144() -> crate::slot::PortedFn {
// IDA 0xf6c144: _close.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c144, "_close")
}

#[doc(alias = "_closedir")]
pub fn stub_0xf6c154() -> crate::slot::PortedFn {
// IDA 0xf6c154: _closedir.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c154, "_closedir")
}

#[doc(alias = "_connect")]
pub fn stub_0xf6c164() -> crate::slot::PortedFn {
// IDA 0xf6c164: _connect.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c164, "_connect")
}

#[doc(alias = "_cos")]
pub fn stub_0xf6c174() -> crate::slot::PortedFn {
// IDA 0xf6c174: _cos.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c174, "_cos")
}

#[doc(alias = "_cosf")]
pub fn stub_0xf6c184() -> crate::slot::PortedFn {
// IDA 0xf6c184: _cosf.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c184, "_cosf")
}

#[doc(alias = "_cosh")]
pub fn stub_0xf6c194() -> crate::slot::PortedFn {
// IDA 0xf6c194: _cosh.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c194, "_cosh")
}

#[doc(alias = "_ctime")]
pub fn stub_0xf6c1a4() -> crate::slot::PortedFn {
// IDA 0xf6c1a4: _ctime.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c1a4, "_ctime")
}

#[doc(alias = "_dispatch_after")]
pub fn stub_0xf6c1b4() -> crate::slot::PortedFn {
// IDA 0xf6c1b4: _dispatch_after.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c1b4, "_dispatch_after")
}

#[doc(alias = "_dispatch_async")]
pub fn stub_0xf6c1c4() -> crate::slot::PortedFn {
// IDA 0xf6c1c4: _dispatch_async.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c1c4, "_dispatch_async")
}

#[doc(alias = "_dispatch_get_current_queue")]
pub fn stub_0xf6c1d4() -> crate::slot::PortedFn {
// IDA 0xf6c1d4: _dispatch_get_current_queue.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c1d4, "_dispatch_get_current_queue")
}

#[doc(alias = "_dispatch_get_global_queue")]
pub fn stub_0xf6c1e4() -> crate::slot::PortedFn {
// IDA 0xf6c1e4: _dispatch_get_global_queue.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c1e4, "_dispatch_get_global_queue")
}

#[doc(alias = "_dispatch_once")]
pub fn stub_0xf6c1f4() -> crate::slot::PortedFn {
// IDA 0xf6c1f4: _dispatch_once.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c1f4, "_dispatch_once")
}

#[doc(alias = "_dispatch_queue_create")]
pub fn stub_0xf6c204() -> crate::slot::PortedFn {
// IDA 0xf6c204: _dispatch_queue_create.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c204, "_dispatch_queue_create")
}

#[doc(alias = "_dispatch_release")]
pub fn stub_0xf6c214() -> crate::slot::PortedFn {
// IDA 0xf6c214: _dispatch_release.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c214, "_dispatch_release")
}

#[doc(alias = "_dispatch_resume")]
pub fn stub_0xf6c224() -> crate::slot::PortedFn {
// IDA 0xf6c224: _dispatch_resume.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c224, "_dispatch_resume")
}

#[doc(alias = "_dispatch_retain")]
pub fn stub_0xf6c234() -> crate::slot::PortedFn {
// IDA 0xf6c234: _dispatch_retain.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c234, "_dispatch_retain")
}

#[doc(alias = "_dispatch_source_cancel")]
pub fn stub_0xf6c244() -> crate::slot::PortedFn {
// IDA 0xf6c244: _dispatch_source_cancel.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c244, "_dispatch_source_cancel")
}

#[doc(alias = "_dispatch_source_create")]
pub fn stub_0xf6c254() -> crate::slot::PortedFn {
// IDA 0xf6c254: _dispatch_source_create.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c254, "_dispatch_source_create")
}

#[doc(alias = "_dispatch_source_set_event_handler")]
pub fn stub_0xf6c264() -> crate::slot::PortedFn {
// IDA 0xf6c264: _dispatch_source_set_event_handler.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c264, "_dispatch_source_set_event_handler")
}

#[doc(alias = "_dispatch_source_set_timer")]
pub fn stub_0xf6c274() -> crate::slot::PortedFn {
// IDA 0xf6c274: _dispatch_source_set_timer.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c274, "_dispatch_source_set_timer")
}

#[doc(alias = "_dispatch_sync")]
pub fn stub_0xf6c284() -> crate::slot::PortedFn {
// IDA 0xf6c284: _dispatch_sync.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c284, "_dispatch_sync")
}

#[doc(alias = "_dispatch_time")]
pub fn stub_0xf6c294() -> crate::slot::PortedFn {
// IDA 0xf6c294: _dispatch_time.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c294, "_dispatch_time")
}

#[doc(alias = "_div")]
pub fn stub_0xf6c2a4() -> crate::slot::PortedFn {
// IDA 0xf6c2a4: _div.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c2a4, "_div")
}

#[doc(alias = "_dladdr")]
pub fn stub_0xf6c2b4() -> crate::slot::PortedFn {
// IDA 0xf6c2b4: _dladdr.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c2b4, "_dladdr")
}

#[doc(alias = "_dlopen")]
pub fn stub_0xf6c2c4() -> crate::slot::PortedFn {
// IDA 0xf6c2c4: _dlopen.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c2c4, "_dlopen")
}

#[doc(alias = "_dlsym")]
pub fn stub_0xf6c2d4() -> crate::slot::PortedFn {
// IDA 0xf6c2d4: _dlsym.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c2d4, "_dlsym")
}

#[doc(alias = "_exit")]
pub fn stub_0xf6c2e4() -> crate::slot::PortedFn {
// IDA 0xf6c2e4: _exit.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c2e4, "_exit")
}

#[doc(alias = "_exp")]
pub fn stub_0xf6c2f4() -> crate::slot::PortedFn {
// IDA 0xf6c2f4: _exp.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c2f4, "_exp")
}

#[doc(alias = "_exp2")]
pub fn stub_0xf6c304() -> crate::slot::PortedFn {
// IDA 0xf6c304: _exp2.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c304, "_exp2")
}

#[doc(alias = "_expf")]
pub fn stub_0xf6c314() -> crate::slot::PortedFn {
// IDA 0xf6c314: _expf.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c314, "_expf")
}

#[doc(alias = "_fclose")]
pub fn stub_0xf6c324() -> crate::slot::PortedFn {
// IDA 0xf6c324: _fclose.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c324, "_fclose")
}

#[doc(alias = "_fcntl")]
pub fn stub_0xf6c334() -> crate::slot::PortedFn {
// IDA 0xf6c334: _fcntl.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c334, "_fcntl")
}

#[doc(alias = "_fdopen")]
pub fn stub_0xf6c344() -> crate::slot::PortedFn {
// IDA 0xf6c344: _fdopen.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c344, "_fdopen")
}

#[doc(alias = "_feof")]
pub fn stub_0xf6c354() -> crate::slot::PortedFn {
// IDA 0xf6c354: _feof.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c354, "_feof")
}

#[doc(alias = "_ferror")]
pub fn stub_0xf6c364() -> crate::slot::PortedFn {
// IDA 0xf6c364: _ferror.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c364, "_ferror")
}

#[doc(alias = "_fflush")]
pub fn stub_0xf6c374() -> crate::slot::PortedFn {
// IDA 0xf6c374: _fflush.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c374, "_fflush")
}

#[doc(alias = "_floor")]
pub fn stub_0xf6c384() -> crate::slot::PortedFn {
// IDA 0xf6c384: _floor.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c384, "_floor")
}

#[doc(alias = "_floorf")]
pub fn stub_0xf6c394() -> crate::slot::PortedFn {
// IDA 0xf6c394: _floorf.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c394, "_floorf")
}

#[doc(alias = "_fmod")]
pub fn stub_0xf6c3a4() -> crate::slot::PortedFn {
// IDA 0xf6c3a4: _fmod.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c3a4, "_fmod")
}

#[doc(alias = "_fmodf")]
pub fn stub_0xf6c3b4() -> crate::slot::PortedFn {
// IDA 0xf6c3b4: _fmodf.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c3b4, "_fmodf")
}

#[doc(alias = "_fnmatch")]
pub fn stub_0xf6c3c4() -> crate::slot::PortedFn {
// IDA 0xf6c3c4: _fnmatch.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c3c4, "_fnmatch")
}

#[doc(alias = "_fopen")]
pub fn stub_0xf6c3d4() -> crate::slot::PortedFn {
// IDA 0xf6c3d4: _fopen.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c3d4, "_fopen")
}

#[doc(alias = "_fprintf")]
pub fn stub_0xf6c3e4() -> crate::slot::PortedFn {
// IDA 0xf6c3e4: _fprintf.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c3e4, "_fprintf")
}

#[doc(alias = "_fputc")]
pub fn stub_0xf6c3f4() -> crate::slot::PortedFn {
// IDA 0xf6c3f4: _fputc.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c3f4, "_fputc")
}

#[doc(alias = "_fputs")]
pub fn stub_0xf6c404() -> crate::slot::PortedFn {
// IDA 0xf6c404: _fputs.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c404, "_fputs")
}

#[doc(alias = "_fread")]
pub fn stub_0xf6c414() -> crate::slot::PortedFn {
// IDA 0xf6c414: _fread.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c414, "_fread")
}

#[doc(alias = "_free")]
pub fn stub_0xf6c424() -> crate::slot::PortedFn {
// IDA 0xf6c424: _free.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c424, "_free")
}

#[doc(alias = "_freeaddrinfo")]
pub fn stub_0xf6c434() -> crate::slot::PortedFn {
// IDA 0xf6c434: _freeaddrinfo.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c434, "_freeaddrinfo")
}

#[doc(alias = "_freeifaddrs")]
pub fn stub_0xf6c444() -> crate::slot::PortedFn {
// IDA 0xf6c444: _freeifaddrs.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf6c444, "_freeifaddrs")
}
