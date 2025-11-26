// Boolean Calculator
// Evaluates boolean expressions and performs boolean operations

use std::io::{self, Write};
use std::collections::HashMap;

fn main() {
    println!("=== Boolean Calculator ===\n");
    
    loop {
        println!("\nChoose a mode:");
        println!("1. Quick Evaluation");
        println!("2. Variable-Based Evaluation");
        println!("3. Expression Simplifier");
        println!("4. Boolean Algebra Calculator");
        println!("5. Truth Table Generator");
        println!("6. Logic Problem Solver");
        println!("7. Exit");
        print!("\nYour choice: ");
        io::stdout().flush().unwrap();
        
        let choice = read_number();
        
        match choice {
            1 => quick_evaluation(),
            2 => variable_evaluation(),
            3 => expression_simplifier(),
            4 => boolean_algebra(),
            5 => truth_table_generator(),
            6 => logic_problem_solver(),
            7 => {
                println!("\nGoodbye!");
                break;
            }
            _ => println!("Invalid choice!"),
        }
    }
}

fn quick_evaluation() {
    println!("\n=== Quick Evaluation ===");
    
    println!("\nEnter boolean values:");
    print!("A = ");
    io::stdout().flush().unwrap();
    let a = read_bool();
    
    print!("B = ");
    io::stdout().flush().unwrap();
    let b = read_bool();
    
    println!("\nAvailable expressions:");
    println!("1. A AND B");
    println!("2. A OR B");
    println!("3. NOT A");
    println!("4. A XOR B");
    println!("5. A NAND B");
    println!("6. A NOR B");
    println!("7. !(A AND B) OR (A AND !B)");
    println!("8. (A OR B) AND !(A AND B)");
    print!("\nChoice: ");
    io::stdout().flush().unwrap();
    
    let choice = read_number();
    
    let (result, expr) = match choice {
        1 => (a && b, "A AND B"),
        2 => (a || b, "A OR B"),
        3 => (!a, "NOT A"),
        4 => (a != b, "A XOR B"),
        5 => (!(a && b), "A NAND B"),
        6 => (!(a || b), "A NOR B"),
        7 => (!(a && b) || (a && !b), "!(A AND B) OR (A AND !B)"),
        8 => ((a || b) && !(a && b), "(A OR B) AND !(A AND B)"),
        _ => {
            println!("Invalid choice!");
            return;
        }
    };
    
    println!("\n📊 Evaluation:");
    println!("   Expression: {}", expr);
    println!("   A = {}, B = {}", a, b);
    println!("   Result: {}", result);
}

fn variable_evaluation() {
    println!("\n=== Variable-Based Evaluation ===");
    
    let mut vars = HashMap::new();
    
    loop {
        println!("\n📝 Current variables:");
        if vars.is_empty() {
            println!("   (None defined)");
        } else {
            for (name, value) in &vars {
                println!("   {} = {}", name, value);
            }
        }
        
        println!("\nOptions:");
        println!("1. Set variable");
        println!("2. Evaluate expression");
        println!("3. Clear variables");
        println!("4. Back");
        print!("\nChoice: ");
        io::stdout().flush().unwrap();
        
        match read_number() {
            1 => {
                print!("Variable name (A-Z): ");
                io::stdout().flush().unwrap();
                let name = read_string().to_uppercase();
                
                print!("Value (true/false): ");
                io::stdout().flush().unwrap();
                let value = read_bool();
                
                vars.insert(name.clone(), value);
                println!("✅ Set {} = {}", name, value);
            }
            2 => {
                if vars.is_empty() {
                    println!("❌ No variables defined!");
                    continue;
                }
                
                println!("\nExample expressions:");
                println!("  - A AND B");
                println!("  - A OR NOT B");
                println!("  - (A OR B) AND C");
                println!("  - A XOR B");
                
                print!("\nEnter expression: ");
                io::stdout().flush().unwrap();
                let expr = read_string();
                
                match evaluate_expression(&expr, &vars) {
                    Ok(result) => println!("✅ Result: {}", result),
                    Err(e) => println!("❌ Error: {}", e),
                }
            }
            3 => {
                vars.clear();
                println!("✅ Variables cleared");
            }
            4 => break,
            _ => println!("Invalid choice!"),
        }
    }
}

