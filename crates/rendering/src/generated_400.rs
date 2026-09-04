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
// IDA 0x5f45a8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_5f45a8() {
}

// 0x5f45b0 — __ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataclEPv
#[doc(alias = "__ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataclEPv")]
#[doc(alias = "boost::thread_specific_ptr<RBX::Security::Context>::delete_data::operator()(void *)")]
// was: __ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataclEPv
// IDA 0x5f45b0: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f45b0() {
}

// 0x5f45c0 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataENS0_14do_heap_deleteIS7_EEED0Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataENS0_14do_heap_deleteIS7_EEED0Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::Security::Context>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::Security::Context>::delete_data>>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataENS0_14do_heap_deleteIS7_EEED0Ev
// IDA 0x5f45c0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5f45c0() {
}

// 0x5f45c8 — __ZN3RBX6RbxRayD0Ev
// type: void __fastcall(RBX::RbxRay *__hidden this)
#[doc(alias = "__ZN3RBX6RbxRayD0Ev")]
#[doc(alias = "RBX::RbxRay::~RbxRay()")]
// was: __ZN3RBX6RbxRayD0Ev
// IDA 0x5f45c8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5f45c8() {
}

// 0x5f45d0 — __ZNK3RBX17copy_on_write_ptrISt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EEEdeEv
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZNK3RBX17copy_on_write_ptrISt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EEEdeEv")]
#[doc(alias = "RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>::operator*(void)const")]
// was: __ZNK3RBX17copy_on_write_ptrISt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EEEdeEv
// IDA 0x5f45d0: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f45d0() {
}

// 0x5f4628 — __ZN3RBX9AllocatorINS_12PartInstance20OnDemandPartInstanceEEnwEm
// type: int(void)
#[doc(alias = "__ZN3RBX9AllocatorINS_12PartInstance20OnDemandPartInstanceEEnwEm")]
#[doc(alias = "RBX::Allocator<RBX::PartInstance::OnDemandPartInstance>::operator new(unsigned long)")]
// was: __ZN3RBX9AllocatorINS_12PartInstance20OnDemandPartInstanceEEnwEm
// IDA 0x5f4628: operator new/delete pair → Rust allocator/global alloc; no-op glue.
pub fn stub_5f4628() {
}

// 0x5f4698 — __ZN5boost14singleton_poolIN3RBX12PartInstance20OnDemandPartInstanceELj200ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
// type: int(void)
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX12PartInstance20OnDemandPartInstanceELj200ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")]
#[doc(alias = "boost::singleton_pool<RBX::PartInstance::OnDemandPartInstance,200u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// was: __ZN5boost14singleton_poolIN3RBX12PartInstance20OnDemandPartInstanceELj200ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
// IDA 0x5f4698: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f4698() {
}

// 0x5f46d0 — __ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE7destroyEv
// type: int(void)
#[doc(alias = "__ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE7destroyEv")]
#[doc(alias = "boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::destroy(void)")]
// was: __ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE7destroyEv
// IDA 0x5f46d0: 15 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f46d0() {
}

// 0x5f46f4 — __ZN3RBX18InterpolatedCFrameC2Ev
// type: _DWORD __fastcall(RBX::InterpolatedCFrame *__hidden this)
#[doc(alias = "__ZN3RBX18InterpolatedCFrameC2Ev")]
#[doc(alias = "RBX::InterpolatedCFrame::InterpolatedCFrame(void)")]
// was: __ZN3RBX18InterpolatedCFrameC2Ev
// IDA 0x5f46f4: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f46f4() {
}

// 0x5f4744 — __ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE8allocateEm
// type: int(void)
#[doc(alias = "__ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE8allocateEm")]
#[doc(alias = "boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::allocate(unsigned long)")]
// was: __ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE8allocateEm
// IDA 0x5f4744: 101 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f4744() {
}

// 0x5f4868 — __ZN5boost15throw_exceptionISt12length_errorEEvRKT_
// type: int(void)
#[doc(alias = "__ZN5boost15throw_exceptionISt12length_errorEEvRKT_")]
#[doc(alias = "void boost::throw_exception<std::length_error>(std::length_error const&)")]
// was: __ZN5boost15throw_exceptionISt12length_errorEEvRKT_
// IDA 0x5f4868: 76 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f4868() {
}

// 0x5f4948 — __ZN5boost16exception_detail19error_info_injectorISt12length_errorED2Ev
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "__ZN5boost16exception_detail19error_info_injectorISt12length_errorED2Ev")]
#[doc(alias = "boost::exception_detail::error_info_injector<std::length_error>::~error_info_injector()")]
// was: __ZN5boost16exception_detail19error_info_injectorISt12length_errorED2Ev
// IDA 0x5f4948: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5f4948() {
}

// 0x5f4a00 — __ZThn8_N5boost16exception_detail19error_info_injectorISt12length_errorED1Ev
#[doc(alias = "__ZThn8_N5boost16exception_detail19error_info_injectorISt12length_errorED1Ev")]
#[doc(alias = "non-virtual thunk to boost::exception_detail::error_info_injector<std::length_error>::~error_info_injector()")]
// was: __ZThn8_N5boost16exception_detail19error_info_injectorISt12length_errorED1Ev
// IDA 0x5f4a00: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5f4a00() {
}

