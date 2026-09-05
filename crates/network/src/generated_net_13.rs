//! network generated_net_13 — auto-generated, do not edit manually
//! Filter: RakNet|Network|Replicator -> 5119 complete, batch EA-sorted asc 120 gap filler (global, since filtered complete)
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +120 stubs | range 0x12bba0..0x13c320 | 22759->22879 distinct (rbx_core::SharedPtr not boost) — preserves ea + mangled + demangled for rg

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
/// JPEG marker emission sink (IDA 0x12bc6c `emit_byte`: append + refill; dry → error 25).
#[derive(Clone, Debug, Default)]
pub struct MarkerEmit {
    pub out: Vec<u8>,
    pub free: usize,
}

impl MarkerEmit {
    /// Store one marker byte; dry dest → error 25 (IDA 0x12bc6c).
    fn byte(&mut self, b: u8, refill: &mut dyn FnMut(&mut MarkerEmit) -> bool) {
        self.out.push(b);
        if self.free <= 1 {
            self.free = 0;
            if !refill(self) {
                panic!("emit_byte: error 25");
            }
        } else {
            self.free -= 1;
        }
    }
}

/// JFIF APP0 header fields (IDA 0x12c3d8).
#[derive(Clone, Copy, Debug)]
pub struct JfifHeader {
    pub major: u8,
    pub minor: u8,
    pub units: u8,
    pub x_density: u16,
    pub y_density: u16,
}

/// SOF component entry (IDA 0x12c294).
#[derive(Clone, Copy, Debug)]
pub struct SofComp {
    pub id: u8,
    pub h: u8,
    pub v: u8,
    pub q: u8,
}

/// Scan component table selectors (IDA 0x12c8ec SOS tail).
#[derive(Clone, Copy, Debug)]
pub struct ScanComp {
    pub id: u8,
    pub dc_tbl: u8,
    pub ac_tbl: u8,
}

/// Table set emitted per scan component (IDA 0x12c8ec DHT prologue).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScanTables {
    DcAc,
    AcOnly,
    None,
}

/// Marker-writer operations installed by `jinit_marker_writer` (IDA 0x12d188).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MarkerOp {
    FileHeader,
    FrameHeader,
    ScanHeader,
    FileTrailer,
    TablesOnly,
    MarkerHeader,
    MarkerByte,
}

/// Master-controller pass state (IDA 0x12d538/0x12d908: +16 state, +20 next scan, +28 completed).
#[derive(Clone, Copy, Debug, Default)]
pub struct MasterPass {
    pub state: u32,
    pub next_scan: u32,
    pub completed: u32,
}

/// Compressor setup hooks in `prepare_for_pass` order (IDA 0x12d908).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrepHook {
    SelectScan,
    PerScanSetup,
    EntropyStart(bool),
    CoefStart(i32),
    ColorStart,
    DownsampStart,
    MainStart,
    FdctStart,
    MainSetSuspended(bool),
    MarkerFileHeader,
    MarkerScanTables,
}
/// Master-controller init result (IDA 0x12ded0).
#[derive(Clone, Debug)]
pub struct MasterInit {
    pub max_h: u32,
    pub max_v: u32,
    pub down_w: Vec<u32>,
    pub down_h: Vec<u32>,
    pub mcu_w: Vec<u32>,
    pub mcu_h: Vec<u32>,
    pub mcu_cols: u32,
    pub progressive: bool,
    pub num_scans: u32,
    pub total_passes: u32,
    pub pass_state: u32,
}
/// Scan script entry: component set plus spectral/refinement band (IDA 0x12d224 script path).
#[derive(Clone, Debug)]
pub struct ScanScript {
    pub comps: Vec<usize>,
    pub ss: u32,
    pub se: u32,
    pub ah: u32,
    pub al: u32,
}

/// Spectral/refinement band selected by `select_scan_parameters` (IDA 0x12d224: Ss, Se, Ah, Al).
#[derive(Clone, Copy, Debug)]
pub struct ScanParams {
    pub ss: u32,
    pub se: u32,
    pub ah: u32,
    pub al: u32,
}

/// Master-controller component sampling factors (IDA 0x12ded0).
#[derive(Clone, Copy, Debug)]
pub struct MasterCompIn {
    pub h: u32,
    pub v: u32,
}


/// Quantization table allocation (IDA 0x12f2bc: 64 entries + sent flag).
#[derive(Clone, Debug)]
pub struct QuantTable {
    pub q: [u16; 64],
    pub sent: bool,
}

/// Huffman table allocation (IDA 0x12f2e4: counts + symbols + sent flag).
#[derive(Clone, Debug)]
pub struct HuffTableSpec {
    pub bits: [u8; 17],
    pub vals: [u8; 256],
    pub sent: bool,
}

impl Default for HuffTableSpec {
    fn default() -> Self {
        HuffTableSpec { bits: [0; 17], vals: [0; 256], sent: false }
    }
}

/// Component setup written by `jpeg_set_colorspace` (IDA 0x12f34c).
#[derive(Clone, Copy, Debug)]
pub struct CompSetup {
    pub id: u8,
    pub h: u8,
    pub v: u8,
    pub q: u8,
    pub dc_tbl: u8,
    pub ac_tbl: u8,
}

/// Colorspace setup written by `jpeg_set_colorspace` (IDA 0x12f34c).
#[derive(Clone, Debug)]
pub struct ColorspaceSetup {
    pub num_comps: u32,
    pub comps: Vec<CompSetup>,
    pub jfif: bool,
    pub adobe: bool,
}

/// Compressor defaults written by `jpeg_set_defaults` (IDA 0x12ff1c).
#[derive(Clone, Copy, Debug)]
pub struct JpegDefaults {
    pub arith_code: bool,
    pub dc_tables: [u8; 16],
    pub ac_tables: [u8; 16],
    pub q_tables: [u8; 16],
    pub jfif: JfifHeader,
}

/// Integer division with rounding up (IDA `jdiv_round_up`).
pub fn jdiv_round_up(a: u32, b: u32) -> u32 {
    (a + b - 1) / b
}

/// Standard luminance base quant table (IJG, IDA `std_luminance_quant_tbl`).
pub const STD_LUMINANCE_QT: [u32; 64] = [
    16, 11, 10, 16, 24, 40, 51, 61, 12, 12, 14, 19, 26, 58, 60, 55,
    14, 13, 16, 24, 40, 57, 69, 56, 14, 17, 22, 29, 51, 87, 80, 62,
    18, 22, 37, 56, 68, 109, 103, 77, 24, 35, 55, 64, 81, 104, 113, 92,
    49, 64, 78, 87, 103, 121, 120, 101, 72, 92, 95, 98, 112, 100, 103, 99,
];

/// Standard chrominance base quant table (IJG, IDA `std_chrominance_quant_tbl`).
pub const STD_CHROMINANCE_QT: [u32; 64] = [
    17, 18, 24, 47, 99, 99, 99, 99, 18, 21, 26, 66, 99, 99, 99, 99,
    24, 26, 56, 99, 99, 99, 99, 99, 47, 66, 99, 99, 99, 99, 99, 99,
    99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99,
    99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99,
];

