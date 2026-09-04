//! rendering — Ogre::|G3D:: strict 13333 total
//! This shard: 0xc8d1f4..0xc9610c (100 stubs, 8370 prior -> +100, 4863 remaining)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;


// 0xc8d1f4 — __ZN4Ogre17FileSystemArchiveD1Ev
#[doc(alias = "Ogre::FileSystemArchive::~FileSystemArchive()")]
// was: Ogre::FileSystemArchive::~FileSystemArchive()
// IDA 0xc8d1f4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c8d1f4() {
}

// 0xc8d28c — __ZN4Ogre17FileSystemArchive4loadEv
#[doc(alias = "Ogre::FileSystemArchive::load(void)")]
// was: Ogre::FileSystemArchive::load(void)
// IDA 0xc8d28c: 271 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8d28c() {
}

// 0xc8d5a4 — __ZN4Ogre17FileSystemArchive6unloadEv
#[doc(alias = "Ogre::FileSystemArchive::unload(void)")]
// was: Ogre::FileSystemArchive::unload(void)
// IDA 0xc8d5a4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c8d5a4() {
}

// 0xc8d5a8 — __ZNK4Ogre17FileSystemArchive4openERKSsb
#[doc(alias = "Ogre::FileSystemArchive::open(std::string const&,bool)const")]
// was: Ogre::FileSystemArchive::open(std::string const&,bool)const
// IDA 0xc8d5a8: 461 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8d5a8() {
}

// 0xc8da8c — __ZNK4Ogre17FileSystemArchive6createERKSs
#[doc(alias = "Ogre::FileSystemArchive::create(std::string const&)const")]
// was: Ogre::FileSystemArchive::create(std::string const&)const
// IDA 0xc8da8c: 480 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8da8c() {
}

// 0xc8dff8 — __ZNK4Ogre17FileSystemArchive6removeERKSs
#[doc(alias = "Ogre::FileSystemArchive::remove(std::string const&)const")]
// was: Ogre::FileSystemArchive::remove(std::string const&)const
// IDA 0xc8dff8: 189 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8dff8() {
}

// 0xc8e224 — __ZN4Ogre17FileSystemArchive4listEbb
#[doc(alias = "Ogre::FileSystemArchive::list(bool,bool)")]
// was: Ogre::FileSystemArchive::list(bool,bool)
// IDA 0xc8e224: 196 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8e224() {
}

// 0xc8e440 — __ZN4Ogre17FileSystemArchive12listFileInfoEbb
#[doc(alias = "Ogre::FileSystemArchive::listFileInfo(bool,bool)")]
// was: Ogre::FileSystemArchive::listFileInfo(bool,bool)
// IDA 0xc8e440: 197 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8e440() {
}

// 0xc8e65c — __ZN4Ogre17FileSystemArchive4findERKSsbb
#[doc(alias = "Ogre::FileSystemArchive::find(std::string const&,bool,bool)")]
// was: Ogre::FileSystemArchive::find(std::string const&,bool,bool)
// IDA 0xc8e65c: 147 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8e65c() {
}

// 0xc8e7d8 — __ZNK4Ogre17FileSystemArchive12findFileInfoERKSsbb
#[doc(alias = "Ogre::FileSystemArchive::findFileInfo(std::string const&,bool,bool)const")]
// was: Ogre::FileSystemArchive::findFileInfo(std::string const&,bool,bool)const
// IDA 0xc8e7d8: 146 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8e7d8() {
}

// 0xc8e954 — __ZN4Ogre17FileSystemArchive6existsERKSs
#[doc(alias = "Ogre::FileSystemArchive::exists(std::string const&)")]
// was: Ogre::FileSystemArchive::exists(std::string const&)
// IDA 0xc8e954: 115 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8e954() {
}

// 0xc8ea98 — __ZN4Ogre17FileSystemArchive15getModifiedTimeERKSs
#[doc(alias = "Ogre::FileSystemArchive::getModifiedTime(std::string const&)")]
// was: Ogre::FileSystemArchive::getModifiedTime(std::string const&)
// IDA 0xc8ea98: 100 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8ea98() {
}

// 0xc8ebbc — __ZNK4Ogre24FileSystemArchiveFactory7getTypeEv
#[doc(alias = "Ogre::FileSystemArchiveFactory::getType(void)const")]
// was: Ogre::FileSystemArchiveFactory::getType(void)const
// IDA 0xc8ebbc: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8ebbc() {
}

