import { onMount } from "solid-js";
import type { D_DocumentFile } from "@schemas/index";
import * as monaco from "monaco-editor";

interface EditorProps {
    file: D_DocumentFile;
}

export const Editor = (props: EditorProps) => {
    onMount(() => {
        const editorDiv = document.getElementById("my-editor");
        if (editorDiv) {
            monaco.editor.create(editorDiv, {
                value: props.file.buffer,
                language: "javascript",
                theme: "vs-dark",
                automaticLayout: true,
                minimap: {
                    enabled: false,
                },
                cursorStyle: "block",
                scrollbar: {},
            });
        }
    });
    return <div id="my-editor" class="absolute inset-0" />;
};
