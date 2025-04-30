import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";

interface AllProjectResponse {
    result: string;
    data: Project[];
}

export default function Projects() {

    const [projects, setProjects] = useState<Project[]>([]);

    useEffect(() => {
        invoke<AllProjectResponse>("get_all_projects").then(response => {
            console.log(response);
            setProjects(response.data);
        });
    }, []);

    return (
        <div className={"flex flex-1 flex-col"}>
            {
                projects.map(project => (
                    <div key={project.svnName} className={"text-2xl m-2 h-32 bg-green-500 hover:bg-green-600 rounded"}>
                        <div className={"m-2"}>
                            {project.simpleName}
                        </div>
                        <div className={"m-2"}>
                            {project.svnName}
                        </div>
                        <div className={"m-2"}>
                            {project.name}
                        </div>
                    </div>

                ))
            }
        </div>
    );
}

