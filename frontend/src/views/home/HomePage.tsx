import { Link } from "@tanstack/react-router";

const HomePage = () => {
  return (
    <div className="flex flex-col items-center">
      <h1 className="">Home</h1>
      <Link to="/login">Go to login</Link>
      <Link to="/experiments">Go to experiments</Link>
    </div>
  );
};

export default HomePage;
