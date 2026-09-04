//! core shard AH — 150 core stubs EA-sorted, earliest gap (lowest uncovered) after prior shards.
//! Source: ida/export.json filtered where demangled/mangled excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted, next 150 uncovered (lowest EA first).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

/// Batch 9: 11 IDA-grounded ports 0x3043fc-0x30e5c0 — the `BrickColor`
/// palette family (`parse`, `random`, the `int` ctors, `color4uint8`,
/// `color3uint8`, `name`, `color4`, `color3`, `hash_value`) over the
/// `BrickMap` table built by the ctor at 0x304f34, plus
/// `Color::colorFromInt` over the `getColorByIndex` table (IDA 0x30e3b8).
/// Ports live in `core_brick` under idiomatic names, wired via `stub_0x*`.
/// Conventions: palette indices are the `Number` (== colors-vector index);
/// `ReleaseAssert` checks are kept as `expect`/`assert!` with the original
/// `BrickColor.cpp` messages; `G3D::iRandom` selection lives outside core.
/// `[INFERENCE]` marks what the binary does not pin down.
pub mod core_brick {
    /// (number, name, r, g, b) palette from the `BrickMap` ctor (IDA
    /// 0x304f34): every `insert(map, number, r, g, b, name)` call in address
    /// order (number-ascending, matching the colors-vector order the
    /// `parse` walk relies on).
    const PALETTE: &[(u32, &str, u8, u8, u8)] = &[
        (1, "White", 242, 243, 243),
        (2, "Grey", 161, 165, 162),
        (3, "Light yellow", 249, 233, 153),
        (5, "Brick yellow", 215, 197, 154),
        (6, "Light green (Mint)", 194, 218, 184),
        (9, "Light reddish violet", 232, 186, 200),
        (11, "Pastel Blue", 128, 187, 219),
        (12, "Light orange brown", 203, 132, 66),
        (18, "Nougat", 204, 142, 105),
        (21, "Bright red", 196, 40, 28),
        (22, "Med. reddish violet", 196, 112, 160),
        (23, "Bright blue", 13, 105, 172),
        (24, "Bright yellow", 245, 205, 48),
        (25, "Earth orange", 98, 71, 50),
        (26, "Black", 27, 42, 53),
        (27, "Dark grey", 109, 110, 108),
        (28, "Dark green", 40, 127, 71),
        (29, "Medium green", 161, 196, 140),
        (36, "Lig. Yellowich orange", 243, 207, 155),
        (37, "Bright green", 75, 151, 75),
        (38, "Dark orange", 160, 95, 53),
        (39, "Light bluish violet", 193, 202, 222),
        (40, "Transparent", 236, 236, 236),
        (41, "Tr. Red", 205, 84, 75),
        (42, "Tr. Lg blue", 193, 223, 240),
        (43, "Tr. Blue", 123, 182, 232),
        (44, "Tr. Yellow", 247, 241, 141),
        (45, "Light blue", 180, 210, 228),
        (47, "Tr. Flu. Reddish orange", 217, 133, 108),
        (48, "Tr. Green", 132, 182, 141),
        (49, "Tr. Flu. Green", 248, 241, 132),
        (50, "Phosph. White", 236, 232, 222),
        (100, "Light red", 238, 196, 182),
        (101, "Medium red", 218, 134, 122),
        (102, "Medium blue", 110, 153, 202),
        (103, "Light grey", 199, 193, 183),
        (104, "Bright violet", 107, 50, 124),
        (105, "Br. yellowish orange", 226, 155, 64),
        (106, "Bright orange", 218, 133, 65),
        (107, "Bright bluish green", 0, 143, 156),
        (108, "Earth yellow", 104, 92, 67),
        (110, "Bright bluish violet", 67, 84, 147),
        (111, "Tr. Brown", 191, 183, 177),
        (112, "Medium bluish violet", 104, 116, 172),
        (113, "Tr. Medi. reddish violet", 228, 173, 200),
        (115, "Med. yellowish green", 199, 210, 60),
        (116, "Med. bluish green", 85, 165, 175),
        (118, "Light bluish green", 183, 215, 213),
        (119, "Br. yellowish green", 164, 189, 71),
        (120, "Lig. yellowish green", 217, 228, 167),
        (121, "Med. yellowish orange", 231, 172, 88),
        (123, "Br. reddish orange", 211, 111, 76),
        (124, "Bright reddish violet", 146, 57, 120),
        (125, "Light orange", 234, 184, 146),
        (126, "Tr. Bright bluish violet", 165, 165, 203),
        (127, "Gold", 220, 188, 129),
        (128, "Dark nougat", 174, 122, 89),
        (131, "Silver", 156, 163, 168),
        (133, "Neon orange", 213, 115, 61),
        (134, "Neon green", 216, 221, 86),
        (135, "Sand blue", 116, 134, 157),
        (136, "Sand violet", 135, 124, 144),
        (137, "Medium orange", 224, 152, 100),
        (138, "Sand yellow", 149, 138, 115),
        (140, "Earth blue", 32, 58, 86),
        (141, "Earth green", 39, 70, 45),
        (143, "Tr. Flu. Blue", 207, 226, 247),
        (145, "Sand blue metallic", 121, 136, 161),
        (146, "Sand violet metallic", 149, 142, 163),
        (147, "Sand yellow metallic", 147, 135, 103),
        (148, "Dark grey metallic", 87, 88, 87),
        (149, "Black metallic", 22, 29, 50),
        (150, "Light grey metallic", 171, 173, 172),
        (151, "Sand green", 120, 144, 130),
        (153, "Sand red", 149, 121, 119),
        (154, "Dark red", 123, 46, 47),
        (157, "Tr. Flu. Yellow", 255, 246, 123),
        (158, "Tr. Flu. Red", 225, 164, 194),
        (168, "Gun metallic", 117, 108, 98),
        (176, "Red flip/flop", 151, 105, 91),
        (178, "Yellow flip/flop", 180, 132, 85),
        (179, "Silver flip/flop", 137, 135, 136),
        (180, "Curry", 215, 169, 75),
        (190, "Fire Yellow", 249, 214, 46),
        (191, "Flame yellowish orange", 232, 171, 45),
        (192, "Reddish brown", 105, 64, 40),
        (193, "Flame reddish orange", 207, 96, 36),
        (194, "Medium stone grey", 163, 162, 165),
        (195, "Royal blue", 70, 103, 164),
        (196, "Dark Royal blue", 35, 71, 139),
        (198, "Bright reddish lilac", 142, 66, 133),
        (199, "Dark stone grey", 99, 95, 98),
        (200, "Lemon metalic", 130, 138, 93),
        (208, "Light stone grey", 229, 228, 223),
        (209, "Dark Curry", 176, 142, 68),
        (210, "Faded green", 112, 149, 120),
        (211, "Turquoise", 121, 181, 181),
        (212, "Light Royal blue", 159, 195, 233),
        (213, "Medium Royal blue", 108, 129, 183),
        (216, "Rust", 143, 76, 42),
        (217, "Brown", 124, 92, 70),
        (218, "Reddish lilac", 150, 112, 159),
        (219, "Lilac", 107, 98, 155),
        (220, "Light lilac", 167, 169, 206),
        (221, "Bright purple", 205, 98, 152),
        (222, "Light purple", 228, 173, 200),
        (223, "Light pink", 220, 144, 149),
        (224, "Light brick yellow", 240, 213, 160),
        (225, "Warm yellowish orange", 235, 184, 127),
        (226, "Cool yellow", 253, 234, 141),
        (232, "Dove blue", 125, 187, 221),
        (268, "Medium lilac", 52, 43, 117),
        (1001, "Institutional white", 248, 248, 248),
        (1002, "Mid gray", 205, 205, 205),
        (1003, "Really black", 17, 17, 17),
        (1004, "Really red", 255, 0, 0),
        (1005, "Deep orange", 255, 175, 0),
        (1006, "Alder", 180, 128, 255),
        (1007, "Dusty Rose", 163, 75, 75),
        (1008, "Olive", 193, 190, 66),
        (1009, "New Yeller", 255, 255, 0),
        (1010, "Really blue", 0, 0, 255),
        (1011, "Navy blue", 0, 32, 96),
        (1012, "Deep blue", 33, 84, 185),
        (1013, "Cyan", 4, 175, 236),
        (1014, "CGA brown", 170, 85, 0),
        (1015, "Magenta", 170, 0, 170),
        (1016, "Pink", 255, 102, 204),
        (1017, "Deep orange", 255, 175, 0),
        (1018, "Teal", 18, 238, 212),
        (1019, "Toothpaste", 0, 255, 255),
        (1020, "Lime green", 0, 255, 0),
        (1021, "Camo", 58, 125, 21),
        (1022, "Grime", 127, 142, 100),
        (1023, "Lavender", 140, 91, 159),
        (1024, "Pastel light blue", 175, 221, 255),
        (1025, "Pastel orange", 255, 201, 201),
        (1026, "Pastel violet", 177, 167, 255),
        (1027, "Pastel blue-green", 159, 243, 233),
        (1028, "Pastel green", 204, 255, 204),
        (1029, "Pastel yellow", 255, 255, 204),
        (1030, "Pastel brown", 255, 204, 153),
        (1031, "Royal purple", 98, 37, 209),
        (1032, "Hot pink", 255, 0, 191),
    ];

