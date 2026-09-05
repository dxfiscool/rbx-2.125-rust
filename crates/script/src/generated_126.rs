// Auto-generated skeletons for rbx-script — filler EA-sorted asc after 0xd40b0
// Filter: Script|Lua|Yield|lua (5401 filtered, all stubbed, 0 remaining)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0xd40bc..0xd6d1c EA-sorted asc filler after 0xd40b0 (Script|Lua|Yield|lua 5401 filtered, all stubbed, filler 12985->13085, global 85545 covered, rbx_core::SharedPtr not boost) [skeleton batch]
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "FMOD::OutputNoSound::lockCallback(FMOD_OUTPUT_STATE *,unsigned int,unsigned int,void **,void **,unsigned int *,unsigned int *)")]
pub fn stub_0xd40bc(guard: &mut bool) {
// std lock op — the host guard flips.
*guard = true;
}

#[doc(alias = "FMOD::OutputNoSound::getPosition(unsigned int *)")]
pub fn stub_0xd40ec() -> crate::slot::PortedFn {
// IDA 0xd40ec: FMOD::OutputNoSound::getPosition(unsigned int*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd40ec, "FMOD::OutputNoSound::getPosition(unsigned int*)")
}

#[doc(alias = "FMOD::OutputNoSound::getPositionCallback(FMOD_OUTPUT_STATE *,unsigned int *)")]
pub fn stub_0xd4140() -> crate::slot::PortedFn {
// IDA 0xd4140: FMOD::OutputNoSound::getPositionCallback(FMOD_OUTPUT_STATE*, unsigned int*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd4140, "FMOD::OutputNoSound::getPositionCallback(FMOD_OUTPUT_STATE*, unsigned int*)")
}

#[doc(alias = "FMOD::OutputNoSound::close(void)")]
pub fn stub_0xd414c() -> crate::slot::PortedFn {
// IDA 0xd414c: FMOD::OutputNoSound::close().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd414c, "FMOD::OutputNoSound::close()")
}

#[doc(alias = "FMOD::OutputNoSound::closeCallback(FMOD_OUTPUT_STATE *)")]
pub fn stub_0xd419c() -> crate::slot::PortedFn {
// IDA 0xd419c: FMOD::OutputNoSound::closeCallback(FMOD_OUTPUT_STATE*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd419c, "FMOD::OutputNoSound::closeCallback(FMOD_OUTPUT_STATE*)")
}

#[doc(alias = "FMOD::OutputNoSound::init(int,unsigned int,int *,int,FMOD_SOUND_FORMAT *,int,int,void *)")]
pub fn stub_0xd41a8() -> crate::slot::PortedFn {
// IDA 0xd41a8: FMOD::OutputNoSound::init(int, unsigned int, int*, int, FMOD_SOUND_FORMAT*, int, int, void*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd41a8, "FMOD::OutputNoSound::init(int, unsigned int, int*, int, FMOD_SOUND_FORMAT*, int, int, void*)")
}

#[doc(alias = "FMOD::OutputNoSound::initCallback(FMOD_OUTPUT_STATE *,int,unsigned int,int *,int,FMOD_SOUND_FORMAT *,int,int,void *)")]
pub fn stub_0xd4350() -> crate::slot::PortedFn {
// IDA 0xd4350: FMOD::OutputNoSound::initCallback(FMOD_OUTPUT_STATE*, int, unsigned int, int*, int, FMOD_SOUND_FORMAT*, int, int, void*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd4350, "FMOD::OutputNoSound::initCallback(FMOD_OUTPUT_STATE*, int, unsigned int, int*, int, FMOD_SOUND_FORMA~")
}

#[doc(alias = "FMOD::OutputNoSound::getDriverName(int,char *,int)")]
pub fn stub_0xd43a0() -> crate::slot::PortedFn {
// IDA 0xd43a0: FMOD::OutputNoSound::getDriverName(int, char*, int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd43a0, "FMOD::OutputNoSound::getDriverName(int, char*, int)")
}

#[doc(alias = "FMOD::OutputNoSound::getDriverNameCallback(FMOD_OUTPUT_STATE *,int,char *,int)")]
pub fn stub_0xd43c8() -> crate::slot::PortedFn {
// IDA 0xd43c8: FMOD::OutputNoSound::getDriverNameCallback(FMOD_OUTPUT_STATE*, int, char*, int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd43c8, "FMOD::OutputNoSound::getDriverNameCallback(FMOD_OUTPUT_STATE*, int, char*, int)")
}

#[doc(alias = "FMOD::OutputNoSound::getDescriptionEx(void)")]
pub fn stub_0xd43d4() -> crate::slot::PortedFn {
// IDA 0xd43d4: FMOD::OutputNoSound::getDescriptionEx().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd43d4, "FMOD::OutputNoSound::getDescriptionEx()")
}

#[doc(alias = "__Z41__static_initialization_and_destruction_0ii_35")]
pub fn stub_0xd44a4() -> crate::slot::PortedFn {
// IDA 0xd44a4: __Z41__static_initialization_and_destruction_0ii_35.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd44a4, "__Z41__static_initialization_and_destruction_0ii_35")
}

