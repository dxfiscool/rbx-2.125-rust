//! Smoke tests for the vorbis/oggpack batch (IDA 0x6d26c..0x701fc).
//! Exercises the read path end-to-end against hand-computed expectations.

use rbx_audio::{
    NullOs, OggpackBuffer, VecHeap, VorbisBackendState, VorbisBlock, VorbisCodebook, VorbisDspState,
    VorbisHeap,
};

fn read_buffer(bytes: &[u8]) -> (OggpackBuffer, Vec<u8>) {
    let store = bytes.to_vec();
    let buf = OggpackBuffer {
        endbyte: 0,
        headbit: 0,
        buffer: store.as_ptr(),
        ptr: store.as_ptr(),
        storage: store.len() as i32,
    };
    (buf, store)
}

#[test]
fn oggpack_read_little_endian_bits() {
    // Bytes 0b1011_0001, 0b0000_1100: bit stream (LSB-first) is
    // 1,0,0,0,1,1,0,1 | 0,0,1,1,0,0,0,0.
    let (mut b, _keep) = read_buffer(&[0xB1, 0x0C]);
    unsafe {
        rbx_audio::stub_6d44c(&mut b, b.buffer, 2);
        assert_eq!(rbx_audio::stub_6d354(&mut b, 3), 0b001);
        assert_eq!(rbx_audio::stub_6d26c(&mut b, 5), 0b10_110);
        assert_eq!(rbx_audio::stub_6d354(&mut b, 5), 0b10_110);
        // 8 bits consumed of 16; bytes() rounds up.
        assert_eq!(rbx_audio::stub_6d434(&b), 1);
        assert_eq!(rbx_audio::stub_6d354(&mut b, 8), 0x0C);
        assert_eq!(rbx_audio::stub_6d434(&b), 2);
        // Past the end: -1, and the window still advances.
        assert_eq!(rbx_audio::stub_6d354(&mut b, 1), -1);
    }
}

