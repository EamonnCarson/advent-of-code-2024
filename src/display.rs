use colored::Colorize;
use std::fmt::Display;

pub mod panic_handler;
pub mod scroll_writer;
pub mod logger;

pub fn banner() -> impl Display {
    let mut banner = "-".green().to_string();
    for _ in 0..12 {
        banner.push_str("-".red().to_string().as_str());
        banner.push_str("-".green().to_string().as_str());
    }

    banner
}

///               (#((##%%%%%((#            
///           #((#%%%%#(((#%%%#(/((        
///        %(#%%&&&&%#&&&%%%%%%%#####      
/// ****** %&     &&&&&&%%%%#((((((((#,    
/// ********     (&%%%%%&&&&%%%%%##(((#(   
///  *****       &%%%%%%%%%%%%%%%%######*  
///           ****************************
///          ******************************
///          ******************************
pub fn santa_hat() -> impl Display {
    let mut hat = "           (#((##%%%%%((#
        #((#%%%%#(((#%%%#(/((
     %(#%%&&&&%#&&&%%%%%%%#####"
        .red()
        .to_string();
    hat.push('\n');
    hat.push_str(" ******".bright_white().to_string().as_str());
    hat.push_str(" %&     &&&&&&%%%%#((((((((#,".red().to_string().as_str());
    hat.push('\n');
    hat.push_str(" ********".bright_white().to_string().as_str());
    hat.push_str("     (&%%%%%&&&&%%%%%##(((#(".red().to_string().as_str());
    hat.push('\n');
    hat.push_str("  *****".bright_white().to_string().as_str());
    hat.push_str("       &%%%%%%%%%%%%%%%%######*".red().to_string().as_str());
    hat.push('\n');
    hat.push_str(
        "           **************************** 
          ******************************
          ******************************"
            .bright_white()
            .to_string()
            .as_str(),
    );
    hat
}