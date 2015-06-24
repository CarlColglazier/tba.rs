use super::Session;
use rustc_serialize::json;
use rustc_serialize::json::DecoderError;
use std::fmt::Display;


#[derive(RustcDecodable)]
/// Team model.
pub struct Team {
    pub website: Option<String>,
    pub name: Option<String>,
    pub locality: Option<String>,
    pub region: Option<String>,
    pub country_name: Option<String>,
    pub location: Option<String>,
    pub team_number: Option<u16>,
    pub key: Option<String>,
    pub nickname: Option<String>,
    pub rookie_year: Option<u16>,
}

impl Team {
    pub fn get<T: Display>(session: &Session, number: T) -> Result<Team, DecoderError> {
        let url_path = format!("team/frc{}", number);
        let response = session.request(&url_path);
        let result: json::DecodeResult<Team> = json::decode(&response);
        return result;
    }
}
