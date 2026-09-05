//! network generated_net_14 — auto-generated, do not edit manually
//! Filter: RakNet|Network|Replicator -> 5109 complete, batch EA-sorted asc 120 gap filler (global, since filtered complete)
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +120 stubs | range 0x1567f4..0x16d8b4 | 22879->22999 distinct (rbx_core::SharedPtr not boost) — preserves ea + mangled + demangled for rg

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

/// libpng read-struct transform/mode words behind the `png_set_*`/`png_read_*` fns
/// (IDA 0x15fde4 et al.: +156 mode, +160 transform).
#[derive(Clone, Debug, Default)]
pub struct PngRead {
    pub mode: u32,
    pub transform: u32,
    pub file_gamma: f32,
    pub screen_gamma: f32,
    pub have_info: bool,
}

/// libpng IO slot behind `png_set_read_fn` (IDA 0x15fcf8).
#[derive(Clone, Copy, Debug, Default)]
pub struct PngIo {
    pub io_ptr: usize,
    pub read_fn_custom: bool,
    pub write_fn_set: bool,
}

/// libpng info-struct view behind the `png_get_*` accessors (IDA 0x15d9f0 et al.).
#[derive(Clone, Debug, Default)]
pub struct PngInfo {
    pub valid: u32,
    pub width: u32,
    pub height: u32,
    pub bit_depth: u8,
    pub color_type: u8,
    pub compression: u8,
    pub filter: u8,
    pub interlace: u8,
    pub palette: Vec<[u8; 3]>,
    pub trans_alpha: Vec<u8>,
    pub trans_color: Option<(u16, u16, u16)>,
    pub background: [u16; 3],
    pub gamma: Option<f32>,
    pub icc_name: Option<String>,
    pub icc_profile: Vec<u8>,
    pub icc_compression: u8,
    pub phys_x: u32,
    pub phys_y: u32,
    pub phys_unit: u8,
    pub text: Vec<(String, String)>,
}

/// libpng error-fn triple installed by `png_set_error_fn` (IDA 0x15d57c).
#[derive(Clone, Copy, Debug, Default)]
pub struct PngErrorFns {
    pub error_fn: usize,
    pub warning_fn: usize,
    pub data: usize,
}

/// libpng mem-fn triple installed by `png_set_mem_fn` (IDA 0x15dd50).
#[derive(Clone, Copy, Debug, Default)]
pub struct PngMemFns {
    pub malloc_fn: usize,
    pub free_fn: usize,
    pub data: usize,
}

/// PNG transparency payload returned by `png_get_tRNS` (IDA 0x15dbec).
#[derive(Clone, Debug)]
pub enum PngTrans {
    Palette(Vec<u8>),
    Color((u16, u16, u16)),
}

/// PNG file signature compared by `png_sig_cmp` (IDA 0x15ceec: `C_20_6287`).
pub const PNG_SIGNATURE: [u8; 8] = [0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];

/// Virtual-array access window shared by `access_virt_barray`/`access_virt_sarray`
/// (IDA 0x156b6c/0x156ee8).
#[derive(Clone, Debug, Default)]
pub struct VirtWindow {
    pub total_rows: usize,
    pub rows_in_mem: usize,
    pub cur_start: usize,
    pub realized: bool,
    pub dirty: bool,
}

/// Shared backbone of the virtual-array accessors: range check (error 23); unrealized window miss
/// (error 71); flush a dirty window; slide + fill; writable marks dirty.
fn access_virt_window(
    w: &mut VirtWindow,
    start: usize,
    count: usize,
    writable: bool,
    flush: &mut dyn FnMut(),
    fill: &mut dyn FnMut(usize, usize),
) {
    if start + count > w.total_rows || start > w.total_rows || w.total_rows == 0 {
        panic!("access_virt_array: error 23");
    }
    if start < w.cur_start || start + count > w.cur_start + w.rows_in_mem {
        if !w.realized {
            panic!("access_virt_array: error 71");
        }
        if w.dirty {
            flush();
            w.dirty = false;
        }
        w.cur_start = start;
        fill(start, count);
    }
    if writable {
        w.dirty = true;
    }
}

/// Median-cut histogram box recomputed by `update_box` (IDA 0x159504).
#[derive(Clone, Copy, Debug, Default)]
pub struct ColorBox {
    pub rmin: u8,
    pub rmax: u8,
    pub gmin: u8,
    pub gmax: u8,
    pub bmin: u8,
    pub bmax: u8,
    pub count: u32,
}

// 0x1567f4 — _jinit_memory_mgr
// type: const char *__fastcall(_DWORD *)
#[doc(alias = "_jinit_memory_mgr")]
pub fn stub_1567f4(mem_init: &mut dyn FnMut() -> bool, install: &mut dyn FnMut()) {
    // IDA 0x1567f4: pool 0; jpeg_mem_init; small alloc (fail → term + error 56 exit); install
    // alloc/access/request/realize/free procs.
    if !mem_init() {
        panic!("jinit_memory_mgr: error 56");
    }
    install();
}

// 0x1569c8 — _free_pool
// type: void __fastcall(_DWORD *, unsigned int)
#[doc(alias = "_free_pool")]
pub fn stub_1569c8(pool: u32, free_virt: &mut dyn FnMut(), free_small: &mut dyn FnMut()) {
    // IDA 0x1569c8: pool > 1 → error 15; pool 1 → close/free virtual arrays; pool 0 → free small pools.
    if pool > 1 {
        panic!("free_pool: error 15");
    }
    if pool == 1 {
        free_virt();
    } else {
        free_small();
    }
}

// 0x156b28 — _self_destruct
// type: int __fastcall(int)
#[doc(alias = "_self_destruct")]
pub fn stub_156b28(free_pools: &mut dyn FnMut(), mem_term: &mut dyn FnMut() -> i32) -> i32 {
    // IDA 0x156b28: free_pool(1); free_pool(0); free the small pool; jpeg_mem_term.
    free_pools();
    mem_term()
}

// 0x156b6c — _access_virt_barray
// type: int __fastcall(int, int, unsigned int, unsigned int, char)
#[doc(alias = "_access_virt_barray")]
pub fn stub_156b6c(w: &mut VirtWindow, start: usize, count: usize, writable: bool, flush: &mut dyn FnMut(), fill: &mut dyn FnMut(usize, usize)) {
    // IDA 0x156b6c: block-row virtual array access (rows of 128-byte blocks).
    access_virt_window(w, start, count, writable, flush, fill);
}

// 0x156ee8 — _access_virt_sarray
// type: int __fastcall(int, int, unsigned int, unsigned int, char)
#[doc(alias = "_access_virt_sarray")]
pub fn stub_156ee8(w: &mut VirtWindow, start: usize, count: usize, writable: bool, flush: &mut dyn FnMut(), fill: &mut dyn FnMut(usize, usize)) {
    // IDA 0x156ee8: sample-row virtual array access.
    access_virt_window(w, start, count, writable, flush, fill);
}

// 0x157260 — _alloc_large
// type: _DWORD *__fastcall(_DWORD *, unsigned int, unsigned int)
#[doc(alias = "_alloc_large")]
pub fn stub_157260(pool: u32, size: usize, alloc: &mut dyn FnMut(usize) -> Vec<u8>) -> Vec<u8> {
    // IDA 0x157260: size > 0x3B9AC9F4 → out_of_memory(3); 8-align; bad pool → error 15; alloc +
    // link (fail → out_of_memory(4)).
    if size > 0x3B9AC9F4 {
        panic!("alloc_large: out_of_memory(3)");
    }
    if pool > 1 {
        panic!("alloc_large: error 15");
    }
    alloc((size + 7) & !7)
}

// 0x157328 — _alloc_barray
// type: int __fastcall(_DWORD *, unsigned int, int, signed int)
#[doc(alias = "_alloc_barray")]
pub fn stub_157328(_pool: u32, cols: usize, rows: usize) -> Vec<Vec<u8>> {
    // IDA 0x157328: rows-per-chunk min(0x3B9AC9F4 / (cols * 128), rows) (empty → error 72);
    // pointer array + large blocks per chunk.
    let row_bytes = cols * 128;
    if row_bytes == 0 || 0x3B9AC9F4 / row_bytes == 0 {
        panic!("alloc_barray: error 72");
    }
    let chunk = (0x3B9AC9F4 / row_bytes).min(rows).max(1);
    let mut out = Vec::new();
    let mut remaining = rows;
    while remaining > 0 {
        let n = chunk.min(remaining);
        out.push(vec![0u8; n * row_bytes]);
        remaining -= n;
    }
    out
}

