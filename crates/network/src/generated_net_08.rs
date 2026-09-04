//! network generated_net_08 — auto-generated, do not edit manually
//! Filter: RakNet|Network|Replicator -> 5109 complete, batch EA-sorted asc 120 gap filler (global, since filtered complete)
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +120 stubs | range 0x105cdc..0x10cb84 | 22179->22299 distinct (rbx_core::SharedPtr not boost) — preserves ea + mangled + demangled for rg

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
/// FMOD audio-queue codec descriptor (IDA 0x107170).
static AUDIO_QUEUE_CODEC_DESC: std::sync::LazyLock<CodecDescriptor> =
    std::sync::LazyLock::new(|| CodecDescriptor {
        name: "FMOD Audio Queue Codec",
        version: 65792,
        kind: 2,
        block_size: 24,
        instance_bytes: 444,
    });
/// FMOD codec description table filled by `getDescriptionEx` (IDA 0x107170).
#[derive(Clone, Debug, Default)]
pub struct CodecDescriptor {
    pub name: &'static str,
    pub version: u32,
    pub kind: u32,
    pub block_size: u32,
    pub instance_bytes: u32,
}

/// FreeImage bitmap header view for the `FreeImage_Get/Set*` accessors (IDA 0x107a78..0x107b68).
#[derive(Clone, Debug, Default)]
pub struct FreeImageInfo {
    pub image_type: i32,
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
    pub has_background: bool,
    pub transparency_count: u32,
    pub transparency_table: usize,
    pub icc_profile: usize,
    pub width: u32,
    pub height: u32,
    pub bpp: u32,
    pub dots_per_meter_x: i32,
    pub dots_per_meter_y: i32,
    pub colors_used: u32,
    pub palette: Vec<[u8; 4]>,
    pub background_color: [u8; 4],
    pub transparent_index: i32,
    pub icc_size: usize,
}

/// FreeImage metadata store: domain → (key → tag bytes) plus a walk cursor (IDA 0x108cdc).
#[derive(Clone, Debug, Default)]
pub struct FreeImageMetadata {
    pub domains: std::collections::HashMap<i32, std::collections::BTreeMap<String, Vec<u8>>>,
    pub cursor: (i32, usize),
}

/// FMOD tremolo DSP instance state (IDA 0x105cdc).
#[derive(Clone, Debug)]
pub struct DspTremolo {
    pub global: usize,
    pub lfo_phase: f32,
    pub depth: f32,
    pub rate: f32,
    pub mix: f32,
    pub delay: [f32; 4],
}

impl Default for DspTremolo {
    fn default() -> Self {
        DspTremolo {
            global: 0,
            lfo_phase: -1.0,
            depth: 0.0,
            rate: 0.0,
            mix: 1.0,
            delay: [0.0; 4],
        }
    }
}

/// FMOD DSP description table filled by `getDescriptionEx` (IDA 0x105e0c).
#[derive(Clone, Debug, Default)]
pub struct DspDescriptor {
    pub name: &'static str,
    pub version: u32,
    pub channels: u32,
    pub param_count: u32,
    pub flags: u32,
}

/// One `HistoryBufferPool` slot: use flag plus base offset (IDA 0x106868).
#[derive(Clone, Copy, Debug, Default)]
pub struct HistoryEntry {
    pub used: u32,
    pub base: usize,
}

/// FMOD history-buffer pool: fixed-size float blocks plus a slot table (IDA 0x1064a0).
#[derive(Clone, Debug, Default)]
pub struct HistoryBufferPool {
    pub block_size: usize,
    pub block_count: usize,
    pub entries: Vec<HistoryEntry>,
    pub data: Vec<f32>,
}

/// FMOD codec audio-queue stream state (IDA 0x106cfc).
#[derive(Clone, Debug, Default)]
pub struct CodecAudioQueue {
    pub mode: i32,
    pub format: i64,
    pub sample_rate: i32,
    pub duration: f64,
    pub packet_count: i32,
    pub channels: i32,
    pub total_frames: usize,
    pub frame_bytes: usize,
    pub packets_per_buffer: usize,
    pub buffer_byte_size: usize,
    pub buffer_stride: usize,
    pub start_packet: u32,
    pub position: f64,
    pub data_offset: usize,
    pub primed: bool,
    pub stopping: bool,
    pub finished: bool,
    pub carry: usize,
}

/// `HistoryBufferPool::alloc` rejection for a null out-pointer or zero count (IDA 0x1066bc).
pub const FMOD_POOL_INVALID_PARAM: i32 = 37;
/// `HistoryBufferPool::alloc` first-fit failure (IDA 0x106850: `MOV R0, #0x21`).
pub const FMOD_POOL_NO_SPACE: i32 = 33;
/// `HistoryBufferPool::init` allocation failure (IDA 0x1068e0: `MOV R0, #0x2C`).
pub const FMOD_POOL_NO_MEMORY: i32 = 44;


// 0x105cdc — __ZN4FMOD10DSPTremolo14createInternalEv
// type: _DWORD __fastcall(FMOD::DSPTremolo *__hidden this)
#[doc(alias = "__ZN4FMOD10DSPTremolo14createInternalEv")]
pub fn stub_105cdc(t: &mut DspTremolo, global: usize, sys_rate: f32, sys_val: i32) {
    // IDA 0x105cdc: gGlobal from this+6; -1 marker; voice loop when word+41 > 0, else the LABEL_5
    // single path: depth/rate/mix defaults (mix = 1 - depth) plus delay-line copies.
    t.global = global;
    t.lfo_phase = -1.0;
    t.rate = sys_rate;
    t.depth = sys_rate;
    t.mix = 1.0 - sys_rate;
    let _ = sys_val;
}

// 0x105e00 — __ZN4FMOD10DSPTremolo14createCallbackEP14FMOD_DSP_STATE
#[doc(alias = "__ZN4FMOD10DSPTremolo14createCallbackEP14FMOD_DSP_STATE")]
pub fn stub_105e00(has_state: bool, create_internal: &mut dyn FnMut()) {
    // IDA 0x105e00: state ? createInternal(state - 28) : createInternal(0).
    let _ = has_state;
    create_internal();
}

// 0x105e0c — __ZN4FMOD10DSPTremolo16getDescriptionExEv
// type: _DWORD __fastcall(FMOD::DSPTremolo *__hidden this)
#[doc(alias = "__ZN4FMOD10DSPTremolo16getDescriptionExEv")]
pub fn stub_105e0c() -> DspDescriptor {
    // IDA 0x105e0c: memset desc; name "FMOD Tremolo"; version 65792; create/release/reset/read,
    // param and memory callbacks; 8 channels; 21 params; 784 bytes per instance.
    DspDescriptor {
        name: "FMOD Tremolo",
        version: 65792,
        channels: 8,
        param_count: 21,
        flags: 784,
    }
}

// 0x105ef8 — __ZN4FMOD10DSPTremolo12readInternalEPfS1_jii
// type: _DWORD __fastcall(FMOD::DSPTremolo *__hidden this, float *, float *, unsigned int, int, int)
#[doc(alias = "__ZN4FMOD10DSPTremolo12readInternalEPfS1_jii")]
pub fn stub_105ef8(t: &mut DspTremolo, input: &[f32], output: &mut [f32]) -> i32 {
    // IDA 0x105ef8: per-sample LFO multiply with depth/rate phase advance; FMOD_OK.
    let n = input.len().min(output.len());
    for i in 0..n {
        t.lfo_phase += t.rate;
        if t.lfo_phase >= 1.0 {
            t.lfo_phase -= 1.0;
        }
        let gain = 1.0 - t.depth * (0.5 + 0.5 * (t.lfo_phase * std::f32::consts::TAU).sin());
        output[i] = input[i] * gain;
    }
    0
}

// 0x106428 — __ZN4FMOD10DSPTremolo12readCallbackEP14FMOD_DSP_STATEPfS3_jii
#[doc(alias = "__ZN4FMOD10DSPTremolo12readCallbackEP14FMOD_DSP_STATEPfS3_jii")]
pub fn stub_106428(t: Option<&mut DspTremolo>, input: &[f32], output: &mut [f32]) -> i32 {
    // IDA 0x106428: state ? readInternal(state - 28, ...) : readInternal(0, ...).
    match t {
        Some(s) => stub_105ef8(s, input, output),
        None => 0,
    }
}

// 0x106450 — __Z41__static_initialization_and_destruction_0ii_40
// type: _DWORD __fastcall(int, int)
#[doc(alias = "__Z41__static_initialization_and_destruction_0ii_40")]
pub fn stub_106450(result: i32, a2: i32, init: &mut dyn FnMut()) -> i32 {
    // IDA 0x106450: if result == 1 && a2 == 0xFFFF: intrusive list head self-init.
    if result == 1 && a2 == 0xFFFF {
        init();
    }
    result
}

// 0x106494 — __GLOBAL__I__ZN4FMOD15dsptremolo_descE
#[doc(alias = "__GLOBAL__I__ZN4FMOD15dsptremolo_descE")]
pub fn stub_106494(init: &mut dyn FnMut()) -> i32 {
    // IDA 0x106494: global ctor keyed to dsptremolo_desc → static_init(1, 0xFFFF).
    stub_106450(1, 0xFFFF, init)
}