// 0x5f4a08 — __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEED1Ev
#[doc(alias = "__ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEED1Ev")]
#[doc(alias = "non-virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::~clone_impl()")]
// was: __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEED1Ev
// IDA 0x5f4a08: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5f4a08() {
}

// 0x5f4a10 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEED1Ev
#[doc(alias = "__ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEED1Ev")]
#[doc(alias = "virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::~clone_impl()")]
// was: __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEED1Ev
// IDA 0x5f4a10: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5f4a10() {
}

// 0x5f4a20 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEE5cloneEv
#[doc(alias = "__ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEE5cloneEv")]
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::clone(void)const")]
// was: __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEE5cloneEv
// IDA 0x5f4a20: 67 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f4a20() {
}

// 0x5f4ae0 — __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEED0Ev
#[doc(alias = "__ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEED0Ev")]
#[doc(alias = "non-virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::~clone_impl()")]
// was: __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEED0Ev
// IDA 0x5f4ae0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5f4ae0() {
}

// 0x5f4af8 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEE5cloneEv
#[doc(alias = "__ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEE5cloneEv")]
#[doc(alias = "virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::clone(void)const")]
// was: __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEE5cloneEv
// IDA 0x5f4af8: 4 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f4af8() {
}

// 0x5f4b08 — __ZN5boost16exception_detail19error_info_injectorISt12length_errorED0Ev
#[doc(alias = "__ZN5boost16exception_detail19error_info_injectorISt12length_errorED0Ev")]
#[doc(alias = "boost::exception_detail::error_info_injector<std::length_error>::~error_info_injector()")]
// was: __ZN5boost16exception_detail19error_info_injectorISt12length_errorED0Ev
// IDA 0x5f4b08: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5f4b08() {
}

// 0x5f4b20 — __ZN5boost16exception_detail19error_info_injectorISt12length_errorEC2ERKS2_
#[doc(alias = "__ZN5boost16exception_detail19error_info_injectorISt12length_errorEC2ERKS2_")]
#[doc(alias = "boost::exception_detail::error_info_injector<std::length_error>::error_info_injector(std::length_error const&)")]
// was: __ZN5boost16exception_detail19error_info_injectorISt12length_errorEC2ERKS2_
// IDA 0x5f4b20: 80 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f4b20() {
}

// 0x5f4c08 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEEC1ERKS4_
#[doc(alias = "__ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEEC1ERKS4_")]
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::clone_impl(boost::exception_detail::error_info_injector<std::length_error> const&)")]
// was: __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEEC1ERKS4_
// IDA 0x5f4c08: 126 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f4c08() {
}

// 0x5f4d78 — __ZN3RBX16OnDemandInstanceD2Ev
// type: void __fastcall(RBX::OnDemandInstance *__hidden this)
#[doc(alias = "__ZN3RBX16OnDemandInstanceD2Ev")]
#[doc(alias = "RBX::OnDemandInstance::~OnDemandInstance()")]
// was: __ZN3RBX16OnDemandInstanceD2Ev
// IDA 0x5f4d78: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5f4d78() {
}

// 0x5f4fdc — __ZN3RBX18OnDemandPVInstanceD1Ev
// type: void __fastcall(RBX::OnDemandPVInstance *__hidden this)
#[doc(alias = "__ZN3RBX18OnDemandPVInstanceD1Ev")]
#[doc(alias = "RBX::OnDemandPVInstance::~OnDemandPVInstance()")]
// was: __ZN3RBX18OnDemandPVInstanceD1Ev
// IDA 0x5f4fdc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5f4fdc() {
}

// 0x5f5008 — __ZN3RBX18OnDemandPVInstanceD0Ev
// type: void __fastcall(RBX::OnDemandPVInstance *__hidden this)
#[doc(alias = "__ZN3RBX18OnDemandPVInstanceD0Ev")]
#[doc(alias = "RBX::OnDemandPVInstance::~OnDemandPVInstance()")]
// was: __ZN3RBX18OnDemandPVInstanceD0Ev
// IDA 0x5f5008: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5f5008() {
}

// 0x5f50dc — __ZN3RBX16OnDemandInstanceD1Ev
// type: void __fastcall(RBX::OnDemandInstance *__hidden this)
#[doc(alias = "__ZN3RBX16OnDemandInstanceD1Ev")]
#[doc(alias = "RBX::OnDemandInstance::~OnDemandInstance()")]
// was: __ZN3RBX16OnDemandInstanceD1Ev
// IDA 0x5f50dc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5f50dc() {
}

// 0x5f50e0 — __ZN3RBX16OnDemandInstanceD0Ev
// type: void __fastcall(RBX::OnDemandInstance *__hidden this)
#[doc(alias = "__ZN3RBX16OnDemandInstanceD0Ev")]
#[doc(alias = "RBX::OnDemandInstance::~OnDemandInstance()")]
// was: __ZN3RBX16OnDemandInstanceD0Ev
// IDA 0x5f50e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5f50e0() {
}

