// SEB MessageBoxFactory shows the native Windows message box; this copies the Windows 10 one (no XAML exists).
import { useEffect } from "react";
import { text } from "../text";
import { answer, param, textParam, useDialogWindow } from "./dialog";
import "./dialogs.css";

const icons = {
	// Windows 10 question icon: blue disc with a white question mark.
	question: (
		<svg viewBox="0 0 32 32" aria-hidden="true">
			<circle cx="16" cy="16" r="15" fill="#1c71d8" />
			<path
				d="M11.5 12a4.5 4.5 0 1 1 6.3 4.1c-1.1.5-1.8 1.4-1.8 2.6V20"
				fill="none"
				stroke="#fff"
				strokeWidth="3"
				strokeLinecap="round"
			/>
			<circle cx="16" cy="25" r="1.9" fill="#fff" />
		</svg>
	),
	// Windows 10 warning icon: yellow triangle with a black exclamation mark.
	warning: (
		<svg viewBox="0 0 32 32" aria-hidden="true">
			<path
				d="M16 2.5 31 29.5H1z"
				fill="#fcd116"
				stroke="#e3a600"
				strokeLinejoin="round"
			/>
			<path d="M16 11v10" stroke="#000" strokeWidth="3" strokeLinecap="round" />
			<circle cx="16" cy="25.5" r="1.8" />
		</svg>
	),
};

const buttons = {
	ok: [["ok", text.Ok]],
	yesno: [
		["yes", text.Yes],
		["no", text.No],
	],
} as const;

export default function MessageBox() {
	const kind = param("buttons") as keyof typeof buttons;
	// Windows disables the close button of a Yes/No box (Wayland cannot, so closing is refused); OK boxes close with Escape too.
	const ref = useDialogWindow<HTMLDivElement>(async (window) => {
		if (kind === "yesno")
			await window.onCloseRequested((e) => e.preventDefault());
	});
	useEffect(() => {
		if (kind !== "ok") return;
		const onKey = (e: KeyboardEvent) => e.key === "Escape" && answer("ok");
		addEventListener("keydown", onKey);
		return () => removeEventListener("keydown", onKey);
	}, [kind]);

	return (
		<div className="message-box" ref={ref}>
			<div className="content">
				{icons[param("icon") as keyof typeof icons]}
				<p>{textParam("message")}</p>
			</div>
			<div className="buttons">
				{buttons[kind].map(([value, label], i) => (
					<button
						key={value}
						type="button"
						className="win32"
						// biome-ignore lint/a11y/noAutofocus: Windows focuses the first button of a message box.
						autoFocus={i === 0}
						onClick={() => answer(value)}
					>
						{label}
					</button>
				))}
			</div>
		</div>
	);
}
