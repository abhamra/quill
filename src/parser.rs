// TRY JUST PARSING THE QREG STUFF OR SOMETHING
// or all of the variable instantiations to make sure that the correct values are assigned
// correctly

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct QuillParser;

// Note that we should try to create an AST with Rust Enums and then use pattern matching and
// destructuring to correctly parse it



