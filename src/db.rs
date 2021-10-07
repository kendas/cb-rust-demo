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
    fn by_id(&self, id: uuid::Uuid) -> std::option::Option<Hours> {
        let guard = self.lock().unwrap();
        let result = guard.iter().find(|&h| h.id == id).map(|h| h.clone());
        return result;
    }
    fn delete(&self, id: uuid::Uuid) -> bool {
        let mut guard = self.lock().unwrap();
        let result = guard.iter().position(|h| h.id == id);
        match result {
            Some(hours_index) => {
                guard.remove(hours_index);
                return true;
            }
            None => false,
        }
    }
    fn list(&self) -> std::vec::Vec<Hours> {
        let guard = self.lock().unwrap();
        let all_hours = &*guard;
        return all_hours.to_vec();
    }
    fn insert(&self, h: NewHours) -> Hours {
        let mut guard = self.lock().unwrap();
        let hours_entry = Hours::new(h);
        guard.push(hours_entry.clone());
        return hours_entry;
    }
}
