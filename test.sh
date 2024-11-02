#!/bin/bash

# Ensure the Rust binary exists
if [[ ! -f "./computor" ]]; then
    echo "Error: my_rust_program not found. Please compile your Rust code first."
    exit 1
fi

# Run the Rust program with some example commands/arguments
echo "Running the computor_v1 program with different tests..."

# Example test 1
echo "Running test 1..."
./computor "X^2 - 5*X + 6 = 0"
echo "--------------------------"

# Example test 2
echo "Running test 2..."
./computor "X^2 - 4*X + 4 = 0"
echo "--------------------------"

# Example test 3
echo "Running test 3..."
./computor "2*X + 3 = 2*X - 5"
echo "--------------------------"

# Example test 4
echo "Running test 4..."
./computor "5 * X^0 = 5 * X^0"
echo "--------------------------"

# Example test 5
echo "Running test 5..."
./computor "2*X^3 + X^2 + 2*X + 5 = 0"
echo "--------------------------"

# Example test 6
echo "Running test 6..."
./computor "6 * X^0 + 11 * X^1 + 5 * X^2 = 1 * X^0 + 0 * X^1"
echo "--------------------------"

# Example test 7
echo "Running test 7..."
./computor "5 + 3 * X + 3 * X^2 = 1 + 0 * X^2"
echo "--------------------------"

# Example test 8
echo "Running test 8..."
./computor "5 + 3.111 * X + 3 * X^2 = 1 + 0.1 * X^2"
echo "--------------------------"

# Indicate completion
echo "All tests executed."
