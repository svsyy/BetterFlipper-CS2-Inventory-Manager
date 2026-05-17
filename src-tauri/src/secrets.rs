

use crate::error::AppResult;

const SERVICE: &str = "cs2-inventory-manager";
const ACCOUNT_LIST_KEY: &str = "__saved_accounts__";

fn entry(username: &str) -> AppResult<keyring::Entry> {
    keyring::Entry::new(SERVICE, username).map_err(Into::into)
}

pub fn save_refresh_token(username: &str, token: &str) -> AppResult<()> {
    entry(username)?.set_password(token)?;
    add_to_saved_list(username)?;
    Ok(())
}

pub fn load_refresh_token(username: &str) -> AppResult<Option<String>> {
    match entry(username)?.get_password() {
        Ok(s) => Ok(Some(s)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

pub fn forget(username: &str) -> AppResult<()> {
    match entry(username)?.delete_credential() {
        Ok(()) | Err(keyring::Error::NoEntry) => {}
        Err(e) => return Err(e.into()),
    }
    remove_from_saved_list(username)?;
    Ok(())
}

pub fn list_saved_accounts() -> AppResult<Vec<String>> {
    let raw = match keyring::Entry::new(SERVICE, ACCOUNT_LIST_KEY)?.get_password() {
        Ok(s) => s,
        Err(keyring::Error::NoEntry) => return Ok(Vec::new()),
        Err(e) => return Err(e.into()),
    };
    Ok(raw.split('\n').filter(|s| !s.is_empty()).map(str::to_owned).collect())
}

fn write_saved_list(list: &[String]) -> AppResult<()> {
    let joined = list.join("\n");
    keyring::Entry::new(SERVICE, ACCOUNT_LIST_KEY)?.set_password(&joined)?;
    Ok(())
}

fn add_to_saved_list(username: &str) -> AppResult<()> {
    let mut list = list_saved_accounts()?;
    if !list.iter().any(|u| u == username) {
        list.push(username.to_owned());
        write_saved_list(&list)?;
    }
    Ok(())
}

fn remove_from_saved_list(username: &str) -> AppResult<()> {
    let list: Vec<String> = list_saved_accounts()?
        .into_iter()
        .filter(|u| u != username)
        .collect();
    write_saved_list(&list)?;
    Ok(())
}
