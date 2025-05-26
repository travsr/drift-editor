import { type EventCallback, listen } from "@tauri-apps/api/event";

import { onCleanup, onMount } from "solid-js";

import { setIsWindowHydrated, setWindowState } from "@state/store";
import {
    type DWindowEventPayloadAll,
    DWindowStateScope,
    type DWindowEventPayload,
    type DWindowEventPayloadTabs,
} from "@schemas/shared_types";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";

const windowStateUpdateHandler: EventCallback<DWindowEventPayload> = (
    event,
) => {
    // console.log("[windowStateUpdate] Received event", event);
    setIsWindowHydrated(true);

    switch (event.payload.scope) {
        case DWindowStateScope.All: {
            const payload = event.payload as DWindowEventPayloadAll;
            setWindowState(payload.window_state);
            break;
        }
        case DWindowStateScope.Content: {
            // To implement
            break;
        }
        case DWindowStateScope.FileList: {
            // To implement
            break;
        }
        case DWindowStateScope.Tabs: {
            const payload = event.payload as DWindowEventPayloadTabs;
            setWindowState("tabs", payload.tabs);
            break;
        }
    }
};

export const startEventListener = () => {
    let cleanup: () => void;

    onMount(() => {
        console.log("[event-listener] mounted app.");
        const window = getCurrentWindow();

        listen<DWindowEventPayload>(
            "window_state_update",
            windowStateUpdateHandler,
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
