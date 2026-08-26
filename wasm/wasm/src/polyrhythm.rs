use music21_rs::polyrhythm::Polyrhythm;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, js_sys, window};

#[wasm_bindgen]
extern "C" {
    fn play_beep(frequency: f32, duration: f32);
}

// Global to track current animation frame id.
thread_local! {
    static CURRENT_ANIMATION: RefCell<Option<i32>> = const { RefCell::new(None) };
}

// Helper: Compute vertices of a regular polygon.
fn compute_polygon_vertices(cx: f64, cy: f64, radius: f64, sides: usize) -> Vec<(f64, f64)> {
    let angle_offset = -std::f64::consts::PI / 2.0;
    let mut vertices: Vec<(f64, f64)> = (0..sides)
        .map(|i| {
            let angle = angle_offset + 2.0 * std::f64::consts::PI * i as f64 / sides as f64;
            (cx + radius * angle.cos(), cy + radius * angle.sin())
        })
        .collect();

    if sides == 2 {
        vertices[1] = (vertices[1].0, vertices[1].1 - 25.0);
    }
    vertices
}

// Helper: Given polygon vertices and fraction t (0.0 to 1.0),
// compute the moving point's position along the polygon perimeter.
fn compute_position_along_polygon(vertices: &[(f64, f64)], t: f64) -> (f64, f64) {
    let n = vertices.len();
    let (x0, y0) = vertices[0];
    let (x1, y1) = vertices[1 % n];
    let side_length = ((x1 - x0).powi(2) + (y1 - y0).powi(2)).sqrt();
    let total_length = side_length * n as f64;
    let distance = t * total_length;
    let edge_index = (distance / side_length).floor() as usize;
    let edge_progress = (distance % side_length) / side_length;
    let (sx, sy) = vertices[edge_index % n];
    let (ex, ey) = vertices[(edge_index + 1) % n];
    (
        sx + (ex - sx) * edge_progress,
        sy + (ey - sy) * edge_progress,
    )
}

fn voice_color(index: usize) -> &'static str {
    const COLORS: [&str; 8] = [
        "#4f8cff", "#ffbc3f", "#fe7fb3", "#61d394", "#dd6fff", "#4dd0e1", "#f87171", "#a3e635",
    ];
    COLORS[index % COLORS.len()]
}

fn voice_frequency(base_pitch: u32, index: usize) -> f32 {
    const INTERVALS: [i32; 8] = [0, 7, 12, 16, 19, 24, 28, 31];
    let octave_offset = (index / INTERVALS.len()) as i32 * 24;
    let semitones = INTERVALS[index % INTERVALS.len()] + octave_offset;
    let frequency = base_pitch as f32 * 2.0_f32.powf(semitones as f32 / 12.0);
    frequency.min(4_000.0)
}

type MyType = Rc<RefCell<Option<Closure<dyn FnMut()>>>>;

