//! network generated_174 — RakNet + global gap filler (auto-generated, do not edit manually)
//! Filter: RakNet -> 944 funcs (strict), 0 remaining before batch (all covered); broad RakNet|Network|Replicat|Socket|HTTP|Upnp -> 6232 funcs, 49 remaining; batch EA-sorted asc next 150 global gaps not yet in network
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +150 stubs | range 0x62778..0x66c9c | existing 19619 -> 19769 total (filtered EA-sorted asc, rbx_core::SharedPtr not boost)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0x62778 — +[RobloxMemoryManager sharedInstance]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxMemoryManager sharedInstance]")]
pub fn stub_62778() -> ! {
    todo!("0x62778 +[RobloxMemoryManager sharedInstance]")
}

// 0x627d4 — ___37+[RobloxMemoryManager sharedInstance]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___37+[RobloxMemoryManager sharedInstance]_block_invoke")]
pub fn stub_627d4() -> ! {
    todo!("0x627d4 ___37+[RobloxMemoryManager sharedInstance]_block_invoke")
}

// 0x62808 — ___copy_helper_block__21
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__21")]
pub fn stub_62808() -> ! {
    todo!("0x62808 ___copy_helper_block__21")
}

// 0x62814 — ___destroy_helper_block__21
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__21")]
pub fn stub_62814() -> ! {
    todo!("0x62814 ___destroy_helper_block__21")
}

// 0x62820 — -[RobloxMemoryManager startMemoryBouncer]
// type: void __cdecl(RobloxMemoryManager *self, SEL)
#[doc(alias = "-[RobloxMemoryManager startMemoryBouncer]")]
pub fn stub_62820() -> ! {
    todo!("0x62820 -[RobloxMemoryManager startMemoryBouncer]")
}

// 0x62a40 — -[RobloxMemoryManager stopMemoryBouncer:]
// type: char __cdecl(RobloxMemoryManager *self, SEL, char)
#[doc(alias = "-[RobloxMemoryManager stopMemoryBouncer:]")]
pub fn stub_62a40() -> ! {
    todo!("0x62a40 -[RobloxMemoryManager stopMemoryBouncer:]")
}

// 0x62ac0 — -[RobloxMemoryManager balloonMemory]
// type: void __cdecl(RobloxMemoryManager *self, SEL)
#[doc(alias = "-[RobloxMemoryManager balloonMemory]")]
pub fn stub_62ac0() -> ! {
    todo!("0x62ac0 -[RobloxMemoryManager balloonMemory]")
}

// 0x62b64 — -[RobloxMemoryManager popBalloon]
// type: void __cdecl(RobloxMemoryManager *self, SEL)
#[doc(alias = "-[RobloxMemoryManager popBalloon]")]
pub fn stub_62b64() -> ! {
    todo!("0x62b64 -[RobloxMemoryManager popBalloon]")
}

// 0x62b88 — -[RobloxMemoryManager bounceFreeMemory:]
// type: void __cdecl(RobloxMemoryManager *self, SEL, id)
#[doc(alias = "-[RobloxMemoryManager bounceFreeMemory:]")]
pub fn stub_62b88() -> ! {
    todo!("0x62b88 -[RobloxMemoryManager bounceFreeMemory:]")
}

// 0x62be8 — -[RobloxMemoryManager startFreeMemoryChecker]
// type: void __cdecl(RobloxMemoryManager *self, SEL)
#[doc(alias = "-[RobloxMemoryManager startFreeMemoryChecker]")]
pub fn stub_62be8() -> ! {
    todo!("0x62be8 -[RobloxMemoryManager startFreeMemoryChecker]")
}

// 0x62d08 — -[RobloxMemoryManager stopFreeMemoryChecker]
// type: void __cdecl(RobloxMemoryManager *self, SEL)
#[doc(alias = "-[RobloxMemoryManager stopFreeMemoryChecker]")]
pub fn stub_62d08() -> ! {
    todo!("0x62d08 -[RobloxMemoryManager stopFreeMemoryChecker]")
}

// 0x62d48 — -[RobloxMemoryManager checkFreeMemory:]
// type: void __cdecl(RobloxMemoryManager *self, SEL, id)
#[doc(alias = "-[RobloxMemoryManager checkFreeMemory:]")]
pub fn stub_62d48() -> ! {
    todo!("0x62d48 -[RobloxMemoryManager checkFreeMemory:]")
}

// 0x62e5c — -[RobloxMemoryManager logMemUsage:]
// type: void __cdecl(RobloxMemoryManager *self, SEL, id)
#[doc(alias = "-[RobloxMemoryManager logMemUsage:]")]
pub fn stub_62e5c() -> ! {
    todo!("0x62e5c -[RobloxMemoryManager logMemUsage:]")
}

// 0x62f08 — __ZNSt6vectorIPvSaIS0_EED1Ev
// demangled: std::vector<void *,std::allocator<void *>>::~vector()
// type: void **__fastcall(void **)
#[doc(alias = "std::vector<void *,std::allocator<void *>>::~vector()")]
pub fn stub_62f08() -> ! {
    todo!("0x62f08 std::vector<void *,std::allocator<void *>>::~vector()")
}

// 0x62f1c — __ZNSt6vectorIPvSaIS0_EE9push_backERKS0_
// demangled: std::vector<void *,std::allocator<void *>>::push_back(void * const&)
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<void *,std::allocator<void *>>::push_back(void * const&)")]
pub fn stub_62f1c() -> ! {
    todo!("0x62f1c std::vector<void *,std::allocator<void *>>::push_back(void * const&)")
}

// 0x62f48 — __ZNSt6vectorIPvSaIS0_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS0_S2_EERKS0_
// demangled: std::vector<void *,std::allocator<void *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<void **,std::vector<void *,std::allocator<void *>>>,void * const&)
// type: char *__fastcall(int, char *__src, _DWORD *)
#[doc(alias = "std::vector<void *,std::allocator<void *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<void **,std::vector<void *,std::allocator<void *>>>,void * const&)")]
pub fn stub_62f48() -> ! {
    todo!("0x62f48 std::vector<void *,std::allocator<void *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<void **,std::vector<void ")
}

