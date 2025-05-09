import {ItemListProject} from "../../core/project.ts";
import ProjectItemCard from "./ProjectItemCard.tsx";
import {ClassNameProps} from "../props.ts";

interface Props extends ClassNameProps {
    project: ItemListProject
}

export default function ProjectListCard({project, className}: Props) {
    return (
        <ProjectItemCard className={className}>
            <div className={"m-2"}>
                {project.simpleName}
            </div>
            <div className={"m-2"}>
                {project.svnName}
            </div>
            <div className={"m-2"}>
                {project.name}
            </div>
        </ProjectItemCard>
    )
}