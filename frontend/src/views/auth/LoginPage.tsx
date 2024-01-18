import { Link } from "@tanstack/react-router";
import { useAuth } from "auth";
import { useState } from "react";
import { FrostedGlass } from "../../components/FrostedGlass.tsx";
import { FaArrowLeft } from "react-icons/fa";
import { signIn } from "../../utils/authUtils.ts";

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
          <FrostedGlass>
            <h1>Authenticated</h1>
            <div className="mt-md">
              <Link to="/" className="flex gap-xs items-center justify-center">
                <FaArrowLeft /> Return to Home Page
              </Link>
            </div>
          </FrostedGlass>
        ) : (
          <>
            <FrostedGlass>
              <h1>Login</h1>
              <div className="flex flex-col mt-md">
                <input
                  className="py-1 px-2"
                  type="text"
                  placeholder="Username"
                  onChange={(e) => setUsername(e.target.value)}
                />
                <input
                  className="mt-sm py-1 px-2"
                  type="password"
                  placeholder="Password"
                  onChange={(e) => setPassword(e.target.value)}
                />
              </div>
              <div className="flex flex-col mt-md">
                <button onClick={handleSignIn} className="border py-1">
                  Sign In
                </button>
              </div>
            </FrostedGlass>
            <div className="absolute left-0 top-0 m-xl">
              <Link to="../" className="flex gap-xs items-center">
                <FaArrowLeft /> Return to Home Page
              </Link>
            </div>
          </>
        )}
      </div>
    </div>
  );
};

export default LoginPage;
