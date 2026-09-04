//! audio generated_09 — next 120 stubs EA-sorted, from ida/export.json
//! Filter: FMOD|Sound|Audio (2544 distinct) — filler from remaining unclaimed EA (workspace EA-sorted asc, skip existing)
//! Batch: 120 stubs | skeleton batch | range 0x82b700..0x125150 EA-sorted asc, skip existing, rbx_core::SharedPtr not boost
//! Generated: 2026-08-31

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use parking_lot::Mutex;
use std::sync::Arc;
use std::sync::LazyLock;
use rbx_core::signal::Signal;

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
/// `s_format_id` globals written by InitJPEG / InitPNG (IDA 0x111ef0 /
/// 0x11434c: STR R1,[PC,R9] to `s_format_id` / `s_format_id_0`), mirroring
/// the TIFF_S_FORMAT_ID above.
static JPEG_S_FORMAT_ID: core::sync::atomic::AtomicI32 =
    core::sync::atomic::AtomicI32::new(0);
static PNG_S_FORMAT_ID: core::sync::atomic::AtomicI32 =
    core::sync::atomic::AtomicI32::new(0);
/// JPEG/PNG `Plugin` vtable filled by InitJPEG (IDA 0x111efc..0x111f80)
/// and InitPNG (IDA 0x114358..0x1143dc): Format at +0x00, Description at
/// +0x04, Extension at +0x08, RegExpr at +0x0C, four reserved zero slots at
/// +0x10..+0x1C, then Load (+0x20), Save (+0x24), Validate (+0x28),
/// MimeType (+0x2C), SupportsExportDepth (+0x30), SupportsExportType
/// (+0x34), SupportsICCProfiles (+0x38).
/// BUG: fields are pointer-sized, so on a 64-bit host this struct is wider
/// than the 32-bit target's 60 bytes; the slot order — not the byte
/// offsets — is what this port preserves (as with TiffPluginView above).
#[repr(C)]
pub struct JpegPluginView {
    pub format: *const c_void,
    pub description: *const c_void,
    pub extension: *const c_void,
    pub reg_expr: *const c_void,
    pub reserved0: *const c_void,
    pub reserved1: *const c_void,
    pub reserved2: *const c_void,
    pub reserved3: *const c_void,
    pub load: *const c_void,
    pub save: *const c_void,
    pub validate: *const c_void,
    pub mime_type: *const c_void,
    pub supports_export_depth: *const c_void,
    pub supports_export_type: *const c_void,
    pub supports_icc_profiles: *const c_void,
}
/// libjpeg error-manager prefix (IDA 0x1122c4..0x1122d0,
/// 0x112f6c..0x112f88): five AAPCS (`extern "C"`) slots, then the message
/// code word at +20 (written by fill_input_buffer at 0x112120) and the
/// following parameter word at +24 (compared against 13 by jpeg_error_exit
/// at 0x112f80..0x112f88).
#[repr(C)]
pub struct JpegErrView {
    pub error_exit: unsafe extern "C" fn(*mut c_void),
    pub emit_message: unsafe extern "C" fn(*mut c_void, i32),
    pub output_message: unsafe extern "C" fn(*mut c_void),
    pub format_message: unsafe extern "C" fn(*mut c_void, *mut c_char),
    pub reset_error_mgr: unsafe extern "C" fn(*mut c_void),
    pub msg_code: i32,
    pub msg_ext24: i32,
}
/// libjpeg source manager behind the FreeImage callbacks (IDA 0x1120a4,
/// 0x112184): next/bytes at +0/+4, opaque handle at +28, FreeImageIO table
/// at +32 (read_proc word at +0), bounce buffer at +36, start-of-file flag
/// at +40.
#[repr(C)]
pub struct JpegSrcMgr {
    pub next_input_byte: *const u8,
    pub bytes_in_buffer: u32,
    pub _pad8: [u8; 20],
    pub handle: *mut c_void,
    pub io: *const TiffIoHooks,
    pub buffer: *mut u8,
    pub start_of_file: u8,
}
/// libjpeg destination manager (IDA 0x1121c8, 0x112240): next/free at
/// +0/+4, opaque handle at +20, FreeImageIO table at +24 (write_proc word
/// at +4), bounce buffer at +28.
#[repr(C)]
pub struct JpegDstMgr {
    pub next_output_byte: *mut u8,
    pub free_in_buffer: u32,
    pub _pad8: [u8; 12],
    pub handle: *mut c_void,
    pub io: *const TiffIoHooks,
    pub buffer: *mut u8,
}
/// External FreeImage/libjpeg/libpng/libtiff entry points called by this
/// batch. Declared, not defined: the host image libraries provide them,
/// exactly as the original binary imports them (BL _FreeImage_Allocate at
/// 0x116c8c, BL _TIFFReadDirectory at 0x116e40, BL _jpeg_destroy at
/// 0x112f90, BL _png_get_io_ptr at 0x115268, ...).
extern "C" {
    fn FreeImage_OutputMessageProc(format_id: i32, msg: *const c_char) -> i32;
    fn read_iptc_profile(dib: *mut c_void, data: *const u8, len: u32) -> i32;
    fn png_get_io_ptr(png: *mut c_void) -> *mut c_void;
    fn FreeImage_Allocate(
        width: i32,
        height: i32,
        bpp: i32,
        red_mask: u32,
        green_mask: u32,
        blue_mask: u32,
    ) -> *mut c_void;
    fn FreeImage_AllocateT(
        image_type: i32,
        width: i32,
        height: i32,
        bpp: i32,
        red_mask: u32,
        green_mask: u32,
        blue_mask: u32,
    ) -> *mut c_void;
    fn TIFFGetField(tif: *mut c_void, tag: u32, ...) -> i32;
    /// BUG: upstream FreeImage declares the SetDotsPerMeter pair as void;
    /// the ports below return R0 residue 1:1 with the decompile tails
    /// (`return FreeImage_SetDotsPerMeterY(...)` at 0x116df4/0x116df8).
    fn FreeImage_SetDotsPerMeterX(dib: *mut c_void, res: u32) -> i32;
    fn FreeImage_SetDotsPerMeterY(dib: *mut c_void, res: u32) -> i32;
    fn TIFFReadDirectory(tif: *mut c_void) -> i32;
    fn TIFFClose(tif: *mut c_void);
    /// BUG: upstream libjpeg declares jpeg_destroy as void; the wrappers
    /// below return R0 residue 1:1 with the decompile (`return
    /// jpeg_destroy(a1)` at 0x123694), so the value is whatever the callee
    /// left behind.
    fn jpeg_destroy(cinfo: *mut c_void) -> i32;
    /// BUG: same R0-residue note as jpeg_destroy: upstream
    /// jinit_memory_mgr returns void.
    fn jinit_memory_mgr(cinfo: *mut c_void) -> *mut c_void;
    fn XTIFFInitialize();
    fn TIFFClientOpen(
        name: *const c_char,
        mode: *const c_char,
        clientdata: *mut c_void,
        read_proc: *mut c_void,
        write_proc: *mut c_void,
        seek_proc: *mut c_void,
        close_proc: *mut c_void,
        size_proc: *mut c_void,
        map_proc: *mut c_void,
        unmap_proc: *mut c_void,
    ) -> *mut c_void;
}

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
pub fn stub_b76c(sig: &Signal<u32>, desc: u32) {
    // IDA 0xb76c (`signal_with_args<1, void(const PropertyDescriptor *)>`
    // `operator()`, decompiled 0xb76c..0xb81c): null-signal check (0xb79c,
    // host: an empty slot list); FastLog "Signal with 1 arg executed" when
    // FLog::SignalPrints (0xb7ce..0xb7e0); next() loop firing each connected
    // slot with the descriptor (0xb7e6..0xb80a, slot-fun validity bit at +12);
    // intrusive_ptr_release of the iterator (0xb80c..0xb812, host: Arc drop).
    // was: boost::signals -> rbx_core::Signal; firing the host slot list
    // with the descriptor id is the call itself (cf. generated_18 stub_b76c).
    sig.fire(desc);
}

