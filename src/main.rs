use colored::*;
mod cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let mut packages: Vec<&str> = vec!();
    let mut currentfunc: u8 = 0;

    for (index, argument) in args.iter().enumerate() { 
        if index == 0 { continue }
        if currentfunc > 0 { packages.push(argument); continue }

        match argument.as_str() {
            "help" => break,
            "install" => currentfunc = 1,
            "uninstall" => currentfunc = 2,
            _ => { eprintln!("{} Unknown argument \"{}\"", "[Warning]".bright_yellow().bold(), argument.bold()); break }
        }
    }

    match currentfunc {
        1 => cli::install(packages).await?,
        2 => cli::uninstall(packages).await?,
        _ => cli::help().await?
    };
    
    Ok(())
}
