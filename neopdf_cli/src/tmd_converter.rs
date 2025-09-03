//! CLI logic for `NeoPDF` TMD conversion utilities.

use neopdf::gridpdf::GridArray;
use neopdf::metadata::{InterpolatorType, MetaData, MetaDataV1, SetType};
use neopdf::subgrid::SubGrid;
use neopdf::writer::GridArrayCollection;
use neopdf_tmdlib::Tmd;
use serde::Deserialize;
use std::f64::consts::PI;
use std::fs;

#[derive(Deserialize)]
struct TmdConfig {
    set_name: String,
    set_desc: String,
    set_index: u32,
    n_members: Option<usize>,
    n_x: usize,
    n_q: usize,
    n_kt: usize,
    x_inner_edges: [f64; 2],
    q_inner_edges: [f64; 2],
    kt_inner_edges: [f64; 2],
    pids: Vec<i32>,
    nucleons: Vec<f64>,
    alphas: Vec<f64>,
    alphas_qs: Vec<f64>,
    alphas_vals: Vec<f64>,
    polarised: bool,
    set_type: String,
    interpolator_type: String,
    error_type: String,
    hadron_pid: i32,
    flavor_scheme: String,
    order_qcd: u32,
    alphas_order_qcd: u32,
    m_w: f64,
    m_z: f64,
    m_up: f64,
    m_down: f64,
    m_strange: f64,
    m_charm: f64,
    m_bottom: f64,
    m_top: f64,
    alphas_type: String,
    number_flavors: u32,
}

#[allow(clippy::cast_precision_loss)]
fn create_cheby_grid(n_points: usize, min: f64, max: f64) -> Vec<f64> {
    let (u_min, u_max) = (min.ln(), max.ln());
    let range = u_max - u_min;
    let n_points_f64 = n_points as f64;
    let n_minus_1 = n_points_f64 - 1.0;

    (0..n_points)
        .map(|j| {
            let j_f64 = j as f64;
            let t_j = (PI * (n_minus_1 - j_f64) / n_minus_1).cos();
            (u_min + range * (t_j + 1.0) / 2.0).exp()
        })
        .collect()
}

fn parse_set_type(s: &str) -> Result<SetType, String> {
    match s.to_ascii_lowercase().as_str() {
        "spacelike" => Ok(SetType::SpaceLike),
        "timelike" => Ok(SetType::TimeLike),
        _ => Err(format!("Unknown SetType: {s}")),
    }
}

fn parse_interpolator_type(s: &str) -> Result<InterpolatorType, String> {
    match s.to_ascii_lowercase().as_str() {
        "logtricubic" => Ok(InterpolatorType::LogTricubic),
        "chebyshev" => Ok(InterpolatorType::LogChebyshev),
        _ => Err(format!("Unknown InterpolatorType: {s}")),
    }
}

fn create_grid_data(
    tmd: &mut Tmd,
    config: &TmdConfig,
    kts: &[f64],
    xs: &[f64],
    q2s: &[f64],
) -> Vec<f64> {
    const TMDLIB_PIDS: &[i32] = &[-6, -5, -4, -3, -2, -1, 21, 1, 2, 3, 4, 5, 6];

    let mut grid_data = Vec::new();

    for _nuc in &config.nucleons {
        for _alpha in &config.alphas {
            for &kt in kts {
                for &x in xs {
                    for &q2 in q2s {
                        let tmd_pds = tmd.xfxq2kt(x, kt, q2.sqrt());
                        for &pid in &config.pids {
                            let value = TMDLIB_PIDS
                                .iter()
                                .position(|&p| p == pid)
                                .map_or(0.0, |pos| tmd_pds[pos]);
                            grid_data.push(value);
                        }
                    }
                }
            }
        }
    }

    grid_data
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
fn create_member_grid(
    tmd: &mut Tmd,
    config: &TmdConfig,
    member: usize,
    kts: &[f64],
    xs: &[f64],
    q2s: &[f64],
) -> GridArray {
    tmd.init(&config.set_name, member as i32);

    let grid_data = create_grid_data(tmd, config, kts, xs, q2s);

    let subgrid = SubGrid::new(
        config.nucleons.clone(),
        config.alphas.clone(),
        kts.to_vec(),
        xs.to_vec(),
        q2s.to_vec(),
        config.pids.len(),
        grid_data,
    );

    GridArray {
        pids: config.pids.clone().into(),
        subgrids: vec![subgrid],
    }
}

/// TODO
///
/// # Errors
///
/// TODO
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
pub fn convert_tmd(input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config: TmdConfig = toml::from_str(&fs::read_to_string(input_path)?)?;

    let mut tmd = Tmd::new();
    tmd.set_verbosity(0);
    tmd.init_set(&config.set_name);

    let n_members = config.n_members.unwrap_or_else(|| tmd.num_members());

    let xs = create_cheby_grid(config.n_x, config.x_inner_edges[0], config.x_inner_edges[1]);
    let q2s = create_cheby_grid(
        config.n_q,
        config.q_inner_edges[0].powi(2),
        config.q_inner_edges[1].powi(2),
    );
    let kts = create_cheby_grid(
        config.n_kt,
        config.kt_inner_edges[0],
        config.kt_inner_edges[1],
    );

    let member_grids: Vec<_> = (0..n_members)
        .map(|m| create_member_grid(&mut tmd, &config, m, &kts, &xs, &q2s))
        .collect();

    let member_grid_refs: Vec<&GridArray> = member_grids.iter().collect();

    let meta = MetaData::new_v1(MetaDataV1 {
        set_desc: config.set_desc,
        set_index: config.set_index,
        num_members: n_members as u32,
        x_min: tmd.x_min(),
        x_max: tmd.x_max(),
        q_min: tmd.q2_min().sqrt(),
        q_max: tmd.q2_max().sqrt(),
        flavors: config.pids.clone(),
        format: "neopdf".to_string(),
        alphas_q_values: config.alphas_qs,
        alphas_vals: config.alphas_vals,
        polarised: config.polarised,
        set_type: parse_set_type(&config.set_type)?,
        interpolator_type: parse_interpolator_type(&config.interpolator_type)?,
        error_type: config.error_type,
        hadron_pid: config.hadron_pid,
        git_version: String::new(),
        code_version: String::new(),
        flavor_scheme: config.flavor_scheme,
        order_qcd: config.order_qcd,
        alphas_order_qcd: config.alphas_order_qcd,
        m_w: config.m_w,
        m_z: config.m_z,
        m_up: config.m_up,
        m_down: config.m_down,
        m_strange: config.m_strange,
        m_charm: config.m_charm,
        m_bottom: config.m_bottom,
        m_top: config.m_top,
        alphas_type: config.alphas_type,
        number_flavors: config.number_flavors,
    });

    GridArrayCollection::compress(&member_grid_refs, &meta, output_path)?;
    println!("Compression succeeded!");

    Ok(())
}