// 0xc8ece4 — __ZN4Ogre4FontC1EPNS_15ResourceManagerERKSsyS4_bPNS_20ManualResourceLoaderE
#[doc(alias = "Ogre::Font::Font(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *)")]
// was: Ogre::Font::Font(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *)
// IDA 0xc8ece4: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8ece4() {
}

// 0xc8ed00 — __ZN4Ogre4FontC2EPNS_15ResourceManagerERKSsyS4_bPNS_20ManualResourceLoaderE
#[doc(alias = "Ogre::Font::Font(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *)")]
// was: Ogre::Font::Font(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *)
// IDA 0xc8ed00: 1729 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8ed00() {
}

// 0xc8ffe4 — __ZN4Ogre4FontD0Ev
#[doc(alias = "Ogre::Font::~Font()")]
// was: Ogre::Font::~Font()
// IDA 0xc8ffe4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c8ffe4() {
}

// 0xc90074 — __ZN4Ogre4FontD1Ev
#[doc(alias = "Ogre::Font::~Font()")]
// was: Ogre::Font::~Font()
// IDA 0xc90074: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c90074() {
}

// 0xc90080 — __ZThn88_N4Ogre4FontD0Ev
#[doc(alias = "non-virtual thunk toOgre::Font::~Font()")]
// was: non-virtual thunk to Ogre::Font::~Font()
// IDA 0xc90080: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c90080() {
}

// 0xc90114 — __ZN4Ogre4FontD2Ev
#[doc(alias = "Ogre::Font::~Font()")]
// was: Ogre::Font::~Font()
// IDA 0xc90114: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c90114() {
}

// 0xc90534 — __ZThn88_N4Ogre4FontD1Ev
#[doc(alias = "non-virtual thunk toOgre::Font::~Font()")]
// was: non-virtual thunk to Ogre::Font::~Font()
// IDA 0xc90534: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c90534() {
}

// 0xc90540 — __ZN4Ogre4Font7setTypeENS_8FontTypeE
#[doc(alias = "Ogre::Font::setType(Ogre::FontType)")]
// was: Ogre::Font::setType(Ogre::FontType)
// IDA 0xc90540: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c90540() {
}

// 0xc90544 — __ZN4Ogre4Font9setSourceERKSs
#[doc(alias = "Ogre::Font::setSource(std::string const&)")]
// was: Ogre::Font::setSource(std::string const&)
// IDA 0xc90544: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c90544() {
}

// 0xc90550 — __ZN4Ogre4Font15setTrueTypeSizeEf
#[doc(alias = "Ogre::Font::setTrueTypeSize(float)")]
// was: Ogre::Font::setTrueTypeSize(float)
// IDA 0xc90550: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c90550() {
}

// 0xc90554 — __ZN4Ogre4Font18setCharacterSpacerEj
#[doc(alias = "Ogre::Font::setCharacterSpacer(unsigned int)")]
// was: Ogre::Font::setCharacterSpacer(unsigned int)
// IDA 0xc90554: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c90554() {
}

// 0xc90558 — __ZN4Ogre4Font21setTrueTypeResolutionEj
#[doc(alias = "Ogre::Font::setTrueTypeResolution(unsigned int)")]
// was: Ogre::Font::setTrueTypeResolution(unsigned int)
// IDA 0xc90558: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c90558() {
}

// 0xc9055c — __ZNK4Ogre4Font12getGlyphInfoEj
#[doc(alias = "Ogre::Font::getGlyphInfo(unsigned int)const")]
// was: Ogre::Font::getGlyphInfo(unsigned int)const
// IDA 0xc9055c: 383 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9055c() {
}

// 0xc909b4 — __ZN4Ogre4Font8loadImplEv
#[doc(alias = "Ogre::Font::loadImpl(void)")]
// was: Ogre::Font::loadImpl(void)
// IDA 0xc909b4: 375 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c909b4() {
}

// 0xc90fc8 — __ZN4Ogre4Font21createTextureFromFontEv
#[doc(alias = "Ogre::Font::createTextureFromFont(void)")]
// was: Ogre::Font::createTextureFromFont(void)
// IDA 0xc90fc8: 311 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c90fc8() {
}

// 0xc91308 — __ZN4Ogre4Font10unloadImplEv
#[doc(alias = "Ogre::Font::unloadImpl(void)")]
// was: Ogre::Font::unloadImpl(void)
// IDA 0xc91308: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c91308() {
}

