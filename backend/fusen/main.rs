use domain::entity::{Fusen, FusenBuilder};
use domain::vo::{FusenNote, FusenTitle, Id};

fn main() {
    let fusen = FusenBuilder::default()
        .id("01F8MECHZX3TBDSZ7XRADM79XE".parse::<Id<Fusen>>().unwrap())
        .title("title".parse::<FusenTitle>().unwrap())
        .note("note".parse::<FusenNote>().unwrap())
        .build()
        .unwrap();

    println!("fusen: {:?}", fusen);
}
