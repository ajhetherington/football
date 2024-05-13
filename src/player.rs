use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::TeamSide;

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
    pub team: Option<TeamSide>,
    pub uuid: String,
}

impl Player {
    pub fn new(name: String, number: usize, side: Option<TeamSide>) -> Player {
        Player {
            name,
            number,
            skills: SkillProfile::new().unwrap(),
            physicals: PhysicalProfile::new().unwrap(),
            team: side,
            uuid: Uuid::new_v4().to_string()
        }
    }

}
