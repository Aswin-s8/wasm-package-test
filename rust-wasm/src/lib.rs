use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::cell::RefCell;

#[derive(Clone, Debug)]
pub struct AppState {
    pub accent: String,
    pub brand_name: String,
    pub brand_tagline: String,
    pub nav_1: String,
    pub nav_2: String,
    pub nav_3: String,
    pub hero_title: String,
    pub hero_subtitle: String,
    pub hero_cta: String,
    pub lbl_loc: String,
    pub lbl_start: String,
    pub lbl_end: String,
    // Cars
    pub car1_name: String,
    pub car1_price: String,
    pub car1_060: String,
    pub car1_top: String,
    pub car1_hp: String,
    pub car1_cat: String,
    pub car1_img: String,
    pub car2_name: String,
    pub car2_price: String,
    pub car2_060: String,
    pub car2_top: String,
    pub car2_hp: String,
    pub car2_cat: String,
    pub car2_img: String,
    pub car3_name: String,
    pub car3_price: String,
    pub car3_060: String,
    pub car3_top: String,
    pub car3_hp: String,
    pub car3_cat: String,
    pub car3_img: String,
    // UI
    pub glow: String,
    pub blur: String,
    pub radius: String,
    pub grid: bool,
    pub font: String,
    // Experiences
    pub exp1_ico: String,
    pub exp1_title: String,
    pub exp1_desc: String,
    pub exp2_ico: String,
    pub exp2_title: String,
    pub exp2_desc: String,
    pub exp3_ico: String,
    pub exp3_title: String,
    pub exp3_desc: String,
}

impl Default for AppState {
    fn default() -> Self {
        get_preset("cyber")
    }
}

thread_local! {
    static STATE: RefCell<AppState> = RefCell::new(AppState::default());
}

fn get_document() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

fn set_text(id: &str, text: &str) {
    if let Some(el) = get_document().get_element_by_id(id) {
        if let Ok(html_el) = el.dyn_into::<web_sys::HtmlElement>() {
            html_el.set_inner_text(text);
        }
    }
}

fn set_input_value(id: &str, val: &str) {
    if let Some(el) = get_document().get_element_by_id(id) {
        if let Ok(input_el) = el.clone().dyn_into::<web_sys::HtmlInputElement>() {
            input_el.set_value(val);
        } else if let Ok(textarea_el) = el.clone().dyn_into::<web_sys::HtmlTextAreaElement>() {
            textarea_el.set_value(val);
        } else if let Ok(select_el) = el.dyn_into::<web_sys::HtmlSelectElement>() {
            select_el.set_value(val);
        }
    }
}

fn get_input_value(id: &str) -> String {
    if let Some(el) = get_document().get_element_by_id(id) {
        if let Ok(input_el) = el.clone().dyn_into::<web_sys::HtmlInputElement>() {
            input_el.value()
        } else if let Ok(textarea_el) = el.clone().dyn_into::<web_sys::HtmlTextAreaElement>() {
            textarea_el.value()
        } else if let Ok(select_el) = el.dyn_into::<web_sys::HtmlSelectElement>() {
            select_el.value()
        } else {
            "".to_string()
        }
    } else {
        "".to_string()
    }
}

fn set_img_src(id: &str, src: &str) {
    if let Some(el) = get_document().get_element_by_id(id) {
        let _ = el.set_attribute("src", src);
    }
}

fn set_class_active(id: &str, active: bool) {
    if let Some(el) = get_document().get_element_by_id(id) {
        if active {
            let _ = el.class_list().add_1("active");
        } else {
            let _ = el.class_list().remove_1("active");
        }
    }
}

fn apply_accent_styles(hex: &str) {
    let doc_el = get_document().document_element().unwrap();
    if let Ok(style_el) = doc_el.dyn_into::<web_sys::HtmlElement>() {
        let css = style_el.style();
        let _ = css.set_property("--brand-accent", hex);
        
        // Convert hex to rgb
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        if hex.len() == 7 && hex.starts_with('#') {
            if let Ok(r_val) = u8::from_str_radix(&hex[1..3], 16) {
                r = r_val;
            }
            if let Ok(g_val) = u8::from_str_radix(&hex[3..5], 16) {
                g = g_val;
            }
            if let Ok(b_val) = u8::from_str_radix(&hex[5..7], 16) {
                b = b_val;
            }
        }
        let rgb_str = format!("{}, {}, {}", r, g, b);
        let _ = css.set_property("--brand-accent-rgb", &rgb_str);
    }
}

fn apply_font_styles(font: &str) {
    let doc_el = get_document().document_element().unwrap();
    if let Ok(style_el) = doc_el.dyn_into::<web_sys::HtmlElement>() {
        let _ = style_el.style().set_property("--page-font", font);
    }
}

fn escape_html(str: &str) -> String {
    str.replace('&', "&amp;")
       .replace('<', "&lt;")
       .replace('>', "&gt;")
       .replace('"', "&quot;")
}