// 0xc913ac — __ZN4Ogre4Font12loadResourceEPNS_8ResourceE
#[doc(alias = "Ogre::Font::loadResource(Ogre::Resource *)")]
// was: Ogre::Font::loadResource(Ogre::Resource *)
// IDA 0xc913ac: 2197 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c913ac() {
}

// 0xc92bf8 — __ZThn88_N4Ogre4Font12loadResourceEPNS_8ResourceE
#[doc(alias = "non-virtual thunk toOgre::Font::loadResource(Ogre::Resource *)")]
// was: non-virtual thunk to Ogre::Font::loadResource(Ogre::Resource *)
// IDA 0xc92bf8: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c92bf8() {
}

// 0xc92c04 — __ZNK4Ogre4Font7CmdType5doGetEPKv
#[doc(alias = "Ogre::Font::CmdType::doGet(void const*)const")]
// was: Ogre::Font::CmdType::doGet(void const*)const
// IDA 0xc92c04: 66 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c92c04() {
}

// 0xc92cc4 — __ZN4Ogre4Font7CmdType5doSetEPvRKSs
#[doc(alias = "Ogre::Font::CmdType::doSet(void *,std::string const&)")]
// was: Ogre::Font::CmdType::doSet(void *,std::string const&)
// IDA 0xc92cc4: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c92cc4() {
}

// 0xc92ce8 — __ZNK4Ogre4Font9CmdSource5doGetEPKv
#[doc(alias = "Ogre::Font::CmdSource::doGet(void const*)const")]
// was: Ogre::Font::CmdSource::doGet(void const*)const
// IDA 0xc92ce8: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c92ce8() {
}

// 0xc92cf8 — __ZN4Ogre4Font9CmdSource5doSetEPvRKSs
#[doc(alias = "Ogre::Font::CmdSource::doSet(void *,std::string const&)")]
// was: Ogre::Font::CmdSource::doSet(void *,std::string const&)
// IDA 0xc92cf8: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c92cf8() {
}

// 0xc92d08 — __ZNK4Ogre4Font13CmdCharSpacer5doGetEPKv
#[doc(alias = "Ogre::Font::CmdCharSpacer::doGet(void const*)const")]
// was: Ogre::Font::CmdCharSpacer::doGet(void const*)const
// IDA 0xc92d08: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c92d08() {
}

// 0xc92d34 — __ZN4Ogre4Font13CmdCharSpacer5doSetEPvRKSs
#[doc(alias = "Ogre::Font::CmdCharSpacer::doSet(void *,std::string const&)")]
// was: Ogre::Font::CmdCharSpacer::doSet(void *,std::string const&)
// IDA 0xc92d34: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c92d34() {
}

// 0xc92d44 — __ZNK4Ogre4Font7CmdSize5doGetEPKv
#[doc(alias = "Ogre::Font::CmdSize::doGet(void const*)const")]
// was: Ogre::Font::CmdSize::doGet(void const*)const
// IDA 0xc92d44: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c92d44() {
}

// 0xc92d60 — __ZN4Ogre4Font7CmdSize5doSetEPvRKSs
#[doc(alias = "Ogre::Font::CmdSize::doSet(void *,std::string const&)")]
// was: Ogre::Font::CmdSize::doSet(void *,std::string const&)
// IDA 0xc92d60: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c92d60() {
}

// 0xc92d74 — __ZNK4Ogre4Font13CmdResolution5doGetEPKv
#[doc(alias = "Ogre::Font::CmdResolution::doGet(void const*)const")]
// was: Ogre::Font::CmdResolution::doGet(void const*)const
// IDA 0xc92d74: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c92d74() {
}

// 0xc92d8c — __ZN4Ogre4Font13CmdResolution5doSetEPvRKSs
#[doc(alias = "Ogre::Font::CmdResolution::doSet(void *,std::string const&)")]
// was: Ogre::Font::CmdResolution::doSet(void *,std::string const&)
// IDA 0xc92d8c: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c92d8c() {
}

// 0xc92da0 — __ZNK4Ogre4Font13CmdCodePoints5doGetEPKv
#[doc(alias = "Ogre::Font::CmdCodePoints::doGet(void const*)const")]
// was: Ogre::Font::CmdCodePoints::doGet(void const*)const
// IDA 0xc92da0: 212 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c92da0() {
}