/// Standard DC luminance Huffman counts (IJG, IDA `bits_dc_luminance_4634`).
pub const BITS_DC_LUMINANCE: [u8; 17] = [0, 0, 1, 5, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0];
/// Standard DC luminance Huffman symbols (IJG, IDA `val_dc_luminance_4635`).
pub const VALS_DC_LUMINANCE: [u8; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
/// Standard DC chrominance Huffman counts (IJG, IDA `bits_dc_chrominance_4636`).
pub const BITS_DC_CHROMINANCE: [u8; 17] = [0, 0, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0];
/// Standard DC chrominance Huffman symbols (IJG, IDA `val_dc_chrominance_4637`).
pub const VALS_DC_CHROMINANCE: [u8; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
/// Standard AC luminance Huffman counts (IJG, IDA `bits_ac_luminance_4638`).
pub const BITS_AC_LUMINANCE: [u8; 17] = [0, 0, 2, 1, 3, 3, 2, 4, 3, 5, 5, 4, 4, 0, 0, 1, 0x7d];
/// Standard AC luminance Huffman symbols (IJG, IDA `val_ac_luminance_4639`).
pub const VALS_AC_LUMINANCE: [u8; 162] = [
    0x01, 0x02, 0x03, 0x00, 0x04, 0x11, 0x05, 0x12, 0x21, 0x31, 0x41, 0x06, 0x13, 0x51, 0x61, 0x07,
    0x22, 0x71, 0x14, 0x32, 0x81, 0x91, 0xa1, 0x08, 0x23, 0x42, 0xb1, 0xc1, 0x15, 0x52, 0xd1, 0xf0,
    0x24, 0x33, 0x62, 0x72, 0x82, 0x09, 0x0a, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x25, 0x26, 0x27, 0x28,
    0x29, 0x2a, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49,
    0x4a, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69,
    0x6a, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89,
    0x8a, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7,
    0xa8, 0xa9, 0xaa, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xc2, 0xc3, 0xc4, 0xc5,
    0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xe1, 0xe2,
    0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8,
    0xf9, 0xfa,
];
/// Standard AC chrominance Huffman counts (IJG, IDA `bits_ac_chrominance_4640`).
pub const BITS_AC_CHROMINANCE: [u8; 17] = [0, 0, 2, 1, 2, 4, 4, 3, 4, 7, 5, 4, 4, 0, 1, 2, 0x77];
/// Standard AC chrominance Huffman symbols (IJG, IDA `val_ac_chrominance_4641`).
pub const VALS_AC_CHROMINANCE: [u8; 162] = [
    0x00, 0x01, 0x02, 0x03, 0x11, 0x04, 0x05, 0x21, 0x31, 0x06, 0x12, 0x41, 0x51, 0x07, 0x61, 0x71,
    0x13, 0x22, 0x32, 0x81, 0x08, 0x14, 0x42, 0x91, 0xa1, 0xb1, 0xc1, 0x09, 0x23, 0x33, 0x52, 0xf0,
    0x15, 0x62, 0x72, 0xd1, 0x0a, 0x16, 0x24, 0x34, 0xe1, 0x25, 0xf1, 0x17, 0x18, 0x19, 0x1a, 0x26,
    0x27, 0x28, 0x29, 0x2a, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48,
    0x49, 0x4a, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68,
    0x69, 0x6a, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87,
    0x88, 0x89, 0x8a, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0xa2, 0xa3, 0xa4, 0xa5,
    0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xc2, 0xc3,
    0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda,
    0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8,
    0xf9, 0xfa,
];

/// Prep-controller start state (IDA 0x130150: rows available, row cursor, rows per group).
#[derive(Clone, Copy, Debug, Default)]
pub struct PrepState {
    pub rows_avail: u32,
    pub row_ctr: u32,
    pub rows_per_group: u32,
}

/// Prep-controller method (IDA 0x1301a4).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrepMethod {
    Context,
    Data,
}

/// Prep-controller buffers (IDA 0x1301a4: per-component sample-row groups).
#[derive(Clone, Debug)]
pub struct PrepController {
    pub method: PrepMethod,
    pub bufs: Vec<Vec<i16>>,
}

/// Context-row pump state (IDA 0x1306ec: rows available/filled/cursor per group).
#[derive(Clone, Debug, Default)]
pub struct PrepContext {
    pub rows_avail: u32,
    pub row_ctr: u32,
    pub rows_filled: u32,
    pub rows_per_group: u32,
    pub topped: bool,
}
// 0x12bba0 — _jinit_c_main_controller
#[doc(alias = "_jinit_c_main_controller")]
pub fn stub_12bba0(raw_data: bool, need_full_buffer: bool, comp_dims: &[(usize, usize)], alloc: &mut dyn FnMut(usize, usize) -> Vec<i16>) -> Vec<Vec<i16>> { // IDA 0x12bba0: install start_pass_main; raw input → no buffers; full-buffer request → error 3; else one sample-row buffer per component.
    if raw_data {
        return Vec::new();
    }
    if need_full_buffer {
        panic!("jinit_c_main_controller: full-buffer main pass unsupported (3)");
    }
    comp_dims.iter().map(|&(w, h)| alloc(w, h)).collect()
}

// 0x12bc6c — _emit_byte_0
#[doc(alias = "_emit_byte_0")]
pub fn stub_12bc6c(st: &mut MarkerEmit, byte: u8, refill: &mut dyn FnMut(&mut MarkerEmit) -> bool) { // IDA 0x12bc6c: store one byte; dry dest → error 25.
    st.byte(byte, refill);
}

// 0x12bccc — _emit_marker
#[doc(alias = "_emit_marker")]
pub fn stub_12bccc(st: &mut MarkerEmit, marker: u8, refill: &mut dyn FnMut(&mut MarkerEmit) -> bool) { // IDA 0x12bccc: emit 0xFF + marker.
    stub_12bc6c(st, 0xFF, refill);
    stub_12bc6c(st, marker, refill);
}

// 0x12bcf4 — _emit_2bytes
#[doc(alias = "_emit_2bytes")]
pub fn stub_12bcf4(st: &mut MarkerEmit, value: u16, refill: &mut dyn FnMut(&mut MarkerEmit) -> bool) { // IDA 0x12bcf4: emit big-endian 2 bytes.
    stub_12bc6c(st, (value >> 8) as u8, refill);
    stub_12bc6c(st, (value & 0xFF) as u8, refill);
}

// 0x12bd20 — _emit_dqt
#[doc(alias = "_emit_dqt")]
pub fn stub_12bd20(st: &mut MarkerEmit, tbl_no: u8, table: Option<&[u16; 64]>, sent: &mut bool, refill: &mut dyn FnMut(&mut MarkerEmit) -> bool) -> bool { // IDA 0x12bd20: missing table → error 54; any entry > 0xFF → 16-bit precision; DQT marker once (length 67/131, zigzag order); returns precision.
    let t = match table {
        Some(t) => t,
        None => panic!("emit_dqt: missing quantization table (54)"),
    };
    let prec16 = t.iter().any(|&v| v > 0xFF);
    if !*sent {
        stub_12bccc(st, 0xDB, refill);
        stub_12bcf4(st, if prec16 { 131 } else { 67 }, refill);
        stub_12bc6c(st, 16 * (prec16 as u8) + tbl_no, refill);
        for k in 0..64 {
            let v = t[crate::generated_net_12::JPEG_NATURAL_ORDER[k] as usize];
            if prec16 {
                stub_12bc6c(st, (v >> 8) as u8, refill);
            }
            stub_12bc6c(st, (v & 0xFF) as u8, refill);
        }
        *sent = true;
    }
    prec16
}

// 0x12bf54 — _emit_dht
#[doc(alias = "_emit_dht")]
pub fn stub_12bf54(st: &mut MarkerEmit, tbl_no: u8, is_ac: bool, table: Option<&HuffTableSpec>, sent: &mut bool, refill: &mut dyn FnMut(&mut MarkerEmit) -> bool) { // IDA 0x12bf54: missing table → error 52 (AC numbers +16); DHT marker once (length 19 + symbols, info byte, 16 counts, symbols); IDA unrolls both loops (IDA returns the last byte result).
    let t = match table {
        Some(t) => t,
        None => panic!("emit_dht: missing Huffman table (52)"),
    };
    if !*sent {
        let total: usize = t.bits[1..=16].iter().map(|&b| b as usize).sum();
        stub_12bccc(st, 0xC4, refill);
        stub_12bcf4(st, (total + 19) as u16, refill);
        stub_12bc6c(st, (if is_ac { 16 } else { 0 }) + tbl_no, refill);
        for l in 1..=16 {
            stub_12bc6c(st, t.bits[l], refill);
        }
        for &s in t.vals.iter().take(total) {
            stub_12bc6c(st, s, refill);
        }
        *sent = true;
    }
}

// 0x12c294 — _emit_sof
#[doc(alias = "_emit_sof")]
pub fn stub_12c294(st: &mut MarkerEmit, marker: u8, precision: u8, height: u32, width: u32, comps: &[SofComp], refill: &mut dyn FnMut(&mut MarkerEmit) -> bool) { // IDA 0x12c294: SOF marker; length 3n + 8; dims over 0xFFFF → error 42; precision, dims, per-component id/sampling/quant entries.
    stub_12bccc(st, marker, refill);
    stub_12bcf4(st, 3 * comps.len() as u16 + 8, refill);
    if height > 0xFFFF || width > 0xFFFF {
        panic!("emit_sof: image too large (42)");
    }
    stub_12bc6c(st, precision, refill);
    stub_12bcf4(st, height as u16, refill);
    stub_12bcf4(st, width as u16, refill);
    stub_12bc6c(st, comps.len() as u8, refill);
    for c in comps {
        stub_12bc6c(st, c.id, refill);
        stub_12bc6c(st, 16 * c.h + c.v, refill);
        stub_12bc6c(st, c.q, refill);
    }
}

// 0x12c380 — _write_marker_header
#[doc(alias = "_write_marker_header")]
pub fn stub_12c380(st: &mut MarkerEmit, marker: u8, length: u32, refill: &mut dyn FnMut(&mut MarkerEmit) -> bool) { // IDA 0x12c380: length over 0xFFFD → error 12; marker + length + 2.
    if length > 0xFFFD {
        panic!("write_marker_header: bad length (12)");
    }
    stub_12bccc(st, marker, refill);
    stub_12bcf4(st, (length + 2) as u16, refill);
}

// 0x12c3d4 — _write_marker_byte
#[doc(alias = "_write_marker_byte")]
pub fn stub_12c3d4(st: &mut MarkerEmit, byte: u8, refill: &mut dyn FnMut(&mut MarkerEmit) -> bool) { // IDA 0x12c3d4: thunk to emit_byte.
    stub_12bc6c(st, byte, refill);
}

// 0x12c3d8 — _write_file_header
#[doc(alias = "_write_file_header")]
pub fn stub_12c3d8(st: &mut MarkerEmit, jfif: Option<JfifHeader>, adobe_space: Option<i32>, refill: &mut dyn FnMut(&mut MarkerEmit) -> bool) { // IDA 0x12c3d8: SOI; optional JFIF APP0 (length 16); optional Adobe APP14 (length 14, transform 1 for YCbCr space 3, 2 for YCCK space 5).
    stub_12bccc(st, 0xD8, refill);
    if let Some(j) = jfif {
        stub_12bccc(st, 0xE0, refill);
        stub_12bcf4(st, 16, refill);
        for &b in b"JFIF\0" {
            stub_12bc6c(st, b, refill);
        }
        stub_12bc6c(st, j.major, refill);
        stub_12bc6c(st, j.minor, refill);
        stub_12bc6c(st, j.units, refill);
        stub_12bcf4(st, j.x_density, refill);
        stub_12bcf4(st, j.y_density, refill);
        stub_12bc6c(st, 0, refill);
        stub_12bc6c(st, 0, refill);
    }
    if let Some(space) = adobe_space {
        stub_12bccc(st, 0xEE, refill);
        stub_12bcf4(st, 14, refill);
        for &b in b"Adobe" {
            stub_12bc6c(st, b, refill);
        }
        stub_12bcf4(st, 100, refill);
        stub_12bcf4(st, 0, refill);
        stub_12bcf4(st, 0, refill);
        stub_12bc6c(st, if space == 3 { 1 } else if space == 5 { 2 } else { 0 }, refill);
    }
}

// 0x12c560 — _write_frame_header
#[doc(alias = "_write_frame_header")]
pub fn stub_12c560(prec16_tables: u32, single_sampled: bool, arith_code: bool, progressive: bool, emit_dqt: &mut dyn FnMut(), emit_sof: &mut dyn FnMut(u8)) { // IDA 0x12c560: per-component DQT emission (caller); 16-bit tables with all-1:1 sampling → error 77; arith → 0xC9/0xCA, progressive → 0xC2, all-1:1 → 0xC0 else 0xC1.
    emit_dqt();
    let marker = if arith_code {
        if progressive { 0xCA } else { 0xC9 }
    } else if progressive {
        0xC2
    } else {
        if prec16_tables != 0 && single_sampled {
            panic!("write_frame_header: 12-bit data with 1:1 sampling unsupported (77)");
        }
        if single_sampled { 0xC0 } else { 0xC1 }
    };
    emit_sof(marker);
}

// 0x12c8ec — _write_scan_header
#[doc(alias = "_write_scan_header")]
pub fn stub_12c8ec(arith_code: bool, progressive: bool, ss: u32, se: u32, ah: u32, al: u32, comps: &[ScanComp], restart_interval: u32, last_dri: &mut u32, emit_tables: &mut dyn FnMut(ScanTables, usize), emit_dac: &mut dyn FnMut(&[u8], &[u8]), st: &mut MarkerEmit, refill: &mut dyn FnMut(&mut MarkerEmit) -> bool) { // IDA 0x12c8ec: per-scan DHT (baseline DC+AC; first-scan DC+AC; AC-first AC-only; refine none) or DAC bitmaps; DRI on change; SOS with progressive table-selector folding.
    let set = if !progressive {
        ScanTables::DcAc
    } else if ss != 0 {
        ScanTables::AcOnly
    } else if ah == 0 {
        ScanTables::DcAc
    } else {
        ScanTables::None
    };
    if !arith_code {
        for (i, _) in comps.iter().enumerate() {
            emit_tables(set, i);
        }
    } else {
        let mut dc_used = [false; 16];
        let mut ac_used = [false; 16];
        for c in comps {
            dc_used[c.dc_tbl as usize] = true;
            ac_used[c.ac_tbl as usize] = true;
        }
        let dc: Vec<u8> = (0..16).filter(|&i| dc_used[i]).map(|i| i as u8).collect();
        let ac: Vec<u8> = (0..16).filter(|&i| ac_used[i]).map(|i| i as u8).collect();
        emit_dac(&dc, &ac);
    }
    if restart_interval != *last_dri {
        stub_12bccc(st, 0xDD, refill);
        stub_12bcf4(st, 4, refill);
        stub_12bcf4(st, restart_interval as u16, refill);
        *last_dri = restart_interval;
    }
    stub_12bccc(st, 0xDA, refill);
    stub_12bcf4(st, 2 * comps.len() as u16 + 6, refill);
    stub_12bc6c(st, comps.len() as u8, refill);
    for c in comps {
        stub_12bc6c(st, c.id, refill);
        let (dc, ac) = if !progressive {
            (c.dc_tbl, c.ac_tbl)
        } else if ss != 0 {
            (0, c.ac_tbl)
        } else if ah == 0 || arith_code {
            (c.dc_tbl, 0)
        } else {
            (0, c.ac_tbl)
        };
        stub_12bc6c(st, 16 * dc + ac, refill);
    }
    stub_12bc6c(st, ss as u8, refill);
    stub_12bc6c(st, se as u8, refill);
    stub_12bc6c(st, (ah * 16 + al) as u8, refill);
}

// 0x12d010 — _write_file_trailer
#[doc(alias = "_write_file_trailer")]
pub fn stub_12d010(st: &mut MarkerEmit, refill: &mut dyn FnMut(&mut MarkerEmit) -> bool) { // IDA 0x12d010: EOI marker.
    stub_12bccc(st, 0xD9, refill);
}

// 0x12d018 — _write_tables_only
#[doc(alias = "_write_tables_only")]
pub fn stub_12d018(quant_present: &[bool; 4], dc_present: &[bool; 4], ac_present: &[bool; 4], arith_code: bool, emit_dqt: &mut dyn FnMut(usize), emit_dht: &mut dyn FnMut(usize, bool), st: &mut MarkerEmit, refill: &mut dyn FnMut(&mut MarkerEmit) -> bool) { // IDA 0x12d018: SOI; DQT for present tables 0..3; DHT pairs per present table (baseline only); EOI.
    stub_12bccc(st, 0xD8, refill);
    for (i, &p) in quant_present.iter().enumerate() {
        if p {
            emit_dqt(i);
        }
    }
    if !arith_code {
        for i in 0..4 {
            if dc_present[i] {
                emit_dht(i, false);
            }
            if ac_present[i] {
                emit_dht(i, true);
            }
        }
    }
    stub_12bccc(st, 0xD9, refill);
}

// 0x12d188 — _jinit_marker_writer
#[doc(alias = "_jinit_marker_writer")]
pub fn stub_12d188() -> [MarkerOp; 7] { // IDA 0x12d188: install the 7 marker-writer operations.
    [MarkerOp::FileHeader, MarkerOp::FrameHeader, MarkerOp::ScanHeader, MarkerOp::FileTrailer, MarkerOp::TablesOnly, MarkerOp::MarkerHeader, MarkerOp::MarkerByte]
}

// 0x12d224 — _select_scan_parameters
// type: int __fastcall(_DWORD *)
#[doc(alias = "_select_scan_parameters")]
pub fn stub_12d224(num_comps: usize, script: Option<&ScanScript>) -> (Vec<usize>, ScanParams) { // IDA 0x12d224: no script → first-n components, full band (Ss 0, Se 63); script entry → its components and band. Over 4 scan components → error 27.
    match script {
        None => {
            if num_comps > 4 {
                panic!("select_scan_parameters: too many components (27)");
            }
            ((0..num_comps).collect(), ScanParams { ss: 0, se: 63, ah: 0, al: 0 })
        }
        Some(e) => (e.comps.clone(), ScanParams { ss: e.ss, se: e.se, ah: e.ah, al: e.al }),
    }
}

// 0x12d500 — _pass_startup
#[doc(alias = "_pass_startup")]
pub fn stub_12d500(clear: &mut dyn FnMut(), file_header: &mut dyn FnMut(), tables: &mut dyn FnMut()) { // IDA 0x12d500: clear the suspension flag, then the file-header and scan-tables hooks.
    clear();
    file_header();
    tables();
}

// 0x12d538 — _finish_pass_master
#[doc(alias = "_finish_pass_master")]
pub fn stub_12d538(s: &mut MasterPass, optimize: bool) -> u32 { // IDA 0x12d538: pass-state machine; returns the pre-increment next-scan number.
    match s.state {
        1 => s.state = 2,
        0 => {
            s.state = 2;
            if !optimize {
                s.completed += 1;
            }
        }
        2 => {
            if optimize {
                s.state = 1;
            }
            s.completed += 1;
        }
        _ => {}
    }
    let r = s.next_scan;
    s.next_scan += 1;
    r
}

// 0x12d5bc — _per_scan_setup
#[doc(alias = "_per_scan_setup")]
pub fn stub_12d5bc(image_w: u32, image_h: u32, max_h: u32, max_v: u32, comps: &[(u32, u32, u32, u32)], restart_rows: u32) -> (u32, u32, Vec<usize>, u32) { // IDA 0x12d5bc: single component → its dims, order [0]; else MCU cols/rows with per-component block interleave (over 10 blocks → error 14, over 4 comps → error 27); restart rows clamped to 0xFFFF MCUs.
    let (cols, rows, order) = if comps.len() == 1 {
        let (_, _, w, h) = comps[0];
        (w, h, vec![0])
    } else {
        if comps.len() > 4 {
            panic!("per_scan_setup: too many components (27)");
        }
        let cols = jdiv_round_up(image_w, 8 * max_h);
        let rows = jdiv_round_up(image_h, 8 * max_v);
        let mut order = Vec::new();
        let mut total = 0u32;
        for (i, &(h, v, _, _)) in comps.iter().enumerate() {
            let n = h * v;
            total += n;
            if total > 10 {
                panic!("per_scan_setup: too many blocks (14)");
            }
            for _ in 0..n {
                order.push(i);
            }
        }
        (cols, rows, order)
    };
    let clamped = if restart_rows > 0 { (restart_rows * cols).min(0xFFFF) } else { restart_rows };
    (cols, rows, order, clamped)
}

// 0x12d908 — _prepare_for_pass
#[doc(alias = "_prepare_for_pass")]
pub fn stub_12d908(master: &mut MasterPass, total: u32, optimize: bool, raw: bool, scans_gt1: bool, ss: u32, ah: u32, hook: &mut dyn FnMut(PrepHook)) -> (bool, bool) { // IDA 0x12d908: pass 0 → full init chain (first = !optimize); pass 1 → fast path unless a refine head (reselect + bump); pass 2 → reselect tail; other → error 49. Returns (first, last).
    match master.state {
        0 => {
            hook(PrepHook::SelectScan);
            hook(PrepHook::PerScanSetup);
            if !raw {
                hook(PrepHook::ColorStart);
                hook(PrepHook::DownsampStart);
                hook(PrepHook::MainStart);
            }
            hook(PrepHook::FdctStart);
            hook(PrepHook::EntropyStart(optimize));
            hook(PrepHook::CoefStart(if scans_gt1 { 3 } else { 0 }));
            hook(PrepHook::MainSetSuspended(false));
            master.completed = 0;
        }
        1 => {
            hook(PrepHook::SelectScan);
            hook(PrepHook::PerScanSetup);
            if ss != 0 || ah == 0 {
                hook(PrepHook::EntropyStart(true));
                hook(PrepHook::CoefStart(2));
            } else {
                master.state = 2;
                master.next_scan += 1;
                if !optimize {
                    hook(PrepHook::SelectScan);
                    hook(PrepHook::PerScanSetup);
                }
                hook(PrepHook::EntropyStart(false));
                hook(PrepHook::CoefStart(2));
                hook(PrepHook::MarkerFileHeader);
                hook(PrepHook::MarkerScanTables);
            }
        }
        2 => {
            if !optimize {
                hook(PrepHook::SelectScan);
                hook(PrepHook::PerScanSetup);
            }
            hook(PrepHook::EntropyStart(false));
            hook(PrepHook::CoefStart(2));
            hook(PrepHook::MarkerFileHeader);
            hook(PrepHook::MarkerScanTables);
        }
        _ => panic!("prepare_for_pass: bad pass type (49)"),
    }
    let first = master.state == 0 && !optimize;
    let last = master.next_scan == total - 1;
    (first, last)
}

// 0x12db18 — _jpeg_calc_jpeg_dimensions
#[doc(alias = "_jpeg_calc_jpeg_dimensions")]
pub fn stub_12db18(w: u32, h: u32, p: u32, q: u32) -> (u32, u32, u32) { // IDA 0x12db18: 16-step ratio ladder between the sampling factors; returns (out_w, out_h, factor).
    if p >= 8 * q {
        return (8 * w, 8 * h, 1);
    }
    if p >= 4 * q {
        return (4 * w, 4 * h, 2);
    }
    if 8 * q <= 3 * p {
        return (jdiv_round_up(2 * w, 3) + 2 * w, jdiv_round_up(2 * h, 3) + 2 * h, 3);
    }
    if p >= 2 * q {
        return (2 * w, 2 * h, 4);
    }
    if 8 * q <= 5 * p {
        return (jdiv_round_up(3 * w, 5) + w, jdiv_round_up(3 * h, 5) + h, 5);
    }
    if 4 * q <= 3 * p {
        return (jdiv_round_up(w, 3) + w, jdiv_round_up(h, 3) + h, 6);
    }
    if 8 * q <= 7 * p {
        return (w + jdiv_round_up(w, 7), h + jdiv_round_up(h, 7), 7);
    }
    if p >= q {
        return (w, h, 8);
    }
    if 8 * q <= 9 * p {
        return (jdiv_round_up(8 * w, 9), jdiv_round_up(8 * h, 9), 9);
    }
    if 4 * q <= 5 * p {
        return (jdiv_round_up(4 * w, 5), jdiv_round_up(4 * h, 5), 10);
    }
    if 8 * q <= 11 * p {
        return (jdiv_round_up(8 * w, 11), jdiv_round_up(8 * h, 11), 11);
    }
    if 3 * p < 2 * q {
        if 8 * q <= 13 * p {
            return (jdiv_round_up(8 * w, 13), jdiv_round_up(8 * h, 13), 13);
        }
        if 4 * q <= 7 * p {
            return (jdiv_round_up(4 * w, 7), jdiv_round_up(4 * h, 7), 14);
        }
        if 8 * q <= 15 * p {
            return (jdiv_round_up(8 * w, 15), jdiv_round_up(8 * h, 15), 15);
        }
        return (jdiv_round_up(w, 2), jdiv_round_up(h, 2), 16);
    }
    (jdiv_round_up(2 * w, 3), jdiv_round_up(2 * h, 3), 12)
}

// 0x12ded0 — _jinit_c_master_control
#[doc(alias = "_jinit_c_master_control")]
pub fn stub_12ded0(image_w: u32, image_h: u32, precision: u8, sampling: &[MasterCompIn], factor: u32, fancy: bool, script: Option<&[ScanScript]>, optimize: bool, need_full: bool) -> MasterInit { // IDA 0x12ded0: validate dims/precision/sampling (errors 33/42/16/27/19); downsampled + MCU dims per component; script validation (errors 27/20/18/46); scan/pass counts.
    assert!(factor != 0 && image_w > 0 && !sampling.is_empty(), "jinit_c_master_control: bad dimensions (33)");
    assert!(image_w <= 65500 && image_h <= 65500, "jinit_c_master_control: image too large (42)");
    assert!(precision == 8, "jinit_c_master_control: bad precision (16): {}", precision);
    assert!(sampling.len() <= 10, "jinit_c_master_control: too many components (27)");
    for c in sampling {
        assert!((1..=4).contains(&c.h) && (1..=4).contains(&c.v), "jinit_c_master_control: bad sampling (19)");
    }
    let max_h = sampling.iter().map(|c| c.h).max().unwrap_or(1);
    let max_v = sampling.iter().map(|c| c.v).max().unwrap_or(1);
    let lim = if fancy { 8 } else { 4 };
    let mut down_w = Vec::with_capacity(sampling.len());
    let mut down_h = Vec::with_capacity(sampling.len());
    let mut mcu_w = Vec::with_capacity(sampling.len());
    let mut mcu_h = Vec::with_capacity(sampling.len());
    for c in sampling {
        let mut dh = 1u32;
        while factor * dh <= lim && max_h % (dh * 2 * c.h) == 0 {
            dh *= 2;
        }
        let mut fh = factor * dh;
        let mut dv = 1u32;
        while factor * dv <= lim && max_v % (dv * 2 * c.v) == 0 {
            dv *= 2;
        }
        let mut fv = factor * dv;
        if fh > 2 * fv {
            fh = 2 * fv;
        } else if 2 * fh < fv {
            fv = 2 * fh;
        }
        down_w.push(jdiv_round_up(c.h * image_w, 8 * max_h));
        down_h.push(jdiv_round_up(c.v * image_h, 8 * max_v));
        mcu_w.push(jdiv_round_up(image_w * c.h * fh, 8 * max_h));
        mcu_h.push(jdiv_round_up(image_h * c.v * fv, 8 * max_v));
    }
    let mcu_cols = jdiv_round_up(image_w, 8 * max_v);
    let mut covered = vec![false; sampling.len()];
    let (progressive, num_scans) = match script {
        None => (false, 1),
        Some(scans) => {
            assert!(!scans.is_empty(), "jinit_c_master_control: bad scan script (20)");
            let arith_single = scans.len() == 1 && scans[0].ss == 0 && scans[0].se == 63;
            for s in scans.iter() {
                assert!((1..=4).contains(&s.comps.len()), "jinit_c_master_control: bad scan components (27)");
                let mut prev = -1i32;
                for &c in &s.comps {
                    assert!(c < sampling.len() && c as i32 > prev, "jinit_c_master_control: bad scan order (20)");
                    prev = c as i32;
                }
                if arith_single {
                    assert!(s.ss == 0 && s.se == 63 && s.ah == 0 && s.al == 0, "jinit_c_master_control: baseline scan must be full band (18)");
                } else {
                    assert!(s.ss <= 63 && s.se <= 63 && s.ss <= s.se && s.ah <= 10 && s.al <= 10, "jinit_c_master_control: bad scan band (18)");
                    if s.ah != 0 {
                        assert!(s.comps.len() == 1, "jinit_c_master_control: refine needs one component (18)");
                    } else {
                        assert!(s.al == 0, "jinit_c_master_control: bad refine low bits (18)");
                    }
                }
                for &c in &s.comps {
                    covered[c] = true;
                }
            }
            (!arith_single, scans.len() as u32)
        }
    };
    if script.is_some() {
        for (i, &u) in covered.iter().enumerate() {
            assert!(u, "jinit_c_master_control: component {} missing from scans (46)", i);
        }
    }
    let pass_state = if need_full { if optimize { 1 } else { 2 } } else { 0 };
    let total_passes = if optimize { num_scans * 2 } else { num_scans };
    MasterInit { max_h, max_v, down_w, down_h, mcu_w, mcu_h, mcu_cols, progressive, num_scans, total_passes, pass_state }
}

// 0x12f248 — _jpeg_abort
#[doc(alias = "_jpeg_abort")]
pub fn stub_12f248(mem_present: bool, is_compress: bool, free_pool: &mut dyn FnMut() -> i32, set_state: &mut dyn FnMut(u32)) -> i32 { // IDA 0x12f248: free the pool; compress → global_state = 200, else next_input_scan = 100; returns the free result.
    if !mem_present {
        return 0;
    }
    let r = free_pool();
    set_state(if is_compress { 200 } else { 100 });
    r
}

// 0x12f290 — _jpeg_destroy
#[doc(alias = "_jpeg_destroy")]
pub fn stub_12f290(mem_present: bool, free_pool: &mut dyn FnMut() -> i32) -> i32 { // IDA 0x12f290: free the pool via the memory manager (caller clears mem and state); returns the free result.
    if !mem_present {
        return 0;
    }
    free_pool()
}

// 0x12f2bc — _jpeg_alloc_quant_table
// type: int __fastcall(_DWORD)
#[doc(alias = "_jpeg_alloc_quant_table")]
pub fn stub_12f2bc() -> QuantTable { // IDA 0x12f2bc: alloc 130 bytes; sent flag clear.
    QuantTable { q: [0; 64], sent: false }
}

// 0x12f2e4 — _jpeg_alloc_huff_table
#[doc(alias = "_jpeg_alloc_huff_table")]
pub fn stub_12f2e4() -> HuffTableSpec { // IDA 0x12f2e4: alloc 274 bytes; sent flag clear.
    HuffTableSpec::default()
}

// 0x12f30c — _jpeg_quality_scaling
#[doc(alias = "_jpeg_quality_scaling")]
pub fn stub_12f30c(quality: i32) -> i32 { // IDA 0x12f30c: clamp to 1..100; 49 and below → 5000/q, else 200 - 2q.
    if quality <= 0 {
        return 5000;
    }
    let mut q = quality;
    if q > 100 {
        q = 100;
    }
    if q <= 49 {
        5000 / q
    } else {
        200 - 2 * q
    }
}

// 0x12f34c — _jpeg_set_colorspace
#[doc(alias = "_jpeg_set_colorspace")]
pub fn stub_12f34c(state_ok: bool, in_comps: u32, space: i32) -> ColorspaceSetup { // IDA 0x12f34c: state != 100 → error 21; per-colorspace component ids, sampling, tables and JFIF/Adobe flags; unknown space → error 11.
    if !state_ok {
        panic!("jpeg_set_colorspace: bad state (21)");
    }
    match space {
        0 => {
            if in_comps < 1 || in_comps > 10 {
                panic!("jpeg_set_colorspace: too many components (27)");
            }
            let comps = (0..in_comps as u8).map(|i| CompSetup { id: i, h: 1, v: 1, q: 0, dc_tbl: 0, ac_tbl: 0 }).collect();
            ColorspaceSetup { num_comps: in_comps, comps, jfif: false, adobe: false }
        }
        1 => ColorspaceSetup {
            num_comps: 1,
            comps: vec![CompSetup { id: 1, h: 1, v: 1, q: 0, dc_tbl: 0, ac_tbl: 0 }],
            jfif: true,
            adobe: false,
        },
        2 => ColorspaceSetup {
            num_comps: 3,
            comps: vec![
                CompSetup { id: 82, h: 1, v: 1, q: 0, dc_tbl: 0, ac_tbl: 0 },
                CompSetup { id: 71, h: 1, v: 1, q: 1, dc_tbl: 0, ac_tbl: 0 },
                CompSetup { id: 66, h: 1, v: 1, q: 1, dc_tbl: 0, ac_tbl: 0 },
            ],
            jfif: false,
            adobe: true,
        },
        3 => ColorspaceSetup {
            num_comps: 3,
            comps: vec![
                CompSetup { id: 1, h: 2, v: 2, q: 0, dc_tbl: 0, ac_tbl: 0 },
                CompSetup { id: 2, h: 1, v: 1, q: 1, dc_tbl: 1, ac_tbl: 1 },
                CompSetup { id: 3, h: 1, v: 1, q: 1, dc_tbl: 1, ac_tbl: 1 },
            ],
            jfif: true,
            adobe: false,
        },
        4 => ColorspaceSetup {
            num_comps: 4,
            comps: vec![
                CompSetup { id: 67, h: 1, v: 1, q: 0, dc_tbl: 0, ac_tbl: 0 },
                CompSetup { id: 77, h: 1, v: 1, q: 1, dc_tbl: 0, ac_tbl: 0 },
                CompSetup { id: 89, h: 1, v: 1, q: 1, dc_tbl: 0, ac_tbl: 0 },
                CompSetup { id: 75, h: 1, v: 1, q: 1, dc_tbl: 0, ac_tbl: 0 },
            ],
            jfif: false,
            adobe: true,
        },
        5 => ColorspaceSetup {
            num_comps: 4,
            comps: vec![
                CompSetup { id: 1, h: 2, v: 2, q: 0, dc_tbl: 0, ac_tbl: 0 },
                CompSetup { id: 2, h: 1, v: 1, q: 1, dc_tbl: 1, ac_tbl: 1 },
                CompSetup { id: 3, h: 1, v: 1, q: 1, dc_tbl: 1, ac_tbl: 1 },
                CompSetup { id: 4, h: 2, v: 2, q: 1, dc_tbl: 0, ac_tbl: 0 },
            ],
            jfif: false,
            adobe: true,
        },
        _ => panic!("jpeg_set_colorspace: bad colorspace (11)"),
    }
}

// 0x12f6c0 — _jpeg_default_colorspace
#[doc(alias = "_jpeg_default_colorspace")]
pub fn stub_12f6c0(in_comps: u32, set: &mut dyn FnMut(i32)) { // IDA 0x12f6c0: component count → colorspace (unknown → error 10), then the colorspace hook.
    let space = match in_comps {
        0 => 0,
        1 => 1,
        2 | 3 => 3,
        4 => 4,
        5 => 5,
        _ => panic!("jpeg_default_colorspace: bad components (10)"),
    };
    set(space);
}

// 0x12f728 — _fill_a_scan
#[doc(alias = "_fill_a_scan")]
pub fn stub_12f728(comp: usize, ss: u32, se: u32, ah: u32, al: u32) -> ScanScript { // IDA 0x12f728: one single-component scan entry.
    ScanScript { comps: vec![comp], ss, se, ah, al }
}

// 0x12f754 — _fill_scans
#[doc(alias = "_fill_scans")]
pub fn stub_12f754(num_comps: usize, ss: u32, se: u32, ah: u32, al: u32) -> Vec<ScanScript> { // IDA 0x12f754: one single-component scan per component; IDA packs the (n&3) head Duff-style and the body 4-wide.
    (0..num_comps).map(|i| ScanScript { comps: vec![i], ss, se, ah, al }).collect()
}

// 0x12f88c — _fill_dc_scans
#[doc(alias = "_fill_dc_scans")]
pub fn stub_12f88c(num_comps: usize, ah: u32, al: u32) -> Vec<ScanScript> { // IDA 0x12f88c: over 4 components → interleaved full scans; else one DC scan covering all components.
    if num_comps > 4 {
        stub_12f754(num_comps, 0, 0, ah, al)
    } else {
        vec![ScanScript { comps: (0..num_comps).collect(), ss: 0, se: 0, ah, al }]
    }
}

// 0x12f9b4 — _jpeg_simple_progression
#[doc(alias = "_jpeg_simple_progression")]
pub fn stub_12f9b4(state_ok: bool, num_comps: usize, max_samp: u32) -> Vec<ScanScript> { // IDA 0x12f9b4: state != 100 → error 21; 3-component 4:4:4 → the 10-scan luma/chroma script, else DC + AC-first/refine/DC-refine scan groups. IDA sizes the script pool 4n+2/6n/10; the Vec grows as needed.
    if !state_ok {
        panic!("jpeg_simple_progression: bad state (21)");
    }
    let mut out = Vec::new();
    if num_comps == 3 && max_samp == 3 {
        out.extend(stub_12f88c(num_comps, 0, max_samp - 2));
        out.push(stub_12f728(0, 1, 5, 0, 2));
        out.push(stub_12f728(2, 1, 63, 0, 1));
        out.push(stub_12f728(1, 1, 63, 0, 1));
        out.push(stub_12f728(0, 6, 63, 0, 2));
        out.push(stub_12f728(0, 1, 63, 2, 1));
        out.extend(stub_12f88c(num_comps, 1, 0));
        out.push(stub_12f728(2, 1, 63, 1, 0));
        out.push(stub_12f728(1, 1, 63, 1, 0));
        out.push(stub_12f728(0, 1, 63, 1, 0));
    } else {
        out.extend(stub_12f88c(num_comps, 0, 1));
        out.extend(stub_12f754(num_comps, 1, 5, 0, 2));
        out.extend(stub_12f754(num_comps, 6, 63, 0, 2));
        out.extend(stub_12f754(num_comps, 1, 63, 2, 1));
        out.extend(stub_12f88c(num_comps, 1, 0));
        out.extend(stub_12f754(num_comps, 1, 63, 1, 0));
    }
    out
}

// 0x12fc08 — _add_huff_table
// type: int __fastcall(int, int, void *__src)
#[doc(alias = "_add_huff_table")]
pub fn stub_12fc08(bits: &[u8; 17], vals: &[u8]) -> HuffTableSpec { // IDA 0x12fc08: copy the 17 count bytes; over 256 symbols → error 9; copy the symbols; sent = false.
    let total: usize = bits[1..=16].iter().map(|&b| b as usize).sum();
    if total > 256 {
        panic!("add_huff_table: bad Huffman table (9)");
    }
    let mut t = HuffTableSpec { bits: *bits, vals: [0; 256], sent: false };
    t.vals[..total].copy_from_slice(&vals[..total]);
    t
}

// 0x12fd18 — _jpeg_add_quant_table
// type: int __fastcall(_DWORD *, unsigned int, int, int, char)
#[doc(alias = "_jpeg_add_quant_table")]
pub fn stub_12fd18(state_ok: bool, tbl_no: u32, base: &[u32; 64], scale: u32, force_baseline: bool) -> QuantTable { // IDA 0x12fd18: state != 100 → error 21; table > 3 → error 32; scale (base * scale + 50) / 100 clamped to 1..0x7FFF (255 cap for 8-bit); sent = false. IDA unrolls the loop 2-wide.
    if !state_ok {
        panic!("jpeg_add_quant_table: bad state (21)");
    }
    if tbl_no > 3 {
        panic!("jpeg_add_quant_table: bad table number (32)");
    }
    let mut q = [0u16; 64];
    for i in 0..64 {
        let mut v = (base[i] * scale + 50) / 100;
        if v == 0 {
            v = 1;
        } else if v >= 0x7FFF {
            v = 0x7FFF;
        }
        if force_baseline && v > 255 {
            v = 255;
        }
        q[i] = v as u16;
    }
    QuantTable { q, sent: false }
}

// 0x12fe94 — _jpeg_set_linear_quality
#[doc(alias = "_jpeg_set_linear_quality")]
pub fn stub_12fe94(scale: u32, force_baseline: bool, add: &mut dyn FnMut(u32, &[u32; 64], u32, bool) -> QuantTable) -> (QuantTable, QuantTable) { // IDA 0x12fe94: scale both standard tables (luminance 0, chrominance 1).
    (add(0, &STD_LUMINANCE_QT, scale, force_baseline), add(1, &STD_CHROMINANCE_QT, scale, force_baseline))
}

// 0x12fef0 — _jpeg_set_quality
#[doc(alias = "_jpeg_set_quality")]
pub fn stub_12fef0(quality: i32, force_baseline: bool, set_linear: &mut dyn FnMut(u32, bool)) { // IDA 0x12fef0: quality → scaling factor → linear table setup.
    set_linear(stub_12f30c(quality) as u32, force_baseline);
}

// 0x12ff1c — _jpeg_set_defaults
#[doc(alias = "_jpeg_set_defaults")]
pub fn stub_12ff1c(state_ok: bool, precision: u8, alloc_comps: &mut dyn FnMut(), set_quality: &mut dyn FnMut(u32, bool), add_htbl: &mut dyn FnMut(&[u8; 17], &[u8]), set_colorspace: &mut dyn FnMut()) -> JpegDefaults { // IDA 0x12ff1c: state != 100 → error 21; alloc components; quality 75 baseline; the 4 standard Huffman tables; default tables/JFIF/restart state; arith iff precision > 8.
    if !state_ok {
        panic!("jpeg_set_defaults: bad state (21)");
    }
    alloc_comps();
    set_quality(75, true);
    add_htbl(&BITS_DC_LUMINANCE, &VALS_DC_LUMINANCE);
    add_htbl(&BITS_AC_LUMINANCE, &VALS_AC_LUMINANCE);
    add_htbl(&BITS_DC_CHROMINANCE, &VALS_DC_CHROMINANCE);
    add_htbl(&BITS_AC_CHROMINANCE, &VALS_AC_CHROMINANCE);
    set_colorspace();
    JpegDefaults {
        arith_code: precision > 8,
        dc_tables: [0; 16],
        ac_tables: [1; 16],
        q_tables: [5; 16],
        jfif: JfifHeader { major: 1, minor: 1, units: 0, x_density: 1, y_density: 1 },
    }
}

// 0x130150 — _start_pass_prep
#[doc(alias = "_start_pass_prep")]
pub fn stub_130150(full_buffer: bool, input_rows: u32, v_max: u32) -> PrepState { // IDA 0x130150: full-buffer request → error 3; latch input rows, clear cursors, rows per group = 2 * v_max; returns the group size.
    if full_buffer {
        panic!("start_pass_prep: full-buffer prep unsupported (3)");
    }
    PrepState { rows_avail: input_rows, row_ctr: 0, rows_per_group: 2 * v_max }
}

// 0x1301a4 — _jinit_c_prep_controller
#[doc(alias = "_jinit_c_prep_controller")]
pub fn stub_1301a4(full_buffer: bool, need_context: bool, comp_widths: &[usize], group_rows: usize, alloc: &mut dyn FnMut(usize) -> Vec<i16>) -> PrepController { // IDA 0x1301a4: full-buffer request → error 3; context mode → pre_process_context plus color buffers, else pre_process_data plus per-component buffers (IDA pre-splits the color buffer with a Duff interleave).
    if full_buffer {
        panic!("jinit_c_prep_controller: full-buffer prep unsupported (3)");
    }
    let bufs = comp_widths.iter().map(|&w| alloc(w * group_rows)).collect();
    PrepController { method: if need_context { PrepMethod::Context } else { PrepMethod::Data }, bufs }
}

// 0x130594 — _expand_bottom_edge
// type: int __fastcall(int, size_t __n, int)
#[doc(alias = "_expand_bottom_edge")]
pub fn stub_130594(rows: &mut Vec<Vec<u8>>, first: usize) { // IDA 0x130594: replicate rows[first - 1] over rows[first..]; (count&3) Duff head + 4-wide body.
    if first >= rows.len() {
        return;
    }
    let fill = rows[first - 1].clone();
    for r in rows.iter_mut().skip(first) {
        r.clone_from(&fill);
    }
}

// 0x1306ec — _pre_process_context
#[doc(alias = "_pre_process_context")]
pub fn stub_1306ec(st: &mut PrepContext, v_max: u32, num_comps: u32, in_row: &mut u32, in_max: u32, out_row: &mut u32, out_max: u32, color_convert: &mut dyn FnMut(u32), expand_top: &mut dyn FnMut(), expand_bottom: &mut dyn FnMut(), downsample: &mut dyn FnMut()) -> u32 { // IDA 0x1306ec: context-row pump; top-edge expand on the first fill, bottom-edge expand at the end, downsample on full groups with 3*v_max window wrap; returns output rows done.
    let window = 3 * v_max;
    while *out_row < out_max {
        if *in_row < in_max {
            let room = st.rows_per_group - st.row_ctr;
            let n = (*in_row + room).min(in_max) - *in_row;
            color_convert(n);
            if !st.topped {
                for _ in 0..num_comps {
                    expand_top();
                }
                st.topped = true;
            }
            *in_row += n;
            st.rows_avail -= n;
            st.row_ctr += n;
        } else if st.rows_avail != 0 {
            return *out_row;
        } else if st.row_ctr < st.rows_per_group {
            expand_bottom();
            st.row_ctr = st.rows_per_group;
        }
        if st.row_ctr == st.rows_per_group {
            downsample();
            *out_row += 1;
            st.rows_filled += v_max;
            if st.rows_filled >= window {
                st.rows_filled = 0;
            }
            if st.row_ctr >= window {
                st.row_ctr = 0;
            }
            st.rows_per_group = v_max + st.row_ctr;
        }
    }
    *out_row
}

// 0x130910 — _pre_process_data
#[doc(alias = "_pre_process_data")]
pub fn stub_130910(in_row: &mut u32, in_max: u32, out_row: &mut u32, out_max: u32, remaining: &mut u32, filled: &mut u32, group: u32, num_comps: u32, color_convert: &mut dyn FnMut(u32), expand_bottom: &mut dyn FnMut(), downsample: &mut dyn FnMut(), finish_bottom: &mut dyn FnMut(usize)) -> u32 { // IDA 0x130910: data-row pump; bottom-edge expand at the end, downsample on full groups, downstream bottom expand when input runs dry; returns output rows done.
    while *in_row < in_max && *out_row < out_max {
        let n = (*in_row + (group - *filled)).min(in_max) - *in_row;
        color_convert(n);
        *in_row += n;
        *remaining -= n;
        *filled += n;
        if *remaining == 0 {
            if *filled < group {
                expand_bottom();
                *filled = group;
            }
            if *filled == group {
                downsample();
                *filled = 0;
                *out_row += 1;
            }
        }
        if *remaining == 0 {
            if *out_row < out_max {
                for i in 0..num_comps as usize {
                    finish_bottom(i);
                }
                *out_row = out_max;
            }
            return *out_row;
        }
    }
    *out_row
}

// 0x130ae8 — _start_pass_downsample
#[doc(alias = "_start_pass_downsample")]
pub fn stub_130ae8() { // IDA 0x130ae8: empty start-pass body.
}

// 0x130aec — _expand_right_edge
#[doc(alias = "_expand_right_edge")]
pub fn stub_130aec(rows: &mut [Vec<u8>], first: usize) { // IDA 0x130aec: replicate the last valid sample across each row's padding; IDA Duff-unrolls by (count&7).
    for r in rows.iter_mut() {
        if first == 0 || first > r.len() {
            continue;
        }
        let v = r[first - 1];
        for o in r.iter_mut().skip(first) {
            *o = v;
        }
    }
}

// 0x130bec — _sep_downsample
#[doc(alias = "_sep_downsample")]
pub fn stub_130bec(num_comps: usize, downsample: &mut dyn FnMut(usize) -> bool) -> bool { // IDA 0x130bec: call each component's downsample method in order; returns the last result (no early exit).
    let mut ok = true;
    for i in 0..num_comps {
        ok = downsample(i);
    }
    ok
}

// 0x130c78 — _int_downsample
#[doc(alias = "_int_downsample")]
pub fn stub_130c78(input: &[Vec<u8>], h: usize, v: usize, out: &mut [u8], expand: &mut dyn FnMut()) { // IDA 0x130c78: expand the right edge, then box-average each h×v cell with half-up rounding; IDA Duff-unrolls the row sum by (h&7).
    expand();
    let n = (h * v) as u32;
    for (x, o) in out.iter_mut().enumerate() {
        let mut sum = 0u32;
        for r in input.iter().take(v) {
            for k in 0..h {
                sum += r[x * h + k] as u32;
            }
        }
        *o = ((sum + (n >> 1)) / n) as u8;
    }
}

// 0x130ea8 — _h2v1_downsample
#[doc(alias = "_h2v1_downsample")]
pub fn stub_130ea8(input: &[u8], out: &mut [u8], expand: &mut dyn FnMut()) { // IDA 0x130ea8: 2:1 horizontal average with alternating rounding bias starting at 0; IDA unrolls 8-wide with a (w&7) bias-carry prologue (folded here).
    expand();
    let mut bias = 0u16;
    for (i, o) in out.iter_mut().enumerate() {
        *o = ((input[2 * i] as u16 + input[2 * i + 1] as u16 + bias) >> 1) as u8;
        bias ^= 1;
    }
}

// 0x13114c — _h2v2_downsample
#[doc(alias = "_h2v2_downsample")]
pub fn stub_13114c(top: &[u8], bottom: &[u8], out: &mut [u8]) { // IDA 0x13114c: 2×2 average with alternating rounding bias (1, 2); (w&3) prologue folded to a bias start of 1.
    let mut bias = 1u16;
    for (i, o) in out.iter_mut().enumerate() {
        *o = ((top[2 * i] as u16 + top[2 * i + 1] as u16 + bottom[2 * i] as u16 + bottom[2 * i + 1] as u16 + bias) >> 2) as u8;
        bias ^= 3;
    }
}

// 0x13136c — _h2v2_smooth_downsample
#[doc(alias = "_h2v2_smooth_downsample")]
pub fn stub_13136c(smooth: u16, rm1: &[u8], r0: &[u8], r1: &[u8], rp1: &[u8], out: &mut [u8]) { // IDA 0x13136c: 2×2 smooth (box-weighted) downsample; weights 0x4000 - 80*smooth on the 4 center taps and 16*smooth on the 20 surround taps; edge taps clamped (IDA special-cases the first/last outputs and odd widths with the same weights).
    let w6 = 16 * smooth as u32;
    let w8 = 0x4000 - 80 * smooth as u32;
    let at = |row: &[u8], i: isize| row[i.clamp(0, row.len() as isize - 1) as usize] as u32;
    for (j, o) in out.iter_mut().enumerate() {
        let x = 2 * j as isize;
        let center = at(r0, x) + at(r0, x + 1) + at(r1, x) + at(r1, x + 1);
        let surround = at(rm1, x) + at(rm1, x + 1) + at(rp1, x) + at(rp1, x + 1)
            + at(rm1, x - 1) + at(r0, x - 1) + at(r1, x - 1) + at(rp1, x - 1)
            + at(rm1, x + 2) + at(r0, x + 2) + at(r1, x + 2) + at(rp1, x + 2)
            + 2 * (at(rm1, x - 2) + at(r0, x - 2) + at(r1, x - 2) + at(rp1, x - 2));
        *o = ((w8 * center + 0x8000 + w6 * surround) >> 16) as u8;
    }
}

// 0x13179c — _fullsize_smooth_downsample
#[doc(alias = "_fullsize_smooth_downsample")]
pub fn stub_13179c() -> ! { todo!("0x13179c _fullsize_smooth_downsample") }

// 0x131a2c — _jinit_downsampler
#[doc(alias = "_jinit_downsampler")]
pub fn stub_131a2c() -> ! { todo!("0x131a2c _jinit_downsampler") }

// 0x131d08 — _fullsize_downsample
// type: int __fastcall(int, int, int, int)
#[doc(alias = "_fullsize_downsample")]
pub fn stub_131d08() -> ! { todo!("0x131d08 _fullsize_downsample") }

// 0x131d68 — _jpeg_consume_input
#[doc(alias = "_jpeg_consume_input")]
pub fn stub_131d68() -> ! { todo!("0x131d68 _jpeg_consume_input") }

// 0x132034 — _jpeg_finish_decompress
#[doc(alias = "_jpeg_finish_decompress")]
pub fn stub_132034() -> ! { todo!("0x132034 _jpeg_finish_decompress") }

// 0x132124 — _jpeg_read_header
#[doc(alias = "_jpeg_read_header")]
pub fn stub_132124() -> ! { todo!("0x132124 _jpeg_read_header") }

// 0x1321b0 — _jpeg_destroy_decompress
#[doc(alias = "_jpeg_destroy_decompress")]
pub fn stub_1321b0() -> ! { todo!("0x1321b0 _jpeg_destroy_decompress") }

// 0x1321c0 — _jpeg_CreateDecompress
// type: int __fastcall(void *__b)
#[doc(alias = "_jpeg_CreateDecompress")]
pub fn stub_1321c0() -> ! { todo!("0x1321c0 _jpeg_CreateDecompress") }

// 0x1322d4 — _output_pass_setup
#[doc(alias = "_output_pass_setup")]
pub fn stub_1322d4() -> ! { todo!("0x1322d4 _output_pass_setup") }

// 0x1323d4 — _jpeg_read_scanlines
#[doc(alias = "_jpeg_read_scanlines")]
pub fn stub_1323d4() -> ! { todo!("0x1323d4 _jpeg_read_scanlines") }

// 0x1324bc — _jpeg_read_raw_data
#[doc(alias = "_jpeg_read_raw_data")]
pub fn stub_1324bc() -> ! { todo!("0x1324bc _jpeg_read_raw_data") }

// 0x1325cc — _jpeg_start_decompress
#[doc(alias = "_jpeg_start_decompress")]
pub fn stub_1325cc() -> ! { todo!("0x1325cc _jpeg_start_decompress") }

// 0x1326e0 — _get_byte
#[doc(alias = "_get_byte")]
pub fn stub_1326e0() -> ! { todo!("0x1326e0 _get_byte") }

// 0x132744 — _arith_decode
#[doc(alias = "_arith_decode")]
pub fn stub_132744() -> ! { todo!("0x132744 _arith_decode") }

// 0x1328a4 — _jinit_arith_decoder
#[doc(alias = "_jinit_arith_decoder")]
pub fn stub_1328a4() -> ! { todo!("0x1328a4 _jinit_arith_decoder") }

// 0x1329f0 — _process_restart
#[doc(alias = "_process_restart")]
pub fn stub_1329f0() -> ! { todo!("0x1329f0 _process_restart") }

// 0x132b08 — _decode_mcu
#[doc(alias = "_decode_mcu")]
pub fn stub_132b08() -> ! { todo!("0x132b08 _decode_mcu") }

// 0x13307c — _decode_mcu_AC_refine
#[doc(alias = "_decode_mcu_AC_refine")]
pub fn stub_13307c() -> ! { todo!("0x13307c _decode_mcu_AC_refine") }

// 0x1334a8 — _decode_mcu_DC_refine
#[doc(alias = "_decode_mcu_DC_refine")]
pub fn stub_1334a8() -> ! { todo!("0x1334a8 _decode_mcu_DC_refine") }

// 0x133548 — _decode_mcu_AC_first
#[doc(alias = "_decode_mcu_AC_first")]
pub fn stub_133548() -> ! { todo!("0x133548 _decode_mcu_AC_first") }

// 0x133758 — _decode_mcu_DC_first
#[doc(alias = "_decode_mcu_DC_first")]
pub fn stub_133758() -> ! { todo!("0x133758 _decode_mcu_DC_first") }

// 0x133980 — _start_pass_0
#[doc(alias = "_start_pass_0")]
pub fn stub_133980() -> ! { todo!("0x133980 _start_pass_0") }

// 0x133d60 — _start_iMCU_row_0
#[doc(alias = "_start_iMCU_row_0")]
pub fn stub_133d60() -> ! { todo!("0x133d60 _start_iMCU_row_0") }

// 0x133dac — _start_input_pass
#[doc(alias = "_start_input_pass")]
pub fn stub_133dac() -> ! { todo!("0x133dac _start_input_pass") }

// 0x133db8 — _dummy_consume_data
#[doc(alias = "_dummy_consume_data")]
pub fn stub_133db8() -> ! { todo!("0x133db8 _dummy_consume_data") }

// 0x133dc0 — _consume_data
#[doc(alias = "_consume_data")]
pub fn stub_133dc0() -> ! { todo!("0x133dc0 _consume_data") }

// 0x133fb4 — _decompress_data
#[doc(alias = "_decompress_data")]
pub fn stub_133fb4() -> ! { todo!("0x133fb4 _decompress_data") }

// 0x134188 — _start_output_pass
#[doc(alias = "_start_output_pass")]
pub fn stub_134188() -> ! { todo!("0x134188 _start_output_pass") }

// 0x134328 — _jinit_d_coef_controller
#[doc(alias = "_jinit_d_coef_controller")]
pub fn stub_134328() -> ! { todo!("0x134328 _jinit_d_coef_controller") }

// 0x1344e8 — _decompress_onepass
#[doc(alias = "_decompress_onepass")]
pub fn stub_1344e8() -> ! { todo!("0x1344e8 _decompress_onepass") }

// 0x1348ec — _decompress_smooth_data
#[doc(alias = "_decompress_smooth_data")]
pub fn stub_1348ec() -> ! { todo!("0x1348ec _decompress_smooth_data") }

// 0x135064 — _build_ycc_rgb_table
#[doc(alias = "_build_ycc_rgb_table")]
pub fn stub_135064() -> ! { todo!("0x135064 _build_ycc_rgb_table") }

// 0x135268 — _ycc_rgb_convert
#[doc(alias = "_ycc_rgb_convert")]
pub fn stub_135268() -> ! { todo!("0x135268 _ycc_rgb_convert") }

// 0x135810 — _null_convert_0
#[doc(alias = "_null_convert_0")]
pub fn stub_135810() -> ! { todo!("0x135810 _null_convert_0") }

// 0x135980 — _gray_rgb_convert
#[doc(alias = "_gray_rgb_convert")]
pub fn stub_135980() -> ! { todo!("0x135980 _gray_rgb_convert") }

// 0x135b68 — _ycck_cmyk_convert
#[doc(alias = "_ycck_cmyk_convert")]
pub fn stub_135b68() -> ! { todo!("0x135b68 _ycck_cmyk_convert") }

// 0x1362a4 — _start_pass_dcolor
#[doc(alias = "_start_pass_dcolor")]
pub fn stub_1362a4() -> ! { todo!("0x1362a4 _start_pass_dcolor") }

// 0x1362a8 — _jinit_color_deconverter
#[doc(alias = "_jinit_color_deconverter")]
pub fn stub_1362a8() -> ! { todo!("0x1362a8 _jinit_color_deconverter") }

// 0x1364b0 — _grayscale_convert_0
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "_grayscale_convert_0")]
pub fn stub_1364b0() -> ! { todo!("0x1364b0 _grayscale_convert_0") }

