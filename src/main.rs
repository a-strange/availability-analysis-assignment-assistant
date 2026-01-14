use eframe::egui;
use rand::seq::SliceRandom;

// Version is injected at build time by build.rs
const VERSION: &str = env!("APP_VERSION");

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([350.0, 550.0])
            .with_resizable(true),
        ..Default::default()
    };
    eframe::run_native(
        "Availability Analysis Assignment Assistant",
        options,
        Box::new(|cc| {
            // Load Manrope-Medium font
            let mut fonts = egui::FontDefinitions::default();
            fonts.font_data.insert(
                "manrope".to_owned(),
                egui::FontData::from_static(include_bytes!("../assets/Manrope-Medium.ttf")).into(),
            );
            fonts
                .families
                .get_mut(&egui::FontFamily::Proportional)
                .unwrap()
                .insert(0, "manrope".to_owned());
            cc.egui_ctx.set_fonts(fonts);

            cc.egui_ctx.set_theme(egui::Theme::Light);

            // Customize colors
            let mut style = (*cc.egui_ctx.style()).clone();
            let button_color = egui::Color32::from_rgb(231, 230, 255);
            style.visuals.widgets.inactive.weak_bg_fill = button_color;
            style.visuals.widgets.inactive.bg_fill = button_color;
            style.visuals.widgets.hovered.weak_bg_fill = button_color;
            style.visuals.widgets.hovered.bg_fill = button_color;
            style.visuals.widgets.active.weak_bg_fill = button_color;
            style.visuals.widgets.active.bg_fill = button_color;
            style.visuals.extreme_bg_color = button_color;
            style.visuals.window_fill = egui::Color32::WHITE;
            style.visuals.override_text_color = Some(egui::Color32::from_rgb(6, 6, 6));
            cc.egui_ctx.set_style(style);

            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<AnalysisAssistant>::default())
        }),
    )
}

struct Service {
    name: String,
    importance: u8,
}

struct AnalysisAssistant {
    names: Vec<(String, bool)>,
    display_text: String,
    services: Vec<Service>,
    detailed_mode: bool,
    logo_texture: Option<egui::TextureHandle>,
}

impl AnalysisAssistant {}

impl Default for AnalysisAssistant {
    fn default() -> Self {
        Self {
            names: vec![
                ("Dane".to_string(), true),
                ("Dhandapani".to_string(), true),
                ("Li Chien".to_string(), true),
                ("Andrea".to_string(), true),
                ("Andrew".to_string(), true),
                ("Michael".to_string(), true),
            ],
            display_text: String::new(),
            services: vec![
                Service {
                    name: "verification-api".to_string(),
                    importance: 1,
                },
                Service {
                    name: "connections-api".to_string(),
                    importance: 1,
                },
                Service {
                    name: "auth-server".to_string(),
                    importance: 1,
                },
                Service {
                    name: "signup-plus".to_string(),
                    importance: 1,
                },
                Service {
                    name: "resource-api".to_string(),
                    importance: 1,
                },
                Service {
                    name: "payouts-flow-orchestrator".to_string(),
                    importance: 1,
                },
                Service {
                    name: "clients-api".to_string(),
                    importance: 1,
                },
            ],
            detailed_mode: false,
            logo_texture: None,
        }
    }
}

impl eframe::App for AnalysisAssistant {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Load logo texture once
        if self.logo_texture.is_none() {
            let logo_bytes = include_bytes!("../assets/logo.png");
            let image = image::load_from_memory(logo_bytes).unwrap();
            let size = [image.width() as _, image.height() as _];
            let image_buffer = image.to_rgba8();
            let pixels = image_buffer.as_flat_samples();
            let color_image = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
            self.logo_texture = Some(ctx.load_texture("logo", color_image, Default::default()));
        }

