# 📝 Rust-Based Student Report Card Generator

This is a console-based Rust application that takes a student's name, total marks, and number of subjects, calculates their average score, assigns a grade, and generates a neat **PDF report card**.

---

## 📦 Features

- Collects student details via CLI input
- Calculates average marks using a custom function
- Assigns grade based on standard scale:
  - A: 90+
  - B: 75–89
  - C: 60–74
  - D: Below 60
- Generates a printable PDF report card
- Easy to run locally and push to GitHub

---

## 🧑‍💻 Technologies Used

- **Rust** — Safe & fast systems programming language
- **printpdf** — Rust crate for generating PDF documents
- **chrono** — For timestamping the report
- **std::io** — For handling user input and output

---

## 🛠️ Setup Instructions

Follow these steps to clone and run the project on your local system:

### 1. 📥 Clone the Repository

```bash
git clone https://github.com/YOUR_USERNAME/student-report-card.git
cd student-report-card
```

> Replace `YOUR_USERNAME` with your actual GitHub username.

### 2. 🦀 Make Sure Rust Is Installed

If not already installed, install Rust using:

```bash
curl https://sh.rustup.rs -sSf | sh
```

Check version:

```bash
rustc --version
```

---

## 📁 Project Structure

```
student-report-card/
├── Cargo.toml         # Project dependencies
└── src/
    └── main.rs        # Main logic
```

---

## 📜 Workflow of the Code

### ➤ `main.rs` consists of:

#### 1. **Student Struct**
Stores student info: name, total marks, number of subjects.

#### 2. **average() Method**
Calculates average marks:

```rust
total_marks / num_subjects
```

#### 3. **grade() Method**
Assigns grade based on average score:
- A: ≥ 90
- B: 75–89
- C: 60–74
- D: < 60

#### 4. **create_pdf() Function**
Generates a formatted PDF using the `printpdf` crate and saves it as `report_card.pdf`.

#### 5. **Main Flow**
- Prompts user for name, total marks, number of subjects
- Creates a `Student` instance
- Prints the report in the terminal
- Generates a clean PDF report

---

## 🚀 Running the Project

To compile and run the app:

```bash
cargo run
```

You’ll be prompted to enter:
- Student name
- Total marks
- Number of subjects

The app will:
- Print the calculated average and grade in the terminal
- Generate a file called `report_card.pdf` in your project root directory

---

## 🖨️ Sample Terminal Output

```bash
Enter student name:
John Doe
Enter total marks:
450
Enter number of subjects:
5

--- Report Card ---
Name: John Doe
Average Marks: 90.00
Grade: A
PDF report generated as report_card.pdf
```

---

## 📤 Push to GitHub

After confirming the project works and the PDF is generated, you can push it to GitHub.

```bash
git init
git add .
git commit -m "Initial commit: Add report card generator in Rust"
git remote add origin https://github.com/YOUR_USERNAME/student-report-card.git
git push -u origin main
```

Then submit the repository URL wherever required.

---

## 📌 Notes

- PDF will be saved in the root directory as `report_card.pdf`.
- Designed for one student at a time (can be expanded later).
- Future enhancements:
  - Support for multiple students
  - Table-formatted PDF
  - Emailing the PDF automatically

---

## 👨‍💻 Author

Developed by **Nishant Gupta**  
📫 [LinkedIn](https://www.linkedin.com/in/-nishant-gupta-/)  
💻 [GitHub](https://github.com/nishant2253)

---

## ✅ License

This project is licensed under the [MIT License](LICENSE).