fn format_price(val: f64) -> String {
    let s = format!("{:.2}", val);
    let parts: Vec<&str> = s.split('.').collect();
    let int_part = parts[0];
    let dec_part = parts[1];
    
    let mut result = String::new();
    let len = int_part.len();
    for (i, ch) in int_part.chars().enumerate() {
        result.push(ch);
        let remaining = len - i - 1;
        if remaining > 0 && remaining % 3 == 0 {
            result.push(',');
        }
    }
    format!("${}.{}", result, dec_part)
}

fn generate_yaml_preview(state: &AppState) -> String {
    format!(
r#"<span class="k-cmt"># File: c:/cortex/registry/eigenvalues/eigen_config.yaml</span>
<span class="k-cmt"># Serialized Eigenstate vector document reflecting live portal adjustments</span>
<span class="k-key">eigenstate_configuration</span>:
  <span class="k-key">brand_vector_a1</span>:
    <span class="k-key">name</span>:        <span class="k-leaf">"{}"</span>
    <span class="k-key">tagline</span>:     <span class="k-leaf">"{}"</span>
    <span class="k-key">accent</span>:      <span class="k-leaf">"{}"</span>
    <span class="k-key">nav_labels</span>:  <span class="k-leaf">[ "{}", "{}", "{}" ]</span>
  <span class="k-key">hero_vector_a2</span>:
    <span class="k-key">headline</span>:    <span class="k-leaf">"{}"</span>
    <span class="k-key">cta_label</span>:   <span class="k-leaf">"{}"</span>
    <span class="k-key">search_labels</span>: <span class="k-leaf">[ "{}", "{}", "{}" ]</span>
  <span class="k-key">fleet_vector_a3</span>:
    - <span class="k-key">model</span>: <span class="k-leaf">"{}"</span>
      <span class="k-key">rate</span>:  <span class="k-leaf">"{}"</span>
      <span class="k-key">zero_to_sixty</span>: <span class="k-leaf">"{}"</span>
      <span class="k-key">image_url</span>:     <span class="k-leaf">"{}"</span>
    - <span class="k-key">model</span>: <span class="k-leaf">"{}"</span>
      <span class="k-key">rate</span>:  <span class="k-leaf">"{}"</span>
      <span class="k-key">zero_to_sixty</span>: <span class="k-leaf">"{}"</span>
      <span class="k-key">image_url</span>:     <span class="k-leaf">"{}"</span>
    - <span class="k-key">model</span>: <span class="k-leaf">"{}"</span>
      <span class="k-key">rate</span>:  <span class="k-leaf">"{}"</span>
      <span class="k-key">zero_to_sixty</span>: <span class="k-leaf">"{}"</span>
      <span class="k-key">image_url</span>:     <span class="k-leaf">"{}"</span>
  <span class="k-key">ui_vector_a4</span>:
    <span class="k-key">font_style</span>: <span class="k-leaf">"{}"</span>
    <span class="k-key">glow_px</span>:    <span class="k-leaf">{}</span>
    <span class="k-key">blur_px</span>:    <span class="k-leaf">{}</span>
    <span class="k-key">radius_px</span>:  <span class="k-leaf">{}</span>
    <span class="k-key">grid</span>:       <span class="k-leaf">{}</span>
  <span class="k-key">logistics_vector_a5</span>:
    - <span class="k-key">icon</span>:  <span class="k-leaf">"{}"</span>
      <span class="k-key">title</span>: <span class="k-leaf">"{}"</span>
    - <span class="k-key">icon</span>:  <span class="k-leaf">"{}"</span>
      <span class="k-key">title</span>: <span class="k-leaf">"{}"</span>
    - <span class="k-key">icon</span>:  <span class="k-leaf">"{}"</span>
      <span class="k-key">title</span>: <span class="k-leaf">"{}"</span>"#,
        escape_html(&state.brand_name),
        escape_html(&state.brand_tagline),
        escape_html(&state.accent),
        escape_html(&state.nav_1),
        escape_html(&state.nav_2),
        escape_html(&state.nav_3),
        escape_html(&state.hero_title),
        escape_html(&state.hero_cta),
        escape_html(&state.lbl_loc),
        escape_html(&state.lbl_start),
        escape_html(&state.lbl_end),
        escape_html(&state.car1_name),
        escape_html(&state.car1_price),
        escape_html(&state.car1_060),
        escape_html(&state.car1_img),
        escape_html(&state.car2_name),
        escape_html(&state.car2_price),
        escape_html(&state.car2_060),
        escape_html(&state.car2_img),
        escape_html(&state.car3_name),
        escape_html(&state.car3_price),
        escape_html(&state.car3_060),
        escape_html(&state.car3_img),
        escape_html(&state.font),
        state.glow,
        state.blur,
        state.radius,
        state.grid,
        escape_html(&state.exp1_ico),
        escape_html(&state.exp1_title),
        escape_html(&state.exp2_ico),
        escape_html(&state.exp2_title),
        escape_html(&state.exp3_ico),
        escape_html(&state.exp3_title),
    )
}

