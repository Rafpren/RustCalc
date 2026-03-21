#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use eframe::egui::{Color32, RichText, Button, Vec2, Key};
use std::collections::VecDeque;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([360.0, 600.0])
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "RustCalc",
        options,
        Box::new(|_cc| Ok(Box::<CalculatriceApp>::default())),
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
                if self.resultat.is_empty() { self.resultat = "0".into(); }
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
        ctx.set_visuals(egui::Visuals::dark());

        // --- GESTION DU CLAVIER ROBUSTE (v1.2.1) ---
        ctx.input(|i| {
            // 1. Capturer la saisie de texte (Chiffres, Opérateurs, Points)
            for event in &i.events {
                if let egui::Event::Text(t) = event {
                    // On ne laisse passer que les caractères valides pour une calculatrice
                    if "0123456789+-*/().".contains(t) {
                        self.input.push_str(t);
                    }
                }
            }

            // 2. Capturer les touches de contrôle (Indépendantes du pavé numérique ou non)
            if i.key_pressed(Key::Enter) { self.calculer(); }
            if i.key_pressed(Key::Escape) || i.key_pressed(Key::Delete) { self.effacer(); }
            if i.key_pressed(Key::Backspace) { self.input.pop(); }
        });

        // --- UI (Layout stable v1.1.2) ---
        egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.add_space(5.0);
            ui.vertical_centered(|ui| {
                ui.label(RichText::new(format!("Version {}", env!("CARGO_PKG_VERSION"))).size(9.0).weak());
                ui.add_space(2.0);
                ui.label(RichText::new("Développé par :").size(10.0).weak());
                ui.label(RichText::new("PRENCIPE Raffaele").strong().color(Color32::LIGHT_GRAY));
                ui.hyperlink_to("raf.prencipe@pm.me", "mailto:raf.prencipe@pm.me");
            });
            ui.add_space(5.0);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.0);
            ui.vertical_centered(|ui| ui.heading(RichText::new("RustCalc").strong().color(Color32::LIGHT_BLUE)));
            ui.add_space(15.0);

            ui.add(egui::TextEdit::singleline(&mut self.input)
                .font(egui::TextStyle::Monospace)
                .desired_width(f32::INFINITY)
                .interactive(false));

            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                let color = if self.a_erreur { Color32::LIGHT_RED } else { Color32::WHITE };
                ui.label(RichText::new(&self.resultat).size(30.0).color(color));
            });

            ui.add_space(10.0); ui.separator(); ui.add_space(10.0);

            let spacing = 8.0;
            let btn_width = (ui.available_width() - (3.0 * spacing)) / 4.0;
            let btn_size = Vec2::new(btn_width, 60.0);

            ui.vertical(|ui| {
                let rows = [
                    vec![("(", None), (")", None), ("C", Some(Color32::LIGHT_RED)), ("/", Some(Color32::GOLD))],
                    vec![("7", None), ("8", None), ("9", None), ("*", Some(Color32::GOLD))],
                    vec![("4", None), ("5", None), ("6", None), ("-", Some(Color32::GOLD))],
                    vec![("1", None), ("2", None), ("3", None), ("+", Some(Color32::GOLD))],
                ];

                for row in rows {
                    ui.horizontal(|ui| {
                        for (label, color) in row {
                            let btn = Button::new(RichText::new(label).color(color.unwrap_or(Color32::WHITE)));
                            if ui.add(btn.min_size(btn_size)).clicked() {
                                if label == "C" { self.effacer(); } else { self.input.push_str(label); }
                            }
                            ui.add_space(spacing);
                        }
                    });
                    ui.add_space(spacing);
                }

                ui.horizontal(|ui| {
                    if ui.add(Button::new("0").min_size(Vec2::new(btn_width * 2.0 + spacing, 60.0))).clicked() { self.input.push('0'); }
                    ui.add_space(spacing);
                    if ui.add(Button::new(".").min_size(btn_size)).clicked() { self.input.push('.'); }
                    ui.add_space(spacing);
                    if ui.add(Button::new(RichText::new("=").color(Color32::GREEN)).min_size(btn_size)).clicked() { self.calculer(); }
                });
            });
        });
    }
}

// --- ENGINE (Statique, robuste) ---
#[derive(Debug, Clone, PartialEq)]
enum Token { Nombre(f64), Plus, Moins, Multiplier, Diviser, ParenOuvrante, ParenFermante, UnaryMoins }

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
        matches!(self, Token::Plus | Token::Moins | Token::Multiplier | Token::Diviser | Token::UnaryMoins)
    }
}

fn lexer(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let mut peut_etre_unaire = true;
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() { chars.next(); continue; }
        if c.is_digit(10) || c == '.' {
            let mut s = String::new();
            while let Some(&nc) = chars.peek() {
                if nc.is_digit(10) || nc == '.' { s.push(nc); chars.next(); } else { break; }
            }
            tokens.push(Token::Nombre(s.parse().map_err(|_| "Erreur nombre")?));
            peut_etre_unaire = false;
        } else {
            chars.next();
            match c {
                '+' => { tokens.push(Token::Plus); peut_etre_unaire = true; }
                '-' => { tokens.push(if peut_etre_unaire { Token::UnaryMoins } else { Token::Moins }); peut_etre_unaire = true; }
                '*' => { tokens.push(Token::Multiplier); peut_etre_unaire = true; }
                '/' => { tokens.push(Token::Diviser); peut_etre_unaire = true; }
                '(' => { tokens.push(Token::ParenOuvrante); peut_etre_unaire = true; }
                ')' => { tokens.push(Token::ParenFermante); peut_etre_unaire = false; }
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
                    if top == Token::ParenOuvrante { break; }
                    sortie.push_back(top);
                }
            }
            _ => {
                while let Some(top) = pile.last() {
                    if top.est_operateur() && top.precedence() >= token.precedence() {
                        sortie.push_back(pile.pop().unwrap());
                    } else { break; }
                }
                pile.push(token);
            }
        }
    }
    while let Some(t) = pile.pop() { sortie.push_back(t); }
    Ok(sortie)
}

fn evaluer_rpn(rpn: VecDeque<Token>) -> Result<f64, String> {
    let mut pile = Vec::new();
    for token in rpn {
        match token {
            Token::Nombre(n) => pile.push(n),
            Token::UnaryMoins => { let n = pile.pop().ok_or("Erreur")?; pile.push(-n); }
            op => {
                let b = pile.pop().ok_or("Manquant")?;
                let a = pile.pop().ok_or("Manquant")?;
                match op {
                    Token::Plus => pile.push(a + b),
                    Token::Moins => pile.push(a - b),
                    Token::Multiplier => pile.push(a * b),
                    Token::Diviser => { if b == 0.0 { return Err("Div/0".into()); } pile.push(a / b); }
                    _ => {}
                }
            }
        }
    }
    pile.pop().ok_or("Vide".into())
}

fn resoudre_expression(input: &str) -> Result<f64, String> {
    if input.trim().is_empty() { return Ok(0.0); }
    evaluer_rpn(shunting_yard(lexer(input)?)?)
}
