//! `RBX::BrickColor::BrickMap` palette data for `rbx-script`.
//!
//! Generated from the binary: each `(number, r, g, b, name)` row mirrors one
//! `BrickMap::insert(number, r, g, b, name)` call inlined in
//! `RBX::BrickColor::BrickMap::BrickMap` (IDA 0x304f34; extracted from the
//! 144 `insert` + `std::string::string` pairs, e.g. `insert(..., 1, 242, 243,
//! 243, ...)` + `"White"` at 0x305004/0x30502e). Channel bytes convert via
//! `G3D::Color4(Color4uint8)` (IDA 0x30ccdc), i.e. `/ 255.0`.
//! `BRICK_COLOR_PALETTE` mirrors the 64-entry `colorPalette` vector behind
//! `BrickColor::random`/`palette` (IDA 0x304468/0x27320c): the 64
//! `push_back` lanes at 0x307670.. (slots 208 down).

/// One `BrickMap::insert` row: color number, sRGB bytes, palette name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BrickColorEntry {
    pub number: u32,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub name: &'static str,
}

/// All 144 `BrickMap::insert` rows in ctor order (IDA 0x304f34).
pub const BRICK_COLOR_TABLE: &[BrickColorEntry] = &[
    BrickColorEntry { number: 1, r: 242, g: 243, b: 243, name: "White" },
    BrickColorEntry { number: 2, r: 161, g: 165, b: 162, name: "Grey" },
    BrickColorEntry { number: 3, r: 249, g: 233, b: 153, name: "Light yellow" },
    BrickColorEntry { number: 5, r: 215, g: 197, b: 154, name: "Brick yellow" },
    BrickColorEntry { number: 6, r: 194, g: 218, b: 184, name: "Light green (Mint)" },
    BrickColorEntry { number: 9, r: 232, g: 186, b: 200, name: "Light reddish violet" },
    BrickColorEntry { number: 11, r: 128, g: 187, b: 219, name: "Pastel Blue" },
    BrickColorEntry { number: 12, r: 203, g: 132, b: 66, name: "Light orange brown" },
    BrickColorEntry { number: 18, r: 204, g: 142, b: 105, name: "Nougat" },
    BrickColorEntry { number: 21, r: 196, g: 40, b: 28, name: "Bright red" },
    BrickColorEntry { number: 22, r: 196, g: 112, b: 160, name: "Med. reddish violet" },
    BrickColorEntry { number: 23, r: 13, g: 105, b: 172, name: "Bright blue" },
    BrickColorEntry { number: 24, r: 245, g: 205, b: 48, name: "Bright yellow" },
    BrickColorEntry { number: 25, r: 98, g: 71, b: 50, name: "Earth orange" },
    BrickColorEntry { number: 26, r: 27, g: 42, b: 53, name: "Black" },
    BrickColorEntry { number: 27, r: 109, g: 110, b: 108, name: "Dark grey" },
    BrickColorEntry { number: 28, r: 40, g: 127, b: 71, name: "Dark green" },
    BrickColorEntry { number: 29, r: 161, g: 196, b: 140, name: "Medium green" },
    BrickColorEntry { number: 36, r: 243, g: 207, b: 155, name: "Lig. Yellowich orange" },
    BrickColorEntry { number: 37, r: 75, g: 151, b: 75, name: "Bright green" },
    BrickColorEntry { number: 38, r: 160, g: 95, b: 53, name: "Dark orange" },
    BrickColorEntry { number: 39, r: 193, g: 202, b: 222, name: "Light bluish violet" },
    BrickColorEntry { number: 40, r: 236, g: 236, b: 236, name: "Transparent" },
    BrickColorEntry { number: 41, r: 205, g: 84, b: 75, name: "Tr. Red" },
    BrickColorEntry { number: 42, r: 193, g: 223, b: 240, name: "Tr. Lg blue" },
    BrickColorEntry { number: 43, r: 123, g: 182, b: 232, name: "Tr. Blue" },
    BrickColorEntry { number: 44, r: 247, g: 241, b: 141, name: "Tr. Yellow" },
    BrickColorEntry { number: 45, r: 180, g: 210, b: 228, name: "Light blue" },
    BrickColorEntry { number: 47, r: 217, g: 133, b: 108, name: "Tr. Flu. Reddish orange" },
    BrickColorEntry { number: 48, r: 132, g: 182, b: 141, name: "Tr. Green" },
    BrickColorEntry { number: 49, r: 248, g: 241, b: 132, name: "Tr. Flu. Green" },
    BrickColorEntry { number: 50, r: 236, g: 232, b: 222, name: "Phosph. White" },
    BrickColorEntry { number: 100, r: 238, g: 196, b: 182, name: "Light red" },
    BrickColorEntry { number: 101, r: 218, g: 134, b: 122, name: "Medium red" },
    BrickColorEntry { number: 102, r: 110, g: 153, b: 202, name: "Medium blue" },
    BrickColorEntry { number: 103, r: 199, g: 193, b: 183, name: "Light grey" },
    BrickColorEntry { number: 104, r: 107, g: 50, b: 124, name: "Bright violet" },
    BrickColorEntry { number: 105, r: 226, g: 155, b: 64, name: "Br. yellowish orange" },
    BrickColorEntry { number: 106, r: 218, g: 133, b: 65, name: "Bright orange" },
    BrickColorEntry { number: 107, r: 0, g: 143, b: 156, name: "Bright bluish green" },
    BrickColorEntry { number: 108, r: 104, g: 92, b: 67, name: "Earth yellow" },
    BrickColorEntry { number: 110, r: 67, g: 84, b: 147, name: "Bright bluish violet" },
    BrickColorEntry { number: 111, r: 191, g: 183, b: 177, name: "Tr. Brown" },
    BrickColorEntry { number: 112, r: 104, g: 116, b: 172, name: "Medium bluish violet" },
    BrickColorEntry { number: 113, r: 228, g: 173, b: 200, name: "Tr. Medi. reddish violet" },
    BrickColorEntry { number: 115, r: 199, g: 210, b: 60, name: "Med. yellowish green" },
    BrickColorEntry { number: 116, r: 85, g: 165, b: 175, name: "Med. bluish green" },
    BrickColorEntry { number: 118, r: 183, g: 215, b: 213, name: "Light bluish green" },
    BrickColorEntry { number: 119, r: 164, g: 189, b: 71, name: "Br. yellowish green" },
    BrickColorEntry { number: 120, r: 217, g: 228, b: 167, name: "Lig. yellowish green" },
    BrickColorEntry { number: 121, r: 231, g: 172, b: 88, name: "Med. yellowish orange" },
    BrickColorEntry { number: 123, r: 211, g: 111, b: 76, name: "Br. reddish orange" },
    BrickColorEntry { number: 124, r: 146, g: 57, b: 120, name: "Bright reddish violet" },
    BrickColorEntry { number: 125, r: 234, g: 184, b: 146, name: "Light orange" },
    BrickColorEntry { number: 126, r: 165, g: 165, b: 203, name: "Tr. Bright bluish violet" },
    BrickColorEntry { number: 127, r: 220, g: 188, b: 129, name: "Gold" },
    BrickColorEntry { number: 128, r: 174, g: 122, b: 89, name: "Dark nougat" },
    BrickColorEntry { number: 131, r: 156, g: 163, b: 168, name: "Silver" },
    BrickColorEntry { number: 133, r: 213, g: 115, b: 61, name: "Neon orange" },
    BrickColorEntry { number: 134, r: 216, g: 221, b: 86, name: "Neon green" },
    BrickColorEntry { number: 135, r: 116, g: 134, b: 157, name: "Sand blue" },
    BrickColorEntry { number: 136, r: 135, g: 124, b: 144, name: "Sand violet" },
    BrickColorEntry { number: 137, r: 224, g: 152, b: 100, name: "Medium orange" },
    BrickColorEntry { number: 138, r: 149, g: 138, b: 115, name: "Sand yellow" },
    BrickColorEntry { number: 140, r: 32, g: 58, b: 86, name: "Earth blue" },
    BrickColorEntry { number: 141, r: 39, g: 70, b: 45, name: "Earth green" },
    BrickColorEntry { number: 143, r: 207, g: 226, b: 247, name: "Tr. Flu. Blue" },
    BrickColorEntry { number: 145, r: 121, g: 136, b: 161, name: "Sand blue metallic" },
    BrickColorEntry { number: 146, r: 149, g: 142, b: 163, name: "Sand violet metallic" },
    BrickColorEntry { number: 147, r: 147, g: 135, b: 103, name: "Sand yellow metallic" },
    BrickColorEntry { number: 148, r: 87, g: 88, b: 87, name: "Dark grey metallic" },
    BrickColorEntry { number: 149, r: 22, g: 29, b: 50, name: "Black metallic" },
    BrickColorEntry { number: 150, r: 171, g: 173, b: 172, name: "Light grey metallic" },
    BrickColorEntry { number: 151, r: 120, g: 144, b: 130, name: "Sand green" },
    BrickColorEntry { number: 153, r: 149, g: 121, b: 119, name: "Sand red" },
    BrickColorEntry { number: 154, r: 123, g: 46, b: 47, name: "Dark red" },
    BrickColorEntry { number: 157, r: 255, g: 246, b: 123, name: "Tr. Flu. Yellow" },
    BrickColorEntry { number: 158, r: 225, g: 164, b: 194, name: "Tr. Flu. Red" },
    BrickColorEntry { number: 168, r: 117, g: 108, b: 98, name: "Gun metallic" },
    BrickColorEntry { number: 176, r: 151, g: 105, b: 91, name: "Red flip/flop" },
    BrickColorEntry { number: 178, r: 180, g: 132, b: 85, name: "Yellow flip/flop" },
    BrickColorEntry { number: 179, r: 137, g: 135, b: 136, name: "Silver flip/flop" },
    BrickColorEntry { number: 180, r: 215, g: 169, b: 75, name: "Curry" },
    BrickColorEntry { number: 190, r: 249, g: 214, b: 46, name: "Fire Yellow" },
    BrickColorEntry { number: 191, r: 232, g: 171, b: 45, name: "Flame yellowish orange" },
    BrickColorEntry { number: 192, r: 105, g: 64, b: 40, name: "Reddish brown" },
    BrickColorEntry { number: 193, r: 207, g: 96, b: 36, name: "Flame reddish orange" },
    BrickColorEntry { number: 194, r: 163, g: 162, b: 165, name: "Medium stone grey" },
    BrickColorEntry { number: 195, r: 70, g: 103, b: 164, name: "Royal blue" },
    BrickColorEntry { number: 196, r: 35, g: 71, b: 139, name: "Dark Royal blue" },
    BrickColorEntry { number: 198, r: 142, g: 66, b: 133, name: "Bright reddish lilac" },
    BrickColorEntry { number: 199, r: 99, g: 95, b: 98, name: "Dark stone grey" },
    BrickColorEntry { number: 200, r: 130, g: 138, b: 93, name: "Lemon metalic" },
    BrickColorEntry { number: 208, r: 229, g: 228, b: 223, name: "Light stone grey" },
    BrickColorEntry { number: 209, r: 176, g: 142, b: 68, name: "Dark Curry" },
    BrickColorEntry { number: 210, r: 112, g: 149, b: 120, name: "Faded green" },
    BrickColorEntry { number: 211, r: 121, g: 181, b: 181, name: "Turquoise" },
    BrickColorEntry { number: 212, r: 159, g: 195, b: 233, name: "Light Royal blue" },
    BrickColorEntry { number: 213, r: 108, g: 129, b: 183, name: "Medium Royal blue" },
    BrickColorEntry { number: 216, r: 143, g: 76, b: 42, name: "Rust" },
    BrickColorEntry { number: 217, r: 124, g: 92, b: 70, name: "Brown" },
    BrickColorEntry { number: 218, r: 150, g: 112, b: 159, name: "Reddish lilac" },
    BrickColorEntry { number: 219, r: 107, g: 98, b: 155, name: "Lilac" },
    BrickColorEntry { number: 220, r: 167, g: 169, b: 206, name: "Light lilac" },
    BrickColorEntry { number: 221, r: 205, g: 98, b: 152, name: "Bright purple" },
    BrickColorEntry { number: 222, r: 228, g: 173, b: 200, name: "Light purple" },
    BrickColorEntry { number: 223, r: 220, g: 144, b: 149, name: "Light pink" },
    BrickColorEntry { number: 224, r: 240, g: 213, b: 160, name: "Light brick yellow" },
    BrickColorEntry { number: 225, r: 235, g: 184, b: 127, name: "Warm yellowish orange" },
    BrickColorEntry { number: 226, r: 253, g: 234, b: 141, name: "Cool yellow" },
    BrickColorEntry { number: 232, r: 125, g: 187, b: 221, name: "Dove blue" },
    BrickColorEntry { number: 268, r: 52, g: 43, b: 117, name: "Medium lilac" },
    BrickColorEntry { number: 1001, r: 248, g: 248, b: 248, name: "Institutional white" },
    BrickColorEntry { number: 1002, r: 205, g: 205, b: 205, name: "Mid gray" },
    BrickColorEntry { number: 1003, r: 17, g: 17, b: 17, name: "Really black" },
    BrickColorEntry { number: 1004, r: 255, g: 0, b: 0, name: "Really red" },
    BrickColorEntry { number: 1005, r: 255, g: 175, b: 0, name: "Deep orange" },
    BrickColorEntry { number: 1006, r: 180, g: 128, b: 255, name: "Alder" },
    BrickColorEntry { number: 1007, r: 163, g: 75, b: 75, name: "Dusty Rose" },
    BrickColorEntry { number: 1008, r: 193, g: 190, b: 66, name: "Olive" },
    BrickColorEntry { number: 1009, r: 255, g: 255, b: 0, name: "New Yeller" },
    BrickColorEntry { number: 1010, r: 0, g: 0, b: 255, name: "Really blue" },
    BrickColorEntry { number: 1011, r: 0, g: 32, b: 96, name: "Navy blue" },
    BrickColorEntry { number: 1012, r: 33, g: 84, b: 185, name: "Deep blue" },
    BrickColorEntry { number: 1013, r: 4, g: 175, b: 236, name: "Cyan" },
    BrickColorEntry { number: 1014, r: 170, g: 85, b: 0, name: "CGA brown" },
    BrickColorEntry { number: 1015, r: 170, g: 0, b: 170, name: "Magenta" },
    BrickColorEntry { number: 1016, r: 255, g: 102, b: 204, name: "Pink" },
    BrickColorEntry { number: 1017, r: 255, g: 175, b: 0, name: "Deep orange" },
    BrickColorEntry { number: 1018, r: 18, g: 238, b: 212, name: "Teal" },
    BrickColorEntry { number: 1019, r: 0, g: 255, b: 255, name: "Toothpaste" },
    BrickColorEntry { number: 1020, r: 0, g: 255, b: 0, name: "Lime green" },
    BrickColorEntry { number: 1021, r: 58, g: 125, b: 21, name: "Camo" },
    BrickColorEntry { number: 1022, r: 127, g: 142, b: 100, name: "Grime" },
    BrickColorEntry { number: 1023, r: 140, g: 91, b: 159, name: "Lavender" },
    BrickColorEntry { number: 1024, r: 175, g: 221, b: 255, name: "Pastel light blue" },
    BrickColorEntry { number: 1025, r: 255, g: 201, b: 201, name: "Pastel orange" },
    BrickColorEntry { number: 1026, r: 177, g: 167, b: 255, name: "Pastel violet" },
    BrickColorEntry { number: 1027, r: 159, g: 243, b: 233, name: "Pastel blue-green" },
    BrickColorEntry { number: 1028, r: 204, g: 255, b: 204, name: "Pastel green" },
    BrickColorEntry { number: 1029, r: 255, g: 255, b: 204, name: "Pastel yellow" },
    BrickColorEntry { number: 1030, r: 255, g: 204, b: 153, name: "Pastel brown" },
    BrickColorEntry { number: 1031, r: 98, g: 37, b: 209, name: "Royal purple" },
    BrickColorEntry { number: 1032, r: 255, g: 0, b: 191, name: "Hot pink" },
];