fn update_dom(state: &AppState) {
    set_text("logo-text", &state.brand_name);
    set_text("nav-item-1", &state.nav_1);
    set_text("nav-item-2", &state.nav_2);
    set_text("nav-item-3", &state.nav_3);
    
    set_text("hero-tagline", &state.brand_tagline);
    set_text("hero-title", &state.hero_title);
    set_text("hero-subtitle", &state.hero_subtitle);
    set_text("hero-cta", &state.hero_cta);
    set_text("lbl-search-loc", &state.lbl_loc);
    set_text("lbl-search-start", &state.lbl_start);
    set_text("lbl-search-end", &state.lbl_end);

    set_text("car-name-1", &state.car1_name);
    set_text("car-price-1", &format!("${}", state.car1_price));
    set_text("car-060-1", &state.car1_060);
    set_text("car-top-1", &state.car1_top);
    set_text("car-hp-1", &state.car1_hp);
    set_text("car-cat-1", &state.car1_cat);
    set_img_src("car-img-1", &state.car1_img);

    set_text("car-name-2", &state.car2_name);
    set_text("car-price-2", &format!("${}", state.car2_price));
    set_text("car-060-2", &state.car2_060);
    set_text("car-top-2", &state.car2_top);
    set_text("car-hp-2", &state.car2_hp);
    set_text("car-cat-2", &state.car2_cat);
    set_img_src("car-img-2", &state.car2_img);

    set_text("car-name-3", &state.car3_name);
    set_text("car-price-3", &format!("${}", state.car3_price));
    set_text("car-060-3", &state.car3_060);
    set_text("car-top-3", &state.car3_top);
    set_text("car-hp-3", &state.car3_hp);
    set_text("car-cat-3", &state.car3_cat);
    set_img_src("car-img-3", &state.car3_img);

    set_text("exp-icon-1", &state.exp1_ico);
    set_text("exp-title-1", &state.exp1_title);
    set_text("exp-desc-1", &state.exp1_desc);

    set_text("exp-icon-2", &state.exp2_ico);
    set_text("exp-title-2", &state.exp2_title);
    set_text("exp-desc-2", &state.exp2_desc);

    set_text("exp-icon-3", &state.exp3_ico);
    set_text("exp-title-3", &state.exp3_title);
    set_text("exp-desc-3", &state.exp3_desc);

    set_text("footer-logo", &format!("{} · Exotic & Performance Car Rentals", state.brand_name));

    if let Some(el) = get_document().get_element_by_id("code-block") {
        el.set_inner_html(&generate_yaml_preview(state));
    }
}

fn is_match(state: &AppState, preset: &AppState) -> bool {
    state.accent == preset.accent &&
    state.brand_name == preset.brand_name &&
    state.brand_tagline == preset.brand_tagline &&
    state.nav_1 == preset.nav_1 &&
    state.nav_2 == preset.nav_2 &&
    state.nav_3 == preset.nav_3 &&
    state.hero_title == preset.hero_title &&
    state.hero_subtitle == preset.hero_subtitle &&
    state.hero_cta == preset.hero_cta &&
    state.lbl_loc == preset.lbl_loc &&
    state.lbl_start == preset.lbl_start &&
    state.lbl_end == preset.lbl_end &&
    state.car1_name == preset.car1_name &&
    state.car1_price == preset.car1_price &&
    state.car1_060 == preset.car1_060 &&
    state.car1_top == preset.car1_top &&
    state.car1_hp == preset.car1_hp &&
    state.car1_cat == preset.car1_cat &&
    state.car1_img == preset.car1_img &&
    state.car2_name == preset.car2_name &&
    state.car2_price == preset.car2_price &&
    state.car2_060 == preset.car2_060 &&
    state.car2_top == preset.car2_top &&
    state.car2_hp == preset.car2_hp &&
    state.car2_cat == preset.car2_cat &&
    state.car2_img == preset.car2_img &&
    state.car3_name == preset.car3_name &&
    state.car3_price == preset.car3_price &&
    state.car3_060 == preset.car3_060 &&
    state.car3_top == preset.car3_top &&
    state.car3_hp == preset.car3_hp &&
    state.car3_cat == preset.car3_cat &&
    state.car3_img == preset.car3_img &&
    state.glow == preset.glow &&
    state.blur == preset.blur &&
    state.radius == preset.radius &&
    state.grid == preset.grid &&
    state.font == preset.font &&
    state.exp1_ico == preset.exp1_ico &&
    state.exp1_title == preset.exp1_title &&
    state.exp1_desc == preset.exp1_desc &&
    state.exp2_ico == preset.exp2_ico &&
    state.exp2_title == preset.exp2_title &&
    state.exp2_desc == preset.exp2_desc &&
    state.exp3_ico == preset.exp3_ico &&
    state.exp3_title == preset.exp3_title &&
    state.exp3_desc == preset.exp3_desc
}

