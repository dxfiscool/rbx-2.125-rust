//! rendering — generated_206 — 100 stubs EA-sorted asc filtered Ogre|G3D|Rendering|Adorn 51f40..3a94e0
//! Filter: Ogre|G3D|Rendering|Adorn remaining 12268 prior -> uses global gap filler EA-sorted asc (global unstub 51004 before)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;
// ---- Local G3D/RBX mirror types ----
// Storage layouts match the binary; see the per-EA notes below.
// `Matrix3` is column-major like `G3D::Matrix3`: element (row r, col c) is
// `data[c * 3 + r]` (IDA `0xc3fd1c` orthonormalizes row 0 from [0],[3],[6]).
use std::num::FpCategory;

/// was: `G3D::Vector3` — three consecutive floats.
#[doc(alias = "G3D::Vector3")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 { x, y, z }
    }
    pub fn splat(v: f32) -> Self {
        Vector3 { x: v, y: v, z: v }
    }
    /// was: `G3D::Vector3::unitX`.
    pub fn unit_x() -> Self {
        Vector3 { x: 1.0, y: 0.0, z: 0.0 }
    }
    pub fn dot(&self, other: &Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn norm_squared(&self) -> f32 {
        self.dot(self)
    }
    pub fn magnitude(&self) -> f32 {
        self.norm_squared().sqrt()
    }
    fn component(&self, index: usize) -> f32 {
        match index {
            0 => self.x,
            1 => self.y,
            _ => self.z,
        }
    }
}

impl std::ops::Neg for Vector3 {
    type Output = Vector3;
    fn neg(self) -> Vector3 {
        Vector3 { x: -self.x, y: -self.y, z: -self.z }
    }
}

/// was: `G3D::Matrix3` — 9 floats, column-major.
#[doc(alias = "G3D::Matrix3")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Matrix3 {
    pub data: [f32; 9],
}

impl Matrix3 {
    /// was: `G3D::Matrix3::identity`.
    pub fn identity() -> Self {
        Matrix3 { data: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0] }
    }
    fn element(&self, row: usize, col: usize) -> f32 {
        self.data[col * 3 + row]
    }
    /// was: `G3D::Matrix3::column(int)` — columns are contiguous triples.
    pub fn column(&self, index: usize) -> Vector3 {
        Vector3 {
            x: self.data[index * 3],
            y: self.data[index * 3 + 1],
            z: self.data[index * 3 + 2],
        }
    }
    /// was: `G3D::Matrix3::setColumn(int,G3D::Vector3 const&)`.
    pub fn set_column(&mut self, index: usize, value: &Vector3) {
        self.data[index * 3] = value.x;
        self.data[index * 3 + 1] = value.y;
        self.data[index * 3 + 2] = value.z;
    }
    /// was: `G3D::Matrix3::transpose` (out-of-place form).
    pub fn transposed(&self) -> Self {
        let mut out = Matrix3 { data: [0.0; 9] };
        for row in 0..3 {
            for col in 0..3 {
                out.data[col * 3 + row] = self.element(col, row);
            }
        }
        out
    }
    /// was: `G3D::Matrix3::operator*` — standard row-by-column product.
    pub fn mul(&self, rhs: &Matrix3) -> Self {
        let mut out = Matrix3 { data: [0.0; 9] };
        for row in 0..3 {
            for col in 0..3 {
                let mut acc = 0.0f32;
                for k in 0..3 {
                    acc += self.element(row, k) * rhs.element(k, col);
                }
                out.data[col * 3 + row] = acc;
            }
        }
        out
    }
    /// G3D epsilon from IDA `0xc3f068` (`0.00000999999975`, i.e. ~1e-5).
    const ORTHONORMAL_EPS: f32 = 1e-5;
    /// was: `G3D::Matrix3::isOrthonormal` (IDA `0xc3f068`): every column pair
    /// dots ~0 and every column norm-squared ~1 under `fuzzy_eq` with 1e-5.
    pub fn is_orthonormal(&self) -> bool {
        let c0 = self.column(0);
        let c1 = self.column(1);
        let c2 = self.column(2);
        g3d_fuzzy_eq(c0.dot(&c1), 0.0)
            && g3d_fuzzy_eq(c1.dot(&c2), 0.0)
            && g3d_fuzzy_eq(c0.dot(&c2), 0.0)
            && g3d_fuzzy_eq(c0.norm_squared(), 1.0)
            && g3d_fuzzy_eq(c1.norm_squared(), 1.0)
            && g3d_fuzzy_eq(c2.norm_squared(), 1.0)
    }
    /// was: `G3D::Matrix3::orthonormalize` (IDA `0xc3fd1c`): Gram-Schmidt over
    /// rows with a `1.0 / sqrt` (double `sqrt`, narrowed to float) scale.
    pub fn orthonormalize(&mut self) {
        let row = |m: &Matrix3, r: usize| [m.data[r], m.data[r + 3], m.data[r + 6]];
        let set_row = |m: &mut Matrix3, r: usize, v: [f32; 3]| {
            m.data[r] = v[0];
            m.data[r + 3] = v[1];
            m.data[r + 6] = v[2];
        };
        let scale = |v: [f32; 3]| {
            let len2 = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]) as f64;
            (1.0f64 / len2.sqrt()) as f32
        };
        let r0 = row(self, 0);
        let s0 = scale(r0);
        let r0 = [r0[0] * s0, r0[1] * s0, r0[2] * s0];
        set_row(self, 0, r0);
        let r1 = row(self, 1);
        let d = r0[0] * r1[0] + r0[1] * r1[1] + r0[2] * r1[2];
        let r1 = [r1[0] - r0[0] * d, r1[1] - r0[1] * d, r1[2] - r0[2] * d];
        let s1 = scale(r1);
        let r1 = [r1[0] * s1, r1[1] * s1, r1[2] * s1];
        set_row(self, 1, r1);
        let r2 = row(self, 2);
        let d0 = r0[0] * r2[0] + r0[1] * r2[1] + r0[2] * r2[2];
        let d1 = r1[0] * r2[0] + r1[1] * r2[1] + r1[2] * r2[2];
        let r2 = [
            r2[0] - (r1[0] * d1 + r0[0] * d0),
            r2[1] - (r0[1] * d0 + d1 * r1[1]),
            r2[2] - (d0 * r0[2] + d1 * r1[2]),
        ];
        let s2 = scale(r2);
        set_row(self, 2, [r2[0] * s2, r2[1] * s2, r2[2] * s2]);
    }
}

/// G3D `fuzzyEq` as used by IDA `0xc3f068`: exact equality wins, else
/// `|a-b| <= (|a|+1)*1e-5`, with a bare-epsilon fallback when `|a|+1` is
/// infinite.
fn g3d_fuzzy_eq(a: f32, b: f32) -> bool {
    if a == b {
        return true;
    }
    let scale = a.abs() + 1.0;
    let tol = if scale == f32::INFINITY {
        Matrix3::ORTHONORMAL_EPS
    } else {
        scale * Matrix3::ORTHONORMAL_EPS
    };
    (a - b).abs() <= tol
}

/// was: `G3D::CoordinateFrame` — rotation (9 floats) then translation
/// (IDA `0x356d70` reads translation at +9 words, `0x357d44` at +36 bytes).
#[doc(alias = "G3D::CoordinateFrame")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CoordinateFrame {
    pub rotation: Matrix3,
    pub translation: Vector3,
}

/// was: `G3D::Color3` — three consecutive floats (r, g, b).
#[doc(alias = "G3D::Color3")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Color3 {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

