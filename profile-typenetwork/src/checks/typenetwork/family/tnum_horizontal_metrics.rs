use fontspector_checkapi::{prelude::*, skip, FileTypeConvert};
use fontations::skrifa::raw::TableProvider;
use std::collections::{HashMap, HashSet};

#[check(
    id = "typenetwork/family/tnum_horizontal_metrics",
    rationale = "All tabular figures must have the same width across the family.",
    proposal = "https://github.com/fonttools/fontbakery/pull/4260",
    title = "Tabular figures have consistent horizontal metrics across the family?",
    implementation = "all",
)]
fn tnum_horizontal_metrics(t: &TestableCollection, context: &Context) -> CheckFnResult {
    println!("Running tnum_horizontal_metrics on collection with {} testables", t.testables.len());
    
    let fonts = TTF.from_collection(t);
    println!("Loaded {} fonts from collection", fonts.len());
    skip!(
        fonts.len() < 2,
        "no-siblings",
        "This check requires at least two sibling fonts to compare metrics."
    );

    // glyph names that identify tabular figures in the original python check
    const SUFFIXES: &[&str] = &[
        ".tnum",
        ".tf",
        ".tosf",
        ".tsc",
        ".tab",
        ".tabular",
    ];

    let mut tnum_widths: HashMap<u16, HashSet<String>> = HashMap::new();

    for font in fonts {
        let hmtx = font.font().hmtx()?;

        font.all_glyphs()
            .filter_map(|gid| {
                font.glyph_name_for_id(gid)
                    .map(|name| (gid, name))
            })
            .filter(|(_, name)| SUFFIXES.iter().any(|s| name.ends_with(s)))
            .for_each(|(gid, name)| {
                println!("Checking glyph '{}' (GID {}) for tabular suffixes", name, gid);
                if let Some(width) = hmtx.advance(gid) {
                    tnum_widths
                        .entry(width)
                        .or_insert_with(HashSet::new)
                        .insert(name.to_string());
                }
            });
    }

    println!("Found tabular figure widths: {:?}", tnum_widths.keys());

    let mut problems = Vec::new();
    if tnum_widths.len() > 1 {
        // determine most common width
        #[allow(clippy::unwrap_used)]
        let (&most_common_width, _) = tnum_widths
            .iter()
            .max_by_key(|(_, glyphs)| glyphs.len())
            .unwrap();

        // search for half‑width entries
        let mut half_width: Option<u16> = None;
        let mut half_width_glyphs: Vec<String> = Vec::new();
        for (&width, glyphs) in &tnum_widths {
            if (most_common_width as f32 / 2.0).round() as u16 == width {
                half_width = Some(width);
                half_width_glyphs = glyphs.iter().cloned().collect();
            }
        }

        // warn about anything that's neither the most common width nor the half width
        let mut inconsistent = Vec::new();
        for (&width, glyphs) in &tnum_widths {
            if width != most_common_width && Some(width) != half_width {
                inconsistent.push((width, glyphs));
            }
        }

        if let Some(hw) = half_width {
            problems.push(Status::info(
                "half-widths",
                &format!(
                    "There are other glyphs with half of the width ({}) of the most common width such as the following ones:\n\n{}",
                    hw,
                    bullet_list(context, half_width_glyphs.iter())
                ),
            ));
        }

        if !inconsistent.is_empty() {
            let detail = inconsistent
                .iter()
                .map(|(width, glyphs)| {
                    format!(
                        "Width: {} - Glyphs: {}",
                        width,
                        glyphs
                            .iter()
                            .map(|s| format!("'{s}'"))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                })
                .collect::<Vec<_>>()
                .join("\n");

            problems.push(Status::warn(
                "inconsistent-widths",
                &format!(
                    "The most common tabular glyph width is {}. But there are other tabular glyphs with different widths such as the following ones:\n\n{}",
                    most_common_width,
                    detail
                ),
            ));
        }
    }

    if problems.is_empty() {
        Ok(Status::just_one_pass())
    } else {
        return_result(problems)
    }
}

// tests
#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use fontations::write::{
        tables::hhea::Hhea,
        tables::hmtx::{Hmtx, LongMetric},
        tables::maxp::Maxp,
        tables::name::{Name, NameRecord},
        tables::post::Post,
        types::NameId,
        FontBuilder,
    };
    use fontspector_checkapi::{Context, StatusCode, Testable, TestableCollection};

    fn make_font_with_tnum(name: &str, glyphs: &[(&str, u16)]) -> Testable {
        let mut fb = FontBuilder::new();
        // add a name table with simple family/subfamily so collection utilities don't panic
        let mut name_table = Name::default();
        let record = NameRecord::new(3, 1, 1033, NameId::SUBFAMILY_NAME, name.to_string().into());
        name_table.name_record = vec![record];
        name_table.name_record.sort();
        fb.add_table(&name_table).unwrap();

        // add maxp with num_glyphs
        let mut maxp = Maxp::default();
        maxp.num_glyphs = glyphs.len() as u16;
        fb.add_table(&maxp).unwrap();

        // ensure hhea & hmtx so we can set advance widths
        fb.add_table(&Hhea::default()).unwrap();
        let mut hmtx = Hmtx::default();
        for &(_, adv) in glyphs {
            hmtx.h_metrics.push(LongMetric { advance: adv, side_bearing: 0 });
        }
        fb.add_table(&hmtx).unwrap();

        // add post for glyph names
        fb.add_table(&Post::default()).unwrap();

        Testable::new_with_contents(format!("{name}.ttf"), fb.build())
    }

    // #[test]
    // fn pass_when_all_same_width() {
    //     let f1 = make_font_with_tnum("Regular", &[("one.tnum", 500), ("two.tnum", 500)]);
    //     let f2 = make_font_with_tnum("Bold", &[("one.tnum", 500), ("two.tnum", 500)]);
    //     let collection = TestableCollection {
    //         testables: vec![f1, f2],
    //         directory: "test".to_string(),
    //     };

    //     let result = tnum_horizontal_metrics_impl(&collection, &Context::default()).unwrap();
    //     let statuses = result.collect::<Vec<_>>();
    //     assert!(statuses.iter().all(|s| s.severity == StatusCode::Pass));
    // }

    #[test]
    fn warn_for_different_widths() {
        let f1 = make_font_with_tnum("Regular", &[("one.tnum", 500), ("two.tnum", 500)]);
        let f2 = make_font_with_tnum("Bold", &[("one.tnum", 600), ("two.tnum", 600)]);
        let collection = TestableCollection {
            testables: vec![f1, f2],
            directory: "test".to_string(),
        };
        let result = tnum_horizontal_metrics_impl(&collection, &Context::default()).unwrap();
        let statuses: Vec<_> = result.collect();
        println!("{:#?}", statuses);
        assert!(statuses.iter().any(|s| s.severity == StatusCode::Warn));
    }

    #[test]
    fn info_for_half_width() {
        let f1 = make_font_with_tnum("Regular", &[("one.tnum", 600)]);
        // second font adds a half-width glyph
        let f2 = make_font_with_tnum("Bold", &[("one.tnum", 600), ("half.tnum", 300)]);
        let collection = TestableCollection {
            testables: vec![f1, f2],
            directory: "test".to_string(),
        };
        let result = tnum_horizontal_metrics_impl(&collection, &Context::default()).unwrap();
        let statuses: Vec<_> = result.collect();
        
        assert!(statuses.iter().any(|s| s.severity == StatusCode::Info));
    }
}