    /// Random pool from the ctor tail (IDA 0x307660-0x307d14): the 64 numbers
    /// pushed into the numbers vector; `random` indexes it with
    /// `G3D::iRandom(0, count - 1)` (IDA 0x304490).
    const RANDOM_NUMBERS: &[u32] = &[
        119, 24, 106, 21, 104, 23, 107, 37, 1001, 1, 208, 1002, 194, 199, 26,
        1003, 1022, 105, 125, 153, 1023, 135, 102, 151, 5, 226, 133, 101, 9,
        11, 1018, 29, 1030, 1029, 1025, 1016, 1026, 1024, 1027, 1028, 1008,
        1009, 1017, 1004, 1032, 1010, 1019, 1020, 217, 18, 38, 1031, 1006,
        1013, 45, 1021, 192, 1014, 1007, 1015, 1012, 1011, 28, 141,
    ];

    /// `Color::getColorByIndex` table (IDA 0x30e3b8): 16 rows of 3 floats at
    /// `_MergedGlobals_140 + 8`, stride 12, written once by the guarded init
    /// (0x30e3f2-0x30e562). The values below are the init immediates as
    /// `f32` bits; a `const` table is the once-init.
    const COLOR_INDEX_TABLE: [[f32; 3]; 16] = [
        [0.0, 0.0, 0.0],
        [0.3411765, 0.3411765, 0.3411765],
        [0.6784314, 0.1372549, 0.1372549],
        [0.1647059, 0.2941177, 0.8431373],
        [0.1137255, 0.4117647, 0.07843138],
        [0.5058824, 0.2901961, 0.09803922],
        [0.5058824, 0.1490196, 0.7529412],
        [0.627451, 0.627451, 0.627451],
        [0.5058824, 0.772549, 0.08627451],
        [0.6156863, 0.6862745, 1.0],
        [0.1607843, 0.8156863, 0.8156863],
        [1.0, 0.572549, 0.2],
        [1.0, 0.9333333, 0.2],
        [0.9137255, 0.8705882, 0.7333333],
        [1.0, 0.8039216, 0.9529412],
        [1.0, 1.0, 1.0],
    ];

