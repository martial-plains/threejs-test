use std::sync::Arc;

use gloo::utils::format::JsValueSerdeExt;
use leptos::*;
use serde_json::json;
use threejs_rs::{
    BoxGeometry, HemisphereLight, Mesh, MeshBasicMaterial, PerspectiveCamera, Scene, WebGLRenderer,
};
use wasm_bindgen::JsValue;

#[component]
fn ThreeJsTest(cx: Scope) -> impl IntoView {
    let scene = create_rw_signal(cx, Arc::new(Scene::new()));
    let window = window();
    let camera = create_rw_signal(
        cx,
        Arc::new(PerspectiveCamera::new_with(
            75.0,
            window.inner_width().unwrap().as_f64().unwrap()
                / window.inner_height().unwrap().as_f64().unwrap(),
            0.1,
            1000.0,
        )),
    );

    let renderer = create_rw_signal(cx, Arc::new(WebGLRenderer::new()));
    renderer.update(|renderer| {
        renderer.set_size(
            &window.inner_width().unwrap(),
            &window.inner_height().unwrap(),
            true,
        );
    });

    document()
        .body()
        .unwrap()
        .append_child(&renderer().domElement())
        .unwrap();

    let geometry = BoxGeometry::new(1.0, 1.0, 1.0, 1.0, 1.0, 1.0);
    let material = MeshBasicMaterial::new_with(
        <::wasm_bindgen::JsValue as JsValueSerdeExt>::from_serde(&json!({
            "color": 0xd8672d
        }))
        .unwrap(),
    );
    let cube = create_rw_signal(cx, Arc::new(Mesh::new_with(&geometry, &material)));

    let light = create_rw_signal(
        cx,
        Arc::new(HemisphereLight::new(
            &JsValue::from(0x404040),
            &JsValue::from(0xFFFFFF),
            1,
        )),
    );
    scene().add(&light());

    scene().add(&cube());

    camera().position().setZ(5.0);

    create_effect(cx, move |_| {
        request_animation_frame(move || {
            cube.update(|cube| {
                cube.rotation().set_x(cube.rotation().x() + 0.01);
                cube.rotation().set_y(cube.rotation().y() + 0.01);
            });

            renderer().render(&scene(), &camera());
        });

        cube()
    });

    view! { cx,
        <></>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    _ = console_log::init_with_level(log::Level::Info);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| view! { cx,  <ThreeJsTest/> })
}
