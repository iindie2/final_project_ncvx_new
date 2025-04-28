
Project Documentation for NCVS Crime Reporting Analysis

Part 1: Code Comments Summary
Module Headers
At the top of every .rs file, I added a quick sentence explaining what the file is mainly doing.
This helps anyone reading the code know right away what’s inside without having to dig through it.
Function Comments
For every important function, I wrote a short comment above it that covers:
What the function is supposed to do.
What inputs it takes and what it gives back as output.
A basic idea of how it works (like if it's looping through things, filtering, or transforming data).
If the function was super simple, I just gave it a quick one-liner description instead of going into detail.
In-Function Annotations
Inside each function, whenever there was a big step (like splitting lines, filtering data, or a key calculation), I added a small comment to explain what’s happening.
This makes it easier for someone else (or future me) to follow what’s going on without having to guess.
Structs & Enums
For every struct and enum, I added a quick one-line comment explaining what it represents (like a cleaned survey record) and how it’s used in the project.

Part 2: Written Report
A. Project Overview
Goal:
The goal of this project was to explore the NCVS survey data and figure out some basic reporting trends, specifically looking at income levels and differences between males and females.
Dataset:
We used a sample from the National Crime Victimization Survey (NCVS) dataset.
The cleaned sample we worked with ended up being about 1,000 rows, so it was small enough to work with locally without needing to upload to GitHub.

B. Data Processing
We used Rust’s built-in std::fs and the csv crate to open and read the data.
Each line in the CSV was split into fields.
Rows missing important information (like income or reported status) were skipped.
We filtered the data to make sure it was clean and ready for analysis.

C. Code Structure
Modules:
clean_data.rs: Handles loading the CSV file and cleaning the raw data.
analysis.rs: Runs the calculations and prints the results.
Key Functions & Types:
load_and_clean: Reads the CSV file and creates a list of cleaned records.
run_analysis: Goes through the records and calculates average income, reporting rates by sex, and finds min/max income.
CleanRecord (Struct): Represents a cleaned-up version of one survey record.
Main Workflow:
Load the data from the CSV file.
Clean and validate the records.
Run analysis functions on the clean records.
Print out the final statistics.

D. Tests
We ran cargo test to make sure everything worked properly.
Sample output:
bash
CopyEdit
running 2 tests
test test_clean_record_parsing ... ok
test test_average_income_calculation ... ok

What the tests check:
Making sure records are loaded and parsed correctly.
Making sure the average income calculation works as expected.
Why it matters:
If these tests fail, it could mean the core parts of the data loading and analysis are broken, which would make all the results wrong.

E. Results
After cleaning the dataset, the program printed:
✅ Total clean rows: 1,000
💵 Average income: 60,280.86
💵 Minimum income: 20,036.72
💵 Maximum income: 99,931.88
📊 Reporting rate by sex:
♂️ Male reporting rate: 23.50%
♀️ Female reporting rate: 27.10%

Interpretation:
After cleaning, there were 1,000 valid survey records left.
The average income of the people surveyed was about $60,280.
Income ranged from about $20,036 to about $99,931.
Females had a slightly higher crime reporting rate (27.10%) compared to males (23.50%).
The focus of this project was mainly on income statistics and differences between male and female reporting rates.
This project does not separate by age group or by income group (low vs high), since those analyses were not part of the final code.


F. Usage Instructions
How to build and run:
bash
CopyEdit
cargo build --release
cargo run

Command-line args:
No command-line arguments needed — the program runs automatically when you start it.
Expected runtime:
The program takes about 5–10 seconds to run on a regular laptop.

G. AI Assistance Disclosure & Citations
I used ChatGPT to help me plan out the structure for my documentation and organize my code comments.
I made sure to fully understand and customize everything myself.

Sources:
Official Rust Documentation: https://doc.rust-lang.org/
NCVS Dataset Information: https://bjs.ojp.gov/data-collection/ncvs