/// was: `RBX::Light` (color at +100 bytes per IDA `0x25b4e0`: channels at
/// words 25/26/27). `color_revision` stands in for the
/// `Instance::raisePropertyChanged(prop_Color)` notification.
#[doc(alias = "RBX::Light")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Light {
    pub color: Color3,
    pub color_revision: u32,
}

impl Light {
    /// IDA 0x25b4e0: stores the new color and raises `prop_Color` only when
    /// some channel differs.
    pub fn set_color(&mut self, color: Color3) {
        if self.color != color {
            self.color = color;
            self.color_revision = self.color_revision.wrapping_add(1);
        }
    }
}

/// was: `MainViewController` — iOS controller owning the Ogre surface.
/// Synthesized ivars verified in disasm: `ogreWindow` (`UIWindow *`,
/// 0x51f40/`0x51f50`), `ogreView` (`UIView *`, 0x51f60/`0x51f70`),
/// `ogreViewController` (`GameViewController *`, 0x51fa0/`0x51fb0`).
/// ObjC `id` is a raw object pointer, hence `usize`; the setters are plain
/// assigns with no retain.
#[doc(alias = "MainViewController (iOS)")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MainViewController {
    pub ogre_window: usize,
    pub ogre_view: usize,
    pub ogre_view_controller: usize,
}

impl MainViewController {
    pub fn ogre_window(&self) -> usize {
        self.ogre_window
    }
    pub fn set_ogre_window(&mut self, value: usize) {
        self.ogre_window = value;
    }
    pub fn ogre_view(&self) -> usize {
        self.ogre_view
    }
    pub fn set_ogre_view(&mut self, value: usize) {
        self.ogre_view = value;
    }
    pub fn ogre_view_controller(&self) -> usize {
        self.ogre_view_controller
    }
    pub fn set_ogre_view_controller(&mut self, value: usize) {
        self.ogre_view_controller = value;
    }
}

/// IDA 0x356c3c: strict componentwise all-less. The third C++ parameter is a
/// phantom (the body only compares the first two vectors).
pub fn less_than(a: &Vector3, b: &Vector3) -> bool {
    a.x < b.x && a.y < b.y && a.z < b.z
}

/// IDA 0x356cc8: true when any component is NaN (self-compare `VCMPE S,S` +
/// `BVS` per component in disasm) or infinite (`|x| == INFINITY`).
pub fn is_nan_inf(v: &Vector3) -> bool {
    v.x.is_nan()
        || v.x.is_infinite()
        || v.y.is_nan()
        || v.y.is_infinite()
        || v.z.is_nan()
        || v.z.is_infinite()
}

/// Returns whether a component counts as "bad" under the IDA `0x356d38`
/// machine check (`fpclassifyf(x) - 3 <= 1` unsigned, i.e. class in
/// {subnormal, zero} given the standard FP_ZERO=4/FP_SUBNORMAL=3 enum).
fn is_zero_or_subnormal(x: f32) -> bool {
    matches!(x.classify(), FpCategory::Zero | FpCategory::Subnormal)
}

/// IDA 0x356d38.
// BUG: original at 0x356d38 returns false only when *every* component is
// zero or subnormal, so ordinary normal vectors (and infinities) report
// true despite the "NanInfDenorm" name. Preserved 1:1.
pub fn is_nan_inf_denorm(v: &Vector3) -> bool {
    !(is_zero_or_subnormal(v.x) && is_zero_or_subnormal(v.y) && is_zero_or_subnormal(v.z))
}

/// IDA 0x356d70: true when any of the 12 frame floats has
/// `|x| == INFINITY` (translation first, then rotation).
// BUG: original at 0x356d70 never detects NaN (`|NaN| != INFINITY` falls
// through) despite the "NanOrInf" name. Preserved 1:1.
pub fn has_nan_or_inf(frame: &CoordinateFrame) -> bool {
    for v in [
        frame.translation.x,
        frame.translation.y,
        frame.translation.z,
    ] {
        if v.abs() == f32::INFINITY {
            return true;
        }
    }
    for v in frame.rotation.data {
        if v.abs() == f32::INFINITY {
            return true;
        }
    }
    false
}

/// IDA 0x356b18: `col = rotation.column(2)`; `heading = atan2(col.x, col.z)`
/// and `elevation = asin(-col.y)`, both computed in double (original calls
/// the double `atan2`/`asin`) and narrowed to `f32` for the out-params; the
/// double elevation is also the return value.
pub fn get_heading_elevation(
    frame: &CoordinateFrame,
    heading: &mut f32,
    elevation: &mut f32,
) -> f64 {
    let col = frame.rotation.column(2);
    let h = f64::atan2(col.x as f64, col.z as f64);
    let e = f64::asin(-(col.y as f64));
    *heading = h as f32;
    *elevation = e as f32;
    e
}

/// IDA 0x357250: `out = rotation^T * inertia * rotation`.
pub fn moment_to_object_space(out: &mut Matrix3, inertia: &Matrix3, rotation: &Matrix3) {
    let rt = rotation.transposed();
    *out = rt.mul(inertia).mul(rotation);
}

/// IDA 0x35728c: `out = rotation * inertia * rotation^T`.
pub fn moment_to_world_space(out: &mut Matrix3, inertia: &Matrix3, rotation: &Matrix3) {
    let rt = rotation.transposed();
    *out = rotation.mul(inertia).mul(&rt);
}

/// IDA 0x3572c4: gathers elements [0],[4],[8] (the diagonal either way,
/// row- or column-major) into a vector.
pub fn to_diagonal(matrix: &Matrix3) -> Vector3 {
    Vector3 {
        x: matrix.data[0],
        y: matrix.data[4],
        z: matrix.data[8],
    }
}

/// IDA 0x3575bc: orthonormalizes only when `isOrthonormal` fails; returns
/// whether it ran.
pub fn orthonormalize_if_necessary(matrix: &mut Matrix3) -> bool {
    if !matrix.is_orthonormal() {
        matrix.orthonormalize();
        true
    } else {
        false
    }
}

/// was: `RBX::Vector3ToNormalId(G3D::Vector3 const&)` (IDA `0x35d8a0`):
/// asserts the input is a unit axis, then maps
/// +X/+Y/+Z/-X/-Y/-Z to 0..5 and anything else to 6, via exact float
/// compares.
pub fn vector3_to_normal_id(v: &Vector3) -> i32 {
    debug_assert!(
        *v == Vector3::unit_x()
            || *v == Vector3::new(0.0, 1.0, 0.0)
            || *v == Vector3::new(0.0, 0.0, 1.0)
            || *v == -Vector3::unit_x()
            || *v == Vector3::new(0.0, -1.0, 0.0)
            || *v == Vector3::new(0.0, 0.0, -1.0),
        "(v == Vector3::unitX()) || (v == Vector3::unitY()) || (v == Vector3::unitZ()) \
         || (v == -Vector3::unitX()) || (v == -Vector3::unitY()) || (v == -Vector3::unitZ())"
    );
    if v.x == 1.0 {
        0
    } else if v.y == 1.0 {
        1
    } else if v.z == 1.0 {
        2
    } else if v.x == -1.0 {
        3
    } else if v.y == -1.0 {
        4
    } else if v.z == -1.0 {
        5
    } else {
        debug_assert!(false, "0");
        6
    }
}

/// IDA 0x35781c (column indices verified in disasm):
/// `normalId(column(1)) + 6 * normalId(column(0))`.
pub fn get_orient_id(matrix: &Matrix3) -> i32 {
    vector3_to_normal_id(&matrix.column(1)) + 6 * vector3_to_normal_id(&matrix.column(0))
}

