import { useQuery } from "@tanstack/react-query";
import { sampleListSchema } from "schemas/sampleSchemas";
import { Link } from "@tanstack/react-router";
import { ButtonSecondary } from "components/Buttons";
import { FaArrowLeft } from "react-icons/fa";

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

  if (isLoading || data == null) {
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
      <h1>Samples</h1>
      <ul className="mt-md">
        {data?.length === 0 && <p>No samples found.</p>}
        {data?.map((sample) => (
          <li key={sample.id.id.String} className="py-sm">
            <p>Name: {sample.name}</p>
            <p>Azimuth: {sample.azimuth}</p>
            <p>Elevation: {sample.elevation}</p>
            <p>
              URL: {VITE_BASE_API_URL}/audio/{sample.id.id.String}
            </p>
          </li>
        ))}
      </ul>
      <Link to="/samples/create" className="mt-md">
        <ButtonSecondary>Create samples</ButtonSecondary>
      </Link>
    </div>
  );
};

export default SamplesListPage;
