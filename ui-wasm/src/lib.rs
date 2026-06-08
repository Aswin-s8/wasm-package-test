use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

fn get_document() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

#[wasm_bindgen]
pub fn render_buttons(parent_id: &str) {
    let doc = get_document();
    let parent = match doc.get_element_by_id(parent_id) {
        Some(el) => el,
        None => return,
    };
    
    // Clear parent contents
    parent.set_inner_html("");

    // Create flex container
    let container = doc.create_element("div").unwrap();
    let container_html = container.dyn_into::<web_sys::HtmlElement>().unwrap();
    let container_style = container_html.style();
    let _ = container_style.set_property("display", "flex");
    let _ = container_style.set_property("gap", "1.5rem");
    let _ = container_style.set_property("justify-content", "center");
    let _ = container_style.set_property("margin-top", "2rem");
    let _ = container_style.set_property("margin-bottom", "2rem");
    
    // ── BUTTON 1: TELEMETRY ──
    let btn1 = doc.create_element("button").unwrap();
    let btn1_html = btn1.clone().dyn_into::<web_sys::HtmlElement>().unwrap();
    btn1_html.set_inner_text("Enable Telemetry");
    let style1 = btn1_html.style();
    set_button_base_style(&style1, "#22d3ee", "rgba(34, 211, 238, 0.08)");

    // Hover effect closure
    {
        let btn_clone = btn1.clone();
        let mouseenter = Closure::wrap(Box::new(move || {
            let el = btn_clone.clone().dyn_into::<web_sys::HtmlElement>().unwrap();
            let style = el.style();
            let _ = style.set_property("background-color", "#22d3ee");
            let _ = style.set_property("color", "#060913");
            let _ = style.set_property("box-shadow", "0 0 20px rgba(34, 211, 238, 0.5)");
        }) as Box<dyn FnMut()>);
        btn1.add_event_listener_with_callback("mouseenter", mouseenter.as_ref().unchecked_ref()).unwrap();
        mouseenter.forget();
    }
    {
        let btn_clone = btn1.clone();
        let mouseleave = Closure::wrap(Box::new(move || {
            let el = btn_clone.clone().dyn_into::<web_sys::HtmlElement>().unwrap();
            let style = el.style();
            let _ = style.set_property("background-color", "rgba(34, 211, 238, 0.08)");
            let _ = style.set_property("color", "#22d3ee");
            let _ = style.set_property("box-shadow", "0 0 10px rgba(34, 211, 238, 0.15)");
        }) as Box<dyn FnMut()>);
        btn1.add_event_listener_with_callback("mouseleave", mouseleave.as_ref().unchecked_ref()).unwrap();
        mouseleave.forget();
    }
    // Click callback using inline custom alert or global alert
    {
        let click_cb = Closure::wrap(Box::new(move || {
            web_sys::window().unwrap().alert_with_message("Telemetry systems initiated successfully via WASM UI!").unwrap();
        }) as Box<dyn FnMut()>);
        btn1.add_event_listener_with_callback("click", click_cb.as_ref().unchecked_ref()).unwrap();
        click_cb.forget();
    }

    // ── BUTTON 2: TRACK MODE ──
    let btn2 = doc.create_element("button").unwrap();
    let btn2_html = btn2.clone().dyn_into::<web_sys::HtmlElement>().unwrap();
    btn2_html.set_inner_text("Activate Track Mode");
    let style2 = btn2_html.style();
    set_button_base_style(&style2, "#f43f5e", "rgba(244, 63, 94, 0.08)");

    // Hover effect closure
    {
        let btn_clone = btn2.clone();
        let mouseenter = Closure::wrap(Box::new(move || {
            let el = btn_clone.clone().dyn_into::<web_sys::HtmlElement>().unwrap();
            let style = el.style();
            let _ = style.set_property("background-color", "#f43f5e");
            let _ = style.set_property("color", "#060913");
            let _ = style.set_property("box-shadow", "0 0 20px rgba(244, 63, 94, 0.5)");
        }) as Box<dyn FnMut()>);
        btn2.add_event_listener_with_callback("mouseenter", mouseenter.as_ref().unchecked_ref()).unwrap();
        mouseenter.forget();
    }
    {
        let btn_clone = btn2.clone();
        let mouseleave = Closure::wrap(Box::new(move || {
            let el = btn_clone.clone().dyn_into::<web_sys::HtmlElement>().unwrap();
            let style = el.style();
            let _ = style.set_property("background-color", "rgba(244, 63, 94, 0.08)");
            let _ = style.set_property("color", "#f43f5e");
            let _ = style.set_property("box-shadow", "0 0 10px rgba(244, 63, 94, 0.15)");
        }) as Box<dyn FnMut()>);
        btn2.add_event_listener_with_callback("mouseleave", mouseleave.as_ref().unchecked_ref()).unwrap();
        mouseleave.forget();
    }
    // Click callback
    {
        let click_cb = Closure::wrap(Box::new(move || {
            web_sys::window().unwrap().alert_with_message("Track Mode activated! Engine remapped & suspension stiffened!").unwrap();
        }) as Box<dyn FnMut()>);
        btn2.add_event_listener_with_callback("click", click_cb.as_ref().unchecked_ref()).unwrap();
        click_cb.forget();
    }

    // Append to container, then append to parent
    let _ = container_html.append_child(&btn1);
    let _ = container_html.append_child(&btn2);
    let _ = parent.append_child(&container_html);
}

fn set_button_base_style(style: &web_sys::CssStyleDeclaration, accent_hex: &str, bg_rgba: &str) {
    let _ = style.set_property("background-color", bg_rgba);
    let _ = style.set_property("border", &format!("1px solid {}", accent_hex));
    let _ = style.set_property("color", accent_hex);
    let _ = style.set_property("font-family", "'Syne', sans-serif");
    let _ = style.set_property("text-transform", "uppercase");
    let _ = style.set_property("letter-spacing", "0.08em");
    let _ = style.set_property("font-size", "0.8rem");
    let _ = style.set_property("font-weight", "600");
    let _ = style.set_property("padding", "0.75rem 1.5rem");
    let _ = style.set_property("border-radius", "8px");
    let _ = style.set_property("cursor", "pointer");
    let _ = style.set_property("outline", "none");
    let _ = style.set_property("box-shadow", &format!("0 0 10px {}", bg_rgba));
    let _ = style.set_property("transition", "all 0.3s cubic-bezier(0.16, 1, 0.3, 1)");
}
