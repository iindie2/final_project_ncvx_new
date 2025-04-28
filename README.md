Final Project: NCVS Data Analysis
For my final project, I analyzed the National Crime Victimization Survey (NCVS) dataset. This dataset contains information about crimes reported by individuals in the United States. The project focuses on examining key metrics like average income, income distribution, and reporting rates by gender. By using Rust, I processed and cleaned the data to calculate these important statistics, which can provide insights into the reporting behaviors of different demographics.
Key Features of the Project:
Data Cleaning: The data is cleaned to remove any invalid records (such as missing or negative values). This ensures that the analysis is based on reliable information.
Statistics Calculations: Once the data is cleaned, the project calculates:
Total number of clean rows
Average income
Minimum and maximum income
Reporting rates based on gender
Sex-based Reporting Rates: The project calculates the percentage of males and females who reported a crime and outputs the results.
How to Run the Project:
Clone the Repository: Download the project from GitHub.
bash
Copy
https://github.com/iindie2/final_project_ncvx_new/tree/main
Install Dependencies: Make sure you have Rust installed. If not, install it from rust-lang.org.
Run the Project: Inside the project directory, run the following commands:
bash
Copy
cargo build
cargo run
This will compile and run the program. It will read the dataset, clean it, and print the results to the terminal.
View the Output: After running the program, you will see output like this:
bash
Copy
✅ Total clean rows: 1000
💵 Average income: 60280.86
💵 Min income: 20036.719
💵 Max income: 99931.88
📊 Reporting rate by sex:
♂️ Male reporting rate: 23.50%
♀️ Female reporting rate: 27.10%
Output Description:
Total Clean Rows: The number of valid records used in the analysis (cleaned from invalid data).
Average Income: The mean income of all individuals in the dataset after filtering out invalid data.
Min/Max Income: The lowest and highest income values found in the dataset.
Reporting Rate by Sex: The percentage of male and female respondents who reported a crime.
Testing:
The project includes unit tests to verify that the data cleaning, statistics calculation, and sex-based reporting rate calculations are working correctly.
bash
Copy
cargo test

This will run tests on the core functionality of the project.
Conclusion:
This project applies Rust to clean and process a real-world dataset. It demonstrates how simple data analysis and cleaning tasks can be done efficiently using Rust’s powerful type system and iterators. The project not only meets the technical requirements but also provides valuable insights into crime reporting behavior, which could be used for further research or policy recommendations.



