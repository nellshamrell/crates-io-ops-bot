fn authorized_users() -> Vec<String> {
    dotenv::var("AUTHORIZED_USERS")
        .unwrap()
        .split(',')
        .map(String::from)
        .collect()
}

pub fn is_authorized(id: String) -> bool {
    authorized_users().contains(&id)
}

#[cfg(test)]
mod tests {
    use super::*;

    // The Authorized users environmental variable
    // is set for local tests in the .env file

    #[test]
    fn list_authorized_users() {
        let result = authorized_users();
        assert!(
            result.contains(&String::from("123")),
            "Result does not contain the expected name. Result was {:?}",
            result
        );

        assert!(
            result.contains(&String::from("456")),
            "Result does not contain the expected name. Result was {:?}",
            result
        );
    }

    #[test]
    fn check_whether_user_is_authorized() {
        assert!(is_authorized(String::from("123")));
        assert!(is_authorized(String::from("456")));
        assert!(!is_authorized(String::from("789")));
    }
}