fn check_custom_status(state: &AppState) {
    let cyber = get_preset("cyber");
    let monaco = get_preset("monaco");
    let stealth = get_preset("stealth");

    let match_cyber = is_match(state, &cyber);
    let match_monaco = is_match(state, &monaco);
    let match_stealth = is_match(state, &stealth);

    let doc = get_document();
    if let Ok(btn_cards) = doc.query_selector_all(".btn-preset-card") {
        for i in 0..btn_cards.length() {
            if let Some(node) = btn_cards.get(i) {
                if let Ok(el) = node.dyn_into::<web_sys::Element>() {
                    let _ = el.class_list().remove_1("active");
                }
            }
        }
    }

    if let Some(custom_btn) = doc.get_element_by_id("btn-pre-custom") {
        if let Ok(custom_el) = custom_btn.dyn_into::<web_sys::HtmlElement>() {
            if match_cyber {
                set_class_active("btn-pre-cyber", true);
                let _ = custom_el.style().set_property("display", "none");
            } else if match_monaco {
                set_class_active("btn-pre-monaco", true);
                let _ = custom_el.style().set_property("display", "none");
            } else if match_stealth {
                set_class_active("btn-pre-stealth", true);
                let _ = custom_el.style().set_property("display", "none");
            } else {
                set_class_active("btn-pre-custom", true);
                let _ = custom_el.style().set_property("display", "block");
            }
        }
    }
}

