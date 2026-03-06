use fontspector_checkapi::{prelude::*, FileTypeConvert};
use fontations::skrifa::raw::types::NameId;
use std::collections::HashSet;

#[check(
    id = "typenetwork/family/duplicated_names",
    rationale = "Having duplicated name records can produce several issues like not all fonts being listed on design apps or incorrect automatic creation of CSS classes and @font-face rules.",
    proposal = "https://github.com/fonttools/fontbakery/pull/4260",
    title = "Check if font doesn't have duplicated names within a family.",
    implementation = "all"
)]
fn duplicated_names(t: &TestableCollection, _context: &Context) -> CheckFnResult {
    let fonts = TTF.from_collection(t);
    let mut seen_full_names = HashSet::new();
    let mut duplicate_full_names = HashSet::new();
    let mut seen_postscript_names = HashSet::new();
    let mut duplicate_postscript_names = HashSet::new();

    for font in fonts {
        // FullName (4)
        let full_name = font.get_name_entry_strings(NameId::FULL_NAME).next().unwrap_or_default();
        if !seen_full_names.insert(full_name.clone()) {
            duplicate_full_names.insert(full_name);
        }


        // PostScript name (6)
        let ps_name = font.get_name_entry_strings(NameId::POSTSCRIPT_NAME).next().unwrap_or_default();
        if !seen_postscript_names.insert(ps_name.clone()) {
            duplicate_postscript_names.insert(ps_name);
        }
    }

    let mut messages = Vec::new();

    if !duplicate_full_names.is_empty() {
        let names_str = duplicate_full_names.iter().map(|n| format!("* {}\n", n)).collect::<String>();
        messages.push(Status::fail(
            "duplicate-full-names",
            &format!("Following full names are duplicate:\n\n{}", names_str),
        ));
    }

    if !duplicate_postscript_names.is_empty() {
        let names_str = duplicate_postscript_names.iter().map(|n| format!("* {}\n", n)).collect::<String>();
        messages.push(Status::fail(
            "duplicate-postscript-names",
            &format!("Following postscript names are duplicate:\n\n{}", names_str),
        ));
    }

    if messages.is_empty() {
        return Ok(Status::just_one_pass());
    } else {
        return_result(messages)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use fontations::write::{
        tables::maxp::Maxp,
        tables::name::{Name, NameRecord},
        types::NameId,
        FontBuilder,
    };
    use fontspector_checkapi::{Context, StatusCode, Testable, TestableCollection};

    #[test]
    fn test_duplicated_names() {
        // Create two fonts with the same full name and postscript name
        let mut font_builder1 = FontBuilder::new();
        let maxp = Maxp::default();
        font_builder1.add_table(&maxp).unwrap();

        let mut name1: Name = Name::default();
        let mut records1 = Vec::new();
        records1.push(NameRecord::new(
            3, 1, 1033, NameId::FULL_NAME, "Test Font".to_string().into(),
        ));
        records1.push(NameRecord::new(
            3, 1, 1033, NameId::POSTSCRIPT_NAME, "TestFont".to_string().into(),
        ));
        records1.sort();
        name1.name_record = records1;
        font_builder1.add_table(&name1).unwrap();

        let font1 = font_builder1.build();

        let mut font_builder2 = FontBuilder::new();
        font_builder2.add_table(&maxp).unwrap();

        let mut name2: Name = Name::default();
        let mut records2 = Vec::new();
        records2.push(NameRecord::new(
            3, 1, 1033, NameId::FULL_NAME, "Test Font".to_string().into(),
        ));
        records2.push(NameRecord::new(
            3, 1, 1033, NameId::POSTSCRIPT_NAME, "TestFont".to_string().into(),
        ));
        records2.sort();
        name2.name_record = records2;
        font_builder2.add_table(&name2).unwrap();

        let font2 = font_builder2.build();

        let testable1 = Testable::new_with_contents("font1.otf", font1);
        let testable2 = Testable::new_with_contents("font2.otf", font2);

        let collection = TestableCollection {
            testables: vec![testable1, testable2],
            directory: "test_dir".to_string(),
        };

        let context = Context::default();
        let result = duplicated_names_impl(&collection, &context).unwrap();

        // Should have failures for duplicate names
        let statuses: Vec<_> = result.collect();
        assert!(!statuses.is_empty());
        // Check that it contains fail statuses
        assert!(statuses.iter().any(|s| s.severity == StatusCode::Fail));
    }

    #[test]
    fn test_no_duplicated_names() {
        // Create two fonts with different names
        let mut font_builder1 = FontBuilder::new();
        let maxp = Maxp::default();
        font_builder1.add_table(&maxp).unwrap();

        let mut name1: Name = Name::default();
        let mut records1 = Vec::new();
        records1.push(NameRecord::new(
            3, 1, 1033, NameId::FULL_NAME, "Test Font 1".to_string().into(),
        ));
        records1.push(NameRecord::new(
            3, 1, 1033, NameId::POSTSCRIPT_NAME, "TestFont1".to_string().into(),
        ));
        records1.sort();
        name1.name_record = records1;
        font_builder1.add_table(&name1).unwrap();

        let font1 = font_builder1.build();

        let mut font_builder2 = FontBuilder::new();
        font_builder2.add_table(&maxp).unwrap();

        let mut name2: Name = Name::default();
        let mut records2 = Vec::new();
        records2.push(NameRecord::new(
            3, 1, 1033, NameId::FULL_NAME, "Test Font 2".to_string().into(),
        ));
        records2.push(NameRecord::new(
            3, 1, 1033, NameId::POSTSCRIPT_NAME, "TestFont2".to_string().into(),
        ));
        records2.sort();
        name2.name_record = records2;
        font_builder2.add_table(&name2).unwrap();

        let font2 = font_builder2.build();

        let testable1 = Testable::new_with_contents("font1.otf", font1);
        let testable2 = Testable::new_with_contents("font2.otf", font2);

        let collection = TestableCollection {
            testables: vec![testable1, testable2],
            directory: "test_dir".to_string(),
        };

        let context = Context::default();
        let result = duplicated_names_impl(&collection, &context).unwrap();

        // Should pass
        let statuses: Vec<_> = result.collect();
        assert_eq!(statuses.len(), 1);
        assert!(matches!(statuses[0].severity, StatusCode::Pass));
    }
}