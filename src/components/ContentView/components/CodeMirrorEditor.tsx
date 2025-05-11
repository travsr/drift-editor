import { onMount, onCleanup } from "solid-js";
import {
    createEditorState,
    createEditorView,
    openSearchPanel,
} from "./CodeMirrorSetup"; // see below
import type { EditorView, ViewUpdate } from "@codemirror/view";

interface CodeMirrorEditorProps {
    value?: string;
    onChange?: (value: string) => void;
}

export const CodeMirrorEditor = (props: CodeMirrorEditorProps) => {
    let editorRef!: HTMLDivElement;
    let view: EditorView;

    onMount(() => {
        const state = createEditorState(props.value ?? "", {
            oneDark: true,
            onUpdate: (update: ViewUpdate) => {
                console.log("editor update!", update);
            },
        });

        view = createEditorView(state, editorRef);
    });

    onCleanup(() => {
        view?.destroy();
    });

    return (
        <div
            ref={(el) => {
                editorRef = el;
            }}
            class="absolute inset-0 overflow-scroll overscroll-none"
        />
    );
};
