use std::f32::consts::PI;

use shuten::{style::Rgb, Config};

use shuten_ui::{
    run,
    widgets::{border, BorderStyle},
};

pub fn main() -> std::io::Result<()> {
    run(Config::default().fixed_timer(60.0), |ui| {
        ui.center(|ui| {
            border(ui, BorderStyle::ROUNDED, |ui| {
                ui.label("asdf");
            })

            // SliderWidget::show(
            //     ui,
            //     Slider::new(0.5, 0.0..=1.0)
            //         .style(SliderStyle::SLIM)
            //         .horizontal(),
            // );
        });
    })
}

#[allow(dead_code)]
fn next_color(n: f32) -> Rgb {
    let h = n * ((1.0 + 5.0_f32.sqrt()) / 2.0);
    let h = (h + 0.5) * -1.0;
    let r = (PI * h).sin();
    let g = (PI * (h + 0.3)).sin();
    let b = (PI * (h + 0.6)).sin();
    Rgb::from_float([r * r, g * g, b * b])
}

// let (mut h, mut s, mut l) = (0.0_f32, 0.0_f32, 0.0_f32);

// ui.panel(|ui| {
//     ui.vertical(|ui| {
//         ui.slider(&mut h, 0.0..=360.0);
//         ui.slider(&mut s, 0.0..=1.0);
//         ui.slider(&mut l, 0.0..=1.0);
//     });

//     ui.align(Align2::RIGHT_CENTER, |ui| {
//         ui.filled(Hsl::new(h, s, l), |ui| ui.expand());
//     });
// });
