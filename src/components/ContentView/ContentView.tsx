import type { D_ContentViewProps, D_DocumentFile } from "@schemas/index";
import { type Accessor, For, Switch, Match } from "solid-js";
import { Editor } from "./components";

export const ContentView = ({
    props,
}: {
    props: Accessor<D_ContentViewProps>;
}) => {
    const documents = () => props().documents;

    return (
        <div
            id="middle-panel"
            class="relative flex-1 bg-[#222] rounded-md mb-4 overflow-hidden"
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
