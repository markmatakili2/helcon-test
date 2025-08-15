//! Report management functionality

use crate::error::Error;
use crate::models::{MultiMediaContent, Report};
use crate::patient::get_patient_by_id;
use crate::storage::REPORT_STORAGE;
use crate::utils::generate_id;

#[ic_cdk::update]
pub fn add_report(
    patient_id: u64,
    username: String,
    symptoms: String,
    diagnostic: String,
    prescription: String,
    recommendations: String,
    multimedia_content: Option<MultiMediaContent>,
) -> Result<Report, Error> {
    // Validate input data
    if username.is_empty()
        || symptoms.is_empty()
        || diagnostic.is_empty()
        || prescription.is_empty()
        || recommendations.is_empty()
    {
        return Err(Error::InvalidInput {
            msg: "All fields must be provided".to_string(),
        });
    }

    // Check if the patient exists
    if get_patient_by_id(&patient_id).is_none() {
        return Err(Error::NotFound {
            msg: format!("Patient with id={} not found", patient_id),
        });
    }

    let id = generate_id();

    let report = Report {
        id,
        patient_id,
        username,
        symptoms,
        diagnostic,
        prescription,
        recommendations,
        multimedia_content,
    };

    REPORT_STORAGE.with(|service| service.borrow_mut().insert(id, report.clone()));
    Ok(report)
}

#[ic_cdk::query]
pub fn get_report(report_id: u64) -> Result<Report, Error> {
    match get_report_by_id(&report_id) {
        Some(report) => Ok(report),
        None => Err(Error::NotFound {
            msg: format!("Report with id={} not found", report_id),
        }),
    }
}

#[ic_cdk::update]
pub fn update_report(
    report_id: u64,
    patient_id: u64,
    username: String,
    symptoms: String,
    diagnostic: String,
    prescription: String,
    recommendations: String,
    multimedia_content: Option<MultiMediaContent>,
) -> Result<Report, Error> {
    // Validate input data
    if username.is_empty()
        || symptoms.is_empty()
        || diagnostic.is_empty()
        || prescription.is_empty()
        || recommendations.is_empty()
    {
        return Err(Error::InvalidInput {
            msg: "All fields must be provided".to_string(),
        });
    }

    let updated_report = Report {
        id: report_id,
        patient_id,
        username,
        symptoms,
        diagnostic,
        prescription,
        recommendations,
        multimedia_content,
    };

    match REPORT_STORAGE.with(|service| {
        service
            .borrow_mut()
            .insert(report_id, updated_report.clone())
    }) {
        Some(_) => Ok(updated_report),
        None => Err(Error::NotFound {
            msg: format!("Report with id={} not found", report_id),
        }),
    }
}

#[ic_cdk::update]
pub fn delete_report(report_id: u64) -> Result<(), Error> {
    match REPORT_STORAGE.with(|service| service.borrow_mut().remove(&report_id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Report with id={} not found", report_id),
        }),
    }
}

#[ic_cdk::query]
pub fn list_reports() -> Vec<Report> {
    REPORT_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .map(|(_, report)| report.clone())
            .collect()
    })
}

pub fn get_report_by_id(report_id: &u64) -> Option<Report> {
    REPORT_STORAGE.with(|service| service.borrow().get(report_id))
}