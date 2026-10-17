// Windows/PasswordDialog.xaml with the FontAwesome 4.7 "Key" icon (FontAwesome.WPF), rotated 90°.
import { useState } from "react";
import { text } from "../text";
import { answer, textOf, useDialogWindow } from "./dialog";
import "./dialogs.css";

export default function PasswordDialog() {
	const [password, setPassword] = useState("");
	// Topmost="True".
	const ref = useDialogWindow<HTMLDivElement>((window) =>
		window.setAlwaysOnTop(true),
	);
	return (
		<div className="password-dialog" ref={ref}>
			<div className="content">
				<svg className="key" viewBox="0 -1408 1684 1592" aria-hidden="true">
					<path
						transform="scale(1,-1)"
						d="M832 1024q0 80 -56 136t-136 56t-136 -56t-56 -136q0 -42 19 -83q-41 19 -83 19q-80 0 -136 -56t-56 -136t56 -136t136 -56t136 56t56 136q0 42 -19 83q41 -19 83 -19q80 0 136 56t56 136zM1683 320q0 -17 -49 -66t-66 -49q-9 0 -28.5 16t-36.5 33t-38.5 40t-24.5 26l-96 -96l220 -220q28 -28 28 -68q0 -42 -39 -81t-81 -39q-40 0 -68 28l-671 671q-176 -131 -365 -131q-163 0 -265.5 102.5t-102.5 265.5q0 160 95 313t248 248t313 95q163 0 265.5 -102.5t102.5 -265.5q0 -189 -131 -365l355 -355l96 96q-3 3 -26 24.5t-40 38.5t-33 36.5t-16 28.5q0 17 49 66t66 49q13 0 23 -10q6 -6 46 -44.5t82 -79.5t86.5 -86t73 -78t28.5 -41z"
					/>
				</svg>
				<div className="form">
					<p>{textOf()}</p>
					<input
						type="password"
						// biome-ignore lint/a11y/noAutofocus: FocusManager.FocusedElement="{Binding ElementName=Password}".
						autoFocus
						value={password}
						onChange={(e) => setPassword(e.target.value)}
						onKeyDown={(e) => e.key === "Enter" && answer(password)}
					/>
				</div>
			</div>
			<div className="buttons">
				<button
					type="button"
					className="wpf confirm"
					onClick={() => answer(password)}
				>
					{text.PasswordDialog_Confirm}
				</button>
				<button type="button" className="wpf" onClick={() => answer(null)}>
					{text.PasswordDialog_Cancel}
				</button>
			</div>
		</div>
	);
}
