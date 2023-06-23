export interface AvesiaAppWindow {
    windowId: string,
    x: number,
    y: number,
    width: number,
    height: number,
    isMaximized: boolean,
}

export interface AvesiaAppProfile {
    id: string,
    windows: AvesiaAppWindow,
}

export interface AvesiaAppConfig {
    profileId: string,
}
