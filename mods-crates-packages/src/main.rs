use mods_crates_packages::tools::caps;

mod hello {
    pub fn say_hello(name: &str) {
        println!("Hi,{} :)",name);
    }

}

fn main() {
    hello::say_hello("Lou");

    let name_lc = "lou";
    let name_uc = caps(name_lc);
    println!("{} capitalised is {}", name_lc, name_uc);
}