// 0x5f5198 — __ZN3rbx7signals6signalIFvbEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "__ZN3rbx7signals6signalIFvbEE13disconnectAllEv")]
#[doc(alias = "rbx::signals::signal<void ()(bool)>::disconnectAll(void)")]
// was: __ZN3rbx7signals6signalIFvbEE13disconnectAllEv
// IDA 0x5f5198: 136 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f5198() {
}

// 0x5f5310 — __ZN3RBX16OnDemandInstanceC2Ev
// type: _DWORD __fastcall(RBX::OnDemandInstance *__hidden this)
#[doc(alias = "__ZN3RBX16OnDemandInstanceC2Ev")]
#[doc(alias = "RBX::OnDemandInstance::OnDemandInstance(void)")]
// was: __ZN3RBX16OnDemandInstanceC2Ev
// IDA 0x5f5310: 230 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f5310() {
}

// 0x5f5564 — __ZN3RBX9AllocatorINS_18OnDemandPVInstanceEEC2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX9AllocatorINS_18OnDemandPVInstanceEEC2Ev")]
#[doc(alias = "RBX::Allocator<RBX::OnDemandPVInstance>::Allocator(void)")]
// was: __ZN3RBX9AllocatorINS_18OnDemandPVInstanceEEC2Ev
// IDA 0x5f5564: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f5564() {
}

// 0x5f55c8 — __ZN3RBX9AllocatorINS_18OnDemandPVInstanceEE13releaseMemoryEv
#[doc(alias = "__ZN3RBX9AllocatorINS_18OnDemandPVInstanceEE13releaseMemoryEv")]
#[doc(alias = "RBX::Allocator<RBX::OnDemandPVInstance>::releaseMemory(void)")]
// was: __ZN3RBX9AllocatorINS_18OnDemandPVInstanceEE13releaseMemoryEv
// IDA 0x5f55c8: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f55c8() {
}

// 0x5f55e4 — __ZN5boost14singleton_poolIN3RBX18OnDemandPVInstanceELj24ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
// type: int(void)
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX18OnDemandPVInstanceELj24ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
#[doc(alias = "boost::singleton_pool<RBX::OnDemandPVInstance,24u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// was: __ZN5boost14singleton_poolIN3RBX18OnDemandPVInstanceELj24ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
// IDA 0x5f55e4: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f55e4() {
}

// 0x5f5618 — __ZN3RBX9AllocatorINS_16OnDemandInstanceEE13releaseMemoryEv
#[doc(alias = "__ZN3RBX9AllocatorINS_16OnDemandInstanceEE13releaseMemoryEv")]
#[doc(alias = "RBX::Allocator<RBX::OnDemandInstance>::releaseMemory(void)")]
// was: __ZN3RBX9AllocatorINS_16OnDemandInstanceEE13releaseMemoryEv
// IDA 0x5f5618: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f5618() {
}

// 0x5f5634 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8MaterialEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// type: int(void)
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8MaterialEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")]
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Material>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Material>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Material>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Material>> *)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8MaterialEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// IDA 0x5f5634: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f5634() {
}

// 0x5f565c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12PartInstance10FormFactorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int(void)
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12PartInstance10FormFactorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PartInstance::FormFactor>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PartInstance::FormFactor>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PartInstance::FormFactor>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::PartInstance::FormFactor>> *)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12PartInstance10FormFactorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// IDA 0x5f565c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f565c() {
}

// 0x5f5684 — __GLOBAL__I_a_233
#[doc(alias = "__GLOBAL__I_a_233")]
#[doc(alias = "global constructor keyed to_a_233")]
// was: __GLOBAL__I_a_233
// IDA 0x5f5684: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_5f5684() {
}

// 0x5f6800 — __ZN3RBX19PhysicsInstructionsC1Ev
// type: _DWORD __fastcall(RBX::PhysicsInstructions *__hidden this)
#[doc(alias = "__ZN3RBX19PhysicsInstructionsC1Ev")]
#[doc(alias = "RBX::PhysicsInstructions::PhysicsInstructions(void)")]
// was: __ZN3RBX19PhysicsInstructionsC1Ev
// IDA 0x5f6800: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5f6800() {
}

// 0x5f6804 — __ZN3RBX19PhysicsInstructionsC2Ev
// type: _DWORD __fastcall(RBX::PhysicsInstructions *__hidden this)
#[doc(alias = "__ZN3RBX19PhysicsInstructionsC2Ev")]
#[doc(alias = "RBX::PhysicsInstructions::PhysicsInstructions(void)")]
// was: __ZN3RBX19PhysicsInstructionsC2Ev
// IDA 0x5f6804: 122 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f6804() {
}

// 0x5f6948 — __ZN3RBX19PhysicsInstructions25dPhysicsServerDutyPercentEv
// type: _DWORD __fastcall(RBX::PhysicsInstructions *__hidden this)
#[doc(alias = "__ZN3RBX19PhysicsInstructions25dPhysicsServerDutyPercentEv")]
#[doc(alias = "RBX::PhysicsInstructions::dPhysicsServerDutyPercent(void)")]
// was: __ZN3RBX19PhysicsInstructions25dPhysicsServerDutyPercentEv
// IDA 0x5f6948: 11 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f6948() {
}

// 0x5f6968 — sub_5F6968
#[doc(alias = "sub_5F6968")]
#[doc(alias = "sub_5F6968")]
// was: sub_5F6968
// IDA 0x5f6968: 8 insns (ADDS..SUBS). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f6968() {
}

