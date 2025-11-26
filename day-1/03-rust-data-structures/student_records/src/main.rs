// Student Records Management System
// Demonstrates Vec, HashMap, and custom structs

use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
struct Student {
    id: u32,
    name: String,
    grades: Vec<f64>,
}

impl Student {
    fn new(id: u32, name: String) -> Self {
        Student {
            id,
            name,
            grades: Vec::new(),
        }
    }
    
    fn add_grade(&mut self, grade: f64) {
        if grade >= 0.0 && grade <= 100.0 {
            self.grades.push(grade);
        }
    }
    
    fn average_grade(&self) -> f64 {
        if self.grades.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.grades.iter().sum();
        sum / self.grades.len() as f64
    }
    
    fn letter_grade(&self) -> char {
        let avg = self.average_grade();
        match avg {
            90.0..=100.0 => 'A',
            80.0..=89.9 => 'B',
            70.0..=79.9 => 'C',
            60.0..=69.9 => 'D',
            _ => 'F',
        }
    }
    
    fn display(&self) {
        println!("\n📚 Student ID: {}", self.id);
        println!("   Name: {}", self.name);
        println!("   Grades: {:?}", self.grades);
        println!("   Average: {:.2}", self.average_grade());
        println!("   Letter Grade: {}", self.letter_grade());
    }
}

struct StudentDatabase {
    students: HashMap<u32, Student>,
    next_id: u32,
}

impl StudentDatabase {
    fn new() -> Self {
        StudentDatabase {
            students: HashMap::new(),
            next_id: 1,
        }
    }
    
    fn add_student(&mut self, name: String) -> u32 {
        let id = self.next_id;
        let student = Student::new(id, name);
        self.students.insert(id, student);
        self.next_id += 1;
        id
    }
    
    fn add_grade_to_student(&mut self, id: u32, grade: f64) -> Result<(), String> {
        match self.students.get_mut(&id) {
            Some(student) => {
                student.add_grade(grade);
                Ok(())
            }
            None => Err(format!("Student with ID {} not found", id)),
        }
    }
    
    fn get_student(&self, id: u32) -> Option<&Student> {
        self.students.get(&id)
    }
    
    fn get_all_students(&self) -> Vec<&Student> {
        self.students.values().collect()
    }
    
    fn get_top_students(&self, n: usize) -> Vec<&Student> {
        let mut students: Vec<&Student> = self.students.values().collect();
        students.sort_by(|a, b| {
            b.average_grade()
                .partial_cmp(&a.average_grade())
                .unwrap()
        });
        students.into_iter().take(n).collect()
    }
    
    fn class_average(&self) -> f64 {
        if self.students.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.students.values().map(|s| s.average_grade()).sum();
        sum / self.students.len() as f64
    }
}

fn main() {
    let mut db = StudentDatabase::new();
    
    println!("=== Student Records Management System ===\n");
    
    loop {
        println!("\n1. Add Student");
        println!("2. Add Grade");
        println!("3. View Student");
        println!("4. View All Students");
        println!("5. View Top Students");
        println!("6. Class Statistics");
        println!("7. Exit");
        println!("Choose an option:");
        
        let choice = read_input().trim().to_string();
        
        match choice.as_str() {
            "1" => add_student(&mut db),
            "2" => add_grade(&mut db),
            "3" => view_student(&db),
            "4" => view_all_students(&db),
            "5" => view_top_students(&db),
            "6" => class_statistics(&db),
            "7" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice!"),
        }
    }
}

fn add_student(db: &mut StudentDatabase) {
    println!("\nEnter student name:");
    let name = read_input().trim().to_string();
    
    let id = db.add_student(name.clone());
    println!("✅ Student '{}' added with ID: {}", name, id);
}

fn add_grade(db: &mut StudentDatabase) {
    println!("\nEnter student ID:");
    let id: u32 = match read_input().trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid ID!");
            return;
        }
    };
    
    println!("Enter grade (0-100):");
    let grade: f64 = match read_input().trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid grade!");
            return;
        }
    };
    
    match db.add_grade_to_student(id, grade) {
        Ok(_) => println!("✅ Grade added successfully!"),
        Err(e) => println!("❌ Error: {}", e),
    }
}

fn view_student(db: &StudentDatabase) {
    println!("\nEnter student ID:");
    let id: u32 = match read_input().trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid ID!");
            return;
        }
    };
    
    match db.get_student(id) {
        Some(student) => student.display(),
        None => println!("Student not found!"),
    }
}

fn view_all_students(db: &StudentDatabase) {
    let students = db.get_all_students();
    
    if students.is_empty() {
        println!("\nNo students in database.");
        return;
    }
    
    println!("\n=== All Students ===");
    for student in students {
        student.display();
    }
}

fn view_top_students(db: &StudentDatabase) {
    println!("\nHow many top students to display?");
    let n: usize = match read_input().trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid number!");
            return;
        }
    };
    
    let top_students = db.get_top_students(n);
    
    println!("\n=== Top {} Students ===", n);
    for (rank, student) in top_students.iter().enumerate() {
        println!("\n🏆 Rank {}", rank + 1);
        student.display();
    }
}

fn class_statistics(db: &StudentDatabase) {
    let students = db.get_all_students();
    
    if students.is_empty() {
        println!("\nNo students in database.");
        return;
    }
    
    println!("\n=== Class Statistics ===");
    println!("Total Students: {}", students.len());
    println!("Class Average: {:.2}", db.class_average());
    
    // Grade distribution
    let mut grade_counts: HashMap<char, usize> = HashMap::new();
    for student in &students {
        let letter = student.letter_grade();
        *grade_counts.entry(letter).or_insert(0) += 1;
    }
    
    println!("\nGrade Distribution:");
    for grade in ['A', 'B', 'C', 'D', 'F'] {
        let count = grade_counts.get(&grade).unwrap_or(&0);
        println!("  {}: {}", grade, count);
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
}
