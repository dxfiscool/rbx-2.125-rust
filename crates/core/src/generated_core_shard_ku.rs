//! core shard ku — 30 IDA-grounded ports 0x77a5ac-0x77e8e0.
//! Continuation of generated_core_shard_f after kk (kk took 0x777ad4-0x778c34):
//! the ScriptDebugger `rbx::signals` slot/bind_t family — `signal<void()>`
//! over `mf0` (IDA 0x77a5ac-0x77a704), `signal<void(lua_State*)>` over `mf1`
//! (IDA 0x77a9e8-0x77af80) — plus the `DebuggerBreakpoint` int-keyed
//! unordered table (IDA 0x77b150-0x77b74c), the `value<string>/arg<1>`
//! bind pair (IDA 0x77e528-0x77e644), and the Script-to-Debugger map table
//! (IDA 0x77e760-0x77e8e0).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + #[doc(alias = mangled)].
//! AGENTS.md section 4: bind/function/_bi::bind_t -> Box<dyn Fn>/closures,
//! signals/slots -> crate::signal::Signal connect/fire, unordered_map ->
//! HashMap-backed bucket tables, shared/intrusive_ptr -> rbx_core::SharedPtr.
//! Carriers in generated_core_shard_f.rs are untouched; these ports live
//! under new idiomatic names.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;

/// was: the `signal<void()>` slot family over
/// `bind_t<void, mf0<void, ScriptDebugger>, list1<value<ScriptDebugger*>>>`
/// (IDA 0x77a5ac-0x77a704). Each item notes the EA whose decompile/disasm
/// grounds it.
pub mod debugger_slot_void {
    /// was: `RBX::Scripting::ScriptDebugger *` — bound receiver
    /// (`value<ScriptDebugger*>`, list1 slot at bind+8, IDA 0x77a6c4
    /// `LDR R0,[R0,#8]`).
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct DebuggerTarget(pub usize);

    /// was: the resolved `mf0<void, ScriptDebugger>` member call
    /// (IDA 0x77a6c0 loads fn ptr + adjust, 0x77a6ce-0x77a6d2 take the
    /// virtual path, 0x77a6d4 `BX R1`).
    pub type VoidHookFn = fn(DebuggerTarget);

    /// was: the `mf0` member-pointer encoding — raw pointer/adjust word
    /// (IDA 0x77a6c0 `LDM.W R0,{R1,R2}`).
    #[derive(Debug, Default, Clone, Copy)]
    pub struct MemberFnAdjust0 {
        pub encoding: usize,
    }

    impl MemberFnAdjust0 {
        /// IDA 0x77a6c6: `TST.W R2,#1` selects the virtual path.
        pub fn is_virtual_thunk(&self) -> bool {
            self.encoding & 1 != 0
        }
        /// IDA 0x77a6ca: `ADD.W R0,R0,R2,ASR#1` — object address.
        pub fn object_word(&self, bound: usize) -> usize {
            bound.wrapping_add(self.encoding >> 1)
        }
    }

    /// was: `bind_t<void, mf0<void, ScriptDebugger>,
    /// list1<value<ScriptDebugger*>>>` — member hook plus bound receiver.
    #[derive(Clone, Copy)]
    pub struct VoidBindImage {
        pub hook: VoidHookFn,
        pub adjust: MemberFnAdjust0,
        pub debugger: DebuggerTarget,
    }

    impl VoidBindImage {
        /// IDA 0x77a6c0-0x77a6d4: resolve (virtual path 0x77a6d0
        /// `LDRNE R2,[R0]`, 0x77a6d2 `LDRNE R1,[R2,R1]`) then `BX R1`.
        /// The `fn` pointer already encodes the resolved target, so
        /// dispatch collapses to the call.
        pub fn invoke(&self) {
            (self.hook)(self.debugger);
        }
    }

    /// was: `rbx::signals::signal<void()>::callable_slot<bind_t<...>>` /
    /// `rbx::callable<slot, bind_t<...>, 0, void()>` — empty or one bound
    /// image. D1 (IDA 0x77a5ac/0x77a6d8) resets both vtable words to the
    /// slot vtable and releases the +8 islot; D0 (IDA 0x77a5d8/0x77a704)
    /// then frees the object (IDA 0x77a652/0x77a77e `operator delete`).
    #[derive(Default, Clone)]
    pub enum VoidSlot {
        #[default]
        Empty,
        Bound(Box<VoidBindImage>),
    }

