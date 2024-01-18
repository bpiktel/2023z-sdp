import { Link } from "@tanstack/react-router";
import { useAuth } from "auth";
import {FrostedGlass} from "../../components/FrostedGlass.tsx";
import {FaArrowRight} from "react-icons/fa";
import { defaultRequestInit } from "utils/fetchUtils.ts";
import { ButtonSecondary } from "components/Buttons.tsx";

const signOut = async (
  setAuth: (auth: boolean) => void
): Promise<void> => {
  const { VITE_BASE_API_URL } = import.meta.env;
  const response = await fetch(`${VITE_BASE_API_URL}/auth/logout`, {
    ...defaultRequestInit,
    method: "POST",
  });
  setAuth(!response.ok);
  if (!response.ok) throw new Error("Failed on sign up request");
};

const HomePage = () => {
  const { authenticated, setAuthenticated } = useAuth();

  const handleSignOut = async () => {
    try {
      await signOut(setAuthenticated);
    } catch (error) {
      console.error(error);
    }
  };

  return (
    <div className="flex w-full flex-col items-center p-xl">
      <div className="ml-auto">
        {authenticated ? (
          <div style={{ textAlign: "right" }}>
            <span>You are authenticated</span>
            {' '}
            <ButtonSecondary onClick={handleSignOut}>
              Sign Out
            </ButtonSecondary>
          </div>
        ) : (
          <div>
            You are not authenticated
            <Link to="/login" className="flex gap-xs mt-sm justify-end">
              Go to Login Page <FaArrowRight />
            </Link>
          </div>
        )}
      </div>
      <FrostedGlass className="flex flex-col items-center p-xl">
        <h1 className="mb-s">Home</h1>
        <div className="flex flex-col items-center mt-md gap-xs">
          <Link className="flex gap-xs py-xs" to="/login">Go to login <FaArrowRight /></Link>
          <Link className="flex gap-xs py-xs" to="/experiments">Go to experiments <FaArrowRight /></Link>
          <Link className="flex gap-xs py-xs" to="/samples">Go to samples <FaArrowRight /></Link>
        </div>
      </FrostedGlass>
    </div>
  );
};

export default HomePage;
