use cosmwasm_std::Attribute;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fs::OpenOptions;

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

pub trait Result {
    fn print_results(&self);
    fn write_to_file(&self, path: &str)
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
}

impl Result for ExecuteAllResult {
    fn print_results(&self) {
        for attr in &self.attributes {
            println!("{}", serde_json::to_string(attr).unwrap());
        }
    }
}

impl<T> Result for QueryAllResult<T>
where
    T: Debug,
{
    fn print_results(&self) {
        for result in &self.responses {
            println!("{:?}", result);
        }
    }
}
