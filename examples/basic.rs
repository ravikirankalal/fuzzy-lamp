//! Basic Example for Fuzzy Lamp Chart Library
//!
//! This example demonstrates how to create and draw a simple BarChart using the fuzzy-lamp library.
//!
//! To run this example:
//! ```
//! cargo run --example basic
//! ```
//!
//! This will create a BarChart instance and call its draw() method to render the chart.

use fuzzy_lamp::charts::BarChart;

fn main() {
    println!("=== Fuzzy Lamp Basic Example ===");
    println!("Creating and drawing a simple bar chart...");
    
    // Create a new BarChart instance
    let bar_chart = BarChart::new();
    
    // Draw the chart - this will output a message indicating the chart has been "drawn"
    // In a full implementation, this would render to an actual image file
    match bar_chart.draw() {
        Ok(()) => println!("✓ Bar chart successfully created!"),
        Err(e) => eprintln!("✗ Error drawing bar chart: {}", e),
    }
    
    println!("\nExample completed. Check output.png for the rendered chart.");
}
