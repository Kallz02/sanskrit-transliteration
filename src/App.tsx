import {  useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
//import { register , unregisterAll } from "@tauri-apps/api/globalShortcut";




function App() {
 



useEffect(() => {



  invoke('bindevent').then(async () => {
    await invoke('press')
  }).catch((error) => {
    console.error(error);
  });
  

}, []);





    
  return (
    <div>
     <div>Click and Release Ctrl key (+) Type Key</div>
    
     <table>
      <tr>
        <td>a 🠦 ā</td> 
        <td>A 🠦 Ā</td>
        <td>j 🠦 ñ</td> 
        <td>J 🠦 Ñ</td>  
      </tr>
      <tr>
        <td>d 🠦 ḍ</td> 
        <td>D 🠦 Ḍ</td>
        <td>r 🠦 ṛ</td> 
        <td>R 🠦 Ṛ</td>  
    
      </tr>
      <tr>
        <td>h 🠦 ḥ</td> 
        <td>H 🠦 Ḥ</td>
        <td>e 🠦 ṝ</td> 
        <td>E 🠦 Ṝ</td>  
      </tr>
      <tr>
        <td>i 🠦 ī</td> 
        <td>I 🠦 Ī</td>
        <td>s 🠦 ṣ</td> 
        <td>S 🠦 Ṣ</td>  
      </tr>
      <tr>
        <td>l 🠦 ḷ</td> 
        <td>L 🠦 Ḷ</td>
        <td>z 🠦 ś</td> 
        <td>Z 🠦 Ś</td>  
      </tr>
      <tr>
        <td>m 🠦 ṁ</td> 
        <td>M 🠦 Ṁ</td>
        <td>t 🠦 ṭ</td> 
        <td>T 🠦 Ṭ</td>  
      </tr>
      <tr>
        <td>n 🠦 ṇ</td> 
        <td>N 🠦 Ṇ</td>
        <td>u 🠦 ū</td> 
        <td>U 🠦 Ū</td>  
      </tr>
      <tr>
        <td>g 🠦 ṅ</td> 
        <td>G 🠦 Ṅ</td>
         
      </tr>
   
     </table>
    <div  className="cont">
     <div className="">
      <a href="mailto:akshay.pranav.kalathil@gmail.com">Contact</a> for additional characters
     </div>
     <div className="">
    Code: <a target="_blank" href="https://github.com/Kallz02/sanskrit-transliteration">Github</a>

     </div>
     </div>


    </div>
  );
}

export default App;
