import { createSignal } from "solid-js";
import logo from "./assets/logo.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { WindowControls } from "./WindowControls";
import { SidebarLeft, SidebarRight } from "./Icons";
import { Editor } from "./Editor";

function App() {
	const [greetMsg, setGreetMsg] = createSignal("");
	const [name, setName] = createSignal("");

	async function greet() {
		// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
		setGreetMsg(await invoke("greet", { name: name() }));
	}

	return (
		<>
			<div id="titlebar-drag-region" data-tauri-drag-region />
			<main class="flex flex-row gap-2 h-full p-2">
				<div class="w-30">
					<div class="flex items-center gap-4 pl-1">
						<WindowControls />
						<div class="opacity-50">
							<SidebarLeft />
						</div>
					</div>

					<div class="flex flex-col gap-2 mt-2">
						<div class="rounded-2xl bg-[#ffffff33] h-9" />
						<div class="rounded-2xl bg-[#ffffff11] h-9" />
						<div class="rounded-2xl bg-[#ffffff11] h-9" />
					</div>
				</div>
				<div id="middle-panel" class="relative flex-1 bg-[#222] rounded-md mb-4 overflow-hidden">
					<Editor />
				</div>
				<div class="w-50">
					<div class="flex justify-end">
						<div class="opacity-50">
							<SidebarRight />
						</div>
					</div>
				</div>
			</main>
		</>
	);
}

export default App;
