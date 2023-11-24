# rust_v_cpp_popl

### Problem Statement:
The primary objective is to implement essential functionalities, such as creating, editing, and deleting notes, in both C++ and Rust and assess the execution time of these operations.. This not only addresses common notemaking requirements but also explores Rust's suitability for real-world applications.

### Software Architecture:
*Architecture Overview:*
- The software architecture follows a modular and organized approach:

- Modules: The program is divided into modules, separating concerns related to notes and the editor.
- File Handling: Notes are stored as text files, and the program utilizes standard file operations.
- Undo Buffer: An undo buffer is implemented to enable reverting to previous note content.

*Testing Component:*
-The testing component is localized, comprising unit tests for each functionality. The application does not incorporate a database; instead, notes are persisted as text files.

### POPL Aspects:

1. *Option Type for Selected Index(Rust):*
    -  The use of Option<usize> for selected_note_index reflects Rust's commitment to explicit handling of potential absence, avoiding null-pointer-                      related issues.(editor.rs, line 6)
    -  Rust's Option type explicitly conveys the possibility of a value being absent, promoting safer handling and reducing runtime errors associated with                null references.

2. *Functional Programming in Rust:*
    -  Rust incorporates functional programming aspects, such as immutable variables and higher-order functions, contributing to expressive and concise code.

3. *Result Type for File Operations (Rust):*
    -  The use of the Result type in file operations explicitly handles potential errors during file I/O operations (editor.rs, line 64).
    -   Rust's Result type emphasizes explicit error handling, making it clear when operations can fail and encouraging developers to handle errors gracefully.

4. *Ownership and Borrowing (Rust):*
    -  The ownership model in Rust dictates how memory is managed, ensuring memory safety without relying on a garbage collector or sacrificing 
       performance (editor.rs, line 11).
    -  Rust's ownership system prevents data races and null pointer dereferences by enforcing strict rules on how references are used and ensuring there
       is only one owner of a piece of data at a time.  

### RESULTS
![result 1](https://github.com/Gautham793/rust_v_cpp_popl/assets/142087982/d8644f85-7e76-46b1-b0f6-765260ee2ebb)




*Data Preparation:*
   -  Provided two sets of data, y_values_cpp representing execution times for a C++ implementation, and y_values_rust representing execution times for a Rust           implementation.
   -  Both sets are associated with different iterations (x_values).

*Graphical Representation:*
   - We used Matplotlib to create a log-log plot, where both x and y axes are on a logarithmic scale, allowing us to visualize a wide range of data points more         clearly.
   - The blue markers and curve represent C++ execution times, while the orange markers and curve represent Rust execution times.

*Statistical Analysis:*
-  We conducted a paired t-test using the ttest_rel function from the scipy.stats module. The paired t-test is appropriate here because each data point at a          specific iteration comes from the same experiment, with one data point from the C++ implementation and one from the Rust implementation.
-  The null hypothesis of the test is that there is no significant difference between the C++ and Rust execution times.     

*Annotation on the Plot:*
- The t-statistic and p-value obtained from the t-test are annotated on the plot. The t-statistic measures the difference between the sample means,
       and the p-value indicates the probability of observing such an extreme result if the null hypothesis is true.

*Detailed Analysis (Printed Output):*
- The printed output below the plot provides a technical interpretation of the statistical analysis.
- It checks if the p-value is below a significance level (0.05 is commonly used). If so, it indicates that there is a statistically significant difference           between the C++ and Rust execution times.
- Depending on the sign of the t-statistic, it provides information about which language performs significantly better.

### Potential for future work:
1. Additional Functionality and Refinements:
      - Tagging System:  Implement a tagging system to allow users to categorize and organize notes more effectively. This can involve adding, removing, and               searching notes based on tags.
2. Large Dataset Handling:
     - Optimize the program's performance when handling a large number of notes and extensive text content. This may involve refining data                                structures and algorithms to ensure scalability.
3. Advanced Ownership Model Exploration (Rust):
     - Explore more advanced use cases of Rust's ownership model, such as introducing lifetime annotations and borrowing patterns, to optimize resource management        further.
4. Concurrency Optimization:
     - Implement concurrency optimizations, leveraging Rust's ownership model to introduce safe and efficient concurrent operations. This can significantly               improve performance, especially in scenarios with multiple simultaneous note edits.