    impl VoidSlot {
        pub fn is_empty(&self) -> bool {
            matches!(self, VoidSlot::Empty)
        }
    }
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Scripting::ScriptDebugger>,boost::_bi::list1<boost::_bi::value<RBX::Scripting::ScriptDebugger*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9Scripting14ScriptDebuggerEEENS6_5list1INS6_5valueIPSC_EEEEEEED1Ev")]
// 0x77a5ac — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9Scripting14ScriptDebuggerEEENS6_5list1INS6_5valueIPSC_EEEEEEED1Ev
pub fn stub_0x77a5ac(slot: &mut debugger_slot_void::VoidSlot) {
    // IDA 0x77a5be/0x77a5c2: both vtable words reset to the slot vtable
    // (fixed layout in Rust — dropping the image is the reset).
    // IDA 0x77a5ca-0x77a5d0: `v3 = +8; if (v3) intrusive_ptr_release(v3)`.
    // Arc drop is the release; clearing first keeps D1 ordering.
    *slot = debugger_slot_void::VoidSlot::Empty;
}


#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Scripting::ScriptDebugger>,boost::_bi::list1<boost::_bi::value<RBX::Scripting::ScriptDebugger*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9Scripting14ScriptDebuggerEEENS6_5list1INS6_5valueIPSC_EEEEEEED0Ev")]
// 0x77a5d8 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9Scripting14ScriptDebuggerEEENS6_5list1INS6_5valueIPSC_EEEEEEED0Ev
pub fn stub_0x77a5d8(slot: debugger_slot_void::VoidSlot) {
    // IDA 0x77a608-0x77a64e: D1 body (vtable reset + conditional release),
    // then 0x77a652 `operator delete(a1)`. Reuse D1, then by-value drop is
    // the free — dtor-then-free in the same order.
    let mut slot = slot;
    stub_0x77a5ac(&mut slot);
}


#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Scripting::ScriptDebugger>,boost::_bi::list1<boost::_bi::value<RBX::Scripting::ScriptDebugger*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9Scripting14ScriptDebuggerEEENS7_5list1INS7_5valueIPSD_EEEEEELi0ES3_E4callEv")]
// 0x77a6b0 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9Scripting14ScriptDebuggerEEENS7_5list1INS7_5valueIPSD_EEEEEELi0ES3_E4callEv
pub fn stub_0x77a6b0(slot: &debugger_slot_void::VoidSlot) {
    // IDA 0x77a6b0: `ADDS R0,#0x10` steps over the slot/callable prefix to
    // the bind image, then 0x77a6b2 `B.W` tail-calls bind_t::operator().
    if let debugger_slot_void::VoidSlot::Bound(image) = slot {
        image.invoke();
    }
}


#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Scripting::ScriptDebugger>,boost::_bi::list1<boost::_bi::value<RBX::Scripting::ScriptDebugger*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9Scripting14ScriptDebuggerEEENS7_5list1INS7_5valueIPSD_EEEEEELi0ES3_E4callEv")]
// 0x77a6b8 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9Scripting14ScriptDebuggerEEENS7_5list1INS7_5valueIPSD_EEEEEELi0ES3_E4callEv
pub fn stub_0x77a6b8(slot: &debugger_slot_void::VoidSlot) {
    // IDA 0x77a6b8: `ADDS R0,#0xC` — the `this - 4` non-virtual thunk: the
    // slot base sits 4 bytes into the callable, so +0xC reaches the same
    // bind image as +0x10 above, then the same tail-call (0x77a6ba).
    stub_0x77a6b0(slot);
}


#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Scripting::ScriptDebugger>,boost::_bi::list1<boost::_bi::value<RBX::Scripting::ScriptDebugger*>>>::operator()(void)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX9Scripting14ScriptDebuggerEEENS0_5list1INS0_5valueIPS6_EEEEEclEv")]
// 0x77a6c0 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX9Scripting14ScriptDebuggerEEENS0_5list1INS0_5valueIPS6_EEEEEclEv
pub fn stub_0x77a6c0(image: &debugger_slot_void::VoidBindImage) {
    // IDA 0x77a6c0: `LDM.W R0,{R1,R2}` loads fn ptr + adjust.
    // IDA 0x77a6c4: `LDR R0,[R0,#8]` loads the bound debugger.
    // IDA 0x77a6c6/0x77a6ca: `TST.W R2,#1` / adjust `ADD.W R0,R0,R2,ASR#1`.
    // IDA 0x77a6ce-0x77a6d2 (`ITT NE`): virtual path reloads through the
    // vtable slot; 0x77a6d4 `BX R1` invokes.
    image.invoke();
}


#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Scripting::ScriptDebugger>,boost::_bi::list1<boost::_bi::value<RBX::Scripting::ScriptDebugger*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9Scripting14ScriptDebuggerEEENS7_5list1INS7_5valueIPSD_EEEEEELi0ES3_ED1Ev")]
// 0x77a6d8 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9Scripting14ScriptDebuggerEEENS7_5list1INS7_5valueIPSD_EEEEEELi0ES3_ED1Ev
pub fn stub_0x77a6d8(slot: &mut debugger_slot_void::VoidSlot) {
    // IDA 0x77a6ea/0x77a6ee: vtable words reset to the same slot vtable pair
    // as 0x77a5ac; IDA 0x77a6f6-0x77a6fc: conditional release of +8.
    // `rbx::callable` shares the slot prefix layout, so D1 is identical.
    stub_0x77a5ac(slot);
}


#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Scripting::ScriptDebugger>,boost::_bi::list1<boost::_bi::value<RBX::Scripting::ScriptDebugger*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9Scripting14ScriptDebuggerEEENS7_5list1INS7_5valueIPSD_EEEEEELi0ES3_ED0Ev")]
// 0x77a704 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9Scripting14ScriptDebuggerEEENS7_5list1INS7_5valueIPSD_EEEEEELi0ES3_ED0Ev
pub fn stub_0x77a704(slot: debugger_slot_void::VoidSlot) {
    // IDA 0x77a734-0x77a77a: D1 body, then 0x77a77e `operator delete`.
    stub_0x77a5d8(slot);
}
/// was: the `signal<void(lua_State*)>` slot family over
/// `bind_t<void, mf1<void, ScriptDebugger, lua_State*>,
/// list2<value<ScriptDebugger*>, arg<1>>>` (IDA 0x77a9e8-0x77af80).
/// Same slot/callable shape as `debugger_slot_void` with one call arg.
pub mod debugger_slot_state {
    /// was: `RBX::Scripting::ScriptDebugger *` — bound receiver
    /// (`value<ScriptDebugger*>`, list2 slot at bind+8, IDA 0x77ac5c
    /// `LDR R0,[R0,#8]`).
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct DebuggerTarget(pub usize);