/// IDA 0x3579d0: snaps a rotation to the nearest axis-aligned orientation.
/// Builds the 3x3 table `dots[c][i] = column(c) . identity.column(i)`,
/// takes the largest-|dot| (column, axis) pair first (strict `>`, so ties
/// keep the earliest), then the largest-|dot| pair from the remaining rows
/// and axes, completes the third axis by elimination, and flips any chosen
/// axis whose dot was negative.
pub fn snap_to_axes(out: &mut Matrix3, m: &Matrix3) {
    let identity = Matrix3::identity();
    let mut dots = [0.0f32; 9];
    // Original seeds the winners at -1 and crashes on an all-zero matrix;
    // any real rotation always records a pair, so plain indices suffice.
    let mut best = 0.0f32;
    let mut best_col = 0usize;
    let mut best_axis = 0usize;
    for c in 0..3 {
        let col = m.column(c);
        for i in 0..3 {
            let d = col.dot(&identity.column(i));
            dots[c * 3 + i] = d;
            if d.abs() > best.abs() {
                best = d;
                best_col = c;
                best_axis = i;
            }
        }
    }
    let mut first = identity.column(best_axis);
    if best < 0.0 {
        first = -first;
    }
    let mut best2 = 0.0f32;
    let mut col2 = 0usize;
    let mut axis2 = 0usize;
    for c in 0..3 {
        if c == best_col {
            continue;
        }
        for i in 0..3 {
            if i == best_axis {
                continue;
            }
            let d = dots[c * 3 + i];
            if d.abs() > best2.abs() {
                best2 = d;
                col2 = c;
                axis2 = i;
            }
        }
    }
    let mut second = identity.column(axis2);
    if best2 < 0.0 {
        second = -second;
    }
    let axis3 = 3 - best_axis - axis2;
    let mut third = identity.column(axis3);
    if dots[(3 - best_col - col2) * 3 + axis3] < 0.0 {
        third = -third;
    }
    out.set_column(best_col, &first);
    out.set_column(col2, &second);
    out.set_column(3 - best_col - col2, &third);
}

/// IDA 0x357c08: `out = round(v / grid) * grid` per component via
/// `iRoundVector3`; returns the rounded z.
pub fn to_grid(out: &mut Vector3, value: &Vector3, grid: &Vector3) -> i32 {
    let scaled = Vector3 {
        x: value.x / grid.x,
        y: value.y / grid.y,
        z: value.z / grid.z,
    };
    let mut rounded = Vector3::default();
    let rz = i_round_vector3(&mut rounded, &scaled);
    *out = Vector3 {
        x: rounded.x * grid.x,
        y: rounded.y * grid.y,
        z: rounded.z * grid.z,
    };
    rz
}

/// IDA 0x357c84: `lrintf` per component into float slots; returns the
/// rounded z. `lrintf` rounds per the current mode (round-to-nearest,
/// ties-to-even), hence `round_ties_even`.
pub fn i_round_vector3(out: &mut Vector3, value: &Vector3) -> i32 {
    let rx = value.x.round_ties_even() as i32;
    let ry = value.y.round_ties_even() as i32;
    let rz = value.z.round_ties_even() as i32;
    *out = Vector3 { x: rx as f32, y: ry as f32, z: rz as f32 };
    rz
}

/// IDA 0x357ce4: splats the scalar into `(g, g, g)` and tail-calls the
/// vector-grid form.
pub fn to_grid_scalar(out: &mut Vector3, value: &Vector3, grid: f32) -> i32 {
    to_grid(out, value, &Vector3::splat(grid))
}

/// IDA 0x357d44: rotation comes from `snapToAxes`, translation from
/// `toGrid(translation)`. The C++ returns via the hidden out-pointer; the
/// `int` left in R0 (gridded-z bits) is an ABI artifact, not a contract.
pub fn snap_to_grid(out: &mut CoordinateFrame, frame: &CoordinateFrame, grid: &Vector3) {
    let mut rotation = Matrix3::default();
    snap_to_axes(&mut rotation, &frame.rotation);
    let mut translation = Vector3::default();
    to_grid(&mut translation, &frame.translation, grid);
    *out = CoordinateFrame { rotation, translation };
}

/// IDA 0x357cfc: uniform-grid form of `snapToGrid`.
pub fn snap_to_grid_scalar(out: &mut CoordinateFrame, frame: &CoordinateFrame, grid: f32) {
    snap_to_grid(out, frame, &Vector3::splat(grid))
}

/// IDA 0x357d88 (2-arg `safeDirection` entry; `0x357df4` shares its tail):
/// normalizes with `1.0 / sqrtf(dot)` when the length exceeds 1e-12 and
/// falls back to `unitX` otherwise. The `FLog::Asserts`/`_debugHook` ladders
/// are debug-only diagnostics, ported as `debug_assert!` with the original
/// messages; release `ReleaseAssert` has no Rust equivalent and is noted.
pub fn safe_direction(out: &mut Vector3, direction: &Vector3) {
    let len = (direction.x * direction.x + direction.y * direction.y + direction.z * direction.z).sqrt();
    if len > 1.0e-12 {
        let inv = 1.0 / len;
        let answer = Vector3 {
            x: direction.x * inv,
            y: direction.y * inv,
            z: direction.z * inv,
        };
        debug_assert!(answer.magnitude() < 1.01, "answer.magnitude() < 1.01f");
        *out = answer;
    } else {
        debug_assert!(false, "0");
        *out = Vector3::unit_x();
    }
}

/// IDA 0x357ee4: `acos(dot)` clamped — dot above 1.0 yields 0.0, below -1.0
/// yields 3.1416.
pub fn angle_between(a: &Vector3, b: &Vector3) -> f32 {
    let dot = a.dot(b);
    if dot < 1.0 {
        if dot > -1.0 {
            dot.acos()
        } else {
            3.1416
        }
    } else {
        0.0
    }
}

/// IDA 0x357f48: `asin(y)` with +-1.5708 saturation at y >= 1.0 / y <= -1.0.
pub fn elevation_angle(direction: &Vector3) -> f32 {
    if direction.y >= 1.0 {
        1.5708
    } else if direction.y <= -1.0 {
        -1.5708
    } else {
        (direction.y as f64).asin() as f32
    }
}

/// IDA 0x3580c0: per-component `|a-b| <= eps * (|a|+1.0)` with an
/// exact-equality fast path (note the tolerance is asymmetric in a/b,
/// matching the machine).
pub fn fuzzy_eq(a: &Vector3, b: &Vector3, epsilon: f32) -> bool {
    for i in 0..3 {
        let (x, y) = (a.component(i), b.component(i));
        if x != y && (x - y).abs() > epsilon * (x.abs() + 1.0) {
            return false;
        }
    }
    true
}

// 0x51f40 — -[MainViewController getOgreWindow]
// type: id __cdecl(MainViewController *self, SEL)
#[doc(alias = "-[MainViewController getOgreWindow]")]
// IDA 0x51f40: LDR R0,[R0,ivar ogreWindow] (see MainViewController).
pub fn stub_0x51f40(controller: &MainViewController) -> usize {
    controller.ogre_window()
}

// 0x51f50 — -[MainViewController setOgreWindow:]
// type: void __cdecl(MainViewController *self, SEL, id)
#[doc(alias = "-[MainViewController setOgreWindow:]")]
// IDA 0x51f50: STR R2,[R0,ivar ogreWindow] (UIWindow *); plain assign, no retain (see MainViewController).
pub fn stub_0x51f50(controller: &mut MainViewController, value: usize) {
    controller.set_ogre_window(value)
}

