use std::io::{self, Write};
use std::collections::{HashMap, VecDeque};

/// Demonstrates advanced loop techniques and algorithms
fn main() {
    loop {
        display_menu();
        
        let choice = get_user_choice();
        
        match choice {
            1 => fibonacci_variations(),
            2 => prime_number_algorithms(),
            3 => matrix_algorithms(),
            4 => graph_traversal(),
            5 => dynamic_programming(),
            6 => recursive_to_iterative(),
            7 => algorithm_challenges(),
            8 => performance_optimization(),
            0 => {
                println!("\nThank you for exploring advanced loops!");
                break;
            }
            _ => println!("\n❌ Invalid choice. Please try again."),
        }
        
        pause();
    }
}

fn display_menu() {
    println!("\n╔════════════════════════════════════════╗");
    println!("║     Advanced Loops in Rust            ║");
    println!("╚════════════════════════════════════════╝");
    println!("1. Fibonacci Variations");
    println!("2. Prime Number Algorithms");
    println!("3. Matrix Algorithms");
    println!("4. Graph Traversal");
    println!("5. Dynamic Programming");
    println!("6. Recursive to Iterative");
    println!("7. Algorithm Challenges");
    println!("8. Performance Optimization");
    println!("0. Exit");
    print!("\nEnter your choice: ");
    io::stdout().flush().unwrap();
}

fn get_user_choice() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap_or(-1)
}

