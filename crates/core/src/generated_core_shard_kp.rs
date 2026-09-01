//! core shard kp — 120 stubs EA-sorted asc global gap filler not yet in core (fallback filter, fills hole 0xeb58b8 before ko).
//! Source: `ida/export.json` (85545 funcs) EA-sorted asc, next 120 smallest not yet in rbx_core (hole 0xeb58b8..0xebc8ac before ko 0xebc958; 33260 filtered, 28437 distinct, 4823 remaining before -> 4703 after, rbx_core::SharedPtr not boost).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + #[doc(alias = mangled)] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "-[GAITransactionItem productCategory]")]
#[doc(alias = "-[GAITransactionItem productCategory]")]
// 0xeb58b8 — -[GAITransactionItem productCategory]
// type: NSString *__cdecl(GAITransactionItem *self, SEL)
pub fn stub_0xeb58b8() -> ! {
    todo!("0xeb58b8 -[GAITransactionItem productCategory]")
}

#[doc(alias = "-[GAITransactionItem setProductCategory:]")]
#[doc(alias = "-[GAITransactionItem setProductCategory:]")]
// 0xeb58d0 — -[GAITransactionItem setProductCategory:]
// type: void __cdecl(GAITransactionItem *self, SEL, id)
pub fn stub_0xeb58d0() -> ! {
    todo!("0xeb58d0 -[GAITransactionItem setProductCategory:]")
}

#[doc(alias = "-[GAITransactionItem priceMicros]")]
#[doc(alias = "-[GAITransactionItem priceMicros]")]
// 0xeb58f4 — -[GAITransactionItem priceMicros]
// type: signed __int64 __cdecl(GAITransactionItem *self, SEL)
pub fn stub_0xeb58f4() -> ! {
    todo!("0xeb58f4 -[GAITransactionItem priceMicros]")
}

#[doc(alias = "-[GAITransactionItem setPriceMicros:]")]
#[doc(alias = "-[GAITransactionItem setPriceMicros:]")]
// 0xeb590c — -[GAITransactionItem setPriceMicros:]
// type: void __cdecl(GAITransactionItem *self, SEL, signed __int64)
pub fn stub_0xeb590c() -> ! {
    todo!("0xeb590c -[GAITransactionItem setPriceMicros:]")
}

#[doc(alias = "-[GAITransactionItem quantity]")]
#[doc(alias = "-[GAITransactionItem quantity]")]
// 0xeb5920 — -[GAITransactionItem quantity]
// type: int __cdecl(GAITransactionItem *self, SEL)
pub fn stub_0xeb5920() -> ! {
    todo!("0xeb5920 -[GAITransactionItem quantity]")
}

#[doc(alias = "-[GAITransactionItem setQuantity:]")]
#[doc(alias = "-[GAITransactionItem setQuantity:]")]
// 0xeb5930 — -[GAITransactionItem setQuantity:]
// type: void __cdecl(GAITransactionItem *self, SEL, int)
pub fn stub_0xeb5930() -> ! {
    todo!("0xeb5930 -[GAITransactionItem setQuantity:]")
}

#[doc(alias = "-[GAITrackerImpl anonymize]")]
#[doc(alias = "-[GAITrackerImpl anonymize]")]
// 0xeb5940 — -[GAITrackerImpl anonymize]
// type: char __cdecl(GAITrackerImpl *self, SEL)
pub fn stub_0xeb5940() -> ! {
    todo!("0xeb5940 -[GAITrackerImpl anonymize]")
}

#[doc(alias = "-[GAITrackerImpl setAnonymize:]")]
#[doc(alias = "-[GAITrackerImpl setAnonymize:]")]
// 0xeb5a08 — -[GAITrackerImpl setAnonymize:]
// type: void __cdecl(GAITrackerImpl *self, SEL, char)
pub fn stub_0xeb5a08() -> ! {
    todo!("0xeb5a08 -[GAITrackerImpl setAnonymize:]")
}

#[doc(alias = "-[GAITrackerImpl useHttps]")]
#[doc(alias = "-[GAITrackerImpl useHttps]")]
// 0xeb5ad0 — -[GAITrackerImpl useHttps]
// type: char __cdecl(GAITrackerImpl *self, SEL)
pub fn stub_0xeb5ad0() -> ! {
    todo!("0xeb5ad0 -[GAITrackerImpl useHttps]")
}

#[doc(alias = "-[GAITrackerImpl setUseHttps:]")]
#[doc(alias = "-[GAITrackerImpl setUseHttps:]")]
// 0xeb5b98 — -[GAITrackerImpl setUseHttps:]
// type: void __cdecl(GAITrackerImpl *self, SEL, char)
pub fn stub_0xeb5b98() -> ! {
    todo!("0xeb5b98 -[GAITrackerImpl setUseHttps:]")
}

#[doc(alias = "-[GAITrackerImpl httpDispatchUrl]")]
#[doc(alias = "-[GAITrackerImpl httpDispatchUrl]")]
// 0xeb5c60 — -[GAITrackerImpl httpDispatchUrl]
// type: id __cdecl(GAITrackerImpl *self, SEL)
pub fn stub_0xeb5c60() -> ! {
    todo!("0xeb5c60 -[GAITrackerImpl httpDispatchUrl]")
}

#[doc(alias = "-[GAITrackerImpl setHttpDispatchUrl:]")]
#[doc(alias = "-[GAITrackerImpl setHttpDispatchUrl:]")]
// 0xeb5d28 — -[GAITrackerImpl setHttpDispatchUrl:]
// type: void __cdecl(GAITrackerImpl *self, SEL, id)
pub fn stub_0xeb5d28() -> ! {
    todo!("0xeb5d28 -[GAITrackerImpl setHttpDispatchUrl:]")
}

#[doc(alias = "-[GAITrackerImpl httpsDispatchUrl]")]
#[doc(alias = "-[GAITrackerImpl httpsDispatchUrl]")]
// 0xeb5df0 — -[GAITrackerImpl httpsDispatchUrl]
// type: id __cdecl(GAITrackerImpl *self, SEL)
pub fn stub_0xeb5df0() -> ! {
    todo!("0xeb5df0 -[GAITrackerImpl httpsDispatchUrl]")
}

#[doc(alias = "-[GAITrackerImpl setHttpsDispatchUrl:]")]
#[doc(alias = "-[GAITrackerImpl setHttpsDispatchUrl:]")]
// 0xeb5eb8 — -[GAITrackerImpl setHttpsDispatchUrl:]
// type: void __cdecl(GAITrackerImpl *self, SEL, id)
pub fn stub_0xeb5eb8() -> ! {
    todo!("0xeb5eb8 -[GAITrackerImpl setHttpsDispatchUrl:]")
}