// 0x63028 — __ZNSt12_Vector_baseIPvSaIS0_EE11_M_allocateEm
// demangled: std::_Vector_base<void *,std::allocator<void *>>::_M_allocate(unsigned long)
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<void *,std::allocator<void *>>::_M_allocate(unsigned long)")]
pub fn stub_63028() -> ! {
    todo!("0x63028 std::_Vector_base<void *,std::allocator<void *>>::_M_allocate(unsigned long)")
}

// 0x63040 — __GLOBAL__I_a_32
// demangled: global constructor keyed to_a_32
#[doc(alias = "global constructor keyed to_a_32")]
pub fn stub_63040() -> ! {
    todo!("0x63040 global constructor keyed to_a_32")
}

// 0x631f8 — -[CameraMove init:]
// type: id __cdecl(CameraMove *self, SEL, CGRect)
#[doc(alias = "-[CameraMove init:]")]
pub fn stub_631f8() -> ! {
    todo!("0x631f8 -[CameraMove init:]")
}

// 0x63280 — -[CameraMove touchesBegan:withEvent:]
// type: void __cdecl(CameraMove *self, SEL, id, id)
#[doc(alias = "-[CameraMove touchesBegan:withEvent:]")]
pub fn stub_63280() -> ! {
    todo!("0x63280 -[CameraMove touchesBegan:withEvent:]")
}

// 0x632f0 — -[CameraMove touchesEnded:withEvent:]
// type: void __cdecl(CameraMove *self, SEL, id, id)
#[doc(alias = "-[CameraMove touchesEnded:withEvent:]")]
pub fn stub_632f0() -> ! {
    todo!("0x632f0 -[CameraMove touchesEnded:withEvent:]")
}

// 0x633c0 — -[CameraMove touchesCancelled:withEvent:]
// type: void __cdecl(CameraMove *self, SEL, id, id)
#[doc(alias = "-[CameraMove touchesCancelled:withEvent:]")]
pub fn stub_633c0() -> ! {
    todo!("0x633c0 -[CameraMove touchesCancelled:withEvent:]")
}

// 0x63490 — -[CameraMove cancelMovement]
// type: void __cdecl(CameraMove *self, SEL)
#[doc(alias = "-[CameraMove cancelMovement]")]
pub fn stub_63490() -> ! {
    todo!("0x63490 -[CameraMove cancelMovement]")
}

// 0x63528 — -[CameraMove touchesMoved:withEvent:]
// type: void __cdecl(CameraMove *self, SEL, id, id)
#[doc(alias = "-[CameraMove touchesMoved:withEvent:]")]
pub fn stub_63528() -> ! {
    todo!("0x63528 -[CameraMove touchesMoved:withEvent:]")
}

// 0x637a0 — __GLOBAL__I_a_33
// demangled: global constructor keyed to_a_33
#[doc(alias = "global constructor keyed to_a_33")]
pub fn stub_637a0() -> ! {
    todo!("0x637a0 global constructor keyed to_a_33")
}

// 0x639b8 — -[FunctionMarshaller marshallFunction]
// type: void __cdecl(FunctionMarshaller *self, SEL)
#[doc(alias = "-[FunctionMarshaller marshallFunction]")]
pub fn stub_639b8() -> ! {
    todo!("0x639b8 -[FunctionMarshaller marshallFunction]")
}

// 0x639e8 — -[FunctionMarshaller pClosure]
// type: void *__cdecl(FunctionMarshaller *self, SEL)
#[doc(alias = "-[FunctionMarshaller pClosure]")]
pub fn stub_639e8() -> ! {
    todo!("0x639e8 -[FunctionMarshaller pClosure]")
}

// 0x639f8 — -[FunctionMarshaller setPClosure:]
// type: void __cdecl(FunctionMarshaller *self, SEL, void *)
#[doc(alias = "-[FunctionMarshaller setPClosure:]")]
pub fn stub_639f8() -> ! {
    todo!("0x639f8 -[FunctionMarshaller setPClosure:]")
}

// 0x63a08 — __ZN6Roblox12sendAppEventEPv
// demangled: Roblox::sendAppEvent(void *)
// type: id __fastcall(Roblox *this, void *)
#[doc(alias = "Roblox::sendAppEvent(void *)")]
pub fn stub_63a08() -> ! {
    todo!("0x63a08 Roblox::sendAppEvent(void *)")
}

// 0x63aac — __ZN6Roblox12postAppEventEPv
// demangled: Roblox::postAppEvent(void *)
// type: id __fastcall(Roblox *this, void *)
#[doc(alias = "Roblox::postAppEvent(void *)")]
pub fn stub_63aac() -> ! {
    todo!("0x63aac Roblox::postAppEvent(void *)")
}

// 0x63b28 — __ZN6Roblox16processAppEventsEv
// demangled: Roblox::processAppEvents(void)
// type: CFRunLoopRunResult __fastcall(Roblox *this)
#[doc(alias = "Roblox::processAppEvents(void)")]
pub fn stub_63b28() -> ! {
    todo!("0x63b28 Roblox::processAppEvents(void)")
}

// 0x63b58 — __GLOBAL__I_a_34
// demangled: global constructor keyed to_a_34
#[doc(alias = "global constructor keyed to_a_34")]
pub fn stub_63b58() -> ! {
    todo!("0x63b58 global constructor keyed to_a_34")
}

// 0x63d30 — +[RobloxCachedFlags sharedInstance]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxCachedFlags sharedInstance]")]
pub fn stub_63d30() -> ! {
    todo!("0x63d30 +[RobloxCachedFlags sharedInstance]")
}

// 0x63d94 — ___35+[RobloxCachedFlags sharedInstance]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___35+[RobloxCachedFlags sharedInstance]_block_invoke")]
pub fn stub_63d94() -> ! {
    todo!("0x63d94 ___35+[RobloxCachedFlags sharedInstance]_block_invoke")
}

// 0x63dc8 — ___copy_helper_block__22
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__22")]
pub fn stub_63dc8() -> ! {
    todo!("0x63dc8 ___copy_helper_block__22")
}

