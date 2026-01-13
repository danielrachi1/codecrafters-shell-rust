#!/bin/bash

# Test script for pipeline functionality

echo "=== Test 1: cat | wc ==="
echo "cat /tmp/pig/file-11 | wc" | ./your_program.sh

echo ""
echo "=== Test 2: tail -f | head (with timeout) ==="
timeout 1 bash -c 'echo "tail -f /tmp/fox/file-64 | head -n 3" | ./your_program.sh'

echo ""
echo "=== Test 3: Multi-stage pipeline ==="
echo "echo hello world | cat | cat | wc" | ./your_program.sh

echo ""
echo "=== Test 4: grep pipeline ==="
echo "cat /tmp/pig/file-11 | grep apple" | ./your_program.sh

echo ""
echo "=== Test 5: Single command (no pipeline) ==="
echo "cat /tmp/pig/file-11" | ./your_program.sh

echo ""
echo "=== Expected outputs ==="
echo "Test 1: Should show '5 10 78'"
echo "Test 2: Should show first 3 lines of file"
echo "Test 3: Should show '1 2 12'"
echo "Test 4: Should show lines containing 'apple'"
echo "Test 5: Should show file contents"
