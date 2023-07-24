mod operator;
mod vec_type;
mod vev_trait_object;

fn main() {
    println!("------ iterator vec use enum wrap ------");
    vec_type::iterator();

    println!("------ iterator vec use triat object ------");
    vev_trait_object::iterator();

    println!("------ Operator add ------");
    operator::demo();
}
