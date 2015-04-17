// Copyright 2015 Carl Colglazier.
// See LICENSE for details.

//! tba provides native interaction with [The Blue Alliance](http://thebluealliance.com/)
//! in the [Rust programming language](http://rust-lang.org/)
extern crate hyper;
extern crate rustc_serialize;

use std::io::Read;

use hyper::Client;

use rustc_serialize::json;


/// Session keys to populate the X-TBA-App-Id.
/// Most functions are implemented for this type.
pub struct Session {
    developer: &'static str,
    application: &'static str,
    pub version: &'static str,
}

#[derive(RustcDecodable)]
/// Based on the team model in [The Blue Alliance API](http://www.thebluealliance.com/apidocs)
pub struct Team {
    pub website: Option<String>,
    pub name: Option<String>,
    pub locality: Option<String>,
    pub region: Option<String>,
    pub country_name: Option<String>,
    pub location: Option<String>,
    pub team_number: Option<u32>,
    pub key: Option<String>,
    pub nickname: Option<String>,
    pub rookie_year: Option<u32>
}

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

#[derive(RustcDecodable)]
/// Based on the match model in [The Blue Alliance API](http://www.thebluealliance.com/apidocs)
pub struct Match {
    pub key: Option<String>,
    pub comp_level: Option<String>,
    pub set_number: Option<i8>,
    pub match_number: Option<isize>,
    pub alliances: Option<Alliances>,
    pub event_key: Option<String>,
    pub time_string: Option<String>,
    pub time: Option<isize>
}

#[derive(RustcDecodable)]
pub struct Alliances {
    pub red: Alliance,
    pub blue: Alliance
}

#[derive(RustcDecodable)]
pub struct Alliance {
    pub score: isize,
    pub teams: Vec<String>
}

impl Session {

    /// Create a new Session.
    /// This function should be called when tba is first used
    /// in order to interface with TBA.
    fn new(d: &'static str, a: &'static str, v: &'static str) -> Session {
        Session { developer: d, application: a, version: v }
    }

    /// http://www.thebluealliance.com/apidocs#team-list-request
    pub fn teams(&self, page: &str) -> json::DecodeResult<Vec<Team>> {
        let path = format!("teams/{}", page);
        let results = request(self, path);
        json::decode(&results)
    }

    /// http://www.thebluealliance.com/apidocs#team-request
    pub fn team(&self, team: &str) -> json::DecodeResult<Team> {
        let path = format!("team/frc{}", team);
        let results = request(self, path);
        json::decode(&results)
    }

    /// http://www.thebluealliance.com/apidocs#team-events-request
    pub fn team_events(&self, team: &str, event: &str) -> json::DecodeResult<Vec<Event>> {
        let path = format!("team/frc{}/event/{}/matches", team, event);
        let results = request(self, path);
        json::decode(&results)
    }

    /// http://www.thebluealliance.com/apidocs#team-event-matches-request
    pub fn team_event_matches(&self, team: &str, year: &str) -> json::DecodeResult<Vec<Match>> {
        let path = format!("team/frc{}/{}/events", team, year);
        let results = request(self, path);
        json::decode(&results)
    }

    /// http://www.thebluealliance.com/apidocs#event-list-request
    pub fn events(&self, year: &str) -> json::DecodeResult<Vec<Event>> {
        let path = format!("events/{}", year);
        let results = request(self, path);
        json::decode(&results)
    }

    /// http://www.thebluealliance.com/apidocs#event-request
    pub fn event(&self, event: &str) -> json::DecodeResult<Event> {
        let path = format!("event/{}", event);
        let results = request(self, path);
        json::decode(&results)
    }

    /// http://www.thebluealliance.com/apidocs#event-teams-request
    pub fn event_teams(&self, event: &str) -> json::DecodeResult<Vec<Team>> {
        let path = format!("event/{}/teams", event);
        let results = request(self, path);
        json::decode(&results)
    }

    /// http://www.thebluealliance.com/apidocs#event-matches-request
    pub fn event_matches(&self, event: &str) -> json::DecodeResult<Vec<Match>> {
        let path = format!("event/{}/matches", event);
        let results = request(self, path);
        json::decode(&results)
    }

    /// `match` is a reserved keyword in Rust,
    /// so we'll use `mach` instead.
    ///
    /// http://www.thebluealliance.com/apidocs#match-request
    pub fn mach(&self, matc: &str) -> json::DecodeResult<Match> {
        let path = format!("match/{}", matc);
        let results = request(self, path);
        json::decode(&results)
    }

}

fn request(tba: &Session, path: String) -> String {

    let base_url = "http://thebluealliance.com/api/v2";
    let url = format!("{}/{}?X-TBA-App-Id={}:{}:{}", base_url, path, tba.developer, tba.application, tba.version);

    let mut client = Client::new();
    let mut res = client.get(&*url).send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    body
}

/// Create a new session.
pub fn new(developer: &'static str, application: &'static str, version: &'static str) -> Session {
    Session::new(developer, application, version)
}
