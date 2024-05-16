fn main() {
    let mut account_1 = BankAccount::new(1, String::from("Account 1"), 10.0);
    let mut account_2 = BankAccount::new(2, String::from("Account 2"), 20.0);

    account_1.deposit(25.0);
    account_2.withdraw(5.0);

    println!("Balance of {} is {} and account number is {}",account_1.holder_name,account_1.balance,account_1.account_number);
    println!("Balance of {} is {} and account number is {}",account_2.holder_name,account_2.balance,account_2.account_number);
}

trait Account {
    fn deposit(&mut self,amount:f64);
    fn withdraw(&mut self, amount:f64);
    fn balance(&self)->f64;
}

struct BankAccount{
    account_number:u32,
    holder_name:String,
    balance:f64
}

impl  BankAccount{
    fn new(account_number:u32,holder_name:String,balance:f64)->BankAccount{
        BankAccount{
            account_number,
            holder_name,
            balance
        }
    }
}

impl Account for BankAccount{
    fn deposit(&mut self,amount:f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self,amount:f64) {
        self.balance -= amount;
    }

    fn balance(&self) ->f64{
        self.balance
    }
}