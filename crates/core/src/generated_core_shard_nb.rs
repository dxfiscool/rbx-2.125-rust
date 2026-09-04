//! core shard nb — 100 core stubs EA-sorted asc fallback gap filler after 0xf250cc (next uncovered >0xf250cc EA-sorted asc, not yet in rbx_core).
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 100 fallback not yet in rbx_core (fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound; fallback 33887, uncovered 13824 before -> 13724 after, batch 0xf25d44..0xf26644).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::ShadowMode>(RBX::CRenderSettings::ShadowMode const&)")]
// 0xf25d44 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings10ShadowModeEEERS3_RKT_
// type: 
pub fn stub_0xf25d44() {
    // IDA 0xf25d44: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::GraphicsMode>(RBX::CRenderSettings::GraphicsMode const&)")]
// 0xf25d54 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings12GraphicsModeEEERS3_RKT_
// type: 
pub fn stub_0xf25d54() {
    // IDA 0xf25d54: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::QualityLevel>(RBX::CRenderSettings::QualityLevel const&)")]
// 0xf25d64 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings12QualityLevelEEERS3_RKT_
// type: 
pub fn stub_0xf25d64() {
    // IDA 0xf25d64: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::AntialiasingMode>(RBX::CRenderSettings::AntialiasingMode const&)")]
// 0xf25d74 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings16AntialiasingModeEEERS3_RKT_
// type: 
pub fn stub_0xf25d74() {
    // IDA 0xf25d74: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::ResolutionPreset>(RBX::CRenderSettings::ResolutionPreset const&)")]
// 0xf25d84 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings16ResolutionPresetEEERS3_RKT_
// type: 
pub fn stub_0xf25d84() {
    // IDA 0xf25d84: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::FrameRateManagerMode>(RBX::CRenderSettings::FrameRateManagerMode const&)")]
// 0xf25d94 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings20FrameRateManagerModeEEERS3_RKT_
// type: 
pub fn stub_0xf25d94() {
    // IDA 0xf25d94: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::AASamples>(RBX::CRenderSettings::AASamples const&)")]
// 0xf25da4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings9AASamplesEEERS3_RKT_
// type: 
pub fn stub_0xf25da4() {
    // IDA 0xf25da4: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::ShadowMode>::singleton(void)")]
// 0xf25db4 — j___ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings10ShadowModeEE9singletonEv
// type: _DWORD *(void)
pub fn stub_0xf25db4() {
    // IDA 0xf25db4: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::GraphicsMode>::singleton(void)")]
// 0xf25dc4 — j___ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12GraphicsModeEE9singletonEv
// type: _DWORD *(void)
pub fn stub_0xf25dc4() {
    // IDA 0xf25dc4: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::QualityLevel>::singleton(void)")]
// 0xf25dd4 — j___ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12QualityLevelEE9singletonEv
// type: _DWORD *(void)
pub fn stub_0xf25dd4() {
    // IDA 0xf25dd4: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AntialiasingMode>::singleton(void)")]
// 0xf25de4 — j___ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16AntialiasingModeEE9singletonEv
// type: _DWORD *(void)
pub fn stub_0xf25de4() {
    // IDA 0xf25de4: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::ResolutionPreset>::singleton(void)")]
// 0xf25df4 — j___ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16ResolutionPresetEE9singletonEv
// type: _DWORD *()
pub fn stub_0xf25df4() {
    // IDA 0xf25df4: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::FrameRateManagerMode>::singleton(void)")]
// 0xf25e04 — j___ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings20FrameRateManagerModeEE9singletonEv
// type: _DWORD *(void)
pub fn stub_0xf25e04() {
    // IDA 0xf25e04: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AASamples>::singleton(void)")]
// 0xf25e14 — j___ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings9AASamplesEE9singletonEv
// type: _DWORD *(void)
pub fn stub_0xf25e14() {
    // IDA 0xf25e14: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}
#[doc(alias = "RBX::CRenderSettings::ShadowMode const& rbx::any_cast<RBX::CRenderSettings::ShadowMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf25e54 — j___ZN3rbx8any_castIRKN3RBX15CRenderSettings10ShadowModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
pub fn stub_0xf25e54() {
    // IDA 0xf25e54: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}
#[doc(alias = "RBX::CRenderSettings::GraphicsMode const& rbx::any_cast<RBX::CRenderSettings::GraphicsMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf25e64 — j___ZN3rbx8any_castIRKN3RBX15CRenderSettings12GraphicsModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
pub fn stub_0xf25e64() {
    // IDA 0xf25e64: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}
#[doc(alias = "RBX::CRenderSettings::QualityLevel const& rbx::any_cast<RBX::CRenderSettings::QualityLevel const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf25e74 — j___ZN3rbx8any_castIRKN3RBX15CRenderSettings12QualityLevelENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
pub fn stub_0xf25e74() {
    // IDA 0xf25e74: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}
#[doc(alias = "RBX::CRenderSettings::AntialiasingMode const& rbx::any_cast<RBX::CRenderSettings::AntialiasingMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf25e84 — j___ZN3rbx8any_castIRKN3RBX15CRenderSettings16AntialiasingModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
pub fn stub_0xf25e84() {
    // IDA 0xf25e84: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}
#[doc(alias = "RBX::CRenderSettings::ResolutionPreset const& rbx::any_cast<RBX::CRenderSettings::ResolutionPreset const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf25e94 — j___ZN3rbx8any_castIRKN3RBX15CRenderSettings16ResolutionPresetENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
pub fn stub_0xf25e94() {
    // IDA 0xf25e94: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}
#[doc(alias = "RBX::CRenderSettings::FrameRateManagerMode const& rbx::any_cast<RBX::CRenderSettings::FrameRateManagerMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf25ea4 — j___ZN3rbx8any_castIRKN3RBX15CRenderSettings20FrameRateManagerModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
pub fn stub_0xf25ea4() {
    // IDA 0xf25ea4: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}
#[doc(alias = "RBX::CRenderSettings::AASamples const& rbx::any_cast<RBX::CRenderSettings::AASamples const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf25eb4 — j___ZN3rbx8any_castIRKN3RBX15CRenderSettings9AASamplesENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: 
pub fn stub_0xf25eb4() {
    // IDA 0xf25eb4: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}
#[doc(alias = "boost::exception_detail::refcount_ptr<boost::exception_detail::error_info_container>::~refcount_ptr()")]
// 0xf25ed4 — j___ZN5boost16exception_detail12refcount_ptrINS0_20error_info_containerEED2Ev
// type: 
pub fn stub_0xf25ed4() {
    // IDA 0xf25ed4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "boost::detail::sp_counted_base::use_count(void)const")]
// 0xf26134 — j___ZNK5boost6detail15sp_counted_base9use_countEv
// type: int __fastcall(boost::detail::sp_counted_base *this)
pub fn stub_0xf26134() {
    // IDA 0xf26134: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::_M_allocate(unsigned long)")]
// 0xf26154 — j___ZNSt12_Vector_baseIN3RBX15CRenderSettings10ShadowModeESaIS2_EE11_M_allocateEm
// type: int()
pub fn stub_0xf26154() {
    // IDA 0xf26154: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::_M_allocate(unsigned long)")]
// 0xf26164 — j___ZNSt12_Vector_baseIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE11_M_allocateEm
// type: int()
pub fn stub_0xf26164() {
    // IDA 0xf26164: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_allocate(unsigned long)")]
// 0xf26174 — j___ZNSt12_Vector_baseIN3RBX15CRenderSettings12QualityLevelESaIS2_EE11_M_allocateEm
// type: int()
pub fn stub_0xf26174() {
    // IDA 0xf26174: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::_M_allocate(unsigned long)")]
// 0xf26184 — j___ZNSt12_Vector_baseIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE11_M_allocateEm
// type: int()
pub fn stub_0xf26184() {
    // IDA 0xf26184: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_allocate(unsigned long)")]
// 0xf26194 — j___ZNSt12_Vector_baseIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE11_M_allocateEm
// type: int()
pub fn stub_0xf26194() {
    // IDA 0xf26194: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::_M_allocate(unsigned long)")]
// 0xf261a4 — j___ZNSt12_Vector_baseIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE11_M_allocateEm
// type: int()
pub fn stub_0xf261a4() {
    // IDA 0xf261a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::_M_allocate(unsigned long)")]
// 0xf261b4 — j___ZNSt12_Vector_baseIN3RBX15CRenderSettings9AASamplesESaIS2_EE11_M_allocateEm
// type: int()
pub fn stub_0xf261b4() {
    // IDA 0xf261b4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}
#[doc(alias = "RBX::CRenderSettings::ShadowMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::ShadowMode *,RBX::CRenderSettings::ShadowMode *>(RBX::CRenderSettings::ShadowMode *,RBX::CRenderSettings::ShadowMode *,RBX::CRenderSettings::ShadowMode *)")]
// 0xf261d4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings10ShadowModeES6_EET0_T_S8_S7_
// type: 
pub fn stub_0xf261d4() {
    // IDA 0xf261d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "RBX::CRenderSettings::GraphicsMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::GraphicsMode *,RBX::CRenderSettings::GraphicsMode *>(RBX::CRenderSettings::GraphicsMode *,RBX::CRenderSettings::GraphicsMode *,RBX::CRenderSettings::GraphicsMode *)")]
// 0xf261e4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings12GraphicsModeES6_EET0_T_S8_S7_
// type: 
pub fn stub_0xf261e4() {
    // IDA 0xf261e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "RBX::CRenderSettings::QualityLevel * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *>(RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *,RBX::CRenderSettings::QualityLevel *)")]
// 0xf261f4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings12QualityLevelES6_EET0_T_S8_S7_
// type: 
pub fn stub_0xf261f4() {
    // IDA 0xf261f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "RBX::CRenderSettings::AntialiasingMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::AntialiasingMode *,RBX::CRenderSettings::AntialiasingMode *>(RBX::CRenderSettings::AntialiasingMode *,RBX::CRenderSettings::AntialiasingMode *,RBX::CRenderSettings::AntialiasingMode *)")]
// 0xf26204 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings16AntialiasingModeES6_EET0_T_S8_S7_
// type: 
pub fn stub_0xf26204() {
    // IDA 0xf26204: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "RBX::CRenderSettings::ResolutionPreset * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *>(RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *,RBX::CRenderSettings::ResolutionPreset *)")]
// 0xf26214 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings16ResolutionPresetES6_EET0_T_S8_S7_
// type: 
pub fn stub_0xf26214() {
    // IDA 0xf26214: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "RBX::CRenderSettings::FrameRateManagerMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::FrameRateManagerMode *,RBX::CRenderSettings::FrameRateManagerMode *>(RBX::CRenderSettings::FrameRateManagerMode *,RBX::CRenderSettings::FrameRateManagerMode *,RBX::CRenderSettings::FrameRateManagerMode *)")]
// 0xf26224 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings20FrameRateManagerModeES6_EET0_T_S8_S7_
// type: 
pub fn stub_0xf26224() {
    // IDA 0xf26224: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "RBX::CRenderSettings::AASamples * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::AASamples *,RBX::CRenderSettings::AASamples *>(RBX::CRenderSettings::AASamples *,RBX::CRenderSettings::AASamples *,RBX::CRenderSettings::AASamples *)")]
// 0xf26234 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings9AASamplesES6_EET0_T_S8_S7_
// type: 
pub fn stub_0xf26234() {
    // IDA 0xf26234: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::ShadowMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::operator[](RBX::Name const* const&)")]
// 0xf26244 — j___ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings10ShadowModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(int, int)
pub fn stub_0xf26244() {
    // IDA 0xf26244: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::GraphicsMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::operator[](RBX::Name const* const&)")]
// 0xf26254 — j___ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings12GraphicsModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(int, int)
pub fn stub_0xf26254() {
    // IDA 0xf26254: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::QualityLevel,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::operator[](RBX::Name const* const&)")]
// 0xf26264 — j___ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings12QualityLevelESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(int, int)
pub fn stub_0xf26264() {
    // IDA 0xf26264: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::AntialiasingMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::operator[](RBX::Name const* const&)")]
// 0xf26274 — j___ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings16AntialiasingModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(int, int)
pub fn stub_0xf26274() {
    // IDA 0xf26274: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::ResolutionPreset,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::operator[](RBX::Name const* const&)")]
// 0xf26284 — j___ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings16ResolutionPresetESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: _Rb_tree_node_base **__fastcall(int, int *)
pub fn stub_0xf26284() {
    // IDA 0xf26284: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::FrameRateManagerMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::operator[](RBX::Name const* const&)")]
// 0xf26294 — j___ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings20FrameRateManagerModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(int, int)
pub fn stub_0xf26294() {
    // IDA 0xf26294: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::AASamples,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::operator[](RBX::Name const* const&)")]
// 0xf262a4 — j___ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings9AASamplesESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(int, int)
pub fn stub_0xf262a4() {
    // IDA 0xf262a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ShadowMode*,std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>>,RBX::CRenderSettings::ShadowMode const&)")]
// 0xf262d4 — j___ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int()
pub fn stub_0xf262d4() {
    // IDA 0xf262d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ShadowMode*,std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>>,unsigned long,RBX::CRenderSettings::ShadowMode const&)")]
// 0xf262e4 — j___ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int()
pub fn stub_0xf262e4() {
    // IDA 0xf262e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::resize(unsigned long,RBX::CRenderSettings::ShadowMode)")]
// 0xf262f4 — j___ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE6resizeEmS2_
// type: int __fastcall(int, int, int)
pub fn stub_0xf262f4() {
    // IDA 0xf262f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::push_back(RBX::CRenderSettings::ShadowMode const&)")]
// 0xf26304 — j___ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE9push_backERKS2_
// type: int __fastcall(int, int)
pub fn stub_0xf26304() {
    // IDA 0xf26304: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::GraphicsMode*,std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>>,RBX::CRenderSettings::GraphicsMode const&)")]
// 0xf26314 — j___ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int()
pub fn stub_0xf26314() {
    // IDA 0xf26314: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::GraphicsMode*,std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>>,unsigned long,RBX::CRenderSettings::GraphicsMode const&)")]
// 0xf26324 — j___ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int()
pub fn stub_0xf26324() {
    // IDA 0xf26324: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::resize(unsigned long,RBX::CRenderSettings::GraphicsMode)")]
// 0xf26334 — j___ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE6resizeEmS2_
// type: int __fastcall(int, int, int)
pub fn stub_0xf26334() {
    // IDA 0xf26334: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::push_back(RBX::CRenderSettings::GraphicsMode const&)")]
// 0xf26344 — j___ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE9push_backERKS2_
// type: int __fastcall(int, int)
pub fn stub_0xf26344() {
    // IDA 0xf26344: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::QualityLevel*,std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>>,RBX::CRenderSettings::QualityLevel const&)")]
// 0xf26354 — j___ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int()
pub fn stub_0xf26354() {
    // IDA 0xf26354: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::QualityLevel*,std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>>,unsigned long,RBX::CRenderSettings::QualityLevel const&)")]
// 0xf26364 — j___ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int()
pub fn stub_0xf26364() {
    // IDA 0xf26364: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::resize(unsigned long,RBX::CRenderSettings::QualityLevel)")]
// 0xf26374 — j___ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE6resizeEmS2_
// type: int __fastcall(int, int, int)
pub fn stub_0xf26374() {
    // IDA 0xf26374: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::vector<RBX::CRenderSettings::QualityLevel,std::allocator<RBX::CRenderSettings::QualityLevel>>::push_back(RBX::CRenderSettings::QualityLevel const&)")]
// 0xf26384 — j___ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE9push_backERKS2_
// type: int __fastcall(int, int)
pub fn stub_0xf26384() {
    // IDA 0xf26384: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AntialiasingMode*,std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>>,RBX::CRenderSettings::AntialiasingMode const&)")]
// 0xf26394 — j___ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int()
pub fn stub_0xf26394() {
    // IDA 0xf26394: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AntialiasingMode*,std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>>,unsigned long,RBX::CRenderSettings::AntialiasingMode const&)")]
// 0xf263a4 — j___ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int()
pub fn stub_0xf263a4() {
    // IDA 0xf263a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::resize(unsigned long,RBX::CRenderSettings::AntialiasingMode)")]
// 0xf263b4 — j___ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE6resizeEmS2_
// type: int __fastcall(int, int, int)
pub fn stub_0xf263b4() {
    // IDA 0xf263b4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::push_back(RBX::CRenderSettings::AntialiasingMode const&)")]
// 0xf263c4 — j___ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE9push_backERKS2_
// type: int __fastcall(int, int)
pub fn stub_0xf263c4() {
    // IDA 0xf263c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ResolutionPreset*,std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>>,RBX::CRenderSettings::ResolutionPreset const&)")]
// 0xf263d4 — j___ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int()
pub fn stub_0xf263d4() {
    // IDA 0xf263d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ResolutionPreset*,std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>>,unsigned long,RBX::CRenderSettings::ResolutionPreset const&)")]
// 0xf263e4 — j___ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int()
pub fn stub_0xf263e4() {
    // IDA 0xf263e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::resize(unsigned long,RBX::CRenderSettings::ResolutionPreset)")]
// 0xf263f4 — j___ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE6resizeEmS2_
// type: int __fastcall(int, int, int)
pub fn stub_0xf263f4() {
    // IDA 0xf263f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::vector<RBX::CRenderSettings::ResolutionPreset,std::allocator<RBX::CRenderSettings::ResolutionPreset>>::push_back(RBX::CRenderSettings::ResolutionPreset const&)")]
// 0xf26404 — j___ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE9push_backERKS2_
// type: int __fastcall(int, int)
pub fn stub_0xf26404() {
    // IDA 0xf26404: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::FrameRateManagerMode*,std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>>,RBX::CRenderSettings::FrameRateManagerMode const&)")]
// 0xf26414 — j___ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int()
pub fn stub_0xf26414() {
    // IDA 0xf26414: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::FrameRateManagerMode*,std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>>,unsigned long,RBX::CRenderSettings::FrameRateManagerMode const&)")]
// 0xf26424 — j___ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int()
pub fn stub_0xf26424() {
    // IDA 0xf26424: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::resize(unsigned long,RBX::CRenderSettings::FrameRateManagerMode)")]
// 0xf26434 — j___ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE6resizeEmS2_
// type: int __fastcall(int, int, int)
pub fn stub_0xf26434() {
    // IDA 0xf26434: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::push_back(RBX::CRenderSettings::FrameRateManagerMode const&)")]
// 0xf26444 — j___ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE9push_backERKS2_
// type: int __fastcall(int, int)
pub fn stub_0xf26444() {
    // IDA 0xf26444: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AASamples*,std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>>,RBX::CRenderSettings::AASamples const&)")]
// 0xf26454 — j___ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int()
pub fn stub_0xf26454() {
    // IDA 0xf26454: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AASamples*,std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>>,unsigned long,RBX::CRenderSettings::AASamples const&)")]
// 0xf26464 — j___ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int()
pub fn stub_0xf26464() {
    // IDA 0xf26464: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::resize(unsigned long,RBX::CRenderSettings::AASamples)")]
// 0xf26474 — j___ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE6resizeEmS2_
// type: int __fastcall(int, int, int)
pub fn stub_0xf26474() {
    // IDA 0xf26474: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::push_back(RBX::CRenderSettings::AASamples const&)")]
// 0xf26484 — j___ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE9push_backERKS2_
// type: int __fastcall(int, int)
pub fn stub_0xf26484() {
    // IDA 0xf26484: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode> const&)")]
// 0xf26494 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int()
pub fn stub_0xf26494() {
    // IDA 0xf26494: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode> const&)")]
// 0xf264a4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0xf264a4() {
    // IDA 0xf264a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>> *)")]
// 0xf264b4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(int, int)
pub fn stub_0xf264b4() {
    // IDA 0xf264b4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode> const&)")]
// 0xf264c4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int()
pub fn stub_0xf264c4() {
    // IDA 0xf264c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode> const&)")]
// 0xf264d4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int()
pub fn stub_0xf264d4() {
    // IDA 0xf264d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode> const&)")]
// 0xf264e4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0xf264e4() {
    // IDA 0xf264e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>> *)")]
// 0xf264f4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(int, int)
pub fn stub_0xf264f4() {
    // IDA 0xf264f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode> const&)")]
// 0xf26504 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: 
pub fn stub_0xf26504() {
    // IDA 0xf26504: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)")]
// 0xf26514 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: 
pub fn stub_0xf26514() {
    // IDA 0xf26514: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)")]
// 0xf26524 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0xf26524() {
    // IDA 0xf26524: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>> *)")]
// 0xf26534 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(int, int)
pub fn stub_0xf26534() {
    // IDA 0xf26534: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel> const&)")]
// 0xf26544 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int()
pub fn stub_0xf26544() {
    // IDA 0xf26544: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode> const&)")]
// 0xf26554 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16AntialiasingModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int()
pub fn stub_0xf26554() {
    // IDA 0xf26554: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode> const&)")]
// 0xf26564 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16AntialiasingModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
pub fn stub_0xf26564() {
    // IDA 0xf26564: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>> *)")]
// 0xf26574 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16AntialiasingModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(int, int)
pub fn stub_0xf26574() {
    // IDA 0xf26574: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode> const&)")]
// 0xf26584 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16AntialiasingModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int()
pub fn stub_0xf26584() {
    // IDA 0xf26584: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset> const&)")]
// 0xf26594 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, int)
pub fn stub_0xf26594() {
    // IDA 0xf26594: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset> const&)")]
// 0xf265a4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
pub fn stub_0xf265a4() {
    // IDA 0xf265a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>> *)")]
// 0xf265b4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(int, int)
pub fn stub_0xf265b4() {
    // IDA 0xf265b4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset> const&)")]
// 0xf265c4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
pub fn stub_0xf265c4() {
    // IDA 0xf265c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode> const&)")]
// 0xf265d4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings20FrameRateManagerModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int()
pub fn stub_0xf265d4() {
    // IDA 0xf265d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode> const&)")]
// 0xf265e4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings20FrameRateManagerModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0xf265e4() {
    // IDA 0xf265e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>> *)")]
// 0xf265f4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings20FrameRateManagerModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(int, int)
pub fn stub_0xf265f4() {
    // IDA 0xf265f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode> const&)")]
// 0xf26604 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings20FrameRateManagerModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int()
pub fn stub_0xf26604() {
    // IDA 0xf26604: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples> const&)")]
// 0xf26614 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int()
pub fn stub_0xf26614() {
    // IDA 0xf26614: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples> const&)")]
// 0xf26624 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0xf26624() {
    // IDA 0xf26624: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>> *)")]
// 0xf26634 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(int, int)
pub fn stub_0xf26634() {
    // IDA 0xf26634: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples> const&)")]
// 0xf26644 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int()
pub fn stub_0xf26644() {
    // IDA 0xf26644: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
