//! core shard ke — 150 stubs EA-sorted asc global gap filler not yet in core (fallback filter).
//! Source: `ida/export.json` (85545 funcs) EA-sorted asc, next 150 not yet in rbx_core after kd 0x789e50 (fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, boost; 25622 filtered, 5677->5527 gaps, 35159->35309 distinct, rbx_core::SharedPtr not boost).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + #[doc(alias = mangled)] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;


#[doc(alias = "ArchiveBinder::~ArchiveBinder()")]
#[doc(alias = "__ZN13ArchiveBinderD1Ev")]
// 0x789ea4 — __ZN13ArchiveBinderD1Ev
// type: void __fastcall(ArchiveBinder *__hidden this)
pub fn stub_0x789ea4() -> ! {
    todo!("0x789ea4 __ZN13ArchiveBinderD1Ev")
}

#[doc(alias = "MemoryBinder::~MemoryBinder()")]
#[doc(alias = "__ZN12MemoryBinderD1Ev")]
// 0x789fb4 — __ZN12MemoryBinderD1Ev
// type: void __fastcall(MemoryBinder *__hidden this)
pub fn stub_0x789fb4() -> ! {
    todo!("0x789fb4 __ZN12MemoryBinderD1Ev")
}

#[doc(alias = "MemoryBinder::resolveRefs(void)")]
#[doc(alias = "__ZN12MemoryBinder11resolveRefsEv")]
// 0x78a410 — __ZN12MemoryBinder11resolveRefsEv
// type: _DWORD __fastcall(MemoryBinder *__hidden this)
pub fn stub_0x78a410() -> ! {
    todo!("0x78a410 __ZN12MemoryBinder11resolveRefsEv")
}

#[doc(alias = "MemoryBinder::~MemoryBinder()")]
#[doc(alias = "__ZN12MemoryBinderD0Ev")]
// 0x78a47c — __ZN12MemoryBinderD0Ev
// type: void __fastcall(MemoryBinder *__hidden this)
pub fn stub_0x78a47c() -> ! {
    todo!("0x78a47c __ZN12MemoryBinderD0Ev")
}

#[doc(alias = "ArchiveBinder::~ArchiveBinder()")]
#[doc(alias = "__ZN13ArchiveBinderD0Ev")]
// 0x78b4b8 — __ZN13ArchiveBinderD0Ev
// type: void __fastcall(ArchiveBinder *__hidden this)
pub fn stub_0x78b4b8() -> ! {
    todo!("0x78b4b8 __ZN13ArchiveBinderD0Ev")
}

#[doc(alias = "ArchiveBinder::resolveIDREF(ArchiveBinder::IDREFBinding)")]
#[doc(alias = "__ZN13ArchiveBinder12resolveIDREFENS_12IDREFBindingE")]
// 0x78bf04 — __ZN13ArchiveBinder12resolveIDREFENS_12IDREFBindingE
pub fn stub_0x78bf04() -> ! {
    todo!("0x78bf04 __ZN13ArchiveBinder12resolveIDREFENS_12IDREFBindingE")
}

#[doc(alias = "global constructor keyed to_a_360")]
#[doc(alias = "__GLOBAL__I_a_360")]
// 0x78c2ac — __GLOBAL__I_a_360
pub fn stub_0x78c2ac() -> ! {
    todo!("0x78c2ac __GLOBAL__I_a_360")
}

#[doc(alias = "global constructor keyed to_a_361")]
#[doc(alias = "__GLOBAL__I_a_361")]
// 0x7986fc — __GLOBAL__I_a_361
pub fn stub_0x7986fc() -> ! {
    todo!("0x7986fc __GLOBAL__I_a_361")
}

#[doc(alias = "XmlElement::isXsiNil(void)const")]
#[doc(alias = "__ZNK10XmlElement8isXsiNilEv")]
// 0x79890c — __ZNK10XmlElement8isXsiNilEv
// type: int __fastcall(XmlElement *this)
pub fn stub_0x79890c() -> ! {
    todo!("0x79890c __ZNK10XmlElement8isXsiNilEv")
}

#[doc(alias = "XmlNameValuePair::getValue(bool &)const")]
#[doc(alias = "__ZNK16XmlNameValuePair8getValueERb")]
// 0x798964 — __ZNK16XmlNameValuePair8getValueERb
// type: int __fastcall(XmlNameValuePair *this, bool *)
pub fn stub_0x798964() -> ! {
    todo!("0x798964 __ZNK16XmlNameValuePair8getValueERb")
}

#[doc(alias = "XmlElement::findNextChildWithSameTag(XmlElement const*)const")]
#[doc(alias = "__ZNK10XmlElement24findNextChildWithSameTagEPKS_")]
// 0x7989bc — __ZNK10XmlElement24findNextChildWithSameTagEPKS_
// type: const XmlElement *__fastcall(XmlElement *this, const XmlElement *)
pub fn stub_0x7989bc() -> ! {
    todo!("0x7989bc __ZNK10XmlElement24findNextChildWithSameTagEPKS_")
}

#[doc(alias = "XmlNameValuePair::clearValue(void)const")]
#[doc(alias = "__ZNK16XmlNameValuePair10clearValueEv")]
// 0x7989ec — __ZNK16XmlNameValuePair10clearValueEv
// type: void __fastcall(std::string **this)
pub fn stub_0x7989ec() -> ! {
    todo!("0x7989ec __ZNK16XmlNameValuePair10clearValueEv")
}

#[doc(alias = "XmlNameValuePair::getValue(int &)const")]
#[doc(alias = "__ZNK16XmlNameValuePair8getValueERi")]
// 0x798d64 — __ZNK16XmlNameValuePair8getValueERi
// type: int __fastcall(XmlNameValuePair *this, int *)
pub fn stub_0x798d64() -> ! {
    todo!("0x798d64 __ZNK16XmlNameValuePair8getValueERi")
}

#[doc(alias = "XmlNameValuePair::getValue(unsigned int &)const")]
#[doc(alias = "__ZNK16XmlNameValuePair8getValueERj")]
// 0x798da4 — __ZNK16XmlNameValuePair8getValueERj
// type: int __fastcall(XmlNameValuePair *this, unsigned int *)
pub fn stub_0x798da4() -> ! {
    todo!("0x798da4 __ZNK16XmlNameValuePair8getValueERj")
}

#[doc(alias = "XmlNameValuePair::getValue(float &)const")]
#[doc(alias = "__ZNK16XmlNameValuePair8getValueERf")]
// 0x798de4 — __ZNK16XmlNameValuePair8getValueERf
// type: int __fastcall(XmlNameValuePair *this, float *)
pub fn stub_0x798de4() -> ! {
    todo!("0x798de4 __ZNK16XmlNameValuePair8getValueERf")
}

