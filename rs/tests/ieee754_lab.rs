use std::ops::BitOr;

/// Retourne le nombre de bits nécessaires pour représenter `n`.
const fn bit_size(mut n: u64) -> usize {
    let mut r = 1;
    while n > 1 {
        n >>= 1;
        r += 1;
    }
    r
}

/// Taille en octets pour un f64 (calculée de façon générique)
const SIZE: usize = (1 + bit_size(f64::MAX_EXP as u64) + f64::MANTISSA_DIGITS as usize) / 8;
/// Taille en hexadécimal (chaque octet → 2 chiffres)
const HEX_SIZE: usize = SIZE * 2;

/// Renvoie une chaîne alignée à droite sur `width` en utilisant le caractère de remplissage `fill`.
fn pad_field(s: &str, width: usize, fill: char) -> String {
    if s.len() >= width {
        s.to_string()
    } else {
        let pad = fill.to_string().repeat(width - s.len());
        format!("{}{}", pad, s)
    }
}

/// Affiche une ligne formatée avec six champs ; chacun est aligné à droite sur une largeur fixe
fn display0(
    expr_as_string: &str,
    expr_as_hexfloat: &str,
    expr_as_hex: &str,
    expr_as_int: &str,
    expr: &str,
    analysis: &str,
    fill: char,
) {
    let field1 = pad_field(expr_as_string, 30, fill);
    let field2 = pad_field(expr_as_hexfloat, HEX_SIZE + 8, fill);
    let field3 = pad_field(expr_as_hex, HEX_SIZE, fill);
    let field4 = pad_field(expr_as_int, HEX_SIZE * 4, fill);
    let field5 = pad_field(expr, 14, fill);
    let field6 = pad_field(analysis, 40, fill);
    println!(
        "{} {} {} {} {} {}",
        field1, field2, field3, field4, field5, field6
    );
}

/// Affiche une ligne "vide" (seulement avec le caractère de remplissage par défaut)
fn default_display0() {
    display0("", "", "", "", "", "", '-');
}

/// Affiche l’en‐tête du tableau.
fn header() {
    default_display0();
    display0(
        "expression",
        "hexfloat",
        &format!("hex {}b", SIZE * 8),
        "bin",
        "approx. value",
        "Custom IEEE754 decoding",
        ' ',
    );
    default_display0();
}

/// Affiche le pied de tableau.
fn footer() {
    default_display0();
}

fn to_hexfloat(x: f64) -> String {
    // Cas particuliers
    if x.is_nan() {
        return "nan".to_string();
    }
    if x.is_infinite() {
        return if x.is_sign_negative() {
            "-inf".to_string()
        } else {
            "inf".to_string()
        };
    }
    if x == 0.0 {
        return if x.is_sign_negative() {
            "-0x0p+0".to_string()
        } else {
            "0x0p+0".to_string()
        };
    }

    // Préparer le signe
    let sign = if x.is_sign_negative() { "-" } else { "" };
    let bits = x.to_bits();
    let bias = 1023;
    let exp = ((bits >> 52) & 0x7ff) as i32;
    let frac = bits & ((1u64 << 52) - 1);

    // Pour supprimer les zéros non significatifs dans la partie fractionnaire
    fn trim_hex(s: &str) -> String {
        s.trim_end_matches('0').to_string()
    }

    if exp == 0 {
        // Nombre dénormalisé
        // La valeur réelle est x = frac * 2^(-1074)
        // Pour obtenir une représentation normalisée de la forme "0x1[.frac]p<exp>",
        // on "normalise" le nombre en décalant la mantisse.
        let f = frac; // f est non nul ici puisque x != 0
                      // Calculer le nombre de bits utilisés dans f.
                      // f est contenu dans 52 bits ; la longueur effective est :
        let bit_length = 64 - f.leading_zeros();
        // On souhaite obtenir une mantisse de 53 bits (1 implicite + 52 bits)
        // On décale donc f à gauche d'un certain nombre s tel que (f << s) ait exactement 53 bits.
        // Pour cela, s = 53 - (nombre de bits dans f)
        let s = 53 - bit_length;
        let normalized = f << s;
        // Dans la représentation normalisée, la partie entière doit être 1 (stockée implicitement)
        let frac_part = normalized & ((1u64 << 52) - 1);
        // L'exposant effectif est alors : e = -1022 - s
        let adjusted_exp = -1022 - (s as i32);
        if frac_part == 0 {
            format!("{}0x1p{:+}", sign, adjusted_exp)
        } else {
            let s_hex = format!("{:013x}", frac_part);
            let trimmed = trim_hex(&s_hex);
            format!("{}0x1.{}p{:+}", sign, trimmed, adjusted_exp)
        }
    } else {
        // Nombre normalisé
        let adjusted_exp = exp - bias;
        if frac == 0 {
            format!("{}0x1p{:+}", sign, adjusted_exp)
        } else {
            let s_hex = format!("{:013x}", frac);
            let trimmed = trim_hex(&s_hex);
            format!("{}0x1.{}p{:+}", sign, trimmed, adjusted_exp)
        }
    }
}

