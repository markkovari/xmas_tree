extern crate cronjob;
use cronjob::CronJob;
use std::env::set_current_dir;
use std::fs;
use std::process::Command;

fn main() {
    let mut cron = CronJob::new("xmas_tree", on_cron);
    cron.seconds("0");
    cron.minutes("0-59");
    // cron.hours("8-16");
    cron.start_job();
}

fn on_cron(_name: &str) {
    println!("READIND FILE");
    let _file_dir = set_current_dir("/home/mark/DEV/xmasTree").unwrap();
    let mut contents = fs::read_to_string("./README.md")
        .expect("Something went wrong reading the file, maybe you should do it the lame way");
    if contents.len() % 2 == 0 {
        let _result = fs::write("./README.md", format!("{} ", contents));
    } else {
        contents.remove(contents.len() - 1);
        let _result = fs::write("./README.md", contents);
    }

    let mut _git_add_result = Command::new("git").arg("add").arg("-A").output().unwrap();
    let mut _git_commit_result = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("yeah cheating")
        .output()
        .unwrap();
    let mut _git_push_result = Command::new("git").arg("push").output().unwrap();
    println!(
        "achtung-achtung {:?} {:?} {:?}",
        _git_add_result, _git_commit_result, _git_push_result
    );
}
