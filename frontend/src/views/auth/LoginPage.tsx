import { useAuth } from "auth";
import { useState } from "react";

const signIn = async (
  username: string,
  password: string,
  setAuth: (auth: boolean) => void
): Promise<void> => {
  const { VITE_BASE_API_URL } = import.meta.env;

  const response = await fetch(`${VITE_BASE_API_URL}/auth/login`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify({ username, password }),
    credentials: "include"
  });
  setAuth(response.ok);

  if (!response.ok) throw new Error("Failed on sign up request");
};

const LoginPage = () => {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");

  const { authenticated, setAuthenticated } = useAuth();

  const handleSignIn = async () => {
    try {
      await signIn(username, password, setAuthenticated);
    } catch (error) {
      console.error(error);
    }
  };

  return (
    <div className="flex h-full items-center justify-center">
      <div className="text-center min-w-[16rem]">
        {authenticated ? (
          <h1>Authenticated</h1>
        ) : (
          <>
            <h1>Login</h1>
            <div className="flex flex-col mt-md">
              <input
                type="text"
                placeholder="Username"
                onChange={(e) => setUsername(e.target.value)}
              />
              <input
                className="mt-sm"
                type="password"
                placeholder="Password"
                onChange={(e) => setPassword(e.target.value)}
              />
            </div>
            <div className="flex flex-col mt-md">
              <button onClick={handleSignIn} className="border">
                Sign In
              </button>
            </div>
          </>
        )}
      </div>
    </div>
  );
};

export default LoginPage;
