# Accessing Freed Memory in Rust Vectors

This repository demonstrates a common error in Rust: accessing elements of a vector after it has been deallocated.  This can lead to a panic at runtime. The `bug.rs` file contains the buggy code, and `bugSolution.rs` provides a corrected version.

## The Bug
The original code attempts to access elements of a vector after potentially deallocating memory.  This leads to undefined behavior and a runtime panic.

## The Solution
The solution ensures proper memory management by avoiding operations that could lead to deallocation of the vector while still accessing its elements.  This is usually achieved by checking if the vector is empty before accessing elements and performing operations accordingly.