fn get_preset(name: &str) -> AppState {
    match name {
        "monaco" => AppState {
            accent: "#10b981".to_string(),
            brand_name: "RivieraRent".to_string(),
            brand_tagline: "Grand Touring & Luxury Convertibles".to_string(),
            nav_1: "Collection".to_string(),
            nav_2: "Lifestyle".to_string(),
            nav_3: "Concierge".to_string(),
            hero_title: "Chase the Sun in Pure Luxury".to_string(),
            hero_subtitle: "Sophisticated styling, premium leather, and open-top GTs tailored for your Mediterranean cruise.".to_string(),
            hero_cta: "Rent a Convertible".to_string(),
            lbl_loc: "Harbor Marina".to_string(),
            lbl_start: "From Date".to_string(),
            lbl_end: "Until Date".to_string(),
            car1_name: "Monaco GT Spyder".to_string(),
            car1_price: "950".to_string(),
            car1_060: "3.4s".to_string(),
            car1_top: "205mph".to_string(),
            car1_hp: "650 HP".to_string(),
            car1_cat: "Convertible".to_string(),
            car1_img: "https://images.unsplash.com/photo-1583121274602-3e2820c69888?auto=format&fit=crop&w=800&q=80".to_string(),
            car2_name: "Prestige Grand GT".to_string(),
            car2_price: "1100".to_string(),
            car2_060: "4.2s".to_string(),
            car2_top: "195mph".to_string(),
            car2_hp: "700 HP".to_string(),
            car2_cat: "V12 Coupe".to_string(),
            car2_img: "https://images.unsplash.com/photo-1525609004556-c46c7d6cf0a3?auto=format&fit=crop&w=800&q=80".to_string(),
            car3_name: "Riviera Convertible".to_string(),
            car3_price: "800".to_string(),
            car3_060: "3.6s".to_string(),
            car3_top: "200mph".to_string(),
            car3_hp: "600 HP".to_string(),
            car3_cat: "Convertible".to_string(),
            car3_img: "https://images.unsplash.com/photo-1492144534655-ae79c964c9d7?auto=format&fit=crop&w=800&q=80".to_string(),
            glow: "10".to_string(),
            blur: "20".to_string(),
            radius: "16".to_string(),
            grid: false,
            font: "'Inter', sans-serif".to_string(),
            exp1_ico: "🥂".to_string(),
            exp1_title: "VIP Concierge".to_string(),
            exp1_desc: "Receive personalized local recommendations, restaurant reservations, and yacht docking passes hand-delivered by our local staff.".to_string(),
            exp2_ico: "🗝️".to_string(),
            exp2_title: "Villa Delivery".to_string(),
            exp2_desc: "Your convertible is placed in your private garage or driveway prior to your flight landing, with keyless smartphone authorization.".to_string(),
            exp3_ico: "🛥️".to_string(),
            exp3_title: "Yacht Tender".to_string(),
            exp3_desc: "Seamless transfers between Monaco Heliport, our marina docks, and your chartered motor yacht with full luggage transfers.".to_string(),
        },
        "stealth" => AppState {
            accent: "#f43f5e".to_string(),
            brand_name: "ApexTrack".to_string(),
            brand_tagline: "Pure Performance Track Cars".to_string(),
            nav_1: "Hypercars".to_string(),
            nav_2: "Track Days".to_string(),
            nav_3: "Telemetry".to_string(),
            hero_title: "Shred the Track Asphalt".to_string(),
            hero_subtitle: "Carbon fiber monocoques, aggressive aerodynamics, and high-revving naturally aspirated engines.".to_string(),
            hero_cta: "Unlock Track Mode".to_string(),
            lbl_loc: "Race Track".to_string(),
            lbl_start: "Check-In".to_string(),
            lbl_end: "Check-Out".to_string(),
            car1_name: "Stealth GT3-R".to_string(),
            car1_price: "850".to_string(),
            car1_060: "2.7s".to_string(),
            car1_top: "211mph".to_string(),
            car1_hp: "720 HP".to_string(),
            car1_cat: "Supercar".to_string(),
            car1_img: "https://images.unsplash.com/photo-1618843479313-40f8afb4b4d8?auto=format&fit=crop&w=800&q=80".to_string(),
            car2_name: "Apex Carbon Coupe".to_string(),
            car2_price: "900".to_string(),
            car2_060: "2.8s".to_string(),
            car2_top: "218mph".to_string(),
            car2_hp: "800 HP".to_string(),
            car2_cat: "Hypercar".to_string(),
            car2_img: "https://images.unsplash.com/photo-1605559424843-9e4c228bf1c2?auto=format&fit=crop&w=800&q=80".to_string(),
            car3_name: "Titan V8 Clubsport".to_string(),
            car3_price: "650".to_string(),
            car3_060: "3.2s".to_string(),
            car3_top: "198mph".to_string(),
            car3_hp: "680 HP".to_string(),
            car3_cat: "Clubsport".to_string(),
            car3_img: "https://images.unsplash.com/photo-1544829099-b9a0c07fad1a?auto=format&fit=crop&w=800&q=80".to_string(),
            glow: "25".to_string(),
            blur: "8".to_string(),
            radius: "4".to_string(),
            grid: true,
            font: "'IBM Plex Mono', monospace".to_string(),
            exp1_ico: "🏁".to_string(),
            exp1_title: "Track Coaching".to_string(),
            exp1_desc: "Book a professional racing driver for real-time radio coaching, telemetry critique, and apex identification at the Nürburgring.".to_string(),
            exp2_ico: "🏎️".to_string(),
            exp2_title: "Slick Tires Fitted".to_string(),
            exp2_desc: "Order customizable tire packages including slick track compounds, tire warmers, and dynamic pressure pit-lane telemetry.".to_string(),
            exp3_ico: "📈".to_string(),
            exp3_title: "Engine Remapping".to_string(),
            exp3_desc: "Optimize horsepower mapping parameters directly from your dashboard using WebAssembly-compiled custom tuning kernels.".to_string(),
        },
        _ => AppState {
            accent: "#22d3ee".to_string(),
            brand_name: "VoltDrive".to_string(),
            brand_tagline: "Electrified Performance Rentals".to_string(),
            nav_1: "Fleet".to_string(),
            nav_2: "Experiences".to_string(),
            nav_3: "About".to_string(),
            hero_title: "Experience the Future of Speed".to_string(),
            hero_subtitle: "Rent state-of-the-art electric hypercars with instantaneous torque and track-ready handling.".to_string(),
            hero_cta: "Reserve a Volt".to_string(),
            lbl_loc: "Location".to_string(),
            lbl_start: "Start Date".to_string(),
            lbl_end: "End Date".to_string(),
            car1_name: "Aether EV Hypercar".to_string(),
            car1_price: "750".to_string(),
            car1_060: "1.85s".to_string(),
            car1_top: "258mph".to_string(),
            car1_hp: "1900 HP".to_string(),
            car1_cat: "Electric".to_string(),
            car1_img: "https://images.unsplash.com/photo-1617788138017-80ad40651399?auto=format&fit=crop&w=800&q=80".to_string(),
            car2_name: "Neuron Roadster".to_string(),
            car2_price: "550".to_string(),
            car2_060: "1.99s".to_string(),
            car2_top: "250mph".to_string(),
            car2_hp: "1200 HP".to_string(),
            car2_cat: "Electric".to_string(),
            car2_img: "https://images.unsplash.com/photo-1503376780353-7e6692767b70?auto=format&fit=crop&w=800&q=80".to_string(),
            car3_name: "Volt-X Sportback".to_string(),
            car3_price: "400".to_string(),
            car3_060: "3.1s".to_string(),
            car3_top: "180mph".to_string(),
            car3_hp: "750 HP".to_string(),
            car3_cat: "EV Sedan".to_string(),
            car3_img: "https://images.unsplash.com/photo-1555215695-3004980ad54e?auto=format&fit=crop&w=800&q=80".to_string(),
            glow: "15".to_string(),
            blur: "12".to_string(),
            radius: "12".to_string(),
            grid: true,
            font: "'Syne', sans-serif".to_string(),
            exp1_ico: "📍".to_string(),
            exp1_title: "On-Demand Delivery".to_string(),
            exp1_desc: "Your selected hypercar will be flatbed-delivered directly to your airport terminal, villa, or residence, freshly detailed and charged/fueled.".to_string(),
            exp2_ico: "🛡️".to_string(),
            exp2_title: "Bespoke Protection".to_string(),
            exp2_desc: "Every lease incorporates complete premium loss-damage waiver, dynamic roadside telemetry assistance, and pre-negotiated premium track insurance.".to_string(),
            exp3_ico: "🦾".to_string(),
            exp3_title: "WASM Telemetry".to_string(),
            exp3_desc: "Connect directly to real-time track telemetry via decentralized dashboard nodes, letting you analyze torque distribution and lap speeds live.".to_string(),
        }
    }
}

