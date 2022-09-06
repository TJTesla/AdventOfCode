# Advent of code

My work for the advent of code
Starting with year 2021

## 2021
- Main language: Rust 🦀
- Status: Day 12

## Project structure
- The Rust part is structured with modules and submodules
- The project is one cargo project and thus can be run with ```cargo run```
- The ```main.rs``` file specifies which function from which modules should be run to solve the problem
- The year is a module and has a folder
- In the folder for the year-module, there a other modules specified (in ```mod.rs```)
- Those submodules each contain the code for one day