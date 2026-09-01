//! rendering shard 400 — 100 stubs 0x5f45a8..0x5f8cf4 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 43210->43310 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Filter Ogre|G3D|Gfx|Render|Adorn 15586/15586 filtered complete; this batch is pure gap filler EA asc not yet in rbx_rendering.
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc gap not yet in rbx_rendering 0x5f45a8..0x5f8cf4 (100 stubs)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x5f45a8 — __ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataD1Ev
#[doc(alias = "__ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataD1Ev")]
#[doc(alias = "boost::thread_specific_ptr<RBX::Security::Context>::delete_data::~delete_data()")]
// was: __ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataD1Ev
pub fn stub_5f45a8() -> ! {
    todo!("0x5f45a8 boost::thread_specific_ptr<RBX::Security::Context>::delete_data::~delete_data()")
}

// 0x5f45b0 — __ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataclEPv
#[doc(alias = "__ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataclEPv")]
#[doc(alias = "boost::thread_specific_ptr<RBX::Security::Context>::delete_data::operator()(void *)")]
// was: __ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataclEPv
pub fn stub_5f45b0() -> ! {
    todo!("0x5f45b0 boost::thread_specific_ptr<RBX::Security::Context>::delete_data::operator()(void *)")
}

// 0x5f45c0 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataENS0_14do_heap_deleteIS7_EEED0Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataENS0_14do_heap_deleteIS7_EEED0Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::Security::Context>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::Security::Context>::delete_data>>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataENS0_14do_heap_deleteIS7_EEED0Ev
pub fn stub_5f45c0() -> ! {
    todo!("0x5f45c0 boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::Security::Context>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::Security::Context>::delete_data>>::~sp_counted_impl_pd()")
}

// 0x5f45c8 — __ZN3RBX6RbxRayD0Ev
// type: void __fastcall(RBX::RbxRay *__hidden this)
#[doc(alias = "__ZN3RBX6RbxRayD0Ev")]
#[doc(alias = "RBX::RbxRay::~RbxRay()")]
// was: __ZN3RBX6RbxRayD0Ev
pub fn stub_5f45c8() -> ! {
    todo!("0x5f45c8 RBX::RbxRay::~RbxRay()")
}

// 0x5f45d0 — __ZNK3RBX17copy_on_write_ptrISt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EEEdeEv
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZNK3RBX17copy_on_write_ptrISt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EEEdeEv")]
#[doc(alias = "RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>::operator*(void)const")]
// was: __ZNK3RBX17copy_on_write_ptrISt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EEEdeEv
pub fn stub_5f45d0() -> ! {
    todo!("0x5f45d0 RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>::operator*(void)const")
}

// 0x5f4628 — __ZN3RBX9AllocatorINS_12PartInstance20OnDemandPartInstanceEEnwEm
// type: int(void)
#[doc(alias = "__ZN3RBX9AllocatorINS_12PartInstance20OnDemandPartInstanceEEnwEm")]
#[doc(alias = "RBX::Allocator<RBX::PartInstance::OnDemandPartInstance>::operator new(unsigned long)")]
// was: __ZN3RBX9AllocatorINS_12PartInstance20OnDemandPartInstanceEEnwEm
pub fn stub_5f4628() -> ! {
    todo!("0x5f4628 RBX::Allocator<RBX::PartInstance::OnDemandPartInstance>::operator new(unsigned long)")
}

// 0x5f4698 — __ZN5boost14singleton_poolIN3RBX12PartInstance20OnDemandPartInstanceELj200ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
// type: int(void)
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX12PartInstance20OnDemandPartInstanceELj200ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")]
#[doc(alias = "boost::singleton_pool<RBX::PartInstance::OnDemandPartInstance,200u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// was: __ZN5boost14singleton_poolIN3RBX12PartInstance20OnDemandPartInstanceELj200ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_5f4698() -> ! {
    todo!("0x5f4698 boost::singleton_pool<RBX::PartInstance::OnDemandPartInstance,200u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")
}

// 0x5f46d0 — __ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE7destroyEv
// type: int(void)
#[doc(alias = "__ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE7destroyEv")]
#[doc(alias = "boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::destroy(void)")]
// was: __ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE7destroyEv
pub fn stub_5f46d0() -> ! {
    todo!("0x5f46d0 boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::destroy(void)")
}

// 0x5f46f4 — __ZN3RBX18InterpolatedCFrameC2Ev
// type: _DWORD __fastcall(RBX::InterpolatedCFrame *__hidden this)
#[doc(alias = "__ZN3RBX18InterpolatedCFrameC2Ev")]
#[doc(alias = "RBX::InterpolatedCFrame::InterpolatedCFrame(void)")]
// was: __ZN3RBX18InterpolatedCFrameC2Ev
pub fn stub_5f46f4() -> ! {
    todo!("0x5f46f4 RBX::InterpolatedCFrame::InterpolatedCFrame(void)")
}

// 0x5f4744 — __ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE8allocateEm
// type: int(void)
#[doc(alias = "__ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE8allocateEm")]
#[doc(alias = "boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::allocate(unsigned long)")]
// was: __ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE8allocateEm
pub fn stub_5f4744() -> ! {
    todo!("0x5f4744 boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::allocate(unsigned long)")
}

// 0x5f4868 — __ZN5boost15throw_exceptionISt12length_errorEEvRKT_
// type: int(void)
#[doc(alias = "__ZN5boost15throw_exceptionISt12length_errorEEvRKT_")]
#[doc(alias = "void boost::throw_exception<std::length_error>(std::length_error const&)")]
// was: __ZN5boost15throw_exceptionISt12length_errorEEvRKT_
pub fn stub_5f4868() -> ! {
    todo!("0x5f4868 void boost::throw_exception<std::length_error>(std::length_error const&)")
}

// 0x5f4948 — __ZN5boost16exception_detail19error_info_injectorISt12length_errorED2Ev
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "__ZN5boost16exception_detail19error_info_injectorISt12length_errorED2Ev")]
#[doc(alias = "boost::exception_detail::error_info_injector<std::length_error>::~error_info_injector()")]
// was: __ZN5boost16exception_detail19error_info_injectorISt12length_errorED2Ev
pub fn stub_5f4948() -> ! {
    todo!("0x5f4948 boost::exception_detail::error_info_injector<std::length_error>::~error_info_injector()")
}

