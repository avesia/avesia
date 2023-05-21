import "./App.css";
import AppLayout from "./ui/layouts/AppLayout";
import { HashRouter, Routes, Route } from "react-router-dom";

function App() {
    return (
        <>
            <AppLayout>
                <HashRouter>
                    <Routes>
                        <Route path="/" element={<><h1>hello</h1></>} />
                    </Routes>
                </HashRouter>
            </AppLayout>
        </>
    );
}

export default App;
