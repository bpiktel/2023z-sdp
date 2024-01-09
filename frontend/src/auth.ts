import { createContext, useContext } from "react";

interface AuthContext {
  authenticated: boolean;
  setAuthenticated: (auth: boolean) => void;
  username?: string;
}

export const AuthContext = createContext<AuthContext>({
  authenticated: false,
  setAuthenticated: () => {}
});

export const useAuth = () => {
  const auth = useContext(AuthContext);
  return auth;
};
