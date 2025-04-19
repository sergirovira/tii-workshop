struct User {
    name: String,
    credit_line: u64,
    balance: i64,
}

impl User {
    fn update_balance(&mut self, amount: i64) {
        self.balance += amount;
    }
}

struct Bank {
    users: Vec<User>,
    name: String,
    credit_interest: u64,
    debit_interest: u64,
}

impl Bank {
    fn new(name: String, credit_interest: u64, debit_interest: u64) -> Self {
        Bank {
            users: Vec::new(),
            name,
            credit_interest,
            debit_interest,
        }
    }

    fn add_user(&mut self, name: String, credit_line: u64) {
        let user = User {
            name,
            credit_line,
            balance: 0,
        };
        self.users.push(user);
    }

    fn get_user(&self, name: &str) -> Option<&User> {
        self.users.iter().find(|user| user.name == name)
    }

    fn calc_balance(&self) -> (i64, u64) {
        let total_liabilities: i64 = self
            .users
            .iter()
            .filter(|user| user.balance < 0)
            .map(|user| user.balance)
            .sum();

        let total_assets: u64 = self
            .users
            .iter()
            .filter(|user| user.credit_line > 0)
            .map(|user| user.credit_line)
            .sum();

        (total_liabilities, total_assets)
    }

    fn transfer_funds(
        &mut self,
        from_user: &str,
        to_user: &str,
        amount: u64,
    ) -> Result<(), String> {
        let from = self.get_user(from_user);
        let to = self.get_user(to_user);

        if let (Some(from_user), Some(to_user)) = (from, to) {
            if from_user.balance + i64::from(amount) < 0 {
                return Err(format!(
                    "Transfer failed: {} does not have enough balance.",
                    from_user.name
                ));
            }

            // Update balances
            from_user.balance -= i64::from(amount);
            to_user.balance += i64::from(amount);
            Ok(())
        } else {
            Err("Transfer failed: One or both users not found.".to_string())
        }
    }

    fn accrue_interest(&mut self) {
        for user in &mut self.users {
            if user.balance < 0 {
                // Debit interest
                user.balance -= i64::from(user.balance.abs() as u64 * self.debit_interest / 100);
            } else if user.credit_line > 0 {
                // Credit interest
                user.balance += i64::from(user.credit_line * self.credit_interest / 100);
            }
        }
    }

    fn merge_bank(&mut self, other: Bank) {
        for user in other.users {
            match self.get_user(&user.name) {
                Some(existing_user) => {
                    // Update existing user's balance
                    existing_user.update_balance(existing_user.balance + user.balance);
                }
                None => {
                    // Add new user
                    self.users.push(user);
                }
            }
        }
    }
}
