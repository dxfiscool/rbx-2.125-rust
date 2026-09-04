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
    fn disconnect_is_self_remove() {
        let mut list = SlotList::new();
        let a = list.insert();
        list.remove(a);
        assert!(list.is_empty());
    }
}
