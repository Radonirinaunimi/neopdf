use criterion::{criterion_group, criterion_main, Criterion};

use neopdf::pdf::PDF;

fn xfxq2(c: &mut Criterion) {
    let pdf = PDF::load("NNPDF40_nnlo_as_01180", 0);

    c.bench_function("xfxq2", |b| {
        b.iter(|| pdf.xfxq2(std::hint::black_box(21), std::hint::black_box(&[1e-3, 4.0])))
    });
}

fn xfxq2_cheby(c: &mut Criterion) {
    let pdf = PDF::load("MAP22_grids_FF_Km_N3LL.neopdf.lz4", 0);

    c.bench_function("xfxq2_cheby", |b| {
        b.iter(|| {
            pdf.xfxq2(
                std::hint::black_box(2),
                std::hint::black_box(&[1e-2, 5e-1, 10.0]),
            )
        })
    });
}

fn xfxq2_cheby_batch(c: &mut Criterion) {
    let pdf = PDF::load("MAP22_grids_FF_Km_N3LL.neopdf.lz4", 0);

    c.bench_function("xfxq2_cheby_batch", |b| {
        b.iter(|| {
            pdf.xfxq2_cheby_batch(
                std::hint::black_box(2),
                std::hint::black_box(&[&[1e-2, 5e-1, 10.0]]),
            )
        })
    });
}

fn xfxq2s(c: &mut Criterion) {
    let pdf = PDF::load("NNPDF40_nnlo_as_01180", 0);

    let ids: Vec<i32> = (-4..=4).filter(|&x| x != 0).collect();
    let xs = [1e-5, 1e-3, 1e-3, 1.0];
    let q2s = [5.0, 10.0, 100.0];

    let flatten_points: Vec<Vec<f64>> = xs
        .iter()
        .flat_map(|&x| q2s.iter().map(move |&q2| vec![x, q2]))
        .collect();
    let points_interp: Vec<&[f64]> = flatten_points.iter().map(Vec::as_slice).collect();
    let slice_points: &[&[f64]] = &points_interp;

    c.bench_function("xfxq2s", |b| {
        b.iter(|| {
            pdf.xfxq2s(
                std::hint::black_box(ids.clone()),
                std::hint::black_box(slice_points),
            )
        })
    });
}

fn xfxq2_members(c: &mut Criterion) {
    let pdfs = PDF::load_pdfs("NNPDF40_nnlo_as_01180");

    c.bench_function("xfxq2_members", |b| {
        b.iter(|| {
            pdfs.iter()
                .map(|pdf| pdf.xfxq2(std::hint::black_box(21), std::hint::black_box(&[1e-3, 4.0])))
        })
    });
}

criterion_group!(
    benches,
    xfxq2,
    xfxq2s,
    xfxq2_members,
    xfxq2_cheby,
    xfxq2_cheby_batch
);
criterion_main!(benches);
