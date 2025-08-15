//! Patient management functionality

use crate::error::Error;
use crate::identity::get_identity_by_id;
use crate::models::Patient;
use crate::storage::{IDENTITY_STORAGE, PATIENT_STORAGE};
use crate::utils::generate_id;

#[ic_cdk::query]
pub fn get_patient(patient_id: u64) -> Result<Patient, Error> {
    match get_patient_by_id(&patient_id) {
        Some(patient) => Ok(patient),
        None => Err(Error::NotFound {
            msg: format!("patient with id={} not found", patient_id),
        }),
    }
}

#[ic_cdk::update]
pub fn register_patient(username: String, identity_id: u64) -> Result<Patient, Error> {
    // Validate input data
    if username.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Name cannot be empty".to_string(),
        });
    }

    // Check if the username already exists
    let username_exists = PATIENT_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .any(|(_, patient)| patient.username == username)
    });

    if username_exists {
        return Err(Error::AlreadyExists {
            msg: "Username already exists".to_string(),
        });
    }

    // Check if the identity_id exists
    let identity_exists =
        IDENTITY_STORAGE.with(|service| service.borrow().contains_key(&identity_id));

    if !identity_exists {
        return Err(Error::NotFound {
            msg: "Identity ID does not exist".to_string(),
        });
    }

    let id = generate_id();

    let patient = Patient {
        id,
        username,
        identity_id,
    };

    PATIENT_STORAGE.with(|service| service.borrow_mut().insert(id, patient.clone()));
    Ok(patient)
}

#[ic_cdk::update]
pub fn delete_patient(patient_id: u64) -> Result<(), Error> {
    match PATIENT_STORAGE.with(|service| service.borrow_mut().remove(&patient_id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Patient with id={} not found", patient_id),
        }),
    }
}

#[ic_cdk::query]
pub fn list_patients() -> Vec<Patient> {
    PATIENT_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .map(|(_, patient)| patient.clone())
            .collect()
    })
}

pub fn get_patient_by_id(patient_id: &u64) -> Option<Patient> {
    PATIENT_STORAGE.with(|service| service.borrow().get(patient_id))
}