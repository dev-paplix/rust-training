// User Management System
// Demonstrates privacy, module organization, and authentication

mod user;
mod auth;

use std::io;
use user::User;
use auth::AuthService;

fn main() {
    let mut auth_service = AuthService::new();
    
    println!("=== User Management System ===\n");
    
    loop {
        println!("\n1. Register User");
        println!("2. Login");
        println!("3. View User Info");
        println!("4. Exit");
        println!("Choose an option:");
        
        let choice = read_input();
        
        match choice.trim() {
            "1" => register_user(&mut auth_service),
            "2" => login_user(&auth_service),
            "3" => view_user_info(&auth_service),
            "4" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice!"),
        }
    }
}

fn register_user(auth_service: &mut AuthService) {
    println!("\n--- User Registration ---");
    
    println!("Enter username:");
    let username = read_input().trim().to_string();
    
    println!("Enter email:");
    let email = read_input().trim().to_string();
    
    println!("Enter password:");
    let password = read_input().trim().to_string();
    
    match auth_service.register(username, email, password) {
        Ok(user_id) => println!("✅ User registered successfully! ID: {}", user_id),
        Err(e) => println!("❌ Registration failed: {}", e),
    }
}

fn login_user(auth_service: &AuthService) {
    println!("\n--- User Login ---");
    
    println!("Enter username:");
    let username = read_input().trim().to_string();
    
    println!("Enter password:");
    let password = read_input().trim().to_string();
    
    if auth_service.authenticate(&username, &password) {
        println!("✅ Login successful!");
        if let Some(user) = auth_service.get_user(&username) {
            user.display_public_info();
        }
    } else {
        println!("❌ Invalid credentials!");
    }
}

fn view_user_info(auth_service: &AuthService) {
    println!("\n--- View User Info ---");
    
    println!("Enter username:");
    let username = read_input().trim().to_string();
    
    if let Some(user) = auth_service.get_user(&username) {
        user.display_public_info();
    } else {
        println!("User not found!");
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
}