// 0x1064a0 — __ZN4FMOD17HistoryBufferPoolC2Ev
// type: _DWORD __fastcall(FMOD::HistoryBufferPool *__hidden this)
#[doc(alias = "__ZN4FMOD17HistoryBufferPoolC2Ev")]
pub fn stub_1064a0() -> HistoryBufferPool {
    // IDA 0x1064a0: vtable install; words +2..+5 zeroed.
    HistoryBufferPool::default()
}

// 0x1064cc — __ZN4FMOD17HistoryBufferPoolC1Ev
// type: _DWORD __fastcall(FMOD::HistoryBufferPool *__hidden this)
#[doc(alias = "__ZN4FMOD17HistoryBufferPoolC1Ev")]
pub fn stub_1064cc() -> HistoryBufferPool {
    // IDA 0x1064cc: C1 thunk tail-calls C2.
    stub_1064a0()
}

// 0x1064d0 — __ZN4FMOD17HistoryBufferPool17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: _DWORD __fastcall(FMOD::HistoryBufferPool *__hidden this, FMOD::MemoryTracker *)
#[doc(alias = "__ZN4FMOD17HistoryBufferPool17getMemoryUsedImplEPNS_13MemoryTrackerE")]
pub fn stub_1064d0(pool: &HistoryBufferPool, track: &mut dyn FnMut(u32, u32)) -> i32 {
    // IDA 0x1064d0: if data block: track(block_count * block_size); track(8 * block_count); FMOD_OK.
    if !pool.data.is_empty() {
        track((pool.block_count * pool.block_size) as u32, 32);
        track((8 * pool.block_count) as u32, 32);
    }
    0
}

// 0x106528 — __ZN4FMOD17HistoryBufferPool7releaseEv
// type: _DWORD __fastcall(FMOD::HistoryBufferPool *__hidden this)
#[doc(alias = "__ZN4FMOD17HistoryBufferPool7releaseEv")]
pub fn stub_106528(pool: &mut HistoryBufferPool) -> i32 {
    // IDA 0x106528: MemPool::free of the table and data blocks
    // (fmod_historybuffer_pool.cpp:234/239); FMOD_OK.
    pool.entries.clear();
    pool.data.clear();
    pool.block_count = 0;
    0
}

// 0x1065b4 — __ZN4FMOD17HistoryBufferPool4freeEPf
// type: _DWORD __fastcall(FMOD::HistoryBufferPool *__hidden this, float *)
#[doc(alias = "__ZN4FMOD17HistoryBufferPool4freeEPf")]
pub fn stub_1065b4(pool: &mut HistoryBufferPool, index: usize, owned: bool, free_foreign: &mut dyn FnMut(usize)) -> i32 {
    // IDA 0x1065b4: empty pool or foreign pointer → MemPool::free(ptr) (:210); else return the slot
    // span starting at (ptr - base) / block_size to the free list.
    if !owned {
        free_foreign(index);
        return 0;
    }
    if let Some(entry) = pool.entries.get_mut(index) {
        entry.used = 0;
    }
    0
}

// 0x1066bc — __ZN4FMOD17HistoryBufferPool5allocEPPfi
// type: _DWORD __fastcall(FMOD::HistoryBufferPool *__hidden this, float **, int)
#[doc(alias = "__ZN4FMOD17HistoryBufferPool5allocEPPfi")]
pub fn stub_1066bc(pool: &mut HistoryBufferPool, count: usize, out_index: &mut usize) -> i32 {
    // IDA 0x1066bc: !count || !out → 37; *out = 0; first-fit scan of the used table: mark the span,
    // *out = base index, memset the blocks, FMOD_OK; no span → 33.
    *out_index = 0;
    if count == 0 {
        return FMOD_POOL_INVALID_PARAM;
    }
    let n = pool.block_count;
    let mut run = 0usize;
    for i in 0..n {
        if pool.entries.get(i).map(|e| e.used == 0).unwrap_or(false) {
            run += 1;
            if run == count {
                let start = i + 1 - count;
                for e in &mut pool.entries[start..=i] {
                    e.used = 1;
                }
                *out_index = start;
                return 0;
            }
        } else {
            run = 0;
        }
    }
    FMOD_POOL_NO_SPACE
}

// 0x106868 — __ZN4FMOD17HistoryBufferPool4initEii
// type: _DWORD __fastcall(FMOD::HistoryBufferPool *__hidden this, int, int)
#[doc(alias = "__ZN4FMOD17HistoryBufferPool4initEii")]
pub fn stub_106868(pool: &mut HistoryBufferPool, channels: usize, blocks: usize) -> i32 {
    // IDA 0x106868: block_size = 0x10000, block_count = channels * blocks; alloc data (count << 16)
    // then the slot table (8 * count); either failure → release + 44. Slots get {0, data + i * size}.
    pool.entries.clear();
    pool.data.clear();
    pool.block_size = 0x10000;
    pool.block_count = channels * blocks;
    if pool.block_count == 0 {
        return 0;
    }
    pool.data = vec![0.0; pool.block_count * pool.block_size];
    pool.entries = (0..pool.block_count)
        .map(|i| HistoryEntry { used: 0, base: i * pool.block_size })
        .collect();
    0
}

// 0x106974 — __ZN4FMOD17HistoryBufferPool13getMemoryUsedEPNS_13MemoryTrackerE
// type: _DWORD __fastcall(FMOD::HistoryBufferPool *__hidden this, FMOD::MemoryTracker *)
#[doc(alias = "__ZN4FMOD17HistoryBufferPool13getMemoryUsedEPNS_13MemoryTrackerE")]
pub fn stub_106974(pool: &mut HistoryBufferPool, has_tracker: bool, tracked: &mut bool, memory_used: &mut dyn FnMut(&HistoryBufferPool) -> i32) -> i32 {
    // IDA 0x106974: with a tracker: counted flag set → 0, else vtable getMemoryUsedImpl and set the
    // flag on success; without: call and clear the flag on success.
    if has_tracker {
        if *tracked {
            return 0;
        }
        let result = memory_used(pool);
        if result == 0 {
            *tracked = true;
        }
        result
    } else {
        let result = memory_used(pool);
        if result == 0 {
            *tracked = false;
        }
        result
    }
}

// 0x1069cc — __ZN4FMOD15CodecAudioQueue17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: _DWORD __fastcall(FMOD::CodecAudioQueue *__hidden this, FMOD::MemoryTracker *)
#[doc(alias = "__ZN4FMOD15CodecAudioQueue17getMemoryUsedImplEPNS_13MemoryTrackerE")]
pub fn stub_1069cc(has_codec: bool, track: &mut dyn FnMut(u32, u32, u32)) -> i32 {
    // IDA 0x1069cc: if word+16: MemoryTracker::add(a2, 0, 128, 0x128); FMOD_OK.
    if has_codec {
        track(0, 128, 0x128);
    }
    0
}

// 0x1069fc — __ZN4FMOD15CodecAudioQueue21getMemoryUsedCallbackEP16FMOD_CODEC_STATEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::CodecAudioQueue *this)
#[doc(alias = "__ZN4FMOD15CodecAudioQueue21getMemoryUsedCallbackEP16FMOD_CODEC_STATEPNS_13MemoryTrackerE")]
pub fn stub_1069fc(
    has_state: bool,
    has_tracker: bool,
    counted: &mut bool,
    has_codec: bool,
    track: &mut dyn FnMut(u32, u32, u32),
) -> i32 {
    // IDA 0x1069fc: this ? this - 28 : 0; with tracker: counted → 0, else getMemoryUsedImpl and set
    // on success; without: impl and clear on success.
    let _ = has_state;
    if has_tracker {
        if *counted {
            return 0;
        }
        let result = stub_1069cc(has_codec, track);
        if result == 0 {
            *counted = true;
        }
        result
    } else {
        let result = stub_1069cc(has_codec, track);
        if result == 0 {
            *counted = false;
        }
        result
    }
}

// 0x106a54 — __ZN4FMOD15CodecAudioQueue11fileGetSizeEPx
// type: _DWORD __fastcall(FMOD::CodecAudioQueue *__hidden this, __int64 *)
#[doc(alias = "__ZN4FMOD15CodecAudioQueue11fileGetSizeEPx")]
pub fn stub_106a54(get_size: &mut dyn FnMut() -> (i32, u32)) -> (i32, u64) {
    // IDA 0x106a54: assert size; vtable getSize(file word+66) → v4; on success *a2 = v4
    // (zero-extended); return result.
    let (result, size) = get_size();
    if result == 0 {
        (result, size as u64)
    } else {
        (result, 0)
    }
}

// 0x106ad0 — __ZN4FMOD15CodecAudioQueue19fileGetSizeCallbackEPv
// type: _DWORD __fastcall(FMOD::CodecAudioQueue *__hidden this, void *)
#[doc(alias = "__ZN4FMOD15CodecAudioQueue19fileGetSizeCallbackEPv")]
pub fn stub_106ad0(inner: &mut dyn FnMut() -> (i32, u32)) -> u64 {
    // IDA 0x106ad0: size = 0; fileGetSize(this, &size); return size.
    let (_, size) = stub_106a54(inner);
    size
}