// 0x5f4a00 — __ZThn8_N5boost16exception_detail19error_info_injectorISt12length_errorED1Ev
#[doc(alias = "__ZThn8_N5boost16exception_detail19error_info_injectorISt12length_errorED1Ev")]
#[doc(alias = "non-virtual thunk to boost::exception_detail::error_info_injector<std::length_error>::~error_info_injector()")]
// was: __ZThn8_N5boost16exception_detail19error_info_injectorISt12length_errorED1Ev
pub fn stub_5f4a00() -> ! {
    todo!("0x5f4a00 non-virtual thunk to boost::exception_detail::error_info_injector<std::length_error>::~error_info_injector()")
}

// 0x5f4a08 — __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEED1Ev
#[doc(alias = "__ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEED1Ev")]
#[doc(alias = "non-virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::~clone_impl()")]
// was: __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEED1Ev
pub fn stub_5f4a08() -> ! {
    todo!("0x5f4a08 non-virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::~clone_impl()")
}

// 0x5f4a10 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEED1Ev
#[doc(alias = "__ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEED1Ev")]
#[doc(alias = "virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::~clone_impl()")]
// was: __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEED1Ev
pub fn stub_5f4a10() -> ! {
    todo!("0x5f4a10 virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::~clone_impl()")
}

// 0x5f4a20 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEE5cloneEv
#[doc(alias = "__ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEE5cloneEv")]
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::clone(void)const")]
// was: __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEE5cloneEv
pub fn stub_5f4a20() -> ! {
    todo!("0x5f4a20 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::clone(void)const")
}

// 0x5f4ae0 — __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEED0Ev
#[doc(alias = "__ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEED0Ev")]
#[doc(alias = "non-virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::~clone_impl()")]
// was: __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEED0Ev
pub fn stub_5f4ae0() -> ! {
    todo!("0x5f4ae0 non-virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::~clone_impl()")
}

// 0x5f4af8 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEE5cloneEv
#[doc(alias = "__ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEE5cloneEv")]
#[doc(alias = "virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::clone(void)const")]
// was: __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEE5cloneEv
pub fn stub_5f4af8() -> ! {
    todo!("0x5f4af8 virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::clone(void)const")
}

// 0x5f4b08 — __ZN5boost16exception_detail19error_info_injectorISt12length_errorED0Ev
#[doc(alias = "__ZN5boost16exception_detail19error_info_injectorISt12length_errorED0Ev")]
#[doc(alias = "boost::exception_detail::error_info_injector<std::length_error>::~error_info_injector()")]
// was: __ZN5boost16exception_detail19error_info_injectorISt12length_errorED0Ev
pub fn stub_5f4b08() -> ! {
    todo!("0x5f4b08 boost::exception_detail::error_info_injector<std::length_error>::~error_info_injector()")
}

// 0x5f4b20 — __ZN5boost16exception_detail19error_info_injectorISt12length_errorEC2ERKS2_
#[doc(alias = "__ZN5boost16exception_detail19error_info_injectorISt12length_errorEC2ERKS2_")]
#[doc(alias = "boost::exception_detail::error_info_injector<std::length_error>::error_info_injector(std::length_error const&)")]
// was: __ZN5boost16exception_detail19error_info_injectorISt12length_errorEC2ERKS2_
pub fn stub_5f4b20() -> ! {
    todo!("0x5f4b20 boost::exception_detail::error_info_injector<std::length_error>::error_info_injector(std::length_error const&)")
}

// 0x5f4c08 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEEC1ERKS4_
#[doc(alias = "__ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEEC1ERKS4_")]
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::clone_impl(boost::exception_detail::error_info_injector<std::length_error> const&)")]
// was: __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEEC1ERKS4_
pub fn stub_5f4c08() -> ! {
    todo!("0x5f4c08 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::clone_impl(boost::exception_detail::error_info_injector<std::length_error> const&)")
}

// 0x5f4d78 — __ZN3RBX16OnDemandInstanceD2Ev
// type: void __fastcall(RBX::OnDemandInstance *__hidden this)
#[doc(alias = "__ZN3RBX16OnDemandInstanceD2Ev")]
#[doc(alias = "RBX::OnDemandInstance::~OnDemandInstance()")]
// was: __ZN3RBX16OnDemandInstanceD2Ev
pub fn stub_5f4d78() -> ! {
    todo!("0x5f4d78 RBX::OnDemandInstance::~OnDemandInstance()")
}

// 0x5f4fdc — __ZN3RBX18OnDemandPVInstanceD1Ev
// type: void __fastcall(RBX::OnDemandPVInstance *__hidden this)
#[doc(alias = "__ZN3RBX18OnDemandPVInstanceD1Ev")]
#[doc(alias = "RBX::OnDemandPVInstance::~OnDemandPVInstance()")]
// was: __ZN3RBX18OnDemandPVInstanceD1Ev
pub fn stub_5f4fdc() -> ! {
    todo!("0x5f4fdc RBX::OnDemandPVInstance::~OnDemandPVInstance()")
}

// 0x5f5008 — __ZN3RBX18OnDemandPVInstanceD0Ev
// type: void __fastcall(RBX::OnDemandPVInstance *__hidden this)
#[doc(alias = "__ZN3RBX18OnDemandPVInstanceD0Ev")]
#[doc(alias = "RBX::OnDemandPVInstance::~OnDemandPVInstance()")]
// was: __ZN3RBX18OnDemandPVInstanceD0Ev
pub fn stub_5f5008() -> ! {
    todo!("0x5f5008 RBX::OnDemandPVInstance::~OnDemandPVInstance()")
}

// 0x5f50dc — __ZN3RBX16OnDemandInstanceD1Ev
// type: void __fastcall(RBX::OnDemandInstance *__hidden this)
#[doc(alias = "__ZN3RBX16OnDemandInstanceD1Ev")]
#[doc(alias = "RBX::OnDemandInstance::~OnDemandInstance()")]
// was: __ZN3RBX16OnDemandInstanceD1Ev
pub fn stub_5f50dc() -> ! {
    todo!("0x5f50dc RBX::OnDemandInstance::~OnDemandInstance()")
}

// 0x5f50e0 — __ZN3RBX16OnDemandInstanceD0Ev
// type: void __fastcall(RBX::OnDemandInstance *__hidden this)
#[doc(alias = "__ZN3RBX16OnDemandInstanceD0Ev")]
#[doc(alias = "RBX::OnDemandInstance::~OnDemandInstance()")]
// was: __ZN3RBX16OnDemandInstanceD0Ev
pub fn stub_5f50e0() -> ! {
    todo!("0x5f50e0 RBX::OnDemandInstance::~OnDemandInstance()")
}

