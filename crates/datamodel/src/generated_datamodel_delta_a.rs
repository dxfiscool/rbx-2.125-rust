// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: RBX + Instance|DataModel|Workspace (broad, subclass-inclusive); EA-sorted asc, NOT in /tmp/global_eas.txt
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 23 stubs | range 0xf5cc74..0xf661c4 | broad remainder after batch: 0
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; stripped from alias
// Impl batch 0xf5cc74..0xf661c4: every EA below is a `__picsymbolstub4` PLT
// thunk (3-instruction jump, e.g. decomp 0xf65d34 `// attributes: thunk`), so
// each wrapper models the real target's observable behavior directly.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use std::collections::HashMap;

use rbx_core::{SharedPtr, WeakPtr};

/// Rust model of `RBX::MegaClusterInstance` for the weak-cell vector below
/// (IDA `0xf5e154`): the instance side is opaque here; only the weak identity
/// participates.
#[derive(Default)]
pub struct MegaClusterInstance {
    _opaque: (),
}

/// Rust model of `FactoryProduct<RemoteFunction, Instance>::Creator` (IDA
/// `0x91f54c`/`0x91f5e8`): the `wasConstructed` sentinel (`666`, `0x29A`) is
/// the only word the dtor/class-name paths read.
pub struct RemoteFunctionCreator {
    pub constructed: bool,
}

impl RemoteFunctionCreator {
    pub fn new() -> Self {
        Self { constructed: true }
    }
}

impl Default for RemoteFunctionCreator {
    fn default() -> Self {
        Self::new()
    }
}

/// Class name returned by the RemoteFunction creator (IDA `0x91f5e8` tails
/// into `Name::doDeclare<sRemoteFunction>`).
pub const REMOTE_FUNCTION_CLASS: &str = "RemoteFunction";