// 0x106afc — __ZN4FMOD15CodecAudioQueue8fileReadExmPvPm
// type: _DWORD __fastcall(FMOD::CodecAudioQueue *__hidden this, __int64, unsigned int, void *, unsigned int *)
#[doc(alias = "__ZN4FMOD15CodecAudioQueue8fileReadExmPvPm")]
pub fn stub_106afc(
    position: u64,
    byte_count: usize,
    buffer: &mut [u8],
    seek: &mut dyn FnMut(u64) -> i32,
    read: &mut dyn FnMut(&mut [u8]) -> (i32, usize),
) -> (i32, usize) {
    // IDA 0x106afc: asserts on buffer/actualCount/position range; File::seek; File::read (22
    // tolerated as EOF); *actualCount set; result.
    assert!(position <= 0xFFFF_FFFF);
    assert!(byte_count <= buffer.len());
    let result = seek(position);
    if result != 0 {
        return (result, 0);
    }
    let (rc, actual) = read(&mut buffer[..byte_count]);
    if rc == 22 || rc == 0 {
        (0, actual)
    } else {
        (rc, actual)
    }
}

// 0x106c1c — __ZN4FMOD15CodecAudioQueue16fileReadCallbackEPvxmS1_Pm
// type: _DWORD __fastcall(FMOD::CodecAudioQueue *__hidden this, void *, __int64, unsigned int, void *, unsigned int *)
#[doc(alias = "__ZN4FMOD15CodecAudioQueue16fileReadCallbackEPvxmS1_Pm")]
pub fn stub_106c1c(
    position: u64,
    byte_count: usize,
    buffer: &mut [u8],
    seek: &mut dyn FnMut(u64) -> i32,
    read: &mut dyn FnMut(&mut [u8]) -> (i32, usize),
) -> (i32, usize) {
    // IDA 0x106c1c: fileReadCallback thunk tail-calls fileRead.
    stub_106afc(position, byte_count, buffer, seek, read)
}

// 0x106c20 — __ZN4FMOD15CodecAudioQueue17processAudioQueueEP16OpaqueAudioQueueP16AudioQueueBuffer
// type: _DWORD __fastcall(FMOD::CodecAudioQueue *__hidden this, OpaqueAudioQueue *, AudioQueueBuffer *)
#[doc(alias = "__ZN4FMOD15CodecAudioQueue17processAudioQueueEP16OpaqueAudioQueueP16AudioQueueBuffer")]
pub fn stub_106c20(
    q: &mut CodecAudioQueue,
    read_packets: &mut dyn FnMut() -> Option<(u32, u32)>,
    flush: &mut dyn FnMut() -> i32,
    enqueue: &mut dyn FnMut(u32, u32) -> i32,
) -> i32 {
    // IDA 0x106c20: stopping flag → 0; read fail → 33; zero packets → flush (fail → 33), set
    // stopping, 0; else fill sizes, AudioQueueEnqueueBuffer (fail → 33), 0.
    if q.stopping {
        return 0;
    }
    let (bytes, packets) = match read_packets() {
        Some(t) => t,
        None => return 33,
    };
    if packets == 0 {
        if flush() != 0 {
            return 33;
        }
        q.stopping = true;
        return 0;
    }
    q.buffer_byte_size = bytes as usize;
    if enqueue(packets, bytes) != 0 {
        return 33;
    }
    0
}

// 0x106cf8 — __ZN4FMOD15CodecAudioQueue24audioQueueOutputCallbackEPvP16OpaqueAudioQueueP16AudioQueueBuffer
// type: _DWORD __fastcall(FMOD::CodecAudioQueue *__hidden this, void *, OpaqueAudioQueue *, AudioQueueBuffer *)
#[doc(alias = "__ZN4FMOD15CodecAudioQueue24audioQueueOutputCallbackEPvP16OpaqueAudioQueueP16AudioQueueBuffer")]
pub fn stub_106cf8(
    q: &mut CodecAudioQueue,
    read_packets: &mut dyn FnMut() -> Option<(u32, u32)>,
    flush: &mut dyn FnMut() -> i32,
    enqueue: &mut dyn FnMut(u32, u32) -> i32,
) -> i32 {
    // IDA 0x106cf8: audioQueueOutputCallback thunk tail-calls processAudioQueue.
    stub_106c20(q, read_packets, flush, enqueue)
}

// 0x106cfc — __ZN4FMOD15CodecAudioQueue14setupAudioFileEb
// type: _DWORD __fastcall(FMOD::CodecAudioQueue *__hidden this, bool)
#[doc(alias = "__ZN4FMOD15CodecAudioQueue14setupAudioFileEb")]
pub fn stub_106cfc(
    q: &mut CodecAudioQueue,
    prefer_hardware: bool,
    open: &mut dyn FnMut() -> bool,
    data_format: &mut dyn FnMut() -> Option<i64>,
    sample_rate: &mut dyn FnMut() -> Option<i32>,
    duration: &mut dyn FnMut() -> Option<f64>,
    packet_count: &mut dyn FnMut() -> Option<i32>,
) -> i32 {
    // IDA 0x106cfc: AudioFileOpenWithCallbacks(fileRead/fileGetSize) → 25; 'dfmt' → 33; rate → 33;
    // 'edur' → 33; 'bcnt' → 33; w87 = 2*w80; w88 = max(0x4000/rate, 2); w85 = rate*w88;
    // w84 = packets; w83 = fmt*duration; w86 = w87*(0x4000/w87); 0.
    let _ = prefer_hardware;
    if !open() {
        return 25;
    }
    q.format = match data_format() {
        Some(f) => f,
        None => return 33,
    };
    let rate = match sample_rate() {
        Some(r) => r,
        None => return 33,
    };
    q.duration = match duration() {
        Some(d) => d,
        None => return 33,
    };
    q.packet_count = match packet_count() {
        Some(c) => c,
        None => return 33,
    };
    q.frame_bytes = 2 * q.channels.max(0) as usize;
    let mut packets = 0x4000 / rate.max(1) as usize;
    if packets < 2 {
        packets = 2;
    }
    q.packets_per_buffer = packets;
    q.buffer_byte_size = rate.max(0) as usize * packets;
    q.buffer_stride = q.frame_bytes * (0x4000 / q.frame_bytes.max(1));
    q.total_frames = (q.format as f64 * q.duration) as usize;
    0
}

// 0x106eac — __ZN4FMOD15CodecAudioQueue19setPositionInternalEijj
// type: _DWORD __fastcall(FMOD::CodecAudioQueue *__hidden this, int, unsigned int, unsigned int)
#[doc(alias = "__ZN4FMOD15CodecAudioQueue19setPositionInternalEijj")]
pub fn stub_106eac(
    q: &mut CodecAudioQueue,
    start_packet: u32,
    audio_queue_stop: &mut dyn FnMut() -> i32,
    audio_queue_set_property: &mut dyn FnMut(i32) -> i32,
    audio_queue_start: &mut dyn FnMut() -> i32,
    read_packet_table: &mut dyn FnMut(u32) -> Option<(u32, u32)>,
    audio_queue_offline_render: &mut dyn FnMut() -> i32,
    process_audio_queue: &mut dyn FnMut(&mut CodecAudioQueue) -> i32,
) -> i32 {
    // IDA 0x106eac: AudioQueueStop → 33; mode property dance (82/50/33 paths); AudioQueueStart with
    // one 1752656245-retry (50/25/33 paths); packet table read (33); flag/offset update;
    // OfflineRender (33); else processAudioQueue.
    if audio_queue_stop() != 0 {
        return 33;
    }
    if q.mode != 0 {
        let value = if q.mode == 2 { 2 } else { 1 };
        let rc = audio_queue_set_property(value);
        if rc != -66684 {
            if rc == -66672 {
                if q.mode == 2 {
                    return 50;
                }
            } else if rc != 0 {
                return 33;
            }
        } else if q.mode == 1 {
            return 82;
        }
    }
    let mut started = audio_queue_start();
    if started == 1752656245 {
        if q.mode == 2 {
            return 50;
        }
        if q.mode != 0 {
            return 25;
        }
        let rc = audio_queue_set_property(3);
        if rc != 0 && rc != -66684 {
            return 33;
        }
        started = audio_queue_start();
    }
    if started != 0 {
        return 25;
    }
    let (base, first) = match read_packet_table(start_packet) {
        Some(t) => t,
        None => return 33,
    };
    q.finished = false;
    q.stopping = false;
    q.start_packet = first;
    q.position = start_packet.wrapping_sub(base) as f64;
    q.data_offset = base as usize * q.frame_bytes.max(1);
    q.primed = true;
    if audio_queue_offline_render() != 0 {
        return 33;
    }
    process_audio_queue(q)
}

