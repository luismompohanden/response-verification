use crate::hash::hash;
use crate::{hash_from_hex, serialize_to_cbor};
use ic_certified_map::{labeled, labeled_hash, AsHashTree, Hash, HashTree, RbTree};
use ic_crypto_tree_hash::Digest;

const LABEL_ASSETS: &[u8] = b"http_assets";

pub struct AssetTree {
    tree: RbTree<&'static str, Hash>,
}

impl Default for AssetTree {
    fn default() -> Self {
        let mut asset_tree = Self::new();
        let body = "Hello World!";

        asset_tree.insert(Self::DEFAULT_PATH, body);

        asset_tree
    }
}

impl AssetTree {
    pub const DEFAULT_PATH: &'static str = "/";

    pub fn new() -> Self {
        Self {
            tree: RbTree::new(),
        }
    }

    pub fn insert(&mut self, path: &'static str, body: &str) {
        let body_hash = hash(body);

        self.tree.insert(path, body_hash);
    }

    pub fn serialize_to_cbor(&self, path: Option<&'static str>) -> Vec<u8> {
        let path = path.unwrap_or(Self::DEFAULT_PATH);
        let tree = self.tree.witness(path.as_bytes());
        let labeled_tree = labeled(LABEL_ASSETS, tree);

        serialize_to_cbor::<HashTree>(&labeled_tree)
    }

    pub fn get_certified_data(&self) -> Digest {
        let root_hash = self.tree.root_hash();
        let labeled_tree = labeled_hash(LABEL_ASSETS, &root_hash);

        Digest(labeled_tree)
    }
}

pub fn create_certified_data(data: &str) -> Digest {
    let hash = hash_from_hex(data);

    let labeled_tree = labeled_hash(LABEL_ASSETS, &hash);

    Digest(labeled_tree)
}
