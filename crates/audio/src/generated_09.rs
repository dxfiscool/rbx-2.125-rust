//! audio generated_09 — next 120 stubs EA-sorted, from ida/export.json
//! Filter: FMOD|Sound|Audio (2544 distinct) — filler from remaining unclaimed EA (workspace EA-sorted asc, skip existing)
//! Batch: 120 stubs | skeleton batch | range 0x82b700..0x125150 EA-sorted asc, skip existing, rbx_core::SharedPtr not boost
//! Generated: 2026-08-31

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// Ensure SharedPtr is seen as used — mirrors boost::shared_ptr<T> -> rbx_core::SharedPtr<T>
const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};
// ---- FreeImage JPEG/PNG plugin leaves + math fmod (IDA 0x82b700..0x1161e0) ----
// Target is 32-bit ARM (armv7); interior plugin words are plain u32/i32/f32
// fields so the byte offsets cited below hold on any host.
// Boost mapping: none in this range (no shared_ptr/intrusive_ptr/signal).
use core::ffi::{c_char, c_void};

/// strcmp against a NUL-terminated literal without libc (IDA 0x11204c).
fn c_str_eq(mut s: *const c_char, lit: &[u8]) -> bool {
    // IDA 0x112070..0x11208c: strcmp("ICC_PROFILE", data) == 0.
    for &want in lit {
        // SAFETY: caller guarantees `s` reads a NUL-terminated string with
        // at least lit.len() readable bytes, as in the original call.
        let got = unsafe { *s as u8 };
        if got != want {
            return false;
        }
        if want == 0 {
            return true;
        }
        s = unsafe { s.add(1) };
    }
    false
}

/// Minimal byte view of jpeg_marker_struct for IDA 0x11204c: marker byte at
/// +4, data length (u32) at +12, data pointer at +16.
#[repr(C)]
pub struct JpegMarkerIccView {
    pub _pad0: [u8; 4],
    pub marker: u8,
    pub _pad1: [u8; 7],
    pub data_length: u32,
    pub data: *const c_char,
}
/// FreeImageIO hook table as consumed by the TIFF client procs
/// (IDA 0x11600c..0x116378): read_proc at +0, write_proc at +4,
/// seek_proc at +8, tell_proc at +12. The original reassembles each entry
/// byte-wise from the table (four LDRB + ORR, e.g. 0x1160b0..0x1160c8) —
/// a little-endian function-pointer word. IDA prints the callees
/// __fastcall, which on ARM is the default AAPCS ABI, spelled
/// `extern "C"` here.
#[repr(C)]
pub struct TiffIoHooks {
    pub read_proc: unsafe extern "C" fn(*mut c_void, u32, u32, *mut c_void) -> u32,
    pub write_proc: unsafe extern "C" fn(*mut c_void, u32, u32, *mut c_void) -> u32,
    pub seek_proc: unsafe extern "C" fn(*mut c_void, u32, i32),
    pub tell_proc: unsafe extern "C" fn(*mut c_void) -> u32,
}

/// libtiff `thandle_t` for this client: the IO table plus the opaque
/// FreeImage handle. IDA 0x11600c: `*a1` is the table (deref'd for the
/// proc word), `a1[1]` is the handle forwarded to every proc call.
#[repr(C)]
pub struct TiffHandlePair {
    pub io: *const TiffIoHooks,
    pub handle: *mut c_void,
}

/// FreeImage `Plugin` vtable slots filled by InitTIFF, in target field
/// order (IDA 0x11629c..0x116334: +0x00 format through +0x38
/// supports-ICC).
/// BUG: fields are pointer-sized, so on a 64-bit host this struct is
/// wider than the 32-bit target's 60 bytes; the slot order — not the
/// byte offsets — is what this port preserves.
#[repr(C)]
pub struct TiffPluginView {
    pub format: *const c_void,
    pub description: *const c_void,
    pub extension: *const c_void,
    pub reg_expr: *const c_void,
    pub open: *const c_void,
    pub close: *const c_void,
    pub page_count: *const c_void,
    pub reserved: *const c_void,
    pub load: *const c_void,
    pub save: *const c_void,
    pub validate: *const c_void,
    pub mime_type: *const c_void,
    pub supports_export_depth: *const c_void,
    pub supports_export_type: *const c_void,
    pub supports_icc_profiles: *const c_void,
}

/// `s_format_id` global written by InitTIFF (IDA 0x116280..0x116288:
/// STR R1,[PC,R12] to `s_format_id_1`).
static TIFF_S_FORMAT_ID: core::sync::atomic::AtomicI32 =
    core::sync::atomic::AtomicI32::new(0);

// 0x82b700 — __ZL9math_fmodP9lua_State
#[doc(alias = "math_fmod(lua_State *)")]
pub fn stub_82b700(state: *mut c_void) -> i32 {
    // IDA 0x82b700: v2 = luaL_checknumber(L, 1) (0x82b70e),
    // v3 = luaL_checknumber(L, 2) (0x82b714), v4 = fmod(v2, v3) (0x82b720),
    // lua_pushnumber(L, v4) (0x82b72e); return 1 (0x82b734).
    // NOTE: no Lua runtime lives in this crate, so the stack traffic above
    // cannot execute here; the numeric core is math_fmod_core below.
    let _ = state;
    1
}

/// Numeric core of IDA 0x82b700: C fmod of the two checked Lua numbers.
/// Rust `%` on f64 lowers to the same trunc-form remainder (fmod) call.
pub fn math_fmod_core(x: f64, y: f64) -> f64 {
    x % y
}

// 0xf6c3a4 — _fmod
#[doc(alias = "_fmod")]
pub fn stub_f6c3a4(x: f64, y: f64) -> f64 {
    // IDA 0xf6c3a4: __picsymbolstub4 — LDR R12,=(_fmod_ptr-.) (0xf6c3a4),
    // ADD R12,PC (0xf6c3a8), LDR PC,[R12] -> __imp__fmod (0xf6c3ac).
    // Pure dyld trampoline; the host mapping is a direct remainder call.
    x % y
}

// 0xf6c3b4 — _fmodf
#[doc(alias = "_fmodf")]
pub fn stub_f6c3b4(x: f32, y: f32) -> f32 {
    // IDA 0xf6c3b4: __picsymbolstub4 — LDR R12,=(_fmodf_ptr-.) (0xf6c3b4),
    // ADD R12,PC (0xf6c3b8), LDR PC,[R12] -> __imp__fmodf (0xf6c3bc).
    // Pure dyld trampoline; the host mapping is a direct remainder call.
    x % y
}