// 0x107090 — __ZN4FMOD15CodecAudioQueue19setPositionCallbackEP16FMOD_CODEC_STATEijj
#[doc(alias = "__ZN4FMOD15CodecAudioQueue19setPositionCallbackEP16FMOD_CODEC_STATEijj")]
pub fn stub_107090(
    has_state: bool,
    q: &mut CodecAudioQueue,
    start_packet: u32,
    audio_queue_stop: &mut dyn FnMut() -> i32,
    audio_queue_set_property: &mut dyn FnMut(i32) -> i32,
    audio_queue_start: &mut dyn FnMut() -> i32,
    read_packet_table: &mut dyn FnMut(u32) -> Option<(u32, u32)>,
    audio_queue_offline_render: &mut dyn FnMut() -> i32,
    process_audio_queue: &mut dyn FnMut(&mut CodecAudioQueue) -> i32,
) -> i32 {
    // IDA 0x107090: state ? setPositionInternal(state - 28, ...) : setPositionInternal(0, ...).
    let _ = has_state;
    stub_106eac(
        q,
        start_packet,
        audio_queue_stop,
        audio_queue_set_property,
        audio_queue_start,
        read_packet_table,
        audio_queue_offline_render,
        process_audio_queue,
    )
}

// 0x10709c — __ZN4FMOD15CodecAudioQueue13closeInternalEv
// type: _DWORD __fastcall(FMOD::CodecAudioQueue *__hidden this)
#[doc(alias = "__ZN4FMOD15CodecAudioQueue13closeInternalEv")]
pub fn stub_10709c(
    linked: &mut bool,
    has_queue: bool,
    dispose_queue: &mut dyn FnMut() -> i32,
    has_file: bool,
    close_file: &mut dyn FnMut() -> i32,
    free_codec: &mut dyn FnMut(),
) -> i32 {
    // IDA 0x10709c: unlink the gCodecHead node; AudioQueueDispose (fail → 44, clear queue);
    // AudioFileClose (fail → 44, clear file); MemPool::free the codec block (:205); 0.
    *linked = false;
    if has_queue {
        if dispose_queue() != 0 {
            return 44;
        }
    }
    if has_file {
        if close_file() != 0 {
            return 44;
        }
    }
    free_codec();
    0
}

// 0x107164 — __ZN4FMOD15CodecAudioQueue13closeCallbackEP16FMOD_CODEC_STATE
#[doc(alias = "__ZN4FMOD15CodecAudioQueue13closeCallbackEP16FMOD_CODEC_STATE")]
pub fn stub_107164(
    has_state: bool,
    linked: &mut bool,
    has_queue: bool,
    dispose_queue: &mut dyn FnMut() -> i32,
    has_file: bool,
    close_file: &mut dyn FnMut() -> i32,
    free_codec: &mut dyn FnMut(),
) -> i32 {
    // IDA 0x107164: state ? closeInternal(state - 28) : closeInternal(0).
    let _ = has_state;
    stub_10709c(linked, has_queue, dispose_queue, has_file, close_file, free_codec)
}

// 0x107170 — __ZN4FMOD15CodecAudioQueue16getDescriptionExEv
// type: _DWORD __fastcall(FMOD::CodecAudioQueue *__hidden this)
#[doc(alias = "__ZN4FMOD15CodecAudioQueue16getDescriptionExEv")]
pub fn stub_107170() -> &'static CodecDescriptor {
    // IDA 0x107170: guard-checked once init of the audioQueueCodec descriptor; memset + fill
    // ("FMOD Audio Queue Codec", 65792, 2, 24, 444, open/read/setPosition/close callbacks); return it.
    &AUDIO_QUEUE_CODEC_DESC
}

// 0x107284 — __ZN4FMOD15CodecAudioQueue15setupAudioQueueEv
// type: _DWORD __fastcall(FMOD::CodecAudioQueue *__hidden this)
#[doc(alias = "__ZN4FMOD15CodecAudioQueue15setupAudioQueueEv")]
pub fn stub_107284(
    q: &mut CodecAudioQueue,
    new_output: &mut dyn FnMut() -> i32,
    channel_layout: &mut dyn FnMut() -> Option<usize>,
    apply_layout: &mut dyn FnMut(usize) -> i32,
    allocate_buffers: &mut dyn FnMut(&mut CodecAudioQueue) -> i32,
) -> i32 {
    // IDA 0x107284: AudioQueueNewOutput (fail → LABEL_5/44); optional 'mgiс' channel layout alloc;
    // 'cmap' get (fail → 44); 'aqlc' set (fail → 44); buffer alloc/prime; 0.
    if new_output() != 0 {
        return 44;
    }
    if let Some(layout) = channel_layout() {
        if apply_layout(layout) != 0 {
            return 44;
        }
    }
    allocate_buffers(q)
}

// 0x107598 — __ZN4FMOD15CodecAudioQueue12readInternalEPvjPj
// type: _DWORD __fastcall(FMOD::CodecAudioQueue *__hidden this, void *, unsigned int, unsigned int *)
#[doc(alias = "__ZN4FMOD15CodecAudioQueue12readInternalEPvjPj")]
pub fn stub_107598(
    q: &mut CodecAudioQueue,
    out: &mut [u8],
    max_bytes: usize,
    render: &mut dyn FnMut(usize) -> Option<usize>,
    stop: &mut dyn FnMut() -> i32,
    dispose: &mut dyn FnMut() -> i32,
    setup: &mut dyn FnMut(&mut CodecAudioQueue) -> i32,
    reposition: &mut dyn FnMut(&mut CodecAudioQueue) -> i32,
    copy_out: &mut dyn FnMut(&mut [u8], usize) -> usize,
) -> (i32, usize) {
    // IDA 0x107598: finished → 22; OfflineRender min(a3, stride) bytes (fail → 33); position +=
    // rendered/frame_bytes; past total → stop (fail → 33), mark finished; empty render before total
    // → dispose (fail → 44) + re-setup + reposition (fail → code); consume the carry offset into
    // *a4 ([1B8] update); 0.
    if q.finished {
        return (22, 0);
    }
    let n = max_bytes.min(q.buffer_stride);
    let available = match render(n / q.frame_bytes.max(1)) {
        Some(b) => b,
        None => return (33, 0),
    };
    q.position += available as f64 / q.frame_bytes.max(1) as f64;
    if q.position >= q.total_frames as f64 {
        if stop() != 0 {
            return (33, 0);
        }
        q.finished = true;
    } else if available == 0 {
        if dispose() != 0 {
            return (44, 0);
        }
        let rc = setup(q);
        if rc != 0 {
            return (rc, 0);
        }
        let rc = reposition(q);
        if rc != 0 {
            return (rc, 0);
        }
    }
    if available < q.carry {
        q.carry -= available;
        return (0, 0);
    }
    let actual = copy_out(out, available - q.carry);
    q.carry = 0;
    (0, actual)
}

// 0x10773c — __ZN4FMOD15CodecAudioQueue12readCallbackEP16FMOD_CODEC_STATEPvjPj
#[doc(alias = "__ZN4FMOD15CodecAudioQueue12readCallbackEP16FMOD_CODEC_STATEPvjPj")]
pub fn stub_10773c(
    has_state: bool,
    q: &mut CodecAudioQueue,
    out: &mut [u8],
    max_bytes: usize,
    render: &mut dyn FnMut(usize) -> Option<usize>,
    stop: &mut dyn FnMut() -> i32,
    dispose: &mut dyn FnMut() -> i32,
    setup: &mut dyn FnMut(&mut CodecAudioQueue) -> i32,
    reposition: &mut dyn FnMut(&mut CodecAudioQueue) -> i32,
    copy_out: &mut dyn FnMut(&mut [u8], usize) -> usize,
) -> (i32, usize) {
    // IDA 0x10773c: state ? readInternal(state - 28, ...) : readInternal(0, ...).
    let _ = has_state;
    stub_107598(q, out, max_bytes, render, stop, dispose, setup, reposition, copy_out)
}

// 0x107748 — __ZN4FMOD15CodecAudioQueue12openInternalEjP22FMOD_CREATESOUNDEXINFO
#[doc(alias = "__ZN4FMOD15CodecAudioQueue12openInternalEjP22FMOD_CREATESOUNDEXINFO")]
pub fn stub_107748(
    q: &mut CodecAudioQueue,
    flags: i32,
    has_subclass: bool,
    seek_start: &mut dyn FnMut() -> i32,
    setup_file: &mut dyn FnMut(&mut CodecAudioQueue, bool) -> i32,
    setup_queue: &mut dyn FnMut(&mut CodecAudioQueue) -> i32,
    alloc_block: &mut dyn FnMut() -> bool,
    link: &mut dyn FnMut(),
) -> i32 {
    // IDA 0x107748: gGlobal init; words 68/260/28/32/272 set (mode 24); File::seek(0) → return;
    // setupAudioFile(a2 & 0x4000) → return; setupAudioQueue → return; calloc 0x128 (:146) → 44;
    // codec + gCodecHead list links; 0.
    q.mode = 24;
    q.primed = false;
    q.finished = false;
    let _ = has_subclass;
    let rc = seek_start();
    if rc != 0 {
        return rc;
    }
    let rc = setup_file(q, flags & 0x4000 != 0);
    if rc != 0 {
        return rc;
    }
    let rc = setup_queue(q);
    if rc != 0 {
        return rc;
    }
    if !alloc_block() {
        return 44;
    }
    link();
    0
}