// 0x63dd4 — ___destroy_helper_block__22
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__22")]
pub fn stub_63dd4() -> ! {
    todo!("0x63dd4 ___destroy_helper_block__22")
}

// 0x63ddc — -[RobloxCachedFlags init]
// type: RobloxCachedFlags *__cdecl(RobloxCachedFlags *self, SEL)
#[doc(alias = "-[RobloxCachedFlags init]")]
pub fn stub_63ddc() -> ! {
    todo!("0x63ddc -[RobloxCachedFlags init]")
}

// 0x63e20 — -[RobloxCachedFlags dealloc]
// type: void __cdecl(RobloxCachedFlags *self, SEL)
#[doc(alias = "-[RobloxCachedFlags dealloc]")]
pub fn stub_63e20() -> ! {
    todo!("0x63e20 -[RobloxCachedFlags dealloc]")
}

// 0x63e4c — -[RobloxCachedFlags sync]
// type: void __cdecl(RobloxCachedFlags *self, SEL)
#[doc(alias = "-[RobloxCachedFlags sync]")]
pub fn stub_63e4c() -> ! {
    todo!("0x63e4c -[RobloxCachedFlags sync]")
}

// 0x63e80 — -[RobloxCachedFlags getBool:withValue:]
// type: char __cdecl(RobloxCachedFlags *self, SEL, id, char *)
#[doc(alias = "-[RobloxCachedFlags getBool:withValue:]")]
pub fn stub_63e80() -> ! {
    todo!("0x63e80 -[RobloxCachedFlags getBool:withValue:]")
}

// 0x63ee8 — -[RobloxCachedFlags getInt:withValue:]
// type: char __cdecl(RobloxCachedFlags *self, SEL, id, int *)
#[doc(alias = "-[RobloxCachedFlags getInt:withValue:]")]
pub fn stub_63ee8() -> ! {
    todo!("0x63ee8 -[RobloxCachedFlags getInt:withValue:]")
}

// 0x63f50 — -[RobloxCachedFlags getString:withValue:]
// type: char __cdecl(RobloxCachedFlags *self, SEL, id, id)
#[doc(alias = "-[RobloxCachedFlags getString:withValue:]")]
pub fn stub_63f50() -> ! {
    todo!("0x63f50 -[RobloxCachedFlags getString:withValue:]")
}

// 0x63fb0 — -[RobloxCachedFlags setBool:withValue:]
// type: void __cdecl(RobloxCachedFlags *self, SEL, id, char)
#[doc(alias = "-[RobloxCachedFlags setBool:withValue:]")]
pub fn stub_63fb0() -> ! {
    todo!("0x63fb0 -[RobloxCachedFlags setBool:withValue:]")
}

// 0x64000 — -[RobloxCachedFlags setInt:withValue:]
// type: void __cdecl(RobloxCachedFlags *self, SEL, id, int)
#[doc(alias = "-[RobloxCachedFlags setInt:withValue:]")]
pub fn stub_64000() -> ! {
    todo!("0x64000 -[RobloxCachedFlags setInt:withValue:]")
}

// 0x64050 — -[RobloxCachedFlags setString:withValue:]
// type: void __cdecl(RobloxCachedFlags *self, SEL, id, id)
#[doc(alias = "-[RobloxCachedFlags setString:withValue:]")]
pub fn stub_64050() -> ! {
    todo!("0x64050 -[RobloxCachedFlags setString:withValue:]")
}

// 0x640a0 — -[CrashReporter activeCrashReporterString]
// type: id __cdecl(CrashReporter *self, SEL)
#[doc(alias = "-[CrashReporter activeCrashReporterString]")]
pub fn stub_640a0() -> ! {
    todo!("0x640a0 -[CrashReporter activeCrashReporterString]")
}

// 0x640e4 — +[CrashReporter sharedInstance]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[CrashReporter sharedInstance]")]
pub fn stub_640e4() -> ! {
    todo!("0x640e4 +[CrashReporter sharedInstance]")
}

// 0x64140 — ___31+[CrashReporter sharedInstance]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___31+[CrashReporter sharedInstance]_block_invoke")]
pub fn stub_64140() -> ! {
    todo!("0x64140 ___31+[CrashReporter sharedInstance]_block_invoke")
}

// 0x64174 — ___copy_helper_block__23
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__23")]
pub fn stub_64174() -> ! {
    todo!("0x64174 ___copy_helper_block__23")
}

// 0x64180 — ___destroy_helper_block__23
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__23")]
pub fn stub_64180() -> ! {
    todo!("0x64180 ___destroy_helper_block__23")
}

// 0x64188 — -[CrashReporter setupBugsense]
// type: void __cdecl(CrashReporter *self, SEL)
#[doc(alias = "-[CrashReporter setupBugsense]")]
pub fn stub_64188() -> ! {
    todo!("0x64188 -[CrashReporter setupBugsense]")
}

// 0x641cc — ___30-[CrashReporter setupBugsense]_block_invoke
// type: void __cdecl(id)
#[doc(alias = "___30-[CrashReporter setupBugsense]_block_invoke")]
pub fn stub_641cc() -> ! {
    todo!("0x641cc ___30-[CrashReporter setupBugsense]_block_invoke")
}

// 0x64308 — ___30-[CrashReporter setupBugsense]_block_invoke_2
// type: void __cdecl(id)
#[doc(alias = "___30-[CrashReporter setupBugsense]_block_invoke_2")]
pub fn stub_64308() -> ! {
    todo!("0x64308 ___30-[CrashReporter setupBugsense]_block_invoke_2")
}

// 0x6447c — -[CrashReporter setupTestFlight]
// type: void __cdecl(CrashReporter *self, SEL)
#[doc(alias = "-[CrashReporter setupTestFlight]")]
pub fn stub_6447c() -> ! {
    todo!("0x6447c -[CrashReporter setupTestFlight]")
}