// 0xc93004 — __ZN4Ogre4Font13CmdCodePoints5doSetEPvRKSs
#[doc(alias = "Ogre::Font::CmdCodePoints::doSet(void *,std::string const&)")]
// was: Ogre::Font::CmdCodePoints::doSet(void *,std::string const&)
// IDA 0xc93004: 385 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c93004() {
}

// 0xc9341c — __ZN4Ogre4Font7CmdTypeD1Ev
#[doc(alias = "Ogre::Font::CmdType::~CmdType()")]
// was: Ogre::Font::CmdType::~CmdType()
// IDA 0xc9341c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c9341c() {
}

// 0xc93420 — __ZN4Ogre4Font9CmdSourceD1Ev
#[doc(alias = "Ogre::Font::CmdSource::~CmdSource()")]
// was: Ogre::Font::CmdSource::~CmdSource()
// IDA 0xc93420: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c93420() {
}

// 0xc93424 — __ZN4Ogre4Font13CmdCharSpacerD1Ev
#[doc(alias = "Ogre::Font::CmdCharSpacer::~CmdCharSpacer()")]
// was: Ogre::Font::CmdCharSpacer::~CmdCharSpacer()
// IDA 0xc93424: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c93424() {
}

// 0xc93428 — __ZN4Ogre4Font7CmdSizeD1Ev
#[doc(alias = "Ogre::Font::CmdSize::~CmdSize()")]
// was: Ogre::Font::CmdSize::~CmdSize()
// IDA 0xc93428: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c93428() {
}

// 0xc9342c — __ZN4Ogre4Font13CmdResolutionD1Ev
#[doc(alias = "Ogre::Font::CmdResolution::~CmdResolution()")]
// was: Ogre::Font::CmdResolution::~CmdResolution()
// IDA 0xc9342c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c9342c() {
}

// 0xc93430 — __ZN4Ogre4Font13CmdCodePointsD1Ev
#[doc(alias = "Ogre::Font::CmdCodePoints::~CmdCodePoints()")]
// was: Ogre::Font::CmdCodePoints::~CmdCodePoints()
// IDA 0xc93430: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c93430() {
}

// 0xc93438 — __ZN4Ogre11MaterialPtrD1Ev
#[doc(alias = "Ogre::MaterialPtr::~MaterialPtr()")]
// was: Ogre::MaterialPtr::~MaterialPtr()
// IDA 0xc93438: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c93438() {
}

// 0xc93528 — __ZN4Ogre21ItemIdentityExceptionD1Ev
#[doc(alias = "Ogre::ItemIdentityException::~ItemIdentityException()")]
// was: Ogre::ItemIdentityException::~ItemIdentityException()
// IDA 0xc93528: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c93528() {
}

// 0xc93538 — __ZN4Ogre4Font7CmdTypeD0Ev
#[doc(alias = "Ogre::Font::CmdType::~CmdType()")]
// was: Ogre::Font::CmdType::~CmdType()
// IDA 0xc93538: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c93538() {
}

// 0xc93544 — __ZN4Ogre4Font9CmdSourceD0Ev
#[doc(alias = "Ogre::Font::CmdSource::~CmdSource()")]
// was: Ogre::Font::CmdSource::~CmdSource()
// IDA 0xc93544: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c93544() {
}

// 0xc93550 — __ZN4Ogre4Font13CmdCharSpacerD0Ev
#[doc(alias = "Ogre::Font::CmdCharSpacer::~CmdCharSpacer()")]
// was: Ogre::Font::CmdCharSpacer::~CmdCharSpacer()
// IDA 0xc93550: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c93550() {
}

// 0xc9355c — __ZN4Ogre4Font7CmdSizeD0Ev
#[doc(alias = "Ogre::Font::CmdSize::~CmdSize()")]
// was: Ogre::Font::CmdSize::~CmdSize()
// IDA 0xc9355c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c9355c() {
}

// 0xc93568 — __ZN4Ogre4Font13CmdResolutionD0Ev
#[doc(alias = "Ogre::Font::CmdResolution::~CmdResolution()")]
// was: Ogre::Font::CmdResolution::~CmdResolution()
// IDA 0xc93568: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c93568() {
}

// 0xc93574 — __ZN4Ogre4Font13CmdCodePointsD0Ev
#[doc(alias = "Ogre::Font::CmdCodePoints::~CmdCodePoints()")]
// was: Ogre::Font::CmdCodePoints::~CmdCodePoints()
// IDA 0xc93574: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c93574() {
}

