use egui::{
    Align2,
    Color32,
    Context,
    FontData,
    FontDefinitions,
    FontFamily,
    FontId,
    FontTweak,
    Key,
    Modifiers,
    Pos2,
    Rect,
    RichText,
    ScrollArea,
    Slider,
    Stroke,
    TextureId,
    TextureOptions,
    Vec2,
    Widget,
    Ui,
};
// TEST
//== Crates ==//
extern crate winreg;
extern crate crossterm;
extern crate winconsole;

//== Use ==//
use std::io;
use std::path::Path;
use winreg::enums::*;
use winreg::RegKey;

use std::collections::HashMap;

use core::time::Duration;

use std::{ thread, time, sync::mpsc };

use crate::offsets;

// use crate::crossterm::style::Stylize;
//use winconsole::console;

//== Main Vars ==//
const REG_NAME: &str = "lock.rs";
const WINDOW_NAME: &str = "lock.rs";
const INFO_CHAR: &str = "ℹ";

const DLL_NAME: &str = "lock.dll";

// Gui Spacing //
const SPACING: f32 = 5.0;
const SMALL_SPACING: f32 = 3.0;
use crate::ADDRESSES;

//== Struct Defaults ==//

//== Draw Toggle Func ==//
fn ui_toggle(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::Vec2::new(2.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    if response.clicked() {
        *on = !*on;
        response.mark_changed();
    }
    response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, *on, ""));

    if ui.is_rect_visible(rect) {
        let how_on = ui.ctx().animate_bool(response.id, *on);
        let visuals = ui.style().interact_selectable(&response, *on);
        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();
        ui.painter().rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
        let circle_x = egui::lerp(rect.left() + radius..=rect.right() - radius, how_on);
        let center = egui::pos2(circle_x, rect.center().y);
        ui.painter().circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
    }

    response
}

pub fn toggle(on: &mut bool) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| ui_toggle(ui, on)
}

//== Main Struct ==//
#[derive(Clone)]
pub struct Lock {
    // Main
    selected_tab: u8,
    window_open: bool,
    window_theme: bool,

    // Settings
    settings_frame_open: bool,
    window_tooltips_enabled: bool,

    // HashMaps
    bool_map: HashMap<String, bool>,
    u32_map: HashMap<String, u32>,
    usize_map: HashMap<String, usize>,
    f32_map: HashMap<String, f32>,
    color_map: HashMap<String, [u8; 4]>,

    // Misc
    unlock_fps: bool,
    fps_limit: u16,
}

//== Struct Defaults ==//
impl Default for Lock {
    fn default() -> Self {
        Self {
            // Main
            selected_tab: 1,
            window_open: true,
            window_theme: false,

            // Settings
            settings_frame_open: false,
            window_tooltips_enabled: true,

            // HashMaps
            bool_map: HashMap::new(),
            u32_map: HashMap::new(),
            usize_map: HashMap::new(),
            f32_map: HashMap::new(),
            color_map: HashMap::new(),

            // Misc
            unlock_fps: false,
            fps_limit: 60,
        }
    }
}

