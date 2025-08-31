#[macro_export]
macro_rules! system {
    // Create a struct and apply the proc macro attribute to it
    ($name:ident, $($sys:ident),+) => {
        #[::num_units_macros::system($($sys),+)]
        pub struct $name;
    }
}
