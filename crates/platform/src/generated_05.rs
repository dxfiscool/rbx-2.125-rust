//! platform generated_05 — next 60 stubs EA-sorted, from ida/export.json
//! Filter: ObjC|iOS|UIKit|Roblox|GVC (3839 total; 6 strict remaining incl 0x63a08..0x295628, 54 high-EA filler) | EA-sorted asc, skip existing, rbx_core::SharedPtr not boost
//! Batch: 60 stubs | skeleton batch | range 0x63a08..0xf6fb4c

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// Ensure SharedPtr is seen as used — mirrors boost::shared_ptr<T> -> rbx_core::SharedPtr<T>
const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

/// Host model of the `Roblox *` object behind `sendAppEvent`/`postAppEvent`
/// (IDA 0x63a08/0x63aac): the opaque identity of `this` plus the `RBX::CEvent *`
/// cell at `this + 8` (`*((_DWORD *)this + 2)`, 0x63a28). `None` is `nil`.
/// There is no ObjC runtime on the host, so the `FunctionMarshaller`
/// alloc/init + `performSelectorOnMainThread:...` hop collapses into the
/// `marshals` counter; the observable branch structure is 1:1.
#[derive(Debug, Default)]
pub struct RobloxAppHost {
    pub this_id: usize,
    pub app_event: Option<ManualAppEvent>,
    pub marshals: u32,
    pub waits: u32,
    pub last_wait_until_done: Option<bool>,
}

/// Host model of `RBX::CEvent` behind the wait in `sendAppEvent`
/// (IDA 0x63a9e `RBX::CEvent::Wait`).
#[derive(Debug, Default)]
pub struct ManualAppEvent {
    pub signaled: bool,
    pub wait_calls: u32,
}

impl ManualAppEvent {
    pub fn wait(&mut self) {
        self.wait_calls += 1;
        self.signaled = true;
    }
}

/// `Roblox::sendAppEvent` (IDA 0x63a08): marshal `marshallFunction` to the
/// main thread, waiting until done only when there is no event (`v3 == 0`
/// feeds `waitUntilDone`, 0x63a7c..0x63a8a); when the event is present,
/// block in `RBX::CEvent::Wait` instead (0x63a90..0x63a9e).
pub fn send_app_event(host: &mut RobloxAppHost) -> usize {
    host.marshals += 1;
    match host.app_event.as_mut() {
        None => {
            host.last_wait_until_done = Some(true);
            0
        }
        Some(event) => {
            host.last_wait_until_done = Some(false);
            host.waits += 1;
            event.wait();
            0
        }
    }
}

/// `Roblox::postAppEvent` (IDA 0x63aac): same marshaller hop with
/// `waitUntilDone:0` always (0x63b16 `MOVS R4,#0`); no event is loaded and
/// no `CEvent::Wait` follows.
pub fn post_app_event(host: &mut RobloxAppHost) -> usize {
    host.marshals += 1;
    host.last_wait_until_done = Some(false);
    0
}

/// Run-loop drain mode pumped by `processAppEvents` (IDA 0x63b28
/// `CFSTR("RobloxAppEvent")`).
pub const ROBLOX_APP_EVENT_MODE: &str = "RobloxAppEvent";

/// `kCFRunLoopRunHandledSource` compared at IDA 0x63b4c (`CMP R0,#4`).
pub const RUN_LOOP_HANDLED_SOURCE: u32 = 4;

/// `Roblox::processAppEvents` (IDA 0x63b28): run the `RobloxAppEvent` mode
/// with zero timeout until the result is anything but
/// `kCFRunLoopRunHandledSource`. `pump` stands in for
/// `CFRunLoopRunInMode` (out of slice); the loop shape is 1:1.
pub fn process_app_events(pump: &dyn Fn() -> u32) -> u32 {
    loop {
        let result = pump();
        if result != RUN_LOOP_HANDLED_SOURCE {
            return result;
        }
    }
}

/// `RBX::BaseScript::isRobloxScript` (IDA 0x28d544): `MOVS R0,#1; BX LR` —
/// unconditionally true.
pub fn base_script_is_roblox_script() -> bool {
    true
}

/// Class-flag word behind `RBX::Script::isRobloxScript` (IDA 0x28d548):
/// `type_tag` is `*(*(this + 0x5C) - 12)` (0x28d54c), `flag_84` is byte
/// `this + 132`, `byte_87` is byte `this + 135`.
#[derive(Debug, Clone, Copy, Default)]
pub struct ScriptRobloxFlags {
    pub type_tag: u32,
    pub flag_84: bool,
    pub byte_87: u8,
}

