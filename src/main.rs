#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use eframe::egui::{Button, Color32, IconData, Key, RichText, Vec2};
use std::collections::VecDeque;
use std::sync::Arc;

fn main() -> eframe::Result<()> {
    // Chargement de l'icône
    let icon_bytes = include_bytes!("../icons/icon.png");
    let icon_image = image::load_from_memory(icon_bytes)
        .expect("Échec du chargement de l'icône")
        .to_rgba8();
    let (width, height) = icon_image.dimensions();

    let icon = IconData {
        rgba: icon_image.into_raw(),
        width,
        height,
    };

    // Configuration de la fenêtre avec largeur optimisée pour la symétrie
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([340.0, 620.0])
            .with_icon(Arc::new(icon))
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "RustCalc",
        native_options,
        Box::new(|_cc| Ok(Box::new(CalculatriceApp::default()))),
    )
}

struct CalculatriceApp {
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
        match resoudre_expression(&self.input) {
            Ok(val) => {
                self.resultat = format!("{:.10}", val)
                    .trim_end_matches('0')
                    .trim_end_matches('.')
                    .to_string();
                if self.resultat.is_empty() {
                    self.resultat = "0".into();
                }
                self.a_erreur = false;
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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // --- STYLE ET VISUELS ---
        let mut visuals = egui::Visuals::dark();
        visuals.widgets.inactive.bg_fill = Color32::from_rgb(45, 45, 50);
        visuals.widgets.hovered.bg_fill = Color32::from_rgb(60, 60, 70);
        visuals.widgets.active.bg_fill = Color32::from_rgb(80, 80, 100);
        visuals.widgets.inactive.rounding = 10.0.into();
        ctx.set_visuals(visuals);

        // --- GESTION DES TOUCHES CLAVIER ---
        ctx.input(|i| {
            for event in &i.events {
                if let egui::Event::Text(t) = event
                    && "0123456789+-*/().".contains(t) {
                    self.input.push_str(t);
                }
            }
            if i.key_pressed(Key::Enter) {
                self.calculer();
            }
            if i.key_pressed(Key::Escape) || i.key_pressed(Key::Delete) {
                self.effacer();
            }
            if i.key_pressed(Key::Backspace) {
                self.input.pop();
            }
        });

        // --- FOOTER ---
        egui::TopBottomPanel::bottom("footer")
            .frame(egui::Frame::none().inner_margin(10.0))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(
                        RichText::new("Développé par : Raffaele PRENCIPE")
                            .size(11.0)
                            .strong()
                            .color(Color32::from_rgb(110, 160, 255)),
                    );
                });
            });

        // --- INTERFACE PRINCIPALE (CENTRAL PANEL) ---
        egui::CentralPanel::default()
            .frame(egui::Frame::none().inner_margin(egui::Margin {
                left: 15.0,  // Marge gauche fixe
                right: 15.0, // Marge droite identique pour la symétrie
                top: 20.0,
                bottom: 10.0,
            }))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading(
                        RichText::new("RUSTCALC")
                            .strong()
                            .extra_letter_spacing(1.5)
                            .color(Color32::LIGHT_BLUE),
                    )
                });
                ui.add_space(20.0);

                // Écran d'affichage
                ui.group(|ui| {
                    ui.set_width(ui.available_width());
                    ui.add(
                        egui::TextEdit::singleline(&mut self.input)
                            .font(egui::FontId::monospace(18.0))
                            .desired_width(f32::INFINITY)
                            .frame(false)
                            .interactive(false),
                    );

                    ui.add_space(5.0);

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        let color = if self.a_erreur {
                            Color32::LIGHT_RED
                        } else {
                            Color32::WHITE
                        };
                        ui.label(
                            RichText::new(&self.resultat)
                                .size(32.0)
                                .strong()
                                .color(color),
                        );
                    });
                });

                ui.add_space(20.0);
                ui.separator();
                ui.add_space(20.0);

                // GRILLE DE BOUTONS SYMÉTRIQUE
                let spacing = 8.0;
                let total_spacing = 3.0 * spacing;
                let btn_width = (ui.available_width() - total_spacing) / 4.0;
                let btn_size = Vec2::new(btn_width, 60.0);

                ui.vertical(|ui| {
                    let rows = [
                        vec![
                            ("(", None),
                            (")", None),
                            ("C", Some(Color32::from_rgb(220, 100, 100))),
                            ("/", Some(Color32::from_rgb(255, 170, 0))),
                        ],
                        vec![
                            ("7", None),
                            ("8", None),
                            ("9", None),
                            ("*", Some(Color32::from_rgb(255, 170, 0))),
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
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = spacing; // Force l'espacement horizontal précis
                            for (label, color) in row {
                                let text = RichText::new(label)
                                    .size(20.0)
                                    .color(color.unwrap_or(Color32::WHITE));
                                if ui.add(Button::new(text).min_size(btn_size)).clicked() {
                                    if label == "C" {
                                        self.effacer();
                                    } else {
                                        self.input.push_str(label);
                                    }
                                }
                            }
                        });
                        ui.add_space(spacing);
                    }

                    // Dernière ligne (0 large, point, égal)
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = spacing;
                        let zero_width = (btn_width * 2.0) + spacing;
                        if ui
                            .add(
                                Button::new(RichText::new("0").size(20.0))
                                    .min_size(Vec2::new(zero_width, 60.0)),
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
                                    RichText::new("=").size(22.0).strong().color(Color32::GREEN),
                                )
                                .min_size(btn_size),
                            )
                            .clicked()
                        {
                            self.calculer();
                        }
                    });
                });
            });
    }
}