// 0x1078d4 — __ZN4FMOD15CodecAudioQueue12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO
#[doc(alias = "__ZN4FMOD15CodecAudioQueue12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO")]
pub fn stub_1078d4(
    has_state: bool,
    q: &mut CodecAudioQueue,
    flags: i32,
    has_subclass: bool,
    seek_start: &mut dyn FnMut() -> i32,
    setup_file: &mut dyn FnMut(&mut CodecAudioQueue, bool) -> i32,
    setup_queue: &mut dyn FnMut(&mut CodecAudioQueue) -> i32,
    alloc_block: &mut dyn FnMut() -> bool,
    link: &mut dyn FnMut(),
) -> i32 {
    // IDA 0x1078d4: state ? openInternal(state - 28, ...) : openInternal(state, ...).
    let _ = has_state;
    stub_107748(q, flags, has_subclass, seek_start, setup_file, setup_queue, alloc_block, link)
}

// 0x1078e0 — __ZN4FMOD15CodecAudioQueue8resetAllEbb
// type: _DWORD __fastcall(FMOD::CodecAudioQueue *__hidden this, bool, bool)
#[doc(alias = "__ZN4FMOD15CodecAudioQueue8resetAllEbb")]
pub fn stub_1078e0(
    codecs: &mut Vec<CodecAudioQueue>,
    dispose_matching: bool,
    reset: bool,
    dispose_queue: &mut dyn FnMut(&mut CodecAudioQueue),
    setup_queue: &mut dyn FnMut(&mut CodecAudioQueue) -> i32,
    reposition: &mut dyn FnMut(&mut CodecAudioQueue) -> i32,
) -> i32 {
    // IDA 0x1078e0: walk gCodecHead; skip finished (byte+369); dispose when v11; skip when !v3;
    // else setupAudioQueue (fail → return) then setPositionInternal (fail → return); end → 0.
    for codec in codecs.iter_mut() {
        if codec.finished {
            continue;
        }
        if dispose_matching {
            dispose_queue(codec);
            if !reset {
                continue;
            }
        } else if !reset {
            continue;
        }
        let rc = setup_queue(codec);
        if rc != 0 {
            return rc;
        }
        let rc = reposition(codec);
        if rc != 0 {
            return rc;
        }
    }
    0
}

// 0x1079e8 — __Z41__static_initialization_and_destruction_0ii_41
// type: _DWORD __fastcall(int, int)
#[doc(alias = "__Z41__static_initialization_and_destruction_0ii_41")]
pub fn stub_1079e8(result: i32, a2: i32, init: &mut dyn FnMut()) -> i32 {
    // IDA 0x1079e8: if result == 1 && a2 == 0xFFFF: gCodecHead list head self-init.
    if result == 1 && a2 == 0xFFFF {
        init();
    }
    result
}

// 0x107a1c — __GLOBAL__I__ZN4FMOD15CodecAudioQueue10gCodecHeadE
#[doc(alias = "__GLOBAL__I__ZN4FMOD15CodecAudioQueue10gCodecHeadE")]
pub fn stub_107a1c(init: &mut dyn FnMut()) -> i32 {
    // IDA 0x107a1c: global ctor keyed to gCodecHead → static_init(1, 0xFFFF).
    stub_1079e8(1, 0xFFFF, init)
}

// 0x107a28 — __ZL22FreeImage_GetImageSizeiii
// type: _DWORD __fastcall(int, int, int)
#[doc(alias = "__ZL22FreeImage_GetImageSizeiii")]
pub fn stub_107a28(width: u32, height: u32, bpp: u32, calculate_line: &mut dyn FnMut(u32, u32) -> u32) -> u32 {
    // IDA 0x107a28: palette size clamped by bpp (out of 1..8 → header only); + 352-byte header;
    // + aligned line pitch * height.
    let (pad, header) = if bpp.wrapping_sub(1) > 7 {
        (0, 352)
    } else {
        let entry = (4 << bpp) & 0xF;
        (if entry != 0 { 16 - entry } else { 0 }, (4 << bpp) + 352)
    };
    pad + header + ((calculate_line(width, bpp) + 3) & !3) * height
}

// 0x107a78 — _FreeImage_GetImageType
#[doc(alias = "_FreeImage_GetImageType")]
pub fn stub_107a78(dib: Option<&FreeImageInfo>) -> i32 {
    // IDA 0x107a78: null → 0, else the image type word.
    dib.map(|d| d.image_type).unwrap_or(0)
}

// 0x107a88 — _FreeImage_GetRedMask
#[doc(alias = "_FreeImage_GetRedMask")]
pub fn stub_107a88(dib: Option<&FreeImageInfo>) -> u32 {
    // IDA 0x107a88: null → 0, else the red mask (+4).
    dib.map(|d| d.red_mask).unwrap_or(0)
}

// 0x107a98 — _FreeImage_GetGreenMask
#[doc(alias = "_FreeImage_GetGreenMask")]
pub fn stub_107a98(dib: Option<&FreeImageInfo>) -> u32 {
    // IDA 0x107a98: null → 0, else the green mask (+8).
    dib.map(|d| d.green_mask).unwrap_or(0)
}

// 0x107aa8 — _FreeImage_GetBlueMask
#[doc(alias = "_FreeImage_GetBlueMask")]
pub fn stub_107aa8(dib: Option<&FreeImageInfo>) -> u32 {
    // IDA 0x107aa8: null → 0, else the blue mask (+12).
    dib.map(|d| d.blue_mask).unwrap_or(0)
}

// 0x107ab8 — _FreeImage_HasBackgroundColor
#[doc(alias = "_FreeImage_HasBackgroundColor")]
pub fn stub_107ab8(dib: Option<&FreeImageInfo>) -> bool {
    // IDA 0x107ab8: null → false, else byte +19 != 0.
    dib.map(|d| d.has_background).unwrap_or(false)
}

// 0x107ad4 — _FreeImage_GetTransparencyTable
#[doc(alias = "_FreeImage_GetTransparencyTable")]
pub fn stub_107ad4(dib: Option<&FreeImageInfo>) -> Option<usize> {
    // IDA 0x107ad4: null → null, else the transparency table handle (+28).
    dib.and_then(|d| if d.transparency_table == 0 { None } else { Some(d.transparency_table) })
}

// 0x107ae4 — _FreeImage_GetTransparencyCount
// type: int __fastcall(int result)
#[doc(alias = "_FreeImage_GetTransparencyCount")]
pub fn stub_107ae4(dib: Option<&FreeImageInfo>) -> u32 {
    // IDA 0x107ae4: null → 0, else the transparency count (+24).
    dib.map(|d| d.transparency_count).unwrap_or(0)
}

// 0x107af4 — _FreeImage_GetICCProfile
#[doc(alias = "_FreeImage_GetICCProfile")]
pub fn stub_107af4(dib: Option<&FreeImageInfo>) -> Option<usize> {
    // IDA 0x107af4: null → null, else the ICC profile handle (+284).
    dib.and_then(|d| if d.icc_profile == 0 { None } else { Some(d.icc_profile) })
}

// 0x107b04 — _FreeImage_GetInfoHeader
// type: int(void)
#[doc(alias = "_FreeImage_GetInfoHeader")]
pub fn stub_107b04(base: Option<usize>) -> Option<usize> {
    // IDA 0x107b04: null → null; else base + 308 + the alignment pad of (base + 300).
    base.map(|b| {
        let mut pad = (b + 300) & 0xF;
        if pad != 0 {
            pad = 16 - pad;
        }
        b + 308 + pad
    })
}

// 0x107b28 — _FreeImage_SetDotsPerMeterY
#[doc(alias = "_FreeImage_SetDotsPerMeterY")]
pub fn stub_107b28(dib: &mut FreeImageInfo, dots_per_meter_y: i32) {
    // IDA 0x107b28: null → no-op; else info header word 7 = y (via GetInfoHeader).
    dib.dots_per_meter_y = dots_per_meter_y;
}

// 0x107b48 — _FreeImage_SetDotsPerMeterX
#[doc(alias = "_FreeImage_SetDotsPerMeterX")]
pub fn stub_107b48(dib: &mut FreeImageInfo, dots_per_meter_x: i32) {
    // IDA 0x107b48: null → no-op; else info header word 6 = x (via GetInfoHeader).
    dib.dots_per_meter_x = dots_per_meter_x;
}

// 0x107b68 — _FreeImage_GetDotsPerMeterY
#[doc(alias = "_FreeImage_GetDotsPerMeterY")]
pub fn stub_107b68(dib: Option<&FreeImageInfo>) -> i32 {
    // IDA 0x107b68: null → 0, else info header word 7.
    dib.map(|d| d.dots_per_meter_y).unwrap_or(0)
}

// 0x107b88 — _FreeImage_GetDotsPerMeterX
#[doc(alias = "_FreeImage_GetDotsPerMeterX")]
pub fn stub_107b88(dib: Option<&FreeImageInfo>) -> i32 {
    // IDA 0x107b88: null → 0, else info header word 6.
    dib.map(|d| d.dots_per_meter_x).unwrap_or(0)
}

