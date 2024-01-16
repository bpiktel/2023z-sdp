import { useQuery } from "@tanstack/react-query";
import { sampleListSchema } from "schemas/sampleSchemas";
import { Link } from "@tanstack/react-router";
import {FaArrowLeft, FaPlus} from "react-icons/fa";
import SamplePlayer from "components/player/SamplePlayer";
import { getAudioPath } from "components/player/utils";
import {FrostedGlass} from "../../components/FrostedGlass.tsx";
import {useRef, useState} from "react";
import {Howl} from "howler";

const SamplesListPage = () => {
  const { VITE_BASE_API_URL } = import.meta.env;

  const playerRef = useRef<Howl>();
  const [playerStatus, setPlayerStatus] = useState<string | null>(null);
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
    <div className="flex flex-col items-center p-xl h-screen overflow-x-hidden">
      <div className="w-full flex justify-between mb-md">
        <Link to="../" className="flex gap-xs items-center">
          <FaArrowLeft/> Return to Home Page
        </Link>
        <Link to="/samples/create" className="flex gap-xs items-center">
          <FaPlus /> Add new sample
        </Link>
      </div>
      <FrostedGlass className="flex flex-col items-center">
        <h1>Samples</h1>
        <ul className="mt-md">
          {data?.length === 0 && <p>No samples found.</p>}
          {data?.map((sample) => (
            <li key={sample.id.id.String} className="py-sm">
              <SamplePlayer
                assetPath={getAudioPath(sample.id.id.String)}
                name={sample.name}
                playerRef={playerRef}
                status={playerStatus}
                setStatus={setPlayerStatus}
              />
              <p>Azimuth: {sample.azimuth}</p>
              <p>Elevation: {sample.elevation}</p>
              <p>
                URL: {VITE_BASE_API_URL}/audio/{sample.id.id.String}
              </p>

            </li>
          ))}
        </ul>
      </FrostedGlass>
    </div>
  );
};

export default SamplesListPage;
