//! Medical Appointment System
//! Main library entry point that coordinates all modules

#[macro_use]
extern crate serde;

// Re-export all public APIs
pub use crate::appointment::*;
pub use crate::availability::*;
pub use crate::calendly::*;
pub use crate::data::*;
pub use crate::doctor::*;
pub use crate::identity::*;
pub use crate::medical_record::*;
pub use crate::message::*;
pub use crate::patient::*;
pub use crate::report::*;

// Internal modules
mod appointment;
mod availability;
mod calendly;
mod data;
mod doctor;
mod error;
mod identity;
mod medical_record;
mod message;
mod models;
mod patient;
mod report;
mod storage;
mod utils;

// Export Candid interface
ic_cdk::export_candid!();