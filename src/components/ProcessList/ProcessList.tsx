import { useContext } from "react";
import { RootContext } from "../../RootContext";
import styles from "./ProcessList.module.css";

function formatTime(secs: number) {
	let vals = [86400, 3600, 60, 1];
	let symbols = ["d", "h", "m", "s"];

	let formatted = "";

	for (let i = 0; i < vals.length; i++) {
		let val = vals[i];
		let sym = symbols[i];

		let v = Math.floor(secs / val);

		if (v > 0) formatted += `${v}${sym} `;

		secs = secs % val;
	}

	return formatted;
}

export function ProcessList() {
	let root_ctx = useContext(RootContext);
	let total_usage = Object.values(root_ctx?.tracked_processes ?? {}).reduce((total, curr) => {
		return total + (curr.usage ?? 0);
	}, 0);

	return (
		<table className={styles["process-list"]}>
			<thead className={styles["table-head"]}>
				<tr className="head-row">
					<th className="pid">PID</th>
					<th className="name" style={{width: "40%"}}>Process name</th>
					<th className="usage">Uptime</th>
					<th className="percent-usage">Usage%</th>
				</tr>
			</thead>
			<tbody className={styles["table-body"]}>
				{Object.values(root_ctx?.tracked_processes ?? {})
					.sort((p1, p2) => {
						return p2.usage - p1.usage;
					})
					.map((p, idx) => (
						<tr className="process-row" key={idx}>
							<td className="process-pid">{p.pid.toString()}</td>
							<td className="process-name" style={{width: "40%"}}>{p.name}</td>
							<td className="process-usage">{`${formatTime(p.usage ?? 0)}`}</td>
							<td className="process-percent-usage">{`${(((p.usage ?? 0) / total_usage) * 100).toFixed(
								2
							)}%`}</td>
						</tr>
					))}
			</tbody>
		</table>
	);
}