fn pause() {
    print!("\nPress Enter to continue...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

// 1. Fibonacci Variations
fn fibonacci_variations() {
    println!("\n=== Fibonacci Variations ===\n");
    
    println!("Example 1: Standard Fibonacci");
    let n = 10;
    let mut fib = vec![0u64, 1];
    
    for i in 2..n {
        let next = fib[i - 1] + fib[i - 2];
        fib.push(next);
    }
    
    println!("First {} Fibonacci numbers:", n);
    println!("{:?}", fib);
    
    println!("\nExample 2: Fibonacci with iterator");
    struct Fibonacci {
        curr: u64,
        next: u64,
    }
    
    impl Iterator for Fibonacci {
        type Item = u64;
        
        fn next(&mut self) -> Option<u64> {
            let new_next = self.curr.checked_add(self.next)?;
            self.curr = self.next;
            self.next = new_next;
            Some(self.curr)
        }
    }
    
    fn fibonacci() -> Fibonacci {
        Fibonacci { curr: 0, next: 1 }
    }
    
    let fib_iter: Vec<u64> = fibonacci().take(15).collect();
    println!("Fibonacci with iterator (15 numbers):");
    println!("{:?}", fib_iter);
    
    println!("\nExample 3: Fibonacci with memoization");
    fn fib_memo(n: usize, memo: &mut HashMap<usize, u64>) -> u64 {
        if n <= 1 {
            return n as u64;
        }
        
        if let Some(&value) = memo.get(&n) {
            return value;
        }
        
        let result = fib_memo(n - 1, memo) + fib_memo(n - 2, memo);
        memo.insert(n, result);
        result
    }
    
    let mut memo = HashMap::new();
    println!("Fibonacci with memoization:");
    for i in 0..15 {
        print!("{} ", fib_memo(i, &mut memo));
    }
    println!();
    
    println!("\nExample 4: Find nth Fibonacci efficiently");
    fn nth_fibonacci(n: usize) -> u64 {
        if n <= 1 {
            return n as u64;
        }
        
        let mut a = 0u64;
        let mut b = 1u64;
        
        for _ in 2..=n {
            let temp = a + b;
            a = b;
            b = temp;
        }
        
        b
    }
    
    for i in [10, 20, 30] {
        println!("F({}) = {}", i, nth_fibonacci(i));
    }
}

// 2. Prime Number Algorithms
fn prime_number_algorithms() {
    println!("\n=== Prime Number Algorithms ===\n");
    
    println!("Example 1: Check if number is prime");
    fn is_prime(n: u32) -> bool {
        if n < 2 {
            return false;
        }
        if n == 2 {
            return true;
        }
        if n % 2 == 0 {
            return false;
        }
        
        let limit = (n as f64).sqrt() as u32;
        for i in (3..=limit).step_by(2) {
            if n % i == 0 {
                return false;
            }
        }
        
        true
    }
    
    let test_numbers = vec![2, 3, 4, 17, 25, 29, 97, 100];
    println!("Prime check:");
    for &n in &test_numbers {
        println!("  {} -> {}", n, if is_prime(n) { "prime" } else { "not prime" });
    }
    
    println!("\nExample 2: Sieve of Eratosthenes");
    fn sieve_of_eratosthenes(limit: usize) -> Vec<usize> {
        let mut is_prime = vec![true; limit + 1];
        is_prime[0] = false;
        is_prime[1] = false;
        
        for i in 2..=((limit as f64).sqrt() as usize) {
            if is_prime[i] {
                for j in ((i * i)..=limit).step_by(i) {
                    is_prime[j] = false;
                }
            }
        }
        
        is_prime.iter()
            .enumerate()
            .filter(|(_, &prime)| prime)
            .map(|(i, _)| i)
            .collect()
    }
    
    let primes = sieve_of_eratosthenes(50);
    println!("Primes up to 50 (Sieve of Eratosthenes):");
    println!("{:?}", primes);
    
    println!("\nExample 3: Prime factorization");
    fn prime_factorization(mut n: u64) -> Vec<u64> {
        let mut factors = Vec::new();
        
        // Check for 2s
        while n % 2 == 0 {
            factors.push(2);
            n /= 2;
        }
        
        // Check odd factors
        let mut i = 3;
        while i * i <= n {
            while n % i == 0 {
                factors.push(i);
                n /= i;
            }
            i += 2;
        }
        
        // If n is still greater than 1, it's prime
        if n > 1 {
            factors.push(n);
        }
        
        factors
    }
    
    let numbers = vec![12, 56, 100, 315];
    println!("Prime factorizations:");
    for &n in &numbers {
        let factors = prime_factorization(n);
        println!("  {} = {:?}", n, factors);
    }
    
    println!("\nExample 4: Twin primes");
    let limit = 100;
    let primes = sieve_of_eratosthenes(limit);
    
    println!("Twin primes up to {}:", limit);
    for window in primes.windows(2) {
        if window[1] - window[0] == 2 {
            println!("  ({}, {})", window[0], window[1]);
        }
    }
}

// 3. Matrix Algorithms
fn matrix_algorithms() {
    println!("\n=== Matrix Algorithms ===\n");
    
    println!("Example 1: Matrix transpose");
    fn transpose(matrix: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if matrix.is_empty() {
            return vec![];
        }
        
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut result = vec![vec![0; rows]; cols];
        
        for i in 0..rows {
            for j in 0..cols {
                result[j][i] = matrix[i][j];
            }
        }
        
        result
    }
    
    let matrix = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
    ];
    
    println!("Original matrix:");
    for row in &matrix {
        println!("  {:?}", row);
    }
    
    let transposed = transpose(&matrix);
    println!("\nTransposed:");
    for row in &transposed {
        println!("  {:?}", row);
    }
    
    println!("\nExample 2: Matrix multiplication");
    fn matrix_multiply(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Option<Vec<Vec<i32>>> {
        if a.is_empty() || b.is_empty() || a[0].len() != b.len() {
            return None;
        }
        
        let rows = a.len();
        let cols = b[0].len();
        let inner = a[0].len();
        
        let mut result = vec![vec![0; cols]; rows];
        
        for i in 0..rows {
            for j in 0..cols {
                for k in 0..inner {
                    result[i][j] += a[i][k] * b[k][j];
                }
            }
        }
        
        Some(result)
    }
    
    let a = vec![
        vec![1, 2],
        vec![3, 4],
    ];
    
    let b = vec![
        vec![5, 6],
        vec![7, 8],
    ];
    
    println!("Matrix A:");
    for row in &a {
        println!("  {:?}", row);
    }
    
    println!("\nMatrix B:");
    for row in &b {
        println!("  {:?}", row);
    }
    
    if let Some(result) = matrix_multiply(&a, &b) {
        println!("\nA × B:");
        for row in &result {
            println!("  {:?}", row);
        }
    }
    
    println!("\nExample 3: Spiral matrix traversal");
    fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::new();
        
        if matrix.is_empty() {
            return result;
        }
        
        let mut top = 0;
        let mut bottom = matrix.len() - 1;
        let mut left = 0;
        let mut right = matrix[0].len() - 1;
        
        loop {
            // Go right
            for i in left..=right {
                result.push(matrix[top][i]);
            }
            top += 1;
            if top > bottom {
                break;
            }
            
            // Go down
            for i in top..=bottom {
                result.push(matrix[i][right]);
            }
            if right == 0 {
                break;
            }
            right -= 1;
            if left > right {
                break;
            }
            
            // Go left
            for i in (left..=right).rev() {
                result.push(matrix[bottom][i]);
            }
            if bottom == 0 {
                break;
            }
            bottom -= 1;
            if top > bottom {
                break;
            }
            
            // Go up
            for i in (top..=bottom).rev() {
                result.push(matrix[i][left]);
            }
            left += 1;
            if left > right {
                break;
            }
        }
        
        result
    }
    
    let matrix = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
    ];
    
    println!("Matrix:");
    for row in &matrix {
        println!("  {:?}", row);
    }
    
    let spiral = spiral_order(matrix);
    println!("\nSpiral order: {:?}", spiral);
    
    println!("\nExample 4: Generate spiral matrix");
    fn generate_spiral(n: usize) -> Vec<Vec<i32>> {
        let mut matrix = vec![vec![0; n]; n];
        let mut num = 1;
        let mut top = 0;
        let mut bottom = n - 1;
        let mut left = 0;
        let mut right = n - 1;
        
        while num <= (n * n) as i32 {
            for i in left..=right {
                matrix[top][i] = num;
                num += 1;
            }
            top += 1;
            
            for i in top..=bottom {
                matrix[i][right] = num;
                num += 1;
            }
            if right == 0 {
                break;
            }
            right -= 1;
            
            if top <= bottom {
                for i in (left..=right).rev() {
                    matrix[bottom][i] = num;
                    num += 1;
                }
                if bottom == 0 {
                    break;
                }
                bottom -= 1;
            }
            
            if left <= right {
                for i in (top..=bottom).rev() {
                    matrix[i][left] = num;
                    num += 1;
                }
                left += 1;
            }
        }
        
        matrix
    }
    
    let spiral_matrix = generate_spiral(4);
    println!("Generated spiral matrix (4×4):");
    for row in &spiral_matrix {
        for &val in row {
            print!("{:3} ", val);
        }
        println!();
    }
}