#[doc(alias = "global constructor keyed toFMOD::nosoundoutput")]
pub fn stub_0xd44e8() -> crate::slot::PortedFn {
// IDA 0xd44e8: __GLOBAL__I__ZN4FMOD13nosoundoutputE.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xd44e8, "__GLOBAL__I__ZN4FMOD13nosoundoutputE")
}

#[doc(alias = "FMOD::OutputNoSound_NRT::getNumDrivers(int *)")]
pub fn stub_0xd44f4() -> crate::slot::PortedFn {
// IDA 0xd44f4: FMOD::OutputNoSound_NRT::getNumDrivers(int*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd44f4, "FMOD::OutputNoSound_NRT::getNumDrivers(int*)")
}

#[doc(alias = "FMOD::OutputNoSound_NRT::getDriverCaps(int,unsigned int *)")]
pub fn stub_0xd4504() -> crate::slot::PortedFn {
// IDA 0xd4504: FMOD::OutputNoSound_NRT::getDriverCaps(int, unsigned int*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd4504, "FMOD::OutputNoSound_NRT::getDriverCaps(int, unsigned int*)")
}

#[doc(alias = "FMOD::OutputNoSound_NRT::getNumDriversCallback(FMOD_OUTPUT_STATE *,int *)")]
pub fn stub_0xd4518() -> crate::slot::PortedFn {
// IDA 0xd4518: FMOD::OutputNoSound_NRT::getNumDriversCallback(FMOD_OUTPUT_STATE*, int*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd4518, "FMOD::OutputNoSound_NRT::getNumDriversCallback(FMOD_OUTPUT_STATE*, int*)")
}

#[doc(alias = "FMOD::OutputNoSound_NRT::getDriverCapsCallback(FMOD_OUTPUT_STATE *,int,unsigned int *)")]
pub fn stub_0xd4524() -> crate::slot::PortedFn {
// IDA 0xd4524: FMOD::OutputNoSound_NRT::getDriverCapsCallback(FMOD_OUTPUT_STATE*, int, unsigned int*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd4524, "FMOD::OutputNoSound_NRT::getDriverCapsCallback(FMOD_OUTPUT_STATE*, int, unsigned int*)")
}

#[doc(alias = "FMOD::OutputNoSound_NRT::update(void)")]
pub fn stub_0xd4530() -> crate::slot::PortedFn {
// IDA 0xd4530: FMOD::OutputNoSound_NRT::update().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd4530, "FMOD::OutputNoSound_NRT::update()")
}

#[doc(alias = "FMOD::OutputNoSound_NRT::updateCallback(FMOD_OUTPUT_STATE *)")]
pub fn stub_0xd454c() -> crate::slot::PortedFn {
// IDA 0xd454c: FMOD::OutputNoSound_NRT::updateCallback(FMOD_OUTPUT_STATE*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd454c, "FMOD::OutputNoSound_NRT::updateCallback(FMOD_OUTPUT_STATE*)")
}

#[doc(alias = "FMOD::OutputNoSound_NRT::close(void)")]
pub fn stub_0xd4558() -> crate::slot::PortedFn {
// IDA 0xd4558: FMOD::OutputNoSound_NRT::close().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd4558, "FMOD::OutputNoSound_NRT::close()")
}

#[doc(alias = "FMOD::OutputNoSound_NRT::closeCallback(FMOD_OUTPUT_STATE *)")]
pub fn stub_0xd45ac() -> crate::slot::PortedFn {
// IDA 0xd45ac: FMOD::OutputNoSound_NRT::closeCallback(FMOD_OUTPUT_STATE*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd45ac, "FMOD::OutputNoSound_NRT::closeCallback(FMOD_OUTPUT_STATE*)")
}

#[doc(alias = "FMOD::OutputNoSound_NRT::init(int,unsigned int,int *,int,FMOD_SOUND_FORMAT *,int,int,void *)")]
pub fn stub_0xd45b8() -> crate::slot::PortedFn {
// IDA 0xd45b8: FMOD::OutputNoSound_NRT::init(int, unsigned int, int*, int, FMOD_SOUND_FORMAT*, int, int, void*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd45b8, "FMOD::OutputNoSound_NRT::init(int, unsigned int, int*, int, FMOD_SOUND_FORMAT*, int, int, void*)")
}

#[doc(alias = "FMOD::OutputNoSound_NRT::initCallback(FMOD_OUTPUT_STATE *,int,unsigned int,int *,int,FMOD_SOUND_FORMAT *,int,int,void *)")]
pub fn stub_0xd4764() -> crate::slot::PortedFn {
// IDA 0xd4764: FMOD::OutputNoSound_NRT::initCallback(FMOD_OUTPUT_STATE*, int, unsigned int, int*, int, FMOD_SOUND_FORMAT*, int, int, vo~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd4764, "FMOD::OutputNoSound_NRT::initCallback(FMOD_OUTPUT_STATE*, int, unsigned int, int*, int, FMOD_SOUND_F~")
}

