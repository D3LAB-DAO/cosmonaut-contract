use cosmwasm_std::Attribute;
use std::fs::OpenOptions;
use std::io::{IoSlice, Write};

pub struct ExecuteAllResult {
    pub total_attributes: Vec<Vec<Attribute>>,
}

pub struct QueryAllResult {
    pub query_results: Vec<String>,
}

pub trait Result {
    fn print_results(&self);
    fn write_to_file(&self, path: &str);
}

impl Result for ExecuteAllResult {
    fn print_results(&self) {
        for attr in &self.total_attributes {
            println!("{}", serde_json::to_string(attr).unwrap());
        }
    }

    fn write_to_file(&self, path: &str) {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .unwrap();
        serde_json::to_writer_pretty(&file, &self.total_attributes).unwrap();
    }
}

impl Result for QueryAllResult {
    fn print_results(&self) {
        for result in &self.query_results {
            println!("{}", result);
        }
    }

    fn write_to_file(&self, path: &str) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .unwrap();

        let mut idx = 0;

        file.write_all("[".as_bytes()).unwrap();
        for i in &self.query_results {
            file.write_all(i.as_bytes()).unwrap();
            if idx != &self.query_results.len() - 1 {
                file.write_all(",\n".as_bytes());
                idx += 1;
            }
        }
        file.write_all("]".as_bytes()).unwrap();
    }
}
