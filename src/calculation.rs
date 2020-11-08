#[derive(Debug, PartialEq)]
pub enum HValue {
    M,
    P,
    T
}

#[derive(Debug, PartialEq)]
pub enum Error {
    NoMatchingRules
}

pub fn calculate(a: bool, b: bool, c: bool, d: f64, e: i64, f: i64) -> Result<(HValue, f64), Error> {
    Ok((HValue::M, 1.0))
}

#[test]
fn base_test_1() {
    let a = true;
    let b = true;
    let c = true;
    let d = 1.5;
    let e = 6;
    let f = 7;
    let (h, k) = calculate(a, b, c, d, e, f).unwrap();
    assert_eq!(h, HValue::P);
    assert_eq!(k, d + (d*(e-f) as f64/25.5));
}

#[test]
fn base_test_2() {
    let a = true;
    let b = true;
    let c = false;
    let d = 1.5;
    let e = 6;
    let f = 7;
    let (h, k) = calculate(a, b, c, d, e, f).unwrap();
    assert_eq!(h, HValue::M);
    assert_eq!(k, d + (d*e as f64 / 10 as f64));
}

#[test]
fn base_test_3() {
    let a = false;
    let b = true;
    let c = true;
    let d = 1.5;
    let e = 6;
    let f = 7;
    let (h, k) = calculate(a, b, c, d, e, f).unwrap();
    assert_eq!(h, HValue::T);
    assert_eq!(k, d - (d*f as f64 / 30 as f64));
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
    let (h, k) = calculate(a, b, c, d, e, f).unwrap();
    assert_eq!(h, HValue::T);
    assert_eq!(k, 2*d + (d*e/100));
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
    let (h, k) = calculate(a, b, c, d, e, f).unwrap();
    assert_eq!(h, HValue::T);
    assert_eq!(k, d - (d*f as f64 / 30 as f64));
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
    let (h, k) = calculate(a, b, c, d, e, f).unwrap();
    assert_eq!(h, HValue::M);
    assert_eq!(k, f + d + (d*e / 100));
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
    let (h, k) = calculate(a, b, c, d, e, f).unwrap();
    assert_eq!(h, HValue::T);
    assert_eq!(k, d - (d*f as f64 / 30 as f64));
}