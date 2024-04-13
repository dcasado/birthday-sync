use birthday_sync::{caldav::create_events, carddav::get_contacts, settings::get_settings};

fn main() {
    let settings = get_settings().expect("Failed to get settings");

    let contacts = get_contacts(&settings.carddav).expect("Failed to retrieve contacts");
    create_events(&settings.caldav, contacts).expect("Failed to create all events");
}
