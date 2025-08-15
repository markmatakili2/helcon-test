//! Doctor management functionality

use crate::error::Error;
use crate::models::Doctor;
use crate::storage::DOCTOR_STORAGE;
use crate::utils::generate_id;

#[ic_cdk::update]
pub fn add_doctor(
    principal_str: String,
    fname: String,
    lname: String,
    dob: String,
    specialism: String,
    licence_no: u64,
    id_no: u64,
    sex: String,
    country: String,
    city: String,
) -> Result<Doctor, Error> {
    // Validate input data
    if principal_str.is_empty()
        || fname.is_empty()
        || lname.is_empty()
        || dob.is_empty()
        || specialism.is_empty()
        || sex.is_empty()
        || country.is_empty()
        || city.is_empty()
    {
        return Err(Error::InvalidInput {
            msg: "All fields must be filled".to_string(),
        });
    }

    // Check if the principal already exists
    let exists = DOCTOR_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .any(|(_, doctor)| doctor.principal_str == principal_str)
    });

    if exists {
        return Err(Error::AlreadyExists {
            msg: "Principal already exists".to_string(),
        });
    }

    let id = generate_id();

    let doctor = Doctor {
        id,
        principal_str,
        fname,
        lname,
        dob,
        specialism,
        licence_no,
        id_no,
        sex,
        country,
        city,
    };

    DOCTOR_STORAGE.with(|service| service.borrow_mut().insert(id, doctor.clone()));
    Ok(doctor)
}

#[ic_cdk::update]
pub fn update_doctor(
    principal_str: String,
    fname: String,
    lname: String,
    dob: String,
    specialism: String,
    licence_no: u64,
    id_no: u64,
    sex: String,
    country: String,
    city: String,
) -> Result<Doctor, Error> {
    // Validate input data
    if fname.is_empty()
        || lname.is_empty()
        || dob.is_empty()
        || specialism.is_empty()
        || sex.is_empty()
        || country.is_empty()
        || city.is_empty()
    {
        return Err(Error::InvalidInput {
            msg: "All fields must be provided".to_string(),
        });
    }

    let identity_id = principal_str
        .parse::<u64>()
        .map_err(|_| Error::InvalidInput {
            msg: "Invalid Identity ID format".to_string(),
        })?;

    let doctor_exists = DOCTOR_STORAGE.with(|service| service.borrow().contains_key(&identity_id));

    if !doctor_exists {
        return Err(Error::NotFound {
            msg: "Doctor with this Identity ID does not exist".to_string(),
        });
    }

    let updated_doctor = Doctor {
        id: identity_id,
        principal_str: identity_id.to_string(),
        fname,
        lname,
        dob,
        specialism,
        licence_no,
        id_no,
        sex,
        country,
        city,
    };

    DOCTOR_STORAGE.with(|service| {
        service
            .borrow_mut()
            .insert(identity_id, updated_doctor.clone())
    });

    Ok(updated_doctor)
}

#[ic_cdk::query]
pub fn get_doctor(docidentity_id: u64) -> Result<Doctor, Error> {
    match get_doctor_by_id(&docidentity_id) {
        Some(doctor) => Ok(doctor),
        None => Err(Error::NotFound {
            msg: format!("Doctor with docidentity_id={} not found", docidentity_id),
        }),
    }
}

#[ic_cdk::update]
pub fn delete_doctor(doctor_id: u64) -> Result<(), Error> {
    match DOCTOR_STORAGE.with(|service| service.borrow_mut().remove(&doctor_id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Doctor with id={} not found", doctor_id),
        }),
    }
}

#[ic_cdk::query]
pub fn list_doctors() -> Vec<Doctor> {
    DOCTOR_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .map(|(_, doctor)| doctor.clone())
            .collect()
    })
}

pub fn get_doctor_by_id(docidentity_id: &u64) -> Option<Doctor> {
    DOCTOR_STORAGE.with(|service| service.borrow().get(docidentity_id))
}