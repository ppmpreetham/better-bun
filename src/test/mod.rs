pub mod file;
pub mod parse;
pub mod reporter;
pub mod types;

pub fn test(_cli_test_filter: Option<&str>) {
    // TODO: implement dynamic testing via JSC
    println!(
        "{}",
        owo_colors::OwoColorize::bright_yellow(
            &"dynamic testing (via JSC) is not implemented bro, please wait for it to be implemented!"
        )
    );
    println!("please use momo check to perform static syntax checking in the meantime");
}
