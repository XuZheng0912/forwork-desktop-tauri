import {ChildrenProps} from "../props.ts";


export default function ProjectItemCard(
    {children, className}: ChildrenProps
) {
    return (
        <div className={`m-2 h-28  transition
            bg-gray-400 hover:bg-gray-300
            rounded 
            duration-300 hover:-translate-1 
            ${className}`}
        >
            <div className={"m-2"}>
                {children}
            </div>
        </div>
    )
}