extern crate cronjob;
use cronjob::CronJob;
use std::process::Command;

fn main() {
    let mut cron = CronJob::new("xmas_tree", on_cron);
    cron.seconds("0");
    cron.minutes("0-59");
    cron.start_job();
}

fn on_cron(name: &str) {
    let mut output = Command::new("date");
    let hello = output.output();
    println!("{:?}: xmas_tree booster {}", hello, name);
}
