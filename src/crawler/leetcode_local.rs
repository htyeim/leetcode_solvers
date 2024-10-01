use super::leetcode_problems::{get_problem, get_problem_by_id, get_problems};
use super::leetcode_problems::{CodeDefinition, Problem, StatWithStatus};

use html2md::parse_html;

use regex::Regex;

use std::fs::File;

use std::thread;
use std::time::Duration;

use log::{info, warn};

use std::fs;
use std::io;
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};

use std::sync::{Arc, Mutex};

const ROOT_PATH: &str = "./";

pub async fn build_all_problems(expect_count: i32, id: Option<u32>) {
    let problems = get_problems().await;
    let mod_file_addon = Arc::new(Mutex::new(vec![]));
    let initialized_ids = get_initialized_ids();
    info!("initialized_ids: {:?}", initialized_ids.len());
    info!("problems size: {}", problems.stat_status_pairs.len());
    let mut count = 0;
    for problem_stat in problems.stat_status_pairs.iter() {
        if initialized_ids.contains(&problem_stat.stat.frontend_question_id) {
            continue;
        }
        if let Some(target_id) = id {
            if problem_stat.stat.frontend_question_id != target_id {
                continue;
            }
        }
        // 0594 solution fn
        info!("problem id: {}", problem_stat.stat.frontend_question_id);

        thread::sleep(Duration::from_millis(10));

        if build_one_problem(problem_stat, &mod_file_addon).await {
            count += 1;
        }
        if count > expect_count {
            break;
        }
    }
    let mut lib_file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("./src/problem/mod.rs")
        .unwrap();
    writeln!(lib_file, "{}", mod_file_addon.lock().unwrap().join("\n")).unwrap();

    info!("mod_file_addon: {}", mod_file_addon.lock().unwrap().len());
}

fn get_initialized_ids() -> Vec<u32> {
    let problem_mod_rs = format!("{}./src/problem/mod.rs", ROOT_PATH);
    let content = fs::read_to_string(problem_mod_rs).unwrap();
    let id_pattern = Regex::new(r"p(\d{4})_").unwrap();
    id_pattern
        .captures_iter(&content)
        .map(|x| x.get(1).unwrap().as_str().parse().unwrap())
        .collect()
}

async fn build_one_problem(
    problem_stat: &StatWithStatus,
    mod_file_addon: &Arc<Mutex<Vec<String>>>,
) -> bool {
    if check_problem_file_existed(
        problem_stat.stat.frontend_question_id,
        &problem_stat.stat.question_title_slug.clone().unwrap(),
    ) {
        mod_file_addon.lock().unwrap().push(format!(
            "mod p{:04}_{};",
            problem_stat.stat.frontend_question_id,
            problem_stat
                .stat
                .question_title_slug
                .as_ref()
                .unwrap()
                .replace("-", "_"),
        ));
        return false;
    }
    let problem = get_problem(problem_stat).await;

    if let None = problem {
        return false;
    }
    let problem = problem.as_ref().unwrap();

    info!("problem: {}-{}", problem.question_id, problem.difficulty);
    let code = problem
        .code_definition
        .iter()
        .find(|&d| d.value == "rust".to_string());

    if code.is_none() {
        info!("Problem {} has no rust version.", problem.question_id);
        let prefix_type = match problem.paid_ony {
            true => "paid_only",
            false => "no_rust",
        };
        mod_file_addon.lock().unwrap().push(format!(
            "// mod p{:04}_{}_{};",
            problem.question_id,
            prefix_type,
            problem.title_slug.replace("-", "_")
        ));
        return true;
    }
    mod_file_addon.lock().unwrap().push(format!(
        "mod p{:04}_{};",
        problem.question_id,
        problem.title_slug.replace("-", "_")
    ));

    let code = code.unwrap();
    return deal_problem(&problem, &code, false);
}

fn parse_extra_use(code: &str) -> String {
    let mut extra_use_line = String::new();
    // a linked-list problem
    if code.contains("pub struct ListNode") {
        extra_use_line.push_str("\nuse crate::util::linked_list::{ListNode, to_list};")
    }
    if code.contains("pub struct TreeNode") {
        extra_use_line.push_str("\nuse crate::util::tree::{TreeNode, to_tree};")
    }
    if code.contains("pub struct Point") {
        extra_use_line.push_str("\nuse crate::util::point::Point;")
    }
    extra_use_line
}

fn parse_problem_link(problem: &Problem) -> String {
    format!("https://leetcode.com/problems/{}/", problem.title_slug)
}