    /// was: `lua_State *` — call arg carried by `arg<1>`. The caller spills
    /// the pointer to the stack and passes its address (IDA 0x77ac34-0x77ac3c),
    /// so the bind sees a `lua_State **` and derefs (IDA 0x77ac62
    /// `LDR R1,[R1]`).
    pub type LuaStatePtr = usize;

    /// was: the resolved `mf1<void, ScriptDebugger, lua_State*>` member call
    /// (IDA 0x77ac58 loads fn ptr + adjust, 0x77ac68-0x77ac6c take the
    /// virtual path, 0x77ac6e `BX R2`).
    pub type StateHookFn = fn(DebuggerTarget, LuaStatePtr);

    /// was: the `mf1` member-pointer encoding
    /// (IDA 0x77ac58 `LDM.W R0,{R2,R3}`).
    #[derive(Debug, Default, Clone, Copy)]
    pub struct MemberFnAdjust1 {
        pub encoding: usize,
    }

    impl MemberFnAdjust1 {
        /// IDA 0x77ac5e: `TST.W R3,#1` selects the virtual path.
        pub fn is_virtual_thunk(&self) -> bool {
            self.encoding & 1 != 0
        }
        /// IDA 0x77ac64: `ADD.W R0,R0,R3,ASR#1` — object address.
        pub fn object_word(&self, bound: usize) -> usize {
            bound.wrapping_add(self.encoding >> 1)
        }
    }

    /// was: `bind_t<void, mf1<...>,
    /// list2<value<ScriptDebugger*>, arg<1>>>` — member hook, bound
    /// receiver, and the call-arg placeholder.
    #[derive(Clone, Copy)]
    pub struct StateBindImage {
        pub hook: StateHookFn,
        pub adjust: MemberFnAdjust1,
        pub debugger: DebuggerTarget,
    }

    impl StateBindImage {
        /// IDA 0x77ac58-0x77ac6e: resolve (virtual path 0x77ac68-0x77ac6c)
        /// then invoke with the dereferenced state pointer.
        pub fn invoke(&self, state: LuaStatePtr) {
            (self.hook)(self.debugger, state);
        }
    }