// ── EXPORTED WASM HANDLERS ──

#[wasm_bindgen]
pub fn scroll_to_top() {
    let _ = js_sys::eval("window.scrollTo({ top: 0, behavior: 'smooth' })");
}

#[wasm_bindgen]
pub fn toggle_drawer(open: bool) {
    set_class_active("drawer", open);
    set_class_active("drawer-overlay", open);
}

#[wasm_bindgen]
pub fn on_search(event: web_sys::Event) {
    event.prevent_default();
    let doc = get_document();
    if let Some(fleet_sec) = doc.get_element_by_id("fleet") {
        fleet_sec.scroll_into_view();
    }
    show_toast("Searching available custom configurations...");
}

#[wasm_bindgen]
pub fn open_checkout(car_index: usize) {
    let loc = get_input_value("pickup-loc");
    let s_date_str = get_input_value("start-date");
    let e_date_str = get_input_value("end-date");

    STATE.with(|state_ref| {
        let state = state_ref.borrow();
        
        let (name, rate_str) = match car_index {
            0 => (&state.car1_name, &state.car1_price),
            1 => (&state.car2_name, &state.car2_price),
            _ => (&state.car3_name, &state.car3_price),
        };

        let rate = rate_str.parse::<f64>().unwrap_or(0.0);

        let mut days = 3;
        if !s_date_str.is_empty() && !e_date_str.is_empty() {
            let s_date = js_sys::Date::new(&wasm_bindgen::JsValue::from_str(&s_date_str));
            let e_date = js_sys::Date::new(&wasm_bindgen::JsValue::from_str(&e_date_str));
            let diff = (e_date.get_time() - s_date.get_time()).abs();
            days = ((diff / (1000.0 * 60.0 * 60.0 * 24.0)).ceil() as i32).max(1);
        }

        let base_rate_total = rate * (days as f64);
        let telemetry_fee = 45.00;
        let tax_rate = 0.12;
        let taxes = base_rate_total * tax_rate;
        let total = base_rate_total + telemetry_fee + taxes;

        set_text("chk-car-name", name);
        set_text("chk-loc", &loc);
        set_text("chk-days", &format!("{} Day{}", days, if days > 1 { "s" } else { "" }));
        set_text("chk-base-rate", &format!("${:.2} / Day", rate));
        set_text("chk-tax", &format!("${:.2}", taxes));
        set_text("chk-total", &format_price(total));

        set_class_active("checkout-drawer", true);
    });
}

#[wasm_bindgen]
pub fn close_checkout() {
    set_class_active("checkout-drawer", false);
}

#[wasm_bindgen]
pub fn confirm_booking() {
    close_checkout();
    show_toast("Rental reservation successfully registered in WASM telemetry nodes!");
}