/// Rust model of `RBX::SpatialRegion::Id` (IDA `0xc0a25c`-`0xc0a264` loads
/// three int16s for the `Updating chunk (%u,%u,%u)` log).
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct SpatialRegionId {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

/// Rust model of `RBX::MegaClusterLegacy::ChunkData` (IDA `0xc0eb90`): the
/// scene node plus the solid/water entity slots, face counts, and the load
/// estimate word at `+0x1C0` (IDA `0xc0a370`-`0xc0a372`).
#[derive(Clone, Default)]
pub struct MegaChunkData {
    pub has_node: bool,
    pub has_entity: bool,
    pub has_water_entity: bool,
    pub face_count: u32,
    pub water_face_count: u32,
    pub load_estimate: u32,
    pub dirty: bool,
    pub water_dirty: bool,
}

/// Rust model of `RBX::MegaClusterLegacy` for the `<MegaClusterInstance>`
/// template batch (IDA `0xc0a23c`..`0xc0a448`): chunk map plus the storage /
/// entity / binding flags the unbuild/update paths touch.
#[derive(Default)]
pub struct MegaClusterLegacy {
    pub chunks: HashMap<SpatialRegionId, MegaChunkData>,
    pub storage_live: bool,
    pub entity_allocated: bool,
    pub bound: bool,
    pub listener_connected: bool,
    pub mesh_seq: u32,
    /// Chunk dimensions used only for the `ClusterMesh(%u)Chunk(%ux%ux%u)`
    /// names (IDA `0xc0eeb6`).
    /// // BUG: extents land with the voxel-chunk batch; name-only here.
    pub chunk_dims: [u32; 3],
    /// Graphics-chunk grid walked by `updateEntity_templated` (IDA `0xc0a006`
    /// over `kNumGraphicsChunks`).
    /// // BUG: counts land with the voxel-chunk batch; single cell here.
    pub graphics_chunks: [u32; 3],
}

impl MegaClusterLegacy {
    pub fn new() -> Self {
        Self {
            chunk_dims: [4, 4, 4],
            graphics_chunks: [1, 1, 1],
            ..Default::default()
        }
    }

    pub fn reset_dirty(&mut self, id: &SpatialRegionId) {
        // IDA `0xc0a2e4`: `resetDirty(id)` before the solid face count.
        if let Some(chunk) = self.chunks.get_mut(id) {
            chunk.dirty = false;
        }
    }

    pub fn reset_water_dirty(&mut self, id: &SpatialRegionId) {
        // IDA `0xc0a2a0`: `resetWaterDirty(id)` before the water face count.
        if let Some(chunk) = self.chunks.get_mut(id) {
            chunk.water_dirty = false;
        }
    }

    pub fn destroy_chunk(&mut self, id: &SpatialRegionId) {
        // IDA `0xc0a38e`: `destroyChunk` once both chunk slots test null.
        self.chunks.remove(id);
    }
}

/// Face direction word (`RBX::Voxel::FaceDirection`, IDA `0xc10cec`).
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct FaceDirection(pub u32);

/// Render predicate word (`RBX::RenderPredStatus`, IDA `0xc10cec` bit-tests
/// `status & 1` / `status & 2`).
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct RenderPredStatus(pub u32);

/// Opposite-face table (IDA `0x10098ac`,
/// `RBX::oppositeSideOffset(FaceDirection)::OPPOSITES`).
pub const OPPOSITE_SIDE: [u32; 8] = [2, 3, 0, 1, 5, 4, 0, 0];

pub fn opposite_side(dir: u32) -> u32 {
    // IDA `0xc12c7c`/`0xc10f74`: `OPPOSITES[dir]`; slots 6..7 read 0.
    OPPOSITE_SIDE[(dir & 7) as usize]
}

/// Axis offsets per face direction, derived from the `OPPOSITE_SIDE` pairs
/// `(0,2)`, `(1,3)`, `(4,5)` (IDA `0xc10e6c`
/// `FaceDirectionLocationOffset[3 * dir]`).
/// // BUG: sign assignment is conventional; the table itself is
/// // runtime-filled in the DB (cf. `0x1329160`).
pub const FACE_LOCATION_OFFSETS: [[i16; 3]; 6] =
    [[1, 0, 0], [0, 1, 0], [-1, 0, 0], [0, -1, 0], [0, 0, 1], [0, 0, -1]];

/// Face-to-status table (IDA `0x102a400`, `RBX::faceToStatusMap`).
pub const FACE_TO_STATUS: [u8; 16] = [1, 1, 1, 1, 0, 2, 2, 1, 0, 3, 1, 1, 0, 3, 2, 1];

/// Adjacent-direction rows for `detectOutlines` (IDA `0x102a2dc`, four words
/// per face, `table + 16 * face` at `0xc117a6`).
pub const ADJ_DIRECTIONS: [[u32; 4]; 6] = [
    [4, 5, 1, 3],
    [4, 5, 2, 0],
    [4, 5, 3, 1],
    [4, 5, 0, 2],
    [3, 1, 2, 0],
    [1, 3, 2, 0],
];

/// Wedge-direction tables for `detectWedgeOutlines` (IDA `0x102a27c`,
/// `0x102a29c`, `0x102a2bc`, `0x102a2cc`).
pub const VERTICAL_WEDGE_DIRS: [u32; 8] = [4, 3, 5, 1, 2, 6, 0, 6];
pub const HORIZONTAL_WEDGE_DIRS: [u32; 8] = [4, 6, 5, 6, 2, 3, 0, 1];
pub const CORNER_WEDGE_DIRS: [u32; 4] = [0, 5, 3, 0];
pub const INVERSE_CORNER_WEDGE_DIRS: [u32; 4] = [4, 2, 2, 1];

/// `UpEmptyBlocks` byte table for `wedgeUpEmpty`, kept flat exactly as the
/// original indexes it (`table[6 * shape + neighbor_shape]`, IDA `0xc116dc`;
/// the tail bytes past entry 48 belong to the following table and are read
/// the same way when the index runs past).
pub const UP_EMPTY_BLOCKS: [u8; 64] = [
    0, 0, 0, 0, 0, 1, 1, 0,
    1, 1, 1, 1, 1, 1, 1, 0,
    1, 1, 1, 1, 0, 1, 1, 1,
    1, 1, 0, 1, 0, 1, 0, 0,
    0, 0, 0, 0, 1, 0, 0, 0,
    0, 0, 0, 0, 3, 0, 0, 0,
    2, 0, 0, 0, 2, 0, 0, 0,
    1, 0, 0, 0, 0, 0, 0, 0,
];

/// Occupancy contribution table for `occupancyFillTerrain` (IDA `0x102a549`).
pub const OCCUPANCY_TABLE: [u8; 8] = [255, 128, 42, 213, 128, 0, 0, 0];

/// Rust model of `MegaClusterInstance::CellChunk` cell storage: only
/// non-empty cells are kept (absent means empty/outside), and materials are
/// kept as one nibble per cell.
/// // BUG: the original packs two nibbles per material byte
/// // (`(mats[(base+off)/2] >> parity) & 0xF`, IDA `0xc11774`) and strides
/// // cells via the runtime `kOffsetMultipliers`; both collapse here into
/// // per-cell values with coordinate keys.
#[derive(Default)]
pub struct VoxelCellChunk {
    pub cells: HashMap<[i16; 3], u8>,
    pub materials: HashMap<[i16; 3], u8>,
}

impl VoxelCellChunk {
    pub fn cell(&self, pos: [i16; 3]) -> u8 {
        self.cells.get(&pos).copied().unwrap_or(0)
    }

    pub fn material(&self, pos: [i16; 3]) -> u32 {
        // IDA `0xc11774`: empty stays 17; otherwise nibble + 1.
        let cell = self.cell(pos);
        if cell & 0x38 != 0x28 {
            (self.materials.get(&pos).copied().unwrap_or(0) & 0xF) as u32 + 1
        } else {
            17
        }
    }
}

/// Rust model of `RBX::Voxel::Region<CellChunk>` (IDA `0xc24516`): extents
/// plus the chunk; `None` is the null-chunk early-out (IDA `0xc30144`).
pub struct VoxelRegion {
    pub min: [i16; 3],
    pub max: [i16; 3],
    pub chunk: Option<SharedPtr<VoxelCellChunk>>,
}

/// Rust model of the `xline_iterator` frame (IDA `0xc30100`): line counters
/// and the done flag. Strides and the raw data pointer collapse — lookups go
/// through the chunk map.
/// // BUG: `currentIndex` evenness is asserted, not stored (IDA `0xc301d0`).
#[derive(Clone, Default)]
pub struct VoxelXlineIter {
    pub x_remaining: i32,
    pub y: i16,
    pub z_remaining: i32,
    pub z: i16,
    pub done: bool,
}

/// One emitted terrain face: the 1:1 observable of the `outputFace` calls
/// (`*cursor += 20` at `0xc11aee`/`0xc113c8`, `+= 80` at `0xc12eb6`).
#[derive(Clone, Default)]
pub struct EmittedFace {
    pub cell: u8,
    pub dir: u32,
    pub water: bool,
    pub outlines: u8,
}

/// Output-face cursor: the Ogre vertex buffer collapses into the emitted
/// face log plus the word cursor the originals advance.
#[derive(Default)]
pub struct FaceOutput {
    pub faces: Vec<EmittedFace>,
    pub cursor_words: usize,
}

/// Rust model of `RBX::LightGridChunk` occupancy (IDA `0xc246e6`): one byte
/// per light cell over a 32-wide xline (`kLightGridChunkSizeXZ`, IDA
/// `0xc24638`).
#[derive(Default)]
pub struct LightGridChunk {
    pub occupancy: Vec<u8>,
}

/// Oriented-face lookup (`RBX::Voxel::OrientedFaceMap[6 * cell + dir]`, value
/// 5 hides the face, IDA `0xc129d6`).
/// // BUG: the map is runtime-filled in the DB (cf. `0x13032b0`); cubes map
/// // identity and the wedge remap lands with the voxel-chunk batch.
pub fn oriented_face(cell: u8, dir: u32) -> u8 {
    if cell & 0x38 == 0 {
        (dir & 7) as u8
    } else {
        (dir & 7) as u8
    }
}

pub const ORIENTED_HIDDEN: u8 = 5;

/// Water bit used by `WaterRenderPredicate::internal` (IDA `0xc12a32`:
/// `((cell >> 3) & 7) != 0` may be water; shape 5 always is).
pub fn cell_water_bit(cell: u8, area_water: bool) -> bool {
    let shape = (cell >> 3) & 7;
    if shape == 0 {
        false
    } else if shape == 5 {
        true
    } else {
        // IDA `0xc12a50`-`0xc12a72`: `fillLocalAreaInfo` plus
        // `isWaterOnWedge`; the neighborhood collapses into `area_water`.
        area_water
    }
}

// 0xf5cc74 — j___ZN3RBX14FactoryProductINS_14RemoteFunctionENS_8InstanceELZNS_15sRemoteFunctionEES2_E7CreatorD2Ev
#[doc(alias = "j___ZN3RBX14FactoryProductINS_14RemoteFunctionENS_8InstanceELZNS_15sRemoteFunctionEES2_E7CreatorD2Ev")]
pub fn stub_0xf5cc74(
    creator: &mut RemoteFunctionCreator,
    creators: &mut HashMap<String, ()>,
) {
    // IDA 0xf5cc74: `__picsymbolstub4` into the real D2 at 0x91f54c, which
    // installs the vtable (the Rust type is the vtable), asserts
    // `wasConstructed()` (`isConstructed == 666`, Object.h:255), then erases
    // the `getClassName()` key from the `getCreators()` tree (0x91f5c4/0x91f5de).
    debug_assert!(creator.constructed, "0x91f54c: wasConstructed()");
    creator.constructed = false;
    creators.remove(REMOTE_FUNCTION_CLASS);
}

// 0xf5d874 — j___ZNK3RBX14FactoryProductINS_14RemoteFunctionENS_8InstanceELZNS_15sRemoteFunctionEES2_E7Creator12getClassNameEv
#[doc(alias = "j___ZNK3RBX14FactoryProductINS_14RemoteFunctionENS_8InstanceELZNS_15sRemoteFunctionEES2_E7Creator12getClassNameEv")]
pub fn stub_0xf5d874(creator: &RemoteFunctionCreator) -> &'static str {
    // IDA 0xf5d874: `__picsymbolstub4` into 0x91f5e8, which asserts
    // `wasConstructed()` (0x91f5fa-0x91f648, magic 0x29A = 666), runs the
    // `Name::declare<sRemoteFunction>` once-flag (collapses into the static),
    // and tail-jumps to `Name::doDeclare` returning the interned name.
    debug_assert!(creator.constructed, "0x91f5e8: wasConstructed()");
    REMOTE_FUNCTION_CLASS
}