fn evaluate_expression(expr: &str, vars: &HashMap<String, bool>) -> Result<bool, String> {
    let expr = expr.to_uppercase();
    let tokens: Vec<&str> = expr.split_whitespace().collect();
    
    if tokens.is_empty() {
        return Err("Empty expression".to_string());
    }
    
    // Simple evaluation (supports basic AND, OR, NOT, XOR)
    let mut result = None;
    let mut current_op = None;
    let mut negate_next = false;
    
    for token in tokens {
        match token {
            "AND" | "&&" => current_op = Some("AND"),
            "OR" | "||" => current_op = Some("OR"),
            "XOR" => current_op = Some("XOR"),
            "NOT" | "!" => negate_next = true,
            _ => {
                // Try to get variable value
                let value = vars.get(token)
                    .ok_or(format!("Unknown variable: {}", token))?;
                
                let mut val = *value;
                if negate_next {
                    val = !val;
                    negate_next = false;
                }
                
                result = match (result, current_op.as_deref()) {
                    (None, _) => Some(val),
                    (Some(prev), Some("AND")) => Some(prev && val),
                    (Some(prev), Some("OR")) => Some(prev || val),
                    (Some(prev), Some("XOR")) => Some(prev != val),
                    _ => return Err("Invalid expression".to_string()),
                };
                
                current_op = None;
            }
        }
    }
    
    result.ok_or("Could not evaluate expression".to_string())
}

fn expression_simplifier() {
    println!("\n=== Expression Simplifier ===");
    
    println!("\nCommon simplifications:");
    println!("1. A AND A → A");
    println!("2. A OR A → A");
    println!("3. A AND true → A");
    println!("4. A OR false → A");
    println!("5. A AND false → false");
    println!("6. A OR true → true");
    println!("7. NOT NOT A → A");
    println!("8. A AND NOT A → false");
    println!("9. A OR NOT A → true");
    println!("10. De Morgan's Laws");
    print!("\nChoice: ");
    io::stdout().flush().unwrap();
    
    let choice = read_number();
    
    print!("\nEnter A (true/false): ");
    io::stdout().flush().unwrap();
    let a = read_bool();
    
    match choice {
        1 => {
            println!("\n📐 Simplification: A AND A = A");
            println!("   {} AND {} = {}", a, a, a && a);
            println!("   Simplified: {}", a);
            println!("   ✅ Verified: {} = {}", a && a, a);
        }
        2 => {
            println!("\n📐 Simplification: A OR A = A");
            println!("   {} OR {} = {}", a, a, a || a);
            println!("   Simplified: {}", a);
            println!("   ✅ Verified: {} = {}", a || a, a);
        }
        3 => {
            println!("\n📐 Simplification: A AND true = A");
            println!("   {} AND true = {}", a, a && true);
            println!("   Simplified: {}", a);
            println!("   ✅ Verified: {} = {}", a && true, a);
        }
        4 => {
            println!("\n📐 Simplification: A OR false = A");
            println!("   {} OR false = {}", a, a || false);
            println!("   Simplified: {}", a);
            println!("   ✅ Verified: {} = {}", a || false, a);
        }
        5 => {
            println!("\n📐 Simplification: A AND false = false");
            println!("   {} AND false = {}", a, a && false);
            println!("   Simplified: false");
            println!("   ✅ Verified: {} = false", a && false);
        }
        6 => {
            println!("\n📐 Simplification: A OR true = true");
            println!("   {} OR true = {}", a, a || true);
            println!("   Simplified: true");
            println!("   ✅ Verified: {} = true", a || true);
        }
        7 => {
            println!("\n📐 Simplification: NOT NOT A = A");
            println!("   NOT NOT {} = {}", a, !!a);
            println!("   Simplified: {}", a);
            println!("   ✅ Verified: {} = {}", !!a, a);
        }
        8 => {
            println!("\n📐 Simplification: A AND NOT A = false");
            println!("   {} AND NOT {} = {}", a, a, a && !a);
            println!("   Simplified: false");
            println!("   ✅ Verified: {} = false", a && !a);
        }
        9 => {
            println!("\n📐 Simplification: A OR NOT A = true");
            println!("   {} OR NOT {} = {}", a, a, a || !a);
            println!("   Simplified: true");
            println!("   ✅ Verified: {} = true", a || !a);
        }
        10 => {
            print!("Enter B (true/false): ");
            io::stdout().flush().unwrap();
            let b = read_bool();
            
            println!("\n📐 De Morgan's Laws:");
            println!("   Law 1: NOT(A AND B) = NOT A OR NOT B");
            println!("   NOT({} AND {}) = {}", a, b, !(a && b));
            println!("   NOT {} OR NOT {} = {}", a, b, !a || !b);
            println!("   ✅ Equal: {}", !(a && b) == (!a || !b));
            
            println!("\n   Law 2: NOT(A OR B) = NOT A AND NOT B");
            println!("   NOT({} OR {}) = {}", a, b, !(a || b));
            println!("   NOT {} AND NOT {} = {}", a, b, !a && !b);
            println!("   ✅ Equal: {}", !(a || b) == (!a && !b));
        }
        _ => println!("Invalid choice!"),
    }
}

