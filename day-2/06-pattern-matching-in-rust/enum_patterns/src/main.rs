use std::io::{self, Write};

/// Demonstrates pattern matching with enums
fn main() {
    println!("=== Enum Pattern Matching Demo ===\n");
    
    loop {
        display_menu();
        
        let choice = get_user_choice();
        
        match choice {
            1 => message_demo(),
            2 => option_demo(),
            3 => result_demo(),
            4 => status_demo(),
            5 => payment_demo(),
            6 => state_machine_demo(),
            7 => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice, try again."),
        }
        
        println!();
    }
}

fn display_menu() {
    println!("\n--- Enum Pattern Matching Examples ---");
    println!("1. Message Types");
    println!("2. Option Handling");
    println!("3. Result Handling");
    println!("4. Status Management");
    println!("5. Payment Processing");
    println!("6. State Machine");
    println!("7. Exit");
}

fn get_user_choice() -> u32 {
    print!("\nEnter your choice: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    input.trim().parse().unwrap_or(0)
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

// Message Types Demo
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

fn message_demo() {
    println!("\n--- Message Types ---");
    println!("1. Quit");
    println!("2. Move");
    println!("3. Write");
    println!("4. Change Color");
    
    let choice = get_input("\nSelect message type: ");
    
    let message = match choice.as_str() {
        "1" => Message::Quit,
        "2" => {
            let x = get_input("Enter x: ").parse().unwrap_or(0);
            let y = get_input("Enter y: ").parse().unwrap_or(0);
            Message::Move { x, y }
        }
        "3" => {
            let text = get_input("Enter text: ");
            Message::Write(text)
        }
        "4" => {
            let r = get_input("Red (0-255): ").parse().unwrap_or(0);
            let g = get_input("Green (0-255): ").parse().unwrap_or(0);
            let b = get_input("Blue (0-255): ").parse().unwrap_or(0);
            Message::ChangeColor(r, g, b)
        }
        _ => {
            println!("Invalid choice!");
            return;
        }
    };
    
    process_message(message);
}

fn process_message(msg: Message) {
    println!("\nProcessing message:");
    
    match msg {
        Message::Quit => {
            println!("  Type: Quit");
            println!("  Action: Application will quit");
        }
        Message::Move { x, y } => {
            println!("  Type: Move");
            println!("  Action: Moving to coordinates ({}, {})", x, y);
            
            if x == 0 && y == 0 {
                println!("  Note: Moving to origin");
            } else if x == 0 {
                println!("  Note: Moving along Y-axis");
            } else if y == 0 {
                println!("  Note: Moving along X-axis");
            }
        }
        Message::Write(text) => {
            println!("  Type: Write");
            println!("  Action: Writing text: '{}'", text);
            println!("  Length: {} characters", text.len());
        }
        Message::ChangeColor(r, g, b) => {
            println!("  Type: ChangeColor");
            println!("  Action: Changing color to RGB({}, {}, {})", r, g, b);
            
            let color_name = match (r, g, b) {
                (255, 0, 0) => "Red",
                (0, 255, 0) => "Green",
                (0, 0, 255) => "Blue",
                (255, 255, 0) => "Yellow",
                (255, 0, 255) => "Magenta",
                (0, 255, 255) => "Cyan",
                (255, 255, 255) => "White",
                (0, 0, 0) => "Black",
                _ => "Custom color",
            };
            
            println!("  Color: {}", color_name);
        }
    }
}

// Option Demo
fn option_demo() {
    println!("\n--- Option Handling ---");
    
    let input = get_input("Enter a number (or leave empty): ");
    
    let option_number = if input.is_empty() {
        None
    } else {
        input.parse::<i32>().ok()
    };
    
    match option_number {
        Some(num) => {
            println!("You entered: {}", num);
            println!("Doubled: {}", num * 2);
            println!("Squared: {}", num * num);
        }
        None => {
            println!("No valid number provided");
            println!("Using default value: 0");
        }
    }
    
    // Demonstrate if let
    println!("\nUsing if let:");
    if let Some(n) = option_number {
        println!("Got value: {}", n);
    } else {
        println!("No value");
    }
    
    // Nested Option
    let nested: Option<Option<i32>> = Some(option_number);
    match nested {
        Some(Some(n)) => println!("Nested value: {}", n),
        Some(None) => println!("Inner is None"),
        None => println!("Outer is None"),
    }
}

// Result Demo
fn result_demo() {
    println!("\n--- Result Handling ---");
    
    let input = get_input("Enter a number to parse: ");
    
    let result = parse_and_validate(&input);
    
    match result {
        Ok(num) => {
            println!("✓ Success!");
            println!("  Number: {}", num);
            println!("  Is positive: {}", num > 0);
            println!("  Is even: {}", num % 2 == 0);
        }
        Err(e) => {
            println!("✗ Error!");
            println!("  {}", e);
        }
    }
    
    // Using if let with Result
    println!("\nTrying division:");
    let dividend = get_input("Enter dividend: ").parse().unwrap_or(10);
    let divisor = get_input("Enter divisor: ").parse().unwrap_or(2);
    
    if let Ok(result) = divide(dividend, divisor) {
        println!("{} / {} = {}", dividend, divisor, result);
    } else {
        println!("Cannot divide by zero!");
    }
}

fn parse_and_validate(s: &str) -> Result<i32, String> {
    let num = s.parse::<i32>()
        .map_err(|_| format!("'{}' is not a valid number", s))?;
    
    if num == 0 {
        Err("Number cannot be zero".to_string())
    } else {
        Ok(num)
    }
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

// Status Demo
#[derive(Debug, Clone)]
enum Status {
    Active,
    Inactive,
    Pending,
    Suspended { reason: String },
    Cancelled { reason: String, refund: bool },
}

fn status_demo() {
    println!("\n--- Status Management ---");
    println!("1. Active");
    println!("2. Inactive");
    println!("3. Pending");
    println!("4. Suspended");
    println!("5. Cancelled");
    
    let choice = get_input("\nSelect status: ");
    
    let status = match choice.as_str() {
        "1" => Status::Active,
        "2" => Status::Inactive,
        "3" => Status::Pending,
        "4" => {
            let reason = get_input("Suspension reason: ");
            Status::Suspended { reason }
        }
        "5" => {
            let reason = get_input("Cancellation reason: ");
            let refund = get_input("Refund? (yes/no): ");
            Status::Cancelled {
                reason,
                refund: refund.to_lowercase() == "yes",
            }
        }
        _ => {
            println!("Invalid choice!");
            return;
        }
    };
    
    display_status(&status);
    
    let can_access = match status {
        Status::Active => true,
        Status::Pending => false,
        Status::Inactive => false,
        Status::Suspended { .. } => false,
        Status::Cancelled { .. } => false,
    };
    
    println!("\nCan access system: {}", can_access);
}

fn display_status(status: &Status) {
    println!("\nStatus Details:");
    
    match status {
        Status::Active => {
            println!("  ✓ Active");
            println!("  User has full access");
        }
        Status::Inactive => {
            println!("  ○ Inactive");
            println!("  User is not currently using the service");
        }
        Status::Pending => {
            println!("  ⏳ Pending");
            println!("  Awaiting approval or verification");
        }
        Status::Suspended { reason } => {
            println!("  ⚠ Suspended");
            println!("  Reason: {}", reason);
            println!("  Access is temporarily restricted");
        }
        Status::Cancelled { reason, refund } => {
            println!("  ✗ Cancelled");
            println!("  Reason: {}", reason);
            println!("  Refund issued: {}", if *refund { "Yes" } else { "No" });
        }
    }
}

// Payment Demo
#[derive(Debug)]
enum Payment {
    Cash(f64),
    CreditCard { number: String, amount: f64 },
    DebitCard { number: String, amount: f64 },
    Crypto { wallet: String, amount: f64, currency: String },
}

fn payment_demo() {
    println!("\n--- Payment Processing ---");
    println!("1. Cash");
    println!("2. Credit Card");
    println!("3. Debit Card");
    println!("4. Cryptocurrency");
    
    let choice = get_input("\nSelect payment method: ");
    
    let payment = match choice.as_str() {
        "1" => {
            let amount = get_input("Amount: ").parse().unwrap_or(0.0);
            Payment::Cash(amount)
        }
        "2" => {
            let number = get_input("Card number (last 4 digits): ");
            let amount = get_input("Amount: ").parse().unwrap_or(0.0);
            Payment::CreditCard { number, amount }
        }
        "3" => {
            let number = get_input("Card number (last 4 digits): ");
            let amount = get_input("Amount: ").parse().unwrap_or(0.0);
            Payment::DebitCard { number, amount }
        }
        "4" => {
            let wallet = get_input("Wallet address: ");
            let amount = get_input("Amount: ").parse().unwrap_or(0.0);
            let currency = get_input("Currency (BTC/ETH): ");
            Payment::Crypto { wallet, amount, currency }
        }
        _ => {
            println!("Invalid choice!");
            return;
        }
    };
    
    process_payment(payment);
}

fn process_payment(payment: Payment) {
    println!("\n--- Processing Payment ---");
    
    match payment {
        Payment::Cash(amount) => {
            println!("Method: Cash");
            println!("Amount: ${:.2}", amount);
            println!("Status: ✓ Received");
        }
        Payment::CreditCard { number, amount } => {
            println!("Method: Credit Card");
            println!("Card: ****{}", number);
            println!("Amount: ${:.2}", amount);
            println!("Fee: ${:.2}", amount * 0.029); // 2.9% fee
            println!("Status: ✓ Authorized");
        }
        Payment::DebitCard { number, amount } => {
            println!("Method: Debit Card");
            println!("Card: ****{}", number);
            println!("Amount: ${:.2}", amount);
            println!("Fee: ${:.2}", amount * 0.015); // 1.5% fee
            println!("Status: ✓ Processing");
        }
        Payment::Crypto { wallet, amount, currency } => {
            println!("Method: Cryptocurrency");
            println!("Currency: {}", currency);
            println!("Wallet: {}...{}", &wallet[..6.min(wallet.len())], 
                     &wallet[wallet.len().saturating_sub(4)..]);
            println!("Amount: {} {}", amount, currency);
            println!("Status: ⏳ Pending confirmation");
        }
    }
}

// State Machine Demo
#[derive(Debug, Clone)]
enum State {
    Idle,
    Running { task: String, progress: u32 },
    Paused { task: String, progress: u32 },
    Completed { task: String },
    Failed { task: String, error: String },
}

fn state_machine_demo() {
    println!("\n--- State Machine ---");
    
    let mut state = State::Idle;
    
    loop {
        display_state(&state);
        
        println!("\nActions:");
        match &state {
            State::Idle => println!("1. Start task"),
            State::Running { .. } => println!("1. Pause  2. Complete  3. Fail"),
            State::Paused { .. } => println!("1. Resume  2. Cancel"),
            State::Completed { .. } | State::Failed { .. } => println!("1. Reset"),
        }
        println!("0. Back to menu");
        
        let choice = get_input("\nAction: ");
        
        state = match (&state, choice.as_str()) {
            (State::Idle, "1") => {
                let task = get_input("Task name: ");
                State::Running { task, progress: 0 }
            }
            (State::Running { task, progress }, "1") => {
                State::Paused { task: task.clone(), progress: *progress }
            }
            (State::Running { task, .. }, "2") => {
                State::Completed { task: task.clone() }
            }
            (State::Running { task, .. }, "3") => {
                let error = get_input("Error message: ");
                State::Failed { task: task.clone(), error }
            }
            (State::Paused { task, progress }, "1") => {
                State::Running { task: task.clone(), progress: *progress + 10 }
            }
            (State::Paused { .. }, "2") => State::Idle,
            (State::Completed { .. } | State::Failed { .. }, "1") => State::Idle,
            (_, "0") => break,
            _ => {
                println!("Invalid action!");
                state.clone()
            }
        };
    }
}

fn display_state(state: &State) {
    println!("\n--- Current State ---");
    
    match state {
        State::Idle => {
            println!("Status: ○ Idle");
            println!("No active task");
        }
        State::Running { task, progress } => {
            println!("Status: ▶ Running");
            println!("Task: {}", task);
            println!("Progress: {}%", progress);
        }
        State::Paused { task, progress } => {
            println!("Status: ⏸ Paused");
            println!("Task: {}", task);
            println!("Progress: {}%", progress);
        }
        State::Completed { task } => {
            println!("Status: ✓ Completed");
            println!("Task: {}", task);
        }
        State::Failed { task, error } => {
            println!("Status: ✗ Failed");
            println!("Task: {}", task);
            println!("Error: {}", error);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_and_validate() {
        assert!(parse_and_validate("42").is_ok());
        assert!(parse_and_validate("0").is_err());
        assert!(parse_and_validate("abc").is_err());
    }
    
    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 2), Ok(5));
        assert!(divide(10, 0).is_err());
    }
}
