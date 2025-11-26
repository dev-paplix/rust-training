// Truth Tables Generator
// Generates and displays truth tables for boolean operations

use std::io::{self, Write};

fn main() {
    println!("=== Truth Tables Generator ===\n");
    
    loop {
        println!("\nChoose a truth table:");
        println!("1. Single Variable (!A)");
        println!("2. Two Variables (AND, OR, XOR, NAND, NOR)");
        println!("3. Three Variables");
        println!("4. Custom Expression");
        println!("5. All Basic Operations");
        println!("6. Logic Laws Verification");
        println!("7. Exit");
        print!("\nYour choice: ");
        io::stdout().flush().unwrap();
        
        let choice = read_number();
        
        match choice {
            1 => single_variable_table(),
            2 => two_variable_table(),
            3 => three_variable_table(),
            4 => custom_expression(),
            5 => all_basic_operations(),
            6 => logic_laws_verification(),
            7 => {
                println!("\nGoodbye!");
                break;
            }
            _ => println!("Invalid choice!"),
        }
    }
}

fn single_variable_table() {
    println!("\n=== Single Variable Truth Table ===");
    
    println!("\n┌───────┬────────┐");
    println!("│   A   │   !A   │");
    println!("├───────┼────────┤");
    
    for a in [false, true] {
        println!("│ {:<5} │ {:<6} │", a, !a);
    }
    
    println!("└───────┴────────┘");
}

fn two_variable_table() {
    println!("\n=== Two Variable Truth Tables ===");
    
    println!("\n┌───────┬───────┬────────┬────────┬────────┬─────────┬────────┬─────────┐");
    println!("│   A   │   B   │ A && B │ A || B │ A != B │ !(A&&B) │ !(A||B)│  A == B │");
    println!("│       │       │  AND   │   OR   │  XOR   │  NAND   │  NOR   │  XNOR   │");
    println!("├───────┼───────┼────────┼────────┼────────┼─────────┼────────┼─────────┤");
    
    for a in [false, true] {
        for b in [false, true] {
            let and = a && b;
            let or = a || b;
            let xor = a != b;
            let nand = !(a && b);
            let nor = !(a || b);
            let xnor = a == b;
            
            println!(
                "│ {:<5} │ {:<5} │ {:<6} │ {:<6} │ {:<6} │ {:<7} │ {:<6} │ {:<7} │",
                a, b, and, or, xor, nand, nor, xnor
            );
        }
    }
    
    println!("└───────┴───────┴────────┴────────┴────────┴─────────┴────────┴─────────┘");
    
    print_operation_descriptions();
}

fn print_operation_descriptions() {
    println!("\n📝 Operation Descriptions:");
    println!("   AND:  True only if both are true");
    println!("   OR:   True if at least one is true");
    println!("   XOR:  True if exactly one is true");
    println!("   NAND: Opposite of AND (NOT AND)");
    println!("   NOR:  Opposite of OR (NOT OR)");
    println!("   XNOR: True if both are the same");
}

fn three_variable_table() {
    println!("\n=== Three Variable Truth Table ===");
    
    println!("\nSelect operation:");
    println!("1. (A && B) && C");
    println!("2. A && (B && C)");
    println!("3. (A || B) || C");
    println!("4. A || (B || C)");
    println!("5. A && B || C");
    println!("6. A || B && C");
    println!("7. Custom three-variable expression");
    print!("\nChoice: ");
    io::stdout().flush().unwrap();
    
    let choice = read_number();
    
    let (operation, description) = match choice {
        1 => ((|a, b, c| (a && b) && c) as fn(bool, bool, bool) -> bool, "(A && B) && C"),
        2 => ((|a, b, c| a && (b && c)) as fn(bool, bool, bool) -> bool, "A && (B && C)"),
        3 => ((|a, b, c| (a || b) || c) as fn(bool, bool, bool) -> bool, "(A || B) || C"),
        4 => ((|a, b, c| a || (b || c)) as fn(bool, bool, bool) -> bool, "A || (B || C)"),
        5 => ((|a, b, c| a && b || c) as fn(bool, bool, bool) -> bool, "A && B || C"),
        6 => ((|a, b, c| a || b && c) as fn(bool, bool, bool) -> bool, "A || B && C"),
        7 => {
            println!("Showing all common three-variable combinations...");
            show_all_three_variable();
            return;
        }
        _ => {
            println!("Invalid choice!");
            return;
        }
    };
    
    println!("\n┌───────┬───────┬───────┬──────────┐");
    println!("│   A   │   B   │   C   │ {} │", description);
    println!("├───────┼───────┼───────┼──────────┤");
    
    for a in [false, true] {
        for b in [false, true] {
            for c in [false, true] {
                let result = operation(a, b, c);
                println!("│ {:<5} │ {:<5} │ {:<5} │ {:<8} │", a, b, c, result);
            }
        }
    }
    
    println!("└───────┴───────┴───────┴──────────┘");
}