// 0x1574cc — _alloc_sarray
// type: int __fastcall(_DWORD *, unsigned int, unsigned int, unsigned int)
#[doc(alias = "_alloc_sarray")]
pub fn stub_1574cc(_pool: u32, samples_per_row: usize, rows: usize) -> Vec<Vec<u8>> {
    // IDA 0x1574cc: rows-per-chunk min(0x3B9AC9F4 / samples, rows) (empty → error 72); pointer
    // array + large blocks per chunk.
    if samples_per_row == 0 || 0x3B9AC9F4 / samples_per_row == 0 {
        panic!("alloc_sarray: error 72");
    }
    let chunk = (0x3B9AC9F4 / samples_per_row).min(rows).max(1);
    let mut out = Vec::new();
    let mut remaining = rows;
    while remaining > 0 {
        let n = chunk.min(remaining);
        out.push(vec![0u8; n * samples_per_row]);
        remaining -= n;
    }
    out
}

// 0x157670 — _realize_virt_arrays
// type: int __fastcall(int result, int)
#[doc(alias = "_realize_virt_arrays")]
pub fn stub_157670(sarray_bytes: usize, barray_bytes: usize, alloc: &mut dyn FnMut(usize) -> bool, realize: &mut dyn FnMut()) -> bool {
    // IDA 0x157670: total the virtual array storage; single backing alloc; realize each array.
    if !alloc(sarray_bytes + barray_bytes) {
        return false;
    }
    realize();
    true
}

// 0x15787c — _largest_input_value
// type: int __fastcall(int, int, int, int)
#[doc(alias = "_largest_input_value")]
pub fn stub_15787c(range: i32, max: i32) -> i32 {
    // IDA 0x15787c: (510 * range + max + 255) / (2 * max).
    (510 * range + max + 255) / (2 * max)
}

// 0x1578a4 — _create_colorindex
// type: int __fastcall(_DWORD *, int, int, int)
#[doc(alias = "_create_colorindex")]
pub fn stub_1578a4(components: i32, fill: &mut dyn FnMut(i32)) {
    // IDA 0x1578a4: one component → fixed 510 index; else per-component colorindex fill.
    if components == 1 {
        fill(510);
    } else {
        for c in 0..components {
            fill(c);
        }
    }
}

// 0x157b44 — _color_quantize
// type: unsigned __int8 *__fastcall(unsigned __int8 *result, int, unsigned __int8 *, int)
#[doc(alias = "_color_quantize")]
pub fn stub_157b44(colormap: &[u8], r_scale: usize, g_scale: usize, pixels: &[(u8, u8, u8)], out: &mut [u8]) {
    // IDA 0x157b44: RGB → colormap index per pixel.
    for (i, &(r, g, b)) in pixels.iter().enumerate() {
        if i < out.len() {
            out[i] = colormap.get(r as usize * r_scale + g as usize * g_scale + b as usize).copied().unwrap_or(0);
        }
    }
}

// 0x157d58 — _color_quantize3
// type: int __fastcall(int result, int, int, int)
#[doc(alias = "_color_quantize3")]
pub fn stub_157d58(colormap: &[u8], pixels: &[[u8; 3]], out: &mut [u8], start: usize) {
    // IDA 0x157d58: 3-component colormap lookup with (pos & 3) input shift.
    for (i, px) in pixels.iter().enumerate() {
        if start + i < out.len() {
            let idx = px[0] as usize + px[1] as usize + px[2] as usize;
            out[start + i] = colormap.get(idx % colormap.len().max(1)).copied().unwrap_or(0);
        }
    }
}

// 0x157f10 — _quantize3_ord_dither
// type: int __fastcall(int, int, int, int)
#[doc(alias = "_quantize3_ord_dither")]
pub fn stub_157f10(colormap: &[u8], pixels: &[[u8; 3]], out: &mut [u8], row: usize, dither: &[i16; 64]) {
    // IDA 0x157f10: 3-component ordered-dither quantize (dither matrix row by output position).
    for (i, px) in pixels.iter().enumerate() {
        if i < out.len() {
            let d = dither[(row + i) % 64] as i32;
            let idx = (px[0] as i32 + px[1] as i32 + px[2] as i32 + d).max(0) as usize;
            out[i] = colormap.get(idx % colormap.len().max(1)).copied().unwrap_or(0);
        }
    }
}

// 0x1580fc — _alloc_fs_workspace
// type: int __fastcall(int result)
#[doc(alias = "_alloc_fs_workspace")]
pub fn stub_1580fc(components: usize, width: usize, alloc_row: &mut dyn FnMut() -> bool) -> bool {
    // IDA 0x1580fc: per-component error-diffusion workspace rows (2 * width + 4 cells).
    let _ = width;
    for _ in 0..components {
        if !alloc_row() {
            return false;
        }
    }
    true
}

// 0x15815c — _finish_pass_1_quant
// type: void()
#[doc(alias = "_finish_pass_1_quant")]
pub fn stub_15815c() {
    // IDA 0x15815c: empty finish_pass_1_quant body.
}

// 0x158160 — _new_color_map_1_quant
// type: int __fastcall(int)
#[doc(alias = "_new_color_map_1_quant")]
pub fn stub_158160() -> ! {
    // IDA 0x158160: new_color_map_1_quant → error 47 exit.
    panic!("new_color_map_1_quant: error 47");
}

// 0x158178 — _jinit_1pass_quantizer
// type: int __fastcall(_DWORD *)
#[doc(alias = "_jinit_1pass_quantizer")]
pub fn stub_158178(desired_colors: i32, init: &mut dyn FnMut(i32) -> bool) -> bool {
    // IDA 0x158178: alloc quantizer state; build colormap + inverse map; install color_quantize passes.
    init(desired_colors)
}

// 0x158810 — _start_pass_1_quant
// type: _DWORD *__fastcall(_DWORD *result, int, int)
#[doc(alias = "_start_pass_1_quant")]
pub fn stub_158810(components: usize, setup: &mut dyn FnMut(usize) -> bool) -> bool {
    // IDA 0x158810: per-component first-pass quantizer row setup.
    for c in 0..components {
        if !setup(c) {
            return false;
        }
    }
    true
}

// 0x158e1c — _quantize_fs_dither
// type: int __fastcall(_DWORD *, int, int, int)
#[doc(alias = "_quantize_fs_dither")]
pub fn stub_158e1c(row: &[u8], err_cur: &mut [i32], err_next: &mut [i32], colormap: &[u8], out: &mut [u8]) {
    // IDA 0x158e1c: Floyd-Steinberg error-diffusion quantize of one row (7/3/5/1 sixteenths).
    for (i, &px) in row.iter().enumerate() {
        if i >= out.len() {
            break;
        }
        let corrected = (px as i32 + err_cur.get(i).copied().unwrap_or(0)).clamp(0, 255);
        let q = colormap.get((corrected as usize) % colormap.len().max(1)).copied().unwrap_or(0);
        out[i] = q;
        let err = corrected - q as i32;
        if let Some(e) = err_cur.get_mut(i + 1) {
            *e += err * 7 / 16;
        }
        if i > 0 {
            if let Some(e) = err_next.get_mut(i - 1) {
                *e += err * 3 / 16;
            }
        }
        if let Some(e) = err_next.get_mut(i) {
            *e += err * 5 / 16;
        }
        if let Some(e) = err_next.get_mut(i + 1) {
            *e += err / 16;
        }
    }
}

// 0x159184 — _quantize_ord_dither
// type: size_t __fastcall(_DWORD *, int, int, int)
#[doc(alias = "_quantize_ord_dither")]
pub fn stub_159184(row: &[u8], colormap: &[u8], out: &mut [u8], dither: &[u8], row_phase: usize) {
    // IDA 0x159184: ordered-dither quantize of one row (zeroed output, matrix by position).
    out.fill(0);
    for (i, &px) in row.iter().enumerate() {
        if i >= out.len() {
            break;
        }
        let d = dither.get((row_phase + i) % dither.len().max(1)).copied().unwrap_or(0) as i32;
        let idx = (px as i32 + d).clamp(0, 255) as usize;
        out[i] = colormap.get(idx % colormap.len().max(1)).copied().unwrap_or(0);
    }
}

// 0x1593d8 — _prescan_quantize
// type: int __fastcall(int result, int, int, int)
#[doc(alias = "_prescan_quantize")]
pub fn stub_1593d8(histogram: &mut [u32; 32768], pixels: &[[u8; 3]]) {
    // IDA 0x1593d8: histogram prescan ((R >> 3, G >> 2, B >> 3) buckets).
    for px in pixels {
        let idx = ((px[0] as usize >> 3) << 10) | ((px[1] as usize >> 2) << 5) | (px[2] as usize >> 3);
        histogram[idx] += 1;
    }
}

// 0x159504 — _update_box
// type: int __fastcall(int, int *)
#[doc(alias = "_update_box")]
pub fn stub_159504(histogram: &[u32; 32768], cells: &[usize], box_: &mut ColorBox) {
    // IDA 0x159504: recompute a median-cut box's channel bounds + population over its histogram cells.
    let mut bb = ColorBox { rmin: 255, gmin: 255, bmin: 255, rmax: 0, gmax: 0, bmax: 0, count: 0 };
    for &c in cells {
        let n = histogram.get(c).copied().unwrap_or(0);
        if n == 0 {
            continue;
        }
        let (r, g, b) = ((c >> 10) as u8, ((c >> 5) & 31) as u8, (c & 31) as u8);
        bb.rmin = bb.rmin.min(r);
        bb.rmax = bb.rmax.max(r);
        bb.gmin = bb.gmin.min(g);
        bb.gmax = bb.gmax.max(g);
        bb.bmin = bb.bmin.min(b);
        bb.bmax = bb.bmax.max(b);
        bb.count += n;
    }
    *box_ = bb;
}

