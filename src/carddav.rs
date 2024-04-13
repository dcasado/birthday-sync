use std::io::BufReader;

use ical::parser::{vcard::component::VcardContact, Component};

use crate::settings::CardDAVSettings;

const DAV_NS: &str = "DAV:";
const CR_NS: &str = "urn:ietf:params:xml:ns:carddav";
const PROPSTAT: &str = "propstat";
const PROP: &str = "prop";
const ADDRESS_DATA: &str = "address-data";

const UID_PROPERTY: &str = "UID";
const FN_PROPERTY: &str = "FN";
const BDAY_PROPERTY: &str = "BDAY";

#[derive(Debug)]
pub struct Contact {
    pub uid: String,
    pub full_name: String,
    pub birthday: String,
}

pub fn get_contacts(
    carddav_settings: &CardDAVSettings,
) -> Result<Vec<Contact>, Box<dyn std::error::Error>> {
    let report_method = reqwest::Method::from_bytes(b"REPORT").unwrap();

    let body = r#"<?xml version="1.0" encoding="UTF-8"?>
<carddav:adbk-query xmlns:d="DAV:" xmlns:carddav="urn:ietf:params:xml:ns:carddav">
    <carddav:filter>
        <carddav:prop-filter name="BDAY"/>
    </carddav:filter>
    <d:prop>
        <carddav:address-data>
            <carddav:prop name="UID"/>
            <carddav:prop name="FN"/>
            <carddav:prop name="BDAY"/>
        </carddav:address-data>
    </d:prop>
</carddav:adbk-query>"#;

    let response = reqwest::blocking::Client::new()
        .request(report_method, carddav_settings.url.as_str())
        .basic_auth(
            carddav_settings.user.as_str(),
            Some(carddav_settings.password.as_str()),
        )
        .body(body)
        .send()?
        .error_for_status()?;

    let contacts = parse_response(response.text()?)?;

    Ok(contacts)
}

fn parse_response(response: String) -> Result<Vec<Contact>, Box<dyn std::error::Error>> {
    let root: minidom::Element = response.parse()?;

    let mut contacts: Vec<Contact> = vec![];

    for response in root.children() {
        let propstat = response
            .get_child(PROPSTAT, DAV_NS)
            .unwrap_or_else(|| panic!("{} element not found", PROPSTAT));
        let prop = propstat
            .get_child(PROP, DAV_NS)
            .unwrap_or_else(|| panic!("{} element not found", PROP));
        let address_data = prop
            .get_child(ADDRESS_DATA, CR_NS)
            .unwrap_or_else(|| panic!("{} element not found", ADDRESS_DATA));
        let vcard = address_data.text();

        let buf = BufReader::new(vcard.as_bytes());
        let parser = ical::VcardParser::new(buf);

        for vcard_contact in parser {
            let contact = parse_vcard_contact(vcard_contact?);
            contacts.push(contact);
        }
    }

    Ok(contacts)
}

fn parse_vcard_contact(vcard_contact: VcardContact) -> Contact {
    let uid = vcard_contact
        .get_property(UID_PROPERTY)
        .unwrap_or_else(|| panic!("Failed to get {} property", UID_PROPERTY))
        .value
        .clone()
        .unwrap_or_else(|| panic!("Failed to get value of {} property", UID_PROPERTY));
    let full_name = vcard_contact
        .get_property(FN_PROPERTY)
        .unwrap_or_else(|| panic!("Failed to get {} property", FN_PROPERTY))
        .value
        .clone()
        .unwrap_or_else(|| panic!("Failed to get value of {} property", FN_PROPERTY));
    let bday = vcard_contact
        .get_property(BDAY_PROPERTY)
        .unwrap_or_else(|| panic!("Failed to get {} property", BDAY_PROPERTY))
        .value
        .clone()
        .unwrap_or_else(|| panic!("Failed to get value of {} property", BDAY_PROPERTY));

    Contact {
        uid,
        full_name,
        birthday: bday,
    }
}
