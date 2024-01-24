import { useQuery } from "@tanstack/react-query";
import { useParams, Link } from "@tanstack/react-router";
import { Howl } from "howler";
import { Stage } from "components/Stage";
import { experimentSchema } from "schemas/experimentSchemas";
import { useEffect, useRef, useState } from "react";
import { ButtonPrimary, ButtonSecondary } from "components/Buttons.tsx";
import { getAudioPath } from "components/player/utils.ts";
import { SphericalCoordinates } from "schemas/coordinates";
import {
  Sample,
  SampleList,
  sampleListSchema,
  SampleResult
} from "schemas/sampleSchemas";
import LoadingSpinner from "../../components/LoadingSpinner.tsx";
import { FrostedGlass } from "../../components/FrostedGlass.tsx";
import { fireAlert } from "components/AlertDialogs.tsx";
import { defaultRequestInit } from "utils/fetchUtils.ts";
import { FaArrowLeft } from "react-icons/fa";
import { onEnterDown } from "utils/formUtils.ts";

const ExperimentPage = () => {
  const { VITE_BASE_API_URL } = import.meta.env;
  const { id } = useParams({ strict: false });

  const getExperiment = () =>
    fetch(`${VITE_BASE_API_URL}/experiments/${id}`, defaultRequestInit)
      .then((res) => res.json())
      .then((data) => experimentSchema.parse(data));

  const { data, isLoading, error } = useQuery({
    queryKey: ["experiment", id],
    queryFn: getExperiment
  });

  const [audioList, setAudioList] = useState<string[]>([]);

  useEffect(() => {
    setAudioList(
      data?.sample_ids
        .map((sampleId) => getAudioPath(sampleId))
        .sort(() => Math.random() - 0.5) ?? []
    );
  }, [data]);

  const [sampleCoordinatesList, setSampleCoordinatesList] = useState<
    SphericalCoordinates[]
  >([]);

  const playerRef = useRef<Howl | undefined>();

  const [currentStep, setCurrentStep] = useState<"start" | number | "end">(
    "start"
  );
  const results = useRef<SampleResult[]>([]);

  // Current location selection, selected by the user
  const [selection, setSelection] = useState<SphericalCoordinates | null>(null);
  // Current location highlight, shows correct answer if applicable
  const [highlight, setHighlight] = useState<SphericalCoordinates | null>(null);

  const [trainingMode, setTrainingMode] = useState<boolean>(false);

  useEffect(() => {
    if (typeof currentStep === "number" && currentStep >= 0) {
      playerRef.current?.stop();
      playerRef.current = new Howl({
        src: [audioList[currentStep]],
        format: ["mp3"],
        volume: 1.0,
        loop: false,
        autoplay: true
      });
    }
  }, [currentStep]);

  useEffect(() => {
    if (!data) return;

    const fetchAllSamplesOfTheWorld = async () => {
      const rawResponse = await fetch(
        `${VITE_BASE_API_URL}/audio/all`,
        defaultRequestInit
      );
      const responseData = await rawResponse.json();
      const allSamplesOfTheWorld: SampleList =
        sampleListSchema.parse(responseData);
      setSampleCoordinatesList(
        data.sample_ids.map((sampleId) => {
          const sample: Sample | undefined = allSamplesOfTheWorld.find(
            (sample) => sample.id.id.String === sampleId
          );
          if (!sample) return { azimuth: 0, elevation: 0 };
          const coords: SphericalCoordinates = {
            azimuth: sample.azimuth,
            elevation: sample.elevation
          };
          return coords;
        })
      );
    };

    fetchAllSamplesOfTheWorld();
  }, [data]);

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
    playerRef.current?.stop();
    saveResult();
    setSelection(null);
    setHighlight(null);
    if (currentStep === audioList.length - 1) setCurrentStep("end");
    else setCurrentStep((currentStep as number) + 1);
  };

  const showHighlight = () => {
    setHighlight(sampleCoordinatesList[currentStep as number]);
  };

  if (isLoading || data == null) {
    return <p>Data is loading...</p>;
  }

  if (error) {
    return <p>There was an error when fetching your data.</p>;
  }

  if (currentStep === "start")
    return (
      <StartInfo
        experimentName={data.name}
        onStart={(isTrainingMode: boolean) => {
          setTrainingMode(isTrainingMode);
          setCurrentStep(-1);
        }}
        readyToStart={audioList.length > 0}
      />
    );

  if (currentStep === "end")
    return (
      <FinishInfo
        experimentId={id}
        results={results.current}
        isTraining={trainingMode}
      />
    );

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
        setSelection={currentStep >= 0 ? setSelection : () => {}}
        highlight={highlight}
        currentSample={currentStep}
      />
      {currentStep === -1 && (
        <div className="absolute w-full h-full flex pointer-events-none">
          <ButtonSecondary
            className="pointer-events-auto m-lg mt-auto ml-auto"
            onClick={() => setCurrentStep(0)}
          >
            Start
          </ButtonSecondary>
        </div>
      )}
      {selection !== null && (
        <div className="absolute w-full h-full flex pointer-events-none">
          <FrostedGlass
            theme="dark"
            className="flex flex-col mt-auto ml-auto rounded-lg py-sm px-lg me-md mb-md"
          >
            <p className="text-white font-semibold">
              Selected: <br />
              Azimuth: {selection.azimuth}
              <br />
              Elevation: {selection.elevation}
            </p>
            {trainingMode && !highlight ? (
              <ButtonSecondary
                className="pointer-events-auto mt-sm"
                onClick={() => showHighlight()}
              >
                Verify
              </ButtonSecondary>
            ) : (
              <ButtonSecondary
                className="pointer-events-auto mt-sm"
                onClick={() => nextSample()}
              >
                Next
              </ButtonSecondary>
            )}
          </FrostedGlass>
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
  onStart,
  readyToStart
}: {
  experimentName: string;
  onStart: (isTrainingMode: boolean) => void;
  readyToStart: boolean;
}) => {
  return (
    <div className="w-full h-full flex flex-col items-center justify-center">
      <FrostedGlass className="flex flex-col items-center justify-center mx-xxl gap-xl">
        <h1>You are about to start {experimentName}</h1>
        <div className="max-w-[48rem]">
          After starting the test you will be presented with a series of sounds
          that are coming from different directions. Your task is to identify
          the direction of the sound by clicking on the corresponding location
          on the sphere that will be displayed on the screen.
          <br />
          Training mode will show you the correct answer after each sample.
        </div>
        {readyToStart ? (
          <div className="flex gap-xl">
            <ButtonPrimary
              onClick={() => onStart(true)}
              disabled={!readyToStart}
            >
              Training mode
            </ButtonPrimary>
            <ButtonPrimary
              onClick={() => {
                onStart(false);
              }}
              disabled={!readyToStart}
            >
              Start experiment
            </ButtonPrimary>
          </div>
        ) : (
          <LoadingSpinner />
        )}
      </FrostedGlass>
    </div>
  );
};

