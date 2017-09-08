extern crate ipconfig;


#[test]
fn no_error() {
    println!("{:#?}", ipconfig::get_adapters().unwrap());
}
