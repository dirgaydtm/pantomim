// Controls/Taskbar/Clock.xaml: bold short time above short date, centered.
import { useEffect, useState } from "react";
import { indonesian } from "../text";

// .NET ShortDatePattern for id-ID is dd/MM/yyyy; the browser default drops the leading zeros.
const dateFormat: Intl.DateTimeFormatOptions = indonesian
	? { day: "2-digit", month: "2-digit", year: "numeric" }
	: {};

export default function Clock() {
	const [now, setNow] = useState(new Date());
	useEffect(() => {
		const timer = setInterval(() => setNow(new Date()), 1000);
		return () => clearInterval(timer);
	}, []);
	return (
		<div className="clock">
			<span className="time">
				{now.toLocaleTimeString([], { hour: "numeric", minute: "2-digit" })}
			</span>
			<span className="date">{now.toLocaleDateString([], dateFormat)}</span>
		</div>
	);
}
