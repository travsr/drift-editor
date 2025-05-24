import {
    DFileTreeNodeType,
    DSidebarType,
    type DWindowState,
} from "@schemas/shared_types";
import { createSignal } from "solid-js";
import { createStore } from "solid-js/store";

const [windowState, _setWindowState] = createStore<DWindowState>({
    id: "unhydrated",
    tabs: [],
    content: {
        documents: [],
    },
    file_path: "~/",
    file_tree: {
        path: "/",
        type: DFileTreeNodeType.Folder,
        name: "root",
        is_expanded: true,
        children: [],
    },
    ui: {
        is_overlay_active: false,
        sidebar: DSidebarType.Tabs,
    },
});

const [isWindowHydrated, setIsWindowHydrated] = createSignal(false);

const setWindowState = (state: DWindowState) => {
    _setWindowState(state);
    setIsWindowHydrated(true);
    console.log(isWindowHydrated());
};

export { isWindowHydrated, windowState, setWindowState };
