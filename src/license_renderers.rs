use include_dir::{include_dir, Dir};
use serde::Deserialize;
use std::collections::BTreeMap;

static LICENSES_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/data");

#[derive(Debug, Deserialize)]
pub struct LicenseDescriptions {
    pub licenses: Vec<LicenseDescription>,
}

impl LicenseDescriptions {
    pub fn render_licences_list(&self) -> String {
        let mut licences_list = String::new();
        for license_description in &self.licenses {
            let license_abbreviation = &license_description.abbreviation;
            let license_name = &license_description.name;
            let licence_names = format!("{: <12}{}\n", license_abbreviation, license_name);
            licences_list.push_str(licence_names.as_str());
        }
        licences_list.pop();
        licences_list
    }

    pub fn fetch_license_template(&self, license_abbreviation: &str) -> &str {
        let mut license_template_relative_path = String::new();
        for license_description in &self.licenses {
            if license_abbreviation == license_description.abbreviation {
                license_template_relative_path
                    .push_str(license_description.template.clone().unwrap().get("path").unwrap());
            }
        }
        let license_template_file = LICENSES_DIR
            .get_file(license_template_relative_path)
            .unwrap();
        let license_template = license_template_file.contents_utf8().unwrap();
        license_template
    }
}

#[derive(Debug, Deserialize)]
pub struct LicenseDescription {
    pub abbreviation: String,
    name: String,
    placeholders: Option<BTreeMap<String, String>>,
    template: Option<BTreeMap<String, String>>,
}

// pub fn load_license_descriptions() -> LicenseDescriptions {
pub fn load_license_descriptions() -> LicenseDescriptions {
    let license_descriptions_file = LICENSES_DIR.get_file("licenses.yml").unwrap();
    let license_descriptions = license_descriptions_file.contents_utf8().unwrap();
    // serde_yaml::from_str::<LicenseDescriptions>(license_descriptions).unwrap()
    serde_yaml::from_str::<LicenseDescriptions>(license_descriptions).unwrap()
}

pub fn render_licence_new(
    license_abbreviation: &str,
    license_template: &str,
    license_author: &str,
    license_year: &u32,
) -> String {
    let license_descriptions = load_license_descriptions();
    for license_description in license_descriptions.licenses {
        if license_abbreviation == license_description.abbreviation {
            let license_placeholders_option = license_description.placeholders;
            if let Some(placeholders) = license_placeholders_option {
                let license_author_placeholder = placeholders.get("author").unwrap();
                let license_year_placeholder = placeholders.get("year").unwrap();
                let license = license_template.replace(license_author_placeholder, license_author);
                return license
                    .replace(license_year_placeholder, license_year.to_string().as_str());
            } else {
                break;
            }
        }
    }
    license_template.to_string()
}