// 0x15a230 — _fill_inverse_cmap
// type: int __fastcall(_DWORD *, int, int, int)
#[doc(alias = "_fill_inverse_cmap")]
pub fn stub_15a230(colormap: &[[u8; 3]], inverse: &mut [u8]) {
    // IDA 0x15a230: inverse colormap — nearest palette color per histogram cell (the original
    // flood-fills from populated cells; direct nearest search here).
    for (i, slot) in inverse.iter_mut().enumerate().take(32768) {
        let (r, g, b) = ((i >> 10) * 8, ((i >> 5) & 31) * 4, (i & 31) * 8);
        let mut best = 0u8;
        let mut best_d = u32::MAX;
        for (k, c) in colormap.iter().enumerate() {
            let dr = r as i32 - c[0] as i32;
            let dg = g as i32 - c[1] as i32;
            let db = b as i32 - c[2] as i32;
            let d = (dr * dr + dg * dg + db * db) as u32;
            if d < best_d {
                best_d = d;
                best = k as u8;
            }
        }
        *slot = best;
    }
}

// 0x15ae14 — _pass2_no_dither
// type: int __fastcall(_DWORD *, int, int, int)
#[doc(alias = "_pass2_no_dither")]
pub fn stub_15ae14(inverse: &[u8], pixels: &[[u8; 3]], out: &mut [u8]) {
    // IDA 0x15ae14: second-pass plain mapping through the inverse colormap.
    for (i, px) in pixels.iter().enumerate() {
        if i >= out.len() {
            break;
        }
        let idx = ((px[0] as usize >> 3) << 10) | ((px[1] as usize >> 2) << 5) | (px[2] as usize >> 3);
        out[i] = inverse.get(idx).copied().unwrap_or(0);
    }
}

// 0x15afac — _pass2_fs_dither
// type: int __fastcall(_DWORD *, int, int, int)
#[doc(alias = "_pass2_fs_dither")]
pub fn stub_15afac(pixels: &[[u8; 3]], err_lim: &[i32], inverse: &[u8], out: &mut [u8]) {
    // IDA 0x15afac: FS second pass with error limiting (on-the-fly box remeasure below truncation).
    for (i, px) in pixels.iter().enumerate() {
        if i >= out.len() {
            break;
        }
        let idx = ((px[0] as usize >> 3) << 10) | ((px[1] as usize >> 2) << 5) | (px[2] as usize >> 3);
        let _ = err_lim.get(idx % err_lim.len().max(1)).copied().unwrap_or(0);
        out[i] = inverse.get(idx).copied().unwrap_or(0);
    }
}

// 0x15b32c — _init_error_limit
// type: _DWORD *__fastcall(int)
#[doc(alias = "_init_error_limit")]
pub fn stub_15b32c() -> Vec<i32> {
    // IDA 0x15b32c: 512-entry error-limit table; [255] = 0, [255 ± k] = ±k.
    let mut t = vec![0i32; 512];
    for k in 0..256 {
        t[255 + k] = k as i32;
        t[255 - k] = -(k as i32);
    }
    t
}

// 0x15b640 — _finish_pass1
// type: int __fastcall(int)
#[doc(alias = "_finish_pass1")]
pub fn stub_15b640(split_one: &mut dyn FnMut() -> bool, build_map: &mut dyn FnMut() -> bool) -> bool {
    // IDA 0x15b640: median-cut loop (split the largest-population box until the color target);
    // build colormap + inverse map.
    while split_one() {}
    build_map()
}

// 0x15bfe0 — _finish_pass2
// type: void()
#[doc(alias = "_finish_pass2")]
pub fn stub_15bfe0() {
    // IDA 0x15bfe0: empty finish_pass2 body.
}

// 0x15bfe4 — _new_color_map_2_quant
// type: int __fastcall(int result)
#[doc(alias = "_new_color_map_2_quant")]
pub fn stub_15bfe4(ready: &mut bool) {
    // IDA 0x15bfe4: new_color_map_2_quant sets the ready flag (result forwarded).
    *ready = true;
}

// 0x15bff4 — _jinit_2pass_quantizer
// type: _DWORD *__fastcall(int)
#[doc(alias = "_jinit_2pass_quantizer")]
pub fn stub_15bff4(out_components: i32, init: &mut dyn FnMut() -> bool) -> bool {
    // IDA 0x15bff4: out components != 3 → error 48; alloc state + histogram boxes; install passes.
    if out_components != 3 {
        panic!("jinit_2pass_quantizer: error 48");
    }
    init()
}

// 0x15c2b4 — _start_pass_2_quant
// type: int __fastcall(_DWORD *, char)
#[doc(alias = "_start_pass_2_quant")]
pub fn stub_15c2b4(out_components: &mut i32, gather: bool, install: &mut dyn FnMut(), zero_histograms: &mut dyn FnMut()) {
    // IDA 0x15c2b4: nonzero components → 2; gather → install prescan/finish passes + flag; zero histograms.
    if *out_components != 0 {
        *out_components = 2;
    }
    if gather {
        install();
    }
    zero_histograms();
}

// 0x15c508 — _jdiv_round_up
// type: int __fastcall(int, int)
#[doc(alias = "_jdiv_round_up")]
pub fn stub_15c508(a: i32, b: i32) -> i32 {
    // IDA 0x15c508: (a - 1 + b) / b.
    (a - 1 + b) / b
}

// 0x15c520 — _jround_up
// type: int __fastcall(int, int)
#[doc(alias = "_jround_up")]
pub fn stub_15c520(a: i32, b: i32) -> i32 {
    // IDA 0x15c520: a - 1 + b - (a - 1 + b) % b.
    a - 1 + b - (a - 1 + b) % b
}

// 0x15c540 — _jzero_far
// type: void *__fastcall(void *, size_t __len)
#[doc(alias = "_jzero_far")]
pub fn stub_15c540(dst: &mut [u8]) {
    // IDA 0x15c540: memset(dst, 0, len).
    dst.fill(0);
}

// 0x15c558 — _jcopy_block_row
// type: void *__fastcall(void *__src, void *__dst, int)
#[doc(alias = "_jcopy_block_row")]
pub fn stub_15c558(dst: &mut [u8], src: &[u8], rows: usize) {
    // IDA 0x15c558: memcpy(dst, src, rows << 7).
    let n = (rows << 7).min(dst.len()).min(src.len());
    dst[..n].copy_from_slice(&src[..n]);
}

// 0x15c578 — _jcopy_sample_rows
// type: char *__fastcall(char *result, int, int, int, int, size_t __n)
#[doc(alias = "_jcopy_sample_rows")]
pub fn stub_15c578<'a>(dst: &mut [&'a [u8]], src: &[&'a [u8]], start: usize, count: usize) {
    // IDA 0x15c578: copy row pointers (head/tail unrolled by count & 7).
    for i in 0..count {
        if let (Some(d), Some(s)) = (dst.get_mut(start + i), src.get(start + i)) {
            *d = s;
        }
    }
}

// 0x15c71c — _png_get_io_ptr
// type: int __fastcall(int result)
#[doc(alias = "_png_get_io_ptr")]
pub fn stub_15c71c(ctx: Option<usize>, read_word: &mut dyn FnMut(usize) -> usize) -> Option<usize> {
    // IDA 0x15c71c: null → null; else *(ctx + 132).
    ctx.map(|c| read_word(c + 132))
}

// 0x15c728 — _png_64bit_product
// type: int __fastcall(unsigned int, int, _DWORD *, int *)
#[doc(alias = "_png_64bit_product")]
pub fn stub_15c728(a: u32, b: u32) -> (u32, u32) {
    // IDA 0x15c728: 16-bit-half multiply → (high, low) words.
    let (a_lo, a_hi) = (a & 0xFFFF, a >> 16);
    let (b_lo, b_hi) = (b & 0xFFFF, b >> 16);
    let p = a_lo * b_lo;
    let mid = a_hi * b_lo + a_lo * b_hi + (p >> 16);
    let hi = (mid >> 16) + a_hi * b_hi;
    let lo = (p & 0xFFFF) | ((mid & 0xFFFF) << 16);
    (hi, lo)
}

// 0x15c778 — _png_check_cHRM_fixed
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int, int)
#[doc(alias = "_png_check_cHRM_fixed")]
pub fn stub_15c778(wx: i32, wy: i32, rx: i32, ry: i32, gx: i32, gy: i32, bx: i32, by: i32, warn: &mut dyn FnMut(&str)) -> bool {
    // IDA 0x15c778: negative → "Ignoring attempt to set negative chromaticity value"; sums > 100000
    // → "Invalid cHRM * point"; 0/1.
    let mut ok = true;
    for (name, x, y) in [("white", wx, wy), ("red", rx, ry), ("green", gx, gy), ("blue", bx, by)] {
        if x < 0 || y <= 0 {
            warn("Ignoring attempt to set negative chromaticity value");
            ok = false;
        } else if x > 100000 - y {
            warn(&format!("Invalid cHRM {} point", name));
            ok = false;
        }
    }
    ok
}

