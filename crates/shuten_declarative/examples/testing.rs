use std::f32::consts::PI;

use shuten::geom::{vec2f, MainAxisAlignment, Margin};
use shuten_declarative::{
    debug_run,
    logger::Logger,
    widgets::{scrollable::scrollable, *},
    Config, Rgb, Term,
};

fn main() -> std::io::Result<()> {
    Logger::init(true);

    debug_run(Config::default().ctrl_z_switches(true), true, |mut term| {
        ui(&mut term);
    })
}

fn ui(_term: &mut Term<'_>) {
    container(0x222222, || {
        margin(Margin::same(3), || {
            container(0x444444, || {
                scrollable(|| {
                    for i in 0..100 {
                        let bg = next_color(i as f32 * 0.01);
                        List::row()
                            .main_axis_alignment(MainAxisAlignment::SpaceBetween)
                            .show(|| {
                                color_box(bg, vec2f(10.0, 1.0));
                                Label::new(format!("{bg:#X}")).fg(bg).show();
                                label(format!("#{i:03}"));
                            });
                    }
                });
            });
        });
    });
}

fn next_color(n: f32) -> Rgb {
    let h = n * ((1.0 + 5.0_f32.sqrt()) / 2.0);
    let h = (h + 0.5) * -1.0;
    let r = (PI * h).sin();
    let g = (PI * (h + 0.3)).sin();
    let b = (PI * (h + 0.6)).sin();
    Rgb::from_float([r * r, g * g, b * b])
}
