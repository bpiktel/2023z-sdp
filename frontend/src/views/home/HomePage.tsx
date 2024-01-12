import { Link } from "@tanstack/react-router";
import { useAuth } from "auth";

const HomePage = () => {
  const { authenticated } = useAuth();

  return (
    <div className="flex w-full flex-col items-center p-xl">
      <div className="ml-auto">
        {authenticated ? (
          <div>You are authenticated</div>
        ) : (
          <div>You are not authenticated</div>
        )}
      </div>
      <h1 className="mb-s">Home</h1>
      <div className="flex flex-col items-center mt-md gap-xs">
        <Link to="/login">Go to login</Link>
        <Link to="/experiments">Go to experiments</Link>
        <Link to="/samples">Go to samples</Link>
      </div>
    </div>
  );
};

export default HomePage;
