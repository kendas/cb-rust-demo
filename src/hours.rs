use std::convert::TryFrom;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::Row;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct NewHours {
    pub employee: String,
    pub date: NaiveDate,
    pub project: String,
    pub story_id: Option<String>,
    pub description: String,
    pub hours: i16,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Hours {
    pub id: Uuid,
    pub employee: String,
    pub date: NaiveDate,
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
            date: new_hours.date,
            project: new_hours.project,
            story_id: new_hours.story_id,
            description: new_hours.description,
            hours: new_hours.hours,
        }
    }
}

impl TryFrom<PgRow> for Hours {
    type Error = sqlx::Error;

    fn try_from(value: PgRow) -> Result<Self, Self::Error> {
        Ok(Hours {
            id: value.try_get("id")?,
            employee: value.try_get("employee")?,
            date: value.try_get("date")?,
            project: value.try_get("project")?,
            story_id: value.try_get("story_id")?,
            description: value.try_get("description")?,
            hours: value.try_get("hours")?,
        })
    }
}
