use std::io::{self, Write};
use std::fs;
use std::path::Path;

fn sum(n1: i32, n2: i32) -> i32 {
n1 + n2
}
fn subtract(n1: i32, n2: i32) -> i32 {
n1 - n2
}
fn multiply(n1: i32, n2: i32) -> i32 {
n1 * n2
}
fn divide(n1: i32, n2: i32) -> i32 {
if n2 == 0 {
println!("Impossible to divide by 0");
0
}
else {
n1 / n2
}
}
fn read_number() -> i32 {
loop {
print!("Type a number: ");
io::stdout().flush().unwrap();
let mut n1 = String::new();
io::stdin().read_line(&mut n1).unwrap();
match n1.trim().parse::<i32>() {
Ok(n) => return n,
Err(_) => print!("Invalid Number try Again"),
}
}
}
fn getnumbers() -> (i32, i32) {
let n1 = read_number();
let n2 = read_number();
(n1, n2)
}
fn menu() {
println!("\x1b[1m---- Calculator Rust V3 ----\x1b[0m\n");
println!("1- add");
println!("2- subtract");
println!("3- multiply");
println!("4- divide");
}
fn calc() {
menu();
print!("Choose: ");
io::stdout().flush().unwrap();
let mut option = String::new();
io::stdin().read_line(&mut option).unwrap();
let option = option.trim();

let (n1, n2) = getnumbers();
match option {
"1" => println!("result: {}\n", sum(n1, n2)),
"2" => println!("result: {}\n", subtract(n1, n2)),
"3" => println!("result: {}\n", multiply(n1, n2)),
"4" => println!("result: {}\n", divide(n1, n2)),
_=> println!("Invalid option"),
}
}
fn get() -> (f64, f64, f64, f64) {
print!("Type the value of X: ");
io::stdout().flush().unwrap();
let mut x = String::new();
io::stdin().read_line(&mut x).unwrap();
let x: f64 = x.trim().parse().expect("Type only numbers");

print!("Type another value to X: ");
io::stdout().flush().unwrap();
let mut x2 = String::new();
io::stdin().read_line(&mut x2).unwrap();
let x2: f64 = x2.trim().parse().expect("Type only numbers");

print!("Type a number: ");
io::stdout().flush().unwrap();
let mut n1 = String::new();
io::stdin().read_line(&mut n1).unwrap();
let n1: f64 = n1.trim().parse().expect("Type only numbers");

print!("Type another number: ");
io::stdout().flush().unwrap();
let mut n2 = String::new();
io::stdin().read_line(&mut n2).unwrap();
let n2: f64 = n2.trim().parse().expect("Type only numbers");

(x, x2, n1, n2)
}
fn equation() {
println!("\x1b[1m\n======================================");
println!("=== First-Degree Equations Rust V2 ===");
println!("======================================\x1b[0m\n");
let (x, x2, n1, n2) = get();
println!("\x1b[1m\nThe equation:");
println!("{}X - {} = {} + {}X\x1b[0m", x, n1, n2, x2);
println!("\x1b[3m\nSeperate number and letter");
println!("{}X - {}X = {} + {}\n", x, x2, n2, n1);
println!("\n{} + {} = {}", n1, n2, n1 + n2);
println!("{}X - {} = {}", x, x2, x - x2);
println!("{}X = {}\x1b[0m\n", x - x2, n1 + n2);
println!("\x1b[1m\nX = {}\x1b[0m\n", (x - x2) / (n1 + n2));
}
fn getabc() -> (f64, f64, f64) {
print!("Type the value of A: ");
io::stdout().flush().unwrap();
let mut a = String::new();
io::stdin().read_line(&mut a).unwrap();
let a: f64 = a.trim().parse().expect("Type only numbers");

print!("Type the value of B: ");
io::stdout().flush().unwrap();
let mut b = String::new();
io::stdin().read_line(&mut b).unwrap();
let b: f64 = b.trim().parse().expect("Type only numbers");

print!("Type the value of C: ");
io::stdout().flush().unwrap();
let mut c = String::new();
io::stdin().read_line(&mut c).unwrap();
let c: f64 = c.trim().parse().expect("Type only numbers");

(a, b, c)
}
fn bhaskara() {
println!("\x1b[1m\n==========================");
println!("==== Bhaskara Rust V3 ====");
println!("==========================\x1b[0m");
let (a, b, c) = getabc();
let delta = b * b - 4.0 * a * c;
println!("\x1b[1m\nThe equation:\x1b[0m");
println!("\x1b[3m{}x² - {}x + {} = 0\x1b[0m", a, b, c);
println!("\x1b[1m\nfind the value of ABC\x1b[0m");
println!("\x1b[3mA = {}, B = {}, C = {}\x1b[0m\n", a, b, c);
println!("\x1b[1mfind the value of delta\x1b[0m");
println!("\x1b[3m∆=(B)²-4AC\n");
println!("({})² = {}", b, b * b);
println!("4 • {} • {} = {}\n", a, c, 4.0 * a * c);
println!("∆= {} - {}", b * b, 4.0 * a * c);
println!("∆= {}\x1b[0m\n", delta);
let epsilon = 1e-10;
if delta < -epsilon {
println!("\x1b[1mThere's no root\x1b[0m\n");
}
else if delta.abs() < epsilon {
let x = -b / (2.0 * a);
println!("There's one root");
println!("\x1b[1mX = {}\x1b[0m\n", x);
}
else {
let sqrt_delta = delta.sqrt();
let x3 = (-b + sqrt_delta) / (2.0 * a);
let x4 = (-b - sqrt_delta) / (2.0 * a);
println!("\x1b[1m\nX¹ = {}", x3);
println!("X² = {}\x1b[0m\n", x4);
}
}
fn calculator() {
println!("\x1b[1m\n==============================");
println!("=== Calculator System Rust ===");
println!("==============================\x1b[0m\n");
println!("1- compile Standard Calculator");
println!("2- compile Bhaskara's formula");
println!("3- compile First-Degree Equations");
print!("Choose: ");
io::stdout().flush().unwrap();
let mut options = String::new();
io::stdin().read_line(&mut options).unwrap();
let options = options.trim();

match options {
"1" => calc(),
"2" => bhaskara(),
"3" => equation(),
_=> println!("Invalid option"),
}
}
fn show_generate() {
print!("Type the printed word: ");
io::stdout().flush().unwrap();
let mut word = String::new();
io::stdin().read_line(&mut word).unwrap();
let word = word.trim();

let show = format!(r#"
fn main() {{
println!("{}");
}}
"#, word);

if Path::new("Show.rs").exists() {
println!("This file is already exists");
}
else {
fs::write("Show.rs", show).unwrap();
println!("file created");
}
}
fn sum_generate() {
let sum = format!(r#"
use std::io::{{self, Write}};

fn main() {{
println!(" --- Sum of two numbers generated ---");
print!("Type a number: ");
io::stdout().flush().unwrap();
let mut n1 = String::new():
io::stdin().read_line(&mut n1).unwrap();

let n1: f64 = n1.trim().parse().expect("Type only numbers");

print!("Type another number: ");
io::stdout().flush().unwrap();
let mut n2 = String::new();
io::stdin().read_line(&mut n2).unwrap();

let n2: f64 = n2.trim().parse().expect("Type only numbers");

println!("The sum of {{}} + {{}} is = {{}}", n1, n2, n1 + n2);
}}
"#);

if Path::new("Sum.rs").exists() {
println!("This file is already exists");
}
else {
fs::write("Sum.rs", sum).unwrap();
println!("file created\n");
}
}
fn calc_generate() {
print!("Type the name of the calculator: ");
io::stdout().flush().unwrap();
let mut calcname = String::new();
io::stdin().read_line(&mut calcname).unwrap();
let calcname = calcname.trim();

let calcs = format!(r#"
use std::io::{{self, Write}};

fn sum(n1: i32, n2: i32) -> i32 {{
n1 + n2
}}
fn subtract(n1: i32, n2: i32) -> i32 {{
n1 - n2
}}
fn multiply(n1: i32, n2: i32) -> i32 {{
n1 * n2
}}
fn divide(n1: i32, n2: i32) -> i32 {{
if n2 == 0 {{
println!("Impossible to divide by 0");
0
}}
else {{
n1 / n2
}}
}}
fn read_numbers() -> i32 {{
loop {{
print!("Type a number: ");
io::stdout().flush().unwrap();
let mut n1 = String::new();
io::stdin().read_line(&mut n1).unwrap();
match n1.trim().parse::<i32>() {{
Ok(n) => return n,
Err(_) => print!("Invalid number try again"),
}}
}}
}}
fn getnumber() -> (i32, i32) {{
let n1 = read_number();
let n2 = read_number();
(n1,n2)
}}
fn menu() {{
println!(" --- {} --- ");
println!("1- add");
println!("2- subtract");
println!("3- multiply");
println!("4- divide");
println!("5- quit");
}}
fn calculator() -> bool {{
menu();
print!("Choose: ");
io::stdout().flush().unwrap();
let mut optionss = String::new();
io::stdin().read_line(&mut optionss).unwrap();
let optionss = optionss.trim();

if optionss == "5" {{
println!("Bye user");
return false;
}}

let (n1, n2) = getnumber();
match optionss {{
"1" => println!("result = {{}}", sum(n1, n2)),
"2" => println!("result = {{}}", subtract(n1, n2)),
"3" => println!("result = {{}}", multiply(n1, n2)),
"4" => println!("result = {{}}", divide(n1, n2)),
_=> println!("Invalid number"),
}}
true
}}
fn main() {{
loop {{
if !calculator() {{
break;
}}
}}
}}
"#, calcname);
if Path::new("Calculator.rs").exists() {
println!("this file already exists");
}
else {
fs::write("Calculator.rs", calcs).unwrap();
println!("file created\n");
}
}
fn codes() {
println!("\x1b[1m\n********************************");
println!("**** Code Generator Rust V5 ****");
println!("********************************\x1b[0m");
print!("\nEnter a code: ");
io::stdout().flush().unwrap();
let mut input = String::new();
io::stdin().read_line(&mut input).unwrap();
let input = input.trim().to_lowercase();

if input.contains("show") {
show_generate();

println!("How to compile The code");
println!("Type => rustc Show.rs");
println!("Type => ./Show.rs\n");
println!("\nType => nano Show.rs to edit the code");
println!("\x1b[33m\nWarning: if the filename is misspiled a different file will be created\x1b[0m\n");
}
else if input.contains("calculator") {
calc_generate();

println!("\nHow to compile the code");
println!("Type => rustc Calculator.rs");
println!("Type => ./Calculator");
println!("\nType => nano Calculator.rs to edit the code");
println!("\x1b[33m\nWarning: if the filename is misspiled a different file will be created\x1b[0m\n");
}
else if input.contains("adds") {
sum_generate();

println!("\nHow to compile the code");
println!("Type => rustc Sum.rs");
println!("Type => ./Sum\n");
println!("\nType => nano Sum.rs to edit the code");
println!("\x1b[33m\nWarning: if the filename is misspiled a different file will be created\x1b[0m\n");
}
println!("\x1b[33m\nWarning: to compile the code you need to exit the Terminal\x1b[0m\n");
println!("Type => /exit to exit the Terminal\n");
}
fn tradutor() {
println!("\x1b[1m\n••••••••••••••••••••••••••••••");
println!("•••••• Tradutor Rust V1 ••••••");
println!("••••••••••••••••••••••••••••••\n");

print!("\nType a word in english: ");
io::stdout().flush().unwrap();
let mut user = String::new();
io::stdin().read_line(&mut user).unwrap();
let user = user.trim().to_lowercase();

match user.as_str() {
"good" => println!("Translation: bom\n"),
"love" => println!("Translation: amor\n"),
"hello" => println!("Translation: olá\n"),
"world" => println!("Translation: mundo\n"),
"happy" => println!("Translation: feliz\n"),
"sad" => println!("Translation: triste\n"),
"cute" => println!("Translation: fofo\n"),
"tasty" => println!("Translation: gostoso\n"),
"you" => println!("Translation: você\n"),
"fear" => println!("Translation: medo\n"),
"candy" => println!("Translation: doce\n"),
_=> println!("word not found\n"),
}
}
fn create() {
print!("Type the filename: ");
io::stdout().flush().unwrap();
let mut filename = String::new();
io::stdin().read_line(&mut filename).unwrap();
let filename = filename.trim();

println!("When you finish the code Type \"run\" on the last line");
println!("Write a Rust code\n");
io::stdout().flush().unwrap();
let mut code = String::new();
loop {
let mut line = String::new();
io::stdin().read_line(&mut line).unwrap();

if line.trim() == "run" {
break;
}
code.push_str(&line);
}
let final_name = format!("{}.rs", filename);

fs::write(&final_name, code).unwrap();

println!("\x1b[1mFile created: {}.rs", filename);
println!("How to compile the code");
println!("Type => rustc {}.rs", filename);
println!("Type => ./{}", filename);
println!("\x1b[33m\nWarning: to compile the code you need to exit the terminal\x1b[0m\n");
println!("/exit => execute this command to exit the terminal");
}
fn terminal() -> bool {
print!("\n\x1b[32m~\x1b[0m V ");
io::stdout().flush().unwrap();
let mut cmd = String::new();
io::stdin().read_line(&mut cmd).unwrap();
let cmd = cmd.trim();

if cmd == "/exit" {
println!("Bye user!\n");
return false;
}
else if cmd == "help" {
println!("Basic Commands:\n");
println!("/codes\n");
println!("/calculator\n");
println!("/tradutor\n");
println!("/createfile");
}
match cmd {
"/codes" => codes(),
"/calculator" => calculator(),
"/tradutor" => tradutor(),
"/createfile" => create(),
_=> println!("{} This command does not exist\n", cmd),
}
true
}
fn main() {
println!("\x1b[1m\nWelcome to Terminal VextOS v1.0.1-alpha\x1b[0m");
loop {
if !terminal() {
break;
}
}
}