#[doc(alias = "-[GAITrackerImpl sampleRate]")]
#[doc(alias = "-[GAITrackerImpl sampleRate]")]
// 0xeb5f80 — -[GAITrackerImpl sampleRate]
// type: double __cdecl(GAITrackerImpl *self, SEL)
pub fn stub_0xeb5f80() -> ! {
    todo!("0xeb5f80 -[GAITrackerImpl sampleRate]")
}

#[doc(alias = "-[GAITrackerImpl setSampleRate:]")]
#[doc(alias = "-[GAITrackerImpl setSampleRate:]")]
// 0xeb604c — -[GAITrackerImpl setSampleRate:]
// type: void __cdecl(GAITrackerImpl *self, SEL, double)
pub fn stub_0xeb604c() -> ! {
    todo!("0xeb604c -[GAITrackerImpl setSampleRate:]")
}

#[doc(alias = "-[GAITrackerImpl trackingId]")]
#[doc(alias = "-[GAITrackerImpl trackingId]")]
// 0xeb6118 — -[GAITrackerImpl trackingId]
// type: NSString *__cdecl(GAITrackerImpl *self, SEL)
pub fn stub_0xeb6118() -> ! {
    todo!("0xeb6118 -[GAITrackerImpl trackingId]")
}

#[doc(alias = "-[GAITrackerImpl appName]")]
#[doc(alias = "-[GAITrackerImpl appName]")]
// 0xeb61e0 — -[GAITrackerImpl appName]
// type: NSString *__cdecl(GAITrackerImpl *self, SEL)
pub fn stub_0xeb61e0() -> ! {
    todo!("0xeb61e0 -[GAITrackerImpl appName]")
}

#[doc(alias = "-[GAITrackerImpl setAppName:]")]
#[doc(alias = "-[GAITrackerImpl setAppName:]")]
// 0xeb62a8 — -[GAITrackerImpl setAppName:]
// type: void __cdecl(GAITrackerImpl *self, SEL, id)
pub fn stub_0xeb62a8() -> ! {
    todo!("0xeb62a8 -[GAITrackerImpl setAppName:]")
}

#[doc(alias = "-[GAITrackerImpl appId]")]
#[doc(alias = "-[GAITrackerImpl appId]")]
// 0xeb645c — -[GAITrackerImpl appId]
// type: NSString *__cdecl(GAITrackerImpl *self, SEL)
pub fn stub_0xeb645c() -> ! {
    todo!("0xeb645c -[GAITrackerImpl appId]")
}

#[doc(alias = "-[GAITrackerImpl setAppId:]")]
#[doc(alias = "-[GAITrackerImpl setAppId:]")]
// 0xeb6524 — -[GAITrackerImpl setAppId:]
// type: void __cdecl(GAITrackerImpl *self, SEL, id)
pub fn stub_0xeb6524() -> ! {
    todo!("0xeb6524 -[GAITrackerImpl setAppId:]")
}

#[doc(alias = "-[GAITrackerImpl appVersion]")]
#[doc(alias = "-[GAITrackerImpl appVersion]")]
// 0xeb65ec — -[GAITrackerImpl appVersion]
// type: NSString *__cdecl(GAITrackerImpl *self, SEL)
pub fn stub_0xeb65ec() -> ! {
    todo!("0xeb65ec -[GAITrackerImpl appVersion]")
}

#[doc(alias = "-[GAITrackerImpl setAppVersion:]")]
#[doc(alias = "-[GAITrackerImpl setAppVersion:]")]
// 0xeb66b4 — -[GAITrackerImpl setAppVersion:]
// type: void __cdecl(GAITrackerImpl *self, SEL, id)
pub fn stub_0xeb66b4() -> ! {
    todo!("0xeb66b4 -[GAITrackerImpl setAppVersion:]")
}

#[doc(alias = "-[GAITrackerImpl clientId]")]
#[doc(alias = "-[GAITrackerImpl clientId]")]
// 0xeb67f4 — -[GAITrackerImpl clientId]
// type: NSString *__cdecl(GAITrackerImpl *self, SEL)
pub fn stub_0xeb67f4() -> ! {
    todo!("0xeb67f4 -[GAITrackerImpl clientId]")
}

#[doc(alias = "-[GAITrackerImpl appScreen]")]
#[doc(alias = "-[GAITrackerImpl appScreen]")]
// 0xeb6818 — -[GAITrackerImpl appScreen]
// type: NSString *__cdecl(GAITrackerImpl *self, SEL)
pub fn stub_0xeb6818() -> ! {
    todo!("0xeb6818 -[GAITrackerImpl appScreen]")
}

#[doc(alias = "-[GAITrackerImpl setAppScreen:]")]
#[doc(alias = "-[GAITrackerImpl setAppScreen:]")]
// 0xeb68e0 — -[GAITrackerImpl setAppScreen:]
// type: void __cdecl(GAITrackerImpl *self, SEL, id)
pub fn stub_0xeb68e0() -> ! {
    todo!("0xeb68e0 -[GAITrackerImpl setAppScreen:]")
}

#[doc(alias = "-[GAITrackerImpl referrerUrl]")]
#[doc(alias = "-[GAITrackerImpl referrerUrl]")]
// 0xeb6a0c — -[GAITrackerImpl referrerUrl]
// type: NSString *__cdecl(GAITrackerImpl *self, SEL)
pub fn stub_0xeb6a0c() -> ! {
    todo!("0xeb6a0c -[GAITrackerImpl referrerUrl]")
}

#[doc(alias = "-[GAITrackerImpl setReferrerUrl:]")]
#[doc(alias = "-[GAITrackerImpl setReferrerUrl:]")]
// 0xeb6ad4 — -[GAITrackerImpl setReferrerUrl:]
// type: void __cdecl(GAITrackerImpl *self, SEL, id)
pub fn stub_0xeb6ad4() -> ! {
    todo!("0xeb6ad4 -[GAITrackerImpl setReferrerUrl:]")
}

#[doc(alias = "-[GAITrackerImpl campaignUrl]")]
#[doc(alias = "-[GAITrackerImpl campaignUrl]")]
// 0xeb6b9c — -[GAITrackerImpl campaignUrl]
// type: NSString *__cdecl(GAITrackerImpl *self, SEL)
pub fn stub_0xeb6b9c() -> ! {
    todo!("0xeb6b9c -[GAITrackerImpl campaignUrl]")
}

