import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";
import { createContext, PropsWithChildren, useEffect, useRef, useState } from "react";

type ProcessMap = {
	[key: string]: Process;
};

export type Process = {
	name: string;
	window_title: string;
	pid: number;
	usage: number;
};

export type RootCtx = {
	tracked_processes: ProcessMap;
};

export let RootContext = createContext<RootCtx>({ tracked_processes: {} });

export function RootContextProvider({ children }: PropsWithChildren) {
	let [procMap, setProcMap] = useState<ProcessMap>({});

	// have to use a ref here to persist a reference to
	// procMap. [the setInterval() captures the procMap reference
	// returned by useState() during the initial render and uses that
	// on subsequent executions]
	let procMapRef = useRef(procMap);

	useEffect(() => {
		let unlisten: Promise<() => void> | null = null;

		(async () => {
			let proc_map: ProcessMap = await invoke("get_tracked_apps");

			setProcMap(proc_map);
			procMapRef.current = proc_map;
		})();

		(async () => {
			unlisten = appWindow.listen("tracking-update", (event) => {
				console.log(event.payload);
				if (Object.keys(procMapRef.current).length == 0) return;

				let proc_list_update: ProcessMap = event.payload as ProcessMap;

				let new_proc_list = Object.assign({}, procMapRef.current);
				Object.values(proc_list_update).forEach((p) => {
					new_proc_list[p.name] = p;
				});

				setProcMap(new_proc_list);
				procMapRef.current = new_proc_list;
			});
		})();

		return () => {
			//cleanup
			if (unlisten) {
				(async () => {
					let removelistener = await unlisten;
					removelistener();
				})();
			}
		};
	}, []);

	return <RootContext.Provider value={{ tracked_processes: procMap }}>{children}</RootContext.Provider>;
}
