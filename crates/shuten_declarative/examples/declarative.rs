use shuten::{
    geom::{vec2f, Align2, Pos2f},
    Config,
};
use shuten_declarative::{run, widgets::*, Hsl};

fn main() -> std::io::Result<()> {
    run(Config::default(), |term| {
        let color = state(|| Hsl::new(0.0, 0.0, 0.0));
        let pos = state(|| Pos2f::ZERO);

        offset(pos.get(), || {
            let resp = draggable(|| {
                column(|| {
                    let h = Slider::new(color.get().0, 0.0, 360.0)
                        .filled(0xFF0000)
                        .show()
                        .value;

                    let s = Slider::new(color.get().1, 0.0, 1.0)
                        .filled(0x00FF00)
                        .show()
                        .value;

                    let l = Slider::new(color.get().2, 0.0, 1.0)
                        .filled(0x0000FF)
                        .show()
                        .value;

                    let mut hsl = color.borrow_mut();

                    if let Some(h) = h {
                        hsl.0 = h;
                    }
                    if let Some(s) = s {
                        hsl.1 = s;
                    }
                    if let Some(l) = l {
                        hsl.2 = l;
                    }
                });
            });

            pos.set_if(resp.current())
        });

        align(Align2::RIGHT_TOP, || {
            color_box(color.get(), vec2f(20.0, 10.0));
        })
    })
}