#[doc(alias = "XmlNameValuePair::getValue(double &)const")]
#[doc(alias = "__ZNK16XmlNameValuePair8getValueERd")]
// 0x798e7c — __ZNK16XmlNameValuePair8getValueERd
// type: int __fastcall(XmlNameValuePair *this, double *)
pub fn stub_0x798e7c() -> ! {
    todo!("0x798e7c __ZNK16XmlNameValuePair8getValueERd")
}

#[doc(alias = "XmlNameValuePair::toString(XmlWriter *)const")]
#[doc(alias = "__ZNK16XmlNameValuePair8toStringEP9XmlWriter")]
// 0x799060 — __ZNK16XmlNameValuePair8toStringEP9XmlWriter
// type: int __fastcall(std::string *, int, int)
pub fn stub_0x799060() -> ! {
    todo!("0x799060 __ZNK16XmlNameValuePair8toStringEP9XmlWriter")
}

#[doc(alias = "global constructor keyed to_a_362")]
#[doc(alias = "__GLOBAL__I_a_362")]
// 0x79972c — __GLOBAL__I_a_362
pub fn stub_0x79972c() -> ! {
    todo!("0x79972c __GLOBAL__I_a_362")
}

#[doc(alias = "TextXmlParser::skipWhitespace(void)")]
#[doc(alias = "__ZN13TextXmlParser14skipWhitespaceEv")]
// 0x799ee4 — __ZN13TextXmlParser14skipWhitespaceEv
// type: int __fastcall(TextXmlParser *this)
pub fn stub_0x799ee4() -> ! {
    todo!("0x799ee4 __ZN13TextXmlParser14skipWhitespaceEv")
}

#[doc(alias = "TextXmlParser::readFirstTag(void)")]
#[doc(alias = "__ZN13TextXmlParser12readFirstTagEv")]
// 0x799f34 — __ZN13TextXmlParser12readFirstTagEv
// type: void __fastcall(TextXmlParser *this, TextXmlParser *)
pub fn stub_0x799f34() -> ! {
    todo!("0x799f34 __ZN13TextXmlParser12readFirstTagEv")
}

#[doc(alias = "TextXmlParser::readTag(void)")]
#[doc(alias = "__ZN13TextXmlParser7readTagEv")]
// 0x79a2a8 — __ZN13TextXmlParser7readTagEv
// type: void __fastcall(TextXmlParser *this, TextXmlParser *)
pub fn stub_0x79a2a8() -> ! {
    todo!("0x79a2a8 __ZN13TextXmlParser7readTagEv")
}

#[doc(alias = "TextXmlParser::readText(bool)")]
#[doc(alias = "__ZN13TextXmlParser8readTextEb")]
// 0x79aca0 — __ZN13TextXmlParser8readTextEb
// type: void __fastcall(TextXmlParser *this, TextXmlParser *, int)
pub fn stub_0x79aca0() -> ! {
    todo!("0x79aca0 __ZN13TextXmlParser8readTextEb")
}

#[doc(alias = "TextXmlWriter::writeOpenTag(XmlElement const*,int,XmlAttribute const*)")]
#[doc(alias = "__ZN13TextXmlWriter12writeOpenTagEPK10XmlElementiPK12XmlAttribute")]
// 0x79af50 — __ZN13TextXmlWriter12writeOpenTagEPK10XmlElementiPK12XmlAttribute
// type: void __fastcall(int, int, int, int)
pub fn stub_0x79af50() -> ! {
    todo!("0x79af50 __ZN13TextXmlWriter12writeOpenTagEPK10XmlElementiPK12XmlAttribute")
}

#[doc(alias = "TextXmlWriter::writeCloseTag(XmlElement const*,int)")]
#[doc(alias = "__ZN13TextXmlWriter13writeCloseTagEPK10XmlElementi")]
// 0x79b250 — __ZN13TextXmlWriter13writeCloseTagEPK10XmlElementi
// type: int __fastcall(TextXmlWriter *this, const XmlElement *, int)
pub fn stub_0x79b250() -> ! {
    todo!("0x79b250 __ZN13TextXmlWriter13writeCloseTagEPK10XmlElementi")
}

#[doc(alias = "TextXmlParser::parse(void)")]
#[doc(alias = "__ZN13TextXmlParser5parseEv")]
// 0x79ba0c — __ZN13TextXmlParser5parseEv
// type: void __fastcall(TextXmlParser *this, TextXmlParser *)
pub fn stub_0x79ba0c() -> ! {
    todo!("0x79ba0c __ZN13TextXmlParser5parseEv")
}

#[doc(alias = "TextXmlWriter::serialize(XmlElement const*)")]
#[doc(alias = "__ZN13TextXmlWriter9serializeEPK10XmlElement")]
// 0x79c9ec — __ZN13TextXmlWriter9serializeEPK10XmlElement
// type: int __fastcall(TextXmlWriter *this, const XmlElement *)
pub fn stub_0x79c9ec() -> ! {
    todo!("0x79c9ec __ZN13TextXmlWriter9serializeEPK10XmlElement")
}

#[doc(alias = "TextXmlWriter::serialize(XmlElement const*,int)")]
#[doc(alias = "__ZN13TextXmlWriter9serializeEPK10XmlElementi")]
// 0x79c9f4 — __ZN13TextXmlWriter9serializeEPK10XmlElementi
// type: TextXmlWriter *__fastcall(TextXmlWriter *this, const XmlElement **, int)
pub fn stub_0x79c9f4() -> ! {
    todo!("0x79c9f4 __ZN13TextXmlWriter9serializeEPK10XmlElementi")
}

#[doc(alias = "TextXmlWriter::serializeNode(XmlElement const*,int)")]
#[doc(alias = "__ZN13TextXmlWriter13serializeNodeEPK10XmlElementi")]
// 0x79ca70 — __ZN13TextXmlWriter13serializeNodeEPK10XmlElementi
// type: void __fastcall(TextXmlWriter *this, const XmlElement *, int)
pub fn stub_0x79ca70() -> ! {
    todo!("0x79ca70 __ZN13TextXmlWriter13serializeNodeEPK10XmlElementi")
}

#[doc(alias = "global constructor keyed to_a_363")]
#[doc(alias = "__GLOBAL__I_a_363")]
// 0x79d364 — __GLOBAL__I_a_363
pub fn stub_0x79d364() -> ! {
    todo!("0x79d364 __GLOBAL__I_a_363")
}

#[doc(alias = "sub_7A3758")]
// 0x7a3758 — sub_7A3758
pub fn stub_0x7a3758() -> ! {
    todo!("0x7a3758 sub_7A3758")
}

#[doc(alias = "global constructor keyed to_a_364")]
#[doc(alias = "__GLOBAL__I_a_364")]
// 0x7aa43c — __GLOBAL__I_a_364
pub fn stub_0x7aa43c() -> ! {
    todo!("0x7aa43c __GLOBAL__I_a_364")
}

