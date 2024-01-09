import { RouterProvider } from "@tanstack/react-router";
import router, { queryClient } from "./routes";
import { QueryClientProvider } from "@tanstack/react-query";
import { AuthContext } from "auth";
import { useState } from "react";

const App = () => {
  const [authenticated, setAuthenticated] = useState(false);

  return (
    <QueryClientProvider client={queryClient}>
      <AuthContext.Provider value={{ authenticated, setAuthenticated }}>
        <RouterProvider router={router} />
      </AuthContext.Provider>
    </QueryClientProvider>
  );
};

export default App;
