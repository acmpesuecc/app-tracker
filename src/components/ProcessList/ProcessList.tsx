import { useContext } from "react";
import { RootContext } from "../../RootContext";
import "./style.css";

function formatTime(secs: number) {
	let vals = [86400, 3600, 60, 1];
	let symbols = ["d", "h", "m", "s"]

	let formatted = ""

	for(let i = 0; i < vals.length; i++) {
		let val = vals[i];
		let sym = symbols[i];

		let v = Math.floor(secs / val);

		if (v > 0)
			formatted += `${v}${sym} `;

		secs = secs % val;
	}

	return formatted;
}

export function ProcessList() {
	let root_ctx = useContext(RootContext);
	let total_usage = Object.values(root_ctx?.tracked_processes ?? {}).reduce((total, curr) => {
		return total + (curr.usage ?? 0);
	}, 0)

	return (
		<>
			<table className="process-list">
				<thead>
					<tr className="head-row">
						<th className="PID">PID</th>
						<th className="Name">Process name</th>
						<th className="usage">Uptime</th>
						<th className="percent-usage">Usage%</th>
					</tr>
				</thead>
				<tbody>
					{Object.values(root_ctx?.tracked_processes ?? {}).sort((p1, p2) => {
						return p2.usage - p1.usage;
					}).map((p, idx) => (
						<tr className="process-row" key={idx}>
							<td className="process-pid">{p.pid.toString()}</td>
							<td className="process-name">{p.name}</td>
							<td className="process-usage">{`${formatTime(p.usage ?? 0)}`}</td>
							<td className="process-percent-usage">{`${((p.usage ?? 0) / total_usage * 100).toFixed(2)}%`}</td>
						</tr>
					))}
				</tbody>
			</table>
		</>
	);
}