#[doc(alias = "global constructor keyed to_a_365")]
#[doc(alias = "__GLOBAL__I_a_365")]
// 0x7ab8c4 — __GLOBAL__I_a_365
pub fn stub_0x7ab8c4() -> ! {
    todo!("0x7ab8c4 __GLOBAL__I_a_365")
}

#[doc(alias = "global constructor keyed to_a_366")]
#[doc(alias = "__GLOBAL__I_a_366")]
// 0x7ac888 — __GLOBAL__I_a_366
pub fn stub_0x7ac888() -> ! {
    todo!("0x7ac888 __GLOBAL__I_a_366")
}

#[doc(alias = "global constructor keyed to_a_367")]
#[doc(alias = "__GLOBAL__I_a_367")]
// 0x7afc24 — __GLOBAL__I_a_367
pub fn stub_0x7afc24() -> ! {
    todo!("0x7afc24 __GLOBAL__I_a_367")
}

#[doc(alias = "global constructor keyed to_a_368")]
#[doc(alias = "__GLOBAL__I_a_368")]
// 0x7b1f98 — __GLOBAL__I_a_368
pub fn stub_0x7b1f98() -> ! {
    todo!("0x7b1f98 __GLOBAL__I_a_368")
}

#[doc(alias = "global constructor keyed to_a_369")]
#[doc(alias = "__GLOBAL__I_a_369")]
// 0x7b2eac — __GLOBAL__I_a_369
pub fn stub_0x7b2eac() -> ! {
    todo!("0x7b2eac __GLOBAL__I_a_369")
}

#[doc(alias = "global constructor keyed to_a_370")]
#[doc(alias = "__GLOBAL__I_a_370")]
// 0x7b3734 — __GLOBAL__I_a_370
pub fn stub_0x7b3734() -> ! {
    todo!("0x7b3734 __GLOBAL__I_a_370")
}

#[doc(alias = "global constructor keyed to_a_371")]
#[doc(alias = "__GLOBAL__I_a_371")]
// 0x7b3fa0 — __GLOBAL__I_a_371
pub fn stub_0x7b3fa0() -> ! {
    todo!("0x7b3fa0 __GLOBAL__I_a_371")
}

#[doc(alias = "global constructor keyed to_a_372")]
#[doc(alias = "__GLOBAL__I_a_372")]
// 0x7b4784 — __GLOBAL__I_a_372
pub fn stub_0x7b4784() -> ! {
    todo!("0x7b4784 __GLOBAL__I_a_372")
}

#[doc(alias = "global constructor keyed to_a_373")]
#[doc(alias = "__GLOBAL__I_a_373")]
// 0x7b4cb0 — __GLOBAL__I_a_373
pub fn stub_0x7b4cb0() -> ! {
    todo!("0x7b4cb0 __GLOBAL__I_a_373")
}

#[doc(alias = "global constructor keyed to_a_374")]
#[doc(alias = "__GLOBAL__I_a_374")]
// 0x7b592c — __GLOBAL__I_a_374
pub fn stub_0x7b592c() -> ! {
    todo!("0x7b592c __GLOBAL__I_a_374")
}

#[doc(alias = "global constructor keyed to_a_375")]
#[doc(alias = "__GLOBAL__I_a_375")]
// 0x7b5ea8 — __GLOBAL__I_a_375
pub fn stub_0x7b5ea8() -> ! {
    todo!("0x7b5ea8 __GLOBAL__I_a_375")
}

#[doc(alias = "___cxx_global_array_dtor")]
// 0x7cb7b4 — ___cxx_global_array_dtor
pub fn stub_0x7cb7b4() -> ! {
    todo!("0x7cb7b4 ___cxx_global_array_dtor")
}

#[doc(alias = "global constructor keyed to_a_376")]
#[doc(alias = "__GLOBAL__I_a_376")]
// 0x7cb890 — __GLOBAL__I_a_376
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0x7cb890() -> ! {
    todo!("0x7cb890 __GLOBAL__I_a_376")
}

#[doc(alias = "global constructor keyed to_a_377")]
#[doc(alias = "__GLOBAL__I_a_377")]
// 0x7d207c — __GLOBAL__I_a_377
pub fn stub_0x7d207c() -> ! {
    todo!("0x7d207c __GLOBAL__I_a_377")
}

#[doc(alias = "global constructor keyed to_a_378")]
#[doc(alias = "__GLOBAL__I_a_378")]
// 0x7d2fe4 — __GLOBAL__I_a_378
pub fn stub_0x7d2fe4() -> ! {
    todo!("0x7d2fe4 __GLOBAL__I_a_378")
}

#[doc(alias = "global constructor keyed to_a_379")]
#[doc(alias = "__GLOBAL__I_a_379")]
// 0x7d423c — __GLOBAL__I_a_379
pub fn stub_0x7d423c() -> ! {
    todo!("0x7d423c __GLOBAL__I_a_379")
}

#[doc(alias = "global constructor keyed to_a_380")]
#[doc(alias = "__GLOBAL__I_a_380")]
// 0x7d4f2c — __GLOBAL__I_a_380
pub fn stub_0x7d4f2c() -> ! {
    todo!("0x7d4f2c __GLOBAL__I_a_380")
}

#[doc(alias = "global constructor keyed to_a_381")]
#[doc(alias = "__GLOBAL__I_a_381")]
// 0x7d63ac — __GLOBAL__I_a_381
pub fn stub_0x7d63ac() -> ! {
    todo!("0x7d63ac __GLOBAL__I_a_381")
}

#[doc(alias = "global constructor keyed to_a_382")]
#[doc(alias = "__GLOBAL__I_a_382")]
// 0x7d69c8 — __GLOBAL__I_a_382
pub fn stub_0x7d69c8() -> ! {
    todo!("0x7d69c8 __GLOBAL__I_a_382")
}

#[doc(alias = "global constructor keyed to_a_383")]
#[doc(alias = "__GLOBAL__I_a_383")]
// 0x7d733c — __GLOBAL__I_a_383
pub fn stub_0x7d733c() -> ! {
    todo!("0x7d733c __GLOBAL__I_a_383")
}

#[doc(alias = "global constructor keyed to_a_384")]
#[doc(alias = "__GLOBAL__I_a_384")]
// 0x7d7800 — __GLOBAL__I_a_384
pub fn stub_0x7d7800() -> ! {
    todo!("0x7d7800 __GLOBAL__I_a_384")
}

#[doc(alias = "global constructor keyed to_a_385")]
#[doc(alias = "__GLOBAL__I_a_385")]
// 0x7d7f90 — __GLOBAL__I_a_385
pub fn stub_0x7d7f90() -> ! {
    todo!("0x7d7f90 __GLOBAL__I_a_385")
}

