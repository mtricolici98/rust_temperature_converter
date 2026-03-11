use k_board::{
    keyboard::{self, Keyboard},
    keys::Keys,
};

enum UserAction {
    SwitchMode,
    EnterKey(char),
    Delete,
}

const INDICATOR_C: &'static str = "*C";
const INDICATOR_F: &'static str = "*F";

fn get_next_handled_keypress() -> Result<UserAction, String> {
    for key in Keyboard::new() {
        match key {
            Keys::Char(any) => {
                println!("Some char");
                return Ok(UserAction::EnterKey(any));
            }
            Keys::Up | Keys::Down | Keys::Left | Keys::Right => return Ok(UserAction::SwitchMode),
            Keys::Delete => {
                println!("Deleting");
                return Ok(UserAction::Delete);
            }
            _ => {}
        }
    }
    Err("No value provided".to_string())
}

fn menu(mode: bool) {
    print!("\x1B[2J");
    let mut op: Vec<char> = vec!['*', ' '];
    if mode == true {
        op[0] = ' ';
        op[1] = '*';
    }
    println!("Welcome to my converter, choose the unit to convert to!");
    println!("Use arrow keys to switch modes, type the number to start converting.");
    println!("[{}] C to F \n[{}] F to C", op[0], op[1]);
}

fn convert_c_to_f(val: f64) -> f64 {
    (val * 9.0 / 5.0) + 32.0
}

fn convert_f_to_c(val: f64) -> f64 {
    (val - 32.0) * 5.0 / 9.0
}
fn convert_and_show_result(value: &String, to_celsius: bool) {
    let val_to_show;
    if (value.is_empty()) {
        println!("Type a value!");
        return;
    }
    let value: f64 = value.parse().unwrap();
    let temp1_indicator = if to_celsius { INDICATOR_F } else { INDICATOR_C };
    let temp2_indicator = if to_celsius { INDICATOR_C } else { INDICATOR_F };
    if to_celsius {
        val_to_show = convert_f_to_c(value);
    } else {
        val_to_show = convert_c_to_f(value);
    }
    println!("{value} {temp1_indicator} = {val_to_show:.2} {temp2_indicator}")
}

fn add_to_value_with_validation(value: &mut String, el: char) {
    if el == '.' && value.contains('.') {
        return;
    }
    if "01234567890.".contains(el) {
        value.push(el)
    }
}

fn main() {
    let mut value = String::new();
    let mut mode = false;
    loop {
        menu(mode);
        convert_and_show_result(&value, mode);
        let key_press = get_next_handled_keypress().unwrap();
        match key_press {
            UserAction::SwitchMode => mode = !mode,
            UserAction::EnterKey(val) => {
                add_to_value_with_validation(&mut value, val);
            }
            UserAction::Delete => {
                value.pop();
            }
            _ => println!("Unnnn"),
        }
    }
}
