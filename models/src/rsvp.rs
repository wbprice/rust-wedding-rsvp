pub enum DietaryRestrictions {
    Vegetarian,
    Vegan,
    Pescetarian,
    GlutenFree,
    DairyFree
}

pub enum DishPreference {
    Chicken,
    Steak,
    Seabass
}

pub enum RSVP {
    No,
    Yes {
        dietary_restrictions: Option<DietaryRestrictions>,
        dish_preference: Option<DishPreference>
    }
}
