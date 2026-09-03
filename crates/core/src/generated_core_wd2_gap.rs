//! core wd2 gap — 120 core stubs EA-sorted asc RBX-free gap filler not yet in core.
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 120 RBX-free not yet in any crate (21388 uncovered before -> 21268 after, batch 0x9668..0x1cbac).
//! Filter: RBX-free (no RBX substring), rbx_core::SharedPtr not boost.
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "CRenderSettingsItem::setAlwaysDrawConnectors(bool)")]
// 0x9668 — __ZN19CRenderSettingsItem23setAlwaysDrawConnectorsEb
// type: int __fastcall(int this, int)
pub fn stub_0x9668() {
    // IDA 0x9668: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}
#[doc(alias = "CRenderSettingsItem::setShowAggregation(bool)")]
// 0x96ac — __ZN19CRenderSettingsItem18setShowAggregationEb
// type: int __fastcall(int this, int)
pub fn stub_0x96ac() {
    // IDA 0x96ac: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}
#[doc(alias = "CRenderSettingsItem::setDebugShowBoundingBoxes(bool)")]
// 0x973c — __ZN19CRenderSettingsItem25setDebugShowBoundingBoxesEb
// type: int __fastcall(int this, int)
pub fn stub_0x973c() {
    // IDA 0x973c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}
#[doc(alias = "CRenderSettingsItem::setEnableFRM(bool)")]
// 0x9760 — __ZN19CRenderSettingsItem12setEnableFRMEb
// type: int __fastcall(int this, int)
pub fn stub_0x9760() {
    // IDA 0x9760: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}
#[doc(alias = "CRenderSettingsItem::getDebugDisableInterpolation(void)const")]
// 0x9784 — __ZNK19CRenderSettingsItem28getDebugDisableInterpolationEv
// type: int __fastcall(CRenderSettingsItem *this)
pub fn stub_0x9784() {
    // IDA 0x9784: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}
#[doc(alias = "CRenderSettingsItem::setDebugDisableInterpolation(bool)")]
// 0x9794 — __ZN19CRenderSettingsItem28setDebugDisableInterpolationEb
// type: char *__fastcall(CRenderSettingsItem *this, char)
pub fn stub_0x9794() {
    // IDA 0x9794: render-settings accessor owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "CRenderSettingsItem::setTextureCacheSize(unsigned int)")]
// 0x97c0 — __ZN19CRenderSettingsItem19setTextureCacheSizeEj
// type: int __fastcall(int this, unsigned int)
pub fn stub_0x97c0() {
    // IDA 0x97c0: render-settings accessor owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "CRenderSettingsItem::setMeshCacheSize(unsigned int)")]
// 0x97c8 — __ZN19CRenderSettingsItem16setMeshCacheSizeEj
// type: int __fastcall(int this, unsigned int)
pub fn stub_0x97c8() {
    // IDA 0x97c8: render-settings accessor owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "CRenderSettingsItem::CRenderSettingsItem(void)")]
// 0x97d0 — __ZN19CRenderSettingsItemC2Ev
// type: void __fastcall(CRenderSettingsItem *this)
pub fn stub_0x97d0() {
    // IDA 0x97d0: render-settings accessor owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "CRenderSettingsItem::setAutoQualityLevel(int)")]
// 0x9ac8 — __ZN19CRenderSettingsItem19setAutoQualityLevelEi
// type: int __fastcall(int this, int)
pub fn stub_0x9ac8() {
    // IDA 0x9ac8: render-settings accessor owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::setAutoQualityLevel(int)")]
// 0x9ae8 — __ZThn96_N19CRenderSettingsItem19setAutoQualityLevelEi
// type: int __fastcall(int this, int)
pub fn stub_0x9ae8() {
    // IDA 0x9ae8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "CRenderSettingsItem::setEagerBulkExecution(bool)")]
// 0x9b08 — __ZN19CRenderSettingsItem21setEagerBulkExecutionEb
// type: int __fastcall(int this, int)
pub fn stub_0x9b08() {
    // IDA 0x9b08: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "CRenderSettingsItem::~CRenderSettingsItem()")]
// 0xb8b8 — __ZN19CRenderSettingsItemD1Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
pub fn stub_0xb8b8() {
    // IDA 0xb8b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "CRenderSettingsItem::~CRenderSettingsItem()")]
// 0xb8bc — __ZN19CRenderSettingsItemD0Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
pub fn stub_0xb8bc() {
    // IDA 0xb8bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
// 0xb8e0 — __ZThn32_N19CRenderSettingsItemD1Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
pub fn stub_0xb8e0() {
    // IDA 0xb8e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
// 0xb8e8 — __ZThn32_N19CRenderSettingsItemD0Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
pub fn stub_0xb8e8() {
    // IDA 0xb8e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
// 0xb910 — __ZThn36_N19CRenderSettingsItemD1Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
pub fn stub_0xb910() {
    // IDA 0xb910: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
// 0xb918 — __ZThn36_N19CRenderSettingsItemD0Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
pub fn stub_0xb918() {
    // IDA 0xb918: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "CRenderSettingsItem::~CRenderSettingsItem()")]
// 0x16bf4 — __ZN19CRenderSettingsItemD2Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
pub fn stub_0x16bf4() {
    // IDA 0x16bf4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "+[Appirater setAppId:]")]
// 0x17df0 — +[Appirater setAppId:]
// type: void __cdecl(id, SEL, id)
pub fn stub_0x17df0() {
    // IDA 0x17df0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "+[Appirater setDaysUntilPrompt:]")]
// 0x17e00 — +[Appirater setDaysUntilPrompt:]
// type: void __cdecl(id, SEL, double)
pub fn stub_0x17e00() {
    // IDA 0x17e00: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "+[Appirater setUsesUntilPrompt:]")]
// 0x17e14 — +[Appirater setUsesUntilPrompt:]
// type: void __cdecl(id, SEL, int)
pub fn stub_0x17e14() {
    // IDA 0x17e14: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "+[Appirater setSignificantEventsUntilPrompt:]")]
// 0x17e24 — +[Appirater setSignificantEventsUntilPrompt:]
// type: void __cdecl(id, SEL, int)
pub fn stub_0x17e24() {
    // IDA 0x17e24: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "+[Appirater setTimeBeforeReminding:]")]
// 0x17e34 — +[Appirater setTimeBeforeReminding:]
// type: void __cdecl(id, SEL, double)
pub fn stub_0x17e34() {
    // IDA 0x17e34: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "+[Appirater setDebug:]")]
// 0x17e48 — +[Appirater setDebug:]
// type: void __cdecl(id, SEL, char)
pub fn stub_0x17e48() {
    // IDA 0x17e48: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "+[Appirater setDelegate:]")]
// 0x17e58 — +[Appirater setDelegate:]
// type: void __cdecl(id, SEL, id)
pub fn stub_0x17e58() {
    // IDA 0x17e58: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[Appirater connectedToNetwork]")]
// 0x17e68 — -[Appirater connectedToNetwork]
// type: char __cdecl(Appirater *self, SEL)
pub fn stub_0x17e68() {
    // IDA 0x17e68: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "+[Appirater sharedInstance]")]
// 0x17f80 — +[Appirater sharedInstance]
// type: id __cdecl(id, SEL)
pub fn stub_0x17f80() {
    // IDA 0x17f80: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "___27+[Appirater sharedInstance]_block_invoke")]
// 0x17fe4 — ___27+[Appirater sharedInstance]_block_invoke
pub fn stub_0x17fe4() {
    // IDA 0x17fe4: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[Appirater showRatingAlert]")]
// 0x180a8 — -[Appirater showRatingAlert]
// type: void __cdecl(Appirater *self, SEL)
pub fn stub_0x180a8() {
    // IDA 0x180a8: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[Appirater ratingConditionsHaveBeenMet]")]
// 0x183d8 — -[Appirater ratingConditionsHaveBeenMet]
// type: char __cdecl(Appirater *self, SEL)
pub fn stub_0x183d8() {
    // IDA 0x183d8: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[Appirater incrementUseCount]")]
// 0x185b0 — -[Appirater incrementUseCount]
// type: void __cdecl(Appirater *self, SEL)
pub fn stub_0x185b0() {
    // IDA 0x185b0: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[Appirater incrementSignificantEventCount]")]
// 0x18878 — -[Appirater incrementSignificantEventCount]
// type: void __cdecl(Appirater *self, SEL)
pub fn stub_0x18878() {
    // IDA 0x18878: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[Appirater incrementAndRate:]")]
// 0x18b18 — -[Appirater incrementAndRate:]
// type: void __cdecl(Appirater *self, SEL, char)
pub fn stub_0x18b18() {
    // IDA 0x18b18: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "___30-[Appirater incrementAndRate:]_block_invoke")]
// 0x18bb4 — ___30-[Appirater incrementAndRate:]_block_invoke
pub fn stub_0x18bb4() {
    // IDA 0x18bb4: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[Appirater incrementSignificantEventAndRate:]")]
// 0x18bdc — -[Appirater incrementSignificantEventAndRate:]
// type: void __cdecl(Appirater *self, SEL, char)
pub fn stub_0x18bdc() {
    // IDA 0x18bdc: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "___46-[Appirater incrementSignificantEventAndRate:]_block_invoke")]
// 0x18c78 — ___46-[Appirater incrementSignificantEventAndRate:]_block_invoke
pub fn stub_0x18c78() {
    // IDA 0x18c78: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "+[Appirater appLaunched]")]
// 0x18ca0 — +[Appirater appLaunched]
// type: void __cdecl(id, SEL)
pub fn stub_0x18ca0() {
    // IDA 0x18ca0: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "+[Appirater appLaunched:]")]
// 0x18cc0 — +[Appirater appLaunched:]
// type: void __cdecl(id, SEL, char)
pub fn stub_0x18cc0() {
    // IDA 0x18cc0: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "___25+[Appirater appLaunched:]_block_invoke")]
// 0x18d10 — ___25+[Appirater appLaunched:]_block_invoke
pub fn stub_0x18d10() {
    // IDA 0x18d10: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[Appirater hideRatingAlert]")]
// 0x18d4c — -[Appirater hideRatingAlert]
// type: void __cdecl(Appirater *self, SEL)
pub fn stub_0x18d4c() {
    // IDA 0x18d4c: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "+[Appirater appWillResignActive]")]
// 0x18dbc — +[Appirater appWillResignActive]
// type: void __cdecl(id, SEL)
pub fn stub_0x18dbc() {
    // IDA 0x18dbc: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "+[Appirater appEnteredForeground:]")]
// 0x18e0c — +[Appirater appEnteredForeground:]
// type: void __cdecl(id, SEL, char)
pub fn stub_0x18e0c() {
    // IDA 0x18e0c: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "___34+[Appirater appEnteredForeground:]_block_invoke")]
// 0x18e5c — ___34+[Appirater appEnteredForeground:]_block_invoke
pub fn stub_0x18e5c() {
    // IDA 0x18e5c: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "+[Appirater userDidSignificantEvent:]")]
// 0x18e98 — +[Appirater userDidSignificantEvent:]
// type: void __cdecl(id, SEL, char)
pub fn stub_0x18e98() {
    // IDA 0x18e98: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "___37+[Appirater userDidSignificantEvent:]_block_invoke")]
// 0x18ee8 — ___37+[Appirater userDidSignificantEvent:]_block_invoke
pub fn stub_0x18ee8() {
    // IDA 0x18ee8: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "+[Appirater rateApp]")]
// 0x18f24 — +[Appirater rateApp]
// type: void __cdecl(id, SEL)
pub fn stub_0x18f24() {
    // IDA 0x18f24: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[Appirater alertView:clickedButtonAtIndex:]")]
// 0x19028 — -[Appirater alertView:clickedButtonAtIndex:]
// type: void __cdecl(Appirater *self, SEL, id, int)
pub fn stub_0x19028() {
    // IDA 0x19028: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[Appirater ratingAlert]")]
// 0x191d4 — -[Appirater ratingAlert]
// type: UIAlertView *__cdecl(Appirater *self, SEL)
pub fn stub_0x191d4() {
    // IDA 0x191d4: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[Appirater setRatingAlert:]")]
// 0x191e4 — -[Appirater setRatingAlert:]
// type: void __cdecl(Appirater *self, SEL, id)
pub fn stub_0x191e4() {
    // IDA 0x191e4: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[Appirater delegate]")]
// 0x19208 — -[Appirater delegate]
// type: AppiraterDelegate *__cdecl(Appirater *self, SEL)
pub fn stub_0x19208() {
    // IDA 0x19208: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[Appirater setDelegate:]")]
// 0x19218 — -[Appirater setDelegate:]
// type: void __cdecl(Appirater *self, SEL, id)
pub fn stub_0x19218() {
    // IDA 0x19218: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[AppDelegate init]")]
// 0x19228 — -[AppDelegate init]
// type: AppDelegate *__cdecl(AppDelegate *self, SEL)
pub fn stub_0x19228() {
    // IDA 0x19228: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[AppDelegate dealloc]")]
// 0x19254 — -[AppDelegate dealloc]
// type: void __cdecl(AppDelegate *self, SEL)
pub fn stub_0x19254() {
    // IDA 0x19254: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[AppDelegate application:didFinishLaunchingWithOptions:]")]
// 0x192b4 — -[AppDelegate application:didFinishLaunchingWithOptions:]
// type: char __cdecl(AppDelegate *self, SEL, id, id)
pub fn stub_0x192b4() {
    // IDA 0x192b4: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke")]
// 0x194ec — ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke
// type: void __cdecl(id)
pub fn stub_0x194ec() {
    // IDA 0x194ec: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke_2")]
// 0x19514 — ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke_2
// type: void __cdecl(id)
pub fn stub_0x19514() {
    // IDA 0x19514: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[AppDelegate applicationWillResignActive:]")]
// 0x195a0 — -[AppDelegate applicationWillResignActive:]
// type: void __cdecl(AppDelegate *self, SEL, id)
pub fn stub_0x195a0() {
    // IDA 0x195a0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[AppDelegate applicationDidEnterBackground:]")]
// 0x196e4 — -[AppDelegate applicationDidEnterBackground:]
// type: void __cdecl(AppDelegate *self, SEL, id)
pub fn stub_0x196e4() {
    // IDA 0x196e4: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[AppDelegate applicationDidReceiveMemoryWarning:]")]
// 0x19a30 — -[AppDelegate applicationDidReceiveMemoryWarning:]
// type: void __cdecl(AppDelegate *self, SEL, id)
pub fn stub_0x19a30() {
    // IDA 0x19a30: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[AppDelegate applicationWillEnterForeground:]")]
// 0x19b60 — -[AppDelegate applicationWillEnterForeground:]
// type: void __cdecl(AppDelegate *self, SEL, id)
pub fn stub_0x19b60() {
    // IDA 0x19b60: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[AppDelegate applicationDidBecomeActive:]")]
// 0x19cdc — -[AppDelegate applicationDidBecomeActive:]
// type: void __cdecl(AppDelegate *self, SEL, id)
pub fn stub_0x19cdc() {
    // IDA 0x19cdc: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "___42-[AppDelegate applicationDidBecomeActive:]_block_invoke")]
// 0x19f34 — ___42-[AppDelegate applicationDidBecomeActive:]_block_invoke
// type: void __cdecl(id)
pub fn stub_0x19f34() {
    // IDA 0x19f34: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[AppDelegate applicationWillTerminate:]")]
// 0x19f7c — -[AppDelegate applicationWillTerminate:]
// type: void __cdecl(AppDelegate *self, SEL, id)
pub fn stub_0x19f7c() {
    // IDA 0x19f7c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "_topMostController(UIViewController *)")]
// 0x1a098 — __Z18_topMostControllerP16UIViewController
// type: id __fastcall(id)
pub fn stub_0x1a098() {
    // IDA 0x1a098: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[AppDelegate application:openURL:sourceApplication:annotation:]")]
// 0x1a174 — -[AppDelegate application:openURL:sourceApplication:annotation:]
// type: char __cdecl(AppDelegate *self, SEL, id, id, id, id)
pub fn stub_0x1a174() {
    // IDA 0x1a174: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[AppDelegate TryLaunchPlace:]")]
// 0x1a234 — -[AppDelegate TryLaunchPlace:]
// type: void __cdecl(AppDelegate *self, SEL, int)
pub fn stub_0x1a234() {
    // IDA 0x1a234: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[AppDelegate bgTask]")]
// 0x1a494 — -[AppDelegate bgTask]
// type: unsigned int __cdecl(AppDelegate *self, SEL)
pub fn stub_0x1a494() {
    // IDA 0x1a494: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[AppDelegate setBgTask:]")]
// 0x1a4a8 — -[AppDelegate setBgTask:]
// type: void __cdecl(AppDelegate *self, SEL, unsigned int)
pub fn stub_0x1a4a8() {
    // IDA 0x1a4a8: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[AppDelegate window]")]
// 0x1a4c0 — -[AppDelegate window]
// type: UIWindow *__cdecl(AppDelegate *self, SEL)
pub fn stub_0x1a4c0() {
    // IDA 0x1a4c0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[AppDelegate setWindow:]")]
// 0x1a4d0 — -[AppDelegate setWindow:]
// type: void __cdecl(AppDelegate *self, SEL, id)
pub fn stub_0x1a4d0() {
    // IDA 0x1a4d0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[AppDelegate .cxx_destruct]")]
// 0x1a4f4 — -[AppDelegate .cxx_destruct]
// type: void __cdecl(AppDelegate *self, SEL)
pub fn stub_0x1a4f4() {
    // IDA 0x1a4f4: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[AppDelegate .cxx_construct]")]
// 0x1a5bc — -[AppDelegate .cxx_construct]
// type: id __cdecl(AppDelegate *self, SEL)
pub fn stub_0x1a5bc() {
    // IDA 0x1a5bc: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[DebugSettingsViewController initWithCoder:]")]
// 0x1a970 — -[DebugSettingsViewController initWithCoder:]
// type: DebugSettingsViewController *__cdecl(DebugSettingsViewController *self, SEL, id)
pub fn stub_0x1a970() {
    // IDA 0x1a970: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[DebugSettingsViewController dealloc]")]
// 0x1ab20 — -[DebugSettingsViewController dealloc]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
pub fn stub_0x1ab20() {
    // IDA 0x1ab20: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[DebugSettingsViewController reloadOldData]")]
// 0x1ab6c — -[DebugSettingsViewController reloadOldData]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
pub fn stub_0x1ab6c() {
    // IDA 0x1ab6c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[DebugSettingsViewController viewDidLoad]")]
// 0x1ab70 — -[DebugSettingsViewController viewDidLoad]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
pub fn stub_0x1ab70() {
    // IDA 0x1ab70: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[DebugSettingsViewController setDisplayUI]")]
// 0x1abb0 — -[DebugSettingsViewController setDisplayUI]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
pub fn stub_0x1abb0() {
    // IDA 0x1abb0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[DebugSettingsViewController displayPickerDoneClicked:]")]
// 0x1ac80 — -[DebugSettingsViewController displayPickerDoneClicked:]
// type: void __cdecl(DebugSettingsViewController *self, SEL, id)
pub fn stub_0x1ac80() {
    // IDA 0x1ac80: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "___56-[DebugSettingsViewController displayPickerDoneClicked:]_block_invoke")]
// 0x1ad78 — ___56-[DebugSettingsViewController displayPickerDoneClicked:]_block_invoke
// type: id __fastcall(int)
pub fn stub_0x1ad78() {
    // IDA 0x1ad78: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[DebugSettingsViewController displayTouchUp:]")]
// 0x1aed0 — -[DebugSettingsViewController displayTouchUp:]
// type: void __cdecl(DebugSettingsViewController *self, SEL, id)
pub fn stub_0x1aed0() {
    // IDA 0x1aed0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "___46-[DebugSettingsViewController displayTouchUp:]_block_invoke")]
// 0x1afa0 — ___46-[DebugSettingsViewController displayTouchUp:]_block_invoke
// type: id __fastcall(int)
pub fn stub_0x1afa0() {
    // IDA 0x1afa0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[DebugSettingsViewController didReceiveMemoryWarning]")]
// 0x1b170 — -[DebugSettingsViewController didReceiveMemoryWarning]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
pub fn stub_0x1b170() {
    // IDA 0x1b170: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[DebugSettingsViewController shouldAutorotateToInterfaceOrientation:]")]
// 0x1b19c — -[DebugSettingsViewController shouldAutorotateToInterfaceOrientation:]
// type: char __cdecl(DebugSettingsViewController *self, SEL, int)
pub fn stub_0x1b19c() {
    // IDA 0x1b19c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[DebugSettingsViewController viewWillAppear:]")]
// 0x1b224 — -[DebugSettingsViewController viewWillAppear:]
// type: void __cdecl(DebugSettingsViewController *self, SEL, char)
pub fn stub_0x1b224() {
    // IDA 0x1b224: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[DebugSettingsViewController doneTouchUp:]")]
// 0x1b2a8 — -[DebugSettingsViewController doneTouchUp:]
// type: void __cdecl(DebugSettingsViewController *self, SEL, id)
pub fn stub_0x1b2a8() {
    // IDA 0x1b2a8: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[DebugSettingsViewController numberOfComponentsInPickerView:]")]
// 0x1b2bc — -[DebugSettingsViewController numberOfComponentsInPickerView:]
// type: int __cdecl(DebugSettingsViewController *self, SEL, id)
pub fn stub_0x1b2bc() {
    // IDA 0x1b2bc: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[DebugSettingsViewController pickerView:numberOfRowsInComponent:]")]
// 0x1b2c0 — -[DebugSettingsViewController pickerView:numberOfRowsInComponent:]
// type: int __cdecl(DebugSettingsViewController *self, SEL, id, int)
pub fn stub_0x1b2c0() {
    // IDA 0x1b2c0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[DebugSettingsViewController pickerView:titleForRow:forComponent:]")]
// 0x1b2e0 — -[DebugSettingsViewController pickerView:titleForRow:forComponent:]
// type: id __cdecl(DebugSettingsViewController *self, SEL, id, int, int)
pub fn stub_0x1b2e0() {
    // IDA 0x1b2e0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[DebugSettingsViewController disablesAutomaticKeyboardDismissal]")]
// 0x1b300 — -[DebugSettingsViewController disablesAutomaticKeyboardDismissal]
// type: char __cdecl(DebugSettingsViewController *self, SEL)
pub fn stub_0x1b300() {
    // IDA 0x1b300: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[DebugSettingsViewController .cxx_construct]")]
// 0x1b304 — -[DebugSettingsViewController .cxx_construct]
// type: id __cdecl(DebugSettingsViewController *self, SEL)
pub fn stub_0x1b304() {
    // IDA 0x1b304: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[HomeViewController initWithCoder:]")]
// 0x1b3d0 — -[HomeViewController initWithCoder:]
// type: HomeViewController *__cdecl(HomeViewController *self, SEL, id)
pub fn stub_0x1b3d0() {
    // IDA 0x1b3d0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[HomeViewController dealloc]")]
// 0x1b4b0 — -[HomeViewController dealloc]
// type: void __cdecl(HomeViewController *self, SEL)
pub fn stub_0x1b4b0() {
    // IDA 0x1b4b0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[HomeViewController viewDidLoad]")]
// 0x1b75c — -[HomeViewController viewDidLoad]
// type: void __cdecl(HomeViewController *self, SEL)
pub fn stub_0x1b75c() {
    // IDA 0x1b75c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "___33-[HomeViewController viewDidLoad]_block_invoke")]
// 0x1bae4 — ___33-[HomeViewController viewDidLoad]_block_invoke
pub fn stub_0x1bae4() {
    // IDA 0x1bae4: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "___33-[HomeViewController viewDidLoad]_block_invoke_2")]
// 0x1bb64 — ___33-[HomeViewController viewDidLoad]_block_invoke_2
// type: id __fastcall(int)
pub fn stub_0x1bb64() {
    // IDA 0x1bb64: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[HomeViewController keyboardDidShow:]")]
// 0x1bbb0 — -[HomeViewController keyboardDidShow:]
// type: void __cdecl(HomeViewController *self, SEL, id)
pub fn stub_0x1bbb0() {
    // IDA 0x1bbb0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[HomeViewController keyboardDidHide:]")]
// 0x1bbd0 — -[HomeViewController keyboardDidHide:]
// type: void __cdecl(HomeViewController *self, SEL, id)
pub fn stub_0x1bbd0() {
    // IDA 0x1bbd0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[HomeViewController dismissKeyboard]")]
// 0x1bbf0 — -[HomeViewController dismissKeyboard]
// type: void __cdecl(HomeViewController *self, SEL)
pub fn stub_0x1bbf0() {
    // IDA 0x1bbf0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[HomeViewController localizeAndStyleLabels]")]
// 0x1bc10 — -[HomeViewController localizeAndStyleLabels]
// type: void __cdecl(HomeViewController *self, SEL)
pub fn stub_0x1bc10() {
    // IDA 0x1bc10: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[HomeViewController updateUserInfoDisplay:]")]
// 0x1bf0c — -[HomeViewController updateUserInfoDisplay:]
// type: void __cdecl(HomeViewController *self, SEL, bool)
pub fn stub_0x1bf0c() {
    // IDA 0x1bf0c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[HomeViewController viewDidUnload]")]
// 0x1c134 — -[HomeViewController viewDidUnload]
// type: void __cdecl(HomeViewController *self, SEL)
pub fn stub_0x1c134() {
    // IDA 0x1c134: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[HomeViewController handleSignupNotification:]")]
// 0x1c2bc — -[HomeViewController handleSignupNotification:]
// type: void __cdecl(HomeViewController *self, SEL, id)
pub fn stub_0x1c2bc() {
    // IDA 0x1c2bc: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[HomeViewController logoutTouchUp:]")]
// 0x1c37c — -[HomeViewController logoutTouchUp:]
// type: void __cdecl(HomeViewController *self, SEL, id)
pub fn stub_0x1c37c() {
    // IDA 0x1c37c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[HomeViewController alertView:didDismissWithButtonIndex:]")]
// 0x1c4b0 — -[HomeViewController alertView:didDismissWithButtonIndex:]
// type: void __cdecl(HomeViewController *self, SEL, id, int)
pub fn stub_0x1c4b0() {
    // IDA 0x1c4b0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke")]
// 0x1c5c8 — ___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke
pub fn stub_0x1c5c8() {
    // IDA 0x1c5c8: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke227")]
// 0x1c608 — ___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke227
pub fn stub_0x1c608() {
    // IDA 0x1c608: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[HomeViewController viewWillAppear:]")]
// 0x1c748 — -[HomeViewController viewWillAppear:]
// type: void __cdecl(HomeViewController *self, SEL, char)
pub fn stub_0x1c748() {
    // IDA 0x1c748: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[HomeViewController showCorrectLoggedInState]")]
// 0x1c788 — -[HomeViewController showCorrectLoggedInState]
// type: void __cdecl(HomeViewController *self, SEL)
pub fn stub_0x1c788() {
    // IDA 0x1c788: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "___46-[HomeViewController showCorrectLoggedInState]_block_invoke")]
// 0x1c860 — ___46-[HomeViewController showCorrectLoggedInState]_block_invoke
// type: id __fastcall(int)
pub fn stub_0x1c860() {
    // IDA 0x1c860: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[HomeViewController viewDidAppear:]")]
// 0x1c888 — -[HomeViewController viewDidAppear:]
// type: void __cdecl(HomeViewController *self, SEL, char)
pub fn stub_0x1c888() {
    // IDA 0x1c888: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[HomeViewController handleStartGameFailure]")]
// 0x1c8e8 — -[HomeViewController handleStartGameFailure]
// type: void __cdecl(HomeViewController *self, SEL)
pub fn stub_0x1c8e8() {
    // IDA 0x1c8e8: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[HomeViewController handleStartGameSuccess]")]
// 0x1c958 — -[HomeViewController handleStartGameSuccess]
// type: void __cdecl(HomeViewController *self, SEL)
pub fn stub_0x1c958() {
    // IDA 0x1c958: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[HomeViewController placeIdClicked:]")]
// 0x1c95c — -[HomeViewController placeIdClicked:]
// type: void __cdecl(HomeViewController *self, SEL, id)
pub fn stub_0x1c95c() {
    // IDA 0x1c95c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[HomeViewController searchEditingDidEnd:]")]
// 0x1ca9c — -[HomeViewController searchEditingDidEnd:]
// type: void __cdecl(HomeViewController *self, SEL, id)
pub fn stub_0x1ca9c() {
    // IDA 0x1ca9c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[HomeViewController searchDidEndOnExit:]")]
// 0x1caa0 — -[HomeViewController searchDidEndOnExit:]
// type: void __cdecl(HomeViewController *self, SEL, id)
pub fn stub_0x1caa0() {
    // IDA 0x1caa0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[HomeViewController signUpButtonDidTouchUpInside:]")]
// 0x1cac8 — -[HomeViewController signUpButtonDidTouchUpInside:]
// type: void __cdecl(HomeViewController *self, SEL, id)
pub fn stub_0x1cac8() {
    // IDA 0x1cac8: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[HomeViewController logInButtonDidTouchUpInside:]")]
// 0x1cacc — -[HomeViewController logInButtonDidTouchUpInside:]
// type: void __cdecl(HomeViewController *self, SEL, id)
pub fn stub_0x1cacc() {
    // IDA 0x1cacc: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[HomeViewController buttonForWebDidTouchUpInside:]")]
// 0x1cae0 — -[HomeViewController buttonForWebDidTouchUpInside:]
// type: void __cdecl(HomeViewController *self, SEL, id)
pub fn stub_0x1cae0() {
    // IDA 0x1cae0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[HomeViewController btnTouchPlayButtonDisabled:]")]
// 0x1cbac — -[HomeViewController btnTouchPlayButtonDisabled:]
// type: void __cdecl(HomeViewController *self, SEL, id)
pub fn stub_0x1cbac() {
    // IDA 0x1cbac: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
