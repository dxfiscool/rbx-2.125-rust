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
pub fn stub_0x9668() -> ! {
    todo!("0x9668 __ZN19CRenderSettingsItem23setAlwaysDrawConnectorsEb")
}
#[doc(alias = "CRenderSettingsItem::setShowAggregation(bool)")]
// 0x96ac — __ZN19CRenderSettingsItem18setShowAggregationEb
// type: int __fastcall(int this, int)
pub fn stub_0x96ac() -> ! {
    todo!("0x96ac __ZN19CRenderSettingsItem18setShowAggregationEb")
}
#[doc(alias = "CRenderSettingsItem::setDebugShowBoundingBoxes(bool)")]
// 0x973c — __ZN19CRenderSettingsItem25setDebugShowBoundingBoxesEb
// type: int __fastcall(int this, int)
pub fn stub_0x973c() -> ! {
    todo!("0x973c __ZN19CRenderSettingsItem25setDebugShowBoundingBoxesEb")
}
#[doc(alias = "CRenderSettingsItem::setEnableFRM(bool)")]
// 0x9760 — __ZN19CRenderSettingsItem12setEnableFRMEb
// type: int __fastcall(int this, int)
pub fn stub_0x9760() -> ! {
    todo!("0x9760 __ZN19CRenderSettingsItem12setEnableFRMEb")
}
#[doc(alias = "CRenderSettingsItem::getDebugDisableInterpolation(void)const")]
// 0x9784 — __ZNK19CRenderSettingsItem28getDebugDisableInterpolationEv
// type: int __fastcall(CRenderSettingsItem *this)
pub fn stub_0x9784() -> ! {
    todo!("0x9784 __ZNK19CRenderSettingsItem28getDebugDisableInterpolationEv")
}
#[doc(alias = "CRenderSettingsItem::setDebugDisableInterpolation(bool)")]
// 0x9794 — __ZN19CRenderSettingsItem28setDebugDisableInterpolationEb
// type: char *__fastcall(CRenderSettingsItem *this, char)
pub fn stub_0x9794() -> ! {
    todo!("0x9794 __ZN19CRenderSettingsItem28setDebugDisableInterpolationEb")
}
#[doc(alias = "CRenderSettingsItem::setTextureCacheSize(unsigned int)")]
// 0x97c0 — __ZN19CRenderSettingsItem19setTextureCacheSizeEj
// type: int __fastcall(int this, unsigned int)
pub fn stub_0x97c0() -> ! {
    todo!("0x97c0 __ZN19CRenderSettingsItem19setTextureCacheSizeEj")
}
#[doc(alias = "CRenderSettingsItem::setMeshCacheSize(unsigned int)")]
// 0x97c8 — __ZN19CRenderSettingsItem16setMeshCacheSizeEj
// type: int __fastcall(int this, unsigned int)
pub fn stub_0x97c8() -> ! {
    todo!("0x97c8 __ZN19CRenderSettingsItem16setMeshCacheSizeEj")
}
#[doc(alias = "CRenderSettingsItem::CRenderSettingsItem(void)")]
// 0x97d0 — __ZN19CRenderSettingsItemC2Ev
// type: void __fastcall(CRenderSettingsItem *this)
pub fn stub_0x97d0() -> ! {
    todo!("0x97d0 __ZN19CRenderSettingsItemC2Ev")
}
#[doc(alias = "CRenderSettingsItem::setAutoQualityLevel(int)")]
// 0x9ac8 — __ZN19CRenderSettingsItem19setAutoQualityLevelEi
// type: int __fastcall(int this, int)
pub fn stub_0x9ac8() -> ! {
    todo!("0x9ac8 __ZN19CRenderSettingsItem19setAutoQualityLevelEi")
}
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::setAutoQualityLevel(int)")]
// 0x9ae8 — __ZThn96_N19CRenderSettingsItem19setAutoQualityLevelEi
// type: int __fastcall(int this, int)
pub fn stub_0x9ae8() -> ! {
    todo!("0x9ae8 __ZThn96_N19CRenderSettingsItem19setAutoQualityLevelEi")
}
#[doc(alias = "CRenderSettingsItem::setEagerBulkExecution(bool)")]
// 0x9b08 — __ZN19CRenderSettingsItem21setEagerBulkExecutionEb
// type: int __fastcall(int this, int)
pub fn stub_0x9b08() -> ! {
    todo!("0x9b08 __ZN19CRenderSettingsItem21setEagerBulkExecutionEb")
}
#[doc(alias = "CRenderSettingsItem::~CRenderSettingsItem()")]
// 0xb8b8 — __ZN19CRenderSettingsItemD1Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
pub fn stub_0xb8b8() -> ! {
    todo!("0xb8b8 __ZN19CRenderSettingsItemD1Ev")
}
#[doc(alias = "CRenderSettingsItem::~CRenderSettingsItem()")]
// 0xb8bc — __ZN19CRenderSettingsItemD0Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
pub fn stub_0xb8bc() -> ! {
    todo!("0xb8bc __ZN19CRenderSettingsItemD0Ev")
}
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
// 0xb8e0 — __ZThn32_N19CRenderSettingsItemD1Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
pub fn stub_0xb8e0() -> ! {
    todo!("0xb8e0 __ZThn32_N19CRenderSettingsItemD1Ev")
}
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
// 0xb8e8 — __ZThn32_N19CRenderSettingsItemD0Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
pub fn stub_0xb8e8() -> ! {
    todo!("0xb8e8 __ZThn32_N19CRenderSettingsItemD0Ev")
}
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
// 0xb910 — __ZThn36_N19CRenderSettingsItemD1Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
pub fn stub_0xb910() -> ! {
    todo!("0xb910 __ZThn36_N19CRenderSettingsItemD1Ev")
}
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
// 0xb918 — __ZThn36_N19CRenderSettingsItemD0Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
pub fn stub_0xb918() -> ! {
    todo!("0xb918 __ZThn36_N19CRenderSettingsItemD0Ev")
}
#[doc(alias = "CRenderSettingsItem::~CRenderSettingsItem()")]
// 0x16bf4 — __ZN19CRenderSettingsItemD2Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
pub fn stub_0x16bf4() -> ! {
    todo!("0x16bf4 __ZN19CRenderSettingsItemD2Ev")
}
#[doc(alias = "+[Appirater setAppId:]")]
// 0x17df0 — +[Appirater setAppId:]
// type: void __cdecl(id, SEL, id)
pub fn stub_0x17df0() -> ! {
    todo!("0x17df0 +[Appirater setAppId:]")
}
#[doc(alias = "+[Appirater setDaysUntilPrompt:]")]
// 0x17e00 — +[Appirater setDaysUntilPrompt:]
// type: void __cdecl(id, SEL, double)
pub fn stub_0x17e00() -> ! {
    todo!("0x17e00 +[Appirater setDaysUntilPrompt:]")
}
#[doc(alias = "+[Appirater setUsesUntilPrompt:]")]
// 0x17e14 — +[Appirater setUsesUntilPrompt:]
// type: void __cdecl(id, SEL, int)
pub fn stub_0x17e14() -> ! {
    todo!("0x17e14 +[Appirater setUsesUntilPrompt:]")
}
#[doc(alias = "+[Appirater setSignificantEventsUntilPrompt:]")]
// 0x17e24 — +[Appirater setSignificantEventsUntilPrompt:]
// type: void __cdecl(id, SEL, int)
pub fn stub_0x17e24() -> ! {
    todo!("0x17e24 +[Appirater setSignificantEventsUntilPrompt:]")
}
#[doc(alias = "+[Appirater setTimeBeforeReminding:]")]
// 0x17e34 — +[Appirater setTimeBeforeReminding:]
// type: void __cdecl(id, SEL, double)
pub fn stub_0x17e34() -> ! {
    todo!("0x17e34 +[Appirater setTimeBeforeReminding:]")
}
#[doc(alias = "+[Appirater setDebug:]")]
// 0x17e48 — +[Appirater setDebug:]
// type: void __cdecl(id, SEL, char)
pub fn stub_0x17e48() -> ! {
    todo!("0x17e48 +[Appirater setDebug:]")
}
#[doc(alias = "+[Appirater setDelegate:]")]
// 0x17e58 — +[Appirater setDelegate:]
// type: void __cdecl(id, SEL, id)
pub fn stub_0x17e58() -> ! {
    todo!("0x17e58 +[Appirater setDelegate:]")
}
#[doc(alias = "-[Appirater connectedToNetwork]")]
// 0x17e68 — -[Appirater connectedToNetwork]
// type: char __cdecl(Appirater *self, SEL)
pub fn stub_0x17e68() -> ! {
    todo!("0x17e68 -[Appirater connectedToNetwork]")
}
#[doc(alias = "+[Appirater sharedInstance]")]
// 0x17f80 — +[Appirater sharedInstance]
// type: id __cdecl(id, SEL)
pub fn stub_0x17f80() -> ! {
    todo!("0x17f80 +[Appirater sharedInstance]")
}
#[doc(alias = "___27+[Appirater sharedInstance]_block_invoke")]
// 0x17fe4 — ___27+[Appirater sharedInstance]_block_invoke
pub fn stub_0x17fe4() -> ! {
    todo!("0x17fe4 ___27+[Appirater sharedInstance]_block_invoke")
}
#[doc(alias = "-[Appirater showRatingAlert]")]
// 0x180a8 — -[Appirater showRatingAlert]
// type: void __cdecl(Appirater *self, SEL)
pub fn stub_0x180a8() -> ! {
    todo!("0x180a8 -[Appirater showRatingAlert]")
}
#[doc(alias = "-[Appirater ratingConditionsHaveBeenMet]")]
// 0x183d8 — -[Appirater ratingConditionsHaveBeenMet]
// type: char __cdecl(Appirater *self, SEL)
pub fn stub_0x183d8() -> ! {
    todo!("0x183d8 -[Appirater ratingConditionsHaveBeenMet]")
}
#[doc(alias = "-[Appirater incrementUseCount]")]
// 0x185b0 — -[Appirater incrementUseCount]
// type: void __cdecl(Appirater *self, SEL)
pub fn stub_0x185b0() -> ! {
    todo!("0x185b0 -[Appirater incrementUseCount]")
}
#[doc(alias = "-[Appirater incrementSignificantEventCount]")]
// 0x18878 — -[Appirater incrementSignificantEventCount]
// type: void __cdecl(Appirater *self, SEL)
pub fn stub_0x18878() -> ! {
    todo!("0x18878 -[Appirater incrementSignificantEventCount]")
}
#[doc(alias = "-[Appirater incrementAndRate:]")]
// 0x18b18 — -[Appirater incrementAndRate:]
// type: void __cdecl(Appirater *self, SEL, char)
pub fn stub_0x18b18() -> ! {
    todo!("0x18b18 -[Appirater incrementAndRate:]")
}
#[doc(alias = "___30-[Appirater incrementAndRate:]_block_invoke")]
// 0x18bb4 — ___30-[Appirater incrementAndRate:]_block_invoke
pub fn stub_0x18bb4() -> ! {
    todo!("0x18bb4 ___30-[Appirater incrementAndRate:]_block_invoke")
}
#[doc(alias = "-[Appirater incrementSignificantEventAndRate:]")]
// 0x18bdc — -[Appirater incrementSignificantEventAndRate:]
// type: void __cdecl(Appirater *self, SEL, char)
pub fn stub_0x18bdc() -> ! {
    todo!("0x18bdc -[Appirater incrementSignificantEventAndRate:]")
}
#[doc(alias = "___46-[Appirater incrementSignificantEventAndRate:]_block_invoke")]
// 0x18c78 — ___46-[Appirater incrementSignificantEventAndRate:]_block_invoke
pub fn stub_0x18c78() -> ! {
    todo!("0x18c78 ___46-[Appirater incrementSignificantEventAndRate:]_block_invoke")
}
#[doc(alias = "+[Appirater appLaunched]")]
// 0x18ca0 — +[Appirater appLaunched]
// type: void __cdecl(id, SEL)
pub fn stub_0x18ca0() -> ! {
    todo!("0x18ca0 +[Appirater appLaunched]")
}
#[doc(alias = "+[Appirater appLaunched:]")]
// 0x18cc0 — +[Appirater appLaunched:]
// type: void __cdecl(id, SEL, char)
pub fn stub_0x18cc0() -> ! {
    todo!("0x18cc0 +[Appirater appLaunched:]")
}
#[doc(alias = "___25+[Appirater appLaunched:]_block_invoke")]
// 0x18d10 — ___25+[Appirater appLaunched:]_block_invoke
pub fn stub_0x18d10() -> ! {
    todo!("0x18d10 ___25+[Appirater appLaunched:]_block_invoke")
}
#[doc(alias = "-[Appirater hideRatingAlert]")]
// 0x18d4c — -[Appirater hideRatingAlert]
// type: void __cdecl(Appirater *self, SEL)
pub fn stub_0x18d4c() -> ! {
    todo!("0x18d4c -[Appirater hideRatingAlert]")
}
#[doc(alias = "+[Appirater appWillResignActive]")]
// 0x18dbc — +[Appirater appWillResignActive]
// type: void __cdecl(id, SEL)
pub fn stub_0x18dbc() -> ! {
    todo!("0x18dbc +[Appirater appWillResignActive]")
}
#[doc(alias = "+[Appirater appEnteredForeground:]")]
// 0x18e0c — +[Appirater appEnteredForeground:]
// type: void __cdecl(id, SEL, char)
pub fn stub_0x18e0c() -> ! {
    todo!("0x18e0c +[Appirater appEnteredForeground:]")
}
#[doc(alias = "___34+[Appirater appEnteredForeground:]_block_invoke")]
// 0x18e5c — ___34+[Appirater appEnteredForeground:]_block_invoke
pub fn stub_0x18e5c() -> ! {
    todo!("0x18e5c ___34+[Appirater appEnteredForeground:]_block_invoke")
}
#[doc(alias = "+[Appirater userDidSignificantEvent:]")]
// 0x18e98 — +[Appirater userDidSignificantEvent:]
// type: void __cdecl(id, SEL, char)
pub fn stub_0x18e98() -> ! {
    todo!("0x18e98 +[Appirater userDidSignificantEvent:]")
}
#[doc(alias = "___37+[Appirater userDidSignificantEvent:]_block_invoke")]
// 0x18ee8 — ___37+[Appirater userDidSignificantEvent:]_block_invoke
pub fn stub_0x18ee8() -> ! {
    todo!("0x18ee8 ___37+[Appirater userDidSignificantEvent:]_block_invoke")
}
#[doc(alias = "+[Appirater rateApp]")]
// 0x18f24 — +[Appirater rateApp]
// type: void __cdecl(id, SEL)
pub fn stub_0x18f24() -> ! {
    todo!("0x18f24 +[Appirater rateApp]")
}
#[doc(alias = "-[Appirater alertView:clickedButtonAtIndex:]")]
// 0x19028 — -[Appirater alertView:clickedButtonAtIndex:]
// type: void __cdecl(Appirater *self, SEL, id, int)
pub fn stub_0x19028() -> ! {
    todo!("0x19028 -[Appirater alertView:clickedButtonAtIndex:]")
}
#[doc(alias = "-[Appirater ratingAlert]")]
// 0x191d4 — -[Appirater ratingAlert]
// type: UIAlertView *__cdecl(Appirater *self, SEL)
pub fn stub_0x191d4() -> ! {
    todo!("0x191d4 -[Appirater ratingAlert]")
}
#[doc(alias = "-[Appirater setRatingAlert:]")]
// 0x191e4 — -[Appirater setRatingAlert:]
// type: void __cdecl(Appirater *self, SEL, id)
pub fn stub_0x191e4() -> ! {
    todo!("0x191e4 -[Appirater setRatingAlert:]")
}
#[doc(alias = "-[Appirater delegate]")]
// 0x19208 — -[Appirater delegate]
// type: AppiraterDelegate *__cdecl(Appirater *self, SEL)
pub fn stub_0x19208() -> ! {
    todo!("0x19208 -[Appirater delegate]")
}
#[doc(alias = "-[Appirater setDelegate:]")]
// 0x19218 — -[Appirater setDelegate:]
// type: void __cdecl(Appirater *self, SEL, id)
pub fn stub_0x19218() -> ! {
    todo!("0x19218 -[Appirater setDelegate:]")
}
#[doc(alias = "-[AppDelegate init]")]
// 0x19228 — -[AppDelegate init]
// type: AppDelegate *__cdecl(AppDelegate *self, SEL)
pub fn stub_0x19228() -> ! {
    todo!("0x19228 -[AppDelegate init]")
}
#[doc(alias = "-[AppDelegate dealloc]")]
// 0x19254 — -[AppDelegate dealloc]
// type: void __cdecl(AppDelegate *self, SEL)
pub fn stub_0x19254() -> ! {
    todo!("0x19254 -[AppDelegate dealloc]")
}
#[doc(alias = "-[AppDelegate application:didFinishLaunchingWithOptions:]")]
// 0x192b4 — -[AppDelegate application:didFinishLaunchingWithOptions:]
// type: char __cdecl(AppDelegate *self, SEL, id, id)
pub fn stub_0x192b4() -> ! {
    todo!("0x192b4 -[AppDelegate application:didFinishLaunchingWithOptions:]")
}
#[doc(alias = "___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke")]
// 0x194ec — ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke
// type: void __cdecl(id)
pub fn stub_0x194ec() -> ! {
    todo!("0x194ec ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke")
}
#[doc(alias = "___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke_2")]
// 0x19514 — ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke_2
// type: void __cdecl(id)
pub fn stub_0x19514() -> ! {
    todo!("0x19514 ___57-[AppDelegate application:didFinishLaunchingWithOptions:]_block_invoke_2")
}
#[doc(alias = "-[AppDelegate applicationWillResignActive:]")]
// 0x195a0 — -[AppDelegate applicationWillResignActive:]
// type: void __cdecl(AppDelegate *self, SEL, id)
pub fn stub_0x195a0() -> ! {
    todo!("0x195a0 -[AppDelegate applicationWillResignActive:]")
}
#[doc(alias = "-[AppDelegate applicationDidEnterBackground:]")]
// 0x196e4 — -[AppDelegate applicationDidEnterBackground:]
// type: void __cdecl(AppDelegate *self, SEL, id)
pub fn stub_0x196e4() -> ! {
    todo!("0x196e4 -[AppDelegate applicationDidEnterBackground:]")
}
#[doc(alias = "-[AppDelegate applicationDidReceiveMemoryWarning:]")]
// 0x19a30 — -[AppDelegate applicationDidReceiveMemoryWarning:]
// type: void __cdecl(AppDelegate *self, SEL, id)
pub fn stub_0x19a30() -> ! {
    todo!("0x19a30 -[AppDelegate applicationDidReceiveMemoryWarning:]")
}
#[doc(alias = "-[AppDelegate applicationWillEnterForeground:]")]
// 0x19b60 — -[AppDelegate applicationWillEnterForeground:]
// type: void __cdecl(AppDelegate *self, SEL, id)
pub fn stub_0x19b60() -> ! {
    todo!("0x19b60 -[AppDelegate applicationWillEnterForeground:]")
}
#[doc(alias = "-[AppDelegate applicationDidBecomeActive:]")]
// 0x19cdc — -[AppDelegate applicationDidBecomeActive:]
// type: void __cdecl(AppDelegate *self, SEL, id)
pub fn stub_0x19cdc() -> ! {
    todo!("0x19cdc -[AppDelegate applicationDidBecomeActive:]")
}
#[doc(alias = "___42-[AppDelegate applicationDidBecomeActive:]_block_invoke")]
// 0x19f34 — ___42-[AppDelegate applicationDidBecomeActive:]_block_invoke
// type: void __cdecl(id)
pub fn stub_0x19f34() -> ! {
    todo!("0x19f34 ___42-[AppDelegate applicationDidBecomeActive:]_block_invoke")
}
#[doc(alias = "-[AppDelegate applicationWillTerminate:]")]
// 0x19f7c — -[AppDelegate applicationWillTerminate:]
// type: void __cdecl(AppDelegate *self, SEL, id)
pub fn stub_0x19f7c() -> ! {
    todo!("0x19f7c -[AppDelegate applicationWillTerminate:]")
}
#[doc(alias = "_topMostController(UIViewController *)")]
// 0x1a098 — __Z18_topMostControllerP16UIViewController
// type: id __fastcall(id)
pub fn stub_0x1a098() -> ! {
    todo!("0x1a098 __Z18_topMostControllerP16UIViewController")
}
#[doc(alias = "-[AppDelegate application:openURL:sourceApplication:annotation:]")]
// 0x1a174 — -[AppDelegate application:openURL:sourceApplication:annotation:]
// type: char __cdecl(AppDelegate *self, SEL, id, id, id, id)
pub fn stub_0x1a174() -> ! {
    todo!("0x1a174 -[AppDelegate application:openURL:sourceApplication:annotation:]")
}
#[doc(alias = "-[AppDelegate TryLaunchPlace:]")]
// 0x1a234 — -[AppDelegate TryLaunchPlace:]
// type: void __cdecl(AppDelegate *self, SEL, int)
pub fn stub_0x1a234() -> ! {
    todo!("0x1a234 -[AppDelegate TryLaunchPlace:]")
}
#[doc(alias = "-[AppDelegate bgTask]")]
// 0x1a494 — -[AppDelegate bgTask]
// type: unsigned int __cdecl(AppDelegate *self, SEL)
pub fn stub_0x1a494() -> ! {
    todo!("0x1a494 -[AppDelegate bgTask]")
}
#[doc(alias = "-[AppDelegate setBgTask:]")]
// 0x1a4a8 — -[AppDelegate setBgTask:]
// type: void __cdecl(AppDelegate *self, SEL, unsigned int)
pub fn stub_0x1a4a8() -> ! {
    todo!("0x1a4a8 -[AppDelegate setBgTask:]")
}
#[doc(alias = "-[AppDelegate window]")]
// 0x1a4c0 — -[AppDelegate window]
// type: UIWindow *__cdecl(AppDelegate *self, SEL)
pub fn stub_0x1a4c0() -> ! {
    todo!("0x1a4c0 -[AppDelegate window]")
}
#[doc(alias = "-[AppDelegate setWindow:]")]
// 0x1a4d0 — -[AppDelegate setWindow:]
// type: void __cdecl(AppDelegate *self, SEL, id)
pub fn stub_0x1a4d0() -> ! {
    todo!("0x1a4d0 -[AppDelegate setWindow:]")
}
#[doc(alias = "-[AppDelegate .cxx_destruct]")]
// 0x1a4f4 — -[AppDelegate .cxx_destruct]
// type: void __cdecl(AppDelegate *self, SEL)
pub fn stub_0x1a4f4() -> ! {
    todo!("0x1a4f4 -[AppDelegate .cxx_destruct]")
}
#[doc(alias = "-[AppDelegate .cxx_construct]")]
// 0x1a5bc — -[AppDelegate .cxx_construct]
// type: id __cdecl(AppDelegate *self, SEL)
pub fn stub_0x1a5bc() -> ! {
    todo!("0x1a5bc -[AppDelegate .cxx_construct]")
}
#[doc(alias = "-[DebugSettingsViewController initWithCoder:]")]
// 0x1a970 — -[DebugSettingsViewController initWithCoder:]
// type: DebugSettingsViewController *__cdecl(DebugSettingsViewController *self, SEL, id)
pub fn stub_0x1a970() -> ! {
    todo!("0x1a970 -[DebugSettingsViewController initWithCoder:]")
}
#[doc(alias = "-[DebugSettingsViewController dealloc]")]
// 0x1ab20 — -[DebugSettingsViewController dealloc]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
pub fn stub_0x1ab20() -> ! {
    todo!("0x1ab20 -[DebugSettingsViewController dealloc]")
}
#[doc(alias = "-[DebugSettingsViewController reloadOldData]")]
// 0x1ab6c — -[DebugSettingsViewController reloadOldData]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
pub fn stub_0x1ab6c() -> ! {
    todo!("0x1ab6c -[DebugSettingsViewController reloadOldData]")
}
#[doc(alias = "-[DebugSettingsViewController viewDidLoad]")]
// 0x1ab70 — -[DebugSettingsViewController viewDidLoad]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
pub fn stub_0x1ab70() -> ! {
    todo!("0x1ab70 -[DebugSettingsViewController viewDidLoad]")
}
#[doc(alias = "-[DebugSettingsViewController setDisplayUI]")]
// 0x1abb0 — -[DebugSettingsViewController setDisplayUI]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
pub fn stub_0x1abb0() -> ! {
    todo!("0x1abb0 -[DebugSettingsViewController setDisplayUI]")
}
#[doc(alias = "-[DebugSettingsViewController displayPickerDoneClicked:]")]
// 0x1ac80 — -[DebugSettingsViewController displayPickerDoneClicked:]
// type: void __cdecl(DebugSettingsViewController *self, SEL, id)
pub fn stub_0x1ac80() -> ! {
    todo!("0x1ac80 -[DebugSettingsViewController displayPickerDoneClicked:]")
}
#[doc(alias = "___56-[DebugSettingsViewController displayPickerDoneClicked:]_block_invoke")]
// 0x1ad78 — ___56-[DebugSettingsViewController displayPickerDoneClicked:]_block_invoke
// type: id __fastcall(int)
pub fn stub_0x1ad78() -> ! {
    todo!("0x1ad78 ___56-[DebugSettingsViewController displayPickerDoneClicked:]_block_invoke")
}
#[doc(alias = "-[DebugSettingsViewController displayTouchUp:]")]
// 0x1aed0 — -[DebugSettingsViewController displayTouchUp:]
// type: void __cdecl(DebugSettingsViewController *self, SEL, id)
pub fn stub_0x1aed0() -> ! {
    todo!("0x1aed0 -[DebugSettingsViewController displayTouchUp:]")
}
#[doc(alias = "___46-[DebugSettingsViewController displayTouchUp:]_block_invoke")]
// 0x1afa0 — ___46-[DebugSettingsViewController displayTouchUp:]_block_invoke
// type: id __fastcall(int)
pub fn stub_0x1afa0() -> ! {
    todo!("0x1afa0 ___46-[DebugSettingsViewController displayTouchUp:]_block_invoke")
}
#[doc(alias = "-[DebugSettingsViewController didReceiveMemoryWarning]")]
// 0x1b170 — -[DebugSettingsViewController didReceiveMemoryWarning]
// type: void __cdecl(DebugSettingsViewController *self, SEL)
pub fn stub_0x1b170() -> ! {
    todo!("0x1b170 -[DebugSettingsViewController didReceiveMemoryWarning]")
}
#[doc(alias = "-[DebugSettingsViewController shouldAutorotateToInterfaceOrientation:]")]
// 0x1b19c — -[DebugSettingsViewController shouldAutorotateToInterfaceOrientation:]
// type: char __cdecl(DebugSettingsViewController *self, SEL, int)
pub fn stub_0x1b19c() -> ! {
    todo!("0x1b19c -[DebugSettingsViewController shouldAutorotateToInterfaceOrientation:]")
}
#[doc(alias = "-[DebugSettingsViewController viewWillAppear:]")]
// 0x1b224 — -[DebugSettingsViewController viewWillAppear:]
// type: void __cdecl(DebugSettingsViewController *self, SEL, char)
pub fn stub_0x1b224() -> ! {
    todo!("0x1b224 -[DebugSettingsViewController viewWillAppear:]")
}
#[doc(alias = "-[DebugSettingsViewController doneTouchUp:]")]
// 0x1b2a8 — -[DebugSettingsViewController doneTouchUp:]
// type: void __cdecl(DebugSettingsViewController *self, SEL, id)
pub fn stub_0x1b2a8() -> ! {
    todo!("0x1b2a8 -[DebugSettingsViewController doneTouchUp:]")
}
#[doc(alias = "-[DebugSettingsViewController numberOfComponentsInPickerView:]")]
// 0x1b2bc — -[DebugSettingsViewController numberOfComponentsInPickerView:]
// type: int __cdecl(DebugSettingsViewController *self, SEL, id)
pub fn stub_0x1b2bc() -> ! {
    todo!("0x1b2bc -[DebugSettingsViewController numberOfComponentsInPickerView:]")
}
#[doc(alias = "-[DebugSettingsViewController pickerView:numberOfRowsInComponent:]")]
// 0x1b2c0 — -[DebugSettingsViewController pickerView:numberOfRowsInComponent:]
// type: int __cdecl(DebugSettingsViewController *self, SEL, id, int)
pub fn stub_0x1b2c0() -> ! {
    todo!("0x1b2c0 -[DebugSettingsViewController pickerView:numberOfRowsInComponent:]")
}
#[doc(alias = "-[DebugSettingsViewController pickerView:titleForRow:forComponent:]")]
// 0x1b2e0 — -[DebugSettingsViewController pickerView:titleForRow:forComponent:]
// type: id __cdecl(DebugSettingsViewController *self, SEL, id, int, int)
pub fn stub_0x1b2e0() -> ! {
    todo!("0x1b2e0 -[DebugSettingsViewController pickerView:titleForRow:forComponent:]")
}
#[doc(alias = "-[DebugSettingsViewController disablesAutomaticKeyboardDismissal]")]
// 0x1b300 — -[DebugSettingsViewController disablesAutomaticKeyboardDismissal]
// type: char __cdecl(DebugSettingsViewController *self, SEL)
pub fn stub_0x1b300() -> ! {
    todo!("0x1b300 -[DebugSettingsViewController disablesAutomaticKeyboardDismissal]")
}
#[doc(alias = "-[DebugSettingsViewController .cxx_construct]")]
// 0x1b304 — -[DebugSettingsViewController .cxx_construct]
// type: id __cdecl(DebugSettingsViewController *self, SEL)
pub fn stub_0x1b304() -> ! {
    todo!("0x1b304 -[DebugSettingsViewController .cxx_construct]")
}
#[doc(alias = "-[HomeViewController initWithCoder:]")]
// 0x1b3d0 — -[HomeViewController initWithCoder:]
// type: HomeViewController *__cdecl(HomeViewController *self, SEL, id)
pub fn stub_0x1b3d0() -> ! {
    todo!("0x1b3d0 -[HomeViewController initWithCoder:]")
}
#[doc(alias = "-[HomeViewController dealloc]")]
// 0x1b4b0 — -[HomeViewController dealloc]
// type: void __cdecl(HomeViewController *self, SEL)
pub fn stub_0x1b4b0() -> ! {
    todo!("0x1b4b0 -[HomeViewController dealloc]")
}
#[doc(alias = "-[HomeViewController viewDidLoad]")]
// 0x1b75c — -[HomeViewController viewDidLoad]
// type: void __cdecl(HomeViewController *self, SEL)
pub fn stub_0x1b75c() -> ! {
    todo!("0x1b75c -[HomeViewController viewDidLoad]")
}
#[doc(alias = "___33-[HomeViewController viewDidLoad]_block_invoke")]
// 0x1bae4 — ___33-[HomeViewController viewDidLoad]_block_invoke
pub fn stub_0x1bae4() -> ! {
    todo!("0x1bae4 ___33-[HomeViewController viewDidLoad]_block_invoke")
}
#[doc(alias = "___33-[HomeViewController viewDidLoad]_block_invoke_2")]
// 0x1bb64 — ___33-[HomeViewController viewDidLoad]_block_invoke_2
// type: id __fastcall(int)
pub fn stub_0x1bb64() -> ! {
    todo!("0x1bb64 ___33-[HomeViewController viewDidLoad]_block_invoke_2")
}
#[doc(alias = "-[HomeViewController keyboardDidShow:]")]
// 0x1bbb0 — -[HomeViewController keyboardDidShow:]
// type: void __cdecl(HomeViewController *self, SEL, id)
pub fn stub_0x1bbb0() -> ! {
    todo!("0x1bbb0 -[HomeViewController keyboardDidShow:]")
}
#[doc(alias = "-[HomeViewController keyboardDidHide:]")]
// 0x1bbd0 — -[HomeViewController keyboardDidHide:]
// type: void __cdecl(HomeViewController *self, SEL, id)
pub fn stub_0x1bbd0() -> ! {
    todo!("0x1bbd0 -[HomeViewController keyboardDidHide:]")
}
#[doc(alias = "-[HomeViewController dismissKeyboard]")]
// 0x1bbf0 — -[HomeViewController dismissKeyboard]
// type: void __cdecl(HomeViewController *self, SEL)
pub fn stub_0x1bbf0() -> ! {
    todo!("0x1bbf0 -[HomeViewController dismissKeyboard]")
}
#[doc(alias = "-[HomeViewController localizeAndStyleLabels]")]
// 0x1bc10 — -[HomeViewController localizeAndStyleLabels]
// type: void __cdecl(HomeViewController *self, SEL)
pub fn stub_0x1bc10() -> ! {
    todo!("0x1bc10 -[HomeViewController localizeAndStyleLabels]")
}
#[doc(alias = "-[HomeViewController updateUserInfoDisplay:]")]
// 0x1bf0c — -[HomeViewController updateUserInfoDisplay:]
// type: void __cdecl(HomeViewController *self, SEL, bool)
pub fn stub_0x1bf0c() -> ! {
    todo!("0x1bf0c -[HomeViewController updateUserInfoDisplay:]")
}
#[doc(alias = "-[HomeViewController viewDidUnload]")]
// 0x1c134 — -[HomeViewController viewDidUnload]
// type: void __cdecl(HomeViewController *self, SEL)
pub fn stub_0x1c134() -> ! {
    todo!("0x1c134 -[HomeViewController viewDidUnload]")
}
#[doc(alias = "-[HomeViewController handleSignupNotification:]")]
// 0x1c2bc — -[HomeViewController handleSignupNotification:]
// type: void __cdecl(HomeViewController *self, SEL, id)
pub fn stub_0x1c2bc() -> ! {
    todo!("0x1c2bc -[HomeViewController handleSignupNotification:]")
}
#[doc(alias = "-[HomeViewController logoutTouchUp:]")]
// 0x1c37c — -[HomeViewController logoutTouchUp:]
// type: void __cdecl(HomeViewController *self, SEL, id)
pub fn stub_0x1c37c() -> ! {
    todo!("0x1c37c -[HomeViewController logoutTouchUp:]")
}
#[doc(alias = "-[HomeViewController alertView:didDismissWithButtonIndex:]")]
// 0x1c4b0 — -[HomeViewController alertView:didDismissWithButtonIndex:]
// type: void __cdecl(HomeViewController *self, SEL, id, int)
pub fn stub_0x1c4b0() -> ! {
    todo!("0x1c4b0 -[HomeViewController alertView:didDismissWithButtonIndex:]")
}
#[doc(alias = "___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke")]
// 0x1c5c8 — ___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke
pub fn stub_0x1c5c8() -> ! {
    todo!("0x1c5c8 ___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke")
}
#[doc(alias = "___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke227")]
// 0x1c608 — ___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke227
pub fn stub_0x1c608() -> ! {
    todo!("0x1c608 ___58-[HomeViewController alertView:didDismissWithButtonIndex:]_block_invoke227")
}
#[doc(alias = "-[HomeViewController viewWillAppear:]")]
// 0x1c748 — -[HomeViewController viewWillAppear:]
// type: void __cdecl(HomeViewController *self, SEL, char)
pub fn stub_0x1c748() -> ! {
    todo!("0x1c748 -[HomeViewController viewWillAppear:]")
}
#[doc(alias = "-[HomeViewController showCorrectLoggedInState]")]
// 0x1c788 — -[HomeViewController showCorrectLoggedInState]
// type: void __cdecl(HomeViewController *self, SEL)
pub fn stub_0x1c788() -> ! {
    todo!("0x1c788 -[HomeViewController showCorrectLoggedInState]")
}
#[doc(alias = "___46-[HomeViewController showCorrectLoggedInState]_block_invoke")]
// 0x1c860 — ___46-[HomeViewController showCorrectLoggedInState]_block_invoke
// type: id __fastcall(int)
pub fn stub_0x1c860() -> ! {
    todo!("0x1c860 ___46-[HomeViewController showCorrectLoggedInState]_block_invoke")
}
#[doc(alias = "-[HomeViewController viewDidAppear:]")]
// 0x1c888 — -[HomeViewController viewDidAppear:]
// type: void __cdecl(HomeViewController *self, SEL, char)
pub fn stub_0x1c888() -> ! {
    todo!("0x1c888 -[HomeViewController viewDidAppear:]")
}
#[doc(alias = "-[HomeViewController handleStartGameFailure]")]
// 0x1c8e8 — -[HomeViewController handleStartGameFailure]
// type: void __cdecl(HomeViewController *self, SEL)
pub fn stub_0x1c8e8() -> ! {
    todo!("0x1c8e8 -[HomeViewController handleStartGameFailure]")
}
#[doc(alias = "-[HomeViewController handleStartGameSuccess]")]
// 0x1c958 — -[HomeViewController handleStartGameSuccess]
// type: void __cdecl(HomeViewController *self, SEL)
pub fn stub_0x1c958() -> ! {
    todo!("0x1c958 -[HomeViewController handleStartGameSuccess]")
}
#[doc(alias = "-[HomeViewController placeIdClicked:]")]
// 0x1c95c — -[HomeViewController placeIdClicked:]
// type: void __cdecl(HomeViewController *self, SEL, id)
pub fn stub_0x1c95c() -> ! {
    todo!("0x1c95c -[HomeViewController placeIdClicked:]")
}
#[doc(alias = "-[HomeViewController searchEditingDidEnd:]")]
// 0x1ca9c — -[HomeViewController searchEditingDidEnd:]
// type: void __cdecl(HomeViewController *self, SEL, id)
pub fn stub_0x1ca9c() -> ! {
    todo!("0x1ca9c -[HomeViewController searchEditingDidEnd:]")
}
#[doc(alias = "-[HomeViewController searchDidEndOnExit:]")]
// 0x1caa0 — -[HomeViewController searchDidEndOnExit:]
// type: void __cdecl(HomeViewController *self, SEL, id)
pub fn stub_0x1caa0() -> ! {
    todo!("0x1caa0 -[HomeViewController searchDidEndOnExit:]")
}
#[doc(alias = "-[HomeViewController signUpButtonDidTouchUpInside:]")]
// 0x1cac8 — -[HomeViewController signUpButtonDidTouchUpInside:]
// type: void __cdecl(HomeViewController *self, SEL, id)
pub fn stub_0x1cac8() -> ! {
    todo!("0x1cac8 -[HomeViewController signUpButtonDidTouchUpInside:]")
}
#[doc(alias = "-[HomeViewController logInButtonDidTouchUpInside:]")]
// 0x1cacc — -[HomeViewController logInButtonDidTouchUpInside:]
// type: void __cdecl(HomeViewController *self, SEL, id)
pub fn stub_0x1cacc() -> ! {
    todo!("0x1cacc -[HomeViewController logInButtonDidTouchUpInside:]")
}
#[doc(alias = "-[HomeViewController buttonForWebDidTouchUpInside:]")]
// 0x1cae0 — -[HomeViewController buttonForWebDidTouchUpInside:]
// type: void __cdecl(HomeViewController *self, SEL, id)
pub fn stub_0x1cae0() -> ! {
    todo!("0x1cae0 -[HomeViewController buttonForWebDidTouchUpInside:]")
}
#[doc(alias = "-[HomeViewController btnTouchPlayButtonDisabled:]")]
// 0x1cbac — -[HomeViewController btnTouchPlayButtonDisabled:]
// type: void __cdecl(HomeViewController *self, SEL, id)
pub fn stub_0x1cbac() -> ! {
    todo!("0x1cbac -[HomeViewController btnTouchPlayButtonDisabled:]")
}
