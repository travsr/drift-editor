import { DSidebarType, type DWindowState } from "@schemas/shared_types";
import { createSignal } from "solid-js";
import { createStore } from "solid-js/store";

export const [windowState, setWindowState] = createStore<DWindowState>({
    id: "unhydrated",
    tabs: [],
    content: {
        documents: [],
    },
    file_path: "~/",
    file_list: [],
    ui: {
        is_overlay_active: false,
        sidebar: DSidebarType.Tabs,
    },
});

export const [isWindowHydrated, setIsWindowHydrated] = createSignal(false);
