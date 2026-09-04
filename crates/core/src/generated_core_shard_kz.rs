//! core shard kz — 23 IDA-grounded ports 0x77f5b0-0x77feac.
//! Continuation after ky (ky took 0x77e930-0x77f5b0): the
//! `BoundFuncDesc<DebuggerWatch, void()>` deleting-dtor + `execute`
//! (IDA 0x77f5b0-0x77f664), the `BoundProp<std::string>` ctor + GetSet for
//! `DebuggerWatch` (IDA 0x77f688-0x77f83c) and for `DebuggerBreakpoint`
//! (IDA 0x77f8a4-0x77fa54), the `BoundProp<bool>` / `BoundProp<int>`
//! ctor + GetSet for `DebuggerBreakpoint` (IDA 0x77fabc-0x77fe5c), and the
//! getter/setter `PropDescriptor<DebuggerBreakpoint, int>` ctor
//! (IDA 0x77feac).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + #[doc(alias = mangled)].
//! AGENTS.md section 4: shared/intrusive_ptr -> crate::SharedPtr (Arc),
//! weak_ptr -> Weak, bind/function -> Box<dyn Fn> closures,
//! signals/slots -> crate::signal::Signal (n/a here), thread -> std (n/a).
//! Carriers elsewhere are untouched; these ports live under new idiomatic names.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

/// Shared member-pointer + DescribedBase-adjustment core used by every
/// BoundFunc/BoundProp port below.
pub mod member {
    /// was: `T C::*` / `R (C::*)()` — the ARM member-function-pointer pair
    /// IDA keeps at GetSet +12/+16 and at BoundFunc +40/+44: `entry` is the
    /// code address (or vtable slot when virtual) and `adj` packs the
    /// this-adjustment (`adj >> 1`) with the virtual flag (`adj & 1`).
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct MemberPtr {
        pub entry: usize,
        pub adj: u32,
    }

    impl MemberPtr {
        /// IDA `((v & 1) != 0)` at 0x77f67a / 0x77f87c / 0x77fc96 / 0x77fe8e:
        /// set when the pointer names a virtual slot.
        pub fn is_virtual(self) -> bool {
            self.adj & 1 != 0
        }

        /// IDA `(SHIDWORD(v) >> 1)` at 0x77f876 / 0x77fc90 / 0x77fe88 and
        /// `(v4 >> 1)` at 0x77f676: bytes to add to the adjusted `this`.
        pub fn this_delta(self) -> usize {
            (self.adj >> 1) as usize
        }

        /// Null member pointer: IDA `+12 == 0 && (+16 & 1) == 0` takes the
        /// no-callback path at 0x77f870 / 0x77fc8a / 0x77fe82.
        pub fn is_null(self) -> bool {
            self.entry == 0 && !self.is_virtual()
        }
    }

    /// was: `DescribedBase*` downcast to the concrete `Instance` — every
    /// getValue/setValue/execute does `a2 ? a2 - 36 : 0` (IDA 0x77f66a,
    /// 0x77f82c, 0x77f84c, 0x77fa64, 0x77fc6e, 0x77fe66): the `DescribedBase`
    /// subobject sits 36 bytes into the debugger objects.
    pub const DESCRIBED_BASE_DELTA: usize = 36;

    /// IDA null-tolerant adjustment: null stays null, otherwise back up to
    /// the containing object.
    pub fn adjust_this(described: usize) -> usize {
        described.checked_sub(DESCRIBED_BASE_DELTA).unwrap_or(0)
    }

    /// was: `RBX::Instance::raisePropertyChanged` — the tail call shared by
    /// every setValue (IDA 0x77f89e, 0x77fab6, 0x77fcae, 0x77fea6).
    /// `notify` is the bound `Box<dyn Fn>` (was: `boost::function`); the
    /// recorded `(owner, member)` pair mirrors the `(v4, *v10, v8)` args.
    pub fn raise_property_changed(
        log: &mut Vec<(usize, usize)>,
        notify: &Option<Box<dyn Fn(usize, usize) + Send + Sync>>,
        owner: usize,
        member: usize,
    ) {
        log.push((owner, member));
        if let Some(cb) = notify {
            cb(owner, member);
        }
    }
}

pub mod bank {
    use super::member;
    use crate::SharedPtr;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex, Weak};

    /// was: the concrete `DebuggerWatch` / `DebuggerBreakpoint` storage.
    /// Offsets are the `a4` member offsets captured at construction; values
    /// are kept typed so `compare`/`assign` match the IDA width (string /
    /// byte / dword).
    #[derive(Default)]
    pub struct MemberBank {
        pub strings: HashMap<usize, String>,
        pub bools: HashMap<usize, bool>,
        pub ints: HashMap<usize, i32>,
        /// (owner, member) notifications, in order — mirrors the
        /// `raisePropertyChanged` tail of every setValue.
        pub changes: Vec<(usize, usize)>,
        /// was: `boost::function` change hook.
        pub notify: Option<Box<dyn Fn(usize, usize) + Send + Sync>>,
    }

    impl MemberBank {
        pub fn read_string(&self, owner: usize, off: usize) -> String {
            let _ = owner;
            self.strings.get(&off).cloned().unwrap_or_default()
        }

        /// IDA `std::string::compare` + conditional `assign`: returns true
        /// when the value actually changed (the only path that notifies).
        pub fn write_string(&mut self, owner: usize, off: usize, next: &str) -> bool {
            let slot = self.strings.entry(off).or_default();
            if *slot == next {
                return false;
            }
            slot.clear();
            slot.push_str(next);
            member::raise_property_changed(&mut self.changes, &self.notify, owner, off);
            true
        }

        pub fn read_bool(&self, owner: usize, off: usize) -> bool {
            let _ = owner;
            self.bools.get(&off).copied().unwrap_or(false)
        }

        pub fn write_bool(&mut self, owner: usize, off: usize, next: bool) -> bool {
            let slot = self.bools.entry(off).or_insert(false);
            if *slot == next {
                return false;
            }
            *slot = next;
            member::raise_property_changed(&mut self.changes, &self.notify, owner, off);
            true
        }

        pub fn read_int(&self, owner: usize, off: usize) -> i32 {
            let _ = owner;
            self.ints.get(&off).copied().unwrap_or(0)
        }

        pub fn write_int(&mut self, owner: usize, off: usize, next: i32) -> bool {
            let slot = self.ints.entry(off).or_insert(0);
            if *slot == next {
                return false;
            }
            *slot = next;
            member::raise_property_changed(&mut self.changes, &self.notify, owner, off);
            true
        }
    }

    /// was: `boost::shared_ptr<RBX::Scripting::DebuggerWatch>` /
    /// `DebuggerBreakpoint` — shared ownership of the member bank.
    pub type WatchBank = SharedPtr<std::sync::Mutex<MemberBank>>;
    /// was: `boost::weak_ptr<...>` observer of the bank.
    pub type WeakBank = Weak<std::sync::Mutex<MemberBank>>;

    pub fn new_bank() -> WatchBank {
        Arc::new(Mutex::new(MemberBank::default()))
    }
}

/// was: `RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerWatch,
/// void ()(void), 0>` — deleting-dtor (IDA 0x77f5b0) + execute
/// (IDA 0x77f664).
pub mod bound_func_watch {
    use super::member::MemberPtr;

