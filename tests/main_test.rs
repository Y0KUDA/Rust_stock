#[test]
fn main_test() {

}
extern crate rust_stock;
#[cfg(test)]
mod main_test {
    #[test]
    fn sum_test_1(){
        assert_eq!(47, rust_stock::main::sum(2,45));
    }
} 