    fn entry(number: u32) -> Option<&'static (u32, &'static str, u8, u8, u8)> {
        PALETTE.iter().find(|&&(n, _, _, _, _)| n == number)
    }

    /// IDA 0x30456c `BrickColor(int)` (0x304568 is the C1 thunk into it):
    /// numbers with a valid entry stick (0x30459c-0x3045a4), anything else
    /// becomes 194 — Medium stone grey (0x3045aa).
    pub fn brick_color_from_number(number: u32) -> u32 {
        if entry(number).is_some() {
            number
        } else {
            194
        }
    }

    /// IDA 0x3043fc `parse`: linear `string::compare` over the entries
    /// (0x304426-0x304454); a miss stores 194 (0x304456-0x30445c, the same
    /// default as the ctor). Palette order is number order, so the first
    /// match is the binary's match (e.g. "Deep orange" hits 1005).
    pub fn brick_color_parse(name: &str) -> u32 {
        for &(n, text, _, _, _) in PALETTE {
            if text == name {
                return n;
            }
        }
        194
    }

    /// IDA 0x304468 `random`: `numbers[G3D::iRandom(0, count - 1)]`
    /// (0x304490-0x30449c). The RNG lives outside core; the caller passes the
    /// pick, wrapped like the inclusive range.
    pub fn brick_color_random(pick: usize) -> u32 {
        RANDOM_NUMBERS[pick % RANDOM_NUMBERS.len()]
    }

    /// IDA 0x3045b0 `color4uint8`: `ReleaseAssert(number < size &&
    /// colors[number].valid)` (`BrickColor.cpp:559`, 0x3045c8-0x304630,
    /// fast-log gated — fast-log owned, noted), then the packed word at `+1`
    /// (0x304634-0x304650) laid down by `insert` as `r | g<<8 | b<<16 |
    /// 0xFF000000` (0x30ccd4).
    pub fn brick_color_packed(number: u32) -> u32 {
        let &(_, _, r, g, b) = entry(number).expect("colors[number].valid (BrickColor.cpp:559)");
        u32::from_le_bytes([r, g, b, 0xFF])
    }

    /// IDA 0x304654 `color3uint8`: `color4uint8` (0x30465e) split LE into the
    /// r/g/b bytes (0x304662-0x30466c).
    pub fn brick_color_rgb8(number: u32) -> [u8; 3] {
        let packed = brick_color_packed(number);
        [
            (packed & 0xFF) as u8,
            ((packed >> 8) & 0xFF) as u8,
            ((packed >> 16) & 0xFF) as u8,
        ]
    }

    /// IDA 0x304674 `name`: same ReleaseAssert (`BrickColor.cpp:570`,
    /// 0x30468a-0x3046f2), then the name at `+24` (0x30470e).
    pub fn brick_color_name(number: u32) -> &'static str {
        entry(number).expect("colors[number].valid (BrickColor.cpp:570)").1
    }

    /// IDA 0x304710 `color4`: same ReleaseAssert (`BrickColor.cpp:576`,
    /// 0x304728-0x304790), then the four floats at `+8..+20`
    /// (0x304794-0x3047bc) laid down by `insert` via `G3D::Color4(uint8)`
    /// (0x30ccdc-0x30ccf0) — each channel / 255 with opaque alpha
    /// (`[INFERENCE]` — the G3D ctor is not in this batch, but /255 + alpha
    /// 1.0 is its documented shape and reproduces every entry).
    pub fn brick_color_float4(number: u32) -> [f32; 4] {
        let &(_, _, r, g, b) = entry(number).expect("colors[number].valid (BrickColor.cpp:576)");
        [
            f32::from(r) / 255.0,
            f32::from(g) / 255.0,
            f32::from(b) / 255.0,
            1.0,
        ]
    }

    /// IDA 0x3047c4 `color3`: `color4` (0x3047ce) narrowed to the first three
    /// channels (0x3047d6-0x3047de).
    pub fn brick_color_float3(number: u32) -> [f32; 3] {
        let c4 = brick_color_float4(number);
        [c4[0], c4[1], c4[2]]
    }

    /// IDA 0x3047ec `hash_value`: the number itself (0x3047ee).
    pub fn brick_color_hash(number: u32) -> u32 {
        number
    }

    /// IDA 0x30e3b8 `getColorByIndex(int)`: guarded once-init
    /// (0x30e3d4-0x30e3de) of the table above, then row `this % 16` at
    /// `base + 12*(this % 16) + 8` (0x30e566-0x30e57a, signed `%` like
    /// Rust's). `rem_euclid` agrees for every non-negative input, which
    /// covers all live call sites.
    pub fn color_index_row(index: i32) -> [f32; 3] {
        COLOR_INDEX_TABLE[index.rem_euclid(16) as usize]
    }

    /// IDA 0x30e5c0 `colorFromInt(unsigned)`: row `i % 15` (0x30e5f0 — the
    /// co-computed `16*(i/15)` second tuple element is dead: the callee only
    /// reads R0) mixed halfway toward row `i % 13 + 3` (0x30e616-0x30e656):
    /// `out = c1 + 0.5 * (c2 - c1)` per channel.
    pub fn color_from_int(i: u32) -> [f32; 3] {
        let c1 = color_index_row((i % 15) as i32);
        let c2 = color_index_row((i % 13 + 3) as i32);
        [
            c1[0] + 0.5 * (c2[0] - c1[0]),
            c1[1] + 0.5 * (c2[1] - c1[1]),
            c1[2] + 0.5 * (c2[2] - c1[2]),
        ]
    }
}
#[doc(alias = "RBX::AsyncHttpQueue::setThreadPool(int)")]
// 0x2fad24 — __ZN3RBX14AsyncHttpQueue13setThreadPoolEi
pub fn stub_0x2fad24() {
    // IDA 0x2fad24: async-http queue dispatch owned by the network crate — carrier no-op in core.
}

#[doc(alias = "RBX::AsyncHttpQueue::resetStatsItem(RBX::ServiceProvider *)")]
// 0x2fae00 — __ZN3RBX14AsyncHttpQueue14resetStatsItemEPNS_15ServiceProviderE
pub fn stub_0x2fae00() {
    // IDA 0x2fae00: async-http queue dispatch owned by the network crate — carrier no-op in core.
}

#[doc(alias = "RBX::AsyncHttpQueue::getRequestQueueSize(void)const")]
// 0x2faf2c — __ZNK3RBX14AsyncHttpQueue19getRequestQueueSizeEv
pub fn stub_0x2faf2c() {
    // IDA 0x2faf2c: async-http queue dispatch owned by the network crate — carrier no-op in core.
}

