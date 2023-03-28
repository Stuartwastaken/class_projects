use std::collections::HashMap;
use std::collections::HashSet;

pub struct DeadlockDetector {
    processes: HashMap<String, Vec<String>>,
    resources: HashMap<String, Vec<String>>,
}

impl DeadlockDetector {
    pub fn new() -> DeadlockDetector {
        DeadlockDetector {
            processes: HashMap::new(),
            resources: HashMap::new(),
        }
    }

    pub fn add_process(&mut self, name: &str) {
        self.processes.entry(name.to_string()).or_insert(vec![]);
    }

    pub fn add_resource(&mut self, name: &str) {
        self.resources.entry(name.to_string()).or_insert(vec![]);
    }

    pub fn request(&mut self, process: &str, resource: &str) -> bool {
        // Check if the resource is already being used
        if let Some(used_by) = self.resources.get(resource) {
            if used_by.contains(&process.to_string()) {
                // Process already has claimed the resource, no deadlock
                return true;
            } else {
                // Add an edge from the resource to the process
                self.resources
                    .entry(resource.to_string())
                    .and_modify(|v| v.push(process.to_string()))
                    .or_insert(vec![process.to_string()]);
            }
        } else {
            // Add the resource to the graph
            self.resources
                .entry(resource.to_string())
                .or_insert(vec![process.to_string()]);
        }
        println!("Added new request {process}")

        // Check if the new edge created a deadlock cycle
        if self.can_deadlock() {
            // Remove the edge that was added
            if let Some(used_by) = self.resources.get_mut(resource) {
                used_by.retain(|p| p != process);
            }
            return false;
        } else {
            return true;
        }
    }

    pub fn release(&mut self, process: &str, resource: &str, next_process: Option<&str>) -> bool {
        // Remove the edge from the process to the resource
        if let Some(uses) = self.processes.get_mut(process) {
            uses.retain(|r| r != resource);
        } else {
            return false;
        }

        // Convert the edge from the resource to the process
        if let Some(next_process) = next_process {
            // Remove the edge from the resource to the current process
            if let Some(used_by) = self.resources.get_mut(resource) {
                used_by.retain(|p| p != process);
            } else {
                return false;
            }

            // Add the edge from the resource to the next process
            if let Some(uses) = self.resources.get_mut(resource) {
                uses.push(next_process.to_string());
            } else {
                return false;
            }

            // Check if the new edge created a deadlock cycle
            if self.can_deadlock() {
                // Remove the edge that was added
                if let Some(used_by) = self.resources.get_mut(resource) {
                    used_by.retain(|p| p != next_process);
                }
                return false;
            } else {
                return true;
            }
        } else {
            return true;
        }
    }

    fn can_deadlock(&self) -> bool {
        for (_, uses) in self.processes.iter() {
            for resource in uses.iter() {
                for waiting_process in self.resources.get(resource).unwrap().into_iter() {
                    if self.can_deadlock_recursive(waiting_process, &mut HashSet::new()) {
                        return true;
                    }
                }
            }
        }
        return false;
    }  

    fn can_deadlock_recursive(&self, current: &str, visited: &mut HashSet<String>) -> bool {
        if visited.contains(current) {
            return true;
        }
        visited.insert(current.to_string());
        for resource in self.processes.get(current).into_iter().flatten() {
            for waiting_process in self.resources.get(resource).into_iter().flatten() {
                if visited.contains(waiting_process) {
                    return true;
                }
                if self.can_deadlock_recursive(waiting_process, visited) {
                    return true;
                }
            }
        }
        visited.remove(current);
        return false;
    }

}
    