#[test]
fn oggpack_adv_splits_bytes() {
    let (mut b, _keep) = read_buffer(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    unsafe {
        rbx_audio::stub_6d44c(&mut b, b.buffer, 4);
        rbx_audio::stub_6d318(&mut b, 10);
        assert_eq!(b.headbit, 2);
        assert_eq!(b.endbyte, 1);
        assert_eq!(rbx_audio::stub_6d26c(&mut b, 4), 0xF);
    }
}

#[test]
fn block_alloc_bump_and_ripcord() {
    let mut heap = VecHeap::default();
    let mut vb: VorbisBlock = unsafe { core::mem::zeroed() };
    unsafe {
        assert_eq!(rbx_audio::stub_6e044(&mut vb, core::ptr::null_mut()), 0);
        let a = rbx_audio::stub_6dee8(&mut vb, 16, &mut heap);
        assert!(!a.is_null());
        // Zero-size bump stays in the live segment: buf + used.
        let c = rbx_audio::stub_6dee8(&mut vb, 0, &mut heap);
        assert_eq!(c, a.add(16));
        // Force a fresh segment: the old one chains instead of leaking.
        let big = rbx_audio::stub_6dee8(&mut vb, 4096, &mut heap);
        assert!(!big.is_null());
        assert!(!vb.reap.is_null());
        assert_eq!(rbx_audio::stub_6df94(&mut vb, &mut heap), 0);
        assert!(vb.reap.is_null());
        assert_eq!(vb.localtop, 0);
        // Clearing frees the store and zeroes the block.
        assert_eq!(rbx_audio::stub_6e6c0(&mut vb, &mut heap), 0);
        assert!(vb.localstore.is_null());
    }
}

#[test]
fn book_decode_firsttable_hit() {
    // One-entry book: firsttable[0] = 1 -> length codelengths[0] = 3,
    // entry 0 -> dec_index[0] = 42.
    let lengths = [3i8];
    let first = [1i32];
    let index = [42i32];
    let bk = VorbisCodebook {
        dim: 1,
        entries: 1,
        used_entries: 1,
        valuelist: core::ptr::null(),
        codelist: core::ptr::null(),
        dec_index: index.as_ptr(),
        dec_codelengths: lengths.as_ptr() as *const u8,
        dec_firsttable: first.as_ptr(),
        dec_firsttablen: 1,
        dec_maxlength: 3,
    };
    let (mut b, _keep) = read_buffer(&[0x00]);
    unsafe {
        rbx_audio::stub_6d44c(&mut b, b.buffer, 1);
        assert_eq!(rbx_audio::stub_6e778(&bk, &mut b), 42);
        assert_eq!(b.headbit, 3);
        // Empty book resolves nothing (arrays outlive the call by scope).
        let empty = VorbisCodebook {
            used_entries: 0,
            ..bk
        };
        assert_eq!(rbx_audio::stub_6e778(&empty, &mut b), -1);
    }
}

#[test]
fn synthesis_state_transitions() {
    let mut dsp: VorbisDspState = unsafe { core::mem::zeroed() };
    let vi_words = [0i32, 2, 0, 0, 0, 0, 0, 0];
    unsafe {
        dsp.vi = vi_words.as_ptr() as *const u8;
        dsp.pcm_current = 10;
        dsp.pcm_returned = 2;
        assert_eq!(rbx_audio::stub_6d5c8(&mut dsp, 0), 0);
        assert_eq!(dsp.pcm_returned, 2);
        assert_eq!(rbx_audio::stub_6d5c8(&mut dsp, 4), 0);
        assert_eq!(dsp.pcm_returned, 6);
        // Reading past pcm_current fails without moving the cursor.
        assert_eq!(rbx_audio::stub_6d5c8(&mut dsp, 5), -131);
        assert_eq!(dsp.pcm_returned, 6);
        // pcmout reports the available window.
        let mut chans: [*mut f32; 2] = [core::ptr::null_mut(); 2];
        let mut back = vec![0.0f32; 16];
        let mut rets = [back.as_mut_ptr(), back.as_mut_ptr()];
        dsp.pcm = rets.as_ptr() as *const *mut f32;
        dsp.pcmret = rets.as_mut_ptr();
        assert_eq!(
            rbx_audio::stub_6d538(&dsp, chans.as_mut_ptr() as *mut *mut *mut f32),
            4
        );
        assert_eq!(rbx_audio::stub_6d5c8(&mut dsp, 4), 0);
        assert_eq!(rbx_audio::stub_6d538(&dsp, core::ptr::null_mut()), 0);
    }
}

#[test]
fn heap_oom_paths_are_null() {
    let mut heap = VecHeap::default();
    unsafe {
        assert!(heap.ogg_malloc(usize::MAX).is_null());
        assert!(heap.ogg_calloc(usize::MAX, 2).is_null());
        assert!(heap.ogg_realloc(core::ptr::null_mut(), 8).is_null() == false);
    }
}

/// Build the (vi, codec_setup) pair blockin reads: vi word 1 = channels,
/// vi word 7 = setup; setup words 0/1 = blocksizes, word 712 = halfrate.
struct BlockinCtx {
    vi_words: Vec<i32>,
    cs_words: Vec<i32>,
}
impl BlockinCtx {
    fn new(channels: i32, b0: i32, b1: i32, hs: i32) -> Self {
        let mut cs_words = vec![0i32; 713];
        cs_words[0] = b0;
        cs_words[1] = b1;
        cs_words[712] = hs;
        let mut vi_words = vec![0i32; 8];
        vi_words[1] = channels;
        Self { vi_words, cs_words }
    }
    unsafe fn vi_ptr(&mut self) -> *const u8 {
        let cs = self.cs_words.as_mut_ptr();
        // vi word 7 is a 4-aligned pointer slot on ARM; write it unaligned.
        ((self.vi_words.as_mut_ptr() as *mut u8).add(28) as *mut *const i32)
            .write_unaligned(cs);
        self.vi_words.as_ptr() as *const u8
    }
}

#[test]
fn blockin_rejects_null_and_stale_cursors() {
    let mut os = NullOs { heap: VecHeap::default() };
    let mut ctx = BlockinCtx::new(1, 256, 512, 0);
    let mut dsp: VorbisDspState = unsafe { core::mem::zeroed() };
    unsafe {
        dsp.vi = ctx.vi_ptr();
        // Null block is OV_EINVAL.
        assert_eq!(rbx_audio::stub_6d600(&os, &mut dsp, core::ptr::null_mut()), -131);
        // Current past a live returned cursor is OV_EINVAL.
        let mut vb: VorbisBlock = core::mem::zeroed();
        dsp.pcm_current = 10;
        dsp.pcm_returned = 2;
        assert_eq!(rbx_audio::stub_6d600(&os, &mut dsp, &mut vb), -131);
    }
}

#[test]
fn blockin_large_large_overlap_and_copy() {
    // Long block into a long dsp (lW=1, W=1) with the null (all-zero)
    // window: the overlap region zeroes, the copy section lands verbatim.
    let mut os = NullOs { heap: VecHeap::default() };
    let mut ctx = BlockinCtx::new(1, 256, 512, 0);
    let mut backend: VorbisBackendState = unsafe { core::mem::zeroed() };
    let mut dsp_pcm = vec![1.0f32; 1024];
    let mut dsp_chans = [dsp_pcm.as_mut_ptr()];
    let mut dsp: VorbisDspState = unsafe { core::mem::zeroed() };
    let mut blk_pcm = vec![2.0f32; 1024];
    let mut blk_chans = [blk_pcm.as_mut_ptr()];
    let mut vb: VorbisBlock = unsafe { core::mem::zeroed() };
    unsafe {
        dsp.vi = ctx.vi_ptr();
        dsp.pcm = dsp_chans.as_ptr();
        dsp.backend_state = &mut backend as *mut VorbisBackendState as *mut u8;
        dsp.l_w = 0;
        dsp.w = 1;
        dsp.center_w = 256; // nonzero -> this=256, prev=0
        dsp.pcm_returned = -1;
        // Fresh stream: sequence/granulepos unset, backend unset.
        dsp._w12_15 = [-1; 4];
        backend.seq_gran = -1;
        vb.pcm = blk_chans.as_mut_ptr();
        vb.w = 1;
        vb.sequence = 5;
        vb.granulepos = 100;
        assert_eq!(rbx_audio::stub_6d600(&os, &mut dsp, &mut vb), 0);
        // n1 = 512>>1 = 256: overlap dp[0..256] = 1*0 + 2*0.
        assert!(dsp_pcm[..256].iter().all(|&x| x == 0.0));
        // Copy: dp[256..512] = bp[256..512] = 2.0.
        assert!(dsp_pcm[256..512].iter().all(|&x| x == 2.0));
        assert_eq!(dsp.center_w, 0);
        assert_eq!(dsp.pcm_returned, 256);
        assert_eq!(dsp.pcm_current, 256);
        // Sequence takes the block's; granulepos follows (fresh -> block's).
        assert_eq!(dsp._w12_15[2], 5);
        assert_eq!(dsp._w12_15[0], 100);
        // Bit counters accumulate into dsp words 16..19.
        assert_eq!(dsp._w16_23[0], 0);
    }
}

#[test]
fn blockin_small_large_middle_copy() {
    // Short dsp into a long block (lW=0, W=1): zero window zeroes the
    // windowed head, the middle section copies straight, then the tail copy.
    let mut os = NullOs { heap: VecHeap::default() };
    let mut ctx = BlockinCtx::new(1, 64, 128, 0);
    let mut backend: VorbisBackendState = unsafe { core::mem::zeroed() };
    let mut dsp_pcm = vec![1.0f32; 512];
    let mut dsp_chans = [dsp_pcm.as_mut_ptr()];
    let mut blk_pcm: Vec<f32> = (0..512).map(|i| i as f32).collect();
    let mut blk_chans = [blk_pcm.as_mut_ptr()];
    let mut dsp: VorbisDspState = unsafe { core::mem::zeroed() };
    let mut vb: VorbisBlock = unsafe { core::mem::zeroed() };
    unsafe {
        dsp.vi = ctx.vi_ptr();
        dsp.pcm = dsp_chans.as_ptr();
        dsp.backend_state = &mut backend as *mut VorbisBackendState as *mut u8;
        dsp.l_w = 0;
        dsp.w = 0;
        dsp.center_w = 0; // -> this=0, prev=n1=64
        dsp.pcm_returned = -1;
        dsp._w12_15 = [-1; 4];
        backend.seq_gran = -1;
        vb.pcm = blk_chans.as_mut_ptr();
        vb.w = 1;
        vb.sequence = 7;
        vb.granulepos = 50;
        assert_eq!(rbx_audio::stub_6d600(&os, &mut dsp, &mut vb), 0);
        // n0 = 32, n1 = 64, off = 16: dp[64..96] windowed to zero.
        assert!(dsp_pcm[64..96].iter().all(|&x| x == 0.0));
        // Middle dp[96..112] = bp[48..64].
        assert_eq!(&dsp_pcm[96..112], &blk_pcm[48..64]);
        // Tail copy dp[0..64] = bp[64..128].
        assert_eq!(&dsp_pcm[0..64], &blk_pcm[64..128]);
        assert_eq!(dsp.center_w, 64);
        // Watchdog dup agrees with the primary.
        assert_eq!(dsp._w12_15[2], 7);
    }
}

#[test]
fn blockin_wd_dup_forwards() {
    let mut os = NullOs { heap: VecHeap::default() };
    let mut ctx = BlockinCtx::new(1, 256, 512, 0);
    let mut backend: VorbisBackendState = unsafe { core::mem::zeroed() };
    let mut dsp: VorbisDspState = unsafe { core::mem::zeroed() };
    unsafe {
        dsp.vi = ctx.vi_ptr();
        dsp.backend_state = &mut backend as *mut VorbisBackendState as *mut u8;
        dsp._w12_15 = [-1; 4];
        backend.seq_gran = -1;
        let mut vb: VorbisBlock = core::mem::zeroed();
        vb.sequence = 1;
        vb.granulepos = -1;
        // Null pcm skips the overlap path; granulepos stays -1 on a -1 block.
        assert_eq!(rbx_audio::stub_6d600_wd055(&os, &mut dsp, &mut vb), 0);
        assert_eq!(dsp._w12_15[2], 1);
        assert_eq!(dsp._w12_15[0], -1);
    }
}
