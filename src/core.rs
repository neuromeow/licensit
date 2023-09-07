use std::error::Error;
use std::fs;

pub fn print_licences_templates_list() {
    let licenses_templates = fs::read_dir("licenses/templates").unwrap();
    for license_template in licenses_templates {
        println!(
            "{}",
            license_template.unwrap().file_name().to_str().unwrap()
        )
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    print_licences_templates_list();
    Ok(())
}
