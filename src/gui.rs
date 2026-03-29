use crate::engine::resoudre_expression;
use eframe::egui;
use eframe::egui::{Button, Color32, Key, RichText, Vec2};

pub struct CalculatriceApp {
    input: String,
    resultat: String,
    a_erreur: bool,
}

impl Default for CalculatriceApp {
    fn default() -> Self {
        Self {
            input: String::new(),
            resultat: String::from("0"),
            a_erreur: false,
        }
    }
}

impl CalculatriceApp {
    fn calculer(&mut self) {
        let trim_input = self.input.trim();
        if trim_input.is_empty() {
            return;
        }

        match resoudre_expression(trim_input) {
            Ok(val) => {
                if val.is_infinite() || val.is_nan() {
                    self.resultat = "Erreur math".into();
                    self.a_erreur = true;
                } else {
                    self.resultat = format!("{:.10}", val)
                        .trim_end_matches('0')
                        .trim_end_matches('.')
                        .to_string();
                    if self.resultat.is_empty() {
                        self.resultat = "0".into();
                    }
                    self.a_erreur = false;
                }
            }
            Err(msg) => {
                self.resultat = format!("Err: {}", msg);
                self.a_erreur = true;
            }
        }
    }

    fn effacer(&mut self) {
        self.input.clear();
        self.resultat = "0".into();
        self.a_erreur = false;
    }
}

impl eframe::App for CalculatriceApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let mut visuals = egui::Visuals::dark();
        visuals.widgets.inactive.bg_fill = Color32::from_rgb(45, 45, 50);
        visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(4);

        // Récupération du contexte depuis l'objet ui fourni par la nouvelle API
        ui.ctx().set_visuals(visuals);

        // --- GESTION DES TOUCHES HYBRIDE ---
        ui.input(|i| {
            for event in &i.events {
                match event {
                    egui::Event::Text(t) => {
                        for c in t.chars() {
                            match c {
                                '0'..='9' | '+' | '-' | '(' | ')' | '.' => self.input.push(c),
                                '*' | '×' => self.input.push('×'),
                                '/' | '÷' => self.input.push('÷'),
                                _ => {}
                            }
                        }
                    }
                    egui::Event::Key {
                        key, pressed: true, ..
                    } => match *key {
                        Key::Enter => self.calculer(),
                        Key::Escape | Key::Delete => self.effacer(),
                        Key::Backspace => {
                            self.input.pop();
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        });

        // --- FOOTER ---
        // Utilisation de egui::Panel au lieu du type déprécié TopBottomPanel
        egui::Panel::bottom("footer")
            .frame(egui::Frame::default().inner_margin(10))
            .show_inside(ui, |footer_ui| {
                footer_ui.vertical_centered(|ui| {
                    ui.label(
                        RichText::new("Développé par : Raffaele PRENCIPE")
                            .size(11.0)
                            .strong()
                            .color(Color32::from_rgb(110, 160, 255)),
                    );
                });
            });

        // --- INTERFACE PRINCIPALE ---
        egui::CentralPanel::default()
            .frame(egui::Frame::default().inner_margin(15))
            .show_inside(ui, |main_ui| {
                main_ui.vertical_centered(|ui| {
                    ui.heading(
                        RichText::new("RUSTCALC")
                            .strong()
                            .extra_letter_spacing(1.5)
                            .color(Color32::LIGHT_BLUE),
                    );
                });
                main_ui.add_space(20.0);

                main_ui.group(|ui| {
                    ui.set_width(ui.available_width());
                    ui.add(
                        egui::TextEdit::singleline(&mut self.input)
                            .font(egui::FontId::monospace(18.0))
                            .desired_width(f32::INFINITY)
                            // Typage strict exigé par la v0.34.1
                            .frame(egui::Frame::NONE)
                            .interactive(false),
                    );
                    ui.add_space(5.0);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {

                        let (text_size, color) = if self.a_erreur {
                            (18.0, Color32::LIGHT_RED)
                        } else {
                            (36.0, Color32::WHITE)
                        };

                        ui.add(
                            egui::Label::new(
                                RichText::new(&self.resultat)
                                    .size(text_size)
                                    .strong()
                                    .color(color),
                            )
                                .truncate()
                        );
                    });
                });

                main_ui.add_space(20.0);
                main_ui.separator();
                main_ui.add_space(20.0);

                let spacing = 4.0;
                let btn_width = (main_ui.available_width() - (3.0 * spacing)) / 4.0;
                let btn_size = Vec2::new(btn_width, 48.0);

                let rows = [
                    vec![
                        ("(", None),
                        (")", None),
                        ("C", Some(Color32::from_rgb(220, 100, 100))),
                        ("÷", Some(Color32::from_rgb(255, 170, 0))),
                    ],
                    vec![
                        ("7", None),
                        ("8", None),
                        ("9", None),
                        ("×", Some(Color32::from_rgb(255, 170, 0))),
                    ],
                    vec![
                        ("4", None),
                        ("5", None),
                        ("6", None),
                        ("-", Some(Color32::from_rgb(255, 170, 0))),
                    ],
                    vec![
                        ("1", None),
                        ("2", None),
                        ("3", None),
                        ("+", Some(Color32::from_rgb(255, 170, 0))),
                    ],
                ];

                for row in rows {
                    main_ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = spacing;
                        for (label, color) in row {
                            if ui
                                .add(
                                    Button::new(
                                        RichText::new(label)
                                            .size(20.0)
                                            .color(color.unwrap_or(Color32::WHITE)),
                                    )
                                        .min_size(btn_size),
                                )
                                .clicked()
                            {
                                if label == "C" {
                                    self.effacer();
                                } else {
                                    self.input.push_str(label);
                                }
                            }
                        }
                    });
                    main_ui.add_space(spacing);
                }

                main_ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = spacing;
                    if ui
                        .add(
                            Button::new(RichText::new("0").size(20.0))
                                .min_size(Vec2::new((btn_width * 2.0) + spacing, 48.0)),
                        )
                        .clicked()
                    {
                        self.input.push('0');
                    }
                    if ui
                        .add(Button::new(RichText::new(".").size(20.0)).min_size(btn_size))
                        .clicked()
                    {
                        self.input.push('.');
                    }

                    if ui
                        .add(
                            Button::new(
                                RichText::new("=").size(24.0).color(Color32::BLACK),
                            )
                                .min_size(btn_size)
                                .fill(Color32::from_rgb(40, 190, 150)),
                        )
                        .clicked()
                    {
                        self.calculer();
                    }
                });
            });
    }
}