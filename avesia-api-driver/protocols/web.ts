import { AvesiaAPIServerDriver } from "../interface";

export interface WebSession {
    sessionId: string,
    userId: string,
    expiresAt: Date,
}

interface AvesiaWebDriverOptions {
    host: string,
    port: number,
    tokenKey: string,
    userId: string,
}

function makeSessionFromToken() {
    
}

export default class AvesiaWebAPIDriver extends AvesiaAPIServerDriver {
    host: string;
    port: number;
    tokenKey: string;
    
    constructor(options: AvesiaWebDriverOptions) {
        super();

        this.host = options.host;
        this.port = options.port;
        this.tokenKey = options.tokenKey;
    }

    public makeSession() {
        
    }

    public checkVersion(): Promise<number> {
        // TODO
        return new Promise<number> (() => {});
    }
}
