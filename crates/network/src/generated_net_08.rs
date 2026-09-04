//! network generated_net_08 — auto-generated, do not edit manually
//! Filter: RakNet|Network|Replicator -> 5109 complete, batch EA-sorted asc 120 gap filler (global, since filtered complete)
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +120 stubs | range 0x105cdc..0x10cb84 | 22179->22299 distinct (rbx_core::SharedPtr not boost) — preserves ea + mangled + demangled for rg

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

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
pub fn stub_107090() -> ! {
    todo!("0x107090 __ZN4FMOD15CodecAudioQueue19setPositionCallbackEP16FMOD_CODEC_STATEijj")
}

// 0x10709c — __ZN4FMOD15CodecAudioQueue13closeInternalEv
// type: _DWORD __fastcall(FMOD::CodecAudioQueue *__hidden this)
#[doc(alias = "__ZN4FMOD15CodecAudioQueue13closeInternalEv")]
pub fn stub_10709c() -> ! {
    todo!("0x10709c __ZN4FMOD15CodecAudioQueue13closeInternalEv")
}

// 0x107164 — __ZN4FMOD15CodecAudioQueue13closeCallbackEP16FMOD_CODEC_STATE
#[doc(alias = "__ZN4FMOD15CodecAudioQueue13closeCallbackEP16FMOD_CODEC_STATE")]
pub fn stub_107164() -> ! {
    todo!("0x107164 __ZN4FMOD15CodecAudioQueue13closeCallbackEP16FMOD_CODEC_STATE")
}

// 0x107170 — __ZN4FMOD15CodecAudioQueue16getDescriptionExEv
// type: _DWORD __fastcall(FMOD::CodecAudioQueue *__hidden this)
#[doc(alias = "__ZN4FMOD15CodecAudioQueue16getDescriptionExEv")]
pub fn stub_107170() -> ! {
    todo!("0x107170 __ZN4FMOD15CodecAudioQueue16getDescriptionExEv")
}

// 0x107284 — __ZN4FMOD15CodecAudioQueue15setupAudioQueueEv
// type: _DWORD __fastcall(FMOD::CodecAudioQueue *__hidden this)
#[doc(alias = "__ZN4FMOD15CodecAudioQueue15setupAudioQueueEv")]
pub fn stub_107284() -> ! {
    todo!("0x107284 __ZN4FMOD15CodecAudioQueue15setupAudioQueueEv")
}

// 0x107598 — __ZN4FMOD15CodecAudioQueue12readInternalEPvjPj
// type: _DWORD __fastcall(FMOD::CodecAudioQueue *__hidden this, void *, unsigned int, unsigned int *)
#[doc(alias = "__ZN4FMOD15CodecAudioQueue12readInternalEPvjPj")]
pub fn stub_107598() -> ! {
    todo!("0x107598 __ZN4FMOD15CodecAudioQueue12readInternalEPvjPj")
}

// 0x10773c — __ZN4FMOD15CodecAudioQueue12readCallbackEP16FMOD_CODEC_STATEPvjPj
#[doc(alias = "__ZN4FMOD15CodecAudioQueue12readCallbackEP16FMOD_CODEC_STATEPvjPj")]
pub fn stub_10773c() -> ! {
    todo!("0x10773c __ZN4FMOD15CodecAudioQueue12readCallbackEP16FMOD_CODEC_STATEPvjPj")
}

// 0x107748 — __ZN4FMOD15CodecAudioQueue12openInternalEjP22FMOD_CREATESOUNDEXINFO
#[doc(alias = "__ZN4FMOD15CodecAudioQueue12openInternalEjP22FMOD_CREATESOUNDEXINFO")]
pub fn stub_107748() -> ! {
    todo!("0x107748 __ZN4FMOD15CodecAudioQueue12openInternalEjP22FMOD_CREATESOUNDEXINFO")
}

// 0x1078d4 — __ZN4FMOD15CodecAudioQueue12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO
#[doc(alias = "__ZN4FMOD15CodecAudioQueue12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO")]
pub fn stub_1078d4() -> ! {
    todo!("0x1078d4 __ZN4FMOD15CodecAudioQueue12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO")
}

// 0x1078e0 — __ZN4FMOD15CodecAudioQueue8resetAllEbb
// type: _DWORD __fastcall(FMOD::CodecAudioQueue *__hidden this, bool, bool)
#[doc(alias = "__ZN4FMOD15CodecAudioQueue8resetAllEbb")]
pub fn stub_1078e0() -> ! {
    todo!("0x1078e0 __ZN4FMOD15CodecAudioQueue8resetAllEbb")
}

// 0x1079e8 — __Z41__static_initialization_and_destruction_0ii_41
// type: _DWORD __fastcall(int, int)
#[doc(alias = "__Z41__static_initialization_and_destruction_0ii_41")]
pub fn stub_1079e8() -> ! {
    todo!("0x1079e8 __Z41__static_initialization_and_destruction_0ii_41")
}

