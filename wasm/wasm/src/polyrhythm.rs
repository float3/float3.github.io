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
        if let Some(prev_id) = *id.borrow() {
            window
                .cancel_animation_frame(prev_id)
                .expect("Failed to cancel previous frame");
        }
        *id.borrow_mut() = None;
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
    let poly = Polyrhythm::new_with_time_signature(base, tempo, subs.as_slice())
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

    // For each voice, track the last triggered beat (None initially).
    let beat_tracker = Rc::new(RefCell::new(vec![None; poly_rc.components.len()]));

    // Create an Rc for the animation closure.
    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let f_clone = f.clone();

    let start_time = js_sys::Date::now();
    let window_clone = window.clone();

    *f.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let current_time = js_sys::Date::now();
        let elapsed = (current_time - start_time) / 1000.0; // seconds
        let t_phase = (elapsed % measure_duration) / measure_duration;

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

        // Reset beat tracking at the start of each measure.
        if t_phase < 0.05 {
            let mut tracker = beat_tracker.borrow_mut();
            for trig in tracker.iter_mut() {
                *trig = None;
            }
        }

        // For each voice in the polyrhythm.
        for (i, &sides) in poly_rc.components.iter().enumerate() {
            let radius = 40.0 + (i * 2) as f64 * 30.0;
            let vertices = compute_polygon_vertices(center_x, center_y, radius, sides as usize);
            ctx1.begin_path();
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
            let pos = compute_position_along_polygon(&vertices, t_phase);
            ctx1.begin_path();
            ctx1.arc(pos.0, pos.1, 5.0, 0.0, std::f64::consts::PI * 2.0)
                .expect("Failed to draw moving point");
            ctx1.fill();

            // --- Beat Counting Visualization ---
            let beat_disp = ((t_phase * sides as f64).floor() as u32) + 1;
            ctx0.set_font("20px sans-serif");
            ctx0.set_fill_style(&"blue".into());
            ctx0.fill_text(
                &beat_disp.to_string(),
                center_x - 10.0,
                center_y - (70.0 + i as f64 * 30.0) - 10.0,
            )
            .expect("Failed to draw beat number");

            // --- Trigger beep per beat for this voice ---
            let vertex_interval = measure_duration / (sides as f64);
            let measure_elapsed = elapsed % measure_duration;
            let measure_start_time = current_time - (measure_elapsed * 1000.0);
            // Compute current beat index (0-indexed).
            let beat_index = (t_phase * sides as f64).floor() as u32;
            // Compute the scheduled start time for this beat.
            let beat_start_time =
                measure_start_time + (beat_index as f64 * vertex_interval * 1000.0);
            let delay = (beat_start_time - current_time) as i32;
            let freq = pitch as f32 + (i as f32 * 20.0);

            // Check if this beat hasn't been triggered yet.
            if beat_tracker.borrow()[i] != Some(beat_index) {
                let tracker_clone = beat_tracker.clone();
                let window_inner = window_clone.clone();
                let i_clone = i;
                // Closure to trigger the beep and update tracker.
                let beep_closure = Closure::wrap(Box::new(move || {
                    play_beep(freq, 0.1);
                    tracker_clone.borrow_mut()[i_clone] = Some(beat_index);
                }) as Box<dyn FnMut()>);

                if delay > 0 {
                    window_inner
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            beep_closure.as_ref().unchecked_ref(),
                            delay,
                        )
                        .expect("Failed to schedule beep");
                } else {
                    // If we're already past the scheduled time, trigger immediately.
                    play_beep(freq, 0.1);
                    beat_tracker.borrow_mut()[i] = Some(beat_index);
                }
                beep_closure.forget();
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
        if let Some(frame_id) = *id.borrow() {
            window()
                .unwrap()
                .cancel_animation_frame(frame_id)
                .expect("Failed to cancel frame");
            *id.borrow_mut() = None;
        }
    });
}
