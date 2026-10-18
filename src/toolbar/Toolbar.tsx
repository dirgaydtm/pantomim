// Windows/BrowserWindow.xaml toolbar row: home, back, forward, reload and the address bar.
// Visibility and enabled state follow BrowserWindow.ApplySettings (passed in by shell.rs).
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { type ReactNode, useEffect, useState } from "react";
import { param } from "../dialogs/dialog";
import { text } from "../text";
import "./toolbar.css";

const on = (name: string) => param(name) === "true";
const browse = (action: string, address?: string) =>
	invoke("browse", { action, address });

// Images/*.xaml: 1024 px canvases stroked at 70 px with round caps and joins.
const icon = (paths: ReactNode) => (
	<svg viewBox="0 0 1024 1024" aria-hidden="true">
		{paths}
	</svg>
);
const icons = {
	home: icon(
		<>
			<path d="M301.086 510.424 786.763 188.341 786.763 835.919 301.086 510.424Z" />
			<path d="M235.016 188.341 235.016 835.919" />
		</>,
	),
	back: icon(<path d="M753.838 835.919 268.162 510.424 753.838 188.341" />),
	forward: icon(<path d="M268.162 835.919 753.838 510.424 268.162 188.341" />),
	reload: icon(
		<>
			<path d="M862 557C862 750.3 705.3 907 512 907 318.7 907 162 750.3 162 557 162 363.7 318.7 207 512 207" />
			<path className="fill" d="M471 67 712.5 207 471 345Z" />
		</>,
	),
};

function Button({
	action,
	label,
	enabled = true,
}: {
	action: keyof typeof icons;
	label: string;
	enabled?: boolean;
}) {
	return (
		<button
			type="button"
			className="browser-button"
			aria-label={label}
			disabled={!enabled}
			onClick={() => browse(action)}
		>
			{icons[action]}
		</button>
	);
}

export default function Toolbar() {
	const [address, setAddress] = useState(param("address"));
	useEffect(() => {
		const unlisten = listen<string>("address", (event) =>
			setAddress(event.payload),
		);
		return () => {
			unlisten.then((stop) => stop());
		};
	}, []);

	return (
		<div className="toolbar">
			{on("home") && (
				<Button action="home" label={text.BrowserWindow_HomeButton} />
			)}
			{on("navigation") && (
				<Button action="back" label={text.BrowserWindow_BackwardButton} />
			)}
			{on("navigation") && (
				<Button action="forward" label={text.BrowserWindow_ForwardButton} />
			)}
			{on("reload") && (
				<Button
					action="reload"
					label={text.BrowserWindow_ReloadButton}
					enabled={on("reloadEnabled")}
				/>
			)}
			{/* Hidden rather than collapsed when the address bar isn't allowed, so it keeps its space. */}
			<input
				className="address"
				style={{ visibility: on("addressBar") ? "visible" : "hidden" }}
				aria-label={text.BrowserWindow_UrlTextBox}
				value={address}
				onChange={(e) => setAddress(e.target.value)}
				onFocus={(e) => e.target.select()}
				onKeyUp={(e) => e.key === "Enter" && browse("address", address)}
			/>
		</div>
	);
}
