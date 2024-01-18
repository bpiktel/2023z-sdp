import { useQuery, useQueryClient } from "@tanstack/react-query";
import { Link } from "@tanstack/react-router";
import { fireConfirmationModal } from "components/AlertDialogs";
import { ButtonSecondary } from "components/Buttons";
import { FaTrash, FaArrowLeft, FaPlus, FaFile } from "react-icons/fa";
import { experimentListSchema } from "schemas/experimentSchemas";
import { FrostedGlass } from "../../components/FrostedGlass.tsx";
import { defaultRequestInit } from "utils/fetchUtils.ts";

const deleteExperiment = async (id: string, callback: () => void) => {
  const { VITE_BASE_API_URL } = import.meta.env;

  try {
    const response = await fetch(`${VITE_BASE_API_URL}/experiments/${id}`, {
      ...defaultRequestInit,
      method: "DELETE",
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
    fetch(`${VITE_BASE_API_URL}/experiments`, defaultRequestInit)
      .then((res) => res.json())
      .then((data) => experimentListSchema.parse(data));

  const { data, isLoading, isFetching, error } = useQuery({
    queryKey: ["experiments"],
    queryFn: getExperiments,
  });

  const onDelete = async (id: string) => {
    await fireConfirmationModal({
      title: "Delete experiment",
      body: "Are you sure you want to delete this experiment?",
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
      <div className="w-full flex justify-between mb-md">
        <Link to="../" className="flex gap-xs items-center">
          <FaArrowLeft /> Return to Home Page
        </Link>
        <Link to="/experiments/create" className="flex gap-xs items-center">
          <FaPlus /> Create new experiment
        </Link>
      </div>
      <FrostedGlass className="flex flex-col items-center">
        <h1>Experiments</h1>
        <div className="mt-md flex flex-col gap-sm items-center w-full">
          {data?.map((experiment) => (
            <div
              key={experiment.id.id.String}
              className="flex w-full justify-between items-center"
            >
              <Link
                className="text-lg min-w-48 pr-xs"
                to={`/experiments/$id`}
                params={{ id: experiment.id.id.String }}
              >
                {experiment.name}
              </Link>

              <div className="flex gap-xs">
                {/*<Link*/}
                {/*  to={`/experiments/$id`}*/}
                {/*  params={{id: experiment.id.id.String}}*/}
                {/*>*/}
                {/*  <FaEdit className="size-md"/>*/}
                {/*</Link>*/}
                <FaTrash
                  className="size-md text-red-500 cursor-pointer"
                  onClick={() => onDelete(experiment.id.id.String)}
                />
                <Link
                  to={`/experiments/$id/results`}
                  params={{ id: experiment.id.id.String }}
                >
                  <FaFile
                    className="size-md text-white cursor-pointer"
                    onClick={() => {}}
                  />
                </Link>
              </div>
            </div>
          ))}
        </div>
        <Link to="/experiments/create" className="mt-lg w-full flex flex-col">
          <ButtonSecondary>Create new experiment</ButtonSecondary>
        </Link>
      </FrostedGlass>
    </div>
  );
};

export default ExperimentsListPage;
