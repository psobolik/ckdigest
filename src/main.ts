import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";

function prompt_display_el(): HTMLParagraphElement {
  return document.querySelector("#prompt-display") as HTMLParagraphElement;
}

function compare_input_el(): HTMLInputElement {
  return document.querySelector("#compare-input") as HTMLInputElement;
}

function digest_file_display_el(): HTMLElement {
  return document.querySelector("#digest-file-display") as HTMLElement;
}

function digest_display_el(): HTMLElement {
  return document.querySelector("#digest-display") as HTMLElement;
}

function clear_button_el(): HTMLButtonElement {
  return document.querySelector("#clear-button") as HTMLButtonElement;
}

function generate_button_el(): HTMLButtonElement {
  return document.querySelector("#generate-button") as HTMLButtonElement;
}

function compare_button_el(): HTMLButtonElement {
  return document.querySelector("#compare-button") as HTMLButtonElement;
}

function digest_select_el(): HTMLSelectElement {
  return document.querySelector("#digest-select") as HTMLSelectElement;
}

function selected_digest_function(): string | null {
  let el = digest_select_el();
  return el ? el.value : null;
}

async function generate_digest() {
  await generate_digest_for_file(await invoke("pick_file"));
}

async function generate_digest_for_file(file: string) {
  digest_file_display_el().textContent = file;
  digest_display_el().textContent = "";
  if (file) {
    let digest_function = selected_digest_function();
    if (digest_function) {
      let please_wait = document.querySelector("#please-wait");
      please_wait?.classList.remove('hidden');
      digest_display_el().textContent = await invoke("generate_digest", {
        pathBuf: file,
        hashFunction: digest_function,
      });
      please_wait?.classList.add('hidden');
      if (compare_input_el()?.value) {
        compare_digests();
      }
    }
  }
}

function compare_digests() {
  let compare_input = compare_input_el();
  let digest_display = digest_display_el();
  let prompt_display = prompt_display_el();

  let msg = "";
  let has_digest = digest_display.innerText.length != 0;
  let has_compare = compare_input.value.length != 0;
  if (has_digest && has_compare) {
    prompt_display.classList.add("bold");
    prompt_display.innerHTML = digest_display.innerText.toLowerCase() === compare_input.value.toLowerCase()
      ? "Digests match" : "Digests <i>do not</i> match";
  } else {
    if (has_compare && !has_digest) {
      msg = "Select a file and try again"
    } else if (!has_compare && has_digest) {
      msg = "Enter the expected digest for the file and try again"
    } else msg = "Select a file, enter its expected digest and try again"
    prompt_display.classList.remove("bold");
    prompt_display_el().textContent = msg;
  }
}

window.addEventListener("DOMContentLoaded", () => {
  generate_button_el().addEventListener("click", event => {
    event.preventDefault();
    prompt_display_el().textContent = "";
    generate_digest().catch(console.error);
  });

  digest_select_el().addEventListener("change", () => {
    generate_digest_for_file(digest_file_display_el().textContent).catch(console.error);
  })

  compare_button_el().addEventListener("click", event => {
    event.preventDefault();
    compare_digests();
  });

  compare_input_el().addEventListener("change", event => {
    event.preventDefault();
    prompt_display_el().textContent = "";
  });

  clear_button_el().addEventListener("click", event => {
    event.preventDefault();
    digest_file_display_el().textContent = "";
    digest_display_el().textContent = "";
    prompt_display_el().textContent = "";
    compare_input_el().value = "";
  })
  document.getElementById('title-bar-minimize')?.addEventListener('click', () => appWindow.minimize())
  document.getElementById('title-bar-maximize')?.addEventListener('click', () => appWindow.toggleMaximize())
  document.getElementById('title-bar-close')?.addEventListener('click', () => appWindow.close())});

