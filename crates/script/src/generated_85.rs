// Auto-generated skeletons for rbx-script — Lua|Script|Yield|lua filtered batch
// Filter: Lua|Script|Yield|lua (5401 filtered, 100 in this batch)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x7750f0..0x77ad60 | remaining 2090 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// ---- ScriptDebugger void-signal slot cluster (IDA 0x7750f0, 0x77a5ac..0x77a704) ----
// Ground truth per stub: `decompile(ea)` + `disasm(ea)` via IDA MCP.
// Boost mapping (AGENTS.md section 4): boost::shared_ptr -> rbx_core::SharedPtr
// (Arc); boost::bind/bind_t/function/mf0 -> Box<dyn Fn> closures; rbx::signals
// slots/connections -> host slot structs below; intrusive_ptr_release on the
// connection -> dropping the Option owner.
// Unmodeled throughout: C++ vtable installs (slot vtable words), RTTIThunkInfo
// unwind tables, and the iOS PIC lazy-pointer slots behind the j__ thunks.

/// was: `RBX::Scripting::DebuggerBreakpoint` — payload owned by the control
/// block below; opaque on host.
#[derive(Debug, Default)]
pub struct DebuggerBreakpoint {
    /// Breakpoint id / line payload (unmodeled layout).
    pub payload: u32,
}

/// was: `boost::detail::sp_counted_impl_pd<RBX::Scripting::DebuggerBreakpoint *,
/// RBX::Creatable<RBX::Instance>::Deleter>` — control block whose untyped
/// deleter lives 16 bytes past the block base (IDA 0x7750f0: `ADDS R0,#0x10`).
#[derive(Debug, Default)]
pub struct DebuggerBreakpointControl {
    /// Raw control-block base; the deleter slot is at +16 (IDA 0x7750f2).
    pub base: usize,
}

/// Byte offset of the untyped deleter from the control-block base.
pub const DEBUGGER_BREAKPOINT_DELETER_OFFSET: usize = 16;

// 0x7750f0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting18DebuggerBreakpointENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::DebuggerBreakpoint *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// IDA 0x7750f0: `ADDS R0,#0x10; BX LR` — return the deleter slot address.
pub fn stub_0x7750f0(block: &DebuggerBreakpointControl) -> usize {
    // IDA 0x7750f0
    block.base.wrapping_add(DEBUGGER_BREAKPOINT_DELETER_OFFSET)
}

/// was: `RBX::Scripting::ScriptDebugger` — target of the bound member fn below.
#[derive(Debug, Default)]
pub struct ScriptDebugger {
    /// Reentrancy / enabled flag (unmodeled layout).
    pub enabled: bool,
}

/// was: `rbx::signals::connection` handle held at slot +8 — released by every
/// dtor below when non-null (IDA 0x77a5ca..0x77a5d0, 0x77a6f6..0x77a6fc,
/// 0x77a74c..0x77a772). intrusive_ptr_release -> dropping the owner.
#[derive(Debug, Default)]
pub struct DebuggerConnection {
    /// Nonzero while connected (mirrors the null check on the raw pointer).
    pub connected: bool,
}

/// was: `rbx::signals::signal<void ()(void)>::callable_slot<bind_t<...mf0<
/// ScriptDebugger>...>>` — slot object: vtable pair at +0 (IDA 0x77a5c6
/// `STRD R2,R3,[R4]`: words at vtbl+8 / vtbl+0x20) plus the connection owner
/// at +8 (IDA 0x77a5ca `LDR [R4,#8]`).
pub struct DebuggerVoidSlot {
    /// Bound `ScriptDebugger::method()` closure (was: bind_t<mf0>).
    pub bound: Option<Box<dyn Fn() + Send + Sync>>,
    /// Connection owner at +8; released on destroy when present.
    pub connection: Option<DebuggerConnection>,
}

impl DebuggerVoidSlot {
    /// Shared D1 body (IDA 0x77a5ac/0x77a6d8): re-point at the `slot` base
    /// vtable, then release the +8 connection when non-null.
    fn destroy_in_place(&mut self) {
        // IDA 0x77a5c6/0x77a6f2: vtable pair reset (unmodeled words).
        self.bound = None;
        // IDA 0x77a5ca..0x77a5d0 / 0x77a6f6..0x77a6fc.
        self.connection.take();
    }
}

// 0x77a5ac — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9Scripting14ScriptDebuggerEEENS6_5list1INS6_5valueIPSC_EEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Scripting::ScriptDebugger>,boost::_bi::list1<boost::_bi::value<RBX::Scripting::ScriptDebugger*>>>>::~callable_slot()")]
// IDA 0x77a5ac (D1): vtable reset + conditional connection release, no free.
pub fn stub_0x77a5ac(slot: &mut DebuggerVoidSlot) {
    // IDA 0x77a5ac
    slot.destroy_in_place();
}

// 0x77a5d8 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9Scripting14ScriptDebuggerEEENS6_5list1INS6_5valueIPSC_EEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Scripting::ScriptDebugger>,boost::_bi::list1<boost::_bi::value<RBX::Scripting::ScriptDebugger*>>>>::~callable_slot() [0x77a5d8]")]
// IDA 0x77a5d8 (D0): D1 body (0x77a608..0x77a646) then operator delete
// (0x77a652). MODEL: consuming Box drops members and frees — same observable.
pub fn stub_0x77a5d8(slot: Box<DebuggerVoidSlot>) {
    // IDA 0x77a5d8
    drop(slot);
}