/// `RBX::Script::isRobloxScript` (IDA 0x28d548): a set class word means
/// Roblox script (`result = 1`, 0x28d550..0x28d552); otherwise the flag at
/// +132 gates the byte at +135, which doubles as the result when set.
pub fn script_is_roblox_script(flags: &ScriptRobloxFlags) -> bool {
    if flags.type_tag != 0 {
        return true;
    }
    if !flags.flag_84 {
        return false;
    }
    if flags.byte_87 != 0 {
        return true;
    }
    false
}

/// Library-service cell behind `registerRobloxLibrary` (IDA 0x295628):
/// `*(a1 + 768)` (`LDR R0,[R0,#0x300]`), the `this` forwarded to
/// `RBX::LibraryService::registerLibrary`.
#[derive(Debug, Clone, Copy, Default)]
pub struct ScriptContextLibs {
    pub library_service: usize,
}

/// `RBX::ScriptContext::registerRobloxLibrary` (IDA 0x295628): forwards both
/// strings plus `is_roblox = 1` (`MOVS R3,#1`, 0x29562c) to
/// `LibraryService::registerLibrary`, which lives out of slice so it arrives
/// as a closure (bind/function become closures).
pub fn register_roblox_library(
    ctx: &ScriptContextLibs,
    name: &str,
    library: &str,
    register: &dyn Fn(usize, &str, &str, bool) -> i32,
) -> i32 {
    register(ctx.library_service, name, library, true)
}

/// Shared dyld `_stub_helpers` carrier for the lazy-bind trampolines below
/// (IDA 0xf6f8d0..0xf6f9a8: `LDR R12, =slot; B _stub_helpers`). The slot
/// ordinal loaded into `R12` is the only per-trampoline payload, so it is
/// the argument; the branch itself is a no-op here with no host dyld.
pub fn stub_helpers(slot: u32) {
    let _ = slot;
}

// 0x63a08 — __ZN6Roblox12sendAppEventEPv
// type: id __fastcall(Roblox *this, void *)
#[doc(alias = "Roblox::sendAppEvent(void *)")]
pub fn stub_63a08(host: &mut RobloxAppHost) -> usize {
    // IDA 0x63a08
    send_app_event(host)
}

// 0x63aac — __ZN6Roblox12postAppEventEPv
// type: id __fastcall(Roblox *this, void *)
#[doc(alias = "Roblox::postAppEvent(void *)")]
pub fn stub_63aac(host: &mut RobloxAppHost) -> usize {
    // IDA 0x63aac
    post_app_event(host)
}

// 0x63b28 — __ZN6Roblox16processAppEventsEv
// type: CFRunLoopRunResult __fastcall(Roblox *this)
#[doc(alias = "Roblox::processAppEvents(void)")]
pub fn stub_63b28(pump: &dyn Fn() -> u32) -> u32 {
    // IDA 0x63b28
    process_app_events(pump)
}

// 0x28d544 — __ZNK3RBX10BaseScript14isRobloxScriptEv
// type: _DWORD __fastcall(RBX::BaseScript *__hidden this)
#[doc(alias = "RBX::BaseScript::isRobloxScript(void)const")]
pub fn stub_28d544() -> bool {
    // IDA 0x28d544
    base_script_is_roblox_script()
}

// 0x28d548 — __ZNK3RBX6Script14isRobloxScriptEv
// type: _DWORD __fastcall(RBX::Script *__hidden this)
#[doc(alias = "RBX::Script::isRobloxScript(void)const")]
pub fn stub_28d548(flags: &ScriptRobloxFlags) -> bool {
    // IDA 0x28d548
    script_is_roblox_script(flags)
}

// 0x295628 — __ZN3RBX13ScriptContext21registerRobloxLibraryESsSs
#[doc(alias = "RBX::ScriptContext::registerRobloxLibrary(std::string,std::string)")]
pub fn stub_295628(
    ctx: &ScriptContextLibs,
    name: &str,
    library: &str,
    register: &dyn Fn(usize, &str, &str, bool) -> i32,
) -> i32 {
    // IDA 0x295628
    register_roblox_library(ctx, name, library, register)
}

// 0xf6f8d0 — sub_F6F8D0
#[doc(alias = "sub_F6F8D0")]
pub fn stub_f6f8d0() {
    // IDA 0xf6f8d0: `LDR R12, =0x545D; B _stub_helpers` — lazy-bind
    // trampoline; faithful thin wrapper over the shared carrier.
    stub_helpers(0x545D)
}

