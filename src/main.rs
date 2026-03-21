#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use eframe::egui::{Color32, RichText};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 500.0])
            .with_resizable(false),
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
}

impl Default for CalculatriceApp {
    fn default() -> Self {
        Self {
            input: String::new(),
            resultat: String::from("0"),
        }
    }
}

impl eframe::App for CalculatriceApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());

        // --- PIED DE PAGE (Version & Crédits) ---
        egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.add_space(5.0);
            ui.vertical_centered(|ui| {
                // Affichage de la version tirée du Cargo.toml
                ui.label(RichText::new(format!("Version {}", env!("CARGO_PKG_VERSION"))).size(9.0).weak());

                ui.add_space(2.0);
                ui.label(RichText::new("Développé par :").size(10.0).weak());
                ui.label(RichText::new("PRENCIPE Raffaele").strong().color(Color32::LIGHT_GRAY));
                ui.hyperlink_to("raf.prencipe@pm.me", "mailto:raf.prencipe@pm.me");
            });
            ui.add_space(5.0);
        });

        // --- ZONE CENTRALE ---
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.0);
            ui.vertical_centered(|ui| {
                ui.heading(RichText::new("RustCalc").strong().color(Color32::LIGHT_BLUE));
            });
            ui.add_space(15.0);

            ui.add(egui::TextEdit::singleline(&mut self.input)
                .font(egui::TextStyle::Monospace)
                .desired_width(f32::INFINITY));

            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                ui.label(RichText::new(&self.resultat).size(30.0).color(Color32::WHITE));
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            let btn_size = egui::vec2(60.0, 60.0);

            egui::Grid::new("btns").spacing([10.0, 10.0]).show(ui, |ui| {
                if ui.add(egui::Button::new("7").min_size(btn_size)).clicked() { self.input.push('7'); }
                if ui.add(egui::Button::new("8").min_size(btn_size)).clicked() { self.input.push('8'); }
                if ui.add(egui::Button::new("9").min_size(btn_size)).clicked() { self.input.push('9'); }
                if ui.add(egui::Button::new(RichText::new("/").color(Color32::GOLD)).min_size(btn_size)).clicked() { self.input.push('/'); }
                ui.end_row();

                if ui.add(egui::Button::new("4").min_size(btn_size)).clicked() { self.input.push('4'); }
                if ui.add(egui::Button::new("5").min_size(btn_size)).clicked() { self.input.push('5'); }
                if ui.add(egui::Button::new("6").min_size(btn_size)).clicked() { self.input.push('6'); }
                if ui.add(egui::Button::new(RichText::new("*").color(Color32::GOLD)).min_size(btn_size)).clicked() { self.input.push('*'); }
                ui.end_row();

                if ui.add(egui::Button::new("1").min_size(btn_size)).clicked() { self.input.push('1'); }
                if ui.add(egui::Button::new("2").min_size(btn_size)).clicked() { self.input.push('2'); }
                if ui.add(egui::Button::new("3").min_size(btn_size)).clicked() { self.input.push('3'); }
                if ui.add(egui::Button::new(RichText::new("-").color(Color32::GOLD)).min_size(btn_size)).clicked() { self.input.push('-'); }
                ui.end_row();

                if ui.add(egui::Button::new(RichText::new("C").color(Color32::LIGHT_RED)).min_size(btn_size)).clicked() {
                    self.input.clear();
                    self.resultat = "0".into();
                }
                if ui.add(egui::Button::new("0").min_size(btn_size)).clicked() { self.input.push('0'); }
                if ui.add(egui::Button::new(RichText::new("=").color(Color32::GREEN)).min_size(btn_size)).clicked() {
                    let tokens = lexer(&self.input);
                    self.resultat = resoudre_parentheses(tokens).to_string();
                }
                if ui.add(egui::Button::new(RichText::new("+").color(Color32::GOLD)).min_size(btn_size)).clicked() { self.input.push('+'); }
                ui.end_row();
            });
        });
    }
}

// --- ENGINE ---

#[derive(Debug, Clone, Copy, PartialEq)]
enum Token {
    Nombre(f64), Plus, Moins, Multiplier, Diviser, ParenOuvrante, ParenFermante,
}

fn lexer(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut nombre_temp = String::new();
    for c in input.chars() {
        if c.is_digit(10) || c == '.' {
            nombre_temp.push(c);
        } else {
            if !nombre_temp.is_empty() {
                if let Ok(val) = nombre_temp.parse::<f64>() {
                    tokens.push(Token::Nombre(val));
                }
                nombre_temp.clear();
            }
            match c {
                '+' => tokens.push(Token::Plus),
                '-' => tokens.push(Token::Moins),
                '*' => tokens.push(Token::Multiplier),
                '/' => tokens.push(Token::Diviser),
                '(' => tokens.push(Token::ParenOuvrante),
                ')' => tokens.push(Token::ParenFermante),
                _ => {}
            }
        }
    }
    if !nombre_temp.is_empty() {
        if let Ok(val) = nombre_temp.parse::<f64>() {
            tokens.push(Token::Nombre(val));
        }
    }
    tokens
}

fn resoudre_parentheses(mut tokens: Vec<Token>) -> f64 {
    if tokens.is_empty() { return 0.0; }
    while let Some(pos_fin) = tokens.iter().position(|t| *t == Token::ParenFermante) {
        if let Some(pos_debut) = tokens[..pos_fin].iter().rposition(|t| *t == Token::ParenOuvrante) {
            let mut sous_expression: Vec<Token> = tokens.drain(pos_debut..pos_fin + 1).collect();
            if sous_expression.len() >= 2 {
                sous_expression.remove(0);
                sous_expression.pop();
                let valeur = evaluer_priorites(&mut sous_expression);
                tokens.insert(pos_debut, Token::Nombre(valeur));
            }
        } else { break; }
    }
    evaluer_priorites(&mut tokens)
}

fn evaluer_priorites(tokens: &mut Vec<Token>) -> f64 {
    if tokens.is_empty() { return 0.0; }
    let mut i = 0;
    while i < tokens.len() {
        match tokens[i] {
            Token::Multiplier | Token::Diviser => {
                let op = tokens.remove(i);
                if i > 0 && i <= tokens.len() {
                    let n1_token = tokens.remove(i - 1);
                    let n2_token = tokens.remove(i - 1);
                    if let (Token::Nombre(n1), Token::Nombre(n2)) = (n1_token, n2_token) {
                        let res = if op == Token::Multiplier { n1 * n2 } else { n1 / n2 };
                        tokens.insert(i - 1, Token::Nombre(res));
                        i -= 1;
                    }
                } else { i += 1; }
            }
            _ => i += 1,
        }
    }
    if tokens.is_empty() { return 0.0; }
    let mut resultat = if let Token::Nombre(n) = tokens[0] { n } else { 0.0 };
    let mut i = 1;
    while i < tokens.len() {
        match tokens[i] {
            Token::Plus => {
                if let Some(Token::Nombre(n)) = tokens.get(i + 1) { resultat += n; }
                i += 2;
            }
            Token::Moins => {
                if let Some(Token::Nombre(n)) = tokens.get(i + 1) { resultat -= n; }
                i += 2;
            }
            _ => i += 1,
        }
    }
    resultat
}