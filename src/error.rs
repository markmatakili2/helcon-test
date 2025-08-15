//! Error types for the medical appointment system

#[derive(candid::CandidType, Deserialize, Serialize)]
pub enum Error {
    NotFound { msg: String },
    InvalidInput { msg: String },
    Unauthorized { msg: String },
    AppointmentConflict { msg: String },
    AlreadyExists { msg: String },
}