// 0x644c0 — ___32-[CrashReporter setupTestFlight]_block_invoke
// type: void __cdecl(id)
#[doc(alias = "___32-[CrashReporter setupTestFlight]_block_invoke")]
pub fn stub_644c0() -> ! {
    todo!("0x644c0 ___32-[CrashReporter setupTestFlight]_block_invoke")
}

// 0x64558 — -[CrashReporter setupFastLogConnection]
// type: void __cdecl(CrashReporter *self, SEL)
#[doc(alias = "-[CrashReporter setupFastLogConnection]")]
pub fn stub_64558() -> ! {
    todo!("0x64558 -[CrashReporter setupFastLogConnection]")
}

// 0x646fc — __ZL13fastLogMesagehPKc
// demangled: fastLogMesage(unsigned char,char const*)
// type: void __fastcall(int, const char *)
#[doc(alias = "fastLogMesage(unsigned char,char const*)")]
pub fn stub_646fc() -> ! {
    todo!("0x646fc fastLogMesage(unsigned char,char const*)")
}

// 0x64764 — -[CrashReporter setup]
// type: void __cdecl(CrashReporter *self, SEL)
#[doc(alias = "-[CrashReporter setup]")]
pub fn stub_64764() -> ! {
    todo!("0x64764 -[CrashReporter setup]")
}

// 0x6496c — -[CrashReporter init]
// type: CrashReporter *__cdecl(CrashReporter *self, SEL)
#[doc(alias = "-[CrashReporter init]")]
pub fn stub_6496c() -> ! {
    todo!("0x6496c -[CrashReporter init]")
}

// 0x649b0 — -[CrashReporter dealloc]
// type: void __cdecl(CrashReporter *self, SEL)
#[doc(alias = "-[CrashReporter dealloc]")]
pub fn stub_649b0() -> ! {
    todo!("0x649b0 -[CrashReporter dealloc]")
}

// 0x649dc — -[CrashReporter tryLogMessage:]
// type: void __cdecl(CrashReporter *self, SEL, const StandardOutMessage *)
#[doc(alias = "-[CrashReporter tryLogMessage:]")]
pub fn stub_649dc() -> ! {
    todo!("0x649dc -[CrashReporter tryLogMessage:]")
}

// 0x64a74 — ___31-[CrashReporter tryLogMessage:]_block_invoke
// type: void __fastcall(int)
#[doc(alias = "___31-[CrashReporter tryLogMessage:]_block_invoke")]
pub fn stub_64a74() -> ! {
    todo!("0x64a74 ___31-[CrashReporter tryLogMessage:]_block_invoke")
}

// 0x64ad0 — ___copy_helper_block_118
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_118")]
pub fn stub_64ad0() -> ! {
    todo!("0x64ad0 ___copy_helper_block_118")
}

// 0x64adc — ___destroy_helper_block_119
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_119")]
pub fn stub_64adc() -> ! {
    todo!("0x64adc ___destroy_helper_block_119")
}

// 0x64ae4 — -[CrashReporter .cxx_destruct]
// type: void __cdecl(CrashReporter *self, SEL)
#[doc(alias = "-[CrashReporter .cxx_destruct]")]
pub fn stub_64ae4() -> ! {
    todo!("0x64ae4 -[CrashReporter .cxx_destruct]")
}

// 0x64bac — -[CrashReporter .cxx_construct]
// type: id __cdecl(CrashReporter *self, SEL)
#[doc(alias = "-[CrashReporter .cxx_construct]")]
pub fn stub_64bac() -> ! {
    todo!("0x64bac -[CrashReporter .cxx_construct]")
}

// 0x64bc0 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE7connectIN5boost8functionIS6_EEEENS0_10connectionERKT_
// demangled: rbx::signals::connection rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::connect<boost::function<void ()(RBX::StandardOutMessage const&)>>(boost::function<void ()(RBX::StandardOutMessage const&)> const&)
// type: void __fastcall(char, boost::mutex *, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::connect<boost::function<void ()(RBX::StandardOutMessage const&)>>(boost::function<void ()(RBX::StandardOutMessage const&)> const&)")]
pub fn stub_64bc0() -> ! {
    todo!("0x64bc0 rbx::signals::connection rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::connect<boost::function<")
}

// 0x64ca8 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE6insertEPNS7_4slotE
// demangled: rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::insert(rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot *)
// type: void __fastcall(int *, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::insert(rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot *)")]
pub fn stub_64ca8() -> ! {
    todo!("0x64ca8 rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::insert(rbx::signals::signal<void ()(RBX::Standard")
}

// 0x64eb8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEEaSEPSA_
// demangled: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot*)
// type: int *__fastcall(int *, int)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot*)")]
pub fn stub_64eb8() -> ! {
    todo!("0x64eb8 boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot>::operator=(rbx::signal")
}

// 0x64f5c — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEEaSERKSB_
// demangled: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot> const&)
// type: int *__fastcall(int *, int *)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot> const&)")]
pub fn stub_64f5c() -> ! {
    todo!("0x64f5c boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot>::operator=(boost::intr")
}

// 0x65000 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE22safe_static_init_mutexEv
// demangled: rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::safe_static_init_mutex(void)
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::safe_static_init_mutex(void)")]
pub fn stub_65000() -> ! {
    todo!("0x65000 rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::safe_static_init_mutex(void)")
}

// 0x65004 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE24safe_static_do_get_mutexEv
// demangled: rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::safe_static_do_get_mutex(void)
// type: int()
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::safe_static_do_get_mutex(void)")]
pub fn stub_65004() -> ! {
    todo!("0x65004 rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::safe_static_do_get_mutex(void)")
}

// 0x650fc — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEN5boost8functionIS7_EELi1ES7_EC2IPS8_EERKSC_T_
// demangled: rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>*>(boost::function<void ()(RBX::StandardOutMessage const&)> const&,rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>*)
// type: _DWORD *__fastcall(_DWORD *, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>*>(boost::function<void ()(RBX::StandardOutMessage const&)> const&,rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>*)")]
pub fn stub_650fc() -> ! {
    todo!("0x650fc rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::S")
}