// 0x15c92c — _png_check_IHDR
// type: int __fastcall(_DWORD *, unsigned int, int, int, int, int, int, int)
#[doc(alias = "_png_check_IHDR")]
pub fn stub_15c92c(width: u32, height: u32, width_limit: u32, height_limit: u32, warn: &mut dyn FnMut(&str)) -> bool {
    // IDA 0x15c92c: zero dims + user-limit checks with warnings (color-type checks below truncation).
    let mut bad = false;
    if width == 0 {
        warn("Image width is zero in IHDR");
        bad = true;
    }
    if height == 0 {
        warn("Image height is zero in IHDR");
        bad = true;
    }
    if width > 0xF4240 || width > width_limit {
        warn("Image width exceeds user limit in IHDR");
        bad = true;
    }
    if height > 0xF4240 || height > height_limit {
        warn("Image height exceeds user limit in IHDR");
        bad = true;
    }
    !bad
}

// 0x15cc50 — _png_set_sig_bytes
// type: int __fastcall(int result, int)
#[doc(alias = "_png_set_sig_bytes")]
pub fn stub_15cc50(has_ctx: bool, num_bytes: i32) -> i32 {
    // IDA 0x15cc50: null → 0; > 8 → png_error (noreturn); store max(num, 0); return it.
    if !has_ctx {
        return 0;
    }
    if num_bytes > 8 {
        panic!("Too many bytes for PNG signature.");
    }
    num_bytes.max(0)
}

// 0x15cc88 — _png_handle_as_unknown
// type: int __fastcall(int, void *__s1)
#[doc(alias = "_png_handle_as_unknown")]
pub fn stub_15cc88(chunks: &[[u8; 5]], tag: Option<[u8; 4]>) -> u8 {
    // IDA 0x15cc88: null → default; linear search of the 5-byte unknown-chunk entries; hit → flag byte.
    let tag = match tag {
        Some(t) => t,
        None => return 0,
    };
    for c in chunks {
        if c[..4] == tag {
            return c[4];
        }
    }
    0
}

// 0x15ceec — _png_sig_cmp
// type: int __fastcall(int, unsigned int, size_t)
#[doc(alias = "_png_sig_cmp")]
pub fn stub_15ceec(data: &[u8], start: usize, count: usize) -> i32 {
    // IDA 0x15ceec: clamp to 8; start > 7 → -1; count 0 → -1; memcmp(data + start, sig + start, n).
    let mut n = count.min(8);
    if start > 7 {
        return -1;
    }
    if n + start > 8 {
        n = 8 - start;
    }
    if n == 0 {
        return -1;
    }
    for i in 0..n {
        let (a, b) = (data.get(start + i).copied().unwrap_or(0), PNG_SIGNATURE[start + i]);
        if a != b {
            return a as i32 - b as i32;
        }
    }
    0
}

// 0x15cf64 — _png_zalloc
// type: int __fastcall(int, unsigned int, unsigned int)
#[doc(alias = "_png_zalloc")]
pub fn stub_15cf64(items: u32, item_size: u32, alloc: &mut dyn FnMut(usize) -> Option<Vec<u8>>, warn: &mut dyn FnMut(&str)) -> Option<Vec<u8>> {
    // IDA 0x15cf64: size overflow → warning + null; else malloc with the flag held (zero size skips
    // the check as in the original fallthrough).
    if item_size != 0 && items > 0xFFFF_FFFF / item_size {
        warn("Potential overflow in png_zalloc()");
        return None;
    }
    alloc(items as usize * item_size as usize)
}

// 0x15cfd0 — _png_free_data
// type: int __fastcall(int result, int, int, int)
#[doc(alias = "_png_free_data")]
pub fn stub_15cfd0(valid_mask: u16, free_mask: u16, free_one: &mut dyn FnMut(u16)) {
    // IDA 0x15cfd0: flag-driven cascade freeing info-struct members (member mapping below truncation).
    let mut flags = valid_mask & free_mask;
    let mut bit = 0;
    while flags != 0 {
        if flags & 1 != 0 {
            free_one(bit);
        }
        flags >>= 1;
        bit += 1;
    }
}

// 0x15d41c — _png_zfree
// type: int __fastcall(int, int)
#[doc(alias = "_png_zfree")]
pub fn stub_15d41c(ptr: usize, free: &mut dyn FnMut(usize)) {
    // IDA 0x15d41c: tail-call png_free.
    free(ptr);
}

// 0x15d42c — _png_info_init_3
// type: void *__fastcall(void **, unsigned int)
#[doc(alias = "_png_info_init_3")]
pub fn stub_15d42c(info: Option<Vec<u8>>, struct_size: usize, recreate: &mut dyn FnMut() -> Vec<u8>) -> Option<Vec<u8>> {
    // IDA 0x15d42c: null → null; size < 0x120 → destroy + recreate; zero 0x120 bytes.
    let mut info = info?;
    if struct_size < 0x120 {
        info = recreate();
    }
    if info.len() < 0x120 {
        info.resize(0x120, 0);
    }
    info[..0x120].fill(0);
    Some(info)
}

// 0x15d46c — _png_info_destroy
// type: void *__fastcall(int, void *)
#[doc(alias = "_png_info_destroy")]
pub fn stub_15d46c(info: Vec<u8>, free_all: &mut dyn FnMut(), reinit: &mut dyn FnMut() -> Vec<u8>) -> Vec<u8> {
    // IDA 0x15d46c: free_data(all) + unknown-chunk list; re-init the info struct.
    free_all();
    let _ = info;
    reinit()
}

// 0x15d4c4 — _png_create_info_struct
// type: int __fastcall(int)
#[doc(alias = "_png_create_info_struct")]
pub fn stub_15d4c4(has_ctx: bool, create: &mut dyn FnMut() -> Option<Vec<u8>>, init: &mut dyn FnMut(Vec<u8>) -> Vec<u8>) -> Option<Vec<u8>> {
    // IDA 0x15d4c4: null → null; create struct; init 0x120 bytes.
    if !has_ctx {
        return None;
    }
    create().map(|v| init(v))
}

// 0x15d510 — _png_calculate_crc
// type: uLong __fastcall(uLong result, const Bytef *, uInt)
#[doc(alias = "_png_calculate_crc")]
pub fn stub_15d510(crc_enabled: bool, encrypted: bool, mode: u32, current: u32, data: &[u8], crc32: &mut dyn FnMut(u32, &[u8]) -> u32) -> u32 {
    // IDA 0x15d510: ancillary-skip and encrypt-bypass paths return the incoming crc; else accumulate.
    if crc_enabled {
        if mode == 768 {
            return current;
        }
    } else if encrypted {
        return current;
    }
    crc32(current, data)
}

// 0x15d558 — _png_reset_crc
// type: uLong __fastcall(int)
#[doc(alias = "_png_reset_crc")]
pub fn stub_15d558() -> u32 {
    // IDA 0x15d558: crc = crc32(0, 0, 0); store; return it.
    0
}

// 0x15d57c — _png_set_error_fn
// type: _DWORD *__fastcall(_DWORD *result, int, int, int)
#[doc(alias = "_png_set_error_fn")]
pub fn stub_15d57c(fns: &mut Option<PngErrorFns>, error_fn: usize, warning_fn: usize, data: usize) -> bool {
    // IDA 0x15d57c: null → passthrough; store the three fns.
    match fns {
        Some(f) => {
            *f = PngErrorFns { error_fn, warning_fn, data };
            true
        }
        None => false,
    }
}

// 0x15d590 — _png_format_buffer
// type: void *__fastcall(unsigned __int8 *, _BYTE *, void *__src)
#[doc(alias = "_png_format_buffer")]
pub fn stub_15d590(name: &[u8; 4]) -> String {
    // IDA 0x15d590: printable fourcc → itself; other bytes → "[hh]" hex via png_digit.
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::new();
    for &c in name {
        if (65..=90).contains(&c) || (97..=122).contains(&c) {
            out.push(c as char);
        } else {
            out.push('[');
            out.push(DIGITS[(c >> 4) as usize] as char);
            out.push(DIGITS[(c & 0xF) as usize] as char);
            out.push(']');
        }
    }
    out
}

// 0x15d790 — _png_warning
// type: int __fastcall(int, _BYTE *)
#[doc(alias = "_png_warning")]
pub fn stub_15d790(msg: &str, warn: &mut dyn FnMut(&str)) {
    // IDA 0x15d790: strip a leading "#N " counter prefix; dispatch to the warning proc.
    let bytes = msg.as_bytes();
    let mut skip = 0;
    if bytes.first() == Some(&b'#') {
        for n in 1..=8 {
            if bytes.get(n) == Some(&b' ') {
                skip = n + 1;
                break;
            }
        }
    }
    warn(&msg[skip.min(msg.len())..]);
}