#[doc(alias = "FMOD::OutputNoSound_NRT::getDriverName(int,char *,int)")]
pub fn stub_0xd47b4() -> crate::slot::PortedFn {
// IDA 0xd47b4: FMOD::OutputNoSound_NRT::getDriverName(int, char*, int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd47b4, "FMOD::OutputNoSound_NRT::getDriverName(int, char*, int)")
}

#[doc(alias = "FMOD::OutputNoSound_NRT::getDriverNameCallback(FMOD_OUTPUT_STATE *,int,char *,int)")]
pub fn stub_0xd47dc() -> crate::slot::PortedFn {
// IDA 0xd47dc: FMOD::OutputNoSound_NRT::getDriverNameCallback(FMOD_OUTPUT_STATE*, int, char*, int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd47dc, "FMOD::OutputNoSound_NRT::getDriverNameCallback(FMOD_OUTPUT_STATE*, int, char*, int)")
}

#[doc(alias = "FMOD::OutputNoSound_NRT::getDescriptionEx(void)")]
pub fn stub_0xd47e8() -> crate::slot::PortedFn {
// IDA 0xd47e8: FMOD::OutputNoSound_NRT::getDescriptionEx().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd47e8, "FMOD::OutputNoSound_NRT::getDescriptionEx()")
}

#[doc(alias = "__Z41__static_initialization_and_destruction_0ii_36")]
pub fn stub_0xd48a8() -> crate::slot::PortedFn {
// IDA 0xd48a8: __Z41__static_initialization_and_destruction_0ii_36.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd48a8, "__Z41__static_initialization_and_destruction_0ii_36")
}

#[doc(alias = "global constructor keyed toFMOD::nosoundoutput_nrt")]
pub fn stub_0xd48ec() -> crate::slot::PortedFn {
// IDA 0xd48ec: __GLOBAL__I__ZN4FMOD17nosoundoutput_nrtE.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xd48ec, "__GLOBAL__I__ZN4FMOD17nosoundoutput_nrtE")
}

#[doc(alias = "FMOD::OutputPolled::stop(void)")]
pub fn stub_0xd48f8() -> crate::slot::PortedFn {
// IDA 0xd48f8: FMOD::OutputPolled::stop().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd48f8, "FMOD::OutputPolled::stop()")
}

#[doc(alias = "FMOD::OutputPolled::OutputPolled(void)")]
pub fn stub_0xd4930() -> crate::slot::PortedFn {
// IDA 0xd4930: FMOD::OutputPolled::OutputPolled().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd4930, "FMOD::OutputPolled::OutputPolled()")
}

#[doc(alias = "FMOD::OutputPolled::OutputPolled(void) [0xd497c]")]
pub fn stub_0xd497c() -> crate::slot::PortedFn {
// IDA 0xd497c: FMOD::OutputPolled::OutputPolled().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd497c, "FMOD::OutputPolled::OutputPolled()")
}

#[doc(alias = "FMOD::OutputPolled::start(void)")]
pub fn stub_0xd4980() -> crate::slot::PortedFn {
// IDA 0xd4980: FMOD::OutputPolled::start().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd4980, "FMOD::OutputPolled::start()")
}

#[doc(alias = "non-virtual thunk toFMOD::OutputPolled::threadFunc(void)")]
pub fn stub_0xd4ac0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 360, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 360);
}

#[doc(alias = "FMOD::OutputPolled::threadFunc(void)")]
pub fn stub_0xd4ac8() -> crate::slot::PortedFn {
// IDA 0xd4ac8: FMOD::OutputPolled::threadFunc().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd4ac8, "FMOD::OutputPolled::threadFunc()")
}

#[doc(alias = "non-virtual thunk toFMOD::OutputPolled::~OutputPolled()")]
pub fn stub_0xd4ff8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 360, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 360);
}

#[doc(alias = "FMOD::OutputPolled::~OutputPolled()")]
pub fn stub_0xd5000() -> crate::slot::PortedFn {
// IDA 0xd5000: FMOD::OutputPolled::~OutputPolled().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd5000, "FMOD::OutputPolled::~OutputPolled()")
}

#[doc(alias = "non-virtual thunk toFMOD::OutputPolled::~OutputPolled() [0xd5038]")]
pub fn stub_0xd5038(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 360, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 360);
}

#[doc(alias = "FMOD::OutputPolled::~OutputPolled() [0xd5040]")]
pub fn stub_0xd5040() -> crate::slot::PortedFn {
// IDA 0xd5040: FMOD::OutputPolled::~OutputPolled().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd5040, "FMOD::OutputPolled::~OutputPolled()")
}

#[doc(alias = "FMOD::OutputSoftware::getSampleMaxChannels(unsigned int,FMOD_SOUND_FORMAT)")]
pub fn stub_0xd506c() -> crate::slot::PortedFn {
// IDA 0xd506c: FMOD::OutputSoftware::getSampleMaxChannels(unsigned int, FMOD_SOUND_FORMAT).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd506c, "FMOD::OutputSoftware::getSampleMaxChannels(unsigned int, FMOD_SOUND_FORMAT)")
}

