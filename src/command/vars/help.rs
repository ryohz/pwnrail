use crate::output::blue;

pub fn help1(args: String, app_conf: &mut crate::config::AppConfig) -> bool {
    println!("{}ars", blue("V"));
    println!("\trefer or modify variables like an ip address.");
    println!("Commands:");
    println!("\tv\tprint this screen.");
    println!("\tvh\tprint this screen.");
    println!("\tvhh\tprint verbose help about Vars.");
    println!(
        "\tvr\trefer the variables by a json query like this: \"vr ip\", \"vr creds[0].password\".",
    );
    println!(
        "\tvm\tmodify the variables by a json query. when you want to register the ip adress, you can do it with this: \"vm ip 0.0.0.0\" for example."
    );
    false
}

pub fn help2(args: String, app_conf: &crate::config::AppConfig) -> bool {
    false
}

