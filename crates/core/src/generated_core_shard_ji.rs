//! core shard ji — 150 core stubs EA-sorted, 0x1a1168..0x1b5288 (EA-sorted asc not yet in rbx_core, next 150 uncovered after 0x1a0b9c, rbx_core::SharedPtr not boost).
//! Source: ida/export.json (85545 funcs) filtered where demangled/mangled excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 150 not yet in rbx_core (core-local gap filler, rbx_core::SharedPtr not boost).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "_uv_encode")]
// 0x1a1168 — _uv_encode — _uv_encode
pub fn stub_0x1a1168() {
    // IDA 0x1a1168: colorspace-conversion helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Luv24fromLuv48")]
// 0x1a12b8 — _Luv24fromLuv48 — _Luv24fromLuv48
pub fn stub_0x1a12b8() {
    // IDA 0x1a12b8: colorspace-conversion helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_LogL10fromY")]
// 0x1a1638 — _LogL10fromY — _LogL10fromY
pub fn stub_0x1a1638() {
    // IDA 0x1a1638: colorspace-conversion helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_LogLuv24fromXYZ")]
// 0x1a1718 — _LogLuv24fromXYZ — _LogLuv24fromXYZ
pub fn stub_0x1a1718() {
    // IDA 0x1a1718: colorspace-conversion helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Luv24fromXYZ")]
// 0x1a1804 — _Luv24fromXYZ — _Luv24fromXYZ
pub fn stub_0x1a1804() {
    // IDA 0x1a1804: colorspace-conversion helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_LogL16fromY")]
// 0x1a19cc — _LogL16fromY — _LogL16fromY
pub fn stub_0x1a19cc() {
    // IDA 0x1a19cc: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_LogLuv32fromXYZ")]
// 0x1a1b74 — _LogLuv32fromXYZ — _LogLuv32fromXYZ
pub fn stub_0x1a1b74() {
    // IDA 0x1a1b74: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Luv32fromXYZ")]
// 0x1a1cf4 — _Luv32fromXYZ — _Luv32fromXYZ
pub fn stub_0x1a1cf4() {
    // IDA 0x1a1cf4: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_L16fromY")]
// 0x1a1ebc — _L16fromY — _L16fromY
pub fn stub_0x1a1ebc() {
    // IDA 0x1a1ebc: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_LogL10toY")]
// 0x1a1fe8 — _LogL10toY — _LogL10toY
pub fn stub_0x1a1fe8() {
    // IDA 0x1a1fe8: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_LogLuv24toXYZ")]
// 0x1a2038 — _LogLuv24toXYZ — _LogLuv24toXYZ
pub fn stub_0x1a2038() {
    // IDA 0x1a2038: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Luv24toRGB")]
// 0x1a2144 — _Luv24toRGB — _Luv24toRGB
pub fn stub_0x1a2144() {
    // IDA 0x1a2144: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Luv24toXYZ")]
// 0x1a227c — _Luv24toXYZ — _Luv24toXYZ
pub fn stub_0x1a227c() {
    // IDA 0x1a227c: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_LogL16toY")]
// 0x1a23e8 — _LogL16toY — _LogL16toY
pub fn stub_0x1a23e8() {
    // IDA 0x1a23e8: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_LogLuv32toXYZ")]
// 0x1a2448 — _LogLuv32toXYZ — _LogLuv32toXYZ
pub fn stub_0x1a2448() {
    // IDA 0x1a2448: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Luv32toRGB")]
// 0x1a2528 — _Luv32toRGB — _Luv32toRGB
pub fn stub_0x1a2528() {
    // IDA 0x1a2528: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Luv32toXYZ")]
// 0x1a2660 — _Luv32toXYZ — _Luv32toXYZ
pub fn stub_0x1a2660() {
    // IDA 0x1a2660: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_L16toGry")]
// 0x1a27cc — _L16toGry — _L16toGry
pub fn stub_0x1a27cc() {
    // IDA 0x1a27cc: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_L16toY")]
// 0x1a2a84 — _L16toY — _L16toY
pub fn stub_0x1a2a84() {
    // IDA 0x1a2a84: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_cl_hash")]
// 0x1a2c70 — _cl_hash — _cl_hash
pub fn stub_0x1a2c70() {
    // IDA 0x1a2c70: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_LZWPreEncode")]
// 0x1a2dc8 — _LZWPreEncode — _LZWPreEncode
pub fn stub_0x1a2dc8() {
    // IDA 0x1a2dc8: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFInitLZW")]
// 0x1a2e80 — _TIFFInitLZW — _TIFFInitLZW
pub fn stub_0x1a2e80() {
    // IDA 0x1a2e80: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_LZWSetupEncode")]
// 0x1a2fc0 — _LZWSetupEncode — _LZWSetupEncode
pub fn stub_0x1a2fc0() {
    // IDA 0x1a2fc0: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_LZWCleanup")]
// 0x1a3048 — _LZWCleanup — _LZWCleanup
pub fn stub_0x1a3048() {
    // IDA 0x1a3048: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_LZWPostEncode")]
// 0x1a30d8 — _LZWPostEncode — _LZWPostEncode
pub fn stub_0x1a30d8() {
    // IDA 0x1a30d8: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_LZWEncode")]
// 0x1a31b0 — _LZWEncode — _LZWEncode
pub fn stub_0x1a31b0() {
    // IDA 0x1a31b0: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_LZWDecodeCompat")]
// 0x1a363c — _LZWDecodeCompat — _LZWDecodeCompat
pub fn stub_0x1a363c() {
    // IDA 0x1a363c: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_LZWPreDecode")]
// 0x1a40d0 — _LZWPreDecode — _LZWPreDecode
pub fn stub_0x1a40d0() {
    // IDA 0x1a40d0: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_LZWSetupDecode")]
// 0x1a4214 — _LZWSetupDecode — _LZWSetupDecode
pub fn stub_0x1a4214() {
    // IDA 0x1a4214: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_LZWDecode")]
// 0x1a4404 — _LZWDecode — _LZWDecode
pub fn stub_0x1a4404() {
    // IDA 0x1a4404: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFInitNeXT")]
// 0x1a4fd8 — _TIFFInitNeXT — _TIFFInitNeXT
pub fn stub_0x1a4fd8() {
    // IDA 0x1a4fd8: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_NeXTDecode")]
// 0x1a4ff8 — _NeXTDecode — _NeXTDecode
pub fn stub_0x1a4ff8() {
    // IDA 0x1a4ff8: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_OJPEGWriteStreamQTable")]
// 0x1a52bc — _OJPEGWriteStreamQTable — _OJPEGWriteStreamQTable
pub fn stub_0x1a52bc() {
    // IDA 0x1a52bc: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_OJPEGWriteStreamDcTable")]
// 0x1a5300 — _OJPEGWriteStreamDcTable — _OJPEGWriteStreamDcTable
pub fn stub_0x1a5300() {
    // IDA 0x1a5300: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_OJPEGWriteStreamAcTable")]
// 0x1a5344 — _OJPEGWriteStreamAcTable — _OJPEGWriteStreamAcTable
pub fn stub_0x1a5344() {
    // IDA 0x1a5344: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_OJPEGLibjpegJpegSourceMgrInitSource")]
// 0x1a5388 — _OJPEGLibjpegJpegSourceMgrInitSource — _OJPEGLibjpegJpegSourceMgrInitSource
pub fn stub_0x1a5388() {
    // IDA 0x1a5388: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_OJPEGLibjpegJpegSourceMgrTermSource")]
// 0x1a538c — _OJPEGLibjpegJpegSourceMgrTermSource — _OJPEGLibjpegJpegSourceMgrTermSource
pub fn stub_0x1a538c() {
    // IDA 0x1a538c: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_OJPEGReadSkip")]
// 0x1a5390 — _OJPEGReadSkip — _OJPEGReadSkip
pub fn stub_0x1a5390() {
    // IDA 0x1a5390: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_OJPEGReadBufferFill")]
// 0x1a540c — _OJPEGReadBufferFill — _OJPEGReadBufferFill
pub fn stub_0x1a540c() {
    // IDA 0x1a540c: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_OJPEGReadByte")]
// 0x1a5610 — _OJPEGReadByte — _OJPEGReadByte
pub fn stub_0x1a5610() {
    // IDA 0x1a5610: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_OJPEGReadWord")]
// 0x1a56a4 — _OJPEGReadWord — _OJPEGReadWord
pub fn stub_0x1a56a4() {
    // IDA 0x1a56a4: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_OJPEGReadHeaderInfoSecStreamSos")]
// 0x1a570c — _OJPEGReadHeaderInfoSecStreamSos — _OJPEGReadHeaderInfoSecStreamSos
pub fn stub_0x1a570c() {
    // IDA 0x1a570c: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_OJPEGPostEncode")]
// 0x1a58b0 — _OJPEGPostEncode — _OJPEGPostEncode
pub fn stub_0x1a58b0() {
    // IDA 0x1a58b0: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_OJPEGEncode")]
// 0x1a58e0 — _OJPEGEncode — _OJPEGEncode
pub fn stub_0x1a58e0() {
    // IDA 0x1a58e0: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_OJPEGPreEncode")]
// 0x1a5910 — _OJPEGPreEncode — _OJPEGPreEncode
pub fn stub_0x1a5910() {
    // IDA 0x1a5910: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_OJPEGSetupEncode")]
// 0x1a5940 — _OJPEGSetupEncode — _OJPEGSetupEncode
pub fn stub_0x1a5940() {
    // IDA 0x1a5940: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFInitOJPEG")]
// 0x1a5970 — _TIFFInitOJPEG — _TIFFInitOJPEG
pub fn stub_0x1a5970() {
    // IDA 0x1a5970: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_OJPEGPrintDir")]
// 0x1a5b50 — _OJPEGPrintDir — _OJPEGPrintDir
pub fn stub_0x1a5b50() {
    // IDA 0x1a5b50: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_OJPEGVSetField")]
// 0x1a5d88 — _OJPEGVSetField — _OJPEGVSetField
pub fn stub_0x1a5d88() {
    // IDA 0x1a5d88: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_OJPEGLibjpegJpegErrorMgrOutputMessage")]
// 0x1a631c — _OJPEGLibjpegJpegErrorMgrOutputMessage — _OJPEGLibjpegJpegErrorMgrOutputMessage
pub fn stub_0x1a631c() {
    // IDA 0x1a631c: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_OJPEGSetupDecode")]
// 0x1a636c — _OJPEGSetupDecode — _OJPEGSetupDecode
pub fn stub_0x1a636c() {
    // IDA 0x1a636c: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_OJPEGReadBlock")]
// 0x1a639c — _OJPEGReadBlock — _OJPEGReadBlock
pub fn stub_0x1a639c() {
    // IDA 0x1a639c: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_OJPEGReadHeaderInfoSec")]
// 0x1a64cc — _OJPEGReadHeaderInfoSec — _OJPEGReadHeaderInfoSec
pub fn stub_0x1a64cc() {
    // IDA 0x1a64cc: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_OJPEGSubsamplingCorrect")]
// 0x1a78f4 — _OJPEGSubsamplingCorrect — _OJPEGSubsamplingCorrect
pub fn stub_0x1a78f4() {
    // IDA 0x1a78f4: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_OJPEGVGetField")]
// 0x1a7b10 — _OJPEGVGetField — _OJPEGVGetField
pub fn stub_0x1a7b10() {
    // IDA 0x1a7b10: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_OJPEGLibjpegSessionAbort")]
// 0x1a7c3c — _OJPEGLibjpegSessionAbort — _OJPEGLibjpegSessionAbort
pub fn stub_0x1a7c3c() {
    // IDA 0x1a7c3c: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_OJPEGCleanup")]
// 0x1a7c94 — _OJPEGCleanup — _OJPEGCleanup
pub fn stub_0x1a7c94() {
    // IDA 0x1a7c94: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_OJPEGPostDecode")]
// 0x1a7de0 — _OJPEGPostDecode — _OJPEGPostDecode
pub fn stub_0x1a7de0() {
    // IDA 0x1a7de0: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_jpeg_read_raw_data_encap")]
// 0x1a7e54 — _jpeg_read_raw_data_encap — _jpeg_read_raw_data_encap
pub fn stub_0x1a7e54() {
    // IDA 0x1a7e54: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_jpeg_read_scanlines_encap")]
// 0x1a7e9c — _jpeg_read_scanlines_encap — _jpeg_read_scanlines_encap
pub fn stub_0x1a7e9c() {
    // IDA 0x1a7e9c: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_OJPEGDecode")]
// 0x1a7ee4 — _OJPEGDecode — _OJPEGDecode
pub fn stub_0x1a7ee4() {
    // IDA 0x1a7ee4: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_jpeg_create_decompress_encap")]
// 0x1a8184 — _jpeg_create_decompress_encap — _jpeg_create_decompress_encap
pub fn stub_0x1a8184() {
    // IDA 0x1a8184: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_jpeg_read_header_encap")]
// 0x1a81c4 — _jpeg_read_header_encap — _jpeg_read_header_encap
pub fn stub_0x1a81c4() {
    // IDA 0x1a81c4: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_jpeg_start_decompress_encap")]
// 0x1a8208 — _jpeg_start_decompress_encap — _jpeg_start_decompress_encap
pub fn stub_0x1a8208() {
    // IDA 0x1a8208: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_OJPEGPreDecode")]
// 0x1a8240 — _OJPEGPreDecode — _OJPEGPreDecode
pub fn stub_0x1a8240() {
    // IDA 0x1a8240: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_OJPEGLibjpegJpegSourceMgrFillInputBuffer")]
// 0x1a8d3c — _OJPEGLibjpegJpegSourceMgrFillInputBuffer — _OJPEGLibjpegJpegSourceMgrFillInputBuffer
pub fn stub_0x1a8d3c() {
    // IDA 0x1a8d3c: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_OJPEGLibjpegJpegErrorMgrErrorExit")]
// 0x1a94e4 — _OJPEGLibjpegJpegErrorMgrErrorExit — _OJPEGLibjpegJpegErrorMgrErrorExit
pub fn stub_0x1a94e4() {
    // IDA 0x1a94e4: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_OJPEGLibjpegJpegSourceMgrResyncToRestart")]
// 0x1a9540 — _OJPEGLibjpegJpegSourceMgrResyncToRestart — _OJPEGLibjpegJpegSourceMgrResyncToRestart
pub fn stub_0x1a9540() {
    // IDA 0x1a9540: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_OJPEGLibjpegJpegSourceMgrSkipInputData")]
// 0x1a957c — _OJPEGLibjpegJpegSourceMgrSkipInputData — _OJPEGLibjpegJpegSourceMgrSkipInputData
pub fn stub_0x1a957c() {
    // IDA 0x1a957c: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "__tiffDummyMapProc")]
// 0x1a95b8 — __tiffDummyMapProc — __tiffDummyMapProc
pub fn stub_0x1a95b8() {
    // IDA 0x1a95b8: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "__tiffDummyUnmapProc")]
// 0x1a95c0 — __tiffDummyUnmapProc — __tiffDummyUnmapProc
pub fn stub_0x1a95c0() {
    // IDA 0x1a95c0: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFInitOrder")]
// 0x1a95c4 — _TIFFInitOrder — _TIFFInitOrder
pub fn stub_0x1a95c4() {
    // IDA 0x1a95c4: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFFileName")]
// 0x1a9614 — _TIFFFileName — _TIFFFileName
pub fn stub_0x1a9614() {
    // IDA 0x1a9614: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFIsTiled")]
// 0x1a961c — _TIFFIsTiled — _TIFFIsTiled
pub fn stub_0x1a961c() {
    // IDA 0x1a961c: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFIsByteSwapped")]
// 0x1a962c — _TIFFIsByteSwapped — _TIFFIsByteSwapped
pub fn stub_0x1a962c() {
    // IDA 0x1a962c: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "__TIFFgetMode")]
// 0x1a963c — __TIFFgetMode — __TIFFgetMode
pub fn stub_0x1a963c() {
    // IDA 0x1a963c: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFClientOpen")]
// 0x1a96b0 — _TIFFClientOpen — _TIFFClientOpen
pub fn stub_0x1a96b0() {
    // IDA 0x1a96b0: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFInitPackBits")]
// 0x1a9c6c — _TIFFInitPackBits — _TIFFInitPackBits
pub fn stub_0x1a9c6c() {
    // IDA 0x1a9c6c: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_PackBitsEncode")]
// 0x1a9cd0 — _PackBitsEncode — _PackBitsEncode
pub fn stub_0x1a9cd0() {
    // IDA 0x1a9cd0: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_PackBitsEncodeChunk")]
// 0x1aa21c — _PackBitsEncodeChunk — _PackBitsEncodeChunk
pub fn stub_0x1aa21c() {
    // IDA 0x1aa21c: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_PackBitsPostEncode")]
// 0x1aa28c — _PackBitsPostEncode — _PackBitsPostEncode
pub fn stub_0x1aa28c() {
    // IDA 0x1aa28c: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_PackBitsPreEncode")]
// 0x1aa2ac — _PackBitsPreEncode — _PackBitsPreEncode
pub fn stub_0x1aa2ac() {
    // IDA 0x1aa2ac: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_PackBitsDecode")]
// 0x1aa304 — _PackBitsDecode — _PackBitsDecode
pub fn stub_0x1aa304() {
    // IDA 0x1aa304: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_PixarLogGuessDataFmt")]
// 0x1aa4e4 — _PixarLogGuessDataFmt — _PixarLogGuessDataFmt
pub fn stub_0x1aa4e4() {
    // IDA 0x1aa4e4: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_multiply_0")]
// 0x1aa5c4 — _multiply_0 — _multiply_0
pub fn stub_0x1aa5c4() {
    // IDA 0x1aa5c4: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_PixarLogClose")]
// 0x1aa5f4 — _PixarLogClose — _PixarLogClose
pub fn stub_0x1aa5f4() {
    // IDA 0x1aa5f4: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_PixarLogVGetField")]
// 0x1aa608 — _PixarLogVGetField — _PixarLogVGetField
pub fn stub_0x1aa608() {
    // IDA 0x1aa608: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFInitPixarLog")]
// 0x1aa660 — _TIFFInitPixarLog — _TIFFInitPixarLog
pub fn stub_0x1aa660() {
    // IDA 0x1aa660: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_PixarLogVSetField")]
// 0x1ab284 — _PixarLogVSetField — _PixarLogVSetField
pub fn stub_0x1ab284() {
    // IDA 0x1ab284: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_PixarLogCleanup")]
// 0x1ab3f4 — _PixarLogCleanup — _PixarLogCleanup
pub fn stub_0x1ab3f4() {
    // IDA 0x1ab3f4: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_PixarLogPostEncode")]
// 0x1ab504 — _PixarLogPostEncode — _PixarLogPostEncode
pub fn stub_0x1ab504() {
    // IDA 0x1ab504: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_PixarLogEncode")]
// 0x1ab5c0 — _PixarLogEncode — _PixarLogEncode
pub fn stub_0x1ab5c0() {
    // IDA 0x1ab5c0: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_PixarLogPreEncode")]
// 0x1ada6c — _PixarLogPreEncode — _PixarLogPreEncode
pub fn stub_0x1ada6c() {
    // IDA 0x1ada6c: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_PixarLogSetupEncode")]
// 0x1adad4 — _PixarLogSetupEncode — _PixarLogSetupEncode
pub fn stub_0x1adad4() {
    // IDA 0x1adad4: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_PixarLogDecode")]
// 0x1adc1c — _PixarLogDecode — _PixarLogDecode
pub fn stub_0x1adc1c() {
    // IDA 0x1adc1c: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_PixarLogPreDecode")]
// 0x1b0abc — _PixarLogPreDecode — _PixarLogPreDecode
pub fn stub_0x1b0abc() {
    // IDA 0x1b0abc: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_PixarLogSetupDecode")]
// 0x1b0b24 — _PixarLogSetupDecode — _PixarLogSetupDecode
pub fn stub_0x1b0b24() {
    // IDA 0x1b0b24: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_horAcc8")]
// 0x1b0c78 — _horAcc8 — _horAcc8
pub fn stub_0x1b0c78() {
    // IDA 0x1b0c78: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_horAcc16")]
// 0x1b1240 — _horAcc16 — _horAcc16
pub fn stub_0x1b1240() {
    // IDA 0x1b1240: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_horAcc32")]
// 0x1b1480 — _horAcc32 — _horAcc32
pub fn stub_0x1b1480() {
    // IDA 0x1b1480: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_horDiff8")]
// 0x1b16c8 — _horDiff8 — _horDiff8
pub fn stub_0x1b16c8() {
    // IDA 0x1b16c8: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_horDiff16")]
// 0x1b1cfc — _horDiff16 — _horDiff16
pub fn stub_0x1b1cfc() {
    // IDA 0x1b1cfc: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_horDiff32")]
// 0x1b1f48 — _horDiff32 — _horDiff32
pub fn stub_0x1b1f48() {
    // IDA 0x1b1f48: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFPredictorCleanup")]
// 0x1b219c — _TIFFPredictorCleanup — _TIFFPredictorCleanup
pub fn stub_0x1b219c() {
    // IDA 0x1b219c: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_PredictorVGetField")]
// 0x1b220c — _PredictorVGetField — _PredictorVGetField
pub fn stub_0x1b220c() {
    // IDA 0x1b220c: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_PredictorVSetField")]
// 0x1b22b4 — _PredictorVSetField — _PredictorVSetField
pub fn stub_0x1b22b4() {
    // IDA 0x1b22b4: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_PredictorEncodeRow")]
// 0x1b2378 — _PredictorEncodeRow — _PredictorEncodeRow
pub fn stub_0x1b2378() {
    // IDA 0x1b2378: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_PredictorDecodeTile")]
// 0x1b2460 — _PredictorDecodeTile — _PredictorDecodeTile
pub fn stub_0x1b2460() {
    // IDA 0x1b2460: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_PredictorDecodeRow")]
// 0x1b2598 — _PredictorDecodeRow — _PredictorDecodeRow
pub fn stub_0x1b2598() {
    // IDA 0x1b2598: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFPredictorInit")]
// 0x1b2688 — _TIFFPredictorInit — _TIFFPredictorInit
pub fn stub_0x1b2688() {
    // IDA 0x1b2688: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_PredictorSetup")]
// 0x1b27a0 — _PredictorSetup — _PredictorSetup
pub fn stub_0x1b27a0() {
    // IDA 0x1b27a0: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_PredictorSetupEncode")]
// 0x1b289c — _PredictorSetupEncode — _PredictorSetupEncode
pub fn stub_0x1b289c() {
    // IDA 0x1b289c: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_PredictorSetupDecode")]
// 0x1b29d0 — _PredictorSetupDecode — _PredictorSetupDecode
pub fn stub_0x1b29d0() {
    // IDA 0x1b29d0: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_fpDiff")]
// 0x1b2ba4 — _fpDiff — _fpDiff
pub fn stub_0x1b2ba4() {
    // IDA 0x1b2ba4: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_fpAcc")]
// 0x1b2f90 — _fpAcc — _fpAcc
pub fn stub_0x1b2f90() {
    // IDA 0x1b2f90: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_PredictorEncodeTile")]
// 0x1b336c — _PredictorEncodeTile — _PredictorEncodeTile
pub fn stub_0x1b336c() {
    // IDA 0x1b336c: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_swabHorAcc32")]
// 0x1b355c — _swabHorAcc32 — _swabHorAcc32
pub fn stub_0x1b355c() {
    // IDA 0x1b355c: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_swabHorAcc16")]
// 0x1b37b8 — _swabHorAcc16 — _swabHorAcc16
pub fn stub_0x1b37b8() {
    // IDA 0x1b37b8: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_PredictorPrintDir")]
// 0x1b3a08 — _PredictorPrintDir — _PredictorPrintDir
pub fn stub_0x1b3a08() {
    // IDA 0x1b3a08: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFStartStrip")]
// 0x1b3afc — _TIFFStartStrip — _TIFFStartStrip
pub fn stub_0x1b3afc() {
    // IDA 0x1b3afc: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "__TIFFNoPostDecode")]
// 0x1b3b90 — __TIFFNoPostDecode — __TIFFNoPostDecode
pub fn stub_0x1b3b90() {
    // IDA 0x1b3b90: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "__TIFFSwab64BitData")]
// 0x1b3b94 — __TIFFSwab64BitData — __TIFFSwab64BitData
pub fn stub_0x1b3b94() {
    // IDA 0x1b3b94: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "__TIFFSwab32BitData")]
// 0x1b3bec — __TIFFSwab32BitData — __TIFFSwab32BitData
pub fn stub_0x1b3bec() {
    // IDA 0x1b3bec: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "__TIFFSwab24BitData")]
// 0x1b3c44 — __TIFFSwab24BitData — __TIFFSwab24BitData
pub fn stub_0x1b3c44() {
    // IDA 0x1b3c44: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "__TIFFSwab16BitData")]
// 0x1b3ca4 — __TIFFSwab16BitData — __TIFFSwab16BitData
pub fn stub_0x1b3ca4() {
    // IDA 0x1b3ca4: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFCheckRead")]
// 0x1b3cf4 — _TIFFCheckRead — _TIFFCheckRead
pub fn stub_0x1b3cf4() {
    // IDA 0x1b3cf4: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFReadBufferSetup")]
// 0x1b3d80 — _TIFFReadBufferSetup — _TIFFReadBufferSetup
pub fn stub_0x1b3d80() {
    // IDA 0x1b3d80: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFReadRawTile1")]
// 0x1b3e84 — _TIFFReadRawTile1 — _TIFFReadRawTile1
pub fn stub_0x1b3e84() {
    // IDA 0x1b3e84: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFFillTile")]
// 0x1b4014 — _TIFFFillTile — _TIFFFillTile
pub fn stub_0x1b4014() {
    // IDA 0x1b4014: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFReadEncodedTile")]
// 0x1b4288 — _TIFFReadEncodedTile — _TIFFReadEncodedTile
pub fn stub_0x1b4288() {
    // IDA 0x1b4288: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFReadRawStrip1")]
// 0x1b436c — _TIFFReadRawStrip1 — _TIFFReadRawStrip1
pub fn stub_0x1b436c() {
    // IDA 0x1b436c: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFFillStrip")]
// 0x1b44e4 — _TIFFFillStrip — _TIFFFillStrip
pub fn stub_0x1b44e4() {
    // IDA 0x1b44e4: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFReadTile")]
// 0x1b46f4 — _TIFFReadTile — _TIFFReadTile
pub fn stub_0x1b46f4() {
    // IDA 0x1b46f4: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFReadEncodedStrip")]
// 0x1b4794 — _TIFFReadEncodedStrip — _TIFFReadEncodedStrip
pub fn stub_0x1b4794() {
    // IDA 0x1b4794: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFDefaultStripSize")]
// 0x1b48d0 — _TIFFDefaultStripSize — _TIFFDefaultStripSize
pub fn stub_0x1b48d0() {
    // IDA 0x1b48d0: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFComputeStrip")]
// 0x1b48d8 — _TIFFComputeStrip — _TIFFComputeStrip
pub fn stub_0x1b48d8() {
    // IDA 0x1b48d8: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_multiply_1")]
// 0x1b4944 — _multiply_1 — _multiply_1
pub fn stub_0x1b4944() {
    // IDA 0x1b4944: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFOldScanlineSize")]
// 0x1b49a4 — _TIFFOldScanlineSize — _TIFFOldScanlineSize
pub fn stub_0x1b49a4() {
    // IDA 0x1b49a4: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFNumberOfStrips")]
// 0x1b4a08 — _TIFFNumberOfStrips — _TIFFNumberOfStrips
pub fn stub_0x1b4a08() {
    // IDA 0x1b4a08: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_summarize")]
// 0x1b4a68 — _summarize — _summarize
pub fn stub_0x1b4a68() {
    // IDA 0x1b4a68: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFNewScanlineSize")]
// 0x1b4a7c — _TIFFNewScanlineSize — _TIFFNewScanlineSize
pub fn stub_0x1b4a7c() {
    // IDA 0x1b4a7c: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFScanlineSize")]
// 0x1b4bb8 — _TIFFScanlineSize — _TIFFScanlineSize
pub fn stub_0x1b4bb8() {
    // IDA 0x1b4bb8: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "__TIFFDefaultStripSize")]
// 0x1b4d80 — __TIFFDefaultStripSize — __TIFFDefaultStripSize
pub fn stub_0x1b4d80() {
    // IDA 0x1b4d80: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFVStripSize")]
// 0x1b4dbc — _TIFFVStripSize — _TIFFVStripSize
pub fn stub_0x1b4dbc() {
    // IDA 0x1b4dbc: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFStripSize")]
// 0x1b4f5c — _TIFFStripSize — _TIFFStripSize
pub fn stub_0x1b4f5c() {
    // IDA 0x1b4f5c: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFSwabShort")]
// 0x1b4f70 — _TIFFSwabShort — _TIFFSwabShort
pub fn stub_0x1b4f70() {
    // IDA 0x1b4f70: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFSwabLong")]
// 0x1b4f84 — _TIFFSwabLong — _TIFFSwabLong
pub fn stub_0x1b4f84() {
    // IDA 0x1b4f84: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFSwabArrayOfShort")]
// 0x1b4fa8 — _TIFFSwabArrayOfShort — _TIFFSwabArrayOfShort
pub fn stub_0x1b4fa8() {
    // IDA 0x1b4fa8: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFSwabArrayOfTriples")]
// 0x1b5118 — _TIFFSwabArrayOfTriples — _TIFFSwabArrayOfTriples
pub fn stub_0x1b5118() {
    // IDA 0x1b5118: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFSwabArrayOfLong")]
// 0x1b5288 — _TIFFSwabArrayOfLong — _TIFFSwabArrayOfLong
pub fn stub_0x1b5288() {
    // IDA 0x1b5288: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}
