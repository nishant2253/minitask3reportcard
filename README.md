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
git clone https://github.com/nishant2253/student-report-card.git
cd student-report-card
