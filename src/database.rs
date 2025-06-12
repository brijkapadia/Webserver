use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum Status {
    InProgress,
    Done
}


#[derive(Serialize, Deserialize)]
struct Task{
    person: String,
    status: Status    
}

impl Task{
    fn new(person: String, status: Status) -> Self{
        Self { person, status}
    }
}


pub struct DataBase{
    data: HashMap<String,Task> // task_name, Task
}

impl DataBase{
    pub fn new() -> Self{
        Self {data: HashMap::new()}
    }

    pub fn add_data_by_name(&mut self, task_name: String, person: String, status: Status){
        self.data.insert(task_name, Task::new(person, status));
    }

    pub fn remove_data_by_name(&mut self, task_name: String) -> Option<Task>{
        self.data.remove(&task_name)
    }

    pub fn get_all_data_as_json(&self) -> String{
        serde_json::to_string_pretty(&self.data).unwrap()
    }

    pub fn add_data_by_json(&mut self, json_str: &str){
        let json_data_map: HashMap<String,Task> =  serde_json::from_str(json_str).unwrap();
        let (task_name, task) = json_data_map.into_iter().next().unwrap();
        self.data.insert(task_name,task);
    }
}

