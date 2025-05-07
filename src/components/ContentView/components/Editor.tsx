import { onMount } from "solid-js";
import * as monaco from "monaco-editor";
import type { D_DocumentFile } from "@schemas/index";

// self.MonacoEnvironment = {
//     getWorker: (workerId, label) => {
//         // @ts-ignore
//         const getWorkerModule = (moduleUrl, label) => {
//             // @ts-ignore
//             return new Worker(self.MonacoEnvironment.getWorkerUrl(moduleUrl), {
//                 name: label,
//                 type: "module",
//             });
//         };

//         switch (label) {
//             case "json":
//                 return getWorkerModule(
//                     "/monaco-editor/esm/vs/language/json/json.worker?worker",
//                     label,
//                 );
//             case "css":
//             case "scss":
//             case "less":
//                 return getWorkerModule(
//                     "/monaco-editor/esm/vs/language/css/css.worker?worker",
//                     label,
//                 );
//             case "html":
//             case "handlebars":
//             case "razor":
//                 return getWorkerModule(
//                     "/monaco-editor/esm/vs/language/html/html.worker?worker",
//                     label,
//                 );
//             case "typescript":
//             case "javascript":
//                 return getWorkerModule(
//                     "/monaco-editor/esm/vs/language/typescript/ts.worker?worker",
//                     label,
//                 );
//             default:
//                 return getWorkerModule(
//                     "/monaco-editor/esm/vs/editor/editor.worker?worker",
//                     label,
//                 );
//         }
//     },
// };

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
