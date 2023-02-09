pub fn main() {
    let input = std::fs::read("./replay.opd").unwrap();
    let _a = opd_parser::parse(&input).unwrap();
}
