use std::{
    collections::HashMap,
    hash::Hash,
    sync::Mutex,
    time::{Duration, Instant},
};

pub struct Cache<K, V> {
    map: Mutex<HashMap<K, (V, Instant, Duration)>>,
}

impl<K, V> Cache<K, V>
where
    K: Eq + PartialEq + Hash,
    V: Clone,
{
    pub fn new() -> Self {
        Self {
            map: Mutex::new(HashMap::new()),
        }
    }

    fn cleanup(&self) {
        let right_now = Instant::now();
        let mut map = self.map.lock().unwrap();

        map.retain(|_, (_, timestamp, ttl)| right_now.duration_since(*timestamp) < *ttl);
    }

    pub fn get(&mut self, key: &K) -> Option<V> {
        self.cleanup();

        let map = self.map.lock().unwrap();
        map.get(key).map(|(value, _, _)| value.clone())
    }

    pub fn set(&mut self, key: K, value: V, ttl: Duration) {
        let mut map = self.map.lock().unwrap();
        map.insert(key, (value, Instant::now(), ttl));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_retrieve() {
        let mut cache = Cache::new();

        cache.set("key1", "value1", Duration::from_secs(10));
        cache.set("key2", "value2", Duration::from_secs(10));

        let result1 = cache.get(&"key1");
        let result2 = cache.get(&"key2");

        assert_eq!(result1, Some("value1"));
        assert_eq!(result2, Some("value2"));
    }

    #[test]
    fn test_retrieve_expired_entry() {
        let mut cache = Cache::new();

        cache.set("key1", "value1", Duration::from_secs(1));

        std::thread::sleep(Duration::from_secs(2));

        let result = cache.get(&"key1");

        assert_eq!(result, None);
    }

    #[test]
    fn test_remove_expired_entry() {
        let mut cache = Cache::new();

        cache.set("key1", "value1", Duration::from_secs(1));

        std::thread::sleep(Duration::from_secs(2));

        let result = cache.get(&"key1");

        assert_eq!(result, None);
    }
}
