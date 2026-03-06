#![deny(clippy::unwrap_used, clippy::expect_used)]
pub mod checks;
use serde_json::json;
use std::collections::HashMap;

use fontspector_checkapi::{Override, ProfileBuilder, Registry, StatusCode};

pub struct TypeNetwork;
impl fontspector_checkapi::Plugin for TypeNetwork {
    fn register(&self, cr: &mut Registry) -> Result<(), String> {
        let builder = ProfileBuilder::new()
            .include_profile("universal")
                .with_configuration_defaults(
                    "universal/required_name_ids",
                    HashMap::from([
                        ("required_name_ids".to_string(), json!([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 16, 17, 25])),
                    ]),
            )
            // excludes
            .exclude_check("opentype/family/panose_familytype")
            .exclude_check("opentype/family/opentype/vendor_id")
            .exclude_check("alt_caron")
            .exclude_check("opentype/family/opentype/file_size")
            .exclude_check("hinting_impact")
            .exclude_check("hinting_impact")
            .exclude_check("math_signs_width") // It really depends on the design and the intended use to make math symbols the same width.
            .exclude_check("name/no_copyright_on_description")
            .with_overrides("varfont/unsupported_axes", vec![
                Override::new("unsupported-ital", StatusCode::Warn, "The 'ital' axis is not supported well supported, is recommended to use 'slnt' instead.")
            ])
            // .add_section("Adobe Fonts Checks")
            .include_profile("adobefonts")
            // .add_and_register_check(checks::adobefonts::family::consistent_upm)
            // .add_and_register_check(checks::adobefonts::nameid_1_win_english)
            // .add_and_register_check(checks::adobefonts::unsupported_tables)
            // .add_and_register_check(checks::adobefonts::STAT_strings)
            // .include_check("fontwerk/style_linking")
            
            .add_section("Type Network")
            .add_and_register_check(checks::typenetwork::weightclass)
            .add_and_register_check(checks::typenetwork::duplicated_names)
            .add_and_register_check(checks::typenetwork::valid_strikeout)
            .add_and_register_check(checks::typenetwork::equal_numbers_of_glyphs)
            .add_and_register_check(checks::typenetwork::valid_underline)

            .add_and_register_check(checks::typenetwork::axes_have_variation)
            // .add_and_register_check(checks::typenetwork::fvar_axes_order)
            // "typenetwork/family/duplicated_names",
            // "typenetwork/family/equal_numbers_of_glyphs",
            // "typenetwork/family/valid_strikeout",
            // "typenetwork/family/valid_underline",
            // "typenetwork/font_is_centered_vertically",
            // "typenetwork/glyph_coverage",
            // "typenetwork/marks_width",
            // "typenetwork/name/mandatory_entries",
            // "typenetwork/PUA_encoded_glyphs",
            // "typenetwork/varfont/axes_have_variation",
            // "typenetwork/varfont/fvar_axes_order",
            // "typenetwork/vertical_metrics",
            // "typenetwork/weightclass",

            // .include_profile("googlefonts")

            // "dotted_circle",
            // "soft_dotted",

            // .with_overrides("valid_glyphnames", vec![
            //     Override::new("found-invalid-names", StatusCode::Warn, "")
            // ])
            // .with_overrides("soft_hyphen", vec![
            //     Override::new("softhyphen", StatusCode::Fail, "For TypeNetwork, the 'Soft Hyphen' character must be removed.")
            // ])
            // // exclude googlefonts checks
            // .exclude_check("googlefonts/canonical_filename")
            // .exclude_check("googlefonts/family/italics_have_roman_counterparts")  // May need some improvements before we decide to include this one.
            // .exclude_check("googlefonts/font_copyright")
            // .exclude_check("googlefonts/fstype")
            // .exclude_check("googlefonts/gasp")
            // .exclude_check("googlefonts/metadata/includes_production_subsets")
            // .exclude_check("googlefonts/meta/script_lang_tags")
            // .exclude_check("googlefonts/name/description_max_length")
            // .exclude_check("googlefonts/name/line_breaks")
            // .exclude_check("googlefonts/production_glyphs_similarity")
            // .exclude_check("googlefonts/vendor_id") // Custom TypeNetwork test below
            // .exclude_check("googlefonts/version_bump")
            // .exclude_check("googlefonts/font_names")
            // .exclude_check("googlefonts/varfont/has_HVAR")
            // .exclude_check("googlefonts/weightclass")
            // .exclude_check("control_chars")
            // .exclude_check("fontdata_namecheck")
            // .include_profile("opentype")
            // .add_section("Fontwerk Checks")
            // .add_and_register_check(checks::fontwerk::name_entries)
            // .add_and_register_check(checks::fontwerk::name_consistency)
            // .add_and_register_check(checks::fontwerk::fstype)
            // .add_and_register_check(checks::fontwerk::glyph_coverage)
            // .add_and_register_check(checks::fontwerk::weightclass)
            // TODO: implement other Fontwerk checks
            // .add_and_register_check("fontwerk/names_match_default_fvar")

            // .with_configuration_defaults(
            //     "opentype/vendor_id",
            //     HashMap::from([
            //         ("vendor_id".to_string(), json!("WERK"))
            //     ]),
            // )
            // .with_configuration_defaults(
            //     "fontwerk/name_entries",
            //     HashMap::from([
            //         ("COPYRIGHT_NOTICE".to_string(), json!(r"regex:Copyright \(c\) (\d{4}(-\d{4})?, )*\d{4}(-\d{4})? Fontwerk GmbH\. All rights reserved\.")),
            //         ("MANUFACTURER".to_string(), json!("Fontwerk")),
            //         ("VENDOR_URL".to_string(), json!("https://fontwerk.com")),
            //         ("LICENSE_DESCRIPTION".to_string(), json!("This Font Software is the property of Fontwerk GmbH its use by you is covered under the terms of an End-User License Agreement (EULA). Unless you have entered into a specific license agreement granting you additional rights, your use of this Font Software is limited by the terms of the actual license agreement you have entered into with Fontwerk. If you have any questions concerning your rights you should review the EULA you received with the software or contact Fontwerk. A copy of the EULA for this Font Software can be found on https://fontwerk.com/licensing.")),
            //         ("LICENSE_URL".to_string(), json!("https://fontwerk.com")),
            //         ]),
            // );
            ;
        builder.build("typenetwork", cr)
    }
}

#[cfg(not(target_family = "wasm"))]
pluginator::plugin_implementation!(fontspector_checkapi::Plugin, TypeNetwork);
