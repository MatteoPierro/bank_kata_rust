use mockall::{automock, Predicate};

fn main() {
    println!("Hello, world!");
}

trait AccountService {
    fn deposit(&mut self, value: u64);
    fn withdraw(&mut self, value: u64);
    fn print_statement(&self);
}

#[automock]
trait Printer {
    fn print(&self, line: &str);
}

trait TransactionsRepository {
    fn add(&mut self, transaction: Transaction);
    fn all(&self) -> Vec<Transaction>;
}

struct Account<P: Printer, TR: TransactionsRepository> {
    printer: P,
    transactions_repository: TR,
}

#[derive(Clone)]
enum Transaction {
    Deposit(u64),
}

impl<P: Printer, TR: TransactionsRepository> AccountService for Account<P, TR> {
    fn deposit(&mut self, value: u64) {
        self.transactions_repository.add(Transaction::Deposit(value));
    }

    fn withdraw(&mut self, value: u64) {
        todo!()
    }

    fn print_statement(&self) {
        self.printer.print("Date       || Amount || Balance");
        let mut result = vec![];
        let mut total = 0;
        for transaction in self.transactions_repository.all() {
            match transaction {
                Transaction::Deposit(value) => {
                    total += value;
                    result.push(format!("15/04/2025 || 100    || {total}    "))
                }
            }
        }
        
        result.reverse();

        for line in &result {
            self.printer.print(line);
        }
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;
    use mockall::Sequence;
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
            printer,
            transactions_repository: InMemoryTransactionRepository::new()
        };
        account.print_statement();
    }

    struct InMemoryTransactionRepository {
        records: Vec<Transaction>,
    }

    impl InMemoryTransactionRepository {
        fn new() -> Self {
            InMemoryTransactionRepository {
                records: vec![]
            }
        }
    }

    impl TransactionsRepository for InMemoryTransactionRepository {
        fn add(&mut self, transaction: Transaction) {
            self.records.push(transaction);
        }

        fn all(&self) -> Vec<Transaction> {
            self.records.clone()
        }
    }

    #[test]
    fn bank_account_statement_with_a_deposit() {
        let mut seq = Sequence::new();
        let mut printer = MockPrinter::new();
        printer
            .expect_print()
            .with(eq("Date       || Amount || Balance"))
            .times(1)
            .returning(|_| ())
            .in_sequence(&mut seq);
        printer
            .expect_print()
            .with(eq("15/04/2025 || 100    || 100    "))
            .times(1)
            .returning(|_| ())
            .in_sequence(&mut seq);

        let transactions_repository = InMemoryTransactionRepository::new();
        let mut account = Account {
            printer,
            transactions_repository
        };
        account.deposit(100);
        account.print_statement();
    }

    #[test]
    fn bank_account_statement_with_two_deposits() {
        let mut seq = Sequence::new();
        let mut printer = MockPrinter::new();
        printer
            .expect_print()
            .with(eq("Date       || Amount || Balance"))
            .times(1)
            .returning(|_| ())
            .in_sequence(&mut seq);
        printer
            .expect_print()
            .with(eq("15/04/2025 || 100    || 200    "))
            .times(1)
            .returning(|_| ())
            .in_sequence(&mut seq);
        printer
            .expect_print()
            .with(eq("15/04/2025 || 100    || 100    "))
            .times(1)
            .returning(|_| ())
            .in_sequence(&mut seq);

        let transactions_repository = InMemoryTransactionRepository::new();
        let mut account = Account {
            printer,
            transactions_repository
        };
        account.deposit(100);
        account.deposit(100);
        account.print_statement();
    }
}