// 4. Graph Traversal
fn graph_traversal() {
    println!("\n=== Graph Traversal ===\n");
    
    type Graph = HashMap<i32, Vec<i32>>;
    
    println!("Example 1: Breadth-First Search (BFS)");
    fn bfs(graph: &Graph, start: i32) -> Vec<i32> {
        let mut visited = Vec::new();
        let mut queue = VecDeque::new();
        let mut seen = std::collections::HashSet::new();
        
        queue.push_back(start);
        seen.insert(start);
        
        while let Some(node) = queue.pop_front() {
            visited.push(node);
            
            if let Some(neighbors) = graph.get(&node) {
                for &neighbor in neighbors {
                    if !seen.contains(&neighbor) {
                        seen.insert(neighbor);
                        queue.push_back(neighbor);
                    }
                }
            }
        }
        
        visited
    }
    
    let mut graph: Graph = HashMap::new();
    graph.insert(1, vec![2, 3]);
    graph.insert(2, vec![1, 4, 5]);
    graph.insert(3, vec![1, 6]);
    graph.insert(4, vec![2]);
    graph.insert(5, vec![2]);
    graph.insert(6, vec![3]);
    
    println!("Graph structure:");
    for (node, neighbors) in &graph {
        println!("  {} -> {:?}", node, neighbors);
    }
    
    let bfs_result = bfs(&graph, 1);
    println!("\nBFS from node 1: {:?}", bfs_result);
    
    println!("\nExample 2: Depth-First Search (DFS) - Iterative");
    fn dfs_iterative(graph: &Graph, start: i32) -> Vec<i32> {
        let mut visited = Vec::new();
        let mut stack = vec![start];
        let mut seen = std::collections::HashSet::new();
        
        while let Some(node) = stack.pop() {
            if !seen.contains(&node) {
                seen.insert(node);
                visited.push(node);
                
                if let Some(neighbors) = graph.get(&node) {
                    for &neighbor in neighbors.iter().rev() {
                        if !seen.contains(&neighbor) {
                            stack.push(neighbor);
                        }
                    }
                }
            }
        }
        
        visited
    }
    
    let dfs_result = dfs_iterative(&graph, 1);
    println!("DFS from node 1: {:?}", dfs_result);
    
    println!("\nExample 3: Find path between nodes");
    fn find_path(graph: &Graph, start: i32, goal: i32) -> Option<Vec<i32>> {
        let mut queue = VecDeque::new();
        let mut parent: HashMap<i32, i32> = HashMap::new();
        
        queue.push_back(start);
        parent.insert(start, start);
        
        while let Some(node) = queue.pop_front() {
            if node == goal {
                // Reconstruct path
                let mut path = Vec::new();
                let mut current = goal;
                
                while current != start {
                    path.push(current);
                    current = parent[&current];
                }
                path.push(start);
                path.reverse();
                
                return Some(path);
            }
            
            if let Some(neighbors) = graph.get(&node) {
                for &neighbor in neighbors {
                    if !parent.contains_key(&neighbor) {
                        parent.insert(neighbor, node);
                        queue.push_back(neighbor);
                    }
                }
            }
        }
        
        None
    }
    
    match find_path(&graph, 1, 6) {
        Some(path) => println!("Path from 1 to 6: {:?}", path),
        None => println!("No path found"),
    }
    
    println!("\nExample 4: Detect cycle in graph");
    fn has_cycle(graph: &Graph) -> bool {
        let mut visited = std::collections::HashSet::new();
        let mut rec_stack = std::collections::HashSet::new();
        
        fn visit(
            node: i32,
            graph: &Graph,
            visited: &mut std::collections::HashSet<i32>,
            rec_stack: &mut std::collections::HashSet<i32>,
        ) -> bool {
            visited.insert(node);
            rec_stack.insert(node);
            
            if let Some(neighbors) = graph.get(&node) {
                for &neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        if visit(neighbor, graph, visited, rec_stack) {
                            return true;
                        }
                    } else if rec_stack.contains(&neighbor) {
                        return true;
                    }
                }
            }
            
            rec_stack.remove(&node);
            false
        }
        
        for &node in graph.keys() {
            if !visited.contains(&node) {
                if visit(node, graph, &mut visited, &mut rec_stack) {
                    return true;
                }
            }
        }
        
        false
    }
    
    println!("Has cycle: {}", has_cycle(&graph));
}

