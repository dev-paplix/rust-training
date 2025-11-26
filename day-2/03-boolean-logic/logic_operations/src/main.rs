// Logic Operations
// Demonstrates basic boolean operations and their properties

use std::io::{self, Write};

fn main() {
    println!("=== Logic Operations ===\n");
    
    loop {
        println!("\nChoose a demonstration:");
        println!("1. Basic Operators (AND, OR, NOT, XOR)");
        println!("2. Comparison Operators");
        println!("3. Short-Circuit Evaluation");
        println!("4. Operator Precedence");
        println!("5. Boolean Algebra Laws");
        println!("6. Practical Examples");
        println!("7. Exit");
        print!("\nYour choice: ");
        io::stdout().flush().unwrap();
        
        let choice = read_number();
        
        match choice {
            1 => basic_operators(),
            2 => comparison_operators(),
            3 => short_circuit(),
            4 => operator_precedence(),
            5 => boolean_laws(),
            6 => practical_examples(),
            7 => {
                println!("\nGoodbye!");
                break;
            }
            _ => println!("Invalid choice!"),
        }
    }
}

fn basic_operators() {
    println!("\n=== Basic Boolean Operators ===");
    
    print!("Enter first boolean (true/false): ");
    io::stdout().flush().unwrap();
    let a = read_bool();
    
    print!("Enter second boolean (true/false): ");
    io::stdout().flush().unwrap();
    let b = read_bool();
    
    println!("\n📊 Results for A={}, B={}:", a, b);
    println!("─────────────────────────────");
    
    // AND
    let and_result = a && b;
    println!("A AND B  (A && B)  = {}", and_result);
    println!("  ↳ Both must be true");
    
    // OR
    let or_result = a || b;
    println!("\nA OR B   (A || B)  = {}", or_result);
    println!("  ↳ At least one must be true");
    
    // NOT
    let not_a = !a;
    let not_b = !b;
    println!("\nNOT A    (!A)      = {}", not_a);
    println!("NOT B    (!B)      = {}", not_b);
    println!("  ↳ Inverts the value");
    
    // XOR (exclusive or)
    let xor_result = a != b;
    println!("\nA XOR B  (A != B)  = {}", xor_result);
    println!("  ↳ Exactly one must be true");
    
    // NAND (NOT AND)
    let nand_result = !(a && b);
    println!("\nA NAND B !(A && B) = {}", nand_result);
    println!("  ↳ NOT of AND");
    
    // NOR (NOT OR)
    let nor_result = !(a || b);
    println!("\nA NOR B  !(A || B) = {}", nor_result);
    println!("  ↳ NOT of OR");
    
    // XNOR (NOT XOR)
    let xnor_result = a == b;
    println!("\nA XNOR B (A == B)  = {}", xnor_result);
    println!("  ↳ Both same (equivalence)");
}

fn comparison_operators() {
    println!("\n=== Comparison Operators ===");
    println!("All comparison operators return boolean values\n");
    
    print!("Enter first number: ");
    io::stdout().flush().unwrap();
    let a = read_number();
    
    print!("Enter second number: ");
    io::stdout().flush().unwrap();
    let b = read_number();
    
    println!("\n📊 Comparing {} and {}:", a, b);
    println!("────────────────────────");
    
    println!("{} == {}  →  {}", a, b, a == b);
    println!("{} != {}  →  {}", a, b, a != b);
    println!("{} <  {}  →  {}", a, b, a < b);
    println!("{} >  {}  →  {}", a, b, a > b);
    println!("{} <= {}  →  {}", a, b, a <= b);
    println!("{} >= {}  →  {}", a, b, a >= b);
    
    // String comparison
    print!("\nEnter first word: ");
    io::stdout().flush().unwrap();
    let word1 = read_string();
    
    print!("Enter second word: ");
    io::stdout().flush().unwrap();
    let word2 = read_string();
    
    println!("\n📊 Comparing '{}' and '{}':", word1, word2);
    println!("────────────────────────────");
    println!("Equal:        {}", word1 == word2);
    println!("Not equal:    {}", word1 != word2);
    println!("Alphabetically less: {}", word1 < word2);
    
    // Boolean comparison
    println!("\n📊 Boolean Comparisons:");
    println!("────────────────────────");
    println!("true == true   →  {}", true == true);
    println!("true == false  →  {}", true == false);
    println!("true > false   →  {}", true > false);
    println!("false < true   →  {}", false < true);
}

