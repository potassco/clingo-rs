use super::theory::*;
use super::{Control, Model, Options, Statistics, Symbol};
use clingo_dl_sys::*;
use std::ptr::NonNull;

#[derive(Debug, Copy, Clone)]
pub struct DLTheory {
    theory: NonNull<clingodl_theory>,
}
impl Theory for DLTheory {
    /// creates the theory
    fn create() -> Self {
        let mut theory_ptr = std::ptr::null_mut();
        unsafe { clingodl_create(&mut theory_ptr) };
        match NonNull::new(theory_ptr) {
            Some(theory) => DLTheory { theory },
            None => panic!("Tried creating NonNull from a null pointer."),
        }
    }
    /// registers the theory with the control
    fn register(&mut self, ctl: &mut Control) -> bool {
        unsafe { clingodl_register(self.theory.as_ptr(), ctl.ctl.as_ptr()) }
    }
    /// prepare the theory between grounding and solving
    fn prepare(&mut self, ctl: &mut Control) -> bool {
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
    /// initialize index so that it can be used with clingodl_assignment_next
    /// does not throw
    fn assignment_begin(&mut self, thread_id: u32, index: &mut usize) {
        unsafe { clingodl_assignment_begin(self.theory.as_ptr(), thread_id, index) }
    }
    /// move to the next index that has a value
    /// returns true if the updated index is valid
    /// does not throw
    fn assignment_next(&mut self, thread_id: u32, index: &mut usize) -> bool {
        unsafe { clingodl_assignment_next(self.theory.as_ptr(), thread_id, index) }
    }
    /// check if the symbol at the given index has a value
    /// does not throw
    fn assignment_has_value(&mut self, thread_id: u32, index: usize) -> bool {
        unsafe { clingodl_assignment_has_value(self.theory.as_ptr(), thread_id, index) }
    }
    /// get the symbol and it's value at the given index
    /// does not throw
    fn assignment_get_value(&mut self, thread_id: u32, index: usize) -> TheoryValue {
        let value_internal = clingodl_value__bindgen_ty_1 { int_number: 0 };
        let mut value = clingodl_value {
            type_: 0,
            __bindgen_anon_1: value_internal,
        };
        unsafe { clingodl_assignment_get_value(self.theory.as_ptr(), thread_id, index, &mut value) };

        let out_value_internal = value__bindgen_ty_1 { int_number: unsafe {value.__bindgen_anon_1.int_number} };
        let out_value = value {
            type_: value.type_,
            __bindgen_anon_1: out_value_internal
        };
        TheoryValue(out_value)
    }
    /// configure theory manually (without using clingo's options facility)
    /// Note that the theory has to be configured before registering it and cannot be reconfigured.
    fn configure(&mut self, key: &str, value: &str) -> bool {
        unsafe { clingodl_configure(self.theory.as_ptr(), key.as_ptr()  as *const i8, value.as_ptr() as *const i8) }
    }
}