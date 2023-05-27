import { Dropdown, Play, Share } from "../icons/icons";

export default function QuickActionBar() {
    return (
        <>
            <div className="quickactionbar">
                <button title="Export" className="button">
                        <div className="wrap-blockicon">
                            <Share />
                        </div>
                        <p>Export</p>
                </button>
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
