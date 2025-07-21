//! # Fuzzy Lamp
//!
//! A Rust library for creating and rendering charts. This library supports various types of charts like bar charts and line charts.
//!
//! ## Example Usage
//!
//! ```rust
//! use fuzzy_lamp::charts::{BarChart, LineChart};
//!
//! let bar_chart = BarChart::new();
//! bar_chart.draw();
//!
//! let line_chart = LineChart::new();
//! line_chart.draw();
//! ```

pub mod charts;

pub use charts::{BarChart, LineChart};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_charts_creation() {
        let bar_chart = BarChart::new();
        let line_chart = LineChart::new();
        
        bar_chart.draw().expect("Failed to draw bar chart");
        line_chart.draw().expect("Failed to draw line chart");
    }
}
