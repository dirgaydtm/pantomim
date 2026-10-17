import React from "react";
import ReactDOM from "react-dom/client";
import MessageBox from "./dialogs/MessageBox";
import PasswordDialog from "./dialogs/PasswordDialog";
import Taskbar from "./taskbar/Taskbar";
import "./main.css";

// Rust opens each screen as its own webview at index.html#screen=<name>&...
const screens = {
	taskbar: Taskbar,
	message: MessageBox,
	password: PasswordDialog,
};
const name = new URLSearchParams(location.hash.slice(1)).get(
	"screen",
) as keyof typeof screens;
const Screen = screens[name];

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
	<React.StrictMode>
		<Screen />
	</React.StrictMode>,
);
