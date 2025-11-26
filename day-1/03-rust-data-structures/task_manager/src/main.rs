// Priority Task Manager
// Demonstrates BinaryHeap (priority queue) and custom structs

use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::io;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Task {
    id: u32,
    description: String,
    priority: u8,  // 1 (low) to 5 (high)
}

impl Task {
    fn new(id: u32, description: String, priority: u8) -> Self {
        Task {
            id,
            description,
            priority: priority.min(5).max(1),  // Clamp between 1 and 5
        }
    }
    
    fn display(&self) {
        let priority_str = match self.priority {
            5 => "🔴 CRITICAL",
            4 => "🟠 HIGH",
            3 => "🟡 MEDIUM",
            2 => "🟢 LOW",
            _ => "⚪ MINIMAL",
        };
        
        println!("Task #{}: {} - {}", self.id, priority_str, self.description);
    }
}

// Implement Ord for priority queue
impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        // Higher priority comes first
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct TaskManager {
    tasks: BinaryHeap<Task>,
    next_id: u32,
    completed: Vec<Task>,
}

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: BinaryHeap::new(),
            next_id: 1,
            completed: Vec::new(),
        }
    }
    
    fn add_task(&mut self, description: String, priority: u8) {
        let task = Task::new(self.next_id, description, priority);
        println!("✅ Added: ");
        task.display();
        self.tasks.push(task);
        self.next_id += 1;
    }
    
    fn process_next_task(&mut self) -> Option<Task> {
        if let Some(task) = self.tasks.pop() {
            println!("⚙️  Processing:");
            task.display();
            self.completed.push(task.clone());
            Some(task)
        } else {
            println!("No tasks to process!");
            None
        }
    }
    
    fn view_pending_tasks(&self) {
        if self.tasks.is_empty() {
            println!("\n📋 No pending tasks.");
            return;
        }
        
        println!("\n📋 Pending Tasks ({}):", self.tasks.len());
        
        // Clone heap to view without consuming
        let mut temp: Vec<Task> = self.tasks.iter().cloned().collect();
        temp.sort_by(|a, b| b.priority.cmp(&a.priority));
        
        for task in temp {
            print!("   ");
            task.display();
        }
    }
    
    fn view_completed_tasks(&self) {
        if self.completed.is_empty() {
            println!("\n✓ No completed tasks.");
            return;
        }
        
        println!("\n✓ Completed Tasks ({}):", self.completed.len());
        for task in &self.completed {
            print!("   ");
            task.display();
        }
    }
    
    fn statistics(&self) {
        println!("\n📊 Task Statistics:");
        println!("   Pending: {}", self.tasks.len());
        println!("   Completed: {}", self.completed.len());
        println!("   Total created: {}", self.next_id - 1);
        
        // Priority breakdown for pending tasks
        let mut priority_counts = [0; 5];
        for task in self.tasks.iter() {
            priority_counts[(task.priority - 1) as usize] += 1;
        }
        
        println!("\n   Pending by Priority:");
        let labels = ["⚪ Minimal", "🟢 Low", "🟡 Medium", "🟠 High", "🔴 Critical"];
        for (i, count) in priority_counts.iter().enumerate() {
            if *count > 0 {
                println!("      {}: {}", labels[i], count);
            }
        }
    }
}

fn main() {
    let mut manager = TaskManager::new();
    
    println!("=== Priority Task Manager ===\n");
    
    loop {
        println!("\n1. Add Task");
        println!("2. Process Next Task (Highest Priority)");
        println!("3. View Pending Tasks");
        println!("4. View Completed Tasks");
        println!("5. Statistics");
        println!("6. Exit");
        println!("Choose an option:");
        
        let choice = read_input().trim().to_string();
        
        match choice.as_str() {
            "1" => add_task(&mut manager),
            "2" => {
                manager.process_next_task();
            }
            "3" => manager.view_pending_tasks(),
            "4" => manager.view_completed_tasks(),
            "5" => manager.statistics(),
            "6" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice!"),
        }
    }
}

fn add_task(manager: &mut TaskManager) {
    println!("\nEnter task description:");
    let description = read_input().trim().to_string();
    
    if description.is_empty() {
        println!("Task description cannot be empty!");
        return;
    }
    
    println!("Enter priority (1=Minimal, 2=Low, 3=Medium, 4=High, 5=Critical):");
    let priority: u8 = match read_input().trim().parse() {
        Ok(p) if p >= 1 && p <= 5 => p,
        _ => {
            println!("Invalid priority! Using default (3=Medium)");
            3
        }
    };
    
    manager.add_task(description, priority);
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
}
