// Auto-generated skeletons for rbx-script — filler EA-sorted ascending earliest gap (next 100)
// Filter: Lua|Script|Yield|lua (5401 filtered, all already stubbed globally) — filler EA-sorted ascending
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x3df948..0x3e3910 | existing 8361 -> 8461 total (filler 0x3df948 ascending, global remaining 32875 -> 32775)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "global constructor keyed to_a_167")]
pub fn stub_0x3df948() -> crate::slot::PortedFn {
// IDA 0x3df948: __GLOBAL__I_a_167.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x3df948, "__GLOBAL__I_a_167")
}

// 0x3e0048 — __ZN3RBX5Shirt11setTemplateENS_9TextureIdE
// type: int __fastcall(int, int)
// was: int __fastcall(int, int)
#[doc(alias = "RBX::Shirt::setTemplate(RBX::TextureId)")]
pub fn stub_0x3e0048(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Shirt setter.
cell.set(value)
}

// 0x3e0068 — __ZN3RBX5Pants11setTemplateENS_9TextureIdE
// type: int __fastcall(int, int)
// was: int __fastcall(int, int)
#[doc(alias = "RBX::Pants::setTemplate(RBX::TextureId)")]
pub fn stub_0x3e0068(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Pants setter.
cell.set(value)
}

// 0x3e0088 — __ZN3RBX12ShirtGraphicC2Ev
// type: RBX::Instance *__fastcall(RBX::ShirtGraphic *this)
// was: RBX::Instance *__fastcall(RBX::ShirtGraphic *this)
#[doc(alias = "RBX::ShirtGraphic::ShirtGraphic(void)")]
pub fn stub_0x3e0088() -> crate::slot::InstanceHandle {
// RBX::ShirtGraphic ctor.
crate::slot::InstanceHandle::new("RBX::ShirtGraphic")
}

// 0x3e0320 — __ZN3RBX8ClothingC2Ev
// type: RBX::Instance *__fastcall(RBX::Clothing *this)
// was: RBX::Instance *__fastcall(RBX::Clothing *this)
#[doc(alias = "RBX::Clothing::Clothing(void)")]
pub fn stub_0x3e0320() -> crate::slot::InstanceHandle {
// RBX::Clothing ctor.
crate::slot::InstanceHandle::new("RBX::Clothing")
}

// 0x3e0614 — __ZN3RBX5ShirtC2Ev
// type: RBX::Clothing *__fastcall(RBX::Shirt *this)
// was: RBX::Clothing *__fastcall(RBX::Shirt *this)
#[doc(alias = "RBX::Shirt::Shirt(void)")]
pub fn stub_0x3e0614() -> crate::slot::InstanceHandle {
// RBX::Shirt ctor.
crate::slot::InstanceHandle::new("RBX::Shirt")
}

// 0x3e0798 — __ZN3RBX5PantsC2Ev
// type: RBX::Clothing *__fastcall(RBX::Pants *this)
// was: RBX::Clothing *__fastcall(RBX::Pants *this)
#[doc(alias = "RBX::Pants::Pants(void)")]
pub fn stub_0x3e0798() -> crate::slot::InstanceHandle {
// RBX::Pants ctor.
crate::slot::InstanceHandle::new("RBX::Pants")
}

