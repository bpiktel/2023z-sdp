import { useQuery, useQueryClient } from "@tanstack/react-query";
import { sampleListSchema } from "schemas/sampleSchemas";
import { Link } from "@tanstack/react-router";
import { FaArrowLeft, FaPlus, FaTrash } from "react-icons/fa";
import SamplePlayer from "components/player/SamplePlayer";
import { getAudioPath } from "components/player/utils";
import { FrostedGlass } from "../../components/FrostedGlass.tsx";
import { useRef, useState } from "react";
import { Howl } from "howler";
import { defaultRequestInit } from "utils/fetchUtils.ts";
import {
  fireAlert,
  fireConfirmationModal
} from "../../components/AlertDialogs.tsx";
import { ButtonSecondary } from "../../components/Buttons.tsx";

const deleteSample = async (id: string, callback: (arg0: boolean, statusCode: number) => void) => {
  const { VITE_BASE_API_URL } = import.meta.env;

  try {
    const response = await fetch(`${VITE_BASE_API_URL}/audio/${id}`, {
      ...defaultRequestInit,
      method: "DELETE"
    });

    callback(response.ok, response.status);
  } catch (error) {
    console.error(error);
    fireAlert("Error occured", String(error));
  }
};

const SamplesListPage = () => {
  const { VITE_BASE_API_URL } = import.meta.env;
  const queryClient = useQueryClient();

  const playerRef = useRef<Howl>();
  const [playerStatus, setPlayerStatus] = useState<string | null>(null);
  const getSamples = () =>
    fetch(`${VITE_BASE_API_URL}/audio/all`, defaultRequestInit)
      .then((res) => res.json())
      .then((data) => sampleListSchema.parse(data));

  const { data, isLoading, error } = useQuery({
    queryKey: ["samples"],
    queryFn: getSamples
  });

  const onDelete = async (id: string) => {
    await fireConfirmationModal({
      title: "Delete sample",
      body: "Are you sure you want to delete this sample?"
    }).then((result) => {
      if (result.isConfirmed) {
        deleteSample(id, (success, statusCode) => {
          if (success) {
            fireAlert("Sample deleted successfully");
          } else if (statusCode === 409) {
            fireAlert("Sample is used in experiments", "You can't remove it without removing those experiments");
          } else {
            fireAlert("Failed to delete sample, check if it's used in experiments");
          }
          queryClient.invalidateQueries({ queryKey: ["samples"] });
        });
      }
    });
  };

  if (isLoading || data == null) {
    return <p>Data is loading...</p>;
  }

  if (error) {
    return <p>There was an error when fetching your data.</p>;
  }

  return (
    <div className="flex flex-col items-center p-xl h-screen overflow-x-hidden">
      <div className="w-full flex justify-between mb-md">
        <Link to="../" className="flex gap-xs items-center">
          <FaArrowLeft /> Return to Home Page
        </Link>
        <Link to="/samples/create" className="flex gap-xs items-center">
          <FaPlus /> Add new sample
        </Link>
      </div>
      <FrostedGlass className="flex flex-col items-center">
        <h1>Samples</h1>
        <ul className="mt-md grid grid-cols-4 gap-lg">
          {data?.length === 0 && <p>No samples found.</p>}
          {data?.map((sample) => (
            <li key={sample.id.id.String} className="py-sm">
              <FrostedGlass
                className="flex flex-col items-center"
                theme="overlay"
              >
                <div className="flex flex-row justify-end self-stretch pb-xs">
                  <div className="flex-1" />
                  <SamplePlayer
                    assetPath={getAudioPath(sample.id.id.String)}
                    name={sample.name}
                    playerRef={playerRef}
                    status={playerStatus}
                    setStatus={setPlayerStatus}
                  />
                  <div className="flex-1 flex justify-end items-center"></div>
                </div>
                <p>Azimuth: {sample.azimuth}</p>
                <p>Elevation: {sample.elevation}</p>
                <FaTrash
                  className="size-md text-red-500 cursor-pointer mt-sm"
                  onClick={() => onDelete(sample.id.id.String)}
                />
              </FrostedGlass>
            </li>
          ))}
        </ul>
        <Link to="/samples/create" className="mt-lg w-full flex flex-col">
          <ButtonSecondary>Add new sample</ButtonSecondary>
        </Link>
      </FrostedGlass>
    </div>
  );
};

export default SamplesListPage;
