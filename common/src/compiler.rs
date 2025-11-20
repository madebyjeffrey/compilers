use std::process::ExitCode;
use crate::source_file::{Id, SourceFile};

#[derive(Eq, PartialEq, Debug, PartialOrd, Ord, Copy, Clone)]
pub enum Phase {
    None,
    Lex,
    Parse,
    Codegen,
    FullCompile
}

pub struct NoLexer;
pub struct NoParser;

pub trait Lexer {
    type Token: Clone;
    fn lex(&self, file: &SourceFile) -> Option<Vec<Self::Token>>;
}

impl<F, Token: Clone> Lexer for F
where
    F: Fn(&SourceFile) -> Option<Vec<Token>>,
{
    type Token = Token;

    fn lex(&self, file: &SourceFile) -> Option<Vec<Self::Token>> {
        (self)(file)
    }
}

pub trait Parser<Token> {
    type Production;

    fn parse(
        &self, file: &SourceFile, tokens: Vec<Token>) -> Option<Self::Production>;
}

impl<F, Token, Production> Parser<Token> for F
where
    F: Fn(&SourceFile, Vec<Token>) -> Option<Production>,
{
    type Production = Production;
    fn parse(&self, file: &SourceFile, tokens: Vec<Token>) -> Option<Self::Production> {
        (self)(file, tokens)
    }
}

fn is_debug(final_phase: Phase, phase: Phase, debug: bool) -> bool {
    phase == final_phase && debug
}

pub struct Compiler<L, P> {
    lexer: L,
    parser: P,
    upto_phase: Phase,
    debug: bool,
    main: Option<SourceFile>,
}

impl Compiler<NoLexer, NoParser> {
    pub fn new(upto_phase: Phase, debug: bool) -> Self {
        Self {
            lexer: NoLexer,
            parser: NoParser,
            upto_phase,
            debug, 
            main: None,
        }
    }
}

impl<L0, P0> Compiler<L0, P0> {
    pub fn with_lexer<L>(self, lexer: L) -> Compiler<L, P0> {
        let Self { parser, upto_phase, debug, main, .. } = self;

        Compiler { lexer, parser, upto_phase, debug, main }
    }

    pub fn with_parser<P>(self, parser: P) -> Compiler<L0, P> {
        let Self { lexer, upto_phase, debug, main, .. } = self;

        Compiler { lexer, parser, upto_phase, debug, main }
    }
}

impl<L, P> Compiler<L, P>
where L: Lexer {
    pub fn lex(&self) -> Option<Vec<L::Token>> {
        self.main.as_ref().and_then(|main| self.lexer.lex(main))
    }
}

impl<L, P> Compiler<L, P>
where L: Lexer,
      P: Parser<L::Token> {
    pub fn parse(&mut self, tokens: Vec<L::Token>) -> Option<P::Production> {
        self.main.as_ref().and_then(|main| self.parser.parse(main, tokens))
    }
}

impl<L, P> Compiler<L, P> {
    pub fn load_main(&mut self, file_path: &str) -> Result<(), ExitCode> {
        let main = SourceFile::from_file(Id::Main, file_path)
            .inspect_err(|err| eprintln!("Couldn't read file: {}", err))
            .or(Err(ExitCode::FAILURE))?;

        self.main = Some(main);

        Ok(())
    }

    pub fn should_stop(&self, phase: Phase) -> Result<(), ExitCode> {
        if self.upto_phase == phase {
            Err(ExitCode::SUCCESS)
        } else {
            Ok(())
        }
    }
}


// impl<L> Compiler<L, NoParser>
// where L: Lexer {
//         pub fn lex(&self, upto_phase: Phase, debug: bool, file_path: &str) -> Result<Option<Vec<L::Token>>, ExitCode> {
//             let main = self.load_main(upto_phase, debug, file_path)?;
// 
//             if upto_phase == Phase::None {
//                 return Ok(None);
//             }
// 
//             let tokens = self.lexer.lex(&main)
//                     .ok_or_else(|| ExitCode::FAILURE)?;
// 
//             if (is_debug(upto_phase, Phase::Lex, debug)) {
//                 // output some representation of the tokens
//             }
// 
//             Ok(Some(tokens))
//         }
// }
// 
// impl<L, P> Compiler<L, P>
// where L: Lexer,
//       P: Parser<L::Token>{
//     pub fn parse_with_main(&self, upto_phase: Phase, debug: bool, file_path: &str) -> Result<Option<P::Production>, ExitCode> {
//         let tokens = self.lex_with_main(upto_phase, debug, file_path)?;
// 
//         if upto_phase == Phase::Lex {
//             return Ok(None);
//         }
// 
// 
//         let program = self.parser.parse(&main, tokens)
//             .ok_or_else(|| ExitCode::FAILURE)?;
// 
//         if (is_debug(upto_phase, Phase::Lex, debug)) {
//             // output some representation of the production token
//         }
// 
//         Ok(Some(program))
//     }
// }