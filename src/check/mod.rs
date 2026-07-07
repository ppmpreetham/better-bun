use crate::test::file::find_files;
use crate::test::parse::parse_all_files;
use crate::test::reporter;
use crate::test::types::{TestFailure, TestReport};

use crate::byte_slices;
use owo_colors::OwoColorize;
use parking_lot::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

const SOURCE_EXTENSIONS: &[&[u8]] = byte_slices![
    ".js",
    ".ts",
    ".jsx",
    ".tsx",
    ".cjs",
    ".mjs",
    ".cts",
    ".mts",
];

pub fn check(cli_test_filter: Option<&str>) {
    let files = find_files(".", cli_test_filter, SOURCE_EXTENSIONS);
    let total_files = files.len();

    println!(
        "{}\n",
        format!("Found {total_files} files. Checking them...").bright_black()
    );

    let failed_count = AtomicUsize::new(0);
    let errors = Mutex::new(Vec::new());

    parse_all_files(
        files,
        |_path, _ast| {},
        |path, err| {
            failed_count.fetch_add(1, Ordering::Relaxed);
            errors.lock().push(TestFailure {
                path: path.to_path_buf(),
                error: err.to_string(),
            });
        },
    );

    let total_failed = failed_count.load(Ordering::Relaxed);
    let total_passed = total_files - total_failed;

    let report = TestReport {
        total_files,
        total_failed,
        total_passed,
        failures: errors.into_inner(),
    };

    reporter::ConsoleReporter::print_summary(&report);
}