// 0x1364ec — _start_pass_1
#[doc(alias = "_start_pass_1")]
pub fn stub_1364ec() -> ! { todo!("0x1364ec _start_pass_1") }

// 0x136de4 — _jinit_inverse_dct
#[doc(alias = "_jinit_inverse_dct")]
pub fn stub_136de4() -> ! { todo!("0x136de4 _jinit_inverse_dct") }

// 0x136e7c — _jpeg_fill_bit_buffer
#[doc(alias = "_jpeg_fill_bit_buffer")]
pub fn stub_136e7c() -> ! { todo!("0x136e7c _jpeg_fill_bit_buffer") }

// 0x136fcc — _jpeg_huff_decode
#[doc(alias = "_jpeg_huff_decode")]
pub fn stub_136fcc() -> ! { todo!("0x136fcc _jpeg_huff_decode") }

// 0x1370e4 — _process_restart_0
#[doc(alias = "_process_restart_0")]
pub fn stub_1370e4() -> ! { todo!("0x1370e4 _process_restart_0") }

// 0x137180 — _decode_mcu_DC_first_0
#[doc(alias = "_decode_mcu_DC_first_0")]
pub fn stub_137180() -> ! { todo!("0x137180 _decode_mcu_DC_first_0") }

// 0x1373f0 — _decode_mcu_AC_first_0
#[doc(alias = "_decode_mcu_AC_first_0")]
pub fn stub_1373f0() -> ! { todo!("0x1373f0 _decode_mcu_AC_first_0") }

