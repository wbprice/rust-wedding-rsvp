use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Contact {
    Email { value: String },
    SMS { value: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub household_id: Uuid,
    pub name: String,
    pub contact: Contact,
    pub rsvp: Option<bool>,
    pub dietary_restrictions: Option<DietaryRestrictions>,
    pub dish_preference: Option<DishPreference>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DietaryRestrictions {
    Vegetarian,
    Vegan,
    Pescetarian,
    GlutenFree,
    DairyFree,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DishPreference {
    Chicken,
    Steak,
    Pancakes,
    Pizza    
}