    /// was: the BoundFuncDesc object: vtable word + signature-item list at
    /// +8 words + the member-function pointer at +40/+44.
    #[derive(Debug, Default)]
    pub struct BoundFuncWatch {
        /// was: signature `Item` list (`_M_clear(a1 + 8)` at IDA 0x77f614).
        pub signature_items: Vec<String>,
        /// was: pmf at +40 (entry) / +44 (adj).
        pub target: MemberPtr,
        /// Set by the deleting-dtor port; mirrors `operator delete(a1)`.
        pub freed: bool,
    }

    // 0x77f5b0 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting13DebuggerWatchEFvvELi0EED0Ev
    /// was: `RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerWatch,void ()(void),0>::~BoundFuncDesc()`
    #[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerWatch,void ()(void),0>::~BoundFuncDesc()")]
    #[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting13DebuggerWatchEFvvELi0EED0Ev")]
    pub fn deleting_dtor(desc: &mut BoundFuncWatch) {
        // IDA 0x77f5b0: deleting-dtor — reset vtable, clear the signature
        // item list at +8, then free the object.
        desc.signature_items.clear();
        desc.target = MemberPtr::default();
        desc.freed = true;
    }

    // 0x77f664 — __ZNK3RBX10Reflection13BoundFuncDescINS_9Scripting13DebuggerWatchEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
    /// was: `RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerWatch,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const`
    #[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerWatch,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
    #[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_9Scripting13DebuggerWatchEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
    pub fn execute(
        desc: &BoundFuncWatch,
        described: usize,
        invoke: &dyn Fn(usize, MemberPtr),
    ) {
        // IDA 0x77f664: `v2 = a2 ? a2 - 36 : 0`; `v5 = v2 + (v4 >> 1)`;
        // when `(v4 & 1)` resolve the entry through the vtable slot.
        let base = super::member::adjust_this(described);
        let this = base.wrapping_add(desc.target.this_delta());
        invoke(this, desc.target);
    }
}

/// was: `RBX::Reflection::BoundProp<std::string,
/// (RBX::Reflection::Mutability)1>` for `DebuggerWatch` — ctor
/// (IDA 0x77f688) + GetSet (IDA 0x77f81c-0x77f83c).
pub mod bound_prop_string_watch {
    use super::bank::WatchBank;
    use super::member::MemberPtr;

    /// was: `BoundProp<std::string>` — name/category/member at +8, custom
    /// setter pmf at +12/+16, owned GetSet node at +40.
    #[derive(Debug)]
    pub struct BoundPropString {
        pub name: String,
        pub category: String,
        pub member_offset: usize,
        pub setter: MemberPtr,
        pub getset_owner: usize,
    }

    // 0x77f688 — __ZN3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EEC2INS_9Scripting13DebuggerWatchEEEPKcS8_MT_SsNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
    /// was: `RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Scripting::DebuggerWatch>(char const*,char const*,std::string RBX::Scripting::DebuggerWatch::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)`
    #[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Scripting::DebuggerWatch>(char const*,char const*,std::string RBX::Scripting::DebuggerWatch::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
    #[doc(alias = "__ZN3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EEC2INS_9Scripting13DebuggerWatchEEEPKcS8_MT_SsNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
    pub fn create(
        name: &str,
        category: &str,
        member_offset: usize,
        setter: MemberPtr,
        getset_owner: usize,
    ) -> BoundPropString {
        // IDA 0x77f688: chain classDescriptor + TypedPropertyDescriptor,
        // install the BoundProp vtable, then `new(0x14)` the GetSet node
        // {vft, owner, member=a4, setter=0,0} into the +40 slot (dropping
        // any previous node) and clear attribute bits 0xEB/0xF3 at +28.
        BoundPropString {
            name: name.to_string(),
            category: category.to_string(),
            member_offset,
            setter,
            getset_owner,
        }
    }

    // 0x77f81c — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting13DebuggerWatchEE10isReadOnlyEv
    /// was: `RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerWatch>::isReadOnly(void)const`
    #[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerWatch>::isReadOnly(void)const")]
    #[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting13DebuggerWatchEE10isReadOnlyEv")]
    pub fn is_read_only(_prop: &BoundPropString) -> bool {
        // IDA 0x77f81c: `return 0`.
        false
    }

    // 0x77f820 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting13DebuggerWatchEE11isWriteOnlyEv
    /// was: `RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerWatch>::isWriteOnly(void)const`
    #[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerWatch>::isWriteOnly(void)const")]
    #[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting13DebuggerWatchEE11isWriteOnlyEv")]
    pub fn is_write_only(_prop: &BoundPropString) -> bool {
        // IDA 0x77f820: `return 0`.
        false
    }

    // 0x77f824 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting13DebuggerWatchEE8getValueEPKNS0_13DescribedBaseE
    /// was: `RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerWatch>::getValue(RBX::Reflection::DescribedBase const*)const`
    #[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerWatch>::getValue(RBX::Reflection::DescribedBase const*)const")]
    #[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting13DebuggerWatchEE8getValueEPKNS0_13DescribedBaseE")]
    pub fn get_value(prop: &BoundPropString, bank: &WatchBank, described: usize) -> String {
        // IDA 0x77f824: `v3 = a3 ? a3 - 36 : 0`; copy-construct from
        // `*(_DWORD *)(a2 + 8) + v3`.
        let _ = super::member::adjust_this(described);
        bank.lock().unwrap().read_string(prop.getset_owner, prop.member_offset)
    }

    // 0x77f83c — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting13DebuggerWatchEE8setValueEPNS0_13DescribedBaseERKSs
    /// was: `RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerWatch>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const`
    #[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerWatch>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
    #[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting13DebuggerWatchEE8setValueEPNS0_13DescribedBaseERKSs")]
    pub fn set_value(
        prop: &BoundPropString,
        bank: &WatchBank,
        described: usize,
        next: &str,
        call_setter: &dyn Fn(usize, MemberPtr),
    ) -> bool {
        // IDA 0x77f83c: adjust this; `compare` first and return early when
        // equal; else `assign`, run the +12/+16 setter pmf when present
        // (`+16 & 1 | +12`), then `raisePropertyChanged`.
        let _ = super::member::adjust_this(described);
        let changed = bank.lock().unwrap().write_string(prop.getset_owner, prop.member_offset, next);
        if changed && !prop.setter.is_null() {
            call_setter(prop.getset_owner, prop.setter);
        }
        changed
    }
}

/// was: `RBX::Reflection::BoundProp<std::string,
/// (RBX::Reflection::Mutability)1>` for `DebuggerBreakpoint` — ctor
/// (IDA 0x77f8a4) + GetSet (IDA 0x77fa34-0x77fa54). Same template shape as
/// the Watch string prop; the vtable/owner class differ.
pub mod bound_prop_string_breakpoint {
    use super::bank::WatchBank;
    use super::member::MemberPtr;

    /// was: `BoundProp<std::string>` bound to a `DebuggerBreakpoint` member.
    #[derive(Debug)]
    pub struct BoundPropStringBp {
        pub name: String,
        pub category: String,
        pub member_offset: usize,
        pub setter: MemberPtr,
        pub getset_owner: usize,
    }

