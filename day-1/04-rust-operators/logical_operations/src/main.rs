// Logical Operations
// Demonstrates logical operators and boolean expressions

use std::io::{self, Write};

fn main() {
    println!("=== Logical Operations Demonstrator ===\n");
    
    loop {
        println!("\nChoose a demonstration:");
        println!("1. Truth Tables");
        println!("2. Short-Circuit Evaluation");
        println!("3. Access Control Simulator");
        println!("4. Complex Conditions");
        println!("5. Boolean Algebra");
        println!("6. Exit");
        print!("\nYour choice: ");
        io::stdout().flush().unwrap();
        
        let choice = read_number();
        
        match choice {
            1 => truth_tables(),
            2 => short_circuit_demo(),
            3 => access_control(),
            4 => complex_conditions(),
            5 => boolean_algebra(),
            6 => {
                println!("\nGoodbye!");
                break;
            }
            _ => println!("Invalid choice!"),
        }
    }
}

fn truth_tables() {
    println!("\n=== Truth Tables ===");
    
    println!("\n--- AND (&&) Truth Table ---");
    println!("  A     | B     | A && B");
    println!("  ------|-------|-------");
    println!("  false | false | {}", false && false);
    println!("  false | true  | {}", false && true);
    println!("  true  | false | {}", true && false);
    println!("  true  | true  | {}", true && true);
    
    println!("\n--- OR (||) Truth Table ---");
    println!("  A     | B     | A || B");
    println!("  ------|-------|-------");
    println!("  false | false | {}", false || false);
    println!("  false | true  | {}", false || true);
    println!("  true  | false | {}", true || false);
    println!("  true  | true  | {}", true || true);
    
    println!("\n--- NOT (!) Truth Table ---");
    println!("  A     | !A");
    println!("  ------|------");
    println!("  false | {}", !false);
    println!("  true  | {}", !true);
    
    println!("\n--- XOR (Exclusive OR) ---");
    println!("  A     | B     | A XOR B");
    println!("  ------|-------|--------");
    println!("  false | false | {}", (false || false) && !(false && false));
    println!("  false | true  | {}", (false || true) && !(false && true));
    println!("  true  | false | {}", (true || false) && !(true && false));
    println!("  true  | true  | {}", (true || true) && !(true && true));
}

fn short_circuit_demo() {
    println!("\n=== Short-Circuit Evaluation ===");
    
    let mut call_count = 0;
    
    // Function to track calls
    let expensive_check = |count: &mut i32| -> bool {
        *count += 1;
        println!("  🔍 expensive_check() called (call #{})", *count);
        true
    };
    
    println!("\n--- AND (&&) Short-Circuit ---");
    println!("Testing: false && expensive_check()");
    call_count = 0;
    let result = false && expensive_check(&mut call_count);
    println!("  Result: {}, Calls: {}", result, call_count);
    println!("  ✅ Second operand NOT evaluated (short-circuited)");
    
    println!("\nTesting: true && expensive_check()");
    call_count = 0;
    let result = true && expensive_check(&mut call_count);
    println!("  Result: {}, Calls: {}", result, call_count);
    println!("  ⚡ Second operand WAS evaluated");
    
    println!("\n--- OR (||) Short-Circuit ---");
    println!("Testing: true || expensive_check()");
    call_count = 0;
    let result = true || expensive_check(&mut call_count);
    println!("  Result: {}, Calls: {}", result, call_count);
    println!("  ✅ Second operand NOT evaluated (short-circuited)");
    
    println!("\nTesting: false || expensive_check()");
    call_count = 0;
    let result = false || expensive_check(&mut call_count);
    println!("  Result: {}, Calls: {}", result, call_count);
    println!("  ⚡ Second operand WAS evaluated");
    
    // Practical example
    println!("\n--- Practical Usage ---");
    let data: Vec<i32> = vec![1, 2, 3];
    let index = 5;
    
    println!("Checking: index < data.len() && data[index] > 0");
    if index < data.len() && data[index] > 0 {
        println!("  ✅ Safe access");
    } else {
        println!("  ❌ Index out of bounds or invalid (prevented panic!)");
    }
}

fn access_control() {
    println!("\n=== Access Control Simulator ===");
    
    print!("\nEnter age: ");
    io::stdout().flush().unwrap();
    let age = read_number();
    
    print!("Has valid ID? (y/n): ");
    io::stdout().flush().unwrap();
    let has_id = read_bool();
    
    print!("Is VIP member? (y/n): ");
    io::stdout().flush().unwrap();
    let is_vip = read_bool();
    
    print!("Is weekday? (y/n): ");
    io::stdout().flush().unwrap();
    let is_weekday = read_bool();
    
    println!("\n--- Access Decisions ---");
    
    // Basic access: age 18+ AND has ID
    let basic_access = age >= 18 && has_id;
    println!("  Basic Access: {} {}", 
        if basic_access { "✅" } else { "❌" },
        if basic_access { "Granted" } else { "Denied" });
    
    // VIP access: (age 18+ AND has ID) OR is VIP
    let vip_access = (age >= 18 && has_id) || is_vip;
    println!("  VIP Access: {} {}", 
        if vip_access { "✅" } else { "❌" },
        if vip_access { "Granted" } else { "Denied" });
    
    // Weekend access: basic access AND NOT weekday
    let weekend_access = basic_access && !is_weekday;
    println!("  Weekend Access: {} {}", 
        if weekend_access { "✅" } else { "❌" },
        if weekend_access { "Granted" } else { "Denied" });
    
    // Premium access: age 21+ AND (has ID OR is VIP)
    let premium_access = age >= 21 && (has_id || is_vip);
    println!("  Premium Access: {} {}", 
        if premium_access { "✅" } else { "❌" },
        if premium_access { "Granted" } else { "Denied" });
    
    // Special event: (VIP OR weekend) AND age 16+
    let special_event = (is_vip || !is_weekday) && age >= 16;
    println!("  Special Event: {} {}", 
        if special_event { "✅" } else { "❌" },
        if special_event { "Granted" } else { "Denied" });
}

