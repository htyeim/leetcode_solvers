use reqwest;
use serde_json;

use log::{error, info};

use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, io};

use serde_derive::{Deserialize, Serialize};
use std::fmt::{Display, Error, Formatter};

const QUESTION_QUERY_OPERATION: &str = "questionData";

const QUESTION_QUERY_STRING: &str = r#"
    query questionData($titleSlug: String!) {
        question(titleSlug: $titleSlug) {
            content
            stats
            codeDefinition
            sampleTestCase
            metaData
        }
    }"#;

#[derive(Serialize, Deserialize)]
pub struct Problem {
    pub paid_ony: bool,
    pub title: String,
    pub title_slug: String,
    pub content: String,
    #[serde(rename = "codeDefinition")]
    pub code_definition: Vec<CodeDefinition>,
    #[serde(rename = "sampleTestCase")]
    pub sample_test_case: String,
    pub difficulty: String,
    pub question_id: u32,
    pub return_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct CodeDefinition {
    pub value: String,
    pub text: String,
    #[serde(rename = "defaultCode")]
    pub default_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Query {
    #[serde(rename = "operationName")]
    operation_name: String,
    variables: serde_json::Value,
    query: String,
}

impl Query {
    fn question_query(title_slug: &str) -> Query {
        Query {
            operation_name: QUESTION_QUERY_OPERATION.to_owned(),
            variables: serde_json::json!({ "titleSlug": title_slug }),
            query: QUESTION_QUERY_STRING.to_owned(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct RawProblem {
    data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    question: Question,
}

#[derive(Debug, Serialize, Deserialize)]
struct Question {
    content: String,
    stats: String,
    #[serde(rename = "codeDefinition")]
    code_definition: String,
    #[serde(rename = "sampleTestCase")]
    sample_test_case: String,
    #[serde(rename = "metaData")]
    meta_data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Problems {
    user_name: String,
    num_solved: u32,
    num_total: u32,
    ac_easy: u32,
    ac_medium: u32,
    ac_hard: u32,
    pub stat_status_pairs: Vec<StatWithStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatWithStatus {
    pub stat: Stat,
    difficulty: Difficulty,
    paid_only: bool,
    is_favor: bool,
    frequency: u32,
    progress: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stat {
    pub question_id: u32,
    #[serde(rename = "question__article__slug")]
    question_article_slug: Option<String>,
    #[serde(rename = "question__title")]
    question_title: Option<String>,
    #[serde(rename = "question__title_slug")]
    pub question_title_slug: Option<String>,
    #[serde(rename = "question__hide")]
    question_hide: bool,
    total_acs: u32,
    total_submitted: u32,
    pub frontend_question_id: u32,
    is_new_question: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Difficulty {
    level: u32,
}

impl Display for Difficulty {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self.level {
            1 => f.write_str("Easy"),
            2 => f.write_str("Medium"),
            3 => f.write_str("Hard"),
            _ => f.write_str("Unknown"),
        }
    }
}

const PROBLEMS_URL: &str = "https://leetcode.com/api/problems/algorithms/";
const GRAPHQL_URL: &str = "https://leetcode.com/graphql";
const TMP_CACHE_PROBLEMS: &str = "log/cache-problems.json";

fn is_file_older_enough(path: &str, enough_in_secs: u64) -> io::Result<fs::Metadata> {
    // = 10 * 24 * 60 * 60
    let metadata = fs::metadata(path)?;
    let modified = metadata.modified()?;

    // Calculate the difference in seconds
    let duration = modified.duration_since(UNIX_EPOCH).unwrap();
    let secs = duration.as_secs();

    // Check if the file is older than 10 days
    let ret = secs
        < (SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            - enough_in_secs);
    if ret {
        return Ok(metadata);
    }
    Err(io::Error::new(io::ErrorKind::Other, "Custom error message"))
}

#[allow(dead_code)]
pub async fn get_problems() -> Problems {
    let cache_good = is_file_older_enough(TMP_CACHE_PROBLEMS, 10 * 24 * 60 * 60);
    if let Ok(file_status) = cache_good {
        let modified = file_status.modified();
        info!("Modified time: {:?}", modified);

        let result = fs::read_to_string(TMP_CACHE_PROBLEMS);
        if let Ok(contents) = result {
            let ret: Result<Problems, serde_json::Error> = serde_json::from_str(&contents);
            if let Ok(mut problems) = ret {
                problems.stat_status_pairs.sort_by(|a, b| {
                    a.stat
                        .frontend_question_id
                        .cmp(&b.stat.frontend_question_id)
                });
                return problems;
            }
        }
    }
    let res = reqwest::get(PROBLEMS_URL).await.unwrap();
    // -> Option<Problems>
    // .json::<Query>()
    // .await;
    let text = res.text().await.unwrap();
    let mut problems: Problems = serde_json::from_str(&text).unwrap();

    problems.stat_status_pairs.sort_by(|a, b| {
        a.stat
            .frontend_question_id
            .cmp(&b.stat.frontend_question_id)
    });
    let json_string = serde_json::to_string(&problems).unwrap();
    fs::write(TMP_CACHE_PROBLEMS, json_string).unwrap();
    problems
}

pub async fn get_problem(problem: &StatWithStatus) -> Option<Problem> {
    if problem.paid_only {
        info!(
            "{} paid_only return None",
            &problem.stat.frontend_question_id
        );
        return Some(Problem {
            paid_ony: true,
            title: problem.stat.question_title.clone().unwrap(),
            title_slug: problem.stat.question_title_slug.clone().unwrap(),
            code_definition: vec![],
            content: "".to_string(),
            sample_test_case: "".to_string(),
            difficulty: problem.difficulty.to_string(),
            question_id: problem.stat.frontend_question_id,
            return_type: "boolean".to_string(),
        });
    }
    let client = reqwest::Client::new();
    let resp = client
        .post(GRAPHQL_URL)
        .json(&Query::question_query(
            problem.stat.question_title_slug.as_ref().unwrap(),
        ))
        .send()
        .await;
    if let Err(err) = resp {
        error!(
            "{} cannot get response: {:?}",
            &problem.stat.frontend_question_id, err
        );
        return None;
    };
    let resp: RawProblem = resp.unwrap().json().await.unwrap();
    return Some(Problem {
        paid_ony: false,
        title: problem.stat.question_title.clone().unwrap(),
        title_slug: problem.stat.question_title_slug.clone().unwrap(),
        code_definition: serde_json::from_str(&resp.data.question.code_definition).unwrap(),
        content: resp.data.question.content,
        sample_test_case: resp.data.question.sample_test_case,
        difficulty: problem.difficulty.to_string(),
        question_id: problem.stat.frontend_question_id,
        return_type: {
            let v: serde_json::Value = serde_json::from_str(&resp.data.question.meta_data).unwrap();
            v["return"]["type"].to_string().replace("\"", "")
        },
    });
}

#[allow(dead_code)]
pub async fn get_problem_by_id(frontend_question_id: u32) -> Option<Problem> {
    let problems = get_problems().await;
    info!("{}", problems.num_total);
    for problem in problems.stat_status_pairs.iter() {
        if problem.stat.frontend_question_id != frontend_question_id {
            continue;
        }
        return get_problem(problem).await;
    }
    return None;
}

#[cfg(test)]
mod test {

    use crate::util::logger;
    use log::info;

    #[ignore]
    #[test]
    fn test() {
        logger::init_logger();

        info!("booting up");

        let ret = async {
            let ret = super::get_problems().await;
            ret
        };
        let ret = tokio::runtime::Runtime::new().unwrap().block_on(ret);
        info!("{}", ret.num_total);

        let p = async { super::get_problem_by_id(1).await };

        let p = tokio::runtime::Runtime::new().unwrap().block_on(p).unwrap();
        info!("{}", p.difficulty);
    }
}