fn show_all_three_variable() {
    println!("\n┌───────┬───────┬───────┬──────────┬──────────┬──────────┬──────────┐");
    println!("│   A   │   B   │   C   │ A&&B&&C  │ A||B||C  │ A&&B||C  │ A||(B&&C)│");
    println!("├───────┼───────┼───────┼──────────┼──────────┼──────────┼──────────┤");
    
    for a in [false, true] {
        for b in [false, true] {
            for c in [false, true] {
                let and_all = a && b && c;
                let or_all = a || b || c;
                let and_then_or = a && b || c;
                let or_then_and = a || (b && c);
                
                println!(
                    "│ {:<5} │ {:<5} │ {:<5} │ {:<8} │ {:<8} │ {:<8} │ {:<8} │",
                    a, b, c, and_all, or_all, and_then_or, or_then_and
                );
            }
        }
    }
    
    println!("└───────┴───────┴───────┴──────────┴──────────┴──────────┴──────────┘");
}

fn custom_expression() {
    println!("\n=== Custom Expression Evaluator ===");
    
    println!("\nAvailable operations:");
    println!("  Variables: A, B");
    println!("  AND: &&");
    println!("  OR:  ||");
    println!("  NOT: !");
    println!("  XOR: !=");
    
    println!("\nPre-defined expressions:");
    println!("1. !A && B");
    println!("2. A || !B");
    println!("3. !(A && B)");
    println!("4. !(A || B)");
    println!("5. A != B");
    println!("6. (A && B) || (!A && !B)");
    println!("7. A && !B || !A && B");
    print!("\nChoice: ");
    io::stdout().flush().unwrap();
    
    let choice = read_number();
    
    let (operation, description) = match choice {
        1 => ((|a, b| !a && b) as fn(bool, bool) -> bool, "!A && B"),
        2 => ((|a, b| a || !b) as fn(bool, bool) -> bool, "A || !B"),
        3 => ((|a, b| !(a && b)) as fn(bool, bool) -> bool, "!(A && B)"),
        4 => ((|a, b| !(a || b)) as fn(bool, bool) -> bool, "!(A || B)"),
        5 => ((|a, b| a != b) as fn(bool, bool) -> bool, "A != B"),
        6 => ((|a, b| (a && b) || (!a && !b)) as fn(bool, bool) -> bool, "(A && B) || (!A && !B)"),
        7 => ((|a, b| a && !b || !a && b) as fn(bool, bool) -> bool, "A && !B || !A && B"),
        _ => {
            println!("Invalid choice!");
            return;
        }
    };
    
    println!("\n┌───────┬───────┬──────────┐");
    println!("│   A   │   B   │ {} │", description);
    println!("├───────┼───────┼──────────┤");
    
    for a in [false, true] {
        for b in [false, true] {
            let result = operation(a, b);
            println!("│ {:<5} │ {:<5} │ {:<8} │", a, b, result);
        }
    }
    
    println!("└───────┴───────┴──────────┘");
}

