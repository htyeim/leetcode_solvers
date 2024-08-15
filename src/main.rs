use leetcode_solvers::crawler;
use leetcode_solvers::util::logger;
use log::info;

use std::env;
use std::process;

#[tokio::main]
async fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    dbg!(&config);
    logger::init_logger();
    info!("start test ...");
    match config.cmd.as_str() {
        "all" => crawler::leetcode_local::build_all_problems(100, None).await,
        "solve" => crawler::leetcode_local::deal_solving(config.problem_id).await,
        _ => println!("Something else"),
    }
    info!("end test=============== {}", 4000.0 / 60.0 / 60.0);
}

#[derive(Debug)]
pub struct Config {
    pub cmd: String,
    pub problem_id: i32,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // 第一个参数是程序名，由于无需使用，因此这里直接空调用一次
        args.next();

        let cmd = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a cmd string"),
        };

        let problem_id: i32 = match args.next() {
            Some(arg) => arg.trim().parse().unwrap_or(-1),
            None => -1,
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            cmd,
            problem_id,
            ignore_case,
        })
    }
}
