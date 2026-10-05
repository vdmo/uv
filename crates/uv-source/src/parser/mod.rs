//! Recursive-descent parser producing the AST of one source file.

mod angle;
mod consume;
mod expr;
mod file;
mod item;
mod paths;
mod pattern;
mod recovery;
mod state;
mod stmt;
mod types;

pub use expr::{parse_expr, parse_expr_opt, parse_predicate_expr};
pub use file::{parse_file, parse_file_ok, parse_items, ParseFileResult};
pub use item::parse_item;
pub use pattern::parse_pattern;
pub use state::{Parsed, Parser};
pub use stmt::{parse_block, parse_stmt, stmt_span};
pub use types::{make_type_modal_ref, parse_type, parse_type_annot_opt, parse_type_no_union};
