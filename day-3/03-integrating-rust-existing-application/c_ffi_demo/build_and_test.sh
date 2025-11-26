#!/bin/bash
# Build script for Linux/macOS

echo "Building Rust library..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "Build successful!"
    echo ""
    echo "Compiling C test program..."
    
    # Determine OS
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        LIB_EXT="so"
        LIB_PATH_VAR="LD_LIBRARY_PATH"
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        LIB_EXT="dylib"
        LIB_PATH_VAR="DYLD_LIBRARY_PATH"
    else
        echo "Unsupported OS: $OSTYPE"
        exit 1
    fi
    
    # Compile C program
    gcc examples/test.c -L target/release -lmathlib -lm -o examples/test
    
    if [ $? -eq 0 ]; then
        echo "Compilation successful!"
        echo ""
        echo "Running test program..."
        echo "========================================"
        
        # Set library path and run
        export $LIB_PATH_VAR="target/release:$$LIB_PATH_VAR"
        ./examples/test
        
        echo "========================================"
    else
        echo "Compilation failed!"
        exit 1
    fi
else
    echo "Build failed!"
    exit 1
fi