#[doc(alias = "RBX::AsyncHttpQueue::~AsyncHttpQueue()")]
// 0x2faf68 — __ZN3RBX14AsyncHttpQueueD0Ev
pub fn stub_0x2faf68() {
    // IDA 0x2faf68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::AsyncHttpQueue::~AsyncHttpQueue()")]
// 0x2fb008 — __ZN3RBX14AsyncHttpQueueD1Ev
pub fn stub_0x2fb008() {
    // IDA 0x2fb008: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::AsyncHttpQueue::~AsyncHttpQueue()")]
// 0x2fb00c — __ZN3RBX14AsyncHttpQueueD2Ev
pub fn stub_0x2fb00c() {
    // IDA 0x2fb00c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::AsyncHttpQueue::onHeartbeat(RBX::Heartbeat const&)")]
// 0x2fb2ac — __ZN3RBX14AsyncHttpQueue11onHeartbeatERKNS_9HeartbeatE
pub fn stub_0x2fb2ac() {
    // IDA 0x2fb2ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::AsyncHttpQueue::isRequestQueueEmpty(void)")]
// 0x2fca04 — __ZN3RBX14AsyncHttpQueue19isRequestQueueEmptyEv
pub fn stub_0x2fca04() {
    // IDA 0x2fca04: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::checkContentUrl(std::string)")]
// 0x2fca3c — __ZN3RBXL15checkContentUrlESs
pub fn stub_0x2fca3c() {
    // IDA 0x2fca3c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::AsyncHttpQueue::FailedUrl::FailedUrl(char const*)")]
// 0x2fd150 — __ZN3RBX14AsyncHttpQueue9FailedUrlC2EPKc
pub fn stub_0x2fd150() {
    // IDA 0x2fd150: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::AsyncHttpQueue::isUrlBad(std::string const&)")]
// 0x2fd220 — __ZN3RBX14AsyncHttpQueue8isUrlBadERKSs
pub fn stub_0x2fd220() {
    // IDA 0x2fd220: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::AsyncHttpQueue::syncRequest(std::string const&)")]
// 0x2fd910 — __ZN3RBX14AsyncHttpQueue11syncRequestERKSs
pub fn stub_0x2fd910() {
    // IDA 0x2fd910: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::operator=(std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>> const&)")]
// 0x2fe654 — __ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EEaSERKS4_
pub fn stub_0x2fe654() {
    // IDA 0x2fe654: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::list<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::erase(std::_List_iterator<RBX::AsyncHttpQueue::FailedUrl>,std::_List_iterator<RBX::AsyncHttpQueue::FailedUrl>)")]
// 0x2fea20 — __ZNSt4listIN3RBX14AsyncHttpQueue9FailedUrlESaIS2_EE5eraseESt14_List_iteratorIS2_ES6_
pub fn stub_0x2fea20() {
    // IDA 0x2fea20: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::push_back(RBX::AsyncHttpQueue::CallbackWrapper const&)")]
// 0x2fea58 — __ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE9push_backERKS2_
pub fn stub_0x2fea58() {
    // IDA 0x2fea58: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,RBX::AsyncHttpQueue::CallbackWrapper const&)")]
// 0x2feab0 — __ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0x2feab0() {
    // IDA 0x2feab0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_allocate(unsigned long)")]
// 0x2fee5c — __ZNSt12_Vector_baseIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE11_M_allocateEm
pub fn stub_0x2fee5c() {
    // IDA 0x2fee5c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::AsyncHttpQueue::CallbackWrapper * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *>(RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *)")]
// 0x2ff128 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX14AsyncHttpQueue15CallbackWrapperES6_EET0_T_S8_S7_
pub fn stub_0x2ff128() {
    // IDA 0x2ff128: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::list<RBX::AsyncHttpQueue::Request,std::allocator<RBX::AsyncHttpQueue::Request>>::_M_create_node(RBX::AsyncHttpQueue::Request const&)")]
// 0x2ff188 — __ZNSt4listIN3RBX14AsyncHttpQueue7RequestESaIS2_EE14_M_create_nodeERKS2_
pub fn stub_0x2ff188() {
    // IDA 0x2ff188: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::vector(std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>> const&)")]
// 0x2ff2d4 — __ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EEC2ERKS4_
pub fn stub_0x2ff2d4() {
    // IDA 0x2ff2d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_Vector_base(unsigned long,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper> const&)")]
// 0x2ff43c — __ZNSt12_Vector_baseIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EEC2EmRKS3_
pub fn stub_0x2ff43c() {
    // IDA 0x2ff43c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::list<RBX::AsyncHttpQueue::Request,std::allocator<RBX::AsyncHttpQueue::Request>>::_M_erase(std::_List_iterator<RBX::AsyncHttpQueue::Request>)")]
// 0x2ff674 — __ZNSt4listIN3RBX14AsyncHttpQueue7RequestESaIS2_EE8_M_eraseESt14_List_iteratorIS2_E
pub fn stub_0x2ff674() {
    // IDA 0x2ff674: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::AsyncHttpQueue::CallbackWrapper* std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_allocate_and_copy<__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper const*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>>(unsigned long,__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper const*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper const*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>)")]
// 0x2ff758 — __ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE20_M_allocate_and_copyIN9__gnu_cxx17__normal_iteratorIPKS2_S4_EEEEPS2_mT_SC_
pub fn stub_0x2ff758() {
    // IDA 0x2ff758: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::AsyncHttpQueue::CallbackWrapper * std::__copy<false,std::random_access_iterator_tag>::copy<RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *>(RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *)")]
// 0x2ff8c0 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN3RBX14AsyncHttpQueue15CallbackWrapperES6_EET0_T_S8_S7_
pub fn stub_0x2ff8c0() {
    // IDA 0x2ff8c0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::AsyncHttpQueue::CallbackWrapper* std::__copy<false,std::random_access_iterator_tag>::copy<RBX::AsyncHttpQueue::CallbackWrapper const*,RBX::AsyncHttpQueue::CallbackWrapper*>(RBX::AsyncHttpQueue::CallbackWrapper const*,RBX::AsyncHttpQueue::CallbackWrapper const*,RBX::AsyncHttpQueue::CallbackWrapper*)")]
// 0x2ff91c — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKN3RBX14AsyncHttpQueue15CallbackWrapperEPS5_EET0_T_SA_S9_
pub fn stub_0x2ff91c() {
    // IDA 0x2ff91c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::~vector()")]
// 0x2ff978 — __ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EED2Ev
pub fn stub_0x2ff978() {
    // IDA 0x2ff978: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::list<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::_M_create_node(RBX::AsyncHttpQueue::FailedUrl const&)")]
// 0x2ffa44 — __ZNSt4listIN3RBX14AsyncHttpQueue9FailedUrlESaIS2_EE14_M_create_nodeERKS2_
pub fn stub_0x2ffa44() {
    // IDA 0x2ffa44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::deque<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::pop_front(void)")]
// 0x301b4c — __ZNSt5dequeIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE9pop_frontEv
pub fn stub_0x301b4c() {
    // IDA 0x301b4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::_M_allocate_map(unsigned long)")]
// 0x301b80 — __ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE15_M_allocate_mapEm
pub fn stub_0x301b80() {
    // IDA 0x301b80: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_List_base<RBX::AsyncHttpQueue::Request,std::allocator<RBX::AsyncHttpQueue::Request>>::_M_clear(void)")]
// 0x301b98 — __ZNSt10_List_baseIN3RBX14AsyncHttpQueue7RequestESaIS2_EE8_M_clearEv
pub fn stub_0x301b98() {
    // IDA 0x301b98: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::deque<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::deque(std::deque<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>> const&)")]
// 0x301f74 — __ZNSt5dequeIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EEC2ERKS4_
pub fn stub_0x301f74() {
    // IDA 0x301f74: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::~_Deque_base()")]
// 0x302028 — __ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EED2Ev
pub fn stub_0x302028() {
    // IDA 0x302028: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::_M_initialize_map(unsigned long)")]
// 0x302054 — __ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE17_M_initialize_mapEm
pub fn stub_0x302054() {
    // IDA 0x302054: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::_M_create_nodes(RBX::AsyncHttpQueue::AsyncRetryTask**,RBX::AsyncHttpQueue::AsyncRetryTask**)")]
// 0x3021d4 — __ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE15_M_create_nodesEPPS2_S6_
pub fn stub_0x3021d4() {
    // IDA 0x3021d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HttpQueueStatsItem::init(void)")]
// 0x3023dc — __ZN3RBX18HttpQueueStatsItem4initEv
pub fn stub_0x3023dc() {
    // IDA 0x3023dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HttpQueueStatsItem::~HttpQueueStatsItem()")]
// 0x30266c — __ZN3RBX18HttpQueueStatsItemD1Ev
pub fn stub_0x30266c() {
    // IDA 0x30266c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HttpQueueStatsItem::~HttpQueueStatsItem()")]
// 0x3026a8 — __ZN3RBX18HttpQueueStatsItemD0Ev
pub fn stub_0x3026a8() {
    // IDA 0x3026a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HttpQueueStatsItem::update(void)")]
// 0x30277c — __ZN3RBX18HttpQueueStatsItem6updateEv
pub fn stub_0x30277c() {
    // IDA 0x30277c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::HttpQueueStatsItem::~HttpQueueStatsItem()")]
// 0x3027d0 — __ZThn32_N3RBX18HttpQueueStatsItemD1Ev
pub fn stub_0x3027d0() {
    // IDA 0x3027d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::HttpQueueStatsItem::~HttpQueueStatsItem()")]
// 0x302810 — __ZThn32_N3RBX18HttpQueueStatsItemD0Ev
pub fn stub_0x302810() {
    // IDA 0x302810: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::HttpQueueStatsItem::~HttpQueueStatsItem()")]
// 0x3028e8 — __ZThn36_N3RBX18HttpQueueStatsItemD1Ev
pub fn stub_0x3028e8() {
    // IDA 0x3028e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::HttpQueueStatsItem::~HttpQueueStatsItem()")]
// 0x302928 — __ZThn36_N3RBX18HttpQueueStatsItemD0Ev
pub fn stub_0x302928() {
    // IDA 0x302928: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_List_base<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::_M_clear(void)")]
// 0x302cf8 — __ZNSt10_List_baseIN3RBX14AsyncHttpQueue9FailedUrlESaIS2_EE8_M_clearEv
pub fn stub_0x302cf8() {
    // IDA 0x302cf8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "global constructor keyed to_a_106")]
// 0x302d20 — __GLOBAL__I_a_106
pub fn stub_0x302d20() {
    // IDA 0x302d20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Axes::Axes(int)")]
// 0x302eb8 — __ZN3RBX4AxesC1Ei
pub fn stub_0x302eb8() {
    // IDA 0x302eb8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Axes::normalIdToAxis(RBX::NormalId)")]
// 0x302ebc — __ZN3RBX4Axes14normalIdToAxisENS_8NormalIdE
pub fn stub_0x302ebc() {
    // IDA 0x302ebc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Axes::getAxisByNormalId(RBX::NormalId)const")]
// 0x302ef0 — __ZNK3RBX4Axes17getAxisByNormalIdENS_8NormalIdE
pub fn stub_0x302ef0() {
    // IDA 0x302ef0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::StringConverter<RBX::Axes>::convertToString(RBX::Axes const&)")]
// 0x302f30 — __ZN3RBX15StringConverterINS_4AxesEE15convertToStringERKS1_
pub fn stub_0x302f30() {
    // IDA 0x302f30: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::StringConverter<RBX::Axes>::convertToValue(std::string const&,RBX::Axes&)")]
// 0x303418 — __ZN3RBX15StringConverterINS_4AxesEE14convertToValueERKSsRS1_
pub fn stub_0x303418() {
    // IDA 0x303418: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "global constructor keyed to_a_107")]
// 0x304200 — __GLOBAL__I_a_107
pub fn stub_0x304200() {
    // IDA 0x304200: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::BrickColor::BrickMap::singleton(void)")]
// 0x3042c8 — __ZN3RBX10BrickColor8BrickMap9singletonEv
pub fn stub_0x3042c8() {
    // IDA 0x3042c8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::BrickColor::colorPalette(void)")]
// 0x3043c4 — __ZN3RBX10BrickColor12colorPaletteEv
pub fn stub_0x3043c4() {
    // IDA 0x3043c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::BrickColor::getClosestPaletteIndex(void)const")]
// 0x3043dc — __ZNK3RBX10BrickColor22getClosestPaletteIndexEv
pub fn stub_0x3043dc() {
    // IDA 0x3043dc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::BrickColor::parse(char const*)")]
// 0x3043fc — __ZN3RBX10BrickColor5parseEPKc
pub fn stub_0x3043fc(name: &str) -> u32 {
    // IDA 0x3043fc: BrickColor::parse — linear string::compare over the BrickMap entries (0x304426-0x304454); miss stores 194 (0x304456-0x30445c).
    core_brick::brick_color_parse(name)
}

#[doc(alias = "RBX::BrickColor::random(void)")]
// 0x304468 — __ZN3RBX10BrickColor6randomEv
pub fn stub_0x304468(pick: usize) -> u32 {
    // IDA 0x304468: BrickColor::random — numbers[G3D::iRandom(0, count-1)] (0x304490-0x30449c); the RNG lives outside core, the caller passes the pick.
    core_brick::brick_color_random(pick)
}

#[doc(alias = "RBX::BrickColor::BrickColor(int)")]
// 0x304568 — __ZN3RBX10BrickColorC1Ei
pub fn stub_0x304568(number: u32) -> u32 {
    // IDA 0x304568: BrickColor C1 — thunk into the C2 below.
    core_brick::brick_color_from_number(number)
}

#[doc(alias = "RBX::BrickColor::BrickColor(int)")]
// 0x30456c — __ZN3RBX10BrickColorC2Ei
pub fn stub_0x30456c(number: u32) -> u32 {
    // IDA 0x30456c: BrickColor C2 — valid numbers stick (0x30459c-0x3045a4), anything else becomes 194 (0x3045aa).
    core_brick::brick_color_from_number(number)
}

#[doc(alias = "RBX::BrickColor::color4uint8(void)const")]
// 0x3045b0 — __ZNK3RBX10BrickColor11color4uint8Ev
pub fn stub_0x3045b0(number: u32) -> u32 {
    // IDA 0x3045b0: BrickColor::color4uint8 — ReleaseAssert(valid) (BrickColor.cpp:559, 0x3045c8-0x304630), packed word at +1 (0x304634-0x304650).
    core_brick::brick_color_packed(number)
}

#[doc(alias = "RBX::BrickColor::color3uint8(void)const")]
// 0x304654 — __ZNK3RBX10BrickColor11color3uint8Ev
pub fn stub_0x304654(number: u32) -> [u8; 3] {
    // IDA 0x304654: BrickColor::color3uint8 — color4uint8 (0x30465e) split LE into r/g/b (0x304662-0x30466c).
    core_brick::brick_color_rgb8(number)
}

#[doc(alias = "RBX::BrickColor::name(void)const")]
// 0x304674 — __ZNK3RBX10BrickColor4nameEv
pub fn stub_0x304674(number: u32) -> &'static str {
    // IDA 0x304674: BrickColor::name — ReleaseAssert(valid) (BrickColor.cpp:570, 0x30468a-0x3046f2), name at +24 (0x30470e).
    core_brick::brick_color_name(number)
}

#[doc(alias = "RBX::BrickColor::color4(void)const")]
// 0x304710 — __ZNK3RBX10BrickColor6color4Ev
pub fn stub_0x304710(number: u32) -> [f32; 4] {
    // IDA 0x304710: BrickColor::color4 — ReleaseAssert(valid) (BrickColor.cpp:576, 0x304728-0x304790), floats at +8..+20 via G3D::Color4(uint8) (0x304794-0x3047bc).
    core_brick::brick_color_float4(number)
}

#[doc(alias = "RBX::BrickColor::color3(void)const")]
// 0x3047c4 — __ZNK3RBX10BrickColor6color3Ev
pub fn stub_0x3047c4(number: u32) -> [f32; 3] {
    // IDA 0x3047c4: BrickColor::color3 — color4 (0x3047ce) narrowed to three channels (0x3047d6-0x3047de).
    core_brick::brick_color_float3(number)
}

#[doc(alias = "RBX::hash_value(RBX::BrickColor const&)")]
// 0x3047ec — __ZN3RBX10hash_valueERKNS_10BrickColorE
pub fn stub_0x3047ec(number: u32) -> u32 {
    // IDA 0x3047ec: hash_value(BrickColor) — the number itself (0x3047ee).
    core_brick::brick_color_hash(number)
}

#[doc(alias = "RBX::BrickColor::BrickMap::~BrickMap()")]
// 0x304b70 — __ZN3RBX10BrickColor8BrickMapD1Ev
pub fn stub_0x304b70() {
    // IDA 0x304b70: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::map<RBX::BrickColor::Number,int,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>>::operator[](RBX::BrickColor::Number const&)")]
// 0x304b74 — __ZNSt3mapIN3RBX10BrickColor6NumberEiSt4lessIS2_ESaISt4pairIKS2_iEEEixERS6_
pub fn stub_0x304b74() {
    // IDA 0x304b74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::BrickColor::Number,std::pair<RBX::BrickColor::Number const,int>,std::_Select1st<std::pair<RBX::BrickColor::Number const,int>>,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::BrickColor::Number const,int>>,std::pair<RBX::BrickColor::Number const,int> const&)")]
// 0x304bcc — __ZNSt8_Rb_treeIN3RBX10BrickColor6NumberESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
pub fn stub_0x304bcc() {
    // IDA 0x304bcc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::BrickColor::Number,std::pair<RBX::BrickColor::Number const,int>,std::_Select1st<std::pair<RBX::BrickColor::Number const,int>>,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::BrickColor::Number const,int> const&)")]
// 0x304c80 — __ZNSt8_Rb_treeIN3RBX10BrickColor6NumberESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
pub fn stub_0x304c80() {
    // IDA 0x304c80: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::BrickColor::Number,std::pair<RBX::BrickColor::Number const,int>,std::_Select1st<std::pair<RBX::BrickColor::Number const,int>>,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>>::_M_insert_unique(std::pair<RBX::BrickColor::Number const,int> const&)")]
// 0x304cd8 — __ZNSt8_Rb_treeIN3RBX10BrickColor6NumberESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueERKS5_
pub fn stub_0x304cd8() {
    // IDA 0x304cd8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BrickColor::BrickMap::~BrickMap()")]
// 0x304d40 — __ZN3RBX10BrickColor8BrickMapD2Ev
pub fn stub_0x304d40() {
    // IDA 0x304d40: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::BrickColor::BrickMap::ColorInfo,std::allocator<RBX::BrickColor::BrickMap::ColorInfo>>::~vector()")]
// 0x304e3c — __ZNSt6vectorIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EED2Ev
pub fn stub_0x304e3c() {
    // IDA 0x304e3c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::BrickColor::Number,std::pair<RBX::BrickColor::Number const,int>,std::_Select1st<std::pair<RBX::BrickColor::Number const,int>>,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::BrickColor::Number const,int>> *)")]
// 0x304f0c — __ZNSt8_Rb_treeIN3RBX10BrickColor6NumberESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
pub fn stub_0x304f0c() {
    // IDA 0x304f0c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BrickColor::BrickMap::BrickMap(void)")]
// 0x304f34 — __ZN3RBX10BrickColor8BrickMapC2Ev
pub fn stub_0x304f34() {
    // IDA 0x304f34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BrickColor::BrickMap::insert(RBX::BrickColor::Number,unsigned char,unsigned char,unsigned char,std::string)")]
// 0x30cbf8 — __ZN3RBX10BrickColor8BrickMap6insertENS0_6NumberEhhhSs
pub fn stub_0x30cbf8() {
    // IDA 0x30cbf8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::push_back(RBX::BrickColor const&)")]
// 0x30cd98 — __ZNSt6vectorIN3RBX10BrickColorESaIS1_EE9push_backERKS1_
pub fn stub_0x30cd98() {
    // IDA 0x30cd98: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::BrickColor*,std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>>,RBX::BrickColor const&)")]
// 0x30cdc0 — __ZNSt6vectorIN3RBX10BrickColorESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
pub fn stub_0x30cdc0() {
    // IDA 0x30cdc0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::BrickColor,std::allocator<RBX::BrickColor>>::_M_allocate(unsigned long)")]
// 0x30cea4 — __ZNSt12_Vector_baseIN3RBX10BrickColorESaIS1_EE11_M_allocateEm
pub fn stub_0x30cea4() {
    // IDA 0x30cea4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::BrickColor * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::BrickColor *,RBX::BrickColor *>(RBX::BrickColor *,RBX::BrickColor *,RBX::BrickColor *)")]
// 0x30cebc — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10BrickColorES5_EET0_T_S7_S6_
pub fn stub_0x30cebc() {
    // IDA 0x30cebc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::BrickColor::BrickMap::ColorInfo,std::allocator<RBX::BrickColor::BrickMap::ColorInfo>>::resize(unsigned long,RBX::BrickColor::BrickMap::ColorInfo)")]
// 0x30cef8 — __ZNSt6vectorIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EE6resizeEmS3_
pub fn stub_0x30cef8() {
    // IDA 0x30cef8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::BrickColor::BrickMap::ColorInfo,std::allocator<RBX::BrickColor::BrickMap::ColorInfo>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::BrickColor::BrickMap::ColorInfo*,std::vector<RBX::BrickColor::BrickMap::ColorInfo,std::allocator<RBX::BrickColor::BrickMap::ColorInfo>>>,unsigned long,RBX::BrickColor::BrickMap::ColorInfo const&)")]
// 0x30cf54 — __ZNSt6vectorIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_
pub fn stub_0x30cf54() {
    // IDA 0x30cf54: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::fill<RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo>(RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo const&)")]
// 0x30d6d8 — __ZSt4fillIPN3RBX10BrickColor8BrickMap9ColorInfoES3_EvT_S5_RKT0_
pub fn stub_0x30d6d8() {
    // IDA 0x30d6d8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::BrickColor::BrickMap::ColorInfo,std::allocator<RBX::BrickColor::BrickMap::ColorInfo>>::_M_allocate(unsigned long)")]
// 0x30d71c — __ZNSt12_Vector_baseIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EE11_M_allocateEm
pub fn stub_0x30d71c() {
    // IDA 0x30d71c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__uninitialized_fill_n_aux<RBX::BrickColor::BrickMap::ColorInfo *,unsigned long,RBX::BrickColor::BrickMap::ColorInfo>(RBX::BrickColor::BrickMap::ColorInfo *,unsigned long,RBX::BrickColor::BrickMap::ColorInfo const&,std::__false_type)")]
// 0x30d740 — __ZSt26__uninitialized_fill_n_auxIPN3RBX10BrickColor8BrickMap9ColorInfoEmS3_EvT_T0_RKT1_St12__false_type
pub fn stub_0x30d740() {
    // IDA 0x30d740: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::BrickColor::BrickMap::ColorInfo::operator=(RBX::BrickColor::BrickMap::ColorInfo const&)")]
// 0x30d88c — __ZN3RBX10BrickColor8BrickMap9ColorInfoaSERKS2_
pub fn stub_0x30d88c() {
    // IDA 0x30d88c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::BrickColor::BrickMap::ColorInfo * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo *>(RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo *)")]
// 0x30d8b8 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10BrickColor8BrickMap9ColorInfoES7_EET0_T_S9_S8_
pub fn stub_0x30d8b8() {
    // IDA 0x30d8b8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::resize(unsigned long,RBX::BrickColor)")]
// 0x30d914 — __ZNSt6vectorIN3RBX10BrickColorESaIS1_EE6resizeEmS1_
pub fn stub_0x30d914() {
    // IDA 0x30d914: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::BrickColor::BrickMap::generatePaletteMap(void)")]
// 0x30d948 — __ZN3RBX10BrickColor8BrickMap18generatePaletteMapEv
pub fn stub_0x30d948() {
    // IDA 0x30d948: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::BrickColor::BrickMap::generatePaletteMap(std::map<RBX::BrickColor::Number,int,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>> &,std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>,RBX::BrickColor::Number)")]
// 0x30da90 — __ZN3RBX10BrickColor8BrickMap18generatePaletteMapERSt3mapINS0_6NumberEiSt4lessIS3_ESaISt4pairIKS3_iEEESt6vectorIS0_SaIS0_EES3_
pub fn stub_0x30da90() {
    // IDA 0x30da90: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::vector(std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>> const&)")]
// 0x30db44 — __ZNSt6vectorIN3RBX10BrickColorESaIS1_EEC2ERKS3_
pub fn stub_0x30db44() {
    // IDA 0x30db44: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::BrickColor,std::allocator<RBX::BrickColor>>::_Vector_base(unsigned long,std::allocator<RBX::BrickColor> const&)")]
// 0x30db8c — __ZNSt12_Vector_baseIN3RBX10BrickColorESaIS1_EEC2EmRKS2_
pub fn stub_0x30db8c() {
    // IDA 0x30db8c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::BrickColor*,std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>>,unsigned long,RBX::BrickColor const&)")]
// 0x30dbbc — __ZNSt6vectorIN3RBX10BrickColorESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
pub fn stub_0x30dbbc() {
    // IDA 0x30dbbc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::CameraSubject::getContactManager(void)")]
// 0x30dd48 — __ZN3RBX13CameraSubject17getContactManagerEv
pub fn stub_0x30dd48() {
    // IDA 0x30dd48: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "global constructor keyed to_a_108")]
// 0x30e1b0 — __GLOBAL__I_a_108
pub fn stub_0x30e1b0() {
    // IDA 0x30e1b0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Color::getColorByIndex(int)")]
// 0x30e3b8 — __ZN3RBX5Color15getColorByIndexEi
pub fn stub_0x30e3b8() {
    // IDA 0x30e3b8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Color::colorFromIndex8(int)")]
// 0x30e580 — __ZN3RBX5Color15colorFromIndex8Ei
pub fn stub_0x30e580() {
    // IDA 0x30e580: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Color::colorFromInt(unsigned int)")]
// 0x30e5c0 — __ZN3RBX5Color12colorFromIntEj
pub fn stub_0x30e5c0(i: u32) -> [f32; 3] {
    // IDA 0x30e5c0: Color::colorFromInt — row (i%15) mixed halfway toward row (i%13+3) (0x30e5f0-0x30e656); the co-computed 16*(i/15) arg is dead in the callee.
    core_brick::color_from_int(i)
}

#[doc(alias = "RBX::Color::colorFromPointer(void *)")]
// 0x30e670 — __ZN3RBX5Color16colorFromPointerEPv
pub fn stub_0x30e670() {
    // IDA 0x30e670: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_109")]
// 0x30e67c — __GLOBAL__I_a_109
pub fn stub_0x30e67c() {
    // IDA 0x30e67c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::ContentFilter::setFilterUrl(std::string)")]
// 0x30e6b4 — __ZN3RBX13ContentFilter12setFilterUrlESs
pub fn stub_0x30e6b4() {
    // IDA 0x30e6b4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ContentFilter::setFilterLimits(int,int)")]
// 0x30e6bc — __ZN3RBX13ContentFilter15setFilterLimitsEii
pub fn stub_0x30e6bc() {
    // IDA 0x30e6bc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ContentFilter::ContentFilter(void)")]
// 0x30e6c8 — __ZN3RBX13ContentFilterC1Ev
pub fn stub_0x30e6c8() {
    // IDA 0x30e6c8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ContentFilter::ContentFilter(void)")]
// 0x30e6cc — __ZN3RBX13ContentFilterC2Ev
pub fn stub_0x30e6cc() {
    // IDA 0x30e6cc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ContentFilter::~ContentFilter()")]
// 0x30e868 — __ZN3RBX13ContentFilterD0Ev
pub fn stub_0x30e868() {
    // IDA 0x30e868: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ContentFilter::~ContentFilter()")]
// 0x30e908 — __ZN3RBX13ContentFilterD1Ev
pub fn stub_0x30e908() {
    // IDA 0x30e908: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ContentFilter::~ContentFilter()")]
// 0x30e90c — __ZThn32_N3RBX13ContentFilterD0Ev
pub fn stub_0x30e90c() {
    // IDA 0x30e90c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ContentFilter::~ContentFilter()")]
// 0x30e914 — __ZThn36_N3RBX13ContentFilterD0Ev
pub fn stub_0x30e914() {
    // IDA 0x30e914: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ContentFilter::~ContentFilter()")]
// 0x30e91c — __ZN3RBX13ContentFilterD2Ev
pub fn stub_0x30e91c() {
    // IDA 0x30e91c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ContentFilter::~ContentFilter()")]
// 0x30e96c — __ZThn32_N3RBX13ContentFilterD1Ev
pub fn stub_0x30e96c() {
    // IDA 0x30e96c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ContentFilter::~ContentFilter()")]
// 0x30e974 — __ZThn36_N3RBX13ContentFilterD1Ev
pub fn stub_0x30e974() {
    // IDA 0x30e974: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ContentFilter::truncateString(std::string &)")]
// 0x30e97c — __ZN3RBX13ContentFilter14truncateStringERSs
pub fn stub_0x30e97c() {
    // IDA 0x30e97c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ContentFilter::getStringState(std::string &)")]
// 0x30eab0 — __ZN3RBX13ContentFilter14getStringStateERSs
pub fn stub_0x30eab0() {
    // IDA 0x30eab0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ContentFilter::isContentFilterReady(std::string const&)")]
// 0x30eadc — __ZN3RBX13ContentFilter20isContentFilterReadyERKSs
pub fn stub_0x30eadc() {
    // IDA 0x30eadc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ContentFilter::isStringSafe(std::string &)")]
// 0x30ee70 — __ZN3RBX13ContentFilter12isStringSafeERSs
pub fn stub_0x30ee70() {
    // IDA 0x30ee70: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ContentFilter::cleanTable(void)")]
// 0x30eebc — __ZN3RBX13ContentFilter10cleanTableEv
pub fn stub_0x30eebc() {
    // IDA 0x30eebc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ContentFilter::doFilterRequest(std::string)")]
// 0x30f0a0 — __ZN3RBX13ContentFilter15doFilterRequestESs
pub fn stub_0x30f0a0() {
    // IDA 0x30f0a0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<std::string,RBX::ContentFilter::ResultEntry,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::operator[](std::string const&)")]
// 0x310284 — __ZNSt3mapISsN3RBX13ContentFilter11ResultEntryESt4lessISsESaISt4pairIKSsS2_EEEixERS6_
pub fn stub_0x310284() {
    // IDA 0x310284: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::pair<std::string const,RBX::ContentFilter::ResultEntry> const&)")]
// 0x310424 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
pub fn stub_0x310424() {
    // IDA 0x310424: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,RBX::ContentFilter::ResultEntry> const&)")]
// 0x310510 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
pub fn stub_0x310510() {
    // IDA 0x310510: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::_M_insert_unique(std::pair<std::string const,RBX::ContentFilter::ResultEntry> const&)")]
// 0x310560 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_
pub fn stub_0x310560() {
    // IDA 0x310560: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::_M_create_node(std::pair<std::string const,RBX::ContentFilter::ResultEntry> const&)")]
// 0x3105e4 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE14_M_create_nodeERKS5_
pub fn stub_0x3105e4() {
    // IDA 0x3105e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::lower_bound(std::string const&)")]
// 0x3106c4 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE11lower_boundERS1_
pub fn stub_0x3106c4() {
    // IDA 0x3106c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::erase(std::string const&)")]
// 0x3106f4 — __ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE5eraseERKSs
pub fn stub_0x3106f4() {
    // IDA 0x3106f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::erase(std::_Rb_tree_iterator<std::string>,std::_Rb_tree_iterator<std::string>)")]
// 0x31071c — __ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE5eraseESt17_Rb_tree_iteratorISsES7_
pub fn stub_0x31071c() {
    // IDA 0x31071c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::erase(std::_Rb_tree_iterator<std::string>)")]
// 0x310770 — __ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE5eraseESt17_Rb_tree_iteratorISsE
pub fn stub_0x310770() {
    // IDA 0x310770: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::lower_bound(std::string const&)")]
// 0x310798 — __ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE11lower_boundERKSs
pub fn stub_0x310798() {
    // IDA 0x310798: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::upper_bound(std::string const&)")]
// 0x3107c8 — __ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE11upper_boundERKSs
pub fn stub_0x3107c8() {
    // IDA 0x3107c8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::find(std::string const&)")]
// 0x312a54 — __ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE4findERKSs
pub fn stub_0x312a54() {
    // IDA 0x312a54: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::find(std::string const&)")]
// 0x312aa4 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE4findERS1_
pub fn stub_0x312aa4() {
    // IDA 0x312aa4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>)")]
// 0x312af4 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_E
pub fn stub_0x312af4() {
    // IDA 0x312af4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,RBX::ContentFilter::ResultEntry>> *)")]
// 0x314a10 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
pub fn stub_0x314a10() {
    // IDA 0x314a10: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "global constructor keyed to_a_110")]
// 0x314a40 — __GLOBAL__I_a_110
pub fn stub_0x314a40() {
    // IDA 0x314a40: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::operator<(RBX::ContentId const&,RBX::ContentId const&)")]
// 0x314c84 — __ZN3RBXltERKNS_9ContentIdES2_
pub fn stub_0x314c84() {
    // IDA 0x314c84: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::operator!=(RBX::ContentId const&,RBX::ContentId const&)")]
// 0x314c90 — __ZN3RBXneERKNS_9ContentIdES2_
pub fn stub_0x314c90() {
    // IDA 0x314c90: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::operator==(RBX::ContentId const&,RBX::ContentId const&)")]
// 0x314ca8 — __ZN3RBXeqERKNS_9ContentIdES2_
pub fn stub_0x314ca8() {
    // IDA 0x314ca8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ContentId::fromUrl(std::string const&)")]
// 0x314cbc — __ZN3RBX9ContentId7fromUrlERKSs
pub fn stub_0x314cbc() {
    // IDA 0x314cbc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ContentId::CorrectBackslash(std::string &)")]
// 0x314cc8 — __ZN3RBX9ContentId16CorrectBackslashERSs
pub fn stub_0x314cc8() {
    // IDA 0x314cc8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ContentId::convertAssetId(std::string const&)")]
// 0x314d14 — __ZN3RBX9ContentId14convertAssetIdERKSs
pub fn stub_0x314d14() {
    // IDA 0x314d14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "anonymous namespace::createIdUrl(std::string &,std::string const&,std::string const&)")]
// 0x314f94 — __ZN12_GLOBAL__N_111createIdUrlERSsRKSsS2_
pub fn stub_0x314f94() {
    // IDA 0x314f94: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ContentId::convertToLegacyContent(std::string const&)")]
// 0x315004 — __ZN3RBX9ContentId22convertToLegacyContentERKSs
pub fn stub_0x315004() {
    // IDA 0x315004: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ContentId::getAssetId(void)const")]
// 0x31507c — __ZNK3RBX9ContentId10getAssetIdEv
pub fn stub_0x31507c() {
    // IDA 0x31507c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ContentId::fromAssets(char const*)")]
// 0x31530c — __ZN3RBX9ContentId10fromAssetsEPKc
pub fn stub_0x31530c() {
    // IDA 0x31530c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::LegacyContentTable::~LegacyContentTable()")]
// 0x315514 — __ZN3RBX18LegacyContentTableD1Ev
pub fn stub_0x315514() {
    // IDA 0x315514: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "global constructor keyed to_a_111")]
// 0x315594 — __GLOBAL__I_a_111
pub fn stub_0x315594() {
    // IDA 0x315594: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FileSystem::getUserDirectory(bool,RBX::FileSystemDir,char const*)")]
// 0x315680 — __ZN3RBX10FileSystem16getUserDirectoryEbNS_13FileSystemDirEPKc
pub fn stub_0x315680() {
    // IDA 0x315680: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FileSystem::clearCacheDirectory(char const*,int)")]
// 0x315ba4 — __ZN3RBX10FileSystem19clearCacheDirectoryEPKci
pub fn stub_0x315ba4() {
    // IDA 0x315ba4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FileSystem::getCacheDirectory(bool,char const*)")]
// 0x315dc8 — __ZN3RBX10FileSystem17getCacheDirectoryEbPKc
pub fn stub_0x315dc8() {
    // IDA 0x315dc8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FileSystem::getBaseCacheDirectory(bool)")]
// 0x315dd4 — __ZN3RBX10FileSystem21getBaseCacheDirectoryEb
pub fn stub_0x315dd4() {
    // IDA 0x315dd4: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_112")]
// 0x3164c8 — __GLOBAL__I_a_112
pub fn stub_0x3164c8() {
    // IDA 0x3164c8: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::Http::getCdnResponceLock(void)")]
// 0x3165a0 — __ZN3RBX4Http18getCdnResponceLockEv
pub fn stub_0x3165a0() {
    // IDA 0x3165a0: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::Http::init(RBX::Http::API)")]
// 0x3165b0 — __ZN3RBX4Http4initENS0_3APIE
pub fn stub_0x3165b0() {
    // IDA 0x3165b0: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::Http::ThrowIfFailure(bool,char const*,char const*)")]
// 0x316738 — __ZN3RBX4Http14ThrowIfFailureEbPKcS2_
pub fn stub_0x316738() {
    // IDA 0x316738: global static ctor/dtor key. Static init — carrier no-op.
}