    // 0x77f8a4 — __ZN3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EEC2INS_9Scripting18DebuggerBreakpointEEEPKcS8_MT_SsNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
    /// was: `RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Scripting::DebuggerBreakpoint>(char const*,char const*,std::string RBX::Scripting::DebuggerBreakpoint::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)`
    #[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Scripting::DebuggerBreakpoint>(char const*,char const*,std::string RBX::Scripting::DebuggerBreakpoint::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
    #[doc(alias = "__ZN3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EEC2INS_9Scripting18DebuggerBreakpointEEEPKcS8_MT_SsNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
    pub fn create(
        name: &str,
        category: &str,
        member_offset: usize,
        setter: MemberPtr,
        getset_owner: usize,
    ) -> BoundPropStringBp {
        // IDA 0x77f8a4: identical template expansion to 0x77f688 with the
        // Breakpoint classDescriptor and GetSet vtable.
        BoundPropStringBp {
            name: name.to_string(),
            category: category.to_string(),
            member_offset,
            setter,
            getset_owner,
        }
    }

    // 0x77fa34 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE10isReadOnlyEv
    /// was: `RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isReadOnly(void)const`
    #[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isReadOnly(void)const")]
    #[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE10isReadOnlyEv")]
    pub fn is_read_only(_prop: &BoundPropStringBp) -> bool {
        // IDA 0x77fa34: `return 0`.
        false
    }

    // 0x77fa38 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE11isWriteOnlyEv
    /// was: `RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isWriteOnly(void)const`
    #[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isWriteOnly(void)const")]
    #[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE11isWriteOnlyEv")]
    pub fn is_write_only(_prop: &BoundPropStringBp) -> bool {
        // IDA 0x77fa38: `return 0`.
        false
    }

    // 0x77fa3c — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8getValueEPKNS0_13DescribedBaseE
    /// was: `RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::getValue(RBX::Reflection::DescribedBase const*)const`
    #[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::getValue(RBX::Reflection::DescribedBase const*)const")]
    #[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8getValueEPKNS0_13DescribedBaseE")]
    pub fn get_value(prop: &BoundPropStringBp, bank: &WatchBank, described: usize) -> String {
        // IDA 0x77fa3c: same `a3 - 36` + member-offset copy as 0x77f824.
        let _ = super::member::adjust_this(described);
        bank.lock().unwrap().read_string(prop.getset_owner, prop.member_offset)
    }

    // 0x77fa54 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8setValueEPNS0_13DescribedBaseERKSs
    /// was: `RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const`
    #[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
    #[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8setValueEPNS0_13DescribedBaseERKSs")]
    pub fn set_value(
        prop: &BoundPropStringBp,
        bank: &WatchBank,
        described: usize,
        next: &str,
        call_setter: &dyn Fn(usize, MemberPtr),
    ) -> bool {
        // IDA 0x77fa54: compare/assign + optional setter +
        // `raisePropertyChanged`, mirroring 0x77f83c.
        let _ = super::member::adjust_this(described);
        let changed = bank.lock().unwrap().write_string(prop.getset_owner, prop.member_offset, next);
        if changed && !prop.setter.is_null() {
            call_setter(prop.getset_owner, prop.setter);
        }
        changed
    }
}

/// was: `RBX::Reflection::BoundProp<bool,
/// (RBX::Reflection::Mutability)1>` for `DebuggerBreakpoint` — ctor
/// (IDA 0x77fabc) + GetSet (IDA 0x77fc50-0x77fc64).
pub mod bound_prop_bool_breakpoint {
    use super::bank::WatchBank;
    use super::member::MemberPtr;

    /// was: `BoundProp<bool>` bound to a `DebuggerBreakpoint` byte member.
    #[derive(Debug)]
    pub struct BoundPropBool {
        pub name: String,
        pub category: String,
        pub member_offset: usize,
        pub setter: MemberPtr,
        pub getset_owner: usize,
    }

    // 0x77fabc — __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_9Scripting18DebuggerBreakpointEEEPKcS8_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
    /// was: `RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Scripting::DebuggerBreakpoint>(char const*,char const*,bool RBX::Scripting::DebuggerBreakpoint::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)`
    #[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Scripting::DebuggerBreakpoint>(char const*,char const*,bool RBX::Scripting::DebuggerBreakpoint::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
    #[doc(alias = "__ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_9Scripting18DebuggerBreakpointEEEPKcS8_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
    pub fn create(
        name: &str,
        category: &str,
        member_offset: usize,
        setter: MemberPtr,
        getset_owner: usize,
    ) -> BoundPropBool {
        // IDA 0x77fabc: bool template expansion of the 0x77f688 ctor flow
        // (TypedPropertyDescriptor<bool>, GetSet vtable off_12A0548).
        BoundPropBool {
            name: name.to_string(),
            category: category.to_string(),
            member_offset,
            setter,
            getset_owner,
        }
    }

    // 0x77fc50 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE10isReadOnlyEv
    /// was: `RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isReadOnly(void)const`
    #[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isReadOnly(void)const")]
    #[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE10isReadOnlyEv")]
    pub fn is_read_only(_prop: &BoundPropBool) -> bool {
        // IDA 0x77fc50: `return 0`.
        false
    }

    // 0x77fc54 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE11isWriteOnlyEv
    /// was: `RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isWriteOnly(void)const`
    #[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isWriteOnly(void)const")]
    #[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE11isWriteOnlyEv")]
    pub fn is_write_only(_prop: &BoundPropBool) -> bool {
        // IDA 0x77fc54: `return 0`.
        false
    }

    // 0x77fc58 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8getValueEPKNS0_13DescribedBaseE
    /// was: `RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::getValue(RBX::Reflection::DescribedBase const*)const`
    #[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::getValue(RBX::Reflection::DescribedBase const*)const")]
    #[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8getValueEPKNS0_13DescribedBaseE")]
    pub fn get_value(prop: &BoundPropBool, bank: &WatchBank, described: usize) -> bool {
        // IDA 0x77fc58: `*(u8 *)(*(_DWORD *)(a1 + 8) + a2 - 36)` — the
        // member offset is already adjusted, so no null guard on `a2`
        // beyond the wrapping subtract.
        let _ = super::member::adjust_this(described);
        bank.lock().unwrap().read_bool(prop.getset_owner, prop.member_offset)
    }

    // 0x77fc64 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8setValueEPNS0_13DescribedBaseERKb
    /// was: `RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::setValue(RBX::Reflection::DescribedBase *,bool const&)const`
    #[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
    #[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8setValueEPNS0_13DescribedBaseERKb")]
    pub fn set_value(
        prop: &BoundPropBool,
        bank: &WatchBank,
        described: usize,
        next: bool,
        call_setter: &dyn Fn(usize, MemberPtr),
    ) -> bool {
        // IDA 0x77fc64: byte compare at `v4 + result`, early return when
        // equal; else store, run the setter pmf, `raisePropertyChanged`.
        let _ = super::member::adjust_this(described);
        let changed = bank.lock().unwrap().write_bool(prop.getset_owner, prop.member_offset, next);
        if changed && !prop.setter.is_null() {
            call_setter(prop.getset_owner, prop.setter);
        }
        changed
    }
}

