import type { D_DocumentFile, D_UIState } from "@schemas/index";
import { For, Switch, Match } from "solid-js";
import { Editor } from "./components";

type ContentViewProps = {
    content: D_UIState["content"];
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
                            <Editor file={doc as D_DocumentFile} />
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
