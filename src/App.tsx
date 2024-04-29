import {  useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { register , unregisterAll } from "@tauri-apps/api/globalShortcut";

//import { active } from "./conetext";


function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  //const { setActive } = useContext(active);
  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }



useEffect(() => {

  async function registerKeyboardShortcuts() : Promise<void>{
    try {
      // await register("Shift+a", async () => {
      //   console.log('F12 pressed');
      
      // });
  
      // await register("a", async () => {
      //   console.log('F13 pressed');
     
      // });
      
      await register("Shift+x", async () => {
      
        console.log('CmdOrControl pressed');
       
        console.log('Deactivated');

        await register("Shift+a", async () => {
          console.log('F12 pressed');
          
        
        });
  
        await register("a", async () => {
          console.log('F13 pressed');
          await invoke('press',{name : "CmdOrControl pressed"});
        });
      
      });
  
    } catch (error) {
      // handle error
      console.error(error);
    }

    await register("CommandOrControl+x", async () => {
      await unregisterAll();
      await register("Shift+x", async () => {
      
        console.log('CmdOrControl pressed');
        
        console.log('Activated');

        await register("Shift+a", async () => {
          console.log('F12 pressed');
          
        
        });
  
        await register("a", async () => {
          console.log('F13 pressed');
          await invoke('press',{name : "CmdOrControl pressed"});
        });
       
      });
   
    });
  }
  registerKeyboardShortcuts().then(async() => {
    await invoke("greet", { name : "User" });
  }).catch((error) => {
    console.error(error);
  });
}, []);





    
  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>

      <p>{greetMsg}</p>
    </div>
  );
}

export default App;
