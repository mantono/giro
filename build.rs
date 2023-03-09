fn main() {
    #[cfg(feature = "man")]
    build_man();
}

#[cfg(feature = "man")]
fn build_man() {
    use bzip2::*;
    use std::io::Read;

    let man: String = std::fs::read_to_string("git-root.1.man").unwrap();
    let man: &[u8] = man.as_bytes();
    let compr = read::BzEncoder::new(man, Compression::fast());
    let mut decompr = read::BzDecoder::new(compr);
    let mut out: String = String::new();
    decompr.read_to_string(&mut out).unwrap();
    std::fs::write("git-root.1.bz2", out).unwrap();
}
