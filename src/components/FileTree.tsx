import { createMemo, For } from "solid-js";
import type { DFileTreeNode } from "@schemas/index";
import { invoke } from "@tauri-apps/api/core";
import { windowState } from "@state/store";
import { createVirtualizer, type VirtualItem } from "@tanstack/solid-virtual";

export const FileTree = () => {
    let scrollRef!: HTMLDivElement;

    const rowVirtualizer = createMemo(() =>
        createVirtualizer({
            count: windowState.file_list.length,
            getScrollElement: () => scrollRef,
            estimateSize: () => 24,
            overscan: 10,
        }),
    );

    return (
        <div
            class="h-full w-full overflow-auto overscroll-none"
            ref={(el) => {
                scrollRef = el;
            }}
        >
            <div
                style={{
                    height: `${rowVirtualizer().getTotalSize()}px`,
                    position: "relative",
                }}
            >
                <For each={rowVirtualizer().getVirtualItems()}>
                    {(virtualItem) => {
                        const node = windowState.file_list[virtualItem.index];
                        // return <FileTreeNode node={node} />;

                        return (
                            <FileTreeNode
                                node={node}
                                virtualItem={virtualItem}
                            />
                        );
                    }}
                </For>
            </div>
        </div>
    );
};

type FileTreeNodeProps = {
    node: DFileTreeNode;
    virtualItem: VirtualItem;
};

export const FileTreeNode = (props: FileTreeNodeProps) => {
    const node = () => props.node;

    const isFolder = createMemo(() => node().children.length > 0);
    const isExpanded = createMemo(() => node().is_expanded ?? false);

    const handleClick = () => {
        // console.log(props.nodeId);

        if (!isFolder()) {
            invoke("tc_tab_open", { filePath: node().id });
            console.timeEnd();
            console.time();
        }
    };

    return (
        <div
            class="absolute top-0 left-0 right-0 hover:bg-white/30 rounded flex items-center"
            style={{
                height: `${props.virtualItem.size}px`,
                transform: `translateY(${props.virtualItem.start}px)`,
            }}
            onPointerDown={handleClick}
        >
            <div
                class="cursor-pointer select-none text-white text-[12px] break-keep"
                style={{ "margin-left": `${node().level * 3}px` }}
                onKeyDown={() => {}}
                onKeyUp={() => {}}
            >
                <span class="mr-1 text-sm">
                    {isFolder() ? (isExpanded() ? "📂" : "📁") : "📄"}
                </span>
                {node().name}
            </div>
        </div>
    );
};
