import { useQuery } from "@tanstack/react-query";
import { useParams } from "@tanstack/react-router";
import { Stage } from "components/Stage";
import { experimentSchema } from "schemas/experimentSchemas";
import SamplePlayer from "../../components/player/SamplePlayer.tsx";
import { useState } from "react";
import { ButtonPrimary } from "components/Buttons.tsx";

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

  const [currentStep, setCurrentStep] = useState<"start" | number | "end">(
    "start"
  );

  if (isLoading || data == null) {
    return <p>Data is loading...</p>;
  }

  if (isFetching) {
    return <p>Data is fetching...</p>;
  }

  if (error) {
    return <p>There was an error when fetching your data.</p>;
  }

  if (currentStep === "start")
    return (
      <StartInfo experimentName={data.name} onStart={() => setCurrentStep(0)} />
    );

  return (
    <div className="w-screen h-screen flex flex-col items-center">
      <div className="my-xs mx-md max-w-[48rem] flex items-center">
        <h1>{data?.name}</h1>
        <SamplePlayer
          assetPath="https://bigsoundbank.com/UPLOAD/mp3/0477.mp3"
          name="Wilhelm Scream"
        />
      </div>
      <Stage />
    </div>
  );
};

const StartInfo = ({
  experimentName,
  onStart
}: {
  experimentName: string;
  onStart: () => void;
}) => {
  return (
    <div className="w-full h-full flex flex-col items-center justify-center">
      <h1 className="">You are about to start {experimentName}</h1>
      <div className="mt-sm mx-xxl max-w-[64rem]">
        You will hear ... Lorem ipsum dolor sit amet, consectetur adipiscing
        elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
        Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi
        ut aliquip ex ea commodo consequat. Duis aute irure dolor in
        reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla
        pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa
        qui officia deserunt mollit anim id est laborum.
      </div>
      <div className="mt-md"></div>
      <ButtonPrimary onClick={() => onStart()} className="mt-md">
        Start experiment
      </ButtonPrimary>
    </div>
  );
};

export default ExperimentPage;