fn parse_discuss_link(problem: &Problem) -> String {
    format!(
        "https://leetcode.com/problems/{}/discuss/?currentPage=1&orderBy=most_votes&query=",
        problem.title_slug
    )
}

fn insert_return_in_code(_return_type: &str, code: &str) -> String {
    // let re = Regex::new(r"\{[ \n]+}\n").unwrap();
    // let new_code = match return_type {
    //     "ListNode" => re
    //         .replace(&code, "{\n        Some(Box::new(ListNode::new(0)))\n}")
    //         .to_string(),
    //     "ListNode[]" => re.replace(&code, "{\n        vec![]\n}").to_string(),
    //     "TreeNode" => re
    //         .replace(
    //             &code,
    //             "{\n        Some(Rc::new(RefCell::new(TreeNode::new(0))))\n}",
    //         )
    //         .to_string(),
    //     "boolean" => re.replace(&code, "{\n    // false\n}").to_string(),
    //     "character" => re.replace(&code, "{\n    // '0'\n}").to_string(),
    //     "character[][]" => re.replace(&code, "{\n    // vec![]\n}").to_string(),
    //     "double" => re.replace(&code, "{\n    // 0f64\n}").to_string(),
    //     "double[]" => re.replace(&code, "{\n    // vec![]\n}").to_string(),
    //     "int[]" => re.replace(&code, "{\n    // vec![]\n}").to_string(),
    //     "integer" => re.replace(&code, "{\n    // 0\n}").to_string(),
    //     "integer[]" => re.replace(&code, "{\n    // vec![]\n}").to_string(),
    //     "integer[][]" => re.replace(&code, "{\n    // vec![]\n}").to_string(),
    //     "list<String>" => re.replace(&code, "{\n    // vec![]\n}").to_string(),
    //     "list<TreeNode>" => re.replace(&code, "{\n    // vec![]\n}").to_string(),
    //     "list<boolean>" => re.replace(&code, "{\n    // vec![]\n}").to_string(),
    //     "list<double>" => re.replace(&code, "{\n    // vec![]\n}").to_string(),
    //     "list<integer>" => re.replace(&code, "{\n    // vec![]\n}").to_string(),
    //     "list<list<integer>>" => re.replace(&code, "{\n    // vec![]\n}").to_string(),
    //     "list<list<string>>" => re.replace(&code, "{\n    // vec![]\n}").to_string(),
    //     "list<string>" => re.replace(&code, "{\n    // vec![]\n}").to_string(),
    //     "null" => code.to_string(),
    //     "string" => re.replace(&code, "{\n    // String::new()\n}").to_string(),
    //     "string[]" => re.replace(&code, "{\n    // vec![]\n}").to_string(),
    //     "void" => code.to_string(),
    //     "NestedInteger" => code.to_string(),
    //     "Node" => code.to_string(),
    //     _ => code.to_string(),
    // };
    // info!("{}", code);
    //

    let re = Regex::new(r"struct Solution \{[\t \n]*\}\n").unwrap();
    let code = re.replace(&code, "\n").to_string();
    let re_fn = Regex::new(r"\n([ ]+(\w* )*fn [^\{]+)\{[\t \n]+\}").unwrap();
    let out_code = re_fn.replace_all(&code, "\n///doc\n// $1 {}").to_string();
    // info!("{}", out_code);
    return out_code;
}

fn build_desc(content: &str) -> String {
    let content = content.replace("<pre>", "<pre>REPLACE_FLAG");
    let ret = parse_html(&content).replace("\nREPLACE_FLAG", "sh");
    return ret;
}

fn check_problem_file_existed(question_id: u32, title_slug: &String) -> bool {
    let file_name = format!("p{:04}_{}", question_id, title_slug.replace("-", "_"));
    let file_path =
        Path::new(&format!("{}/src/problem", ROOT_PATH)).join(format!("{}.rs", file_name));
    return check_file_existed(&file_path, false);
}

