use std::{collections::HashSet, ops::Range};

use rocket::time::Date;
use serde::{Deserialize, Serialize};

use crate::{item::Item, search::from_number};

#[derive(Deserialize, Default, Serialize, Debug)]
pub struct Filter {
    pub(super) id: Option<usize>,
    pub(super) id_range: Option<FilterRange<usize>>,
    pub(super) name: Option<String>,
    pub(super) start_name: Option<String>,
    pub(super) contains_name: Option<String>,
    pub(super) date: Option<FilterDate>,
    pub(super) date_range: Option<FilterRange<FilterDate>>,
    pub(super) tag: Option<HashSet<String>>,
    pub(super) tag_sub: Option<HashSet<String>>,
    pub(super) tag_super: Option<HashSet<String>>,
}
impl Filter {
    pub fn run(self, db: &mut [Item]) -> Vec<&mut Item> {
        db.iter_mut().filter(|item| self.check(item)).collect()
    }
    pub fn check(&self, item: &Item) -> bool {
        // God I wish is_none_and was a thing
        self.id.map_or(true, |x| x == item.id)
            && self
                .id_range
                .map_or(true, |x| Range::from(x).contains(&item.id))
            && self.name.as_ref().map_or(true, |name| {
                item.path
                    .file_name()
                    .and_then(std::ffi::OsStr::to_str)
                    .map_or(false, |x| x == name)
            })
            && self.start_name.as_ref().map_or(true, |name| {
                item.path
                    .file_name()
                    .and_then(std::ffi::OsStr::to_str)
                    .map_or(false, |x| x.starts_with(name))
            })
            && self.contains_name.as_ref().map_or(true, |name| {
                item.path
                    .file_name()
                    .and_then(std::ffi::OsStr::to_str)
                    .map_or(false, |x| x.contains(name))
            })
            && self.date.map_or(true, |date| {
                FilterDate {
                    year: item.year,
                    month: item.month,
                    day: item.day,
                } == date
            })
            && self.date_range.map_or(true, |date_range| {
                let a: Result<bool, rocket::time::error::ComponentRange> = (|| {
                    Ok(
                        (Date::try_from(date_range.start)?..Date::try_from(date_range.end)?)
                            .contains(&Date::from_calendar_date(
                                item.year,
                                from_number(item.month),
                                item.day,
                            )?),
                    )
                })();
                a.unwrap_or(false)
            })
            && self.tag.as_ref().map_or(true, |tags| tags == &item.tags)
            && self
                .tag_sub
                .as_ref()
                .map_or(true, |tags| tags.is_subset(&item.tags))
            && self
                .tag_super
                .as_ref()
                .map_or(true, |tags| tags.is_superset(&item.tags))
    }
}
#[cfg(test)]
#[allow(clippy::missing_const_for_fn)]
impl Filter {
    pub fn id(mut self, id: usize) -> Self {
        self.id = Some(id);
        self
    }
    pub fn id_range(mut self, id1: usize, id2: usize) -> Self {
        self.id_range = Some(FilterRange {
            start: id1,
            end: id2,
        });
        self
    }
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
    pub fn start_name(mut self, name: String) -> Self {
        self.start_name = Some(name);
        self
    }
    pub fn contains_name(mut self, contains: String) -> Self {
        self.contains_name = Some(contains);
        self
    }
    pub fn date(mut self, date: FilterDate) -> Self {
        self.date = Some(date);
        self
    }
    pub fn date_range(mut self, date1: FilterDate, date2: FilterDate) -> Self {
        self.date_range = Some(FilterRange {
            start: date1,
            end: date2,
        });
        self
    }
    pub fn tag(mut self, tags: HashSet<String>) -> Self {
        self.tag = Some(tags);
        self
    }
    pub fn tag_sub(mut self, tags: HashSet<String>) -> Self {
        self.tag_sub = Some(tags);
        self
    }
    pub fn tag_super(mut self, tags: HashSet<String>) -> Self {
        self.tag_super = Some(tags);
        self
    }
}
#[derive(Deserialize, Clone, Serialize, Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct FilterRange<T: Clone> {
    start: T,
    end: T,
}
impl<T: Clone + Copy> std::marker::Copy for FilterRange<T> {}

impl<T: Clone> From<FilterRange<T>> for Range<T> {
    fn from(val: FilterRange<T>) -> Self {
        val.start..val.end
    }
}
#[derive(Deserialize, Clone, Copy, PartialEq, Eq, Serialize, Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct FilterDate {
    pub(super) year: i32,
    pub(super) month: u8,
    pub(super) day: u8,
}

impl TryFrom<FilterDate> for Date {
    type Error = rocket::time::error::ComponentRange;

    fn try_from(value: FilterDate) -> Result<Self, Self::Error> {
        Self::from_calendar_date(value.year, from_number(value.month), value.day)
    }
}
