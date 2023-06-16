use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::domain::link_metadata::LinkMetadata;
use crate::app::persistence::{HeapLinkStore, LinkStore};
use crate::domain::err::AppErr;

mod persistence;

pub struct App {
    store: Mutex<Box<dyn LinkStore + Send + Sync>>,
}

impl Default for App {
    fn default() -> Self {
        App {
            store: Mutex::new(Box::new(HeapLinkStore::default()))
        }
    }
}

impl App {
    fn shorten(&mut self, link_metadata: LinkMetadata) -> Result<(), AppErr> {
        let mut store = self.store.lock().unwrap();
        if store.contains(&link_metadata.alias) {
            return Err(AppErr::AliasAlreadyUsed(link_metadata.redirect, link_metadata.alias));
        }

        store.store(link_metadata);

        Ok(())
    }

    fn get_redirect(&self, alias: String) -> Result<String, AppErr> {
        let store = self.store.lock().unwrap();

        store.get(&alias).ok_or(AppErr::AliasNotFound(alias)).map(|metadata| {
            metadata.redirect.clone()
        })
    }
}