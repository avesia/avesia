import HeaderBar from "../components/HeaderBar";
import QuickActionBar from "../components/QuickActionBar";
import "../../renderer/renderer";
import { useEffect } from "react";
import { start } from "avesia-render-wasm";


export default function MainWindowPage() {
    useEffect(() => {
        start("#avsrenderer");
    }, []);

    return (
        <>
            <HeaderBar />
            <QuickActionBar />
            <canvas id="avsrenderer"></canvas>
        </>
    );
}