// 0x5f5198 — __ZN3rbx7signals6signalIFvbEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "__ZN3rbx7signals6signalIFvbEE13disconnectAllEv")]
#[doc(alias = "rbx::signals::signal<void ()(bool)>::disconnectAll(void)")]
// was: __ZN3rbx7signals6signalIFvbEE13disconnectAllEv
pub fn stub_5f5198() -> ! {
    todo!("0x5f5198 rbx::signals::signal<void ()(bool)>::disconnectAll(void)")
}

// 0x5f5310 — __ZN3RBX16OnDemandInstanceC2Ev
// type: _DWORD __fastcall(RBX::OnDemandInstance *__hidden this)
#[doc(alias = "__ZN3RBX16OnDemandInstanceC2Ev")]
#[doc(alias = "RBX::OnDemandInstance::OnDemandInstance(void)")]
// was: __ZN3RBX16OnDemandInstanceC2Ev
pub fn stub_5f5310() -> ! {
    todo!("0x5f5310 RBX::OnDemandInstance::OnDemandInstance(void)")
}

// 0x5f5564 — __ZN3RBX9AllocatorINS_18OnDemandPVInstanceEEC2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX9AllocatorINS_18OnDemandPVInstanceEEC2Ev")]
#[doc(alias = "RBX::Allocator<RBX::OnDemandPVInstance>::Allocator(void)")]
// was: __ZN3RBX9AllocatorINS_18OnDemandPVInstanceEEC2Ev
pub fn stub_5f5564() -> ! {
    todo!("0x5f5564 RBX::Allocator<RBX::OnDemandPVInstance>::Allocator(void)")
}

// 0x5f55c8 — __ZN3RBX9AllocatorINS_18OnDemandPVInstanceEE13releaseMemoryEv
#[doc(alias = "__ZN3RBX9AllocatorINS_18OnDemandPVInstanceEE13releaseMemoryEv")]
#[doc(alias = "RBX::Allocator<RBX::OnDemandPVInstance>::releaseMemory(void)")]
// was: __ZN3RBX9AllocatorINS_18OnDemandPVInstanceEE13releaseMemoryEv
pub fn stub_5f55c8() -> ! {
    todo!("0x5f55c8 RBX::Allocator<RBX::OnDemandPVInstance>::releaseMemory(void)")
}

// 0x5f55e4 — __ZN5boost14singleton_poolIN3RBX18OnDemandPVInstanceELj24ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
// type: int(void)
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX18OnDemandPVInstanceELj24ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
#[doc(alias = "boost::singleton_pool<RBX::OnDemandPVInstance,24u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// was: __ZN5boost14singleton_poolIN3RBX18OnDemandPVInstanceELj24ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_5f55e4() -> ! {
    todo!("0x5f55e4 boost::singleton_pool<RBX::OnDemandPVInstance,24u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")
}

// 0x5f5618 — __ZN3RBX9AllocatorINS_16OnDemandInstanceEE13releaseMemoryEv
#[doc(alias = "__ZN3RBX9AllocatorINS_16OnDemandInstanceEE13releaseMemoryEv")]
#[doc(alias = "RBX::Allocator<RBX::OnDemandInstance>::releaseMemory(void)")]
// was: __ZN3RBX9AllocatorINS_16OnDemandInstanceEE13releaseMemoryEv
pub fn stub_5f5618() -> ! {
    todo!("0x5f5618 RBX::Allocator<RBX::OnDemandInstance>::releaseMemory(void)")
}

// 0x5f5634 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8MaterialEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// type: int(void)
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8MaterialEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")]
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Material>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Material>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Material>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Material>> *)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8MaterialEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
pub fn stub_5f5634() -> ! {
    todo!("0x5f5634 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Material>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Material>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Material>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Material>> *)")
}

// 0x5f565c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12PartInstance10FormFactorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int(void)
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12PartInstance10FormFactorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PartInstance::FormFactor>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PartInstance::FormFactor>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PartInstance::FormFactor>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::PartInstance::FormFactor>> *)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12PartInstance10FormFactorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_5f565c() -> ! {
    todo!("0x5f565c std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PartInstance::FormFactor>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PartInstance::FormFactor>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PartInstance::FormFactor>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::PartInstance::FormFactor>> *)")
}

// 0x5f5684 — __GLOBAL__I_a_233
#[doc(alias = "__GLOBAL__I_a_233")]
#[doc(alias = "global constructor keyed to_a_233")]
// was: __GLOBAL__I_a_233
pub fn stub_5f5684() -> ! {
    todo!("0x5f5684 `global constructor keyed to'_a_233")
}

// 0x5f6800 — __ZN3RBX19PhysicsInstructionsC1Ev
// type: _DWORD __fastcall(RBX::PhysicsInstructions *__hidden this)
#[doc(alias = "__ZN3RBX19PhysicsInstructionsC1Ev")]
#[doc(alias = "RBX::PhysicsInstructions::PhysicsInstructions(void)")]
// was: __ZN3RBX19PhysicsInstructionsC1Ev
pub fn stub_5f6800() -> ! {
    todo!("0x5f6800 RBX::PhysicsInstructions::PhysicsInstructions(void)")
}

// 0x5f6804 — __ZN3RBX19PhysicsInstructionsC2Ev
// type: _DWORD __fastcall(RBX::PhysicsInstructions *__hidden this)
#[doc(alias = "__ZN3RBX19PhysicsInstructionsC2Ev")]
#[doc(alias = "RBX::PhysicsInstructions::PhysicsInstructions(void)")]
// was: __ZN3RBX19PhysicsInstructionsC2Ev
pub fn stub_5f6804() -> ! {
    todo!("0x5f6804 RBX::PhysicsInstructions::PhysicsInstructions(void)")
}

// 0x5f6948 — __ZN3RBX19PhysicsInstructions25dPhysicsServerDutyPercentEv
// type: _DWORD __fastcall(RBX::PhysicsInstructions *__hidden this)
#[doc(alias = "__ZN3RBX19PhysicsInstructions25dPhysicsServerDutyPercentEv")]
#[doc(alias = "RBX::PhysicsInstructions::dPhysicsServerDutyPercent(void)")]
// was: __ZN3RBX19PhysicsInstructions25dPhysicsServerDutyPercentEv
pub fn stub_5f6948() -> ! {
    todo!("0x5f6948 RBX::PhysicsInstructions::dPhysicsServerDutyPercent(void)")
}

