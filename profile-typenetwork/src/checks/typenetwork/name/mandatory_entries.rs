use fontspector_checkapi::{prelude::*, testfont};

const RIBBI_STYLE_NAMES: &[&str] = &["Regular", "Italic", "Bold", "Bold Italic"];

#[check(
    id = "typenetwork/name/mandatory_entries",
    rationale = "
        For proper functioning, fonts must have some specific records.
        Other name records are optional but desirable to be present.
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/4260",
    title = "Font has all mandatory 'name' table entries?"
)]
fn mandatory_entries(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let style = f.best_subfamilyname().unwrap_or("Regular".to_string());

    let mut required_name_ids = vec![
        NameId::FAMILY_NAME,
        NameId::SUBFAMILY_NAME,
        NameId::FULL_NAME,
        NameId::POSTSCRIPT_NAME,
    ];

    let mut unnecessary_name_ids = vec![];

    if !RIBBI_STYLE_NAMES.contains(&style.as_str()) {
        required_name_ids.push(NameId::TYPOGRAPHIC_FAMILY_NAME);
        required_name_ids.push(NameId::TYPOGRAPHIC_SUBFAMILY_NAME);
    } else {
        unnecessary_name_ids.push(NameId::TYPOGRAPHIC_FAMILY_NAME);
        unnecessary_name_ids.push(NameId::TYPOGRAPHIC_SUBFAMILY_NAME);
    }

    let optional_name_ids = vec![
        NameId::COPYRIGHT_NOTICE,
        NameId::UNIQUE_FONT_IDENTIFIER,
        NameId::VERSION_STRING,
        NameId::TRADEMARK,
        NameId::MANUFACTURER_NAME,
        NameId::DESIGNER,
        NameId::DESCRIPTION,
        NameId::VENDOR_URL,
        NameId::DESIGNER_URL,
        NameId::LICENSE_DESCRIPTION,
        NameId::LICENSE_INFO_URL,
    ];

    let mut passed = true;
    let mut results = vec![];

    // Check required
    for &name_id in &required_name_ids {
        let entries: Vec<_> = f.get_name_entry_strings(name_id).collect();
        if entries.is_empty() || entries.iter().any(|e| e.is_empty()) {
            passed = false;
            results.push(Status::fail(
                "missing-required-entry",
                &format!("Font lacks entry with nameId={:?}", name_id),
            ));
        }
    }

    // Check optional
    for &name_id in &optional_name_ids {
        let entries: Vec<_> = f.get_name_entry_strings(name_id).collect();
        if entries.is_empty() {
            passed = false;
            results.push(Status::warn(
                "missing-optional-entry",
                &format!("Font lacks entry with nameId={:?}", name_id),
            ));
        }
    }

    // Check unnecessary
    for &name_id in &unnecessary_name_ids {
        let entries: Vec<_> = f.get_name_entry_strings(name_id).collect();
        if !entries.is_empty() {
            passed = false;
            results.push(Status::warn(
                "unnecessary-entry",
                &format!("Font has unnecessary name entry with nameId={:?}", name_id),
            ));
        }
    }

    if passed {
        Ok(Status::just_one_pass())
    } else {
        Ok(Box::new(results.into_iter()))
    }
}