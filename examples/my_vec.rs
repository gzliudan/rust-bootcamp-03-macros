use anyhow::Result;

fn main() -> Result<()> {
    let v: Vec<i32> = my_vec!["1".parse()?, "2".parse()?, "3".parse()?,];
    println!("{:?}", v);

    let v: Vec<i32> = my_vec![1; 3];
    println!("{:?}", v);
    Ok(())
}

#[macro_export]
macro_rules! my_vec {
    () => {
        Vec::new()
    };
    ($elem:expr; $n:expr) => {
        std::vec::from_elem($elem, $n)
    };
    ($($x:expr),+ $(,)?) => {{
        // let mut temp_vec = Vec::new();
        // $(
        //   temp_vec.push($x);
        // )*
        // temp_vec
        <[_]>::into_vec(Box::new([$($x),*]))
    }};
}
