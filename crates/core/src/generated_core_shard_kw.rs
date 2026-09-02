//! core shard kw — 100 stubs EA-sorted asc global gap filler not yet in core (fallback filter).
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 100 after kv 0xed7694..0xedaa2c (fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound; 52285 filtered, 4123 remaining before -> 4023 after, rbx_core::SharedPtr not boost).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + #[doc(alias = mangled)] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "-[TFURLConnectionOperation _finishWithData:response:error:]")]
#[doc(alias = "-[TFURLConnectionOperation _finishWithData:response:error:]")]
// 0xed7694 — -[TFURLConnectionOperation _finishWithData:response:error:]
// type: void __cdecl(TFURLConnectionOperation *self, SEL, id, id, id)
pub fn stub_ed7694() -> ! {
    todo!("0xed7694 -[TFURLConnectionOperation _finishWithData:response:error:]")
}

#[doc(alias = "___59-[TFURLConnectionOperation _finishWithData:response:error:]_block_invoke")]
#[doc(alias = "___59-[TFURLConnectionOperation _finishWithData:response:error:]_block_invoke")]
// 0xed7834 — ___59-[TFURLConnectionOperation _finishWithData:response:error:]_block_invoke
pub fn stub_ed7834() -> ! {
    todo!("0xed7834 ___59-[TFURLConnectionOperation _finishWithData:response:error:]_block_invoke")
}

#[doc(alias = "___copy_helper_block_88_1")]
#[doc(alias = "___copy_helper_block_88_1")]
// 0xed7860 — ___copy_helper_block_88_1
pub fn stub_ed7860() -> ! {
    todo!("0xed7860 ___copy_helper_block_88_1")
}

#[doc(alias = "___destroy_helper_block_89_1")]
#[doc(alias = "___destroy_helper_block_89_1")]
// 0xed7880 — ___destroy_helper_block_89_1
pub fn stub_ed7880() -> ! {
    todo!("0xed7880 ___destroy_helper_block_89_1")
}

#[doc(alias = "-[TFURLConnectionOperation connection:needNewBodyStream:]")]
#[doc(alias = "-[TFURLConnectionOperation connection:needNewBodyStream:]")]
// 0xed78a0 — -[TFURLConnectionOperation connection:needNewBodyStream:]
// type: id __cdecl(TFURLConnectionOperation *self, SEL, id, id)
pub fn stub_ed78a0() -> ! {
    todo!("0xed78a0 -[TFURLConnectionOperation connection:needNewBodyStream:]")
}

#[doc(alias = "-[TFURLConnectionOperation connection:didReceiveResponse:]")]
#[doc(alias = "-[TFURLConnectionOperation connection:didReceiveResponse:]")]
// 0xed78e0 — -[TFURLConnectionOperation connection:didReceiveResponse:]
// type: void __cdecl(TFURLConnectionOperation *self, SEL, id, id)
pub fn stub_ed78e0() -> ! {
    todo!("0xed78e0 -[TFURLConnectionOperation connection:didReceiveResponse:]")
}

#[doc(alias = "-[TFURLConnectionOperation connection:didReceiveData:]")]
#[doc(alias = "-[TFURLConnectionOperation connection:didReceiveData:]")]
// 0xed7924 — -[TFURLConnectionOperation connection:didReceiveData:]
// type: void __cdecl(TFURLConnectionOperation *self, SEL, id, id)
pub fn stub_ed7924() -> ! {
    todo!("0xed7924 -[TFURLConnectionOperation connection:didReceiveData:]")
}

#[doc(alias = "-[TFURLConnectionOperation connectionDidFinishLoading:]")]
#[doc(alias = "-[TFURLConnectionOperation connectionDidFinishLoading:]")]
// 0xed794c — -[TFURLConnectionOperation connectionDidFinishLoading:]
// type: void __cdecl(TFURLConnectionOperation *self, SEL, id)
pub fn stub_ed794c() -> ! {
    todo!("0xed794c -[TFURLConnectionOperation connectionDidFinishLoading:]")
}

#[doc(alias = "-[TFURLConnectionOperation connection:didFailWithError:]")]
#[doc(alias = "-[TFURLConnectionOperation connection:didFailWithError:]")]
// 0xed7990 — -[TFURLConnectionOperation connection:didFailWithError:]
// type: void __cdecl(TFURLConnectionOperation *self, SEL, id, id)
pub fn stub_ed7990() -> ! {
    todo!("0xed7990 -[TFURLConnectionOperation connection:didFailWithError:]")
}

#[doc(alias = "-[TFURLConnectionOperation isExecuting]")]
#[doc(alias = "-[TFURLConnectionOperation isExecuting]")]
// 0xed79b0 — -[TFURLConnectionOperation isExecuting]
// type: char __cdecl(TFURLConnectionOperation *self, SEL)
pub fn stub_ed79b0() -> ! {
    todo!("0xed79b0 -[TFURLConnectionOperation isExecuting]")
}

#[doc(alias = "-[TFURLConnectionOperation setIsExecuting:]")]
#[doc(alias = "-[TFURLConnectionOperation setIsExecuting:]")]
// 0xed79c8 — -[TFURLConnectionOperation setIsExecuting:]
// type: void __cdecl(TFURLConnectionOperation *self, SEL, char)
pub fn stub_ed79c8() -> ! {
    todo!("0xed79c8 -[TFURLConnectionOperation setIsExecuting:]")
}

