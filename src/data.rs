//! Data management functionality

use crate::error::Error;
use crate::models::Data;
use crate::storage::DATA_STORAGE;
use crate::utils::generate_id;

#[ic_cdk::update]
pub fn add_data(
    patient_username: String,
    doctor_username: String,
    data: Vec<u8>,
) -> Result<Data, Error> {
    let id = generate_id();

    let data = Data {
        id,
        patient_username,
        doctor_username,
        data,
    };

    DATA_STORAGE.with(|service| service.borrow_mut().insert(id, data.clone()));
    Ok(data)
}

#[ic_cdk::query]
pub fn get_data(id: u64) -> Result<Data, Error> {
    DATA_STORAGE.with(|service| {
        service
            .borrow()
            .get(&id)
            .map(|data| data.clone())
            .ok_or(Error::NotFound {
                msg: format!("Data with id={} not found", id),
            })
    })
}