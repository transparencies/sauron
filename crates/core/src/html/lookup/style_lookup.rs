use once_cell::sync::Lazy;
use phf::phf_map;
use std::collections::BTreeSet;
use std::iter::FromIterator;

/// return the corrected property name
pub fn match_property(property: &str) -> Option<&str> {
    if let Some(html_style) = HTML_STYLES.get(property) {
        Some(*html_style)
    } else if let Some(svg_style) = SVG_STYLES.get(property) {
        Some(svg_style)
    } else if let Some(style) = ALL_STYLES.get(property) {
        Some(style)
    } else {
        None
    }
}

static ALL_STYLES: Lazy<BTreeSet<&'static str>> = Lazy::new(|| {
    BTreeSet::from_iter(
        HTML_STYLES
            .entries()
            .chain(SVG_STYLES.entries())
            .map(|(_ident, style)| *style),
    )
});

/// A list of ident style in snake_case style
/// [Reference](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Properties_Reference)
/// [Reference](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference)
static HTML_STYLES: phf::Map<&'static str, &'static str> = phf_map! {
    "accent_color" => "accent-color",
    "additive_symbols" => "additive-symbols",
    "align_content" => "align-content",
    "align_items" => "align-items",
    "align_self" => "align-self",
    "align_tracks" => "align-tracks",
    "all" => "all",
    "animation" => "animation",
    "animation_delay" => "animation-delay",
    "animation_direction" => "animation-direction",
    "animation_duration" => "animation-duration",
    "animation_fill_mode" => "animation-fill-mode",
    "animation_iteration_count" => "animation-iteration-count",
    "animation_name" => "animation-name",
    "animation_play_state" => "animation-play-state",
    "animation_timing_function" => "animation-timing-function",
    "@annotation" => "@annotation",
    "appearance" => "appearance",
    "ascent_override" => "ascent-override",
    "aspect_ratio" => "aspect-ratio",
    "backdrop_filter" => "backdrop-filter",
    "backface_visibility" => "backface-visibility",
    "background" => "background",
    "background_attachment" => "background-attachment",
    "background_blend_mode" => "background-blend-mode",
    "background_clip" => "background-clip",
    "background_color" => "background-color",
    "background_image" => "background-image",
    "background_origin" => "background-origin",
    "background_position" => "background-position",
    "background_position_x" => "background-position-x",
    "background_position_y" => "background-position-y",
    "background_repeat" => "background-repeat",
    "background_size" => "background-size",
    "bleed" => "bleed",
    "block_overflow" => "block-overflow",
    "block_size" => "block-size",
    "border" => "border",
    "border_block" => "border-block",
    "border_block_color" => "border-block-color",
    "border_block_end" => "border-block-end",
    "border_block_end_color" => "border-block-end-color",
    "border_block_end_style" => "border-block-end-style",
    "border_block_end_width" => "border-block-end-width",
    "border_block_start" => "border-block-start",
    "border_block_start_color" => "border-block-start-color",
    "border_block_start_style" => "border-block-start-style",
    "border_block_start_width" => "border-block-start-width",
    "border_block_style" => "border-block-style",
    "border_block_width" => "border-block-width",
    "border_bottom" => "border-bottom",
    "border_bottom_color" => "border-bottom-color",
    "border_bottom_left_radius" => "border-bottom-left-radius",
    "border_bottom_right_radius" => "border-bottom-right-radius",
    "border_bottom_style" => "border-bottom-style",
    "border_bottom_width" => "border-bottom-width",
    "border_collapse" => "border-collapse",
    "border_color" => "border-color",
    "border_end_end_radius" => "border-end-end-radius",
    "border_end_start_radius" => "border-end-start-radius",
    "border_image" => "border-image",
    "border_image_outset" => "border-image-outset",
    "border_image_repeat" => "border-image-repeat",
    "border_image_slice" => "border-image-slice",
    "border_image_source" => "border-image-source",
    "border_image_width" => "border-image-width",
    "border_inline" => "border-inline",
    "border_inline_color" => "border-inline-color",
    "border_inline_end" => "border-inline-end",
    "border_inline_end_color" => "border-inline-end-color",
    "border_inline_end_style" => "border-inline-end-style",
    "border_inline_end_width" => "border-inline-end-width",
    "border_inline_start" => "border-inline-start",
    "border_inline_start_color" => "border-inline-start-color",
    "border_inline_start_style" => "border-inline-start-style",
    "border_inline_start_width" => "border-inline-start-width",
    "border_inline_style" => "border-inline-style",
    "border_inline_width" => "border-inline-width",
    "border_left" => "border-left",
    "border_left_color" => "border-left-color",
    "border_left_style" => "border-left-style",
    "border_left_width" => "border-left-width",
    "border_radius" => "border-radius",
    "border_right" => "border-right",
    "border_right_color" => "border-right-color",
    "border_right_style" => "border-right-style",
    "border_right_width" => "border-right-width",
    "border_spacing" => "border-spacing",
    "border_start_end_radius" => "border-start-end-radius",
    "border_start_start_radius" => "border-start-start-radius",
    "border_style" => "border-style",
    "border_top" => "border-top",
    "border_top_color" => "border-top-color",
    "border_top_left_radius" => "border-top-left-radius",
    "border_top_right_radius" => "border-top-right-radius",
    "border_top_style" => "border-top-style",
    "border_top_width" => "border-top-width",
    "border_width" => "border-width",
    "bottom" => "bottom",
    "@bottom_center" => "@bottom-center",
    "box_decoration_break" => "box-decoration-break",
    "box_shadow" => "box-shadow",
    "box_sizing" => "box-sizing",
    "break_after" => "break-after",
    "break_before" => "break-before",
    "break_inside" => "break-inside",
    "caption_side" => "caption-side",
    "caret_color" => "caret-color",
    "@character_variant" => "@character-variant",
    "@charset" => "@charset",
    "clear" => "clear",
    "clip" => "clip",
    "clip_path" => "clip-path",
    "color" => "color",
    "color_adjust" => "color-adjust",
    "color_scheme" => "color-scheme",
    "column_count" => "column-count",
    "column_fill" => "column-fill",
    "column_gap" => "column-gap",
    "column_rule" => "column-rule",
    "column_rule_color" => "column-rule-color",
    "column_rule_style" => "column-rule-style",
    "column_rule_width" => "column-rule-width",
    "column_span" => "column-span",
    "column_width" => "column-width",
    "columns" => "columns",
    "contain" => "contain",
    "content" => "content",
    "content_visibility" => "content-visibility",
    "<counter>" => "<counter>",
    "counter_increment" => "counter-increment",
    "counter_reset" => "counter-reset",
    "counter_set" => "counter-set",
    "@counter_style" => "@counter-style",
    "cursor" => "cursor",
    "direction" => "direction",
    "display" => "display",
    "empty_cells" => "empty-cells",
    "fallback" => "fallback",
    "filter" => "filter",
    "flex" => "flex",
    "flex_basis" => "flex-basis",
    "flex_direction" => "flex-direction",
    "flex_flow" => "flex-flow",
    "flex_grow" => "flex-grow",
    "flex_shrink" => "flex-shrink",
    "flex_wrap" => "flex-wrap",
    "float" => "float",
    "font" => "font",
    "font_display" => "font-display",
    "@font_face" => "@font-face",
    "font_family" => "font-family",
    "font_feature_settings" => "font-feature-settings",
    "@font_feature_values" => "@font-feature-values",
    "font_kerning" => "font-kerning",
    "font_language_override" => "font-language-override",
    "font_optical_sizing" => "font-optical-sizing",
    "font_size" => "font-size",
    "font_size_adjust" => "font-size-adjust",
    "font_stretch" => "font-stretch",
    "font_style" => "font-style",
    "font_synthesis" => "font-synthesis",
    "font_variant" => "font-variant",
    "font_variant_alternates" => "font-variant-alternates",
    "font_variant_caps" => "font-variant-caps",
    "font_variant_east_asian" => "font-variant-east-asian",
    "font_variant_ligatures" => "font-variant-ligatures",
    "font_variant_numeric" => "font-variant-numeric",
    "font_variant_position" => "font-variant-position",
    "font_variation_settings" => "font-variation-settings",
    "font_weight" => "font-weight",
    "forced_color_adjust" => "forced-color-adjust",
    "gap" => "gap",
    "grid" => "grid",
    "grid_area" => "grid-area",
    "grid_auto_columns" => "grid-auto-columns",
    "grid_auto_flow" => "grid-auto-flow",
    "grid_auto_rows" => "grid-auto-rows",
    "grid_column" => "grid-column",
    "grid_column_end" => "grid-column-end",
    "grid_column_start" => "grid-column-start",
    "grid_row" => "grid-row",
    "grid_row_end" => "grid-row-end",
    "grid_row_start" => "grid-row-start",
    "grid_template" => "grid-template",
    "grid_template_areas" => "grid-template-areas",
    "grid_template_columns" => "grid-template-columns",
    "grid_template_rows" => "grid-template-rows",
    "hanging_punctuation" => "hanging-punctuation",
    "height" => "height",
    "hyphens" => "hyphens",
    "image_orientation" => "image-orientation",
    "image_rendering" => "image-rendering",
    "image_resolution" => "image-resolution",
    "@import" => "@import",
    "inherit" => "inherit",
    "inherits" => "inherits",
    "initial" => "initial",
    "initial_letter" => "initial-letter",
    "initial_letter_align" => "initial-letter-align",
    "initial_value" => "initial-value",
    "inline_size" => "inline-size",
    "inset" => "inset",
    "inset_block" => "inset-block",
    "inset_block_end" => "inset-block-end",
    "inset_block_start" => "inset-block-start",
    "inset_inline" => "inset-inline",
    "inset_inline_end" => "inset-inline-end",
    "inset_inline_start" => "inset-inline-start",
    "isolation" => "isolation",
    "justify_content" => "justify-content",
    "justify_items" => "justify-items",
    "justify_self" => "justify-self",
    "justify_tracks" => "justify-tracks",
    "@keyframes" => "@keyframes",
    "left" => "left",
    "@left_bottom" => "@left-bottom",
    "letter_spacing" => "letter-spacing",
    "line_break" => "line-break",
    "line_clamp" => "line-clamp",
    "line_gap_override" => "line-gap-override",
    "line_height" => "line-height",
    "line_height_step" => "line-height-step",
    "list_style" => "list-style",
    "list_style_image" => "list-style-image",
    "list_style_position" => "list-style-position",
    "list_style_type" => "list-style-type",
    "margin" => "margin",
    "margin_block" => "margin-block",
    "margin_block_end" => "margin-block-end",
    "margin_block_start" => "margin-block-start",
    "margin_bottom" => "margin-bottom",
    "margin_inline" => "margin-inline",
    "margin_inline_end" => "margin-inline-end",
    "margin_inline_start" => "margin-inline-start",
    "margin_left" => "margin-left",
    "margin_right" => "margin-right",
    "margin_top" => "margin-top",
    "margin_trim" => "margin-trim",
    "marks" => "marks",
    "mask" => "mask",
    "mask_border" => "mask-border",
    "mask_border_mode" => "mask-border-mode",
    "mask_border_outset" => "mask-border-outset",
    "mask_border_repeat" => "mask-border-repeat",
    "mask_border_slice" => "mask-border-slice",
    "mask_border_source" => "mask-border-source",
    "mask_border_width" => "mask-border-width",
    "mask_clip" => "mask-clip",
    "mask_composite" => "mask-composite",
    "mask_image" => "mask-image",
    "mask_mode" => "mask-mode",
    "mask_origin" => "mask-origin",
    "mask_position" => "mask-position",
    "mask_repeat" => "mask-repeat",
    "mask_size" => "mask-size",
    "mask_type" => "mask-type",
    "masonry_auto_flow" => "masonry-auto-flow",
    "math_style" => "math-style",
    "max_block_size" => "max-block-size",
    "max_height" => "max-height",
    "max_inline_size" => "max-inline-size",
    "max_lines" => "max-lines",
    "max_width" => "max-width",
    "max_zoom" => "max-zoom",
    "@media" => "@media",
    "min_block_size" => "min-block-size",
    "min_height" => "min-height",
    "min_inline_size" => "min-inline-size",
    "min_width" => "min-width",
    "min_zoom" => "min-zoom",
    "mix_blend_mode" => "mix-blend-mode",
    "@namespace" => "@namespace",
    "negative" => "negative",
    "object_fit" => "object-fit",
    "object_position" => "object-position",
    "offset" => "offset",
    "offset_anchor" => "offset-anchor",
    "offset_distance" => "offset-distance",
    "offset_path" => "offset-path",
    "offset_position" => "offset-position",
    "offset_rotate" => "offset-rotate",
    "opacity" => "opacity",
    "order" => "order",
    "orientation" => "orientation",
    "@ornaments" => "@ornaments",
    "orphans" => "orphans",
    "outline" => "outline",
    "outline_color" => "outline-color",
    "outline_offset" => "outline-offset",
    "outline_style" => "outline-style",
    "outline_width" => "outline-width",
    "overflow" => "overflow",
    "overflow_anchor" => "overflow-anchor",
    "overflow_block" => "overflow-block",
    "overflow_clip_margin" => "overflow-clip-margin",
    "overflow_inline" => "overflow-inline",
    "overflow_wrap" => "overflow-wrap",
    "overflow_x" => "overflow-x",
    "overflow_y" => "overflow-y",
    "overscroll_behavior" => "overscroll-behavior",
    "overscroll_behavior_block" => "overscroll-behavior-block",
    "overscroll_behavior_inline" => "overscroll-behavior-inline",
    "overscroll_behavior_x" => "overscroll-behavior-x",
    "overscroll_behavior_y" => "overscroll-behavior-y",
    "pad" => "pad",
    "padding" => "padding",
    "padding_block" => "padding-block",
    "padding_block_end" => "padding-block-end",
    "padding_block_start" => "padding-block-start",
    "padding_bottom" => "padding-bottom",
    "padding_inline" => "padding-inline",
    "padding_inline_end" => "padding-inline-end",
    "padding_inline_start" => "padding-inline-start",
    "padding_left" => "padding-left",
    "padding_right" => "padding-right",
    "padding_top" => "padding-top",
    "@page" => "@page",
    "page_break_after" => "page-break-after",
    "page_break_before" => "page-break-before",
    "page_break_inside" => "page-break-inside",
    "paint_order" => "paint-order",
    "perspective" => "perspective",
    "perspective_origin" => "perspective-origin",
    "place_content" => "place-content",
    "place_items" => "place-items",
    "place_self" => "place-self",
    "pointer_events" => "pointer-events",
    "position" => "position",
    "prefix" => "prefix",
    "@property" => "@property",
    "quotes" => "quotes",
    "range" => "range",
    "resize" => "resize",
    "revert" => "revert",
    "right" => "right",
    "@right_bottom" => "@right-bottom",
    "rotate" => "rotate",
    "row_gap" => "row-gap",
    "ruby_align" => "ruby-align",
    "ruby_merge" => "ruby-merge",
    "ruby_position" => "ruby-position",
    "scale" => "scale",
    "scroll_behavior" => "scroll-behavior",
    "scroll_margin" => "scroll-margin",
    "scroll_margin_block" => "scroll-margin-block",
    "scroll_margin_block_end" => "scroll-margin-block-end",
    "scroll_margin_block_start" => "scroll-margin-block-start",
    "scroll_margin_bottom" => "scroll-margin-bottom",
    "scroll_margin_inline" => "scroll-margin-inline",
    "scroll_margin_inline_end" => "scroll-margin-inline-end",
    "scroll_margin_inline_start" => "scroll-margin-inline-start",
    "scroll_margin_left" => "scroll-margin-left",
    "scroll_margin_right" => "scroll-margin-right",
    "scroll_margin_top" => "scroll-margin-top",
    "scroll_padding" => "scroll-padding",
    "scroll_padding_block" => "scroll-padding-block",
    "scroll_padding_block_end" => "scroll-padding-block-end",
    "scroll_padding_block_start" => "scroll-padding-block-start",
    "scroll_padding_bottom" => "scroll-padding-bottom",
    "scroll_padding_inline" => "scroll-padding-inline",
    "scroll_padding_inline_end" => "scroll-padding-inline-end",
    "scroll_padding_inline_start" => "scroll-padding-inline-start",
    "scroll_padding_left" => "scroll-padding-left",
    "scroll_padding_right" => "scroll-padding-right",
    "scroll_padding_top" => "scroll-padding-top",
    "scroll_snap_align" => "scroll-snap-align",
    "scroll_snap_stop" => "scroll-snap-stop",
    "scroll_snap_type" => "scroll-snap-type",
    "scrollbar_color" => "scrollbar-color",
    "scrollbar_gutter" => "scrollbar-gutter",
    "scrollbar_width" => "scrollbar-width",
    "shape_image_threshold" => "shape-image-threshold",
    "shape_margin" => "shape-margin",
    "shape_outside" => "shape-outside",
    "size" => "size",
    "size_adjust" => "size-adjust",
    "speak_as" => "speak-as",
    "src" => "src",
    "@styleset" => "@styleset",
    "@stylistic" => "@stylistic",
    "suffix" => "suffix",
    "@supports" => "@supports",
    "@swash" => "@swash",
    "symbols" => "symbols",
    "syntax" => "syntax",
    "system" => "system",
    "tab_size" => "tab-size",
    "table_layout" => "table-layout",
    "text_align" => "text-align",
    "text_align_last" => "text-align-last",
    "text_combine_upright" => "text-combine-upright",
    "text_decoration" => "text-decoration",
    "text_decoration_color" => "text-decoration-color",
    "text_decoration_line" => "text-decoration-line",
    "text_decoration_skip" => "text-decoration-skip",
    "text_decoration_skip_ink" => "text-decoration-skip-ink",
    "text_decoration_style" => "text-decoration-style",
    "text_decoration_thickness" => "text-decoration-thickness",
    "text_emphasis" => "text-emphasis",
    "text_emphasis_color" => "text-emphasis-color",
    "text_emphasis_position" => "text-emphasis-position",
    "text_emphasis_style" => "text-emphasis-style",
    "text_indent" => "text-indent",
    "text_justify" => "text-justify",
    "text_orientation" => "text-orientation",
    "text_overflow" => "text-overflow",
    "text_rendering" => "text-rendering",
    "text_shadow" => "text-shadow",
    "text_size_adjust" => "text-size-adjust",
    "text_transform" => "text-transform",
    "text_underline_offset" => "text-underline-offset",
    "text_underline_position" => "text-underline-position",
    "top" => "top",
    "@top_center" => "@top-center",
    "touch_action" => "touch-action",
    "transform" => "transform",
    "transform_box" => "transform-box",
    "transform_origin" => "transform-origin",
    "transform_style" => "transform-style",
    "transition" => "transition",
    "transition_delay" => "transition-delay",
    "transition_duration" => "transition-duration",
    "transition_property" => "transition-property",
    "transition_timing_function" => "transition-timing-function",
    "translate" => "translate",
    "unicode_bidi" => "unicode-bidi",
    "unicode_range" => "unicode-range",
    "unset" => "unset",
    "user_select" => "user-select",
    "user_zoom" => "user-zoom",
    "vertical_align" => "vertical-align",
    "@viewport" => "@viewport",
    "viewport_fit" => "viewport-fit",
    "visibility" => "visibility",
    "white_space" => "white-space",
    "widows" => "widows",
    "width" => "width",
    "will_change" => "will-change",
    "word_break" => "word-break",
    "word_spacing" => "word-spacing",
    "word_wrap" => "word-wrap",
    "writing_mode" => "writing-mode",
    "z_index" => "z-index",
    "zoom" => "zoom",
};

