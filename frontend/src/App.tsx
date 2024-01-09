import "./index.css";
import { Outlet, RootRoute, Route, Router } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/router-devtools";
import HomePage from "./views/home/HomePage";
import { QueryClient } from "@tanstack/react-query";
import ExperimentsListPage from "views/experiments/ExperimentsListPage";
import LoginPage from "views/auth/LoginPage";
import ExperimentPage from "views/experiments/ExperimentPage";

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

const experimentsRoute = new Route({
  getParentRoute: () => rootRoute,
  path: "/experiments"
});

const experimentsListRoute = new Route({
  getParentRoute: () => experimentsRoute,
  path: "/",
  component: ExperimentsListPage
});

const experimentRoute = new Route({
  getParentRoute: () => experimentsRoute,
  path: "/$id",
  component: ExperimentPage
});

const loginRoute = new Route({
  getParentRoute: () => rootRoute,
  path: "/login",
  component: LoginPage
});

const routeTree = rootRoute.addChildren([
  homeRoute,
  loginRoute,
  experimentsRoute.addChildren([experimentsListRoute, experimentRoute])
]);

export const queryClient = new QueryClient();

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