// 0x107ba8 — _FreeImage_GetColorsUsed
#[doc(alias = "_FreeImage_GetColorsUsed")]
pub fn stub_107ba8(dib: Option<&FreeImageInfo>) -> u32 {
    // IDA 0x107ba8: null → 0, else info header word 8 (colors used).
    dib.map(|d| d.colors_used).unwrap_or(0)
}

// 0x107bc8 — _FreeImage_GetBPP
// type: int(void)
#[doc(alias = "_FreeImage_GetBPP")]
pub fn stub_107bc8(dib: Option<&FreeImageInfo>) -> u32 {
    // IDA 0x107bc8: null → 0, else the bpp halfword (header half 7).
    dib.map(|d| d.bpp).unwrap_or(0)
}

// 0x107be8 — _FreeImage_GetPalette
#[doc(alias = "_FreeImage_GetPalette")]
pub fn stub_107be8(dib: Option<&FreeImageInfo>) -> Option<&[[u8; 4]]> {
    // IDA 0x107be8: dib && bpp <= 15 → palette handle (header + 10); else null.
    dib.filter(|d| d.bpp <= 15).map(|d| d.palette.as_slice())
}

// 0x107c1c — _FreeImage_SetTransparent
#[doc(alias = "_FreeImage_SetTransparent")]
pub fn stub_107c1c(dib: &mut FreeImageInfo, index: i32) -> i32 {
    // IDA 0x107c1c: bpp <= 8 or == 32 → word+20 = index, return dib (nonzero); else word+20 = 0,
    // return bpp.
    if dib.bpp <= 8 || dib.bpp == 32 {
        dib.transparent_index = index;
        1
    } else {
        dib.transparent_index = 0;
        dib.bpp as i32
    }
}

// 0x107c60 — _FreeImage_GetHeight
// type: int __fastcall(int)
#[doc(alias = "_FreeImage_GetHeight")]
pub fn stub_107c60(dib: Option<&FreeImageInfo>) -> i32 {
    // IDA 0x107c60: null → 0, else info header word 2 (height).
    dib.map(|d| d.height as i32).unwrap_or(0)
}

// 0x107c80 — _FreeImage_GetWidth
#[doc(alias = "_FreeImage_GetWidth")]
pub fn stub_107c80(dib: Option<&FreeImageInfo>) -> u32 {
    // IDA 0x107c80: null → 0, else info header word 1 (width).
    dib.map(|d| d.width).unwrap_or(0)
}

// 0x107ca0 — _FreeImage_GetLine
#[doc(alias = "_FreeImage_GetLine")]
pub fn stub_107ca0(dib: Option<&FreeImageInfo>) -> u32 {
    // IDA 0x107ca0: null → 0; else (width * bpp + 7) >> 3.
    dib.map(|d| (d.width * d.bpp + 7) >> 3).unwrap_or(0)
}

// 0x107cd4 — _FreeImage_GetPitch
#[doc(alias = "_FreeImage_GetPitch")]
pub fn stub_107cd4(dib: Option<&FreeImageInfo>) -> u32 {
    // IDA 0x107cd4: null → 0; else (GetLine + 3) & ~3.
    (stub_107ca0(dib) + 3) & !3
}

// 0x107cf8 — _FreeImage_GetBackgroundColor
#[doc(alias = "_FreeImage_GetBackgroundColor")]
pub fn stub_107cf8(dib: Option<&FreeImageInfo>, out: &mut [u8; 4], find_index: &mut dyn FnMut(&FreeImageInfo, [u8; 4]) -> u8) -> i32 {
    // IDA 0x107cf8: null/no-bg → 0; copy the +16 bytes; bpp != 8 → alpha 0, 1; bpp 8 → palette
    // index search, 1.
    let dib = match dib {
        Some(d) if d.has_background => d,
        _ => return 0,
    };
    *out = dib.background_color;
    if dib.bpp != 8 {
        out[3] = 0;
        return 1;
    }
    out[3] = find_index(dib, dib.background_color);
    1
}

// 0x10813c — _FreeImage_FindCloseMetadata
#[doc(alias = "_FreeImage_FindCloseMetadata")]
pub fn stub_10813c(handle: Option<usize>, free_inner: &mut dyn FnMut(usize), free_outer: &mut dyn FnMut(usize)) {
    // IDA 0x10813c: null → no-op; free the search handle then the parent block.
    if let Some(h) = handle {
        free_inner(h);
        free_outer(h);
    }
}

// 0x108168 — _FreeImage_DestroyICCProfile
#[doc(alias = "_FreeImage_DestroyICCProfile")]
pub fn stub_108168(dib: &mut FreeImageInfo, is_reserved: bool, free: &mut dyn FnMut(usize)) {
    // IDA 0x108168: null → no-op; *dib == -284 (reserved magic) → skip; else free the ICC data and
    // clear size/data.
    if is_reserved {
        return;
    }
    if dib.icc_profile != 0 {
        free(dib.icc_profile);
    }
    dib.icc_profile = 0;
    dib.icc_size = 0;
}

// 0x1081a0 — __Z22FreeImage_Aligned_FreePv
// type: _DWORD __fastcall(void *)
#[doc(alias = "__Z22FreeImage_Aligned_FreePv")]
pub fn stub_1081a0(aligned: usize, raw_of: &mut dyn FnMut(usize) -> usize, free: &mut dyn FnMut(usize)) {
    // IDA 0x1081a0: free the raw block kept at slot - 4.
    free(raw_of(aligned));
}

// 0x1081b4 — _FreeImage_CreateICCProfile
#[doc(alias = "_FreeImage_CreateICCProfile")]
pub fn stub_1081b4(
    has_dib: bool,
    dib: &mut FreeImageInfo,
    is_reserved: bool,
    data: &[u8],
    destroy: &mut dyn FnMut(&mut FreeImageInfo),
    alloc_copy: &mut dyn FnMut(&[u8]) -> Option<usize>,
) -> usize {
    // IDA 0x1081b4: DestroyICCProfile; null dib → 0; empty data or reserved magic → the +284 slot;
    // else malloc + copy, store size/data; return the +284 slot.
    destroy(dib);
    if !has_dib {
        return 0;
    }
    if data.is_empty() || is_reserved {
        return 284;
    }
    if let Some(h) = alloc_copy(data) {
        dib.icc_profile = h;
        dib.icc_size = data.len();
    }
    284
}

// 0x108220 — _FreeImage_SetTransparencyTable
// type: void *__fastcall(void *result, const void *, size_t)
#[doc(alias = "_FreeImage_SetTransparencyTable")]
pub fn stub_108220(dib: &mut FreeImageInfo, data: Option<&[u8]>, fill: &mut dyn FnMut(&mut FreeImageInfo, Option<&[u8]>)) -> bool {
    // IDA 0x108220: bpp > 8 → no-op (false); else transparent flag + count set; copy the table or
    // fill 255.
    if dib.bpp > 8 {
        return false;
    }
    dib.transparent_index = 1;
    dib.transparency_count = data.map(|d| d.len() as u32).unwrap_or(0);
    fill(dib, data);
    true
}

// 0x108290 — _FreeImage_SetBackgroundColor
#[doc(alias = "_FreeImage_SetBackgroundColor")]
pub fn stub_108290(dib: Option<&mut FreeImageInfo>, color: Option<[u8; 4]>) -> i32 {
    // IDA 0x108290: null → 0; color → copy to +16, set flag; none → clear +16..19; 1.
    let dib = match dib {
        Some(d) => d,
        None => return 0,
    };
    match color {
        Some(c) => {
            dib.background_color = c;
            dib.has_background = true;
        }
        None => {
            dib.background_color = [0; 4];
            dib.has_background = false;
        }
    }
    1
}

// 0x1082dc — _FreeImage_GetColorType
#[doc(alias = "_FreeImage_GetColorType")]
pub fn stub_1082dc(dib: Option<&FreeImageInfo>, analyze_bitmap: &mut dyn FnMut(&FreeImageInfo) -> i32) -> i32 {
    // IDA 0x1082dc: FIT_RGBA16/RGBAF (9/11) → 2 (palette); FIT_RGBF/type-12 (10/12) → 4 (rgb-alpha);
    // bitmap and other types → pixel analysis.
    let dib = match dib {
        Some(d) => d,
        None => return 0,
    };
    match dib.image_type {
        9 | 11 => 2,
        10 | 12 => 4,
        _ => analyze_bitmap(dib),
    }
}

// 0x108858 — _FreeImage_IsTransparent
#[doc(alias = "_FreeImage_IsTransparent")]
pub fn stub_108858(dib: Option<&FreeImageInfo>, color_type_of: &mut dyn FnMut(&FreeImageInfo) -> i32) -> bool {
    // IDA 0x108858: null → false; bpp 32 → GetColorType == 4 (rgb-alpha); else word+20 != 0.
    let dib = match dib {
        Some(d) => d,
        None => return false,
    };
    if dib.bpp == 32 {
        return color_type_of(dib) == 4;
    }
    dib.transparent_index != 0
}