// 0x51f60 — -[MainViewController getOgreView]
// type: id __cdecl(MainViewController *self, SEL)
#[doc(alias = "-[MainViewController getOgreView]")]
// IDA 0x51f60: LDR R0,[R0,ivar ogreView] (see MainViewController).
pub fn stub_0x51f60(controller: &MainViewController) -> usize {
    controller.ogre_view()
}

// 0x51f70 — -[MainViewController setOgreView:]
// type: void __cdecl(MainViewController *self, SEL, id)
#[doc(alias = "-[MainViewController setOgreView:]")]
// IDA 0x51f70: STR R2,[R0,ivar ogreView] (UIView *); plain assign, no retain (see MainViewController).
pub fn stub_0x51f70(controller: &mut MainViewController, value: usize) {
    controller.set_ogre_view(value)
}

// 0x51fa0 — -[MainViewController getOgreViewController]
// type: id __cdecl(MainViewController *self, SEL)
#[doc(alias = "-[MainViewController getOgreViewController]")]
// IDA 0x51fa0: LDR R0,[R0,ivar ogreViewController] (see MainViewController).
pub fn stub_0x51fa0(controller: &MainViewController) -> usize {
    controller.ogre_view_controller()
}

// 0x51fb0 — -[MainViewController setOgreViewController:]
// type: void __cdecl(MainViewController *self, SEL, id)
#[doc(alias = "-[MainViewController setOgreViewController:]")]
// IDA 0x51fb0: STR R2,[R0,ivar ogreViewController] (GameViewController *); plain assign, no retain (see MainViewController).
pub fn stub_0x51fb0(controller: &mut MainViewController, value: usize) {
    controller.set_ogre_view_controller(value)
}

// 0x25b4e0 — __ZN3RBX5Light8setColorEN3G3D6Color3E
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "RBX::Light::setColor(G3D::Color3)")]
// IDA 0x25b4e0: exact-match fast path skips raisePropertyChanged(prop_Color); any channel differs stores + notifies (see Light::set_color).
pub fn stub_0x25b4e0(light: &mut Light, color: &Color3) {
    light.set_color(*color)
}

// 0x356ae0 — __ZN3RBX4Math13getFocusSpaceERKN3G3D15CoordinateFrameE
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::CoordinateFrame *)
#[doc(alias = "RBX::Math::getFocusSpace(G3D::CoordinateFrame const&)")]
// IDA 0x356ae0: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x356ae0() {
}

// 0x356b18 — __ZN3RBX4Math19getHeadingElevationERKN3G3D15CoordinateFrameERfS5_
// type: double __fastcall(RBX::Math *this, const G3D::CoordinateFrame *, float *, float *)
#[doc(alias = "RBX::Math::getHeadingElevation(G3D::CoordinateFrame const&,float &,float &)")]
// IDA 0x356b18: heading = atan2(col2.x, col2.z), elevation = asin(-col2.y) (see get_heading_elevation).
pub fn stub_0x356b18(frame: &CoordinateFrame, heading: &mut f32, elevation: &mut f32) -> f64 {
    get_heading_elevation(frame, heading, elevation)
}

// 0x356b84 — __ZN3RBX4Math19setHeadingElevationERN3G3D15CoordinateFrameEff
// type: _DWORD __fastcall(RBX::Math *__hidden this, G3D::CoordinateFrame *, float, float)
#[doc(alias = "RBX::Math::setHeadingElevation(G3D::CoordinateFrame &,float,float)")]
// IDA 0x356b84: 53 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x356b84() {
}

// 0x356c3c — __ZN3RBX4Math8lessThanERKN3G3D7Vector3ES4_
// type: _DWORD __fastcall(RBX::Math *__hidden this, const Vector3 *, const Vector3 *)
#[doc(alias = "RBX::Math::lessThan(G3D::Vector3 const&,G3D::Vector3 const&)")]
// IDA 0x356c3c: strict componentwise x/y/z all-less (see less_than).
pub fn stub_0x356c3c(a: &Vector3, b: &Vector3) -> bool {
    less_than(a, b)
}

// 0x356cc8 — __ZN3RBX4Math15isNanInfVector3ERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, const Vector3 *)
#[doc(alias = "RBX::Math::isNanInfVector3(G3D::Vector3 const&)")]
// IDA 0x356cc8: per-component NaN self-compare plus |x| == INFINITY (see is_nan_inf).
pub fn stub_0x356cc8(value: &Vector3) -> bool {
    is_nan_inf(value)
}

// 0x356d38 — __ZN3RBX4Math21isNanInfDenormVector3ERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, const Vector3 *)
#[doc(alias = "RBX::Math::isNanInfDenormVector3(G3D::Vector3 const&)")]
// IDA 0x356d38: fpclassify-3 machine check; false only when every component is zero/subnormal (see is_nan_inf_denorm).
pub fn stub_0x356d38(value: &Vector3) -> bool {
    is_nan_inf_denorm(value)
}

// 0x356d70 — __ZN3RBX4Math11hasNanOrInfERKN3G3D15CoordinateFrameE
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::CoordinateFrame *)
#[doc(alias = "RBX::Math::hasNanOrInf(G3D::CoordinateFrame const&)")]
// IDA 0x356d70: any |component| == INFINITY across translation then rotation (see has_nan_or_inf).
pub fn stub_0x356d70(frame: &CoordinateFrame) -> bool {
    has_nan_or_inf(frame)
}

// 0x356df4 — __ZN3RBX4Math9fixDenormERN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, Vector3 *)
#[doc(alias = "RBX::Math::fixDenorm(G3D::Vector3 &)")]
// IDA 0x356df4: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x356df4() {
}

// 0x35711c — __ZN3RBX4Math16getIWorldAtPointERKN3G3D7Vector3ES4_RKNS1_7Matrix3Ef
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Vector3 *, const G3D::Vector3 *, const G3D::Matrix3 *, float)
#[doc(alias = "RBX::Math::getIWorldAtPoint(G3D::Vector3 const&,G3D::Vector3 const&,G3D::Matrix3 const&,float)")]
// IDA 0x35711c: 49 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x35711c() {
}

// 0x3571c0 — __ZN3RBX4Math15getIBodyAtPointERKN3G3D7Vector3ERKNS1_7Matrix3Ef
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Vector3 *, const G3D::Matrix3 *, float)
#[doc(alias = "RBX::Math::getIBodyAtPoint(G3D::Vector3 const&,G3D::Matrix3 const&,float)")]
// IDA 0x3571c0: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3571c0() {
}

// 0x357250 — __ZN3RBX4Math19momentToObjectSpaceERKN3G3D7Matrix3ES4_
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Matrix3 *, const G3D::Matrix3 *)
#[doc(alias = "RBX::Math::momentToObjectSpace(G3D::Matrix3 const&,G3D::Matrix3 const&)")]
// IDA 0x357250: out = rot^T * inertia * rot (see moment_to_object_space).
pub fn stub_0x357250(out: &mut Matrix3, inertia: &Matrix3, rotation: &Matrix3) {
    moment_to_object_space(out, inertia, rotation)
}

// 0x35728c — __ZN3RBX4Math18momentToWorldSpaceERKN3G3D7Matrix3ES4_
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Matrix3 *, const G3D::Matrix3 *)
#[doc(alias = "RBX::Math::momentToWorldSpace(G3D::Matrix3 const&,G3D::Matrix3 const&)")]
// IDA 0x35728c: out = rot * inertia * rot^T (see moment_to_world_space).
pub fn stub_0x35728c(out: &mut Matrix3, inertia: &Matrix3, rotation: &Matrix3) {
    moment_to_world_space(out, inertia, rotation)
}