// 0x5f6968 — sub_5F6968
#[doc(alias = "sub_5F6968")]
#[doc(alias = "sub_5F6968")]
// was: sub_5F6968
pub fn stub_5f6968() -> ! {
    todo!("0x5f6968 sub_5F6968")
}

// 0x5f6978 — __ZN3RBX19PhysicsInstructions22changeSimulationRadiusEPNS_7Network6PlayerEf
// type: _DWORD __fastcall(RBX::PhysicsInstructions *__hidden this, RBX::Network::Player *, float)
#[doc(alias = "__ZN3RBX19PhysicsInstructions22changeSimulationRadiusEPNS_7Network6PlayerEf")]
#[doc(alias = "RBX::PhysicsInstructions::changeSimulationRadius(RBX::Network::Player *,float)")]
// was: __ZN3RBX19PhysicsInstructions22changeSimulationRadiusEPNS_7Network6PlayerEf
pub fn stub_5f6978() -> ! {
    todo!("0x5f6978 RBX::PhysicsInstructions::changeSimulationRadius(RBX::Network::Player *,float)")
}

// 0x5f69ec — __ZN3RBX19PhysicsInstructions25changeMaxSimulationRadiusEPNS_7Network6PlayerEf
// type: _DWORD __fastcall(RBX::PhysicsInstructions *__hidden this, RBX::Network::Player *, float)
#[doc(alias = "__ZN3RBX19PhysicsInstructions25changeMaxSimulationRadiusEPNS_7Network6PlayerEf")]
#[doc(alias = "RBX::PhysicsInstructions::changeMaxSimulationRadius(RBX::Network::Player *,float)")]
// was: __ZN3RBX19PhysicsInstructions25changeMaxSimulationRadiusEPNS_7Network6PlayerEf
pub fn stub_5f69ec() -> ! {
    todo!("0x5f69ec RBX::PhysicsInstructions::changeMaxSimulationRadius(RBX::Network::Player *,float)")
}

// 0x5f6a60 — __ZN3RBX19PhysicsInstructions25dPhysicsClientDutyPercentEv
// type: _DWORD __fastcall(RBX::PhysicsInstructions *__hidden this)
#[doc(alias = "__ZN3RBX19PhysicsInstructions25dPhysicsClientDutyPercentEv")]
#[doc(alias = "RBX::PhysicsInstructions::dPhysicsClientDutyPercent(void)")]
// was: __ZN3RBX19PhysicsInstructions25dPhysicsClientDutyPercentEv
pub fn stub_5f6a60() -> ! {
    todo!("0x5f6a60 RBX::PhysicsInstructions::dPhysicsClientDutyPercent(void)")
}

// 0x5f6a78 — __ZN3RBX19PhysicsInstructions34dPhysicsClientEThrottleDutyPercentEv
// type: _DWORD __fastcall(RBX::PhysicsInstructions *__hidden this)
#[doc(alias = "__ZN3RBX19PhysicsInstructions34dPhysicsClientEThrottleDutyPercentEv")]
#[doc(alias = "RBX::PhysicsInstructions::dPhysicsClientEThrottleDutyPercent(void)")]
// was: __ZN3RBX19PhysicsInstructions34dPhysicsClientEThrottleDutyPercentEv
pub fn stub_5f6a78() -> ! {
    todo!("0x5f6a78 RBX::PhysicsInstructions::dPhysicsClientEThrottleDutyPercent(void)")
}

// 0x5f6a90 — __ZN3RBX19PhysicsInstructions12setThrottlesEPNS_7Network6PlayerEPNS_9WorkspaceEdd
// type: _DWORD __fastcall(RBX::PhysicsInstructions *__hidden this, RBX::Network::Player *, RBX::Workspace *, double, double)
#[doc(alias = "__ZN3RBX19PhysicsInstructions12setThrottlesEPNS_7Network6PlayerEPNS_9WorkspaceEdd")]
#[doc(alias = "RBX::PhysicsInstructions::setThrottles(RBX::Network::Player *,RBX::Workspace *,double,double)")]
// was: __ZN3RBX19PhysicsInstructions12setThrottlesEPNS_7Network6PlayerEPNS_9WorkspaceEdd
pub fn stub_5f6a90() -> ! {
    todo!("0x5f6a90 RBX::PhysicsInstructions::setThrottles(RBX::Network::Player *,RBX::Workspace *,double,double)")
}

// 0x5f6cf8 — __ZNSt6vectorIdSaIdEE6resizeEmd
// type: int(void)
#[doc(alias = "__ZNSt6vectorIdSaIdEE6resizeEmd")]
#[doc(alias = "std::vector<double,std::allocator<double>>::resize(unsigned long,double)")]
// was: __ZNSt6vectorIdSaIdEE6resizeEmd
pub fn stub_5f6cf8() -> ! {
    todo!("0x5f6cf8 std::vector<double,std::allocator<double>>::resize(unsigned long,double)")
}

// 0x5f6d3c — __GLOBAL__I_a_234
#[doc(alias = "__GLOBAL__I_a_234")]
#[doc(alias = "global constructor keyed to_a_234")]
// was: __GLOBAL__I_a_234
pub fn stub_5f6d3c() -> ! {
    todo!("0x5f6d3c `global constructor keyed to'_a_234")
}

// 0x5f6fac — __ZN3RBX14PhysicsServiceD0Ev
// type: void __fastcall(RBX::PhysicsService *__hidden this)
#[doc(alias = "__ZN3RBX14PhysicsServiceD0Ev")]
#[doc(alias = "RBX::PhysicsService::~PhysicsService()")]
// was: __ZN3RBX14PhysicsServiceD0Ev
pub fn stub_5f6fac() -> ! {
    todo!("0x5f6fac RBX::PhysicsService::~PhysicsService()")
}

// 0x5f704c — __ZN3RBX14PhysicsServiceD1Ev
// type: void __fastcall(RBX::PhysicsService *__hidden this)
#[doc(alias = "__ZN3RBX14PhysicsServiceD1Ev")]
#[doc(alias = "RBX::PhysicsService::~PhysicsService()")]
// was: __ZN3RBX14PhysicsServiceD1Ev
pub fn stub_5f704c() -> ! {
    todo!("0x5f704c RBX::PhysicsService::~PhysicsService()")
}

