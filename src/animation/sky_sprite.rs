use crate::animation::cloud_bank::CloudBank;
use crate::animation::sun_path::on_sun_path;
use crate::render::TerminalRenderer;
use std::io;

pub trait SkySprite {
    fn size(&self) -> (u16, u16);
    fn update(&mut self);
    fn render(&self, renderer: &mut TerminalRenderer, x: u16, y: u16) -> io::Result<()>;
}

pub fn fits(height: u16, horizon_y: u16) -> bool {
    horizon_y >= CloudBank::TOP + height
}

pub fn in_place_of_sun(
    progress: Option<f64>,
    (width, height): (u16, u16),
    term_width: u16,
    horizon_y: u16,
) -> (u16, u16) {
    match progress {
        Some(progress) => on_sun_path(
            progress,
            width,
            horizon_y.saturating_sub(height),
            term_width,
        ),
        None => (term_width.saturating_sub(width) / 2, CloudBank::TOP),
    }
}

pub fn in_place_of_moon(
    (moon_x, moon_y): (u16, u16),
    (width, height): (u16, u16),
    term_width: u16,
    horizon_y: u16,
) -> (u16, u16) {
    (
        moon_x.min(term_width.saturating_sub(width)),
        moon_y.min(horizon_y.saturating_sub(height)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animation::candlemas::GiantCrepe;
    use crate::animation::may_day::GiantMuguet;
    use crate::animation::moon::MoonSystem;
    use crate::animation::music::SkyNotes;
    use crate::scene::WorldScene;

    const SIZES: [(u16, u16); 3] = [
        (SkyNotes::WIDTH, SkyNotes::HEIGHT),
        (GiantCrepe::WIDTH, GiantCrepe::HEIGHT),
        (GiantMuguet::WIDTH, GiantMuguet::HEIGHT),
    ];
    const TERMINALS: [(u16, u16); 3] = [(80, 24), (120, 40), (200, 50)];

    fn assert_in_sky((x, y): (u16, u16), (width, height): (u16, u16), term: (u16, u16)) {
        let horizon_y = term.1 - WorldScene::GROUND_HEIGHT;
        assert!(y >= CloudBank::TOP, "{term:?}");
        assert!(y + height <= horizon_y, "{term:?}");
        assert!(x + width <= term.0, "{term:?}");
    }

    #[test]
    fn test_sky_sprites_stay_in_the_sky_by_day() {
        for size in SIZES {
            for (term_width, term_height) in TERMINALS {
                let horizon_y = term_height - WorldScene::GROUND_HEIGHT;
                let progresses = (0..=20).map(|step| Some(step as f64 / 20.0));
                for progress in progresses.chain([None]) {
                    let at = in_place_of_sun(progress, size, term_width, horizon_y);
                    assert_in_sky(at, size, (term_width, term_height));
                }
            }
        }
    }

    #[test]
    fn test_sky_sprites_stay_in_the_sky_by_night() {
        for size in SIZES {
            for (term_width, term_height) in TERMINALS {
                let horizon_y = term_height - WorldScene::GROUND_HEIGHT;
                let mut moon = MoonSystem::new(term_width, term_height);
                moon.update(term_width, term_height);
                let at = in_place_of_moon(moon.position(), size, term_width, horizon_y);
                assert_in_sky(at, size, (term_width, term_height));
            }
        }
    }

    #[test]
    fn test_sky_sprite_takes_the_moon_place_when_it_fits() {
        let mut moon = MoonSystem::new(120, 40);
        moon.update(120, 40);
        let size = (GiantCrepe::WIDTH, GiantCrepe::HEIGHT);
        assert_eq!(
            in_place_of_moon(moon.position(), size, 120, 40 - WorldScene::GROUND_HEIGHT),
            moon.position()
        );
    }

    #[test]
    fn test_short_sky_fits_no_sprite() {
        assert!(fits(5, CloudBank::TOP + 5));
        assert!(!fits(5, CloudBank::TOP + 4));
    }
}
