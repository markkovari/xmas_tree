extern crate cronjob;
use cronjob::CronJob;
use std::env::set_current_dir;
use std::fs;
use std::process::Command;
use std::env::current_dir;

fn main() {
    let mut cron = CronJob::new("xmas_tree", on_cron);
    cron.seconds("0");
    cron.minutes("0-59");
    cron.start_job();
}

fn on_cron(_name: &str) {
    set_current_dir("/home/mark/DEV/xmasTree").unwrap();
    println!("{:?}", current_dir().unwrap());
    let mut contents = fs::read_to_string("./README.md")
        .expect("Something went wrong reading the file, maybe you should do it the lame way");
    if contents.len() % 2 == 0 {
        let _result = fs::write("./README.md", format!("{} ", contents));
    } else {
        contents.remove(contents.len() - 1);
        let _result = fs::write("./README.md", contents);
    }

    let add = Command::new("git").arg("add").arg("-A").output().unwrap();
    println!("{:?}", add);
    let commit = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("yeah cheating")
        .output()
        .unwrap();
    println!("{:?}", commit);
    let push = Command::new("git").arg("push").output().unwrap();
    Command::new("cargo").arg("build").output().unwrap();
    Command::new("cp").arg("./target/debug/xmas_tree").arg("./").output().unwrap();

}
