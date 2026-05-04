use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn graph_clamp_zoom(value: f64, min: f64, max: f64) -> f64 {
    if !value.is_finite() {
        return 1.0;
    }

    (value.clamp(min, max) * 100.0).round() / 100.0
}

#[wasm_bindgen]
pub fn graph_wheel_delta_pixels(delta_mode: u32, delta: f64, page_pixels: f64) -> f64 {
    match delta_mode {
        1 => delta * 16.0,
        2 => delta * page_pixels,
        _ => delta,
    }
}

#[wasm_bindgen]
pub fn graph_link_path(from_x: f64, from_y: f64, to_x: f64, to_y: f64) -> String {
    let tension = 80.0_f64.max((to_x - from_x).abs() * 0.55);
    format!(
        "M {from_x} {from_y} C {} {from_y}, {} {to_y}, {to_x} {to_y}",
        from_x + tension,
        to_x - tension
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamps_zoom() {
        assert_eq!(graph_clamp_zoom(0.333, 0.5, 2.0), 0.5);
        assert_eq!(graph_clamp_zoom(1.234, 0.5, 2.0), 1.23);
        assert_eq!(graph_clamp_zoom(3.0, 0.5, 2.0), 2.0);
    }

    #[test]
    fn converts_wheel_delta_modes() {
        assert_eq!(graph_wheel_delta_pixels(0, 3.0, 100.0), 3.0);
        assert_eq!(graph_wheel_delta_pixels(1, 3.0, 100.0), 48.0);
        assert_eq!(graph_wheel_delta_pixels(2, 3.0, 100.0), 300.0);
    }
}
