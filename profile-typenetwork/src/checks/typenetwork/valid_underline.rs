use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};
use fontations::skrifa::raw::TableProvider;

#[check(
    id = "typenetwork/valid_underline",
    rationale = "If underline thickness is not set nothing gets rendered on Figma.",
    proposal = "https://github.com/fonttools/fontbakery/pull/4260",
    title = "Font has a valid underline thickness?"
)]
fn valid_underline(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let underline_thickness = f.font().post()?.underline_thickness();

    if underline_thickness == 0i16.into() {
        Ok(Status::just_one_fail(
            "invalid-underline-thickness",
            &format!("Thickness of the underline is {underline_thickness} which is not valid."),
        ))
    } else {
        Ok(Status::just_one_pass())
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use fontations::write::{
        tables::maxp::Maxp,
        tables::post::Post,
        FontBuilder,
    };
    use fontspector_checkapi::{Context, StatusCode, Testable};

    #[test]
    fn test_valid_underline_pass() {
        let mut font_builder = FontBuilder::new();
        let maxp = Maxp::default();
        font_builder.add_table(&maxp).unwrap();

        let post = Post {
            underline_thickness: 100i16.into(),
            ..Default::default()
        };
        font_builder.add_table(&post).unwrap();

        let font = font_builder.build();
        let testable = Testable::new_with_contents("test.ttf", font);
        let context = Context::default();

        let result = valid_underline_impl(&testable, &context).unwrap();
        let statuses: Vec<_> = result.collect();

        assert_eq!(statuses.len(), 1);
        assert_eq!(statuses[0].severity, StatusCode::Pass);
    }

    #[test]
    fn test_valid_underline_fail_zero() {
        let mut font_builder = FontBuilder::new();
        let maxp = Maxp::default();
        font_builder.add_table(&maxp).unwrap();

        let post = Post {
            underline_thickness: 0i16.into(),
            ..Default::default()
        };
        font_builder.add_table(&post).unwrap();

        let font = font_builder.build();
        let testable = Testable::new_with_contents("test.ttf", font);
        let context = Context::default();

        let result = valid_underline_impl(&testable, &context).unwrap();
        let statuses: Vec<_> = result.collect();

        assert_eq!(statuses.len(), 1);
        assert_eq!(statuses[0].severity, StatusCode::Fail);
        assert_eq!(statuses[0].code, Some("invalid-underline-thickness".to_string()));
    }
}
