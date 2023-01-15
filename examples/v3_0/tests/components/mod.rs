mod request_bodies;
mod schemas;

fn flatten(x: &str) -> String {
    x.replace([' ', '\n'], "")
}
