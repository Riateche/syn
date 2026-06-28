#[test]
fn issue1108() {
    let data = "impl<x<>>::x for";
    let _ = syn_send::parse_file(data);
}
