use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub email: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountStore {
    pub accounts: Vec<Account>,
    pub active_account_email: Option<String>,
}

pub fn add_account(store: &mut AccountStore, account: Account) {
    let existing = store
        .accounts
        .iter()
        .position(|a| a.email == account.email);

    match existing {
        Some(index) => {
            store.accounts[index] = account;
        }
        None => {
            let is_first = store.accounts.is_empty();
            store.accounts.push(account.clone());
            if is_first {
                store.active_account_email = Some(account.email);
            }
        }
    }
}

pub fn list_accounts(store: &AccountStore) -> &[Account] {
    &store.accounts
}

pub fn switch_account(store: &mut AccountStore, email: &str) -> Result<(), String> {
    let exists = store.accounts.iter().any(|a| a.email == email);
    if !exists {
        return Err(format!("Account not found: {}", email));
    }
    store.active_account_email = Some(email.to_string());
    Ok(())
}

pub fn remove_account(store: &mut AccountStore, email: &str) -> Result<(), String> {
    let index = store
        .accounts
        .iter()
        .position(|a| a.email == email)
        .ok_or_else(|| format!("Account not found: {}", email))?;

    store.accounts.remove(index);

    if store.active_account_email.as_deref() == Some(email) {
        store.active_account_email = None;
    }

    Ok(())
}

pub fn get_active_account(store: &AccountStore) -> Option<&Account> {
    let active_email = store.active_account_email.as_deref()?;
    store.accounts.iter().find(|a| a.email == active_email)
}
