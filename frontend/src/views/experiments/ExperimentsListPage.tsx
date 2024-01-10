import { useQuery } from "@tanstack/react-query";
import { Link } from "@tanstack/react-router";
import { experimentListSchema } from "schemas/experimentSchemas";

const ExperimentsListPage = () => {
  const { VITE_BASE_API_URL } = import.meta.env;

  const getExperiments = () =>
    fetch(`${VITE_BASE_API_URL}/experiments`)
      .then((res) => res.json())
      .then((data) => experimentListSchema.parse(data));

  const { data, isLoading, isFetching, error } = useQuery({
    queryKey: ["experiments"],
    queryFn: getExperiments
  });

  if (isLoading) {
    return <p>Data is loading...</p>;
  }

  if (isFetching) {
    return <p>Data is fetching...</p>;
  }

  if (error) {
    return <p>There was an error when fetching your data.</p>;
  }

  return (
    <div className="flex flex-col items-center">
      <h1>Experiments</h1>
      <ul className="mt-md">
        {data?.map((experiment) => (
          <li key={experiment.id.id.String}>
            <Link
              to={`/experiments/$id`}
              params={{ id: experiment.id.id.String }}
            >
              {experiment.name}
            </Link>
          </li>
        ))}
      </ul>
      {/*ToDo: Turn into a button. Also make table ^ more readable.*/}
      <Link to="/experiments/create">Create experiments</Link>
    </div>
  );
};

export default ExperimentsListPage;
