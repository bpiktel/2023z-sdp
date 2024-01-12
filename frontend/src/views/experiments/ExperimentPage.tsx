import { useQuery } from "@tanstack/react-query";
import { useParams } from "@tanstack/react-router";
import { Howl } from "howler";
import { Stage } from "components/Stage";
import { experimentSchema } from "schemas/experimentSchemas";
import { useRef, useState } from "react";
import { ButtonPrimary, ButtonSecondary } from "components/Buttons.tsx";
import { getAudioPath } from "components/player/utils.ts";
import { SphericalCoordinates } from "schemas/coordinates";
import { SampleResult } from "schemas/sampleSchemas";

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

  const audioList =
    data?.sample_ids.map((sampleId) => getAudioPath(sampleId)) ?? [];

  const playerRef = useRef<Howl>(
    new Howl({
      src: audioList,
      format: ["mp3"],
      volume: 1,
      loop: false
    })
  );

  const [currentStep, setCurrentStep] = useState<"start" | number | "end">(
    "start"
  );
  const results = useRef<SampleResult[]>([]);

  // Current location selection, selected by the user
  const [selection, setSelection] = useState<SphericalCoordinates | null>(null);

  // Current location highlight, shows correct answer if applicable
  const [highlight, setHighlight] = useState<SphericalCoordinates | null>(null);

  const saveResult = () => {
    results.current = [
      ...results.current,
      {
        sample_id: data!.sample_ids[currentStep as number],
        azimuth: selection!.azimuth,
        elevation: selection!.elevation
      }
    ];
  };

  const nextSample = () => {
    saveResult();
    setSelection(null);
    if (currentStep === audioList.length - 1) setCurrentStep("end");
    else setCurrentStep((currentStep as number) + 1);
  };

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

  if (currentStep === "end") return <FinishInfo />;

  return (
    <div className="w-screen h-screen flex flex-col items-center relative">
      <div className="my-xs mx-md max-w-[48rem] flex items-center">
        <h1>{data.name}</h1>
        <ProgressWidget
          currentStep={currentStep}
          totalSteps={data.sample_ids.length}
        />
      </div>
      <Stage
        selection={selection}
        setSelection={setSelection}
        highlight={highlight}
      />
      {selection !== null && (
        <div className="absolute w-full h-full flex pointer-events-none">
          <div className="mt-auto ml-auto bg-black rounded-md p-xs me-md mb-md items-end text-end">
            {selection?.azimuth !== null && selection?.elevation !== null && (
              <p>
                Selected: <br />
                Azimuth: {selection?.azimuth}
                <br />
                Elevation: {selection?.elevation}
              </p>
            )}
            <ButtonSecondary
              className="pointer-events-auto mt-sm"
              onClick={() => nextSample()}
            >
              Next
            </ButtonSecondary>
          </div>
        </div>
      )}
    </div>
  );
};

const ProgressWidget = ({
  currentStep,
  totalSteps
}: {
  currentStep: number;
  totalSteps: number;
}) => {
  return (
    <div className="flex flex-col gap-xs px-md items-center">
      <div>Progress:</div>
      <div className="flex flex-row gap-xs">
        {Array.from(Array(totalSteps).keys()).map((step) => (
          <div
            key={step}
            className={`w-4 h-4 rounded-full border-2 border-white ${
              currentStep < step ? "bg-transparent" : "bg-white"
            }`}
          ></div>
        ))}
      </div>
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

const FinishInfo = () => {
  return (
    <div className="w-full h-full flex flex-col items-center justify-center">
      <h1 className="">You have finished the experiment</h1>
      <div className="mt-sm mx-xxl max-w-[64rem]">
        You will hear ... Lorem ipsum dolor sit amet, consectetur adipiscing
        elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
      </div>
      <div className="mt-md"></div>
      <ButtonPrimary className="mt-md">Save results</ButtonPrimary>
    </div>
  );
};

export default ExperimentPage;