// 0x3572c4 — __ZN3RBX4Math10toDiagonalERKN3G3D7Matrix3E
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "RBX::Math::toDiagonal(G3D::Matrix3 const&)")]
// IDA 0x3572c4: copies elements [0],[4],[8] (the diagonal) into a Vector3 (see to_diagonal).
pub fn stub_0x3572c4(matrix: &Matrix3) -> Vector3 {
    to_diagonal(matrix)
}

// 0x3572e4 — __ZN3RBX4Math26fromVectorToVectorRotationERKN3G3D7Vector3ES4_
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Vector3 *, const G3D::Vector3 *)
#[doc(alias = "RBX::Math::fromVectorToVectorRotation(G3D::Vector3 const&,G3D::Vector3 const&)")]
// IDA 0x3572e4: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3572e4() {
}

// 0x357450 — __ZN3RBX4Math24fromRotationAxisAndAngleERKN3G3D7Vector3ERKf
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Vector3 *, const float *)
#[doc(alias = "RBX::Math::fromRotationAxisAndAngle(G3D::Vector3 const&,float const&)")]
// IDA 0x357450: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x357450() {
}

// 0x3575bc — __ZN3RBX4Math25orthonormalizeIfNecessaryERN3G3D7Matrix3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, G3D::Matrix3 *)
#[doc(alias = "RBX::Math::orthonormalizeIfNecessary(G3D::Matrix3 &)")]
// IDA 0x3575bc: orthonormalizes only when isOrthonormal fails, returns whether it ran (see orthonormalize_if_necessary).
pub fn stub_0x3575bc(matrix: &mut Matrix3) -> bool {
    orthonormalize_if_necessary(matrix)
}

// 0x3575dc — __ZN3RBX4Math20fromDirectionCosinesERKN3G3D7Vector3ES4_S4_S4_S4_S4_
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Vector3 *, const G3D::Vector3 *, const G3D::Vector3 *, const G3D::Vector3 *, const G3D::Vector3 *, const G3D::Vector3 *)
#[doc(alias = "RBX::Math::fromDirectionCosines(G3D::Vector3 const&,G3D::Vector3 const&,G3D::Vector3 const&,G3D::Vector3 const&,G3D::Vector3 const&,G3D::Vector3 const&)")]
// IDA 0x3575dc: 97 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3575dc() {
}

// 0x357744 — __ZN3RBX4Math13isAxisAlignedERKN3G3D7Matrix3E
// type: int __fastcall(RBX::Math *this, const G3D::Matrix3 *)
#[doc(alias = "RBX::Math::isAxisAligned(G3D::Matrix3 const&)")]
// IDA 0x357744: 85 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x357744() {
}

// 0x35781c — __ZN3RBX4Math11getOrientIdERKN3G3D7Matrix3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Matrix3 *)
#[doc(alias = "RBX::Math::getOrientId(G3D::Matrix3 const&)")]
// IDA 0x35781c: normalId(col1) + 6*normalId(col0); column indices verified in disasm (see get_orient_id).
pub fn stub_0x35781c(matrix: &Matrix3) -> i32 {
    get_orient_id(matrix)
}

// 0x357858 — __ZN3RBX4Math11idToMatrix3EiRN3G3D7Matrix3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, int, G3D::Matrix3 *)
#[doc(alias = "RBX::Math::idToMatrix3(int,G3D::Matrix3 &)")]
// IDA 0x357858: 62 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x357858() {
}

// 0x357924 — __ZN3RBX4Math12rotateAboutZERKN3G3D7Matrix3Ef
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Matrix3 *, float)
#[doc(alias = "RBX::Math::rotateAboutZ(G3D::Matrix3 const&,float)")]
// IDA 0x357924: 56 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x357924() {
}

// 0x3579d0 — __ZN3RBX4Math10snapToAxesERKN3G3D7Matrix3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Matrix3 *)
#[doc(alias = "RBX::Math::snapToAxes(G3D::Matrix3 const&)")]
// IDA 0x3579d0: 180 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3579d0() {
}

// 0x357c08 — __ZN3RBX4Math6toGridERKN3G3D7Vector3ES4_
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Vector3 *, const G3D::Vector3 *)
#[doc(alias = "RBX::Math::toGrid(G3D::Vector3 const&,G3D::Vector3 const&)")]
// IDA 0x357c08: round(v/grid)*grid per component via iRoundVector3 (see to_grid).
pub fn stub_0x357c08(out: &mut Vector3, value: &Vector3, grid: &Vector3) -> i32 {
    to_grid(out, value, grid)
}

// 0x357c84 — __ZN3RBX4Math13iRoundVector3ERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, const Vector3 *)
#[doc(alias = "RBX::Math::iRoundVector3(G3D::Vector3 const&)")]
// IDA 0x357c84: lrintf per component, returns the rounded z (see i_round_vector3).
pub fn stub_0x357c84(out: &mut Vector3, value: &Vector3) -> i32 {
    i_round_vector3(out, value)
}

// 0x357ce4 — __ZN3RBX4Math6toGridERKN3G3D7Vector3Ef
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Vector3 *, float)
#[doc(alias = "RBX::Math::toGrid(G3D::Vector3 const&,float)")]
// IDA 0x357ce4: splats the scalar into (g,g,g) and tail-calls toGrid(vec,vec) (see to_grid_scalar).
pub fn stub_0x357ce4(out: &mut Vector3, value: &Vector3, grid: f32) -> i32 {
    to_grid_scalar(out, value, grid)
}

// 0x357cfc — __ZN3RBX4Math10snapToGridERKN3G3D15CoordinateFrameEf
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::CoordinateFrame *, float)
#[doc(alias = "RBX::Math::snapToGrid(G3D::CoordinateFrame const&,float)")]
// IDA 0x357cfc: uniform-grid form builds (g,g,g) then snaps (see snap_to_grid_scalar).
pub fn stub_0x357cfc(out: &mut CoordinateFrame, frame: &CoordinateFrame, grid: f32) {
    snap_to_grid_scalar(out, frame, grid)
}

// 0x357d44 — __ZN3RBX4Math10snapToGridERKN3G3D15CoordinateFrameERKNS1_7Vector3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::CoordinateFrame *, const G3D::Vector3 *)
#[doc(alias = "RBX::Math::snapToGrid(G3D::CoordinateFrame const&,G3D::Vector3 const&)")]
// IDA 0x357d44: rotation = snapToAxes, translation = toGrid(translation) (see snap_to_grid).
pub fn stub_0x357d44(out: &mut CoordinateFrame, frame: &CoordinateFrame, grid: &Vector3) {
    snap_to_grid(out, frame, grid)
}

// 0x357d88 — __ZN3RBX4Math13safeDirectionERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Vector3 *)
#[doc(alias = "RBX::Math::safeDirection(G3D::Vector3 const&)")]
// IDA 0x357d88: normalize with 1e-12 fallback to unitX plus magnitude asserts (see safe_direction).
pub fn stub_0x357d88(out: &mut Vector3, direction: &Vector3) {
    safe_direction(out, direction)
}

// 0x357ee4 — __ZN3RBX4Math5angleERKN3G3D7Vector3ES4_
// type: _DWORD __fastcall(RBX::Math *__hidden this, const Vector3 *, const Vector3 *)
#[doc(alias = "RBX::Math::angle(G3D::Vector3 const&,G3D::Vector3 const&)")]
// IDA 0x357ee4: acos(dot) with >1.0 yielding 0.0 and <-1.0 yielding 3.1416 (see angle_between).
pub fn stub_0x357ee4(a: &Vector3, b: &Vector3) -> f32 {
    angle_between(a, b)
}

