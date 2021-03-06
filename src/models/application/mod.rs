extern crate git2;
extern crate scribe;
extern crate rustbox;

pub mod modes;
mod clipboard;

// Published API
pub use self::clipboard::ClipboardContent;

use std::env;
use std::path::PathBuf;
use self::modes::jump::JumpMode;
use self::modes::line_jump::LineJumpMode;
use self::modes::symbol_jump::SymbolJumpMode;
use self::modes::insert::InsertMode;
use self::modes::open::OpenMode;
use self::modes::select::SelectMode;
use self::modes::select_line::SelectLineMode;
use self::modes::search_insert::SearchInsertMode;
use scribe::{Buffer, Workspace};
use view::View;
use self::clipboard::Clipboard;
use self::git2::Repository;

pub enum Mode {
    Normal,
    Insert(InsertMode),
    Jump(JumpMode),
    LineJump(LineJumpMode),
    SymbolJump(SymbolJumpMode),
    Open(OpenMode),
    Select(SelectMode),
    SelectLine(SelectLineMode),
    SearchInsert(SearchInsertMode),
    Exit,
}

pub struct Application {
    pub mode: Mode,
    pub workspace: Workspace,
    pub search_query: Option<String>,
    pub view: View,
    pub clipboard: Clipboard,
    pub repository: Option<Repository>,
}

pub fn new() -> Application {
    // Set up a workspace in the current directory.
    let mut workspace = match env::current_dir() {
        Ok(path) => Workspace::new(path),
        Err(_) => panic!("Could not initialize workspace to the current directory."),
    };

    // Try to open the specified file.
    // TODO: Handle non-existent files as new empty buffers.
    for path in env::args().skip(1) {
        let argument_buffer = match Buffer::from_file(PathBuf::from(path.clone())) {
            Ok(buf) => buf,
            Err(_) => panic!("Ran into an error trying to open {}.", path),
        };

        workspace.add_buffer(argument_buffer);
    }

    let view = View::new();
    let clipboard = Clipboard::new();

    // Try to initialize a repository in the working directory.
    let repo = match Repository::open(&workspace.path) {
        Ok(repo) => Some(repo),
        Err(_) => None,
    };

    Application {
        mode: Mode::Normal,
        workspace: workspace,
        search_query: None,
        view: view,
        clipboard: clipboard,
        repository: repo,
    }
}
