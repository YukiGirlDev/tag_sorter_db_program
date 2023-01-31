use std::collections::HashSet;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Item {
    pub id: usize,
    pub path: PathBuf,
    pub year: i32,
    pub month: u8,
    pub day: u8,
    pub tags: HashSet<String>,
}
#[cfg(test)]
#[allow(clippy::missing_const_for_fn)]
impl Item {
    pub fn id(mut self, id: usize) -> Self {
        self.id = id;
        self
    }
    pub fn path(mut self, path: PathBuf) -> Self {
        self.path = path;
        self
    }
    pub fn year(mut self, year: i32) -> Self {
        self.year = year;
        self
    }
    pub fn month(mut self, month: u8) -> Self {
        self.month = month;
        self
    }
    pub fn day(mut self, day: u8) -> Self {
        self.day = day;
        self
    }
    pub fn tags(mut self, tags: HashSet<String>) -> Self {
        self.tags = tags;
        self
    }
}