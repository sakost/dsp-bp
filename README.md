# dsp-bp
This is DysonSphere blueprint parser on rust

It's under development right now but all PRs are welcome

> None of the repo, the tool, nor the repo owner is affiliated with, or sponsored or authorized by, Youthcat Studio or its affiliates.


# Installation and running
```shell
cargo run
```
You could also build this crate to (release) binary via this command
```shell
cargo build --release
```
Right now it's just parses bp.txt file and writes it's data within JSON format to stdout. It also parses some station parameters.

# TODOs
* [x] make correct parsing of blueprint data
* [x] make correct serializing of rust structures into the blueprint data format
* [x] make custom md5 hash algorithm for validating blueprints
* [x] support new versions of buildings format
* [ ] support all buildings formats
    1. * [x] support stations buildings
    2. * [ ] support other buildings (all types of buildings are in [this file](src/entities/building_types.rs))
* [ ] all unit-tests are up to date
* [ ] unit-tests coverage is 100%
* [ ] make some useful stuff
    1. * [ ] maybe it should interchange PLS(Planetary Logistic Station) to ILS(Interstellar Logistic Station) within blueprints
    2. * [ ] maybe other things

# Goals
Well there's no goal of this project, I just wanted to be able to parse Blueprints from this awesome game.

Maybe it would have some strong functionality in the future maybe not. It's just my rust pet project.

# Contributions
Any contributions are welcome

# License
This code is under BSD-4-Clause