fn short_circuit() {
    println!("\n=== Short-Circuit Evaluation ===");
    println!("Operators && and || use short-circuit evaluation\n");
    
    let mut counter = 0;
    
    // Helper closure that increments counter
    let increment = || {
        counter += 1;
        println!("  → Function called! (counter = {})", counter);
        true
    };
    
    println!("1️⃣  Testing: false && increment()");
    counter = 0;
    let result = false && increment();
    println!("   Result: {}", result);
    println!("   Counter: {} (function NOT called due to short-circuit)", counter);
    
    println!("\n2️⃣  Testing: true && increment()");
    counter = 0;
    let result = true && increment();
    println!("   Result: {}", result);
    println!("   Counter: {} (function WAS called)", counter);
    
    println!("\n3️⃣  Testing: true || increment()");
    counter = 0;
    let result = true || increment();
    println!("   Result: {}", result);
    println!("   Counter: {} (function NOT called due to short-circuit)", counter);
    
    println!("\n4️⃣  Testing: false || increment()");
    counter = 0;
    let result = false || increment();
    println!("   Result: {}", result);
    println!("   Counter: {} (function WAS called)", counter);
    
    println!("\n💡 Short-circuit evaluation:");
    println!("   • AND (&&): Stops at first false");
    println!("   • OR (||):  Stops at first true");
    println!("   • Prevents unnecessary computation");
    println!("   • Prevents errors (e.g., division by zero)");
    
    // Practical example
    println!("\n📝 Practical Example:");
    print!("Enter a divisor (0 for division by zero): ");
    io::stdout().flush().unwrap();
    let divisor = read_number();
    
    // Safe division using short-circuit
    if divisor != 0 && 100 % divisor == 0 {
        println!("✅ 100 is evenly divisible by {}", divisor);
    } else if divisor != 0 {
        println!("⚠️  100 is not evenly divisible by {}", divisor);
    } else {
        println!("❌ Cannot divide by zero");
    }
}

fn operator_precedence() {
    println!("\n=== Operator Precedence ===");
    println!("Understanding how operators are evaluated\n");
    
    println!("📋 Precedence Order (highest to lowest):");
    println!("   1. NOT    (!)");
    println!("   2. Comparison (==, !=, <, >, <=, >=)");
    println!("   3. AND    (&&)");
    println!("   4. OR     (||)");
    
    print!("\nEnter value for A (true/false): ");
    io::stdout().flush().unwrap();
    let a = read_bool();
    
    print!("Enter value for B (true/false): ");
    io::stdout().flush().unwrap();
    let b = read_bool();
    
    print!("Enter value for C (true/false): ");
    io::stdout().flush().unwrap();
    let c = read_bool();
    
    println!("\nA={}, B={}, C={}", a, b, c);
    println!("─────────────────────────");
    
    // Example 1: NOT has highest precedence
    let expr1 = !a && b;
    println!("\n!A && B");
    println!("  Step 1: !{} = {}", a, !a);
    println!("  Step 2: {} && {} = {}", !a, b, expr1);
    println!("  Result: {}", expr1);
    
    // Example 2: AND before OR
    let expr2 = a || b && c;
    println!("\nA || B && C");
    println!("  Step 1: {} && {} = {}", b, c, b && c);
    println!("  Step 2: {} || {} = {}", a, b && c, expr2);
    println!("  Result: {}", expr2);
    
    // Example 3: Using parentheses
    let expr3 = (a || b) && c;
    println!("\n(A || B) && C");
    println!("  Step 1: {} || {} = {}", a, b, a || b);
    println!("  Step 2: {} && {} = {}", a || b, c, expr3);
    println!("  Result: {}", expr3);
    
    println!("\n💡 Use parentheses for clarity!");
}

