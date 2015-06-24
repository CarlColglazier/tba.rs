//! tba is a [Rust] wrapper for [The Blue Alliance] API.
//!
//!
//!
//! # Examples
//!
//! ```
//! let the_blue_alliance = tba::Session::new("Team", "App", "Version");
//! let the_hitchhikers = match the_blue_alliance.get_team(2059) {
//!     Ok(team) => team,
//!     Err(e) => panic!("{}", e),
//! };
//! assert_eq!(2059, the_hitchhikers.team_number.unwrap());
//! ```
//!
//! [Rust]: http://www.rust-lang.org/ "The Rust Programming Language"
//! [The Blue Alliance]: http://www.thebluealliance.com/

// Hyper handles the HTTP request we will make to The Blue Alliance's API.
extern crate hyper;
// rustc_serialize parses the JSON response to the native structures that we will define below.
extern crate rustc_serialize;

use std::string::ToString;
use std::io::Read;
use std::fmt::Display;

use rustc_serialize::json::DecoderError;
use hyper::Client;

mod team;
mod event;
mod game;
mod shared;

pub use team::Team;
pub use event::Event;
pub use game::Game;
pub use shared::*;

const BASE_URL: &'static str = "http://thebluealliance.com/api/v2/";

pub struct Session {
    developer: String,
    application: String,
    version: String,
}

impl ToString for Session {
        fn to_string(&self) -> String {
            let mut string = String::new();
            string.push_str(&self.developer);
            string.push(':');
            string.push_str(&self.application);
            string.push(':');
            string.push_str(&self.version);
            return string;
        }
}

impl Session {
    /// Create a new Session.
    pub fn new<T, U, V>(developer: T, application: U, version: V) -> Session
        where T: ToString, U: ToString, V: ToString
    {
        return Session {
            developer: developer.to_string(),
            application: application.to_string(),
            version: version.to_string(),
        };
    }

    pub fn request(&self, path: &str) -> String {
        let mut client = Client::new();
        let url = format!("{}{}?X-TBA-App-Id={}", BASE_URL, path, self.to_string());
        let mut res = client.get(&url).send().unwrap();
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        return body;
    }

    /* Team events */

    pub fn get_team<T: Display>(&self, number: T) -> Result<Team, DecoderError> {
        return Team::get(self, number);
    }

    // pub fn get_teams<T: Display>


    /* Event events */

    pub fn get_event<T: Display>(&self, key: T) -> Result<Event, DecoderError> {
        return Event::get_event(self, key);
    }

    pub fn get_event_teams<T: Display>(&self, key: T) -> Result<Vec<Team>, DecoderError> {
        return Event::get_event_teams(self, key);
    }

    /* Match events */

    pub fn get_match<T: Display>(&self, key: T) -> Result<Game, DecoderError> {
        return Game::get_match(self, key);
    }
}

#[cfg(test)]
mod test {
    use super::Session;
    #[test]
    fn get_team_u16() {
        let the_blue_alliance = Session::new("Carl Colglazier", "tba.rs", "0.0.0");
        let team_info = match the_blue_alliance.get_team(2059) {
            Ok(team) => team,
            Err(e) => panic!("{}", e),
        };
        assert_eq!(2059, team_info.team_number.unwrap());
        assert_eq!("Apex", team_info.locality.unwrap());
        assert_eq!("NC", team_info.region.unwrap());
        assert_eq!("The Hitchhikers", team_info.nickname.unwrap());
        // Not quite true; our team first started competing in 2011!
        assert_eq!(2007, team_info.rookie_year.unwrap());
    }

    #[test]
    fn get_team_str() {
        let the_blue_alliance = Session::new("Carl Colglazier", "tba.rs", "0.0.0");
        let team_info = match the_blue_alliance.get_team("2059") {
            Ok(team) => team,
            Err(e) => panic!("{}", e),
        };
        assert_eq!(2059, team_info.team_number.unwrap());
        assert_eq!("Apex", team_info.locality.unwrap());
        assert_eq!("NC", team_info.region.unwrap());
        assert_eq!("The Hitchhikers", team_info.nickname.unwrap());
        // Not quite true; our team first started competing in 2011!
        assert_eq!(2007, team_info.rookie_year.unwrap());
    }

    #[test]
    fn get_bad_team() {
        let the_blue_alliance = Session::new("Carl Colglazier", "tba.rs", "0.0.0");
        assert!(the_blue_alliance.get_team("The Hichhikers").is_err());
    }
}