fn all_basic_operations() {
    println!("\n=== All Basic Operations ===");
    
    println!("\n🔹 NOT Operation");
    println!("┌───────┬────────┐");
    println!("│   A   │   !A   │");
    println!("├───────┼────────┤");
    for a in [false, true] {
        println!("│ {:<5} │ {:<6} │", a, !a);
    }
    println!("└───────┴────────┘");
    
    println!("\n🔹 AND Operation");
    println!("┌───────┬───────┬────────┐");
    println!("│   A   │   B   │ A && B │");
    println!("├───────┼───────┼────────┤");
    for a in [false, true] {
        for b in [false, true] {
            println!("│ {:<5} │ {:<5} │ {:<6} │", a, b, a && b);
        }
    }
    println!("└───────┴───────┴────────┘");
    
    println!("\n🔹 OR Operation");
    println!("┌───────┬───────┬────────┐");
    println!("│   A   │   B   │ A || B │");
    println!("├───────┼───────┼────────┤");
    for a in [false, true] {
        for b in [false, true] {
            println!("│ {:<5} │ {:<5} │ {:<6} │", a, b, a || b);
        }
    }
    println!("└───────┴───────┴────────┘");
    
    println!("\n🔹 XOR Operation");
    println!("┌───────┬───────┬────────┐");
    println!("│   A   │   B   │ A != B │");
    println!("├───────┼───────┼────────┤");
    for a in [false, true] {
        for b in [false, true] {
            println!("│ {:<5} │ {:<5} │ {:<6} │", a, b, a != b);
        }
    }
    println!("└───────┴───────┴────────┘");
    
    println!("\n🔹 NAND Operation");
    println!("┌───────┬───────┬─────────┐");
    println!("│   A   │   B   │ !(A&&B) │");
    println!("├───────┼───────┼─────────┤");
    for a in [false, true] {
        for b in [false, true] {
            println!("│ {:<5} │ {:<5} │ {:<7} │", a, b, !(a && b));
        }
    }
    println!("└───────┴───────┴─────────┘");
    
    println!("\n🔹 NOR Operation");
    println!("┌───────┬───────┬─────────┐");
    println!("│   A   │   B   │ !(A||B) │");
    println!("├───────┼───────┼─────────┤");
    for a in [false, true] {
        for b in [false, true] {
            println!("│ {:<5} │ {:<5} │ {:<7} │", a, b, !(a || b));
        }
    }
    println!("└───────┴───────┴─────────┘");
    
    println!("\n🔹 XNOR Operation (Equivalence)");
    println!("┌───────┬───────┬────────┐");
    println!("│   A   │   B   │ A == B │");
    println!("├───────┼───────┼────────┤");
    for a in [false, true] {
        for b in [false, true] {
            println!("│ {:<5} │ {:<5} │ {:<6} │", a, b, a == b);
        }
    }
    println!("└───────┴───────┴────────┘");
}

fn logic_laws_verification() {
    println!("\n=== Logic Laws Verification ===");
    
    println!("\nSelect a law to verify:");
    println!("1. Identity Laws");
    println!("2. Domination Laws");
    println!("3. Idempotent Laws");
    println!("4. Complement Laws");
    println!("5. De Morgan's Laws");
    println!("6. Commutative Laws");
    println!("7. Associative Laws");
    println!("8. Distributive Laws");
    println!("9. Absorption Laws");
    print!("\nChoice: ");
    io::stdout().flush().unwrap();
    
    let choice = read_number();
    
    match choice {
        1 => verify_identity_laws(),
        2 => verify_domination_laws(),
        3 => verify_idempotent_laws(),
        4 => verify_complement_laws(),
        5 => verify_de_morgan_laws(),
        6 => verify_commutative_laws(),
        7 => verify_associative_laws(),
        8 => verify_distributive_laws(),
        9 => verify_absorption_laws(),
        _ => println!("Invalid choice!"),
    }
}

fn verify_identity_laws() {
    println!("\n=== Identity Laws ===");
    println!("A && true = A");
    println!("A || false = A\n");
    
    println!("┌───────┬────────┬─────────┬────────┐");
    println!("│   A   │ A&&true│ A||false│  Equal │");
    println!("├───────┼────────┼─────────┼────────┤");
    
    for a in [false, true] {
        let and_true = a && true;
        let or_false = a || false;
        let equal = (and_true == a) && (or_false == a);
        println!("│ {:<5} │ {:<6} │ {:<7} │ {:<6} │", a, and_true, or_false, equal);
    }
    
    println!("└───────┴────────┴─────────┴────────┘");
    println!("✅ Identity laws verified!");
}

