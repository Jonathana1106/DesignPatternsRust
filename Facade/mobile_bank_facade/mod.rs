mod account_verifier;
mod account_mannager;
mod account_tool;
use std::io::stdin;

pub fn action_selector(num: i32){
    let mut account_name = String::new();
    let mut password = String::new();
    let mut choose = i32;

    if num == 1 {
        print!("Please enter your account name: ");
        //let _ = stdout().flush();
        stdin().read_line(&mut account_name).expect("Did not enter a correct string");
        print!("Please enter your password: ");
        stdin().read_line(&mut password).expect("Did not enter a correct string");
        account_verifier::verifier(account_name, password);
    } else if num == 2 {
        print!("Choose an action: \n 1- Check your account money \n 2- Make a transference to \
        external accout \n 3- Withdraw money from your account");

        stdin().read_line(&mut choose).expect("Did not enter a correct choose");
        if choose == 1 { account_mannager::show_money();}
        if choose == 2 { account_mannager::make_transf();}
        if choose == 3 { account_mannager::withdraw_money();}
    }else if num == 3 {
        print!("Choose an action: \n 1- Change account name \n 2- change currency from the colones \
        to dollars account \n 3- Delete account");

        stdin().read_line(&mut choose).expect("Did not enter a correct choose");
        if choose == 1 { account_tool::change_name();}
        if choose == 2 { account_tool::change_currency();}
        if choose == 3 { account_tool::delete_account();}
    }
}
