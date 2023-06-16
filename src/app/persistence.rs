use std::collections::HashMap;
use std::fs::File;
use crate::domain::link_metadata::LinkMetadata;

pub trait LinkStore {
    fn store(&mut self, metadata: LinkMetadata);

    fn get(&self, alias: &String) -> Option<&LinkMetadata>;

    fn contains(&self, alias: &String) -> bool;
}

pub struct HeapLinkStore {
    store: HashMap<String, LinkMetadata>
}

impl LinkStore for HeapLinkStore {
    fn store(&mut self, metadata: LinkMetadata) {
        self.store.insert(metadata.alias.clone(), metadata);
    }

    fn get(&self, alias: &String) -> Option<&LinkMetadata> {
        self.store.get(alias)
    }

    fn contains(&self, alias: &String) -> bool {
        self.store.contains_key(alias)
    }
}

impl Default for HeapLinkStore {
    fn default() -> Self {
        HeapLinkStore {
            store: HashMap::new()
        }
    }
}