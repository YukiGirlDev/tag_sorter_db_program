use std::{path::PathBuf, collections::HashSet};

use crate::item::Item;

use super::filter::{Filter, FilterDate};

#[test]
fn id() {
    assert!(Filter::default().id(4).check(&Item::default().id(4)));
    assert!(Filter::default()
        .id(usize::MAX)
        .check(&Item::default().id(usize::MAX)));
    assert!(!Filter::default().id(0).check(&Item::default().id(42)));
}
#[test]
fn id_range() {
    assert!(Filter::default()
        .id_range(0, 43)
        .check(&Item::default().id(23)));
    assert!(Filter::default()
        .id_range(52, 64)
        .check(&Item::default().id(54)));
    assert!(!Filter::default()
        .id_range(0, 43)
        .check(&Item::default().id(56)));
}
#[test]
fn name() {
    assert!(Filter::default()
        .name(String::from("test_case"))
        .check(&Item::default().path(PathBuf::from("/test/test_case"))));
    assert!(Filter::default()
        .name(String::from("test_case"))
        .check(&Item::default().path(PathBuf::from("/test/test_case"))));
    assert!(Filter::default()
        .name(String::from("test_2"))
        .check(&Item::default().path(PathBuf::from("/test_2"))));
    assert!(!Filter::default()
        .name(String::from("test_4"))
        .check(&Item::default().path(PathBuf::from("/test_2"))));
}
#[test]
fn name_start() {
    assert!(Filter::default()
        .start_name(String::from("test"))
        .check(&Item::default().path(PathBuf::from("/test/test_case"))));
    assert!(Filter::default()
        .start_name(String::from("case"))
        .check(&Item::default().path(PathBuf::from("/test/case_test"))));
    assert!(Filter::default()
        .start_name(String::from("test"))
        .check(&Item::default().path(PathBuf::from("/test_case"))));
    assert!(!Filter::default()
        .start_name(String::from("case"))
        .check(&Item::default().path(PathBuf::from("/test_case"))));
}
#[test]
fn name_contains() {
    assert!(Filter::default()
        .contains_name(String::from("es"))
        .check(&Item::default().path(PathBuf::from("/test/test_case"))));
    assert!(!Filter::default()
        .contains_name(String::from("es"))
        .check(&Item::default().path(PathBuf::from("/test/case"))));
    assert!(Filter::default()
        .contains_name(String::from("e_"))
        .check(&Item::default().path(PathBuf::from("/test/te_ca"))));
}
#[test]
fn date() {
    assert!(Filter::default()
        .date(FilterDate {
            year: 2001,
            month: 9,
            day: 11
        })
        .check(&Item::default().year(2001).month(9).day(11)));
    assert!(Filter::default()
        .date(FilterDate {
            year: 2022,
            month: 4,
            day: 1
        })
        .check(&Item::default().year(2022).month(4).day(1)));
    assert!(!Filter::default()
        .date(FilterDate {
            year: 1954,
            month: 1,
            day: 14
        })
        .check(&Item::default().year(2003).month(2).day(16)));
}
#[test]
fn date_range() {
    assert!(Filter::default()
        .date_range(
            FilterDate {
                year: 1900,
                month: 1,
                day: 1
            },
            FilterDate {
                year: 2000,
                month: 1,
                day: 1
            }
        )
        .check(&Item::default().year(1950).month(3).day(12)));
    assert!(!Filter::default()
        .date_range(
            FilterDate {
                year: 1900,
                month: 1,
                day: 1
            },
            FilterDate {
                year: 2000,
                month: 1,
                day: 1
            }
        )
        .check(&Item::default().year(2000).month(3).day(15)));
}

#[test]
fn tags() {
    assert!(Filter::default()
        .tag(HashSet::from([
            "test".to_string(),
            "foo".to_string(),
            "bar".to_string()
        ]))
        .check(&Item::default().tags(HashSet::from([
            "foo".to_string(),
            "bar".to_string(),
            "test".to_string()
        ]))));
    assert!(!Filter::default()
        .tag(HashSet::from([
            "testing".to_string(),
            "foo".to_string(),
            "bar".to_string()
        ]))
        .check(&Item::default().tags(HashSet::from([
            "foo".to_string(),
            "bar".to_string(),
            "test".to_string()
        ]))));
    assert!(!Filter::default()
        .tag(HashSet::from([
            "test".to_string(),
            "foo".to_string(),
            "bar".to_string()
        ]))
        .check(&Item::default().tags(HashSet::from([
            "foo".to_string(),
            "baring".to_string(),
            "test".to_string()
        ]))));
}

#[test]
fn test_sub() {
    assert!(Filter::default()
        .tag_sub(HashSet::from([
            "test".to_string(),
            "foo".to_string()
        ]))
        .check(&Item::default().tags(HashSet::from([
            "foo".to_string(),
            "bar".to_string(),
            "test".to_string()
        ]))));
    assert!(!Filter::default()
        .tag_sub(HashSet::from([
            "testing".to_string(),
            "foo".to_string()
        ]))
        .check(&Item::default().tags(HashSet::from([
            "foo".to_string(),
            "bar".to_string(),
            "test".to_string()
        ]))));
}
#[test]
fn test_super() {
    assert!(Filter::default()
        .tag_super(HashSet::from([
            "test".to_string(),
            "foo".to_string(),
            "bar".to_string(),
        ]))
        .check(&Item::default().tags(HashSet::from([
            "foo".to_string(),
            "test".to_string()
        ]))));
    assert!(!Filter::default()
        .tag_super(HashSet::from([
            "test".to_string(),
            "foo".to_string()
        ]))
        .check(&Item::default().tags(HashSet::from([
            "foo".to_string(),
            "test".to_string(),
            "bar".to_string(),
        ]))));
}