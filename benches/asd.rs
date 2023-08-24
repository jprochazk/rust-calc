use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn benchmark(c: &mut Criterion) {
    let expr = match calc::parser::parse(include_str!("expr.txt")) {
        Ok(expr) => expr,
        Err(e) => {
            eprintln!("{}", e.report());
            panic!();
        }
    };
    let ops = calc::compiler::compile(&expr);

    c.bench_function("fold", |c| c.iter(|| black_box(calc::folder::fold(&expr))));
    c.bench_function("exec", |c| c.iter(|| black_box(calc::vm::eval(&ops))));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
