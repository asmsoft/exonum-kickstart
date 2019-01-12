use exonum::{
    crypto::Hash,
    storage::{ProofMapIndex, Snapshot, Fork},
};

use blockchain::ToHash;

#[derive(Debug)]
pub struct Schema<T> {
    view: T
}

impl<T> Schema<T> {
    pub fn new(snapshot: T) -> Schema<T> {
        Schema { view: snapshot }
    }

    pub fn into_view(self) -> T {
        self.view
    }
}

impl<T> Schema<T> where T: AsRef<Snapshot> {
    pub fn load(&self, key: &str) -> Option<String> {
        self.storage().get(&key.to_hash())
    }

    pub fn has_key(&self, key: &str) -> bool {
        self.storage().contains(&key.to_hash())
    }

    pub fn storage(&self) -> ProofMapIndex<&T, Hash, String> {
        ProofMapIndex::new("{{project_name}}.storage", &self.view)
    }

    pub fn state_hash(&self) -> Vec<Hash> {
        vec![self.storage().merkle_root()]
    }
}

impl<'a> Schema<&'a mut Fork> {
    pub fn store(&mut self, key: &str, val: String) {
        self.storage_mut().put(&key.to_hash(), val)
    }

    pub fn storage_mut(&mut self) -> ProofMapIndex<&mut Fork, Hash, String> {
        ProofMapIndex::new("{{project_name}}.storage", &mut self.view)
    }
}