use std::env::current_dir;
use std::fs;
use std::path::PathBuf;
use common::compiler::{Compiler, Phase};
use seacat::phases::{run_lexer, run_parser};

static TEST_PATH: &str = "../writing-a-c-compiler-tests/tests/chapter_1";

#[test]
fn invalid_lex() {
    let mut path = PathBuf::from(TEST_PATH);
    path.push("invalid_lex");

    let files = fs::read_dir(path.to_str().unwrap())
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .collect::<Vec<_>>();

    for path in files {
        eprintln!("Testing {:?}", path);
        let mut compiler = Compiler::new(Phase::Lex, false)
            .with_lexer(run_lexer);

        let result = compiler.load_main(path.to_str().unwrap());
        assert!(result.is_ok());
        let tokens = compiler.lex();
        assert!(tokens.is_none());
    }
}

#[test]
fn invalid_parse() {
    let cur = current_dir();
    let mut path = PathBuf::from(TEST_PATH);
    path.push("invalid_parse");

    let files = fs::read_dir(path.to_str().unwrap())
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .collect::<Vec<_>>();

    for path in files {
        println!("Testing {:?}", path);
        let mut compiler = Compiler::new(Phase::Lex, false)
            .with_lexer(run_lexer)
            .with_parser(run_parser);

        let result = compiler.load_main(path.to_str().unwrap());
        assert!(result.is_ok());
        let tokens = compiler.lex();
        assert!(tokens.is_some());
        let program = compiler.parse(tokens.unwrap());
        assert!(program.is_none());
    }
}

#[test]
fn valid_parse() {
    let cur = current_dir();
    let mut path = PathBuf::from(TEST_PATH);
    path.push("valid");

    let files = fs::read_dir(path.to_str().unwrap())
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .collect::<Vec<_>>();

    for path in files {
        println!("Testing {:?}", path);
        let mut compiler = Compiler::new(Phase::Lex, false)
            .with_lexer(run_lexer)
            .with_parser(run_parser);

        let result = compiler.load_main(path.to_str().unwrap());
        assert!(result.is_ok());
        let tokens = compiler.lex();
        assert!(tokens.is_some());
        let program = compiler.parse(tokens.unwrap());
        assert!(program.is_some());
    }
}