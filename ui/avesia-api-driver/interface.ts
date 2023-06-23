export abstract class AvesiaAPIServerDriver {
    abstract checkVersion(): Promise<number>;
}