/// was: `rbx::callable<signal<void()(void)>::slot, bind_t<mf0<ScriptDebugger>>,
/// 0, void()(void)>` — callable wrapper; the bound functor lives at +0x10
/// (IDA 0x77a6b0: `ADDS R0,#0x10`).
pub struct DebuggerVoidCallable {
    /// Padding standing in for the slot base; functor starts at +0x10.
    pub _slot_base: [u8; 16],
    /// Bound functor (was: bind_t<mf0<void,ScriptDebugger>>).
    pub bound: Option<Box<dyn Fn() + Send + Sync>>,
}

impl DebuggerVoidCallable {
    /// Shared call body (IDA 0x77a6b0/0x77a6b8): tail-call the bound
    /// operator() (disasm branches to the bind_t shim).
    fn invoke(&self) {
        if let Some(bound) = &self.bound {
            bound();
        }
    }
}

// 0x77a6b0 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9Scripting14ScriptDebuggerEEENS7_5list1INS7_5valueIPSD_EEEEEELi0ES3_E4callEv
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Scripting::ScriptDebugger>,boost::_bi::list1<boost::_bi::value<RBX::Scripting::ScriptDebugger*>>>,0,void ()(void)>::call(void)")]
// IDA 0x77a6b0: `ADDS R0,#0x10; B bind_t::operator()` — invoke the functor.
pub fn stub_0x77a6b0(callable: &DebuggerVoidCallable) {
    // IDA 0x77a6b0
    callable.invoke();
}

// 0x77a6b8 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9Scripting14ScriptDebuggerEEENS7_5list1INS7_5valueIPSD_EEEEEELi0ES3_E4callEv
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Scripting::ScriptDebugger>,boost::_bi::list1<boost::_bi::value<RBX::Scripting::ScriptDebugger*>>>,0,void ()(void)>::call(void)")]
// IDA 0x77a6b8: non-virtual thunk — `ADDS R0,#0xC` (this-4 adjust to the
// callable body) then the same tail-call as 0x77a6b0.
pub fn stub_0x77a6b8(callable: &DebuggerVoidCallable) {
    // IDA 0x77a6b8: -4 this-adjust is structural (host takes &body directly).
    callable.invoke();
}

/// was: `boost::_bi::bind_t<void, mf0<void,ScriptDebugger>, list1<value<
/// ScriptDebugger*>>>` — bound member call: mf0 pointer + adjust word + object
/// (IDA 0x77a6c0 `LDM R0,{R1,R2}`: R1 = mf0, R2 = adjust; then object at +8).
pub struct DebuggerBinding {
    /// Bound member fn (was: mf0<void,ScriptDebugger>).
    pub method: fn(&mut ScriptDebugger),
    /// Virtual-adjust word: object += adjust>>1; odd bit resolves the vtbl
    /// entry (IDA 0x77a6ca..0x77a6d2).
    pub adjust: i32,
}

impl DebuggerBinding {
    /// was: `bind_t::operator()` (IDA 0x77a6c0): apply the virtual adjust,
    /// then invoke the member fn on the object.
    pub fn call(&self, debugger: &mut ScriptDebugger) {
        // IDA 0x77a6c0..0x77a6d4: LDM mf0/adjust, ADD object+(adjust>>1),
        // odd-bit vtbl resolve, BX method. MODEL: the method value is the
        // already-resolved entry (odd-bit path precomputed by the binder).
        let _ = self.adjust;
        (self.method)(debugger);
    }
}

// 0x77a6c0 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX9Scripting14ScriptDebuggerEEENS0_5list1INS0_5valueIPS6_EEEEEclEv
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Scripting::ScriptDebugger>,boost::_bi::list1<boost::_bi::value<RBX::Scripting::ScriptDebugger*>>>::operator()(void)")]
// IDA 0x77a6c0: member-call with virtual-adjust as above.
pub fn stub_0x77a6c0(binding: &DebuggerBinding, debugger: &mut ScriptDebugger) {
    // IDA 0x77a6c0
    binding.call(debugger);
}

// 0x77a6d8 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9Scripting14ScriptDebuggerEEENS7_5list1INS7_5valueIPSD_EEEEEELi0ES3_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Scripting::ScriptDebugger>,boost::_bi::list1<boost::_bi::value<RBX::Scripting::ScriptDebugger*>>>,0,void ()(void)>::~callable()")]
// IDA 0x77a6d8 (D1): same vtable-reset + conditional release shape as
// 0x77a5ac (0x77a6ea..0x77a6fc), no free. Destroys the slot half; the bound
// functor at +0x10 drops with the owner.
pub fn stub_0x77a6d8(callable: &mut DebuggerVoidCallable) {
    // IDA 0x77a6d8
    callable.bound = None;
}

// 0x77a704 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9Scripting14ScriptDebuggerEEENS7_5list1INS7_5valueIPSD_EEEEEELi0ES3_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Scripting::ScriptDebugger>,boost::_bi::list1<boost::_bi::value<RBX::Scripting::ScriptDebugger*>>>,0,void ()(void)>::~callable() [0x77a704]")]
// IDA 0x77a704 (D0): vtable reset (0x77a734/0x77a744), conditional release
// (0x77a74c..0x77a772), operator delete (0x77a77e). MODEL: consuming Box.
pub fn stub_0x77a704(callable: Box<DebuggerVoidCallable>) {
    // IDA 0x77a704
    drop(callable);
}