// 0x107a1c — __GLOBAL__I__ZN4FMOD15CodecAudioQueue10gCodecHeadE
#[doc(alias = "__GLOBAL__I__ZN4FMOD15CodecAudioQueue10gCodecHeadE")]
pub fn stub_107a1c() -> ! {
    todo!("0x107a1c __GLOBAL__I__ZN4FMOD15CodecAudioQueue10gCodecHeadE")
}

// 0x107a28 — __ZL22FreeImage_GetImageSizeiii
// type: _DWORD __fastcall(int, int, int)
#[doc(alias = "__ZL22FreeImage_GetImageSizeiii")]
pub fn stub_107a28() -> ! {
    todo!("0x107a28 __ZL22FreeImage_GetImageSizeiii")
}

// 0x107a78 — _FreeImage_GetImageType
#[doc(alias = "_FreeImage_GetImageType")]
pub fn stub_107a78() -> ! {
    todo!("0x107a78 _FreeImage_GetImageType")
}

// 0x107a88 — _FreeImage_GetRedMask
#[doc(alias = "_FreeImage_GetRedMask")]
pub fn stub_107a88() -> ! {
    todo!("0x107a88 _FreeImage_GetRedMask")
}

// 0x107a98 — _FreeImage_GetGreenMask
#[doc(alias = "_FreeImage_GetGreenMask")]
pub fn stub_107a98() -> ! {
    todo!("0x107a98 _FreeImage_GetGreenMask")
}

// 0x107aa8 — _FreeImage_GetBlueMask
#[doc(alias = "_FreeImage_GetBlueMask")]
pub fn stub_107aa8() -> ! {
    todo!("0x107aa8 _FreeImage_GetBlueMask")
}

// 0x107ab8 — _FreeImage_HasBackgroundColor
#[doc(alias = "_FreeImage_HasBackgroundColor")]
pub fn stub_107ab8() -> ! {
    todo!("0x107ab8 _FreeImage_HasBackgroundColor")
}

// 0x107ad4 — _FreeImage_GetTransparencyTable
#[doc(alias = "_FreeImage_GetTransparencyTable")]
pub fn stub_107ad4() -> ! {
    todo!("0x107ad4 _FreeImage_GetTransparencyTable")
}

// 0x107ae4 — _FreeImage_GetTransparencyCount
// type: int __fastcall(int result)
#[doc(alias = "_FreeImage_GetTransparencyCount")]
pub fn stub_107ae4() -> ! {
    todo!("0x107ae4 _FreeImage_GetTransparencyCount")
}

// 0x107af4 — _FreeImage_GetICCProfile
#[doc(alias = "_FreeImage_GetICCProfile")]
pub fn stub_107af4() -> ! {
    todo!("0x107af4 _FreeImage_GetICCProfile")
}

// 0x107b04 — _FreeImage_GetInfoHeader
// type: int(void)
#[doc(alias = "_FreeImage_GetInfoHeader")]
pub fn stub_107b04() -> ! {
    todo!("0x107b04 _FreeImage_GetInfoHeader")
}

// 0x107b28 — _FreeImage_SetDotsPerMeterY
#[doc(alias = "_FreeImage_SetDotsPerMeterY")]
pub fn stub_107b28() -> ! {
    todo!("0x107b28 _FreeImage_SetDotsPerMeterY")
}

// 0x107b48 — _FreeImage_SetDotsPerMeterX
#[doc(alias = "_FreeImage_SetDotsPerMeterX")]
pub fn stub_107b48() -> ! {
    todo!("0x107b48 _FreeImage_SetDotsPerMeterX")
}

// 0x107b68 — _FreeImage_GetDotsPerMeterY
#[doc(alias = "_FreeImage_GetDotsPerMeterY")]
pub fn stub_107b68() -> ! {
    todo!("0x107b68 _FreeImage_GetDotsPerMeterY")
}

// 0x107b88 — _FreeImage_GetDotsPerMeterX
#[doc(alias = "_FreeImage_GetDotsPerMeterX")]
pub fn stub_107b88() -> ! {
    todo!("0x107b88 _FreeImage_GetDotsPerMeterX")
}

// 0x107ba8 — _FreeImage_GetColorsUsed
#[doc(alias = "_FreeImage_GetColorsUsed")]
pub fn stub_107ba8() -> ! {
    todo!("0x107ba8 _FreeImage_GetColorsUsed")
}

// 0x107bc8 — _FreeImage_GetBPP
// type: int(void)
#[doc(alias = "_FreeImage_GetBPP")]
pub fn stub_107bc8() -> ! {
    todo!("0x107bc8 _FreeImage_GetBPP")
}

// 0x107be8 — _FreeImage_GetPalette
#[doc(alias = "_FreeImage_GetPalette")]
pub fn stub_107be8() -> ! {
    todo!("0x107be8 _FreeImage_GetPalette")
}

// 0x107c1c — _FreeImage_SetTransparent
#[doc(alias = "_FreeImage_SetTransparent")]
pub fn stub_107c1c() -> ! {
    todo!("0x107c1c _FreeImage_SetTransparent")
}

