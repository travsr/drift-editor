import {
    createSignal,
    Match,
    onCleanup,
    onMount,
    Show,
    Switch,
} from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import classNames from "classnames";

import "./App.css";

import { TabView, ContentView, FileTree } from "@components/index";
import { isWindowHydrated, windowState } from "@state/store";
import { Folder, Server } from "./assets";
import { startEventListener } from "./surface";

function App() {
    const [greetMsg, setGreetMsg] = createSignal("");
    const [name, setName] = createSignal("");
    const [isOverlayActive, setIsOverlayActive] = createSignal(false);

    async function greet() {
        // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
        setGreetMsg(await invoke("greet", { name: name() }));
    }

    const [lastKey, setLastKey] = createSignal<string | null>(null);

    startEventListener();

    onMount(() => {
        // Tauri window keyboard capture (fallback for full native apps)
        const handleKey = (e: KeyboardEvent) => {
            console.log(
                "Key Pressed:",
                e.key,
                e.ctrlKey ? "Ctrl" : "",
                e.metaKey ? "Meta" : "",
            );

            // Handle Ctrl+P or Cmd+P (mac)
            if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "p") {
                e.preventDefault();
                setIsOverlayActive((value) => !value);
            }

            // ESC to clear
            if (e.key === "Escape") {
                setIsOverlayActive(false);
            }
        };

        window.addEventListener("keydown", handleKey);

        onCleanup(() => {
            window.removeEventListener("keydown", handleKey);
        });
    });

    return (
        <Show when={isWindowHydrated()}>
            {/* <div id="titlebar-drag-region" data-tauri-drag-region /> */}
            <main
                class={classNames(
                    "absolute inset-0 z-10 flex flex-row gap-2 p-2",
                    "transition-all duration-200",
                    { blur: isOverlayActive() },
                )}
            >
                <div class="w-40 flex flex-col gap-4 pt-8">
                    <div class="flex items-center justify-center gap-2 text-white/30">
                        <Folder />
                        <Server />
                    </div>
                    <Switch>
                        <Match when={windowState.ui.sidebar === "tabs"}>
                            <TabView tabs={windowState.tabs} />
                        </Match>
                        <Match when={windowState.ui.sidebar === "tree"}>
                            <FileTree node={windowState.file_tree} />
                        </Match>
                    </Switch>
                </div>
                {/* <DetailView props={detailView} /> */}
                <div class="flex-1 flex flex-col gap-2">
                    <ContentView content={windowState.content} />
                </div>
            </main>

            <Show when={isOverlayActive()}>
                <div class="absolute inset-0 z-100">
                    <div class="absolute right-8 top-8 w-60 bottom-8 bg-[#333] rounded z-100" />
                </div>
            </Show>
        </Show>
    );
}

export default App;
