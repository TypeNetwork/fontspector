use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};
use fontations::skrifa::raw::TableProvider;

#[check(
    id = "typenetwork/varfont/fvar_axes_order",
    rationale = "If a font doesn't have a STAT table, instances get sorted better on Adobe Apps when fvar axes follow a specific order: 'opsz', 'wdth', 'wght','ital', 'slnt'. We should deprecate this check since STAT is a required table.",
    proposal = "https://github.com/fonttools/fontbakery/pull/4260",
    title = "Check fvar axes order",
)]
fn fvar_axes_order(f: &Testable, context: &Context) -> CheckFnResult {
    let font = testfont!(f);
    let fvar = font.font().fvar()?;

    // Check if STAT table exists
    if font.font().stat().is_ok() {
        return Ok(vec![Status::skip(
            "has-stat",
            "The font has a STAT table. This will control instances order.",
        )].into_iter().boxed());
    }

    let preferred_order = ["opsz", "wdth", "wght", "ital", "slnt"];
    let mut font_registered_axes = Vec::new();
    let mut custom_axes = Vec::new();

    for (index, axis) in fvar.axes().iter().enumerate() {
        let tag_str = axis.axis_tag.to_string();
        if preferred_order.contains(&tag_str.as_str()) {
            font_registered_axes.push(tag_str);
        } else {
            custom_axes.push((tag_str, index));
        }
    }

    let filtered: Vec<&str> = preferred_order
        .iter()
        .filter(|p| font_registered_axes.contains(&p.to_string()))
        .copied()
        .collect();

    let mut problems = vec![];

    if filtered != font_registered_axes.iter().map(|s| s.as_str()).collect::<Vec<_>>() {
        problems.push(Status::warn(
            "axes-incorrect-order",
            &format!(
                "Font's registered axes are not in a correct order to get good instances sorting on Adobe apps.\n\nCurrent order is {:?}, but it should be {:?}",
                font_registered_axes, filtered
            ),
        ));
    }

    if !custom_axes.is_empty() {
        problems.push(Status::info(
            "custom-axes",
            &format!(
                "The font has custom axes with the indicated order:\n\n{:?}\n\nIts order can depend on the kind of variation and the subfamily",
                custom_axes
            ),
        ));
    }

    if problems.is_empty() {
        Ok(Status::just_one_pass())
    } else {
        return_result(problems)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use fontations::write::{
        tables::fvar::{Axis, Fvar, VariationAxisRecord},
        tables::name::{Name, NameRecord},
        tables::stat::Stat,
        types::NameId,
        FontBuilder,
    };
    use fontspector_checkapi::{Context, StatusCode, Testable};

    fn make_var_font_with_axes_and_stat(axes: Vec<(u32, f32, f32)>, has_stat: bool) -> Testable {
        let mut fb = FontBuilder::new();
        // add name
        let mut name = Name::default();
        let record = NameRecord::new(3, 1, 1033, NameId::FAMILY_NAME, "Test".into());
        name.name_record = vec![record];
        fb.add_table(&name).unwrap();

        // add fvar
        let mut fvar = Fvar::default();
        for (tag, min, max) in axes {
            let mut axis = VariationAxisRecord::default();
            axis.axis_tag = tag;
            axis.min_value = min;
            axis.max_value = max;
            fvar.axes.push(axis);
        }
        fb.add_table(&fvar).unwrap();

        if has_stat {
            fb.add_table(&Stat::default()).unwrap();
        }

        Testable::new_with_contents("test.ttf".to_string(), fb.build())
    }

    #[test]
    fn skip_when_has_stat() {
        let f = make_var_font_with_axes_and_stat(vec![(0x77647468, 50.0, 200.0)], true);
        let status = fvar_axes_order(&f, &Context::default()).unwrap();
        assert_eq!(status.code(), StatusCode::Skip);
    }

    #[test]
    fn pass_when_correct_order() {
        let f = make_var_font_with_axes_and_stat(vec![
            (0x6f70737a, 8.0, 144.0), // opsz
            (0x77647468, 50.0, 200.0), // wdth
            (0x77676874, 100.0, 900.0), // wght
        ], false);
        let status = fvar_axes_order(&f, &Context::default()).unwrap();
        assert_eq!(status.code(), StatusCode::Pass);
    }

    #[test]
    fn warn_when_incorrect_order() {
        let f = make_var_font_with_axes_and_stat(vec![
            (0x77647468, 50.0, 200.0), // wdth
            (0x6f70737a, 8.0, 144.0), // opsz
        ], false);
        let status = fvar_axes_order(&f, &Context::default()).unwrap();
        assert_eq!(status.code(), StatusCode::Warn);
    }

    #[test]
    fn info_for_custom_axes() {
        let f = make_var_font_with_axes_and_stat(vec![
            (0x6f70737a, 8.0, 144.0), // opsz
            (0x12345678, 0.0, 1.0), // custom
        ], false);
        let status = fvar_axes_order(&f, &Context::default()).unwrap();
        assert_eq!(status.code(), StatusCode::Info);
    }
}