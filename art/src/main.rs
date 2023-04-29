use art;

fn main() {
    let c1 = art::PrimaryColor::Red;
    let c2 = art::PrimaryColor::Blue;
    let c3 = art::mix(c1, c2)
        .unwrap_or_else(|err| {
            eprintln!("{err}");
            art::SecondaryColor::Green
        });
    println!("{:?}", c3);
}