// 0x651f8 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE13callable_slotIN5boost8functionIS6_EEED1Ev
// demangled: rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::callable_slot<boost::function<void ()(RBX::StandardOutMessage const&)>>::~callable_slot()
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::callable_slot<boost::function<void ()(RBX::StandardOutMessage const&)>>::~callable_slot()")]
pub fn stub_651f8() -> ! {
    todo!("0x651f8 rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::callable_slot<boost::function<void ()(RBX::Standa")
}

// 0x652cc — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE13callable_slotIN5boost8functionIS6_EEED0Ev
// demangled: rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::callable_slot<boost::function<void ()(RBX::StandardOutMessage const&)>>::~callable_slot()
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::callable_slot<boost::function<void ()(RBX::StandardOutMessage const&)>>::~callable_slot()")]
pub fn stub_652cc() -> ! {
    todo!("0x652cc rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::callable_slot<boost::function<void ()(RBX::Standa")
}

// 0x653a4 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slot10disconnectEv
// demangled: rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::disconnect(void)
// type: void __fastcall(int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::disconnect(void)")]
pub fn stub_653a4() -> ! {
    todo!("0x653a4 rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::disconnect(void)")
}

// 0x654b4 — __ZNK3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slot9connectedEv
// demangled: rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::connected(void)const
// type: bool __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::connected(void)const")]
pub fn stub_654b4() -> ! {
    todo!("0x654b4 rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::connected(void)const")
}

// 0x654c0 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEN5boost8functionIS7_EELi1ES7_E4callES6_
// demangled: rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::call(RBX::StandardOutMessage const&)
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::call(RBX::StandardOutMessage const&)")]
pub fn stub_654c0() -> ! {
    todo!("0x654c0 rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::S")
}

// 0x654c8 — __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEN5boost8functionIS7_EELi1ES7_E4callES6_
// demangled: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::call(RBX::StandardOutMessage const&)
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::call(RBX::StandardOutMessage const&)")]
pub fn stub_654c8() -> ! {
    todo!("0x654c8 non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::f")
}

// 0x654d0 — __ZNK5boost9function1IvRKN3RBX18StandardOutMessageEEclES4_
// demangled: boost::function1<void,RBX::StandardOutMessage const&>::operator()(RBX::StandardOutMessage const&)const
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "boost::function1<void,RBX::StandardOutMessage const&>::operator()(RBX::StandardOutMessage const&)const")]
pub fn stub_654d0() -> ! {
    todo!("0x654d0 boost::function1<void,RBX::StandardOutMessage const&>::operator()(RBX::StandardOutMessage const&)const")
}

// 0x65594 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE6removeEPNS7_4slotE
// demangled: rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::remove(rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot *)
// type: int __fastcall(char **, char *, int, const void *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::remove(rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot *)")]
pub fn stub_65594() -> ! {
    todo!("0x65594 rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::remove(rbx::signals::signal<void ()(RBX::Standard")
}

// 0x65684 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slot22safe_static_init_mutexEv
// demangled: rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::safe_static_init_mutex(void)
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::safe_static_init_mutex(void)")]
pub fn stub_65684() -> ! {
    todo!("0x65684 rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::safe_static_init_mutex(void)")
}

// 0x65688 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slot24safe_static_do_get_mutexEv
// demangled: rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::safe_static_do_get_mutex(void)
// type: void *()
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::safe_static_do_get_mutex(void)")]
pub fn stub_65688() -> ! {
    todo!("0x65688 rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::safe_static_do_get_mutex(void)")
}

// 0x65778 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEN5boost8functionIS7_EELi1ES7_ED1Ev
// demangled: rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::~callable()
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::~callable()")]
pub fn stub_65778() -> ! {
    todo!("0x65778 rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::S")
}

// 0x6584c — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEN5boost8functionIS7_EELi1ES7_ED0Ev
// demangled: rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::~callable()
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::~callable()")]
pub fn stub_6584c() -> ! {
    todo!("0x6584c rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::S")
}

// 0x65924 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotD1Ev
// demangled: rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::~slot()
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::~slot()")]
pub fn stub_65924() -> ! {
    todo!("0x65924 rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::~slot()")
}

// 0x659d0 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotD0Ev
// demangled: rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::~slot()
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::~slot()")]
pub fn stub_659d0() -> ! {
    todo!("0x659d0 rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::~slot()")
}

// 0x65a80 — __ZN5boost9function1IvRKN3RBX18StandardOutMessageEE13assign_to_ownERKS5_
// demangled: boost::function1<void,RBX::StandardOutMessage const&>::assign_to_own(boost::function1<void,RBX::StandardOutMessage const&> const&)
// type: int __fastcall(int result, int *)
#[doc(alias = "boost::function1<void,RBX::StandardOutMessage const&>::assign_to_own(boost::function1<void,RBX::StandardOutMessage const&> const&)")]
pub fn stub_65a80() -> ! {
    todo!("0x65a80 boost::function1<void,RBX::StandardOutMessage const&>::assign_to_own(boost::function1<void,RBX::StandardOutMessa")
}

// 0x65ab0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorRKN3RBX18StandardOutMessageEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::StandardOutMessage const&),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::StandardOutMessage const&),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::StandardOutMessage const&),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::StandardOutMessage const&),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_65ab0() -> ! {
    todo!("0x65ab0 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::Sta")
}

// 0x65b10 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorRKN3RBX18StandardOutMessageEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_
// demangled: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::StandardOutMessage const&),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::StandardOutMessage const>::invoke(boost::detail::function::function_buffer &,RBX::StandardOutMessage const)
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::StandardOutMessage const&),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::StandardOutMessage const>::invoke(boost::detail::function::function_buffer &,RBX::StandardOutMessage const)")]
pub fn stub_65b10() -> ! {
    todo!("0x65b10 boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector")
}

// 0x65b20 — __ZN5boost9function1IvRKN3RBX18StandardOutMessageEE5clearEv
// demangled: boost::function1<void,RBX::StandardOutMessage const&>::clear(void)
// type: int __fastcall(int *)
#[doc(alias = "boost::function1<void,RBX::StandardOutMessage const&>::clear(void)")]
pub fn stub_65b20() -> ! {
    todo!("0x65b20 boost::function1<void,RBX::StandardOutMessage const&>::clear(void)")
}