// 0x107c60 — _FreeImage_GetHeight
// type: int __fastcall(int)
#[doc(alias = "_FreeImage_GetHeight")]
pub fn stub_107c60() -> ! {
    todo!("0x107c60 _FreeImage_GetHeight")
}

// 0x107c80 — _FreeImage_GetWidth
#[doc(alias = "_FreeImage_GetWidth")]
pub fn stub_107c80() -> ! {
    todo!("0x107c80 _FreeImage_GetWidth")
}

// 0x107ca0 — _FreeImage_GetLine
#[doc(alias = "_FreeImage_GetLine")]
pub fn stub_107ca0() -> ! {
    todo!("0x107ca0 _FreeImage_GetLine")
}

// 0x107cd4 — _FreeImage_GetPitch
#[doc(alias = "_FreeImage_GetPitch")]
pub fn stub_107cd4() -> ! {
    todo!("0x107cd4 _FreeImage_GetPitch")
}

// 0x107cf8 — _FreeImage_GetBackgroundColor
#[doc(alias = "_FreeImage_GetBackgroundColor")]
pub fn stub_107cf8() -> ! {
    todo!("0x107cf8 _FreeImage_GetBackgroundColor")
}

// 0x10813c — _FreeImage_FindCloseMetadata
#[doc(alias = "_FreeImage_FindCloseMetadata")]
pub fn stub_10813c() -> ! {
    todo!("0x10813c _FreeImage_FindCloseMetadata")
}

// 0x108168 — _FreeImage_DestroyICCProfile
#[doc(alias = "_FreeImage_DestroyICCProfile")]
pub fn stub_108168() -> ! {
    todo!("0x108168 _FreeImage_DestroyICCProfile")
}

// 0x1081a0 — __Z22FreeImage_Aligned_FreePv
// type: _DWORD __fastcall(void *)
#[doc(alias = "__Z22FreeImage_Aligned_FreePv")]
pub fn stub_1081a0() -> ! {
    todo!("0x1081a0 __Z22FreeImage_Aligned_FreePv")
}

// 0x1081b4 — _FreeImage_CreateICCProfile
#[doc(alias = "_FreeImage_CreateICCProfile")]
pub fn stub_1081b4() -> ! {
    todo!("0x1081b4 _FreeImage_CreateICCProfile")
}

// 0x108220 — _FreeImage_SetTransparencyTable
// type: void *__fastcall(void *result, const void *, size_t)
#[doc(alias = "_FreeImage_SetTransparencyTable")]
pub fn stub_108220() -> ! {
    todo!("0x108220 _FreeImage_SetTransparencyTable")
}

// 0x108290 — _FreeImage_SetBackgroundColor
#[doc(alias = "_FreeImage_SetBackgroundColor")]
pub fn stub_108290() -> ! {
    todo!("0x108290 _FreeImage_SetBackgroundColor")
}

// 0x1082dc — _FreeImage_GetColorType
#[doc(alias = "_FreeImage_GetColorType")]
pub fn stub_1082dc() -> ! {
    todo!("0x1082dc _FreeImage_GetColorType")
}

// 0x108858 — _FreeImage_IsTransparent
#[doc(alias = "_FreeImage_IsTransparent")]
pub fn stub_108858() -> ! {
    todo!("0x108858 _FreeImage_IsTransparent")
}

// 0x1088a4 — __Z24FreeImage_Aligned_Mallocmm
// type: _DWORD __fastcall(unsigned int, unsigned int)
#[doc(alias = "__Z24FreeImage_Aligned_Mallocmm")]
pub fn stub_1088a4() -> ! {
    todo!("0x1088a4 __Z24FreeImage_Aligned_Mallocmm")
}

// 0x1088fc — _FreeImage_AllocateT
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "_FreeImage_AllocateT")]
pub fn stub_1088fc() -> ! {
    todo!("0x1088fc _FreeImage_AllocateT")
}

// 0x108afc — _FreeImage_Allocate
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "_FreeImage_Allocate")]
pub fn stub_108afc() -> ! {
    todo!("0x108afc _FreeImage_Allocate")
}

// 0x108b40 — _FreeImage_Unload
#[doc(alias = "_FreeImage_Unload")]
pub fn stub_108b40() -> ! {
    todo!("0x108b40 _FreeImage_Unload")
}

// 0x108cdc — _FreeImage_FindNextMetadata
#[doc(alias = "_FreeImage_FindNextMetadata")]
pub fn stub_108cdc() -> ! {
    todo!("0x108cdc _FreeImage_FindNextMetadata")
}

// 0x108e98 — _FreeImage_GetMetadataCount
#[doc(alias = "_FreeImage_GetMetadataCount")]
pub fn stub_108e98() -> ! {
    todo!("0x108e98 _FreeImage_GetMetadataCount")
}

// 0x108f00 — _FreeImage_GetMetadata
#[doc(alias = "_FreeImage_GetMetadata")]
pub fn stub_108f00() -> ! {
    todo!("0x108f00 _FreeImage_GetMetadata")
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
