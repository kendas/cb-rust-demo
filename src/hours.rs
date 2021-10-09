use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct NewHours {
    pub employee: String,
    pub project: String,
    pub story_id: Option<String>,
    pub description: String,
    pub hours: i16,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Hours {
    pub id: Uuid,
    pub employee: String,
    pub project: String,
    pub story_id: Option<String>,
    pub description: String,
    pub hours: i16,
}

impl Hours {
    pub fn new(new_hours: NewHours) -> Hours {
        Hours {
            id: Uuid::new_v4(),
            employee: new_hours.employee,
            project: new_hours.project,
            story_id: new_hours.story_id,
            description: new_hours.description,
            hours: new_hours.hours,
        }
    }
}
