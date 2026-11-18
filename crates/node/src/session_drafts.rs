//! Replicated unsent editor state; separate from execution and delivery state.
use super::*;
use khor_sync::drafts::{DraftDoc, DraftRevision, drafts_dir};

impl Node {
    pub fn session_drafts(&self) -> Result<Vec<DraftRevision>, String> {
        Ok(load_pile::<DraftDoc>(&drafts_dir(self.root()), self.peer)?.doc.all())
    }

    pub fn edit_session_draft(&self, revision: &DraftRevision, parents: &[String]) -> Result<Vec<DraftRevision>, String> {
        let mut loaded = load_pile::<DraftDoc>(&drafts_dir(self.root()), self.peer)?;
        loaded.doc.edit(revision, parents)?;
        loaded.store.flush(&loaded.doc)?;
        Ok(loaded.doc.all())
    }
}