// 0x5f6978 — __ZN3RBX19PhysicsInstructions22changeSimulationRadiusEPNS_7Network6PlayerEf
// type: _DWORD __fastcall(RBX::PhysicsInstructions *__hidden this, RBX::Network::Player *, float)
#[doc(alias = "__ZN3RBX19PhysicsInstructions22changeSimulationRadiusEPNS_7Network6PlayerEf")]
#[doc(alias = "RBX::PhysicsInstructions::changeSimulationRadius(RBX::Network::Player *,float)")]
// was: __ZN3RBX19PhysicsInstructions22changeSimulationRadiusEPNS_7Network6PlayerEf
// IDA 0x5f6978: 37 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f6978() {
}

// 0x5f69ec — __ZN3RBX19PhysicsInstructions25changeMaxSimulationRadiusEPNS_7Network6PlayerEf
// type: _DWORD __fastcall(RBX::PhysicsInstructions *__hidden this, RBX::Network::Player *, float)
#[doc(alias = "__ZN3RBX19PhysicsInstructions25changeMaxSimulationRadiusEPNS_7Network6PlayerEf")]
#[doc(alias = "RBX::PhysicsInstructions::changeMaxSimulationRadius(RBX::Network::Player *,float)")]
// was: __ZN3RBX19PhysicsInstructions25changeMaxSimulationRadiusEPNS_7Network6PlayerEf
// IDA 0x5f69ec: 35 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f69ec() {
}

// 0x5f6a60 — __ZN3RBX19PhysicsInstructions25dPhysicsClientDutyPercentEv
// type: _DWORD __fastcall(RBX::PhysicsInstructions *__hidden this)
#[doc(alias = "__ZN3RBX19PhysicsInstructions25dPhysicsClientDutyPercentEv")]
#[doc(alias = "RBX::PhysicsInstructions::dPhysicsClientDutyPercent(void)")]
// was: __ZN3RBX19PhysicsInstructions25dPhysicsClientDutyPercentEv
// IDA 0x5f6a60: 3 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f6a60() {
}

// 0x5f6a78 — __ZN3RBX19PhysicsInstructions34dPhysicsClientEThrottleDutyPercentEv
// type: _DWORD __fastcall(RBX::PhysicsInstructions *__hidden this)
#[doc(alias = "__ZN3RBX19PhysicsInstructions34dPhysicsClientEThrottleDutyPercentEv")]
#[doc(alias = "RBX::PhysicsInstructions::dPhysicsClientEThrottleDutyPercent(void)")]
// was: __ZN3RBX19PhysicsInstructions34dPhysicsClientEThrottleDutyPercentEv
// IDA 0x5f6a78: 3 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f6a78() {
}

// 0x5f6a90 — __ZN3RBX19PhysicsInstructions12setThrottlesEPNS_7Network6PlayerEPNS_9WorkspaceEdd
// type: _DWORD __fastcall(RBX::PhysicsInstructions *__hidden this, RBX::Network::Player *, RBX::Workspace *, double, double)
#[doc(alias = "__ZN3RBX19PhysicsInstructions12setThrottlesEPNS_7Network6PlayerEPNS_9WorkspaceEdd")]
#[doc(alias = "RBX::PhysicsInstructions::setThrottles(RBX::Network::Player *,RBX::Workspace *,double,double)")]
// was: __ZN3RBX19PhysicsInstructions12setThrottlesEPNS_7Network6PlayerEPNS_9WorkspaceEdd
// IDA 0x5f6a90: 172 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f6a90() {
}

// 0x5f6cf8 — __ZNSt6vectorIdSaIdEE6resizeEmd
// type: int(void)
#[doc(alias = "__ZNSt6vectorIdSaIdEE6resizeEmd")]
#[doc(alias = "std::vector<double,std::allocator<double>>::resize(unsigned long,double)")]
// was: __ZNSt6vectorIdSaIdEE6resizeEmd
// IDA 0x5f6cf8: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f6cf8() {
}

// 0x5f6d3c — __GLOBAL__I_a_234
#[doc(alias = "__GLOBAL__I_a_234")]
#[doc(alias = "global constructor keyed to_a_234")]
// was: __GLOBAL__I_a_234
// IDA 0x5f6d3c: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_5f6d3c() {
}

// 0x5f6fac — __ZN3RBX14PhysicsServiceD0Ev
// type: void __fastcall(RBX::PhysicsService *__hidden this)
#[doc(alias = "__ZN3RBX14PhysicsServiceD0Ev")]
#[doc(alias = "RBX::PhysicsService::~PhysicsService()")]
// was: __ZN3RBX14PhysicsServiceD0Ev
// IDA 0x5f6fac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5f6fac() {
}

// 0x5f704c — __ZN3RBX14PhysicsServiceD1Ev
// type: void __fastcall(RBX::PhysicsService *__hidden this)
#[doc(alias = "__ZN3RBX14PhysicsServiceD1Ev")]
#[doc(alias = "RBX::PhysicsService::~PhysicsService()")]
// was: __ZN3RBX14PhysicsServiceD1Ev
// IDA 0x5f704c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5f704c() {
}