fn check_file_existed(file_path: &PathBuf, log: bool) -> bool {
    if file_path.exists() {
        if log {
            warn!("problem already initialized")
        };
        return true;
    }
    return false;
}
fn deal_problem(problem: &Problem, code: &CodeDefinition, write_mod_file: bool) -> bool {
    let file_name = format!(
        "p{:04}_{}",
        problem.question_id,
        &problem.title_slug.replace("-", "_")
    );
    let file_path =
        Path::new(&format!("{}/src/problem", ROOT_PATH)).join(format!("{}.rs", file_name));

    if check_file_existed(&file_path, true) {
        return false;
    }

    let template = fs::read_to_string("./template.trs").unwrap();
    let source = template
        .replace("__PROBLEM_TITLE__", &problem.title)
        .replace("__PROBLEM_DESC__", &build_desc(&problem.content))
        .replace(
            "__PROBLEM_DEFAULT_CODE__",
            &insert_return_in_code(&problem.return_type, &code.default_code),
        )
        .replace("__PROBLEM_ID__", &format!("{}", problem.question_id))
        .replace("__EXTRA_USE__", &parse_extra_use(&code.default_code))
        .replace("__PROBLEM_LINK__", &parse_problem_link(problem))
        .replace("__DISCUSS_LINK__", &parse_discuss_link(problem));

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&file_path)
        .unwrap();

    file.write_all(source.as_bytes()).unwrap();
    drop(file);

    if write_mod_file {
        let mut lib_file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(format!("{}/src/problem/mod.rs", ROOT_PATH))
            .unwrap();
        writeln!(lib_file, "mod {};", file_name).unwrap();
    }
    return true;
}

pub async fn deal_solving(id: i32) {
    info!("deal_solving {:?}", id);
    if id < 1 {
        return;
    }
    let id: u32 = id.try_into().unwrap();
    let problem = get_problem_by_id(id).await.unwrap();
    let file_name = format!(
        "p{:04}_{}",
        problem.question_id,
        problem.title_slug.replace("-", "_")
    );
    info!("{:?}", file_name);
    let file_path =
        Path::new(&format!("{}./src/problem", ROOT_PATH)).join(format!("{}.rs", file_name));
    // check problem/ existence
    if !file_path.exists() {
        build_all_problems(0, id.try_into().unwrap()).await;
    }
    if !file_path.exists() {
        log::error!("problem does not exist: {}", id);
        return;
    }
    // check solution/ no existence
    let solution_name = format!(
        "s{:04}_{}",
        problem.question_id,
        problem.title_slug.replace("-", "_")
    );
    let solution_path =
        Path::new(&format!("{}./src/solution", ROOT_PATH)).join(format!("{}.rs", solution_name));
    info!("path: {:?}", solution_path.to_str());
    if solution_path.exists() {
        warn!("solution exists {}", id);
        return;
    }
    // rename/move file
    fs::rename(file_path, solution_path).unwrap();
    // remove from problem/mod.rs
    let mod_file = format!("{}./src/problem/mod.rs", ROOT_PATH);
    let target_line = format!("mod {};", file_name);
    let lines: Vec<String> = io::BufReader::new(File::open(mod_file.as_str()).unwrap())
        .lines()
        .map(|x| x.unwrap())
        .filter(|x| *x != target_line)
        .collect();
    fs::write(mod_file.as_str(), lines.join("\n")).unwrap();
    // insert into solution/mod.rs
    let mut lib_file = fs::OpenOptions::new()
        .append(true)
        .open("./src/solution/mod.rs")
        .unwrap();
    writeln!(lib_file, "pub mod {};", solution_name).unwrap();
}

#[cfg(test)]
mod test {

    use super::*;

    use std::thread;
    use std::time::Duration;

    use crate::util::logger;

    #[ignore]
    #[test]
    fn test_insert_return_in_code() {
        logger::init_logger();
        info!("start test ...");
        let ori_code = r#"
        struct Solution {

        }

                    "#;
        info!("{}", ori_code);
        let ret = insert_return_in_code("boolean", ori_code);
        info!("{}", ret);
        info!("end test===============");
    }

    #[ignore]
    #[test]
    fn test_build_all_problems() {
        logger::init_logger();
        info!("start test ...");
        for number in 1..40 {
            info!("for {}", number);
            let ret = async {
                build_all_problems(50, None).await;
            };
            tokio::runtime::Runtime::new().unwrap().block_on(ret);
            thread::sleep(Duration::from_secs(10));
            // break;
        }
        info!("end test===============");
    }

    #[ignore]
    #[test]
    fn test_build_all_problems_only_one() {
        logger::init_logger();
        info!("start test ... test_build_all_problems_only_one",);

        info!("for {}", 0);
        let ret = async {
            build_all_problems(100, None).await;
        };
        tokio::runtime::Runtime::new().unwrap().block_on(ret);
        thread::sleep(Duration::from_secs(10));
        // break;

        info!("end test===============");
    }
}
