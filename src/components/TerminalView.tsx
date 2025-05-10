// src/components/Terminal.tsx
import { onCleanup, onMount } from "solid-js";
import { Terminal } from "@xterm/xterm";
import "@xterm/xterm/css/xterm.css";

export const TerminalView = () => {
    let terminalContainer: HTMLDivElement | undefined;
    let term: Terminal;

    const prompt = () => {
        term.write("\r\n");
    };

    onMount(() => {
        if (!terminalContainer) {
            return;
        }
        term = new Terminal({
            theme: {
                background: "rgba(0,0,0,0)",
            },
        });
        term.open(terminalContainer);
        term.writeln("Hello from xterm.js in SolidJS + Tauri!");

        // Simulate input/output
        prompt();

        term.onData((data) => {
            switch (data) {
                case "\r": // Enter
                    // term.write("\r\n");
                    prompt();
                    break;
                case "\u007F": // Backspace
                    term.write("\b \b");
                    break;
                default:
                    term.write(data);
            }
        });
    });

    onCleanup(() => {
        term?.dispose();
    });

    return <div ref={terminalContainer} />;
};
