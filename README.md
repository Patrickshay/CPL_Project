# CPL_Project
This Project includes QuickSort Algorithm Implementation in two different languages with two different paradigm.

Programming Languages:
C#
rust

# Description
This code demonstrates QuickSort algorithm, a sorting technique for arrays of integers or strings. This document demonstrates the QuickSort algorithm implemented in both C# and Rust, along with their major differences. It also includes the complexity and scoping

# Requirements/Commands
Requires Compilers for the files:
File: 
	Required Compilers: Visual Studio, Mono
File: 
	Required Compiler: rustc

# Explanation/Comparison

# concept 1
Paradigm: C# is an object-oriented programming language, while Rust is a systems programming language with a focus on memory management and concurrency.

Call by Reference vs. Call by Value:
	C#: In C#, arrays are passed by reference. This means that any changes made to the array inside the function are reflected in the original array. This is efficient for large data structures as it avoids unnecessary copying.
	Rust: In Rust, data is typically passed by value. However, slices, which are used in the QuickSort implementation, are passed by reference. This allows functions to modify the original data without requiring an explicit return statement.

Parameter Passing:
	C#: C# uses positional parameter passing, where each parameter is matched to its corresponding position in the function signature.
	Rust: Rust uses named parameter passing, where each parameter is explicitly named and matched to its corresponding name in the function signature. This can improve code clarity and avoid errors, especially when dealing with many parameters.

Dynamic Scoping vs. Lexical Scoping:
	C#: C# uses dynamic scoping, where variables are resolved based on the current call stack at runtime.
	Rust: Rust uses lexical scoping, where variables are resolved based on their static location in the code. This makes code behavior more predictable and easier to reason about.

# concept 2
Garbage Collection vs. Manual Memory Management:
	C#: C# uses automatic garbage collection to manage memory allocation and deallocation. This frees developers from the burden of manually managing memory, but can sometimes lead to performance overhead.
	Rust: Rust uses a ownership system and borrowing rules for memory management. This allows for more efficient and predictable memory usage, but requires developers to be more mindful of memory allocation and deallocation.

Error Handling:
	C# uses exceptions for error handling, while Rust uses a more explicit result-based approach with the Result type.

Concurrency: 
	Rust provides built-in features for concurrency through ownership and borrowing rules, while C# requires additional libraries and frameworks.


# concept 3
Data types: C# uses arrays for data structures, while Rust uses slices which are flexible and growable.

Memory management: C# uses garbage collection, while Rust requires manual memory management through ownership and borrowing rules.

Swapping: C# uses temporary variable for swapping, while Rust leverages the swap method on slices.

Recursion: Both languages use recursion for the sorting process.

# Concept 4
	Best complexity: O(n log n) - achieved when the pivot element effectively divides the array into sub-arrays of equal size.
	Worst complexity: O(n^2) - occurs when the pivot element is the smallest or largest element in the array, leading to unbalanced sub-arrays.
