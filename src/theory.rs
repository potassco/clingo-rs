#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use super::ast;
use super::{Control, Id, Model, Options, Statistics, Symbol};
use std::fmt;

pub trait Theory<'a> {
    type AssignmentIterator: Iterator<Item = (Symbol, TheoryValue)>;
    /// registers the theory with the control
    fn register(&mut self, ctl: &mut Control) -> bool;
    /// Rewrite statements before adding them via the given callback.
    fn rewrite_statement(&mut self, stm: &ast::Ast, builder: &mut ast::ProgramBuilder) -> bool;
    /// prepare the theory between grounding and solving
    fn prepare(&mut self, ctl: &mut Control) -> bool;
    /// add options for your theory
    fn register_options(&mut self, options: &mut Options) -> bool;
    /// validate options for your theory
    fn validate_options(&mut self) -> bool;
    /// callback on every model
    fn on_model(&mut self, model: &mut Model) -> bool;
    /// callback on statistic updates
    /// please add a subkey with the name of your theory
    fn on_statistics(&mut self, step: &mut Statistics, akku: &mut Statistics) -> bool;
    /// obtain a symbol index which can be used to get the value of a symbol
    /// returns true if the symbol exists
    /// does not throw
    fn lookup_symbol(&mut self, symbol: Symbol, index: &mut usize) -> bool;
    /// obtain the symbol at the given index
    /// does not throw
    fn get_symbol(&mut self, index: usize) -> Symbol;
    /// an iterator over the assigned theory values
    fn assignment(&'a self, thread_id: Id) -> Self::AssignmentIterator;
    /// configure theory manually (without using clingo's options facility)
    /// Note that the theory has to be configured before registering it and cannot be reconfigured.
    fn configure(&mut self, key: &str, value: &str) -> bool;
}
#[derive(Copy, Clone, Debug)]
pub enum TheoryValue {
    IntNumber(u64),
    DoubleNumber(f64),
    Symbol(Symbol),
}
impl fmt::Display for TheoryValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TheoryValue::IntNumber(int) => u64::fmt(int, f),
            TheoryValue::DoubleNumber(double) => f64::fmt(double, f),
            TheoryValue::Symbol(sym) => Symbol::fmt(sym, f),
        }
    }
}