    /// was: `signal<void(lua_State*)>::callable_slot<bind_t<...>>` /
    /// `rbx::callable<slot, bind_t<...>, 1, void(lua_State*)>` — empty or one
    /// bound image. D1 (IDA 0x77aa14/0x77af54) resets both vtable words
    /// (off_12A02E8/12A0300) and releases +8; D0 (0x77aa40/0x77af80) frees
    /// (IDA 0x77aaba/0x77affa `operator delete`).
    #[derive(Default, Clone)]
    pub enum StateSlot {
        #[default]
        Empty,
        Bound(Box<StateBindImage>),
    }
}


#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(lua_State *)>::slot>::operator=(rbx::signals::signal<void ()(lua_State *)>::slot*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvP9lua_StateEE4slotEEaSEPS8_")]
// 0x77a9e8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvP9lua_StateEE4slotEEaSEPS8_
pub fn stub_0x77a9e8(
    dst: &mut Option<SharedPtr<debugger_slot_state::StateSlot>>,
    src: Option<SharedPtr<debugger_slot_state::StateSlot>>,
) -> Option<SharedPtr<debugger_slot_state::StateSlot>> {
    // IDA 0x77a9f2-0x77a9f6 (`ITT NE`): add-ref the incoming slot first, so
    // self-assignment stays alive; `Arc::clone` is the add-ref.
    let incoming = src;
    // IDA 0x77a9fa-0x77a9fc: swap the pointer; 0x77aa00-0x77aa02: release
    // the old slot. Clone-then-drop preserves the add-before-release order,
    // and dropping the replaced value is the release.
    let old = std::mem::replace(dst, incoming.clone());
    drop(old);
    // IDA 0x77aa08: returns the destination slot.
    dst.clone()
}


#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Scripting::ScriptDebugger,lua_State *>,boost::_bi::list2<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvP9lua_StateEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED1Ev")]
// 0x77aa14 — __ZN3rbx7signals6signalIFvP9lua_StateEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED1Ev
pub fn stub_0x77aa14(slot: &mut debugger_slot_state::StateSlot) {
    // IDA 0x77aa26/0x77aa2a: both vtable words reset to the lua_State-slot
    // vtable (fixed layout in Rust — dropping the image is the reset).
    // IDA 0x77aa32-0x77aa38: conditional release of +8.
    *slot = debugger_slot_state::StateSlot::Empty;
}


#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Scripting::ScriptDebugger,lua_State *>,boost::_bi::list2<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvP9lua_StateEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED0Ev")]
// 0x77aa40 — __ZN3rbx7signals6signalIFvP9lua_StateEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED0Ev
pub fn stub_0x77aa40(slot: debugger_slot_state::StateSlot) {
    // IDA 0x77aa70-0x77aaae: D1 body, then 0x77aaba `operator delete`.
    let mut slot = slot;
    stub_0x77aa14(&mut slot);
}


#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(lua_State *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Scripting::ScriptDebugger,lua_State *>,boost::_bi::list2<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>>>,1,void ()(lua_State *)>::call(lua_State *)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvP9lua_StateEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_")]
// 0x77ac30 — __ZN3rbx8callableINS_7signals6signalIFvP9lua_StateEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
pub fn stub_0x77ac30(slot: &debugger_slot_state::StateSlot, state: debugger_slot_state::LuaStatePtr) {
    // IDA 0x77ac34: `PUSH.W {R1}` spills the lua_State* to the stack;
    // 0x77ac38 `ADDS R0,#0x10` reaches the bind image; 0x77ac3a/0x77ac42
    // pass the spill address and tail-call bind_t::operator()<lua_State*>.
    // BUG (preserved shape): the callee receives `&state` and must deref
    // (see 0x77ac58) — the port passes the value, which is that deref.
    if let debugger_slot_state::StateSlot::Bound(image) = slot {
        image.invoke(state);
    }
}


#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(lua_State *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Scripting::ScriptDebugger,lua_State *>,boost::_bi::list2<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>>>,1,void ()(lua_State *)>::call(lua_State *)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvP9lua_StateEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_")]
// 0x77ac44 — __ZThn4_N3rbx8callableINS_7signals6signalIFvP9lua_StateEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
pub fn stub_0x77ac44(slot: &debugger_slot_state::StateSlot, state: debugger_slot_state::LuaStatePtr) {
    // IDA 0x77ac4c: `ADDS R0,#0xC` — the `this - 4` non-virtual thunk over
    // the same bind image, same spill (0x77ac48) and tail-call (0x77ac50).
    stub_0x77ac30(slot, state);
}


#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Scripting::ScriptDebugger,lua_State *>,boost::_bi::list2<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>>>::operator()<lua_State *>(lua_State * &)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateEENS0_5list2INS0_5valueIPS6_EENS_3argILi1EEEEEEclIS8_EEvRT_")]
// 0x77ac58 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateEENS0_5list2INS0_5valueIPS6_EENS_3argILi1EEEEEEclIS8_EEvRT_
pub fn stub_0x77ac58(image: &debugger_slot_state::StateBindImage, state: debugger_slot_state::LuaStatePtr) {
    // IDA 0x77ac58: `LDM.W R0,{R2,R3}` fn ptr + adjust; 0x77ac5c bound
    // debugger; 0x77ac5e `TST.W R3,#1`; 0x77ac62 `LDR R1,[R1]` derefs the
    // spilled state pointer; 0x77ac64 adjust; 0x77ac68-0x77ac6c virtual
    // path; 0x77ac6e `BX R2`.
    image.invoke(state);
}


#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(lua_State *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Scripting::ScriptDebugger,lua_State *>,boost::_bi::list2<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>>>,1,void ()(lua_State *)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvP9lua_StateEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED1Ev")]
// 0x77af54 — __ZN3rbx8callableINS_7signals6signalIFvP9lua_StateEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED1Ev
pub fn stub_0x77af54(slot: &mut debugger_slot_state::StateSlot) {
    // IDA 0x77af66/0x77af6a: vtable reset; 0x77af72-0x77af78: release +8.
    // `rbx::callable` shares the slot prefix layout, so D1 is identical.
    stub_0x77aa14(slot);
}


#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(lua_State *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Scripting::ScriptDebugger,lua_State *>,boost::_bi::list2<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>>>,1,void ()(lua_State *)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvP9lua_StateEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED0Ev")]
// 0x77af80 — __ZN3rbx8callableINS_7signals6signalIFvP9lua_StateEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED0Ev
pub fn stub_0x77af80(slot: debugger_slot_state::StateSlot) {
    // IDA 0x77afb0-0x77afee: D1 body, then 0x77affa `operator delete`.
    stub_0x77aa40(slot);
}
/// was: `boost::_bi::list2<value<std::string>, arg<1>>` /
/// `storage2<value<std::string>, arg<1>>` — one owned string plus the
/// first-call-arg placeholder (IDA 0x77e528/0x77e644 copy the string by
/// placement-new under an SjLj guard frame: 0x77e52c-0x77e548 and
/// 0x77e648-0x77e664 spill/restore D8-D15 and register unwind info).
/// No unwinding in Rust, so the guard collapses to a clone.
pub mod bind_string_arg {
    /// was: the list2/storage2 bound image — fixed string plus call-time
    /// first-argument slot (`boost::arg<1>`).
    #[derive(Debug, Default, Clone)]
    pub struct StringArg1 {
        pub fixed: String,
        pub use_caller_arg: bool,
    }