// 5. Dynamic Programming
fn dynamic_programming() {
    println!("\n=== Dynamic Programming ===\n");
    
    println!("Example 1: Longest Common Subsequence");
    fn lcs(s1: &str, s2: &str) -> String {
        let chars1: Vec<char> = s1.chars().collect();
        let chars2: Vec<char> = s2.chars().collect();
        let m = chars1.len();
        let n = chars2.len();
        
        let mut dp = vec![vec![0; n + 1]; m + 1];
        
        for i in 1..=m {
            for j in 1..=n {
                if chars1[i - 1] == chars2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
                }
            }
        }
        
        // Reconstruct LCS
        let mut result = String::new();
        let mut i = m;
        let mut j = n;
        
        while i > 0 && j > 0 {
            if chars1[i - 1] == chars2[j - 1] {
                result.push(chars1[i - 1]);
                i -= 1;
                j -= 1;
            } else if dp[i - 1][j] > dp[i][j - 1] {
                i -= 1;
            } else {
                j -= 1;
            }
        }
        
        result.chars().rev().collect()
    }
    
    let s1 = "ABCDGH";
    let s2 = "AEDFHR";
    println!("String 1: {}", s1);
    println!("String 2: {}", s2);
    println!("LCS: {}", lcs(s1, s2));
    
    println!("\nExample 2: Coin change problem");
    fn coin_change(coins: &[i32], amount: i32) -> i32 {
        let mut dp = vec![amount + 1; (amount + 1) as usize];
        dp[0] = 0;
        
        for i in 1..=amount {
            for &coin in coins {
                if coin <= i {
                    dp[i as usize] = dp[i as usize].min(dp[(i - coin) as usize] + 1);
                }
            }
        }
        
        if dp[amount as usize] > amount {
            -1
        } else {
            dp[amount as usize]
        }
    }
    
    let coins = vec![1, 2, 5];
    let amount = 11;
    println!("Coins: {:?}", coins);
    println!("Amount: {}", amount);
    println!("Minimum coins: {}", coin_change(&coins, amount));
    
    println!("\nExample 3: Maximum subarray sum (Kadane's algorithm)");
    fn max_subarray(nums: &[i32]) -> i32 {
        let mut max_sum = nums[0];
        let mut current_sum = nums[0];
        
        for &num in &nums[1..] {
            current_sum = num.max(current_sum + num);
            max_sum = max_sum.max(current_sum);
        }
        
        max_sum
    }
    
    let numbers = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    println!("Array: {:?}", numbers);
    println!("Maximum subarray sum: {}", max_subarray(&numbers));
    
    println!("\nExample 4: Climbing stairs");
    fn climb_stairs(n: i32) -> i32 {
        if n <= 2 {
            return n;
        }
        
        let mut prev2 = 1;
        let mut prev1 = 2;
        
        for _ in 3..=n {
            let current = prev1 + prev2;
            prev2 = prev1;
            prev1 = current;
        }
        
        prev1
    }
    
    for n in 1..=10 {
        println!("Stairs {}: {} ways", n, climb_stairs(n));
    }
}

