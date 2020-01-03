# Changelog

All notable changes to this project will be documented in this file.

## v0.6.0 - unreleased

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
