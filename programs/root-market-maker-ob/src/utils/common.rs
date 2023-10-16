use anchor_lang::{prelude::Error, require};

use crate::errors::RootError;

pub fn match_deposit_ratios(
    current_vault_base_token: u64,
    current_vault_quote_token: u64,
    max_user_deposit_base_token: u64,
    max_user_deposit_quote_token: u64,
) -> Result<(u64, u64), Error> {
    // Check if the vault has any deposits yet
    if current_vault_base_token == 0 || current_vault_quote_token == 0 {
        return Ok((max_user_deposit_base_token, max_user_deposit_quote_token));
    }

    // Calculate the proportion set by the first user using the greatest common divisor (gcd)
    let gcd = gcd(current_vault_base_token, current_vault_quote_token);
    let proportion_a = current_vault_base_token / gcd;
    let proportion_b = current_vault_quote_token / gcd;

    // Calculate the maximum amount of token A based on the proportion
    let max_token_a = (max_user_deposit_quote_token / proportion_b) * proportion_a;
    let max_token_a = max_token_a.min(max_user_deposit_base_token);

    // Calculate the maximum amount of token B based on the proportion
    let max_token_b = (max_user_deposit_base_token / proportion_a) * proportion_b;
    let max_token_b = max_token_b.min(max_user_deposit_quote_token);

    // Assert that the ratio of computed values is equal to the ratio of the vault deposits
    require!(
        (max_token_a * current_vault_quote_token) == (max_token_b * current_vault_base_token),
        RootError::DepositRatioCheckFail
    );

    Ok((max_token_a, max_token_b))
}

// Function to calculate the greatest common divisor (gcd) using Euclid's algorithm
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

mod tests {
    #[allow(unused_imports)]
    use super::match_deposit_ratios;

    #[test]
    pub fn match0() {
        let vault_base = 0;
        let vault_quote = 0;

        let max_user_base = 3232;
        let max_user_quote = 121;

        let (adjusted_base, adjusted_quote) =
            match_deposit_ratios(vault_base, vault_quote, max_user_base, max_user_quote).unwrap();

        assert_eq!(adjusted_base, 3232);
        assert_eq!(adjusted_quote, 121);
    }

    #[test]
    pub fn match1() {
        let vault_base = 50;
        let vault_quote = 100;

        let max_user_base = 50;
        let max_user_quote = 100;

        let (adjusted_base, adjusted_quote) =
            match_deposit_ratios(vault_base, vault_quote, max_user_base, max_user_quote).unwrap();

        assert_eq!(adjusted_base, 50);
        assert_eq!(adjusted_quote, 100);
    }

    #[test]
    pub fn match2() {
        let vault_base = 50;
        let vault_quote = 100;

        let max_user_base = 51;
        let max_user_quote = 100;

        let (adjusted_base, adjusted_quote) =
            match_deposit_ratios(vault_base, vault_quote, max_user_base, max_user_quote).unwrap();

        assert_eq!(adjusted_base, 50);
        assert_eq!(adjusted_quote, 100);
    }

    #[test]
    pub fn match3() {
        let vault_base = 50;
        let vault_quote = 100;

        let max_user_base = 51;
        let max_user_quote = 102;

        let (adjusted_base, adjusted_quote) =
            match_deposit_ratios(vault_base, vault_quote, max_user_base, max_user_quote).unwrap();

        assert_eq!(adjusted_base, 51);
        assert_eq!(adjusted_quote, 102);
    }

    #[test]
    pub fn match4() {
        let vault_base = 500_000_000;
        let vault_quote = 100_000_000;

        let max_user_base = 490_100_000;
        let max_user_quote = 98_000_000;

        let (adjusted_base, adjusted_quote) =
            match_deposit_ratios(vault_base, vault_quote, max_user_base, max_user_quote).unwrap();

        assert_eq!(adjusted_base, 490_000_000);
        assert_eq!(adjusted_quote, 98_000_000);
    }
}
