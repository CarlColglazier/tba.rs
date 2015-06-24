use super::Alliances;
use super::Session;
use rustc_serialize::json;
use rustc_serialize::json::DecoderError;
use std::fmt::Display;

#[derive(RustcDecodable)]
/// Based on the match model in [The Blue Alliance API](http://www.thebluealliance.com/apidocs)
pub struct Game {
    pub key: Option<String>,
    pub comp_level: Option<String>,
    pub set_number: Option<u8>,
    pub match_number: Option<isize>,
    pub alliances: Option<Alliances>,
    pub event_key: Option<String>,
    pub time_string: Option<String>,
    pub time: Option<isize>
}

impl Game {
    pub fn get_match<T: Display>(session: &Session, key: T) -> Result<Game, DecoderError> {
        let url_path = format!("match/{}", key);
        let response = session.request(&url_path);
        let result: json::DecodeResult<Game> = json::decode(&response);
        return result;
    }
}

#[cfg(test)]
mod test {
    use super::super::Session;
    use super::*;
    #[test]
    fn get_match() {
        let the_blue_alliance = Session::new("Carl Colglazier", "tba.rs", "0.0.0");
        let key = "2015ncre_f1m2";
        let rr_ncre_final = match Game::get_match(&the_blue_alliance, key) {
            Ok(game) => game,
            Err(e) => panic!("{}", e),
        };
        assert_eq!(key, rr_ncre_final.key.unwrap());
        assert_eq!("f", rr_ncre_final.comp_level.unwrap());
        assert_eq!(1, rr_ncre_final.set_number.unwrap());
        assert_eq!(2, rr_ncre_final.match_number.unwrap());
        let alliances = rr_ncre_final.alliances.unwrap();
        assert_eq!(72, alliances.blue.score);
        assert_eq!(vec!["frc2059", "frc1225", "frc900"], alliances.blue.teams);
        assert_eq!("2015ncre", rr_ncre_final.event_key.unwrap());
    }
}
