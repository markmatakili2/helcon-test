//! Identity management functionality

use crate::error::Error;
use crate::models::{DocIdentity, Identity};
use crate::storage::{DOCIDENTITY_STORAGE, IDENTITY_STORAGE};
use crate::utils::generate_id;

// Regular Identity functions
#[ic_cdk::update]
pub fn add_identity(principal: String) -> Result<Identity, Error> {
    // Validate input data
    if principal.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Principal cannot be empty".to_string(),
        });
    }

    // Check if the principal already exists
    let exists = IDENTITY_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .any(|(_, identity)| identity.principal == principal)
    });

    if exists {
        return Err(Error::AlreadyExists {
            msg: "Principal already exists".to_string(),
        });
    }

    let id = generate_id();

    let identity = Identity { id, principal };

    IDENTITY_STORAGE.with(|service| service.borrow_mut().insert(id, identity.clone()));
    Ok(identity)
}

#[ic_cdk::query]
pub fn get_identity(identity_id: u64) -> Result<Identity, Error> {
    match get_identity_by_id(&identity_id) {
        Some(identity) => Ok(identity),
        None => Err(Error::NotFound {
            msg: format!("Identity with id={} not found", identity_id),
        }),
    }
}

#[ic_cdk::query]
pub fn list_identities() -> Vec<Identity> {
    IDENTITY_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .map(|(_, identity)| identity.clone())
            .collect()
    })
}

#[ic_cdk::update]
pub fn delete_identity(identity_id: u64) -> Result<(), Error> {
    match IDENTITY_STORAGE.with(|service| service.borrow_mut().remove(&identity_id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Report with id={} not found", identity_id),
        }),
    }
}

#[ic_cdk::query]
pub fn does_identity_exist(input_principal: String) -> bool {
    let identities = list_identities();
    identities
        .into_iter()
        .any(|identity| identity.principal == input_principal)
}

// Doctor Identity functions
#[ic_cdk::update]
pub fn add_docidentity(principal: String) -> Result<DocIdentity, Error> {
    // Validate input data
    if principal.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Principal cannot be empty".to_string(),
        });
    }

    // Check if the principal already exists
    let exists = DOCIDENTITY_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .any(|(_, docidentity)| docidentity.principal == principal)
    });

    if exists {
        return Err(Error::AlreadyExists {
            msg: "Principal already exists".to_string(),
        });
    }

    let id = generate_id();

    let docidentity = DocIdentity { id, principal };

    DOCIDENTITY_STORAGE.with(|service| service.borrow_mut().insert(id, docidentity.clone()));
    Ok(docidentity)
}

#[ic_cdk::query]
pub fn get_docidentity(docidentity_id: u64) -> Result<DocIdentity, Error> {
    match DOCIDENTITY_STORAGE.with(|storage| storage.borrow().get(&docidentity_id)) {
        Some(docidentity) => Ok(docidentity.clone()),
        None => Err(Error::NotFound {
            msg: format!("DocIdentity with id={} not found", docidentity_id),
        }),
    }
}

#[ic_cdk::update]
pub fn delete_docidentity(docidentity_id: u64) -> Result<(), Error> {
    match DOCIDENTITY_STORAGE.with(|service| service.borrow_mut().remove(&docidentity_id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Report with id={} not found", docidentity_id),
        }),
    }
}

#[ic_cdk::query]
pub fn list_docidentities() -> Vec<DocIdentity> {
    DOCIDENTITY_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .map(|(_, docidentity)| docidentity.clone())
            .collect()
    })
}

#[ic_cdk::query]
pub fn does_docidentity_exist(principal: String) -> bool {
    let identities = list_docidentities();
    identities
        .into_iter()
        .any(|identity| identity.principal == principal)
}

// Helper functions
pub fn get_identity_by_id(identity_id: &u64) -> Option<Identity> {
    IDENTITY_STORAGE.with(|service| service.borrow().get(identity_id))
}

pub fn get_docidentity_by_id(docidentity_id: &u64) -> Option<DocIdentity> {
    DOCIDENTITY_STORAGE.with(|service| service.borrow().get(docidentity_id))
}