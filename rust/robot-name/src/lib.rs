extern crate rand;
use rand::Rng;

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Robot {
            name: Self::gen_name(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name[..]
    }

    pub fn reset_name(&mut self) {
        self.name = Self::gen_name();
    }

    fn gen_name() -> String {
        let mut rng = rand::thread_rng();
        [
            (b'A', 26 as u8),
            (b'A', 26),
            (b'0', 10),
            (b'0', 10),
            (b'0', 10),
        ]
        .iter()
        .map(|(offset, num)| (rng.gen_range(0, num) + offset) as char)
        .collect()
    }
}