// 0xb76c — __ZN3rbx7signals16signal_with_argsILi1EFvPKN3RBX10Reflection18PropertyDescriptorEEEclES6_
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Reflection::PropertyDescriptor const*)>::operator()(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_b76c() -> ! {
    todo!("0xb76c rbx::signals::signal_with_args<1,void ()(RBX::Reflection::PropertyDescriptor const*)>::operator()(RBX::Reflection::PropertyDescriptor const*)")
}

// 0xf574 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4nextERN5boost13intrusive_ptrINS8_4slotEEE
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot> &)")]
pub fn stub_f574() -> ! {
    todo!("0xf574 rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot> &)")
}

// 0xf6dc — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE8on_errorERSt9exception
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::on_error(std::exception &)")]
pub fn stub_f6dc() -> ! {
    todo!("0xf6dc rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::on_error(std::exception &)")
}

// 0x17aac — __ZN5boost10shared_ptrIN3RBX10Reflection5TupleEEC1IS3_EEPT_
#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::Tuple>::shared_ptr<RBX::Reflection::Tuple>(RBX::Reflection::Tuple *)")]
pub fn stub_17aac() -> ! {
    todo!("0x17aac boost::shared_ptr<RBX::Reflection::Tuple>::shared_ptr<RBX::Reflection::Tuple>(RBX::Reflection::Tuple *)")
}

// 0x17b80 — __ZN5boost10shared_ptrIKN3RBX10Reflection5TupleEEC2IS3_EERKNS0_IT_EENS_6detail24sp_enable_if_convertibleIS7_S4_E4typeE
#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::Tuple const>::shared_ptr<RBX::Reflection::Tuple>(rbx_core::SharedPtr<RBX::Reflection::Tuple> const&,boost::detail::sp_enable_if_convertible<RBX::Reflection::Tuple,RBX::Reflection::Tuple const>::type)")]
pub fn stub_17b80() -> ! {
    todo!("0x17b80 boost::shared_ptr<RBX::Reflection::Tuple const>::shared_ptr<RBX::Reflection::Tuple>(boost::shared_ptr<RBX::Reflection::Tuple> const&,boost::detail::sp_enable_if_convertible<RBX::Reflection::Tuple,RBX::Reflection::Tuple const>::type)")
}

// 0x31a10 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12LoginServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LoginService,RBX::LoginService>(rbx_core::SharedPtr<RBX::LoginService> const*,RBX::LoginService *)const")]
pub fn stub_31a10() {
    // IDA 0x31a10: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0x31c30 — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_13sLoginServiceEEE15isNullClassNameEv
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_13sLoginServiceEEE15isNullClassNameEv")]
pub fn stub_31c30() -> ! {
    todo!("0x31c30 __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_13sLoginServiceEEE15isNullClassNameEv")
}

// 0x32410 — __ZNK3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_32410() -> ! {
    todo!("0x32410 __ZNK3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7Creator12getClassNameEv")
}

// 0x3247c — __ZNK3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7Creator6createEv")]
pub fn stub_3247c() -> ! {
    todo!("0x3247c __ZNK3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7Creator6createEv")
}

// 0x32768 — __ZNK3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E7Creator12getClassNameEv")]
pub fn stub_32768() -> ! {
    todo!("0x32768 __ZNK3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E7Creator12getClassNameEv")
}

// 0x3a790 — __ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E7CreatorD1Ev")]
pub fn stub_3a790() {
    // IDA 0x3a790: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3aaa0 — __ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E7CreatorC2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E7CreatorC2Ev")]
pub fn stub_3aaa0() -> ! {
    todo!("0x3aaa0 __ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E7CreatorC2Ev")
}

// 0x3bb58 — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_18sControllerServiceEEE15isNullClassNameEv
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_18sControllerServiceEEE15isNullClassNameEv")]
pub fn stub_3bb58() -> ! {
    todo!("0x3bb58 __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_18sControllerServiceEEE15isNullClassNameEv")
}

// 0x111e78 — __ZL11Descriptionv
#[doc(alias = "Description(void)")]
pub fn stub_111e78() -> &'static str {
    // IDA 0x111e78: LDR R0,="JPEG - JFIF Compliant" (0x111e78..0x111e7c); BX LR (0x111e80).
    "JPEG - JFIF Compliant"
}

// 0x111e88 — __ZL9Extensionv
#[doc(alias = "Extension(void)")]
pub fn stub_111e88() -> &'static str {
    // IDA 0x111e88: LDR R0,="jpg,jif,jpeg,jpe" (0x111e88..0x111e8c); BX LR (0x111e90).
    "jpg,jif,jpeg,jpe"
}

// 0x111e98 — __ZL7RegExprv
#[doc(alias = "RegExpr(void)")]
pub fn stub_111e98() -> &'static [u8] {
    // IDA 0x111e98: LDR R0,=asc_10CF934 (0x111e98..0x111e9c); BX LR (0x111ea0);
    // decompile: "^\\xFF\\xD8\\xFF". The raw bytes are not valid UTF-8, so
    // the exact image bytes are preserved as &[u8], not &str.
    b"^\xFF\xD8\xFF"
}

// 0x111ea8 — __ZL8MimeTypev
#[doc(alias = "MimeType(void)")]
pub fn stub_111ea8() -> &'static str {
    // IDA 0x111ea8: LDR R0,="image/jpeg" (0x111ea8..0x111eac); BX LR (0x111eb0).
    "image/jpeg"
}

// 0x111eb8 — __ZL19SupportsExportDepthi
#[doc(alias = "SupportsExportDepth(int)")]
pub fn stub_111eb8(depth: i32) -> bool {
    // IDA 0x111eb8: CMP R0,#0x18 (0x111eb8); CMPNE R0,#8 (0x111ebc);
    // EQ -> 1 else 0 (0x111ec0..0x111ec8). JPEG exports 24- and 8-bit only.
    depth == 24 || depth == 8
}

// 0x111ecc — __ZL18SupportsExportType15FREE_IMAGE_TYPE
#[doc(alias = "SupportsExportType(FREE_IMAGE_TYPE)")]
pub fn stub_111ecc(image_type: i32) -> bool {
    // IDA 0x111ecc: CMP R0,#1 (0x111ecc); NE -> 0 else 1
    // (0x111ed0..0x111ed8). Only FIT_UINT16 (1) is JPEG-exportable.
    image_type == 1
}

// 0x111edc — __ZL19SupportsICCProfilesv
#[doc(alias = "SupportsICCProfiles(void)")]
pub fn stub_111edc() -> i32 {
    // IDA 0x111edc: MOV R0,#1 (0x111edc); BX LR (0x111ee0). Always TRUE.
    1
}