// 0x65b4c — __GLOBAL__I_a_35
// demangled: global constructor keyed to_a_35
#[doc(alias = "global constructor keyed to_a_35")]
pub fn stub_65b4c() -> ! {
    todo!("0x65b4c global constructor keyed to_a_35")
}

// 0x65ce4 — -[ExternalLoginViewController initWithCoder:]
// type: ExternalLoginViewController *__cdecl(ExternalLoginViewController *self, SEL, id)
#[doc(alias = "-[ExternalLoginViewController initWithCoder:]")]
pub fn stub_65ce4() -> ! {
    todo!("0x65ce4 -[ExternalLoginViewController initWithCoder:]")
}

// 0x65df4 — -[ExternalLoginViewController dealloc]
// type: void __cdecl(ExternalLoginViewController *self, SEL)
#[doc(alias = "-[ExternalLoginViewController dealloc]")]
pub fn stub_65df4() -> ! {
    todo!("0x65df4 -[ExternalLoginViewController dealloc]")
}

// 0x65ec0 — +[ExternalLoginViewController getLoginFinishedNotification]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[ExternalLoginViewController getLoginFinishedNotification]")]
pub fn stub_65ec0() -> ! {
    todo!("0x65ec0 +[ExternalLoginViewController getLoginFinishedNotification]")
}

// 0x65ecc — -[ExternalLoginViewController viewDidLoad]
// type: void __cdecl(ExternalLoginViewController *self, SEL)
#[doc(alias = "-[ExternalLoginViewController viewDidLoad]")]
pub fn stub_65ecc() -> ! {
    todo!("0x65ecc -[ExternalLoginViewController viewDidLoad]")
}

// 0x65f0c — -[ExternalLoginViewController viewWillAppear:]
// type: void __cdecl(ExternalLoginViewController *self, SEL, char)
#[doc(alias = "-[ExternalLoginViewController viewWillAppear:]")]
pub fn stub_65f0c() -> ! {
    todo!("0x65f0c -[ExternalLoginViewController viewWillAppear:]")
}

// 0x65f38 — -[ExternalLoginViewController didReceiveMemoryWarning]
// type: void __cdecl(ExternalLoginViewController *self, SEL)
#[doc(alias = "-[ExternalLoginViewController didReceiveMemoryWarning]")]
pub fn stub_65f38() -> ! {
    todo!("0x65f38 -[ExternalLoginViewController didReceiveMemoryWarning]")
}

// 0x65f64 — -[ExternalLoginViewController localizeStrings]
// type: void __cdecl(ExternalLoginViewController *self, SEL)
#[doc(alias = "-[ExternalLoginViewController localizeStrings]")]
pub fn stub_65f64() -> ! {
    todo!("0x65f64 -[ExternalLoginViewController localizeStrings]")
}

// 0x66078 — -[ExternalLoginViewController usernameDidEndOnExit:]
// type: void __cdecl(ExternalLoginViewController *self, SEL, id)
#[doc(alias = "-[ExternalLoginViewController usernameDidEndOnExit:]")]
pub fn stub_66078() -> ! {
    todo!("0x66078 -[ExternalLoginViewController usernameDidEndOnExit:]")
}

// 0x660a0 — -[ExternalLoginViewController passwordDidEndOnExit:]
// type: void __cdecl(ExternalLoginViewController *self, SEL, id)
#[doc(alias = "-[ExternalLoginViewController passwordDidEndOnExit:]")]
pub fn stub_660a0() -> ! {
    todo!("0x660a0 -[ExternalLoginViewController passwordDidEndOnExit:]")
}

// 0x660b4 — -[ExternalLoginViewController showLoggingIn]
// type: void __cdecl(ExternalLoginViewController *self, SEL)
#[doc(alias = "-[ExternalLoginViewController showLoggingIn]")]
pub fn stub_660b4() -> ! {
    todo!("0x660b4 -[ExternalLoginViewController showLoggingIn]")
}

// 0x66100 — -[ExternalLoginViewController showLoginFields]
// type: void __cdecl(ExternalLoginViewController *self, SEL)
#[doc(alias = "-[ExternalLoginViewController showLoginFields]")]
pub fn stub_66100() -> ! {
    todo!("0x66100 -[ExternalLoginViewController showLoginFields]")
}

// 0x6614c — -[ExternalLoginViewController gotLoginFailedNotification:]
// type: void __cdecl(ExternalLoginViewController *self, SEL, id)
#[doc(alias = "-[ExternalLoginViewController gotLoginFailedNotification:]")]
pub fn stub_6614c() -> ! {
    todo!("0x6614c -[ExternalLoginViewController gotLoginFailedNotification:]")
}

// 0x6619c — ___58-[ExternalLoginViewController gotLoginFailedNotification:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___58-[ExternalLoginViewController gotLoginFailedNotification:]_block_invoke")]
pub fn stub_6619c() -> ! {
    todo!("0x6619c ___58-[ExternalLoginViewController gotLoginFailedNotification:]_block_invoke")
}

// 0x661b0 — ___copy_helper_block__24
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__24")]
pub fn stub_661b0() -> ! {
    todo!("0x661b0 ___copy_helper_block__24")
}

// 0x661bc — ___destroy_helper_block__24
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__24")]
pub fn stub_661bc() -> ! {
    todo!("0x661bc ___destroy_helper_block__24")
}

// 0x661c4 — -[ExternalLoginViewController gotLoginSuccessfulNotification:]
// type: void __cdecl(ExternalLoginViewController *self, SEL, id)
#[doc(alias = "-[ExternalLoginViewController gotLoginSuccessfulNotification:]")]
pub fn stub_661c4() -> ! {
    todo!("0x661c4 -[ExternalLoginViewController gotLoginSuccessfulNotification:]")
}

// 0x66244 — ___62-[ExternalLoginViewController gotLoginSuccessfulNotification:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___62-[ExternalLoginViewController gotLoginSuccessfulNotification:]_block_invoke")]
pub fn stub_66244() -> ! {
    todo!("0x66244 ___62-[ExternalLoginViewController gotLoginSuccessfulNotification:]_block_invoke")
}