fn boolean_algebra() {
    println!("\n=== Boolean Algebra Calculator ===");
    
    print!("Enter A: ");
    io::stdout().flush().unwrap();
    let a = read_bool();
    
    print!("Enter B: ");
    io::stdout().flush().unwrap();
    let b = read_bool();
    
    print!("Enter C: ");
    io::stdout().flush().unwrap();
    let c = read_bool();
    
    println!("\n🧮 Boolean Algebra Operations:");
    println!("═══════════════════════════════");
    
    // Distributive
    println!("\n📐 Distributive Law:");
    let dist1_left = a && (b || c);
    let dist1_right = (a && b) || (a && c);
    println!("   A AND (B OR C) = {} AND ({} OR {}) = {}", a, b, c, dist1_left);
    println!("   (A AND B) OR (A AND C) = ({} AND {}) OR ({} AND {}) = {}", 
        a, b, a, c, dist1_right);
    println!("   ✅ Equal: {}", dist1_left == dist1_right);
    
    // Absorption
    println!("\n📐 Absorption Law:");
    let abs1 = a || (a && b);
    println!("   A OR (A AND B) = {} OR ({} AND {}) = {}", a, a, b, abs1);
    println!("   Simplified: {}", a);
    println!("   ✅ Equal: {}", abs1 == a);
    
    // Consensus
    println!("\n📐 Consensus Theorem:");
    let cons = (a && b) || (!a && c) || (b && c);
    let cons_simplified = (a && b) || (!a && c);
    println!("   (A AND B) OR (NOT A AND C) OR (B AND C)");
    println!("   = ({} AND {}) OR (NOT {} AND {}) OR ({} AND {})", a, b, a, c, b, c);
    println!("   = {}", cons);
    println!("\n   Simplified: (A AND B) OR (NOT A AND C)");
    println!("   = {}", cons_simplified);
    println!("   ✅ Equal: {}", cons == cons_simplified);
}

fn truth_table_generator() {
    println!("\n=== Truth Table Generator ===");
    
    println!("\nSelect expression to generate truth table:");
    println!("1. (A AND B) OR C");
    println!("2. A XOR B XOR C");
    println!("3. (A OR B) AND (B OR C)");
    println!("4. Majority function (at least 2 true)");
    print!("\nChoice: ");
    io::stdout().flush().unwrap();
    
    let choice = read_number();
    
    let (func, desc): (fn(bool, bool, bool) -> bool, &str) = match choice {
        1 => ((|a, b, c| (a && b) || c) as fn(bool, bool, bool) -> bool, "(A AND B) OR C"),
        2 => ((|a, b, c| (a != b) != c) as fn(bool, bool, bool) -> bool, "A XOR B XOR C"),
        3 => ((|a, b, c| (a || b) && (b || c)) as fn(bool, bool, bool) -> bool, "(A OR B) AND (B OR C)"),
        4 => ((|a, b, c| {
            let count = (a as i32) + (b as i32) + (c as i32);
            count >= 2
        }) as fn(bool, bool, bool) -> bool, "Majority(A, B, C)"),
        _ => {
            println!("Invalid choice!");
            return;
        }
    };
    
    println!("\n┌───────┬───────┬───────┬──────────┐");
    println!("│   A   │   B   │   C   │ {} │", desc);
    println!("├───────┼───────┼───────┼──────────┤");
    
    for a in [false, true] {
        for b in [false, true] {
            for c in [false, true] {
                let result = func(a, b, c);
                println!("│ {:<5} │ {:<5} │ {:<5} │ {:<8} │", a, b, c, result);
            }
        }
    }
    
    println!("└───────┴───────┴───────┴──────────┘");
}

