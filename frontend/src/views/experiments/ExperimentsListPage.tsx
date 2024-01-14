import { useQuery, useQueryClient } from "@tanstack/react-query";
import { Link } from "@tanstack/react-router";
import { fireConfirmationModal } from "components/AlertDialogs";
import { ButtonSecondary } from "components/Buttons";
import { FaTrash, FaArrowLeft } from "react-icons/fa";
import { experimentListSchema } from "schemas/experimentSchemas";

const deleteExperiment = async (id: string, callback: () => void) => {
  const { VITE_BASE_API_URL } = import.meta.env;

  try {
    const response = await fetch(`${VITE_BASE_API_URL}/experiments/${id}`, {
      method: "DELETE"
    });

    if (response.ok) {
      callback();
    }
  } catch (error) {
    console.error(error);
  }
};

const ExperimentsListPage = () => {
  const { VITE_BASE_API_URL } = import.meta.env;

  const queryClient = useQueryClient();

  const getExperiments = () =>
    fetch(`${VITE_BASE_API_URL}/experiments`)
      .then((res) => res.json())
      .then((data) => experimentListSchema.parse(data));

  const { data, isLoading, isFetching, error } = useQuery({
    queryKey: ["experiments"],
    queryFn: getExperiments
  });

  const onDelete = async (id: string) => {
    await fireConfirmationModal({
      title: "Delete experiment",
      body: "Are you sure you want to delete this experiment?"
    }).then((result) => {
      if (result.isConfirmed) {
        deleteExperiment(id, () => {
          queryClient.invalidateQueries({ queryKey: ["experiments"] });
        });
      }
    });
  };

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
    <div className="flex flex-col items-center p-xl">
      <div className="absolute left-0 top-0 m-xl">
        <Link to="../" className="flex gap-xs items-center">
          <FaArrowLeft /> Return to Home Page
        </Link>
      </div>
      <h1>Experiments</h1>
      <div className="mt-md flex flex-col gap-sm items-center">
        {data?.map((experiment) => (
          <div
            key={experiment.id.id.String}
            className="flex gap-xs items-center"
          >
            <Link
              to={`/experiments/$id`}
              params={{ id: experiment.id.id.String }}
            >
              {experiment.name}
            </Link>
            <FaTrash
              className="text-red-500 cursor-pointer"
              onClick={() => onDelete(experiment.id.id.String)}
            />
          </div>
        ))}
      </div>
      <Link to="/experiments/create" className="mt-md">
        <ButtonSecondary>Create experiments</ButtonSecondary>
      </Link>
    </div>
  );
};

export default ExperimentsListPage;
