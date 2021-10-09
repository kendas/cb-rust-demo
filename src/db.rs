use std::sync::Mutex;

use uuid::Uuid;

use crate::hours::{Hours, NewHours};

pub type MemDb = Mutex<Vec<Hours>>;

pub trait HoursRepo: Send + Sync {
    fn by_id(&self, id: Uuid) -> Option<Hours>;
    fn delete(&self, id: Uuid) -> bool;
    fn list(&self) -> Vec<Hours>;
    fn insert(&self, h: NewHours) -> Hours;
}

impl HoursRepo for MemDb {
    fn by_id(&self, id: uuid::Uuid) -> Option<Hours> {
        let guard = self.lock().unwrap();
        guard.iter().find(|&h| h.id == id).cloned()
    }

    fn delete(&self, id: uuid::Uuid) -> bool {
        let mut guard = self.lock().unwrap();
        guard
            .iter()
            .position(|h| h.id == id)
            .map(|pos| {
                guard.remove(pos);
                true
            })
            .unwrap_or(false)
    }

    fn list(&self) -> std::vec::Vec<Hours> {
        let guard = self.lock().unwrap();
        let all_hours = &*guard;
        all_hours.to_vec()
    }

    fn insert(&self, h: NewHours) -> Hours {
        let mut guard = self.lock().unwrap();
        let hours_entry = Hours::new(h);
        guard.push(hours_entry.clone());
        hours_entry
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;
    use crate::hours::NewHours;

    #[test]
    fn by_id_when_db_is_empty() {
        let db: MemDb = Default::default();

        let result = db.by_id(Uuid::new_v4());

        assert!(result.is_none());
    }

    #[test]
    fn by_id_exists() {
        let db: MemDb = Default::default();
        let hours = db.insert(NewHours {
            employee: "employee".to_owned(),
            project: "project".to_owned(),
            story_id: None,
            description: "description".to_owned(),
            hours: 1,
        });

        match db.by_id(hours.id) {
            Some(result) => assert_eq!(result, hours),
            None => panic!("Expected hours to be returned."),
        }
    }

    #[test]
    fn by_id_db_not_empty_invalid_key() {
        let db: MemDb = Default::default();
        db.insert(NewHours {
            employee: "employee".to_owned(),
            project: "project".to_owned(),
            story_id: None,
            description: "description".to_owned(),
            hours: 1,
        });

        let result = db.by_id(Uuid::new_v4());

        assert!(result.is_none());
    }

    #[test]
    fn delete_db_empty() {
        let db: MemDb = Default::default();

        let result = db.delete(Uuid::new_v4());

        assert_eq!(result, false);
    }

    #[test]
    fn delete_db_not_empty() {
        let db: MemDb = Default::default();
        let hours = db.insert(NewHours {
            employee: "employee".to_owned(),
            project: "project".to_owned(),
            story_id: None,
            description: "description".to_owned(),
            hours: 1,
        });

        let result = db.delete(hours.id);

        assert_eq!(result, true);

        assert!(db.by_id(hours.id).is_none());
    }

    #[test]
    fn delete_db_not_empty_invalid_key() {
        let db: MemDb = Default::default();
        let hours = db.insert(NewHours {
            employee: "employee".to_owned(),
            project: "project".to_owned(),
            story_id: None,
            description: "description".to_owned(),
            hours: 1,
        });

        let result = db.delete(Uuid::new_v4());

        assert_eq!(result, false);

        match db.by_id(hours.id) {
            Some(stored) => assert_eq!(stored, hours),
            None => panic!("Expected hours to still be stored."),
        }
    }

    #[test]
    fn list_db_empty() {
        let db: MemDb = Default::default();

        let result = db.list();

        assert_eq!(result, vec![]);
    }

    #[test]
    fn list_db_not_empty() {
        let db: MemDb = Default::default();
        let hours = db.insert(NewHours {
            employee: "employee".to_owned(),
            project: "project".to_owned(),
            story_id: None,
            description: "description".to_owned(),
            hours: 1,
        });

        let result = db.list();

        assert_eq!(result, vec![hours]);
    }
}
