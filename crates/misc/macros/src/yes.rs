#[macro_export]
macro_rules! yes {
    () => {{
        println!("Yes");
    }};
    ($a:expr) => {{
        if $a {
            println!("Yes");
        } else {
            println!("No");
        }
    }};
}