// 0xc93580 — __ZNK4Ogre15StringInterface12getParameterERKSs
#[doc(alias = "Ogre::StringInterface::getParameter(std::string const&)const")]
// was: Ogre::StringInterface::getParameter(std::string const&)const
// IDA 0xc93580: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c93580() {
}

// 0xc935d0 — __ZN4Ogre8Resource11preLoadImplEv
#[doc(alias = "Ogre::Resource::preLoadImpl(void)")]
// was: Ogre::Resource::preLoadImpl(void)
// IDA 0xc935d0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c935d0() {
}

// 0xc935d8 — __ZN4Ogre8Resource13preUnloadImplEv
#[doc(alias = "Ogre::Resource::preUnloadImpl(void)")]
// was: Ogre::Resource::preUnloadImpl(void)
// IDA 0xc935d8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c935d8() {
}

// 0xc935e0 — __ZN4Ogre8Resource11prepareImplEv
#[doc(alias = "Ogre::Resource::prepareImpl(void)")]
// was: Ogre::Resource::prepareImpl(void)
// IDA 0xc935e0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c935e0() {
}

// 0xc935e8 — __ZNK4Ogre4Font13calculateSizeEv
#[doc(alias = "Ogre::Font::calculateSize(void)const")]
// was: Ogre::Font::calculateSize(void)const
// IDA 0xc935e8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c935e8() {
}

// 0xc935f0 — __ZNK4Ogre8Resource7getNameEv
#[doc(alias = "Ogre::Resource::getName(void)const")]
// was: Ogre::Resource::getName(void)const
// IDA 0xc935f0: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c935f0() {
}

// 0xc935f8 — __ZNK4Ogre8Resource8isLoadedEv
#[doc(alias = "Ogre::Resource::isLoaded(void)const")]
// was: Ogre::Resource::isLoaded(void)const
// IDA 0xc935f8: 6 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c935f8() {
}

// 0xc93608 — __ZNK4Ogre8Resource15getLoadingStateEv
#[doc(alias = "Ogre::Resource::getLoadingState(void)const")]
// was: Ogre::Resource::getLoadingState(void)const
// IDA 0xc93608: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c93608() {
}

// 0xc93610 — __ZN4Ogre8Resource10getCreatorEv
#[doc(alias = "Ogre::Resource::getCreator(void)")]
// was: Ogre::Resource::getCreator(void)
// IDA 0xc93610: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c93610() {
}

// 0xc93618 — __ZN4Ogre8Resource13_notifyOriginERKSs
#[doc(alias = "Ogre::Resource::_notifyOrigin(std::string const&)")]
// was: Ogre::Resource::_notifyOrigin(std::string const&)
// IDA 0xc93618: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c93618() {
}

// 0xc93628 — __ZN4Ogre20ManualResourceLoader15prepareResourceEPNS_8ResourceE
#[doc(alias = "Ogre::ManualResourceLoader::prepareResource(Ogre::Resource *)")]
// was: Ogre::ManualResourceLoader::prepareResource(Ogre::Resource *)
// IDA 0xc93628: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c93628() {
}

// 0xc9362c — __ZNSt6vectorIPKN4Ogre5ImageENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_
#[doc(alias = "std::vector<Ogre::Image const*,Ogre::STLAllocator<Ogre::Image const*,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Image const**,std::vector<Ogre::Image const*,Ogre::STLAllocator<Ogre::Image const*,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Image const* const&)")]
// was: std::vector<Ogre::Image const*,Ogre::STLAllocator<Ogre::Image const*,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Image const**,std::vector<Ogre::Image const*,Ogre::STLAllocator<Ogre::Image const*,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Image const* const&)
// IDA 0xc9362c: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_c9362c() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xc93728 — __ZNSt12_Vector_baseIPKN4Ogre5ImageENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::Image const*,Ogre::STLAllocator<Ogre::Image const*,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::Image const*,Ogre::STLAllocator<Ogre::Image const*,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc93728: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c93728() {
}

// 0xc9372c — __ZNSt12_Vector_baseIPKN4Ogre5ImageENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::Image const*,Ogre::STLAllocator<Ogre::Image const*,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::Image const*,Ogre::STLAllocator<Ogre::Image const*,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc9372c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c9372c() {
}

// 0xc93738 — __ZN4Ogre9SharedPtrINS_10DataStreamEED0Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::DataStream>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::DataStream>::~SharedPtr()
// IDA 0xc93738: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c93738() {
}

