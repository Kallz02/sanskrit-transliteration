// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use enigo::{
    Enigo, Keyboard, Settings,
};

//use tauri::{Manager, Window};
// Create the command:
// This command must be async so that it doesn't run on the main thread.
// ,
      
//     {
//        "width": 400,
//       "height": 200,
//       "decorations": true,
//       "url": "splash.html",
//       "label": "loading"
//    }
// #[tauri::command]
// async fn close_splashscreen(window: Window) {
//   // Close splashscreen
//   window.get_window("loading").expect("no window labeled 'loading' found").close().unwrap();
//   // Show main window
//   window.get_window("main").expect("no window labeled 'main' found").show().unwrap();
// }
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    dbg!(name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}




#[tauri::command]
async fn press() {
    dbg!("Apploaded");
   
    
}



#[tauri::command]
async fn press_a() {
    dbg!("Pressed Ā");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.text("Ā").unwrap();
    
}

#[tauri::command]
async fn pressa() {
    dbg!("Pressed ā");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.text("ā").unwrap();
    
}

#[tauri::command]
async fn pressd() {
    dbg!("ḍ");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.text("ḍ").unwrap();
    
}

#[tauri::command]
async fn press_d() {
    dbg!("Ḍ");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.text("Ḍ").unwrap();
    
}

#[tauri::command]
async fn pressh() {
    dbg!("ḥ");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.text("ḥ").unwrap();
    
}

#[tauri::command]
async fn press_h() {
    dbg!("Ḥ");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.text("Ḥ").unwrap();
    
}

#[tauri::command]
async fn pressi() {
    dbg!("ī");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.text("ī").unwrap();
    
}

#[tauri::command]
async fn press_i() {
    dbg!("Ī");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.text("Ī").unwrap();
    
}

#[tauri::command]
async fn pressl() {
    dbg!("ḷ");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.text("ḷ").unwrap();
    
}

#[tauri::command]
async fn press_l() {
    dbg!("Ḷ");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.text("Ḷ").unwrap();
    
}
#[tauri::command]
async fn press_m() {
    dbg!("Ṁ");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.text("Ṁ").unwrap();
    
}

#[tauri::command]
async fn pressm() {
    dbg!("ṁ");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.text("ṁ").unwrap();
    
}

#[tauri::command]
async fn press_n() {
    dbg!("Ṇ");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.text("Ṇ").unwrap();
    
}
#[tauri::command]
async fn pressn() {
    dbg!("ṇ");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.text("ṇ").unwrap();
    
}

#[tauri::command]
async fn press_g() {
    dbg!("Ṅ");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.text("Ṅ").unwrap();
    
}

#[tauri::command]
async fn pressg() {
    dbg!("ṅ");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.text("ṅ").unwrap();
    
}

#[tauri::command]
async fn press_j() {
    dbg!("Ñ");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.text("Ñ").unwrap();
    
}

#[tauri::command]
async fn pressj() {
    dbg!("ñ");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.text("ñ").unwrap();
    
}

#[tauri::command]
async fn press_r() {
    dbg!("Ṙ");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.text("Ṙ").unwrap();
    
}

#[tauri::command]
async fn pressr() {
    dbg!("ṙ");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.text("ṙ").unwrap();
    
}


#[tauri::command]
async fn press_e() {
    dbg!("Ṝ");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.text("Ṝ").unwrap();
    
}

#[tauri::command]
async fn presse() {
    dbg!("ṝ");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.text("ṝ").unwrap();
    
}

#[tauri::command]
async fn press_s() {
    dbg!("Ṣ");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.text("Ṣ").unwrap();
    
}

#[tauri::command]
async fn presss() {
    dbg!("ṣ");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.text("ṣ").unwrap();
    
}

#[tauri::command]
async fn press_z() {
    dbg!("Ś");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.text("Ś").unwrap();
    
}

#[tauri::command]
async fn pressz() {
    dbg!("ś");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.text("ś").unwrap();
    
}

#[tauri::command]
async fn press_t() {
    dbg!("Ṭ");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.text("Ṭ").unwrap();
    
}

#[tauri::command]
async fn presst() {
    dbg!("ṭ");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.text("ṭ").unwrap();
    
}

#[tauri::command]
async fn press_u() {
    dbg!("Ū");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.text("Ū").unwrap();
    
}

#[tauri::command]
async fn pressu() {
    dbg!("ū");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.text("ū").unwrap();
    
}






fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,press  , pressa ,  press_a , pressd , press_d , pressh , press_h, pressi , press_i , pressj , press_j ,pressl , press_l, pressm , press_m , pressn , press_n  , pressr , press_r , presss , press_s , presst , press_t , pressu , press_u , press_z , pressz , presse , press_e , pressg , press_g])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
