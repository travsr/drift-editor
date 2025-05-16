type DetailViewProps = {
    value: string;
};

export const DetailView = ({
    props,
}: {
    props: DetailViewProps;
}) => {
    return (
        <div class="w-10 rounded-md hover:w-50 transition-all">
            <div class="border-t border-b border-[rgba(255,255,255,0.3)]">
                test
            </div>
        </div>
    );
};
