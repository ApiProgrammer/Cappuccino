use colored::*;

pub async fn help() -> Result<(), Box<dyn std::error::Error>> {
    println!("{} help menu\n", "Cappuccino".truecolor(177, 144, 127));
    Ok(())
}

pub async fn install(packages: Vec<&str>)   -> Result<(), Box<dyn std::error::Error>> {
    print!("{}", "Installing: ".truecolor(177, 144, 127));
    for (index, package) in packages.iter().enumerate() {
        print!("{}{}", package, if index < packages.len() - 1 { ", " } else { "" })
    }
    println!();

    Ok(())
}

pub async fn uninstall(packages: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    print!("{}", "Uninstalling: ".truecolor(177, 144, 127));
    for (index, package) in packages.iter().enumerate() {
        print!("{}{}", package, if index < packages.len() - 1 { ", " } else { "" })
    }
    println!();
    Ok(())
}