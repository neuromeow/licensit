use include_dir::{include_dir, Dir};
use serde::Deserialize;
use std::collections::BTreeMap;

static LICENSES_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/data");
const LICENSES_DESCRIPTIONS_FILE_BASENAME: &str = "licenses.yml";

#[derive(Debug, Deserialize)]
pub struct LicenseDescription {
    pub abbreviation: String,
    name: String,
    placeholders: Option<BTreeMap<String, String>>,
    template: Option<BTreeMap<String, String>>,
}

impl LicenseDescription {
    fn get_license_template_path(&self) -> String {
        self.template.clone().unwrap().get("path").unwrap().to_string()
    }

    pub fn fetch_license_template(&self) -> &str {
        let license_template_relative_path = &self.get_license_template_path();
        let license_template_file = LICENSES_DIR
            .get_file(license_template_relative_path)
            .unwrap();
        let license_template = license_template_file.contents_utf8().unwrap();
        license_template
    }

    pub fn render_licence(&self, license_author: &str, license_year: &u32,) -> String {
        let license_template = self.fetch_license_template();
        let license_placeholders_option = self.placeholders.clone();
            if let Some(placeholders) = license_placeholders_option {
                let license_author_placeholder = placeholders.get("author").unwrap();
                let license_year_placeholder = placeholders.get("year").unwrap();
                    let license = license_template.replace(license_author_placeholder, license_author);
                    return license.replace(license_year_placeholder, license_year.to_string().as_str());
        }
        license_template.to_string()
    }
}

#[derive(Debug, Deserialize)]
pub struct LicenseDescriptions {
    pub licenses: Vec<LicenseDescription>,
}

impl LicenseDescriptions {
    pub fn from_licenses_descriptions_file() -> Self {
        let licenses_descriptions_file = LICENSES_DIR.get_file(LICENSES_DESCRIPTIONS_FILE_BASENAME).unwrap();
        let licenses_descriptions_file_content = licenses_descriptions_file.contents_utf8().unwrap();
        serde_yaml::from_str::<LicenseDescriptions>(licenses_descriptions_file_content).unwrap()
    }

    pub fn get_license_description(&self, license_abbreviation: &str) -> Result<&LicenseDescription, String> {
        for license_description in &self.licenses {
            if license_abbreviation == license_description.abbreviation {
                return Ok(license_description);
            }
        }
        Err(format!(
            "specified license not in list {:?}",
            license_abbreviation
        ))
    }

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
        let license_template = "";
        for license_description in &self.licenses {
            if license_abbreviation == license_description.abbreviation {
                return license_description.fetch_license_template();
            }
        }
        license_template
    }

    pub fn render_licence(
        &self,
        license_abbreviation: &str,
        license_author: &str,
        license_year: &u32,
    ) -> String {
        for license_description in &self.licenses {
            if license_abbreviation == license_description.abbreviation {
                return license_description.render_licence(license_author, license_year);
            }
        }
        String::new()
    }
}
