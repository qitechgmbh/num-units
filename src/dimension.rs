#[macro_export]
macro_rules! dimension {
    // Create a struct and apply the proc macro attribute to it
    ($name:ident, $($dim:ident),+) => {
        #[::num_units_macros::dimension($($dim),+)]
        pub struct $name;
    };
}
