import React from "react";
import ReactDOM from "react-dom/client";
import "./App.css";
import {RouterProvider} from "react-router-dom";
import {router} from "./routes/router.tsx";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <RouterProvider router={router}/>
    </React.StrictMode>,
);