// 0x662a0 — ___62-[ExternalLoginViewController gotLoginSuccessfulNotification:]_block_invoke_2
// type: id __fastcall(int)
#[doc(alias = "___62-[ExternalLoginViewController gotLoginSuccessfulNotification:]_block_invoke_2")]
pub fn stub_662a0() -> ! {
    todo!("0x662a0 ___62-[ExternalLoginViewController gotLoginSuccessfulNotification:]_block_invoke_2")
}

// 0x66368 — ___copy_helper_block_88_0
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_88_0")]
pub fn stub_66368() -> ! {
    todo!("0x66368 ___copy_helper_block_88_0")
}

// 0x6638c — ___destroy_helper_block_89_0
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_89_0")]
pub fn stub_6638c() -> ! {
    todo!("0x6638c ___destroy_helper_block_89_0")
}

// 0x663a8 — ___copy_helper_block_93
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_93")]
pub fn stub_663a8() -> ! {
    todo!("0x663a8 ___copy_helper_block_93")
}

// 0x663cc — ___destroy_helper_block_94
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_94")]
pub fn stub_663cc() -> ! {
    todo!("0x663cc ___destroy_helper_block_94")
}

// 0x663e8 — -[ExternalLoginViewController doLogin:]
// type: void __cdecl(ExternalLoginViewController *self, SEL, id)
#[doc(alias = "-[ExternalLoginViewController doLogin:]")]
pub fn stub_663e8() -> ! {
    todo!("0x663e8 -[ExternalLoginViewController doLogin:]")
}

// 0x66480 — -[ExternalLoginViewController cancelTouched:]
// type: void __cdecl(ExternalLoginViewController *self, SEL, id)
#[doc(alias = "-[ExternalLoginViewController cancelTouched:]")]
pub fn stub_66480() -> ! {
    todo!("0x66480 -[ExternalLoginViewController cancelTouched:]")
}

// 0x664d0 — ___45-[ExternalLoginViewController cancelTouched:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___45-[ExternalLoginViewController cancelTouched:]_block_invoke")]
pub fn stub_664d0() -> ! {
    todo!("0x664d0 ___45-[ExternalLoginViewController cancelTouched:]_block_invoke")
}

// 0x66528 — ___45-[ExternalLoginViewController cancelTouched:]_block_invoke_2
// type: id __fastcall(int)
#[doc(alias = "___45-[ExternalLoginViewController cancelTouched:]_block_invoke_2")]
pub fn stub_66528() -> ! {
    todo!("0x66528 ___45-[ExternalLoginViewController cancelTouched:]_block_invoke_2")
}

// 0x665e0 — ___copy_helper_block_106
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_106")]
pub fn stub_665e0() -> ! {
    todo!("0x665e0 ___copy_helper_block_106")
}

// 0x665ec — ___destroy_helper_block_107
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_107")]
pub fn stub_665ec() -> ! {
    todo!("0x665ec ___destroy_helper_block_107")
}

// 0x665f4 — ___copy_helper_block_109
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_109")]
pub fn stub_665f4() -> ! {
    todo!("0x665f4 ___copy_helper_block_109")
}

// 0x66600 — ___destroy_helper_block_110
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_110")]
pub fn stub_66600() -> ! {
    todo!("0x66600 ___destroy_helper_block_110")
}

// 0x66608 — -[ExternalLoginViewController usernameTextField]
// type: UITextField *__cdecl(ExternalLoginViewController *self, SEL)
#[doc(alias = "-[ExternalLoginViewController usernameTextField]")]
pub fn stub_66608() -> ! {
    todo!("0x66608 -[ExternalLoginViewController usernameTextField]")
}

// 0x66618 — -[ExternalLoginViewController setUsernameTextField:]
// type: void __cdecl(ExternalLoginViewController *self, SEL, id)
#[doc(alias = "-[ExternalLoginViewController setUsernameTextField:]")]
pub fn stub_66618() -> ! {
    todo!("0x66618 -[ExternalLoginViewController setUsernameTextField:]")
}

// 0x6663c — -[ExternalLoginViewController passwordTextField]
// type: UITextField *__cdecl(ExternalLoginViewController *self, SEL)
#[doc(alias = "-[ExternalLoginViewController passwordTextField]")]
pub fn stub_6663c() -> ! {
    todo!("0x6663c -[ExternalLoginViewController passwordTextField]")
}

// 0x6664c — -[ExternalLoginViewController setPasswordTextField:]
// type: void __cdecl(ExternalLoginViewController *self, SEL, id)
#[doc(alias = "-[ExternalLoginViewController setPasswordTextField:]")]
pub fn stub_6664c() -> ! {
    todo!("0x6664c -[ExternalLoginViewController setPasswordTextField:]")
}

// 0x66670 — -[ExternalLoginViewController loginButton]
// type: UIButton *__cdecl(ExternalLoginViewController *self, SEL)
#[doc(alias = "-[ExternalLoginViewController loginButton]")]
pub fn stub_66670() -> ! {
    todo!("0x66670 -[ExternalLoginViewController loginButton]")
}

// 0x66680 — -[ExternalLoginViewController setLoginButton:]
// type: void __cdecl(ExternalLoginViewController *self, SEL, id)
#[doc(alias = "-[ExternalLoginViewController setLoginButton:]")]
pub fn stub_66680() -> ! {
    todo!("0x66680 -[ExternalLoginViewController setLoginButton:]")
}

// 0x666a4 — -[ExternalLoginViewController loginLabel]
// type: UILabel *__cdecl(ExternalLoginViewController *self, SEL)
#[doc(alias = "-[ExternalLoginViewController loginLabel]")]
pub fn stub_666a4() -> ! {
    todo!("0x666a4 -[ExternalLoginViewController loginLabel]")
}