// 0x3e091c — __ZN3RBX12ShirtGraphic13applyByMyselfEPNS_8HumanoidE
// type: void __fastcall(RBX::ShirtGraphic *this, RBX::Humanoid *)
// was: void __fastcall(RBX::ShirtGraphic *this, RBX::Humanoid *)
#[doc(alias = "RBX::ShirtGraphic::applyByMyself(RBX::Humanoid *)")]
pub fn stub_0x3e091c(handle: &crate::slot::InstanceHandle) {
// RBX::ShirtGraphic::applyByMyself(RBX::Humanoid *) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3e0a58 — __ZN3RBX8Clothing13applyByMyselfEPNS_8HumanoidE
// type: RBX::PartInstance *__fastcall(RBX::Clothing *this, RBX::Humanoid *)
// was: RBX::PartInstance *__fastcall(RBX::Clothing *this, RBX::Humanoid *)
#[doc(alias = "RBX::Clothing::applyByMyself(RBX::Humanoid *)")]
pub fn stub_0x3e0a58(handle: &crate::slot::InstanceHandle) {
// RBX::Clothing::applyByMyself(RBX::Humanoid *) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3e0aac — __ZN3RBX4SkinC2Ev
// type: RBX::Instance *__fastcall(RBX::Skin *this)
// was: RBX::Instance *__fastcall(RBX::Skin *this)
#[doc(alias = "RBX::Skin::Skin(void)")]
pub fn stub_0x3e0aac() -> crate::slot::InstanceHandle {
// RBX::Skin ctor.
crate::slot::InstanceHandle::new("RBX::Skin")
}

// 0x3e0d20 — __ZN3RBX4Skin13applyByMyselfEPNS_8HumanoidE
// type: int __fastcall(RBX::Skin *this, RBX::Humanoid *)
// was: int __fastcall(RBX::Skin *this, RBX::Humanoid *)
#[doc(alias = "RBX::Skin::applyByMyself(RBX::Humanoid *)")]
pub fn stub_0x3e0d20(handle: &crate::slot::InstanceHandle) {
// RBX::Skin::applyByMyself(RBX::Humanoid *) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3e0d9c — __ZN3RBX10BodyColorsC2Ev
// type: RBX::Instance *__fastcall(RBX::BodyColors *this)
// was: RBX::Instance *__fastcall(RBX::BodyColors *this)
#[doc(alias = "RBX::BodyColors::BodyColors(void)")]
pub fn stub_0x3e0d9c() -> crate::slot::InstanceHandle {
// RBX::BodyColors ctor.
crate::slot::InstanceHandle::new("RBX::BodyColors")
}

// 0x3e1028 — __ZN3RBX10BodyColors13applyByMyselfEPNS_8HumanoidE
// type: int __fastcall(RBX::BodyColors *this, RBX::Humanoid *)
// was: int __fastcall(RBX::BodyColors *this, RBX::Humanoid *)
#[doc(alias = "RBX::BodyColors::applyByMyself(RBX::Humanoid *)")]
pub fn stub_0x3e1028(handle: &crate::slot::InstanceHandle) {
// RBX::BodyColors::applyByMyself(RBX::Humanoid *) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3e10b0 — __ZN3RBX25LegacyCharacterAppearance5applyEv
// type: int __fastcall(RBX::LegacyCharacterAppearance *this, int, bool)
// was: int __fastcall(RBX::LegacyCharacterAppearance *this, int, bool)
#[doc(alias = "RBX::LegacyCharacterAppearance::apply(void)")]
pub fn stub_0x3e10b0(handle: &crate::slot::InstanceHandle) {
// RBX::LegacyCharacterAppearance::apply(void) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3e10cc — __ZN3RBX19CharacterAppearance5applyEv
// type: int __fastcall(RBX::Humanoid **this, RBX::Instance *)
// was: int __fastcall(RBX::Humanoid **this, RBX::Instance *)
#[doc(alias = "RBX::CharacterAppearance::apply(void)")]
pub fn stub_0x3e10cc(handle: &crate::slot::InstanceHandle) {
// RBX::CharacterAppearance::apply(void) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::CharacterAppearance::onAncestorChanged(RBX::AncestorChanged const&)")]
pub fn stub_0x3e10f0(handle: &crate::slot::InstanceHandle) {
// RBX::CharacterAppearance::onAncestorChanged(RBX::AncestorChanged const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3e122c — __ZN3RBX12ShirtGraphicD1Ev
// type: void __fastcall(RBX::ShirtGraphic *__hidden this)
// was: void __fastcall(RBX::ShirtGraphic *__hidden this)
#[doc(alias = "RBX::ShirtGraphic::~ShirtGraphic()")]
pub fn stub_0x3e122c(handle: crate::slot::InstanceHandle) {
// RBX::ShirtGraphic dtor.
drop(handle);
}

// 0x3e126c — __ZN3RBX12ShirtGraphicD0Ev
// type: void __fastcall(RBX::ShirtGraphic *__hidden this)
// was: void __fastcall(RBX::ShirtGraphic *__hidden this)
#[doc(alias = "RBX::ShirtGraphic::~ShirtGraphic() [0x3e126c]")]
pub fn stub_0x3e126c(handle: crate::slot::InstanceHandle) {
// RBX::ShirtGraphic dtor.
drop(handle);
}

// 0x3e1344 — __ZNK3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE12getClassNameEv
// type: int()
// was: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x3e1344() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ShirtGraphic"
}

// 0x3e1354 — __ZThn32_N3RBX12ShirtGraphicD1Ev
// type: void __fastcall(RBX::ShirtGraphic *__hidden this)
// was: void __fastcall(RBX::ShirtGraphic *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ShirtGraphic::~ShirtGraphic()")]
pub fn stub_0x3e1354(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3e1394 — __ZThn32_N3RBX12ShirtGraphicD0Ev
// type: void __fastcall(RBX::ShirtGraphic *__hidden this)
// was: void __fastcall(RBX::ShirtGraphic *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ShirtGraphic::~ShirtGraphic() [0x3e1394]")]
pub fn stub_0x3e1394(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3e1470 — __ZThn32_NK3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE12getClassNameEv
// type: int()
// was: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x3e1470() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ShirtGraphic"
}

// 0x3e1480 — __ZThn36_N3RBX12ShirtGraphicD1Ev
// type: void __fastcall(RBX::ShirtGraphic *__hidden this)
// was: void __fastcall(RBX::ShirtGraphic *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ShirtGraphic::~ShirtGraphic() [0x3e1480]")]
pub fn stub_0x3e1480(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3e14c0 — __ZThn36_N3RBX12ShirtGraphicD0Ev
// type: void __fastcall(RBX::ShirtGraphic *__hidden this)
// was: void __fastcall(RBX::ShirtGraphic *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ShirtGraphic::~ShirtGraphic() [0x3e14c0]")]
pub fn stub_0x3e14c0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3e159c — __ZThn92_N3RBX12ShirtGraphicD1Ev
// type: void __fastcall(RBX::ShirtGraphic *__hidden this)
// was: void __fastcall(RBX::ShirtGraphic *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ShirtGraphic::~ShirtGraphic() [0x3e159c]")]
pub fn stub_0x3e159c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3e15dc — __ZThn92_N3RBX12ShirtGraphicD0Ev
// type: void __fastcall(RBX::ShirtGraphic *__hidden this)
// was: void __fastcall(RBX::ShirtGraphic *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ShirtGraphic::~ShirtGraphic() [0x3e15dc]")]
pub fn stub_0x3e15dc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3e16b8 — __ZN3RBX8ClothingD1Ev
// type: void __fastcall(RBX::Clothing *__hidden this)
// was: void __fastcall(RBX::Clothing *__hidden this)
#[doc(alias = "RBX::Clothing::~Clothing()")]
pub fn stub_0x3e16b8(handle: crate::slot::InstanceHandle) {
// RBX::Clothing dtor.
drop(handle);
}

// 0x3e1700 — __ZN3RBX8ClothingD0Ev
// type: void __fastcall(RBX::Clothing *__hidden this)
// was: void __fastcall(RBX::Clothing *__hidden this)
#[doc(alias = "RBX::Clothing::~Clothing() [0x3e1700]")]
pub fn stub_0x3e1700(handle: crate::slot::InstanceHandle) {
// RBX::Clothing dtor.
drop(handle);
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEE12getClassNameEv")]
pub fn stub_0x3e17e0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"CharacterAppearance"
}

// 0x3e1808 — __ZNK3RBX8Clothing11getTemplateEv
// type: int __fastcall(RBX::Clothing *this)
// was: int __fastcall(RBX::Clothing *this)
#[doc(alias = "RBX::Clothing::getTemplate(void)const")]
pub fn stub_0x3e1808(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Clothing getter.
cell.get()
}

// 0x3e1864 — __ZThn32_N3RBX8ClothingD1Ev
// type: void __fastcall(RBX::Clothing *__hidden this)
// was: void __fastcall(RBX::Clothing *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Clothing::~Clothing()")]
pub fn stub_0x3e1864(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3e18b0 — __ZThn32_N3RBX8ClothingD0Ev
// type: void __fastcall(RBX::Clothing *__hidden this)
// was: void __fastcall(RBX::Clothing *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Clothing::~Clothing() [0x3e18b0]")]
pub fn stub_0x3e18b0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEE12getClassNameEv")]
pub fn stub_0x3e1994() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"CharacterAppearance"
}

// 0x3e19bc — __ZThn36_N3RBX8ClothingD1Ev
// type: void __fastcall(RBX::Clothing *__hidden this)
// was: void __fastcall(RBX::Clothing *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Clothing::~Clothing() [0x3e19bc]")]
pub fn stub_0x3e19bc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3e1a08 — __ZThn36_N3RBX8ClothingD0Ev
// type: void __fastcall(RBX::Clothing *__hidden this)
// was: void __fastcall(RBX::Clothing *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Clothing::~Clothing() [0x3e1a08]")]
pub fn stub_0x3e1a08(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3e1aec — __ZThn92_N3RBX8ClothingD1Ev
// type: void __fastcall(RBX::Clothing *__hidden this)
// was: void __fastcall(RBX::Clothing *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Clothing::~Clothing() [0x3e1aec]")]
pub fn stub_0x3e1aec(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3e1b38 — __ZThn92_N3RBX8ClothingD0Ev
// type: void __fastcall(RBX::Clothing *__hidden this)
// was: void __fastcall(RBX::Clothing *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Clothing::~Clothing() [0x3e1b38]")]
pub fn stub_0x3e1b38(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3e1c1c — __ZN3RBX4SkinD1Ev
// type: void __fastcall(RBX::Skin *__hidden this)
// was: void __fastcall(RBX::Skin *__hidden this)
#[doc(alias = "RBX::Skin::~Skin()")]
pub fn stub_0x3e1c1c(handle: crate::slot::InstanceHandle) {
// RBX::Skin dtor.
drop(handle);
}

// 0x3e1c20 — __ZN3RBX4SkinD0Ev
// type: void __fastcall(RBX::Skin *__hidden this)
// was: void __fastcall(RBX::Skin *__hidden this)
#[doc(alias = "RBX::Skin::~Skin() [0x3e1c20]")]
pub fn stub_0x3e1c20(handle: crate::slot::InstanceHandle) {
// RBX::Skin dtor.
drop(handle);
}

// 0x3e1cc0 — __ZNK3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE12getClassNameEv
// type: int()
// was: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x3e1cc0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Skin"
}

// 0x3e1cd0 — __ZThn32_N3RBX4SkinD1Ev
// type: void __fastcall(RBX::Skin *__hidden this)
// was: void __fastcall(RBX::Skin *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Skin::~Skin()")]
pub fn stub_0x3e1cd0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3e1cd8 — __ZThn32_N3RBX4SkinD0Ev
// type: void __fastcall(RBX::Skin *__hidden this)
// was: void __fastcall(RBX::Skin *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Skin::~Skin() [0x3e1cd8]")]
pub fn stub_0x3e1cd8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3e1d7c — __ZThn32_NK3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE12getClassNameEv
// type: int()
// was: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x3e1d7c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Skin"
}

// 0x3e1d8c — __ZThn36_N3RBX4SkinD1Ev
// type: void __fastcall(RBX::Skin *__hidden this)
// was: void __fastcall(RBX::Skin *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Skin::~Skin() [0x3e1d8c]")]
pub fn stub_0x3e1d8c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3e1d94 — __ZThn36_N3RBX4SkinD0Ev
// type: void __fastcall(RBX::Skin *__hidden this)
// was: void __fastcall(RBX::Skin *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Skin::~Skin() [0x3e1d94]")]
pub fn stub_0x3e1d94(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3e1e38 — __ZThn92_N3RBX4SkinD1Ev
// type: void __fastcall(RBX::Skin *__hidden this)
// was: void __fastcall(RBX::Skin *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Skin::~Skin() [0x3e1e38]")]
pub fn stub_0x3e1e38(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3e1e40 — __ZThn92_N3RBX4SkinD0Ev
// type: void __fastcall(RBX::Skin *__hidden this)
// was: void __fastcall(RBX::Skin *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Skin::~Skin() [0x3e1e40]")]
pub fn stub_0x3e1e40(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3e1ee4 — __ZN3RBX10BodyColorsD1Ev
// type: void __fastcall(RBX::BodyColors *__hidden this)
// was: void __fastcall(RBX::BodyColors *__hidden this)
#[doc(alias = "RBX::BodyColors::~BodyColors()")]
pub fn stub_0x3e1ee4(handle: crate::slot::InstanceHandle) {
// RBX::BodyColors dtor.
drop(handle);
}

// 0x3e1ee8 — __ZN3RBX10BodyColorsD0Ev
// type: void __fastcall(RBX::BodyColors *__hidden this)
// was: void __fastcall(RBX::BodyColors *__hidden this)
#[doc(alias = "RBX::BodyColors::~BodyColors() [0x3e1ee8]")]
pub fn stub_0x3e1ee8(handle: crate::slot::InstanceHandle) {
// RBX::BodyColors dtor.
drop(handle);
}

// 0x3e1f88 — __ZNK3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE12getClassNameEv
// type: int()
// was: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x3e1f88() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BodyColors"
}

// 0x3e1f98 — __ZThn32_N3RBX10BodyColorsD1Ev
// type: void __fastcall(RBX::BodyColors *__hidden this)
// was: void __fastcall(RBX::BodyColors *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::BodyColors::~BodyColors()")]
pub fn stub_0x3e1f98(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3e1fa0 — __ZThn32_N3RBX10BodyColorsD0Ev
// type: void __fastcall(RBX::BodyColors *__hidden this)
// was: void __fastcall(RBX::BodyColors *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::BodyColors::~BodyColors() [0x3e1fa0]")]
pub fn stub_0x3e1fa0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3e2044 — __ZThn32_NK3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE12getClassNameEv
// type: int()
// was: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x3e2044() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BodyColors"
}

// 0x3e2054 — __ZThn36_N3RBX10BodyColorsD1Ev
// type: void __fastcall(RBX::BodyColors *__hidden this)
// was: void __fastcall(RBX::BodyColors *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::BodyColors::~BodyColors() [0x3e2054]")]
pub fn stub_0x3e2054(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3e205c — __ZThn36_N3RBX10BodyColorsD0Ev
// type: void __fastcall(RBX::BodyColors *__hidden this)
// was: void __fastcall(RBX::BodyColors *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::BodyColors::~BodyColors() [0x3e205c]")]
pub fn stub_0x3e205c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3e2100 — __ZThn92_N3RBX10BodyColorsD1Ev
// type: void __fastcall(RBX::BodyColors *__hidden this)
// was: void __fastcall(RBX::BodyColors *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::BodyColors::~BodyColors() [0x3e2100]")]
pub fn stub_0x3e2100(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3e2108 — __ZThn92_N3RBX10BodyColorsD0Ev
// type: void __fastcall(RBX::BodyColors *__hidden this)
// was: void __fastcall(RBX::BodyColors *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::BodyColors::~BodyColors() [0x3e2108]")]
pub fn stub_0x3e2108(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3e21ac — __ZN3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7CreatorD1Ev
// type: int()
// was: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x3e21ac() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Shirt"
}

// 0x3e21b0 — __ZN3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7CreatorD1Ev
// type: int()
// was: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x3e21b0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Pants"
}

// 0x3e21b4 — __ZN3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE7CreatorD1Ev
// type: int()
// was: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x3e21b4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ShirtGraphic"
}

// 0x3e21b8 — __ZN3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE7CreatorD1Ev
// type: int()
// was: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x3e21b8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Skin"
}

// 0x3e21bc — __ZN3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE7CreatorD1Ev
// type: int()
// was: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x3e21bc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BodyColors"
}

// 0x3e21c0 — __ZN3RBX5ShirtD1Ev
// type: void __fastcall(RBX::Shirt *__hidden this)
// was: void __fastcall(RBX::Shirt *__hidden this)
#[doc(alias = "RBX::Shirt::~Shirt()")]
pub fn stub_0x3e21c0(handle: crate::slot::InstanceHandle) {
// RBX::Shirt dtor.
drop(handle);
}

// 0x3e2208 — __ZN3RBX5ShirtD0Ev
// type: void __fastcall(RBX::Shirt *__hidden this)
// was: void __fastcall(RBX::Shirt *__hidden this)
#[doc(alias = "RBX::Shirt::~Shirt() [0x3e2208]")]
pub fn stub_0x3e2208(handle: crate::slot::InstanceHandle) {
// RBX::Shirt dtor.
drop(handle);
}

// 0x3e22e8 — __ZNK3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE12getClassNameEv
// type: int()
// was: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x3e22e8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Shirt"
}

// 0x3e22f8 — __ZNK3RBX5Shirt11getTemplateEv
// type: int __fastcall(RBX::Shirt *this, int)
// was: int __fastcall(RBX::Shirt *this, int)
#[doc(alias = "RBX::Shirt::getTemplate(void)const")]
pub fn stub_0x3e22f8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Shirt getter.
cell.get()
}

// 0x3e2310 — __ZThn32_N3RBX5ShirtD1Ev
// type: void __fastcall(RBX::Shirt *__hidden this)
// was: void __fastcall(RBX::Shirt *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Shirt::~Shirt()")]
pub fn stub_0x3e2310(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3e235c — __ZThn32_N3RBX5ShirtD0Ev
// type: void __fastcall(RBX::Shirt *__hidden this)
// was: void __fastcall(RBX::Shirt *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Shirt::~Shirt() [0x3e235c]")]
pub fn stub_0x3e235c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3e2440 — __ZThn32_NK3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE12getClassNameEv
// type: int()
// was: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x3e2440() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Shirt"
}

// 0x3e2450 — __ZThn36_N3RBX5ShirtD1Ev
// type: void __fastcall(RBX::Shirt *__hidden this)
// was: void __fastcall(RBX::Shirt *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Shirt::~Shirt() [0x3e2450]")]
pub fn stub_0x3e2450(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3e249c — __ZThn36_N3RBX5ShirtD0Ev
// type: void __fastcall(RBX::Shirt *__hidden this)
// was: void __fastcall(RBX::Shirt *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Shirt::~Shirt() [0x3e249c]")]
pub fn stub_0x3e249c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3e2580 — __ZThn92_N3RBX5ShirtD1Ev
// type: void __fastcall(RBX::Shirt *__hidden this)
// was: void __fastcall(RBX::Shirt *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Shirt::~Shirt() [0x3e2580]")]
pub fn stub_0x3e2580(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3e25cc — __ZThn92_N3RBX5ShirtD0Ev
// type: void __fastcall(RBX::Shirt *__hidden this)
// was: void __fastcall(RBX::Shirt *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Shirt::~Shirt() [0x3e25cc]")]
pub fn stub_0x3e25cc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3e26b0 — __ZN3RBX5PantsD1Ev
// type: void __fastcall(RBX::Pants *__hidden this)
// was: void __fastcall(RBX::Pants *__hidden this)
#[doc(alias = "RBX::Pants::~Pants()")]
pub fn stub_0x3e26b0(handle: crate::slot::InstanceHandle) {
// RBX::Pants dtor.
drop(handle);
}

// 0x3e26f8 — __ZN3RBX5PantsD0Ev
// type: void __fastcall(RBX::Pants *__hidden this)
// was: void __fastcall(RBX::Pants *__hidden this)
#[doc(alias = "RBX::Pants::~Pants() [0x3e26f8]")]
pub fn stub_0x3e26f8(handle: crate::slot::InstanceHandle) {
// RBX::Pants dtor.
drop(handle);
}

// 0x3e27d8 — __ZNK3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE12getClassNameEv
// type: int()
// was: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x3e27d8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Pants"
}

// 0x3e27e8 — __ZNK3RBX5Pants11getTemplateEv
// type: int __fastcall(RBX::Pants *this, int)
// was: int __fastcall(RBX::Pants *this, int)
#[doc(alias = "RBX::Pants::getTemplate(void)const")]
pub fn stub_0x3e27e8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Pants getter.
cell.get()
}

// 0x3e2800 — __ZThn32_N3RBX5PantsD1Ev
// type: void __fastcall(RBX::Pants *__hidden this)
// was: void __fastcall(RBX::Pants *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Pants::~Pants()")]
pub fn stub_0x3e2800(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3e284c — __ZThn32_N3RBX5PantsD0Ev
// type: void __fastcall(RBX::Pants *__hidden this)
// was: void __fastcall(RBX::Pants *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Pants::~Pants() [0x3e284c]")]
pub fn stub_0x3e284c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3e2930 — __ZThn32_NK3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE12getClassNameEv
// type: int()
// was: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x3e2930() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Pants"
}

// 0x3e2940 — __ZThn36_N3RBX5PantsD1Ev
// type: void __fastcall(RBX::Pants *__hidden this)
// was: void __fastcall(RBX::Pants *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Pants::~Pants() [0x3e2940]")]
pub fn stub_0x3e2940(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3e298c — __ZThn36_N3RBX5PantsD0Ev
// type: void __fastcall(RBX::Pants *__hidden this)
// was: void __fastcall(RBX::Pants *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Pants::~Pants() [0x3e298c]")]
pub fn stub_0x3e298c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3e2a70 — __ZThn92_N3RBX5PantsD1Ev
// type: void __fastcall(RBX::Pants *__hidden this)
// was: void __fastcall(RBX::Pants *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Pants::~Pants() [0x3e2a70]")]
pub fn stub_0x3e2a70(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3e2abc — __ZThn92_N3RBX5PantsD0Ev
// type: void __fastcall(RBX::Pants *__hidden this)
// was: void __fastcall(RBX::Pants *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Pants::~Pants() [0x3e2abc]")]
pub fn stub_0x3e2abc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3e2ba0 — __ZN3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE17static_getCreatorEv
// type: void *()
// was: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x3e2ba0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Pants"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x3e2c14() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Pants"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_6sPantsEEEEvv")]
pub fn stub_0x3e2c9c() -> crate::slot::PortedFn {
// IDA 0x3e2c9c: void RBX::Name::callDoDeclare<RBX::sPants>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3e2c9c, "void RBX::Name::callDoDeclare<RBX::sPants>()")
}

// 0x3e2ca0 — __ZN3RBX4Name9doDeclareILZNS_6sPantsEEEERKS0_v
// type: int()
// was: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_6sPantsEEEERKS0_v")]
pub fn stub_0x3e2ca0(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sPants>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3e2d80 — __ZN3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE17static_getCreatorEv
// type: void *()
// was: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x3e2d80() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Shirt"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x3e2df4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Shirt"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_6sShirtEEEEvv")]
pub fn stub_0x3e2e7c() -> crate::slot::PortedFn {
// IDA 0x3e2e7c: void RBX::Name::callDoDeclare<RBX::sShirt>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3e2e7c, "void RBX::Name::callDoDeclare<RBX::sShirt>()")
}

// 0x3e2e80 — __ZN3RBX4Name9doDeclareILZNS_6sShirtEEEERKS0_v
// type: int()
// was: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_6sShirtEEEERKS0_v")]
pub fn stub_0x3e2e80(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sShirt>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3e2f60 — __ZN3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
// was: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x3e2f60() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BodyColors"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x3e2ffc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BodyColors"
}

// 0x3e3084 — __ZNK3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(_DWORD *)
// was: void __fastcall(_DWORD *)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x3e3084() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BodyColors"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sBodyColorsEEEEvv")]
pub fn stub_0x3e3574() -> crate::slot::PortedFn {
// IDA 0x3e3574: void RBX::Name::callDoDeclare<RBX::sBodyColors>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3e3574, "void RBX::Name::callDoDeclare<RBX::sBodyColors>()")
}

// 0x3e3578 — __ZN3RBX4Name9doDeclareILZNS_11sBodyColorsEEEERKS0_v
// type: int()
// was: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sBodyColorsEEEERKS0_v")]
pub fn stub_0x3e3578(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sBodyColors>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3e3658 — __ZN3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
// was: pthread_mutex_t *__fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x3e3658() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BodyColors"
}

// 0x3e389c — __ZN3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE17static_getCreatorEv
// type: void *()
// was: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x3e389c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BodyColors"
}

// 0x3e3910 — __ZN3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
// was: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x3e3910() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Skin"
}
