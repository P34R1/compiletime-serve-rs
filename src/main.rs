mod files {
    include!(concat!(env!("OUT_DIR"), "/generated_files.rs"));
}

fn main() {
    println!("{:?}", files::index_html);
}
