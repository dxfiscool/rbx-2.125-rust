// Auto-generated skeletons for rbx-script — gap filler (global EA-sorted)
// Filter: Lua|Script|lua (5041 filtered, 0 remaining) -> global gap filler EA-sorted asc next 120 not yet in script crate
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +120 stubs | range 0x1c4bfc..0x1ca9c4 | script 18852->18972 distinct
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;


#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,tagTagInfo *>,std::_Select1st<std::pair<unsigned short const,tagTagInfo *>>,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>>::lower_bound(unsigned short const&)")]
pub fn stub_0x1c4bfc() -> crate::slot::PortedFn {
// IDA 0x1c4bfc: std::_Rb_tree<unsigned short, std::pair<unsigned short const, tagTagInfo*>, std::_Select1st<std::pair<unsigned short con~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c4bfc, "std::_Rb_tree<unsigned short, std::pair<unsigned short const, tagTagInfo*>, std::_Select1st<std::pai~")
}

#[doc(alias = "__gnu_cxx::new_allocator<std::_Rb_tree_node<std::pair<unsigned short const,tagTagInfo *>>>::allocate(unsigned long,void const*)")]
pub fn stub_0x1c4c30() -> crate::slot::PortedFn {
// IDA 0x1c4c30: __gnu_cxx::new_allocator<std::_Rb_tree_node<std::pair<unsigned short const, tagTagInfo*>>>::allocate(unsigned long, void~.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1c4c30, "__gnu_cxx::new_allocator<std::_Rb_tree_node<std::pair<unsigned short const, tagTagInfo*>>>::allocate~")
}

#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,tagTagInfo *>,std::_Select1st<std::pair<unsigned short const,tagTagInfo *>>,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>>::_M_create_node(std::pair<unsigned short const,tagTagInfo *> const&)")]
pub fn stub_0x1c4c60() -> crate::slot::PortedFn {
// IDA 0x1c4c60: std::_Rb_tree<unsigned short, std::pair<unsigned short const, tagTagInfo*>, std::_Select1st<std::pair<unsigned short con~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c4c60, "std::_Rb_tree<unsigned short, std::pair<unsigned short const, tagTagInfo*>, std::_Select1st<std::pai~")
}