        egui::Window::new("Availability Analysis Assignment Assistant")
            .auto_sized()
            .resizable(true)
            .title_bar(false)
            .show(ctx, |ui| {
                egui::Frame::none().inner_margin(5.0).show(ui, |ui| {
                    // Draw logo in background at 40% of window width, centered over Team Members
                    if let Some(texture) = &self.logo_texture {
                        let available_rect = ui.available_rect_before_wrap();
                        let window_width = available_rect.width();

                        // Calculate logo size maintaining aspect ratio
                        let logo_width = window_width * 0.4;
                        let texture_size = texture.size_vec2();
                        let aspect_ratio = texture_size.y / texture_size.x;
                        let logo_height = logo_width * aspect_ratio;

                        // Center the logo horizontally, position at top of Team Members section
                        let logo_x = available_rect.left() + (window_width - logo_width) / 2.0;
                        let logo_y = available_rect.top() + 60.0; // Offset below heading

                        let logo_rect = egui::Rect::from_min_size(
                            egui::pos2(logo_x, logo_y),
                            egui::vec2(logo_width, logo_height),
                        );

                        let mut mesh = egui::Mesh::with_texture(texture.id());
                        mesh.add_rect_with_uv(
                            logo_rect,
                            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                            egui::Color32::from_rgba_unmultiplied(255, 255, 255, 255),
                        );
                        ui.painter().add(egui::Shape::mesh(mesh));
                    }

                    ui.heading("Team Members");
                    for (name, enabled) in &mut self.names {
                        ui.horizontal(|ui| {
                            ui.with_layout(
                                egui::Layout::left_to_right(egui::Align::Center),
                                |ui| {
                                    ui.label(name.to_string());
                                },
                            );
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    ui.checkbox(enabled, "");
                                },
                            );
                        });
                    }

                    ui.add_space(20.0);
                    ui.heading("Services");

                    for service in &mut self.services {
                        ui.horizontal(|ui| {
                            ui.set_min_height(20.0);
                            ui.with_layout(
                                egui::Layout::left_to_right(egui::Align::Center),
                                |ui| {
                                    ui.label(&service.name);
                                },
                            );
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if self.detailed_mode {
                                        // Detailed mode: slider (0-3 importance)
                                        ui.add(egui::Slider::new(&mut service.importance, 0..=3));
                                    } else {
                                        // Basic mode: checkbox (0 or 1 importance)
                                        let mut is_selected = service.importance > 0;
                                        if ui.checkbox(&mut is_selected, "").changed() {
                                            service.importance = if is_selected { 1 } else { 0 };
                                        }
                                    }
                                },
                            );
                        });
                    }

                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        if ui.button("Assign Services").clicked() {
                            let mut enabled_names: Vec<&str> = self
                                .names
                                .iter()
                                .filter(|(_, enabled)| *enabled)
                                .map(|(name, _)| name.as_str())
                                .collect();

                            let mut selected_services: Vec<String> = self
                                .services
                                .iter()
                                .filter(|service| service.importance > 0)
                                .map(|service| service.name.clone())
                                .collect();

                            if !enabled_names.is_empty() && !selected_services.is_empty() {
                                let mut rng = rand::thread_rng();
                                enabled_names.shuffle(&mut rng);

                                let assignments = if self.detailed_mode {
                                    // Detailed mode: weighted assignment with two-pass system
                                    let unique_services = selected_services.clone();

                                    let mut service_pool: Vec<String> = self
                                        .services
                                        .iter()
                                        .flat_map(|service| {
                                            std::iter::repeat(service.name.clone())
                                                .take(service.importance as usize)
                                        })
                                        .collect();

                                    service_pool.shuffle(&mut rng);

                                    let mut assignments = Vec::new();
                                    let mut assigned_services = std::collections::HashSet::new();

                                    // First pass: ensure each unique service is assigned at least once
                                    for name in &enabled_names {
                                        if let Some(service) = unique_services
                                            .iter()
                                            .find(|s| !assigned_services.contains(*s))
                                        {
                                            assignments.push(format!("{} => {}", name, service));
                                            assigned_services.insert(service.clone());
                                            // Remove this service from the pool
                                            if let Some(pos) =
                                                service_pool.iter().position(|x| x == service)
                                            {
                                                service_pool.remove(pos);
                                            }
                                        } else {
                                            break;
                                        }
                                    }

                                    // Second pass: assign remaining services to remaining people
                                    let remaining_names = &enabled_names[assignments.len()..];
                                    for name in remaining_names {
                                        if let Some(service) = service_pool.pop() {
                                            assignments.push(format!("{} => {}", name, service));
                                        } else {
                                            assignments
                                                .push(format!("{} => No Service Available", name));
                                        }
                                    }

                                    assignments
                                } else {
                                    // Basic mode: equal distribution with round-robin
                                    let num_members = enabled_names.len();
                                    let num_services = selected_services.len();
                                    let base_count = num_members / num_services;
                                    let extra = num_members % num_services;

                                    // Shuffle services so distribution is fair
                                    selected_services.shuffle(&mut rng);

                                    let mut service_pool = Vec::new();
                                    for (i, service) in selected_services.iter().enumerate() {
                                        let count = if i < extra {
                                            base_count + 1
                                        } else {
                                            base_count
                                        };
                                        for _ in 0..count {
                                            service_pool.push(service.clone());
                                        }
                                    }
                                    service_pool.shuffle(&mut rng);

                                    enabled_names
                                        .iter()
                                        .zip(service_pool.iter())
                                        .map(|(name, service)| format!("{} => {}", name, service))
                                        .collect()
                                };

                                self.display_text = assignments.join("\n");
                            } else {
                                self.display_text =
                                    "Please select at least one team member and service"
                                        .to_string();
                            }
                        }

                        // Copy button with 5px gap
                        ui.add_space(5.0);
                        if ui.small_button("📋").clicked() {
                            ui.output_mut(|o| o.copied_text = self.display_text.clone());
                        }

                        // Toggle button on the right side
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let toggle_text = if self.detailed_mode {
                                "Switch to Basic Mode"
                            } else {
                                "Switch to Detailed Mode"
                            };
                            if ui.button(toggle_text).clicked() {
                                self.detailed_mode = !self.detailed_mode;
                            }
                        });
                    });
                    ui.add_space(10.0);
                    ui.centered_and_justified(|ui| {
                        ui.add_sized(
                            [ui.available_width(), 40.0],
                            egui::TextEdit::multiline(&mut self.display_text),
                        );
                    });
                });
            });
    }
}
