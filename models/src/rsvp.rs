use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DietaryRestrictions {
    Vegetarian,
    Vegan,
    Pescetarian,
    GlutenFree,
    DairyFree
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DishPreference {
    Chicken,
    Steak,
    Seabass
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RSVP {
    No,
    Yes {
        dietary_restrictions: Option<DietaryRestrictions>,
        dish_preference: Option<DishPreference>
    }
}
