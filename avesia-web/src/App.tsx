import "./App.css";
import AppLayout from "./ui/layouts/AppLayout";
import { HashRouter, Routes, Route } from "react-router-dom";
import MainWindowPage from "./ui/pages/MainWindowPage";

function App() {
    return (
        <>
            <AppLayout>
                <HashRouter>
                    <Routes>
                        <Route path="/" element={<MainWindowPage />} />
                    </Routes>
                </HashRouter>
            </AppLayout>
        </>
    );
}

export default App;
