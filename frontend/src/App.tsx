import "./index.css";
import { Outlet, RootRoute, Route, Router } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/router-devtools";
import HomePage from "./views/home/HomePage";
import { QueryClient } from "@tanstack/react-query";
import ExperimentPage from "./views/experiments/ExperimentPage";

const rootRoute = new RootRoute({
  component: () => (
    <>
      <Outlet />
      <TanStackRouterDevtools />
    </>
  )
});

const homeRoute = new Route({
  getParentRoute: () => rootRoute,
  path: "/",
  component: HomePage
});

const adminRoute = new Route({
  getParentRoute: () => rootRoute,
  path: "/experiment",
  component: ExperimentPage
});

const routeTree = rootRoute.addChildren([homeRoute, adminRoute]);

const queryClient = new QueryClient();

const router = new Router({
  routeTree,
  defaultPreload: "intent",
  // Since we're using React Query, we don't want loader calls to ever be stale
  // This will ensure that the loader is always called when the route is preloaded or visited
  defaultPreloadStaleTime: 0,
  context: {
    queryClient
  }
});

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}

export default router;
