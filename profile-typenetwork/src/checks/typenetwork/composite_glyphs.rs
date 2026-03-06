use fontspector_checkapi::{prelude::*, testfont};

#[check(
    id = "typenetwork/composite_glyphs",
    rationale = "
        For performance reasons, it is recommended that TTF fonts use composite glyphs.
    ",
    conditions = ["is_ttf"],
    proposal = "https://github.com/fonttools/fontbakery/pull/4260",
    title = "Check if TTF font uses composite glyphs."
)]
fn composite_glyphs(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let glyf = f.font().glyf().map_err(|_| "No glyf table")?;
    let maxp = f.font().maxp().map_err(|_| "No maxp table")?;
    let num_glyphs = maxp.num_glyphs();

    let base_glyphs: std::collections::HashSet<String> = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~ \t\n\r\x0b\x0c".chars().map(|c| c.to_string()).collect();

    let mut not_composite = 0;
    for glyph_name in glyf.glyphs().keys() {
        if !base_glyphs.contains(glyph_name) {
            if let Some(glyph) = glyf.get(glyph_name) {
                if !glyph.is_composite() {
                    not_composite += 1;
                }
            }
        }
    }

    let percentage = (not_composite * 100) / num_glyphs as usize;
    if percentage > 50 {
        Ok(Status::just_one_warn(
            "low-composites",
            &format!("{}% of the glyphs are not composites.", percentage),
        ))
    } else {
        Ok(Status::just_one_pass(
            &format!("{}% of the glyphs are composites.", 100 - percentage),
        ))
    }
}