#[doc(alias = "global constructor keyed to_a_386")]
#[doc(alias = "__GLOBAL__I_a_386")]
// 0x7db638 — __GLOBAL__I_a_386
pub fn stub_0x7db638() -> ! {
    todo!("0x7db638 __GLOBAL__I_a_386")
}

#[doc(alias = "XmlNameValuePair::setValue(char const*)")]
#[doc(alias = "__ZN16XmlNameValuePair8setValueEPKc")]
// 0x7dc6cc — __ZN16XmlNameValuePair8setValueEPKc
// type: _DWORD __fastcall(XmlNameValuePair *__hidden this, const char *)
pub fn stub_0x7dc6cc() -> ! {
    todo!("0x7dc6cc __ZN16XmlNameValuePair8setValueEPKc")
}

#[doc(alias = "global constructor keyed to_a_387")]
#[doc(alias = "__GLOBAL__I_a_387")]
// 0x7dc784 — __GLOBAL__I_a_387
pub fn stub_0x7dc784() -> ! {
    todo!("0x7dc784 __GLOBAL__I_a_387")
}

#[doc(alias = "-[MacHttpController receivedData]")]
// 0x7dc98c — -[MacHttpController receivedData]
// type: id __cdecl(MacHttpController *self, SEL)
pub fn stub_0x7dc98c() -> ! {
    todo!("0x7dc98c -[MacHttpController receivedData]")
}

#[doc(alias = "-[MacHttpController initWithUrl:additionalHeaders:]")]
// 0x7dc99c — -[MacHttpController initWithUrl:additionalHeaders:]
// type: MacHttpController *__cdecl(MacHttpController *self, SEL, const basic_string<char, std::char_traits<char>, std::allocator<char> > *, const map<std::string, std::string, std::less<std::string >, std::allocator<std::pair<const std::string, std::string > > > *)
pub fn stub_0x7dc99c() -> ! {
    todo!("0x7dc99c -[MacHttpController initWithUrl:additionalHeaders:]")
}

#[doc(alias = "-[MacHttpController setPostDataFromStream:]")]
// 0x7dcc6c — -[MacHttpController setPostDataFromStream:]
// type: void __cdecl(MacHttpController *self, SEL, basic_istream<char, std::char_traits<char> > *)
pub fn stub_0x7dcc6c() -> ! {
    todo!("0x7dcc6c -[MacHttpController setPostDataFromStream:]")
}

#[doc(alias = "-[MacHttpController setPostCompressedDataFromString:]")]
// 0x7dcd0c — -[MacHttpController setPostCompressedDataFromString:]
// type: void __cdecl(MacHttpController *self, SEL, basic_string<char, std::char_traits<char>, std::allocator<char> > *)
pub fn stub_0x7dcd0c() -> ! {
    todo!("0x7dcd0c -[MacHttpController setPostCompressedDataFromString:]")
}

#[doc(alias = "-[MacHttpController dealloc]")]
// 0x7dcdbc — -[MacHttpController dealloc]
// type: void __cdecl(MacHttpController *self, SEL)
pub fn stub_0x7dcdbc() -> ! {
    todo!("0x7dcdbc -[MacHttpController dealloc]")
}

#[doc(alias = "-[MacHttpController setAuthDomain:withr:]")]
// 0x7dce08 — -[MacHttpController setAuthDomain:withr:]
// type: void __cdecl(MacHttpController *self, SEL, const basic_string<char, std::char_traits<char>, std::allocator<char> > *, id)
pub fn stub_0x7dce08() -> ! {
    todo!("0x7dce08 -[MacHttpController setAuthDomain:withr:]")
}

#[doc(alias = "-[MacHttpController configureRequest:]")]
// 0x7dce68 — -[MacHttpController configureRequest:]
// type: void __cdecl(MacHttpController *self, SEL, id)
pub fn stub_0x7dce68() -> ! {
    todo!("0x7dce68 -[MacHttpController configureRequest:]")
}

#[doc(alias = "-[MacHttpController startConnectionWithRequest:]")]
// 0x7dcfa4 — -[MacHttpController startConnectionWithRequest:]
// type: void __cdecl(MacHttpController *self, SEL, id)
pub fn stub_0x7dcfa4() -> ! {
    todo!("0x7dcfa4 -[MacHttpController startConnectionWithRequest:]")
}

#[doc(alias = "-[MacHttpController doGetPost:]")]
// 0x7dd034 — -[MacHttpController doGetPost:]
// type: int __cdecl(MacHttpController *self, SEL, const basic_string<char, std::char_traits<char>, std::allocator<char> > *)
pub fn stub_0x7dd034() -> ! {
    todo!("0x7dd034 -[MacHttpController doGetPost:]")
}

#[doc(alias = "-[MacHttpController connection:didFailWithError:]")]
// 0x7dd18c — -[MacHttpController connection:didFailWithError:]
// type: void __cdecl(MacHttpController *self, SEL, id, id)
pub fn stub_0x7dd18c() -> ! {
    todo!("0x7dd18c -[MacHttpController connection:didFailWithError:]")
}

#[doc(alias = "-[MacHttpController connection:didReceiveData:]")]
// 0x7dd1c4 — -[MacHttpController connection:didReceiveData:]
// type: void __cdecl(MacHttpController *self, SEL, id, id)
pub fn stub_0x7dd1c4() -> ! {
    todo!("0x7dd1c4 -[MacHttpController connection:didReceiveData:]")
}

#[doc(alias = "-[MacHttpController connection:didReceiveResponse:]")]
// 0x7dd1e4 — -[MacHttpController connection:didReceiveResponse:]
// type: void __cdecl(MacHttpController *self, SEL, id, id)
pub fn stub_0x7dd1e4() -> ! {
    todo!("0x7dd1e4 -[MacHttpController connection:didReceiveResponse:]")
}

#[doc(alias = "-[MacHttpController connectionDidFinishLoading:]")]
// 0x7dd24c — -[MacHttpController connectionDidFinishLoading:]
// type: void __cdecl(MacHttpController *self, SEL, id)
pub fn stub_0x7dd24c() -> ! {
    todo!("0x7dd24c -[MacHttpController connectionDidFinishLoading:]")
}

#[doc(alias = "-[MacHttpController connection:willSendRequest:redirectResponse:]")]
// 0x7dd260 — -[MacHttpController connection:willSendRequest:redirectResponse:]
// type: id __cdecl(MacHttpController *self, SEL, id, id, id)
pub fn stub_0x7dd260() -> ! {
    todo!("0x7dd260 -[MacHttpController connection:willSendRequest:redirectResponse:]")
}

#[doc(alias = "-[MacHttpController url]")]
// 0x7dd35c — -[MacHttpController url]
// type: NSURL *__cdecl(MacHttpController *self, SEL)
pub fn stub_0x7dd35c() -> ! {
    todo!("0x7dd35c -[MacHttpController url]")
}

