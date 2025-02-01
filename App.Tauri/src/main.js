const { invoke } = window.__TAURI__.core;

let upInputEl;
let downInputEl;
let yaoInputEl;
let resultEl;
let guaciEl;
let gua_unicodeEl;
let gua_nameEl;
let gua_orderEl;
let yaociEl;
let ch_guaciEl;
let ch_gua_unicodeEl;
let ch_gua_nameEl;
let ch_gua_orderEl;

async function show_gua() {
  let res = await invoke("get_guaci", {
    up: upInputEl.value,
    down: downInputEl.value,
  });
  const guares = JSON.parse(res);
  guaciEl.innerHTML = guares.html;
  gua_unicodeEl.innerHTML = guares.unicode;
  gua_nameEl.innerHTML = guares.name;
  gua_orderEl.innerHTML = guares.order;

  res = await invoke("get_yaoci", {
    up: upInputEl.value,
    down: downInputEl.value,
    yao: yaoInputEl.value,
  });
  const yaores = JSON.parse(res);
  yaociEl.innerHTML = yaores.html;

  res = await invoke("get_guaci_alt", {
    up: upInputEl.value,
    down: downInputEl.value,
    yao: yaoInputEl.value,
  });
  const ch_guares = JSON.parse(res);
  ch_guaciEl.innerHTML = ch_guares.html;
  ch_gua_unicodeEl.innerHTML = ch_guares.unicode;
  ch_gua_nameEl.innerHTML = ch_guares.name;
  ch_gua_orderEl.innerHTML = ch_guares.order;

  resultEl.style.display = "block";
}

window.addEventListener("DOMContentLoaded", () => {
  upInputEl = document.querySelector("#up-input");
  downInputEl = document.querySelector("#down-input");
  yaoInputEl = document.querySelector("#yao-input");
  resultEl = document.querySelector("#result");

  guaciEl = document.querySelector("#hexagram_guaci");
  gua_unicodeEl = document.querySelector("#unicode");
  gua_nameEl = document.querySelector("#name");
  gua_orderEl = document.querySelector("#order");

  yaociEl = document.querySelector("#hexagram_yaoci");

  ch_guaciEl = document.querySelector("#c_hexagram_guaci");
  ch_gua_unicodeEl = document.querySelector("#c_unicode");
  ch_gua_nameEl = document.querySelector("#c_name");
  ch_gua_orderEl = document.querySelector("#c_order");

  document.querySelector("#gua-form").addEventListener("submit", (e) => {
    e.preventDefault();
    show_gua();
  });
});
