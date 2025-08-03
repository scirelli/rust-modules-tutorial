mod steve {
    pub mod vegetables {
        pub struct Asparagus {
            pub name: String,
        }
    }
}

fn main() {
    let veg = crate::steve::vegetables::Asparagus{name: String::from("My butt")};
    let veg_name = &veg.name[..];
    println!("Hello, world!\nI have asparagus comming out of {veg_name}!!");
    let k = &veg.name;
    println!("Hello, world!\nI have asparagus comming out of {k}!!");

    let j = veg.name;
    println!("Hello, world!\nI have asparagus comming out of {j}!!");
}