/// A list of svg style and their ident using snake_case
/// [Reference](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute)
pub(super) static SVG_STYLES: phf::Map<&'static str, &'static str> = phf_map! {
        "accent_height" => "accent-height",
        "accumulate" => "accumulate",
        "additive" => "additive",
        "alignment_baseline" => "alignment-baseline",
        "alphabetic" => "alphabetic",
        "amplitude" => "amplitude",
        "arabic_form" => "arabic-form",
        "ascent" => "ascent",
        "attribute_name" => "attributeName",
        "attribute_type" => "attributeType",
        "azimuth" => "azimuth",
        "base_frequency" => "baseFrequency",
        "baseline_shift" => "baseline-shift",
        "baseProfile" => "baseProfile",
        "bbox" => "bbox",
        "begin" => "begin",
        "bias" => "bias",
        "by" => "by",
        "calc_mode" => "calcMode",
        "cap_height" => "cap-height",
        "class" => "class",
        "clip" => "clip",
        "clip_path_units" => "clipPathUnits",
        "clip_path" => "clip-path",
        "clip_rule" => "clip-rule",
        "color" => "color",
        "color_interpolation" => "color-interpolation",
        "color_interpolation_filters" => "color-interpolation-filters",
        "color_profile" => "color-profile",
        "color_rendering" => "color-rendering",
        "content_script_type" => "contentScriptType",
        "content_style_type" => "contentStyleType",
        "crossorigin" => "crossorigin",
        "cursor" => "cursor",
        "cx" => "cx",
        "cy" => "cy",
        "d" => "d",
        "decelerate" => "decelerate",
        "descent" => "descent",
        "diffuse_constant" => "diffuseConstant",
        "direction" => "direction",
        "display" => "display",
        "divisor" => "divisor",
        "dominant_baseline" => "dominant-baseline",
        "dur" => "dur",
        "dx" => "dx",
        "dy" => "dy",
        "edge_mode" => "edgeMode",
        "elevation" => "elevation",
        "enable_background" => "enable-background",
        "end" => "end",
        "exponent" => "exponent",
        "fill" => "fill",
        "fill_opacity" => "fill-opacity",
        "fill_rule" => "fill-rule",
        "filter" => "filter",
        "filter_res" => "filterRes",
        "filter_units" => "filterUnits",
        "flood_color" => "flood-color",
        "flood_opacity" => "flood-opacity",
        "font_family" => "font-family",
        "font_size" => "font-size",
        "font_size_adjust" => "font-size-adjust",
        "font_stretch" => "font-stretch",
        "font_style" => "font-style",
        "font_variant" => "font-variant",
        "font_weight" => "font-weight",
        "format" => "format",
        "from" => "from",
        "fr" => "fr",
        "fx" => "fx",
        "fy" => "fy",
        "g1" => "g1",
        "g2" => "g2",
        "glyph_name" => "glyph-name",
        "glyph_orientation_horizontal" => "glyph-orientation-horizontal",
        "glyph_orientation_vertical" => "glyph-orientation-vertical",
        "glyph_ref" => "glyphRef",
        "gradient_transform" => "gradientTransform",
        "gradient_units" => "gradientUnits",
        "hanging" => "hanging",
        "height" => "height",
        "href" => "href",
        "hreflang" => "hreflang",
        "horiz_adv_x" => "horiz-adv-x",
        "horiz_origin_x" => "horiz-origin-x",
        "id" => "id",
        "ideographic" => "ideographic",
        "image_rendering" => "image-rendering",
        "in" => "in",
        "in2" => "in2",
        "intercept" => "intercept",
        "k" => "k",
        "k1" => "k1",
        "k2" => "k2",
        "k3" => "k3",
        "k4" => "k4",
        "kernel_matrix" => "kernelMatrix",
        "kernel_unit_length" => "kernelUnitLength",
        "kerning" => "kerning",
        "key_points" => "keyPoints",
        "key_splines" => "keySplines",
        "key_times" => "keyTimes",
        "lang" => "lang",
        "length_adjust" => "lengthAdjust",
        "letter_spacing" => "letter-spacing",
        "lighting_color" => "lighting-color",
        "limiting_cone_angle" => "limitingConeAngle",
        "local" => "local",
        "marker_end" => "marker-end",
        "marker_mid" => "marker-mid",
        "marker_start" => "marker-start",
        "marker_height" => "markerHeight",
        "marker_units" => "markerUnits",
        "marker_width" => "markerWidth",
        "mask" => "mask",
        "mask_content_units" => "maskContentUnits",
        "mask_units" => "maskUnits",
        "mathematical" => "mathematical",
        "max" => "max",
        "media" => "media",
        "method" => "method",
        "min" => "min",
        "mode" => "mode",
        "name" => "name",
        "num_octaves" => "numOctaves",
        "offset" => "offset",
        "opacity" => "opacity",
        "operator" => "operator",
        "order" => "order",
        "orient" => "orient",
        "orientation" => "orientation",
        "origin" => "origin",
        "overflow" => "overflow",
        "overline_position" => "overline-position",
        "overline_thickness" => "overline-thickness",
        "panose_1" => "panose-1",
        "paint_order" => "paint-order",
        "path" => "path",
        "path_length" => "pathLength",
        "pattern_content_units" => "patternContentUnits",
        "pattern_transform" => "patternTransform",
        "pattern_units" => "patternUnits",
        "ping" => "ping",
        "pointer_events" => "pointer-events",
        "points" => "points",
        "points_at_x" => "pointsAtX",
        "points_at_y" => "pointsAtY",
        "points_at_z" => "pointsAtZ",
        "preserve_alpha" => "preserveAlpha",
        "preserve_aspect_ratio" => "preserveAspectRatio",
        "primitive_units" => "primitiveUnits",
        "r" => "r",
        "radius" => "radius",
        "referrer_policy" => "referrerPolicy",
        "ref_x" => "refX",
        "ref_y" => "refY",
        "rel" => "rel",
        "rendering_intent" => "rendering-intent",
        "repeat_count" => "repeatCount",
        "repeat_dur" => "repeatDur",
        "required_extensions" => "requiredExtensions",
        "required_features" => "requiredFeatures",
        "restart" => "restart",
        "result" => "result",
        "rotate" => "rotate",
        "rx" => "rx",
        "ry" => "ry",
        "scale" => "scale",
        "seed" => "seed",
        "shape_rendering" => "shape-rendering",
        "slope" => "slope",
        "spacing" => "spacing",
        "specular_constant" => "specularConstant",
        "specular_exponent" => "specularExponent",
        "speed" => "speed",
        "spread_method" => "spreadMethod",
        "start_offset" => "startOffset",
        "std_deviation" => "stdDeviation",
        "stemh" => "stemh",
        "stemv" => "stemv",
        "stitch_tiles" => "stitchTiles",
        "stop_color" => "stop-color",
        "stop_opacity" => "stop-opacity",
        "strikethrough_position" => "strikethrough-position",
        "strikethrough_thickness" => "strikethrough-thickness",
        "string" => "string",
        "stroke" => "stroke",
        "stroke_dasharray" => "stroke-dasharray",
        "stroke_dashoffset" => "stroke-dashoffset",
        "stroke_linecap" => "stroke-linecap",
        "stroke_linejoin" => "stroke-linejoin",
        "stroke_miterlimit" => "stroke-miterlimit",
        "stroke_opacity" => "stroke-opacity",
        "stroke_width" => "stroke-width",
        "style" => "style",
        "surface_scale" => "surfaceScale",
        "system_language" => "systemLanguage",
        "tabindex" => "tabindex",
        "table_values" => "tableValues",
        "target" => "target",
        "target_x" => "targetX",
        "target_y" => "targetY",
        "text_anchor" => "text-anchor",
        "text_decoration" => "text-decoration",
        "text_rendering" => "text-rendering",
        "text_length" => "textLength",
        "to" => "to",
        "transform" => "transform",
        "transform_origin" => "transform-origin",
        "type" => "type",
        "u1" => "u1",
        "u2" => "u2",
        "underline_position" => "underline-position",
        "underline_thickness" => "underline-thickness",
        "unicode" => "unicode",
        "unicode_bidi" => "unicode-bidi",
        "unicode_range" => "unicode-range",
        "units_per_em" => "units-per-em",
        "v_alphabetic" => "v-alphabetic",
        "v_hanging" => "v-hanging",
        "v_ideographic" => "v-ideographic",
        "v_mathematical" => "v-mathematical",
        "values" => "values",
        "vector_effect" => "vector-effect",
        "version" => "version",
        "vert_adv_y" => "vert-adv-y",
        "vert_origin_x" => "vert-origin-x",
        "vert_origin_y" => "vert-origin-y",
        "view_box" => "viewBox",
        "view_target" => "viewTarget",
        "visibility" => "visibility",
        "width" => "width",
        "widths" => "widths",
        "word_spacing" => "word-spacing",
        "writing_mode" => "writing-mode",
        "x" => "x",
        "x_height" => "x-height",
        "x1" => "x1",
        "x2" => "x2",
        "x_channel_selector" => "xChannelSelector",
        "xlink_actuate" => "xlink:actuate",
        "xlink_arcrole" => "xlink:arcrole",
        "xlink_href" => "xlink:href",
        "xlink_role" => "xlink:role",
        "xlink_show" => "xlink:show",
        "xlink_title" => "xlink:title",
        "xlink_type" => "xlink:type",
        "xml_base" => "xml:base",
        "xml_lang" => "xml:lang",
        "xml_space" => "xml:space",
        "y" => "y",
        "y1" => "y1",
        "y2" => "y2",
        "y_channel_selector" => "yChannelSelector",
        "z" => "z",
        "zoom_and_pan" => "zoomAndPan",
};