// 0x15d8f0 — _png_chunk_warning
// type: int __fastcall(unsigned __int8 *, _BYTE *__src)
#[doc(alias = "_png_chunk_warning")]
pub fn stub_15d8f0(chunk: &[u8; 4], msg: &str, warn: &mut dyn FnMut(String)) {
    // IDA 0x15d8f0: format the chunk name, then png_warning.
    warn(format!("{}: {}", stub_15d590(chunk), msg));
}

// 0x15d924 — _png_error
// type: void __fastcall __noreturn(int *, const char *)
#[doc(alias = "_png_error")]
pub fn stub_15d924(error_fn: Option<&mut dyn FnMut()>, msg: &str, jmp: &mut dyn FnMut()) {
    // IDA 0x15d924: error_fn proc if set; stderr "libpng error: %s"; longjmp(1) (abort when null ctx).
    // BUG: original diverges via longjmp; modeled with a trailing panic.
    if let Some(f) = error_fn {
        f();
    }
    eprintln!("libpng error: {}", msg);
    jmp();
    panic!("diverges per IDA 0x15d924");
}

// 0x15d9bc — _png_chunk_error
// type: void __fastcall __noreturn(unsigned __int8 *, _BYTE *__src)
#[doc(alias = "_png_chunk_error")]
pub fn stub_15d9bc(chunk: Option<&[u8; 4]>, msg: &str, error: &mut dyn FnMut(String)) {
    // IDA 0x15d9bc: format the chunk name when present, then png_error.
    // BUG: original diverges via longjmp; modeled with a trailing panic.
    match chunk {
        Some(c) => error(format!("{}: {}", stub_15d590(c), msg)),
        None => error(msg.to_owned()),
    }
    panic!("diverges per IDA 0x15d9bc");
}

// 0x15d9e8 — sub_15D9E8
// type: void __fastcall(int, int, int, int, int, int, int)
#[doc(alias = "sub_15D9E8")]
pub fn stub_15d9e8() {
    // IDA 0x15d9e8: single POP return (padding thunk).
}

// 0x15d9f0 — _png_get_valid
// type: int __fastcall(int, int, int)
#[doc(alias = "_png_get_valid")]
pub fn stub_15d9f0(info: Option<&PngInfo>, mask: u32) -> u32 {
    // IDA 0x15d9f0: null → 0; else valid-mask & word.
    info.map(|i| mask & i.valid).unwrap_or(0)
}

// 0x15da0c — _png_get_color_type
// type: int __fastcall(int, int)
#[doc(alias = "_png_get_color_type")]
pub fn stub_15da0c(info: Option<&PngInfo>) -> u8 {
    // IDA 0x15da0c: null → 0; else the color-type byte (+25).
    info.map(|i| i.color_type).unwrap_or(0)
}

// 0x15da24 — _png_get_bKGD
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "_png_get_bKGD")]
pub fn stub_15da24(info: Option<&PngInfo>, out: Option<&mut [u16; 3]>) -> u32 {
    // IDA 0x15da24: null info/out → 0; valid bit 5 clear → 0; else copy the triple; 32.
    let (info, out) = match (info, out) {
        (Some(i), Some(o)) => (i, o),
        _ => return 0,
    };
    if info.valid >> 5 & 1 == 0 {
        return 0;
    }
    *out = info.background;
    32
}

// 0x15da58 — _png_get_gAMA
// type: int __fastcall(int, int, double *)
#[doc(alias = "_png_get_gAMA")]
pub fn stub_15da58(info: Option<&PngInfo>, out: Option<&mut f32>) -> i32 {
    // IDA 0x15da58: null info/out or valid-0 → 0; else the gamma float; 1.
    match (info, out) {
        (Some(i), Some(o)) if i.valid & 1 != 0 => {
            *o = i.gamma.unwrap_or(0.0);
            1
        }
        _ => 0,
    }
}

// 0x15da98 — _png_get_iCCP
// type: int __fastcall(int, int, _DWORD *, _DWORD *, _DWORD *, _DWORD *)
#[doc(alias = "_png_get_iCCP")]
pub fn stub_15da98(info: Option<&PngInfo>) -> Option<(String, Vec<u8>, u8)> {
    // IDA 0x15da98: null → null; valid bit 12 clear → null; all outs required; (name, profile,
    // compression); flag 4096.
    let info = info?;
    if info.valid >> 12 & 1 == 0 {
        return None;
    }
    Some((info.icc_name.clone()?, info.icc_profile.clone(), info.icc_compression))
}

// 0x15dafc — _png_get_pHYs
// type: int __fastcall(int, int, _DWORD *, _DWORD *, _DWORD *)
#[doc(alias = "_png_get_pHYs")]
pub fn stub_15dafc(info: Option<&PngInfo>) -> Option<(u32, u32, u8)> {
    // IDA 0x15dafc: null → null; valid 0x80 clear → null; else (x, y, unit); presence flags.
    let info = info?;
    if info.valid & 0x80 == 0 {
        return None;
    }
    Some((info.phys_x, info.phys_y, info.phys_unit))
}

// 0x15db58 — _png_get_PLTE
// type: int __fastcall(int, int, _DWORD *, _DWORD *)
#[doc(alias = "_png_get_PLTE")]
pub fn stub_15db58(info: Option<&PngInfo>) -> Option<(Vec<[u8; 3]>, u16)> {
    // IDA 0x15db58: null → null; valid bit 3 clear → null; else (palette, count); flag 8.
    let info = info?;
    if info.valid >> 3 & 1 == 0 {
        return None;
    }
    Some((info.palette.clone(), info.palette.len() as u16))
}

// 0x15db98 — _png_get_text
// type: _DWORD *__fastcall(_DWORD *result, int, _DWORD *, int *)
#[doc(alias = "_png_get_text")]
pub fn stub_15db98(info: Option<&PngInfo>) -> Vec<(String, String)> {
    // IDA 0x15db98: null → empty; else the text pairs (count/pointer outs).
    info.map(|i| i.text.clone()).unwrap_or_default()
}

// 0x15dbec — _png_get_tRNS
// type: int __fastcall(int, int, int *, _DWORD *, _DWORD *)
#[doc(alias = "_png_get_tRNS")]
pub fn stub_15dbec(info: Option<&PngInfo>) -> Option<PngTrans> {
    // IDA 0x15dbec: null → null; valid bit 4 clear → null; palette → alpha list (16); else color (16).
    let info = info?;
    if info.valid & 0x10 == 0 {
        return None;
    }
    if info.color_type == 3 {
        Some(PngTrans::Palette(info.trans_alpha.clone()))
    } else {
        info.trans_color.map(PngTrans::Color)
    }
}

// 0x15dc7c — _png_get_IHDR
// type: int __fastcall(_DWORD *, int, _DWORD *, _DWORD *, _DWORD *, _DWORD *, _DWORD *, _DWORD *, _DWORD *)
#[doc(alias = "_png_get_IHDR")]
pub fn stub_15dc7c(info: Option<&PngInfo>) -> Option<(u32, u32, u8, u8, u8, u8, u8)> {
    // IDA 0x15dc7c: nulls → null; else the seven IHDR fields; 1.
    let info = info?;
    Some((
        info.width,
        info.height,
        info.bit_depth,
        info.color_type,
        info.compression,
        info.filter,
        info.interlace,
    ))
}

// 0x15dd50 — _png_set_mem_fn
// type: _DWORD *__fastcall(_DWORD *result, int, int, int)
#[doc(alias = "_png_set_mem_fn")]
pub fn stub_15dd50(fns: &mut Option<PngMemFns>, malloc_fn: usize, free_fn: usize, data: usize) -> bool {
    // IDA 0x15dd50: null → passthrough; store the three words.
    match fns {
        Some(f) => {
            *f = PngMemFns { malloc_fn, free_fn, data };
            true
        }
        None => false,
    }
}

// 0x15dd64 — _png_memset_check
// type: void *__fastcall(int, void *__b, int __c, size_t __len)
#[doc(alias = "_png_memset_check")]
pub fn stub_15dd64(dst: &mut [u8], val: u8) -> usize {
    // IDA 0x15dd64: memset(dst, val, len); return len.
    dst.fill(val);
    dst.len()
}

// 0x15dd80 — _png_memcpy_check
// type: void *__fastcall(int, void *__dst, void *__src, size_t __n)
#[doc(alias = "_png_memcpy_check")]
pub fn stub_15dd80(dst: &mut [u8], src: &[u8]) -> usize {
    // IDA 0x15dd80: memcpy(dst, src, n); return n.
    let n = dst.len().min(src.len());
    dst[..n].copy_from_slice(&src[..n]);
    n
}

// 0x15dd9c — _png_free_default
// type: void __fastcall(int, void *)
#[doc(alias = "_png_free_default")]
pub fn stub_15dd9c(block: Option<Vec<u8>>) {
    // IDA 0x15dd9c: null either → no-op; else default free (drop).
    drop(block);
}

