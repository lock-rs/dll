#![allow(warnings)]
use egui::{
    Align2, Color32, Context, FontData, FontDefinitions, FontFamily, FontId, FontTweak, Key,
    Modifiers, Pos2, Rect, RichText, ScrollArea, Slider, Stroke, TextureId, TextureOptions, Vec2,
    Widget,
};
use egui_d3d11::DirectX11App;
use faithe::{internal::alloc_console, pattern::Pattern};
use std::{
    intrinsics::transmute,
    sync::{Arc, Once},
};
use windows::{
    core::HRESULT,
    Win32::{
        Foundation::{HWND, LPARAM, LRESULT, WPARAM},
        Graphics::Dxgi::{Common::DXGI_FORMAT, IDXGISwapChain},
        UI::WindowsAndMessaging::{CallWindowProcW, SetWindowLongPtrA, GWLP_WNDPROC, WNDPROC},
    },
};

use crate::offsets;
use offsets::Vector2;
use offsets::Vector3;
use retour::static_detour;
use shroud::directx::directx11;

static mut APP: DirectX11App<i32> = DirectX11App::new();
static mut OLD_WND_PROC: Option<WNDPROC> = None;

type FnPresent = unsafe extern "stdcall" fn(IDXGISwapChain, u32, u32) -> HRESULT;
type FnResizeBuffers =
    unsafe extern "stdcall" fn(IDXGISwapChain, u32, u32, u32, DXGI_FORMAT, u32) -> HRESULT;

static mut O_PRESENT: Option<FnPresent> = None;
static mut O_RESIZE_BUFFERS: Option<FnResizeBuffers> = None;

static_detour! {
    static PresentHook: unsafe extern "stdcall" fn(IDXGISwapChain, u32, u32) -> HRESULT;
    static ResizeBufferHook: unsafe extern "stdcall" fn(IDXGISwapChain, u32, u32, u32, DXGI_FORMAT, u32) -> HRESULT;
}

fn hk_present(swap_chain: IDXGISwapChain, sync_interval: u32, flags: u32) -> HRESULT {
    unsafe {
        static INIT: Once = Once::new();

        INIT.call_once(|| {

            APP.init_default(&swap_chain, crate::ui::ui);

            let mut desc = unsafe { std::mem::zeroed() };

            swap_chain.GetDesc(&mut desc).unwrap();

            if desc.OutputWindow.0 == -1 {
                panic!("Invalid window handle");
            }

            OLD_WND_PROC = Some(transmute(SetWindowLongPtrA(
                desc.OutputWindow,
                GWLP_WNDPROC,
                hk_wnd_proc as usize as _,
            )));
        });

        APP.present(&swap_chain);

        PresentHook.call(swap_chain, sync_interval, flags)
    }
}

fn hk_resize_buffers(
    swap_chain: IDXGISwapChain,
    buffer_count: u32,
    width: u32,
    height: u32,
    new_format: DXGI_FORMAT,
    swap_chain_flags: u32,
) -> HRESULT {
    eprintln!("Resizing buffers");
    unsafe {
        APP.resize_buffers(&swap_chain, || {
            ResizeBufferHook.call(
                swap_chain.clone(),
                buffer_count,
                width,
                height,
                new_format,
                swap_chain_flags,
            )
        })
    }
}

unsafe extern "stdcall" fn hk_wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    APP.wnd_proc(msg, wparam, lparam);

    CallWindowProcW(OLD_WND_PROC.unwrap(), hwnd, msg, wparam, lparam)
}




