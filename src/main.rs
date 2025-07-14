use eframe::egui;
use rand::seq::SliceRandom;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_resizable(true),
        ..Default::default()
    };
    eframe::run_native(
        "Availability Analysis Assignment Assistant",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_theme(egui::Theme::Dark);
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
}

impl AnalysisAssistant {}

impl Default for AnalysisAssistant {
    fn default() -> Self {
        Self {
            names: vec![
                ("Dane".to_string(), false),
                ("Dhandapani".to_string(), false),
                ("Li Chien".to_string(), false),
                ("Andrea".to_string(), false),
                ("Andrew".to_string(), false),
                ("Michael".to_string(), false),
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
        }
    }
}

impl eframe::App for AnalysisAssistant {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_theme(egui::Theme::Light);
        egui::Window::new("Availability Analysis Assignment Assistant")
            .auto_sized()
            .resizable(true)
            .title_bar(false)
            .show(ctx, |ui| {
                ui.heading("Team Members");
                for (name, enabled) in &mut self.names {
                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.label(name.to_string());
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.checkbox(enabled, "");
                        });
                    });
                }

                ui.add_space(20.0);
                ui.heading("Services");

                for service in &mut self.services {
                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.label(&service.name);
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.add(
                                egui::Slider::new(&mut service.importance, 0..=3)
                                    .text("Importance"),
                            );
                        });
                    });
                }

                ui.add_space(10.0);
                if ui.button("Assign services").clicked() {
                    let mut enabled_names: Vec<&str> = self
                        .names
                        .iter()
                        .filter(|(_, enabled)| *enabled)
                        .map(|(name, _)| name.as_str())
                        .collect();

                    // Get unique services first, then build full pool
                    let unique_services: Vec<String> = self
                        .services
                        .iter()
                        .filter(|service| service.importance > 0)
                        .map(|service| service.name.clone())
                        .collect();

                    let mut service_pool: Vec<String> = self
                        .services
                        .iter()
                        .flat_map(|service| {
                            std::iter::repeat(service.name.clone())
                                .take(service.importance as usize)
                        })
                        .collect();

                    if !enabled_names.is_empty() && !service_pool.is_empty() {
                        let mut rng = rand::thread_rng();
                        enabled_names.shuffle(&mut rng);
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
                                if let Some(pos) = service_pool.iter().position(|x| x == service) {
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
                                assignments.push(format!("{} => No service available", name));
                            }
                        }

                        self.display_text = assignments.join("\n");
                    } else {
                        self.display_text =
                            "Please select at least one team member and service".to_string();
                    }
                }
                ui.add_space(10.0);
                ui.centered_and_justified(|ui| {
                    ui.add_sized(
                        [ui.available_width(), 40.0],
                        egui::TextEdit::multiline(&mut self.display_text),
                    );
                });
            });
    }
}
