mod mocha_result;

use std::{
    fmt,
    fs::{canonicalize, File},
    io::prelude::*,
    path::PathBuf,
};

use actix::System;
use actix_files::NamedFile;
use actix_web::{
    dev::Server, web, App, HttpRequest, HttpServer, Result as HttpResult,
};
use fantoccini::{error::CmdError, Client, Locator};
use futures::{
    future::{Either, Loop},
    Future,
};
use serde::Deserialize;
use serde_json::json;
use std::io::stdin;
use webdriver::capabilities::Capabilities;
use yansi::Paint;

use crate::mocha_result::TestResults;

const TESTS_ADDR: &str = "127.0.0.1:8088";

fn index(req: HttpRequest) -> HttpResult<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

fn generate_html(test_js: &str) -> String {
    let dont_edit_warning = "<!--DON'T EDIT THIS FILE. THIS IS AUTOGENERATED \
                             FILE FOR TESTS-->"
        .to_string();
    let html_body =
        include_str!("../test_template.html").replace("{{{}}}", test_js);
    format!("{}\n{}", dont_edit_warning, html_body)
}

fn generate_html_test(test_path: &PathBuf) -> PathBuf {
    let mut file = File::open(test_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let html_test_file_path = test_path.with_extension("html");
    let mut file = File::create(&html_test_file_path).unwrap();
    let test_html = generate_html(&content);
    file.write_all(test_html.as_bytes()).unwrap();
    html_test_file_path
}

fn check_test_results(
    mut client: Client,
    tests: Vec<PathBuf>,
) -> impl Future<Item = Loop<(), (Client, Vec<PathBuf>)>, Error = CmdError> {
    client
        .execute("return console.logs", Vec::new())
        .map(move |e| (e, client))
        .and_then(move |(result, client)| {
            let logs = result.as_array().unwrap();
            for message in logs {
                let message = message.as_array().unwrap()[0].as_str().unwrap();
                if let Ok(test_results) =
                    serde_json::from_str::<TestResults>(message)
                {
                    println!("{}", test_results);
                    if test_results.is_has_error() {
                        return Ok(Loop::Break(()));
                    } else {
                        return Ok(Loop::Continue((client, tests)));
                    }
                }
            }
            for messages in logs {
                let messages = messages.as_array().unwrap();
                for message in messages {
                    let message = message.as_str().unwrap();
                    println!("{}", message);
                }
            }
            panic!("Tests result not found in console logs.");
        })
}

fn wait_for_tests_end(
    client: Client,
) -> impl Future<Item = Client, Error = CmdError> {
    client
        .wait_for_find(Locator::Id("test-end"))
        .map(|e| e.client())
}

fn get_url_to_test(test_path: PathBuf) -> String {
    let filename = test_path.file_name().unwrap().to_str().unwrap();
    format!("http://localhost:8088/specs/{}", filename)
}

fn tests_loop(
    client: Client,
    tests: Vec<PathBuf>,
) -> impl Future<Item = (), Error = ()> {
    futures::future::loop_fn((client, tests), |(mut client, mut tests)| {
        if let Some(test) = tests.pop() {
            let test_path = generate_html_test(&test);
            let test_url = get_url_to_test(test_path);
            println!(
                "\nRunning {} test...",
                test.file_name().unwrap().to_str().unwrap()
            );
            Either::A(
                client
                    .goto(&test_url)
                    .and_then(|client| wait_for_tests_end(client))
                    .and_then(|mut client| check_test_results(client, tests)),
            )
        } else {
            Either::B(futures::future::ok(Loop::Break(())))
        }
    })
    .map_err(|e| ())
}

fn run_tests(
    paths_to_tests: Vec<PathBuf>,
    caps: Capabilities,
) -> impl Future<Item = (), Error = ()> {
    Client::with_capabilities("http://localhost:9515", caps)
        .map_err(|_| ())
        .and_then(|client| tests_loop(client, paths_to_tests))
}

fn get_webdriver_capabilities() -> Capabilities {
    let mut capabilities = Capabilities::new();

    let firefox_settings = json!({
        "prefs": {
            "media.navigator.streams.fake": true,
            "security.fileuri.strict_origin_policy": false,
            "media.navigator.permission.disabled": true,
            "media.autoplay.enabled": true,
            "media.autoplay.enabled.user-gestures-needed ": false,
            "media.autoplay.ask-permission": false,
            "media.autoplay.default": 0,
        },
        "args": ["--headless"]
    });
    capabilities.insert("moz:firefoxOptions".to_string(), firefox_settings);

    let chrome_settings = json!({
        "args": [
            "--use-fake-device-for-media-stream",
            "--use-fake-ui-for-media-stream",
            "--disable-web-security",
        ]
    });
    capabilities.insert("goog:chromeOptions".to_string(), chrome_settings);

    capabilities
}

fn get_all_tests_paths(path_to_test_dir: PathBuf) -> Vec<PathBuf> {
    let mut tests_paths = Vec::new();
    for entry in std::fs::read_dir(path_to_test_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "js" {
                    tests_paths.push(path);
                }
            }
        }
    }
    tests_paths
}

fn run_http_server() -> Server {
    HttpServer::new(|| App::new().route("{filename:.*}", web::get().to(index)))
        .bind(TESTS_ADDR)
        .unwrap()
        .start()
}

fn get_path_to_tests_from_args() -> PathBuf {
    let path_to_tests = std::env::args().skip(1).next().unwrap();
    let path_to_tests = PathBuf::from(path_to_tests);
    let path_to_tests = canonicalize(path_to_tests).unwrap();
    path_to_tests
}

// TODO: make optional headless
fn main() {
    actix::run(|| {
        let server = run_http_server();
        let path_to_tests = get_path_to_tests_from_args();
        let capabilities = get_webdriver_capabilities();

        if path_to_tests.is_dir() {
            let tests_paths = get_all_tests_paths(path_to_tests);
            run_tests(tests_paths, capabilities.clone())
        } else {
            run_tests(vec![path_to_tests], capabilities)
        }
        .and_then(move |_| server.stop(true))
        .map(|_| System::current().stop())
    });
}
