use crate::default;
use indexmap::IndexMap;
use std::hash::Hash;

pub trait VecPairs<K, V> {
    /// Splits key-value pairs into unique pairs and duplicate pairs.
    ///
    /// Returns a tuple where:
    /// - The first element contains key-value pairs with duplicate keys removed,
    ///   keeping only the first occurrence of each key.
    /// - The second element contains all key-value pairs that were excluded from the first element
    ///   due to having duplicate keys.
    fn partition_dedup_by_key(self) -> (Vec<(K, V)>, Vec<(K, V)>);
}

impl<K, V> VecPairs<K, V> for Vec<(K, V)>
where
    K: PartialEq + Eq + Hash,
{
    fn partition_dedup_by_key(self) -> (Vec<(K, V)>, Vec<(K, V)>) {
        let separate = |acc, (key, value)| {
            let (mut unique, mut duplicated): (IndexMap<K, V>, Vec<(K, V)>) = acc;
            if unique.get(&key).is_none() {
                unique.insert(key, value);
            } else {
                duplicated.push((key, value));
            }
            (unique, duplicated)
        };
        let (unique_map, duplicated) = self.into_iter().fold(default(), separate);
        let unique = unique_map.into_iter().collect();
        (unique, duplicated)
    }
}