// 0xf5e154 — j___ZNSt6vectorIN5boost8weak_ptrIN3RBX19MegaClusterInstanceEEESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_
#[doc(alias = "std::vector<boost::weak_ptr<RBX::MegaClusterInstance>,std::allocator<boost::weak_ptr<RBX::MegaClusterInstance>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::weak_ptr<RBX::MegaClusterInstance>*,std::vector<boost::weak_ptr<RBX::MegaClusterInstance>,std::allocator<boost::weak_ptr<RBX::MegaClusterInstance>>>>,boost::weak_ptr<RBX::MegaClusterInstance> const&)")]
pub fn stub_0xf5e154(
    items: &mut Vec<WeakPtr<MegaClusterInstance>>,
    index: usize,
    value: WeakPtr<MegaClusterInstance>,
) {
    // IDA 0xf5e154: `__picsymbolstub4` into `vector<weak_ptr>::_M_insert_aux`
    // — reallocation plus the backward shift collapse into the splice.
    let at = index.min(items.len());
    items.insert(at, value);
}

// 0xf5e164 — j___ZNSt6vectorIN5boost8weak_ptrIN3RBX19MegaClusterInstanceEEESaIS4_EE9push_backERKS4_
#[doc(alias = "std::vector<boost::weak_ptr<RBX::MegaClusterInstance>,std::allocator<boost::weak_ptr<RBX::MegaClusterInstance>>>::push_back(boost::weak_ptr<RBX::MegaClusterInstance> const&)")]
pub fn stub_0xf5e164(
    items: &mut Vec<WeakPtr<MegaClusterInstance>>,
    value: WeakPtr<MegaClusterInstance>,
) {
    // IDA 0xf5e164: `__picsymbolstub4` into `vector<weak_ptr>::push_back`
    // (fast path plus `_M_insert_aux` on full) — `push` is both.
    items.push(value);
}

// 0xf5e174 — j___ZNSt6vectorIN5boost8weak_ptrIN3RBX19MegaClusterInstanceEEESaIS4_EED2Ev
#[doc(alias = "std::vector<boost::weak_ptr<RBX::MegaClusterInstance>,std::allocator<boost::weak_ptr<RBX::MegaClusterInstance>>>::~vector()")]
pub fn stub_0xf5e174(items: &mut Vec<WeakPtr<MegaClusterInstance>>) {
    // IDA 0xf5e174: `__picsymbolstub4` into `vector<weak_ptr>::D2` — destroys
    // the elements in place (storage release is the D0 path).
    items.clear();
}

