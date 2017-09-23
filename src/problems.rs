use std::io;
use std::path::PathBuf;
use std::collections::{HashMap, HashSet};
use read_elmi;
use files;
use cli;
use exposed_tests;
use generate_elm;

#[derive(Debug)]
pub enum Problem {
    MissingElmJson,
    InvalidCwd(io::Error),
    ChDirError(io::Error),
    ReadTestFiles(io::Error),
    NoTestsFound(Vec<PathBuf>),
    UnexposedTests(HashMap<String, HashSet<String>>),
    NoExposedTests(bool),

    WriteGeneratedCode(io::Error),

    // Reading elm.json
    ReadElmJson(files::ElmJsonError),

    // Running elm make
    SpawnElmMake(io::Error),
    CompilationFailed(io::Error),

    // Running node
    SpawnNodeProcess(io::Error),

    // Problems from other modules
    ReadElmi(read_elmi::Problem),
    Cli(cli::Problem),
    ExposedTest(PathBuf, exposed_tests::Problem),
    GenerateElm(generate_elm::Problem),
}
