// Auto-generated skeletons for rbx-script — filler EA-sorted ascending earliest gap (next 100)
// Filter: Lua|Script|Yield|lua (5401 filtered, all already stubbed globally) — filler EA-sorted ascending
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x483410..0x49fc84 | existing ~9391 -> ~9491 total (union; filler 0x483410 ascending, global remaining 29338 -> 29238)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;


#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sBlockMeshEEEEvv")]
pub fn stub_0x483410(guard: &mut bool) {
// std lock op — the host guard flips.
*guard = true;
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sBlockMeshEEEERKS0_v")]
pub fn stub_0x483414(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sBlockMesh>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEED1Ev")]
pub fn stub_0x483520(handle: crate::slot::InstanceHandle) {
// settings-item dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEED0Ev")]
pub fn stub_0x483560(handle: crate::slot::InstanceHandle) {
// settings-item dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEED1Ev")]
pub fn stub_0x483640(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEED0Ev")]
pub fn stub_0x483684(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEED1Ev")]
pub fn stub_0x48368c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEED0Ev")]
pub fn stub_0x4836d0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "DummyJob::DummyJob(bool,double)")]
pub fn stub_0x48d9bc() -> crate::slot::PortedFn {
// IDA 0x48d9bc: DummyJob::DummyJob(bool, double).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x48d9bc, "DummyJob::DummyJob(bool, double)")
}

#[doc(alias = "DummyJob::~DummyJob()")]
pub fn stub_0x48db88() -> crate::slot::PortedFn {
// IDA 0x48db88: DummyJob::~DummyJob().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x48db88, "DummyJob::~DummyJob()")
}

#[doc(alias = "DummyJob::~DummyJob() [0x48db8c]")]
pub fn stub_0x48db8c() -> crate::slot::PortedFn {
// IDA 0x48db8c: DummyJob::~DummyJob().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x48db8c, "DummyJob::~DummyJob()")
}

#[doc(alias = "DummyJob::getPriorityFactor(void)")]
pub fn stub_0x48dc4c() -> crate::slot::PortedFn {
// IDA 0x48dc4c: DummyJob::getPriorityFactor().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x48dc4c, "DummyJob::getPriorityFactor()")
}

#[doc(alias = "global constructor keyed to_a_182")]
pub fn stub_0x48ddb0() -> crate::slot::PortedFn {
// IDA 0x48ddb0: __GLOBAL__I_a_182.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x48ddb0, "__GLOBAL__I_a_182")
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x490ab0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Decal"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x490ab4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Texture"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x490bd4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Decal"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x490d04() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Decal"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x490f50() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Texture"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x491080() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Texture"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x4911b0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Texture"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x491224() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Texture"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_8sTextureEEEEvv")]
pub fn stub_0x4912ac() -> crate::slot::PortedFn {
// IDA 0x4912ac: void RBX::Name::callDoDeclare<RBX::sTexture>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x4912ac, "void RBX::Name::callDoDeclare<RBX::sTexture>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sTextureEEEERKS0_v")]
pub fn stub_0x4912b0(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sTexture>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x491390() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Decal"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x491404() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Decal"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_6sDecalEEEEvv")]
pub fn stub_0x49148c() -> crate::slot::PortedFn {
// IDA 0x49148c: void RBX::Name::callDoDeclare<RBX::sDecal>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x49148c, "void RBX::Name::callDoDeclare<RBX::sDecal>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_6sDecalEEEERKS0_v")]
pub fn stub_0x491490(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sDecal>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x491570() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Texture"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x49160c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Texture"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x491afc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Texture"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x491d40() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Decal"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x491ddc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Decal"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x4922cc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Decal"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_5DecalENS_12FaceInstanceELZNS_6sDecalEENS_8InstanceEED2Ev")]
pub fn stub_0x492b40(handle: crate::slot::InstanceHandle) {
// RBX::FactoryProduct dtor.
drop(handle);
}

#[doc(alias = "global constructor keyed to_a_183")]
pub fn stub_0x493248() -> crate::slot::PortedFn {
// IDA 0x493248: __GLOBAL__I_a_183.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x493248, "__GLOBAL__I_a_183")
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_12DialogChoiceENS_8InstanceELZNS_13sDialogChoiceEES2_E12getClassNameEv")]
pub fn stub_0x493d58() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"DialogChoice"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_12DialogChoiceENS_8InstanceELZNS_13sDialogChoiceEES2_E12getClassNameEv")]
pub fn stub_0x494028() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"DialogChoice"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_12DialogChoiceENS_8InstanceELZNS_13sDialogChoiceEES2_E7CreatorD1Ev")]
pub fn stub_0x4942f8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"DialogChoice"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_12DialogChoiceENS_8InstanceELZNS_13sDialogChoiceEES2_E7CreatorD2Ev")]
pub fn stub_0x4942fc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"DialogChoice"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_12DialogChoiceENS_8InstanceELZNS_13sDialogChoiceEES2_E7Creator12getClassNameEv")]
pub fn stub_0x494398() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"DialogChoice"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_12DialogChoiceENS_8InstanceELZNS_13sDialogChoiceEES2_E7Creator6createEv")]
pub fn stub_0x494420() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"DialogChoice"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sDialogChoiceEEEEvv")]
pub fn stub_0x494950() -> crate::slot::PortedFn {
// IDA 0x494950: void RBX::Name::callDoDeclare<RBX::sDialogChoice>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x494950, "void RBX::Name::callDoDeclare<RBX::sDialogChoice>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sDialogChoiceEEEERKS0_v")]
pub fn stub_0x494954(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sDialogChoice>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_12DialogChoiceENS_8InstanceELZNS_13sDialogChoiceEES2_E7CreatorC2Ev")]
pub fn stub_0x494a34() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"DialogChoice"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_12DialogChoiceENS_8InstanceELZNS_13sDialogChoiceEES2_E17static_getCreatorEv")]
pub fn stub_0x494c78() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"DialogChoice"
}

#[doc(alias = "global constructor keyed to_a_184")]
pub fn stub_0x49519c() -> crate::slot::PortedFn {
// IDA 0x49519c: __GLOBAL__I_a_184.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x49519c, "__GLOBAL__I_a_184")
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_10DialogRootENS_8InstanceELZNS_11sDialogRootEES2_E12getClassNameEv")]
pub fn stub_0x496dec() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"DialogRoot"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_10DialogRootENS_8InstanceELZNS_11sDialogRootEES2_E12getClassNameEv")]
pub fn stub_0x496ea8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"DialogRoot"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10DialogRootENS_8InstanceELZNS_11sDialogRootEES2_E7CreatorD1Ev")]
pub fn stub_0x496f64() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"DialogRoot"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10DialogRootENS_8InstanceELZNS_11sDialogRootEES2_E7CreatorD2Ev")]
pub fn stub_0x496f68() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"DialogRoot"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_10DialogRootENS_8InstanceELZNS_11sDialogRootEES2_E7Creator12getClassNameEv")]
pub fn stub_0x497004() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"DialogRoot"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_10DialogRootENS_8InstanceELZNS_11sDialogRootEES2_E7Creator6createEv")]
pub fn stub_0x49708c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"DialogRoot"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sDialogRootEEEEvv")]
pub fn stub_0x49757c() -> crate::slot::PortedFn {
// IDA 0x49757c: void RBX::Name::callDoDeclare<RBX::sDialogRoot>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x49757c, "void RBX::Name::callDoDeclare<RBX::sDialogRoot>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sDialogRootEEEERKS0_v")]
pub fn stub_0x497580(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sDialogRoot>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10DialogRootENS_8InstanceELZNS_11sDialogRootEES2_E7CreatorC2Ev")]
pub fn stub_0x497660() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"DialogRoot"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10DialogRootENS_8InstanceELZNS_11sDialogRootEES2_E17static_getCreatorEv")]
pub fn stub_0x4978a4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"DialogRoot"
}

#[doc(alias = "std::vector<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>::resize(unsigned long,RBX::DialogRoot::DialogPurpose)")]
pub fn stub_0x4983ec(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "std::vector<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>::push_back(RBX::DialogRoot::DialogPurpose const&)")]
pub fn stub_0x498420(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::map<RBX::Name const*,RBX::DialogRoot::DialogPurpose,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x498448(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose> const&)")]
pub fn stub_0x4984a0(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose> const&)")]
pub fn stub_0x498554(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose> const&)")]
pub fn stub_0x4985ac(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::vector<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DialogRoot::DialogPurpose*,std::vector<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>>,RBX::DialogRoot::DialogPurpose const&)")]
pub fn stub_0x498614(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>::_M_allocate(unsigned long)")]
pub fn stub_0x4986f8() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::DialogRoot::DialogPurpose * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DialogRoot::DialogPurpose *,RBX::DialogRoot::DialogPurpose *>(RBX::DialogRoot::DialogPurpose *,RBX::DialogRoot::DialogPurpose *,RBX::DialogRoot::DialogPurpose *)")]
pub fn stub_0x498710(handle: &crate::slot::InstanceHandle) {
// RBX::DialogRoot::DialogPurpose* std::__copy_backward<false, std::random_access_iterator_ta~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DialogRoot::DialogPurpose*,std::vector<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>>,unsigned long,RBX::DialogRoot::DialogPurpose const&)")]
pub fn stub_0x49874c(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "RBX::DialogRoot::~DialogRoot() [0x49ac00]")]
pub fn stub_0x49ac00(handle: crate::slot::InstanceHandle) {
// RBX::DialogRoot dtor.
drop(handle);
}

#[doc(alias = "global constructor keyed to_a_185")]
pub fn stub_0x49aee0() -> crate::slot::PortedFn {
// IDA 0x49aee0: __GLOBAL__I_a_185.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x49aee0, "__GLOBAL__I_a_185")
}

#[doc(alias = "RBX::Effect::Effect(void)")]
pub fn stub_0x49b3e0() -> crate::slot::InstanceHandle {
// RBX::Effect ctor.
crate::slot::InstanceHandle::new("RBX::Effect")
}

#[doc(alias = "RBX::Effect::~Effect()")]
pub fn stub_0x49b3f0(handle: crate::slot::InstanceHandle) {
// RBX::Effect dtor.
drop(handle);
}

#[doc(alias = "RBX::Effect::~Effect() [0x49b3f4]")]
pub fn stub_0x49b3f4(handle: crate::slot::InstanceHandle) {
// RBX::Effect dtor.
drop(handle);
}

#[doc(alias = "RBX::Effect::~Effect() [0x49b3f8]")]
pub fn stub_0x49b3f8(handle: crate::slot::InstanceHandle) {
// RBX::Effect dtor.
drop(handle);
}

#[doc(alias = "global constructor keyed to_a_186")]
pub fn stub_0x49b3fc() -> crate::slot::PortedFn {
// IDA 0x49b3fc: __GLOBAL__I_a_186.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x49b3fc, "__GLOBAL__I_a_186")
}

#[doc(alias = "std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>::resize(unsigned long,RBX::GuiObject::SizeConstraint)")]
pub fn stub_0x49d59c(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>::push_back(RBX::GuiObject::SizeConstraint const&)")]
pub fn stub_0x49d5d0(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::map<RBX::Name const*,RBX::GuiObject::SizeConstraint,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x49d5f8(map: &crate::slot::TreeMapModel) -> usize {
// map size.
map.len()
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>,std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint> const&)")]
pub fn stub_0x49d650(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint> const&)")]
pub fn stub_0x49d704(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint> const&)")]
pub fn stub_0x49d75c(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GuiObject::SizeConstraint*,std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>>,RBX::GuiObject::SizeConstraint const&)")]
pub fn stub_0x49d7c4(vec: &crate::slot::VecModel) -> usize {
// sequence size.
vec.len()
}

#[doc(alias = "std::_Vector_base<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>::_M_allocate(unsigned long)")]
pub fn stub_0x49d8a8() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::GuiObject::SizeConstraint * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GuiObject::SizeConstraint *,RBX::GuiObject::SizeConstraint *>(RBX::GuiObject::SizeConstraint *,RBX::GuiObject::SizeConstraint *,RBX::GuiObject::SizeConstraint *)")]
pub fn stub_0x49d8c0(handle: &crate::slot::InstanceHandle) {
// RBX::GuiObject::SizeConstraint* std::__copy_backward<false, std::random_access_iterator_ta~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GuiObject::SizeConstraint*,std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>>,unsigned long,RBX::GuiObject::SizeConstraint const&)")]
pub fn stub_0x49d8fc(vec: &crate::slot::VecModel) -> usize {
// sequence size.
vec.len()
}

#[doc(alias = "std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>::resize(unsigned long,RBX::Handles::VisualStyle)")]
pub fn stub_0x49da8c(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>::push_back(RBX::Handles::VisualStyle const&)")]
pub fn stub_0x49dac0(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Handles::VisualStyle,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x49dae8(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle> const&)")]
pub fn stub_0x49db40(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle> const&)")]
pub fn stub_0x49dbf4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Handles::VisualStyle> const&)")]
pub fn stub_0x49dc4c(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Handles::VisualStyle*,std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>>,RBX::Handles::VisualStyle const&)")]
pub fn stub_0x49dcb4(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>::_M_allocate(unsigned long)")]
pub fn stub_0x49dd98() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::Handles::VisualStyle * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Handles::VisualStyle *,RBX::Handles::VisualStyle *>(RBX::Handles::VisualStyle *,RBX::Handles::VisualStyle *,RBX::Handles::VisualStyle *)")]
pub fn stub_0x49ddb0(handle: &crate::slot::InstanceHandle) {
// RBX::Handles::VisualStyle* std::__copy_backward<false, std::random_access_iterator_tag>::_~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Handles::VisualStyle*,std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>>,unsigned long,RBX::Handles::VisualStyle const&)")]
pub fn stub_0x49ddec(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "global constructor keyed to_a_187")]
pub fn stub_0x49f33c() -> crate::slot::PortedFn {
// IDA 0x49f33c: __GLOBAL__I_a_187.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x49f33c, "__GLOBAL__I_a_187")
}

#[doc(alias = "RBX::Explosion::setBlastRadius(float)")]
pub fn stub_0x49f5ac(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Explosion setter.
cell.set(value)
}

#[doc(alias = "RBX::Explosion::setExplosionType(RBX::Explosion::ExplosionType)")]
pub fn stub_0x49f5f0(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Explosion setter.
cell.set(value)
}

#[doc(alias = "RBX::Explosion::Explosion(void)")]
pub fn stub_0x49f7ec() -> crate::slot::InstanceHandle {
// RBX::Explosion ctor.
crate::slot::InstanceHandle::new("RBX::Explosion")
}

#[doc(alias = "RBX::Explosion::Explosion(void) [0x49f7f0]")]
pub fn stub_0x49f7f0() -> crate::slot::InstanceHandle {
// RBX::Explosion ctor.
crate::slot::InstanceHandle::new("RBX::Explosion")
}

#[doc(alias = "RBX::Explosion::~Explosion()")]
pub fn stub_0x49fbe4(handle: crate::slot::InstanceHandle) {
// RBX::Explosion dtor.
drop(handle);
}

#[doc(alias = "RBX::Explosion::~Explosion() [0x49fc84]")]
pub fn stub_0x49fc84(handle: crate::slot::InstanceHandle) {
// RBX::Explosion dtor.
drop(handle);
}
