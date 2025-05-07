import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { SidebarRight } from "./Icons";

import type { D_UIState } from "@schemas/index";
import { TabView, ContentView } from "@components/index";

const [uiState, setUIState] = createSignal({
    tabView: {
        tabs: [
            {
                title: "OpenFile.tsx",
                documentRefs: [
                    {
                        type: "file",
                        title: "Text Document.txt",
                        id: "document-1",
                    },
                ],
            },
        ],
    },
    contentView: {
        documents: [
            {
                id: "document-1",
                title: "Test Document",
                type: "file",
                status: "new",
                buffer: "Hello world.",
                filePath: "/",
            },
        ],
    },
    detailView: {
        type: "explorer",
    },
} satisfies D_UIState);

const tabView = () => uiState().tabView;
const contentView = () => uiState().contentView;
const detailView = () => uiState().detailView;

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
                <TabView props={tabView} />
                <ContentView props={contentView} />
                <div class="w-50 bg-[#222] rounded-md">
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
