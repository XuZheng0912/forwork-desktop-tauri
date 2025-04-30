import {NavLink} from "react-router-dom";

export default function Home() {
    return (
        <div>
            <h2>
                <NavLink to={"/projects"}>
                    Projects
                </NavLink>
            </h2>
        </div>
    )
}