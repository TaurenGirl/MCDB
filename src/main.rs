mod utilities;
mod cli_utilities;
fn main() {
    let x = utilities::populate("<Card>", "</Card>", "./src/info.xml".to_string());
    let y: &utilities::Card = &x.contents[0];
    println!("{}", y.owner);
    cli_utilities::start();
}
