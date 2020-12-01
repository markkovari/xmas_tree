extern crate cronjob;
use cronjob::CronJob;
use std::fs;
use std::process::Command;

fn main() {
    let mut cron = CronJob::new("xmas_tree", on_cron);
    cron.seconds("0");
    cron.minutes("0");
    cron.hours("8-16");
    cron.start_job();
}

fn on_cron(_name: &str) {
    let mut contents = fs::read_to_string("./README.md")
        .expect("Something went wrong reading the file, maybe you should do it the lame way");
    if contents.len() % 2 == 0 {
        let _result = fs::write("./README.md", format!("{} ", contents));
        if _result.is_ok() {
            println!("updated with a newchar")
        }
    } else {
        contents.remove(contents.len() - 1);
        let _result = fs::write("./README.md", contents);
        if _result.is_ok() {
            println!("updated with a removed char")
        }
    }

    let mut _git_add_result = Command::new("git").arg("add").arg("-A").output().unwrap();
    println!("{:?}", _git_add_result);
    let mut _git_commit_result = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("yeah cheating")
        .output()
        .unwrap();
        println!("{:?}", _git_commit_result);

    let mut _git_push_result = Command::new("git").arg("push").output().unwrap();
    println!("{:?}", _git_push_result);

}
