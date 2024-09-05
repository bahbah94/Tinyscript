use std::collections::HashMap;

pub struct Runtime {
    global_memory: HashMap<String, i64>,  // Global variables
    stack: Vec<HashMap<String, i64>>,     // Stack frames for local variables
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            global_memory: HashMap::new(),
            stack: Vec::new(),
        }
    }

    pub fn enter_scope(&mut self) {
        self.stack.push(HashMap::new());  // Create a new stack frame
    }

    pub fn exit_scope(&mut self) {
        self.stack.pop();  // Remove the current stack frame
    }

    pub fn set_global(&mut self, name: String, value: i64) {
        self.global_memory.insert(name, value);
    }

    pub fn get_global(&self, name: &str) -> Option<&i64> {
        self.global_memory.get(name)
    }

    pub fn set_local(&mut self, name: String, value: i64) {
        if let Some(frame) = self.stack.last_mut() {
            frame.insert(name, value);
        }
    }

    pub fn get_local(&self, name: &str) -> Option<&i64> {
        if let Some(frame) = self.stack.last() {
            frame.get(name)
        } else {
            None
        }
    }
}
