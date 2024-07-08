// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::time::{Instant, Duration};
use std::sync::{Arc, Mutex};
use enigo::{
    Enigo, Keyboard, Settings,
};
use inputbot::{handle_input_events, KeybdKey::* ,  BlockInput::*,};
use std::thread;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command


#[tauri::command]
fn bindevent(){
        // inputbot::init_device();
        let long_press_timeout = 100; // adjust this value to your liking
        let binding_active = Arc::new(Mutex::new(false));
        let enigo =  Arc::new(Mutex::new(Enigo::new(&Settings::default()).unwrap()));
    
    

        LShiftKey.bind(move|| {
            let start_time = Instant::now();
            thread::sleep(Duration::from_millis(long_press_timeout));
            let current_time = Instant::now();
            if current_time.duration_since(start_time) >= Duration::from_millis(long_press_timeout) {
                *binding_active.lock().unwrap() = true;
                dbg!("Lctrl Long press detected");


let binding_active_clone = Arc::clone(&binding_active);
let enigo_clone = Arc::clone(&enigo);

AKey.blockable_bind(move || {
    if !LControlKey.is_pressed() && !RControlKey.is_pressed() && *binding_active_clone.lock().unwrap() {
        let binding_active = Arc::clone(&binding_active_clone);
        let enigo = Arc::clone(&enigo_clone);
        thread::spawn(move || {
            let mut binding_active = binding_active.lock().unwrap();
            let mut enigo = enigo.lock().unwrap();
            if *binding_active {
                *binding_active=false;
                dbg!("AKey pressed");
                if !CapsLockKey.is_toggled() && !LShiftKey.is_pressed() && !RShiftKey.is_pressed() {
                    enigo.text("ā").unwrap(); // send a when Caps Lock is off
                } else {
                    enigo.text("Ā").unwrap(); // send A when Caps Lock is on
                }
               
              
            }
        });
        Block
    } else {
        DontBlock
    }
});

let binding_active_clone = Arc::clone(&binding_active);
let enigo_clone = Arc::clone(&enigo);
DKey.blockable_bind(move || {
    if !LControlKey.is_pressed() && !RControlKey.is_pressed() && *binding_active_clone.lock().unwrap() {
        let binding_active = Arc::clone(&binding_active_clone);
        let enigo = Arc::clone(&enigo_clone);
        thread::spawn(move || {
            let mut binding_active = binding_active.lock().unwrap();
            let mut enigo = enigo.lock().unwrap();
            if *binding_active {
                *binding_active=false;
                dbg!("DKey pressed");
                if !CapsLockKey.is_toggled() && !LShiftKey.is_pressed() && !RShiftKey.is_pressed() {
                    enigo.text("ḍ").unwrap(); // send a when Caps Lock is off
                } else {
                    enigo.text("Ḍ").unwrap(); // send A when Caps Lock is on
                }
               
              
            }
        });
        Block
    } else {
        DontBlock
    }
});

let binding_active_clone = Arc::clone(&binding_active);
let enigo_clone = Arc::clone(&enigo);
HKey.blockable_bind(move || {
    if !LControlKey.is_pressed() && !RControlKey.is_pressed() && *binding_active_clone.lock().unwrap(){
        let binding_active = Arc::clone(&binding_active_clone);
        let enigo = Arc::clone(&enigo_clone);
        thread::spawn(move || {
            let mut binding_active = binding_active.lock().unwrap();
            let mut enigo = enigo.lock().unwrap();
            if *binding_active {
                *binding_active=false;
                dbg!("HKey pressed");
                if !CapsLockKey.is_toggled() && !LShiftKey.is_pressed() && !RShiftKey.is_pressed() {
                    enigo.text("ḥ").unwrap(); // send a when Caps Lock is off
                } else {
                    enigo.text("Ḥ").unwrap(); // send A when Caps Lock is on
                }
               
              
            }
        });
        Block
    } else {
        DontBlock
    }
});


let binding_active_clone = Arc::clone(&binding_active);
let enigo_clone = Arc::clone(&enigo);
IKey.blockable_bind(move || {
    if !LControlKey.is_pressed() && !RControlKey.is_pressed() && *binding_active_clone.lock().unwrap() {
        let binding_active = Arc::clone(&binding_active_clone);
        let enigo = Arc::clone(&enigo_clone);
        thread::spawn(move || {
            let mut binding_active = binding_active.lock().unwrap();
            let mut enigo = enigo.lock().unwrap();
            if *binding_active {
                *binding_active=false;
                dbg!("IKey pressed");
                if !CapsLockKey.is_toggled() && !LShiftKey.is_pressed() && !RShiftKey.is_pressed() {
                    enigo.text("ī").unwrap(); // send a when Caps Lock is off
                } else {
                    enigo.text("Ī").unwrap(); // send A when Caps Lock is on
                }
               
              
            }
        });
        Block
    } else {
        DontBlock
    }
});

let binding_active_clone = Arc::clone(&binding_active);
let enigo_clone = Arc::clone(&enigo);
LKey.blockable_bind(move || {
    if !LControlKey.is_pressed() && !RControlKey.is_pressed() && *binding_active_clone.lock().unwrap() {
        let binding_active = Arc::clone(&binding_active_clone);
        let enigo = Arc::clone(&enigo_clone);
        thread::spawn(move || {
            let mut binding_active = binding_active.lock().unwrap();
            let mut enigo = enigo.lock().unwrap();
            if *binding_active {
                *binding_active=false;
                dbg!("LKey pressed");
                if !CapsLockKey.is_toggled() && !LShiftKey.is_pressed() && !RShiftKey.is_pressed() {
                    enigo.text("ḷ").unwrap(); // send a when Caps Lock is off
                } else {
                    enigo.text("Ḷ").unwrap(); // send A when Caps Lock is on
                }
               
              
            }
        });
        Block
    } else {
        DontBlock
    }
});

let binding_active_clone = Arc::clone(&binding_active);
let enigo_clone = Arc::clone(&enigo);
MKey.blockable_bind(move || {
    if !LControlKey.is_pressed() && !RControlKey.is_pressed() && *binding_active_clone.lock().unwrap() {
        let binding_active = Arc::clone(&binding_active_clone);
        let enigo = Arc::clone(&enigo_clone);
        thread::spawn(move || {
            let mut binding_active = binding_active.lock().unwrap();
            let mut enigo = enigo.lock().unwrap();
            if *binding_active {
                *binding_active=false;
                dbg!("MKey pressed");
                if !CapsLockKey.is_toggled() && !LShiftKey.is_pressed() && !RShiftKey.is_pressed() {
                    enigo.text("ṁ").unwrap(); // send a when Caps Lock is off
                } else {
                    enigo.text("Ṁ").unwrap(); // send A when Caps Lock is on
                }
               
              
            }
        });
        Block
    } else {
        DontBlock
    }
});


let binding_active_clone = Arc::clone(&binding_active);
let enigo_clone = Arc::clone(&enigo);
NKey.blockable_bind(move || {
    if !LControlKey.is_pressed() && !RControlKey.is_pressed() && *binding_active_clone.lock().unwrap() {
        let binding_active = Arc::clone(&binding_active_clone);
        let enigo = Arc::clone(&enigo_clone);
        thread::spawn(move || {
            let mut binding_active = binding_active.lock().unwrap();
            let mut enigo = enigo.lock().unwrap();
            if *binding_active {
                *binding_active=false;
                dbg!("NKey pressed");
                if !CapsLockKey.is_toggled() && !LShiftKey.is_pressed() && !RShiftKey.is_pressed() {
                    enigo.text("ṇ").unwrap(); // send a when Caps Lock is off
                } else {
                    enigo.text("Ṇ").unwrap(); // send A when Caps Lock is on
                }
               
              
            }
        });
        Block
    } else {
        DontBlock
    }
});


let binding_active_clone = Arc::clone(&binding_active);
let enigo_clone = Arc::clone(&enigo);
GKey.blockable_bind(move || {
    if !LControlKey.is_pressed() && !RControlKey.is_pressed() && *binding_active_clone.lock().unwrap() {
        let binding_active = Arc::clone(&binding_active_clone);
        let enigo = Arc::clone(&enigo_clone);
        thread::spawn(move || {
            let mut binding_active = binding_active.lock().unwrap();
            let mut enigo = enigo.lock().unwrap();
            if *binding_active {
                *binding_active=false;
                dbg!("GKey pressed");
                if !CapsLockKey.is_toggled() && !LShiftKey.is_pressed() && !RShiftKey.is_pressed() {
                    enigo.text("ṅ").unwrap(); // send a when Caps Lock is off
                } else {
                    enigo.text("Ṅ").unwrap(); // send A when Caps Lock is on
                }
               
              
            }
        });
        Block
    } else {
        DontBlock
    }
});

let binding_active_clone = Arc::clone(&binding_active);
let enigo_clone = Arc::clone(&enigo);
JKey.blockable_bind(move || {
    if !LControlKey.is_pressed() && !RControlKey.is_pressed() && *binding_active_clone.lock().unwrap(){
        let binding_active = Arc::clone(&binding_active_clone);
        let enigo = Arc::clone(&enigo_clone);
        thread::spawn(move || {
            let mut binding_active = binding_active.lock().unwrap();
            let mut enigo = enigo.lock().unwrap();
            if *binding_active {
                *binding_active=false;
                dbg!("JKey pressed");
                if !CapsLockKey.is_toggled() && !LShiftKey.is_pressed() && !RShiftKey.is_pressed() {
                    enigo.text("ñ").unwrap(); // send a when Caps Lock is off
                } else {
                    enigo.text("Ñ").unwrap(); // send A when Caps Lock is on
                }
               
              
            }
        });
        Block
    } else {
        DontBlock
    }
});

let binding_active_clone = Arc::clone(&binding_active);
let enigo_clone = Arc::clone(&enigo);
RKey.blockable_bind(move || {
    if !LControlKey.is_pressed() && !RControlKey.is_pressed() && *binding_active_clone.lock().unwrap(){
        let binding_active = Arc::clone(&binding_active_clone);
        let enigo = Arc::clone(&enigo_clone);
        thread::spawn(move || {
            let mut binding_active = binding_active.lock().unwrap();
            let mut enigo = enigo.lock().unwrap();
            if *binding_active {
                *binding_active=false;
                dbg!("RKey pressed");
                if !CapsLockKey.is_toggled() && !LShiftKey.is_pressed() && !RShiftKey.is_pressed() {
                    enigo.text("ṛ").unwrap(); // send a when Caps Lock is off
                } else {
                    enigo.text("Ṛ").unwrap(); // send A when Caps Lock is on
                }
               
              
            }
        });
        Block
    } else {
        DontBlock
    }
});

let binding_active_clone = Arc::clone(&binding_active);
let enigo_clone = Arc::clone(&enigo);
EKey.blockable_bind(move || {
    if !LControlKey.is_pressed() && !RControlKey.is_pressed() && *binding_active_clone.lock().unwrap() {
        let binding_active = Arc::clone(&binding_active_clone);
        let enigo = Arc::clone(&enigo_clone);
        thread::spawn(move || {
            let mut binding_active = binding_active.lock().unwrap();
            let mut enigo = enigo.lock().unwrap();
            if *binding_active {
                *binding_active=false;
                dbg!("EKey pressed");
                if !CapsLockKey.is_toggled() && !LShiftKey.is_pressed() && !RShiftKey.is_pressed() {
                    enigo.text("ṝ").unwrap(); // send a when Caps Lock is off
                } else {
                    enigo.text("Ṝ").unwrap(); // send A when Caps Lock is on
                }
               
              
            }
        });
        Block
    } else {
        DontBlock
    }
});


let binding_active_clone = Arc::clone(&binding_active);
let enigo_clone = Arc::clone(&enigo);
SKey.blockable_bind(move || {
    if !LControlKey.is_pressed() && !RControlKey.is_pressed() && *binding_active_clone.lock().unwrap() {
        let binding_active = Arc::clone(&binding_active_clone);
        let enigo = Arc::clone(&enigo_clone);
        thread::spawn(move || {
            let mut binding_active = binding_active.lock().unwrap();
            let mut enigo = enigo.lock().unwrap();
            if *binding_active {
                *binding_active=false;
                dbg!("SKey pressed");
                if !CapsLockKey.is_toggled() && !LShiftKey.is_pressed() && !RShiftKey.is_pressed() {
                    enigo.text("ṣ").unwrap(); // send a when Caps Lock is off
                } else {
                    enigo.text("Ṣ").unwrap(); // send A when Caps Lock is on
                }
               
              
            }
        });
        Block
    } else {
        DontBlock
    }
});


let binding_active_clone = Arc::clone(&binding_active);
let enigo_clone = Arc::clone(&enigo);
ZKey.blockable_bind(move || {
    if !LControlKey.is_pressed() && !RControlKey.is_pressed() && *binding_active_clone.lock().unwrap() {
        let binding_active = Arc::clone(&binding_active_clone);
        let enigo = Arc::clone(&enigo_clone);
        thread::spawn(move || {
            let mut binding_active = binding_active.lock().unwrap();
            let mut enigo = enigo.lock().unwrap();
            if *binding_active {
                *binding_active=false;
                dbg!("ZKey pressed");
                if !CapsLockKey.is_toggled() && !LShiftKey.is_pressed() && !RShiftKey.is_pressed() {
                    enigo.text("ś").unwrap(); // send a when Caps Lock is off
                } else {
                    enigo.text("Ś").unwrap(); // send A when Caps Lock is on
                }
               
              
            }
        });
        Block
    } else {
        DontBlock
    }
});

let binding_active_clone = Arc::clone(&binding_active);
let enigo_clone = Arc::clone(&enigo);
TKey.blockable_bind(move || {
    if !LControlKey.is_pressed() && !RControlKey.is_pressed() && *binding_active_clone.lock().unwrap() {
        let binding_active = Arc::clone(&binding_active_clone);
        let enigo = Arc::clone(&enigo_clone);
        thread::spawn(move || {
            let mut binding_active = binding_active.lock().unwrap();
            let mut enigo = enigo.lock().unwrap();
            if *binding_active {
                *binding_active=false;
                dbg!("TKey pressed");
                if !CapsLockKey.is_toggled() && !LShiftKey.is_pressed() && !RShiftKey.is_pressed() {
                    enigo.text("ṭ").unwrap(); // send a when Caps Lock is off
                } else {
                    enigo.text("Ṭ").unwrap(); // send A when Caps Lock is on
                }
               
              
            }
        });
        Block
    } else {
        DontBlock
    }
});

let binding_active_clone = Arc::clone(&binding_active);
let enigo_clone = Arc::clone(&enigo);
UKey.blockable_bind(move || {
    if !LControlKey.is_pressed() && !RControlKey.is_pressed() && *binding_active_clone.lock().unwrap(){
        let binding_active = Arc::clone(&binding_active_clone);
        let enigo = Arc::clone(&enigo_clone);
        thread::spawn(move || {
            let mut binding_active = binding_active.lock().unwrap();
            let mut enigo = enigo.lock().unwrap();
            if *binding_active {
                *binding_active=false;
                dbg!("UKey pressed");
                if !CapsLockKey.is_toggled() && !LShiftKey.is_pressed() && !RShiftKey.is_pressed() {
                    enigo.text("ū").unwrap(); // send a when Caps Lock is off
                } else {
                    enigo.text("Ū").unwrap(); // send A when Caps Lock is on
                }
               
              
            }
        });
        Block
    } else {
        DontBlock
    }
});
              thread::spawn({
        let binding_active = Arc::clone(&binding_active);
        move || {
            thread::sleep(Duration::from_millis(1000));
            *binding_active.lock().unwrap() = false;
            AKey.unbind();
            DKey.unbind();
            HKey.unbind();
            IKey.unbind();
            LKey.unbind();
            MKey.unbind();
            NKey.unbind();
            GKey.unbind();
            JKey.unbind();
            RKey.unbind();
            EKey.unbind();
            SKey.unbind();
            ZKey.unbind();
            TKey.unbind();
            UKey.unbind();
        }
    });
    
            }
        });

        let binding_active = Arc::new(Mutex::new(false));
        let enigo =  Arc::new(Mutex::new(Enigo::new(&Settings::default()).unwrap()));
    
    
    
        RShiftKey.bind(move|| {
            let start_time = Instant::now();
            thread::sleep(Duration::from_millis(long_press_timeout));
            let current_time = Instant::now();
            if current_time.duration_since(start_time) >= Duration::from_millis(long_press_timeout) {
                *binding_active.lock().unwrap() = true;
                dbg!("Lctrl Long press detected");


let binding_active_clone = Arc::clone(&binding_active);
let enigo_clone = Arc::clone(&enigo);

AKey.blockable_bind(move || {
    if !LControlKey.is_pressed() && !RControlKey.is_pressed() && *binding_active_clone.lock().unwrap() {
        let binding_active = Arc::clone(&binding_active_clone);
        let enigo = Arc::clone(&enigo_clone);
        thread::spawn(move || {
            let mut binding_active = binding_active.lock().unwrap();
            let mut enigo = enigo.lock().unwrap();
            if *binding_active {
                *binding_active  = false;
                dbg!("AKey pressed");
                if !CapsLockKey.is_toggled() && !LShiftKey.is_pressed() && !RShiftKey.is_pressed() {
                    enigo.text("ā").unwrap(); // send a when Caps Lock is off
                } else {
                    enigo.text("Ā").unwrap(); // send A when Caps Lock is on
                }
               
              
            }
        });
        Block
    } else {
        DontBlock
    }
});

let binding_active_clone = Arc::clone(&binding_active);
let enigo_clone = Arc::clone(&enigo);
DKey.blockable_bind(move || {
    if !LControlKey.is_pressed() && !RControlKey.is_pressed() && *binding_active_clone.lock().unwrap() {
        let binding_active = Arc::clone(&binding_active_clone);
        let enigo = Arc::clone(&enigo_clone);
        thread::spawn(move || {
            let mut binding_active = binding_active.lock().unwrap();
            let mut enigo = enigo.lock().unwrap();
            if *binding_active {
                *binding_active  =  false;
                dbg!("DKey pressed");
                if !CapsLockKey.is_toggled() && !LShiftKey.is_pressed() && !RShiftKey.is_pressed() {
                    enigo.text("ḍ").unwrap(); // send a when Caps Lock is off
                } else {
                    enigo.text("Ḍ").unwrap(); // send A when Caps Lock is on
                }
               
              
            }
        });
        Block
    } else {
        DontBlock
    }
});

let binding_active_clone = Arc::clone(&binding_active);
let enigo_clone = Arc::clone(&enigo);
HKey.blockable_bind(move || {
    if !LControlKey.is_pressed() && !RControlKey.is_pressed()  && *binding_active_clone.lock().unwrap(){
        let binding_active = Arc::clone(&binding_active_clone);
        let enigo = Arc::clone(&enigo_clone);
        thread::spawn(move || {
            let mut binding_active = binding_active.lock().unwrap();
            let mut enigo = enigo.lock().unwrap();
            if *binding_active {
                *binding_active  =  false;
                dbg!("HKey pressed");
                if !CapsLockKey.is_toggled() && !LShiftKey.is_pressed() && !RShiftKey.is_pressed() {
                    enigo.text("ḥ").unwrap(); // send a when Caps Lock is off
                } else {
                    enigo.text("Ḥ").unwrap(); // send A when Caps Lock is on
                }
               
              
            }
        });
        Block
    } else {
        DontBlock
    }
});


let binding_active_clone = Arc::clone(&binding_active);
let enigo_clone = Arc::clone(&enigo);
IKey.blockable_bind(move || {
    if !LControlKey.is_pressed() && !RControlKey.is_pressed() && *binding_active_clone.lock().unwrap() {
        let binding_active = Arc::clone(&binding_active_clone);
        let enigo = Arc::clone(&enigo_clone);
        thread::spawn(move || {
            let mut binding_active = binding_active.lock().unwrap();
            let mut enigo = enigo.lock().unwrap();
            if *binding_active {
                *binding_active  =  false;
                dbg!("IKey pressed");
                if !CapsLockKey.is_toggled() && !LShiftKey.is_pressed() && !RShiftKey.is_pressed() {
                    enigo.text("ī").unwrap(); // send a when Caps Lock is off
                } else {
                    enigo.text("Ī").unwrap(); // send A when Caps Lock is on
                }
               
              
            }
        });
        Block
    } else {
        DontBlock
    }
});

let binding_active_clone = Arc::clone(&binding_active);
let enigo_clone = Arc::clone(&enigo);
LKey.blockable_bind(move || {
    if !LControlKey.is_pressed() && !RControlKey.is_pressed() && *binding_active_clone.lock().unwrap() {
        let binding_active = Arc::clone(&binding_active_clone);
        let enigo = Arc::clone(&enigo_clone);
        thread::spawn(move || {
            let mut binding_active = binding_active.lock().unwrap();
            let mut enigo = enigo.lock().unwrap();
            if *binding_active {
                *binding_active  =  false;
                dbg!("LKey pressed");
                if !CapsLockKey.is_toggled() && !LShiftKey.is_pressed() && !RShiftKey.is_pressed() {
                    enigo.text("ḷ").unwrap(); // send a when Caps Lock is off
                } else {
                    enigo.text("Ḷ").unwrap(); // send A when Caps Lock is on
                }
               
              
            }
        });
        Block
    } else {
        DontBlock
    }
});

let binding_active_clone = Arc::clone(&binding_active);
let enigo_clone = Arc::clone(&enigo);
MKey.blockable_bind(move || {
    if !LControlKey.is_pressed() && !RControlKey.is_pressed() && *binding_active_clone.lock().unwrap() {
        let binding_active = Arc::clone(&binding_active_clone);
        let enigo = Arc::clone(&enigo_clone);
        thread::spawn(move || {
            let mut binding_active = binding_active.lock().unwrap();
            let mut enigo = enigo.lock().unwrap();
            if *binding_active {
                *binding_active  =  false;
                dbg!("MKey pressed");
                if !CapsLockKey.is_toggled() && !LShiftKey.is_pressed() && !RShiftKey.is_pressed() {
                    enigo.text("ṁ").unwrap(); // send a when Caps Lock is off
                } else {
                    enigo.text("Ṁ").unwrap(); // send A when Caps Lock is on
                }
               
              
            }
        });
        Block
    } else {
        DontBlock
    }
});


let binding_active_clone = Arc::clone(&binding_active);
let enigo_clone = Arc::clone(&enigo);
NKey.blockable_bind(move || {
    if !LControlKey.is_pressed() && !RControlKey.is_pressed() && *binding_active_clone.lock().unwrap() {
        let binding_active = Arc::clone(&binding_active_clone);
        let enigo = Arc::clone(&enigo_clone);
        thread::spawn(move || {
            let mut binding_active = binding_active.lock().unwrap();
            let mut enigo = enigo.lock().unwrap();
            if *binding_active {
                *binding_active  =  false;
                dbg!("NKey pressed");
                if !CapsLockKey.is_toggled() && !LShiftKey.is_pressed() && !RShiftKey.is_pressed() {
                    enigo.text("ṇ").unwrap(); // send a when Caps Lock is off
                } else {
                    enigo.text("Ṇ").unwrap(); // send A when Caps Lock is on
                }
               
              
            }
        });
        Block
    } else {
        DontBlock
    }
});


let binding_active_clone = Arc::clone(&binding_active);
let enigo_clone = Arc::clone(&enigo);
GKey.blockable_bind(move || {
    if !LControlKey.is_pressed() && !RControlKey.is_pressed() && *binding_active_clone.lock().unwrap() {
        let binding_active = Arc::clone(&binding_active_clone);
        let enigo = Arc::clone(&enigo_clone);
        thread::spawn(move || {
            let mut binding_active = binding_active.lock().unwrap();
            let mut enigo = enigo.lock().unwrap();
            if *binding_active {
                *binding_active  =  false;
                dbg!("GKey pressed");
                if !CapsLockKey.is_toggled() && !LShiftKey.is_pressed() && !RShiftKey.is_pressed() {
                    enigo.text("ṅ").unwrap(); // send a when Caps Lock is off
                } else {
                    enigo.text("Ṅ").unwrap(); // send A when Caps Lock is on
                }
               
              
            }
        });
        Block
    } else {
        DontBlock
    }
});

let binding_active_clone = Arc::clone(&binding_active);
let enigo_clone = Arc::clone(&enigo);
JKey.blockable_bind(move || {
    if !LControlKey.is_pressed() && !RControlKey.is_pressed() && *binding_active_clone.lock().unwrap() {
        let binding_active = Arc::clone(&binding_active_clone);
        let enigo = Arc::clone(&enigo_clone);
        thread::spawn(move || {
            let mut binding_active = binding_active.lock().unwrap();
            let mut enigo = enigo.lock().unwrap();
            if *binding_active {
                *binding_active  =  false;
                dbg!("JKey pressed");
                if !CapsLockKey.is_toggled() && !LShiftKey.is_pressed() && !RShiftKey.is_pressed() {
                    enigo.text("ñ").unwrap(); // send a when Caps Lock is off
                } else {
                    enigo.text("Ñ").unwrap(); // send A when Caps Lock is on
                }
               
              
            }
        });
        Block
    } else {
        DontBlock
    }
});

let binding_active_clone = Arc::clone(&binding_active);
let enigo_clone = Arc::clone(&enigo);
RKey.blockable_bind(move || {
    if !LControlKey.is_pressed() && !RControlKey.is_pressed() && *binding_active_clone.lock().unwrap() {
        let binding_active = Arc::clone(&binding_active_clone);
        let enigo = Arc::clone(&enigo_clone);
        thread::spawn(move || {
            let mut binding_active = binding_active.lock().unwrap();
            let mut enigo = enigo.lock().unwrap();
            if *binding_active {
                *binding_active  =  false;
                dbg!("RKey pressed");
                if !CapsLockKey.is_toggled() && !LShiftKey.is_pressed() && !RShiftKey.is_pressed() {
                    enigo.text("ṛ").unwrap(); // send a when Caps Lock is off
                } else {
                    enigo.text("Ṛ").unwrap(); // send A when Caps Lock is on
                }
               
              
            }
        });
        Block
    } else {
        DontBlock
    }
});

let binding_active_clone = Arc::clone(&binding_active);
let enigo_clone = Arc::clone(&enigo);
EKey.blockable_bind(move || {
    if !LControlKey.is_pressed() && !RControlKey.is_pressed() && *binding_active_clone.lock().unwrap() {
        let binding_active = Arc::clone(&binding_active_clone);
        let enigo = Arc::clone(&enigo_clone);
        thread::spawn(move || {
            let mut binding_active = binding_active.lock().unwrap();
            let mut enigo = enigo.lock().unwrap();
            if *binding_active {
                *binding_active  =  false;
                dbg!("EKey pressed");
                if !CapsLockKey.is_toggled() && !LShiftKey.is_pressed() && !RShiftKey.is_pressed() {
                    enigo.text("ṝ").unwrap(); // send a when Caps Lock is off
                } else {
                    enigo.text("Ṝ").unwrap(); // send A when Caps Lock is on
                }
               
              
            }
        });
        Block
    } else {
        DontBlock
    }
});


let binding_active_clone = Arc::clone(&binding_active);
let enigo_clone = Arc::clone(&enigo);
SKey.blockable_bind(move || {
    if !LControlKey.is_pressed() && !RControlKey.is_pressed() && *binding_active_clone.lock().unwrap() {
        let binding_active = Arc::clone(&binding_active_clone);
        let enigo = Arc::clone(&enigo_clone);
        thread::spawn(move || {
            let mut binding_active = binding_active.lock().unwrap();
            let mut enigo = enigo.lock().unwrap();
            if *binding_active {
                *binding_active  =  false;
                dbg!("SKey pressed");
                if !CapsLockKey.is_toggled() && !LShiftKey.is_pressed() && !RShiftKey.is_pressed() {
                    enigo.text("ṣ").unwrap(); // send a when Caps Lock is off
                } else {
                    enigo.text("Ṣ").unwrap(); // send A when Caps Lock is on
                }
               
              
            }
        });
        Block
    } else {
        DontBlock
    }
});


let binding_active_clone = Arc::clone(&binding_active);
let enigo_clone = Arc::clone(&enigo);
ZKey.blockable_bind(move || {
    if !LControlKey.is_pressed() && !RControlKey.is_pressed() && *binding_active_clone.lock().unwrap() {
        let binding_active = Arc::clone(&binding_active_clone);
        let enigo = Arc::clone(&enigo_clone);
        thread::spawn(move || {
            let mut binding_active = binding_active.lock().unwrap();
            let mut enigo = enigo.lock().unwrap();
            if *binding_active {
                *binding_active  =  false;
                dbg!("ZKey pressed");
                if !CapsLockKey.is_toggled() && !LShiftKey.is_pressed() && !RShiftKey.is_pressed() {
                    enigo.text("ś").unwrap(); // send a when Caps Lock is off
                } else {
                    enigo.text("Ś").unwrap(); // send A when Caps Lock is on
                }
               
              
            }
        });
        Block
    } else {
        DontBlock
    }
});

let binding_active_clone = Arc::clone(&binding_active);
let enigo_clone = Arc::clone(&enigo);
TKey.blockable_bind(move || {
    if !LControlKey.is_pressed() && !RControlKey.is_pressed() && *binding_active_clone.lock().unwrap() {
        let binding_active = Arc::clone(&binding_active_clone);
        let enigo = Arc::clone(&enigo_clone);
        thread::spawn(move || {
            let mut binding_active = binding_active.lock().unwrap();
            let mut enigo = enigo.lock().unwrap();
            if *binding_active {
                *binding_active  =  false;
                dbg!("TKey pressed");
                if !CapsLockKey.is_toggled() && !LShiftKey.is_pressed() && !RShiftKey.is_pressed() {
                    enigo.text("ṭ").unwrap(); // send a when Caps Lock is off
                } else {
                    enigo.text("Ṭ").unwrap(); // send A when Caps Lock is on
                }
               
              
            }
        });
        Block
    } else {
        DontBlock
    }
});

let binding_active_clone = Arc::clone(&binding_active);
let enigo_clone = Arc::clone(&enigo);
UKey.blockable_bind(move || {
    if !LControlKey.is_pressed() && !RControlKey.is_pressed() && *binding_active_clone.lock().unwrap() {
        let binding_active = Arc::clone(&binding_active_clone);
        let enigo = Arc::clone(&enigo_clone);
        thread::spawn(move || {
            let mut binding_active = binding_active.lock().unwrap();
            let mut enigo = enigo.lock().unwrap();
            if *binding_active {
                *binding_active  =  false;
                dbg!("UKey pressed");
                if !CapsLockKey.is_toggled() && !LShiftKey.is_pressed() && !RShiftKey.is_pressed() {
                    enigo.text("ū").unwrap(); // send a when Caps Lock is off
                } else {
                    enigo.text("Ū").unwrap(); // send A when Caps Lock is on
                }
               
              
            }
        });
        Block
    } else {
        DontBlock
    }
});
              thread::spawn({
        let binding_active = Arc::clone(&binding_active);
        move || {
            thread::sleep(Duration::from_millis(1000));
            *binding_active.lock().unwrap() = false;
            AKey.unbind();
            DKey.unbind();
            HKey.unbind();
            IKey.unbind();
            LKey.unbind();
            MKey.unbind();
            NKey.unbind();
            GKey.unbind();
            JKey.unbind();
            RKey.unbind();
            EKey.unbind();
            SKey.unbind();
            ZKey.unbind();
            TKey.unbind();
            UKey.unbind();
        }
    });
    
            }
        });
    
        handle_input_events();
}




#[tauri::command]
async fn press() {
    dbg!("Apploaded");
   }







fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![press  ,bindevent])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