/// Starts the animation with user settings.
/// If a previous instance is running, it is canceled.
#[wasm_bindgen]
pub fn start_with_settings(
    base: u32,
    tempo: u32,
    subdivisions: &str,
    pitch: u32,
) -> Result<(), JsValue> {
    // Cancel any previous animation.
    let window = window().unwrap();
    CURRENT_ANIMATION.with(|id| {
        if let Some(prev_id) = id.borrow_mut().take() {
            window
                .cancel_animation_frame(prev_id)
                .expect("Failed to cancel previous frame");
        }
    });

    // Parse subdivisions string; supports comma or colon separated values.
    let subs: Vec<u32> = subdivisions
        .split([',', ':'])
        .filter_map(|s| s.trim().parse::<u32>().ok())
        .collect();
    if subs.is_empty() {
        return Err(JsValue::from_str("Invalid subdivisions"));
    }

    // Obtain canvas and context.
    let document = window.document().unwrap();
    let canvas1: HtmlCanvasElement = document
        .get_element_by_id("canvas1")
        .expect("Canvas element not found")
        .dyn_into()?;
    let context1 = canvas1
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    let canvas0: HtmlCanvasElement = document
        .get_element_by_id("canvas0")
        .expect("Canvas element not found")
        .dyn_into()?;
    let context0 = canvas0
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    let canvas2: HtmlCanvasElement = document
        .get_element_by_id("canvas2")
        .expect("Canvas element not found")
        .dyn_into()?;
    let context2 = canvas2
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    // Create the polyrhythm.
    let poly = Polyrhythm::from_time_signature(base, tempo, subs.as_slice())
        .expect("Failed to create Polyrhythm");
    let measure_duration = poly
        .measure_duration()
        .expect("Failed to compute measure duration");

    let poly_rc = Rc::new(poly);
    let context0_rc = Rc::new(context0);
    let canvas0_rc = Rc::new(canvas0);
    let context1_rc = Rc::new(context1);
    let canvas1_rc = Rc::new(canvas1);
    let context2_rc = Rc::new(context2);
    let canvas2_rc = Rc::new(canvas2);

    // For each voice, track the last triggered (measure, beat) pair.
    let beat_tracker = Rc::new(RefCell::new(vec![None; poly_rc.components.len()]));

    // Create an Rc for the animation closure.
    let f: MyType = Rc::new(RefCell::new(None));
    let f_clone = f.clone();

    let start_time = js_sys::Date::now();
    let window_clone = window.clone();

    *f.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let current_time = js_sys::Date::now();
        let elapsed = (current_time - start_time) / 1000.0; // seconds
        let t_phase = (elapsed % measure_duration) / measure_duration;
        let measure_index = (elapsed / measure_duration).floor() as u64;

        // Clear canvases.
        let cvs0 = &*canvas0_rc;
        let ctx0 = &*context0_rc;
        let cvs1 = &*canvas1_rc;
        let ctx1 = &*context1_rc;
        let cvs2 = &*canvas2_rc;
        let ctx2 = &*context2_rc;

        ctx0.clear_rect(0.0, 0.0, cvs0.width().into(), cvs0.height().into());
        ctx1.clear_rect(0.0, 0.0, cvs1.width().into(), cvs1.height().into());
        ctx2.clear_rect(0.0, 0.0, cvs2.width().into(), cvs2.height().into());

        let center_x = cvs1.width() as f64 / 2.0;
        let center_y = cvs1.height() as f64 / 2.0;

        // For each voice in the polyrhythm.
        for (i, &sides) in poly_rc.components.iter().enumerate() {
            let color = voice_color(i);
            let radius = 40.0 + (i * 2) as f64 * 30.0;
            let vertices = compute_polygon_vertices(center_x, center_y, radius, sides as usize);
            ctx1.begin_path();
            ctx1.set_stroke_style_str(color);
            ctx1.set_line_width(2.0);
            if let Some(&(x, y)) = vertices.first() {
                ctx1.move_to(x, y);
                for &(vx, vy) in vertices.iter().skip(1) {
                    ctx1.line_to(vx, vy);
                }
                if sides > 2 {
                    ctx1.close_path();
                }
                ctx1.stroke();
            }
            ctx1.set_fill_style_str(color);
            for &(vx, vy) in &vertices {
                ctx1.begin_path();
                ctx1.arc(vx, vy, 3.0, 0.0, std::f64::consts::PI * 2.0)
                    .expect("Failed to draw polygon vertex");
                ctx1.fill();
            }
            let pos = compute_position_along_polygon(&vertices, t_phase);
            ctx1.begin_path();
            ctx1.arc(pos.0, pos.1, 6.0, 0.0, std::f64::consts::PI * 2.0)
                .expect("Failed to draw moving point");
            ctx1.fill();

            // --- Beat Counting Visualization ---
            let beat_index = ((t_phase * sides as f64).floor() as u32).min(sides.saturating_sub(1));
            let beat_disp = beat_index + 1;
            ctx0.set_font("18px sans-serif");
            ctx0.set_fill_style_str(color);
            ctx0.fill_text(
                &format!("{} hits: {}", sides, beat_disp),
                24.0,
                30.0 + i as f64 * 26.0,
            )
            .expect("Failed to draw beat number");

            // --- Timeline visualization ---
            let timeline_left = 26.0;
            let timeline_width = cvs2.width() as f64 - 110.0;
            let timeline_y = 34.0 + i as f64 * 28.0;
            ctx2.set_stroke_style_str("#667085");
            ctx2.set_line_width(1.0);
            ctx2.begin_path();
            ctx2.move_to(timeline_left, timeline_y);
            ctx2.line_to(timeline_left + timeline_width, timeline_y);
            ctx2.stroke();

            for beat in 0..sides {
                let x = timeline_left + timeline_width * beat as f64 / sides as f64;
                ctx2.begin_path();
                ctx2.set_fill_style_str(if beat == beat_index { color } else { "#667085" });
                ctx2.arc(
                    x,
                    timeline_y,
                    if beat == beat_index { 5.0 } else { 3.0 },
                    0.0,
                    std::f64::consts::PI * 2.0,
                )
                .expect("Failed to draw timeline beat");
                ctx2.fill();
            }
            ctx2.set_font("14px sans-serif");
            ctx2.set_fill_style_str(color);
            ctx2.fill_text(
                &format!("voice {}", i + 1),
                timeline_left + timeline_width + 14.0,
                timeline_y + 5.0,
            )
            .expect("Failed to draw timeline label");

            // --- Trigger beep per beat for this voice ---
            let freq = voice_frequency(pitch, i);
            let beat_key = (measure_index, beat_index);

            if beat_tracker.borrow()[i] != Some(beat_key) {
                play_beep(freq, if beat_index == 0 { 0.13 } else { 0.09 });
                beat_tracker.borrow_mut()[i] = Some(beat_key);
            }
        }

        // Schedule the next frame.
        let frame_id = window_clone
            .request_animation_frame(f_clone.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .expect("Failed to schedule frame");
        CURRENT_ANIMATION.with(|id| {
            *id.borrow_mut() = Some(frame_id);
        });
    }) as Box<dyn FnMut()>));

    // Start the animation loop.
    let initial_id = window
        .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
        .expect("Failed to schedule initial frame");
    CURRENT_ANIMATION.with(|id| {
        *id.borrow_mut() = Some(initial_id);
    });

    Ok(())
}

#[wasm_bindgen]
pub fn stop() {
    CURRENT_ANIMATION.with(|id| {
        if let Some(frame_id) = id.borrow_mut().take() {
            window()
                .unwrap()
                .cancel_animation_frame(frame_id)
                .expect("Failed to cancel frame");
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn voice_frequencies_are_related_to_the_base_pitch() {
        assert!((voice_frequency(440, 0) - 440.0).abs() < 0.001);
        assert!((voice_frequency(440, 1) - 659.255).abs() < 0.01);
        assert!(voice_frequency(440, 3) > voice_frequency(440, 2));
    }

    #[test]
    fn voice_colors_cycle() {
        assert_eq!(voice_color(0), voice_color(8));
    }
}
