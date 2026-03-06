use fontspector_checkapi::{prelude::*, FileTypeConvert};
use fontations::skrifa::raw::TableProvider;
use std::collections::HashMap;

#[check(
    id = "typenetwork/family/equal_numbers_of_glyphs",
    rationale = "Check if all fonts in a family have the same number of glyphs.",
    proposal = "https://github.com/fonttools/fontbakery/pull/4260",
    title = "Equal number of glyphs",
    implementation = "all"
)]
fn equal_numbers_of_glyphs(t: &TestableCollection, _context: &Context) -> CheckFnResult {
    let fonts = TTF.from_collection(t);
    
    let mut roman_fonts: HashMap<String, u16> = HashMap::new();
    let mut italic_fonts: HashMap<String, u16> = HashMap::new();

    // Separate and count glyphs in roman vs italic fonts
    for (testable, font) in fonts.iter().zip(t.testables.iter()) {
        let filename = font.filename.to_string_lossy().to_string();
        let glyph_count = testable.font().maxp().map(|maxp| maxp.num_glyphs()).unwrap_or(0);
        
        let style = testable.best_subfamilyname().unwrap_or_default().to_lowercase();
        if style.contains("italic") || style.contains("oblique") {
            italic_fonts.insert(filename, glyph_count);
        } else {
            roman_fonts.insert(filename, glyph_count);
        }
    }

    let mut messages = Vec::new();

    // Check roman fonts
    if !roman_fonts.is_empty() {
        let max_roman_count = *roman_fonts.values().max().unwrap_or(&0);
        let mut mismatches = Vec::new();
        
        for (filename, count) in &roman_fonts {
            if *count != max_roman_count {
                mismatches.push(format!("  {}: {} glyphs", filename, count));
            }
        }

        if !mismatches.is_empty() {
            let mismatch_str = mismatches.join("\n");
            messages.push(Status::warn(
                "roman-different-number-of-glyphs",
                &format!("Romans don't have the same number of glyphs. Expected {max_roman_count}:\n{mismatch_str}"),
            ));
        }
    }

    // Check italic fonts
    if !italic_fonts.is_empty() {
        let max_italic_count = *italic_fonts.values().max().unwrap_or(&0);
        let mut mismatches = Vec::new();
        
        for (filename, count) in &italic_fonts {
            if *count != max_italic_count {
                mismatches.push(format!("  {}: {} glyphs", filename, count));
            }
        }

        if !mismatches.is_empty() {
            let mismatch_str = mismatches.join("\n");
            messages.push(Status::warn(
                "italic-different-number-of-glyphs",
                &format!("Italics don't have the same number of glyphs. Expected {max_italic_count}:\n{mismatch_str}"),
            ));
        }
    }

    if messages.is_empty() {
        Ok(Status::just_one_pass())
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
    fn test_equal_glyphs_pass() {
        let mut font_builder1 = FontBuilder::new();
        let mut maxp1 = Maxp::default();
        maxp1.num_glyphs = 1000;
        font_builder1.add_table(&maxp1).unwrap();
        
        let mut name1 = Name::default();
        let mut records1 = Vec::new();
        records1.push(NameRecord::new(
            3, 1, 1033, NameId::SUBFAMILY_NAME, "Regular".to_string().into(),
        ));
        records1.sort();
        name1.name_record = records1;
        font_builder1.add_table(&name1).unwrap();

        let font1 = font_builder1.build();

        let mut font_builder2 = FontBuilder::new();
        let mut maxp2 = Maxp::default();
        maxp2.num_glyphs = 1000;
        font_builder2.add_table(&maxp2).unwrap();
        
        let mut name2 = Name::default();
        let mut records2 = Vec::new();
        records2.push(NameRecord::new(
            3, 1, 1033, NameId::SUBFAMILY_NAME, "Bold".to_string().into(),
        ));
        records2.sort();
        name2.name_record = records2;
        font_builder2.add_table(&name2).unwrap();

        let font2 = font_builder2.build();

        let testable1 = Testable::new_with_contents("font1.ttf", font1);
        let testable2 = Testable::new_with_contents("font2.ttf", font2);

        let collection = TestableCollection {
            testables: vec![testable1, testable2],
            directory: "test".to_string(),
        };

        let context = Context::default();
        let result = equal_numbers_of_glyphs_impl(&collection, &context).unwrap();
        let statuses: Vec<_> = result.collect();

        assert!(statuses.iter().all(|s| s.severity == StatusCode::Pass));
    }

    #[test]
    fn test_equal_glyphs_fail() {
        let mut font_builder1 = FontBuilder::new();
        let mut maxp1 = Maxp::default();
        maxp1.num_glyphs = 1000;
        font_builder1.add_table(&maxp1).unwrap();
        
        let mut name1 = Name::default();
        let mut records1 = Vec::new();
        records1.push(NameRecord::new(
            3, 1, 1033, NameId::SUBFAMILY_NAME, "Regular".to_string().into(),
        ));
        records1.sort();
        name1.name_record = records1;
        font_builder1.add_table(&name1).unwrap();

        let font1 = font_builder1.build();

        let mut font_builder2 = FontBuilder::new();
        let mut maxp2 = Maxp::default();
        maxp2.num_glyphs = 950; // Different count
        font_builder2.add_table(&maxp2).unwrap();
        
        let mut name2 = Name::default();
        let mut records2 = Vec::new();
        records2.push(NameRecord::new(
            3, 1, 1033, NameId::SUBFAMILY_NAME, "Bold".to_string().into(),
        ));
        records2.sort();
        name2.name_record = records2;
        font_builder2.add_table(&name2).unwrap();

        let font2 = font_builder2.build();

        let testable1 = Testable::new_with_contents("font1.ttf", font1);
        let testable2 = Testable::new_with_contents("font2.ttf", font2);

        let collection = TestableCollection {
            testables: vec![testable1, testable2],
            directory: "test".to_string(),
        };

        let context = Context::default();
        let result = equal_numbers_of_glyphs_impl(&collection, &context).unwrap();
        let statuses: Vec<_> = result.collect();

        assert!(statuses.iter().any(|s| s.severity == StatusCode::Warn));
    }
}