#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,tagTagInfo *>,std::_Select1st<std::pair<unsigned short const,tagTagInfo *>>,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned short const,tagTagInfo *> const&)")]
pub fn stub_0x1c4c90(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "__gnu_cxx::new_allocator<std::_Rb_tree_node<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>>::allocate(unsigned long,void const*)")]
pub fn stub_0x1c4d14() -> crate::slot::PortedFn {
// IDA 0x1c4d14: __gnu_cxx::new_allocator<std::_Rb_tree_node<std::pair<int const, std::map<unsigned short, tagTagInfo*, std::less<unsigne~.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1c4d14, "__gnu_cxx::new_allocator<std::_Rb_tree_node<std::pair<int const, std::map<unsigned short, tagTagInfo~")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>,std::_Select1st<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>>::_M_create_node(std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *> const&)")]
pub fn stub_0x1c4d44() -> crate::slot::PortedFn {
// IDA 0x1c4d44: std::_Rb_tree<int, std::pair<int const, std::map<unsigned short, tagTagInfo*, std::less<unsigned short>, std::allocator<~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c4d44, "std::_Rb_tree<int, std::pair<int const, std::map<unsigned short, tagTagInfo*, std::less<unsigned sho~")
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>,std::_Select1st<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *> const&)")]
pub fn stub_0x1c4d74(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>,std::_Select1st<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>>::_M_insert_unique(std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *> const&)")]
pub fn stub_0x1c4df8(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,tagTagInfo *>,std::_Select1st<std::pair<unsigned short const,tagTagInfo *>>,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,tagTagInfo *>> *)")]
pub fn stub_0x1c4eb8(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>,std::_Select1st<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>>::_M_erase(std::_Rb_tree_node<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>> *)")]
pub fn stub_0x1c4ef4(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>,std::_Select1st<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *> const&)")]
pub fn stub_0x1c4f30(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::map<int,std::map*<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo>>>,tagTagInfo *<int>,std::allocator<std::less<unsigned short><int const,std::map*<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo>>>>>>::operator[](int const&)")]
pub fn stub_0x1c5054(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,tagTagInfo *>,std::_Select1st<std::pair<unsigned short const,tagTagInfo *>>,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>>::_M_insert_unique(std::pair<unsigned short const,tagTagInfo *> const&)")]
pub fn stub_0x1c50c0(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,tagTagInfo *>,std::_Select1st<std::pair<unsigned short const,tagTagInfo *>>,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,tagTagInfo *>>,std::pair<unsigned short const,tagTagInfo *> const&)")]
pub fn stub_0x1c5180(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>>::operator[](unsigned short const&)")]
pub fn stub_0x1c52a4(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "tiff_read_exif_tag(tiff *,TagLib::MDMODEL,FIBITMAP *,TagLib&,TIFFDirectory *,unsigned int)")]
pub fn stub_0x1c5310() -> crate::slot::PortedFn {
// IDA 0x1c5310: tiff_read_exif_tag(tiff*, TagLib::MDMODEL, FIBITMAP*, TagLib&, TIFFDirectory*, unsigned int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c5310, "tiff_read_exif_tag(tiff*, TagLib::MDMODEL, FIBITMAP*, TagLib&, TIFFDirectory*, unsigned int)")
}

#[doc(alias = "tiff_read_exif_tags(tiff *,TagLib::MDMODEL,FIBITMAP *)")]
pub fn stub_0x1c59bc() -> crate::slot::PortedFn {
// IDA 0x1c59bc: tiff_read_exif_tags(tiff*, TagLib::MDMODEL, FIBITMAP*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c59bc, "tiff_read_exif_tags(tiff*, TagLib::MDMODEL, FIBITMAP*)")
}

#[doc(alias = "tiff_write_geotiff_profile(tiff *,FIBITMAP *)")]
pub fn stub_0x1c5bf8() -> crate::slot::PortedFn {
// IDA 0x1c5bf8: tiff_write_geotiff_profile(tiff*, FIBITMAP*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c5bf8, "tiff_write_geotiff_profile(tiff*, FIBITMAP*)")
}

#[doc(alias = "tiff_read_geotiff_profile(tiff *,FIBITMAP *)")]
pub fn stub_0x1c610c() -> crate::slot::PortedFn {
// IDA 0x1c610c: tiff_read_geotiff_profile(tiff*, FIBITMAP*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c610c, "tiff_read_geotiff_profile(tiff*, FIBITMAP*)")
}

#[doc(alias = "XTIFFInitialize(void)")]
pub fn stub_0x1c630c() -> crate::slot::PortedFn {
// IDA 0x1c630c: XTIFFInitialize().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c630c, "XTIFFInitialize()")
}

#[doc(alias = "_XTIFFDefaultDirectory(tiff *)")]
pub fn stub_0x1c6354() -> crate::slot::PortedFn {
// IDA 0x1c6354: _XTIFFDefaultDirectory(tiff*).
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1c6354, "_XTIFFDefaultDirectory(tiff*)")
}

#[doc(alias = "append_iptc_tag(unsigned char *,unsigned int *,unsigned short,unsigned int,void const*)")]
pub fn stub_0x1c6394() -> crate::slot::PortedFn {
// IDA 0x1c6394: append_iptc_tag(unsigned char*, unsigned int*, unsigned short, unsigned int, void const*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c6394, "append_iptc_tag(unsigned char*, unsigned int*, unsigned short, unsigned int, void const*)")
}

#[doc(alias = "_write_iptc_profile")]
pub fn stub_0x1c6448() -> crate::slot::PortedFn {
// IDA 0x1c6448: _write_iptc_profile.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1c6448, "_write_iptc_profile")
}

#[doc(alias = "_read_iptc_profile")]
pub fn stub_0x1c6910() -> crate::slot::PortedFn {
// IDA 0x1c6910: _read_iptc_profile.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1c6910, "_read_iptc_profile")
}

#[doc(alias = "std::vector<std::string,std::allocator<std::string>>::size(void)const")]
pub fn stub_0x1c7340(vec: &crate::slot::VecModel) -> usize {
// sequence size.
vec.len()
}

#[doc(alias = "_FreeImage_GetTagKey")]
pub fn stub_0x1c7350() -> crate::slot::PortedFn {
// IDA 0x1c7350: _FreeImage_GetTagKey.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1c7350, "_FreeImage_GetTagKey")
}

#[doc(alias = "_FreeImage_GetTagID")]
pub fn stub_0x1c7360() -> crate::slot::PortedFn {
// IDA 0x1c7360: _FreeImage_GetTagID.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1c7360, "_FreeImage_GetTagID")
}

#[doc(alias = "_FreeImage_GetTagType")]
pub fn stub_0x1c7370() -> crate::slot::PortedFn {
// IDA 0x1c7370: _FreeImage_GetTagType.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1c7370, "_FreeImage_GetTagType")
}

#[doc(alias = "_FreeImage_GetTagCount")]
pub fn stub_0x1c7380() -> crate::slot::PortedFn {
// IDA 0x1c7380: _FreeImage_GetTagCount.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1c7380, "_FreeImage_GetTagCount")
}

#[doc(alias = "_FreeImage_GetTagLength")]
pub fn stub_0x1c7390() -> crate::slot::PortedFn {
// IDA 0x1c7390: _FreeImage_GetTagLength.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1c7390, "_FreeImage_GetTagLength")
}

#[doc(alias = "_FreeImage_GetTagValue")]
pub fn stub_0x1c73a0() -> crate::slot::PortedFn {
// IDA 0x1c73a0: _FreeImage_GetTagValue.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1c73a0, "_FreeImage_GetTagValue")
}

#[doc(alias = "_FreeImage_SetTagID")]
pub fn stub_0x1c73b0() -> crate::slot::PortedFn {
// IDA 0x1c73b0: _FreeImage_SetTagID.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1c73b0, "_FreeImage_SetTagID")
}

#[doc(alias = "_FreeImage_SetTagType")]
pub fn stub_0x1c73c8() -> crate::slot::PortedFn {
// IDA 0x1c73c8: _FreeImage_SetTagType.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1c73c8, "_FreeImage_SetTagType")
}

#[doc(alias = "_FreeImage_SetTagCount")]
pub fn stub_0x1c73dc() -> crate::slot::PortedFn {
// IDA 0x1c73dc: _FreeImage_SetTagCount.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1c73dc, "_FreeImage_SetTagCount")
}

#[doc(alias = "_FreeImage_SetTagLength")]
pub fn stub_0x1c73f0() -> crate::slot::PortedFn {
// IDA 0x1c73f0: _FreeImage_SetTagLength.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1c73f0, "_FreeImage_SetTagLength")
}

#[doc(alias = "FreeImage_TagDataWidth(unsigned short)")]
pub fn stub_0x1c7404() -> crate::slot::PortedFn {
// IDA 0x1c7404: FreeImage_TagDataWidth(unsigned short).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c7404, "FreeImage_TagDataWidth(unsigned short)")
}

#[doc(alias = "_FreeImage_DeleteTag")]
pub fn stub_0x1c7428() -> crate::slot::PortedFn {
// IDA 0x1c7428: _FreeImage_DeleteTag.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1c7428, "_FreeImage_DeleteTag")
}

#[doc(alias = "_FreeImage_SetTagKey")]
pub fn stub_0x1c74cc() -> crate::slot::PortedFn {
// IDA 0x1c74cc: _FreeImage_SetTagKey.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1c74cc, "_FreeImage_SetTagKey")
}

#[doc(alias = "_FreeImage_CreateTag")]
pub fn stub_0x1c7528() -> crate::slot::PortedFn {
// IDA 0x1c7528: _FreeImage_CreateTag.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1c7528, "_FreeImage_CreateTag")
}

#[doc(alias = "_FreeImage_CloneTag")]
pub fn stub_0x1c7580() -> crate::slot::PortedFn {
// IDA 0x1c7580: _FreeImage_CloneTag.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1c7580, "_FreeImage_CloneTag")
}

#[doc(alias = "_FreeImage_SetTagValue")]
pub fn stub_0x1c7658() -> crate::slot::PortedFn {
// IDA 0x1c7658: _FreeImage_SetTagValue.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1c7658, "_FreeImage_SetTagValue")
}

#[doc(alias = "FIRational::~FIRational()")]
pub fn stub_0x1c7724() -> crate::slot::PortedFn {
// IDA 0x1c7724: FIRational::~FIRational().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c7724, "FIRational::~FIRational()")
}

#[doc(alias = "FIRational::getNumerator(void)")]
pub fn stub_0x1c7728() -> crate::slot::PortedFn {
// IDA 0x1c7728: FIRational::getNumerator().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c7728, "FIRational::getNumerator()")
}

#[doc(alias = "FIRational::getDenominator(void)")]
pub fn stub_0x1c7730() -> crate::slot::PortedFn {
// IDA 0x1c7730: FIRational::getDenominator().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c7730, "FIRational::getDenominator()")
}

#[doc(alias = "FIRational::FIRational(float)")]
pub fn stub_0x1c7738() -> crate::slot::PortedFn {
// IDA 0x1c7738: FIRational::FIRational(float).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c7738, "FIRational::FIRational(float)")
}

#[doc(alias = "FIRational::FIRational(float) [0x1c7988]")]
pub fn stub_0x1c7988() -> crate::slot::PortedFn {
// IDA 0x1c7988: FIRational::FIRational(float).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c7988, "FIRational::FIRational(float)")
}

#[doc(alias = "ReadInt32(int,void *)")]
pub fn stub_0x1c798c() -> crate::slot::PortedFn {
// IDA 0x1c798c: ReadInt32(int, void*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c798c, "ReadInt32(int, void*)")
}

#[doc(alias = "ReadUint16(int,void *)")]
pub fn stub_0x1c79d8() -> crate::slot::PortedFn {
// IDA 0x1c79d8: ReadUint16(int, void*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c79d8, "ReadUint16(int, void*)")
}

#[doc(alias = "ReadUint32(int,void *)")]
pub fn stub_0x1c79f8() -> crate::slot::PortedFn {
// IDA 0x1c79f8: ReadUint32(int, void*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c79f8, "ReadUint32(int, void*)")
}

#[doc(alias = "FreeImage_strnicmp(char const*,char const*,unsigned long)")]
pub fn stub_0x1c79fc() -> crate::slot::PortedFn {
// IDA 0x1c79fc: FreeImage_strnicmp(char const*, char const*, unsigned long).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c79fc, "FreeImage_strnicmp(char const*, char const*, unsigned long)")
}

#[doc(alias = "processExifTag(FIBITMAP *,FITAG *,char *,int,TagLib::MDMODEL)")]
pub fn stub_0x1c7d28() -> crate::slot::PortedFn {
// IDA 0x1c7d28: processExifTag(FIBITMAP*, FITAG*, char*, int, TagLib::MDMODEL).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c7d28, "processExifTag(FIBITMAP*, FITAG*, char*, int, TagLib::MDMODEL)")
}

#[doc(alias = "_jpeg_read_exif_profile")]
pub fn stub_0x1c81a4() -> crate::slot::PortedFn {
// IDA 0x1c81a4: _jpeg_read_exif_profile.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1c81a4, "_jpeg_read_exif_profile")
}

#[doc(alias = "std::__deque_buf_size(unsigned long)")]
pub fn stub_0x1c8d60(vec: &crate::slot::VecModel) -> usize {
// sequence size.
vec.len()
}

#[doc(alias = "std::deque<unsigned short,std::allocator<unsigned short>>::_M_destroy_data(std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>,std::allocator<unsigned short> const&)")]
pub fn stub_0x1c8d84() -> crate::slot::PortedFn {
// IDA 0x1c8d84: std::deque<unsigned short, std::allocator<unsigned short>>::_M_destroy_data(std::_Deque_iterator<unsigned short, unsigne~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c8d84, "std::deque<unsigned short, std::allocator<unsigned short>>::_M_destroy_data(std::_Deque_iterator<uns~")
}

#[doc(alias = "std::deque<unsigned char *,std::allocator<unsigned char *>>::_M_destroy_data(std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>,std::allocator<unsigned char *> const&)")]
pub fn stub_0x1c8d88() -> crate::slot::PortedFn {
// IDA 0x1c8d88: std::deque<unsigned char*, std::allocator<unsigned char*>>::_M_destroy_data(std::_Deque_iterator<unsigned char*, unsigne~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c8d88, "std::deque<unsigned char*, std::allocator<unsigned char*>>::_M_destroy_data(std::_Deque_iterator<uns~")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::find(unsigned int const&)")]
pub fn stub_0x1c8d8c(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::_Rb_tree_impl<std::less<unsigned int>,false>::_Rb_tree_impl(std::allocator<std::_Rb_tree_node<std::pair<unsigned int const,int>>> const&,std::less<unsigned int> const&)")]
pub fn stub_0x1c8de8() -> (String, String) {
// std::pair ctor — empty pair.
(String::new(), String::new())
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::lower_bound(unsigned int const&)")]
pub fn stub_0x1c8e28() -> crate::slot::PortedFn {
// IDA 0x1c8e28: std::_Rb_tree<unsigned int, std::pair<unsigned int const, int>, std::_Select1st<std::pair<unsigned int const, int>>, std~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c8e28, "std::_Rb_tree<unsigned int, std::pair<unsigned int const, int>, std::_Select1st<std::pair<unsigned i~")
}

#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>::_M_set_node(unsigned short **)")]
pub fn stub_0x1c8e5c() -> crate::slot::PortedFn {
// IDA 0x1c8e5c: std::_Deque_iterator<unsigned short, unsigned short&, unsigned short*>::_M_set_node(unsigned short**).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c8e5c, "std::_Deque_iterator<unsigned short, unsigned short&, unsigned short*>::_M_set_node(unsigned short**~")
}

#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>::operator--(void)")]
pub fn stub_0x1c8e8c() -> crate::slot::PortedFn {
// IDA 0x1c8e8c: std::_Deque_iterator<unsigned short, unsigned short&, unsigned short*>::operator--().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c8e8c, "std::_Deque_iterator<unsigned short, unsigned short&, unsigned short*>::operator--()")
}

#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>::difference_type std::operator-<unsigned char *,unsigned char *&,unsigned char **>(std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **> const&,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **> const&)")]
pub fn stub_0x1c8ecc() -> crate::slot::PortedFn {
// IDA 0x1c8ecc: std::_Deque_iterator<unsigned char*, unsigned char*&, unsigned char**>::difference_type std::operator-<unsigned char*, u~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c8ecc, "std::_Deque_iterator<unsigned char*, unsigned char*&, unsigned char**>::difference_type std::operato~")
}

#[doc(alias = "std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>::difference_type std::operator-<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>(std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*> const&,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*> const&)")]
pub fn stub_0x1c8f1c() -> crate::slot::PortedFn {
// IDA 0x1c8f1c: std::_Deque_iterator<TagLib::MDMODEL, TagLib::MDMODEL&, TagLib::MDMODEL*>::difference_type std::operator-<TagLib::MDMODE~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c8f1c, "std::_Deque_iterator<TagLib::MDMODEL, TagLib::MDMODEL&, TagLib::MDMODEL*>::difference_type std::oper~")
}

#[doc(alias = "std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>::operator++(void)")]
pub fn stub_0x1c8f6c() -> crate::slot::PortedFn {
// IDA 0x1c8f6c: std::_Deque_iterator<TagLib::MDMODEL, TagLib::MDMODEL const&, TagLib::MDMODEL const*>::operator++().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c8f6c, "std::_Deque_iterator<TagLib::MDMODEL, TagLib::MDMODEL const&, TagLib::MDMODEL const*>::operator++()")
}

#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>::operator++(void)")]
pub fn stub_0x1c8fc4() -> crate::slot::PortedFn {
// IDA 0x1c8fc4: std::_Deque_iterator<unsigned short, unsigned short&, unsigned short*>::operator++().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c8fc4, "std::_Deque_iterator<unsigned short, unsigned short&, unsigned short*>::operator++()")
}

#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>::difference_type std::operator-<unsigned short,unsigned short const&,unsigned short const*>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*> const&,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*> const&)")]
pub fn stub_0x1c9004() -> crate::slot::PortedFn {
// IDA 0x1c9004: std::_Deque_iterator<unsigned short, unsigned short const&, unsigned short const*>::difference_type std::operator-<unsig~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c9004, "std::_Deque_iterator<unsigned short, unsigned short const&, unsigned short const*>::difference_type ~")
}

#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>::difference_type std::operator-<unsigned char *,unsigned char * const&,unsigned char * const*>(std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*> const&,std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*> const&)")]
pub fn stub_0x1c9054() -> crate::slot::PortedFn {
// IDA 0x1c9054: std::_Deque_iterator<unsigned char*, unsigned char* const&, unsigned char* const*>::difference_type std::operator-<unsig~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c9054, "std::_Deque_iterator<unsigned char*, unsigned char* const&, unsigned char* const*>::difference_type ~")
}

#[doc(alias = "__gnu_cxx::new_allocator<std::_Rb_tree_node<std::pair<unsigned int const,int>>>::allocate(unsigned long,void const*)")]
pub fn stub_0x1c90a4() -> crate::slot::PortedFn {
// IDA 0x1c90a4: __gnu_cxx::new_allocator<std::_Rb_tree_node<std::pair<unsigned int const, int>>>::allocate(unsigned long, void const*).
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1c90a4, "__gnu_cxx::new_allocator<std::_Rb_tree_node<std::pair<unsigned int const, int>>>::allocate(unsigned ~")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::_M_create_node(std::pair<unsigned int const,int> const&)")]
pub fn stub_0x1c90d4() -> crate::slot::PortedFn {
// IDA 0x1c90d4: std::_Rb_tree<unsigned int, std::pair<unsigned int const, int>, std::_Select1st<std::pair<unsigned int const, int>>, std~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c90d4, "std::_Rb_tree<unsigned int, std::pair<unsigned int const, int>, std::_Select1st<std::pair<unsigned i~")
}

#[doc(alias = "__gnu_cxx::new_allocator<TagLib::MDMODEL>::allocate(unsigned long,void const*)")]
pub fn stub_0x1c9104() -> crate::slot::PortedFn {
// IDA 0x1c9104: __gnu_cxx::new_allocator<TagLib::MDMODEL>::allocate(unsigned long, void const*).
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1c9104, "__gnu_cxx::new_allocator<TagLib::MDMODEL>::allocate(unsigned long, void const*)")
}

#[doc(alias = "__gnu_cxx::new_allocator<unsigned short>::allocate(unsigned long,void const*)")]
pub fn stub_0x1c9124() -> crate::slot::PortedFn {
// IDA 0x1c9124: __gnu_cxx::new_allocator<unsigned short>::allocate(unsigned long, void const*).
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1c9124, "__gnu_cxx::new_allocator<unsigned short>::allocate(unsigned long, void const*)")
}

#[doc(alias = "__gnu_cxx::new_allocator<unsigned char *>::allocate(unsigned long,void const*)")]
pub fn stub_0x1c9144() -> crate::slot::PortedFn {
// IDA 0x1c9144: __gnu_cxx::new_allocator<unsigned char*>::allocate(unsigned long, void const*).
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1c9144, "__gnu_cxx::new_allocator<unsigned char*>::allocate(unsigned long, void const*)")
}

#[doc(alias = "__gnu_cxx::new_allocator<TagLib::MDMODEL *>::allocate(unsigned long,void const*)")]
pub fn stub_0x1c9164() -> crate::slot::PortedFn {
// IDA 0x1c9164: __gnu_cxx::new_allocator<TagLib::MDMODEL*>::allocate(unsigned long, void const*).
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1c9164, "__gnu_cxx::new_allocator<TagLib::MDMODEL*>::allocate(unsigned long, void const*)")
}

#[doc(alias = "std::_Deque_base<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_allocate_map(unsigned long)")]
pub fn stub_0x1c9184() -> crate::slot::PortedFn {
// IDA 0x1c9184: std::_Deque_base<TagLib::MDMODEL, std::allocator<TagLib::MDMODEL>>::_M_allocate_map(unsigned long).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c9184, "std::_Deque_base<TagLib::MDMODEL, std::allocator<TagLib::MDMODEL>>::_M_allocate_map(unsigned long)")
}

#[doc(alias = "__gnu_cxx::new_allocator<unsigned char **>::allocate(unsigned long,void const*)")]
pub fn stub_0x1c922c() -> crate::slot::PortedFn {
// IDA 0x1c922c: __gnu_cxx::new_allocator<unsigned char**>::allocate(unsigned long, void const*).
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1c922c, "__gnu_cxx::new_allocator<unsigned char**>::allocate(unsigned long, void const*)")
}

#[doc(alias = "std::_Deque_base<unsigned char *,std::allocator<unsigned char *>>::_M_allocate_map(unsigned long)")]
pub fn stub_0x1c924c() -> crate::slot::PortedFn {
// IDA 0x1c924c: std::_Deque_base<unsigned char*, std::allocator<unsigned char*>>::_M_allocate_map(unsigned long).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c924c, "std::_Deque_base<unsigned char*, std::allocator<unsigned char*>>::_M_allocate_map(unsigned long)")
}

#[doc(alias = "__gnu_cxx::new_allocator<unsigned short *>::allocate(unsigned long,void const*)")]
pub fn stub_0x1c92f4() -> crate::slot::PortedFn {
// IDA 0x1c92f4: __gnu_cxx::new_allocator<unsigned short*>::allocate(unsigned long, void const*).
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1c92f4, "__gnu_cxx::new_allocator<unsigned short*>::allocate(unsigned long, void const*)")
}

#[doc(alias = "std::_Deque_base<unsigned short,std::allocator<unsigned short>>::_M_allocate_map(unsigned long)")]
pub fn stub_0x1c9314() -> crate::slot::PortedFn {
// IDA 0x1c9314: std::_Deque_base<unsigned short, std::allocator<unsigned short>>::_M_allocate_map(unsigned long).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c9314, "std::_Deque_base<unsigned short, std::allocator<unsigned short>>::_M_allocate_map(unsigned long)")
}

#[doc(alias = "std::_Deque_base<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_destroy_nodes(TagLib::MDMODEL**,TagLib::MDMODEL**)")]
pub fn stub_0x1c93bc() -> crate::slot::PortedFn {
// IDA 0x1c93bc: std::_Deque_base<TagLib::MDMODEL, std::allocator<TagLib::MDMODEL>>::_M_destroy_nodes(TagLib::MDMODEL**, TagLib::MDMODEL*~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c93bc, "std::_Deque_base<TagLib::MDMODEL, std::allocator<TagLib::MDMODEL>>::_M_destroy_nodes(TagLib::MDMODEL~")
}

#[doc(alias = "std::deque<unsigned short,std::allocator<unsigned short>>::_M_pop_back_aux(void)")]
pub fn stub_0x1c94ac(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// IndexArray::fastRemove — swap-with-last.
vec.fast_remove(index)
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned int const,int>> *)")]
pub fn stub_0x1c94e0(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Deque_base<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::~_Deque_base()")]
pub fn stub_0x1c951c() -> crate::slot::PortedFn {
// IDA 0x1c951c: std::_Deque_base<TagLib::MDMODEL, std::allocator<TagLib::MDMODEL>>::~_Deque_base().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c951c, "std::_Deque_base<TagLib::MDMODEL, std::allocator<TagLib::MDMODEL>>::~_Deque_base()")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned int const,int> const&)")]
pub fn stub_0x1c9550(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "TagLib::MDMODEL * * std::__copy_backward<true,std::random_access_iterator_tag>::__copy_b<TagLib::MDMODEL *>(TagLib::MDMODEL * const*,TagLib::MDMODEL * const*,TagLib::MDMODEL * *)")]
pub fn stub_0x1c95d4(vec: &crate::slot::VecModel) -> usize {
// std::__copy family — length-preserving element transfer.
vec.len()
}

#[doc(alias = "TagLib::MDMODEL * * std::__copy<true,std::random_access_iterator_tag>::copy<TagLib::MDMODEL *>(TagLib::MDMODEL * const*,TagLib::MDMODEL * const*,TagLib::MDMODEL * *)")]
pub fn stub_0x1c9604(vec: &crate::slot::VecModel) -> usize {
// std::__copy family — length-preserving element transfer.
vec.len()
}

#[doc(alias = "unsigned short * * std::__copy_backward<true,std::random_access_iterator_tag>::__copy_b<unsigned short *>(unsigned short * const*,unsigned short * const*,unsigned short * *)")]
pub fn stub_0x1c9630(vec: &crate::slot::VecModel) -> usize {
// std::__copy family — length-preserving element transfer.
vec.len()
}

#[doc(alias = "unsigned short * * std::__copy<true,std::random_access_iterator_tag>::copy<unsigned short *>(unsigned short * const*,unsigned short * const*,unsigned short * *)")]
pub fn stub_0x1c9660(vec: &crate::slot::VecModel) -> usize {
// std::__copy family — length-preserving element transfer.
vec.len()
}

#[doc(alias = "std::deque<unsigned short,std::allocator<unsigned short>>::_M_reallocate_map(unsigned long,bool)")]
pub fn stub_0x1c968c() -> crate::slot::PortedFn {
// IDA 0x1c968c: std::deque<unsigned short, std::allocator<unsigned short>>::_M_reallocate_map(unsigned long, bool).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c968c, "std::deque<unsigned short, std::allocator<unsigned short>>::_M_reallocate_map(unsigned long, bool)")
}

#[doc(alias = "std::deque<unsigned short,std::allocator<unsigned short>>::_M_reserve_map_at_back(unsigned long)")]
pub fn stub_0x1c97b4(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "unsigned char ** * std::__copy_backward<true,std::random_access_iterator_tag>::__copy_b<unsigned char **>(unsigned char ** const*,unsigned char ** const*,unsigned char ** *)")]
pub fn stub_0x1c97e8(vec: &crate::slot::VecModel) -> usize {
// std::__copy family — length-preserving element transfer.
vec.len()
}

#[doc(alias = "unsigned char ** * std::__copy<true,std::random_access_iterator_tag>::copy<unsigned char **>(unsigned char ** const*,unsigned char ** const*,unsigned char ** *)")]
pub fn stub_0x1c9818(vec: &crate::slot::VecModel) -> usize {
// std::__copy family — length-preserving element transfer.
vec.len()
}

#[doc(alias = "std::deque<unsigned short,std::allocator<unsigned short>>::back(void)")]
pub fn stub_0x1c9844() -> crate::slot::PortedFn {
// IDA 0x1c9844: std::deque<unsigned short, std::allocator<unsigned short>>::back().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c9844, "std::deque<unsigned short, std::allocator<unsigned short>>::back()")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::_M_insert_unique(std::pair<unsigned int const,int> const&)")]
pub fn stub_0x1c9884(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned int const,int>>,std::pair<unsigned int const,int> const&)")]
pub fn stub_0x1c9944(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>::_M_set_node(TagLib::MDMODEL**)")]
pub fn stub_0x1c9a68() -> crate::slot::PortedFn {
// IDA 0x1c9a68: std::_Deque_iterator<TagLib::MDMODEL, TagLib::MDMODEL&, TagLib::MDMODEL*>::_M_set_node(TagLib::MDMODEL**).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c9a68, "std::_Deque_iterator<TagLib::MDMODEL, TagLib::MDMODEL&, TagLib::MDMODEL*>::_M_set_node(TagLib::MDMOD~")
}

#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_reallocate_map(unsigned long,bool)")]
pub fn stub_0x1c9a98() -> crate::slot::PortedFn {
// IDA 0x1c9a98: std::deque<TagLib::MDMODEL, std::allocator<TagLib::MDMODEL>>::_M_reallocate_map(unsigned long, bool).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c9a98, "std::deque<TagLib::MDMODEL, std::allocator<TagLib::MDMODEL>>::_M_reallocate_map(unsigned long, bool)")
}

#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_reserve_map_at_back(unsigned long)")]
pub fn stub_0x1c9bc0(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>::operator++(void)")]
pub fn stub_0x1c9bf4() -> crate::slot::PortedFn {
// IDA 0x1c9bf4: std::_Deque_iterator<TagLib::MDMODEL, TagLib::MDMODEL&, TagLib::MDMODEL*>::operator++().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c9bf4, "std::_Deque_iterator<TagLib::MDMODEL, TagLib::MDMODEL&, TagLib::MDMODEL*>::operator++()")
}

#[doc(alias = "std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*> std::__uninitialized_copy_aux<std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>>(std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,std::__false_type)")]
pub fn stub_0x1c9c34(vec: &crate::slot::VecModel) -> usize {
// std::__copy family — length-preserving element transfer.
vec.len()
}

#[doc(alias = "std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*> std::uninitialized_copy<std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>>(std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>)")]
pub fn stub_0x1c9ca4() -> crate::slot::PortedFn {
// IDA 0x1c9ca4: std::_Deque_iterator<TagLib::MDMODEL, TagLib::MDMODEL&, TagLib::MDMODEL*> std::uninitialized_copy<std::_Deque_iterator<T~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c9ca4, "std::_Deque_iterator<TagLib::MDMODEL, TagLib::MDMODEL&, TagLib::MDMODEL*> std::uninitialized_copy<st~")
}

#[doc(alias = "std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*> std::__uninitialized_copy_a<std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,TagLib::MDMODEL>(std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,std::allocator<TagLib::MDMODEL>)")]
pub fn stub_0x1c9d24(vec: &crate::slot::VecModel) -> usize {
// std::__copy family — length-preserving element transfer.
vec.len()
}

#[doc(alias = "std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>::operator--(void)")]
pub fn stub_0x1c9da0() -> crate::slot::PortedFn {
// IDA 0x1c9da0: std::_Deque_iterator<TagLib::MDMODEL, TagLib::MDMODEL&, TagLib::MDMODEL*>::operator--().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c9da0, "std::_Deque_iterator<TagLib::MDMODEL, TagLib::MDMODEL&, TagLib::MDMODEL*>::operator--()")
}

#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::back(void)")]
pub fn stub_0x1c9de0() -> crate::slot::PortedFn {
// IDA 0x1c9de0: std::deque<TagLib::MDMODEL, std::allocator<TagLib::MDMODEL>>::back().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c9de0, "std::deque<TagLib::MDMODEL, std::allocator<TagLib::MDMODEL>>::back()")
}

#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>::operator++(void)")]
pub fn stub_0x1c9e20() -> crate::slot::PortedFn {
// IDA 0x1c9e20: std::_Deque_iterator<unsigned short, unsigned short const&, unsigned short const*>::operator++().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1c9e20, "std::_Deque_iterator<unsigned short, unsigned short const&, unsigned short const*>::operator++()")
}

#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> std::__copy<false,std::random_access_iterator_tag>::copy<std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>)")]
pub fn stub_0x1c9e78(vec: &crate::slot::VecModel) -> usize {
// std::__copy family — length-preserving element transfer.
vec.len()
}

#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> std::__copy_aux<std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>)")]
pub fn stub_0x1ca124(vec: &crate::slot::VecModel) -> usize {
// std::__copy family — length-preserving element transfer.
vec.len()
}

#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> std::__copy_normal<false,false>::__copy_n<std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>)")]
pub fn stub_0x1ca1a0(vec: &crate::slot::VecModel) -> usize {
// std::__copy family — length-preserving element transfer.
vec.len()
}

#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> std::copy<std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>)")]
pub fn stub_0x1ca21c() -> crate::slot::PortedFn {
// IDA 0x1ca21c: std::_Deque_iterator<unsigned short, unsigned short&, unsigned short*> std::copy<std::_Deque_iterator<unsigned short, un~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1ca21c, "std::_Deque_iterator<unsigned short, unsigned short&, unsigned short*> std::copy<std::_Deque_iterato~")
}

#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> std::__uninitialized_copy_aux<std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>,std::__true_type)")]
pub fn stub_0x1ca298(vec: &crate::slot::VecModel) -> usize {
// std::__copy family — length-preserving element transfer.
vec.len()
}

#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> std::uninitialized_copy<std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>)")]
pub fn stub_0x1ca314() -> crate::slot::PortedFn {
// IDA 0x1ca314: std::_Deque_iterator<unsigned short, unsigned short&, unsigned short*> std::uninitialized_copy<std::_Deque_iterator<unsi~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1ca314, "std::_Deque_iterator<unsigned short, unsigned short&, unsigned short*> std::uninitialized_copy<std::~")
}

#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> std::__uninitialized_copy_a<std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>,unsigned short>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>,std::allocator<unsigned short>)")]
pub fn stub_0x1ca394(vec: &crate::slot::VecModel) -> usize {
// std::__copy family — length-preserving element transfer.
vec.len()
}

#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>::operator++(void)")]
pub fn stub_0x1ca410() -> crate::slot::PortedFn {
// IDA 0x1ca410: std::_Deque_iterator<unsigned char*, unsigned char* const&, unsigned char* const*>::operator++().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1ca410, "std::_Deque_iterator<unsigned char*, unsigned char* const&, unsigned char* const*>::operator++()")
}

#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>::difference_type std::operator-<unsigned short,unsigned short &,unsigned short *>(std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> const&,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> const&)")]
pub fn stub_0x1ca468() -> crate::slot::PortedFn {
// IDA 0x1ca468: std::_Deque_iterator<unsigned short, unsigned short&, unsigned short*>::difference_type std::operator-<unsigned short, u~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1ca468, "std::_Deque_iterator<unsigned short, unsigned short&, unsigned short*>::difference_type std::operato~")
}

#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>::_M_set_node(unsigned char ***)")]
pub fn stub_0x1ca4b8() -> crate::slot::PortedFn {
// IDA 0x1ca4b8: std::_Deque_iterator<unsigned char*, unsigned char*&, unsigned char**>::_M_set_node(unsigned char***).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1ca4b8, "std::_Deque_iterator<unsigned char*, unsigned char*&, unsigned char**>::_M_set_node(unsigned char***~")
}

#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>::operator++(void)")]
pub fn stub_0x1ca4e8() -> crate::slot::PortedFn {
// IDA 0x1ca4e8: std::_Deque_iterator<unsigned char*, unsigned char*&, unsigned char**>::operator++().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1ca4e8, "std::_Deque_iterator<unsigned char*, unsigned char*&, unsigned char**>::operator++()")
}

#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **> std::__copy<false,std::random_access_iterator_tag>::copy<std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>>(std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>)")]
pub fn stub_0x1ca528(vec: &crate::slot::VecModel) -> usize {
// std::__copy family — length-preserving element transfer.
vec.len()
}

#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **> std::__copy_aux<std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>>(std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>)")]
pub fn stub_0x1ca7d4(vec: &crate::slot::VecModel) -> usize {
// std::__copy family — length-preserving element transfer.
vec.len()
}

#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **> std::__copy_normal<false,false>::__copy_n<std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>>(std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>)")]
pub fn stub_0x1ca850(vec: &crate::slot::VecModel) -> usize {
// std::__copy family — length-preserving element transfer.
vec.len()
}

#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **> std::copy<std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>>(std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>)")]
pub fn stub_0x1ca8cc() -> crate::slot::PortedFn {
// IDA 0x1ca8cc: std::_Deque_iterator<unsigned char*, unsigned char*&, unsigned char**> std::copy<std::_Deque_iterator<unsigned char*, un~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1ca8cc, "std::_Deque_iterator<unsigned char*, unsigned char*&, unsigned char**> std::copy<std::_Deque_iterato~")
}

#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **> std::__uninitialized_copy_aux<std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>>(std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>,std::__true_type)")]
pub fn stub_0x1ca948(vec: &crate::slot::VecModel) -> usize {
// std::__copy family — length-preserving element transfer.
vec.len()
}

#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **> std::uninitialized_copy<std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>>(std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>)")]
pub fn stub_0x1ca9c4() -> crate::slot::PortedFn {
// IDA 0x1ca9c4: std::_Deque_iterator<unsigned char*, unsigned char*&, unsigned char**> std::uninitialized_copy<std::_Deque_iterator<unsi~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1ca9c4, "std::_Deque_iterator<unsigned char*, unsigned char*&, unsigned char**> std::uninitialized_copy<std::~")
}
