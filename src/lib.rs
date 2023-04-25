pub mod models;
pub mod schema;
pub mod errors;
pub mod sql;

use rand::Rng;

pub fn generate_code(code: u8, string: &mut String) {
    let mut rng = rand::thread_rng();
    for _i in 1..code {
        if let 1 = rng.gen_range(1..3) {
            string.push((rng.gen_range(65..90) as u8) as char)
        }
        else {
            string.push((rng.gen_range(97..122) as u8) as char)
        }
    }
}

// impl generate for String {

// }