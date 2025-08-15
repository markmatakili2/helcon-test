//! Data models for the medical appointment system

use candid::CandidType;
use serde::{Deserialize, Serialize};
use ic_stable_structures::{Storable, BoundedStorable};
use std::borrow::Cow;

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct Patient {
    pub id: u64,
    pub username: String,
    pub identity_id: u64,
}

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct Doctor {
    pub id: u64,
    pub principal_str: String,
    pub fname: String,
    pub lname: String,
    pub dob: String,
    pub specialism: String,
    pub licence_no: u64,
    pub id_no: u64,
    pub sex: String,
    pub country: String,
    pub city: String,
}

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct Appointment {
    pub id: u64,
    pub patient_id: u64,
    pub doctor_id: u64,
    pub phone_no: String,
    pub slot: String,
    pub reason: String,
    pub symtoms: String,
    pub status: String,
    pub appointment_type: String,
}

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct Message {
    pub id: u64,
    pub sender_id: u64,
    pub receiver_id: u64,
    pub content: String,
    pub multimedia_content: Option<MultiMediaContent>,
}

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct MultiMediaContent {
    pub content_type: String,
    pub data: Vec<u8>,
}

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct MedicalRecord {
    pub id: u64,
    pub patient_id: u64,
    pub lab_results: String,
    pub treatment_history: String,
}

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct Report {
    pub id: u64,
    pub patient_id: u64,
    pub username: String,
    pub symptoms: String,
    pub diagnostic: String,
    pub prescription: String,
    pub recommendations: String,
    pub multimedia_content: Option<MultiMediaContent>,
}

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct Identity {
    pub id: u64,
    pub principal: String,
}

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct DocIdentity {
    pub id: u64,
    pub principal: String,
}

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct Availability {
    pub id: u64,
    pub doctor_id: u64,
    pub day_of_week: u8,
    pub start_time: String,
    pub end_time: String,
    pub is_available: bool,
}

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct Calendly {
    pub id: u64,
    pub principle_id: String,
    pub calendly: String,
}

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct Data {
    pub id: u64,
    pub patient_username: String,
    pub doctor_username: String,
    pub data: Vec<u8>,
}

// Implement Storable and BoundedStorable for all types to work with stable structures
impl Storable for Patient {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }
}

impl BoundedStorable for Patient {
    const MAX_SIZE: u32 = 1024; // Adjust based on your needs
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Doctor {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }
}

impl BoundedStorable for Doctor {
    const MAX_SIZE: u32 = 2048; // Larger size for doctor data
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Appointment {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }
}

impl BoundedStorable for Appointment {
    const MAX_SIZE: u32 = 1536; // Adjust based on your needs
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Message {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }
}

impl BoundedStorable for Message {
    const MAX_SIZE: u32 = 4096; // Larger for multimedia content
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for MedicalRecord {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }
}

impl BoundedStorable for MedicalRecord {
    const MAX_SIZE: u32 = 2048; // Adjust based on your needs
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Report {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }
}

impl BoundedStorable for Report {
    const MAX_SIZE: u32 = 4096; // Larger for multimedia content
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Identity {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }
}

impl BoundedStorable for Identity {
    const MAX_SIZE: u32 = 512; // Smaller for identity data
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for DocIdentity {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }
}

impl BoundedStorable for DocIdentity {
    const MAX_SIZE: u32 = 512; // Smaller for identity data
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Availability {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }
}

impl BoundedStorable for Availability {
    const MAX_SIZE: u32 = 512; // Smaller for availability data
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Calendly {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }
}

impl BoundedStorable for Calendly {
    const MAX_SIZE: u32 = 1024; // Adjust based on your needs
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Data {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }
}

impl BoundedStorable for Data {
    const MAX_SIZE: u32 = 8192; // Larger for binary data
    const IS_FIXED_SIZE: bool = false;
}