#[doc(alias = "-[GAITrackerImpl setCampaignUrl:]")]
#[doc(alias = "-[GAITrackerImpl setCampaignUrl:]")]
// 0xeb6c64 — -[GAITrackerImpl setCampaignUrl:]
// type: void __cdecl(GAITrackerImpl *self, SEL, id)
pub fn stub_0xeb6c64() -> ! {
    todo!("0xeb6c64 -[GAITrackerImpl setCampaignUrl:]")
}

#[doc(alias = "-[GAITrackerImpl sessionStart]")]
#[doc(alias = "-[GAITrackerImpl sessionStart]")]
// 0xeb6d2c — -[GAITrackerImpl sessionStart]
// type: char __cdecl(GAITrackerImpl *self, SEL)
pub fn stub_0xeb6d2c() -> ! {
    todo!("0xeb6d2c -[GAITrackerImpl sessionStart]")
}

#[doc(alias = "-[GAITrackerImpl setSessionStart:]")]
#[doc(alias = "-[GAITrackerImpl setSessionStart:]")]
// 0xeb6e18 — -[GAITrackerImpl setSessionStart:]
// type: void __cdecl(GAITrackerImpl *self, SEL, char)
pub fn stub_0xeb6e18() -> ! {
    todo!("0xeb6e18 -[GAITrackerImpl setSessionStart:]")
}

#[doc(alias = "-[GAITrackerImpl sessionTimeout]")]
#[doc(alias = "-[GAITrackerImpl sessionTimeout]")]
// 0xeb6ef4 — -[GAITrackerImpl sessionTimeout]
// type: double __cdecl(GAITrackerImpl *self, SEL)
pub fn stub_0xeb6ef4() -> ! {
    todo!("0xeb6ef4 -[GAITrackerImpl sessionTimeout]")
}

#[doc(alias = "-[GAITrackerImpl setSessionTimeout:]")]
#[doc(alias = "-[GAITrackerImpl setSessionTimeout:]")]
// 0xeb6f24 — -[GAITrackerImpl setSessionTimeout:]
// type: void __cdecl(GAITrackerImpl *self, SEL, double)
pub fn stub_0xeb6f24() -> ! {
    todo!("0xeb6f24 -[GAITrackerImpl setSessionTimeout:]")
}

#[doc(alias = "-[GAITrackerImpl init]")]
#[doc(alias = "-[GAITrackerImpl init]")]
// 0xeb6f4c — -[GAITrackerImpl init]
// type: GAITrackerImpl *__cdecl(GAITrackerImpl *self, SEL)
pub fn stub_0xeb6f4c() -> ! {
    todo!("0xeb6f4c -[GAITrackerImpl init]")
}

#[doc(alias = "-[GAITrackerImpl initWithDispatcher:trackingId:appName:appVersion:]")]
#[doc(alias = "-[GAITrackerImpl initWithDispatcher:trackingId:appName:appVersion:]")]
// 0xeb6fa8 — -[GAITrackerImpl initWithDispatcher:trackingId:appName:appVersion:]
// type: GAITrackerImpl *__cdecl(GAITrackerImpl *self, SEL, id, id, id, id)
pub fn stub_0xeb6fa8() -> ! {
    todo!("0xeb6fa8 -[GAITrackerImpl initWithDispatcher:trackingId:appName:appVersion:]")
}

#[doc(alias = "-[GAITrackerImpl dealloc]")]
#[doc(alias = "-[GAITrackerImpl dealloc]")]
// 0xeb72e0 — -[GAITrackerImpl dealloc]
// type: void __cdecl(GAITrackerImpl *self, SEL)
pub fn stub_0xeb72e0() -> ! {
    todo!("0xeb72e0 -[GAITrackerImpl dealloc]")
}

#[doc(alias = "+[GAITrackerImpl trackerWithDispatcher:trackingId:appName:appVersion:]")]
#[doc(alias = "+[GAITrackerImpl trackerWithDispatcher:trackingId:appName:appVersion:]")]
// 0xeb7370 — +[GAITrackerImpl trackerWithDispatcher:trackingId:appName:appVersion:]
// type: id __cdecl(id, SEL, id, id, id, id)
pub fn stub_0xeb7370() -> ! {
    todo!("0xeb7370 +[GAITrackerImpl trackerWithDispatcher:trackingId:appName:appVersion:]")
}

#[doc(alias = "-[GAITrackerImpl queueDispatch:]")]
#[doc(alias = "-[GAITrackerImpl queueDispatch:]")]
// 0xeb73c8 — -[GAITrackerImpl queueDispatch:]
// type: void __cdecl(GAITrackerImpl *self, SEL, id)
pub fn stub_0xeb73c8() -> ! {
    todo!("0xeb73c8 -[GAITrackerImpl queueDispatch:]")
}

#[doc(alias = "-[GAITrackerImpl shouldTrack]")]
#[doc(alias = "-[GAITrackerImpl shouldTrack]")]
// 0xeb7430 — -[GAITrackerImpl shouldTrack]
// type: char __cdecl(GAITrackerImpl *self, SEL)
pub fn stub_0xeb7430() -> ! {
    todo!("0xeb7430 -[GAITrackerImpl shouldTrack]")
}

#[doc(alias = "-[GAITrackerImpl updateAdParameters]")]
#[doc(alias = "-[GAITrackerImpl updateAdParameters]")]
// 0xeb7454 — -[GAITrackerImpl updateAdParameters]
// type: void __cdecl(GAITrackerImpl *self, SEL)
pub fn stub_0xeb7454() -> ! {
    todo!("0xeb7454 -[GAITrackerImpl updateAdParameters]")
}

#[doc(alias = "-[GAITrackerImpl shouldThrottle]")]
#[doc(alias = "-[GAITrackerImpl shouldThrottle]")]
// 0xeb74f0 — -[GAITrackerImpl shouldThrottle]
// type: char __cdecl(GAITrackerImpl *self, SEL)
pub fn stub_0xeb74f0() -> ! {
    todo!("0xeb74f0 -[GAITrackerImpl shouldThrottle]")
}

#[doc(alias = "-[GAITrackerImpl trackView]")]
#[doc(alias = "-[GAITrackerImpl trackView]")]
// 0xeb75c0 — -[GAITrackerImpl trackView]
// type: char __cdecl(GAITrackerImpl *self, SEL)
pub fn stub_0xeb75c0() -> ! {
    todo!("0xeb75c0 -[GAITrackerImpl trackView]")
}

#[doc(alias = "-[GAITrackerImpl sendView]")]
#[doc(alias = "-[GAITrackerImpl sendView]")]
// 0xeb75d8 — -[GAITrackerImpl sendView]
// type: char __cdecl(GAITrackerImpl *self, SEL)
pub fn stub_0xeb75d8() -> ! {
    todo!("0xeb75d8 -[GAITrackerImpl sendView]")
}

