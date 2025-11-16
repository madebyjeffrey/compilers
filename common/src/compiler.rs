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
}