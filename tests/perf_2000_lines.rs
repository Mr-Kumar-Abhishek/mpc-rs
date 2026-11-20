use mpc::*;
use std::fmt::Write as _;
use std::time::{Duration, Instant};

fn report_perf(label: &str, lines: usize, bytes: usize, elapsed: Duration) {
    let secs = elapsed.as_secs_f64();
    let lines_per_sec = if secs > 0.0 { lines as f64 / secs } else { f64::INFINITY };
    let mb_per_sec = if secs > 0.0 {
        (bytes as f64 / (1024.0 * 1024.0)) / secs
    } else {
        f64::INFINITY
    };
    println!(
        "[perf] {label}: {lines} lines ({bytes} bytes) parsed in {ms:.2} ms ({lines_per_sec:.0} lines/s, {mb_per_sec:.2} MB/s)",
        ms = secs * 1000.0
    );
}

fn gen_plain_lines(count: usize) -> String {
    let mut out = String::with_capacity(count * 36);
    for i in 0..count {
        let _ = writeln!(out, "line {i:04}: hello world from mpc-rs");
    }
    out
}

fn gen_kv_lines(count: usize) -> String {
    let mut out = String::with_capacity(count * 24);
    for i in 0..count {
        match i % 4 {
            0 => {
                let _ = writeln!(out, "key_{i:04} = 12345");
            }
            1 => {
                let _ = writeln!(out, "key_{i:04} = 0x1aF");
            }
            2 => {
                let _ = writeln!(out, "key_{i:04} = 3.14f");
            }
            _ => {
                let _ = writeln!(out, "key_{i:04} = \"val_{i:04}\"");
            }
        }
    }
    out
}

fn build_kv_parser() -> MpcParser {
    let value = mpc_or(vec![mpc_float(), mpc_number(), mpc_string_lit()]);
    let line = mpc_and(
        vec![
            mpc_ident(),
            mpc_whitespaces(),
            mpc_char('='),
            mpc_whitespaces(),
            value,
            mpc_newline(),
        ],
        mpcf_null,
    );
    mpc_and(vec![mpc_many(line, mpcf_null), mpc_eoi()], mpcf_null)
}

#[test]
fn parse_2000_lines_plain_text() {
    let data = gen_plain_lines(2000);
    let line = mpc_and(
        vec![mpc_many(mpc_noneof("\n"), mpcf_strfold), mpc_newline()],
        mpcf_fst,
    );
    let parser = mpc_and(vec![mpc_many(line, mpcf_null), mpc_eoi()], mpcf_null);

    let mut input = MpcInput::new("plain_2000.txt", &data);
    let start = Instant::now();
    let res = parser.parse(&mut input);
    let elapsed = start.elapsed();

    assert!(matches!(res, MpcResult::Ok(_)), "expected plain text parse to succeed");
    assert_eq!(input.state.row, 2000);
    assert_eq!(input.pos, data.len());
    report_perf("plain_text", 2000, data.len(), elapsed);

    #[cfg(not(debug_assertions))]
    assert!(
        elapsed < Duration::from_secs(1),
        "release parse exceeded 1s: {:?}",
        elapsed
    );
}

#[test]
fn parse_2000_lines_key_value_config() {
    let data = gen_kv_lines(2000);
    let parser = build_kv_parser();

    let mut input = MpcInput::new("config_2000.cfg", &data);
    let start = Instant::now();
    let res = parser.parse(&mut input);
    let elapsed = start.elapsed();

    assert!(matches!(res, MpcResult::Ok(_)), "expected kv config parse to succeed");
    assert_eq!(input.state.row, 2000);
    assert_eq!(input.pos, data.len());
    report_perf("key_value_config", 2000, data.len(), elapsed);

    #[cfg(not(debug_assertions))]
    assert!(
        elapsed < Duration::from_secs(1),
        "release parse exceeded 1s: {:?}",
        elapsed
    );
}

