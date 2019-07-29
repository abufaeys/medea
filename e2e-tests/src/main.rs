//! E2e tests runner.

pub mod mocha_result;
pub mod test_runner;

use std::path::PathBuf;

use actix::System;
use actix_files::NamedFile;
use actix_web::{
    dev::Server, web, App, HttpRequest, HttpServer, Result as HttpResult,
};
use clap::{App as ClapApp, Arg, ArgMatches};
use futures::Future;

use crate::test_runner::TestRunner;
use std::path::Path;

/// HTTP endpoint which return requested file from this dir.
/// Used for loading tests.
#[allow(clippy::needless_pass_by_value)]
fn index(req: HttpRequest) -> HttpResult<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

/// Start server which serve files from this dir.
///
/// WebDriver's browser will go into test files from this server.
///
/// This is needed because restriction for type=module scripts.
fn run_test_files_server(addr: &str) -> Server {
    HttpServer::new(|| App::new().route("{filename:.*}", web::get().to(index)))
        .bind(addr)
        .unwrap()
        .start()
}

/// Returns [`PathBuf`] to test/test dir from clap's [`ArgMatches`].
fn get_path_to_tests_from_args(opts: &ArgMatches) -> PathBuf {
    let mut test_path = std::env::current_dir().unwrap();
    test_path.push("specs");
    if let Some(path_to_test) = opts.value_of("specs_path") {
        test_path.push(path_to_test);
        if !test_path.exists() {
            // TOOD: maybe print message
            panic!("Test '{}' doesn't exist!", path_to_test);
        }
    }
    test_path
}

fn delete_all_tests_htmls(path_test_dir: &Path) {
    for entry in std::fs::read_dir(path_test_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext == "html" {
                std::fs::remove_file(path).unwrap();
            }
        }
    }
}

fn main() {
    let opts = ClapApp::new("e2e-tests-runner")
        .arg(
            Arg::with_name("headless")
                .help("Run tests in headless browser")
                .long("headless"),
        )
        .arg(
            Arg::with_name("specs_path")
                .help("Path to specs/spec")
                .index(1),
        )
        .arg(
            Arg::with_name("tests_files_addr")
                .help("Address for tests files host server")
                .default_value("localhost:9000")
                .long("files-host")
                .short("f"),
        )
        .arg(
            Arg::with_name("webdriver_addr")
                .help("Address to running webdriver")
                .default_value("http://localhost:4444")
                .long("webdriver-addr")
                .short("w"),
        )
        .get_matches();

    actix::run(|| {
        let server =
            run_test_files_server(opts.value_of("tests_files_addr").unwrap());
        let path_to_tests = get_path_to_tests_from_args(&opts);
        TestRunner::run(path_to_tests, &opts)
            .map_err(|e| panic!("{:?}", e))
            .and_then(move |_| server.stop(true))
            .then(move |_| {
                let test_path = get_path_to_tests_from_args(&opts);
                if test_path.is_file() {
                    let test_dir = test_path.parent().unwrap();
                    delete_all_tests_htmls(&test_dir);
                } else {
                    delete_all_tests_htmls(&test_path);
                }
                futures::future::ok(())
            })
            .map(|_| System::current().stop())
    })
    .unwrap();
}
