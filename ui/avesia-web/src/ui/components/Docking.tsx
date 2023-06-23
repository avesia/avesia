import { useEffect } from "react";

export interface Window {
    windowId: string,
    title: string,
    dockedAreaId: string,
    element: React.ReactNode,
}

export interface DockingArea {
    areaId: string,
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