// 0x111ee4 — __Z8InitJPEGP6Plugini
#[doc(alias = "InitJPEG(Plugin *,int)")]
pub fn stub_111ee4() -> ! {
    todo!("0x111ee4 InitJPEG(Plugin *,int)")
}

// 0x111fb8 — __ZL8ValidateP11FreeImageIOPv
#[doc(alias = "Validate(FreeImageIO *,void *)")]
pub fn stub_111fb8() -> ! {
    todo!("0x111fb8 Validate(FreeImageIO *,void *)")
}

// 0x11204c — __ZL13marker_is_iccP18jpeg_marker_struct
#[doc(alias = "marker_is_icc(jpeg_marker_struct *)")]
pub fn stub_11204c(marker: *const JpegMarkerIccView) -> bool {
    // IDA 0x11204c: LDRB R3,[R0,#4] (0x112054), marker == 0xE2 (APP2)
    // (0x11205c); LDR [R0,#0xC] > 0xD (0x112064..0x11206c); then
    // strcmp("ICC_PROFILE", [R0,#16]) == 0 (0x112070..0x11208c).
    // SAFETY: `marker` is a readable jpeg_marker_struct per the caller.
    let m = unsafe { &*marker };
    m.marker == 0xE2 && m.data_length > 0xD && c_str_eq(m.data, b"ICC_PROFILE\0")
}

// 0x11209c — __ZL17fill_input_bufferP22jpeg_decompress_struct
#[doc(alias = "fill_input_buffer(jpeg_decompress_struct *)")]
pub fn stub_11209c() -> ! {
    todo!("0x11209c fill_input_buffer(jpeg_decompress_struct *)")
}

// 0x112174 — __ZL15skip_input_dataP22jpeg_decompress_structl
#[doc(alias = "skip_input_data(jpeg_decompress_struct *,long)")]
pub fn stub_112174() -> ! {
    todo!("0x112174 skip_input_data(jpeg_decompress_struct *,long)")
}

// 0x1121c0 — __ZL16term_destinationP20jpeg_compress_struct
#[doc(alias = "term_destination(jpeg_compress_struct *)")]
pub fn stub_1121c0() -> ! {
    todo!("0x1121c0 term_destination(jpeg_compress_struct *)")
}

// 0x112238 — __ZL19empty_output_bufferP20jpeg_compress_struct
#[doc(alias = "empty_output_buffer(jpeg_compress_struct *)")]
pub fn stub_112238() -> ! {
    todo!("0x112238 empty_output_buffer(jpeg_compress_struct *)")
}

// 0x1122b8 — __ZL19jpeg_output_messageP18jpeg_common_struct
#[doc(alias = "jpeg_output_message(jpeg_common_struct *)")]
pub fn stub_1122b8() -> ! {
    todo!("0x1122b8 jpeg_output_message(jpeg_common_struct *)")
}

// 0x1122f0 — __ZL22jpeg_write_icc_profileP20jpeg_compress_structP8FIBITMAP
#[doc(alias = "jpeg_write_icc_profile(jpeg_compress_struct *,FIBITMAP *)")]
pub fn stub_1122f0() -> ! {
    todo!("0x1122f0 jpeg_write_icc_profile(jpeg_compress_struct *,FIBITMAP *)")
}

// 0x11240c — __ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3_
#[doc(alias = "Save(FreeImageIO *,FIBITMAP *,void *,int,int,void *)")]
pub fn stub_11240c() -> ! {
    todo!("0x11240c Save(FreeImageIO *,FIBITMAP *,void *,int,int,void *)")
}

// 0x112f64 — __ZL15jpeg_error_exitP18jpeg_common_struct
#[doc(alias = "jpeg_error_exit(jpeg_common_struct *)")]
pub fn stub_112f64() -> ! {
    todo!("0x112f64 jpeg_error_exit(jpeg_common_struct *)")
}

// 0x112fc0 — __Z22jpeg_read_iptc_profileP8FIBITMAPPKhj
#[doc(alias = "jpeg_read_iptc_profile(FIBITMAP *,unsigned char const*,unsigned int)")]
pub fn stub_112fc0() -> ! {
    todo!("0x112fc0 jpeg_read_iptc_profile(FIBITMAP *,unsigned char const*,unsigned int)")
}

// 0x112fd0 — __ZL4LoadP11FreeImageIOPviiS1_
#[doc(alias = "Load(FreeImageIO *,void *,int,int,void *)")]
pub fn stub_112fd0() -> ! {
    todo!("0x112fd0 Load(FreeImageIO *,void *,int,int,void *)")
}

// 0x114260 — __Z11INPLACESWAPIhEvRT_S1_
#[doc(alias = "void INPLACESWAP<unsigned char>(unsigned char &,unsigned char &)")]
pub fn stub_114260(result: *mut u8, a2: *mut u8) -> *mut u8 {
    // IDA 0x114260: triple-XOR swap through R9/R12, returns `result`:
    // xor = *r ^ *a (0x114268); *r = xor (0x11426c); reload *a (0x114270);
    // *a = xor ^ *a (0x114274..0x114278); reload *r (0x11427c);
    // *r = *a ^ *r (0x114280..0x114284); BX LR (0x114288).
    // BUG: result == a2 zeroes the byte (the reloads re-read the stored
    // 0); preserved 1:1 via the reload formulation below.
    // SAFETY: both pointers are readable/writable u8 per the caller.
    unsafe {
        let xor = *result ^ *a2;
        *result = xor;
        let cur_b = *a2;
        *a2 = xor ^ cur_b;
        let cur_r = *result;
        *result = (xor ^ cur_b) ^ cur_r;
        result
    }
}

// 0x11428c — __ZL10_FlushProcP14png_struct_def
#[doc(alias = "_FlushProc(png_struct_def *)")]
pub fn stub_11428c() {
    // IDA 0x11428c: single BX LR — libpng flush callback that flushes nothing.
}

// 0x114290 — __ZL15warning_handlerP14png_struct_defPKc
#[doc(alias = "warning_handler(png_struct_def *,char const*)")]
pub fn stub_114290(_png: *mut c_void, _msg: *const c_char) {
    // IDA 0x114290: single BX LR — libpng warning callback that ignores
    // the message (paired with error_handler at 0x1152a4, which throws).
}

// 0x114294 — __ZL6Formatv_0
#[doc(alias = "__ZL6Formatv_0")]
pub fn stub_114294() -> &'static str {
    // IDA 0x114294: LDR R0,="PNG" (0x114294..0x114298); BX LR (0x11429c).
    "PNG"
}