// 0x13766c — _decode_mcu_DC_refine_0
#[doc(alias = "_decode_mcu_DC_refine_0")]
pub fn stub_13766c() -> ! { todo!("0x13766c _decode_mcu_DC_refine_0") }

// 0x137774 — _decode_mcu_AC_refine_0
#[doc(alias = "_decode_mcu_AC_refine_0")]
pub fn stub_137774() -> ! { todo!("0x137774 _decode_mcu_AC_refine_0") }

// 0x1386c4 — _decode_mcu_0
#[doc(alias = "_decode_mcu_0")]
pub fn stub_1386c4() -> ! { todo!("0x1386c4 _decode_mcu_0") }

// 0x138bc0 — _jinit_huff_decoder
#[doc(alias = "_jinit_huff_decoder")]
pub fn stub_138bc0() -> ! { todo!("0x138bc0 _jinit_huff_decoder") }

// 0x138ccc — _jpeg_make_d_derived_tbl
#[doc(alias = "_jpeg_make_d_derived_tbl")]
pub fn stub_138ccc() -> ! { todo!("0x138ccc _jpeg_make_d_derived_tbl") }

// 0x139938 — _start_pass_huff_decoder
#[doc(alias = "_start_pass_huff_decoder")]
pub fn stub_139938() -> ! { todo!("0x139938 _start_pass_huff_decoder") }