#[doc(alias = "-[GAITrackerImpl trackView:]")]
#[doc(alias = "-[GAITrackerImpl trackView:]")]
// 0xeb78e8 — -[GAITrackerImpl trackView:]
// type: char __cdecl(GAITrackerImpl *self, SEL, id)
pub fn stub_0xeb78e8() -> ! {
    todo!("0xeb78e8 -[GAITrackerImpl trackView:]")
}

#[doc(alias = "-[GAITrackerImpl sendView:]")]
#[doc(alias = "-[GAITrackerImpl sendView:]")]
// 0xeb7900 — -[GAITrackerImpl sendView:]
// type: char __cdecl(GAITrackerImpl *self, SEL, id)
pub fn stub_0xeb7900() -> ! {
    todo!("0xeb7900 -[GAITrackerImpl sendView:]")
}

#[doc(alias = "-[GAITrackerImpl trackEventWithCategory:withAction:withLabel:withValue:]")]
#[doc(alias = "-[GAITrackerImpl trackEventWithCategory:withAction:withLabel:withValue:]")]
// 0xeb7a38 — -[GAITrackerImpl trackEventWithCategory:withAction:withLabel:withValue:]
// type: char __cdecl(GAITrackerImpl *self, SEL, id, id, id, id)
pub fn stub_0xeb7a38() -> ! {
    todo!("0xeb7a38 -[GAITrackerImpl trackEventWithCategory:withAction:withLabel:withValue:]")
}

#[doc(alias = "-[GAITrackerImpl sendEventWithCategory:withAction:withLabel:withValue:]")]
#[doc(alias = "-[GAITrackerImpl sendEventWithCategory:withAction:withLabel:withValue:]")]
// 0xeb7a5c — -[GAITrackerImpl sendEventWithCategory:withAction:withLabel:withValue:]
// type: char __cdecl(GAITrackerImpl *self, SEL, id, id, id, id)
pub fn stub_0xeb7a5c() -> ! {
    todo!("0xeb7a5c -[GAITrackerImpl sendEventWithCategory:withAction:withLabel:withValue:]")
}

#[doc(alias = "-[GAITrackerImpl trackTransaction:]")]
#[doc(alias = "-[GAITrackerImpl trackTransaction:]")]
// 0xeb7d0c — -[GAITrackerImpl trackTransaction:]
// type: char __cdecl(GAITrackerImpl *self, SEL, id)
pub fn stub_0xeb7d0c() -> ! {
    todo!("0xeb7d0c -[GAITrackerImpl trackTransaction:]")
}

#[doc(alias = "-[GAITrackerImpl sendTransaction:]")]
#[doc(alias = "-[GAITrackerImpl sendTransaction:]")]
// 0xeb7d24 — -[GAITrackerImpl sendTransaction:]
// type: char __cdecl(GAITrackerImpl *self, SEL, id)
pub fn stub_0xeb7d24() -> ! {
    todo!("0xeb7d24 -[GAITrackerImpl sendTransaction:]")
}

#[doc(alias = "-[GAITrackerImpl trackException:withNSException:]")]
#[doc(alias = "-[GAITrackerImpl trackException:withNSException:]")]
// 0xeb8598 — -[GAITrackerImpl trackException:withNSException:]
// type: char __cdecl(GAITrackerImpl *self, SEL, char, id)
pub fn stub_0xeb8598() -> ! {
    todo!("0xeb8598 -[GAITrackerImpl trackException:withNSException:]")
}

#[doc(alias = "-[GAITrackerImpl sendException:withNSException:]")]
#[doc(alias = "-[GAITrackerImpl sendException:withNSException:]")]
// 0xeb85b0 — -[GAITrackerImpl sendException:withNSException:]
// type: char __cdecl(GAITrackerImpl *self, SEL, char, id)
pub fn stub_0xeb85b0() -> ! {
    todo!("0xeb85b0 -[GAITrackerImpl sendException:withNSException:]")
}

#[doc(alias = "-[GAITrackerImpl trackException:withNSError:]")]
#[doc(alias = "-[GAITrackerImpl trackException:withNSError:]")]
// 0xeb85f0 — -[GAITrackerImpl trackException:withNSError:]
// type: char __cdecl(GAITrackerImpl *self, SEL, char, id)
pub fn stub_0xeb85f0() -> ! {
    todo!("0xeb85f0 -[GAITrackerImpl trackException:withNSError:]")
}

#[doc(alias = "-[GAITrackerImpl sendException:withNSError:]")]
#[doc(alias = "-[GAITrackerImpl sendException:withNSError:]")]
// 0xeb8608 — -[GAITrackerImpl sendException:withNSError:]
// type: char __cdecl(GAITrackerImpl *self, SEL, char, id)
pub fn stub_0xeb8608() -> ! {
    todo!("0xeb8608 -[GAITrackerImpl sendException:withNSError:]")
}

#[doc(alias = "-[GAITrackerImpl trackTimingWithCategory:withValue:withName:withLabel:]")]
#[doc(alias = "-[GAITrackerImpl trackTimingWithCategory:withValue:withName:withLabel:]")]
// 0xeb86a4 — -[GAITrackerImpl trackTimingWithCategory:withValue:withName:withLabel:]
// type: char __cdecl(GAITrackerImpl *self, SEL, id, double, id, id)
pub fn stub_0xeb86a4() -> ! {
    todo!("0xeb86a4 -[GAITrackerImpl trackTimingWithCategory:withValue:withName:withLabel:]")
}

#[doc(alias = "-[GAITrackerImpl sendTimingWithCategory:withValue:withName:withLabel:]")]
#[doc(alias = "-[GAITrackerImpl sendTimingWithCategory:withValue:withName:withLabel:]")]
// 0xeb86d8 — -[GAITrackerImpl sendTimingWithCategory:withValue:withName:withLabel:]
// type: char __cdecl(GAITrackerImpl *self, SEL, id, double, id, id)
pub fn stub_0xeb86d8() -> ! {
    todo!("0xeb86d8 -[GAITrackerImpl sendTimingWithCategory:withValue:withName:withLabel:]")
}

#[doc(alias = "-[GAITrackerImpl trackSocial:withAction:withTarget:]")]
#[doc(alias = "-[GAITrackerImpl trackSocial:withAction:withTarget:]")]
// 0xeb8980 — -[GAITrackerImpl trackSocial:withAction:withTarget:]
// type: char __cdecl(GAITrackerImpl *self, SEL, id, id, id)
pub fn stub_0xeb8980() -> ! {
    todo!("0xeb8980 -[GAITrackerImpl trackSocial:withAction:withTarget:]")
}