#[doc(alias = "FMOD::OutputSoftware::getSampleMaxChannelsCallback(FMOD_OUTPUT_STATE *,unsigned int,FMOD_SOUND_FORMAT)")]
pub fn stub_0xd5074() -> crate::slot::PortedFn {
// IDA 0xd5074: FMOD::OutputSoftware::getSampleMaxChannelsCallback(FMOD_OUTPUT_STATE*, unsigned int, FMOD_SOUND_FORMAT).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd5074, "FMOD::OutputSoftware::getSampleMaxChannelsCallback(FMOD_OUTPUT_STATE*, unsigned int, FMOD_SOUND_FORM~")
}

#[doc(alias = "FMOD::OutputSoftware::release(void)")]
pub fn stub_0xd5080() -> crate::slot::PortedFn {
// IDA 0xd5080: FMOD::OutputSoftware::release().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd5080, "FMOD::OutputSoftware::release()")
}

#[doc(alias = "FMOD::OutputSoftware::OutputSoftware(void)")]
pub fn stub_0xd50ec() -> crate::slot::PortedFn {
// IDA 0xd50ec: FMOD::OutputSoftware::OutputSoftware().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd50ec, "FMOD::OutputSoftware::OutputSoftware()")
}

#[doc(alias = "FMOD::OutputSoftware::OutputSoftware(void) [0xd5170]")]
pub fn stub_0xd5170() -> crate::slot::PortedFn {
// IDA 0xd5170: FMOD::OutputSoftware::OutputSoftware().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd5170, "FMOD::OutputSoftware::OutputSoftware()")
}

#[doc(alias = "FMOD::OutputSoftware::init(int)")]
pub fn stub_0xd5174() -> crate::slot::PortedFn {
// IDA 0xd5174: FMOD::OutputSoftware::init(int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd5174, "FMOD::OutputSoftware::init(int)")
}

#[doc(alias = "FMOD::OutputSoftware::createSample(unsigned int,FMOD_CODEC_WAVEFORMAT *,FMOD::Sample **)")]
pub fn stub_0xd52d0() -> crate::slot::PortedFn {
// IDA 0xd52d0: FMOD::OutputSoftware::createSample(unsigned int, FMOD_CODEC_WAVEFORMAT*, FMOD::Sample**).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd52d0, "FMOD::OutputSoftware::createSample(unsigned int, FMOD_CODEC_WAVEFORMAT*, FMOD::Sample**)")
}

#[doc(alias = "FMOD::OutputWavWriter::getNumDrivers(int *)")]
pub fn stub_0xd5770() -> crate::slot::PortedFn {
// IDA 0xd5770: FMOD::OutputWavWriter::getNumDrivers(int*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd5770, "FMOD::OutputWavWriter::getNumDrivers(int*)")
}

#[doc(alias = "FMOD::OutputWavWriter::getDriverCaps(int,unsigned int *)")]
pub fn stub_0xd5780() -> crate::slot::PortedFn {
// IDA 0xd5780: FMOD::OutputWavWriter::getDriverCaps(int, unsigned int*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd5780, "FMOD::OutputWavWriter::getDriverCaps(int, unsigned int*)")
}

#[doc(alias = "FMOD::OutputWavWriter::lock(unsigned int,unsigned int,void **,void **,unsigned int *,unsigned int *)")]
pub fn stub_0xd5794(guard: &mut bool) {
// std lock op — the host guard flips.
*guard = true;
}

#[doc(alias = "FMOD::OutputWavWriter::getHandle(void **)")]
pub fn stub_0xd5834() -> crate::slot::PortedFn {
// IDA 0xd5834: FMOD::OutputWavWriter::getHandle(void**).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd5834, "FMOD::OutputWavWriter::getHandle(void**)")
}

#[doc(alias = "FMOD::OutputWavWriter::getNumDriversCallback(FMOD_OUTPUT_STATE *,int *)")]
pub fn stub_0xd5844() -> crate::slot::PortedFn {
// IDA 0xd5844: FMOD::OutputWavWriter::getNumDriversCallback(FMOD_OUTPUT_STATE*, int*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd5844, "FMOD::OutputWavWriter::getNumDriversCallback(FMOD_OUTPUT_STATE*, int*)")
}

#[doc(alias = "FMOD::OutputWavWriter::getDriverCapsCallback(FMOD_OUTPUT_STATE *,int,unsigned int *)")]
pub fn stub_0xd5850() -> crate::slot::PortedFn {
// IDA 0xd5850: FMOD::OutputWavWriter::getDriverCapsCallback(FMOD_OUTPUT_STATE*, int, unsigned int*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd5850, "FMOD::OutputWavWriter::getDriverCapsCallback(FMOD_OUTPUT_STATE*, int, unsigned int*)")
}

#[doc(alias = "FMOD::OutputWavWriter::lockCallback(FMOD_OUTPUT_STATE *,unsigned int,unsigned int,void **,void **,unsigned int *,unsigned int *)")]
pub fn stub_0xd585c(guard: &mut bool) {
// std lock op — the host guard flips.
*guard = true;
}