#[doc(alias = "-[TFURLConnectionOperation isFinished]")]
#[doc(alias = "-[TFURLConnectionOperation isFinished]")]
// 0xed79e0 — -[TFURLConnectionOperation isFinished]
// type: char __cdecl(TFURLConnectionOperation *self, SEL)
pub fn stub_ed79e0() -> ! {
    todo!("0xed79e0 -[TFURLConnectionOperation isFinished]")
}

#[doc(alias = "-[TFURLConnectionOperation setIsFinished:]")]
#[doc(alias = "-[TFURLConnectionOperation setIsFinished:]")]
// 0xed79f8 — -[TFURLConnectionOperation setIsFinished:]
// type: void __cdecl(TFURLConnectionOperation *self, SEL, char)
pub fn stub_ed79f8() -> ! {
    todo!("0xed79f8 -[TFURLConnectionOperation setIsFinished:]")
}

#[doc(alias = "-[TFURLConnectionOperation .cxx_destruct]")]
#[doc(alias = "-[TFURLConnectionOperation .cxx_destruct]")]
// 0xed7a10 — -[TFURLConnectionOperation .cxx_destruct]
// type: void __cdecl(TFURLConnectionOperation *self, SEL)
pub fn stub_ed7a10() -> ! {
    todo!("0xed7a10 -[TFURLConnectionOperation .cxx_destruct]")
}

#[doc(alias = "+[TFSDKUpgradeManager sharedSDKUpgradeManager]")]
#[doc(alias = "+[TFSDKUpgradeManager sharedSDKUpgradeManager]")]
// 0xed7aa4 — +[TFSDKUpgradeManager sharedSDKUpgradeManager]
// type: id __cdecl(id, SEL)
pub fn stub_ed7aa4() -> ! {
    todo!("0xed7aa4 +[TFSDKUpgradeManager sharedSDKUpgradeManager]")
}

#[doc(alias = "___46+[TFSDKUpgradeManager sharedSDKUpgradeManager]_block_invoke")]
#[doc(alias = "___46+[TFSDKUpgradeManager sharedSDKUpgradeManager]_block_invoke")]
// 0xed7ad4 — ___46+[TFSDKUpgradeManager sharedSDKUpgradeManager]_block_invoke
// type: void __cdecl(id)
pub fn stub_ed7ad4() -> ! {
    todo!("0xed7ad4 ___46+[TFSDKUpgradeManager sharedSDKUpgradeManager]_block_invoke")
}

#[doc(alias = "-[TFSDKUpgradeManager upgradeSDKIfNecessary]")]
#[doc(alias = "-[TFSDKUpgradeManager upgradeSDKIfNecessary]")]
// 0xed7b1c — -[TFSDKUpgradeManager upgradeSDKIfNecessary]
// type: void __cdecl(TFSDKUpgradeManager *self, SEL)
pub fn stub_ed7b1c() -> ! {
    todo!("0xed7b1c -[TFSDKUpgradeManager upgradeSDKIfNecessary]")
}

#[doc(alias = "-[TFSDKUpgradeManager _upgradePaths]")]
#[doc(alias = "-[TFSDKUpgradeManager _upgradePaths]")]
// 0xed7c7c — -[TFSDKUpgradeManager _upgradePaths]
// type: id __cdecl(TFSDKUpgradeManager *self, SEL)
pub fn stub_ed7c7c() -> ! {
    todo!("0xed7c7c -[TFSDKUpgradeManager _upgradePaths]")
}

#[doc(alias = "___36-[TFSDKUpgradeManager _upgradePaths]_block_invoke")]
#[doc(alias = "___36-[TFSDKUpgradeManager _upgradePaths]_block_invoke")]
// 0xed7d50 — ___36-[TFSDKUpgradeManager _upgradePaths]_block_invoke
// type: void __cdecl(id)
pub fn stub_ed7d50() -> ! {
    todo!("0xed7d50 ___36-[TFSDKUpgradeManager _upgradePaths]_block_invoke")
}

#[doc(alias = "-[TFSDKUpgradeManager _getCurrentSDKUpgradeVersion]")]
#[doc(alias = "-[TFSDKUpgradeManager _getCurrentSDKUpgradeVersion]")]
// 0xed7dd4 — -[TFSDKUpgradeManager _getCurrentSDKUpgradeVersion]
// type: int __cdecl(TFSDKUpgradeManager *self, SEL)
pub fn stub_ed7dd4() -> ! {
    todo!("0xed7dd4 -[TFSDKUpgradeManager _getCurrentSDKUpgradeVersion]")
}

#[doc(alias = "-[TFSDKUpgradeManager _setCurrentSDKUpgradeVersion:]")]
#[doc(alias = "-[TFSDKUpgradeManager _setCurrentSDKUpgradeVersion:]")]
// 0xed7ef4 — -[TFSDKUpgradeManager _setCurrentSDKUpgradeVersion:]
// type: void __cdecl(TFSDKUpgradeManager *self, SEL, int)
pub fn stub_ed7ef4() -> ! {
    todo!("0xed7ef4 -[TFSDKUpgradeManager _setCurrentSDKUpgradeVersion:]")
}

