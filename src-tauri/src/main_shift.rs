// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use enigo::{Enigo, Keyboard, Settings};
use inputbot::{handle_input_events, BlockInput::*, KeybdKey::{self, *}};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
fn disable_binding_on_any_key(binding_active_clone: Arc<Mutex<bool>>) {
    let special_keys = vec![
        KeybdKey::LControlKey, KeybdKey::RControlKey, KeybdKey::LShiftKey, KeybdKey::RShiftKey, KeybdKey::CapsLockKey,
        KeybdKey::AKey, KeybdKey::DKey, KeybdKey::HKey, KeybdKey::IKey, KeybdKey::LKey, KeybdKey::MKey, KeybdKey::NKey,
        KeybdKey::GKey, KeybdKey::JKey, KeybdKey::RKey, KeybdKey::EKey, KeybdKey::SKey, KeybdKey::ZKey, KeybdKey::TKey,
        KeybdKey::UKey,
    ];

    let all_keys = vec![
        KeybdKey::AKey, KeybdKey::BKey, KeybdKey::CKey, KeybdKey::DKey, KeybdKey::EKey,
        KeybdKey::FKey, KeybdKey::GKey, KeybdKey::HKey, KeybdKey::IKey, KeybdKey::JKey,
        KeybdKey::KKey, KeybdKey::LKey, KeybdKey::MKey, KeybdKey::NKey, KeybdKey::OKey,
        KeybdKey::PKey, KeybdKey::QKey, KeybdKey::RKey, KeybdKey::SKey, KeybdKey::TKey,
        KeybdKey::UKey, KeybdKey::VKey, KeybdKey::WKey, KeybdKey::XKey, KeybdKey::YKey,
        KeybdKey::ZKey, KeybdKey::Numpad0Key, KeybdKey::Numpad1Key, KeybdKey::Numpad2Key, KeybdKey::Numpad3Key,
        KeybdKey::Numpad4Key, KeybdKey::Numpad5Key, KeybdKey::Numpad6Key, KeybdKey::Numpad7Key, KeybdKey::Numpad8Key,
        KeybdKey::Numpad9Key, KeybdKey::F1Key, KeybdKey::F2Key, KeybdKey::F3Key, KeybdKey::F4Key,
        KeybdKey::F5Key, KeybdKey::F6Key, KeybdKey::F7Key, KeybdKey::F8Key, KeybdKey::F9Key,
        KeybdKey::F10Key, KeybdKey::F11Key, KeybdKey::F12Key, KeybdKey::EscapeKey, KeybdKey::SpaceKey,
        KeybdKey::EnterKey, KeybdKey::TabKey, KeybdKey::BackspaceKey, KeybdKey::InsertKey, KeybdKey::DeleteKey,
        KeybdKey::HomeKey, KeybdKey::EndKey, KeybdKey::PageUpKey, KeybdKey::PageDownKey, KeybdKey::LeftKey,
        KeybdKey::RightKey, KeybdKey::UpKey, KeybdKey::DownKey,  KeybdKey::LAltKey, KeybdKey::RAltKey, KeybdKey::CapsLockKey,
        KeybdKey::NumLockKey, KeybdKey::ScrollLockKey, KeybdKey::VolumeMuteKey, KeybdKey::VolumeDownKey,
        KeybdKey::VolumeUpKey, KeybdKey::MediaNextTrackKey, KeybdKey::MediaPrevTrackKey, KeybdKey::MediaStopKey,
        KeybdKey::MediaPlayPauseKey, KeybdKey::BackquoteKey, KeybdKey::SlashKey, KeybdKey::BackslashKey,
        KeybdKey::CommaKey, KeybdKey::PeriodKey, KeybdKey::MinusKey, KeybdKey::QuoteKey, KeybdKey::SemicolonKey,
        KeybdKey::LBracketKey, KeybdKey::RBracketKey, KeybdKey::EqualKey
    ];

    for key in all_keys {
        if !special_keys.contains(&key) {
            let binding_active_clone = Arc::clone(&binding_active_clone);
            key.blockable_bind(move || {
                let mut binding_active = binding_active_clone.lock().unwrap();
                *binding_active = false;
                DontBlock
            });
        }
    }
}
#[tauri::command]
fn bindevent() {
    // inputbot::init_device();
    // Timoue Required For Long Press Action to activate Diacritic
    let long_press_timeout = 100; // adjust this value to your liking
    // Variable to check if the binding is active or not 
    let binding_active = Arc::new(Mutex::new(false));
    // Enigo instance to send key events
    let enigo = Arc::new(Mutex::new(Enigo::new(&Settings::default()).unwrap()));

    for shift_key in &[LShiftKey, RShiftKey] {
        let shift_key = *shift_key;
        let binding_active = Arc::clone(&binding_active);
        let enigo = Arc::clone(&enigo);
        let last_bind_time = Arc::new(Mutex::new(Instant::now()));

        shift_key.bind(move || {
            let start_time = Instant::now();
            thread::sleep(Duration::from_millis(long_press_timeout));
            let current_time = Instant::now();
            if current_time.duration_since(start_time) >= Duration::from_millis(long_press_timeout) {
                *binding_active.lock().unwrap() = true;
                *last_bind_time.lock().unwrap() = Instant::now();
                dbg!("{:?} Long press detected", shift_key);

                let binding_active_clone = Arc::clone(&binding_active);
                let enigo_clone = Arc::clone(&enigo);
                let last_bind_time_clone = Arc::clone(&last_bind_time);
                disable_binding_on_any_key(Arc::clone(&binding_active_clone));

                let keys = vec![
                    (AKey, "ā", "Ā"), (DKey, "ḍ", "Ḍ"), (HKey, "ḥ", "Ḥ"), (IKey, "ī", "Ī"),
                    (LKey, "ḷ", "Ḷ"), (MKey, "ṁ", "Ṁ"), (NKey, "ṇ", "Ṇ"), (GKey, "ṅ", "Ṅ"),
                    (JKey, "ñ", "Ñ"), (RKey, "ṛ", "Ṛ"), (EKey, "ṝ", "Ṝ"), (SKey, "ṣ", "Ṣ"),
                    (ZKey, "ś", "Ś"), (TKey, "ṭ", "Ṭ"), (UKey, "ū", "Ū"),
                ];

                let keys_clone = keys.clone();
                for (key, lower, upper) in keys_clone {
                    let binding_active_clone = Arc::clone(&binding_active_clone);
                    let enigo_clone = Arc::clone(&enigo_clone);
                    let last_bind_time = Arc::clone(&last_bind_time_clone);
                    key.blockable_bind(move || {
                        if !LControlKey.is_pressed()
                            && !RControlKey.is_pressed()
                            && !LShiftKey.is_pressed()
                            && !RShiftKey.is_pressed()
                            && *binding_active_clone.lock().unwrap()
                        {
                            let binding_active = Arc::clone(&binding_active_clone);
                            let enigo = Arc::clone(&enigo_clone);
                            *last_bind_time.lock().unwrap() = Instant::now();
                            thread::spawn(move || {
                                let mut binding_active = binding_active.lock().unwrap();
                                let mut enigo = enigo.lock().unwrap();
                                if *binding_active {
                                    *binding_active = false;
                                    dbg!("{:?} pressed", key);
                                    if !CapsLockKey.is_toggled()
                                        && !LShiftKey.is_pressed()
                                        && !RShiftKey.is_pressed()
                                    {
                                        enigo.text(lower).unwrap();
                                    } else {
                                        enigo.text(upper).unwrap();
                                    }
                                }
                            });
                            Block
                        } else {
                            let binding_active = Arc::clone(&binding_active_clone);
                            let mut binding_active = binding_active.lock().unwrap();
                            *binding_active = false;
                            DontBlock
                        }
                    });
                }

                let keys_clone = keys.clone();
                thread::spawn({
                    let binding_active = Arc::clone(&binding_active);
                    let last_bind_time = Arc::clone(&last_bind_time_clone);
                    move || {
                        loop {
                            thread::sleep(Duration::from_millis(1500));
                            let last_bind = last_bind_time.lock().unwrap().elapsed();
                            if last_bind >= Duration::from_millis(1500) {
                                *binding_active.lock().unwrap() = false;
                                for (key, _, _) in &keys_clone {
                                    key.unbind();
                                }
                                break;
                            }
                        }
                    }
                });
            }
        });
    }


    handle_input_events();
}

#[tauri::command]
async fn press() {
    dbg!("Apploaded");
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![press, bindevent])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