fn boolean_laws() {
    println!("\n=== Boolean Algebra Laws ===");
    
    print!("Enter value for A (true/false): ");
    io::stdout().flush().unwrap();
    let a = read_bool();
    
    print!("Enter value for B (true/false): ");
    io::stdout().flush().unwrap();
    let b = read_bool();
    
    println!("\nA={}, B={}", a, b);
    
    println!("\n1️⃣  Identity Laws:");
    println!("   A && true  = {} && true  = {}", a, a && true);
    println!("   A || false = {} || false = {}", a, a || false);
    
    println!("\n2️⃣  Domination Laws:");
    println!("   A && false = {} && false = {}", a, a && false);
    println!("   A || true  = {} || true  = {}", a, a || true);
    
    println!("\n3️⃣  Idempotent Laws:");
    println!("   A && A     = {} && {}     = {}", a, a, a && a);
    println!("   A || A     = {} || {}     = {}", a, a, a || a);
    
    println!("\n4️⃣  Complement Laws:");
    println!("   A && !A    = {} && {}    = {}", a, !a, a && !a);
    println!("   A || !A    = {} || {}    = {}", a, !a, a || !a);
    
    println!("\n5️⃣  Double Negation:");
    println!("   !!A        = !!{}        = {}", a, !!a);
    
    println!("\n6️⃣  De Morgan's Laws:");
    println!("   !(A && B)  = !({} && {})  = {}", a, b, !(a && b));
    println!("   !A || !B   = !{} || !{}   = {}", a, b, !a || !b);
    println!("   Equal? {}", !(a && b) == (!a || !b));
    
    println!("\n   !(A || B)  = !({} || {})  = {}", a, b, !(a || b));
    println!("   !A && !B   = !{} && !{}   = {}", a, b, !a && !b);
    println!("   Equal? {}", !(a || b) == (!a && !b));
    
    println!("\n7️⃣  Commutative Laws:");
    println!("   A && B     = {} && {}     = {}", a, b, a && b);
    println!("   B && A     = {} && {}     = {}", b, a, b && a);
    println!("   Equal? {}", (a && b) == (b && a));
    
    println!("\n8️⃣  Associative Laws:");
    let c = true;
    println!("   (A && B) && C = ({} && {}) && {} = {}", a, b, c, (a && b) && c);
    println!("   A && (B && C) = {} && ({} && {}) = {}", a, b, c, a && (b && c));
    println!("   Equal? {}", ((a && b) && c) == (a && (b && c)));
}

fn practical_examples() {
    println!("\n=== Practical Examples ===");
    
    println!("\n1️⃣  Access Control System");
    print("Is user logged in? (true/false): ");
    io::stdout().flush().unwrap();
    let logged_in = read_bool();
    
    print!("Is user admin? (true/false): ");
    io::stdout().flush().unwrap();
    let is_admin = read_bool();
    
    print!("Is user banned? (true/false): ");
    io::stdout().flush().unwrap();
    let is_banned = read_bool();
    
    let can_access = logged_in && !is_banned;
    let can_moderate = can_access && is_admin;
    
    println!("\n   Can access site: {}", can_access);
    println!("   Can moderate: {}", can_moderate);
    
    println!("\n2️⃣  Form Validation");
    print!("Enter age: ");
    io::stdout().flush().unwrap();
    let age = read_number();
    
    print!("Has parental consent? (true/false): ");
    io::stdout().flush().unwrap();
    let has_consent = read_bool();
    
    let is_adult = age >= 18;
    let can_register = is_adult || (age >= 13 && has_consent);
    
    println!("\n   Is adult: {}", is_adult);
    println!("   Can register: {}", can_register);
    
    if !can_register {
        if age < 13 {
            println!("   ❌ Too young (minimum age is 13)");
        } else {
            println!("   ❌ Need parental consent (under 18)");
        }
    } else {
        println!("   ✅ Registration allowed");
    }
    
    println!("\n3️⃣  Discount Calculator");
    print!("Is member? (true/false): ");
    io::stdout().flush().unwrap();
    let is_member = read_bool();
    
    print!("Is first purchase? (true/false): ");
    io::stdout().flush().unwrap();
    let is_first = read_bool();
    
    print!("Purchase amount: $");
    io::stdout().flush().unwrap();
    let amount = read_number();
    
    let over_100 = amount >= 100;
    let gets_discount = is_member || is_first || over_100;
    
    let discount_percent = if is_member && over_100 {
        20
    } else if is_member || over_100 {
        10
    } else if is_first {
        15
    } else {
        0
    };
    
    println!("\n   Gets discount: {}", gets_discount);
    println!("   Discount: {}%", discount_percent);
    println!("   Final price: ${}", amount - (amount * discount_percent / 100));
}

fn read_number() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().unwrap_or(0)
}

fn read_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn read_bool() -> bool {
    let input = read_string().to_lowercase();
    input == "true" || input == "t" || input == "1" || input == "yes" || input == "y"
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_and_operator() {
        assert_eq!(true && true, true);
        assert_eq!(true && false, false);
        assert_eq!(false && true, false);
        assert_eq!(false && false, false);
    }
    
    #[test]
    fn test_or_operator() {
        assert_eq!(true || true, true);
        assert_eq!(true || false, true);
        assert_eq!(false || true, true);
        assert_eq!(false || false, false);
    }
    
    #[test]
    fn test_not_operator() {
        assert_eq!(!true, false);
        assert_eq!(!false, true);
        assert_eq!(!!true, true);
    }
    
    #[test]
    fn test_xor() {
        assert_eq!(true != true, false);
        assert_eq!(true != false, true);
        assert_eq!(false != true, true);
        assert_eq!(false != false, false);
    }
    
    #[test]
    fn test_de_morgans() {
        let a = true;
        let b = false;
        assert_eq!(!(a && b), !a || !b);
        assert_eq!(!(a || b), !a && !b);
    }
}