#[wasm_bindgen]
pub fn load_preset(preset_key: &str) {
    let preset = get_preset(preset_key);
    
    STATE.with(|state_ref| {
        *state_ref.borrow_mut() = preset.clone();
    });

    let doc = get_document();
    if let Ok(btn_cards) = doc.query_selector_all(".btn-preset-card") {
        for i in 0..btn_cards.length() {
            if let Some(node) = btn_cards.get(i) {
                if let Ok(el) = node.dyn_into::<web_sys::Element>() {
                    let _ = el.class_list().remove_1("active");
                }
            }
        }
    }
    let btn_id = format!("btn-pre-{}", preset_key);
    set_class_active(&btn_id, true);

    set_input_value("fld-brand-name", &preset.brand_name);
    set_input_value("fld-brand-tag", &preset.brand_tagline);
    set_input_value("fld-brand-color", &preset.accent);
    
    set_input_value("fld-nav-1", &preset.nav_1);
    set_input_value("fld-nav-2", &preset.nav_2);
    set_input_value("fld-nav-3", &preset.nav_3);

    set_input_value("fld-hero-title", &preset.hero_title);
    set_input_value("fld-hero-sub", &preset.hero_subtitle);
    set_input_value("fld-hero-cta", &preset.hero_cta);
    
    set_input_value("fld-lbl-loc", &preset.lbl_loc);
    set_input_value("fld-lbl-start", &preset.lbl_start);
    set_input_value("fld-lbl-end", &preset.lbl_end);

    set_input_value("fld-car1-name", &preset.car1_name);
    set_input_value("fld-car1-price", &preset.car1_price);
    set_input_value("fld-car1-060", &preset.car1_060);
    set_input_value("fld-car1-top", &preset.car1_top);
    set_input_value("fld-car1-hp", &preset.car1_hp);
    set_input_value("fld-car1-cat", &preset.car1_cat);
    set_input_value("fld-car1-img", &preset.car1_img);

    set_input_value("fld-car2-name", &preset.car2_name);
    set_input_value("fld-car2-price", &preset.car2_price);
    set_input_value("fld-car2-060", &preset.car2_060);
    set_input_value("fld-car2-top", &preset.car2_top);
    set_input_value("fld-car2-hp", &preset.car2_hp);
    set_input_value("fld-car2-cat", &preset.car2_cat);
    set_input_value("fld-car2-img", &preset.car2_img);

    set_input_value("fld-car3-name", &preset.car3_name);
    set_input_value("fld-car3-price", &preset.car3_price);
    set_input_value("fld-car3-060", &preset.car3_060);
    set_input_value("fld-car3-top", &preset.car3_top);
    set_input_value("fld-car3-hp", &preset.car3_hp);
    set_input_value("fld-car3-cat", &preset.car3_cat);
    set_input_value("fld-car3-img", &preset.car3_img);

    set_input_value("fld-ui-font", &preset.font);
    set_input_value("fld-ui-glow", &preset.glow);
    set_text("lbl-ui-glow", &format!("{}px", preset.glow));
    set_input_value("fld-ui-blur", &preset.blur);
    set_text("lbl-ui-blur", &format!("{}px", preset.blur));
    set_input_value("fld-ui-radius", &preset.radius);
    set_text("lbl-ui-radius", &format!("{}px", preset.radius));
    
    if let Some(el) = get_document().get_element_by_id("fld-ui-grid") {
        if let Ok(chk_el) = el.dyn_into::<web_sys::HtmlInputElement>() {
            chk_el.set_checked(preset.grid);
        }
    }

    set_input_value("fld-exp1-ico", &preset.exp1_ico);
    set_input_value("fld-exp1-title", &preset.exp1_title);
    set_input_value("fld-exp1-desc", &preset.exp1_desc);

    set_input_value("fld-exp2-ico", &preset.exp2_ico);
    set_input_value("fld-exp2-title", &preset.exp2_title);
    set_input_value("fld-exp2-desc", &preset.exp2_desc);

    set_input_value("fld-exp3-ico", &preset.exp3_ico);
    set_input_value("fld-exp3-title", &preset.exp3_title);
    set_input_value("fld-exp3-desc", &preset.exp3_desc);

    apply_accent_styles(&preset.accent);
    apply_font_styles(&preset.font);

    let doc_el = get_document().document_element().unwrap();
    if let Ok(style_el) = doc_el.dyn_into::<web_sys::HtmlElement>() {
        let style = style_el.style();
        let _ = style.set_property("--glow-strength", &format!("{}px", preset.glow));
        let _ = style.set_property("--card-blur", &format!("{}px", preset.blur));
        let _ = style.set_property("--border-radius", &format!("{}px", preset.radius));
    }
    if let Some(grid_el) = get_document().get_element_by_id("bg-grid") {
        if let Ok(grid_html) = grid_el.dyn_into::<web_sys::HtmlElement>() {
            let display_val = if preset.grid { "block" } else { "none" };
            let _ = grid_html.style().set_property("display", display_val);
        }
    }

    update_dom(&preset);
}