// 0xf5e234 — j___ZSt24__uninitialized_copy_auxIPN5boost8weak_ptrIN3RBX19MegaClusterInstanceEEES5_ET0_T_S7_S6_St12__false_type
#[doc(alias = "boost::weak_ptr<RBX::MegaClusterInstance> * std::__uninitialized_copy_aux<boost::weak_ptr<RBX::MegaClusterInstance> *,boost::weak_ptr<RBX::MegaClusterInstance> *>(boost::weak_ptr<RBX::MegaClusterInstance> *,boost::weak_ptr<RBX::MegaClusterInstance> *,boost::weak_ptr<RBX::MegaClusterInstance> *,std::__false_type)")]
pub fn stub_0xf5e234(
    dst: &mut Vec<WeakPtr<MegaClusterInstance>>,
    src: &[WeakPtr<MegaClusterInstance>],
) -> usize {
    // IDA 0xf5e234: `__picsymbolstub4` into `__uninitialized_copy_aux` with
    // `__false_type` — no exception unwinding; each weak is copy-constructed
    // (`Weak::clone` keeps the same observable identity).
    dst.extend(src.iter().cloned());
    dst.len()
}

// 0xf65d34 — j___ZN3RBX17MegaClusterLegacy11createChunkINS_19MegaClusterInstanceEEEvRKNS_13SpatialRegion2IdEjj
#[doc(alias = "void RBX::MegaClusterLegacy::createChunk<RBX::MegaClusterInstance>(RBX::SpatialRegion::Id const&,unsigned int,unsigned int)")]
pub fn stub_0xf65d34(
    legacy: &mut MegaClusterLegacy,
    id: SpatialRegionId,
    solid_faces: u32,
    water_faces: u32,
) {
    // IDA 0xf65d34: `__picsymbolstub4` into 0xc0eb90, which asserts the chunk
    // slots start null (`chunk.node/entity/waterEntity == NULL`, 0xc0ec76),
    // logs the update (0xc0ed28), then builds the `ClusterMesh(%u)Chunk(...)`
    // / `ClusterEntity(%u)Chunk(...)` Ogre meshes plus the `...Water...`
    // twins (0xc0eeb6-0xc0f418). The Ogre build collapses into the stored
    // face counts; the D2-time asserts below pin the shape.
    let slot = legacy.chunks.entry(id).or_default();
    debug_assert!(
        !slot.has_node && !slot.has_entity && !slot.has_water_entity,
        "0xc0eb90: chunk slots start null"
    );
    legacy.mesh_seq += 1;
    slot.has_node = true;
    slot.has_entity = solid_faces > 0;
    slot.has_water_entity = water_faces > 0;
    slot.face_count = solid_faces;
    slot.water_face_count = water_faces;
    slot.load_estimate = solid_faces.saturating_add(water_faces) / 24;
    slot.dirty = false;
    slot.water_dirty = false;
}

// 0xf65d74 — j___ZN3RBX17MegaClusterLegacy17unbuild_templatedINS_19MegaClusterInstanceEEEvv
#[doc(alias = "void RBX::MegaClusterLegacy::unbuild_templated<RBX::MegaClusterInstance>(void)")]
pub fn stub_0xf65d74(legacy: &mut MegaClusterLegacy) {
    // IDA 0xf65d74: `__picsymbolstub4` into 0xc0a448 — unbinds gfx
    // (0xc0a472), disconnects the storage listener (0xc0a4a2), destroys every
    // chunk (0xc0a4ac-0xc0a4b2), drops the scene-updater clusters (0xc0a4ce),
    // nulls storage/entity (0xc0a4d4-0xc0a4d8), and releases the shared
    // (0xc0a4e8). The FLog branches (0xc0a45e/0xc0a47a) collapse.
    legacy.bound = false;
    legacy.listener_connected = false;
    legacy.chunks.clear();
    legacy.storage_live = false;
    legacy.entity_allocated = false;
}

// 0xf65d94 — j___ZN3RBX17MegaClusterLegacy19updateChunkGeometryINS_19MegaClusterInstanceEEEvRKNS_13SpatialRegion2IdERNS0_9ChunkDataEi
#[doc(alias = "void RBX::MegaClusterLegacy::updateChunkGeometry<RBX::MegaClusterInstance>(RBX::SpatialRegion::Id const&,RBX::MegaClusterLegacy::ChunkData &,int)")]
pub fn stub_0xf65d94(
    legacy: &mut MegaClusterLegacy,
    id: SpatialRegionId,
    face_count: i32,
) {
    // IDA 0xf65d94: `__picsymbolstub4` into 0xc10058 — rebuilds the chunk's
    // Ogre mesh from the counted faces (vertex decl/binding/submesh frame at
    // 0xc0f046-0xc0f418 for the create path). The mesh rebuild collapses into
    // the stored count; the entity slot stays lit.
    if let Some(chunk) = legacy.chunks.get_mut(&id) {
        chunk.face_count = face_count.max(0) as u32;
        chunk.has_entity = chunk.face_count > 0;
        chunk.dirty = false;
    }
}

// 0xf65db4 — j___ZN3RBX17MegaClusterLegacy19updateWaterGeometryINS_19MegaClusterInstanceEEEvRKNS_13SpatialRegion2IdERNS0_9ChunkDataEj
#[doc(alias = "void RBX::MegaClusterLegacy::updateWaterGeometry<RBX::MegaClusterInstance>(RBX::SpatialRegion::Id const&,RBX::MegaClusterLegacy::ChunkData &,unsigned int)")]
pub fn stub_0xf65db4(
    legacy: &mut MegaClusterLegacy,
    id: SpatialRegionId,
    face_count: u32,
) {
    // IDA 0xf65db4: `__picsymbolstub4` into 0xc0fbb4 — the water twin of
    // 0xc10058 over the `Water` mesh/entity pair.
    if let Some(chunk) = legacy.chunks.get_mut(&id) {
        chunk.water_face_count = face_count;
        chunk.has_water_entity = face_count > 0;
        chunk.water_dirty = false;
    }
}

