use music21_rs::polyrhythm::Polyrhythm;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::js_sys;
use web_sys::window;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen]
extern "C" {
    fn play_beep(frequency: f32, duration: f32);
}

// Global to track current animation frame id.
thread_local! {
    static CURRENT_ANIMATION: RefCell<Option<i32>> = RefCell::new(None);
}

// Helper: Compute vertices of a regular polygon.
fn compute_polygon_vertices(cx: f64, cy: f64, radius: f64, sides: usize) -> Vec<(f64, f64)> {
    let angle_offset = -std::f64::consts::PI / 2.0;
    (0..sides)
        .map(|i| {
            let angle = angle_offset + 2.0 * std::f64::consts::PI * i as f64 / sides as f64;
            (cx + radius * angle.cos(), cy + radius * angle.sin())
        })
        .collect()
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
    let subs: Vec<u64> = subdivisions
        .split(|c| c == ',' || c == ':')
        .filter_map(|s| s.trim().parse::<u64>().ok())
        .collect();
    if subs.is_empty() {
        return Err(JsValue::from_str("Invalid subdivisions"));
    }

    // Obtain canvas and context.
    let document = window.document().unwrap();
    let canvas: HtmlCanvasElement = document
        .get_element_by_id("canvas")
        .expect("Canvas element not found")
        .dyn_into()?;
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    // Create the polyrhythm.
    let poly = Polyrhythm::new_with_time_signature(base.into(), tempo.into(), subs.as_slice())
        .expect("Failed to create Polyrhythm");
    let measure_duration = poly
        .measure_duration()
        .expect("Failed to compute measure duration");

    let poly_rc = Rc::new(poly);
    let context_rc = Rc::new(context);
    let canvas_rc = Rc::new(canvas);

    // Vector to track if a beep has already been triggered per voice.
    let beep_flags = Rc::new(RefCell::new(vec![false; poly_rc.components.len()]));

    // Create an Rc for the animation closure.
    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let f_clone = f.clone();

    let start_time = js_sys::Date::now();
    let window_clone = window.clone();

    *f.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let current_time = js_sys::Date::now();
        let elapsed = (current_time - start_time) / 1000.0;
        let t_phase = (elapsed % measure_duration) / measure_duration;

        // Clear canvas.
        let canvas = &*canvas_rc;
        let ctx = &*context_rc;
        ctx.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

        let center_x = canvas.width() as f64 / 2.0;
        let center_y = canvas.height() as f64 / 2.0;

        let mut flags = beep_flags.borrow_mut();

        // For each voice, draw its polygon and animate its moving point.
        for (i, &sides) in poly_rc.components.iter().enumerate() {
            let radius = 40.0 + i as f64 * 30.0;
            let vertices = compute_polygon_vertices(center_x, center_y, radius, sides as usize);

            ctx.begin_path();
            if let Some(&(x, y)) = vertices.first() {
                ctx.move_to(x, y);
                for &(vx, vy) in vertices.iter().skip(1) {
                    ctx.line_to(vx, vy);
                }
                ctx.close_path();
                ctx.stroke();
            }

            let pos = compute_position_along_polygon(&vertices, t_phase);
            ctx.begin_path();
            ctx.arc(pos.0, pos.1, 5.0, 0.0, std::f64::consts::PI * 2.0)
                .expect("Failed to draw arc");
            ctx.fill();

            // Trigger beep near a vertex.
            let vertex_interval = 1.0 / (sides as f64);
            let mut near_vertex = false;
            for k in 0..sides {
                let vertex_phase = k as f64 * vertex_interval;
                if (t_phase - vertex_phase).abs() < 0.02 {
                    near_vertex = true;
                    break;
                }
            }
            if near_vertex && !flags[i] {
                let freq = pitch as f32 + (i as f32 * 20.0);
                play_beep(freq, 0.1);
                flags[i] = true;
            }
            if !near_vertex {
                flags[i] = false;
            }
        }

        // Schedule the next frame and store the new frame id.
        let frame_id = window_clone
            .request_animation_frame(f_clone.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .expect("Failed to schedule frame");
        CURRENT_ANIMATION.with(|id| {
            *id.borrow_mut() = Some(frame_id);
        });
    }) as Box<dyn FnMut()>));

    // Start the animation loop and store its id.
    let initial_id = window
        .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
        .expect("Failed to schedule initial frame");
    CURRENT_ANIMATION.with(|id| {
        *id.borrow_mut() = Some(initial_id);
    });

    Ok(())
}
