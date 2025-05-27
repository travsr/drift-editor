import { For, Switch, Match, createMemo } from "solid-js";
import { CodeMirrorEditor } from "./components/index";
import type { DWindowState } from "@schemas/index";
import { windowState } from "@state/store";

type ContentViewProps = {
    content: DWindowState["content"];
};

export const ContentView = (props: ContentViewProps) => {
    const contentItems = createMemo(() => windowState.content.content_items);

    return (
        <div
            id="content-view"
            class="relative flex-1 bg-[#222]/90 rounded-md overflow-hidden"
        >
            <div class="absolute inset-0 overflow-scroll">
                <For each={contentItems()}>
                    {(contentItem) => (
                        <Switch>
                            <Match when={contentItem.type === "file"}>
                                {/* <CodeMirrorEditor /> */}
                                <pre class="text-white">
                                    {contentItem.document?.buffer ||
                                        "Not found"}
                                </pre>
                            </Match>
                            <Match when={contentItem.type === "terminal"}>
                                <div>💻 Terminal</div>
                            </Match>
                            <Match when={contentItem.type === "settings"}>
                                <div>⚙️ Settings</div>
                            </Match>
                        </Switch>
                    )}
                </For>
            </div>
        </div>
    );
};
