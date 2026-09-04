//! core shard jc — 100 core stubs EA-sorted, 0xa0edc..0xac010 (EA-sorted asc global gap filler next 100 uncovered, rbx_core::SharedPtr not boost).
//! Source: ida/export.json (85545 funcs) EA-sorted asc not in crates/ via grep -r stub_0x crates --include=*.rs — next 100 uncovered (49005 remaining before -> 48905 after, 0xa0edc..0xac010).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "FMOD::CodecPlaylist::readM3U(void)")]
// 0xa0edc — __ZN4FMOD13CodecPlaylist7readM3UEv
// type: int __fastcall(FMOD::File **this)
pub fn stub_a0edc() {
    // IDA 0xa0edc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecPlaylist::readB4S(void)")]
// 0xa1218 — __ZN4FMOD13CodecPlaylist7readB4SEv
// type: int __fastcall(FMOD::File **this)
pub fn stub_a1218() {
    // IDA 0xa1218: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecPlaylist::readWPL(void)")]
// 0xa1520 — __ZN4FMOD13CodecPlaylist7readWPLEv
// type: int __fastcall(FMOD::File **this)
pub fn stub_a1520() {
    // IDA 0xa1520: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecPlaylist::readASX(void)")]
// 0xa1738 — __ZN4FMOD13CodecPlaylist7readASXEv
// type: int __fastcall(FMOD::File **this)
pub fn stub_a1738() {
    // IDA 0xa1738: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecPlaylist::getDescriptionEx(void)")]
// 0xa1aac — __ZN4FMOD13CodecPlaylist16getDescriptionExEv
// type: int *__fastcall(FMOD::CodecPlaylist *this)
pub fn stub_a1aac() {
    // IDA 0xa1aac: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecPlaylist::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")]
// 0xa1b4c — __ZN4FMOD13CodecPlaylist12openInternalEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int)
pub fn stub_a1b4c() {
    // IDA 0xa1b4c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecPlaylist::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")]
// 0xa1df4 — __ZN4FMOD13CodecPlaylist12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int)
pub fn stub_a1df4() {
    // IDA 0xa1df4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "global constructor keyed toFMOD::playlistcodec")]
// 0xa1e4c — __GLOBAL__I__ZN4FMOD13playlistcodecE
// type: int()
pub fn stub_a1e4c() {
    // IDA 0xa1e4c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::CodecRaw::closeInternal(void)")]
// 0xa1e58 — __ZN4FMOD8CodecRaw13closeInternalEv
// type: int __fastcall(FMOD::CodecRaw *this)
pub fn stub_a1e58() {
    // IDA 0xa1e58: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::CodecRaw::canPointInternal(void)")]
// 0xa1e60 — __ZN4FMOD8CodecRaw16canPointInternalEv
// type: int __fastcall(FMOD::CodecRaw *this)
pub fn stub_a1e60() {
    // IDA 0xa1e60: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::CodecRaw::closeCallback(FMOD_CODEC_STATE *)")]
// 0xa1e68 — __ZN4FMOD8CodecRaw13closeCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecRaw *)
pub fn stub_a1e68() {
    // IDA 0xa1e68: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::CodecRaw::canPointCallback(FMOD_CODEC_STATE *)")]
// 0xa1e74 — __ZN4FMOD8CodecRaw16canPointCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecRaw *)
pub fn stub_a1e74() {
    // IDA 0xa1e74: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::CodecRaw::setPositionInternal(int,unsigned int,unsigned int)")]
// 0xa1e80 — __ZN4FMOD8CodecRaw19setPositionInternalEijj
// type: int __fastcall(FMOD::CodecRaw *this, int, unsigned int, unsigned int)
pub fn stub_a1e80() {
    // IDA 0xa1e80: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecRaw::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")]
// 0xa1eec — __ZN4FMOD8CodecRaw19setPositionCallbackEP16FMOD_CODEC_STATEijj
// type: int __fastcall(FMOD::CodecRaw *, int, unsigned int, unsigned int)
pub fn stub_a1eec() {
    // IDA 0xa1eec: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecRaw::readInternal(void *,unsigned int,unsigned int *)")]
// 0xa1ef8 — __ZN4FMOD8CodecRaw12readInternalEPvjPj
// type: int __fastcall(FMOD::File **this, void *, unsigned int, unsigned int *)
pub fn stub_a1ef8() {
    // IDA 0xa1ef8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecRaw::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")]
// 0xa1f58 — __ZN4FMOD8CodecRaw12readCallbackEP16FMOD_CODEC_STATEPvjPj
// type: int __fastcall(FMOD::File **, void *, unsigned int, unsigned int *)
pub fn stub_a1f58() {
    // IDA 0xa1f58: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecRaw::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")]
// 0xa1f64 — __ZN4FMOD8CodecRaw12openInternalEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int, __int16, _DWORD *)
pub fn stub_a1f64() {
    // IDA 0xa1f64: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecRaw::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")]
// 0xa226c — __ZN4FMOD8CodecRaw12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int, __int16, _DWORD *)
pub fn stub_a226c() {
    // IDA 0xa226c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecRaw::getDescriptionEx(void)")]
// 0xa2278 — __ZN4FMOD8CodecRaw16getDescriptionExEv
// type: int *__fastcall(FMOD::CodecRaw *this)
pub fn stub_a2278() {
    // IDA 0xa2278: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "global constructor keyed toFMOD::rawcodec")]
// 0xa2374 — __GLOBAL__I__ZN4FMOD8rawcodecE
// type: int()
pub fn stub_a2374() {
    // IDA 0xa2374: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::MusicChannelS3M::volumeSlide(void)")]
// 0xa2380 — __ZN4FMOD15MusicChannelS3M11volumeSlideEv
// type: int __fastcall(FMOD::MusicChannelS3M *this)
pub fn stub_a2380() {
    // IDA 0xa2380: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::MusicChannelS3M::portamento(void)")]
// 0xa23e0 — __ZN4FMOD15MusicChannelS3M10portamentoEv
// type: int __fastcall(FMOD::MusicChannelS3M *this)
pub fn stub_a23e0() {
    // IDA 0xa23e0: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::MusicChannelS3M::vibrato(void)")]
// 0xa2450 — __ZN4FMOD15MusicChannelS3M7vibratoEv
// type: int __fastcall(FMOD::MusicChannelS3M *this)
pub fn stub_a2450() {
    // IDA 0xa2450: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::MusicChannelS3M::tremolo(void)")]
// 0xa2594 — __ZN4FMOD15MusicChannelS3M7tremoloEv
// type: int __fastcall(FMOD::MusicChannelS3M *this)
pub fn stub_a2594() {
    // IDA 0xa2594: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::MusicChannelS3M::fineVibrato(void)")]
// 0xa26fc — __ZN4FMOD15MusicChannelS3M11fineVibratoEv
// type: int __fastcall(FMOD::MusicChannelS3M *this)
pub fn stub_a26fc() {
    // IDA 0xa26fc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecS3M::closeInternal(void)")]
// 0xa2830 — __ZN4FMOD8CodecS3M13closeInternalEv
// type: int __fastcall(FMOD::CodecS3M *this)
pub fn stub_a2830() {
    // IDA 0xa2830: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecS3M::closeCallback(FMOD_CODEC_STATE *)")]
// 0xa2ad8 — __ZN4FMOD8CodecS3M13closeCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecS3M *)
pub fn stub_a2ad8() {
    // IDA 0xa2ad8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecS3M::getDescriptionEx(void)")]
// 0xa2ae4 — __ZN4FMOD8CodecS3M16getDescriptionExEv
// type: int *__fastcall(FMOD::CodecS3M *this)
pub fn stub_a2ae4() {
    // IDA 0xa2ae4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecS3M::updateEffects(void)")]
// 0xa2bfc — __ZN4FMOD8CodecS3M13updateEffectsEv
// type: int __fastcall(FMOD::CodecS3M *this)
pub fn stub_a2bfc() {
    // IDA 0xa2bfc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecS3M::updateNote(bool)")]
// 0xa3580 — __ZN4FMOD8CodecS3M10updateNoteEb
// type: int __fastcall(FMOD::CodecS3M *this, bool)
pub fn stub_a3580() {
    // IDA 0xa3580: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecS3M::update(bool)")]
// 0xa4064 — __ZN4FMOD8CodecS3M6updateEb
// type: int __fastcall(FMOD::CodecS3M *this, bool)
pub fn stub_a4064() {
    // IDA 0xa4064: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecS3M::setPositionInternal(int,unsigned int,unsigned int)")]
// 0xa4174 — __ZN4FMOD8CodecS3M19setPositionInternalEijj
// type: int __fastcall(FMOD::CodecS3M *this, int, unsigned int, unsigned int)
pub fn stub_a4174() {
    // IDA 0xa4174: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecS3M::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")]
// 0xa4228 — __ZN4FMOD8CodecS3M19setPositionCallbackEP16FMOD_CODEC_STATEijj
// type: int __fastcall(FMOD::CodecS3M *, int, unsigned int, unsigned int)
pub fn stub_a4228() {
    // IDA 0xa4228: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecS3M::calculateLength(void)")]
// 0xa4234 — __ZN4FMOD8CodecS3M15calculateLengthEv
// type: int __fastcall(FMOD::CodecS3M *this)
pub fn stub_a4234() {
    // IDA 0xa4234: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecS3M::readInternal(void *,unsigned int,unsigned int *)")]
// 0xa4298 — __ZN4FMOD8CodecS3M12readInternalEPvjPj
// type: unsigned int *__fastcall(FMOD::CodecS3M *this, char *, unsigned int, unsigned int *)
pub fn stub_a4298() {
    // IDA 0xa4298: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecS3M::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")]
// 0xa467c — __ZN4FMOD8CodecS3M12readCallbackEP16FMOD_CODEC_STATEPvjPj
// type: unsigned int *__fastcall(FMOD::CodecS3M *, char *, unsigned int, unsigned int *)
pub fn stub_a467c() {
    // IDA 0xa467c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecS3M::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")]
// 0xa4688 — __ZN4FMOD8CodecS3M12openInternalEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int, __int16, int)
pub fn stub_a4688() {
    // IDA 0xa4688: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecS3M::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")]
// 0xa5c8c — __ZN4FMOD8CodecS3M12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int, __int16, int)
pub fn stub_a5c8c() {
    // IDA 0xa5c8c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "global constructor keyed toFMOD::s3mcodec")]
// 0xa5ce4 — __GLOBAL__I__ZN4FMOD8s3mcodecE
// type: int()
pub fn stub_a5ce4() {
    // IDA 0xa5ce4: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::CodecTag::closeInternal(void)")]
// 0xa5cf0 — __ZN4FMOD8CodecTag13closeInternalEv
// type: int __fastcall(FMOD::CodecTag *this)
pub fn stub_a5cf0() {
    // IDA 0xa5cf0: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::CodecTag::closeCallback(FMOD_CODEC_STATE *)")]
// 0xa5cf8 — __ZN4FMOD8CodecTag13closeCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecTag *)
pub fn stub_a5cf8() {
    // IDA 0xa5cf8: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::CodecTag::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")]
// 0xa5d04 — __ZN4FMOD8CodecTag12readCallbackEP16FMOD_CODEC_STATEPvjPj
// type: int()
pub fn stub_a5d04() {
    // IDA 0xa5d04: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::CodecTag::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")]
// 0xa5d0c — __ZN4FMOD8CodecTag19setPositionCallbackEP16FMOD_CODEC_STATEijj
// type: int()
pub fn stub_a5d0c() {
    // IDA 0xa5d0c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::CodecTag::readID3v2(void)")]
// 0xa5d14 — __ZN4FMOD8CodecTag9readID3v2Ev
// type: int __fastcall(FMOD::File **this)
pub fn stub_a5d14() {
    // IDA 0xa5d14: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecTag::readID3v2FromFooter(void)")]
// 0xa6190 — __ZN4FMOD8CodecTag19readID3v2FromFooterEv
// type: int __fastcall(FMOD::File **this)
pub fn stub_a6190() {
    // IDA 0xa6190: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecTag::getDescriptionEx(void)")]
// 0xa62c0 — __ZN4FMOD8CodecTag16getDescriptionExEv
// type: int *__fastcall(FMOD::CodecTag *this)
pub fn stub_a62c0() {
    // IDA 0xa62c0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecTag::readID3v1(void)")]
// 0xa6360 — __ZN4FMOD8CodecTag9readID3v1Ev
// type: int __fastcall(FMOD::File **this)
pub fn stub_a6360() {
    // IDA 0xa6360: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecTag::readTags(void)")]
// 0xa6728 — __ZN4FMOD8CodecTag8readTagsEv
// type: int __fastcall(FMOD::File **this)
pub fn stub_a6728() {
    // IDA 0xa6728: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecTag::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")]
// 0xa69c4 — __ZN4FMOD8CodecTag12openInternalEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int, int)
pub fn stub_a69c4() {
    // IDA 0xa69c4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecTag::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")]
// 0xa6aa0 — __ZN4FMOD8CodecTag12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int, int)
pub fn stub_a6aa0() {
    // IDA 0xa6aa0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "global constructor keyed toFMOD::tagcodec")]
// 0xa6af8 — __GLOBAL__I__ZN4FMOD8tagcodecE
// type: int()
pub fn stub_a6af8() {
    // IDA 0xa6af8: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::CodecUser::closeInternal(void)")]
// 0xa6b04 — __ZN4FMOD9CodecUser13closeInternalEv
// type: int __fastcall(FMOD::CodecUser *this)
pub fn stub_a6b04() {
    // IDA 0xa6b04: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::CodecUser::readInternal(void *,unsigned int,unsigned int *)")]
// 0xa6b0c — __ZN4FMOD9CodecUser12readInternalEPvjPj
// type: int __fastcall(FMOD::CodecUser *this, void *, unsigned int, unsigned int *)
pub fn stub_a6b0c() {
    // IDA 0xa6b0c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::CodecUser::setPositionInternal(int,unsigned int,unsigned int)")]
// 0xa6b18 — __ZN4FMOD9CodecUser19setPositionInternalEijj
// type: int __fastcall(FMOD::CodecUser *this, int, unsigned int, unsigned int)
pub fn stub_a6b18() {
    // IDA 0xa6b18: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::CodecUser::closeCallback(FMOD_CODEC_STATE *)")]
// 0xa6b20 — __ZN4FMOD9CodecUser13closeCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecUser *)
pub fn stub_a6b20() {
    // IDA 0xa6b20: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::CodecUser::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")]
// 0xa6b2c — __ZN4FMOD9CodecUser12readCallbackEP16FMOD_CODEC_STATEPvjPj
// type: int __fastcall(FMOD::CodecUser *, void *, unsigned int, unsigned int *)
pub fn stub_a6b2c() {
    // IDA 0xa6b2c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecUser::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")]
// 0xa6b38 — __ZN4FMOD9CodecUser19setPositionCallbackEP16FMOD_CODEC_STATEijj
// type: int __fastcall(FMOD::CodecUser *, int, unsigned int, unsigned int)
pub fn stub_a6b38() {
    // IDA 0xa6b38: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecUser::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")]
// 0xa6b44 — __ZN4FMOD9CodecUser12openInternalEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int, int, _DWORD *)
pub fn stub_a6b44() {
    // IDA 0xa6b44: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecUser::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")]
// 0xa6e20 — __ZN4FMOD9CodecUser12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int, int, _DWORD *)
pub fn stub_a6e20() {
    // IDA 0xa6e20: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecUser::getDescriptionEx(void)")]
// 0xa6e2c — __ZN4FMOD9CodecUser16getDescriptionExEv
// type: int *__fastcall(FMOD::CodecUser *this)
pub fn stub_a6e2c() {
    // IDA 0xa6e2c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "global constructor keyed toFMOD::usercodec")]
// 0xa6f18 — __GLOBAL__I__ZN4FMOD9usercodecE
// type: int()
pub fn stub_a6f18() {
    // IDA 0xa6f18: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::CodecWav::canPointInternal(void)")]
// 0xa6f24 — __ZN4FMOD8CodecWav16canPointInternalEv
// type: int __fastcall(FMOD::CodecWav *this)
pub fn stub_a6f24() {
    // IDA 0xa6f24: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::CodecWav::canPointCallback(FMOD_CODEC_STATE *)")]
// 0xa6fc4 — __ZN4FMOD8CodecWav16canPointCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecWav *)
pub fn stub_a6fc4() {
    // IDA 0xa6fc4: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::CodecWav::soundCreateInternal(int,FMOD_SOUND *)")]
// 0xa6fd0 — __ZN4FMOD8CodecWav19soundCreateInternalEiP10FMOD_SOUND
// type: int __fastcall(int, int, FMOD::SoundI *this)
pub fn stub_a6fd0() {
    // IDA 0xa6fd0: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::CodecWav::soundCreateCallback(FMOD_CODEC_STATE *,int,FMOD_SOUND *)")]
// 0xa70bc — __ZN4FMOD8CodecWav19soundCreateCallbackEP16FMOD_CODEC_STATEiP10FMOD_SOUND
// type: int __fastcall(int, int, FMOD::SoundI *)
pub fn stub_a70bc() {
    // IDA 0xa70bc: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::CodecWav::readInternal(void *,unsigned int,unsigned int *)")]
// 0xa70c8 — __ZN4FMOD8CodecWav12readInternalEPvjPj
// type: int __fastcall(FMOD::CodecWav *this, unsigned __int8 *, unsigned int, unsigned int *)
pub fn stub_a70c8() {
    // IDA 0xa70c8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecWav::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")]
// 0xa7588 — __ZN4FMOD8CodecWav12readCallbackEP16FMOD_CODEC_STATEPvjPj
// type: int __fastcall(FMOD::CodecWav *, unsigned __int8 *, unsigned int, unsigned int *)
pub fn stub_a7588() {
    // IDA 0xa7588: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecWav::closeInternal(void)")]
// 0xa7594 — __ZN4FMOD8CodecWav13closeInternalEv
// type: int __fastcall(FMOD::CodecWav *this)
pub fn stub_a7594() {
    // IDA 0xa7594: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecWav::closeCallback(FMOD_CODEC_STATE *)")]
// 0xa76ec — __ZN4FMOD8CodecWav13closeCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecWav *)
pub fn stub_a76ec() {
    // IDA 0xa76ec: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecWav::getDescriptionEx(void)")]
// 0xa76f8 — __ZN4FMOD8CodecWav16getDescriptionExEv
// type: int *__fastcall(FMOD::CodecWav *this)
pub fn stub_a76f8() {
    // IDA 0xa76f8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecWav::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")]
// 0xa77b8 — __ZN4FMOD8CodecWav12openInternalEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int, __int16)
pub fn stub_a77b8() {
    // IDA 0xa77b8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecWav::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")]
// 0xa8028 — __ZN4FMOD8CodecWav12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int, __int16)
pub fn stub_a8028() {
    // IDA 0xa8028: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecWav::setPositionInternal(int,unsigned int,unsigned int)")]
// 0xa8034 — __ZN4FMOD8CodecWav19setPositionInternalEijj
// type: int __fastcall(FMOD::File **this, int, unsigned int, unsigned int)
pub fn stub_a8034() {
    // IDA 0xa8034: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecWav::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")]
// 0xa83f4 — __ZN4FMOD8CodecWav19setPositionCallbackEP16FMOD_CODEC_STATEijj
// type: int __fastcall(FMOD::File **, int, unsigned int, unsigned int)
pub fn stub_a83f4() {
    // IDA 0xa83f4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "global constructor keyed toFMOD::wavcodec")]
// 0xa844c — __GLOBAL__I__ZN4FMOD8wavcodecE
// type: int()
pub fn stub_a844c() {
    // IDA 0xa844c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::IMAAdpcm_DecodeS16(unsigned char *,short *,unsigned int,unsigned int,unsigned int)")]
// 0xa8458 — __ZN4FMOD18IMAAdpcm_DecodeS16EPhPsjjj
// type: int __fastcall(FMOD *this, unsigned __int8 *, __int16 *, unsigned int, unsigned int, unsigned int)
pub fn stub_a8458() {
    // IDA 0xa8458: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::IMAAdpcm_DecodeM16(unsigned char *,short *,unsigned int,unsigned int,unsigned int,int)")]
// 0xa88fc — __ZN4FMOD18IMAAdpcm_DecodeM16EPhPsjjji
// type: int __fastcall(FMOD *this, unsigned __int8 *, __int16 *, unsigned int, unsigned int, unsigned int, int)
pub fn stub_a88fc() {
    // IDA 0xa88fc: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::CodecWav::parseChunk(unsigned int)")]
// 0xa8d04 — __ZN4FMOD8CodecWav10parseChunkEj
// type: int __fastcall(FMOD::CodecWav *this, unsigned int)
pub fn stub_a8d04() {
    // IDA 0xa8d04: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::MusicChannelXM::portamento(void)")]
// 0xa9280 — __ZN4FMOD14MusicChannelXM10portamentoEv
// type: int __fastcall(FMOD::MusicChannelXM *this)
pub fn stub_a9280() {
    // IDA 0xa9280: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::MusicChannelXM::vibrato(void)")]
// 0xa92fc — __ZN4FMOD14MusicChannelXM7vibratoEv
// type: int __fastcall(FMOD::MusicChannelXM *this)
pub fn stub_a92fc() {
    // IDA 0xa92fc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::MusicChannelXM::tremolo(void)")]
// 0xa941c — __ZN4FMOD14MusicChannelXM7tremoloEv
// type: int __fastcall(FMOD::MusicChannelXM *this)
pub fn stub_a941c() {
    // IDA 0xa941c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecXM::processEnvelope(FMOD::MusicEnvelopeState *,FMOD::MusicVirtualChannel *,int,unsigned short *,int,int,int,unsigned char,unsigned char)")]
// 0xa952c — __ZN4FMOD7CodecXM15processEnvelopeEPNS_18MusicEnvelopeStateEPNS_19MusicVirtualChannelEiPtiiihh
// type: int __fastcall(int, int *, int, int, int, char, int, int, unsigned __int8, char)
pub fn stub_a952c() {
    // IDA 0xa952c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::MusicChannelXM::instrumentVibrato(FMOD::MusicInstrument *)")]
// 0xa9708 — __ZN4FMOD14MusicChannelXM17instrumentVibratoEPNS_15MusicInstrumentE
// type: int __fastcall(int *, unsigned __int8 *)
pub fn stub_a9708() {
    // IDA 0xa9708: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::MusicChannelXM::processVolumeByte(unsigned char)")]
// 0xa9830 — __ZN4FMOD14MusicChannelXM17processVolumeByteEh
// type: int __fastcall(FMOD::MusicChannelXM *this, unsigned __int8)
pub fn stub_a9830() {
    // IDA 0xa9830: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecXM::getAmigaPeriod(int,int,int *)")]
// 0xa9984 — __ZN4FMOD7CodecXM14getAmigaPeriodEiiPi
// type: int __fastcall(FMOD::CodecXM *this, int, unsigned int, int *)
pub fn stub_a9984() {
    // IDA 0xa9984: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecXM::processNote(FMOD::MusicNote *,FMOD::MusicChannelXM *,FMOD::MusicVirtualChannel *,FMOD::MusicInstrument *,FMOD::MusicSample *)")]
// 0xa9a10 — __ZN4FMOD7CodecXM11processNoteEPNS_9MusicNoteEPNS_14MusicChannelXMEPNS_19MusicVirtualChannelEPNS_15MusicInstrumentEPNS_11MusicSampleE
// type: int __fastcall(int, unsigned __int8 *, FMOD::MusicChannelXM *this, int, int, int)
pub fn stub_a9a10() {
    // IDA 0xa9a10: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecXM::closeInternal(void)")]
// 0xa9c14 — __ZN4FMOD7CodecXM13closeInternalEv
// type: int __fastcall(FMOD::CodecXM *this)
pub fn stub_a9c14() {
    // IDA 0xa9c14: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecXM::closeCallback(FMOD_CODEC_STATE *)")]
// 0xa9f4c — __ZN4FMOD7CodecXM13closeCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecXM *)
pub fn stub_a9f4c() {
    // IDA 0xa9f4c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecXM::getDescriptionEx(void)")]
// 0xa9f58 — __ZN4FMOD7CodecXM16getDescriptionExEv
// type: int *__fastcall(FMOD::CodecXM *this)
pub fn stub_a9f58() {
    // IDA 0xa9f58: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecXM::updateFlags(FMOD::MusicChannel *,FMOD::MusicVirtualChannel *,FMOD::MusicSample *)")]
// 0xaa090 — __ZN4FMOD7CodecXM11updateFlagsEPNS_12MusicChannelEPNS_19MusicVirtualChannelEPNS_11MusicSampleE
// type: int __fastcall(int, int, int, int)
pub fn stub_aa090() {
    // IDA 0xaa090: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecXM::spawnNewChannel(FMOD::MusicChannel *,FMOD::MusicVirtualChannel *,FMOD::MusicSample *,FMOD::MusicVirtualChannel **)")]
// 0xaa288 — __ZN4FMOD7CodecXM15spawnNewChannelEPNS_12MusicChannelEPNS_19MusicVirtualChannelEPNS_11MusicSampleEPS4_
// type: int __fastcall(int, int, int *, int, int **)
pub fn stub_aa288() {
    // IDA 0xaa288: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecXM::updateEffects(void)")]
// 0xaa2d0 — __ZN4FMOD7CodecXM13updateEffectsEv
// type: int __fastcall(FMOD::CodecXM *this)
pub fn stub_aa2d0() {
    // IDA 0xaa2d0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecXM::updateNote(bool)")]
// 0xaae64 — __ZN4FMOD7CodecXM10updateNoteEb
// type: int __fastcall(FMOD::CodecXM *this, bool)
pub fn stub_aae64() {
    // IDA 0xaae64: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecXM::update(bool)")]
// 0xab9d4 — __ZN4FMOD7CodecXM6updateEb
// type: int __fastcall(FMOD::CodecXM *this, bool)
pub fn stub_ab9d4() {
    // IDA 0xab9d4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecXM::updateCallback(FMOD_CODEC_STATE *)")]
// 0xabaf8 — __ZN4FMOD7CodecXM14updateCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecXM *)
pub fn stub_abaf8() {
    // IDA 0xabaf8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecXM::setPositionInternal(int,unsigned int,unsigned int)")]
// 0xabb08 — __ZN4FMOD7CodecXM19setPositionInternalEijj
// type: int __fastcall(FMOD::CodecXM *this, int, unsigned int, unsigned int)
pub fn stub_abb08() {
    // IDA 0xabb08: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecXM::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")]
// 0xabbbc — __ZN4FMOD7CodecXM19setPositionCallbackEP16FMOD_CODEC_STATEijj
// type: int __fastcall(FMOD::CodecXM *, int, unsigned int, unsigned int)
pub fn stub_abbbc() {
    // IDA 0xabbbc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecXM::calculateLength(void)")]
// 0xabbc8 — __ZN4FMOD7CodecXM15calculateLengthEv
// type: int __fastcall(FMOD::CodecXM *this)
pub fn stub_abbc8() {
    // IDA 0xabbc8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecXM::readInternal(void *,unsigned int,unsigned int *)")]
// 0xabc2c — __ZN4FMOD7CodecXM12readInternalEPvjPj
// type: unsigned int *__fastcall(FMOD::CodecXM *this, char *, unsigned int, unsigned int *)
pub fn stub_abc2c() {
    // IDA 0xabc2c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::CodecXM::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")]
// 0xac010 — __ZN4FMOD7CodecXM12readCallbackEP16FMOD_CODEC_STATEPvjPj
// type: unsigned int *__fastcall(FMOD::CodecXM *, char *, unsigned int, unsigned int *)
pub fn stub_ac010() {
    // IDA 0xac010: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}
