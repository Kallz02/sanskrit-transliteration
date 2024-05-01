import {  useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { register , unregisterAll } from "@tauri-apps/api/globalShortcut";




function App() {
 



useEffect(() => {

  async function registerKeyboardShortcuts() : Promise<void>{
    try {
   
      document.addEventListener('DOMContentLoaded', () => {
        // This will wait for the window to load, but you could
        // run this function on whatever trigger you want
        
      })
      
      await register("Shift+x", async () => {
      
     
        await Promise.all([
          register("CommandOrControl+x", async () => {
            await unregisterAll();
            await registerKeyboardShortcuts();
        
          }),
          register("Shift+a", async () => {
            await invoke('press_a');
          }),
          register("a", async () => {
            await invoke('pressa');
          }),
          register("Shift+d", async () => {
            await invoke('press_d');
          }),
          register("d", async () => {
            await invoke('pressd');
          }),
          register("Shift+h", async () => {
            await invoke('press_h');
          }),
          register("h", async () => {
            await invoke('pressh');
          }),
          register("Shift+i", async () => {
            await invoke('press_i');
          }),
          register("i", async () => {
            await invoke('pressi');
          }),
          register("Shift+l", async () => {
            await invoke('press_l');
          }),
          register("l", async () => {
            await invoke('pressl');
          }),
          register("Shift+m", async () => {
            await invoke('press_m');
          }),
          register("m", async () => {
            await invoke('pressm');
          }),
          register("Shift+n", async () => {
            await invoke('press_n');
          }),
          register("n", async () => {
            await invoke('pressn');
          }),
          register("Shift+g", async () => {
            await invoke('press_g');
          }),
          register("g", async () => {
            await invoke('pressg');
          }),
          register("Shift+j", async () => {
            await invoke('press_j');
          }),
          register("j", async () => {
            await invoke('pressj');
          }),
          register("Shift+r", async () => {
            await invoke('press_r');
          }),
          register("r", async () => {
            await invoke('pressr');
          }),
          register("Shift+e", async () => {
            await invoke('press_e');
          }),
          register("e", async () => {
            await invoke('presse');
          }),
          register("Shift+s", async () => {
            await invoke('press_s');
          }),
          register("s", async () => {
            await invoke('presss');
          }),
          register("Shift+z", async () => {
            await invoke('press_z');
          }),
          register("z", async () => {
            await invoke('pressz');
          }),
          register("Shift+t", async () => {
            await invoke('press_t');
          }),
          register("t", async () => {
            await invoke('presst');
          }),
          register("Shift+u", async () => {
            await invoke('press_u');
          }),
          register("u", async () => {
            await invoke('pressu');
          }),
        ]).then(() => {
          console.log("All registrations complete");
         
        }).catch((error) => {
          console.error("Error registering shortcuts:", error);
        });
      
      });
  
    } catch (error) {
      // handle error
      console.error(error);
    }

  
  }
  registerKeyboardShortcuts().then(async() => {
    await invoke('press')
   
  }).catch((error) => {
    console.error(error);
  });
}, []);





    
  return (
    <div className="">
     <div className=" text-sm">Press Shift + X to Activate , Ctrl + X to Deactivate</div>
     <div className=" text-sm">Note: use Shift for capital Letter Not Capslock </div>
     <div className=" text-sm bg-blue-50 flex">
      {/* <div className=" txt">
      Eg: 
      </div> */}
    
      Eg: use Shift+ a key , not Capslock + a key </div>
     <table className=" border border-gray-400">
      <tr className="border border-gray-400" style={{border: "1px solid black"}}>
        <td>a 🠦 ā</td> 
        <td>A 🠦 Ā</td>
        <td>j 🠦 ñ</td> 
        <td>J 🠦 Ñ</td>  
      </tr>
      <tr className="border border-gray-400" style={{border: "1px solid black"}}>
        <td>d 🠦 ḍ</td> 
        <td>D 🠦 Ḍ</td>
        <td>r 🠦 ṙ</td> 
        <td>R 🠦 Ṙ</td>  
    
      </tr>
      <tr className="border border-gray-400" style={{border: "1px solid black"}}>
        <td>h 🠦 ḥ</td> 
        <td>H 🠦 Ḥ</td>
        <td>e 🠦 ṝ</td> 
        <td>E 🠦 Ṝ</td>  
      </tr>
      <tr className="border border-gray-400" style={{border: "1px solid black"}}>
        <td>i 🠦 ī</td> 
        <td>I 🠦 Ī</td>
        <td>s 🠦 ṣ</td> 
        <td>S 🠦 Ṣ</td>  
      </tr>
      <tr className="border border-gray-400" style={{border: "1px solid black"}}>
        <td>l 🠦 ḷ</td> 
        <td>L 🠦 Ḷ</td>
        <td>z 🠦 ś</td> 
        <td>Z 🠦 Ś</td>  
      </tr>
      <tr className="border border-gray-400" style={{border: "1px solid black"}}>
        <td>m 🠦 ṁ</td> 
        <td>M 🠦 Ṁ</td>
        <td>t 🠦 ṭ</td> 
        <td>T 🠦 Ṭ</td>  
      </tr>
      <tr className="border border-gray-400" style={{border: "1px solid black"}}>
        <td>n 🠦 ṇ</td> 
        <td>N 🠦 Ṇ</td>
        <td>u 🠦 ū</td> 
        <td>U 🠦 Ū</td>  
      </tr>
      <tr className="border border-gray-400" style={{border: "1px solid black"}}>
        <td>g 🠦 ṅ</td> 
        <td>G 🠦 Ṅ</td>
         
      </tr>
   
     </table>


    </div>
  );
}

export default App;