// 0xf6f8dc — sub_F6F8DC
#[doc(alias = "sub_F6F8DC")]
pub fn stub_f6f8dc() {
    // IDA 0xf6f8dc: `LDR R12, =0x5477; B _stub_helpers` — lazy-bind
    // trampoline; faithful thin wrapper over the shared carrier.
    stub_helpers(0x5477)
}

// 0xf6f8e8 — sub_F6F8E8
#[doc(alias = "sub_F6F8E8")]
pub fn stub_f6f8e8() {
    // IDA 0xf6f8e8: `LDR R12, =0x5492; B _stub_helpers` — lazy-bind
    // trampoline; faithful thin wrapper over the shared carrier.
    stub_helpers(0x5492)
}

// 0xf6f8f4 — sub_F6F8F4
#[doc(alias = "sub_F6F8F4")]
pub fn stub_f6f8f4() {
    // IDA 0xf6f8f4: `LDR R12, =0x54AE; B _stub_helpers` — lazy-bind
    // trampoline; faithful thin wrapper over the shared carrier.
    stub_helpers(0x54AE)
}

// 0xf6f900 — sub_F6F900
#[doc(alias = "sub_F6F900")]
pub fn stub_f6f900() {
    // IDA 0xf6f900: `LDR R12, =0x54D5; B _stub_helpers` — lazy-bind
    // trampoline; faithful thin wrapper over the shared carrier.
    stub_helpers(0x54D5)
}

// 0xf6f90c — sub_F6F90C
#[doc(alias = "sub_F6F90C")]
pub fn stub_f6f90c() {
    // IDA 0xf6f90c: `LDR R12, =0x54F8; B _stub_helpers` — lazy-bind
    // trampoline; faithful thin wrapper over the shared carrier.
    stub_helpers(0x54F8)
}

// 0xf6f918 — sub_F6F918
#[doc(alias = "sub_F6F918")]
pub fn stub_f6f918() {
    // IDA 0xf6f918: `LDR R12, =0x551B; B _stub_helpers` — lazy-bind
    // trampoline; faithful thin wrapper over the shared carrier.
    stub_helpers(0x551B)
}

// 0xf6f924 — sub_F6F924
#[doc(alias = "sub_F6F924")]
pub fn stub_f6f924() {
    // IDA 0xf6f924: `LDR R12, =0x5538; B _stub_helpers` — lazy-bind
    // trampoline; faithful thin wrapper over the shared carrier.
    stub_helpers(0x5538)
}

// 0xf6f930 — sub_F6F930
#[doc(alias = "sub_F6F930")]
pub fn stub_f6f930() {
    // IDA 0xf6f930: `LDR R12, =0x5558; B _stub_helpers` — lazy-bind
    // trampoline; faithful thin wrapper over the shared carrier.
    stub_helpers(0x5558)
}

// 0xf6f93c — sub_F6F93C
#[doc(alias = "sub_F6F93C")]
pub fn stub_f6f93c() {
    // IDA 0xf6f93c: `LDR R12, =0x5582; B _stub_helpers` — lazy-bind
    // trampoline; faithful thin wrapper over the shared carrier.
    stub_helpers(0x5582)
}

// 0xf6f948 — sub_F6F948
#[doc(alias = "sub_F6F948")]
pub fn stub_f6f948() {
    // IDA 0xf6f948: `LDR R12, =0x55A6; B _stub_helpers` — lazy-bind
    // trampoline; faithful thin wrapper over the shared carrier.
    stub_helpers(0x55A6)
}

// 0xf6f954 — sub_F6F954
#[doc(alias = "sub_F6F954")]
pub fn stub_f6f954() {
    // IDA 0xf6f954: `LDR R12, =0x55C6; B _stub_helpers` — lazy-bind
    // trampoline; faithful thin wrapper over the shared carrier.
    stub_helpers(0x55C6)
}

// 0xf6f960 — sub_F6F960
#[doc(alias = "sub_F6F960")]
pub fn stub_f6f960() {
    // IDA 0xf6f960: `LDR R12, =0x55DD; B _stub_helpers` — lazy-bind
    // trampoline; faithful thin wrapper over the shared carrier.
    stub_helpers(0x55DD)
}

// 0xf6f96c — sub_F6F96C
#[doc(alias = "sub_F6F96C")]
pub fn stub_f6f96c() {
    // IDA 0xf6f96c: `LDR R12, =0x55FB; B _stub_helpers` — lazy-bind
    // trampoline; faithful thin wrapper over the shared carrier.
    stub_helpers(0x55FB)
}

