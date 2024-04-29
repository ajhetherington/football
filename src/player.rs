use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SkillProfile {
    pub technique: f32,
}

impl SkillProfile {
    pub fn new() -> Result<SkillProfile, String> {
        Ok(SkillProfile {
            technique: 5.0,
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PhysicalProfile {
    pub speed: i32,
    pub strength: i32,
}

impl PhysicalProfile {
    pub fn new() -> Result<PhysicalProfile, String> {
        Ok(PhysicalProfile {
            speed: 10,
            strength: 10,
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub number: usize,
    pub skills: SkillProfile,
    pub physicals: PhysicalProfile,
}

impl Player {
    pub fn new(name: String, number: usize) -> Player {
        Player {
            name,
            number,
            skills: SkillProfile::new().unwrap(),
            physicals: PhysicalProfile::new().unwrap(),
        }
    }

}