/// Effectue un décodage personnalisé du nombre IEEE754 et renvoie une chaîne récapitulative.
///
/// La fonction extrait le signe, l’exposant et la fraction à partir des bits, puis
/// reconstitue la valeur en effectuant le calcul correspondant. Elle compare ensuite
/// la valeur reconstituée avec la valeur originale.
fn compute_number(expr: &f64) -> String {
    let expr_as_int = expr.to_bits();
    let max_exp = f64::MAX_EXP as u64;
    let exp_bsize = bit_size(max_exp);
    let fraction_bsize = f64::MANTISSA_DIGITS as u32; // Pour f64, 53 bits (dont 1 implicite)
    let bsize = SIZE * 8; // Pour f64, 64 bits

    // Définition d'une fonction anonyme pour calculer 2^n
    let my_exp2 = |n: i32| -> f64 { 2.0_f64.powi(n) };

    // Extraction de la fraction (les 52 bits de poids faible)
    let fraction = expr_as_int & ((1u64 << (fraction_bsize - 1)) - 1);
    // Extraction de l'exposant (les 11 bits suivants)
    let exponent = (expr_as_int >> (fraction_bsize - 1)) & ((1u64 << exp_bsize) - 1);
    // Extraction du bit de signe (bit de poids fort)
    let sign_bit = (expr_as_int & (1u64 << (bsize - 1))) != 0;

    let mut oss = format!(
        "[s={} e={} f={:x}]",
        if sign_bit { 1 } else { 0 },
        exponent,
        fraction
    );

    // Fonction locale de comparaison (vérifie aussi le cas NaN)
    let compare = |a: f64, b: f64| -> String {
        if (a.is_nan() && b.is_nan()) || (a == b) {
            format!("[  OK  ]            ")
        } else {
            format!("[FAILED] {:12}", b)
        }
    };

    if exponent == ((1u64 << exp_bsize) - 1) {
        // Cas NaN et Infini
        if fraction == 0 {
            let mut r = f64::INFINITY;
            if sign_bit {
                r = -r;
            }
            oss.push_str(&compare(*expr, r));
        } else {
            oss.push_str(&compare(*expr, f64::NAN));
        }
    } else {
        let denormalized = exponent == 0;
        let mut r = if denormalized { 0.0 } else { 1.0 };
        // Reconstitution de la partie fractionnaire
        for i in 1..(fraction_bsize) {
            if expr_as_int & (1u64 << ((fraction_bsize - 1) - i)) != 0 {
                r += my_exp2(-(i as i32));
            }
        }
        if denormalized {
            r *= my_exp2(-((max_exp as i32) - 2));
        } else {
            r *= my_exp2((exponent as i32) - ((max_exp as i32) - 1));
        }
        if sign_bit {
            r = -r;
        }
        oss.push_str(&compare(*expr, r));
    }
    oss
}

