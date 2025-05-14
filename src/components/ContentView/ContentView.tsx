import { For, Switch, Match } from "solid-js";
import { CodeMirrorEditor } from "./components/index";
import type { DWindowState } from "@schemas/index";

type ContentViewProps = {
    content: DWindowState["content"];
};

export const ContentView = (props: ContentViewProps) => {
    const documents = () => props.content.documents;

    return (
        <div
            id="content-view"
            class="relative flex-1 bg-[#222] rounded-md overflow-hidden"
        >
            <For each={documents()}>
                {(doc) => (
                    <Switch>
                        <Match when={doc.type === "file"}>
                            <CodeMirrorEditor />
                        </Match>
                        <Match when={doc.type === "terminal"}>
                            <div>💻 Terminal: {doc.title}</div>
                        </Match>
                        <Match when={doc.type === "settings"}>
                            <div>⚙️ Settings: {doc.title}</div>
                        </Match>
                    </Switch>
                )}
            </For>
        </div>
    );
};
