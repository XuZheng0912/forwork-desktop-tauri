import {createBrowserRouter} from "react-router-dom";
import Home from "../pages/Home.tsx";
import App from "../App.tsx";
import Preview from "../pages/Preview.tsx";


export const router = createBrowserRouter([
    {
        path: "/",
        Component: App,
        children: [
            {index: true, Component: Preview},
            {index: false, Component: Home},
        ]
    }
]);