import { For, createMemo } from "solid-js";
import classNames from "classnames";
import { createVirtualizer } from "@tanstack/solid-virtual";
import { windowState } from "@state/store";

export const TabView = () => {
    let scrollRef!: HTMLDivElement;

    const rowVirtualizer = createMemo(() =>
        createVirtualizer({
            getScrollElement: () => scrollRef,
            estimateSize: () => 36,
            overscan: 10,
            count: windowState.tabs.length,
            debug: true,
        }),
    );

    return (
        <div
            class="h-full w-full overflow-auto overscroll-none flex flex-col gap-2"
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
                        const tab = windowState.tabs[virtualItem.index];
                        return (
                            <div
                                class="absolute top-0 left-0 right-0 flex items-center"
                                style={{
                                    height: `${virtualItem.size}px`,
                                    transform: `translateY(${virtualItem.start}px)`,
                                }}
                            >
                                <div
                                    class={classNames(
                                        "rounded-2xl cursor-default select-none flex-1",
                                        "text-[12px] text-white/70",
                                        "h-9 flex items-center px-4",
                                        "hover:bg-white/10 hover:text-white",
                                        {
                                            "!bg-white/20 !text-white":
                                                tab.is_selected,
                                        },
                                    )}
                                >
                                    <p>{tab.title}</p>
                                </div>
                            </div>
                        );
                    }}
                </For>
            </div>
        </div>
    );
};
