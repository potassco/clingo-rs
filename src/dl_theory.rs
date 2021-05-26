use super::ast;
use super::theory::*;
use super::{
    FunctionHandler, GenericControl, GroundProgramObserver, Id, Logger, Model, Options, Propagator,
    Statistics, Symbol,
};
use clingo_dl_sys::*;
use clingo_sys::*;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct DLTheory {
    theory: NonNull<clingodl_theory>,
}
impl<'a> DLTheory {
    /// creates the theory
    pub fn create() -> DLTheory {
        let mut theory_ptr = std::ptr::null_mut();
        unsafe { clingodl_create(&mut theory_ptr) };
        match NonNull::new(theory_ptr) {
            Some(theory) => DLTheory { theory },
            None => panic!("Tried creating NonNull from a null pointer."),
        }
    }
}
impl Drop for DLTheory {
    fn drop(&mut self) {
        let success = unsafe { clingodl_destroy(self.theory.as_ptr()) };
        if !success {
            panic!("call clingodl_destroy returned false")
        }
    }
}
/// An iterator over dl theory values.
pub struct DLTheoryAssignment<'a> {
    dl_theory: &'a DLTheory,
    thread_id: Id,
    index: usize,
}
impl<'a> Iterator for DLTheoryAssignment<'a> {
    type Item = (Symbol, TheoryValue);

    fn next(&mut self) -> Option<(Symbol, TheoryValue)> {
        if !unsafe {
            clingodl_assignment_next(
                self.dl_theory.theory.as_ptr(),
                self.thread_id.0,
                &mut self.index,
            )
        } {
            None
        } else if unsafe {
            clingodl_assignment_has_value(
                self.dl_theory.theory.as_ptr(),
                self.thread_id.0,
                self.index,
            )
        } {
            let sym = unsafe { clingodl_get_symbol(self.dl_theory.theory.as_ptr(), self.index) };
            let sym = Symbol(sym);
            let value_internal = clingodl_value__bindgen_ty_1 { int_number: 0 };
            let mut value = clingodl_value {
                type_: 0,
                __bindgen_anon_1: value_internal,
            };
            unsafe {
                clingodl_assignment_get_value(
                    self.dl_theory.theory.as_ptr(),
                    self.thread_id.0,
                    self.index,
                    &mut value,
                )
            };
            match value.type_ {
                0 => Some((
                    sym,
                    TheoryValue::IntNumber(unsafe { value.__bindgen_anon_1.int_number } as u64),
                )),
                1 => Some((
                    sym,
                    TheoryValue::DoubleNumber(unsafe { value.__bindgen_anon_1.double_number }),
                )),
                2 => Some((
                    sym,
                    TheoryValue::Symbol(Symbol(unsafe { value.__bindgen_anon_1.symbol })),
                )),
                x => panic!("unexpected DLTheoryValue {}", x),
            }
        } else {
            None
        }
    }
}
impl<'a> Theory<'a> for DLTheory {
    fn assignment(&'a self, thread_id: Id) -> Box<dyn Iterator<Item = (Symbol, TheoryValue)> + 'a> {
        let mut index = 0;
        unsafe { clingodl_assignment_begin(self.theory.as_ptr(), thread_id.0, &mut index) }
        Box::new(DLTheoryAssignment {
            dl_theory: self,
            thread_id,
            index,
        })
    }
    /// registers the theory with the control
    fn register<L, P, O, F>(&mut self, ctl: &mut GenericControl<L, P, O, F>) -> bool
    where
        L: Logger,
        P: Propagator,
        O: GroundProgramObserver,
        F: FunctionHandler,
    {
        unsafe { clingodl_register(self.theory.as_ptr(), ctl.ctl.as_ptr()) }
    }
    /// Rewrite statements before adding them via the given callback.
    fn rewrite_statement(
        &mut self,
        stm: &ast::Statement,
        builder: &mut ast::ProgramBuilder,
    ) -> bool {
        let add = super::ast::unsafe_program_builder_add;
        unsafe {
            clingodl_rewrite_ast(
                self.theory.as_ptr(),
                stm.ast.0.as_ptr(),
                Some(add),
                (builder.theref as *mut clingo_program_builder) as *mut ::std::os::raw::c_void,
            )
        }
    }
    /// prepare the theory between grounding and solving
    fn prepare<L, P, O, F>(&mut self, ctl: &mut GenericControl<L, P, O, F>) -> bool
    where
        L: Logger,
        P: Propagator,
        O: GroundProgramObserver,
        F: FunctionHandler,
    {
        unsafe { clingodl_prepare(self.theory.as_ptr(), ctl.ctl.as_ptr()) }
    }
    /// add options for your theory
    fn register_options(&mut self, options: &mut Options) -> bool {
        unsafe { clingodl_register_options(self.theory.as_ptr(), &mut options.0) }
    }
    /// validate options for your theory
    fn validate_options(&mut self) -> bool {
        unsafe { clingodl_validate_options(self.theory.as_ptr()) }
    }
    /// callback on every model
    fn on_model(&mut self, model: &mut Model) -> bool {
        unsafe { clingodl_on_model(self.theory.as_ptr(), &mut model.0) }
    }
    /// callback on statistic updates
    /// please add a subkey with the name of your theory
    fn on_statistics(&mut self, step: &mut Statistics, accu: &mut Statistics) -> bool {
        unsafe { clingodl_on_statistics(self.theory.as_ptr(), &mut step.0, &mut accu.0) }
    }
    /// obtain a symbol index which can be used to get the value of a symbol
    /// returns true if the symbol exists
    /// does not throw
    fn lookup_symbol(&mut self, symbol: Symbol, index: &mut usize) -> bool {
        unsafe { clingodl_lookup_symbol(self.theory.as_ptr(), symbol.0, index) }
    }
    /// obtain the symbol at the given index
    /// does not throw
    fn get_symbol(&mut self, index: usize) -> Symbol {
        let sym = unsafe { clingodl_get_symbol(self.theory.as_ptr(), index) };
        Symbol(sym)
    }
    /// configure theory manually (without using clingo's options facility)
    /// Note that the theory has to be configured before registering it and cannot be reconfigured.
    fn configure(&mut self, key: &str, value: &str) -> bool {
        unsafe {
            clingodl_configure(
                self.theory.as_ptr(),
                key.as_ptr() as *const i8,
                value.as_ptr() as *const i8,
            )
        }
    }
}
