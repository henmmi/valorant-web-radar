use crate::components::canvas::{get_number, get_radian_angle, ROTATION_ANGLE};
use crate::components::elements::{get_canvas_context_document, get_html_image_element_by_id};

#[derive(Debug)]
pub struct DeadPlayers {
    pub x: f64,
    pub y: f64,
    pub prevail_count: i32,
}

/// Data container for all dead players
/// # Example
/// ```
/// let mut dead_players = DeadPlayers::new(0.0, 0.0);
/// ```
impl DeadPlayers {
    /// Create a new DeadPlayers object
    /// # Arguments
    /// * `x` - The x coordinate of the dead player
    /// * `y` - The y coordinate of the dead player
    /// # Example
    /// ```
    /// let mut dead_players = DeadPlayers::new(0.0, 0.0);
    /// ```
    pub fn new(x: f64, y: f64) -> Self {
        DeadPlayers {
            x,
            y,
            prevail_count: 5,
        }
    }
    /// Display dead players as Killed icon on the map for a few seconds
    /// # Arguments
    /// * `killed` - The vector of killed players
    /// # Example
    /// ```
    /// DeadPlayers::draw_dead_players(&mut Vec<DeadPlayers>);
    /// ```
    pub fn draw_dead_players(killed: &mut Vec<DeadPlayers>) {
        const ALPHA_FACTOR: f64 = 0.2;
        const DEATH_ICON_SIZE: f64 = 32.0;
        let (_, context, _) = get_canvas_context_document();
        let angle = get_number(&ROTATION_ANGLE);
        for player in killed.iter_mut() {
            context.save();
            context.translate(player.x, player.y).unwrap();
            let angle_rad = get_radian_angle(-angle);
            let death_icon = get_html_image_element_by_id("Killed").unwrap();
            context.rotate(angle_rad).unwrap();
            context.set_global_alpha(ALPHA_FACTOR * player.prevail_count as f64);
            context
                .draw_image_with_html_image_element_and_dw_and_dh(
                    &death_icon,
                    -DEATH_ICON_SIZE / 2.0,
                    -DEATH_ICON_SIZE / 2.0,
                    DEATH_ICON_SIZE,
                    DEATH_ICON_SIZE,
                )
                .unwrap();
            context.restore();
            player.prevail_count -= 1;
        }
        // Retain the killed players that still have a prevail count
        killed.retain(|x| x.prevail_count > 0);
    }
}