// 0xc93830 — __ZNSt12_Vector_baseISt4pairIjjEN4Ogre12STLAllocatorIS1_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<std::pair<unsigned int,unsigned int>,Ogre::STLAllocator<std::pair<unsigned int,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<std::pair<unsigned int,unsigned int>,Ogre::STLAllocator<std::pair<unsigned int,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc93830: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c93830() {
}

// 0xc93834 — __ZNSt12_Vector_baseISt4pairIjjEN4Ogre12STLAllocatorIS1_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<std::pair<unsigned int,unsigned int>,Ogre::STLAllocator<std::pair<unsigned int,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<std::pair<unsigned int,unsigned int>,Ogre::STLAllocator<std::pair<unsigned int,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc93834: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c93834() {
}

// 0xc93840 — __ZNKSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const
// IDA 0xc93840: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c93840() {
}

// 0xc93a68 — __ZN4Ogre9SharedPtrINS_7TextureEED0Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::Texture>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::Texture>::~SharedPtr()
// IDA 0xc93a68: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c93a68() {
}

// 0xc93b60 — __ZN4Ogre22InternalErrorExceptionD0Ev
#[doc(alias = "Ogre::InternalErrorException::~InternalErrorException()")]
// was: Ogre::InternalErrorException::~InternalErrorException()
// IDA 0xc93b60: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c93b60() {
}

// 0xc93b78 — __ZN4Ogre9SharedPtrINS_8MaterialEED0Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::Material>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::Material>::~SharedPtr()
// IDA 0xc93b78: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c93b78() {
}

// 0xc93c70 — __ZNSt6vectorIN4Ogre12ParameterDefENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE9push_backERKS1_
#[doc(alias = "std::vector<Ogre::ParameterDef,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::push_back(Ogre::ParameterDef const&)")]
// was: std::vector<Ogre::ParameterDef,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::push_back(Ogre::ParameterDef const&)
// IDA 0xc93c70: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_c93c70() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0xc93d90 — __ZNSt3mapISsPN4Ogre12ParamCommandESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_
#[doc(alias = "std::map<std::string,Ogre::ParamCommand *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
// was: std::map<std::string,Ogre::ParamCommand *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)
// IDA 0xc93d90: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c93d90() {
}

// 0xc93f50 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::ParamCommand *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::ParamCommand *> const&)
// IDA 0xc93f50: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c93f50() {
}

// 0xc94038 — __ZSt22__uninitialized_copy_aIPN4Ogre12ParameterDefES2_NS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEET0_T_S9_S8_T1_
#[doc(alias = "Ogre::ParameterDef * std::__uninitialized_copy_a<Ogre::ParameterDef *,Ogre::ParameterDef *,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(Ogre::ParameterDef *,Ogre::ParameterDef *,Ogre::ParameterDef *,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")]
// was: Ogre::ParameterDef * std::__uninitialized_copy_a<Ogre::ParameterDef *,Ogre::ParameterDef *,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(Ogre::ParameterDef *,Ogre::ParameterDef *,Ogre::ParameterDef *,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)
// IDA 0xc94038: 89 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c94038() {
}

// 0xc94238 — __ZN4Ogre12STLAllocatorINS_12ParameterDefENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEED0Ev
#[doc(alias = "Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()")]
// was: Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()
// IDA 0xc94238: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c94238() {
}

// 0xc94248 — __ZNSt12_Vector_baseIN4Ogre12ParameterDefENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::ParameterDef,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::ParameterDef,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc94248: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c94248() {
}

// 0xc94250 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xc94250: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c94250() {
}

// 0xc94260 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE7_M_copyEPKSt13_Rb_tree_nodeIS5_EPSH_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_copy(std::_Rb_tree_node<std::pair<std::string const,Ogre::ParamCommand *>> const*,std::_Rb_tree_node<std::pair<std::string const,Ogre::ParamCommand *>>*)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_copy(std::_Rb_tree_node<std::pair<std::string const,Ogre::ParamCommand *>> const*,std::_Rb_tree_node<std::pair<std::string const,Ogre::ParamCommand *>>*)
// IDA 0xc94260: 195 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c94260() {
}

// 0xc94450 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre15ParamDictionaryEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamDictionary>,std::_Select1st<std::pair<std::string const,Ogre::ParamDictionary>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamDictionary>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::ParamDictionary> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamDictionary>,std::_Select1st<std::pair<std::string const,Ogre::ParamDictionary>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamDictionary>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::ParamDictionary> const&)
// IDA 0xc94450: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c94450() {
}