// 0x139d84 — _finish_input_pass
#[doc(alias = "_finish_input_pass")]
pub fn stub_139d84() -> ! { todo!("0x139d84 _finish_input_pass") }

// 0x139d9c — _reset_input_controller
#[doc(alias = "_reset_input_controller")]
pub fn stub_139d9c() -> ! { todo!("0x139d9c _reset_input_controller") }

// 0x139df4 — _jinit_input_controller
#[doc(alias = "_jinit_input_controller")]
pub fn stub_139df4() -> ! { todo!("0x139df4 _jinit_input_controller") }

// 0x139e70 — _start_input_pass_0
#[doc(alias = "_start_input_pass_0")]
pub fn stub_139e70() -> ! { todo!("0x139e70 _start_input_pass_0") }

// 0x13a278 — _consume_markers
#[doc(alias = "_consume_markers")]
pub fn stub_13a278() -> ! { todo!("0x13a278 _consume_markers") }

// 0x13a5a8 — _start_pass_main_0
#[doc(alias = "_start_pass_main_0")]
pub fn stub_13a5a8() -> ! { todo!("0x13a5a8 _start_pass_main_0") }

// 0x13aae0 — _process_data_simple_main_0
#[doc(alias = "_process_data_simple_main_0")]
pub fn stub_13aae0() -> ! { todo!("0x13aae0 _process_data_simple_main_0") }