fn complex_conditions() {
    println!("\n=== Complex Conditions ===");
    
    print!("\nEnter temperature (°C): ");
    io::stdout().flush().unwrap();
    let temp = read_number();
    
    print!("Is raining? (y/n): ");
    io::stdout().flush().unwrap();
    let is_raining = read_bool();
    
    print!("Is windy? (y/n): ");
    io::stdout().flush().unwrap();
    let is_windy = read_bool();
    
    print!("Has umbrella? (y/n): ");
    io::stdout().flush().unwrap();
    let has_umbrella = read_bool();
    
    println!("\n--- Weather Recommendations ---");
    
    // Perfect weather
    if temp >= 20 && temp <= 25 && !is_raining && !is_windy {
        println!("  🌞 Perfect weather for outdoor activities!");
    }
    
    // Good weather
    else if temp >= 15 && temp <= 30 && (!is_raining || has_umbrella) {
        println!("  ☀️ Good weather, enjoy your day!");
        if is_raining && has_umbrella {
            println!("     Don't forget your umbrella!");
        }
    }
    
    // Rainy but manageable
    else if is_raining && has_umbrella && temp >= 10 {
        println!("  ☔ Rainy, but you have an umbrella - stay dry!");
    }
    
    // Bad weather
    else if (temp < 0 || temp > 35) || (is_raining && is_windy) {
        println!("  ❄️🌧️ Bad weather - consider staying indoors!");
        
        if temp < 0 {
            println!("     It's freezing!");
        }
        if temp > 35 {
            println!("     It's extremely hot!");
        }
        if is_raining && is_windy {
            println!("     Storm conditions!");
        }
    }
    
    // Moderate weather
    else {
        println!("  🌤️ Moderate weather conditions");
    }
    
    // Additional warnings
    if is_windy && temp < 10 {
        println!("  ⚠️  Wind chill warning!");
    }
    
    if is_raining && !has_umbrella {
        println!("  ⚠️  You'll get wet without an umbrella!");
    }
}

fn boolean_algebra() {
    println!("\n=== Boolean Algebra ===");
    
    let a = true;
    let b = false;
    
    println!("\nGiven: a = {}, b = {}", a, b);
    
    println!("\n--- De Morgan's Laws ---");
    println!("  !(a && b) = {}", !(a && b));
    println!("  !a || !b = {}", !a || !b);
    println!("  Equal? {}", !(a && b) == (!a || !b));
    
    println!("\n  !(a || b) = {}", !(a || b));
    println!("  !a && !b = {}", !a && !b);
    println!("  Equal? {}", !(a || b) == (!a && !b));
    
    println!("\n--- Associative Laws ---");
    let c = true;
    println!("  Given: c = {}", c);
    println!("  (a && b) && c = {}", (a && b) && c);
    println!("  a && (b && c) = {}", a && (b && c));
    println!("  Equal? {}", ((a && b) && c) == (a && (b && c)));
    
    println!("\n--- Distributive Laws ---");
    println!("  a && (b || c) = {}", a && (b || c));
    println!("  (a && b) || (a && c) = {}", (a && b) || (a && c));
    println!("  Equal? {}", (a && (b || c)) == ((a && b) || (a && c)));
    
    println!("\n--- Identity Laws ---");
    println!("  a && true = {}", a && true);
    println!("  a || false = {}", a || false);
    println!("  Both equal a? {}", (a && true) == a && (a || false) == a);
    
    println!("\n--- Negation Laws ---");
    println!("  a && !a = {}", a && !a);
    println!("  a || !a = {}", a || !a);
    println!("  !!a = {}", !!a);
    println!("  Equal to a? {}", !!a == a);
}

fn read_number() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().unwrap_or(0)
}

fn read_bool() -> bool {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    matches!(input.trim().to_lowercase().as_str(), "y" | "yes" | "true" | "1")
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_logical_operators() {
        assert!(true && true);
        assert!(!(true && false));
        assert!(true || false);
        assert!(!false);
    }
    
    #[test]
    fn test_de_morgans_laws() {
        let a = true;
        let b = false;
        
        assert_eq!(!(a && b), !a || !b);
        assert_eq!(!(a || b), !a && !b);
    }
    
    #[test]
    fn test_short_circuit() {
        let mut called = false;
        let check = || { called = true; true };
        
        // Should not call check
        let _ = false && check();
        assert!(!called);
        
        // Should call check
        let _ = true && check();
        assert!(called);
    }
}
