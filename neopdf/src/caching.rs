use std::env;

/// A caching mechanism to store results of previously interpolated computations.
///
/// The cache is enabled by setting the `NEOPDF_ENABLE_CACHE` environment variable.
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

type CacheKey = (String, Vec<u64>);
type Cache = Mutex<HashMap<CacheKey, f64>>;

static PDF_CACHE: Lazy<Cache> = Lazy::new(|| Mutex::new(HashMap::new()));

/// Checks if the cache is enabled via the `NEOPDF_ENABLE_CACHE` environment variable.
fn is_cache_enabled() -> bool {
    env::var("NEOPDF_ENABLE_CACHE").is_ok()
}

/// Caches and retrieves interpolated PDF values.
///
/// If caching is enabled, this function first checks the cache for a pre-computed
/// result. If found, it returns the cached value. Otherwise, it computes the value
/// using the provided function, stores the result in the cache, and then returns it.
///
/// # Arguments
///
/// * `key` - A unique key to identify the cached value.
/// * `f` - A function that computes the value if it's not in the cache.
///
/// # Returns
///
/// The computed or cached value.
pub fn with_cache<F, E>(key: CacheKey, f: F) -> Result<f64, E>
where
    F: FnOnce() -> Result<f64, E>,
{
    if !is_cache_enabled() {
        return f();
    }

    let mut cache = PDF_CACHE.lock().unwrap();
    if let Some(&value) = cache.get(&key) {
        return Ok(value);
    }

    let result = f();
    if let Ok(value) = result {
        cache.insert(key, value);
    }
    result
}
