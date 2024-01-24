import { QueryClient } from "@tanstack/react-query";
import { Outlet, RootRoute, Route, Router } from "@tanstack/react-router";
import LoginPage from "views/auth/LoginPage";
import ExperimentPage from "views/experiments/ExperimentPage";
import ExperimentsListPage from "views/experiments/ExperimentsListPage";
import "./index.css";
import HomePage from "./views/home/HomePage";
import CreateExperimentPage from "./views/experiments/CreateExperimentPage.tsx";
import SamplesListPage from "./views/samples/SamplesListPage.tsx";
import CreateSamplePage from "./views/samples/CreateSamplePage.tsx";
import ExperimentResultsPage from "views/experiments/ExperimentResultsPage.tsx";

const rootRoute = new RootRoute({
  component: () => (
    <>
      <Outlet />
    </>
  )
});

const homeRoute = new Route({
  getParentRoute: () => rootRoute,
  path: "/",
  component: HomePage
});

// All experiments routes
const baseAllExperimentsRoute = new Route({
  getParentRoute: () => rootRoute,
  path: "/experiments"
});

const experimentsListRoute = new Route({
  getParentRoute: () => baseAllExperimentsRoute,
  path: "/",
  component: ExperimentsListPage
});

const createExperimentRoute = new Route({
  getParentRoute: () => baseAllExperimentsRoute,
  path: "/create",
  component: CreateExperimentPage
});

// Single experiment routes
const baseExperimentRoute = new Route({
  getParentRoute: () => baseAllExperimentsRoute,
  path: "/$id"
});

const experimentRoute = new Route({
  getParentRoute: () => baseExperimentRoute,
  path: "/",
  component: ExperimentPage
});

const experimentResultsRoute = new Route({
  getParentRoute: () => baseExperimentRoute,
  path: "/results",
  component: ExperimentResultsPage
});

// All samples routes
const samplesRoute = new Route({
  getParentRoute: () => rootRoute,
  path: "/samples"
});

const samplesListRoute = new Route({
  getParentRoute: () => samplesRoute,
  path: "/",
  component: SamplesListPage
});

const createSampleRoute = new Route({
  getParentRoute: () => samplesRoute,
  path: "/create",
  component: CreateSamplePage
});

// Auth routes
const loginRoute = new Route({
  getParentRoute: () => rootRoute,
  path: "/login",
  component: LoginPage
});

const routeTree = rootRoute.addChildren([
  homeRoute,
  loginRoute,
  baseAllExperimentsRoute.addChildren([
    experimentResultsRoute,
    experimentsListRoute,
    baseExperimentRoute.addChildren([experimentRoute, experimentResultsRoute]),
    createExperimentRoute
  ]),
  samplesRoute.addChildren([samplesListRoute, createSampleRoute])
]);

export const queryClient = new QueryClient();

const router = new Router({
  routeTree,
  defaultPreload: "intent",
  // Since we're using React Query, we don't want loader calls to ever be stale
  // This will ensure that the loader is always called when the route is preloaded or visited
  defaultPreloadStaleTime: 0
});

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}

export default router;
