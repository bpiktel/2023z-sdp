import { Link } from "@tanstack/react-router";
import { useAuth } from "auth";
import { FrostedGlass } from "../../components/FrostedGlass.tsx";
import { FaArrowRight, FaDoorOpen } from "react-icons/fa";
import { signOut } from "../../utils/authUtils.ts";
import { ButtonSecondary } from "../../components/Buttons.tsx";

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
      <div className="ml-auto mb-md">
        {authenticated ? (
          <div className="flex flex-row items-center gap-sm">
            You are authenticated
            <ButtonSecondary onClick={handleSignOut}>
              <div className="flex gap-xs">
                Logout <FaDoorOpen />
              </div>
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
          <Link className="flex gap-xs py-xs" to="/login">
            Go to login <FaArrowRight />
          </Link>
          <Link className="flex gap-xs py-xs" to="/experiments">
            Go to experiments <FaArrowRight />
          </Link>
          {authenticated && (
            <Link className="flex gap-xs py-xs" to="/samples">
              Go to samples <FaArrowRight />
            </Link>
          )}
        </div>
      </FrostedGlass>
    </div>
  );
};

export default HomePage;