// --- LOGIQUE DE CALCUL (ENGINE) ---

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Nombre(f64),
    Plus,
    Moins,
    Multiplier,
    Diviser,
    ParenOuvrante,
    ParenFermante,
    UnaryMoins,
}

impl Token {
    fn precedence(&self) -> i32 {
        match self {
            Token::Plus | Token::Moins => 1,
            Token::Multiplier | Token::Diviser => 2,
            Token::UnaryMoins => 3,
            _ => 0,
        }
    }
    fn est_operateur(&self) -> bool {
        matches!(
            self,
            Token::Plus | Token::Moins | Token::Multiplier | Token::Diviser | Token::UnaryMoins
        )
    }
}

fn lexer(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let mut peut_etre_unaire = true;
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
            continue;
        }
        if c.is_ascii_digit() || c == '.' {
            let mut s = String::new();
            while let Some(&nc) = chars.peek() {
                if nc.is_ascii_digit() || nc == '.' {
                    s.push(nc);
                    chars.next();
                } else {
                    break;
                }
            }
            tokens.push(Token::Nombre(s.parse().map_err(|_| "Erreur nombre")?));
            peut_etre_unaire = false;
        } else {
            chars.next();
            match c {
                '+' => {
                    tokens.push(Token::Plus);
                    peut_etre_unaire = true;
                }
                '-' => {
                    tokens.push(if peut_etre_unaire {
                        Token::UnaryMoins
                    } else {
                        Token::Moins
                    });
                    peut_etre_unaire = true;
                }
                '*' => {
                    tokens.push(Token::Multiplier);
                    peut_etre_unaire = true;
                }
                '/' => {
                    tokens.push(Token::Diviser);
                    peut_etre_unaire = true;
                }
                '(' => {
                    tokens.push(Token::ParenOuvrante);
                    peut_etre_unaire = true;
                }
                ')' => {
                    tokens.push(Token::ParenFermante);
                    peut_etre_unaire = false;
                }
                _ => return Err(format!("Invalide: {}", c)),
            }
        }
    }
    Ok(tokens)
}

fn shunting_yard(tokens: Vec<Token>) -> Result<VecDeque<Token>, String> {
    let mut sortie = VecDeque::new();
    let mut pile = Vec::new();
    for token in tokens {
        match token {
            Token::Nombre(_) => sortie.push_back(token),
            Token::ParenOuvrante | Token::UnaryMoins => pile.push(token),
            Token::ParenFermante => {
                while let Some(top) = pile.pop() {
                    if top == Token::ParenOuvrante {
                        break;
                    }
                    sortie.push_back(top);
                }
            }
            _ => {
                while let Some(top) = pile.last() {
                    if top.est_operateur() && top.precedence() >= token.precedence() {
                        sortie.push_back(pile.pop().unwrap());
                    } else {
                        break;
                    }
                }
                pile.push(token);
            }
        }
    }
    while let Some(t) = pile.pop() {
        sortie.push_back(t);
    }
    Ok(sortie)
}

fn evaluer_rpn(rpn: VecDeque<Token>) -> Result<f64, String> {
    let mut pile = Vec::new();
    for token in rpn {
        match token {
            Token::Nombre(n) => pile.push(n),
            Token::UnaryMoins => {
                let n = pile.pop().ok_or("Erreur")?;
                pile.push(-n);
            }
            op => {
                let b = pile.pop().ok_or("Manquant")?;
                let a = pile.pop().ok_or("Manquant")?;
                match op {
                    Token::Plus => pile.push(a + b),
                    Token::Moins => pile.push(a - b),
                    Token::Multiplier => pile.push(a * b),
                    Token::Diviser => {
                        if b == 0.0 {
                            return Err("Div/0".into());
                        }
                        pile.push(a / b);
                    }
                    _ => {}
                }
            }
        }
    }
    pile.pop().ok_or("Vide".into())
}

fn resoudre_expression(input: &str) -> Result<f64, String> {
    if input.trim().is_empty() {
        return Ok(0.0);
    }
    evaluer_rpn(shunting_yard(lexer(input)?)?)
}