    impl StringArg1 {
        pub fn new(fixed: String) -> Self {
            Self { fixed, use_caller_arg: true }
        }
    }
}


#[doc(alias = "boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>::list2(boost::_bi::value<std::string>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueISsEENS_3argILi1EEEEC2ES3_S5_")]
// 0x77e528 — __ZN5boost3_bi5list2INS0_5valueISsEENS_3argILi1EEEEC2ES3_S5_
pub fn stub_0x77e528(fixed: &str) -> bind_string_arg::StringArg1 {
    // IDA 0x77e528-0x77e548: SjLj guard frame, then the string is copied
    // into the bound slot by placement-new.
    bind_string_arg::StringArg1::new(String::from(fixed))
}


#[doc(alias = "boost::_bi::storage2<boost::_bi::value<std::string>,boost::arg<1>>::storage2(boost::_bi::value<std::string>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost3_bi8storage2INS0_5valueISsEENS_3argILi1EEEEC2ES3_S5_")]
// 0x77e644 — __ZN5boost3_bi8storage2INS0_5valueISsEENS_3argILi1EEEEC2ES3_S5_
pub fn stub_0x77e644(fixed: String) -> bind_string_arg::StringArg1 {
    // IDA 0x77e644-0x77e664: same guard frame; storage2 is list2's storage
    // base, so construction is identical with the string moved in.
    bind_string_arg::StringArg1::new(fixed)
}

/// was: `boost::unordered::detail::table` /
/// `table_impl<map<alloc<pair<int const, DebuggerBreakpoint*>>, int,
/// DebuggerBreakpoint*, hash<int>, equal_to<int>>>` — the
/// DebuggerBreakpoint registry keyed by breakpoint id (IDA 0x77b150-0x77b74c).
/// Layout observed in disasm: buckets pointer +20 (a1[5], IDA 0x77b2d2),
/// bucket count +16 (a1[4], IDA 0x77b2d6), element count +8 (a1[2],
/// IDA 0x77b69a/0x77b5a8), max-load float +12 (IDA 0x77b454 `VLDR S0,[R0,#0xC]`).
pub mod breakpoint_map {
    /// was: `RBX::Scripting::DebuggerBreakpoint *` — opaque pointee; only
    /// the pointer value participates (hash/compare touch the int key).
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct BreakpointPtr(pub usize);

    /// was: one `ptr_node<pair<int const, DebuggerBreakpoint*>>` — the 0x10
    /// allocation at IDA 0x77b57a holds link words and the pair; `hash`
    /// caches `*(node+12)` consumed by place_in_bucket (IDA 0x77b51c
    /// `LDR R0,[R5,#0xC]`).
    #[derive(Debug, Clone, Copy)]
    pub struct BreakEntry {
        pub hash: usize,
        pub key: i32,
        pub value: BreakpointPtr,
    }

    /// was: `node_constructor<alloc<ptr_node<pair<...>>>>` — +4 held node
    /// (IDA 0x77b562 `LDR R0,[R4,#4]`), +8 state bytes (IDA 0x77b576),
    /// +9 adopted flag (IDA 0x77b568/0x77b570).
    #[derive(Debug, Default, Clone)]
    pub struct NodeCtor {
        pub node: Option<RawNode>,
        pub armed: bool,
    }

    /// was: the 0x10 `ptr_node` (IDA 0x77b57a `operator new(0x10)`,
    /// 0x77b588 second-qword zero, 0x77b576 first words zero).
    #[derive(Debug, Default, Clone, Copy)]
    pub struct RawNode {
        pub link: [usize; 2],
        pub payload: [usize; 2],
    }

    impl RawNode {
        pub fn zeroed() -> Self {
            Self { link: [0; 2], payload: [0; 2] }
        }
    }

    /// was: the table above — bucket chains plus element count and the
    /// max-load factor (+12). Backs every int-map stub below.
    #[derive(Debug, Default, Clone)]
    pub struct BreakTable {
        pub buckets: Vec<Vec<BreakEntry>>,
        pub len: usize,
        pub max_load: f32,
    }

    impl BreakTable {
        pub fn new() -> Self {
            Self { buckets: Vec::new(), len: 0, max_load: 1.0 }
        }
        pub fn bucket_count(&self) -> usize {
            self.buckets.len()
        }
    }

    /// was: `boost::hash<int>` — 32-bit identity. `as u32` keeps the
    /// 32-bit wrap on 64-bit hosts so `___umodsi3` (IDA 0x77b5ae/0x77b6ca/
    /// 0x77b760) sees the same value as the 32-bit original.
    pub fn int_hash(key: i32) -> usize {
        key as u32 as usize
    }

