pub mod models;
pub mod schema;
pub mod errors;
pub mod sql;

use rand::Rng;

pub trait GenCode {
    fn generate_code(&mut self, code: u8) -> &mut String;
}

impl GenCode for String {
    fn generate_code(&mut self, code: u8) -> &mut String {
        let mut rng = rand::thread_rng();
        for _i in 0..code {
            // let rd = rng.gen_range(1..2);
            match rng.gen_range(1..3) {
                1 => self.push((rng.gen_range(65..90) as u8) as char),
                2 => self.push((rng.gen_range(97..122) as u8) as char),
                3 => self.push((rng.gen_range(48..57) as u8) as char),
                _ => (),
            }
        }
        self
    }
}
