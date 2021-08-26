# Changelog

All notable changes to this project will be documented in this file.

## v0.7.0-dev - Unreleased

- Add Theory trait
- Control::solve() now consumes the control object and returns a SolveHandle
- SolveHandle::close() consumes the solve handle and returns the corresponding Control object
- Properly use solve event data
  - remove SolveEventType
  - add SolveEvent
  - change SolveEventHandler::on_solve_event(...)
- return &'static str for internalized strings in
  - Location::begin_file()
  - Location::end_file()
  - Signature::name()
  - Symbol::name()
  - Symbol::string()
- add FromSymbol trait
- update clingo-sys to 0.5.2
- add functions to PropagateInit
  - PropagateInit::add_literal()
  - PropagateInit::add_weight_constraint()
  - PropagateInit::add_minimize()
  - PropagateInit::propagate()
- remove Control::use_enumeration_assumption() from Control
- add Control::get_enable_enumeration_assumption()
- add Control::set_enable_enumeration_assumption()
- add Control::get_enable_cleanup()
- add Control::set_enable_cleanup()

## v0.6.0 - Jan 3, 2020

### Changed

- refactor ClingoError
- use clingo-sys 0.5.1 which includes clingo via git submodule

## v0.5.0 - Nov 4, 2019

### Changed

- use clingo-sys 0.5.0
- improve API added functions for the AST
- correct lifetime of objects

## v0.4.3 - Jul 10, 2019

### Fixed

- remove optimality check from AllModels Iterator

## v0.4.2 - Jun 28, 2019

### Added

- added the possiblibity to create HeadLiterals ast::HeadLiteral::new()
- added clingo::ToSymbol trait
- added clingo::FactBase
- added method clingo::Control.add_facts(fb:FactBase)
- added Iterators for AllModels and AllOptimalModels
- enable dynamic linking of a shared clingo library

## v0.4.0 - Feb 2, 2019

### Fixed

- removed copy derive for opaque types

### Changed

- use libclingo 5.3.0
- use rust stable

### Added

- more tests
- this CHANGELOG file

## Undocumented versions

- 0.3.1 Apr 11, 2018
- 0.3.0 Apr 11, 2018
- 0.1.0 May 4, 2017