// 0x357f48 — __ZN3RBX4Math14elevationAngleERKN3G3D7Vector3E
// type: int __fastcall(RBX::Math *this, const Vector3 *)
#[doc(alias = "RBX::Math::elevationAngle(G3D::Vector3 const&)")]
// IDA 0x357f48: asin(y) clamped to +-1.5708 at +-1.0 (see elevation_angle).
pub fn stub_0x357f48(direction: &Vector3) -> f32 {
    elevation_angle(direction)
}

// 0x357fa0 — __ZN3RBX4Math16fuzzyAxisAlignedERKN3G3D7Matrix3ES4_f
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Matrix3 *, const G3D::Matrix3 *, float)
#[doc(alias = "RBX::Math::fuzzyAxisAligned(G3D::Matrix3 const&,G3D::Matrix3 const&,float)")]
// IDA 0x357fa0: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x357fa0() {
}

// 0x3580b4 — __ZN3RBX4Math13isOrthonormalERKN3G3D7Matrix3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Matrix3 *)
#[doc(alias = "RBX::Math::isOrthonormal(G3D::Matrix3 const&)")]
// IDA 0x3580b4: tail-calls G3D::Matrix3::isOrthonormal (see Matrix3::is_orthonormal).
pub fn stub_0x3580b4(matrix: &Matrix3) -> bool {
    matrix.is_orthonormal()
}

// 0x3580c0 — __ZN3RBX4Math7fuzzyEqERKN3G3D7Vector3ES4_f
// type: _DWORD __fastcall(RBX::Math *__hidden this, const Vector3 *, const Vector3 *, float)
#[doc(alias = "RBX::Math::fuzzyEq(G3D::Vector3 const&,G3D::Vector3 const&,float)")]
// IDA 0x3580c0: per-component |a-b| <= eps*(|a|+1.0), exact-equality fast path (see fuzzy_eq).
pub fn stub_0x3580c0(a: &Vector3, b: &Vector3, epsilon: f32) -> bool {
    fuzzy_eq(a, b, epsilon)
}

// 0x35810c — __ZN3RBX4Math7fuzzyEqERKN3G3D7Matrix3ES4_f
#[doc(alias = "RBX::Math::fuzzyEq(G3D::Matrix3 const&,G3D::Matrix3 const&,float)")]
// IDA 0x35810c: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x35810c() {
}

// 0x35817c — __ZN3RBX4Math7fuzzyEqERKN3G3D7Matrix4ES4_f
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Matrix4 *, const G3D::Matrix4 *, float)
#[doc(alias = "RBX::Math::fuzzyEq(G3D::Matrix4 const&,G3D::Matrix4 const&,float)")]
// IDA 0x35817c: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x35817c() {
}

// 0x3581ec — __ZN3RBX4Math7fuzzyEqERKN3G3D15CoordinateFrameES4_ff
#[doc(alias = "RBX::Math::fuzzyEq(G3D::CoordinateFrame const&,G3D::CoordinateFrame const&,float,float)")]
// IDA 0x3581ec: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3581ec() {
}

// 0x358254 — __ZN3RBX4Math18rotateAboutYGlobalERN3G3D15CoordinateFrameEf
// type: _DWORD __fastcall(RBX::Math *__hidden this, G3D::CoordinateFrame *, float)
#[doc(alias = "RBX::Math::rotateAboutYGlobal(G3D::CoordinateFrame &,float)")]
// IDA 0x358254: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x358254() {
}

// 0x35829c — __ZN3RBX4Math18rotateAboutYGlobalERKN3G3D7Vector3Ef
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Vector3 *, float)
#[doc(alias = "RBX::Math::rotateAboutYGlobal(G3D::Vector3 const&,float)")]
// IDA 0x35829c: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x35829c() {
}

// 0x358314 — __ZN3RBX4Math24getClosestObjectNormalIdERKN3G3D7Vector3ERKNS1_7Matrix3E
#[doc(alias = "RBX::Math::getClosestObjectNormalId(G3D::Vector3 const&,G3D::Matrix3 const&)")]
// IDA 0x358314: 58 insns (SUB..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x358314() {
}

// 0x3583cc — __ZN3RBX4Math11sortVector3ERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, const Vector3 *)
#[doc(alias = "RBX::Math::sortVector3(G3D::Vector3 const&)")]
// IDA 0x3583cc: 28 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3583cc() {
}

// 0x358430 — __ZN3RBX4Math10vector3AbsERKN3G3D7Vector3E
// type: _DWORD *__fastcall(_DWORD *this, const Vector3 *)
#[doc(alias = "RBX::Math::vector3Abs(G3D::Vector3 const&)")]
// IDA 0x358430: 11 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x358430() {
}

// 0x358aa4 — __ZN3RBX4Math15toYAxisQuadrantERKN3G3D15CoordinateFrameE
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::CoordinateFrame *)
#[doc(alias = "RBX::Math::toYAxisQuadrant(G3D::CoordinateFrame const&)")]
// IDA 0x358aa4: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x358aa4() {
}

// 0x358ae4 — __ZN3RBX4Math25intersectRayConvexPolygonERKNS_6RbxRayERKSt6vectorIN3G3D7Vector3ESaIS6_EERS6_b
// type: int __fastcall(int, int, G3D::Plane *)
#[doc(alias = "RBX::Math::intersectRayConvexPolygon(RBX::RbxRay const&,std::vector<G3D::Vector3,std::allocator<G3D::Vector3>> const&,G3D::Vector3&,bool)")]
// IDA 0x358ae4: 177 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x358ae4() {
}

// 0x358d38 — __ZN3RBX4Math17intersectRayPlaneERKNS_6RbxRayERKN3G3D5PlaneERNS4_7Vector3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, const RBX::RbxRay *, const G3D::Plane *, G3D::Vector3 *)
#[doc(alias = "RBX::Math::intersectRayPlane(RBX::RbxRay const&,G3D::Plane const&,G3D::Vector3 &)")]
// IDA 0x358d38: 117 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x358d38() {
}

// 0x358ea0 — __ZN3RBX4Math26spatialPolygonIntersectionERKSt6vectorIN3G3D7Vector3ESaIS3_EES7_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, void *, int, int, void *, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Math::spatialPolygonIntersection(std::vector<G3D::Vector3,std::allocator<G3D::Vector3>> const&,std::vector<G3D::Vector3,std::allocator<G3D::Vector3>> const&)")]
// IDA 0x358ea0: 493 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x358ea0() {
}

// 0x3594fc — __ZN3RBX4Math25planarPolygonIntersectionERKSt6vectorIN3G3D7Vector2ESaIS3_EES7_
// type: void __fastcall(_DWORD *, __int64 *, _DWORD *)
#[doc(alias = "RBX::Math::planarPolygonIntersection(std::vector<G3D::Vector2,std::allocator<G3D::Vector2>> const&,std::vector<G3D::Vector2,std::allocator<G3D::Vector2>> const&)")]
// IDA 0x3594fc: 181 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3594fc() {
}

// 0x3596f8 — __ZN3RBX4Math18intersectLinePlaneERKN3G3D4LineERKNS1_5PlaneERNS1_7Vector3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Line *, const G3D::Plane *, G3D::Vector3 *)
#[doc(alias = "RBX::Math::intersectLinePlane(G3D::Line const&,G3D::Plane const&,G3D::Vector3 &)")]
// IDA 0x3596f8: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3596f8() {
}