// 6. Recursive to Iterative
fn recursive_to_iterative() {
    println!("\n=== Recursive to Iterative Conversion ===\n");
    
    println!("Example 1: Factorial");
    
    // Recursive version
    fn factorial_recursive(n: u64) -> u64 {
        if n <= 1 {
            1
        } else {
            n * factorial_recursive(n - 1)
        }
    }
    
    // Iterative version
    fn factorial_iterative(n: u64) -> u64 {
        let mut result = 1;
        for i in 2..=n {
            result *= i;
        }
        result
    }
    
    println!("Factorial of 10:");
    println!("  Recursive:  {}", factorial_recursive(10));
    println!("  Iterative:  {}", factorial_iterative(10));
    
    println!("\nExample 2: Sum of array");
    
    // Recursive version
    fn sum_recursive(arr: &[i32]) -> i32 {
        if arr.is_empty() {
            0
        } else {
            arr[0] + sum_recursive(&arr[1..])
        }
    }
    
    // Iterative version
    fn sum_iterative(arr: &[i32]) -> i32 {
        let mut sum = 0;
        for &num in arr {
            sum += num;
        }
        sum
    }
    
    let numbers = vec![1, 2, 3, 4, 5];
    println!("Sum of {:?}:", numbers);
    println!("  Recursive: {}", sum_recursive(&numbers));
    println!("  Iterative: {}", sum_iterative(&numbers));
    
    println!("\nExample 3: Binary search");
    
    // Iterative version (more efficient)
    fn binary_search_iterative(arr: &[i32], target: i32) -> Option<usize> {
        let mut left = 0;
        let mut right = arr.len();
        
        while left < right {
            let mid = left + (right - left) / 2;
            
            if arr[mid] == target {
                return Some(mid);
            } else if arr[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        
        None
    }
    
    let sorted = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
    println!("Binary search in {:?}", sorted);
    println!("  Find 7:  {:?}", binary_search_iterative(&sorted, 7));
    println!("  Find 13: {:?}", binary_search_iterative(&sorted, 13));
    println!("  Find 8:  {:?}", binary_search_iterative(&sorted, 8));
}

// 7. Algorithm Challenges
fn algorithm_challenges() {
    println!("\n=== Algorithm Challenges ===\n");
    
    println!("Challenge 1: FizzBuzz");
    println!("Print numbers 1-30, but:");
    println!("  - 'Fizz' for multiples of 3");
    println!("  - 'Buzz' for multiples of 5");
    println!("  - 'FizzBuzz' for multiples of both");
    
    for i in 1..=30 {
        let result = match (i % 3 == 0, i % 5 == 0) {
            (true, true) => "FizzBuzz".to_string(),
            (true, false) => "Fizz".to_string(),
            (false, true) => "Buzz".to_string(),
            (false, false) => i.to_string(),
        };
        print!("{} ", result);
        if i % 10 == 0 {
            println!();
        }
    }
    
    println!("\nChallenge 2: Reverse words in string");
    fn reverse_words(s: &str) -> String {
        s.split_whitespace()
            .rev()
            .collect::<Vec<_>>()
            .join(" ")
    }
    
    let sentence = "Hello World Rust Programming";
    println!("Original: '{}'", sentence);
    println!("Reversed: '{}'", reverse_words(sentence));
    
    println!("\nChallenge 3: Find missing number");
    fn find_missing(nums: &[i32]) -> i32 {
        let n = nums.len() as i32 + 1;
        let expected_sum = n * (n + 1) / 2;
        let actual_sum: i32 = nums.iter().sum();
        expected_sum - actual_sum
    }
    
    let numbers = vec![1, 2, 3, 5, 6, 7, 8, 9, 10];
    println!("Numbers: {:?}", numbers);
    println!("Missing: {}", find_missing(&numbers));
    
    println!("\nChallenge 4: Anagram check");
    fn are_anagrams(s1: &str, s2: &str) -> bool {
        if s1.len() != s2.len() {
            return false;
        }
        
        let mut chars1: Vec<char> = s1.to_lowercase().chars().collect();
        let mut chars2: Vec<char> = s2.to_lowercase().chars().collect();
        
        chars1.sort();
        chars2.sort();
        
        chars1 == chars2
    }
    
    let pairs = vec![
        ("listen", "silent"),
        ("hello", "world"),
        ("triangle", "integral"),
    ];
    
    for (s1, s2) in pairs {
        println!("'{}' and '{}': {}", s1, s2,
                 if are_anagrams(s1, s2) { "anagrams" } else { "not anagrams" });
    }
}

// 8. Performance Optimization
fn performance_optimization() {
    println!("\n=== Performance Optimization ===\n");
    
    println!("Example 1: Avoid repeated calculations");
    
    // Inefficient
    fn sum_of_squares_inefficient(n: i32) -> i32 {
        let mut sum = 0;
        for i in 1..=n {
            sum += i * i;
        }
        sum
    }
    
    // Efficient (using formula)
    fn sum_of_squares_efficient(n: i32) -> i32 {
        n * (n + 1) * (2 * n + 1) / 6
    }
    
    let n = 1000;
    println!("Sum of squares 1-{}:", n);
    println!("  Loop:    {}", sum_of_squares_inefficient(n));
    println!("  Formula: {}", sum_of_squares_efficient(n));
    
    println!("\nExample 2: Early termination");
    fn contains_duplicate_slow(nums: &[i32]) -> bool {
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                if nums[i] == nums[j] {
                    return true;
                }
            }
        }
        false
    }
    
    fn contains_duplicate_fast(nums: &[i32]) -> bool {
        use std::collections::HashSet;
        let mut seen = HashSet::new();
        
        for &num in nums {
            if !seen.insert(num) {
                return true;
            }
        }
        false
    }
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 2];
    println!("Has duplicates: {}", contains_duplicate_fast(&numbers));
    
    println!("\nExample 3: Use appropriate data structures");
    println!("Finding elements in large collection:");
    
    let data: Vec<i32> = (1..=1000).collect();
    
    // Vector search - O(n)
    let target = 750;
    let found = data.iter().any(|&x| x == target);
    println!("  Vec search: {}", found);
    
    // HashSet search - O(1)
    use std::collections::HashSet;
    let set: HashSet<i32> = data.iter().copied().collect();
    let found = set.contains(&target);
    println!("  Set search: {}", found);
    
    println!("\nExample 4: Iterator chaining vs multiple passes");
    let numbers: Vec<i32> = (1..=100).collect();
    
    // Multiple passes
    let evens: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).copied().collect();
    let doubled: Vec<i32> = evens.iter().map(|x| x * 2).collect();
    let large: Vec<i32> = doubled.iter().filter(|&&x| x > 50).copied().collect();
    
    println!("Multiple passes result: {} elements", large.len());
    
    // Single chain
    let result: Vec<i32> = numbers.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|x| x * 2)
        .filter(|&x| x > 50)
        .collect();
    
    println!("Single chain result: {} elements", result.len());
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fibonacci() {
        fn fib(n: usize) -> u64 {
            if n <= 1 {
                return n as u64;
            }
            let mut a = 0;
            let mut b = 1;
            for _ in 2..=n {
                let temp = a + b;
                a = b;
                b = temp;
            }
            b
        }
        
        assert_eq!(fib(10), 55);
        assert_eq!(fib(20), 6765);
    }
    
    #[test]
    fn test_prime_check() {
        fn is_prime(n: u32) -> bool {
            if n < 2 {
                return false;
            }
            for i in 2..=(n as f64).sqrt() as u32 {
                if n % i == 0 {
                    return false;
                }
            }
            true
        }
        
        assert!(is_prime(17));
        assert!(!is_prime(4));
    }
}