fn logic_problem_solver() {
    println!("\n=== Logic Problem Solver ===");
    
    println!("\nSample problem:");
    println!("Three friends: Alice, Bob, and Charlie");
    println!("At least one tells the truth");
    println!("At least one lies");
    
    println!("\nStatements:");
    println!("Alice: 'Bob is lying'");
    println!("Bob: 'Charlie is telling the truth'");
    println!("Charlie: 'Alice is lying'");
    
    println!("\nLet's evaluate all possibilities:");
    println!("A=Alice truth, B=Bob truth, C=Charlie truth");
    
    println!("\n┌─────┬─────┬─────┬──────────┬────────┬────────────┬───────┐");
    println!("│  A  │  B  │  C  │ Alice OK │ Bob OK │ Charlie OK │ Valid │");
    println!("├─────┼─────┼─────┼──────────┼────────┼────────────┼───────┤");
    
    for a in [false, true] {
        for b in [false, true] {
            for c in [false, true] {
                // Alice says Bob is lying
                let alice_ok = a == !b;
                
                // Bob says Charlie is telling the truth
                let bob_ok = b == c;
                
                // Charlie says Alice is lying
                let charlie_ok = c == !a;
                
                // Valid if all statements match their truth values
                let valid = alice_ok && bob_ok && charlie_ok;
                
                // And at least one truth and one lie
                let at_least_one_truth = a || b || c;
                let at_least_one_lie = !a || !b || !c;
                let constraint_ok = at_least_one_truth && at_least_one_lie;
                
                let final_valid = valid && constraint_ok;
                
                println!(
                    "│ {:^3} │ {:^3} │ {:^3} │ {:^8} │ {:^6} │ {:^10} │ {:^5} │",
                    a, b, c, alice_ok, bob_ok, charlie_ok, final_valid
                );
            }
        }
    }
    
    println!("└─────┴─────┴─────┴──────────┴────────┴────────────┴───────┘");
    
    println!("\n✅ Solution: A=true, B=false, C=false");
    println!("   Alice tells the truth (Bob is lying) ✓");
    println!("   Bob lies (Charlie is not telling truth) ✓");
    println!("   Charlie lies (Alice is not lying) ✓");
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
    fn test_de_morgans() {
        for a in [false, true] {
            for b in [false, true] {
                assert_eq!(!(a && b), !a || !b);
                assert_eq!(!(a || b), !a && !b);
            }
        }
    }
    
    #[test]
    fn test_distributive() {
        for a in [false, true] {
            for b in [false, true] {
                for c in [false, true] {
                    assert_eq!(a && (b || c), (a && b) || (a && c));
                    assert_eq!(a || (b && c), (a || b) && (a || c));
                }
            }
        }
    }
    
    #[test]
    fn test_absorption() {
        for a in [false, true] {
            for b in [false, true] {
                assert_eq!(a || (a && b), a);
                assert_eq!(a && (a || b), a);
            }
        }
    }
    
    #[test]
    fn test_expression_evaluator() {
        let mut vars = HashMap::new();
        vars.insert("A".to_string(), true);
        vars.insert("B".to_string(), false);
        
        assert_eq!(evaluate_expression("A AND B", &vars).unwrap(), false);
        assert_eq!(evaluate_expression("A OR B", &vars).unwrap(), true);
        assert_eq!(evaluate_expression("NOT A", &vars).unwrap(), false);
    }
}
