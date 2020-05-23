use crate::rsvp::RSVP;
enum Contact {
    Email { value: String },
    Sms { value : String }
}

pub struct Person {
    name: String,
    contact: Contact,
    rsvp: Option<RSVP>
}