#[wasm_bindgen]
pub fn on_field_input() {
    STATE.with(|state_ref| {
        let mut state = state_ref.borrow_mut();
        
        state.brand_name = get_input_value("fld-brand-name");
        state.brand_tagline = get_input_value("fld-brand-tag");
        state.nav_1 = get_input_value("fld-nav-1");
        state.nav_2 = get_input_value("fld-nav-2");
        state.nav_3 = get_input_value("fld-nav-3");
        
        state.hero_title = get_input_value("fld-hero-title");
        state.hero_subtitle = get_input_value("fld-hero-sub");
        state.hero_cta = get_input_value("fld-hero-cta");
        state.lbl_loc = get_input_value("fld-lbl-loc");
        state.lbl_start = get_input_value("fld-lbl-start");
        state.lbl_end = get_input_value("fld-lbl-end");

        state.car1_name = get_input_value("fld-car1-name");
        state.car1_price = get_input_value("fld-car1-price");
        state.car1_060 = get_input_value("fld-car1-060");
        state.car1_top = get_input_value("fld-car1-top");
        state.car1_hp = get_input_value("fld-car1-hp");
        state.car1_cat = get_input_value("fld-car1-cat");
        state.car1_img = get_input_value("fld-car1-img");

        state.car2_name = get_input_value("fld-car2-name");
        state.car2_price = get_input_value("fld-car2-price");
        state.car2_060 = get_input_value("fld-car2-060");
        state.car2_top = get_input_value("fld-car2-top");
        state.car2_hp = get_input_value("fld-car2-hp");
        state.car2_cat = get_input_value("fld-car2-cat");
        state.car2_img = get_input_value("fld-car2-img");

        state.car3_name = get_input_value("fld-car3-name");
        state.car3_price = get_input_value("fld-car3-price");
        state.car3_060 = get_input_value("fld-car3-060");
        state.car3_top = get_input_value("fld-car3-top");
        state.car3_hp = get_input_value("fld-car3-hp");
        state.car3_cat = get_input_value("fld-car3-cat");
        state.car3_img = get_input_value("fld-car3-img");

        state.exp1_ico = get_input_value("fld-exp1-ico");
        state.exp1_title = get_input_value("fld-exp1-title");
        state.exp1_desc = get_input_value("fld-exp1-desc");

        state.exp2_ico = get_input_value("fld-exp2-ico");
        state.exp2_title = get_input_value("fld-exp2-title");
        state.exp2_desc = get_input_value("fld-exp2-desc");

        state.exp3_ico = get_input_value("fld-exp3-ico");
        state.exp3_title = get_input_value("fld-exp3-title");
        state.exp3_desc = get_input_value("fld-exp3-desc");

        check_custom_status(&state);
        update_dom(&state);
    });
}

#[wasm_bindgen]
pub fn on_ui_field_input(ui_type: &str, value: &str) {
    STATE.with(|state_ref| {
        let mut state = state_ref.borrow_mut();
        
        let doc_el = get_document().document_element().unwrap();
        if let Ok(style_el) = doc_el.dyn_into::<web_sys::HtmlElement>() {
            let style = style_el.style();
            
            match ui_type {
                "glow" => {
                    state.glow = value.to_string();
                    set_text("lbl-ui-glow", &format!("{}px", value));
                    let _ = style.set_property("--glow-strength", &format!("{}px", value));
                }
                "blur" => {
                    state.blur = value.to_string();
                    set_text("lbl-ui-blur", &format!("{}px", value));
                    let _ = style.set_property("--card-blur", &format!("{}px", value));
                }
                "radius" => {
                    state.radius = value.to_string();
                    set_text("lbl-ui-radius", &format!("{}px", value));
                    let _ = style.set_property("--border-radius", &format!("{}px", value));
                }
                "grid" => {
                    let is_checked = value == "true";
                    state.grid = is_checked;
                    if let Some(grid_el) = get_document().get_element_by_id("bg-grid") {
                        if let Ok(grid_html) = grid_el.dyn_into::<web_sys::HtmlElement>() {
                            let display_val = if is_checked { "block" } else { "none" };
                            let _ = grid_html.style().set_property("display", display_val);
                        }
                    }
                }
                _ => {}
            }
        }
        
        check_custom_status(&state);
        update_dom(&state);
    });
}

#[wasm_bindgen]
pub fn on_ui_font_input(value: &str) {
    STATE.with(|state_ref| {
        let mut state = state_ref.borrow_mut();
        state.font = value.to_string();
        apply_font_styles(value);
        check_custom_status(&state);
        update_dom(&state);
    });
}

#[wasm_bindgen]
pub fn set_accent_color(hex: &str) {
    STATE.with(|state_ref| {
        let mut state = state_ref.borrow_mut();
        state.accent = hex.to_string();
        set_input_value("fld-brand-color", hex);
        apply_accent_styles(hex);
        check_custom_status(&state);
        update_dom(&state);
    });
}

#[wasm_bindgen]
pub fn init_app() {
    let today = js_sys::Date::new_0();
    
    let tomorrow = js_sys::Date::new_0();
    tomorrow.set_date(today.get_date() + 1);
    
    let next_day = js_sys::Date::new_0();
    next_day.set_date(today.get_date() + 4);

    let tomorrow_iso = tomorrow.to_iso_string().as_string().unwrap_or_default();
    let tomorrow_str = tomorrow_iso.split('T').next().unwrap_or("");
    
    let next_day_iso = next_day.to_iso_string().as_string().unwrap_or_default();
    let next_day_str = next_day_iso.split('T').next().unwrap_or("");

    set_input_value("start-date", tomorrow_str);
    set_input_value("end-date", next_day_str);

    load_preset("cyber");
}

#[wasm_bindgen]
pub fn show_toast(text: &str) {
    let doc = get_document();
    if let Some(toast_text) = doc.get_element_by_id("notification-text") {
        toast_text.set_text_content(Some(text));
    }
    set_class_active("notification", true);
    
    let closure = Closure::wrap(Box::new(move || {
        set_class_active("notification", false);
    }) as Box<dyn FnMut()>);
    
    let window = web_sys::window().unwrap();
    let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
        closure.as_ref().unchecked_ref(),
        4000
    );
    closure.forget();
}