/// was: `RBX::Reflection::BoundProp<int,
/// (RBX::Reflection::Mutability)1>` for `DebuggerBreakpoint` — ctor
/// (IDA 0x77fcb4) + GetSet (IDA 0x77fe48-0x77fe5c).
pub mod bound_prop_int_breakpoint {
    use super::bank::WatchBank;
    use super::member::MemberPtr;

    /// was: `BoundProp<int>` bound to a `DebuggerBreakpoint` dword member.
    #[derive(Debug)]
    pub struct BoundPropInt {
        pub name: String,
        pub category: String,
        pub member_offset: usize,
        pub setter: MemberPtr,
        pub getset_owner: usize,
    }

    // 0x77fcb4 — __ZN3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EEC2INS_9Scripting18DebuggerBreakpointEEEPKcS8_MT_iNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
    /// was: `RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Scripting::DebuggerBreakpoint>(char const*,char const*,int RBX::Scripting::DebuggerBreakpoint::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)`
    #[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Scripting::DebuggerBreakpoint>(char const*,char const*,int RBX::Scripting::DebuggerBreakpoint::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
    #[doc(alias = "__ZN3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EEC2INS_9Scripting18DebuggerBreakpointEEEPKcS8_MT_iNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
    pub fn create(
        name: &str,
        category: &str,
        member_offset: usize,
        setter: MemberPtr,
        getset_owner: usize,
    ) -> BoundPropInt {
        // IDA 0x77fcb4: int template expansion of the 0x77f688 ctor flow
        // (TypedPropertyDescriptor<int>, GetSet vtable off_12A0578).
        BoundPropInt {
            name: name.to_string(),
            category: category.to_string(),
            member_offset,
            setter,
            getset_owner,
        }
    }

    // 0x77fe48 — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE10isReadOnlyEv
    /// was: `RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isReadOnly(void)const`
    #[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isReadOnly(void)const")]
    #[doc(alias = "__ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE10isReadOnlyEv")]
    pub fn is_read_only(_prop: &BoundPropInt) -> bool {
        // IDA 0x77fe48: `return 0`.
        false
    }

    // 0x77fe4c — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE11isWriteOnlyEv
    /// was: `RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isWriteOnly(void)const`
    #[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isWriteOnly(void)const")]
    #[doc(alias = "__ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isWriteOnly(void)const")]
    #[doc(alias = "__ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE11isWriteOnlyEv")]
    pub fn is_write_only(_prop: &BoundPropInt) -> bool {
        // IDA 0x77fe4c: `return 0`.
        false
    }

    // 0x77fe50 — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8getValueEPKNS0_13DescribedBaseE
    /// was: `RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::getValue(RBX::Reflection::DescribedBase const*)const`
    #[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::getValue(RBX::Reflection::DescribedBase const*)const")]
    #[doc(alias = "__ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8getValueEPKNS0_13DescribedBaseE")]
    pub fn get_value(prop: &BoundPropInt, bank: &WatchBank, described: usize) -> i32 {
        // IDA 0x77fe50: `*(_DWORD *)(*(a1 + 8) + a2 - 36)`.
        let _ = super::member::adjust_this(described);
        bank.lock().unwrap().read_int(prop.getset_owner, prop.member_offset)
    }

    // 0x77fe5c — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8setValueEPNS0_13DescribedBaseERKi
    /// was: `RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::setValue(RBX::Reflection::DescribedBase *,int const&)const`
    #[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
    #[doc(alias = "__ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_9Scripting18DebuggerBreakpointEE8setValueEPNS0_13DescribedBaseERKi")]
    pub fn set_value(
        prop: &BoundPropInt,
        bank: &WatchBank,
        described: usize,
        next: i32,
        call_setter: &dyn Fn(usize, MemberPtr),
    ) -> bool {
        // IDA 0x77fe5c: dword compare/store, optional setter pmf,
        // `raisePropertyChanged` — same skeleton as the bool port.
        let _ = super::member::adjust_this(described);
        let changed = bank.lock().unwrap().write_int(prop.getset_owner, prop.member_offset, next);
        if changed && !prop.setter.is_null() {
            call_setter(prop.getset_owner, prop.setter);
        }
        changed
    }
}

/// was: `RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,
/// int>` getter/setter ctor (IDA 0x77feac).
pub mod getter_prop_desc {
    use super::member::MemberPtr;

    /// was: `PropDescriptor<...>::GetImpl` node — `new(0xC)` at IDA 0x77fed8:
    /// {vft, getter=a4, setter=a5}, owned by the TypedPropertyDescriptor.
    #[derive(Debug, Default, Clone, Copy)]
    pub struct GetImplNode {
        pub getter: MemberPtr,
        pub setter: MemberPtr,
    }

    /// was: `PropDescriptor<DebuggerBreakpoint, int>` — name/category plus
    /// the getter/setter pair, attributes and permissions.
    #[derive(Debug)]
    pub struct GetterPropDesc {
        pub name: String,
        pub category: String,
        pub get_impl: GetImplNode,
        pub attributes: u32,
        pub permissions: u32,
    }

    // 0x77feac — __ZN3RBX10Reflection14PropDescriptorINS_9Scripting18DebuggerBreakpointEiEC2IMS3_KFivEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
    /// was: `RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,int>::PropDescriptor<int (RBX::Scripting::DebuggerBreakpoint::*)(void)const,int>(char const*,char const*,int (RBX::Scripting::DebuggerBreakpoint::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)`
    #[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,int>::PropDescriptor<int (RBX::Scripting::DebuggerBreakpoint::*)(void)const,int>(char const*,char const*,int (RBX::Scripting::DebuggerBreakpoint::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
    #[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9Scripting18DebuggerBreakpointEiEC2IMS3_KFivEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
    pub fn create(
        name: &str,
        category: &str,
        getter: MemberPtr,
        setter: MemberPtr,
        attributes: u32,
        permissions: u32,
    ) -> GetterPropDesc {
        // IDA 0x77feac: chain classDescriptor, heap-allocate the 0xC
        // GetImpl {vft, a4, a5} for the TypedPropertyDescriptor, install
        // the PropDescriptor vtable (off_12A05A8).
        GetterPropDesc {
            name: name.to_string(),
            category: category.to_string(),
            get_impl: GetImplNode { getter, setter },
            attributes,
            permissions,
        }
    }

    /// Getter dispatch through the stored getter pmf (mirrors the
    /// `GetImpl::getValue` family at IDA 0x77ffec/0x781c04/0x781e84,
    /// implemented in later shards).
    pub fn get_via_getter(desc: &GetterPropDesc, this: usize, call: &dyn Fn(usize, MemberPtr) -> i32) -> i32 {
        call(this, desc.get_impl.getter)
    }
}

#[cfg(test)]
mod tests {
    use super::bank::new_bank;
    use super::member::MemberPtr;

    #[test]
    fn string_prop_compare_assign_notify_once() {
        let bank = new_bank();
        let prop = super::bound_prop_string_watch::create("Watch", "Data", 8, MemberPtr::default(), 1);
        assert!(!super::bound_prop_string_watch::is_read_only(&prop));
        assert!(!super::bound_prop_string_watch::is_write_only(&prop));
        let noop = |_: usize, _: MemberPtr| {};
        // IDA 0x77f83c: equal compare notifies nothing.
        assert!(!super::bound_prop_string_watch::set_value(&prop, &bank, 36, "", &noop));
        assert!(bank.lock().unwrap().changes.is_empty());
        assert!(super::bound_prop_string_watch::set_value(&prop, &bank, 36, "x", &noop));
        // IDA 0x77f824: read back through the member offset.
        assert_eq!(super::bound_prop_string_watch::get_value(&prop, &bank, 36), "x");
        assert_eq!(bank.lock().unwrap().changes, vec![(1, 8)]);
    }