// 0x1142a4 — __ZL11Descriptionv_0
#[doc(alias = "__ZL11Descriptionv_0")]
pub fn stub_1142a4() -> &'static str {
    // IDA 0x1142a4: LDR R0,="Portable Network Graphics"
    // (0x1142a4..0x1142a8); BX LR (0x1142ac).
    "Portable Network Graphics"
}

// 0x1142b4 — __ZL9Extensionv_0
#[doc(alias = "__ZL9Extensionv_0")]
pub fn stub_1142b4() -> &'static str {
    // IDA 0x1142b4: LDR R0,="png" (0x1142b4..0x1142b8); BX LR (0x1142bc).
    "png"
}

// 0x1142c4 — __ZL7RegExprv_0
#[doc(alias = "__ZL7RegExprv_0")]
pub fn stub_1142c4() -> &'static str {
    // IDA 0x1142c4: LDR R0,="^.PNG\r" (0x1142c4..0x1142c8); BX LR (0x1142cc).
    "^.PNG\r"
}

// 0x1142d4 — __ZL8MimeTypev_0
#[doc(alias = "__ZL8MimeTypev_0")]
pub fn stub_1142d4() -> &'static str {
    // IDA 0x1142d4: LDR R0,="image/png" (0x1142d4..0x1142d8); BX LR (0x1142dc).
    "image/png"
}

// 0x1142e4 — __ZL19SupportsExportDepthi_0
#[doc(alias = "__ZL19SupportsExportDepthi_0")]
pub fn stub_1142e4(depth: i32) -> bool {
    // IDA 0x1142e4: {4,1} -> 1 (0x1142e4..0x1142ec); {24,8} -> 1
    // (0x1142f0..0x1142f8); 0x20 -> 1 else 0 (0x1142fc..0x114310).
    matches!(depth, 1 | 4 | 8 | 24 | 32)
}

// 0x114314 — __ZL18SupportsExportType15FREE_IMAGE_TYPE_0
#[doc(alias = "__ZL18SupportsExportType15FREE_IMAGE_TYPE_0")]
pub fn stub_114314(image_type: u32) -> bool {
    // IDA 0x114314: SUB R3,R0,#1; a1 == 9 -> v1 = (a1 > 9) = false -> true;
    // else v1 = (a1 - 1 > 1) unsigned so !v1 covers {1, 2}; tail a1 == 10.
    // Net true set: {1, 2, 9, 10} (FIT_UINT16/INT16/FLOAT/RGBAF).
    matches!(image_type, 1 | 2 | 9 | 10)
}

// 0x114338 — __ZL19SupportsICCProfilesv_0
#[doc(alias = "__ZL19SupportsICCProfilesv_0")]
pub fn stub_114338() -> i32 {
    // IDA 0x114338: MOV R0,#1 (0x114338); BX LR (0x11433c). Always TRUE.
    1
}

// 0x114340 — __Z7InitPNGP6Plugini
#[doc(alias = "InitPNG(Plugin *,int)")]
pub fn stub_114340() -> ! {
    todo!("0x114340 InitPNG(Plugin *,int)")
}

// 0x114414 — __ZL8ValidateP11FreeImageIOPv_0
#[doc(alias = "__ZL8ValidateP11FreeImageIOPv_0")]
pub fn stub_114414() -> ! {
    todo!("0x114414 __ZL8ValidateP11FreeImageIOPv_0")
}

// 0x1144a8 — __ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3__0
#[doc(alias = "__ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3__0")]
pub fn stub_1144a8() -> ! {
    todo!("0x1144a8 __ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3__0")
}

// 0x115258 — __ZL10_WriteProcP14png_struct_defPhm
#[doc(alias = "_WriteProc(png_struct_def *,unsigned char *,unsigned long)")]
pub fn stub_115258() -> ! {
    todo!("0x115258 _WriteProc(png_struct_def *,unsigned char *,unsigned long)")
}

// 0x1152a4 — __ZL13error_handlerP14png_struct_defPKc
#[doc(alias = "error_handler(png_struct_def *,char const*)")]
pub fn stub_1152a4(_png: *mut c_void, msg: *const c_char) -> ! {
    // IDA 0x1152a4: exception = __cxa_allocate_exception(4) (0x1152b4),
    // *exception = msg (0x1152c4),
    // __cxa_throw(exception, typeinfo for char const*, 0) (0x1152c8) —
    // throws the message across the C API. Rust cannot throw Itanium
    // exceptions across extern frames here; panic! is the host mapping.
    let mut buf = [0u8; 256];
    let mut len = 0;
    while len < buf.len() {
        // SAFETY: `msg` is a readable NUL-terminated string per the
        // png_error contract.
        let b = unsafe { *msg.add(len) as u8 };
        if b == 0 {
            break;
        }
        buf[len] = b;
        len += 1;
    }
    panic!(
        "png error: {}",
        core::str::from_utf8(&buf[..len]).unwrap_or("<non-utf8>")
    );
}

// 0x1152d0 — __ZL9_ReadProcP14png_struct_defPhm
#[doc(alias = "_ReadProc(png_struct_def *,unsigned char *,unsigned long)")]
pub fn stub_1152d0() -> ! {
    todo!("0x1152d0 _ReadProc(png_struct_def *,unsigned char *,unsigned long)")
}

// 0x11535c — __ZL4LoadP11FreeImageIOPviiS1__0
#[doc(alias = "__ZL4LoadP11FreeImageIOPviiS1__0")]
pub fn stub_11535c() -> ! {
    todo!("0x11535c __ZL4LoadP11FreeImageIOPviiS1__0")
}

// 0x11600c — __ZL13_tiffReadProcPvS_i
#[doc(alias = "_tiffReadProc(void *,void *,int)")]
pub fn stub_11600c(pair: *const TiffHandlePair, buf: *mut c_void, size: i32) -> i32 {
    // IDA 0x11600c: read_proc = io[+0] word, little-endian (four LDRB +
    // ORR, 0x116018..0x116030); decompile: return size *
    // read_proc(buf, size, 1, handle) (0x116050).
    // SAFETY: libtiff guarantees `pair`/`pair.io` readable and `buf`
    // writable for `size` bytes; the stored proc obeys the FreeImageIO
    // (buffer, size, count, handle) contract.
    unsafe {
        let pair = &*pair;
        let io = &*pair.io;
        let count = (io.read_proc)(buf, size as u32, 1, pair.handle);
        (size as u32).wrapping_mul(count) as i32
    }
}

