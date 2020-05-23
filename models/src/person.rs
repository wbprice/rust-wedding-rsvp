use crate::rsvp::RSVP;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Contact {
    Email { value: String },
    SMS { value: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub contact: Contact,
    pub rsvp: Option<RSVP>,
}