#[doc(alias = "-[MacHttpController setUrl:]")]
// 0x7dd370 — -[MacHttpController setUrl:]
// type: void __cdecl(MacHttpController *self, SEL, id)
pub fn stub_0x7dd370() -> ! {
    todo!("0x7dd370 -[MacHttpController setUrl:]")
}

#[doc(alias = "-[MacHttpController .cxx_destruct]")]
// 0x7dd388 — -[MacHttpController .cxx_destruct]
// type: void __cdecl(MacHttpController *self, SEL)
pub fn stub_0x7dd388() -> ! {
    todo!("0x7dd388 -[MacHttpController .cxx_destruct]")
}

#[doc(alias = "-[MacHttpController .cxx_construct]")]
// 0x7dd39c — -[MacHttpController .cxx_construct]
// type: id __cdecl(MacHttpController *self, SEL)
pub fn stub_0x7dd39c() -> ! {
    todo!("0x7dd39c -[MacHttpController .cxx_construct]")
}

#[doc(alias = "rbx_isRobloxSite(char const*)")]
#[doc(alias = "__Z16rbx_isRobloxSitePKc")]
// 0x7dd3c0 — __Z16rbx_isRobloxSitePKc
// type: _DWORD __fastcall(const char *)
pub fn stub_0x7dd3c0() -> ! {
    todo!("0x7dd3c0 __Z16rbx_isRobloxSitePKc")
}

#[doc(alias = "global constructor keyed to_a_388")]
#[doc(alias = "__GLOBAL__I_a_388")]
// 0x7e436c — __GLOBAL__I_a_388
pub fn stub_0x7e436c() -> ! {
    todo!("0x7e436c __GLOBAL__I_a_388")
}

#[doc(alias = "global constructor keyed to_a_389")]
#[doc(alias = "__GLOBAL__I_a_389")]
// 0x7e6300 — __GLOBAL__I_a_389
pub fn stub_0x7e6300() -> ! {
    todo!("0x7e6300 __GLOBAL__I_a_389")
}

#[doc(alias = "global constructor keyed to_a_390")]
#[doc(alias = "__GLOBAL__I_a_390")]
// 0x7e755c — __GLOBAL__I_a_390
pub fn stub_0x7e755c() -> ! {
    todo!("0x7e755c __GLOBAL__I_a_390")
}

#[doc(alias = "global constructor keyed to_a_391")]
#[doc(alias = "__GLOBAL__I_a_391")]
// 0x7e89ac — __GLOBAL__I_a_391
pub fn stub_0x7e89ac() -> ! {
    todo!("0x7e89ac __GLOBAL__I_a_391")
}

#[doc(alias = "global constructor keyed to_a_392")]
#[doc(alias = "__GLOBAL__I_a_392")]
// 0x7e9fa8 — __GLOBAL__I_a_392
pub fn stub_0x7e9fa8() -> ! {
    todo!("0x7e9fa8 __GLOBAL__I_a_392")
}

#[doc(alias = "global constructor keyed to_a_393")]
#[doc(alias = "__GLOBAL__I_a_393")]
// 0x7f8f2c — __GLOBAL__I_a_393
pub fn stub_0x7f8f2c() -> ! {
    todo!("0x7f8f2c __GLOBAL__I_a_393")
}

#[doc(alias = "global constructor keyed to_a_394")]
#[doc(alias = "__GLOBAL__I_a_394")]
// 0x7fe228 — __GLOBAL__I_a_394
pub fn stub_0x7fe228() -> ! {
    todo!("0x7fe228 __GLOBAL__I_a_394")
}

#[doc(alias = "global constructor keyed to_a_395")]
#[doc(alias = "__GLOBAL__I_a_395")]
// 0x815660 — __GLOBAL__I_a_395
pub fn stub_0x815660() -> ! {
    todo!("0x815660 __GLOBAL__I_a_395")
}

#[doc(alias = "global constructor keyed to_a_396")]
#[doc(alias = "__GLOBAL__I_a_396")]
// 0x816be8 — __GLOBAL__I_a_396
pub fn stub_0x816be8() -> ! {
    todo!("0x816be8 __GLOBAL__I_a_396")
}

#[doc(alias = "global constructor keyed to_a_397")]
#[doc(alias = "__GLOBAL__I_a_397")]
// 0x816fa4 — __GLOBAL__I_a_397
pub fn stub_0x816fa4() -> ! {
    todo!("0x816fa4 __GLOBAL__I_a_397")
}

#[doc(alias = "global constructor keyed to_a_398")]
#[doc(alias = "__GLOBAL__I_a_398")]
// 0x8226f0 — __GLOBAL__I_a_398
pub fn stub_0x8226f0() -> ! {
    todo!("0x8226f0 __GLOBAL__I_a_398")
}

#[doc(alias = "RobloxExtraSpace::RobloxExtraSpace(RobloxExtraSpace*)")]
#[doc(alias = "__ZN16RobloxExtraSpaceC2EPS_")]
// 0x8238a8 — __ZN16RobloxExtraSpaceC2EPS_
// type: RobloxExtraSpace *__fastcall(RobloxExtraSpace *__hidden this, RobloxExtraSpace *)
pub fn stub_0x8238a8() -> ! {
    todo!("0x8238a8 __ZN16RobloxExtraSpaceC2EPS_")
}

#[doc(alias = "global constructor keyed to_a_399")]
#[doc(alias = "__GLOBAL__I_a_399")]
// 0x823f24 — __GLOBAL__I_a_399
pub fn stub_0x823f24() -> ! {
    todo!("0x823f24 __GLOBAL__I_a_399")
}

#[doc(alias = "l_alloc(void *,void *,unsigned long,unsigned long)")]
#[doc(alias = "__ZL7l_allocPvS_mm")]
// 0x824fd4 — __ZL7l_allocPvS_mm
// type: _DWORD __fastcall(void *, void *, unsigned int, size_t __size)
pub fn stub_0x824fd4() -> ! {
    todo!("0x824fd4 __ZL7l_allocPvS_mm")
}

#[doc(alias = "global constructor keyed to_a_400")]
#[doc(alias = "__GLOBAL__I_a_400")]
// 0x825024 — __GLOBAL__I_a_400
pub fn stub_0x825024() -> ! {
    todo!("0x825024 __GLOBAL__I_a_400")
}

#[doc(alias = "global constructor keyed to_a_401")]
#[doc(alias = "__GLOBAL__I_a_401")]
// 0x826288 — __GLOBAL__I_a_401
pub fn stub_0x826288() -> ! {
    todo!("0x826288 __GLOBAL__I_a_401")
}

#[doc(alias = "patchlistaux(FuncState *,int,int,int,int)")]
#[doc(alias = "__ZL12patchlistauxP9FuncStateiiii")]
// 0x8264d0 — __ZL12patchlistauxP9FuncStateiiii
pub fn stub_0x8264d0() -> ! {
    todo!("0x8264d0 __ZL12patchlistauxP9FuncStateiiii")
}

