import React from "react";
import ReactDOM from "react-dom/client";
import Taskbar from "./taskbar/Taskbar";
import "./main.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <Taskbar />
  </React.StrictMode>,
);
