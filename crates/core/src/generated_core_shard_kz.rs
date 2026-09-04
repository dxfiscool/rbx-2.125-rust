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
