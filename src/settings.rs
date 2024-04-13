use std::env::VarError;

const CARDDAV_URL_ENV_VAR: &str = "CARDDAV_URL";
const CARDDAV_USER_ENV_VAR: &str = "CARDDAV_USER";
const CARDDAV_PASSWORD_ENV_VAR: &str = "CARDDAV_PASSWORD";

const CALDAV_URL_ENV_VAR: &str = "CALDAV_URL";
const CALDAV_USER_ENV_VAR: &str = "CALDAV_USER";
const CALDAV_PASSWORD_ENV_VAR: &str = "CALDAV_PASSWORD";

pub struct Settings {
    pub carddav: CardDAVSettings,
    pub caldav: CalDAVSettings,
}

impl Settings {
    fn new(
        carddav_url: String,
        carddav_user: String,
        carddav_password: String,
        caldav_url: String,
        caldav_user: String,
        caldav_password: String,
    ) -> Settings {
        let carddav = CardDAVSettings::new(carddav_url, carddav_user, carddav_password);
        let caldav = CalDAVSettings::new(caldav_url, caldav_user, caldav_password);

        Settings { carddav, caldav }
    }
}

pub struct CardDAVSettings {
    pub url: String,
    pub user: String,
    pub password: String,
}

impl CardDAVSettings {
    fn new(url: String, user: String, password: String) -> CardDAVSettings {
        CardDAVSettings {
            url,
            user,
            password,
        }
    }
}

pub struct CalDAVSettings {
    pub url: String,
    pub user: String,
    pub password: String,
}

impl CalDAVSettings {
    fn new(url: String, user: String, password: String) -> CalDAVSettings {
        CalDAVSettings {
            url,
            user,
            password,
        }
    }
}

pub fn get_settings() -> Result<Settings, VarError> {
    let carddav_url: String = std::env::var(CARDDAV_URL_ENV_VAR)
        .unwrap_or_else(|_| panic!("{} not set.", CARDDAV_URL_ENV_VAR));
    let carddav_user: String = std::env::var(CARDDAV_USER_ENV_VAR)
        .unwrap_or_else(|_| panic!("{} not set.", CARDDAV_USER_ENV_VAR));
    let carddav_password: String = std::env::var(CARDDAV_PASSWORD_ENV_VAR)
        .unwrap_or_else(|_| panic!("{} not set.", CARDDAV_PASSWORD_ENV_VAR));

    let caldav_url: String = std::env::var(CALDAV_URL_ENV_VAR)
        .unwrap_or_else(|_| panic!("{} not set.", CALDAV_URL_ENV_VAR));
    let caldav_user: String = std::env::var(CALDAV_USER_ENV_VAR)
        .unwrap_or_else(|_| panic!("{} not set.", CALDAV_USER_ENV_VAR));
    let caldav_password: String = std::env::var(CALDAV_PASSWORD_ENV_VAR)
        .unwrap_or_else(|_| panic!("{} not set.", CALDAV_PASSWORD_ENV_VAR));

    Ok(Settings::new(
        carddav_url,
        carddav_user,
        carddav_password,
        caldav_url,
        caldav_user,
        caldav_password,
    ))
}