/// rbx::signals::slot_exception_handler (IDA 0xf6dc): the process-wide slot
/// error hook consulted by signal::on_error. `parking_lot::Mutex` stands in
/// for the static function-local guard; None is the null boost::function.
static SLOT_EXCEPTION_HANDLER: LazyLock<Mutex<Option<Arc<dyn Fn(String) + Send + Sync>>>> =
    LazyLock::new(|| Mutex::new(None));

/// Host accessor for the slot exception hook (IDA 0xf6dc).
pub fn slot_exception_handler() -> &'static Mutex<Option<Arc<dyn Fn(String) + Send + Sync>>> {
    &*SLOT_EXCEPTION_HANDLER
}

// 0xf574 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4nextERN5boost13intrusive_ptrINS8_4slotEEE
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot> &)")]
pub fn stub_f574(
    slots: &[Arc<dyn Fn(u32) + Send + Sync>],
    cursor: &mut usize,
) -> Option<Arc<dyn Fn(u32) + Send + Sync>> {
    // IDA 0xf574 (`signal::next`, decompiled 0xf574..0xf64c+): add_ref the
    // incoming iterator (0xf5c4..0xf5ce, host: Arc clone below); call_once
    // static-mutex init (0xf5ee) + lock (0xf5f8..0xf608, host: the
    // parking_lot::Mutex inside Signal — the caller holds the locked
    // snapshot this slice borrows); intrusive_ptr<slot>::operator= advances
    // the iterator (0xf61c..0xf636); unlock when locked (0xf638..0xf640);
    // release the old ref (0xf646.., host: Arc drop); yields the next live
    // slot or null. Host: bump the cursor over the snapshot, cloning the Arc
    // (the add_ref) and letting the previous cursor value drop (the release).
    let slot = slots.get(*cursor)?.clone();
    *cursor += 1;
    Some(slot)
}

