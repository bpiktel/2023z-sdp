import { Link } from "@tanstack/react-router";

const HomePage = () => {
  return (
    <div className="flex flex-col items-center">
      <h1 className="">Home</h1>
      <Link to="/experiments">Go to experiment</Link>
    </div>
  );
};

export default HomePage;