#[doc(alias = "fixjump(FuncState *,int,int)")]
#[doc(alias = "__ZL7fixjumpP9FuncStateii")]
// 0x826534 — __ZL7fixjumpP9FuncStateii
pub fn stub_0x826534() -> ! {
    todo!("0x826534 __ZL7fixjumpP9FuncStateii")
}

#[doc(alias = "exp2reg(FuncState *,expdesc *,int)")]
#[doc(alias = "__ZL7exp2regP9FuncStateP7expdesci")]
// 0x82687c — __ZL7exp2regP9FuncStateP7expdesci
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x82687c() -> ! {
    todo!("0x82687c __ZL7exp2regP9FuncStateP7expdesci")
}

#[doc(alias = "invertjump(FuncState *,expdesc *)")]
#[doc(alias = "__ZL10invertjumpP9FuncStateP7expdesc")]
// 0x826bec — __ZL10invertjumpP9FuncStateP7expdesc
pub fn stub_0x826bec() -> ! {
    todo!("0x826bec __ZL10invertjumpP9FuncStateP7expdesc")
}

#[doc(alias = "jumponcond(FuncState *,expdesc *,int)")]
#[doc(alias = "__ZL10jumponcondP9FuncStateP7expdesci")]
// 0x826c34 — __ZL10jumponcondP9FuncStateP7expdesci
pub fn stub_0x826c34() -> ! {
    todo!("0x826c34 __ZL10jumponcondP9FuncStateP7expdesci")
}

#[doc(alias = "codearith(FuncState *,OpCode,expdesc *,expdesc *)")]
#[doc(alias = "__ZL9codearithP9FuncState6OpCodeP7expdescS3_")]
// 0x826dd0 — __ZL9codearithP9FuncState6OpCodeP7expdescS3_
pub fn stub_0x826dd0() -> ! {
    todo!("0x826dd0 __ZL9codearithP9FuncState6OpCodeP7expdescS3_")
}

#[doc(alias = "codecomp(FuncState *,OpCode,int,expdesc *,expdesc *)")]
#[doc(alias = "__ZL8codecompP9FuncState6OpCodeiP7expdescS3_")]
// 0x827148 — __ZL8codecompP9FuncState6OpCodeiP7expdescS3_
pub fn stub_0x827148() -> ! {
    todo!("0x827148 __ZL8codecompP9FuncState6OpCodeiP7expdescS3_")
}

#[doc(alias = "discharge2anyreg(FuncState *,expdesc *)")]
#[doc(alias = "__ZL16discharge2anyregP9FuncStateP7expdesc")]
// 0x827320 — __ZL16discharge2anyregP9FuncStateP7expdesc
pub fn stub_0x827320() -> ! {
    todo!("0x827320 __ZL16discharge2anyregP9FuncStateP7expdesc")
}

#[doc(alias = "removevalues(FuncState *,int)")]
#[doc(alias = "__ZL12removevaluesP9FuncStatei")]
// 0x82734c — __ZL12removevaluesP9FuncStatei
pub fn stub_0x82734c() -> ! {
    todo!("0x82734c __ZL12removevaluesP9FuncStatei")
}

#[doc(alias = "patchtestreg(FuncState *,int,int)")]
#[doc(alias = "__ZL12patchtestregP9FuncStateii")]
// 0x82738c — __ZL12patchtestregP9FuncStateii
pub fn stub_0x82738c() -> ! {
    todo!("0x82738c __ZL12patchtestregP9FuncStateii")
}

#[doc(alias = "discharge2reg(FuncState *,expdesc *,int)")]
#[doc(alias = "__ZL13discharge2regP9FuncStateP7expdesci")]
// 0x8273f4 — __ZL13discharge2regP9FuncStateP7expdesci
// type: int __fastcall(_DWORD *, _DWORD *, int)
pub fn stub_0x8273f4() -> ! {
    todo!("0x8273f4 __ZL13discharge2regP9FuncStateP7expdesci")
}

#[doc(alias = "need_value(FuncState *,int)")]
#[doc(alias = "__ZL10need_valueP9FuncStatei")]
// 0x8274ac — __ZL10need_valueP9FuncStatei
pub fn stub_0x8274ac() -> ! {
    todo!("0x8274ac __ZL10need_valueP9FuncStatei")
}

#[doc(alias = "global constructor keyed to_a_402")]
#[doc(alias = "__GLOBAL__I_a_402")]
// 0x82751c — __GLOBAL__I_a_402
pub fn stub_0x82751c() -> ! {
    todo!("0x82751c __GLOBAL__I_a_402")
}

#[doc(alias = "symbexec(Proto const*,int,int)")]
#[doc(alias = "__ZL8symbexecPK5Protoii")]
// 0x827b2c — __ZL8symbexecPK5Protoii
pub fn stub_0x827b2c() -> ! {
    todo!("0x827b2c __ZL8symbexecPK5Protoii")
}

#[doc(alias = "checkArgMode(Proto const*,int,OpArgMask)")]
#[doc(alias = "__ZL12checkArgModePK5Protoi9OpArgMask")]
// 0x828404 — __ZL12checkArgModePK5Protoi9OpArgMask
pub fn stub_0x828404() -> ! {
    todo!("0x828404 __ZL12checkArgModePK5Protoi9OpArgMask")
}

#[doc(alias = "global constructor keyed to_a_403")]
#[doc(alias = "__GLOBAL__I_a_403")]
// 0x82843c — __GLOBAL__I_a_403
pub fn stub_0x82843c() -> ! {
    todo!("0x82843c __GLOBAL__I_a_403")
}

#[doc(alias = "global constructor keyed to_a_404")]
#[doc(alias = "__GLOBAL__I_a_404")]
// 0x8291d8 — __GLOBAL__I_a_404
pub fn stub_0x8291d8() -> ! {
    todo!("0x8291d8 __GLOBAL__I_a_404")
}

#[doc(alias = "DumpFunction(Proto const*,TString const*,DumpState *)")]
#[doc(alias = "__ZL12DumpFunctionPK5ProtoPK7TStringP9DumpState")]
// 0x829330 — __ZL12DumpFunctionPK5ProtoPK7TStringP9DumpState
pub fn stub_0x829330() -> ! {
    todo!("0x829330 __ZL12DumpFunctionPK5ProtoPK7TStringP9DumpState")
}

#[doc(alias = "DumpString(TString const*,DumpState *)")]
#[doc(alias = "__ZL10DumpStringPK7TStringP9DumpState")]
// 0x8295c0 — __ZL10DumpStringPK7TStringP9DumpState
pub fn stub_0x8295c0() -> ! {
    todo!("0x8295c0 __ZL10DumpStringPK7TStringP9DumpState")
}

