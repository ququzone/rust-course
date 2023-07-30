macro_rules! vec_pow_10 {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(10_u128.pow($x));
            )*
            temp_vec
        }
    };
}

fn main() {
    let test = vec_pow_10!(2, 4);
    println!("{:?}", test);
}
