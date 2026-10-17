// Controls/Taskbar/QuitButton.xaml with the Images/ShutDown.xaml icon.
import { text } from "../text";

export default function QuitButton({ onClick }: { onClick: () => void }) {
	return (
		<button
			type="button"
			className="taskbar-button"
			title={text.Shell_QuitButton}
			aria-label={text.Shell_QuitButton}
			onClick={onClick}
		>
			<svg
				aria-hidden="true"
				viewBox="0 0 1024 1024"
				fill="none"
				stroke="#000"
				strokeWidth="79"
				strokeLinecap="round"
				strokeLinejoin="round"
			>
				<path d="M 512,902 C 296.61,902 122,727.28 122,511.74 C 122,388.11 179.45,277.91 269.09,206.4" />
				<path d="M 512,902 C 727.39,902 902,727.28 902,511.74 C 902,388.11 844.55,277.91 754.91,206.4" />
				<path d="M 512.56,112 L 512.56,494.46" />
			</svg>
		</button>
	);
}