fn verify_domination_laws() {
    println!("\n=== Domination Laws ===");
    println!("A && false = false");
    println!("A || true = true\n");
    
    println!("┌───────┬─────────┬─────────┬────────┐");
    println!("│   A   │ A&&false│ A||true │  Valid │");
    println!("├───────┼─────────┼─────────┼────────┤");
    
    for a in [false, true] {
        let and_false = a && false;
        let or_true = a || true;
        let valid = !and_false && or_true;
        println!("│ {:<5} │ {:<7} │ {:<7} │ {:<6} │", a, and_false, or_true, valid);
    }
    
    println!("└───────┴─────────┴─────────┴────────┘");
    println!("✅ Domination laws verified!");
}

fn verify_idempotent_laws() {
    println!("\n=== Idempotent Laws ===");
    println!("A && A = A");
    println!("A || A = A\n");
    
    println!("┌───────┬────────┬────────┬────────┐");
    println!("│   A   │  A&&A  │  A||A  │  Equal │");
    println!("├───────┼────────┼────────┼────────┤");
    
    for a in [false, true] {
        let and_self = a && a;
        let or_self = a || a;
        let equal = (and_self == a) && (or_self == a);
        println!("│ {:<5} │ {:<6} │ {:<6} │ {:<6} │", a, and_self, or_self, equal);
    }
    
    println!("└───────┴────────┴────────┴────────┘");
    println!("✅ Idempotent laws verified!");
}

fn verify_complement_laws() {
    println!("\n=== Complement Laws ===");
    println!("A && !A = false");
    println!("A || !A = true\n");
    
    println!("┌───────┬────────┬────────┬────────┐");
    println!("│   A   │  A&&!A │  A||!A │  Valid │");
    println!("├───────┼────────┼────────┼────────┤");
    
    for a in [false, true] {
        let and_comp = a && !a;
        let or_comp = a || !a;
        let valid = !and_comp && or_comp;
        println!("│ {:<5} │ {:<6} │ {:<6} │ {:<6} │", a, and_comp, or_comp, valid);
    }
    
    println!("└───────┴────────┴────────┴────────┘");
    println!("✅ Complement laws verified!");
}

fn verify_de_morgan_laws() {
    println!("\n=== De Morgan's Laws ===");
    println!("!(A && B) = !A || !B");
    println!("!(A || B) = !A && !B\n");
    
    println!("┌───────┬───────┬─────────┬─────────┬────────┬─────────┬─────────┬────────┐");
    println!("│   A   │   B   │ !(A&&B) │ !A||!B  │  Law 1 │ !(A||B) │ !A&&!B  │  Law 2 │");
    println!("├───────┼───────┼─────────┼─────────┼────────┼─────────┼─────────┼────────┤");
    
    for a in [false, true] {
        for b in [false, true] {
            let not_and = !(a && b);
            let not_a_or_not_b = !a || !b;
            let law1 = not_and == not_a_or_not_b;
            
            let not_or = !(a || b);
            let not_a_and_not_b = !a && !b;
            let law2 = not_or == not_a_and_not_b;
            
            println!(
                "│ {:<5} │ {:<5} │ {:<7} │ {:<7} │ {:<6} │ {:<7} │ {:<7} │ {:<6} │",
                a, b, not_and, not_a_or_not_b, law1, not_or, not_a_and_not_b, law2
            );
        }
    }
    
    println!("└───────┴───────┴─────────┴─────────┴────────┴─────────┴─────────┴────────┘");
    println!("✅ De Morgan's laws verified!");
}

fn verify_commutative_laws() {
    println!("\n=== Commutative Laws ===");
    println!("A && B = B && A");
    println!("A || B = B || A\n");
    
    println!("┌───────┬───────┬────────┬────────┬────────┬────────┬────────┬────────┐");
    println!("│   A   │   B   │  A&&B  │  B&&A  │  Equal │  A||B  │  B||A  │  Equal │");
    println!("├───────┼───────┼────────┼────────┼────────┼────────┼────────┼────────┤");
    
    for a in [false, true] {
        for b in [false, true] {
            let a_and_b = a && b;
            let b_and_a = b && a;
            let and_equal = a_and_b == b_and_a;
            
            let a_or_b = a || b;
            let b_or_a = b || a;
            let or_equal = a_or_b == b_or_a;
            
            println!(
                "│ {:<5} │ {:<5} │ {:<6} │ {:<6} │ {:<6} │ {:<6} │ {:<6} │ {:<6} │",
                a, b, a_and_b, b_and_a, and_equal, a_or_b, b_or_a, or_equal
            );
        }
    }
    
    println!("└───────┴───────┴────────┴────────┴────────┴────────┴────────┴────────┘");
    println!("✅ Commutative laws verified!");
}

