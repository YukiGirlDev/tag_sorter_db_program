use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct TagSearcher {
    recommends: HashMap<String, usize>,
}

impl TagSearcher {
    pub fn check(&mut self, are_tags: &HashSet<String>, entering: &String, in_tags: &HashSet<String>) -> bool {
        if are_tags.is_empty()&&entering.trim().is_empty() {
            return true;
        }
        if !are_tags.is_subset(in_tags) {
            return false
        }

        let mut and_could = false;
        for e in in_tags {
            
            if e.contains(entering) {
                and_could = true;
                match self.recommends.get_mut(e) {
                    Some(count) => *count += 1,
                    None => {
                        if !are_tags.contains(e) {
                            self.recommends.insert(e.clone(), 1);
                        }
                    }
                }
            }
        }
        and_could
    }
    #[allow(clippy::missing_const_for_fn)] // False positive
    pub fn get(self) -> HashMap<String, usize> {
        self.recommends
    }
}
