import { For, Switch, Match } from "solid-js";
import { CodeMirrorEditor } from "./components/index";
import type { DWindowState } from "@schemas/index";

type ContentViewProps = {
    content: DWindowState["content"];
};

export const ContentView = (props: ContentViewProps) => {
    const contentItems = () => props.content.content_items;

    return (
        <div
            id="content-view"
            class="relative flex-1 bg-[#222]/90 rounded-md overflow-hidden"
        >
            <For each={contentItems()}>
                {(contentItem) => (
                    <Switch>
                        <Match when={contentItem.type === "file"}>
                            <CodeMirrorEditor />
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
    );
};