// 0x1088a4 — __Z24FreeImage_Aligned_Mallocmm
// type: _DWORD __fastcall(unsigned int, unsigned int)
#[doc(alias = "__Z24FreeImage_Aligned_Mallocmm")]
pub fn stub_1088a4(size: usize, alignment: u32, malloc: &mut dyn FnMut(usize) -> Option<usize>) -> Option<usize> {
    // IDA 0x1088a4: assert alignment == 16; malloc(size + 32); slot = (raw & ~15) + 32 with raw kept
    // at slot - 4; null on failure.
    assert_eq!(alignment, 16);
    malloc(size + 32)
}

// 0x1088fc — _FreeImage_AllocateT
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "_FreeImage_AllocateT")]
pub fn stub_1088fc(
    image_type: i32,
    width: u32,
    height: u32,
    bpp: u32,
    build: &mut dyn FnMut(i32, u32, u32, u32) -> Option<FreeImageInfo>,
) -> Option<FreeImageInfo> {
    // IDA 0x1088fc: handle = malloc(4) (null → null); bpp by type (1: validate 1/2/4/8/16/32 else 8;
    // 2/3: 16; 4/5/6: 32; others fall through); image-size + aligned data alloc; header fill.
    let bits = match image_type {
        1 => {
            let shift = bpp.wrapping_sub(1);
            if shift > 0x1F || ((1u32 << shift) & 0x80808089) == 0 {
                8
            } else {
                bpp
            }
        }
        2 | 3 => 16,
        4 | 5 | 6 => 32,
        _ => bpp,
    };
    build(image_type, width, height, bits)
}

// 0x108afc — _FreeImage_Allocate
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "_FreeImage_Allocate")]
pub fn stub_108afc(
    width: u32,
    height: u32,
    bpp: u32,
    masks: [u32; 3],
    build: &mut dyn FnMut(i32, u32, u32, u32) -> Option<FreeImageInfo>,
) -> Option<FreeImageInfo> {
    // IDA 0x108afc: Allocate(width, height, bpp, masks) → AllocateT(FIT_BITMAP = 1, ...).
    let _ = masks;
    stub_1088fc(1, width, height, bpp, build)
}

// 0x108b40 — _FreeImage_Unload
#[doc(alias = "_FreeImage_Unload")]
pub fn stub_108b40(dib: &mut Option<FreeImageInfo>, free_tags: &mut dyn FnMut()) {
    // IDA 0x108b40: null → no-op; free the ICC profile block; walk the metadata map deleting tags;
    // free the dib.
    if dib.is_none() {
        return;
    }
    free_tags();
    *dib = None;
}

// 0x108cdc — _FreeImage_FindNextMetadata
#[doc(alias = "_FreeImage_FindNextMetadata")]
pub fn stub_108cdc(model: &mut FreeImageMetadata, domain: i32) -> Option<(String, Vec<u8>)> {
    // IDA 0x108cdc: null → 0; walk the domain map from the cursor in tree order; each hit bumps the
    // index and returns the tag (1); exhausted → 0.
    let tags = model.domains.get(&domain)?;
    let next = tags.iter().nth(model.cursor.1)?;
    model.cursor = (domain, model.cursor.1 + 1);
    Some((next.0.clone(), next.1.clone()))
}

// 0x108e98 — _FreeImage_GetMetadataCount
#[doc(alias = "_FreeImage_GetMetadataCount")]
pub fn stub_108e98(model: Option<&FreeImageMetadata>, domain: i32) -> i32 {
    // IDA 0x108e98: null dib → 0; domain missing → 0; else the domain tag count.
    model
        .and_then(|m| m.domains.get(&domain))
        .map(|tags| tags.len() as i32)
        .unwrap_or(0)
}

// 0x108f00 — _FreeImage_GetMetadata
#[doc(alias = "_FreeImage_GetMetadata")]
pub fn stub_108f00(model: Option<&FreeImageMetadata>, domain: i32, key: Option<&str>) -> Option<Vec<u8>> {
    // IDA 0x108f00: null key/model/out → 0; *out = 0; domain + key lookup in the tag trees;
    // miss → 0; hit → tag stored, nonzero.
    let (model, key) = match (model, key) {
        (Some(m), Some(k)) => (m, k),
        _ => return None,
    };
    model.domains.get(&domain)?.get(key).cloned()
}

// 0x1090ac — _FreeImage_SetMetadata
#[doc(alias = "_FreeImage_SetMetadata")]
pub fn stub_1090ac() -> ! {
    todo!("0x1090ac _FreeImage_SetMetadata")
}

// 0x109578 — _FreeImage_CloneMetadata
#[doc(alias = "_FreeImage_CloneMetadata")]
pub fn stub_109578() -> ! {
    todo!("0x109578 _FreeImage_CloneMetadata")
}

// 0x1097ac — _FreeImage_FindFirstMetadata
#[doc(alias = "_FreeImage_FindFirstMetadata")]
pub fn stub_1097ac() -> ! {
    todo!("0x1097ac _FreeImage_FindFirstMetadata")
}

// 0x1098b4 — _FreeImage_Clone
#[doc(alias = "_FreeImage_Clone")]
pub fn stub_1098b4() -> ! {
    todo!("0x1098b4 _FreeImage_Clone")
}

// 0x109b88 — __Z13CalculateLineii
// type: _DWORD __fastcall(int, int)
#[doc(alias = "__Z13CalculateLineii")]
pub fn stub_109b88() -> ! {
    todo!("0x109b88 __Z13CalculateLineii")
}

// 0x109b9c — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE4findERS1_
#[doc(alias = "__ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE4findERS1_")]
pub fn stub_109b9c() -> ! {
    todo!("0x109b9c __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE4findERS1_")
}

// 0x109bf8 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE13_Rb_tree_implISF_Lb0EEC2ERKSaISt13_Rb_tree_nodeISC_EERKSF_
#[doc(alias = "__ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE13_Rb_tree_implISF_Lb0EEC2ERKSaISt13_Rb_tree_nodeISC_EERKSF_")]
pub fn stub_109bf8() -> ! {
    todo!("0x109bf8 __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE13_Rb_tree_implISF_Lb0EEC2ERKSaISt13_Rb_tree_nodeISC_EERKSF_")
}

// 0x109c38 — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE13_Rb_tree_implIS8_Lb0EEC2ERKSaISt13_Rb_tree_nodeIS4_EERKS8_
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE13_Rb_tree_implIS8_Lb0EEC2ERKSaISt13_Rb_tree_nodeIS4_EERKS8_")]
pub fn stub_109c38() -> ! {
    todo!("0x109c38 __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE13_Rb_tree_implIS8_Lb0EEC2ERKSaISt13_Rb_tree_nodeIS4_EERKS8_")
}

// 0x109c78 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE11lower_boundERS1_
#[doc(alias = "__ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE11lower_boundERS1_")]
pub fn stub_109c78() -> ! {
    todo!("0x109c78 __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE11lower_boundERS1_")
}

// 0x109cac — __ZN9__gnu_cxx13new_allocatorISt4pairIKSsP5FITAGEE9constructEPS5_RKS5_
#[doc(alias = "__ZN9__gnu_cxx13new_allocatorISt4pairIKSsP5FITAGEE9constructEPS5_RKS5_")]
pub fn stub_109cac() -> ! {
    todo!("0x109cac __ZN9__gnu_cxx13new_allocatorISt4pairIKSsP5FITAGEE9constructEPS5_RKS5_")
}

// 0x109d68 — __ZN9__gnu_cxx13new_allocatorISt13_Rb_tree_nodeISt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS2_IKSsS6_EEEEEE8allocateEmPKv
#[doc(alias = "__ZN9__gnu_cxx13new_allocatorISt13_Rb_tree_nodeISt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS2_IKSsS6_EEEEEE8allocateEmPKv")]
pub fn stub_109d68() -> ! {
    todo!("0x109d68 __ZN9__gnu_cxx13new_allocatorISt13_Rb_tree_nodeISt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS2_IKSsS6_EEEEEE8allocateEmPKv")
}

// 0x109d98 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE14_M_create_nodeERKSC_
#[doc(alias = "__ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE14_M_create_nodeERKSC_")]
pub fn stub_109d98() -> ! {
    todo!("0x109d98 __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE14_M_create_nodeERKSC_")
}

// 0x109dc8 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE9_M_insertEPSt18_Rb_tree_node_baseSJ_RKSC_
#[doc(alias = "__ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE9_M_insertEPSt18_Rb_tree_node_baseSJ_RKSC_")]
pub fn stub_109dc8() -> ! {
    todo!("0x109dc8 __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE9_M_insertEPSt18_Rb_tree_node_baseSJ_RKSC_")
}

// 0x109e4c — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE16_M_insert_uniqueERKSC_
#[doc(alias = "__ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE16_M_insert_uniqueERKSC_")]
pub fn stub_109e4c() -> ! {
    todo!("0x109e4c __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE16_M_insert_uniqueERKSC_")
}

