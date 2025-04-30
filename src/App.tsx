import {NavLink, Outlet} from "react-router-dom";
import {useState} from "react";

export default function App() {

    const [activeMenuItem, setActiveMenuItem] = useState("");

    return (
        <div className="flex h-screen bg-gray-200">
            <nav className={"flex flex-col w-32 bg-white"}>
                {
                    navMenuItems().map(item => (
                        <NavLink to={item.path}
                                 onClick={() => setActiveMenuItem(item.name)}
                                 className={`rounded text-center mx-1 my-1 text-lg ${
                                     activeMenuItem === item.name ? "bg-gray-500" : "hover:bg-gray-400 bg-gray-300"
                                 }`}>
                            {item.label}
                        </NavLink>
                    ))
                }
            </nav>
            <Outlet/>
        </div>
    )
}

interface NavMenuItem {
    name: string;
    label: string;
    path: string;
}

function navMenuItems(): NavMenuItem[] {
    return [
        {
            name: "today",
            label: "今日",
            path: "/"
        },
        {
            name: "tasks",
            label: "任务",
            path: "/tasks"
        },
        {
            name: "projects",
            label: "项目",
            path: "/projects"
        },
        {
            name: "statistics",
            label: "统计",
            path: "/statistics"
        }
    ];
}