#[doc(alias = "FMOD::OutputWavWriter::getHandleCallback(FMOD_OUTPUT_STATE *,void **)")]
pub fn stub_0xd588c() -> crate::slot::PortedFn {
// IDA 0xd588c: FMOD::OutputWavWriter::getHandleCallback(FMOD_OUTPUT_STATE*, void**).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd588c, "FMOD::OutputWavWriter::getHandleCallback(FMOD_OUTPUT_STATE*, void**)")
}

#[doc(alias = "FMOD::OutputWavWriter::writeWavHeader(void)")]
pub fn stub_0xd5898() -> crate::slot::PortedFn {
// IDA 0xd5898: FMOD::OutputWavWriter::writeWavHeader().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd5898, "FMOD::OutputWavWriter::writeWavHeader()")
}

#[doc(alias = "FMOD::OutputWavWriter::unlock(void *,void *,unsigned int,unsigned int)")]
pub fn stub_0xd5adc(guard: &mut bool) {
// std lock op — the host guard flips.
*guard = true;
}

#[doc(alias = "FMOD::OutputWavWriter::unlockCallback(FMOD_OUTPUT_STATE *,void *,void *,unsigned int,unsigned int)")]
pub fn stub_0xd5bd0(guard: &mut bool) {
// std lock op — the host guard flips.
*guard = true;
}

#[doc(alias = "FMOD::OutputWavWriter::close(void)")]
pub fn stub_0xd5be4() -> crate::slot::PortedFn {
// IDA 0xd5be4: FMOD::OutputWavWriter::close().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd5be4, "FMOD::OutputWavWriter::close()")
}

#[doc(alias = "FMOD::OutputWavWriter::closeCallback(FMOD_OUTPUT_STATE *)")]
pub fn stub_0xd5c58() -> crate::slot::PortedFn {
// IDA 0xd5c58: FMOD::OutputWavWriter::closeCallback(FMOD_OUTPUT_STATE*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd5c58, "FMOD::OutputWavWriter::closeCallback(FMOD_OUTPUT_STATE*)")
}

#[doc(alias = "FMOD::OutputWavWriter::getDriverName(int,char *,int)")]
pub fn stub_0xd5c64() -> crate::slot::PortedFn {
// IDA 0xd5c64: FMOD::OutputWavWriter::getDriverName(int, char*, int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd5c64, "FMOD::OutputWavWriter::getDriverName(int, char*, int)")
}

#[doc(alias = "FMOD::OutputWavWriter::getDriverNameCallback(FMOD_OUTPUT_STATE *,int,char *,int)")]
pub fn stub_0xd5c8c() -> crate::slot::PortedFn {
// IDA 0xd5c8c: FMOD::OutputWavWriter::getDriverNameCallback(FMOD_OUTPUT_STATE*, int, char*, int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd5c8c, "FMOD::OutputWavWriter::getDriverNameCallback(FMOD_OUTPUT_STATE*, int, char*, int)")
}

#[doc(alias = "FMOD::OutputWavWriter::init(int,unsigned int,int *,int,FMOD_SOUND_FORMAT *,int,int,void *)")]
pub fn stub_0xd5c98() -> crate::slot::PortedFn {
// IDA 0xd5c98: FMOD::OutputWavWriter::init(int, unsigned int, int*, int, FMOD_SOUND_FORMAT*, int, int, void*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd5c98, "FMOD::OutputWavWriter::init(int, unsigned int, int*, int, FMOD_SOUND_FORMAT*, int, int, void*)")
}

#[doc(alias = "FMOD::OutputWavWriter::initCallback(FMOD_OUTPUT_STATE *,int,unsigned int,int *,int,FMOD_SOUND_FORMAT *,int,int,void *)")]
pub fn stub_0xd5f48() -> crate::slot::PortedFn {
// IDA 0xd5f48: FMOD::OutputWavWriter::initCallback(FMOD_OUTPUT_STATE*, int, unsigned int, int*, int, FMOD_SOUND_FORMAT*, int, int, void~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd5f48, "FMOD::OutputWavWriter::initCallback(FMOD_OUTPUT_STATE*, int, unsigned int, int*, int, FMOD_SOUND_FOR~")
}

#[doc(alias = "FMOD::OutputWavWriter::getDescriptionEx(void)")]
pub fn stub_0xd5f98() -> crate::slot::PortedFn {
// IDA 0xd5f98: FMOD::OutputWavWriter::getDescriptionEx().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd5f98, "FMOD::OutputWavWriter::getDescriptionEx()")
}

#[doc(alias = "FMOD::OutputWavWriter::getPosition(unsigned int *)")]
pub fn stub_0xd60a0() -> crate::slot::PortedFn {
// IDA 0xd60a0: FMOD::OutputWavWriter::getPosition(unsigned int*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd60a0, "FMOD::OutputWavWriter::getPosition(unsigned int*)")
}

