import { Dropdown, Play } from "../icons/icons";

export default function QuickActionBar() {
    return (
        <>
            <div className="quickactionbar">
                <div className="button-group">
                    <button title="Preview" className="button primary">
                        <div className="wrap-blockicon">
                            <Play />
                        </div>
                        <p>Preview</p>
                    </button>
                    <button className="button primary fit" title="More actions..."><Dropdown /></button>
                </div>
            </div>
        </>
    );
}
