//! Smoke tests for the vorbis/oggpack batch (IDA 0x6d26c..0x701fc).
//! Exercises the read path end-to-end against hand-computed expectations.

use rbx_audio::{
    OggpackBuffer, VecHeap, VorbisBlock, VorbisCodebook, VorbisDspState, VorbisHeap,
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
