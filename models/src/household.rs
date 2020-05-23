use uuid::Uuid;
use crate::Person;

#[derive(Debug)]
pub struct Household {
    pub id: Uuid,
    pub people: Vec<Person>
}

