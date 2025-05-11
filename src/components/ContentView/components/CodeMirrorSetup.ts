// codemirror-setup.ts

import { EditorState } from "@codemirror/state";
import { type ViewUpdate, EditorView } from "@codemirror/view";
import { openSearchPanel, highlightSelectionMatches } from "@codemirror/search";
import {
    indentWithTab,
    history,
    defaultKeymap,
    historyKeymap,
} from "@codemirror/commands";
import {
    foldGutter,
    indentOnInput,
    indentUnit,
    bracketMatching,
    foldKeymap,
    syntaxHighlighting,
    defaultHighlightStyle,
} from "@codemirror/language";
import {
    closeBrackets,
    autocompletion,
    closeBracketsKeymap,
    completionKeymap,
} from "@codemirror/autocomplete";
import {
    lineNumbers,
    highlightActiveLineGutter,
    highlightSpecialChars,
    drawSelection,
    dropCursor,
    rectangularSelection,
    crosshairCursor,
    highlightActiveLine,
    keymap,
} from "@codemirror/view";
import { oneDark } from "@codemirror/theme-one-dark";
import { javascript } from "@codemirror/lang-javascript";

export function createEditorState(
    initialContents: string,
    options: {
        oneDark?: boolean;
        onUpdate?: (update: ViewUpdate) => void;
    } = {},
) {
    const extensions = [
        lineNumbers(),
        highlightActiveLineGutter(),
        highlightSpecialChars(),
        history(),
        foldGutter(),
        drawSelection(),
        dropCursor(),
        indentUnit.of("    "),
        EditorState.allowMultipleSelections.of(true),
        indentOnInput(),
        bracketMatching(),
        closeBrackets(),
        autocompletion(),
        rectangularSelection(),
        crosshairCursor(),
        highlightActiveLine(),
        highlightSelectionMatches(),
        keymap.of([
            indentWithTab,
            ...closeBracketsKeymap,
            ...defaultKeymap,
            ...historyKeymap,
            ...foldKeymap,
            ...completionKeymap,
        ]),
        javascript(),
        syntaxHighlighting(defaultHighlightStyle, { fallback: true }),
    ];

    if (options.oneDark) {
        extensions.push(oneDark);
    }

    if (options.onUpdate) {
        extensions.push(
            EditorView.updateListener.of((update) => {
                options.onUpdate?.(update);
            }),
        );
    }

    return EditorState.create({
        doc: initialContents,
        extensions,
    });
}

export function createEditorView(state: EditorState, parent: HTMLElement) {
    return new EditorView({ state, parent });
}

export { openSearchPanel };
