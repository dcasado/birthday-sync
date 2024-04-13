use crate::{carddav::Contact, settings::CalDAVSettings};

pub fn create_events(
    caldav_settings: &CalDAVSettings,
    contacts: Vec<Contact>,
) -> Result<(), reqwest::Error> {
    let client = reqwest::blocking::Client::new();

    for contact in contacts {
        create_event(
            caldav_settings,
            &client,
            contact.uid.as_ref(),
            contact.full_name.as_ref(),
            contact.birthday.as_ref(),
        )
        .unwrap_or_else(|e| {
            println!(
                "Failed to create event event for {:?} because {}",
                contact, e
            )
        });
    }

    Ok(())
}

fn create_event(
    caldav_settings: &CalDAVSettings,
    client: &reqwest::blocking::Client,
    uid: &str,
    full_name: &str,
    bday: &str,
) -> Result<(), reqwest::Error> {
    let vcalendar: String = format!(
        "BEGIN:VCALENDAR
BEGIN:VEVENT
UID:{}
SUMMARY:{}'s birthday
DTSTART;VALUE=DATE:{}
RRULE:FREQ=YEARLY;INTERVAL=1
BEGIN:VALARM
ACTION:DISPLAY
DESCRIPTION:{}'s birthday
TRIGGER:PT0S
END:VALARM
END:VEVENT
END:VCALENDAR",
        uid,
        full_name,
        bday.replace('-', ""),
        full_name,
    );

    client
        .request(
            reqwest::Method::PUT,
            format!("{}/{}.ics", caldav_settings.url, uid),
        )
        .basic_auth(
            caldav_settings.user.as_str(),
            Some(caldav_settings.password.as_str()),
        )
        .header(reqwest::header::CONTENT_TYPE, "text/calendar")
        .body(vcalendar)
        .send()?
        .error_for_status()?;

    Ok(())
}
