use std::collections::{BTreeSet, HashMap};
use std::hash::Hash;

#[derive(Clone, Debug)]
pub struct LatestMap<Key: Eq + Hash + Clone + Ord, Value> {
    pub(crate) data: HashMap<Key, Value>,
    data_index: BTreeSet<Key>,
}

impl<Key: Eq + Hash + Clone + Ord, Value> Default for LatestMap<Key, Value> {
    fn default() -> Self {
        Self {
            data: HashMap::new(),
            data_index: BTreeSet::new(),
        }
    }
}

impl<Key: Eq + Hash + Clone + Ord, Value> LatestMap<Key, Value> {
    pub fn insert(&mut self, key: Key, value: Value) {
        self.data.insert(key.clone(), value);
        self.data_index.insert(key);
    }
    pub fn get_latest(&self, key: &Key) -> Option<&Value> {
        let target_key = if self.data_index.contains(key) {
            key
        } else {
            let sorted_keys: Vec<&Key> = self.data_index.iter().collect();
            match sorted_keys.binary_search(&key) {
                Ok(_) => key,
                Err(gt_index) => {
                    if gt_index == 0 {
                        return None;
                    }
                    sorted_keys[gt_index - 1]
                }
            }
        };

        self.data.get(target_key)
    }
    pub fn get_last_with_key(&self) -> Option<(&Key, &Value)> {
        let sorted_keys: Vec<&Key> = self.data_index.iter().collect();
        sorted_keys
            .last()
            .and_then(|&key| self.data.get(key).map(|v| (key, v)))
    }

    pub fn get_mut(&mut self, key: &Key) -> Option<&mut Value> {
        self.data.get_mut(key)
    }
    pub fn contains_key(&self, key: &Key) -> bool {
        self.data_index.contains(key)
    }

    pub fn pop_latest(&mut self, key: &Key) -> Option<(Key, Value)> {
        let target_key = if self.data_index.contains(key) {
            key.clone()
        } else {
            let sorted_keys: Vec<&Key> = self.data_index.iter().collect();
            match sorted_keys.binary_search(&key) {
                Ok(_) => key.clone(),
                Err(gt_index) => {
                    if gt_index == 0 {
                        return None;
                    }
                    sorted_keys[gt_index - 1].clone()
                }
            }
        };
        self.data_index.remove(&target_key);
        self.data.remove(&target_key).map(|value|(target_key, value))
    }
}

#[cfg(test)]
mod test {
    use crate::LatestMap;

    #[test]
    fn should_insert() {
        let mut map: LatestMap<i32, i32> = LatestMap::default();
        map.insert(1, 2);
        assert!(map.data_index.contains(&1));
        assert_eq!(map.data.get(&1), Some(&2));
    }

    #[test]
    fn should_contains_key() {
        let mut map: LatestMap<i32, i32> = LatestMap::default();
        map.insert(1, 2);
        assert!(map.contains_key(&1));
        assert!(!map.contains_key(&2));
    }

    #[test]
    fn should_get_mut() {
        let mut map: LatestMap<i32, i32> = LatestMap::default();
        map.insert(1, 2);
        let value = map.get_mut(&1).unwrap();
        *value = 3;

        assert_eq!(map.data.get(&1), Some(&3));
    }

    #[test]
    fn should_get_latest() {
        let mut map: LatestMap<i32, i32> = LatestMap::default();
        map.insert(1, 2);
        map.insert(10, 20);
        map.insert(20, 40);
        map.insert(50, 100);

        assert_eq!(map.get_latest(&0), None);
        assert_eq!(map.get_latest(&1), Some(&2));
        assert_eq!(map.get_latest(&3), Some(&2));
        assert_eq!(map.get_latest(&20), Some(&40));
        assert_eq!(map.get_latest(&24), Some(&40));
        assert_eq!(map.get_latest(&1000), Some(&100));
    }

    #[test]
    fn should_work_given_map_is_empty() {
        let map: LatestMap<i32, i32> = LatestMap::default();
        assert_eq!(map.get_latest(&0), None);
        assert_eq!(map.get_latest(&1), None);
        assert_eq!(map.get_latest(&3), None);
    }

    #[test]
    fn should_work_pop_latest() {
        let mut map: LatestMap<i32, i32> = LatestMap::default();
        map.insert(1, 2);
        map.insert(10, 20);
        map.insert(20, 40);
        map.insert(50, 100);

        assert_eq!(map.get_latest(&0), None);
        assert_eq!(map.data.len(), 4);
        assert_eq!(map.get_latest(&1), Some(&2));
        assert_eq!(map.data.len(), 4);
        assert_eq!(map.pop_latest(&3), Some((1,2)));
        assert_eq!(map.data.len(), 3);
        assert_eq!(map.data_index.len(), 3);
    }
}