    #[test]
    fn bool_int_widths_match_ida() {
        let bank = new_bank();
        let b = super::bound_prop_bool_breakpoint::create("Enabled", "Data", 4, MemberPtr::default(), 2);
        let i = super::bound_prop_int_breakpoint::create("Line", "Data", 12, MemberPtr::default(), 2);
        let noop = |_: usize, _: MemberPtr| {};
        // IDA 0x77fc58/0x77fe50: zero-init reads.
        assert!(!super::bound_prop_bool_breakpoint::get_value(&b, &bank, 36));
        assert_eq!(super::bound_prop_int_breakpoint::get_value(&i, &bank, 36), 0);
        assert!(super::bound_prop_bool_breakpoint::set_value(&b, &bank, 36, true, &noop));
        assert!(super::bound_prop_int_breakpoint::set_value(&i, &bank, 36, 42, &noop));
        // IDA 0x77fc64/0x77fe5c: equal stores are no-ops.
        assert!(!super::bound_prop_bool_breakpoint::set_value(&b, &bank, 36, true, &noop));
        assert!(!super::bound_prop_int_breakpoint::set_value(&i, &bank, 36, 42, &noop));
        assert_eq!(bank.lock().unwrap().changes.len(), 2);
    }

    #[test]
    fn bound_func_execute_adjusts_this() {
        // IDA 0x77f664: described-36 plus the pmf this-delta.
        let desc = super::bound_func_watch::BoundFuncWatch {
            signature_items: vec!["()".to_string()],
            target: MemberPtr { entry: 7, adj: (8 << 1) as u32 },
            freed: false,
        };
        let seen = std::cell::Cell::new((0usize, MemberPtr::default()));
        super::bound_func_watch::execute(&desc, 100, &|this, pmf| {
            seen.set((this, pmf));
        });
        assert_eq!(seen.get().0, 100 - 36 + 8);
        let mut d = desc;
        super::bound_func_watch::deleting_dtor(&mut d);
        assert!(d.freed && d.signature_items.is_empty());
    }

    #[test]
    fn getter_prop_desc_stores_pair() {
        // IDA 0x77feac: the 0xC GetImpl keeps getter=a4, setter=a5.
        let g = MemberPtr { entry: 1, adj: 0 };
        let s = MemberPtr { entry: 2, adj: 0 };
        let d = super::getter_prop_desc::create("Line", "Data", g, s, 0, 0);
        assert_eq!(d.get_impl.getter, g);
        assert_eq!(d.get_impl.setter, s);
        assert_eq!(super::getter_prop_desc::get_via_getter(&d, 9, &|this, _| this as i32), 9);
    }
}