static mut FRAME: i32 = 0;
/* fn ui(ctx: &Context, i: &mut i32) {
    unsafe {
        // You should not use statics like this, it's made
        // this way for the sake of example.
        static mut UI_CHECK: bool = true;
        static mut TEXT: Option<String> = None;
        static mut VALUE: f32 = 0.;
        static mut COLOR: [f32; 3] = [0., 0., 0.];
        static ONCE: Once = Once::new();

        /* ONCE.call_once(|| {
                    // Uncomment this to set other fonts.
                    // let mut fonts = FontDefinitions::default();
                    // let mut tweak = FontTweak::default();
                    // fonts.font_data.insert(
                    //     "my_font".to_owned(),
                    //     FontData::from_static(include_bytes!("Lobster-Regular.ttf")).tweak(tweak),
                    // );
                    // fonts
                    //     .families
                    //     .get_mut(&FontFamily::Proportional)
                    //     .unwrap()
                    //     .insert(0, "my_font".to_owned());
                    // fonts
                    //     .families
                    //     .get_mut(&FontFamily::Monospace)
                    //     .unwrap()
                    //     .push("my_font".to_owned());
                    // ctx.set_fonts(fonts);
                });

                if TEXT.is_none() {
                    TEXT = Some(String::from("Test"));
                }

                ctx.debug_painter().text(
                    Pos2::new(0., 0.),
                    Align2::LEFT_TOP,
                    "Bruh",
                    FontId::default(),
                    Color32::RED,
                );

                egui::containers::Window::new("Main menu").show(ctx, |ui| {
                    let mods = ui.input(|input| input.modifiers);
                    ui.label(format!(
                        "Ctrl: {} Shift: {} Alt: {}",
                        mods.ctrl, mods.shift, mods.alt
                    ));

                    ui.separator();

                    ui.label(RichText::new(format!("I: {}", *i)).color(Color32::LIGHT_RED));

                    let input = ctx.input(|input| input.pointer.clone());
                    ui.label(format!(
                        "X1: {} X2: {}",
                        input.button_down(egui::PointerButton::Extra1),
                        input.button_down(egui::PointerButton::Extra2)
                    ));

                    let mods = ui.input(|input| input.modifiers);
                    ui.label(format!(
                        "Ctrl: {} Shift: {} Alt: {}",
                        mods.ctrl, mods.shift, mods.alt
                    ));

                    if ui.input(|input| {
                        input.modifiers.matches(Modifiers::CTRL) && input.key_pressed(Key::R)
                    }) {
                        println!("Pressed");
                    }

                    unsafe {
                        ui.checkbox(&mut UI_CHECK, "Some checkbox");
                        ui.text_edit_singleline(TEXT.as_mut().unwrap());
                        ScrollArea::vertical().max_height(200.).show(ui, |ui| {
                            for i in 1..=100 {
                                ui.label(format!("Label: {}", i));
                            }
                        });

                        Slider::new(&mut VALUE, -1.0..=1.0).ui(ui);

                        ui.color_edit_button_rgb(&mut COLOR);
                    }

                    ui.label(format!(
                        "{:?}",
                        &ui.input(|input| input.pointer.button_down(egui::PointerButton::Primary))
                    ));
                    if ui.button("You can't click me yet").clicked() {
                        *i += 1;
                    }
                });

                egui::Window::new("Image").show(ctx, |ui| unsafe {
                    static mut IMG: TextureId = TextureId::Managed(0);

                    if IMG == TextureId::Managed(0) {
                        let tex = Box::leak(Box::new(ctx.load_texture(
                            "logo",
                            egui_extras::image::load_image_bytes(include_bytes!("logo.bmp")).unwrap(),
                            TextureOptions::LINEAR,
                        )));

                        IMG = tex.id();
                    }

                    ui.image(IMG, Vec2::new(500., 391.));
                });


                let datamodel = offsets.get_datamodel();
                let players = offsets.find_first_child(datamodel, "Players");

                for player in offsets.get_children(players) {
                    if (player == 0) {continue};
                    let character = offsets.get_character(player);
                    if (character == 0) {continue};
                    let head = offsets.find_first_child(character, "Head");
                    if (head == 0) {continue};
                    let pos = offsets.get_position(head);


                    let cool = offsets.world2screen(pos);

        /*             ctx.debug_painter().circle(
                        Pos2::new(cool.x, cool.y),
                        35.0,
                        Color32::from_rgba_premultiplied(0, 255, 0, 200),
                        Stroke::none(),
                    ); */

                    let middl = offsets.getscreendim();
                    // Make a line to the world 2 screen position
        /*             let input = ctx.input(|input| input.pointer.clone());
                    // Pos2::new(middl.x / 2.0, middl.y / 2.0)
                    let curserpos = input.hover_pos().unwrap(); */

                    let stroke = Stroke::new(2.0, Color32::from_rgba_premultiplied(255, 255, 255, 75));
                    ctx.debug_painter().line_segment([Pos2::new(cool.x, cool.y), Pos2::new(middl.x / 2.0, middl.y / 2.0)],stroke);

                }  */

        let pointer_pos = ctx.pointer_latest_pos();
        if pointer_pos.is_some() {
            // FOV Circle
            if !*self
                .bool_map
                .entry("aimbot_hide_fov".to_owned())
                .or_insert(false as bool)
                && *self
                    .bool_map
                    .entry("aimbot_fov_enabled".to_owned())
                    .or_insert(false as bool)
                && *self
                    .bool_map
                    .entry("aimbot_enabled".to_owned())
                    .or_insert(false as bool)
            {
                let fov_clr = *self
                    .color_map
                    .entry("aimbot_fov_color".to_owned())
                    .or_insert([255, 255, 255, 255] as [u8; 4]);
                let pointer_pos_with_offset = Pos2::new(
                    pointer_pos.unwrap().x
                        + *self
                            .f32_map
                            .entry("aimbot_xoffset".to_owned())
                            .or_insert(0.0 as f32),
                    pointer_pos.unwrap().y
                        + *self
                            .f32_map
                            .entry("aimbot_yoffset".to_owned())
                            .or_insert(0.0 as f32),
                );

                if *self
                    .bool_map
                    .entry("fov_circle_filled".to_owned())
                    .or_insert(false as bool)
                    == true
                {
                    ctx.debug_painter().circle(
                        pointer_pos_with_offset,
                        *self
                            .u32_map
                            .entry("aimbot_fov_value".to_owned())
                            .or_insert(100 as u32) as f32,
                        Color32::from_rgba_unmultiplied(
                            fov_clr[0], fov_clr[1], fov_clr[2], fov_clr[3],
                        ),
                        egui::Stroke::new(
                            *self
                                .f32_map
                                .entry("fov_circle_thickness".to_owned())
                                .or_insert(1.0 as f32),
                            Color32::from_rgba_unmultiplied(
                                fov_clr[0], fov_clr[1], fov_clr[2], fov_clr[3],
                            ),
                        ),
                    );
                } else {
                    ctx.debug_painter().circle_stroke(
                        pointer_pos_with_offset,
                        *self
                            .u32_map
                            .entry("aimbot_fov_value".to_owned())
                            .or_insert(100 as u32) as f32,
                        egui::Stroke::new(
                            *self
                                .f32_map
                                .entry("fov_circle_thickness".to_owned())
                                .or_insert(1.0 as f32),
                            Color32::from_rgba_unmultiplied(
                                fov_clr[0], fov_clr[1], fov_clr[2], fov_clr[3],
                            ),
                        ),
                    );
                }
            }
        };

        // Player ESP
        if *self
            .bool_map
            .entry("esp_enabled".to_owned())
            .or_insert(false as bool)
        {
            let esp_clr = *self
                .color_map
                .entry("esp_color".to_owned())
                .or_insert([255, 255, 255, 255] as [u8; 4]);

            // Player Vars
            let rect = egui::Rect::from_two_pos(
                egui::Pos2::new(200.0, 200.0),
                egui::Pos2::new(250.0, 300.0),
            );
            let name = "Player";
            let mut distance = *self
                .u32_map
                .entry("temp_debug_slider2".to_owned())
                .or_insert(69 as u32);

            // Tracers
            if *self
                .bool_map
                .entry("esp_tracers_enabled".to_owned())
                .or_insert(false as bool)
            {
                let mut tracer_clr = *self
                    .color_map
                    .entry("esp_tracers_color".to_owned())
                    .or_insert([255, 255, 255, 255] as [u8; 4]);

                if *self
                    .bool_map
                    .entry("esp_tracers_distance_based".to_owned())
                    .or_insert(false as bool)
                {
                    let mut max_dist = 1000;
                    if *self
                        .bool_map
                        .entry("esp_distance_limited".to_owned())
                        .or_insert(false as bool)
                    {
                        max_dist = *self
                            .u32_map
                            .entry("esp_distance_limit".to_owned())
                            .or_insert(1000 as u32)
                    };

                    let new_dist = if distance == 0 { 1 } else { distance };

                    let green = 255.0 - 255.0 / (max_dist as f32 / new_dist as f32);
                    let red = 255.0 / (max_dist as f32 / new_dist as f32);

                    tracer_clr = [red as u8, green as u8, 0, 255];
                }

                match *self
                    .usize_map
                    .entry("esp_tracers_type".to_owned())
                    .or_insert(0 as usize)
                {
                    0 => {
                        // TOP
                        ctx.debug_painter().line_segment(
                            [rect.center_bottom(), ctx.available_rect().center_top()],
                            egui::Stroke::new(
                                1.0,
                                Color32::from_rgba_unmultiplied(
                                    tracer_clr[0],
                                    tracer_clr[1],
                                    tracer_clr[2],
                                    tracer_clr[3],
                                ),
                            ),
                        );
                    }
                    1 => {
                        // Middle
                        ctx.debug_painter().line_segment(
                            [rect.center_bottom(), ctx.available_rect().center()],
                            egui::Stroke::new(
                                1.0,
                                Color32::from_rgba_unmultiplied(
                                    tracer_clr[0],
                                    tracer_clr[1],
                                    tracer_clr[2],
                                    tracer_clr[3],
                                ),
                            ),
                        );
                    }
                    2 => {
                        // Bottom Middle
                        ctx.debug_painter().line_segment(
                            [
                                rect.center_bottom(),
                                ctx.available_rect().center_bottom()
                                    - vec2(0.0, ctx.available_rect().height() / 5.0),
                            ],
                            egui::Stroke::new(
                                1.0,
                                Color32::from_rgba_unmultiplied(
                                    tracer_clr[0],
                                    tracer_clr[1],
                                    tracer_clr[2],
                                    tracer_clr[3],
                                ),
                            ),
                        );
                    }
                    3 => {
                        // Bottom
                        ctx.debug_painter().line_segment(
                            [rect.center_bottom(), ctx.available_rect().center_bottom()],
                            egui::Stroke::new(
                                1.0,
                                Color32::from_rgba_unmultiplied(
                                    tracer_clr[0],
                                    tracer_clr[1],
                                    tracer_clr[2],
                                    tracer_clr[3],
                                ),
                            ),
                        );
                    }
                    4 => {
                        if pointer_pos.is_some() {
                            ctx.debug_painter().line_segment(
                                [rect.center_bottom(), pointer_pos.unwrap()],
                                egui::Stroke::new(
                                    1.0,
                                    Color32::from_rgba_unmultiplied(
                                        tracer_clr[0],
                                        tracer_clr[1],
                                        tracer_clr[2],
                                        tracer_clr[3],
                                    ),
                                ),
                            )
                        };
                    }

                    _ => {
                        // Overflow
                        ctx.debug_painter().line_segment(
                            [rect.center_bottom(), ctx.available_rect().center_top()],
                            egui::Stroke::new(
                                1.0,
                                Color32::from_rgba_unmultiplied(
                                    tracer_clr[0],
                                    tracer_clr[1],
                                    tracer_clr[2],
                                    tracer_clr[3],
                                ),
                            ),
                        );
                    }
                }
            }

            // ESP BOX
            match *self
                .usize_map
                .entry("esp_type".to_owned())
                .or_insert(1 as usize)
            {
                0 => { // None
                }
                1 => {
                    // 2D Box
                    if *self
                        .bool_map
                        .entry("esp_box_filled".to_owned())
                        .or_insert(false as bool)
                    {
                        ctx.debug_painter().rect(
                            rect,
                            *self
                                .f32_map
                                .entry("esp_box_rounding".to_owned())
                                .or_insert(6.0 as f32),
                            Color32::from_rgba_unmultiplied(
                                esp_clr[0], esp_clr[1], esp_clr[2], esp_clr[3],
                            ),
                            egui::Stroke::new(
                                1.0,
                                Color32::from_rgba_unmultiplied(
                                    esp_clr[0], esp_clr[1], esp_clr[2], esp_clr[3],
                                ),
                            ),
                        );
                    } else {
                        ctx.debug_painter().rect_stroke(
                            rect,
                            *self
                                .f32_map
                                .entry("esp_box_rounding".to_owned())
                                .or_insert(6.0 as f32),
                            egui::Stroke::new(
                                1.0,
                                Color32::from_rgba_unmultiplied(
                                    esp_clr[0], esp_clr[1], esp_clr[2], esp_clr[3],
                                ),
                            ),
                        );
                    }
                }
                2 => { // 3D Box
                }
                3 => {
                    // Corners
                    if *self
                        .bool_map
                        .entry("esp_box_filled".to_owned())
                        .or_insert(false as bool)
                    {
                        ctx.debug_painter().rect(
                            rect,
                            *self
                                .f32_map
                                .entry("esp_box_rounding".to_owned())
                                .or_insert(6.0 as f32),
                            Color32::from_rgba_unmultiplied(
                                esp_clr[0], esp_clr[1], esp_clr[2], esp_clr[3],
                            ),
                            egui::Stroke::new(
                                1.0,
                                Color32::from_rgba_unmultiplied(
                                    esp_clr[0], esp_clr[1], esp_clr[2], esp_clr[3],
                                ),
                            ),
                        );
                    } else {
                        ctx.debug_painter().line_segment(
                            [
                                rect.left_top(),
                                rect.left_top() + vec2(rect.width() / 4.0, 0.0),
                            ],
                            egui::Stroke::new(
                                1.0,
                                Color32::from_rgba_unmultiplied(
                                    esp_clr[0], esp_clr[1], esp_clr[2], esp_clr[3],
                                ),
                            ),
                        );
                        ctx.debug_painter().line_segment(
                            [
                                rect.left_top(),
                                rect.left_top() + vec2(0.0, rect.width() / 2.0),
                            ],
                            egui::Stroke::new(
                                1.0,
                                Color32::from_rgba_unmultiplied(
                                    esp_clr[0], esp_clr[1], esp_clr[2], esp_clr[3],
                                ),
                            ),
                        );

                        ctx.debug_painter().line_segment(
                            [
                                rect.right_top(),
                                rect.right_top() - vec2(rect.width() / 4.0, 0.0),
                            ],
                            egui::Stroke::new(
                                1.0,
                                Color32::from_rgba_unmultiplied(
                                    esp_clr[0], esp_clr[1], esp_clr[2], esp_clr[3],
                                ),
                            ),
                        );
                        ctx.debug_painter().line_segment(
                            [
                                rect.right_top(),
                                rect.right_top() + vec2(0.0, rect.width() / 2.0),
                            ],
                            egui::Stroke::new(
                                1.0,
                                Color32::from_rgba_unmultiplied(
                                    esp_clr[0], esp_clr[1], esp_clr[2], esp_clr[3],
                                ),
                            ),
                        );

                        ctx.debug_painter().line_segment(
                            [
                                rect.left_bottom(),
                                rect.left_bottom() + vec2(rect.width() / 4.0, 0.0),
                            ],
                            egui::Stroke::new(
                                1.0,
                                Color32::from_rgba_unmultiplied(
                                    esp_clr[0], esp_clr[1], esp_clr[2], esp_clr[3],
                                ),
                            ),
                        );
                        ctx.debug_painter().line_segment(
                            [
                                rect.left_bottom(),
                                rect.left_bottom() - vec2(0.0, rect.width() / 2.0),
                            ],
                            egui::Stroke::new(
                                1.0,
                                Color32::from_rgba_unmultiplied(
                                    esp_clr[0], esp_clr[1], esp_clr[2], esp_clr[3],
                                ),
                            ),
                        );

                        ctx.debug_painter().line_segment(
                            [
                                rect.right_bottom(),
                                rect.right_bottom() - vec2(rect.width() / 4.0, 0.0),
                            ],
                            egui::Stroke::new(
                                1.0,
                                Color32::from_rgba_unmultiplied(
                                    esp_clr[0], esp_clr[1], esp_clr[2], esp_clr[3],
                                ),
                            ),
                        );
                        ctx.debug_painter().line_segment(
                            [
                                rect.right_bottom(),
                                rect.right_bottom() - vec2(0.0, rect.width() / 2.0),
                            ],
                            egui::Stroke::new(
                                1.0,
                                Color32::from_rgba_unmultiplied(
                                    esp_clr[0], esp_clr[1], esp_clr[2], esp_clr[3],
                                ),
                            ),
                        );
                    }
                }

                _ => {
                    // Exces
                    if *self
                        .bool_map
                        .entry("esp_box_filled".to_owned())
                        .or_insert(false as bool)
                    {
                        ctx.debug_painter().rect(
                            egui::Rect::from_two_pos(
                                egui::Pos2::new(200.0, 200.0),
                                egui::Pos2::new(250.0, 300.0),
                            ),
                            *self
                                .f32_map
                                .entry("esp_box_rounding".to_owned())
                                .or_insert(6.0 as f32),
                            Color32::from_rgba_unmultiplied(
                                esp_clr[0], esp_clr[1], esp_clr[2], esp_clr[3],
                            ),
                            egui::Stroke::new(
                                1.0,
                                Color32::from_rgba_unmultiplied(
                                    esp_clr[0], esp_clr[1], esp_clr[2], esp_clr[3],
                                ),
                            ),
                        );
                    } else {
                        ctx.debug_painter().rect_stroke(
                            egui::Rect::from_two_pos(
                                egui::Pos2::new(200.0, 200.0),
                                egui::Pos2::new(250.0, 300.0),
                            ),
                            *self
                                .f32_map
                                .entry("esp_box_rounding".to_owned())
                                .or_insert(6.0 as f32),
                            egui::Stroke::new(
                                1.0,
                                Color32::from_rgba_unmultiplied(
                                    esp_clr[0], esp_clr[1], esp_clr[2], esp_clr[3],
                                ),
                            ),
                        );
                    }
                }
            }

            if *self
                .bool_map
                .entry("esp_health_bar".to_owned())
                .or_insert(false as bool)
                && *self
                    .bool_map
                    .entry("esp_show_health".to_owned())
                    .or_insert(false as bool)
            {
                let tl = rect.left_top() - vec2(7.0 + 1.0 / 2.0, (1.0 / 2.0 - 0.5));
                let br = rect.left_bottom() - vec2(3.0 + 1.0 / 2.0, -(1.0 / 2.0 - 0.5));

                let hp = *self
                    .u32_map
                    .entry("temp_debug_slider".to_owned())
                    .or_insert(50 as u32) as f32;
                let max_hp = 100.0 as f32;

                // Outer
                ctx.debug_painter().rect(
                    egui::Rect::from_two_pos(tl - vec2(1.0, 1.0), br + vec2(1.0, 1.0)),
                    *self
                        .f32_map
                        .entry("esp_box_rounding".to_owned())
                        .or_insert(6.0 as f32),
                    Color32::from_rgba_unmultiplied(esp_clr[0], esp_clr[1], esp_clr[2], esp_clr[3]),
                    egui::Stroke::none(),
                );

                // Inner Black
                ctx.debug_painter().rect(
                    egui::Rect::from_two_pos(tl, br),
                    *self
                        .f32_map
                        .entry("esp_box_rounding".to_owned())
                        .or_insert(6.0 as f32),
                    Color32::from_rgba_unmultiplied(0, 0, 0, 255),
                    egui::Stroke::none(),
                );

                // Main Inner
                let new_hp = if hp <= 1.0 {
                    1.0
                } else if hp >= max_hp {
                    max_hp
                } else {
                    hp
                };
                let addon = (rect.height() as f32) / (max_hp as f32 / new_hp as f32);

                let red = 255.0 - 255.0 / (max_hp as f32 / new_hp as f32);
                let green = 255.0 / (max_hp as f32 / new_hp as f32);

                ctx.debug_painter().rect(
                    egui::Rect::from_two_pos(
                        egui::pos2(tl.x + 1.0, br.y - addon),
                        br - vec2(1.0, 0.0),
                    ),
                    *self
                        .f32_map
                        .entry("esp_box_rounding".to_owned())
                        .or_insert(6.0 as f32),
                    Color32::from_rgb(red as u8, green as u8, 0),
                    egui::Stroke::none(),
                );

                if *self
                    .bool_map
                    .entry("esp_health_text".to_owned())
                    .or_insert(false as bool)
                {
                    ctx.debug_painter().text(
                        egui::pos2(tl.x - 4.0, br.y + 2.0),
                        Align2::RIGHT_BOTTOM,
                        format!("{}/{}", hp, max_hp),
                        FontId::proportional(
                            *self
                                .f32_map
                                .entry("esp_text_size".to_owned())
                                .or_insert(12.0 as f32),
                        ),
                        Color32::from_rgb(red as u8, green as u8, 0),
                    );
                }
            }

            // Distance
            if *self
                .bool_map
                .entry("esp_distance".to_owned())
                .or_insert(false as bool)
            {
                ctx.debug_painter().text(
                    rect.center_bottom() + vec2(0.0, 2.0),
                    Align2::CENTER_TOP,
                    format!("{}", distance),
                    FontId::proportional(
                        *self
                            .f32_map
                            .entry("esp_text_size".to_owned())
                            .or_insert(12.0 as f32),
                    ),
                    Color32::from_rgba_unmultiplied(esp_clr[0], esp_clr[1], esp_clr[2], esp_clr[3]),
                );
            }

            // Names
            if *self
                .bool_map
                .entry("esp_names".to_owned())
                .or_insert(false as bool)
            {
                ctx.debug_painter().text(
                    rect.center_top() - vec2(0.0, 2.0),
                    Align2::CENTER_BOTTOM,
                    format!("{}", name),
                    FontId::proportional(
                        *self
                            .f32_map
                            .entry("esp_text_size".to_owned())
                            .or_insert(12.0 as f32),
                    ),
                    Color32::from_rgba_unmultiplied(esp_clr[0], esp_clr[1], esp_clr[2], esp_clr[3]),
                );
            }
        }

        // HERE

        //== Login Frame ==//
        if self.injection_progress == 0 {
            egui::TopBottomPanel::top("dawg").show(ctx, |ui| {
                ctx.set_visuals(egui::Visuals::dark());

                // Log In //
                ui.add_space(5.0);

                ui.label("Username");
                ui.add_sized(
                    vec2(ui.available_width(), 0.0),
                    egui::TextEdit::singleline(&mut self.login_username).interactive(true),
                );

                ui.label("Password");
                ui.add_sized(
                    vec2(ui.available_width(), 0.0),
                    egui::TextEdit::singleline(&mut self.login_password)
                        .password(true)
                        .interactive(true),
                );

                ui.add_space(5.0);

                if ui
                    .add_sized(vec2(ui.available_width(), 0.0), egui::Button::new("Enter"))
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .clicked()
                {
                    self.injection_progress = 1;
                }

                ui.add_space(100.0);
            });
        } else if self.injection_progress == 1 {
            _frame.set_window_size(egui::vec2(300.0, 65.0));

            egui::TopBottomPanel::top("dawg").show(ctx, |ui| {
                ctx.set_visuals(egui::Visuals::dark());

                // Inject //
                ui.add_space(5.0);

                ui.label(
                    egui::RichText::new("⏳ Injecting DLL...")
                        .color(egui::Color32::from_rgb(255, 255, 100)),
                );

                //== INJECTING DLL ==//

                // DLL Check with spam prevention
                if self.injected == false {
                    self.injected = true;

                    if std::path::Path::new(DLL_NAME).exists() == false {
                        self.injection_status = String::from("⚠ DLL not found!");
                    } else {
                        if OwnedProcess::find_first_by_name("RobloxPlayerBeta").is_none() {
                            self.injection_status = String::from("⚠ Roblox not found.");
                        } else {
                            ui.label(
                                egui::RichText::new("⏳ Roblox found, injecting...")
                                    .color(egui::Color32::from_rgb(255, 255, 100)),
                            );

                            // Injecting
                            let target_process =
                                OwnedProcess::find_first_by_name("RobloxPlayerBeta").unwrap();

                            let syringe = Syringe::for_process(target_process);

                            syringe.inject(DLL_NAME).unwrap();

                            ui.label(
                                egui::RichText::new("👍 Success!")
                                    .color(egui::Color32::from_rgb(100, 255, 100)),
                            );

                            self.injection_progress = 3;
                        }
                    }
                } else {
                    ui.label(
                        egui::RichText::new(&*self.injection_status)
                            .color(egui::Color32::from_rgb(255, 100, 100)),
                    );

                    if ui
                        .add_sized(vec2(ui.available_width(), 0.0), egui::Button::new("Retry"))
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        self.injected = false;
                    }
                }

                ui.add_space(100.0);
            });
        } else if self.injection_progress == 3 {
            _frame.set_window_size(egui::vec2(2000.0, 1000.0));

            egui::Window::new(WINDOW_NAME)
                .resizable(false)
                .fixed_size(egui::vec2(700.0, 500.0))
                .show(ctx, |ui| {
                    //== Main Frame ==//
                    // let win = egui::Window::new(WINDOW_NAME)
                    //     .scroll2([true, true])
                    //     .default_size(vec2(600.0, 9000.0))
                    //     .title_bar(true)
                    //     .resizable(false);

                    // Toasts
                    let mut toasts = Toasts::new(ctx)
                        .anchor((595.0, 5.0))
                        .direction(egui::Direction::TopDown)
                        .align_to_end(true)
                        .custom_contents(ToastKind::Error, &|ui, toast| {
                            egui::Frame::window(ui.style())
                                //.fill()
                                .inner_margin(egui::style::Margin::same(5.0))
                                .rounding(3.0)
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            egui::RichText::new("❗")
                                                .color(Color32::from_rgb(255, 32, 0)),
                                        );
                                        ui.label(toast.text.clone().monospace());
                                    })
                                })
                                .response
                        })
                        .custom_contents(ToastKind::Success, &|ui, toast| {
                            egui::Frame::window(ui.style())
                                //.fill()
                                .inner_margin(egui::style::Margin::same(5.0))
                                .rounding(3.0)
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            egui::RichText::new("✔")
                                                .color(Color32::from_rgb(0, 255, 32)),
                                        );
                                        ui.label(toast.text.clone().monospace());
                                    })
                                })
                                .response
                        });
                    //.align_to_end(true);

                    ui.horizontal(|ui| {
                        // ui.menu_button("⛭", |ui|{

                        // });

                        if ui
                            .add(egui::Button::new("⛭"))
                            .on_hover_cursor(egui::CursorIcon::PointingHand)
                            .clicked()
                        {
                            self.settings_frame_open = !self.settings_frame_open;
                        }

                        if self.window_theme {
                            let mut lightmode = egui::Visuals::light();
                            lightmode.window_shadow = egui::epaint::Shadow {
                                extrusion: 0.0,
                                color: Color32::from_rgb(0, 0, 0),
                            };
                            ctx.set_visuals(lightmode);

                            if ui
                                .add(egui::Button::new("☀"))
                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                .clicked()
                            {
                                self.window_theme = false;
                            }
                        } else {
                            let mut darkmode = egui::Visuals::dark();
                            darkmode.window_shadow = egui::epaint::Shadow {
                                extrusion: 0.0,
                                color: Color32::from_rgb(0, 0, 0),
                            };
                            ctx.set_visuals(darkmode);

                            if ui
                                .add(egui::Button::new("🌙"))
                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                .clicked()
                            {
                                self.window_theme = true;
                            }
                        }
                    });

                    ui.add(egui::Separator::spacing(
                        egui::Separator::horizontal(egui::Separator::default()),
                        6.0,
                    ));

                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            let tab1 = ui
                                .selectable_value(&mut self.selected_tab, 1, "Aimbot")
                                .on_hover_cursor(egui::CursorIcon::PointingHand);
                            let tab2 = ui
                                .selectable_value(&mut self.selected_tab, 2, "ESP")
                                .on_hover_cursor(egui::CursorIcon::PointingHand);
                            let tab3 = ui
                                .selectable_value(&mut self.selected_tab, 3, "Configs")
                                .on_hover_cursor(egui::CursorIcon::PointingHand);
                            let tab4 = ui
                                .selectable_value(&mut self.selected_tab, 4, "Lua")
                                .on_hover_cursor(egui::CursorIcon::PointingHand);
                            let tab5 = ui
                                .selectable_value(&mut self.selected_tab, 5, "Misc")
                                .on_hover_cursor(egui::CursorIcon::PointingHand);
                            ui.add_space(150.0);

                            if tab1.clicked()
                                || tab2.clicked()
                                || tab3.clicked()
                                || tab4.clicked()
                                || tab5.clicked()
                            {
                                self.settings_frame_open = false;
                            }
                        });

                        ui.add(egui::Separator::spacing(
                            egui::Separator::vertical(egui::Separator::default()),
                            10.0,
                        ));

                        ui.vertical(|ui| {
                            egui::ScrollArea::both().show(ui, |ui| {
                                if self.settings_frame_open {
                                    self.selected_tab = 0;

                                    //== GUI Settings ==//
                                    ui.heading("GUI Settings");
                                    ui.add(egui::Separator::spacing(
                                        egui::Separator::horizontal(egui::Separator::default()),
                                        6.0,
                                    ));

                                    ui.checkbox(&mut self.window_notifs_enabled, "Notifications")
                                        .on_hover_cursor(egui::CursorIcon::PointingHand);
                                    ui.checkbox(
                                        &mut self.window_tooltips_enabled,
                                        "Widget Tooltips",
                                    )
                                    .on_hover_cursor(egui::CursorIcon::PointingHand);
                                    ui.checkbox(&mut self.window_theme, "Window Light Theme")
                                        .on_hover_cursor(egui::CursorIcon::PointingHand);

                                    // ESP Settings
                                    ui.add_space(SPACING);
                                    ui.heading("ESP Settings");
                                    ui.add(egui::Separator::spacing(
                                        egui::Separator::horizontal(egui::Separator::default()),
                                        6.0,
                                    ));

                                    ui.horizontal(|ui| {
                                        ui.add(
                                            egui::DragValue::new(
                                                &mut *self
                                                    .f32_map
                                                    .entry("esp_box_rounding".to_owned())
                                                    .or_insert(6.0 as f32),
                                            )
                                            .clamp_range(0..=10)
                                            .speed(0.01),
                                        )
                                        .on_hover_cursor(egui::CursorIcon::PointingHand);
                                        ui.label(egui::RichText::new("Box Rounding"));
                                    });

                                    ui.horizontal(|ui| {
                                        ui.add(
                                            egui::DragValue::new(
                                                &mut *self
                                                    .f32_map
                                                    .entry("esp_text_size".to_owned())
                                                    .or_insert(12.0 as f32),
                                            )
                                            .clamp_range(5..=30)
                                            .speed(1),
                                        )
                                        .on_hover_cursor(egui::CursorIcon::PointingHand);
                                        ui.label(egui::RichText::new("ESP Text Size"));
                                    });

                                    ui.checkbox(
                                        &mut *self
                                            .bool_map
                                            .entry("esp_box_filled".to_owned())
                                            .or_insert(false as bool),
                                        "Box Filled",
                                    )
                                    .on_hover_cursor(egui::CursorIcon::PointingHand);

                                    // Aimbot Settings
                                    ui.add_space(SPACING);
                                    ui.heading("Aimbot Settings");

                                    ui.horizontal(|ui| {
                                        ui.add(
                                            egui::DragValue::new(
                                                &mut *self
                                                    .f32_map
                                                    .entry("fov_circle_thickness".to_owned())
                                                    .or_insert(1.0 as f32),
                                            )
                                            .clamp_range(1..=10)
                                            .speed(0.01),
                                        )
                                        .on_hover_cursor(egui::CursorIcon::PointingHand);
                                        ui.label(egui::RichText::new("FOV Circle Thickness"));
                                    });

                                    ui.checkbox(
                                        &mut *self
                                            .bool_map
                                            .entry("fov_circle_filled".to_owned())
                                            .or_insert(false as bool),
                                        "FOV Circle Filled",
                                    )
                                    .on_hover_cursor(egui::CursorIcon::PointingHand);
                                } else {
                                    // Loading Tab Panels
                                    match self.selected_tab {
                                        1 => {
                                            self.draw_aimbot_panel(ui);
                                        }
                                        2 => {
                                            self.draw_esp_panel(ui);
                                        }
                                        3 => {
                                            self.draw_config_panel(ctx, ui, &mut toasts).ok();
                                        }
                                        4 => {
                                            self.draw_lua_panel(ui);
                                        }
                                        5 => {
                                            self.draw_misc_panel(ui);
                                        }

                                        // Invalid Tab Handling
                                        _ => self.selected_tab = 1,
                                    }
                                }
                            });
                        });
                    });

                    if self.window_notifs_enabled {
                        toasts.show()
                    };
                });
        };
    }
} */

pub unsafe fn main_thread(_hinst: usize) {
    eprintln!("Hello World!");

    let methods = directx11::methods().unwrap();

    let present = methods.swapchain_vmt()[8];
    eprintln!("Present: {:X}", present as usize);

    let swap_buffers = methods.swapchain_vmt()[13];
    eprintln!("Buffers: {:X}", swap_buffers as usize);

    let present: FnPresent = std::mem::transmute(methods.swapchain_vmt()[8]);
    let swap_buffers: FnResizeBuffers = std::mem::transmute(methods.swapchain_vmt()[13]);

    PresentHook
        .initialize(present, hk_present)
        .unwrap()
        .enable()
        .unwrap();

    ResizeBufferHook
        .initialize(swap_buffers, hk_resize_buffers)
        .unwrap()
        .enable()
        .unwrap();
}
