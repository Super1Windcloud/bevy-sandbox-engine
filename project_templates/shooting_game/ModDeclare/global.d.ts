/**
 * Project-wide global type declarations.
 *
 * Put custom engine-adjacent global declarations here instead of editing
 * generated files under Cache/ or Library/.
 *
 * This project uses `module: "none"`, so top-level `declare` statements are
 * merged into the global script scope automatically.
 */

declare type Nullable<T> = T | null;
declare type Optional<T> = T | undefined;

/**
 * Example global build-time constants.
 * Replace or remove these when the project has real values wired in.
 */
declare const BUILD_ENV: "dev" | "test" | "prod";
declare const GAME_VERSION: string;

/**
 * Example shape for globally shared runtime config.
 * Extend this interface as the project grows.
 */
declare interface GameRuntimeConfig {
	debug: boolean;
	version: string;
}

/**
 * Example global singleton declaration pattern.
 * Copy this style for managers exposed globally by the engine or launcher.
 */
declare class GlobalGameManager {
	static Instance: GlobalGameManager;
	Init(): void;
}
