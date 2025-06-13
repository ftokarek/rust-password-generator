pub fn print_passwords(passwords: &[String]) 
{
    for (i, password) in passwords.iter().enumerate() 
    {
        println!("Password {}: {}", i + 1, password);
    }
}