// 0x109f0c — __ZN9__gnu_cxx13new_allocatorISt13_Rb_tree_nodeISt4pairIKSsP5FITAGEEE8allocateEmPKv
#[doc(alias = "__ZN9__gnu_cxx13new_allocatorISt13_Rb_tree_nodeISt4pairIKSsP5FITAGEEE8allocateEmPKv")]
pub fn stub_109f0c() -> ! {
    todo!("0x109f0c __ZN9__gnu_cxx13new_allocatorISt13_Rb_tree_nodeISt4pairIKSsP5FITAGEEE8allocateEmPKv")
}

// 0x109f3c — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE14_M_create_nodeERKS4_
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE14_M_create_nodeERKS4_")]
pub fn stub_109f3c() -> ! {
    todo!("0x109f3c __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE14_M_create_nodeERKS4_")
}

// 0x10a03c — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS4_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS4_E")]
pub fn stub_10a03c() -> ! {
    todo!("0x10a03c __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS4_E")
}

// 0x10a0e4 — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
// type: int __fastcall(int result, int)
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E")]
pub fn stub_10a0e4() -> ! {
    todo!("0x10a0e4 __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E")
}

// 0x10a124 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE8_M_eraseEPSt13_Rb_tree_nodeISC_E
#[doc(alias = "__ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE8_M_eraseEPSt13_Rb_tree_nodeISC_E")]
pub fn stub_10a124() -> ! {
    todo!("0x10a124 __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE8_M_eraseEPSt13_Rb_tree_nodeISC_E")
}

// 0x10a160 — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE5eraseESt17_Rb_tree_iteratorIS4_E
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE5eraseESt17_Rb_tree_iteratorIS4_E")]
pub fn stub_10a160() -> ! {
    todo!("0x10a160 __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE5eraseESt17_Rb_tree_iteratorIS4_E")
}

// 0x10a198 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE5eraseESt17_Rb_tree_iteratorISC_E
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "__ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE5eraseESt17_Rb_tree_iteratorISC_E")]
pub fn stub_10a198() -> ! {
    todo!("0x10a198 __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE5eraseESt17_Rb_tree_iteratorISC_E")
}

// 0x10a1c8 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISC_ERKSC_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "__ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISC_ERKSC_")]
pub fn stub_10a1c8() -> ! {
    todo!("0x10a1c8 __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISC_ERKSC_")
}

// 0x10a2ec — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE11upper_boundERS1_
// type: int __fastcall(int, std::string *this)
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE11upper_boundERS1_")]
pub fn stub_10a2ec() -> ! {
    todo!("0x10a2ec __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE11upper_boundERS1_")
}

// 0x10a334 — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE11lower_boundERS1_
// type: int __fastcall(int, std::string *)
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE11lower_boundERS1_")]
pub fn stub_10a334() -> ! {
    todo!("0x10a334 __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE11lower_boundERS1_")
}

// 0x10a37c — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE11equal_rangeERS1_
// type: int __fastcall(int, int, std::string *)
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE11equal_rangeERS1_")]
pub fn stub_10a37c() -> ! {
    todo!("0x10a37c __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE11equal_rangeERS1_")
}

// 0x10a3c4 — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE4findERS1_
// type: int __fastcall(int, std::string *this)
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE4findERS1_")]
pub fn stub_10a3c4() -> ! {
    todo!("0x10a3c4 __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE4findERS1_")
}

// 0x10a43c — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE9_M_insertEPSt18_Rb_tree_node_baseSC_RKS4_
// type: int __fastcall(int, int, int, std::string *this)
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE9_M_insertEPSt18_Rb_tree_node_baseSC_RKS4_")]
pub fn stub_10a43c() -> ! {
    todo!("0x10a43c __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE9_M_insertEPSt18_Rb_tree_node_baseSC_RKS4_")
}

// 0x10a4c0 — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE16_M_insert_uniqueERKS4_
// type: int __fastcall(int, int, std::string *this)
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE16_M_insert_uniqueERKS4_")]
pub fn stub_10a4c0() -> ! {
    todo!("0x10a4c0 __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE16_M_insert_uniqueERKS4_")
}

// 0x10a584 — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_
// type: int __fastcall(int, int, std::string *this)
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_")]
pub fn stub_10a584() -> ! {
    todo!("0x10a584 __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_")
}

// 0x10a6e4 — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE5eraseESt17_Rb_tree_iteratorIS4_ESC_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE5eraseESt17_Rb_tree_iteratorIS4_ESC_")]
pub fn stub_10a6e4() -> ! {
    todo!("0x10a6e4 __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE5eraseESt17_Rb_tree_iteratorIS4_ESC_")
}

// 0x10a760 — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE5eraseERS1_
// type: int __fastcall(int, std::string *)
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE5eraseERS1_")]
pub fn stub_10a760() -> ! {
    todo!("0x10a760 __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE5eraseERS1_")
}

// 0x10a7a8 — __ZNSt3mapISsP5FITAGSt4lessISsESaISt4pairIKSsS1_EEEixERS5_
#[doc(alias = "__ZNSt3mapISsP5FITAGSt4lessISsESaISt4pairIKSsS1_EEEixERS5_")]
pub fn stub_10a7a8() -> ! {
    todo!("0x10a7a8 __ZNSt3mapISsP5FITAGSt4lessISsESaISt4pairIKSsS1_EEEixERS5_")
}

// 0x10a8e4 — __ZNSt3mapIiPS_ISsP5FITAGSt4lessISsESaISt4pairIKSsS1_EEES2_IiESaIS4_IKiS9_EEEixERSB_
#[doc(alias = "__ZNSt3mapIiPS_ISsP5FITAGSt4lessISsESaISt4pairIKSsS1_EEES2_IiESaIS4_IKiS9_EEEixERSB_")]
pub fn stub_10a8e4() -> ! {
    todo!("0x10a8e4 __ZNSt3mapIiPS_ISsP5FITAGSt4lessISsESaISt4pairIKSsS1_EEES2_IiESaIS4_IKiS9_EEEixERSB_")
}

// 0x10a950 — _FreeImage_ConvertLine1To24
#[doc(alias = "_FreeImage_ConvertLine1To24")]
pub fn stub_10a950() -> ! {
    todo!("0x10a950 _FreeImage_ConvertLine1To24")
}

// 0x10ab1c — _FreeImage_ConvertLine4To24
#[doc(alias = "_FreeImage_ConvertLine4To24")]
pub fn stub_10ab1c() -> ! {
    todo!("0x10ab1c _FreeImage_ConvertLine4To24")
}

// 0x10ad30 — _FreeImage_ConvertLine8To24
#[doc(alias = "_FreeImage_ConvertLine8To24")]
pub fn stub_10ad30() -> ! {
    todo!("0x10ad30 _FreeImage_ConvertLine8To24")
}

// 0x10af0c — _FreeImage_ConvertLine16To24_555
#[doc(alias = "_FreeImage_ConvertLine16To24_555")]
pub fn stub_10af0c() -> ! {
    todo!("0x10af0c _FreeImage_ConvertLine16To24_555")
}

// 0x10b0b4 — _FreeImage_ConvertLine16To24_565
#[doc(alias = "_FreeImage_ConvertLine16To24_565")]
pub fn stub_10b0b4() -> ! {
    todo!("0x10b0b4 _FreeImage_ConvertLine16To24_565")
}

// 0x10b270 — _FreeImage_ConvertLine32To24
#[doc(alias = "_FreeImage_ConvertLine32To24")]
pub fn stub_10b270() -> ! {
    todo!("0x10b270 _FreeImage_ConvertLine32To24")
}

// 0x10b4a0 — _FreeImage_ConvertTo24Bits
#[doc(alias = "_FreeImage_ConvertTo24Bits")]
pub fn stub_10b4a0() -> ! {
    todo!("0x10b4a0 _FreeImage_ConvertTo24Bits")
}

// 0x10c390 — _FreeImage_ConvertLine1To32
#[doc(alias = "_FreeImage_ConvertLine1To32")]
pub fn stub_10c390() -> ! {
    todo!("0x10c390 _FreeImage_ConvertLine1To32")
}

// 0x10c590 — _FreeImage_ConvertLine4To32
#[doc(alias = "_FreeImage_ConvertLine4To32")]
pub fn stub_10c590() -> ! {
    todo!("0x10c590 _FreeImage_ConvertLine4To32")
}

// 0x10c7c0 — _FreeImage_ConvertLine8To32
#[doc(alias = "_FreeImage_ConvertLine8To32")]
pub fn stub_10c7c0() -> ! {
    todo!("0x10c7c0 _FreeImage_ConvertLine8To32")
}

// 0x10c9c4 — _FreeImage_ConvertLine16To32_555
#[doc(alias = "_FreeImage_ConvertLine16To32_555")]
pub fn stub_10c9c4() -> ! {
    todo!("0x10c9c4 _FreeImage_ConvertLine16To32_555")
}

// 0x10cb84 — _FreeImage_ConvertLine16To32_565
#[doc(alias = "_FreeImage_ConvertLine16To32_565")]
pub fn stub_10cb84() -> ! {
    todo!("0x10cb84 _FreeImage_ConvertLine16To32_565")
}
