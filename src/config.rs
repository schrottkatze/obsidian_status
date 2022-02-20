use termcolor::{Color, ColorSpec};

use super::bar::{Bar, BarConfig, SepSet};
use super::module::{JustifyModule, Module};

pub const UPDATE_MS: u64 = 100;

pub fn make_bar() -> Bar {
    //let color_spec = make_color_spec(127,0,255);
    //let sep_color_spec = make_color_spec(0,127,255);
    let mut color_spec = ColorSpec::new();
    color_spec.set_fg(Some(Color::Rgb(127, 0, 255)));

    let mut sep_color_spec = ColorSpec::new();
    sep_color_spec.set_fg(Some(Color::Rgb(0, 127, 255)));

    let mut bar = Bar::new(BarConfig {
        sep_set: SepSet::DualDifferentAll(
            ("<".to_string(), ">".to_string()),
            ("{".to_string(), "}".to_string()),
            ("[".to_string(), "]".to_string()),
        ),
        color_spec,
        sep_color_spec,
        show_both_seps_on_overlap: false,
        outer_sep_config: (true, true),
    });

    bar.add_module(
        JustifyModule::Left,
        Module::new(5, || String::from("Helloo"), None, None, None),
    );
    bar.add_module(
        JustifyModule::Left,
        Module::new(5, || String::from("Helloo"), None, None, None),
    );

    bar.add_module(
        JustifyModule::Center,
        Module::new(12, || String::from("Hello World"), None, None, None),
    );

    bar.add_module(
        JustifyModule::Right,
        Module::new(6, || String::from("FooBar"), None, None, None),
    );
    bar.add_module(
        JustifyModule::Right,
        Module::new(6, || String::from("FooBar"), None, None, None),
    );
    bar.add_module(
        JustifyModule::Right,
        Module::new(6, || String::from("FooBar"), None, None, None),
    );

    bar
}
