//! Calendly integration functionality

use crate::error::Error;
use crate::models::Calendly;
use crate::storage::CALENDLY_STORAGE;
use crate::utils::generate_id;

#[ic_cdk::update]
pub fn add_calendly(principle_id: String, calendly: String) -> Result<Calendly, Error> {
    let id = generate_id();

    let calendly = Calendly {
        id,
        principle_id,
        calendly,
    };

    CALENDLY_STORAGE.with(|service| service.borrow_mut().insert(id, calendly.clone()));
    Ok(calendly)
}

#[ic_cdk::query]
pub fn get_calendly(id: u64) -> Result<Calendly, Error> {
    CALENDLY_STORAGE.with(|service| {
        service
            .borrow()
            .get(&id)
            .map(|calendly| calendly.clone())
            .ok_or(Error::NotFound {
                msg: format!("Calendly with id={} not found", id),
            })
    })
}

#[ic_cdk::update]
pub fn delete_calendly(id: u64) -> Result<(), Error> {
    CALENDLY_STORAGE.with(|service| {
        let mut storage = service.borrow_mut();
        if storage.remove(&id).is_some() {
            Ok(())
        } else {
            Err(Error::NotFound {
                msg: format!("Calendly with id={} not found", id),
            })
        }
    })
}