#[doc(alias = "-[TFTimer init]")]
#[doc(alias = "-[TFTimer init]")]
// 0xed7ff8 — -[TFTimer init]
// type: TFTimer *__cdecl(TFTimer *self, SEL)
pub fn stub_ed7ff8() -> ! {
    todo!("0xed7ff8 -[TFTimer init]")
}

#[doc(alias = "-[TFTimer initWithInterval:dispatchQueue:block:]")]
#[doc(alias = "-[TFTimer initWithInterval:dispatchQueue:block:]")]
// 0xed8004 — -[TFTimer initWithInterval:dispatchQueue:block:]
// type: TFTimer *__cdecl(TFTimer *self, SEL, double, dispatch_queue_s *, id)
pub fn stub_ed8004() -> ! {
    todo!("0xed8004 -[TFTimer initWithInterval:dispatchQueue:block:]")
}

#[doc(alias = "___48-[TFTimer initWithInterval:dispatchQueue:block:]_block_invoke")]
#[doc(alias = "___48-[TFTimer initWithInterval:dispatchQueue:block:]_block_invoke")]
// 0xed8144 — ___48-[TFTimer initWithInterval:dispatchQueue:block:]_block_invoke
pub fn stub_ed8144() -> ! {
    todo!("0xed8144 ___48-[TFTimer initWithInterval:dispatchQueue:block:]_block_invoke")
}

#[doc(alias = "-[TFTimer dealloc]")]
#[doc(alias = "-[TFTimer dealloc]")]
// 0xed815c — -[TFTimer dealloc]
// type: void __cdecl(TFTimer *self, SEL)
pub fn stub_ed815c() -> ! {
    todo!("0xed815c -[TFTimer dealloc]")
}

#[doc(alias = "-[TFTimer setInterval:]")]
#[doc(alias = "-[TFTimer setInterval:]")]
// 0xed819c — -[TFTimer setInterval:]
// type: void __cdecl(TFTimer *self, SEL, double)
pub fn stub_ed819c() -> ! {
    todo!("0xed819c -[TFTimer setInterval:]")
}

#[doc(alias = "-[TFTimer restart]")]
#[doc(alias = "-[TFTimer restart]")]
// 0xed81c8 — -[TFTimer restart]
// type: void __cdecl(TFTimer *self, SEL)
pub fn stub_ed81c8() -> ! {
    todo!("0xed81c8 -[TFTimer restart]")
}

#[doc(alias = "-[TFTimer fire]")]
#[doc(alias = "-[TFTimer fire]")]
// 0xed8260 — -[TFTimer fire]
// type: void __cdecl(TFTimer *self, SEL)
pub fn stub_ed8260() -> ! {
    todo!("0xed8260 -[TFTimer fire]")
}

#[doc(alias = "-[TFTimer invalidate]")]
#[doc(alias = "-[TFTimer invalidate]")]
// 0xed828c — -[TFTimer invalidate]
// type: void __cdecl(TFTimer *self, SEL)
pub fn stub_ed828c() -> ! {
    todo!("0xed828c -[TFTimer invalidate]")
}

#[doc(alias = "-[TFTimer isValid]")]
#[doc(alias = "-[TFTimer isValid]")]
// 0xed82d8 — -[TFTimer isValid]
// type: char __cdecl(TFTimer *self, SEL)
pub fn stub_ed82d8() -> ! {
    todo!("0xed82d8 -[TFTimer isValid]")
}

#[doc(alias = "-[TFTimer block]")]
#[doc(alias = "-[TFTimer block]")]
// 0xed82e8 — -[TFTimer block]
// type: id __cdecl(TFTimer *self, SEL)
pub fn stub_ed82e8() -> ! {
    todo!("0xed82e8 -[TFTimer block]")
}

#[doc(alias = "-[TFTimer setBlock:]")]
#[doc(alias = "-[TFTimer setBlock:]")]
// 0xed8300 — -[TFTimer setBlock:]
// type: void __cdecl(TFTimer *self, SEL, id)
pub fn stub_ed8300() -> ! {
    todo!("0xed8300 -[TFTimer setBlock:]")
}

#[doc(alias = "-[TFTimer interval]")]
#[doc(alias = "-[TFTimer interval]")]
// 0xed8324 — -[TFTimer interval]
// type: double __cdecl(TFTimer *self, SEL)
pub fn stub_ed8324() -> ! {
    todo!("0xed8324 -[TFTimer interval]")
}

#[doc(alias = "-[TFTimer .cxx_destruct]")]
#[doc(alias = "-[TFTimer .cxx_destruct]")]
// 0xed833c — -[TFTimer .cxx_destruct]
// type: void __cdecl(TFTimer *self, SEL)
pub fn stub_ed833c() -> ! {
    todo!("0xed833c -[TFTimer .cxx_destruct]")
}

#[doc(alias = "+[TF_OpenUDID _setDict:forPasteboard:]")]
#[doc(alias = "+[TF_OpenUDID _setDict:forPasteboard:]")]
// 0xed8358 — +[TF_OpenUDID _setDict:forPasteboard:]
// type: void __cdecl(id, SEL, id, id)
pub fn stub_ed8358() -> ! {
    todo!("0xed8358 +[TF_OpenUDID _setDict:forPasteboard:]")
}

