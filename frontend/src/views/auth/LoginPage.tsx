import { useState } from "react";

const signIn = async (username: string, password: string): Promise<void> => {
  const { VITE_BASE_API_URL } = import.meta.env;

  const response = await fetch(`${VITE_BASE_API_URL}/auth/login`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify({ username, password }),
    credentials: "include"
  });
  if (!response.ok) throw new Error("Failed on sign up request");
};

const LoginPage = () => {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");

  const handleSignIn = async () => {
    try {
      await signIn(username, password);
    } catch (error) {
      console.error(error);
    }
  };

  return (
    <div>
      <h1>Login Page</h1>
      <div className="flex flex-col">
        <input
          type="text"
          placeholder="Username"
          onChange={(e) => setUsername(e.target.value)}
        />
        <input
          type="password"
          placeholder="Password"
          onChange={(e) => setPassword(e.target.value)}
        />
      </div>
      <div className="flex flex-col">
        <button onClick={handleSignIn}>Sign In</button>
      </div>
    </div>
  );
};

export default LoginPage;
