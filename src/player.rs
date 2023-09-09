extern crate rand;
use rand::Rng;

#[derive(Debug)]
pub struct SkillProfile {
    passing: u8,
    shooting: u8,
    tackle: u8,
}

impl SkillProfile {
    pub fn new() -> Result<SkillProfile, String> {
        let mut rng = rand::thread_rng();
        // let random_value: u8 = rng.gen_range(1..=10); // 1 to 10 inclusive
        Ok(SkillProfile {
            passing: rng.gen_range(1..=10),
            shooting: rng.gen_range(1..=10),
            tackle: rng.gen_range(1..=10),
        })
    }
}

#[derive(Debug)]
pub struct PhysicalProfile {
    height: i32,
    speed: i32,
}

impl PhysicalProfile {
    pub fn new() -> Result<PhysicalProfile, String> {
        let mut rng = rand::thread_rng();
        // let random_value: u8 = rng.gen_range(1..=10); // 1 to 10 inclusive
        Ok(PhysicalProfile {
            height: rng.gen_range(1..=10),
            speed: rng.gen_range(1..=10),
        })
    }
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub number: i32,
    pub skills: SkillProfile,
    pub physicals: PhysicalProfile,
}

impl Player {
    pub fn new(name: String, number: i32) -> Result<Player, String> {
        if name == "Alex Hetherington" {
            Err(String::from("No, Alex Hetherington cannot play football"))
        } else {
            Ok(Player {
                name,
                number,
                skills: SkillProfile::new().unwrap(),
                physicals: PhysicalProfile::new().unwrap(),
            })
        }
    }
}
