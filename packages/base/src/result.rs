use cosmwasm_std::Attribute;
use std::fs::OpenOptions;
use std::io::Write;

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
        for attr in &self.total_attributes {
            serde_json::to_writer(&file, attr).unwrap();
        }
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
        for attr in &self.query_results {
            file.write_all(attr.as_bytes()).unwrap();
        }
    }
}
