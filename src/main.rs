use mockall::{Predicate, automock};

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

#[automock]
trait Calendar {
    fn today(&self) -> String;
}

struct Account<P: Printer, TR: TransactionsRepository, C: Calendar> {
    printer: P,
    transactions_repository: TR,
    calendar: C,
}

#[derive(Clone)]
enum Transaction {
    Deposit {
        date: String,
        amount: u64
    }
}

impl<P: Printer, TR: TransactionsRepository, C: Calendar> AccountService for Account<P, TR, C> {
    fn deposit(&mut self, value: u64) {
        self.transactions_repository
            .add(Transaction::Deposit {
                date: self.calendar.today(),
                amount: value
            });
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
                Transaction::Deposit {amount, date} => {
                    total += amount;
                    result.push(format!("{date} || {amount}    || {total}    "))
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
    use super::*;
    use mockall::Sequence;
    use mockall::predicate::eq;

    #[test]
    fn it_works() {
        assert!(true)
    }

    #[test]
    fn new_bank_account_statement() {
        let mut printer = MockPrinter::new();
        let calendar = MockCalendar::new();
        printer
            .expect_print()
            .with(eq("Date       || Amount || Balance"))
            .times(1)
            .returning(|_| ());

        let account = Account {
            printer,
            transactions_repository: InMemoryTransactionRepository::new(),
            calendar,
        };
        account.print_statement();
    }

    struct InMemoryTransactionRepository {
        records: Vec<Transaction>,
    }

    impl InMemoryTransactionRepository {
        fn new() -> Self {
            InMemoryTransactionRepository { records: vec![] }
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
        let mut calendar = MockCalendar::new();
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

        calendar
            .expect_today()
            .returning(|| "15/04/2025".to_string());

        let transactions_repository = InMemoryTransactionRepository::new();
        let mut account = Account {
            printer,
            transactions_repository,
            calendar,
        };
        account.deposit(100);
        account.print_statement();
    }

    #[test]
    fn bank_account_statement_with_two_deposits() {
        let mut seq = Sequence::new();
        let mut calendar = MockCalendar::new();
        let mut printer = MockPrinter::new();
        printer
            .expect_print()
            .with(eq("Date       || Amount || Balance"))
            .times(1)
            .returning(|_| ())
            .in_sequence(&mut seq);
        printer
            .expect_print()
            .with(eq("15/04/2025 || 200    || 300    "))
            .times(1)
            .returning(|_| ())
            .in_sequence(&mut seq);
        printer
            .expect_print()
            .with(eq("15/04/2025 || 100    || 100    "))
            .times(1)
            .returning(|_| ())
            .in_sequence(&mut seq);

        calendar
            .expect_today()
            .returning(|| "15/04/2025".to_string());

        let transactions_repository = InMemoryTransactionRepository::new();
        let mut account = Account {
            printer,
            transactions_repository,
            calendar,
        };
        account.deposit(100);
        account.deposit(200);
        account.print_statement();
    }

    #[test]
    fn bank_account_statement_with_two_deposits_on_different_days() {
        let mut seq = Sequence::new();
        let mut seq_calendar = Sequence::new();
        let mut printer = MockPrinter::new();
        let mut calendar = MockCalendar::new();
        printer
            .expect_print()
            .with(eq("Date       || Amount || Balance"))
            .times(1)
            .returning(|_| ())
            .in_sequence(&mut seq);
        printer
            .expect_print()
            .with(eq("16/04/2025 || 200    || 300    "))
            .times(1)
            .returning(|_| ())
            .in_sequence(&mut seq);
        printer
            .expect_print()
            .with(eq("15/04/2025 || 100    || 100    "))
            .times(1)
            .returning(|_| ())
            .in_sequence(&mut seq);

        calendar
            .expect_today()
            .returning(|| "15/04/2025".to_string())
            .times(1)
            .in_sequence(&mut seq_calendar);
        calendar
            .expect_today()
            .returning(|| "16/04/2025".to_string())
            .times(1)
            .in_sequence(&mut seq_calendar);

        let transactions_repository = InMemoryTransactionRepository::new();
        let mut account = Account {
            printer,
            transactions_repository,
            calendar,
        };
        account.deposit(100);
        account.deposit(200);
        account.print_statement();
    }
}
