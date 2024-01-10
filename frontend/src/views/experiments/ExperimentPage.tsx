import { useQuery } from "@tanstack/react-query";
import { useParams } from "@tanstack/react-router";
import { Stage } from "components/Stage";
import { experimentSchema } from "schemas/experimentSchemas";
import SamplePlayer from "../../components/player/SamplePlayer.tsx";

const ExperimentPage = () => {
  const { VITE_BASE_API_URL } = import.meta.env;
  const { id } = useParams({ strict: false });

  const getExperiment = () =>
    fetch(`${VITE_BASE_API_URL}/experiments/${id}`)
      .then((res) => res.json())
      .then((data) => experimentSchema.parse(data));

  const { data, isLoading, isFetching, error } = useQuery({
    queryKey: ["experiment", id],
    queryFn: getExperiment
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
    <div className="w-screen h-screen flex flex-row">
      <Stage />
      <div>
        {data?.name} - {data?.id.id.String}
        {data?.sample_ids.map((sample) => (
          <div key={sample}>{sample}</div>
        ))}
        <SamplePlayer
          assetPath="https://bigsoundbank.com/UPLOAD/mp3/0477.mp3"
          name="Wilhelm Scream"
        />
      </div>
    </div>
  );
};

export default ExperimentPage;
