import { Outlet } from "react-router-dom"

function Layout() {
    return (
        <div>
            <h1>Layout</h1>
            <Outlet></Outlet>
        </div>
    )
}

export default Layout