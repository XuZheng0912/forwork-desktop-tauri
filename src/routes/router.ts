import {createBrowserRouter} from "react-router-dom";
import Home from "../pages/Home.tsx";
import App from "../App.tsx";
import Projects from "../pages/Projects.tsx";

export const router = createBrowserRouter([
    {
        path: "/",
        Component: App,
        children: [
            {index: true, Component: Home},
            {path: "projects", Component: Projects}
        ]
    }
]);