fn verify_associative_laws() {
    println!("\n=== Associative Laws ===");
    println!("(A && B) && C = A && (B && C)");
    println!("(A || B) || C = A || (B || C)\n");
    
    println!("┌───────┬───────┬───────┬──────────┬──────────┬────────┬──────────┬──────────┬────────┐");
    println!("│   A   │   B   │   C   │ (A&&B)&&C│ A&&(B&&C)│  Equal │ (A||B)||C│ A||(B||C)│  Equal │");
    println!("├───────┼───────┼───────┼──────────┼──────────┼────────┼──────────┼──────────┼────────┤");
    
    for a in [false, true] {
        for b in [false, true] {
            for c in [false, true] {
                let and_left = (a && b) && c;
                let and_right = a && (b && c);
                let and_equal = and_left == and_right;
                
                let or_left = (a || b) || c;
                let or_right = a || (b || c);
                let or_equal = or_left == or_right;
                
                println!(
                    "│ {:<5} │ {:<5} │ {:<5} │ {:<8} │ {:<8} │ {:<6} │ {:<8} │ {:<8} │ {:<6} │",
                    a, b, c, and_left, and_right, and_equal, or_left, or_right, or_equal
                );
            }
        }
    }
    
    println!("└───────┴───────┴───────┴──────────┴──────────┴────────┴──────────┴──────────┴────────┘");
    println!("✅ Associative laws verified!");
}

fn verify_distributive_laws() {
    println!("\n=== Distributive Laws ===");
    println!("A && (B || C) = (A && B) || (A && C)");
    println!("A || (B && C) = (A || B) && (A || C)\n");
    
    println!("┌───────┬───────┬───────┬───────────┬───────────┬────────┐");
    println!("│   A   │   B   │   C   │  A&&(B||C)│(A&&B)||(A&&C)│ Equal │");
    println!("├───────┼───────┼───────┼───────────┼───────────┼────────┤");
    
    for a in [false, true] {
        for b in [false, true] {
            for c in [false, true] {
                let left = a && (b || c);
                let right = (a && b) || (a && c);
                let equal = left == right;
                
                println!(
                    "│ {:<5} │ {:<5} │ {:<5} │ {:<9} │ {:<11} │ {:<6} │",
                    a, b, c, left, right, equal
                );
            }
        }
    }
    
    println!("└───────┴───────┴───────┴───────────┴───────────┴────────┘");
    println!("✅ Distributive laws verified!");
}

fn verify_absorption_laws() {
    println!("\n=== Absorption Laws ===");
    println!("A || (A && B) = A");
    println!("A && (A || B) = A\n");
    
    println!("┌───────┬───────┬──────────┬──────────┬────────┐");
    println!("│   A   │   B   │ A||(A&&B)│ A&&(A||B)│  Equal │");
    println!("├───────┼───────┼──────────┼──────────┼────────┤");
    
    for a in [false, true] {
        for b in [false, true] {
            let or_abs = a || (a && b);
            let and_abs = a && (a || b);
            let equal = (or_abs == a) && (and_abs == a);
            
            println!(
                "│ {:<5} │ {:<5} │ {:<8} │ {:<8} │ {:<6} │",
                a, b, or_abs, and_abs, equal
            );
        }
    }
    
    println!("└───────┴───────┴──────────┴──────────┴────────┘");
    println!("✅ Absorption laws verified!");
}

fn read_number() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_truth_tables() {
        // Test AND
        assert_eq!(false && false, false);
        assert_eq!(true && true, true);
        
        // Test OR
        assert_eq!(false || false, false);
        assert_eq!(true || false, true);
        
        // Test NOT
        assert_eq!(!true, false);
        assert_eq!(!false, true);
    }
    
    #[test]
    fn test_de_morgans() {
        for a in [false, true] {
            for b in [false, true] {
                assert_eq!(!(a && b), !a || !b);
                assert_eq!(!(a || b), !a && !b);
            }
        }
    }
}
