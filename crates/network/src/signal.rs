//! `rbx::signals::signal` slot list (maps to `rbx_core::Signal` for
//! emission; this covers the list surgery).
//!
//! Decompiled from `disconnectAll` (IDA 0xa5391c) and `remove` (IDA
//! 0xb21c58): slots form an intrusive singly-linked list (next at +8).
//! `disconnectAll` unlinks every node under the signal mutex;
//! `remove(slot)` asserts `!intrusive_ptr_expired(item)` (signal.h:261/284,
//! debug-only) and unlinks that one node (head or interior); a missing node
//! falls off the end as a no-op. `slot::disconnect()` is self-unlink, i.e.
//! `remove`. Slot payloads are generic: the functor itself stays
//! engine-side, this tracks linkage only.

#![allow(dead_code)]

/// One linked slot: its identity stands in for the `slot *`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SlotId(u64);

/// `rbx::signals::signal<...>` linkage reduced to an ordered slot set.
#[derive(Clone, Debug, Default)]
pub struct SlotList {
    slots: Vec<SlotId>,
    next: u64,
}

impl SlotList {
    pub fn new() -> Self {
        Self::default()
    }

    /// `signal::insert(slot *)`: link the node. The liveness of the
    /// intrusive pointer is engine-side; the id stands in for it.
    pub fn insert(&mut self) -> SlotId {
        let id = SlotId(self.next);
        self.next += 1;
        self.slots.push(id);
        id
    }

    /// `signal::remove(slot *)` (IDA 0xb21c58) and `slot::disconnect()`:
    /// unlink the node; a missing node is a no-op (the original walks off
    /// the end and only runs the trailing assert).
    pub fn remove(&mut self, id: SlotId) {
        if let Some(pos) = self.slots.iter().position(|s| *s == id) {
            self.slots.remove(pos);
        }
    }

    /// `signal::disconnectAll` (IDA 0xa5391c): unlink every node.
    pub fn disconnect_all(&mut self) {
        self.slots.clear();
    }


    pub fn len(&self) -> usize {
        self.slots.len()
    }

    pub fn is_empty(&self) -> bool {
        self.slots.is_empty()
    }

    pub fn contains(&self, id: SlotId) -> bool {
        self.slots.contains(&id)
    }
}
/// `EventDescBase::connect` (IDA 0x971db4): wraps the functor in a slot
/// node, assigns it, inserts it into the event signal, and returns the
/// connection — null when sourceless. Node malloc, vtables, and weak-ref
/// traffic stay engine-side; the functor arrives pre-composed.
pub fn connect<T>(
    list: &mut SlotList,
    slot: &mut Option<Box<T>>,
    functor: T,
    source_present: bool,
) -> Option<SlotId> {
    if !source_present {
        // IDA 0x971db4: `*out = 0`.
        return None;
    }
    super::functor::assign_to(slot, functor);
    // IDA 0x971db4: `signal::insert(...)`, out = slot, weak-ref bump.
    Some(list.insert())
}

/// `signal_with_args<N>::operator()(args)` (IDA 0x9e39e0): when linked,
/// log under `SignalPrints`, then walk the slots via `next`, invoking each
/// functor with `args` (refcount traffic engine-side). The per-slot
/// functors stay engine-side; this drives the caller's composed emission
/// once per linked slot.
pub fn emit_each(list: &SlotList, mut fire_one: impl FnMut()) {
    for _ in 0..list.len() {
        fire_one();
    }
}

/// `signal::next(cursor)` (IDA 0x9e39e0): advances the emission cursor to
/// the following linked slot, `false` at the end (empty cursor starts over
/// at the head).
pub fn next_slot(list: &SlotList, cursor: &mut Option<SlotId>) -> bool {
    let next = match *cursor {
        None => list.slots.first().copied(),
        Some(id) => list
            .slots
            .iter()
            .position(|s| *s == id)
            .and_then(|pos| list.slots.get(pos + 1).copied()),
    };
    *cursor = next;
    next.is_some()
}

/// `boost::intrusive_ptr<slot>::operator=` — stores the pointer; the
/// refcount release/acquire pair stays engine-side.
pub fn slot_assign(target: &mut Option<SlotId>, other: Option<SlotId>) {
    *target = other;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_remove_disconnect_all() {
        // IDA 0xb21c58 (unlink one) / 0xa5391c (unlink all).
        let mut list = SlotList::new();
        let a = list.insert();
        let b = list.insert();
        assert_eq!(list.len(), 2);
        list.remove(a);
        assert!(!list.contains(a));
        assert!(list.contains(b));
        // Missing node: no-op, like walking off the list end.
        list.remove(a);
        assert_eq!(list.len(), 1);
        list.disconnect_all();
        assert!(list.is_empty());
    }

    #[test]
    fn connect_assigns_and_returns_handle() {
        // IDA 0x971db4: sourceless connect yields null; otherwise the slot
        // installs and links.
        let mut list = SlotList::new();
        let mut slot: Option<Box<fn()>> = None;
        fn f() {}
        assert_eq!(connect(&mut list, &mut slot, f as fn(), false), None);
        assert!(list.is_empty());
        let id = connect(&mut list, &mut slot, f as fn(), true).expect("handle");
        assert!(list.contains(id));
    }

    #[test]
    fn disconnect_is_self_remove() {
        let mut list = SlotList::new();
        let a = list.insert();
        list.remove(a);
        assert!(list.is_empty());
    }

    #[test]
    fn emit_walks_each_slot() {
        // IDA 0x9e39e0: empty signal emits nothing; otherwise once per slot.
        let mut list = SlotList::new();
        let mut calls = 0;
        emit_each(&list, || calls += 1);
        assert_eq!(calls, 0);
        list.insert();
        list.insert();
        list.insert();
        emit_each(&list, || calls += 1);
        assert_eq!(calls, 3);
    }

    #[test]
    fn next_cursor_walks_and_ends() {
        let mut list = SlotList::new();
        let a = list.insert();
        let b = list.insert();
        let mut cursor = None;
        assert!(next_slot(&list, &mut cursor));
        assert_eq!(cursor, Some(a));
        assert!(next_slot(&list, &mut cursor));
        assert_eq!(cursor, Some(b));
        assert!(!next_slot(&list, &mut cursor));
        assert_eq!(cursor, None);
    }

    #[test]
    fn slot_assign_stores() {
        let mut target = None;
        let mut list = SlotList::new();
        let id = list.insert();
        slot_assign(&mut target, Some(id));
        assert_eq!(target, Some(id));
        slot_assign(&mut target, None);
        assert_eq!(target, None);
    }
}