#[doc(alias = "-[GAITrackerImpl sendSocial:withAction:withTarget:]")]
#[doc(alias = "-[GAITrackerImpl sendSocial:withAction:withTarget:]")]
// 0xeb89a4 — -[GAITrackerImpl sendSocial:withAction:withTarget:]
// type: char __cdecl(GAITrackerImpl *self, SEL, id, id, id)
pub fn stub_0xeb89a4() -> ! {
    todo!("0xeb89a4 -[GAITrackerImpl sendSocial:withAction:withTarget:]")
}

#[doc(alias = "-[GAITrackerImpl set:value:]")]
#[doc(alias = "-[GAITrackerImpl set:value:]")]
// 0xeb8d64 — -[GAITrackerImpl set:value:]
// type: char __cdecl(GAITrackerImpl *self, SEL, id, id)
pub fn stub_0xeb8d64() -> ! {
    todo!("0xeb8d64 -[GAITrackerImpl set:value:]")
}

#[doc(alias = "-[GAITrackerImpl get:]")]
#[doc(alias = "-[GAITrackerImpl get:]")]
// 0xeb8ebc — -[GAITrackerImpl get:]
// type: id __cdecl(GAITrackerImpl *self, SEL, id)
pub fn stub_0xeb8ebc() -> ! {
    todo!("0xeb8ebc -[GAITrackerImpl get:]")
}

#[doc(alias = "-[GAITrackerImpl send:params:]")]
#[doc(alias = "-[GAITrackerImpl send:params:]")]
// 0xeb9010 — -[GAITrackerImpl send:params:]
// type: char __cdecl(GAITrackerImpl *self, SEL, id, id)
pub fn stub_0xeb9010() -> ! {
    todo!("0xeb9010 -[GAITrackerImpl send:params:]")
}

#[doc(alias = "-[GAITrackerImpl setCustom:dimension:]")]
#[doc(alias = "-[GAITrackerImpl setCustom:dimension:]")]
// 0xeb9318 — -[GAITrackerImpl setCustom:dimension:]
// type: char __cdecl(GAITrackerImpl *self, SEL, int, id)
pub fn stub_0xeb9318() -> ! {
    todo!("0xeb9318 -[GAITrackerImpl setCustom:dimension:]")
}

#[doc(alias = "-[GAITrackerImpl setCustom:metric:]")]
#[doc(alias = "-[GAITrackerImpl setCustom:metric:]")]
// 0xeb95b0 — -[GAITrackerImpl setCustom:metric:]
// type: char __cdecl(GAITrackerImpl *self, SEL, int, id)
pub fn stub_0xeb95b0() -> ! {
    todo!("0xeb95b0 -[GAITrackerImpl setCustom:metric:]")
}

#[doc(alias = "-[GAITrackerImpl close]")]
#[doc(alias = "-[GAITrackerImpl close]")]
// 0xeb98ac — -[GAITrackerImpl close]
// type: void __cdecl(GAITrackerImpl *self, SEL)
pub fn stub_0xeb98ac() -> ! {
    todo!("0xeb98ac -[GAITrackerImpl close]")
}

#[doc(alias = "-[GAITrackerImpl appDidBecomeActive]")]
#[doc(alias = "-[GAITrackerImpl appDidBecomeActive]")]
// 0xeb9a60 — -[GAITrackerImpl appDidBecomeActive]
// type: void __cdecl(GAITrackerImpl *self, SEL)
pub fn stub_0xeb9a60() -> ! {
    todo!("0xeb9a60 -[GAITrackerImpl appDidBecomeActive]")
}

#[doc(alias = "-[GAITrackerImpl appWillResignActive]")]
#[doc(alias = "-[GAITrackerImpl appWillResignActive]")]
// 0xeb9b90 — -[GAITrackerImpl appWillResignActive]
// type: void __cdecl(GAITrackerImpl *self, SEL)
pub fn stub_0xeb9b90() -> ! {
    todo!("0xeb9b90 -[GAITrackerImpl appWillResignActive]")
}

#[doc(alias = "-[GAITrackerImpl dispatcher]")]
#[doc(alias = "-[GAITrackerImpl dispatcher]")]
// 0xeb9c68 — -[GAITrackerImpl dispatcher]
// type: GAIDispatcher *__cdecl(GAITrackerImpl *self, SEL)
pub fn stub_0xeb9c68() -> ! {
    todo!("0xeb9c68 -[GAITrackerImpl dispatcher]")
}

#[doc(alias = "-[GAITrackerImpl setDispatcher:]")]
#[doc(alias = "-[GAITrackerImpl setDispatcher:]")]
// 0xeb9c78 — -[GAITrackerImpl setDispatcher:]
// type: void __cdecl(GAITrackerImpl *self, SEL, id)
pub fn stub_0xeb9c78() -> ! {
    todo!("0xeb9c78 -[GAITrackerImpl setDispatcher:]")
}

#[doc(alias = "-[GAITrackerImpl trackerState]")]
#[doc(alias = "-[GAITrackerImpl trackerState]")]
// 0xeb9c9c — -[GAITrackerImpl trackerState]
// type: GAITrackerState *__cdecl(GAITrackerImpl *self, SEL)
pub fn stub_0xeb9c9c() -> ! {
    todo!("0xeb9c9c -[GAITrackerImpl trackerState]")
}

#[doc(alias = "-[GAITrackerImpl setTrackerState:]")]
#[doc(alias = "-[GAITrackerImpl setTrackerState:]")]
// 0xeb9cac — -[GAITrackerImpl setTrackerState:]
// type: void __cdecl(GAITrackerImpl *self, SEL, id)
pub fn stub_0xeb9cac() -> ! {
    todo!("0xeb9cac -[GAITrackerImpl setTrackerState:]")
}

#[doc(alias = "-[GAITrackerImpl adSdkPresent]")]
#[doc(alias = "-[GAITrackerImpl adSdkPresent]")]
// 0xeb9cd0 — -[GAITrackerImpl adSdkPresent]
// type: char __cdecl(GAITrackerImpl *self, SEL)
pub fn stub_0xeb9cd0() -> ! {
    todo!("0xeb9cd0 -[GAITrackerImpl adSdkPresent]")
}

#[doc(alias = "-[GAITrackerImpl setAdSdkPresent:]")]
#[doc(alias = "-[GAITrackerImpl setAdSdkPresent:]")]
// 0xeb9ce0 — -[GAITrackerImpl setAdSdkPresent:]
// type: void __cdecl(GAITrackerImpl *self, SEL, char)
pub fn stub_0xeb9ce0() -> ! {
    todo!("0xeb9ce0 -[GAITrackerImpl setAdSdkPresent:]")
}

