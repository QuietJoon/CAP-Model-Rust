fn main() {
    let stdout = stdout();
    let lock = stdout.lock();
    let mut _buf = BufWriter::new(lock);

    fastp!(
        _buf,
        "\n\n----------------------------------------------------------------\n\n\
         \t\tStart FileComparing-Rust!\n\n\
         ----------------------------------------------------------------\n\n"
    );
}
