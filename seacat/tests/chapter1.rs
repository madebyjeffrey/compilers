use std::fs;
use std::path::PathBuf;
use common::compiler::Compiler;
use seacat::phases::run_lexer;

static test_path: &str = "../writing-a-c-compiler-tests/tests/chapter1";

#[test]
fn invalid_lex() {
    let mut path = PathBuf::from(test_path);
    path.push("invalid_lex");

    let files = fs::read_dir(path.to_str().unwrap())
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path());

//     let compiler = Compiler::new(Some(run_lexer), None);
//
//     for path in files {
//
//     }
// }

