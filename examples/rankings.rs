extern crate tba;

use std::process::exit;
use tba::Match;

use std::cmp::Ordering;

struct Team {
    key: String,
    rating: f32,
}

// Contains a list of teams and a rating for that group.
struct Alliance {
    teams: Vec<String>,
    rating: f32,
}

// K constant used in elo ratings.
static K: f32 = 16f32;

// TODO: Implement these functions as an impl for Vec<Team>

// Check if a team is already in the vector.
fn has_team(list: &Vec<Team>, team: &str) -> bool {
    for t in list.iter() {
        if t.key == team {
            return true
        }
    }
    return false
}

// Only gets called after `has_team` is checked.
fn get_team_rating(list: &Vec<Team>, team: &str) -> f32 {
    let mut index: usize = 0;
    for t in list.iter() {
        index += 1;
        if t.key == team {
            break;
        }
    }
    match list.get(index) {
        None => return 0f32,
        Some(c_team) => return c_team.rating,
    };
}

fn update_team_rating(list: &mut Vec<Team>, team: &str, change: f32) {
    let mut index: usize = 0;
    for t in list.iter() {
        index += 1;
        if t.key == team {
            break;
        }
    }
    match list.get_mut(index) {
        None => (),
        Some(current_team) => current_team.rating += change,
    };
}

// Elo calculation.
fn elo_change(winner: f32, loser: f32, result: f32) -> f32 {
    K * (result - 1f32 / (1f32 + 10f32.powf((loser - winner) / 400f32)))
}

fn main() {
    // Create a new instance of `tba` and pass in authentication information.
    let blue_alliance = tba::new("Carl Colglazier", "FRC ELO (testing)", "0.0.0");

    // Load the events. Since tba::events returns
    let events = match blue_alliance.events("2015") {
        Err(_) => {
            println!("Year not found");
            exit(1);
        },
        Ok(events) => events,
    };
    let mut match_list: Vec<Match> = Vec::new();
    for event in events.iter() {
        match event.official {
            None => continue,
            Some(is_official) => {
                if !is_official {
                    continue
                }
            }
        }
        let key = match event.key {
            Some(ref value) => value,
            None => return
        };
        let matches = match blue_alliance.event_matches(&key) {
            Err(_) => return,
            Ok(m) => m,
        };

        // It took me an hour to figure out that I should use
        // `into_iter` instead of `iter` here. Yikes!
        for x in matches.into_iter() {
            match_list.push(x);
        }
    }
    match_list.retain(|x| return match x.alliances {
        None => false,
        Some(ref value) => {
            if value.blue.score >= 0 {
                true
            } else {
                false
            }
        }
    });

    // Sort the matches by time.
    match_list.sort_by(|a, b| return match match a.time {
        None => 0,
        Some(value) => value,
    } > match b.time {
        None => 0,
        Some(value) => value,
    } {
        true => Ordering::Greater,
        false => Ordering::Less
    });

    let mut teams: Vec<Team> = Vec::new();

    // Used for sorting later on.
    let mut team_names: Vec<String> = Vec::new();

    for m in match_list.into_iter() {
        match m.alliances {
            None => continue,
            Some(a) => {
                let mut red = Alliance { teams: Vec::new(), rating: 0f32 };
                let mut blue = Alliance { teams: Vec::new(), rating: 0f32 };
                for team in a.red.teams.into_iter() {
                    let team_rating: f32;
                    if has_team(&teams, &team) {
                        team_rating = get_team_rating(&teams, &team);
                    } else {
                        team_rating = 1000f32;
                        let new_team = Team{ key: team.to_string(), rating: 1000f32 };
                        teams.push(new_team);
                        team_names.push(team.clone());
                    }
                    red.rating += team_rating;
                    red.teams.push(team);
                }
                for team in a.blue.teams.into_iter() {
                    let team_rating: f32;
                    if has_team(&teams, &team) {
                        team_rating = get_team_rating(&teams, &team);
                    } else {
                        team_rating = 1000f32;
                        let new_team = Team{ key: team.to_string(), rating: 1000f32 };
                        teams.push(new_team);
                        team_names.push(team.clone());
                    }
                    blue.rating += team_rating;
                    blue.teams.push(team);
                }

                // Turn the ratings into averages.
                red.rating /= red.teams.len() as f32;
                blue.rating /= blue.teams.len() as f32;

                // K * (score - expected).
                let updated_score: f32;
                if a.red.score > a.blue.score {
                    // Red win.
                    updated_score = elo_change(red.rating, blue.rating, 1f32);
                } else if a.red.score < a.blue.score {
                    // Blue win.
                    updated_score = elo_change(red.rating, blue.rating, 0f32);
                } else {
                    // Tie.
                    updated_score = elo_change(red.rating, blue.rating, 0.5f32);
                }

                // Update the ratings for each team.
                for team in red.teams.iter() {
                    let change = updated_score * (get_team_rating(&teams, &team) / red.rating);
                    update_team_rating(&mut teams, team, change);
                }
                for team in blue.teams.iter() {
                    let change = -updated_score * (get_team_rating(&teams, &team) / blue.rating);
                    update_team_rating(&mut teams, team, change);
                }
            },
        }
    }

    // Sort teams by rating.
    team_names.sort_by(|a, b| return match get_team_rating(&teams, &a) < get_team_rating(&teams, &b) {
        true => Ordering::Greater,
        false => Ordering::Less,
    });

    // Print the ratings.
    for team in team_names.iter() {
        println!("{}  : {}", team, get_team_rating(&teams, &team));
    }

}
