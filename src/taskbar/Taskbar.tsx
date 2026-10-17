// Windows/Taskbar.xaml: clock and quit button on the right (no application, notification or system controls yet).
import { invoke } from "@tauri-apps/api/core";
import Clock from "./Clock";
import QuitButton from "./QuitButton";
import "./taskbar.css";

const flags = new URLSearchParams(location.hash.slice(1));

export default function Taskbar() {
  return (
    <div className="taskbar">
      {flags.get("clock") === "true" && <Clock />}
      {flags.get("quit") === "true" && <QuitButton onClick={() => invoke("quit")} />}
    </div>
  );
}