// 0x13ab84 — _process_data_context_main
#[doc(alias = "_process_data_context_main")]
pub fn stub_13ab84() -> ! { todo!("0x13ab84 _process_data_context_main") }

// 0x13b194 — _process_data_crank_post
#[doc(alias = "_process_data_crank_post")]
pub fn stub_13b194() -> ! { todo!("0x13b194 _process_data_crank_post") }

// 0x13b1c8 — _jinit_d_main_controller
#[doc(alias = "_jinit_d_main_controller")]
pub fn stub_13b1c8() -> ! { todo!("0x13b1c8 _jinit_d_main_controller") }

// 0x13b3b8 — _get_sof
#[doc(alias = "_get_sof")]
pub fn stub_13b3b8() -> ! { todo!("0x13b3b8 _get_sof") }

// 0x13b7a0 — _examine_app0
#[doc(alias = "_examine_app0")]
pub fn stub_13b7a0() -> ! { todo!("0x13b7a0 _examine_app0") }

// 0x13ba30 — _examine_app14
#[doc(alias = "_examine_app14")]
pub fn stub_13ba30() -> ! { todo!("0x13ba30 _examine_app14") }

// 0x13bb18 — _get_interesting_appn
#[doc(alias = "_get_interesting_appn")]
pub fn stub_13bb18() -> ! { todo!("0x13bb18 _get_interesting_appn") }

// 0x13be18 — _save_marker
#[doc(alias = "_save_marker")]
pub fn stub_13be18() -> ! { todo!("0x13be18 _save_marker") }

// 0x13c120 — _skip_variable
#[doc(alias = "_skip_variable")]
pub fn stub_13c120() -> ! { todo!("0x13c120 _skip_variable") }

// 0x13c1f8 — _next_marker
#[doc(alias = "_next_marker")]
pub fn stub_13c1f8() -> ! { todo!("0x13c1f8 _next_marker") }

// 0x13c320 — _read_restart_marker
#[doc(alias = "_read_restart_marker")]
pub fn stub_13c320() -> ! { todo!("0x13c320 _read_restart_marker") }