#[doc(alias = "DumpVector(void const*,int,unsigned long,DumpState *)")]
#[doc(alias = "__ZL10DumpVectorPKvimP9DumpState")]
// 0x829614 — __ZL10DumpVectorPKvimP9DumpState
pub fn stub_0x829614() -> ! {
    todo!("0x829614 __ZL10DumpVectorPKvimP9DumpState")
}

#[doc(alias = "global constructor keyed to_a_405")]
#[doc(alias = "__GLOBAL__I_a_405")]
// 0x829654 — __GLOBAL__I_a_405
pub fn stub_0x829654() -> ! {
    todo!("0x829654 __GLOBAL__I_a_405")
}

#[doc(alias = "global constructor keyed to_a_406")]
#[doc(alias = "__GLOBAL__I_a_406")]
// 0x8299d8 — __GLOBAL__I_a_406
pub fn stub_0x8299d8() -> ! {
    todo!("0x8299d8 __GLOBAL__I_a_406")
}

#[doc(alias = "reallymarkobject(global_State *,GCObject *)")]
#[doc(alias = "__ZL16reallymarkobjectP12global_StateP8GCObject")]
// 0x82a138 — __ZL16reallymarkobjectP12global_StateP8GCObject
// type: int *__fastcall(int, int)
pub fn stub_0x82a138() -> ! {
    todo!("0x82a138 __ZL16reallymarkobjectP12global_StateP8GCObject")
}

#[doc(alias = "markmt(global_State *)")]
#[doc(alias = "__ZL6markmtP12global_State")]
// 0x82a238 — __ZL6markmtP12global_State
pub fn stub_0x82a238() -> ! {
    todo!("0x82a238 __ZL6markmtP12global_State")
}

#[doc(alias = "propagatemark(global_State *)")]
#[doc(alias = "__ZL13propagatemarkP12global_State")]
// 0x82a264 — __ZL13propagatemarkP12global_State
pub fn stub_0x82a264() -> ! {
    todo!("0x82a264 __ZL13propagatemarkP12global_State")
}

#[doc(alias = "global constructor keyed to_a_407")]
#[doc(alias = "__GLOBAL__I_a_407")]
// 0x82a740 — __GLOBAL__I_a_407
pub fn stub_0x82a740() -> ! {
    todo!("0x82a740 __GLOBAL__I_a_407")
}

#[doc(alias = "llex(LexState *,SemInfo *)")]
#[doc(alias = "__ZL4llexP8LexStateP7SemInfo")]
// 0x82aa20 — __ZL4llexP8LexStateP7SemInfo
pub fn stub_0x82aa20() -> ! {
    todo!("0x82aa20 __ZL4llexP8LexStateP7SemInfo")
}

#[doc(alias = "inclinenumber(LexState *)")]
#[doc(alias = "__ZL13inclinenumberP8LexState")]
// 0x82b004 — __ZL13inclinenumberP8LexState
pub fn stub_0x82b004() -> ! {
    todo!("0x82b004 __ZL13inclinenumberP8LexState")
}

#[doc(alias = "skip_sep(LexState *)")]
#[doc(alias = "__ZL8skip_sepP8LexState")]
// 0x82b078 — __ZL8skip_sepP8LexState
pub fn stub_0x82b078() -> ! {
    todo!("0x82b078 __ZL8skip_sepP8LexState")
}

#[doc(alias = "read_long_string(LexState *,SemInfo *,int)")]
#[doc(alias = "__ZL16read_long_stringP8LexStateP7SemInfoi")]
// 0x82b0dc — __ZL16read_long_stringP8LexStateP7SemInfoi
pub fn stub_0x82b0dc() -> ! {
    todo!("0x82b0dc __ZL16read_long_stringP8LexStateP7SemInfoi")
}

#[doc(alias = "save(LexState *,int)")]
#[doc(alias = "__ZL4saveP8LexStatei")]
// 0x82b1e4 — __ZL4saveP8LexStatei
pub fn stub_0x82b1e4() -> ! {
    todo!("0x82b1e4 __ZL4saveP8LexStatei")
}

#[doc(alias = "check_next(LexState *,char const*)")]
#[doc(alias = "__ZL10check_nextP8LexStatePKc")]
// 0x82b248 — __ZL10check_nextP8LexStatePKc
// type: int __fastcall(int, char *__s)
pub fn stub_0x82b248() -> ! {
    todo!("0x82b248 __ZL10check_nextP8LexStatePKc")
}

#[doc(alias = "read_numeral(LexState *,SemInfo *)")]
#[doc(alias = "__ZL12read_numeralP8LexStateP7SemInfo")]
// 0x82b288 — __ZL12read_numeralP8LexStateP7SemInfo
pub fn stub_0x82b288() -> ! {
    todo!("0x82b288 __ZL12read_numeralP8LexStateP7SemInfo")
}

#[doc(alias = "global constructor keyed to_a_408")]
#[doc(alias = "__GLOBAL__I_a_408")]
// 0x82b40c — __GLOBAL__I_a_408
pub fn stub_0x82b40c() -> ! {
    todo!("0x82b40c __GLOBAL__I_a_408")
}

#[doc(alias = "global constructor keyed to_a_409")]
#[doc(alias = "__GLOBAL__I_a_409")]
// 0x82bb10 — __GLOBAL__I_a_409
pub fn stub_0x82bb10() -> ! {
    todo!("0x82bb10 __GLOBAL__I_a_409")
}

#[doc(alias = "global constructor keyed to_a_410")]
#[doc(alias = "__GLOBAL__I_a_410")]
// 0x82bca8 — __GLOBAL__I_a_410
pub fn stub_0x82bca8() -> ! {
    todo!("0x82bca8 __GLOBAL__I_a_410")
}

#[doc(alias = "global constructor keyed to_a_411")]
#[doc(alias = "__GLOBAL__I_a_411")]
// 0x82c1a4 — __GLOBAL__I_a_411
pub fn stub_0x82c1a4() -> ! {
    todo!("0x82c1a4 __GLOBAL__I_a_411")
}

#[doc(alias = "global constructor keyed to_a_412")]
#[doc(alias = "__GLOBAL__I_a_412")]
// 0x82c26c — __GLOBAL__I_a_412
pub fn stub_0x82c26c() -> ! {
    todo!("0x82c26c __GLOBAL__I_a_412")
}

#[doc(alias = "open_func(LexState *,FuncState *)")]
#[doc(alias = "__ZL9open_funcP8LexStateP9FuncState")]
// 0x82c3a0 — __ZL9open_funcP8LexStateP9FuncState
pub fn stub_0x82c3a0() -> ! {
    todo!("0x82c3a0 __ZL9open_funcP8LexStateP9FuncState")
}

#[doc(alias = "chunk(LexState *)")]
#[doc(alias = "__ZL5chunkP8LexState")]
// 0x82c440 — __ZL5chunkP8LexState
pub fn stub_0x82c440() -> ! {
    todo!("0x82c440 __ZL5chunkP8LexState")
}

