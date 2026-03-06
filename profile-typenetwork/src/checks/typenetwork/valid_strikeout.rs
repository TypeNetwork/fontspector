use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};
use fontations::skrifa::raw::TableProvider;

#[check(
    id = "typenetwork/valid_strikeout",
    rationale = "If strikeout size is not set, nothing gets rendered on Figma.",
    proposal = "https://github.com/fonttools/fontbakery/pull/4260",
    title = "Font has a valid strikeout size?"
)]
fn valid_strikeout(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let strikeout_size = f.font().os2()?.y_strikeout_size();

    if strikeout_size == 0 {
        Ok(Status::just_one_fail(
            "invalid-strikeout-size",
            &format!("Size of the strikeout is {strikeout_size} which is not valid."),
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
        tables::os2::Os2,
        FontBuilder,
    };
    use fontspector_checkapi::{Context, StatusCode, Testable};

    #[test]
    fn test_valid_strikeout_pass() {
        let mut font_builder = FontBuilder::new();
        let maxp = Maxp::default();
        font_builder.add_table(&maxp).unwrap();

        let os2 = Os2 {
            y_strikeout_size: 100,
            ..Default::default()
        };
        font_builder.add_table(&os2).unwrap();

        let font = font_builder.build();
        let testable = Testable::new_with_contents("test.ttf", font);
        let context = Context::default();

        let result = valid_strikeout_impl(&testable, &context).unwrap();
        let statuses: Vec<_> = result.collect();

        assert_eq!(statuses.len(), 1);
        assert_eq!(statuses[0].severity, StatusCode::Pass);
    }

    #[test]
    fn test_valid_strikeout_fail_zero() {
        let mut font_builder = FontBuilder::new();
        let maxp = Maxp::default();
        font_builder.add_table(&maxp).unwrap();

        let os2 = Os2 {
            y_strikeout_size: 0,
            ..Default::default()
        };
        font_builder.add_table(&os2).unwrap();

        let font = font_builder.build();
        let testable = Testable::new_with_contents("test.ttf", font);
        let context = Context::default();

        let result = valid_strikeout_impl(&testable, &context).unwrap();
        let statuses: Vec<_> = result.collect();

        assert_eq!(statuses.len(), 1);
        assert_eq!(statuses[0].severity, StatusCode::Fail);
        assert_eq!(statuses[0].code, Some("invalid-strikeout-size".to_string()));
    }
}