// 0x5f7050 — __ZThn32_N3RBX14PhysicsServiceD0Ev
// type: void __fastcall(RBX::PhysicsService *__hidden this)
#[doc(alias = "__ZThn32_N3RBX14PhysicsServiceD0Ev")]
#[doc(alias = "non-virtual thunk to RBX::PhysicsService::~PhysicsService()")]
// was: __ZThn32_N3RBX14PhysicsServiceD0Ev
pub fn stub_5f7050() -> ! {
    todo!("0x5f7050 non-virtual thunk to RBX::PhysicsService::~PhysicsService()")
}

// 0x5f7058 — __ZThn36_N3RBX14PhysicsServiceD0Ev
// type: void __fastcall(RBX::PhysicsService *__hidden this)
#[doc(alias = "__ZThn36_N3RBX14PhysicsServiceD0Ev")]
#[doc(alias = "non-virtual thunk to RBX::PhysicsService::~PhysicsService()")]
// was: __ZThn36_N3RBX14PhysicsServiceD0Ev
pub fn stub_5f7058() -> ! {
    todo!("0x5f7058 non-virtual thunk to RBX::PhysicsService::~PhysicsService()")
}

// 0x5f7060 — __ZN3RBX14PhysicsServiceD2Ev
// type: void __fastcall(RBX::PhysicsService *__hidden this)
#[doc(alias = "__ZN3RBX14PhysicsServiceD2Ev")]
#[doc(alias = "RBX::PhysicsService::~PhysicsService()")]
// was: __ZN3RBX14PhysicsServiceD2Ev
pub fn stub_5f7060() -> ! {
    todo!("0x5f7060 RBX::PhysicsService::~PhysicsService()")
}

// 0x5f7410 — __ZThn32_N3RBX14PhysicsServiceD1Ev
// type: void __fastcall(RBX::PhysicsService *__hidden this)
#[doc(alias = "__ZThn32_N3RBX14PhysicsServiceD1Ev")]
#[doc(alias = "non-virtual thunk to RBX::PhysicsService::~PhysicsService()")]
// was: __ZThn32_N3RBX14PhysicsServiceD1Ev
pub fn stub_5f7410() -> ! {
    todo!("0x5f7410 non-virtual thunk to RBX::PhysicsService::~PhysicsService()")
}

// 0x5f7418 — __ZThn36_N3RBX14PhysicsServiceD1Ev
// type: void __fastcall(RBX::PhysicsService *__hidden this)
#[doc(alias = "__ZThn36_N3RBX14PhysicsServiceD1Ev")]
#[doc(alias = "non-virtual thunk to RBX::PhysicsService::~PhysicsService()")]
// was: __ZThn36_N3RBX14PhysicsServiceD1Ev
pub fn stub_5f7418() -> ! {
    todo!("0x5f7418 non-virtual thunk to RBX::PhysicsService::~PhysicsService()")
}

// 0x5f7420 — __ZN3RBX14PhysicsService17onServiceProviderEPNS_15ServiceProviderES2_
// type: _DWORD __fastcall(RBX::PhysicsService *__hidden this, RBX::ServiceProvider *, RBX::ServiceProvider *)
#[doc(alias = "__ZN3RBX14PhysicsService17onServiceProviderEPNS_15ServiceProviderES2_")]
#[doc(alias = "RBX::PhysicsService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// was: __ZN3RBX14PhysicsService17onServiceProviderEPNS_15ServiceProviderES2_
pub fn stub_5f7420() -> ! {
    todo!("0x5f7420 RBX::PhysicsService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")
}

// 0x5f7598 — __ZN3RBX14PhysicsService19onAssemblyPhysicsOnEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::PhysicsService *__hidden this, RBX::Primitive *)
#[doc(alias = "__ZN3RBX14PhysicsService19onAssemblyPhysicsOnEPNS_9PrimitiveE")]
#[doc(alias = "RBX::PhysicsService::onAssemblyPhysicsOn(RBX::Primitive *)")]
// was: __ZN3RBX14PhysicsService19onAssemblyPhysicsOnEPNS_9PrimitiveE
pub fn stub_5f7598() -> ! {
    todo!("0x5f7598 RBX::PhysicsService::onAssemblyPhysicsOn(RBX::Primitive *)")
}

// 0x5f788c — __ZN3RBX14PhysicsService20onAssemblyPhysicsOffEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::PhysicsService *__hidden this, RBX::Primitive *)
#[doc(alias = "__ZN3RBX14PhysicsService20onAssemblyPhysicsOffEPNS_9PrimitiveE")]
#[doc(alias = "RBX::PhysicsService::onAssemblyPhysicsOff(RBX::Primitive *)")]
// was: __ZN3RBX14PhysicsService20onAssemblyPhysicsOffEPNS_9PrimitiveE
pub fn stub_5f788c() -> ! {
    todo!("0x5f788c RBX::PhysicsService::onAssemblyPhysicsOff(RBX::Primitive *)")
}

// 0x5f7b48 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_14PhysicsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int(void)
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_14PhysicsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEEENS0_10connectionERKT_")]
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Primitive *)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>> const&)")]
// was: __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_14PhysicsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEEENS0_10connectionERKT_
pub fn stub_5f7b48() -> ! {
    todo!("0x5f7b48 rbx::signals::connection rbx::signals::signal<void ()(RBX::Primitive *)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>> const&)")
}

// 0x5f7bbc — __ZN3RBX9Intrusive3SetINS_12PartInstanceENS_14PhysicsServiceEE6insertERS2_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "__ZN3RBX9Intrusive3SetINS_12PartInstanceENS_14PhysicsServiceEE6insertERS2_")]
#[doc(alias = "RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::insert(RBX::PartInstance&)")]
// was: __ZN3RBX9Intrusive3SetINS_12PartInstanceENS_14PhysicsServiceEE6insertERS2_
pub fn stub_5f7bbc() -> ! {
    todo!("0x5f7bbc RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::insert(RBX::PartInstance&)")
}

// 0x5f7e64 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE6insertEPNS6_4slotE
// type: void __fastcall(int *, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE6insertEPNS6_4slotE")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::insert(rbx::signals::signal<void ()(RBX::Primitive *)>::slot *)")]
// was: __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE6insertEPNS6_4slotE
pub fn stub_5f7e64() -> ! {
    todo!("0x5f7e64 rbx::signals::signal<void ()(RBX::Primitive *)>::insert(rbx::signals::signal<void ()(RBX::Primitive *)>::slot *)")
}

