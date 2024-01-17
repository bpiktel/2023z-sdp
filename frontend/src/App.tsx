import { RouterProvider } from "@tanstack/react-router";
import router, { queryClient } from "./routes";
import { QueryClientProvider } from "@tanstack/react-query";
import { AuthContext } from "auth";
import { useEffect, useState } from "react";
import { defaultRequestInit } from 'utils/fetchUtils';

const checkLoginStatus = async (setAuth: (auth: boolean) => void): Promise<void> => {
  const { VITE_BASE_API_URL } = import.meta.env;
  const response = await fetch(`${VITE_BASE_API_URL}/auth/status`, defaultRequestInit);
  setAuth(response.ok);
};

const App = () => {
  const [authenticated, setAuthenticated] = useState(false);

  useEffect(() => {
    checkLoginStatus(setAuthenticated);
  }, []);

  return (
    <QueryClientProvider client={queryClient}>
      <AuthContext.Provider value={{ authenticated, setAuthenticated }}>
        <RouterProvider router={router} />
      </AuthContext.Provider>
    </QueryClientProvider>
  );
};

export default App;
