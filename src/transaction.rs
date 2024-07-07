use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction{
    pub id: u64,
    pub input_counter: i32,
    pub inputs: Vec<String>,
    pub output_counter: i32,
    pub outputs: Vec<String>,
    pub locktime: f64,
}