/// Affiche une ligne pour une expression donnée.
///
/// Le premier argument est une chaîne décrivant l'expression (généralement générée par `stringify!`),
/// le deuxième est la valeur calculée, et le troisième le résultat du décodage personnalisé.
fn display(expr_as_string: &str, expr: f64, extra: String) {
    let expr_as_int = expr.to_bits();
    let hexfloat = to_hexfloat(expr); // format hexadécimal flottant
    let hex_int = format!("{:x}", expr_as_int);
    let bin_str = format!("{:0width$b}", expr_as_int, width = HEX_SIZE * 4);
    println!(
        "{:>30} {:>width1$} {:>width2$} {:>width3$} {:>14} {:>52}",
        expr_as_string,
        hexfloat,
        hex_int,
        bin_str,
        format!("{:.5e}", expr),
        extra,
        width1 = HEX_SIZE + 8,
        width2 = HEX_SIZE,
        width3 = HEX_SIZE * 4
    );
}

macro_rules! display {
    ($expr:expr) => {
        display(
            stringify!($expr),
            f64::from($expr),
            compute_number(&f64::from($expr)),
        )
    };
}

/// Macro qui retourne un f64 dont les bits sont définis par les positions indiquées.
macro_rules! bits {
    ( $( $b:expr ),* ) => {{
        #[allow(unused_mut)]
        let mut value: u64 = 0;
        $(
            value |= 1u64 << $b;
        )*
        f64::from_bits(value)
    }};
}

/// Structure similaire à `float_bit` en C++.
#[derive(Copy, Clone)]
struct FloatBit {
    n: u64,
}

impl From<FloatBit> for f64 {
    fn from(fb: FloatBit) -> Self {
        f64::from_bits(fb.n)
    }
}

impl BitOr for FloatBit {
    type Output = FloatBit;
    fn bitor(self, rhs: FloatBit) -> FloatBit {
        FloatBit { n: self.n | rhs.n }
    }
}

/// Fonction équivalente à l'opérateur littéral `_b` en C++.
/// Par exemple, `b(5)` correspond à `5_b`.
fn b(i: u32) -> FloatBit {
    FloatBit { n: 1u64 << i }
}

/// Constante équivalente à `minus` en C++.
const MINUS: FloatBit = FloatBit {
    n: 1u64 << (SIZE * 8 - 1),
};

/// Construit le bit de l’exposant en décalant `i` vers la gauche de (MANTISSA_DIGITS-1) positions.
fn exponent(i: u32) -> FloatBit {
    FloatBit {
        n: (i as u64) << (f64::MANTISSA_DIGITS - 1),
    }
}

#[test]
fn main() {
    // Valeurs limites pour f64
    let eps = f64::EPSILON;

    header();
    display!(eps);
    display!(f64::MIN_POSITIVE);
    display!(f64::MAX);
    display!(f64::MIN_POSITIVE / 10.0);
    // Pour obtenir le plus petit subnormal (équivalent de denorm_min)
    display!(f64::from_bits(1));
    display!(f64::INFINITY);
    display!(-f64::INFINITY);
    display!(f64::NAN);
    display!(f64::NAN); // en Rust, quiet et signaling NaN ne sont pas distincts
    display!(1.0 + eps - 1.0); // Exact
    display!(2.0 + eps - 2.0); // Pas exact
    footer();

    display!(0.0);
    display!(1.0);
    display!(1.1);
    display!(314.0);
    footer();

    display!(bits!());
    display!(bits!(0));
    display!(bits!(1));
    display!(bits!(2));
    display!(b(5));
    display!(MINUS | b(5));
    display!(exponent(1));
    display!(exponent(1) | b(5));
    footer();

    display!(bits!(48));
    display!(bits!(49));
    display!(bits!(50));
    display!(bits!(51));
    display!(bits!(52));
    display!(bits!(53));
    display!(bits!(54));
    display!(bits!(55));
    display!(bits!(56));
    display!(bits!(57));
    display!(bits!(58));
    display!(bits!(59));
    display!(bits!(60));
    display!(bits!(61));
    display!(bits!(62));
    display!(bits!(63));
    footer();
}
