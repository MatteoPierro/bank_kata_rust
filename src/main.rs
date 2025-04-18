use mockall::automock;

fn main() {
    println!("Hello, world!");
}

trait AccountService {
    fn deposit(&self, value: u64);
    fn withdraw(&self, value: u64);
    fn print_statement(&self);
}

#[automock]
trait Printer {
    fn print(&self, line: &str);
}

struct Account {
    printer: Box<dyn Printer>,
}

impl AccountService for Account {
    fn deposit(&self, value: u64) {
        todo!()
    }

    fn withdraw(&self, value: u64) {
        todo!()
    }

    fn print_statement(&self) {
        self.printer.print("Date       || Amount || Balance");
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;

    use super::*;

    #[test]
    fn it_works() {
        assert!(true)
    }

    #[test]

    fn new_bank_account_statement() {
        let mut printer = MockPrinter::new();
        printer
            .expect_print()
            .with(eq("Date       || Amount || Balance"))
            .times(1)
            .returning(|_| ());
        let account = Account {
            printer: Box::new(printer),
        };
        account.print_statement();
    }
}