#[test]
fn parse_2000_lines_ast_expressions() {
    let mut data = String::with_capacity(2000 * 22);
    for i in 0..2000 {
        let _ = writeln!(data, "var_{i:04} = 100 + 200");
    }

    let ident = mpca_tag(mpc_ident(), "var");
    let num = mpca_tag(mpc_int(), "num");
    let op = mpca_tag(mpc_oneof("+-"), "op");
    let assign = mpca_tag(
        mpc_and(
            vec![
                ident,
                mpc_whitespaces(),
                mpc_char('='),
                mpc_whitespaces(),
                num.clone(),
                mpc_whitespaces(),
                op,
                mpc_whitespaces(),
                num,
                mpc_newline(),
            ],
            mpcf_fold_ast,
        ),
        "assign",
    );
    let program = mpca_root(mpca_tag(
        mpc_and(vec![mpc_many(assign, mpcf_fold_ast), mpc_eoi()], mpcf_fst),
        "program",
    ));

    let mut input = MpcInput::new("expr_2000.mpc", &data);
    let start = Instant::now();
    let res = program.parse(&mut input);
    let elapsed = start.elapsed();

    match res {
        MpcResult::Ok(val) => {
            let ast = val.downcast_ref::<MpcAst>().expect("expected MpcAst");
            assert_eq!(ast.tag, ">|program");
            assert_eq!(ast.children_num, 2000);
        }
        MpcResult::Err(err) => panic!("unexpected err: {}", err),
    }
    assert_eq!(input.state.row, 2000);
    assert_eq!(input.pos, data.len());
    report_perf("ast_expressions", 2000, data.len(), elapsed);

    #[cfg(not(debug_assertions))]
    assert!(
        elapsed < Duration::from_secs(1),
        "release parse exceeded 1s: {:?}",
        elapsed
    );
}

#[test]
fn parse_2000_lines_csv_rows() {
    let mut data = String::with_capacity(2000 * 32);
    for i in 0..2000 {
        let _ = writeln!(data, "{i:04},item_{i:04},3.14,\"desc_{i:04}\"");
    }

    let field = mpc_or(vec![
        mpc_float(),
        mpc_int(),
        mpc_string_lit(),
        mpc_ident(),
    ]);
    let row = mpc_and(
        vec![mpc_sepby1(field, mpc_char(','), mpcf_null), mpc_newline()],
        mpcf_null,
    );
    let parser = mpc_and(vec![mpc_many(row, mpcf_null), mpc_eoi()], mpcf_null);

    let mut input = MpcInput::new("rows_2000.csv", &data);
    let start = Instant::now();
    let res = parser.parse(&mut input);
    let elapsed = start.elapsed();

    assert!(matches!(res, MpcResult::Ok(_)), "expected csv parse to succeed");
    assert_eq!(input.state.row, 2000);
    assert_eq!(input.pos, data.len());
    report_perf("csv_rows", 2000, data.len(), elapsed);

    #[cfg(not(debug_assertions))]
    assert!(
        elapsed < Duration::from_secs(1),
        "release parse exceeded 1s: {:?}",
        elapsed
    );
}

#[test]
fn parse_2000_lines_linear_scaling_check() {
    let data_500 = gen_kv_lines(500);
    let data_2000 = gen_kv_lines(2000);
    let parser = build_kv_parser();

    let runs = 3;
    let start_500 = Instant::now();
    for _ in 0..runs {
        let res = mpc_parse("scale_500.cfg", &data_500, &parser);
        assert!(matches!(res, MpcResult::Ok(_)));
    }
    let avg_500 = start_500.elapsed() / runs;

    let start_2000 = Instant::now();
    for _ in 0..runs {
        let res = mpc_parse("scale_2000.cfg", &data_2000, &parser);
        assert!(matches!(res, MpcResult::Ok(_)));
    }
    let avg_2000 = start_2000.elapsed() / runs;

    let ratio = avg_2000.as_secs_f64() / avg_500.as_secs_f64().max(1e-6);
    println!(
        "[perf] scaling_500_to_2000: 500={:.2} ms, 2000={:.2} ms, ratio={:.2}x (4x input)",
        avg_500.as_secs_f64() * 1000.0,
        avg_2000.as_secs_f64() * 1000.0,
        ratio
    );
    assert!(
        ratio < 12.0,
        "expected near-linear scaling (< 12.0x for 4x input), got {:.2}x",
        ratio
    );
}
