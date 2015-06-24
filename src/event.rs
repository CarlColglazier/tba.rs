use super::Session;
use super::Team;
use rustc_serialize::json;
use rustc_serialize::json::DecoderError;
use std::fmt::Display;

#[derive(RustcDecodable)]
/// Based on the event model in [The Blue Alliance API](http://www.thebluealliance.com/apidocs)
pub struct Event {
    pub key: Option<String>,
    pub name: Option<String>,
    pub short_name: Option<String>,
    pub event_code: Option<String>,
    pub event_type_string: Option<String>,
    pub event_type: Option<isize>,
    pub event_district_string: Option<String>,
    pub event_district: Option<isize>,
    pub year: Option<isize>,
    pub location: Option<String>,
    pub venue_address: Option<String>,
    pub website: Option<String>,
    pub official: Option<bool>
}

impl Event {
    pub fn get_event<T: Display>(session: &Session, key: T) -> Result<Event, DecoderError> {
        let url_path = format!("event/{}", key);
        let response = session.request(&url_path);
        let result: json::DecodeResult<Event> = json::decode(&response);
        return result;
    }

    pub fn get_event_teams<T: Display>(session: &Session, key: T) -> Result<Vec<Team>, DecoderError> {
        let url_path = format!("event/{}/teams", key);
        let response = session.request(&url_path);
        let result: json::DecodeResult<Vec<Team>> = json::decode(&response);
        return result;
    }
}