    /// was: the `prime_list_template<unsigned long>::value` walk
    /// (IDA 0x77b48e-0x77b4a8 over the table at 0x77b496) — first prime
    /// >= need. Computed by trial division; same observable mapping.
    /// BUG (divergence risk, documented): the overflow arm (need 0) returns
    /// 2 here while the original returns `prime_list[0]` (table head unseen
    /// past 0x77b496). Unreachable without >= 2^32 elements.
    pub fn next_prime_at_least(need: u64) -> usize {
        let mut n = need.max(2);
        if n > 2 && n % 2 == 0 {
            n += 1;
        }
        while !is_prime(n) {
            n += 2;
        }
        n as usize
    }

    fn is_prime(n: u64) -> bool {
        if n < 2 {
            return false;
        }
        if n % 2 == 0 {
            return n == 2;
        }
        let mut d = 3u64;
        while d * d <= n {
            if n % d == 0 {
                return false;
            }
            d += 2;
        }
        true
    }
}


#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::operator[](int const&)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEEixERS5_")]
// 0x77b150 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEEixERS5_
pub fn stub_0x77b150(table: &mut breakpoint_map::BreakTable, key: i32) -> breakpoint_map::BreakpointPtr {
    // IDA 0x77b1ac/0x77b1ae (`CMP R6,#0; BNE loc_77B27E`): hit → return the
    // mapped slot. Miss path builds a node through node_constructor
    // (0x77b1c8 `BLX`), stores the key (0x77b1d4-0x77b1d6 `STRNE [R2], key`)
    // and zeroes the mapped slot (0x77b1da `STR [R2,#4], 0`).
    // BUG (preserved): operator[] default-inserts a null breakpoint on miss.
    use breakpoint_map::{BreakEntry, BreakpointPtr};
    if table.buckets.is_empty() {
        stub_0x77b320(table, 1);
    }
    let idx = breakpoint_map::int_hash(key).wrapping_rem(table.bucket_count());
    if let Some(entry) = table.buckets[idx].iter().find(|e| e.key == key) {
        return entry.value;
    }
    table.buckets[idx].push(BreakEntry { hash: breakpoint_map::int_hash(key), key, value: BreakpointPtr(0) });
    table.len += 1;
    BreakpointPtr(0)
}


#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::reserve_for_insert(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEE18reserve_for_insertEm")]
// 0x77b2cc — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEE18reserve_for_insertEm
pub fn stub_0x77b2cc(table: &mut breakpoint_map::BreakTable, wanted: usize) -> usize {
    // IDA 0x77b2d2-0x77b2d4 (`LDR R0,[R4,#0x14]; CBZ loc_77B2FA`): no buckets
    // yet → keep the current count. IDA 0x77b2d6-0x77b2dc: current (a1[4])
    // >= wanted → return current (`POPCS`).
    // Else need = min_buckets_for_size (0x77b2f0 `BLX`), then grow
    // `size + (size >> 1)` (0x77b2e0 `ADD.W R0,R0,R0,LSR#1`), create buckets,
    // and rehash into them.
    if table.buckets.is_empty() {
        return table.bucket_count();
    }
    if table.bucket_count() >= wanted {
        return table.bucket_count();
    }
    let need = stub_0x77b448(table.max_load, table.len as u32);
    let grow = table.len + (table.len >> 1);
    stub_0x77b320(table, need.max(grow).max(wanted));
    stub_0x77b4d8(table);
    table.bucket_count()
}


#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::create_buckets(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEE14create_bucketsEm")]
// 0x77b320 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEE14create_bucketsEm
pub fn stub_0x77b320(table: &mut breakpoint_map::BreakTable, count: usize) {
    // IDA 0x77b33c (`MOV R4,R1`): requested count saved; bucket storage is
    // allocated and heads zeroed. Replaces the array, preserving no entries
    // itself — callers rehash afterwards (cf. 0x77b4de).
    table.buckets = vec![Vec::new(); count.max(1)];
}


#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::min_buckets_for_size(unsigned long)const")]
#[doc(alias = "__ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEE20min_buckets_for_sizeEm")]
// 0x77b448 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEE20min_buckets_for_sizeEm
pub fn stub_0x77b448(max_load: f32, size: u32) -> usize {
    // IDA 0x77b450-0x77b45c (`VCVT`/`VDIV`): `size / mlf` (+12, 0x77b454);
    // 0x77b464 `BLX _floor`; 0x77b468-0x77b47c: keep only when below
    // 4294967300.0 (`VLDR D18, =4.2949673e9`); 0x77b486: +1;
    // 0x77b48e-0x77b4a8: first prime_list entry >= that.
    // BUG (preserved): exact multiples still round up one (`floor(x) + 1`),
    // and overflow (floor >= 4294967300.0) falls back to need 0.
    let floored = (size as f64 / max_load as f64).floor();
    let need = if floored < 4294967300.0 { floored as u64 + 1 } else { 0 };
    breakpoint_map::next_prime_at_least(need)
}


#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::rehash_impl(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEE11rehash_implEm")]
// 0x77b4d8 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEE11rehash_implEm
pub fn stub_0x77b4d8(table: &mut breakpoint_map::BreakTable) {
    // IDA 0x77b4de: `BLX create_buckets` — fresh bucket array (sized by the
    // caller, cf. reserve_for_insert); then 0x77b4e2-0x77b4e6 walk
    // `[buckets + idx*4]` (0x77b4e6 `LDR.W R2,[R1,R0,LSL#2]`),
    // 0x77b4ec-0x77b4ee: empty bucket → done; else unlink each node and
    // place_in_bucket into the new array.
    let mut all = Vec::new();
    for chain in table.buckets.iter_mut() {
        all.append(chain);
    }
    table.len = 0;
    let count = table.bucket_count().max(1);
    for entry in all {
        let idx = entry.hash.wrapping_rem(count);
        table.buckets[idx].push(entry);
        table.len += 1;
    }
}


#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>> &,boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE")]
// 0x77b504 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE
pub fn stub_0x77b504(table: &mut breakpoint_map::BreakTable, entry: breakpoint_map::BreakEntry) {
    // IDA 0x77b50a-0x77b512: null node → 0, else `node - 8` header adjust
    // (`SUBNE.W R5,R1,#8`); 0x77b518-0x77b51c: `bucket = *(adj+12) % count`;
    // the occupied-head branch links the node into that bucket.
    let count = table.bucket_count().max(1);
    let idx = entry.hash.wrapping_rem(count);
    table.buckets[idx].insert(0, entry);
    table.len += 1;
}


#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>>>::construct(void)")]
#[doc(alias = "__ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEEEE9constructEv")]
// 0x77b55c — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEEEE9constructEv
pub fn stub_0x77b55c(ctor: &mut breakpoint_map::NodeCtor) -> bool {
    // IDA 0x77b562-0x77b566: existing node (`[R4,#4]`) → 0x77b568 checks flag
    // byte+9; set → clear it (0x77b56e-0x77b570 `MOVNE/STRBNE`) and return 0:
    // the node is adopted, no construction needed.
    // Else 0x77b576 zeroes words+8; 0x77b57a `operator new(0x10)`; 0x77b57e
    // stores the node; 0x77b588 zeroes the second qword; byte+8 = 1
    // (owned + constructed). Returns whether a fresh node was allocated.
    if ctor.node.is_some() {
        ctor.armed = false;
        false
    } else {
        ctor.node = Some(breakpoint_map::RawNode::zeroed());
        ctor.armed = true;
        true
    }
}


#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::find_node_impl<int,std::equal_to<int>>(unsigned long,int const&,std::equal_to<int> const&)const")]
#[doc(alias = "__ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEE14find_node_implIiSF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEmRKT_RKT0_")]
// 0x77b594 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEE14find_node_implIiSF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEmRKT_RKT0_
pub fn stub_0x77b594(table: &breakpoint_map::BreakTable, hash: usize, key: i32) -> Option<breakpoint_map::BreakpointPtr> {
    // IDA 0x77b5a0/0x77b5a8: bucket count (+4), size (+8, nonzero guard
    // 0x77b5b6); 0x77b5ae `BLX ___umodsi3` → `hash % count`; 0x77b5b4
    // default null; 0x77b5bc head = buckets[idx]; walk the chain comparing
    // stored hash then key (`equal_to<int>`).
    if table.buckets.is_empty() {
        return None;
    }
    let idx = hash.wrapping_rem(table.bucket_count());
    table.buckets[idx].iter().find(|e| e.hash == hash && e.key == key).map(|e| e.value)
}


#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::erase_key(int const&)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEE9erase_keyERS5_")]
// 0x77b690 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEE9erase_keyERS5_
pub fn stub_0x77b690(table: &mut breakpoint_map::BreakTable, key: i32) -> Option<breakpoint_map::BreakpointPtr> {
    // IDA 0x77b69a-0x77b69c: empty (`+8 == 0`) → 0 (0x77b6e8-0x77b6ea `MOVS/B`).
    // 0x77b6a2 key load; 0x77b6ac bucket; walk from the head (0x77b6b2):
    // 0x77b6ca recompute `hash % count`, 0x77b6ce bucket match, 0x77b6d2-0x77b6dc
    // key match, 0x77b6de-0x77b6e6 next-or-miss.
    // Hit: 0x77b6f6 `fix_bucket` unlinks, 0x77b702 `delete_nodes` frees the
    // single node, 0x77b6fa/0x77b706 return the erased node.
    // BUG (preserved): returns the deleted node pointer, not an erase count.
    if table.len == 0 || table.buckets.is_empty() {
        return None;
    }
    let hash = breakpoint_map::int_hash(key);
    let idx = hash.wrapping_rem(table.bucket_count());
    let pos = table.buckets[idx].iter().position(|e| e.hash == hash && e.key == key)?;
    let entry = table.buckets[idx].remove(pos);
    stub_0x77b74c(table, idx);
    table.len -= 1;
    Some(entry.value)
}


#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::delete_nodes(boost::unordered::detail::ptr_bucket *,boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEE12delete_nodesEPNS1_10ptr_bucketESJ_")]
// 0x77b710 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEE12delete_nodesEPNS1_10ptr_bucketESJ_
pub fn stub_0x77b710(table: &mut breakpoint_map::BreakTable, bucket: usize) -> usize {
    // IDA 0x77b71c-0x77b740: `do { head ? node = head - 8 : 0
    // (0x77b728); *first = node[2] (0x77b72e); operator delete(node)
    // (0x77b730); ++count (0x77b736); --size (0x77b73a); }
    // while (*first != last) (0x77b740)`. Frees one bucket chain and
    // returns the freed count; the `last` sentinel collapses into the
    // chain end in the Vec model.
    if bucket >= table.bucket_count() {
        return 0;
    }
    let freed = table.buckets[bucket].len();
    table.len -= freed;
    table.buckets[bucket].clear();
    freed
}


#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEE10fix_bucketEmPNS1_10ptr_bucketE")]
// 0x77b74c — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEE10fix_bucketEmPNS1_10ptr_bucketE
pub fn stub_0x77b74c(table: &mut breakpoint_map::BreakTable, bucket: usize) -> usize {
    // IDA 0x77b752: `*node == 0` → the empty branch (0x77b776 tail).
    // Else 0x77b75c-0x77b760 recompute `hash(head) % count` (`___umodsi3`);
    // 0x77b766-0x77b76a: same bucket → return it; else 0x77b76c-0x77b770
    // store the node as the head of its own bucket. Returns the bucket
    // that now owns the head. Heads always hash home in the Vec model, so
    // the relocate arm only fires after manual bucket surgery.
    if bucket >= table.bucket_count() {
        return bucket;
    }
    let count = table.bucket_count();
    let home = table.buckets[bucket].first().map(|e| e.hash.wrapping_rem(count));
    match home {
        Some(own) if own != bucket => {
            let entry = table.buckets[bucket].remove(0);
            table.buckets[own].insert(0, entry);
            own
        }
        _ => bucket,
    }
}

/// was: `table_impl<map<alloc<pair<Script const* const, ScriptDebugger*>>,
/// Script const*, ScriptDebugger*, hash<Script const*>,
/// equal_to<Script const*>>>` — Script-to-Debugger registry using the same
/// bucket machine (IDA 0x77e760's 147-line SjLj frame mirrors 0x77b150;
/// 0x77e8e0's fast path mirrors 0x77b2cc: 0x77e8e6 `CBZ`, 0x77e8ea-0x77e8f0
/// current-vs-wanted, 0x77e8f4 `ADD.W R0,R0,R0,LSR#1` growth).
pub mod script_debugger_map {
    /// was: `RBX::Script const *` — map key, hashed by pointer value.
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct ScriptPtr(pub usize);

