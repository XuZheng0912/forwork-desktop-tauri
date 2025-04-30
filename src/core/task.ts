import {invoke} from "@tauri-apps/api/core";

interface Task {

}

export const getTodayTasks = async () => {
    return await invoke<Task[]>("getTodayTasks");
}