#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        // Implement this method
        BankAccount {balance: initial_balance}
    }

    pub fn deposit(&mut self, amount: f64) {
        // Implement this method
        if amount > 0.0 {
            self.balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        // Implement this method
        if amount <= self.balance && amount > 0.0 {
            self.balance -= amount;
        }
    }

    pub fn balance(&self) -> f64 {
        // Implement this method
        self.balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account() {
        // Write a test for creating a new account
        let myAct = BankAccount::new(25.0);
        assert_eq!(myAct.balance(), 25.0);
    }

    #[test]
    fn test_deposit() {
        // Write a test for depositing money
        let mut myAct = BankAccount::new(125.5);
        myAct.deposit(24.5);
        assert_eq!(myAct.balance(), 150.0);
    }

    #[test]
    fn test_withdraw() {
        // Write a test for withdrawing money
        let mut myAct = BankAccount::new(125.5);
        myAct.withdraw(25.5);
        assert_eq!(myAct.balance(), 100.0);
    }

    // Add more tests here

    //edge case: withdrawing more than the balance
    #[test]
    fn test_overdraft(){
        let mut myAct = BankAccount::new(20.0);
        myAct.withdraw(400.0);
        assert_eq!(myAct.balance(), 20.0);
    }

    // edge case: depositing negative
    #[test]
    fn test_negative_deposit(){
        let mut myAct = BankAccount::new(40.0);
        myAct.deposit(-12.0);
        assert_eq!(myAct.balance(), 40.0);
    }

    //edge case: withdrawing negative.
    #[test]
    fn test_negative_withdraw(){
        let mut myAct = BankAccount::new(100.0);
        myAct.withdraw(-400.0);
        assert_eq!(myAct.balance(), 100.0);
    }

}