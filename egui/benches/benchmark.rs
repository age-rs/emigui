use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut example_app = egui::examples::ExampleApp::default();
    let mut ctx = egui::Context::new();

    let raw_input = egui::RawInput {
        screen_size: egui::vec2(1280.0, 1024.0),
        ..Default::default()
    };

    c.bench_function("example_app", |b| {
        b.iter(|| {
            let mut ui = ctx.begin_frame(raw_input.clone());
            example_app.ui(&mut ui, "");
            ctx.end_frame()
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
