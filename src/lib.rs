use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("Hello Solana Rust program entrypoint");

    // // Iterating accounts is safer than indexing
    // let accounts_iter = &mut accounts.iter();

    // // Get the account to say hello to
    // let account = next_account_info(accounts_iter)?;

    // // The account must be owned by the program in order to modify its data
    // if account.owner != program_id {
    //     msg!("Greeted account does not have the correct program id");
    //     return Err(ProgramError::IncorrectProgramId);
    // }

    // // Increment and store the number of times the account has been greeted
    // let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;
    // greeting_account.counter += 1;
    // greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    // msg!("Greeted {} time(s)!", greeting_account.counter);

    Ok(())
}


// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