    /// was: `RBX::Scripting::ScriptDebugger *` — mapped debugger.
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct ScriptDebuggerPtr(pub usize);

    /// was: the table above — pointer-keyed bucket chains plus count.
    #[derive(Debug, Default, Clone)]
    pub struct ScriptDebuggerTable {
        pub buckets: Vec<Vec<(usize, ScriptDebuggerPtr)>>,
        pub len: usize,
        pub max_load: f32,
    }

    impl ScriptDebuggerTable {
        pub fn new() -> Self {
            Self { buckets: Vec::new(), len: 0, max_load: 1.0 }
        }
    }
}


#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::operator[](RBX::Script const* const&)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEEixERS9_")]
// 0x77e760 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEEixERS9_
pub fn stub_0x77e760(table: &mut script_debugger_map::ScriptDebuggerTable, key: usize) -> script_debugger_map::ScriptDebuggerPtr {
    // Find-or-insert mirroring 0x77b150: hit → mapped debugger; miss →
    // node_constructor path stores the key and zeroes the mapped slot.
    // BUG (preserved): default-inserts a null debugger on miss.
    use script_debugger_map::ScriptDebuggerPtr;
    if table.buckets.is_empty() {
        table.buckets = vec![Vec::new(); 1];
    }
    let idx = key.wrapping_rem(table.buckets.len());
    if let Some((_, value)) = table.buckets[idx].iter().find(|(k, _)| *k == key) {
        return *value;
    }
    table.buckets[idx].push((key, ScriptDebuggerPtr(0)));
    table.len += 1;
    ScriptDebuggerPtr(0)
}


#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::reserve_for_insert(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE18reserve_for_insertEm")]
// 0x77e8e0 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE18reserve_for_insertEm
pub fn stub_0x77e8e0(table: &mut script_debugger_map::ScriptDebuggerTable, wanted: usize) -> usize {
    // IDA 0x77e8e6-0x77e8e8 (`LDR R0,[R4,#0x14]; CBZ loc_77E90E`): no buckets
    // → keep; 0x77e8ea-0x77e8f0: current >= wanted → return current
    // (`POPCS`); else grow `size + (size >> 1)` (0x77e8f4) and re-bucket.
    if table.buckets.is_empty() {
        return 0;
    }
    if table.buckets.len() >= wanted {
        return table.buckets.len();
    }
    let grow = table.len + (table.len >> 1);
    table.buckets.resize_with(grow.max(wanted).max(1), Vec::new);
    table.buckets.len()
}
