//! Medical record management functionality

use crate::error::Error;
use crate::models::MedicalRecord;
use crate::storage::MEDICAL_RECORD_STORAGE;

#[ic_cdk::query]
pub fn get_medical_record(record_id: u64) -> Result<MedicalRecord, Error> {
    match get_medical_record_by_id(&record_id) {
        Some(record) => Ok(record),
        None => Err(Error::NotFound {
            msg: format!("medical record with id={} not found", record_id),
        }),
    }
}

#[ic_cdk::update]
pub fn create_medical_record(
    record_id: u64,
    patient_id: u64,
    lab_results: String,
    treatment_history: String,
) -> Result<MedicalRecord, Error> {
    // Input validation
    if lab_results.trim().is_empty() || treatment_history.trim().is_empty() {
        return Err(Error::InvalidInput {
            msg: "Lab results and treatment history cannot be empty".to_string(),
        });
    }

    let new_record = MedicalRecord {
        id: record_id,
        patient_id,
        lab_results,
        treatment_history,
    };

    match MEDICAL_RECORD_STORAGE
        .with(|service| service.borrow_mut().insert(record_id, new_record.clone()))
    {
        Some(_) => Err(Error::AlreadyExists {
            msg: format!("Medical record with id={} already exists", record_id),
        }),
        None => Ok(new_record),
    }
}

#[ic_cdk::update]
pub fn update_medical_record(
    record_id: u64,
    patient_id: u64,
    lab_results: String,
    treatment_history: String,
) -> Result<MedicalRecord, Error> {
    // Input validation
    if lab_results.trim().is_empty() || treatment_history.trim().is_empty() {
        return Err(Error::InvalidInput {
            msg: "Lab results and treatment history cannot be empty".to_string(),
        });
    }

    let updated_record = MedicalRecord {
        id: record_id,
        patient_id,
        lab_results,
        treatment_history,
    };

    match MEDICAL_RECORD_STORAGE.with(|service| {
        service
            .borrow_mut()
            .insert(record_id, updated_record.clone())
    }) {
        Some(_) => Ok(updated_record),
        None => Err(Error::NotFound {
            msg: format!("Medical record with id={} not found", record_id),
        }),
    }
}

#[ic_cdk::update]
pub fn delete_medical_record(record_id: u64) -> Result<(), Error> {
    // Input validation
    if record_id == 0 {
        return Err(Error::InvalidInput {
            msg: "Record ID cannot be zero".to_string(),
        });
    }

    match MEDICAL_RECORD_STORAGE.with(|service| service.borrow_mut().remove(&record_id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Medical record with id={} not found", record_id),
        }),
    }
}

#[ic_cdk::query]
pub fn list_medical_records() -> Vec<MedicalRecord> {
    MEDICAL_RECORD_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .map(|(_, record)| record.clone())
            .collect()
    })
}

pub fn get_medical_record_by_id(record_id: &u64) -> Option<MedicalRecord> {
    MEDICAL_RECORD_STORAGE.with(|service| service.borrow().get(record_id))
}