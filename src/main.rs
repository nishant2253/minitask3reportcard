// Import necessary libraries
use printpdf::*; // For generating PDF
use std::fs::File;
use std::io::{self, BufWriter}; // For user input and writing PDF
use chrono::Local; // For adding current date to the report

// Define a struct to hold student details
#[derive(Debug)]
struct Student {
    name: String,
    total_marks: f32,
    num_subjects: u32,
}

// Implement methods for Student struct
impl Student {
    // Method to calculate average marks
    fn average(&self) -> f32 {
        self.total_marks / self.num_subjects as f32
    }

    // Method to assign grade based on average
    fn grade(&self) -> char {
        let avg = self.average();
        match avg {
            x if x >= 90.0 => 'A',
            x if x >= 75.0 => 'B',
            x if x >= 60.0 => 'C',
            _ => 'D',
        }
    }
}

fn main() {
    // Collect student details from user input
    let mut name = String::new();
    let mut total = String::new();
    let mut subjects = String::new();

    println!("Enter student name:");
    io::stdin().read_line(&mut name).unwrap();

    println!("Enter total marks:");
    io::stdin().read_line(&mut total).unwrap();

    println!("Enter number of subjects:");
    io::stdin().read_line(&mut subjects).unwrap();

    // Create a Student instance with parsed input
    let student = Student {
        name: name.trim().to_string(),
        total_marks: total.trim().parse::<f32>().unwrap(),
        num_subjects: subjects.trim().parse::<u32>().unwrap(),
    };

    // Print basic report to terminal
    println!("\n--- Report Card ---");
    println!("Name: {}\n", student.name);
    println!("Average Marks: {:.2}\n", student.average());
    println!("Grade: {}\n", student.grade());

    // Generate and save PDF report card
    create_pdf(&student);
}

// Function to generate a simple PDF report card
fn create_pdf(student: &Student) {
    // Create a new PDF document (A4 size)
    let (doc, page1, layer1) = PdfDocument::new("Report Card", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    // Load a built-in font for the PDF
    let font = doc.add_builtin_font(BuiltinFont::HelveticaBold).unwrap();

    // Format the student data as text for the PDF
    let text = format!(
        "Student Report Card\n\n\
        Name: {}\n\
        Total Marks: {}\n\
        Subjects: {}\n\
        Average: {:.2}\n\
        Grade: {}\n\
        Date: {}",
        student.name,
        student.total_marks,
        student.num_subjects,
        student.average(),
        student.grade(),
        Local::now().format("%Y-%m-%d")
    );

    // Add text to the PDF layer
    current_layer.use_text(text, 14.0, Mm(20.0), Mm(250.0), &font);

    // Save the PDF as "report_card.pdf"
    let mut file = BufWriter::new(File::create("report_card.pdf").unwrap());
    doc.save(&mut file).unwrap();

    println!("PDF report generated as report_card.pdf");
}
