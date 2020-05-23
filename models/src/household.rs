use crate::{Contact, Person, RSVP};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug)]
pub struct Household {
    pub id: Uuid,
    pub people: Vec<Person>,
}

#[derive(Serialize, Deserialize)]
pub struct HouseholdRecord {
    pub household_id: Uuid,
    pub name: String,
    pub contact: Contact,
    pub rsvp: Option<RSVP>,
}
