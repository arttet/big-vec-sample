use {
    arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs},
    borsh::{BorshDeserialize, BorshSchema, BorshSerialize},
    solana_program::{program_error::ProgramError, program_pack::Pack, program_pack::Sealed},
    spl_stake_pool::big_vec::BigVec,
};

/// Storage list for all validator stake accounts in the pool.
pub struct ValidatorList<'data> {
    /// List of stake info for each validator in the pool
    pub validators: BigVec<'data>,
}

#[repr(C)]
#[derive(Clone, Debug, Default, Eq, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct Validator {
    /// Sum of the balances of the stake accounts and unstake accounts.
    pub stake_accounts_balance: u64,

    /// Sum of the balances of the unstake accounts.
    pub unstake_accounts_balance: u64,

    /// Controls if a validator is allowed to have new stake deposits.
    /// When removing a validator, this flag should be set to `false`.
    pub active: bool,
}

impl Sealed for Validator {}

impl Pack for Validator {
    const LEN: usize = 17;

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, 17];
        let (stake_accounts_balance, unstake_accounts_balance, active) = array_refs![src, 8, 8, 1];

        let stake_accounts_balance = u64::from_le_bytes(*stake_accounts_balance);
        let unstake_accounts_balance = u64::from_le_bytes(*unstake_accounts_balance);
        let active = match active {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(Validator {
            stake_accounts_balance,
            unstake_accounts_balance,
            active,
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, 17];
        let (stake_accounts_balance_dst, unstake_accounts_balance_dst, active_dst) =
            mut_array_refs![dst, 8, 8, 1];

        let &Validator {
            stake_accounts_balance,
            unstake_accounts_balance,
            active,
        } = self;

        *stake_accounts_balance_dst = stake_accounts_balance.to_le_bytes();
        *unstake_accounts_balance_dst = unstake_accounts_balance.to_le_bytes();
        active_dst[0] = active as u8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut data = [0; 1024 * 10];
        let mut validator_list = ValidatorList {
            validators: BigVec { data: &mut data },
        };

        let validator = Validator {
            stake_accounts_balance: 1024,
            unstake_accounts_balance: 4201,
            active: true,
        };

        let _ = validator_list.validators.push(validator);

        for val in validator_list.validators.iter::<Validator>() {
            println!("Validator: {:?}", val);
        }
    }
}
