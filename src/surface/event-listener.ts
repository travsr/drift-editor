import { listen } from "@tauri-apps/api/event";

import { setWindowState } from "@state/store";
import type { DWindowState } from "@schemas/shared_types";

const unlistenPromise = listen<DWindowState>("backend-event", (event) => {
    setWindowState(event.payload);
});