// 0x116054 — __ZL14_tiffWriteProcPvS_i
#[doc(alias = "_tiffWriteProc(void *,void *,int)")]
pub fn stub_116054(pair: *const TiffHandlePair, buf: *mut c_void, size: i32) -> i32 {
    // IDA 0x116054: same shape as _tiffReadProc but with write_proc =
    // io[+4] word, little-endian (four LDRB + ORR, 0x116060..0x116078);
    // decompile: return size * write_proc(buf, size, 1, handle) (0x116098).
    // SAFETY: as for stub_11600c, with `buf` readable for `size` bytes.
    unsafe {
        let pair = &*pair;
        let io = &*pair.io;
        let count = (io.write_proc)(buf, size as u32, 1, pair.handle);
        (size as u32).wrapping_mul(count) as i32
    }
}

// 0x11609c — __ZL13_tiffSeekProcPvji
#[doc(alias = "_tiffSeekProc(void *,unsigned int,int)")]
pub fn stub_11609c(pair: *const TiffHandlePair, offset: u32, whence: i32) -> i32 {
    // IDA 0x11609c: handle = pair[1] (0x1160ac); seek_proc = io[+8]
    // word, little-endian (0x1160b0..0x1160c8); seek_proc(handle, offset,
    // whence) (0x1160cc); tell_proc = io[+12] word, little-endian
    // (0x1160d4..0x1160e4); decompile tail: return tell_proc(handle).
    // SAFETY: `pair`/`pair.io` readable; `handle` valid for both procs.
    unsafe {
        let pair = &*pair;
        let io = &*pair.io;
        (io.seek_proc)(pair.handle, offset, whence);
        (io.tell_proc)(pair.handle) as i32
    }
}

// 0x1160fc — __ZL14_tiffCloseProcPv
#[doc(alias = "_tiffCloseProc(void *)")]
pub fn stub_1160fc(_handle: *mut c_void) -> i32 {
    // IDA 0x1160fc: MOV R0,#0 (0x1160fc); BX LR (0x116100). Closing is a
    // no-op — the FreeImage handle is owned by the caller, not libtiff.
    0
}

// 0x116104 — __ZL13_tiffSizeProcPv
#[doc(alias = "_tiffSizeProc(void *)")]
pub fn stub_116104(pair: *const TiffHandlePair) -> i32 {
    // IDA 0x116104: pos = tell_proc(handle), io[+12] LE (0x116114..0x116134,
    // saved in R6); seek_proc(handle, 0, SEEK_END) — MOV R2,#2 (0x116158),
    // R1=#0 (0x116164), BLX (0x116168); end = tell_proc(handle)
    // (0x11618c..0x116190, saved in R5 at 0x1161ac);
    // seek_proc(handle, pos, SEEK_SET) — MOV R2,#0 (0x1161b4), R1=R6
    // (0x1161c0), BLX (0x1161c4); return end (0x1161c8..0x1161cc).
    // SAFETY: as for stub_11609c.
    unsafe {
        let pair = &*pair;
        let io = &*pair.io;
        let pos = (io.tell_proc)(pair.handle);
        (io.seek_proc)(pair.handle, 0, 2);
        let end = (io.tell_proc)(pair.handle);
        (io.seek_proc)(pair.handle, pos, 0);
        end as i32
    }
}

// 0x1161d0 — __ZL12_tiffMapProcPvPS_Pj
#[doc(alias = "_tiffMapProc(void *,void **,unsigned int *)")]
pub fn stub_1161d0(
    _handle: *mut c_void,
    _base: *mut *mut c_void,
    _size: *mut u32,
) -> i32 {
    // IDA 0x1161d0: MOV R0,#0 (0x1161d0); BX LR (0x1161d4). No
    // memory-mapped IO — returning 0 makes libtiff fall back to
    // read/write procs.
    0
}

// 0x1161d8 — __ZL14_tiffUnmapProcPvS_j
#[doc(alias = "_tiffUnmapProc(void *,void *,unsigned int)")]
pub fn stub_1161d8(_handle: *mut c_void, _base: *mut c_void, _size: u32) {
    // IDA 0x1161d8: single BX LR — unmap counterpart of the
    // always-zero map proc; nothing to release.
}

// 0x1161dc — __ZL19msdosWarningHandlerPKcS0_Pv
#[doc(alias = "msdosWarningHandler(char const*,char const*,void *)")]
pub fn stub_1161dc(_module: *const c_char, _fmt: *const c_char, _ap: *mut c_void) {
    // IDA 0x1161dc: single BX LR — TIFF warning handler that ignores
    // (module, fmt, ap).
}

// 0x1161e0 — __ZL17msdosErrorHandlerPKcS0_Pv
#[doc(alias = "msdosErrorHandler(char const*,char const*,void *)")]
pub fn stub_1161e0(_module: *const c_char, _fmt: *const c_char, _ap: *mut c_void) {
    // IDA 0x1161e0: single BX LR — TIFF error handler that ignores
    // (module, fmt, ap).
}

// 0x1161e4 — __ZL6Formatv_1
#[doc(alias = "__ZL6Formatv_1")]
pub fn stub_1161e4() -> &'static str {
    // IDA 0x1161e4: LDR R0,=aTiff (0x1161e4); ADD R0,PC (0x1161e8);
    // BX LR (0x1161ec).
    "TIFF"
}

// 0x1161f4 — __ZL11Descriptionv_1
#[doc(alias = "__ZL11Descriptionv_1")]
pub fn stub_1161f4() -> &'static str {
    // IDA 0x1161f4: LDR R0,=aTaggedImageFil (0x1161f4); ADD R0,PC
    // (0x1161f8); BX LR (0x1161fc).
    "Tagged Image File Format"
}

// 0x116204 — __ZL9Extensionv_1
#[doc(alias = "__ZL9Extensionv_1")]
pub fn stub_116204() -> &'static str {
    // IDA 0x116204: LDR R0,=aTifTiff (0x116204); ADD R0,PC (0x116208);
    // BX LR (0x11620c).
    "tif,tiff"
}

// 0x116214 — __ZL7RegExprv_1
#[doc(alias = "__ZL7RegExprv_1")]
pub fn stub_116214() -> &'static str {
    // IDA 0x116214: LDR R0,=aMiMiX01X01 (0x116214); ADD R0,PC (0x116218);
    // BX LR (0x11621c). Raw bytes at 0x10cfaf4 start
    // 5E 5B 4D 49 5D 5B 4D 49 5D 5B 5C 78 30 31 2A 5D 5B 5C … — the
    // `\x01` runs are literal backslash-x-0-1 text (all ASCII), so &str
    // holds the exact image bytes here (unlike the JPEG regex at
    // 0x111e98, which needs &[u8]).
    "^[MI][MI][\\x01*][\\x01*]"
}