#[doc(alias = "+[TF_OpenUDID _getDictFromPasteboard:]")]
#[doc(alias = "+[TF_OpenUDID _getDictFromPasteboard:]")]
// 0xed839c — +[TF_OpenUDID _getDictFromPasteboard:]
// type: id __cdecl(id, SEL, id)
pub fn stub_ed839c() -> ! {
    todo!("0xed839c +[TF_OpenUDID _getDictFromPasteboard:]")
}

#[doc(alias = "+[TF_OpenUDID _generateFreshOpenUDID]")]
#[doc(alias = "+[TF_OpenUDID _generateFreshOpenUDID]")]
// 0xed84fc — +[TF_OpenUDID _generateFreshOpenUDID]
// type: id __cdecl(id, SEL)
pub fn stub_ed84fc() -> ! {
    todo!("0xed84fc +[TF_OpenUDID _generateFreshOpenUDID]")
}

#[doc(alias = "+[TF_OpenUDID value]")]
#[doc(alias = "+[TF_OpenUDID value]")]
// 0xed8618 — +[TF_OpenUDID value]
// type: id __cdecl(id, SEL)
pub fn stub_ed8618() -> ! {
    todo!("0xed8618 +[TF_OpenUDID value]")
}

#[doc(alias = "+[TF_OpenUDID valueWithError:]")]
#[doc(alias = "+[TF_OpenUDID valueWithError:]")]
// 0xed863c — +[TF_OpenUDID valueWithError:]
// type: id __cdecl(id, SEL, id *)
pub fn stub_ed863c() -> ! {
    todo!("0xed863c +[TF_OpenUDID valueWithError:]")
}

#[doc(alias = "+[TF_OpenUDID setOptOut:]")]
#[doc(alias = "+[TF_OpenUDID setOptOut:]")]
// 0xed8d20 — +[TF_OpenUDID setOptOut:]
// type: void __cdecl(id, SEL, char)
pub fn stub_ed8d20() -> ! {
    todo!("0xed8d20 +[TF_OpenUDID setOptOut:]")
}

#[doc(alias = "+[TFAppUpdater promptIfUpdateExists:]")]
#[doc(alias = "+[TFAppUpdater promptIfUpdateExists:]")]
// 0xed8e74 — +[TFAppUpdater promptIfUpdateExists:]
// type: void __cdecl(id, SEL, id)
pub fn stub_ed8e74() -> ! {
    todo!("0xed8e74 +[TFAppUpdater promptIfUpdateExists:]")
}

#[doc(alias = "___37+[TFAppUpdater promptIfUpdateExists:]_block_invoke")]
#[doc(alias = "___37+[TFAppUpdater promptIfUpdateExists:]_block_invoke")]
// 0xed8eec — ___37+[TFAppUpdater promptIfUpdateExists:]_block_invoke
pub fn stub_ed8eec() -> ! {
    todo!("0xed8eec ___37+[TFAppUpdater promptIfUpdateExists:]_block_invoke")
}

#[doc(alias = "___copy_helper_block__35")]
#[doc(alias = "___copy_helper_block__35")]
// 0xed8f2c — ___copy_helper_block__35
pub fn stub_ed8f2c() -> ! {
    todo!("0xed8f2c ___copy_helper_block__35")
}

#[doc(alias = "___destroy_helper_block__35")]
#[doc(alias = "___destroy_helper_block__35")]
// 0xed8f38 — ___destroy_helper_block__35
pub fn stub_ed8f38() -> ! {
    todo!("0xed8f38 ___destroy_helper_block__35")
}

#[doc(alias = "+[TFAppUpdater setInAppUpdatesDisabled:]")]
#[doc(alias = "+[TFAppUpdater setInAppUpdatesDisabled:]")]
// 0xed8f44 — +[TFAppUpdater setInAppUpdatesDisabled:]
// type: void __cdecl(id, SEL, char)
pub fn stub_ed8f44() -> ! {
    todo!("0xed8f44 +[TFAppUpdater setInAppUpdatesDisabled:]")
}

#[doc(alias = "___40+[TFAppUpdater setInAppUpdatesDisabled:]_block_invoke")]
#[doc(alias = "___40+[TFAppUpdater setInAppUpdatesDisabled:]_block_invoke")]
// 0xed8f9c — ___40+[TFAppUpdater setInAppUpdatesDisabled:]_block_invoke
pub fn stub_ed8f9c() -> ! {
    todo!("0xed8f9c ___40+[TFAppUpdater setInAppUpdatesDisabled:]_block_invoke")
}

#[doc(alias = "___copy_helper_block_8")]
#[doc(alias = "___copy_helper_block_8")]
// 0xed8fb8 — ___copy_helper_block_8
pub fn stub_ed8fb8() -> ! {
    todo!("0xed8fb8 ___copy_helper_block_8")
}

#[doc(alias = "___destroy_helper_block_9")]
#[doc(alias = "___destroy_helper_block_9")]
// 0xed8fbc — ___destroy_helper_block_9
pub fn stub_ed8fbc() -> ! {
    todo!("0xed8fbc ___destroy_helper_block_9")
}

