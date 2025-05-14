import { For } from "solid-js";
import type { DTab } from "@schemas/index";
import classNames from "classnames";

type TabViewProps = {
    tabs: DTab[];
};

export const TabView = (props: TabViewProps) => (
    <div class="flex flex-col gap-2">
        <For each={props.tabs}>
            {(tab, index) => (
                <div
                    class={classNames(
                        "rounded-2xl cursor-default select-none",
                        "text-center text-[12px] text-white",
                        "h-9 flex items-center justify-center",
                        "hover:bg-white/10",
                        {
                            "!bg-white/20": tab.is_selected,
                        },
                    )}
                >
                    <p>{tab.title}</p>
                </div>
            )}
        </For>
    </div>
);