// 0x116224 — __ZL8MimeTypev_1
#[doc(alias = "__ZL8MimeTypev_1")]
pub fn stub_116224() -> &'static str {
    // IDA 0x116224: LDR R0,=aImageTiff (0x116224); ADD R0,PC (0x116228);
    // BX LR (0x11622c).
    "image/tiff"
}

// 0x116234 — __ZL19SupportsExportDepthi_1
#[doc(alias = "__ZL19SupportsExportDepthi_1")]
pub fn stub_116234(depth: i32) -> bool {
    // IDA 0x116234: CMP R0,#4 (0x116234); CMPNE R0,#1 (0x116238);
    // EQ -> 1 (0x11623c -> 0x11625c); CMP R0,#0x18 (0x116240);
    // CMPNE R0,#8 (0x116244); EQ -> 1 (0x116248 -> 0x11625c);
    // CMP R0,#0x20 (0x11624c); EQ -> 1 else 0 (0x116250..0x116260).
    // Same {1, 4, 8, 24, 32} shape as the PNG twin at 0x1142e4.
    matches!(depth, 1 | 4 | 8 | 24 | 32)
}

// 0x116264 — __ZL18SupportsExportType15FREE_IMAGE_TYPE_1
#[doc(alias = "__ZL18SupportsExportType15FREE_IMAGE_TYPE_1")]
pub fn stub_116264(image_type: i32) -> bool {
    // IDA 0x116264: SUB R0,R0,#1 (0x116264); CMP R0,#0xA (0x116268);
    // HI -> 0 (0x11626c), LS -> 1 (0x116270); BX LR (0x116274).
    // (image_type - 1) <= 10 unsigned, i.e. image_type in 1..=11 —
    // every FreeImage type up to FIT_RGBAF exports to TIFF.
    matches!(image_type, 1..=11)
}

// 0x116278 — __ZL19SupportsICCProfilesv_1
#[doc(alias = "__ZL19SupportsICCProfilesv_1")]
pub fn stub_116278() -> i32 {
    // IDA 0x116278: MOV R0,#1 (0x116278); BX LR (0x11627c). Always TRUE.
    1
}

// 0x116280 — __Z8InitTIFFP6Plugini
#[doc(alias = "InitTIFF(Plugin *,int)")]
pub fn stub_116280(plugin: *mut TiffPluginView, format_id: i32) -> *mut TiffPluginView {
    // IDA 0x116280: s_format_id = a2 (0x116280..0x116288); then one STR
    // per vtable slot: +0x00 Format (0x11629c), +0x08 Extension (0x1162a0),
    // +0x04 Description (0x1162b4), +0x0C RegExpr (0x1162c4),
    // +0x14 Close (0x1162bc..0x1162c8), +0x10 Open (0x1162e0),
    // +0x20 Load (0x1162e4), +0x1C 0 (0x1162f0), +0x18 PageCount (0x1162f8),
    // +0x24 Save (0x116308), +0x28 Validate (0x11630c), +0x2C MimeType
    // (0x116314), +0x30 SupportsExportDepth (0x11632c), +0x34
    // SupportsExportType (0x116330), +0x38 SupportsICCProfiles (0x116334);
    // returns the plugin (BX LR, 0x116338).
    // BUG: Open/Close/PageCount/Load/Save still carry todo!() bodies, so
    // those wired slots panic with their EA if invoked before they land.
    // SAFETY: `plugin` is a writable Plugin per the caller.
    TIFF_S_FORMAT_ID.store(format_id, core::sync::atomic::Ordering::Relaxed);
    let p = unsafe { &mut *plugin };
    p.format = stub_1161e4 as *const c_void;
    p.description = stub_1161f4 as *const c_void;
    p.extension = stub_116204 as *const c_void;
    p.reg_expr = stub_116214 as *const c_void;
    p.open = stub_116f34 as *const c_void;
    p.close = stub_116e58 as *const c_void;
    p.page_count = stub_116e20 as *const c_void;
    p.reserved = core::ptr::null();
    p.load = stub_11855c as *const c_void;
    p.save = stub_116fe8 as *const c_void;
    p.validate = stub_116378 as *const c_void;
    p.mime_type = stub_116224 as *const c_void;
    p.supports_export_depth = stub_116234 as *const c_void;
    p.supports_export_type = stub_116264 as *const c_void;
    p.supports_icc_profiles = stub_116278 as *const c_void;
    plugin
}

// 0x116378 — __ZL8ValidateP11FreeImageIOPv_1
#[doc(alias = "__ZL8ValidateP11FreeImageIOPv_1")]
pub fn stub_116378(io: *const TiffIoHooks, handle: *mut c_void) -> bool {
    // IDA 0x116378: dst = "II*\0" (static C.189 = 49 49 2A 00;
    // 0x116388..0x11639c), want = "MM\0*" (static C.190 = 4D 4D 00 2A;
    // 0x1163a0..0x1163b0); sig zero-filled (static C.191_0 = 00 00 00 00;
    // 0x1163b4..0x1163c4); read_proc = io[+0] word, little-endian
    // (0x1163c8..0x1163dc); read_proc(sig, 1, 4, handle)
    // (0x1163e4..0x1163f4); memcmp(dst, sig, 4) == 0 -> true
    // (0x1163f8..0x116410), else memcmp(want, sig, 4) == 0
    // (0x116414..0x116428). TIFF magic in either byte order validates.
    // SAFETY: `io` readable; `handle` valid for the stored read_proc.
    let mut sig = [0u8; 4];
    unsafe {
        let io = &*io;
        (io.read_proc)(sig.as_mut_ptr() as *mut c_void, 1, 4, handle);
    }
    sig == *b"II*\0" || sig == *b"MM\0*"
}

// 0x116440 — __TIFFmemcmp
#[doc(alias = "__TIFFmemcmp")]
pub fn stub_116440(a1: *const c_void, a2: *const c_void, n: usize) -> i32 {
    // IDA 0x116440: BL _memcmp (0x11644c) — tail call to system memcmp.
    // SAFETY: both pointers are readable for `n` bytes per the caller.
    unsafe {
        let a = core::slice::from_raw_parts(a1 as *const u8, n);
        let b = core::slice::from_raw_parts(a2 as *const u8, n);
        for i in 0..n {
            let d = (a[i] as i32) - (b[i] as i32);
            if d != 0 {
                return d;
            }
        }
        0
    }
}

