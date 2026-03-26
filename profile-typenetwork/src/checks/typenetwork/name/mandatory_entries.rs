use fontspector_checkapi::{prelude::*, FileTypeConvert, testfont};
use fontations::skrifa::raw::types::NameId;

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
        NameId::UNIQUE_ID,
        NameId::VERSION_STRING,
        NameId::TRADEMARK,
        NameId::MANUFACTURER,
        NameId::DESIGNER,
        NameId::DESCRIPTION,
        NameId::VENDOR_URL,
        NameId::DESIGNER_URL,
        NameId::LICENSE_DESCRIPTION,
        NameId::LICENSE_URL,
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

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use fontations::write::{
        tables::maxp::Maxp,
        tables::name::{Name, NameRecord},
        types::NameId as WriteNameId,
        FontBuilder,
    };
    use fontspector_checkapi::{Context, StatusCode, Testable};

    /// Helper function to create a test font with specified name entries
    fn create_test_font_with_names(name_entries: Vec<(WriteNameId, &str)>) -> Vec<u8> {
        let mut font_builder = FontBuilder::new();
        let maxp = Maxp::default();
        font_builder.add_table(&maxp).unwrap();

        let mut name_table: Name = Name::default();
        let mut records = Vec::new();

        for (name_id, value) in name_entries {
            records.push(NameRecord::new(
                3, 1, 1033, name_id, value.to_string().into(),
            ));
        }

        records.sort();
        name_table.name_record = records;
        font_builder.add_table(&name_table).unwrap();

        font_builder.build()
    }

    #[test]
    fn test_all_required_entries_present() {
        // Create a font with ALL required AND optional name entries
        let font_data = create_test_font_with_names(vec![
            // Required
            (WriteNameId::FAMILY_NAME, "Test Family"),
            (WriteNameId::SUBFAMILY_NAME, "Regular"),
            (WriteNameId::FULL_NAME, "Test Family Regular"),
            (WriteNameId::POSTSCRIPT_NAME, "TestFamily-Regular"),
            // Optional
            (WriteNameId::COPYRIGHT_NOTICE, "© 2024"),
            (WriteNameId::UNIQUE_ID, "TestFamily-Regular-1.0"),
            (WriteNameId::VERSION_STRING, "Version 1.0"),
            (WriteNameId::TRADEMARK, "Test is a trademark"),
            (WriteNameId::MANUFACTURER, "Test Foundry"),
            (WriteNameId::DESIGNER, "Designer Name"),
            (WriteNameId::DESCRIPTION, "A test font"),
            (WriteNameId::VENDOR_URL, "https://example.com"),
            (WriteNameId::DESIGNER_URL, "https://example.com/designer"),
            (WriteNameId::LICENSE_DESCRIPTION, "Licensed under MIT"),
            (WriteNameId::LICENSE_URL, "https://example.com/license"),
        ]);

        let testable = Testable::new_with_contents("test.otf", font_data);
        let context = Context::default();
        let result = mandatory_entries_impl(&testable, &context).unwrap();

        let statuses: Vec<_> = result.collect();
        assert_eq!(statuses.len(), 1, "Should have exactly one status when all entries present");
        assert!(matches!(statuses[0].severity, StatusCode::Pass), "Should pass when all entries present");
    }

    #[test]
    fn test_missing_required_entry() {
        // Create a font missing the POSTSCRIPT_NAME entry
        let font_data = create_test_font_with_names(vec![
            (WriteNameId::FAMILY_NAME, "Test Family"),
            (WriteNameId::SUBFAMILY_NAME, "Regular"),
            (WriteNameId::FULL_NAME, "Test Family Regular"),
            // Missing POSTSCRIPT_NAME
        ]);

        let testable = Testable::new_with_contents("test.otf", font_data);
        let context = Context::default();
        let result = mandatory_entries_impl(&testable, &context).unwrap();

        let statuses: Vec<_> = result.collect();
        assert!(!statuses.is_empty(), "Should have at least one status");
        assert!(
            statuses.iter().any(|s| s.severity == StatusCode::Fail),
            "Should have a Fail status for missing required entry"
        );
    }

    #[test]
    fn test_missing_optional_entries() {
        // Create a font with all required entries but missing optional ones
        let font_data = create_test_font_with_names(vec![
            // Only required entries
            (WriteNameId::FAMILY_NAME, "Test Family"),
            (WriteNameId::SUBFAMILY_NAME, "Regular"),
            (WriteNameId::FULL_NAME, "Test Family Regular"),
            (WriteNameId::POSTSCRIPT_NAME, "TestFamily-Regular"),
        ]);

        let testable = Testable::new_with_contents("test.otf", font_data);
        let context = Context::default();
        let result = mandatory_entries_impl(&testable, &context).unwrap();

        let statuses: Vec<_> = result.collect();
        assert!(!statuses.is_empty(), "Should have warning entries for missing optional entries");
        // Should have multiple warn statuses for missing optional entries
        let warn_count = statuses
            .iter()
            .filter(|s| s.severity == StatusCode::Warn)
            .count();
        assert!(warn_count > 0, "Should have at least one warning for missing optional entries");
    }

    #[test]
    fn test_ribbi_with_typographic_names() {
        // Create a RIBBI style font with unnecessary typographic names
        let font_data = create_test_font_with_names(vec![
            (WriteNameId::FAMILY_NAME, "Test Family"),
            (WriteNameId::SUBFAMILY_NAME, "Regular"),
            (WriteNameId::FULL_NAME, "Test Family Regular"),
            (WriteNameId::POSTSCRIPT_NAME, "TestFamily-Regular"),
            // Add unnecessary typographic names for RIBBI style
            (WriteNameId::TYPOGRAPHIC_FAMILY_NAME, "Test Family Variable"),
            (WriteNameId::TYPOGRAPHIC_SUBFAMILY_NAME, "Regular Variable"),
        ]);

        let testable = Testable::new_with_contents("test.otf", font_data);
        let context = Context::default();
        let result = mandatory_entries_impl(&testable, &context).unwrap();

        let statuses: Vec<_> = result.collect();
        // Will have warns for unnecessary entries + warns for missing optional entries
        assert!(
            statuses.iter().any(|s| {
                s.severity == StatusCode::Warn
                    && s.message.as_ref().map_or(false, |msg| msg.contains("unnecessary"))
            }),
            "Should have a Warn status for unnecessary entries"
        );
    }
}