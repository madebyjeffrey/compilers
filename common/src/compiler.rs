use std::process::ExitCode;
use crate::source_file::{Id, SourceFile};

#[derive(Eq, PartialEq, Debug, PartialOrd, Ord)]
pub enum Phase {
    None,
    Lex,
    Parse,
    Codegen,
    FullCompile
}

pub trait HasDebug {
    fn is_debug(&self, phase: Phase) -> bool;
}

pub struct Compiler {
    debug: bool,
    final_phase: Phase,
}

impl<'a> Compiler {
    pub fn new() -> Self {
        Self { debug: false, final_phase: Phase::None }
    }

    pub fn compile(&mut self, upto_phase: Phase, debug: bool, file_path: &str) -> Result<SourceFile, ExitCode> {
        self.final_phase = upto_phase;
        self.debug = debug;

        let main = SourceFile::from_file(Id::Main, file_path)
            .inspect_err(|err| eprintln!("Couldn't read file: {}", err))
            .or(Err(ExitCode::FAILURE))?;

        Ok(main)
    }

    pub fn with_lexer<L, Token>(self, lexer: L) -> WithLexer<L, Token>
        where L: Fn(&SourceFile, &dyn HasDebug) -> Option<Vec<Token>> {
        WithLexer { compiler: self, lexer }
    }
}

impl HasDebug for Compiler {
    fn is_debug(&self, phase: Phase) -> bool {
        phase == self.final_phase && self.debug
    }
}

pub struct WithLexer<L, Token>
    where L: Fn(&SourceFile, &dyn HasDebug) -> Option<Vec<Token>> {
    compiler: Compiler,
    lexer: L,
}

impl<L, Token> WithLexer<L, Token>
    where L: Fn(&SourceFile, &dyn HasDebug) -> Option<Vec<Token>>, Token: Clone {

    pub fn compile(&mut self, upto_phase: Phase, debug: bool, file_path: &str) -> Result<Option<(Vec<Token>, SourceFile)>, ExitCode> {
        let file = self.compiler.compile(upto_phase, debug, file_path)?;

        if self.compiler.final_phase == Phase::None {
            return Ok(None);
        }

        let tokens = (self.lexer)(&file, &self.compiler)
                .ok_or_else(|| ExitCode::FAILURE)?;

        Ok(Some((tokens, file)))
    }

    pub fn with_parser<P, Production>(self, parser: P) -> WithParser<P, L, Token, Production>
        where L: Fn(&SourceFile, &dyn HasDebug) -> Option<Vec<Token>>,
              P: Fn(&SourceFile, &dyn HasDebug, Vec<Token>) -> Option<Production>
    {
        WithParser { lexer: self, parser }
    }
}

pub struct WithParser<P, L, Token, Production>
    where L: Fn(&SourceFile, &dyn HasDebug) -> Option<Vec<Token>>,
          P: Fn(&SourceFile, &dyn HasDebug, Vec<Token>) -> Option<Production>,
          Token: Clone
{
    lexer: WithLexer<L, Token>,
    parser: P,
}

impl<'a, P, L, Token, Production> WithParser<P, L, Token, Production>
where L: Fn(&SourceFile, &dyn HasDebug) -> Option<Vec<Token>>,
      P: Fn(&SourceFile, &dyn HasDebug, Vec<Token>) -> Option<Production>,
      Token: Clone
{
    pub fn compile(&mut self, upto_phase: Phase, debug: bool, file_path: &str) -> Result<Option<Production>, ExitCode> {
        if self.lexer.compiler.final_phase == Phase::Lex {
            return Ok(None);
        }

        if let Some((tokens, file)) = self.lexer.compile(upto_phase, debug, file_path)? {
            let program = (self.parser)(&file, &self.lexer.compiler, tokens)
                .ok_or_else(|| ExitCode::FAILURE)?;

            Ok(Some(program))
        } else {
            Ok(None)
        }
    }
}



/*
pub struct Compiler<Token, Program, L, P>
where
    L: Fn(&SourceFile, &dyn HasDebug) -> Option<Vec<Token>>,
    P: Fn(&SourceFile, &dyn HasDebug, Vec<Token>) -> Option<Program>
{
    lexer: Option<L>,
    parser: Option<P>,
    debug: bool,
    final_phase: Phase,
}

impl<Token, Program, L, P> HasDebug for Compiler<Token, Program, L, P>
    where
    L: Fn(&SourceFile, &dyn HasDebug) -> Option<Vec<Token>>,
    P: Fn(&SourceFile, &dyn HasDebug, Vec<Token>) -> Option<Program> {
    fn is_debug(&self, phase: Phase) -> bool {
        phase == self.final_phase && self.debug
    }
}

enum Empty {}

*/

/*
trait Lexer { .. }
impl<F: Fn(..)> Lexer for F { .. }
impl Lexer for DefaultLexer { .. } // some concrete default type

struct Builder<L> { .. }
impl Builder<DefaultLexer> {
    fn new() -> Self { .. }
}

impl<L> Builder<L> {
    fn lexer<L1: Lexer>(self, lexer: L1) -> Builder<L1> { .. }
}*/

/*
impl<Token, Program, L, P> Compiler<Token, Program, L, P>
where L: Fn(&SourceFile, &dyn HasDebug) -> Option<Vec<Token>>,
      P: Fn(&SourceFile, &dyn HasDebug, Vec<Token>) -> Option<Program>
{
    pub fn new(lexer: Option<L>, parser: Option<P>) -> Self {
        Self {
            lexer,
            parser,
            debug: false,
            final_phase: Phase::Codegen
        }
    }

    pub fn compile(&mut self, upto_phase: Phase, debug: bool, file_path: &str) -> Result<(), ExitCode> {
        self.final_phase = upto_phase;
        self.debug = debug;

        let mut main = SourceFile::from_file(Id::Main, file_path)
            .inspect_err(|err| eprintln!("Couldn't read file: {}", err))
            .or(Err(ExitCode::FAILURE))?;

        if self.final_phase == Phase::None {
            return Ok(());
        }

        let tokens = match &self.lexer {
            None => return Ok(()),
            Some(lexer) => lexer(&mut main, self)
                .ok_or_else(|| ExitCode::FAILURE)?
        };

        if self.final_phase == Phase::Lex {
            return Ok(());
        }

        let program = match &self.parser {
            None => return Ok(()),
            Some(parser) => parser(&mut main, self, tokens)
                .ok_or_else(|| ExitCode::FAILURE)?
        };

        if self.final_phase == Phase::Parse {
            return Ok(());
        }

        // code gen after

        Ok(())
    }
}*/