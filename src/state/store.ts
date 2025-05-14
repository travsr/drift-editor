import {
    DDocumentStatus,
    DDocumentType,
    DFileTreeNodeType,
    DSidebarType,
    type DWindowState,
} from "@schemas/shared_types";
import { createStore } from "solid-js/store";

export const [windowState, setWindowState] = createStore<DWindowState>({
    id: "window-1",
    tabs: [
        {
            id: "tab-1",
            title: "OpenFile.tsx",
            is_selected: true,
            document_refs: [
                {
                    id: "document-1",
                    type: DDocumentType.File,
                    title: "Text Document.txt",
                },
            ],
        },
        {
            id: "tab-2",
            title: "file2.tsx",
            is_selected: false,
            document_refs: [
                {
                    id: "document-1",
                    type: DDocumentType.File,
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
                type: DDocumentType.File,
                status: DDocumentStatus.New,
                buffer: "Hello world.",
                file_path: "/",
            },
        ],
    },
    file_tree: {
        path: "/",
        type: DFileTreeNodeType.Folder,
        name: "root",
        is_expanded: true,
        children: [
            {
                path: "/utils",
                type: DFileTreeNodeType.File,
                name: "utils",
            },
            {
                path: "/utils",
                type: DFileTreeNodeType.File,
                name: "schemas",
            },
        ],
    },
    ui: {
        is_overlay_active: false,
        sidebar: DSidebarType.Tabs,
    },
});