// 0x3599b8 — __ZN3RBX4Math29lineSegmentDistanceIfCrossingERKN3G3D7Vector3ES4_S4_S4_Rff
// type: _DWORD __fastcall(RBX::Math *__hidden this, const Vector3 *, const Vector3 *, const Vector3 *, const Vector3 *, float *, float)
#[doc(alias = "RBX::Math::lineSegmentDistanceIfCrossing(G3D::Vector3 const&,G3D::Vector3 const&,G3D::Vector3 const&,G3D::Vector3 const&,float &,float)")]
// IDA 0x3599b8: 146 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3599b8() {
}

// 0x359d50 — __ZN3RBX4Math26getWellFormedRotForZVectorERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Vector3 *)
#[doc(alias = "RBX::Math::getWellFormedRotForZVector(G3D::Vector3 const&)")]
// IDA 0x359d50: 106 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x359d50() {
}

// 0x359f38 — __ZNSt6vectorIN3G3D7Vector2ESaIS1_EE9push_backERKS1_
#[doc(alias = "std::vector<G3D::Vector2,std::allocator<G3D::Vector2>>::push_back(G3D::Vector2 const&)")]
// IDA 0x359f38: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_0x359f38() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x359f64 — __ZNSt6vectorIN3G3D7Vector3ESaIS1_EE9push_backERKS1_
#[doc(alias = "std::vector<G3D::Vector3,std::allocator<G3D::Vector3>>::push_back(G3D::Vector3 const&)")]
// IDA 0x359f64: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_0x359f64() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x359f98 — __ZNK3G3D5Plane17halfSpaceContainsENS_7Vector3E
#[doc(alias = "G3D::Plane::halfSpaceContains(G3D::Vector3)const")]
// IDA 0x359f98: 56 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x359f98() {
}

// 0x35a058 — __ZN3G3D4Line21fromPointAndDirectionERKNS_7Vector3ES3_
// type: _DWORD __fastcall(G3D::Line *__hidden this, const Vector3 *, const Vector3 *)
#[doc(alias = "G3D::Line::fromPointAndDirection(G3D::Vector3 const&,G3D::Vector3 const&)")]
// IDA 0x35a058: 46 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x35a058() {
}

// 0x35a0f4 — __ZN3G3D4LineD1Ev
// type: void __fastcall(G3D::Line *__hidden this)
#[doc(alias = "G3D::Line::~Line()")]
// IDA 0x35a0f4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0x35a0f4() {
}

// 0x35a0f8 — __ZNSt6vectorIN3G3D7Vector3ESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: int __fastcall(unsigned int *, __int64 *, __int64 *)
#[doc(alias = "std::vector<G3D::Vector3,std::allocator<G3D::Vector3>>::_M_insert_aux(__gnu_cxx::__normal_iterator<G3D::Vector3*,std::vector<G3D::Vector3,std::allocator<G3D::Vector3>>>,G3D::Vector3 const&)")]
// IDA 0x35a0f8: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_0x35a0f8() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x35a24c — __ZNSt12_Vector_baseIN3G3D7Vector3ESaIS1_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<G3D::Vector3,std::allocator<G3D::Vector3>>::_M_allocate(unsigned long)")]
// IDA 0x35a24c: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_0x35a24c() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x35a270 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D7Vector3ES5_EET0_T_S7_S6_
#[doc(alias = "G3D::Vector3 * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Vector3 *,G3D::Vector3 *>(G3D::Vector3 *,G3D::Vector3 *,G3D::Vector3 *)")]
// IDA 0x35a270: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_0x35a270() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x35a2d8 — __ZNSt6vectorIN3G3D7Vector2ESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
#[doc(alias = "std::vector<G3D::Vector2,std::allocator<G3D::Vector2>>::_M_insert_aux(__gnu_cxx::__normal_iterator<G3D::Vector2*,std::vector<G3D::Vector2,std::allocator<G3D::Vector2>>>,G3D::Vector2 const&)")]
// IDA 0x35a2d8: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_0x35a2d8() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x35a3e4 — __ZNSt12_Vector_baseIN3G3D7Vector2ESaIS1_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<G3D::Vector2,std::allocator<G3D::Vector2>>::_M_allocate(unsigned long)")]
// IDA 0x35a3e4: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_0x35a3e4() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x35a3fc — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D7Vector2ES5_EET0_T_S7_S6_
#[doc(alias = "G3D::Vector2 * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Vector2 *,G3D::Vector2 *>(G3D::Vector2 *,G3D::Vector2 *,G3D::Vector2 *)")]
// IDA 0x35a3fc: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_0x35a3fc() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x35a448 — __ZN3G3D4LineD0Ev
// type: void __fastcall(G3D::Line *__hidden this)
#[doc(alias = "G3D::Line::~Line()")]
// IDA 0x35a448: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_0x35a448() {
}

// 0x35cfa8 — __ZN3RBX11uvwToObjectERKN3G3D7Vector3ENS_8NormalIdE
#[doc(alias = "RBX::uvwToObject(G3D::Vector3 const&,RBX::NormalId)")]
// IDA 0x35cfa8: 81 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x35cfa8() {
}

// 0x35d0c8 — __ZN3RBX11objectToUvwERKN3G3D7Vector3ENS_8NormalIdE
#[doc(alias = "RBX::objectToUvw(G3D::Vector3 const&,RBX::NormalId)")]
// IDA 0x35d0c8: 81 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x35d0c8() {
}

// 0x35d8a0 — __ZN3RBX17Vector3ToNormalIdERKN3G3D7Vector3E
// type: int __fastcall(RBX *this, const G3D::Vector3 *)
#[doc(alias = "RBX::Vector3ToNormalId(G3D::Vector3 const&)")]
// IDA 0x35d8a0: 192 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x35d8a0() {
}

// 0x35db38 — __ZN3RBX17Matrix3ToNormalIdERKN3G3D7Matrix3E
// type: _DWORD __fastcall(RBX *__hidden this, const G3D::Matrix3 *)
#[doc(alias = "RBX::Matrix3ToNormalId(G3D::Matrix3 const&)")]
// IDA 0x35db38: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x35db38() {
}

// 0x3602bc — __ZN3RBX10QuaternionC1ERKN3G3D7Matrix3E
// type: _DWORD __fastcall(RBX::Quaternion *__hidden this, const G3D::Matrix3 *)
#[doc(alias = "RBX::Quaternion::Quaternion(G3D::Matrix3 const&)")]
// IDA 0x3602bc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_0x3602bc() {
}

// 0x3602c0 — __ZN3RBX10QuaternionC2ERKN3G3D7Matrix3E
#[doc(alias = "RBX::Quaternion::Quaternion(G3D::Matrix3 const&)")]
// IDA 0x3602c0: 114 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3602c0() {
}

// 0x360478 — __ZNK3RBX10Quaternion16toRotationMatrixERN3G3D7Matrix3E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Quaternion::toRotationMatrix(G3D::Matrix3 &)const")]
// IDA 0x360478: 45 insns (PUSH.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x360478() {
}

// 0x373bf4 — __Z7convertRKN3G3D7Vector3ER11FMOD_VECTOR
// type: int __fastcall(RBX::Math **, _DWORD *)
#[doc(alias = "convert(G3D::Vector3 const&,FMOD_VECTOR &)")]
// IDA 0x373bf4: 62 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x373bf4() {
}

