#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate matches;
use rocket_contrib::json::Json;

mod calculation;

#[get("/calculate?<a>&<b>&<c>&<d>&<e>&<f>")]
fn calculate(a: bool, b: bool, c: bool, d: f64, e: i64, f: i64) -> Result<Json<calculation::CalcResult>, calculation::Error> {
    calculation::calculate(a, b, c, d, e, f).map(|res| Json(res))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![calculate]).launch();
}