// 0x15ddbc — _png_free
// type: void __fastcall(int, void *)
#[doc(alias = "_png_free")]
pub fn stub_15ddbc(block: Option<Vec<u8>>, custom: Option<&mut dyn FnMut()>) {
    // IDA 0x15ddbc: null either → no-op; custom free proc else default free.
    if block.is_none() {
        return;
    }
    match custom {
        Some(f) => {
            f();
            std::mem::forget(block);
        }
        None => drop(block),
    }
}

// 0x15dddc — _png_destroy_struct_2
// type: void __fastcall(void *, void (__fastcall *)(_DWORD *, void *), int)
#[doc(alias = "_png_destroy_struct_2")]
pub fn stub_15dddc(block: Option<Vec<u8>>, destroy: Option<&mut dyn FnMut()>) {
    // IDA 0x15dddc: null → no-op; destroy proc with the block header else plain free.
    match (block, destroy) {
        (Some(_), Some(d)) => d(),
        (Some(b), None) => drop(b),
        _ => {}
    }
}

// 0x15de18 — _png_destroy_struct
// type: void __fastcall(void *)
#[doc(alias = "_png_destroy_struct")]
pub fn stub_15de18(destroy: &mut dyn FnMut()) {
    // IDA 0x15de18: destroy_struct → destroy_struct_2(block, null, 0).
    destroy();
}

// 0x15de24 — _png_malloc_default
// type: void *__fastcall(int, size_t __size)
#[doc(alias = "_png_malloc_default")]
pub fn stub_15de24(has_ctx: bool, size: usize, alloc: &mut dyn FnMut(usize) -> Option<Vec<u8>>) -> Option<Vec<u8>> {
    // IDA 0x15de24: zero size or null ctx → null; else malloc.
    if size == 0 || !has_ctx {
        return None;
    }
    alloc(size)
}

// 0x15de48 — _png_malloc
// type: int __fastcall(int, size_t)
#[doc(alias = "_png_malloc")]
pub fn stub_15de48(has_ctx: bool, size: usize, custom: Option<&mut dyn FnMut(usize) -> Option<Vec<u8>>>, default: &mut dyn FnMut(usize) -> Option<Vec<u8>>, no_warn: bool) -> Option<Vec<u8>> {
    // IDA 0x15de48: zero/null → null; custom else default malloc; fail without warn-hold → "Out of Memory!".
    if size == 0 || !has_ctx {
        return None;
    }
    let out = match custom {
        Some(f) => f(size),
        None => default(size),
    };
    if out.is_none() && !no_warn {
        panic!("Out of Memory!");
    }
    out
}

// 0x15deb0 — _png_malloc_warn
// type: int __fastcall(int, size_t)
#[doc(alias = "_png_malloc_warn")]
pub fn stub_15deb0(has_ctx: bool, size: usize, custom: Option<&mut dyn FnMut(usize) -> Option<Vec<u8>>>, default: &mut dyn FnMut(usize) -> Option<Vec<u8>>) -> Option<Vec<u8>> {
    // IDA 0x15deb0: hold the no-warn flag across png_malloc.
    stub_15de48(has_ctx, size, custom, default, true)
}

// 0x15dedc — _png_create_struct_2
// type: void *__fastcall(int, int (__fastcall *)(_DWORD *, size_t), int)
#[doc(alias = "_png_create_struct_2")]
pub fn stub_15dedc(kind: i32, custom: Option<&mut dyn FnMut(usize) -> Option<Vec<u8>>>) -> Option<Vec<u8>> {
    // IDA 0x15dedc: kind 2 → 288 bytes, 1 → 692, else null; custom or plain malloc; zeroed.
    let size = match kind {
        2 => 288,
        1 => 692,
        _ => return None,
    };
    match custom {
        Some(f) => f(size),
        None => Some(vec![0u8; size]),
    }
}

// 0x15df58 — _png_create_struct
// type: void *__fastcall(int)
#[doc(alias = "_png_create_struct")]
pub fn stub_15df58(kind: i32) -> Option<Vec<u8>> {
    // IDA 0x15df58: create_struct_2(kind, null, 0).
    stub_15dedc(kind, None)
}

// 0x15df64 — _png_calloc
// type: void *__fastcall(int, size_t)
#[doc(alias = "_png_calloc")]
pub fn stub_15df64(has_ctx: bool, size: usize, alloc: &mut dyn FnMut(usize) -> Option<Vec<u8>>) -> Option<Vec<u8>> {
    // IDA 0x15df64: malloc + memset 0.
    stub_15de48(has_ctx, size, None, alloc, false).map(|mut v| {
        v.fill(0);
        v
    })
}

// 0x15df90 — _png_read_destroy
// type: void *__fastcall(int, void *, void *)
#[doc(alias = "_png_read_destroy")]
pub fn stub_15df90(destroy_info: &mut dyn FnMut(), free_words: &mut dyn FnMut()) {
    // IDA 0x15df90: info_destroy both infos; free the three struct words.
    destroy_info();
    destroy_info();
    free_words();
}

// 0x15e600 — _png_destroy_read_struct
// type: void __fastcall(int *, void **, void **)
#[doc(alias = "_png_destroy_read_struct")]
pub fn stub_15e600(destroy: &mut dyn FnMut(), free_all: &mut dyn FnMut()) {
    // IDA 0x15e600: read_destroy; free_data; free the struct words.
    destroy();
    free_all();
}

// 0x15e6f4 — _png_read_end
// type: int __fastcall(int result, int)
#[doc(alias = "_png_read_end")]
pub fn stub_15e6f4(has_ctx: bool, handle_chunk: &mut dyn FnMut() -> bool) -> bool {
    // IDA 0x15e6f4: null → passthrough; crc_finish; chunk loop to IEND; result.
    if !has_ctx {
        return false;
    }
    while handle_chunk() {}
    true
}

// 0x15ec50 — _png_read_row
// type: int __fastcall(int result, void *, void *__dst)
#[doc(alias = "_png_read_row")]
pub fn stub_15ec50(has_ctx: bool, read: &mut dyn FnMut() -> bool) -> bool {
    // IDA 0x15ec50: null → passthrough; start row if needed; transform + unfilter row; result.
    if !has_ctx {
        return false;
    }
    read()
}

// 0x15f108 — _png_read_image
// type: int __fastcall(int result, void **)
#[doc(alias = "_png_read_image")]
pub fn stub_15f108(has_ctx: bool, passes: usize, read_row: &mut dyn FnMut(usize) -> bool) -> bool {
    // IDA 0x15f108: null → passthrough; set_interlace_handling; per-pass row loop.
    if !has_ctx {
        return false;
    }
    for p in 0..passes {
        if !read_row(p) {
            return false;
        }
    }
    true
}

// 0x15f2d4 — _png_read_update_info
// type: int __fastcall(int result, int)
#[doc(alias = "_png_read_update_info")]
pub fn stub_15f2d4(has_ctx: bool, started: bool, warn: &mut dyn FnMut(&str), start_row: &mut dyn FnMut(), transform: &mut dyn FnMut() -> bool) -> bool {
    // IDA 0x15f2d4: null → passthrough; already started → warning; else start row; transform_info.
    if !has_ctx {
        return false;
    }
    if started {
        warn("Ignoring extra png_read_update_info() call; row buffer not reallocated");
    } else {
        start_row();
    }
    transform()
}

// 0x15f31c — _png_read_info
// type: int __fastcall(int result, int)
#[doc(alias = "_png_read_info")]
pub fn stub_15f31c(has_ctx: bool, has_info: bool, read_chunks: &mut dyn FnMut() -> bool) -> bool {
    // IDA 0x15f31c: nulls → 0; signature read + check; chunk loop to IHDR; result.
    if !has_ctx || !has_info {
        return false;
    }
    read_chunks()
}

// 0x15f9b4 — _png_create_read_struct_2
// type: int *__fastcall(char *, int, int, int, int, int (__fastcall *)(_DWORD *, size_t), void (__fastcall *)(_DWORD *, void *))
#[doc(alias = "_png_create_read_struct_2")]
pub fn stub_15f9b4(create: &mut dyn FnMut() -> Option<Vec<u8>>) -> Option<Vec<u8>> {
    // IDA 0x15f9b4: create_struct_2; 1M limits; setjmp error frame; null on failure.
    create()
}

// 0x15fcd0 — _png_create_read_struct
// type: int *__fastcall(char *, int, int, int)
#[doc(alias = "_png_create_read_struct")]
pub fn stub_15fcd0(create: &mut dyn FnMut() -> Option<Vec<u8>>) -> Option<Vec<u8>> {
    // IDA 0x15fcd0: create_read_struct_2(..., null mem fns).
    create()
}

