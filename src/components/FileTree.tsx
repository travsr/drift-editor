import { For, Show } from "solid-js";
import type { DFileTreeNode } from "@schemas/index";
import { invoke } from "@tauri-apps/api/core";

type Props = {
    nodeId: string;
    fileTree: Record<string, DFileTreeNode>;
};

export const FileTreeNode = (props: Props) => {
    const node = () => props.fileTree[props.nodeId];

    const isFolder = () => node().children.length > 0;
    const isExpanded = () => node().is_expanded ?? false;

    const handleClick = () => {
        console.log(props.nodeId);

        invoke("tc_tab_open", { filePath: props.nodeId });
    };

    return (
        <div>
            <div
                class="cursor-pointer select-none hover:bg-white/30 rounded text-white text-[12px] break-keep p-0.5"
                onClick={handleClick}
                onKeyDown={() => {}}
                onKeyUp={() => {}}
            >
                <span class="mr-1 text-sm">
                    {isFolder() ? (isExpanded() ? "📂" : "📁") : "📄"}
                </span>
                {node().name}
            </div>

            <Show when={isFolder() && isExpanded()}>
                <For each={node().children}>
                    {(childId) => (
                        <FileTreeNode
                            nodeId={childId}
                            fileTree={props.fileTree}
                        />
                    )}
                </For>
            </Show>
        </div>
    );
};