const createResult = async (
  experimentId: string,
  username: string,
  isTraining: boolean,
  results: SampleResult[],
  callback: (success: boolean) => void
): Promise<void> => {
  const { VITE_BASE_API_URL } = import.meta.env;

  const response = await fetch(
    `${VITE_BASE_API_URL}/experiments/results/${experimentId}`,
    {
      ...defaultRequestInit,
      method: "POST",
      body: JSON.stringify({
        sample_results: results,
        training: isTraining,
        user: username
      })
    }
  );

  if (response.ok) {
    callback(true);
    return;
  }

  callback(false);
};

const FinishInfo = ({
  experimentId,
  results,
  isTraining: isTraining
}: {
  experimentId: string;
  results: SampleResult[];
  isTraining: boolean;
}) => {
  const [resultSent, setResultSent] = useState<boolean>(false);
  const [username, setUsername] = useState<string>("");

  const onResultsSave = () => {
    if (username.length === 0) {
      fireAlert({ title: "Please enter your name" });
      return;
    }

    createResult(experimentId, username, isTraining, results, (success) => {
      if (success) {
        fireAlert({ title: "Results saved" });
        setResultSent(true);
      }
    });
  };

  return (
    <div className="w-full h-full flex flex-col items-center justify-center">
      <FrostedGlass className="flex flex-col items-center justify-center mx-xxl gap-xl">
        <h1>You have finished the experiment</h1>
        <div className="max-w-[64rem]">
          Thank you for participating in this experiment.
        </div>
        {resultSent ? (
          <Link className="flex gap-xs py-xs" to="/experiments">
            <FaArrowLeft /> Return to Experiments
          </Link>
        ) : (
          <>
            <div className="flex flex-row items-center w-full">
              <p className="pr-md">Name</p>
              <input
                className="flex-1 px-2 py-1"
                type="text"
                placeholder="name..."
                onChange={(e) => setUsername(e.target.value)}
                onKeyDown={onEnterDown(onResultsSave)}
              />
            </div>
            <ButtonPrimary onClick={onResultsSave}>Save results</ButtonPrimary>
          </>
        )}
      </FrostedGlass>
    </div>
  );
};

export default ExperimentPage;
