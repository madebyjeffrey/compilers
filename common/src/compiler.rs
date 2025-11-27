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
pub struct NoCodegen;

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

pub trait Codegen<Production> {
    type IrProduction;

    fn codegen(&self, production: Production) -> Self::IrProduction;
}

impl<F, Production, IrProduction> Codegen<Production> for F
where
    F: Fn(Production) -> IrProduction,
{
    type IrProduction = IrProduction;
    fn codegen(&self, production: Production) -> Self::IrProduction {
        (self)(production)
    }
}

fn is_debug(final_phase: Phase, phase: Phase, debug: bool) -> bool {
    phase == final_phase && debug
}

pub struct Compiler<L, P, C> {
    lexer: L,
    parser: P,
    codegen: C,
    upto_phase: Phase,
    debug: bool,
    main: Option<SourceFile>,
}

impl Compiler<NoLexer, NoParser, NoCodegen> {
    pub fn new(upto_phase: Phase, debug: bool) -> Self {
        Self {
            lexer: NoLexer,
            parser: NoParser,
            codegen: NoCodegen,
            upto_phase,
            debug, 
            main: None,
        }
    }
}

impl<L0, P0, C0> Compiler<L0, P0, C0> {
    pub fn with_lexer<L>(self, lexer: L) -> Compiler<L, P0, C0> {
        let Self { parser, codegen, upto_phase, debug, main, .. } = self;

        Compiler { lexer, parser, codegen, upto_phase, debug, main }
    }

    pub fn with_parser<P>(self, parser: P) -> Compiler<L0, P, C0> {
        let Self { lexer, codegen, upto_phase, debug, main, .. } = self;

        Compiler { lexer, parser, codegen, upto_phase, debug, main }
    }
    
    pub fn with_codegen<C>(self, codegen: C) -> Compiler<L0, P0, C> {
        let Self { lexer, parser, upto_phase, main, debug, .. } = self;
        
        Compiler { lexer, parser, codegen, upto_phase, debug, main }
    }
}

impl<L, P, C> Compiler<L, P, C>
where L: Lexer {
    pub fn lex(&self) -> Option<Vec<L::Token>> {
        self.main.as_ref().and_then(|main| self.lexer.lex(main))
    }
}

impl<L, P, C> Compiler<L, P, C>
where L: Lexer,
      P: Parser<L::Token> {
    pub fn parse(&mut self, tokens: Vec<L::Token>) -> Option<P::Production> {
        self.main.as_ref().and_then(|main| self.parser.parse(main, tokens))
    }
}

impl<L, P, C> Compiler<L, P, C>
where L: Lexer,
      P: Parser<L::Token>,
      C: Codegen<P::Production>
{
    pub fn codegen(&mut self, production: P::Production) -> C::IrProduction {
        self.codegen.codegen(production)
    }
}

impl<L, P, C> Compiler<L, P, C> {
    pub fn load_main(&mut self, file_path: &str) -> Result<(), ExitCode> {
        let main = SourceFile::from_file(Id::Main, file_path)
            .inspect_err(|err| eprintln!("Couldn't read file: {}", err))
            .or(Err(ExitCode::FAILURE))?;

        self.main = Some(main);

        Ok(())
    }

    pub fn should_stop(&self, phase: Phase) -> bool {
        self.upto_phase == phase
    }
}
