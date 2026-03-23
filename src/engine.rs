use std::collections::VecDeque;

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
        match c {
            ' ' | '\t' => {
                chars.next();
            }
            '0'..='9' | '.' => {
                let mut s = String::new();
                while let Some(&nc) = chars.peek() {
                    if nc.is_ascii_digit() || nc == '.' {
                        s.push(nc);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Nombre(s.parse().map_err(|_| "Nombre invalide")?));
                peut_etre_unaire = false;
            }
            '+' | '-' | '*' | '/' | '(' | ')' => {
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
                    _ => unreachable!(),
                }
            }
            _ => return Err(format!("Caractère interdit: {}", c)),
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
                let mut trouve = false;
                while let Some(top) = pile.pop() {
                    if top == Token::ParenOuvrante {
                        trouve = true;
                        break;
                    }
                    sortie.push_back(top);
                }
                if !trouve {
                    return Err("Parenthèse orpheline".into());
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
        if t == Token::ParenOuvrante {
            return Err("Parenthèse non fermée".into());
        }
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
                let b = pile.pop().ok_or("Opérande manquant")?;
                let a = pile.pop().ok_or("Opérande manquant")?;
                match op {
                    Token::Plus => pile.push(a + b),
                    Token::Moins => pile.push(a - b),
                    Token::Multiplier => pile.push(a * b),
                    Token::Diviser => {
                        if b.abs() < f64::EPSILON {
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

pub fn resoudre_expression(input: &str) -> Result<f64, String> {
    evaluer_rpn(shunting_yard(lexer(input)?)?)
}