#[doc(alias = "-[GAITrackerImpl trackCount]")]
#[doc(alias = "-[GAITrackerImpl trackCount]")]
// 0xeb9cf0 — -[GAITrackerImpl trackCount]
// type: unsigned __int64 __cdecl(GAITrackerImpl *self, SEL)
pub fn stub_0xeb9cf0() -> ! {
    todo!("0xeb9cf0 -[GAITrackerImpl trackCount]")
}

#[doc(alias = "-[GAITrackerImpl setTrackCount:]")]
#[doc(alias = "-[GAITrackerImpl setTrackCount:]")]
// 0xeb9d08 — -[GAITrackerImpl setTrackCount:]
// type: void __cdecl(GAITrackerImpl *self, SEL, unsigned __int64)
pub fn stub_0xeb9d08() -> ! {
    todo!("0xeb9d08 -[GAITrackerImpl setTrackCount:]")
}

#[doc(alias = "-[GAITrackerImpl hitCount]")]
#[doc(alias = "-[GAITrackerImpl hitCount]")]
// 0xeb9d1c — -[GAITrackerImpl hitCount]
// type: unsigned __int64 __cdecl(GAITrackerImpl *self, SEL)
pub fn stub_0xeb9d1c() -> ! {
    todo!("0xeb9d1c -[GAITrackerImpl hitCount]")
}

#[doc(alias = "-[GAITrackerImpl setHitCount:]")]
#[doc(alias = "-[GAITrackerImpl setHitCount:]")]
// 0xeb9d34 — -[GAITrackerImpl setHitCount:]
// type: void __cdecl(GAITrackerImpl *self, SEL, unsigned __int64)
pub fn stub_0xeb9d34() -> ! {
    todo!("0xeb9d34 -[GAITrackerImpl setHitCount:]")
}

#[doc(alias = "-[GAITrackerImpl lastActiveTime]")]
#[doc(alias = "-[GAITrackerImpl lastActiveTime]")]
// 0xeb9d48 — -[GAITrackerImpl lastActiveTime]
// type: double __cdecl(GAITrackerImpl *self, SEL)
pub fn stub_0xeb9d48() -> ! {
    todo!("0xeb9d48 -[GAITrackerImpl lastActiveTime]")
}

#[doc(alias = "-[GAITrackerImpl setLastActiveTime:]")]
#[doc(alias = "-[GAITrackerImpl setLastActiveTime:]")]
// 0xeb9d60 — -[GAITrackerImpl setLastActiveTime:]
// type: void __cdecl(GAITrackerImpl *self, SEL, double)
pub fn stub_0xeb9d60() -> ! {
    todo!("0xeb9d60 -[GAITrackerImpl setLastActiveTime:]")
}

#[doc(alias = "-[GAITrackerImpl trackingCredits]")]
#[doc(alias = "-[GAITrackerImpl trackingCredits]")]
// 0xeb9d74 — -[GAITrackerImpl trackingCredits]
// type: double __cdecl(GAITrackerImpl *self, SEL)
pub fn stub_0xeb9d74() -> ! {
    todo!("0xeb9d74 -[GAITrackerImpl trackingCredits]")
}

#[doc(alias = "-[GAITrackerImpl setTrackingCredits:]")]
#[doc(alias = "-[GAITrackerImpl setTrackingCredits:]")]
// 0xeb9d8c — -[GAITrackerImpl setTrackingCredits:]
// type: void __cdecl(GAITrackerImpl *self, SEL, double)
pub fn stub_0xeb9d8c() -> ! {
    todo!("0xeb9d8c -[GAITrackerImpl setTrackingCredits:]")
}

#[doc(alias = "-[GAITrackerImpl lastTrackTime]")]
#[doc(alias = "-[GAITrackerImpl lastTrackTime]")]
// 0xeb9da0 — -[GAITrackerImpl lastTrackTime]
// type: double __cdecl(GAITrackerImpl *self, SEL)
pub fn stub_0xeb9da0() -> ! {
    todo!("0xeb9da0 -[GAITrackerImpl lastTrackTime]")
}

#[doc(alias = "-[GAITrackerImpl setLastTrackTime:]")]
#[doc(alias = "-[GAITrackerImpl setLastTrackTime:]")]
// 0xeb9db8 — -[GAITrackerImpl setLastTrackTime:]
// type: void __cdecl(GAITrackerImpl *self, SEL, double)
pub fn stub_0xeb9db8() -> ! {
    todo!("0xeb9db8 -[GAITrackerImpl setLastTrackTime:]")
}

#[doc(alias = "-[GAIDataStore init]")]
#[doc(alias = "-[GAIDataStore init]")]
// 0xeb9dcc — -[GAIDataStore init]
// type: GAIDataStore *__cdecl(GAIDataStore *self, SEL)
pub fn stub_0xeb9dcc() -> ! {
    todo!("0xeb9dcc -[GAIDataStore init]")
}

#[doc(alias = "-[GAIDataStore dealloc]")]
#[doc(alias = "-[GAIDataStore dealloc]")]
// 0xeb9e28 — -[GAIDataStore dealloc]
// type: void __cdecl(GAIDataStore *self, SEL)
pub fn stub_0xeb9e28() -> ! {
    todo!("0xeb9e28 -[GAIDataStore dealloc]")
}

#[doc(alias = "-[GAIDataStore openPersistentStoreWithError:]")]
#[doc(alias = "-[GAIDataStore openPersistentStoreWithError:]")]
// 0xeb9edc — -[GAIDataStore openPersistentStoreWithError:]
// type: char __cdecl(GAIDataStore *self, SEL, id *)
pub fn stub_0xeb9edc() -> ! {
    todo!("0xeb9edc -[GAIDataStore openPersistentStoreWithError:]")
}

#[doc(alias = "-[GAIDataStore initWithPath:withError:]")]
#[doc(alias = "-[GAIDataStore initWithPath:withError:]")]
// 0xeba26c — -[GAIDataStore initWithPath:withError:]
// type: GAIDataStore *__cdecl(GAIDataStore *self, SEL, id, id *)
pub fn stub_0xeba26c() -> ! {
    todo!("0xeba26c -[GAIDataStore initWithPath:withError:]")
}

#[doc(alias = "-[GAIDataStore initWithError:]")]
#[doc(alias = "-[GAIDataStore initWithError:]")]
// 0xeba3e8 — -[GAIDataStore initWithError:]
// type: GAIDataStore *__cdecl(GAIDataStore *self, SEL, id *)
pub fn stub_0xeba3e8() -> ! {
    todo!("0xeba3e8 -[GAIDataStore initWithError:]")
}

