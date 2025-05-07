import type { D_TabViewProps } from "@schemas/index";
import { For, type Accessor } from "solid-js";

import { SidebarLeft } from "../Icons";
import { WindowControls } from "../WindowControls";

export const TabView = ({
    props,
}: {
    props: Accessor<D_TabViewProps>;
}) => (
    <div class="w-30">
        <div class="flex items-center gap-4 pl-1">
            <WindowControls />
            <div class="opacity-50">
                <SidebarLeft />
            </div>
        </div>

        <div class="flex flex-col gap-2 mt-2">
            <For each={props().tabs}>
                {(tab, index) => (
                    <div class="rounded-2xl bg-[#ffffff33] h-9 flex text-center items-center justify-center text-sm text-white">
                        {tab.title}
                    </div>
                )}
            </For>
        </div>
    </div>
);
