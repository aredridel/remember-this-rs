use tokio;
use remember_this::*;

#[tokio::main]
async fn main() {
    // A cache manager lets us organize several strongly-typed caches.
    let manager_options = CacheManagerOptions::builder()
        .path("/tmp/test_reopen.db")
        .build();
    let manager = CacheManager::new(&manager_options).unwrap();

    // An individual cache is strongly-typed.
    //
    // With the default policy, items remain 1h in memory and 1d on disk,
    // this should be sufficient for our test.
    let cache_options = CacheOptions::default();
    let cache = manager.cache("my_cache", &cache_options).unwrap();

    // Fill the cache. Imagine that it's actually a long computation.
    //
    // Recall that async blocks are computed lazily in Rust, so `async { i * i }`
    // will only execute if it's not in the cache yet.
    for i in 0..100 {
        let obtained = cache.get_or_insert_infallible(&i, async { i * i }).await.unwrap();
        assert_eq!(*obtained, i * i);
    }

    // Let's refetch from the cache.
    for i in 0..100 {
        // Here, the async blocks are actually not executed.
        let obtained = cache.get_or_insert_infallible(&i, async {
            panic!("We shouldn't reach this point");
        }).await.unwrap();
        assert_eq!(*obtained, i * i);
    }
}