#[doc(alias = "+[TFAppUpdater _setInAppUpdatesDisabledInternal:]")]
#[doc(alias = "+[TFAppUpdater _setInAppUpdatesDisabledInternal:]")]
// 0xed8fc0 — +[TFAppUpdater _setInAppUpdatesDisabledInternal:]
// type: void __cdecl(id, SEL, char)
pub fn stub_ed8fc0() -> ! {
    todo!("0xed8fc0 +[TFAppUpdater _setInAppUpdatesDisabledInternal:]")
}

#[doc(alias = "+[TFAppUpdater isUpdating]")]
#[doc(alias = "+[TFAppUpdater isUpdating]")]
// 0xed8fd0 — +[TFAppUpdater isUpdating]
// type: char __cdecl(id, SEL)
pub fn stub_ed8fd0() -> ! {
    todo!("0xed8fd0 +[TFAppUpdater isUpdating]")
}

#[doc(alias = "+[TFAppUpdater setIsUpdating:]")]
#[doc(alias = "+[TFAppUpdater setIsUpdating:]")]
// 0xed8fe0 — +[TFAppUpdater setIsUpdating:]
// type: void __cdecl(id, SEL, char)
pub fn stub_ed8fe0() -> ! {
    todo!("0xed8fe0 +[TFAppUpdater setIsUpdating:]")
}

#[doc(alias = "___30+[TFAppUpdater setIsUpdating:]_block_invoke")]
#[doc(alias = "___30+[TFAppUpdater setIsUpdating:]_block_invoke")]
// 0xed9034 — ___30+[TFAppUpdater setIsUpdating:]_block_invoke
pub fn stub_ed9034() -> ! {
    todo!("0xed9034 ___30+[TFAppUpdater setIsUpdating:]_block_invoke")
}

#[doc(alias = "+[TFAppUpdater shouldSkipVersion:]")]
#[doc(alias = "+[TFAppUpdater shouldSkipVersion:]")]
// 0xed9044 — +[TFAppUpdater shouldSkipVersion:]
// type: char __cdecl(id, SEL, id)
pub fn stub_ed9044() -> ! {
    todo!("0xed9044 +[TFAppUpdater shouldSkipVersion:]")
}

#[doc(alias = "+[TFAppUpdater skipVersion:]")]
#[doc(alias = "+[TFAppUpdater skipVersion:]")]
// 0xed90d4 — +[TFAppUpdater skipVersion:]
// type: void __cdecl(id, SEL, id)
pub fn stub_ed90d4() -> ! {
    todo!("0xed90d4 +[TFAppUpdater skipVersion:]")
}

#[doc(alias = "+[TFAppUpdater updateAlert:]")]
#[doc(alias = "+[TFAppUpdater updateAlert:]")]
// 0xed9150 — +[TFAppUpdater updateAlert:]
// type: id __cdecl(id, SEL, id)
pub fn stub_ed9150() -> ! {
    todo!("0xed9150 +[TFAppUpdater updateAlert:]")
}

#[doc(alias = "-[__TFAppUpdater_Helper init]")]
#[doc(alias = "-[__TFAppUpdater_Helper init]")]
// 0xed977c — -[__TFAppUpdater_Helper init]
// type: __TFAppUpdater_Helper *__cdecl(__TFAppUpdater_Helper *self, SEL)
pub fn stub_ed977c() -> ! {
    todo!("0xed977c -[__TFAppUpdater_Helper init]")
}

#[doc(alias = "-[__TFAppUpdater_Helper dealloc]")]
#[doc(alias = "-[__TFAppUpdater_Helper dealloc]")]
// 0xed9830 — -[__TFAppUpdater_Helper dealloc]
// type: void __cdecl(__TFAppUpdater_Helper *self, SEL)
pub fn stub_ed9830() -> ! {
    todo!("0xed9830 -[__TFAppUpdater_Helper dealloc]")
}

#[doc(alias = "-[__TFAppUpdater_Helper dismissAlert]")]
#[doc(alias = "-[__TFAppUpdater_Helper dismissAlert]")]
// 0xed98ac — -[__TFAppUpdater_Helper dismissAlert]
// type: void __cdecl(__TFAppUpdater_Helper *self, SEL)
pub fn stub_ed98ac() -> ! {
    todo!("0xed98ac -[__TFAppUpdater_Helper dismissAlert]")
}

#[doc(alias = "-[__TFAppUpdater_Helper alertView:clickedButtonAtIndex:]")]
#[doc(alias = "-[__TFAppUpdater_Helper alertView:clickedButtonAtIndex:]")]
// 0xed98e8 — -[__TFAppUpdater_Helper alertView:clickedButtonAtIndex:]
// type: void __cdecl(__TFAppUpdater_Helper *self, SEL, id, int)
pub fn stub_ed98e8() -> ! {
    todo!("0xed98e8 -[__TFAppUpdater_Helper alertView:clickedButtonAtIndex:]")
}

#[doc(alias = "-[__TFAppUpdater_Helper willPresentAlertView:]")]
#[doc(alias = "-[__TFAppUpdater_Helper willPresentAlertView:]")]
// 0xed9a84 — -[__TFAppUpdater_Helper willPresentAlertView:]
// type: void __cdecl(__TFAppUpdater_Helper *self, SEL, id)
pub fn stub_ed9a84() -> ! {
    todo!("0xed9a84 -[__TFAppUpdater_Helper willPresentAlertView:]")
}