// 0x5f7050 — __ZThn32_N3RBX14PhysicsServiceD0Ev
// type: void __fastcall(RBX::PhysicsService *__hidden this)
#[doc(alias = "__ZThn32_N3RBX14PhysicsServiceD0Ev")]
#[doc(alias = "non-virtual thunk to RBX::PhysicsService::~PhysicsService()")]
// was: __ZThn32_N3RBX14PhysicsServiceD0Ev
// IDA 0x5f7050: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5f7050() {
}

// 0x5f7058 — __ZThn36_N3RBX14PhysicsServiceD0Ev
// type: void __fastcall(RBX::PhysicsService *__hidden this)
#[doc(alias = "__ZThn36_N3RBX14PhysicsServiceD0Ev")]
#[doc(alias = "non-virtual thunk to RBX::PhysicsService::~PhysicsService()")]
// was: __ZThn36_N3RBX14PhysicsServiceD0Ev
// IDA 0x5f7058: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5f7058() {
}

// 0x5f7060 — __ZN3RBX14PhysicsServiceD2Ev
// type: void __fastcall(RBX::PhysicsService *__hidden this)
#[doc(alias = "__ZN3RBX14PhysicsServiceD2Ev")]
#[doc(alias = "RBX::PhysicsService::~PhysicsService()")]
// was: __ZN3RBX14PhysicsServiceD2Ev
// IDA 0x5f7060: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5f7060() {
}

// 0x5f7410 — __ZThn32_N3RBX14PhysicsServiceD1Ev
// type: void __fastcall(RBX::PhysicsService *__hidden this)
#[doc(alias = "__ZThn32_N3RBX14PhysicsServiceD1Ev")]
#[doc(alias = "non-virtual thunk to RBX::PhysicsService::~PhysicsService()")]
// was: __ZThn32_N3RBX14PhysicsServiceD1Ev
// IDA 0x5f7410: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5f7410() {
}

// 0x5f7418 — __ZThn36_N3RBX14PhysicsServiceD1Ev
// type: void __fastcall(RBX::PhysicsService *__hidden this)
#[doc(alias = "__ZThn36_N3RBX14PhysicsServiceD1Ev")]
#[doc(alias = "non-virtual thunk to RBX::PhysicsService::~PhysicsService()")]
// was: __ZThn36_N3RBX14PhysicsServiceD1Ev
// IDA 0x5f7418: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5f7418() {
}

// 0x5f7420 — __ZN3RBX14PhysicsService17onServiceProviderEPNS_15ServiceProviderES2_
// type: _DWORD __fastcall(RBX::PhysicsService *__hidden this, RBX::ServiceProvider *, RBX::ServiceProvider *)
#[doc(alias = "__ZN3RBX14PhysicsService17onServiceProviderEPNS_15ServiceProviderES2_")]
#[doc(alias = "RBX::PhysicsService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// was: __ZN3RBX14PhysicsService17onServiceProviderEPNS_15ServiceProviderES2_
// IDA 0x5f7420: 136 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f7420() {
}

// 0x5f7598 — __ZN3RBX14PhysicsService19onAssemblyPhysicsOnEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::PhysicsService *__hidden this, RBX::Primitive *)
#[doc(alias = "__ZN3RBX14PhysicsService19onAssemblyPhysicsOnEPNS_9PrimitiveE")]
#[doc(alias = "RBX::PhysicsService::onAssemblyPhysicsOn(RBX::Primitive *)")]
// was: __ZN3RBX14PhysicsService19onAssemblyPhysicsOnEPNS_9PrimitiveE
// IDA 0x5f7598: 274 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f7598() {
}

// 0x5f788c — __ZN3RBX14PhysicsService20onAssemblyPhysicsOffEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::PhysicsService *__hidden this, RBX::Primitive *)
#[doc(alias = "__ZN3RBX14PhysicsService20onAssemblyPhysicsOffEPNS_9PrimitiveE")]
#[doc(alias = "RBX::PhysicsService::onAssemblyPhysicsOff(RBX::Primitive *)")]
// was: __ZN3RBX14PhysicsService20onAssemblyPhysicsOffEPNS_9PrimitiveE
// IDA 0x5f788c: 253 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f788c() {
}

// 0x5f7b48 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_14PhysicsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int(void)
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_14PhysicsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEEENS0_10connectionERKT_")]
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Primitive *)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>> const&)")]
// was: __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_14PhysicsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEEENS0_10connectionERKT_
// IDA 0x5f7b48: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f7b48() {
}

// 0x5f7bbc — __ZN3RBX9Intrusive3SetINS_12PartInstanceENS_14PhysicsServiceEE6insertERS2_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "__ZN3RBX9Intrusive3SetINS_12PartInstanceENS_14PhysicsServiceEE6insertERS2_")]
#[doc(alias = "RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::insert(RBX::PartInstance&)")]
// was: __ZN3RBX9Intrusive3SetINS_12PartInstanceENS_14PhysicsServiceEE6insertERS2_
// IDA 0x5f7bbc: 242 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f7bbc() {
}

// 0x5f7e64 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE6insertEPNS6_4slotE
// type: void __fastcall(int *, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE6insertEPNS6_4slotE")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::insert(rbx::signals::signal<void ()(RBX::Primitive *)>::slot *)")]
// was: __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE6insertEPNS6_4slotE
// IDA 0x5f7e64: 184 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f7e64() {
}

