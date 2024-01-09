import { Link } from "@tanstack/react-router";

function HomePage() {
  return (
    <div className="flex flex-col items-center">
      <h1 className="">Home</h1>
      <Link to="/experiment">Go to experiment</Link>
    </div>
  );
}

export default HomePage;
