#[derive(RustcDecodable, Clone)]
pub struct Alliances {
    pub red: Alliance,
    pub blue: Alliance
}

#[derive(RustcDecodable, Clone)]
pub struct Alliance {
    pub score: isize,
    pub teams: Vec<String>
}