#[doc(alias = "-[GAIDataStore contextForThread:]")]
#[doc(alias = "-[GAIDataStore contextForThread:]")]
// 0xeba544 — -[GAIDataStore contextForThread:]
// type: id __cdecl(GAIDataStore *self, SEL, id *)
pub fn stub_0xeba544() -> ! {
    todo!("0xeba544 -[GAIDataStore contextForThread:]")
}

#[doc(alias = "-[GAIDataStore entityCount:withError:]")]
#[doc(alias = "-[GAIDataStore entityCount:withError:]")]
// 0xeba720 — -[GAIDataStore entityCount:withError:]
// type: int __cdecl(GAIDataStore *self, SEL, id, id *)
pub fn stub_0xeba720() -> ! {
    todo!("0xeba720 -[GAIDataStore entityCount:withError:]")
}

#[doc(alias = "+[GAIDataStore dataStoreWithPath:withError:]")]
#[doc(alias = "+[GAIDataStore dataStoreWithPath:withError:]")]
// 0xeba9c4 — +[GAIDataStore dataStoreWithPath:withError:]
// type: id __cdecl(id, SEL, id, id *)
pub fn stub_0xeba9c4() -> ! {
    todo!("0xeba9c4 +[GAIDataStore dataStoreWithPath:withError:]")
}

#[doc(alias = "+[GAIDataStore inMemoryDataStoreWithError:]")]
#[doc(alias = "+[GAIDataStore inMemoryDataStoreWithError:]")]
// 0xebaa28 — +[GAIDataStore inMemoryDataStoreWithError:]
// type: id __cdecl(id, SEL, id *)
pub fn stub_0xebaa28() -> ! {
    todo!("0xebaa28 +[GAIDataStore inMemoryDataStoreWithError:]")
}

#[doc(alias = "-[GAIDataStore hitCount:]")]
#[doc(alias = "-[GAIDataStore hitCount:]")]
// 0xebaa88 — -[GAIDataStore hitCount:]
// type: int __cdecl(GAIDataStore *self, SEL, id *)
pub fn stub_0xebaa88() -> ! {
    todo!("0xebaa88 -[GAIDataStore hitCount:]")
}

#[doc(alias = "-[GAIDataStore fetchHitsWithLimit:withError:]")]
#[doc(alias = "-[GAIDataStore fetchHitsWithLimit:withError:]")]
// 0xebaab0 — -[GAIDataStore fetchHitsWithLimit:withError:]
// type: id __cdecl(GAIDataStore *self, SEL, unsigned int, id *)
pub fn stub_0xebaab0() -> ! {
    todo!("0xebaab0 -[GAIDataStore fetchHitsWithLimit:withError:]")
}

#[doc(alias = "-[GAIDataStore addHit:]")]
#[doc(alias = "-[GAIDataStore addHit:]")]
// 0xebadbc — -[GAIDataStore addHit:]
// type: id __cdecl(GAIDataStore *self, SEL, id *)
pub fn stub_0xebadbc() -> ! {
    todo!("0xebadbc -[GAIDataStore addHit:]")
}

#[doc(alias = "-[GAIDataStore propertyCount:]")]
#[doc(alias = "-[GAIDataStore propertyCount:]")]
// 0xebafdc — -[GAIDataStore propertyCount:]
// type: int __cdecl(GAIDataStore *self, SEL, id *)
pub fn stub_0xebafdc() -> ! {
    todo!("0xebafdc -[GAIDataStore propertyCount:]")
}

#[doc(alias = "-[GAIDataStore propertyForName:createIfNeeded:withError:]")]
#[doc(alias = "-[GAIDataStore propertyForName:createIfNeeded:withError:]")]
// 0xebb004 — -[GAIDataStore propertyForName:createIfNeeded:withError:]
// type: id __cdecl(GAIDataStore *self, SEL, id, char, id *)
pub fn stub_0xebb004() -> ! {
    todo!("0xebb004 -[GAIDataStore propertyForName:createIfNeeded:withError:]")
}

#[doc(alias = "-[GAIDataStore fetchProperties:]")]
#[doc(alias = "-[GAIDataStore fetchProperties:]")]
// 0xebb52c — -[GAIDataStore fetchProperties:]
// type: id __cdecl(GAIDataStore *self, SEL, id *)
pub fn stub_0xebb52c() -> ! {
    todo!("0xebb52c -[GAIDataStore fetchProperties:]")
}

#[doc(alias = "-[GAIDataStore deleteObject:]")]
#[doc(alias = "-[GAIDataStore deleteObject:]")]
// 0xebb7bc — -[GAIDataStore deleteObject:]
// type: void __cdecl(GAIDataStore *self, SEL, id)
pub fn stub_0xebb7bc() -> ! {
    todo!("0xebb7bc -[GAIDataStore deleteObject:]")
}

#[doc(alias = "-[GAIDataStore deleteObjects:]")]
#[doc(alias = "-[GAIDataStore deleteObjects:]")]
// 0xebb8f8 — -[GAIDataStore deleteObjects:]
// type: void __cdecl(GAIDataStore *self, SEL, id)
pub fn stub_0xebb8f8() -> ! {
    todo!("0xebb8f8 -[GAIDataStore deleteObjects:]")
}

#[doc(alias = "-[GAIDataStore deleteAll:withError:]")]
#[doc(alias = "-[GAIDataStore deleteAll:withError:]")]
// 0xebbb14 — -[GAIDataStore deleteAll:withError:]
// type: char __cdecl(GAIDataStore *self, SEL, id, id *)
pub fn stub_0xebbb14() -> ! {
    todo!("0xebbb14 -[GAIDataStore deleteAll:withError:]")
}

#[doc(alias = "-[GAIDataStore deleteAllHits:]")]
#[doc(alias = "-[GAIDataStore deleteAllHits:]")]
// 0xebbf68 — -[GAIDataStore deleteAllHits:]
// type: char __cdecl(GAIDataStore *self, SEL, id *)
pub fn stub_0xebbf68() -> ! {
    todo!("0xebbf68 -[GAIDataStore deleteAllHits:]")
}

#[doc(alias = "-[GAIDataStore deleteAllProperties:]")]
#[doc(alias = "-[GAIDataStore deleteAllProperties:]")]
// 0xebbf90 — -[GAIDataStore deleteAllProperties:]
// type: char __cdecl(GAIDataStore *self, SEL, id *)
pub fn stub_0xebbf90() -> ! {
    todo!("0xebbf90 -[GAIDataStore deleteAllProperties:]")
}

#[doc(alias = "-[GAIDataStore hasChanges]")]
#[doc(alias = "-[GAIDataStore hasChanges]")]
// 0xebbfb8 — -[GAIDataStore hasChanges]
// type: char __cdecl(GAIDataStore *self, SEL)
pub fn stub_0xebbfb8() -> ! {
    todo!("0xebbfb8 -[GAIDataStore hasChanges]")
}

