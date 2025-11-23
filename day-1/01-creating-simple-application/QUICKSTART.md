# Quick Start Guide - Day 1 Projects

This guide will help you run all the example projects from Day 1.

## Prerequisites

Make sure you have Rust installed. Verify with:
```powershell
rustc --version
cargo --version
```

## Running the Projects

### Method 1: Using Cargo (Recommended)

Navigate to each project directory and run:

```powershell
# Example: Temperature Converter
cd temperature_converter
cargo run

# Example: Calculator
cd ..\calculator
cargo run

# Example: Guessing Game
cd ..\guessing_game
cargo run

# Example: Grade Calculator
cd ..\grade_calculator
cargo run

# Example: BMI Calculator
cd ..\bmi_calculator
cargo run

# Example: Todo List
cd ..\todo_list
cargo run
```

### Method 2: Running Exercise Files Directly

For standalone exercise files in the `exercises` folder:

```powershell
cd exercises

# Compile and run Age Calculator
rustc age_calculator.rs
.\age_calculator.exe

# Compile and run Area Calculator
rustc area_calculator.rs
.\area_calculator.exe

# Compile and run Palindrome Checker
rustc palindrome_checker.rs
.\palindrome_checker.exe

# Compile and run Fibonacci Generator
rustc fibonacci.rs
.\fibonacci.exe

# Compile and run Unit Converter
rustc unit_converter.rs
.\unit_converter.exe
```

## Running Tests

Some projects include unit tests. To run them:

```powershell
cd temperature_converter
cargo test

cd ..\grade_calculator
cargo test

cd ..\bmi_calculator
cargo test
```

## Building Release Versions

For optimized executables:

```powershell
cd temperature_converter
cargo build --release

# The executable will be in target/release/
.\target\release\temperature_converter.exe
```

## Project List

### Complete Cargo Projects
1. **hello_rust** - Basic Hello World
2. **temperature_converter** - Celsius ↔ Fahrenheit converter
3. **calculator** - Basic arithmetic calculator
4. **guessing_game** - Number guessing game (requires `rand` crate)
5. **grade_calculator** - Score to letter grade converter
6. **bmi_calculator** - Body Mass Index calculator
7. **todo_list** - Simple task manager

### Exercise Files (exercises folder)
1. **age_calculator.rs** - Calculate age from birth year
2. **area_calculator.rs** - Calculate area of shapes
3. **palindrome_checker.rs** - Check if word is palindrome
4. **fibonacci.rs** - Generate Fibonacci sequence
5. **unit_converter.rs** - Multi-unit converter

## Cleaning Build Artifacts

To clean up compiled files and save disk space:

```powershell
# For Cargo projects
cargo clean

# For manually compiled files
Remove-Item *.exe
Remove-Item *.pdb
```

## Troubleshooting

### Issue: "cargo not found"
**Solution:** Restart your terminal or run:
```powershell
$env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
```

### Issue: Guessing game won't compile
**Solution:** The guessing game requires the `rand` crate. Cargo will download it automatically when you run `cargo run`.

### Issue: Permission denied
**Solution:** On Windows, you may need to allow the executable:
```powershell
Unblock-File .\target\debug\project_name.exe
```

## Next Steps

After running these examples:

1. ✅ Modify the code to add new features
2. ✅ Complete the exercises in the README
3. ✅ Try creating your own variations
4. ✅ Read the inline comments to understand the code
5. ✅ Experiment with different inputs

## Additional Resources

- **README.md** - Complete lecture notes
- **Inline comments** - Detailed explanations in each file
- **Cargo.toml** - Project dependencies and metadata

Happy coding! 🦀