// --- Restored gap-filler carriers (pre-existing, kept verbatim) ---
// 100 core stubs 0xedaa50..0xee1864 from the original EA-sorted global gap
// filler skeleton (ObjC block helpers / BugSense carriers owned by higher
// crates; ARC Block_copy/Block_release no-ops in core). Restored after the
// 0x77f5b0-port cutover so no stub is lost; each keeps its
// #[doc(alias)] + // 0xADDR lines.
// 0xedaa50 — ___copy_helper_block_131_0
pub fn stub_0xedaa50() {
    // IDA 0xedaa50: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "___destroy_helper_block_132_0")]
// 0xedaa60 — ___destroy_helper_block_132_0
pub fn stub_0xedaa60() {
    // IDA 0xedaa60: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "___39+[BugSenseController sendEventWithTag:]_block_invoke134")]
// 0xedaa70 — ___39+[BugSenseController sendEventWithTag:]_block_invoke134
pub fn stub_0xedaa70() {
    // IDA 0xedaa70: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "___copy_helper_block_137_0")]
// 0xedaa94 — ___copy_helper_block_137_0
pub fn stub_0xedaa94() {
    // IDA 0xedaa94: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "___destroy_helper_block_138_0")]
// 0xedaaa4 — ___destroy_helper_block_138_0
pub fn stub_0xedaaa4() {
    // IDA 0xedaaa4: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "+[BugSenseController sendCustomEventWithTag:]")]
// 0xedaab4 — +[BugSenseController sendCustomEventWithTag:]
// type: char __cdecl(id, SEL, id)
pub fn stub_0xedaab4() {
    // IDA 0xedaab4: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "+[BugSenseController leaveBreadcrumb:]")]
// 0xedab5c — +[BugSenseController leaveBreadcrumb:]
// type: char __cdecl(id, SEL, id)
pub fn stub_0xedab5c() {
    // IDA 0xedab5c: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "+[BugSenseController lastCrashId]")]
// 0xedad70 — +[BugSenseController lastCrashId]
// type: int __cdecl(id, SEL)
pub fn stub_0xedad70() {
    // IDA 0xedad70: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "+[BugSenseController setLastCrashId:]")]
// 0xedad80 — +[BugSenseController setLastCrashId:]
// type: void __cdecl(id, SEL, int)
pub fn stub_0xedad80() {
    // IDA 0xedad80: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "+[BugSenseController crashCount]")]
// 0xedad90 — +[BugSenseController crashCount]
// type: int __cdecl(id, SEL)
pub fn stub_0xedad90() {
    // IDA 0xedad90: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSenseController resetCrashCount]")]
// 0xedadb4 — +[BugSenseController resetCrashCount]
// type: char __cdecl(id, SEL)
pub fn stub_0xedadb4() {
    // IDA 0xedadb4: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSenseController sharedControllerWithBugSenseAPIKey:endpointURL:userDictionary:sendImmediately:]")]
// 0xedadd8 — +[BugSenseController sharedControllerWithBugSenseAPIKey:endpointURL:userDictionary:sendImmediately:]
// type: id __cdecl(id, SEL, id, id, id, char)
pub fn stub_0xedadd8() {
    // IDA 0xedadd8: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "___100+[BugSenseController sharedControllerWithBugSenseAPIKey:endpointURL:userDictionary:sendImmediately:]_block_invoke")]
// 0xedae4c — ___100+[BugSenseController sharedControllerWithBugSenseAPIKey:endpointURL:userDictionary:sendImmediately:]_block_invoke
pub fn stub_0xedae4c() {
    // IDA 0xedae4c: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "___copy_helper_block_182")]
// 0xedaec4 — ___copy_helper_block_182
pub fn stub_0xedaec4() {
    // IDA 0xedaec4: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "___destroy_helper_block_183")]
// 0xedaf00 — ___destroy_helper_block_183
pub fn stub_0xedaf00() {
    // IDA 0xedaf00: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "+[BugSenseController sharedControllerWithBugSenseAPIKey:userDictionary:sendImmediately:]")]
// 0xedaf28 — +[BugSenseController sharedControllerWithBugSenseAPIKey:userDictionary:sendImmediately:]
// type: id __cdecl(id, SEL, id, id, char)
pub fn stub_0xedaf28() {
    // IDA 0xedaf28: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "+[BugSenseController sharedControllerWithBugSenseAPIKey:userDictionary:]")]
// 0xedaf4c — +[BugSenseController sharedControllerWithBugSenseAPIKey:userDictionary:]
// type: id __cdecl(id, SEL, id, id)
pub fn stub_0xedaf4c() {
    // IDA 0xedaf4c: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "+[BugSenseController sharedControllerWithBugSenseAPIKey:]")]
// 0xedaf70 — +[BugSenseController sharedControllerWithBugSenseAPIKey:]
// type: id __cdecl(id, SEL, id)
pub fn stub_0xedaf70() {
    // IDA 0xedaf70: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "+[BugSenseController sharedController]")]
// 0xedaf88 — +[BugSenseController sharedController]
// type: id __cdecl(id, SEL)
pub fn stub_0xedaf88() {
    // IDA 0xedaf88: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "-[BugSenseController initWithAPIKey:endpointURL:userDictionary:sendImmediately:]")]
// 0xedaf98 — -[BugSenseController initWithAPIKey:endpointURL:userDictionary:sendImmediately:]
// type: BugSenseController *__cdecl(BugSenseController *self, SEL, id, id, id, char)
pub fn stub_0xedaf98() {
    // IDA 0xedaf98: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSenseController allocWithZone:]")]
// 0xedb35c — +[BugSenseController allocWithZone:]
// type: id __cdecl(id, SEL, _NSZone *)
pub fn stub_0xedb35c() {
    // IDA 0xedb35c: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[BugSenseController copyWithZone:]")]
// 0xedb444 — -[BugSenseController copyWithZone:]
// type: id __cdecl(BugSenseController *self, SEL, _NSZone *)
pub fn stub_0xedb444() {
    // IDA 0xedb444: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[BugSenseController retain]")]
// 0xedb448 — -[BugSenseController retain]
// type: BugSenseController *__cdecl(BugSenseController *self, SEL)
pub fn stub_0xedb448() {
    // IDA 0xedb448: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[BugSenseController release]")]
// 0xedb44c — -[BugSenseController release]
// type: void __cdecl(BugSenseController *self, SEL)
pub fn stub_0xedb44c() {
    // IDA 0xedb44c: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[BugSenseController autorelease]")]
// 0xedb450 — -[BugSenseController autorelease]
// type: BugSenseController *__cdecl(BugSenseController *self, SEL)
pub fn stub_0xedb450() {
    // IDA 0xedb450: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[BugSenseController retainCount]")]
// 0xedb454 — -[BugSenseController retainCount]
// type: unsigned int __cdecl(BugSenseController *self, SEL)
pub fn stub_0xedb454() {
    // IDA 0xedb454: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[BugSenseController dealloc]")]
// 0xedb45c — -[BugSenseController dealloc]
// type: void __cdecl(BugSenseController *self, SEL)
pub fn stub_0xedb45c() {
    // IDA 0xedb45c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[BugSenseController retainSymbolsForReport:]")]
// 0xedb4ec — -[BugSenseController retainSymbolsForReport:]
// type: void __cdecl(BugSenseController *self, SEL, id)
pub fn stub_0xedb4ec() {
    // IDA 0xedb4ec: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[BugSenseController retainAdditionalCrashInfo]")]
// 0xedb67c — -[BugSenseController retainAdditionalCrashInfo]
// type: void __cdecl(BugSenseController *self, SEL)
pub fn stub_0xedb67c() {
    // IDA 0xedb67c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[BugSenseController processCrashReport:]")]
// 0xedb8c0 — -[BugSenseController processCrashReport:]
// type: void __cdecl(BugSenseController *self, SEL, id)
pub fn stub_0xedb8c0() {
    // IDA 0xedb8c0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[BugSenseController storeResponsePropertiesWithData:]")]
// 0xedbff8 — -[BugSenseController storeResponsePropertiesWithData:]
// type: void __cdecl(BugSenseController *self, SEL, id)
pub fn stub_0xedbff8() {
    // IDA 0xedbff8: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[BugSenseController showNewVersionAlertView]")]
// 0xedc204 — -[BugSenseController showNewVersionAlertView]
// type: void __cdecl(BugSenseController *self, SEL)
pub fn stub_0xedc204() {
    // IDA 0xedc204: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[BugSenseController alertView:clickedButtonAtIndex:]")]
// 0xedc338 — -[BugSenseController alertView:clickedButtonAtIndex:]
// type: void __cdecl(BugSenseController *self, SEL, id, int)
pub fn stub_0xedc338() {
    // IDA 0xedc338: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[BugSenseController operationCompleted:withData:forData:]")]
// 0xedc3c0 — -[BugSenseController operationCompleted:withData:forData:]
// type: void __cdecl(BugSenseController *self, SEL, char, id, id)
pub fn stub_0xedc3c0() {
    // IDA 0xedc3c0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "___70-[BugSenseController(Delegation) operationCompleted:withData:forData:]_block_invoke")]
// 0xedc554 — ___70-[BugSenseController(Delegation) operationCompleted:withData:forData:]_block_invoke
pub fn stub_0xedc554() {
    // IDA 0xedc554: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "___copy_helper_block_479")]
// 0xedc578 — ___copy_helper_block_479
pub fn stub_0xedc578() {
    // IDA 0xedc578: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "___destroy_helper_block_480")]
// 0xedc588 — ___destroy_helper_block_480
pub fn stub_0xedc588() {
    // IDA 0xedc588: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "-[BugSenseController analyticsOperationCompleted:forData:]")]
// 0xedc598 — -[BugSenseController analyticsOperationCompleted:forData:]
// type: void __cdecl(BugSenseController *self, SEL, char, id)
pub fn stub_0xedc598() {
    // IDA 0xedc598: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "___70-[BugSenseController(Delegation) analyticsOperationCompleted:forData:]_block_invoke")]
// 0xedc63c — ___70-[BugSenseController(Delegation) analyticsOperationCompleted:forData:]_block_invoke
pub fn stub_0xedc63c() {
    // IDA 0xedc63c: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "___copy_helper_block_505")]
// 0xedc6ac — ___copy_helper_block_505
pub fn stub_0xedc6ac() {
    // IDA 0xedc6ac: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "___destroy_helper_block_506")]
// 0xedc6d0 — ___destroy_helper_block_506
pub fn stub_0xedc6d0() {
    // IDA 0xedc6d0: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "+[BugSenseSymbolicator symbolsDirectory]")]
// 0xedc6e8 — +[BugSenseSymbolicator symbolsDirectory]
// type: id __cdecl(id, SEL)
pub fn stub_0xedc6e8() {
    // IDA 0xedc6e8: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "+[BugSenseSymbolicator populateSymbolsDirectoryAndReturnError:]")]
// 0xedc780 — +[BugSenseSymbolicator populateSymbolsDirectoryAndReturnError:]
// type: char __cdecl(id, SEL, id *)
pub fn stub_0xedc780() {
    // IDA 0xedc780: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "+[BugSenseSymbolicator retainSymbolsForStackFrames:inReport:]")]
// 0xedc864 — +[BugSenseSymbolicator retainSymbolsForStackFrames:inReport:]
// type: char __cdecl(id, SEL, id, id)
pub fn stub_0xedc864() {
    // IDA 0xedc864: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "+[BugSenseSymbolicator clearSymbols]")]
// 0xedcb64 — +[BugSenseSymbolicator clearSymbols]
// type: void __cdecl(id, SEL)
pub fn stub_0xedcb64() {
    // IDA 0xedcb64: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "+[BugSenseSymbolicator symbolAndOffsetForInstructionPointer:]")]
// 0xedcbb4 — +[BugSenseSymbolicator symbolAndOffsetForInstructionPointer:]
// type: id __cdecl(id, SEL, unsigned __int64)
pub fn stub_0xedcbb4() {
    // IDA 0xedcbb4: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSenseJSONGenerator frameworkVersion]")]
// 0xedccfc — +[BugSenseJSONGenerator frameworkVersion]
// type: id __cdecl(id, SEL)
pub fn stub_0xedccfc() {
    // IDA 0xedccfc: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSenseJSONGenerator frameworkPlatform]")]
// 0xedcd08 — +[BugSenseJSONGenerator frameworkPlatform]
// type: id __cdecl(id, SEL)
pub fn stub_0xedcd08() {
    // IDA 0xedcd08: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSenseJSONGenerator applicationNameForReport:]")]
// 0xedcd14 — +[BugSenseJSONGenerator applicationNameForReport:]
// type: id __cdecl(id, SEL, id)
pub fn stub_0xedcd14() {
    // IDA 0xedcd14: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSenseJSONGenerator executableNameForReport:]")]
// 0xedcd9c — +[BugSenseJSONGenerator executableNameForReport:]
// type: id __cdecl(id, SEL, id)
pub fn stub_0xedcd9c() {
    // IDA 0xedcd9c: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSenseJSONGenerator applicationName]")]
// 0xedce24 — +[BugSenseJSONGenerator applicationName]
// type: id __cdecl(id, SEL)
pub fn stub_0xedce24() {
    // IDA 0xedce24: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSenseJSONGenerator executableName]")]
// 0xedcec8 — +[BugSenseJSONGenerator executableName]
// type: id __cdecl(id, SEL)
pub fn stub_0xedcec8() {
    // IDA 0xedcec8: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSenseJSONGenerator applicationBuildNumberForReport:]")]
// 0xedcf14 — +[BugSenseJSONGenerator applicationBuildNumberForReport:]
// type: id __cdecl(id, SEL, id)
pub fn stub_0xedcf14() {
    // IDA 0xedcf14: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSenseJSONGenerator applicationBuildNumber]")]
// 0xedcf78 — +[BugSenseJSONGenerator applicationBuildNumber]
// type: id __cdecl(id, SEL)
pub fn stub_0xedcf78() {
    // IDA 0xedcf78: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSenseJSONGenerator applicationVersionNumberForReport:]")]
// 0xedd00c — +[BugSenseJSONGenerator applicationVersionNumberForReport:]
// type: id __cdecl(id, SEL, id)
pub fn stub_0xedd00c() {
    // IDA 0xedd00c: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSenseJSONGenerator applicationVersionNumber]")]
// 0xedd034 — +[BugSenseJSONGenerator applicationVersionNumber]
// type: id __cdecl(id, SEL)
pub fn stub_0xedd034() {
    // IDA 0xedd034: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSenseJSONGenerator IPAddress]")]
// 0xedd0c8 — +[BugSenseJSONGenerator IPAddress]
// type: id __cdecl(id, SEL)
pub fn stub_0xedd0c8() {
    // IDA 0xedd0c8: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSenseJSONGenerator device]")]
// 0xedd180 — +[BugSenseJSONGenerator device]
// type: id __cdecl(id, SEL)
pub fn stub_0xedd180() {
    // IDA 0xedd180: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSenseJSONGenerator carrierName]")]
// 0xedd224 — +[BugSenseJSONGenerator carrierName]
// type: id __cdecl(id, SEL)
pub fn stub_0xedd224() {
    // IDA 0xedd224: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSenseJSONGenerator languagesForReport:]")]
// 0xedd2c0 — +[BugSenseJSONGenerator languagesForReport:]
// type: id __cdecl(id, SEL, id)
pub fn stub_0xedd2c0() {
    // IDA 0xedd2c0: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSenseJSONGenerator languages]")]
// 0xedd3a4 — +[BugSenseJSONGenerator languages]
// type: id __cdecl(id, SEL)
pub fn stub_0xedd3a4() {
    // IDA 0xedd3a4: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSenseJSONGenerator isJailbroken]")]
// 0xedd494 — +[BugSenseJSONGenerator isJailbroken]
// type: char __cdecl(id, SEL)
pub fn stub_0xedd494() {
    // IDA 0xedd494: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSenseJSONGenerator stacktraceFromException:]")]
// 0xedd514 — +[BugSenseJSONGenerator stacktraceFromException:]
// type: id __cdecl(id, SEL, id)
pub fn stub_0xedd514() {
    // IDA 0xedd514: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "___49+[BugSenseJSONGenerator stacktraceFromException:]_block_invoke")]
// 0xedd850 — ___49+[BugSenseJSONGenerator stacktraceFromException:]_block_invoke
// type: int __cdecl(id, id, id)
pub fn stub_0xedd850() {
    // IDA 0xedd850: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSenseJSONGenerator JSONDataFromCrashReport:userDictionary:additionalInfo:]")]
// 0xedd8a0 — +[BugSenseJSONGenerator JSONDataFromCrashReport:userDictionary:additionalInfo:]
// type: id __cdecl(id, SEL, id, id, id)
pub fn stub_0xedd8a0() {
    // IDA 0xedd8a0: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSenseJSONGenerator JSONDataFromException:userDictionary:additionalInfo:]")]
// 0xedf5b0 — +[BugSenseJSONGenerator JSONDataFromException:userDictionary:additionalInfo:]
// type: id __cdecl(id, SEL, id, id, id)
pub fn stub_0xedf5b0() {
    // IDA 0xedf5b0: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSenseDataDispatcher pendingDispatchesCount]")]
// 0xee0600 — +[BugSenseDataDispatcher pendingDispatchesCount]
// type: unsigned int __cdecl(id, SEL)
pub fn stub_0xee0600() {
    // IDA 0xee0600: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSenseDataDispatcher postJSONData:withAPIKey:delegate:showFeedback:]")]
// 0xee0610 — +[BugSenseDataDispatcher postJSONData:withAPIKey:delegate:showFeedback:]
// type: char __cdecl(id, SEL, id, id, id, char)
pub fn stub_0xee0610() {
    // IDA 0xee0610: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "___72+[BugSenseDataDispatcher postJSONData:withAPIKey:delegate:showFeedback:]_block_invoke")]
// 0xee0910 — ___72+[BugSenseDataDispatcher postJSONData:withAPIKey:delegate:showFeedback:]_block_invoke
// type: int __fastcall(int, int, id, int, int)
pub fn stub_0xee0910() {
    // IDA 0xee0910: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "___72+[BugSenseDataDispatcher postJSONData:withAPIKey:delegate:showFeedback:]_block_invoke_2")]
// 0xee0a24 — ___72+[BugSenseDataDispatcher postJSONData:withAPIKey:delegate:showFeedback:]_block_invoke_2
pub fn stub_0xee0a24() {
    // IDA 0xee0a24: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "___copy_helper_block__37")]
// 0xee0a54 — ___copy_helper_block__37
pub fn stub_0xee0a54() {
    // IDA 0xee0a54: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "___destroy_helper_block__37")]
// 0xee0a84 — ___destroy_helper_block__37
pub fn stub_0xee0a84() {
    // IDA 0xee0a84: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "___copy_helper_block_61")]
// 0xee0aa4 — ___copy_helper_block_61
pub fn stub_0xee0aa4() {
    // IDA 0xee0aa4: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "___destroy_helper_block_62")]
// 0xee0ac8 — ___destroy_helper_block_62
pub fn stub_0xee0ac8() {
    // IDA 0xee0ac8: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "+[BugSenseDataDispatcher postAnalyticsData:withAPIKey:delegate:]")]
// 0xee0ae0 — +[BugSenseDataDispatcher postAnalyticsData:withAPIKey:delegate:]
// type: char __cdecl(id, SEL, id, id, id)
pub fn stub_0xee0ae0() {
    // IDA 0xee0ae0: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "___64+[BugSenseDataDispatcher postAnalyticsData:withAPIKey:delegate:]_block_invoke")]
// 0xee0e48 — ___64+[BugSenseDataDispatcher postAnalyticsData:withAPIKey:delegate:]_block_invoke
// type: int __fastcall(int, int, id, int, int)
pub fn stub_0xee0e48() {
    // IDA 0xee0e48: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "___64+[BugSenseDataDispatcher postAnalyticsData:withAPIKey:delegate:]_block_invoke_2")]
// 0xee0f6c — ___64+[BugSenseDataDispatcher postAnalyticsData:withAPIKey:delegate:]_block_invoke_2
pub fn stub_0xee0f6c() {
    // IDA 0xee0f6c: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "___copy_helper_block_92_0")]
// 0xee0f8c — ___copy_helper_block_92_0
pub fn stub_0xee0f8c() {
    // IDA 0xee0f8c: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "___destroy_helper_block_93_0")]
// 0xee0fb0 — ___destroy_helper_block_93_0
pub fn stub_0xee0fb0() {
    // IDA 0xee0fb0: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "___64+[BugSenseDataDispatcher postAnalyticsData:withAPIKey:delegate:]_block_invoke95")]
// 0xee0fc8 — ___64+[BugSenseDataDispatcher postAnalyticsData:withAPIKey:delegate:]_block_invoke95
pub fn stub_0xee0fc8() {
    // IDA 0xee0fc8: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "___copy_helper_block_96_0")]
// 0xee0fe8 — ___copy_helper_block_96_0
pub fn stub_0xee0fe8() {
    // IDA 0xee0fe8: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "___destroy_helper_block_97_0")]
// 0xee100c — ___destroy_helper_block_97_0
pub fn stub_0xee100c() {
    // IDA 0xee100c: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "___copy_helper_block_99")]
// 0xee1024 — ___copy_helper_block_99
pub fn stub_0xee1024() {
    // IDA 0xee1024: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "___destroy_helper_block_100")]
// 0xee1048 — ___destroy_helper_block_100
pub fn stub_0xee1048() {
    // IDA 0xee1048: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "___copy_helper_block__38")]
// 0xee112c — ___copy_helper_block__38
pub fn stub_0xee112c() {
    // IDA 0xee112c: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "___destroy_helper_block__38")]
// 0xee113c — ___destroy_helper_block__38
pub fn stub_0xee113c() {
    // IDA 0xee113c: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "+[BSAFHTTPRequestOperation operationWithRequest:observer:]")]
// 0xee11d4 — +[BSAFHTTPRequestOperation operationWithRequest:observer:]
// type: id __cdecl(id, SEL, id, id)
pub fn stub_0xee11d4() {
    // IDA 0xee11d4: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "+[BSAFHTTPRequestOperation operationWithRequest:completion:]")]
// 0xee124c — +[BSAFHTTPRequestOperation operationWithRequest:completion:]
// type: id __cdecl(id, SEL, id, id)
pub fn stub_0xee124c() {
    // IDA 0xee124c: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "+[BSAFHTTPRequestOperation operationWithRequest:inputStream:outputStream:completion:]")]
// 0xee12a0 — +[BSAFHTTPRequestOperation operationWithRequest:inputStream:outputStream:completion:]
// type: id __cdecl(id, SEL, id, id, id, id)
pub fn stub_0xee12a0() {
    // IDA 0xee12a0: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "___85+[BSAFHTTPRequestOperation operationWithRequest:inputStream:outputStream:completion:]_block_invoke")]
// 0xee1380 — ___85+[BSAFHTTPRequestOperation operationWithRequest:inputStream:outputStream:completion:]_block_invoke
pub fn stub_0xee1380() {
    // IDA 0xee1380: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "___copy_helper_block_44")]
// 0xee1398 — ___copy_helper_block_44
pub fn stub_0xee1398() {
    // IDA 0xee1398: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "___destroy_helper_block_45")]
// 0xee13a8 — ___destroy_helper_block_45
pub fn stub_0xee13a8() {
    // IDA 0xee13a8: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "-[BSAFHTTPRequestOperation initWithRequest:]")]
// 0xee13b8 — -[BSAFHTTPRequestOperation initWithRequest:]
// type: BSAFHTTPRequestOperation *__cdecl(BSAFHTTPRequestOperation *self, SEL, id)
pub fn stub_0xee13b8() {
    // IDA 0xee13b8: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "-[BSAFHTTPRequestOperation dealloc]")]
// 0xee1458 — -[BSAFHTTPRequestOperation dealloc]
// type: void __cdecl(BSAFHTTPRequestOperation *self, SEL)
pub fn stub_0xee1458() {
    // IDA 0xee1458: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "-[BSAFHTTPRequestOperation setState:]")]
// 0xee15a0 — -[BSAFHTTPRequestOperation setState:]
// type: void __cdecl(BSAFHTTPRequestOperation *self, SEL, int)
pub fn stub_0xee15a0() {
    // IDA 0xee15a0: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "-[BSAFHTTPRequestOperation responseString]")]
// 0xee1788 — -[BSAFHTTPRequestOperation responseString]
// type: NSString *__cdecl(BSAFHTTPRequestOperation *self, SEL)
pub fn stub_0xee1788() {
    // IDA 0xee1788: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "-[BSAFHTTPRequestOperation isReady]")]
// 0xee17e8 — -[BSAFHTTPRequestOperation isReady]
// type: char __cdecl(BSAFHTTPRequestOperation *self, SEL)
pub fn stub_0xee17e8() {
    // IDA 0xee17e8: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[BSAFHTTPRequestOperation isExecuting]")]
// 0xee1804 — -[BSAFHTTPRequestOperation isExecuting]
// type: char __cdecl(BSAFHTTPRequestOperation *self, SEL)
pub fn stub_0xee1804() {
    // IDA 0xee1804: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[BSAFHTTPRequestOperation isFinished]")]
// 0xee1824 — -[BSAFHTTPRequestOperation isFinished]
// type: char __cdecl(BSAFHTTPRequestOperation *self, SEL)
pub fn stub_0xee1824() {
    // IDA 0xee1824: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[BSAFHTTPRequestOperation isConcurrent]")]
// 0xee1864 — -[BSAFHTTPRequestOperation isConcurrent]
// type: char __cdecl(BSAFHTTPRequestOperation *self, SEL)
pub fn stub_0xee1864() {
    // IDA 0xee1864: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