// 0x5f8070 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotEEaSEPS9_
// type: int(void)
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotEEaSEPS9_")]
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Primitive *)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Primitive *)>::slot*)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotEEaSEPS9_
// IDA 0x5f8070: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f8070() {
}

// 0x5f8094 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotEEaSERKSA_
// type: int(void)
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotEEaSERKSA_")]
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Primitive *)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Primitive *)>::slot> const&)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotEEaSERKSA_
// IDA 0x5f8094: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f8094() {
}

// 0x5f80b8 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE22safe_static_init_mutexEv
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE22safe_static_init_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE22safe_static_init_mutexEv
// IDA 0x5f80b8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5f80b8() {
}

// 0x5f80bc — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE24safe_static_do_get_mutexEv
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE24safe_static_do_get_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE24safe_static_do_get_mutexEv
// IDA 0x5f80bc: 84 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f80bc() {
}

// 0x5f81b4 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_14PhysicsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEED1Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_14PhysicsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEED1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_14PhysicsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEED1Ev
// IDA 0x5f81b4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5f81b4() {
}

// 0x5f81e0 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_14PhysicsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEED0Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_14PhysicsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEED0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_14PhysicsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEED0Ev
// IDA 0x5f81e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5f81e0() {
}

// 0x5f82b4 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot10disconnectEv
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot10disconnectEv")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::slot::disconnect(void)")]
// was: __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot10disconnectEv
// IDA 0x5f82b4: 93 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f82b4() {
}

// 0x5f83c4 — __ZNK3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot9connectedEv
#[doc(alias = "__ZNK3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot9connectedEv")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::slot::connected(void)const")]
// was: __ZNK3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot9connectedEv
// IDA 0x5f83c4: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f83c4() {
}

// 0x5f83d0 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_14PhysicsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_14PhysicsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Primitive *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>>,1,void ()(RBX::Primitive *)>::call(RBX::Primitive *)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_14PhysicsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_
// IDA 0x5f83d0: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f83d0() {
}

// 0x5f83e4 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_14PhysicsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_14PhysicsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_")]
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::Primitive *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>>,1,void ()(RBX::Primitive *)>::call(RBX::Primitive *)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_14PhysicsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_
// IDA 0x5f83e4: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f83e4() {
}

// 0x5f83f8 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX14PhysicsServiceEPNS4_9PrimitiveEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS7_EEvRT_
// type: int(void)
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX14PhysicsServiceEPNS4_9PrimitiveEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS7_EEvRT_")]
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>>::operator()<RBX::Primitive *>(RBX::Primitive * &)")]
// was: __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX14PhysicsServiceEPNS4_9PrimitiveEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS7_EEvRT_
// IDA 0x5f83f8: 9 insns (LDM.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f83f8() {
}

// 0x5f8410 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE6removeEPNS6_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE6removeEPNS6_4slotE")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::remove(rbx::signals::signal<void ()(RBX::Primitive *)>::slot *)")]
// was: __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE6removeEPNS6_4slotE
// IDA 0x5f8410: 78 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f8410() {
}

// 0x5f8500 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot22safe_static_init_mutexEv
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot22safe_static_init_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::slot::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot22safe_static_init_mutexEv
// IDA 0x5f8500: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5f8500() {
}

// 0x5f8504 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot24safe_static_do_get_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::slot::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot24safe_static_do_get_mutexEv
// IDA 0x5f8504: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f8504() {
}

// 0x5f85f4 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotD1Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotD1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotD1Ev
// IDA 0x5f85f4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5f85f4() {
}

// 0x5f8620 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotD0Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotD0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slotD0Ev
// IDA 0x5f8620: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5f8620() {
}

// 0x5f86f4 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_14PhysicsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_ED1Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_14PhysicsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_ED1Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Primitive *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>>,1,void ()(RBX::Primitive *)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_14PhysicsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_ED1Ev
// IDA 0x5f86f4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5f86f4() {
}

// 0x5f8720 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_14PhysicsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_ED0Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_14PhysicsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_ED0Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Primitive *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::PhysicsService,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::PhysicsService*>,boost::arg<1>>>,1,void ()(RBX::Primitive *)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_14PhysicsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_ED0Ev
// IDA 0x5f8720: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5f8720() {
}

// 0x5f87f4 — __GLOBAL__I_a_235
#[doc(alias = "__GLOBAL__I_a_235")]
#[doc(alias = "global constructor keyed to_a_235")]
// was: __GLOBAL__I_a_235
// IDA 0x5f87f4: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_5f87f4() {
}

// 0x5f8a64 — __ZNK3RBX15PhysicsSettings20getShowAnchoredPartsEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
#[doc(alias = "__ZNK3RBX15PhysicsSettings20getShowAnchoredPartsEv")]
#[doc(alias = "RBX::PhysicsSettings::getShowAnchoredParts(void)const")]
// was: __ZNK3RBX15PhysicsSettings20getShowAnchoredPartsEv
// IDA 0x5f8a64: 5 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f8a64() {
}