// 0x666b4 — -[ExternalLoginViewController setLoginLabel:]
// type: void __cdecl(ExternalLoginViewController *self, SEL, id)
#[doc(alias = "-[ExternalLoginViewController setLoginLabel:]")]
pub fn stub_666b4() -> ! {
    todo!("0x666b4 -[ExternalLoginViewController setLoginLabel:]")
}

// 0x666d8 — -[ExternalLoginViewController loggingInIndicator]
// type: UIActivityIndicatorView *__cdecl(ExternalLoginViewController *self, SEL)
#[doc(alias = "-[ExternalLoginViewController loggingInIndicator]")]
pub fn stub_666d8() -> ! {
    todo!("0x666d8 -[ExternalLoginViewController loggingInIndicator]")
}

// 0x666e8 — -[ExternalLoginViewController setLoggingInIndicator:]
// type: void __cdecl(ExternalLoginViewController *self, SEL, id)
#[doc(alias = "-[ExternalLoginViewController setLoggingInIndicator:]")]
pub fn stub_666e8() -> ! {
    todo!("0x666e8 -[ExternalLoginViewController setLoggingInIndicator:]")
}

// 0x6670c — -[ExternalLoginViewController loginView]
// type: UIView *__cdecl(ExternalLoginViewController *self, SEL)
#[doc(alias = "-[ExternalLoginViewController loginView]")]
pub fn stub_6670c() -> ! {
    todo!("0x6670c -[ExternalLoginViewController loginView]")
}

// 0x6671c — -[ExternalLoginViewController setLoginView:]
// type: void __cdecl(ExternalLoginViewController *self, SEL, id)
#[doc(alias = "-[ExternalLoginViewController setLoginView:]")]
pub fn stub_6671c() -> ! {
    todo!("0x6671c -[ExternalLoginViewController setLoginView:]")
}

// 0x66740 — -[NSString(Escaping) stringWithPercentEscape]_1
// type: NSString *__cdecl(NSString *self, SEL)
#[doc(alias = "-[NSString(Escaping) stringWithPercentEscape]_1")]
pub fn stub_66740() -> ! {
    todo!("0x66740 -[NSString(Escaping) stringWithPercentEscape]_1")
}

// 0x66794 — +[AppController sharedInstance]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[AppController sharedInstance]")]
pub fn stub_66794() -> ! {
    todo!("0x66794 +[AppController sharedInstance]")
}

// 0x667f0 — ___31+[AppController sharedInstance]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___31+[AppController sharedInstance]_block_invoke")]
pub fn stub_667f0() -> ! {
    todo!("0x667f0 ___31+[AppController sharedInstance]_block_invoke")
}

// 0x66824 — ___copy_helper_block__25
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__25")]
pub fn stub_66824() -> ! {
    todo!("0x66824 ___copy_helper_block__25")
}

// 0x66830 — ___destroy_helper_block__25
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__25")]
pub fn stub_66830() -> ! {
    todo!("0x66830 ___destroy_helper_block__25")
}

// 0x66838 — -[AppController init]
// type: AppController *__cdecl(AppController *self, SEL)
#[doc(alias = "-[AppController init]")]
pub fn stub_66838() -> ! {
    todo!("0x66838 -[AppController init]")
}

// 0x66a68 — -[AppController dealloc]
// type: void __cdecl(AppController *self, SEL)
#[doc(alias = "-[AppController dealloc]")]
pub fn stub_66a68() -> ! {
    todo!("0x66a68 -[AppController dealloc]")
}

// 0x66acc — -[AppController checkForGameLaunch]
// type: void __cdecl(AppController *self, SEL)
#[doc(alias = "-[AppController checkForGameLaunch]")]
pub fn stub_66acc() -> ! {
    todo!("0x66acc -[AppController checkForGameLaunch]")
}

// 0x66af8 — -[AppController gotLoginFailedNotification:]
// type: void __cdecl(AppController *self, SEL, id)
#[doc(alias = "-[AppController gotLoginFailedNotification:]")]
pub fn stub_66af8() -> ! {
    todo!("0x66af8 -[AppController gotLoginFailedNotification:]")
}

// 0x66b08 — -[AppController gotLoginSuccessfulNotification:]
// type: void __cdecl(AppController *self, SEL, id)
#[doc(alias = "-[AppController gotLoginSuccessfulNotification:]")]
pub fn stub_66b08() -> ! {
    todo!("0x66b08 -[AppController gotLoginSuccessfulNotification:]")
}

// 0x66b18 — -[AppController authenticateUserWithUrl:ticket:]
// type: void __cdecl(AppController *self, SEL, id, id)
#[doc(alias = "-[AppController authenticateUserWithUrl:ticket:]")]
pub fn stub_66b18() -> ! {
    todo!("0x66b18 -[AppController authenticateUserWithUrl:ticket:]")
}

// 0x66b1c — -[AppController runJoinScriptWithUrl:]
// type: void __cdecl(AppController *self, SEL, id)
#[doc(alias = "-[AppController runJoinScriptWithUrl:]")]
pub fn stub_66b1c() -> ! {
    todo!("0x66b1c -[AppController runJoinScriptWithUrl:]")
}

// 0x66bd0 — -[AppController performPollingOnMainThreadWithObject:]
// type: void __cdecl(AppController *self, SEL, id)
#[doc(alias = "-[AppController performPollingOnMainThreadWithObject:]")]
pub fn stub_66bd0() -> ! {
    todo!("0x66bd0 -[AppController performPollingOnMainThreadWithObject:]")
}

// 0x66c44 — ___54-[AppController performPollingOnMainThreadWithObject:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___54-[AppController performPollingOnMainThreadWithObject:]_block_invoke")]
pub fn stub_66c44() -> ! {
    todo!("0x66c44 ___54-[AppController performPollingOnMainThreadWithObject:]_block_invoke")
}

// 0x66c78 — ___copy_helper_block_76_0
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_76_0")]
pub fn stub_66c78() -> ! {
    todo!("0x66c78 ___copy_helper_block_76_0")
}

// 0x66c9c — ___destroy_helper_block_77_0
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_77_0")]
pub fn stub_66c9c() -> ! {
    todo!("0x66c9c ___destroy_helper_block_77_0")
}
