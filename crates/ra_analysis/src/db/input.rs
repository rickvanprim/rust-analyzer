use std::{
    sync::Arc,
};

use salsa;
use rustc_hash::FxHashSet;

use crate::{FileId, FileResolverImp, CrateGraph, symbol_index::SymbolIndex};

salsa::query_group! {
    pub(crate) trait FilesDatabase: salsa::Database {
        fn file_text(file_id: FileId) -> Arc<String> {
            type FileTextQuery;
            storage input;
        }
        fn file_source_root(file_id: FileId) -> SourceRootId {
            type FileSourceRootQuery;
            storage input;
        }
        fn source_root(id: SourceRootId) -> Arc<SourceRoot> {
            type SourceRootQuery;
            storage input;
        }
        fn libraries() -> Arc<Vec<SourceRootId>> {
            type LibrarieseQuery;
            storage input;
        }
        fn library_symbols(id: SourceRootId) -> Arc<SymbolIndex> {
            type LibrarySymbolsQuery;
            storage input;
        }
        fn crate_graph() -> Arc<CrateGraph> {
            type CrateGraphQuery;
            storage input;
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct SourceRootId(pub(crate) u32);

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub(crate) struct SourceRoot {
    pub(crate) file_resolver: FileResolverImp,
    pub(crate) files: FxHashSet<FileId>,
}

pub(crate) const WORKSPACE: SourceRootId = SourceRootId(0);
