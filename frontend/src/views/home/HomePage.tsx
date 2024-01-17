import { Link } from "@tanstack/react-router";
import { useAuth } from "auth";
import {FrostedGlass} from "../../components/FrostedGlass.tsx";
import {FaArrowRight} from "react-icons/fa";

const HomePage = () => {
  const { authenticated } = useAuth();

  return (
    <div className="flex w-full flex-col items-center p-xl">
      <div className="ml-auto">
        {authenticated ? (
          <div>You are authenticated</div>
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
