use clap::{App, Arg};
use colored::*;
use inkwell::context::Context;

mod ast;
mod compiler;
#[allow(dead_code)]
mod parser;

use compiler::external::stdio;
use compiler::stdlib;
use std::path::Path;

static SRC: &str = "src";

fn main() {
    let matches = App::new("FAANG Compiler")
        .version("1.0")
        .author("Ryan Lee <drdgvhbh@gmail.com>")
        .about("Compiles the FAANG language - A language for leetcoding!")
        .arg(
            Arg::with_name(SRC)
                .value_name("SOURCE")
                .help("The source glass file to compile")
                .required(true),
        )
        .get_matches();

    let source_file_name = matches.value_of(SRC).unwrap();
    let contents = read_to_string(source_file_name);

    let program_parser = parser::ExpressionParser::new();
    let program = program_parser.parse(&contents).unwrap();

    let context = Context::create();
    let module = context.create_module(source_file_name);
    let builder = context.create_builder();

    stdio::add(
        &[
            stdio::Features::PRINTF,
            stdio::Features::FPRINTF,
            stdio::Features::FOPEN,
            stdio::Features::FFLUSH,
        ],
        &context,
        &module,
    );

    stdlib::add(&[stdlib::Features::PRINTLN], &context, &module, &builder);

    let mut compiler = compiler::Compiler::new(&context, &module, &builder);
    let module = compiler.compile(&program).unwrap();
    module.verify().unwrap();

    let file_stem = Path::new(source_file_name).file_stem().unwrap();
    module
        .print_to_file(format!("{}.ll", file_stem.to_string_lossy()))
        .unwrap();
}

fn read_to_string(file_path: &str) -> String {
    match std::fs::read_to_string(file_path) {
        Err(err) => {
            eprintln!("{} {}: {}", "error:".red().bold(), file_path, err);

            std::process::exit(1);
        }
        Ok(contents) => contents,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::OptimizationLevel;
    use pretty_assertions::assert_eq;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_hello_world() {
        let context = Context::create();
        let module = context.create_module("test");
        let builder = context.create_builder();

        let dir = tempdir().unwrap();
        let file_path = dir.path().join("hello_world.txt");
        let file_path_str = file_path.to_str().unwrap();

        stdio::add(
            &[
                stdio::Features::FPRINTF,
                stdio::Features::FOPEN,
                stdio::Features::FFLUSH,
            ],
            &context,
            &module,
        );
        compiler::external::stdio::mock::add_printf(file_path_str, &context, &module, &builder);

        stdlib::add(&[stdlib::Features::PRINTLN], &context, &module, &builder);

        let program_parser = parser::ExpressionParser::new();
        let program = program_parser.parse("println(\"Hello World!\")").unwrap();
        let mut compiler = compiler::Compiler::new(&context, &module, &builder);
        compiler.compile(&program).unwrap();

        module.verify().unwrap();

        let ee = module
            .create_jit_execution_engine(OptimizationLevel::None)
            .unwrap();
        File::create(file_path.clone()).unwrap();

        unsafe {
            let main_fcn = ee
                .get_function::<unsafe extern "C" fn() -> i32>("main")
                .expect("main function should be defined");
            main_fcn.call();
        }

        assert_eq!(
            "Hello World!\n",
            std::fs::read_to_string(file_path_str).unwrap()
        );
    }
}
