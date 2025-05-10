import type { D_UIState } from "@schemas/D_AppState";
import { createStore } from "solid-js/store";

export const [appState, setAppState] = createStore<D_UIState>({
    tabs: [
        {
            id: "tab-1",
            title: "OpenFile.tsx",
            isSelected: true,
            documentRefs: [
                {
                    id: "document-1",
                    type: "file",
                    title: "Text Document.txt",
                },
            ],
        },
        {
            id: "tab-2",
            title: "file2.tsx",
            isSelected: false,
            documentRefs: [
                {
                    id: "document-1",
                    type: "file",
                    title: "Text Document.txt",
                },
            ],
        },
    ],
    content: {
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
    fileTree: {
        path: "/",
        type: "folder",
        name: "root",
        isExpanded: true,
        children: [
            {
                path: "/utils",
                type: "file",
                name: "utils",
            },
            {
                path: "/utils",
                type: "file",
                name: "schemas",
            },
        ],
    },
    ui: {
        isOverlayActive: false,
        sidebar: "tree",
    },
});