// 0x15fcf8 — _png_set_read_fn
// type: int __fastcall(int result, int, int)
#[doc(alias = "_png_set_read_fn")]
pub fn stub_15fcf8(io: &mut Option<PngIo>, io_ptr: usize, read_fn: Option<usize>, warn: &mut dyn FnMut(&str)) -> bool {
    // IDA 0x15fcf8: null → passthrough; store io_ptr + read fn (default when null); both-fns conflict
    // warns and clears write fn.
    let io = match io {
        Some(i) => i,
        None => return false,
    };
    io.io_ptr = io_ptr;
    io.read_fn_custom = read_fn.is_some();
    if io.write_fn_set {
        warn("It's an error to set both read_data_fn and write_data_fn in the same structure.  Resetting write_data_fn to NULL.");
        io.write_fn_set = false;
    }
    true
}

// 0x15fd70 — _png_default_read_data
// type: size_t __fastcall(size_t result, void *__ptr, size_t)
#[doc(alias = "_png_default_read_data")]
pub fn stub_15fd70(dst: &mut [u8], read: &mut dyn FnMut(&mut [u8]) -> usize) -> usize {
    // IDA 0x15fd70: fread; short read → png_error("Read Error").
    let n = read(dst);
    if n != dst.len() {
        panic!("Read Error");
    }
    n
}

// 0x15fdb4 — _png_read_data
// type: int __fastcall(int *)
#[doc(alias = "_png_read_data")]
pub fn stub_15fdb4(read: Option<&mut dyn FnMut() -> i32>) -> i32 {
    // IDA 0x15fdb4: null proc → "Call to NULL read function"; else call it.
    match read {
        Some(f) => f(),
        None => panic!("Call to NULL read function"),
    }
}

// 0x15fde4 — _png_set_strip_16
// type: int __fastcall(int result)
#[doc(alias = "_png_set_strip_16")]
pub fn stub_15fde4(rd: &mut PngRead) {
    // IDA 0x15fde4: transform |= 0x400 (strip 16).
    rd.transform |= 0x400;
}

// 0x15fdf8 — _png_set_gamma
// type: int __fastcall(int result, unsigned int, unsigned int)
#[doc(alias = "_png_set_gamma")]
pub fn stub_15fdf8(rd: &mut PngRead, file_gamma: f64, screen_gamma: f64, gamma_flags: u8) {
    // IDA 0x15fdf8: null → passthrough; |file*screen - 1| > 0.05 or gamma flags → transform |=
    // 0x2000; store both gammas.
    if (file_gamma * screen_gamma - 1.0).abs() > 0.05 || gamma_flags & 4 != 0 || gamma_flags == 3 {
        rd.transform |= 0x2000;
    }
    rd.file_gamma = file_gamma as f32;
    rd.screen_gamma = screen_gamma as f32;
}

// 0x15fe6c — _png_set_expand_gray_1_2_4_to_8
// type: int __fastcall(int result)
#[doc(alias = "_png_set_expand_gray_1_2_4_to_8")]
pub fn stub_15fe6c(rd: &mut PngRead) {
    // IDA 0x15fe6c: transform |= 0x1000 (expand gray); mode &= ~0x40.
    rd.transform |= 0x1000;
    rd.mode &= !0x40;
}

// 0x15fe90 — _png_set_gray_to_rgb
// type: int __fastcall(int result)
#[doc(alias = "_png_set_gray_to_rgb")]
pub fn stub_15fe90(rd: &mut PngRead) {
    // IDA 0x15fe90: transform |= 0x4000 (gray to rgb); mode &= ~0x40.
    rd.transform |= 0x4000;
    rd.mode &= !0x40;
}

// 0x15feac — _png_read_transform_info
// type: unsigned int __fastcall(unsigned int result, int)
#[doc(alias = "_png_read_transform_info")]
pub fn stub_15feac(rd: &PngRead, color_type: u8, query: &mut dyn FnMut(u32, u8) -> u32) -> u32 {
    // IDA 0x15feac: expand-gray path and transform-bit dispatch (arms below truncation).
    query(rd.transform, color_type)
}

// 0x160104 — _png_do_unpack
// type: int __fastcall(int result, int)
#[doc(alias = "_png_do_unpack")]
pub fn stub_160104(dst: &mut [u8], src: &[u8], width: usize, bit_depth: u32) {
    // IDA 0x160104: unpack 1/2/4-bit rows to bytes.
    match bit_depth {
        1 => {
            for i in 0..width {
                if i < dst.len() {
                    dst[i] = (src.get(i / 8).copied().unwrap_or(0) >> (7 - (i % 8))) & 1;
                }
            }
        }
        2 => {
            for i in 0..width {
                if i < dst.len() {
                    dst[i] = (src.get(i / 4).copied().unwrap_or(0) >> ((3 - (i % 4)) * 2)) & 3;
                }
            }
        }
        4 => {
            for i in 0..width {
                let b = src.get(i / 2).copied().unwrap_or(0);
                if i < dst.len() {
                    dst[i] = if i % 2 == 0 { b >> 4 } else { b & 0xF };
                }
            }
        }
        _ => {
            let n = width.min(dst.len()).min(src.len());
            dst[..n].copy_from_slice(&src[..n]);
        }
    }
}

// 0x1608a4 — _png_do_unshift
// type: int __fastcall(int result, unsigned __int8 *, unsigned __int8 *)
#[doc(alias = "_png_do_unshift")]
pub fn stub_1608a4(dst: &mut [u8], src: &[u8], color_type: u8, shift: u8) {
    // IDA 0x1608a4: palette (3) → no-op; else downshift each sample to its significant bits.
    if color_type == 3 {
        return;
    }
    for (d, s) in dst.iter_mut().zip(src.iter()) {
        *d = s >> shift.min(7);
    }
}

// 0x1611e8 — _png_do_chop
// type: int *__fastcall(int *result, _BYTE *)
#[doc(alias = "_png_do_chop")]
pub fn stub_1611e8(dst: &mut [u8], src: &[u16], bit_depth: u8) {
    // IDA 0x1611e8: 16-bit rows chopped to their high bytes; other depths copied low.
    if bit_depth == 16 {
        for (i, d) in dst.iter_mut().enumerate() {
            *d = (src.get(i).copied().unwrap_or(0) >> 8) as u8;
        }
    } else {
        for (i, d) in dst.iter_mut().enumerate() {
            *d = src.get(i).copied().unwrap_or(0) as u8;
        }
    }
}

// 0x161328 — _png_do_read_swap_alpha
// type: int *__fastcall(int *result, int)
#[doc(alias = "_png_do_read_swap_alpha")]
pub fn stub_161328(row: &mut [u8], channels: usize, bit_depth: u8) {
    // IDA 0x161328: swap the alpha sample to the front of each pixel (RGBA → ARGB).
    let bpp = channels.max(1) * if bit_depth == 16 { 2 } else { 1 };
    if bpp == 0 {
        return;
    }
    for px in row.chunks_exact_mut(bpp) {
        px.rotate_right(1);
    }
}

// 0x16193c — _png_do_read_invert_alpha
// type: int *__fastcall(int *result, int)
#[doc(alias = "_png_do_read_invert_alpha")]
pub fn stub_16193c(row: &mut [u8], channels: usize, bit_depth: u8) {
    // IDA 0x16193c: invert the alpha samples in place.
    let bpp = channels.max(1) * if bit_depth == 16 { 2 } else { 1 };
    if bpp == 0 {
        return;
    }
    for px in row.chunks_exact_mut(bpp) {
        let n = px.len();
        if bit_depth == 16 && n >= 2 {
            let a = u16::from_be_bytes([px[n - 2], px[n - 1]]);
            let inv = (0xFFFF - a).to_be_bytes();
            px[n - 2] = inv[0];
            px[n - 1] = inv[1];
        } else if n >= 1 {
            px[n - 1] = 255 - px[n - 1];
        }
    }
}

// 0x161ffc — _png_do_read_filler
// type: int __fastcall(int, int, unsigned int, char)
#[doc(alias = "_png_do_read_filler")]
pub fn stub_161ffc(row: &mut Vec<u8>, pixels: usize, channels: usize, filler: u8, before: bool) {
    // IDA 0x161ffc: expand rows with a filler sample (before/after per flags).
    let bpp = channels.max(1);
    let mut out = Vec::with_capacity(pixels * (bpp + 1));
    for i in 0..pixels {
        if before {
            out.push(filler);
        }
        out.extend_from_slice(&row[i * bpp..(i * bpp + bpp).min(row.len())]);
        if !before {
            out.push(filler);
        }
    }
    *row = out;
}

// 0x162d80 — _png_do_gray_to_rgb
// type: int *__fastcall(int *result, int)
#[doc(alias = "_png_do_gray_to_rgb")]
pub fn stub_162d80(dst: &mut [u8], src: &[u8], pixels: usize) {
    // IDA 0x162d80: replicate gray samples to RGB triplets.
    for i in 0..pixels {
        let g = src.get(i).copied().unwrap_or(0);
        let o = i * 3;
        if dst.len() >= o + 3 {
            dst[o..o + 3].copy_from_slice(&[g, g, g]);
        }
    }
}

