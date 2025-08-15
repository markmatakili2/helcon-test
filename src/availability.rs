//! Availability management functionality

use crate::doctor::get_doctor_by_id;
use crate::error::Error;
use crate::models::Availability;
use crate::storage::AVAILABILITY_STORAGE;
use crate::utils::generate_id;

#[ic_cdk::update]
pub fn add_availability(
    doctor_id: u64,
    day_of_week: u8,
    start_time: String,
    end_time: String,
    is_available: bool,
) -> Result<Availability, Error> {
    // Validate input data
    if day_of_week > 6 {
        return Err(Error::InvalidInput {
            msg: "Invalid day of the week".to_string(),
        });
    }
    if start_time.is_empty() || end_time.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Start time or end time cannot be empty".to_string(),
        });
    }

    // Check if the doctor exists
    if get_doctor_by_id(&doctor_id).is_none() {
        return Err(Error::NotFound {
            msg: format!("Doctor with id={} not found", doctor_id),
        });
    }

    let id = generate_id();

    let availability = Availability {
        id,
        doctor_id,
        day_of_week,
        start_time,
        end_time,
        is_available,
    };

    AVAILABILITY_STORAGE.with(|service| service.borrow_mut().insert(id, availability.clone()));
    Ok(availability)
}

#[ic_cdk::query]
pub fn get_availability(availability_id: u64) -> Result<Availability, Error> {
    match get_availability_by_id(&availability_id) {
        Some(availability) => Ok(availability),
        None => Err(Error::NotFound {
            msg: format!("Availability with id={} not found", availability_id),
        }),
    }
}

#[ic_cdk::update]
pub fn update_availability(
    availability_id: u64,
    doctor_id: u64,
    day_of_week: u8,
    start_time: String,
    end_time: String,
    is_available: bool,
) -> Result<Availability, Error> {
    // Validate input data
    if day_of_week > 6 {
        return Err(Error::InvalidInput {
            msg: "Invalid day of the week".to_string(),
        });
    }
    if start_time.is_empty() || end_time.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Start time or end time cannot be empty".to_string(),
        });
    }

    let updated_availability = Availability {
        id: availability_id,
        doctor_id,
        day_of_week,
        start_time,
        end_time,
        is_available,
    };

    match AVAILABILITY_STORAGE.with(|service| {
        service
            .borrow_mut()
            .insert(availability_id, updated_availability.clone())
    }) {
        Some(_) => Ok(updated_availability),
        None => Err(Error::NotFound {
            msg: format!("Availability with id={} not found", availability_id),
        }),
    }
}

#[ic_cdk::update]
pub fn delete_availability(availability_id: u64) -> Result<(), Error> {
    match AVAILABILITY_STORAGE.with(|service| service.borrow_mut().remove(&availability_id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Availability with id={} not found", availability_id),
        }),
    }
}

#[ic_cdk::query]
pub fn list_availabilities() -> Vec<Availability> {
    AVAILABILITY_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .map(|(_, availability)| availability.clone())
            .collect()
    })
}

#[ic_cdk::query]
pub fn filter_available_slots_by_doctor_id(doctor_id: u64) -> Vec<Availability> {
    AVAILABILITY_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .filter(|(_, availability)| {
                availability.doctor_id == doctor_id && availability.is_available
            })
            .map(|(_, availability)| availability.clone())
            .collect()
    })
}

#[ic_cdk::query]
pub fn filter_availability_by_doctor_id(doctor_id: u64) -> Vec<Availability> {
    AVAILABILITY_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .filter(|(_, availability)| availability.doctor_id == doctor_id)
            .map(|(_, availability)| availability.clone())
            .collect()
    })
}

pub fn get_availability_by_id(availability_id: &u64) -> Option<Availability> {
    AVAILABILITY_STORAGE.with(|service| service.borrow().get(availability_id))
}

pub fn update_availability_status(doctor_id: u64, slot: &str, is_available: bool) {
    AVAILABILITY_STORAGE.with(|service| {
        let mut storage = service.borrow_mut();
        if let Some((id, mut availability)) = storage
            .iter()
            .find(|(_, availability)| {
                availability.doctor_id == doctor_id && availability.start_time == slot
            })
            .map(|(id, availability)| (id, availability.clone()))
        {
            availability.is_available = is_available;
            storage.insert(id, availability);
        }
    });
}