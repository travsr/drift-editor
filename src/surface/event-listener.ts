import { type EventCallback, listen } from "@tauri-apps/api/event";

import { onCleanup, onMount } from "solid-js";

import { setIsWindowHydrated, setWindowState } from "@state/store";
import {
    type DWindowEventPayloadAll,
    DWindowEventScope,
    type DWindowEventPayload,
    type DWindowEventPayloadTabs,
    type DWindowEventPayloadContent,
} from "@schemas/shared_types";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";

const windowEventHandler: EventCallback<DWindowEventPayload> = (event) => {
    console.log("[windowStateUpdate] Received event", event);
    setIsWindowHydrated(true);

    switch (event.payload.scope) {
        case DWindowEventScope.All: {
            const payload = event.payload as DWindowEventPayloadAll;
            setWindowState(payload.window_state);
            break;
        }
        case DWindowEventScope.Content: {
            const payload = event.payload as DWindowEventPayloadContent;
            // To implement
            setWindowState("content", payload.content);
            break;
        }
        case DWindowEventScope.FileList: {
            // To implement
            break;
        }
        case DWindowEventScope.Tabs: {
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

        listen<DWindowEventPayload>("window_state_update", windowEventHandler, {
            target: window.label,
        }).then((unlisten) => {
            invoke("tc_window_ready");
            cleanup = unlisten;
        });
    });

    onCleanup(() => {
        cleanup?.();
    });
};