//== Struct Funcs ==//
impl Lock {
    //== Aimbot Panel ==//
    fn draw_aimbot_panel(&mut self, ui: &mut egui::Ui) {
        // Enabled
        ui.horizontal(|ui| {
            ui.add(
                toggle(&mut *self.bool_map.entry("aimbot_enabled".to_owned()).or_insert(false))
            ).on_hover_cursor(egui::CursorIcon::PointingHand);
            ui.label(egui::RichText::new("Aimbot Enabled").strong());
            if self.window_tooltips_enabled {
                ui.label(INFO_CHAR).on_hover_text("Enable Aimlock");
            }

            ui.add_enabled_ui(
                *self.bool_map.entry("aimbot_enabled".to_owned()).or_insert(false),
                |ui| {
                    // If Aimbot On
                    ui.label(egui::RichText::new("X Offset:"));
                    ui.add(
                        egui::DragValue
                            ::new(
                                &mut *self.f32_map
                                    .entry("aimbot_xoffset".to_owned())
                                    .or_insert(0.0 as f32)
                            )
                            .speed(0.01)
                    ).on_hover_cursor(egui::CursorIcon::PointingHand);

                    ui.label(egui::RichText::new("Y Offset:"));
                    ui.add(
                        egui::DragValue
                            ::new(
                                &mut *self.f32_map
                                    .entry("aimbot_yoffset".to_owned())
                                    .or_insert(0.0 as f32)
                            )
                            .speed(0.01)
                    ).on_hover_cursor(egui::CursorIcon::PointingHand);
                }
            );
        });

        // Aimbot Settings
        if *self.bool_map.entry("aimbot_enabled".to_owned()).or_insert(false) {
            ui.horizontal(|ui| {
                ui.checkbox(
                    &mut *self.bool_map
                        .entry("aimbot_teamcheck".to_owned())
                        .or_insert(false as bool),
                    "Teamcheck"
                ).on_hover_cursor(egui::CursorIcon::PointingHand);
                ui.checkbox(
                    &mut *self.bool_map
                        .entry("aimbot_wallcheck".to_owned())
                        .or_insert(false as bool),
                    "Wallcheck"
                ).on_hover_cursor(egui::CursorIcon::PointingHand);
                ui.checkbox(
                    &mut *self.bool_map
                        .entry("aimbot_stickyaim".to_owned())
                        .or_insert(false as bool),
                    "Stinky Aim"
                ).on_hover_cursor(egui::CursorIcon::PointingHand);
            });
            ui.add_space(SPACING);
        }

        ui.add_enabled_ui(*self.bool_map.entry("aimbot_enabled".to_owned()).or_insert(false), |ui| {
            // If Aimbot On

            // Range
            ui.add_space(SPACING);
            ui.horizontal(|ui| {
                ui.add(
                    toggle(
                        &mut *self.bool_map
                            .entry("aimbot_range_enabled".to_owned())
                            .or_insert(false as bool)
                    )
                ).on_hover_cursor(egui::CursorIcon::PointingHand);
                ui.label(egui::RichText::new("Range Limit").strong());
                if self.window_tooltips_enabled {
                    ui.label(INFO_CHAR).on_hover_text(
                        "Enable to add a maximum range at which the aimbot looks for targets"
                    );
                }

                ui.add_enabled_ui(
                    *self.bool_map
                        .entry("aimbot_range_enabled".to_owned())
                        .or_insert(false as bool),
                    |ui| {
                        ui.add(
                            egui::Slider
                                ::new(
                                    &mut *self.u32_map
                                        .entry("aimbot_range".to_owned())
                                        .or_insert(1000 as u32),
                                    0..=5000
                                )
                                .clamp_to_range(false)
                        ).on_hover_cursor(egui::CursorIcon::PointingHand);
                    }
                );
            });

            // FOV
            ui.add_space(SPACING);
            ui.horizontal(|ui| {
                ui.add(
                    toggle(
                        &mut *self.bool_map
                            .entry("aimbot_fov_enabled".to_owned())
                            .or_insert(false as bool)
                    )
                ).on_hover_cursor(egui::CursorIcon::PointingHand);
                ui.label(egui::RichText::new("FOV").strong());
                if self.window_tooltips_enabled {
                    ui.label(INFO_CHAR).on_hover_text(
                        "Limit the screen area in which targets are searched for"
                    );
                }

                ui.add_enabled_ui(
                    *self.bool_map.entry("aimbot_fov_enabled".to_owned()).or_insert(false as bool),
                    |ui| {
                        ui.add(
                            egui::Slider
                                ::new(
                                    &mut *self.u32_map
                                        .entry("aimbot_fov_value".to_owned())
                                        .or_insert(100 as u32),
                                    0..=1000
                                )
                                .clamp_to_range(false)
                        ).on_hover_cursor(egui::CursorIcon::PointingHand);
                        ui.checkbox(
                            &mut *self.bool_map
                                .entry("aimbot_hide_fov".to_owned())
                                .or_insert(false as bool),
                            "Hide Circle"
                        ).on_hover_cursor(egui::CursorIcon::PointingHand);

                        ui.add_enabled_ui(
                            !*self.bool_map
                                .entry("aimbot_hide_fov".to_owned())
                                .or_insert(false as bool),
                            |ui| {
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
                                    ui.color_edit_button_srgba_unmultiplied(
                                        &mut *self.color_map
                                            .entry("aimbot_fov_color".to_owned())
                                            .or_insert([255, 255, 255, 255] as [u8; 4])
                                    ).on_hover_cursor(egui::CursorIcon::PointingHand);
                                });
                            }
                        );
                    }
                );
            });

            // Triggerbot
            ui.add_space(SPACING);
            ui.horizontal(|ui| {
                ui.add(
                    toggle(
                        &mut *self.bool_map
                            .entry("aimbot_triggerbot".to_owned())
                            .or_insert(false as bool)
                    )
                ).on_hover_cursor(egui::CursorIcon::PointingHand);
                ui.label(egui::RichText::new("Triggerbot").strong());
                if self.window_tooltips_enabled {
                    ui.label(INFO_CHAR).on_hover_text(
                        "Automatically shoots when mouse is over a player"
                    );
                }

                ui.add_enabled_ui(
                    *self.bool_map.entry("aimbot_triggerbot".to_owned()).or_insert(false as bool),
                    |ui| {
                        ui.add(
                            egui::Slider
                                ::new(
                                    &mut *self.u32_map
                                        .entry("aimbot_triggerbot_delay".to_owned())
                                        .or_insert(0 as u32),
                                    0..=1000
                                )
                                .suffix("ms")
                                .clamp_to_range(false)
                                .text("Shoot Delay")
                        ).on_hover_cursor(egui::CursorIcon::PointingHand);
                    }
                );
            });

            // Aimbot Extras
            ui.add(
                egui::Separator::spacing(
                    egui::Separator::horizontal(egui::Separator::default()),
                    10.0
                )
            );

            ui.horizontal(|ui| {
                ui.add(
                    egui::Slider
                        ::new(
                            &mut *self.u32_map
                                .entry("aimbot_smoothness".to_owned())
                                .or_insert(0 as u32),
                            0..=100
                        )
                        .text("Aimbot Smoothness")
                ).on_hover_cursor(egui::CursorIcon::PointingHand);
                if self.window_tooltips_enabled {
                    ui.label(INFO_CHAR).on_hover_text("Makes lock-on more smooth");
                }

                ui.add_enabled_ui(
                    *self.u32_map.entry("aimbot_smoothness".to_owned()).or_insert(0 as u32) > 0,
                    |ui| {
                        // If Aimbot On
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
                            egui::ComboBox
                                ::from_id_source("aimbot_smoothness_type")
                                .selected_text(
                                    format!(
                                        "{:?}",
                                        ["Linear", "Ease in", "Ease out", "Smooth"]
                                            [
                                                *self.usize_map
                                                    .entry("aimbot_smoothness_type".to_owned())
                                                    .or_insert(0 as usize)
                                            ]
                                    )
                                )
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut *self.usize_map
                                            .entry("aimbot_smoothness_type".to_owned())
                                            .or_insert(0 as usize),
                                        0,
                                        "Linear"
                                    ).on_hover_cursor(egui::CursorIcon::PointingHand);
                                    ui.selectable_value(
                                        &mut *self.usize_map
                                            .entry("aimbot_smoothness_type".to_owned())
                                            .or_insert(0 as usize),
                                        1,
                                        "Ease in"
                                    ).on_hover_cursor(egui::CursorIcon::PointingHand);
                                    ui.selectable_value(
                                        &mut *self.usize_map
                                            .entry("aimbot_smoothness_type".to_owned())
                                            .or_insert(0 as usize),
                                        2,
                                        "Ease out"
                                    ).on_hover_cursor(egui::CursorIcon::PointingHand);
                                    ui.selectable_value(
                                        &mut *self.usize_map
                                            .entry("aimbot_smoothness_type".to_owned())
                                            .or_insert(0 as usize),
                                        3,
                                        "Smooth"
                                    ).on_hover_cursor(egui::CursorIcon::PointingHand);
                                });
                        });
                    }
                );
            });

            ui.add_space(SMALL_SPACING);

            ui.horizontal(|ui| {
                ui.add(
                    egui::Slider
                        ::new(
                            &mut *self.u32_map
                                .entry("aimbot_prediction".to_owned())
                                .or_insert(0 as u32),
                            0..=100
                        )
                        .text("Prediction Strength")
                ).on_hover_cursor(egui::CursorIcon::PointingHand);
                if self.window_tooltips_enabled {
                    ui.label(INFO_CHAR).on_hover_text("Aims ahead of players");
                }
            });

            ui.add_space(SPACING);

            ui.horizontal(|ui| {
                egui::ComboBox
                    ::from_label("Target Priority")
                    .selected_text(
                        format!(
                            "{:?}",
                            ["Distance", "Cursor"]
                                [
                                    *self.usize_map
                                        .entry("aimbot_type".to_owned())
                                        .or_insert(0 as usize)
                                ]
                        )
                    )
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut *self.usize_map
                                .entry("aimbot_type".to_owned())
                                .or_insert(0 as usize),
                            0,
                            "Distance"
                        ).on_hover_cursor(egui::CursorIcon::PointingHand);
                        ui.selectable_value(
                            &mut *self.usize_map
                                .entry("aimbot_type".to_owned())
                                .or_insert(0 as usize),
                            1,
                            "Cursor"
                        ).on_hover_cursor(egui::CursorIcon::PointingHand);
                    });
                if self.window_tooltips_enabled {
                    ui.label(INFO_CHAR).on_hover_text("Canges aimbot priority");
                }
            });

            ui.add_space(SMALL_SPACING);

            ui.horizontal(|ui| {
                egui::ComboBox
                    ::from_label("Target Aimpart")
                    .selected_text(
                        format!(
                            "{:?}",
                            ["Head", "Torso"]
                                [
                                    *self.usize_map
                                        .entry("aimbot_aimpart".to_owned())
                                        .or_insert(0 as usize)
                                ]
                        )
                    )
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut *self.usize_map
                                .entry("aimbot_aimpart".to_owned())
                                .or_insert(0 as usize),
                            0,
                            "Head"
                        ).on_hover_cursor(egui::CursorIcon::PointingHand);
                        ui.selectable_value(
                            &mut *self.usize_map
                                .entry("aimbot_aimpart".to_owned())
                                .or_insert(0 as usize),
                            1,
                            "Torso"
                        ).on_hover_cursor(egui::CursorIcon::PointingHand);
                    });
                if self.window_tooltips_enabled {
                    ui.label(INFO_CHAR).on_hover_text("Changes the targeted bodypart");
                }
            });
        });
    }

    //== ESP Panel ==//
    fn draw_esp_panel(&mut self, ui: &mut Ui) {
        // ESP Enabled
        ui.horizontal(|ui| {
            ui.add(
                toggle(&mut *self.bool_map.entry("esp_enabled".to_owned()).or_insert(false))
            ).on_hover_cursor(egui::CursorIcon::PointingHand);
            ui.label(egui::RichText::new("ESP Enabled").strong());

            ui.add_enabled_ui(
                *self.bool_map.entry("esp_enabled".to_owned()).or_insert(false),
                |ui| {
                    // If ESP On
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
                        ui.add_enabled_ui(
                            !*self.bool_map
                                .entry("esp_team_based_color".to_owned())
                                .or_insert(false),
                            |ui| {
                                // If ESP On
                                ui.color_edit_button_srgba_unmultiplied(
                                    &mut *self.color_map
                                        .entry("esp_color".to_owned())
                                        .or_insert([255, 255, 255, 255] as [u8; 4])
                                ).on_hover_cursor(egui::CursorIcon::PointingHand);
                            }
                        );

                        egui::ComboBox
                            ::from_id_source("esp_type")
                            .selected_text(
                                format!(
                                    "{:?}",
                                    ["None", "2D Box", "3D Box", "Corners"]
                                        [
                                            *self.usize_map
                                                .entry("esp_type".to_owned())
                                                .or_insert(1 as usize)
                                        ]
                                )
                            )
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut *self.usize_map
                                        .entry("esp_type".to_owned())
                                        .or_insert(1 as usize),
                                    0,
                                    "None"
                                ).on_hover_cursor(egui::CursorIcon::PointingHand);
                                ui.selectable_value(
                                    &mut *self.usize_map
                                        .entry("esp_type".to_owned())
                                        .or_insert(1 as usize),
                                    1,
                                    "2D Box"
                                ).on_hover_cursor(egui::CursorIcon::PointingHand);
                                ui.selectable_value(
                                    &mut *self.usize_map
                                        .entry("esp_type".to_owned())
                                        .or_insert(1 as usize),
                                    2,
                                    "3D Box"
                                ).on_hover_cursor(egui::CursorIcon::PointingHand);
                                ui.selectable_value(
                                    &mut *self.usize_map
                                        .entry("esp_type".to_owned())
                                        .or_insert(1 as usize),
                                    3,
                                    "Corners"
                                ).on_hover_cursor(egui::CursorIcon::PointingHand);
                            });
                    });
                }
            );
        });

        ui.add_enabled_ui(*self.bool_map.entry("esp_enabled".to_owned()).or_insert(false), |ui| {
            // If ESP On

            // ESP Types
            if *self.bool_map.entry("esp_enabled".to_owned()).or_insert(false) {
                ui.horizontal(|ui| {
                    ui.checkbox(
                        &mut *self.bool_map.entry("esp_names".to_owned()).or_insert(false as bool),
                        "Show Names"
                    ).on_hover_cursor(egui::CursorIcon::PointingHand);
                    ui.checkbox(
                        &mut *self.bool_map
                            .entry("esp_distance".to_owned())
                            .or_insert(false as bool),
                        "Show Distance"
                    ).on_hover_cursor(egui::CursorIcon::PointingHand);
                });
                ui.add_space(SPACING);
            }

            {
                // Teamcheck
                ui.add_space(SPACING);

                ui.horizontal(|ui| {
                    ui.add(
                        toggle(
                            &mut *self.bool_map
                                .entry("esp_teamcheck".to_owned())
                                .or_insert(false as bool)
                        )
                    ).on_hover_cursor(egui::CursorIcon::PointingHand);
                    ui.label(egui::RichText::new("Teamcheck").strong());
                });

                ui.add_enabled_ui(
                    *self.bool_map.entry("esp_teamcheck".to_owned()).or_insert(false as bool),
                    |ui| {
                        // If Teamcheck On

                        // ESP Types
                        if
                            *self.bool_map
                                .entry("esp_teamcheck".to_owned())
                                .or_insert(false as bool)
                        {
                            ui.horizontal(|ui| {
                                ui.checkbox(
                                    &mut *self.bool_map
                                        .entry("esp_hide_team".to_owned())
                                        .or_insert(false as bool),
                                    "Hide Team Members"
                                ).on_hover_cursor(egui::CursorIcon::PointingHand);
                                ui.checkbox(
                                    &mut *self.bool_map
                                        .entry("show_team_names".to_owned())
                                        .or_insert(false as bool),
                                    "Show Team Names"
                                ).on_hover_cursor(egui::CursorIcon::PointingHand);
                                ui.checkbox(
                                    &mut *self.bool_map
                                        .entry("esp_team_based_color".to_owned())
                                        .or_insert(false as bool),
                                    "Team Based ESP Color"
                                ).on_hover_cursor(egui::CursorIcon::PointingHand);
                            });
                            ui.add_space(SPACING);
                        }
                    }
                );
            }

            {
                // Wallcheck
                ui.horizontal(|ui| {
                    ui.add(
                        toggle(
                            &mut *self.bool_map
                                .entry("esp_wallcheck_enabled".to_owned())
                                .or_insert(false as bool)
                        )
                    ).on_hover_cursor(egui::CursorIcon::PointingHand);
                    ui.label(egui::RichText::new("Wallcheck").strong());

                    // ui.add_enabled_ui(*self.bool_map.entry("esp_wallcheck_enabled".to_owned()).or_insert(false as bool), |ui| { // If ESP On
                    //     ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {

                    //         egui::ComboBox::from_id_source("esp_type")
                    //         .selected_text(format!("{:?}", (WALLCHECK_TYPES)[*self.usize_map.entry("esp_wallcheck_type".to_owned()).or_insert(0 as usize)]))
                    //         .show_ui(ui, |ui| {
                    //             ui.selectable_value(&mut *self.usize_map.entry("esp_wallcheck_type".to_owned()).or_insert(0 as usize), 0, "Only Show Visible").on_hover_cursor(egui::CursorIcon::PointingHand);
                    //             ui.selectable_value(&mut *self.usize_map.entry("esp_wallcheck_type".to_owned()).or_insert(0 as usize), 1, "Highlight Visible").on_hover_cursor(egui::CursorIcon::PointingHand);
                    //             ui.selectable_value(&mut *self.usize_map.entry("esp_wallcheck_type".to_owned()).or_insert(0 as usize), 2, "Highlight Invisible").on_hover_cursor(egui::CursorIcon::PointingHand);
                    //         });

                    //         ui.add_enabled_ui(*self.usize_map.entry("esp_wallcheck_type".to_owned()).or_insert(0 as usize) == 1 || *self.usize_map.entry("esp_wallcheck_type".to_owned()).or_insert(0 as usize) == 2, |ui| { // If "Team Based ESP Color" Off
                    //             ui.color_edit_button_srgba_unmultiplied(&mut *self.color_map.entry("esp_wallcheck_color".to_owned()).or_insert([255,255,255,255] as [u8;4])).on_hover_cursor(egui::CursorIcon::PointingHand);
                    //         });
                    //     });
                    // });
                });
            }

            // Tracers
            ui.add_space(SPACING);

            ui.horizontal(|ui| {
                ui.add(
                    toggle(
                        &mut *self.bool_map
                            .entry("esp_tracers_enabled".to_owned())
                            .or_insert(false as bool)
                    )
                ).on_hover_cursor(egui::CursorIcon::PointingHand);
                ui.label(egui::RichText::new("Tracers").strong());

                // If enabled
                ui.add_enabled_ui(
                    *self.bool_map.entry("esp_tracers_enabled".to_owned()).or_insert(false as bool),
                    |ui| {
                        // If Tracers On
                        ui.add_space(10.0);
                        ui.checkbox(
                            &mut *self.bool_map
                                .entry("esp_tracers_distance_based".to_owned())
                                .or_insert(false as bool),
                            "Distance Based Color"
                        ).on_hover_cursor(egui::CursorIcon::PointingHand);

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
                            ui.add_enabled_ui(
                                !*self.bool_map
                                    .entry("esp_tracers_distance_based".to_owned())
                                    .or_insert(false as bool),
                                |ui| {
                                    ui.color_edit_button_srgba_unmultiplied(
                                        &mut *self.color_map
                                            .entry("esp_tracers_color".to_owned())
                                            .or_insert([255, 255, 255, 255] as [u8; 4])
                                    ).on_hover_cursor(egui::CursorIcon::PointingHand);
                                }
                            );

                            egui::ComboBox
                                ::from_id_source("tracer_type")
                                .selected_text(
                                    format!(
                                        "{:?}",
                                        ["Top", "Middle", "Bottom Middle", "Bottom", "Mouse"]
                                            [
                                                *self.usize_map
                                                    .entry("esp_tracers_type".to_owned())
                                                    .or_insert(0 as usize)
                                            ]
                                    )
                                )
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut *self.usize_map
                                            .entry("esp_tracers_type".to_owned())
                                            .or_insert(0 as usize),
                                        0,
                                        "Top"
                                    ).on_hover_cursor(egui::CursorIcon::PointingHand);
                                    ui.selectable_value(
                                        &mut *self.usize_map
                                            .entry("esp_tracers_type".to_owned())
                                            .or_insert(0 as usize),
                                        1,
                                        "Middle"
                                    ).on_hover_cursor(egui::CursorIcon::PointingHand);
                                    ui.selectable_value(
                                        &mut *self.usize_map
                                            .entry("esp_tracers_type".to_owned())
                                            .or_insert(0 as usize),
                                        2,
                                        "Bottom Middle"
                                    ).on_hover_cursor(egui::CursorIcon::PointingHand);
                                    ui.selectable_value(
                                        &mut *self.usize_map
                                            .entry("esp_tracers_type".to_owned())
                                            .or_insert(0 as usize),
                                        3,
                                        "Bottom"
                                    ).on_hover_cursor(egui::CursorIcon::PointingHand);
                                    ui.selectable_value(
                                        &mut *self.usize_map
                                            .entry("esp_tracers_type".to_owned())
                                            .or_insert(0 as usize),
                                        4,
                                        "Mouse"
                                    ).on_hover_cursor(egui::CursorIcon::PointingHand);
                                });
                        });
                    }
                );
            });

            // Health
            ui.add_space(SPACING);

            ui.horizontal(|ui| {
                ui.add(
                    toggle(
                        &mut *self.bool_map
                            .entry("esp_show_health".to_owned())
                            .or_insert(false as bool)
                    )
                ).on_hover_cursor(egui::CursorIcon::PointingHand);
                ui.label(egui::RichText::new("Show Player Health").strong());

                ui.add_enabled_ui(
                    *self.bool_map.entry("esp_show_health".to_owned()).or_insert(false as bool),
                    |ui| {
                        // If Show Health On
                        ui.add_space(10.0);

                        ui.checkbox(
                            &mut *self.bool_map
                                .entry("esp_health_bar".to_owned())
                                .or_insert(false as bool),
                            "Health Bar"
                        ).on_hover_cursor(egui::CursorIcon::PointingHand);
                        ui.checkbox(
                            &mut *self.bool_map
                                .entry("esp_health_text".to_owned())
                                .or_insert(false as bool),
                            "Health Text"
                        ).on_hover_cursor(egui::CursorIcon::PointingHand);
                    }
                );
            });

            // Distance Limit
            ui.add_space(SPACING);

            ui.horizontal(|ui| {
                ui.add(
                    toggle(
                        &mut *self.bool_map
                            .entry("esp_distance_limited".to_owned())
                            .or_insert(false as bool)
                    )
                ).on_hover_cursor(egui::CursorIcon::PointingHand);
                ui.label(egui::RichText::new("Distance Limited").strong());

                // If enabled
                ui.add_enabled_ui(
                    *self.bool_map
                        .entry("esp_distance_limited".to_owned())
                        .or_insert(false as bool),
                    |ui| {
                        // If Show Health On
                        ui.add_space(10.0);

                        ui.add(
                            egui::Slider
                                ::new(
                                    &mut *self.u32_map
                                        .entry("esp_distance_limit".to_owned())
                                        .or_insert(1000 as u32),
                                    0..=5000
                                )
                                .clamp_to_range(false)
                        ).on_hover_cursor(egui::CursorIcon::PointingHand);
                    }
                );
            });
        });

        ui.add(
            egui::Separator::spacing(egui::Separator::horizontal(egui::Separator::default()), 10.0)
        );
    }

    //== Misc Panel ==//
    fn draw_misc_panel(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.add(toggle(&mut self.unlock_fps)).on_hover_cursor(egui::CursorIcon::PointingHand);
            ui.label(egui::RichText::new("Unlock FPS").strong());
            if self.window_tooltips_enabled {
                ui.label(INFO_CHAR).on_hover_text("Disables the Roblox FPS cap");
            }

            ui.add_enabled_ui(self.unlock_fps, |ui| {
                // If Range On
                ui.add(
                    egui::Slider::new(&mut self.fps_limit, 0..=1000).clamp_to_range(false)
                ).on_hover_cursor(egui::CursorIcon::PointingHand);
            });
        });
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut i32) {
        //== Rendering ESP ==//
        let pointer_pos = ctx.pointer_latest_pos();
        if pointer_pos.is_some() {
            // FOV Circle
            if
                !*self.bool_map.entry("aimbot_hide_fov".to_owned()).or_insert(false as bool) &&
                *self.bool_map.entry("aimbot_fov_enabled".to_owned()).or_insert(false as bool) &&
                *self.bool_map.entry("aimbot_enabled".to_owned()).or_insert(false as bool)
            {
                let fov_clr = *self.color_map
                    .entry("aimbot_fov_color".to_owned())
                    .or_insert([255, 255, 255, 255] as [u8; 4]);
                let pointer_pos_with_offset = Pos2::new(
                    pointer_pos.unwrap().x +
                        *self.f32_map.entry("aimbot_xoffset".to_owned()).or_insert(0.0 as f32),
                    pointer_pos.unwrap().y +
                        *self.f32_map.entry("aimbot_yoffset".to_owned()).or_insert(0.0 as f32)
                );

                let fov_color = Color32::from_rgba_unmultiplied(
                    fov_clr[0],
                    fov_clr[1],
                    fov_clr[2],
                    fov_clr[3]
                );
                if
                    *self.bool_map.entry("fov_circle_filled".to_owned()).or_insert(false as bool) ==
                    true
                {
                    ctx.debug_painter().circle(
                        pointer_pos_with_offset,
                        *self.u32_map
                            .entry("aimbot_fov_value".to_owned())
                            .or_insert(100 as u32) as f32,
                            fov_color,
                        egui::Stroke::new(
                            *self.f32_map
                                .entry("fov_circle_thickness".to_owned())
                                .or_insert(1.0 as f32),
                                fov_color
                        )
                    );
                } else {
                    ctx.debug_painter().circle_stroke(
                        pointer_pos_with_offset,
                        *self.u32_map
                            .entry("aimbot_fov_value".to_owned())
                            .or_insert(100 as u32) as f32,
                        egui::Stroke::new(
                            *self.f32_map
                                .entry("fov_circle_thickness".to_owned())
                                .or_insert(1.5 as f32),
                                fov_color
                        )
                    );
                }
            }
        }

        unsafe {
            for player in offsets.get_children(ADDRESSES.players) {
                if player == 0 {
                    continue;
                }
                let character = offsets.get_character(player);
                if character == 0 {
                    continue;
                }
                let head = offsets.find_first_child(character, "Head");
                if head == 0 {
                    continue;
                }
                let pos = offsets.get_position(head);

                let cool = offsets.world2screen(pos);

                let cooo2 = egui::Pos2::new(cool.x, cool.y);

                let rect = egui::Rect::from_two_pos(cooo2, cooo2);

                let mut distance = *self.u32_map
                    .entry("temp_debug_slider2".to_owned())
                    .or_insert(69 as u32);
                let name = "Player";

                // Tracers
                if *self.bool_map.entry("esp_tracers_enabled".to_owned()).or_insert(false as bool) {
                    let mut tracer_clr = *self.color_map
                        .entry("esp_tracers_color".to_owned())
                        .or_insert([255, 255, 255, 255] as [u8; 4]);

                    if
                        *self.bool_map
                            .entry("esp_tracers_distance_based".to_owned())
                            .or_insert(false as bool)
                    {
                        let mut max_dist = 1000;
                        if
                            *self.bool_map
                                .entry("esp_distance_limited".to_owned())
                                .or_insert(false as bool)
                        {
                            max_dist = *self.u32_map
                                .entry("esp_distance_limit".to_owned())
                                .or_insert(1000 as u32);
                        }

                        let new_dist = if distance == 0 { 1 } else { distance };

                        let green = 255.0 - 255.0 / ((max_dist as f32) / (new_dist as f32));
                        let red = 255.0 / ((max_dist as f32) / (new_dist as f32));

                        tracer_clr = [red as u8, green as u8, 0, 255];
                    }
                    let tracer_color = Color32::from_rgba_premultiplied(
                        tracer_clr[0],
                        tracer_clr[1],
                        tracer_clr[2],
                        tracer_clr[3]
                    );
                    match
                        *self.usize_map.entry("esp_tracers_type".to_owned()).or_insert(0 as usize)
                    {
                        0 => {
                            // TOP
                            ctx.debug_painter().line_segment(
                                [rect.center_bottom(), ctx.available_rect().center_top()],
                                egui::Stroke::new(1.0, tracer_color)
                            );
                        }
                        1 => {
                            // Middle
                            ctx.debug_painter().line_segment(
                                [rect.center_bottom(), ctx.available_rect().center()],
                                egui::Stroke::new(1.0, tracer_color)
                            );
                        }
                        2 => {
                            // Bottom Middle
                            ctx.debug_painter().line_segment(
                                [
                                    rect.center_bottom(),
                                    ctx.available_rect().center_bottom() -
                                        Vec2::new(0.0, ctx.available_rect().height() / 5.0),
                                ],
                                egui::Stroke::new(1.0, tracer_color)
                            );
                        }
                        3 => {
                            // Bottom
                            ctx.debug_painter().line_segment(
                                [rect.center_bottom(), ctx.available_rect().center_bottom()],
                                egui::Stroke::new(1.0, tracer_color)
                            );
                        }
                        4 => {
                            if pointer_pos.is_some() {
                                ctx.debug_painter().line_segment(
                                    [rect.center_bottom(), pointer_pos.unwrap()],
                                    egui::Stroke::new(1.0, tracer_color)
                                );
                            }
                        }

                        _ => {
                            // Overflow
                            ctx.debug_painter().line_segment(
                                [rect.center_bottom(), ctx.available_rect().center_top()],
                                egui::Stroke::new(1.0, tracer_color)
                            );
                        }
                    }
                }
            }

            // Player ESP

            egui::Window
                ::new(WINDOW_NAME)
                .resizable(false)
                .fixed_size(egui::Vec2::new(700.0, 500.0))
                .show(ctx, |ui| {
                    //== Main Frame ==//
                    // let win = egui::Window::new(WINDOW_NAME)
                    //     .scroll2([true, true])
                    //     .default_size(Vec2::new(600.0, 9000.0))
                    //     .title_bar(true)
                    //     .resizable(false);

                    //.align_to_end(true);

                    ui.horizontal(|ui| {
                        // ui.menu_button("⛭", |ui|{

                        // });

                        if
                            ui
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

                            if
                                ui
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

                            if
                                ui
                                    .add(egui::Button::new("🌙"))
                                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                                    .clicked()
                            {
                                self.window_theme = true;
                            }
                        }
                    });

                    ui.add(
                        egui::Separator::spacing(
                            egui::Separator::horizontal(egui::Separator::default()),
                            6.0
                        )
                    );

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

                            if
                                tab1.clicked() ||
                                tab2.clicked() ||
                                tab3.clicked() ||
                                tab4.clicked() ||
                                tab5.clicked()
                            {
                                self.settings_frame_open = false;
                            }
                        });

                        ui.add(
                            egui::Separator::spacing(
                                egui::Separator::vertical(egui::Separator::default()),
                                10.0
                            )
                        );

                        ui.vertical(|ui| {
                            egui::ScrollArea::both().show(ui, |ui| {
                                if self.settings_frame_open {
                                    self.selected_tab = 0;

                                    //== GUI Settings ==//
                                    ui.heading("GUI Settings");
                                    ui.add(
                                        egui::Separator::spacing(
                                            egui::Separator::horizontal(egui::Separator::default()),
                                            6.0
                                        )
                                    );

                                    ui.checkbox(
                                        &mut self.window_tooltips_enabled,
                                        "Widget Tooltips"
                                    ).on_hover_cursor(egui::CursorIcon::PointingHand);
                                    ui.checkbox(
                                        &mut self.window_theme,
                                        "Window Light Theme"
                                    ).on_hover_cursor(egui::CursorIcon::PointingHand);

                                    // ESP Settings
                                    ui.add_space(SPACING);
                                    ui.heading("ESP Settings");
                                    ui.add(
                                        egui::Separator::spacing(
                                            egui::Separator::horizontal(egui::Separator::default()),
                                            6.0
                                        )
                                    );

                                    ui.horizontal(|ui| {
                                        ui.add(
                                            egui::DragValue
                                                ::new(
                                                    &mut *self.f32_map
                                                        .entry("esp_box_rounding".to_owned())
                                                        .or_insert(6.0 as f32)
                                                )
                                                .clamp_range(0..=10)
                                                .speed(0.01)
                                        ).on_hover_cursor(egui::CursorIcon::PointingHand);
                                        ui.label(egui::RichText::new("Box Rounding"));
                                    });

                                    ui.horizontal(|ui| {
                                        ui.add(
                                            egui::DragValue
                                                ::new(
                                                    &mut *self.f32_map
                                                        .entry("esp_text_size".to_owned())
                                                        .or_insert(12.0 as f32)
                                                )
                                                .clamp_range(5..=30)
                                                .speed(1)
                                        ).on_hover_cursor(egui::CursorIcon::PointingHand);
                                        ui.label(egui::RichText::new("ESP Text Size"));
                                    });

                                    ui.checkbox(
                                        &mut *self.bool_map
                                            .entry("esp_box_filled".to_owned())
                                            .or_insert(false as bool),
                                        "Box Filled"
                                    ).on_hover_cursor(egui::CursorIcon::PointingHand);

                                    // Aimbot Settingsdraw_aimbot_panelup
                                    ui.add_space(SPACING);
                                    ui.heading("Aimbot Settings");

                                    ui.horizontal(|ui| {
                                        ui.add(
                                            egui::DragValue
                                                ::new(
                                                    &mut *self.f32_map
                                                        .entry("fov_circle_thickness".to_owned())
                                                        .or_insert(1.0 as f32)
                                                )
                                                .clamp_range(1..=10)
                                                .speed(0.01)
                                        ).on_hover_cursor(egui::CursorIcon::PointingHand);
                                        ui.label(egui::RichText::new("FOV Circle Thickness"));
                                    });

                                    ui.checkbox(
                                        &mut *self.bool_map
                                            .entry("fov_circle_filled".to_owned())
                                            .or_insert(false as bool),
                                        "FOV Circle Filled"
                                    ).on_hover_cursor(egui::CursorIcon::PointingHand);
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
                                            self.draw_misc_panel(ui);
                                        }

                                        // Invalid Tab Handling
                                        _ => {
                                            self.selected_tab = 1;
                                        }
                                    }
                                }
                            });
                        });
                    });
                });
        }
    }
}

/* //== Main Render Loop ==//
impl Clone for Lock {
    fn clone(&self) -> Self {
        Lock { data: self.data }
    }
}
 */

/* pub unsafe fn init_ui() {
/*     let mut cool = Lock::default();
    let cloned_data = cool::clone();
    LOCK = Some(&mut cloned_data); */
}

static LOCK: Lock = Lock.clone();

pub fn ui(ctx: &Context, i: &mut i32) {
    unsafe {
        
        LOCK.update(ctx, i);
    }
} */
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref LOCK: Mutex<Lock> = Mutex::new(Lock::default());
}

pub fn ui(ctx: &Context, i: &mut i32) {
    unsafe {
        LOCK.lock().unwrap().update(ctx, i);
    }
}