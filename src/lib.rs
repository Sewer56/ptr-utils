#![doc = include_str!("../README.MD")]
#![no_std]
#[cfg(feature = "std")]
extern crate std;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
