use codegraph::read_file;

fn main() {
    let path = "data/2021-10-11-15.json.gz";
    read_file(&path).unwrap();
}
