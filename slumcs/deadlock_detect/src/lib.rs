use std::collections::HashMap;
use std::collections::HashSet;

pub struct DeadlockDetector {
    graph: HashMap<String, Vec<String>>,
    waiting: HashMap<String, Vec<String>>,
}

impl DeadlockDetector {
    pub fn new() -> DeadlockDetector {
        DeadlockDetector {
            graph: HashMap::new(),
            waiting: HashMap::new(),
        }
    }

    pub fn add_process(&mut self, name: &str) {
        self.graph.entry(name.to_string()).or_insert(Vec::new());
    }

    pub fn add_resource(&mut self, name: &str) {
        self.graph.entry(name.to_string()).or_insert(Vec::new());
        self.waiting.entry(name.to_string()).or_insert(Vec::new());
    }

    pub fn request(&mut self, process: &str, resource: &str) -> bool {
        if self.graph[resource].is_empty() {
            self.graph.get_mut(resource).unwrap().push(process.to_string());
            println!("Request (empty): process: {}, resource: {}, graph: {:?}, waiting: {:?}", process, resource, self.graph, self.waiting);
            true
        } else {
            self.graph.get_mut(process).unwrap().push(resource.to_string());
            
            if self.can_deadlock(process) {
                self.graph.get_mut(process).unwrap().pop();
                println!("Request (not empty - can deadlock): process: {}, resource: {}, graph: {:?}, waiting: {:?}", process, resource, self.graph, self.waiting);
                false
            } else {
                self.waiting.entry(resource.to_string()).or_insert(Vec::new()).push(process.to_string());
                println!("Request (not empty - cannot deadlock): process: {}, resource: {}, graph: {:?}, waiting: {:?}", process, resource, self.graph, self.waiting);
                true
            }   
                     
        }
    }
    

    pub fn release(&mut self, process: &str, resource: &str, next_process: Option<&str>) -> bool {
        println!("(1) process: {}, resource: {}, next_process: {:?}, graph: {:?}", process, resource, next_process, self.graph);
        
        if let Some(pos) = self.graph[resource].iter().position(|x| x == process) {
            self.graph.get_mut(resource).unwrap().remove(pos);
            if let Some(pos) = self.graph[process].iter().position(|x| x == resource) {
                self.graph.get_mut(process).unwrap().remove(pos);
            }
        }
    
        let waiting_queue = self.waiting.entry(resource.to_string()).or_insert(Vec::new());
    
        if let Some(next) = next_process {
            println!("(2) process: {}, resource: {}, next_process: {:?}", process, resource, next_process);
    
            if Some(next) == waiting_queue.first().map(|s| s.as_str()) {
                println!("(expected): process: {}, resource: {}, next_process: {:?}, graph: {:?}", process, resource, next_process, self.graph);
    
                waiting_queue.remove(0);
                self.graph.get_mut(resource).unwrap().push(next.to_string());
                if let Some(pos) = self.graph[next].iter().position(|x| x == resource) {
                    self.graph.get_mut(next).unwrap().remove(pos);
                }
                
    
                if self.can_deadlock(next) {
                    println!("release: process: {}, resource: {}, next_process: {:?}, graph: {:?}", process, resource, next_process, self.graph);
                    self.graph.get_mut(resource).unwrap().pop();
                    println!("release: process: {}, resource: {}, next_process: {:?}, graph: {:?}", process, resource, next_process, self.graph);
                    false
                } else {
                    true
                }
            } else {
                false
            }
        } else {
            println!("(3) process: {}, resource: {}, next_process: {:?}", process, resource, next_process);
    
            if let Some(next) = waiting_queue.get(0).cloned() {
                waiting_queue.remove(0);
                self.graph.get_mut(resource).unwrap().push(next);
            }
            true
        }
    }
    

    pub fn can_deadlock(&self, start: &str) -> bool {
        fn dfs_visit(
            node: &str,
            graph: &HashMap<String, Vec<String>>,
            colors: &mut HashMap<String, char>,
            is_process: bool,
        ) -> bool {
            colors.insert(node.to_string(), 'g'); // mark the node as gray
        
            if let Some(neighbors) = graph.get(node) {
                for neighbor in neighbors {
                    // Only visit the neighbor if it's a process and the current node is a resource, or vice versa
                    let neighbor_is_process = !is_process;
                    if is_process != neighbor_is_process {
                        let color = colors.entry(neighbor.to_string()).or_insert('w'); // default to white
                        
                        if *color == 'g' {
                            return true; // found a cycle
                        } else if *color == 'w' {
                            if dfs_visit(neighbor, graph, colors, neighbor_is_process) {
                                return true;
                            }
                        }
                    }
                }
            }
        
            colors.insert(node.to_string(), 'b'); // mark the node as black
            false
        } 
        let mut colors = HashMap::new();
        dfs_visit(start, &self.graph, &mut colors, true)
    }
}
