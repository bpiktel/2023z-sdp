import { useQuery } from "@tanstack/react-query";
import { sampleListSchema } from "schemas/sampleSchemas";

const SamplesListPage = () => {
  const { VITE_BASE_API_URL } = import.meta.env;

  const getSamples = () =>
    fetch(`${VITE_BASE_API_URL}/audio/all`)
      .then((res) => res.json())
      .then((data) => sampleListSchema.parse(data));

  const { data, isLoading, isFetching, error } = useQuery({
    queryKey: ["samples"],
    queryFn: getSamples
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
      <h1>Samples</h1>
      <ul className="mt-md">
        {data?.map((sample) => (
          <li key={sample.id.id.String} className="py-sm">
            <p>Name: {sample.name}</p>
            <p>Azimuth: {sample.azimuth}</p>
            <p>Elevation: {sample.elevation}</p>
            <p>URL: {VITE_BASE_API_URL}/audio/{sample.id.id.String}</p>
          </li>
        ))}
      </ul>
    </div>
  );
};

export default SamplesListPage;