// 0x5f8070 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotEEaSEPS9_
// type: int(void)
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotEEaSEPS9_")]
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Primitive *)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Primitive *)>::slot*)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotEEaSEPS9_
pub fn stub_5f8070() -> ! {
    todo!("0x5f8070 boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Primitive *)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Primitive *)>::slot*)")
}

// 0x5f8094 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotEEaSERKSA_
// type: int(void)
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotEEaSERKSA_")]
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Primitive *)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Primitive *)>::slot> const&)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotEEaSERKSA_
pub fn stub_5f8094() -> ! {
    todo!("0x5f8094 boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Primitive *)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Primitive *)>::slot> const&)")
}

// 0x5f80b8 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE22safe_static_init_mutexEv
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE22safe_static_init_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE22safe_static_init_mutexEv
pub fn stub_5f80b8() -> ! {
    todo!("0x5f80b8 rbx::signals::signal<void ()(RBX::Primitive *)>::safe_static_init_mutex(void)")
}

// 0x5f80bc — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE24safe_static_do_get_mutexEv
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE24safe_static_do_get_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE24safe_static_do_get_mutexEv
pub fn stub_5f80bc() -> ! {
    todo!("0x5f80bc rbx::signals::signal<void ()(RBX::Primitive *)>::safe_static_do_get_mutex(void)")
}

// 0x5f81b4 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_14PhysicsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEED1Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_14PhysicsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEED1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_14PhysicsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEED1Ev
pub fn stub_5f81b4() -> ! {
    todo!("0x5f81b4 rbx::signals::signal<void ()(RBX::Primitive *)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x5f81e0 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_14PhysicsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEED0Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_14PhysicsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEED0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_14PhysicsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEED0Ev
pub fn stub_5f81e0() -> ! {
    todo!("0x5f81e0 rbx::signals::signal<void ()(RBX::Primitive *)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x5f82b4 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot10disconnectEv
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot10disconnectEv")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::slot::disconnect(void)")]
// was: __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot10disconnectEv
pub fn stub_5f82b4() -> ! {
    todo!("0x5f82b4 rbx::signals::signal<void ()(RBX::Primitive *)>::slot::disconnect(void)")
}

// 0x5f83c4 — __ZNK3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot9connectedEv
#[doc(alias = "__ZNK3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot9connectedEv")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::slot::connected(void)const")]
// was: __ZNK3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot9connectedEv
pub fn stub_5f83c4() -> ! {
    todo!("0x5f83c4 rbx::signals::signal<void ()(RBX::Primitive *)>::slot::connected(void)const")
}

// 0x5f83d0 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_14PhysicsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_14PhysicsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Primitive *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>>,1,void ()(RBX::Primitive *)>::call(RBX::Primitive *)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_14PhysicsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_
pub fn stub_5f83d0() -> ! {
    todo!("0x5f83d0 rbx::callable<rbx::signals::signal<void ()(RBX::Primitive *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>>,1,void ()(RBX::Primitive *)>::call(RBX::Primitive *)")
}

// 0x5f83e4 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_14PhysicsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_14PhysicsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_")]
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::Primitive *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>>,1,void ()(RBX::Primitive *)>::call(RBX::Primitive *)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_14PhysicsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_
pub fn stub_5f83e4() -> ! {
    todo!("0x5f83e4 non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::Primitive *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>>,1,void ()(RBX::Primitive *)>::call(RBX::Primitive *)")
}

// 0x5f83f8 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX14PhysicsServiceEPNS4_9PrimitiveEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS7_EEvRT_
// type: int(void)
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX14PhysicsServiceEPNS4_9PrimitiveEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS7_EEvRT_")]
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>>::operator()<RBX::Primitive *>(RBX::Primitive * &)")]
// was: __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX14PhysicsServiceEPNS4_9PrimitiveEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS7_EEvRT_
pub fn stub_5f83f8() -> ! {
    todo!("0x5f83f8 void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>>::operator()<RBX::Primitive *>(RBX::Primitive * &)")
}

// 0x5f8410 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE6removeEPNS6_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE6removeEPNS6_4slotE")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::remove(rbx::signals::signal<void ()(RBX::Primitive *)>::slot *)")]
// was: __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE6removeEPNS6_4slotE
pub fn stub_5f8410() -> ! {
    todo!("0x5f8410 rbx::signals::signal<void ()(RBX::Primitive *)>::remove(rbx::signals::signal<void ()(RBX::Primitive *)>::slot *)")
}

// 0x5f8500 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot22safe_static_init_mutexEv
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot22safe_static_init_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::slot::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot22safe_static_init_mutexEv
pub fn stub_5f8500() -> ! {
    todo!("0x5f8500 rbx::signals::signal<void ()(RBX::Primitive *)>::slot::safe_static_init_mutex(void)")
}

// 0x5f8504 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot24safe_static_do_get_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::slot::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot24safe_static_do_get_mutexEv
pub fn stub_5f8504() -> ! {
    todo!("0x5f8504 rbx::signals::signal<void ()(RBX::Primitive *)>::slot::safe_static_do_get_mutex(void)")
}

// 0x5f85f4 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotD1Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotD1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotD1Ev
pub fn stub_5f85f4() -> ! {
    todo!("0x5f85f4 rbx::signals::signal<void ()(RBX::Primitive *)>::slot::~slot()")
}

// 0x5f8620 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotD0Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotD0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotD0Ev
pub fn stub_5f8620() -> ! {
    todo!("0x5f8620 rbx::signals::signal<void ()(RBX::Primitive *)>::slot::~slot()")
}

// 0x5f86f4 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_14PhysicsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_ED1Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_14PhysicsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_ED1Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Primitive *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>>,1,void ()(RBX::Primitive *)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_14PhysicsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_ED1Ev
pub fn stub_5f86f4() -> ! {
    todo!("0x5f86f4 rbx::callable<rbx::signals::signal<void ()(RBX::Primitive *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>>,1,void ()(RBX::Primitive *)>::~callable()")
}

// 0x5f8720 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_14PhysicsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_ED0Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_14PhysicsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_ED0Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Primitive *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>>,1,void ()(RBX::Primitive *)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_14PhysicsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_ED0Ev
pub fn stub_5f8720() -> ! {
    todo!("0x5f8720 rbx::callable<rbx::signals::signal<void ()(RBX::Primitive *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>>,1,void ()(RBX::Primitive *)>::~callable()")
}

// 0x5f87f4 — __GLOBAL__I_a_235
#[doc(alias = "__GLOBAL__I_a_235")]
#[doc(alias = "global constructor keyed to_a_235")]
// was: __GLOBAL__I_a_235
pub fn stub_5f87f4() -> ! {
    todo!("0x5f87f4 `global constructor keyed to'_a_235")
}

// 0x5f8a64 — __ZNK3RBX15PhysicsSettings20getShowAnchoredPartsEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
#[doc(alias = "__ZNK3RBX15PhysicsSettings20getShowAnchoredPartsEv")]
#[doc(alias = "RBX::PhysicsSettings::getShowAnchoredParts(void)const")]
// was: __ZNK3RBX15PhysicsSettings20getShowAnchoredPartsEv
pub fn stub_5f8a64() -> ! {
    todo!("0x5f8a64 RBX::PhysicsSettings::getShowAnchoredParts(void)const")
}

// 0x5f8a74 — __ZN3RBX15PhysicsSettings20setShowAnchoredPartsEb
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this, bool)
#[doc(alias = "__ZN3RBX15PhysicsSettings20setShowAnchoredPartsEb")]
#[doc(alias = "RBX::PhysicsSettings::setShowAnchoredParts(bool)")]
// was: __ZN3RBX15PhysicsSettings20setShowAnchoredPartsEb
pub fn stub_5f8a74() -> ! {
    todo!("0x5f8a74 RBX::PhysicsSettings::setShowAnchoredParts(bool)")
}

// 0x5f8aa4 — __ZNK3RBX15PhysicsSettings27getShowPartCoordinateFramesEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
#[doc(alias = "__ZNK3RBX15PhysicsSettings27getShowPartCoordinateFramesEv")]
#[doc(alias = "RBX::PhysicsSettings::getShowPartCoordinateFrames(void)const")]
// was: __ZNK3RBX15PhysicsSettings27getShowPartCoordinateFramesEv
pub fn stub_5f8aa4() -> ! {
    todo!("0x5f8aa4 RBX::PhysicsSettings::getShowPartCoordinateFrames(void)const")
}

// 0x5f8ab4 — __ZN3RBX15PhysicsSettings27setShowPartCoordinateFramesEb
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this, bool)
#[doc(alias = "__ZN3RBX15PhysicsSettings27setShowPartCoordinateFramesEb")]
#[doc(alias = "RBX::PhysicsSettings::setShowPartCoordinateFrames(bool)")]
// was: __ZN3RBX15PhysicsSettings27setShowPartCoordinateFramesEb
pub fn stub_5f8ab4() -> ! {
    todo!("0x5f8ab4 RBX::PhysicsSettings::setShowPartCoordinateFrames(bool)")
}