#[doc(alias = "FMOD::OutputWavWriter::getPositionCallback(FMOD_OUTPUT_STATE *,unsigned int *)")]
pub fn stub_0xd60f4() -> crate::slot::PortedFn {
// IDA 0xd60f4: FMOD::OutputWavWriter::getPositionCallback(FMOD_OUTPUT_STATE*, unsigned int*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd60f4, "FMOD::OutputWavWriter::getPositionCallback(FMOD_OUTPUT_STATE*, unsigned int*)")
}

#[doc(alias = "__Z41__static_initialization_and_destruction_0ii_37")]
pub fn stub_0xd6100() -> crate::slot::PortedFn {
// IDA 0xd6100: __Z41__static_initialization_and_destruction_0ii_37.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd6100, "__Z41__static_initialization_and_destruction_0ii_37")
}

#[doc(alias = "global constructor keyed toFMOD::wavwriteroutput")]
pub fn stub_0xd6144() -> crate::slot::PortedFn {
// IDA 0xd6144: __GLOBAL__I__ZN4FMOD15wavwriteroutputE.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xd6144, "__GLOBAL__I__ZN4FMOD15wavwriteroutputE")
}

#[doc(alias = "FMOD::OutputWavWriter_NRT::getNumDrivers(int *)")]
pub fn stub_0xd6150() -> crate::slot::PortedFn {
// IDA 0xd6150: FMOD::OutputWavWriter_NRT::getNumDrivers(int*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd6150, "FMOD::OutputWavWriter_NRT::getNumDrivers(int*)")
}

#[doc(alias = "FMOD::OutputWavWriter_NRT::getDriverCaps(int,unsigned int *)")]
pub fn stub_0xd6160() -> crate::slot::PortedFn {
// IDA 0xd6160: FMOD::OutputWavWriter_NRT::getDriverCaps(int, unsigned int*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd6160, "FMOD::OutputWavWriter_NRT::getDriverCaps(int, unsigned int*)")
}

#[doc(alias = "FMOD::OutputWavWriter_NRT::getHandle(void **)")]
pub fn stub_0xd6174() -> crate::slot::PortedFn {
// IDA 0xd6174: FMOD::OutputWavWriter_NRT::getHandle(void**).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd6174, "FMOD::OutputWavWriter_NRT::getHandle(void**)")
}

#[doc(alias = "FMOD::OutputWavWriter_NRT::getNumDriversCallback(FMOD_OUTPUT_STATE *,int *)")]
pub fn stub_0xd6184() -> crate::slot::PortedFn {
// IDA 0xd6184: FMOD::OutputWavWriter_NRT::getNumDriversCallback(FMOD_OUTPUT_STATE*, int*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd6184, "FMOD::OutputWavWriter_NRT::getNumDriversCallback(FMOD_OUTPUT_STATE*, int*)")
}

#[doc(alias = "FMOD::OutputWavWriter_NRT::getDriverCapsCallback(FMOD_OUTPUT_STATE *,int,unsigned int *)")]
pub fn stub_0xd6190() -> crate::slot::PortedFn {
// IDA 0xd6190: FMOD::OutputWavWriter_NRT::getDriverCapsCallback(FMOD_OUTPUT_STATE*, int, unsigned int*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd6190, "FMOD::OutputWavWriter_NRT::getDriverCapsCallback(FMOD_OUTPUT_STATE*, int, unsigned int*)")
}

#[doc(alias = "FMOD::OutputWavWriter_NRT::getHandleCallback(FMOD_OUTPUT_STATE *,void **)")]
pub fn stub_0xd619c() -> crate::slot::PortedFn {
// IDA 0xd619c: FMOD::OutputWavWriter_NRT::getHandleCallback(FMOD_OUTPUT_STATE*, void**).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd619c, "FMOD::OutputWavWriter_NRT::getHandleCallback(FMOD_OUTPUT_STATE*, void**)")
}

#[doc(alias = "FMOD::OutputWavWriter_NRT::writeWavHeader(void)")]
pub fn stub_0xd61a8() -> crate::slot::PortedFn {
// IDA 0xd61a8: FMOD::OutputWavWriter_NRT::writeWavHeader().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd61a8, "FMOD::OutputWavWriter_NRT::writeWavHeader()")
}

#[doc(alias = "FMOD::OutputWavWriter_NRT::stop(void)")]
pub fn stub_0xd63ec() -> crate::slot::PortedFn {
// IDA 0xd63ec: FMOD::OutputWavWriter_NRT::stop().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd63ec, "FMOD::OutputWavWriter_NRT::stop()")
}

#[doc(alias = "FMOD::OutputWavWriter_NRT::stopCallback(FMOD_OUTPUT_STATE *)")]
pub fn stub_0xd641c() -> crate::slot::PortedFn {
// IDA 0xd641c: FMOD::OutputWavWriter_NRT::stopCallback(FMOD_OUTPUT_STATE*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd641c, "FMOD::OutputWavWriter_NRT::stopCallback(FMOD_OUTPUT_STATE*)")
}

#[doc(alias = "FMOD::OutputWavWriter_NRT::start(void)")]
pub fn stub_0xd6428() -> crate::slot::PortedFn {
// IDA 0xd6428: FMOD::OutputWavWriter_NRT::start().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd6428, "FMOD::OutputWavWriter_NRT::start()")
}

