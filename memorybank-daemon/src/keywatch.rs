use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use device_query::{DeviceQuery, DeviceState, Keycode};
use crate::Config;

pub fn start_key_watcher(config: Arc<Mutex<Config>>) {
    thread::spawn(move || {
        let device_state = DeviceState::new();
        let mut last_keys = HashSet::new();
        let mut last_trigger_time = Instant::now();
        loop {
            let config = config.lock().unwrap().clone();
            let keys: HashSet<_> = device_state.get_keys().into_iter().collect();
            if keys != last_keys {
                println!("🔍 Keys pressed: {:?}", keys);

                if config.is_enabled {
                    let copy_modifiers = vec![
                        config.modifier_1.clone(),
                        config.modifier_2.clone(),
                        config.modifier_3.clone(),
                    ];

                    // Map modifiers to keycodes
                    let pressed_mods: Vec<_> = copy_modifiers
                        .iter()
                        .filter_map(|m| parse_keycode(m))
                        .collect();
                    // Check if all modifiers are currently pressed
                    let all_mods_pressed = pressed_mods.iter().all(|key| keys.contains(key));
                    // Check if any number key is pressed (0–9)
                    let number_keys = [
                        Keycode::Key0, Keycode::Key1, Keycode::Key2, Keycode::Key3, Keycode::Key4,
                        Keycode::Key5, Keycode::Key6, Keycode::Key7, Keycode::Key8, Keycode::Key9,
                    ];
                    let number_pressed = number_keys.iter().find(|key| keys.contains(key));

                    if all_mods_pressed {
                        if let Some(num_key) = number_pressed {
                            println!("✅ Combination detected: {:?} + {:?}", copy_modifiers, num_key);
                            if last_trigger_time.elapsed() > Duration::from_millis(500) {
                                trigger_copy_slot(&copy_modifiers, num_key);
                                last_trigger_time = Instant::now();
                            }
                        }
                    }
                }

                last_keys = keys;
            }

            thread::sleep(Duration::from_millis(50));
        }
    });
}

fn parse_keycode(s: &str) -> Option<Keycode> {
    match s.to_lowercase().as_str() {
        "ctrl" | "control" => Some(Keycode::LControl),
        "shift" | "lshift" => Some(Keycode::LShift),
        "alt" | "option" | "lalt" | "loption" => Some(Keycode::LAlt),
        "cmd" | "command" | "meta" | "lcmd" | "lmeta" => Some(Keycode::LMeta),
        "none" => None,
        "1" => Some(Keycode::Key1),
        "2" => Some(Keycode::Key2),
        "3" => Some(Keycode::Key3),
        "4" => Some(Keycode::Key4),
        "5" => Some(Keycode::Key5),
        "6" => Some(Keycode::Key6),
        "7" => Some(Keycode::Key7),
        "8" => Some(Keycode::Key8),
        "9" => Some(Keycode::Key9),
        "0" => Some(Keycode::Key0),
        _ => None,
    }
}

fn trigger_copy_slot(mods: &Vec<String>, key: &Keycode) {
    println!("🧠 Atajo de COPY detectado: {:?} + {:?}", mods, key);
    // TODO: aquí irá la lógica para copiar al slot correspondiente.
}