// 0xf6f978 — sub_F6F978
#[doc(alias = "sub_F6F978")]
pub fn stub_f6f978() {
    // IDA 0xf6f978: `LDR R12, =0x5621; B _stub_helpers` — lazy-bind
    // trampoline; faithful thin wrapper over the shared carrier.
    stub_helpers(0x5621)
}

// 0xf6f984 — sub_F6F984
#[doc(alias = "sub_F6F984")]
pub fn stub_f6f984() {
    // IDA 0xf6f984: `LDR R12, =0x5643; B _stub_helpers` — lazy-bind
    // trampoline; faithful thin wrapper over the shared carrier.
    stub_helpers(0x5643)
}

// 0xf6f990 — sub_F6F990
#[doc(alias = "sub_F6F990")]
pub fn stub_f6f990() {
    // IDA 0xf6f990: `LDR R12, =0x565D; B _stub_helpers` — lazy-bind
    // trampoline; faithful thin wrapper over the shared carrier.
    stub_helpers(0x565D)
}

// 0xf6f99c — sub_F6F99C
#[doc(alias = "sub_F6F99C")]
pub fn stub_f6f99c() {
    // IDA 0xf6f99c: `LDR R12, =0x567C; B _stub_helpers` — lazy-bind
    // trampoline; faithful thin wrapper over the shared carrier.
    stub_helpers(0x567C)
}

// 0xf6f9a8 — sub_F6F9A8
#[doc(alias = "sub_F6F9A8")]
pub fn stub_f6f9a8() {
    // IDA 0xf6f9a8: `LDR R12, =0x56A3; B _stub_helpers` — lazy-bind
    // trampoline; faithful thin wrapper over the shared carrier.
    stub_helpers(0x56A3)
}

// 0xf6f9b4 — sub_F6F9B4
#[doc(alias = "sub_F6F9B4")]
pub fn stub_f6f9b4() -> ! {
    todo!("0xf6f9b4 sub_F6F9B4")
}

// 0xf6f9c0 — sub_F6F9C0
#[doc(alias = "sub_F6F9C0")]
pub fn stub_f6f9c0() -> ! {
    todo!("0xf6f9c0 sub_F6F9C0")
}

// 0xf6f9cc — sub_F6F9CC
#[doc(alias = "sub_F6F9CC")]
pub fn stub_f6f9cc() -> ! {
    todo!("0xf6f9cc sub_F6F9CC")
}

// 0xf6f9d8 — sub_F6F9D8
#[doc(alias = "sub_F6F9D8")]
pub fn stub_f6f9d8() -> ! {
    todo!("0xf6f9d8 sub_F6F9D8")
}

// 0xf6f9e4 — sub_F6F9E4
#[doc(alias = "sub_F6F9E4")]
pub fn stub_f6f9e4() -> ! {
    todo!("0xf6f9e4 sub_F6F9E4")
}

// 0xf6f9f0 — sub_F6F9F0
#[doc(alias = "sub_F6F9F0")]
pub fn stub_f6f9f0() -> ! {
    todo!("0xf6f9f0 sub_F6F9F0")
}

// 0xf6f9fc — sub_F6F9FC
#[doc(alias = "sub_F6F9FC")]
pub fn stub_f6f9fc() -> ! {
    todo!("0xf6f9fc sub_F6F9FC")
}

// 0xf6fa08 — sub_F6FA08
#[doc(alias = "sub_F6FA08")]
pub fn stub_f6fa08() -> ! {
    todo!("0xf6fa08 sub_F6FA08")
}

// 0xf6fa14 — sub_F6FA14
#[doc(alias = "sub_F6FA14")]
pub fn stub_f6fa14() -> ! {
    todo!("0xf6fa14 sub_F6FA14")
}

// 0xf6fa20 — sub_F6FA20
#[doc(alias = "sub_F6FA20")]
pub fn stub_f6fa20() -> ! {
    todo!("0xf6fa20 sub_F6FA20")
}

// 0xf6fa2c — sub_F6FA2C
#[doc(alias = "sub_F6FA2C")]
pub fn stub_f6fa2c() -> ! {
    todo!("0xf6fa2c sub_F6FA2C")
}

// 0xf6fa38 — sub_F6FA38
#[doc(alias = "sub_F6FA38")]
pub fn stub_f6fa38() -> ! {
    todo!("0xf6fa38 sub_F6FA38")
}

// 0xf6fa44 — sub_F6FA44
#[doc(alias = "sub_F6FA44")]
pub fn stub_f6fa44() -> ! {
    todo!("0xf6fa44 sub_F6FA44")
}

// 0xf6fa50 — sub_F6FA50
#[doc(alias = "sub_F6FA50")]
pub fn stub_f6fa50() -> ! {
    todo!("0xf6fa50 sub_F6FA50")
}