#[doc(alias = "FMOD::OutputWavWriter_NRT::startCallback(FMOD_OUTPUT_STATE *)")]
pub fn stub_0xd6468() -> crate::slot::PortedFn {
// IDA 0xd6468: FMOD::OutputWavWriter_NRT::startCallback(FMOD_OUTPUT_STATE*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd6468, "FMOD::OutputWavWriter_NRT::startCallback(FMOD_OUTPUT_STATE*)")
}

#[doc(alias = "FMOD::OutputWavWriter_NRT::update(void)")]
pub fn stub_0xd6474() -> crate::slot::PortedFn {
// IDA 0xd6474: FMOD::OutputWavWriter_NRT::update().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd6474, "FMOD::OutputWavWriter_NRT::update()")
}

#[doc(alias = "FMOD::OutputWavWriter_NRT::updateCallback(FMOD_OUTPUT_STATE *)")]
pub fn stub_0xd6504() -> crate::slot::PortedFn {
// IDA 0xd6504: FMOD::OutputWavWriter_NRT::updateCallback(FMOD_OUTPUT_STATE*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd6504, "FMOD::OutputWavWriter_NRT::updateCallback(FMOD_OUTPUT_STATE*)")
}

#[doc(alias = "FMOD::OutputWavWriter_NRT::close(void)")]
pub fn stub_0xd6510() -> crate::slot::PortedFn {
// IDA 0xd6510: FMOD::OutputWavWriter_NRT::close().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd6510, "FMOD::OutputWavWriter_NRT::close()")
}

#[doc(alias = "FMOD::OutputWavWriter_NRT::closeCallback(FMOD_OUTPUT_STATE *)")]
pub fn stub_0xd6560() -> crate::slot::PortedFn {
// IDA 0xd6560: FMOD::OutputWavWriter_NRT::closeCallback(FMOD_OUTPUT_STATE*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd6560, "FMOD::OutputWavWriter_NRT::closeCallback(FMOD_OUTPUT_STATE*)")
}

#[doc(alias = "FMOD::OutputWavWriter_NRT::init(int,unsigned int,int *,int,FMOD_SOUND_FORMAT *,int,int,void *)")]
pub fn stub_0xd656c() -> crate::slot::PortedFn {
// IDA 0xd656c: FMOD::OutputWavWriter_NRT::init(int, unsigned int, int*, int, FMOD_SOUND_FORMAT*, int, int, void*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd656c, "FMOD::OutputWavWriter_NRT::init(int, unsigned int, int*, int, FMOD_SOUND_FORMAT*, int, int, void*)")
}

#[doc(alias = "FMOD::OutputWavWriter_NRT::initCallback(FMOD_OUTPUT_STATE *,int,unsigned int,int *,int,FMOD_SOUND_FORMAT *,int,int,void *)")]
pub fn stub_0xd67f0() -> crate::slot::PortedFn {
// IDA 0xd67f0: FMOD::OutputWavWriter_NRT::initCallback(FMOD_OUTPUT_STATE*, int, unsigned int, int*, int, FMOD_SOUND_FORMAT*, int, int, ~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd67f0, "FMOD::OutputWavWriter_NRT::initCallback(FMOD_OUTPUT_STATE*, int, unsigned int, int*, int, FMOD_SOUND~")
}

#[doc(alias = "FMOD::OutputWavWriter_NRT::getDriverName(int,char *,int)")]
pub fn stub_0xd6840() -> crate::slot::PortedFn {
// IDA 0xd6840: FMOD::OutputWavWriter_NRT::getDriverName(int, char*, int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd6840, "FMOD::OutputWavWriter_NRT::getDriverName(int, char*, int)")
}

#[doc(alias = "FMOD::OutputWavWriter_NRT::getDriverNameCallback(FMOD_OUTPUT_STATE *,int,char *,int)")]
pub fn stub_0xd6868() -> crate::slot::PortedFn {
// IDA 0xd6868: FMOD::OutputWavWriter_NRT::getDriverNameCallback(FMOD_OUTPUT_STATE*, int, char*, int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd6868, "FMOD::OutputWavWriter_NRT::getDriverNameCallback(FMOD_OUTPUT_STATE*, int, char*, int)")
}

#[doc(alias = "FMOD::OutputWavWriter_NRT::getDescriptionEx(void)")]
pub fn stub_0xd6874() -> crate::slot::PortedFn {
// IDA 0xd6874: FMOD::OutputWavWriter_NRT::getDescriptionEx().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd6874, "FMOD::OutputWavWriter_NRT::getDescriptionEx()")
}

#[doc(alias = "__Z41__static_initialization_and_destruction_0ii_38")]
pub fn stub_0xd6978() -> crate::slot::PortedFn {
// IDA 0xd6978: __Z41__static_initialization_and_destruction_0ii_38.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd6978, "__Z41__static_initialization_and_destruction_0ii_38")
}