// 0x5f8ae4 — __ZNK3RBX15PhysicsSettings21getShowUnalignedPartsEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
#[doc(alias = "__ZNK3RBX15PhysicsSettings21getShowUnalignedPartsEv")]
#[doc(alias = "RBX::PhysicsSettings::getShowUnalignedParts(void)const")]
// was: __ZNK3RBX15PhysicsSettings21getShowUnalignedPartsEv
pub fn stub_5f8ae4() -> ! {
    todo!("0x5f8ae4 RBX::PhysicsSettings::getShowUnalignedParts(void)const")
}

// 0x5f8af4 — __ZN3RBX15PhysicsSettings21setShowUnalignedPartsEb
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this, bool)
#[doc(alias = "__ZN3RBX15PhysicsSettings21setShowUnalignedPartsEb")]
#[doc(alias = "RBX::PhysicsSettings::setShowUnalignedParts(bool)")]
// was: __ZN3RBX15PhysicsSettings21setShowUnalignedPartsEb
pub fn stub_5f8af4() -> ! {
    todo!("0x5f8af4 RBX::PhysicsSettings::setShowUnalignedParts(bool)")
}

// 0x5f8b24 — __ZNK3RBX15PhysicsSettings28getShowModelCoordinateFramesEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
#[doc(alias = "__ZNK3RBX15PhysicsSettings28getShowModelCoordinateFramesEv")]
#[doc(alias = "RBX::PhysicsSettings::getShowModelCoordinateFrames(void)const")]
// was: __ZNK3RBX15PhysicsSettings28getShowModelCoordinateFramesEv
pub fn stub_5f8b24() -> ! {
    todo!("0x5f8b24 RBX::PhysicsSettings::getShowModelCoordinateFrames(void)const")
}

// 0x5f8b34 — __ZN3RBX15PhysicsSettings28setShowModelCoordinateFramesEb
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this, bool)
#[doc(alias = "__ZN3RBX15PhysicsSettings28setShowModelCoordinateFramesEb")]
#[doc(alias = "RBX::PhysicsSettings::setShowModelCoordinateFrames(bool)")]
// was: __ZN3RBX15PhysicsSettings28setShowModelCoordinateFramesEb
pub fn stub_5f8b34() -> ! {
    todo!("0x5f8b34 RBX::PhysicsSettings::setShowModelCoordinateFrames(bool)")
}

// 0x5f8b64 — __ZNK3RBX15PhysicsSettings27getShowWorldCoordinateFrameEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
#[doc(alias = "__ZNK3RBX15PhysicsSettings27getShowWorldCoordinateFrameEv")]
#[doc(alias = "RBX::PhysicsSettings::getShowWorldCoordinateFrame(void)const")]
// was: __ZNK3RBX15PhysicsSettings27getShowWorldCoordinateFrameEv
pub fn stub_5f8b64() -> ! {
    todo!("0x5f8b64 RBX::PhysicsSettings::getShowWorldCoordinateFrame(void)const")
}

// 0x5f8b74 — __ZN3RBX15PhysicsSettings27setShowWorldCoordinateFrameEb
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this, bool)
#[doc(alias = "__ZN3RBX15PhysicsSettings27setShowWorldCoordinateFrameEb")]
#[doc(alias = "RBX::PhysicsSettings::setShowWorldCoordinateFrame(bool)")]
// was: __ZN3RBX15PhysicsSettings27setShowWorldCoordinateFrameEb
pub fn stub_5f8b74() -> ! {
    todo!("0x5f8b74 RBX::PhysicsSettings::setShowWorldCoordinateFrame(bool)")
}

// 0x5f8ba4 — __ZNK3RBX15PhysicsSettings21getShowEPhysicsOwnersEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
#[doc(alias = "__ZNK3RBX15PhysicsSettings21getShowEPhysicsOwnersEv")]
#[doc(alias = "RBX::PhysicsSettings::getShowEPhysicsOwners(void)const")]
// was: __ZNK3RBX15PhysicsSettings21getShowEPhysicsOwnersEv
pub fn stub_5f8ba4() -> ! {
    todo!("0x5f8ba4 RBX::PhysicsSettings::getShowEPhysicsOwners(void)const")
}

