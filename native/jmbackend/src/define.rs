use jmcomic::Client;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::time::Duration;

use lazy_static::lazy_static;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;
use tokio::sync::MutexGuard;

use crate::types::*;

pub const HASH_LOCK_COUNT: u64 = 64;

lazy_static! {

    pub static ref UA:&'static str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.110 Safari/537.36";

    pub static ref RUNTIME: Runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_keep_alive(Duration::new(60, 0))
        .worker_threads(30)
        .max_blocking_threads(30)
        .build()
        .unwrap();

    pub(crate) static ref CONTEXT: Mutex<BackendContext> =
        Mutex::<BackendContext>::new(BackendContext {
            login: false,
            last_login: 0,
        });

    pub(crate) static ref FIRST_LOGIN: Mutex<bool> = Mutex::new(false);

    pub(crate) static ref CLIENT :Client = Client::new();
}

lazy_static! {
    pub(crate) static ref INITED: Mutex<bool> = Mutex::<bool>::new(false);
    pub(crate) static ref FOLDER: Mutex<String> = Mutex::<String>::new(String::new());
}

lazy_static::lazy_static! {
    static ref HASH_LOCK: Vec<Mutex::<()>> = {
        let mut mutex_vec = vec![];
        for _ in 0..HASH_LOCK_COUNT {
            mutex_vec.push(Mutex::<()>::new(()));
        }
        mutex_vec
    };
}

pub(crate) async fn take_hash_lock(url: String) -> MutexGuard<'static, ()> {
    let mut s = DefaultHasher::new();
    s.write(url.as_bytes());
    HASH_LOCK[(s.finish() % HASH_LOCK_COUNT) as usize]
        .lock()
        .await
}
