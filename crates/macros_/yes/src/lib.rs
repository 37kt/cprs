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

#[macro_export]
macro_rules! takahashi {
    () => {{
        println!("Takahashi");
    }};
    ($a:expr) => {{
        if $a {
            println!("Takahashi");
        } else {
            println!("Aoki");
        }
    }};
}

#[macro_export]
macro_rules! alice {
    () => {{
        println!("Alice");
    }};
    ($a:expr) => {{
        if $a {
            println!("Alice");
        } else {
            println!("Bob");
        }
    }};
}
