use std::convert::TryFrom;

use async_trait::async_trait;
use sqlx::pool::PoolConnection;
use sqlx::{PgConnection, Postgres};
use uuid::Uuid;

use crate::hours::{Hours, NewHours};

pub struct HoursRepository<'a>(&'a mut PgConnection);

#[async_trait]
pub trait HoursRepo {
    async fn by_id(&mut self, id: Uuid) -> Option<Hours>;
    async fn list(&mut self) -> Vec<Hours>;
    async fn insert(&mut self, h: NewHours) -> Hours;
    async fn delete(&mut self, id: Uuid) -> bool;
}

#[async_trait]
impl HoursRepo for PoolConnection<Postgres> {
    async fn by_id(&mut self, id: Uuid) -> Option<Hours> {
        sqlx::query("SELECT * FROM hours WHERE id = $1")
            .bind(id)
            .map(|row| Hours::try_from(row).unwrap())
            .fetch_optional(self)
            .await
            .unwrap()
    }

    async fn list(&mut self) -> Vec<Hours> {
        sqlx::query("SELECT * FROM hours")
            .map(|row| Hours::try_from(row).unwrap())
            .fetch_all(self)
            .await
            .unwrap()
    }

    async fn insert(&mut self, h: NewHours) -> Hours {
        let hours = Hours::new(h);
        let sql = "INSERT INTO hours (id, employee, date, project, story_id, description, hours)
            VALUES ($1, $2, $3, $4, $5, $6, $7)";
        sqlx::query(sql)
            .bind(hours.id)
            .bind(hours.employee.clone())
            .bind(hours.date)
            .bind(hours.project.clone())
            .bind(hours.story_id.clone())
            .bind(hours.description.clone())
            .bind(hours.hours)
            .execute(self)
            .await
            .unwrap();
        hours
    }

    async fn delete(&mut self, id: Uuid) -> bool {
        sqlx::query("DELETE FROM hours WHERE id = $1 RETURNING 1")
            .bind(id)
            .fetch_optional(self)
            .await
            .unwrap()
            .is_some()
    }
}

#[cfg(test)]
pub mod tests {
    use chrono::NaiveDate;
    use uuid::Uuid;

    use super::*;
    use crate::hours::NewHours;
    use crate::test_utils;

    #[actix_rt::test]
    async fn by_id_when_db_is_empty() {
        let mut db = test_utils::internal::get_db_connection().await;

        let result = db.by_id(Uuid::new_v4()).await;

        assert!(result.is_none());
    }

    #[actix_rt::test]
    async fn by_id_exists() {
        let mut db = test_utils::internal::get_db_connection().await;

        let hours = db.insert(get_hours()).await;

        match db.by_id(hours.id).await {
            Some(result) => assert_eq!(result, hours),
            None => panic!("Expected hours to be returned."),
        }
    }

    #[actix_rt::test]
    async fn by_id_db_not_empty_invalid_key() {
        let mut db = test_utils::internal::get_db_connection().await;

        db.insert(get_hours()).await;

        let result = db.by_id(Uuid::new_v4()).await;

        assert!(result.is_none());
    }

    #[actix_rt::test]
    async fn delete_db_empty() {
        let mut db = test_utils::internal::get_db_connection().await;

        let result = db.delete(Uuid::new_v4()).await;

        assert!(!result);
    }

    #[actix_rt::test]
    async fn delete_db_not_empty() {
        let mut db = test_utils::internal::get_db_connection().await;

        let hours = db.insert(get_hours()).await;

        let result = db.delete(hours.id).await;

        assert!(result);

        assert!(db.by_id(hours.id).await.is_none());
    }

    #[actix_rt::test]
    async fn delete_db_not_empty_invalid_key() {
        let mut db = test_utils::internal::get_db_connection().await;

        let hours = db.insert(get_hours()).await;

        let result = db.delete(Uuid::new_v4()).await;

        assert!(!result);

        match db.by_id(hours.id).await {
            Some(stored) => assert_eq!(stored, hours),
            None => panic!("Expected hours to still be stored."),
        }
    }

    #[actix_rt::test]
    async fn list_db_empty() {
        let mut db = test_utils::internal::get_db_connection().await;

        let result = db.list().await;

        assert_eq!(result, vec![]);
    }

    #[actix_rt::test]
    async fn list_db_not_empty() {
        let mut db = test_utils::internal::get_db_connection().await;

        let hours = db.insert(get_hours()).await;

        let result = db.list().await;

        assert_eq!(result, vec![hours]);
    }

    fn get_hours() -> NewHours {
        NewHours {
            employee: "employee".to_owned(),
            date: NaiveDate::from_ymd(2021, 10, 9),
            project: "project".to_owned(),
            story_id: None,
            description: "description".to_owned(),
            hours: 1,
        }
    }
}