#[doc(alias = "-[__TFAppUpdater_Helper fixMessageForOrientation:]")]
#[doc(alias = "-[__TFAppUpdater_Helper fixMessageForOrientation:]")]
// 0xed9b0c — -[__TFAppUpdater_Helper fixMessageForOrientation:]
// type: void __cdecl(__TFAppUpdater_Helper *self, SEL, int)
pub fn stub_ed9b0c() -> ! {
    todo!("0xed9b0c -[__TFAppUpdater_Helper fixMessageForOrientation:]")
}

#[doc(alias = "-[__TFAppUpdater_Helper orientationChange:]")]
#[doc(alias = "-[__TFAppUpdater_Helper orientationChange:]")]
// 0xed9bf0 — -[__TFAppUpdater_Helper orientationChange:]
// type: void __cdecl(__TFAppUpdater_Helper *self, SEL, id)
pub fn stub_ed9bf0() -> ! {
    todo!("0xed9bf0 -[__TFAppUpdater_Helper orientationChange:]")
}

#[doc(alias = "-[__TFAppUpdater_Helper info]")]
#[doc(alias = "-[__TFAppUpdater_Helper info]")]
// 0xed9c6c — -[__TFAppUpdater_Helper info]
// type: NSDictionary *__cdecl(__TFAppUpdater_Helper *self, SEL)
pub fn stub_ed9c6c() -> ! {
    todo!("0xed9c6c -[__TFAppUpdater_Helper info]")
}

#[doc(alias = "-[__TFAppUpdater_Helper setInfo:]")]
#[doc(alias = "-[__TFAppUpdater_Helper setInfo:]")]
// 0xed9c84 — -[__TFAppUpdater_Helper setInfo:]
// type: void __cdecl(__TFAppUpdater_Helper *self, SEL, id)
pub fn stub_ed9c84() -> ! {
    todo!("0xed9c84 -[__TFAppUpdater_Helper setInfo:]")
}

#[doc(alias = "-[__TFAppUpdater_Helper alert]")]
#[doc(alias = "-[__TFAppUpdater_Helper alert]")]
// 0xed9ca8 — -[__TFAppUpdater_Helper alert]
// type: UIAlertView *__cdecl(__TFAppUpdater_Helper *self, SEL)
pub fn stub_ed9ca8() -> ! {
    todo!("0xed9ca8 -[__TFAppUpdater_Helper alert]")
}

#[doc(alias = "-[__TFAppUpdater_Helper setAlert:]")]
#[doc(alias = "-[__TFAppUpdater_Helper setAlert:]")]
// 0xed9cb8 — -[__TFAppUpdater_Helper setAlert:]
// type: void __cdecl(__TFAppUpdater_Helper *self, SEL, id)
pub fn stub_ed9cb8() -> ! {
    todo!("0xed9cb8 -[__TFAppUpdater_Helper setAlert:]")
}

#[doc(alias = "-[__TFAppUpdater_Helper .cxx_destruct]")]
#[doc(alias = "-[__TFAppUpdater_Helper .cxx_destruct]")]
// 0xed9cc8 — -[__TFAppUpdater_Helper .cxx_destruct]
// type: void __cdecl(__TFAppUpdater_Helper *self, SEL)
pub fn stub_ed9cc8() -> ! {
    todo!("0xed9cc8 -[__TFAppUpdater_Helper .cxx_destruct]")
}

#[doc(alias = "-[BugSenseController crashReporter]")]
#[doc(alias = "-[BugSenseController crashReporter]")]
// 0xed9cf8 — -[BugSenseController crashReporter]
// type: id __cdecl(BugSenseController *self, SEL)
pub fn stub_ed9cf8() -> ! {
    todo!("0xed9cf8 -[BugSenseController crashReporter]")
}

#[doc(alias = "-[BugSenseController crashReport]")]
#[doc(alias = "-[BugSenseController crashReport]")]
// 0xed9d38 — -[BugSenseController crashReport]
// type: id __cdecl(BugSenseController *self, SEL)
pub fn stub_ed9d38() -> ! {
    todo!("0xed9d38 -[BugSenseController crashReport]")
}

#[doc(alias = "-[BugSenseController dispatchQueue]")]
#[doc(alias = "-[BugSenseController dispatchQueue]")]
// 0xed9e2c — -[BugSenseController dispatchQueue]
// type: dispatch_queue_s *__cdecl(BugSenseController *self, SEL)
pub fn stub_ed9e2c() -> ! {
    todo!("0xed9e2c -[BugSenseController dispatchQueue]")
}

#[doc(alias = "-[BugSenseController operationQueue]")]
#[doc(alias = "-[BugSenseController operationQueue]")]
// 0xed9e60 — -[BugSenseController operationQueue]
// type: id __cdecl(BugSenseController *self, SEL)
pub fn stub_ed9e60() -> ! {
    todo!("0xed9e60 -[BugSenseController operationQueue]")
}

