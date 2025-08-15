//! Utility functions for ID generation and common operations

use crate::storage::ID_COUNTER;

pub fn generate_id() -> u64 {
    ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter")
}