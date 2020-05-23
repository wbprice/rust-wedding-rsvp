use uuid::Uuid;
use crate::Person;

pub struct Household {
    id: Uuid,
    people: Vec<Person>
}