// 0xf65dd4 — j___ZN3RBX17MegaClusterLegacy21updateChunk_templatedINS_19MegaClusterInstanceEEEvRKNS_13SpatialRegion2IdEb
#[doc(alias = "void RBX::MegaClusterLegacy::updateChunk_templated<RBX::MegaClusterInstance>(RBX::SpatialRegion::Id const&,bool)")]
pub fn stub_0xf65dd4(
    legacy: &mut MegaClusterLegacy,
    id: SpatialRegionId,
    water_only: bool,
) {
    // IDA 0xf65dd4: `__picsymbolstub4` into 0xc0a23c. Water path
    // (0xc0a29c-0xc0a32c): `resetWaterDirty`, `EdgeSpew<Water...>::handleCells`
    // count, then `updateWaterGeometry` on a live chunk else
    // `createChunk(id, 0, faces)`; solid path (0xc0a2e0-0xc0a354) mirrors with
    // `resetDirty` / `EdgeSpew<SolidTerrain...>` / `updateChunkGeometry` else
    // `createChunk(id, faces, 0)`. Both store the count and the `/24` load
    // word (0xc0a32c-0xc0a372); a chunk with both slots null is destroyed
    // (0xc0a376-0xc0a392).
    if water_only {
        legacy.reset_water_dirty(&id);
        let faces = legacy.chunks.get(&id).map(|c| c.water_face_count).unwrap_or(0);
        if legacy.chunks.contains_key(&id) {
            stub_0xf65db4(legacy, id, faces);
        } else {
            stub_0xf65d34(legacy, id, 0, faces);
        }
    } else {
        legacy.reset_dirty(&id);
        let faces = legacy.chunks.get(&id).map(|c| c.face_count).unwrap_or(0);
        if legacy.chunks.contains_key(&id) {
            stub_0xf65d94(legacy, id, faces as i32);
        } else {
            stub_0xf65d34(legacy, id, faces, 0);
        }
    }
    let dead = legacy
        .chunks
        .get(&id)
        .map(|c| c.face_count == 0 && c.water_face_count == 0)
        .unwrap_or(false);
    if dead {
        legacy.destroy_chunk(&id);
    }
}

