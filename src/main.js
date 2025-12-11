const { invoke } = window.__TAURI__.core;

let greetMsgEl;

async function greet() {
  greetMsgEl = document.querySelector("#greet-msg");
  greetMsgEl.textContent = await invoke("run_file_list_command");
}

window.addEventListener("DOMContentLoaded", () => {
  document.querySelector("#greet-button").addEventListener("click", greet);
});