/// The 64-entry `colorPalette` order behind `random`/`palette`.
pub const BRICK_COLOR_PALETTE: &[u32] = &[
    119, 24, 106, 21, 104, 23, 107, 37,
    1001, 1, 208, 1002, 194, 199, 26, 1003,
    1022, 105, 125, 153, 1023, 135, 102, 151,
    5, 226, 133, 101, 9, 11, 1018, 29,
    1030, 1029, 1025, 1016, 1026, 1024, 1027, 1028,
    1008, 1009, 1017, 1004, 1032, 1010, 1019, 1020,
    217, 18, 38, 1031, 1006, 1013, 45, 1021,
    192, 1014, 1007, 1015, 1012, 1011, 28, 141,
];

/// `BrickMap::colors` lookup by number (IDA 0x304794): the `valid` arm is
/// always set for inserted rows; a missing number fails like the
/// `ReleaseAssert` guards in `color4`/`name` (IDA 0x304758/0x304698).
pub fn brick_color_entry(number: u32) -> Option<&'static BrickColorEntry> {
    BRICK_COLOR_TABLE.iter().find(|entry| entry.number == number)
}

/// `RBX::BrickColor::BrickColor(int)` (IDA 0x30456c): valid numbers keep
/// their value, anything else becomes 194 (`Medium stone grey`).
pub fn brickcolor_from_number(number: i64) -> u32 {
    if number >= 0 && brick_color_entry(number as u32).is_some() {
        number as u32
    } else {
        194
    }
}

/// `RBX::BrickColor::parse` (IDA 0x3043fc): case-sensitive name scan,
/// miss (or empty map) yields 194.
pub fn brickcolor_parse(name: &str) -> u32 {
    for entry in BRICK_COLOR_TABLE {
        if entry.name == name {
            return entry.number;
        }
    }
    194
}

/// `RBX::BrickColor::closest(Color4)` (IDA 0x3044c4): L1 distance over
/// valid rows, strictly-better updates from `(194, 10000.0)`, exact-zero
/// early-out.
pub fn brickcolor_closest(r: f32, g: f32, b: f32) -> u32 {
    let mut best = 194;
    let mut best_dist = 10000.0f32;
    for entry in BRICK_COLOR_TABLE {
        let er = f32::from(entry.r) / 255.0;
        let eg = f32::from(entry.g) / 255.0;
        let eb = f32::from(entry.b) / 255.0;
        let dist = (r - er).abs() + (g - eg).abs() + (b - eb).abs();
        if dist < best_dist {
            best_dist = dist;
            best = entry.number;
            if dist == 0.0 {
                break;
            }
        }
    }
    best
}
