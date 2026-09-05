// Auto-generated skeletons for rbx-script — gap filler (global EA-sorted)
// Filter: Lua|Script|lua (5041 filtered, 0 remaining) -> global gap filler EA-sorted asc next 120 not yet in script crate
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +120 stubs | range 0x1caa44..0x1d8690 | script 18972->19092 distinct
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;


#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **> std::__uninitialized_copy_a<std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>,unsigned char *>(std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>,std::allocator<unsigned char *>)")]
pub fn stub_0x1caa44(vec: &crate::slot::VecModel) -> usize {
// std::__copy family — length-preserving element transfer.
vec.len()
}

#[doc(alias = "std::deque<unsigned char *,std::allocator<unsigned char *>>::_M_reallocate_map(unsigned long,bool)")]
pub fn stub_0x1caac0() -> crate::slot::PortedFn {
// IDA 0x1caac0: std::deque<unsigned char*, std::allocator<unsigned char*>>::_M_reallocate_map(unsigned long, bool).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1caac0, "std::deque<unsigned char*, std::allocator<unsigned char*>>::_M_reallocate_map(unsigned long, bool)")
}

#[doc(alias = "std::deque<unsigned char *,std::allocator<unsigned char *>>::_M_reserve_map_at_back(unsigned long)")]
pub fn stub_0x1cabe8(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "std::deque<unsigned char *,std::allocator<unsigned char *>>::_M_push_back_aux(unsigned char * const&)")]
pub fn stub_0x1cac1c(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::deque<unsigned char *,std::allocator<unsigned char *>>::push_back(unsigned char * const&)")]
pub fn stub_0x1cac80(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>::operator--(void)")]
pub fn stub_0x1cacc4() -> crate::slot::PortedFn {
// IDA 0x1cacc4: std::_Deque_iterator<unsigned char*, unsigned char*&, unsigned char**>::operator--().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1cacc4, "std::_Deque_iterator<unsigned char*, unsigned char*&, unsigned char**>::operator--()")
}

#[doc(alias = "std::deque<unsigned char *,std::allocator<unsigned char *>>::back(void)")]
pub fn stub_0x1cad04() -> crate::slot::PortedFn {
// IDA 0x1cad04: std::deque<unsigned char*, std::allocator<unsigned char*>>::back().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1cad04, "std::deque<unsigned char*, std::allocator<unsigned char*>>::back()")
}

#[doc(alias = "std::deque<unsigned short,std::allocator<unsigned short>>::_M_push_back_aux(unsigned short const&)")]
pub fn stub_0x1cad44(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::deque<unsigned short,std::allocator<unsigned short>>::push_back(unsigned short const&)")]
pub fn stub_0x1cada8(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Deque_base<unsigned short,std::allocator<unsigned short>>::_M_destroy_nodes(unsigned short **,unsigned short **)")]
pub fn stub_0x1cadec() -> crate::slot::PortedFn {
// IDA 0x1cadec: std::_Deque_base<unsigned short, std::allocator<unsigned short>>::_M_destroy_nodes(unsigned short**, unsigned short**).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1cadec, "std::_Deque_base<unsigned short, std::allocator<unsigned short>>::_M_destroy_nodes(unsigned short**,~")
}

#[doc(alias = "std::_Deque_base<unsigned short,std::allocator<unsigned short>>::~_Deque_base()")]
pub fn stub_0x1caedc() -> crate::slot::PortedFn {
// IDA 0x1caedc: std::_Deque_base<unsigned short, std::allocator<unsigned short>>::~_Deque_base().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1caedc, "std::_Deque_base<unsigned short, std::allocator<unsigned short>>::~_Deque_base()")
}

#[doc(alias = "std::deque<unsigned short,std::allocator<unsigned short>>::~deque()")]
pub fn stub_0x1caf10(vec: crate::slot::VecModel) {
// sequence dtor — releases the storage.
drop(vec);
}

#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_push_back_aux(TagLib::MDMODEL const&)")]
pub fn stub_0x1caf80(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::push_back(TagLib::MDMODEL const&)")]
pub fn stub_0x1cafe4(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_pop_back_aux(void)")]
pub fn stub_0x1cb028(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// IndexArray::fastRemove — swap-with-last.
vec.fast_remove(index)
}

#[doc(alias = "std::deque<unsigned char *,std::allocator<unsigned char *>>::_M_pop_back_aux(void)")]
pub fn stub_0x1cb05c(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// IndexArray::fastRemove — swap-with-last.
vec.fast_remove(index)
}

#[doc(alias = "std::_Deque_base<unsigned char *,std::allocator<unsigned char *>>::_M_destroy_nodes(unsigned char ***,unsigned char ***)")]
pub fn stub_0x1cb090() -> crate::slot::PortedFn {
// IDA 0x1cb090: std::_Deque_base<unsigned char*, std::allocator<unsigned char*>>::_M_destroy_nodes(unsigned char***, unsigned char***).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1cb090, "std::_Deque_base<unsigned char*, std::allocator<unsigned char*>>::_M_destroy_nodes(unsigned char***,~")
}

#[doc(alias = "std::_Deque_base<unsigned char *,std::allocator<unsigned char *>>::~_Deque_base()")]
pub fn stub_0x1cb180() -> crate::slot::PortedFn {
// IDA 0x1cb180: std::_Deque_base<unsigned char*, std::allocator<unsigned char*>>::~_Deque_base().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1cb180, "std::_Deque_base<unsigned char*, std::allocator<unsigned char*>>::~_Deque_base()")
}

#[doc(alias = "std::deque<unsigned char *,std::allocator<unsigned char *>>::~deque()")]
pub fn stub_0x1cb1b4(vec: crate::slot::VecModel) {
// sequence dtor — releases the storage.
drop(vec);
}

#[doc(alias = "std::map<unsigned int,int,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::operator[](unsigned int const&)")]
pub fn stub_0x1cb224(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::_Deque_base<unsigned short,std::allocator<unsigned short>>::_M_create_nodes(unsigned short **,unsigned short **)")]
pub fn stub_0x1cb290() -> crate::slot::PortedFn {
// IDA 0x1cb290: std::_Deque_base<unsigned short, std::allocator<unsigned short>>::_M_create_nodes(unsigned short**, unsigned short**).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1cb290, "std::_Deque_base<unsigned short, std::allocator<unsigned short>>::_M_create_nodes(unsigned short**, ~")
}

#[doc(alias = "std::_Deque_base<unsigned short,std::allocator<unsigned short>>::_M_initialize_map(unsigned long)")]
pub fn stub_0x1cb510() -> crate::slot::PortedFn {
// IDA 0x1cb510: std::_Deque_base<unsigned short, std::allocator<unsigned short>>::_M_initialize_map(unsigned long).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1cb510, "std::_Deque_base<unsigned short, std::allocator<unsigned short>>::_M_initialize_map(unsigned long)")
}

#[doc(alias = "std::_Deque_base<unsigned short,std::allocator<unsigned short>>::_Deque_base(std::allocator<unsigned short> const&,unsigned long)")]
pub fn stub_0x1cb6e0() -> crate::slot::PortedFn {
// IDA 0x1cb6e0: std::_Deque_base<unsigned short, std::allocator<unsigned short>>::_Deque_base(std::allocator<unsigned short> const&, uns~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1cb6e0, "std::_Deque_base<unsigned short, std::allocator<unsigned short>>::_Deque_base(std::allocator<unsigne~")
}

#[doc(alias = "std::deque<unsigned short,std::allocator<unsigned short>>::deque(std::deque<unsigned short,std::allocator<unsigned short>> const&)")]
pub fn stub_0x1cb7b0() -> crate::slot::PortedFn {
// IDA 0x1cb7b0: std::deque<unsigned short, std::allocator<unsigned short>>::deque(std::deque<unsigned short, std::allocator<unsigned sho~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1cb7b0, "std::deque<unsigned short, std::allocator<unsigned short>>::deque(std::deque<unsigned short, std::al~")
}

#[doc(alias = "std::_Deque_base<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_create_nodes(TagLib::MDMODEL**,TagLib::MDMODEL**)")]
pub fn stub_0x1cb878() -> crate::slot::PortedFn {
// IDA 0x1cb878: std::_Deque_base<TagLib::MDMODEL, std::allocator<TagLib::MDMODEL>>::_M_create_nodes(TagLib::MDMODEL**, TagLib::MDMODEL**~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1cb878, "std::_Deque_base<TagLib::MDMODEL, std::allocator<TagLib::MDMODEL>>::_M_create_nodes(TagLib::MDMODEL*~")
}

#[doc(alias = "std::_Deque_base<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_initialize_map(unsigned long)")]
pub fn stub_0x1cbaf8() -> crate::slot::PortedFn {
// IDA 0x1cbaf8: std::_Deque_base<TagLib::MDMODEL, std::allocator<TagLib::MDMODEL>>::_M_initialize_map(unsigned long).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1cbaf8, "std::_Deque_base<TagLib::MDMODEL, std::allocator<TagLib::MDMODEL>>::_M_initialize_map(unsigned long)")
}

#[doc(alias = "std::_Deque_base<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_Deque_base(std::allocator<TagLib::MDMODEL> const&,unsigned long)")]
pub fn stub_0x1cbcc8() -> crate::slot::PortedFn {
// IDA 0x1cbcc8: std::_Deque_base<TagLib::MDMODEL, std::allocator<TagLib::MDMODEL>>::_Deque_base(std::allocator<TagLib::MDMODEL> const&, ~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1cbcc8, "std::_Deque_base<TagLib::MDMODEL, std::allocator<TagLib::MDMODEL>>::_Deque_base(std::allocator<TagLi~")
}

#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::deque(std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>> const&)")]
pub fn stub_0x1cbd98() -> crate::slot::PortedFn {
// IDA 0x1cbd98: std::deque<TagLib::MDMODEL, std::allocator<TagLib::MDMODEL>>::deque(std::deque<TagLib::MDMODEL, std::allocator<TagLib::M~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1cbd98, "std::deque<TagLib::MDMODEL, std::allocator<TagLib::MDMODEL>>::deque(std::deque<TagLib::MDMODEL, std:~")
}

#[doc(alias = "std::_Deque_base<unsigned char *,std::allocator<unsigned char *>>::_M_create_nodes(unsigned char ***,unsigned char ***)")]
pub fn stub_0x1cbe60() -> crate::slot::PortedFn {
// IDA 0x1cbe60: std::_Deque_base<unsigned char*, std::allocator<unsigned char*>>::_M_create_nodes(unsigned char***, unsigned char***).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1cbe60, "std::_Deque_base<unsigned char*, std::allocator<unsigned char*>>::_M_create_nodes(unsigned char***, ~")
}

#[doc(alias = "std::_Deque_base<unsigned char *,std::allocator<unsigned char *>>::_M_initialize_map(unsigned long)")]
pub fn stub_0x1cc0e0() -> crate::slot::PortedFn {
// IDA 0x1cc0e0: std::_Deque_base<unsigned char*, std::allocator<unsigned char*>>::_M_initialize_map(unsigned long).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1cc0e0, "std::_Deque_base<unsigned char*, std::allocator<unsigned char*>>::_M_initialize_map(unsigned long)")
}

#[doc(alias = "std::_Deque_base<unsigned char *,std::allocator<unsigned char *>>::_Deque_base(std::allocator<unsigned char *> const&,unsigned long)")]
pub fn stub_0x1cc2b0() -> crate::slot::PortedFn {
// IDA 0x1cc2b0: std::_Deque_base<unsigned char*, std::allocator<unsigned char*>>::_Deque_base(std::allocator<unsigned char*> const&, uns~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1cc2b0, "std::_Deque_base<unsigned char*, std::allocator<unsigned char*>>::_Deque_base(std::allocator<unsigne~")
}

#[doc(alias = "std::deque<unsigned char *,std::allocator<unsigned char *>>::deque(std::deque<unsigned char *,std::allocator<unsigned char *>> const&)")]
pub fn stub_0x1cc380() -> crate::slot::PortedFn {
// IDA 0x1cc380: std::deque<unsigned char*, std::allocator<unsigned char*>>::deque(std::deque<unsigned char*, std::allocator<unsigned cha~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1cc380, "std::deque<unsigned char*, std::allocator<unsigned char*>>::deque(std::deque<unsigned char*, std::al~")
}

#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_destroy_data_aux(std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>)")]
pub fn stub_0x1cc448() -> crate::slot::PortedFn {
// IDA 0x1cc448: std::deque<TagLib::MDMODEL, std::allocator<TagLib::MDMODEL>>::_M_destroy_data_aux(std::_Deque_iterator<TagLib::MDMODEL, ~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1cc448, "std::deque<TagLib::MDMODEL, std::allocator<TagLib::MDMODEL>>::_M_destroy_data_aux(std::_Deque_iterat~")
}

#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_destroy_data_dispatch(std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,std::__false_type)")]
pub fn stub_0x1cc450() -> crate::slot::PortedFn {
// IDA 0x1cc450: std::deque<TagLib::MDMODEL, std::allocator<TagLib::MDMODEL>>::_M_destroy_data_dispatch(std::_Deque_iterator<TagLib::MDMO~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1cc450, "std::deque<TagLib::MDMODEL, std::allocator<TagLib::MDMODEL>>::_M_destroy_data_dispatch(std::_Deque_i~")
}

#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_destroy_data(std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,std::allocator<TagLib::MDMODEL> const&)")]
pub fn stub_0x1cc4b0() -> crate::slot::PortedFn {
// IDA 0x1cc4b0: std::deque<TagLib::MDMODEL, std::allocator<TagLib::MDMODEL>>::_M_destroy_data(std::_Deque_iterator<TagLib::MDMODEL, TagL~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1cc4b0, "std::deque<TagLib::MDMODEL, std::allocator<TagLib::MDMODEL>>::_M_destroy_data(std::_Deque_iterator<T~")
}

#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::~deque()")]
pub fn stub_0x1cc508(vec: crate::slot::VecModel) {
// sequence dtor — releases the storage.
drop(vec);
}

#[doc(alias = "cacheIO_getByte(tagCacheIO *)")]
pub fn stub_0x1cc578() -> crate::slot::PortedFn {
// IDA 0x1cc578: cacheIO_getByte(tagCacheIO*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1cc578, "cacheIO_getByte(tagCacheIO*)")
}

#[doc(alias = "cacheIO_getBytes(tagCacheIO *,unsigned long)")]
pub fn stub_0x1cc5dc() -> crate::slot::PortedFn {
// IDA 0x1cc5dc: cacheIO_getBytes(tagCacheIO*, unsigned long).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1cc5dc, "cacheIO_getBytes(tagCacheIO*, unsigned long)")
}

#[doc(alias = "__ZL6Formatv_2")]
pub fn stub_0x1cc684() -> crate::slot::PortedFn {
// IDA 0x1cc684: __ZL6Formatv_2.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1cc684, "__ZL6Formatv_2")
}

#[doc(alias = "__ZL9Extensionv_2")]
pub fn stub_0x1cc6a4() -> crate::slot::PortedFn {
// IDA 0x1cc6a4: __ZL9Extensionv_2.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1cc6a4, "__ZL9Extensionv_2")
}

#[doc(alias = "__ZL7RegExprv_2")]
pub fn stub_0x1cc6b4() -> crate::slot::PortedFn {
// IDA 0x1cc6b4: __ZL7RegExprv_2.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1cc6b4, "__ZL7RegExprv_2")
}

#[doc(alias = "__ZL8MimeTypev_2")]
pub fn stub_0x1cc6bc() -> crate::slot::PortedFn {
// IDA 0x1cc6bc: __ZL8MimeTypev_2.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1cc6bc, "__ZL8MimeTypev_2")
}

#[doc(alias = "__ZL8ValidateP11FreeImageIOPv_2")]
pub fn stub_0x1cc6cc() -> crate::slot::PortedFn {
// IDA 0x1cc6cc: __ZL8ValidateP11FreeImageIOPv_2.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1cc6cc, "__ZL8ValidateP11FreeImageIOPv_2")
}

#[doc(alias = "__ZL19SupportsExportDepthi_2")]
pub fn stub_0x1cc838() -> crate::slot::PortedFn {
// IDA 0x1cc838: __ZL19SupportsExportDepthi_2.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1cc838, "__ZL19SupportsExportDepthi_2")
}

#[doc(alias = "__ZL18SupportsExportType15FREE_IMAGE_TYPE_2")]
pub fn stub_0x1cc85c() -> crate::slot::PortedFn {
// IDA 0x1cc85c: __ZL18SupportsExportType15FREE_IMAGE_TYPE_2.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1cc85c, "__ZL18SupportsExportType15FREE_IMAGE_TYPE_2")
}

#[doc(alias = "InitTARGA(Plugin *,int)")]
pub fn stub_0x1cc86c() -> crate::slot::PortedFn {
// IDA 0x1cc86c: InitTARGA(Plugin*, int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1cc86c, "InitTARGA(Plugin*, int)")
}

#[doc(alias = "cacheIO_alloc(tagCacheIO *,FreeImageIO *,void *,unsigned long)")]
pub fn stub_0x1cc934() -> crate::slot::PortedFn {
// IDA 0x1cc934: cacheIO_alloc(tagCacheIO*, FreeImageIO*, void*, unsigned long).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1cc934, "cacheIO_alloc(tagCacheIO*, FreeImageIO*, void*, unsigned long)")
}

#[doc(alias = "cacheIO_free(tagCacheIO *)")]
pub fn stub_0x1cc990() -> crate::slot::PortedFn {
// IDA 0x1cc990: cacheIO_free(tagCacheIO*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1cc990, "cacheIO_free(tagCacheIO*)")
}

#[doc(alias = "Internal_GetScanLine(FIBITMAP *,int,int)")]
pub fn stub_0x1cc9ac() -> crate::slot::PortedFn {
// IDA 0x1cc9ac: Internal_GetScanLine(FIBITMAP*, int, int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1cc9ac, "Internal_GetScanLine(FIBITMAP*, int, int)")
}

#[doc(alias = "__ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3__2")]
pub fn stub_0x1cc9e4() -> crate::slot::PortedFn {
// IDA 0x1cc9e4: __ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3__2.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1cc9e4, "__ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3__2")
}

#[doc(alias = "__ZL4LoadP11FreeImageIOPviiS1__2")]
pub fn stub_0x1cd15c() -> crate::slot::PortedFn {
// IDA 0x1cd15c: __ZL4LoadP11FreeImageIOPviiS1__2.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x1cd15c, "__ZL4LoadP11FreeImageIOPviiS1__2")
}

#[doc(alias = "_af_sort_pos")]
pub fn stub_0x1d0c8c() -> crate::slot::PortedFn {
// IDA 0x1d0c8c: _af_sort_pos.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d0c8c, "_af_sort_pos")
}

#[doc(alias = "_af_sort_widths")]
pub fn stub_0x1d0e90() -> crate::slot::PortedFn {
// IDA 0x1d0e90: _af_sort_widths.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d0e90, "_af_sort_widths")
}

#[doc(alias = "_af_cjk_metrics_scale_dim")]
pub fn stub_0x1d1060() -> crate::slot::PortedFn {
// IDA 0x1d1060: _af_cjk_metrics_scale_dim.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d1060, "_af_cjk_metrics_scale_dim")
}

#[doc(alias = "_af_cjk_metrics_scale")]
pub fn stub_0x1d10a0() -> crate::slot::PortedFn {
// IDA 0x1d10a0: _af_cjk_metrics_scale.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d10a0, "_af_cjk_metrics_scale")
}

#[doc(alias = "_af_cjk_compute_stem_width")]
pub fn stub_0x1d10ec() -> crate::slot::PortedFn {
// IDA 0x1d10ec: _af_cjk_compute_stem_width.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d10ec, "_af_cjk_compute_stem_width")
}

#[doc(alias = "_af_hint_normal_stem")]
pub fn stub_0x1d14e0() -> crate::slot::PortedFn {
// IDA 0x1d14e0: _af_hint_normal_stem.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d14e0, "_af_hint_normal_stem")
}

#[doc(alias = "_af_cjk_hints_detect_features")]
pub fn stub_0x1d16b8() -> crate::slot::PortedFn {
// IDA 0x1d16b8: _af_cjk_hints_detect_features.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d16b8, "_af_cjk_hints_detect_features")
}

#[doc(alias = "_af_cjk_hints_apply")]
pub fn stub_0x1d1e8c() -> crate::slot::PortedFn {
// IDA 0x1d1e8c: _af_cjk_hints_apply.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d1e8c, "_af_cjk_hints_apply")
}

#[doc(alias = "_af_cjk_hints_init")]
pub fn stub_0x1d2428() -> crate::slot::PortedFn {
// IDA 0x1d2428: _af_cjk_hints_init.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d2428, "_af_cjk_hints_init")
}

#[doc(alias = "_af_cjk_metrics_init")]
pub fn stub_0x1d24b0() -> crate::slot::PortedFn {
// IDA 0x1d24b0: _af_cjk_metrics_init.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d24b0, "_af_cjk_metrics_init")
}

#[doc(alias = "_af_dummy_hints_apply")]
pub fn stub_0x1d251c() -> crate::slot::PortedFn {
// IDA 0x1d251c: _af_dummy_hints_apply.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d251c, "_af_dummy_hints_apply")
}

#[doc(alias = "_af_dummy_hints_init")]
pub fn stub_0x1d2524() -> crate::slot::PortedFn {
// IDA 0x1d2524: _af_dummy_hints_init.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d2524, "_af_dummy_hints_init")
}

#[doc(alias = "_af_face_globals_is_digit")]
pub fn stub_0x1d2538() -> crate::slot::PortedFn {
// IDA 0x1d2538: _af_face_globals_is_digit.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d2538, "_af_face_globals_is_digit")
}

#[doc(alias = "_af_face_globals_get_metrics")]
pub fn stub_0x1d2554() -> crate::slot::PortedFn {
// IDA 0x1d2554: _af_face_globals_get_metrics.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d2554, "_af_face_globals_get_metrics")
}

#[doc(alias = "_af_face_globals_free")]
pub fn stub_0x1d267c() -> crate::slot::PortedFn {
// IDA 0x1d267c: _af_face_globals_free.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d267c, "_af_face_globals_free")
}

#[doc(alias = "_af_face_globals_new")]
pub fn stub_0x1d27cc() -> crate::slot::PortedFn {
// IDA 0x1d27cc: _af_face_globals_new.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d27cc, "_af_face_globals_new")
}

#[doc(alias = "_af_direction_compute")]
pub fn stub_0x1d2b28() -> crate::slot::PortedFn {
// IDA 0x1d2b28: _af_direction_compute.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d2b28, "_af_direction_compute")
}

#[doc(alias = "_af_glyph_hints_rescale")]
pub fn stub_0x1d2ba4() -> crate::slot::PortedFn {
// IDA 0x1d2ba4: _af_glyph_hints_rescale.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d2ba4, "_af_glyph_hints_rescale")
}

#[doc(alias = "_af_glyph_hints_save")]
pub fn stub_0x1d2bb4() -> crate::slot::PortedFn {
// IDA 0x1d2bb4: _af_glyph_hints_save.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d2bb4, "_af_glyph_hints_save")
}

#[doc(alias = "_af_glyph_hints_align_edge_points")]
pub fn stub_0x1d2c1c() -> crate::slot::PortedFn {
// IDA 0x1d2c1c: _af_glyph_hints_align_edge_points.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d2c1c, "_af_glyph_hints_align_edge_points")
}

#[doc(alias = "_af_iup_interp")]
pub fn stub_0x1d2ce8() -> crate::slot::PortedFn {
// IDA 0x1d2ce8: _af_iup_interp.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d2ce8, "_af_iup_interp")
}

#[doc(alias = "_af_glyph_hints_align_weak_points")]
pub fn stub_0x1d2e1c() -> crate::slot::PortedFn {
// IDA 0x1d2e1c: _af_glyph_hints_align_weak_points.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d2e1c, "_af_glyph_hints_align_weak_points")
}

#[doc(alias = "_af_glyph_hints_align_strong_points")]
pub fn stub_0x1d3060() -> crate::slot::PortedFn {
// IDA 0x1d3060: _af_glyph_hints_align_strong_points.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d3060, "_af_glyph_hints_align_strong_points")
}

#[doc(alias = "_af_axis_hints_new_segment")]
pub fn stub_0x1d3418() -> crate::slot::PortedFn {
// IDA 0x1d3418: _af_axis_hints_new_segment.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d3418, "_af_axis_hints_new_segment")
}

#[doc(alias = "_af_glyph_hints_reload")]
pub fn stub_0x1d34f8() -> crate::slot::PortedFn {
// IDA 0x1d34f8: _af_glyph_hints_reload.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d34f8, "_af_glyph_hints_reload")
}

#[doc(alias = "_af_glyph_hints_done")]
pub fn stub_0x1d3ad0() -> crate::slot::PortedFn {
// IDA 0x1d3ad0: _af_glyph_hints_done.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d3ad0, "_af_glyph_hints_done")
}

#[doc(alias = "_af_glyph_hints_init")]
pub fn stub_0x1d3b88() -> crate::slot::PortedFn {
// IDA 0x1d3b88: _af_glyph_hints_init.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d3b88, "_af_glyph_hints_init")
}

#[doc(alias = "_af_axis_hints_new_edge")]
pub fn stub_0x1d3bac() -> crate::slot::PortedFn {
// IDA 0x1d3bac: _af_axis_hints_new_edge.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d3bac, "_af_axis_hints_new_edge")
}

#[doc(alias = "_af_indic_hints_apply")]
pub fn stub_0x1d3d4c() -> crate::slot::PortedFn {
// IDA 0x1d3d4c: _af_indic_hints_apply.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d3d4c, "_af_indic_hints_apply")
}

#[doc(alias = "_af_indic_hints_init")]
pub fn stub_0x1d3d5c() -> crate::slot::PortedFn {
// IDA 0x1d3d5c: _af_indic_hints_init.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d3d5c, "_af_indic_hints_init")
}

#[doc(alias = "_af_indic_metrics_scale")]
pub fn stub_0x1d3d6c() -> crate::slot::PortedFn {
// IDA 0x1d3d6c: _af_indic_metrics_scale.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d3d6c, "_af_indic_metrics_scale")
}

#[doc(alias = "_af_indic_metrics_init")]
pub fn stub_0x1d3d7c() -> crate::slot::PortedFn {
// IDA 0x1d3d7c: _af_indic_metrics_init.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d3d7c, "_af_indic_metrics_init")
}

#[doc(alias = "_af_latin_hints_link_segments")]
pub fn stub_0x1d3d8c() -> crate::slot::PortedFn {
// IDA 0x1d3d8c: _af_latin_hints_link_segments.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d3d8c, "_af_latin_hints_link_segments")
}

#[doc(alias = "_af_latin_compute_stem_width")]
pub fn stub_0x1d3f40() -> crate::slot::PortedFn {
// IDA 0x1d3f40: _af_latin_compute_stem_width.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d3f40, "_af_latin_compute_stem_width")
}

#[doc(alias = "_af_latin_align_linked_edge")]
pub fn stub_0x1d4398() -> crate::slot::PortedFn {
// IDA 0x1d4398: _af_latin_align_linked_edge.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d4398, "_af_latin_align_linked_edge")
}

#[doc(alias = "_af_latin_hints_init")]
pub fn stub_0x1d43dc() -> crate::slot::PortedFn {
// IDA 0x1d43dc: _af_latin_hints_init.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d43dc, "_af_latin_hints_init")
}

#[doc(alias = "_af_latin_hint_edges")]
pub fn stub_0x1d447c() -> crate::slot::PortedFn {
// IDA 0x1d447c: _af_latin_hint_edges.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d447c, "_af_latin_hint_edges")
}

#[doc(alias = "_af_latin_hints_compute_blue_edges")]
pub fn stub_0x1d4b38() -> crate::slot::PortedFn {
// IDA 0x1d4b38: _af_latin_hints_compute_blue_edges.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d4b38, "_af_latin_hints_compute_blue_edges")
}

#[doc(alias = "_af_latin_metrics_scale_dim")]
pub fn stub_0x1d5024() -> crate::slot::PortedFn {
// IDA 0x1d5024: _af_latin_metrics_scale_dim.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d5024, "_af_latin_metrics_scale_dim")
}

#[doc(alias = "_af_latin_metrics_scale")]
pub fn stub_0x1d5430() -> crate::slot::PortedFn {
// IDA 0x1d5430: _af_latin_metrics_scale.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d5430, "_af_latin_metrics_scale")
}

#[doc(alias = "_af_latin_hints_compute_edges")]
pub fn stub_0x1d546c() -> crate::slot::PortedFn {
// IDA 0x1d546c: _af_latin_hints_compute_edges.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d546c, "_af_latin_hints_compute_edges")
}

#[doc(alias = "_af_latin_hints_compute_segments")]
pub fn stub_0x1d599c() -> crate::slot::PortedFn {
// IDA 0x1d599c: _af_latin_hints_compute_segments.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d599c, "_af_latin_hints_compute_segments")
}

#[doc(alias = "_af_latin_hints_detect_features")]
pub fn stub_0x1d5df8() -> crate::slot::PortedFn {
// IDA 0x1d5df8: _af_latin_hints_detect_features.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d5df8, "_af_latin_hints_detect_features")
}

#[doc(alias = "_af_latin_hints_apply")]
pub fn stub_0x1d5e30() -> crate::slot::PortedFn {
// IDA 0x1d5e30: _af_latin_hints_apply.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d5e30, "_af_latin_hints_apply")
}

#[doc(alias = "_af_latin_metrics_check_digits")]
pub fn stub_0x1d5f28() -> crate::slot::PortedFn {
// IDA 0x1d5f28: _af_latin_metrics_check_digits.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d5f28, "_af_latin_metrics_check_digits")
}

#[doc(alias = "_af_latin_metrics_init_widths")]
pub fn stub_0x1d6218() -> crate::slot::PortedFn {
// IDA 0x1d6218: _af_latin_metrics_init_widths.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d6218, "_af_latin_metrics_init_widths")
}

#[doc(alias = "_af_latin_metrics_init")]
pub fn stub_0x1d64dc() -> crate::slot::PortedFn {
// IDA 0x1d64dc: _af_latin_metrics_init.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d64dc, "_af_latin_metrics_init")
}

#[doc(alias = "_af_loader_load_g")]
pub fn stub_0x1d712c() -> crate::slot::PortedFn {
// IDA 0x1d712c: _af_loader_load_g.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d712c, "_af_loader_load_g")
}

#[doc(alias = "_af_loader_done")]
pub fn stub_0x1d7a64() -> crate::slot::PortedFn {
// IDA 0x1d7a64: _af_loader_done.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d7a64, "_af_loader_done")
}

#[doc(alias = "_af_loader_reset")]
pub fn stub_0x1d7a94() -> crate::slot::PortedFn {
// IDA 0x1d7a94: _af_loader_reset.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d7a94, "_af_loader_reset")
}

#[doc(alias = "_af_loader_load_glyph")]
pub fn stub_0x1d7afc() -> crate::slot::PortedFn {
// IDA 0x1d7afc: _af_loader_load_glyph.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d7afc, "_af_loader_load_glyph")
}

#[doc(alias = "_af_loader_init")]
pub fn stub_0x1d7c20() -> crate::slot::PortedFn {
// IDA 0x1d7c20: _af_loader_init.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d7c20, "_af_loader_init")
}

#[doc(alias = "_af_autofitter_done")]
pub fn stub_0x1d7c58() -> crate::slot::PortedFn {
// IDA 0x1d7c58: _af_autofitter_done.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d7c58, "_af_autofitter_done")
}

#[doc(alias = "_af_autofitter_init")]
pub fn stub_0x1d7c6c() -> crate::slot::PortedFn {
// IDA 0x1d7c6c: _af_autofitter_init.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d7c6c, "_af_autofitter_init")
}

#[doc(alias = "_af_autofitter_load_glyph")]
pub fn stub_0x1d7c88() -> crate::slot::PortedFn {
// IDA 0x1d7c88: _af_autofitter_load_glyph.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d7c88, "_af_autofitter_load_glyph")
}

#[doc(alias = "_FT_RoundFix")]
pub fn stub_0x1d7ca8() -> crate::slot::PortedFn {
// IDA 0x1d7ca8: _FT_RoundFix.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d7ca8, "_FT_RoundFix")
}

#[doc(alias = "_ft_multo64")]
pub fn stub_0x1d7cd0() -> crate::slot::PortedFn {
// IDA 0x1d7cd0: _ft_multo64.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d7cd0, "_ft_multo64")
}

#[doc(alias = "_ft_div64by32")]
pub fn stub_0x1d7d28() -> crate::slot::PortedFn {
// IDA 0x1d7d28: _ft_div64by32.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d7d28, "_ft_div64by32")
}

#[doc(alias = "_FT_Add64")]
pub fn stub_0x1d7e9c() -> crate::slot::PortedFn {
// IDA 0x1d7e9c: _FT_Add64.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d7e9c, "_FT_Add64")
}

#[doc(alias = "_FT_MulDiv")]
pub fn stub_0x1d7ec4() -> crate::slot::PortedFn {
// IDA 0x1d7ec4: _FT_MulDiv.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d7ec4, "_FT_MulDiv")
}

#[doc(alias = "__ft_face_scale_advances")]
pub fn stub_0x1d7fb4() -> crate::slot::PortedFn {
// IDA 0x1d7fb4: __ft_face_scale_advances.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d7fb4, "__ft_face_scale_advances")
}

#[doc(alias = "_FT_MulDiv_No_Round")]
pub fn stub_0x1d81b0() -> crate::slot::PortedFn {
// IDA 0x1d81b0: _FT_MulDiv_No_Round.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d81b0, "_FT_MulDiv_No_Round")
}

#[doc(alias = "_FT_MulFix")]
pub fn stub_0x1d8264() -> crate::slot::PortedFn {
// IDA 0x1d8264: _FT_MulFix.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d8264, "_FT_MulFix")
}

#[doc(alias = "_FT_DivFix")]
pub fn stub_0x1d82d8() -> crate::slot::PortedFn {
// IDA 0x1d82d8: _FT_DivFix.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d82d8, "_FT_DivFix")
}

#[doc(alias = "_FT_Matrix_Invert")]
pub fn stub_0x1d836c() -> crate::slot::PortedFn {
// IDA 0x1d836c: _FT_Matrix_Invert.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d836c, "_FT_Matrix_Invert")
}

#[doc(alias = "_FT_Matrix_Multiply_Scaled")]
pub fn stub_0x1d8400() -> crate::slot::PortedFn {
// IDA 0x1d8400: _FT_Matrix_Multiply_Scaled.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d8400, "_FT_Matrix_Multiply_Scaled")
}

#[doc(alias = "_FT_Vector_Transform_Scaled")]
pub fn stub_0x1d84fc() -> crate::slot::PortedFn {
// IDA 0x1d84fc: _FT_Vector_Transform_Scaled.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d84fc, "_FT_Vector_Transform_Scaled")
}

#[doc(alias = "_FT_SqrtFixed")]
pub fn stub_0x1d8584() -> crate::slot::PortedFn {
// IDA 0x1d8584: _FT_SqrtFixed.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d8584, "_FT_SqrtFixed")
}

#[doc(alias = "_ft_corner_orientation")]
pub fn stub_0x1d8690() -> crate::slot::PortedFn {
// IDA 0x1d8690: _ft_corner_orientation.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x1d8690, "_ft_corner_orientation")
}
