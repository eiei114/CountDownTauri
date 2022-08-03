import React, {useState} from 'react';
import './App.css';
import {invoke} from "@tauri-apps/api";

function App() {

  const [result,setResult]=useState<number>(0); // カウントの定数を用意

  return (
      <div className="App">
        <h1>tauri-sample</h1>
        <button id='increment_btn' onClick={()=>{invoke("increment").then((res) => setResult(res as number));}}>増加</button>
        <button id='decrement_btn' onClick={()=>{invoke("decrement").then((res) => setResult(res as number));}}>減少</button>
        <div id='result'>{result}</div>
      </div>
  );
}

export default App;