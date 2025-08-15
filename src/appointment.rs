//! Appointment management functionality

use crate::availability::update_availability_status;
use crate::doctor::get_doctor_by_id;
use crate::error::Error;
use crate::models::Appointment;
use crate::patient::get_patient_by_id;
use crate::storage::{APPOINTMENT_STORAGE, AVAILABILITY_STORAGE};
use crate::utils::generate_id;

#[ic_cdk::query]
pub fn get_appointment(appointment_id: u64) -> Result<Appointment, Error> {
    match get_appointment_by_id(&appointment_id) {
        Some(appointment) => Ok(appointment),
        None => Err(Error::NotFound {
            msg: format!("appointment with id={} not found", appointment_id),
        }),
    }
}

#[ic_cdk::update]
pub fn add_appointment(
    patient_id: u64,
    doctor_id: u64,
    phone_no: String,
    slot: String,
    reason: String,
    symtoms: String,
    status: String,
    appointment_type: String,
) -> Result<Appointment, Error> {
    // Validate input data
    if phone_no.is_empty() {
        return Err(Error::InvalidInput {
            msg: "phone_no cannot be empty".to_string(),
        });
    }

    // Check if the doctor and patient exist
    if get_doctor_by_id(&doctor_id).is_none() {
        return Err(Error::NotFound {
            msg: format!("Doctor with id={} not found", doctor_id),
        });
    }
    if get_patient_by_id(&patient_id).is_none() {
        return Err(Error::NotFound {
            msg: format!("Patient with id={} not found", patient_id),
        });
    }

    // Find the available slot for the doctor
    let available_slot = AVAILABILITY_STORAGE.with(|service| {
        service.borrow().iter().find(|(_, availability)| {
            availability.doctor_id == doctor_id
                && availability.is_available
                && availability.start_time == slot
        })
    });

    if available_slot.is_none() {
        return Err(Error::InvalidInput {
            msg: "Selected slot is not available".to_string(),
        });
    }

    // Mark the slot as unavailable
    let mut availability = available_slot.unwrap().1.clone();
    availability.is_available = false;
    AVAILABILITY_STORAGE.with(|service| {
        service
            .borrow_mut()
            .insert(availability.id, availability.clone())
    });

    let id = generate_id();

    let appointment = Appointment {
        id,
        patient_id,
        doctor_id,
        phone_no,
        slot,
        reason,
        symtoms,
        status: "pending".to_string(),
        appointment_type,
    };

    APPOINTMENT_STORAGE.with(|service| service.borrow_mut().insert(id, appointment.clone()));
    Ok(appointment)
}

#[ic_cdk::update]
pub fn update_appointment(
    appointment_id: u64,
    patient_id: u64,
    doctor_id: u64,
    phone_no: String,
    slot: String,
    reason: String,
    symtoms: String,
    status: String,
    appointment_type: String,
) -> Result<Appointment, Error> {
    // Validate input data
    if phone_no.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Phone number cannot be empty".to_string(),
        });
    }

    // Check if the appointment exists
    if get_appointment_by_id(&appointment_id).is_none() {
        return Err(Error::NotFound {
            msg: format!("Appointment with id={} not found", appointment_id),
        });
    }

    // If the appointment is canceled or completed, mark the slot as available
    if status == "cancelled" || status == "confirmed" {
        update_availability_status(doctor_id, &slot, true);
    }

    let updated_appointment = Appointment {
        id: appointment_id,
        patient_id,
        doctor_id,
        phone_no,
        slot,
        reason,
        symtoms,
        status,
        appointment_type,
    };

    // Update the appointment in storage
    match APPOINTMENT_STORAGE.with(|service| {
        service
            .borrow_mut()
            .insert(appointment_id, updated_appointment.clone())
    }) {
        Some(_) => Ok(updated_appointment),
        None => Err(Error::NotFound {
            msg: format!("Appointment with id={} not found", appointment_id),
        }),
    }
}

#[ic_cdk::update]
pub fn cancel_appointment(appointment_id: u64) -> Result<Appointment, Error> {
    let current_appointment = get_appointment_by_id(&appointment_id).ok_or(Error::NotFound {
        msg: format!("Appointment with id={} not found", appointment_id),
    })?;

    let mut updated_appointment = current_appointment.clone();
    updated_appointment.status = "cancelled".to_string();

    update_availability_status(
        current_appointment.doctor_id,
        &current_appointment.slot,
        true,
    );

    APPOINTMENT_STORAGE.with(|service| {
        service
            .borrow_mut()
            .insert(appointment_id, updated_appointment.clone())
    });

    Ok(updated_appointment)
}

#[ic_cdk::update]
pub fn complete_appointment(appointment_id: u64) -> Result<Appointment, Error> {
    let current_appointment = get_appointment_by_id(&appointment_id).ok_or(Error::NotFound {
        msg: format!("Appointment with id={} not found", appointment_id),
    })?;

    let mut updated_appointment = current_appointment.clone();
    updated_appointment.status = "confirmed".to_string();

    update_availability_status(
        current_appointment.doctor_id,
        &current_appointment.slot,
        true,
    );

    APPOINTMENT_STORAGE.with(|service| {
        service
            .borrow_mut()
            .insert(appointment_id, updated_appointment.clone())
    });

    Ok(updated_appointment)
}

#[ic_cdk::update]
pub fn delete_appointment(appointment_id: u64) -> Result<(), Error> {
    match APPOINTMENT_STORAGE.with(|service| service.borrow_mut().remove(&appointment_id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Appointment with id={} not found", appointment_id),
        }),
    }
}

#[ic_cdk::query]
pub fn list_appointments() -> Vec<Appointment> {
    APPOINTMENT_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .map(|(_, appointment)| appointment.clone())
            .collect()
    })
}

#[ic_cdk::query]
pub fn filter_appointments_by_doctor_id(doctor_id: u64) -> Vec<Appointment> {
    APPOINTMENT_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .filter(|(_, appointment)| appointment.doctor_id == doctor_id)
            .map(|(_, appointment)| appointment.clone())
            .collect()
    })
}

#[ic_cdk::query]
pub fn filter_appointments_by_patient_id(patient_id: u64) -> Vec<Appointment> {
    APPOINTMENT_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .filter(|(_, appointment)| appointment.patient_id == patient_id)
            .map(|(_, appointment)| appointment.clone())
            .collect()
    })
}

pub fn get_appointment_by_id(appointment_id: &u64) -> Option<Appointment> {
    APPOINTMENT_STORAGE.with(|service| service.borrow().get(appointment_id))
}