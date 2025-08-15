//! Message management functionality

use crate::error::Error;
use crate::models::{Message, MultiMediaContent};
use crate::patient::get_patient_by_id;
use crate::storage::MESSAGE_STORAGE;
use crate::utils::generate_id;

#[ic_cdk::query]
pub fn get_message(message_id: u64) -> Result<Message, Error> {
    match get_message_by_id(&message_id) {
        Some(message) => Ok(message),
        None => Err(Error::NotFound {
            msg: format!("message with id={} not found", message_id),
        }),
    }
}

#[ic_cdk::update]
pub fn send_message(
    sender_id: u64,
    receiver_id: u64,
    content: String,
    multimedia_content: Option<MultiMediaContent>,
) -> Result<Message, Error> {
    // Validate input data
    if content.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Message content cannot be empty".to_string(),
        });
    }

    let id = generate_id();

    let message = Message {
        id,
        sender_id,
        receiver_id,
        content,
        multimedia_content,
    };

    MESSAGE_STORAGE.with(|service| service.borrow_mut().insert(id, message.clone()));
    Ok(message)
}

#[ic_cdk::update]
pub fn update_message(
    message_id: u64,
    sender_id: u64,
    receiver_id: u64,
    content: String,
    multimedia_content: Option<MultiMediaContent>,
) -> Result<Message, Error> {
    // Validate input data
    if content.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Message content cannot be empty".to_string(),
        });
    }

    let updated_message = Message {
        id: message_id,
        sender_id,
        receiver_id,
        content,
        multimedia_content,
    };

    match MESSAGE_STORAGE.with(|service| {
        service
            .borrow_mut()
            .insert(message_id, updated_message.clone())
    }) {
        Some(_) => Ok(updated_message),
        None => Err(Error::NotFound {
            msg: format!("Message with id={} not found", message_id),
        }),
    }
}

#[ic_cdk::update]
pub fn delete_message(message_id: u64) -> Result<(), Error> {
    match MESSAGE_STORAGE.with(|service| service.borrow_mut().remove(&message_id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Message with id={} not found", message_id),
        }),
    }
}

#[ic_cdk::query]
pub fn list_messages() -> Vec<Message> {
    MESSAGE_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .map(|(_, message)| message.clone())
            .collect()
    })
}

#[ic_cdk::update]
pub fn send_reminder_to_patient(
    patient_id: u64,
    content: String,
    multimedia_content: Option<MultiMediaContent>,
) -> Result<Message, Error> {
    // Validate input data
    if content.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Reminder content cannot be empty".to_string(),
        });
    }

    // Check if the patient exists
    if get_patient_by_id(&patient_id).is_none() {
        return Err(Error::NotFound {
            msg: format!("Patient with id={} not found", patient_id),
        });
    }

    let sender_id = 0; // System ID
    let id = generate_id();

    let message = Message {
        id,
        sender_id,
        receiver_id: patient_id,
        content,
        multimedia_content,
    };

    MESSAGE_STORAGE.with(|service| service.borrow_mut().insert(id, message.clone()));
    Ok(message)
}

pub fn get_message_by_id(message_id: &u64) -> Option<Message> {
    MESSAGE_STORAGE.with(|service| service.borrow().get(message_id))
}