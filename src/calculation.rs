use crate::models::{CalcResult, HValue, Error};

pub fn calculate(a: bool, b: bool, c: bool, d: f64, e: i64, f: i64) -> Result<CalcResult, Error> {
    let h = _h_calc(a, b, c)?;
    let k = _k_calc(&h, d, e, f);
    Ok(CalcResult{h, k})
}

fn _h_calc(a: bool, b: bool, c: bool) -> Result<HValue, Error> {
    let h = match (a, b, c) {
        (true, true, false) => Ok(HValue::M),
        (true, true, true) => Ok(HValue::P),
        (false, true, true) => Ok(HValue::T),
        _ => Err(Error::NoMatchingRules("No matching rules for this input".to_string()))
    };
    #[cfg(feature="custom2")]
    let h = match (a, b, c, h) {
        (true, true, false, _) => Ok(HValue::T),
        (true, false, true, _) => Ok(HValue::M),
        (_, _, _, e) => e
    };
    h
}

fn _k_calc(h: &HValue, d: f64, e: i64, f: i64) -> f64 {
    let k = match h {
        &HValue::M => d + (d*e as f64 / 10.0),
        &HValue::P => d + (d*(e-f) as f64/25.5),
        &HValue::T => d - (d*f as f64 / 30.0)
    };
    #[cfg(feature="custom1")]
    let k = match (h, k) {
        (&HValue::P, _) => 2.0*d + (d*e as f64/100.0),
        (_, k) => k
    };
    #[cfg(feature="custom2")]
    let k = match (h, k) {
        (&HValue::M, _) => f as f64 + d + (d*e  as f64 / 100.0),
        (_, k) => k
    };
    k
}

#[test]
#[cfg(not(feature="custom1"))]
fn base_test_1() {
    let a = true;
    let b = true;
    let c = true;
    let d = 1.5;
    let e = 6;
    let f = 7;
    let CalcResult{h, k} = calculate(a, b, c, d, e, f).unwrap();
    assert_eq!(h, HValue::P);
    assert_eq!(k, d + (d*(e-f) as f64/25.5));
}

#[test]
#[cfg(not(feature="custom2"))]
fn base_test_2() {
    let a = true;
    let b = true;
    let c = false;
    let d = 1.5;
    let e = 6;
    let f = 7;
    let CalcResult{h, k} = calculate(a, b, c, d, e, f).unwrap();
    assert_eq!(h, HValue::M);
    assert_eq!(k, d + (d*e as f64 / 10.0));
}

#[test]
fn base_test_3() {
    let a = false;
    let b = true;
    let c = true;
    let d = 1.5;
    let e = 6;
    let f = 7;
    let CalcResult{h, k} = calculate(a, b, c, d, e, f).unwrap();
    assert_eq!(h, HValue::T);
    assert_eq!(k, d - (d*f as f64 / 30.0));
}

#[test]
fn base_test_4() {
    let a = false;
    let b = false;
    let c = false;
    let d = 1.5;
    let e = 6;
    let f = 7;
    let e = calculate(a, b, c, d, e, f).unwrap_err();
    assert_matches!(e, Error::NoMatchingRules(_));
}

#[test]
#[cfg(not(feature="custom2"))]
fn base_test_5() {
    let a = true;
    let b = false;
    let c = true;
    let d = 1.5;
    let e = 6;
    let f = 7;
    let e = calculate(a, b, c, d, e, f).unwrap_err();
    assert_matches!(e, Error::NoMatchingRules(_));
}

#[test]
#[cfg(feature="custom1")]
fn custom1_test() {
    let a = true;
    let b = true;
    let c = true;
    let d = 1.5;
    let e = 6;
    let f = 7;
    let CalcResult{h, k} = calculate(a, b, c, d, e, f).unwrap();
    assert_eq!(h, HValue::P);
    assert_eq!(k, 2.0*d + (d*e as f64/100.0));
}

#[test]
#[cfg(feature="custom2")]
fn custom2_test_1() {
    let a = true;
    let b = true;
    let c = false;
    let d = 1.5;
    let e = 6;
    let f = 7;
    let CalcResult{h, k} = calculate(a, b, c, d, e, f).unwrap();
    assert_eq!(h, HValue::T);
    assert_eq!(k, d - (d*f as f64 / 30.0));
}

#[test]
#[cfg(feature="custom2")]
fn custom2_test_2() {
    let a = true;
    let b = false;
    let c = true;
    let d = 1.5;
    let e = 6;
    let f = 7;
    let CalcResult{h, k} = calculate(a, b, c, d, e, f).unwrap();
    assert_eq!(h, HValue::M);
    assert_eq!(k, f as f64 + d + (d*e as f64 / 100.0));
}

#[test]
#[cfg(feature="custom2")]
fn custom2_test_3() {
    let a = false;
    let b = true;
    let c = true;
    let d = 1.5;
    let e = 6;
    let f = 7;
    let CalcResult{h, k} = calculate(a, b, c, d, e, f).unwrap();
    assert_eq!(h, HValue::T);
    assert_eq!(k, d - (d*f as f64 / 30.0));
}