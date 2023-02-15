import { invoke } from "@tauri-apps/api/tauri";
import { createContext, PropsWithChildren, useEffect, useRef, useState } from "react";
import { UPDATE_INTERVAL } from "./constants";

type ProcessMap = {
    [key : string] : Process
}

export type Process = {
    name: string,
    window_title: string,
    pid: number,
    usage: number
}

export type RootCtx = {
    tracked_processes: ProcessMap
}

export let RootContext = createContext<RootCtx>({tracked_processes : {}});

export function RootContextProvider({children}: PropsWithChildren) {
    let [procMap, setProcMap] = useState<ProcessMap>({}); 
    
    // have to use a ref here to persist a reference to
    // procMap. [the setInterval() captures the procMap reference
    // returned by useState() during the initial render and uses that 
    // on subsequent executions]
    let procMapRef = useRef(procMap);

    useEffect(() => {
        (async () => {
            let proc_map: ProcessMap = await invoke("get_processes");

            setProcMap(proc_map);
            procMapRef.current = procMap
        })();
        
        let interv = setInterval(async () => {
            let proc_list_update: ProcessMap = await invoke("get_process_list_update");
            
            let new_proc_list = Object.assign({}, procMapRef.current);
            Object.values(proc_list_update).forEach((p) => {
                new_proc_list[p.name] = p;
            });

            setProcMap(new_proc_list);
            procMapRef.current = new_proc_list;
        }, UPDATE_INTERVAL * 1000)
        
        return () => {
            //cleanup
            clearInterval(interv);
        }
    }, [])

    return (
        <RootContext.Provider value={{tracked_processes: procMap}}>
            {children}
        </RootContext.Provider>
    )
}