use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn benchmark(c: &mut Criterion) {
    let expr = match calc::parser::parse(include_str!("expr.txt")) {
        Ok(expr) => expr,
        Err(e) => {
            eprintln!("{}", e.report());
            panic!();
        }
    };

    c.bench_function("fold", |c| c.iter(|| black_box(calc::folder::fold(&expr))));

    let ops = calc::rpn::compiler::compile(&expr);
    c.bench_function("rpn", |c| c.iter(|| black_box(calc::rpn::vm::eval(&ops))));

    let (ops, pool) = calc::stack::compiler::compile(&expr);
    c.bench_function("stack", |c| {
        c.iter(|| black_box(calc::stack::vm::eval(&ops, &pool)))
    });
    c.bench_function("unsafe_stack", |c| {
        c.iter(|| black_box(calc::unsafe_stack::vm::eval(&ops, &pool)))
    });

    let (ops, pool, stack_size) = calc::alloc_exact_stack::compiler::compile(&expr);
    c.bench_function("alloc_exact_stack", |c| {
        c.iter(|| black_box(calc::alloc_exact_stack::vm::eval(&ops, &pool, stack_size)))
    });

    let (ops, pool, stack_size) = calc::register::compiler::compile(&expr);
    c.bench_function("register", |c| {
        c.iter(|| black_box(calc::register::vm::eval(&ops, &pool, stack_size)))
    });

    let (ops, pool, stack_size) = calc::unsafe_register::compiler::compile(&expr);
    c.bench_function("unsafe_register", |c| {
        c.iter(|| black_box(calc::unsafe_register::vm::eval(&ops, &pool, stack_size)))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
