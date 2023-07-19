use std::sync::{LazyLock, RwLock};
use lru::LruCache;
use std::num::NonZeroUsize;
use serde::{Deserialize, Serialize};
use crate::utils::CowStr;

pub static USER_CACHE: LazyLock<RwLock<LruCache<String, User>>> = LazyLock::new(|| RwLock::new(LruCache::new(NonZeroUsize::new(100).unwrap())));

#[derive(Deserialize, Serialize, Clone)]
pub struct User {
    user_name: CowStr,
    display_name: CowStr,
}

pub async fn get_user(name: String) -> Option<User> {
    let mut cache = USER_CACHE.write().unwrap();

    if let Some(user) = cache.get(&name) {
        return Some(user.clone());
    }

    return None;
}