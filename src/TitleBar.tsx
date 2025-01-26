import {appWindow} from "@tauri-apps/api/window";
import "./TitleBar.css"
import React from "react";
import Tauri from "./tauri.ts";

const TitleBar: React.FunctionComponent = () => {
    const [version, setVersion] = React.useState<string>("");

    React.useEffect(() => {
        let is_setup = false;
        const setupVersion = async () => {
            if (!is_setup) {
                const version = await Tauri.version();
                setVersion(`v${version}`);
            }
        }
        setupVersion().catch(console.error);
        return () => {
            is_setup = true;
        }
    })
    return (
        <div data-tauri-drag-region="" className="title-bar">
            <div data-tauri-drag-region="" className="title">Verify Digest&emsp;<span className={"version"}>{version}</span></div>
            <div className="title-bar-button" id="title-bar-minimize"
                 onClick={() => appWindow.minimize()}>
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                    <path d="M20 14H4v-4h16z"/>
                </svg>
            </div>
            <div className="title-bar-button" id="title-bar-maximize"
                 onClick={() => appWindow.toggleMaximize()}>
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                    <path d="M4 4h16v16H4zm2 4v10h12V8z"/>
                </svg>
            </div>
            <div className="title-bar-button" id="title-bar-close"
                 onClick={() => appWindow.close()}>
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                    <path
                        d="M19 6.41L17.59 5L12 10.59L6.41 5L5 6.41L10.59 12L5 17.59L6.41 19L12 13.41L17.59 19L19 17.59L13.41 12z"/>
                </svg>
            </div>
        </div>
    )
}
export default TitleBar;