// 0x116450 — __TIFFmalloc
#[doc(alias = "__TIFFmalloc")]
pub fn stub_116450(size: usize) -> *mut c_void {
    // IDA 0x116450: BL _malloc (0x11645c) — tail call to system malloc.
    // BUG: no libc link in this crate, so this hosts malloc on
    // std::alloc with a 16-byte size-prefix header (base stays
    // 16-aligned, and free recovers the layout from the header).
    // Pointer identity therefore differs from system malloc: blocks
    // must be released with stub_116460, not foreign free.
    const HDR: usize = 16;
    const ALIGN: usize = 16;
    let total = size.saturating_add(HDR).max(1);
    let Ok(layout) = core::alloc::Layout::from_size_align(total, ALIGN) else {
        return core::ptr::null_mut();
    };
    let base = unsafe { std::alloc::alloc(layout) };
    if base.is_null() {
        return core::ptr::null_mut();
    }
    unsafe {
        (base as *mut usize).write(size);
        base.add(HDR) as *mut c_void
    }
}

// 0x116460 — __TIFFfree
#[doc(alias = "__TIFFfree")]
pub fn stub_116460(ptr: *mut c_void) {
    // IDA 0x116460: BL _free (0x116468) — tail call to system free.
    // BUG: paired with the prefixed stub_116460 allocator above, not
    // system free (see stub_116450). free(NULL) stays a no-op, as in C.
    const HDR: usize = 16;
    const ALIGN: usize = 16;
    if ptr.is_null() {
        return;
    }
    unsafe {
        let base = (ptr as *mut u8).sub(HDR);
        let size = (base as *const usize).read();
        let layout =
            core::alloc::Layout::from_size_align(size.saturating_add(HDR).max(1), ALIGN)
                .expect("TIFFfree layout");
        std::alloc::dealloc(base, layout);
    }
}

// 0x116470 — __TIFFmemcpy
#[doc(alias = "__TIFFmemcpy")]
pub fn stub_116470(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void {
    // IDA 0x116470: BL _memcpy (0x11647c) — tail call to system memcpy.
    // SAFETY: `src` readable and `dst` writable for `n` bytes; ranges
    // must not overlap, exactly as with C memcpy.
    unsafe {
        core::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, n);
        dst
    }
}

// 0x116480 — __TIFFmemset
#[doc(alias = "__TIFFmemset")]
pub fn stub_116480(dst: *mut c_void, c: i32, n: usize) -> *mut c_void {
    // IDA 0x116480: BL _memset (0x11648c) — tail call to system memset.
    // SAFETY: `dst` writable for `n` bytes per the caller.
    unsafe {
        core::ptr::write_bytes(dst as *mut u8, c as u8, n);
        dst
    }
}

// 0x116490 — __ZL11ReadPaletteP4tiffttP8FIBITMAP
#[doc(alias = "ReadPalette(tiff *,unsigned short,unsigned short,FIBITMAP *)")]
pub fn stub_116490() -> ! {
    todo!("0x116490 ReadPalette(tiff *,unsigned short,unsigned short,FIBITMAP *)")
}

// 0x116ba4 — __ZL15CreateImageType15FREE_IMAGE_TYPEiitt
#[doc(alias = "CreateImageType(FREE_IMAGE_TYPE,int,int,unsigned short,unsigned short)")]
pub fn stub_116ba4() -> ! {
    todo!("0x116ba4 CreateImageType(FREE_IMAGE_TYPE,int,int,unsigned short,unsigned short)")
}

// 0x116cd0 — __ZL14ReadResolutionP4tiffP8FIBITMAP
#[doc(alias = "ReadResolution(tiff *,FIBITMAP *)")]
pub fn stub_116cd0() -> ! {
    todo!("0x116cd0 ReadResolution(tiff *,FIBITMAP *)")
}

// 0x116e20 — __ZL9PageCountP11FreeImageIOPvS1_
#[doc(alias = "PageCount(FreeImageIO *,void *,void *)")]
pub fn stub_116e20() -> ! {
    todo!("0x116e20 PageCount(FreeImageIO *,void *,void *)")
}

// 0x116e58 — __ZL5CloseP11FreeImageIOPvS1_
#[doc(alias = "Close(FreeImageIO *,void *,void *)")]
pub fn stub_116e58() -> ! {
    todo!("0x116e58 Close(FreeImageIO *,void *,void *)")
}

// 0x116e7c — __TIFFrealloc
#[doc(alias = "__TIFFrealloc")]
pub fn stub_116e7c() -> ! {
    todo!("0x116e7c __TIFFrealloc")
}

// 0x116e8c — __Z10TIFFFdOpenPvPKcS1_
#[doc(alias = "TIFFFdOpen(void *,char const*,char const*)")]
pub fn stub_116e8c() -> ! {
    todo!("0x116e8c TIFFFdOpen(void *,char const*,char const*)")
}

// 0x116f34 — __ZL4OpenP11FreeImageIOPvi
#[doc(alias = "Open(FreeImageIO *,void *,int)")]
pub fn stub_116f34() -> ! {
    todo!("0x116f34 Open(FreeImageIO *,void *,int)")
}

// 0x116fe8 — __ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3__1
#[doc(alias = "__ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3__1")]
pub fn stub_116fe8() -> ! {
    todo!("0x116fe8 __ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3__1")
}

// 0x11855c — __ZL4LoadP11FreeImageIOPviiS1__1
#[doc(alias = "__ZL4LoadP11FreeImageIOPviiS1__1")]
pub fn stub_11855c() -> ! {
    todo!("0x11855c __ZL4LoadP11FreeImageIOPviiS1__1")
}

// 0x11c0f8 — __Z24tiff_ConvertLineXYZToRGBPhS_di
#[doc(alias = "tiff_ConvertLineXYZToRGB(unsigned char *,unsigned char *,double,int)")]
pub fn stub_11c0f8() -> ! {
    todo!("0x11c0f8 tiff_ConvertLineXYZToRGB(unsigned char *,unsigned char *,double,int)")
}

// 0x11c268 — __Z24tiff_ConvertLineRGBToXYZPhS_i
#[doc(alias = "tiff_ConvertLineRGBToXYZ(unsigned char *,unsigned char *,int)")]
pub fn stub_11c268() -> ! {
    todo!("0x11c268 tiff_ConvertLineRGBToXYZ(unsigned char *,unsigned char *,int)")
}

// 0x11c47c — __ZL14HorizontalSkewP8FIBITMAPS0_iidPKv
#[doc(alias = "HorizontalSkew(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")]
pub fn stub_11c47c() -> ! {
    todo!("0x11c47c HorizontalSkew(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")
}

// 0x11c57c — __ZL9RotateAnyP8FIBITMAPdPKv
#[doc(alias = "RotateAny(FIBITMAP *,double,void const*)")]
pub fn stub_11c57c() -> ! {
    todo!("0x11c57c RotateAny(FIBITMAP *,double,void const*)")
}