#[doc(alias = "-[BugSenseController sessionStartTimestampInMilliseconds]")]
#[doc(alias = "-[BugSenseController sessionStartTimestampInMilliseconds]")]
// 0xed9eb0 — -[BugSenseController sessionStartTimestampInMilliseconds]
// type: unsigned __int64 __cdecl(BugSenseController *self, SEL)
pub fn stub_ed9eb0() -> ! {
    todo!("0xed9eb0 -[BugSenseController sessionStartTimestampInMilliseconds]")
}

#[doc(alias = "+[BugSenseController openUDID]")]
#[doc(alias = "+[BugSenseController openUDID]")]
// 0xed9ec8 — +[BugSenseController openUDID]
// type: id __cdecl(id, SEL)
pub fn stub_ed9ec8() -> ! {
    todo!("0xed9ec8 +[BugSenseController openUDID]")
}

#[doc(alias = "+[BugSenseController endpointURL]")]
#[doc(alias = "+[BugSenseController endpointURL]")]
// 0xed9ee8 — +[BugSenseController endpointURL]
// type: id __cdecl(id, SEL)
pub fn stub_ed9ee8() -> ! {
    todo!("0xed9ee8 +[BugSenseController endpointURL]")
}

#[doc(alias = "+[BugSenseController apiKey]")]
#[doc(alias = "+[BugSenseController apiKey]")]
// 0xed9f08 — +[BugSenseController apiKey]
// type: id __cdecl(id, SEL)
pub fn stub_ed9f08() -> ! {
    todo!("0xed9f08 +[BugSenseController apiKey]")
}

#[doc(alias = "+[BugSenseController userIdentifier]")]
#[doc(alias = "+[BugSenseController userIdentifier]")]
// 0xed9f18 — +[BugSenseController userIdentifier]
// type: id __cdecl(id, SEL)
pub fn stub_ed9f18() -> ! {
    todo!("0xed9f18 +[BugSenseController userIdentifier]")
}

#[doc(alias = "+[BugSenseController setUsesProxy:]")]
#[doc(alias = "+[BugSenseController setUsesProxy:]")]
// 0xed9f48 — +[BugSenseController setUsesProxy:]
// type: void __cdecl(id, SEL, char)
pub fn stub_ed9f48() -> ! {
    todo!("0xed9f48 +[BugSenseController setUsesProxy:]")
}

#[doc(alias = "+[BugSenseController usesProxy]")]
#[doc(alias = "+[BugSenseController usesProxy]")]
// 0xed9f58 — +[BugSenseController usesProxy]
// type: char __cdecl(id, SEL)
pub fn stub_ed9f58() -> ! {
    todo!("0xed9f58 +[BugSenseController usesProxy]")
}

#[doc(alias = "+[BugSenseController setLogMessagesCount:]")]
#[doc(alias = "+[BugSenseController setLogMessagesCount:]")]
// 0xed9f68 — +[BugSenseController setLogMessagesCount:]
// type: void __cdecl(id, SEL, unsigned int)
pub fn stub_ed9f68() -> ! {
    todo!("0xed9f68 +[BugSenseController setLogMessagesCount:]")
}

#[doc(alias = "+[BugSenseController setLogMessagesLevel:]")]
#[doc(alias = "+[BugSenseController setLogMessagesLevel:]")]
// 0xed9f78 — +[BugSenseController setLogMessagesLevel:]
// type: void __cdecl(id, SEL, unsigned int)
pub fn stub_ed9f78() -> ! {
    todo!("0xed9f78 +[BugSenseController setLogMessagesLevel:]")
}

#[doc(alias = "+[BugSenseController setFixNotificationsTitle:message:]")]
#[doc(alias = "+[BugSenseController setFixNotificationsTitle:message:]")]
// 0xed9f88 — +[BugSenseController setFixNotificationsTitle:message:]
// type: void __cdecl(id, SEL, id, id)
pub fn stub_ed9f88() -> ! {
    todo!("0xed9f88 +[BugSenseController setFixNotificationsTitle:message:]")
}

#[doc(alias = "+[BugSenseController setUserIdentifier:]")]
#[doc(alias = "+[BugSenseController setUserIdentifier:]")]
// 0xed9fc8 — +[BugSenseController setUserIdentifier:]
// type: void __cdecl(id, SEL, id)
pub fn stub_ed9fc8() -> ! {
    todo!("0xed9fc8 +[BugSenseController setUserIdentifier:]")
}

#[doc(alias = "_post_crash_callback")]
#[doc(alias = "_post_crash_callback")]
// 0xeda014 — _post_crash_callback
pub fn stub_eda014() -> ! {
    todo!("0xeda014 _post_crash_callback")
}

#[doc(alias = "-[BugSenseController performPostCrashOperations]")]
#[doc(alias = "-[BugSenseController performPostCrashOperations]")]
// 0xeda0b0 — -[BugSenseController performPostCrashOperations]
// type: void __cdecl(BugSenseController *self, SEL)
pub fn stub_eda0b0() -> ! {
    todo!("0xeda0b0 -[BugSenseController performPostCrashOperations]")
}

#[doc(alias = "+[BugSenseController logException:withExtraData:]")]
#[doc(alias = "+[BugSenseController logException:withExtraData:]")]
// 0xeda268 — +[BugSenseController logException:withExtraData:]
// type: char __cdecl(id, SEL, id, id)
pub fn stub_eda268() -> ! {
    todo!("0xeda268 +[BugSenseController logException:withExtraData:]")
}

