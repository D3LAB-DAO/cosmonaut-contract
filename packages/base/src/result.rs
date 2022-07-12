use cosmwasm_std::{Attribute, Uint128};
use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fs::OpenOptions;
use std::iter::zip;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ExecuteAllResult {
    pub attributes: Vec<Vec<Attribute>>,
    pub errors: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QueryAllResult<T> {
    pub responses: Vec<T>,
    pub errors: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AnswerCheck {
    pub answer_type: String,
    pub lesson: String,
    pub result: String,
    pub error: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Freight {
    pub denom: String,
    pub amount: Uint128,
    pub unit_weight: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Metadata {
    pub unit_denom: String,
    pub price: u128,
    pub name: Option<String>,
    pub freight: Vec<Freight>,
    pub health: u128,
    pub fuel: u128,
}

pub trait Result {
    fn print_results(&self);
    fn write_answer_to_file(&self, path: &str)
    where
        Self: Serialize,
    {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .unwrap();
        serde_json::to_writer_pretty(&file, &self).unwrap();
    }
    fn check_answer(&self, lesson: &str, correct_answer_path: &str) -> AnswerCheck;
}

impl Result for ExecuteAllResult {
    fn print_results(&self) {
        for attr in &self.attributes {
            println!("{}", serde_json::to_string(attr).unwrap());
        }
    }
    fn check_answer(&self, lesson: &str, correct_answer_path: &str) -> AnswerCheck {
        let content: String = std::fs::read_to_string(correct_answer_path)
            .unwrap()
            .parse()
            .unwrap();
        let correct_answer: ExecuteAllResult = serde_json::from_str(&content).unwrap();

        if &correct_answer == self {
            AnswerCheck {
                answer_type: "execute".to_string(),
                lesson: lesson.to_string(),
                result: "success".to_string(),
                error: vec![],
            }
        } else {
            AnswerCheck {
                answer_type: "execute".to_string(),
                lesson: lesson.to_string(),
                result: "fail".to_string(),
                error: self.errors.clone(),
            }
        }
    }
}

impl<T> Result for QueryAllResult<T>
where
    T: Debug + DeserializeOwned + PartialEq + Serialize + Clone,
{
    fn print_results(&self) {
        for result in &self.responses {
            println!("{:?}", result);
        }
    }

    fn check_answer(&self, lesson: &str, correct_answer_path: &str) -> AnswerCheck {
        let content: String = std::fs::read_to_string(correct_answer_path)
            .unwrap()
            .parse()
            .unwrap();
        let correct_answer: QueryAllResult<T> = serde_json::from_str(&content).unwrap();

        for (i, j) in zip(&self.responses, &correct_answer.responses) {
            if i != j {
                println!("your result: {:?}\ncorrect answer: {:?}", i, j);
            }
        }
        if &correct_answer == self {
            AnswerCheck {
                answer_type: "query".to_string(),
                lesson: lesson.to_string(),
                result: "success".to_string(),
                error: vec![],
            }
        } else {
            AnswerCheck {
                answer_type: "query".to_string(),
                lesson: lesson.to_string(),
                result: "fail".to_string(),
                error: self.errors.clone(),
            }
        }
    }
}

impl AnswerCheck {
    pub fn write_to_file(&self, file_path: &str) {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(file_path)
            .unwrap();
        serde_json::to_writer_pretty(&file, &self).unwrap();
    }
}
