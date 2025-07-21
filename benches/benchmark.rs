use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fuzzy_lamp::charts::bar::BarChart;

fn benchmark_bar_chart_draw(c: &mut Criterion) {
    let bar_chart = BarChart::new();
    
    c.bench_function("BarChart::draw", |b| {
        b.iter(|| {
            // Use black_box to prevent compiler optimizations
            black_box(bar_chart.draw()).unwrap();
        })
    });
}

criterion_group!(benches, benchmark_bar_chart_draw);
criterion_main!(benches);
