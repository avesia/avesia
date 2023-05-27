import { Canvas, Code, Sequence, Stage } from "../icons/icons";

export default function HeaderBar() {
    return (
        <>
            <div className="headerbar-border"></div>
            <div className="headerbar">
                <nav className="headerbar_menu">
                    <button className="headerbar_menu_item active">
                        <div className="headerbar_menu_item_icon wrap-blockicon">
                            <Canvas />
                        </div>
                        <p className="headerbar_menu_item_label">Canvas</p>
                    </button>
                    <button className="headerbar_menu_item">
                        <div className="headerbar_menu_item_icon wrap-blockicon">
                            <Stage />
                        </div>
                        <p className="headerbar_menu_item_label">Stage</p>
                    </button>
                    <button className="headerbar_menu_item">
                        <div className="headerbar_menu_item_icon wrap-blockicon">
                            <Sequence />
                        </div>
                        <p className="headerbar_menu_item_label">Sequence</p>
                    </button>
                    <button className="headerbar_menu_item">
                        <div className="headerbar_menu_item_icon wrap-blockicon">
                            <Code />
                        </div>
                        <p className="headerbar_menu_item_label">Script</p>
                    </button>
                </nav>
            </div>
        </>
    );
}
