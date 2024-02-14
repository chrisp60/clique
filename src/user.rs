use std::{collections::HashSet, sync::Arc};

use rand::Rng;
use tokio::sync::Mutex;

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

    pub async fn create_one(&self) -> User {
        let mut lock = self.0.lock().await;
        let count = lock.len();
        let user = User::new(count);
        lock.insert(user.clone());
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
    let index = rng.gen_range(0..MOTTOS.len());
    MOTTOS[index]
}
