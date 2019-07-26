use std::{fs::File, io::prelude::*, path::PathBuf};

use clap::ArgMatches;
use fantoccini::{error::CmdError, Client, Locator};
use futures::{
    future::{Either, Loop},
    Future,
};
use serde_json::json;
use webdriver::capabilities::Capabilities;

use crate::mocha_result::TestResults;

pub struct TestRunner {
    tests: Vec<PathBuf>,
    test_addr: String,
}

impl TestRunner {
    pub fn run(
        path_to_tests: PathBuf,
        opts: ArgMatches,
    ) -> impl Future<Item = (), Error = ()> {
        if path_to_tests.is_dir() {
            let tests_paths = get_all_tests_paths(path_to_tests);
            let runner = TestRunner {
                tests: tests_paths,
                test_addr: opts
                    .value_of("tests_files_addr")
                    .unwrap()
                    .to_string(),
            };
            runner.run_tests(&opts)
        } else {
            let runner = TestRunner {
                tests: vec![path_to_tests],
                test_addr: opts
                    .value_of("tests_files_addr")
                    .unwrap()
                    .to_string(),
            };
            runner.run_tests(&opts)
        }
    }

    fn run_tests(
        self,
        opts: &ArgMatches,
    ) -> impl Future<Item = (), Error = ()> {
        let caps = get_webdriver_capabilities(opts);
        Client::with_capabilities(
            opts.value_of("webdriver_addr").unwrap(),
            caps,
        )
        .map_err(|e| panic!("Client session start error: {:?}", e))
        .and_then(|client| self.tests_loop(client))
    }

    fn tests_loop(self, client: Client) -> impl Future<Item = (), Error = ()> {
        futures::future::loop_fn((client, self), |(client, mut runner)| {
            if let Some(test) = runner.tests.pop() {
                let test_path = generate_html_test(&test);
                let test_url = runner.get_url_to_test(test_path);
                println!(
                    "\nRunning {} test...",
                    test.file_name().unwrap().to_str().unwrap()
                );
                Either::A(
                    client
                        .goto(&test_url)
                        .and_then(|client| wait_for_tests_end(client))
                        .and_then(|client| runner.check_test_results(client)),
                )
            } else {
                Either::B(futures::future::ok(Loop::Break(())))
            }
        })
        .map_err(|e| panic!("WebDriver command error: {:?}", e))
    }

    fn check_test_results(
        self,
        mut client: Client,
    ) -> impl Future<Item = Loop<(), (Client, TestRunner)>, Error = CmdError>
    {
        client
            .execute("return console.logs", Vec::new())
            .map(move |e| (e, client))
            .and_then(move |(result, client)| {
                let logs = result.as_array().unwrap();
                for message in logs {
                    let message =
                        message.as_array().unwrap()[0].as_str().unwrap();
                    if let Ok(test_results) =
                        serde_json::from_str::<TestResults>(message)
                    {
                        println!("{}", test_results);
                        if test_results.is_has_error() {
                            return Ok(Loop::Break(()));
                        } else {
                            return Ok(Loop::Continue((client, self)));
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

    fn get_url_to_test(&self, test_path: PathBuf) -> String {
        let filename = test_path.file_name().unwrap().to_str().unwrap();
        format!("http://{}/specs/{}", self.test_addr, filename)
    }
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

fn wait_for_tests_end(
    client: Client,
) -> impl Future<Item = Client, Error = CmdError> {
    client
        .wait_for_find(Locator::Id("test-end"))
        .map(|e| e.client())
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

fn get_webdriver_capabilities(opts: &ArgMatches) -> Capabilities {
    let mut capabilities = Capabilities::new();

    let mut firefox_args = Vec::new();
    let mut chrome_args = vec![
        "--use-fake-device-for-media-stream",
        "--use-fake-ui-for-media-stream",
        "--disable-web-security",
    ];
    if opts.is_present("headless") {
        firefox_args.push("--headless");
        chrome_args.push("--headless");
    }

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
        "args": firefox_args
    });
    capabilities.insert("moz:firefoxOptions".to_string(), firefox_settings);

    let chrome_settings = json!({ "args": chrome_args });
    capabilities.insert("goog:chromeOptions".to_string(), chrome_settings);

    capabilities
}