#[doc(alias = "-[GAIDataStore save:]")]
#[doc(alias = "-[GAIDataStore save:]")]
// 0xebc0fc — -[GAIDataStore save:]
// type: char __cdecl(GAIDataStore *self, SEL, id *)
pub fn stub_0xebc0fc() -> ! {
    todo!("0xebc0fc -[GAIDataStore save:]")
}

#[doc(alias = "-[GAIDataStore path]")]
#[doc(alias = "-[GAIDataStore path]")]
// 0xebc2c8 — -[GAIDataStore path]
// type: NSString *__cdecl(GAIDataStore *self, SEL)
pub fn stub_0xebc2c8() -> ! {
    todo!("0xebc2c8 -[GAIDataStore path]")
}

#[doc(alias = "-[GAIDataStore setPath:]")]
#[doc(alias = "-[GAIDataStore setPath:]")]
// 0xebc2e0 — -[GAIDataStore setPath:]
// type: void __cdecl(GAIDataStore *self, SEL, id)
pub fn stub_0xebc2e0() -> ! {
    todo!("0xebc2e0 -[GAIDataStore setPath:]")
}

#[doc(alias = "-[GAIDataStore hitEntity]")]
#[doc(alias = "-[GAIDataStore hitEntity]")]
// 0xebc304 — -[GAIDataStore hitEntity]
// type: NSEntityDescription *__cdecl(GAIDataStore *self, SEL)
pub fn stub_0xebc304() -> ! {
    todo!("0xebc304 -[GAIDataStore hitEntity]")
}

#[doc(alias = "-[GAIDataStore setHitEntity:]")]
#[doc(alias = "-[GAIDataStore setHitEntity:]")]
// 0xebc314 — -[GAIDataStore setHitEntity:]
// type: void __cdecl(GAIDataStore *self, SEL, id)
pub fn stub_0xebc314() -> ! {
    todo!("0xebc314 -[GAIDataStore setHitEntity:]")
}

#[doc(alias = "-[GAIDataStore propertyEntity]")]
#[doc(alias = "-[GAIDataStore propertyEntity]")]
// 0xebc338 — -[GAIDataStore propertyEntity]
// type: NSEntityDescription *__cdecl(GAIDataStore *self, SEL)
pub fn stub_0xebc338() -> ! {
    todo!("0xebc338 -[GAIDataStore propertyEntity]")
}

#[doc(alias = "-[GAIDataStore setPropertyEntity:]")]
#[doc(alias = "-[GAIDataStore setPropertyEntity:]")]
// 0xebc348 — -[GAIDataStore setPropertyEntity:]
// type: void __cdecl(GAIDataStore *self, SEL, id)
pub fn stub_0xebc348() -> ! {
    todo!("0xebc348 -[GAIDataStore setPropertyEntity:]")
}

#[doc(alias = "-[GAIDataStore model]")]
#[doc(alias = "-[GAIDataStore model]")]
// 0xebc36c — -[GAIDataStore model]
// type: NSManagedObjectModel *__cdecl(GAIDataStore *self, SEL)
pub fn stub_0xebc36c() -> ! {
    todo!("0xebc36c -[GAIDataStore model]")
}

#[doc(alias = "-[GAIDataStore setModel:]")]
#[doc(alias = "-[GAIDataStore setModel:]")]
// 0xebc37c — -[GAIDataStore setModel:]
// type: void __cdecl(GAIDataStore *self, SEL, id)
pub fn stub_0xebc37c() -> ! {
    todo!("0xebc37c -[GAIDataStore setModel:]")
}

#[doc(alias = "-[GAIDataStore coordinator]")]
#[doc(alias = "-[GAIDataStore coordinator]")]
// 0xebc3a0 — -[GAIDataStore coordinator]
// type: NSPersistentStoreCoordinator *__cdecl(GAIDataStore *self, SEL)
pub fn stub_0xebc3a0() -> ! {
    todo!("0xebc3a0 -[GAIDataStore coordinator]")
}

#[doc(alias = "-[GAIDataStore setCoordinator:]")]
#[doc(alias = "-[GAIDataStore setCoordinator:]")]
// 0xebc3b0 — -[GAIDataStore setCoordinator:]
// type: void __cdecl(GAIDataStore *self, SEL, id)
pub fn stub_0xebc3b0() -> ! {
    todo!("0xebc3b0 -[GAIDataStore setCoordinator:]")
}

#[doc(alias = "-[GAIDataStore contexts]")]
#[doc(alias = "-[GAIDataStore contexts]")]
// 0xebc3d4 — -[GAIDataStore contexts]
// type: NSMutableDictionary *__cdecl(GAIDataStore *self, SEL)
pub fn stub_0xebc3d4() -> ! {
    todo!("0xebc3d4 -[GAIDataStore contexts]")
}

#[doc(alias = "-[GAIDataStore setContexts:]")]
#[doc(alias = "-[GAIDataStore setContexts:]")]
// 0xebc3e4 — -[GAIDataStore setContexts:]
// type: void __cdecl(GAIDataStore *self, SEL, id)
pub fn stub_0xebc3e4() -> ! {
    todo!("0xebc3e4 -[GAIDataStore setContexts:]")
}

#[doc(alias = "-[GAIHit parameters]")]
#[doc(alias = "-[GAIHit parameters]")]
// 0xebc56c — -[GAIHit parameters]
// type: NSDictionary *__cdecl(GAIHit *self, SEL)
pub fn stub_0xebc56c() -> ! {
    todo!("0xebc56c -[GAIHit parameters]")
}

#[doc(alias = "-[GAIHit setParameters:]")]
#[doc(alias = "-[GAIHit setParameters:]")]
// 0xebc6d0 — -[GAIHit setParameters:]
// type: void __cdecl(GAIHit *self, SEL, id)
pub fn stub_0xebc6d0() -> ! {
    todo!("0xebc6d0 -[GAIHit setParameters:]")
}

#[doc(alias = "-[GAIError init]")]
#[doc(alias = "-[GAIError init]")]
// 0xebc850 — -[GAIError init]
// type: GAIError *__cdecl(GAIError *self, SEL)
pub fn stub_0xebc850() -> ! {
    todo!("0xebc850 -[GAIError init]")
}

#[doc(alias = "+[GAIError errorWithCode:]")]
#[doc(alias = "+[GAIError errorWithCode:]")]
// 0xebc8ac — +[GAIError errorWithCode:]
// type: id __cdecl(id, SEL, int)
pub fn stub_0xebc8ac() -> ! {
    todo!("0xebc8ac +[GAIError errorWithCode:]")
}
