import {ReactNode} from "react";

export interface ClassNameProps {
    className?: string
}

export interface ChildrenProps extends ClassNameProps {
    children: ReactNode
}