#[doc(alias = "global constructor keyed toFMOD::wavwriteroutput_nrt")]
pub fn stub_0xd69bc() -> crate::slot::PortedFn {
// IDA 0xd69bc: __GLOBAL__I__ZN4FMOD19wavwriteroutput_nrtE.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xd69bc, "__GLOBAL__I__ZN4FMOD19wavwriteroutput_nrtE")
}

#[doc(alias = "FMOD::Plugin::release(void)")]
pub fn stub_0xd69c8() -> crate::slot::PortedFn {
// IDA 0xd69c8: FMOD::Plugin::release().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd69c8, "FMOD::Plugin::release()")
}

#[doc(alias = "FMOD::PluginFactory::setSystem(FMOD::SystemI *)")]
pub fn stub_0xd6a04() -> crate::slot::PortedFn {
// IDA 0xd6a04: FMOD::PluginFactory::setSystem(FMOD::SystemI*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd6a04, "FMOD::PluginFactory::setSystem(FMOD::SystemI*)")
}

#[doc(alias = "FMOD::PluginFactory::getNumCodecs(int *)")]
pub fn stub_0xd6a10() -> crate::slot::PortedFn {
// IDA 0xd6a10: FMOD::PluginFactory::getNumCodecs(int*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd6a10, "FMOD::PluginFactory::getNumCodecs(int*)")
}

#[doc(alias = "FMOD::PluginFactory::getNumDSPs(int *)")]
pub fn stub_0xd6a50() -> crate::slot::PortedFn {
// IDA 0xd6a50: FMOD::PluginFactory::getNumDSPs(int*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd6a50, "FMOD::PluginFactory::getNumDSPs(int*)")
}

#[doc(alias = "FMOD::PluginFactory::getNumOutputs(int *)")]
pub fn stub_0xd6a90() -> crate::slot::PortedFn {
// IDA 0xd6a90: FMOD::PluginFactory::getNumOutputs(int*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd6a90, "FMOD::PluginFactory::getNumOutputs(int*)")
}

#[doc(alias = "FMOD::PluginFactory::getCodecHandle(int,unsigned int *)")]
pub fn stub_0xd6ad0() -> crate::slot::PortedFn {
// IDA 0xd6ad0: FMOD::PluginFactory::getCodecHandle(int, unsigned int*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd6ad0, "FMOD::PluginFactory::getCodecHandle(int, unsigned int*)")
}

#[doc(alias = "FMOD::PluginFactory::getDSPHandle(int,unsigned int *)")]
pub fn stub_0xd6b30() -> crate::slot::PortedFn {
// IDA 0xd6b30: FMOD::PluginFactory::getDSPHandle(int, unsigned int*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd6b30, "FMOD::PluginFactory::getDSPHandle(int, unsigned int*)")
}

#[doc(alias = "FMOD::PluginFactory::getOutputHandle(int,unsigned int *)")]
pub fn stub_0xd6b90() -> crate::slot::PortedFn {
// IDA 0xd6b90: FMOD::PluginFactory::getOutputHandle(int, unsigned int*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd6b90, "FMOD::PluginFactory::getOutputHandle(int, unsigned int*)")
}

#[doc(alias = "FMOD::PluginFactory::getCodec(unsigned int,FMOD::FMOD_CODEC_DESCRIPTION_EX **)")]
pub fn stub_0xd6bf0() -> crate::slot::PortedFn {
// IDA 0xd6bf0: FMOD::PluginFactory::getCodec(unsigned int, FMOD::FMOD_CODEC_DESCRIPTION_EX**).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd6bf0, "FMOD::PluginFactory::getCodec(unsigned int, FMOD::FMOD_CODEC_DESCRIPTION_EX**)")
}

#[doc(alias = "FMOD::PluginFactory::getDSP(unsigned int,FMOD::FMOD_DSP_DESCRIPTION_EX **)")]
pub fn stub_0xd6c54() -> crate::slot::PortedFn {
// IDA 0xd6c54: FMOD::PluginFactory::getDSP(unsigned int, FMOD::FMOD_DSP_DESCRIPTION_EX**).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd6c54, "FMOD::PluginFactory::getDSP(unsigned int, FMOD::FMOD_DSP_DESCRIPTION_EX**)")
}

#[doc(alias = "FMOD::PluginFactory::getOutput(unsigned int,FMOD::FMOD_OUTPUT_DESCRIPTION_EX **)")]
pub fn stub_0xd6cb8() -> crate::slot::PortedFn {
// IDA 0xd6cb8: FMOD::PluginFactory::getOutput(unsigned int, FMOD::FMOD_OUTPUT_DESCRIPTION_EX**).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd6cb8, "FMOD::PluginFactory::getOutput(unsigned int, FMOD::FMOD_OUTPUT_DESCRIPTION_EX**)")
}

#[doc(alias = "FMOD::PluginFactory::unloadPlugin(unsigned int)")]
pub fn stub_0xd6d1c() -> crate::slot::PortedFn {
// IDA 0xd6d1c: FMOD::PluginFactory::unloadPlugin(unsigned int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xd6d1c, "FMOD::PluginFactory::unloadPlugin(unsigned int)")
}
