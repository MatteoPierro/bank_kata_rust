fn main() {
    println!("Hello, world!");
}


trait AccountService {
    fn deposit(value :i64);
    fn withdraw(value: i64);
    fn print_statement();
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works(){
        assert!(true)
    }

}