// 0xf6dc — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE8on_errorERSt9exception
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::on_error(std::exception &)")]
pub fn stub_f6dc() -> Option<Arc<dyn Fn(String) + Send + Sync>> {
    // IDA 0xf6dc (`signal::on_error`, decompiled 0xf6dc..0xf702): result =
    // &slot_exception_handler (0xf6f0); a set handler normalizes through the
    // nonnull dummy (0xf6f2..0xf6f8); non-null -> return the stored function
    // (0xf6fe), else the null slot (0xf702). Host: clone the handler Arc.
    slot_exception_handler().lock().clone()
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
pub fn stub_111ee4(plugin: *mut JpegPluginView, format_id: i32) -> *mut JpegPluginView {
    // IDA 0x111ee4: s_format_id = a2 (0x111ef0); *result = Format (0x111efc);
    // result[2] = Extension (0x111f10); result[1] = Description (0x111f20);
    // result[8] = Load (0x111f24); result[4..7] = 0 (0x111f30..0x111f3c);
    // result[3] = RegExpr (0x111f44); result[9] = Save (0x111f54);
    // result[10] = Validate (0x111f58); result[11] = MimeType (0x111f60);
    // result[12] = SupportsExportDepth (0x111f78);
    // result[13] = SupportsExportType (0x111f7c);
    // result[14] = SupportsICCProfiles (0x111f80); returns plugin (0x111f84).
    // BUG: Format lives in generated_08 (0x111e68, still todo!) and
    // Load/Save (0x112fd0/0x11240c below) still carry todo!() bodies, so
    // invoking those wired slots panics until they land; only addresses are
    // wired here, mirroring stub_116280.
    // SAFETY: `plugin` is a writable Plugin per the caller.
    JPEG_S_FORMAT_ID.store(format_id, core::sync::atomic::Ordering::Relaxed);
    let p = unsafe { &mut *plugin };
    p.format = super::generated_08::stub_111e68 as *const c_void;
    p.description = stub_111e78 as *const c_void;
    p.extension = stub_111e88 as *const c_void;
    p.reg_expr = stub_111e98 as *const c_void;
    p.reserved0 = core::ptr::null();
    p.reserved1 = core::ptr::null();
    p.reserved2 = core::ptr::null();
    p.reserved3 = core::ptr::null();
    p.load = stub_112fd0 as *const c_void;
    p.save = stub_11240c as *const c_void;
    p.validate = stub_111fb8 as *const c_void;
    p.mime_type = stub_111ea8 as *const c_void;
    p.supports_export_depth = stub_111eb8 as *const c_void;
    p.supports_export_type = stub_111ecc as *const c_void;
    p.supports_icc_profiles = stub_111edc as *const c_void;
    plugin
}

// 0x111fb8 — __ZL8ValidateP11FreeImageIOPv
#[doc(alias = "Validate(FreeImageIO *,void *)")]
pub fn stub_111fb8(io: *const TiffIoHooks, handle: *mut c_void) -> bool {
    // IDA 0x111fb8: __dst = C.191 (0xf759f1 = FF D8, via 0x111fdc), v5 =
    // C.192 (00 00, via 0x111ff0); read_proc = io[+0] word, little-endian
    // (0x111ff8..0x11200c); read_proc(v5, 1, 2, handle) (0x112020);
    // return memcmp(__dst, v5, 2) == 0 (0x112040). The v5 init bytes are
    // overwritten by the read, so only the FF D8 expectation is load-bearing.
    // SAFETY: `io` readable; `handle` valid for the stored read_proc.
    let mut sig = [0u8; 2];
    unsafe {
        let io = &*io;
        (io.read_proc)(sig.as_mut_ptr() as *mut c_void, 1, 2, handle);
    }
    sig == [0xFF, 0xD8]
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
pub fn stub_11209c(cinfo: *mut c_void) -> i32 {
    // IDA 0x11209c: src = cinfo[6] (0x1120a4); n = read_proc(buf, 1, 4096,
    // handle) via the LE io[+0] word (0x1120ac..0x1120dc); if n == 0 and
    // start_of_file is still set: throw J_MESSAGE_CODE 43
    // (0x1120ec..0x112114); else warn: err.msg_code = 123 (0x112120),
    // emit_message(cinfo, -1) (0x112138), fake n = 2 with buffer = FF D9
    // (0x112140..0x112150); bytes_in_buffer = n (0x112158),
    // next_input_byte = buffer (0x112164), start_of_file = 0 (0x112168);
    // return 1 (0x11216c). Throws cross the C API; panic! is the host
    // mapping (as in stub_1152a4).
    // SAFETY: `cinfo` is a readable/writable jpeg_decompress_struct whose
    // src slot (+24) points at a JpegSrcMgr, per the caller.
    unsafe {
        let src = &mut *(*(cinfo as *mut *mut JpegSrcMgr).add(6));
        let io = &*src.io;
        let mut n = (io.read_proc)(src.buffer as *mut c_void, 1, 4096, src.handle);
        if n == 0 {
            if src.start_of_file != 0 {
                panic!("0x11209c fill_input_buffer: J_MESSAGE_CODE 43");
            }
            let err = *(cinfo as *mut *mut JpegErrView);
            (*err).msg_code = 123;
            ((*err).emit_message)(cinfo, -1);
            n = 2;
            *src.buffer = 0xFF;
            *src.buffer.add(1) = 0xD9;
        }
        src.bytes_in_buffer = n;
        src.next_input_byte = src.buffer as *const u8;
        src.start_of_file = 0;
        1
    }
}

// 0x112174 — __ZL15skip_input_dataP22jpeg_decompress_structl
#[doc(alias = "skip_input_data(jpeg_decompress_struct *,long)")]
pub fn stub_112174(cinfo: *mut c_void, mut skip: i32) -> *mut c_void {
    // IDA 0x112174: skip <= 0 -> return cinfo (0x112188..0x11218c); else
    // while skip > bytes_in_buffer: skip -= avail (0x112194),
    // fill_input_buffer(cinfo) (0x112198); then next += skip, bytes -= skip
    // (0x1121a8..0x1121b8); return the pre-advance next pointer
    // (0x1121a8, via 0x11218c).
    // SAFETY: as for stub_11209c.
    if skip <= 0 {
        return cinfo;
    }
    unsafe {
        let src = *(cinfo as *mut *mut JpegSrcMgr).add(6);
        loop {
            let avail = (*src).bytes_in_buffer as i32;
            if skip <= avail {
                break;
            }
            skip -= avail;
            stub_11209c(cinfo);
        }
        let next = (*src).next_input_byte as *mut u8;
        (*src).bytes_in_buffer = ((*src).bytes_in_buffer as i32 - skip) as u32;
        (*src).next_input_byte = next.add(skip as usize);
        next as *mut c_void
    }
}

// 0x1121c0 — __ZL16term_destinationP20jpeg_compress_struct
#[doc(alias = "term_destination(jpeg_compress_struct *)")]
pub fn stub_1121c0(cinfo: *mut c_void) -> i32 {
    // IDA 0x1121c0: dest = cinfo[6] (0x1121c8); free = dest.free (0x1121cc);
    // pending = 4096 - free (0x1121d0); free == 4096 -> return cinfo
    // (0x1121d4); else n = write_proc(buf, 1, pending, handle) via the LE
    // io[+4] word (0x1121d8..0x112208); n != pending -> throw
    // J_MESSAGE_CODE 38 (0x112210..0x112230); return n. The throw crosses
    // the C API; panic! is the host mapping (as in stub_1152a4).
    // BUG: the early return carries the pointer as i32 (32-bit target), so
    // it truncates on a 64-bit host.
    // SAFETY: `cinfo` holds a JpegDstMgr at its dest slot (+24).
    unsafe {
        let dest = &mut *(*(cinfo as *mut *mut JpegDstMgr).add(6));
        let free = dest.free_in_buffer;
        if free == 4096 {
            return cinfo as u32 as i32;
        }
        let pending = 4096 - free;
        let io = &*dest.io;
        let n = (io.write_proc)(dest.buffer as *mut c_void, 1, pending, dest.handle);
        if n != pending {
            panic!("0x1121c0 term_destination: J_MESSAGE_CODE 38");
        }
        n as i32
    }
}

// 0x112238 — __ZL19empty_output_bufferP20jpeg_compress_struct
#[doc(alias = "empty_output_buffer(jpeg_compress_struct *)")]
pub fn stub_112238(cinfo: *mut c_void) -> i32 {
    // IDA 0x112238: dest = cinfo[6] (0x112240); n = write_proc(buf, 1,
    // 4096, handle) via the LE io[+4] word (0x112244..0x112274);
    // n != 4096 -> throw J_MESSAGE_CODE 38 (0x112278..0x11229c);
    // free = 4096 (0x1122a4), next = buffer (0x1122ac); return 1 (0x1122b0).
    // SAFETY: as for stub_1121c0.
    unsafe {
        let dest = &mut *(*(cinfo as *mut *mut JpegDstMgr).add(6));
        let io = &*dest.io;
        let n = (io.write_proc)(dest.buffer as *mut c_void, 1, 4096, dest.handle);
        if n != 4096 {
            panic!("0x112238 empty_output_buffer: J_MESSAGE_CODE 38");
        }
        dest.free_in_buffer = 4096;
        dest.next_output_byte = dest.buffer;
        1
    }
}

// 0x1122b8 — __ZL19jpeg_output_messageP18jpeg_common_struct
#[doc(alias = "jpeg_output_message(jpeg_common_struct *)")]
pub fn stub_1122b8(cinfo: *mut c_void) -> i32 {
    // IDA 0x1122b8: err = *cinfo (0x1122c4); err.format_message(cinfo, buf)
    // via the +12 slot (0x1122c8..0x1122d0) into a 200-byte stack buffer
    // (0x1122c0); return FreeImage_OutputMessageProc(s_format_id, buf)
    // (0x1122d4..0x1122e8).
    // SAFETY: `cinfo` is a readable jpeg_common_struct with an err manager.
    unsafe {
        let err = *(cinfo as *mut *mut JpegErrView);
        let mut buf = [0u8; 200];
        ((*err).format_message)(cinfo, buf.as_mut_ptr() as *mut c_char);
        let id = JPEG_S_FORMAT_ID.load(core::sync::atomic::Ordering::Relaxed);
        FreeImage_OutputMessageProc(id, buf.as_ptr() as *const c_char)
    }
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
pub fn stub_112f64(cinfo: *mut c_void) -> *mut c_void {
    // IDA 0x112f64: err = *cinfo (0x112f6c); err.output_message(cinfo) via
    // the +8 slot (0x112f74..0x112f78); result = err (0x112f7c); if the +24
    // word != 13 (0x112f80..0x112f88): jpeg_destroy(cinfo) (0x112f90),
    // throw (int)s_format_id (0x112f98..0x112fb4). The int throw crosses
    // the C API; panic! carrying the format id is the host mapping (as in
    // stub_1152a4). Returns the err pointer, which IDA spells int (see the
    // stub_1121c0 BUG note).
    // SAFETY: as for stub_1122b8.
    unsafe {
        let err = *(cinfo as *mut *mut JpegErrView);
        ((*err).output_message)(cinfo);
        if (*err).msg_ext24 != 13 {
            jpeg_destroy(cinfo);
            let id = JPEG_S_FORMAT_ID.load(core::sync::atomic::Ordering::Relaxed);
            panic!("0x112f64 jpeg_error_exit: {id}");
        }
        err as *mut c_void
    }
}

// 0x112fc0 — __Z22jpeg_read_iptc_profileP8FIBITMAPPKhj
#[doc(alias = "jpeg_read_iptc_profile(FIBITMAP *,unsigned char const*,unsigned int)")]
pub fn stub_112fc0(dib: *mut c_void, data: *const u8, len: u32) -> i32 {
    // IDA 0x112fc0: tail call to read_iptc_profile (0x112fc8), whose return
    // passes through (0x112fcc).
    // SAFETY: `dib` writable, `data` readable for `len` bytes per caller.
    unsafe { read_iptc_profile(dib, data, len) }
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
pub fn stub_114340(plugin: *mut JpegPluginView, format_id: i32) -> *mut JpegPluginView {
    // IDA 0x114340: s_format_id_0 = a2 (0x11434c), then the same slot order
    // as InitJPEG with the _0 twins: Format (0x114358), Extension
    // (0x11436c), Description (0x11437c), Load (0x114380), zeros
    // (0x11438c..0x114398), RegExpr (0x1143a0), Save (0x1143b0), Validate
    // (0x1143b4), MimeType (0x1143bc), SupportsExportDepth (0x1143d4),
    // SupportsExportType (0x1143d8), SupportsICCProfiles (0x1143dc);
    // returns plugin (0x1143e0).
    // BUG: Load/Save (0x11535c/0x1144a8 below) still carry todo!() bodies —
    // only addresses wired, mirroring stub_116280.
    // SAFETY: `plugin` is a writable Plugin per the caller.
    PNG_S_FORMAT_ID.store(format_id, core::sync::atomic::Ordering::Relaxed);
    let p = unsafe { &mut *plugin };
    p.format = stub_114294 as *const c_void;
    p.description = stub_1142a4 as *const c_void;
    p.extension = stub_1142b4 as *const c_void;
    p.reg_expr = stub_1142c4 as *const c_void;
    p.reserved0 = core::ptr::null();
    p.reserved1 = core::ptr::null();
    p.reserved2 = core::ptr::null();
    p.reserved3 = core::ptr::null();
    p.load = stub_11535c as *const c_void;
    p.save = stub_1144a8 as *const c_void;
    p.validate = stub_114414 as *const c_void;
    p.mime_type = stub_1142d4 as *const c_void;
    p.supports_export_depth = stub_1142e4 as *const c_void;
    p.supports_export_type = stub_114314 as *const c_void;
    p.supports_icc_profiles = stub_114338 as *const c_void;
    plugin
}

// 0x114414 — __ZL8ValidateP11FreeImageIOPv_0
#[doc(alias = "__ZL8ValidateP11FreeImageIOPv_0")]
pub fn stub_114414(io: *const TiffIoHooks, handle: *mut c_void) -> bool {
    // IDA 0x114414: __dst = C.152 (0xf75a13 = 89 50 4E 47 0D 0A 1A 0A, via
    // 0x114438), v5 = C.153 (zeros, via 0x11444c); read_proc = io[+0] word,
    // little-endian (0x114450..0x114468); read_proc(v5, 1, 8, handle)
    // (0x11447c); return memcmp(__dst, v5, 8) == 0 (0x11448c..0x11449c).
    // The v5 init bytes are overwritten by the read, so only the PNG
    // signature expectation is load-bearing.
    // SAFETY: `io` readable; `handle` valid for the stored read_proc.
    let mut sig = [0u8; 8];
    unsafe {
        let io = &*io;
        (io.read_proc)(sig.as_mut_ptr() as *mut c_void, 1, 8, handle);
    }
    sig == [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]
}

// 0x1144a8 — __ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3__0
#[doc(alias = "__ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3__0")]
pub fn stub_1144a8() -> ! {
    todo!("0x1144a8 __ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3__0")
}

// 0x115258 — __ZL10_WriteProcP14png_struct_defPhm
#[doc(alias = "_WriteProc(png_struct_def *,unsigned char *,unsigned long)")]
pub fn stub_115258(png: *mut c_void, data: *mut c_void, len: u32) -> u32 {
    // IDA 0x115258: pair = png_get_io_ptr(png) (0x115268); write_proc =
    // pair.io[+4] word, little-endian (0x11526c..0x11528c); tail call
    // write_proc(data, len, 1, pair.handle) (0x115290..0x1152a0).
    // SAFETY: libpng guarantees a readable png_ptr; the stored pair holds
    // a readable TiffIoHooks table and a handle valid for write_proc.
    unsafe {
        let pair = &*(png_get_io_ptr(png) as *const TiffHandlePair);
        let io = &*pair.io;
        (io.write_proc)(data, len, 1, pair.handle)
    }
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
pub fn stub_1152d0(png: *mut c_void, data: *mut c_void, len: u32) -> u32 {
    // IDA 0x1152d0: pair = png_get_io_ptr(png) (0x1152e0); n =
    // read_proc(data, len, 1, handle) via the LE io[+0] word
    // (0x1152e4..0x115314); fail = (len != 0) (0x115318..0x11531c);
    // n != 0 -> fail = false (0x115320..0x115324); fail -> throw "Read
    // error: invalid or corrupted PNG file" as char * (0x115328..0x115350);
    // return n (0x11532c). The char* throw crosses the C API; panic! is the
    // host mapping (as in stub_1152a4).
    // SAFETY: as for stub_115258, with `data` writable for `len` bytes.
    unsafe {
        let pair = &*(png_get_io_ptr(png) as *const TiffHandlePair);
        let io = &*pair.io;
        let n = (io.read_proc)(data, len, 1, pair.handle);
        let mut fail = len != 0;
        if n != 0 {
            fail = false;
        }
        if fail {
            panic!("0x1152d0 _ReadProc: Read error: invalid or corrupted PNG file");
        }
        n
    }
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
pub fn stub_116ba4(image_type: i32, width: i32, height: i32, bits: u16, channels: u16) -> *mut c_void {
    // IDA 0x116ba4: depth = (u16)(bits * channels) (MUL + UXTH,
    // 0x116bc0..0x116bc8). FIT_UINT16 (1) with 16 bits: 1 channel -> 8-bit
    // with zero masks (0x116bd8..0x116c38), 3 channels -> 24-bit RGB
    // (0x116be0..0x116c08, R=0xFF0000 G=0xFF00 B=0xFF), else NULL
    // (0x116be4..0x116be8); depth 16 from other pairs: (8,2) -> 8-bit
    // (0x116c10..0x116c38), else 16-bit 5-6-5 (0x116c3c..0x116c5c, R=0xF800
    // G=0x7E0 B=0x1F); depth 24/32 -> RGB(A) (0x116c60..0x116c88, same RGB
    // masks); other depths -> unpacked with zero masks (0x116c94..0x116cac);
    // non-UINT16 types -> FreeImage_AllocateT(type, w, h, depth, 0, 0, 0)
    // (0x116cb0..0x116cc4).
    // SAFETY: the Allocate twins own their contract; only ints cross here.
    unsafe {
        let depth = bits.wrapping_mul(channels);
        if image_type == 1 {
            if bits == 16 {
                if channels == 1 {
                    return FreeImage_Allocate(width, height, 8, 0, 0, 0);
                }
                if channels != 3 {
                    return core::ptr::null_mut();
                }
                return FreeImage_Allocate(width, height, 24, 0xFF00_00, 0xFF00, 0xFF);
            }
            if depth == 16 && !(channels == 2 && bits == 8) {
                return FreeImage_Allocate(width, height, 16, 0xF800, 0x7E0, 0x1F);
            }
            if depth == 16 {
                return FreeImage_Allocate(width, height, 8, 0, 0, 0);
            }
            if depth == 24 || depth == 32 {
                return FreeImage_Allocate(width, height, depth as i32, 0xFF00_00, 0xFF00, 0xFF);
            }
            return FreeImage_Allocate(width, height, depth as i32, 0, 0, 0);
        }
        FreeImage_AllocateT(image_type, width, height, depth as i32, 0, 0, 0)
    }
}

// 0x116cd0 — __ZL14ReadResolutionP4tiffP8FIBITMAP
#[doc(alias = "ReadResolution(tiff *,FIBITMAP *)")]
pub fn stub_116cd0(tif: *mut c_void, dib: *mut c_void) -> i32 {
    // IDA 0x116cd0: unit = 2, xres = yres = 300.0 (0x116cf4..0x116cfc);
    // TIFFGetField(tif, 296, &unit) (0x116d00..0x116d04), (282, &xres)
    // (0x116d08..0x116d14), (283, &yres) -> result (0x116d18..0x116d24);
    // unit 1 (inch) with positive res falls into the unit 2 path as unit 2
    // (0x116d34..0x116d58); unit 2: res / 0.0254 + 0.5 in f64 from f32
    // (0x116d68..0x116dac); unit 3 (cm): res * 100.0 + 0.5 (0x116db8..0x116df4);
    // other units, or inch with non-positive res, return result
    // (0x116db0..0x116db4, 0x116d40..0x116dfc).
    // SAFETY: `tif`/`dib` valid for the TIFF/FreeImage calls per caller.
    unsafe {
        let mut unit: u16 = 2;
        let mut xres: f32 = 300.0;
        let mut yres: f32 = 300.0;
        TIFFGetField(tif, 296, &mut unit);
        TIFFGetField(tif, 282, &mut xres);
        let result = TIFFGetField(tif, 283, &mut yres);
        if unit == 1 {
            if xres > 0.0 && yres > 0.0 {
                unit = 2;
            } else {
                return result;
            }
        }
        if unit == 2 {
            FreeImage_SetDotsPerMeterX(dib, (xres as f64 / 0.0254 + 0.5) as u32);
            return FreeImage_SetDotsPerMeterY(dib, (yres as f64 / 0.0254 + 0.5) as u32);
        }
        if unit == 3 {
            FreeImage_SetDotsPerMeterX(dib, (xres as f64 * 100.0 + 0.5) as u32);
            return FreeImage_SetDotsPerMeterY(dib, (yres as f64 * 100.0 + 0.5) as u32);
        }
        result
    }
}

// 0x116e20 — __ZL9PageCountP11FreeImageIOPvS1_
#[doc(alias = "PageCount(FreeImageIO *,void *,void *)")]
pub fn stub_116e20(_io: *const TiffIoHooks, _handle: *mut c_void, ctx: *mut c_void) -> i32 {
    // IDA 0x116e20: ctx == NULL -> 0 (0x116e28..0x116e2c); tif = ctx[2]
    // (0x116e34); do ++n while TIFFReadDirectory(tif) (0x116e38..0x116e4c) —
    // the initial directory counts, so no iteration is special-cased.
    // SAFETY: `ctx` (when non-null) holds a TIFF* at +8 per stub_116f34.
    if ctx.is_null() {
        return 0;
    }
    unsafe {
        let tif = *(ctx as *mut *mut c_void).add(2);
        let mut n = 0;
        loop {
            n += 1;
            if TIFFReadDirectory(tif) == 0 {
                break;
            }
        }
        n
    }
}

// 0x116e58 — __ZL5CloseP11FreeImageIOPvS1_
#[doc(alias = "Close(FreeImageIO *,void *,void *)")]
pub fn stub_116e58(_io: *const TiffIoHooks, _handle: *mut c_void, ctx: *mut c_void) {
    // IDA 0x116e58: ctx == NULL -> return (0x116e60..0x116e64);
    // TIFFClose(ctx[2]) (0x116e68..0x116e6c); free(ctx) (0x116e70..0x116e74).
    // BUG: freed with stub_116460 (the prefixed-allocator host mapping),
    // pairing stub_116f34's stub_116450 malloc below.
    // SAFETY: `ctx` came from stub_116f34's 12-byte block when non-null.
    if ctx.is_null() {
        return;
    }
    unsafe {
        TIFFClose(*(ctx as *mut *mut c_void).add(2));
        stub_116460(ctx);
    }
}

// 0x116e7c — __TIFFrealloc
#[doc(alias = "__TIFFrealloc")]
pub fn stub_116e7c(ptr: *mut c_void, size: usize) -> *mut c_void {
    // IDA 0x116e7c: tail call to system realloc (0x116e84).
    // BUG: hosted on the prefixed std::alloc pairing from stub_116450 (a
    // 16-byte header holding the size): the old size is recovered from the
    // header, the common prefix copied, and the old block released; a
    // failed realloc keeps the old block live and returns NULL, as in C.
    // SAFETY: `ptr` (when non-null) is a live stub_116450 block.
    if ptr.is_null() {
        return stub_116450(size);
    }
    unsafe {
        let old = ((ptr as *const u8).sub(16) as *const usize).read();
        let new = stub_116450(size);
        if new.is_null() {
            return core::ptr::null_mut();
        }
        core::ptr::copy_nonoverlapping(ptr as *const u8, new as *mut u8, old.min(size));
        stub_116460(ptr);
        new
    }
}

// 0x116e8c — __Z10TIFFFdOpenPvPKcS1_
#[doc(alias = "TIFFFdOpen(void *,char const*,char const*)")]
pub fn stub_116e8c(ctx: *mut c_void, name: *const c_char, mode: *const c_char) -> *mut c_void {
    // IDA 0x116e8c: XTIFFInitialize() (0x116ea4); tif = TIFFClientOpen(name,
    // mode, ctx, read, write, seek, close, size, map, unmap) (0x116f04);
    // if tif: tif.clientdata (+4) = ctx (0x116f0c); return tif (0x116f14).
    // SAFETY: `name`/`mode` are readable NUL-terminated strings; the proc
    // addresses are this file's _tiff*Proc ports, as in the original.
    unsafe {
        XTIFFInitialize();
        let tif = TIFFClientOpen(
            name,
            mode,
            ctx,
            stub_11600c as *mut c_void,
            stub_116054 as *mut c_void,
            stub_11609c as *mut c_void,
            stub_1160fc as *mut c_void,
            stub_116104 as *mut c_void,
            stub_1161d0 as *mut c_void,
            stub_1161d8 as *mut c_void,
        );
        if !tif.is_null() {
            *(tif as *mut *mut c_void).add(1) = ctx;
        }
        tif
    }
}

// 0x116f34 — __ZL4OpenP11FreeImageIOPvi
#[doc(alias = "Open(FreeImageIO *,void *,int)")]
pub fn stub_116f34(io: *const TiffIoHooks, handle: *mut c_void, read_flag: i32) -> *mut c_void {
    // IDA 0x116f34: ctx = malloc(12) (0x116f44..0x116f50); NULL -> return
    // NULL (0x116f54..0x116f58); ctx[0] = io, ctx[1] = handle (0x116f60);
    // mode = read_flag ? "r" : "w" (0x116f5c..0x116f88, aR/aW); ctx[2] =
    // TIFFFdOpen(ctx, "", mode) (0x116f8c..0x116f90); NULL tif -> free(ctx)
    // (0x116fa0..0x116fa4), OutputMessageProc(s_format_id_1, "Error while
    // opening TIFF: data is invalid") (0x116fa8..0x116fb8), return NULL
    // (0x116fbc..0x116fc8); else return ctx (0x116fc4).
    // BUG: malloc/free hosted on stub_116450/stub_116460 (see stub_116e58).
    // SAFETY: the byte-string literals carry their NUL terminators.
    unsafe {
        let ctx = stub_116450(12) as *mut *mut c_void;
        if ctx.is_null() {
            return core::ptr::null_mut();
        }
        *ctx = io as *mut c_void;
        *ctx.add(1) = handle;
        let mode: &[u8] = if read_flag != 0 { b"r\0" } else { b"w\0" };
        let tif = stub_116e8c(
            ctx as *mut c_void,
            b"\0".as_ptr() as *const c_char,
            mode.as_ptr() as *const c_char,
        );
        *ctx.add(2) = tif;
        if tif.is_null() {
            stub_116460(ctx as *mut c_void);
            let id = TIFF_S_FORMAT_ID.load(core::sync::atomic::Ordering::Relaxed);
            FreeImage_OutputMessageProc(
                id,
                b"Error while opening TIFF: data is invalid\0".as_ptr() as *const c_char,
            );
            core::ptr::null_mut()
        } else {
            ctx as *mut c_void
        }
    }
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
pub fn stub_123284(cinfo: *mut *mut u8, value: u8) -> *mut u8 {
    // IDA 0x123284: value truncated to a byte (UXTB, 0x123288); for table
    // slots [21..24] (quant, +0x54..+0x60): if non-null, the byte at +128
    // = value (0x123284..0x1232b4); for slots [29, 33, 30, 34, 31, 35, 32,
    // 36] (huff, +0x74..+0x90, in that order): if non-null, the byte at
    // +273 = value (0x1232b8..0x123314); return slot 36 (0x12330c..0x123318).
    // SAFETY: `cinfo` points at 37 table-pointer words per the caller.
    unsafe {
        let tables = core::slice::from_raw_parts(cinfo as *const *mut u8, 37);
        for &i in &[21usize, 22, 23, 24] {
            if !tables[i].is_null() {
                *tables[i].add(128) = value;
            }
        }
        for &i in &[29usize, 33, 30, 34, 31, 35, 32, 36] {
            if !tables[i].is_null() {
                *tables[i].add(273) = value;
            }
        }
        tables[36]
    }
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
pub fn stub_123688(cinfo: *mut c_void) -> i32 {
    // IDA 0x123688: tail call to jpeg_destroy (0x123690), whose R0 residue
    // passes through (see the jpeg_destroy BUG note in the extern block).
    // SAFETY: `cinfo` is a live jpeg_compress_struct per the caller.
    unsafe { jpeg_destroy(cinfo) }
}

// 0x123698 — _jpeg_CreateCompress
#[doc(alias = "_jpeg_CreateCompress")]
pub fn stub_123698(cinfo: *mut u32, version: i32, struct_size: i32) -> *mut c_void {
    // IDA 0x123698: cinfo[1] = 0 (0x1236b4); version != 70
    // (JPEG_LIB_VERSION, CMP #0x46 at 0x1236a4): err.msg_code = 13,
    // expected = 70, actual = version (0x1236c8..0x1236d8), then
    // err.error_exit(cinfo) (0x1236e4); struct_size != 0x190 (CMP at
    // 0x1236e8): code 22, expected 0x190, actual (0x1236fc..0x12370c), then
    // error_exit (0x12371c); save err/client_data (0x123720..0x123724),
    // memset(cinfo, 0, 0x190) (0x123734), restore both, is_decompressor
    // byte (+16) = 0 (0x12373c..0x123744); result =
    // jinit_memory_mgr(cinfo) (0x123748); defaults (0x123758..0x1237b0):
    // progress/mem/quant slots zero, quality fields 100, scale 1.0
    // (0x3FF00000 at +0x30).
    // SAFETY: `cinfo` is a writable 0x190-byte jpeg_compress_struct.
    unsafe {
        *cinfo.add(1) = 0;
        if version != 70 {
            let err = *cinfo as *mut u32;
            *err.add(5) = 13;
            *err.add(6) = 70;
            *err.add(7) = version as u32;
            let exit = *(*err as *mut unsafe extern "C" fn(*mut c_void));
            exit(cinfo as *mut c_void);
        }
        if struct_size != 0x190 {
            let err = *cinfo as *mut u32;
            *err.add(5) = 22;
            *err.add(6) = 0x190;
            *err.add(7) = struct_size as u32;
            let exit = *(*err as *mut unsafe extern "C" fn(*mut c_void));
            exit(cinfo as *mut c_void);
        }
        let err_saved = *cinfo;
        let client_data = *cinfo.add(3);
        core::ptr::write_bytes(cinfo as *mut u8, 0, 0x190);
        *cinfo = err_saved;
        *cinfo.add(3) = client_data;
        *(cinfo as *mut u8).add(16) = 0;
        let result = jinit_memory_mgr(cinfo as *mut c_void);
        *cinfo.add(2) = 0;
        *cinfo.add(6) = 0;
        *cinfo.add(20) = 0;
        *cinfo.add(21) = 0;
        *cinfo.add(25) = 100;
        *cinfo.add(22) = 0;
        *cinfo.add(26) = 100;
        *cinfo.add(23) = 0;
        *cinfo.add(27) = 100;
        *cinfo.add(24) = 0;
        *cinfo.add(28) = 100;
        *cinfo.add(29) = 0;
        *cinfo.add(33) = 0;
        *cinfo.add(30) = 0;
        *cinfo.add(34) = 0;
        *cinfo.add(31) = 0;
        *cinfo.add(35) = 0;
        *cinfo.add(32) = 0;
        *cinfo.add(36) = 0;
        *cinfo.add(98) = 0;
        *cinfo.add(11) = 0;
        *cinfo.add(12) = 0x3FF0_0000;
        *cinfo.add(5) = 100;
        result
    }
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
