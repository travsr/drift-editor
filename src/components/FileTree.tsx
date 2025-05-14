import { For, Show } from "solid-js";
import type { DFileTreeNode } from "@schemas/index";

type FileTreeProps = {
    node: DFileTreeNode;
    level?: number;
};

export const FileTree = (props: FileTreeProps) => {
    const isFolder = props.node.type === "folder";
    const indent = `${(props.level ?? 0) * 1.25}rem`;

    return (
        <div>
            <div
                style={{ "padding-left": indent }}
                class="cursor-pointer select-none hover:bg-white/30 rounded text-white"
                onClick={() => {}}
                onKeyDown={() => {}}
                onKeyUp={() => {}}
            >
                {isFolder ? (props.node.is_expanded ? "📂" : "📁") : "📄"}{" "}
                {props.node.name}
            </div>

            <Show when={isFolder && props.node.is_expanded}>
                <For each={props.node.children}>
                    {(child) => (
                        <FileTree node={child} level={(props.level ?? 0) + 1} />
                    )}
                </For>
            </Show>
        </div>
    );
};
