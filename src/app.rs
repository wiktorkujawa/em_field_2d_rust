use egui::{ ScrollArea, RichText };

use crate::{ structs::{ Route, Interface, CalcResult }, enums::LineType, components };

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    // this how you opt-out of serialization of a member
    routes: Vec<Route>,
    interface: Interface,
    result: CalcResult
    // #[serde(skip)]
    // routes: Vec<Route>,
    // interface: Interface,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            routes: vec![],
            interface: Interface::default(),
            result: CalcResult::default()
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { routes, interface, result } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {

                egui::widgets::global_dark_light_mode_buttons(ui);

                components::line_choice(ui, interface);
                components::electric_details(ui, interface);
                components::phase_order_choice(ui, interface);
                if matches!(interface.line_type, LineType::PowerLine) {
                    components::bundle_specification(ui, interface);
                }
                components::arrangement_method(ui, interface);
                components::coordinates(ui, interface);

                ui.separator();
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    if ui.add(egui::Button::new(RichText::new("Add route").size(20.0))).clicked() {
                        let route = Route::new(interface);
                        routes.push(route);
                    }
                });

                if routes.len() > 0 {
                    components::routes(ui, routes);
                }

                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        ui.label("powered by ");
                        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                        ui.label(" and ");
                        ui.hyperlink_to(
                            "eframe",
                            "https://github.com/emilk/egui/tree/master/crates/eframe"
                        );
                        ui.label(".");
                    });
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_min_width(600.0);
            // The central panel the region left after adding TopPanel's and SidePanel's
            components::plot(ui, result);

            ui.group(|ui| {
                ui.horizontal(|ui| {
                    let Interface { x_range, y_range, .. } = interface;

                    components::range_inputs(ui, *x_range, *y_range);

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if
                            ui
                                .add(egui::Button::new(RichText::new("Generate plot").size(20.0)))
                                .clicked()
                        {
                            result.new(*x_range, *y_range, routes);
                        }
                    });
                });
            });

            egui::warn_if_debug_build(ui);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}
