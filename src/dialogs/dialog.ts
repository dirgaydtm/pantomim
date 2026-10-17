import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useEffect, useRef } from "react";
import { type TextKey, text } from "../text";

const params = new URLSearchParams(location.hash.slice(1));
export const param = (name: string) => params.get(name) ?? "";
// SEB texts name the configuration as %%URI%%.
/** The page's text (`text` param) or its title (`text` + "Title"), from src/text.ts. */
export const textOf = (suffix = "") =>
	(text[(param("text") + suffix) as TextKey] ?? "").replace(
		"%%URI%%",
		param("uri"),
	);

/** Sends the dialog's result to Rust (shell/dialog.rs), which closes it. `null` means cancelled. */
export const answer = (value: string | null) => invoke("answer", { value });

/** The window starts hidden: set its title, fit it to the returned element (shell/dialog.rs), then show it. */
export function useDialogWindow<T extends HTMLElement>(
	setup?: (window: ReturnType<typeof getCurrentWindow>) => Promise<void>,
) {
	const ref = useRef<T>(null);
	// biome-ignore lint/correctness/useExhaustiveDependencies: the window is set up once; `setup` is a new closure each render.
	useEffect(() => {
		const window = getCurrentWindow();
		const element = ref.current;
		if (!element) return;
		(async () => {
			await window.setTitle(textOf("Title"));
			await setup?.(window);
			await invoke("fit", {
				width: element.offsetWidth,
				height: element.offsetHeight,
			});
			await window.show();
			await window.setFocus();
		})();
	}, []);
	return ref;
}
