import {useEffect, useState} from "react";
import {fetchProjectItemList, ItemListProject} from "../core/project.ts";
import ProjectListCard from "../components/project/ProjectListCard.tsx";


export default function Projects() {

    const [projects, setProjects] = useState<ItemListProject[]>([]);

    useEffect(() => {
        fetchProjectItemList(itemProjects => {
            setProjects(itemProjects);
        });
    }, []);

    return (
        <div className={"flex flex-1 flex-col"}>
            {
                projects.map(project => (
                    <div key={project.svnName}>
                        <ProjectListCard project={project}/>
                    </div>
                ))
            }
        </div>
    );
}