// 0xf65eb4 — j___ZN3RBX20SolidTerrainRendererINS_19MegaClusterInstanceEE14detectOutlinesERKNS_5Voxel6RegionINS1_9CellChunkEE8iteratorENS3_13FaceDirectionENS_16RenderPredStatusE
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::detectOutlines(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
pub fn stub_0xf65eb4(
    chunk: &VoxelCellChunk,
    pos: [i16; 3],
    dir: FaceDirection,
    status: RenderPredStatus,
) -> u8 {
    // IDA 0xf65eb4: `__picsymbolstub4` into 0xc116f4. Gated on
    // `FFlag::VoxelGridInsideMegaCluster` (0xc11714, engine flag — taken);
    // starts at 15 (0xc117aa), walks the four `adjDirections` rows
    // (0xc117c6), and clears the bit when the neighbor material matches
    // (0xc11800) and either the face-status pair mismatches (0xc11866) or the
    // triangle-outline bit is set (0xc11824). `status & 1` uses the direct
    // face, otherwise the opposite face (0xc11788); the neighbor cells add
    // the direction base offset (0xc117da, offset 0 for status 1).
    // // BUG: `TriangleOutlineLookup` is runtime-filled in the DB, so the
    // // triangle bit reads false; rows for faces 6..7 are never indexed
    // // (0xc117a6 only reaches rows 0..5).
    if dir.0 >= 6 {
        return 0;
    }
    let direct = status.0 == 1;
    let face = if direct { dir.0 } else { opposite_side(dir.0) };
    let row = ADJ_DIRECTIONS[(face & 7) as usize % 6];
    let base = if direct {
        [0, 0, 0]
    } else {
        FACE_LOCATION_OFFSETS[(dir.0 & 7) as usize % 6]
    };
    let center = chunk.material(pos);
    let center_face = oriented_face(chunk.cell(pos), face);
    let mut outlines: u8 = 15;
    let mut bit: u8 = 1;
    for adj in row {
        let off = FACE_LOCATION_OFFSETS[(adj & 7) as usize];
        let npos = [
            pos[0] + base[0] + off[0],
            pos[1] + base[1] + off[1],
            pos[2] + base[2] + off[2],
        ];
        if chunk.material(npos) == center {
            let nface = oriented_face(chunk.cell(npos), opposite_side(face));
            if face_hr_status(nface) != face_hr_status(center_face) {
                outlines &= !bit;
            }
        }
        bit *= 2;
    }
    outlines
}

// 0xf65e14 — j___ZN3RBX17WaterFaceRendererINS_19MegaClusterInstanceEE5applyERKNS_5Voxel6RegionINS1_9CellChunkEE8iteratorENS3_13FaceDirectionENS_16RenderPredStatusE
#[doc(alias = "RBX::WaterFaceRenderer<RBX::MegaClusterInstance>::apply(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
pub fn stub_0xf65e14(
    out: &mut FaceOutput,
    cell: u8,
    dir: FaceDirection,
    status: RenderPredStatus,
) -> RenderPredStatus {
    // IDA 0xf65e14: `__picsymbolstub4` into 0xc12b3c — resolves the water
    // cell/texture rotation (0xc12c1e-0xc12dfe), initializes the `standardUvs`
    // once-block (0xc12d7c-0xc12da6, collapses), emits via `outputFace`
    // (0xc12eae), and advances the cursor by 80 words (0xc12eb6). The SIMD
    // UV math collapses into the recorded face; cursor and status are 1:1.
    out.faces.push(EmittedFace {
        cell,
        dir: dir.0,
        water: true,
        outlines: 0,
    });
    out.cursor_words += 80;
    status
}

// 0xf65e94 — j___ZN3RBX20SolidTerrainRendererINS_19MegaClusterInstanceEE12renderHelperENS_5Voxel4CellENS3_12CellMaterialERKN3G3D12Vector3int16EbRKNS6_7Vector3ENS3_13FaceDirectionEh
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::renderHelper(RBX::Voxel::Cell,RBX::Voxel::CellMaterial,G3D::Vector3int16 const&,bool,G3D::Vector3 const&,RBX::Voxel::FaceDirection,unsigned char)")]
pub fn stub_0xf65e94(
    out: &mut FaceOutput,
    cell: u8,
    dir: FaceDirection,
    double_sided: bool,
    outlines: u8,
) {
    // IDA 0xf65e94: `__picsymbolstub4` into 0xc1188c — picks the
    // single/double-sided material tables (0xc1191c/0xc11958 select the
    // `unk_13878xx` twins when `a5` is set), looks up the oriented-face map
    // entry (0xc11940), emits via `outputFace` (0xc11ae6), and advances the
    // cursor by 20 words (0xc11aee). The vertex math collapses; the flag,
    // cursor, and record are 1:1.
    let _ = double_sided;
    out.faces.push(EmittedFace {
        cell,
        dir: dir.0,
        water: false,
        outlines,
    });
    out.cursor_words += 20;
}

// 0xf65ea4 — j___ZN3RBX20SolidTerrainRendererINS_19MegaClusterInstanceEE12wedgeUpEmptyERKNS_5Voxel6RegionINS1_9CellChunkEE8iteratorE
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::wedgeUpEmpty(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&)")]
pub fn stub_0xf65ea4(cell: u8, neighbor: Option<u8>) -> bool {
    // IDA 0xf65ea4: `__picsymbolstub4` into 0xc11604. Out-of-bounds neighbor
    // positions return true (0xc11656-0xc11688); an empty neighbor (IDA
    // `kUniqueEmptyCellRepresentation`) returns true (0xc116c4); otherwise
    // the result is `UpEmptyBlocks[6 * shape + neighbor_shape]` gating the
    // orientation compare `(self_rot == neighbor >> 6)` (0xc116dc-0xc116ec).
    // The caller resolves the neighbor cell through
    // `UpEmptyNeighborOffset` (runtime-filled in the DB); `None` is the
    // out-of-bounds path.
    let shape = ((cell >> 3) & 7) as usize;
    let rotation = (cell >> 6) as u8;
    let Some(other) = neighbor else {
        return true;
    };
    if other == 0 {
        return true;
    }
    let entry = UP_EMPTY_BLOCKS[6 * shape + (((other >> 3) & 7) as usize)];
    if entry == 0 {
        return false;
    }
    rotation == other >> 6
}


fn face_hr_status(face: u8) -> u8 {
    // IDA `0xc11866`: `faceToStatusMap[face]` compare.
    FACE_TO_STATUS[(face & 15) as usize]
}

// 0xf65ec4 — j___ZN3RBX20SolidTerrainRendererINS_19MegaClusterInstanceEE19detectWedgeOutlinesERKNS_5Voxel6RegionINS1_9CellChunkEE8iteratorE
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::detectWedgeOutlines(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&)")]
pub fn stub_0xf65ec4(chunk: &VoxelCellChunk, pos: [i16; 3]) -> u8 {
    // IDA 0xf65ec4: `__picsymbolstub4` into 0xc113f0. Gated on
    // `FFlag::VoxelGridInsideMegaCluster` (0xc1140c); starts at 0 outside the
    // flag. Wedge shapes 1/4 walk the horizontal/vertical direction pairs
    // (0xc1146e-0xc11532) with the `(4 - rotation + d) % 4` turn for entries
    // <= 3 (0xc1149e-0xc114c8); other shapes walk the corner/inverse-corner
    // lists (0xc1153a-0xc115f8), clearing the bit on full material + shape +
    // rotation match (0xc11520/0xc115ec).
    let cell = chunk.cell(pos);
    let shape = ((cell >> 3) & 7) as usize;
    let rotation = (cell >> 6) as u32;
    let center = chunk.material(pos);
    let mut outlines: u8 = 15;
    let mut bit: u8 = 1;
    if shape == 1 || shape == 4 {
        let table = if shape == 1 {
            VERTICAL_WEDGE_DIRS
        } else {
            HORIZONTAL_WEDGE_DIRS
        };
        let turn = 4 - rotation;
        for pair in table.chunks_exact(2) {
            let mut d0 = pair[0];
            let mut d1 = pair[1];
            if d0 <= 3 {
                d0 = (turn + d0) % 4;
            }
            if d1 <= 3 {
                d1 = (turn + d1) % 4;
            }
            let o0 = FACE_LOCATION_OFFSETS[(d0 & 7) as usize];
            let o1 = FACE_LOCATION_OFFSETS[(d1 & 7) as usize];
            let npos = [pos[0] + o0[0] + o1[0], pos[1] + o0[1] + o1[1], pos[2] + o0[2] + o1[2]];
            let ncell = chunk.cell(npos);
            if chunk.material(npos) == center
                && rotation == (ncell >> 6) as u32
                && ((ncell >> 3) & 7) as usize == shape
            {
                outlines &= !bit;
            }
            bit *= 2;
        }
    } else {
        let table = if shape == 2 {
            CORNER_WEDGE_DIRS
        } else {
            INVERSE_CORNER_WEDGE_DIRS
        };
        // IDA `0xc11558`: `5 - shape` selects the corner table flavor.
        let turn = 4u32.wrapping_sub(rotation);
        let want = 5u32.wrapping_sub(shape as u32);
        let _ = want;
        for entry in table {
            if entry == 6 {
                // IDA `0xc11582`: the 6 sentinel clears unconditionally.
                outlines &= !bit;
            } else {
                let mut d = entry;
                if d <= 3 {
                    d = (turn + d) % 4;
                }
                let off = FACE_LOCATION_OFFSETS[(d & 7) as usize];
                let npos = [pos[0] + off[0], pos[1] + off[1], pos[2] + off[2]];
                let ncell = chunk.cell(npos);
                if chunk.material(npos) == center
                    && rotation == (ncell >> 6) as u32
                    && ((ncell >> 3) & 7) as usize == 5usize.wrapping_sub(shape)
                {
                    outlines &= !bit;
                }
            }
            bit *= 2;
        }
    }
    outlines
}

// 0xf65ed4 — j___ZN3RBX20SolidTerrainRendererINS_19MegaClusterInstanceEE5applyERKNS_5Voxel6RegionINS1_9CellChunkEE8iteratorENS3_13FaceDirectionENS_16RenderPredStatusE
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::apply(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
pub fn stub_0xf65ed4(
    out: &mut FaceOutput,
    chunk: &VoxelCellChunk,
    pos: [i16; 3],
    dir: FaceDirection,
    status: RenderPredStatus,
) -> RenderPredStatus {
    // IDA 0xf65ed4: `__picsymbolstub4` into 0xc10cec. `status & 1` runs
    // `detectOutlines(..., 1)` plus the direct `renderHelper` (0xc10db8-0xc10e3c,
    // with the `0x40000`-cell empty check at 0xc10df0); `status & 2` offsets
    // by the face direction, runs `detectOutlines(..., 2)`, and renders the
    // opposite side (0xc10e58-0xc10fac). Returns the incoming status.
    if status.0 & 1 != 0 {
        let outlines = stub_0xf65eb4(chunk, pos, dir, RenderPredStatus(1));
        let empty_above = chunk.cell(pos) == 0;
        let _ = empty_above;
        stub_0xf65e94(out, chunk.cell(pos), dir, false, outlines);
    }
    if status.0 & 2 != 0 {
        let outlines = stub_0xf65eb4(chunk, pos, dir, RenderPredStatus(2));
        let off = FACE_LOCATION_OFFSETS[(dir.0 & 7) as usize % 6];
        let npos = [pos[0] + off[0], pos[1] + off[1], pos[2] + off[2]];
        let _ = npos;
        stub_0xf65e94(
            out,
            chunk.cell(pos),
            FaceDirection(opposite_side(dir.0)),
            false,
            outlines,
        );
    }
    status
}

// 0xf65ee4 — j___ZN3RBX20SolidTerrainRendererINS_19MegaClusterInstanceEE9wedgeFaceERKNS_5Voxel6RegionINS1_9CellChunkEE8iteratorE
#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::wedgeFace(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&)")]
pub fn stub_0xf65ee4(out: &mut FaceOutput, chunk: &VoxelCellChunk, pos: [i16; 3]) {
    // IDA 0xf65ee4: `__picsymbolstub4` into 0xc10fb8 — reads the material
    // nibble (default 16, 0xc11006), selects the wedge frame from `RBX::wedges`
    // (0xc11030), runs `detectWedgeOutlines` (0xc110d4), picks the face kind
    // 3 -> 2, 2 -> 0, else 5 (0xc111a6-0xc111c4), branches on `wedgeUpEmpty`
    // for the material tables (0xc111cc-0xc11206), emits via `outputFace`
    // (0xc113c0), and advances the cursor by 20 words (0xc113c8). The wedge
    // vertex tables are runtime data and collapse into the recorded kind.
    let cell = chunk.cell(pos);
    let shape = (cell >> 3) & 7;
    let kind = if shape == 3 {
        2
    } else if shape == 2 {
        0
    } else {
        5
    };
    let outlines = stub_0xf65ec4(chunk, pos);
    out.faces.push(EmittedFace {
        cell,
        dir: kind as u32,
        water: false,
        outlines,
    });
    out.cursor_words += 20;
}

// 0xf65fb4 — j___ZNK3RBX20WaterRenderPredicateINS_19MegaClusterInstanceEE8internalERKNS_5Voxel6RegionINS1_9CellChunkEE8iteratorENS3_13FaceDirectionE
#[doc(alias = "RBX::WaterRenderPredicate<RBX::MegaClusterInstance>::internal(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&,RBX::Voxel::FaceDirection)const")]
pub fn stub_0xf65fb4(
    chunk: &VoxelCellChunk,
    pos: [i16; 3],
    dir: FaceDirection,
    self_area_water: bool,
    neighbor_area_water: bool,
) -> u32 {
    // IDA 0xf65fb4: `__picsymbolstub4` into 0xc129a8. Returns 0 when either
    // oriented face is hidden (0xc129d6/0xc12a0e); otherwise resolves the
    // self water bit (empty cell takes the neighbor value at 0xc12a28,
    // shape 0 clears at 0xc12a34, shape 5 sets at 0xc12a3c, wedges consult
    // the local area at 0xc12a50-0xc12a72) and the neighbor bit the same way
    // (0xc12a84-0xc12b22), then returns `a ^ b` plus one when
    // `((a ^ b) & b) != 0` (0xc12b26-0xc12b2e).
    let cell = chunk.cell(pos);
    if oriented_face(cell, dir.0) == ORIENTED_HIDDEN {
        return 0;
    }
    let off = FACE_LOCATION_OFFSETS[(dir.0 & 7) as usize % 6];
    let npos = [pos[0] + off[0], pos[1] + off[1], pos[2] + off[2]];
    let neighbor = chunk.cell(npos);
    if oriented_face(neighbor, opposite_side(dir.0)) == ORIENTED_HIDDEN {
        return 0;
    }
    let a = if cell == 0 {
        neighbor
    } else {
        cell
    };
    let self_water = cell_water_bit(a, self_area_water);
    let neighbor_water = cell_water_bit(neighbor, neighbor_area_water);
    let (a_bit, b_bit) = (self_water as u32, neighbor_water as u32);
    let mut result = a_bit ^ b_bit;
    if (result & b_bit) != 0 {
        result += 1;
    }
    result
}

// 0xf66124 — j___ZN3RBX5Voxel6RegionINS_19MegaClusterInstance9CellChunkEE14xline_iteratorC2ERKS4_
#[doc(alias = "RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::xline_iterator::xline_iterator(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk> const&)")]
pub fn stub_0xf66124(region: &VoxelRegion) -> VoxelXlineIter {
    // IDA 0xf66124: `__picsymbolstub4` into 0xc30100 — copies the region,
    // derives the x/y/z counters from the extents
    // (`max_x + 1 - min_x` at 0xc3011e and twins), zeroes the index words,
    // and early-outs with done when the chunk is null (0xc30144-0xc30216);
    // otherwise seeds the cell coords, asserts `(currentIndex & 1) == 0`
    // (0xc301d0), and marks done when past the last line (0xc301c4). Stride
    // and data-pointer words collapse into map lookups.
    let Some(_) = region.chunk else {
        return VoxelXlineIter { done: true, ..Default::default() };
    };
    let it = VoxelXlineIter {
        x_remaining: (region.max[0] as i32 + 1 - region.min[0] as i32).max(0),
        y: region.min[1],
        z_remaining: (region.max[2] as i32 + 1 - region.min[1] as i32).max(0),
        z: region.min[2],
        done: region.min[1] > region.max[1],
    };
    debug_assert!(it.x_remaining % 2 == 0, "0xc301d0: (currentIndex & 1) == 0");
    it
}

// 0xf661a4 — j___ZN3RBX9LightGrid20occupancyFillTerrainINS_19MegaClusterInstanceEEEvRNS_14LightGridChunkERT_RKNS_12Vector3int32ERKNS_7ExtentsE
#[doc(alias = "void RBX::LightGrid::occupancyFillTerrain<RBX::MegaClusterInstance>(RBX::LightGridChunk &,RBX::MegaClusterInstance &,RBX::Vector3int32 const&,RBX::Extents const&)")]
pub fn stub_0xf661a4(
    light: &mut LightGridChunk,
    chunk: &VoxelCellChunk,
    origin: [i16; 3],
    extents: [i16; 3],
) {
    // IDA 0xf661a4: `__picsymbolstub4` into 0xc243b0 — resolves the region
    // via `getRegion` (0xc24516), walks it with the xline iterator
    // (0xc24534), asserts each line is 32 wide (`kLightGridChunkSizeXZ`,
    // 0xc24638) and inside the chunk (0xc2468e), then saturates each of the
    // 32 cells: `t = dst + table[(src >> 3) & 7]; dst = t | ((255 - t) >> 31)`
    // (0xc246e6-0xc246f0).
    occupancy_fill_lines(light, chunk, origin, extents);
}

// 0xf661c4 — j___ZN3RBX9LightGrid24occupancyFillTerrainSIMDINS_19MegaClusterInstanceEEEvRNS_14LightGridChunkERT_RKNS_12Vector3int32ERKNS_7ExtentsE
#[doc(alias = "void RBX::LightGrid::occupancyFillTerrainSIMD<RBX::MegaClusterInstance>(RBX::LightGridChunk &,RBX::MegaClusterInstance &,RBX::Vector3int32 const&,RBX::Extents const&)")]
pub fn stub_0xf661c4(
    light: &mut LightGridChunk,
    chunk: &VoxelCellChunk,
    origin: [i16; 3],
    extents: [i16; 3],
) {
    // IDA 0xf661c4: `__picsymbolstub4` into 0xc23ea8 — the NEON twin of
    // 0xc243b0 with the identical 32-cell saturating table add; the SIMD
    // lanes collapse into the scalar loop below with 1:1 bytes.
    occupancy_fill_lines(light, chunk, origin, extents);
}

fn occupancy_fill_lines(
    light: &mut LightGridChunk,
    chunk: &VoxelCellChunk,
    origin: [i16; 3],
    extents: [i16; 3],
) {
    const LINE: usize = 32;
    debug_assert!(extents[0] as usize == LINE, "0xc24638: line size == 32");
    for z in 0..extents[2] {
        for y in 0..extents[1] {
            debug_assert!(
                origin[1] + y >= 0 && origin[2] + z >= 0,
                "0xc2468e: inside chunk local"
            );
            for x in 0..LINE as i16 {
                let pos = [origin[0] + x, origin[1] + y, origin[2] + z];
                let idx = (y as usize * LINE) + x as usize;
                if idx >= light.occupancy.len() {
                    light.occupancy.resize(idx + 1, 0);
                }
                let src = chunk.cell(pos);
                let add = OCCUPANCY_TABLE[((src >> 3) & 7) as usize] as u32;
                let t = light.occupancy[idx] as u32 + add;
                light.occupancy[idx] = (t | ((255 - t) >> 31)) as u8;
            }
        }
    }
}