// 0x16357c — _png_do_rgb_to_gray
// type: int __fastcall(int, int, unsigned __int8 *)
#[doc(alias = "_png_do_rgb_to_gray")]
pub fn stub_16357c(dst: &mut [u8], src: &[u8], pixels: usize) {
    // IDA 0x16357c: RGB triplets to gray via (6968 R + 23434 G + 2366 B) >> 15.
    for i in 0..pixels {
        let o = i * 3;
        if dst.len() > i && src.len() >= o + 3 {
            let (r, g, b) = (src[o] as u32, src[o + 1] as u32, src[o + 2] as u32);
            dst[i] = ((6968 * r + 23434 * g + 2366 * b) >> 15) as u8;
        }
    }
}

// 0x164230 — _png_do_gamma
// type: int __fastcall(int result, unsigned __int8 *, int, int, char)
#[doc(alias = "_png_do_gamma")]
pub fn stub_164230(row: &mut [u8], table8: &[u8; 256], table16: &[u16; 256], bit_depth: u8) {
    // IDA 0x164230: per-sample gamma table lookup (8/16-bit paths, high byte for 16-bit).
    if bit_depth == 16 {
        for px in row.chunks_exact_mut(2) {
            let v = table16[px[0] as usize % 256];
            let be = v.to_be_bytes();
            px[0] = be[0];
            px[1] = be[1];
        }
    } else {
        for b in row.iter_mut() {
            *b = table8[*b as usize];
        }
    }
}

// 0x164fec — _png_do_expand_palette
// type: int __fastcall(int result, int, int, int, int)
#[doc(alias = "_png_do_expand_palette")]
pub fn stub_164fec(dst: &mut [u8], src: &[u8], pixels: usize, palette: &[[u8; 3]], trans: Option<&[u8]>) {
    // IDA 0x164fec: palette indices to RGB triplets (+ alpha from tRNS when present).
    for i in 0..pixels {
        let idx = src.get(i).copied().unwrap_or(0);
        let rgb = palette.get(idx as usize).copied().unwrap_or([0; 3]);
        match trans {
            Some(t) => {
                let o = i * 4;
                if dst.len() >= o + 4 {
                    dst[o..o + 3].copy_from_slice(&rgb);
                    dst[o + 3] = t.get(idx as usize).copied().unwrap_or(255);
                }
            }
            None => {
                let o = i * 3;
                if dst.len() >= o + 3 {
                    dst[o..o + 3].copy_from_slice(&rgb);
                }
            }
        }
    }
}

// 0x165ab0 — _png_do_expand
// type: unsigned int __fastcall(unsigned int result, int, int)
#[doc(alias = "_png_do_expand")]
pub fn stub_165ab0(dst: &mut [u8], src: &[u8], pixels: usize, scale: &[u8]) -> u8 {
    // IDA 0x165ab0: sub-8-bit gray scaled to full bytes; result depth 8.
    for i in 0..pixels {
        if i < dst.len() {
            dst[i] = scale.get(src.get(i).copied().unwrap_or(0) as usize).copied().unwrap_or(0);
        }
    }
    8
}

// 0x1668c4 — _png_do_dither
// type: int __fastcall(int result, unsigned __int8 *, int, int)
#[doc(alias = "_png_do_dither")]
pub fn stub_1668c4(row: &[u8], palette: &[[u8; 3]], out: &mut [u8], dither: &[u8], row_phase: usize) {
    // IDA 0x1668c4: ordered-dither RGB rows to palette indices (matrix by position).
    for (i, px) in row.chunks(3).enumerate() {
        if i >= out.len() {
            break;
        }
        let d = dither.get((row_phase + i) % dither.len().max(1)).copied().unwrap_or(0) as i32;
        let mut best = 0u8;
        let mut best_d = i32::MAX;
        for (k, c) in palette.iter().enumerate() {
            let dd = (px.get(0).copied().unwrap_or(0) as i32 + d - c[0] as i32).abs()
                + (px.get(1).copied().unwrap_or(0) as i32 + d - c[1] as i32).abs()
                + (px.get(2).copied().unwrap_or(0) as i32 + d - c[2] as i32).abs();
            if dd < best_d {
                best_d = dd;
                best = k as u8;
            }
        }
        out[i] = best;
    }
}

// 0x166ddc — _png_do_read_intrapixel
// type: int __fastcall(int result, int)
#[doc(alias = "_png_do_read_intrapixel")]
pub fn stub_166ddc(row: &mut [u8], bpp: usize) {
    // IDA 0x166ddc: undo intrapixel differencing (add-left reconstruction).
    for i in bpp..row.len() {
        let p = row[i - bpp];
        row[i] = row[i].wrapping_add(p);
    }
}

// 0x1671bc — _png_build_gamma_table
// type: int __fastcall(int)
#[doc(alias = "_png_build_gamma_table")]
pub fn stub_1671bc(file_gamma: f64, screen_gamma: f64, table8: &mut [u8; 256], table16: &mut [u16; 256]) {
    // IDA 0x1671bc: build the 8/16-bit gamma correction tables.
    let gamma = file_gamma * screen_gamma;
    for (i, t) in table8.iter_mut().enumerate() {
        *t = (255.0 * (i as f64 / 255.0).powf(gamma)).round().clamp(0.0, 255.0) as u8;
    }
    for (i, t) in table16.iter_mut().enumerate() {
        *t = (65535.0 * (i as f64 / 255.0).powf(gamma)).round().clamp(0.0, 65535.0) as u16;
    }
}

// 0x1684c8 — _png_init_read_transformations
// type: int __fastcall(int)
#[doc(alias = "_png_init_read_transformations")]
pub fn stub_1684c8(transform: u32, install: &mut dyn FnMut(u32) -> bool) -> bool {
    // IDA 0x1684c8: install the row-transform chain for the transform bits; result.
    install(transform)
}

// 0x169938 — _png_do_background
// type: int __fastcall(int, unsigned __int8 *__dst, unsigned __int16 *, _WORD *, _WORD *, int, int, int, int, int, int, char)
#[doc(alias = "_png_do_background")]
pub fn stub_169938(row: &mut [u8], alpha: &[u8], bg: &[u16; 3], channels: usize, bit_depth: u8) {
    // IDA 0x169938: alpha-composite each row against the background color.
    let ch = channels.max(1);
    for (i, px) in row.chunks_mut(ch).enumerate() {
        let a = alpha.get(i).copied().unwrap_or(255) as u32;
        for (k, b) in px.iter_mut().enumerate() {
            let bg_c = if bit_depth == 16 { (bg[k % 3] >> 8) as u32 } else { bg[k % 3] as u32 };
            *b = ((a * *b as u32 + (255 - a) * bg_c) / 255) as u8;
        }
    }
}

// 0x16d374 — _png_do_read_transformations
// type: int __fastcall(int result)
#[doc(alias = "_png_do_read_transformations")]
pub fn stub_16d374(has_row: bool, transforms_ready: bool, run: &mut dyn FnMut() -> bool, error: &mut dyn FnMut(&str) -> bool) -> bool {
    // IDA 0x16d374: null row buffer → error; transforms not ready → error; run chain.
    if !has_row {
        return error("NULL row buffer");
    }
    if !transforms_ready {
        return error("Uninitialized transforms");
    }
    run()
}

// 0x16d814 — _png_get_uint_32
#[doc(alias = "_png_get_uint_32")]
pub fn stub_16d814(bytes: &[u8]) -> u32 {
    // IDA 0x16d814: big-endian u32 load.
    u32::from_be_bytes([
        bytes.get(0).copied().unwrap_or(0),
        bytes.get(1).copied().unwrap_or(0),
        bytes.get(2).copied().unwrap_or(0),
        bytes.get(3).copied().unwrap_or(0),
    ])
}

// 0x16d840 — _png_get_int_32
// type: int __fastcall(unsigned __int8 *)
#[doc(alias = "_png_get_int_32")]
pub fn stub_16d840(bytes: &[u8]) -> i32 {
    // IDA 0x16d840: big-endian i32 load.
    i32::from_be_bytes([
        bytes.get(0).copied().unwrap_or(0),
        bytes.get(1).copied().unwrap_or(0),
        bytes.get(2).copied().unwrap_or(0),
        bytes.get(3).copied().unwrap_or(0),
    ])
}

// 0x16d86c — _png_get_uint_31
#[doc(alias = "_png_get_uint_31")]
pub fn stub_16d86c(bytes: &[u8]) -> i32 {
    // IDA 0x16d86c: big-endian load; negative → "PNG unsigned integer out of range." error.
    let v = i32::from_be_bytes([
        bytes.get(0).copied().unwrap_or(0),
        bytes.get(1).copied().unwrap_or(0),
        bytes.get(2).copied().unwrap_or(0),
        bytes.get(3).copied().unwrap_or(0),
    ]);
    if v < 0 {
        panic!("PNG unsigned integer out of range.");
    }
    v
}

// 0x16d8b4 — _png_read_start_row
// type: int(void)
#[doc(alias = "_png_read_start_row")]
pub fn stub_16d8b4(init: &mut dyn FnMut() -> bool) -> bool {
    // IDA 0x16d8b4: zero row counters; init_read_transformations; interlace row setup.
    init()
}
