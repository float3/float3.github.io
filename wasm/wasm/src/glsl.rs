use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    fn downloadFile(name: &str, contents: &str);
    fn downloadImage(name: &str, contents: &str);
    fn reset();
}

#[wasm_bindgen]
pub fn transpile(input: String, extract_props: bool, raymarch: bool) -> String {
    use glsl2hlsl::ShaderType;

    glsl2hlsl::transpile(
        input,
        extract_props,
        raymarch,
        ShaderType::MainImage("main".to_string(), None, vec![]),
    )
}

#[wasm_bindgen]
pub fn download(json: String, extract_props: bool, raymarch: bool) {
    use glsl2hlsl::get_files;
    use glsl2hlsl::get_image_files;
    use glsl2hlsl::make_shader;

    let shader = make_shader(&json).unwrap();
    let files = get_files(&shader, extract_props, raymarch);
    let images = get_image_files(&shader);
    reset();
    for f in files.iter() {
        downloadFile(&f.name, &f.contents);
    }
    for f in images.iter() {
        downloadImage(&f.name, &f.contents);
    }
}
