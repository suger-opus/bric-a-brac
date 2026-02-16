use metadata::presentation::openapi;

fn main() {
    let json = openapi::get_openapi_json();
    println!("{}", json);
}