// 0x380f1c — __ZN3RBX12SpanningTree22findAndDeactivateEdgesEPNS_12SpanningNodeEPNS_12SpanningEdgeERN3G3D5ArrayIS4_Li10ELm32EEE
// type: RBX::SpanningEdge *__fastcall(RBX::SpanningEdge *result, int, RBX::SpanningEdge *, int)
#[doc(alias = "RBX::SpanningTree::findAndDeactivateEdges(RBX::SpanningNode *,RBX::SpanningEdge *,G3D::Array<RBX::SpanningEdge *,10,32ul> &)")]
// IDA 0x380f1c: 97 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x380f1c() {
}

// 0x38103c — __ZN3RBX12SpanningTree13activateEdgesEPNS_12SpanningNodeERKN3G3D5ArrayIPNS_12SpanningEdgeELi10ELm32EEE
// type: int __fastcall(void (__fastcall ***)(RBX::SpanningTree *, RBX::SpanningEdge *, int), RBX::SpanningNode *, _DWORD *)
#[doc(alias = "RBX::SpanningTree::activateEdges(RBX::SpanningNode *,G3D::Array<RBX::SpanningEdge *,10,32ul> const&)")]
// IDA 0x38103c: 76 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x38103c() {
}

// 0x3812ac — __ZN3G3D5ArrayIPN3RBX12SpanningEdgeELi10ELm32EE6appendERKS3_
// type: int __fastcall(unsigned int *, _DWORD *)
#[doc(alias = "G3D::Array<RBX::SpanningEdge *,10,32ul>::append(RBX::SpanningEdge * const&)")]
// IDA 0x3812ac: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3812ac() {
}

// 0x38147c — __ZN3G3D5ArrayIPN3RBX12SpanningEdgeELi10ELm32EE6resizeEib
// type: int __fastcall(int result, int, int)
#[doc(alias = "G3D::Array<RBX::SpanningEdge *,10,32ul>::resize(int,bool)")]
// IDA 0x38147c: 59 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x38147c() {
}

// 0x381534 — __ZN3G3D5ArrayIPN3RBX12SpanningEdgeELi10ELm32EE7reallocEi
// type: void __fastcall(int, int)
#[doc(alias = "G3D::Array<RBX::SpanningEdge *,10,32ul>::realloc(int)")]
// IDA 0x381534: 147 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x381534() {
}

// 0x38171c — __ZN3G3D5ArrayIPN3RBX12SpanningEdgeELi10ELm32EED2Ev
// type: int __fastcall(int)
#[doc(alias = "G3D::Array<RBX::SpanningEdge *,10,32ul>::~Array()")]
// IDA 0x38171c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x38171c() {
}

// 0x3817f0 — __ZN3G3D5ArrayIPN3RBX12SpanningEdgeELi10ELm32EEC2Ev
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int)
#[doc(alias = "G3D::Array<RBX::SpanningEdge *,10,32ul>::Array(void)")]
// IDA 0x3817f0: 87 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3817f0() {
}

// 0x38c140 — __ZNK3RBX5UDim2mlEN3G3D7Vector2E
// type: _DWORD *__fastcall(_DWORD *result, int, __int32 *)
#[doc(alias = "RBX::UDim2::operator*(G3D::Vector2)const")]
// IDA 0x38c140: 18 insns (LDRSH.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x38c140() {
}

// 0x38c434 — __ZN3RBX5Units20kmsAccelerationToRbxERKN3G3D7Vector3E
// type: _DWORD *__fastcall(_DWORD *this, const Vector3 *)
#[doc(alias = "RBX::Units::kmsAccelerationToRbx(G3D::Vector3 const&)")]
// IDA 0x38c434: 12 insns (VMOV.F32..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x38c434() {
}

// 0x38db20 — __ZN3RBX12Accoutrement18setAttachmentPointERKN3G3D15CoordinateFrameE
// type: int __fastcall(RBX::Accoutrement *this, const G3D::CoordinateFrame *)
#[doc(alias = "RBX::Accoutrement::setAttachmentPoint(G3D::CoordinateFrame const&)")]
// IDA 0x38db20: 83 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x38db20() {
}

// 0x38dc40 — __ZN3RBX12Accoutrement16setAttachmentPosERKN3G3D7Vector3E
// type: int __fastcall(RBX::Accoutrement *this, const G3D::Vector3 *)
#[doc(alias = "RBX::Accoutrement::setAttachmentPos(G3D::Vector3 const&)")]
// IDA 0x38dc40: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x38dc40() {
}

// 0x38dcb0 — __ZN3RBX12Accoutrement20setAttachmentForwardERKN3G3D7Vector3E
// type: int __fastcall(RBX::Accoutrement *this, const G3D::Vector3 *)
#[doc(alias = "RBX::Accoutrement::setAttachmentForward(G3D::Vector3 const&)")]
// IDA 0x38dcb0: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x38dcb0() {
}

// 0x38de0c — __ZN3RBX12Accoutrement15setAttachmentUpERKN3G3D7Vector3E
// type: int __fastcall(RBX::Accoutrement *this, const G3D::Vector3 *)
#[doc(alias = "RBX::Accoutrement::setAttachmentUp(G3D::Vector3 const&)")]
// IDA 0x38de0c: 88 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x38de0c() {
}

// 0x38df40 — __ZN3RBX12Accoutrement18setAttachmentRightERKN3G3D7Vector3E
// type: int __fastcall(RBX::Accoutrement *this, const G3D::Vector3 *)
#[doc(alias = "RBX::Accoutrement::setAttachmentRight(G3D::Vector3 const&)")]
// IDA 0x38df40: 88 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x38df40() {
}

// 0x394464 — __ZNK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEE12getClassNameEv")]
// IDA 0x394464: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x394464() {
}

// 0x394730 — __ZThn32_NK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEE12getClassNameEv")]
// IDA 0x394730: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x394730() {
}

// 0x3949fc — __ZN3RBX4Name13callDoDeclareILZNS_12sPVAdornmentEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sPVAdornmentEEEEvv")]
// IDA 0x3949fc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_0x3949fc() {
}

// 0x394a00 — __ZN3RBX4Name9doDeclareILZNS_12sPVAdornmentEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sPVAdornmentEEEERKS0_v")]
// IDA 0x394a00: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x394a00() {
}

// 0x3a7f68 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE15setListenerModeEb
// type: void __fastcall(int, int)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::setListenerMode(bool)")]
// IDA 0x3a7f68: 124 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3a7f68() {
}

// 0x3a80c8 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE15setListenerModeEb
// type: void __fastcall(int, int)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::setListenerMode(bool)")]
// IDA 0x3a80c8: 124 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3a80c8() {
}

// 0x3a82e8 — __ZN3rbx7signals16signal_with_argsILi3EFvN3G3D7Vector34AxisEffEEclES4_ff
// type: void __fastcall(_DWORD *, int, int, const void *, int, int, int, int, void *, int)
#[doc(alias = "rbx::signals::signal_with_args<3,void ()(G3D::Vector3::Axis,float,float)>::operator()(G3D::Vector3::Axis,float,float)")]
// IDA 0x3a82e8: 81 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3a82e8() {
}

// 0x3a8440 — __ZN3rbx7signals16signal_with_argsILi1EFvN3G3D7Vector34AxisEEEclES4_
// type: void __fastcall(_DWORD *, int, int, const void *, int, int, int, int, void *, int)
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(G3D::Vector3::Axis)>::operator()(G3D::Vector3::Axis)")]
// IDA 0x3a8440: 76 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3a8440() {
}

// 0x3a9380 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE
// type: int __fastcall(int, int *, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot> &)")]
// was: __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE
// IDA 0x3a9380: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3a9380() {
}

// 0x3a94e0 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE8on_errorERSt9exception
// type: int *()
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::on_error(std::exception &)")]
// IDA 0x3a94e0: 14 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3a94e0() {
}