// 0x5f8a74 — __ZN3RBX15PhysicsSettings20setShowAnchoredPartsEb
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this, bool)
#[doc(alias = "__ZN3RBX15PhysicsSettings20setShowAnchoredPartsEb")]
#[doc(alias = "RBX::PhysicsSettings::setShowAnchoredParts(bool)")]
// was: __ZN3RBX15PhysicsSettings20setShowAnchoredPartsEb
// IDA 0x5f8a74: 15 insns (MOV..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f8a74() {
}

// 0x5f8aa4 — __ZNK3RBX15PhysicsSettings27getShowPartCoordinateFramesEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
#[doc(alias = "__ZNK3RBX15PhysicsSettings27getShowPartCoordinateFramesEv")]
#[doc(alias = "RBX::PhysicsSettings::getShowPartCoordinateFrames(void)const")]
// was: __ZNK3RBX15PhysicsSettings27getShowPartCoordinateFramesEv
// IDA 0x5f8aa4: 5 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f8aa4() {
}

// 0x5f8ab4 — __ZN3RBX15PhysicsSettings27setShowPartCoordinateFramesEb
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this, bool)
#[doc(alias = "__ZN3RBX15PhysicsSettings27setShowPartCoordinateFramesEb")]
#[doc(alias = "RBX::PhysicsSettings::setShowPartCoordinateFrames(bool)")]
// was: __ZN3RBX15PhysicsSettings27setShowPartCoordinateFramesEb
// IDA 0x5f8ab4: 15 insns (MOV..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f8ab4() {
}

// 0x5f8ae4 — __ZNK3RBX15PhysicsSettings21getShowUnalignedPartsEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
#[doc(alias = "__ZNK3RBX15PhysicsSettings21getShowUnalignedPartsEv")]
#[doc(alias = "RBX::PhysicsSettings::getShowUnalignedParts(void)const")]
// was: __ZNK3RBX15PhysicsSettings21getShowUnalignedPartsEv
// IDA 0x5f8ae4: 5 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f8ae4() {
}

// 0x5f8af4 — __ZN3RBX15PhysicsSettings21setShowUnalignedPartsEb
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this, bool)
#[doc(alias = "__ZN3RBX15PhysicsSettings21setShowUnalignedPartsEb")]
#[doc(alias = "RBX::PhysicsSettings::setShowUnalignedParts(bool)")]
// was: __ZN3RBX15PhysicsSettings21setShowUnalignedPartsEb
// IDA 0x5f8af4: 15 insns (MOV..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f8af4() {
}

// 0x5f8b24 — __ZNK3RBX15PhysicsSettings28getShowModelCoordinateFramesEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
#[doc(alias = "__ZNK3RBX15PhysicsSettings28getShowModelCoordinateFramesEv")]
#[doc(alias = "RBX::PhysicsSettings::getShowModelCoordinateFrames(void)const")]
// was: __ZNK3RBX15PhysicsSettings28getShowModelCoordinateFramesEv
// IDA 0x5f8b24: 5 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f8b24() {
}

// 0x5f8b34 — __ZN3RBX15PhysicsSettings28setShowModelCoordinateFramesEb
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this, bool)
#[doc(alias = "__ZN3RBX15PhysicsSettings28setShowModelCoordinateFramesEb")]
#[doc(alias = "RBX::PhysicsSettings::setShowModelCoordinateFrames(bool)")]
// was: __ZN3RBX15PhysicsSettings28setShowModelCoordinateFramesEb
// IDA 0x5f8b34: 15 insns (MOV..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f8b34() {
}

// 0x5f8b64 — __ZNK3RBX15PhysicsSettings27getShowWorldCoordinateFrameEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
#[doc(alias = "__ZNK3RBX15PhysicsSettings27getShowWorldCoordinateFrameEv")]
#[doc(alias = "RBX::PhysicsSettings::getShowWorldCoordinateFrame(void)const")]
// was: __ZNK3RBX15PhysicsSettings27getShowWorldCoordinateFrameEv
// IDA 0x5f8b64: 5 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f8b64() {
}

// 0x5f8b74 — __ZN3RBX15PhysicsSettings27setShowWorldCoordinateFrameEb
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this, bool)
#[doc(alias = "__ZN3RBX15PhysicsSettings27setShowWorldCoordinateFrameEb")]
#[doc(alias = "RBX::PhysicsSettings::setShowWorldCoordinateFrame(bool)")]
// was: __ZN3RBX15PhysicsSettings27setShowWorldCoordinateFrameEb
// IDA 0x5f8b74: 15 insns (MOV..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f8b74() {
}

// 0x5f8ba4 — __ZNK3RBX15PhysicsSettings21getShowEPhysicsOwnersEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
#[doc(alias = "__ZNK3RBX15PhysicsSettings21getShowEPhysicsOwnersEv")]
#[doc(alias = "RBX::PhysicsSettings::getShowEPhysicsOwners(void)const")]
// was: __ZNK3RBX15PhysicsSettings21getShowEPhysicsOwnersEv
// IDA 0x5f8ba4: 5 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f8ba4() {
}

