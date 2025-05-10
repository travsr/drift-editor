import { For, Show } from "solid-js";
import type { D_FileTreeNode } from "@schemas/index";

type FileTreeProps = {
    node: D_FileTreeNode;
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
                {isFolder ? (props.node.isExpanded ? "📂" : "📁") : "📄"}{" "}
                {props.node.name}
            </div>

            <Show when={isFolder && props.node.isExpanded}>
                <For each={props.node.children}>
                    {(child) => (
                        <FileTree node={child} level={(props.level ?? 0) + 1} />
                    )}
                </For>
            </Show>
        </div>
    );
};
