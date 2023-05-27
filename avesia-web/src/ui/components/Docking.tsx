import { useEffect } from "react";

export interface Window {
    windowId: String,
    title: String,
    dockedAreaId: String,
    element: React.ReactNode,
}

export interface DockingArea {
    areaId: String,
}

interface WindowManagerProps {
    areas: Array<DockingArea>,
    windows: Array<Window>,
}

export default function WindowManager(props: WindowManagerProps) {
    props;
    useEffect(() => {
        
    });

    return (
        <>
            <div className="window-area">
                
            </div>
        </>
    );
}
