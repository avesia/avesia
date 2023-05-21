interface AppLayoutInterface {
    children?: React.ReactNode,
}

export default function AppLayout(props: AppLayoutInterface) {
    return (
        <>
            <div className="app">
                {props.children}
            </div>
        </>
    );
}
