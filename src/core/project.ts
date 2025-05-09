import {invoke} from "@tauri-apps/api/core";
import {isSuccess, Response} from "./resposne/response.ts";

export interface Project {
    name: string;
    simpleName: string;
    svnName: string;
}

export class ItemListProject implements Project {
    name: string;
    simpleName: string;
    svnName: string;

    constructor(name: string, simpleName: string, svnName: string) {
        this.name = name;
        this.simpleName = simpleName;
        this.svnName = svnName;
    }
}

type GetProjectItemListCallback = (projects: ItemListProject[]) => void;

export function fetchProjectItemList(callback: GetProjectItemListCallback) {
    invoke<Response<ItemListProject[]>>("get_all_projects").then(response => {
        if (isSuccess(response)) {
            callback(response.data);
        }
    });
}


