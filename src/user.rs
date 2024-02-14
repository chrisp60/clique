use std::{collections::HashSet, sync::Arc};

use rand::Rng;
use tokio::sync::Mutex;
use tracing::{debug_span, info, instrument, trace, Instrument};

const MOTTOS: &[&str] = &[
    "questionable morals",
    "didn't wash their hands",
    "lies about reading",
    "wanted war criminal",
    "best pizza maker in Brazil",
    "barely making it",
    "the underdog",
];

#[derive(Clone, Debug, Default)]
pub struct UserSet(Arc<Mutex<HashSet<User>>>);

impl UserSet {
    pub fn new() -> Self {
        Self::default()
    }

    #[instrument(skip(self))]
    pub async fn create_one(&self) -> User {
        let lock_fut = self.0.lock();
        let mut lock = lock_fut
            .await
            .instrument(debug_span!("create_user_lock"))
            .into_inner();
        let count = lock.len();
        let user = User::new(count);
        lock.insert(user.clone());
        info!(user.id = user.id, "created user");
        user
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct User {
    id: usize,
    motto: &'static str,
}

impl User {
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn motto(&self) -> &str {
        self.motto
    }

    fn new(id: usize) -> Self {
        Self {
            id,
            motto: get_motto(),
        }
    }
}

fn get_motto() -> &'static str {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..MOTTOS.len());
    let msg = MOTTOS[n];
    trace!("picked motto with rng {n}: {msg:.30}");
    msg
}
