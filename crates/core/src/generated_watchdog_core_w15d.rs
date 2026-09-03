//! core — generated_watchdog_core_w15d — 120 stubs, core namespace (SharedPtr/atomic/thread/TaskScheduler/signals).
//! Source: ida/export.json (85545 funcs), 120 lowest EA core candidates absent from /tmp/global_eas.txt
//! Range 0xd4c6bc..0xe59ab8.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xd4c6bc — __ZN4Ogre4Pass26setVertexProgramParametersENS_9SharedPtrINS_20GpuProgramParametersEEE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "__ZN4Ogre4Pass26setVertexProgramParametersENS_9SharedPtrINS_20GpuProgramParametersEEE")]
#[doc(alias = "Ogre::Pass::setVertexProgramParameters(Ogre::SharedPtr<Ogre::GpuProgramParameters>)")]
pub fn stub_0xd4c6bc() {
    // IDA 0xd4c6bc: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xd4cb00 — __ZN4Ogre4Pass28setFragmentProgramParametersENS_9SharedPtrINS_20GpuProgramParametersEEE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "__ZN4Ogre4Pass28setFragmentProgramParametersENS_9SharedPtrINS_20GpuProgramParametersEEE")]
#[doc(alias = "Ogre::Pass::setFragmentProgramParameters(Ogre::SharedPtr<Ogre::GpuProgramParameters>)")]
pub fn stub_0xd4cb00() {
    // IDA 0xd4cb00: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xd5889c — __ZN4Ogre9SharedPtrINS_5Codec9CodecDataEEaSERKS3_
// type: int __fastcall(int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int, int)
#[doc(alias = "__ZN4Ogre9SharedPtrINS_5Codec9CodecDataEEaSERKS3_")]
#[doc(alias = "Ogre::SharedPtr<Ogre::Codec::CodecData>::operator=(Ogre::SharedPtr<Ogre::Codec::CodecData> const&)")]
pub fn stub_0xd5889c() {
    // IDA 0xd5889c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xd58a1c — __ZN4Ogre9SharedPtrINS_5Codec9CodecDataEED1Ev
#[doc(alias = "__ZN4Ogre9SharedPtrINS_5Codec9CodecDataEED1Ev")]
#[doc(alias = "Ogre::SharedPtr<Ogre::Codec::CodecData>::~SharedPtr()")]
pub fn stub_0xd58a1c() {
    // IDA 0xd58a1c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd58db8 — __ZN4Ogre9SharedPtrINS_5Codec9CodecDataEED0Ev
#[doc(alias = "__ZN4Ogre9SharedPtrINS_5Codec9CodecDataEED0Ev")]
#[doc(alias = "Ogre::SharedPtr<Ogre::Codec::CodecData>::~SharedPtr()")]
pub fn stub_0xd58db8() {
    // IDA 0xd58db8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd58eac — __ZN4Ogre9SharedPtrINS_5Codec9CodecDataEE7destroyEv
#[doc(alias = "__ZN4Ogre9SharedPtrINS_5Codec9CodecDataEE7destroyEv")]
#[doc(alias = "Ogre::SharedPtr<Ogre::Codec::CodecData>::destroy(void)")]
pub fn stub_0xd58eac() {
    // IDA 0xd58eac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd58ee4 — __ZN4Ogre9SharedPtrINS_5Codec9CodecDataEE4swapERS3_
#[doc(alias = "__ZN4Ogre9SharedPtrINS_5Codec9CodecDataEE4swapERS3_")]
#[doc(alias = "Ogre::SharedPtr<Ogre::Codec::CodecData>::swap(Ogre::SharedPtr<Ogre::Codec::CodecData>&)")]
pub fn stub_0xd58ee4() {
    // IDA 0xd58ee4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd81040 — __ZN4Ogre23ResourceBackgroundQueue16ResourceResponseC1ENS_9SharedPtrINS_8ResourceEEERKNS0_15ResourceRequestE
// type: int __fastcall(char, int, int, int, int, int)
#[doc(alias = "__ZN4Ogre23ResourceBackgroundQueue16ResourceResponseC1ENS_9SharedPtrINS_8ResourceEEERKNS0_15ResourceRequestE")]
#[doc(alias = "Ogre::ResourceBackgroundQueue::ResourceResponse::ResourceResponse(Ogre::SharedPtr<Ogre::Resource>,Ogre::ResourceBackgroundQueue::ResourceRequest const&)")]
pub fn stub_0xd81040() {
    // IDA 0xd81040: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd87e30 — __ZN4Ogre20ResourceGroupManager22_notifyResourceCreatedERNS_9SharedPtrINS_8ResourceEEE
#[doc(alias = "__ZN4Ogre20ResourceGroupManager22_notifyResourceCreatedERNS_9SharedPtrINS_8ResourceEEE")]
#[doc(alias = "Ogre::ResourceGroupManager::_notifyResourceCreated(Ogre::SharedPtr<Ogre::Resource> &)")]
pub fn stub_0xd87e30() {
    // IDA 0xd87e30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd87ebc — __ZN4Ogre20ResourceGroupManager18addCreatedResourceERNS_9SharedPtrINS_8ResourceEEERNS0_13ResourceGroupE
#[doc(alias = "__ZN4Ogre20ResourceGroupManager18addCreatedResourceERNS_9SharedPtrINS_8ResourceEEERNS0_13ResourceGroupE")]
#[doc(alias = "Ogre::ResourceGroupManager::addCreatedResource(Ogre::SharedPtr<Ogre::Resource> &,Ogre::ResourceGroupManager::ResourceGroup &)")]
pub fn stub_0xd87ebc() {
    // IDA 0xd87ebc: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

// 0xd87ffc — __ZN4Ogre20ResourceGroupManager22_notifyResourceRemovedERNS_9SharedPtrINS_8ResourceEEE
#[doc(alias = "__ZN4Ogre20ResourceGroupManager22_notifyResourceRemovedERNS_9SharedPtrINS_8ResourceEEE")]
#[doc(alias = "Ogre::ResourceGroupManager::_notifyResourceRemoved(Ogre::SharedPtr<Ogre::Resource> &)")]
pub fn stub_0xd87ffc() {
    // IDA 0xd87ffc: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

// 0xd89274 — __ZN4Ogre9SharedPtrISt4listINS0_ISt6vectorINS_8FileInfoENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEEENS4_ISA_S7_EEEED1Ev
#[doc(alias = "__ZN4Ogre9SharedPtrISt4listINS0_ISt6vectorINS_8FileInfoENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEEENS4_ISA_S7_EEEED1Ev")]
#[doc(alias = "Ogre::SharedPtr<std::list<Ogre::SharedPtr<std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::STLAllocator<Ogre::SharedPtr<std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::~SharedPtr()")]
pub fn stub_0xd89274() {
    // IDA 0xd89274: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd89324 — __ZN4Ogre9SharedPtrINS_8ResourceEEaSERKS2_
// type: int __fastcall(int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int, int)
#[doc(alias = "__ZN4Ogre9SharedPtrINS_8ResourceEEaSERKS2_")]
#[doc(alias = "Ogre::SharedPtr<Ogre::Resource>::operator=(Ogre::SharedPtr<Ogre::Resource> const&)")]
pub fn stub_0xd89324() {
    // IDA 0xd89324: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd89c20 — __ZNSt8_Rb_treeIfSt4pairIKfPSt4listIN4Ogre9SharedPtrINS3_8ResourceEEENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ESt4lessIfENS7_ISE_SA_EEE8_M_eraseEPSt13_Rb_tree_nodeISE_E
#[doc(alias = "__ZNSt8_Rb_treeIfSt4pairIKfPSt4listIN4Ogre9SharedPtrINS3_8ResourceEEENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ESt4lessIfENS7_ISE_SA_EEE8_M_eraseEPSt13_Rb_tree_nodeISE_E")]
#[doc(alias = "std::_Rb_tree<float,std::pair<float const,std::list<Ogre::SharedPtr<Ogre::Resource>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::Resource>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<float const,std::list<Ogre::SharedPtr<Ogre::Resource>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::Resource>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<float>,Ogre::STLAllocator<std::pair<float const,std::list<Ogre::SharedPtr<Ogre::Resource>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::Resource>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<float const,std::list<Ogre::SharedPtr<Ogre::Resource>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::Resource>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>> *)")]
pub fn stub_0xd89c20() {
    // IDA 0xd89c20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd89c48 — __ZNSt8_Rb_treeIfSt4pairIKfPSt4listIN4Ogre9SharedPtrINS3_8ResourceEEENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ESt4lessIfENS7_ISE_SA_EEE16_M_insert_uniqueESt17_Rb_tree_iteratorISE_ERKSE_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, float *)
#[doc(alias = "__ZNSt8_Rb_treeIfSt4pairIKfPSt4listIN4Ogre9SharedPtrINS3_8ResourceEEENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ESt4lessIfENS7_ISE_SA_EEE16_M_insert_uniqueESt17_Rb_tree_iteratorISE_ERKSE_")]
#[doc(alias = "std::_Rb_tree<float,std::pair<float const,std::list<Ogre::SharedPtr<Ogre::Resource>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::Resource>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<float const,std::list<Ogre::SharedPtr<Ogre::Resource>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::Resource>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<float>,Ogre::STLAllocator<std::pair<float const,std::list<Ogre::SharedPtr<Ogre::Resource>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::Resource>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<float const,std::list<Ogre::SharedPtr<Ogre::Resource>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::Resource>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::pair<float const,std::list<Ogre::SharedPtr<Ogre::Resource>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::Resource>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *> const&)")]
pub fn stub_0xd89c48() {
    // IDA 0xd89c48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd89e50 — __ZNSt8_Rb_treeIfSt4pairIKfPSt4listIN4Ogre9SharedPtrINS3_8ResourceEEENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ESt4lessIfENS7_ISE_SA_EEE16_M_insert_uniqueERKSE_
// type: int __fastcall(char *)
#[doc(alias = "__ZNSt8_Rb_treeIfSt4pairIKfPSt4listIN4Ogre9SharedPtrINS3_8ResourceEEENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ESt4lessIfENS7_ISE_SA_EEE16_M_insert_uniqueERKSE_")]
#[doc(alias = "std::_Rb_tree<float,std::pair<float const,std::list<Ogre::SharedPtr<Ogre::Resource>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::Resource>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<float const,std::list<Ogre::SharedPtr<Ogre::Resource>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::Resource>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<float>,Ogre::STLAllocator<std::pair<float const,std::list<Ogre::SharedPtr<Ogre::Resource>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::Resource>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<float const,std::list<Ogre::SharedPtr<Ogre::Resource>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::Resource>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *> const&)")]
pub fn stub_0xd89e50() {
    // IDA 0xd89e50: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xd89f70 — __ZNSt10_List_baseIN4Ogre9SharedPtrINS0_8ResourceEEENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD0Ev
#[doc(alias = "__ZNSt10_List_baseIN4Ogre9SharedPtrINS0_8ResourceEEENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD0Ev")]
#[doc(alias = "std::_List_base<Ogre::SharedPtr<Ogre::Resource>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::Resource>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
pub fn stub_0xd89f70() {
    // IDA 0xd89f70: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd89f7c — __ZN4Ogre9SharedPtrISt4listINS0_ISt6vectorINS_8FileInfoENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEEENS4_ISA_S7_EEEED0Ev
#[doc(alias = "__ZN4Ogre9SharedPtrISt4listINS0_ISt6vectorINS_8FileInfoENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEEENS4_ISA_S7_EEEED0Ev")]
#[doc(alias = "Ogre::SharedPtr<std::list<Ogre::SharedPtr<std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::STLAllocator<Ogre::SharedPtr<std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::~SharedPtr()")]
pub fn stub_0xd89f7c() {
    // IDA 0xd89f7c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd8a030 — __ZN4Ogre9SharedPtrISt4listINS0_ISt6vectorINS_8FileInfoENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEEENS4_ISA_S7_EEEE7destroyEv
#[doc(alias = "__ZN4Ogre9SharedPtrISt4listINS0_ISt6vectorINS_8FileInfoENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEEENS4_ISA_S7_EEEE7destroyEv")]
#[doc(alias = "Ogre::SharedPtr<std::list<Ogre::SharedPtr<std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::STLAllocator<Ogre::SharedPtr<std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::destroy(void)")]
pub fn stub_0xd8a030() {
    // IDA 0xd8a030: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd8a150 — __ZN4Ogre9SharedPtrISt4listINS0_ISt6vectorINS_8FileInfoENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEEENS4_ISA_S7_EEEE4swapERSD_
#[doc(alias = "__ZN4Ogre9SharedPtrISt4listINS0_ISt6vectorINS_8FileInfoENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEEENS4_ISA_S7_EEEE4swapERSD_")]
#[doc(alias = "Ogre::SharedPtr<std::list<Ogre::SharedPtr<std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::STLAllocator<Ogre::SharedPtr<std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::swap(Ogre::SharedPtr<std::list<Ogre::SharedPtr<std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::STLAllocator<Ogre::SharedPtr<std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>&)")]
pub fn stub_0xd8a150() {
    // IDA 0xd8a150: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd8a16c — __ZNSt10_List_baseIN4Ogre9SharedPtrISt6vectorINS0_8FileInfoENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEEENS4_ISA_S7_EEE10_List_implD1Ev
#[doc(alias = "__ZNSt10_List_baseIN4Ogre9SharedPtrISt6vectorINS0_8FileInfoENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEEENS4_ISA_S7_EEE10_List_implD1Ev")]
#[doc(alias = "std::_List_base<Ogre::SharedPtr<std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::STLAllocator<Ogre::SharedPtr<std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
pub fn stub_0xd8a16c() {
    // IDA 0xd8a16c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd8a170 — __ZNSt10_List_baseIN4Ogre9SharedPtrISt6vectorINS0_8FileInfoENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEEENS4_ISA_S7_EEE10_List_implD0Ev
#[doc(alias = "__ZNSt10_List_baseIN4Ogre9SharedPtrISt6vectorINS0_8FileInfoENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEEENS4_ISA_S7_EEE10_List_implD0Ev")]
#[doc(alias = "std::_List_base<Ogre::SharedPtr<std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::STLAllocator<Ogre::SharedPtr<std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
pub fn stub_0xd8a170() {
    // IDA 0xd8a170: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd8ab40 — __ZNSt8_Rb_treeIfSt4pairIKfPSt4listIN4Ogre9SharedPtrINS3_8ResourceEEENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ESt4lessIfENS7_ISE_SA_EEE13_Rb_tree_implISI_Lb0EED1Ev
#[doc(alias = "__ZNSt8_Rb_treeIfSt4pairIKfPSt4listIN4Ogre9SharedPtrINS3_8ResourceEEENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ESt4lessIfENS7_ISE_SA_EEE13_Rb_tree_implISI_Lb0EED1Ev")]
#[doc(alias = "std::_Rb_tree<float,std::pair<float const,std::list<Ogre::SharedPtr<Ogre::Resource>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::Resource>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<float const,std::list<Ogre::SharedPtr<Ogre::Resource>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::Resource>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<float>,Ogre::STLAllocator<std::pair<float const,std::list<Ogre::SharedPtr<Ogre::Resource>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::Resource>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<float>,false>::~_Rb_tree_impl()")]
pub fn stub_0xd8ab40() {
    // IDA 0xd8ab40: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd8ab44 — __ZNSt8_Rb_treeIfSt4pairIKfPSt4listIN4Ogre9SharedPtrINS3_8ResourceEEENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ESt4lessIfENS7_ISE_SA_EEE13_Rb_tree_implISI_Lb0EED0Ev
#[doc(alias = "__ZNSt8_Rb_treeIfSt4pairIKfPSt4listIN4Ogre9SharedPtrINS3_8ResourceEEENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ESt4lessIfENS7_ISE_SA_EEE13_Rb_tree_implISI_Lb0EED0Ev")]
#[doc(alias = "std::_Rb_tree<float,std::pair<float const,std::list<Ogre::SharedPtr<Ogre::Resource>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::Resource>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<float const,std::list<Ogre::SharedPtr<Ogre::Resource>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::Resource>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<float>,Ogre::STLAllocator<std::pair<float const,std::list<Ogre::SharedPtr<Ogre::Resource>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::Resource>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<float>,false>::~_Rb_tree_impl()")]
pub fn stub_0xd8ab44() {
    // IDA 0xd8ab44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd8b94c — __ZN4Ogre15ResourceManager7addImplERNS_9SharedPtrINS_8ResourceEEE
#[doc(alias = "__ZN4Ogre15ResourceManager7addImplERNS_9SharedPtrINS_8ResourceEEE")]
#[doc(alias = "Ogre::ResourceManager::addImpl(Ogre::SharedPtr<Ogre::Resource> &)")]
pub fn stub_0xd8b94c() {
    // IDA 0xd8b94c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd8d0a0 — __ZN4Ogre15ResourceManager10removeImplERNS_9SharedPtrINS_8ResourceEEE
#[doc(alias = "__ZN4Ogre15ResourceManager10removeImplERNS_9SharedPtrINS_8ResourceEEE")]
#[doc(alias = "Ogre::ResourceManager::removeImpl(Ogre::SharedPtr<Ogre::Resource> &)")]
pub fn stub_0xd8d0a0() {
    // IDA 0xd8d0a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd8d6d8 — __ZN4Ogre15ResourceManager6removeERNS_9SharedPtrINS_8ResourceEEE
#[doc(alias = "__ZN4Ogre15ResourceManager6removeERNS_9SharedPtrINS_8ResourceEEE")]
#[doc(alias = "Ogre::ResourceManager::remove(Ogre::SharedPtr<Ogre::Resource> &)")]
pub fn stub_0xd8d6d8() {
    // IDA 0xd8d6d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd8df80 — __ZNSt10_List_baseIN4Ogre9SharedPtrINS0_8ResourceEEENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD1Ev
#[doc(alias = "__ZNSt10_List_baseIN4Ogre9SharedPtrINS0_8ResourceEEENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD1Ev")]
#[doc(alias = "std::_List_base<Ogre::SharedPtr<Ogre::Resource>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::Resource>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
pub fn stub_0xd8df80() {
    // IDA 0xd8df80: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd8e004 — __ZNSt8_Rb_treeIySt4pairIKyN4Ogre9SharedPtrINS2_8ResourceEEEESt10_Select1stIS6_ESt4lessIyENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
#[doc(alias = "__ZNSt8_Rb_treeIySt4pairIKyN4Ogre9SharedPtrINS2_8ResourceEEEESt10_Select1stIS6_ESt4lessIyENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E")]
#[doc(alias = "std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,Ogre::SharedPtr<Ogre::Resource>>,std::_Select1st<std::pair<unsigned long long const,Ogre::SharedPtr<Ogre::Resource>>>,std::less<unsigned long long>,Ogre::STLAllocator<std::pair<unsigned long long const,Ogre::SharedPtr<Ogre::Resource>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned long long const,Ogre::SharedPtr<Ogre::Resource>>> *)")]
pub fn stub_0xd8e004() {
    // IDA 0xd8e004: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd8e02c — __ZNSt8_Rb_treeIySt4pairIKyN4Ogre9SharedPtrINS2_8ResourceEEEESt10_Select1stIS6_ESt4lessIyENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS6_E
#[doc(alias = "__ZNSt8_Rb_treeIySt4pairIKyN4Ogre9SharedPtrINS2_8ResourceEEEESt10_Select1stIS6_ESt4lessIyENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS6_E")]
#[doc(alias = "std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,Ogre::SharedPtr<Ogre::Resource>>,std::_Select1st<std::pair<unsigned long long const,Ogre::SharedPtr<Ogre::Resource>>>,std::less<unsigned long long>,Ogre::STLAllocator<std::pair<unsigned long long const,Ogre::SharedPtr<Ogre::Resource>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<unsigned long long const,Ogre::SharedPtr<Ogre::Resource>>> *)")]
pub fn stub_0xd8e02c() {
    // IDA 0xd8e02c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd8e47c — __ZNSt8_Rb_treeIySt4pairIKyN4Ogre9SharedPtrINS2_8ResourceEEEESt10_Select1stIS6_ESt4lessIyENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_
// type: int __fastcall(int, int, __int64 *)
#[doc(alias = "__ZNSt8_Rb_treeIySt4pairIKyN4Ogre9SharedPtrINS2_8ResourceEEEESt10_Select1stIS6_ESt4lessIyENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_")]
#[doc(alias = "std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,Ogre::SharedPtr<Ogre::Resource>>,std::_Select1st<std::pair<unsigned long long const,Ogre::SharedPtr<Ogre::Resource>>>,std::less<unsigned long long>,Ogre::STLAllocator<std::pair<unsigned long long const,Ogre::SharedPtr<Ogre::Resource>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned long long const,Ogre::SharedPtr<Ogre::Resource>> const&)")]
pub fn stub_0xd8e47c() {
    // IDA 0xd8e47c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd8e544 — __ZNSt8_Rb_treeIySt4pairIKyN4Ogre9SharedPtrINS2_8ResourceEEEESt10_Select1stIS6_ESt4lessIyENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSI_RKS6_
#[doc(alias = "__ZNSt8_Rb_treeIySt4pairIKyN4Ogre9SharedPtrINS2_8ResourceEEEESt10_Select1stIS6_ESt4lessIyENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSI_RKS6_")]
#[doc(alias = "std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,Ogre::SharedPtr<Ogre::Resource>>,std::_Select1st<std::pair<unsigned long long const,Ogre::SharedPtr<Ogre::Resource>>>,std::less<unsigned long long>,Ogre::STLAllocator<std::pair<unsigned long long const,Ogre::SharedPtr<Ogre::Resource>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned long long const,Ogre::SharedPtr<Ogre::Resource>> const&)")]
pub fn stub_0xd8e544() {
    // IDA 0xd8e544: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd8f590 — __ZNSt8_Rb_treeIySt4pairIKyN4Ogre9SharedPtrINS2_8ResourceEEEESt10_Select1stIS6_ESt4lessIyENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED1Ev
#[doc(alias = "__ZNSt8_Rb_treeIySt4pairIKyN4Ogre9SharedPtrINS2_8ResourceEEEESt10_Select1stIS6_ESt4lessIyENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED1Ev")]
#[doc(alias = "std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,Ogre::SharedPtr<Ogre::Resource>>,std::_Select1st<std::pair<unsigned long long const,Ogre::SharedPtr<Ogre::Resource>>>,std::less<unsigned long long>,Ogre::STLAllocator<std::pair<unsigned long long const,Ogre::SharedPtr<Ogre::Resource>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned long long>,false>::~_Rb_tree_impl()")]
pub fn stub_0xd8f590() {
    // IDA 0xd8f590: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd8f594 — __ZNSt8_Rb_treeIySt4pairIKyN4Ogre9SharedPtrINS2_8ResourceEEEESt10_Select1stIS6_ESt4lessIyENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED0Ev
#[doc(alias = "__ZNSt8_Rb_treeIySt4pairIKyN4Ogre9SharedPtrINS2_8ResourceEEEESt10_Select1stIS6_ESt4lessIyENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED0Ev")]
#[doc(alias = "std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,Ogre::SharedPtr<Ogre::Resource>>,std::_Select1st<std::pair<unsigned long long const,Ogre::SharedPtr<Ogre::Resource>>>,std::less<unsigned long long>,Ogre::STLAllocator<std::pair<unsigned long long const,Ogre::SharedPtr<Ogre::Resource>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned long long>,false>::~_Rb_tree_impl()")]
pub fn stub_0xd8f594() {
    // IDA 0xd8f594: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd92a18 — __ZN4Ogre9SharedPtrINS_15ControllerValueIfEEEaSERKS3_
// type: int __fastcall(int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int, int)
#[doc(alias = "__ZN4Ogre9SharedPtrINS_15ControllerValueIfEEEaSERKS3_")]
#[doc(alias = "Ogre::SharedPtr<Ogre::ControllerValue<float>>::operator=(Ogre::SharedPtr<Ogre::ControllerValue<float>> const&)")]
pub fn stub_0xd92a18() {
    // IDA 0xd92a18: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xdb2a64 — __ZN4Ogre12SceneManager20setShadowCameraSetupERKNS_9SharedPtrINS_17ShadowCameraSetupEEE
#[doc(alias = "__ZN4Ogre12SceneManager20setShadowCameraSetupERKNS_9SharedPtrINS_17ShadowCameraSetupEEE")]
#[doc(alias = "Ogre::SceneManager::setShadowCameraSetup(Ogre::SharedPtr<Ogre::ShadowCameraSetup> const&)")]
pub fn stub_0xdb2a64() {
    // IDA 0xdb2a64: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xdb7e94 — __ZN4Ogre9SharedPtrINS_17ShadowCameraSetupEED1Ev
#[doc(alias = "__ZN4Ogre9SharedPtrINS_17ShadowCameraSetupEED1Ev")]
#[doc(alias = "Ogre::SharedPtr<Ogre::ShadowCameraSetup>::~SharedPtr()")]
pub fn stub_0xdb7e94() {
    // IDA 0xdb7e94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xdb8798 — __ZN4Ogre9SharedPtrINS_20GpuProgramParametersEEaSERKS2_
#[doc(alias = "__ZN4Ogre9SharedPtrINS_20GpuProgramParametersEEaSERKS2_")]
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuProgramParameters>::operator=(Ogre::SharedPtr<Ogre::GpuProgramParameters> const&)")]
pub fn stub_0xdb8798() {
    // IDA 0xdb8798: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xdb88a4 — __ZN4Ogre9SharedPtrINS_17ShadowCameraSetupEEaSERKS2_
// type: int __fastcall(int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int, int)
#[doc(alias = "__ZN4Ogre9SharedPtrINS_17ShadowCameraSetupEEaSERKS2_")]
#[doc(alias = "Ogre::SharedPtr<Ogre::ShadowCameraSetup>::operator=(Ogre::SharedPtr<Ogre::ShadowCameraSetup> const&)")]
pub fn stub_0xdb88a4() {
    // IDA 0xdb88a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xdbb90c — __ZN4Ogre9SharedPtrINS_17ShadowCameraSetupEED0Ev
#[doc(alias = "__ZN4Ogre9SharedPtrINS_17ShadowCameraSetupEED0Ev")]
#[doc(alias = "Ogre::SharedPtr<Ogre::ShadowCameraSetup>::~SharedPtr()")]
pub fn stub_0xdbb90c() {
    // IDA 0xdbb90c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xdbba00 — __ZN4Ogre9SharedPtrINS_17ShadowCameraSetupEE7destroyEv
#[doc(alias = "__ZN4Ogre9SharedPtrINS_17ShadowCameraSetupEE7destroyEv")]
#[doc(alias = "Ogre::SharedPtr<Ogre::ShadowCameraSetup>::destroy(void)")]
pub fn stub_0xdbba00() {
    // IDA 0xdbba00: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xdbba38 — __ZN4Ogre9SharedPtrINS_17ShadowCameraSetupEE4swapERS2_
#[doc(alias = "__ZN4Ogre9SharedPtrINS_17ShadowCameraSetupEE4swapERS2_")]
#[doc(alias = "Ogre::SharedPtr<Ogre::ShadowCameraSetup>::swap(Ogre::SharedPtr<Ogre::ShadowCameraSetup>&)")]
pub fn stub_0xdbba38() {
    // IDA 0xdbba38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xdbfc74 — __ZN4Ogre9SharedPtrINS_8MaterialEEaSERKS2_
// type: int __fastcall(int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int, int)
#[doc(alias = "__ZN4Ogre9SharedPtrINS_8MaterialEEaSERKS2_")]
#[doc(alias = "Ogre::SharedPtr<Ogre::Material>::operator=(Ogre::SharedPtr<Ogre::Material> const&)")]
pub fn stub_0xdbfc74() {
    // IDA 0xdbfc74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xdbfdf4 — __ZN4Ogre9SharedPtrINS_19HardwarePixelBufferEED0Ev
#[doc(alias = "__ZN4Ogre9SharedPtrINS_19HardwarePixelBufferEED0Ev")]
#[doc(alias = "Ogre::SharedPtr<Ogre::HardwarePixelBuffer>::~SharedPtr()")]
pub fn stub_0xdbfdf4() {
    // IDA 0xdbfdf4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xdbfee8 — __ZN4Ogre9SharedPtrINS_19HardwarePixelBufferEE7destroyEv
#[doc(alias = "__ZN4Ogre9SharedPtrINS_19HardwarePixelBufferEE7destroyEv")]
#[doc(alias = "Ogre::SharedPtr<Ogre::HardwarePixelBuffer>::destroy(void)")]
pub fn stub_0xdbfee8() {
    // IDA 0xdbfee8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xdbff20 — __ZN4Ogre9SharedPtrINS_19HardwarePixelBufferEE4swapERS2_
#[doc(alias = "__ZN4Ogre9SharedPtrINS_19HardwarePixelBufferEE4swapERS2_")]
#[doc(alias = "Ogre::SharedPtr<Ogre::HardwarePixelBuffer>::swap(Ogre::SharedPtr<Ogre::HardwarePixelBuffer>&)")]
pub fn stub_0xdbff20() {
    // IDA 0xdbff20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xdc00a8 — __ZN4Ogre28HardwareIndexBufferSharedPtrD0Ev
// type: void __fastcall(Ogre::HardwareIndexBufferSharedPtr *__hidden this)
#[doc(alias = "__ZN4Ogre28HardwareIndexBufferSharedPtrD0Ev")]
#[doc(alias = "Ogre::HardwareIndexBufferSharedPtr::~HardwareIndexBufferSharedPtr()")]
pub fn stub_0xdc00a8() {
    // IDA 0xdc00a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xddd184 — __ZN4Ogre12STLAllocatorINS_9SharedPtrINS_12AbstractNodeEEENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEED1Ev
#[doc(alias = "__ZN4Ogre12STLAllocatorINS_9SharedPtrINS_12AbstractNodeEEENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEED1Ev")]
#[doc(alias = "Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()")]
pub fn stub_0xddd184() {
    // IDA 0xddd184: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xddd188 — __ZN4Ogre9SharedPtrINS_12AbstractNodeEED1Ev
#[doc(alias = "__ZN4Ogre9SharedPtrINS_12AbstractNodeEED1Ev")]
#[doc(alias = "Ogre::SharedPtr<Ogre::AbstractNode>::~SharedPtr()")]
pub fn stub_0xddd188() {
    // IDA 0xddd188: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xddd5fc — __ZN4Ogre9SharedPtrISt4listINS0_INS_12ConcreteNodeEEENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEED1Ev
#[doc(alias = "__ZN4Ogre9SharedPtrISt4listINS0_INS_12ConcreteNodeEEENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEED1Ev")]
#[doc(alias = "Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::~SharedPtr()")]
pub fn stub_0xddd5fc() {
    // IDA 0xddd5fc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xddd6ac — __ZN4Ogre9SharedPtrISt4listINS0_INS_12AbstractNodeEEENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEED1Ev
#[doc(alias = "__ZN4Ogre9SharedPtrISt4listINS0_INS_12AbstractNodeEEENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEED1Ev")]
#[doc(alias = "Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::~SharedPtr()")]
pub fn stub_0xddd6ac() {
    // IDA 0xddd6ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xddd8d4 — __ZNSt4listIN4Ogre9SharedPtrINS0_12AbstractNodeEEENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE6insertISt14_List_iteratorIS3_EEEvSC_T_SD_
// type: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "__ZNSt4listIN4Ogre9SharedPtrINS0_12AbstractNodeEEENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE6insertISt14_List_iteratorIS3_EEEvSC_T_SD_")]
#[doc(alias = "void std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::insert<std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>>(std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>)")]
pub fn stub_0xddd8d4() {
    // IDA 0xddd8d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xddda08 — __ZN4Ogre9SharedPtrISt4listINS0_INS_12ConcreteNodeEEENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEEaSERKSA_
#[doc(alias = "__ZN4Ogre9SharedPtrISt4listINS0_INS_12ConcreteNodeEEENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEEaSERKSA_")]
#[doc(alias = "Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::operator=(Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
pub fn stub_0xddda08() {
    // IDA 0xddda08: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xdddb14 — __ZN4Ogre9SharedPtrISt4listINS0_INS_12AbstractNodeEEENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEEaSERKSA_
#[doc(alias = "__ZN4Ogre9SharedPtrISt4listINS0_INS_12AbstractNodeEEENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEEaSERKSA_")]
#[doc(alias = "Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::operator=(Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
pub fn stub_0xdddb14() {
    // IDA 0xdddb14: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xdddcc8 — __ZN4Ogre9SharedPtrINS_12AbstractNodeEEaSERKS2_
// type: int __fastcall(int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int, int)
#[doc(alias = "__ZN4Ogre9SharedPtrINS_12AbstractNodeEEaSERKS2_")]
#[doc(alias = "Ogre::SharedPtr<Ogre::AbstractNode>::operator=(Ogre::SharedPtr<Ogre::AbstractNode> const&)")]
pub fn stub_0xdddcc8() {
    // IDA 0xdddcc8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xddde48 — __ZN4Ogre9SharedPtrINS_12ConcreteNodeEEaSERKS2_
#[doc(alias = "__ZN4Ogre9SharedPtrINS_12ConcreteNodeEEaSERKS2_")]
#[doc(alias = "Ogre::SharedPtr<Ogre::ConcreteNode>::operator=(Ogre::SharedPtr<Ogre::ConcreteNode> const&)")]
pub fn stub_0xddde48() {
    // IDA 0xddde48: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xdddf54 — __ZN4Ogre9SharedPtrINS_12ConcreteNodeEED1Ev
#[doc(alias = "__ZN4Ogre9SharedPtrINS_12ConcreteNodeEED1Ev")]
#[doc(alias = "Ogre::SharedPtr<Ogre::ConcreteNode>::~SharedPtr()")]
pub fn stub_0xdddf54() {
    // IDA 0xdddf54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xddec50 — __ZN4Ogre9SharedPtrINS_12AbstractNodeEED0Ev
#[doc(alias = "__ZN4Ogre9SharedPtrINS_12AbstractNodeEED0Ev")]
#[doc(alias = "Ogre::SharedPtr<Ogre::AbstractNode>::~SharedPtr()")]
pub fn stub_0xddec50() {
    // IDA 0xddec50: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xdded44 — __ZN4Ogre9SharedPtrINS_12AbstractNodeEE7destroyEv
#[doc(alias = "__ZN4Ogre9SharedPtrINS_12AbstractNodeEE7destroyEv")]
#[doc(alias = "Ogre::SharedPtr<Ogre::AbstractNode>::destroy(void)")]
pub fn stub_0xdded44() {
    // IDA 0xdded44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xdded7c — __ZN4Ogre9SharedPtrINS_12AbstractNodeEE4swapERS2_
#[doc(alias = "__ZN4Ogre9SharedPtrINS_12AbstractNodeEE4swapERS2_")]
#[doc(alias = "Ogre::SharedPtr<Ogre::AbstractNode>::swap(Ogre::SharedPtr<Ogre::AbstractNode>&)")]
pub fn stub_0xdded7c() {
    // IDA 0xdded7c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xddee44 — __ZN4Ogre9SharedPtrINS_12ConcreteNodeEED0Ev
#[doc(alias = "__ZN4Ogre9SharedPtrINS_12ConcreteNodeEED0Ev")]
#[doc(alias = "Ogre::SharedPtr<Ogre::ConcreteNode>::~SharedPtr()")]
pub fn stub_0xddee44() {
    // IDA 0xddee44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xddeef8 — __ZN4Ogre9SharedPtrINS_12ConcreteNodeEE7destroyEv
#[doc(alias = "__ZN4Ogre9SharedPtrINS_12ConcreteNodeEE7destroyEv")]
#[doc(alias = "Ogre::SharedPtr<Ogre::ConcreteNode>::destroy(void)")]
pub fn stub_0xddeef8() {
    // IDA 0xddeef8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xddf114 — __ZN4Ogre9SharedPtrINS_12ConcreteNodeEE4swapERS2_
#[doc(alias = "__ZN4Ogre9SharedPtrINS_12ConcreteNodeEE4swapERS2_")]
#[doc(alias = "Ogre::SharedPtr<Ogre::ConcreteNode>::swap(Ogre::SharedPtr<Ogre::ConcreteNode>&)")]
pub fn stub_0xddf114() {
    // IDA 0xddf114: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xddf698 — __ZN4Ogre12STLAllocatorINS_9SharedPtrINS_12AbstractNodeEEENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEED0Ev
#[doc(alias = "__ZN4Ogre12STLAllocatorINS_9SharedPtrINS_12AbstractNodeEEENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEED0Ev")]
#[doc(alias = "Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()")]
pub fn stub_0xddf698() {
    // IDA 0xddf698: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xddfcac — __ZNSt6vectorISt4pairIN4Ogre9SharedPtrINS1_12AbstractNodeEEESt14_List_iteratorIS4_EENS1_12STLAllocatorIS7_NS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS7_SD_EERKS7_
// type: int(void)
#[doc(alias = "__ZNSt6vectorISt4pairIN4Ogre9SharedPtrINS1_12AbstractNodeEEESt14_List_iteratorIS4_EENS1_12STLAllocatorIS7_NS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS7_SD_EERKS7_")]
#[doc(alias = "std::vector<std::pair<Ogre::SharedPtr<Ogre::AbstractNode>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>>,Ogre::STLAllocator<std::pair<Ogre::SharedPtr<Ogre::AbstractNode>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<Ogre::SharedPtr<Ogre::AbstractNode>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>>*,std::vector<std::pair<Ogre::SharedPtr<Ogre::AbstractNode>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>>,Ogre::STLAllocator<std::pair<Ogre::SharedPtr<Ogre::AbstractNode>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::pair<Ogre::SharedPtr<Ogre::AbstractNode>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>> const&)")]
pub fn stub_0xddfcac() {
    // IDA 0xddfcac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xde0178 — __ZNSt6vectorISt4pairIN4Ogre9SharedPtrINS1_12AbstractNodeEEESt14_List_iteratorIS4_EENS1_12STLAllocatorIS7_NS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEED2Ev
#[doc(alias = "__ZNSt6vectorISt4pairIN4Ogre9SharedPtrINS1_12AbstractNodeEEESt14_List_iteratorIS4_EENS1_12STLAllocatorIS7_NS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEED2Ev")]
#[doc(alias = "std::vector<std::pair<Ogre::SharedPtr<Ogre::AbstractNode>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>>,Ogre::STLAllocator<std::pair<Ogre::SharedPtr<Ogre::AbstractNode>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~vector()")]
pub fn stub_0xde0178() {
    // IDA 0xde0178: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xde0294 — __ZNSt12_Vector_baseISt4pairIN4Ogre9SharedPtrINS1_12AbstractNodeEEESt14_List_iteratorIS4_EENS1_12STLAllocatorIS7_NS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "__ZNSt12_Vector_baseISt4pairIN4Ogre9SharedPtrINS1_12AbstractNodeEEESt14_List_iteratorIS4_EENS1_12STLAllocatorIS7_NS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev")]
#[doc(alias = "std::_Vector_base<std::pair<Ogre::SharedPtr<Ogre::AbstractNode>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>>,Ogre::STLAllocator<std::pair<Ogre::SharedPtr<Ogre::AbstractNode>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
pub fn stub_0xde0294() {
    // IDA 0xde0294: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xde0298 — __ZNSt12_Vector_baseISt4pairIN4Ogre9SharedPtrINS1_12AbstractNodeEEESt14_List_iteratorIS4_EENS1_12STLAllocatorIS7_NS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "__ZNSt12_Vector_baseISt4pairIN4Ogre9SharedPtrINS1_12AbstractNodeEEESt14_List_iteratorIS4_EENS1_12STLAllocatorIS7_NS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev")]
#[doc(alias = "std::_Vector_base<std::pair<Ogre::SharedPtr<Ogre::AbstractNode>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>>,Ogre::STLAllocator<std::pair<Ogre::SharedPtr<Ogre::AbstractNode>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
pub fn stub_0xde0298() {
    // IDA 0xde0298: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xde02a4 — __ZN4Ogre9SharedPtrISt4listINS0_INS_12AbstractNodeEEENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEED0Ev
#[doc(alias = "__ZN4Ogre9SharedPtrISt4listINS0_INS_12AbstractNodeEEENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEED0Ev")]
#[doc(alias = "Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::~SharedPtr()")]
pub fn stub_0xde02a4() {
    // IDA 0xde02a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xde0358 — __ZN4Ogre9SharedPtrISt4listINS0_INS_12AbstractNodeEEENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEE7destroyEv
#[doc(alias = "__ZN4Ogre9SharedPtrISt4listINS0_INS_12AbstractNodeEEENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEE7destroyEv")]
#[doc(alias = "Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::destroy(void)")]
pub fn stub_0xde0358() {
    // IDA 0xde0358: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xde0478 — __ZN4Ogre9SharedPtrISt4listINS0_INS_12AbstractNodeEEENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEE4swapERSA_
#[doc(alias = "__ZN4Ogre9SharedPtrISt4listINS0_INS_12AbstractNodeEEENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEE4swapERSA_")]
#[doc(alias = "Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::swap(Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>&)")]
pub fn stub_0xde0478() {
    // IDA 0xde0478: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xde0498 — __ZN4Ogre9SharedPtrINS_10DataStreamEE7destroyEv
#[doc(alias = "__ZN4Ogre9SharedPtrINS_10DataStreamEE7destroyEv")]
#[doc(alias = "Ogre::SharedPtr<Ogre::DataStream>::destroy(void)")]
pub fn stub_0xde0498() {
    // IDA 0xde0498: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xde04d0 — __ZN4Ogre9SharedPtrINS_10DataStreamEE4swapERS2_
#[doc(alias = "__ZN4Ogre9SharedPtrINS_10DataStreamEE4swapERS2_")]
#[doc(alias = "Ogre::SharedPtr<Ogre::DataStream>::swap(Ogre::SharedPtr<Ogre::DataStream>&)")]
pub fn stub_0xde04d0() {
    // IDA 0xde04d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xde04ec — __ZNSt4listIN4Ogre9SharedPtrINS0_12AbstractNodeEEENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEC2ISt14_List_iteratorIS3_EEET_SD_RKS8_
#[doc(alias = "__ZNSt4listIN4Ogre9SharedPtrINS0_12AbstractNodeEEENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEC2ISt14_List_iteratorIS3_EEET_SD_RKS8_")]
#[doc(alias = "std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::list<std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>>(std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>> const&)")]
pub fn stub_0xde04ec() {
    // IDA 0xde04ec: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

// 0xde063c — __ZNSt10_List_baseIN4Ogre9SharedPtrINS0_12AbstractNodeEEENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD1Ev
#[doc(alias = "__ZNSt10_List_baseIN4Ogre9SharedPtrINS0_12AbstractNodeEEENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD1Ev")]
#[doc(alias = "std::_List_base<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
pub fn stub_0xde063c() {
    // IDA 0xde063c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xde0640 — __ZNSt10_List_baseIN4Ogre9SharedPtrINS0_12AbstractNodeEEENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD0Ev
#[doc(alias = "__ZNSt10_List_baseIN4Ogre9SharedPtrINS0_12AbstractNodeEEENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD0Ev")]
#[doc(alias = "std::_List_base<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
pub fn stub_0xde0640() {
    // IDA 0xde0640: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xde08e0 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrISt4listINS3_INS2_12AbstractNodeEEENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEESt10_Select1stISE_ESt4lessISsENS7_ISE_SA_EEE16_M_insert_uniqueERKSE_
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrISt4listINS3_INS2_12AbstractNodeEEENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEESt10_Select1stISE_ESt4lessISsENS7_ISE_SA_EEE16_M_insert_uniqueERKSE_")]
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> const&)")]
pub fn stub_0xde08e0() {
    // IDA 0xde08e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xde09c4 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrISt4listINS3_INS2_12AbstractNodeEEENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEESt10_Select1stISE_ESt4lessISsENS7_ISE_SA_EEE9_M_insertEPSt18_Rb_tree_node_baseSM_RKSE_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrISt4listINS3_INS2_12AbstractNodeEEENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEESt10_Select1stISE_ESt4lessISsENS7_ISE_SA_EEE9_M_insertEPSt18_Rb_tree_node_baseSM_RKSE_")]
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> const&)")]
pub fn stub_0xde09c4() {
    // IDA 0xde09c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xde0a38 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrISt4listINS3_INS2_12AbstractNodeEEENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEESt10_Select1stISE_ESt4lessISsENS7_ISE_SA_EEE14_M_create_nodeERKSE_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrISt4listINS3_INS2_12AbstractNodeEEENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEESt10_Select1stISE_ESt4lessISsENS7_ISE_SA_EEE14_M_create_nodeERKSE_")]
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> const&)")]
pub fn stub_0xde0a38() {
    // IDA 0xde0a38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xde0b50 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrISt4listINS3_INS2_12AbstractNodeEEENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEESt10_Select1stISE_ESt4lessISsENS7_ISE_SA_EEE4findERS1_
// type: int(void)
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrISt4listINS3_INS2_12AbstractNodeEEENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEESt10_Select1stISE_ESt4lessISsENS7_ISE_SA_EEE4findERS1_")]
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
pub fn stub_0xde0b50() {
    // IDA 0xde0b50: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xde0e78 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrISt4listINS3_INS2_12AbstractNodeEEENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEESt10_Select1stISE_ESt4lessISsENS7_ISE_SA_EEE8_M_eraseEPSt13_Rb_tree_nodeISE_E
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrISt4listINS3_INS2_12AbstractNodeEEENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEESt10_Select1stISE_ESt4lessISsENS7_ISE_SA_EEE8_M_eraseEPSt13_Rb_tree_nodeISE_E")]
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>> *)")]
pub fn stub_0xde0e78() {
    // IDA 0xde0e78: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xde119c — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrISt4listINS3_INS2_12AbstractNodeEEENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEESt10_Select1stISE_ESt4lessISsENS7_ISE_SA_EEE13_Rb_tree_implISI_Lb0EED1Ev
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrISt4listINS3_INS2_12AbstractNodeEEENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEESt10_Select1stISE_ESt4lessISsENS7_ISE_SA_EEE13_Rb_tree_implISI_Lb0EED1Ev")]
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
pub fn stub_0xde119c() {
    // IDA 0xde119c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xde11a0 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrISt4listINS3_INS2_12AbstractNodeEEENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEESt10_Select1stISE_ESt4lessISsENS7_ISE_SA_EEE13_Rb_tree_implISI_Lb0EED0Ev
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrISt4listINS3_INS2_12AbstractNodeEEENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEESt10_Select1stISE_ESt4lessISsENS7_ISE_SA_EEE13_Rb_tree_implISI_Lb0EED0Ev")]
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
pub fn stub_0xde11a0() {
    // IDA 0xde11a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xde11ac — __ZN4Ogre9SharedPtrISt4listINS0_INS_12ConcreteNodeEEENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEED0Ev
#[doc(alias = "__ZN4Ogre9SharedPtrISt4listINS0_INS_12ConcreteNodeEEENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEED0Ev")]
#[doc(alias = "Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::~SharedPtr()")]
pub fn stub_0xde11ac() {
    // IDA 0xde11ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xde1260 — __ZN4Ogre9SharedPtrISt4listINS0_INS_12ConcreteNodeEEENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEE7destroyEv
#[doc(alias = "__ZN4Ogre9SharedPtrISt4listINS0_INS_12ConcreteNodeEEENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEE7destroyEv")]
#[doc(alias = "Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::destroy(void)")]
pub fn stub_0xde1260() {
    // IDA 0xde1260: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xde1380 — __ZN4Ogre9SharedPtrISt4listINS0_INS_12ConcreteNodeEEENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEE4swapERSA_
#[doc(alias = "__ZN4Ogre9SharedPtrISt4listINS0_INS_12ConcreteNodeEEENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEE4swapERSA_")]
#[doc(alias = "Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::swap(Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>&)")]
pub fn stub_0xde1380() {
    // IDA 0xde1380: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xde70a0 — __ZNSt10_List_baseIN4Ogre9SharedPtrINS0_12ConcreteNodeEEENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD1Ev
// type: void()
#[doc(alias = "__ZNSt10_List_baseIN4Ogre9SharedPtrINS0_12ConcreteNodeEEENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD1Ev")]
#[doc(alias = "std::_List_base<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
pub fn stub_0xde70a0() {
    // IDA 0xde70a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xde70a4 — __ZNSt10_List_baseIN4Ogre9SharedPtrINS0_12ConcreteNodeEEENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD0Ev
#[doc(alias = "__ZNSt10_List_baseIN4Ogre9SharedPtrINS0_12ConcreteNodeEEENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD0Ev")]
#[doc(alias = "std::_List_base<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
pub fn stub_0xde70a4() {
    // IDA 0xde70a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xe1d3b0 — __ZN4Ogre10Serializer19determineEndiannessERNS_9SharedPtrINS_10DataStreamEEE
#[doc(alias = "__ZN4Ogre10Serializer19determineEndiannessERNS_9SharedPtrINS_10DataStreamEEE")]
#[doc(alias = "Ogre::Serializer::determineEndianness(Ogre::SharedPtr<Ogre::DataStream> &)")]
pub fn stub_0xe1d3b0() {
    // IDA 0xe1d3b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xe1db30 — __ZN4Ogre10Serializer14readFileHeaderERNS_9SharedPtrINS_10DataStreamEEE
#[doc(alias = "__ZN4Ogre10Serializer14readFileHeaderERNS_9SharedPtrINS_10DataStreamEEE")]
#[doc(alias = "Ogre::Serializer::readFileHeader(Ogre::SharedPtr<Ogre::DataStream> &)")]
pub fn stub_0xe1db30() {
    // IDA 0xe1db30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xe1e170 — __ZN4Ogre10Serializer10readShortsERNS_9SharedPtrINS_10DataStreamEEEPtm
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "__ZN4Ogre10Serializer10readShortsERNS_9SharedPtrINS_10DataStreamEEEPtm")]
#[doc(alias = "Ogre::Serializer::readShorts(Ogre::SharedPtr<Ogre::DataStream> &,unsigned short *,unsigned long)")]
pub fn stub_0xe1e170() {
    // IDA 0xe1e170: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xe1e198 — __ZN4Ogre10Serializer10readStringERNS_9SharedPtrINS_10DataStreamEEE
#[doc(alias = "__ZN4Ogre10Serializer10readStringERNS_9SharedPtrINS_10DataStreamEEE")]
#[doc(alias = "Ogre::Serializer::readString(Ogre::SharedPtr<Ogre::DataStream> &)")]
pub fn stub_0xe1e198() {
    // IDA 0xe1e198: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xe1e1a8 — __ZN4Ogre10Serializer9readChunkERNS_9SharedPtrINS_10DataStreamEEE
#[doc(alias = "__ZN4Ogre10Serializer9readChunkERNS_9SharedPtrINS_10DataStreamEEE")]
#[doc(alias = "Ogre::Serializer::readChunk(Ogre::SharedPtr<Ogre::DataStream> &)")]
pub fn stub_0xe1e1a8() {
    // IDA 0xe1e1a8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xe1e200 — __ZN4Ogre10Serializer8readIntsERNS_9SharedPtrINS_10DataStreamEEEPjm
#[doc(alias = "__ZN4Ogre10Serializer8readIntsERNS_9SharedPtrINS_10DataStreamEEEPjm")]
#[doc(alias = "Ogre::Serializer::readInts(Ogre::SharedPtr<Ogre::DataStream> &,unsigned int *,unsigned long)")]
pub fn stub_0xe1e200() {
    // IDA 0xe1e200: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xe1e228 — __ZN4Ogre10Serializer9readBoolsERNS_9SharedPtrINS_10DataStreamEEEPbm
// type: int __fastcall(int, int, int, size_t __size)
#[doc(alias = "__ZN4Ogre10Serializer9readBoolsERNS_9SharedPtrINS_10DataStreamEEEPbm")]
#[doc(alias = "Ogre::Serializer::readBools(Ogre::SharedPtr<Ogre::DataStream> &,bool *,unsigned long)")]
pub fn stub_0xe1e228() {
    // IDA 0xe1e228: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xe1e270 — __ZN4Ogre10Serializer10readFloatsERNS_9SharedPtrINS_10DataStreamEEEPfm
#[doc(alias = "__ZN4Ogre10Serializer10readFloatsERNS_9SharedPtrINS_10DataStreamEEEPfm")]
#[doc(alias = "Ogre::Serializer::readFloats(Ogre::SharedPtr<Ogre::DataStream> &,float *,unsigned long)")]
pub fn stub_0xe1e270() {
    // IDA 0xe1e270: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xe1e298 — __ZN4Ogre10Serializer10readObjectERNS_9SharedPtrINS_10DataStreamEEERNS_7Vector3E
#[doc(alias = "__ZN4Ogre10Serializer10readObjectERNS_9SharedPtrINS_10DataStreamEEERNS_7Vector3E")]
#[doc(alias = "Ogre::Serializer::readObject(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Vector3 &)")]
pub fn stub_0xe1e298() {
    // IDA 0xe1e298: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xe1e2c0 — __ZN4Ogre10Serializer10readObjectERNS_9SharedPtrINS_10DataStreamEEERNS_10QuaternionE
#[doc(alias = "__ZN4Ogre10Serializer10readObjectERNS_9SharedPtrINS_10DataStreamEEERNS_10QuaternionE")]
#[doc(alias = "Ogre::Serializer::readObject(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Quaternion &)")]
pub fn stub_0xe1e2c0() {
    // IDA 0xe1e2c0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xe1ed00 — __ZN4Ogre12ShadowCaster20generateShadowVolumeEPNS_8EdgeDataERKNS_28HardwareIndexBufferSharedPtrEPKNS_5LightERSt6vectorIPNS_16ShadowRenderableENS_12STLAllocatorISB_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEm
// type: int __fastcall(int, int, int, Ogre::Light *this, int, int)
#[doc(alias = "__ZN4Ogre12ShadowCaster20generateShadowVolumeEPNS_8EdgeDataERKNS_28HardwareIndexBufferSharedPtrEPKNS_5LightERSt6vectorIPNS_16ShadowRenderableENS_12STLAllocatorISB_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEm")]
#[doc(alias = "Ogre::ShadowCaster::generateShadowVolume(Ogre::EdgeData *,Ogre::HardwareIndexBufferSharedPtr const&,Ogre::Light const*,std::vector<Ogre::ShadowRenderable *,Ogre::STLAllocator<Ogre::ShadowRenderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> &,unsigned long)")]
pub fn stub_0xe1ed00() {
    // IDA 0xe1ed00: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xe1f48c — __ZN4Ogre12ShadowCaster15extrudeVerticesERKNS_29HardwareVertexBufferSharedPtrEmRKNS_7Vector4Ef
#[doc(alias = "__ZN4Ogre12ShadowCaster15extrudeVerticesERKNS_29HardwareVertexBufferSharedPtrEmRKNS_7Vector4Ef")]
#[doc(alias = "Ogre::ShadowCaster::extrudeVertices(Ogre::HardwareVertexBufferSharedPtr const&,unsigned long,Ogre::Vector4 const&,float)")]
pub fn stub_0xe1f48c() {
    // IDA 0xe1f48c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xe22d34 — __ZN4Ogre9SharedPtrINS_10GpuProgramEED1Ev
#[doc(alias = "__ZN4Ogre9SharedPtrINS_10GpuProgramEED1Ev")]
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuProgram>::~SharedPtr()")]
pub fn stub_0xe22d34() {
    // IDA 0xe22d34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xe22de4 — __ZN4Ogre9SharedPtrINS_10GpuProgramEED0Ev
#[doc(alias = "__ZN4Ogre9SharedPtrINS_10GpuProgramEED0Ev")]
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuProgram>::~SharedPtr()")]
pub fn stub_0xe22de4() {
    // IDA 0xe22de4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xe22ed8 — __ZN4Ogre9SharedPtrINS_10GpuProgramEE7destroyEv
// type: int(void)
#[doc(alias = "__ZN4Ogre9SharedPtrINS_10GpuProgramEE7destroyEv")]
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuProgram>::destroy(void)")]
pub fn stub_0xe22ed8() {
    // IDA 0xe22ed8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xe22f10 — __ZN4Ogre9SharedPtrINS_10GpuProgramEE4swapERS2_
#[doc(alias = "__ZN4Ogre9SharedPtrINS_10GpuProgramEE4swapERS2_")]
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuProgram>::swap(Ogre::SharedPtr<Ogre::GpuProgram>&)")]
pub fn stub_0xe22f10() {
    // IDA 0xe22f10: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xe30580 — __ZN4Ogre14StaticGeometry6Region33getShadowVolumeRenderableIteratorENS_15ShadowTechniqueEPKNS_5LightEPNS_28HardwareIndexBufferSharedPtrEbfm
// type: int __fastcall(int, int, int, bool, int, int, float, int)
#[doc(alias = "__ZN4Ogre14StaticGeometry6Region33getShadowVolumeRenderableIteratorENS_15ShadowTechniqueEPKNS_5LightEPNS_28HardwareIndexBufferSharedPtrEbfm")]
#[doc(alias = "Ogre::StaticGeometry::Region::getShadowVolumeRenderableIterator(Ogre::ShadowTechnique,Ogre::Light const*,Ogre::HardwareIndexBufferSharedPtr *,bool,float,unsigned long)")]
pub fn stub_0xe30580() {
    // IDA 0xe30580: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xe306c8 — __ZN4Ogre14StaticGeometry9LODBucket23updateShadowRenderablesENS_15ShadowTechniqueERKNS_7Vector4EPNS_28HardwareIndexBufferSharedPtrEbfm
// type: int __fastcall(int, int, int, int, int, float)
#[doc(alias = "__ZN4Ogre14StaticGeometry9LODBucket23updateShadowRenderablesENS_15ShadowTechniqueERKNS_7Vector4EPNS_28HardwareIndexBufferSharedPtrEbfm")]
#[doc(alias = "Ogre::StaticGeometry::LODBucket::updateShadowRenderables(Ogre::ShadowTechnique,Ogre::Vector4 const&,Ogre::HardwareIndexBufferSharedPtr *,bool,float,unsigned long)")]
pub fn stub_0xe306c8() {
    // IDA 0xe306c8: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

// 0xe30bec — __ZN4Ogre14StaticGeometry9LODBucket19LODShadowRenderableC2EPS1_PNS_28HardwareIndexBufferSharedPtrEPKNS_10VertexDataEbb
// type: _DWORD __fastcall(Ogre::StaticGeometry::LODBucket::LODShadowRenderable *__hidden this, Ogre::StaticGeometry::LODBucket *, Ogre::HardwareIndexBufferSharedPtr *, const Ogre::VertexData *, bool, bool)
#[doc(alias = "__ZN4Ogre14StaticGeometry9LODBucket19LODShadowRenderableC2EPS1_PNS_28HardwareIndexBufferSharedPtrEPKNS_10VertexDataEbb")]
#[doc(alias = "Ogre::StaticGeometry::LODBucket::LODShadowRenderable::LODShadowRenderable(Ogre::StaticGeometry::LODBucket*,Ogre::HardwareIndexBufferSharedPtr *,Ogre::VertexData const*,bool,bool)")]
pub fn stub_0xe30bec() {
    // IDA 0xe30bec: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

// 0xe312b0 — __ZN4Ogre14StaticGeometry9LODBucket19LODShadowRenderable17rebindIndexBufferERKNS_28HardwareIndexBufferSharedPtrE
// type: _DWORD __fastcall(Ogre::StaticGeometry::LODBucket::LODShadowRenderable *__hidden this, const Ogre::HardwareIndexBufferSharedPtr *)
#[doc(alias = "__ZN4Ogre14StaticGeometry9LODBucket19LODShadowRenderable17rebindIndexBufferERKNS_28HardwareIndexBufferSharedPtrE")]
#[doc(alias = "Ogre::StaticGeometry::LODBucket::LODShadowRenderable::rebindIndexBuffer(Ogre::HardwareIndexBufferSharedPtr const&)")]
pub fn stub_0xe312b0() {
    // IDA 0xe312b0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xe4542c — __ZN4Ogre7Texture11loadRawDataERNS_9SharedPtrINS_10DataStreamEEEttNS_11PixelFormatE
#[doc(alias = "__ZN4Ogre7Texture11loadRawDataERNS_9SharedPtrINS_10DataStreamEEEttNS_11PixelFormatE")]
#[doc(alias = "Ogre::Texture::loadRawData(Ogre::SharedPtr<Ogre::DataStream> &,unsigned short,unsigned short,Ogre::PixelFormat)")]
pub fn stub_0xe4542c() {
    // IDA 0xe4542c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xe481f8 — __ZN4Ogre14TextureManager11loadRawDataERKSsS2_RNS_9SharedPtrINS_10DataStreamEEEttNS_11PixelFormatENS_11TextureTypeEifb
// type: int __fastcall(int, int, int, int, char, int, int, int, int, Ogre::NedPoolingImpl *, float, int, int, int)
#[doc(alias = "__ZN4Ogre14TextureManager11loadRawDataERKSsS2_RNS_9SharedPtrINS_10DataStreamEEEttNS_11PixelFormatENS_11TextureTypeEifb")]
#[doc(alias = "Ogre::TextureManager::loadRawData(std::string const&,std::string const&,Ogre::SharedPtr<Ogre::DataStream> &,unsigned short,unsigned short,Ogre::PixelFormat,Ogre::TextureType,int,float,bool)")]
pub fn stub_0xe481f8() {
    // IDA 0xe481f8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xe501b8 — __ZN4Ogre9SharedPtrINS_19HighLevelGpuProgramEEaSERKS2_
// type: _DWORD *__fastcall(_DWORD *, _DWORD *, int, int, Ogre::NedPoolingImpl *, int, int, int, int, int)
#[doc(alias = "__ZN4Ogre9SharedPtrINS_19HighLevelGpuProgramEEaSERKS2_")]
#[doc(alias = "Ogre::SharedPtr<Ogre::HighLevelGpuProgram>::operator=(Ogre::SharedPtr<Ogre::HighLevelGpuProgram> const&)")]
pub fn stub_0xe501b8() {
    // IDA 0xe501b8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xe58cc8 — __ZN4Ogre9SharedPtrINS_20DefaultWorkQueueBase20RequestHandlerHolderEED1Ev
#[doc(alias = "__ZN4Ogre9SharedPtrINS_20DefaultWorkQueueBase20RequestHandlerHolderEED1Ev")]
#[doc(alias = "Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>::~SharedPtr()")]
pub fn stub_0xe58cc8() {
    // IDA 0xe58cc8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xe58fe0 — __ZNSt8_Rb_treeItSt4pairIKtSt4listIN4Ogre9SharedPtrINS3_20DefaultWorkQueueBase20RequestHandlerHolderEEENS3_12STLAllocatorIS7_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ESt4lessItENS8_ISE_SB_EEE7_M_copyEPKSt13_Rb_tree_nodeISE_EPSM_
// type: int __fastcall(int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, void *, int)
#[doc(alias = "__ZNSt8_Rb_treeItSt4pairIKtSt4listIN4Ogre9SharedPtrINS3_20DefaultWorkQueueBase20RequestHandlerHolderEEENS3_12STLAllocatorIS7_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ESt4lessItENS8_ISE_SB_EEE7_M_copyEPKSt13_Rb_tree_nodeISE_EPSM_")]
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_copy(std::_Rb_tree_node<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> const*,std::_Rb_tree_node<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>*)")]
pub fn stub_0xe58fe0() {
    // IDA 0xe58fe0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xe591d8 — __ZNSt8_Rb_treeItSt4pairIKtSt4listIN4Ogre9SharedPtrINS3_20DefaultWorkQueueBase20RequestHandlerHolderEEENS3_12STLAllocatorIS7_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ESt4lessItENS8_ISE_SB_EEE8_M_eraseEPSt13_Rb_tree_nodeISE_E
#[doc(alias = "__ZNSt8_Rb_treeItSt4pairIKtSt4listIN4Ogre9SharedPtrINS3_20DefaultWorkQueueBase20RequestHandlerHolderEEENS3_12STLAllocatorIS7_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ESt4lessItENS8_ISE_SB_EEE8_M_eraseEPSt13_Rb_tree_nodeISE_E")]
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> *)")]
pub fn stub_0xe591d8() {
    // IDA 0xe591d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xe592b4 — __ZNSt4listIN4Ogre9SharedPtrINS0_20DefaultWorkQueueBase20RequestHandlerHolderEEENS0_12STLAllocatorIS4_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEC2ERKSA_
#[doc(alias = "__ZNSt4listIN4Ogre9SharedPtrINS0_20DefaultWorkQueueBase20RequestHandlerHolderEEENS0_12STLAllocatorIS4_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEC2ERKSA_")]
#[doc(alias = "std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::list(std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
pub fn stub_0xe592b4() {
    // IDA 0xe592b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xe59404 — __ZNSt10_List_baseIN4Ogre9SharedPtrINS0_20DefaultWorkQueueBase20RequestHandlerHolderEEENS0_12STLAllocatorIS4_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD1Ev
#[doc(alias = "__ZNSt10_List_baseIN4Ogre9SharedPtrINS0_20DefaultWorkQueueBase20RequestHandlerHolderEEENS0_12STLAllocatorIS4_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD1Ev")]
#[doc(alias = "std::_List_base<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
pub fn stub_0xe59404() {
    // IDA 0xe59404: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xe59408 — __ZNSt10_List_baseIN4Ogre9SharedPtrINS0_20DefaultWorkQueueBase20RequestHandlerHolderEEENS0_12STLAllocatorIS4_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD0Ev
#[doc(alias = "__ZNSt10_List_baseIN4Ogre9SharedPtrINS0_20DefaultWorkQueueBase20RequestHandlerHolderEEENS0_12STLAllocatorIS4_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD0Ev")]
#[doc(alias = "std::_List_base<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
pub fn stub_0xe59408() {
    // IDA 0xe59408: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xe59944 — __ZN4Ogre9SharedPtrINS_20DefaultWorkQueueBase20RequestHandlerHolderEED0Ev
#[doc(alias = "__ZN4Ogre9SharedPtrINS_20DefaultWorkQueueBase20RequestHandlerHolderEED0Ev")]
#[doc(alias = "Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>::~SharedPtr()")]
pub fn stub_0xe59944() {
    // IDA 0xe59944: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xe599f8 — __ZN4Ogre9SharedPtrINS_20DefaultWorkQueueBase20RequestHandlerHolderEE7destroyEv
#[doc(alias = "__ZN4Ogre9SharedPtrINS_20DefaultWorkQueueBase20RequestHandlerHolderEE7destroyEv")]
#[doc(alias = "Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>::destroy(void)")]
pub fn stub_0xe599f8() {
    // IDA 0xe599f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xe59ab8 — __ZN4Ogre9SharedPtrINS_20DefaultWorkQueueBase20RequestHandlerHolderEE4swapERS3_
#[doc(alias = "__ZN4Ogre9SharedPtrINS_20DefaultWorkQueueBase20RequestHandlerHolderEE4swapERS3_")]
#[doc(alias = "Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>::swap(Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>&)")]
pub fn stub_0xe59ab8() {
    // IDA 0xe59ab8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