// 0x11e5e8 — _FreeImage_Rotate
#[doc(alias = "_FreeImage_Rotate")]
pub fn stub_11e5e8() -> ! {
    todo!("0x11e5e8 _FreeImage_Rotate")
}

// 0x11e990 — __Z13VerticalSkewTIfEvP8FIBITMAPS1_iidPKv
#[doc(alias = "void VerticalSkewT<float>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")]
pub fn stub_11e990() -> ! {
    todo!("0x11e990 void VerticalSkewT<float>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")
}

// 0x11f678 — __Z13VerticalSkewTItEvP8FIBITMAPS1_iidPKv
#[doc(alias = "void VerticalSkewT<unsigned short>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")]
pub fn stub_11f678() -> ! {
    todo!("0x11f678 void VerticalSkewT<unsigned short>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")
}

// 0x120330 — __Z13VerticalSkewTIhEvP8FIBITMAPS1_iidPKv
#[doc(alias = "void VerticalSkewT<unsigned char>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")]
pub fn stub_120330() -> ! {
    todo!("0x120330 void VerticalSkewT<unsigned char>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")
}

// 0x120eb8 — __Z15HorizontalSkewTIfEvP8FIBITMAPS1_iidPKv
#[doc(alias = "void HorizontalSkewT<float>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")]
pub fn stub_120eb8() -> ! {
    todo!("0x120eb8 void HorizontalSkewT<float>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")
}

// 0x121734 — __Z15HorizontalSkewTItEvP8FIBITMAPS1_iidPKv
#[doc(alias = "void HorizontalSkewT<unsigned short>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")]
pub fn stub_121734() -> ! {
    todo!("0x121734 void HorizontalSkewT<unsigned short>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")
}

// 0x121f84 — __Z15HorizontalSkewTIhEvP8FIBITMAPS1_iidPKv
#[doc(alias = "void HorizontalSkewT<unsigned char>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")]
pub fn stub_121f84() -> ! {
    todo!("0x121f84 void HorizontalSkewT<unsigned char>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")
}

// 0x12278c — _FreeImage_FlipVertical
#[doc(alias = "_FreeImage_FlipVertical")]
pub fn stub_12278c() -> ! {
    todo!("0x12278c _FreeImage_FlipVertical")
}

// 0x122a58 — _FreeImage_FlipHorizontal
#[doc(alias = "_FreeImage_FlipHorizontal")]
pub fn stub_122a58() -> ! {
    todo!("0x122a58 _FreeImage_FlipHorizontal")
}

// 0x123284 — _jpeg_suppress_tables
#[doc(alias = "_jpeg_suppress_tables")]
pub fn stub_123284() -> ! {
    todo!("0x123284 _jpeg_suppress_tables")
}

// 0x12331c — _jpeg_write_marker
#[doc(alias = "_jpeg_write_marker")]
pub fn stub_12331c() -> ! {
    todo!("0x12331c _jpeg_write_marker")
}

// 0x1234bc — _jpeg_write_tables
#[doc(alias = "_jpeg_write_tables")]
pub fn stub_1234bc() -> ! {
    todo!("0x1234bc _jpeg_write_tables")
}

// 0x123544 — _jpeg_finish_compress
#[doc(alias = "_jpeg_finish_compress")]
pub fn stub_123544() -> ! {
    todo!("0x123544 _jpeg_finish_compress")
}

// 0x123688 — _jpeg_destroy_compress
#[doc(alias = "_jpeg_destroy_compress")]
pub fn stub_123688() -> ! {
    todo!("0x123688 _jpeg_destroy_compress")
}

// 0x123698 — _jpeg_CreateCompress
#[doc(alias = "_jpeg_CreateCompress")]
pub fn stub_123698() -> ! {
    todo!("0x123698 _jpeg_CreateCompress")
}

// 0x1237c0 — _jpeg_write_scanlines
#[doc(alias = "_jpeg_write_scanlines")]
pub fn stub_1237c0() -> ! {
    todo!("0x1237c0 _jpeg_write_scanlines")
}

// 0x1238cc — _jpeg_write_raw_data
#[doc(alias = "_jpeg_write_raw_data")]
pub fn stub_1238cc() -> ! {
    todo!("0x1238cc _jpeg_write_raw_data")
}

// 0x1239f0 — _jpeg_start_compress
#[doc(alias = "_jpeg_start_compress")]
pub fn stub_1239f0() -> ! {
    todo!("0x1239f0 _jpeg_start_compress")
}

// 0x123a9c — _emit_byte
#[doc(alias = "_emit_byte")]
pub fn stub_123a9c() -> ! {
    todo!("0x123a9c _emit_byte")
}

// 0x123b00 — _finish_pass
#[doc(alias = "_finish_pass")]
pub fn stub_123b00() -> ! {
    todo!("0x123b00 _finish_pass")
}

// 0x123d40 — _arith_encode
#[doc(alias = "_arith_encode")]
pub fn stub_123d40() -> ! {
    todo!("0x123d40 _arith_encode")
}

// 0x123f98 — _jinit_arith_encoder
#[doc(alias = "_jinit_arith_encoder")]
pub fn stub_123f98() -> ! {
    todo!("0x123f98 _jinit_arith_encoder")
}

// 0x124064 — _emit_restart
#[doc(alias = "_emit_restart")]
pub fn stub_124064() -> ! {
    todo!("0x124064 _emit_restart")
}

// 0x124178 — _encode_mcu
#[doc(alias = "_encode_mcu")]
pub fn stub_124178() -> ! {
    todo!("0x124178 _encode_mcu")
}

// 0x124748 — _encode_mcu_AC_refine
#[doc(alias = "_encode_mcu_AC_refine")]
pub fn stub_124748() -> ! {
    todo!("0x124748 _encode_mcu_AC_refine")
}

// 0x124c5c — _encode_mcu_DC_refine
#[doc(alias = "_encode_mcu_DC_refine")]
pub fn stub_124c5c() -> ! {
    todo!("0x124c5c _encode_mcu_DC_refine")
}

// 0x124d08 — _encode_mcu_AC_first
#[doc(alias = "_encode_mcu_AC_first")]
pub fn stub_124d08() -> ! {
    todo!("0x124d08 _encode_mcu_AC_first")
}

// 0x125150 — _encode_mcu_DC_first
#[doc(alias = "_encode_mcu_DC_first")]
pub fn stub_125150() -> ! {
    todo!("0x125150 _encode_mcu_DC_first")
}
