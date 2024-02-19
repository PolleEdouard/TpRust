pub fn add(left: usize, right: usize) -> usize {
    left + right
}
use clap::Parser;
#[derive(Parser)]
pub struct Options{
    pub host: String,
    pub port:u16,

}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