#[doc(alias = "close_func(LexState *)")]
#[doc(alias = "__ZL10close_funcP8LexState")]
// 0x82cb20 — __ZL10close_funcP8LexState
pub fn stub_0x82cb20() -> ! {
    todo!("0x82cb20 __ZL10close_funcP8LexState")
}

#[doc(alias = "enterlevel(LexState *)")]
#[doc(alias = "__ZL10enterlevelP8LexState")]
// 0x82ccd8 — __ZL10enterlevelP8LexState
pub fn stub_0x82ccd8() -> ! {
    todo!("0x82ccd8 __ZL10enterlevelP8LexState")
}

#[doc(alias = "block(LexState *)")]
#[doc(alias = "__ZL5blockP8LexState")]
// 0x82ccf8 — __ZL5blockP8LexState
pub fn stub_0x82ccf8() -> ! {
    todo!("0x82ccf8 __ZL5blockP8LexState")
}

#[doc(alias = "check_match(LexState *,int,int,int)")]
#[doc(alias = "__ZL11check_matchP8LexStateiii")]
// 0x82cd30 — __ZL11check_matchP8LexStateiii
// type: int __fastcall(int)
pub fn stub_0x82cd30() -> ! {
    todo!("0x82cd30 __ZL11check_matchP8LexStateiii")
}

#[doc(alias = "breakstat(LexState *)")]
#[doc(alias = "__ZL9breakstatP8LexState")]
// 0x82cda8 — __ZL9breakstatP8LexState
pub fn stub_0x82cda8() -> ! {
    todo!("0x82cda8 __ZL9breakstatP8LexState")
}

#[doc(alias = "primaryexp(LexState *,expdesc *)")]
#[doc(alias = "__ZL10primaryexpP8LexStateP7expdesc")]
// 0x82ce00 — __ZL10primaryexpP8LexStateP7expdesc
pub fn stub_0x82ce00() -> ! {
    todo!("0x82ce00 __ZL10primaryexpP8LexStateP7expdesc")
}

#[doc(alias = "assignment(LexState *,LHS_assign *,int)")]
#[doc(alias = "__ZL10assignmentP8LexStateP10LHS_assigni")]
// 0x82cf08 — __ZL10assignmentP8LexStateP10LHS_assigni
pub fn stub_0x82cf08() -> ! {
    todo!("0x82cf08 __ZL10assignmentP8LexStateP10LHS_assigni")
}

#[doc(alias = "errorlimit(FuncState *,int,char const*)")]
#[doc(alias = "__ZL10errorlimitP9FuncStateiPKc")]
// 0x82d02c — __ZL10errorlimitP9FuncStateiPKc
pub fn stub_0x82d02c() -> ! {
    todo!("0x82d02c __ZL10errorlimitP9FuncStateiPKc")
}

#[doc(alias = "checknext(LexState *,int)")]
#[doc(alias = "__ZL9checknextP8LexStatei")]
// 0x82d074 — __ZL9checknextP8LexStatei
pub fn stub_0x82d074() -> ! {
    todo!("0x82d074 __ZL9checknextP8LexStatei")
}

#[doc(alias = "explist1(LexState *,expdesc *)")]
#[doc(alias = "__ZL8explist1P8LexStateP7expdesc")]
// 0x82d090 — __ZL8explist1P8LexStateP7expdesc
pub fn stub_0x82d090() -> ! {
    todo!("0x82d090 __ZL8explist1P8LexStateP7expdesc")
}

#[doc(alias = "adjust_assign(LexState *,int,int,expdesc *)")]
#[doc(alias = "__ZL13adjust_assignP8LexStateiiP7expdesc")]
// 0x82d0cc — __ZL13adjust_assignP8LexStateiiP7expdesc
pub fn stub_0x82d0cc() -> ! {
    todo!("0x82d0cc __ZL13adjust_assignP8LexStateiiP7expdesc")
}

#[doc(alias = "subexpr(LexState *,expdesc *,unsigned int)")]
#[doc(alias = "__ZL7subexprP8LexStateP7expdescj")]
// 0x82d12c — __ZL7subexprP8LexStateP7expdescj
pub fn stub_0x82d12c() -> ! {
    todo!("0x82d12c __ZL7subexprP8LexStateP7expdescj")
}

#[doc(alias = "constructor(LexState *,expdesc *)")]
#[doc(alias = "__ZL11constructorP8LexStateP7expdesc")]
// 0x82d3d0 — __ZL11constructorP8LexStateP7expdesc
pub fn stub_0x82d3d0() -> ! {
    todo!("0x82d3d0 __ZL11constructorP8LexStateP7expdesc")
}

#[doc(alias = "body(LexState *,expdesc *,int,int)")]
#[doc(alias = "__ZL4bodyP8LexStateP7expdescii")]
// 0x82d530 — __ZL4bodyP8LexStateP7expdescii
pub fn stub_0x82d530() -> ! {
    todo!("0x82d530 __ZL4bodyP8LexStateP7expdescii")
}

#[doc(alias = "new_localvar(LexState *,TString *,int)")]
#[doc(alias = "__ZL12new_localvarP8LexStateP7TStringi")]
// 0x82d734 — __ZL12new_localvarP8LexStateP7TStringi
pub fn stub_0x82d734() -> ! {
    todo!("0x82d734 __ZL12new_localvarP8LexStateP7TStringi")
}

#[doc(alias = "adjustlocalvars(LexState *,int)")]
#[doc(alias = "__ZL15adjustlocalvarsP8LexStatei")]
// 0x82d81c — __ZL15adjustlocalvarsP8LexStatei
pub fn stub_0x82d81c() -> ! {
    todo!("0x82d81c __ZL15adjustlocalvarsP8LexStatei")
}

#[doc(alias = "str_checkname(LexState *)")]
#[doc(alias = "__ZL13str_checknameP8LexState")]
// 0x82d860 — __ZL13str_checknameP8LexState
pub fn stub_0x82d860() -> ! {
    todo!("0x82d860 __ZL13str_checknameP8LexState")
}

#[doc(alias = "listfield(LexState *,ConsControl *)")]
#[doc(alias = "__ZL9listfieldP8LexStateP11ConsControl")]
// 0x82d888 — __ZL9listfieldP8LexStateP11ConsControl
pub fn stub_0x82d888() -> ! {
    todo!("0x82d888 __ZL9listfieldP8LexStateP11ConsControl")
}

#[doc(alias = "recfield(LexState *,ConsControl *)")]
#[doc(alias = "__ZL8recfieldP8LexStateP11ConsControl")]
// 0x82d8cc — __ZL8recfieldP8LexStateP11ConsControl
pub fn stub_0x82d8cc() -> ! {
    todo!("0x82d8cc __ZL8recfieldP8LexStateP11ConsControl")
}