// 0x5f8bb4 — __ZN3RBX15PhysicsSettings21setShowEPhysicsOwnersEb
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this, bool)
#[doc(alias = "__ZN3RBX15PhysicsSettings21setShowEPhysicsOwnersEb")]
#[doc(alias = "RBX::PhysicsSettings::setShowEPhysicsOwners(bool)")]
// was: __ZN3RBX15PhysicsSettings21setShowEPhysicsOwnersEb
pub fn stub_5f8bb4() -> ! {
    todo!("0x5f8bb4 RBX::PhysicsSettings::setShowEPhysicsOwners(bool)")
}

// 0x5f8be4 — __ZNK3RBX15PhysicsSettings22getShowEPhysicsRegionsEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
#[doc(alias = "__ZNK3RBX15PhysicsSettings22getShowEPhysicsRegionsEv")]
#[doc(alias = "RBX::PhysicsSettings::getShowEPhysicsRegions(void)const")]
// was: __ZNK3RBX15PhysicsSettings22getShowEPhysicsRegionsEv
pub fn stub_5f8be4() -> ! {
    todo!("0x5f8be4 RBX::PhysicsSettings::getShowEPhysicsRegions(void)const")
}

// 0x5f8bf4 — __ZN3RBX15PhysicsSettings22setShowEPhysicsRegionsEb
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this, bool)
#[doc(alias = "__ZN3RBX15PhysicsSettings22setShowEPhysicsRegionsEb")]
#[doc(alias = "RBX::PhysicsSettings::setShowEPhysicsRegions(bool)")]
// was: __ZN3RBX15PhysicsSettings22setShowEPhysicsRegionsEb
pub fn stub_5f8bf4() -> ! {
    todo!("0x5f8bf4 RBX::PhysicsSettings::setShowEPhysicsRegions(bool)")
}

// 0x5f8c24 — __ZNK3RBX15PhysicsSettings22getHighlightAwakePartsEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
#[doc(alias = "__ZNK3RBX15PhysicsSettings22getHighlightAwakePartsEv")]
#[doc(alias = "RBX::PhysicsSettings::getHighlightAwakeParts(void)const")]
// was: __ZNK3RBX15PhysicsSettings22getHighlightAwakePartsEv
pub fn stub_5f8c24() -> ! {
    todo!("0x5f8c24 RBX::PhysicsSettings::getHighlightAwakeParts(void)const")
}

// 0x5f8c34 — __ZN3RBX15PhysicsSettings22setHighlightAwakePartsEb
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this, bool)
#[doc(alias = "__ZN3RBX15PhysicsSettings22setHighlightAwakePartsEb")]
#[doc(alias = "RBX::PhysicsSettings::setHighlightAwakeParts(bool)")]
// was: __ZN3RBX15PhysicsSettings22setHighlightAwakePartsEb
pub fn stub_5f8c34() -> ! {
    todo!("0x5f8c34 RBX::PhysicsSettings::setHighlightAwakeParts(bool)")
}

// 0x5f8c64 — __ZNK3RBX15PhysicsSettings16getShowBodyTypesEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
#[doc(alias = "__ZNK3RBX15PhysicsSettings16getShowBodyTypesEv")]
#[doc(alias = "RBX::PhysicsSettings::getShowBodyTypes(void)const")]
// was: __ZNK3RBX15PhysicsSettings16getShowBodyTypesEv
pub fn stub_5f8c64() -> ! {
    todo!("0x5f8c64 RBX::PhysicsSettings::getShowBodyTypes(void)const")
}

// 0x5f8c74 — __ZN3RBX15PhysicsSettings16setShowBodyTypesEb
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this, bool)
#[doc(alias = "__ZN3RBX15PhysicsSettings16setShowBodyTypesEb")]
#[doc(alias = "RBX::PhysicsSettings::setShowBodyTypes(bool)")]
// was: __ZN3RBX15PhysicsSettings16setShowBodyTypesEb
pub fn stub_5f8c74() -> ! {
    todo!("0x5f8c74 RBX::PhysicsSettings::setShowBodyTypes(bool)")
}

// 0x5f8ca4 — __ZNK3RBX15PhysicsSettings17getShowReceiveAgeEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
#[doc(alias = "__ZNK3RBX15PhysicsSettings17getShowReceiveAgeEv")]
#[doc(alias = "RBX::PhysicsSettings::getShowReceiveAge(void)const")]
// was: __ZNK3RBX15PhysicsSettings17getShowReceiveAgeEv
pub fn stub_5f8ca4() -> ! {
    todo!("0x5f8ca4 RBX::PhysicsSettings::getShowReceiveAge(void)const")
}

// 0x5f8cb4 — __ZN3RBX15PhysicsSettings17setShowReceiveAgeEb
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this, bool)
#[doc(alias = "__ZN3RBX15PhysicsSettings17setShowReceiveAgeEb")]
#[doc(alias = "RBX::PhysicsSettings::setShowReceiveAge(bool)")]
// was: __ZN3RBX15PhysicsSettings17setShowReceiveAgeEb
pub fn stub_5f8cb4() -> ! {
    todo!("0x5f8cb4 RBX::PhysicsSettings::setShowReceiveAge(bool)")
}

// 0x5f8ce4 — __ZNK3RBX15PhysicsSettings20getShowContactPointsEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
#[doc(alias = "__ZNK3RBX15PhysicsSettings20getShowContactPointsEv")]
#[doc(alias = "RBX::PhysicsSettings::getShowContactPoints(void)const")]
// was: __ZNK3RBX15PhysicsSettings20getShowContactPointsEv
pub fn stub_5f8ce4() -> ! {
    todo!("0x5f8ce4 RBX::PhysicsSettings::getShowContactPoints(void)const")
}

// 0x5f8cf4 — __ZN3RBX15PhysicsSettings20setShowContactPointsEb
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this, bool)
#[doc(alias = "__ZN3RBX15PhysicsSettings20setShowContactPointsEb")]
#[doc(alias = "RBX::PhysicsSettings::setShowContactPoints(bool)")]
// was: __ZN3RBX15PhysicsSettings20setShowContactPointsEb
pub fn stub_5f8cf4() -> ! {
    todo!("0x5f8cf4 RBX::PhysicsSettings::setShowContactPoints(bool)")
}