#[doc(alias = "_get_used_memory")]
#[doc(alias = "_get_used_memory")]
// 0xeda3f0 — _get_used_memory
pub fn stub_eda3f0() -> ! {
    todo!("0xeda3f0 _get_used_memory")
}

#[doc(alias = "-[BugSenseController initiateReporting]")]
#[doc(alias = "-[BugSenseController initiateReporting]")]
// 0xeda420 — -[BugSenseController initiateReporting]
// type: void __cdecl(BugSenseController *self, SEL)
pub fn stub_eda420() -> ! {
    todo!("0xeda420 -[BugSenseController initiateReporting]")
}

#[doc(alias = "___39-[BugSenseController initiateReporting]_block_invoke")]
#[doc(alias = "___39-[BugSenseController initiateReporting]_block_invoke")]
// 0xeda510 — ___39-[BugSenseController initiateReporting]_block_invoke
pub fn stub_eda510() -> ! {
    todo!("0xeda510 ___39-[BugSenseController initiateReporting]_block_invoke")
}

#[doc(alias = "___copy_helper_block__36")]
#[doc(alias = "___copy_helper_block__36")]
// 0xeda540 — ___copy_helper_block__36
pub fn stub_eda540() -> ! {
    todo!("0xeda540 ___copy_helper_block__36")
}

#[doc(alias = "___destroy_helper_block__36")]
#[doc(alias = "___destroy_helper_block__36")]
// 0xeda550 — ___destroy_helper_block__36
pub fn stub_eda550() -> ! {
    todo!("0xeda550 ___destroy_helper_block__36")
}

#[doc(alias = "+[BugSenseController startSession]")]
#[doc(alias = "+[BugSenseController startSession]")]
// 0xeda654 — +[BugSenseController startSession]
// type: void __cdecl(id, SEL)
pub fn stub_eda654() -> ! {
    todo!("0xeda654 +[BugSenseController startSession]")
}

#[doc(alias = "+[BugSenseController stopSession]")]
#[doc(alias = "+[BugSenseController stopSession]")]
// 0xeda69c — +[BugSenseController stopSession]
// type: void __cdecl(id, SEL)
pub fn stub_eda69c() -> ! {
    todo!("0xeda69c +[BugSenseController stopSession]")
}

#[doc(alias = "___33+[BugSenseController stopSession]_block_invoke")]
#[doc(alias = "___33+[BugSenseController stopSession]_block_invoke")]
// 0xeda794 — ___33+[BugSenseController stopSession]_block_invoke
pub fn stub_eda794() -> ! {
    todo!("0xeda794 ___33+[BugSenseController stopSession]_block_invoke")
}

#[doc(alias = "___copy_helper_block_109_0")]
#[doc(alias = "___copy_helper_block_109_0")]
// 0xeda7e4 — ___copy_helper_block_109_0
pub fn stub_eda7e4() -> ! {
    todo!("0xeda7e4 ___copy_helper_block_109_0")
}

#[doc(alias = "___destroy_helper_block_110_0")]
#[doc(alias = "___destroy_helper_block_110_0")]
// 0xeda7f4 — ___destroy_helper_block_110_0
pub fn stub_eda7f4() -> ! {
    todo!("0xeda7f4 ___destroy_helper_block_110_0")
}

#[doc(alias = "___33+[BugSenseController stopSession]_block_invoke113")]
#[doc(alias = "___33+[BugSenseController stopSession]_block_invoke113")]
// 0xeda804 — ___33+[BugSenseController stopSession]_block_invoke113
pub fn stub_eda804() -> ! {
    todo!("0xeda804 ___33+[BugSenseController stopSession]_block_invoke113")
}

#[doc(alias = "___copy_helper_block_119_0")]
#[doc(alias = "___copy_helper_block_119_0")]
// 0xeda8e4 — ___copy_helper_block_119_0
pub fn stub_eda8e4() -> ! {
    todo!("0xeda8e4 ___copy_helper_block_119_0")
}

#[doc(alias = "___destroy_helper_block_120_0")]
#[doc(alias = "___destroy_helper_block_120_0")]
// 0xeda8f4 — ___destroy_helper_block_120_0
pub fn stub_eda8f4() -> ! {
    todo!("0xeda8f4 ___destroy_helper_block_120_0")
}

#[doc(alias = "+[BugSenseController sendEventWithTag:]")]
#[doc(alias = "+[BugSenseController sendEventWithTag:]")]
// 0xeda904 — +[BugSenseController sendEventWithTag:]
// type: char __cdecl(id, SEL, id)
pub fn stub_eda904() -> ! {
    todo!("0xeda904 +[BugSenseController sendEventWithTag:]")
}

#[doc(alias = "___39+[BugSenseController sendEventWithTag:]_block_invoke")]
#[doc(alias = "___39+[BugSenseController sendEventWithTag:]_block_invoke")]
// 0xedaa2c — ___39+[BugSenseController sendEventWithTag:]_block_invoke
pub fn stub_edaa2c() -> ! {
    todo!("0xedaa2c ___39+[BugSenseController sendEventWithTag:]_block_invoke")
}
