use std::{env, process};

use birthday_sync::{caldav::create_events, carddav::get_contacts, settings::get_settings};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let flag = &args[1];

        if flag == "--version" {
            let version = env!("CARGO_PKG_VERSION");
            println!("{}", version);
            process::exit(0);
        } else {
            println!("Flag \"{}\" not recognized", flag);
            process::exit(1);
        }
    }

    let settings = get_settings().expect("Failed to get settings");

    let contacts = get_contacts(&settings.carddav).expect("Failed to retrieve contacts");
    create_events(&settings.caldav, contacts).expect("Failed to create all events");
}
