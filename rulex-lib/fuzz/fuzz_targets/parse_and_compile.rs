#![no_main]
use libfuzzer_sys::fuzz_target;

use rulex::features::RulexFeatures;
use rulex::options::{CompileOptions, ParseOptions, RegexFlavor};
use rulex::Rulex;

fuzz_target!(|data: (u8, u8, u16, &str)| {
    let (flavor, max_range_size, feature_bits, input) = data;

    let flavor = match flavor % 6 {
        0 => RegexFlavor::Pcre,
        1 => RegexFlavor::Python,
        2 => RegexFlavor::Java,
        3 => RegexFlavor::JavaScript,
        4 => RegexFlavor::DotNet,
        _ => RegexFlavor::Ruby,
    };

    // enable/disable features randomly
    let mut allowed_features = RulexFeatures::default();
    allowed_features.grapheme((feature_bits >> 0) & 1 == 1);
    allowed_features.numbered_groups((feature_bits >> 1) & 1 == 1);
    allowed_features.named_groups((feature_bits >> 2) & 1 == 1);
    allowed_features.references((feature_bits >> 3) & 1 == 1);
    allowed_features.lazy_mode((feature_bits >> 4) & 1 == 1);
    allowed_features.ranges((feature_bits >> 5) & 1 == 1);
    allowed_features.variables((feature_bits >> 6) & 1 == 1);
    allowed_features.lookahead((feature_bits >> 7) & 1 == 1);
    allowed_features.lookbehind((feature_bits >> 8) & 1 == 1);
    allowed_features.boundaries((feature_bits >> 9) & 1 == 1);

    let parse_opts = ParseOptions { max_range_size, allowed_features };
    let compile_opts = CompileOptions { flavor };

    let _ = Rulex::parse_and_compile(input, parse_opts, compile_opts);
});