// 0x5f8bb4 — __ZN3RBX15PhysicsSettings21setShowEPhysicsOwnersEb
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this, bool)
#[doc(alias = "__ZN3RBX15PhysicsSettings21setShowEPhysicsOwnersEb")]
#[doc(alias = "RBX::PhysicsSettings::setShowEPhysicsOwners(bool)")]
// was: __ZN3RBX15PhysicsSettings21setShowEPhysicsOwnersEb
// IDA 0x5f8bb4: 15 insns (MOV..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f8bb4() {
}

// 0x5f8be4 — __ZNK3RBX15PhysicsSettings22getShowEPhysicsRegionsEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
#[doc(alias = "__ZNK3RBX15PhysicsSettings22getShowEPhysicsRegionsEv")]
#[doc(alias = "RBX::PhysicsSettings::getShowEPhysicsRegions(void)const")]
// was: __ZNK3RBX15PhysicsSettings22getShowEPhysicsRegionsEv
// IDA 0x5f8be4: 5 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f8be4() {
}

// 0x5f8bf4 — __ZN3RBX15PhysicsSettings22setShowEPhysicsRegionsEb
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this, bool)
#[doc(alias = "__ZN3RBX15PhysicsSettings22setShowEPhysicsRegionsEb")]
#[doc(alias = "RBX::PhysicsSettings::setShowEPhysicsRegions(bool)")]
// was: __ZN3RBX15PhysicsSettings22setShowEPhysicsRegionsEb
// IDA 0x5f8bf4: 15 insns (MOV..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f8bf4() {
}

// 0x5f8c24 — __ZNK3RBX15PhysicsSettings22getHighlightAwakePartsEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
#[doc(alias = "__ZNK3RBX15PhysicsSettings22getHighlightAwakePartsEv")]
#[doc(alias = "RBX::PhysicsSettings::getHighlightAwakeParts(void)const")]
// was: __ZNK3RBX15PhysicsSettings22getHighlightAwakePartsEv
// IDA 0x5f8c24: 5 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f8c24() {
}

// 0x5f8c34 — __ZN3RBX15PhysicsSettings22setHighlightAwakePartsEb
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this, bool)
#[doc(alias = "__ZN3RBX15PhysicsSettings22setHighlightAwakePartsEb")]
#[doc(alias = "RBX::PhysicsSettings::setHighlightAwakeParts(bool)")]
// was: __ZN3RBX15PhysicsSettings22setHighlightAwakePartsEb
// IDA 0x5f8c34: 15 insns (MOV..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f8c34() {
}

// 0x5f8c64 — __ZNK3RBX15PhysicsSettings16getShowBodyTypesEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
#[doc(alias = "__ZNK3RBX15PhysicsSettings16getShowBodyTypesEv")]
#[doc(alias = "RBX::PhysicsSettings::getShowBodyTypes(void)const")]
// was: __ZNK3RBX15PhysicsSettings16getShowBodyTypesEv
// IDA 0x5f8c64: 5 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f8c64() {
}

// 0x5f8c74 — __ZN3RBX15PhysicsSettings16setShowBodyTypesEb
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this, bool)
#[doc(alias = "__ZN3RBX15PhysicsSettings16setShowBodyTypesEb")]
#[doc(alias = "RBX::PhysicsSettings::setShowBodyTypes(bool)")]
// was: __ZN3RBX15PhysicsSettings16setShowBodyTypesEb
// IDA 0x5f8c74: 15 insns (MOV..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f8c74() {
}

// 0x5f8ca4 — __ZNK3RBX15PhysicsSettings17getShowReceiveAgeEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
#[doc(alias = "__ZNK3RBX15PhysicsSettings17getShowReceiveAgeEv")]
#[doc(alias = "RBX::PhysicsSettings::getShowReceiveAge(void)const")]
// was: __ZNK3RBX15PhysicsSettings17getShowReceiveAgeEv
// IDA 0x5f8ca4: 5 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f8ca4() {
}

// 0x5f8cb4 — __ZN3RBX15PhysicsSettings17setShowReceiveAgeEb
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this, bool)
#[doc(alias = "__ZN3RBX15PhysicsSettings17setShowReceiveAgeEb")]
#[doc(alias = "RBX::PhysicsSettings::setShowReceiveAge(bool)")]
// was: __ZN3RBX15PhysicsSettings17setShowReceiveAgeEb
// IDA 0x5f8cb4: 15 insns (MOV..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f8cb4() {
}

// 0x5f8ce4 — __ZNK3RBX15PhysicsSettings20getShowContactPointsEv
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this)
#[doc(alias = "__ZNK3RBX15PhysicsSettings20getShowContactPointsEv")]
#[doc(alias = "RBX::PhysicsSettings::getShowContactPoints(void)const")]
// was: __ZNK3RBX15PhysicsSettings20getShowContactPointsEv
// IDA 0x5f8ce4: 5 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f8ce4() {
}

// 0x5f8cf4 — __ZN3RBX15PhysicsSettings20setShowContactPointsEb
// type: _DWORD __fastcall(RBX::PhysicsSettings *__hidden this, bool)
#[doc(alias = "__ZN3RBX15PhysicsSettings20setShowContactPointsEb")]
#[doc(alias = "RBX::PhysicsSettings::setShowContactPoints(bool)")]
// was: __ZN3RBX15PhysicsSettings20setShowContactPointsEb
// IDA 0x5f8cf4: 15 insns (MOV..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f8cf4() {
}
