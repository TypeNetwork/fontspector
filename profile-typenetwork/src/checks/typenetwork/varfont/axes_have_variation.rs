use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};
use fontations::skrifa::MetadataProvider;


#[check(
    id = "typenetwork/varfont/axes_have_variation",
    rationale = "Axes on a variable font must have variation. In other words min and max values need to be different. It's common to find fonts with unnecessary axes added like `ital`.",
    proposal = "https://github.com/fonttools/fontbakery/pull/4260",
    title = "Check if font axes have variation",
)]
fn axes_have_variation(t: &Testable, _context: &Context) -> CheckFnResult {
    let font = testfont!(t);
    skip!(!font.is_variable_font(), "not-variable", "Not a variable font");
    let mut problems = vec![];

    for axis in font.font().axes().iter() {
        println!("Checking axis '{}' with min value {} and max value {}", axis.tag(), axis.min_value(), axis.max_value());
        if axis.min_value() == axis.max_value() {
            problems.push(Status::fail(
                "axis-has-no-variation",
                &format!(
                    "'{}' axis has no variation its min and max values are {}, {}",
                    axis.tag(),
                    axis.min_value(),
                    axis.max_value()
                ),
            ));
        }
    }

    return_result(problems)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use fontations::write::{
        tables::fvar::{Fvar, VariationAxisRecord},
        tables::head::Head,
        tables::hhea::Hhea,
        tables::maxp::Maxp,
        tables::name::{Name, NameRecord},
        tables::os2::Os2,
        types::{Fixed, NameId, Tag},
        FontBuilder,
    };
    use fontspector_checkapi::{Context, StatusCode, Testable};

    fn make_var_font_with_axes(axes: Vec<(&[u8; 4], f64, f64)>) -> Testable {
        let mut fb = FontBuilder::new();
        // add required tables
        fb.add_table(&Head::default()).unwrap();
        fb.add_table(&Hhea::default()).unwrap();
        fb.add_table(&Maxp::default()).unwrap();
        fb.add_table(&Os2::default()).unwrap();
        // add name
        let mut name = Name::default();
        let record = NameRecord::new(3, 1, 1033, NameId::FAMILY_NAME, "Test VF".to_string().into());
        name.name_record = vec![record];
        fb.add_table(&name).unwrap();

        // add fvar
        let mut fvar = Fvar::default();
        for (tag, min, max) in axes {
            let mut axis = VariationAxisRecord::default();
            axis.axis_tag = Tag::new(tag);
            axis.min_value = Fixed::from_f64(min);
            axis.max_value = Fixed::from_f64(max);
            fvar.axis_instance_arrays.axes.push(axis);
        }
        fb.add_table(&fvar).unwrap();

        Testable::new_with_contents("test.ttf".to_string(), fb.build())
    }

    #[test]
    fn pass_when_axes_have_variation() {
        let f = make_var_font_with_axes(vec![(b"wdth", 50.0, 200.0), (b"wght", 100.0, 900.0)]);
        let result = axes_have_variation_impl(&f, &Context::default()).unwrap();
        let statuses: Vec<_> = result.collect();
        assert!(statuses.iter().all(|s| s.severity == StatusCode::Pass));
    }

    #[test]
    fn fail_when_axis_has_no_variation() {
        let f = make_var_font_with_axes(vec![(b"wght", 100.0, 100.0)]);
        let result = axes_have_variation_impl(&f, &Context::default()).unwrap();

        let statuses: Vec<_> = result.collect();
        assert!(statuses.iter().any(|s| s.severity == StatusCode::Fail));
    }
}