// 0xc94538 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre15ParamDictionaryEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE14_M_create_nodeERKS4_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamDictionary>,std::_Select1st<std::pair<std::string const,Ogre::ParamDictionary>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamDictionary>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string const,Ogre::ParamDictionary> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamDictionary>,std::_Select1st<std::pair<std::string const,Ogre::ParamDictionary>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamDictionary>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string const,Ogre::ParamDictionary> const&)
// IDA 0xc94538: 106 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c94538() {
}

// 0xc947c0 — __ZN4Ogre11FontManager12getSingletonEv
#[doc(alias = "Ogre::FontManager::getSingleton(void)")]
// was: Ogre::FontManager::getSingleton(void)
// IDA 0xc947c0: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c947c0() {
}

// 0xc947d0 — __ZN4Ogre11FontManagerC1Ev
#[doc(alias = "Ogre::FontManager::FontManager(void)")]
// was: Ogre::FontManager::FontManager(void)
// IDA 0xc947d0: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c947d0() {
}

// 0xc947dc — __ZN4Ogre11FontManagerC2Ev
#[doc(alias = "Ogre::FontManager::FontManager(void)")]
// was: Ogre::FontManager::FontManager(void)
// IDA 0xc947dc: 171 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c947dc() {
}

// 0xc949c4 — __ZN4Ogre11FontManagerD0Ev
#[doc(alias = "Ogre::FontManager::~FontManager()")]
// was: Ogre::FontManager::~FontManager()
// IDA 0xc949c4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c949c4() {
}

// 0xc94abc — __ZN4Ogre11FontManagerD1Ev
#[doc(alias = "Ogre::FontManager::~FontManager()")]
// was: Ogre::FontManager::~FontManager()
// IDA 0xc94abc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c94abc() {
}

// 0xc94ba4 — __ZN4Ogre11FontManager10createImplERKSsyS2_bPNS_20ManualResourceLoaderEPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::FontManager::createImpl(std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")]
// was: Ogre::FontManager::createImpl(std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)
// IDA 0xc94ba4: 79 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c94ba4() {
}

// 0xc94c78 — __ZN4Ogre11FontManager11parseScriptERNS_9SharedPtrINS_10DataStreamEEERKSs
#[doc(alias = "Ogre::FontManager::parseScript(Ogre::SharedPtr<Ogre::DataStream> &,std::string const&)")]
// was: Ogre::FontManager::parseScript(Ogre::SharedPtr<Ogre::DataStream> &,std::string const&)
// IDA 0xc94c78: 676 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c94c78() {
}

// 0xc95368 — __ZN4Ogre11FontManager14parseAttributeERKSsRNS_7FontPtrE
#[doc(alias = "Ogre::FontManager::parseAttribute(std::string const&,Ogre::FontPtr &)")]
// was: Ogre::FontManager::parseAttribute(std::string const&,Ogre::FontPtr &)
// IDA 0xc95368: 891 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c95368() {
}

// 0xc95cfc — __ZN4Ogre11FontManager12logBadAttribERKSsRNS_7FontPtrE
#[doc(alias = "Ogre::FontManager::logBadAttrib(std::string const&,Ogre::FontPtr &)")]
// was: Ogre::FontManager::logBadAttrib(std::string const&,Ogre::FontPtr &)
// IDA 0xc95cfc: 285 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c95cfc() {
}

// 0xc96014 — __ZN4Ogre7FontPtrD1Ev
#[doc(alias = "Ogre::FontPtr::~FontPtr()")]
// was: Ogre::FontPtr::~FontPtr()
// IDA 0xc96014: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c96014() {
}

// 0xc96108 — __ZNK4Ogre15ResourceManager15getLoadingOrderEv
#[doc(alias = "Ogre::ResourceManager::getLoadingOrder(void)const")]
// was: Ogre::ResourceManager::getLoadingOrder(void)const
// IDA 0xc96108: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c96108() {
}

// 0xc9610c — __ZNK4Ogre15ResourceManager14getMemoryUsageEv
#[doc(alias = "Ogre::ResourceManager::getMemoryUsage(void)const")]
// was: Ogre::ResourceManager::getMemoryUsage(void)const
// IDA 0xc9610c: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c9610c() {
}