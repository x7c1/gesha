use crate::default::default;
use indexmap::{IndexMap, IndexSet};
use std::hash::Hash;

pub trait VecPairs<K, V> {
    /// Splits key-value pairs into unique pairs and duplicate pairs.
    ///
    /// Returns a tuple where:
    /// - The first element contains key-value pairs with duplicate keys removed,
    ///   keeping only the first occurrence of each key.
    /// - The second element contains all key-value pairs that were excluded from the first element
    ///   due to having duplicate keys.
    #[allow(clippy::type_complexity)]
    fn partition_dedup_by_key(self) -> (Vec<(K, V)>, Vec<(K, V)>);

    /// Returns a vector of unique keys from the key-value pairs.
    fn dedup_keys(self) -> Vec<K>;
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

    fn dedup_keys(self) -> Vec<K> {
        self.into_iter()
            .fold(default::<IndexSet<K>>(), |mut acc, (k, _)| {
                if !acc.contains(&k) {
                    acc.insert(k);
                }
                acc
            })
            .into_iter()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_duplicated() {
        let input = vec![("a", 1), ("b", 2), ("a", 3), ("c", 4), ("b", 5)];
        let (unique, duplicated) = input.partition_dedup_by_key();
        assert_eq!(unique, vec![("a", 1), ("b", 2), ("c", 4)]);
        assert_eq!(duplicated, vec![("a", 3), ("b", 5)]);
    }
}
