import { listen } from "@tauri-apps/api/event";

import { onCleanup, onMount } from "solid-js";

import { setWindowState } from "@state/store";
import type { DWindowState } from "@schemas/shared_types";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";

export const startEventListener = () => {
    let cleanup: () => void;

    onMount(() => {
        console.log("[event-listener] mounted app.");
        const window = getCurrentWindow();

        listen<DWindowState>(
            "window_state_update",
            (event) => {
                console.log("[event-listener] received event", event);
                setWindowState(event.payload);
            },
            { target: window.label },
        ).then((unlisten) => {
            invoke("tc_window_ready");
            cleanup = unlisten;
        });
    });

    onCleanup(() => {
        cleanup?.();
    });
};
