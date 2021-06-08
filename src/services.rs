use super::configs;
use super::repositories;

use nanoid::nanoid;

pub fn get_url_by_id(id: &str) -> Result<String, &str> {
    repositories::get_url_by_id(id)
}

pub fn shorten_url(url: &str) -> String {
    let id = generate_unique_id();
    repositories::insert_url(&id, url);
    id
}

fn generate_unique_id() -> String {
    let mut id: String;
    loop {
        id = generate_random_id();
        if id_is_available(&id) { break; }
    }
    id
}

fn generate_random_id() -> String {
    let size = configs::ALIAS_SIZE;
    nanoid!(size, &nanoid::alphabet::SAFE)
}

fn id_is_available(id: &str) -> bool {
    match get_url_by_id(id) {
        Ok(_x) => false,
        Err(_x) => true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_randon_id() {
        assert_ne!(generate_random_id(), "")
    }
}