// 0xf6fa5c — sub_F6FA5C
#[doc(alias = "sub_F6FA5C")]
pub fn stub_f6fa5c() -> ! {
    todo!("0xf6fa5c sub_F6FA5C")
}

// 0xf6fa68 — sub_F6FA68
#[doc(alias = "sub_F6FA68")]
pub fn stub_f6fa68() -> ! {
    todo!("0xf6fa68 sub_F6FA68")
}

// 0xf6fa74 — sub_F6FA74
#[doc(alias = "sub_F6FA74")]
pub fn stub_f6fa74() -> ! {
    todo!("0xf6fa74 sub_F6FA74")
}

// 0xf6fa80 — sub_F6FA80
#[doc(alias = "sub_F6FA80")]
pub fn stub_f6fa80() -> ! {
    todo!("0xf6fa80 sub_F6FA80")
}

// 0xf6fa8c — sub_F6FA8C
#[doc(alias = "sub_F6FA8C")]
pub fn stub_f6fa8c() -> ! {
    todo!("0xf6fa8c sub_F6FA8C")
}

// 0xf6fa98 — sub_F6FA98
#[doc(alias = "sub_F6FA98")]
pub fn stub_f6fa98() -> ! {
    todo!("0xf6fa98 sub_F6FA98")
}

// 0xf6faa4 — sub_F6FAA4
#[doc(alias = "sub_F6FAA4")]
pub fn stub_f6faa4() -> ! {
    todo!("0xf6faa4 sub_F6FAA4")
}

// 0xf6fab0 — sub_F6FAB0
#[doc(alias = "sub_F6FAB0")]
pub fn stub_f6fab0() -> ! {
    todo!("0xf6fab0 sub_F6FAB0")
}

// 0xf6fabc — sub_F6FABC
#[doc(alias = "sub_F6FABC")]
pub fn stub_f6fabc() -> ! {
    todo!("0xf6fabc sub_F6FABC")
}

// 0xf6fac8 — sub_F6FAC8
#[doc(alias = "sub_F6FAC8")]
pub fn stub_f6fac8() -> ! {
    todo!("0xf6fac8 sub_F6FAC8")
}

// 0xf6fad4 — sub_F6FAD4
#[doc(alias = "sub_F6FAD4")]
pub fn stub_f6fad4() -> ! {
    todo!("0xf6fad4 sub_F6FAD4")
}

// 0xf6fae0 — sub_F6FAE0
#[doc(alias = "sub_F6FAE0")]
pub fn stub_f6fae0() -> ! {
    todo!("0xf6fae0 sub_F6FAE0")
}

// 0xf6faec — sub_F6FAEC
#[doc(alias = "sub_F6FAEC")]
pub fn stub_f6faec() -> ! {
    todo!("0xf6faec sub_F6FAEC")
}

// 0xf6faf8 — sub_F6FAF8
#[doc(alias = "sub_F6FAF8")]
pub fn stub_f6faf8() -> ! {
    todo!("0xf6faf8 sub_F6FAF8")
}

// 0xf6fb04 — sub_F6FB04
#[doc(alias = "sub_F6FB04")]
pub fn stub_f6fb04() -> ! {
    todo!("0xf6fb04 sub_F6FB04")
}

// 0xf6fb10 — sub_F6FB10
#[doc(alias = "sub_F6FB10")]
pub fn stub_f6fb10() -> ! {
    todo!("0xf6fb10 sub_F6FB10")
}

// 0xf6fb1c — sub_F6FB1C
#[doc(alias = "sub_F6FB1C")]
pub fn stub_f6fb1c() -> ! {
    todo!("0xf6fb1c sub_F6FB1C")
}

// 0xf6fb28 — sub_F6FB28
#[doc(alias = "sub_F6FB28")]
pub fn stub_f6fb28() -> ! {
    todo!("0xf6fb28 sub_F6FB28")
}

// 0xf6fb34 — sub_F6FB34
#[doc(alias = "sub_F6FB34")]
pub fn stub_f6fb34() -> ! {
    todo!("0xf6fb34 sub_F6FB34")
}

// 0xf6fb40 — sub_F6FB40
#[doc(alias = "sub_F6FB40")]
pub fn stub_f6fb40() -> ! {
    todo!("0xf6fb40 sub_F6FB40")
}

// 0xf6fb4c — sub_F6FB4C
// type: int()
#[doc(alias = "sub_F6FB4C")]
pub fn stub_f6fb4c() -> ! {
    todo!("0xf6fb4c sub_F6FB4C")
}
