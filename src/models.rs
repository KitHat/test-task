use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub enum HValue {
    M,
    P,
    T
}

#[derive(Debug, PartialEq, Serialize, Responder)]
pub enum Error {
    #[response(status=404, content_type="plain")]
    NoMatchingRules(String)
}

#[derive(Serialize, Debug)]
pub struct CalcResult {
    pub h: HValue,
    pub k: f64
}
